# `0x00E5` decoded against three real captures, and the client is waiting to be told its HP

2026-08-20. No client run was spent on this; the captures are from the run the owner already
made. Tags: **[L]** read off a listing or a capture, **[D]** derived from something [L],
**[I]** inferred or from the reference source.

Listings added beside this file:

| | |
|---|---|
| `research/msexe-hitinfo-encode-14025d810.txt` | `FUN_14025d810`, the HITINFO serialiser - 155 lines, **no branches** |
| `research/msexe-userhit-mobbuilder2-1428aa0a0.txt` | `FUN_1428aa0a0`, **the builder that produced all three captures** |
| `research/msexe-userhit-mobbuilder-14288ac30.txt` | `FUN_14288ac30`, the other mob builder - ruled out, §3.1 |
| `research/msexe-userhitshow-142771360.txt` | `FUN_142771360`, the show/log function |
| `research/msexe-statchanged-142d54780.txt` | `FUN_142d54780`, the `0x007C` handler - the HP arm and the death branch |
| `research/msexe-statdecode-1402cbb50.txt` | `FUN_1402cbb50`, the `0x007C` value decoder |

The run is preserved as
`research/fixtures/user-hit-00e5-147-byte-bodies-world.log`.

---

## 0. The short answer

**The client does not subtract its own HP.** It reports the damage and waits. The owner's bar did
not move because nothing told it to. The reply is a `0x007C` carrying the new HP - 13 body
bytes, `net::stats::StatChange::hp_only`, added in this pass with the bytes pinned.

The body decodes cleanly. The fields the server needs are:

| | | |
|---|---|---|
| **damage** | body offset **+4** (struct `+0x08`) | `1` in all three captures |
| **mob object id** | body offset **+46** (struct `+0x34`) | `2004`, `2000`, `2000` |
| mob **template** id | body offset **+113** (struct `+0x84`) | `2` - the template the server sent |

**This falsifies `research/touch-damage.md` §5.3**, which reported that a body/touch-damage
path could not be found in this client. It exists, it fires on contact with a snail, and it
reports through `FUN_1428aa0a0` with attack index **-1**. §8 records that correction and
four others; I do not own that file.

---

## 1. The instruments, and their controls

Every claim below rests on a tool that was made to speak first, per `CLAUDE.md`.

| instrument | positive control | result |
|---|---|---|
| `tools/listing.py` | `0x140304100` must show `raw@140304138, u8@140304144, u8@140304183`, then a `u16` run | reproduced exactly, and matches `tools/reads.py` |
| `tools/callers.py` | `0x1402fa9a0` must give 96 sites in 15 functions | reproduced exactly |
| whole-image store scan (§4.1) | must find `1402cbcaa`, the `0x007C` decoder's hp store | found |
| whole-image `lea` scan (§4.2) | must find `142d5485c`, the `0x007C` handler's oldHp snapshot | found |
| hp-sign-branch scan (§6.2) | must find `142903db7`, read by hand first | found |
| remote jump table (§5.3) | index 17 must decode to `0x02AF`, which `research/level-up.md` established independently | reproduced |
| latch grep (§7) | must find `[rcx+0x2330]` inside `FUN_142cc4430` | found |

---

## 2. The body closes against all three captures

`FUN_14025d810` is **604 bytes and completely straight-line** - I read all 155 lines and there
is not one conditional branch. So the body is **unconditionally 147 bytes**; there are no
gated fields and no length prefix. **[L]** That matters because it is the property
`tools/encodes.py` cannot establish on its own, and it means a decoder can be a fixed layout.

The wire order and struct offsets in `research/touch-damage.md` §1.2 are **correct as
published** - I re-derived them from the listing rather than trusting them, and they agree.

### 2.1 The alignment is confirmed independently of the length

Length alone is weak evidence: many layouts sum to 147. Two independent checks pin it.

**The tick.** Struct `+0x14` (body offset +16) across the three captures:

```
183329718   183395058   183404448
```

against the log's own timestamps `18:35:37.086`, `18:36:42.417`, `18:36:51.808`:

```
capture 1->2: tick delta 65340 ms, wall clock 65331 ms, error   9 ms
capture 2->3: tick delta  9390 ms, wall clock  9391 ms, error  -1 ms
```

A millisecond counter, tracking the log to within 9 ms and 1 ms. **[L]** The builder fills
this field from `FUN_1429e3ef0()` at `1428ad4e3`, which is what `touch-damage.md` called
"the update tick". Confirmed. A one-field misalignment would have destroyed this.

**The position.** Struct `+0x5c`/`+0x60` (body offsets +73/+77) are `841, 395` and `878, 395`.
The next `0x00D9 CLIENT_USER_MOVE` in the same log begins its path at `49038b01` and
`6e038b01` - `x=841 y=395` and `x=878 y=395`. **Exact match on both.** **[L]**

### 2.2 The three bodies, field by field

Body offsets are from the start of the body (opcode already stripped). "same" means all
three captures agree.

| body | struct | kind | cap 1 | cap 2 | cap 3 | what it is |
|---|---|---|---|---|---|---|
| +0 | `0x00` | u32 | 0 | 0 | 0 | ctor fills it from `FUN_141892bb0()` as a zero-extended **byte** - a bool. Unnamed. **[L]** shape |
| **+4** | **`0x04`** | u32 | **-1** | **-1** | **-1** | **the mob's attack index. `-1` = the body/touch, i.e. no `attack<N>` entry.** §3.2 |
| **+8** | **`0x08`** | u32 | **1** | **1** | **1** | **the damage.** Argument 2 of the builder; the same register is negated into the `/logUserHitDamage` printer. §3.3 |
| +12 | `0x10` | u32 | 2162690 | 2558854294 | 1076015281 | **a client PRNG value**, not semantic. §3.4 |
| +16 | `0x14` | u32 | *(tick)* | | | the millisecond tick, §2.1 **[L]** |
| +20 | `0x18` | u32 | 0 | same | same | builder writes literal 0 |
| +24 | `0x1c` | u8 | 0 | same | same | ctor default, builder never writes it |
| +25 | `0x20` | u32 | 0 | same | same | builder writes 0 on this arm |
| +29 | `0x24` | u8 | 0 | same | same | from a byte local |
| +30 | `0x0c` | u32 | 1 | same | same | from `[rbp+0x20]`, the same value as `+0x30` |
| +34 | `0x28` | raw4 | `01000000` | same | same | first half of a `movsd` covering `+0x28..+0x2f` |
| +38 | `0x30` | u32 | 1 | same | same | from `[rbp+0x20]`, same source as `+0x0c` |
| +42 | `0x2c` | u32 | 1 | same | same | second half of that same `movsd` |
| **+46** | **`0x34`** | u32 | **2004** | **2000** | **2000** | **the mob OBJECT id.** `FUN_141c54eb0(mob)` at `1428ad6c9` |
| +50 | `0x38` | u32 | 2004 | 2000 | 2000 | **the same accessor again** - `FUN_141c54eb0(mob)` at `1428ad614`. Not a template id; that is why the two are always equal |
| +54 | `0x3c` | u8 | **1** | 0 | 0 | a sign bit - direction/`left`. **[L]** it is a sign, **[I]** the name |
| +55 | `0x3d` | u8 | 0 | same | same | from a byte local |
| +56 | `0x3e` | u8 | 0 | same | same | conditional byte |
| +57 | `0x44` | u8 | 0 | same | same | conditional byte |
| +58 | `0x3f` | u8 | 0 | same | same | `r13d != 0` |
| +59 | `0x48` | u8 | 0 | same | same | inside a `movups xmm7` covering `+0x48..+0x57` |
| +60 | `0x49` | u8 | 0 | same | same | same `movups` |
| +61 | `0x50` | u64 | 0 | same | same | same `movups` |
| +69 | `0x58` | u32 | 0 | same | same | from `[rbp+0x44]` |
| **+73** | **`0x5c`** | u32 | **841** | **878** | **783** | **the player's X.** §2.1 |
| **+77** | **`0x60`** | u32 | **395** | 395 | 395 | **the player's Y.** Written together as one qword at `1428ad69e` |
| +81 | `0x64` | u32 | 0 | same | same | literal 0 |
| +85 | `0x68` | u32 | 0 | same | same | from `[rbp+0x40]` |
| +89 | `0x6c` | u32 | 0 | same | same | ctor default on this arm |
| +93 | `0x70` | u32 | -1 | same | same | `FUN_141c56830(mob)` |
| +97 | `0x74` | u32 | 0 | same | same | `FUN_141c56cc0(mob, 0)` |
| +101 | `0x78` | u32 | -1 | same | same | ctor default `0xffffffff`, not overwritten on this arm |
| +105 | `0x7c` | u32 | 0 | same | same | ctor default |
| +109 | `0x80` | u32 | 0 | same | same | ctor default |
| **+113** | **`0x84`** | u32 | **2** | **2** | **2** | **the mob TEMPLATE id.** §3.5 |
| +117..+129 | `0x88`..`0x94` | u32 x4 | 0 | same | same | ctor default / a qword of `rdi` |
| +133 | `0x98` | u8 | 1 | same | same | from a byte local |
| +134 | `0x9c` | u32 | 0 | same | same | ctor default |
| +138 | `0xa0` | u8 | 0 | same | same | ctor default |
| +139 | `0xa4` | u32 | 0 | same | same | ctor default |
| +143 | `0xa8` | u32 | 0 | same | same | ctor default, written by the encoder's tail `jmp` |

