# Flag #7: the wire layout of `[0x140304e64, 0x140304fa6)`

The first gate of `FUN_140304b20`, the block that contains the character-stat decoder.
Written 2026-08-19 from text on disk plus the client binary itself. **No Ghidra was used.**

Tags: **[L]** read off a listing or a decompilation. **[D]** derived by analysis over one of
those. **[I]** inferred, not provable from what I had.

---

## 0. The instrument, checked before anything was read with it

The listing this document is built on is
`<scratchpad>/crfull.txt` (`0x140304b20 .. 0x14030941f`), left by the pass that wrote
`charrecord-loops.md`. It is not in `research/`, and `charrecord-reuse.md` correctly reported
that the body of `FUN_140304b20` "is not on disk in any form" — that was true of `research/`.

**[L] I re-assembled all 322 bytes of the region out of the listing and compared them
byte-for-byte against `client-patched/MapleStory.exe` through `tools/dump_va.py`:**

```
instructions=80  bytes_covered=322  region_size=322  mismatches=0
```

80 instructions, no address gaps, and every byte matches the file. So for this region the
listing is the binary. Two spot controls (`0x140304e49`, `0x140304e64`) were also dumped
directly and print the expected opcodes.

**[D] The gate is flag #7, by arithmetic, not by assumption.** I re-derived all 43 gates
mechanically from the listing (find `CALL 0x1402fa9a0`, walk back for `LEA R8,[key]`, forward
for the `JNZ` body and the `JMP` skip). The first one:

```
site 0x140304e49   key 0x143abee20   (key - 0x143abeb10) / 0x70 = 7 remainder 0
body 0x140304e64   skip 0x140304fa6
```

Exactly the region in the task. The second gate lands on index 8 with remainder 0 as well.

**[L] `FUN_1402fa9a0` (research/msexe-presence-gate.c:5) is `out[i] = presence[i] & key[i]`
for `i` in `0..100`** — two unrolled passes of 50 — and returns `out`. The caller then scans
`out` for any non-zero byte. That confirms the semantics the task states, off the
decompilation rather than by repetition.

**[D] The gate's `JNZ` at `0x140304e53` is the only way into the region.** I scanned the whole
4935-instruction listing for any branch from outside the region to an address inside it; that
`JNZ` is the single hit. The four indirect `JMP RAX` dispatches in this function have their
case bodies at `0x14030552b`, `0x140305e90`, `0x1403068a3`, `0x140306995` (**[L]**
`charrecord-loops.md` §0), all outside. So everything below is control-dependent on flag #7
and on nothing else.

---

## 1. The seven reads, in order

**[L]** Full listing of the region, reduced to what touches the stream:

| # | address | primitive | kind | wire cost | destination | condition |
|---|---|---|---|---|---|---|
| — | `0x140304e71` | `CALL 0x140302e30` | **nested stat decoder** | **108** | `param_1 + 0x00 .. +0x12a` | unconditional |
| 1 | `0x140304e79` | `0x1406e8ae0` | u8 | 1 | `dword [R12+0x118b]` (zero-extended) | unconditional |
| 2 | `0x140304e8c` | `0x1406e8ae0` | u8 | 1 | none — `TEST AL,AL / JZ 0x140304ee6` | unconditional |
| 3 | `0x140304e9c` | `0x1406e9050` | string | 2 + n | `qword [R12+0x1253]` | **only if #2 != 0** |
| 4 | `0x140304ee9` | `0x1406e8ae0` | u8 | 1 | none — `TEST AL,AL / JZ 0x140304f49` | unconditional |
| 5 | `0x140304efc` | `0x1406e9050` | string | 2 + n | `qword [R12+0x125b]` | **only if #4 != 0** |
| 6 | `0x140304f4c` | `0x1406e8ae0` | u8 | 1 | none — `TEST AL,AL / JZ 0x140304fa6` | unconditional |
| 7 | `0x140304f5c` | `0x1406e9050` | string | 2 + n | `qword [R12+0x1263]` | **only if #6 != 0** |

Four `u8`, three `string`. That matches `charrecord-loops.md` §6 rows 9-15 exactly.

**Where the nested call sits: 0 of the 7 before it, all 7 after.** **[L]** `0x140304e64` is
`MOV R8D,[RBP+0x3118]`, `0x140304e6b` `MOV RDX,R15`, `0x140304e6e` `MOV RCX,R12`,
`0x140304e71` `CALL 0x140302e30`. The call is the first instruction sequence in the region,
and nothing branches between it and read #1.

