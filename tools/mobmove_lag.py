"""How far the 0x02FF path HEAD lags the mob's real position, and three controls.

    python tools/mobmove_lag.py            # from the repo root

Settles: `crates/world/src/dropsite.rs`'s claim that the head of a client mob-move report
is where the walk BEGAN, and that the mob's position now is the last path element's
destination at element offset 1.  A drop placed on the head is one whole path stale.

Two rules from CLAUDE.md are built in rather than left to the caller:

* **Deduplicate the EVENTS, not the files.**  `research/fixtures/` holds copies of
  `previous-runs/` files, and eleven known pairs are the same run with different bytes,
  because a fixture is copied while the run is still being written.  Counting files
  double-counts; counting `(timestamp, opcode, body)` does not.
* **Exit non-zero on an empty result**, so a search that found nothing fails loudly
  instead of printing a clean, confident zero.

Every claim it prints is [L] - read off archived captures, no client run.
"""

import glob
import os
import re
import struct
import sys
from collections import defaultdict

# `<- 0x02FF ... <hex body>` at the end of a world.log line.
LINE = re.compile(r"^(\d\d:\d\d:\d\d\.\d+)\s+<-\s+0x02FF\b.*?\b([0-9a-f]{40,})\s*$")

#: `net::mobmove::MOB_PATH_HEAD_LEN` - u32, i16 x, i16 y, u16, u16, i16 count.
PATH_HEAD_LEN = 14
#: `net::mobmove::MOB_PATH_ELEMENT_LEN`.
PATH_ELEMENT_LEN = 21
#: `world::dropsite::PATH_ELEMENT_XY_AT` - the element opens with one byte, then i16 x, i16 y.
ELEMENT_XY_AT = 1


def parse(body):
    """`net::mobmove::parse_mob_move`, plus the elements it re-emits verbatim."""
    p = 0

    def need(n):
        nonlocal p
        if p + n > len(body):
            raise ValueError("short body")
        v = body[p:p + n]
        p += n
        return v

    obj = struct.unpack_from("<I", need(4))[0]
    move_id = struct.unpack_from("<H", need(2))[0]
    need(1)                       # packed
    need(1)                       # moveAction
    need(8)
    need(2)
    n1 = need(1)[0]
    need(n1 * 4)
    n2 = need(1)[0]
    need(n2 * 2)
    if struct.unpack_from("<I", need(4))[0] != 0:
        need(11 * 4)
    need(1 + 5 * 4 + 1)

    start = p
    need(4)
    x = struct.unpack_from("<h", need(2))[0]
    y = struct.unpack_from("<h", need(2))[0]
    need(4)
    count = struct.unpack_from("<h", need(2))[0]
    if count < 0:                 # 1404b26a9 is signed; the client bails on JLE
        raise ValueError("negative element count")
    elems = [need(PATH_ELEMENT_LEN) for _ in range(count)]
    # The identity that makes the walk above more than arithmetic that happened to fit.
    if p - start != PATH_HEAD_LEN + PATH_ELEMENT_LEN * count:
        raise ValueError("path length identity failed")
    return obj, move_id, x, y, count, elems


def element_xy(e):
    return struct.unpack_from("<hh", e, ELEMENT_XY_AT)


def load():
    """Every distinct 0x02FF event, grouped by (file, mob) so the order is one mob's own."""
    files = sorted(glob.glob("previous-runs/world*.log") + glob.glob("research/fixtures/*.log"))
    seen, per_file, raw, unparsed = set(), defaultdict(lambda: defaultdict(list)), 0, 0
    for f in files:
        try:
            text = open(f, "r", encoding="utf-8", errors="replace").read()
        except OSError:
            continue
        for line in text.splitlines():
            m = LINE.match(line.strip())
            if not m:
                continue
            raw += 1
            key = (m.group(1), "0x02FF", m.group(2))
            if key in seen:
                continue
            seen.add(key)
            try:
                rec = parse(bytes.fromhex(m.group(2)))
            except (ValueError, struct.error):
                unparsed += 1
                continue
            per_file[f][rec[0]].append(rec)
    return files, raw, len(seen), unparsed, per_file


def main():
    if not os.path.isdir("crates"):
        sys.exit("run this from the repo root: python tools/mobmove_lag.py")
    files, raw, distinct, unparsed, per_file = load()
    print("archived logs scanned                : %d" % len(files))
    print("raw 0x02FF lines                     : %d" % raw)
    print("distinct (timestamp, opcode, body)   : %d   <- %d were fixture copies"
          % (distinct, raw - distinct))
    print("bodies that would not parse          : %d" % unparsed)
    if distinct == 0:
        sys.exit("FOUND NOTHING. An empty result here is a broken instrument, not a finding.")

    pairs = matched = head_unchanged = 0
    walked = walked_matched = 0
    multi = first_is_head = tail_repeat = 0
    lag_x, lag_y = [], []
    for mobs in per_file.values():
        for recs in mobs.values():
            for r in recs:
                _, _, x, y, count, elems = r
                if count == 0:
                    continue
                ex, ey = element_xy(elems[-1])
                lag_x.append(abs(ex - x))
                lag_y.append(abs(ey - y))
                if count >= 2:
                    multi += 1
                    first_is_head += element_xy(elems[0]) == (x, y)
                    tail_repeat += element_xy(elems[-1]) == element_xy(elems[-2])
            for a, b in zip(recs, recs[1:]):
                if a[4] == 0:
                    continue
                pairs += 1
                hit = element_xy(a[5][-1]) == (b[2], b[3])
                matched += hit
                if (a[2], a[3]) == (b[2], b[3]):
                    head_unchanged += 1
                else:
                    walked += 1
                    walked_matched += hit

    pct = lambda n, d: 100.0 * n / d if d else 0.0
    print("\nTHE CLAIM -- report N's last element is report N+1's head")
    print("  consecutive report pairs for one mob : %d" % pairs)
    print("  head(N+1) == elem[last](N) @ off %d    : %d  (%.1f%%)"
          % (ELEMENT_XY_AT, matched, pct(matched, pairs)))

    print("\nCONTROL A -- is offset %d the step's START rather than its end?" % ELEMENT_XY_AT)
    print("  multi-element paths                  : %d" % multi)
    print("  elem[0] == head                      : %d  (%.1f%%)   [100%% would mean START]"
          % (first_is_head, pct(first_is_head, multi)))
    print("\nCONTROL B -- is the last element a degenerate 'stand still' step?")
    print("  elem[last] == elem[last-1]           : %d  (%.1f%%)"
          % (tail_repeat, pct(tail_repeat, multi)))
    print("\nCONTROL C -- does the match survive on the pairs where the mob really moved?")
    print("  head moved  : %6d pairs, matched %6d  (%.1f%%)"
          % (walked, walked_matched, pct(walked_matched, walked)))
    print("  head still  : %6d pairs" % head_unchanged)

    n = len(lag_x)
    lag_x.sort()
    lag_y.sort()
    print("\nHOW WRONG THE HEAD IS, over %d reports with at least one step" % n)
    for name, v in (("|dx|", lag_x), ("|dy|", lag_y)):
        print("  %s  mean %5.1f  median %4d  p90 %4d  p99 %5d  max %d"
              % (name, sum(v) / n, v[n // 2], v[int(n * .90)], v[int(n * .99)], v[-1]))
    over = sum(1 for d in lag_x if d > 25)
    print("  |dx| > 25 px, half the client's own pick-up box: %d  (%.1f%%)"
          % (over, pct(over, n)))

    if pairs == 0 or matched == 0:
        sys.exit("no consecutive pairs matched - the decode or the log format has moved")


main()
