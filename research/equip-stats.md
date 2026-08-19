# What the equip item's four bitmasks actually carry

**Written 2026-08-19, entirely statically. No client run was spent.**

> **Read section 11 first if you are here because the tooltip still shows no stats.**
> The run of 2026-08-19 sent the absolute values section 8 concluded, the items decoded, the
> character is dressed - and there is still no stat line. Section 11 settles the encoding
> question ([L], both directions, and the answer is "the server has nothing to encode"),
> corrects nothing in section 2, and moves the search downstream. **Section 11.4.1 carries
> the run-1 tooltip transcript**, which proves the pipeline was intact and section 8's guard
> reading correct - and makes run 2, not run 1, the thing that needs explaining. Section 12
> is the two-line lesson this file has now paid for twice.

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
| Do the stat fields carry the item's stats? | **Yes - they are the item's TOTAL stats, and a fresh Undershirt must be sent with `inc_pdd = 6`.** The tooltip prints the packet value alone and *subtracts* `ITEMINFO.incPDD` from it; a separate function compares the two directly in the same units. Zeros are suppressed by a print guard, which is why the run-1 screenshot has no stat section. Section 8, and the run-1 transcript in 11.4.1 tests it: template 6, packet 0, **no line** | [L] |
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
   `base + 8k`**, value at `+0`..`+4`, integrity dword at `+4`. **[L]** The value on the
   **wire** is a plain `u16`; the client invents the key and computes the checksum itself
   inside `FUN_1402f7010`. Section 11.1. **[L]**
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

> **This paragraph was retracted on 2026-08-19 and the retraction is WITHDRAWN.** Section
> 11.4.1 argued that "printed" was a prediction written in the past tense, because the only
> account of that screenshot then on file was the commit message of `7a59910`. The
> **screenshot itself** was posted by the owner and transcribed by the coordinator, and it does
> carry the line. The transcript is in section 11.4.1; the paragraph above stands as
> written. What the commit message summarised, the image recorded.

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

> **This sentence was retracted on 2026-08-19 and the retraction is WITHDRAWN.** The
> screenshot has `Remaining Enhancements: 0` in it - transcript in section 11.4.1. The line
> did appear, and because it sits **behind** the `ITEMINFO` gate at `0x1426b223e`
> (section 11.4), its appearance proves that gate was **open** in that run. That turned out
> to be the single most useful fact in this document.

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

---

## 11. Where the `u16` goes, what the tooltip reads, and why the two agreeing is not enough

**Written 2026-08-19, second pass, entirely statically. No client run was spent.** Reading
order: 11.1 answers the question that was asked, 11.4 answers the question that matters.

### 11.0 The three answers, first

| question | answer | label |
|---|---|---|
| Does the `u16` we send reach `item + 0x62 + 8k`? | **Yes.** Section 2's `base + 8k` is correct and the base really is `item + 0x62`. | [L] |
| Do the write path and the tooltip read path use the same encoding? | **Yes, and there was never a way for them not to.** `FUN_1402f7010` (write) and `FUN_1401ab420` (read) are an exact inverse pair, and **the wire carries a raw `u16`** - the client generates the key, encodes and checksums *inside the same function that reads the packet*. There is nothing for the server to encode. | [L] |
| So why is there no stat line **in run 2**? | **The tooltip's object carries none of our optional fields.** The run-2 transcript (11.4.5) has `Remaining Enhancements: 0` where we sent 7 and `Scissors Usages Available : 0` where we sent 0xFF, beside the missing stat line - three fields, two widths, one object, all zero, with the `ITEMINFO` gate demonstrably **open**. | [D] |
| So where does that object come from? | **Not established, and it is the only question left.** Not the equipped array (it stores the decoded object by refcounted pointer, 11.4.5 step 1), not `FUN_14030b560` (dead unless itemId is 1660000..1669999, step 2), not the bag lists (empty, and empty yields no item at all, step 3), not `FUN_1403d2200` (it fills stats from `ITEMINFO`, so it would have printed `+4`). The remaining fork is **a second object** vs **our object holding zeros**, and pointer identity separates them: section 11.6 | **not established** |

### 11.1 The obfuscated-slot hypothesis, disproved

The suspicion was that `base + 8k` with a dword beside it is a ZtlSecure slot, and that the
server might have to send an *encoded* value plus a checksum. It does not, and the reason is
structural rather than arithmetic.

**The writer.** `research/msexe-itemslot-800.txt`, bounded `0x140303800 .. 0x140303a6c` by
`.pdata`. Bit 0 is the shape all seventeen share:

```text
14030381d  CALL 0x1406e8c20        ; u32 mask -> EBP
140303826  TEST AL,0x1
140303828  JZ   0x140303834
14030382a  CALL 0x1406e8b80        ; u16 from the PACKET
140303834  MOV  EAX,EDI            ; bit clear -> 0
140303836  MOV  RDX,RBX            ; RDX = base + 0    (bit k: LEA RDX,[RBX + 8k])
140303839  MOVZX ECX,AX            ; RCX = the raw value
14030383c  CALL 0x1402f7010
140303841  MOV  dword ptr [RBX + 0x4],EAX     ; the checksum it returned
```

