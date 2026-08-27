# There is no `Buy_Done` in this build. What to send instead, and why it is a pair

2026-08-26. **Static only — no client run was spent on this.** It rests on one live capture
(`research/fixtures/`, the 2026-08-26 run) for the *request* side and on the image for
everything else.

Tags: **[L]** read out of the image or a log, **[D]** derived from something read, **[I]**
inferred.

This file answers the question `research/cash-shop-stage.md` §6.6 left open: *which `0x05AE`
sub-op tells the client a purchase succeeded?* The answer is that **none of them does**, and
the reason is structural rather than a missing opcode number. §3 below enumerates all 23 live
arms against the two fields that block the UI, which is the measurement §6.6 did not have.

---

## 0. The short version

**Send two packets, in this order.** [D], from [L] parts:

```text
1)  0x05AE   u8 0x19                     <- releases BOTH latches, adds the item, SILENT
              u8  1                          bRelease: non-zero clears [stage+0x74]
              u16 nPOS                       1-based slot in the tab implied by the item id
              <GW_ItemSlot>                  u8 nType(1|2|3) + the standard item body
              u8  0                          bEffect: non-zero fires effect 0x3EE

2)  0x05AD   u32 nxCredit                 <- the DEBITED balance. Must come SECOND.
             u32 maplePoints                 While [stage+0x120] == 1 this packet
             u32 0                           RE-SENDS the purchase (0x140D736F6).
```

* **`0x19` is the only silent arm that clears both `[stage+0x74]` and `[stage+0x120]` and
  hands over an item.** `0x1B` is the only other silent one and it carries a crash/data-loss
  hazard (§4.3). Every other arm that clears the latch calls `FUN_140D7C7F0`, and **all 127
  of its reason codes are now mapped and all 127 are error text** (§4.1) — so §6.2.1's "there
  is no silent refusal" is confirmed, and extended: there is no silent *reason* either.
* **`0x05AE` sub-op `0x03` (`Res_AddedCashItem_Done`) is optional and is not the release.**
  It puts the record in the client's cash locker and touches neither latch, exactly as §6.6
  said. Send it only if you want the item to appear in the *locker* panel as well; `0x19`
  alone already puts it in the player's inventory. §5 has the 71-byte record either way.
* **`0x05AD` must be second.** The wallet arm reads `[stage+0x120]`, and while it is `1` it
  calls the buy builder back. `0x19` sets it to `0` first, which makes the wallet inert.

**Correction to `cash-shop-stage.md` §6.2.1.** That section says *"Every `0x05AE` arm that
clears the latch puts a message on screen."* That is true of the six arms whose bodies are
**inline** in `FUN_140D7DCA0`, which is the set it enumerated. It is false for the two that
**delegate to a sub-function**: `0x19` (`FUN_140D7F8A0`) and `0x1B` (`FUN_140D80410`) clear
the latch and call neither `FUN_140D7C7F0` nor `FUN_140D84B70`. Two independent instruments
agree on that negative (§1).

---

## 1. Instruments, and the controls that were run first

Everything below was run **with the repo as the working directory** (`python tools/...`),
because `CLAUDE.md`'s scratchpad-shadowing section applies: the session scratchpad holds a
stale `reads.py` without the eleventh primitive.

| instrument | control | result |
|---|---|---|
| `tools/reads.py` | `0x140304100 2` must show reads via `FUN_1403035a0` **and** `FUN_140303b40` **and** direct | passed — `0x14030411a`, `0x140304126`, then `0x140304138 raw` |
| `tools/listing.py` | `0x140304100 \| grep READ` must match those addresses | passed — `140304138 raw`, `140304144 u8`, `140304183 u8`, then the `u16` run |
| `tools/reads.py` `PRIM` | must contain the **f64** reader `0x1406e8fb0` | passed — 11 entries, and the import resolves to `C:\MapleCW\tools\reads.py` (printed, not assumed) |
| `tools/callers.py` | `0x1402fa9a0` must give 96 sites | passed — 96 sites in 15 functions |
| `tools/rangescan.py` | `0x74` over `0x140d70000..0x140d90000` must return a non-empty set before any empty one is believed | passed — 41 sites; the same range returns **0** for `0x47`, `0x53`, `0x3a`, `0x4f` |
| `tools/rtti.py` | `--list CLogin` must find `CLoginQueueDlg` | passed — so the **absence** of any `Cash` class name is a property of the image, not of the tool |
| `tools/dump_stringids.py` | id 661 must be *"Due to an unknown error…"* | passed, exactly |

**The load-bearing negative — "`0x19` and `0x1B` show no message" — was taken twice, with
instruments that do not share a blind spot:**

* `tools/listing.py` over each function's `.pdata` extent, grepping for `140d7c7f0`,
  `140d84b70`, `1408a9e40`. Blind spot: a wrong extent.
