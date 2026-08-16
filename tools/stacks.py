"""Show where every thread in the running client actually is.

Six sessions of work have assumed the client "hangs waiting on the socket for a packet
we have not guessed". That is an inference, not an observation, and everything downstream
of it - the opcode sweeps, the 26-packet budget - inherits the risk. This checks it
directly.

For each thread: suspend, read RIP/RSP, resume, then scan its stack for values that land
inside a loaded module and resolve them through `.pdata` to function starts. That is a
heuristic stack walk rather than a real RtlVirtualUnwind one, so it prints stale frames
too - but it names the module and function the thread is parked in, which is the thing
we need. If the main thread sits in `ws2_32!recv` the packet theory holds; if it sits in
a WZ loader, a wait handle, or MapleSecurePC, it never did.

Threads are suspended one at a time and resumed immediately; nothing is written.

    python tools/stacks.py                 # all threads, top frames
    python tools/stacks.py --depth 4000    # scan deeper into each stack
"""

import argparse
import bisect
import ctypes
import ctypes.wintypes as w
import os
import struct
import sys

STATIC_BASE = 0x140000000

PROCESS_VM_READ = 0x0010
PROCESS_QUERY_INFORMATION = 0x0400
THREAD_GET_CONTEXT = 0x0008
THREAD_SUSPEND_RESUME = 0x0002
THREAD_QUERY_INFORMATION = 0x0040
TH32CS_SNAPMODULE = 0x00000008
TH32CS_SNAPMODULE32 = 0x00000010
TH32CS_SNAPTHREAD = 0x00000004
CONTEXT_AMD64 = 0x00100000
CONTEXT_CONTROL = CONTEXT_AMD64 | 0x1
CONTEXT_INTEGER = CONTEXT_AMD64 | 0x2

CTX_SIZE = 1232
OFF_CONTEXTFLAGS = 0x30
OFF_RSP = 0x98
OFF_RBP = 0xA0
OFF_RIP = 0xF8

k32 = ctypes.WinDLL("kernel32", use_last_error=True)

# ctypes defaults every restype to C int, which silently truncates a 64-bit HANDLE to a
# signed 32-bit value. It usually survives because handles are small, but it fails at the
# worst possible moment, so pin the ones we use.
k32.OpenProcess.restype = w.HANDLE
k32.OpenThread.restype = w.HANDLE
k32.CreateToolhelp32Snapshot.restype = w.HANDLE
k32.ReadProcessMemory.argtypes = [w.HANDLE, ctypes.c_void_p, ctypes.c_void_p,
                                  ctypes.c_size_t, ctypes.POINTER(ctypes.c_size_t)]
k32.GetThreadContext.argtypes = [w.HANDLE, ctypes.c_void_p]
k32.VirtualQueryEx.argtypes = [w.HANDLE, ctypes.c_void_p, ctypes.c_void_p,
                               ctypes.c_size_t]
k32.VirtualQueryEx.restype = ctypes.c_size_t


class MEMORY_BASIC_INFORMATION(ctypes.Structure):
    _fields_ = [
        ("BaseAddress", ctypes.c_void_p), ("AllocationBase", ctypes.c_void_p),
        ("AllocationProtect", w.DWORD), ("__alignment1", w.DWORD),
        ("RegionSize", ctypes.c_size_t), ("State", w.DWORD),
        ("Protect", w.DWORD), ("Type", w.DWORD), ("__alignment2", w.DWORD),
    ]


class MODULEENTRY32(ctypes.Structure):
    _fields_ = [
        ("dwSize", w.DWORD), ("th32ModuleID", w.DWORD), ("th32ProcessID", w.DWORD),
        ("GlblcntUsage", w.DWORD), ("ProccntUsage", w.DWORD),
        ("modBaseAddr", ctypes.POINTER(ctypes.c_byte)), ("modBaseSize", w.DWORD),
        ("hModule", w.HMODULE), ("szModule", ctypes.c_char * 256),
        ("szExePath", ctypes.c_char * 260),
    ]


class THREADENTRY32(ctypes.Structure):
    _fields_ = [
        ("dwSize", w.DWORD), ("cntUsage", w.DWORD), ("th32ThreadID", w.DWORD),
        ("th32OwnerProcessID", w.DWORD), ("tpBasePri", ctypes.c_long),
        ("tpDeltaPri", ctypes.c_long), ("dwFlags", w.DWORD),
    ]