So the call is `FUN_1402f7010(raw_u16, dst)` and it returns the checksum. **[L]**

**What that function does** (`research/msexe-tooltip.c`, `FUN_1402f7010 @ 1402f7010`,
131 bytes) - two iterations, `i = 0, 1`:

```text
k_i = FUN_1407386b0(&DAT_143ac1ab0)       ; the client's own byte PRNG
dst[i]     = k_i
dst[2 + i] = k_i ^ value_byte_i
chk        = ror32(chk ^ k_i, 5) + (k_i ^ value_byte_i)      ; chk starts 0xbaadf00d
```

**The reader** (`research/msexe-tooltip-stats.c`, `FUN_1401ab420 @ 1401ab420`):

```text
return  CONCAT11(p[3] ^ p[1], p[2] ^ p[0])                   ; lo = p[2]^p[0], hi = p[3]^p[1]
chk  =  ror32((ror32(p[0] ^ 0xbaadf00d, 5) + p[2]) ^ p[1], 5) + p[3]
        if chk != argument -> FUN_141804970 -> _CxxThrowException("throw ZException")
```

Substitute the writer's `p[0]=k0, p[1]=k1, p[2]=k0^v0, p[3]=k1^v1` and the reader's
recurrence is the writer's, term for term. **Exact inverse pair.** **[D]** from two **[L]**.

**And the key is random, generated at decode time.** `FUN_1407386b0(&DAT_143ac1ab0)` is
called *inside* `FUN_1402f7010`, which is called *by the packet reader*, with the value it
just read off the wire in `RCX`. There is exactly one writer of these slots on this path and
it is the packet decoder. A server cannot send an encoded value even in principle - it does
not know the key, and the client would overwrite it with its own. **The wire is a raw
little-endian `u16` and nothing else.** **[L]**

Three further things checked so the same suspicion is not raised again:

* **`FUN_1402f70a0` is not a second encoding.** It is byte-identical to `FUN_1402f7010` -
  same 131 bytes, same decompilation - an un-folded duplicate. `FUN_140304100` uses it for
  the seven `u16` at `item+0x3bf..0x3ef` and `FUN_1402f7010` for the three at
  `item+0x3f7..0x407`; both produce the layout `FUN_1401ab420` reads. **[L]**
* **The `u8` variant is inlined, not a call, and it matches `FUN_1401b0050`.** In
  `FUN_140303b40` at `0x140303b81`: `key -> [base+0x98]`, `key^value -> [base+0x99]`,
  `ror32(key ^ 0xbaadf00d, 5) + (key^value) -> [base+0x9c]`. `FUN_1401b0050` returns
  `p[1]^p[0]` and recomputes exactly that. **[L]** This also confirms section 3's
  `item + 0xfa + 8k`: the mask-2 base is `base + 0x98` = `item + 0x62 + 0x98`.
* **The `0x9a65` scheme is a different mechanism and does not touch these slots.** It is the
  rolling re-key of the *secure* fields - `MOV EAX,0x9a65` at `0x14030364f`, inside
  `FUN_1403035a0`, operating on the buffer at `item+0x28` with a counter at `item+0x20` that
  re-keys every `0x6f` accesses. That is what holds the **itemId** (`FUN_1401b0340` /
  `FUN_14019a5d0` read it), and what `charstat-layout.md` describes for the map id. The stat
  slots use the `0xbaadf00d` scheme above. Two schemes, not one. **[L]**

### 11.2 Section 2's spacing is right, and so is its base

Chain, all off listings, each bounded by `.pdata`:

| step | listing | evidence |
|---|---|---|
| `FUN_140304100` calls `FUN_140303b40(item + 0x62, packet)` | `msexe-itemslot-equip-decode.txt` | `0x14030411f LEA RCX,[RSI+0x62]` / `0x140304126 CALL 0x140303b40` |
| `FUN_140303b40` passes its `RCX` straight through | `msexe-itemslot-b40.txt` | `0x140303b5b MOV RBX,RCX` / `0x140303b5e CALL 0x140303800` - `RCX` untouched |
| bit *k* lands at `base + 8k`, checksum at `+4` | `msexe-itemslot-800.txt` | `MOV RDX,RBX`, `LEA RDX,[RBX+8]`, `[RBX+0x10]` ... `LEA RDX,[RBX+0x80]` for bit 16; stores at `[RBX+0x4]`, `[RBX+0xc]` ... `[RBX+0x84]` |
| the tooltip reads those same addresses | `msexe-tooltip-stats.txt` | `LEA RCX,[RSI+0xe2] / MOV EDX,[RSI+0xe6]` for bit 16 = `0x62 + 8*16`; nine offsets, all landing |

**And the two `RSI`s are the same object**, by a cross-check that does not go through any of
the stat offsets: the decoder reads the itemId at `this + 0x20`
(`0x14030432a LEA RCX,[RSI+0x20] / CALL 0x14019a5d0`) and the tooltip reads the itemId at
`param_2 + 0x20` (`0x1426b2128 LEA RCX,[RDX+0x20] / CALL 0x1401b0340`). Same offset, same
secure-slot accessor family. **[L]**

Nothing in section 2 needs correcting.

### 11.3 The bytes the owner's client actually received