* `tools/callers.py 0x140d7c7f0` and `0x140d84b70`, which scan the **whole image** for
  `call`, tail `jmp` and data pointers. `FUN_140D7F8A0` and `FUN_140D80410` appear in neither
  caller list, in any of the three modes. Blind spot: an indirect call through a register.

Both come back the same way, and the second one cannot be fooled by a mis-scoped extent.
**[L]** for the two lists; the residual blind spot — a message raised through a virtual call
— is named here and is not closed.

Ghidra was not used; another agent holds the single-process lock.

---

## 2. The state machine that actually blocks the UI

Two fields, not one. This is the part §6.2.1 had half of.

### 2.1 `[stage+0x74]` — the in-flight latch (a byte)

Set to `1` by every outbound request builder; cleared by the arms in §3. **[L]**

### 2.2 `[stage+0x120]` — the *pending operation type* (a dword)

Not a boolean. `tools/rangescan.py 0x120 0x140d70000 0x140d90000` gives 35 sites; the writers
are: **[L]**

```text
140d71dda  ctor                        = 0
140d7a500  FUN_140D785F0  (buy)        = 1
140d7b6c8  FUN_140D7B240  (gift)       = 2
140d74c49  FUN_140D74A70  (drainer)    = the popped request's type
140d74ddb  FUN_140D74C70  (cancel)     = 0
140d736eb  0x05AD arm, when it was 1   = 0, then RE-SENDS the buy
140d73702  0x05AD arm, when it was 6   = 0, then drains
140d80351  0x19 arm, when it was 4     = 0
140d811c6  0x1B arm, when it was 5     = 0
```

The type numbers pair one-for-one with the outbound `0x03E1` sub-ops. `FUN_140D74A70` is a
queue drainer over a **`std::deque` at `[stage+0x128]`** (`_Map` `+0x130`, `_Mapsize`
`+0x138`, `_Myoff` `+0x140`, `_Mysize` `+0x148`), and it switches on the popped request's
first dword: **[L]**

| `[+0x120]` | outbound `0x03E1` sub-op | builder | the `0x05AE` arm keyed to it |
|---:|---|---|---|
| 1 | `0x02` / `0x1F` **Req_Buy** | `FUN_140D785F0` | **none** — see §4 |
| 2 | `0x03` Req_Gift | `FUN_140D7B240` | `0x13` clears it, but unconditionally |
| 4 | `0x0A` | `FUN_140D74E10` | **`0x19`** (`cmp [+0x120], 4` at `0x140D80348`) |
| 5 | `0x0B` | `FUN_140D75080` | **`0x1B`** (`cmp [+0x120], 5` at `0x140D811BD`) |
| 6 | `0x1C` | `FUN_140D75480` | **`0x3C`** (`cmp [+0x120], 6` at `0x140D7DF4E`) |

`FUN_140D74A70`'s first two instructions are what makes `0x19` usable for a buy: **[L]**

```asm
140d74a82  cmp qword [rcx+0x148], 0 ; je 140d74b40    ; deque empty      -> cancel everything
140d74a90  cmp dword [rcx+0x120], 0 ; jne 140d74b40   ; something pending -> cancel everything
...
140d74b40  mov rcx, rbx ; call 140d74c70              ; = [+0x120] = 0, deque emptied
```

So calling the drainer with a pending **buy** and an empty queue clears `[+0x120]`. `0x19`
calls it **unconditionally**, at `0x140D8035B`.

### 2.3 Both must be zero for the next request

Eight of the ten request entry points in the stage refuse at their first instruction while
**either** field is non-zero: **[L]**

```text
FUN_140D74E10  140d74e41 / 140d74e4d      FUN_140D75080  140d750b6 / 140d750c3
FUN_140D75480  140d754b7 / 140d754c4      FUN_140D76090  140d760c3 / 140d760cd
FUN_140D76570  140d765a1 / 140d765ab      FUN_140D76F50  140d76fab / 140d77004
FUN_140D78070  140d78096 / 140d780a0      FUN_140D7B240  140d7b278 / 140d7b282
```

`FUN_140D78070` — the cart/buy entry — is the one that matters here:

```asm
140d78096  cmp byte  [rcx+0x74],  0 ; jne <ret>
140d780a0  cmp dword [rcx+0x120], 0 ; jne <ret>
```

`FUN_140D785F0` itself only *writes* `[+0x74]` (`0x140D7A4FC`); it never reads it. **So
clearing only the byte would leave the shop dead anyway.** That is the whole reason this is
a pair and not a single packet.

---

## 3. Every live `0x05AE` sub-op, against the three questions

Both tables were re-dumped rather than taken from §6.2 (`tools/dump_va.py 0x140D7E194 96`
for the byte index, `0x140D7E13C 88` for the 22 targets) and decode identically to §6.2's —
an independent reproduction, not a copy. **[L]**

Columns: **+0x74** = clears the in-flight latch. **+0x120** = clears or consumes the pending
operation. **item** = hands the client an item. **msg** = puts text on screen. **exit** =
sends `0x00D1` and drops the stage.

