#!/usr/bin/env python3
"""Extract the id -> name tables from String.wz into plain `id, name` text files.

Produces three files: maps, items and mobs. Items are the union of the six item
categories, because "items in the game" is not one image:

    Eqp.img       equipment, nested one level deeper than the rest
    Consume.img   potions and usables
    Ins.img       setup / installable
    Etc.img       etc / crafting
    Cash.img      cash shop
    Pet.img       pets

The walker does not hard-code the nesting depth. It descends any node and emits a row for
every node whose key is all digits and which carries a name - which is why the same code
handles `Mob.img` (flat), `Map.img` (one category level) and `Eqp.img` (two) without three
different parsers, and will keep working if a category is added.

    python tools/dump_names.py                     # writes into gm-handbook/
    python tools/dump_names.py --out-dir somewhere
    python tools/dump_names.py --street            # prefix map names with their street

Output goes to `gm-handbook/`. These files are game data, not project source, so they are
gitignored deliberately - the repo carries the code to regenerate them, not the content.
"""
import argparse
import json
import os
import subprocess
import sys

STRING_WZ = os.path.join("client-patched", "Data", "String", "String_000.wz")
WZ_DUMP = os.path.join("target", "release", "wz-dump")

ITEM_IMAGES = ["Eqp.img", "Consume.img", "Ins.img", "Etc.img", "Cash.img", "Pet.img"]


def read_image(archive, image):
    """One image out of the archive, as parsed JSON."""
    exe = WZ_DUMP + (".exe" if os.name == "nt" else "")
    if not os.path.exists(exe):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % exe)
    r = subprocess.run([exe, "cat", archive, image], capture_output=True, text=True, encoding="utf-8")
    if r.returncode != 0:
        raise SystemExit("wz-dump cat %s failed: %s" % (image, r.stderr.strip()))
    return json.loads(r.stdout)


def walk(node, name_keys, out, street=False):
    """Emit (id, name) for every all-digit key carrying one of `name_keys`."""
    if not isinstance(node, dict):
        return
    for key, value in node.items():
        if not isinstance(value, dict):
            continue
        if key.isdigit():
            label = None
            for want in name_keys:
                if isinstance(value.get(want), str):
                    label = value[want]
                    break
            if label is not None:
                if street and isinstance(value.get("streetName"), str):
                    label = "%s: %s" % (value["streetName"], label)
                # Map ids are zero-padded in the archive; the wire uses the integer.
                out[int(key)] = label.replace("\n", " ").replace("\r", " ").strip()
                continue
        walk(value, name_keys, out, street)


def write(path, rows, what):
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        for ident in sorted(rows):
            fh.write("%d, %s\n" % (ident, rows[ident]))
    print("%-10s %6d rows  ->  %s" % (what, len(rows), path))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--archive", default=STRING_WZ)
    ap.add_argument("--out-dir", default="gm-handbook")
    ap.add_argument("--street", action="store_true",
                    help="prefix each map name with its street, which disambiguates the "
                         "many maps sharing a name")
    args = ap.parse_args()

    if not os.path.exists(args.archive):
        raise SystemExit("no archive at %s" % args.archive)
    os.makedirs(args.out_dir, exist_ok=True)

    maps = {}
    walk(read_image(args.archive, "Map.img"), ["mapName"], maps, street=args.street)
    write(os.path.join(args.out_dir, "maps.txt"), maps, "maps")

    mobs = {}
    walk(read_image(args.archive, "Mob.img"), ["name"], mobs)
    # **Not `mobs.txt`.** That name belongs to tools/dump_portals.py's SPAWN table (map,
    # template, x, cy, fh, ..., name), which the world server loads and which already carries
    # the name in its last column. This tool wrote the same file first, as an id/name table,
    # and on 2026-09-10 a re-run of it overwrote the spawn table with 3 KB of names - every
    # map lost its mobs and four tests said so. Nothing reads the name-only table, so it goes
    # under its own name.
    write(os.path.join(args.out_dir, "mobnames.txt"), mobs, "mob names")

    items = {}
    for image in ITEM_IMAGES:
        before = len(items)
        walk(read_image(args.archive, image), ["name"], items)
        print("  %-14s +%d" % (image, len(items) - before))
    write(os.path.join(args.out_dir, "items.txt"), items, "items")
    return 0


if __name__ == "__main__":
    sys.exit(main())
