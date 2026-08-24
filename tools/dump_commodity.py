#!/usr/bin/env python3
"""Extract the cash shop's sale list, tab layout, bundles and block lists from Etc.wz.

The cash shop is stocked from the client's own data, not from anything the server
invents.  Five images in `client-patched/Data/Etc/Etc_000.wz` carry it:

    Commodity.img                   one node per SALE ROW: a serial number (SN), the
                                    item it hands over, how many, the price in NX, how
                                    long it lasts, and whether it is switched on
    CashShopCategory.img            the tabs and sub-tabs, and which rows land in each
    CashPackage.img                 bundles: a package item id -> the SNs it contains
    CommodityLimit.img              country / policy blocks on individual item ids
    CommodityTradeBlockBuyList.img  the trade-block buy list and its drop exceptions

    python tools/dump_commodity.py                  # writes into gm-handbook/
    python tools/dump_commodity.py --out-dir tmp

Run it WITH THE REPO AS THE WORKING DIRECTORY (it chdirs there itself).  Output is game
data regenerated from the client, so `gm-handbook/` is gitignored deliberately - the repo
carries the code, not the content.  Same arrangement as `tools/dump_portals.py`.

WHAT THIS CAN SAY
=================
The COLUMN NAMES are the client's own property names, and the READ ORDER was taken off
the listing of `FUN_140229d10`, the client function that walks a Commodity node property
by property (`research/cash-shop-items.md` has the addresses).  So the field set is
measured, not guessed.

`category` and `sub` are DERIVED, not read:

    category = SN / 10000000 - 10        sub = (SN / 100000) % 100

CashShopCategory's `scope` numbers are `category * 100 + sub`, and every scope it names
lands on exactly the item class you would expect from the tab's own label - scope 402
"Capes" holds only 110xxxx capes, 406 "Shoes" only 107xxxx shoes, and so on for all 21
non-empty scopes.  That is the check; the arithmetic is not asserted anywhere in the WZ.

WHAT THIS CANNOT KNOW
=====================
* **Whether the server has to send any of this.** The client contains a Commodity reader
  that pulls fields BY NAME, which is a WZ property bag and not a packet - but the call
  site that opens `Etc/Commodity.img` has not been found, so "the client stocks its own
  shop" is a derivation, not a measurement.
* **What the live shop actually sold on any given day.** Commodity.img is the authored
  list.  A real service rotates rows on and off on top of it, and nothing in the client
  records that rotation.
* **`WebShop` and `IsGift`.** Both are on all 159 rows and NEITHER STRING EXISTS ANYWHERE
  IN THE CLIENT, so no reader consumes them and this tool cannot say what they mean.  They
  are emitted as columns anyway rather than dropped.
* **`Class`.** Read by the client at `0x14022aa20`, present on 13 rows, values 0 and 2.
  Meaning not established.
* **The 92xxxxxx block.** 21 rows whose SN is outside the category scheme entirely
  (`category` comes out negative).  Every one is `Price 0, OnSale 0`.  Nothing in these
  five images says what they are for.
* **The wire format.** How a row reaches the client in a cash-shop packet is not in Etc.wz.

THE INSTRUMENT CHECKS ITSELF
============================
Three positive controls run before anything is written (see CONTROLS below).  Each one
names a node that is known to be present, so an empty output proves the DATA is absent
rather than that the parser broke - `CLAUDE.md`, "verify the instrument before believing
it".  A parse that yields zero sale rows raises instead of writing an empty file, and any
Commodity property this tool has no column for is reported by name rather than dropped
silently.
"""
import argparse
import json
import os
import subprocess
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

