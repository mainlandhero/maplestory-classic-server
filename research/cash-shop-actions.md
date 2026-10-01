# The five non-buy `0x03E1` sub-ops, decoded

2026-08-26. Static only — **no client run was spent on this.** This file continues
`research/cash-shop-stage.md` section 5.2, which read the *shapes* of six `0x03E1` builders
and named only two of them. This file says what every field **is**, names the UI gesture that
produces each packet, and names the `0x05AE` sub-op that answers it.

Tags: **[L]** read out of the image, **[D]** derived from something read, **[I]** inferred.

---

## 0. The short version

| `0x03E1` sub-op | what it is | body after the sub-op byte | answer |
|---|---|---|---|
| `0x03` | **gift** | `u32 commoditySN, str recipient, str message` | **not established** — see §7 |
| **`0x0A`** | **locker → character inventory** | `raw[8] SN, u32 itemId, u8 invType, u16 slot` | **`0x05AE` sub-op `0x19`** |
| `0x0B` | **character inventory → locker** | `raw[8] SN, u32 itemId, u8 invType, **u32** slot` | `0x05AE` sub-op `0x1B` |
| `0x1C` | **delete a cash item** | `raw[8] SN` | `0x05AE` sub-op `0x3C` |
| `0x2B` | **query a commodity's stock** | `u32 commoditySN` | `0x05AE` sub-op `0x57` |
| — | *refusal for `0x0A`/`0x0B`/`0x1C`* | — | `0x05AE` sub-op `0x3D`: `u16 reason` |

**`0x0A` is the locker → cash-tab move**, the one the `!locker` GM command is currently
faking. Its success reply is `0x05AE` sub-op `0x19` and it is fully specified in §3.

Three things here contradict or extend `cash-shop-stage.md`, all **[L]**, all in §8:
`0x2B` sets **no** in-flight latch; `0x1B`'s last field is a **`u32`, not a `u16`**; and
`0x19` reads **an entire item slot plus a trailing `u8`** that section 6.2 did not see.

---

## 1. The instruments, and their controls

Every tool was run in the repo working directory (`python tools/...`), so the stale
scratchpad copies `CLAUDE.md` warns about were not on `sys.path`.

| instrument | control | result |
|---|---|---|
| `tools/listing.py` | `0x140304100 \| grep READ` must give `140304138 raw`, `140304144 u8`, `140304183 u8`, then the `u16` run | passed, exactly |
| `tools/callers.py` | `0x1402fa9a0` must give 96 sites in 15 functions | passed — 96 / 15 |
| `tools/encodes.py` | `0x141cb6880 1` must show CTOR at `141cb7eb1`, SEND at `141cb8365` | passed, both |
| `tools/dump_stringids.py` | id 651 must decrypt to readable English | passed — *"A refundable item cannot be deleted."* |
| `tools/rangescan.py` | disp `0x130` in the stage range must find the pump's own `140d74ada mov rcx,[rbx+0x130]` | passed |

### 1.1 The control that matters most: the measured buy

`cash-shop-stage.md`'s §5.2 shapes are now credible because the **buy** was captured on the
wire. Before trusting my reading of the other five, I re-derived the measured bytes from
`FUN_140D785F0`'s listing. Measured body after sub-op `0x02`:

```text
01 | 02 00 00 00 | 00 | 00 | <u32 commoditySN> | 00 00 00 00
```

Read off the listing at `0x140D7A45E`..`0x140D7A4EB`: [L]

```asm
140d7a445  mov   edx, 2                     ; sub-op, or...
140d7a44f  mov   eax, 0x1f
140d7a454  cmovb edx, eax                   ; ...0x1F if itemId - 0x08ADDAE0 < 10000
140d7a463  cmp   r14d, 2 ; sete dl          ; u8  = (mode == 2)          -> measured 01
140d7a476  mov   edx, [rbx + 0x168]         ; u32 = a stage field        -> measured 02 00 00 00
140d7a4b4  jb    140d7a4d2                  ; SHORT form SKIPS the next two writes
140d7a4bf  w_u8  0                          ; u8                          -> measured 00
140d7a4cd  w_u8  0                          ; u8                          -> measured 00
140d7a4db  mov   edx, edi                   ; u32 commoditySN
140d7a4eb  mov   edx, [rsp + 0x30]          ; u32                        -> measured 00 00 00 00
```

