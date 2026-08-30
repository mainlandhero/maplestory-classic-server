#!/usr/bin/env python3
"""Enumerate every packet event in every archived log, deduplicated by EVENT.

Why this exists
---------------
`research/fixtures/` holds COPIES of files that are still in `previous-runs/`, so a
glob over both counts a capture once per name it has. Worse, a fixture is copied
while the run is still being written, so the same run can differ by a few bytes and
therefore HASH DIFFERENTLY - eleven such pairs are known. CLAUDE.md's rule is
explicit: **deduplicate the EVENTS, not the files**, on `(timestamp, opcode, body)`.

This tool is that rule as an instrument. It parses every log line of the shape

    18:25:28.771 <- 0x0070 CLIENT_ENV_REPORT (...), 45 byte body 0264...
    18:25:29.301 -> 0x044F NPC_ENTER_FIELD ...
    18:25:29.301    body e8030000ef0300...          <- body on the NEXT line

into `(timestamp, direction, opcode, body)` and folds identical tuples together,
remembering every file each event was seen in.

Usage, from the repo root (never from a scratchpad - see CLAUDE.md on shadowed
tooling):

    python tools/archive_events.py --census
    python tools/archive_events.py --opcode 0x010E --show
    python tools/archive_events.py --opcode 0x0073 --bodies
    python tools/archive_events.py --grep "connection from" --raw

Exit codes: 0 with events, 1 if the sweep came back empty (that is a broken
instrument, not a clean run).

## `--unredact`, and why counting bodies without it is wrong

Commit `4f6b448` ("Redact the capturing machine from every capture") rewrote every
committed capture, replacing the capturing machine's MAC and the machine id derived
from it with `aabbccddeeff` / `deadbeef`, length-preservingly, in the working tree
and in all 458 commits. `research/fixtures/README.md` says so.

The consequence for THIS tool: a fixture and its `previous-runs/` original are the
same run, and their `0x0073` / `0x007D` bodies **differ**, so event-level dedup on
`(timestamp, opcode, body)` does NOT collapse them. Counting bodies naively reports
**two distinct `0x0073` identity blocks**; there is one, plus its placeholder.

`--unredact` maps the real values onto the placeholders before keying, which is the
direction that loses no information. Use it for any body census; without it, every
count that touches a MAC-bearing opcode is inflated.

Known blind spots, stated so nobody mistakes silence for absence:
  * It only sees what the SERVER logged. A packet the client built and never sent,
    or sent on a socket no server was listening on, cannot appear here.
  * **No archived log records the client's command line.** Which launch flags a run
    used is not recoverable from here at all - only from prose.
  * The redaction commit also converted the fixtures to CRLF, so a fixture is 1 byte
    per line larger than its original. That is a file-level difference only; this
    tool works line by line and is unaffected.
  * Bodies over a cap are logged truncated ("...(+3 bytes, known opcode so capped)").
    The truncated text is part of the key, which is fine for dedup but means a body
    printed here may be short of the wire.
  * Two genuinely distinct events in different runs that share a millisecond
    timestamp, an opcode AND a byte-identical body would collapse into one. For
    per-second-varying bodies that is vanishingly unlikely; for constant tiny bodies
    (e.g. an empty body) it is possible, so `--per-run` reports per-file counts too.
"""

import argparse
import os
import re
import sys
from collections import Counter, defaultdict

LINE_RE = re.compile(r"^(\d\d:\d\d:\d\d\.\d\d\d)\s+(<-|->)\s+0x([0-9A-Fa-f]{4})\b(.*)$")
INLINE_BODY_RE = re.compile(r"(\d+) byte body ([0-9a-fA-F]*)")
NEXT_BODY_RE = re.compile(r"^\d\d:\d\d:\d\d\.\d\d\d\s+body ([0-9a-fA-F]*)\s*$")

LOG_DIRS = ("", "previous-runs", os.path.join("research", "fixtures"))

# The capturing machine's identifiers, and the placeholders commit 4f6b448 put in
# their place throughout `research/fixtures/`. Mapping real -> placeholder makes a
# fixture and its previous-runs original key identically.
REDACTIONS = (
    ("d843ae4c5617", "aabbccddeeff"),
    ("b6ae9cd2", "deadbeef"),
)


def unredact(body):
    for real, placeholder in REDACTIONS:
        body = body.replace(real, placeholder)
    return body


