#!/usr/bin/env python3
"""Enumerate every `call`/`jmp rel32` whose TARGET lands anywhere in a VA range.

`tools/callers.py` answers "who calls THIS function". That question can only be asked
once you already know the function's address, so it searches a known list - the exact
shape `CLAUDE.md` warns about twice. This tool asks the enumerating form instead:

    python tools/callrange.py 0x142c95c00 0x142c96400

"who calls anything at all in the config-accessor cluster" - including accessors nobody
has named yet, which is the whole point. Leaf accessors here are 4-5 byte functions with
no `.pdata` entry, so they cannot be listed any other way.

Output groups by target, then by the `.pdata` function containing each call site.

    --data      also report qword pointers into the range (vtables, jump tables)

POSITIVE CONTROL, run it before believing any empty result:

    python tools/callrange.py 0x1402fa9a0 0x1402fa9a1
        -> 96 call sites, 43 of them in 0x140304b20

which is `tools/callers.py`'s own documented control expressed as a one-address range.

BLIND SPOT, stated because an empty result looks identical to a real absence: this scans
`rel32` forms only. An indirect call through a register or a function pointer loaded at
runtime leaves no trace here, and neither does anything reached only from the
Themida-virtualised dispatcher. "No hits" means "nothing reaches it by a direct relative
branch", never "nothing reaches it".
"""
import bisect
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from rtti import load_pe            # noqa: E402
from dataref import parse_pdata     # noqa: E402

IMAGE_SCN_MEM_EXECUTE = 0x20000000
EXE = os.path.join(os.path.dirname(os.path.abspath(__file__)),
                   "..", "client-patched", "MapleStory.exe")


def scan_rel32_range(data, base, sections, lo, hi, opcode):
    """[(site VA, target VA)] for every `opcode rel32` landing in [lo, hi)."""
    hits = []
    needle = bytes([opcode])
    for s in sections:
        if not (s["chars"] & IMAGE_SCN_MEM_EXECUTE) or not s["rsize"]:
            continue
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        secbase = base + s["vaddr"]
        start = 0
        while True:
            i = blob.find(needle, start)
            if i < 0 or i + 5 > len(blob):
                break
            start = i + 1
            rel = struct.unpack_from("<i", blob, i + 1)[0]
            va = secbase + i
            tgt = va + 5 + rel
            if lo <= tgt < hi:
                hits.append((va, tgt))
    return hits


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("-")]
    if len(args) < 2:
        sys.exit(__doc__)
    lo, hi = int(args[0], 0), int(args[1], 0)
    want_data = "--data" in sys.argv

    data, base, sections = load_pe(EXE)
    funcs = parse_pdata(data, sections, base)      # sorted [(begin, end)]
    starts = [b for b, _ in funcs]
    ends = {b: e for b, e in funcs}

    def owner(va):
        i = bisect.bisect_right(starts, va) - 1
        # .pdata entries for one function can be split; walk back to the one covering va
        while i >= 0:
            st = starts[i]
            if ends[st] > va:
                return st
            i -= 1
        return None

    total = 0
    for opcode, noun in ((0xE8, "call"), (0xE9, "jmp")):
        hits = scan_rel32_range(data, base, sections, lo, hi, opcode)
        total += len(hits)
        by_target = {}
        for site, tgt in hits:
            by_target.setdefault(tgt, []).append(site)
        print(f"=== {noun} rel32 into [{hex(lo)}, {hex(hi)}): "
              f"{len(hits)} site(s), {len(by_target)} distinct target(s)")
        for tgt in sorted(by_target):
            sites = sorted(by_target[tgt])
            print(f"  target {hex(tgt)}   {len(sites)} site(s)")
            by_fn = {}
            for s in sites:
                by_fn.setdefault(owner(s), []).append(s)
            for fn in sorted(by_fn, key=lambda x: (x is None, x)):
                ss = by_fn[fn]
                name = hex(fn) if fn else "(outside any .pdata function)"
                print(f"      in {name:14s} {len(ss):3d}  "
                      f"first {hex(ss[0])}  last {hex(ss[-1])}")
        print()

    if want_data:
        found = []
        for s in sections:
            if not s["rsize"]:
                continue
            blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
            secbase = base + s["vaddr"]
            for i in range(0, len(blob) - 8, 8):
                q = struct.unpack_from("<Q", blob, i)[0]
                if lo <= q < hi:
                    found.append((secbase + i, s["name"], q))
        print(f"=== aligned qword pointers into the range: {len(found)}")
        for slot, sec, q in found[:60]:
            print(f"  {hex(slot)}  [{sec}]  -> {hex(q)}")
        total += len(found)

    if total == 0:
        print("NOTHING FOUND. Run the documented control before concluding absence:")
        print("    python tools/callrange.py 0x1402fa9a0 0x1402fa9a1"
              "   # 96 sites, 43 in 0x140304b20")
        print("This scan sees rel32 branches only - not indirect calls, not the")
        print("Themida-virtualised dispatcher.")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
