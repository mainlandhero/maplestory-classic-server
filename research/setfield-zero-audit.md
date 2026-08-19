# `SetField` — is zero safe? An audit of everything *around* the character record

Handler `FUN_142097f80`, the `characterData != 0` path. The 224-byte record is settled
elsewhere (`charrecord-flag7.md`, `charstat-layout.md`, `charrecord-presence-map.md`); this
pass audits the fields **before and after** it, all of which the server currently sends as
zero, looking for the ones where zero is not "absent" but a valid index, count, rate or
flag value.

**Labels.** `[L]` read directly from a decompilation on disk. `[D]` derived from on-disk
text by cross-reference. `[I]` inferred / candidate — weaker, and where it comes from the
reference source at `ModernMapleSource` it is a **candidate only** (CLAUDE.md: 1 of 8 on a
held-out control).

**No Ghidra was opened for this pass.** Everything below is from text already on disk.

---

## 0. Instrument check, and what it cannot see

**Positive control.** The decompiled body `research/msexe-stage-setfield.c` lines 1–1842
contains **exactly 50** calls to the seven packet-read primitives. The earlier pass counted
50 in a full disassembly of all 11726 bytes (`msexe-stage-setfield.md`). Two independent
instruments, same number — so the decompilation is **complete for primitive read sites**,
and a read I did not list is a read that is not there.

**What that control does not cover.** The primitives are not the only things that consume
wire. **Five non-primitive calls on this path also receive the packet pointer**, and the
decompiler shows nothing of what they eat. Enumerated in §2.5. Any statement of the form
"the next field is at offset N" past the first of those is not supported by anything I read.

**Not on disk in any form**, so anything said about them is `[D]` from the call graph or
`[I]`: `FUN_14025ea90`, `FUN_142ce5170`, `FUN_142cbef90`, `FUN_14187e880`, `FUN_142cc3d90`,
`FUN_142d9c9f0`, `FUN_142d0e420/4d0/570`, `FUN_142d1d050`, `FUN_142cc0160`, `FUN_142d16ef0`,
`FUN_141816220`, `FUN_14209ee50`, `FUN_142834df0`, `FUN_142835840`, `FUN_1428358a0`,
`FUN_1427bdd40`, `FUN_1420a1b20`, `FUN_1418c5820`, `FUN_142d96f80..fd0`.

**Address caveat.** `msexe-stage-setfield.md` carries listing addresses for the head and for
the first eight reads on this branch; I have those. **I have no listing for the region after
that**, so reads past `1420984ca` are cited by decompiler line number only. I did not
manufacture addresses for them.

---

## 1. The three `u32`s read immediately before `FUN_140304b20`

`msexe-stage-setfield.c:482-496`, between the `characterData` fork at `1420983a3` and the
record decoder call at `14209842d`:

```c
uVar8  = FUN_1406e8c20();          // u32
uVar10 = FUN_1406e8c20(param_2);   // u32
uVar11 = FUN_1406e8c20(local_3e0); // u32
uVar17 = FUN_142cbef90(lVar24);            // lVar24 = world (DAT_143aa84a0)
FUN_14025ea90(uVar17, uVar8, uVar10, uVar11);
FUN_142ce5170(lVar24, uVar8, uVar10, uVar11);
uVar33  = FUN_1420a3080(0);                // the user object
FUN_140304b20(uVar33, local_b8, local_3e0, 0);
```

**`[L]` All three are passed intact, as a triple, to two different sinks, and nowhere else.**
Neither value is used separately, indexed, compared, or stored on its own. That shape rules
out `(fieldId, portal, x)` — a triple whose members had individual meanings would be
consumed individually.

**This retires the candidate in `charrecord-flag7.md` §6**, which floated these as "the shape
a `(fieldId, portal, ...)` triple would have" while explicitly labelling it a pointer, not a
finding. The map id is now settled at stat-block offset 84, and the consumption shape here
says these are not it. That entry should be corrected.

**`[D]` They are a seed triple for a randomiser held by the world object.** The evidence is
the call graph in `msexe-packet-fields.txt`, which is on-disk text:

| line | opcode | calls |
|---|---|---|
| 1536 | `0x0309` (`Sound/Mob/%d/sequenceAttack`) | `FUN_142cbef90`, `FUN_140268a60`, then four `u32` reads |
| 120 | `0x00E5` | `FUN_142cbef90`, `FUN_140268a90` |
| 125 | `0x00E5` | `FUN_142cbef90`, `FUN_140268a90` |

`FUN_142cbef90(world)` returns an object; **mob-behaviour packets fetch it and immediately
call a `FUN_14026xxxx` method on it**, while `SetField` fetches it and calls
`FUN_14025ea90(obj, a, b, c)` with three ints. That is a getter/consumer pair whose consumers
sit in exactly the two handlers that would need a *server-synchronised* random stream
(mob attack sequence selection, mob action). `FUN_142ce5170(world, a, b, c)` receives the
same triple, so there are **two** streams seeded from one packet.

**`[I]` candidate name, reference source:** `Stage.java:47-53` writes exactly three `int`s
here and feeds the same three to `chr.setDamageCalc(new DamageCalc(chr, s1, s2, s3))`. So the
candidate is *damage-calculation / mob-action RNG seeds*. Candidate only — I did not read
`FUN_14025ea90`.

**Is zero safe?**

* **Not an index, not a count, not a pointer.** `[D]` Nothing dereferences or bounds-checks
  them, so zero cannot fault here. This is the reassuring half.
* **`[I]` But zero is very likely a degenerate seed.** MapleStory's `CRand32` is a
  Tausworthe/LFSR-113 combined generator, and every generator of that family has all-zero as
  an **absorbing state**: seeded with `(0,0,0)` it emits `0` forever. If that is what these
  are, the client's mob-action and damage streams become a constant. That is a *behavioural*
  failure (mobs pick the same action; a locally-computed damage roll is always the same
  number), not a crash — unless something later divides by a draw. I did not read the
  generator, so this is inference from the family, and it is the one place I would spend a
  cheap change: **send three non-zero pseudo-random `u32`s instead of zeros.** It costs
  nothing and removes the whole class.

**Loudly, since the task asked:** *none of the three is an index into a table or a count.*
They are consumed as an opaque triple.

---

## 2. Every read after `FUN_140304b20` returns

### 2.1 The gate at `142098435` — confirmed

`[L]` `msexe-stage-setfield.c:497`:

```c
cVar2 = FUN_1406e8ae0(uVar17);       // 142098435
if (cVar2 != '\0') { ...seven reads... }
```

**Confirmed: a zero here skips the whole block in one branch.** What is skipped, in order,
with the addresses from `msexe-stage-setfield.md`:

| # | addr | type | dest | consumer |
|---|---|---|---|---|
| 1 | `142098445` | `u8` | `cVar2` | sub-gate for #2 only |
| 2 | — (only if #1 ≠ 0) | `u32` | `local_450` low | `FUN_142d0e420(world, v)` |
| 3 | `142098471` | raw 8 | `local_320` | `local_456 = FUN_142d0e4d0(world, local_320)` |
| 4 | `142098479` | `u8` | `uVar3` | `FUN_142d0e570(world, uVar3, local_250)` |
| 5 | `14209849f` | raw 8 | `local_250` | (same call as #4) |
| 6 | `1420984c2` | raw 8 | `local_248` | `FUN_142d1d050(world, local_248, v)` |
| 7 | `1420984ca` | `u32` | `uVar8` | (same call as #6) |

then `FUN_142cc0160(lVar24)`. **Seven read sites, one of them itself gated** — which is what
the task described.

`[L]` Two details worth having:

* The client **pre-initialises `local_250` and `local_248` to `DAT_14342b7f0`** before
  overwriting them from the wire (lines 509, 511). That is a source-level default for the two
  8-byte fields — i.e. the client itself considers them optional and has a canonical value.
* `local_450` is initialised to `0` at line 346 and is read at line 1427:
  `if ((int)local_450 != 0) { iVar13 = local_450 / 0x3c; ...string 0x926... }` — divided by
  **60** and formatted into a message. `local_456` (from read #3) gates a second message from
  strings `0x927` / `0x928` at line 1452. **`[D]` This block is a play-time / logout-warning
  notice** ("you have N minutes left"), which is consistent with the reference's own comment
  on the same block (`Stage.java:56`, "something with logout event").

**Is zero safe?** `[D]` Yes for the handler: everything downstream of it is gated on `!= 0`
or on `local_456 != '\0'`, both of which stay at their initialised zero. **`[I]` But the
reference sends `true` here**, and skipping means `FUN_142d0e420/4d0/570`, `FUN_142d1d050`
and `FUN_142cc0160(world)` never run at all — four world setters and what looks like an apply
step left untouched. This zero was also in the packet the client **accepted** on 2026-08-19,
so it is measured not to abort the handler.

### 2.2 The reads on the common path, in decompiler order

Everything below runs on both branches. **No listing addresses available** — line numbers in
`msexe-stage-setfield.c`.

| line | type | what the client does | zero safe? |
|---|---|---|---|
| 552 | `u8` | `0` → `FUN_142da05c0(world, &local_350)`; non-zero → two `u32`s (557, 558) → `FUN_142da05a0(world, u64)`. `[L]` `FUN_142da05c0` is on disk (`msexe-fieldload.c:449`): it reads `world+0x31b4`, copies `world+0x31b8` out, and **zeroes `world+0x31b4`**. Its return is discarded here. | **Yes.** `[D]` Zero means "no override, consume whatever the client already had pending". It is the client's own default path. |
| 557,558 | `u32`×2 | only if 552 ≠ 0 | n/a |
| 586 | `u8` | `[L]` `if (DAT_143ac7fdc == 1 && (b>>1 & 1) == 0) DAT_143ac7fdc = 2;` then `FUN_142cc3d90(world, b & 1)` | **NO — see §4.4.** Bit 1 **clear** is an action, not an absence. `[I]` the reference sends `2` here (bit 1 set, bit 0 clear). |
| 591 | `u8` | `if (b != 0 \|\| stage[0x49] != 0) { stage[0x49] = 0; ...; if (b != 0) stage[0x48] = 0; }` | **Yes.** Zero skips, and `[I]` the reference also sends `0`. |
| 667 | `u32` | `local_364`. `[D]` `local_364`/`local_368`/`local_370`/`local_378` are contiguous stack slots and `local_378` is passed whole to `FUN_14209ee50(*(u64*)(field_holder+8), local_378)` at line 746 — i.e. this `u32` is a **field of the user-enter event posted to the field object**. Nothing else reads it. | **NO — see §4.2.** `[I]` the reference sends `100` (`mobStatAdjustRate`). |
| 668 | `u8` | non-zero → allocate `0x38`, `FUN_1418c5820(obj, packet)` — a **nested decoder**. Zero → release and leave null. | **Yes.** `[I]` reference `hasFieldCustom`, normally false. Zero is also what keeps a nested reader out of our tail. |
| 719 | `u8` | `local_318`; read at line 1709: **`if (local_318 == 0) FUN_142d9c9f0(world)`** | **Yes**, though zero *does* run code. `[I]` the reference sends `false` here too (`CWvsContext::OnInitPvPStat`), so zero is the normal value and `FUN_142d9c9f0` is the ordinary path. |
| 838 | `u8` **count** | Read **only** when `FUN_141892840() != 0 && FUN_141830050(field) != '\0'` — a predicate on the *current field*. Then loops that many `FUN_1406e9050` string reads (841) into `FUN_141b7ec60`. | **Yes** as a count (MSVC-rotated, guard before head, zero never enters). **But see §5 gap 1**: whether the byte is consumed *at all* depends on client state, not on the wire. |
| 876 | `u8` gate | non-zero → five `u32`s (878–886) into `FUN_142d96f80/f90/fa0/fb0/fc0(world, v)` and one string (888) into `FUN_142d96fd0` | **Probably**, see §4.6. `[I]` the reference sends `true` with `(-1, 0, 0, 999999999, 999999999, "")`, and **`999999999` is this client's own "none" sentinel** — line 732 tests `FUN_1402fa540(user+0xf3) != 999999999`. |
| 892 | `u8` gate | `local_448`; non-zero → three strings (902, 909, 916) and two `u32`s (923, 925), consumed at lines 1084 and 1516 by `FUN_1420a1a40(&local_3d8,&local_398,&local_390,&local_3b0)` behind further client-side conditions | **Yes.** Zero leaves `local_448 == 0`, which short-circuits both consumers. |

That is the complete primitive-read list after `FUN_140304b20`: **7 skipped by the gate, plus
14 more sites on the common path**, of which 9 are themselves gated to zero by our bytes.

### 2.3 Where each of those lands in the body we actually send

`set_field_with_character` builds `33 + 12 + 224 + 1 + 384 = 654` bytes.

| body offset | field |
|---:|---|
| 0..32 | the head |
| 33..44 | the three `u32`s (§1) |
| 45..268 | the character record; `45` is `presence[0]` |
| **269** | the gate `u8` at `142098435` — currently `0` |
| **270** | the `u8` at line 552 |
| **271** | the `u8` at line 586 — **reference sends `2`** |
| **272** | the `u8` at line 591 — reference also `0` |
| **273..276** | the `u32` at line 667 — **reference sends `100`** |
| **277** | the `u8` at line 668 |
| **278** | the `u8` at line 719 |
| 279.. | either the line-838 count (if the field qualifies) or the first byte eaten by `FUN_142834df0` — **not determined** |

If offset 269 is ever set to `1`, the seven reads consume 31 bytes and **everything from 270
shifts to 301**.

### 2.4 One `characterData`-only block worth knowing about

`[L]` lines 1046–1062, reached only because `characterData = 1`:

```c
puVar15 = *(undefined8 **)(uVar33 + 0x1111);         // uVar33 = the user object
if (puVar15 != 0 && (puVar19 = puVar15 + *(uint*)(uVar33 + 0x1119), puVar15 < puVar19)) {
    ... walk it, then LAB_142099690 dereferences node+8, node+0x10, node+0x14 ...
    FUN_142ce4870(world, node[0x10], node[0x14] * 1000);
}
```

A **hash table pointer and count inside the user object**, neither of which our record writes
(the presence-#7 region touches `+0x1001`, the stat block, `+0x118b` and `+0x1253/125b/1263`).
A non-null-but-garbage `+0x1111` would fault here. **`[D]` Measured safe**: the 2026-08-19 run
took this exact path with an all-zero record and reached the game stage, so `+0x1111` was null
or the count was zero. Nothing in the change since then writes it.

### 2.5 The five calls that eat wire without showing it

`[L]` after `FUN_140304b20`, these receive the packet pointer:

| line | call | conditional? |
|---|---|---|
| 717 | `FUN_1418c5820(obj, packet)` | yes — only if line 668 ≠ 0. We send 0. |
| 873 | `FUN_142834df0(DAT_143aa8518, packet)` | **no** |
| 874 | `FUN_142835840(DAT_143aa8518, packet)` | **no** |
| 875 | `FUN_1428358a0(DAT_143aa8518, packet)` | **no** |
| 891 | `FUN_1427bdd40(DAT_143aa8518, packet)` | **no** |
| 935 | `FUN_1420a1b20(packet, DAT_143aa8518 + 0x53c8)` | **no** |

Six, of which five are unconditional. `[D]` **Empirically bounded**: the packet accepted on
2026-08-19 carried the same 384-byte zero margin after the same terminator, and the client did
not throw an underrun — so on that field these five consumed fewer than ~375 bytes of zeros
between them. The record grew by 112 bytes *before* the margin, which does not change what
they see. **`[I]`** the bound is field-dependent if any of them branches on client state.

---

## 3. The head fields we send as zero

### 3.1 The reference lines up — structurally, not by name

`ModernMapleSource/v214 src/.../connection/packet/Stage.java` `setField()` matches this
client's head with **exactly one insertion** (`encodeByte(unk)` right after the FileTime,
which this client does not read):

| our offset | client behaviour `[L]` | reference field `[I]` |
|---:|---|---|
| 0 | `raw 8` → `FUN_1408f67d0`, a clock base | `encodeFT(currentTime())` |
| 8 | `u32` → `world+0x2260`, "Channel" toast on change | `encodeInt(channelId - 1)` |
| 12 | `u8` → `world+0x226c` | `encodeByte(dev)` |
| 13 | `u32` → `world+0x2884` | `encodeInt(oldDriverID)` |
| 17 | `u8`, `if (== 1)` → `FUN_142d16ef0(world)` | `encodeByte(characterData ? 1 : 2)` |
| 18 | `u32` **read and discarded** | `encodeInt(...) // unused` |
| 22 | `u32` → `local_418` | `encodeInt(field.getWidth())` |
| 26 | `u32` → `local_438` | `encodeInt(field.getHeight())` |
| 30 | `u8` `characterData` | `encodeByte(characterData)` |
| 31 | `u16`, then one string then that many strings | `encodeShort(notifierCheck)`, then `encodeString` + loop |

**Why I am willing to lean on this more than CLAUDE.md's 1-of-8 warning normally allows.**
That score was for *field naming*. What I am using it for here is *structure*, and the
structure is corroborated by **eight client-side facts the reference correctly predicts**,
each of which was read out of this binary independently:

1. offset 18 is *discarded by the client* ↔ the reference's own comment is `// unused`.
2. offset 31 is a count followed by one string then a loop ↔ `notifierCheck` written exactly
   that way.
3. the seven post-record reads are, in order, `u8` gate / `u8` gate / optional `u32` /
   raw 8 / `u8` / raw 8 / raw 8 / `u32` ↔ the reference's `bool1` block, field for field.
4. the client **pre-loads the two 8-byte slots with `DAT_14342b7f0`** ↔ the reference writes
   `FileTime.MIN_TIME()` into exactly those two.
5. the client gates a `u8 count + N strings` read on a property of the *current field*
   (`FUN_141830050`) ↔ the reference gates the same shape on `isVonbonField(chr.getFieldID())`.
6. offset 8 goes to a channel slot and raises a "Channel" toast on change ↔ `channelId - 1`.
7. offsets 22/26 are handed as a pair to the field object ↔ `getWidth(), getHeight()`.
8. the line-876 block is `u8` gate + five `u32` + one string ↔ the reference's
   `encodeByte(true) { -1, 0, 0, 999999999, 999999999, "" }`, and **999999999 appears as a
   literal sentinel inside this binary** at line 732.

Eight consecutive shape matches is a different kind of evidence from a name. **I am still not
claiming the names are right.** What I am claiming is that where the reference writes a
*specific non-zero value* into a slot whose shape matches, that is a testable prediction about
a byte we currently zero. The alignment **breaks after item 7 of §2.2** — the reference has
`int 0 / byte / extraTMSSystem / byte isExtendSpJob / long / int -1 / byte` where this client
reads nothing — so nothing past the line-719 byte should be taken from it at all.

### 3.2 Field by field

**offset 12, `u8` → `world+0x226c`.** `[L]` set unconditionally, no consumer read.
`[I]` candidate `dev`. **Zero is almost certainly right** — a "dev/debug" flag would be 0 on a
live service. *No evidence either way from this binary.* Low concern.

**offset 13, `u32` → `world+0x2884`.** `[L]` set unconditionally, no consumer read.
`[I]` candidate `oldDriverID`, an id of the previous connection/instance. **Zero plausibly
means "none".** No evidence from this binary. Low concern.

**offset 17, the tree-reset byte.** `[L]` two consumers, not one:

1. line 237: `if (local_457 == '\x01') FUN_142d16ef0(lVar24)` — per `msexe-stage-setfield.md`,
   walks the tree at `world+0x3bd0` clearing field 5 of every node. (I did not re-read
   `FUN_142d16ef0`; that is `[L]` from the earlier pass.)
2. line 665: `local_368 = local_457` — copied into the user-enter event struct posted to the
   field object at line 746.

**`[I]` The reference writes `1` here on exactly the case we are emulating** (`characterData`
true) and `2` otherwise. **We send `0`, which is not either of the two values it ever writes.**
So this is not "an optional thing left off"; it is an out-of-range enum reaching a field-object
event. Flagged. See §4.3.

**offset 18, read and discarded.** `[L]` line 240, return value never stored. **Zero is
entirely safe** and this is the one field where I have a two-instrument agreement: the client
discards it and the reference comments it `// unused`. No action.

**offsets 22 and 26, the two `u32`s to the stack.** `[L]` `local_418` and the low half of
`local_438`. Traced every use: they are read **once**, together, at line 871:

```c
uVar21 = FUN_141892840();                                       // the current field
FUN_14187e880(uVar21, local_418, (ulonglong)local_438 & 0xffffffff);
```

`[D]` `FUN_141892840()` is the current-field getter — line 1081 does
`FUN_141829f70(FUN_141892840())` and compares the result against `10000`, a map id, and line
1516 compares `FUN_141829fd0(...)` against `0xda`. So **offsets 22 and 26 are a pair of `u32`s
handed to the field object and nothing else.** `[I]` candidate: the field's width and height.
**This is the most load-bearing pair of zeros in the head.** See §4.1.

---

## 4. Ranked: the most likely reason this run fails

**The frame that decides the ranking.** Every byte outside the record is **identical** to the
packet the client *accepted* on 2026-08-19 (`fixtures/setfield-accepted-client-entered-world-*`):
it entered `FUN_142097f80`, both early returns passed, and 422 ms later it sent `0x0238` and
`0x024D` — it reached the game stage. So none of these zeros aborts the handler. **What
changed is `presence[0] = 1`, which means the map id is now `1` instead of `0`, and for the
first time a field actually loads.** That reorders everything: the dangerous zeros are the
ones consumed *by the field object*, because in the accepted run there was no real field to
consume them.

One thing this pass could check cheaply and did: **map 1 exists, and map 0 does not.**
`[L]` `FUN_1403999e0` (`msexe-fieldload.c:38`, the function `SetField` calls with the map id
and `PTR_s_mapName`) builds `Map/Map/Map{id/100000000}/{id:09d}.img`.

*Instrument, stated because a silent negative here would be worthless:* the WZ directory
names are XOR-obfuscated (`byte ^ (0xAA + i)`), which is why a plain string search finds
nothing. I decoded **every** negative-length-prefixed entry of length 4..40 in
`client-patched/Data/Map/Map/Map0/Map0_000.wz` rather than searching for a pattern — 423
names, self-consistent (`0000000xx` Maple Road, `01xxxxxxx` Henesys/Ellinia, `02xxxxxxx`
Perion/Kerning, `08x`/`09x` specials). Two cross-checks: `Map9_000.wz` decodes to exactly
`900000000/1/2.img`, and the first eleven Map0 names match `gm-handbook/maps.txt` line for
line (`1, 10, 20, 21, 30, 40, 41, 42, 50, 60, 61`).

**`[D]` `Map/Map/Map0/000000001.img` is present. `000000000.img` is not** — the lowest id in
the archive is 1, independently confirming the note on `START_MAP_ID`. So the previous run's
specific cause — `FUN_140ce89c0` releasing a holder that a lookup keyed by map `0` never
filled in — should be gone. That is why "the map does not exist" is *not* on this list.

---

**1. `FUN_14187e880(field, 0, 0)` — the pair at head offsets 22 and 26.**
Read at `142098152` and `14209815d`; consumed at `msexe-stage-setfield.c:871`.
`[D]` The only two numbers from the wire that go to the field object, handed over right after
the field is loaded. `[I]` candidate width/height. A field that is told it is 0 wide and 0 tall
is the classic input for a zero divisor in a grid/quadtree partition, a zero-size allocation
that later indexes, or a camera clamp that produces NaN. **In the accepted run these same
zeros were consumed by a field that never finished loading; this time they will not be.**
This is the highest-variance zero on the list and also one of the cheapest to change — a
plausible pair (the map's own VR rect, or simply a large positive number) costs one commit.

**2. The `u32` at line 667 — `[I]` `mobStatAdjustRate`, reference value `100`.**
`[D]` It is a member of the user-enter event struct posted to the field at line 746
(`local_364` sits contiguously with `local_368`, `local_370`, `local_378`, and `local_378` is
passed whole). A field of `100` in a slot named "…Rate" is a percentage; **`0` in a percentage
slot is either "everything scales to nothing" or, if the code divides by it, `0xC0000094`.**
Same argument as #1: it now reaches a field that exists. Body offset 273..276.

**3. The tree-reset byte, head offset 17, read at `142098132`.**
`[L]` Two consumers (§3.2). `[I]` The reference writes `1` on precisely this case and `2`
otherwise, so **`0` is a value the real server never sends**, reaching both a world-tree reset
and a field event. Skipping `FUN_142d16ef0` leaves field 5 of every node in `world+0x3bd0`
un-cleared across a field change; if that field is a validity flag over per-field objects, the
stale-pointer story writes itself — but that last step is `[I]` and I did not read
`FUN_142d16ef0` this pass.

**4. The `u8` at line 586 — reference value `2`, body offset 271.**
`[L]` `if (DAT_143ac7fdc == 1 && (b >> 1 & 1) == 0) DAT_143ac7fdc = 2;` plus
`FUN_142cc3d90(world, b & 1)`. **Bit 1 being clear is an action.** We send `0`, so if
`DAT_143ac7fdc` is `1` at this moment we move a global into state `2` that the reference's
value would have left alone. `DAT_143ac7fdc` appears **only** in this function across all of
`research/`, so I have no idea what state `2` means. Ranked here rather than higher because it
was also `0` in the accepted run — but so was everything, and that run never loaded a field.

**5. The gate at `142098435` closed, body offset 269 — reference sends `true`.**
`[D]` Skipping leaves `FUN_142d0e420`, `FUN_142d0e4d0`, `FUN_142d0e570`, `FUN_142d1d050` and
`FUN_142cc0160(world)` unrun. Four setters and an apply. `[D]` Measured survivable: identical
in the accepted run. Risk is that one of those world fields has a construction default that is
wrong for a live field, which would show as misbehaviour rather than a fault.

**6. The line-876 block skipped — five world setters and a string.**
`[I]` The reference sends `true` here with two of the five values equal to `999999999`, and
`[L]` `999999999` is this binary's own "none" sentinel (line 732). If any of
`FUN_142d96f80..fc0`'s targets defaults to `0` rather than `999999999`, a later
`!= 999999999` test mis-fires. Speculative; I did not read the setters.

**7. The five unaudited packet-consuming calls (§2.5).**
`[D]` Bounded by measurement — the same 384-byte zero margin survived the accepted run without
an underrun. They stay on the list only because the bound is field-dependent and the field is
what changed.

**8. The three seeds at `0`.**
`[I]` Degenerate-state risk only (§1). Not a fault path on anything I read. Cheapest possible
fix, so worth doing even at rank 8.

**Outside this audit but honest about it:** if the run fails, the record itself is at least as
likely a cause as any of the above, because the record is the only thing that changed. This
pass did not re-audit it.

**Two on-screen signals to have the owner watch for.** `[L]` Both are string-splice toasts in this
handler and both are cheap discriminators:

* **"MAP" + a map name** (line 533–551) fires whenever `FUN_142d01d50(world)` differs from
  `FUN_1402fa540(user+0xf3)`. It will differ — world is at map 0, the record now says 1. **If
  that line appears with a real name, the stat block decoded and the map id resolved through
  `FUN_1403999e0` to a WZ node.** If it appears blank, the id arrived but the lookup missed.
  If it does not appear at all, `presence[0]` did not open the stat block.
* **"Channel"** (line 223–234) fires only if the channel changed. We send `0`; if the client
  is already on 0 it will not appear, and its absence means nothing.

---

## 5. Gaps — stated rather than guessed

1. **`[L]` The byte at line 838 is read conditionally on client state**, not on anything the
   server said: `FUN_141892840() != 0 && FUN_141830050(field) != '\0'`. So the wire position of
   everything after it is **not determined by the packet alone**. Harmless while the tail is
   zeros; fatal to any future attempt to place a non-zero field after it. `[I]` the reference
   has the same shape gated on `isVonbonField`, i.e. a field-type test — so the discriminator
   is probably "is this map one of a small set", and map 1 is probably not in it.
2. **The five nested decoders (§2.5) are completely unread.** Byte counts, and therefore every
   offset past body 279, are unknown. Only the ~375-byte empirical bound applies.
3. **`DAT_143ac7fdc` is unidentified.** It occurs once in the whole of `research/`.
4. **`FUN_14025ea90` / `FUN_142ce5170` / `FUN_142cbef90` were not read.** The seed conclusion
   in §1 rests on the call graph in `msexe-packet-fields.txt` plus a reference candidate. If it
   matters, those three functions settle it in one Ghidra pass.
5. **`FUN_14187e880` was not read**, so "width and height" for offsets 22/26 is a candidate,
   not a finding. Everything in §4.1 above the words "candidate width/height" survives without
   it: the pair *is* handed to the field object and nothing else, and that much is `[L]`.
6. **No listing exists on disk for the region after `1420984ca`.** Every read in §2.2 is cited
   by decompiler line number. The decompiler's ordering was trusted here because the 50-site
   count agrees with the listing pass, but `msexe-stage-setfield.md` explicitly warns that its
   *ordering* degrades once it reaches the string machinery — which is exactly where lines
   838–925 sit. **Treat the order of the line-838, -876 and -892 blocks relative to the five
   nested calls as unconfirmed.**

---

## 6. Corrections this pass owes to other documents

* **`research/charrecord-flag7.md` §6** offers the three `u32`s as "a better candidate" for
  where the map id lives, shaped like `(fieldId, portal, ...)`. §1 above shows they are passed
  as an intact triple to two sinks and never used individually, which does not fit. The map id
  question was settled separately (stat-block offset 84), so nothing was built on it — but the
  paragraph should say the candidate was checked and dropped, and why.
* **`crates/net/src/opcode.rs`**, the doc comment on `set_field_head`, calls offsets 22 and 26
  "`u32 u32`" with no destination. They have one, and it is the field object (§3.2).

---

## Addendum from the main session: risks 1 and 3, read rather than inferred

Both were `[I]` above. Both are now `[L]`, from `research/msexe-field-dims.c`.

### Risk 1 - `FUN_14187e880` is a plain setter, 28 bytes

```c
void FUN_14187e880(longlong field, undefined4 a, undefined4 b) {
    *(undefined4 *)(*(longlong *)(field + 0xa8) + 0x1ad8) = a;
    *(undefined4 *)(*(longlong *)(field + 0xa8) + 0x1adc) = b;
}
```

Two stores into a sub-object at `field+0xa8`. **No division, no allocation, no indexing.** The
inferred failure story above - "the classic input for a zero divisor in a grid/quadtree
partition, a zero-size allocation that later indexes, or a camera clamp that produces NaN" -
**is not present at the call site.** Zeros are inert where they land.

That does not prove a *later* reader is safe, and `tools/xref.py --field 0x1ad8` cannot settle
it (it reports writes only, and `0x1ad8` still collides across unrelated types). But the
specific mechanism this item was ranked #1 for is not there, so **it should not be ranked #1**,
and it is **not** a reason to invent a width/height pair. Inventing one would put fabricated
numbers on the wire to fix a fault nobody has seen.

### Risk 3 - the tree walk is a no-op on an empty tree

`FUN_142d16ef0` is a red-black tree in-order traversal - `std::map`, by the `_Isnil` byte at
node `+0x19` and the head-node idiom:

```c
plVar4 = (longlong *)**(longlong **)(param_1 + 0x3bd0);
cVar1  = *(char *)((longlong)plVar4 + 0x19);
while (cVar1 == '\0') { if ((int)plVar4[5] != 0) plVar4[5] = 0; ... }
```

On an **empty** map the head node's `_Isnil` is 1, so the loop body never executes. On a fresh
migration nothing has populated `world+0x3bd0`, so calling it and not calling it are the same
thing. **[D]** Sending `0` instead of the reference's `1` therefore changes nothing here. The
"stale-pointer story writes itself" concern needs a *non-empty* tree, which a first field entry
does not have. Correctly labelled `[I]` above; the answer is that it is inert.

## What this means for the run: change nothing

Weighing all six against `CLAUDE.md`'s "test one variant at a time":

| item | verdict |
|---|---|
| three `u32` seeds | **Defer.** The audit itself says zero cannot fault. All-zero being an LFSR absorbing state is a real correctness point but cannot stop a map loading, so changing it now buys no risk reduction and costs a second variable. Worth a follow-up commit *after* this run. |
| offsets 22/26 | **Leave zero.** Now known to be a plain setter; a "plausible pair" would be fabricated data. |
| offset 17 | **Leave zero.** Its consumer is a no-op on an empty tree. |
| the rest | **Leave zero.** All were zero in the run the client accepted. |

The variant under test is the record: `presence[0] = 1` and a real map id. Nothing else moves.
