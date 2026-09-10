#!/usr/bin/env python3
"""Every item's price, quest flag, trade block and stack size, from the client's own WZ.

Writes `gm-handbook/itemdata.txt`. Same shape as `tools/dump_equips.py` and
`tools/dump_portals.py`: generated game data, gitignored, never hand-edited.

    python tools/dump_itemdata.py

## What this measures, and what it does not

Four properties, all read straight out of `<item>/info` and none of them derived:

| column | property | what the server does with it |
|---|---|---|
| `price` | `info/price` | the SELL price - what an NPC pays for the item |
| `quest` | `info/quest` | 1 means a quest item. The owner: "do not allow quest items to be sold" |
| `tradeBlock` | `info/tradeBlock` | 1 means untradeable. The owner: "do not allow untradeable items to be stored" |
| `slotMax` | `info/slotMax` | stack size. Absent on equips, which do not stack |

**`price` is the SELL price, not the buy price, and the buy price cannot be derived from
it.** The owner, 2026-08-19: *"The prices client side most likely represents sell prices."* The
evidence beside that reading is a ratio that is nearly constant and then is not - every
consumable checked is exactly 10.00x its WZ price (Red Potion 5 -> 50, Meat 8 -> 80, nine of
nine) but Pet Food is 15 -> 35. A rule with one counterexample is not a rule, so what an NPC
*charges* is authored in `data/shops.txt` instead. This file is the other half: what an NPC
*pays*, and which items may not be sold or stored at all.

## Where the items are

Two different archive shapes, which is why this is not one loop:

* `Item/<cat>/<cat>_000.wz` - **grouped**. One image per 4-digit prefix (`0200.img`), whose
  children are the 8-digit item ids. Consume, Etc, Install, Cash, Special.
* `Item/Pet/Pet_000.wz` and `Character/<slot>/<slot>_000.wz` - **one image per item**, the
  id being the image stem. Every equip lives here, not under `Item.wz` at all.

The walker handles both without being told which is which: it descends any node and emits a
row for whatever carries an `info` child under an all-digit key, plus the image-root case.
That is the same trick `tools/dump_names.py` uses for `Eqp.img`'s extra nesting level.

## A row means MEASURED, and an absent row means unknown

Unlike `dump_equips.py`, an all-zero row is **kept**. `price 0, quest 0, tradeBlock 0,
slotMax 0` is a real statement about an item - it is worthless and freely tradeable - and it
is different from the item not being in the client at all. The server must be able to tell
those apart, because "id not in the table" is the case where it should refuse rather than
assume. `slotMax` is 0 wherever the property is absent, which is every equip; equips do not
stack and the client ships no slotMax for them.

## The instrument, and why nothing here was grepped

A raw byte search over a `.wz` archive returns **zero for `price`, `quest` and `info` alike**
- WZ encodes property names, so a negative from a grep over these files is worthless (see
STATUS.md goal F, where that nearly became a finding). Everything below goes through
`wz-dump`.
"""
import json
import os
import subprocess
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

