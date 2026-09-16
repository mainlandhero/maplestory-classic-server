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
| `Item/Pet/Pet_000.wz` | `patch`: every pet gets `info/permanent` -> 1 and its start skills; **`info/life` is left alone** - zeroing it (the modern permanent pet's shape) is what made a summoned pet invisible, see the note at the patch rows; the eight pets the classic shop never listed get Commodity rows under the Pets tab |
| `Effect/Effect_000.wz`, `Effect/_Canvas/_Canvas_000.wz` | `merge`: the classic client has no `ItemEff.img` at all, so a NEW one is made from the set items' worn-effect nodes (Himmel's Blessing, 1103918) and a new canvas `ItemEff.img` from the holders their outlinks name (1103930) |

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
# The Signature Style Collection box: Nexon's id, and the id it wears in the classic client
# (family 568, the one the client opens on double-click - see step 3). Must equal
# `world::signaturestyle::COLLECTION`; a test there reads gm-handbook/commodity.txt, which
# this script regenerates after an install, so a mismatch fails the suite rather than a run.
BOX_MODERN_ID = 5222221
BOX_ID = 5681599
# The eight face coupons, 2026-09-12: "The face coupons from the backported collaboration
# items still does not work." The classic client opens its Beauty Coupon dialog
# (FUN_142dc8100, the 0x0165 CONFIRM) only for ids in seven ranges read off the opener at
# 0x141785d90: 2540000..2549999 (hair coupons - ours work), 2890000..2890999 (the modern
# client's own "Face Coupon" family), 2889000, 2893000 (skins), 2894000/2895000 (android
# faces), 2900168 (a thousand more). 2897xxx is in none of them, so a double-click on a face
# coupon opened nothing and sent nothing - world.log has no packet for it in any run. [L]
# So they wear 2890907..2890914: same 0289.img, same canvas nodes, a free stretch of the
# family (the modern client uses 2890000..2890054).
FACE_COUPON_RENAMES = {2897000 + n: 2890900 + n for n in range(7, 15)}
# Every id that changes family or number on the way into the classic client, old -> new.
RENAMES = {BOX_MODERN_ID: BOX_ID, **FACE_COUPON_RENAMES}


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
    ap.add_argument("--check", action="store_true",
                    help="release preflight: rebuild into a scratch dir and prove the INSTALLED archives are "
                         "byte-identical to it, and that gm-handbook was regenerated after the install")
    args = ap.parse_args()

    if args.revert:
        return revert()
    if args.check:
        # The client payload (tools/make-installer.ps1) ships whatever sits in client-patched\Data.
        # Nothing else proves that what sits there is THIS script's current output rather than an
        # earlier install - and an earlier install is exactly what a stale box node, a z 10 cape
        # or a 2897 face coupon would be. So: build again, into a scratch directory, and require
        # the installed archive to hash equal to the fresh build for every tree this script
        # touches. The owner, 2026-09-12: "Make sure everything we worked on is release-able to the
        # server and client packages."
        import hashlib
        import tempfile
        args.build_dir = tempfile.mkdtemp(prefix="maplecw-backport-check-")
        args.install = False
        args._check = True

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

    # 1b. Weapon covers: one link child per classic weapon TYPE.
    #
    # The owner, 2026-09-12: "Ubel's weapon still cannot be equipped over all weapons." The client
    # decides whether a cash weapon cover may go on by looking for a child of the cover's
    # image named after the equipped weapon's type: the classic cover 01702001.img carries a
    # real `30` subtree and `31`/`32`/`33` as UOL links to it. Every modern cover here carries
    # only `30` and `49` (49 = gun, a type this client does not have), so over Cobalt's
    # suitcase - type 32 - the client found nothing and refused, exactly as the tooltip's
    # "all weapons" promised it would not. [L on the data; the check itself is I, and the
    # classic file is the control.] So each cover gets a UOL to `30` for every weapon type
    # the classic Weapon archive actually contains. The two-handed types (40..47) will be
    # equippable and may not draw in two-handed stances, which the link cannot supply; covers
    # do not draw at all yet (STATUS: the record carries worn slots 1..31 only), so that is
    # not a regression of anything.
    weapon_base = os.path.join(CLASSIC, "Character", "Weapon", "Weapon_000.wz")
    if os.path.exists(weapon_base + ".bak"):
        weapon_base += ".bak"  # the untouched classic, when an earlier install left it beside
    tree = subprocess.run([WZ_DUMP, "tree", weapon_base, "1"], capture_output=True, text=True,
                          encoding="utf-8", errors="replace").stdout
    classic_types = sorted({int(l.split("[IMG]", 1)[1].split()[0][2:4]) for l in tree.splitlines()
                            if "[IMG]" in l and l.split("[IMG]", 1)[1].split()[0][:2] == "01"
                            and l.split("[IMG]", 1)[1].split()[0][2:4] not in ("70",)})
    classic_types = [t for t in classic_types if 30 <= t <= 49]
    if not classic_types:
        raise SystemExit("no weapon types found in %s - the tree listing is broken" % weapon_base)
    os.makedirs(args.build_dir, exist_ok=True)
    for set_name, items in manifest["sets"].items():
        for it in items:
            if it["type"] != "Weapon" or not (1700000 <= it["id"] < 1800000):
                continue
            image = "%08d.img" % it["id"]
            src = os.path.join(source, it["prop_archive"])
            have = json.loads(subprocess.run([WZ_DUMP, "cat", src, image], capture_output=True,
                                             text=True, encoding="utf-8", errors="replace").stdout)
            present = sorted(int(k) for k in have if k.isdigit())
            anchor = next((t for t in present if t in classic_types), None)
            if anchor is None:
                raise SystemExit("%s has no type child this client knows (%s)" % (image, present))
            tsv = os.path.join(args.build_dir, "cover-types-%08d.tsv" % it["id"])
            with open(tsv, "w", encoding="utf-8", newline="\n") as fh:
                fh.write("# %s: every classic weapon type as a link to %d, the classic covers' own pattern\n" % (image, anchor))
                for t in classic_types:
                    if t not in present:
                        fh.write("%d\tuol\t%d\n" % (t, anchor))
            add("Character/Weapon", "patch\t%s\t%s" % (image, tsv))

    # 1d. The hair-hats' slot type. The owner, 2026-09-12: "The Aura, Lugner and Linie hair does
    # not wear when double clicked on." world.log has NO 0x0107 for 1006910/11/12 across
    # every run they were in a bag - the client never sent the move, so it refused locally.
    # Their `info/islot` is `HrCp`, the modern two-slot type (takes the hair slot and the
    # cap slot); every classic cap says `Cp`. The client reads islot in two-letter tokens,
    # and the first token here is `Hr` - hair, which is not an equip a bag can put on - so
    # the double-click had no destination. [L on the data and the absent packet; the token
    # reading is I: `MaPn` also has no whole-string match in the image and the overalls
    # equip fine.] So islot becomes `Cp` for every cap whose type starts with `Hr`. vslot
    # (which hair parts the hat hides) is left as Nexon wrote it - one variant at a time.
    for set_name, items in manifest["sets"].items():
        for it in items:
            if it["type"] != "Cap":
                continue
            prop = json.load(open(os.path.join(EXTRACT, "wz", "Cap", "%08d" % it["id"], "prop.json"), encoding="utf-8"))
            islot = prop.get("info", {}).get("islot", "")
            if not islot.startswith("Hr"):
                continue
            tsv = os.path.join(args.build_dir, "islot-%08d.tsv" % it["id"])
            with open(tsv, "w", encoding="utf-8", newline="\n") as fh:
                fh.write("# %s: islot %s -> Cp, the classic cap type; the client took Hr for hair\n" % (it["id"], islot))
                fh.write("info/islot\tstr\tCp\n")
            add("Character/Cap", "patch\t%08d.img\t%s" % (it["id"], tsv))
            print("  islot    %-8s %8d  %s -> Cp" % (set_name, it["id"], islot))

    # 1c. Worn-item effects. The owner, 2026-09-12: "Himmel's cape should actually have an effect,
    # but this effect currently does not appear in our version of the game."
    #
    # The cape's own image (Cape/01103918.img) is 1x1 frames - the garment IS its effect,
    # and that lives in `Effect/ItemEff.img/<id>/effect`, a different archive. The classic
    # client has the loader (`Effect/ItemEff.img/%d/%s` + `effect`, read through pointer
    # slots by the avatar code and even the character-select slot filler - `tools/dataref.py
    # 0x143a47080`), and its Effect_000.wz has NO ItemEff.img at all: 26 images, none of
    # them that. Pure missing data, so: for every set item the modern ItemEff.img has a node
    # for, merge that node onto a NEW classic ItemEff.img, and merge the canvas holders its
    # outlinks name (`Effect/_Canvas/ItemEff.img/<holder>/...`; the Himmel frames sit under
    # holder 1103930) onto a new `_Canvas/ItemEff.img`. The modern canvas image is 244 MB;
    # the merge takes the one holder's subtree and nothing else (46 KB on disk).
    eff_src = modern_part(source, "Effect", "ItemEff.img")
    eff_tree = json.loads(subprocess.run([WZ_DUMP, "cat", eff_src, "ItemEff.img"], capture_output=True,
                                         text=True, encoding="utf-8", errors="replace").stdout)
    eff_keys, eff_holders = [], set()
    for set_name, items in manifest["sets"].items():
        for it in items:
            node = eff_tree.get("%d" % it["id"])
            if node is None:
                continue
            eff_keys.append("%d" % it["id"])
            def holders(n):
                if isinstance(n, dict):
                    ol = n.get("_outlink")
                    if isinstance(ol, str) and ol.startswith("Effect/_Canvas/ItemEff.img/"):
                        eff_holders.add(ol.split("/")[3])
                    for v in n.values():
                        holders(v)
            holders(node)
            print("  effect   %-8s %8d  %s" % (set_name, it["id"], it.get("name", "")))
    if eff_keys:
        add("Effect", "merge\tItemEff.img\t%s\tItemEff.img\t%s" % (eff_src, ",".join(eff_keys)))
        # The effect's depth. The owner, 2026-09-12, with a screenshot of Cobalt standing inside a
        # grey block: "Himmel's cape should have an offset and appear behind the player's
        # character, currently it blocks the character when idle." Nexon's node says `z 10`
        # on `effect` and on `effect/stand1`, and this client drew that IN FRONT of the
        # body. The same modern image gives its plain aura entries `z -2` (1103988 is the
        # first key), which is the value a behind-the-body effect carries; so every node
        # we ship gets its `z` leaves rewritten to -2. [I: the sign convention is read off
        # the sibling entries, not the client; the screen is the test.] The frame origin
        # (38,141 on an 81x143 canvas, i.e. centred on the body) is left alone - "offset"
        # in the owner's sentence is what a behind-the-body draw looks like from the front.
        eff_z_tsv = os.path.join(args.build_dir, "itemeff-z.tsv")
        with open(eff_z_tsv, "w", encoding="utf-8", newline="\n") as fh:
            fh.write("# every z leaf under <id>/effect rewritten to -2: behind the body\n")
            for key in eff_keys:
                node = eff_tree[key]["effect"]
                if "z" in node:
                    fh.write("%s/effect/z\tint\t-2\n" % key)
                for action, sub in node.items():
                    if isinstance(sub, dict) and "z" in sub:
                        fh.write("%s/effect/%s/z\tint\t-2\n" % (key, action))
        add("Effect", "patch\tItemEff.img\t%s" % eff_z_tsv)
        if eff_holders:
            eff_canvas_src = modern_part(source, "Effect/_Canvas", "ItemEff.img")
            add("Effect/_Canvas", "merge\tItemEff.img\t%s\tItemEff.img\t%s" % (
                eff_canvas_src, ",".join(sorted(eff_holders))))

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
    #
    # **Except the box, which changes id family.** The owner, 2026-09-12: *"Double clicking the
    # Signature Style Collection box does not grant all 8 character costume coupons."* The
    # world log shows NO packet at all for the double-click - the client did not treat it as
    # a use. The classic client opens a Cash item on double-click by its id family, and the
    # modern box is 5222221 (family 522, which this client has no items of); the set coupons
    # are 5681xxx (family 568 - its native 5-slot coupons) and their double-click sends
    # 0x0114 every time, measured on screen this session. The two nodes are otherwise the
    # same shape (`info`: cash, collabo, icons). [The family reading is I; the pair of
    # controls is L.] So the box's property node is merged into 0568.img under BOX_ID; its
    # icon outlinks still name the 0522 canvas, which is copied as before, and the string,
    # the Cash Shop row and the server (world::signaturestyle::COLLECTION) all say BOX_ID.
    for (tree_rel, image), keys in sorted(item_keys.items()):
        pkey = (tree_rel, image, "prop")
        if pkey not in part_cache:
            part_cache[pkey] = modern_part(source, tree_rel, image)
        # Keys that keep their id merge onto the classic image of the same name; a renamed
        # one merges onto the image its NEW id belongs to (the box moves 0522 -> 0568; a face
        # coupon stays in 0289), under `old=new`. The icon outlinks inside the node still name
        # the old id's canvas node, which step 2 merges untouched.
        plain = [k for k in keys if int(k) not in RENAMES]
        if plain:
            add(tree_rel, "merge	%s	%s	%s	%s" % (image, part_cache[pkey], image, ",".join(plain)))
        homes = {}
        for k in keys:
            if int(k) in RENAMES:
                new_id = RENAMES[int(k)]
                homes.setdefault("%04d.img" % (new_id // 10000), []).append("%s=%08d" % (k, new_id))
        for home, pairs in sorted(homes.items()):
            add(tree_rel, "merge	%s	%s	%s	%s" % (home, part_cache[pkey], image, ",".join(pairs)))

    # 4. Strings: one TSV per image, `path<TAB>value`.
    os.makedirs(args.build_dir, exist_ok=True)
    # `Npc.img`: the package receipt (world::signaturestyle::ADMINISTRATOR_NPC) speaks as
    # NPC 9010000, which the classic String.wz names "Maple Administrator". The owner asked for the
    # dialogue to come from "MapleStory Administrator" (2026-09-12), and the name the dialog
    # shows is this string, so it is renamed here - everywhere the NPC appears, which is the
    # one place a name can live. Reversible with --revert like everything else in this file.
    npc_strings = {"9010000": {"name": "MapleStory Administrator"}}
    for image, root in [("Eqp.img", strings["ClassicWorld"]), ("Cash.img", strings["Cash"]),
                        ("Consume.img", strings["Consume"]), ("Npc.img", npc_strings)]:
        tsv = os.path.join(args.build_dir, "strings-" + image + ".tsv")
        if image in ("Cash.img", "Consume.img"):
            # Renamed ids (RENAMES) carry their strings to the new id.
            root = {("%d" % RENAMES[int(k)]) if k.isdigit() and int(k) in RENAMES else k: v
                    for k, v in root.items()}
        if image == "Cash.img":
            # The modern text says "obtain 1 item according to set probability rates". Ours
            # gives every set (the owner, 2026-09-10), and the tooltip is the one place a player
            # reads the rule.
            root["%d" % BOX_ID] = {
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
            item_id = BOX_ID if name == "Signature Style Collection" else by_name[name]
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
    # 4c. The pets. The owner, 2026-09-13: "Current, Brown Puppy, Panda and Dino Boy all have 3 day
    #     duration. Please edit the WZ if needed to change all of them to permanent duration.
    #     Also please add all of the other pets into the Cash Shop too ... They should also be
    #     permanent duration. They should never need to be revived."
    #
    #     The duration the shop shows is the pet's own `Item/Pet/<id>.img/info/life`, in DAYS
    #     (3 for Brown Puppy, Panda and Dino Boy; 7 and 90 for the rest) - the three Commodity
    #     rows already say Period 0. The modern client's one permanent pet (5000060) carries
    #     `life 0` and `permanent 1` [L], so every classic pet gets both. The eight pets with
    #     no row get one in the Pets tab: SN 1600000NN puts a row under category 6 / scope 600
    #     (tools/dump_commodity.py's arithmetic), Period 0, the shipped rows' price. Whether the
    #     classic client reads `permanent` is I; `life 0` is what the permanent pet ships with.
    #     The lifespan the SERVER sends is the pet body's dateDead, which is never
    #     (net::bag::pet_item_with_cash_sn) - that is the "never revived" half.
    pets = [(5000000, "Brown Kitty"), (5000001, "Brown Puppy"), (5000002, "Pink Bunny"),
            (5000003, "Mini Kargo"), (5000004, "Black Kitty"), (5000005, "White Bunny"),
            (5000006, "Husky"), (5000007, "Black Pig"), (5000008, "Panda"),
            (5000009, "Dino Boy"), (5000010, "Dino Girl")]
    for pet_id, pet_name in pets:
        tsv = os.path.join(args.build_dir, "pet-%07d.tsv" % pet_id)
        with open(tsv, "w", encoding="utf-8", newline="\n") as fh:
            # **`info/life` IS NO LONGER ZEROED, and that is the pet-invisibility fix.**
            #
            # It used to be set to 0 beside `permanent 1`, because the MODERN client's one
            # permanent pet (5000060) carries that pair. `CLAUDE.md` rates the modern source
            # 1-of-8 against a held-out control and says to label every claim from it a
            # candidate; this one was applied to the classic client anyway, and the comment
            # above it admitted that whether this client reads `permanent` was [I].
            #
            # On 2026-09-14 a full diff of the Husky image against `Pet_000.wz.bak` showed
            # `life: 7 -> 0` and the three added keys were the ONLY differences in 1400 lines -
            # every animation node, canvas and `_outlink` was byte-identical. A summoned pet
            # drew its name tag, reported 34 movements and never appeared, and the client's own
            # show/hide ladder (FUN_141ecde00) was measured deciding SHOW: all eleven gates pass
            # and both session flags gate 11 reads are untouched. So the client wanted to draw a
            # pet whose declared lifespan was zero days.
            #
            # Permanence does not need this key. It is on the wire already, in the pet body
            # `net::bag` builds: `dateDead = ITEM_NEVER_EXPIRES` and `remainLife = 0`, which is
            # what puts "This miraculous pet will never expire!" in the tooltip the owner
            # screenshotted. `permanent 1` is kept - it is additive and harmless - and the
            # lifespan is left exactly as Nexon shipped it.
            fh.write("# %s: permanent via the pet BODY (dateDead/remainLife), not by zeroing\n" % pet_name)
            fh.write("# info/life. See tools/backport_install.py - a life of 0 days is why a\n")
            fh.write("# summoned pet had a name tag, walked, and drew nothing.\n")
            fh.write("info/permanent\tint\t1\n")
            # **A pet declares the skills it starts with, and every pet starts as a vacuum
            # pet.** The owner, 2026-09-16: "Can we turn all pets into vacuum pets, so they loot
            # from long range similar to current Luna Petite pets in modern MapleStory?" -
            # which is what 370 of the modern archive's 1561 pets are in data: pickupItem 1,
            # sweepForDrop 1, longRange 1 (research/pet-vacuum-2026-09-13.md).
            #
            # The tooltip prints a line per skill the pet IMAGE declares, and says
            # "(Learned)" or "This is an unregistered pet." by ANDing the item body's
            # petSkill mask (FUN_14266f2d0). Declaration and mask move together:
            #
            #   pickupItem    Item Pouch          declared here, and in the mask
            #   sweepForDrop  Auto Move           declared here, and in the mask
            #   longRange     Expanded Auto Move  declared here, and in the mask
            #                                     (net::bag::PET_SKILLS_LEARNED_AT_START)
            #   consumeHP/MP  Auto HP/MP Pouch    items 5190000/1, 100 LP - NOT declared
            #
            # 2026-09-13 had zeroed the two auto-move keys so the shop's Auto Move items
            # (5190002/3) would be the way to learn them; with every pet a vacuum pet those
            # two items add nothing and are still sold - noted, not removed.
            #
            # Meso Magnet needs no key at all: it is absent from the client's own skill
            # table (FUN_141ed1ad0) and shows with every key cleared.
            fh.write("info/pickupItem\tint\t1\n")
            fh.write("info/sweepForDrop\tint\t1\n")
            fh.write("info/longRange\tint\t1\n")
        add("Item/Pet", "patch\t%07d.img\t%s" % (pet_id, tsv))
    shipped_pet_rows = {5000001, 5000008, 5000009}  # SN 160000000..2 in the classic Commodity.img
    pet_rows_patch = os.path.join(args.build_dir, "patch-Commodity-pets.tsv")
    with open(pet_rows_patch, "w", encoding="utf-8", newline="\n") as fh:
        n = 0
        for pet_id, pet_name in pets:
            if pet_id in shipped_pet_rows:
                continue
            sn = 160_000_003 + n
            row = classic_rows + len(wares) + n
            n += 1
            fh.write("# %s\n" % pet_name)
            for field, value in [
                ("SN", sn), ("ItemId", pet_id), ("Count", 1), ("Price", 100), ("Bonus", 0),
                ("Period", 0), ("Priority", 100), ("ReqPOP", 0), ("ReqLEV", 0), ("Gender", 2),
                ("OnSale", 1), ("originalPrice", 100), ("PbCash", 0), ("PbPoint", 0),
                ("PbGift", 0), ("Refundable", 0), ("WebShop", 0), ("IsGift", 0),
            ]:
                fh.write("%d/%s\tint\t%d\n" % (row, field, value))
    add("Etc", "patch\tCommodity.img\t%s" % commodity_patch)
    add("Etc", "patch\tCommodity.img\t%s" % pet_rows_patch)
    add("Etc", "patch\tCashShopCategory.img\t%s" % category_patch)
    del rows

    # 5. Build every archive against its classic base.
    built = []
    for tree_rel, lines in sorted(specs.items()):
        name = tree_rel.rsplit("/", 1)[-1]
        # `target` is the archive the client reads and the one --install replaces; `base` is
        # what the build starts from.
        target = os.path.join(CLASSIC, *tree_rel.split("/"), name + "_000.wz")
        # **Build against the pristine classic archive, not the last install.** `--install`
        # keeps the untouched original as `.bak` beside each archive; building on the hybrid
        # instead carried every earlier install's nodes forward verbatim, so a node this spec
        # no longer emits (the box under its old id, 2026-09-12) survived as a stale twin and
        # the store's item-count drift test caught it (2864 where 2863 was right). With the
        # `.bak` as base a rebuild is a function of the original and this script alone.
        #
        # The two paths are deliberately separate variables: the first version of this
        # reassigned one name, and --install then wrote the build OVER the .bak and moved the
        # pristine original to .bak.bak while the client's archive went untouched.
        base = target + ".bak" if os.path.exists(target + ".bak") else target
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
        built.append((tree_rel, target, out))

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

    if getattr(args, "_check", False):
        import hashlib

        def sha(path):
            h = hashlib.sha256()
            with open(path, "rb") as fh:
                for chunk in iter(lambda: fh.read(1 << 20), b""):
                    h.update(chunk)
            return h.hexdigest()

        bad = []
        print("release check: installed archive vs a fresh build of the same spec")
        for tree_rel, target, out in built:
            if not os.path.exists(target + ".bak"):
                bad.append("%s: never installed (no .bak beside it)" % tree_rel)
                print("   %-32s NEVER INSTALLED" % tree_rel)
                continue
            same = sha(target) == sha(out)
            print("   %-32s %s" % (tree_rel, "matches the fresh build" if same else "DIFFERS from the fresh build"))
            if not same:
                bad.append("%s: installed archive differs from the current build" % tree_rel)
        # The world server debits and names from gm-handbook, regenerated by --install after the
        # copy. A handbook older than the installed String archive was made from other data.
        strings = os.path.join(CLASSIC, "String", "String_000.wz")
        for table in ["items.txt", "equips.txt", "commodity.txt", "itemdata.txt"]:
            path = os.path.join(REPO, "gm-handbook", table)
            if not os.path.exists(path):
                bad.append("gm-handbook/%s is missing" % table)
            elif os.path.getmtime(path) < os.path.getmtime(strings):
                bad.append("gm-handbook/%s is older than the installed String archive - regenerate" % table)
        shutil.rmtree(args.build_dir, ignore_errors=True)
        if bad:
            print("RELEASE CHECK FAILED:")
            for b in bad:
                print("   " + b)
            print("run: python tools/backport_install.py --install   (with the client closed)")
            return 1
        print("release check passed: %d archive(s) installed and current, handbook regenerated after them" % len(built))
        return 0

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
    for tool in ["dump_names.py", "dump_equips.py", "dump_itemdata.py", "dump_commodity.py",
                 "dump_pets.py", "gen_item_rules.py"]:
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
