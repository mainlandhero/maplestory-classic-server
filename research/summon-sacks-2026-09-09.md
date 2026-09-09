# Summoning sacks: `0x0111`, not `0x010E` — and the client already refuses them on some maps

2026-09-09. Markers: **[L]** read out of this client's image, its WZ or an archived run,
**[D]** derived from two or more [L], **[I]** inferred, including anything from the v214 tree
at `C:\Users\user\Desktop\ModernMapleSource`, which is a different game version and scored
**1 of 8** against a held-out control. No Ghidra was used; everything below is `tools/`.

---

## 0. Answer up front

| | |
|---|---|
| how many sacks this client has | **8**, `2100000`..`2100007` **[L]** |
| where the mob list lives | `Item/Consume/Consume_000.wz :: 0210.img/<id>/mob`, a **sibling of `info`, not inside it** **[L]** |
| what says "how many" | **the number of `mob` entries naming that template.** `2100007` lists `700005` twice **[L]** |
| what the client sends | **`0x0111`, 10 bytes: `u32 tick, u16 slot, u32 itemId`** **[L]** |
| what it does **not** send | `0x010E`. `FUN_1404169b0` is the predicate that gates `0x010E` and the `0210` prefix is **not in it** **[L]** |
| does the server handle `0x0111` today | **No.** `session::mod`'s dispatcher ends `_ => return Vec::new()`, so today a sack use is answered with **silence** and the `ctx+0x2330` latch stays set for the rest of the session **[L]** |
| client-side map gating | **yes** — `fieldLimit` **bit 2** (`& 0x4`), plus the field's per-item allow list. Refused **locally**, no packet **[L]** |
| does the client read the sack's own `mob` node | **no** — three searches for the property name find three sites and all three are elsewhere (§5) **[D]** |
| does this reduce to "reuse the existing mob spawn plus an item table" | **Yes on the spawn side; no on the inbound side** — `0x0111` is a new opcode this server has never decoded, and it is 10 bytes |

**Nothing here has been on a screen.** No archived run contains a client `0x0111` (§7), and
nobody has ever held a `0210` item on this server. Everything below is static plus WZ.

---

## 1. The eight sacks, and where the data is

`python tools/dump_summon_sacks.py` → `gm-handbook/summonsacks.txt`. **[L]**

```
# itemId, slotMax, price, mobCount, mobs, name
2100000, 100, 1, 1, 700004:100, Black Sack
2100001, 100, 1, 1, 800018:100, GM Black Sack: Mano Level 20
2100002, 100, 1, 1, 800019:100, GM Black Sack: King Slime Level 32
2100003, 100, 1, 1, 800020:100, GM Black Sack: Mushmom Level 40
2100004, 100, 1, 1, 800021:100, GM Black Sack: Zombie Mushmom Level 50
2100005, 100, 1, 1, 800022:100, GM Black Sack: Rotten Mushmom Level 60
2100006, 100, 1, 1, 800023:100, GM Black Sack: Jr. Balrog Level 80
2100007,  10, 1, 2, 700005:100;700005:100, Summoning New-Type Balrog
```

The node, verbatim from `wz-dump cat`:

```
02100007
  info/ slotMax 10   type 1   price 1   icon   iconRaw
  mob/
    0/ id 700005   prob 100
    1/ id 700005   prob 100
```

Three things about that shape:

* **`mob` is a sibling of `info`.** The brief said the mob list lives "in the consumable
  item's own info block"; in this client it does not, it sits beside it. **[L]**
* **The count is the repetition.** `2100007` means *two* Crimson Balrogs, and nothing else in
  the node says two. A reader that keys a dict by template id loses that and spawns one. **[L]**
* **`prob` is 100 on all 9 entries in all 8 items.** So nothing in this client exercises a
  probability roll, and a server that ignores `prob` is indistinguishable from one that
  honours it — on *this* data. The column is emitted anyway. **[L]**

All eight templates exist in `Mob_000.wz` and in `gm-handbook/mobtemplates.txt` **[L]**:
`700004` (Mano, 7 420 HP, lvl 20), `800018` (Mano), `800019` (King Slime), `800020`
(Mushmom), `800021` (Zombie Mushmom), `800022` (Rotten Mushmom), `800023` (Jr. Balrog),
`700005` (Crimson Balrog, **741 240 HP, lvl 100**). Every one has `boss = 1`.

