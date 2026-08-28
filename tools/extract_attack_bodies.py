#!/usr/bin/env python3
"""Pull every client attack packet body out of the captured world logs.

The client's attack opcodes are 0x00DF (melee), 0x00E0 (shoot) and 0x00E1 (magic).
All three share one body format - the log line for 0x00E1 literally says
"same body as 0x00DF". `crates/net/src/attack.rs` parses that body; this script is
what feeds it real bytes.

Run it from the repo root so it picks up `tools/` and not a scratchpad copy:

    python tools/extract_attack_bodies.py --dedupe --summary
    python tools/extract_attack_bodies.py --dedupe --out corpus.txt

Default inputs, in this order:

    world*.log                       the live run, if one is sitting there
    previous-runs/world*.log         the rolling archive
    research/fixtures/*world*.log    the kept runs

Every world-log line that carries a body looks like

    04:01:06.251 <- 0x00E1 CLIENT_MAGIC_ATTACK (...), 260 byte body 00016b881e...

so the hex is matched off `([0-9]+) byte body ([0-9a-f]+)` and cross-checked
against the stated length. A line whose hex length disagrees with its stated byte
count is reported rather than silently kept - that mismatch has never been seen,
and if it ever appears the log format changed underneath us.

Output format, one record per line, tab separated:

    <opcode hex>  <length>  <source file>  <timestamp>  <body hex>

which is what `attack.rs`'s corpus sweep reads. Point that sweep at a file with

    set MAPLECW_ATTACK_CORPUS=C:\\MapleCW\\corpus.txt
    cargo test -p net attack

## --dedupe, and why you almost always want it

`research/fixtures/` holds **copies** of files that are still in `previous-runs/`,
and scanning both counts the same session twice. Four such pairs exist today and
one capture is present **three** times (fixture, second fixture, and the live
`world.log`), which is how `research/attack-skill-id.md` came to report 14 Magic
Claw casts when the client sent 7.

`--dedupe` hashes each input file and skips one whose bytes have already been
seen, naming what it dropped. It does **not** de-duplicate individual bodies:
two identical bodies in one session would be a finding, not noise.

`damage-formula.md` section 3 already flagged one of these pairs by hand. This
makes it the default question rather than something to remember.
"""

import argparse
import hashlib
import os
import re
import sys
from collections import Counter

# `NNN byte body <hex>` - the length is stated separately from the hex, which
# gives a free consistency check on every single record.
BODY_RE = re.compile(r"(\d+) byte body ([0-9a-fA-F]+)")
# `04:01:06.251 <- 0x00E1 ...`. The direction arrow matters: `<-` is client to
# server. An attack is only ever client to server.
LINE_RE = re.compile(r"^(\d\d:\d\d:\d\d\.\d\d\d)\s+(<-|->)\s+0x([0-9A-Fa-f]{4})\b")

# 0x00E2 is deliberately absent: research/mob-combat.md 1.2 shows it uses the
# classic inline shape and never calls the shared header builder FUN_140f31fe0.
ATTACK_OPCODES = (0x00DF, 0x00E0, 0x00E1)


def default_inputs(root):
    """Every world log this repo can see, newest sources last."""
    out = []
    for name in sorted(os.listdir(root)):
        if name.startswith("world") and name.endswith(".log"):
            out.append(os.path.join(root, name))
    prev = os.path.join(root, "previous-runs")
    if os.path.isdir(prev):
        for name in sorted(os.listdir(prev)):
            if name.startswith("world") and name.endswith(".log"):
                out.append(os.path.join(prev, name))
    fixtures = os.path.join(root, "research", "fixtures")
    if os.path.isdir(fixtures):
        for name in sorted(os.listdir(fixtures)):
            if "world" in name and name.endswith(".log"):
                out.append(os.path.join(fixtures, name))
    return out