Decoded out of `world.log` (the `0x01A0` body of 2026-08-19 19:48:15, 1189 bytes) by
replaying `FUN_140304100`'s read order field by field. The equipped block starts at body
offset 268 with `flagA = 0`, and the four items are **129 bytes each**, back to back with
their `u16` slots, followed by the five `u16` terminators. **[L]**

| slot | itemId | mask 1 | stats | mask 2 | options |
|---|---|---|---|---|---|
| 5 | 1040003 | `0x00000400` | bit 10 = **6** | `0x00040001` | bit 0 = 7, bit 18 = 0xFF |
| 6 | 1060002 | `0x00000400` | bit 10 = **4** | `0x00040001` | bit 0 = 7, bit 18 = 0xFF |
| 7 | 1072003 | `0x00000400` | bit 10 = **2** | `0x00040001` | bit 0 = 5, bit 18 = 0xFF |
| 11 | 1302000 | `0x00010000` | bit 16 = **17** | `0x00040001` | bit 0 = 7, bit 18 = 0xFF |

The parse consumes each item to exactly 129 bytes and lands on the next slot `u16` four
times running, which is the discriminator: a width error would not close.

**So the wire is right, the encoding is the client's own, and the value is at
`item + 0xb2` (or `+0xe2`) when the tooltip reads it.** Everything from here is about what
happens *after* that.

### 11.4 One gate hides everything the packet could have shown - and in run 1 it was open

New listing this pass: `research/msexe-equiptooltip.txt`, the whole of `FUN_1426b20f0`,
`0x1426b20f0 .. 0x1426b3e1a`, bounded by `.pdata`. The decompiler and the listing agree
instruction for instruction on the part that matters.

```text
1426b2128  LEA  RCX,[RDX + 0x20]        ; RDX = the item
1426b212c  CALL 0x1401b0340             ; -> itemId
1426b2136  CALL 0x140388c60             ; ItemInfoMgr::GetItemInfo(itemId)
1426b213b  MOV  R15,RAX
   ...     (an alternate lookup when param_3 != 0, which also lands in R15)
1426b223b  TEST R15,R15
1426b223e  JZ   0x1426b3d8e             ; ITEMINFO == 0  ->  the function's epilogue
```

**Everything the packet can influence is behind that `JZ`:**

| what | where | inside the gate? |
|---|---|---|
| `FUN_140396250` - fills the baseline struct | `1426b22a8` | yes |
| `FUN_1426aed50`, `FUN_1426af680`, `FUN_1426af8c0`, `FUN_1426afdc0` - all four stat helpers | `1426b22b6`, `22cb`, `22e0`, `22f5` | yes |
| Speed (0x038A) and Jump (0x038B), called directly | `1426b2367`, `1426b23e9` | yes |
| `Remaining Enhancements` - `MOV EDX,0x39e` | `1426b3a72` | yes |
| `Scissors Usages Available` - `MOV EDX,0x3a0` | `1426b3cb3` | yes |
| **`Cannot be Traded when equipped`** - `FUN_1426e10e0` | `0x14264f8ae`, in **`FUN_14264f750`**, 3741 bytes earlier | **no** |

The last row is the one that matters. `FUN_14264f750` calls the restriction-line builder and
the stat-line builder with the **same item** (`MOV RDX,R12` at both `0x14264f8a4` and
`0x142650745`) but into **different sinks** (`LEA RCX,[RBP+0x870]` vs `MOV RCX,R15`), 3741
bytes apart - `0x14265074b - 0x14264f8ae = 0xe9d`. So *"the trade line prints and nothing
else does"* is a single-fault state, not a coincidence. **[L]**

And `FUN_140388c60` really can return 0: after the hash-bucket miss it calls
`FUN_1403e18a0(itemId)` and, if that yields a null or empty wide string, returns **0**
without ever calling the `ITEMINFO` loader `FUN_1403b5130`.
**[L]**, `research/msexe-iteminfo-lookup.c` / `.txt`.

**There is exactly one stat-line path in this client**, so this is not a case of looking at
the wrong renderer: `python tools/callers.py 0x142699710` gives **17 call sites in 4
functions** - `FUN_1426af680` (4), `FUN_1426af8c0` (2), `FUN_1426afdc0` (9), `FUN_1426b20f0`
(2) - and `FUN_1426b20f0` itself has exactly **one** caller. **[L]**

#### 11.4.1 The run-1 screenshot, and the retraction that is WITHDRAWN

An earlier version of this section retracted section 5's *"`Scissors Usages Available : 0`
printed"* and section 7's *"why the line appeared at all"* as predictions written in the past
tense, on the grounds that the only account of that screenshot on file was the commit message
of `7a59910`. **That retraction is withdrawn.** The owner posted the image and the coordinator
transcribed it directly:

```text
Undershirt (M)
Cannot be Traded when equipped
[icon]
REQ LEV : 0    REQ STR : 0    REQ DEX : 0
REQ INT : 0    REQ LUK : 0    REQ FAM : 0
BEGINNER WARRIOR MAGICIAN BOWMAN THIEF
Type: Top
Remaining Enhancements: 0
Scissors Usages Available : 0
```

Sections 5 and 7 are restored. The commit message summarised the complaint; the image is the
record.

