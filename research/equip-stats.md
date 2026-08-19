# What the equip item's four bitmasks actually carry

**Written 2026-08-19, entirely statically. No client run was spent.**

`research/naked-character.md` established the 125-byte type-1 item body and said the three
`u32` bitmasks gate "17 optional `u16`" and "21 optional mixed-width fields" without naming
any of them. This document names them, bit by bit, from **this binary**, and answers the
three things the owner's Equipment-window screenshot raised.

Labels: **[L]** read off a listing, the WZ, or a capture. **[D]** derived from two or more
[L] rows. **[I]** inferred - including anything from the v214 reference tree, which is a
different game version and is a candidate, never a fact.

---

## 0. The three answers, first

| question | answer | label |
|---|---|---|
| Do the stat fields carry the item's stats? | **Yes - they are the item's TOTAL stats, and a fresh Undershirt must be sent with `inc_pdd = 6`.** The tooltip prints the packet value alone and *subtracts* `ITEMINFO.incPDD` from it; a separate function compares the two directly in the same units. Zeros are suppressed by a print guard, which is why the screenshot has no stat section. Section 8 | [L] |
| Why "Cannot be Traded when equipped"? | Because we send **`scissor_uses = 0`**. `FUN_1402fd610` (`vtable+0x200`) returns true for any item with `scissor_uses <= 20`, and that makes `FUN_14038cf10` report a trade restriction. The wording then comes from the WZ. **The fix is one packet byte:** option-mask **bit 18** set to a value **> 0x14**. | [L] |
| Are the six `REQ` zeros wrong? | **No.** No requirement value exists anywhere in the 125-byte body; `reqLevel/STR/DEX/INT/LUK/POP/Job` live in `ITEMINFO`, loaded from the WZ, and `01040002.img/info` really does have all of them at 0. Nothing to fix. | [L] |

The one value that **is** genuinely wrong at zero besides `scissor_uses` is
**`remaining_enhancements`** (option-mask bit 0): the client prints it straight from the
packet with no WZ fallback, and `01040002.img/info/tuc` is **7**.

---

## 1. How the bits were read, and the control that says the instrument works

Three instruments, and they agree everywhere they overlap.

1. **The decode listings.** `FUN_140303800` (`research/msexe-itemslot-800.txt`,
   `0x140303800..0x140303a6c`, bounded by `.pdata`) is `u32 mask` then, for bit *k*,
   `CALL 0x1406e8b80` (u16) and `CALL FUN_1402f7010(value, base + 8k)` with the result
   stored at `base + 8k + 4`. So **bit *k* of that mask writes an 8-byte obfuscated pair at
   `base + 8k`**, value at `+0`..`+4`, integrity dword at `+4`. **[L]**
   `FUN_140303b40` (`research/msexe-itemslot-b40.txt`) calls `FUN_140303800(base)` first,
   then reads its own `u32` mask and 21 mixed-width fields starting at `base + 0x98`. **[L]**

2. **The getters.** `FUN_1401ab420(ptr, checksum)` returns `ptr[2..4] ^ ptr[0..2]` as a
   `u16`; `FUN_1401b0050(ptr, checksum)` returns `ptr[1] ^ ptr[0]` as a `u8`. Both raise
   through `FUN_141804970` when the checksum disagrees. **[L]**, `research/msexe-tooltip-stats.c`.

3. **The tooltip.** `FUN_1426b20f0` and its five helpers call those getters on
   `item + <offset>` and pass the result to `FUN_142699710` together with a **string id**
   resolved by `FUN_1408a9e40`. `tools/dump_stringids.py` decrypts the id. That pairs an
   item offset with an English label in a single statement. **[L]**

**Positive control for instrument 3**: the string search for `'Cannot be Traded'` in the exe
returns 0 hits (the message table is XOR-encrypted), while the same search for
`'UI/Login.img'` returns 67. The negative was a property of the search, exactly as
`CLAUDE.md` warns; `tools/dump_stringids.py` finds the string as id **0x03C6**.

**Cross-check between instruments 1 and 3, nine times over.** The tooltip reads
`item+0xfa`, `0x10a`, `0x146`, `0x152`, `0x186`, `0x18e`, `0x196`, `0x19e`, `0x1a6` with the
checksum always at `+4` (and `+0x10` for the two `u64` slots). Every one of those lands
exactly on an offset the `FUN_140303b40` listing writes, in the right width. If the offset
arithmetic were wrong, none of them would line up.

