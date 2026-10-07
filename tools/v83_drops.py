#!/usr/bin/env python3
"""Bring data/drops.txt to the v83 drop tables - chances, quest gates and missing items.

    python tools/v83_drops.py --report     # what would change, write nothing
    python tools/v83_drops.py              # rewrite data/drops.txt

Run it WITH THE REPO AS THE WORKING DIRECTORY, after tools/dump_names.py has generated
gm-handbook/ (it reads mobnames.txt, mobtemplates.txt and items.txt from there).

WHERE THE NUMBERS COME FROM
===========================
The owner, 2026-10-01: "Please make them closer to v83 drop rates and drop behavior ... If
there's an item in our server not dropped but the old sources have them as dropping as such,
add them to the drop list."

data/drops.txt was scraped from meowdb, which has NO drop rates - every chance in it was a flat
per-category policy (tools/scrape_drops.py). The v83 baseline is Cosmic, the maintained
OdinMS -> HeavenMS server (github.com/P0nk/Cosmic), at a PINNED commit so a rerun is
reproducible: its drop_data SQL (chance out of 1 000 000, questid for quest-only drops) and its
String.wz/Mob.img.xml and Mob.wz/<id>.img.xml for names and levels. Neither Cosmic nor anybody
else has official per-mob rates - Nexon never published them - so this is a calibration, the
closest classic one there is. research/drop-rates-vs-v83-2026-10-01.md has the comparison.

THE RULES
=========
* A template is matched to a Cosmic mob BY NAME (this client numbers its mobs its own way),
  preferring the Cosmic id at the same level, then the one with the most rows. A name with no
  Cosmic mob is "unmatched".
* For a matched template, every Cosmic row whose item exists in this client is taken with
  Cosmic's chance and quantity (an equip is always one: Cosmic's multi-equip copies are an
  option it ships, not classic behaviour). Rows meowdb had and Cosmic does not are KEPT, at the
  v83 median for their category (computed from Cosmic here, not typed in). Cosmic's meso row,
  if it has one, replaces meowdb's.
* An unmatched template keeps its meowdb rows, at the same category medians.
* Every line from the top of the file down to the "per-monster" marker, and from the Dark
  Marble marker to the end, is copied VERBATIM - the global table and the hand-authored
  exceptions are the owner's decisions, not data. So is any per-monster item row at 100% (the
  Ligator's party-quest coupon): no scraped or v83 row is ever 100%, only a hand edit is.
* A Cosmic row naming an item this client does not have is skipped and counted.
* A Cosmic QUEST-ONLY row (questid != 0) is skipped unless meowdb already had the item, in which
  case it keeps v83's chance and drops for everyone as before. Measured 2026-10-01: all ten
  such rows on matched mobs name a quest this client's Quest.wz does not have, so a gate would
  never open.
* An item this client flags as a QUEST item (itemdata.txt) is added only when one of this
  client's own quests (quests.json) mentions its id. Otherwise it is a quest item for a quest
  that does not exist here - junk that cannot even be sold.
* Templates 800000 and up - the party quest's and the second-job tests' own copies of field
  mobs - are copied verbatim. Their drops are set by hand and by code (session/combat.rs).

The score column: meowdb's vote score where meowdb had the row, 0 for a row that only v83 has.
"""

import argparse
import collections
import json
import os
import re
import statistics
import sys
import time
import urllib.request

COSMIC_COMMIT = "fec53bc7714dc0f1ae3f50b2986cdf2727e0912a"
RAW = f"https://raw.githubusercontent.com/P0nk/Cosmic/{COSMIC_COMMIT}/"
CACHE = os.path.join(".cache", "cosmic-" + COSMIC_COMMIT[:12])
UA = "Mozilla/5.0 (MapleCW drop calibration; one request per file, cached)"

DROPS = os.path.join("data", "drops.txt")
MOB_NAMES = os.path.join("gm-handbook", "mobnames.txt")
MOB_TEMPLATES = os.path.join("gm-handbook", "mobtemplates.txt")
ITEMS = os.path.join("gm-handbook", "items.txt")
ITEM_DATA = os.path.join("gm-handbook", "itemdata.txt")
SCROLLS = os.path.join("gm-handbook", "scrolls.txt")
QUESTS = os.path.join("gm-handbook", "quests.json")