**This is the most informative thing in the document, and it points the other way.** Read it
against the gate in 11.4:

| observation, run 1 (everything we sent was zero) | what it proves | label |
|---|---|---|
| `Remaining Enhancements: 0` is on screen - emitted at `0x1426b3a72`, **behind** the gate | `FUN_140388c60` returned non-null: **the gate was OPEN** | [D] |
| `Scissors Usages Available : 0` is on screen - `0x1426b3cb3`, also behind it | the same, independently | [D] |
| both lines rendered at all | `FUN_1426aed50`, `FUN_1426af680`, `FUN_1426af8c0` and `FUN_1426afdc0` all ran to completion **without throwing** - they are called at `0x1426b22b6..0x1426b22f5`, before both | [D] |
| both lines rendered at all | `FUN_14269a1d0` was **not** dropping lines: its silent discard needs `*(int *)(tooltip + 0xa0) == 0x22`, and these two go through the same appender with the same `param_1` | [D] |
| **no** `Weapon Def.` line, while `ITEMINFO.incPDD` for this item is **6** | `FUN_142699710`'s guard is **not** on the template value. Section 8's reading is **corroborated by a run**, not undermined: arg 3 = 6 with arg 5 = 0 printed nothing | [D] |

So run 1 is completely coherent: **gate open, whole pipeline intact, guard on the packet
field, our zeros suppressed every stat line.** Nothing about it is anomalous, and section 8's
model predicts it exactly.

**Run 2 is the anomaly.** Same items, same itemIds, `incPDD = 6 / 4 / 2` and `incWAT = 17`
on the wire (11.3), the record decoded 26 times with no fault, the character is dressed - and
the report is still "no stat lines, not even a weapon-attack line on the sword".

#### 11.4.2 The three candidates, weighed against run 1

**(1) Did something we started sending in run 2 close the gate?** **No mechanism exists.**
The gate's only input is `FUN_1401b0340(item + 0x20)` - the itemId - fed to
`FUN_140388c60`. Run 2 changed three things and none of them is `item + 0x20`: mask-1 stat
values, mask-2 bit 0 (`tuc`), mask-2 bit 18 (`scissorUses`). The itemId bytes are identical
and at the same wire offset, ahead of every mask. Nor can a helper have started throwing:
every obfuscated slot is written by the client with a checksum the client computes, so no
checksum can mismatch, and `FUN_140396250` - the only pre-helper function that reads optional
fields - reads mask-2 bit 11 (`item+0x16e`) and bit 19 (`item+0x1ae`), both still zero.
**And it is falsifiable from the run-2 screenshot**: if the gate had closed,
`Remaining Enhancements` and `Scissors Usages Available` would both have vanished too.

*One honest loose end under (1).* We now send `remaining_enhancements = tuc` exactly, and
`FUN_14038d3c0` tests that field with `CMP AL, byte ptr [RDI+0xb8] / JNC -> return 1` at
`0x14038d41c` - so `== tuc` lands on the "not fresh" side of the boundary section 7 flagged.
`python tools/callers.py 0x14038d3c0` reports **0 call sites**, but *that tool sees only
`call rel32`*: it cannot see a vtable slot or an indirect call, and this binary reaches a lot
of code that way. The positive control from the same session is `FUN_1402f7010` -> **499**
call sites, so the tool speaks; the zero is simply not evidence of absence. The cheap variant
for a future run is to send `tuc - 1`. **Not** indicated by anything measured.

**(2) Is the tooltip reading a copy?** **Very unlikely, and run 1 is what rules it out.**

A copy that *loses* the stat values cannot be a byte copy: the slots are opaque
`{key, key^value, checksum}` bytes, so `memcpy` or a compiler-generated `operator=` carries
key, value and checksum across together and the value survives intact. A copy can only lose
them by being a **reconstruction** - a fresh object plus field-by-field assignment.

This client has exactly one such reconstruction, and it argues against (2):
**`FUN_1403d2200(mgr, out, itemId, ...)`** builds a `GW_ItemSlotEquip` from an itemId, and
it fills the stat block **from `ITEMINFO`** (`research/msexe-itemclone.c`):

```c
FUN_1402f7010(*(u16 *)(ITEMINFO + 0xba), item + 0x62);   // incSTR  -> bit 0
FUN_1402f7010(*(u16 *)(ITEMINFO + 0xbc), item + 0x6a);   // incDEX  -> bit 1
...
FUN_1402f7010(*(u16 *)(ITEMINFO + 0xca), item + 0xe2);   // incWAT  -> bit 16
FUN_1402f7010(*(u16 *)(ITEMINFO + 0xcc), item + 0xa2);   // incPAD  -> bit 8
FUN_1402f7010(*(u16 *)(ITEMINFO + 0xd0), item + 0xb2);   // incPDD  -> bit 10
```

`python tools/callers.py 0x1403d2200` gives **98 call sites in 88 functions** - it is the
general "make me an item for this id" utility the whole UI uses for shop rows, quest-reward
previews and the like. **[L]**

