#!/usr/bin/env python3
"""Every summoning sack, and the mobs it spawns, out of the client's own `Item.wz`.

Writes `gm-handbook/summonsacks.txt`. Generated game data, gitignored, never hand-edited -
same contract as `tools/dump_mobs.py` and `tools/dump_itemdata.py`.

    python tools/dump_summon_sacks.py

## What a summoning sack is, in this client's data

A consumable whose image node carries a **`mob` child that is a sibling of `info`**, not a
member of it:

```
Item/Consume/Consume_000.wz :: 0210.img / 02100007
    info/  slotMax 10   type 1   price 1   icon   iconRaw
    mob/
        0/ id 700005   prob 100
        1/ id 700005   prob 100
```

Two things fall out of that shape and both matter to the server:

* **How MANY of a mob to spawn is the number of `mob` entries naming it**, not a count
  field. `02100007` carries `700005` twice and means two Balrogs. Nothing else in the node
  says "two".
* **`prob` is 100 on every entry in this client** (8 items, 9 entries). It is emitted anyway
  rather than collapsed to "always", because a table that drops a column cannot later be
  asked whether the column was uniform or absent.

## Enumerated, not assumed

The family is **found**, not filtered to `0210.img`: every image of every `Item/*` archive
and every `Character/*` equip archive is walked and any node with a `mob` child is emitted.
On this client that returns 8 rows and all 8 are in `Consume/0210.img` - so "the 0210 prefix
is the summoning sack family" is a *result* here, not an input. If a later client puts one
somewhere else this still finds it.

## It refuses to write rather than write a short table

`CLAUDE.md`: a silently-short generated table looks exactly like a working one. This exits
non-zero, having written nothing, if

* `wz-dump` or an archive is missing,
* the walk finds **zero** sacks,
* any `mob` entry has no `id`,
* any spawned template has **no `Mob/%07d.img` in `Mob_000.wz`**.

The last one is not tidiness. `research/mob-spawn.md`: `MobEnterField`'s HP field is divided
by the template's max HP at `141c50502` by an `IDIV` **with no zero guard**, so a template the
client cannot load is a template the server must never send. A sack row that names one is a
crash waiting for a double-click, and it is better to have no file than that row.
"""
import json
import os
import subprocess
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