**[L] The three strings are an optional-string idiom, not three separate fields.** Each is a
`u8` flag whose zero value jumps past the string read; a non-zero value reads one string and
installs it in a `qword` slot, releasing whatever was there. The three slots are consecutive
(`+0x1253`, `+0x125b`, `+0x1263`).

**[L] The string primitive `0x1406e9050`** (`research/msexe-packet-readers.c:85`) reads a
`u16 n` at the cursor, throws unless `remaining >= n + 2`, and advances the cursor by `n + 2`.
So `n` is a **byte** count and an empty string costs 2 bytes (`00 00`).

**[L] Six other calls in the region — `0x14019f2c0` ×6 — do not touch the stream.** Each is
reached with `RCX = <string pointer> - 0x10`; the packet (`R15`) is never passed to them. They
are the string-release half of the idiom.

**[D] The decompiler agrees with the listing.** `<scratchpad>/record.c:377-425` renders the
same block: `FUN_140302e30`, then `FUN_1406e8ae0` into `param_1+0x118b`, then three
`if (FUN_1406e8ae0() != 0) { FUN_1406e9050(...) }` groups. Same count, same order, same
conditionality. This is the "field order from the listing, meaning from the decompiler"
cross-check, and for once the read count matched on the first pass.

---

## 2. Loops in the region: none

**[D] There is no loop inside `[0x140304e64, 0x140304fa6)`.** I enumerated every branch in the
region: 23 control-transfer instructions, of which 9 are `Jcc` and **all 9 are forward**. The
only backward transfers are `CALL`s to lower addresses (`0x140302e30`, `0x14019f2c0`), which
are calls, not back-edges.

This is corroborated independently: `charrecord-loops.md` §3 lists loop #4 at head
`0x140304e20` (before the region) and loop #5 at head `0x140305171` (after it), with nothing
between.

**So nothing in this region has a trip count, from the packet or otherwise.** The only
variability is the three optional strings, and each is controlled by its own preceding `u8`.

**[L] Inside the nested `FUN_140302e30` there is exactly one count-driven loop**: the
extended-SP table, `FUN_1402cb0d0` (`research/msexe-newchar-scan.c:1957`) — `u8 count`, then
`count x (u8 jobLevel, u32 sp)`. **Its count does come from the packet**, it is top-tested
(`if (bVar1 != 0)`), and a count of `0` costs one byte. It is only reached for the job set
below.

---

## 3. What `FUN_140302e30` consumes, and the argument that decides it

### 3a. `param_3` is `0` — settled two ways

**[L]** The call passes `R8D = dword [RBP+0x3118]`.

**[D] `[RBP+0x3118]` is `FUN_140304b20`'s 4th argument.** From the prologue: entry `RSP = E`;
`0x140304b20 MOV [RSP+0x20],R9D` spills `R9D` to its home slot at `E+0x20`; eight `PUSH`es
take `RSP` to `E-0x40`; `0x140304b31 LEA RBP,[RSP-0x30b8]` gives `RBP = E-0x30f8`; therefore
`RBP+0x3118 = E+0x20`, the `R9` home slot, i.e. `param_4`.

**[L] The decompiler says the same thing outright.** `<scratchpad>/record.c:239` is
`local_res20[0] = param_4;` and line 382 is `FUN_140302e30(param_1,param_3,local_res20[0]);`.
`local_res20` is Ghidra's name for the `E+0x20` home slot. Two instruments, one answer.

**[L] Every call site passes literal `0`:** `research/msexe-stage-setfield.c:496, 1892, 2028,
2161` and `research/msexe-gamestage-dispatch.c:1389` all read
`FUN_140304b20(..., ..., ..., 0)`.

**[D] So `FUN_140302e30` runs its `param_3 == 0` branch — the same branch the character-list
record uses** (`FUN_1403094b0` also passes `0`, `research/msexe-charrecord.c:384`). This
retires disagreement **D9** in `charrecord-reuse.md`: the client does *not* discard the
`characterId`; the identity at offset 0 is real on this path.

It would not have changed the byte count either way — **[L]** both branches of
`FUN_140302e30` read `u32,u32,u32,13B,u8,u8,u32,u32,u32` = 39 bytes — but it changes what the
fields mean, and that was open.