**Fourth instrument, independent of the exe's code: the WZ.** `FUN_1403b5130` is the
`ITEMINFO` loader; `research/msexe-iteminfo-load.txt` is its listing. Pairing each WZ
property name with the struct offset it is stored to gives
`incWAT -> +0xca, incPAD -> +0xcc, incMAD -> +0xce, incPDD -> +0xd0, incMDD -> +0xd2,
incACC -> +0xd4, incEVA -> +0xd6, incCRT -> +0xd8, incCRD -> +0xda, incSpeed -> +0xdc,
incJump -> +0xde` and `incSTR/incDEX/incINT/incLUK/incMHP/incMMP -> +0xba..+0xc4`. The
tooltip reads **those same ITEMINFO offsets** as the base value for each line. So each label
is confirmed twice: once by the message-table string, once by the WZ property name. **[D]**

**The `LEA`-shaped scan that lied.** The first search for readers of `item+0x10a` looked for
`LEA r64,[r64+0x10a]` and returned **zero**. The compiler emits `ADD RCX,0x10a`. A
shape-blind scan for the 4-byte value in `.text` found 32 functions. Same failure mode as
the two in `CLAUDE.md`; the scanner is `scratchpad/dword_scan.py` and it filters nothing.

---

## 2. Mask 1 - `FUN_140303800`, 17 optional `u16`

Called four times on an equip. The **first** call is the one that matters: inside
`FUN_140303b40`, whose base is `item + 0x62` (`LEA RCX,[RSI+0x62] / CALL 0x140303b40` at
`0x14030411f`). **[L]**

Bit *k* is a `u16`, present iff bit *k* of the mask is set, written to `item + 0x62 + 8k`.
Every row's label is the string id the tooltip formats it with.

| bit | item offset | WZ base field | string id | label | label |
|---|---|---|---|---|---|
| 0 | 0x62 | `incSTR` (ITEMINFO+0xba) | 0x0663 | `STR: +%d` | [L] |
| 1 | 0x6a | `incDEX` (+0xbc) | 0x0664 | `DEX: +%d` | [L] |
| 2 | 0x72 | `incINT` (+0xbe) | 0x0665 | `INT: +%d` | [L] |
| 3 | 0x7a | `incLUK` (+0xc0) | 0x0666 | `LUK: +%d` | [L] |
| 4 | 0x82 | `incMHP` (+0xc2) | 0x0668 | `MaxHP: +%d` | [L] |
| 5 | 0x8a | `incMMP` (+0xc4) | 0x0669 | `MaxMP: +%d` | [L] |
| 6 | 0x92 | `incSpeed` (+0xdc) | 0x038A | `Speed: +%d` | [L] |
| 7 | 0x9a | `incJump` (+0xde) | 0x038B | `Jump: +%d` | [L] |
| 8 | 0xa2 | `incPAD` (+0xcc) | 0x0380 | `Attack Power: +%d` | [L] |
| 9 | 0xaa | `incMAD` (+0xce) | 0x0381 | `Magic Attack: +%d` | [L] |
| 10 | 0xb2 | `incPDD` (+0xd0) | 0x0383 | `Weapon Def.: +%d` | [L] |
| 11 | 0xba | `incMDD` (+0xd2) | 0x0384 | `Magic Def.: +%d` | [L] |
| 12 | 0xc2 | `incACC` (+0xd4) | 0x0385 | `Accuracy: +%d` | [L] |
| 13 | 0xca | `incEVA` (+0xd6) | 0x0386 | `Evasion: +%d` | [L] |
| 14 | 0xd2 | `incCRT` (+0xd8) | 0x0387 | `Critical Rate: +%d` | [L] |
| 15 | 0xda | `incCRD` (+0xda) | 0x0388 | `Critical Damage: +%d` | [L] |
| 16 | 0xe2 | `incWAT` (+0xca) | 0x037F | `Weapon Attack: +%d` | [L] |

Where each row comes from, so it can be re-checked without re-deriving:

* bits 0-3: `FUN_1426af680`, `research/msexe-tooltip-stats.c` lines 1554-1580
* bits 4-5: `FUN_1426af8c0`, lines 1633-1643
* bits 6-7: `FUN_1426b20f0`, `research/msexe-tooltip.c` lines 163-173
* bits 8-16: `FUN_1426afdc0`, and this one was cross-checked against the **listing**
  `research/msexe-tooltip-stats.txt` instruction by instruction, because nine of the
  seventeen rows depend on it. The listing shows `MOV EDX,0x37f` then
  `MOV EDX,[RSI+0xe6] / CALL 0x1401ab420`, and so on for all nine. The decompiler did not
  reorder anything here.

**The expectation from the game family was [I] and it was wrong about the order.** The
classic set is STR/DEX/INT/LUK/MaxHP/MaxMP/PAD/MAD/PDD/MDD/ACC/EVA/Craft/Speed/Jump. This
client puts **Speed and Jump at bits 6-7**, has **no Craft field**, and adds
**Critical Rate, Critical Damage and a separate Weapon Attack** at bits 14-16. Anyone who
had built the packet from the family expectation would have put `incPAD` at bit 6 and
desynchronised the record.