So the client's own idea of a display item built from an id carries the **template's** stats.
Run 1's tooltip showed **no** `Weapon Def.` line on an item whose template is 6. A
`FUN_1403d2200` item would have shown `Weapon Def.: +6` in run 1. **It did not, so the
tooltip is not rendering a from-template copy.** **[D]** The only copy shape left is a
bare-constructed item with nothing filled in at all, which is a strange thing for a tooltip
to render and which run 2's screenshot tests directly (see 11.4.3).

*A bonus worth recording:* `FUN_1403d2200` is a **fourth, fully independent instrument**
confirming section 2's whole bit -> `ITEMINFO` map, including the two that matter here -
bit 16 <- `+0xca` (`incWAT`) and bit 8 <- `+0xcc` (`incPAD`). It was found by enumerating the
callers of the type-1 constructor `FUN_1402f7da0` (**5 sites in 4 functions**), not by
searching for it.

**(3) Is the guard not what section 8 read?** **Least likely of the three, and run 1 is why.**
The guard reading now has three legs: the listing at `0x142699745`, the decompilation of
`FUN_1426af680` (arg 5 is the `item + 0x62 + 8k` read, arg 3 the `ITEMINFO` one), and
frame arithmetic done twice from scratch (`LEA RBP,[RSP-0x1f]` after five pushes puts
`[RBP+0x6f]` at `entry + 0x28`, which is arg 5). Run 1 then **tested** it: a template value of
6 with a packet value of 0 printed nothing, which is exactly and only what "the guard is on
arg 5" predicts. A wrong reading of the guard would have had to produce a line there.

#### 11.4.3 ANSWERED - the run-2 transcript arrived

This section asked for a transcription of the run-2 tooltip instead of a paraphrase, because
it was the only unmeasured link in the chain. It arrived, and it is section 11.4.5. The
prediction table below stood: the outcome was **row 2, all three lines unchanged from run 1**.

Every link in run 2's chain has a listing or a capture behind it **except one**. The wire is
measured (11.3). The decode is structural (11.1, 11.2). The gate was open in run 1 and
nothing we changed feeds it. The guard is corroborated by run 1. What is **not** measured is
the run-2 observation itself: *"Equipment stats are still not in, I do not see any correct
stats for any equipment, the sword does not even have a weapon attack line"* is a paraphrase,
and this document has now been wrong twice about a screen state that nobody transcribed.

**So: transcribe the run-2 tooltip box the way run 1's was transcribed.** No launch is
needed if the screenshot exists. Three lines in it are already-sent, independent
discriminators, and they separate (2) from (3) outright:

| line in the run-2 box | reads | what it settles |
|---|---|---|
| `Remaining Enhancements:` | **7** | the packet's optional fields reach the tooltip's object -> **(2) is dead**, and a mask-1 value must be in there too |
| | **0** | the object the tooltip reads does not carry our fields -> **(2)**, a bare-constructed item |
| `Scissors Usages Available :` | **absent** | `scissorUses = 0xFF` arrived, so `vtable+0x200` is false -> same conclusion as `7` |
| | still `: 0` | same conclusion as `0` |
| `Cannot be Traded when equipped` | **absent** | decided by `FUN_14038cf10` on **`R12`** - the *identical pointer* `FUN_14264f750` passes to `FUN_1426b20f0` at `0x142650745`. Not a correlation: the same object |
| any `Weapon Def.:` or `Weapon Attack:` line | **present** | there is no bug to find; the paraphrase was about the values being wrong, not about the lines being absent |

If those three lines are unchanged from run 1, the answer is **(2)** and the next static step
is 11.4.4. If they changed, the answer is **(3)-adjacent** - the object holds our data, the
guard passed, and the line was built and lost between `FUN_142699710`'s `sprintf` at
`0x142699771` and `FUN_14269a1d0`.

#### 11.4.4 ANSWERED - the three static steps, and what each returned

All three were taken; the results are in 11.4.5. Steps 1 and 2 came off listings already on
disk, step 3 off `research/msexe-equiplists.c`. The list below is kept because it is the
reasoning that chose them.

1. **The per-slot cell.** `0x140306229` - `LEA EAX,[RCX-1] / CMP EAX,0x1e / JA` - stores the
   decoded item at `record + 0x1a8 + slot*0x10` for `1 <= slot <= 31`
   (`naked-character.md`, and the `EQUIP_SLOTS` doc in `crates/net`). **16 bytes per slot is
   wider than a pointer.** Dump that region and read what the cell holds: a `{ptr, refcount}`
   or shared-pointer pair means one shared object and **(2) is dead outright**; a value copy
   of anything is the copy, named.
2. **`FUN_1402fbb30` = `return this + 0x242`** is the item's `vtable+0x330`, and it is what
   `FUN_14030b560` calls to insert the item into its map. Read what the map stores - the
   object or an interior copy.
3. **Do not try to enumerate the writers of the stat slots.**
   `python tools/callers.py 0x1402f7010` is **499 call sites in 121 functions**; that is not
   a list anyone will read correctly, and filtering it by shape is the mistake `CLAUDE.md`
   names. The tractable enumeration is the one already done - the **constructor**
   `FUN_1402f7da0` has **5 call sites in 4 functions** (`0x14030ddb0` the type-1 allocator,
   plus `0x1402cc180`, `0x1402d4100`, `0x1403d2200`), because a reconstruction has to
   allocate. Only `FUN_1403d2200` fills the stat block, and 11.4.2 rules it out.


