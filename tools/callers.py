#!/usr/bin/env python3
"""Every `call rel32` to a function, attributed to its containing `.pdata` function.

`xref.py --va` matches code that takes an address (`lea`, `mov imm64`); `dataref.py`
matches code that reads or writes a global. Neither matches a direct CALL, so both report
**0 references** for a function with dozens of call sites - and on 2026-08-19 that clean
zero was very nearly believed. Verify with a positive control before trusting a negative:

    python tools/callers.py 0x1402fa9a0     # expect 43, all in 0x140304b20
    python tools/callers.py 0x140302e30
"""
import bisect
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rtti import load_pe          # noqa: E402
from dataref import parse_pdata   # noqa: E402


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 1
    target = int(sys.argv[1], 16)
    exe = sys.argv[2] if len(sys.argv) > 2 else "client-patched/MapleStory.exe"

    data, base, sections = load_pe(exe)
    funcs = parse_pdata(data, sections, base)
    starts = [f[0] for f in funcs]

    def containing(va):
        i = bisect.bisect_right(starts, va) - 1
        return funcs[i][0] if i >= 0 and funcs[i][0] <= va < funcs[i][1] else None

    hits = []
    for s in sections:
        if not (s["chars"] & 0x20000000) or not s["rsize"]:
            continue
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        secbase = base + s["vaddr"]
        for i in range(len(blob) - 5):
            if blob[i] != 0xE8:                     # call rel32
                continue
            rel = struct.unpack_from("<i", blob, i + 1)[0]
            va = secbase + i
            if va + 5 + rel == target:
                hits.append(va)

    by_fn = {}
    for va in hits:
        by_fn.setdefault(containing(va), []).append(va)

    print("%#x: %d call site(s) in %d function(s)" % (target, len(hits), len(by_fn)))
    for fn in sorted(by_fn, key=lambda f: (f is None, f or 0)):
        rows = by_fn[fn]
        home = ("%#x" % fn) if fn else "(outside any .pdata function)"
        print("    %-14s %3d call(s)   first %#x  last %#x" % (home, len(rows), rows[0], rows[-1]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