Every measured byte is accounted for, including **why the two `00`s are present** — `jb`
skips them in the short (`0x1F`) form, and the capture is the long (`0x02`) form. The same
reading method produced everything below.

---

## 2. The shape they all share: a request queue

`FUN_140D74A70` is the pump, and it is the single most useful thing in this file. [L]

```asm
140d74a82  cmp  qword [rcx + 0x148], 0     ; queue empty -> give up
140d74a90  cmp  dword [rcx + 0x120], 0     ; an operation already in flight -> give up
140d74ac9  ...  [rbx+0x130] / [rbx+0x138] / [rbx+0x140] / [rbx+0x148]
140d74ae5  movups xmm0, [rdx] ; movups xmm1, [rdx+0x10]   ; pop ONE 0x20-byte request
140d74b14  mov  ecx, [rsp + 0x30]          ; the request's KIND
140d74b18  sub  ecx, 4 ; je 140d74bf0      ; kind 4 -> builds 0x0A
140d74b21  sub  ecx, 1 ; je 140d74b5a      ; kind 5 -> builds 0x0B
140d74b26  cmp  ecx, 1 ; jne 140d74b40     ; kind 6 -> builds 0x1C, else CANCEL
140d74c45  mov  eax, [rsp + 0x30]
140d74c49  mov  [rbx + 0x120], eax         ; [stage+0x120] = the in-flight KIND
```

So **`[stage+0x120]` is not a boolean "a purchase is pending" — it is the kind of the
operation in flight**, and the whole family shares one queue: [L]

| `[stage+0x120]` | set by | meaning |
|---:|---|---|
| `0` | `FUN_140D74C70` | idle |
| `1` | `0x140D7A500`, the buy builder | buy in flight |
| `2` | `0x140D7B6C8`, the gift builder | gift in flight |
| `4` | the pump | `0x0A` locker → inventory |
| `5` | the pump | `0x0B` inventory → locker |
| `6` | the pump | `0x1C` delete |

The 32-byte request struct, read off `FUN_1410B43E0` where it is built and off the three
builders where it is consumed: [L]

```text
+0x00  u32   kind            4 / 5 / 6
+0x08  u64   liCashItemSN    <- cashItem + 0x20   (record offset +0)
+0x10  u32   nItemID         <- cashItem + 0x30   (record offset +16)
+0x14  u32   nInventoryType
+0x18  u32   nSlotPosition
```

`FUN_140D749A0` is the enqueue (`lea rbx,[rcx+0x128]`, a ring buffer); it refuses unless
`[stage+0x120] == 0`. `FUN_140D74C70` is the cancel: it **swaps the whole queue out** and
writes `[stage+0x120] = 0`. [L]

> **This matters for the server.** `cash-shop-stage.md` §6.2.1 recommends `0x05AE` sub-op
> `0x1A` as the safe refusal because it calls `FUN_140D74C70`. That is still right for a
> **buy**, but for a *move* it now reads differently: `0x1A` does not merely cancel the one
> operation, it **discards every other queued move as well**. The delete UI (§6) enqueues one
> request per selected item, so a multi-select delete refused with `0x1A` loses the rest of
> the batch silently. Use `0x3D` (§5) for this family instead.

### 2.1 The client picks the destination slot itself

The kind-4 path, before the `0x0A` builder is called: [L]

```asm
140d74bf0  mov  ecx, [rsp + 0x40]     ; nItemID
140d74bf4  call 0x1403e8af0           ; itemId -> inventory type
140d74bf9  mov  [rsp + 0x44], eax     ; req+0x14 = type      <- COMPUTED CLIENT-SIDE
140d74c05  call 0x14022fae0           ; find a free slot of that type
140d74c0a  mov  [rsp + 0x48], eax     ; req+0x18 = slot      <- COMPUTED CLIENT-SIDE
140d74c10  jne  140d74c30             ; slot found -> send
140d74c12  mov  edx, 0x9b             ; else string 155 "Your inventory is full."
```

