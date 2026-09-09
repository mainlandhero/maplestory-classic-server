#!/usr/bin/env python3
"""Every rip-relative `lea` in a VA range, with the target decoded as a string if it is one.

    python tools/ripstrings.py 0x1428923e0 21926

## Why

"No strings are referenced in this function" is a **negative**, and this repo's oldest rule
is that a negative needs an instrument that can produce a positive. Grepping a listing for
a quoted string cannot: the listing prints `lea rcx, [rip + 0x2e48f7e]` and nothing else.
This resolves the target, reads bytes there, and prints them as ASCII and UTF-16 when they
look like text - so a run that prints nothing has been *asked* the question.

## Verify it before you believe it

**THE CONTROL BELOW IS STALE AND HAS BEEN SINCE AT LEAST 2026-09-09. DO NOT USE IT.**

    python tools/ripstrings.py 0x141b2c7c0 934      # <- BROKEN CONTROL, always silent

It says this "must print at least one decoded string". It cannot: that range contains
**zero rip-relative `lea` instructions at all** -

    python tools/dis_at.py 0x141b2c7c0 934 | grep -c "lea.*rip"   ->  0

so it can never speak whatever the tool's state. Two agents hit it the same day and drew
*different* conclusions from the same silence - one called the tool broken and discarded its
output unread, one called the control stale. The second was right, and the first cost real
work: the discarded output was the pass that would have attributed the beauty-UI packet.

**A control that cannot produce a positive is worse than no control**, because "it printed
nothing" then looks like a verdict about the binary. That is this repo's oldest rule and this
is it happening to a control rather than to a search.

**No replacement control is offered yet, deliberately.** Four functions were tried
(`0x141C3D3E0`, `0x140d73bf0`, `0x141826bc0`, `0x141E99700`); the tool resolved `lea` targets
in three of them and decoded **no text in any**, every hit landing in `.data` with no raw
bytes. That is consistent with this client's strings being **encrypted and decrypted at
runtime** - which is why `tools/clusterstrings.py` exists - rather than with the tool being
broken. Until someone finds a function that loads a PLAINTEXT string through a rip-relative
`lea`, this tool has no verified positive control and **a silent result from it means
nothing in either direction.** Prefer `clusterstrings.py`.

## Blind spots

* A string reached by `mov reg, imm64` or through a pointer table is invisible here.
* This binary **splices control characters into strings** to defeat search
  (`docs/ghidra.md`), so a partial decode is still a hit.
* `.themida`/`.boot` are zero on disk: a target there prints as unmapped, which is a
  property of the file.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from rtti import load_pe  # noqa: E402
from dis_at import va_to_off  # noqa: E402

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64  # noqa: E402
except ImportError:
    sys.exit("capstone not installed")

DEFAULT_EXE = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                           "client-patched", "MapleStory.exe")

PRINTABLE = set(range(0x20, 0x7f)) | {9, 10, 13}


def _ascii(data, off, limit=96):
    out = bytearray()
    for b in data[off:off + limit]:
        if b == 0:
            break
        if b not in PRINTABLE:
            return None if len(out) < 4 else bytes(out)
        out.append(b)
    return bytes(out) if len(out) >= 4 else None


def _utf16(data, off, limit=192):
    out = bytearray()
    for i in range(0, limit, 2):
        lo, hi = data[off + i], data[off + i + 1]
        if lo == 0 and hi == 0:
            break
        if hi != 0 or lo not in PRINTABLE:
            return None if len(out) < 4 else bytes(out)
        out.append(lo)
    return bytes(out) if len(out) >= 4 else None


def main():
    if len(sys.argv) < 3:
        sys.exit(__doc__)
    va = int(sys.argv[1], 0)
    length = int(sys.argv[2], 0)
    data, image_base, sections = load_pe(DEFAULT_EXE)
    off, sect = va_to_off(va, image_base, sections)
    if off is None:
        sys.exit("0x%x has no raw bytes (section %s)" % (va, sect))
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = False
    hits = 0
    for ins in md.disasm(data[off:off + length], va):
        if ins.mnemonic != "lea" or "rip" not in ins.op_str:
            continue
        try:
            disp = ins.op_str.split("rip +")[1].split("]")[0].strip()
            tgt = ins.address + ins.size + int(disp, 16)
        except (IndexError, ValueError):
            try:
                disp = ins.op_str.split("rip -")[1].split("]")[0].strip()
                tgt = ins.address + ins.size - int(disp, 16)
            except (IndexError, ValueError):
                continue
        toff, tsect = va_to_off(tgt, image_base, sections)
        if toff is None:
            print(" %012x  -> 0x%x  [%s: no raw bytes]" % (ins.address, tgt, tsect))
            hits += 1
            continue
        a = _ascii(data, toff)
        w = _utf16(data, toff)
        if a:
            print(" %012x  -> 0x%x  (%s)  A %r" % (ins.address, tgt, tsect, a))
            hits += 1
        elif w:
            print(" %012x  -> 0x%x  (%s)  W %r" % (ins.address, tgt, tsect, w))
            hits += 1
    print("; %d lea targets decoded as text in 0x%x..0x%x" % (hits, va, va + length))


if __name__ == "__main__":
    main()
