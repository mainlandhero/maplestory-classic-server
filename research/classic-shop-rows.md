# The `0x055D` classic shop row, end to end - price, tail, head, result, and Buy Back

**2026-08-22.** `research/classic-shop-opcode.md` established *which* opcode opens the classic
shop and listed four things it had not decoded. This file is those four, plus the Buy Back tab,
plus the request opcode that file never found.

Markers: **[L]** read off this client's listing or capture, **[D]** derived from two or more [L]
facts, **[I]** inferred.

**Nothing was implemented.** No `crates/` file was touched. §9 is a byte-for-byte recipe, not a
patch.

---

## 0. UNRESOLVED, at the top where it belongs

1. **`row+0x04`, `row+0x30`, `row+0xcc`, `row+0xd0`, `row+0xd8`, `row+0xdc`, `row+0xe0`,
   `row+0xe8`, `row+0xec`, `row+0xf8`** - ten fields - are read and stored, and no consumer was
   traced on the plain meso-purchase path. (`row+0x24` and `row+0x100` each have exactly one
   traced consumer and are named only that far in §3.) Sent as zero / empty. **[L]** that they
   are read; meaning **not established**. The blind spot: `tools/fieldrefs.py` matches
   `[reg + disp]` only, so a use through a copy of the struct under a different base register is
   invisible to it, and `FUN_141fbc610` copies the whole 0x130-byte row.
2. **Head fields `c` (`window+0x310`) and `e` (`window+0x337`)** have no traced individual
   consumer. `c` is only touched by a 16-byte bulk `movups` in `FUN_141fa5310`. Same blind spot
   as above.
3. **The Buy Back list has never been seen on a wire.** Every link is [L] separately - the
   second per-row `u8` at `141f9f834`, the push into `shopUI+0x370` at `141fa004e`, the tab
   index at `141fa1cfa`, the tab lookup in `FUN_141fb9cf0`, the index array at `+0x378` - but
   nothing joins them into one observed shop. §9's test plan says which step falsifies which
   link.
4. **Whether a buy-back row draws anything with a null item blob.** The trailing item slot goes
   to `row+0x120`, and the row renderer null-checks it (`141faef78`), so it does not crash - but
   what it draws instead was not traced. **[L]** null-safe, **[I]** that it falls back to the
   item template.
5. **`0x055E` types 6, 8, 21, 22, 24, 29-31, 34, 36-40** are listed in §8 with their reads and
   their message, but their *meaning* comes from the message text alone.

---

## 1. The instruments, and their controls, before any answer below was used

* `python tools/reads.py 0x140304100 2` - its own documented positive control - printed the
  mixed direct/helper run the docstring specifies.
* `python tools/listing.py 0x140304100 | grep READ` reproduced `140304138 raw`, `140304144 u8`,
  `140304183 u8`, then the `u16` run: the control its docstring specifies.
* `python tools/callers.py 0x1402fa9a0` returned **96 call sites in 15 functions** - non-empty,
  so a later zero from it is a real zero.
* `python tools/encodes.py 0x141cb6880 1` reproduced the documented `0x2FF` builder.
* `python tools/fieldrefs.py 0x2f4 --lo 0x141c40000 --hi 0x141d60000 --write` returned its
  documented 3 hits.

**The read count was cross-checked three ways before anything was built on it**, which is the
standing rule and the one this project has paid for twice:

| function | listing | decompiler | `tools/reads.py` |
|---|---:|---:|---:|
| `FUN_1404ba100` (the row item blob) | **42** | **42** | **42** |
| `FUN_141f9f5e0` (the body reader) | **6** direct | **6** direct | 6 direct + 3 helper calls |

Dumps kept: `research/msexe-classicshop-rowdecode.txt` (the row decoder and all three
sub-decoders, listing), `research/msexe-classicshop-rowloop.txt` (the body reader, listing),
`research/msexe-classicshop-055e.txt` (the `0x055D`/`0x055E` handler, listing),
`research/msexe-classicshop-transaction.txt` (the request builders and the layout, decompiled).

---

## 2. The four open questions, answered

### 2.1 The price is `row+0x38`, a **64-bit** meso amount. **[L]**, three independent sites

`research/classic-shop-opcode.md` called it *"`u64 -> +0x38`"* and said nothing establishes
which field is the price. It is that one, and the row struct base in the transaction is `r13`,
proved by `141fb56d2  imul r13, r8, 0x130` - the rows are an array of **0x130-byte** elements
and `r8` is `shopUI+0x4a0`, the selected index.

```asm
; 1. the affordability check - FUN_141fb55e0
141fb78c2  mov    rax, qword ptr [rbp + 0x28]   ; a discounted price, if any
141fb78c6  test   rax, rax
141fb78c9  jne    0x141fb78cf
141fb78cb  mov    rax, qword ptr [r13 + 0x38]   ; <- otherwise the row's own price
141fb78cf  mov    ebx, r15d                     ; the quantity
141fb78d2  imul   rbx, rax
141fb78d6  mov    rcx, qword ptr [rbp - 0x40]
141fb78da  call   0x1401d35e0                   ; the player's meso count
141fb78df  lea    rcx, [rbx - 1]
141fb78e3  movabs rdx, 0x746a5287fe             ; 499 999 999 998
141fb78ed  cmp    rcx, rdx
141fb78f0  ja     0x141fb78fb                   ; total outside [1, 499999999999] -> refuse
141fb78f2  cmp    rax, rbx
141fb78f5  jge    0x141fb7f38                   ; mesos >= total -> send the request
141fb78fb  mov    edx, 0x9d                     ; "You don't have enough Mesos."
```

```asm
; 2. the discount, which takes the price as its money argument - FUN_141fb55e0
141fb7162  mov  eax, dword ptr [r13 + 0x100]
141fb7172  mov  r8d, dword ptr [r13 + 8]        ; itemId
141fb7176  mov  rdx, qword ptr [r13 + 0x38]     ; price
141fb717d  call 0x141fb9f30                     ; -> the effective unit price, or 0
```

```asm
; 3. the on-screen price - FUN_141faecc0, the row renderer
141fb00d6  mov  rdx, qword ptr [r15 + r14 + 0x38]
141fb00e2  call 0x141fba580                     ; format the number
141fb0106  mov  edx, 0x4af                      ; string 0x4AF = " Mesos"
```

