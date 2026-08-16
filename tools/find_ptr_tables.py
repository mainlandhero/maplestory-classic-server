#!/usr/bin/env python3
"""Find arrays of code pointers in a PE image.

The mscw packet dispatcher is Themida-virtualised, so its *code* cannot be read
(`docs/transport.md`). Its *table*, if it has one, is a different matter: an array of
handler pointers is ordinary initialised data, and nothing about virtualising the
dispatch loop requires hiding it.

So scan every section that has file bytes for runs of qwords that all point into an
executable section. Vtables produce short runs everywhere; a ~700-entry opcode table
would be an enormous outlier, which is exactly what makes this worth a look.

    python tools/find_ptr_tables.py --min 24
    python tools/find_ptr_tables.py --min 64 --show 0x141234000

Nulls are allowed inside a run (unhandled opcodes leave holes) but do not extend one
past the tail, and a run must carry `--min` genuine pointers to be reported.
"""
import argparse
import struct
import sys

IMAGE_SCN_MEM_EXECUTE = 0x20000000


def load_pe(path):
    with open(path, "rb") as fh:
        data = fh.read()

    e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
    assert data[e_lfanew:e_lfanew + 4] == b"PE\0\0", "not a PE"

    coff = e_lfanew + 4
    n_sections = struct.unpack_from("<H", data, coff + 2)[0]
    size_opt = struct.unpack_from("<H", data, coff + 16)[0]
    opt = coff + 20

    magic = struct.unpack_from("<H", data, opt)[0]
    assert magic == 0x20B, "expected PE32+"
    image_base = struct.unpack_from("<Q", data, opt + 24)[0]

    sections = []
    tbl = opt + size_opt
    for i in range(n_sections):
        off = tbl + i * 40
        name = data[off:off + 8].rstrip(b"\0").decode("ascii", "replace")
        vsize, vaddr, rsize, raddr = struct.unpack_from("<IIII", data, off + 8)
        chars = struct.unpack_from("<I", data, off + 36)[0]
        sections.append({
            "name": name, "vaddr": vaddr, "vsize": vsize,
            "raddr": raddr, "rsize": rsize, "chars": chars,
        })
    return data, image_base, sections


def exec_ranges(image_base, sections):
    """VA ranges of executable sections - what a code pointer must land in."""
    out = []
    for s in sections:
        if s["chars"] & IMAGE_SCN_MEM_EXECUTE:
            lo = image_base + s["vaddr"]
            out.append((lo, lo + max(s["vsize"], s["rsize"]), s["name"]))
    return out


def which_range(va, ranges):
    for lo, hi, name in ranges:
        if lo <= va < hi:
            return name
    return None


def scan(data, image_base, sections, ranges, minimum):
    """Report every aligned run of code pointers with at least `minimum` entries."""
    hits = []
    for s in sections:
        if s["rsize"] == 0 or (s["chars"] & IMAGE_SCN_MEM_EXECUTE):
            continue                                   # no bytes, or it is code itself
        base_va = image_base + s["vaddr"]
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]

        run_start = None       # index of the first entry in the current run
        run_ptrs = 0           # genuine pointers seen in it
        last_ptr = None        # index of the most recent genuine pointer

        for i in range(0, len(blob) - 7, 8):
            q = struct.unpack_from("<Q", blob, i)[0]
            ok = q != 0 and which_range(q, ranges) is not None
            if ok:
                if run_start is None:
                    run_start = i
                run_ptrs += 1
                last_ptr = i
            elif q == 0 and run_start is not None:
                continue                               # a hole; keep the run alive
            else:
                if run_start is not None and run_ptrs >= minimum:
                    hits.append((base_va + run_start, (last_ptr - run_start) // 8 + 1,
                                 run_ptrs, s["name"]))
                run_start, run_ptrs, last_ptr = None, 0, None

        if run_start is not None and run_ptrs >= minimum:
            hits.append((base_va + run_start, (last_ptr - run_start) // 8 + 1,
                         run_ptrs, s["name"]))
    return hits


def show(data, image_base, sections, ranges, va, count):
    for s in sections:
        lo = image_base + s["vaddr"]
        if lo <= va < lo + max(s["vsize"], s["rsize"]):
            off = s["raddr"] + (va - lo)
            for i in range(count):
                q = struct.unpack_from("<Q", data, off + i * 8)[0]
                where = which_range(q, ranges) or ("-" if q == 0 else "?")
                print(f"  [{i:4}] @{va + i * 8:#x}  {q:#018x}  {where}")
            return
    print(f"VA {va:#x} is not in any section")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--exe", default="client-patched/MapleStory.exe")
    ap.add_argument("--min", type=int, default=24, help="fewest pointers to report")
    ap.add_argument("--show", help="dump entries at this VA instead of scanning")
    ap.add_argument("--count", type=int, default=64, help="entries for --show")
    args = ap.parse_args()

    data, image_base, sections = load_pe(args.exe)
    ranges = exec_ranges(image_base, sections)

    if args.show:
        show(data, image_base, sections, ranges, int(args.show, 16), args.count)
        return 0

    print(f"image base {image_base:#x}")
    print("executable ranges:")
    for lo, hi, name in ranges:
        print(f"  {name:<10} {lo:#x} - {hi:#x}")
    print("\nsections scanned:")
    for s in sections:
        if s["rsize"] and not (s["chars"] & IMAGE_SCN_MEM_EXECUTE):
            print(f"  {s['name']:<10} va {image_base + s['vaddr']:#x}  "
                  f"file {s['rsize']:#x} bytes")

    hits = scan(data, image_base, sections, ranges, args.min)
    hits.sort(key=lambda h: -h[2])
    print(f"\n{len(hits)} runs with >= {args.min} code pointers, largest first:\n")
    for va, span, ptrs, sect in hits[:40]:
        print(f"  {va:#014x}  {ptrs:5} pointers  span {span:5}  {sect}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