PER_MONSTER = "# ---- per-monster"

# **Every equip, use and etc chance a monster drops is DOUBLED from the v83 base** - the owner,
# 2026-10-05: "For all equipment, consumable, and etc items that comes from monsters, can we double
# all of the base rate?" Applied to the chances this script takes from v83 and to the medians it
# fills in, capped at certainty. Rows it copies verbatim (pinned 100% rows, the party-quest
# templates, meso rows) are read from the file, which already carries the doubling, so a re-run is
# idempotent. Setup items (3xxxxxx) are not in the request and stay at the base.
BASE_MULTIPLIER = 2
DOUBLED_CATEGORIES = {"equip", "use", "scroll", "trophy", "etc"}


def boosted(item, ppm):
    """A v83 or median chance with the owner's multiplier applied."""
    if category(item) not in DOUBLED_CATEGORIES:
        return ppm
    return min(ppm * BASE_MULTIPLIER, 1_000_000)
MARBLE = "# ---- THE SIX DARK MARBLE ROWS"

def fetch(path):
    """One file from Cosmic at the pinned commit, cached under .cache/."""
    local = os.path.join(CACHE, path.replace("/", os.sep))
    if not os.path.exists(local):
        os.makedirs(os.path.dirname(local), exist_ok=True)
        req = urllib.request.Request(RAW + path, headers={"User-Agent": UA})
        with urllib.request.urlopen(req, timeout=60) as r:
            data = r.read()
        with open(local, "wb") as f:
            f.write(data)
        time.sleep(0.05)
    with open(local, encoding="utf-8") as f:
        return f.read()


def category(item_id):
    """The categories the medians are taken over. Trophies are the 4000xxx monster parts."""
    if item_id == 0:
        return "mesos"
    if item_id < 2000000:
        return "equip"
    if 2040000 <= item_id < 2050000:
        return "scroll"
    if item_id < 3000000:
        return "use"
    if item_id < 4000000:
        return "setup"
    if item_id < 4001000:
        return "trophy"
    return "etc"


def load_cosmic():
    sql = fetch("src/main/resources/db/data/152-drop-data.sql")
    rows = collections.defaultdict(list)
    for m in re.finditer(r"\((\d+),\s*(\d+),\s*(-?\d+),\s*(-?\d+),\s*(\d+),\s*(\d+)\)", sql):
        mob, item, lo, hi, quest, chance = map(int, m.groups())
        if chance <= 0:
            continue
        rows[mob].append({"item": item, "lo": max(lo, 1) if item else lo, "hi": hi, "quest": quest, "ppm": chance})
    names = collections.defaultdict(list)
    xml = fetch("wz/String.wz/Mob.img.xml")
    for m in re.finditer(r'<imgdir name="(\d+)">\s*<string name="name" value="([^"]*)"', xml):
        names[m.group(2).strip().lower()].append(int(m.group(1)))
    return rows, names


def norm(name):
    """Names compared without case, spacing or punctuation: "Cape Magic Def. Scroll" == "cape magic def scroll"."""
    return re.sub(r"[^a-z0-9]", "", name.lower())


def load_cosmic_item_names():
    """GMS item id -> name, from Cosmic's String.wz."""
    out = {}
    for img in ("Eqp", "Consume", "Etc", "Ins", "Cash", "Pet"):
        xml = fetch(f"wz/String.wz/{img}.img.xml")
        for m in re.finditer(r'<imgdir name="(\d+)">\s*<string name="name" value="([^"]*)"', xml):
            out[int(m.group(1))] = m.group(2).strip()
    return out


SCROLL_STATS = ["incSTR", "incDEX", "incINT", "incLUK", "incMHP", "incMMP", "incPAD", "incMAD",
                "incPDD", "incMDD", "incACC", "incEVA", "incSpeed", "incJump"]