Three different subsystems - display, discount, affordability - read the **same qword** at
`+0x38` and treat it as money. Corroborating: string `0x4B0` is `'Recharge: %lld'`, a 64-bit
format, and the meso cap in the comparison is 5·10^11, which does not fit a `u32`.

**`row+0x38` is a signed 64-bit little-endian meso price. It is not negated for the sell tab**
(unlike Shop2, where the sign chose the tab - see §5).

### 2.2 The conditional tail: the `u8` is **not** the end of the row. The row is 42 reads long

`classic-shop-opcode.md` listed thirteen fields and then *"`u8` flag; when non-zero, a further
block runs (NOT decoded)"*. That reading stopped one branch too early. From the listing at
`1404ba1c0`:

```asm
1404ba1c0  call 0x1406e8ae0   <<< READ u8
1404ba1c5  test al, al
1404ba1c7  je   0x1404ba213          ; <- the JUMP TARGET is the NEXT read, 0x1404ba216
1404ba1c9  ...                       ; the conditional block: FUN_1404b9480 only
1404ba20e  call 0x1404b9480
1404ba213  mov  rcx, rsi
1404ba216  call 0x1406e8c20   <<< READ u32   ; UNCONDITIONAL, and 27 more reads follow
```

So the `u8` guards **one** sub-decoder, and everything after `0x1404ba213` runs for every row.
The row is **42 direct reads** plus two sub-decoders, not thirteen fields. `tools/reads.py`
marks reads after `0x1404ba1c0` as `gated?`, which is the over-report its own docstring warns
about; the listing settles it.

The three optional blocks, all of which have a one-byte "off" encoding:

| block | gate | off encoding | on |
|---|---|---|---|
| A - `FUN_1404b9480` | the `u8` at `1404ba1c0` | `00` | `u32, u8, u8, str, u32, str, …` |
| B - `FUN_1404b7de0` | its own leading `u8 kind` | `00` (also 5..255) | kind 2: nothing more. kinds 1/3/4: `u32 n`, then `n` × `raw 8`, then one more `raw 8` |
| C - `FUN_140303530` | the **second** per-row `u8` | `00` slot type | `u8 slotType` 1/2/3 -> `GW_ItemSlot::RawDecode`, documented in `research/msexe-setfield.md` |

Block B's leading byte is **always on the wire**, even when zero - `1404b7e0c` reads it before
any branch. Block C's byte is only on the wire when the second per-row flag is set (§5).

### 2.3 The head fields `a`..`e`

```asm
; FUN_141fa66c0, the 0x055D arm at 141fa767b
141fa767b  cmp  qword ptr [rip + 0x1b00e9d], 0   ; -> 0x143AA8520
141fa7683  je   0x141fa768c
141fa7685  call 0x141fb94c0                      ; a modal is open: send 0x00F5 sub-op 3, DROP
141fa768f  call 0x1406e8c20   <<< READ u32   a
141fa769b  call 0x1406e8ae0   <<< READ u8    hasB
141fa76a7  call 0x1406e8c20   <<< READ u32   b     (only when hasB != 0)
141fa76b0  call 0x141f9f400                       ; window = factory(a)
141fa76b5  mov  dword ptr [rax + 0x1658], esi     ; window+0x1658 = b, else 0
141fa76c1  call 0x141f9f5e0                       ; the body
```

| field | what | evidence |
|---|---|---|
| `a` | **the NPC template id** | **[D]**. `FUN_141f9f400(a)` special-cases `a == 0x8A4701` (9 061 633) and `a == 0x8A4A5A` (9 062 490) to an `adventureShop` window; both are NPC template ids. The generic path stores `a` at `window+0x314`, and `FUN_141fb55e0` at `141fb7116` tests `window+0x314 - 9090100 < 100`, another template range |
| `hasB` | a presence byte for `b` | **[L]** |
| `b` | `window+0x1658` | **[L]** read; consumed once, at `141fa8c1d` in `FUN_141fa8060`. Meaning **not established**. Send `hasB = 0` |
| `c` | `window+0x310` | **[L]** read. No individual consumer traced (§0 item 2) |
| `name` | `window+0x318`, char[0x1f], `strncpy`-limited to 0x1f | **[L]**. **Echoed verbatim** in `0x00F5` sub-op 0 (`141fb7fcd  lea rdx, [rbx + 0x318]`) |
| `e` | `window+0x337`, immediately after the name, **unaligned** | **[L]**. Part of the same echo |
| `d` | `window+0x3f8` | **[L]**. **Echoed** in `0x00F5` sub-op 0 as a `u32` (`141fb7fbb`). Its only other reference is the write |

`FUN_1402d2bb0(pkt, dst)` reads the **string first, then the `u32`** (`1402d2bc5`, `1402d2be0`),
copies at most 0x1f characters, and puts the `u32` at `dst+0x1f`. So `name` and `e` are one
23-byte packed record in the window, but two separate fields on the wire.

**The guard is the important part of this section.** `0x143AA8520` is not the shop's own slot -
it is the client's single **modal-dialog** slot. `tools/dataref.py 0x143AA8520 --writes` finds
one setter (`FUN_14177fe00`) and one clearer, and `tools/callers.py 0x14177fe00` finds **36
call sites in 36 functions**, one of which is `FUN_140d22310`, the Shop2 constructor that
`research/npc-shop-crash2.md` measured. `research/npc-shop.md` §3 already called `0x143AA8520`
*"the current dialog"* for a completely different window. **[D]**

> **A `0x055D` sent while any modal dialog is up - including the `0x055B` script message - is
> discarded, and the client answers with `0x00F5 03`.** The shop never opens and nothing in
> `world.log` says why.

### 2.4 `0x055E` is a 41-case switch, and two of its cases carry a whole shop list

