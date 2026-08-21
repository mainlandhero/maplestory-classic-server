#!/usr/bin/env python3
"""Decode a ***** DUMP ***** line out of client-patched/maplecw-hook.log.

    python tools/decode_dump.py                       # every dump in the current run
    python tools/decode_dump.py --width 8             # read it as u64s (the default)
    python tools/decode_dump.py --exp-curve           # print it as an EXP table
    python tools/decode_dump.py some/other/hook.log

Run it WITH THE REPO AS THE WORKING DIRECTORY.

WHY THIS EXISTS
===============
Some of the client's tables have no bytes on disk. The EXP curve at 0x143AC2400 - 121 u64s,
the experience needed for levels 1..120 - lives in the zero-initialised tail of `.data`, so
a static read cannot produce it: `.data` has vsize 0xa2aa8 and rsize 0x67400, and an address
past the raw bytes maps arithmetically into `.pdata`. Reading it that way returned 120
confident wrong numbers that were exception-handling records, which is why tools/rtti.py now
raises instead of answering.

A running client has the real table. `dump=<VA>/<len>` on a probe watch copies it into the
hook log; this turns that line back into numbers.
"""

import argparse
import os
import re
import struct
import sys

LINE = re.compile(r"\*{5} DUMP (\d+) bytes at (0x[0-9a-fA-F]+): ([0-9a-fA-F]+) \*{5}")
DEFAULT_LOG = os.path.join("client-patched", "maplecw-hook.log")


def decode(raw, width):
    fmt = {1: "<B", 2: "<H", 4: "<I", 8: "<Q"}[width]
    return [struct.unpack_from(fmt, raw, i)[0] for i in range(0, len(raw) - width + 1, width)]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("log", nargs="?", default=DEFAULT_LOG)
    ap.add_argument("--width", type=int, default=8, choices=[1, 2, 4, 8])
    ap.add_argument("--exp-curve", action="store_true",
                    help="print as 'level N needs X' and sanity-check that it rises")
    args = ap.parse_args()

    try:
        with open(args.log, encoding="utf-8", errors="replace") as fh:
            text = fh.read()
    except FileNotFoundError:
        print("%s not found. Run the client first, or pass a path." % args.log, file=sys.stderr)
        return 1

    found = LINE.findall(text)
    if not found:
        print("No DUMP lines in %s.\n"
              "A -SetFieldProbe run dumps the EXP curve on the 140304100 watch's first hit,\n"
              "so if that watch produced no lines either, the hook never armed and the run\n"
              "proves nothing - which is exactly what the positive control is for."
              % args.log, file=sys.stderr)
        return 1

    for n, (length, at, hexs) in enumerate(found, 1):
        raw = bytes.fromhex(hexs)
        print("dump %d: %s bytes at %s" % (n, length, at))
        if len(raw) != int(length):
            print("  SHORT: the line claims %s bytes and carries %d" % (length, len(raw)))
        values = decode(raw, args.width)

        if args.exp_curve:
            # **Entry i is level i's requirement, and entry 0 is a hole.** The comment that
            # stood here said "Entry 0 is level 1's requirement" and the loop below labelled
            # accordingly with `enumerate(values, 1)` - so every level printed one too high,
            # and comparing the output against data/exp-curve.txt showed all 98 levels
            # disagreeing when in fact all 99 agree exactly. An instrument that answers
            # confidently and wrongly, which is the failure CLAUDE.md is mostly about.
            #
            # The measured shape settles it, 2026-08-21: 121 entries, [0] = 0, [1] = 15,
            # [119] = 28171993, [120] = 0. Level 0 does not exist (the hole that makes the
            # index 1-based, the same arrangement as inventory slot 0), and level 120 is the
            # cap, so it has no next level. 1 hole + levels 1..120 = 121, and 2 zeros leaves
            # exactly the 119 non-zero entries the file reports. data/exp-curve.txt's level 1
            # is 15, independently sourced, and it lands on index 1.
            nonzero = sum(1 for v in values if v)
            # Check the rise over the POPULATED range only. Including the trailing cap zero
            # made this print "monotonic: NO - suspect" on a table that is strictly rising -
            # a false alarm on every dump this tool has ever taken.
            body = [v for v in values if v]
            rising = all(b > a for a, b in zip(body, body[1:]))
            print("  %d entries, %d non-zero, rising across the populated range: %s"
                  % (len(values), nonzero, "yes" if rising else "NO - suspect"))
            if not nonzero:
                print("  EVERY ENTRY IS ZERO. The table was not populated when the dump ran,\n"
                      "  or the address is wrong. This is the failure the dump exists to make\n"
                      "  visible rather than to paper over.")
            for lvl, v in enumerate(values):
                if v:
                    print("  level %3d -> %d" % (lvl, v))
        else:
            for i, v in enumerate(values):
                print("  [%3d] %d (%#x)" % (i, v, v))
    return 0


if __name__ == "__main__":
    sys.exit(main())
