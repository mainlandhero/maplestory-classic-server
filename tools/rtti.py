#!/usr/bin/env python3
"""Recover C++ class names and vtables from the client's MSVC RTTI.

The mscw client ships **full RTTI**: `.data` is packed with type descriptors like
`.?AVCLoginQueueDlg@@`. Themida virtualised the packet dispatcher, but it left the
type system completely intact, and that is a much better map than anything the call
graph gives us - it names classes, and a class name plus a vtable locates the virtual
method that handles packets for that stage.

MSVC x64 layout used here:

    TypeDescriptor          +0x00 vfptr  +0x08 spare  +0x10 name ".?AVCLogin@@"
    CompleteObjectLocator   +0x00 sig    +0x04 offset +0x08 cdOffset
                            +0x0C typeDescriptor RVA  +0x10 classDescriptor RVA
                            +0x14 self RVA
    vtable                  [-8] -> CompleteObjectLocator, then the virtual methods

    python tools/rtti.py --list Login          # class names matching a substring
    python tools/rtti.py --vtable CLogin       # vtables + virtual method addresses
"""
import argparse
import re
import struct
import sys

IMAGE_SCN_MEM_EXECUTE = 0x20000000
NAME_RE = re.compile(rb"\.\?A[VU][A-Za-z0-9_@?$.<>\-]{1,300}?@@")


def load_pe(path):
    with open(path, "rb") as fh:
        data = fh.read()
    e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
    assert data[e_lfanew:e_lfanew + 4] == b"PE\0\0", "not a PE"
    coff = e_lfanew + 4
    n_sections = struct.unpack_from("<H", data, coff + 2)[0]
    size_opt = struct.unpack_from("<H", data, coff + 16)[0]
    opt = coff + 20
    assert struct.unpack_from("<H", data, opt)[0] == 0x20B, "expected PE32+"
    image_base = struct.unpack_from("<Q", data, opt + 24)[0]
    sections = []
    tbl = opt + size_opt
    for i in range(n_sections):
        off = tbl + i * 40
        name = data[off:off + 8].rstrip(b"\0").decode("ascii", "replace")
        vsize, vaddr, rsize, raddr = struct.unpack_from("<IIII", data, off + 8)
        chars = struct.unpack_from("<I", data, off + 36)[0]
        sections.append({"name": name, "vaddr": vaddr, "vsize": vsize,
                         "raddr": raddr, "rsize": rsize, "chars": chars})
    return data, image_base, sections


def off_to_rva(off, sections):
    for s in sections:
        if s["rsize"] and s["raddr"] <= off < s["raddr"] + s["rsize"]:
            return s["vaddr"] + (off - s["raddr"])
    return None


def rva_to_off(rva, sections):
    for s in sections:
        if s["vaddr"] <= rva < s["vaddr"] + max(s["vsize"], s["rsize"]):
            d = rva - s["vaddr"]
            if d < s["rsize"]:
                return s["raddr"] + d
    return None


def exec_ranges(image_base, sections):
    out = []
    for s in sections:
        if s["chars"] & IMAGE_SCN_MEM_EXECUTE:
            lo = image_base + s["vaddr"]
            out.append((lo, lo + max(s["vsize"], s["rsize"])))
    return out


def find_type_descriptors(data, sections):
    """name -> TypeDescriptor RVA. The name sits at descriptor+0x10."""
    out = {}
    for m in NAME_RE.finditer(data):
        name_off = m.start()
        td_off = name_off - 0x10
        if td_off < 0:
            continue
        rva = off_to_rva(td_off, sections)
        if rva is None:
            continue
        out.setdefault(m.group().decode("ascii", "replace"), rva)
    return out


def find_col_for(data, sections, td_rva):
    """CompleteObjectLocator RVAs whose typeDescriptor field is td_rva."""
    cols = []
    packed = struct.pack("<I", td_rva)
    for s in sections:
        if not s["rsize"] or s["name"] not in (".rdata", ".data"):
            continue
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        start = 0
        while True:
            i = blob.find(packed, start)
            if i < 0:
                break
            start = i + 1
            col_off = s["raddr"] + i - 0x0C
            col_rva = off_to_rva(col_off, sections)
            if col_rva is None:
                continue
            sig = struct.unpack_from("<I", data, col_off)[0]
            self_rva = struct.unpack_from("<I", data, col_off + 0x14)[0]
            if sig == 1 and self_rva == col_rva:      # x64 COLs are self-referential
                cols.append(col_rva)
    return cols


def find_vtables(data, image_base, sections, col_rva):
    """vtable VAs: a qword pointing at the COL; the methods start 8 bytes later."""
    out = []
    target = struct.pack("<Q", image_base + col_rva)
    for s in sections:
        if not s["rsize"] or s["name"] not in (".rdata", ".data"):
            continue
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        start = 0
        while True:
            i = blob.find(target, start)
            if i < 0:
                break
            start = i + 1
            if i % 8 == 0:
                out.append(image_base + s["vaddr"] + i + 8)
    return out


def read_vtable(data, image_base, sections, ranges, vt_va, limit=64):
    methods = []
    off = rva_to_off(vt_va - image_base, sections)
    if off is None:
        return methods
    for i in range(limit):
        q = struct.unpack_from("<Q", data, off + i * 8)[0]
        if not any(lo <= q < hi for lo, hi in ranges):
            break
        methods.append(q)
    return methods


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--exe", default="client-patched/MapleStory.exe")
    ap.add_argument("--list", help="print class names containing this substring")
    ap.add_argument("--vtable", help="exact-ish class name, e.g. CLogin")
    ap.add_argument("--limit", type=int, default=64, help="max virtual methods to print")
    args = ap.parse_args()

    data, image_base, sections = load_pe(args.exe)
    names = find_type_descriptors(data, sections)
    ranges = exec_ranges(image_base, sections)
    print(f"{len(names)} RTTI type descriptors")

    if args.list is not None:
        pat = args.list.lower()
        hits = sorted(n for n in names if pat in n.lower())
        print(f"{len(hits)} matching {args.list!r}:\n")
        for n in hits:
            print(f"  {names[n] + image_base:#x}  {n}")
        return 0

    if args.vtable:
        want = f".?AV{args.vtable}@@"
        matches = [n for n in names if n == want] or \
                  [n for n in names if args.vtable.lower() in n.lower()]
        if not matches:
            print(f"no class matching {args.vtable!r}")
            return 1
        for n in matches[:10]:
            td = names[n]
            cols = find_col_for(data, sections, td)
            print(f"\n{n}\n  typeDescriptor {image_base + td:#x}   {len(cols)} locator(s)")
            for col in cols:
                vts = find_vtables(data, image_base, sections, col)
                for vt in vts:
                    ms = read_vtable(data, image_base, sections, ranges, vt, args.limit)
                    print(f"  vtable {vt:#x}  ({len(ms)} virtual methods)")
                    for i, m in enumerate(ms):
                        print(f"      [{i:3}]  {m:#x}")
        return 0

    ap.print_help()
    return 0


if __name__ == "__main__":
    sys.exit(main())