DATA_ROOT = os.path.join("client-patched", "Data")
STRING_WZ = os.path.join(DATA_ROOT, "String", "String_000.wz")
MOB_ARCHIVE = os.path.join(DATA_ROOT, "Mob", "Mob_000.wz")
OUT = os.path.join("gm-handbook", "summonsacks.txt")
WZ_DUMP = os.path.abspath(
    os.path.join("target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
)

# Every archive an item can live in. Same two shapes `tools/dump_itemdata.py` handles:
# `Item/<cat>` images whose children are the 8-digit ids, and `Character/<slot>` images that
# are themselves one item. The walk below does not need to be told which is which.
ITEM_CATEGORIES = ["Consume", "Etc", "Install", "Cash", "Special", "Pet"]
EQUIP_SLOTS = ["Accessory", "Cap", "Cape", "Coat", "Glove", "Longcoat", "Pants",
               "Ring", "Shield", "Shoes", "Weapon", "PetEquip"]


def run(*args):
    r = subprocess.run([WZ_DUMP, *args], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        return None
    return r.stdout


def images(archive):
    out = run("tree", archive, "1")
    if not out:
        return []
    return [ln.split("[IMG]", 1)[1].split()[0] for ln in out.splitlines() if "[IMG]" in ln]


def image_json(archive, image):
    out = run("cat", archive, image)
    if out is None:
        raise SystemExit("wz-dump cat %s %s failed" % (archive, image))
    return json.loads(out)


def find_sacks(node, path, found):
    """Any node with a `mob` child is a sack. Recursive, so nesting depth is not assumed."""
    if not isinstance(node, dict):
        return
    for key, value in node.items():
        if not isinstance(value, dict):
            continue
        if isinstance(value.get("mob"), dict) and key.isdigit():
            found.append((int(key), value))
            continue
        find_sacks(value, path + "/" + key, found)


def mob_entries(node):
    """`mob/0`, `mob/1`, ... in numeric order, as `(templateId, prob)`.

    Ordered by the numeric key rather than by dict order, because the count of a repeated
    template is what says how many to spawn and a shuffled list would still be right - but a
    list read in insertion order stops being reproducible the moment the dumper changes.
    """
    out = []
    for key in sorted(node["mob"], key=lambda k: int(k) if k.isdigit() else 1 << 30):
        entry = node["mob"][key]
        if not isinstance(entry, dict) or not isinstance(entry.get("id"), int):
            return None
        prob = entry.get("prob")
        out.append((entry["id"], prob if isinstance(prob, int) else -1))
    return out or None


def string_names(image, out):
    """`id -> name` out of String.wz. Same walk as `tools/dump_names.py`."""
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
    walk(image_json(STRING_WZ, image))


def main():
    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)
    for path in (STRING_WZ, MOB_ARCHIVE):
        if not os.path.exists(path):
            raise SystemExit("%s is missing" % path)

    archives = [os.path.join(DATA_ROOT, "Item", c, "%s_000.wz" % c) for c in ITEM_CATEGORIES]
    archives += [os.path.join(DATA_ROOT, "Character", s, "%s_000.wz" % s) for s in EQUIP_SLOTS]

    found, scanned = [], 0
    for archive in archives:
        if not os.path.exists(archive):
            raise SystemExit("%s is missing" % archive)
        for image in images(archive):
            scanned += 1
            here = []
            find_sacks(image_json(archive, image), "%s::%s" % (archive, image), here)
            for item_id, node in here:
                found.append((item_id, node, "%s/%s" % (os.path.basename(archive), image)))
    print("scanned %d images across %d archives" % (scanned, len(archives)))

    if not found:
        raise SystemExit(
            "no item in this client carries a `mob` node. That is either a client with no "
            "summoning sacks or a broken walk - either way nothing is written, because a "
            "summonsacks.txt with no rows reads on the server exactly like one that failed "
            "to load"
        )

    # Every template the client can actually construct. `0700004.img` -> 700004.
    templates = set()
    for image in images(MOB_ARCHIVE):
        stem = image.split(".", 1)[0]
        if stem.isdigit():
            templates.add(int(stem))
    if not templates:
        raise SystemExit("no mob templates were read out of %s" % MOB_ARCHIVE)

    item_names, mob_names = {}, {}
    string_names("Consume.img", item_names)
    string_names("Mob.img", mob_names)

    rows, missing = [], []
    for item_id, node, where in sorted(found):
        entries = mob_entries(node)
        if entries is None:
            raise SystemExit("item %d in %s has a `mob` node this walk cannot read" % (item_id, where))
        for template, _ in entries:
            if template not in templates:
                missing.append((item_id, template))
        info = node.get("info", {})
        rows.append((
            item_id,
            info.get("slotMax", 0) if isinstance(info.get("slotMax"), int) else 0,
            info.get("price", 0) if isinstance(info.get("price"), int) else 0,
            entries,
            item_names.get(item_id, ""),
            where,
        ))
    if missing:
        for item_id, template in missing:
            print("  item %d spawns template %d, which has no image in %s"
                  % (item_id, template, MOB_ARCHIVE), file=sys.stderr)
        raise SystemExit(
            "refusing to write: a template with no Mob/%07d.img has no maxHP, and "
            "MobEnterField's HP field is divided by maxHP at 141c50502 with no zero guard. "
            "See research/mob-spawn.md"
        )

    os.makedirs("gm-handbook", exist_ok=True)
    with open(OUT, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# itemId, slotMax, price, mobCount, mobs, name\n")
        fh.write("# `mobs` is templateId:prob separated by ';' - a template listed TWICE "
                 "means spawn TWO of it\n")
        fh.write("# generated by tools/dump_summon_sacks.py from the client's own Item.wz "
                 "(the `mob` node beside `info`) and String.wz. Never hand-edit.\n")
        for item_id, slot_max, price, entries, name, _where in rows:
            mobs = ";".join("%d:%d" % (t, p) for t, p in entries)
            fh.write("%d, %d, %d, %d, %s, %s\n"
                     % (item_id, slot_max, price, len(entries), mobs, name))
    print("summonsacks %4d rows  ->  %s" % (len(rows), OUT))
    for item_id, _s, _p, entries, name, where in rows:
        print("  %d  %-34s %-22s %s"
              % (item_id, name, where,
                 ", ".join("%d x%d (%s)" % (t, sum(1 for u, _ in entries if u == t),
                                            mob_names.get(t, "?"))
                           for t in sorted({t for t, _ in entries}))))
    return 0


if __name__ == "__main__":
    sys.exit(main())
