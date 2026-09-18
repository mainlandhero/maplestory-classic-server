# Every change to the client's WZ data, and how to make them again

The client's `Data\` is not the Classic World data as Nexon shipped it. **Twenty-seven archives**
are rebuilt from the pristine originals by one script, and everything else about the client
(the executable, the hook, the launcher) is untouched on disk and patched at runtime. When
Nexon ships a new Classic World version, the originals change under us and every change below
has to be made again on top of the new ones. This document is the ledger for that.

The one rule that makes it possible: **nothing was ever hand-edited.** Every change is an
instruction in `tools/backport_install.py`, executed by `wz-dump build` against a pristine
base, and the pristine base is kept beside each built archive as `<name>_000.wz.bak`. Making
the changes again on a new version is: put the new originals in place, run the script, then
re-measure the handful of numbers the script read off the OLD data (section 5).

## 1. The pipeline

```text
modern client (read-only)        C:\Nexon\Library\maplestory\appdata\Data           source of the assets
   |  tools/backport_signature_style.py
   v
backport/signature-style/        manifest.json, strings.json, wz/, canvas/       the extracted assets
   |  tools/backport_install.py   (--install)
   v
client-patched/Data/             <Tree>_000.wz  built;  <Tree>_000.wz.bak  pristine base
   |  tools/dump_*.py, gen_item_rules.py      (run by --install)
   v
gm-handbook/                     the server's tables, regenerated from the INSTALLED archives
```

| step | command | what it does |
|---|---|---|
| extract | `python tools/backport_signature_style.py` | resolves the set items **by name** against the modern `String.wz`, dumps their property trees, follows every `_outlink` to its canvas image, writes the manifest. A name that resolves to nothing aborts |
| build | `python tools/backport_install.py` | writes one `wz-dump build` spec per classic tree and builds each against its `.bak` base into `backport/signature-style/build/`. The four ops, documented at `cmd_build` in `crates/wz/src/bin/wz_dump.rs`: `copy` an image byte for byte; `merge` named top-level keys of a source image onto the base's image of that name (`k=k2` renames); `strings` sets string leaves from `path<TAB>value`; `patch` sets `str`/`int`/`uol` leaves from `path<TAB>kind<TAB>value`, creating the image if absent. Later rows layer on earlier ones |
| verify | (inside build) | `wz-dump verify` parses every image of every built archive; a parse failure aborts |
| install | `python tools/backport_install.py --install` | client closed. Creates the `.bak` if absent, copies the build over, regenerates `gm-handbook/` (`dump_names`, `dump_equips`, `dump_itemdata`, `dump_commodity`, `gen_item_rules`) |
| preflight | `python tools/backport_install.py --check` | rebuilds into a scratch dir and requires every installed archive to hash equal; requires the handbook to be newer than the installed `String_000.wz`. Both packagers run this |
| undo | `python tools/backport_install.py --revert` | puts every `.bak` back |
| ship | `tools/make-installer.ps1`, `tools/package-server.ps1` | the client payload is `client-patched\Data` minus the `.bak`s; the server payload needs the handbook tables |

Built with `wz-dump` at classic version **779** (`CLASSIC_VERSION`). The tool is
`target/release/wz-dump.exe` (`cargo build --release -p wz`).

## 2. The ledger, archive by archive

Ids and counts are what the current extraction produced; the per-set breakdown is in
`backport/signature-style/manifest.md`. Tags: **[L]** read off the client's data or code, **[I]**
inferred, **[E]** the owner's rule.

### 2.1 The Signature Style Collection (Frieren collaboration) - 206 modern items