#### 11.4.5 The run-2 transcript, and the three no-run steps it bought

The owner posted the run-2 screenshot; the coordinator transcribed it. The item is
**Brown Cotton Shorts (M)**, itemId **1060002**, equip slot 6 - sent with `incPDD = 4`
(mask-1 bit 10), `remaining_enhancements = 7` (mask-2 bit 0) and `scissor_uses = 0xFF`
(mask-2 bit 18).

```text
Brown Cotton Shorts (M)
Cannot be Traded when equipped
[icon]
REQ LEV : 0    REQ STR : 0    REQ DEX : 0
REQ INT : 0    REQ LUK : 0    REQ FAM : 0
BEGINNER WARRIOR MAGICIAN BOWMAN THIEF
Type: Bottom
Remaining Enhancements: 0
Scissors Usages Available : 0
```

**Three packet fields, two widths, one object, all zero.** [D]

| line | reads | the field it reads | so |
|---|---|---|---|
| `Remaining Enhancements: 0` | 0, sent 7 | `FUN_1401b0050(item + 0xfa, [item + 0xfe])`, a `u8` slot | `item + 0xfa` is 0, **with a valid checksum** - it printed instead of throwing |
| `Scissors Usages Available : 0` | 0, sent 0xFF | `FUN_1401b0050(item + 0x1a6, [item + 0x1aa])`, a `u8` slot | `item + 0x1a6` is 0. And the line printing **at all** means `FUN_1403e8c40` -> `vtable+0x200` found `<= 0x14`, which 0xFF is not |
| no `Weapon Def.` line | -, sent 4 | `FUN_1401ab420(item + 0xb2, [item + 0xb6])`, a `u16` slot | `item + 0xb2` is 0, or `FUN_142699710`'s guard would have printed `+4` |
| both option lines printing | - | both are emitted behind the `ITEMINFO` gate | **the gate is open**, and the whole pipeline still runs |

`Cannot be Traded when equipped` is *consistent* with `scissorUses = 0` but is **not**
independent evidence, and section 5's derivation was for 1040002: `FUN_14038cf10` returns 1
at `0x14038cf47` - before it ever consults `vtable+0x200` - if `FUN_14038ce90`
(`exchangeableOnce || equipTradeBlock`) is set, and nobody has checked those two WZ
properties for **1060002**. The two `: 0` lines are the evidence; the trade line is a
bystander here.

**So the tooltip's object carries none of our optional fields while its identity fields -
name, icon, `Type: Bottom` - are right.** Those identity fields come from `ITEMINFO`, reached
through `FUN_1401b0340(param_2 + 0x20)`, so all they prove is that `param_2 + 0x20` holds
1060002. An object of the same class with only the itemId set fits every line above.

##### Step 1: `record + 0x1a8 + slot*0x10` holds **the decoded object itself**. No copy

Off the listing already on disk, `research/msexe-charrecord-full.txt`,
`0x1403061c9 .. 0x1403062ff`:

```text
1403061fc  CALL 0x1406e8b80              ; u16 slot, 0 ends the list
14030621e  CALL 0x1403095e0              ; the factory decodes into [RBP+0x30];
                                         ;   the item pointer lands at [RBP+0x38]
140306229  LEA  EAX,[RCX + -0x1]
14030622c  CMP  EAX,0x1e
14030622f  JA   0x1403062bc              ; slot outside 1..31 -> released, discarded
140306238  SHL  RAX,0x4                  ; slot * 0x10
14030623c  LEA  RDI,[R15 + 0x1a8]
140306243  ADD  RDI,RAX                  ; RDI = the cell
14030627b  MOV  RBX,qword ptr [RBP + 0x38]   ; the decoded object
14030629b  INC.LOCK qword ptr [RBX + 0x8]    ; refcount++
1403062a4  MOV  RCX,RDI / CALL 0x1401abd80   ; release the previous occupant
1403062ac  MOV  qword ptr [RDI + 0x8],RBX    ; store the POINTER at cell+8
```

The 16-byte cell is a refcounted handle - `{?, ptr}` with the pointer at `+8` and an
interlocked refcount at `object + 8`. **The array stores the very object `FUN_1403095e0`
produced, by pointer.** There is no value copy here and (2)-via-the-equipped-array is dead.
**[L]**

##### Step 2: `FUN_14030b560` never runs for our items

`equip-block.md` called the `vtable+0x330` call inside `FUN_14030b560` "the blocker". It is
not on our path at all - the whole body is behind one range test:

```c
iVar4 = FUN_14019a5d0(item + 0x20);          // the itemId
if (iVar4 - 0x195460U < 10000) { ... }       // 0x195460 = 1660000
```

So `FUN_14030b560` does nothing unless the itemId is in **1660000..1669999**. Ours are
1040003, 1060002, 1072003 and 1302000. Whatever that map is, it is not where our tooltip's
item comes from. **[L]**, `research/msexe-equiplists.c`. (Same family as the
`FUN_1402cb4f0` post-pass at `0x14030435e`, which `equipped_item`'s `debug_assert` already
guards.)