```asm
141fa66ee  jne  0x141fa76c6                 ; not 0x055E -> return
141fa6712  call qword ptr [rax + 0xd0]      ; the modal must be a shop -> else return
141fa6730  cmp  dword ptr [r14 + 0x4b0], 0  ; no request outstanding -> a diagnostic log line
141fa6786  mov  dword ptr [r14 + 0x4b0], esi ; THE LATCH IS CLEARED, before the switch
141fa6790  call 0x1406e8ae0   <<< READ u8   ; the result type
141fa6798  cmp  eax, 0x28
141fa679b  ja   0x141fa7671                 ; > 0x28 -> string 0x4AD, no crash
141fa67a8  mov  ecx, dword ptr [rdx + rax*4 + 0x1fa76e4]   ; table at 0x141fa76e4
```

41 entries at `0x141fa76e4`, each an offset from `0x140000000`. Full table in §8. The single
most useful entry: **type 10 re-reads the entire shop body** through `FUN_141f9f5e0` - the same
function `0x055D` uses - and then selects the Buy Back tab.

---

## 3. The `0x055D` body, field by field

```text
HEAD, read by FUN_141fa66c0
  u32   a          npc template id
  u8    hasB
  u32   b          only when hasB != 0

HEAD, read by FUN_141f9f5e0
  u32   c          -> window+0x310
  u32   d          -> window+0x3f8   echoed back in 0x00F5 sub-op 0
  str   name       -> window+0x318   echoed back in 0x00F5 sub-op 0 (max 0x1f chars kept)
  u32   e          -> window+0x337   part of the same echo
  u16   nRows      0 is legal: an EMPTY WINDOW, not a dialog (141f9f76e jumps past the loop)

PER ROW, nRows times
  u32              -> row+0x0c      a COUNT - see below
  <FUN_1404ba100, 42 reads, below>
  u8               -> the SELL flag
  u8               -> the BUY BACK flag
  [ if the BUY BACK flag != 0 ]
  u8    slotType   -> 0 = nothing further; 1/2/3 = a GW_ItemSlot, into row+0x120
```

**`row+0x0c`** is read by the loop, before the item blob, and has two consumers. **[L]**
`141f9fc3d`: with `row+0x10 > 0`, a `row+0x0c` of zero puts the itemId into `shopUI+0x398`, the
list `FUN_141fbac80` scans to raise `0x4A9` *"The vendor doesn't have enough in stock."* -- so it
is **remaining stock**. `141fb8f7f` (`FUN_141fb8ea0`, the sell builder) reads `row+0x0c` off a
row in `shopUI+0x360` and uses it as the ceiling on "How many are you willing to sell?" -- so on
a row that represents an item the player holds it is a **stack count**. The first is [L] for
packet rows; the second is [L] for the client-built sell list and **[I]** for a Buy Back row,
where nothing was traced. `row+0x1c` is the other candidate for the number a Buy Back row draws.

### `FUN_1404ba100`, the row item blob

Listing order (authoritative), meanings from the decompiler and from the consumers.

| # | wire | dest | what | mark |
|---:|---|---|---|---|
| 1 | `u32` | `+0x04` | no consumer traced on the meso path | [L] read |
| 2 | `u32` | `+0x08` | **itemId** - `FUN_1403e8af0` (GetInventoryType), `FUN_140398ba0`/`FUN_1402c8c00` (name), and the `/10000 == 207 or 233` rechargeable tests | **[L]** |
| 3 | `u32` | `+0x20` | **buy-tab tag** - hashed at `141fa0b53`; the number of distinct values is the number of Buy tabs | **[D]** |
| 4 | `u32` | `+0x10` | **stock limit present** - `> 0` arms the sold-out test at `141f9fc34` | **[D]** |
| 5 | `u32` | `+0x24` | a duration, paired with `+0x28` in `FUN_141fbad10` | [L] read, [I] meaning |
| 6 | `raw 8` | `+0x28` | **the purchased item's expiry FILETIME.** `FUN_141fbad10`: `+0x24 >= 1` **or** `+0x28 > 94354848000000000` means "limited duration", which suppresses the quantity box and adds string `0x498` | **[L]** |
| 7 | `raw 4` | `+0x30` | no consumer traced | [L] read |
| 8 | `u64` | `+0x38` | **PRICE, mesos** - §2.1 | **[L]** |
| 9 | `u32` | `+0x48` | **required item id** (a barter cost) - compared at `141fb79a2` against a de-obfuscated inventory `GW_ItemSlot+0x20` | **[L]** |
| 10 | `u32` | `+0x4c` | **required item count.** `!= 0` switches the whole purchase onto the barter path | **[L]** |
| 11 | `u32` | `+0x50` | **pointShop point type** - formats `UI/UIWindow4.img/pointShop/%d/pointName` (string `0x5F4`) | **[L]** |
| 12 | `u32` | `+0x54` | **pointShopWsr point type** - string `0x5F6` | **[L]** |
| 13 | `u32` | `+0x58` | **point cost.** `!= 0` and the purchase is refused with a point-name message instead of sending anything (`141fb817b`) | **[L]** |
| 14 | `u8` | - | **optional block A gate**. `0` = nothing more | **[L]** |
| - | *(A)* | `+0x68`/`+0x70` | `FUN_1404b9480`: `u32, u8, u8, str, u32, str, …` | [L] |
| 15 | `u32` | `+0x78` | purchase-limit key, used with `+0x7c` in `FUN_141fbb4a0` | [D] |
| 16 | `u32` | `+0x7c` | **purchase-limit count.** `> 0` -> string `0x497` *"This item can only be purchased %d time(s)"* and the quantity box is suppressed | **[L]** |
| 17 | `u8` | - | **optional block B kind**, `FUN_1404b7de0`. Always on the wire. `0` = nothing more | **[L]** |
| - | *(B)* | `+0x80` | kinds 1/3/4: `u32 n`, `n` × `raw 8`, then one `raw 8`. Kind 2: nothing | [L] |
| 18 | `u32` | `+0x90` | **signed level gate.** `> 0` -> min level (string `0x499`); `< 0` -> max level, `|v|` (string `0x49A`). Checked at purchase, not at list build | **[L]** |
| 19 | `u16` | `+0x94` | **min level**, checked at list build - a row below it is **dropped from the list** | **[L]** |
| 20 | `u16` | `+0x98` | **max level**, same | **[L]** |
| 21 | `u8` | `+0xf0` | **HIDE.** Non-zero and the row is destroyed at `141f9f849` **before block C is read** - see §4 | **[L]** |
| 22 | `raw 8` | `+0x9c` | **sale start FILETIME.** `CompareFileTime(+0x9c, now) > 0` aborts the purchase (`141fb646c`) | **[L]** |
| 23 | `raw 8` | `+0xa4` | **sale end FILETIME.** At list build, `CompareFileTime(+0xa4, now) < 0` **drops the row** (`141f9fa42`); at purchase, `<= 0` aborts (`141fb648b`) | **[L]** |
| 24 | `u32` | `+0xac` | **quest id** | **[L]** |
| 25 | `u16` | `+0xb0` | **required quest state** - `FUN_142d9c490(user, +0xac, 0, 0)` must equal it | **[L]** |
| 26 | `u8` | `+0xb4` | show-anyway flag: `0` drops the row from the list when the quest test fails; `1` keeps it and refuses at purchase with string `0x1021` | **[D]** |
| 27 | `u32` | `+0xb8` | a purchase-counter id, passed with `+0xc0` to `FUN_1402e01c0` | [D] |
| 28 | `str` | `+0xc0` | the counter's name | [D] |
| 29 | `u32` | `+0xc8` | **a threshold on that counter.** `141fa0989`: the counter's value is `atoi`-ed and the row is **kept only when `value >= +0xc8`** - so it reads as an unlock condition, not a purchase cap. `+0xc8 == 0` short-circuits the whole test at `141fa0917` and the row is always kept | **[L]** on the polarity, **[I]** on the name |
| 30-36 | `u32, str, u32, u32, str, u32, u8` | `+0xcc`, `+0xd0`, `+0xd8`, `+0xdc`, `+0xe0`, `+0xe8`, `+0xec` | two more counter-shaped groups; no consumer traced on the meso path | [L] read |
| 37 | `str` | `+0xf8` | no consumer traced | [L] read |
| 38 | `u32` | `+0x100` | passed to the discount function `FUN_141fb9f30` as its last argument | [L] |
| 39 | `u32` | `+0x104` | **required CITIZENSHIP type** - `FUN_1402c8af0` validates it, `FUN_1402c90f0(user, t)` must return 1, refusal is string `0x17DA` *"%s citizenship required."*. The row loop also copies the first non-zero one to `shopUI+0x3fc` (`141f9fc88`), which is what `/citizenshipBackgrnd/%d` draws | **[L]** |
| 40 | `u32` | `+0x108` | **required citizenship GRADE** - `FUN_1402c9150(user, t)` must be `>= +0x108`, refusal is string `0x17DB` *"You need to be Citizenship Grade %s or higher to purchase this item."* | **[L]** |
| 41a | `raw 8` | `+0x40` | **only when `itemId / 10000` is 207 or 233** (throwing stars, bullets). An **IEEE-754 double**: `FUN_141fb9240` refuses to recharge when `*(double*)(row+0x40) == 0.0`. This is the `/BtRecharge` price, string `0x4B0` `'Recharge: %lld'` | **[L]** |
| 41b | `i16` | `+0x1c` | **otherwise.** Sign-extended. `> 1` suppresses the quantity box, so it reads as "units delivered per purchase" | **[L]** read, **[I]** meaning |
| 42 | `i16` | `+0x10c` | **maximum quantity per purchase.** `== 1` -> a plain yes/no (string `0x49C`); `> 1` -> the quantity box with this as its ceiling, and `qty > +0x10c` is rejected | **[L]** |

