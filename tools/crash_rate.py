"""The crash rate, from the SERVER, without asking a player for anything.

`crates/world/src/server.rs` logs one verdict per channel session:

    {peer} closed                 the socket was closed gracefully - a logout, a channel
                                  change, or a return to character select
    {peer} ended: <error>         the read failed. `os error 10054` is
                                  WSAECONNRESET: the process died underneath the socket

So a client that CRASHES leaves `10054` and a client that QUITS leaves `closed`, and the
difference is already in every archived `world*.log`. That makes a crash rate free: no hook
log, no dump, nothing asked of the player, and it works for people whose machines we will
never see.

**Why this file exists rather than a grep.** Three things this repo has been caught by:

* `research/fixtures/` holds COPIES of `previous-runs/`, so a glob over both counts a capture
  once per name it has. Worse, a fixture is copied while the run is still being written, so
  the pair differ by a few bytes and a content hash calls them two files. This deduplicates
  **the verdict lines themselves**, which is the only level at which the answer is stable.
* A search that finds nothing is usually a broken search. This refuses to report a rate until
  it has found BOTH outcomes, and prints the control.
* `closed` also covers Change Channel and character select, and `10054` also covers a
  task-kill or a network drop. This is a proxy and the output says so.

Usage:  python tools/crash_rate.py [--by-date]
"""

import hashlib
import os
import re
import sys
from collections import defaultdict

ROOTS = ['previous-runs', 'research/fixtures', '.']
CRASH = re.compile(r'ended:.*?os error 10054')
ENDED_OTHER = re.compile(r'ended:')
CLOSED = re.compile(r'\bclosed$')
STAMP = re.compile(r'^(\d{2}):(\d{2}):(\d{2})')
# world-20260907-024031.log -> 20260907
DATED = re.compile(r'world(?:-ch\d+)?-(\d{8})-\d{6}\.log$')


def world_logs():
    seen = set()
    for root in ROOTS:
        if not os.path.isdir(root):
            continue
        for name in sorted(os.listdir(root)):
            if not name.startswith('world') or not name.endswith('.log'):
                continue
            p = os.path.join(root, name)
            if os.path.isfile(p) and p not in seen:
                seen.add(p)
                yield p


def main():
    by_date = '--by-date' in sys.argv
    verdicts = {}          # hash of the verdict line -> (kind, date)
    files = 0
    for path in world_logs():
        files += 1
        m = DATED.search(os.path.basename(path))
        date = m.group(1) if m else 'current'
        try:
            fh = open(path, encoding='utf-8', errors='replace')
        except OSError:
            continue
        with fh:
            for line in fh:
                line = line.rstrip('\n')
                if 'ended:' not in line and not line.endswith('closed'):
                    continue
                if CRASH.search(line):
                    kind = 'crash'
                elif ENDED_OTHER.search(line):
                    kind = 'other-error'
                elif CLOSED.search(line):
                    kind = 'clean'
                else:
                    continue
                # Deduplicate the EVENT, not the file. Two copies of one run share the
                # timestamped line verbatim.
                key = hashlib.md5(line.encode('utf-8', 'replace')).hexdigest()
                verdicts.setdefault(key, (kind, date))

    counts = defaultdict(int)
    per_date = defaultdict(lambda: defaultdict(int))
    for kind, date in verdicts.values():
        counts[kind] += 1
        per_date[date][kind] += 1

    print('%d world log(s) read, %d distinct session verdict(s)' % (files, len(verdicts)))
    print()
    print('  crashed  (ended: ... os error 10054) : %d' % counts['crash'])
    print('  clean    (... closed)                : %d' % counts['clean'])
    print('  other    (ended: some other error)   : %d' % counts['other-error'])

    # THE CONTROL. A rate computed from a search that can only find one outcome is not a rate.
    if not counts['crash'] or not counts['clean']:
        print()
        print('REFUSING TO REPORT A RATE: the search found only one kind of verdict, which is')
        print('the shape of a broken search rather than of a healthy server. Fix the patterns.')
        return 1

    total = counts['crash'] + counts['clean'] + counts['other-error']
    print()
    print('  CRASH RATE: %d of %d ended sessions = %.1f%%' % (counts['crash'], total, 100.0 * counts['crash'] / total))
    print()
    print('  Both outcomes were found, so the search discriminates. Read it as a PROXY:')
    print('  "closed" also covers Change Channel and character select; 10054 also covers a')
    print('  task-kill or a network drop. What it is good for is the SAME number before and')
    print('  after a change, over comparable play.')

    if by_date:
        print()
        print('  %-10s %7s %7s %7s   %s' % ('date', 'crash', 'clean', 'other', 'rate'))
        for date in sorted(per_date):
            d = per_date[date]
            t = d['crash'] + d['clean'] + d['other-error']
            rate = ('%.0f%%' % (100.0 * d['crash'] / t)) if t else '-'
            print('  %-10s %7d %7d %7d   %s' % (date, d['crash'], d['clean'], d['other-error'], rate))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
