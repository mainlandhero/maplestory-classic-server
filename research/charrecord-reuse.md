# Character record: what we already build, and what `SetField` can reuse

Written 2026-08-19 from the repo only — no client run, no Ghidra. Every row below is tagged
**[read]** (taken from a decompilation or source file on disk, cited) or **[inferred]**
(reasoning, a reference-source name, or a claim that no file on disk supports).

Sources used, all on disk:

| file | what it gives |
|---|---|
| `research/msexe-charstats.c` | `FUN_140302e30`, the stat decoder — the full body |
| `research/msexe-charrecord.c` | `FUN_1403094b0` (the list record), `FUN_14108bdf0`, `FUN_14108d290` |
| `research/msexe-avatarlook.c` | `FUN_1402ee8d0`, the avatar look — the full body |
| `research/msexe-packet-readers.c`, `research/msexe-decoders3.c` | the five read primitives |
| `research/msexe-newchar-scan.c:1952` | `FUN_1402cb0d0`, the extended-SP table |
| `research/msexe-stage-setfield.md` | the `0x01A0` head and the `FUN_140304b20` call site |
| `crates/net/src/opcode.rs:803` | `character_record`, our builder |

## The read primitives, so every width below is a fact and not a guess

**[read]** `research/msexe-packet-readers.c` / `msexe-decoders3.c` — each advances the packet
cursor at `pkt+0x24` by exactly this much and throws `_CxxThrowException` on underrun:

| function | width |
|---|---|
| `FUN_1406e8ae0` | 1 |
| `FUN_1406e8b80` | 2 |
| `FUN_1406e8c20` | 4 |
| `FUN_1406e8f10` | 8 |
| `FUN_1406e9170(pkt, dst, n)` | n |
| `FUN_1406e9050` | `u16` length + that many bytes |

## First: a premise in the task is wrong, and the correction is in our favour

**[read]** `research/msexe-setfield.md:352` says `FUN_140304b20` "is the only caller of the
already-documented `FUN_140302e30`". It is not. `research/msexe-charrecord.c:384` shows
`FUN_1403094b0` — the character-list record decoder — calling
`FUN_140302e30(param_1, param_2, 0)` as its first statement, and
`research/msexe-charrecord.c:113` shows `FUN_14108bdf0` (the character list inside `0x0010`)
calling `FUN_1403094b0`. `docs/character.md` lists both functions in its character-manager
table.

So `FUN_140302e30` has **at least two callers**: the list record and the `SetField` record.
That is the whole basis of the reuse — the two paths do not merely have similar stat blocks,
they call the *same function*, so the stat block is byte-identical by construction.

## The two `param_3` branches consume identical bytes

**[read]** `FUN_140302e30` forks on `param_3` for its first nine fields only.

* `param_3 == 0` (what `FUN_1403094b0` passes): `u32,u32,u32,13B,u8,u8,u32,u32,u32` → **39 bytes**, all stored.
* `param_3 != 0`: `u32(discarded),u32,u32,13B(to a stack buffer),u8,u8,u32,u32,u32` → **39 bytes**, most discarded.

Same widths, same order, same total. **[inferred, but tightly]** Whatever `FUN_140304b20`
passes for `param_3`, the wire layout does not change; the only consequence is that a
non-zero `param_3` means the client *ignores* the `characterId` it reads and must be getting
the identity from somewhere else in the packet.

## The table: field by field, our builder against `FUN_140302e30`

Offsets are byte offsets **into the record**, for an extended-SP job with zero SP pools
(which is what we send: `Character::default().job == 0`). "dst" is where the client stores
it, which is what identifies several of the fields.

### Part 1 — the stat block, `FUN_140302e30` (offsets 0..107)

