#!/usr/bin/env python3
"""Build the drop-table browser: one self-contained HTML page from data/drops.txt.

    python tools/drops_page.py OUT.html

Run it WITH THE REPO AS THE WORKING DIRECTORY, after tools/dump_names.py (names and levels
come from gm-handbook/). The page has every mob's table and the global one, a search by mob
or item, and a drop-rate box that recomputes every chance the way the server does
(crates/world/src/droptables.rs):

* a mob's own rows are multiplied by the rate and capped at 100%;
* the global table is NOT multiplied (v83's rule, `DropTables::roll_at`);
* a mob with no meso row drops level-scaled mesos at 100% (`droptables::level_meso_range`),
  outside the table and outside the rate;
* a quest item (gm-handbook/itemdata.txt `info/quest`) drops only to a player who has its
  quest in progress (`crate::questitems`).
"""

import json
import os
import re
import sys

DROPS = os.path.join("data", "drops.txt")
GM = "gm-handbook"


def read_csv_names(path, col):
    out = {}
    for line in open(path, encoding="utf-8"):
        if line.startswith("#") or not line.strip():
            continue
        p = [x.strip() for x in line.split(",")]
        if p[0].isdigit() and len(p) > col:
            out[int(p[0])] = p[col]
    return out


def category(item_id):
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
    return "etc"


def level_meso_range(level):
    """crates/world/src/droptables.rs `level_meso_range`, the same arithmetic."""
    if level == 0:
        return None
    lo = max(level * 18 // 10, 1)
    return lo, max(level * 22 // 10, lo)


MAGIC_BOX = os.path.join("crates", "world", "src", "magicbox.rs")


def magic_box_lines():
    """First Time Together's reward box (crates/world/src/magicbox.rs), read from the source so the
    page cannot drift from it: every line of ETC, ATTACK_SCROLLS, USE_OTHER and EQUIPS, equal odds."""
    src = open(MAGIC_BOX, encoding="utf-8").read()
    lines = []
    for block in ("ETC", "ATTACK_SCROLLS", "USE_OTHER", "EQUIPS"):
        m = re.search(r"pub const " + block + r": \[[^\]]*\] = \[(.*?)\];", src, re.S)
        if not m:
            sys.exit(f"{MAGIC_BOX}: no {block} table - has the box moved?")
        body = re.sub(r"//[^\n]*", "", m.group(1))
        if block == "ATTACK_SCROLLS":
            lines += [(int(n.replace("_", "")), 1) for n in re.findall(r"\d[\d_]*", body)]
        else:
            lines += [(int(a.replace("_", "")), int(b)) for a, b in re.findall(r"\((\d[\d_]*),\s*(\d+)\)", body)]
    return lines


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: python tools/drops_page.py OUT.html")
    mob_names = read_csv_names(os.path.join(GM, "mobnames.txt"), 1)
    mob_levels = {k: int(v) for k, v in read_csv_names(os.path.join(GM, "mobtemplates.txt"), 3).items() if v.isdigit()}
    item_names = {}
    for line in open(os.path.join(GM, "items.txt"), encoding="utf-8"):
        p = line.rstrip("\n").split(",", 1)
        if p[0].strip().isdigit():
            item_names[int(p[0])] = p[1].strip() if len(p) > 1 else ""
    quest_items = {k for k, v in read_csv_names(os.path.join(GM, "itemdata.txt"), 2).items() if v == "1"}

    mobs, glob, items = {}, [], {}
    for line in open(DROPS, encoding="utf-8"):
        body = line.split("#")[0].strip()
        if not body:
            continue
        c = [x.strip() for x in body.split("|")]
        if len(c) < 5 or not c[1].isdigit():
            continue
        item = int(c[1])
        ppm = int(round(float(c[2]) * 10_000))
        row = [item, ppm, int(c[3]), int(c[4])]
        if item:
            items[item] = [item_names.get(item) or (c[6] if len(c) > 6 else "") or f"Item {item}",
                           category(item), 1 if item in quest_items else 0]
        if c[0] == "*":
            glob.append(row)
        else:
            mobs.setdefault(int(c[0]), []).append(row)

    out_mobs = []
    for tid in sorted(mobs, key=lambda t: (mob_levels.get(t, 0), t)):
        rows = mobs[tid]
        level = mob_levels.get(tid, 0)
        if not any(r[0] == 0 for r in rows):
            rng = level_meso_range(level)
            if rng:
                rows = [[0, 1_000_000, rng[0], rng[1], 1]] + rows  # 5th field: not scaled by the rate
        out_mobs.append({"id": tid, "name": mob_names.get(tid, f"Mob {tid}"), "level": level, "rows": rows})

    # The party quest's reward box, as one more source. Each line is 1 in N per box, the same
    # for every member, and the drop rate does not touch it (5th field 1).
    box = magic_box_lines()
    share = 1_000_000 // len(box)
    for item, q in box:
        items.setdefault(item, [item_names.get(item, f"Item {item}"), category(item), 1 if item in quest_items else 0])
    out_mobs.append({"id": "box", "name": "Companion's Magic Box", "level": 0, "kind": "First Time Together reward",
                     "rows": [[item, share, q, q, 1] for item, q in box]})

    data = {"mobs": out_mobs, "global": glob, "items": {str(k): v for k, v in items.items()}}
    blob = json.dumps(data, separators=(",", ":")).replace("</", "<\\/")
    page = open(os.path.join("crates", "world", "src", "dropweb.html"), encoding="utf-8").read()
    page = page.replace("/*__DATA__*/null", blob)
    with open(sys.argv[1], "w", encoding="utf-8", newline="\n") as f:
        f.write(page)
    total = sum(len(m["rows"]) for m in out_mobs)
    print(f"wrote {sys.argv[1]}: {len(out_mobs)} mobs, {total} rows, {len(glob)} global, {len(items)} items")


if __name__ == "__main__":
    main()
