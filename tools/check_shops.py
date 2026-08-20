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

## The (M) / (F) tag

A trailing `(M)` or `(F)` on a shop row is a **disambiguator, not decoration**. MapleStory
equips encode gender in the id - `(id / 1000) % 10` is 0 male, 1 female, 2+ unisex - so
`Green Bennis Chainmail` is two items, 1040033 and 1041040, and Sam sells both. Stripping the
tag and taking the first id would be exactly the silent wrong answer this file exists to
prevent, so the tag is applied as a filter over the candidate ids instead.

That the convention holds *in this client* is measured, not assumed: across the 89 tagged
rows there is no row whose tag disagrees with the gender its id implies. `--gender-audit`
re-runs that check, and it is the control for every tagged row in the file.

## The control

`gm-handbook/items.txt` is gitignored generated data. If it is missing this reports every
name as unresolved, which looks exactly like a catastrophic transcription failure - so the
absence is checked for explicitly and reported as a missing instrument instead.

    python tools/dump_names.py     # regenerate items.txt
"""
import collections
import io
import os
import re
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

SHOPS = os.path.join("data", "shops.txt")
ITEMS = os.path.join("gm-handbook", "items.txt")

GENDER_TAG = re.compile(r"^(.*?)\s*\((M|F)\)$")


def gender_of(item_id):
    """M / F / U for an equip id, '-' for anything else.

    Equips are 1xxxxxx and the thousands digit is the gender. Non-equips have no gender, so
    a tag on one is neither satisfied nor violated - it just does not filter.
    """
    if not 1000000 <= item_id < 2000000:
        return "-"
    return {0: "M", 1: "F"}.get((item_id // 1000) % 10, "U")


def resolve(name, index):
    """(ids, tag) for a shop row's name, the tag having already filtered the candidates.

    The name is looked up whole first. Only if that fails is the tag peeled off, so a client
    name that genuinely ends in `(M)` - `Natalie's Fashion Box (M)` is one - resolves as
    itself rather than being mangled into a filter.
    """
    if name in index:
        return index[name], None
    m = GENDER_TAG.match(name)
    if not m:
        return [], None
    base, tag = m.group(1), m.group(2)
    ids = index.get(base, [])
    return [i for i in ids if gender_of(i) in (tag, "U", "-")], tag


def item_index():
    """name -> [ids]. Several ids can share a name; that is the point of the list.

    Ids are ints, because `gender_of` does arithmetic on them.
    """
    by = collections.defaultdict(list)
    for line in io.open(ITEMS, encoding="utf-8"):
        line = line.strip()
        if not line or line.startswith("#") or "," not in line:
            continue
        iid, name = line.split(",", 1)
        by[name.strip()].append(int(iid.strip()))
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
    audit = "--gender-audit" in sys.argv

    total = resolved = tagged = 0
    missing, ambiguous, violations = [], [], []
    for npc, _role, _map, items in data:
        for name, price, rank in items:
            total += 1
            ids, tag = resolve(name, by)
            if tag is not None:
                tagged += 1
                # The control for the whole (M)/(F) convention: a tag that filtered every
                # candidate away, on a name the untagged table does know, is a tag that
                # disagrees with the client. That is a finding, not a missing item.
                if not ids and by.get(GENDER_TAG.match(name).group(1)):
                    violations.append(
                        (npc, name, by[GENDER_TAG.match(name).group(1)]))
            if not ids:
                missing.append((npc, name, price))
            elif len(ids) > 1:
                ambiguous.append((npc, name, ids))
                resolved += 1
            else:
                resolved += 1
                if want_ids:
                    print("%d\t%s\t%d\t%s" % (ids[0], name, price, rank))

    if want_ids:
        return 0

    print("data/shops.txt: %d shops, %d item rows" % (len(data), total))
    print("  resolved against the client's name table: %d" % resolved)
    print("  UNRESOLVED: %d" % len(missing))
    print("  ambiguous (name maps to several ids): %d" % len(ambiguous))
    print("  rows disambiguated by a (M)/(F) tag: %d" % tagged)
    print("  tags that disagree with the client's own id convention: %d" % len(violations))
    if audit:
        print()
        print("Gender audit - every tagged row, and the gender its resolved id implies:")
        for npc, _role, _map, items in data:
            for name, price, rank in items:
                ids, tag = resolve(name, by)
                if tag is None:
                    continue
                print("  %-22s %-40s (%s) -> %s" % (
                    npc, name, tag,
                    ", ".join("%d[%s]" % (i, gender_of(i)) for i in ids) or "NOTHING"))
    if missing:
        print()
        print("Unresolved names - each is a transcription error, an item this client does")
        print("not have, or a different spelling. None of them can be sold until settled:")
        for npc, name, price in missing:
            print("  %-22s %-44s %d" % (npc, name, price))
    if violations:
        print()
        print("A (M)/(F) tag that no candidate id satisfies. The tag is wrong, or the")
        print("gender-from-id convention does not hold for this item:")
        for npc, name, ids in violations:
            print("  %-22s %-40s candidates %s" % (
                npc, name, ", ".join("%d[%s]" % (i, gender_of(i)) for i in ids)))
    if ambiguous:
        print()
        print("Ambiguous - picking the first id would be a silent wrong answer. A (M)/(F)")
        print("tag settles the gender pairs; anything else needs shop context:")
        for npc, name, ids in ambiguous[:40]:
            print("  %-22s %-40s %s" % (
                npc, name, ",".join("%d[%s]" % (i, gender_of(i)) for i in ids)))
    return 1 if missing or ambiguous else 0


if __name__ == "__main__":
    sys.exit(main())