| off | field | type | dst | we emit it? | source of truth |
|---:|---|---|---|---|---|
| 0 | `characterId` | u32 | `+0x00` | yes, `chr.id` | **[read]** charstats.c:40 |
| 4 | `characterIdForLog` | u32 | `+0x04` | yes, `chr.id` again | **[read]** width/order; **[inferred]** name and value |
| 8 | `worldIdForLog` | u32 | `+0x08` | yes, `world_id` | **[read]** width/order; **[inferred]** name |
| 12 | `name` | 13B fixed | `+0x0c` | yes, NUL-padded | **[read]** `FUN_1406e9170(...,0xd)`; measured on screen |
| 25 | `gender` | u8 | `+0x19` | yes | **[read]** width; **[inferred]** name |
| 26 | `skin` | u8 | `+0x1a` | yes | **[read]** width; **[inferred]** name |
| 27 | (zero) | u32 | `+0x1b` | yes, `0` | **[read]** width; **[inferred]** "always zero" |
| 31 | `face` | u32 | `+0x1f` | yes, `chr.face` | **[inferred]** — see disagreement **D4** |
| 35 | `hair` | u32 | `+0x23` | yes, `chr.hair` | **[inferred]** — see **D4** |
| 39 | `level` | u32 | obf `+0x2b` | yes, `chr.level` | **[read]** width; **measured** — level drew correctly |
| 43 | `job` | u16 | `+0x33` | yes, `chr.job` | **[read]** and *proved* by the mask branch below |
| 45 | `str` | u16 | `+0x3b` | yes | **[read]** width; **[inferred]** which of the four |
| 47 | `dex` | u16 | `+0x43` | yes | same |
| 49 | `int` | u16 | `+0x4b` | yes | same |
| 51 | `luk` | u16 | `+0x53` | yes | same |
| 53 | `hp` | u32 | obf `+0x5f` | yes | **[read]** width; **[inferred]** which of the four — **D5** |
| 57 | `maxHp` | u32 | obf `+0x6b` | yes | same |
| 61 | `mp` | u32 | obf `+0x77` | yes | same |
| 65 | `maxMp` | u32 | obf `+0x83` | yes | same |
| 69 | `ap` | u16 | `+0x8b` | yes, `chr.ap` | **[read]** width; **[inferred]** name |
| 71 | **SP fork** | see below | `+0x93` / `+0xd7` | yes, both sides | **[read]** — the strongest evidence in the record |
| 72 | `exp` | u64 | `+0x9b` | yes, `0` | **[read]** `FUN_1406e8f10` |
| 80 | `fame` | u32 | obf `+0xb7` | yes, `0` | **[read]** width; **[inferred]** name |
| 84 | (unnamed) | u32 | obf, into the heap object at `+0xfb` | yes, `0` | **[read]** charstats.c:179 + the mangling loop |
| 88 | `portal` | u8 | `+0x10b` | yes, hardcoded `0` | **[read]** width; **[inferred]** name — **D6** |
| 89 | `subJob` | u16 | `+0x10c` | yes, `0` | **[read]** width; **[inferred]** name |
| 91 | (unnamed) | u8 | `+0x10e`, widened to u32 | yes, `0` | **[read]** |
| 92 | FILETIME | 8B raw | `+0x112` via `DAT_1432625b0` | yes, `0` | **[read]** `FUN_1406e9170(...,8)` |
| 100 | time, **high** dword | u32 | pairs with the next | yes, `0` | **[read]** — **D2** |
| 104 | time, **low** dword | u32 | `+0x126` via `DAT_1432625b0` | yes, `0` | **[read]** — **D2** |

**The SP fork, at offset 71.** **[read]** `charstats.c:124-168`: the client re-reads the `u16`
it stored at `+0x33` and bit-tests it against `0x1c0701c01` over `job-100`, the same mask over
`job-200`, `0x701c01` over `job-300` / `job-400` / `job-500`, plus `job-0x1ae < 10`. Decoding
the masks by hand gives bits `{0,10,11,12,20,21,22,30,31,32}` — jobs
100/110/111/112/120/121/122/130/131/132 and the same shape at 200/300/400/500, plus 430–439.
Job `0` also takes it. Those jobs (and only those) take the branch that reads **no `u16`** and
calls `FUN_1402cb0d0`; everything else reads a plain `u16 sp`.

