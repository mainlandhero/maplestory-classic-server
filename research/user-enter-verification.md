# `0x0224` / `0x0225` — verification pass over `research/user-enter-field.md` and the builder

**2026-08-31. Static only — no client run, no Ghidra (the project was held by siblings).**
Everything below comes from `tools/listing.py`, `tools/reads.py`, `tools/callers.py`,
`tools/dump_va.py`, `tools/pdata_lookup.py` against the PE, from the pre-made dumps in
`research/`, and from the archived logs. All tooling run with the repo as the working
directory.

Tags: **[L]** read off a listing, raw bytes, or a log; **[D]** derived from two or more [L];
**[I]** inferred.

---

## 0. The answer, before the working

* **The `0x0224` body is SEVEN BYTES SHORT.** `net::userpool::user_enter_field` builds 508 +
  name + 5·equips. `CUser::Init` reads **515** + the same. The gap is entirely inside the
  remote temporary-stat block: `FUN_140a46e50` reads the 124-byte mask **and then four more
  fields unconditionally** — `u8` at `0x140a4a007`, `u8` at `0x140a4a024`, `u32` at
  `0x140a4a041`, `u8` at `0x140a4a29e` — **131 bytes, not 124**. Established three
  independent ways (§1.3). **[L]**
  * **The fourth read was challenged and is settled: the fix is 515, not 514.** The first
    three are visible in raw bytes; `0x140a4a29e` is not, and it sits among six *gated*
    neighbours. §1.3a proves it with a fourth instrument that needs no CFG — no jump in the
    whole function has a source below it and a target above it, all 186 jumps have literal
    targets, there is one `ret` (after it) and one entry point. The `je` at `0x140a4a240`
    that gates three of the neighbours lands **two instructions before** it. Six of the
    fourteen short-Jcc *bytes* in that stretch are not instructions at all. **[L]**
* **This makes T1 outcome (b) the *predicted* outcome, not a fear.** The client will consume
  our 508 bytes and then want four more for the `u32` at its offset 507, with **one byte
  left**, and raise inside `FUN_1429ce270`. The throwing instruction is **`0x1429cfd4d`**;
  the return address in the ELog stack would be `0x1429cfd52`. **[D]**
* **How the 124 became 508:** `research/msexe-secondarystat-remote-140a46e50.txt` is
  **truncated at 3 945 bytes** — it covers `0x140a46e50..0x140a46ff7` of a **14 143-byte**
  function and stops 14 KB before the tail. Its own index line calls it *"same mask, short
  list"*. The short list was the dump, not the function. This is `CLAUDE.md`'s "an instrument
  that is silently the wrong version answers short, clean and confident", in a `research/`
  file rather than in `tools/`. **[L]**
* **The rest of `user-enter-field.md` §2.2 is correct.** Its 58 direct unconditional reads in
  `FUN_1429ce270` reproduce **exactly** under an independent dominator analysis, and every
  wire-gated block really is skipped by the zero we send (§2.4). The one error is in a
  *callee*, which is the class of error a direct-read enumeration structurally cannot see.
* **`0x0225` is confirmed: exactly one `u32`, unconditional, and the packet pointer is dead
  after it.** The builder is right. A leave for an id not in the pool is a silent no-op. **[L]**
* **The avatar look is confirmed byte-for-byte: 195 + 5 per equip, at the offset the client
  reads.** It is the *same function* `character_record` already puts through the same reader
  on the character-select screen, so these bytes have been accepted on screen. No two builders
  disagree. **[L]**
* **`FUN_142833030` — `user-enter-field.md` §6 blind spot 2 — is closed.** It is
  unconditional and does take the packet, but it hands it to a virtual only when
  `user+0x3b90 != 0`; the base constructor writes `0` there (`0x142769c9c`, with `rbx` zeroed
  at `0x1427694a3`), and with the `u32` we send at that field set to `0` it returns before
  reaching the hand-off. **Keep that field zero.** **[D]**
* **Two wiring defects that are not length bugs:**
  `Bus::refresh_spawn` has **zero call sites** — a player's spawn is frozen at their entry
  position, level and look, so a late arrival sees them at the map origin however far they
  have walked. And **entering the Cash Shop publishes no leave**, so a shopper stays visible
  to everyone while their client receives field packets in the wrong stage (§5.3).
* **Every departure by warp depends on the client volunteering `0x00DC`.** Across the archive:
  **273 `0x01A0` SetFields, 268 `0x00DC`s** — deduplicated on `(time, direction, opcode)`
  across `previous-runs/` and `research/fixtures/`. **[L]** `0x0224` and `0x0225` appear
  **zero** times, which is the brief's claim measured rather than repeated.

---

## 1. Instruments, and the control each one passed first

### 1.1 The repo tools, not the scratchpad copies

`tools/reads.py` is 9 605 bytes and contains `0x142d23ef0` twice — the tenth read primitive
`CLAUDE.md` names. Its documented control, `python tools/reads.py 0x140304100 2`, printed the
equipped-item decoder reading **through `FUN_1403035a0` and `FUN_140303b40` and directly** —
a mix of both kinds, which is what that control exists to show. `tools/listing.py 0x140304100`
printed its own `READ raw / u8 / u8 / u16…` control. Both passed. **[L]**

### 1.2 A dominator analysis, and its control

`uncond.py` (scratchpad) builds a CFG from a `tools/listing.py` dump and reports which reads
**dominate every exit** — i.e. execute on every path. Run on `FUN_1402ee8d0` (the avatar look,
whose shape is already known from `research/avatar-look-reader.c`) it reproduces the known
answer exactly: the 19-byte head unconditional, **both** `0xFF` slot bytes unconditional, both
equip-loop bodies gated, the 174-byte tail unconditional. That is a positive control on a
function whose answer was established by other means.

### 1.3 The 131-byte finding, three ways, deliberately not three of the same way

`CLAUDE.md`: *"two scans agreeing is not corroboration when they share a blind spot."* So:

1. **Forced-value reachability** (`zeroreach.py`): force every `je` that follows
   `call 0x1402bf6d0` (the mask bit test) to be taken, explore every other branch both ways.
   **5 reads reachable.** Control: with no forcing the same walker reaches all **327**.