The `itemId` test is the same magic divide in both places -
`imul 0x68db8bad / sar edx, 12` is `x / 10000` - at `1404ba582` (which read to take) and at
`141f9fff5` (which list to join).

---

## 4. The five ways a row you sent is not on screen, and the one that desynchronises the stream

All five are **[L]**, in listing order inside the loop body.

| # | test | address | consequence |
|---|---|---|---|
| 1 | `row+0xf0 != 0` | `141f9f849` | destroyed, **before block C is read** |
| 2 | `CompareFileTime(row+0xa4, now) < 0` | `141f9fa42` | destroyed, **before block C is read** |
| 3 | `row+0x94 > level`, or `row+0x98 != 0 && row+0x98 < level` | `141fa032c`, `141fa0565` | dropped, after block C |
| 4 | `row+0xac > 0 && questState != row+0xb0 && row+0xb4 == 0` | `141fa070d` | dropped, after block C |
| 5 | `row+0xc8 != 0` and the counter named by `row+0xb8`/`row+0xc0` is **below** it | `141fa098f` | dropped, after block C |

> **Tests 1 and 2 run before `FUN_140303530`.** If a row carries the Buy Back flag *and* is
> dropped by test 1 or 2, the client never reads the trailing item-slot byte, every later row is
> parsed one byte out of phase, and the packet becomes garbage. This is the same failure mode as
> the two truncated packets `CLAUDE.md` records, arriving from the other direction. **[L]**

`row+0xa4` is compared against **now**, unconditionally - there is no zero-means-forever
sentinel on that path. **`row+0xa4 = 0` hides every row in the shop.** The client's own
far-future constant is `150842304000000000` (2079-01-01) and its "never" is
`94354848000000000` (1900-01-01); the two sit adjacent at `0x14341CB10` in `.rdata` and appear
**177** and **249** times in the image, so they are this client's conventions, not an
assumption imported from elsewhere. **[D]**

---

## 5. The tab model, and where Buy Back lives

**The Buy Back list is not a second block and not a separate packet. It is the same row array,
tagged by the second per-row `u8`.** **[L]**

```asm
; inside the row loop, after the row survives tests 1 and 2
141f9fe7e  test r12d, r12d              ; the SECOND per-row u8
141f9fe81  je   0x141f9fff5
141f9fe91  call 0x140303530             ; <- the trailing item slot, into row+0x120
...
141f9fff5  mov  eax, 0x68db8bad         ; itemId / 10000
141fa000a  cmp  edx, 0xcf               ; 207 - throwing stars
141fa0012  cmp  edx, 0xe9               ; 233 - bullets
141fa001a  lea  rcx, [r15 + 0x358]      ;   -> the RECHARGE list
141fa0049  test r12d, r12d              ; the SECOND per-row u8 again
141fa004c  je   0x141fa0093
141fa004e  lea  rcx, [r15 + 0x370]      ;   -> the BUY BACK list
141fa007d  lea  rcx, [r15 + 0x378]      ;   -> its parallel index array
141fa008e  mov  ecx, dword ptr [rbp + 0x58]  ;      the row's index in the packet
...
141fa107e  test r13d, r13d              ; the FIRST per-row u8
141fa1087  lea  rcx, [r14 + 0x348]      ;   -> the SELL list  (+0x350 = its index array)
141fa0af7  lea  r12, [rdi + 0x340]      ; every surviving row also joins +0x340 (+0x390)
```