| sub-op | body at | +0x74 | +0x120 | item | msg | exit |
|---|---|---|---|---|---|---|
| `0x03` | `140d7dcfd` inline | – | – | **locker** | – | – |
| `0x04` | → `FUN_140D7E1F0` | – | – | **locker** | yes (`140d7e3f4`) | – |
| `0x05`, `0x07` | `140d7de31` inline | – | – | – | yes | **yes** |
| `0x06` | → `FUN_140D819C0` | – | – | – | builds | on a branch |
| `0x0C` | → `FUN_140D7E5E0` | – | – | **locker** | – | – |
| `0x0D` | → `FUN_140D7EAE0` | conditional (`r13b`) | **yes** `140d7eb04` | – | yes ×6 | on a branch |
| `0x13` | → `FUN_140D81590` | **yes** `140d815b7` | **yes** `140d815bf` | – | yes, str 594 *"You have given…"* | – |
| `0x14` | `140d7e0b9` inline | **yes** `140d7e0c3` | **yes** `140d7e0c7` | – | yes | if reason ≤ 2 |
| **`0x19`** | → `FUN_140D7F8A0` | **yes, iff the leading `u8` ≠ 0** `140d7f8fa` | **yes, always** `140d8035b` | **inventory** | **none** | – |
| `0x1A`, `0x1C` | `140d7de81` inline | **yes** `140d7de93` | **yes** `140d7de8b` | – | yes | – |
| **`0x1B`** | → `FUN_140D80410` | **yes** `140d80447` | **yes** `140d811d4` | **locker** (`u8`=0 branch) | **none** | – |
| `0x1D` | `140d7debf` inline | **yes** `140d7debf` | – | removes | yes, str 598 | – |
| `0x1E` | `140d7de93` inline | **yes** | – | – | yes | – |
| `0x3C` | `140d7df3c` inline | – | **yes** | removes (if `+0x120`=6) | yes, str 696 | – |
| `0x3D` | `140d7dfc6` inline | **yes** `140d7dfe1` | **yes** `140d7dfdc` | – | yes (`u16` reason) | – |
| `0x3E` | → `FUN_140D7F020` | – | – | **locker ×2** | builds | – |
| `0x3F` | → `FUN_140D7F550` | **yes** `140d7f571` | **yes** `140d7f56c` | – | yes ×2 | on a branch |
| `0x55` | `140d7dfea` inline | – | – | – | – | – |
| `0x56` | `140d7e01a` inline | – | – | – | – | – |
| `0x57` | `140d7e027` inline | – | – | – | – | – |
| `0x58` | `140d7e06f` inline | – | – | – | – | – |
| 63 others in `3..0x58` | `140d7e128` | – | – | – | – | – |

**Read that table down the two silent columns and only two rows survive: `0x19` and `0x1B`.**

"locker" means the object is inserted into `[stage+0x150]` by `FUN_140D75850`, a
`std::map<u64 liSN, shared_ptr<CashItemInfo>>` keyed on the record's first 8 bytes. **[L]** —
`FUN_140D75850` reads `[obj+0x20]` (record `+0`) into `rdx` at `0x140D75880` and walks a
red-black tree at `stage+0x150` comparing `[node+0x20]` against it. Note `0x140D75884`:
`cmp rdx, -1 ; je <return>` — **an object whose serial is `-1` is dropped without being
added.**

"inventory" means `FUN_1402E4C20(character, nTab, nPOS, &item)` — the real character
inventory, not the locker.

### 3.1 `0x19` read byte for byte

`FUN_140D7F8A0`, `0x140d7f8a0..0x140d803b8`. All four reads are on the unconditional path:
`0x140D7F8F8`'s `je` and `0x140D7F95C`'s `je` both land at `0x140D7F97B`, immediately before
the last read. **[L]**

```asm
140d7f8d3  call 142aa2810 (ecx=2)              ; the wait-cursor / modal UI call [I]
140d7f8e6  rdi = FUN_142CBE730(g_ctx)          ; the character
140d7f8f1  READ u8    -> al
140d7f8f8  je 140d7f8fe                        ;   al == 0 -> DO NOT clear the latch
140d7f8fa  mov byte [rsi+0x74], 0              ;   al != 0 -> clear it       (r12d = 0)
140d7f901  READ u16   -> [rbp-0x41]            ; nPOS
140d7f913  call 140303530(&[rbp-0x39], pkt)    ; <<< a whole GW_ItemSlot
140d7f936  itemId = FUN_14019A5D0(item+0x20)
140d7f954  cmp qword [item+0x38], -1 ; je      ;   the item's own cash serial
140d7f965  call 140d85c10(stage+0x150, &sn)    ;   erase that serial from the locker map
140d7f97e  READ u8    -> r14d = (al != 0)      ; bEffect
140d7f9b4  tab = FUN_1403E8AF0(itemId)         ; = itemId / 1000000, with one exception
140d7f9f5  eax = FUN_1402E4C20(char, tab, nPOS, &item)
140d7f9fa  test eax,eax ; jne 140d802b2        ; NON-ZERO = placed. everything from
           ...                                 ; 140d7fa02..140d802b1 is the ELog-only
           ...                                 ; failure report - no dialog.
140d80319  FUN_142CE70C0(g_ctx, itemId, nPOS)
140d80321  if bEffect: FUN_142A37120(0x3EE, ...)
140d80348  cmp dword [rsi+0x120], 4 ; jne      ;   == 4 -> zero it here
140d80351  mov dword [rsi+0x120], 0
140d8035b  call 140d74a70                      ; <<< UNCONDITIONAL. With a pending buy and
                                               ;     an empty deque this reaches FUN_140D74C70
                                               ;     and zeroes [+0x120]. §2.2.
```

