#!/usr/bin/env python3
"""Per-function `lea` string sweep over a VA range, with this binary's splice bytes removed.

    python tools/clusterstrings.py 0x140c8f000 0x140c9d000

## Why not `ripstrings.py` over the same range

Two blind spots, both of which produced an empty result on this exact neighbourhood:

1. **A linear sweep desyncs.** Disassembling 60 KB from an arbitrary start walks into
   jump tables and alignment padding and never resynchronises, so `lea` instructions
   after the first desync are simply not seen. This walks **one `.pdata` function at a
   time**, each from a real entry point.
2. **The strings are spliced.** `docs/ghidra.md` records that this client splices CR/TAB/LF
   into its strings to defeat search. `"[ROYAL Connector].exe"` is stored as
   `5b 0d 52 0d 09 4f 0d 59 ...`, so an "is this printable ASCII" test rejects it at byte 2.
   Bytes 0x09/0x0a/0x0d are stripped before the readability test, and the raw form is
   printed too so nothing is hidden by the cleaning.

## Verify it before you believe it

    python tools/clusterstrings.py 0x140c9b800 0x140c9ba80

must print `Royal.Secure.Runtime.dll` and `[ROYAL Connector].exe`. Those are the positive
control for both fixes at once: they are past where a linear sweep desyncs *and* spliced.

## Blind spot

Still `lea`-only. A string reached through a pointer table or `mov reg, imm64` is invisible.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from rtti import load_pe  # noqa: E402
from dataref import parse_pdata  # noqa: E402
from dis_at import va_to_off  # noqa: E402

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64  # noqa: E402
except ImportError:
    sys.exit("capstone not installed")

DEFAULT_EXE = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                           "client-patched", "MapleStory.exe")
SPLICE = {0x09, 0x0a, 0x0d}
RIP = re.compile(r"rip ([+-]) (0x[0-9a-f]+)")


def clean(raw):
    return bytes(b for b in raw if b not in SPLICE)


def readable(raw, minlen=5):
    body = raw.split(b"\0")[0]
    if not body:
        return None
    c = clean(body)
    if len(c) < minlen:
        return None
    if any(b < 0x20 or b >= 0x7f for b in c):
        return None
    return c


def main():
    if len(sys.argv) < 3:
        sys.exit(__doc__)
    lo, hi = int(sys.argv[1], 0), int(sys.argv[2], 0)
    data, ib, secs = load_pe(DEFAULT_EXE)
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    fns = [f for f in parse_pdata(data, secs, ib) if lo <= f[0] < hi]
    print("; %d .pdata functions in 0x%x..0x%x" % (len(fns), lo, hi))
    total = 0
    for start, end in fns:
        off, _ = va_to_off(start, ib, secs)
        if off is None:
            continue
        found = []
        for ins in md.disasm(data[off:off + (end - start)], start):
            if ins.mnemonic != "lea" or "rip" not in ins.op_str:
                continue
            m = RIP.search(ins.op_str)
            if not m:
                continue
            d = int(m.group(2), 16) * (1 if m.group(1) == "+" else -1)
            tgt = ins.address + ins.size + d
            toff, tsect = va_to_off(tgt, ib, secs)
            if toff is None:
                continue
            s = readable(data[toff:toff + 128])
            if s:
                found.append((ins.address, tgt, s))
        if found:
            print("FUN_%x:" % start)
            for a, t, s in found:
                print("   %012x -> 0x%x  %r" % (a, t, s.decode("ascii")))
                total += 1
    print("; %d string references" % total)


if __name__ == "__main__":
    main()
