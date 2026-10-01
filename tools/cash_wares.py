#!/usr/bin/env python3
"""The classic client's named cash items that its own Cash Shop never listed - as sale rows.

    python tools/cash_wares.py            # print what would be added, by tab (dry run)

Called by `tools/backport_install.py` step 4f, which writes the rows into the client's
`Etc/Commodity.img`. The owner, 2026-09-23: *"Add all of the items that are not listed but named
except those that are part of the collaboration signature sets since they come from the Cash
Coupons instead."*

## Why the PRISTINE client, and not client-patched

The install is re-run on every release, and `backport_install.py --check` requires the
installed archives to be byte-identical to a fresh build. So the list of wares must not depend
on what is installed: read from `client-patched`, every item this adds would be "already
listed" after the first install and would vanish from the second. The untouched original at
`C:\\Nexon\\Library\\maplestorycw` never changes, so the same rows come out every time.

It also settles the collaboration exclusion without a name match: every Frieren / Fern /
Stark / Himmel / Übel / Aura / Lügner / Linie item was *backported from the modern client*, so
none of them exists in the pristine one. The exclusion against the backport manifest is kept
anyway, as a guard that must come out empty - a name match would have been wrong twice over
(the "Label Rings" match `bel`, and the hair-hats were renumbered after the manifest was
written).

## What a row looks like, and where each number comes from

Every choice is copied from the classic client's own 159 rows for the same kind of item
(`gm-handbook/commodity.txt`), not invented:

| kind | tab (scope) | price | period | source |
|---|---|---|---|---|
| clothing, by Character slot | Weapon 400, Cap 401, Cape 402, Longcoat 403, Coat 404, Pants 405, Shoes 406, **Glove 407**, face acc. 408, eye acc. 409, Ring 411 | 100 | permanent | the classic fashion rows' tab |
| `503` hired merchants | Convenience 300 | 700 | 7 days | the classic 503 rows' tab and period |
| `514` store permits | Convenience 300 | 100 | permanent | the classic 514 rows' tab |
| `515` beauty coupons | Beauty / Misc 502 | 100 | permanent | the classic 515 rows' tab |
| `516` emotions | Beauty / Expressions 503 | 100 | permanent | the classic 516 rows' tab |
| `501` character effects | **Fashion / Effects 410** | 100 | permanent | **[I]**: no classic 501 row exists; the tab does, and is empty |

**The TAB is the classic client's; the price and period are the owner's shop-wide rule**
(2026-09-23 and 2026-09-24): *"everything in the Cash Shop costs 100 LP and does not have
duration with the exception of shop merchants. 7 day shop merchants should cost 700 LP, 1 day
shop merchants should cost 100 LP. Collaboration signature style packages should maintain their
price."* [`price_rule`] is that sentence, and [`shipped_row_fixes`] applies it to the classic
shop's OWN rows too - the 90-day clothing, the 1000 LP bundles, the 90-day permits and emotions.
The server never applied a row's period anyway (`world::commodity::Commodity::period_days` is
only logged), so the durations only change what the shop draws; the prices are what the server
debits.

Gender, for clothing, follows the client's own rule (`FUN_140253130`, the id's fourth digit:
0/5 male, 1/6 female, anything else unisex), which is also what the classic rows do - 1055003
is `0`, 1056002 is `1`, every 1057xxx is `2`. **Every non-equip is `2`**: that digit means
nothing on a `5xxxxxx` id, and reading it there locked `5010000` to male on the first build. A shield has no tab of its own and goes under Weapons.

A kind this table does not cover is **reported, not guessed**, and gets no row.

## SNs

`SN = (10 + scope // 100) * 10_000_000 + (scope % 100) * 100_000 + index`, which is exactly the
arithmetic `tools/dump_commodity.py` derives the tab from. `index` continues after the highest
SN the pristine client already uses in that scope, and items are taken in id order, so the
same item gets the same SN on every build.
"""
import json
import os
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import dump_itemdata as D  # noqa: E402  (chdirs to the repo root on import)

PRISTINE = os.path.join("C:\\", "Nexon", "Library", "maplestorycw", "appdata", "Data")

SLOT_SCOPE = {"Weapon": 400, "Shield": 400, "Cap": 401, "Cape": 402, "Longcoat": 403,
              "Coat": 404, "Pants": 405, "Shoes": 406, "Glove": 407, "Ring": 411}
FASHION = (100, 0)  # price LP, period days. 0 = permanent - the owner, 2026-09-23