**[D] `R12 = param_1`.** `0x140304b67 MOV R12,RCX` and `0x140304e04 MOV R12,[RSP+0x40]`, where
`[RSP+0x40]` was written from `RCX` at `0x140304b6a`. `FUN_140302e30` therefore writes the
stat block at **offset 0 of the same object** whose `+0x118b` / `+0x1253` the region's own
reads fill. There is no header in front of the stat block *inside* this region.

### 3b. 108 bytes, and they are bytes we already build

**[L]** `research/msexe-charstats.c` gives the full body; `charrecord-reuse.md` §"Part 1" and
`docs/character.md` give the field names. **The 108 bytes were measured on the wire on
2026-08-17** — the same function decoded the same bytes inside `0x0010` and the client drew
the character. That measurement transfers here **[D]** because it is literally the same
function called with the same `param_3`.

Offsets below are absolute wire offsets in the minimal record of §4.

| off | field | type | bytes | note |
|---:|---|---|---:|---|
| 111 | `characterId` | u32 | 4 | **[L]** charstats.c:40 |
| 115 | `characterIdForLog` | u32 | 4 | **[L]** width; **[I]** name |
| 119 | `worldIdForLog` | u32 | 4 | **[L]** width; **[I]** name |
| 123 | `name` | 13B fixed | 13 | **[L]** `FUN_1406e9170(...,0xd)` — **not** length-prefixed |
| 136 | `gender` | u8 | 1 | |
| 137 | `skin` | u8 | 1 | |
| 138 | (zero) | u32 | 4 | |
| 142 | `face` | u32 | 4 | **[I]** name — reuse doc **D4** |
| 146 | `hair` | u32 | 4 | **[I]** name — **D4** |
| 150 | `level` | u32 | 4 | |
| 154 | `job` | u16 | 2 | **[L]** re-read at `+0x33` and bit-tested; decides the SP fork |
| 156 | `str` | u16 | 2 | |
| 158 | `dex` | u16 | 2 | |
| 160 | `int` | u16 | 2 | |
| 162 | `luk` | u16 | 2 | |
| 164 | `hp` | u32 | 4 | **[I]** which of the four — **D5** |
| 168 | `maxHp` | u32 | 4 | **[I]** — **D5** |
| 172 | `mp` | u32 | 4 | **[I]** — **D5** |
| 176 | `maxMp` | u32 | 4 | **[I]** — **D5** |
| 180 | `ap` | u16 | 2 | |
| 182 | **SP fork** | u8 or u16 | 1 or 2 | see below |
| 183 | `exp` | u64 | 8 | **[L]** `FUN_1406e8f10` |
| 191 | `fame` | u32 | 4 | |
| 195 | (unnamed) | u32 | 4 | |
| 199 | `portal` | u8 | 1 | **[I]** name — reuse doc **D6** |
| 200 | `subJob` | u16 | 2 | |
| 202 | (unnamed) | u8 | 1 | |
| 203 | FILETIME | 8B raw | 8 | |
| 211 | time, **high** dword | u32 | 4 | **[L]** high half first — reuse doc **D2** |
| 215 | time, **low** dword | u32 | 4 | |
| 219 | — end, 108 bytes | | | |

**The SP fork, at offset 182. [L]** `charstats.c:124-168`: the client decodes the `u16` it
stored at `+0x33` (the job) and takes the *extended* branch when the job is `0`, or in
`{100,110,111,112,120,121,122,130,131,132}` and the same shape at 200/300/400/500, plus
430-439. The extended branch reads **no `u16`**; it calls `FUN_1402cb0d0` = `u8 count` then
`count x (u8, u32)`. Everything else reads a plain `u16 sp`.

`job = 0` therefore costs **one** byte here (`count = 0`) and the stat block is **108**.
A non-listed job costs two and the block is **109**.

`crates::net::opcode::uses_extended_sp` (`opcode.rs:720`) already implements the fork, and the
**prefix** of `crates::net::opcode::character_record(&chr, world_id)` up to the end of the
stat block **is** this block, byte for byte. Mind the length: that prefix is `108` bytes when
`uses_extended_sp(chr.job)` and `109` when it is not, so slicing a hardcoded `[..108]` would
truncate a plain-`sp` job by one byte. Everything after that prefix in `character_record` is
`FUN_1403094b0`'s 24-byte trailer and the 195-byte avatar look, and **neither is on this
path** — do not append them.

---