**Named with confidence:** damage, mob object id (twice), mob template id, tick, player X/Y,
attack index, direction bit. **Not named:** `+0x00`, `+0x0c`/`+0x30` (both `1`), `+0x10`
beyond "PRNG", and the run of ctor-default zeros. Those are all constants across the three
captures and none of them is needed to apply damage.

---

## 3. Which builder, and why that settles the field meanings

### 3.1 It is `FUN_1428aa0a0`, not `FUN_14288ac30` **[L]**

`research/touch-damage.md` §2.4 names both as the mob resolver's send sites. Only one can
have produced these bodies.

The HITINFO ctor `FUN_14025d6a0` **zeroes the whole struct** except `+0x70`/`+0x74` (a qword
of `-1`) and `+0x78` (`-1`), then sets `+0x00` from `FUN_141892bb0()`. So any field that is
non-zero on the wire was written by the builder.

`FUN_14288ac30`'s HITINFO base is `rbp+0x140`. A grep of its full listing for writes to
`+0x14c`, `+0x168`, `+0x16c`, `+0x170`, `+0x19c`, `+0x1a0` - i.e. struct `+0x0c`, `+0x28`,
`+0x2c`, `+0x30`, `+0x5c`, `+0x60` - returns **zero**, while the same grep finds the fields
it *does* set (`+0x144`, `+0x148`, `+0x174`, `+0x178`). Our captures have `+0x0c = 1`,
`+0x30 = 1`, `+0x5c = 841`, `+0x60 = 395`. **`FUN_14288ac30` cannot have built them.**

`FUN_1428aa0a0` (base `rbp+0x240`) writes all of them, including
`mov qword ptr [rbp+0x29c], rcx` at `1428ad69e` - the player's X and Y as one 64-bit store
across `+0x5c` and `+0x60`, taken from `[vtbl+0x30]` on the user object.

### 3.2 `+0x04` is the attack index and `-1` means the body **[L]/[D]**

Scanning all 14 `0x00E5` builders for immediate stores into HITINFO `+0x04`:

```
FUN_14185ad30   -11      FUN_14288cb40    -5      FUN_14288d340    -6
FUN_14288d4d0   -11      FUN_1428923e0    -4 (x3) FUN_142903600    -6
FUN_1429077f0    -4
```