2. **Dominator analysis** (`uncond.py`, a different algorithm on the same listing):
   **the same 5**, all `UNCOND`.
3. **Raw bytes**, which do not go through the disassembler at all —
   `python tools/dump_va.py 0x140a49fff 96`:

```text
000140a49fff  48 8b 8c 24 90 01 00 00   mov rcx,[rsp+0x190]      ; the packet
000140a4a007  e8 d4 ea c9 ff            call 0x1406e8ae0         ; READ u8
000140a4a00f  48 8b 8c 24 80 01 00 00   mov rcx,[rsp+0x180]
000140a4a017  e8 94 d7 e4 ff            call 0x1408977b0
000140a4a01f  ...  e8 b7 ea c9 ff       call 0x1406e8ae0         ; READ u8
000140a4a02f  ...  e8 27 d8 e4 ff       call 0x140897860
000140a4a03f  ...  e8 da eb c9 ff       call 0x1406e8c20         ; READ u32
000140a4a04f  ...  e8 5b 6f e5 ff       call 0x1408a0fb0
000140a4a055  ba 7d 01 00 00            mov edx,0x17d            ; the next bit test
```

There is **no conditional-jump byte anywhere between `0x140a49fff` and `0x140a4a055`**, and
`0x140a49fff` is the `je` target of the preceding bit test, so it is reached by fall-through
*and* by the branch. The fifth read, `0x140a4a29e`, sits at the same kind of join.

### 1.3a The seventh byte, challenged and settled — by a third question, not a third run

The coordinator reproduced bytes 179-184 independently and **could not confirm the fourth
read, `0x140a4a29e`**. Two objections, both fair:

1. a raw scan finds **14 short-Jcc bytes** between `0x140a4a055` and `0x140a4a29e`, so the
   region is not visibly straight-line;
2. `tools/reads.py` shows **six neighbouring reads** in that same stretch — `0x140a4a218`,
   `24a`, `266`, `282`, `2d3`, `2ef` — and prints `gated?` on all of them without deciding.
   *Why would one of seven be unconditional?*

The right answer is not to re-run the dominator analysis. It is to ask a **different
question that needs no CFG at all**:

> An instruction at `A` is bypassed **iff** some jump at `B < A` has a target `T > A`, or
> some `ret` sits at `R < A`. If neither exists, every path that enters the function
> executes `A`.

That is a flat scan over the instruction list, reproducible from `tools/listing.py` output in
five minutes. Its three preconditions are each proven for this function rather than assumed:

* **No indirect jumps.** 186 jump instructions, and **186 of 186 have a literal target**. A
  `jmp rax` or a jump table would be invisible to the scan; there are none. **[L]**
* **One exit, and it is after the reads.** Exactly one `ret`, at `0x140a4a58e`. Nothing can
  return early. **[L]**
* **One entry.** `tools/callers.py 0x140a46e50`: 2 call sites, **0 tail-jmp sites, 0 qword
  pointers** in the image. The function is never entered anywhere but its first instruction,
  so "every path from the entry" is the only kind of path there is. **[L]**

Result, and it is the discriminator that was asked for:

```text
  0x140a4a007  UNCONDITIONAL   no jump in the function has source < A and target > A
  0x140a4a024  UNCONDITIONAL   ditto
  0x140a4a041  UNCONDITIONAL   ditto
  0x140a4a29e  UNCONDITIONAL   ditto          <-- the seventh byte
  0x140a4a218  SKIPPABLE       0x140a4a20e je -> 0x140a4a22c
  0x140a4a24a  SKIPPABLE       0x140a4a240 je -> 0x140a4a296
  0x140a4a266  SKIPPABLE       0x140a4a240 je -> 0x140a4a296
  0x140a4a282  SKIPPABLE       0x140a4a240 je -> 0x140a4a296
  0x140a4a2d3  SKIPPABLE       0x140a4a2c9 je -> 0x140a4a303
  0x140a4a2ef  SKIPPABLE       0x140a4a2c9 je -> 0x140a4a303
```

**The branch that gates three of the neighbours lands two instructions before the read in
question.** `0x140a4a240 je -> 0x140a4a296`; `0x140a4a296` is `mov rcx,[rsp+0x190]`, and the
very next instruction is `0x140a4a29e call 0x1406e8ae0`. The gated block and the
unconditional read are adjacent, which is exactly why the two look alike in a `reads.py`
dump and are not alike at all. The other gate, `0x140a4a2c9`, starts *after* `0x29e` and so
cannot reach it.

**Control for the scan**, on a function whose answer was established by other means: run on
`FUN_1402ee8d0`, `0x1402ee8f3` (head) and `0x1402eeada` (tail) come back UNCONDITIONAL and
the two equip-loop bodies `0x1402ee9b6` / `0x1402eea06` come back SKIPPABLE, naming the exact
`je` that skips each. It discriminates.

#### The 14 short-Jcc bytes: eight are real, six are not

Checked against instruction starts rather than against bytes — which is the hazard the
objection itself named:

```text
  0x140a4a056  NOT an instruction - the 0x7d inside `mov edx, 0x17d`
  0x140a4a069  je  -> 0x140a4a087
  0x140a4a09b  je  -> 0x140a4a0bc
  0x140a4a0b9  NOT an instruction - inside `call 0x1408a14b0`
  0x140a4a0d0  je  -> 0x140a4a0ee
  0x140a4a13e  NOT an instruction - inside `cmp [rsp+0x24], 8`
  0x140a4a142  jge -> 0x140a4a1ab      the `for (i=0; i<8; i++)` loop exit
  0x140a4a16f  je  -> 0x140a4a1a9
  0x140a4a1dc  je  -> 0x140a4a1fa
  0x140a4a1f7  NOT an instruction - inside `call 0x1408a1460`
  0x140a4a20e  je  -> 0x140a4a22c
  0x140a4a229  NOT an instruction - inside `call 0x1408a1320`
  0x140a4a240  je  -> 0x140a4a296
  0x140a4a25b  NOT an instruction - inside `call 0x1408a13c0`
```

