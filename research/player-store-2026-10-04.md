# Player stores: the whole protocol, decoded statically

2026-10-04. From `client-patched/MapleStory.exe` (base `0x140000000`), Ghidra headless plus
`tools/reads.py`, `tools/encodes.py`, `tools/listing.py`. Implemented in `crates/net/src/playershop.rs`,
`crates/store/src/playershop.rs` and `crates/world/src/session/playershop.rs`. **Nothing here has been
on a screen yet** - plan step 49.

The owner: *"I just tried to open a player store, but nothing happened."* The create arrived
(`world-ch0.log` 18:14:31, `0x017F`, `00000000 03000000 0700 "garbage" 0f00 216e4e00`) and the server
answered it with the generic latch unlock, because no handler knew the opcode.

Tags: **[L]** read off the listing or decompile, **[D]** derived, **[I]** inferred.

## 0. A store is not the trade/Omok miniroom

Trade, Omok and Match Cards share `0x017E` out and `0x0575` in, and `0x0575` mode 4 builds a window
only for room types 1, 3 and 4 (`FUN_141C3D980`) [L]. The store has **two opcodes each way** and its
own dialog class. Found by following the create's builder (`FUN_142CD4550`, the only `0x017F` sender
whose write shape matches the capture) into its neighbourhood: every other `0x017F` sender, both
`0x0180` senders and four inbound handlers sit in `0x140D90E90..0x140DA2000` [L].

Inbound opcodes, from `CField::OnPacket`'s two-level table at `141821a52` (byte table `0x1418223C4`,
dword table `0x141822358`, decoded by script and reproducing `research/storage.md`'s `0x0572` and
`research/trade-2026-09-09.md`'s `0x0575` exactly) [L]:

| opcode | handler | what |
|---|---|---|
| `0x0576` | `FUN_140D90E90` | an event in an open window |
| `0x0577` | `FUN_140D91000` | the room: open, close, messages |
| `0x0578` | `FUN_140D949C0` | the shelf, unasked |
| `0x0579` | `FUN_140D94BB0` | the answer to a shelf request |

The window class: constructor `FUN_140D984D0`, 0x438 bytes, vtable `0x14336F0C0`, base
`FUN_140D91E20` (vtable `0x14336E2A0`) [L]. `0x0576`/`0x0577` check the open window against
descriptor `0x143A87E08`, `0x0578`/`0x0579` against `0x143A86BA8`. The class's own IsKindOf
(`FUN_140DA1FA0`) starts at `0x143A86BA8`, whose first qword is `0x143A87E08` - so **both checks
pass for this one window** [L]. (Had they not, every shelf answer would have been dropped silently
and the latch with it.)

## 1. Client -> server

`0x017F`, `u32 mode` first [L]:

| mode | builder | body | latch |
|---|---|---|---|
| 0 | `FUN_142CD4550` (Store Permit used) | `raw4 type, str title, u16 permit slot, u32 permit item` | no |
| 1 | `FUN_140D99620` "storeclose", after Yes to `0x1775` | - | yes |
| 2 | `FUN_1428B7600` (double-click on a sign) | `u32 user+0x114c` | no |
| 3 | `FUN_140D91F30` (the window closing), unless `[dlg+0x2c8]` | - | no |
| 5 | `FUN_140D91C40` | `str` | no |
| 6 | `FUN_140D93F50` = `vt+0x1d8`, after Yes to `0x1773` "Would you like to ban %s?" | `u16 seat` | yes |

`0x0180`, `u32 mode` first [L]:

| mode | builder | body |
|---|---|---|
| 0 | `FUN_140D99620` "storeopen", after `0x1774`; refused by the client with no rows (`0x177A`) | - |
| 1 | `FUN_140D9BEE0` | `u16 row` |
| 2 | `FUN_140D9BFE0` at `140d9c8e2` | `u8 tab, u16 slot, u16 bundles, u16 per bundle, u64 price` - a star stack as `1, 1` |
| 3 | `FUN_140D9D750` at `140d9e91e` | `u8 row, u16 bundles, u64 total, u32 hash` |
| 4 | `FUN_140D9EEE0` "collect" (`0x177C`) | - |

All five latch. Modes 0, 1, 2 are refused by the client unless the window's state `[dlg+0x2a8]`
(`FUN_140D94020`) is 1 - *"Not available in the current state."* (`0x177D`) [L].

