"""Read the client's crypto tables out of the *running* process.

The static bytes at 0x143A86810 are the stock MapleStory AES key, and encrypting with it
does not work in either direction - while every non-AES part of the transport (framing,
header constant, IV chain) is provably correct in both. That points at the key being
patched at runtime, which a Themida-protected binary is entirely capable of doing and
which no amount of static reading will reveal.

Read-only: OpenProcess with PROCESS_VM_READ, then ReadProcessMemory. Nothing is written.

    python tools/dump_runtime.py            # while the client is running
"""

import ctypes
import ctypes.wintypes as w
import struct
import sys

STATIC_BASE = 0x140000000
KEY_VA = 0x143A86810
SHUFFLE_VA = 0x143A86890
TABLE_INIT_FLAG = 0x143AC38D0  # FUN_140c759a0 sets this once the AES tables are built

PROCESS_VM_READ = 0x0010
PROCESS_QUERY_INFORMATION = 0x0400
TH32CS_SNAPMODULE = 0x00000008
TH32CS_SNAPMODULE32 = 0x00000010

k32 = ctypes.WinDLL("kernel32", use_last_error=True)


class MODULEENTRY32(ctypes.Structure):
    _fields_ = [
        ("dwSize", w.DWORD),
        ("th32ModuleID", w.DWORD),
        ("th32ProcessID", w.DWORD),
        ("GlblcntUsage", w.DWORD),
        ("ProccntUsage", w.DWORD),
        ("modBaseAddr", ctypes.POINTER(ctypes.c_byte)),
        ("modBaseSize", w.DWORD),
        ("hModule", w.HMODULE),
        ("szModule", ctypes.c_char * 256),
        ("szExePath", ctypes.c_char * 260),
    ]


def find_client():
    """(pid, module base) for MapleStory.exe, or (None, None)."""
    import subprocess

    out = subprocess.run(
        ["tasklist", "/FI", "IMAGENAME eq MapleStory.exe", "/FO", "CSV", "/NH"],
        capture_output=True, text=True,
    ).stdout
    pids = []
    for line in out.splitlines():
        parts = [p.strip('"') for p in line.split('","')]
        if len(parts) > 1 and parts[0].lower().startswith("maplestory"):
            try:
                pids.append(int(parts[1]))
            except ValueError:
                pass
    if not pids:
        return None, None

    pid = pids[0]
    snap = k32.CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid)
    if snap == -1:
        return pid, None
    me = MODULEENTRY32()
    me.dwSize = ctypes.sizeof(MODULEENTRY32)
    base = None
    if k32.Module32First(snap, ctypes.byref(me)):
        while True:
            if me.szModule.decode(errors="replace").lower() == "maplestory.exe":
                base = ctypes.cast(me.modBaseAddr, ctypes.c_void_p).value
                break
            if not k32.Module32Next(snap, ctypes.byref(me)):
                break
    k32.CloseHandle(snap)
    return pid, base


def read(pid, addr, size):
    h = k32.OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, False, pid)
    if not h:
        raise OSError(f"OpenProcess failed: {ctypes.get_last_error()} "
                      "(try running this from an elevated shell)")
    try:
        buf = (ctypes.c_char * size)()
        got = ctypes.c_size_t(0)
        ok = k32.ReadProcessMemory(h, ctypes.c_void_p(addr), buf, size, ctypes.byref(got))
        if not ok:
            raise OSError(f"ReadProcessMemory failed at {addr:#x}: {ctypes.get_last_error()}")
        return bytes(buf[: got.value])
    finally:
        k32.CloseHandle(h)


def static_bytes(va, n):
    # Resolve relative to this file, not the cwd - this gets run from anywhere,
    # including an elevated shell that starts in system32.
    import os

    here = os.path.dirname(os.path.abspath(__file__))
    sys.path.insert(0, here)
    from dump_va import load_sections, va_to_off

    exe = os.path.join(os.path.dirname(here), "client-patched", "MapleStory.exe")
    data, base, sections = load_sections(exe)
    off, _ = va_to_off(va, base, sections)
    return data[off : off + n] if off is not None else b""


def dump_region(pid, va, size, out_path, slide=0):
    """Dump live memory to a file.

    The reason this exists: the packet dispatcher `FUN_1415d60e0` tail-jumps to
    0x144ADD569, which lands in the `.themida` section - a section with **no file bytes**,
    materialised only at runtime. It is the one thing that knows the inbound opcode table,
    and static analysis cannot reach it. But it is plain code once the process is running,
    so dumping it is the way in.
    """
    data = read(pid, va + slide, size)
    with open(out_path, "wb") as fh:
        fh.write(data)
    nonzero = sum(1 for b in data if b)
    print(f"dumped {len(data)} bytes from {va:#x} -> {out_path}")
    print(f"  {nonzero} non-zero bytes ({100.0 * nonzero / max(len(data), 1):.1f}%)")
    print(f"  first 32: {data[:32].hex(' ')}")
    return data


MEM_COMMIT = 0x1000
PAGE_READABLE = 0x02 | 0x04 | 0x08 | 0x20 | 0x40 | 0x80   # R, RW, WC, XR, XRW, XWC
PAGE_GUARD = 0x100


class MEMORY_BASIC_INFORMATION(ctypes.Structure):
    _fields_ = [
        ("BaseAddress", ctypes.c_void_p), ("AllocationBase", ctypes.c_void_p),
        ("AllocationProtect", w.DWORD), ("__alignment1", w.DWORD),
        ("RegionSize", ctypes.c_size_t), ("State", w.DWORD),
        ("Protect", w.DWORD), ("Type", w.DWORD), ("__alignment2", w.DWORD),
    ]