ETC_WZ = os.path.join("client-patched", "Data", "Etc", "Etc_000.wz")
STRING_WZ = os.path.join("client-patched", "Data", "String", "String_000.wz")
WZ_DUMP = os.path.abspath(
    os.path.join("target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
)

#: String.wz images that between them name every item a commodity row can hand over.
#: Cash equipment lives in Eqp.img, not Cash.img, so the list is the whole union.
NAME_IMAGES = ["Eqp.img", "Consume.img", "Ins.img", "Etc.img", "Cash.img", "Pet.img"]

#: Every Commodity property this tool knows about, in the order `FUN_140229d10` reads
#: them.  A property in the data that is not in here is REPORTED, never dropped quietly.
#: The tail of this list is read by the client but absent from this build's data; it is
#: kept so a future Commodity.img that carries those fields does not read as "unknown".
COMMODITY_FIELDS = [
    "SN", "ItemId", "Count", "Price", "mileageRate", "onlyMileage", "token", "Bonus",
    "Priority", "Period", "ReqPOP", "ReqLEV", "MaplePoint", "Meso", "Premium", "Gender",
    "OnSale", "Class", "Limit", "PbCash", "PbPoint", "PbGift", "originalPrice",
    "discount", "Refundable", "bombSale", "forcedCategory", "forcedSubCategory",
    "LimitMax", "LimitQuestID", "CheckQuest", "favorType", "WSLimitMax",
    "WSLimitRecordID", "MonthlyLimited", "gameWorld", "possibleTrading",
    "exchangeableOnce", "expireOnNonPremiumLogin", "expireOnLogout", "couponType",
    "blockRewardTrade", "ShowDiscount", "SubstituteSN", "ReqLevType", "Country",
    # in the data, read by nothing - no such string exists in the client at all
    "WebShop", "IsGift",
]

#: Columns of gm-handbook/commodity.txt, as (header, Commodity property).  Derived
#: columns and the name are appended after these.
COLUMNS = [
    ("sn", "SN"), ("itemId", "ItemId"), ("count", "Count"), ("price", "Price"),
    ("originalPrice", "originalPrice"), ("discount", "discount"), ("period", "Period"),
    ("gender", "Gender"), ("onSale", "OnSale"), ("class", "Class"),
    ("priority", "Priority"), ("reqLEV", "ReqLEV"), ("reqPOP", "ReqPOP"),
    ("bonus", "Bonus"), ("refundable", "Refundable"), ("pbCash", "PbCash"),
    ("pbPoint", "PbPoint"), ("pbGift", "PbGift"), ("webShop", "WebShop"),
    ("isGift", "IsGift"),
]

#: Positive controls.  Each names a node that IS in this archive, so a run that produces
#: nothing has to be the data's fault and not the parser's.  If the client is ever
#: repacked these will fail loudly - that is the point; a stale control that still passes
#: is the failure `CLAUDE.md` keeps warning about.
CONTROLS = {
    # Commodity.img node "0" - the first sale row, a Regular Store Permit at 100 NX.
    "commodity": ("0", {"SN": 130000000, "ItemId": 5140000, "Price": 100, "OnSale": 1}),
    # CashShopCategory.img tab 3 sub 0 - Fashion / Weapons, scope 400.
    "category": (("3", "0"), ("Fashion", "Weapons", 400)),
    # String.wz must be able to name that same item, or the name column is silently blank.
    "name": (5140000, "Regular Store Permit"),
}

NX_SENTINEL_NONE = 999999999


def run(*args):
    """One wz-dump invocation.  Explicit UTF-8 because Etc.wz carries Korean comments and
    Windows would otherwise decode wz-dump's stdout with the ANSI code page."""
    r = subprocess.run([WZ_DUMP, *args], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        raise SystemExit("wz-dump %s failed (rc=%s): %s"
                         % (" ".join(args), r.returncode, (r.stderr or "").strip()[:300]))
    return r.stdout


def image(archive, name):
    return json.loads(run("cat", archive, name))


def unquote(v):
    """WZ string values in CommodityLimit.img arrive wrapped in literal quote marks."""
    if isinstance(v, str) and len(v) >= 2 and v[0] == '"' and v[-1] == '"':
        return v[1:-1]
    return v


def scope_of(sn):
    """CashShopCategory's `scope` for a sale row.  DERIVED - see the module docstring."""
    return sn // 100000 - 1000


def category_of(sn):
    return sn // 10000000 - 10, (sn // 100000) % 100


def item_names():
    """id -> name from String.wz.  Emits a row for any all-digit key carrying a `name`,
    at whatever depth, so the flat images and Eqp.img's two levels use the same walker."""
    out = {}

    def walk(node):
        if not isinstance(node, dict):
            return
        for key, value in node.items():
            if not isinstance(value, dict):
                continue
            if key.isdigit() and isinstance(value.get("name"), str):
                out[int(key)] = value["name"].replace("\n", " ").replace("\r", " ").strip()
                continue
            walk(value)

    for img in NAME_IMAGES:
        try:
            walk(image(STRING_WZ, img))
        except SystemExit as exc:
            print("  ! %s: %s" % (img, exc), file=sys.stderr)
    return out


def check_controls(commodity, categories, names):
    """Prove the parser can see a known-present node before any negative is believed."""
    key, want = CONTROLS["commodity"]
    row = commodity.get(key)
    if not isinstance(row, dict):
        raise SystemExit("POSITIVE CONTROL FAILED: Commodity.img has no node %r. The "
                         "parser or the archive changed; an empty run would not have "
                         "meant 'no rows'." % key)
    for field, value in want.items():
        if row.get(field) != value:
            raise SystemExit("POSITIVE CONTROL FAILED: Commodity.img node %s has %s=%r, "
                             "expected %r. Re-read the archive before trusting this run."
                             % (key, field, row.get(field), value))

    (tab, sub), (tab_name, sub_name, scope) = CONTROLS["category"]
    node = (categories.get(tab) or {})
    leaf = node.get(sub) if isinstance(node, dict) else None
    got = (node.get("name") if isinstance(node, dict) else None,
           leaf.get("name") if isinstance(leaf, dict) else None,
           ((leaf or {}).get("scope") or {}).get("0"))
    if got != (tab_name, sub_name, scope):
        raise SystemExit("POSITIVE CONTROL FAILED: CashShopCategory %s/%s reads %r, "
                         "expected %r." % (tab, sub, got, (tab_name, sub_name, scope)))

    ident, want_name = CONTROLS["name"]
    if names.get(ident) != want_name:
        raise SystemExit("POSITIVE CONTROL FAILED: String.wz names item %d as %r, "
                         "expected %r. Without this the name column would be blank on "
                         "every row and nothing would say so." % (ident, names.get(ident), want_name))
    print("positive controls: Commodity node 0, CashShopCategory 3/0, String.wz name for "
          "%d - all three parse" % ident)


def write_commodity(out_dir, commodity, names):
    rows, skipped, unknown = [], [], {}
    commas = 0
    for key in sorted(commodity, key=lambda k: int(k) if k.isdigit() else -1):
        node = commodity[key]
        if not isinstance(node, dict):
            skipped.append((key, "not a property bag"))
            continue
        sn, item = node.get("SN"), node.get("ItemId")
        if not isinstance(sn, int) or not isinstance(item, int):
            skipped.append((key, "no integer SN/ItemId"))
            continue
        for field in node:
            if field not in COMMODITY_FIELDS:
                unknown.setdefault(field, 0)
                unknown[field] += 1
        cat, sub = category_of(sn)
        name = names.get(item, "")
        if "," in name:
            name = name.replace(",", " ")
            commas += 1
        rows.append([str(node.get(prop, "")) for _, prop in COLUMNS]
                    + [str(cat), str(sub), str(scope_of(sn)), name])

    if not rows:
        # The skip reasons are put IN the exception rather than printed after it: "why
        # did it parse zero" is the only question worth answering at this point, and a
        # summary printed further down would never be reached.
        raise SystemExit("Commodity.img parsed to ZERO sale rows. The positive control "
                         "passed, so this is the data and not the parser - but an empty "
                         "commodity.txt would be worse than no file, so nothing was "
                         "written. %d node(s) were skipped: %s"
                         % (len(skipped), "; ".join("%s (%s)" % s for s in skipped[:20])
                            or "none - the image had no nodes at all"))

    path = os.path.join(out_dir, "commodity.txt")
    header = [h for h, _ in COLUMNS] + ["category", "sub", "scope", "name"]
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# " + ", ".join(header) + "\n")
        fh.write("# generated by tools/dump_commodity.py from the client's own\n")
        fh.write("# Data/Etc/Etc_000.wz `Commodity.img`. NEVER HAND-EDIT: regenerate.\n")
        fh.write("# One row = one SALE ROW, keyed by SN. The same itemId can appear on\n")
        fh.write("# several rows at different counts, prices and periods.\n")
        fh.write("# AN EMPTY CELL MEANS THE PROPERTY IS ABSENT FROM THAT WZ NODE, which\n")
        fh.write("# is not the same as 0 - `class` and `discount` are absent on most rows.\n")
        fh.write("#   price / originalPrice  NX. discount=1 marks a row sold under its\n")
        fh.write("#                          own originalPrice.\n")
        fh.write("#   count                  how many of itemId the row hands over.\n")
        fh.write("#   period                 DAYS the item lasts. 0 = no expiry.\n")
        fh.write("#   gender                 0 male, 1 female, 2 either.\n")
        fh.write("#   onSale                 0 means the row is switched OFF.\n")
        fh.write("#   category, sub, scope   DERIVED from SN, not read from the WZ:\n")
        fh.write("#                          category = SN/10000000 - 10,\n")
        fh.write("#                          sub      = (SN/100000) mod 100,\n")
        fh.write("#                          scope    = category*100 + sub, which is what\n")
        fh.write("#                          cashshopcategory.txt matches on.\n")
        fh.write("#   webShop, isGift        in the data; NO SUCH STRING EXISTS IN THE\n")
        fh.write("#                          CLIENT, so nothing reads them. Meaning unknown.\n")
        for r in rows:
            fh.write(", ".join(r) + "\n")

    on = sum(1 for r in rows if r[header.index("onSale")] == "1")
    prices = sorted({int(r[header.index("price")]) for r in rows})
    items = {r[1] for r in rows}
    print("%s: %d sale rows, %d distinct item ids, %d on sale / %d switched off, "
          "price %d..%d NX" % (path, len(rows), len(items), on, len(rows) - on,
                               prices[0], prices[-1]))
    if commas:
        print("  %d item name(s) contained a comma and had it replaced with a space"
              % commas)
    for key, why in skipped:
        print("  skipped node %s: %s" % (key, why))
    print("  skipped %d node(s)" % len(skipped))
    if unknown:
        print("  ! Commodity properties with NO column in this tool (emitted nowhere): %s"
              % ", ".join("%s x%d" % kv for kv in sorted(unknown.items())))
    else:
        print("  every property present in the data has a column")
    return rows, header


def write_categories(out_dir, categories, sale_sns):
    rows, skipped = [], []
    for tab_key in sorted(categories, key=lambda k: int(k) if k.isdigit() else -1):
        tab = categories[tab_key]
        if not isinstance(tab, dict):
            skipped.append((tab_key, "not a property bag"))
            continue
        tab_name = tab.get("name", "")
        leaves = [k for k in tab if k != "name"]
        if not leaves:
            # A tab with no sub-tabs is real - "Special" is one - and it is emitted with
            # an empty rule so the layout is not silently short a tab.
            rows.append((tab_key, tab_name, "", "", "", ""))
            continue
        for sub_key in sorted(leaves, key=lambda k: int(k) if k.isdigit() else -1):
            sub = tab[sub_key]
            if not isinstance(sub, dict):
                skipped.append(("%s/%s" % (tab_key, sub_key), "not a property bag"))
                continue
            sub_name = sub.get("name", "")
            emitted = False
            for _, value in sorted((sub.get("scope") or {}).items(),
                                   key=lambda kv: int(kv[0]) if kv[0].isdigit() else 0):
                rows.append((tab_key, tab_name, sub_key, sub_name, "scope", str(value)))
                emitted = True
            for _, value in sorted((sub.get("commoditySN") or {}).items(),
                                   key=lambda kv: int(kv[0]) if kv[0].isdigit() else 0):
                rows.append((tab_key, tab_name, sub_key, sub_name, "sn", str(value)))
                emitted = True
            if not emitted:
                rows.append((tab_key, tab_name, sub_key, sub_name, "", ""))

    if not rows:
        raise SystemExit("CashShopCategory.img parsed to ZERO rows; nothing written.")

    path = os.path.join(out_dir, "cashshopcategory.txt")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# tab, tabName, sub, subName, rule, value\n")
        fh.write("# generated by tools/dump_commodity.py from the client's own\n")
        fh.write("# Data/Etc/Etc_000.wz `CashShopCategory.img`. NEVER HAND-EDIT.\n")
        fh.write("# rule=scope  every commodity row whose derived scope equals value\n")
        fh.write("#             (see commodity.txt's `scope` column) belongs here.\n")
        fh.write("# rule=sn     one explicit commodity SN, listed by hand in the WZ.\n")
        fh.write("# rule empty  a tab or sub-tab the WZ declares with no contents rule.\n")
        for r in rows:
            fh.write(", ".join(r) + "\n")

    scopes = {int(v) for _, _, _, _, k, v in rows if k == "scope"}
    sns = {int(v) for _, _, _, _, k, v in rows if k == "sn"}
    have = {scope_of(sn) for sn in sale_sns}
    empty = sorted(s for s in scopes if s not in have)
    orphan = sorted(s for s in sns if s not in sale_sns)
    print("%s: %d rows, %d tabs, %d scopes, %d explicit SNs"
          % (path, len(rows), len({r[0] for r in rows}), len(scopes), len(sns)))
    print("  scopes named by a tab with NO sale row behind them: %s"
          % (empty if empty else "none"))
    print("  explicit SNs with no sale row in Commodity.img: %s"
          % (orphan if orphan else "none"))
    for key, why in skipped:
        print("  skipped %s: %s" % (key, why))
    return rows


def write_packages(out_dir, packages, sale_sns, names):
    rows, skipped = [], []
    for pkg_key in sorted(packages, key=lambda k: int(k) if k.isdigit() else -1):
        node = packages[pkg_key]
        if not isinstance(node, dict) or not isinstance(node.get("SN"), dict):
            skipped.append((pkg_key, "no SN list"))
            continue
        for _, sn in sorted(node["SN"].items(),
                            key=lambda kv: int(kv[0]) if kv[0].isdigit() else 0):
            rows.append((pkg_key, str(sn), "1" if sn in sale_sns else "0",
                         names.get(int(pkg_key), "") if pkg_key.isdigit() else ""))

    path = os.path.join(out_dir, "cashpackage.txt")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# packageItemId, sn, snHasSaleRow, packageName\n")
        fh.write("# generated by tools/dump_commodity.py from the client's own\n")
        fh.write("# Data/Etc/Etc_000.wz `CashPackage.img`. NEVER HAND-EDIT.\n")
        fh.write("# One row per SN inside one package. snHasSaleRow=0 means the package\n")
        fh.write("# names a commodity SN that Commodity.img does not define - the bundle\n")
        fh.write("# cannot be assembled from this client's data as it stands.\n")
        for r in rows:
            fh.write(", ".join(r) + "\n")
    dangling = sum(1 for r in rows if r[2] == "0")
    print("%s: %d package member rows across %d packages, %d of them naming an SN with "
          "no sale row" % (path, len(rows), len({r[0] for r in rows}), dangling))
    for key, why in skipped:
        print("  skipped package %s: %s" % (key, why))
    return rows


def write_limits(out_dir, limit, tradeblock):
    """Country and policy blocks.  Emitted with a `source` column because two different
    images say 'this item id is restricted' in two different shapes."""
    rows = []
    for group_name, group in sorted((limit or {}).items()):
        if group_name == "info" or not isinstance(group, dict):
            continue
        if group_name == "LimitCountry":
            for key, value in sorted(group.items()):
                if key == "info":
                    continue
                rows.append(("CommodityLimit/LimitCountry", str(unquote(value)), ""))
            continue
        for key, entry in sorted(group.items()):
            if key == "info" or not isinstance(entry, dict):
                continue
            tag = unquote(entry.get("CountryCode") or entry.get("info")
                          or entry.get("BlockType") or key)
            ids = entry.get("ItemID") or {}
            if not ids:
                rows.append(("CommodityLimit/" + group_name, str(tag), ""))
            for _, item in sorted(ids.items(),
                                  key=lambda kv: int(kv[0]) if kv[0].isdigit() else 0):
                rows.append(("CommodityLimit/" + group_name, str(tag), str(item)))

    for group_name in ("LimitItemID", "PossibleDropItemID"):
        group = (tradeblock or {}).get(group_name) or {}
        if not group:
            rows.append(("CommodityTradeBlockBuyList/" + group_name, "", ""))
        for _, item in sorted(group.items(),
                              key=lambda kv: int(kv[0]) if kv[0].isdigit() else 0):
            rows.append(("CommodityTradeBlockBuyList/" + group_name, "", str(item)))
    block_day = ((tradeblock or {}).get("Info") or {}).get("blockDay")
    if block_day is not None:
        rows.append(("CommodityTradeBlockBuyList/Info", "blockDay", str(block_day)))

    path = os.path.join(out_dir, "commoditylimit.txt")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# source, key, itemId\n")
        fh.write("# generated by tools/dump_commodity.py from the client's own\n")
        fh.write("# Data/Etc/Etc_000.wz `CommodityLimit.img` and\n")
        fh.write("# `CommodityTradeBlockBuyList.img`. NEVER HAND-EDIT.\n")
        fh.write("# A row with an empty itemId is a declared group that lists no items.\n")
        for r in rows:
            fh.write(", ".join(r) + "\n")
    with_item = sum(1 for r in rows if r[2])
    print("%s: %d rows, %d of them naming an item id" % (path, len(rows), with_item))
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-dir", default="gm-handbook")
    ap.add_argument("--archive", default=ETC_WZ)
    args = ap.parse_args()

    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)
    if not os.path.exists(args.archive):
        raise SystemExit("%s is missing" % args.archive)

    commodity = image(args.archive, "Commodity.img")
    categories = image(args.archive, "CashShopCategory.img")
    packages = image(args.archive, "CashPackage.img")
    limit = image(args.archive, "CommodityLimit.img")
    tradeblock = image(args.archive, "CommodityTradeBlockBuyList.img")
    names = item_names()

    check_controls(commodity, categories, names)

    os.makedirs(args.out_dir, exist_ok=True)
    rows, header = write_commodity(args.out_dir, commodity, names)
    sale_sns = {int(r[0]) for r in rows}
    write_categories(args.out_dir, categories, sale_sns)
    write_packages(args.out_dir, packages, sale_sns, names)
    write_limits(args.out_dir, limit, tradeblock)
    return 0


if __name__ == "__main__":
    sys.exit(main())