Plus two the byte scan does not look for: `0x140a4a131 jmp -> 0x140a4a13d` and
`0x140a4a1a9 jmp -> 0x140a4a133`, the loop's entry and back-edge.

**Ten real branches in the span, and the largest target of any of them is `0x140a4a296`.**
Not one jumps past `0x140a4a29e`.

#### The encoder does not exist in this binary, and that is a bounded negative

`CSecondaryStat::EncodeForRemote` would be the ideal corroboration and **it is not here.**
Enumerating every `mov r8d, 0x7c` in all of `.text` whose next `E8 rel32` reaches a wire
primitive gives **three sites**, with the known decoder site present as the asserted control:

```text
  0x140a16646  READ  raw   in FUN_140a165f0    DecodeForLocal
  0x140a46e95  READ  raw   in FUN_140a46e50    DecodeForRemote   <- the control
  0x142973365  WRITE raw   in FUN_142973160    u32, u32, u8, raw[124], send
```

The one writer, `FUN_142973160`, has **zero** calls to the mask bit test `0x1402bf6d0` — it
blits a mask it already holds and sends — and writes **no tail** after the raw block. It is
the client's outbound cancel, the mirror of `0x007E`, not the mirror of what
`FUN_140a46e50` reads. That is what one should expect: this is a client, and only a server
ever *writes* a remote user's stat block.

**The bound on that negative, stated rather than implied:** the scan finds a 124-byte wire
move only when the length is an **immediate in `r8d`**. An encoder that loaded the length
from a register or a field, or that wrote the mask as 31 separate `u32`s, would not appear.
So this is "no encoder with an immediate-length mask write", not "no encoder". Given that,
the encoder corroboration cannot be had from this binary and **I am not substituting a
weaker instrument for it.** The seventh byte rests on §1.3a, not on the forced-value walk.

**Verdict: 515, not 514.** The forced-value walk was the first sighting and would not have
been enough on its own; the skip-over argument is what settles it, and its three
preconditions are measured rather than assumed.

### 1.4 The mask really is 124, and that is now three readings too

* `0x140a46e82 mov r8d, 0x7c` — the raw read length. **[L]**
* `FUN_1402bf6d0` (the bit test, dumped raw): `cmp edx, 0x3e0 / jb` — **992 bits = 124 bytes**,
  and out-of-range returns 0. **[L]**
* `FUN_14080fa00` (the loop gate, dumped raw): tests 31 dwords plus one more = `0x7c` bytes.
  **[L]**

So `REMOTE_STAT_MASK_LEN = 124` was never wrong. What was wrong is that the **block** is not
the mask.

### 1.5 The negative that mattered, and why it is not a property of the search

`pktcalls.py` enumerates every call in `FUN_1429ce270` that **receives the packet pointer**,
with whether it is unconditional. That negative is only worth anything if the packet register
cannot escape the pattern it looks for. It cannot: grepping the Init listing for `r12` shows
**every** use is `mov r12, rdx` (once), `mov rcx/rdx/r8/r9, r12`, or `push`/`pop`. It is never
copied to another register, never spilled to the stack, and `r12d` does not appear at all.
**[L]** The enumeration is therefore complete for this function.

Result — seven unconditional packet-taking calls, and nothing else:

```text
1429ce4e4  UNCOND  FUN_140a46e50   temp stat     131  <-- was counted as 124
1429ce6a9  UNCOND  FUN_1402ee8d0   avatar look   195 + 5/equip
1429cf0db  UNCOND  FUN_14073a7b0                  18
1429cfbb5  UNCOND  FUN_142833030                   0  (see §2.3)
1429cfd2f  UNCOND  FUN_142834df0                   4  (count, entries gated)
1429cfd3a  UNCOND  FUN_142835840 -> FUN_1408cf0d0   5
1429cfd45  UNCOND  FUN_1428358a0                    4
```

Each of the six that read was itself put through the dominator tool. `FUN_14073a7b0`'s five
reads are **all** unconditional (`raw4, u32, raw4, str, u32` = 18); `FUN_142834df0` reads its
count unconditionally and its entries gated; `FUN_1408cf0d0` reads `u8, u32` unconditionally;
`FUN_1428358a0` reads one `u32`. All **[L]**, all agreeing with §2.2.

### 1.6 Independent corroboration that "a temp-stat block has an unconditional tail"

Two, from code nobody touched for this pass:

* **`crates/net/src/buff.rs`** already records the identical class of error in the *sibling*
  decoder: `research/buffs.md` §7.1 listed `0x007E` as `u8,u8,u8,raw[124]` = 127, and the
  client **crashed**, with a throw stack naming the `u8` primitive's raise path, because
  `FUN_142d56f80` reads **two more `u8`s after the mask** that the field list did not mention.
  Same shape, same cause, already paid for once. **[L]**
* **`FUN_140a165f0`**, the LOCAL sibling of `FUN_140a46e50`, run through the same dominator
  tool: `raw(mask)`, then `u16 @140a456f8`, `u8 @140a457fa`, `u8 @140a45817`, `u32 @140a45834`
  — **an unconditional tail there too**, of a different width. **[L]** Two decoders in this
  family, both with a tail; the enumeration that produced 508 saw neither.

*This does not touch any builder in the repo today* — `0x007D`/`0x007E` go to
`FUN_142d563d0`/`FUN_142d56f80`, not to `FUN_140a165f0` — but it is where to look next if a
local buff packet is ever found short.

---

## 2. Question 1 — field by field, and the two numbers

### 2.1 The counts asked for

| | |
|---|---|
| direct reads in `FUN_1429ce270`, depth 3 | **86** (27 `u8`, 8 `u16`, 37 `u32`, 8 `str`, 6 `raw`) |
| — of those, **unconditional** | **58** |
| unconditional non-primitive calls that take the packet | **7** (six of which read) |
| `PacketWriter` calls in `net::userpool::user_enter_field` | **72** — counted by script, not by eye: 3 header `u32`, 68 body fields, 1 `w.bytes(&avatar_look(chr))`. By kind: 32 `u32`, 21 `u8`, 6 `str`, 4 `u16`, 4 `i16`, 4 `zeros`, 1 `bytes` |
| bytes the builder emits, empty name, no equips | **508** |
| bytes `FUN_1429ba3e0` + `FUN_1429ce270` read | **515** |

