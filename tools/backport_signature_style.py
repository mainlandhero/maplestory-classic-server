#!/usr/bin/env python3
"""Extract every WZ asset behind the modern client's "Signature Style Collection".

    python tools/backport_signature_style.py
    python tools/backport_signature_style.py --source "C:/Nexon/Library/maplestory/appdata/Data" --out backport/signature-style

**Reads the modern client and writes only under `--out`.** Nothing under `--source` is
opened for writing, and the script refuses an `--out` that is inside it.

## What the collection is, read out of the modern client's data (2026-09-10)

Cash item `5222221` "Signature Style Collection" is a box: its own node carries `cash 1`,
`collabo 1` and an icon, nothing else. `Etc/Commodity.img` sells it (SN 110000115, 7,900 NX
for one; 110000116, 79,000 for ten; 7-day period) and sells NOTHING else in this family.
The ten "Outfit Set Coupon" cash items `5681543..5681552` and the "Outfit Set" consume items
`2830642..2830658` are all `spec/script = cash_NNN` / `consume_NNN` with `notConsume 1`: the
client asks the server to run a script and the SERVER decides what comes out. No package
table in `Etc` (`CashPackage.img`, `ClothingBox.img`) names any of them.

So the WZ supplies the ITEMS and their ART, and the set contents are server-side rules -
The owner's listing is that rule for MapleCW. This script extracts the items; `world` will hold
the rule.

## The layout, and why the extraction is two dumps per item

Both the modern client (v271) and the classic one (v779) keep an equip as two images: the
property tree in `Character/<Type>/<Type>_NNN.wz/<id>.img`, whose canvases are 1x1 stubs
carrying `_outlink`s, and the pixels in `Character/<Type>/_Canvas/_Canvas_NNN.wz/<id>.img`
at the same paths. Parts are discovered from `<Type>.ini`'s `LastWzIndex`, not from the
79-byte `<Type>.wz` stub, which is byte-identical for a one-part and a two-part tree.

**Outlinks cross images, so the pixels are NOT always under the item's own name.** A hair
colour variant (`42541`) owns no canvases: every one of its nodes outlinks into the base
colour's image (`_Canvas/00042540.img/...`). A face's expressions outlink into a shared sheet
(`_Canvas/00022000.img/blaze/...`) that no item is named after, and one Longcoat node points
at `01051850.img`. The first version of this script exported `_Canvas/<own id>.img` and
reported the variants as "0 canvases", which was true and useless. It now dumps the property
trees, collects every `_outlink`, exports each target image ONCE, and then checks that every
referenced path is in the export - a missing one fails the run.

## What is written

    <out>/manifest.json                  every item: id, type, names, source archive, its outlink targets
    <out>/manifest.md                    the same, readable, grouped by set
    <out>/strings.json                   name + desc for every id, keyed the way the CLASSIC
                                         String.wz keys them (`ClassicWorld/<Type>/<id>`)
    <out>/wz/<Type>/<id>/prop.json       the property tree (outlinks intact)
    <out>/wz/Item/<Cash|Consume>/<id>/prop.json   the box, the coupons and the sets
    <out>/canvas/<Tree>/<image>/canvas.json       the _Canvas image's tree, one per TARGET image
    <out>/canvas/<Tree>/<image>/*.bin, *.png      raw payloads (wz-dump canvas) and renders (wz_png.py)

Item ids are resolved by NAME against `String/Eqp.img`, `Cash.img` and `Consume.img`, and a
name that resolves to nothing aborts the run: a silently-missing item is exactly the failure
this repo keeps paying for.
"""
import argparse
import json
import os
import re
import subprocess
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
WZ_DUMP = os.path.join(REPO, "target", "release", "wz-dump" + (".exe" if os.name == "nt" else ""))
WZ_PNG = os.path.join(REPO, "tools", "wz_png.py")
DEFAULT_SOURCE = r"C:\Nexon\Library\maplestory\appdata\Data"

# ---------------------------------------------------------------------------------------
# The sets. Names are the modern client's own (String/Eqp.img, Cash.img, Consume.img) and
# are resolved to ids at run time. The owner's listing of 2026-09-10 is the rule for the three
# sets they spelled out; the other five follow the same shape from the name families.
# ---------------------------------------------------------------------------------------