**[read]** `FUN_1402cb0d0` (`msexe-newchar-scan.c:1957`) is `u8 count`, then `count ×
(u8, u32)`, and it sums the `u32`s into a running total at `param_1+0x18` — which is why the
`u32` is the SP amount and the `u8` is the per-job level, rather than the other way round.

`crates/net/src/opcode.rs:720` (`uses_extended_sp`) implements exactly this, and
`opcode.rs:1622` pins the job list by test. This is the one place in the record where a wrong
field offset anywhere above would show up as a shifted decode, so it is load-bearing evidence
that offsets 0..70 are right.

**Stat block total: 108 bytes** for an extended-SP job with zero pools; **109** for a plain
`u16 sp` job; **108 + 5×pools** in general.

### Part 2 — `FUN_1403094b0`'s own four fields (offsets 108..131)

**[read]** `msexe-charrecord.c:385-392`. These belong to the *list* record decoder, not to the
stat block.

| off | type | dst | we emit | source |
|---:|---|---|---|---|
| 108 | u32 | `+0x319` | `0` | **[read]** |
| 112 | u64 | `+0x31d` | `0` | **[read]** |
| 120 | u32 | `+0x325`, **also copied to `+0x308`** | `chr.map_id` | **[read]** offset; **[inferred, undocumented]** that this is the map — **D3** |
| 124 | u64 | `+0x329` | `0` | **[read]** |

### Part 3 — the avatar look, `FUN_1402ee8d0` (offsets 132..326, base `record+0x136`)

**[read]** `research/msexe-avatarlook.c`, offsets relative to the look base.

| off | field | type | dst | we emit | source |
|---:|---|---|---|---|---|
| 132 | gender | u8 | `+0x20` | yes | **[read]** width |
| 133 | skin | u8 | `+0x21` | yes | **[read]** width |
| 134 | (unused) | u32 | `+0x25` | `0` | **[inferred]** — **D7** |
| 138 | face | u32 | `+0x29` | `chr.face` | **[inferred]** — **D7** |
| 142 | job | u32 | `+0x1bd` | `chr.job` widened | **[read]** — the destination is far from the look block |
| 146 | (discarded) | u8 | — | `0` | **[read]** — return value never stored |
| 147 | hair | u32 | `+0x39` | `chr.hair` | **[read]** — equipment index 0, unreachable from the pair loop |
| 151 | equipment | `(u8 slot, u32 id)*` then `0xFF` | `+0x39 + slot*4` | yes, `chr.equips` | **[read]** — loop guards `(u8)(slot-1) < 0x1f`, so slots 1..31 only |
| … | second map | `(u8, u32)*` then `0xFF` | `+0xb9 + slot*4` | terminator only | **[read]** — **two** maps, not the reference's three |
| … | 4 × u32 | u32 | `+0x2d,+0x31,+0x35,+0x1c1` | `0` | **[read]** |
| … | u32 mod 360 | u32 | `+0x1c5` | `0` | **[read]** — negative clamps to 0 |
| … | u8 | u8 | `+0x1c9` | `0` | **[read]** |
| … | u32 | u32 | `+0x1ca` | `0` | **[read]** |
| … | 4B | raw | `+0x1b9` | `0` | **[read]** |
| … | 128B | raw | `+0x139` | `0` | **[read]** |
| … | u32 | u32 | `+0x1d2` | `0` | **[read]** |
| … | 13B | raw | `+0x1d6` | `0` | **[read]** |

## How many bytes we can already produce

**[read]** from `crates/net/src/opcode.rs:803` and pinned by `opcode.rs:1817`
(`assert_eq!(character_record(&chr, 0).len(), 327)`):