### 1.1 The family was enumerated, not assumed

`tools/dump_summon_sacks.py` walks **1 864 images across 18 archives** — every `Item/*` and
every `Character/*` equip archive — and emits any node with a `mob` child. It returns exactly
8, all in `Consume/0210.img`. So "0210 is the summoning-sack prefix" is a *result* here, not
an input. **[L]**

The dumper refuses rather than writing short, and both refusals were exercised: patched to
lose one `Mob/%07d.img` it exits **1** and writes nothing; patched to match no `mob` node it
exits **1** and writes nothing; unpatched it exits **0**. **[L]** The template check is not
tidiness — `research/mob-spawn.md`: `MobEnterField`'s HP is divided by the template's max HP
at `141c50502` by an `IDIV` **with no zero guard**.

---

## 2. `0x010E` is the wrong opcode, and here is the function that says so

`FUN_142cc8ab0` is the **only** builder of `0x010E` in the image
(`research/msexe-send-opcodes.txt`, one call site at `142cca557`) **[L]**. Its four callers all
guard the call. Three of the four guard it with the same predicate:

```asm
1417857a1  call 0x1404169b0   ; in FUN_141784fa0, the inventory-window virtual
14178766d  call 0x1404169b0   ; in FUN_141787410
1428b0099  call 0x1404169b0   ; in FUN_1428af6d0, twice - also at 1428b01f4
```

The fourth, `FUN_1428b10d0` at `1428b12c8`, is **not** gated on it — it is the "use the slot
holding item id X" helper and checks only a map lookup. **[L]**

`FUN_1404169b0(itemId)` is 185 bytes and is a pure range list **[L]**:

```
returns 1 for  2000000..2009999   2010000..2019999   2020000..2029999   2050000..2059999
               2210000..2219999   2360000..2369999   2380000..2389999   2450000..2459999
               2800000..2809999   2900000..2909999
returns 0 for  FUN_140416d60(id) != 0,  and 2002005..2002010,  and everything else
```

**`2100000..2109999` is not in that list**, so `FUN_1404169b0(2100000) == 0` and no path
that is gated on it can build a `0x010E` for a sack. **[L]**

Positive control for the reading: `2000000` (Red Potion) *is* in the first range, and the
archive contains real client `0x010E` bodies naming `0x001e8480` = 2 000 000,
`0x001eab90` = 2 010 000 and `0x001e8483` = 2 000 003 — all inside those ranges and none
outside them. **[L]**

### 2.1 The `0x010E` body, re-read, and one open field closed

`142cca54b`..`142cca5a8`, in order **[L]**:

```
1406ed520(pkt, 0x010E)          opcode
1406ed9d0(pkt, FUN_1429e3ef0()) u32  tick  (GetTickCount)
1406ed940(pkt, [rsp+0x70].w)    u16  slot
1406ed9d0(pkt, r13d)            u32  itemId
1406ed9d0(pkt, 1)               u32  IMMEDIATE 1
1415d01c0(pkt)                  send
```

`crates/net/src/useitem.rs` calls the last field `tail` and says *"its meaning is not
established"*. It is **a literal `1` in the client's own code** — `mov edx, 1` — so it carries
no information and the doc block can say so. **[L]**

---

## 3. The item-use dispatcher, enumerated, with the opcode each branch sends

The Use-tab double-click runs a chain of item-class tests. The one that carries the `0210`
branch is `FUN_141784fa0` (reached only through a vtable slot at `0x14336e930`, so it is an
inventory-window virtual **[L]**; that it is *the* double-click handler is **[I]**, from
`[rdi+0x34]` being a UI slot record). Every branch, in order, with the outbound opcode taken
from `research/msexe-send-opcodes.txt` **[L]**:

| test | handler | opcode |
|---|---|---|
| `FUN_140417c00` → 2740000..2740999 | `FUN_142cc88b0` | `0x012C` |
| `FUN_1404169b0` → the ten ranges in §2 | `FUN_142cc8ab0` | **`0x010E`** |
| 2190000..2199999 | `FUN_142d4e120` (takes a string) | — |
| 2030000..2039999 (**return scrolls**) | `FUN_142d4be40` | **`0x0123`** |
| **2100000..2109999** | **`FUN_142ccb0f0`** | **`0x0111`** |
| … further branches below `1417859e3` | | |