## 4. The byte-exact minimal flag-#7 record

### 4a. A correction first: three head fields the task omitted

The task describes the head as "the 100-byte presence array, then field 2 (u8) and field 3
(u32)". **[L] There are three more mandatory reads between field 3 and the gate**, and leaving
them out desynchronises everything after:

* `0x140304c61` **u8** — loop #1's count. Guard `TEST EAX,EAX / JZ 0x140304c8f` skips the body.
* `0x140304c92` **u32** — loop #2's count. Guard `TEST EAX,EAX / JLE 0x140304ce8` skips it.
* `0x140304cf2` **u8** — a boolean. `TEST AL,AL / JZ 0x140304e38` on zero jumps straight to the
  flag-#7 gate, skipping `0x140304d02`, loop #3, `0x140304e11` and loop #4 in one branch.

This matches `charrecord-loops.md` §7, which I reproduced independently: I computed all 43
gate regions from the listing and listed every read outside all of them, then applied the
zero-count guards. Result: with all flags clear and all counts zero exactly seven reads fire,
totalling **112 bytes** — `100 + 1 + 4 + 1 + 4 + 1 + 1`. Same seven as §7.

### 4b. The recipe

**224 bytes.** Offsets are from the first byte of the character record (i.e. from the first
byte `FUN_140304b20` reads).

```
off  len  addr           what                                       value
---  ---  -------------  -----------------------------------------  ---------------------
  0  100  0x140304b95    presence array, field 1                    see 4c - NOT SETTLED
100    1  0x140304ba7    u8   -> dword [param_1+0x1001]             00
101    4  0x140304bba    u32  -> a duration added to a tick         00 00 00 00
105    1  0x140304c61    u8   loop #1 count                         00
106    4  0x140304c92    u32  loop #2 count                         00 00 00 00
110    1  0x140304cf2    u8   bool; 0 skips two more counts+loops   00
---  ---  --- gate #7 fires here (0x140304e49) ---
111  108  0x140302e30    the stat block, param_3 == 0               see 3b
219    1  0x140304e79    u8   -> dword [param_1+0x118b]             00
220    1  0x140304e8c    u8   optional-string flag A                00  (1 + "0000" also legal)
221    1  0x140304ee9    u8   optional-string flag B                00
222    1  0x140304f4c    u8   optional-string flag C                00
---  ---  --- gate #7 region ends (0x140304fa6); gates #8..#40 all skip ---
223    1  0x140308b3f    u8   ungated, after every gate              00
---  ---
224 bytes total
```

**[D]** With `job = 0` the stat block is 108 and the record is **224**. With a job that takes
the plain-`u16 sp` branch it is 109 and the record is **225**.

**[L] Byte values that are load-bearing rather than cosmetic:**

* offset 110 **must** be `00`. A non-zero byte there pulls in a `u32` count, a raw-8 loop,
  another `u32` count and a second raw-8 loop before the gate.
* offsets 105 and 106 **must** be zero counts. Both loops read from the packet inside their
  bodies (`0x140304c73` u32; `0x140304ca3` u32 + `0x140304cc6` raw 8).
* offsets 220/221/222 as `00` skip the three strings. Sending `01` followed by `00 00`
  (an empty string) is equally well-framed and costs 2 bytes each — **[D]** from the string
  primitive, which advances by `n + 2` with no lower bound on `n`.
* offset 154 (`job`) selects the SP encoding one byte later. Getting it wrong shifts every
  subsequent byte by one.
* the stat block's own field values: reuse what `crates/net` already sends
  (`hp = max_hp = 50`, `mp = max_mp = 5`, zeros elsewhere). Those exact bytes have been
  through this decoder on the wire.

### 4c. The 100-byte presence array — the part that is NOT settled

This is the one input the recipe cannot fix, and it is worth being blunt about.

**[L, verified negative] Key #7's 100 bytes cannot be read from disk.** The key table lives at
`0x143abeb10 + 7*0x70 = 0x143abee20`, in `.data`, and that address has **no file bytes** —
it is in the zero-filled tail of the section and is built at runtime. `tools/dump_va.py`
reports `no file bytes (uninitialised)`. The instrument is verified: the same tool, on the
same run, printed correct bytes for two `.text` control addresses that match the listing. So
this is an absence of data, not an absence of searching.

