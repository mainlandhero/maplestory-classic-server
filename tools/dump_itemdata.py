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
    return {str(k).strip(): v for k, v in info.items()
            if not isinstance(v, (dict, list))}


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

    rows = [(i, [num(info, c) for c in COLUMNS]) for i, info in sorted(items.items())]

    os.makedirs("gm-handbook", exist_ok=True)
    path = os.path.join("gm-handbook", "itemdata.txt")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# itemId, " + ", ".join(COLUMNS) + "\n")
        fh.write("# generated by tools/dump_itemdata.py from the client's own Item.wz and\n")
        fh.write("# Character.wz. price is the SELL price; the BUY price is not in the\n")
        fh.write("# client at all and is authored in data/shops.txt.\n")
        fh.write("# A row means measured. An id with no row is an item this client does not\n")
        fh.write("# have - refuse it rather than assume a default. slotMax 0 means the\n")
        fh.write("# property is absent, which is every equip: equips do not stack.\n")
        for item_id, values in rows:
            fh.write("%d, %s\n" % (item_id, ", ".join(str(v) for v in values)))

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