**What `0x19` really is.** Its counterpart builder is `FUN_140D74E10`, whose send site is
`0x03E1 / u8 0x0A / raw[8] serial / u32 itemId / u8 tab / u16 slot` (`0x140D74F79`..
`0x140D74FBD`) — built by the queue drainer after `FUN_14022FAE0(char, tab, 0)` finds a free
slot, with *"Your inventory is full."* (str 155) if it does not. So the pair is
**"move this locker item into inventory slot N"** and `0x19` is its confirmation. **[D]**

It is therefore a slight semantic stretch to use it for a purchase — the client will show the
item in the player's Cash/Equip tab rather than in the shop's locker panel. That is
cosmetically wrong and mechanically harmless: the item's real home is the database, and the
`0x01A0 SetField` that answers `0x00D1` re-sends the character record from there.

### 3.2 The `GW_ItemSlot` body, and the one way it kills the client

`FUN_140303530(&out, pkt)` is 101 bytes: **[L]**

```asm
140303543  READ u8  nType
140303550  FUN_1402CC180(&sp, nType)      ; the factory
14030355e  jne 140303572                  ; a NULL result -> out.ptr = null, RETURN
140303578  call [vt+0x358](pkt)           ; the type's own decoder
```

`FUN_1402CC180` allocates by type and **returns null for anything else**: **[L]**

| `nType` | size | ctor | vtable |
|---:|---:|---|---|
| 1 | `0x467` | `FUN_1402F7DA0` | equip |
| 2 | `0x7E` | `FUN_1402F7CD0` | **`0x14327E588`** |
| 3 | `0xCE` | `FUN_1402F8B40` | pet |

`0x14327E588` is the same vtable `crates/net/src/bag.rs` already documents for the type-2
bundle body, and `vt+0x358` is the same `FUN_140304450`. **So the server's existing
`equipped_item()` (type 1, proven on a live client) and `bundle_item()` (type 2, built but
never on a wire) produce exactly the bytes `FUN_140303530` will read.** [L] for the vtable
identity, [D] for "therefore the encoders match".

> **A type byte outside 1..3 crashes the client.** The factory returns null,
> `FUN_142E52ED0(0x431)` only *reports* the null, and `0x140D7F932` then does
> `add rcx, 0x20 ; call 0x14019A5D0` on it — a read at address `0x20`. **[L]**

---

## 4. Why there is no `Buy_Done`, said three ways

### 4.1 There is no silent `nReason`, and now all 127 are mapped

`cash-shop-stage.md` §6.3 mapped 126 of 127 cases and flagged `0x49` as unmapped because its
case head does not begin with `mov edx, imm32`. Re-extracted by scanning each case body up to
its first `mov edx, imm32` (rather than only its first instruction), **all 127 resolve**:

* `0x49` → string id **17**, which decrypts to a `https://private.api.nexon.com/blockcashout/...`
  URL used as a format argument, i.e. still a message.
* 44 of the 127 fall through to the default at `0x140D7D3A3`, string **661**, the generic
  error.
* The remaining 83 load a specific id. **Every one of the 37 ids spot-decrypted is error
  text** — refunds, coupons, gender restrictions, purchase limits. Not one reads as a
  success, and none decrypts to an empty string.

`FUN_140D7C7F0`'s exit at `0x140D7D92F` calls `FUN_140D84B70(&str, 0)` **unconditionally**,
and `FUN_140D84B70` has no empty-string guard — it copies and calls `FUN_142A26280`. **[L]**

So `0x05AE 0x1A <reason>` cannot be dressed up as a success. §6.2.1 was right.

### 4.2 The v214 name that survives is not the release

`Res_AddedCashItem_Done(3)` still matches our `0x03` — `u16 count` then that many records —
and §6.6's conclusion stands unchanged: it touches neither latch and cannot release the UI.
What §6.6 could not say is what *would*, and §3's table answers it: only `0x19` and `0x1B`.

