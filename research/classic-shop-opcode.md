# The classic shop is `0x055D`. We have been sending the wrong one of two.

**2026-08-20.** `research/npc-shop-crash2.md` ended with: *"The exe holds two copies of the
literal `UI/UIShop.img/Shop` ... **Who uses them is not established.**"* This is that,
established.

Markers: **[L]** read off a listing or a capture, **[D]** derived from two or more [L] facts,
**[I]** inferred.

## The answer

**This client has two entirely separate shop windows, on two adjacent opcode pairs.** [L]

| | window | art | open | result |
|---|---|---|---|---|
| classic | `UI/UIShop.img/Shop` | **present** in `UI_000.wz` and `_Canvas_000.wz` | **`0x055D`** | `0x055E` |
| Shop2 | `UI/UIWindow2.img/Shop2` | **absent from this client's WZ** | `0x0560` | `0x055F` |

`crates/net/src/shop.rs` builds `0x0560`. That is the one whose art does not exist, which is
why it killed the client twice and why one correctly-formed row killed it exactly as twelve
did - the constructor dies before any row byte is read.

**Goal F is no longer blocked on finding an opcode.** It is now an ordinary decode job, and
the part that remains undone is named at the bottom of this file.

## How it was found

### 1. The strings said the classic window composes its paths

The Shop2 window uses full path literals - `UI/UIWindow2.img/Shop2/backgrnd` and eight more,
all `lea`'d directly. The classic one does not, which is why a search for its nodes came back
empty. Its `.rdata` neighbourhood is **fragments**: [L]

```text
0x14341ce80  'TabShop'
0x14341ceb8  'repurchaseInfo'
0x14341ced8  'mesoBox'
0x14341cf00  '/BtBuy'
0x14341cf10  '/BtSell'
0x14341cf48  '/TabSell'
0x14341cf60  '/VScr'
0x14341cf88  'TabBuyTag/%d'
0x14341cfc8  '/citizenshipBackgrnd/%d'
0x14341cff8  'UI/UIShop.img/Shop/BtRecharge'
```

`/citizenshipBackgrnd/%d` is the tell that this is the **Classic World** shop and not a
leftover: citizenship is goal H, and the shop's background is graded by it.

`UI/UIShop.img/Shop` itself has **zero** references of any kind - no `lea`, no RIP-relative
access, no stored qword pointer. The instrument was checked against a positive control first:
`0x14336c940`, the Shop2 background path, resolves to the documented `lea` at `0x140d22415`
and nothing else. So the negative is a property of the binary, not of the search. The base
path is presumably composed too, or it comes from `.themida`. **[L]** for the negative,
**[I]** for the explanation.

The fragments *are* referenced, and they land in three functions:

| function | size | references |
|---|---|---|
| `FUN_141f9f5e0` | 12284 | `TabShop` |
| `FUN_141fa2670` | 10326 | `repurchaseInfo`, `/BtBuy`, `TabBuyTag/%d` |
| `FUN_141faa150` | 2416 | `/citizenshipBackgrnd/%d` |

`FUN_141fa2670` has **zero call sites and three qword pointers into vtables** - it is a
virtual, slot 4 of both a 79-slot and a 160-slot table. This is exactly the case
`tools/callers.py` was fixed for on 2026-08-19: a call-only scan would have called it dead
code.

### 2. `CField::OnPacket` is a chain of range tests, and both shops are in it

`FUN_141820080` is `CField::OnPacket` (already named in
`research/msexe-droppool-onpacket.txt`). It dispatches by a chain of the unsigned-range
idiom. From the listing: **[L]**

```asm
>000141821fc0  lea      eax, [r9 - 0x55d]
 000141821fc7  cmp      eax, 1
 000141821fca  ja       0x141821fdc
 000141821fcc  mov      rdx, rbx
 000141821fcf  mov      ecx, r9d          ; the opcode itself is passed in
 000141821fd2  call     0x141fa66c0       ; <- the CLASSIC shop
...
>000141822011  lea      eax, [r9 - 0x55f]
 000141822018  cmp      eax, 1
 00014182201b  ja       0x14182202d
 000141822023  call     0x140d225f0       ; <- Shop2, the one whose art is missing
```

**The chain is validated by two entries we already know.** Immediately between them:

```asm
>000141821fdc  lea      eax, [r9 - 0x1a0]
 000141821fe3  cmp      eax, 3
 000141821fee  call     0x142097ee0       ; 0x01A0..0x01A3 -> SetField
```

