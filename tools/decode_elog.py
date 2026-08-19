#!/usr/bin/env python3
"""Decode the client's own error log out of a server log.

The client uploads `0x008F`, `0x0090` and `0x0091` as **plain text**: an `ELog|` record
with the version, the socket, the HR and a symbolic reason, and - in `0x0090` - a full
call stack with module names.

This was invisible for weeks because the server truncated packet bodies at 96 bytes and
these are hundreds to thousands of bytes long. `net::names` now logs anything it cannot
name in full, which is the whole reason this tool can exist.

**It is the cheapest instrument in the project.** It costs no client run, no watch slot
and no decompilation, and it reports the client's own verdict in words.

    python tools/decode_elog.py login.log
    python tools/decode_elog.py login.log --stack     # include the call stack
"""
import argparse
import binascii
import re
import sys

# The client reports raw addresses with the top image-base nibble dropped.
IMAGE_BASE = 0x140000000

# Error codes the client names, from docs/client-messages.md.
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
            elif args.stack and re.match(r"^[0-9A-F]{16} ", line):
                addr = int(line[:16], 16)
                # The client prints VA - 0x100000000; put the image base back.
                va = addr + (IMAGE_BASE - (addr & 0xF00000000)) if addr < IMAGE_BASE else addr
                print("        %016X  -> VA %#x  %s" % (addr, va, line[34:].strip()))

    if seen == 0:
        print("no ELog records in %s." % args.log)
        print("Either the client logged none, or the log predates net::names and the")
        print("bodies were truncated at 96 bytes. Check for '0x008F' in the file.")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