So for `0x0A` the server is **told** which type and slot the client chose, and no packet is
sent at all when the inventory is full. **[L]**

---

## 3. `0x0A` — LOCKER → CHARACTER INVENTORY (the one that matters)

Builder `FUN_140D74E10`, 605 bytes. [L]

### 3.1 Guards, in order

```asm
140d74e41  cmp  [rcx + 0x120], ebp     ; ebp = 0 : nothing else in flight
140d74e4d  cmp  [rcx + 0x74],  bpl     ; no request in flight
140d74e57  cmp  dword [rdx], 4         ; the request's KIND must be 4
140d74e60  cmp  [rcx + 0xc8], rbp      ; the locker window must exist
```

Then it indexes the inventory's per-type slot vector at `[invMgr + type*8 + 0x5d0]`, looks up
`(type, slot)` with `FUN_1402E3CD0`, and **requires the destination slot to be EMPTY**
(`cmp qword [rax+8], 0` — a non-empty slot sets `r14b = 1` and aborts before any send). [L]
It then finds the item in the locker list `[stage+0xc8]` by `(SN, itemId)` via
`FUN_1410B6CA0`; a miss returns false with no packet. [L]

### 3.2 The wire

```asm
140d74f79  w_u8   0x0a                        ; sub-op
140d74f8b  w_raw  8  from [rdi + 8]           ; liCashItemSN   (u64)
140d74f99  w_u32     from [rdi + 0x10]        ; nItemID
140d74fa6  w_u8      from [rdi + 0x14]        ; nInventoryType
140d74fb4  w_u16     from [rdi + 0x18]        ; nSlotPosition
140d74fcc  mov  byte [rsi + 0x74], 1          ; in-flight latch SET
```

```text
0x03E1
  u8      0x0A
  u8[8]   liCashItemSN
  u32     nItemID
  u8      nInventoryType     (1..5; derived by the client from nItemID)
  u16     nSlotPosition      <- NOTE: u16 here, u32 in 0x0B
```

### 3.3 Its success reply: `0x05AE` sub-op `0x19`

`FUN_140D7F8A0`. Its **last act** is the proof of the pairing: [L]

```asm
140d802af  xor  r12d, r12d
140d80348  cmp  dword [rsi + 0x120], 4       ; only acts if the in-flight kind is 4
140d80351  mov  dword [rsi + 0x120], r12d    ; mode = 0
140d8035b  call 0x140d74a70                  ; pump the next queued request
```

Read shape, all three direct reads enumerated with `listing.py` (not a depth-limited walk):
[L]

```text
0x05AE
  u8    nResult = 0x19
  u8    bClearLatch     140d7f8f1   ; NONZERO -> [stage+0x74] = 0.  Zero leaves the UI blocked.
  u16   nSlotPosition   140d7f901   ; where the item landed
  u8    nItemType + <GW_ItemSlot body>        FUN_140303530
  u8    bFlag           140d7f97e   ; -> setne -> a bool
```

In between, at `0x140D7F95E`, it erases the item from the locker map `[stage+0x150]`
(`FUN_140D85C10`) keyed by the decoded item's `+0x38` cash serial, and refreshes the locker
window — provided that serial is not `-1`. [L] It re-derives the inventory type from the
decoded item's own id, not from the packet (`0x140D7F9AD` secure-tear getter → `0x1403E8AF0`).
[L]

> `FUN_140303530` reads `u8 type` then builds the slot through a factory
> (`0x140303543` READ u8 → `FUN_1402CC180`) and decodes the body virtually. It is the same
> decoder `cash-shop-stage.md` §6.5 names at the tail of the cash-item record, and the same
> family this server already builds for ordinary inventory.

**`cash-shop-stage.md` §6.2 records `0x19` as `u8, u16`.** That is a depth-4 `reads.py` walk
and it **missed the item body and the trailing `u8`**. A server that sends only `u8, u16`
sends a packet two-plus fields short. [L]

---

## 4. `0x0B` — CHARACTER INVENTORY → LOCKER

Builder `FUN_140D75080`, 1015 bytes. [L] The mirror of `0x0A`: it finds the item **in the
character's inventory** at `(type, slot)` and requires its cash serial `[item+0x38]` to equal
the request's, where `0x0A` finds it in the locker.