`r12` is reloaded as a pointer at `141fa0af7`, which is **after** both uses of `r12d`, so the
flag is intact at both. Checked, not assumed.

At `141fa0303` a Buy Back row **skips the Buy-tab classification entirely** (`jne 0x141fa1178`),
so a row is either merchandise or repurchase, never both.

### The lists on the window

| offset | list | index array | how it is filled |
|---|---|---|---|
| `+0x340` | every surviving row | `+0x390` | unconditional |
| `+0x348` | **Sell** | `+0x350` | first per-row `u8 != 0` |
| `+0x358` | **Recharge** | - | `itemId / 10000` is 207 or 233 |
| `+0x370` | **Buy Back** | `+0x378` | **second per-row `u8 != 0`** |
| `+0x380[i]` | **Buy tab `i`** | `+0x388[i]` | grouped by `row+0x20` |
| `+0x360` | the Sell tab as displayed | - | built client-side from the player's inventory |

`FUN_141fb9cf0(shopUI, tabIndex)` picks between them: `tabIndex == shopUI+0x4d8` -> Sell,
`tabIndex == shopUI+0x4dc` -> Buy Back, otherwise `+0x380[tabIndex]`, else `+0x340`. Tab indices
are validated `0..6`, so **at most seven tabs**. **[L]**

The two special tabs are created after the loop, and only if their list is non-empty:

```asm
141fa1ad1  mov  rax, qword ptr [r15 + 0x348]   ; the Sell list
141fa1adb  je   0x141fa1cce                    ;   empty -> no Sell tab
141fa1aeb  cmp  byte ptr [rbp], 0
141fa1aef  je   0x141fa1cce                    ;   more than one Buy tab -> no Sell tab
141fa1b07  mov  dword ptr [r15 + 0x4d8], ecx

141fa1cce  mov  rax, qword ptr [r15 + 0x370]   ; the Buy Back list
141fa1cd8  je   0x141fa1ec6                    ;   empty -> NO BUY BACK TAB
141fa1cfa  mov  dword ptr [r15 + 0x4dc], ecx
```

`byte ptr [rbp]` is `setbe` on `[rbp+0x11c] <= 1`, and `[rbp+0x11c]` is the **size field of the
hash map keyed on `row+0x20`** (its header is initialised at `141f9f628`: mask `0x1f`, size,
load `0x64`/`0x18`). So **sending rows with two or more distinct `row+0x20` values makes the
Sell tab disappear**. **[D]** - the map's identity is read off its header shape, which is an
inference from layout, not a symbol.

> **Send every ordinary row with `row+0x20 = 0`** until someone wants multiple Buy tabs.

### The buy-back list is per-session, and the client agrees

The owner's spec: the counter keeps the last 15 items sold to any NPC, cleared on logout or server
restart. Nothing in the client persists it - the rows arrive in the packet and are destroyed
with the window. `FUN_141fbab90` (called by `0x055E` types 10 and 35) clears
`shopUI+0x4d8`/`+0x4dc` and the lists before re-reading. **The 15-item cap and the clearing
policy are server policy; the client imposes neither.** **[L]** for the absence in the parse
path; the blind spot is that I did not enumerate the window's destructor.

---

## 6. `0x00F5` - the classic shop's request opcode, which `classic-shop-opcode.md` never found

`research/msexe-send-opcodes.txt` filtered to `FUN_141fb*` gives exactly one outbound opcode.
It is **not** Shop2's `0x0104`.

```text
0x00F5  245  FUN_141fb55e0 @ 141fb55e0  (call at 141fb7f78)
0x00F5  245  FUN_141fb8ea0 @ 141fb8ea0  (call at 141fb9177)
0x00F5  245  FUN_141fb9240 @ 141fb9240  (call at 141fb93fd)
0x00F5  245  FUN_141fb94c0 @ 141fb94c0  (call at 141fb94e3)
```

`tools/encodes.py` at depth 2 gives four bodies, and the two entry points are covered:
`FUN_141fa5120` (a button dispatcher, **[I]** from its two adjacent call sites) calls the first two, `FUN_141fa6360` tail-jumps to the first
three, and `FUN_141fb94c0` is reached from the window's vtable slot 40 (`FUN_141fa25f0`, which
tail-jumps to the UI teardown `FUN_14177fef0`) and from the `0x055D` modal guard.

| sub-op | builder | body after the `u8` | what |
|---:|---|---|---|
| **0** | `FUN_141fb55e0` | `u16 rowIndex, u32 itemId, u16 quantity, u32 d, str name, u32 e` | **BUY - and BUY BACK** |
| **1** | `FUN_141fb8ea0` | `u16 inventorySlot, u32 itemId, u16 quantity` | **SELL** |
| **2** | `FUN_141fb9240` | `u16 inventorySlot` | **RECHARGE** (string `0x4B5`) |
| **3** | `FUN_141fb94c0` | nothing | **CLOSE** |

`d`, `name` and `e` are the head fields echoed back verbatim from `window+0x3f8` and
`window+0x318`.

### `rowIndex` is the index in **your** row array. **[L]**

```asm
141fb56aa  movsxd r8, dword ptr [r12 + 0x4a0]   ; the selected display row
141fb56dc  mov    edx, ebx                      ; the selected tab
141fb56e1  call   0x141fb9da0                   ; -> the ORIGINAL packet index
```

The decompiler shows `FUN_141fb9da0` with two arguments because it never saw `r8` set; the
listing shows three, and the function body indexes `+0x350` / `+0x378` / `+0x388[tab]` /
`+0x390` by it. Those arrays were filled in the loop with, for Buy Back, `[rbp+0x58]` - the raw
loop counter - and for the others with `skipped - 1 + listLength`, which is the same number.

