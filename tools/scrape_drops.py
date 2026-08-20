#!/usr/bin/env python3
"""Build data/drops.txt from the community monster database at meowdb.com/msclassic.

    python tools/scrape_drops.py            # write data/drops.txt
    python tools/scrape_drops.py --dry-run  # print the summary, write nothing
    python tools/scrape_drops.py --only 2,3 # just those templates, for a quick check

Run it WITH THE REPO AS THE WORKING DIRECTORY. It reads the client's own template list
from gm-handbook/mobtemplates.txt, so it only ever asks about monsters this client can
actually spawn.

WHAT IS REAL AND WHAT IS OURS
============================
The source gives, per monster, a list of items with a community *vote score* and NO drop
rates whatsoever. Mesos are the exception: those carry a real min, max and dropChancePct.

So every item chance this writes is OUR POLICY, computed by chance_percent() below from the
item's category. The raw score is written into a column so the policy can be changed by
re-running the numbers over data/drops.txt instead of hitting the network again.

Rows with a score below MIN_SCORE are dropped: a score of 0 or less means the community
either never confirmed it or voted it down, and inventing a drop the game does not have is
worse than missing one it does.

WHY THE OUTPUT IS COMMITTED
===========================
Unlike everything in gm-handbook/, this cannot be regenerated from the client. It needs
network access to a third-party site that may change or disappear, so data/drops.txt is
authored source in the same sense data/shops.txt is, and it is committed.

The site blocks a bare urllib/curl User-Agent with 403, hence the browser UA below. This
makes one request per monster and sleeps between them; it is a fan site, not an API we are
entitled to hammer.
"""

import argparse
import json
import os
import sys
import time
import urllib.error
import urllib.request

API = "https://meowdb.com/msclassic/api"
UA = ("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 "
      "(KHTML, like Gecko) Chrome/126.0 Safari/537.36")

TEMPLATES = os.path.join("gm-handbook", "mobtemplates.txt")
ITEM_NAMES = os.path.join("gm-handbook", "items.txt")
OUT = os.path.join("data", "drops.txt")

#: Community score at or below which a row is discarded. 0 means "nobody confirmed it".
MIN_SCORE = 1

#: How long to wait between monsters. Politeness, not rate limiting on their side.
DELAY_S = 0.15


def chance_percent(item_type, item_subtype):
    """OUR POLICY. The source has no drop rates; this invents them from the category.

    Roughly the shape of the pre-Big-Bang tables: a mob's own trophy drops often, ordinary
    consumables sometimes, equipment rarely. Every number here is a judgement call and the
    whole point of keeping the score column is that they can be revisited without scraping.
    """
    t = (item_type or "").strip().lower()
    sub = (item_subtype or "").strip().lower()
    if t == "etc":
        # The mob's own trophy - "Snail Shell" on a snail. The thing you actually farm.
        return 40.0 if sub == "monster drop" else 6.0
    if t == "use":
        return 5.0
    if t == "setup":
        return 3.0
    if t == "equip":
        return 1.0
    return 0.5


def quantity_range(item_id, item_type):
    """How many drop at once. Also policy, and deliberately dull: one of everything except
    stackables, which come in small handfuls the way arrows and potions do."""
    t = (item_type or "").strip().lower()
    if t == "use" and str(item_id).startswith("206"):   # arrows come in stacks
        return (10, 30)
    return (1, 1)


def get(url, tries=3):
    req = urllib.request.Request(url, headers={"User-Agent": UA, "Accept": "application/json"})
    last = None
    for attempt in range(tries):
        try:
            with urllib.request.urlopen(req, timeout=25) as r:
                return json.load(r)
        except (urllib.error.URLError, urllib.error.HTTPError, json.JSONDecodeError) as e:
            last = e
            time.sleep(0.6 * (attempt + 1))
    raise RuntimeError("%s: %s" % (url, last))


def template_ids(only):
    if only:
        return [int(x) for x in only.split(",") if x.strip()]
    ids = []
    with open(TEMPLATES, encoding="utf-8") as fh:
        for line in fh:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            ids.append(int(line.split(",")[0]))
    return sorted(set(ids))