# Equip names per character. Hair and Face resolve to EVERY colour variant (8 hair, 9 face).
SETS = {
    "Frieren": {
        "hair": ["Frieren Hair", "Frieren Hair (Ringlets)", "Frieren Hair (Sleep)"],
        "face": ["Frieren Face"],
        "equips": [
            "Frieren's Clothes", "Frieren's Winter Clothes", "Frieren's Sleep Clothes",
            "Frieren's Shoes", "Frieren's Earrings", "Frieren's Staff",
        ],
    },
    "Fern": {
        "hair": ["Fern Hair"],
        "face": ["Fern Face"],
        "equips": ["Fern's Clothes", "Fern's Winter Clothes", "Fern's Shoes", "Fern's Staff"],
    },
    "Stark": {
        "hair": ["Stark Hair"],
        "face": ["Stark Face"],
        "equips": [
            "Stark's Clothes", "Stark's Winter Clothes", "Stark's Shoes", "Stark's Winter Shoes",
            "Stark's Gloves", "Stark's Winter Gloves", "Stark's Axe",
        ],
    },
    "\u00dcbel": {
        "hair": ["\u00dcbel Hair"],
        "face": ["\u00dcbel Face"],
        "equips": ["\u00dcbel's Clothes", "\u00dcbel's Shoes", "\u00dcbel's Gloves", "\u00dcbel's Staff"],
    },
    "Himmel": {
        "hair": ["Himmel Hair"],
        "face": ["Himmel Face"],
        # Himmel's Blessing is a cape; the modern Himmel set carries a coupon for it (2830654).
        "equips": ["Himmel's Clothes", "Himmel's Shoes", "Himmel's Sword", "Himmel's Blessing"],
    },
    # Aura, Linie and L\u00fcgner have no hairstyle: their hair is a CAP ("... Hair (Hat)").
    "Aura": {
        "hair": [],
        "face": ["Aura Face"],
        "equips": [
            "Aura Hair (Hat)", "Aura's Clothes", "Aura's Shoes", "Aura's Gloves",
            "Aura's Scales of Obedience",
        ],
    },
    "L\u00fcgner": {
        "hair": [],
        "face": ["L\u00fcgner Face"],
        "equips": ["L\u00fcgner Hair (Hat)", "L\u00fcgner's Clothes", "L\u00fcgner's Shoes"],
    },
    "Linie": {
        "hair": [],
        "face": ["Linie Face"],
        "equips": ["Linie Hair (Hat)", "Linie's Clothes", "Linie's Shoes"],
    },
}

# The box and the two coupon layers, by name, in the modern String tables.
CASH_NAMES = [
    "Signature Style Collection",
    "Frieren Outfit Set Coupon", "Frieren Outfit Set (Ringlets) Coupon",
    "Frieren Outfit Set (Sleep) Coupon", "Fern Outfit Set Coupon", "Stark Outfit Set Coupon",
    "\u00dcbel Outfit Set Coupon", "Himmel Outfit Set Coupon", "Aura Outfit Set Coupon",
    "L\u00fcgner Outfit Set Coupon", "Linie Outfit Set Coupon",
]
CONSUME_NAMES = [
    # the sets
    "Frieren Outfit Set", "Frieren Outfit Set (Ringlets)", "Frieren Outfit Set (Sleep)",
    "Fern Outfit Set", "Stark Outfit Set", "\u00dcbel Outfit Set", "Himmel Outfit Set",
    "Aura Outfit Set", "L\u00fcgner Outfit Set", "Linie Outfit Set",
    # the selectors the modern sets hand out
    "Frieren Clothes Selector Coupon", "Fern's Clothes Selector Coupon",
    "Stark's Clothes Selector Coupon", "Stark's Shoes Selector Coupon",
    "Stark's Gloves Selector Coupon", "Himmel's Blessing Coupon",
    # hair coupons
    "Frieren Hair Coupon", "Frieren Hair (Ringlets) Coupon", "Frieren Hair (Sleep) Coupon",
    "Fern Hair Coupon", "Stark Hair Coupon", "Himmel Hair Coupon", "\u00dcbel Hair Coupon",
    # face coupons
    "Frieren Face Coupon", "Fern Face Coupon", "Stark Face Coupon", "Himmel Face Coupon",
    "\u00dcbel Face Coupon", "Aura Face Coupon", "Linie Face Coupon", "L\u00fcgner Face Coupon",
]