The quickslot/keymap dispatcher `FUN_1428af6d0` (item id read as `[r13+1]`, i.e. out of a
`{u8 type; u32 id}` record — **[I]** that this is the quickslot) has the *same* handler set
but a **narrower** `0x0111` branch: only `2109000..2109999`, `2103000..2103999` and the single
id `2100067`. A `2100000..2100007` id reaching that dispatcher falls all the way through to
`1428b0441`, which calls a predicate, discards the result and returns — **nothing is sent**.
**[L]** So a sack bound to a quickslot may well do nothing at all; a sack double-clicked in
the bag sends `0x0111`.

### 3.1 A by-product that is not this brief's business but is load-bearing

**Return scrolls (`2030000..2039999`) route to `0x0123`, not `0x010E`.** **[L]** Both
dispatchers agree, and the archive's 10 `0x010E` bodies across 9 runs name only `0200`/`0201`
items — no `0203` has ever produced one. `crates/world/src/session/consume.rs` implements
return scrolls on the `0x010E` path (`Session::use_return_scroll`), with tests that exercise
the handler directly rather than the wire. If that reading is right, the feature is
`CLAUDE.md`'s *"Built is not wired"* and has never been on a screen. **I have not opened
`FUN_142d4be40`'s body**, so what `0x0123` carries is unread; I did not touch `crates/`.
Cheapest settle: `!item 2030000 1`, double-click, and grep `world.log` for `0x0123` — it is
one action inside a run that is happening anyway.

---

## 4. `0x0111` — the summoning-sack request, `FUN_142ccb0f0`, 601 bytes

### 4.1 The proof of identity is a string, not a name

Before the send, unless the item is `2109000..2109999` / `2103000..2103999` / `2100067`, the
builder raises a confirmation dialog with string id **`0x00BD`**, which
`tools/dump_stringids.py --id 189` decrypts to **[L]**:

> `Opening this item may hinder \r\nyour process of gaining EXP. \r\n\r\nWill you summon the monster?`

A summoning sack is exactly the excluded-from-the-exclusion case, so **using one raises that
dialog**, and `FUN_142a269c0` must return `6` (OK) or the function returns having sent
nothing. **[L]**

### 4.2 The body — 10 bytes

`142ccb2aa`..`142ccb2e9`, in order **[L]**:

```
1406ed520(pkt, 0x0111)          opcode
1406ed9d0(pkt, FUN_1429e3ef0()) u32  tick    offset 0   (GetTickCount)
1406ed940(pkt, bp)              u16  slot    offset 4   (Use-tab slot, 1-based)
1406ed9d0(pkt, edi)             u32  itemId  offset 6
1415d01c0(pkt)                  send                      body length 10
```

`1406ed9d0` = u32 and `1406ed940` = u16 are the same two encoders `0x010E` uses, and that
mapping is corroborated by a *measured* `0x010E` capture whose 14 bytes split exactly
`u32/u16/u32/u32` around a known item id and a known slot. **[D]**

**There is no trailing `u32`.** `0x010E` has one and `0x0111` does not. A parser copied from
`useitem.rs` with its `len() < 14` guard would reject every real `0x0111`.

### 4.3 What it does after the send — the latch

```asm
142ccb2ee  mov dword [rbx+0x2330], 1        ; THE LATCH
142ccb2f8  call 1429e3ef0 ; mov [rbx+0x2334], eax
```

`ctx+0x2330` is the one-request-outstanding latch `research/ap-allocation.md` decoded: the
client **will not build another latched request until the server answers**, and an unanswered
one kills item use, AP allocation and the cash shop for the rest of the session. `0x007C`
`StatChanged` byte 0 (`bExclRequestSent`) clears it; so does a `0x0070` `InventoryOperation`.
**[L]**, and both are already shipped shapes in this server.

### 4.4 The preconditions, and they are the same ones `0x010E` has

`0x010E`'s builder calls `FUN_142cc42d0(ctx, 0xc8, 0)`; `0x0111`'s inlines the identical four
tests with the same 200 ms. **[L]**

| test | meaning |
|---|---|
| `ctx+0x2338 == 0` | no other request outstanding |
| `ctx+0x2330 == 0` | the latch is clear |
| `ctx+0x2358` non-null and an obfuscated pair at `+0x5b`/`+0x63` agrees (`FUN_1401ba9d0`, `xor 0xbaadf00d`) | an anti-tamper checksum. Not decoded further; the shipped pet-food path (`0x010F`) passes the same test |
| `now - ctx+0x2334 >= 200` | a 200 ms throttle |