# item group (id // 10000) -> (scope, price, period). Prices and periods follow the shop-wide
# rule below (`price_rule`), so a new ware and a shipped one of the same kind cannot disagree.
ITEM_GROUP = {
    501: (410, 100, 0),    # [I] tab - no classic row; the Effects tab exists and is empty
    503: (300, 700, 7),    # a 7-day shop merchant - price_rule
    514: (300, 100, 0),
    515: (502, 100, 0),
    516: (503, 100, 0),
}

# **The shop-wide price rule.** The owner, 2026-09-24: *"make sure everything in the Cash Shop costs
# 100 LP and does not have duration with the exception of shop merchants. 7 day shop merchants
# should cost 700 LP, 1 day shop merchants should cost 100 LP. Collaboration signature style
# packages should maintain their price."*
#
# "Shop merchants" is read as the hired merchants, family 503 (Mushroom House Elf, Cozy
# Coffeehouse, Granny's Food Stand) - the only rows whose duration IS the product. Store permits
# (514) are the player's own shop and follow the general rule. The Signature Style packages are
# the Special tab's SN 120000000..120000008 and are left exactly as they are. **The four
# collaboration pets stay at 1000 LP** (the owner, same day: *"collab pets should remain at 1000
# LP"*) - they are written by backport_install.py's pet step (COLLAB_PET_PRICE_LP), never by
# this rule, and the server's drift test names them as the third exception.
MERCHANT_FAMILY = 503
MERCHANT_PRICE_BY_DAYS = {7: 700, 1: 100}
SIGNATURE_SNS = range(120_000_000, 120_000_009)


def price_rule(sn, item_id, period):
    """(price, period) a sale row must carry, or None to leave it alone (Signature Style)."""
    if sn in SIGNATURE_SNS:
        return None
    if item_id // 10_000 == MERCHANT_FAMILY:
        if period not in MERCHANT_PRICE_BY_DAYS:
            raise SystemExit("a shop merchant row (SN %d) runs %d days - the rule names only 1 and 7" % (sn, period))
        return MERCHANT_PRICE_BY_DAYS[period], period
    return 100, 0

STRING_IMAGES = ["Eqp.img", "Cash.img", "Consume.img", "Ins.img", "Etc.img", "Pet.img"]