DATA_ROOT = os.path.join("client-patched", "Data")
WZ_DUMP = os.path.abspath(
    os.path.join("target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
)

# Item/<name>/<name>_000.wz - children of each image are the items.
ITEM_CATEGORIES = ["Consume", "Etc", "Install", "Cash", "Special", "Pet"]

# Character/<slot>/<slot>_000.wz - one image per item. Same list as tools/dump_equips.py:
# Face and Hair are appearance rather than equipment, Afterimage and _Canvas are art.
EQUIP_SLOTS = ["Accessory", "Cap", "Cape", "Coat", "Glove", "Longcoat", "Pants",
               "Ring", "Shield", "Shoes", "Weapon", "PetEquip"]

COLUMNS = ["price", "quest", "tradeBlock", "slotMax"]
# Written after the integer columns so every reader that knew the five-column file still
# reads the same five things in the same places. `world::shops::load_item_data` reads it;
# the loadout and store drift tests only require "at least five".
FLOAT_COLUMNS = ["unitPrice"]


def run(*args):
    # Explicit UTF-8: Windows would otherwise decode wz-dump's output with the ANSI code
    # page, and some images carry bytes cp1252 has no mapping for.
    r = subprocess.run([WZ_DUMP, *args], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        return None
    return r.stdout


def images(archive):
    out = run("tree", archive, "1")
    if not out:
        return
    for line in out.splitlines():
        if "[IMG]" in line:
            yield line.split("[IMG]", 1)[1].split()[0]


def scalars(node):
    """The scalar members of a node's `info` child, with keys whitespace-stripped.

    `tools/dump_equips.py` found two keys in this client's data carrying an embedded
    newline (`inc\\nMHP`), so stripping is not defensive programming - it is a measured
    property of the archive.
    """
    if not isinstance(node, dict):
        return None
    info = node.get("info")
    if not isinstance(info, dict):
        return None
    out = {str(k).strip(): v for k, v in info.items()
           if not isinstance(v, (dict, list))}
    # **`spec`, carried alongside `info` under a prefix.** What a potion restores is not in
    # `info` at all - it is `spec/hp` and `spec/mp`, and the Red Potion's `spec/hp` is 100,
    # which is the number the owner expected on screen and did not get. The prefix keeps the two
    # namespaces apart: `info/time` and `spec/time` are different properties and merging
    # them flat would let one silently win.
    spec = node.get("spec")
    if isinstance(spec, dict):
        for k, v in spec.items():
            if not isinstance(v, (dict, list)):
                out["spec." + str(k).strip()] = v
    return out


def walk(node, out, root_id=None):
    """Emit id -> info-scalars for every item reachable in `node`.

    Two shapes, one walker: an image whose ROOT is the item (`root_id` names it), and an
    image whose all-digit CHILDREN are the items.
    """
    if not isinstance(node, dict):
        return
    if root_id is not None:
        info = scalars(node)
        if info is not None:
            out[root_id] = info
            return
    for key, value in node.items():
        if not isinstance(value, dict):
            continue
        if key.isdigit():
            info = scalars(value)
            if info is not None:
                out[int(key)] = info
                continue
        walk(value, out)


def num(info, key):
    v = info.get(key, 0)
    try:
        return int(v)
    except (TypeError, ValueError):
        # A handful of WZ scalars are floats or strings; a value that is not an integer is
        # not a price, and silently reading it as one is how a wrong number ships.
        try:
            return int(float(v))
        except (TypeError, ValueError):
            return 0


def real(info, key):
    """A float property, written as the WZ has it. `unitPrice` is 0.3 for a Subi star and
    `num()` would silently make that 0 - which is not a price of zero, it is a price the
    reader was never able to represent. Absent is 0."""
    v = info.get(key, 0)
    try:
        f = float(v)
    except (TypeError, ValueError):
        return 0
    return int(f) if f == int(f) else f


def collect_grouped(archive, out):
    """Item/<cat>: children of each image are the items."""
    found = 0
    for image in images(archive):
        text = run("cat", archive, image)
        if not text:
            continue
        try:
            node = json.loads(text)
        except ValueError:
            continue
        before = len(out)
        stem = image[:-4] if image.endswith(".img") else image
        # Pet_000.wz is grouped by directory but one image per item; its stem is the id.
        walk(node, out, root_id=int(stem) if stem.isdigit() and len(stem) > 4 else None)
        found += len(out) - before
    return found


def collect_per_image(archive, out):
    """Character/<slot>: one image per item, the stem being the id."""
    found = 0
    for image in images(archive):
        stem = image[:-4] if image.endswith(".img") else image
        if not stem.isdigit():
            continue
        text = run("cat", archive, image)
        if not text:
            continue
        try:
            node = json.loads(text)
        except ValueError:
            continue
        before = len(out)
        walk(node, out, root_id=int(stem))
        found += len(out) - before
    return found


def main():
    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)

    items = {}
    for category in ITEM_CATEGORIES:
        archive = os.path.join(DATA_ROOT, "Item", category, category + "_000.wz")
        if not os.path.exists(archive):
            print("  ! no archive for Item/%s" % category, file=sys.stderr)
            continue
        print("  %-12s %d" % (category, collect_grouped(archive, items)))

    for slot in EQUIP_SLOTS:
        archive = os.path.join(DATA_ROOT, "Character", slot, slot + "_000.wz")
        if not os.path.exists(archive):
            print("  ! no archive for Character/%s" % slot, file=sys.stderr)
            continue
        print("  %-12s %d" % (slot, collect_per_image(archive, items)))

    rows = [(i, [num(info, c) for c in COLUMNS] + [real(info, c) for c in FLOAT_COLUMNS])
            for i, info in sorted(items.items())]

    os.makedirs("gm-handbook", exist_ok=True)
    path = os.path.join("gm-handbook", "itemdata.txt")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# itemId, " + ", ".join(COLUMNS + FLOAT_COLUMNS) + "\n")
        fh.write("# generated by tools/dump_itemdata.py from the client's own Item.wz and\n")
        fh.write("# Character.wz. price is the SELL price; the BUY price is not in the\n")
        fh.write("# client at all and is authored in data/shops.txt.\n")
        fh.write("# A row means measured. An id with no row is an item this client does not\n")
        fh.write("# have - refuse it rather than assume a default. slotMax 0 means the\n")
        fh.write("# property is absent, which is every equip: equips do not stack.\n")
        fh.write("# unitPrice is the RECHARGE price per unit in mesos (a float: Subi is 0.3);\n")
        fh.write("# it is non-zero only on throwing stars (207xxxx) and bullets (233xxxx).\n")
        for item_id, values in rows:
            fh.write("%d, %s\n" % (item_id, ", ".join(str(v) for v in values)))

    # ---------------------------------------------------------------------------------
    # The second file: what a consumable restores.
    # ---------------------------------------------------------------------------------
    #
    # Separate from itemdata.txt on purpose. That file's five columns are parsed by
    # `world::ShopTable` and adding to them would change a format three other things read;
    # this is a different question with a different natural key set, and only ~1 item in 8
    # has any of it.
    #
    # `hp`/`mp` are FLAT amounts and `hpR`/`mpR` are PERCENTAGES OF THE MAXIMUM. Both exist
    # in this client and a few items carry both. Getting that pair the wrong way round is
    # exactly the class of mistake CLAUDE.md's "the unit, not the arithmetic" section is
    # about, so the units are in the header of the generated file too, not only here.
    # **The buff columns are here because HP/MP alone made three quarters of the potions
    # look broken.** The owner, 2026-09-09: *"Drinking the Dexterity Potion or the Magic Potion
    # also does not give me the proper buff"*, and the server's own refusal said why - *"item
    # 2002003 restores nothing this server knows about"*. It restores nothing; it BUFFS.
    #
    # The set below is not a guess at what a potion might carry. Every `spec` child of every
    # one of the 89 consumables with a `spec` node was enumerated first, and this is what is
    # actually there:
    #
    #   hp 29  time 26  mp 15  script 11  npc 11  moveTo 10  morph 7  mpR 4  acc 4
    #   hpR 3  pad 3  eva 3  mad 2  expBuff 2  speed 1  crt 1  crd 1  pdd 1  thaw 1
    #   indieSpeed 1  indieJump 1  incRepleteness 1  incTameness 1  exp 1
    #
    # `script`/`npc`/`moveTo`/`morph` are other mechanics with their own handlers, not stats,
    # so they are deliberately out. `indieSpeed`/`indieJump` are the **Indie index space**,
    # which `research/magic-damage.md` §7.4 records as separate from the CTS space and not
    # obviously carried by `0x007D`; they are emitted so the server can decide, and the server
    # says out loud which of them it can act on.
    spec_columns = [
        "hp", "mp", "hpR", "mpR",
        "time",
        "pad", "mad", "pdd", "mdd", "acc", "eva", "speed", "jump", "crt", "crd",
        "expBuff",
        "indieSpeed", "indieJump",
    ]
    spec_rows = []
    for item_id, info in sorted(items.items()):
        values = [num(info, "spec." + c) for c in spec_columns]
        if any(values):
            spec_rows.append((item_id, values))

    cpath = os.path.join("gm-handbook", "consumables.txt")
    with open(cpath, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# itemId, " + ", ".join(spec_columns) + "\n")
        fh.write("# generated by tools/dump_itemdata.py from the client's own Item.wz `spec`\n")
        fh.write("# nodes. A row means the item DOES something when used.\n")
        fh.write("#\n")
        fh.write("# hp, mp   FLAT amounts. Red Potion (2000000) is hp 100.\n")
        fh.write("# hpR, mpR PERCENTAGES of the maximum, not amounts. An item may carry both\n")
        fh.write("#          a flat and a percentage term; the client's own data does.\n")
        fh.write("# time     MILLISECONDS the buff columns last. 0 means the item has no\n")
        fh.write("#          timed effect - it is a restore, not a buff.\n")
        fh.write("# pad..crd FLAT stat bonuses for `time` milliseconds.\n")
        fh.write("# expBuff  a PERCENTAGE of normal experience: the 3x coupon carries 300.\n")
        fh.write("# indie*   the Indie index space, which is NOT the CTS space. Emitted so\n")
        fh.write("#          the server can decide; see world::consumables.\n")
        fh.write("#\n")
        fh.write("# An id with no row does nothing when used. That is not the same as an id\n")
        fh.write("# that is not in itemdata.txt, which this client does not have at all.\n")
        for item_id, values in spec_rows:
            fh.write("%d, %s\n" % (item_id, ", ".join(str(v) for v in values)))
    print("%s: %d items do something when used" % (cpath, len(spec_rows)))

    quest = sum(1 for _, v in rows if v[COLUMNS.index("quest")])
    blocked = sum(1 for _, v in rows if v[COLUMNS.index("tradeBlock")])
    priced = sum(1 for _, v in rows if v[COLUMNS.index("price")])
    stacking = sum(1 for _, v in rows if v[COLUMNS.index("slotMax")])
    print()
    print("%s: %d items" % (path, len(rows)))
    print("  %4d carry quest      - these may not be SOLD to an NPC" % quest)
    print("  %4d carry tradeBlock - these may not be STORED" % blocked)
    print("  %4d carry a non-zero price" % priced)
    print("  %4d carry slotMax (the rest are equips, which do not stack)" % stacking)


if __name__ == "__main__":
    main()
