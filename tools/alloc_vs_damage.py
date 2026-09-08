#!/usr/bin/env python3
"""Pair every pool-sentry finding with the nearest preceding ZArray allocation.

    python tools/alloc_vs_damage.py previous-runs research/fixtures

The `WATCH ... 0x140ca61d0 ENTERED ... rdx=0x5` line is the anti-cheat module's 180 s
allocation; `POOL SENTRY` findings/repairs are the damage the sentry noticed. If the
allocation *causes* the damage, every finding has an allocation a few tens of milliseconds
before it and the allocations with no finding after them are rare. If the two are merely
both periodic, the gap wanders.

## Deduplicate the EVENTS, not the files

`CLAUDE.md`: `research/fixtures/` holds **copies** of `previous-runs/` files, and eleven
known pairs are the same run yet hash differently, because a fixture is copied while the
run is still being written. So a file hash is not enough. Events are keyed on
`(timestamp, kind, detail)` across every file given, and the counts printed are of distinct
events. Both the raw line count and the deduplicated count are printed, because the ratio
between them is the thing that has been wrong before.

## Verify it before you believe it

The run5 fixture is known to contain both kinds of line. If either count comes back zero
for a directory that contains `write-watch-run3-*-hook.log`, the patterns below no longer
match the hook's output and every number here is worthless.
"""
import os
import re
import sys
from datetime import datetime, timedelta

ALLOC = re.compile(r"^(\d\d:\d\d:\d\d\.\d+).*WATCH #\d+: 0x140ca61d0 ENTERED.*rdx=0x([0-9a-f]+)")
FIND = re.compile(r"^(\d\d:\d\d:\d\d\.\d+).*POOL SENTRY (?:REPAIR|FINDING)[: ].*?(0x[0-9a-f]+)")


def parse(t):
    return datetime.strptime(t, "%H:%M:%S.%f")


def main():
    roots = sys.argv[1:] or ["previous-runs", "research/fixtures"]
    allocs, finds = {}, {}
    raw_a = raw_f = 0
    for root in roots:
        if not os.path.isdir(root):
            continue
        for name in sorted(os.listdir(root)):
            if "hook" not in name or not name.endswith(".log"):
                continue
            path = os.path.join(root, name)
            with open(path, "r", errors="replace") as fh:
                for line in fh:
                    m = ALLOC.match(line)
                    if m:
                        raw_a += 1
                        allocs[(m.group(1), m.group(2))] = None
                        continue
                    m = FIND.match(line)
                    if m:
                        raw_f += 1
                        finds[(m.group(1), m.group(2))] = None
    A = sorted(parse(t) for t, _ in allocs)
    F = sorted((parse(t), a) for t, a in finds)
    print("allocations: %d raw lines -> %d distinct events" % (raw_a, len(A)))
    print("findings   : %d raw lines -> %d distinct events" % (raw_f, len(F)))
    if not A or not F:
        print("ZERO on one side - the patterns no longer match the hook's output. "
              "Do not read anything into this.")
        return
    gaps = []
    for t, addr in F:
        prev = [a for a in A if a <= t and t - a < timedelta(seconds=30)]
        if prev:
            gaps.append(((t - prev[-1]).total_seconds(), addr))
    print("findings with an allocation within 30 s before: %d of %d" % (len(gaps), len(F)))
    if gaps:
        vals = sorted(g for g, _ in gaps)
        print("  gap  min %.3f s  median %.3f s  max %.3f s"
              % (vals[0], vals[len(vals) // 2], vals[-1]))
        for g, addr in gaps[:12]:
            print("   %+.3f s  damaged %s" % (g, addr))


if __name__ == "__main__":
    main()