The 86/58 figures reproduce `user-enter-field.md` §6's read count *and its type split*
exactly, from a different instrument. The prior pass's enumeration of Init itself is right;
the defect is one level down.

### 2.2 The corrected minimum body

Summed by script, not by hand (`offsets.py`). Only the rows that move are annotated.

```text
   0   4  1429ba40a  u32 userId
   4   4  1429ba43b  u32 charId          NONZERO or a fourth u32 is read
   8   4  1429ba4a0  u32 fieldCheck
  12   4  1429ce310  u32 level
  16   2  1429ce323  str name            (+len)
  18   2  1429ce371  str parentName
  20   4  1429ce3bb  u32 guildId
  24   2  1429ce3ce  str guildName
  26 2+1+2+1        guild logo bg / colour / logo / colour
  32   4+4          guild block tail
  40   1  1429ce474  u8  gender
  41   4  1429ce483  u32 fame
  45   4  1429ce492  u32 nameTagMark
  49   1  1429ce4a1  u8
  50   4  1429ce4b3  u32
  54   1  1429ce4c2  u8
  55 124  140a46e95  raw  the temporary-stat MASK
 179   1  140a4a007  u8   \
 180   1  140a4a024  u8    |  THE SEVEN MISSING BYTES
 181   4  140a4a041  u32   |  all four unconditional, mask-independent
 185   1  140a4a29e  u8   /
 186   2  1429ce4ec  u16 job          <- was 179
 188   2  1429ce4fe  u16 subJob
 190   4  1429ce510  u32
 194 195  1402ee8d0  AVATAR LOOK      <- was 187;  195 + 5 per equip
 389   4  1429ce6b2  u32 driverId
 393   4  1429ce6c1  u32 passengerId
 397   1  1429ce6da  u8
 398   4  1429ce6ea  u32
 402   4  1429ce6f4  u32
 406   4  1429ce6fe  u32
 410   1  1429ce70a  u8  0 -> no trailing string
 411   4  1429ce764  u32 damageSkin
 415   2  1429ce777  str
 417   2  1429ce7c5  str
 419   4  1429ce80f  u32
 423   2  1429ce81e  i16 fieldSeat
 425   4  1429ce830  u32 chairItemId
 429   4  1429ce843  u32
 433   2  1429ce852  i16 X            <- was 426
 435   2  1429ce85f  i16 Y
 437   1  1429ce86c  u8  moveAction
 438   2  1429ce885  i16 foothold
 440   1  1429ce89c  u8
 441   1  1429ce8b1  u8
 442   1  1429cea87  u8  0 -> no chair object
 443   1  1429cec20  u8  0 -> no pets
 444   1  1429cec9d  u8  0 -> no familiars
 445  4+4+4         taming-mob level / exp / fatigue
 457   1  1429ced26  u8  0
 458   4  1429cee29  raw4 0 -> NO MINIROOM        <- was 451
 462  18  14073a7b0  raw4, u32, raw4, str, u32    unconditional
 480   1  1429cf14a  u8  0 -> no trailing string
 481   1  1429cf9f8  u8  0 couple ring
 482   1  1429cfa52  u8  0 friendship ring
 483   1  1429cfaac  u8  0 marriage ring
 484   1  1429cfafe  u8  0
 485   1  1429cfb38  u8  0 bitmask (bit 0x20 would add a u32)
 486   4  1429cfb96  u32
 490   4  1429cfba8  u32 MUST BE ZERO - see §2.3
 494   4  142834e3d  u32 count = 0
 498   1  1408cf0e3  u8
 499   4  1408cf0ed  u32
 503   4  1428358ac  u32
 507   4  1429cfd4d  u32 count = 0     <-- WHERE A 508-BYTE BODY RAISES
 511   4  1429cff0e  u32 count = 0
 515                 END
```

**515 with an empty name and no equips; 515 + name + 5·equips otherwise.** With an
8-character name and 5 equips: **548**.

### 2.3 The one field whose value is load-bearing beyond its own width

Body offset **490** feeds `FUN_142833030(user, v, pkt)` at `0x1429cfbb5`, which is
unconditional and does receive the packet. Reading it:

```text
142833055  r15 = [global];  if (r15 == 0) return            no read
142833065  rax = [user+0x3b90]
14283306f  if (rax != 0) goto 14283307e                     -> can reach the hand-off
142833071  if (edx == 0)  return                            no read   <-- our zero lands here
...
1428332a1  mov r8, r13 (the packet) / call rbp              a VIRTUAL - blind spot
```

`user+0x3b90` is written **`0`** by the base constructor at `0x142769c9c`, with `rbx` zeroed
at `0x1427694a3` and never reloaded in between. **[L]** So on a `CUser` that
`FUN_1429cdb70` has just built, a zero at offset 490 takes the `return` and the virtual is
never reached. **A nonzero there opens an indirect decode nothing static here can measure.**
**[D]** — this closes `user-enter-field.md` §6's blind spot 2 *conditionally*, and the
condition is "send zero".

### 2.4 Every wire-gated block, and the polarity of its gate

Checked one by one in the listing; all are `test al,al / je <skip>` or `test eax,eax / je`,
so **a zero byte skips**:

`1429ce70a` (410), `1429cea87` (442), `1429cec20` (443), `1429cec9d` (444), `1429ced26` (457),
`1429cf14a` (480), `1429cf9f8` (481), `1429cfa52` (482), `1429cfaac` (483), `1429cfafe` (484).
The counts at `1429cfafe`, `1429cfd4d`, `1429cff0e` and `1429cfc0a` are `test eax,eax / jle`,
so zero also skips. The `raw4` at `1429cee29` is `test eax,eax / je` on the dword. The bitmask
at `1429cfb38` is `test bl,2` and separately bit `0x20`. **All [L].** The builder's zeros are
right everywhere; only the *offsets* move.

The one gate that is **not** on the wire is unchanged from §3 of the prior document:
`0x1429cfc0a` reads a `u32` count if the *look we just sent* carries a taming-mob/vehicle
item id. Send no mount and it is absent.