```asm
140d750cd  mov  eax, [rdx + 0x14] ; sub eax, 5 ; cmp eax, 1 ; ja <fail>
                                              ; nInventoryType must be 5 or 6
140d75125  call 0x1402e3cd0                   ; the CHARACTER's slot (type, slot)
140d75140  cmp  [r14 + 8], rax                ; request SN == item + 0x38
140d75146  lea  rcx, [rbx + 0x20] ; call 0x14019a5d0   ; -> nItemID
140d75293  lea  eax, [r15 - 0x4c4b40] ; cmp eax, 0x2710 ; the 5000000..5009999 pet range
```

The wire:

```asm
140d753d8  w_u8   0x0b
140d753e9  w_raw  8  from [r14 + 8]      ; liCashItemSN
140d753f6  w_u32     from r15d           ; nItemID  (from the inventory item, not the request)
140d75402  w_u8      from [r14 + 0x14]   ; nInventoryType (5 or 6)
140d75410  w_u32     from [r14 + 0x18]   ; nSlotPosition   <- u32, NOT u16
140d75426  mov  byte [r13 + 0x74], 1
```

**The slot is a `u32` here and a `u16` in `0x0A`.** Read off both listings; the asymmetry is
real. [L]

### 4.1 Its success reply: `0x05AE` sub-op `0x1B`

`FUN_140D80410`, same proof: [L]

```asm
140d80437  xor  r15d, r15d
140d80447  mov  byte [rsi + 0x74], r15b     ; latch cleared UNCONDITIONALLY, at the head
140d811bd  cmp  dword [rsi + 0x120], 5      ; only acts if the in-flight kind is 5
140d811c6  mov  dword [rsi + 0x120], r15d
140d811cd  call 0x140d74a70                 ; pump
```

All four direct reads, enumerated: [L]

```text
0x05AE
  u8      nResult = 0x1B
  u8      bSuccess       140d80461   ; ZERO branches to a short failure path at 0x140D8103C
  u8[8]   liCashItemSN   140d80479
  u32     nItemID        140d80481   ; -> 0x1403E8AF0 -> inventory type
  u32     <count/slot>   140d80496   ; feeds a message-formatting block; meaning NOT established
```

**`cash-shop-stage.md` §6.2 records the last field as `u16`. It is a `u32`** —
`0x1406E8C20`, which `tools/reads.py`'s `PRIM` table maps to `u32`. [L]

---

## 5. `0x1C` — DELETE

Builder `FUN_140D75480`, 558 bytes, request kind 6. [L] The identification is not an
inference from a number: it is the client's own text.

```asm
140d754ce  cmp  dword [rdx], 6
140d75501  call 0x1410b6ca0                  ; find the item in the locker by (SN, itemId)
140d7558a  call [rax + 0x48]  ; jne <fail>   ; two virtual predicates must both be false
140d7559b  call [rax + 0x40]  ; jne <fail>
140d755a8  call 0x14038a480(global, itemId)  ; must return 0
140d755b1  cmp  byte [rdi + 0x43], al        ; cashItem+0x63 (record +67) must be 0
140d755b6  mov  edx, 0x28b                   ; else: string 651
140d75618  mov  edx, 0x2b6                   ; the other failure: string 694
```

Decrypted with `tools/dump_stringids.py`: [L]

* **651 — "A refundable item cannot be deleted."**  → record offset **+67 is a refundable flag**
* **694 — "This item cannot be deleted."**

The wire is the shortest in the family:

```asm
140d755d2  w_u8   0x1c
140d755e4  w_raw  8  from rdi = &cashItem[0x20]   ; liCashItemSN
140d755fb  mov  byte [rsi + 0x74], 1
```

### 5.1 Its success reply: `0x05AE` sub-op `0x3C`

The `0x3C` arm, inside `FUN_140D7DCA0` at `0x140D7DF3C`: [L]