Both of those, on the v214 numbering, are `Res_IncSlotCount_Done(25)` and an unnamed `27`.
Neither name describes what the code does — `0x19` moves an item into an inventory slot and
`0x1B` moves one back — which is the fourth independent demonstration that the `// v263`
half of that enum does not transfer. It is recorded here so nobody re-derives it.

### 4.3 `0x1B` is the other silent arm, and it is a trap

`FUN_140D80410`, `u8 bToSlot` first:

* `bToSlot != 0` → `raw[8] serial, u32 itemId, u32 nPOS`, then it *verifies* the item already
  at `(tab, nPOS)` has that serial.
* `bToSlot == 0` → **a full 71-byte cash-item record**, added to the locker map by
  `FUN_140D75850`, then `FUN_140230CB0(char, tab, serial)` looks the serial up in the
  **character's** tabs 5 and 6.

For a freshly purchased item that lookup cannot succeed — the item is not in the character's
inventory — so `FUN_140230CB0` returns `0` (`0x140230F38 xor eax,eax`), and the tail runs
anyway: **[L]**

```asm
140d81125  tab  = [rbp+0x7f] ; rdi = char + tab*8
140d8112d  slot = [rbp-0x59]           ; == 0
140d81131  rax  = [rdi+0x5d0]          ; that tab's array
140d81147  cmp ebx, ecx ; jb 140d8116a ; 0 < 0 is false -> falls into the range report
140d8116a  rcx = rax + slot*16 ; call 140d84b80-family FUN_1401ABD80
```

`FUN_1401ABD80` is a `shared_ptr` reset and its **first instruction is `mov rbx,[rcx+8]`** —
no null check. **[L]** So `bToSlot = 0` either resets slot 0 of the player's cash tab
(silently destroying whatever is in it) or, if that array is null, faults at address `8`.

Which of the two happens is **not established** — it depends on whether the character's tab
arrays are allocated when empty, and that was not measured. Either outcome is a reason not to
use `0x1B` for a purchase.

---

## 5. The 71-byte cash-item record, `FUN_1402D0950`

### 5.1 Offsets — measured three independent ways

`cash-shop-stage.md` §6.5's layout is reproduced exactly by reading the listing again, and
**confirmed a third time by `FUN_1402D0010`**, which decodes the same wire record into a
plain struct at offset 0 with no vtable — the same field order, the same widths, the same
`f64` in the middle. **[L]**

```text
wire   size   -> obj    at          FUN_1402D0010's POD    meaning
+0     8      +0x20     1402d0972   +0x00                  u64 liSN, the cash serial   [L]
+8     4      +0x28     1402d097a   +0x08                  ?                           [I]
+12    4      +0x2c     1402d0985   +0x0c                  ?                           [I]
+16    4      +0x30     1402d0990   +0x10                  ITEM ID                     [L]
+20    4      +0x34     1402d099b   +0x14                  ?                           [I]
+24    2      +0x38     1402d09a6   +0x18                  ?                           [I]
+26    13raw  +0x3a     1402d09bc   +0x1a                  a 13-byte fixed name field  [I]
+39    8raw   +0x47     1402d09ce   +0x27                  FILETIME-shaped             [I]
+47    4      +0x4f     1402d09d6   +0x2f                  ?                           [I]
+51    8 f64  +0x53     1402d09e1   +0x33                  a DOUBLE (via 0x1406E8FB0)  [L type]
+59    4      +0x5b     1402d09ee   +0x3b                  ?                           [I]
+63    4      +0x5f     1402d09f9   +0x3f                  ?                           [I]
+67    1      +0x63     1402d0a04   +0x43                  ?                           [I]
+68    1      +0x64     1402d0a0f                          ?                           [I]
+69    1      +0x65     1402d0a1a                          ?                           [I]
+70    1      flag      1402d0a25                          0 -> the record ENDS
       [ if != 0: a whole GW_ItemSlot via FUN_140303530, stored at obj+0x66 ]
```

**71 bytes with the trailing flag 0.** The object is `0x76` bytes: vtable `+0`, three qwords
`+8/+0x10/+0x18`, the record `+0x20..+0x65`, a `shared_ptr` `+0x66..+0x75`. The ctor
`FUN_1402D0770` zeroes exactly those. **[L]**

### 5.2 The two fields that are measured, and why

* **`+0` u64 liSN.** Two independent users. `FUN_140D75850` keys the locker map on
  `[obj+0x20]` and **drops the record if it is `-1`**. `0x1B` passes `[obj+0x20]` to
  `FUN_140230CB0` as the value compared against a `GW_ItemSlot`'s `+0x38`, which is
  `liCashItemSN`. **[L]**
* **`+16` u32 item id.** `0x1B` at `0x140D81094` reads `[obj+0x30]` and feeds it to
  `FUN_1403E8AF0`, the inventory-tab function. Only an item id makes sense there. **[L]**

### 5.2.1 `FUN_1403E8AF0` is `itemId / 1000000` — except for one branch

