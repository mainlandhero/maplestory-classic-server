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
                        me.modBaseSize,
                        me.szExePath.decode(errors="replace")))
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


def thread_identity(tid):
    """(created, cpu_seconds, win32_start_address).

    Creation time is how the main thread gets identified - it is the oldest thread in
    the process, and knowing which one it is decides whether a hang is "the UI thread is
    stuck" or "a worker is idle, as workers are". CPU time separates threads that have
    done work from threads that have been parked since birth.
    """
    h = k32.OpenThread(THREAD_QUERY_INFORMATION | THREAD_GET_CONTEXT, False, tid)
    if not h:
        return None, None, None
    try:
        created, exited, kern, user = (w.FILETIME() for _ in range(4))
        cpu = None
        crt = None
        if k32.GetThreadTimes(h, ctypes.byref(created), ctypes.byref(exited),
                              ctypes.byref(kern), ctypes.byref(user)):
            def q(ft):
                return (ft.dwHighDateTime << 32) | ft.dwLowDateTime
            crt = q(created)
            cpu = (q(kern) + q(user)) / 1e7
        start = ctypes.c_ulonglong(0)
        ntdll = ctypes.WinDLL("ntdll")
        # ThreadQuerySetWin32StartAddress = 9
        if ntdll.NtQueryInformationThread(h, 9, ctypes.byref(start),
                                          ctypes.sizeof(start), None) != 0:
            start.value = 0
        return crt, cpu, start.value or None
    finally:
        k32.CloseHandle(h)


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


_EXPORTS = {}


def module_exports(path):
    """[(rva, name)] sorted, parsed from the module on disk.

    Turns `ntdll.dll+0x9d694` into `ntdll.dll!NtWaitForSingleObject+0x4`, which is the
    difference between a stack dump you can read and one you can only stare at. Cached,
    and failures degrade to the raw offset rather than raising - some modules are not
    readable and that must not take the whole dump down.
    """
    if path in _EXPORTS:
        return _EXPORTS[path]
    out = []
    try:
        with open(path, "rb") as fh:
            data = fh.read()
        e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
        if data[e_lfanew:e_lfanew + 4] != b"PE\0\0":
            raise ValueError("not a PE")
        coff = e_lfanew + 4
        n_sec = struct.unpack_from("<H", data, coff + 2)[0]
        size_opt = struct.unpack_from("<H", data, coff + 16)[0]
        opt = coff + 20
        magic = struct.unpack_from("<H", data, opt)[0]
        dd = opt + (0x70 if magic == 0x20B else 0x60)
        exp_rva, exp_size = struct.unpack_from("<II", data, dd)

        secs = []
        tbl = opt + size_opt
        for i in range(n_sec):
            o = tbl + i * 40
            vsize, vaddr, rsize, raddr = struct.unpack_from("<IIII", data, o + 8)
            secs.append((vaddr, max(vsize, rsize), raddr, rsize))

        def to_off(rva):
            for vaddr, vsize, raddr, rsize in secs:
                if vaddr <= rva < vaddr + vsize:
                    d = rva - vaddr
                    return raddr + d if d < rsize else None
            return None

        if exp_rva:
            eo = to_off(exp_rva)
            if eo is not None:
                n_names = struct.unpack_from("<I", data, eo + 24)[0]
                a_funcs = struct.unpack_from("<I", data, eo + 28)[0]
                a_names = struct.unpack_from("<I", data, eo + 32)[0]
                a_ords = struct.unpack_from("<I", data, eo + 36)[0]
                fo, no, oo = to_off(a_funcs), to_off(a_names), to_off(a_ords)
                if None not in (fo, no, oo):
                    for i in range(n_names):
                        nm_rva = struct.unpack_from("<I", data, no + i * 4)[0]
                        nm_off = to_off(nm_rva)
                        if nm_off is None:
                            continue
                        end = data.find(b"\0", nm_off)
                        name = data[nm_off:end].decode("ascii", "replace")
                        ordi = struct.unpack_from("<H", data, oo + i * 2)[0]
                        frva = struct.unpack_from("<I", data, fo + ordi * 4)[0]
                        # forwarded exports point back into the export directory
                        if exp_rva <= frva < exp_rva + exp_size:
                            continue
                        out.append((frva, name))
        out.sort()
    except Exception:
        out = []
    _EXPORTS[path] = out
    return out


def nearest_export(path, off):
    exps = module_exports(path)
    if not exps:
        return None
    j = bisect.bisect_right(exps, (off, "\xff")) - 1
    if j < 0:
        return None
    rva, name = exps[j]
    delta = off - rva
    if delta > 0x40000:                       # too far to be meaningful
        return None
    return f"{name}+{delta:#x}" if delta else name


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


def describe(addr, mods, starts, ends, client_base, image="maplestory.exe"):
    for name, base, size, path in mods:
        if base <= addr < base + size:
            off = addr - base
            if name.lower() == image.lower() and starts:
                rva = addr - client_base
                j = bisect.bisect_right(starts, rva) - 1
                if j >= 0 and rva < ends.get(starts[j], 0):
                    fn = STATIC_BASE + starts[j]
                    return f"{name}!FUN_{fn:x}+{rva - starts[j]:#x}"
                # .themida and .boot have no .pdata entries, and saying so is the point
                for sect, lo, hi in ((".themida", 0x3D87000, 0x5173000),
                                     (".boot", 0x5173000, 0x5DAA400)):
                    if lo <= rva < hi:
                        return f"{name}{sect}+{rva - lo:#x}"
                return f"{name}+{off:#x}"
            exp = nearest_export(path, off)
            return f"{name}!{exp}" if exp else f"{name}+{off:#x}"
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
    ap.add_argument("--out", help="write the report here as UTF-8. Prefer this over a "
                                  "shell redirect - PowerShell writes UTF-16 and every "
                                  "reader downstream then has to know that")
    args = ap.parse_args()

    if args.out:
        sink = open(args.out, "w", encoding="utf-8")
        real = print

        def _print(*a, **k):
            k["file"] = sink
            real(*a, **k)
        globals()["print"] = _print

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

    # Order by creation time so the main thread comes first and is labelled as such.
    ident = {t: thread_identity(t) for t in tids}
    tids.sort(key=lambda t: (ident[t][0] is None, ident[t][0] or 0))
    print(f"{len(tids)} threads, oldest first\n")

    for n, tid in enumerate(tids):
        ctx = thread_context(tid)
        if ctx is None:
            print(f"thread {tid}: could not read context")
            continue
        rip, rsp, rbp = ctx
        _, cpu, start = ident[tid]
        at = describe(rip, mods, starts, ends, client_base, args.image) \
            or f"{rip:#x} (unmapped)"
        who = describe(start, mods, starts, ends, client_base, args.image) if start else None
        label = "  <-- MAIN THREAD" if n == 0 else ""
        print(f"thread {tid:6}  cpu {cpu if cpu is not None else -1:8.3f}s  "
              f"rip {rip:#018x}  {at}{label}")
        if who:
            print(f"                 started at {who}")

        want = min(args.depth * 8, committed_bytes(h, rsp) or args.depth * 8)
        stack = read(h, rsp, want, why="reading the stack") if want else b""
        shown = 0
        seen = set()
        for i in range(0, len(stack) - 7, 8):
            q = struct.unpack_from("<Q", stack, i)[0]
            d = describe(q, mods, starts, ends, client_base, args.image)
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
    if args.out:
        sink.close()
        globals()["print"] = __builtins__["print"] if isinstance(__builtins__, dict) \
            else __builtins__.print
        print(f"wrote {args.out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
