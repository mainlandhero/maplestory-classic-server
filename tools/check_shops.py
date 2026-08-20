#!/usr/bin/env python3
"""Resolve every name in `data/shops.txt` against the client's own item table.

    python tools/check_shops.py            # summary plus every unresolved name
    python tools/check_shops.py --ids      # emit id,price,rank rows for the resolved ones

## Why this exists

`data/shops.txt` is transcribed by hand from the live game - it is the only source for shop
contents, because they are not in the client's WZ (STATUS.md goal F). Hand-transcribed game
data is exactly the kind of input this project has been burned by, so it is not trusted: every
name is resolved against `gm-handbook/items.txt`, which is generated from the client's own
`String.wz`. A name that does not resolve is a transcription error, an item this client does
not have, or a name the client spells differently - and all three are worth knowing before a
shop is built rather than after it silently sells the wrong thing.

**Ambiguity is a failure too.** Some names appear on more than one item id, and picking the
first would be a silent wrong answer. Those are reported separately.

## The control

`gm-handbook/items.txt` is gitignored generated data. If it is missing this reports every
name as unresolved, which looks exactly like a catastrophic transcription failure - so the
absence is checked for explicitly and reported as a missing instrument instead.

    python tools/dump_names.py     # regenerate items.txt
"""
import collections
import io
import os
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

SHOPS = os.path.join("data", "shops.txt")
ITEMS = os.path.join("gm-handbook", "items.txt")


def item_index():
    """name -> [ids]. Several ids can share a name; that is the point of the list."""
    by = collections.defaultdict(list)
    for line in io.open(ITEMS, encoding="utf-8"):
        line = line.strip()
        if not line or line.startswith("#") or "," not in line:
            continue
        iid, name = line.split(",", 1)
        by[name.strip()].append(iid.strip())
    return by


def shops():
    """[(npc, role, map, [(name, price, rank)])]"""
    out, cur = [], None
    for raw in io.open(SHOPS, encoding="utf-8"):
        line = raw.rstrip("\n")
        if line.startswith("#") or not line.strip():
            continue
        if line.startswith("shop:"):
            parts = [p.strip() for p in line[len("shop:"):].split("|")]
            while len(parts) < 3:
                parts.append("")
            cur = (parts[0], parts[1], parts[2], [])
            out.append(cur)
            continue
        if cur is None:
            continue
        cells = line.strip("\t").split("\t")
        if len(cells) < 2:
            continue
        name, price = cells[0].strip(), cells[1].strip()
        rank = cells[2].strip() if len(cells) > 2 else ""
        try:
            price = int(price.replace(",", ""))
        except ValueError:
            continue
        cur[3].append((name, price, rank))
    return out


def main():
    if not os.path.exists(ITEMS):
        print("%s is missing - the instrument, not the data, is absent." % ITEMS)
        print("  regenerate with: python tools/dump_names.py")
        return 2
    by = item_index()
    data = shops()
    want_ids = "--ids" in sys.argv

    total = resolved = 0
    missing, ambiguous = [], []
    for npc, _role, _map, items in data:
        for name, price, rank in items:
            total += 1
            ids = by.get(name, [])
            if not ids:
                missing.append((npc, name, price))
            elif len(ids) > 1:
                ambiguous.append((npc, name, ids))
                resolved += 1
            else:
                resolved += 1
                if want_ids:
                    print("%s\t%s\t%d\t%s" % (ids[0], name, price, rank))

    if want_ids:
        return 0

    print("data/shops.txt: %d shops, %d item rows" % (len(data), total))
    print("  resolved against the client's name table: %d" % resolved)
    print("  UNRESOLVED: %d" % len(missing))
    print("  ambiguous (name maps to several ids): %d" % len(ambiguous))
    if missing:
        print()
        print("Unresolved names - each is a transcription error, an item this client does")
        print("not have, or a different spelling. None of them can be sold until settled:")
        for npc, name, price in missing:
            print("  %-22s %-44s %d" % (npc, name, price))
    if ambiguous:
        print()
        print("Ambiguous - picking the first id would be a silent wrong answer:")
        for npc, name, ids in ambiguous[:40]:
            print("  %-22s %-40s %s" % (npc, name, ",".join(ids)))
    return 1 if missing else 0


if __name__ == "__main__":
    sys.exit(main())