## 2. Server -> client

**`0x0577` mode 2 opens the window** (`FUN_140D91330`), only when no store window is open [L]:

```text
u32 2, u32 result           nonzero -> FUN_140D91680's message, no window
u32 type                    3 or 4 build the window (FUN_140D984D0); 3 = store, 4 = hired merchant
u32 id                      -> ctor -> [dlg+0x298]
str title                   -> ctor -> [dlg+0x2a0]
raw4 state                  -> [dlg+0x2a8]   1 setup, 2 open
-- vt+0x170 = FUN_140D9A090
u32 owner id                -> [dlg+0x378]   == ctx+0x232c (own character id) makes it the owner's window
u32 owner account           -> [dlg+0x37c]   == ctx+0x2204 for "collect"
str owner name              -> [dlg+0x380]
u32 permit item             -> [dlg+0x388]   FUN_140417ED0 == 0xB draws a hired merchant's employee
u32 max rows                -> [dlg+0x38c]
-- vt+0x178 = FUN_140D9F4E0 -> FUN_140D923E0, the seats
u32 count, per seat FUN_14073A430: u32 id, u32, str name, u8 hasLook, [avatar look FUN_1402EE8D0]
   four seats (FUN_140D99CF0 sizes them); a count past four RETURNS EARLY and the shelf goes unread
-- FUN_140D9ABC0, the shelf
u16 count, per row FUN_140429CE0: u16, u16, u32 bundles, u32 per bundle, u64 price, u8 hasItem,
   [GW_ItemSlot, FUN_140303530]
```

The buy dialog (`FUN_140D9D750`) offers up to `bundles x per bundle`, accepts multiples of
`per bundle` (`0x173C` otherwise), and prices `count / per bundle x price` (`0x173B`); the row label
is `0x176F` *"%d for %s mesos"* from the per-bundle count and the price (`FUN_140D94D20`) [L]. The
two leading `u16`s of a row are not read on either path; the server sends the tab and 0 [D].

Other modes [L]:

| opcode | mode | body | effect |
|---|---|---|---|
| `0x0576` | 0 | the seats | `vt+0x1a8`; a seat whose id changed posts `SID_MAPLECHAT_USERENTER` |
| `0x0576` | 1 | `raw4 state` | `FUN_140D927F0` |
| `0x0576` | 2 | `u32 seat, u32, u32 id, str name, str text` | a chat line (`FUN_14073A8D0`, `FUN_140D928C0`) |
| `0x0576` | 3 | `u32 seat, raw4 reason` | own seat: close the window, release the latch; else "has left" |
| `0x0577` | 0 | `u32 code` | message |
| `0x0577` | 1 | - | close the window |
| `0x0577` | 3 | `u32 code` | 0 closes, else a message |
| `0x0577` | 4 | `raw4 state` | as `0x0576` mode 1 |
| `0x0577` | 6 | - | nothing; with modes 0..4 it ends by releasing the latch |
| `0x0578` | 0 | the shelf | redraw |
| `0x0578` | 1 | `u64, u64, u16 n, n x (str, u16, u32, u16, u64, u64)` | the sold log - not sent |
| `0x0579` | 0 / 3 | `u32 code` | room / store message (`FUN_140D91680` / `FUN_140DA0030`) |
| `0x0579` | 1 / 2 | `u8 has, [shelf]` | a listing / a purchase went through |

`0x0579` always releases the latch. A closing window shows *"The shop has been closed."* (`0x177B`)
for reasons 3, 5, 7, 8 (`FUN_140D98ED0`) [L]. The owner's window warns *"a 3% fee will be deducted
from sales"* - `0x1772` formatted with a literal 3 at `FUN_140D98830` - so the server takes 3% [L].

## 3. The sign: `0x0234`, and `0x0224` offset 455