The magic `0x431BDE83` with `sar 0x12` at `0x1403E8AF6` is exactly division by 1 000 000;
checked against the arithmetic, `999999 -> 0`, `1000000 -> 1`, `5180000 -> 5`. **[L]**

**But when the quotient is 1 there is a second test**, and it can return **6** instead: **[L]**

```asm
1403e8b09  cmp  ebx, 1 ; jne 1403e8b63          ; quotient != 1 -> return the quotient
1403e8b0e  eax = itemId - 5000000 ; cmp 0xf4240 ; jb  1403e8b63
1403e8b1b  eax = itemId - 1000000 ; cmp 0xf4240 ; jb  1403e8b45   <- every 1xxxxxx item
1403e8b45  rax = FUN_140388C60(g_itemInfo, itemId)                ; an ItemInfo lookup
1403e8b53  test rax,rax ; je 1403e8b63
1403e8b58  cmp dword [rax+0x18], 0 ; mov eax, 6 ; jne <return 6>
1403e8b63  mov eax, ebx                                           ; otherwise the quotient
```

So **every equip id goes through a client-side WZ lookup**, and the ones whose ItemInfo
`+0x18` is non-zero land in tab **6** — the pseudo-tab `FUN_1402E4C20` handles with the
*negative-position* (equipped) path, not the Equip bag. `180xxxx` pet equipment is the
obvious candidate for that flag. **Which ids set it is not established** — it needs the WZ
node, not the image.

### 5.3 What I could NOT establish, and what a wrong value does

**No code that reads `+0x28`, `+0x2c`, `+0x34`, `+0x38`, `+0x3a`, `+0x47`, `+0x4f`, `+0x53`,
`+0x5b`, `+0x5f`, `+0x63`, `+0x64` or `+0x65` was found.** That negative is bounded, and the
bound matters:

* `tools/rangescan.py` for `0x47`, `0x53`, `0x3a`, `0x4f` over `0x140d70000..0x140d90000`
  (the whole stage class) returns **0**, while the same tool over the same range returns 41
  sites for `0x74`. The instrument speaks; those offsets are genuinely unused there.
* Over the cash-shop UI, `0x141030000..0x1410e0000` and `0x1410e0000..0x141200000`, the same
  scans return nothing belonging to this object.
* `FUN_1410B5540`, the locker-panel refresh, is **`mov byte [rcx+0x250], 1 ; ret`** — a dirty
  flag. **[L]** The fields are therefore read in a lazy repaint reached through a virtual
  call, which is exactly the shape `rangescan.py` cannot follow. **I did not chase it.**
* `tools/rtti.py` finds no class name containing `Cash` (control passed on `CLogin`), and the
  vtable the ctor installs, `0x14327ECC8`, has a `.text` address rather than a
  CompleteObjectLocator at `[vtable-8]` — **no RTTI for this class.** So the names cannot be
  recovered that way either.

**The structural candidate**, and it is only that: the classic `GW_CashItemInfo` encode order
is `liSN, dwAccountID, dwCharacterID, nItemID, nCommodityID, nNumber(u16),
sBuyCharacterName[13], dateExpire(FILETIME), nPaybackRate, dDiscountRate(double)`. That
aligns field-for-field, including a `double` at exactly `+51`, which is an unusual enough
shape to be worth noting. **[I], from a memory of the layout, not from this image, and
`CLAUDE.md` scores the reference tree 1 of 8.** Do not write it into a builder's doc comment
as fact.

So, plainly:

| field | status | if it is wrong |
|---|---|---|
| `+0` serial | **[L]** | `-1` makes `FUN_140D75850` silently drop the record; a duplicate collides in the map |
| `+16` item id | **[L]** | a bad id gives a bad tab from `FUN_1403E8AF0` (§5.2.1); `FUN_140230CB0` rejects tabs outside 5..6 and `FUN_1402E4C20` rejects tabs outside 1..6 |
| `+24` u16 — the quantity candidate | **[I]** | if it really is `nNumber` and you send 0, the locker row is likely to draw as an empty stack. Nothing crashes: no code was found that reads it. |
| `+39` 8 bytes — the expiry candidate | **[I]** | if it is a FILETIME and you send zeros, the item dates to 1601 and a repaint may draw it as expired or hide it. **I could not find the "permanent" sentinel.** A byte search for the well-known `150842304000000000` returns 177 hits, all in packed/`.rdata` noise with no code immediate among them — that search has no discrimination and settles nothing. |
| everything else | **[I]** | unknown. Sending zeros is the only defensible choice, and it is untested. |

**This is why §0 recommends `0x19` and not `0x03`.** `0x19` carries a `GW_ItemSlot`, whose
every field the server already knows how to build and one of whose two encoders has been on a
live wire. The 71-byte record is needed only if you also want the locker panel populated, and
it is the part of this file with the least evidence behind it.

