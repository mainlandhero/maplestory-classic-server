#!/usr/bin/env python3
"""Mirror of tools/reads.py for the ENCODE side: every packet field a function writes.

    python encodes.py 0x141cb6880 3

Same transitive helper walk, same tail-`jmp` handling, same .pdata merge - it reuses
tools/reads.py's loader and `calls_of` so the instrument is literally the same one whose
positive control (`python tools/reads.py 0x140304100 2`) is documented.

**Positive control for THIS tool:** `python encodes.py 0x141cb6880 1` must show the
`0x2ff` COutPacket ctor at 141cb7eb1 and a SendPacket at 141cb8365, both of which are read
by hand in research/mob-behaviour.md section 5.
"""
import os
import sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import reads as R

EXE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "client-patched", "MapleStory.exe")

ENC = {
    0x1406ED520: "CTOR",
    0x1406ED840: "w_u8 ",
    0x1406ED940: "w_u16",
    0x1406ED9D0: "w_u32",
    0x1406EDBC0: "w_u64",
    0x1406EDC80: "w_str",
    0x1406EDE20: "w_raw",
    0x1406ED610: "SEND",
}


def reaches(fn, depth, stack=frozenset()):
    """Which encode kinds this subtree touches. Mirrors reads.reaches()."""
    if depth <= 0 or fn in stack or R.extent(fn) is None:
        return []
    out = []
    for _va, tgt, _g in R.calls_of(fn):
        if tgt in ENC:
            out.append(ENC[tgt])
        else:
            out += reaches(tgt, depth - 1, stack | {fn})
    seen, uniq = set(), []
    for k in out:
        if k not in seen:
            seen.add(k)
            uniq.append(k)
    return uniq


def main():
    R._load(EXE)
    fn = int(sys.argv[1], 0)
    depth = int(sys.argv[2], 0) if len(sys.argv) > 2 else 3
    ext = R.extent(fn)
    if ext is None:
        raise SystemExit("no .pdata entry at %#x" % fn)
    print("FUN_%x  %#x..%#x (%d bytes)  depth %d" % (fn, ext[0], ext[1], ext[1] - ext[0], depth))
    for va, tgt, gated in R.calls_of(fn):
        g = "  gated?" if gated else ""
        if tgt in ENC:
            print("  %#x  %s (direct)%s" % (va, ENC[tgt], g))
        else:
            sub = reaches(tgt, depth - 1)
            if sub:
                print("  %#x  call %#x -> WRITES via helper: %s%s" % (va, tgt, " ".join(sub), g))


main()