**[I, unproven] The obvious hypothesis is that key #k has byte k set**, which would make
`presence[7] = 1` fire flag #7 and nothing else. It is plausible — the stride is 1:1 with the
index, the array length and the scan bound are both 100 — and `charrecord-loops.md` §5 flags
it the same way. **It is still unproven, and the key contents are the only thing that could
prove it.**

Three consequences, all **[D]** from `out[i] = presence[i] & key[i]`:

1. **Use `0xFF`, not `0x01`.** The gate is a bitwise AND. If key7[7] were, say, `0x02`, then
   `0x01 & 0x02 == 0` and the block would silently not run. `0xFF` fires on any non-zero key
   byte. There is no downside: the scan only asks "non-zero".
2. **A single set byte can only ever fire a subset of the gates.** Setting only byte 7 fires
   gate k iff `key_k[7] != 0`. So the failure modes are asymmetric: too few blocks run (the
   record is too long, the client reads past its end into the next field) or extra blocks run
   (the record is too short, the reader throws).
3. **All-`0x01`, which the reference server sends, fires every gate** and therefore demands
   the full 126-read record. It is not a shortcut.

**[L]** The client's readers throw `_CxxThrowException` on underrun (`0x1406e9050` and its
siblings), so a wrong presence array is an exception inside the handler, not a visual
artefact — the same failure shape `docs/character.md` warns about.

---

## 5. What I could not pin down

Stated as gaps, not filled with guesses.

1. **Which presence byte turns flag #7 on.** §4c. This is the single unknown that stands
   between this document and a record that can be sent. Static analysis cannot answer it;
   the key table is runtime-initialised. It needs either a read of the running process
   (the in-process probe already dumps memory) or a one-byte-at-a-time client experiment.
2. **Whether other keys also select the same byte.** Same cause, same fix.
3. **What the `u8` at `0x140304e79` means.** **[L]** It goes to `dword [param_1+0x118b]`,
   zero-extended. No consumer of that offset was read. Its width and position are certain;
   its meaning is not.
4. **What the three optional strings are.** **[L]** `+0x1253`, `+0x125b`, `+0x1263`, three
   consecutive `qword` slots. No consumer read. I have no basis even for candidate names, and
   the reference server is a different version, so I am not offering any.
5. **`face` / `hair` at offsets 142/146, and which of the four obfuscated `u32`s is `hp` vs
   `maxHp`.** Carried over unchanged from `charrecord-reuse.md` **D4** and **D5** — reference
   names, never discriminated against this binary. Invisible in the character list; a
   `SetField` is exactly the situation that would expose them.
6. **The stat block's field *values* for a live character.** The 108-byte *layout* is
   measured. That it is safe to put `hp = 0` or `level = 0` into a character standing in a
   field is a different question and nothing on disk answers it.

---

## 6. One premise in the task does not survive the read

> "That block is almost certainly where the map id lives, so it is the block we must switch on."

**[D] There is no map id in this block.** It is the 108-byte stat block, one `u8`, and three
optional strings — nothing else. And the stat block has no map field: in the character-list
path `chr.map_id` sits at record offset 120, which belongs to **`FUN_1403094b0`'s** trailer
(`crates/net/src/opcode.rs:845`, and `charrecord-reuse.md` **D3** notes that placement was
never evidenced either). `FUN_1403094b0` is not on the `SetField` path at all.

**[L] A better candidate is three `u32`s the *caller* reads immediately before invoking the
record decoder.** `research/msexe-stage-setfield.c:482-496`, the `characterData != 0` branch:

```c
uVar8  = FUN_1406e8c20();          // u32
uVar10 = FUN_1406e8c20(param_2);   // u32
uVar11 = FUN_1406e8c20(local_3e0); // u32
uVar17 = FUN_142cbef90(lVar24);
FUN_14025ea90(uVar17, uVar8, uVar10, uVar11);
FUN_142ce5170(lVar24, uVar8, uVar10, uVar11);
...
FUN_140304b20(uVar33, local_b8, local_3e0, 0);
```

Three `u32`s handed as a triple to two world-object calls, immediately before the character
record. **[I]** That is the shape a `(fieldId, portal, ...)` triple would have, and it is
where I would look next — but I did not read `FUN_14025ea90` or `FUN_142ce5170`, so this is a
pointer, not a finding.

Flag #7 is still worth switching on: it is what carries the character's stats, and **[L]**
`0x140304e71` is the only call to `FUN_140302e30` in the whole 18525-byte function. Without
it the client gets no stat block at all.
