# Backporting the "Signature Style Collection" from the modern client

The owner, 2026-09-10: *"figure out how to backport modern maplestory assets from the modern
maplestory client ... extract all WZ item/equipment related to the current event 'Signature
Style Collection'."* Source: `C:\Nexon\Library\maplestory\appdata` (v271). **Nothing there
was written to**; `tools/backport_signature_style.py` refuses an output path inside it.

`manifest.md` is the item table. `strings.json` is every name and description keyed the way
the classic `String.wz` keys them. `wz/` and `canvas/` (60 MB, regenerable, gitignored) are
the extracted trees and pixels. Regenerate with:

```bash
python "C:\MapleCW\tools\backport_signature_style.py"
```

## What the collection IS, read out of the client [L]

| layer | id(s) | what the WZ says |
|---|---|---|
| the box | Cash `5222221` | `cash 1, collabo 1`, an icon, **nothing else**. Sold by `Etc/Commodity.img` as SN 110000115 (7,900 NX, 1) and 110000116 (79,000 NX, 10), 7-day period. The **only** item in this family on sale |
| set coupons | Cash `5681543..5681552` | ten, not eight: Frieren has three (plain, Ringlets, Sleep). Each is `spec/script = cash_NNN`, `notConsume 1` |
| sets | Consume `2830642..2830658` | `spec/script = consume_NNN`, `notConsume 1`. Six "selector" coupons sit beside them (Clothes / Shoes / Gloves selectors, Himmel's Blessing) |
| hair coupons | Consume `2543137..2543143` | `spec/cosmetic = <hair id>` - the target is in the WZ |
| face coupons | Consume `2897007..2897014` | `spec/cosmetic = <face id>` |
| the equips | 206 ids, `manifest.md` | Longcoat, Shoes, Glove, Weapon (170xxxx covers), Accessory, Cap, Cape, Hair (8 colours each), Face (9 colours each) |

**Every package layer is a server script.** `notConsume 1` plus `spec/script` means the client
asks the server to act and removes nothing itself. No `Etc` package table (`CashPackage.img`,
`ClothingBox.img`) names any of these ids. So the *contents* of the box and of each set are
not in the client at all - they are the rule the owner wrote down, and `world` will hold it, the
same way it already holds the 5-slot coupons and the AP/SP resets (`session::cashitem`).

Two shape differences between the modern sets and the owner's listing, recorded rather than
resolved: the modern client has **three** Frieren set coupons (one per hairstyle) where the
listing has one set with all three hairs and all three clothes; and the modern sets hand out
"Clothes Selector" coupons where the listing hands out both clothes. The owner's listing wins.

Three characters have no hairstyle: **Aura, Linie and Lügner's hair is a Cap** ("Aura Hair
(Hat)", `1006910..1006912`, `islot HrCp`) worn over the head, plus a face.

## How the data is laid out, and why the extraction follows outlinks

Both clients keep an equip as **two images**: the property tree in
`Character/<Type>/<Type>_NNN.wz/<id>.img`, whose every canvas is a 1x1 stub with an
`_outlink`, and the pixels in `Character/<Type>/_Canvas/_Canvas_NNN.wz/<id>.img`. The classic
client (`client-patched/Data`) has the identical split - `Coat/01040021.img` outlinks into
`Coat/_Canvas/01040021.img` - so this is not a modern-only layout.

**Outlinks cross images.** A hair colour variant owns no pixels: `42541`'s nodes outlink into
`_Canvas/00042540.img`. Every face's expressions outlink into a shared sheet
`_Canvas/00022000.img`. The first extraction reported the variants as "0 canvases", which
was true and useless; the script now collects every outlink and exports each of the **58**
target images once, then checks that all 4,186 referenced paths are present. They are.
Pixel formats among ours: 4,184 format 1, 2 format 2 - both already implemented in
`tools/wz_png.py` and both used by the classic client.

Parts are found from `<Type>.ini`'s `LastWzIndex|N`, **not** from the 79-byte `<Type>.wz`
stub, which is byte-identical between a one-part and a two-part tree (modern `Coat.wz` vs
`Longcoat.wz`). Both clients use the zero string key. The only format difference is the
archive version: **271 modern, 779 classic**, which changes the offset hash and nothing else.

## INSTALLED, 2026-09-10 evening - the hybrid build is in `client-patched/Data`

`crates/wz/src/writer.rs` is the WZ writer; `wz-dump build` assembles an archive from a base
plus a spec; `tools/backport_install.py` writes the specs for all 24 affected classic
archives, builds them against the classic originals, verifies every image parses, and
installs them with `.bak` siblings (`--revert` restores). Every pre-existing image is carried
byte for byte and every new image is identical to its modern source - both checked by
re-reading and diffing JSON.

```bash
python "C:\MapleCW\tools\backport_install.py" --install
```

```bash
python "C:\MapleCW\tools\backport_install.py" --revert
```

Then regenerate the handbook (`dump_names`, `dump_equips`, `dump_itemdata`, `dump_commodity`,
`gen_item_rules`) so the server knows the items - 206 names, 78 item rows, 9 sale rows.

**The Cash Shop's Special tab now lists nine wares badged NEW**: the box at 7,900 LP and the
eight set coupons (Frieren, Fern, Stark, Übel, Himmel, Aura, Lügner, Linie) at 3,900 LP.
Read from the classic client: `CashShopCategory.img/2` had no sub-tab, which is why it was
blank; Main lists wares by explicit `commoditySN`, so Special now does too; `Class 0` on a
Commodity row is the NEW badge and `Class 2` is HOT - every one of the 15 rows on the Main
tab agrees with the owner's screenshot. **The prices are a placeholder policy** (the modern
client sells the box for 7,900 NX and never sells the coupons); change them in
`backport_install.py` and rebuild.