```asm
140d7df3c  READ raw[8]                       ; the SN that was deleted
140d7df4e  cmp  dword [rdi + 0x120], 6       ; only acts if the in-flight kind is 6
140d7df55  je   140d7df64
140d7df57  call 0x140d74c70                  ; wrong mode -> cancel everything
140d7df64  cmp  qword [rbp + 0x40], -1       ; -1 = a sentinel, "nothing was removed"
140d7df6b  lea  rcx, [rdi + 0x150] ; call 0x140d85c10   ; ERASE from the locker map by SN
140d7dfae  cmp  qword [rdi + 0x148], 0       ; queue not empty -> exit
140d7dfbc  mov  edx, 0x2b8                   ; string 696
```

**696 — "The cash item has been deleted."** [L]

```text
0x05AE
  u8      nResult = 0x3C
  u8[8]   liCashItemSN        (or -1 to mean "removed nothing")
```

### 5.2 The refusal for this whole family: `0x05AE` sub-op `0x3D`

At `0x140D7DFC6`: [L]

```asm
140d7dfc9  READ u16                     ; the reason
140d7dfce  movzx edx, ax ; call 0x140d7c7f0   ; the §6.3 failure-message table
140d7dfd9  call 0x140d74c70                  ; cancel the queue
140d7dfe1  mov  byte [rdi + 0x74], 0         ; clear the latch
```

```text
0x05AE
  u8      nResult = 0x3D
  u16     nReason      (truncated to u8 into cash-shop-stage.md §6.3's table; 2 = generic)
```

This is the correct "no" for `0x0A` / `0x0B` / `0x1C`: it shows a message, clears the latch,
and empties the queue in one packet. **[L] on every one of those four effects.**

---

## 6. `0x2B` — QUERY A COMMODITY'S STOCK

Builder `FUN_140D7ADC0`. This one is different from the other four in a way that matters. [L]

```asm
140d7adf4  test edx, edx ; jne 140d7afd4     ; arg 0 -> a different, non-sending path
140d7afd4  lea  r8, [rcx + 0x188]            ; a hash map keyed by the id
140d7b000  cmp  dword [rax + 0x10], ebx      ; already cached? -> no packet at all
140d7b090  ...  cmp edi, 0x7fffffff          ; cached as "pending"? -> no packet
140d7b01f  mov  dword [rax], 0x7fffffff      ; insert the pending sentinel
140d7b035  w_u8   0x2b
140d7b041  w_u32     from ebx                ; commoditySN
140d7b05d  SEND
```

**`0x2B` sets no in-flight latch and no mode.** `python tools/listing.py 0x140d7adc0 | grep
"0x74]"` returns nothing, and the same pattern in the same session returns both sites in
`FUN_140D74E10` — a positive control beside the negative. [L] So `0x2B` is fire-and-forget:
it never blocks the UI, and an unanswered one costs a cache entry stuck at `0x7FFFFFFF`, not
a frozen client.

> This **corrects `cash-shop-stage.md` §5.1**, which states "Every `0x03E1` builder sets the
> same `[this+0x74]` latch". Five of the six do; `0x2B` does not.

What the number is: the caller `FUN_140D7A700` passes `[[commodityEntry+8]+0x20]`
(`0x140D7A79D`), and the cache-hit path feeds the same value to `FUN_142D47770` — the
commodity-by-`commoditySN` lookup that the buy builder also uses at `0x140D7A3FB`. So the
`u32` is a **commoditySN**. [D] The client then computes `[commodityEntry + 0xCF] - cached`
and clamps at 0 (`cmovg`), i.e. a **remaining-stock count**. [L]

### 6.1 Its reply: `0x05AE` sub-op `0x57`

`0x140D7E027`, which falls into the shared cache-write block at `0x140D7E046`: [L]

```asm
140d7e02a  READ u32 -> ebx        ; the key
140d7e037  READ u32 -> esi        ; the value
140d7e040  test ebx, ebx ; je <ignore>       ; key 0 is ignored by 0x57
140d7e051  call 0x140d83b00                  ; cache[key]
140d7e056  mov  dword [rax], esi             ; = value
```

```text
0x05AE
  u8    nResult = 0x57
  u32   commoditySN     (must be non-zero, or the packet does nothing)
  u32   count
```

Two neighbours share that block and are worth knowing: **`0x55`** is the same pair but a
key of `0` sets the global "any limited stock left" flag `[stage+0x180]`; **`0x58`**
(`u8, u32 key`) writes the `0x7FFFFFFF` sentinel back, i.e. a server push meaning *"that
count is stale, ask again"*. [L]