### 2.1 The same 17-bit layout appears three more times

`FUN_140303800` is called on four different bases in one item:

| base | when | read by | label |
|---|---|---|---|
| `item + 0x62` | always, from `FUN_140303b40` at `0x140304126` | the tooltip's "+n" column | [L] |
| `item + 0x26b` | always, at `0x1403043f6` | nothing in the tooltip | [L] for the layout, meaning **not established** |
| `item + 0x313` | only when the `u8` tailFlag at `0x1403043fe` is non-zero | the tooltip's *second* value argument on every stat line (`item+0x313 + 8k`) | [L] |

The `item+0x26b` struct also has two `u8` read **unconditionally before** its mask, at
`0x14030436d` and `0x1403043b1`, into `0x303` and `0x30b` - which are the `+0x98`/`+0xa0`
slots of a `FUN_140303b40`-shaped struct. So `+0x26b` is a full option struct whose option
mask is never read. **[L]**

---

## 3. Mask 2 - `FUN_140303b40`'s own mask, 21 optional fields

Base `item + 0x62 + 0x98` = **`item + 0xfa`**. Widths straight off the listing.

| bit | width | item offset | what | label |
|---|---|---|---|---|
| 0 | u8 | 0xfa | **Remaining Enhancements** - string 0x039E `Remaining Enhancements: %d` | [L] |
| 1 | u8 | 0x102 | not established | - |
| 2 | **u16** | 0x10a | **the attribute bitfield** - section 4 | [L] |
| 3 | u8 | 0x112 | not established | - |
| 4 | u8 | 0x11a | not established | - |
| 5 | u64 | 0x122 | not established | - |
| 6 | u32 | 0x13a | not established | - |
| 7 | u32 | 0x146 | read by `FUN_1401ba9d0(item+0x146, [item+0x14e])`; a positive value plus a WZ check prints 0x0DA7 `Golden Hammer reforging applied` | [L] |
| 8 | u8 | 0x152 | string 0x0394 `Required Level: -%d` - a **reduction**, not a requirement | [L] |
| 9 | u16 | 0x15a | not established | - |
| 10 | u32 | 0x162 | not established | - |
| 11 | u8 | 0x16e | not established | - |
| 12 | u8 | 0x176 | not established | - |
| 13 | u8 | 0x17e | not established | - |
| 14 | u8 | 0x186 | string 0x0D65 `Boss Damage +%d%%` | [L] |
| 15 | u8 | 0x18e | string 0x0672 `Ignored Enemy DEF : +%d%%` | [L] |
| 16 | u8 | 0x196 | string 0x0389 `Damage: +%d%%` | [L] |
| 17 | u8 | 0x19e | string 0x0671 `All Stats: +%d%%` | [L] |
| 18 | u8 | 0x1a6 | **Scissors uses** - string 0x03A0 `Scissors Usages Available : %d`. Section 5 | [L] |
| 19 | u64 | 0x1ae | not established | - |
| 20 | u32 | 0x1c6 | not established | - |

The width column matches `naked-character.md` section 3.3 exactly; this pass re-read it off
the listing rather than trusting the earlier table.

Candidate names for the twelve "not established" rows are in section 10. They are **[I]**
and they are kept out of this table on purpose.

In the item object the pairs are `{key, key^value, checksum: u32}` at the field's own width:
`u8` puts the checksum at `+4`, `u16` at `+4`, `u32` at `+8`, `u64` (via `FUN_1402f7170`) at
`+0x10`. That is why every tooltip getter above passes `[offset + 4]` and the two `u64` rows
would pass `+0x10`. Only the **wire** size matters to us, and that is the column above.

---

## 4. The attribute bitfield - mask 2, bit 2, a `u16` at `item + 0x10a`

Twelve one-line accessors read it, each masking one bit and shifting it down. All twelve are
`ADD RCX,0x1a6`-shaped reads of `item+0x10a` with the checksum at `item+0x10e`; found by
enumerating every function in `0x1402f0000..0x140310000` that mentions the dword `0x10a`.
**[L]**

| attribute bit | accessor | item vtable slot |
|---|---|---|
| 0 | `FUN_1402FDCC0` | +0x10 |
| 1 | `FUN_1402FDC20` | +0x18 |
| 2 | `FUN_1402FDF20` | +0x20 |
| 3 | `FUN_1402FD540` | **+0x28** - the trade-line gate, section 5 |
| 4 | `FUN_1402FDB20` | +0x38 (plus extra conditions) |
| 6 | `FUN_1402FDF80` | +0x08 |
| 7 | `FUN_1402FD9A0` | +0x50 |
| 8 | `FUN_1402FD4D0` | +0x58 |
| 9 | `FUN_1402FD950` | +0x60 |
| 12 | `FUN_1402FD3D0` | +0x68 |
| 13 | `FUN_1402FDD00` | +0x70 |
| 14 | `FUN_1402FDDF0` | +0x78 |