### 5.4 A minimum record, if you send `0x03` anyway

For the measured click on **Water of Life**, `Commodity.img` SN `160300001` → item `5180000`,
count `1`, period `0` (`gm-handbook/commodity.txt`):

```text
0x05AE
  u8    0x03
  u16   1                     count
  u64   <unique, != 0, != -1> liSN
  u32   0                     [I] account id
  u32   <characterId>         [I] character id
  u32   5180000               ITEM ID                       [L]
  u32   160300001             [I] commodity SN
  u16   1                     [I] quantity
  u8[13] 00 * 13              [I] buyer name
  u8[8]  <FILETIME>           [I] expiry - see the table above
  u32   0
  f64   0.0
  u32   0
  u32   0
  u8    0
  u8    0
  u8    0
  u8    0                     the trailer flag - 0 ENDS the record
```

---

## 6. Wire it like this

### 6.1 The request, re-read against the builder

The 2026-08-26 capture and the builder agree exactly, which is worth stating because it
validates the whole reading. `FUN_140D785F0`'s send site, `0x140D7A40C`..`0x140D7A511`: **[L]**

```asm
0x3E1 CTOR
  u8    2, or 0x1F if (commoditySN - 0x08ADDAE0) < 0x2710      ; cmov at 140d7a445
  u8    (r14d == 2)                                            ; sete  at 140d7a463
  u32   [stage+0x168]                                          ;       at 140d7a476
  if (commoditySN - 0x08ADDAE0) >= 0x2710:                     ; jb SKIPS these two
      u8 0 ; u8 0                                              ; 140d7a4bf / 140d7a4cd
  u32   commoditySN                                            ; edi,  at 140d7a4d2
  u32   arg2                                                   ; [rsp+0x30], at 140d7a4e0
SEND
  [stage+0x74]  = 1
  [stage+0x120] = 1
```

against the captured `02 | 01 02 00 00 00 00 00 | <SN> | 00 00 00 00`:

```text
02          sub-op            ; SN 160300001 - 145600224 = 14699777 >= 10000 -> the LONG form
01          u8   (r14d == 2)
02 00 00 00 u32  [stage+0x168]
00 00       the two conditional zeros -> confirms the long form was taken
<SN>        u32  commoditySN
00 00 00 00 u32  arg2
```

Fifteen bytes, every one accounted for. **[L]** on both halves.

### 6.2 The answer

```text
opcode 0x05AE
  u8    0x19
  u8    1              bRelease. MUST be non-zero or [stage+0x74] stays set.
  u16   nPOS           1-based slot in the tab FUN_1403E8AF0 derives from the item id
                       (usually itemId / 1000000 - but see §5.2.1). Must be free and
                       within that tab's size, or the placement fails: ELog only, no
                       dialog, and the item is lost.
  <GW_ItemSlot>        u8 nType then the body. nType MUST be 1, 2 or 3 (§3.2).
  u8    0              bEffect. 1 plays effect 0x3EE.
```

then, and only then:

```text
opcode 0x05AD
  u32   nxCredit       the DEBITED balance, >= 0 (a negative value ejects the player)
  u32   maplePoints    >= 0
  u32   0              read, sign-checked, discarded
```

**Order is not cosmetic.** `0x05AD` before `0x19` hits `0x140D736E6 cmp eax,1` →
`0x140D736F6 call FUN_140D785F0`, and the client buys again.

### 6.3 Which item to test with

Pick by the encoder that has already been on a wire, not by the item.

| commodity | item | tab | `nType` | server encoder |
|---|---|---:|---:|---|
| `140100001` **Beret** | `1007116` | 1, unless §5.2.1's flag fires | **1** | `equipped_item()` — **proven on a live client** |
| `160300001` Water of Life | `5180000` | 5 | 2 | `bundle_item()` — built, `bag.rs` says NOT WIRED |
| `160100000` Red Hat | `1802002` | **1 or 6** | 1 | `equipped_item()`, but `180xxxx` is pet equipment and is the likeliest thing to trip §5.2.1's tab-6 branch. Avoid. |
| `160000000` Brown Puppy | `5000001` | 5 | **3** | **none. Do not use.** A pet needs the `0xCE`-byte type-3 body nobody has decoded. |

**Start with Beret** (`140100001`, item `1007116`, count 1, period 90 —
`gm-handbook/commodity.txt`). It is a plain hat, so §5.2.1's lookup should leave it in tab 1,
and its body is the type-1 encoder this server has already put on a live wire without a
fault. If it lands in the wrong tab the failure is visible and harmless: nothing appears, and
the ELog path is the only thing that runs.

Note that all three of the SNs the owner actually clicked on 2026-08-26 are poor first tests —
one is a pet, one is pet equipment, one needs an untested encoder. The measurement is still
good; the items just are not.

### 6.4 What falsifies each step