---

## 7. `0x03` — GIFT, and what I could not settle

Builder `FUN_140D7B240`. The shape confirms §5.2's guess. [L]

```asm
140d7b56d  call 0x142d47770                  ; commodity by SN (r14d)
140d7b586  lea  rcx, [rbx + 0x28] ; call 0x14019a5d0    ; -> nItemID
140d7b649  call 0x1410c4ca0                  ; construct a modal, seeded with the SN
140d7b657  call 0x14177f640                  ; run it  (DoModal)
140d7b665  call 0x1410c5c20 -> [rsp+0x40]    ; two strings out of the dialog
140d7b67d  w_u8   0x03
140d7b68b  w_u32     from r14d               ; commoditySN
140d7b69a  w_str     from [rsp + 0x40]       ; recipient character name
140d7b6ab  w_str     from [rsp + 0x48]       ; gift message
140d7b6c8  mov  dword [rdi + 0x120], 2       ; mode = 2
```

```text
0x03E1
  u8      0x03
  u32     commoditySN
  str     recipient character name
  str     gift message
```

Two measured facts, and one negative I am **not** turning into a recommendation:

* **The gift builder never writes `[stage+0x74]`.** It reads it as a guard at `0x140D7B278`
  and writes only `[stage+0x120] = 2`. A loose `grep "0x74]"` over its whole listing returns
  that single read. [L]
* **Nothing in the reachable class compares `[stage+0x120]` against `2`.** `tools/rangescan.py
  0x120 0x140d70000 0x140d90000` enumerates **35 sites**; the comparisons are against `0`,
  `4`, `5` and `6` only, and the two `lea` forms it also reports
  (`0x140D7011B`, `0x140D72D03`) are `[rbp+0x120]` frame locals in unrelated functions, not
  the stage. The known writes at `0x140D7A500` and `0x140D7B6C8` both appear, so the scan is
  not silent. [L]

Taken together those say mode 2 is set by the gift and cleared only by `FUN_140D74C70` —
which is reached from `0x05AE` sub-ops `0x1A` and `0x1C`, and from the pump's failure exit.
Since every builder in the family guards on `[stage+0x120] == 0`, a gift that is never
answered with a cancelling packet **leaves the client unable to buy, gift, move or delete
anything for the rest of the session**, while the UI still looks responsive because `0x74`
was never set. **[D]** — and it is a derivation, not a measurement, because I did not find
the packet the client's authors intended here.

**`Res_Gift_Done(23)` = `0x17` is dead in this build.** `cash-shop-stage.md` §6.2's dumped
byte-index table gives `0x17` and `0x18` the default slot, i.e. silently ignored. So the
v214 name does not transfer, and I could not identify a live arm that means "gift sent".
See §10.

---

## 8. Which v214 candidate names I killed

`CLAUDE.md` scores `ModernMapleSource` **1 of 8**. I enumerated the whole `CashItemType`
enum rather than working from the five names in the brief, generated candidates from it, and
then confirmed or killed each against the builder. Result: **the reference is wrong on every
one of the five numbers, and right about two of the concepts.**

| sub-op | v214 name at that number | verdict | what killed or confirmed it |
|---|---|---|---|
| `0x03` = 3 | `Req_Gift(3)` | **confirmed** | the builder writes `u32, str, str` and its modal returns two strings [L] |
| `0x0A` = 10 | `Req_EnableEquipSlotExt(10)` | **killed** | the builder moves one cash item into an empty character-inventory slot; nothing about equip-slot extension [L] |
| `0x0B` = 11 | `Req_CancelPurchase(11)` | **killed** | the builder reads a character inventory slot, matches its cash serial, and its failure string is *"This item cannot be moved."* [L] |
| `0x1C` = 28 | `Req_SendMemo(28)` | **killed** | no string field at all; failure strings are *"…cannot be deleted."* [L] |
| `0x2B` = 43 | `Req_ShopOptionScan(43)` | **killed** | one `u32` commoditySN into a stock-count cache [L] |
| `0x0A`/`0x0B` | `Req_MoveLtoS(14)` / `Req_MoveStoL(15)` | **concepts confirmed, numbers wrong** | both are marked `// v263` in the enum, i.e. the numbering that shifted; this build has them at 10 and 11 [D] |
| `0x1C` | `Req_Destroy(32)`, `//Req_Destroy(13)` | **concept confirmed, number wrong** | string 696 *"The cash item has been deleted."* [L] |
| reply `0x19` | `Res_IncSlotCount_Done(25)` | **killed** | `0x19` gates on in-flight kind **4** and erases from the locker map [L] |
| reply `0x17` | `Res_Gift_Done(23)` | **killed** | `0x17` is in the dead set of the byte-index table [L] |

