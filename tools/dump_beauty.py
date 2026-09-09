#!/usr/bin/env python3
"""Hair, face and the coupons that change them, from the client's own WZ.

Writes `gm-handbook/beauty.txt`. Same shape as `tools/dump_chairs.py` and
`tools/dump_itemdata.py`: generated game data, gitignored, never hand-edited.

    python tools/dump_beauty.py

Run it from the repo root (`python tools/dump_beauty.py`), not from a scratchpad - see
`CLAUDE.md`, "The scratchpad shadows the real tools".

## Why this exists

The server can already store a character's `hair` and `face` and it puts both on the wire
(`net::opcode::avatar_look`). What it has never had is the client's own answer to two
questions:

1. **Which ids draw at all.** A hair id with no `.img` under `Character/Hair` composes to
   nothing - the character renders bald, with no error anywhere. That failure looks exactly
   like a packet bug and is not one.
2. **Which items are the coupons, and what each one offers.** The coupon items themselves
   carry no behaviour: `Item/Cash/Cash_000.wz` `0515.img` gives every one of the ten
   `info = {cash: 1}` and nothing else. The candidate lists live somewhere else entirely -
   `Etc/BeautyPreview.img` - keyed by coupon id and split by gender.

## The four sources, and the region each is in

| section | archive | node |
|---|---|---|
| `[coupons]` | `Item/Cash/Cash_000.wz` | `0515.img` - the **cash** region, not `Consume` |
| `[sale]` | `Etc/Etc_000.wz` | `Commodity.img`, rows whose itemId is a coupon |
| `[preview]` | `Etc/Etc_000.wz` | `BeautyPreview.img` - coupon -> candidate ids, per gender |
| `[hair]` / `[face]` | `Character/Hair/Hair_000.wz`, `Character/Face/Face_000.wz` | one `.img` per id |

`Item/Consume` carries no coupon at all. Saying which region a row came from is the point
of the `region` column: a reader that assumes "usable item, therefore Consume" finds
nothing and concludes the feature is absent.

## The id arithmetic, measured rather than assumed

Both hold exactly, with **no exceptions**, over every image in the two archives:

    hair   base = id - (id % 10),  colour = id % 10,  colour in 0..7
           159 bases x 8 colours = 1272 images.  30xxx male (81 bases), 31xxx female (78)

    face   style = id % 100,  eye colour = (id // 100) % 10,  eye colour in 0..8
           52 styles x 9 eye colours = 468 images.  20xxx male (26), 21xxx female (26)

That regularity is what makes a colour coupon implementable without a second table: a hair
colour change is `base + newColour`, an eye colour change is `id % 100 + newColour * 100`
within the same gender thousand. The script asserts both decompositions and refuses to
write if either breaks, because a partial regularity would be worse than none.

**Gender is a convention of the id range, not a check the client makes.** Nothing in the
compact-look reader `FUN_1402ee8d0` validates hair or face; it validates the *equipment*
pairs (`FUN_140253980`) and silently drops the ones that fail, but hair is read as a bare
`u32` into equipment slot 0. So a `30xxx` hair on a female character draws male hair - it
is wrong, not invisible. Only an id with no `.img` is invisible.

## A row means MEASURED

Every id listed here has a real image in the archive - the smallest hair image is 4713
bytes and the smallest face image 6919, so this client ships **no empty placeholders** and
"present in the tree" and "has art" are the same statement here. If a future client does
ship 13-byte stubs (the Cash archive is full of them), this script's `MIN_IMAGE_BYTES`
check will start rejecting them and the count guard will fire.

## The instrument

A raw byte search over a `.wz` returns zero for a property that is really there, because WZ
encodes property names. Everything here goes through `wz-dump`, and the script fails loudly
if it cannot find or run it. It also refuses to overwrite a good table with a short one:
five separate count guards, each with the number it expected.
"""
import json
import os
import re
import subprocess
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