> **The server tells a buy from a buy-back by which row index it is**, because the server is
> the one that set the Buy Back flag on that row. There is no direction byte on the wire, and
> the client sends the *same* sub-op for both.

### The latch, and it is stricter than Shop2's. **[L]**

`tools/fieldrefs.py 0x4b0 --lo 0x141f90000 --hi 0x141fd0000` gives nine hits and they are
unambiguous:

```text
141fb7fec  mov dword ptr [rbx + 0x4b0], 1    FUN_141fb55e0   sub-op 0 sets it
141fb91b8  mov dword ptr [r15 + 0x4b0], 1    FUN_141fb8ea0   sub-op 1 sets it
141fb9430  mov dword ptr [r14 + 0x4b0], 1    FUN_141fb9240   sub-op 2 sets it
141fa6786  mov dword ptr [r14 + 0x4b0], esi  FUN_141fa66c0   ONLY 0x055E clears it
141fa41f4  mov dword ptr [r13 + 0x4b0], esi  FUN_141fa2670   CreateLayout clears it
```

and each builder's first act is `if (shopUI+0x4b0 != 0) return;` (`141fb56c4`, `141fb8f08`,
`141fb929c`).

Shop2's equivalent latch was cleared by **either** the result **or** a fresh list packet. This
one is not: **only a `0x055E` clears it.** An unanswered sub-op 0/1/2 leaves every further click
on Buy, Sell and Recharge a silent no-op until the window is rebuilt - and a fresh `0x055D`
cannot rebuild it, because §2.3's guard drops `0x055D` while the shop is open. The player must
close the shop (sub-op 3) and click the NPC again.

**Sub-op 3 latches nothing** and the client tears the window down on the next instruction, so no
answer is owed - the same narrow exemption `net::script` records for `0x0151`.

---

## 7. What `0x055E` type 0 actually carries

```asm
141fa67d8  call 0x1406e8ae0   <<< READ u8
141fa67e5  je   0x141fa67fb
141fa67e7  call 0x1406e8c20   <<< READ u32   -> FUN_141fb9530(shopUI, x)
141fa67fb  call 0x1406e8c20   <<< READ u32   itemId
141fa6806  call 0x1406e8c20   <<< READ u32   remaining
141fa6816  call 0x141fa6550                  ; update the row's stock
141fa686d  cmp  dword ptr [rcx + rdx + 0x10], esi   ; the row had a stock limit
141fa6871  jle  0x141fa68a5
141fa6873  test ebx, ebx                            ; and remaining == 0
141fa6877  mov  edx, 0x101d                  ; "You are purchasing all of the remaining stock."
141fa688f  lea  rcx, [r14 + 0x398]           ; -> the sold-out list
```

So type 0 is `u8 kind`, then **either** `u32` (kind != 0) **or** `u32 itemId, u32 remaining`
(kind == 0). `classic-shop-opcode.md`'s *"a case 0 that reads `u8, u32, u32`"* is the second
branch. **Send `00` then `itemId` then the remaining stock.** Send `remaining = -1` (or any
non-zero) for an unlimited row - `0` combined with `row+0x10 > 0` marks the item sold out
client-side.

---

## 8. The `0x055E` result table

41 cases, from the jump table at `0x141fa76e4` (`python tools/dump_va.py 0x141FA76E4 168`).
Targets are `0x140000000 + entry`. Reads are from the listing.

| type | reads | effect |
|---:|---|---|
| **0** | `u8`, then `u32` or `u32,u32` | **success** - §7 |
| 1 | `u32 itemId` | mark sold out + `0x4A9` *"The vendor doesn't have enough in stock."* |
| 2, 17 | - | `0x9D` *"You don't have enough Mesos."* |
| 3 | - | `0xA5` *"You don't have enough points."* |
| 4 | `u32` | `0x126C` *"can be purchased once you clear Floor %d"* |
| 5 | - | `0x1021` *"you haven't met the quest requirements"* |
| 6 | `u32` | a requirement message |
| 7 | - | `0x4B8` *"That item cannot be purchased right now."* |
| 8 | `u32` | `0x13D7` *"Must be rank %s to use."* |
| 9 | - | `0x4AA` *"Please check if your inventory is full or not."* |
| **10** | **the whole body** | `FUN_141fbab90` + **`FUN_141f9f5e0`**, then select the **Buy Back** tab |
| 11 | - | `0x4A9`, without the sold-out marking |
| 13 | - | `0x101E` *"You already have too many mesos on you."* |
| 14 | `u64` | `0x101F` *"The meso cap per sale is %s."* |
| 16 | - | nothing at all, but the latch is cleared |
| 20 | - | `0xA2` *"You need more items"* |
| 21 | `u32` | `0x49A` *"You must be under lv.%d"* |
| 22 | `u32` | `0x499` *"You must be over lv.%d"* |
| 23 | - | `0x4AB` *"You can no longer purchase this item."* |
| 24 | `u32 itemId` | `0x494` *"%s has been purchased the maximum number of times."* |
| 25 | - | `0xDAB` *"Items or mesos cannot be moved."* |
| 26 | - | `0x4AE` *"Shop restocked. Please close and reopen the window."* |
| 27 | `u32 itemId, u32 n` | `0x101C` *"You only purchase %d more of that item."* |
| 29 | - | `0x925` inactive-account |
| 30 | `u32` | `0x926` IP-change restriction |
| 31 | `raw` | `0x927` / `0x928` trade restriction |
| 32 | - | `0x1BE` *"This function cannot be used right now."* |
| 33 | - | `0x1BF` *"That item is temporarily restricted from trading."* |
| 34 | - | `0x12D6` *"That cannot be done in the current world."* |
| **35** | `u8`, then **the whole body** if non-zero | refill + select **tab 0**; if zero, `0xA3` *"The item details have changed."* |
| 36 | - | `0x1383` *"That item can't be purchased again."* |
| 37 | - | `0x1384` *"That item can't be sold."* |
| 38 | - | `0x14E9` *"You cannot use the Shop while incapacitated."* |
| 39 | `u32` | a citizenship-type refusal (`FUN_1402c8c00`, the same name lookup `row+0x104` uses) |
| 40 | `u32, u32` | a citizenship type + grade refusal |
| 12, 15, 18, 19, 28, **> 0x28** | - | `0x4AD` *"An unknown error prevented the trade from completing."* |

