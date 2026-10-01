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
| `Item/Pet/Pet_000.wz` | `patch`: every pet gets `info/life` -> 0 (unlimited: no days line in the shop, alive unconditionally - see the note at the patch rows; the 2026-09-14 "life 0 made the pet invisible" was a coincidence, f0c3010 found giantRate), `info/permanent` -> 1 and its start skills; the eight pets the classic shop never listed get Commodity rows under the Pets tab. **The four collaboration pets** (2026-09-17, Lil Frieren / Fern / Stark / Übel) are `copy`ed in whole - property image here, pixels into `Item/Pet/_Canvas` - then patched like the classic eleven, plus `del` of the leaves that name UI nodes this client lacks (`chatBalloon`, `nameTag`, `setItemID`) |
| `Character/PetEquip/PetEquip_000.wz` | `inline`: the four collaboration pet weapons. The classic client keeps pet-equip pixels INSIDE the property image (its ten hats have no `_Canvas` tree at all), so a plain `copy` of the modern image would leave 1x1 stubs pointing at a tree that is not there; `inline` follows every outlink into the modern `_Canvas` archive and writes the pixels in place |
| `String/String_000.wz` | ...and `Pet.img` (name, desc, descD), `PetCommand.img` (the words) and `PetDialog.img` (the lines) for the four pets - which is where `tools/dump_pets.py` reads a pet's commands from |
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
import importlib.util
import zlib
import pathlib
import json
import os
import shutil
import subprocess
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
WZ_DUMP = os.path.join(REPO, "target", "release", "wz-dump" + (".exe" if os.name == "nt" else ""))
EXTRACT = os.path.join(REPO, "backport", "signature-style")

# The Cash Shop prices of the Signature Style wares, in Leaf Points. The owner's, 2026-09-17:
# 200 LP per Outfit Set Coupon, 800 LP for the Collection box that holds all eight. These
# are written into the client's Commodity.img (step 4b) and, through --install's handbook
# regeneration, into gm-handbook/commodity.txt, which is what the server debits.
SET_COUPON_PRICE_LP = 200
COLLECTION_PRICE_LP = 800

# The pets and pet equipment, in Leaf Points. The owner, 2026-09-17: *"All pets from these
# collaboration should be 1000 LP. Pet equipment should remain 100 LP each."* The classic
# eleven pets stay at the shipped rows' 100.
COLLAB_PET_PRICE_LP = 1000  # an exception to the shop-wide 100 LP rule - the owner, 2026-09-24:
#                             "collab pets should remain at 1000 LP" (cash_wares.price_rule)
PET_EQUIP_PRICE_LP = 100

# The classic client's ten pet hats, `Character/PetEquip/PetEquip_000.wz` [L], with the
# names `String/Eqp.img/ClassicWorld/PetEquip` gives them. The shipped Commodity.img sells
# three of them (1802002, 1802005, 1802006 - SN 160100000..2); the owner, 2026-09-17: *"Currently
# I only see 3 pet equipment, search the classic server WZ data and make sure all pet
# equipment is available."* The other seven get rows here.
CLASSIC_PET_EQUIPS = [
    (1802000, "Red Ribbon"), (1802001, "Yellow Hat"), (1802002, "Red Hat"), (1802003, "Black Hat"),
    (1802004, "Pink Laced Cap"), (1802005, "Sky Blue Laced Cap"), (1802006, "Blue Top Hat"),
    (1802007, "Red Top Hat"), (1802008, "Rudolph's Hat"), (1802009, "Tree Hat"),
]
SHIPPED_PET_EQUIP_ROWS = {1802002, 1802005, 1802006}

# The Petite pet badge. The owner, 2026-09-17: *"Special petite luna pets should have an icon on
# the pet. Is there a way we can achieve the same thing in both the Cash Shop item and the
# inventory icon to label them with this P (stands for Petite Pet)?"* - and, on a first draft
# that drew its own disc: *"Please use assets from the modern client if possible."*
#
# The modern client PAINTS the label over a Petite pet's icon at draw time - the extracted
# `info/icon` of Lil Frieren is the bare portrait. The badges are the client's own
# `UI/CashShop.img/CashItem_label/<n>` canvases (12x12, one per pet label: 7 = Sweet,
# 8 = Dream, **9 = Petite**, read off a contact sheet of all nineteen, 2026-09-18) [L];
# the classic client has neither the node nor the draw code, so the badge is composited
# into the icon PIXELS: `info/icon`, `iconRaw`, `iconD`, `iconRawD` of each collab pet's
# `Item/Pet/_Canvas/<id>.img` are decoded (tools/wz_png.py), the label alpha-blended at the
# bottom-right the way the modern shop draws it, and written back as BGRA8888 (format 2,
# which the classic client's own data already uses) through `wz-dump build`'s `canvas`
# patch kind. The Cash Shop and the bag both draw `info/icon`, so one edit reaches both.
PETITE_LABEL_NODE = 9
LABEL_INSET = 1  # the label's bottom-right corner sits this far inside the icon's


def _wz_png():
    spec = importlib.util.spec_from_file_location("wz_png", os.path.join(REPO, "tools", "wz_png.py"))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def petite_label(source, build_dir):
    """Nexon's Petite badge as `(rgba, w, h)`, exported from the modern UI canvas archive."""
    wz_png = _wz_png()
    part = modern_part(source, "UI/_Canvas", "CashShop.img")
    out = os.path.join(build_dir, "cash-item-labels")
    os.makedirs(out, exist_ok=True)
    r = subprocess.run([WZ_DUMP, "canvas", part, "CashShop.img", out, "CashItem_label"],
                       capture_output=True, text=True, encoding="utf-8", errors="replace")
    if r.returncode != 0:
        raise SystemExit("wz-dump canvas CashShop.img failed: %s" % r.stderr.strip())
    entries = json.load(open(os.path.join(out, "manifest.json"), encoding="utf-8"))
    want = "/CashItem_label/%d" % PETITE_LABEL_NODE
    meta = next((e for e in entries if e["node"] == want), None)
    if meta is None:
        raise SystemExit("the modern UI has no %s - the Petite badge moved" % want)
    w, h, fmt = int(meta["width"]), int(meta["height"]), int(meta["format"])
    payload = open(os.path.join(out, "CashItem_label.%d.bin" % PETITE_LABEL_NODE), "rb").read()
    return wz_png.to_rgba(wz_png.inflate(payload), w, h, fmt), w, h


def composite(rgba, width, height, label, lw, lh, x0, y0):
    """`label` alpha-blended onto `rgba` with its top-left at (x0, y0). Returns new bytes."""
    px = bytearray(rgba)
    for y in range(lh):
        for x in range(lw):
            tx, ty = x0 + x, y0 + y
            if not (0 <= tx < width and 0 <= ty < height):
                continue
            s = label[(y * lw + x) * 4:(y * lw + x) * 4 + 4]
            a = s[3]
            if a == 0:
                continue
            i = (ty * width + tx) * 4
            d = px[i:i + 4]
            da = d[3]
            # Source-over in exact integer arithmetic: out_alpha * 255 = a*255 + da*(255-a),
            # with no intermediate floor. The first version floored `da*(255-a)//255` in the
            # alpha and `d*da*(255-a)//255` in the colour separately, and for a = da = 1 the
            # two roundings disagreed enough to put 509 in a byte (2026-09-18, the first
            # hair icon with a soft edge; the badge never hit it - its alpha is 0 or 255).
            oa255 = a * 255 + da * (255 - a)
            if oa255 == 0:
                continue
            for c in range(3):
                px[i + c] = (s[c] * a * 255 + d[c] * da * (255 - a)) // oa255
            px[i + 3] = (oa255 + 127) // 255
    return bytes(px)


