#!/usr/bin/env python3
"""Every site that hands an opcode to the client's packet constructor - by BYTES, not by name.

    python tools/builder_scan.py 0x1fd
    python tools/builder_scan.py 0x17e --control

## Why this exists

`research/msexe-send-opcodes.txt` is the enumeration everything else uses, and its own header
says so in capitals: **it is not a complete census.** It resolved 1881 of 1894 call sites, and
an agent re-decoding the byte scan found 52 builders for `0x017E` where that file lists 38.
Its rule is "a hit is [L], a miss is nothing at all".

That rule has a cost. Twice now a feature has been written up as *"the client never sends
this"* on the strength of an absence from that file - and `CLAUDE.md`'s oldest lesson is that
a silent negative is usually a property of the search. This is the second opinion: it does not
read the census, does not need Ghidra, and does not care whether Ghidra ever made a function
at the address.

## How

`FUN_1406ed520(buf, opcode)` is `__fastcall`, so the opcode arrives in `edx` (or `dx`). The
scan walks `.text` for every immediate load of the opcode into that register:

    ba fd 01 00 00        mov edx, 0x1fd
    66 ba fd 01           mov dx, 0x1fd

and then looks forward a short window for a `call rel32` whose target is the constructor. It
prints the site, the distance to the call, and whether the call resolved - so a load that is
*not* followed by a constructor call is reported as such rather than dropped.

## The positive control, and why it is not optional

`--control` runs the same scan for `0x017E` (trade) and requires **at least the 38 sites the
census names**, because every one of those is a decompiled, confirmed builder. A scan that
cannot re-find a known set has no business reporting an empty one. It exits non-zero on
failure and prints which census addresses it missed.

## Blind spots, named

* An opcode that reaches `edx` from a **register or memory** rather than an immediate is
  invisible here. That is the shape the census resolves and this does not, which is why the
  two are complements rather than substitutes.
* A `jmp` to the constructor instead of a `call` is not matched.
* `.text` only.
"""
import os
import re
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from rtti import load_pe  # noqa: E402

DEFAULT_EXE = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                           "client-patched", "MapleStory.exe")

CONSTRUCTOR = 0x1406ED520
WINDOW = 128
CENSUS = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                      "research", "msexe-send-opcodes.txt")


def text_section(exe):
    data, image_base, sections = load_pe(exe)
    for s in sections:
        if s["name"].startswith(".text"):
            return data, image_base, s["vaddr"], s["raddr"], min(s["vsize"], s["rsize"])
    raise SystemExit("no .text section")


def scan(opcode, exe=DEFAULT_EXE):
    """[(site_va, call_va_or_None, distance)] for every immediate load of `opcode` into edx."""
    data, image_base, va, off, size = text_section(exe)
    blob = data[off:off + size]
    pats = [
        (b"\xba" + struct.pack("<I", opcode), 5),
        (b"\x66\xba" + struct.pack("<H", opcode), 4),
    ]
    hits = []
    for pat, width in pats:
        start = 0
        while True:
            i = blob.find(pat, start)
            if i < 0:
                break
            start = i + 1
            site = image_base + va + i
            j = i + width
            end = min(len(blob), j + WINDOW)
            calls = []
            # **Every** constructor call in the window, not the first: one `mov edx` often
            # dominates both arms of a branch, and stopping at the first drops the other. That
            # is exactly the two sites the control missed on 2026-09-22.
            while j < end - 4:
                if blob[j] == 0xE8:
                    rel = struct.unpack("<i", blob[j + 1:j + 5])[0]
                    if image_base + va + j + 5 + rel == CONSTRUCTOR:
                        calls.append((image_base + va + j, j - i))
                j += 1
            if calls:
                hits.extend((site, c, d) for c, d in calls)
            else:
                hits.append((site, None, None))
    return sorted(hits)


def census_sites(opcode):
    """The call addresses `research/msexe-send-opcodes.txt` names for this opcode."""
    if not os.path.exists(CENSUS):
        return set()
    want = "0x%04x" % opcode
    out = set()
    for line in open(CENSUS, encoding="utf-8", errors="replace"):
        if not line.strip().lower().startswith(want):
            continue
        m = re.search(r"call at ([0-9a-fA-F]+)", line)
        if m:
            out.add(int(m.group(1), 16))
    return out


def report(opcode, exe=DEFAULT_EXE, out=sys.stdout):
    hits = scan(opcode, exe)
    built = [h for h in hits if h[1] is not None]
    print("opcode %#06x: %d immediate load(s), %d followed by a constructor call"
          % (opcode, len(hits), len(built)), file=out)
    for site, call, dist in hits:
        if call is None:
            print("  %09x  mov edx, %#x   (no constructor call within %d bytes)"
                  % (site, opcode, WINDOW), file=out)
        else:
            print("  %09x  -> call %09x  (+%d)" % (site, call, dist), file=out)
    named = census_sites(opcode)
    if named:
        found = {c for _, c, _ in built}
        extra = sorted(found - named)
        missing = sorted(named - found)
        print("  census names %d call site(s); this scan finds %d of them, and %d it does not name"
              % (len(named), len(named) - len(missing), len(extra)), file=out)
        for c in extra:
            print("    NOT IN THE CENSUS: call at %09x" % c, file=out)
    return built


def control(out=sys.stdout):
    """0x017E: the scan must re-find every call site the census names."""
    named = census_sites(0x017E)
    if not named:
        print("control FAILED: the census names no 0x017E sites - is research/ present?", file=out)
        return False
    found = {c for _, c, _ in scan(0x017E) if c is not None}
    missing = sorted(named - found)
    print("control: census names %d 0x017E call sites, scan finds %d, misses %d"
          % (len(named), len(named & found), len(missing)), file=out)
    for c in missing:
        print("  MISSED %09x" % c, file=out)
    ok = not missing
    print("control %s" % ("PASS" if ok else "FAILED"), file=out)
    return ok


def main(argv):
    args = [a for a in argv if not a.startswith("--")]
    if "--control" in argv:
        ok = control()
        if not ok:
            return 1
    if not args:
        if "--control" in argv:
            return 0
        print(__doc__)
        return 2
    opcode = int(args[0], 0)
    if not control():
        print("refusing to report: the instrument cannot re-find a known set")
        return 1
    report(opcode)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
