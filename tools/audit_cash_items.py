#!/usr/bin/env python3
"""Every cash item in the client's WZ, against what the Cash Shop actually sells.

    python tools/audit_cash_items.py            # writes gm-handbook/cashaudit.txt, prints a summary

Run from anywhere; it chdirs to the repo (via tools/dump_itemdata.py, whose walkers it reuses).

## What "available" means here, and why it is not a guess

The server does not stock the shop. The client reads `Etc/Commodity.img` itself, and
`SetCashShop` sends a modified-commodity count of zero (`world::commodity`, `net::cashshop`).
The server's own purchase gate is the same file: `session/cashshop.rs` refuses a serial that
is not in `gm-handbook/commodity.txt` and refuses one whose `OnSale` is 0. So an item is

    ON SALE      at least one Commodity row with OnSale = 1          -> buyable
      of which   VISIBLE: that row's scope (or its SN) is named by a tab in CashShopCategory;
                 NO TAB:  on sale, but no tab lists it, so the client has nowhere to draw it
    OFF          it has Commodity rows, and every one has OnSale = 0  -> listed, switched off
    NOT LISTED   no Commodity row at all                              -> cannot be bought

"Cash item" is the client's own flag: `info/cash = 1` on the item, read out of Item.wz and
Character.wz by the same walkers `tools/dump_itemdata.py` uses.

## Controls, run before anything is written

* `5070000` (Megaphone) must be a cash item and must have a Commodity row;
* `2000000` (Red Potion) must NOT be a cash item;
* at least 500 cash items must be found at all - an archive that failed to open would
  otherwise read as "the client has few cash items", which is the silent negative CLAUDE.md
  keeps warning about.

## Blind spots, named

* An item that is a cash item by id range but lacks `info/cash` is not counted. The count of
  such items in the `5xxxxxx` block is printed so it is visible rather than silent.
* Commodity.img is the *authored* list. What a live service rotated on and off is not in the
  client and not knowable from here.
"""
import collections
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import dump_itemdata as D  # noqa: E402  (chdirs to the repo root on import)

OUT = os.path.join("gm-handbook", "cashaudit.txt")
COMMODITY = os.path.join("gm-handbook", "commodity.txt")
CATEGORY = os.path.join("gm-handbook", "cashshopcategory.txt")
NAMES = os.path.join("gm-handbook", "items.txt")


def load_names():
    names = {}
    if os.path.exists(NAMES):
        for line in open(NAMES, encoding="utf-8", errors="replace"):
            if line.startswith("#") or "," not in line:
                continue
            a, b = line.split(",", 1)
            if a.strip().isdigit():
                names[int(a)] = b.strip()
    return names


def load_commodity():
    rows = collections.defaultdict(list)
    cols = None
    for line in open(COMMODITY, encoding="utf-8", errors="replace"):
        if line.startswith("# sn,"):
            cols = [c.strip() for c in line[2:].split(",")]
            continue
        if line.startswith("#") or not line.strip():
            continue
        f = [x.strip() for x in line.rstrip("\n").split(",")]
        r = dict(zip(cols, f))
        try:
            rows[int(r["itemId"])].append(r)
        except (KeyError, ValueError):
            pass
    return rows


def load_tabs():
    """(scopes a tab names, SNs a tab names), each mapped to 'Tab / Sub'."""
    scopes, sns = {}, {}
    for line in open(CATEGORY, encoding="utf-8", errors="replace"):
        if line.startswith("#") or not line.strip():
            continue
        f = [x.strip() for x in line.split(",")]
        if len(f) < 6:
            continue
        label = "%s / %s" % (f[1], f[3])
        (scopes if f[4] == "scope" else sns)[f[5]] = label
    return scopes, sns


def collect():
    """{id: (source, info)} for every item in Item.wz and Character.wz."""
    out = {}
    for category in D.ITEM_CATEGORIES:
        archive = os.path.join(D.DATA_ROOT, "Item", category, category + "_000.wz")
        if os.path.exists(archive):
            got = {}
            D.collect_grouped(archive, got)
            for i, info in got.items():
                out[i] = ("Item/" + category, info)
    for slot in D.EQUIP_SLOTS:
        archive = os.path.join(D.DATA_ROOT, "Character", slot, slot + "_000.wz")
        if os.path.exists(archive):
            got = {}
            D.collect_per_image(archive, got)
            for i, info in got.items():
                out[i] = ("Character/" + slot, info)
    return out


def is_cash(info):
    return D.num(info, "cash") == 1