```
stat block   108  (extended-SP job, 0 SP pools)   +1 if the job takes the plain u16 sp
trailer       24
avatar look  195  (both 0xFF maps empty)
             ---
             327  bytes
```

**It is variable, on three axes** — all three are in our builder and all three are exercised
by `opcode.rs:1597`:

```
len = 327
    + 5 × equips.len()          each (u8 slot, u32 itemId) pair
    + 5 × sp_pools              each (u8 jobLevel, u32 sp) pair
    + 1 if !uses_extended_sp(job)
```

A character created through our own create path carries 4 equips (top 5, bottom 6, shoes 7,
weapon 11 — `opcode.rs:495`), so **a real stored character encodes to 347 bytes**, not 327.
Only the empty-list default is 327.

**For `SetField`, the number that matters is 108, not 327.** The 108 bytes are the shared
function. The other 219 belong to `FUN_1403094b0`, which is *not* on the `SetField` path.

## Same stat block, or merely similar? — checked, not assumed

**Same stat block. [read]** Both records are decoded by the same `FUN_140302e30`, reached
from two different outer functions:

```
0x0010 character list :  FUN_14108bdf0 -> FUN_1403094b0 (285 bytes) -> FUN_140302e30
0x01A0 SetField       :  FUN_142097f80 -> FUN_140304b20 (18525 bytes) -> FUN_140302e30
```

**Different record. [read + inferred].** `FUN_1403094b0` is 285 bytes and its whole tree is
about 58 read call sites (30 in the stat block, 4 of its own, ~24 in the look).
`FUN_140304b20` is 18525 bytes, and the task reports 117 reads **in its own body** — a figure
I could not verify, because **the body of `FUN_140304b20` is not on disk in any form**. I
checked: `research/msexe-packet-fields.txt` is outbound builders only (its own header says
so, and it contains no inbound decoder at all — `FUN_1403094b0` and `FUN_140302e30` are
equally absent from it, which is the positive control for that negative). Every hit for
`140304b20` in `research/` is a *call site*, never a `// FUN_140304b20 @ ...` dump header,
and that header convention is what every other decompiled function in `research/` uses.

So: the stat block is shared and byte-identical; everything after it is a different, roughly
four-times-larger structure that has never been read. **The docs do not settle what follows
the stat block in a `SetField`, and nothing on disk can.**

## The disagreements and ambiguities — the part that will break a `SetField`

Ordered by how much damage each can do.

### D1 — 219 of our 327 bytes have no evidence of belonging to a `SetField` at all
**[inferred, high confidence]** The 24-byte trailer and the 195-byte avatar look are read by
`FUN_1403094b0` and `FUN_1402ee8d0`, neither of which is known to be on the `FUN_140304b20`
path. Appending `character_record()` wholesale to a `SetField` is a bet that a 285-byte
decoder and an 18525-byte one share their layout past the one function they demonstrably
share. Reuse the **first 108 bytes** and treat everything after as unknown.

### D2 — the last 8 bytes of the stat block are one time value sent high-half first
**[read]** `charstats.c:250-252`. The client reads `local_5c` first and `local_60` second,
then hands `&local_60` — an 8-byte span whose low dword is `local_60` and whose high dword is
`local_5c` — to `DAT_1432625b0`, the same converter used for the raw 8-byte FILETIME at
offset 92. So the **first** `u32` on the wire is the **high** half. Our docs and our builder
both call this "u32, u32" with no order attached. Harmless today (we send zeros); wrong the
moment anyone puts a real timestamp there.