The inventory window applies its own `FUN_142cc42d0(ctx, 0x1f4, 0)` first, so the effective
gap between two sack uses is **500 ms**. **[L]**

---

## 5. The client never reads the sack's `mob` list

Three searches of the whole image for the property name, in the three forms the client uses
for WZ property names, and **every hit is accounted for** **[L]**:

| form | hits | where |
|---|---|---|
| inline wide immediate `movabs rcx, 0x62006f006d` | 1, at `0x14240aaf7` | in `FUN_14240a480`, whose caller chain's strings are `UI/UIWindow4.img/mazeMap/backgrnd`, `area`, `potal`, `icon`, `number` — the **minimap/mazeMap** window |
| wide `.rdata` literal `L"mob"` at `0x1432aa368` | 1 `lea`, at `0x14240c678` | the same mazeMap code |
| ASCII `"mob"` at `0x143284060` | 1 `lea`, at `0x1403b06bf` | in a table loader whose neighbouring literals are `completeCount`, `mobName`, `setItemName` — the **Quest.wz check** parser (kill requirements) |

None is on the item-use path, and `FUN_142ccb0f0` reads no WZ at all. **[D]: the client does
not know what a sack contains; the whole mob list is server policy.**

Instrument check, because this is a negative: the same scan located the quest-check reader and
the minimap reader, i.e. it *can* find a `mob` reader when one exists; and the sibling scan
that found these also independently located the already-known read at `0x14038dec8`. What it
cannot see is a property name assembled at runtime from pieces — no such construction was
found near the item-use path, but that is the named blind spot.

---

## 6. Client-side gating — there is some, and it is a `fieldLimit` bit

Two field tests sit in front of the send, at `142ccb16d` and `142ccb17e` **[L]**:

```asm
14182e660:  rcx = field->[0xa8] + 0xd0 ; FUN_1403326d0(rcx) ; shr eax,2  ; and eax,1   ; bit 2
14182ff20:  walk field->[0xa8]->[0x16e0], a linked list; return 1 if a node's first dword == itemId
```

Read together:

> **If the field's `fieldLimit` has bit 2 (`0x4`) set, a summoning sack is refused unless its
> item id is in the field's own allow list.** The refusal draws string `0x00B8` —
> *"You can't use that on this map."* — and **sends nothing**. **[L]**

Bit 2 is `SUMMON_LIMIT` in the v214 tree's `FieldLimit` enum — **[I]**, but the placement is
not: the same builder's `0x010E` twin uses **bit 10** instead, so the two item families are
gated by two different bits of the same word, which is what a per-capability limit mask looks
like. **[D]**

Consequences for the server:

* **The server cannot enforce this and does not need to.** A map that refuses produces no
  packet at all, so the server cannot distinguish "the map refused" from "nobody clicked".
* `fieldLimit` is **not** in `gm-handbook/` today — no dumper reads it. If the server ever
  wants its own copy (for a GM summon that bypasses the bag, say), that is a new column on the
  map table.
* Two more state gates sit in the same function and are not item-specific: `FUN_142d2e710`
  on `ctx+0x2210` and `ctx+0x2294 != 0` each **skip** the map check, i.e. some state makes the
  client stop asking the map at all. Not decoded. **[L]** that they exist, nothing more.