def main():
    if not os.path.exists(D.WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % D.WZ_DUMP)
    for p in (COMMODITY, CATEGORY):
        if not os.path.exists(p):
            raise SystemExit("%s is missing - run: python tools/dump_commodity.py" % p)

    names = load_names()
    commodity = load_commodity()
    tab_scopes, tab_sns = load_tabs()
    everything = collect()
    cash = {i: v for i, v in everything.items() if is_cash(v[1])}

    # ---- controls
    problems = []
    if 5070000 not in cash:
        problems.append("5070000 Megaphone is not flagged cash - the flag reader is wrong")
    if 5070000 not in commodity:
        problems.append("5070000 Megaphone has no Commodity row - commodity.txt did not load")
    if 2000000 in cash:
        problems.append("2000000 Red Potion reads as cash - the flag reader is wrong")
    if len(cash) < 500:
        problems.append("only %d cash items found - an archive failed to open" % len(cash))
    if problems:
        for p in problems:
            print("CONTROL FAILED: " + p)
        raise SystemExit(1)

    def status(item_id):
        rows = commodity.get(item_id, [])
        if not rows:
            return "NOT LISTED", "", ""
        on = [r for r in rows if r.get("onSale") == "1"]
        if not on:
            return "OFF", ",".join(r["sn"] for r in rows), ""
        visible = [r for r in on if r.get("scope") in tab_scopes or r["sn"] in tab_sns]
        tab = ""
        if visible:
            r = visible[0]
            tab = tab_scopes.get(r.get("scope")) or tab_sns.get(r["sn"], "")
            return "ON SALE", ",".join(r["sn"] for r in on), tab
        return "ON SALE, NO TAB", ",".join(r["sn"] for r in on), ""

    def group(item_id, source):
        if source.startswith("Character/"):
            return source.split("/", 1)[1]
        return "%03d" % (item_id // 10000)

    table = []
    by_group = collections.defaultdict(collections.Counter)
    for item_id in sorted(cash):
        source, _ = cash[item_id]
        st, sns, tab = status(item_id)
        g = group(item_id, source)
        by_group[g][st] += 1
        table.append((item_id, source, g, st, sns, tab, names.get(item_id, "")))

    # Commodity rows for items WZ does not flag cash (packages, or data drift): list them too.
    orphan_rows = sorted(i for i in commodity if i not in cash)

    # A 5xxxxxx id without the flag - the named blind spot, made visible.
    unflagged_5 = sorted(i for i, (s, info) in everything.items()
                         if 5000000 <= i < 6000000 and not is_cash(info))

    os.makedirs("gm-handbook", exist_ok=True)
    with open(OUT, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# itemId, source, group, status, onSaleSNs, tab, name\n")
        fh.write("# generated by tools/audit_cash_items.py: every item whose WZ info/cash = 1,\n")
        fh.write("# against gm-handbook/commodity.txt (the client's Commodity.img). NEVER HAND-EDIT.\n")
        for row in table:
            fh.write("%d, %s, %s, %s, %s, %s, %s\n" % row)
        fh.write("# Commodity rows whose itemId is NOT a WZ cash item (%d):\n" % len(orphan_rows))
        for i in orphan_rows:
            fh.write("#   %d  %s  sns=%s\n" % (i, names.get(i, "?"),
                                              ",".join(r["sn"] for r in commodity[i])))

    totals = collections.Counter(st for _, _, _, st, _, _, _ in table)
    print("cash items in WZ: %d   (Commodity rows: %d over %d item ids)"
          % (len(cash), sum(len(v) for v in commodity.values()), len(commodity)))
    for k in ("ON SALE", "ON SALE, NO TAB", "OFF", "NOT LISTED"):
        print("  %-16s %5d" % (k, totals[k]))
    print("  commodity rows for non-cash item ids: %d" % len(orphan_rows))
    print("  5xxxxxx ids WITHOUT the cash flag (blind spot, not counted): %d" % len(unflagged_5))
    print()
    print("%-10s %8s %8s %6s %6s %10s   example" % ("group", "total", "on sale", "no tab", "off", "not listed"))
    for g in sorted(by_group, key=lambda x: (not x[0].isdigit(), x)):
        c = by_group[g]
        example = next((n for i, s, gg, st, sn, t, n in table if gg == g and n), "")
        print("%-10s %8d %8d %6d %6d %10d   %s" % (
            g, sum(c.values()), c["ON SALE"], c["ON SALE, NO TAB"], c["OFF"], c["NOT LISTED"], example[:40]))
    print("\nwrote %s" % OUT)


if __name__ == "__main__":
    main()