WZ_DUMP = os.path.abspath(
    os.path.join("target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
)

CASH_WZ = os.path.join("client-patched", "Data", "Item", "Cash", "Cash_000.wz")
ETC_WZ = os.path.join("client-patched", "Data", "Etc", "Etc_000.wz")
HAIR_WZ = os.path.join("client-patched", "Data", "Character", "Hair", "Hair_000.wz")
FACE_WZ = os.path.join("client-patched", "Data", "Character", "Face", "Face_000.wz")

COUPON_IMG = "0515.img"
OUT = os.path.join("gm-handbook", "beauty.txt")

# Guards. Each is well under the real count, so a normal client passes and a broken read
# fails. The reason they are here at all: a silently-short table looks exactly like a
# working one, and this project has shipped that mistake before.
MIN_COUPONS = 8
MIN_HAIR = 400
MIN_FACE = 200
MIN_PREVIEW_GROUPS = 4
MIN_PREVIEW_ENTRIES = 100
MIN_IMAGE_BYTES = 100  # a 13-byte .img is an empty placeholder, not art


def run(archive, node):
    r = subprocess.run([WZ_DUMP, "cat", archive, node], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        print("wz-dump cat %s %s failed: %s" % (archive, node, r.stderr or "no output"))
        return None
    return json.loads(r.stdout)


IMG_LINE = re.compile(r"\[IMG\]\s+(\d+)\.img\s+size=(\d+)")


def image_ids(archive):
    """Every `<digits>.img` in an archive, with its size. Ids are the WZ node names."""
    r = subprocess.run([WZ_DUMP, "tree", archive, "1"], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        print("wz-dump tree %s failed: %s" % (archive, r.stderr or "no output"))
        return None
    out = []
    for line in r.stdout.splitlines():
        m = IMG_LINE.search(line)
        if m:
            out.append((int(m.group(1)), int(m.group(2))))
    return sorted(out)


def load_names():
    names = {}
    path = os.path.join("gm-handbook", "items.txt")
    if os.path.exists(path):
        with open(path, encoding="utf-8") as fh:
            for line in fh:
                if line.startswith("#"):
                    continue
                item_id, _, name = line.partition(",")
                if name:
                    names[item_id.strip().lstrip("0") or "0"] = name.strip()
    return names


def main():
    if not os.path.exists(WZ_DUMP):
        print("no wz-dump at %s - build it first: cargo build --release -p wz" % WZ_DUMP)
        return 2
    for path in (CASH_WZ, ETC_WZ, HAIR_WZ, FACE_WZ):
        if not os.path.exists(path):
            print("no archive at %s" % path)
            return 2

    names = load_names()

    # --- coupons -----------------------------------------------------------------
    coupon_tree = run(CASH_WZ, COUPON_IMG)
    if coupon_tree is None:
        return 1
    coupons = []
    for key in sorted(coupon_tree):
        if not key.isdigit():
            continue
        info = coupon_tree[key].get("info")
        if not isinstance(info, dict):
            continue
        # `icon` / `iconRaw` are art, present on every cash item; they say nothing about
        # behaviour. Everything else in `info` is the whole of what the client's data
        # claims this coupon does.
        props = sorted(k for k in info if not k.startswith("icon"))
        item_id = int(key)
        coupons.append((item_id, props, names.get(str(item_id), "")))

    if len(coupons) < MIN_COUPONS:
        print("REFUSING TO WRITE: found %d coupon(s) in %s, expected at least %d."
              % (len(coupons), COUPON_IMG, MIN_COUPONS))
        print("%s left untouched." % OUT)
        return 1

    # --- BeautyPreview: coupon -> candidate ids, per gender -----------------------
    preview = run(ETC_WZ, "BeautyPreview.img")
    if preview is None:
        return 1
    preview_rows = []
    entry_total = 0
    for group in sorted(preview):
        node = preview[group]
        if not isinstance(node, dict):
            continue
        for coupon in sorted(node):
            lst = node[coupon]
            if not isinstance(lst, dict):
                continue
            ids = [lst[k] for k in sorted(lst, key=lambda s: int(s) if s.isdigit() else -1)
                   if isinstance(lst[k], (int, float))]
            if not ids:
                continue
            entry_total += len(ids)
            preview_rows.append((group, int(coupon), [int(v) for v in ids]))

    if len(preview_rows) < MIN_PREVIEW_GROUPS or entry_total < MIN_PREVIEW_ENTRIES:
        print("REFUSING TO WRITE: BeautyPreview.img gave %d row(s) / %d id(s), expected at "
              "least %d / %d." % (len(preview_rows), entry_total,
                                  MIN_PREVIEW_GROUPS, MIN_PREVIEW_ENTRIES))
        print("%s left untouched." % OUT)
        return 1

    # --- the drawable ids ---------------------------------------------------------
    hair = image_ids(HAIR_WZ)
    face = image_ids(FACE_WZ)
    if hair is None or face is None:
        return 1
    if len(hair) < MIN_HAIR or len(face) < MIN_FACE:
        print("REFUSING TO WRITE: %d hair and %d face image(s), expected at least %d and %d."
              % (len(hair), len(face), MIN_HAIR, MIN_FACE))
        print("That is the shape of a broken read rather than of a client with no hair.")
        print("%s left untouched." % OUT)
        return 1

    stubs = [i for i, size in hair + face if size < MIN_IMAGE_BYTES]
    if stubs:
        print("REFUSING TO WRITE: %d id(s) have an image under %d bytes - a placeholder that "
              "draws nothing. First: %s" % (len(stubs), MIN_IMAGE_BYTES, stubs[:8]))
        print("Listing them as valid is exactly the invisible-hair bug this file exists to")
        print("prevent. %s left untouched." % OUT)
        return 1

    hair_ids = [i for i, _ in hair]
    face_ids = [i for i, _ in face]

    # The arithmetic is asserted, not assumed. If a future client breaks it, the server's
    # colour maths breaks with it and this is where that has to be noticed.
    hair_bases = {}
    for i in hair_ids:
        hair_bases.setdefault(i - i % 10, []).append(i % 10)
    bad_hair = {b: sorted(c) for b, c in hair_bases.items() if sorted(c) != list(range(8))}
    face_styles = {}
    for i in face_ids:
        face_styles.setdefault((i // 1000) * 1000 + i % 100, []).append((i // 100) % 10)
    bad_face = {s: sorted(c) for s, c in face_styles.items() if sorted(c) != list(range(9))}
    if bad_hair or bad_face:
        print("REFUSING TO WRITE: the id decomposition does not hold.")
        print("  %d hair base(s) without all 8 colours: %s"
              % (len(bad_hair), sorted(bad_hair)[:6]))
        print("  %d face style(s) without all 9 eye colours: %s"
              % (len(bad_face), sorted(bad_face)[:6]))
        print("The server's colour arithmetic (base + colour) rests on this. %s left "
              "untouched." % OUT)
        return 1

    # --- cash-shop sale rows for the coupons --------------------------------------
    commodity = run(ETC_WZ, "Commodity.img")
    coupon_ids = set(i for i, _, _ in coupons)
    sale_rows = []
    if isinstance(commodity, dict):
        for key in sorted(commodity):
            row = commodity[key]
            if not isinstance(row, dict):
                continue
            item_id = row.get("ItemId")
            if not isinstance(item_id, (int, float)) or int(item_id) not in coupon_ids:
                continue
            sale_rows.append((int(row.get("SN", 0)), int(item_id),
                              int(row.get("Price", 0)), int(row.get("Gender", -1)),
                              int(row.get("OnSale", 0))))
    # No guard on this one on purpose: a coupon with no sale row is a real statement
    # (05151200 and 05152300 have none), so an empty result here is data, not a failure.

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8", newline="\n") as fh:
        w = fh.write
        w("# generated by tools/dump_beauty.py from the client's own WZ. Never hand-edit.\n")
        w("# Sources, and the region each section came from:\n")
        w("#   [coupons] Item/Cash/Cash_000.wz  0515.img       (the CASH region, not Consume)\n")
        w("#   [sale]    Etc/Etc_000.wz         Commodity.img\n")
        w("#   [preview] Etc/Etc_000.wz         BeautyPreview.img\n")
        w("#   [hair]    Character/Hair/Hair_000.wz   one .img per id\n")
        w("#   [face]    Character/Face/Face_000.wz   one .img per id\n")
        w("#\n")
        w("# An id listed under [hair] or [face] HAS ART. An id that is absent draws\n")
        w("# nothing at all and reports no error - that is the whole reason for this file.\n")
        w("# Gender is a convention of the id range; the client's look reader does not\n")
        w("# check it. A 30xxx hair on a female character draws MALE HAIR, not nothing.\n")
        w("#\n")

        w("[coupons] itemId, infoProps, name\n")
        w("# infoProps is every `info` child that is not art. `cash` on its own means the\n")
        w("# item's own data says NOTHING about what it does - the behaviour is the\n")
        w("# server's to define. `choice` marks a coupon the client treats as picked-from.\n")
        for item_id, props, name in coupons:
            w("%08d, %s, %s\n" % (item_id, "|".join(props) or "-", name or "?"))
        w("\n")

        w("[sale] commoditySN, itemId, price, gender, onSale\n")
        w("# gender: 0 male-only, 1 female-only, 2 either. A coupon with no row here is\n")
        w("# not sold in this client's cash shop, which is a fact, not a missing read.\n")
        for sn, item_id, price, gender, on_sale in sorted(sale_rows):
            w("%d, %08d, %d, %d, %d\n" % (sn, item_id, price, gender, on_sale))
        w("\n")

        w("[preview] group, couponId, count, candidate ids\n")
        w("# The client's own candidate list per coupon, from Etc/BeautyPreview.img. These\n")
        w("# are BASE ids (colour digit 0 for hair, eye colour 0 for face) - the colour is\n")
        w("# chosen separately. Note the coupon ids here are a SUPERSET of the ten items in\n")
        w("# [coupons]: 5150001 and 5150101 have preview lists but no item and no sale row.\n")
        for group, coupon, ids in sorted(preview_rows):
            w("%s, %d, %d, %s\n" % (group, coupon, len(ids),
                                    " ".join(str(v) for v in ids)))
        w("\n")

        w("[hair] baseId, gender, colours, name\n")
        w("# One row per BASE. The eight drawable ids are baseId+0 .. baseId+7.\n")
        for base in sorted(hair_bases):
            gender = "male" if base < 31000 else "female"
            w("%d, %s, 0-7, %s\n" % (base, gender, names.get(str(base), "?")))
        w("\n")

        w("[face] styleId, gender, eyeColours, name\n")
        w("# One row per STYLE, written at eye colour 0. The nine drawable ids are\n")
        w("# styleId + 100*c for c in 0..8.\n")
        for style in sorted(face_styles):
            gender = "male" if style < 21000 else "female"
            w("%d, %s, 0-8, %s\n" % (style, gender, names.get(str(style), "?")))

    male_hair = sum(1 for b in hair_bases if b < 31000)
    male_face = sum(1 for s in face_styles if s < 21000)
    print("wrote %s" % OUT)
    print("  %d coupon item(s) in %s, %d cash-shop sale row(s)"
          % (len(coupons), COUPON_IMG, len(sale_rows)))
    print("  %d preview list(s), %d candidate id(s) total" % (len(preview_rows), entry_total))
    print("  hair: %d ids = %d bases x 8 colours  (%d male, %d female)"
          % (len(hair_ids), len(hair_bases), male_hair, len(hair_bases) - male_hair))
    print("  face: %d ids = %d styles x 9 eye colours  (%d male, %d female)"
          % (len(face_ids), len(face_styles), male_face, len(face_styles) - male_face))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