1. **Send `0x05AE 0x19` after a click.** *Falsified if:* the client dies inside the handler —
   check `client-patched\maplecw-hook.log` for a dispatch line on the `0x05AE` handler; per
   `CLAUDE.md`, that line is written on **return**, so a missing one means the handler was
   entered and never came back, and the `GW_ItemSlot` body is wrong.
2. **Watch the shop's buttons.** *Falsified if:* the Buy button is still dead — then either
   `bRelease` was 0, or `[stage+0x120]` did not reach 0 and `FUN_140D78070`'s second `cmp` is
   still refusing. Those two are distinguishable: click Buy again and grep `world.log` for a
   second `0x03E1`. One arrives ⇒ both cleared. None ⇒ one of them did not.
3. **Look for the item in the character's inventory tab.** *Falsified if:* nothing appears ⇒
   `FUN_1402E4C20` refused, i.e. `nPOS` was outside that tab, and the failure is ELog-only so
   nothing on screen will say so.
4. **Send `0x05AD` with the debited balance.** *Falsified if:* a second `0x03E1` sub-op `0x02`
   appears in `world.log` right after ⇒ `[stage+0x120]` was still 1 and step 2's reading was
   wrong.
5. **Wait 60 s without clicking.** A `0x03E0` poll should arrive and be answerable with
   `0x05AD` inertly. *Falsified if:* a `0x03E1` follows it.

Steps 1–2 are one launch and settle the whole of §2 and §3.

---

## 7. What this file does NOT establish

* **Nothing here has been on a wire.** The request side is measured; every claim about the
  *reply* is static.
* **Whether `0x19` is semantically a purchase result.** It is not — it is the answer to
  `0x03E1` sub-op `0x0A`, "move a locker item to an inventory slot" (§3.1). It is being
  recommended because it is the only silent arm that clears both fields and hands over an
  item, not because the protocol intends it here. A real client's cash shop, talking to a
  real Nexon shop server, would probably see `0x03` followed by something this build's jump
  table does not decode at all.
* **The record's field meanings past the serial and the item id** (§5.3), including the two
  the brief specifically asked for — the quantity and the expiry. Offsets yes, semantics no,
  and the search that would settle them is a lazy UI repaint behind a virtual call.
* **Whether `0x1B`'s tail faults or merely destroys slot 0** (§4.3). It depends on whether an
  empty inventory tab has a null array, which was not measured.
* **Which item ids set the ItemInfo flag that sends `FUN_1403E8AF0` to tab 6** (§5.2.1). It
  is a WZ property, not an image constant, and `tools/dump_itemdata.py` was not run against
  it. Every `1xxxxxx` id passes through that test, so the tab of any equip is a prediction
  until the first run says otherwise — which is exactly what step 3 of §6.4 measures.
* **`FUN_142AA2810(2)`, `FUN_142CE70C0`, `FUN_142A37120(0x3EE, …)`** are named by position
  only. Called by every arm that releases the UI; not decoded.
* **Whether a message can reach the screen through a virtual call** from `FUN_140D7F8A0`.
  Two direct-call instruments say no (§1); an indirect one is the residual blind spot, and it
  is the single thing that would make the "`0x19` is silent" claim wrong.
* **19 of the 23 live sub-ops are still unnamed**; §3 says what each *does* to the two state
  fields, which is a different and much narrower claim than knowing what each *is*.

---

## 8. Corrections to `research/cash-shop-stage.md`

Recorded here rather than edited into that file, because it belongs to another pass.

1. **§6.2.1**, *"Every `0x05AE` arm that clears the latch puts a message on screen"* — false
   for `0x19` and `0x1B`, which delegate to sub-functions. Two instruments (§1).
2. **§6.3**, *"case `0x49` … is unmapped"* — it is mapped: string id **17**. All 127 reason
   codes now resolve; 44 fall to the generic 661 and 83 load a specific id. None is silent
   and none is empty, so §6.2.1's conclusion is unaffected — only its footnote.
3. **§8**, *"`FUN_140D7DB60` … is a `0x5AE` sub-handler reached through the jump table"* —
   it is **not** in the 22-slot table (re-dumped, §3), and `tools/callers.py` gives it
   **zero** sites in all three modes. It is the unreachable class's copy of the `0x05AD` arm.
   §8's conclusion — that no CWvsContext opcode carries NX — is unaffected; the sentence
   describing why is wrong.
4. **§6.2**'s read shapes for `0x0C`, `0x14`, `0x19`, `0x1B`, `0x3E` and `0x3F` are
   `reads.py` at depth 4 and therefore **flatten the embedded 71-byte record into its
   component reads** and miss the conditional ones (`0x14`'s extra `u32` for reasons
   `0x1F`/`0x20`; `0x1B`'s `u8`-selected fork). §3 gives the listing order instead. The
   flattening is a property of a depth-limited walk, not an error in the tool.
5. **§5.2**'s buy-builder shape is now confirmed against a live capture, field by field
   (§6.1). It was right.