**No cap on concurrent summoned mobs was found on the client side.** `research/mob-spawn.md`
enumerates every branch of `FUN_141d33630` (`0x03C6`'s handler) and none of them counts the
pool. **[D]**, with the honest caveat that "I read a decode that enumerated the branches" is
weaker than having enumerated them again. Any cap is server policy;
`Config::mob_limit` today applies only to `Fields::seed`.

---

## 7. Nothing has ever been measured on a screen

`0x0111` appears in the archive **once**, in
`research/fixtures/sweep-0024-01c3-reply-0171.log` line 281 as
`[8484] >>> opcode 0x0111 +32B` — a **server-side sweep on the login port**, not a client
send. There is no `<- 0x0111` anywhere. **[L]**

That is expected rather than informative: no character on this server has ever held a `0210`
item. It is a control that has never been run, not a negative result. The unknown-opcode log
line already prints the full body (`<- 0x010E UNKNOWN, 14 byte body …`), so the moment a sack
is used the wire format above is checkable for free.

---

## 8. What the server must do

An implementer can follow this in order. Every packet named already exists in `crates/net`
except the inbound parse in step 1.

1. **Decode inbound `0x0111`.** New. Body is **10 bytes** and short bodies come off a socket,
   so parse to `Option`, never panic:

   | offset | size | field | marker |
   |---|---|---|---|
   | 0 | `u32` | `tick` (GetTickCount, informational) | **[L]** `142ccb2ba` |
   | 4 | `u16` | `slot` — Use-tab slot, 1-based | **[L]** `142ccb2cb` |
   | 6 | `u32` | `itemId` | **[L]** `142ccb2d8` |

   Total **10**. **Do not reuse `net::useitem::parse_use_item`** — its `len() < 14` guard
   rejects every real `0x0111`.

2. **Answer on every path, refusals included.** The client sets `ctx+0x2330 = 1` on send
   (**[L]** `142ccb2ee`) and builds no further latched request until something with
   `bExclRequestSent` arrives. Use exactly what `Session::use_refused` already sends: an empty
   `0x007C` `StatChanged` (`net::stats::StatChange::default().build()`, byte 0 = 1). Silence
   here costs not one sack but every later item use, AP click and cash-shop action in the
   session. **[L]**, `research/ap-allocation.md`.

3. **Check the slot, don't trust it.** Same as `on_use_item`: `store::InventoryType::Use`,
   `bag_items`, the row's `item_id` must equal the request's. The server holds the bag.

4. **Look the item up in `gm-handbook/summonsacks.txt`.** Unknown id → refuse (step 2) and
   keep the item. A row gives an ordered list of `(templateId, prob)`; **spawn one mob per
   entry**, so a duplicated template means two mobs. **[L]** §1.

5. **Resolve the spawn position.** `Session::last_position` is the player's last reported
   point and can be mid-jump; `Config::footholds.landing(map, x, y)` returns
   `Landing { x, y, foothold }` — the same call the drop path uses. **No `last_position`, or
   no landing → refuse and keep the item**, for the reason `ground.rs` already gives: an object
   placed in the wrong place looks identical on screen to nothing having happened. **[D]**

6. **Allocate object ids that cannot collide.** Spawn-point ids are `2000 + index` per map
   (`config.rs:656`) and drop ids start at `20_000_000`. A summoned range starting at
   `1_000_000` is clear of both; pass each through `net::mob::next_usable_object_id`, which
   rejects `0` and multiples of `178`. A repeated live id makes `FUN_141d33630` take its
   "already in the pool" branch and **stop reading after 31 bytes**, desynchronising the rest
   of the stream. **[L]**, `research/mob-spawn.md` §2.

7. **Insert into the live pool, then send the spawn.** `crates/world/src/fields.rs` has no
   ad-hoc spawn today — `due_respawns` only promotes ids that already exist in
   `config.mobs[map]`, so a new `Fields` method is needed that inserts a `LiveMob` directly.
   Without it the mob is drawn but cannot be hurt (`Fields::hurt` returns `Hurt::Alive(0)` for
   an id it does not hold) and is invisible to anyone who walks in afterwards
   (`Fields::mobs_on`). **[L]** from the code.

8. **Send `0x03C6` `MobEnterField` per mob**, built by `net::mob::mob_enter_field`, exactly as
   `Session::spawn_due_mobs` does — **137 bytes** plus the forced-stat block:
   * `object_id` from step 6, `template_id` from step 4;
   * `x`, `y`, `fh` = `home_fh` from the `Landing` in step 5;
   * `hp` = the template's `maxHP` from `gm-handbook/mobtemplates.txt`. Zero is a structurally
     valid packet that draws a 0 % bar; a missing template is an `IDIV` by zero. **[L]**
   * `appear_type = net::mob::APPEAR_SPAWNING` (**-2**);
   * `move_action` left at `MOVE_ACTION_MIN_SAFE` (2) — `0` is the byte that killed the client
     on 2026-08-19;
   * `forced_stat = self.forced_stat_for(template)`, or the mob's contact damage floors at 1.

   **Do not reach for the summon "poof" effect.** `Effect/Summon.img/%d` is created only on
   the `appear_type >= 0` default branch (`141c52a9f`), and that same branch sets
   `mob+0x504 = 1`, which `research/mob-target-gates.md` shows makes the mob **permanently
   unhittable**. `-2` gets the fade-in and a hittable mob; you cannot have both. **[L]**

9. **Send `0x03D2` `MobChangeController` for each mob this connection claimed**, and
   **publish the `0x03C6` map-wide** — `crate::mobshare::audience_for` and
   `Bus::publish`, exactly the split `spawn_due_mobs` documents. The grant must **not** be
   map-wide: two controllers roll two different wander paths and the screens diverge. **[L]**

10. **Consume one from the stack** and tell the client with `0x0070`
    `InventoryOperation` — `Session::stack_change_replies(inv, slot, left)`, which already
    picks mode 3 for the last one. Its byte 0 also carries `bExclRequestSent`, so this is the
    belt to step 2's braces. **[L]**

11. **Every effect hangs off the transition.** The removal, the spawns and the replies are one
    branch: `CLAUDE.md`'s Heena rule. A refusal consumes nothing and spawns nothing.

**No new outbound packet is needed, and no effect or animation packet is missing.**
`FUN_142ccb0f0` does nothing locally after the send except set the latch and two action
timers (**[L]** `142ccb2ee`..`142ccb321`), so everything the player sees is server-sent, and
`0x03C6` with `appear_type = -2` is the arrival animation. **[D]**

### So, in one sentence

**On the outbound side this is exactly "reuse the existing mob spawn plus an item table" —
`0x03C6` and `0x0070` are already built and proven — but it is not free, because the inbound
half is a new 10-byte opcode `0x0111` that this server has never decoded and `fields.rs` has
no ad-hoc spawn to insert into.**

---

## 9. What I could not determine, and the cheapest thing that would settle each

| open | cheapest settle |
|---|---|
| **That a Use-tab double-click really takes `FUN_141784fa0`** (the dispatcher with the `2100000..2109999` → `0x0111` branch) rather than the quickslot one, which drops sacks on the floor. **[I]** today. | `!item 2100000 1` on a GM character, double-click it, grep `world.log` for `0x0111`. The unknown-opcode line already prints the whole body, so the same action also confirms the 10-byte layout in §4.2. One action inside a run that is happening anyway. If **nothing** arrives, the quickslot dispatcher is the live one and the feature needs a different entry point. |
| **The wire layout has never been seen.** §4.2 is read off the builder, not off a capture. | The same grep. `<- 0x0111 UNKNOWN, 10 byte body <tick><slot><itemId>` proves all three fields at once against a slot and an id the server itself wrote. |
| **Whether the confirmation dialog (§4.1) actually appears**, and whether `FUN_142a269c0` returning anything but 6 aborts silently. | The same run: the owner reports whether they saw *"Will you summon the monster?"*. If they click Cancel and no `0x0111` follows, the dialog is confirmed as the gate. |
| **Which maps set `fieldLimit` bit 2.** No dumper reads `fieldLimit`. | A `wz-dump cat` of a few `Map/Map<N>/<id>.img` `info` nodes, or a new column on `dump_portals.py`. Free, no launch. Only needed if a summon is later expected to work in a town. |
| **`FUN_1401ba9d0`'s anti-tamper check** on `ctx+0x2358` (§4.4). | Nothing, probably. The shipped pet-food path `0x010F` passes the same test on this client, so it is not a summon-specific obstacle. |
| **Whether a boss template needs anything extra** — all 8 have `boss = 1`, and this server has never spawned one. Crimson Balrog is 741 240 HP and level 100. | Start with `2100001` (Mano, 7 420 HP, level 20), not `2100007`. If the boss HP bar or a boss-specific packet is missing, it will show as a missing UI element rather than a crash. |
| **Any concurrent-summon cap.** §6 finds none on the client, but from a re-read of an existing decode rather than a fresh enumeration. | Spawn `2100007` (two mobs) first, then several sacks in a row, and watch. Server-side policy costs one constant either way. |
| **What `0x0123` carries**, and therefore whether the shipped return-scroll feature is wired to the wrong opcode (§3.1). | `!item 2030000 1`, double-click, grep `world.log` for `0x0123` — the same run, the same cost. This is out of this brief's scope but it is the single cheapest thing on this list relative to what it would be worth. |
