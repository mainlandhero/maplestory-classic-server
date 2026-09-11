#!/usr/bin/env python3
r"""Build the hybrid classic + modern WZ archives, and install them into `client-patched/Data`.

    python tools/backport_install.py            # build into backport/signature-style/build/
    python tools/backport_install.py --install  # ...and copy over client-patched/Data, keeping .bak
    python tools/backport_install.py --revert   # put every .bak back

Second half of the backport. `tools/backport_signature_style.py` located and extracted the
assets from the modern client; this reads that run's `manifest.json` and, for every classic
archive the assets touch, writes a `wz-dump build` spec and runs it against the CLASSIC
archive as the base. Every existing image is carried over byte for byte; ours are added.

## What goes where

| classic archive | how |
|---|---|
| `Character/<Type>/<Type>_000.wz` | `copy` each item's property image from the modern part that holds it. Ids are all new to the classic client, so nothing is replaced |
| `Character/<Type>/_Canvas/_Canvas_000.wz` | `copy` each `_Canvas` image the property trees outlink into - including the shared ones (`Face/00022000.img`, `Longcoat/01051850.img`) |
| `Item/Cash/Cash_000.wz`, `Item/Consume/Consume_000.wz` | `merge`: an item image (`0522.img`) holds every item with that prefix, so only OUR nodes are taken from the modern image and laid onto the classic image of the same name (or an empty one) |
| `Item/<Cash|Consume>/_Canvas/_Canvas_000.wz` | `merge`, the same keys |
| `String/String_000.wz` | `strings`: `Eqp.img` (under `ClassicWorld/<Type>/<id>`), `Cash.img` and `Consume.img` (flat `<id>`) gain `name` and `desc` leaves |

The modern client is opened read-only as the SOURCE of every copy; nothing there is
written. The classic base is `client-patched/Data`, which is the copy this project exists
to patch (`CLAUDE.md`); the untouched original is `C:\Nexon\Library\maplestorycw`.

## Two things this deliberately does NOT do

* It does not strip the modern-only nodes (`info/level/.../EquipmentSkill` on the weapons,
  `islot HrCp` on the three hats). The first client run is the measurement of whether the
  classic client tolerates them; stripping first would make a clean result uninformative.
* It does not touch `Base/Base.wz` or any `.ini`: no tree is added and no part count
  changes, so neither needs to.
"""
import argparse
import json
import os
import shutil
import subprocess
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
WZ_DUMP = os.path.join(REPO, "target", "release", "wz-dump" + (".exe" if os.name == "nt" else ""))
EXTRACT = os.path.join(REPO, "backport", "signature-style")
CLASSIC = os.path.join(REPO, "client-patched", "Data")
CLASSIC_VERSION = "779"