def find_pid():
    import subprocess
    out = subprocess.run(
        ["tasklist", "/FI", "IMAGENAME eq MapleStory.exe", "/FO", "CSV", "/NH"],
        capture_output=True, text=True).stdout
    for line in out.splitlines():
        parts = [p.strip('"') for p in line.split('","')]
        if len(parts) > 1 and parts[0].lower().startswith("maplestory"):
            try:
                return int(parts[1])
            except ValueError:
                pass
    return None


def modules(pid):
    """[(name, base, size)] for every module in the target."""
    snap = k32.CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid)
    if snap == -1:
        return []
    me = MODULEENTRY32()
    me.dwSize = ctypes.sizeof(MODULEENTRY32)
    out = []
    if k32.Module32First(snap, ctypes.byref(me)):
        while True:
            out.append((me.szModule.decode(errors="replace"),
                        ctypes.cast(me.modBaseAddr, ctypes.c_void_p).value,
                        me.modBaseSize))
            if not k32.Module32Next(snap, ctypes.byref(me)):
                break
    k32.CloseHandle(snap)
    return out


def threads(pid):
    snap = k32.CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0)
    if snap == -1:
        return []
    te = THREADENTRY32()
    te.dwSize = ctypes.sizeof(THREADENTRY32)
    out = []
    if k32.Thread32First(snap, ctypes.byref(te)):
        while True:
            if te.th32OwnerProcessID == pid:
                out.append(te.th32ThreadID)
            if not k32.Thread32Next(snap, ctypes.byref(te)):
                break
    k32.CloseHandle(snap)
    return out


def open_proc(pid):
    h = k32.OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, False, pid)
    if not h:
        raise OSError(f"OpenProcess failed: {ctypes.get_last_error()} "
                      "(run this from an elevated shell)")
    return h


def committed_bytes(h, addr):
    """How much readable memory follows `addr` in its region.

    A thread's stack is only committed as far as it has actually grown, so a fixed-size
    read past the top fails wholesale with ERROR_PARTIAL_COPY and looks exactly like an
    idle thread. Ask first, then read what is really there.
    """
    mbi = MEMORY_BASIC_INFORMATION()
    n = k32.VirtualQueryEx(h, ctypes.c_void_p(addr), ctypes.byref(mbi),
                           ctypes.sizeof(mbi))
    if not n or mbi.State != 0x1000:                 # MEM_COMMIT
        return 0
    base = mbi.BaseAddress or 0
    return max(0, base + mbi.RegionSize - addr)


def read(h, addr, size, why=""):
    """Best-effort read. A stack can end mid-region, so a short read is normal; a zero
    read is not, and staying quiet about it is how a broken walker looks like an idle
    process."""
    buf = (ctypes.c_char * size)()
    got = ctypes.c_size_t(0)
    ok = k32.ReadProcessMemory(h, ctypes.c_void_p(addr), buf, size, ctypes.byref(got))
    if not ok and got.value == 0:
        err = ctypes.get_last_error()
        if err and why:
            print(f"    (read {size:#x} at {addr:#x} failed: error {err} {why})")
        return b""
    return bytes(buf[: got.value])


def thread_context(tid):
    """(rip, rsp, rbp) or None. Suspends only for as long as the read takes."""
    k32.OpenThread.restype = w.HANDLE
    h = k32.OpenThread(THREAD_GET_CONTEXT | THREAD_SUSPEND_RESUME |
                       THREAD_QUERY_INFORMATION, False, tid)
    if not h:
        return None
    try:
        if k32.SuspendThread(h) == 0xFFFFFFFF:
            return None
        try:
            raw = (ctypes.c_char * (CTX_SIZE + 16))()
            addr = ctypes.addressof(raw)
            aligned = (addr + 15) & ~15
            ctypes.memset(aligned, 0, CTX_SIZE)
            struct.pack_into("<I", (ctypes.c_char * CTX_SIZE).from_address(aligned),
                             OFF_CONTEXTFLAGS, CONTEXT_CONTROL | CONTEXT_INTEGER)
            if not k32.GetThreadContext(h, ctypes.c_void_p(aligned)):
                return None
            blob = ctypes.string_at(aligned, CTX_SIZE)
            return (struct.unpack_from("<Q", blob, OFF_RIP)[0],
                    struct.unpack_from("<Q", blob, OFF_RSP)[0],
                    struct.unpack_from("<Q", blob, OFF_RBP)[0])
        finally:
            k32.ResumeThread(h)
    finally:
        k32.CloseHandle(h)