**No builder writes `-1` as an immediate.** The remaining builders - including both mob ones
- set `+0x04` from a register. So the negative constants are an enum of *non-mob* damage
sources, and `-1` is a **passed-in value**. `FUN_1428aa0a0` has two arms: the no-mob arm at
`1428ad527` computes `-2` or `-3` (`neg / sbb eax,eax / add eax,-2`), and the mob arm at
`1428ad608` takes `[rbp-0x3c]`. Our `-1` is the mob arm. **[L]**

`-1` as "not one of the mob's `attack<N>` entries" is **[D]**, and it is the reading that
makes `touch-damage.md` §3 consistent with these captures: the snail has no `attack` node, so
there is no index to report, so the client reports `-1`.

### 3.3 `+0x08` is the damage **[L]**

`FUN_1428aa0a0`'s prologue: `1428aa0fd mov r12d, edx` - **r12d is argument 2**. It goes
straight into the HITINFO at `1428ad4ca mov dword ptr [rbp+0x248], r12d` = struct `+0x08`.

The anchor is what happens to the same register afterwards. In the field-damage builder
`FUN_14185ad30`, `touch-damage.md` already showed `mov dword [rsp+0x98], edi` (struct `+0x08`)
then `neg edi` then `call 0x142771360`. `FUN_14288ac30` does the same at `14288c725`
(`mov edx, r12d / neg edx / ... call 0x142771360`). And `FUN_142771360` is the **sole caller**
of the function that prints `"[Client] User Hit Damage: %d"`.

So struct `+0x08` is, by construction, the number the client's own GM command calls the user
hit damage. Value `1` in all three captures.

### 3.4 `+0x10` is a PRNG value, not a field the server needs **[L]**

At `1428aa54c`: `lea rbx,[rbp+0x440]`, then a loop calling `FUN_140268a60(rng)` and storing
to `[rbx]`, `rbx += 4`, until `rbx == rbp+0x468` - **ten u32 slots**, seeded by
`FUN_140268a70` just above. The builder then takes `[rbp+0x440]`, the first slot, into
struct `+0x10` at `1428ad4d7`. That is why the three captures show unrelated values.
Naming it "nonce / anti-replay" is **[I]**; that it is client-generated randomness is **[L]**.

### 3.5 `+0x84` is the mob template id, and the capture proves it **[L]/[D]**

The mob arm at `1428ad6d7`:

```asm
1428ad6d7  call 0x141c54dd0        ; mob -> its template record
1428ad6dc  mov  ecx, [rax + 0x60]
1428ad6df  mov  dword ptr [rbp + 0x2c4], ecx   ; struct +0x84
```

`FUN_141c54dd0` is the same accessor `FUN_14185ad30` calls at `14185af31` immediately after
the mob-pool lookup. The field's value in all three captures is **2**, and the server's own
spawn packets in the same log say *"MobEnterField: template 2 ... object id 2000"*. The
value the client reports back is the template id the server minted. **[L]** the value,
**[D]** the name.

That is worth stating plainly because it corrects a natural reading of `touch-damage.md`
§1.2, which lists `+0x34` and `+0x38` as "mob-derived ids" and might be taken for
(object id, template id). They are **the same accessor called twice** and both are the
object id; the template id is `+0x84`.

---

## 4. The client does **not** subtract its own HP

This is the load-bearing claim, so it is established three ways with different blind spots.

HP lives at `charstat + 0x5b`, obfuscated: the plaintext is passed through
`FUN_1407386b0`, stored at `+0x5b`, and guarded by two derived words at `+0x5f` and `+0x63`.
It is read back through `FUN_1401ba9d0(&field, key)`. `research/level-up.md` §1 established
the offset; the encode/checksum shape is read here off `FUN_1402cbb50` at `1402cbca5..1402cbcbc`
and off the out-of-line setter `FUN_14299d9a0` (71 bytes - `CharacterStat::SetHP`).

### 4.1 Every write to the field, enumerated **[L]**

Disassembling **every `.pdata` extent in the image** and keeping every instruction whose
destination operand is a 4-byte `[reg+0x5b]`, any mnemonic, any source:

> **18 writes total; 3 stack-based, 15 through an object pointer.**