def scroll_map():
    """v83 scroll id -> this client's scroll id, by WHAT THE SCROLL DOES.

    This client renamed every scroll ("Dagger Attack Scroll: Intermediate" for v83's "Scroll for
    Dagger for ATT"), reused ids for different scrolls (its 2040002 is a Hat Accuracy scroll,
    v83's a Helmet DEF one) and cut each scroll to one stat - so neither the id nor the name can
    pair them. What both keep is the equipment type (the id's first five digits), the main stat
    (the largest bonus, HP/MP counted at a tenth) and the tier. This client has four tiers per
    scroll - 100%, 60%, 10%, and a 10% that destroys on failure half the time - so v83's
    non-cursed 100/60/10 pair on the rate, and v83's cursed dark scrolls (30%/70%) pair with
    the cursed tier. Anything else (15%, 65%, 5%...) and any scroll restricted to one item or
    untradeable (an event scroll) is left out. 242 of v83's 754 pair, none ambiguously.
    """
    xml = fetch("wz/Item.wz/Consume/0204.img.xml")
    v83 = {}
    for m in re.finditer(r'<imgdir name="(0?204\d{4})">(.*?)</imgdir>\s*</imgdir>', xml, re.S):
        v83[int(m.group(1))] = {k: int(x) for k, x in re.findall(r'<int name="(\w+)" value="(-?\d+)"', m.group(2))}
    cw = {}
    for line in open(SCROLLS, encoding="utf-8"):
        p = [x.strip() for x in line.split(",")]
        if p[0].isdigit():
            # scrolls.txt: success, cursed, then the stats in SCROLL_STATS order (its incWAT is incPAD)
            cw[int(p[0])] = dict(zip(["success", "cursed"] + SCROLL_STATS, map(int, p[1:17])))

    def primary(d):
        scored = [(d.get(k, 0) / (10 if k in ("incMHP", "incMMP") else 1), k) for k in SCROLL_STATS if d.get(k, 0) > 0]
        return max(scored)[1] if scored else None

    def key(i, d):
        cursed = d.get("cursed", 0) > 0
        return (i // 100, "cursed" if cursed else d.get("success", 0), primary(d))

    index = collections.defaultdict(list)
    for i, d in cw.items():
        index[key(i, d)].append(i)
    out = {}
    for i, d in v83.items():
        if d.get("tradeBlock") or "0" in d:
            continue  # an event scroll: untradeable, or for one named item only
        if not d.get("cursed") and d.get("success") not in (100, 60, 10):
            continue
        found = index.get(key(i, d), [])
        if len(found) == 1:
            out[i] = found[0]
    return out


def cosmic_level(mob_id):
    try:
        xml = fetch(f"wz/Mob.wz/{mob_id:07d}.img.xml")
    except Exception:
        return None
    m = re.search(r'<int name="level" value="(\d+)"', xml)
    return int(m.group(1)) if m else None


def read_ours():
    def table(path, key_col, val_col):
        out = {}
        for line in open(path, encoding="utf-8"):
            if line.startswith("#") or not line.strip():
                continue
            p = [x.strip() for x in line.split(",")]
            if p[0].isdigit():
                out[int(p[0])] = p[val_col] if val_col < len(p) else ""
        return out

    names = {k: v for k, v in table(MOB_NAMES, 0, 1).items()}
    levels = {k: int(v) for k, v in table(MOB_TEMPLATES, 0, 3).items() if v.isdigit()}
    items = {}
    for line in open(ITEMS, encoding="utf-8"):
        p = line.rstrip("\n").split(",", 1)
        if p[0].strip().isdigit():
            items[int(p[0])] = p[1].strip() if len(p) > 1 else ""
    return names, levels, items


def split_file():
    lines = open(DROPS, encoding="utf-8").read().splitlines()
    start = next(i for i, l in enumerate(lines) if l.startswith(PER_MONSTER))
    end = next(i for i, l in enumerate(lines) if l.startswith(MARBLE))
    rows = []
    for l in lines[start + 1:end]:
        body = l.split("#")[0].strip()
        if not body:
            continue
        c = [x.strip() for x in body.split("|")]
        rows.append({
            "tid": int(c[0]), "item": int(c[1]), "pct": float(c[2]), "lo": int(c[3]), "hi": int(c[4]),
            "score": int(c[5]) if len(c) > 5 and c[5].lstrip("-").isdigit() else 0,
            "name": c[6] if len(c) > 6 else "",
        })
    return lines[:start + 1], rows, lines[end:]


def fmt_pct(ppm):
    """Parts per million as a percentage with no float noise: 1287 -> 0.1287."""
    whole, frac = divmod(ppm, 10_000)
    return f"{whole}.{frac:04d}".rstrip("0").rstrip(".") if frac else str(whole)


def build():
    cosmic, cnames = load_cosmic()
    gms_items = load_cosmic_item_names()
    names, levels, items = read_ours()
    # **This client renumbered some items.** Its 4000000 is "Jr. Sentinel Shellpiece"; v83's is
    # Blue Snail Shell. So a v83 item id is trusted only when the NAMES agree, and otherwise
    # moved to this client's item of the same name when exactly one exists.
    by_name = collections.defaultdict(list)
    for i, n in items.items():
        by_name[norm(n)].append(i)

    scrolls = scroll_map()
    quest_flagged = set()
    for line in open(ITEM_DATA, encoding="utf-8"):
        p = [x.strip() for x in line.split(",")]
        if p[0].isdigit() and len(p) > 2 and p[2] == "1":
            quest_flagged.add(int(p[0]))

    def regular(candidates):
        """Of several same-name items, the one that is NOT a quest item, when exactly one is.

        The owner, 2026-10-01: *"The non-tutorial Jr. Sentinel should not drop the Jr. Sentinel
        Shellpiece quest item, but rather the regular etc item."* This client has two of that name:
        4000000, the tutorial's quest item, and 4000067, the monster's own part."""
        plain = [i for i in candidates if i not in quest_flagged]
        return plain[0] if len(plain) == 1 else None

    def here(gms_id):
        if 2_040_000 <= gms_id < 2_050_000:
            return scrolls.get(gms_id)  # by what it does - see scroll_map
        if gms_id == 0:
            return 0
        g = norm(gms_items.get(gms_id, ""))
        if not g:
            return None
        if gms_id in items and norm(items[gms_id]) == g:
            return gms_id
        # Same name AND same subtype (id // 10000 - "Egg" is a setup item in one numbering and a
        # use item in the other, and those are different things), and exactly one candidate.
        same = [i for i in by_name.get(g, []) if i // 10_000 == gms_id // 10_000]
        return same[0] if len(same) == 1 else regular(same)

    quest_text = open(QUESTS, encoding="utf-8").read()
    used_by_a_quest = {i for i in quest_flagged if re.search(rf"(?<![0-9]){i}(?![0-9])", quest_text)}

    head, ours, tail = split_file()

    # The medians, per category, over Cosmic's ordinary mobs (ids under 8 000 000, no quest gate).
    by_cat = collections.defaultdict(list)
    for mob, rs in cosmic.items():
        if mob >= 8_000_000:
            continue
        for r in rs:
            if r["quest"] == 0:
                by_cat[category(r["item"])].append(r["ppm"])
    median = {c: int(statistics.median(v)) for c, v in by_cat.items()}

    by_tid = collections.defaultdict(list)
    for r in ours:
        by_tid[r["tid"]].append(r)

    report = collections.Counter()
    matches = {}
    out_rows = []
    for tid in sorted(by_tid):
        rows = by_tid[tid]
        name = names.get(tid, "")
        cands = [m for m in cnames.get(name.lower(), []) if m in cosmic]
        cid = None
        if cands:
            lv = levels.get(tid)
            same = [m for m in cands if cosmic_level(m) == lv] if lv else []
            cid = max(same or cands, key=lambda m: len(cosmic[m]))
            matches[tid] = (cid, bool(same))
        # Hand-set rows: 100% items (the tutorial shellpiece, the Ligator coupon) are the owner's.
        pinned = {r["item"] for r in rows if r["item"] != 0 and r["pct"] >= 100}
        # A row naming a quest item that has a regular twin of the same name means the twin - the
        # community site lists items by name. Only the hand-set rows above the per-monster marker
        # (the tutorial's) ever mean the quest item, and this loop never sees them.
        for r in rows:
            if r["item"] in quest_flagged:
                twin = regular(by_name.get(norm(items.get(r["item"], "")), []))
                if twin is not None and twin != r["item"]:
                    report["quest item replaced by its regular twin"] += 1
                    r["item"], r["name"] = twin, items.get(twin, r["name"])
        mine = {r["item"]: r for r in rows}
        new = []
        if tid >= 800_000:
            for r in rows:
                new.append({**r, "ppm": int(round(r["pct"] * 10_000))})
            report["party-quest/test template, verbatim"] += 1
            new.sort(key=lambda r: (r["item"] != 0, -r["ppm"], r["item"]))
            out_rows.extend(new)
            continue
        if cid is not None:
            v83 = {}
            for r in cosmic[cid]:
                cw = here(r["item"])
                if cw is None:
                    report["v83 item not in this client (by id and name)"] += 1
                    continue
                if cw != r["item"]:
                    report["v83 item renumbered in this client"] += 1
                r = {**r, "item": cw}
                if r["item"] in v83:
                    continue  # a duplicate row in the SQL; the first wins
                if r["quest"] and r["item"] not in mine:
                    report["v83 quest-only row skipped"] += 1
                    continue
                v83[r["item"]] = r
            if 0 in v83:
                report["meso row from v83"] += 1
            for item, r in v83.items():
                if item in pinned:
                    continue
                if item != 0 and item not in items:
                    report["v83 item not in this client"] += 1
                    continue
                had = mine.get(item)
                if had is None and item in quest_flagged and item not in used_by_a_quest:
                    report["v83 quest item no quest here uses, skipped"] += 1
                    continue
                if had is None:
                    report["added from v83"] += 1
                else:
                    report["chance from v83"] += 1
                lo, hi = r["lo"], r["hi"]
                if item and category(item) == "equip":
                    lo, hi = 1, 1  # Cosmic's multi-equip copies are an option it ships switched on; one is classic
                new.append({
                    "tid": tid, "item": item, "ppm": boosted(item, r["ppm"]), "lo": max(lo, 0), "hi": max(hi, lo),
                    "score": had["score"] if had else 0,
                    "name": had["name"] if had else ("mesos" if item == 0 else items.get(item, "")),
                })
            for item, r in mine.items():
                if item in v83 and item not in pinned and (item == 0 or item in items):
                    continue
                if item in pinned:
                    new.append({**r, "ppm": int(round(r["pct"] * 10_000))})
                    continue
                if item == 0 and 0 in v83:
                    continue
                report["meowdb-only, v83 median"] += 1
                new.append({**r, "ppm": r["pct"] * 10_000 if item == 0 else boosted(item, median[category(item)])})
        else:
            for r in rows:
                if r["item"] in pinned or r["item"] == 0:
                    new.append({**r, "ppm": int(round(r["pct"] * 10_000))})
                else:
                    report["unmatched mob, v83 median"] += 1
                    new.append({**r, "ppm": boosted(r["item"], median[category(r["item"])])})
        new.sort(key=lambda r: (r["item"] != 0, -r["ppm"], r["item"]))
        out_rows.extend(new)

    return head, out_rows, tail, matches, median, report, names, levels, items, cosmic


def write(head, rows, tail, median):
    body = list(head)
    body.append("# Chances are v83's (Cosmic " + COSMIC_COMMIT[:12] + ", tools/v83_drops.py) where the mob")
    body.append("# matched by name; otherwise the v83 median for the category: " +
                ", ".join(f"{c} {fmt_pct(v)}%" for c, v in sorted(median.items()) if c != "mesos") + ".")
    body.append("# Every equip, use and etc chance below is DOUBLED from that base, capped at 100% (the owner,")
    body.append("# 2026-10-05; BASE_MULTIPLIER in tools/v83_drops.py). Mesos and setup items are not.")
    for r in rows:
        body.append(f"{r['tid']} | {r['item']} | {fmt_pct(int(round(r['ppm'])))} | {r['lo']} | {r['hi']} | {r['score']} | {r['name']}")
    body.append("")
    body.extend(tail)
    with open(DROPS, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(body).rstrip("\n") + "\n")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--report", action="store_true", help="print what would change and write nothing")
    args = ap.parse_args()
    if not os.path.exists(DROPS) or not os.path.exists(ITEMS):
        sys.exit("run from the repo root, after tools/dump_names.py")
    head, rows, tail, matches, median, report, names, levels, items, cosmic = build()
    print("v83 medians:", {c: fmt_pct(v) + "%" for c, v in sorted(median.items())})
    print(f"matched {len(matches)} templates by name ({sum(1 for _, s in matches.values() if s)} at the same level)")
    for k, v in sorted(report.items()):
        print(f"  {k}: {v}")
    print(f"rows: {len(rows)}")
    if not args.report:
        write(head, rows, tail, median)
        print("wrote", DROPS)


if __name__ == "__main__":
    main()