The direction/assignment of `0x0A` vs `0x0B` does **not** rest on the reference at all. It
rests on two things read out of the client: `0x0A` requires its destination inventory slot to
be **empty** and finds its item in the locker, while `0x0B` finds its item in the character's
inventory and its own failure string is *"This pet cannot be moved **to the locker** while
equipped."* (string 1834). [L]

---

## 9. UI entry points — what to click

| sub-op | entry point | gesture |
|---|---|---|
| `0x0A` | `FUN_1410B43E0` builds kind 4; called from `FUN_1410B4220`, `FUN_1410B4373`, `FUN_1410B50E0`, `FUN_1410B6330`, each of which then calls the pump | a locker item moved to the inventory. `FUN_1410B4220` is a window-message handler (`sub eax, 0x201`). The selected item's serial is read from `[ui + 0x11B8]` [L] |
| `0x0B` | `FUN_1410CFA30`, a vtable slot at `0x14337EDF8`; `edx == 0x203` (`WM_LBUTTONDBLCLK`) → `FUN_1410CFE70` builds the request → `FUN_140D749A0` enqueue → pump. Also `FUN_1410CFE03`, `FUN_1410D1CA0` | **double-click a cash item in the character's inventory** [L] |
| `0x1C` | `FUN_1410B6490` — writes kind `6` at `0x1410B65EE`, loops over a multi-selection (`add rbx, 0x10`), and shows string **540** *"The selected item will be deleted. Are you sure you want to delete this item?"* at `0x1410B666C` | the locker's delete button, with a confirm dialog, over one **or many** selected items [L] |
| `0x2B` | `FUN_140D7A700`, two sites; only `0x140D7A79D` sends (the other passes `edx = 0`, which takes the non-sending path) | automatic, on drawing a limited-stock commodity [D] |
| `0x03` | the modal built at `0x1410C4CA0` / run at `0x14177F640` / read at `0x1410C5C20` | the gift dialog. **The builder itself has zero callers** by call, tail-jmp and data-pointer scan — the known indirect-call blind spot, not evidence of absence [L] |

`FUN_1410BCDA0`, the cash shop window's button dispatcher named in `cash-shop-stage.md` §7
(`exit checkCash chargeCash cartSelectAll cartDelete cartBuy`), does **not** appear on any of
these paths. Those are the main-window buttons; the locker and inventory windows are separate
classes in the `0x1410Bxxxx` / `0x1410Cxxxx` range. [D]

---

## 10. What this file does not establish

* **Nothing here has been on a wire.** Every claim is static. The only measurement in the
  whole cash-shop chain is the buy body in §1.1.
* **The gift's success reply.** `Res_Gift_Done(23)` is dead in this build (§7) and I did not
  find a live `0x05AE` arm that gates on mode `2`. The consequence in §7 — that an
  unanswered gift wedges every later operation — is **[D]** from an enumeration, not a
  measurement, and the cheapest way to settle it is one client run: send `0x03`'s refusal as
  `0x1A` and see whether a subsequent locker move still works.
