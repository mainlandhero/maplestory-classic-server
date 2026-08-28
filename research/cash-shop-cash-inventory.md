# The purchase reply is `0x05AE` sub-op `0x0C`, and the success message is the client's own

2026-08-27. **Static only — no client run was spent on this.** It rests on the owner's 2026-08-27
observation (the `0x19` purchase landed in the Item Inventory, silently) and on the image and
the client's own WZ data for everything else.

Tags: **[L]** read out of the image, a log or the WZ; **[D]** derived from something read;
**[I]** inferred.

This file answers three questions and corrects the two files it continues.
`research/cash-shop-buy-done.md` recommended `0x19` because it was *"the only silent arm that
clears both latches and hands over an item"*. That sentence is true and the recommendation was
still wrong, because the question it answered was the wrong one: **a purchase is not supposed
to clear the latches at all.** `0x05AD` clears them (§4.2), and the arm that carries a bought
item is the one that fills the **locker** and leaves the latches alone.

---

## 0. The short version

**Send two packets, in this order.** [D], from [L] parts.

```text
1)  0x05AE   u8  0x0C                    <- the item into the CASH INVENTORY panel
             <71-byte cash-item record>     see section 6 for the field list
             u32 0                          non-zero => 8 more bytes are read and DISCARDED
             u8  0                          read and discarded, but it must be present

2)  0x05AD   u32 nxCredit                <- the DEBITED balance. Must come SECOND.
             u32 maplePoints                All three must be >= 0.
             u32 0
```

That is the whole purchase. What the client then does, by itself:

* `0x0C` allocates the record, inserts it into the locker map at `[stage+0x150]`, and repaints
  the **Cash Inventory** panel (`[stage+0xc8]`). **[L]**
* `0x0C` removes the commodity from the cart (`FUN_140D7A610`) and repaints the cart window. **[L]**
* `0x05AD` clears `[stage+0x74]` **unconditionally** (`0x140D736DC`), sees `[stage+0x120] == 1`,
  zeroes it, and calls the buy builder `FUN_140D785F0` back. **[L]**
* The buy builder finds the cart index `[stage+0x164]` exhausted, so it sends nothing and takes
  its completion path, which puts **string 590, "You have successfully made the purchase."** on
  screen. **[L]**

So the missing success message is not a packet we failed to send — it is a message the client
raises itself, at the end of the cart, and it never ran because the buy was never completed the
way the buy builder expects. §4 has the whole chain.

**`0x19` was the wrong arm.** It is the reply to `0x03E1` sub-op `0x0A`, *"move a locker item
into inventory slot N"*, and it does exactly that: `FUN_1402E4C20(character, tab, nPOS, &item)`
puts the item in the character's bag — the **Item Inventory** panel. That is not a bug in the
implementation, it is the packet's meaning. `research/cash-shop-buy-done.md` §7 said so in its
own "what this file does not establish"; the recommendation was made anyway.

**`bEffect` is not the sound.** A non-zero `bEffect` on `0x19` calls
`FUN_142A37120(0x3EE, 0, -1, ...)`, whose resource path is `UI/CashShop.img/CSPopup/YesNo/11`
— and `CashShop.img` **does not exist in this client's UI archive** (§5). Leave it 0.

---

## 1. Instruments, and the controls run before any negative below was believed

Everything was run **with the repo as the working directory** (`python tools/...`), because
`CLAUDE.md`'s scratchpad-shadowing section applies. `tools/reads.py` resolved to
`C:\MapleCW\tools\reads.py` (printed, not assumed) and its `PRIM` table has
**11** entries including the `f64` reader `0x1406e8fb0`.

| instrument | control | result |
|---|---|---|
| `tools/reads.py` | `0x140304100 2` must read via `FUN_1403035a0` **and** `FUN_140303b40` **and** directly | passed — `0x14030411a`, `0x140304126`, then `0x140304138 raw` |
| `tools/callers.py` | `0x1402fa9a0` must give 96 sites in 15 functions | passed — 96 / 15 |
| `tools/rangescan.py` | disp `0x43` over `0x140d70000..0x140d90000` must return exactly the one known cash-item read `0x140D755B1` | passed — 1 site, that one |
| `tools/dump_stringids.py` | id 696 must be *"The cash item has been deleted."* | passed, exactly |
| `wz-dump` (prebuilt `target/release/wz-dump.exe`; **no `cargo` was run**) | `CashShopUI.img` must be found in **both** `UI_000.wz` and `_Canvas/_Canvas_000.wz` | passed — present in both (67 and 64 images listed) |

Ghidra was not used; another agent holds the single-process lock.

**One instrument in `cash-shop-buy-done.md` §5.3 was searching the wrong shape, and this file
re-runs it correctly.** That pass scanned for **object-relative** displacements (`0x47`, `0x53`,
`0x3a`, `0x4f`) and found nothing. But the client reaches the record through a pointer to
`obj+0x20` — proven by its own quoted line, `0x140D755B1 cmp byte [rdi+0x43], al`, where `rdi`
is `cashItem+0x20` and `0x43` is a **record**-relative displacement. A scan for `0x53` cannot
see a read written as `[rdi+0x33]`. §6 re-runs both shapes.

---

## 2. The two panels, named from the client's own data

The owner: *"The shop window has two panels down its left side, Cash Inventory and Item Inventory."*

`UI/CashShopUI.img`'s top-level nodes are, verbatim: **[L]**

```text
backgrnd  cameraPos  context  preview  cashLocker  itemInven  saleEntry
checkOut  slotEntity  sendGift  receiveGift  packageDetail  tooltipPackage  BeautyPreview
```

`cashLocker` and `itemInven` are the owner's two panels. The stage holds one pointer to each:

