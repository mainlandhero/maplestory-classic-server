#!/usr/bin/env python3
"""Decode the client's own error log out of a server log.

The client uploads `0x008F`, `0x0090` and `0x0091` as **plain text**: an `ELog|` record
with the version, the socket, the HR and a symbolic reason, and - in `0x0090` - a full
call stack with module names.

This was invisible for weeks because the server truncated packet bodies at 96 bytes and
these are hundreds to thousands of bytes long. `net::names` now logs anything it cannot
name in full, which is the whole reason this tool can exist.

**HEALTH WARNING, and it inverted a conclusion once already: these records describe an
EARLIER failure, not the run that uploaded them.** `FUN_1415ddd10` opens a log file with
`OPEN_EXISTING`, reads it, closes it and deletes it - the upload is a **file replay**. The
`Time1`/`Time2` fields were stamped by the *writer*, so their few-millisecond spread says
nothing about when the upload happened, and reading it as "16 ms ago" is exactly the trap
that produced a wrong diagnosis on 2026-08-19.

So the reliable protocol is **drain, experiment, re-read**:

1. run once and read the ELog - that is the *previous* failure, and it clears the file;
2. run the experiment;
3. read the ELog again - *now* it is about the run you just did.

A record appearing on the first run after a change tells you about the run before it.

Otherwise it is the cheapest instrument in the project: no client run, no watch slot and no
decompilation, and it reports the client's own verdict in words.

    python tools/decode_elog.py login.log
    python tools/decode_elog.py login.log --stack     # include the call stack
"""
import argparse
import binascii
import re
import sys

# The client's stack frames are the **low 32 bits** of the address, not the address.
#
# The positive control is in the frame itself: a MapleStory.exe frame printed as
# `0000000040194DF5` is annotated by the client as `0001:00193DF5`, section 1 offset
# 0x193DF5, which with `.text` at RVA 0x1000 is VA 0x140194DF5. And 0x140194DF5 masked to
# 32 bits is exactly 0x40194DF5. A USER32 frame printed as `00000000A7F9EF5C` is the same
# masking of a 0x7FF... address - which is why a DLL frame **cannot** be recovered here:
# nothing in the log carries that module's base.
#
# The previous arithmetic added `IMAGE_BASE - (addr & 0xF00000000)`, i.e. 0x140000000 to
# anything below the base, and every VA it printed was 0x40000000 too high. It looked
# plausible and was never checked against the annotation sitting on the same line.
IMAGE_BASE = 0x140000000

# What the client dropped: bits 32 and up. Add it back for image frames only.
HIGH_BITS = 0x100000000

# The one module whose base is known, because it is the image this project reverses.
KNOWN_MODULE = "maplestory.exe"

# Error codes the client names, from docs/client-messages.md.
# Raise sites, by source line. See docs/client-messages.md for the full table.
SITES = {
    735: "FUN_1415d10e0 - the L gate",
    807: "FUN_1415d10e0 - first-connect version check",
    827: "FUN_1415d10e0 - second-connect version check",
    840: "FUN_1415d10e0 - the G == 1 && H == 1 gate",
    846: "FUN_1415d10e0 - cannot access the game",
    1327: "FUN_142c45e50 - a handler re-reporting a caught code, not a new failure",
}

CODES = {
    0x22000001: "cannot access the game",
    0x22000005: "client is outdated (short form)",
    0x22000007: "client is outdated",
    0x2200000C: "connection failed",
}


def records(path):
    pattern = re.compile(r"<- 0x(008F|0090|0091)[^,]*, (\d+) byte body ([0-9a-f]+)")
    for line in open(path, encoding="utf-8", errors="replace"):
        m = pattern.search(line)
        if m:
            yield m.group(1), binascii.unhexlify(m.group(3)).decode("latin1")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("log", nargs="?", default="login.log")
    ap.add_argument("--stack", action="store_true", help="print the call stack too")
    args = ap.parse_args()

    seen = 0
    for opcode, text in records(args.log):
        for line in text.splitlines():
            if line.startswith("ELog|") or "ELog|" in line:
                seen += 1
                print("0x%s  %s" % (opcode, line.strip()))
                for hr in re.findall(r"HR\|(\d+)", line):
                    code = int(hr)
                    name = CODES.get(code, "unknown code")
                    print("        HR %d = %#010x  (%s)" % (code, code, name))
                # The number just before HR is a __LINE__, and docs/client-messages.md
                # has the complete table of raise sites keyed by it.
                for site in re.findall(r"\|(\d+)\|HR\|", line):
                    n = int(site)
                    print("        raised at line %d (%#06x) - see the raise-site table in"
                          " docs/client-messages.md" % (n, n))
                    if n in SITES:
                        print("            %s" % SITES[n])
            elif args.stack and re.match(r"^[0-9A-F]{16} ", line):
                addr = int(line[:16], 16)
                tail = line[34:].strip()
                if KNOWN_MODULE in tail.lower() and addr < HIGH_BITS:
                    where = "VA %#x" % (addr + HIGH_BITS)
                elif addr >= HIGH_BITS:
                    where = "VA %#x" % addr
                else:
                    # A DLL frame. Saying "VA 0x1e7f9ef5c" here would be an invented
                    # number: the log never carries that module's load base.
                    where = "low 32 bits only, base unknown"
                print("        %016X  -> %s  %s" % (addr, where, tail))

    if seen == 0:
        print("no ELog records in %s." % args.log)
        print("Either the client logged none, or the log predates net::names and the")
        print("bodies were truncated at 96 bytes. Check for '0x008F' in the file.")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