### D3 — `map_id` sits in trailer slot 3 on no recorded evidence
**[read]** `opcode.rs:845` writes `chr.map_id` into the `u32` at record offset 120 (client
`+0x325`, which `FUN_1403094b0` then copies to `+0x308`). **[read]** Nothing in
`docs/character.md`, in `opcode.rs`'s own doc comment, or in either commit that touched it
(`493e31a`, `8bf19fd`) says why. The doc comment describes the trailer only as
"`u32, u64, u32, u64`". The one test that covers it,
`the_start_map_reaches_the_character_record` (`opcode.rs:1838`), scans the whole 327-byte
record for *any* `u32` equal to `1` — it passes no matter which slot the map is in, and for
`map_id == 1` it can pass on an unrelated field. This is the least-supported placement in
everything we build, and it is precisely the field `SetField` exists to deliver.

### D4 — `face` and `hair` in the **stat block** are named from the reference only
**[read]** The client stores offset 31 at `+0x1f` and offset 35 at `+0x23` and we have not
read any consumer of either. **[inferred]** The names come from the reference encoder
(`docs/character.md:29-32`). The character-select render is driven by the *avatar look*, not
by these, so a swap here would have been invisible in the run that validated the record. It
would not necessarily be invisible in a map.

### D5 — which of the four obfuscated `u32`s is `hp` vs `maxHp` was never discriminated
**[read]** Four identical obfuscated stores at `+0x5f/+0x6b/+0x77/+0x83`. **[inferred]** The
`hp, maxHp, mp, maxMp` assignment is the reference's. Our default sends `hp == max_hp == 50`
and `mp == max_mp == 5`, so a pairwise swap is invisible today and stays invisible until a
character takes damage — which is exactly the situation a `SetField` creates.

### D6 — `portal` is hardcoded `0` and `Character` has no field for it
**[read]** `opcode.rs:835` pushes a literal `0`; the struct at `opcode.rs:646` has no portal
member. Fine for a list entry. For a `SetField` the spawn portal is a real input, so this is a
model gap, not a layout bug.

### D7 — the avatar look's "unused, face" pair is a reference name, not a read
**[read]** `avatarlook.c:19-22` stores the two `u32`s at `+0x25` and `+0x29`; neither
destination tells you which is which. **[inferred]** The split comes from the reference's
`0, face, job, pad, hair`. Contrast `hair`, where the argument *is* decisive: the pair loop
guards `(u8)(slot-1) < 0x1f` and therefore cannot write index 0, so the standalone `u32` at
`+0x39` must be it. `face` has no equivalent argument.

### D8 — `characterIdForLog` is a value guess
**[read]** width and position. **[inferred]** that echoing `chr.id` is right. No consumer read.

### D9 — a non-zero `param_3` would make the client discard the id
**[read]** the `else` branch of `charstats.c` throws away the first `u32` and the name.
**[inferred]** If `FUN_140304b20` passes non-zero, a `SetField` record must carry the identity
somewhere else, and our habit of putting `chr.id` at offset 0 becomes decorative. The byte
count is unaffected either way, so this is a semantics risk, not a framing risk.

### Not a disagreement
Every field the client reads in `FUN_1403094b0`'s tree is emitted by our builder, in order,
at the right width, with nothing extra — `opcode.rs:1507` (`read_character_record`) walks the
client's sequence and asserts it lands exactly on the end of the body, across the SP fork and
both equipment maps. The problems above are all about *meaning* and *scope*, not presence.

## What to do next, in cost order

1. **Decompile `FUN_140304b20` and read its first ten reads.** One question decides
   everything: does it call `FUN_140302e30` immediately, or is there a header in front of it?
   If there is a header, our 108 bytes are not at offset 0 of the record and nothing else
   matters.
2. **Read its `param_3`** at the `FUN_140302e30` call. Decides D9.
3. **Read whether it calls `FUN_1402ee8d0`, and where.** Decides how much of the remaining
   219 bytes is reusable.
4. Only then send anything. `crates/world/src/session.rs:91` currently answers the migration
   hello with `set_field_head(..., characterData = 0)` plus 256 zero bytes — the branch the
   run of 2026-08-19 proved faults. The next attempt needs `characterData = 1` and a record
   whose head is at least the 108 bytes above.
