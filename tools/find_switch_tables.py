#!/usr/bin/env python3
"""Find MSVC switch jump tables, which is where a packet-opcode switch would live.

`tools/find_ptr_tables.py` looks for arrays of 8-byte pointers - vtables. A `switch`
does not compile to that. MSVC x64 emits

    lea   rbase, [rip + __ImageBase]
    mov   eax, [rtable + rindex*4]      ; table of 4-byte RVAs
    add   rax, rbase
    jmp   rax

so the interesting object is a dense run of **DWORD RVAs** pointing into `.text`, often
preceded by a byte-sized index table that maps sparse cases onto compact slots.

A `switch (opcode)` over hundreds of packet types produces one enormous such run. That is
the thing worth finding: the dispatcher `FUN_1415d60e0` is virtualised, but only functions
Themida was told to protect get that treatment, and any per-stage `OnPacket` switch
elsewhere in the client is ordinary code with an ordinary table.

    python tools/find_switch_tables.py --min 64
    python tools/find_switch_tables.py --show 0x1433xxxxx --count 300
"""
import argparse
import bisect
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


def parse_pdata(data, sections):
    s = next(x for x in sections if x["name"] == ".pdata")
    blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
    ends = {}
    for i in range(0, len(blob) - 11, 12):
        begin, end, _ = struct.unpack_from("<III", blob, i)
        if begin and end > begin:
            ends[begin] = max(ends.get(begin, 0), end)
    return sorted(ends), ends


def owner(rva, starts, ends):
    j = bisect.bisect_right(starts, rva) - 1
    if j < 0:
        return None
    st = starts[j]
    return st if rva < ends.get(st, 0) else None


def text_range(sections):
    s = next(x for x in sections if x["name"] == ".text")
    return s["vaddr"], s["vaddr"] + max(s["vsize"], s["rsize"])


def scan(data, sections, lo, hi, minimum):
    """Runs of DWORDs that are all plausible .text RVAs."""
    hits = []
    for s in sections:
        if not s["rsize"] or (s["chars"] & IMAGE_SCN_MEM_EXECUTE):
            continue
        if s["name"] in (".pdata", ".rsrc", ".idata", ".reloc"):
            continue
        base_rva = s["vaddr"]
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        start = None
        count = 0
        for i in range(0, len(blob) - 3, 4):
            v = struct.unpack_from("<I", blob, i)[0]
            if lo <= v < hi:
                if start is None:
                    start = i
                    count = 0
                count += 1
            else:
                if start is not None and count >= minimum:
                    hits.append((base_rva + start, count, s["name"]))
                start, count = None, 0
        if start is not None and count >= minimum:
            hits.append((base_rva + start, count, s["name"]))
    return hits


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--exe", default="client-patched/MapleStory.exe")
    ap.add_argument("--min", type=int, default=64)
    ap.add_argument("--top", type=int, default=40)
    ap.add_argument("--show")
    ap.add_argument("--count", type=int, default=64)
    args = ap.parse_args()

    data, image_base, sections = load_pe(args.exe)
    lo, hi = text_range(sections)
    starts, ends = parse_pdata(data, sections)

    if args.show:
        rva = int(args.show, 16) - image_base
        for s in sections:
            if s["vaddr"] <= rva < s["vaddr"] + s["rsize"]:
                off = s["raddr"] + (rva - s["vaddr"])
                seen = {}
                for i in range(args.count):
                    v = struct.unpack_from("<I", data, off + i * 4)[0]
                    o = owner(v, starts, ends)
                    seen[v] = seen.get(v, 0) + 1
                    tag = f"FUN_{image_base + o:x}" if o is not None else "-"
                    print(f"  [{i:4}] {image_base + rva + i * 4:#x}  "
                          f"{v:#010x} -> {image_base + v:#x}  in {tag}")
                print(f"\n  {len(seen)} distinct targets in {args.count} slots")
                return 0
        print("not found")
        return 1

    hits = scan(data, sections, lo, hi, args.min)
    hits.sort(key=lambda h: -h[1])
    print(f".text RVA range {lo:#x}-{hi:#x}")
    print(f"{len(hits)} DWORD-RVA runs of >= {args.min} entries, largest first:\n")
    for rva, count, sect in hits[:args.top]:
        # which function owns the *first* target - a switch table's cases all live in
        # the function the switch is in, so this names the owner outright
        first = struct.unpack_from(
            "<I", data,
            next(s["raddr"] + (rva - s["vaddr"]) for s in sections
                 if s["vaddr"] <= rva < s["vaddr"] + s["rsize"]))[0]
        o = owner(first, starts, ends)
        who = f"FUN_{image_base + o:x}" if o is not None else "?"
        size = (ends.get(o, 0) - o) if o is not None else 0
        print(f"  {image_base + rva:#014x}  {count:5} cases   in {who} (size {size})  {sect}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