def scan(path, opcodes, problems):
    """Yield (opcode, timestamp, body_hex) for every attack packet in one log."""
    with open(path, "r", encoding="utf-8", errors="replace") as fh:
        for lineno, line in enumerate(fh, 1):
            head = LINE_RE.match(line)
            if head is None:
                continue
            stamp, arrow, op_text = head.groups()
            opcode = int(op_text, 16)
            if opcode not in opcodes:
                continue
            if arrow != "<-":
                problems.append(
                    "%s:%d: attack opcode 0x%04X sent server->client" % (path, lineno, opcode)
                )
                continue
            body = BODY_RE.search(line)
            if body is None:
                problems.append("%s:%d: attack line with no body hex" % (path, lineno))
                continue
            stated = int(body.group(1))
            hexbytes = body.group(2).lower()
            if len(hexbytes) % 2 or len(hexbytes) // 2 != stated:
                problems.append(
                    "%s:%d: stated %d bytes, hex carries %d"
                    % (path, lineno, stated, len(hexbytes) // 2)
                )
                continue
            yield opcode, stamp, hexbytes


def field_u32(hexbytes, off):
    """Little-endian u32 at a body byte offset, or None if the body is short."""
    if len(hexbytes) < (off + 4) * 2:
        return None
    raw = bytes.fromhex(hexbytes[off * 2 : (off + 4) * 2])
    return int.from_bytes(raw, "little")


def field_u8(hexbytes, off):
    if len(hexbytes) < (off + 1) * 2:
        return None
    return int(hexbytes[off * 2 : off * 2 + 2], 16)


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("logs", nargs="*", help="log files to scan (default: every world log)")
    ap.add_argument("--out", help="write the corpus here instead of stdout")
    ap.add_argument(
        "--summary",
        action="store_true",
        help="print the (opcode, skill id, skill level, length) census instead of bodies",
    )
    ap.add_argument(
        "--dedupe",
        action="store_true",
        help="skip an input whose bytes are identical to one already scanned",
    )
    ap.add_argument(
        "--root",
        default=os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
        help="repo root (default: the parent of tools/)",
    )
    args = ap.parse_args(argv)

    inputs = args.logs or default_inputs(args.root)
    if not inputs:
        sys.stderr.write("no logs to scan\n")
        return 2

    problems = []
    records = []
    per_file = Counter()
    seen_hashes = {}
    for path in inputs:
        if not os.path.isfile(path):
            problems.append("%s: not a file" % path)
            continue
        if args.dedupe:
            with open(path, "rb") as fh:
                digest = hashlib.sha256(fh.read()).hexdigest()
            if digest in seen_hashes:
                sys.stderr.write(
                    "DUPE %s is byte-identical to %s - skipped\n" % (path, seen_hashes[digest])
                )
                continue
            seen_hashes[digest] = path
        for opcode, stamp, hexbytes in scan(path, ATTACK_OPCODES, problems):
            records.append((opcode, stamp, path, hexbytes))
            per_file[path] += 1

    if args.summary:
        census = Counter()
        for opcode, _stamp, path, hexbytes in records:
            census[
                (
                    opcode,
                    field_u32(hexbytes, 2),
                    field_u8(hexbytes, 6),
                    len(hexbytes) // 2,
                )
            ] += 1
        print("%5s  %-8s %-10s %-6s %s" % ("n", "opcode", "skill", "level", "body len"))
        for key in sorted(census, key=lambda k: (-census[k], k)):
            opcode, skill, level, length = key
            print(
                "%5d  0x%04X   %-10s %-6s %d"
                % (census[key], opcode, skill, level, length)
            )
        print()
        print("%d bodies from %d files" % (len(records), len(per_file)))
        for path in sorted(per_file):
            print("  %5d  %s" % (per_file[path], path))
    else:
        lines = [
            "0x%04X\t%d\t%s\t%s\t%s" % (op, len(hx) // 2, path, stamp, hx)
            for op, stamp, path, hx in records
        ]
        text = "\n".join(lines) + ("\n" if lines else "")
        if args.out:
            with open(args.out, "w", encoding="utf-8") as fh:
                fh.write(text)
            sys.stderr.write("%d bodies -> %s\n" % (len(records), args.out))
        else:
            sys.stdout.write(text)

    for note in problems:
        sys.stderr.write("WARN %s\n" % note)
    # An empty result is a failure, not a clean run. CLAUDE.md: prove the
    # instrument can find a positive control before believing its silence.
    if not records:
        sys.stderr.write("FAIL no attack bodies found - check the inputs\n")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
