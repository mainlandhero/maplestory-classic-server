# `0x00D1` — the client's transfer-field request, and what answers it

Written 2026-08-19 from the 2026-08-19 playable-on-map-1 capture and a fresh read of the
client image on disk. **No Ghidra was used**: the Ghidra project was held by another
session, so every new fact here comes from raw PE reads (`.pdata` bounds, `E8 rel32`
targets, hand-decoded bytes) cross-checked against the decompilations already in
`research/`. Where that mattered it is said so.

Marks: **[L]** read from a listing/decompilation/capture, **[D]** derived, **[I]** inferred.

---

## 0. Summary

* **`0x00D1` is `UserTransferFieldRequest`.** Its builder is **`FUN_1418283f0`**, which is
  **missing from `research/msexe-send-opcodes.txt`** — that table names two other builders
  for `0x00D1` and both write an empty body. `FUN_1418283f0` is called from
  `FUN_1409f6eb0`, the `0x00D9` move builder, with `targetField = -1`. **[L]**
* **All 34 captured bytes are accounted for**, and the byte count closes exactly against
  the builder's encode sequence. Four of the fields are proven against the wire: the
  portal name, the two position `u16`s, the `-1` target field, and the literal `100`. **[L]**
* The first **14 bytes are a client integrity/telemetry block**, not gameplay data: a
  literal `100`, a tamper-checked `u16` counter, and a protected `u64`. **[D]**
