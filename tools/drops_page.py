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
    if 2040000 <= item_id < 2050000 or item_id == 2530000:  # Lucky Day: a scroll, outside the 204 block
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

#: The four backported scrolls' rules on this server, for their tooltips. A copy of
#: `tip_lines` in crates/world/src/dropweb.rs, which is what the served page uses - keep them
#: together.
BACKPORTED_RULES = {
    2049100: (60, "Changes one of the item's own stats by -5 to +5, never 0. Uses an upgrade slot whether it "
                  "succeeds or fails; a failed slot can be restored with a Clean Slate."),
    2049000: (1, "Restores one upgrade slot lost to a failed scroll."),
    2049001: (3, "Restores one upgrade slot lost to a failed scroll."),
    2049002: (5, "Restores one upgrade slot lost to a failed scroll."),
    2049003: (20, "Restores one upgrade slot lost to a failed scroll."),
    2049190: (70, "Returns the item to its original stats and all of its upgrade slots. Does not use an upgrade slot."),
    2530000: (100, "The next scroll used on the item succeeds and cannot destroy it - Chaos, Clean Slate and "
                   "Innocence included. Does not use an upgrade slot."),
}

#: gm-handbook/equips.txt columns -> the client's tooltip labels, in the order the client draws them.
EQUIP_LINES = [("incSTR", "STR"), ("incDEX", "DEX"), ("incINT", "INT"), ("incLUK", "LUK"), ("incMHP", "MaxHP"),
               ("incMMP", "MaxMP"), ("incWAT", "Weapon Attack"), ("incMAD", "Magic Attack"), ("incPDD", "Weapon Def."),
               ("incMDD", "Magic Def."), ("incACC", "Accuracy"), ("incEVA", "Evasion"), ("incSpeed", "Speed"),
               ("incJump", "Jump")]


def tooltip_lines():
    """item -> the lines under the description (dropweb.rs `tip_lines`)."""
    out = {}
    path = os.path.join(GM, "equips.txt")
    cols = None
    for line in open(path, encoding="utf-8"):
        if line.startswith("# itemId"):
            cols = [c.strip() for c in line[2:].split(",")]
            continue
        if line.startswith("#") or not line.strip() or cols is None:
            continue
        p = dict(zip(cols, [x.strip() for x in line.split(",")]))
        num = lambda k: int(p.get(k) or 0) if (p.get(k) or "0").lstrip("-").isdigit() else 0
        lines = []
        if num("reqLevel"):
            lines.append(f"REQ LEV : {num('reqLevel')}")
        lines += [f"{label} : +{num(k)}" for k, label in EQUIP_LINES if num(k) > 0]
        if num("tuc"):
            lines.append(f"Number of upgrades available : {num('tuc')}")
        if num("tradeBlock"):
            lines.append("Untradeable")
        out[int(p["itemId"])] = lines
    for item, (pct, rule) in BACKPORTED_RULES.items():
        out[item] = [f"Success rate: {pct}%", f"On this server: {rule}"]
    return out


def whereabouts(item_names):
    """dropweb.rs `Whereabouts`: template -> [[map, name, spawn points]] (most first), and
    template -> the other ways it appears (the ship invasion, a summoning sack)."""
    counts = {}
    for line in open(os.path.join(GM, "mobs.txt"), encoding="utf-8"):
        p = [x.strip() for x in line.split(",")]
        if p[0].isdigit() and len(p) > 1 and p[1].isdigit():
            per = counts.setdefault(int(p[1]), {})
            per[int(p[0])] = per.get(int(p[0]), 0) + 1
    map_names = read_csv_names(os.path.join(GM, "maps.txt"), 1)
    maps_of = {t: [[m, map_names.get(m, f"Map {m}"), n] for m, n in sorted(per.items(), key=lambda kv: (-kv[1], kv[0]))]
               for t, per in counts.items()}
    also_of = {700005: ["Invades the ship between Ellinia and Orbis"]}
    for line in open(os.path.join(GM, "summonsacks.txt"), encoding="utf-8"):
        p = [x.strip() for x in line.split(",")]
        if not p[0].isdigit() or len(p) < 5:
            continue
        seen = []
        for part in p[4].split(";"):
            t = part.split(":")[0]
            if t.isdigit() and int(t) not in seen:
                seen.append(int(t))
                also_of.setdefault(int(t), []).append(f"Summoned by {item_names.get(int(p[0]), 'Item ' + p[0])}")
    return maps_of, also_of


def descriptions():
    """gm-handbook/itemdesc.txt (tools/dump_names.py): id -> the client's text, \\n kept as two characters."""
    out = {}
    path = os.path.join(GM, "itemdesc.txt")
    if os.path.exists(path):
        for line in open(path, encoding="utf-8"):
            ident, _, desc = line.rstrip("\n").partition(", ")
            if ident.isdigit():
                out[int(ident)] = desc
    return out


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

    maps_of, also_of = whereabouts(item_names)
    out_mobs = []
    for tid in sorted(mobs, key=lambda t: (mob_levels.get(t, 0), t)):
        if tid not in maps_of and tid not in also_of:
            continue  # on no map and brought no other way: hidden, as the served page does
        rows = mobs[tid]
        level = mob_levels.get(tid, 0)
        if not any(r[0] == 0 for r in rows):
            rng = level_meso_range(level)
            if rng:
                rows = [[0, 1_000_000, rng[0], rng[1], 1]] + rows  # 5th field: not scaled by the rate
        out_mobs.append({"id": tid, "name": mob_names.get(tid, f"Mob {tid}"), "level": level,
                         "maps": maps_of.get(tid, []), "also": also_of.get(tid, []), "rows": rows})

    # The party quest's reward box, as one more source. Each line is 1 in N per box, the same
    # for every member, and the drop rate does not touch it (5th field 1).
    box = magic_box_lines()
    share = 1_000_000 // len(box)
    for item, q in box:
        items.setdefault(item, [item_names.get(item, f"Item {item}"), category(item), 1 if item in quest_items else 0])
    out_mobs.append({"id": "box", "name": "Companion's Magic Box", "level": 0, "kind": "First Time Together reward",
                     "rows": [[item, share, q, q, 1] for item, q in box]})

    # The tooltip: the client's description and the lines under it, as the served page has them.
    descs, lines = descriptions(), tooltip_lines()
    for item, entry in items.items():
        entry[3:] = [descs.get(item, ""), lines.get(item, [])]

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