def candidate_logs(root, want):
    """Every log file in the three places runs land, as (path, kind)."""
    out = []
    for rel in LOG_DIRS:
        d = os.path.join(root, rel) if rel else root
        if not os.path.isdir(d):
            continue
        for name in sorted(os.listdir(d)):
            if not name.endswith(".log") and not name.endswith(".txt"):
                continue
            low = name.lower()
            if want and not any(w in low for w in want):
                continue
            path = os.path.join(d, name)
            if os.path.isfile(path):
                out.append(path)
    return out


def parse(path):
    """Yield (timestamp, direction, opcode, body, lineno, headtext)."""
    try:
        fh = open(path, "r", encoding="utf-8", errors="replace")
    except OSError:
        return
    with fh:
        lines = fh.readlines()
    for i, line in enumerate(lines):
        m = LINE_RE.match(line)
        if not m:
            continue
        stamp, arrow, ophex, rest = m.groups()
        opcode = int(ophex, 16)
        body = None
        inline = INLINE_BODY_RE.search(rest)
        if inline:
            body = inline.group(2).lower()
        elif i + 1 < len(lines):
            nxt = NEXT_BODY_RE.match(lines[i + 1])
            if nxt:
                body = nxt.group(1).lower()
        yield stamp, arrow, opcode, (body or ""), i + 1, rest.strip()


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--root", default=os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
    ap.add_argument("--name", action="append", default=[],
                    help="only files whose name contains this (repeatable). default: all")
    ap.add_argument("--opcode", action="append", default=[],
                    help="only this opcode, e.g. 0x010E (repeatable)")
    ap.add_argument("--census", action="store_true", help="count distinct events per opcode")
    ap.add_argument("--show", action="store_true", help="print each distinct event")
    ap.add_argument("--bodies", action="store_true", help="census of distinct bodies")
    ap.add_argument("--per-run", action="store_true", help="also print per-file counts")
    ap.add_argument("--dir", choices=["<-", "->"], help="restrict direction")
    ap.add_argument("--unredact", action="store_true",
                    help="fold the redacted MAC/machine-id onto the placeholder before "
                         "keying, so a fixture and its previous-runs original collapse")
    args = ap.parse_args(argv)

    wanted = set()
    for text in args.opcode:
        wanted.add(int(text, 16) if text.lower().startswith("0x") else int(text, 16))

    logs = candidate_logs(args.root, [n.lower() for n in args.name])
    events = {}
    per_file = Counter()
    files_scanned = 0
    for path in logs:
        saw = 0
        for stamp, arrow, opcode, body, lineno, rest in parse(path):
            if wanted and opcode not in wanted:
                continue
            if args.dir and arrow != args.dir:
                continue
            saw += 1
            if args.unredact:
                body = unredact(body)
            key = (stamp, arrow, opcode, body)
            rec = events.get(key)
            if rec is None:
                events[key] = {"files": [path], "text": rest, "line": lineno}
            else:
                rec["files"].append(path)
        if saw:
            per_file[path] = saw
            files_scanned += 1

    print("scanned %d log files, %d of them carried a matching packet line"
          % (len(logs), files_scanned))
    print("%d distinct events (event-level dedup on timestamp+direction+opcode+body)"
          % len(events))
    print("%d raw event lines before dedup" % sum(per_file.values()))
    print()

    if args.census:
        cens = Counter()
        for (stamp, arrow, opcode, body) in events:
            cens[(arrow, opcode)] += 1
        print("%6s  %-3s %-8s" % ("n", "dir", "opcode"))
        for (arrow, opcode), n in sorted(cens.items(), key=lambda kv: -kv[1]):
            print("%6d  %-3s 0x%04X" % (n, arrow, opcode))
        print()

    if args.bodies:
        cens = Counter()
        for (stamp, arrow, opcode, body) in events:
            cens[(arrow, opcode, body)] += 1
        print("distinct (direction, opcode, body) tuples: %d" % len(cens))
        for (arrow, opcode, body), n in sorted(cens.items(), key=lambda kv: -kv[1]):
            print("%6d  %-3s 0x%04X  len=%-4d %s" % (n, arrow, opcode, len(body) // 2, body))
        print()

    if args.show:
        for key in sorted(events):
            stamp, arrow, opcode, body = key
            rec = events[key]
            names = sorted({os.path.basename(p) for p in rec["files"]})
            print("%s %s 0x%04X  %s" % (stamp, arrow, opcode, rec["text"][:160]))
            print("        body(%d) %s" % (len(body) // 2, body[:200]))
            print("        seen in: %s" % ", ".join(names))
        print()

    if args.per_run:
        for path, n in sorted(per_file.items()):
            print("%6d  %s" % (n, os.path.relpath(path, args.root)))
        print()

    if not events:
        sys.stderr.write("FAIL no events matched - check the filter before believing this\n")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