**Type > 0x28 is safe** - it takes the default arm and shows `0x4AD`. It still clears the latch,
because the latch is cleared before the switch.

**Types 10 and 35 are the buy-back refresh.** After a sale, send type 10 with the head-less body
(`u32 c, u32 d, str name, u32 e, u16 nRows, rows…`) and the client rebuilds every list and jumps
the player to the Buy Back tab. Type 35 does the same and jumps to tab 0.

---

## 9. WIRE IT LIKE THIS

Nothing below is implemented. `crates/net/src/shop.rs` still builds `0x0560`, whose art is
missing from this client's WZ, and NPC shops are still off.

### 9.1 The exact `0x055D` body for a two-item shop

Head, then `nRows`, then rows. Row one is Red Potion (2000000) at 50 mesos. Row two is a Buy Back
entry for the same item at 25 mesos with three in the stack. Constants:
`PERMANENT = 150842304000000000`, `ZERO_TIME = 94354848000000000`, both little-endian `u64`,
both the client's own (§4).

```text
HEAD (19 bytes) then the row count (2 bytes)
  u32  21              npc template id                (a)
  u8   0               hasB
  u32  0                                              (c)
  u32  0               echoed back in every request   (d)
  u16  0               name, empty                    (an empty str: a u16 zero length)
  u32  0               echoed back in every request   (e)
  u16  2               nRows

ROW, 157 bytes for an ordinary non-rechargeable item
  u32  0        +0x0c   remaining stock; irrelevant while +0x10 == 0
  u32  0        +0x04
  u32  2000000  +0x08   itemId
  u32  0        +0x20   buy-tab tag - KEEP 0 or the Sell tab vanishes
  u32  0        +0x10   stock limit - 0 = unlimited
  u32  0        +0x24
  u64  0        +0x28   purchased-item expiry; 0 reads as "none"
  u32  0        +0x30
  u64  50       +0x38   *** THE PRICE ***
  u32  0        +0x48   required item id
  u32  0        +0x4c   required item count - MUST be 0 for a meso purchase
  u32  0        +0x50   pointShop type
  u32  0        +0x54   pointShopWsr type
  u32  0        +0x58   point cost - MUST be 0 or the purchase is refused, silently to us
  u8   0                optional block A: OFF
  u32  0        +0x78
  u32  0        +0x7c   purchase limit - 0 = unlimited
  u8   0                optional block B: kind 0, nothing follows
  u32  0        +0x90   signed level gate
  u16  0        +0x94   min level
  u16  0        +0x98   max level
  u8   0        +0xf0   *** MUST BE 0 or the row is destroyed before block C ***
  u64  ZERO_TIME +0x9c  sale start
  u64  PERMANENT +0xa4  *** sale end - MUST be in the future or the row VANISHES ***
  u32  0        +0xac   quest id
  u16  0        +0xb0   quest state
  u8   0        +0xb4
  u32  0        +0xb8
  u16  0        +0xc0   counter name, empty string
  u32  0        +0xc8   counter threshold - 0 disables the test entirely
  u32  0        +0xcc
  u16  0        +0xd0   empty string
  u32  0        +0xd8
  u32  0        +0xdc
  u16  0        +0xe0   empty string
  u32  0        +0xe8
  u8   0        +0xec
  u16  0        +0xf8   empty string
  u32  0        +0x100
  u32  0        +0x104  required citizenship type  (goal H)
  u32  0        +0x108  required citizenship grade
  i16  1        +0x1c   bundle quantity   (or, for itemId/10000 in {207,233},
                                           an 8-byte IEEE-754 double into +0x40)
  i16  100      +0x10c  *** max quantity per purchase - NEVER 0 ***
  u8   0                the SELL flag
  u8   0                the BUY BACK flag

BUY BACK ROW, 158 bytes
  ... identical, except:
  u32  3        +0x0c   the stack count
  u64  25       +0x38   the price the player gets it back for
  i16  3        +0x10c  cap the quantity at the stack
  u8   0                the SELL flag
  u8   1                the BUY BACK flag
  u8   0                item-slot type: 0 consumes exactly one byte and decodes nothing.
                        1/2/3 = a real GW_ItemSlot into row+0x120 - see
                        research/msexe-setfield.md for its layout
```

Total for this example: **336 bytes**, `19 + 2 + 157 + 158`. The exact bytes, so an
implementation can be diffed against them rather than re-derived:

```text
0000  15 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0010  00 00 00 02 00 00 00 00 00 00 00 00 00 80 84 1e
0020  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0030  00 00 00 00 00 00 00 00 00 32 00 00 00 00 00 00
0040  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0050  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0060  00 00 00 00 00 00 00 00 00 40 e0 fd 3b 37 4f 01
0070  00 80 05 bb 46 e6 17 02 00 00 00 00 00 00 00 00
0080  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0090  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00a0  00 00 00 00 00 00 00 00 00 00 00 00 01 00 64 00
00b0  00 00 03 00 00 00 00 00 00 00 80 84 1e 00 00 00
00c0  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00d0  00 00 00 00 00 00 19 00 00 00 00 00 00 00 00 00
00e0  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
00f0  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0100  00 00 00 00 00 00 40 e0 fd 3b 37 4f 01 00 80 05
0110  bb 46 e6 17 02 00 00 00 00 00 00 00 00 00 00 00
0120  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0130  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0140  00 00 00 00 00 00 00 00 00 01 00 03 00 00 01 00
```

Strings are `u16` length then that many bytes - the same `put_str` `crates/net/src/opcode.rs`
already has.

### 9.2 The inbound requests to answer