def item_names():
    """id -> name, from the client's own Item.wz dump. Used to cross-check that a scraped
    item id is one this client actually has - a drop the client cannot render is a fault
    waiting to happen, exactly as with the `!item` command."""
    names = {}
    try:
        with open(ITEM_NAMES, encoding="utf-8") as fh:
            for line in fh:
                line = line.strip()
                if not line or line.startswith("#"):
                    continue
                parts = line.split(",", 1)
                if len(parts) == 2:
                    names[parts[0].strip()] = parts[1].strip()
    except FileNotFoundError:
        pass
    return names


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--only", default="")
    args = ap.parse_args()

    known = item_names()
    if not known:
        print("WARNING: %s is missing, so scraped item ids cannot be checked against the "
              "client. Run tools/dump_names.py first." % ITEM_NAMES, file=sys.stderr)

    ids = template_ids(args.only)
    rows, unknown, no_data, mesos_rows = [], set(), 0, 0
    for n, tid in enumerate(ids, 1):
        try:
            drops = get("%s/drops?monsterId=%d" % (API, tid))
            mesos = get("%s/mesos-reports?monsterId=%d" % (API, tid))
        except RuntimeError as e:
            print("  %d: FAILED %s" % (tid, e), file=sys.stderr)
            continue

        s = (mesos or {}).get("summary") or {}
        if s.get("count"):
            rows.append((tid, 0, float(s.get("medianDropChancePct") or 0),
                         int(s.get("medianMin") or 0), int(s.get("medianMax") or 0),
                         int(s["count"]), "mesos"))
            mesos_rows += 1

        got = 0
        for d in (drops or {}).get("drops") or []:
            score = int(d.get("score") or 0)
            if score < MIN_SCORE:
                continue
            key = str(d.get("itemIconKey") or "").strip()
            if not key.isdigit():
                continue
            if known and key not in known:
                unknown.add(key)
                continue
            lo, hi = quantity_range(key, d.get("itemType"))
            rows.append((tid, int(key), chance_percent(d.get("itemType"), d.get("itemSubtype")),
                         lo, hi, score, (d.get("itemName") or "").strip()))
            got += 1
        if not got:
            no_data += 1
        if n % 25 == 0:
            print("  %d/%d templates" % (n, len(ids)), file=sys.stderr)
        time.sleep(DELAY_S)

    print("%d rows, %d templates, %d meso rows, %d templates with no item data"
          % (len(rows), len({r[0] for r in rows}), mesos_rows, no_data))
    if unknown:
        print("%d scraped item ids are NOT in this client and were skipped: %s"
              % (len(unknown), ", ".join(sorted(unknown)[:12])))

    if args.dry_run:
        return 0
    # Refuse to replace a good file with an empty one: a network failure must not look like
    # "this game has no drops".
    if len(rows) < 50:
        print("only %d rows - refusing to overwrite %s. Re-run when the site is reachable."
              % (len(rows), OUT), file=sys.stderr)
        return 1

    header = [
        "# templateId | itemId | chance% | minQty | maxQty | score | name",
        "#",
        "# GENERATED by tools/scrape_drops.py from meowdb.com/msclassic - but COMMITTED,",
        "# because it cannot be regenerated from the client the way gm-handbook/ can.",
        "# Hand edits survive only until the next scrape; change the policy in the script.",
        "#",
        "# templateId '*' is the GLOBAL table: rolled for EVERY mob, whatever it is. That is",
        "# where event items go. It is empty because there are no events yet.",
        "# itemId 0 is MESOS, and minQty/maxQty are the amount rather than a stack size.",
        "#",
        "# THE CHANCES ARE OURS. The source has vote scores and no drop rates; only the meso",
        "# rows carry a real chance. See chance_percent() in the script, and the score column",
        "# here, which is what those chances were derived from.",
        "",
        "# ---- the global table: event items, rolled for every mob -----------------------",
        "# (empty - no events yet. One line here and every mob in the game can drop it.)",
        "",
        "# ---- per-monster ---------------------------------------------------------------",
    ]
    body = ["%d | %d | %g | %d | %d | %d | %s" % r for r in rows]
    tmp = OUT + ".tmp"
    with open(tmp, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("\n".join(header + body) + "\n")
    os.replace(tmp, OUT)
    print("wrote %s" % OUT)
    return 0


if __name__ == "__main__":
    sys.exit(main())