* **The reply is another `SetField` (`0x01A0`).** On the question of
  `characterData = 0` vs `1`:
  * The short form's precondition is now **exactly identified**: it needs
    `DAT_143aa84a0->[0x2358]` (the world's `CUserLocal` slot) to hold a live object. That
    slot — **not** "a character and a field in general" — is what faulted last time. **[L]**
  * The world constructor `FUN_142ca5c50` **zeroes** `+0x2358`, and an exhaustive scan of
    `.text` for `disp32 = 0x2358` found **no other writer**. So I **cannot prove the
    precondition is satisfied now**, and the one measurement we have says it was not
    satisfied then. **[L] for the scan, and the negative is a weak negative — see §3.4.**
  * **Recommendation: send the long form (`characterData = 1`) with map 10**, which is the
    packet the client already accepted with one `u32` changed, and which is idempotent
    because `FUN_1420a3080` is a lazy singleton. **Measure the short form's precondition on
    the same run for free** with `watch@142cfb500:peek=2358`. §3.5.
* **The server owes map 10 nothing else.** `000000010.img` exists (16448 bytes), has three
  portals, 0 mobs, 0 reactors, `town = 1`. **[L]**

---

## 1. Finding the builder, and one instrument correction

`research/msexe-send-opcodes.txt` lists two builders for `0x00D1`:

```
0x00D1  209    FUN_140d73bf0 @ 140d73bf0  (call at 140d73c13)
0x00D1  209    FUN_14107dfc0 @ 14107dfc0  (call at 14107dfed)
```

Neither can be ours. In `msexe-packet-fields.txt` both write **no primitives at all** —
`FUN_1415d01c0`, an allocate/construct/post triple, then `FUN_1406ed610`. That is an
**empty-body** `0x00D1`, and our capture has 34 bytes. **[L]**

*(Corrected 2026-09-09: those two were labelled "the outbound packet logger" and "send"
respectively, and it is the other way round - `1415d01c0` is the **send**, `1406ed610` is
`~COutPacket`. **The conclusion is not just unaffected but strengthened.** The whole argument
here is that this builder writes no field primitives, and under the corrected labels the shape
is ctor, send, then destructor at scope exit on the same `[rsp+0x30]` buffer - a packet sent
with nothing encoded into it, which is what "empty body" means stated directly rather than
inferred from an absence. See `research/cash-shop-stage.md`, the same function.)*

So I re-enumerated from the binary rather than filtering the existing list — the mistake
`CLAUDE.md` names twice. A byte scan of every `.text` section for `E8 rel32` targeting
`0x1406ed520` (the "make packet with opcode in EDX" call), taking the nearest preceding
`BA imm32`:

| opcode | scan result | table result |
|---|---|---|
| `0x008B` (control, known) | `FUN_141b28750`, `FUN_141b2cb70` | identical |
| `0x00D1` | `FUN_140d73bf0`, `FUN_14107dfc0`, **`FUN_1418283f0`** | first two only |

**`FUN_1418283f0` @ `0x1418283f0`, 1017 bytes, `mov edx,0xd1` at `0x1418284de`, call at
`0x1418284e8`.** **[L]** It is not a new function to this repo — `msexe-packet-fields.txt`
already lists it as a *callee* of three builders:

* `FUN_1409f6eb0` — the **`0x00D9` move builder**, whose string table contains `Portal`
* `FUN_1418e2270` — the `0x01A3` builder, string `mapName`
* `FUN_1428d7b80` — an `0x01ED` builder

The scan found 2067 call sites where Ghidra's script reported 1894, which is why it was
missed. **[D]**

> ### A correction to `msexe-send-opcodes.txt` worth carrying forward
>
> The table's row `0x00B8 → FUN_142e40530 (call at 142e405e5)` is a **false positive**. At
> that site the opcode is not an immediate:
>
> ```asm
> 142e405d8  8b 94 24 b8 00 00 00   MOV EDX,[RSP+0xb8]
> 142e405df  49 8b cf               MOV RCX,R15
> 142e405e2  e8 36 cf 8a fd         CALL 0x1406ed51d
> ```
>
> A backwards scan for four bytes reads the **displacement** `b8 00 00 00` as the opcode
> `0xB8`. **[L]** So the builder of the wire's `0x00B8` is **unknown**, and any other row in
> that table whose opcode equals a plausible stack displacement deserves the same check.
> My scan requires a literal `BA imm32`, so it cannot make this particular mistake — but it
> would silently miss an opcode loaded from a register, which is the trade.

---

## 2. `0x00D1` field by field

Capture: `research/fixtures/character-on-map1-playable-world.log`, 14:02:16.408, 34 bytes.

```
64000000 ad1500000000000000000000 ffffffff 0500 6f757430 30 5304 6d01 00 00 00
```

The encode sequence in `FUN_1418283f0` after `FUN_1406ed520(pkt, 0xD1)`, hand-decoded from
the bytes at `0x1418284e8`–`0x141828716`: **[L]**

| # | site | call | width | wire off |
|---|---|---|---|---|
| 1 | `1418284fa` | `FUN_140c7b890(DAT_143a82210, pkt)` | 14 | 0 |
| 2 | `14182850b` | `Encode1(byte [RBP+0x418])` | 1 | 14 |
| 3 | `141828572` | `Encode1(<stage byte> or 0)` | 1 | 15 |
| 4 | `14182857e` | `Encode4(ESI)` | 4 | 16 |
| 5 | `141828685` | `EncodeStr(temp)` | 2+n | 20 |
| 6 | `1418286c2` | `Encode2(pos.x)` | 2 | 27 |
| 7 | `1418286e6` | `Encode2(pos.y)` | 2 | 29 |
| 8 | `1418286f2` | `Encode1(0)` — hard-coded | 1 | 31 |
| 9 | `141828700` | `Encode1(R12B)` | 1 | 32 |
| 10 | `141828711` | `Encode1(byte [RBP+0x420])` | 1 | 33 |

`14 + 1 + 1 + 4 + 7 + 2 + 2 + 1 + 1 + 1 = 34` — **the count closes exactly against the
capture**, which is what makes the assignment forced rather than plausible. **[D]**

The primitives were calibrated, not assumed: `1406ed840`/`1406ed940`/`1406ed9d0`/`1406edbc0`
= Encode1/2/4/8 and `1406edc80` = EncodeStr, fixed by aligning my call dumps for
`FUN_141b28750` (`0x008B`, known on the wire as `u32 characterId`) and `FUN_1418287f0`
(`0x00D2`) against those functions' rows in `msexe-packet-fields.txt`. **[D]**

### 2.1 The table

| off | width | value here | field | confidence |
|---:|---|---|---|---|
| 0 | `u32` | `100` | literal `0x64`, `MOV EDX,0x64` at `140c7b8ca` | **PROVEN [L]** — an immediate in the code and on the wire |
| 4 | `u16` | `5549` | `FUN_140cae000(obj)` — an obfuscated, checksum-protected counter (XORs five bytes at `obj+0x88..0x8c`, reports through `FUN_141804970` with code `0x53`) | **[D]** width and source proven; *meaning* (sequence? tick?) inferred |
| 6 | `u64` | `0` | `Encode8` of a qword pulled through `*(param1+0x90)` and run through the `0x9a65` rolling-checksum unmangler, the same family as `FUN_1402fa540` | **[D]** width and provenance proven; meaning unknown |
| 14 | `u8` | `0` | the builder's **arg8**; the move-path caller writes `0` | **PROVEN [L]** by reading the call site |
| 15 | `u8` | `0` | derived from the current stage: `FUN_14209ee40()` → `[+8]` → an interface cast against `0x143a87ec8` → `[obj+0xa8]` → `byte [+0x78]`; `0` when the cast fails | **[D]** shape proven, meaning unknown |
| 16 | `u32` | `-1` | **`targetField`** — the builder's arg2. The move-path caller loads `MOV EDX,-1` literally | **PROVEN [L]** at `1409f9722` |
| 20 | `u16`+bytes | `5, "out00"` | **`portalName`** | **PROVEN [L]** — ASCII in the capture, and `research/map1-exists.md` names map 1 portal 4 `out00` |
| 27 | `u16` | `1107` | **character x** | **PROVEN [L]** — identical to the final position in the `0x00D9` sent in the same millisecond, and 9 px from the portal's own x (1116) |
| 29 | `u16` | `365` | **character y** | **PROVEN [L]** — same, and *exactly* the portal's y (365) |
| 31 | `u8` | `0` | hard-coded `XOR EDX,EDX` | **PROVEN [L]** — the client can never send anything else here |
| 32 | `u8` | `0` | the builder's **arg4** (`R9B`); caller passes `XOR R9D,R9D` | **PROVEN [L]** |
| 33 | `u8` | `0` | the builder's **arg9**; caller writes `0` | **PROVEN [L]** |

**Offsets 27–30 are written only when the portal name pointer is non-null**
(`4d 85 f6 / 74 47` at `1418286a2` skips both `Encode2`s). So a transfer request issued
*without* a portal name is **31 bytes, not 34**, and the trailing three bytes move to 27.
**[L]** Anything we build must read the string first and only then look for x/y.

### 2.2 What offsets 0–13 are, and why they can be ignored

`FUN_140c7b890` is shared by the `0x00D1`, `0x00D2` and `0x00D3` builders. It opens with

```asm
140c7b8a4  48 b8 00 10 00 40 01 00 00 00   MOV RAX,0x140001000
           0f b6 00                        MOVZX EAX,byte ptr [RAX]
```

— it reads a byte of the client's **own `.text`** — and ends by doing it again, with a
`0x9a65` rolling checksum over an 8-byte protected value in between. **[L]** That is an
integrity/telemetry preamble, not gameplay state. **[D]**

Corroboration from a different direction: the v214 reference's
`handleUserTransferFieldRequest` reads `byte, byte fieldKey, int targetField, String
portalName` — **6** bytes before the string, where we see **20**. Fields 14, 15 and 16 line
up with that three-field prefix exactly; the 14 bytes in front of them are what this
version added. **[I]**, and labelled a candidate as `CLAUDE.md` requires — the reference
scored 1 of 8 on a held-out control.

### 2.3 One side effect of sending the request

Before building the packet the client does `MOV byte [R15+0x49],1` when its arg5 is set
(`1418284d9`). **[L]** That has the shape of the delete button's `stage+0xd4` in-flight
latch. **[I]** I did not enumerate readers of `user+0x49`, so I do not know whether a second
portal use is refused while it is set. If a first reply is ignored and a second attempt does
nothing, this is the first place to look.

The client did **not** freeze on the unanswered request: it went on sending `0x00B8` at
14:02:22 and the socket closed only when the owner quit at 14:02:26. So `0x00D1` is not one of
the blocking requests — but the "always answer" rule stands regardless.

---

## 3. What the client expects back

### 3.1 The two branches, re-derived

An independent read of `FUN_142097f80` reproduced `research/msexe-stage-setfield.md`
**address for address**: the same 33-byte head at the same sites, and **exactly 50**
packet-read primitive calls, the number that document arrived at by a different method.
Two instruments now agree, so the head can be treated as settled. **[L]**

The primitives are `1406e8ae0` = Decode1, **`1406e8b80` = Decode2** (this is the u16 at
head offset 31, previously unnamed), `1406e8c20` = Decode4, `1406e9050` = DecodeStr,
`1406e9170` = DecodeBuffer(n). **[D]**

### 3.2 What the short branch actually does

From `0x14209854b`, hand-decoded: **[L]**

```asm
14209854b  CALL 1406e8ae0                  ; u8  -> EBX   (the short form's first field)
142098553  MOV  RCX,[RIP+..] -> 143aa8518  ; if (EBX != 0 ||
142098566  CALL 140f810b0                  ;     FUN_140f810b0(DAT_143aa8518+0x100) != 0)
142098576  CALL 142d62e30(world)           ;   tear the current field down
142098580  CALL 142cdeca0(world, 0)
142098585  LEA  RDX,[RBP+0x2c0]            ; an out-slot
14209858c  MOV  RCX,R14                    ; R14 = DAT_143aa84a0, never reloaded
14209858f  CALL 142cbe6e0                  ; out->[8] = world->[0x2358]; addref
1420985ac  MOV  RBX,[RSI+8]                ; the CUserLocal
...
142098685  CALL 1402fa540(user + 0xf3)     ; <-- THE FAULT SITE
14209868f  CALL 142d01d40(world, oldMapId) ; stash the map being left
142098697  CALL 1406e8c20                  ; u32 -> re-seeds user+0xfb (the protected map)
142098862  CALL 1406e8ae0                  ; u8  -> user+0x10b
142098892  CALL 1406e8c20                  ; u32 -> user+0x5b / +0x5f / +0x63, obfuscated
1420988bf  ; JOIN — the long branch's `JMP` at 142098546 lands here
```

`FUN_142cbe6e0` is **not** a map lookup, which is how this reads at first glance in the
decompiler. It is 71 bytes: **[L]**

```asm
142cbe6ea  MOV RBX,[RCX+0x2358]
142cbe6f4  MOV [RDX+8],RBX
           ...addref...
```

And `R14` is written **once** in the whole of `FUN_142097f80`, at `142097fdc`
(`MOV R14,[RIP+…] → DAT_143aa84a0`), and never again before `14209858c` — checked by
scanning the function's bytes for every `4C/4D 8B` with `reg == 6`. **[L]** So the short
branch's user is `DAT_143aa84a0->[0x2358]`, full stop.

### 3.3 Why it faulted, now proven rather than inferred

`research/charstat-layout.md` §5 says of the `characterData = 0` crash: *"I did not prove
which operand was bad, so treat the causal story as inference"*. It can be upgraded.

```asm
1402fa540  40 55 53 56 57 41 54 41 55 41 56 41 57   ; FUN_1402fa540 prologue
1402fa54d  48 8d 6c 24 e1                            LEA RBP,[RSP-0x1f]
1402fa552  48 81 ec 98 00 00 00                      SUB RSP,0x98
1402fa559  48 8b d9                                  MOV RBX,RCX
1402fa55c  4c 8b 51 08                               MOV R10,[RCX+8]   ; +0x1c
```

`+0x1c` is the **first dereference in the function**, and `RCX = user + 0xf3`. With
`user == 0` that reads address `0xFB` — inside the null page. The recorded fault was
`0xC0000005` at exactly `FUN_1402fa540+0x1c`. **[L]** A wild-but-mapped pointer would far
more likely have survived to `+0x20`. **[D]**

And `world+0x2358` starts null: **[L]**

```asm
142ca5c73  45 33 e4                XOR R12D,R12D     ; and R12 is never rewritten
...
142ca643c  4c 89 a7 58 23 00 00    MOV [RDI+0x2358],R12
```

inside `FUN_142ca5c50`, the world constructor (`STATUS.md` already identifies it as the
function that writes `DAT_143aa84a0`), in a run of zero-initialisers. **[L]**

The long branch never touches `+0x2358`: it calls `FUN_1420a3080(0)` instead. **[L]**

**So: the short form failed because the world's `CUserLocal` slot was empty on a fresh
migration.** Not "because a character and a field did not exist" in some general sense —
because *that slot* was zero.

### 3.4 Is the precondition satisfied now? — the honest answer is "not established"

The client now has a constructed `CUserLocal`: it is drawn, it walks, and
`FUN_142cc42d0` — reached from the `0x00D1` builder itself — dereferences `+0x2358` at
`[user+0x5b]`/`[user+0x63]`, the very offsets the short branch writes, which independently
confirms `+0x2358` **is** the `CUserLocal` slot. **[L]**

But I could not find the code that fills it.

* A scan of every `.text` section for `disp32 = 0x2358` found **222** sites: the one
  constructor store above, ~180 plain loads, a handful of `CMP qword [reg+0x2358], 0` null
  tests, and **no second store**. **[L]**
* `FUN_1420a3080` has exactly **four** callers — the four stage handlers `0x01A0`–`0x01A3`
  — and none of them stores the returned pointer anywhere but a local. **[L]**

**This is a weak negative and I am not treating it as evidence of absence.** The scan can
only see a store whose displacement is literally `0x2358`; a compiler that kept
`world + 0x2000` in a register and stored at `[reg+0x358]`, or a helper handed
`&world->user`, is invisible to it. Two hundred readers and several explicit null checks
say the slot is written somewhere. What I can say is: **static reading does not establish
that it is non-null after a successful long-form `SetField`, and the only measurement we
have says it was null before one.**

That is the whole answer to the question the task called central. The short form's
assumption is now *identified* precisely, and it is a single, checkable memory word — but
it is not *shown* to be satisfied.

### 3.5 Recommendation, and the free measurement

**Send the long form (`characterData = 1`) with map 10.** Reasons, in order:

1. It is **byte-identical to the packet the client accepted 37 seconds earlier** except one
   `u32` (stat-block offset 84: `1` → `10`). That is one variable, which is the rule.
2. It is **idempotent by construction**. `FUN_1420a3080` is a lazy singleton:
   ```asm
   1420a3100  48 8b 75 28   MOV RSI,[RBP+0x28]
              48 85 f6      TEST RSI,RSI
              75 14         JNZ  (skip)
              ...  operator new(0x14c0) ...
              48 89 45 28   MOV [RBP+0x28],RAX
   ```
   A second long-form `SetField` reuses the same `CUserLocal` rather than building a new
   one. **[L]**
3. The map is loaded in the **common tail**, after the two branches join, out of
   `FUN_1402fa540(user + 0xf3)` — the tail calls it at `+0x95f`, `+0x982` and `+0xbb8`.
   Both branches feed that same word, so **either form changes the map**; the long form
   just reaches it by a route we have already flown. **[D]**

**And measure the short form's precondition on the same run, for free.** `FUN_142cfb500` is
called at `SetField+0x73` with `RCX = world` — this is the latch read that already prints
`[rcx+0x33f4]=u8:0x00` in the hook log. The probe's `peek=` option prints an arbitrary
offset:

```
watch@142cfb500:peek=2358
```

That fires **once per `SetField`**, and it carries its own negative control:

| reading | means |
|---|---|
| first `SetField` (the migration): `u32:0x00000000` | the slot was empty, as §3.3 predicts — the instrument is telling the truth |
| second `SetField` (this transfer): non-zero | **the short form's precondition is satisfied**; switch to it next run |
| second `SetField`: still zero | the short form would fault again; the long form is the only option and §3.4's negative was right |
| no `WATCH` line at all | the hook did not arm — conclude nothing |

One caveat on the falsifier: `peek` shows the low `u32` of the pointer, so a zero reading is
"almost certainly null" rather than "certainly null". A heap pointer with a zero low dword
is possible and vanishingly unlikely. **[I]**

---

## 4. The short form's exact byte layout, if it is chosen

For completeness, and so it can be built the moment §3.5's measurement comes back positive.
Every offset is from the listing. **[L]**

```
off  width  field
---  -----  -------------------------------------------------------------------
  0   u8[8] server clock base            }
  8   u32   channel id                   }
 12   u8    -> world+0x226c              }
 13   u32   -> world+0x2884              }  the SAME 33-byte head we already
 17   u8    ==1 clears a tree - send 0   }  build; nothing here moves
 18   u32   read and discarded           }
 22   u32                                }
 26   u32                                }
 30   u8    characterData = 0            <-- the only change in the head
 31   u16   string count = 0             }

 33   u8    non-zero -> tear down the current field (142d62e30 + 142cdeca0)
 34   u32   THE NEW MAP ID  (10)         <-- re-seeds user+0xfb, read back as the map
 38   u8    -> user+0x10b
 39   u32   -> user+0x5b / +0x5f / +0x63

 43   ...   the common tail, identical to the long form's, which we already
            satisfy with zeros plus the existing pad
```

So the short form is **10 bytes where the long form is 12 + 112 + …**, and everything from
offset 43 on is unchanged from `set_field_with_character`. In `crates/net/src/opcode.rs`
that is `set_field_head(clock, channel, 0)` with byte 30 left at
`SET_FIELD_NO_CHARACTER_DATA`, then `[0u8], map_id.to_le_bytes(), [0u8], [0u8;4]`, then the
same zero tail.

Two things to know before trusting it:

* **Field 33 is the field teardown.** It is OR-ed with a client-side condition
  (`DAT_143aa8518` / `FUN_140f810b0`), so sending `0` does **not** guarantee the old field
  survives, and sending `1` does force the teardown. Which value is correct for a portal
  transfer is **not established**. **[I]** `0` is the conservative choice on the first try
  because it matches "the server asserts nothing".
* **Field 39's `u32` is consumed by the protected-value machinery** at `user+0x5b`, the same
  triple `FUN_142cc42d0` reads. Zero is what we send everywhere else and nothing in the
  branch divides by it. **[D]**

---

## 5. What else the client needs — before, during and after

1. **Nothing about the field's contents.** `research/map1-exists.md` surveyed all 426 field
   images. `000000010.img` is present (16448 bytes) with **0 mobs, 0 reactors, `town = 1`**,
   and the client loads the field from WZ itself — exactly as it did for map 1, where we
   sent no field data at all and a character stood on it. **[L]**
2. **No spawn packet.** The character was drawn on map 1 with nothing but the `SetField`
   record. **[L]**
3. **The map-name lookup will hit.** `FUN_1403999e0` only builds `Map/Map/Map%d/%09d.img`
   and reaches its non-returning `E_POINTER` path on a *miss*; map 10 has both a field image
   and a `String.wz` entry. **[L]** (`map1-exists.md` addendum established the failure mode.)
4. **The portal byte will land you in the wrong place, harmlessly.** Map 1's `out00` targets
   **map 10 portal `in00`**, which is index **1**: **[L]**

   | idx | pn | pt | x | y | tm | tn |
   |---|---|---|---|---|---|---|
   | 0 | `sp` | 0 | -91 | 181 | 999999999 | "" |
   | **1** | **`in00`** | **2** | **-195** | **278** | **1** | **out00** |
   | 2 | `out00` | 2 | 706 | 216 | 20 | in00 |

   We send `portal = 0` at stat-block offset 88. Portal 0 exists and is a type-0 `sp`, so
   the lookup cannot miss — the character just arrives at the spawn point instead of at the
   door. **Leave it at 0 for this run** (one variable), and try `1` afterwards; that also
   finally settles whether the byte is an index or a spawn id, which
   `research/charstat-layout.md` records as open. **[D]**
5. **`0x00D1` may need to be answered even to be *refused*.** There is no refusal opcode in
   evidence and the v214 reference simply warps or drops the request. If the transfer is to
   be denied, the honest form is a `SetField` naming the *current* map rather than silence.
   **[I]**
6. **Watch for the "MAP" toast.** `research/setfield-zero-audit.md` established that the
   handler shows a spliced `\tM\rA\nP` notice with the map name whenever
   `FUN_142d01d50(world)` differs from `FUN_1402fa540(user+0xf3)`. Going 1 → 10 it will
   differ, so **a "MAP" line naming map 10 is the on-screen confirmation that the id
   arrived and resolved**; a blank one means the id arrived and the WZ lookup missed. **[L]**

---

## 6. The other opcodes in the capture — one line each, guesses marked

* **`0x00D9`** — the **move packet**. Builder `FUN_1409f6eb0` (10498 bytes), and it is the
  function that calls the `0x00D1` builder, which is why the two arrive in the same
  millisecond. **[L]** The final pair in the last one, `53 04 6d 01`, is byte-identical to
  `0x00D1`'s x/y. Its second dword is an obfuscated counter whose adjacent bytes sum to a
  constant (`0x57+0xa3 = 0xFA`, `0x55+0xa5`, `0x53+0xa7`, …), i.e. a self-checking sequence
  number. **[D]**
* **`0x013D`** — 9 bytes, `u8` flag then two zero `u32`s; builder `FUN_142d19260`, in the
  world-object subsystem. Sent once at field entry with `01` and once mid-session with `00`.
  **Guess [I]:** a show/hide or open/close report for some world-owned UI element.
* **`0x00B8`** — 1 byte, alternating `01`/`00` at long irregular intervals (`01` at entry,
  `00` at +5s, `01` at +25s, `00` at +42s). **Guess [I]:** window focus / foreground state.
  Its builder is **unknown** — see the `msexe-send-opcodes.txt` correction in §1; the row
  naming `FUN_142e40530` is an artefact of a stack displacement.

---

## 7. Gaps, stated rather than papered over

1. **The writer of `world+0x2358` was not found.** §3.4. This is the one thing that decides
   the short form, and it is unresolved. `watch@142cfb500:peek=2358` answers it empirically
   at zero extra cost.
2. **`FUN_1418283f0` contains a `JMP` into `.themida` at `+0x41`**, before the packet is
   constructed, with ~70 bytes of non-code after it. The *encode* section from `+0xf8`
   onward is entirely plain code and is what §2 is built on, but there is logic in front of
   it I cannot read. **[L]** It does not affect the decode; it could affect *when* the
   request is sent.
3. **Fields at wire offsets 4, 6 and 15 have proven widths and unproven meanings.** They are
   zero or client-derived in every capture we have, so the server can ignore them — but if a
   future packet needs a non-zero value there, none of this says what it should be.
4. **The 31-byte variant** (no portal name, `targetField` set) has never been captured. Any
   parser must branch on the string length rather than assume 34 bytes. **[D]**
5. **Field 33 of the short form** — the teardown flag — has no established correct value.
6. **My call-sequence dumps are byte scans, not a disassembler.** They are a *superset* of
   the direct calls (indirect `CALL [reg+n]` is invisible, and a stray `E8` in data reads as
   a call — several obviously-bogus targets appear in the raw output and were discarded).
   Where it mattered I hand-decoded the actual instructions; where the report says a call
   sequence, that is what it is.