def run(*args):
    """Run wz-dump and return stdout; a non-zero exit is fatal, with the stderr shown."""
    p = subprocess.run([WZ_DUMP, *args], capture_output=True, text=True, encoding="utf-8",
                       errors="replace")
    if p.returncode != 0:
        raise SystemExit("wz-dump %s failed:\n%s" % (" ".join(args[:2]), p.stderr.strip()))
    return p.stdout


def last_index(tree_dir):
    """`LastWzIndex|N` from the tree's ini; the number of parts is N + 1."""
    name = os.path.basename(tree_dir)
    ini = os.path.join(tree_dir, name + ".ini")
    with open(ini, encoding="utf-8", errors="replace") as fh:
        m = re.search(r"LastWzIndex\|(\d+)", fh.read())
    if not m:
        raise SystemExit("%s has no LastWzIndex" % ini)
    return int(m.group(1))


class Tree:
    """One WZ tree (`Character/Hair`, `Item/Cash`, ...): its parts and their image lists."""

    def __init__(self, tree_dir):
        self.dir = tree_dir
        self.name = os.path.basename(tree_dir)
        self.parts = []
        self.images = {}  # image name -> part path
        for i in range(last_index(tree_dir) + 1):
            part = os.path.join(tree_dir, "%s_%03d.wz" % (self.name, i))
            if not os.path.exists(part):
                raise SystemExit("%s names part %d but %s is missing" % (self.name, i, part))
            self.parts.append(part)
            for line in run("tree", part, "1").splitlines():
                if "[IMG]" in line:
                    img = line.split("[IMG]", 1)[1].split()[0]
                    self.images.setdefault(img, part)

    def cat(self, image):
        part = self.images.get(image)
        if part is None:
            return None, None
        return part, json.loads(run("cat", part, image))


def string_names(data_dir, img):
    """`{id: (type, name, desc)}` from one modern String image."""
    node = json.loads(run("cat", os.path.join(data_dir, "String", "String_000.wz"), img))
    out = {}

    def walk(n, path):
        if not isinstance(n, dict):
            return
        name = n.get("name")
        if isinstance(name, str) and path and path[-1].isdigit():
            kind = path[-2] if len(path) >= 2 else ""
            out[int(path[-1])] = (kind, name, n.get("desc", "") or "")
        for k, v in n.items():
            walk(v, path + [k])

    walk(node, [])
    return out


def ids_named(table, name, allow_many):
    """Every id in `table` whose name is exactly `name`. Fatal if none."""
    hits = sorted(i for i, (_, n, _) in table.items() if n == name)
    if not hits:
        raise SystemExit("no item named %r in the modern client's strings" % name)
    if len(hits) > 1 and not allow_many:
        raise SystemExit("%r names %d items, expected one: %s" % (name, len(hits), hits))
    return hits


def equip_tree_for(kind):
    """Which `Character/` tree a String category lives in. Same names both clients use."""
    return kind


def image_name(item_id, kind):
    # Hair/Face are `000NNNNN.img`, everything else `0NNNNNNN.img`: eight digits either way.
    return "%08d.img" % item_id


def dump_prop(prop_tree, image, dest, note, src, node_key=None):
    """Dump one property image (or one keyed node of it) and return its outlink targets."""
    part, prop = prop_tree.cat(image)
    if prop is None:
        raise SystemExit("%s: %s is in no part of %s" % (note, image, prop_tree.dir))
    if node_key is not None:
        prop = prop.get(node_key)
        if prop is None:
            raise SystemExit("%s: %s has no %s" % (note, image, node_key))
    os.makedirs(dest, exist_ok=True)
    with open(os.path.join(dest, "prop.json"), "w", encoding="utf-8") as fh:
        json.dump(prop, fh, indent=1, ensure_ascii=False)
    links = outlinks(prop)
    return prop, {
        "prop_archive": os.path.relpath(part, src),
        "outlinks": len(links),
        "canvas_images": sorted({"%s/_Canvas/%s" % (t, i) for (t, i, _) in links}),
    }, links


