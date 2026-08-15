#!/usr/bin/env python3
"""Dump bytes at a virtual address from a PE image.

The client's crypto tables live in plain .data (Ghidra reads them statically),
so we can pull them straight off disk instead of attaching a debugger.

    python tools/dump_va.py 0x143A86810 128
"""
import struct
import sys


def load_sections(path):
    with open(path, "rb") as fh:
        data = fh.read()

    e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
    assert data[e_lfanew:e_lfanew + 4] == b"PE\0\0", "not a PE"

    coff = e_lfanew + 4
    n_sections = struct.unpack_from("<H", data, coff + 2)[0]
    size_opt = struct.unpack_from("<H", data, coff + 16)[0]
    opt = coff + 20

    magic = struct.unpack_from("<H", data, opt)[0]
    if magic == 0x20B:                       # PE32+
        image_base = struct.unpack_from("<Q", data, opt + 24)[0]
    else:
        image_base = struct.unpack_from("<I", data, opt + 28)[0]

    sections = []
    tbl = opt + size_opt
    for i in range(n_sections):
        off = tbl + i * 40
        name = data[off:off + 8].rstrip(b"\0").decode("ascii", "replace")
        vsize, vaddr, rsize, raddr = struct.unpack_from("<IIII", data, off + 8)
        sections.append((name, vaddr, vsize, raddr, rsize))

    return data, image_base, sections


def va_to_off(va, image_base, sections):
    rva = va - image_base
    for name, vaddr, vsize, raddr, rsize in sections:
        if vaddr <= rva < vaddr + max(vsize, rsize):
            delta = rva - vaddr
            if delta >= rsize:
                return None, name          # in the zero-filled tail
            return raddr + delta, name
    return None, None


def main():
    exe = "client-patched/MapleStory.exe"
    args = [a for a in sys.argv[1:]]
    if args and not args[0].startswith("0x"):
        exe = args.pop(0)
    va = int(args[0], 16)
    count = int(args[1]) if len(args) > 1 else 64

    data, image_base, sections = load_sections(exe)
    off, sect = va_to_off(va, image_base, sections)
    if off is None:
        print(f"VA {va:#x} -> section {sect}, no file bytes (uninitialised)")
        return 1

    blob = data[off:off + count]
    print(f"image base {image_base:#x}   section {sect}   file offset {off:#x}")
    for i in range(0, len(blob), 16):
        row = blob[i:i + 16]
        hexs = " ".join(f"{b:02x}" for b in row)
        text = "".join(chr(b) if 32 <= b < 127 else "." for b in row)
        print(f"  {va + i:012x}  {hexs:<47}  {text}")

    print("\ndwords:")
    for i in range(0, len(blob) - 3, 4):
        w = struct.unpack_from("<I", blob, i)[0]
        print(f"  [{i // 4:2}] @{va + i:#x}  {w:#010x}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