##### Step 3: the four extra lists are the **bag**, and empty is correct

`presence[2]` opens `FUN_14030b6f0(ctx, 1)` and `FUN_14030b9e0` (run for indices 2, 3 and 4).
Both resolve a per-inventory-type key through `FUN_1403023d0(out, N)`, read a `u16`, and run
the **same** item factory `FUN_1403095e0` in a loop until the `u16` is zero - the identical
shape to the equipped list, one per inventory type. These are **inventory types 1..4: the
bag**. **[L]**, `research/msexe-equiplists.c`.

An empty bag is the correct thing to send for a character carrying nothing, and - this is the
point - an empty list produces **no item at all**. It cannot produce a window entry with a
correct name, icon and `Type: Bottom`. So "the Equipment window renders from one of the lists
we send empty" does not fit the transcript and is **ruled out**. **[D]**

##### And the last unverified link in the wire model is now [L]

`FUN_1403035a0`, the base decode shared by all three item types, was the one step this pass
had taken from a source comment rather than a listing. Enumerating **every** call target in
`research/msexe-itemslot-base.txt` (`0x1403035a0 .. 0x1403037f5`) gives 2x `0x1406e8c20`,
2x `0x1406e8ae0`, 2x `0x1406e9170` and three non-readers - so, in order:

| read | at | into |
|---|---|---|
| `u32` itemId | `0x1403035c5` | the secure slot at `item+0x20` |
| `u8` hasCashSN | `0x140303787` | - |
| `raw[8]` cash serial, **only if non-zero** | `0x14030379d` | `item+0x38` |
| `raw[8]` dateExpire | `0x1403037b9` | `item+0x40` |
| `u32` | `0x1403037c1` | `item+0x48` |
| `u8` | `0x1403037cc` | `item+0x4c` |

Exactly what `crates/net::equipped_item` sends, in that order. **[L]** So the mask `u32`s are
read at the wire offsets we write them to.

#### 11.4.6 What is left: two objects, or one object with zeros

Everything above eliminates a copy *store*. It does not eliminate a copy. Two possibilities
remain, and they are separated by **pointer identity**, not by any further reading:

**(2a) A different object.** The tooltip is handed a distinct `GW_ItemSlotEquip` carrying the
right itemId and nothing else. It is **not** from the equipped array (step 1), **not** from
`FUN_14030b560` (step 2), **not** from the bag (step 3), and **not** from `FUN_1403d2200` -
that one fills the stat block from `ITEMINFO`, so a `FUN_1403d2200` item for 1060002 would
have printed `Weapon Def.: +4`, and the transcript has no line at all. Where it does come from
is not established, and the three forwarders into `FUN_14264f750` (`FUN_142656500`,
`FUN_142664790`, `FUN_142687b40`) have **33 call sites between them** - too many to walk
blind, which is what 11.6 now solves with a stack trace instead.

**(2b) The same object, holding zeros.** Our decoded item itself never received the values,
because `FUN_140303800` stored zeros - which needs the client to have read the masks as 0.
The argument against it is that the client would then consume 4 fewer bytes per item, the
record has no resync point, and world entry, the avatar, portals and NPCs all work in run 2 -
but that argument is **[D], not [L]**, and it is the cheaper of the two to be wrong about.
It was treated as closed earlier in this pass and it should not have been.

*Ruled out by arithmetic rather than by reading:* "something zeroed the stat block after the
decode" cannot be it. `FUN_1402fb7e0(base)` zeroes exactly 17 slots, `base+0` to `base+0x80`;
applied at `item+0x62` that covers `0x62..0xea` and **cannot** touch `item+0xfa` or
`item+0x1a6`, both of which also read zero. No single zeroing pass explains all three fields.
**[D]**

*One positive control that bounds all of this:* the **avatar is correct**. Whatever the
character's appearance is built from has our items in it, so the decode ran and stored
something usable, and the problem is specific to the tooltip's path.

### 11.5 What the server must write: **nothing new**

* **The value:** the raw little-endian `u16`, which is what `equipped_item` already sends.
* **The checksum:** none. The server never writes one; `FUN_1402f7010` computes it from a
  key the client invents at decode time.
* **The encoding:** none. See 11.1.

No byte of `crates/net::equipped_item` is indicated by this pass, and changing one without
evidence is the mistake this document already made once. `EquipStats::default()` still
produces the 125-byte body; `EquipStats::fresh` still produces 129 for the starter items.

### 11.6 The watch that separates them

**The question has changed.** It was "does the tooltip's object hold our value" - the run-2
transcript answered that: **no**, on three fields at once (11.4.5). It is now "**which object
is it**", and that is settled by comparing pointers, not by peeking at bytes. The probe pair
changes accordingly.

`tools/test-server.ps1` has two free slots (`1415db360:ret` and `141b2a280:rdx=0` are not
negotiable). The probe logs `rcx/rdx/r8/r9`, the first dword behind any pointer register, the
return address, `:peek=<off>` at **`rcx + off`** - and, on every hit, **a stack trace**: up to
16 return-address-shaped values from the first `0x400` bytes of stack, each tagged `<-TEXT`,
`(vm)` or `(?)` (`crates/grap-stub/src/probe.rs`, `stack_trace`). That trace is what makes
this cheap: it names the Equipment-window chain that owns the tooltip's item, which statically
would mean walking **33 call sites** across the three forwarders into `FUN_14264f750`.