| classic archive | op | what |
|---|---|---|
| `Character/<Type>/<Type>_000.wz` (Hair, Face, Longcoat, Shoes, Glove, Accessory, Weapon, Cape, Cap) | `copy` | **164** equip property images, one per item, taken whole from the modern part that holds them. New ids to the classic client; nothing replaced. Frieren 39, Fern 21, Stark 24, Übel 21, Himmel 21, Aura 14, Lügner 12, Linie 12 (hair and face count every colour variant) |
| `Character/<Type>/_Canvas/_Canvas_000.wz` | `copy` | **58** canvas images - every outlink target, including the shared sheets no item is named after (`Face/00022000.img`, `Longcoat/01051850.img`). Copied once each, whole |
| `Item/Cash/Cash_000.wz` | `merge` | the ten Outfit Set Coupons `5681543..5681552` onto the classic `0568.img`; **the box `5222221` renamed to `5681599`** and merged onto `0568.img` (see renames) |
| `Item/Cash/_Canvas/_Canvas_000.wz` | `merge` | the `0568.img` and `0522.img` icon nodes the outlinks name (the box's icon still lives under its modern id in `0522`) |
| `Item/Consume/Consume_000.wz` | `merge` | the ten Outfit Sets and six Selector Coupons `2830642..2830658` onto `0283.img`; the seven Hair Coupons `2543137..2543143` onto `0254.img`; **the eight Face Coupons `2897007..2897014` renamed to `2890907..2890914`** and merged onto `0289.img` |
| `Item/Consume/_Canvas/_Canvas_000.wz` | `merge` | the matching icon nodes, keyed by the outlinks (three Frieren coupons share one icon node) |
| `String/String_000.wz` | `strings` | `Eqp.img`: `ClassicWorld/<Type>/<id>/name,desc` for every equip; `Cash.img` and `Consume.img`: flat `<id>/name,desc`, renamed ids carrying their strings; the box's `desc` is **ours** (it says the box gives all eight sets, because it does); `Npc.img/9010000/name` = "MapleStory Administrator" **[E]** |

Why each shape is what it is, with the measurement behind it:

* **The box changes id family, 5222221 -> 5681599.** The classic client opens a Cash item on
  double-click by its id family; family 522 has no items in this client and the click sent
  nothing, while the 568 coupons' double-click sends `0x0114` every time. **[L]** for the two
  controls, **[I]** for the family reading. `research/collection-box-id-2026-09-12.md`.
* **The face coupons change family, 2897xxx -> 28909xx.** The Beauty Coupon dialog opener
  (`0x141785d90`) accepts seven id ranges; `2897xxx` is in none of them, `2890000..2890999` is,
  and `2890907..` is free. **[L]** `research/face-coupon-family-2026-09-12.md`.
* **The set contents are not in the WZ.** Coupons and sets are `spec/script` items - the
  server decides what comes out (`world::signaturestyle`). The WZ supplies items and art only.

### 2.2 Fixes on top of the backported items

| classic archive | op | what | why |
|---|---|---|---|
| `Character/Weapon/Weapon_000.wz` | `patch` `<cover>.img`, `uol` | every cash weapon cover (`1700000..1799999` in the sets: Übel's Staff `1703726`) gets a UOL child named after **every weapon type the classic archive contains** (`30..49` present), pointing at the cover's own `30` subtree | the client permits a cover over a weapon only if the cover's image has a child named after the weapon's type; the modern covers carry `30` and `49` only, the classic `01702001.img` carries `30` plus links. **[L]** `research/weapon-cover-types-2026-09-12.md` |
| `Character/Cap/Cap_000.wz` | `patch` `info/islot` `str` `Cp` | the three hair-hats (Aura `1006910`, Lügner `1006911`, Linie `1006912`) lose the modern two-slot type `HrCp` | the client read the first token `Hr` as hair and had no bag destination for the double-click. **[I]**; still unverified on screen. `research/hair-hat-islot-2026-09-12.md` |
| `Effect/Effect_000.wz` | `merge` a **new** `ItemEff.img` | the worn-item effect node of every set item that has one (Himmel's Blessing `1103918`) from the modern `ItemEff.img` | the classic client has the loader and **no `ItemEff.img` at all** (26 images, none that). `research/himmel-cape-effect-2026-09-12.md` |
| `Effect/Effect_000.wz` | `patch` `ItemEff.img`, every `<id>/effect/**/z` `int` `-2` | the effect draws behind the body | Nexon's `z 10` drew in front; the modern image's plain auras carry `-2`. Confirmed on screen 2026-09-12 |
| `Effect/_Canvas/_Canvas_000.wz` | `merge` a new `ItemEff.img` | only the canvas holders the effect outlinks name (`1103930`) out of a 244 MB modern image | |

### 2.3 The Cash Shop

| classic archive | op | what |
|---|---|---|
| `Etc/Etc_000.wz` `Commodity.img` | `patch` rows **159..167** | the box (SN `120000000`, **800 LP**) and the eight set coupons (`120000001..120000008`, **200 LP** each); `Period 0`, `Class 0` (the NEW badge), `Gender 2`, `OnSale 1`. Prices **[E]** 2026-09-17 (8000 / 2000 from 2026-09-10 until then) |
| `Etc/Etc_000.wz` `Commodity.img` | `patch` rows **168..175** | the eight pets the classic shop never listed (`5000000, 5000002..5000007, 5000010`), SN `160000003..160000010`, 100 LP, `Period 0` - under the Pets tab by the SN arithmetic (category 6 / scope 600) |
| `Etc/Etc_000.wz` `CashShopCategory.img` | `patch` `2/0/name` `str` "Signature Style", `2/0/commoditySN/<n>` | the Special tab (category 2), which shipped empty, gets a sub-tab listing the nine serials |

| `Etc/Etc_000.wz` `Commodity.img` | `patch` rows **176..179** | the four collaboration pets (`5002828..5002831`), SN `160000011..160000014`, **1000 LP**, `Period 0` - the Pets tab. Prices **[E]** 2026-09-17 |
| `Etc/Etc_000.wz` `Commodity.img` | `patch` rows **180..190** | every pet equip the classic shop left out: the seven classic hats (`1802000, 1802001, 1802003, 1802004, 1802007, 1802008, 1802009`) and the four collaboration weapons (`1803148..1803151`), SN `160100003..160100013`, **100 LP**, `Period 0` - the Pet Equip tab (scope 601). The shipped table sold three of the ten hats **[L]**; the owner, 2026-09-17: *"make sure all pet equipment is available"* |

The row numbers are **appended after the classic 159 rows**; SN prefixes `12` and `16000000NN`
were read as unused before being taken. Both are re-measured on a new version (section 5).

### 2.4 The pets

| classic archive | op | what | why |
|---|---|---|---|
| `Item/Pet/Pet_000.wz`, `Item/Pet/_Canvas/_Canvas_000.wz` | `copy` `5002828..5002831.img` (property and canvas), then `patch` | **the four collaboration pets** - Lil Frieren, Lil Fern, Lil Stark, Lil Übel - from the modern client, 2026-09-17 | the modern client has each twice (a Heroic-world twin `5004047..5004050`); the lower id is taken. `del`: `info/chatBalloon 911`, `info/nameTag 913` (the classic UI has ChatBalloon 0..25 and NameTag 3..14 - a leaf naming a missing UI node is a lookup **[L]**), `info/setItemID 1127` (no `SetItemInfo.img` entry), `info/sweepForDrop` (a pet declares only Item Pouch); then the same `permanent 1` / `pickupItem 1` as the eleven. `life 90` left alone. **[I]**: the modern animation set differs (`love`, `sleep`, `what`, `roll`, `sit` new; `rise`, `prone`, `nap`, `tedious`, `hand` absent) - the interact table only names nodes the image has; whether the client draws them is the plan step |
| `Character/PetEquip/PetEquip_000.wz` | `inline` `01803148..01803151.img` | **the four collaboration pet weapons** (Lil Frieren's Staff, Lil Fern's Staff, Lil Stark's Axe, Lil Übel's Staff) | the classic client keeps pet-equip pixels INSIDE the property image - its ten hats have no `_Canvas` tree at all **[L]** - so a `copy` would leave 1x1 stubs outlinking into a tree the client has no folder for. `wz-dump build`'s `inline` (new 2026-09-17) follows every outlink into the modern `_Canvas_000.wz` (the frames live in a SHARED image, `01802653.img`) and writes the pixels in place, keeping the stub's `origin`/`z`/`delay`. 44 of 91 frames in 01803148 are genuine 1x1 blanks in the modern image (no link) and stay so. Each image's one pet node (`5002828`) is what makes the staff fit that pet |
| `String/String_000.wz` `Pet.img`, `PetCommand.img`, `PetDialog.img` | `strings` per pet | name, desc, descD; the command words (`cN`); the lines | the modern client keeps words and lines together in `PetDialog.img`; the classic client (and `tools/dump_pets.py`, which enumerates pets from `PetCommand.img`'s keys) keeps them apart, so the installer splits them. The desc is rewritten: the modern one sells a "Beyond Journey's End" set skill this client has no mechanism for, and it ends with the command list (`Commands: sit, slap, iloveyou, sleep, talk, roll, angry`) - the owner, 2026-09-17 |
| `Item/Pet/_Canvas/_Canvas_000.wz` (the four) | `patch` `info/icon`, `iconRaw`, `iconD`, `iconRawD` with the `canvas` kind | **Nexon's Petite "P" badge composited into the icon pixels** (BGRA8888) | The owner, 2026-09-17: *"Special petite luna pets should have an icon on the pet ... in both the Cash Shop item and the inventory icon"*, then *"use assets from the modern client"*. The modern client paints the label over the icon at draw time; the classic one has no such code. The badge is `UI/CashShop.img/CashItem_label/9` (12x12; 7 = Sweet, 8 = Dream, 9 = Petite, read off a contact sheet of all nineteen) **[L]**, alpha-blended at the bottom-right, one pixel in, the way the modern shop draws it. Both the shop and the bag draw `info/icon`, so one edit reaches both. `wz-dump build` gained the `canvas` patch kind for it |
| `Item/Pet/Pet_000.wz` (the four) | `patch` `interact/21`, `interact/22`; `interact/12..17/{success,fail}/0/act` | **two commands for the animations Nexon's table never played** (`roll`, `angry`, refusal `what`), and Nexon's `sleep` / `talk` acts corrected to their own lines | The owner, 2026-09-17: *"If the pets have more animations, make sure our pet chat commands support them."* Read off the images: `roll` and `angry` are real animations no entry reached (`sit` and `dung` are UOL aliases of `rest0`). And Nexon's own rows: `slap`/`iloveyou` (c5..c12) have NO lines in the modern `PetDialog.img`, so the dump dropped them - lines supplied; `sleep` (c13..15) said "Good night!" and played `dung` = `rest0`, `talk` (c16..18) played `sleep` and refused with `hungry`/`jump`/`move` - acts set to `sleep` / `chat`, refusal `what`. **[I]** that the lines carry the intent. `world::petcommands` has the test against the generated table |
| `Item/Pet/Pet_000.wz` | `patch` all eleven `5000000..5000010`: `info/life` `int` `0`, `info/permanent` `int` `1` | permanent, never expiring | the shop's "3 days" was each pet's own `life` **[L]**; the modern permanent pet `5000060` carries exactly this pair **[L]**. `research/pets-2026-09-13.md` |
| `Item/Pet/Pet_000.wz` | `patch` all eleven: `info/pickupItem 1`, `info/sweepForDrop 1`, `info/longRange 1` | every pet is a vacuum pet **[E]** | this client names all three keys and reads them in its pet loader; 370 of the modern archive's 1561 pets carry this trio **[L]**. What the radius is on screen is Nexon's code. `research/pet-vacuum-2026-09-13.md`. **History:** 2026-09-13 set 1/1/1, then the same day's skill-item work zeroed the two auto-move keys so the shop's items would teach them; 2026-09-16 (the owner: *"turn all pets into vacuum pets"*) sets them back to 1 and puts the two bits in `net::bag::PET_SKILLS_LEARNED_AT_START`, so declaration and mask agree without an item |

The "never needs reviving" half is the **server's** pet item body (`dateDead` = never), not WZ.

## 3. The server-side twins that must move with the WZ

A WZ change that renumbers or prices something has a copy in the server, and the two are kept
in step by tests. On a new version, change them together:

| the WZ fact | its twin | pinned by |
|---|---|---|
| box `5222221 -> 5681599`, face coupons `2897007..14 -> 2890907..14` | `store::inventory::ITEM_ID_RENAMES` (runs on every database open, so players' existing items are renumbered too); `world::signaturestyle::COLLECTION`; `world::cosmetics` (face coupon -> face id `22035..22042`) | `commodity` and `signaturestyle` tests read `gm-handbook/commodity.txt` |
| Commodity rows 159..175 | `world::commodity` expects **176** rows and the pet rows at `160000003` / `160000010` | `crates/world/src/commodity.rs` tests |
| the set contents | `world::signaturestyle` (the owner's listing of which items each coupon gives) | its tests |
| every id, name, price, slot count the world server debits and names | `gm-handbook/*.txt`, regenerated by `--install` | `--check` refuses a handbook older than the installed `String_000.wz`; `cargo test -p store` re-derives the item rules |
| pet permanence on the wire | `net::bag::pet_item_with_state` (`dateDead` never) | `net::bag` tests |

## 4. Client modifications that are NOT in the WZ

The executable on disk is byte-identical to Nexon's. Everything else is applied in memory at
launch by `grap-stub` and the launcher, and is versioned with our code - `docs/launcher.md`
"The patch inventory" is the authority. In one table, because every address in it belongs to
**this** executable build and will move on a new one:

| patch | site | what | retires |
|---|---|---|---|
| `grap64.dll` stub | (DLL) | GameGuard never starts | never |
| `skip_net_check` | `FUN_1415db360` -> `ret` | a firewalled reachability check `__fastfail`s the client | never |
| `suppress_login_dialog` | `FUN_141b2a280`, `rdx = 0` | the "trouble logging in" dialog blocks the login tick | when the login reply that stops result 12 is found |
| `enable_creation` | calls `FUN_140c9e230` | the create button's flag is set from virtualised code | when the packet that sets it is found |
| `mode=2` | the login-stage mode byte | see `docs/launcher.md`; possibly already retired | measure |
| `selectfill` | after every `0x0010`, `FUN_141177e40(selectUi)` | the blank character-select screen (`research/charselect-avatar-fade-race.md`) | never while `mode=2` stands. **Never put a `-Probe` watch on `141177e40`** |
| `guardpage=0x20+0x40` | a quarantine allocator for the `0x20` and `0x40` pool size classes | a stale-pointer write into freed-and-reused pool memory faults at the writer instead of days later; `crates/grap-stub/src/guardpage.rs` | see its docs |

The launcher's default session token is `mode=2,create=on,guardpage=0x20+0x40`
(`launcher::client::DEFAULT_SESSION`) and its default `-Probe` carries `1415db360:ret,141b2a280:rdx=0`.
`crates/grap-stub/src` holds **43 distinct** hard-coded image addresses; each is named for its function
in `research/` and re-found with `docs/ghidra.md` and the tools in `tools/` when the executable
changes. Beyond the patches, the whole server is written against this executable's opcode
tables and packet shapes (`crates/net/src/names.rs`, `research/msexe-*.txt`); a client update
that changes those is a server port, not a data reconciliation, and is outside this document.

## 5. The reconciliation procedure for a new Classic World version

Everything the installer does is a function of two inputs, the pristine classic data and the
extracted modern assets, plus a short list of **numbers it read off the old classic data**. The
procedure is: replace the first input, re-measure the numbers, run, and let the tests catch the
rest.

### 5.1 Before touching anything

1. Keep the old `client-patched\` whole somewhere - it is the last known-working client.
2. Note the old and new client version numbers and the `wz-dump` version tag (`779` today,
   `CLASSIC_VERSION`); `wz-dump tree <archive> 1` prints the archive's own version in its first
   line, and the new archives may say something else.
3. List the archives the installer touches (`--check` prints all 27) and confirm each still
   exists in the new data with the same part layout (`<Tree>.ini` `LastWzIndex`). A tree that
   grew a second part changes which file is the base.

### 5.2 Put the new originals in as the base

The installer's base for each tree is `<Tree>_000.wz.bak` if it exists, else `<Tree>_000.wz`.
So for a new version, the `.bak`s must become the NEW originals:

```powershell
# with the client closed, in the repo
python tools\backport_install.py --revert          # put the OLD originals back (tidy; optional)
# replace client-patched\Data with the NEW client's Data (the untouched copy of the new install)
# delete every *_000.wz.bak under client-patched\Data - they are the OLD originals
python tools\backport_install.py --install         # rebuilds on the new bases, creates new .bak, regenerates the handbook
python tools\backport_install.py --check
cargo test --workspace
```

If the extraction source (the modern client) has also moved on, re-run
`python tools\backport_signature_style.py` first. It resolves by **name**, so a renamed item
aborts loudly and a changed id shows up as a changed `manifest.json`; a changed id for the box
or a face coupon means `RENAMES` and the server twins in section 3 change with it.

### 5.3 Re-measure the numbers the script read off the OLD data

Each of these was a measurement, not a constant of the game. Re-take it; if it moved, the
script's constant (named) moves with it.

| assumption | where | how to re-measure |
|---|---|---|
| `Commodity.img` has rows `0..158` | `classic_rows = 159` | `wz-dump cat client-patched\Data\Etc\Etc_000.wz.bak Commodity.img` and count the top-level keys; ours must append after the last |
| SN prefix `12` (category 2) is unused; `160000003..160000010` are unused | step 4b/4c | search the same dump for `"SN": 12` and `16000000` values |
| the Special tab is `CashShopCategory.img/2` and declares no sub-tab | step 4b | `wz-dump cat ... CashShopCategory.img`, key `2` |
| the classic `Commodity.img` already lists pets `5000001, 5000008, 5000009` at SN `160000000..2` | `shipped_pet_rows` | same dump, `ItemId` in `5000000..5000010` |
| the classic Weapon archive's types are `30..49` minus `70` | step 1b | `wz-dump tree ...\Weapon\Weapon_000.wz.bak 1` - the images `01TT....img` |
| `Effect_000.wz` has **no** `ItemEff.img` | step 1c | `wz-dump tree ...\Effect\Effect_000.wz.bak 1`. **If Nexon added one, the `merge` now lays our nodes onto theirs; check the result has both** |
| family 568 opens on double-click and family 522 does not; the free stretch `2890907..` | `BOX_ID`, `FACE_COUPON_RENAMES` | only a run can re-measure the click; the free stretch: `wz-dump cat ...\Item\Consume\Consume_000.wz.bak 0289.img` and check the keys |
| NPC `9010000` is the Maple Administrator | step 4 | `wz-dump cat ...\String\String_000.wz.bak Npc.img`, key `9010000` |
| the eleven pets are `5000000..5000010` with `pickupItem 1` and a `life` in days | step 4c | `wz-dump cat ...\Item\Pet\Pet_000.wz.bak 500000N.img` |
| the classic client names the pet keys `sweepForDrop`, `longRange`, `pickupItem` | step 4c | a UTF-16 search of the new executable for each (the check in `research/pet-vacuum-2026-09-13.md` §2) |
| the hair-hats' `islot` is `HrCp` in the modern data | step 1d | `backport/signature-style/wz/Cap/<id>/prop.json` after re-extraction |

### 5.4 What the tests will catch, and what only a launch will

`cargo test --workspace` pins the box id, the face coupon ids, the commodity row count, the pet
rows, the set contents and the item rules against the regenerated handbook, so an id that
moved fails on the dev box. What the tests **cannot** see is the client's own behaviour: that
the box opens, the covers equip, the cape draws behind, the hats equip, the pets summon and
vacuum. `tools/test-server.ps1`'s plan steps for each (TO(i), TO(j), TO(p), TO(u)) are the
launch checks; run them once on the new version even if every test is green.

### 5.5 If the executable changed

Section 4 applies in full: every hard-coded address in `grap-stub` and every opcode number the
server speaks is a measurement of the old executable. `docs/ghidra.md` and `research/` hold how
each was found; `tools/xref.py`, `tools/reads.py`, `tools/encodes.py`, `tools/callers.py` and
`tools/dispatchers.py` re-find them. That is a port, and this document ends where it begins.

## 6. Where everything lives

| | |
|---|---|
| `tools/backport_signature_style.py` | the extraction: the sets by name, the outlink walk, the manifest |
| `tools/backport_install.py` | the ledger as code - every op in section 2 is a line in it, with its reason |
| `backport/signature-style/` | the extracted assets and the last build (gitignored: rebuilt from the two clients) |
| `client-patched/Data/**/*_000.wz.bak` | the pristine bases; excluded from the client payload |
| `docs/wz-format.md`; `crates/wz/src/bin/wz_dump.rs` (`cmd_build`) | the on-disk WZ format; the build spec's four ops |
| `docs/deployment.md` "The client payload carries the hybrid WZ archives" | why server and client ship together |
| `docs/launcher.md` "The patch inventory" | the runtime patches in section 4 |
| `research/*-2026-09-1[023].md` named above | the measurement behind each shape |