def extract_pet_icons(extract, pet_id):
    """A collab pet's four icon canvases as `{node: (w, h, format, payload)}`, from the extract."""
    cdir = os.path.join(extract, "canvas", "Item", "Pet", "%d" % pet_id)
    tree = json.load(open(os.path.join(cdir, "canvas.json"), encoding="utf-8"))
    out = {}
    for node in ["icon", "iconRaw", "iconD", "iconRawD"]:
        meta = tree["info"][node]
        out[node] = (int(meta["width"]), int(meta["height"]), int(meta["format"]),
                     open(os.path.join(cdir, "info.%s.bin" % node), "rb").read())
    return out


def classic_pet_icons(base_archive, build_dir, pet_id):
    """A classic pet's icon canvases, exported from the PRISTINE `_Canvas` archive (the
    `.bak`, once an install has made one) with `wz-dump canvas`."""
    out_dir = os.path.join(build_dir, "classic-pet-icons", "%d" % pet_id)
    os.makedirs(out_dir, exist_ok=True)
    r = subprocess.run([WZ_DUMP, "canvas", base_archive, "%d.img" % pet_id, out_dir, "info"],
                       capture_output=True, text=True, encoding="utf-8", errors="replace")
    if r.returncode != 0:
        raise SystemExit("wz-dump canvas %d.img failed: %s" % (pet_id, r.stderr.strip()))
    entries = json.load(open(os.path.join(out_dir, "manifest.json"), encoding="utf-8"))
    out = {}
    for e in entries:
        node = e["node"].strip("/").split("/")
        if len(node) == 2 and node[0] == "info" and node[1] in ("icon", "iconRaw", "iconD", "iconRawD"):
            payload = open(os.path.join(out_dir, "info.%s.bin" % node[1]), "rb").read()
            out[node[1]] = (int(e["width"]), int(e["height"]), int(e["format"]), payload)
    if "icon" not in out:
        raise SystemExit("%d.img has no info/icon canvas in %s" % (pet_id, base_archive))
    return out