def modern_part(source, tree_rel, image):
    """Which part of a modern tree holds `image`, via the tree listing."""
    tree_dir = os.path.join(source, *tree_rel.split("/"))
    name = os.path.basename(tree_dir)
    with open(os.path.join(tree_dir, name + ".ini"), encoding="utf-8", errors="replace") as fh:
        last = int(fh.read().split("|")[1])
    for i in range(last + 1):
        part = os.path.join(tree_dir, "%s_%03d.wz" % (name, i))
        out = subprocess.run([WZ_DUMP, "tree", part, "1"], capture_output=True, text=True,
                             encoding="utf-8", errors="replace").stdout
        if any(line.split("[IMG]", 1)[1].split()[0] == image
               for line in out.splitlines() if "[IMG]" in line):
            return part
    raise SystemExit("%s is in no part of %s" % (image, tree_rel))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--install", action="store_true", help="copy the built archives over client-patched/Data")
    ap.add_argument("--revert", action="store_true", help="restore every .bak under client-patched/Data")
    ap.add_argument("--build-dir", default=os.path.join(EXTRACT, "build"))
    args = ap.parse_args()

    if args.revert:
        return revert()

    manifest = json.load(open(os.path.join(EXTRACT, "manifest.json"), encoding="utf-8"))
    strings = json.load(open(os.path.join(EXTRACT, "strings.json"), encoding="utf-8"))
    source = manifest["source"]
    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)

    specs = {}  # classic tree rel -> list of spec lines

    def add(tree_rel, line):
        specs.setdefault(tree_rel, []).append(line)

    # 1. Property images, per equip: copy from the modern part that holds it.
    part_cache = {}
    for set_name, items in manifest["sets"].items():
        for it in items:
            tree_rel = "Character/" + it["type"]
            image = "%08d.img" % it["id"]
            src = os.path.join(source, it["prop_archive"])
            add(tree_rel, "copy\t%s\t%s\t%s" % (image, src, image))

    # 2. Canvas images: every outlink target, copied whole for Character trees; merged by
    #    key for the Item trees (an Item canvas image holds every item of that prefix).
    item_keys = {}  # (tree_rel, image) -> keys we own
    for it in manifest["cash"]:
        item_keys.setdefault(("Item/Cash", "%04d.img" % (it["id"] // 10000)), []).append("%08d" % it["id"])
    for it in manifest["consume"]:
        item_keys.setdefault(("Item/Consume", "%04d.img" % (it["id"] // 10000)), []).append("%08d" % it["id"])

    for target, paths in manifest["canvas_images"].items():
        tree_rel, _, image = target.rpartition("/_Canvas/")
        key = (tree_rel, image)
        if key not in part_cache:
            part_cache[key] = modern_part(source, tree_rel + "/_Canvas", image)
        src = part_cache[key]
        if tree_rel.startswith("Item/"):
            # **The keys come from the OUTLINKS, not from our item ids.** The three Frieren
            # set coupons share one icon: 5681544 and 5681545 outlink into
            # `0568.img/05681543/info/icon`, and the canvas image has no node of their own.
            # Keying on item ids asked the modern image for nodes it never had.
            top = sorted({p.split("/", 1)[0] for p in paths})
            add(tree_rel + "/_Canvas", "merge\t%s\t%s\t%s\t%s" % (image, src, image, ",".join(top)))
        else:
            add(tree_rel + "/_Canvas", "copy\t%s\t%s\t%s" % (image, src, image))

    # 3. Item property images: merge our nodes onto the classic image of the same name.
    for (tree_rel, image), keys in sorted(item_keys.items()):
        pkey = (tree_rel, image, "prop")
        if pkey not in part_cache:
            part_cache[pkey] = modern_part(source, tree_rel, image)
        add(tree_rel, "merge\t%s\t%s\t%s\t%s" % (image, part_cache[pkey], image, ",".join(keys)))

    # 4. Strings: one TSV per image, `path<TAB>value`.
    os.makedirs(args.build_dir, exist_ok=True)
    for image, root in [("Eqp.img", strings["ClassicWorld"]), ("Cash.img", strings["Cash"]), ("Consume.img", strings["Consume"])]:
        tsv = os.path.join(args.build_dir, "strings-" + image + ".tsv")
        if image == "Cash.img":
            # The modern text says "obtain 1 item according to set probability rates". Ours
            # gives every set (the owner, 2026-09-10), and the tooltip is the one place a player
            # reads the rule.
            root = dict(root)
            root["5222221"] = {
                "name": "Signature Style Collection",
                "desc": "A collection of every Signature Style outfit. #cDouble-click# to receive "
                        "all eight Outfit Set Coupons: Frieren, Fern, Stark, \u00dcbel, Himmel, "
                        "Aura, L\u00fcgner and Linie.",
            }
        with open(tsv, "w", encoding="utf-8", newline="\n") as fh:
            if image == "Eqp.img":
                for kind, ids in root.items():
                    for item_id, leaves in ids.items():
                        for leaf, value in leaves.items():
                            fh.write("ClassicWorld/%s/%s/%s\t%s\n" % (kind, item_id, leaf, value.replace("\n", "\\n")))
            else:
                for item_id, leaves in root.items():
                    for leaf, value in leaves.items():
                        fh.write("%s/%s\t%s\n" % (item_id, leaf, value.replace("\n", "\\n")))
        add("String", "strings\t%s\t%s" % (image, tsv))

    # 4b. The Cash Shop: sale rows for the box and the eight set coupons, in the SPECIAL tab,
    #     badged NEW. The owner, 2026-09-10: *"put the full package items for sale in the special
    #     tab of Cash Shop, which is currently blank. Label them with the 'NEW' icon."*
    #
    #     Read out of the classic client [L]: the Special tab is `CashShopCategory.img/2` and
    #     declares no sub-tab, which is why it draws "No results found"; Main lists its wares
    #     as an explicit `commoditySN` list, and that form is on screen. `Class 0` on a
    #     Commodity row is the NEW badge and `Class 2` is HOT - all 15 badged/unbadged rows of
    #     the Main tab in the owner's screenshot agree with the table, no exceptions. `Period 0` is
    #     permanent (49 rows). SN prefix 12 (category 2 = Special under the derived
    #     arithmetic) is unused by every shipped row, so the new serials are 120000000..
    #
    #     **Prices are the owner's, 2026-09-10**: *"each signature style set coupon should cost
    #     2000 LP"* and the box *"give[s] them all of the sets for 8000 LP"* - so the box is
    #     the eight coupons at half price. The modern client sells the box for 7,900 NX as a
    #     one-at-random gacha and never sells the coupons; both rules are ours.
    #
    #     Seen on screen 2026-09-10 at the first prices (7,900 / 3,900): the tab, the nine
    #     entries, the NEW badges, the icons and the tooltips all drew. The shape is proven;
    #     only the numbers changed after.
    rows = json.load(open(os.path.join(EXTRACT, "manifest.json"), encoding="utf-8"))
    by_name = {it["name"]: it["id"] for it in manifest["cash"]}
    wares = [("Signature Style Collection", 8000)] + [
        (n + " Outfit Set Coupon", 2000)
        for n in ["Frieren", "Fern", "Stark", "Übel", "Himmel", "Aura", "Lügner", "Linie"]
    ]
    commodity_patch = os.path.join(args.build_dir, "patch-Commodity.img.tsv")
    category_patch = os.path.join(args.build_dir, "patch-CashShopCategory.img.tsv")
    classic_rows = 159  # Commodity.img rows 0..158 in the classic client; ours append after
    with open(commodity_patch, "w", encoding="utf-8", newline="\n") as fh, \
            open(category_patch, "w", encoding="utf-8", newline="\n") as ch:
        ch.write("2/0/name\tstr\tSignature Style\n")
        for i, (name, price) in enumerate(wares):
            item_id = by_name[name]
            sn = 120_000_000 + i
            row = classic_rows + i
            for field, value in [
                ("SN", sn), ("ItemId", item_id), ("Count", 1), ("Price", price), ("Bonus", 0),
                ("Period", 0), ("Priority", 100), ("ReqPOP", 0), ("ReqLEV", 0), ("Gender", 2),
                ("OnSale", 1), ("Class", 0), ("originalPrice", price), ("PbCash", 0),
                ("PbPoint", 0), ("PbGift", 0), ("Refundable", 0), ("WebShop", 0), ("IsGift", 0),
            ]:
                fh.write("%d/%s\tint\t%d\n" % (row, field, value))
            ch.write("2/0/commoditySN/%d\tint\t%d\n" % (i, sn))
    add("Etc", "patch\tCommodity.img\t%s" % commodity_patch)
    add("Etc", "patch\tCashShopCategory.img\t%s" % category_patch)
    del rows

    # 5. Build every archive against its classic base.
    built = []
    for tree_rel, lines in sorted(specs.items()):
        name = tree_rel.rsplit("/", 1)[-1]
        base = os.path.join(CLASSIC, *tree_rel.split("/"), name + "_000.wz")
        if not os.path.exists(base):
            raise SystemExit("classic base missing: %s" % base)
        out = os.path.join(args.build_dir, "Data", *tree_rel.split("/"), name + "_000.wz")
        spec = os.path.join(args.build_dir, "spec-" + tree_rel.replace("/", "-") + ".tsv")
        with open(spec, "w", encoding="utf-8", newline="\n") as fh:
            fh.write("# built by tools/backport_install.py; base %s\n" % base)
            fh.write("\n".join(lines) + "\n")
        print("== %s: %d instruction(s), base %s" % (tree_rel, len(lines), os.path.relpath(base, REPO)))
        p = subprocess.run([WZ_DUMP, "build", out, CLASSIC_VERSION, spec, base], capture_output=True,
                           text=True, encoding="utf-8", errors="replace")
        sys.stdout.write("".join("   " + l + "\n" for l in p.stdout.splitlines()[-3:]))
        if p.returncode != 0:
            sys.stdout.write(p.stdout)
            raise SystemExit("wz-dump build failed for %s:\n%s" % (tree_rel, p.stderr))
        built.append((tree_rel, base, out))

    # 6. Prove the builds parse, image by image, with the ordinary reader.
    print("verifying every image of every built archive parses...")
    for tree_rel, base, out in built:
        p = subprocess.run([WZ_DUMP, "verify", os.path.dirname(out)], capture_output=True, text=True,
                           encoding="utf-8", errors="replace")
        tail = p.stdout.strip().splitlines()[-1:] or [p.stderr.strip()]
        print("   %-32s %s" % (tree_rel, tail[0]))
        # The summary is "<ok> ok, <failed> failed (...)": read the number before "failed",
        # not a field position - CLAUDE.md's awk lesson.
        words = tail[0].split()
        failed = int(words[words.index("failed")-1].rstrip(",")) if "failed" in words else -1
        if p.returncode != 0 or failed != 0 or "parsed" not in p.stdout:
            sys.stdout.write(p.stdout)
            raise SystemExit("verify failed for %s" % out)

    if not args.install:
        print("built %d archive(s) under %s - not installed (pass --install)" % (len(built), args.build_dir))
        return 0

    for tree_rel, base, out in built:
        bak = base + ".bak"
        if not os.path.exists(bak):
            shutil.copy2(base, bak)
        try:
            shutil.copy2(out, base)
        except PermissionError:
            raise SystemExit(
                "%s is held open - the client is running. Close MapleStory.exe and re-run "
                "--install; nothing was left half-done (archives are replaced one file at a time "
                "and each is complete or untouched)." % os.path.relpath(base, REPO))
        print("installed %s (%d -> %d bytes; original kept as %s)" % (
            os.path.relpath(base, REPO), os.path.getsize(bak), os.path.getsize(base), os.path.basename(bak)))

    # **The server's tables must say what the client's data says.** Prices live in the
    # client's Commodity.img AND in gm-handbook/commodity.txt, which the world server debits
    # from; names in String.wz AND items.txt. Regenerating here, after the copy, is what keeps
    # "the tab says 2,000" and "the server charged 3,900" from ever both being true.
    for tool in ["dump_names.py", "dump_equips.py", "dump_itemdata.py", "dump_commodity.py", "gen_item_rules.py"]:
        p = subprocess.run([sys.executable, os.path.join(REPO, "tools", tool)], capture_output=True,
                           text=True, encoding="utf-8", errors="replace")
        tail = (p.stdout.strip().splitlines() or [""])[-1]
        print("   %-20s %s" % (tool, tail if p.returncode == 0 else "FAILED: " + p.stderr.strip()[-200:]))
        if p.returncode != 0:
            raise SystemExit("%s failed after install - the server tables are stale" % tool)
    return 0


def revert():
    n = 0
    for dp, _, fn in os.walk(CLASSIC):
        for f in fn:
            if f.endswith("_000.wz.bak"):
                bak = os.path.join(dp, f)
                shutil.copy2(bak, bak[:-4])
                print("restored", os.path.relpath(bak[:-4], REPO))
                n += 1
    print("%d archive(s) restored" % n)
    return 0


if __name__ == "__main__":
    sys.exit(main())