---

## 3. Question 2 — the avatar look

**Confirmed, and there is no second builder to disagree with.**

`FUN_1402ee8d0` under the dominator tool (§1.2): 19-byte head (`u8 gender`, `u8 skin`,
`u32`, `u32 face`, `u32 job`→`+0x1bd`, `u8` discarded, `u32 hair`→`+0x39`), then
`u8 slot; while slot != 0xFF { u32 itemId; u8 slot }` **twice**, then a 174-byte
unconditional tail (`5×u32`, `u8`, `u32`, `raw4`, `raw128`, `u32`, `raw13`).
**195 + 5 per equipped item.** **[L]**

* `crates/net/src/opcode.rs::avatar_look` matches that field for field, including the two
  `0xFF` terminators (all equips in the first map, none in the second).
* **`character_record` — the character-list / `SetField` path — calls the SAME
  `avatar_look(chr)`** (`opcode.rs:1191`). These exact bytes have already been through this
  exact reader on screen. There is no disagreement between builders to adjudicate.
* The equipment source is real: `Session::claimed_character` loads `chr.equips` from the store
  before `presence()` builds the packet, so a remote player is dressed, not naked.
* Harmless detail worth knowing: the client validates `slot-1 <= 0x1e` (`0x1402ee9c0`) and
  **discards** an out-of-range slot's item — but still consumes its 5 bytes, so a bad slot
  costs a missing garment, never a desync. **[L]**
* Unchanged caveat: the `u32` at `+0x1bd` inside the look is the job, and the `u16` at body
  offset **186** is a *separate* job field on the `CUser`. Both are sent.

---

## 4. Question 3 — which of the six gates can fire, and what each looks like

All six predicates dumped as raw bytes. Four of them are two-instruction getters, which is
what makes this table worth having.

| # | at | predicate | what it really reads | fires when | on screen |
|---|---|---|---|---|---|
| 1 | `1429ba4ba` | `charId == FUN_142cb9550(ctx)` | `mov eax,[rcx+0x232c]; ret` → **`ctx+0x232c`**, the local character id | we send someone their own id | nothing appears; one dword zeroed; body **not read** |
| 2 | `1429ba546` | hash hit in `pool+0xf8` | — | a second `0x0224` for an id already in the pool | nothing; body **not read**; `add rax,0x18 / jne` is always taken |
| 3 | `1429ba563` | `FUN_142cc3d80(ctx) != 0` | `mov eax,[rcx+0x24b4]; ret` → **`ctx+0x24b4`** | unknown | nothing at all |
| 4 | `1429ba582` | `FUN_141bc8c60(field)` | `mov rax,[rcx+0x68]; movzx eax,[rax+0x481]` → **`field->[0x68]+0x481`** | unknown | nothing at all |
| 5 | `1429ba5a1` | `FUN_141bc8c80(field)` → then `FUN_142cc03a0(ctx)==0` or `FUN_142dec860(ctx,id)==0` | same struct, **`+0x482`** | unknown | nothing at all |
| 6 | `1429ba5e6` | `FUN_141bc8ca0(field)` → then `FUN_142cc0400(ctx)==0` or `!= body field 2` | same struct, **`+0x483`**; `FUN_142cc0400` is `mov eax,[rcx+0x23e8]; ret` | unknown | nothing at all |

Two things this dump adds that the prior document could not say:

* **Gates 4, 5 and 6 are three consecutive bytes of one structure** — `field->[0x68] + 0x481`,
  `+0x482`, `+0x483`, read by three getters that differ only in the displacement. They are
  three flags of one field-mode block, not three unrelated tests. **[L]** on the bytes,
  **[D]** on "one block". `FUN_1428e1fa0` calls all three plus gate 3, which is a second,
  independent sighting of the same set.
* **`FUN_142cc0400` (`ctx+0x23e8`) has exactly one caller in the entire image: this handler.**
  So header field 2 exists solely to be compared against that one context dword under gate 6.
  Sending `0` is safe unless gate 6's byte is set — and if it is, `FUN_142cc0400` returning
  `0` drops the packet before the comparison anyway. **[L]**
* `FUN_141883ea0`, on the self-id path, is `32 c0 c3` = `xor al,al; ret` — **always 0**,
  exactly as `user-enter-field.md` §5 said. Re-confirmed from bytes. **[L]**

**Discriminator, one watch, one run:** `0x1429ba60b` (`mov edx,0x40`, the ZRef allocation)
sits past all six. If it fires, no gate is involved and the problem is the body. If it never
fires while a `0x0224` was sent, it is one of the six and no body work will help. Positive
control for the watch slot budget: `140304100:hits=200`, per `user-chat-round2.md` §9.

---

## 5. Question 4 — `0x0225`, and every way a player can leave

### 5.1 The body: confirmed

`FUN_1429ba980` — one `u32` at `0x1429ba9a2`, `UNCOND`, and **that is the only read in the
function**. Better than a register scan: the packet arrives in `rdx`, is consumed as
`mov rcx, rdx` at `0x1429ba99f`, and is **never saved anywhere** — `rcx`/`rdx` are volatile,
so after that call nothing in the function can reach the packet at all. **[L]**
`net::userpool::user_leave_field` is correct at 4 bytes.

An id that is not in the hash falls through to `je 0x1429baf82`, the epilogue: **a leave for
somebody who was never there is a silent no-op.** **[L]** That makes `Bus::part`'s
idempotency free.

### 5.2 Every route out of a map