def badge_pet_icons(build_dir, pet_id, pet_name, label, canvases):
    """Composite Nexon's Petite label onto each of one pet's icon canvases and write BGRA8888
    payloads. Returns `(canvas rows, png dir)`: the patch rows for the `_Canvas` image, and a
    folder of PNG renders for a look. Every pet gets it - the owner, 2026-09-18: *"Every single
    pet needs to have these P badges because every pet is now a Petite Luna pet."*"""
    wz_png = _wz_png()
    label, lw, lh = label
    out_dir = os.path.join(build_dir, "petite-%d" % pet_id)
    os.makedirs(out_dir, exist_ok=True)
    rows = []
    # **Only `icon` and `iconRaw`.** The owner, 2026-09-18, with Lil Frieren on the field: *"The
    # collaboration pets have the P icons on them as part of the animation. This is
    # undesirable."* The modern pets reuse `info/iconRawD` as an animation FRAME - eight of
    # Lil Frieren's and three of Lil Fern's stubs (`stand0/0`, `chat/0`, ...) outlink to it
    # [L] - so a badge on it walks around. `iconD`/`iconRawD` are the dead-doll icons, and a
    # pet with `life 0` is never dead here, so neither is ever shown: left as Nexon drew
    # them. No classic pet shares an icon with a frame; the rule is the same for all fifteen.
    for node, (w, h, fmt, payload) in sorted(canvases.items()):
        if node not in ("icon", "iconRaw"):
            continue
        rgba = wz_png.to_rgba(wz_png.inflate(payload), w, h, fmt)
        badged = composite(rgba, w, h, label, lw, lh, w - lw - LABEL_INSET, h - lh - LABEL_INSET)
        bgra = bytearray(badged)
        bgra[0::4], bgra[2::4] = bgra[2::4], bgra[0::4]
        payload_out = os.path.join(out_dir, "info.%s.bin" % node)
        with open(payload_out, "wb") as fh:
            fh.write(zlib.compress(bytes(bgra), 9))
        wz_png.write_png(pathlib.Path(os.path.join(out_dir, "info.%s.png" % node)), badged, w, h)
        rows.append("info/%s\tcanvas\t%d,%d,2,%s" % (node, w, h, payload_out))
    tsv = os.path.join(out_dir, "badge.tsv")
    with open(tsv, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# %s: UI/CashShop.img/CashItem_label/%d (Nexon's Petite badge) composited into icon and iconRaw (BGRA8888); the D icons are animation frames on two modern pets and never shown\n" % (pet_name, PETITE_LABEL_NODE))
        fh.write("\n".join(rows) + "\n")
    return tsv, out_dir
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
# The backported Innocence Scroll 70%, 2026-10-01. This client's scroll-applicability predicate
# (FUN_1404174b0) lets 2049000..2049199 onto any non-pet equip but sends 2049600 to the
# "scroll category == equip category" rule, which nothing meets - so the drag would be refused
# client-side. It wears 2049190: same 0204.img, a free number in the accepted range, and the
# icon outlinks still name the modern 02049600 canvas node, merged untouched (like the box).
# Must equal `world::scrolls::BACKPORTED`'s Innocence id. [L, research in STATUS.md 2026-10-01]
SCROLL_RENAMES = {2049600: 2049190}
# The Lucky Day Scroll's text, the owner's own (2026-10-01): it guarantees the next scroll, it
# does not add 10%.
LUCKY_DAY_ID = 2530000
LUCKY_DAY_DESC = "Increases the success chance of your next scroll by 100%"
# Every id that changes family or number on the way into the classic client, old -> new.
RENAMES = {BOX_MODERN_ID: BOX_ID, **FACE_COUPON_RENAMES, **SCROLL_RENAMES}

# The three hair-hats, 2026-09-18: Aura / Linie / Lügner Hair (Hat), 1006910..1006912. They
# would not go on a male character - no 0x0107, no message box, across every run since
# 2026-09-12 - and `islot Cp` (step 1d) was not the gate. The gate is the item ID. This
# client's gender-from-id rule (FUN_140253130, the fourth digit `(id / 1000) % 10`):
# 0 male, 1 female, 5 male, **6 female**, anything else unisex - and the three escape hatches
# in front of it (FUN_1402531f0, FUN_140416760, FUN_140416820) are hard-coded id ranges,
# not WZ keys. So a 1006xxx cap is female-only in this build, the body-part resolver
# (FUN_142d44b20 -> FUN_1402543e0) returns an empty list for a male character, and the
# double-click handler never calls the equip function. [L, the whole chain decompiled:
# research/hair-hat-islot-2026-09-12.md section 6.] the owner: "Nexon has made these items
# unisex. These items need to work on male characters ... fix it in the WZ data instead of
# patching the client." So they wear 1007910..1007912: digit 7 is unisex under that rule
# (316 of the classic client's own equips carry it), and the numbers are free. Only the
# PROPERTY image and its string move; the `_Canvas` image keeps its name, because the
# property image reaches its frames through explicit outlink paths
# (`Character/Cap/_Canvas/01006910.img/...`), the same reason the box's canvas stayed at
# 0522. The server's set contents (world::signaturestyle) and store::ITEM_ID_RENAMES (hats
# already in a bag) say the new numbers.
HAIR_HAT_RENAMES = {1006910: 1007910, 1006911: 1007911, 1006912: 1007912}


# ---------------------------------------------------------------------------------------
# Hair and face icons, so the Character Info ITEM tab can list them
# ---------------------------------------------------------------------------------------
#
# The owner, 2026-09-18: *"Character Info also does not show the full Item List of the character of
# everything they are wearing. This item list should include the hair, face, equipment and
# cash shop cover items that the player is wearing."* and, shown a modern client: *"Showing
# hair and face is absolutely do-able."*
#
# The ITEM tab builds one icon widget per item body and the client finds an equip's icon at
# `Character/<Type>/<id>.img/info/icon` - a hair is `Character/Hair/000300xx.img` under the
# same lookup (id / 10000 = 3 -> Hair, 2 -> Face). Every classic hair and face image carries
# `info/{islot,vslot,cash}` and NO `info/icon` (and neither do the modern ones: that client
# draws those cells in UI code). So this step renders one: the part's own `default` frame
# canvases - `hairBelowBody`, `hair`, `hairOverHead` for a hair, `face` for a face - composited
# on their shared `brow` anchor at natural size, cropped, and written inline as `info/icon` and
# `info/iconRaw` (BGRA8888, `newcanvas`) with the origin a cap icon uses: `(-2, height)`.
# `hairShade` is left out; it is the shadow the hair casts on the skin, not the hair.
#
# Every image in the classic Hair and Face archives gets one, and so does every backported
# hair and face (their canvases live in the modern `_Canvas` parts). **[I]** that the widget
# draws a synthesised icon for a hair id the way it draws a cap's; plan step "FAME, AND THE
# ITEM LIST" says what to look at.
LOOK_LAYERS = {
    "Hair": ("default/hairBelowBody", "default/hair", "default/hairOverHead"),
    "Face": ("default/face",),
}
ICON_ORIGIN_X = -2  # what every classic cap icon carries
# The Character Info ITEM tab draws nothing for an icon larger than its cell: Fern Hair's
# 46x56 render came up blank on 2026-09-18 while Fern Face's 27x17 and every cap (27..30 on a
# side) drew. Every classic equip icon fits in this box; a hair or face is scaled to it.
ICON_FIT = 32


def stub_payload(build_dir):
    """The 10-byte payload every real icon stub carries: one BGRA4444 pixel, deflated. Written
    once per build. **Every real icon is a 1x1 stub in the property image that outlinks to
    an originless pixel node in the `_Canvas` archive**, and the engine builds its canvas from
    the PIXEL node - so an origin on an inline canvas moved the picture (2026-09-18 evening:
    the tooltip drew a face 32 px low, then a hair; the ITEM tab cell, which computes its own
    position, did not care). Ours are the same two nodes now."""
    path = os.path.join(build_dir, "icon-stub.bin")
    if not os.path.exists(path):
        os.makedirs(build_dir, exist_ok=True)
        with open(path, "wb") as fh:
            fh.write(zlib.compress(b"\x00\x00", 9))
    return path


def to_bgra4444(rgba, w, h):
    """RGBA8888 -> the client's BGRA4444 (format 1): low byte G|B nibbles, high byte A|R -
    the inverse of `wz_png.to_rgba`. Rounded, so 255 -> 15 and 0 -> 0 exactly.

    **Every one of the 778 classic equip icon canvases is format 1** (a survey of
    Cap/Weapon/Longcoat/Shoes/Glove/Accessory/Cape `_Canvas` archives, 2026-09-18 evening) and
    the first hair/face icons went in as format 2: Fern Face drew in the list but its tooltip
    showed a garbled block - 8888 pixels read two bytes per pixel - and Fern Hair drew
    nowhere. So a synthesised icon is written in the one format the client has ever seen
    under `info/icon`."""
    out = bytearray(w * h * 2)
    for i in range(w * h):
        r, g, b, a = rgba[i * 4:i * 4 + 4]
        q = lambda v: (v * 15 + 127) // 255
        out[i * 2] = (q(g) << 4) | q(b)
        out[i * 2 + 1] = (q(a) << 4) | q(r)
    return bytes(out)


def pad_icon(rgba, w, h, box):
    """`rgba` centred on a transparent `box` x `box` canvas. `(rgba, box, box)`.

    **The equip tooltip anchors the image on the canvas ORIGIN and expects a cap-shaped icon:
    ~30 tall with the origin at its bottom row.** The owner, 2026-09-18 evening, three
    screenshots: a classic face (26x16, origin y 16) drew in the ITEM tab but its tooltip
    image sat at the bottom-left of the preview frame, half outside it; the list cell places
    by width and height (`FUN_141199600`: x centred, y bottom-aligned at posIcon) and was
    fine. So every synthesised icon is a full 32x32 canvas with origin (-2, 32), the shape of
    a cap's, and both widgets see what they were built for."""
    if w == box and h == box:
        return rgba, w, h
    out = bytearray(box * box * 4)
    x0, y0 = (box - w) // 2, (box - h) // 2
    for y in range(h):
        src = rgba[y * w * 4:(y + 1) * w * 4]
        o = ((y0 + y) * box + x0) * 4
        out[o:o + w * 4] = src
    return bytes(out), box, box


def fit_icon(rgba, w, h, box):
    """`rgba` scaled down to fit `box` x `box` (aspect kept; never scaled up), by area
    averaging in premultiplied alpha so a soft edge stays a soft edge, then padded to the
    box (`pad_icon`). `(rgba, box, box)`."""
    if w <= box and h <= box:
        return pad_icon(rgba, w, h, box)
    scale = max(w, h) / float(box)
    nw, nh = max(1, int(round(w / scale))), max(1, int(round(h / scale)))
    out = bytearray(nw * nh * 4)
    for y in range(nh):
        y0, y1 = int(y * h / nh), max(int(y * h / nh) + 1, int((y + 1) * h / nh))
        for x in range(nw):
            x0, x1 = int(x * w / nw), max(int(x * w / nw) + 1, int((x + 1) * w / nw))
            r = g = b = a = 0
            n = 0
            for sy in range(y0, min(y1, h)):
                for sx in range(x0, min(x1, w)):
                    i = (sy * w + sx) * 4
                    pa = rgba[i + 3]
                    r += rgba[i] * pa
                    g += rgba[i + 1] * pa
                    b += rgba[i + 2] * pa
                    a += pa
                    n += 1
            o = (y * nw + x) * 4
            if a:
                out[o], out[o + 1], out[o + 2], out[o + 3] = r // a, g // a, b // a, a // n
    return pad_icon(bytes(out), nw, nh, box)


def _image_names(archive):
    out = subprocess.run([WZ_DUMP, "tree", archive, "1"], capture_output=True, text=True,
                         encoding="utf-8", errors="replace")
    if out.returncode != 0:
        raise SystemExit("wz-dump tree %s failed: %s" % (archive, out.stderr.strip()))
    return [line.split("[IMG]", 1)[1].split()[0] for line in out.stdout.splitlines() if "[IMG]" in line]


def _layer_origins(prop_archive, image):
    """`{node path: (ax, ay, outlink)}` for every real canvas under `default`, from the
    property image: `(ax, ay)` is the layer's `origin + map/brow` - where its brow anchor sits
    relative to its top-left, so layers align when each is placed at top-left = -(ax, ay).
    `outlink` is where the pixels live (`Character/Hair/_Canvas/<image>/<node>`)."""
    out = subprocess.run([WZ_DUMP, "cat", prop_archive, image], capture_output=True, text=True,
                         encoding="utf-8", errors="replace")
    if out.returncode != 0:
        raise SystemExit("wz-dump cat %s %s failed: %s" % (prop_archive, image, out.stderr.strip()))
    tree = json.loads(out.stdout)
    found = {}

    def walk(node, path):
        if not isinstance(node, dict):
            return
        if node.get("_canvas"):
            o = node.get("origin") or {"x": 0, "y": 0}
            b = (node.get("map") or {}).get("brow") or {"x": 0, "y": 0}
            # A 1x1 canvas with no outlink is a placeholder (00030430's whole `default`
            # is one), not art: it has no pixels anywhere and gets no icon.
            if not node.get("_outlink") and node.get("width") == 1 and node.get("height") == 1:
                return
            found[path] = (int(o["x"]) + int(b["x"]), int(o["y"]) + int(b["y"]), node.get("_outlink"))
            return
        for k, v in node.items():
            walk(v, path + "/" + k if path else k)

    walk(tree.get("default"), "default")
    return found


def render_look_icon(wz_png, prop_archive, canvas_archive, image, canvas_image, layers, out_dir):
    """Composite `layers` of `image` on their brow anchor; `(rgba, w, h)` or None when the
    image has none of them (a placeholder hair, or a face with only animation frames).
    `canvas_image` is where the pixels live: the same name for a classic part, and the base
    colour's image for a backported hair or face (the colour variants share one)."""
    origins = _layer_origins(prop_archive, image)
    want = [l for l in layers if l in origins]
    if not want:
        return None
    # A colour variant's canvases live in the base colour's canvas image (00030430 outlinks
    # into 00030000.img): follow each layer's own outlink, and export each image once.
    exported = {}

    def canvases_of(cimg):
        if cimg not in exported:
            ex = os.path.join(out_dir, "x-" + cimg[:-4])
            r = subprocess.run([WZ_DUMP, "canvas", canvas_archive, cimg, ex, "default"],
                               capture_output=True, text=True, encoding="utf-8", errors="replace")
            if r.returncode != 0:
                raise SystemExit("wz-dump canvas %s %s failed: %s" % (canvas_archive, cimg, r.stderr.strip()))
            exported[cimg] = (ex, {e["node"].strip("/"): e for e in json.load(open(os.path.join(ex, "manifest.json"), encoding="utf-8"))})
        return exported[cimg]

    parts = []
    for l in want:
        ox, oy, outlink = origins[l]
        cimg, node = canvas_image, l
        if outlink:
            cimg, node = outlink.split("/_Canvas/", 1)[1].split("/", 1)
        ex, manifest = canvases_of(cimg)
        e = manifest.get(node)
        if e is None:
            continue  # the property image names a canvas the canvas archive does not hold
        payload = open(os.path.join(ex, e["file"]), "rb").read()
        rgba = wz_png.to_rgba(wz_png.inflate(payload), e["width"], e["height"], e["format"])
        # Align every layer on its brow anchor: top-left = -(origin + brow).
        parts.append((-ox, -oy, e["width"], e["height"], rgba))
    if not parts:
        return None
    x0 = min(p[0] for p in parts)
    y0 = min(p[1] for p in parts)
    x1 = max(p[0] + p[2] for p in parts)
    y1 = max(p[1] + p[3] for p in parts)
    w, h = x1 - x0, y1 - y0
    canvas = bytes(w * h * 4)
    for (px, py, pw, ph, rgba) in parts:  # in LOOK_LAYERS order: below, hair, over
        canvas = composite(canvas, w, h, rgba, pw, ph, px - x0, py - y0)
    return fit_icon(canvas, w, h, ICON_FIT)


def _info_canvas(prop_archive, image, node):
    """`(w, h, (ox, oy), outlink)` of `info/<node>` in a property image, or None."""
    out = subprocess.run([WZ_DUMP, "cat", prop_archive, image], capture_output=True, text=True,
                         encoding="utf-8", errors="replace")
    if out.returncode != 0:
        raise SystemExit("wz-dump cat %s %s failed: %s" % (prop_archive, image, out.stderr.strip()))
    v = (json.loads(out.stdout).get("info") or {}).get(node)
    if not isinstance(v, dict) or not v.get("_canvas"):
        return None
    o = v.get("origin") or {"x": 0, "y": 0}
    return v.get("width"), v.get("height"), (int(o["x"]), int(o["y"])), v.get("_outlink")


def cover_icons(build_dir, source, manifest, add):
    """`info/icon` for every backported equip whose modern image has only `info/iconRaw`.

    The owner, 2026-09-18, with Fern's Staff blank in another player's ITEM tab while its tooltip
    drew: the six modern weapon covers (1703722..1703727) carry `iconRaw` and no `icon` - the
    modern client's lists read `iconRaw`, this client's read `icon` (the tooltip reads
    `iconRaw`, which is why it drew). A copy of the `iconRaw` pixels under `info/icon`, with
    its origin, as a `newcanvas` row on the property image. Returns the ids patched."""
    # (no pixel work here since the outlink form: the stub borrows the iconRaw pixels)
    patched = []
    for items in manifest["sets"].values():
        for it in items:
            if it["type"] in ("Hair", "Face"):
                continue
            image = "%08d.img" % it["id"]
            dest = "%08d.img" % HAIR_HAT_RENAMES.get(it["id"], it["id"])
            prop = os.path.join(source, it["prop_archive"])
            if _info_canvas(prop, image, "icon") is not None:
                continue
            raw = _info_canvas(prop, image, "iconRaw")
            if raw is None or not raw[3]:
                print("  cover icon %-8d %s: neither info/icon nor an outlinked info/iconRaw - left alone" % (it["id"], it["name"]))
                continue
            tree_rel = "Character/" + it["type"]
            out_dir = os.path.join(build_dir, "cover-icons", dest[:-4])
            os.makedirs(out_dir, exist_ok=True)
            # A stub that borrows the iconRaw PIXELS the copied canvas image already holds -
            # the same shape as the iconRaw stub beside it, and exactly what a real icon is.
            tsv = os.path.join(out_dir, "icon.tsv")
            with open(tsv, "w", encoding="utf-8", newline="\n") as fh:
                fh.write("# %s: an info/icon stub outlinking to the iconRaw pixels (the modern image has no icon node; this client's lists read icon)\n" % it["name"])
                fh.write("info/icon\tnewcanvas\t1,1,1,%s,%d,%d,%s\n" % (stub_payload(build_dir), raw[2][0], raw[2][1], raw[3]))
            add(tree_rel, "patch\t%s\t%s" % (dest, tsv))
            patched.append(it["id"])
            print("  cover icon %-8d %s: info/icon stub -> %s" % (it["id"], it["name"], raw[3]))
    return patched


def look_icons(build_dir, source, manifest, add):
    """One `info/icon` + `info/iconRaw` per hair and face image, classic and backported, as
    `newcanvas` patch rows on the property archives. Returns `{type: count}`."""
    wz_png = _wz_png()
    counts = {}
    for kind, layers in LOOK_LAYERS.items():
        tree_rel = "Character/" + kind
        prop = os.path.join(CLASSIC, "Character", kind, kind + "_000.wz")
        canv = os.path.join(CLASSIC, "Character", kind, "_Canvas", "_Canvas_000.wz")
        # The pristine originals, never the last install (see step 5).
        prop = prop + ".bak" if os.path.exists(prop + ".bak") else prop
        canv = canv + ".bak" if os.path.exists(canv + ".bak") else canv
        jobs = [(image, prop, canv, image, image) for image in _image_names(prop)]
        # The backported ones: property image from its modern part (under the renamed id where
        # HAIR_HAT_RENAMES applies - the patch row must name the image the build wrote), canvases
        # from the modern `_Canvas` part that holds them.
        for items in manifest["sets"].values():
            for it in items:
                if it["type"] != kind:
                    continue
                image = "%08d.img" % it["id"]
                dest = "%08d.img" % HAIR_HAT_RENAMES.get(it["id"], it["id"])
                # The colour variants of one hair share the base colour's canvas image
                # (42541 outlinks into 00042540.img); the manifest names it.
                canvas_image = it["canvas_images"][0].rsplit("/", 1)[1]
                jobs.append((image, os.path.join(source, it["prop_archive"]),
                             modern_part(source, tree_rel + "/_Canvas", canvas_image), canvas_image, dest))
        out_root = os.path.join(build_dir, "look-icons", kind)
        os.makedirs(out_root, exist_ok=True)
        done = skipped = 0
        for image, p_arch, c_arch, canvas_image, dest in jobs:
            out_dir = os.path.join(out_root, dest[:-4])
            os.makedirs(out_dir, exist_ok=True)
            got = render_look_icon(wz_png, p_arch, c_arch, image, canvas_image, layers, out_dir)
            if got is None:
                skipped += 1
                continue
            rgba, w, h = got
            payload = os.path.join(out_dir, "icon.bin")
            with open(payload, "wb") as fh:
                fh.write(zlib.compress(to_bgra4444(rgba, w, h), 9))
            wz_png.write_png(pathlib.Path(os.path.join(out_dir, "icon.png")), rgba, w, h)
            # Two nodes per icon, the shape of a real one: the pixels, originless, in the
            # `_Canvas` archive under this image's own name (created there if the image has
            # none - a colour variant's does not exist), and a 1x1 stub with the origin and
            # the outlink in the property image.
            pixels = os.path.join(out_dir, "pixels.tsv")
            with open(pixels, "w", encoding="utf-8", newline="\n") as fh:
                fh.write("# %s %s: the icon pixels, rendered from the part's own default frame (%s); BGRA4444, no origin, like every _Canvas node\n"
                         % (kind, dest, ", ".join(l for l in layers)))
                for node in ("icon", "iconRaw"):
                    fh.write("info/%s\tnewcanvas\t%d,%d,1,%s,-,-\n" % (node, w, h, payload))
            add(tree_rel + "/_Canvas", "patch\t%s\t%s" % (dest, pixels))
            tsv = os.path.join(out_dir, "icon.tsv")
            with open(tsv, "w", encoding="utf-8", newline="\n") as fh:
                fh.write("# %s %s: info/icon and info/iconRaw stubs, origin (%d, %d) like a cap icon, outlinking to the _Canvas pixels\n"
                         % (kind, dest, ICON_ORIGIN_X, h))
                for node in ("icon", "iconRaw"):
                    fh.write("info/%s\tnewcanvas\t1,1,1,%s,%d,%d,%s/_Canvas/%s/info/%s\n"
                             % (node, stub_payload(build_dir), ICON_ORIGIN_X, h, tree_rel, dest, node))
            add(tree_rel, "patch\t%s\t%s" % (dest, tsv))
            done += 1
        counts[kind] = (done, skipped)
        print("  look icons %-4s %4d rendered, %d without a default frame (left without an icon)" % (kind, done, skipped))
    return counts


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

    # 1. Property images, per equip: copy from the modern part that holds it. A renamed one
    #    (HAIR_HAT_RENAMES) is written under its NEW name; the source image is the old one.
    part_cache = {}
    for set_name, items in manifest["sets"].items():
        for it in items:
            tree_rel = "Character/" + it["type"]
            image = "%08d.img" % it["id"]
            dest = "%08d.img" % HAIR_HAT_RENAMES.get(it["id"], it["id"])
            src = os.path.join(source, it["prop_archive"])
            add(tree_rel, "copy\t%s\t%s\t%s" % (dest, src, image))
            if dest != image:
                print("  rename   %-8s %8d -> %d  (a female-only id under the client's digit rule)" % (
                    set_name, it["id"], HAIR_HAT_RENAMES[it["id"]]))

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
    # not wear when double clicked on." Their `info/islot` is `HrCp`, the modern two-slot
    # type; every classic cap says `Cp`, so islot becomes `Cp`. **This was NOT the gate**
    # (2026-09-12 second launch: still no 0x0107) - the gate was the id's gender digit, see
    # HAIR_HAT_RENAMES. The `Cp` is kept: it is what every classic cap says, and `Hr` is a
    # token this client has no slot for. The patch targets the RENAMED image.
    for set_name, items in manifest["sets"].items():
        for it in items:
            if it["type"] != "Cap":
                continue
            prop = json.load(open(os.path.join(EXTRACT, "wz", "Cap", "%08d" % it["id"], "prop.json"), encoding="utf-8"))
            islot = prop.get("info", {}).get("islot", "")
            if not islot.startswith("Hr"):
                continue
            dest_id = HAIR_HAT_RENAMES.get(it["id"], it["id"])
            tsv = os.path.join(args.build_dir, "islot-%08d.tsv" % dest_id)
            with open(tsv, "w", encoding="utf-8", newline="\n") as fh:
                fh.write("# %s: islot %s -> Cp, the classic cap type; the client has no Hr slot\n" % (dest_id, islot))
                fh.write("info/islot\tstr\tCp\n")
            add("Character/Cap", "patch\t%08d.img\t%s" % (dest_id, tsv))
            print("  islot    %-8s %8d  %s -> Cp" % (set_name, dest_id, islot))

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
        if tree_rel == "Item/Pet":
            # A pet's canvas image is its own (`_Canvas/5002828.img`), like a Character
            # tree's and unlike the grouped `0568.img`: copied whole.
            add(tree_rel + "/_Canvas", "copy\t%s\t%s\t%s" % (image, src, image))
        elif tree_rel.startswith("Item/"):
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
    # The pets' three string images are flat `<id>/<leaf>` like Cash.img. `Pet.img` names the
    # pet (and the tooltip's desc / descD); `PetCommand.img/<id>/cN` are the words a chat line
    # is matched against and `PetDialog.img/<id>/<key>` the lines - `tools/dump_pets.py`
    # enumerates pets from PetCommand.img's keys, so without these rows the pet would summon
    # and never answer a command.
    for image, root in [("Eqp.img", strings["ClassicWorld"]), ("Cash.img", strings["Cash"]),
                        ("Consume.img", strings["Consume"]), ("Npc.img", npc_strings),
                        ("Pet.img", strings.get("Pet", {})), ("PetCommand.img", strings.get("PetCommand", {})),
                        ("PetDialog.img", strings.get("PetDialog", {}))]:
        if not root:
            continue
        tsv = os.path.join(args.build_dir, "strings-" + image + ".tsv")
        if image in ("Cash.img", "Consume.img"):
            # Renamed ids (RENAMES) carry their strings to the new id.
            root = {("%d" % RENAMES[int(k)]) if k.isdigit() and int(k) in RENAMES else k: v
                    for k, v in root.items()}
        if image == "Eqp.img":
            # The same for the hair-hats, one level down: `ClassicWorld/<Type>/<id>/name`.
            root = {t: {("%d" % HAIR_HAT_RENAMES[int(k)]) if k.isdigit() and int(k) in HAIR_HAT_RENAMES else k: v
                        for k, v in ids.items()}
                    for t, ids in root.items()}
        if image == "Consume.img" and str(LUCKY_DAY_ID) in root:
            root[str(LUCKY_DAY_ID)] = dict(root[str(LUCKY_DAY_ID)], desc=LUCKY_DAY_DESC)
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
    #     **Prices are the owner's.** 2026-09-10: *"each signature style set coupon should cost
    #     2000 LP"* and the box *"give[s] them all of the sets for 8000 LP"*. 2026-09-17:
    #     *"change the Frieren collaboration package prices from 2000 LP to 200 LP
    #     individually, or 800 LP for the signature set of all of them."* So a coupon is 200
    #     and the box - all eight - is 800, half the eight coupons' 1,600. The modern client
    #     sells the box for 7,900 NX as a one-at-random gacha and never sells the coupons;
    #     both rules are ours.
    #
    #     **The price lives in the CLIENT's Commodity.img** - the shop draws its price tags
    #     from the WZ - and the server debits from gm-handbook/commodity.txt, which --install
    #     regenerates from the installed archive. A price changed here reaches neither until
    #     `python tools/backport_install.py --install` is run with the client closed and the
    #     client package is rebuilt; until then the shop shows the old tag and the server
    #     charges the old amount, consistently.
    #
    #     Seen on screen 2026-09-10 at the first prices (7,900 / 3,900): the tab, the nine
    #     entries, the NEW badges, the icons and the tooltips all drew. The shape is proven;
    #     only the numbers changed after.
    rows = json.load(open(os.path.join(EXTRACT, "manifest.json"), encoding="utf-8"))
    by_name = {it["name"]: it["id"] for it in manifest["cash"]}
    wares = [("Signature Style Collection", COLLECTION_PRICE_LP)] + [
        (n + " Outfit Set Coupon", SET_COUPON_PRICE_LP)
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
    # 4d. The four collaboration pets. The owner, 2026-09-17: *"backport these pets into our build
    #     as well as these pet equipment. All pets from these collaboration should be 1000
    #     LP."* Property image copied whole from the modern `Item/Pet` (the canvas image is
    #     copied by step 2 - the classic client HAS an `Item/Pet/_Canvas` tree), then the same
    #     permanence / start-skill patch as the classic eleven, plus `del` rows for the
    #     leaves that name things this client does not have: `chatBalloon 911` and
    #     `nameTag 913` (the classic UI has ChatBalloon 0..25 and NameTag 3..14 - a leaf that
    #     names a missing UI node is a lookup, not an ignored key [L]), `setItemID 1127` (no
    #     SetItemInfo.img entry), and `sweepForDrop` (the pet agent's rule: a pet declares
    #     only Item Pouch; Auto Move is bought). `multiPet`, `autoBuff`, `noPermanentTicket`,
    #     `collabo`, `noPrism`, `wonderGrade` are left: unknown keys are skipped by name.
    #     [I] on whether the classic client draws the modern animation set (`love`, `sleep`,
    #     `what`, `roll`, `sit` are new; `rise`, `prone`, `nap`, `tedious`, `hand` are absent)
    #     - the interact table only names nodes the image has, and the plan step is the test.
    collab_pets = [(it["id"], it["name"]) for it in manifest.get("pets", [])]
    label = petite_label(source, args.build_dir)
    for it in manifest.get("pets", []):
        src = os.path.join(source, it["prop_archive"])
        add("Item/Pet", "copy\t%d.img\t%s\t%d.img" % (it["id"], src, it["id"]))
        # The Petite badge, onto the canvas image step 2 copies (this row runs after it:
        # specs are appended in order and the canvas copy was added in step 2 above).
        badge_tsv, badge_dir = badge_pet_icons(args.build_dir, it["id"], it["name"], label,
                                               extract_pet_icons(EXTRACT, it["id"]))
        add("Item/Pet/_Canvas", "patch\t%d.img\t%s" % (it["id"], badge_tsv))
        print("  badge    %8d  %s -> %s" % (it["id"], it["name"], os.path.relpath(badge_dir, REPO)))
        tsv = os.path.join(args.build_dir, "pet-%07d-strip.tsv" % it["id"])
        with open(tsv, "w", encoding="utf-8", newline="\n") as fh:
            fh.write("# %s: modern-only leaves the classic client would look up and not find\n" % it["name"])
            # `autoBuff` too (2026-09-18, the other session, for the owner: the tooltip listed
            # Auto Buff and Auto Move as skills; a collab pet declares only Item Pouch).
            for leaf in ["chatBalloon", "nameTag", "setItemID", "sweepForDrop", "autoBuff"]:
                fh.write("info/%s\tdel\n" % leaf)
            # The commands added for the animations the modern table never plays (`roll`,
            # `angry`) - the owner, 2026-09-17. An interact entry in the pet's own image, shaped
            # exactly like Nexon's 21: the action packet carries the entry's index and the
            # client plays its `act`. The words and lines land in PetCommand.img /
            # PetDialog.img under the same `cN` key (step 4), which is where dump_pets.py
            # joins them for the server.
            # Nexon's acts corrected to their own lines (backport_signature_style.py ACT_FIXES):
            # "sleep" played the poop animation and "talk" the sleep one.
            for fix in it.get("act_fixes", []):
                fh.write("# %s (%s): success %s -> %s, fail -> %s\n" % (fix["key"], fix["word"], fix["was"][0], fix["success"], fix["fail"]))
                fh.write("interact/%d/success/0/act\tstr\t%s\n" % (fix["index"], fix["success"]))
                fh.write("interact/%d/fail/0/act\tstr\t%s\n" % (fix["index"], fix["fail"]))
            for t in it.get("extra_commands", []):
                base = "interact/%d" % t["index"]
                fh.write("# %s: %s -> %s\n" % (t["key"], t["words"], t["act"]))
                fh.write("%s/command\tstr\t%s\n" % (base, t["key"]))
                for leaf, value in [("inc", t["inc"]), ("prob", t["prob"]), ("l0", 1), ("l1", 30)]:
                    fh.write("%s/%s\tint\t%d\n" % (base, leaf, value))
                for kind, act, count in [("success", t["act"], len(t["success"])), ("fail", t["fail_act"], len(t["fail"]))]:
                    fh.write("%s/%s/0/act\tstr\t%s\n" % (base, kind, act))
                    for i in range(count):
                        fh.write("%s/%s/0/%d\tstr\t%s_%s%d\n" % (base, kind, i, t["key"], kind[0], i + 1))
        add("Item/Pet", "patch\t%d.img\t%s" % (it["id"], tsv))
        print("  pet      %8d  %s (+%d commands)" % (it["id"], it["name"], len(it.get("extra_commands", []))))
    # ...and the classic eleven, whose canvas images are already in the base archive: the
    # icons come out of the PRISTINE `_Canvas` (the .bak, or the archive itself before any
    # install), get the same label, and go back with a `patch` on the existing image.
    pet_canvas_base = os.path.join(CLASSIC, "Item", "Pet", "_Canvas", "_Canvas_000.wz")
    if os.path.exists(pet_canvas_base + ".bak"):
        pet_canvas_base += ".bak"
    for pet_id, pet_name in pets:
        badge_tsv, badge_dir = badge_pet_icons(args.build_dir, pet_id, pet_name, label,
                                               classic_pet_icons(pet_canvas_base, args.build_dir, pet_id))
        add("Item/Pet/_Canvas", "patch\t%d.img\t%s" % (pet_id, badge_tsv))
        print("  badge    %8d  %s -> %s" % (pet_id, pet_name, os.path.relpath(badge_dir, REPO)))
    for pet_id, pet_name in pets + collab_pets:
        tsv = os.path.join(args.build_dir, "pet-%07d.tsv" % pet_id)
        with open(tsv, "w", encoding="utf-8", newline="\n") as fh:
            # **`info/life` is 0 on every pet: unlimited.** The owner, 2026-09-18: *"All pets
            # should be unlimited duration, some item descriptions in Cash Shop still dates
            # them for 3 days, 7 days or 90 days."*
            #
            # The days the shop prints are the pet's own `life` (research/pets-2026-09-13.md
            # section 1: the Commodity rows say Period 0). The tooltip builder
            # (`FUN_1426bc040`, string 0x03CD ' %d day(s)' at 1426bc29b) prints that line only
            # when the count is non-zero (`1426bc284 test r14d,r14d / je`), and the client
            # has no "Unlimited" branch there - so 0 means the line is simply absent, which
            # is as close to "unlimited" as this shop can say. And the dead-check
            # (`FUN_1402cf680`, research/pet-dead-is-datedead-2026-09-14.md) reads `life == 0`
            # as ALIVE UNCONDITIONALLY, which is the permanence the owner asked for on the pet
            # itself. [L] both.
            #
            # **This used to be left alone, on the belief that zeroing it made a summoned pet
            # invisible** - the 2026-09-14 diff found `life 7 -> 0` as the only WZ change and
            # the pet drew nothing. That was a coincidence written up as a cause: the pet was
            # blank with life 7 too, through eleven more commits, until f0c3010 found the real
            # one - `giantRate`, the pet's SIZE in percent, which this server had sent as 0.
            # The claim was never isolated against a control; CLAUDE.md's oldest rule.
            fh.write("info/life\tint\t0\n")
            fh.write("info/permanent\tint\t1\n")
            # **A pet declares only the skills it is BORN with: Item Pouch.** The owner,
            # 2026-09-17: "The only default skills it should have is Meso Magnet and Item
            # Pouch ... Auto Move, Expanded Auto Move, Auto HP Potion Skill, and Auto MP
            # Potion Skill are skills that players have to purchase and individually register."
            #
            # The tooltip lists a skill when the pet IMAGE declares it (this file) OR when the
            # item body's petSkill mask has learned it (FUN_1414b89b0: `test bit,mask; jne
            # list-as-learned`, else the declared-flag table decides). So a declared-but-
            # unlearned key printed "Expanded Auto Move / Auto Move ... This is an unregistered
            # pet." on every fresh pet, which is the clutter the owner is removing. A LEARNED skill
            # still lists (as "(Learned)") without its key here, so a purchased Auto Move or
            # Expanded Auto Move shows correctly once registered - the declaration is not what
            # lets it be bought.
            #
            #   pickupItem    Item Pouch          declared - the one default skill
            #   sweepForDrop  Auto Move           NOT declared; item 5190002 registers it
            #   longRange     Expanded Auto Move  NOT declared; item 5190003 registers it
            #   consumeHP/MP  Auto HP/MP Pouch    items 5190000/1, 100 LP - never declared
            #
            # The vacuum does NOT come from longRange here: it is the item's wonderGrade == 6
            # (net::bag::PET_WONDER_GRADE_VACUUM, on every pet - the in-range vacuum is free,
            # The owner 2026-09-17), which the client also labels "Petite Luna". So removing these
            # two keys leaves the vacuum,
            # the Petite Luna designation and the ability to register the skills all intact -
            # it only stops the fresh pet from advertising skills it has not learned.
            #
            # Meso Magnet needs no key at all: it is absent from the client's own skill
            # table (FUN_141ed1ad0) and shows with every key cleared.
            fh.write("info/pickupItem\tint\t1\n")
        add("Item/Pet", "patch\t%07d.img\t%s" % (pet_id, tsv))
    # 4e. The four pet weapons: `inline` (see the module table) from the modern PetEquip
    #     archive and its `_Canvas`. Nothing else to patch: `info/cash 1` puts them in the
    #     Deco tab (`Config::tab_for`), and each image's one pet node is what makes the staff
    #     fit its own pet and no other - the client's own rule, the same one its ten hats use
    #     with eleven nodes each.
    if manifest.get("pet_equips"):
        pe_src = modern_part(source, "Character/PetEquip", "%08d.img" % manifest["pet_equips"][0]["id"])
        pe_canvas = modern_part(source, "Character/PetEquip/_Canvas",
                                sorted(manifest["inline_canvas_images"])[0].rsplit("/", 1)[1])
        for it in manifest["pet_equips"]:
            add("Character/PetEquip", "inline\t%08d.img\t%s\t%08d.img\t%s" % (it["id"], pe_src, it["id"], pe_canvas))
            print("  petequip %8d  %s (fits %d)" % (it["id"], it["name"], it["pet"]))

    shipped_pet_rows = {5000001, 5000008, 5000009}  # SN 160000000..2 in the classic Commodity.img
    pet_rows_patch = os.path.join(args.build_dir, "patch-Commodity-pets.tsv")
    with open(pet_rows_patch, "w", encoding="utf-8", newline="\n") as fh:
        n = 0
        # The Pets tab (scope 600): the eight classic pets at the shipped rows' 100, then the
        # collaboration pets at COLLAB_PET_PRICE_LP. SN 160000003.. in that order, so the
        # eight keep the serials the 2026-09-13 install gave them.
        priced_pets = [(pid, name, 100) for pid, name in pets if pid not in shipped_pet_rows] + \
                      [(pid, name, COLLAB_PET_PRICE_LP) for pid, name in collab_pets]
        for pet_id, pet_name, price in priced_pets:
            sn = 160_000_003 + n
            row = classic_rows + len(wares) + n
            n += 1
            fh.write("# %s\n" % pet_name)
            for field, value in [
                ("SN", sn), ("ItemId", pet_id), ("Count", 1), ("Price", price), ("Bonus", 0),
                ("Period", 0), ("Priority", 100), ("ReqPOP", 0), ("ReqLEV", 0), ("Gender", 2),
                ("OnSale", 1), ("originalPrice", price), ("PbCash", 0), ("PbPoint", 0),
                ("PbGift", 0), ("Refundable", 0), ("WebShop", 0), ("IsGift", 0),
            ]:
                fh.write("%d/%s\tint\t%d\n" % (row, field, value))
        # The Pet Equip tab (scope 601, SN 1601xxxxx): the seven classic hats the shipped
        # table left out, then the four collaboration weapons, all at PET_EQUIP_PRICE_LP.
        # SN 160100003.. continues the shipped three.
        pet_equips = [(pid, name) for pid, name in CLASSIC_PET_EQUIPS if pid not in SHIPPED_PET_EQUIP_ROWS] + \
                     [(it["id"], it["name"]) for it in manifest.get("pet_equips", [])]
        for i, (item_id, name) in enumerate(pet_equips):
            sn = 160_100_003 + i
            row = classic_rows + len(wares) + n
            n += 1
            fh.write("# %s\n" % name)
            for field, value in [
                ("SN", sn), ("ItemId", item_id), ("Count", 1), ("Price", PET_EQUIP_PRICE_LP), ("Bonus", 0),
                ("Period", 0), ("Priority", 100), ("ReqPOP", 0), ("ReqLEV", 0), ("Gender", 2),
                ("OnSale", 1), ("originalPrice", PET_EQUIP_PRICE_LP), ("PbCash", 0), ("PbPoint", 0),
                ("PbGift", 0), ("Refundable", 0), ("WebShop", 0), ("IsGift", 0),
            ]:
                fh.write("%d/%s\tint\t%d\n" % (row, field, value))
    # 4f. Every named cash item the classic shop never listed. The owner, 2026-09-23: *"Add all of
    #     the items that are not listed but named except those that are part of the
    #     collaboration signature sets since they come from the Cash Coupons instead."*
    #
    #     `tools/cash_wares.py` has the whole argument. In short: the wares come from the
    #     PRISTINE client (so this build does not depend on what an earlier install left, and
    #     `--check` stays byte-exact), every placement copies the classic rows for the same kind
    #     of item (tab, price, period, gender), and the SNs continue each tab's shipped range in
    #     id order. The pets and pet hats this script already sells above are passed in so they
    #     are not listed twice. The collaboration items do not exist in the pristine client at
    #     all; the manifest check below must therefore come out empty, and stops the build if
    #     it does not - a collaboration piece in the shop would bypass the coupon.
    import cash_wares
    collaboration = set()

    def _ids(node):
        if isinstance(node, dict):
            if isinstance(node.get("id"), int):
                collaboration.add(node["id"])
            for v in node.values():
                _ids(v)
        elif isinstance(node, list):
            for v in node:
                _ids(v)
    _ids(manifest)
    collaboration |= {RENAMES.get(i, i) for i in collaboration} | {HAIR_HAT_RENAMES.get(i, i) for i in collaboration}
    already = [pid for pid, _name, _price in priced_pets] + [item_id for item_id, _name in pet_equips]
    ware_rows, unplaced, leaked = cash_wares.wares(also_sold=already, collaboration=collaboration)
    if leaked:
        raise SystemExit("step 4f: collaboration items would be sold outright: %s" % leaked)
    if unplaced:
        raise SystemExit("step 4f: no classic row says where these go: %s" % unplaced)
    wares_patch = os.path.join(args.build_dir, "patch-Commodity-classic-wares.tsv")
    with open(wares_patch, "w", encoding="utf-8", newline="\n") as fh:
        for sn, item_id, scope, price, period, gender, name in ware_rows:
            row = classic_rows + len(wares) + n
            n += 1
            fh.write("# %s (scope %d)\n" % (name, scope))
            for field, value in [
                ("SN", sn), ("ItemId", item_id), ("Count", 1), ("Price", price), ("Bonus", 0),
                ("Period", period), ("Priority", 100), ("ReqPOP", 0), ("ReqLEV", 0), ("Gender", gender),
                ("OnSale", 1), ("originalPrice", price), ("PbCash", 0), ("PbPoint", 0),
                ("PbGift", 0), ("Refundable", 0), ("WebShop", 0), ("IsGift", 0),
            ]:
                fh.write("%d/%s\tint\t%d\n" % (row, field, value))
        # And the classic shop's OWN rows, under the shop-wide rule (the owner, 2026-09-24):
        # everything 100 LP and permanent, except the hired merchants (7 days 700, 1 day 100)
        # and the Signature Style packages, which keep their prices. `cash_wares.price_rule`.
        # The server never applied a period, so the durations only change what the shop draws;
        # the prices are what the server debits, via the regenerated commodity.txt.
        shipped = cash_wares.shipped_row_fixes()
        for row, sn, item_id, fields in shipped:
            fh.write("# shipped SN %d, item %d: %s\n" % (sn, item_id, fields))
            for field, value in fields:
                fh.write("%d/%s\tint\t%d\n" % (row, field, value))
    print("  commodity  %d classic wares added, %d shipped rows brought under the price rule (step 4f)"
          % (len(ware_rows), len(shipped)))

    add("Etc", "patch\tCommodity.img\t%s" % commodity_patch)
    add("Etc", "patch\tCommodity.img\t%s" % pet_rows_patch)
    add("Etc", "patch\tCommodity.img\t%s" % wares_patch)
    add("Etc", "patch\tCashShopCategory.img\t%s" % category_patch)
    del rows

    # 4c. Hair and face icons, so the Character Info ITEM tab can list them (2026-09-18).
    #     Every classic hair and face image and every backported one. See `look_icons`.
    look_icons(args.build_dir, source, manifest, add)
    # 4d. info/icon for the backported equips whose modern image has only iconRaw (the six
    #     weapon covers): this client's lists read `icon`. See `cover_icons`.
    cover_icons(args.build_dir, source, manifest, add)

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
        for table in ["items.txt", "equips.txt", "commodity.txt", "itemdata.txt", "petcommands.txt"]:
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
        # cwd=REPO: the handbook tools name their inputs and outputs relative to the repo
        # (`client-patched\Data\...`, `gm-handbook\...`). The owner runs this from an elevated
        # window that opens in system32, and on 2026-09-17 every archive installed and then
        # dump_names.py reported "no archive at client-patched\Data\String\String_000.wz" -
        # the file it had just been handed, looked for in the wrong directory. CLAUDE.md's
        # oldest rule about their machine, one directory up from where it usually bites.
        p = subprocess.run([sys.executable, os.path.join(REPO, "tools", tool)], capture_output=True,
                           text=True, encoding="utf-8", errors="replace", cwd=REPO)
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