`0x01A0` is `SetField` and its handler is the documented `FUN_142097f80` family. And
`0x140d225f0` is in the Shop2 window's own address range, whose constructor
`FUN_140d22310` is the one `npc-shop-crash2.md` measured loading the missing
`UIWindow2.img/Shop2/backgrnd`. Two independent known facts land where this reading predicts.
**[D]**

## The `0x055D` body, as far as it is decoded

**Two instruments agree** on the order below - the listing at `0x141fa767b` and the
decompiler - which is the standing rule in `docs/ghidra.md`. `tools/reads.py` independently
lists the same reads at the same addresses in `FUN_141f9f5e0`.

```text
--- FUN_141fa66c0, the 0x055D arm at 141fa767b -------------------------------
  (guard) if the shop singleton 0x143AA8520 is NOT null, the packet is DISCARDED
          and FUN_141fb94c0 runs instead. Sending 0x055D twice does nothing.   [L]

u32   a          141fa768f   -> FUN_141f9f400(a) constructs the window
u8    hasB       141fa769b
u32   b          141fa76a7   ONLY when hasB != 0; stored at window+0x1658

--- FUN_141f9f5e0, called with (window, packet) ------------------------------
u32   c          141f9f6c4   -> window+0x310   (qword index 0x62)
u32   d          141f9f6d3   -> window+0x3f8   (qword index 0x7f)
str   name       141f9f6e9   via FUN_1402d2bb0: a string, copied with a limit of
                             0x1f characters, then
u32   e          141f9f6e9   the same helper's second read
u16   nRows      141f9f75d

  per row, nRows times:
    u32   rowKey?    141f9f802
    <item>           141f9f817   FUN_1404ba100, thirteen fields - below
    u8               141f9f81f
    u8               141f9f834
```

### `FUN_1404ba100` - the per-row item, and it is bespoke

**One caller, and it is this shop.** [L] It is not the general item decoder that `bag.rs`
builds, so nothing we already have can be reused as-is.

```text
u32   -> +0x04
u32   -> +0x08
u32   -> +0x20
u32   -> +0x10
u32   -> +0x24
raw 8 -> +0x28        FUN_1406e9170(pkt, dst, 8)
raw 4 -> +0x30
u64   -> +0x38        FUN_1406e8f10
u32   -> +0x48
u32   -> +0x4c
u32   -> +0x50
u32   -> +0x54
u32   -> +0x58
u8    flag            when non-zero, a further block runs (NOT decoded)
```

A row carrying a full item structure rather than an id and a price is what lets the classic
window draw real item stats in its tooltip. **[I]**, from the shape.

## What is NOT done, and why nothing was implemented

**No packet was built and nothing was wired.** The shop has killed the client twice, both
times from a body the server was confident about, and this body is **not** fully decoded:

* **The meanings of `a`, `b`, `c`, `d` and `e` are unknown.** `a` is the window factory's
  only argument, so "NPC template id" is the obvious reading and is **[I]**, not [L].
* **`FUN_1404ba100`'s conditional tail is unread.** A row whose flag byte is non-zero reads
  more, and how much is unknown. Sending `0` for it is the safe first move but is untested.
* **Nothing establishes which of the thirteen row fields is the price**, which is the one
  field a shop cannot be wrong about.
* `0x055E`, the result packet, has a type byte with at least a `case 0` that reads
  `u8, u32, u32` - undecoded beyond that.

The next step is `FUN_141fa2670` (the layout) and `FUN_141f9f5e0`'s row loop read closely
enough to name the fields, exactly the way `research/npc-shop.md` did it for Shop2. That file
is the template for this work and most of its method transfers.

**SUPERSEDED 2026-08-28.** Everything this section lists as undone was done by
`research/classic-shop-rows.md` on 2026-08-22 - the price (`row+0x38`), the conditional tail,
the head fields, the `0x055E` table - and the request opcode that section never found is
`0x00F5`. `crates/net/src/classicshop.rs` now builds `0x055D` against that file's golden
vector, and `crates/world/src/session/shop.rs` sends it. Shops are on by default.

The warning below was right and was heeded: the body **was** decoded before the opcode was
changed. `crates/net/src/shop.rs` still builds `0x0560` and nothing calls it - it is kept for
its result table and because the two-window fact is worth being able to point at.