| route | code | publishes a leave? |
|---|---|---|
| **portal / warp / `!map` / NPC warp** | `go_to_map` sends only a `SetField`; the leave happens when the client's `0x00DC` reaches `on_field_entered` → `announce_field_entry` → `Bus::enter_field`, which posts the **old** farewell as its first act | **yes, but only via the client's `0x00DC`** |
| **channel change** | `session/field.rs:423` `leave_the_field()` before the migrate sweep | yes |
| **log out** | `session/field.rs:507` `leave_the_field()` | yes |
| **disconnect / crash / kill** | `Drop for Session` → `Bus::part` | yes; the only path that always runs |
| **death** | nothing — a dead character is still on the map | correctly no |
| **revive** | `revive()` → `go_to_map(target)`, same as a warp. If `revive_field` has no row, `target == died_on` and `Bus::enter_field` still does a full leave-and-re-enter of the same map, which is right because the client tears its pool down on any `SetField` | yes, via `0x00DC` |
| **Cash Shop enter** | `on_cash_shop_request` sets `in_cash_shop = true` and sends `0x01A3`. **No `leave_the_field`.** | **NO** |
| **Cash Shop exit** | `on_cash_shop_exit` → `go_to_map(same map)` → `SetField` → `0x00DC` | yes (leave then enter, so the shopper blinks) |

### 5.3 The two defects this table exposes

**(a) The Cash Shop ghost, and something worse than a ghost.** A player in the shop keeps
their `Presence`, so they stay in everyone else's `here` list and on everyone else's screen.
That alone is the "vanishes from the server but not from other screens" bug in reverse.

But `Session::tick` calls `collect_mail()` on its **first** line (`mod.rs:536`), and the
`in_cash_shop` early return is ~30 lines later and only suppresses NPC chatter. So a shopper's
socket keeps receiving `0x0224`, `0x0225`, `0x0293` and remote attacks **while the client is
in the Cash Shop stage**, where `CField::OnPacket` is not the active dispatcher. Whether that
is merely ignored or is fatal is **[I]** — nobody has done it. It is a cheap and separable
variant for a later run; it should not be mixed into T1.

**(b) `Bus::refresh_spawn` has zero call sites.** `Presence.spawn` is built once, in
`Session::presence`, from `remote_at()` — and `remote_at()` reads `last_position`, which is
`(0,0)` until the player's first `0x00D9`. The module doc says this "corrects itself on that
player's first step", and that is true **only for observers who were already there**: a
`0x0293` is a move for a user the observer's pool already holds. A player who arrives, walks
across the map, and stands still is announced to every **later** arrival at the map origin,
for as long as they stand still. The same freeze applies to their level and their look — an
equip or a level-up never reaches a later arrival. `refresh_spawn` is exactly the fix and is
`CLAUDE.md`'s "built is not wired".

**(c) The `0x00DC` dependency, measured.** Every leave-by-warp is published only when the
client volunteers the field-entered marker for the *next* map. Counting across
`previous-runs/` and `research/fixtures/`, deduplicated on `(time, direction, opcode)` — 444
files, 152 186 distinct events:

```text
  -> 0x01A0 SetField              273
  <- 0x00DC ClientFieldEntered    268      <- five SetFields with no marker
  <- 0x00D9 ClientUserMove       6689      <- the positive control: the key finds events
  -> 0x0224                         0      <- the brief's claim, measured
  -> 0x0225                         0
```

**[L]** on the counts. Five is not zero. Some of those five will be a `SetField` at the very
end of a truncated log rather than a lost marker — **I have not separated them**, and the
claim here is only that the marker is not guaranteed, not that it is dropped 2 % of the time.
The design consequence stands either way: a lost `0x00DC` leaves the character standing on the
old map for every other player until the socket closes.

---

## 6. Question 5 — ordering, verified against the code

**`Bus::enter_field`'s claim is true.** Reading it (`broadcast.rs:264`):

1. `m.presence.take()` — if we were somewhere, post the **old** farewell to the **old** map.
2. Collect `here` = every *other* mailbox whose `presence.map == presence.map`, taking each
   one's `spawn`. Our own presence was already taken, and the iterator also filters
   `**other != id`, so we cannot be in our own list — two independent protections.
3. Set our presence, clear `queue` (not `events`).
4. `post(id, map, spawn, None)` — our spawn to everyone else on the new map.

So the arrival gets everyone present as the direct return value, and everyone present gets the
arrival in their mailbox. **Both halves, one call, one lock.** `Inner::post` skips
`*id == from` and delivers only to a matching `presence.map`. `drain` is FIFO
(`std::mem::take` of a `Vec` pushed in order), so on a same-map re-entry the observer receives
`0x0225` **then** `0x0224` — the right order; the reverse would delete the character
permanently.

One case the code gets right and is worth naming because it is easy to break: a connection
that has already `part`ed has no mailbox, and `enter_field` returns an empty `Vec` rather than
re-inserting one — no resurrection, no announcement.

**What is not verified here:** none of this has been on a wire. `Bus`'s own tests exercise the
ordering with placeholder bodies; they cannot say anything about the bytes.

---

## 7. Symptom → cause, for the next two-client run

Run it as T1 says: one client in, settle, second client in, both on the **same map**, no Cash
Shop, no mount, short names.

| what the owner sees | most likely cause | how to confirm without another launch | fix |
|---|---|---|---|
| **The already-present client dies within a second of the second player arriving.** Second client is fine. | **The `0x0224` body is 7 bytes short.** This is the predicted outcome with today's code. | `client-patched\maplecw-hook.log` has **no dispatch line** for `0x0224` (the line is written on handler *return*), and the ELog / call stack names **`0x1429cfd52`** — the instruction after the `u32` read at `0x1429cfd4d`. | §8: add the 7 bytes. |
| Client dies, but the stack names an address **inside `FUN_1402ee8d0`** (`0x1402ee8d0..0x1402eeb3d`) | the equip list disagreed with the look length — a slot/item pair went out half-written | compare `user_enter_field_len` in the `what` string against the logged body length | check `chr.equips` |
| Client dies, stack inside `FUN_140a46e50` | the stat mask is not all zero, so bit-gated reads fired | grep the logged body: bytes 55..178 must be zero | `w.zeros` |
| **Neither client dies. Neither sees the other. Nothing on screen at all.** | one of the six gates (§4) swallowed the packet **or** the packet never left | first `world.log`: is there a `-> 0x0224` at all? If yes, arm a watch on **`0x1429ba60b`** with `140304100:hits=200` as the control. | if `1429ba60b` never fires: dump `field->[0x68]+0x481..0x483` and `ctx+0x24b4`. **No body work will help.** |
| **A sees B; B does not see A** (or the reverse) | *not* `Bus::enter_field` — §6 shows both halves are one call. Look at the **direction that failed**: the arrival's copy is a direct reply to `0x00DC`; the resident's copy is queued and needs a `tick` or a packet | count `0x0224` in `world.log` vs `world-ch1.log`; each client's log should show exactly one | — |
| **The second player appears at the map origin** and stays there | `last_position` is `None` until their first `0x00D9`; known and self-healing | it heals the instant they walk | portal coordinates (`tools/dump_portals.py` emits no x/y) |
| **A player who walked away is drawn back at the origin for a third arrival** | `Bus::refresh_spawn` is never called (§5.3b) | third client entering sees a stale position while the first two see the right one | wire `refresh_spawn` on `publish_user_move` |
| **Character appears rotated / mirrored / at a reflected position** | X and Y swapped — still **[D]**, not [L] | — | swap `at.x` / `at.y` before touching anything else |
| **The character appears but is naked / wrong hair** | the look block, which is the same bytes character select accepts | compare the logged body's `[194..389]` against `avatar_look(chr)` | — |
| **A player who logged out is still standing there** | a departure route with no leave — check §5.2. Cash Shop is the known one | `world.log` for a `-> 0x0225` with that id | §8 |
| **The client freezes rather than dying** (UI dead, quit prompt dead) | not this packet — `CLAUDE.md`'s unanswered-packet rule; something else went unanswered | the hook log's last dispatch line | — |