Bits 5, 10, 11 and 15 have no single-bit accessor of this shape. That is **not** evidence
they are unused - the scan enumerated one shape. Do not treat their absence as meaning.

---

## 5. "Cannot be Traded when equipped", end to end

The tooltip is assembled by `FUN_14264f750`, which calls **both** `FUN_1426e10e0` (the
restriction lines) and `FUN_1426b20f0` (the stat lines). `tools/callers.py` says each has
exactly one call site and it is that function. **[L]**

`FUN_1426e10e0` decides whether to print a restriction line with
**`FUN_14038cf10(ItemInfoMgr, item)`**, whose listing is `research/msexe-tradeflag.txt`:

```text
14038cf2b  CALL [vtable+0x28]        ; attribute bit 3
14038cf30  JNZ  -> return 0          ; set  => no line at all
14038cf40  CALL FUN_14038ce90        ; exchangeableOnce || equipTradeBlock   (WZ)
14038cf47  JNZ  -> return 1
14038cf4f  CALL [vtable+0x200]       ; FUN_1402fd610
14038cf57  JZ   -> return 0
14038cf67  CALL FUN_14038aae0        ; tradeBlock                            (WZ)
14038cf6e  JNZ  -> return 0
14038cf70  return 1
```

and `FUN_1402fd610`, the `vtable+0x200` in the middle of it, is seven instructions:

```text
1402fd614  CMP  qword ptr [RCX+0x38],0     ; the cash-item serial
1402fd61c  JZ   0x1402fd625                ; serial == 0 -> keep going
1402fd61e  XOR  AL,AL / RET                ; serial != 0 -> false
1402fd625  MOV  EDX,[RAX+0x1aa]            ; the checksum
1402fd62b  ADD  RCX,0x1a6                  ; mask-2 bit 18: scissor uses
1402fd632  CALL FUN_1401b0050
1402fd637  CMP  AL,0x14 / SETBE AL         ; true when uses <= 20
```

**So for a non-cash item, `vtable+0x200` is "scissor uses <= 20", and 0 satisfies it.** [L]

The wording then comes from `FUN_1403e8e00(out, itemId, 0)`: for an itemId in
`[1000000, 2000000)` with `exchangeableOnce == 0` it picks string **0x03C6**,
`Cannot be Traded when equipped`. **[L]**

The WZ side, all read from the `ITEMINFO` loader listing and confirmed against
`Character/Coat/Coat_000.wz -> 01040002.img/info` with `wz-dump cat`:

| helper | reads | WZ property | ITEMINFO offset | value for 1040002 |
|---|---|---|---|---|
| `FUN_140389c10` | `cash` | `cash` | +0x18 | 0 |
| `FUN_14038aae0` | `tradeBlock` | `tradeBlock` | +0x1c | absent -> 0 |
| `FUN_14038abd0` | `exchangeableOnce` | `exchangeableOnce` | +0x20 | absent -> 0 |
| `FUN_14038ce90` | `exchangeableOnce`, else `equipTradeBlock` | `equipTradeBlock` | +0x1b4 | absent -> 0 |
| `FUN_1403e8c40` | `tradeAvailable` | `tradeAvailable` | +0x1a8 | absent -> 0 |