It does **not** log stack arguments, which still rules out watching `FUN_142699710` - its
packet value is arg 5 and arrives on the stack.

```text
$Probe = 'watch@1415db360:ret,141b2a280:rdx=0,1426afdc0:hits=60,140304100:hits=200'
```

| watch | why | what to read |
|---|---|---|
| `1426afdc0:hits=60` | the first thing past the `ITEMINFO` gate that only equips reach (`0x1426b22f5`, 183 bytes after the `JZ`) | **`rdx` = the object the stat section reads**, `r8` = `ITEMINFO`, and the **stack trace**, which names its owner |
| `140304100:hits=200` | the type-1 equip decode; already known to fire 4x per `SetField` | **`rcx` = every decoded item pointer**, the set to compare `rdx` against |

**Hover the trousers (slot 6, 1060002)** - the item the run-2 transcript is for - so the
pointer comparison is against a known line of the box.

| outcome | which branch | next step |
|---|---|---|
| **`rdx` is NOT one of the `140304100` `rcx` values** | **(2a)**, a second object - the expected result | Read the `<-TEXT` frames of the stack trace. They name the function that built it, and that is the whole remaining question. Nothing about the packet changes until it is answered |
| **`rdx` IS one of them** | **(2b)**, our own decoded object holds zeros | The bug is in the decode, not the tooltip, and it is much more tractable. Re-arm `1402f7010:hits=200`: `rcx` is the **raw `u16` being stored** and `rdx` the destination, so `rcx=6 rdx=<item+0xb2>` proves the store and its absence names the mask read. Also re-check the item's byte length against `FUN_140304b20`'s consumption |
| **`1426afdc0` never fires on a hover** | the gate closed between run 1 and run 2 | Contradicts 11.4.5 - both option lines printed, and they are behind the same gate. Suspect the watch, not the client: confirm it is armed by checking `140304100` fired in the same log |
| **`r8` is 0** | impossible if the gate is what 11.4 says | the same: verify the instrument first |

The `1402fd610:peek=b2` watch this section used to recommend is retired. Its question - "is 6
in the object" - is answered by `Scissors Usages Available : 0` and `Remaining
Enhancements: 0` in the transcript, for free and on two more fields than a peek would have
covered.

### 11.7 Files from this pass

| file | what |
|---|---|
| `research/msexe-equiptooltip.txt` | the **listing** of `FUN_1426b20f0`, `0x1426b20f0..0x1426b3e1a`. The authority for 11.4 - the one gate, and what is behind it |
| `research/msexe-tooltip-af680.txt`, `research/msexe-tooltip-af8c0.txt` | listings of the bits 0-3 and bits 4-5 helpers |
| `research/msexe-iteminfo-lookup.c` / `.txt` | `FUN_140388c60` (the `ITEMINFO` lookup that can return 0), `FUN_140396250` (the baseline struct), `FUN_1402fd610` (`vtable+0x200`, `RCX` = the item) |
| `research/msexe-itemclone.c` | the three non-factory callers of the type-1 constructor. `FUN_1403d2200` is the client's build-an-item-from-an-id utility and fills the stat block from `ITEMINFO` - the fourth instrument confirming section 2's bit map, and what rules out a from-template copy |
| `research/msexe-equiplists.c` | `FUN_14030b560` (dead outside itemId 1660000..1669999), `FUN_14030b6f0` and `FUN_14030b9e0` (the four **bag** inventory lists), `FUN_14030ca50`, `FUN_1401abd80` |
| `research/msexe-tooltip-callers.c` / `msexe-tooltip-caller-664790.txt` | the three forwarders into `FUN_14264f750`. All three pass the item straight through from their own callers - 33 call sites, which is why 11.6 gets the answer from a stack trace instead |

---

## 12. The tell this file has now paid for twice

Both errors in this document have the **same shape**: *a prediction recorded in the past
tense.*

| # | what was written | what it actually was |
|---|---|---|
| 1 | Section 8.0 - "the tooltip renders `ITEMINFO.incPDD + packet.incPDD`", labelled **[L]** | an inference from *which arguments were passed*. `FUN_142699710` was never opened |
| 2 | Section 11.4.1's first version - "the scissors line was never on screen", used to retract sections 5 and 7 | an inference from *which document mentioned it*. The screenshot was never asked for |

Error 2 is the mirror of error 1 and cost a whole diagnostic detour: it moved the search onto
a gate that a five-line transcript proves was open.

**The tell, both times: a sentence about what the client did, with no capture, screenshot or
listing cited beside it.** Neither error is visible in the prose - both read as confident
statements of fact.

**The rule that catches it.** Every claim about *what appeared on screen* carries its source
inline, the way section 11.4.1 now does: the transcript, the `world.log` timestamp, or the
`maplecw-hook.log` WATCH line. If the source is "a commit message", "an earlier section", or
nothing, the claim is **[I]** no matter how certain it feels - and if it is load-bearing, the
next step is to ask for the artefact, not to reason around it. Asking cost one message here.