### The first client test, and what each outcome means

Not in `tools/test-server.ps1` yet - another agent held that file when this was written.

1. `!item 1054555 1` (Frieren's Clothes), equip it from the Equip tab.
   - it draws on the character -> the hybrid archive loads and a modern Longcoat renders
   - the slot fills but the body is bare -> the property image loaded and the pixels did not;
     say so, the `_Canvas` half is the suspect
   - the client dies on `!item` or on equip -> grab `client-exit.log`; the suspects in order are
     the `info/level/EquipmentSkill` node on the STAFFS (not on clothes, so try `!item 1703722`
     separately and last), then the archive itself
2. `!hair 42540` then `!face 22035`. Each re-enters the map.
   - Frieren's white hair and face draw -> the id space is open; the biggest unknown is closed
   - the head is blank -> the client range-checks the id before the `%08d` lookup; revert with
     `!hair 30025` / `!face 20002`
3. Cash Shop, SPECIAL tab: nine entries with a NEW badge.
   - buy the box (7,900 LP) -> it lands in the Cash Inventory
   - the tab is still blank -> the client does not honour an explicit list on that tab;
     the fallback is the scope form (`scope: 200`), one line in the installer
4. Do NOT double-click the box yet: opening it is the next piece of server work (below).

## Still to build - the server-side rules

* Box (`5222221`) -> one of the eight set coupons at random; set coupon (`5681543`..) -> the
  set's items plus its hair and face coupons; both are `0x0114` cash-item uses and today get
  "no arm for this item, kept". `session::cashitem` is where the 5-slot coupons live.
* Hair / face coupons (`2543137`.., `2897007`..): `spec/cosmetic` names the target; the
  request opcode for a Consume-tab double-click is not yet captured. `!hair` / `!face` are
  the same effect by hand.
* The modern-only nodes are still in: `info/level/.../EquipmentSkill` on the six weapons and
  `islot HrCp` on the three hats. Step 1 above measures whether they matter.

## The backport route as first planned, and what was NOT built then

The clean, reversible way in is the one the modern client uses itself: **add a `_001` part**
to each affected tree (`Longcoat_001.wz`, `Longcoat/_Canvas/_Canvas_001.wz`, and so on) and
bump the tree's `.ini` to `LastWzIndex|1`. No existing archive is rewritten and removing the
backport is deleting the new files and restoring one line.

What stands between the extracted data and a character wearing it:

1. **A WZ writer.** `crates/wz` reads only. Writing a v779 archive means: the PKG1 header,
   a directory of image entries with the version-779 offset encoding, and image
   serialisation for the four property types these trees use (int, string, vector, canvas
   with outlink). Canvas payloads are copied byte for byte - same formats, same zlib
   framing - so no pixel work. This is the one piece of real construction.
2. **Names.** `String.wz/Eqp.img` is one image; adding names means rewriting it (or
   finding out how the client resolves an image present in two parts, which is unknown).
   `strings.json` is already in the classic key layout (`ClassicWorld/<Type>/<id>`).
3. **Id acceptance in the classic client** - unmeasured and the likeliest hard stop.
   Classic hair ids run `30000..31807`, faces `20000..21825`; the modern ones are
   `42540..42607` and `22035..22842`. Whether the classic avatar validator accepts a
   five-digit hair in the `4xxxx` range or a face above `22000` has to be read out of the
   binary before anything is built on it. Longcoat, Shoes, Glove, Accessory, Cap and Cape ids
   all fall inside families the classic client already has; weapon covers `1703xxx` sit above
   its highest `1702026` but in the same family.
4. **Two modern-only nodes** to strip or test: the six staffs/sword/axe carry
   `info/level/case/.../EquipmentSkill 80012714` (a Zoltraak skill grant the classic client
   has no skill for), and the three Hair (Hat) caps use `islot HrCp`.
5. **The server rules.** Box -> random set coupon (the owner's eight), set coupon -> set, set ->
   items, hair/face coupon -> cosmetic change via `spec/cosmetic`. A cosmetic change has to
   go out as a re-entry (`SetField`): `0x0138 UserAvatarModified` is dead code in this
   client (`docs/opcodes.md`). Item rows for `gm-handbook` come from the `info` blocks in
   `wz/*/prop.json`.

None of 1-5 is started. What is done is the part that had to come first: every asset the
collection touches is located, extracted, verified complete, and rendered.
