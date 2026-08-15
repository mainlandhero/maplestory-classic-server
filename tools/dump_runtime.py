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
    sys.path.insert(0, "tools")
    from dump_va import load_sections, va_to_off

    data, base, sections = load_sections("client-patched/MapleStory.exe")
    off, _ = va_to_off(va, base, sections)
    return data[off : off + n] if off is not None else b""


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

    flag = read(pid, TABLE_INIT_FLAG + slide, 1)
    built = "built" if flag and flag[0] != 0 else "NOT built"
    print(f"AES table-init flag @ {TABLE_INIT_FLAG:#x}: {flag.hex()} ({built})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