| field | panel | evidence |
|---|---|---|
| `[stage+0xc8]` | **Cash Inventory** (`cashLocker`) | `FUN_140D75850`, the locker-map insert, repaints it at `0x140D7598B` after every insert (null-checked). `FUN_1410B6CA0` searches its list `[ui+0x268..0x270]` by (SN, itemId). `FUN_140D74E10`, the `0x0A` builder, refuses if it is null. **[L]** |
| `[stage+0xd8]` | **Item Inventory** (`itemInven`) | `0x19` calls `FUN_1410D0F00([stage+0xd8], invType != 5)` then `FUN_1410D0FD0([stage+0xd8])` right after `FUN_1402E4C20` puts the item in the character's bag. `0x0B`'s gesture — double-click a cash item in the character's inventory — is handled in the sibling `0x1410Cxxxx` class. **[L]** for the calls, **[D]** for the naming |

Two more facts pin the identification harder than the offsets do:

* **The client's own confirmation dialog says where a purchase goes.** String **570**:
  *"Are you sure you want to make this purchase?\r\nYou can find purchased items in your **Cash
  Inventory**."* — fetched by `FUN_140D76090` at `0x140D761DF`, i.e. the pre-buy confirm. **[L]**
* **The panel's row widget is built in `.rdata` next to the panel's own name.**
  `FUN_1410B5550` (the repaint that `FUN_1410B5540`'s dirty flag arms) walks the red-black tree
  at `stage+0x150`, allocates a `0x11C0`-byte row per node, installs a vtable at
  **`0x14337CB68`**, and stores the cash-item pointer at `row+0x11B8` (`0x1410B5790`). The
  string `UI/CashShopUI.img/cashLocker` is at **`0x14337CB28`**, 0x40 bytes earlier in the same
  `.rdata` neighbourhood. **[L]**

`row+0x11B8` is the same field `cash-shop-actions.md` §9 already named as "the selected item's
serial" — it is the selected item's **object pointer**.

---

## 3. Which packet fills the Cash Inventory panel

### 3.1 The column `cash-shop-buy-done.md` §3 did not have

That table asked each arm three questions: does it clear `+0x74`, does it clear `+0x120`, does
it hand over an item. The fourth question is the one that matters here: **does it insert into
the locker map `[stage+0x150]`**, which is what `FUN_140D75850` does and what the Cash Inventory
panel draws from.

`python tools/callers.py 0x140d75850` — 7 call sites in 6 functions, whole-image, all three
modes: **[L]**

| sub-op | function | what it does to the locker |
|---|---|---|
| **`0x03`** | inline `0x140D7DCFD` | `u16 count`, then `count` records **appended** |
| **`0x04`** | `FUN_140D7E1F0` | **clears the whole map first** (`FUN_140D82B70`, `[stage+0x158]=0`), then `count` records — a full reload |
| **`0x0C`** | `FUN_140D7E5E0` | **one** record appended |
| **`0x1B`** | `FUN_140D80410` | one record appended, on the `bResult == 0` branch only |
| **`0x3E`** | `FUN_140D7F020` | up to **two** records appended, each behind its own `u8` flag |
| — | `FUN_140D7E450` | same shape as `0x04`, **zero callers in all three modes** — not in the 22-slot jump table. An orphan. |

`FUN_140D75850` drops a record whose serial is `-1` without adding it (`0x140D75884
cmp rdx,-1 ; je <return>`), and repaints `[stage+0xc8]` itself when the insert succeeds. **[L]**

### 3.2 Why `0x0C` and not `0x03`

`0x03` really does fill the panel, and it is the right packet for *"here is your locker"*. It is
the wrong packet for *"you just bought this"*, because it does nothing else: no cart update, no
message, no latch. Sending `0x03` then `0x05AD` gives an item in the Cash Inventory and **total
silence** — §7.3 makes that the control experiment.

`0x0C` is the purchase reply. Read in order (`tools/listing.py 0x140d7e5e0`): **[L]**

```asm
140d7e60e  alloc 0x76 ; 140d7e622 ctor FUN_1402D0770
140d7e66b  call 1402d0950(obj, pkt)      ; <<< the 71-byte record
140d7e6a1  call 140d75850(stage, &sp)    ; insert into [stage+0x150]  -> CASH INVENTORY
140d7e6b1  READ u32 -> eax
140d7e6b9  test eax,eax ; je 140d7e6cf   ;   non-zero -> 8 more bytes, into a buffer
140d7e6ca  READ raw[8]                   ;   that is never read again
140d7e6d2  READ u8                       ; read, discarded
140d7e6d7  if [stage+0xc8]: FUN_1410B5540   ; repaint the panel
140d7e6e8  cmp [stage+0x168], 0 ; jne 140d7e9fb
     ==0 : 140d7e70b  ecx = [obj+0x34]   ; the record's wire+20
           if ecx in 80,000,000..89,999,999 and NOT in 81,000,000..81,999,999:
                 string 592 "Purchase successful!\r\n(You currently have %lld Mesos.)"
           else: string 590 "You have successfully made the purchase."
           FUN_142A26280(...)            ; show it
     !=0 : 140d7e9fb  push [stage+0x164] onto the vector at [stage+0x178]  ; no message
140d7ea1a  idx = [stage+0x164]
           if 0 <= idx < len([stage+0x170]):
                 FUN_140D7A610(stage, [stage+0x170][idx])   ; REMOVE FROM CART
```

`FUN_140D7A610` erases that commodity SN from the vector at `[stage+0x108..0x110]` and refreshes
`[stage+0xa8]` and `[stage+0xb8]` — the cart list and the cart window. **[L]**

**`0x0C` touches neither `[stage+0x74]` nor `[stage+0x120]`.** No write, no call to the pump
`FUN_140D74A70`, no call to the cancel `FUN_140D74C70` (whole-function disassembly, not a grep).
That is correct, not a gap: §4.2 shows the wallet packet is what releases them.

### 3.3 `0x3E` — the same thing, with payback points and a bonus item

`FUN_140D7F020`, reads in listing order: **[L]**

```text
0x05AE  u8 0x3E
  u8    bItem1        ; != 0 -> <71-byte record> -> locker
  u16   nPaybackPts   ; -> esi
  u32   nSomeId       ; != 0 -> raw[8] follows, both handed to FUN_142D4AA20
  [ u8[8] ]
  u8    bItem2        ; != 0 -> <71-byte record> -> locker
```

then `[stage+0x168]` gates the message exactly as `0x0C` does, and

```asm
140d7f2b4  test esi,esi
140d7f2b6  jne  -> string 2438 "Purchase successful.\r\nYou have received %d Leaf Points."
140d7f2b8  else -> string 590  "You have successfully made the purchase."
```

So `0x3E` with `u16 = 0` produces the identical 590. It is the arm to use if a purchase should
also award payback points (`commodity.txt`'s `pbPoint` column). It is strictly more complex than
`0x0C` and buys nothing else, so §0 recommends `0x0C`.

### 3.4 `0x04` is a locker reload, not a purchase

`FUN_140D7E1F0`:

```text
0x05AE  u8 0x04
  u8    bShowMessage
  [ u32 nOverLimitCount ]     ; only if bShowMessage != 0
  u16   count
  count x <71-byte record>    ; the locker map is CLEARED FIRST
  u16 u16 u16 u16             ; read and discarded, four of them
  -> repaint; if bShowMessage: string 4186
```

String **4186** is *"You have %d items over the Cash inventory limit. Make room in your Cash
inventory and reconnect to the Cash Shop to view the rest of your items."* **[L]** — a warning,
not a success. `0x04` is `Res_LoadLocker_Done`. Useful for populating the panel at shop entry;
never for a purchase, because it wipes whatever is already there.

---

## 4. The success message and the sound

### 4.1 All 127 refusal reasons, enumerated and decrypted — 88 of 88 are error text

`cash-shop-buy-done.md` §4.1 mapped all 127 cases of `FUN_140D7C7F0` and then **spot-decrypted
37** of the 83 specific ids. That is a filter, not an enumeration, and a "Purchase complete"
hiding in the other 46 would have changed the answer. Re-extracted from the jump table at
`0x140D7D95C` (127 dwords, base `0x140000000`) and decrypted in full: **88 distinct string ids,
every single one error text.** **[L]**

The 88 are 157 188 586 587 600 601 603 609 610 612–649 653–657 661–671 674–678 680–683 694 721
1194 1831 1832 2268 2269 2502 3097–3100 3427 3931 4165 4166 4167 4187 4822. Spot samples:
601 *"You don't have enough cash."*, 661 *"Due to an unknown error, the Cash Shop request has
failed."*, 674 *"That item cannot be moved."*, 4187 *"Unable to complete purchase. Your previous
Value Pack is still active."*, 644 (Korean) *"PC-room item purchases have been temporarily
suspended."* Forty-four of the 127 fall to the generic 661.

So `cash-shop-stage.md` §6.2.1's conclusion is not merely confirmed, it is now closed: there is
no reason code that can be dressed as a success, and none of the 88 ids is empty.

### 4.2 The success message is raised by the buy builder, not by a packet

Every string-id fetch in the whole stage range was enumerated in one pass (`mov edx, imm` +
`call 0x1408A9E40`): **169 sites in 25 functions**, of which 91 are inside `FUN_140D7C7F0`. Of
the remaining 78, exactly three fetch string **590**: **[L]**

```text
0x140D78DE1   FUN_140D785F0   the BUY BUILDER's completion path
0x140D7E829   FUN_140D7E5E0   the 0x0C arm
0x140D7F2C1   FUN_140D7F020   the 0x3E arm
```

String **590 = "You have successfully made the purchase."** **[L]**

The builder's completion path is the one that fires for the shape the owner's client is actually in:

```asm
140d78635  cmp dword [stage+0x164], 0 ; jg <send the next cart item>
140d78646  edx = len([stage+0x178])          ; results accumulated by 0x0C / 0x3E
140d78657  r8d = len([stage+0x170])          ; the commodity SNs being bought
140d78671  cmp edx, r8d ; jne 140d78e0d      ; mismatch -> the per-item report (string 581)
140d7867a  test rax,rax ; je  140d78dd8      ; no results at all  -> string 590
140d78683  cmp [rax-8], 1 ; jbe 140d78dd8    ; exactly one result -> string 590
140d78dd8  mov edx, 0x24e ; call 1408a9e40   ; <<< "You have successfully made the purchase."
```

and it is re-entered from the wallet arm:

```asm
140d736dc  mov  byte  [stage+0x74], 0        ; <<< the latch, cleared UNCONDITIONALLY
140d736e0  eax = [stage+0x120]
140d736e6  cmp eax, 1 ; jne 140d736fd
140d736eb  mov  dword [stage+0x120], 0
140d736f6  call 140d785f0                    ; <<< back into the buy builder
```

**That single `mov byte [stage+0x74], 0` is why a purchase does not need a latch-clearing arm.**
`cash-shop-buy-done.md` built its whole recommendation on finding one, and the wallet packet it
already told us to send second had been doing it all along.

`[stage+0x168]` — the `u32` the buy request carries, **measured as 2** on 2026-08-26 — decides
which end shows the message. With `!= 0` the per-item arms stay quiet and push their index onto
`[stage+0x178]`, so lengths match at the end and the builder shows 590 **once**. With `== 0` the
arm shows 590 itself and `[stage+0x178]` is null, so `0x140D78E0D`'s `test rax,rax ; je` skips
the report. **Either way exactly one message.** **[D]**

### 4.3 `bEffect` points at art this build does not ship

`0x19`'s `bEffect` fires `FUN_142A37120(0x3EE, 0, -1, 0, 0, 0, 0)` at `0x140D80343`. That
function resolves its id through `FUN_142A3AAA0`, which is a three-case switch: **[L]**

```text
1004 (0x3EC) -> UI/CashShop.img/CSPopup/YesNo/5     @0x1434893B0
1005 (0x3ED) -> UI/CashShop.img/CSPopup/YesNo/6     @0x1434893F0
1006 (0x3EE) -> UI/CashShop.img/CSPopup/YesNo/11    @0x143489430
```

`UI/CashShop.img` **is not in this client's data.** `UI_000.wz` lists 67 images and
`_Canvas/_Canvas_000.wz` lists 64; both contain `CashShopUI.img` (the positive control) and
`CashShopPreview.img`, and a grep for `CashShop.img` across all four UI archives
(`UI.wz`, `UI_000.wz`, `_Canvas.wz`, `_Canvas_000.wz`) returns **0**. The executable still
carries **23**
`UI/CashShop.img/CSPopup/...` path strings plus `CSNotice`, `CSCoupon`, `CSBargainSale`,
`CSTimeSale` and `Base/backgrnd` — a whole legacy UI whose art was removed. **[L]**

Two more reasons `bEffect` is not what the owner is missing: `0x19` passes **item id 0 and count -1**
where the three other call sites of `FUN_142A37120` all pass a real item id
(`0x14104E587`, `0x14107A917`, `0x14107CD79` — all `ecx = 0x3EE`, all `edx = <itemId>`). **[L]**

> **Do not set `bEffect = 1` to "try it".** The canvas it names does not exist, and no run has
> ever exercised that path. It is a one-variable experiment worth a separate launch if anyone
> wants it, not something to bundle with a purchase test.

### 4.4 The sound is the message box's, not the cash shop's

`FUN_142085330(soundMgr, path, ...)` is the UI sound player — found from
`FUN_1428B5FB0`'s `lea rdx,[rip+…]` at `0x1428B6E25`, which points at the wide string
`Sound/UI.img/`. `tools/callers.py 0x142085330` gives **117 call sites in 104 functions**
(so the instrument speaks), and **not one of them is in `0x140d7xxxx`, `0x140d8xxxx` or
`0x1410xxxxx`** — the cash-shop stage and its whole window family never play a sound directly.
**[L]**

Three of the 104 are `FUN_14177A8B0`, `FUN_14177AE70`, `FUN_14177AF80` — the dialog framework.
`FUN_142A26280`, the message box that both `FUN_140D84B70` and `0x0C`'s inline path end in,
calls `FUN_14177F070` in that same family. **[D] — the sound the owner expects is the message box's
own, and it arrives with string 590 or not at all.** I did not decode which node
`FUN_14177AE70` plays, and there may be other sound entry points I have not enumerated.

---

## 5. The two moves

`cash-shop-actions.md` §3 and §4 have the request side and it is unchanged. What follows is the
reply side, re-read, with one correction that matters and one trap that is real.

### 5.1 `0x0A` locker -> Item Inventory: answer with `0x19`

```text
client -> 0x03E1  u8 0x0A, u8[8] liCashItemSN, u32 nItemID, u8 nInventoryType, u16 nSlotPosition

server -> 0x05AE  u8    0x19
                  u8    1                 bRelease. MUST be non-zero: zero leaves [stage+0x74]
                                          set and the shop UI dead for the session.
                  u16   nSlotPosition     echo the client's slot. 1-based, in the tab
                                          FUN_1403E8AF0 derives from the item id.
                  <GW_ItemSlot>           u8 nType (1, 2 or 3 ONLY) then the body
                  u8    0                 bEffect. Keep it 0 - see 4.3.
```

The client then erases the serial from the locker map, calls `FUN_1402E4C20`, repaints **both**
panels, zeroes `[stage+0x120]` if it is 4, and pumps the next queued request. No second packet.
**[L]**

`nType` outside 1..3 makes `FUN_1402CC180` return null and `0x140D7F932` dereference address
`0x20`. That hazard is unchanged from `cash-shop-buy-done.md` §3.2. **[L]**

To refuse instead: `0x05AE u8 0x3D, u16 nReason` — shows a message, clears the latch, empties
the queue. Reason 2 is the generic 661.

### 5.2 `0x0B` Item Inventory -> locker: answer with `0x1B`, and the shape is not what §4.3 said

**Correction.** `cash-shop-buy-done.md` §4.3 describes `0x1B`'s first byte as `bToSlot`, with
`!= 0` meaning SN/itemId/nPOS and `== 0` meaning a full record. The polarity of the *fork* is
right; the *reads* are not. Off `tools/listing.py 0x140d80410`: **[L]**

```asm
140d80447  mov byte [stage+0x74], 0        ; the latch is cleared FIRST, unconditionally
140d80461  READ u8   -> al
140d80468  test al,al ; je 140d8103c       ;   ZERO -> the RECORD form
140d80479  READ raw[8]                     ;   liCashItemSN
140d80481  READ u32                        ;   nItemID -> FUN_1403E8AF0 -> nInventoryType
140d80496  READ u32                        ;   nSlotPosition
140d8049e  test eax,eax ; jg 140d80f59     ;   > 0 -> verify (type, slot) holds that serial
```

So the two forms are:

```text
form A   0x05AE u8 0x1B, u8 1, u8[8] liCashItemSN, u32 nItemID, u32 nSlotPosition
         "the item at (tab, slot) with this serial has left your inventory"
         Verifies FUN_1402E3CD0(char, tab, slot)'s +0x38 == the serial, then clears that slot.
         It does NOT put anything in the locker.

form B   0x05AE u8 0x1B, u8 0, <71-byte cash-item record>
         "here is the locker row for the item that left your inventory"
         Inserts the record into [stage+0x150] (the CASH INVENTORY panel), derives the tab from
         the record's own item id, finds the slot with FUN_140230CB0(char, tab, liSN),
         then clears that slot.
```

**Form B is the one to send for `0x0B`**, because form A leaves the Cash Inventory panel empty —
the item vanishes from one side and appears on neither. That is very likely what the owner saw when
*"I tried to move items from Item Inventory to Cash Inventory and was not able to"*, except that
in their case nothing was sent at all.

### 5.3 The trap is real, and this is the value that avoids it

Both forms converge on `0x140D81125`: **[L]**

```asm
140d81125  rax  = [rbp+0x7f]              ; nInventoryType
140d81129  rdi  = character + rax*8
140d8112d  rbx  = [rbp-0x59]              ; the slot (signed)
140d81131  rax  = [rdi + 0x5d0]           ; that tab's slot array - MAY BE NULL
140d81138  ecx  = rax ? [rax-8] : 0       ; its length
140d81143  if (rbx >= 0 && rbx < ecx) goto 140d8116a
140d81159  call 142e54290(0xbc, rbx, ecx) ; an out-of-range REPORT - it does NOT abort
140d81163  rax  = [rdi + 0x5d0]           ; reload, still possibly null
140d8116a  rcx  = rax + rbx*16 ; call 140d81174 -> FUN_1401ABD80
```

`FUN_1401ABD80`'s first instruction is `mov rbx,[rcx+8]`, no null check. **[L]** So:

* If the tab array is **null**, the call reads address `8` and the client faults — and the
  range report above it does not prevent that, it only logs.
* If it is non-null, the call `shared_ptr`-resets `(tab, slot)`, destroying whatever is there.

`[rbp-0x59]` is the packet's `nSlotPosition` in form A and `FUN_140230CB0`'s answer in form B.
`FUN_140230CB0` returns **0** when the serial is not in the character's tabs 5 or 6
(`0x140230F38 xor eax,eax`), and `0x140D810B5`'s `jg` merely routes to an ELog build that falls
through to the same reset.

**The value that avoids it:**

* **Form B: the record's `liSN` must be a serial the character's cash tab actually holds.**
  For a genuine `0x0B` it always is — the player just double-clicked that item — so form B is
  safe for the move it was designed for.
* **Form A: `nSlotPosition` must be `> 0` and must be the slot the item is really in.** Then
  `FUN_1402E3CD0` found it there, which proves the array is non-null.
* **Never use `0x1B` for a purchase.** A freshly bought item is not in the character's
  inventory, `FUN_140230CB0` returns 0, and the tail resets index 0 of a tab that may not have
  an array. This is the concrete reason `0x1B` must not be the buy reply, and it is stronger
  than the "it is a trap" hedge in `cash-shop-buy-done.md` §4.3.

Whether an *empty* tab has a null array is still **not measured**, and it is the only thing
standing between "resets an unused slot 0" and "faults at address 8". Tab **6** — the pseudo-tab
`FUN_1403E8AF0` sends some `1xxxxxx` equips to — is the likeliest to be null.

---

## 6. The 71-byte record: what is read, what is not, and why the old explanation was wrong

### 6.1 The layout, now cross-checked a fourth way

`FUN_1402D1320(dst, record)` is a compiler-generated **copy into a compact POD**, and
`FUN_1402D1380(pod, packet)` **re-encodes that POD onto a packet**. Neither has any caller in
any of the three scan modes — they are dead code — but they are *typed* dead code, and the
widths they use are a fourth independent statement of the layout. **[L]**

```text
FUN_1402D1320                       FUN_1402D1380 writes
  pod+0x00 <- rec+0x00 (qword)        w_raw 8   pod+0x00
  pod+0x08 <- rec+0x10 (dword)        w_u32     pod+0x08
  pod+0x0c <- rec+0x18 (WORD)         w_u16     pod+0x0c    <<< wire+24 IS a u16
  pod+0x0e <- rec+0x27 (qword)        w_raw 8   pod+0x0e    <<< wire+39 IS 8 opaque bytes
  pod+0x16 <- rec+0x14 (dword)        w_u32     pod+0x16
  pod+0x1a <- rec+0x2f (dword)        w_u32     pod+0x1a
  pod+0x1e <- rec+0x33 (f64)          w_f64     pod+0x1e
  pod+0x26 <- FUN_1401B1040(...)      w_u32     pod+0x26    <<< computed, not from the record
  pod+0x2a <- rec+0x44 (byte)         w_u8      pod+0x2a
```

Nine payload fields. Note the fifth field is `rec+0x14` — **wire+20** — sitting between the
expiry and the `u32` at wire+47 in the compact form, exactly where a `nCommodityID` belongs.

Full table. "obj" is the displacement from the `0x76`-byte heap object; "rec" is the
displacement from `obj+0x20`, which is what the code actually uses.

| wire | size | obj | rec | status |
|---|---|---|---|---|
| +0 | 8 | +0x20 | +0x00 | **u64 liSN [L]** — map key; `-1` makes `FUN_140D75850` drop the record |
| +8 | 4 | +0x28 | +0x08 | no reader [L]; `dwAccountID` [I] |
| +12 | 4 | +0x2c | +0x0c | no reader [L]; `dwCharacterID` [I] |
| +16 | 4 | +0x30 | +0x10 | **u32 nItemID [L]** — `0x1B` at `0x140D81094` -> `FUN_1403E8AF0` |
| +20 | 4 | +0x34 | +0x14 | **READ — `0x140D7E70B`, the `0x0C` meso-message gate [L].** `nCommodityID` [I] |
| +24 | 2 | +0x38 | +0x18 | u16 [L]; **quantity `nNumber` [I]** — no reader anywhere below |
| +26 | 13 | +0x3a | +0x1a | 13 raw bytes [L]; `sBuyCharacterName` [I]; no reader |
| +39 | 8 | +0x47 | +0x27 | 8 raw bytes [L]; **expiry FILETIME [I]** — no reader anywhere below |
| +47 | 4 | +0x4f | +0x2f | u32 [L]; no reader |
| +51 | 8 | +0x53 | +0x33 | f64 [L]; no reader |
| +59 | 4 | +0x5b | +0x3b | u32 [L]; no reader |
| +63 | 4 | +0x5f | +0x3f | u32 [L]; no reader |
| +67 | 1 | +0x63 | +0x43 | **READ — `0x140D755B1`, the delete builder's refundable gate [D]** (string 651) |
| +68 | 1 | +0x64 | +0x44 | u8 [L]; no reader |
| +69 | 1 | +0x65 | +0x45 | u8 [L]; no reader |
| +70 | 1 | — | — | **trailing flag [L]** — `0` ends the record; non-zero means a whole `GW_ItemSlot` follows |

`FUN_1402D0010` reads exactly the first 70 bytes into a POD whose offsets are the "rec" column,
and stops — it does **not** read the trailing flag. That is why 70 and 71 both appear in the
notes; the wire record with the flag is **71 bytes**.

### 6.2 What wire+20 does, which is new

`0x0C`'s message gate: **[L]**

```asm
140d7e70b  ecx = [obj+0x34]                  ; wire+20
140d7e70f  eax = ecx - 0x04C4B400 (80,000,000) ; cmp 0x98967F (9,999,999) ; ja  -> string 590
140d7e720  eax = ecx - 0x04D3F640 (81,000,000) ; cmp 0x000F423F (999,999) ; jbe -> string 590
           otherwise: decode the character's meso through the secure-tear pair at
           [char+0xBF]/[char+0xC7]/[char+0xCB]/[char+0xCF] and show string 592,
           "Purchase successful!\r\n(You currently have %lld Mesos.)"
```

Commodity SNs in the 80,000,000–89,999,999 band (minus the 81 million sub-band) are the classic
**meso-shop** range, which is what makes `nCommodityID` the reading. **[I], and it does not need
to be right:** `gm-handbook/commodity.txt` holds **159** commodities with SNs from **92,000,000
to 160,300,005** and **zero** in 80–90 million, so every commodity in this build takes the
string-590 branch whatever this field means — as long as the server puts the commodity SN there
and not something that lands in that band. **[L]**

### 6.3 The negative on the quantity and the expiry, re-taken with the right shape

`cash-shop-buy-done.md` §5.3 reported no readers and explained it as *"a lazy repaint reached
through a virtual call, which is exactly the shape rangescan.py cannot follow."* **That
explanation is wrong, in two separate ways.**

**One: the class has no virtual getters.** The ctor `FUN_1402D0770` installs vtable
`0x14327ECC8`. Its slots are `0x1402FB450`, `0x1402FB0A0`, `0x1402FB6C0`, `0x1402FB220`,
`0x1402FB380`, `0x1402FB080` — and `0x1402FB450` is this class's *scalar deleting destructor*
(`add rcx,0x66 ; call FUN_1401ABD80` to release the trailing `shared_ptr`, then free `0x76`
bytes). The rest belong to neighbouring classes. There is exactly **one** virtual function and
it is the destructor. Nothing about this object is read behind a virtual call. **[L]**

**Two: the lazy repaint is `FUN_1410B5550`, it is in the range that was scanned, and all it does
with the object is store its pointer.** `FUN_1410B5540` is the dirty flag
(`mov byte [rcx+0x250],1 ; ret`); `FUN_1410B4E00` tests and clears it; `FUN_1410B5550` walks the
red-black tree at `stage+0x150` and, per node, allocates a `0x11C0`-byte row and writes
`row+0x11B8 = <the cash-item object>` (`0x1410B5790`). The first **40** slots of the row's
vtable at `0x14337CB68` were enumerated and disassembled, and **not one of those methods touches
any record displacement** (the walk stopped at 40 by its own limit, not at a table end, so
deeper slots are unchecked). The panel's sort
comparators (`FUN_1410B7770`, `FUN_1410B7B90`, `FUN_1410B7EF0`, eleven sites) compare
`[row+0x11B8]` against `[row+0x11B8]` — **pointer identity**, not any field. **[L]**

With that excuse removed, the scan was re-run properly: one disassembly pass per range,
collecting every `[reg+disp]` memory operand (**`rbp` included** — only `rip` and `rsp` were
excluded, so a frame-pointer-as-object-pointer read cannot hide), for **both** shapes at once —
record-relative `{0x18, 0x1a, 0x27, 0x2f, 0x33, 0x3b, 0x3f, 0x43, 0x44, 0x45}` and
object-relative `{0x28, 0x2c, 0x34, 0x38, 0x3a, 0x47, 0x4f, 0x53, 0x5b, 0x5f, 0x63, 0x64, 0x65}`
— over three ranges:

```text
0x140d70000..0x140d90000   the cash-shop stage
0x1410a0000..0x1410e0000   the cash-shop windows (cashLocker, itemInven, cart, gift)
0x141700000..0x141720000   the row-widget family the panel instantiates
```

Every hit was read and eliminated by hand. The only two that looked like the record were
`FUN_140D7B770`'s `lea rcx,[rdi+0x27] ; mov edx,[rdi+0x2f]` — an exact match for
expiry+`rec+0x2f` — and its `rdi` is `FUN_142CBE730(g_ctx)`, **the character**, feeding the
age gate `FUN_1401BA9D0`. Coincidence, eliminated. **[L]**

**So: four of the fifteen record fields have a reader — wire+0, +16, +20, +67 — and the other
eleven, including the quantity at wire+24 and the expiry at wire+39, have none anywhere in the
cash shop.** The remaining blind spots are named, not hand-waved:

* Address ranges outside the three above. The stage's own helpers and the shop windows are
  covered; a reader living in, say, the item-tooltip code would not be.
* An access through a pointer that already carries part of the offset — `lea rcx,[rec+0x20]`
  then `[rcx+0x07]`. This is the `mob+0x42c` blind spot and it is exactly what caught the
  previous pass. No such two-step was found, but a displacement scan cannot prove there is none.
* The `0x14107xxxx` parallel implementation (§8.2), which also decodes these records and which
  I did not scan for field reads because I believe it is dead.

### 6.4 What to put in the fields, then

| field | send | why |
|---|---|---|
| +0 liSN | a unique non-zero `u64`, never `-1` | `FUN_140D75850` drops `-1` and a duplicate collides in the map. This is the handle every later `0x0A`/`0x0B`/`0x1C` will name. **[L]** |
| +16 nItemID | the real item id | drives `FUN_1403E8AF0` on every later move. **[L]** |
| +20 | the commodity SN | keeps `0x0C` on the string-590 branch. Any value outside 80–90 M does, but the SN is the one that is probably right. **[L]** for the branch, **[I]** for the name |
| +24 quantity | the commodity's `count` column | nothing reads it in this build **[L]**; `count` is 1 for 149 of 159 commodities |
| +39 expiry | 8 bytes, zeros are as defensible as anything | nothing reads it in this build **[L]**. The "permanent" sentinel is still **not established** — `cash-shop-buy-done.md` §5.3's byte search for `150842304000000000` returned 177 undiscriminated hits and settles nothing, and I did not improve on it |
| +67 refundable | `0` | non-zero makes the client refuse a delete with string 651 *"A refundable item cannot be deleted."* **[D]** |
| everything else | zeros | no reader found; untested |
| +70 trailer | `0` | non-zero means a whole `GW_ItemSlot` follows and the client will read one |

For the **Mystery Hair Coupon** the owner bought: `gm-handbook/commodity.txt` gives SN `150000000`,
item `5150000`, count `1`, period `0`. **[L]**

---

## 7. Wire it like this

Nothing below is wired. `crates/` was off-limits for this task.

### 7.1 A purchase

```text
client -> 0x03E1  u8 0x02, u8 1, u32 2, u8 0, u8 0, u32 commoditySN, u32 0
          (measured 2026-08-26; the u8 and u32 are [stage+0x168], see 4.2)

server:   debit the account, mint a cash serial, insert the row, then

server -> 0x05AE  u8   0x0C
                  u64  liSN                 unique, != 0, != -1
                  u32  0                    [I] dwAccountID
                  u32  characterId          [I] dwCharacterID
                  u32  itemId               [L]
                  u32  commoditySN          [L] must stay outside 80,000,000..89,999,999
                  u16  count                [I] quantity
                  u8[13] 00 x 13            [I] buyer name
                  u8[8]  00 x 8             [I] expiry
                  u32  0
                  f64  0.0
                  u32  0
                  u32  0
                  u8   0                    refundable - keep 0
                  u8   0
                  u8   0
                  u8   0                    the trailer flag: 0 ENDS the record
                  u32  0                    non-zero => 8 more bytes are consumed
                  u8   0                    consumed and discarded, must be present

server -> 0x05AD  u32  nxCredit             the DEBITED balance, >= 0
                  u32  maplePoints          >= 0
                  u32  0                    read, sign-checked, discarded
```

**Order is not cosmetic.** `0x05AD` is what clears `[stage+0x74]` and `[stage+0x120]`, and while
`[stage+0x120] == 1` it re-enters the buy builder. Sending it first means the client re-enters
the builder before the result has arrived.

Any of the three `u32`s in `0x05AD` being negative takes `0x140D73651`/`0x140D73659`/
`0x140D73661` to `0x140D7371A`. **[L]** — keep them non-negative.

To refuse a purchase instead: `0x05AE u8 0x1A, u8 nReason` (§4.1's table; 2 is the generic).
Note `cash-shop-actions.md` §2's warning still stands — `0x1A` empties the whole request queue.

### 7.2 The two moves

```text
0x0A  ->  0x05AE  u8 0x19, u8 1, u16 slot, u8 nType + <GW_ItemSlot>, u8 0
0x0B  ->  0x05AE  u8 0x1B, u8 0, <71-byte record>          <- FORM B. Fills the Cash Inventory.
          refuse either with 0x05AE u8 0x3D, u16 reason
```

### 7.3 What falsifies each step

Run these in this order; the first two are one launch and settle §3 and §4 together.

1. **Buy one item; answer `0x0C` then `0x05AD`.**
   *Expected:* the item appears in the **Cash Inventory** panel, and a message box says
   *"You have successfully made the purchase."*
   *Falsified if the item appears but there is no message* ⇒ the message is not coming from
   `FUN_140D785F0`'s completion path, i.e. §4.2's reading of `[stage+0x164]`/`[stage+0x178]` is
   wrong. Distinguish by clicking Buy again: if a second `0x03E1` appears in `world.log` the
   builder ran and chose the *send* branch, so the cart index was not exhausted.
   *Falsified if the client dies inside the handler* ⇒ check
   `client-patched\maplecw-hook.log` for the dispatch line on `0x05AE`; it is written on
   **return**, so a missing one means the record is malformed.
   *Falsified if the item lands in the Item Inventory* ⇒ `[stage+0xc8]`/`[stage+0xd8]` are the
   other way round and §2 is wrong.

2. **The control: buy a second item and answer `0x03` (count 1, same record) then `0x05AD`.**
   *Expected:* the item appears in the Cash Inventory and **nothing is said.**
   This is the A/B that proves the message belongs to the buy-completion path and not to the
   locker insert. If `0x03` also shows a message, §4.2 is wrong and `0x0C` is not special.

3. **Click Buy a third time.** *Expected:* a `0x03E1` sub-op `0x02` in `world.log`, i.e. the
   shop is not latched. *Falsified if nothing is sent* ⇒ `0x05AD` did not clear `[stage+0x74]`
   or `[stage+0x120]`, and `0x140D736DC` is not doing what §4.2 says.

4. **Double-click the new item in the shop's Item Inventory panel** (`0x0B`), answer form B.
   *Expected:* it moves from the Item Inventory to the Cash Inventory.
   *Falsified if it disappears from both* ⇒ the record's `liSN` did not match, `FUN_140230CB0`
   returned 0, and the client cleared slot 0. *Falsified if the client faults* ⇒ §5.3's null
   tab array is real and should be written up.

5. **Only if someone wants it, and alone:** re-send `0x19` with `bEffect = 1`.
   *Expected, per §4.3:* nothing, or a fault, because the canvas does not exist. This is a
   separate launch and must not be bundled with anything above.

---

## 8. Corrections to the two files this continues

Recorded here rather than edited into them, because they belong to other passes.

### 8.1 `research/cash-shop-buy-done.md`

1. **§0 and §6.2, the whole recommendation.** `0x19` is the reply to a locker→inventory move and
   puts the item in the character's bag. Confirmed on a live client 2026-08-27. The purchase
   reply is `0x0C` (or `0x3E`), and the release is `0x05AD`, which that file already told us to
   send — `0x140D736DC` clears `[stage+0x74]` unconditionally, so no arm had to.
2. **§3's table is missing the column that mattered** — "inserts into the locker map". Adding it
   (§3.1 here, from a whole-image `callers.py` on `FUN_140D75850`) makes `0x03`, `0x04`, `0x0C`,
   `0x1B` and `0x3E` the candidate set, and `0x0C`/`0x3E` the two that also say something.
3. **§3's "msg" column is wrong for `0x0C` and understated for `0x3E`.** Both show string 590
   through `FUN_142A26280` directly, not through `FUN_140D7C7F0`/`FUN_140D84B70`, which is why a
   scan for those two callees missed them.
4. **§4.1's 37-of-83 spot check is now a full enumeration.** All 127 reasons, 88 distinct ids,
   every one error text (§4.1 here). The conclusion is unchanged and is now closed.
   That file's *"case `0x49` → string id 17"* does not reproduce: re-extracting the table at
   `0x140D7D95C` gives reason 73 (`0x49`) → **string 610**, *"You need more %s to make the
   purchase."* Both walkers take the first `mov edx, imm` before the first `FUN_1408A9E40`
   call, so one of us is following a different edge; it changes nothing, since both are error
   text, but the row should not be quoted as settled.
5. **§4.3's `0x1B` read shape is wrong.** The first `u8` is a success/form selector whose ZERO
   branch carries the record; the SN, item id and slot are read on the NON-zero branch, and the
   slot is a `u32` whose **sign** picks the verify path. §5.2 here has the listing.
6. **§5.3's explanation of the negative is wrong.** The class has one virtual function and it is
   the destructor; the lazy repaint is `FUN_1410B5550`, inside the range that was scanned, and
   it only stores the object pointer. The negative itself survives a correctly-shaped re-scan
   (§6.3), but the reason given for it was not the reason.
7. **§5.3 scanned object-relative displacements when the code is record-relative.** Its own
   quoted evidence, `0x140D755B1 cmp byte [rdi+0x43]`, shows the pointer is `obj+0x20`.
8. **§5.1's "71 bytes" and `FUN_1402D0010`'s reads disagree by one** and both are right:
   `FUN_1402D0010` reads 70 bytes and stops; `FUN_1402D0950` reads those 70 plus the trailing
   flag. Worth saying once so nobody "fixes" it.

### 8.2 `research/cash-shop-actions.md`

1. **§4.1's `0x1B` shape** — same correction as above. Its four reads are right; the branch
   structure is not, and the fourth `u32`'s meaning is now known: it is the slot, and `> 0`
   selects the verify path.
2. **§11.2's `0x0B -> 0x1B, u8 1, ...` line will not fill the Cash Inventory.** Form B (`u8 0`
   plus a record) is the one that does.
3. **A second, parallel `0x05AE` dispatcher exists and neither file mentions it.**
   `FUN_141073F80`, reached from vtable slot `FUN_141072EC0` (`0x143378290`), switches on
   sub-ops **2..0x73** — a wider range than the live stage's 3..0x58 — through a byte index at
   `0x141074BB0` and 50 targets at `0x141074AE8`. Its `0x19` arm is `FUN_14107A560`, which reads
   a **71-byte cash-item record** and fires `FUN_142A37120(0x3EE, itemId, count)` with a real
   item id. Nine of the fifteen callers of `FUN_1402D0950` live in that family.
   **I believe it is dead** — the live stage is `FUN_140D734E0` (vtable `0x14336DEF8`, ctor
   `0x140D71CB0`, the one whose `0x03E1` builder matched the 2026-08-26 capture byte for byte),
   and the owner's `0x19` behaved exactly as `FUN_140D7F8A0` predicts and not at all as
   `FUN_14107A560` would. It also matches the missing `CashShop.img` art (§4.3): an older
   implementation left in the binary. **[D], and it is a derivation, not a measurement.** If
   anything here ever behaves strangely, that dispatcher is the first place to look.

---

## 9. What this file does NOT establish

* **Nothing here has been on a wire.** The only measurements in the whole cash-shop chain remain
  the outbound buy of 2026-08-26 and the owner's 2026-08-27 observation of where the `0x19` item
  landed. Every claim about a *reply* is static.
* **The quantity at wire+24 and the expiry at wire+39.** Widths yes (§6.1, from the dead but
  typed `FUN_1402D1380`), semantics no. What is now established is stronger than before and
  in the other direction: **nothing in the cash-shop stage, its windows, or its row widgets
  reads either field**, under either addressing shape, with `rbp` included, and the class has no
  virtual getters to hide behind. The three blind spots that remain are named in §6.3.
* **The "permanent" expiry sentinel.** Not found, not searched better than the previous pass.
* **Who writes `[stage+0x168]`.** Measured as **2** on the wire; the only writes visible are the
  ctor's zeroing and two reset paths in `FUN_140D785F0`. `rangescan` for disp `0x168` and `0x164`
  over both the stage and the shop-UI ranges finds no other write, so it is set through a pointer
  that already carries part of the offset — the `mob+0x42c` shape. It matters only in that it
  decides *which end* shows string 590 (§4.2), and both ends show it exactly once.
* **Which node the message box's sound plays**, or whether it plays one at all. §4.4 shows the
  cash shop never calls `FUN_142085330` itself and that the dialog framework does; I did not
  follow it into `FUN_14177AE70`, and I did not enumerate other sound entry points.
* **Whether `bEffect = 1` is harmless.** The canvas it names is absent from this build's UI
  archive; whether the effect object tolerates a missing node or faults is untested. Treat it as
  a separate experiment.
* **Whether an empty inventory tab has a null slot array** (§5.3). Still the difference between
  `0x1B`'s bad path resetting an unused slot and faulting at address `8`.
* **`FUN_140D7E450`, `FUN_140D81250`, `FUN_140D81320`, `FUN_140D7DB60`, `FUN_1402D0010`,
  `FUN_1402D1320`, `FUN_1402D1380`** all have zero callers in all three `callers.py` modes.
  They are treated here as dead code and used only as layout evidence. An indirect call through
  a register would leave no trace in any of those scans.
* **`FUN_142AA2810(2)`, `FUN_142CE70C0`, `FUN_1401B1040`, `FUN_142D4AA20`** are named by
  position only; not decoded.
* **The gift reply.** Unchanged and still unknown — `cash-shop-actions.md` §7 and §10 stand.