def _cat(archive, image):
    r = subprocess.run([D.WZ_DUMP, "cat", archive, image], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        return None
    try:
        return json.loads(r.stdout)
    except ValueError:
        return None


def _names(node, out):
    if not isinstance(node, dict):
        return
    for k, v in node.items():
        if isinstance(v, dict):
            if str(k).isdigit() and isinstance(v.get("name"), str) and v["name"].strip():
                out[int(k)] = v["name"].strip()
            _names(v, out)


def pristine_names(root=PRISTINE):
    archive = os.path.join(root, "String", "String_000.wz")
    out = {}
    for image in STRING_IMAGES:
        node = _cat(archive, image)
        if node is None:
            raise SystemExit("cannot read %s from %s" % (image, archive))
        _names(node, out)
    return out


def pristine_commodity(root=PRISTINE):
    node = _cat(os.path.join(root, "Etc", "Etc_000.wz"), "Commodity.img")
    if node is None:
        raise SystemExit("cannot read the pristine Commodity.img under %s" % root)
    rows = []
    for row in node.values():
        if isinstance(row, dict) and "SN" in row and "ItemId" in row:
            rows.append((int(row["SN"]), int(row["ItemId"])))
    return rows


def shipped_row_fixes(root=PRISTINE):
    """[(row, sn, item_id, name_hint, [(field, value), ...])] for every SHIPPED on-sale row that
    [`price_rule`] would change - the classic shop's own rows, which this project does not
    otherwise rewrite. `row` is the Commodity.img child key, which is what a `patch` line
    addresses.

    A repriced row also gets `originalPrice` = the new price and `discount` = 0 when it has one:
    the 11-Megaphone pack ships at 1000 with `originalPrice 1100, discount 1`, and leaving those
    would draw 100 LP as a 91% sale."""
    node = _cat(os.path.join(root, "Etc", "Etc_000.wz"), "Commodity.img")
    if node is None:
        raise SystemExit("cannot read the pristine Commodity.img under %s" % root)
    out = []
    for key, row in node.items():
        if not (isinstance(row, dict) and str(key).isdigit() and "ItemId" in row):
            continue
        if int(row.get("OnSale", 0)) != 1:
            continue  # the 92xxxxxx placeholders are not in the shop
        sn, item_id = int(row["SN"]), int(row["ItemId"])
        period, price = int(row.get("Period", 0)), int(row.get("Price", 0))
        want = price_rule(sn, item_id, period)
        if want is None:
            continue
        fields = []
        if price != want[0] or int(row.get("originalPrice", price)) != want[0]:
            fields += [("Price", want[0]), ("originalPrice", want[0])]
            if "discount" in row:
                fields.append(("discount", 0))
        if period != want[1]:
            fields.append(("Period", want[1]))
        if fields:
            out.append((int(key), sn, item_id, fields))
    return sorted(out)


def pristine_cash_items(root=PRISTINE):
    """{id: source} for every item the pristine client flags info/cash = 1."""
    out = {}
    for category in D.ITEM_CATEGORIES:
        archive = os.path.join(root, "Item", category, category + "_000.wz")
        if os.path.exists(archive):
            got = {}
            D.collect_grouped(archive, got)
            out.update({i: "Item/" + category for i, info in got.items() if D.num(info, "cash") == 1})
    for slot in D.EQUIP_SLOTS + ["Shield"]:
        archive = os.path.join(root, "Character", slot, slot + "_000.wz")
        if os.path.exists(archive):
            got = {}
            D.collect_per_image(archive, got)
            out.update({i: slot for i, info in got.items() if D.num(info, "cash") == 1})
    return out


def gender(item_id, source):
    """The client's gender-from-id rule - for EQUIPS only.

    `FUN_140253130` reads the fourth digit of a Character item's id. On a `5xxxxxx` cash item
    that digit is just part of the number: `5010000` would come out "male" and `5151036`
    "female", and the shop would refuse the other half of the players for no reason. The first
    build of this step did exactly that, and a spot check of the built Commodity.img caught it
    (`5010000` Gender 0). Every classic row for a non-equip says 2."""
    if source.startswith("Item/"):
        return 2
    digit = (item_id // 1000) % 10
    return 0 if digit in (0, 5) else 1 if digit in (1, 6) else 2


def placement(item_id, source):
    """(scope, price, period) for this item, or None when no classic row says where it goes."""
    if source in SLOT_SCOPE:
        return (SLOT_SCOPE[source],) + FASHION
    if source == "Accessory":
        return (408 if item_id // 10000 == 101 else 409,) + FASHION
    if source.startswith("Item/"):
        return ITEM_GROUP.get(item_id // 10000)
    return None


def sn_for(scope, index):
    return (10 + scope // 100) * 10_000_000 + (scope % 100) * 100_000 + index


def wares(also_sold=(), collaboration=(), root=PRISTINE):
    """([(sn, item_id, scope, price, period, gender, name)], [(item_id, source, name) not placed],
    [collaboration ids that WOULD have been added - must be empty])."""
    if not os.path.isdir(root):
        raise SystemExit("the pristine client is not at %s - it is the source of truth here" % root)
    names = pristine_names(root)
    shipped = pristine_commodity(root)
    listed = {i for _, i in shipped} | set(also_sold)
    cash = pristine_cash_items(root)
    if 5070000 not in cash or 5070000 not in listed:
        raise SystemExit("control failed: the Megaphone must be a listed pristine cash item")

    candidates = sorted(i for i in cash if i not in listed and i in names)
    leaked = [i for i in candidates if i in set(collaboration)]
    next_index = {}
    for sn, _ in shipped:
        scope = (sn // 10_000_000 - 10) * 100 + (sn // 100_000) % 100
        next_index[scope] = max(next_index.get(scope, 0), sn % 100_000 + 1)

    rows, unplaced = [], []
    for item_id in candidates:
        if item_id in leaked:
            continue
        where = placement(item_id, cash[item_id])
        if where is None:
            unplaced.append((item_id, cash[item_id], names[item_id]))
            continue
        scope, price, period = where
        index = next_index.get(scope, 0)
        next_index[scope] = index + 1
        rows.append((sn_for(scope, index), item_id, scope, price, period, gender(item_id, cash[item_id]), names[item_id]))
    return rows, unplaced, leaked


def main():
    rows, unplaced, leaked = wares()
    by_scope = {}
    for r in rows:
        by_scope.setdefault(r[2], []).append(r)
    print("%d rows to add" % len(rows))
    for scope in sorted(by_scope):
        rs = by_scope[scope]
        print("  scope %d  %3d  e.g. %s" % (scope, len(rs), "; ".join(r[6] for r in rs[:3])))
    print("unplaced (no classic row says where): %d" % len(unplaced))
    for u in unplaced:
        print("  %d %s %s" % u)
    print("collaboration leaks (must be 0): %d" % len(leaked))


if __name__ == "__main__":
    main()