Of the 15, seven are disassembly-resync artefacts on unrelated structures (`fnsave`, `rcl`,
`shl`, `neg`, an `fs:`-segment store, `xor`/`add` with 32-bit addressing). The eight real
`mov ..., eax` stores are:

```
1402cbcaa  FUN_1402cbb50   the 0x007C stat decoder      <- POSITIVE CONTROL, found
1402f7952  FUN_1402f78c0   charstat writer
140302fde  FUN_140302e30   charstat writer
1420988a5  FUN_142097f80   a packet handler
142905cf4  FUN_1429052d0   writes hp and mp together
14299d9bb  FUN_14299d9a0   CharacterStat::SetHP
1402d09f3  FUN_1402d0950   NOT charstat - stores a double at +0x53 and a raw u32 at +0x5b
1402d0cee  FUN_1402d0bf0   same shape
```

I enumerated before filtering, and the control was found.

### 4.2 The `lea`-handoff blind spot, closed **[L]**

`CLAUDE.md` records the exact failure this scan would otherwise have: a store through a
pointer that was `lea`'d and handed to a helper is invisible to a `[reg+disp]` write scan.
So: **every `lea reg,[reg+0x5b]` in the image - 196 of them.**

The control (`142d5485c`, the `0x007C` handler's oldHp snapshot) is present. Classifying the
rest: all but a handful are stack locals (`rsp`/`rbp` bases) or 32-bit `lea` used as
arithmetic. Every site whose base is an object pointer feeds `FUN_1401ba9d0` - **the getter**.
The three that my first 5-instruction window did not classify were widened by hand and are
all reads too; one of them, `FUN_142936030`, is a bare `lea rcx,[rax+0x5b] / jmp 0x1401ba9d0`
- i.e. `CharacterStat::GetHP`.

**No pointer to the HP field escapes to anything that writes it.** The two scans in §4.1 and
§4.2 have *different* blind spots, which is the corroboration `CLAUDE.md` demands - unlike
the two write-scans that agreed and were wrong the same way.

### 4.3 None of the writers is reachable from the hit path **[D]**

A call + tail-`jmp` walk from `FUN_141c69f40`, its nine handlers, `FUN_141c626c0`,
`FUN_141c6f830` and both mob builders, to **depth 8 - 5716 functions**:

```
controls:  0x14025d810 HITINFO encoder     REACHED
           0x142771360 show/log            REACHED
           0x142918b40 the damage printer  REACHED
hp writers:                                NONE
```

Widening to all 14 `0x00E5` builders reaches exactly **one** writer, at depth 2, by exactly
one path: `FUN_1428923e0 -> FUN_140d1bf80 (14289456d) -> FUN_1429052d0 (140d1c308)`.
`FUN_1428923e0` is the `-4` builder. **It is not the builder that sent our packets** (§3.1),
and it is not on the mob resolver's path at all.

`FUN_142771360` - the function called immediately after the send with the *negated* damage,
which is the obvious place to apply it - calls `FUN_1401ba9d0` three times and no writer.
It **reads** HP; it never sets it. **[L]**

**Named blind spot.** A call-edge walk cannot see a virtual call (`call qword ptr [rax+N]`),
so §4.3 alone is **[D]**. It does not stand alone: §4.1 and §4.2 together enumerate the write
sites themselves, and a virtual call still has to arrive at one of them.

### 4.4 So the owner's observation is explained

The bar did not move because **nothing moved it**. Three hits, 147 bytes each, unanswered.
The client's floating damage number and its HP bar are driven from two different places: the
number at *send* time by `FUN_142771360`, the bar at *receive* time by `0x007C`.

---

## 5. What to send back

### 5.1 `0x007C` with the new HP - right, and sufficient **[L]**

`net::stats::StatChange::hp_only(new_hp)`, added in this pass. **13 body bytes**, pinned by
`the_reply_to_a_user_hit_is_thirteen_bytes`:

```
01 00 01        head
00 04 00 00     mask = bit 10 = hp
2c 00 00 00     the new HP, u32 LE
00              charm absent
00              recovery absent
```

Bit 10 is confirmed twice from the listing: `1402cbc8e bt ebp, 0xa` gates the store into
`charstat+0x5b`, and `142d5617c bt r12d, 0xa` gates the handler's HP arm.

**Send the new HP, not a delta.** The decoder stores the decoded value straight into the
field; there is no subtraction anywhere on that path.

The head bytes are the existing defaults `1, 0, 1`. Byte 1 clears the exclusive-request
latch, which is harmless here and is what `StatChange::default()` already does.

### 5.2 Nothing else is required for the local player **[D]**

The handler's HP arm calls `FUN_142d9cd90` (a reader of the field, i.e. the bar refresh),
verifies the `+0x5f`/`+0x63` checksum, and moves on. No second packet is needed to make the
bar move. The `recovery` trailer is a *separate* pair of numbers handed to
`FUN_140fd31f0`; it is not how ordinary HP change is reported and should stay `None`.

### 5.3 For other players on the field: `0x02A5` **[L]**

There is a decoder mirroring the encoder. `FUN_14025da80` is 614 bytes to the encoder's 604
and `tools/reads.py` counts **44 reads** against the encoder's 44 written fields. Tracing up:

```
FUN_14025da80  (the HITINFO decoder, 44 reads)
  <- FUN_14025d750     one caller, at 14025d7ef
  <- FUN_1429d48c0     one caller, at 1429d48ff ; its ONLY read path is this decoder
  <- FUN_1429bb720     stub at 1429bb9b2
```

`research/level-up.md` §6 already established `FUN_1429bb720` as the **remote-user** handler
for `0x293..0x2C4`, reading `u32 charId` before dispatch, with `index = opcode - 0x29e` and
the table at `0x1429bbc34`. The stub `1429bb9b2` is **index 7**, so the opcode is **`0x02A5`**.

*Control for that table decode:* index 17 comes out as `0x02AF`, which `level-up.md`
independently established as `USER_EFFECT_REMOTE`. The table reproduces a known answer.

So the broadcast is **`0x02A5` = `u32 charId` + the same 147-byte HITINFO = 151 body bytes**,
and it is unimplemented. It is cosmetic for a single-player test - remote clients use it to
draw the hit - so it is not needed to make the owner's bar move. **[D]** that it is optional.

---

## 6. Death - what is established and where I stopped

### 6.1 `hp = 0` in a `0x007C` is **not** by itself a death packet **[L]**

The handler's HP arm at `142d5617c`:

```asm
142d5617c  bt   r12d, 0xa                  ; the hp bit
142d56181  jae  142d56242                  ; not present -> skip
142d5618c  call 0x142d9cd90                ; the bar
142d5619c  mov  edx,[r13+0x63]             ; the guard word
142d561a6  mov  ecx,[r13+0x5b]             ; the stored hp
142d561aa  mov  eax,[r13+0x5f]
...        ebx = rol(eax,5) ^ ecx          ; ebx = the DECODED hp
142d561c6  cmp  eax, edx / jne -> build a string, call 0x141804970   ; integrity report
142d56220  test ebx, ebx
142d56222  jg   142d56346                  ; hp > 0
```

Both arms converge four instructions later. The `jg` target is
`142d56346: xor ecx,ecx / jmp 142d5622c`; the fall-through is
`142d56228: xor ecx,ecx / mov esi,ecx`. **The only difference is whether `esi` is forced to
zero**, which gates one write of `0` to `[global+0x1560]`. There is no death animation, no
stance change and no UI here.

So a server that sends `hp = 0` gets a zeroed bar and nothing else. **[L]**

Note the integrity check: a `0x007C` whose HP does not survive the encode/checksum round trip
makes the client build a string and call `FUN_141804970` with `edx = 0x53`. Sending a
well-formed value avoids it; this is not a reason to avoid `0`.

### 6.2 What HP <= 0 actually does: it disables the player **[L]**

Scanning for sites that read `charstat+0x5b` through the getter and immediately branch on
sign or zero - control `142903db7`, found:

> **65 sites.** Four are in the `0x00E5`/attack builders; the rest are a long run of small
> functions in `142caf...`-`142d1c...`, each of the shape *read hp, `jle` to an early exit*.

That is the shape of a family of "is the player alive" action gates, one per action. So the
client's death model is: **the server sets hp to 0, and everything the player can do is
gated off.** Naming them individually is **[I]**; the shape and the count are **[L]**.

### 6.3 Where I stopped

I did **not** find what plays the death/tomb sequence, and I did not find a revive request or
a revive reply. The utf16 `"dead"` stance strings (`0x1432793f8`, `0x143279418`,
`0x143279438`) are referenced from three avatar-animation functions (`FUN_14023d790`,
`FUN_14023e8e0`, `FUN_1402385d0`), which is where that thread should be picked up.

What is safe to build on today: **the server is the authority, the client will not kill
itself, and `hp = 0` will not crash or hang it** - it takes the same four instructions as
`hp > 0` and then continues.

---

## 7. `0x00E5` is **not** a latch **[L]**

Asked from the listing, as instructed, rather than from "it kept playing".

The client's one-request-outstanding latch is `[ctx+0x2330]` with `[ctx+0x2334]` as its
timestamp, set by `FUN_142cc4430`. Two checks:

1. **No call.** Of the 14 `0x00E5` builders, the mob resolver, its nine handlers,
   `FUN_142771360` and the encoder, exactly **one** function calls `FUN_142cc4430` -
   `FUN_14289a3a0`, at three sites. Neither mob builder does, and neither does anything on
   the resolver's path. *Control:* `tools/callers.py 0x142cc4430` reports **241 call sites in
   204 functions**, so the instrument sees latch calls easily.
2. **No direct write.** Grepping the full listings of `FUN_1428aa0a0`, `FUN_14288ac30` and
   `FUN_142771360` for `[reg+0x2330]` or `[reg+0x2334]` returns **zero**. *Control:* the same
   grep over `FUN_142cc4430` finds `142cc4439 mov dword ptr [rcx+0x2330], edx` and
   `142cc4444 mov dword ptr [rbx+0x2334], eax`.

So an unanswered `0x00E5` blocks nothing. That matches the run - three unanswered, minutes of
play afterwards - but it is now established from the code rather than from the observation.

**This does not mean it may be left unanswered.** `CLAUDE.md`'s "always answer" rule is about
not returning an *error*; here the reply is the entire feature. An unanswered `0x00E5` is
precisely why the owner takes no damage.

---

## 8. Corrections to files I do not own

Recorded here as instructed; `research/touch-damage.md` and `research/mob-combat.md` are
untouched.

1. **`research/touch-damage.md` §0 and §5.3 - "I could not establish that this client has a
   body/touch damage path at all"** and *"A body/touch-damage path in this client was not
   found."* **Falsified by capture.** Three `0x00E5` arrived from mob object ids the server
   minted, built by `FUN_1428aa0a0`, carrying attack index `-1` and the mob's template id.
   The path exists and runs. §5.3 was careful to name its blind spot and label itself **[D]**
   - the file was right to hedge, and the hedge is what the captures cash in.

   §3's *"a snail cannot hurt the player through §2's machinery"* is **not** contradicted:
   the `attack<N>` machinery is indeed empty, which is exactly why the reported index is `-1`
   rather than `0` or above. What was wrong is the inference that no *other* path exists.

2. **`research/touch-damage.md` §1.2 table - "`+0x34`, `+0x38` | `FUN_141c54eb0(mob)` |
   mob-derived ids".** Correct but easy to misread as (object, template). Both are the mob
   **object id** - the same accessor, called twice, `1428ad614` and `1428ad6c9`, and equal in
   every capture. The template id is `+0x84`, from `FUN_141c54dd0(mob)` then `[rax+0x60]`, and
   it reads `2` against the server's own `template 2`.

3. **`research/touch-damage.md` §1.3 - `FUN_14288ac30` and `FUN_1428aa0a0` as the two central
   builders.** Confirmed, and `FUN_1428aa0a0` is the one that fires for touch damage.
   `FUN_14288ac30` is ruled out for these captures by §3.1.

4. **`research/touch-damage.md` §1.1 - "14 distinct `0x00E5` builders" - there are 15, and
   `research/msexe-send-opcodes.txt` is missing one.** `tools/callers.py 0x14025d810` reports
   17 call sites in **15** functions; the extra is **`FUN_142907b60`**, and it builds
   `0x00E5` outright:

   ```asm
   142907c15  call 0x14025d6a0                     ; HITINFO ctor at [rsp+0x30]
   142907c1a  mov  dword [rsp+0x34], 0xfffffffc    ; +0x04 = -4
   142907c22  mov  dword [rsp+0x38], ebx           ; +0x08 = the damage
   142907c56  mov  edx, 0xe5                       ; <- 0x00E5
   142907c76  call 0x14025d810
   ```

   `touch-damage.md` took its list from `research/msexe-send-opcodes.txt`, which does not
   name this function at all - grepping that file for `142907b60` returns nothing. So the
   send-opcode listing is **incomplete**, and anything derived from it by enumeration is
   suspect. §1.1's conclusion survives: it rests on the 13-of-14 overlap, not the count, and
   the count is now 14 of 15.

   It also **strengthens** §3.2 above rather than weakening it: this fifteenth builder writes
   `-4` as an immediate, so the "no builder writes `-1` as an immediate" scan now covers 15
   and still holds.

5. **Not every `0x00E5` is 147 bytes.** `FUN_142907b60` serialises the HITINFO and then, at
   `142907c88`, calls `FUN_14025dcf0` on a **second** structure at `[rsp+0x20]` into the same
   packet. All three of our captures are exactly 147 and the mob builder emits exactly 147,
   so nothing above is affected - but a parser must not *assert* 147. See §9.

6. **`research/level-up.md` §6 - corroborated, not corrected.** Its `0x29e`-based table decode
   reproduces exactly; index 17 gives `0x02AF` as that file says, which is what makes index 7
   giving `0x02A5` trustworthy.

---

## 9. Wiring - this is NOT wired

Per `CLAUDE.md` § "Built is not wired", loudly:

* `net::stats::StatChange::hp_only` exists, is tested, and **nothing calls it.**
* Nothing in `crates/world/` reads `0x00E5`. The log line still says *"is not answered yet,
  and it is UNKNOWN"*.
* The decode above is in this file only; there is no Rust type for the 147-byte body.

To connect it, in `crates/world/src/session/` (the coordinator's file, deliberately not
touched here):

1. Read the first 147 bytes at fixed offsets - `damage = u32 @ +8`,
   `mob_object_id = u32 @ +46`, `mob_template_id = u32 @ +113` - and ignore the rest.
   **Require at least 147 bytes; do not require exactly 147.** `FUN_142907b60` appends a
   second structure after the HITINFO (§8.5), so a `==` length check would reject a body
   this client can legitimately send. All three captures are exactly 147.
2. `hp = hp.saturating_sub(damage)`.
3. Reply `0x007C` with `StatChange::hp_only(hp).build()`. **Always reply**, including when
   `damage` is `0`.
4. Optionally broadcast `0x02A5` (`u32 charId` + the 147 bytes echoed) to other players on
   the field. Not needed for a single-client test.

A sanity check that costs no launch: `damage` should be `1` and `mob_object_id` should be one
of the ids in the server's own spawn log.

---

## 10. Probe watches, if a run is ever spent on this

Not needed to answer anything above - all of it is static or from the existing capture.

| | what it would answer |
|---|---|
| `watch@1428aa0a0:args=4:hits=10` | confirms argument 2 is the damage at runtime, and that this is the builder |
| `watch@14299d9a0:args=2:hits=10` | `CharacterStat::SetHP`. Should fire **only** when a `0x007C` arrives. If it fires on a mob touch with no packet in flight, §4 is wrong |

The second is the real falsifier for §4 and it is cheap, because SetHP is not on any hot path.