def load_pdata(exe):
    here = os.path.dirname(os.path.abspath(__file__))
    sys.path.insert(0, here)
    from dump_va import load_sections
    data, base, sections = load_sections(exe)
    pd = next(s for s in sections if s[0] == ".pdata")
    _, vaddr, vsize, raddr, rsize = pd
    ends = {}
    for i in range(0, rsize - 11, 12):
        begin, end, _ = struct.unpack_from("<III", data, raddr + i)
        if begin and end > begin:
            ends[begin] = max(ends.get(begin, 0), end)
    return sorted(ends), ends


def describe(addr, mods, starts, ends, client_base):
    for name, base, size in mods:
        if base <= addr < base + size:
            off = addr - base
            if name.lower() == "maplestory.exe":
                rva = addr - client_base
                j = bisect.bisect_right(starts, rva) - 1
                if j >= 0 and rva < ends.get(starts[j], 0):
                    fn = STATIC_BASE + starts[j]
                    return f"MapleStory.exe!FUN_{fn:x}+{rva - starts[j]:#x}"
                return f"MapleStory.exe+{off:#x}"
            return f"{name}+{off:#x}"
    return None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--depth", type=int, default=1200,
                    help="qwords of stack to scan per thread")
    ap.add_argument("--frames", type=int, default=14, help="frames to print per thread")
    ap.add_argument("--all", action="store_true", help="print threads with no frames too")
    ap.add_argument("--pid", type=int,
                    help="attach to this pid instead of finding the client "
                         "(used to smoke-test the walker against any process)")
    ap.add_argument("--image", default="maplestory.exe",
                    help="main module name, for use with --pid")
    args = ap.parse_args()

    pid = args.pid if args.pid else find_pid()
    if pid is None:
        print("MapleStory.exe is not running - start a test and leave it up.")
        return 1

    mods = modules(pid)
    client = next((m for m in mods if m[0].lower() == args.image.lower()), None)
    if client is None:
        print(f"pid {pid}: could not read the module list (elevated shell needed?)")
        return 1
    client_base = client[1]
    here = os.path.dirname(os.path.abspath(__file__))
    exe = os.path.join(os.path.dirname(here), "client-patched", "MapleStory.exe")
    starts, ends = load_pdata(exe) if os.path.exists(exe) and not args.pid else ([], {})

    print(f"pid {pid}   {client[0]} base {client_base:#x}"
          + (f"   slide {client_base - STATIC_BASE:+#x}" if starts else ""))
    print(f"{len(mods)} modules, {len(starts)} known functions\n")

    h = open_proc(pid)
    tids = threads(pid)
    print(f"{len(tids)} threads\n")
    for tid in tids:
        ctx = thread_context(tid)
        if ctx is None:
            print(f"thread {tid}: could not read context")
            continue
        rip, rsp, rbp = ctx
        at = describe(rip, mods, starts, ends, client_base) or f"{rip:#x} (unmapped)"
        print(f"thread {tid:6}  rip {rip:#018x}  {at}")

        want = min(args.depth * 8, committed_bytes(h, rsp) or args.depth * 8)
        stack = read(h, rsp, want, why="reading the stack") if want else b""
        shown = 0
        seen = set()
        for i in range(0, len(stack) - 7, 8):
            q = struct.unpack_from("<Q", stack, i)[0]
            d = describe(q, mods, starts, ends, client_base)
            if d is None or d in seen:
                continue
            seen.add(d)
            print(f"                 [rsp+{i:#06x}]  {q:#018x}  {d}")
            shown += 1
            if shown >= args.frames:
                break
        if shown == 0:
            print("                 (no resolvable frames on the stack)")
        print()
    k32.CloseHandle(h)
    return 0


if __name__ == "__main__":
    sys.exit(main())