def outlinks(node):
    """Every `_outlink` under `node` as `(tree, image, path)` - e.g.
    `("Character/Face", "00022000.img", "blaze/0/face")`."""
    out = set()
    if isinstance(node, dict):
        ol = node.get("_outlink")
        if ol:
            parts = ol.split("/")
            i = parts.index("_Canvas")
            out.add(("/".join(parts[:i]), parts[i + 1], "/".join(parts[i + 2:])))
        for v in node.values():
            out |= outlinks(v)
    return out


def export_canvases(src, out, targets):
    """Export every target `_Canvas` image once and verify every referenced path is there."""
    missing = []
    by_image = {}
    for tree_rel, image, path in targets:
        by_image.setdefault((tree_rel, image), set()).add(path)
    trees = {}
    for (tree_rel, image), paths in sorted(by_image.items()):
        if tree_rel not in trees:
            trees[tree_rel] = Tree(os.path.join(src, *(tree_rel + "/_Canvas").split("/")))
        cpart, canvas = trees[tree_rel].cat(image)
        if canvas is None:
            missing.append(("%s/_Canvas/%s" % (tree_rel, image), "<whole image>"))
            continue
        dest = os.path.join(out, "canvas", *tree_rel.split("/"), image[:-4])
        os.makedirs(dest, exist_ok=True)
        with open(os.path.join(dest, "canvas.json"), "w", encoding="utf-8") as fh:
            json.dump(canvas, fh, indent=1, ensure_ascii=False)
        run("canvas", cpart, image, dest)
        with open(os.path.join(dest, "manifest.json"), encoding="utf-8") as fh:
            have = {e["node"].strip("/") for e in json.load(fh)}
        subprocess.run([sys.executable, WZ_PNG, dest], check=True, capture_output=True)
        for missing_path in sorted(paths - have):
            missing.append(("%s/_Canvas/%s" % (tree_rel, image), missing_path))
        print("  canvas %-22s %-14s %4d exported, %3d referenced" % (tree_rel, image, len(have), len(paths)))
    return by_image, missing


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--source", default=DEFAULT_SOURCE, help="the modern client's Data dir (read-only)")
    ap.add_argument("--out", default=os.path.join(REPO, "backport", "signature-style"))
    args = ap.parse_args()

    src = os.path.abspath(args.source)
    out = os.path.abspath(args.out)
    if out.lower().startswith(src.lower()):
        raise SystemExit("--out must not be inside --source: nothing is written to the modern client")
    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)
    os.makedirs(out, exist_ok=True)

    print("reading names from", os.path.join(src, "String"))
    eqp = string_names(src, "Eqp.img")
    cash = string_names(src, "Cash.img")
    consume = string_names(src, "Consume.img")

    trees = {}

    def tree(rel):
        if rel not in trees:
            trees[rel] = Tree(os.path.join(src, *rel.split("/")))
        return trees[rel]

    manifest = {"source": src, "sets": {}, "cash": [], "consume": []}
    strings = {"ClassicWorld": {}, "Cash": {}, "Consume": {}}
    total = 0
    targets = set()

    for set_name, spec in SETS.items():
        items = []
        wanted = [(n, True) for n in spec["hair"]] + [(n, True) for n in spec["face"]] + \
                 [(n, False) for n in spec["equips"]]
        for name, many in wanted:
            for item_id in ids_named(eqp, name, allow_many=many):
                kind, _, desc = eqp[item_id]
                prop_tree = tree("Character/" + equip_tree_for(kind))
                dest = os.path.join(out, "wz", kind, "%08d" % item_id)
                _, info, links = dump_prop(prop_tree, image_name(item_id, kind), dest,
                                           "%s / %s" % (set_name, name), src)
                targets |= links
                info.update({"id": item_id, "type": kind, "name": name, "desc": desc})
                items.append(info)
                strings["ClassicWorld"].setdefault(kind, {})[str(item_id)] = \
                    {"name": name, **({"desc": desc} if desc else {})}
                total += 1
                print("  %-8s %-10s %8d  %-32s outlinks=%d -> %s" % (
                    set_name, kind, item_id, name, info["outlinks"],
                    ", ".join(i.rsplit("/", 1)[1] for i in info["canvas_images"])))
        manifest["sets"][set_name] = items

    for label, names, table, tree_rel, bucket in [
        ("cash", CASH_NAMES, cash, "Item/Cash", "Cash"),
        ("consume", CONSUME_NAMES, consume, "Item/Consume", "Consume"),
    ]:
        prop_tree = tree(tree_rel)
        for name in names:
            for item_id in ids_named(table, name, allow_many=False):
                _, _, desc = table[item_id]
                # Items are grouped by their first four digits: 5222221 -> 0522.img.
                image = "%04d.img" % (item_id // 10000)
                key = "%08d" % item_id
                dest = os.path.join(out, "wz", "Item", bucket, key)
                node, info, links = dump_prop(prop_tree, image, dest, name, src, node_key=key)
                targets |= links
                spec = node.get("spec", {})
                info.update({
                    "id": item_id, "name": name, "desc": desc,
                    "script": spec.get("script"),
                    "npc": spec.get("npc"),
                    "info": {k: v for k, v in node.get("info", {}).items()
                             if not (isinstance(v, dict) and v.get("_canvas"))},
                })
                manifest[label].append(info)
                strings[bucket][str(item_id)] = {"name": name, **({"desc": desc} if desc else {})}
                total += 1
                print("  %-8s %8d  %-42s script=%s" % (label, item_id, name, spec.get("script")))

    print("exporting the %d canvas images every outlink resolves to" % len({(t, i) for t, i, _ in targets}))
    by_image, missing = export_canvases(src, out, targets)
    manifest["canvas_images"] = {
        "%s/_Canvas/%s" % (t, i): sorted(paths) for (t, i), paths in sorted(by_image.items())
    }
    manifest["unresolved_outlinks"] = ["%s -> %s" % m for m in missing]
    with open(os.path.join(out, "manifest.json"), "w", encoding="utf-8") as fh:
        json.dump(manifest, fh, indent=1, ensure_ascii=False)
    with open(os.path.join(out, "strings.json"), "w", encoding="utf-8") as fh:
        json.dump(strings, fh, indent=1, ensure_ascii=False)
    write_markdown(manifest, os.path.join(out, "manifest.md"))
    print("%d items, %d canvas images -> %s" % (total, len(by_image), out))
    if missing:
        print("%d outlinks resolve to NOTHING in the modern client:" % len(missing))
        for m in missing:
            print("  %s -> %s" % m)
        return 1
    print("every outlink resolved")
    return 0


def write_markdown(manifest, path):
    lines = ["# Signature Style Collection - extracted from the modern client", "",
             "Source: `%s`. Generated by `tools/backport_signature_style.py`; do not hand-edit." % manifest["source"],
             ""]
    lines += ["## The box and the coupons (server-scripted; contents are NOT in the WZ)", "",
              "| layer | id | name | script | canvas image |", "|---|---|---|---|---|"]
    for label in ("cash", "consume"):
        for it in manifest[label]:
            lines.append("| %s | %d | %s | `%s` | %s |" % (
                label, it["id"], it["name"], it["script"], ", ".join(it["canvas_images"])))
    for set_name, items in manifest["sets"].items():
        lines += ["", "## %s (%d ids)" % (set_name, len(items)), "",
                  "| id | type | name | property archive | outlinks | pixels live in |",
                  "|---|---|---|---|---|---|"]
        for it in items:
            lines.append("| %d | %s | %s | `%s` | %d | %s |" % (
                it["id"], it["type"], it["name"], it["prop_archive"], it["outlinks"],
                ", ".join(i.rsplit("/", 1)[1] for i in it["canvas_images"])))
    lines += ["", "## Canvas images exported (%d)" % len(manifest["canvas_images"]), ""]
    for img, paths in manifest["canvas_images"].items():
        lines.append("- `%s` - %d referenced paths" % (img, len(paths)))
    if manifest["unresolved_outlinks"]:
        lines += ["", "## UNRESOLVED outlinks", ""] + ["- " + m for m in manifest["unresolved_outlinks"]]
    with open(path, "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines) + "\n")


if __name__ == "__main__":
    sys.exit(main())