---

## 8. WIRE IT LIKE THIS

Nothing below is applied — this pass wrote no code and edited no existing file.

### 8.1 The seven bytes — `crates/net/src/userpool.rs`

Add a constant beside `REMOTE_STAT_MASK_LEN`:

```rust
/// The unconditional tail `FUN_140a46e50` reads **after** the mask: `u8, u8, u32, u8`.
///
/// `0x140a4a007`, `0x140a4a024`, `0x140a4a041`, `0x140a4a29e` - all four dominate every
/// exit of that function, so they are read whatever the mask says.
///
/// The first three are visible in raw bytes: `0x140a49fff..0x140a4a055` contains no
/// conditional-jump byte at all. **The fourth needs a real argument and has one**
/// (`research/user-enter-verification.md` §1.3a): no jump anywhere in `FUN_140a46e50` has a
/// source below `0x140a4a29e` and a target above it; all 186 of its jumps have literal
/// targets, so none is invisible; it has one `ret`, at `0x140a4a58e`, after the read; and
/// `tools/callers.py` gives it 2 call sites, 0 tail jumps and 0 data pointers, so it is only
/// ever entered at its first instruction. The `je` at `0x140a4a240` that gates the three
/// neighbouring reads lands on `0x140a4a296` - **two instructions before this one**, which
/// is why a `reads.py` dump makes them look alike.
///
/// There is no `EncodeForRemote` in this client to check the tail against; the only
/// 124-byte wire *write* in the image, `FUN_142973160`, is an outbound cancel with no tail.
///
/// **This is the seven bytes that made the first version of this builder 508 rather than
/// 515.** The source of the mistake was `research/msexe-secondarystat-remote-140a46e50.txt`,
/// which is truncated 14 KB before the tail.
pub const REMOTE_STAT_TAIL_LEN: usize = 7;

/// The whole remote temporary-stat block: mask plus tail.
pub const REMOTE_STAT_BLOCK_LEN: usize = REMOTE_STAT_MASK_LEN + REMOTE_STAT_TAIL_LEN;
```

In `user_enter_field`, immediately after the existing mask write:

```rust
    w.zeros(REMOTE_STAT_MASK_LEN);      // 55   no buffs
    w.zeros(REMOTE_STAT_TAIL_LEN);      // 179  u8, u8, u32, u8 - read whatever the mask says
    debug_assert_eq!(w.len(), 186 + shift, "the job field moved");
    w.u16(chr.job);                     // 186  (was 179)
```

and update the three constants and the two remaining `debug_assert_eq!`s:

```rust
pub const USER_ENTER_FIELD_MIN_LEN: usize = 515;   // was 508
pub const USER_ENTER_FIELD_LOOK_AT: usize = 194;   // was 187
pub const USER_ENTER_FIELD_POS_AT:  usize = 433;   // was 426
```

The four tests that assert `508`, `508 + 8`, `508 + 15`, and `body[451..455]` /
`body[418..422]` / `body[435..437]` move by the same 7. **Keep the numbers literal in the
tests** — a test written as `USER_ENTER_FIELD_MIN_LEN` would follow the constant wherever it
goes, which is exactly the "a test that pins what the code already does" failure.

Worth adding, because it is the assertion that would have caught this:

```rust
/// The block is the mask PLUS a tail. A test that only knows the mask is how 508 happened.
#[test]
fn the_remote_stat_block_is_131_bytes_not_124() {
    assert_eq!(REMOTE_STAT_MASK_LEN, 124, "mov r8d,0x7c at 140a46e82; cmp edx,0x3e0 in the bit test");
    assert_eq!(REMOTE_STAT_TAIL_LEN, 7, "u8+u8+u32+u8 at 140a4a007/024/041/29e, all unconditional");
    assert_eq!(REMOTE_STAT_BLOCK_LEN, 131);
    let body = user_enter_field(&someone("", &[]), RemoteAt::default());
    assert!(body[55..186].iter().all(|&b| b == 0));
    assert_eq!(u16::from_le_bytes([body[186], body[187]]), 100, "job right after the block");
}
```

### 8.2 A trailing pad is cheap insurance, and it is licensed by a measurement

`crates/net/src/buff.rs` records that **198 bytes of `0x007D` were accepted without
complaint** — this client does not check that a body was fully consumed. So appending zeros
at the **very end** of a `0x0224` costs nothing and covers any further missed unconditional
read *in the tail*. It does **not** cover a missed read in the middle, which is what this one
was, so it is insurance and not a substitute for §8.1.

### 8.3 Publish the leave from the server, not from the client's marker

`crates/world/src/session/field.rs`, in `go_to_map`, before building the `SetField`:

```rust
    // The character stops standing on the old map the moment we send the SetField, not when
    // the client gets round to volunteering 0x00DC. 273 SetFields in the archive produced
    // 268 markers (research/user-enter-verification.md §5.3c), and a lost marker leaves a
    // ghost on the old map until the socket closes. `Bus::enter_field` copes with a None
    // old presence, so calling both is safe.
    self.leave_the_field();
```

### 8.4 The Cash Shop

`on_cash_shop_request`: add `self.leave_the_field();` beside `self.in_cash_shop = true;`.
That fixes the ghost **and** stops field packets being queued for a client that is not
drawing a field — `Inner::post` skips a mailbox whose `presence` is `None`, so the leave does
both jobs at once. `on_cash_shop_exit` already goes through `go_to_map`, so the return
re-announces.

### 8.5 `refresh_spawn`

In `Session::publish_user_move`, after the successful `publish`:

```rust
    // A later arrival is dressed from `Presence.spawn`, which is frozen at field entry.
    // Without this a player who walked away is drawn at the origin for everyone who
    // arrives afterwards. `Bus::refresh_spawn` exists for exactly this and had no callers.
    let chr = /* the same Character presence() used */;
    self.bus().refresh_spawn(self.subscriber, self.presence(&chr).spawn);
```

`presence()` is currently private to the module; either make `spawn` reachable or hoist the
`Reply` construction. This is the lowest-priority item — it is a wrong position, not a crash.

---

## 9. Named blind spots — the hedged negatives

1. **Four decodes in `FUN_1429ce270` go through indirect calls and no instrument here can
   follow them**: `[chairObj+0x18]` at `0x1429ceb44`, `[user vtable+0x150]` at `0x1429cecb9`,
   `[o+8]` at `0x1429cede5`, and the pet decoder. All four are behind a wire flag we send as
   zero, and I confirmed each gate's polarity in the listing (§2.4). The minimum body is
   unaffected; the moment a chair, pet or familiar is sent, §2.2 is wrong.
2. **`FUN_142833030` (§2.3) is closed only while offset 490 is zero.** If it ever carries a
   value, the packet reaches a virtual on `user+0x3b90` and the tail length is unknown.
3. **The 8-iteration loop at `0x140a4a133` inside the stat decoder ends in an indirect call
   that takes the packet.** It is gated by `FUN_14080fa00`, which returns "any of the 31 mask
   dwords nonzero" — I read that function's raw bytes, so with an all-zero mask it is
   provably skipped. **[L]** With a nonzero mask, it is a blind spot.
4. **X and Y remain [D], not [L].** Two signed `u16`s become args 3 and 4 of
   `vtable[0x118]`; nothing labels which is which. Unchanged from the prior pass.
5. **Gates 3–6 are named by offset, not by meaning.** I read the four getters' bytes and can
   say *which* fields they are (`ctx+0x24b4`, `field->[0x68]+0x481..0x483`, `ctx+0x23e8`). I
   did **not** find what writes them, and I deliberately did not run a write-scan:
   `CLAUDE.md`'s `mob+0x42c` lesson is that a `[reg+disp]` write-scan cannot see a store
   through a handed-off pointer, so an empty result would have been worthless. The watch at
   `0x1429ba60b` answers the question these gates raise without needing their names.
6. **The `273 / 268` SetField-to-marker gap is a count, not a diagnosis.** I did not check
   whether the five missing markers are truncated logs.
7. **Whether a field packet delivered to a client in the Cash Shop stage is harmful is [I].**
   Nobody has done it, and it is a separate variant from T1.
8. **Nothing here has been on a wire.** `0x0224` and `0x0225` appear **zero** times in 444
   archived log files (§5.3c), which is the strongest form of the brief's own warning: every
   claim in this document and in `user-enter-field.md` is static.

---

## 9a. How many other `research/` dumps stop early — audited, because one just cost seven bytes

63 of the 150 `research/msexe-*.txt` files carry a `; 0xA .. 0xB (N bytes)` header.
Comparing that declared end against the last instruction address in the file:

```text
   missing   declared  file                                          note
   197 994    198 742  msexe-secondarystat-140a165f0.txt             prose says partial
    13 720     14 143  msexe-secondarystat-remote-140a46e50.txt      <-- THIS BUG
       865     13 864  msexe-localuser-dispatch.txt
       507     45 228  msexe-usereffect-handler.txt
       402      2 796  msexe-mobcmd-dispatch-141d32b30.txt
```

`msexe-migrate-command.txt` and `msexe-skillwnd-teardown.txt` also flag, and are **false
positives** — they hold 6 and 7 concatenated function dumps, so "the last address" belongs to
a later dump. The three sub-1 KB gaps are plausibly the dumper stopping at the last decodable
instruction before tail padding; I did not check them.

**The instrument itself needed a positive control.** My first version read the header from
line 1 only, and the file that caused this bug puts a prose title on line 1 and the header on
line 3 — so it came back clean, with the known positive missing. That is the same failure
shape as everything else in this document, one level up.

The generalisation: **`research/*.txt` is not a substitute for the binary, and only a dump's
own header says how much of the function it contains.** `msexe-secondarystat-remote` calls
itself a *"short list"*, which reads as "there are not many stat entries" and means "this
dump is cut off 14 KB early". Read the header's byte count before drawing a negative from a
dump's silence.

---

## 10. One correction to a file I did not edit

`research/user-enter-field.md` §2.2 row **55** reads *"raw 124 | 124 | the remote
temporary-stat block"*, and §2.2's total, §4's offset 187, §7's wire diagram and
`crates/net/src/userpool.rs`'s `USER_ENTER_FIELD_MIN_LEN`, `USER_ENTER_FIELD_LOOK_AT` and
`USER_ENTER_FIELD_POS_AT` all descend from it. **The block is 131 bytes.** The mask is 124;
the block is the mask plus an unconditional `u8, u8, u32, u8`.

The finding in that document is otherwise sound and its Init enumeration reproduces exactly.
The lesson is narrower than "the pass was wrong": **a call site that names a length names the
length of one field, not of the callee.** The instrument that catches it is
"which calls dominate every exit, and which of those receive the packet" — `pktcalls.py` /
`uncondcalls.py` in the scratchpad, worth promoting to `tools/` if anyone decodes another
handler of this size.