```text
0x00F5  u8 0, u16 rowIndex, u32 itemId, u16 quantity, u32 d, str name, u32 e
        BUY, or BUY BACK if rowIndex names a row you flagged. Answer 0x055E ALWAYS.
        On success: 0x055E 00 00 <u32 itemId> <u32 remainingStock>, then the 0x0070
        inventory delta and the meso change, exactly as research/npc-shop.md §3.2 records
        for Shop2 - the result packet moves nothing by itself.
        On a buy back, follow with 0x055E type 10 + the head-less body so the Buy Back
        tab loses the row.

0x00F5  u8 1, u16 inventorySlot, u32 itemId, u16 quantity
        SELL. Answer 0x055E ALWAYS. On success, push the sold item onto the session's
        15-entry buy-back ring and send 0x055E type 10 + the head-less body.

0x00F5  u8 2, u16 inventorySlot
        RECHARGE. Answer 0x055E ALWAYS. Only reachable if row+0x40 carried a non-zero
        double, so a shop with no rechargeable rows never sees it.

0x00F5  u8 3
        CLOSE. Nothing is latched; drop the session's shop state and send nothing.
```

The head-less body for `0x055E` types 10 and 35 is everything `FUN_141f9f5e0` reads:
`u32 c, u32 d, str name, u32 e, u16 nRows, rows…` - i.e. the §9.1 body **without** the leading
`u32 a` and `u8 hasB`.

### 9.3 The three sentences worth more than the rest

> **`row+0xa4` is a guess in the sense that matters: it is not optional, and a wrong value is
> invisible.** It is compared against the wall clock with no sentinel. Send `0` and the shop
> opens completely empty with nothing in any log to say why - which reads on screen exactly like
> "the shop still does not work". Send `150842304000000000`.

> **`row+0x10c = 0` makes every purchase fail with no message**, the same trap as Shop2's row
> offset 29 (`research/npc-shop.md` §2.6). `ItemData::slot_max` is zero for 2495 of the 2785 rows
> in `gm-handbook/itemdata.txt`, so it cannot be copied straight through - reuse
> `npc-shop.md`'s `max_per_purchase` policy helper.

> **The Buy Back flag and the row-drop tests interact, and that is the one thing here that can
> crash the client rather than merely disappoint.** If a row has the Buy Back flag set *and*
> `row+0xf0 != 0` or an expired `row+0xa4`, the trailing item-slot byte is never consumed and
> every following row is parsed one byte out of phase. **Never set the Buy Back flag on a row
> that any gate might drop.** In practice: buy-back rows carry `+0xf0 = 0`,
> `+0xa4 = PERMANENT`, `+0x94 = +0x98 = 0`, `+0xac = 0`, `+0xc8 = 0` - no gates at all.

### 9.4 The one-variant test, with what each outcome means

One launch. The variant is **the shop packet on `0x055D`**; nothing else changes.

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

Lucy is NPC template 21 on map 1013. Route: `!map 1010`, walk the `in02` portal into Amherst
Department Store, click Lucy. **Do not let a `0x055B` script message be on screen when the
`0x055D` goes out** - §2.3 - so the shop branch must replace the dialogue, not follow it.

| # | do | watch | what each outcome means |
|---|---|---|---|
| 1 | click Lucy | does a shop window with the classic art appear? | **A window** = `0x055D`, the head and the row width are all right. **The old dialogue box** = the NPC->template join is still missing and no packet went out. **Nothing at all, and `client-patched\maplecw-hook.log` shows the dispatch line for `0x055D` returning** = the modal guard fired; a dialog was already open. **A freeze or a fault** = the row width is wrong; `world.log`'s last outbound length should be `21 + 157n` for a shop with no buy-back rows |
| 2 | count the rows | are all of Lucy's items there at the `data/shops.txt` prices, with " Mesos"? | **Right count, right prices** = `row+0x38` and the 157-byte row are both right. **A window with zero rows** = a gate dropped every row, and §4 lists five; the first suspect is `row+0xa4`. **Fewer rows than sent** = count how many rendered and that names which row first tripped a gate |
| 3 | is there a **Buy Back** tab? | | **Present** = the second per-row `u8` and `shopUI+0x370` are right. **Absent while you sent a flagged row** = the flagged row was dropped by a gate before `141fa0049`, which is also the desync case - check whether the rows after it rendered |
| 4 | buy one Red Potion | the item, the mesos, then **click Buy again** | **Item and mesos both move** = the whole loop works. **Nothing visible, and the second click still opens the quantity box** = the `0x055E` went out and the inventory/meso packets did not. **The second click does nothing at all, silently** = the `0x055E` never went out and `shopUI+0x4b0` is latched (§6). Closing the shop and re-clicking Lucy recovers it; a fresh `0x055D` alone will not |
| 5 | on the Buy Back tab, buy the potion back | | **It comes back and leaves the tab** = `0x055E` type 10 works. **It comes back but stays listed** = the type-10 refresh was not sent |

Steps 1-3 are three disjoint observables on one packet. A failure at step 1 makes the rest
unreadable; nothing else gates anything.

---

## 10. What I did NOT establish

* **Anything on a wire.** No client run was made and no packet was sent. Every claim here is
  static.
* **The ten row fields in §0 item 1.** Read and stored; no consumer on the meso path.
* **`c` and `e`.** `e` is at least echoed back, so it round-trips whatever the server puts
  there; `c` is not even that.
* **`b` / `window+0x1658`.** One consumer (`FUN_141fa8060`), not read.
* **Optional block A's field meanings.** Its read shape is [L] (`u32, u8, u8, str, u32, str, …`)
  and it is skipped by sending `0`; the fields were not named.
* **What `FUN_141fb9f30`'s discount actually keys on.** It reads three globals and takes the
  minimum positive candidate. On the Buy Back tab it returns 0 immediately, so the row price is
  used unchanged - which is [L] and is the only part that mattered here.
* **Whether a null `row+0x120` renders anything on a Buy Back row** - §0 item 4.
* **The 15-item cap.** That is the owner's spec and the client does not enforce it; I found nothing
  in the parse path that counts buy-back rows. The blind spot: I did not read the window's
  destructor or its scroll control, so "no cap anywhere in the client" is not what this says.
* **Whether `0x055E` may be sent without a preceding `0x00F5`.** It does not crash - the latch
  test at `141fa6730` only emits a diagnostic - but the diagnostic call at `141fa676b` was not
  followed, so "harmless" is **[I]**.