`equipTradeBlock`'s default is provably 0: `XOR R8D,R8D / LEA RDX,[L"equipTradeBlock"] /
CALL 0x140910eb0 / TEST EAX,EAX / SETNZ CL / MOV [R14+0x1b4],ECX` at `0x1403b6a9d`. **[L]**

Substituting our packet - attribute 0, cash serial 0, scissor uses 0 - and the WZ's zeros,
`FUN_14038cf10` returns **1** and the wording is **0x03C6**. That is exactly the line on
The owner's screen, and the same `vtable+0x200` also explains why
`Scissors Usages Available : 0` printed (`FUN_1403e8c40` ends in
`JMP qword ptr [RAX+0x200]`). **Both wrong lines have one cause.** **[D]**

### 5.1 What to change

**Set mask-2 bit 18 to a value greater than 0x14.** That makes `vtable+0x200` false, which
removes the `Scissors Usages Available` line and, with `equipTradeBlock == 0` and
`tradeBlock == 0`, removes `Cannot be Traded when equipped` too. Cost on the wire: **one
byte per item**, plus the mask bit that is already in the `u32` we send.

`0xFF` is the value to send. **[D] for "> 0x14"**, and **[D] for "0xFF specifically"** -
upgraded from [I] once `FUN_1403e8d40`, the one function that tests for exactly `0xFF`, was
read end to end. It returns 1 only when the field reads `0xFF` **and** the item is otherwise
unrestricted, and its single caller (`FUN_1417e2c90`, the Rebirth Flame confirmation)
uses that to append string **0x0F66**, `"However, this will restrict the Scissors count."`

So `0xFF` means **"no karma restriction yet"** - the warning exists precisely because
applying a flame *would* impose one. It is not a different special case we would be walking
into: the only behaviour `0xFF` changes anywhere is that one warning line, and that line is
correct for an unrestricted item. `FUN_1402fb900`, the item's own decrement method, stops at
zero, which is consistent with a count that runs downward from a sentinel.

Still short of [L]: nothing in the binary was found that *writes* `0xFF` into the field.

The alternative - setting **attribute bit 3** - also silences the line (`JNZ -> return 0` on
the first instruction) but costs 2 bytes plus the mask bit, and bit 3 is read by three other
predicates whose meaning is not established. Prefer the scissors byte.

---

## 6. `REQ LEV / STR / DEX / INT / LUK / FAM` - the WZ, not the packet

Three facts, each [L]:

1. **The 125-byte body has no requirement field.** Every byte is accounted for by
   `naked-character.md` section 3.3 plus the four mask tables above, and the only
   requirement-shaped optional anywhere in them is mask-2 bit 8, which formats as
   `Required Level: -%d` - a *reduction* applied on top of a requirement that must come from
   somewhere else.
2. **`ITEMINFO` carries all six**, from the WZ: `reqSTR -> +0x58`, `reqINT -> +0x5c`,
   `reqDEX -> +0x60`, `reqLUK -> +0x64`, `reqPOP -> +0x68` (this is `REQ FAM`),
   `reqJob -> +0x6c`, `reqLevel -> +0x78`. Read off `research/msexe-iteminfo-load.txt`.
   `FUN_14038c560` additionally looks `reqLevel` up live from the WZ node at runtime.
3. **The tooltip assembler reads them from `ITEMINFO`**: `CMP dword ptr [RBX+0x6c],0x2` and
   `CMP dword ptr [RBX+0x70],0x0` at `0x14264fcc7`/`0x14264fc9a`, with `RBX` the `ITEMINFO`
   pointer. That is `reqJob` and `reqSpecJob`.

And `01040002.img/info` has `reqLevel 0, reqSTR 0, reqDEX 0, reqINT 0, reqLUK 0, reqJob 0`
and no `reqPOP` at all. **The six zeros on screen are correct.** Nothing to fix, and a
"fix" would have had nowhere to go.

**Bound on that, stated honestly:** I did not locate the function that *draws* the
`REQ LEV :` panel - its labels are bitmaps, not strings (`CLAUDE.md`, baked UI text), so
there was nothing to search for. Fact 1 is what settles it: a value that is not in the
packet cannot have come from the packet.

---

## 7. `Remaining Enhancements` and `Scissors Usages Available`

Both are mask-2 fields, both `u8`, and the client uses the **packet** value with no WZ
fallback.

| line | mask 2 bit | item offset | is 0 correct? |
|---|---|---|---|
| `Remaining Enhancements: %d` (0x039E) | **0** | 0xfa | **No.** The line only prints when `ITEMINFO.tuc != 0` (`*(char *)(itemInfo+0xb8)`), and `01040002.img/info/tuc` is **7**, so a fresh Undershirt should read **7**. **[L]** for the field, the guard, **and now the range**: `FUN_14038d3c0` compares this field against `ITEMINFO.tuc` directly (`CMP AL,byte ptr [RDI+0xb8]` at `0x14038d41c`), so it is constrained to `0..=tuc` - `>= tuc` reads as "not fresh" on one path and `> tuc` on the other. **[I]** remains only on "`tuc` exactly is the fresh value" rather than some smaller number; nothing writes it |
| `Scissors Usages Available : %d` (0x03A0) | **18** | 0x1a6 | **No** - and this is the trade bug. Section 5 |

The `Remaining Enhancements` guard is worth writing down because it is why the line appeared
at all: `if (itemInfo[0x328] == 0 && !trialMode && itemInfo[0xb8] != 0)`. `+0xb8` is WZ
`tuc`, `+0x328` is WZ `exceptUpgrade`. **[L]**, `research/msexe-tooltip.c` around line 840.

---

## 8. What the stat fields are *for* - **corrected 2026-08-19, this section was wrong**

### 8.0 The retraction

The first version of this section said, labelled **[L]**:

> *the stat fields carry the increment on top of the WZ base. The tooltip renders
> `ITEMINFO.incPDD + packet.incPDD`. A fresh, unscrolled starter item's increments really
> are 0.*

**That is wrong, and it should never have been labelled [L].** What I actually measured was
that `FUN_142699710` is called with *both* the `ITEMINFO` value and the packet value as
arguments. From that I inferred "it renders their sum" and wrote the inference down as a
reading of the listing. **I never opened `FUN_142699710`.** That is precisely the failure
`CLAUDE.md` names - a plausible inference committed as a fact - and it would have sent the owner
into a client run with the wrong model.

The tell was in the screenshot the whole time and I explained it away: an Undershirt with
`incPDD 6` printed **no Weapon Def. line at all**. Under the "increment" reading the client
had a 6 in hand and chose not to show it, which I should have treated as a contradiction
rather than as an omission in the owner's transcription.

### 8.1 What the assembler actually does

`FUN_142699710` is the printf-style line assembler; listing at
`research/msexe-statline.txt`, bounded `0x142699710 .. 0x14269a1bd`. Its seven arguments,
from the prologue (`RBP = entry - 0x47`, so arg 5/6/7 are `[RBP+0x6f]`/`[RBP+0x77]`/`[RBP+0x7f]`):

| arg | register | what the caller passes |
|---|---|---|
| 1 | RCX | the tooltip |
| 2 | RDX | a scratch string |
| 3 | R8D -> EDI | `ITEMINFO.inc*` - the **WZ template** value |
| 4 | R9D -> ESI | a baseline from the fourth struct `FUN_140396250` fills |
| 5 | `[RBP+0x6f]` -> R8D | **our packet value**, `item + 0x62 + 8k` |
| 6 | `[RBP+0x77]` -> R13D | the tailFlag set, `item + 0x313 + 8k` |
| 7 | `[RBP+0x7f]` | the label string |

**The guard**, `0x142699745`:

```text
142699745  MOV  R8D,dword ptr [RBP + 0x6f]   ; arg 5 - the PACKET value
142699749  TEST R8D,R8D
14269974c  JG   0x142699757                  ; > 0 -> print
14269974e  TEST R9D,R9D                      ; arg 4 - the baseline
142699751  JLE  0x14269a19d                  ; both <= 0 -> RETURN, print nothing
```

**`EDI` - the WZ value - is not tested.** The template alone can never make a line appear.

**The headline**, `0x142699757`-`0x142699771`:

```text
142699757  MOV  R15D,R8D                     ; packet
14269975a  SUB  R15D,EDI                     ;   - the WZ template value
14269975d  SUB  R15D,ESI                     ;   - the baseline
142699760  MOV  R13D,dword ptr [RBP + 0x77]
142699764  SUB  R15D,R13D                    ;   - the tailFlag set
142699767  MOV  RDX,qword ptr [RBP + 0x7f]
14269976b  MOV  RDX,qword ptr [RDX]          ; the label, e.g. "Weapon Def.: +%d"
14269976e  MOV  RCX,RBX
142699771  CALL 0x14019ba10                  ; sprintf(scratch, label, R8D)
```

`R8D` is loaded at `0x142699745` and **never written again** before the call - the only
writes in between are `R15D`, `R13D`, `RDX`, `RCX` - so the `%d` the user reads is the
**packet value alone**. **[L]**

`R15D = packet - template - baseline - timed` is then the leftover, and the rest of the
function emits the breakdown with three format strings resolved from `.rdata`:

| string | fed with |
|---|---|
| `" (%d"` at `0x143479558` | `EDI`, the **WZ template** value |
| `" +%d"` at `0x143479560` | `ESI` (baseline), `R15D` (leftover), `R13D` (timed) |
| `" %d"` at `0x1433de688` | `R15D` (leftover) |

So the rendered line is `Weapon Def.: +N (base +scroll +bonus)`, **N being the packet's
total**, decomposed using the WZ value as the base term. And the short path - just the
headline, no parenthetical - is taken exactly when `baseline <= 0 && leftover == 0 &&
timed == 0`, i.e. **when the packet value equals the WZ template value**. That condition is
only coherent if the packet carries the total.

### 8.2 The independent confirmation, from a different subsystem

`FUN_14038d3c0(ItemInfoMgr, item, strict)` - listing at
`research/msexe-itemvstemplate.txt` - walks the item against its template field by field:

```text
14038d3f0  CALL 0x140388c60                       ; RDI = ITEMINFO for this itemId
14038d401  MOV  EDX,dword ptr [RBX + 0xfe]        ; the checksum
14038d407  LEA  RCX,[RBX + 0xfa]                  ; mask-2 bit 0: remaining enhancements
14038d40e  CALL 0x1401b0050
14038d41c  CMP  AL,byte ptr [RDI + 0xb8]          ; vs ITEMINFO.tuc
14038d422  JNC  0x14038d911                       ; -> return 1
14038d428  MOV  EDX,dword ptr [RBX + 0x66]
14038d42b  LEA  RCX,[RBX + 0x62]                  ; mask-1 bit 0: inc_str
14038d42f  CALL 0x1401ab420
14038d434  CMP  AX,word ptr [RDI + 0xba]          ; vs ITEMINFO.incSTR
14038d43b  JGE  0x14038d911                       ; -> return 1
...  0x6a/0xbc, 0x72/0xbe, 0x7a/0xc0, 0x82/0xc2, 0x8a/0xc4, 0xa2/0xcc, 0xaa/0xce ...
```

`CMP AX, word ptr [RDI+0xba]` compares the **packet field** with the **WZ template field**,
same width, same units, no arithmetic on either side. That test is meaningless if one side
is an increment and the other an absolute. It returns 1 when any field is at or above its
template (`strict`) or above it (the `0x14038d651` path, `JA`/`JG`), and 0 otherwise - the
client's own "is this item still its template, or has it been enhanced" test. **[L]**

### 8.3 What this means for the server

* **A fresh equip must be sent with its WZ `info/inc*` values.** For 1040002 that is
  `inc_pdd = 6` and nothing else. That needs a per-item generator over `Character.wz` /
  `Item.wz`; `crates/net::EQUIP_STAT_WZ_PROPERTIES` gives the 17 property names in mask-bit
  order and `EquipStatSet::from_wz_template` takes them in that order.
* **Sending zeros is not "correct for an unscrolled item"** - it is an item with no stats,
  and the guard hides the whole section rather than showing zeros. The owner's complaint that
  "the items do not have the stats they should possess" was **right**, and my previous
  answer to it was wrong.
* The cost is 2 bytes per non-zero stat. A starter Undershirt goes from 125 to 129 bytes
  (one `u16` stat, plus the two `u8` option fields from section 5.1).

### 8.4 What the third answer would have looked like, and why it is ruled out

The coordinator's option 3 was "the section is built somewhere I have not looked".
`tools/callers.py` gives `FUN_1426b20f0` **one** call site, `FUN_14264f750`, which is the
same function that calls `FUN_1426e10e0` for the restriction lines that *did* appear on
screen. So the stat section and the trade line are assembled by one parent, and the stat
section's absence is a decision inside `FUN_142699710`, not a different code path. **[L]**

---

## 9. Files from this pass

| file | what |
|---|---|
| `research/msexe-tooltip.c` | `FUN_1426b20f0` the equip tooltip, `FUN_1403e8e00` the trade-wording chooser, the `FUN_1402f7010` obfuscated-store family |
| `research/msexe-tooltip-stats.c` | the five stat-line helpers, the two getters `FUN_1401ab420`/`FUN_1401b0050`, `FUN_1426e10e0` the restriction-line builder |
| `research/msexe-tooltip-stats.txt` | the **listing** of `FUN_1426afdc0` - the authority for nine of the seventeen mask-1 rows |
| `research/msexe-tradeflag.c` / `.txt` | `FUN_14038cf10` and the WZ predicates around it, decompiled and as a listing |
| `research/msexe-scissorsflag.txt` | `FUN_1403e8c40`, whose tail `JMP [RAX+0x200]` is what makes the scissors line print |
| `research/msexe-itemprops.c` | `FUN_1403e8c40`, `FUN_14038c560` (`reqLevel` from the WZ), `FUN_14038c0f0`, `FUN_14038c3c0`, `FUN_1403e8d40` |
| `research/msexe-iteminfo-load.txt` | the **listing** of `FUN_1403b5130`, the `ITEMINFO` loader - the WZ-property-name to struct-offset map every [L] in sections 2, 5 and 6 leans on |
| `research/msexe-tooltip-assembler.txt` | the listing of `FUN_14264f750`, which calls both halves of the tooltip |
| `research/msexe-statline.c` / `.txt` | `FUN_142699710`, the stat-line assembler - **the function section 8 turns on**, decompiled and as a listing; plus `FUN_1403e8d40` and its Rebirth Flame caller |
| `research/msexe-itemvstemplate.txt` | the listing of `FUN_14038d3c0`, which compares every packet stat field against its WZ template value in the same units |
| `research/msexe-upgradecount.c` | the four functions that read `item+0xfa`, found by the pointer/checksum co-occurrence scan |

---

## 10. The v214 reference tree, scored against this pass - **[I] throughout**

`C:\Users\user\Desktop\ModernMapleSource` is a different game version and scored **1 of 8**
against a held-out control. Nothing below is evidence. It is recorded because this pass
produced a rare thing: an **independently derived table to score it against**, and the score
is informative in both directions.

The relevant file is `v214 src/src/main/java/net/swordie/ms/enums/EquipBaseStat.java`, whose
entries carry `(bitValue, maskIndex)` - `maskIndex 0` is our mask 1 and `maskIndex 1` is our
mask 2.

### 10.1 Mask 1 - it **disagrees**, and that is the point

| v214 bit | v214 name | what this binary has at that bit **[L]** |
|---|---|---|
| 6 | `iPAD` | **Speed** |
| 7 | `iMAD` | **Jump** |
| 8 | `iPDD` | Attack Power (PAD) |
| 10 | `iCraft` | Weapon Def. (PDD) |
| 11 | `iSpeed` | Magic Def. (MDD) |
| 12 | `iJump` | Accuracy |
| - | `iACC`, `iEVA` marked *removed* | present, at bits 12 and 13 |
| - | no equivalent | Critical Rate, Critical Damage, Weapon Attack at 14-16 |

v214 has **13** live bits; this client has **17**. Bits 0-5 (STR/DEX/INT/LUK/MaxHP/MaxMP)
are the only ones that line up.

**This is the abstract warning in `CLAUDE.md` made concrete.** Section 2 already said that
building mask 1 from the family expectation would have put `incPAD` at bit 6; here is the
reference tree doing exactly that. Anyone who had taken this file as a spec would have sent
Speed where the client reads Attack Power, and - because there is no length prefix - would
also have shipped a 13-bit mask into a 17-bit reader and desynchronised the record.

### 10.2 Mask 2 - it **agrees on every bit that was established independently**

Nine bits carry [L] evidence in section 3. v214 names all nine, at the same positions:

| bit | this binary **[L]** | v214 name |
|---|---|---|
| 0 | `Remaining Enhancements: %d` | `tuc` |
| 2 | the attribute bitfield | `attribute` |
| 8 | `Required Level: -%d` | `iReduceReq` |
| 14 | `Boss Damage +%d%%` | `bdr` |
| 15 | `Ignored Enemy DEF : +%d%%` | `imdr` |
| 16 | `Damage: +%d%%` | `damR` |
| 17 | `All Stats: +%d%%` | `statR` |
| 18 | `Scissors Usages Available : %d` | `cuttable` |
| - | 21 fields total | 21 entries, `0x1` .. `0x100000` |

Bit 7 is the near-miss: v214 calls it `iuc`, this binary uses it for the
`Golden Hammer reforging applied` line - which is what an "item upgrade count" would gate.
Consistent, not identical.

**Four of those names appear in this binary too**, as WZ property names in the `ITEMINFO`
loader, in the same order: `bdR -> +0x108`, `imdR -> +0x109`, `damR -> +0x10a`,
`statR -> +0x10b`, and `reduceReq -> +0x117` beside `incReq -> +0x118`. So bits 14-17 now
have their labels from **two** in-binary sources - the message-table string and the WZ
property name - and the reference merely agrees. **[D]**

### 10.3 Candidate names for the twelve unestablished bits - **[I], do not build on these**

| bit | width **[L]** | v214 candidate |
|---|---|---|
| 1 | u8 | `cuc` |
| 3 | u8 | `levelUpType` |
| 4 | u8 | `level` |
| 5 | u64 | `exp` |
| 6 | u32 | `durability` |
| 7 | u32 | `iuc` (see above) |
| 9 | u16 | `specialAttribute` |
| 10 | u32 | `durabilityMax` |
| 11 | u8 | `iIncReq` |
| 12 | u8 | `growthEnchant` |
| 13 | u8 | `psEnchant` |
| 19 | u64 | `exGradeOption` |
| 20 | u32 | `hyperUpgrade` |

The widths are ours and are [L]; only the names are [I]. `exp` at a `u64` and `level` at a
`u8` are the kind of coincidence that makes a candidate feel settled - it is not. Given 10.1,
a mask-2 agreement of 9/9 and a mask-1 agreement of 6/17 is the honest summary.

### 10.4 It corroborates `0xFF`, which is the one place it changes a decision

`v214 src/.../loaders/ItemData.java:56` is `equip.setCuttable((short) -1);` - the default a
freshly created equip gets from the template, i.e. **`0xFF` as a byte**, exactly the sentinel
section 5.1 derived from `FUN_1403e8d40`. `Equip.java:1719` encodes it as a **byte** when the
bit is set, matching our [L] width, and `ItemData.java:1242` carries the comment *"Equipment
items that do not have a Scissor count will have `cuttable` Scissor uses added to them."*

This does **not** move `NO_SCISSOR_RESTRICTION` from [D] to [L] - a 1-of-8 source cannot do
that - but it is a second, independent source agreeing with a value derived from this
binary, and it agrees on the width and the meaning as well as the number.