`0x0234` is `FUN_142795A70`, the user-pool row after the Omok balloon's `0x0233`: `u32 charId`
(the dispatcher's), then `FUN_14073A7B0(user+0x1148, pkt)` - the same unconditional block `0x0224`
reads at 455 [L]:

```text
raw4 type    +0x1148   0 = no sign (FUN_141596F40 takes the old one down first)
u32  id      +0x114c   a double-click sends it back as 0x017F mode 2 (FUN_14276F7A0)
raw4 state   +0x1150   the click needs 2 (FUN_1429B64E0: cmp [rax+8], 2); the sign draws
                       canEnter for 2, cannotEnter otherwise (FUN_141595E60)
str  title
u32  permit  +0x1160   the art: UI/RoomSessionBalloon.img/backgrnd_PlayerShop/<permit>,
                       which has 5140001..5140004 (wz-dump)
```

**The reader has no branch**: all five fields are read whatever the type, so "no sign" is all five
zeroed - which is what `0x0224` already sent.

## 4. Hired merchants: the check, `0x0181` / `0x00A1` (2026-10-04, later)

The owner: *"I just tried double clicking the elf hired merchant and it doesn't do anything."*
`world-ch0.log` 20:47:15-19: nine `0x0181`, each `1000 70c04c00` - Cash slot 16, item 5030000.

* **`0x0181`** is `FUN_142CD4B70`: after its own checks (Free Market field, not near a portal or
  NPC, no window open) `u16 cash slot, u32 item`, and it keeps both at `ctx+0x415c` / `+0x4160` [L].
* Those two fields are read in one place in the whole `CWvsContext` region: `FUN_142D1CDF0`,
  `CWvsContext` case **`0x00A1`** (`research/msexe-gamestage-cases.txt`) [L]. `raw4 mode`; mode 0 then
  `raw4 code`: **0 calls `FUN_142CD4550(ctx, slot, item, 1)`** - the title prompt, then `0x017F`
  mode 0 with room type `3 + 1 = 4` (`142cd4550`: `local_508 = param_4 + 3`); 2 or 3 is `0x17C7`
  *"You already have an open Hired Merchant."*, else `0x178A` *"An unknown error has occurred."*.
  Modes 1 and 2 read a code and show the same messages. Every path ends `ctx+0x2330 = 0` [L].
* Type 4 opens the same window class. A `503xxxx` permit makes `FUN_140417ED0` return `0xB`
  (`503 -> '\v'`), so the window loads `UI/HiredShop.img/<permit>/employee` and does not draw seat
  0's avatar (`FUN_140D9F4E0`) [L].
* **The merchant on the map is a separate object**: the click path `FUN_1428B7600` asks the current
  field (`FUN_141892840`, a field-class check against `0x143A87EC8`) for an employee under the cursor
  (`FUN_1418921F0` = `field+0xd0` -> `FUN_140D90060`) and sends `0x017F` mode 2 with its id [L].

## 5. The employee: `0x0622` / `0x0623` (2026-10-04, later still)

The owner: *"We need to make it stay behind. Also it needs to be setup every 24 hours or it will be
automatically removed"* (the item's description says so).

`CField::OnPacket`'s default arm ends `lea eax,[r9-0x621] / cmp eax,4 / ja / lea rcx,[rsi+0xb8] /
call 0x140D8F740` (`141822116`): opcodes **`0x0621..0x0625`** go to `FUN_140D8F740(field+0xd0, opcode,
pkt)`, the same `field+0xd0` pool the click hit-tests [L]. It handles two:

* **`0x0622` EmployeeEnterField** - `FUN_140D8F7D0`, ignored unless the field is a market
  (`FUN_14182E530`); a record with the same id is removed first, so a resend is an update [L]:

  ```text
  u32  id      +0x58   FUN_140D90060 returns it to the click -> 0x017F mode 2
  u32  item    +0x5c   UI/HiredShop.img/%d/employee (FUN_140D8DF90), UI/HiredShop.img/%d (FUN_140D8E230)
  str  owner   +0x60
  str  title   +0x68   drawn in the balloon (FUN_140D8E230, 140d8e8a0)
  i16  x, i16 y        the layer's position (FUN_140D8D670)
  raw4 state   +0x70   2 draws /canEnter, else /cannotEnter
  ```

  Field order from the listing: `140d8f80a u32, 8f814 u32, 8f823 str, 8f830 str, 8f839 u16, 8f848 u16,
  8f8a8 raw4`. The string ids `UI/HiredShop.img/...` were found by `tools/clusterstrings.py` over
  `0x140D84000..0x140D90E90` - that is what named this pool.
* **`0x0623` EmployeeLeaveField** - `u32 id` [L].

`0x0621`, `0x0624`, `0x0625` read nothing.

**Server side**: a hired merchant is a `hired_merchants` row (owner, title, item, channel, map,
position, `opened_at`) beside its `shop_escrow` shelf. It stands without its owner once opened, is
restored on its channel after a restart, pays the owner's wallet directly, goes into maintenance when
its owner double-clicks it, and closes 24 hours after setup - the shelf back in the owner's bag at
once if they are on that channel, else at their next login.

## 6. The first run: Open Store froze the owner's UI; sold-out rows (2026-10-04, evening)

`research/fixtures/store-open-left-ui-locked-sold-out-world.log`. The store opened, listed, opened for
business, took a visitor, chat and two purchases - and the owner: *"Once everything is sold out, the
player also loses all ability to close the shop. The button becomes not clickable and the UI is
completely unresponsive."*

**The fixture says the owner's client sent nothing after Open Store** (21:10:57) - no Close Store,
nothing - while both `0x0578` updates it was sent dispatched cleanly in the hook log. So the click was
refused on the client, not left waiting on the server. The cause, from the listing [L]:

* "Open Store" (`FUN_140D99620`), a purchase (`FUN_140D9D750`), "collect" (`FUN_140D9EEE0`) and
  `FUN_140D9D580` each call **`FUN_142AA27F0(4)`** after sending: it inserts 4 into an id set at
  `[0x143AC0038]+0x438` (`FUN_142C16850`) that blocks the UI while non-empty.
* The only removal the store reaches is **`FUN_142AA2810(4)`**, called by the `0x0579` handler
  `FUN_140D94BB0` (and by `FUN_142D1CDF0`, the hired-merchant check) [L].
* Open Store was answered with `0x0577` mode 4 alone, so **the owner's UI was locked from 21:10:57**
  and the sell-out was only when it was noticed. Purchases were answered by `0x0579`, which is why the
  visitor never froze.

Open Store is now answered `0x0577` mode 4 **and** `0x0579` mode 0 code 0 - which `FUN_140D9FE50`
reads as "no message" (`FUN_140D91680` case 0), releasing the latch and redrawing.

**Sold-out rows** (*"when an item is sold, it should remain as a row ... to display the price it was
sold at"*): the row widget `FUN_140D94D20` draws the `soldout` canvas over a row whose bundle count
(`+0x11b0`) is below 1, still with its price, and `FUN_140D9CCB0` will not select it [L]. So a line
that sells out stays at 0 bundles - a record, never sold from and never given back. **When every line
is sold out the store closes** (the owner: *"once the shop is out of items, the store should
automatically close"*), everybody's window with "The shop has been closed.".

## 7. Where a store may stand

Before sending a create, `FUN_142CD4550` (and the hired-merchant check `FUN_142CD4B70`) refuse [L]:

| check | against | message |
|---|---|---|
| `FUN_1429B5EF0(userPool, pos)` | every user with an Omok/Match Cards balloon (`+0x1118`, area `(-50,-120,50,10)` at `0x1432874c8`) or a **store sign** (`+0x1148`, area `(-60,-120,60,10)` at `0x143299f08`) | `0x1776` "You can't open a store here" |
| portal rectangles (`DAT_143AC3740`, `FUN_141EEF900(.., 0x14)`) | | `0x1778` "... too close to a portal." |
| town-portal pool `[0x143ACC528]` | mystic doors | `0x1778` |
| NPC pool `[0x143AA84F8]` | every NPC's rectangle | `0x1779` "... too close to a NPC." |
| reactor pool `[0x143ACF070]` (opcodes `0x478..0x48C`) | every reactor's rectangle | `0x1776` |

**Hired merchants are in none of these** - the employee pool is not walked - and none of it is a
server check. So the server refuses a create within `net::playershop::STORE_SPACING_X` (120) /
`_Y` (130) px of any store or merchant on the map - exactly where two `(-60,-120,60,10)` areas
overlap - with `0x0577` mode 2 code `0x1C`, which the client words as `0x1776`.

## What is still open

* Nothing has been on a screen. Plan step 49 walks create, list, open, visit, buy, chat, ban, close.
* The member entry's second `u32` and a row's two leading `u16`s are sent as 0 / the tab; nothing
  read on the paths above uses them.
* Fredrick, the unsold-items NPC, is not involved: an expired merchant's shelf goes straight back to
  the owner's bag. An owner on another channel at expiry gets it at their next login.
* One merchant per character, checked from the row on every channel ("You already have an open
  Hired Merchant."); a player store is refused while one stands, since both use the same shelf.
* The sold log (`0x0578` mode 1) is not sent; the owner's "sold" tab will be empty.