* **`0x3C` does not clear `[stage+0x74]` and does not pump.** Read off its listing: the arm
  erases from the locker map, refreshes two windows, and exits — `0x140D7DFB6` jumps to the
  common exit when the queue is non-empty. `0x19` and `0x1B` both clear the latch explicitly
  (`xor r12d,r12d` / `xor r15d,r15d` at their heads); `0x3C` has no such write in the
  `tools/rangescan.py 0x74` enumeration of the stage range. **Named blind spot:** the two UI
  refresh functions it calls, `0x1410B5540` and `0x1410D0FD0`, have **no `.pdata` entry** and
  `listing.py` refused both (0 instructions), so I could not check whether either reaches the
  stage. Until that is closed, a server should follow `0x3C` with `0x3D` or re-send the
  wallet, and **a delete is the one operation in this family I would not ship first.**
  **Closed 2026-09-24:** `research/cash-shop-buy-done.md` section 3 lists `0x3C` as clearing
  `+0x120` but not `+0x74`, so the server now sends `0x3C` followed by `0x05AD`, whose arm clears
  `+0x74` at `0x140D736DC` and re-triggers a purchase only when `+0x120` is 1 - which `0x3C` has
  just reset. `session/cashshop.rs::on_delete_cash_item`.
* **`0x1B`'s fourth `u32`.** It feeds a string-formatting block (`0x1408BC4C0`); whether it
  is a count, a slot or a remaining quantity is not established. Offsets yes, meaning no.
* **`0x0A`'s `u32` at request `+0x10`.** It is `cashItem + 0x30` (record offset +16) and it
  is used as the second half of the locker lookup key. I call it `nItemID` because
  `FUN_1410B6CA0` keys on it beside the serial and because `0x0B`'s corresponding field
  provably *is* an item id — but for `0x0A` that is **[I]**, one step short of the listing.
* **`[cashItem + 0x63]`** (record offset +67) is a refundable flag on the strength of string
  651 alone. [D]
* **Where `[stage+0x168]` comes from** — the buy's `u32` field, measured as `2`. Not chased.
* Whether the gift's age/price gate (`[commodityEntry + 0x44]` vs `FUN_1401BA9D0`) matters
  server-side. It is entirely client-side as far as this pass went.

---

## 11. WIRE IT LIKE THIS

Nothing below is wired. `crates/` was off-limits for this task.

### 11.1 The locker → cash tab move, end to end

```text
client -> 0x03E1  u8 0x0A, u8[8] SN, u32 itemId, u8 invType, u16 slot

server:  verify the SN is in this account's locker and the slot is free, then

server -> 0x05AE  u8 0x19
                  u8    1                 <- MUST be non-zero or the UI stays blocked
                  u16   slot              <- echo the client's slot
                  u8    itemType + <GW_ItemSlot>    (the same encoder the inventory uses)
                  u8    0
```

The client erases the item from its locker map itself, refreshes both windows, clears the
latch, sets mode 0 and pumps the next request. No second packet is needed. [D]

To refuse instead:

```text
server -> 0x05AE  u8 0x3D, u16 2      ; "Due to an unknown error…", latch cleared, queue emptied
```

### 11.2 The other three

```text
0x0B  ->  0x05AE  u8 0x1B, u8 1, u8[8] SN, u32 itemId, u32 <count>
0x1C  ->  0x05AE  u8 0x3C, u8[8] SN            (see the §10 caveat before shipping this)
0x2B  ->  0x05AE  u8 0x57, u32 commoditySN, u32 count
```

`0x2B` may also be ignored safely — it sets no latch (§6). It is the only one of the five
where "always answer" is not load-bearing, and even so answering it is one packet.

### 11.3 What falsifies each step

1. **Move one item out of the locker.** *Falsified if:* the client draws the item in the wrong
   slot — then the `u16` is being read at the wrong offset, or the item body is short. *Also
   falsified if:* the cash shop UI goes dead but the window still paints — that is
   `bClearLatch` sent as `0`.
2. **Immediately move a second item.** *Falsified if:* nothing happens — then `[stage+0x120]`
   was not returned to 0, i.e. the reply reached the wrong arm and §3.3's mode-4 gate is
   wrong.
3. **Double-click an item in the character inventory.** *Expected:* `0x03E1 0x0B` in
   `world.log`. *Falsified if:* nothing is sent — then §9's gesture is wrong.
4. **Refuse a move with `0x3D`.** *Expected:* a message box, and the shop still usable.
   *Falsified if:* the client ejects — then `0x3D` is not this family's refusal.

Steps 1–2 are one launch and settle §2, §3 and §11.1 together.