def regions(h):
    """Every committed, readable region in the target."""
    k32.VirtualQueryEx.argtypes = [w.HANDLE, ctypes.c_void_p, ctypes.c_void_p,
                                   ctypes.c_size_t]
    k32.VirtualQueryEx.restype = ctypes.c_size_t
    out = []
    addr = 0
    mbi = MEMORY_BASIC_INFORMATION()
    while addr < 0x7FFFFFFF0000:
        if not k32.VirtualQueryEx(h, ctypes.c_void_p(addr), ctypes.byref(mbi),
                                  ctypes.sizeof(mbi)):
            break
        base = mbi.BaseAddress or 0
        size = mbi.RegionSize
        if size == 0:
            break
        if (mbi.State == MEM_COMMIT and (mbi.Protect & PAGE_READABLE)
                and not (mbi.Protect & PAGE_GUARD)):
            out.append((base, size, mbi.Protect))
        addr = base + size
    return out


def find_qwords(pid, values, context=12):
    """Search the whole address space for 8-byte values, and show their neighbours.

    Why: `FUN_1415e5c20` is the handler that unblocks the client's startup loop, and the
    opcode that reaches it lives only in the virtualised dispatcher. But whatever form
    the dispatch takes, it has to hold that address somewhere. Finding the address in
    memory and printing what surrounds it says whether it sits in an array - and if it
    does, its index *is* the opcode.
    """
    h = k32.OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, False, pid)
    if not h:
        raise OSError(f"OpenProcess failed: {ctypes.get_last_error()} "
                      "(try running this from an elevated shell)")
    k32.ReadProcessMemory.argtypes = [w.HANDLE, ctypes.c_void_p, ctypes.c_void_p,
                                      ctypes.c_size_t, ctypes.POINTER(ctypes.c_size_t)]
    needles = {struct.pack("<Q", v): v for v in values}
    hits = []
    scanned = 0
    try:
        for base, size, protect in regions(h):
            step = 1 << 20
            off = 0
            while off < size:
                n = min(step, size - off)
                buf = (ctypes.c_char * n)()
                got = ctypes.c_size_t(0)
                if not k32.ReadProcessMemory(h, ctypes.c_void_p(base + off), buf, n,
                                             ctypes.byref(got)) and got.value == 0:
                    off += n
                    continue
                blob = bytes(buf[: got.value])
                scanned += len(blob)
                for needle, val in needles.items():
                    start = 0
                    while True:
                        i = blob.find(needle, start)
                        if i < 0:
                            break
                        start = i + 1
                        if (base + off + i) % 8 == 0:
                            hits.append((base + off + i, val, protect))
                off += n
        print(f"scanned {scanned / (1 << 20):.0f} MB of committed memory")
        print(f"{len(hits)} aligned hits\n")
        for addr, val, protect in hits:
            print(f"=== {val:#x} found at {addr:#x}   protect {protect:#x}")
            lo = addr - context * 8
            blob = read(pid, lo, (context * 2 + 1) * 8)
            for j in range(0, len(blob) - 7, 8):
                q = struct.unpack_from("<Q", blob, j)[0]
                a = lo + j
                mark = "  <<<<" if a == addr else ""
                kind = "code" if 0x140001000 <= q < 0x143261A00 else (
                    "-" if q == 0 else "")
                print(f"    [{(a - addr) // 8:+4}] {a:#x}  {q:#018x}  {kind}{mark}")
            print()
    finally:
        k32.CloseHandle(h)
    return hits


def main():
    pid, base = find_client()
    if pid is None:
        print("MapleStory.exe is not running - start a test and leave it hanging.")
        return 1
    if base is None:
        print(f"found pid {pid} but could not read its module base")
        return 1

    slide = base - STATIC_BASE
    print(f"pid {pid}   module base {base:#x}   ASLR slide {slide:+#x}\n")

    # Locate a known handler's address in memory, to recover the dispatch mapping.
    #   python tools/dump_runtime.py --find 0x1415e5c20
    if "--find" in sys.argv:
        i = sys.argv.index("--find")
        vals = []
        for a in sys.argv[i + 1:]:
            if a.startswith("-"):
                break
            vals.append(int(a, 16) + slide)
        find_qwords(pid, vals)
        return 0

    for name, va, n in (("AES key table", KEY_VA, 128), ("IV shuffle table", SHUFFLE_VA, 256)):
        live = read(pid, va + slide, n)
        disk = static_bytes(va, n)
        same = live == disk
        print(f"{name} @ {va:#x}  ({'IDENTICAL to disk' if same else '*** DIFFERS FROM DISK ***'})")
        print(f"  live: {live[:32].hex(' ')}")
        print(f"  disk: {disk[:32].hex(' ')}")
        if name.startswith("AES") :
            words = struct.unpack("<32I", live)
            key = b"".join(struct.pack("<I", words[i]) for i in range(0, 32, 4))
            print(f"  live key (8 words at stride 4): {key.hex(' ')}")
        if not same:
            diff = [i for i, (a, b) in enumerate(zip(live, disk)) if a != b]
            print(f"  {len(diff)} of {n} bytes differ, first at offset {diff[0]}")
        print()

    # Optional: dump a live region, e.g. the .themida dispatcher.
    #   python tools/dump_runtime.py --dump 0x144ADD569 0x8000 research/themida-dispatch.bin
    if "--dump" in sys.argv:
        i = sys.argv.index("--dump")
        va = int(sys.argv[i + 1], 16)
        size = int(sys.argv[i + 2], 0)
        out = sys.argv[i + 3]
        dump_region(pid, va, size, out, slide)
        return 0

    flag = read(pid, TABLE_INIT_FLAG + slide, 1)
    built = "built" if flag and flag[0] != 0 else "NOT built"
    print(f"AES table-init flag @ {TABLE_INIT_FLAG:#x}: {flag.hex()} ({built})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
