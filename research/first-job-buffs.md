# First-job buffs: Warrior, Bowman and Thief

The owner, 2026-08-28: *"I need all 1st job skills of all branches to have their damage
calculation ready and their skills available to test next session."*

`crates/net/src/buff.rs` covers Nimble Feet, Magic Guard and Magic Armor. This is the other
three branches' stat-setting skills, plus the answer on the nine passives.

**Nothing here has been watched on a client.** Every row is read off `gm-handbook/skills.txt`
and cross-checked against the client's own per-level tooltip text; every bit is placed by a
static instrument with the controls written down below. Same status as the Magician pair.

---

## 1. The answers, in one table

| skill | id | CTS bit(s) | value | duration | confidence |
|---|---|---|---|---|---|
| **Iron Body** | `1001000` | **86** `PDD` | `indiePddR` is a **PERCENT** and bit 86 is a **FLAT add** - the server must resolve it | `time` 300..600 s | bit **[L]**, unit **[L]**, resolution **[I]** |
| **Focus** | `3001000` | **88** `ACC` + **89** `EVA` | `indieAcc` / `indieEva`, flat points | `time` 70..300 s | 88 **[L]**, 89 **[D]** |
| **Dark Sight** | `4001001` | **99** `DarkSight` + **92** `Speed` | a flag (`1`), and `-speed` | **toggle**, no `time` node | 99 **[L]**, the negation **[D]** |
| **Disorder** | `4001000` | **none** | a debuff on the **mob** | `time` 10..30 s | **[L]** it is mob-directed |

And the passives: **the client applies them itself, and the server sends nothing** -
except the two that change a number the server owns. §5.

---

## 2. The instrument: a CTS index -> name table, re-derived with five controls

`research/magic-damage.md` §7.2 established how a CTS bit gets named: `.rdata` holds the
plain-ASCII temporary-stat names, nothing in data points at them, and they are `lea`'d by
four enormous unrolled functions whose blocks look like

```asm
140afe4b0  mov  dword [rsp+0x6c0], 0x61      ; the CTS index, 97
140afe4d3  cmp  dword [rsp+0x6c0], 0x52 / jg ; skip unless > 82
140afe517  lea  r8, "MagicGuard"
```

That pass produced **323 names**. This one produces **408**, and the difference matters -
the 85 it was missing include `86 PDD`, `87 MDD`, `88 ACC`, `89 EVA` and `99 DarkSight`,
i.e. every bit this file needed.

### 2.1 What changed, and why it is not just "run it again"

`CLAUDE.md`: *"re-running the same tool is not a second opinion; changing the question is."*
Two things changed.

* **The names are not in one `.rdata` run.** §7.2 gave a single run and searched it. This
  build has `MagicGuard` at `0x143304ff8` and **`DarkSight` at `0x1432731e0`, 200 KB away**,
  in a completely different block interleaved with a compiler constant pool. A scan bounded
  by the first run cannot see the second, and it will not say so - it will return 323 clean,
  confident names. So the string set here is *every* NUL-terminated identifier-shaped ASCII
  string in the whole of `.rdata`, and the `lea` set is *every* rip-relative `lea` in the
  whole of `.text`. Enumerate, then intersect.
* **The index pairing is structural rather than nearest-neighbour.** The index is taken from
  the last preceding `mov dword [rsp+D], imm32` **whose destination slot `D` is also the
  slot compared against `0x52` in the same window**. Both `disp8` and `disp32` encodings of
  both instructions are decoded. That is the block shape §7.2 wrote down, used as a filter
  instead of as a description.

### 2.2 The controls, all four kinds

```text
rip-relative lea sites in .text : 211080
  ... whose target is a string  : 16493
  ... paired with any mov       : 1986
  ... STRUCTURAL (slot cmp'd 82):  880
distinct indices                :  408
indices carrying >1 name        :    0
index range                     : 83 .. 502
```

1. **Self-consistency.** 880 pairings, **zero** indices that got two different names.
2. **Positive controls.** The five bits §7.3 had already named by a different route -
   `92 Speed`, `96 Booster`, `97 MagicGuard`, `98 IronWill`, `100 PowerGuard` - **5 of 5**
   come back identical. Bit 92 in particular was established in `research/buffs.md` §6 from
   a string id and a guard function, which shares no code with any of this.
3. **Both ends agree with the decoder.** The lowest index is 83 and the highest is 502;
   `FUN_140a165f0`'s own bit census has exactly the same two endpoints.
4. **It agrees with a completely different instrument on nine consecutive bits.**
   `FUN_14087c130` reads CTS 83..91 **in index order** into the attacker totals. The name
   table's 83..91 is `WAT, PAD, MAD, PDD, MDD, ACC, EVA, CRT, CRD`; §7.4's independent
   reading of what those totals fields *do* gives attack, attack, magic attack, **the field
   seeded `floor(STR/4)` = Weapon Def.**, **the field seeded `floor(INT/4)` = Magic Def.**,
   **the accuracy line**, -, **crit rate**, **crit damage**. Six of the nine are pinned by
   both, and none disagrees.

> **Named blind spot.** 12 of the 420 indices in 83..502 got no name:
> `120 121 172 200 231 275 316 380 412 413 425 426`. Either their blocks use an encoding
> this decoder misses, or the client has no name for them. None of the five bits this file
> uses is among them, but a future question about one of those twelve must not read the
> silence as "no such stat" - the decoder census has blocks for several of them.

> **Second blind spot.** This instrument says what the client *calls* a bit. It does not by
> itself say what the client *does* with it. That is why each bit below carries a second,
> behavioural piece of evidence, and why bit 89 is [D] and not [L] - it is the one where the
> behavioural half is missing.

### 2.3 The value offsets, re-derived rather than copied

§7.3's table of per-bit `value/reason/expire` offsets was not trusted; the census names each
bit's setter function and each was disassembled:

| bit | name | value setter | writes |
|---:|---|---|---|
| 86 | PDD | `FUN_1408998f0` | `[rbx + 0x440]` |
| 87 | MDD | `FUN_140899080` | `[rbx + 0x47c]` |
| 88 | ACC | `FUN_140896b30` | `[rbx + 0x4b8]` |
| 89 | EVA | `FUN_140897b40` | `[rbx + 0x4f4]` |
| 99 | DarkSight | `FUN_140897670` | `[rbx + 0x65c]` |

All five match §7.3 exactly. **[L]**

---

## 3. Iron Body `1001000` - the bit was right and the unit was not

### 3.1 The data

`type 10`, `processtype 6`, `req 1000001:3`, **max level 20**, `mpCon` **15 at every level**
(it sits on `common`, not on a `level` node), no `cooltime`, and a stray constant `x = 300`
on every level that matches nothing in the tooltip and is **not** the duration (which
varies). `x` is left unread; `research/magician-first-job.md` §6's rule applies - an `x`
without a tooltip that explains it is unverified.

| lv | time (s) | `indiePddR` | tooltip |
|---:|---:|---:|---|
| 1 | 300 | 5 | *MP -15; Weapon Def. **+5%** for 300 sec* |
| 10 | 435 | 14 | *… +14% for 435 sec* |
| 19 | 570 | 23 | *… +23% for 570 sec* |
| **20** | **600** | **25** | *… +25% for 600 sec* |

Levels 1..=19 are `4 + lv` and `300 + 15*(lv-1)` exactly; **level 20 breaks both runs**
(24 predicted / 25 measured, 585 predicted / 600 measured), the same double break Magic
Armor's last row has. A fitted formula would be right for nineteen rows and wrong for the
one a maxed skill uses.

### 3.2 The finding, stated plainly

**The bit is 86 and that is not the interesting part.** The interesting part is that
`indiePddR` is not `indiePdd`.

* Magic Armor's WZ key is **`indiePdd`** - *"Weapon Def. +40"*, flat points.
* Iron Body's is **`indiePddR`** - *"Weapon Def. +5%"*, a ratio.

Those are two distinct entries in the client's own Indie name array, which is a clean
16-byte-stride table in index order at `0x14327ce58`:

```text
0 IndieWAT   1 IndiePAD   2 IndieMAD   3 IndiePDD   4 IndieMDD   5 IndieACC   6 IndieEVA
7 IndieCRT   8 IndieCRD   9 IndieWATR 10 IndiePADR 11 IndieMADR 12 IndiePDDR 13 IndieMDDR
```

**[L]** (and note in passing: §7.4 read this run as `IndieMAD = 1, IndiePDD = 2, IndieMDD = 3`.
It is off by one - the run starts at `IndieWAT`, exactly the same first-entry drop as the
323-name CTS pass. Nothing in this file depends on the numbering, but the correction is
worth having.)

**CTS 86 is a flat add.** `FUN_14087c130` adds bit 86's value to `totals+0x0c`, the field
seeded with `floor(STR/4)`. So `5` on bit 86 means *"+5 Weapon Def."*, not *"+5%"*.

### 3.3 Does `0x007D` carry the Indie space instead? No - [D], with the blind spot

`FUN_140a165f0` is 198 742 bytes and entirely unrolled: 407 standard `u32,u16,u32,u32` stat
blocks plus 67 conditional-extras blocks, each gated on `FUN_1402bf6d0(mask, bit)`. Its tail
was read to the `ret`: the last block is a bit test on `0x6b`, then the mask copy-out and the
epilogue. **There is no loop, no length-prefixed list, and no second array.** The only `raw`
read in the whole function is the 124-byte mask at the top.

> **Blind spot.** This rests on a census of read *shapes* per block. A read performed through
> a helper that is not one of the ten known packet primitives would not be counted as a read
> at all, and `CLAUDE.md` records that this count has been wrong three times by exactly that
> mechanism. What is solid: **no block in this function reads a variable number of bytes**,
> so if the Indie stats ride in this packet they do so at a fixed width under a bit that is
> already counted.

### 3.4 What the server does, and what a run would decide

`crates/net/src/jobbuffs.rs` **resolves the percentage** before it reaches the wire:
`points = weapon_defence * indiePddR / 100`, truncating. `buff_level` takes the character's
Weapon Def. as a required parameter so the resolution cannot be skipped by accident.

The truncation is **[I]** - no client code resolves an `indiePddR`, because on this reading
the client never sees a percentage. Integer truncation is chosen because that is what every
`/100` this project has decoded in the client does (§7.3's `imul`+`sar edx,5`).

**The failure this guards against is silent.** A level-1 Iron Body on a Warrior with 100
Weapon Def. is `5` either way - the percentage and the flat number are the *same number*.
The two only separate as gear improves, and by then nobody is looking. A run that wants to
check it should read the stat window before and after at a **known** Weapon Def.: with
W.Def 100 the level-20 buff must add **25**, with W.Def 400 it must add **100**. If it adds
25 in both cases, the resolution is being thrown away somewhere and the raw percent is
reaching the wire.

---

## 4. Focus `3001000` and Dark Sight `4001001`

### 4.1 Focus - two flat stats, and the columns are not equal

`type 10`, `processtype 6`, `req 3000001:3`, max level 20, no `cooltime`.

| lv | mpCon | time (s) | `indieAcc` | `indieEva` | tooltip |
|---:|---:|---:|---:|---:|---|
| 1 | 8 | 70 | 1 | 5 | *MP -8; Accuracy +1; Evasion +5 for 70 sec* |
| 20 | 16 | 300 | 20 | **25** | *MP -16; Accuracy +20; Evasion +25 for 300 sec* |

Two things to keep:

* **`time` is not a line.** It steps by 10 inside each five-level band and by **20** across a
  band boundary (110->130, 170->195, 235->260). Three discontinuities, not one.
* **`indieEva` is not `indieAcc`.** It runs four ahead at levels 1..19 and five ahead at 20.
  Magic Armor's two defence columns happen to be equal and `buff.rs` keeps them as two arrays
  anyway *in case* they ever diverge; Focus is the case that already has.

Bit **88 = ACC** is **[L]**: the name table plus `FUN_14087c130` putting bit 88 on
`totals+0x18`, which §3.1 identifies as the accuracy line.

Bit **89 = EVA** is **[D]**.

> **Blind spot, quoted.** *"Nothing I found reads `secStat+0x4f4` and applies it to a dodge
> roll."* §7.4's own table leaves `totals+0x1c`'s consumption blank. What places `indieEva`
> on 89 is the name, the position in the in-order 83..91 run, and the setter offset. A run in
> which Focus moves Accuracy but not Avoidability would mean the pair is off by one, and the
> run names which - exactly the Magic Armor §9.3 shape.

### 4.2 Dark Sight - a toggle, a flag, and a Speed penalty that is not in the data

`type 10`, **`processtype 113`**, `req 4001000:3`, max level 20, **no `time` node at any
level**, no `cooltime`. `research/magician-first-job.md` §4 already established that all 16
`processtype 113` skills lack `time` at every level and that all of their descriptions say
the effect is switched on and off by using the skill again - and it named Dark Sight as one
of the only **two** `type 10` skills in the archive with no `time`, the other being Magic
Guard. So `BuffDuration::Toggle`, on the same evidence.

| lv | mpCon | `speed` | tooltip |
|---:|---:|---:|---|
| 1 | 50 | 20 | *MP -50; Disappear into the shadows; **Speed -20** while active* |
| 19 | 32 | 2 | *… Speed -2 while active* |
| **20** | **30** | **absent** | *… **regular movement speed** while active* |

**Bit 99 = DarkSight, [L], and not from the name table alone.** `FUN_14276e0e0` is 42 bytes
and does exactly one thing:

```asm
14276e0ea  mov   edx, [rax + 0x664]     ; the checksum of bit 99's value triple
14276e0f0  lea   rcx, [rax + 0x65c]     ; bit 99's value
14276e0f7  call  0x1401ba9d0            ; the de-obfuscating getter
14276e100  setne cl                     ; -> value != 0
```

An `IsDarkSight()` predicate reading the offset §7.3's table gives for bit 99. Two routes,
one offset. **The value is consumed as a boolean there**, so the server sends `1`.

> **Blind spot.** A whole-image `[reg + 0x65c]` scan returns 56 sites, ten of them the
> `lea rcx,[X+0x65c]` getter idiom, and **one** of those ten was disassembled. If another
> multiplies the value, `1` is a number in the wrong unit rather than a flag. The scan is
> also an exact-displacement match - a wider load at a lower displacement covering `0x65c`
> would not appear in it at all. (`CLAUDE.md`: *"ask whether this operand's byte range covers
> the byte"*.)

**The speed penalty goes on bit 92, negative. [D].** The WZ stores `speed` as a positive
magnitude on ten skills and the sign is never in the data:

```text
1002    Nimble Feet   speed 10   "speed +10 for 30 sec"     <- own speed, UP   (confirmed cast)
4001001 Dark Sight    speed 20   "Speed -20 while active"   <- own speed, DOWN
1201004 Threaten      speed 35   "Enemy Speed -35"          <- the MOB's
2101002 Slow          speed 30   "Enemy Speed -30"          <- the MOB's
```

Nimble Feet is the control and it is the strongest kind available here: same property, same
client, opposite sign, and its `+10` on bit 92 is the one row in this whole area that the owner
has watched work. So Dark Sight's `20` reaches bit 92 as `-20`.

**Bit 99's value is a flag and bit 92's is the magnitude** - each field means one thing. The
alternative (put the magnitude on bit 99 and let non-zero double as "on") works at nineteen
levels and then needs an invented number at level 20, which is how an absent property turns
into a fake value.

**Level 20 has no `speed` node**, so it grants **one** stat where every other level grants
two, and the packet is ten bytes shorter. `CLAUDE.md` records a mob's hit box collapsing
because an absent property was sent as `0`; `0` on bit 92 would be a granted Speed stat worth
nothing, which is a different thing from no Speed stat.

### 4.3 Bit 99 makes the decoder read eight more bytes - and it is the only one of the five that does

The census counts each bit's occurrences in `FUN_140a165f0`. Bits **86, 87, 88, 89, 92 and
97 appear once each** (the standard 87-byte `u32,u16,u32,u32` block). **Bit 99 appears
three times**:

```asm
; occ2, in the conditional-extras pass near the end of the function
140a46025  mov  edx, 0x63              ; 99
140a46032  call 0x1402bf6d0            ; is bit 99 set in the mask?
140a4603c  je   skip
140a46046  READ u32 -> FUN_140896680   ; writes secStat+0x6a4
140a46062  READ u32 -> FUN_140896810   ; writes secStat+0x6b0
; occ3 at 0x140a463f0 is 19 bytes and reads nothing
```

**[L]** So a Dark Sight `0x007D` consumes **8 bytes more** than a Nimble Feet one. They come
out of `buff::TAIL_LEN`'s 64 zero bytes, both are fixed width, and 56 bytes of slack remain.
`jobbuffs::DARK_SIGHT_EXTRA_BYTES` records it so that a future attempt to bisect `TAIL_LEN`
downwards - which `buff.rs` invites - does not do it on a Nimble Feet packet and then apply
the answer to a Dark Sight one.

---

## 5. Disorder `4001000` - a mob debuff, and this server has no packet for it

`type 1`, `processtype 108`, `damage 100`, `attackCount 1`, `mobCount 1`, max level 20.

| lv | mpCon | time (s) | `x` | `y` | tooltip |
|---:|---:|---:|---:|---:|---|
| 1 | 5 | 10 | 5 | 1 | *MP -5; **Enemy's** Attack Power -5; Weapon Def. -1 for 10 sec* |
| 20 | 10 | 30 | 25 | 5 | *MP -10; Enemy's Attack Power -25; Weapon Def. -5 for 30 sec* |

The tooltip says **Enemy's**, and the same archive uses the same wording for Threaten
`1201004`, Slow `2101002` and Amazon's Judgement `3100001`. `x` is the mob's attack-power
drop, `y` is the mob's weapon-defence drop, and `time` is how long it sits **on the mob**.
`damage 100` means the cast is also a 100%-damage single-target attack. **[L]**

**This server has never sent a mob temporary stat.** Enumerating every
`pub const … : u16 = 0x03xx` in `crates/net/src`:

```text
0x03C6 MobEnterField   0x03D1 MobLeaveField   0x03D2 MobChangeController
0x03D9 MobMove         0x03E4 MobCtrlAck      0x03F0 MobHpChange
```

Nothing that carries a mob stat. **[L]**

**No packet was built and none should be** until a mob-stat opcode is decoded. What exists is
`jobbuffs::disorder_debuff(level)` - the four numbers, as data - and `buff_level(4001000, …)`
returning `None` with a test that says why. The damage half of Disorder is a normal attack
and is the attack path's business, not this file's.

---

## 6. The passives: the client applies them, and that is measured

Nine `type 50` skills: Improved HP Recovery `1000000`, Max HP Increase `1000001`, Precise
Strikes `1000002`, Improved MP Recovery `2000000`, Max MP Increase `2000001`, Critical Shot
`3000000`, The Eye of Amazon `3000001`, Nimble Body `4000000`, Keen Eyes `4000001`.

`research/magician-first-job.md` §7 left this open: *"the property names exist in the client
image as UTF-16 literals, which means client code can look them up; that is not a read."*
It is now a read.

### 6.1 The mechanism, and three functions that use it

The client's own lookup chain is three calls:

```text
FUN_1407b3df0(userLocal, ?, skillId, &outRecord)  -> the player's LEVEL in that skill
FUN_14079fe90(record, level)                      -> the per-level node
FUN_1401ba9d0(&node.field, node.field_checksum)   -> the de-obfuscating getter
```

**`FUN_1407b4c10` - attack range.** It switches on the weapon type, seeds a base range
(`0xc8`/`0x12c`/`0x17c`/`0x190`/`0x1c2` = 200/300/380/400/450 px), then:

```asm
1407b4cdc  mov  r8d, 0x2dc6c1      ; 3000001  The Eye of Amazon   (bow / crossbow arm)
1407b4d7d  mov  r8d, 0x3d0901      ; 4000001  Keen Eyes           (claw arm)
1407b4d93  call 0x1407b3df0        ; -> level
1407b4da3  call 0x14079fe90        ; -> level node
1407b4da8  mov  edx, [rax + 0x20c] / lea rcx, [rax + 0x204] / call 0x1401ba9d0
1407b4dba  add  ebx, eax           ; base range + the skill's own value
```

Both skills' **only** per-level column in the WZ is `range`. The client reads one field and
adds it to a range. **[L]**

**`FUN_1407e49b0` - critical rate and critical damage.** Weapon-type gate, then
`FUN_1407b3df0(..., 0x2dc6c0 = 3000000 Critical Shot, ...)`, then it reads **two** per-level
fields (`+0x204` and `+0x14c`) into the caller's **two** out-pointers. Critical Shot's only
per-level columns in the WZ are **`crtX` and `crdX`**. Two fields in the data, two fields
read. **[L]**

**`FUN_1407e4250` - and this one is called from the totals builder itself.** It resolves the
job's Mastery skill (`FUN_1407e3f20()` returns `1100000`, `1200000`, `1200001`, `1300001`, …),
runs the same three-call chain, and switches on the id. Its one caller inside
`FUN_14087c130` is the point: **the client's attacker-totals builder consults its own skill
record while building the totals.** **[L]**

In all three, no packet is in the path. The only thing the server supplies is the skill
**level**, which it already sends in the skill record.

> **Blind spot, and it is a real one.** The enumeration behind this is "every `call`/`jmp`
> into `FUN_1407b3df0`, then the preceding `mov r8d, imm32`": **401 call sites + 7 tail
> jumps, of which 128 carry a constant id and 280 take the id from a register.** So this
> establishes that the mechanism exists and covers Critical Shot, The Eye of Amazon, Keen
> Eyes and the Masteries. It **cannot** produce a per-skill negative: "Nimble Body does not
> appear with a constant id" is not evidence that the client does not apply Nimble Body,
> because two thirds of the call sites are invisible to that scan - and `FUN_1407e4250`,
> which is the most important hit here, is itself one of the register-driven ones.

A separate scan for the raw 4-byte value of every real skill id found `1000000` **441 times**
in `.text`. Almost all of those are the integer one million: the site at `0x140403fed` is
`imul r8d, edx, 0xf4241`, the tail of a division-by-1000001 idiom. Reported so nobody counts
it as evidence later.

### 6.2 The two exceptions

`1000001` Max HP Increase (`mhpR`) and `2000001` Max MP Increase (`mmpR`) change numbers the
**server** owns - `research/user-hit.md` establishes the client computes damage and never
writes HP, and the max values arrive in this server's own stat packet. A percentage the
client folds in locally on top of a max the server already sent is **distinguishable** from
one the server folded in, and `research/magician-first-job.md` §8 experiment A is that
measurement. **It is still unrun**, it costs a login and no launch of its own, and it decides
whether "Max MP Increase works" means the server multiplies or the server does nothing.

`jobbuffs::passive_owner` records the split: seven `Client`, two `Unmeasured`.

---

## 7. WIRE IT LIKE THIS

`crates/net/src/jobbuffs.rs` is built, tested and **not wired**. `cargo test -p net` is
452 passing; `cargo clippy -p net --all-targets -- -D warnings` is clean.

1. **Call `all_granted_by`, never `granted_by`.** Focus and Dark Sight (levels 1..=19) each
   grant **two** stats. `granted_by` returns one, and there is a test in `jobbuffs.rs` that
   pins exactly what it loses. For Dark Sight the dropped half is the Speed penalty - on
   screen that is a working skill with a missing drawback, which nobody reports as a bug.
2. **Iron Body needs the character's Weapon Def.**
   `jobbuffs::buff_level(skill_id, level, weapon_defence)` - the third argument is read by
   Iron Body only and ignored by the other two (there is a test). Pass the same Weapon Def.
   total the stat window shows, *before* the buff. Passing `0` yields a working cast that
   adds nothing, never a refusal.
3. **The off-path must clear every bit the on-path set.** `0x013F` and any expiry sweep have
   to send `0x007E` with **all** of `all_granted_by`'s bits. If Dark Sight's off-path clears
   only bit 99, the player stays permanently slow. Same hazard for Focus and bit 89.
4. **Bit 92 is shared.** Dark Sight and Nimble Feet both set Speed. A cast of one while the
   other is held overwrites it, and the client's own guard `FUN_1429755a0` refuses a second
   Nimble Feet while Speed is held (`research/buffs.md` §6). Decide that on purpose.
5. **Dark Sight is a toggle**, like Magic Guard: `duration == BuffDuration::Toggle`,
   `seconds == 0`, and the only off-switch is the server. Whatever `buff::BuffLevel::granted_by`
   decides about `duration_ms = 0` applies here unchanged - one decision, two skills.
6. **Disorder: do not send anything.** `buff_level` returns `None` for `4001000`. Its damage
   half belongs to the attack path. If the skill has to be castable next session, the honest
   behaviour is to answer the `0x013C`, spend the MP, deal the 100% hit, and grant no stat.
7. **Budget eight extra tail bytes for Dark Sight** - `jobbuffs::DARK_SIGHT_EXTRA_BYTES`.
   The current 64-byte tail covers it with 56 to spare, so nothing needs changing today; it
   matters only if `TAIL_LEN` is ever bisected downwards.
8. **The passives need no packet.** Seven of the nine: send the skill level and stop. The
   two max-pool ones are open - run `research/magician-first-job.md` §8 experiment A before
   writing any code for them, because both answers are actionable and they point opposite
   ways.

### What one launch would settle, and what each outcome means

| watch | outcome | reading |
|---|---|---|
| Iron Body at level 20 with a **known** Weapon Def. | W.Def rises by 25% of it | the resolution works; promote §3.4 to [L] |
| | W.Def rises by exactly **25** regardless of the base | the raw percent is reaching the wire |
| | W.Def does not move | bit 86 is wrong, and `buff.rs`'s Magic Armor doubt is the same doubt |
| Focus at any level | Accuracy **and** Avoidability both rise | 88/89 correct, promote to [L] |
| | only Accuracy rises | 89 is not EVA - the pair is off by one, sweep 89..91 |
| | neither rises | the stat window may show equipment only; cast Magic Armor as the control |
| Dark Sight at level 1 | character turns translucent **and** is visibly slower | 99 and the negation both correct |
| | translucent, same speed | either bit 92 was dropped (see item 1) or the client refuses a negative value |
| | slower, not translucent | `1` is not enough on bit 99 - send the `speed` magnitude instead |
| Dark Sight at level **20** | translucent, **normal speed** | the absent-node handling is right |
| Dark Sight cast twice | second cast turns it **off** | `processtype 113` -> toggle confirmed, for both this and Magic Guard |

---

## Appendix A. The full CTS index -> name table, 83..502

408 of the 420 indices, zero conflicts, five controls reproduced. Missing:
`120 121 172 200 231 275 316 380 412 413 425 426`.

```text
 83 WAT                 84 PAD                 85 MAD                 86 PDD
 87 MDD                 88 ACC                 89 EVA                 90 CRT
 91 CRD                 92 Speed               93 Jump                94 MaxHP
 95 MaxMP               96 Booster             97 MagicGuard          98 IronWill
 99 DarkSight          100 PowerGuard         101 FinalAttack        102 RecoveryUp
103 Invincible         104 SoulArrow          105 DragonBlood        106 WeaponElemCharge
107 ComboAttack        108 ElementAmplification 109 SpellBooster      110 HolySymbol
111 MesoSaver          112 MesoGuard          113 ShadowPartner      114 Chakra
115 Bless              116 Stun               117 Poison             118 Seal
119 Darkness           122 MesoUp             123 ThiefSteal         124 PickPocket
125 BloodyExplosion    126 Thaw               127 Weakness           128 Curse
129 Slow               130 Morph              131 Regen              132 BasicStatUp
133 Stance             134 SharpEyes          135 ManaReflection     136 Attract
137 NoBulletConsume    138 Infinity           139 AdvancedBless      140 IllusionStep
141 Blind              142 Concentration      143 BanMap             144 MaxLevelBuff
145 MesoUpByItem       146 MesoAmountRate     147 Ghost              148 Barrier
149 ReverseInput       150 ItemUpByItem       151 RespectPImmune     152 RespectMImmune
153 DefenseAtt         154 DefenseState       155 DojangBerserk      156 DojangInvincible
157 DojangShield       158 KarmaBlade         159 ElementalReset     160 EventRate
161 ComboDrain         162 RepeatEffect       163 ExpBuffRate        164 StopPortion
165 StopMotion         166 Fear               167 HiddenPieceOn      168 MagicResistance
169 SoulStone          170 Flying             171 Frozen             173 Enrage
174 DrawBack           175 NotDamaged         176 FinalCut           177 HowlingAttackDamage
178 BeastForm          179 Dance              180 EMHP              181 EMMP
182 EPAD               183 EMAD               184 EPDD              185 Guard
186 RidingExpireInfoSave 187 NextSpecificSkillDamageUp 188 GravityConstraint 189 BeastFormMaxHP
190 Dice               191 DamR               192 TeleportMasteryOn 193 CombatOrders
194 Beholder           195 DispelItemOption   196 Inflation         197 OnixDivineProtection
198 Web                199 TimeBomb           201 Thread            202 Team
203 Explosion          204 BuffLimit          205 STR               206 INT
207 DEX                208 LUK                209 DispelItemOptionByField 210 DarkTornado
211 WeaknessMdamage    212 Frozen2            213 Shock             214 IncMaxHP
215 IncMaxMP           216 HolyMagicShell     217 KeyDownTimeIgnore 218 ArcaneAim
219 MasterMagicOn      220 AsrR               221 TerR              222 DamAbsorbShield
223 DevilishPower      224 SpiritLink         225 AsrRByItem        226 Event
227 CriticalBuff       228 DropRate           229 PlusExpRate       230 ItemInvincible
232 ItemCritical       233 ItemEvade          234 Event2            235 DDR
236 IncTerR            237 IncAsrR            238 DeathMark         239 UsefulAdvancedBless
240 Lapidification     241 VenomSnake         242 CarnivalAttack    243 CarnivalDefence
244 CarnivalExp        245 SlowAttack         246 PyramidEffect     247 HollowPointBullet
248 KeyDownMoving      249 IgnoreTargetDEF    250 ReviveOnce        251 Invisible
252 EnrageCr           253 EnrageCrDamMin     254 Judgement         255 DojangLuckyBonus
256 PainMark           257 Magnet             258 MagnetArea        259 GuidedArrow
260 StraightForceAtomTargets 261 TempSecondaryStat 262 CoalitionSupportSoldierStorm 263 GrandCross
264 DropPer            265 VampDeath          266 AntiMagicShell    267 LifeTidal
268 HitCriDamR         269 PartyBarrier       270 SpecialAction     271 VampDeathSummon
272 StopForceAtomInfo  273 SoulRageCount      274 PowerTransferGauge 276 BossShield
277 MobZoneState       278 GiveMeHeal         279 TouchMe           280 Contagion
281 IgnoreAllCounter   282 IgnorePImmune      283 IgnoreAllImmune   284 IgnoreAllAbout
285 FinalJudgement     286 FireAura           287 VengeanceOfAngel  288 HeavensDoor
289 Preparation        290 BullsEye           291 IncEffectHPPotion 292 IncEffectMPPotion
293 BleedingToxin      294 IgnoreMobDamR      295 Asura             296 FlipTheCoin
297 UnityOfPower       298 Stimulate          299 ReturnTeleport    300 DropRIncrease
301 IgnoreMobpdpR      302 BdR                303 FireBomb          304 HalfstatByDebuff
305 SetBaseDamage      306 EVAR               307 NewFlying         308 EventPointAbsorb
309 EventAssemble      310 ACCR               311 DEXR              312 Translucence
313 SoulMP             314 FullSoulMP         315 SoulSkillDamageUp 317 Restoration
318 Reincarnation      319 ReincarnationMission 320 ReincarnationOnOff 321 DotBasedBuff
322 SixthDotBasedBuff  323 BlessEnsenble      324 ComboCostInc      325 NaviFlying
326 Holding            327 QuiverCatridge     328 ImmuneBarrier     329 CriticalGrowing
330 LastUseSkillAttr   331 QuickDraw          332 BowMasterConcentration 333 TimeFastABuff
334 TimeFastBBuff      335 GatherDropR        336 AimBox2D          337 DebuffTolerance
338 TearsOfFairy       339 DotHealHPPerSecond 340 DotHealMPPerSecond 341 PreReviveOnce
342 SetBaseDamageByBuff 343 LimitMP           344 ReflectDamR       345 MHPCutR
346 MMPCutR            347 SelfWeakness       348 FlareTrick        349 DamageReduce
350 KnockBack          351 AddAttackCount     352 ComplusionSlant   353 ShieldAttack
354 AttackCountX       355 BombTime           356 NoDebuff          357 NightLordMark
358 WizardIgnite       359 FireBarrier        360 ChangeFoxMan      361 PairingUser
362 MastemaGuard       363 QuiverFullBurst    364 FieldGimmickFear  365 ImmuneStun
366 RandAreaAttack     367 RandAreaAttack2    368 NextAttackEnhance 369 PirateDesire
370 ViperTimeLeap      371 BladeStance        372 DebuffActiveSkillHPCon 373 DebuffIncHP
374 BowMasterMortalBlow 375 Fever             376 RpSiksin          377 TeleportMasteryRange
378 FixCoolTime        379 IncMobRateDummy    381 Stigma            382 HeavensDoorNotTime
383 TransformOverMan   384 BulletParty        385 LoadedDice        386 BishopPray
387 Warrior_AuraWeapon 388 Warrior_AuraWeaponStack 389 Wizard_OverloadMana 390 NightLord_SpreadThrow
391 Shadower_ShadowAssault 392 CreateEventMeso 393 MesoRanger_MesoDizerX 394 FifthAdvWarriorShield
395 SplitArrow         396 FreudBlessing      397 OutSide           398 Shadower_Assassination
399 ConvertAD          400 EtherealForm       401 ReadyToDie        402 Cr2CriDamR
403 HitStackDamR       404 BuffControlDebuff  405 DispersionDamage  406 BuffIconNoShadow
407 CoalitionSupportWarplaneBuffIcon 408 AntiMagicShellByJewelMaking 409 DamageRateSetUpForApc
410 HolyAdvent         411 DiscoveryWeather   414 HolyMagicShellReUse 415 HeroComboInstinct
416 RescuedSoldierBuffIcon1 417 RescuedSoldierBuffIcon2 418 FireBirdSupport 419 FireBirdSupportActive
420 AuraOfLife         421 MemoryOfJourney    422 DecBaseDamageDebuff 423 LimitConsumeDebuff
424 LimitEquipStatDebuff 427 ZeroDamageDebuff 428 FifthGoddessBless 429 CommonItemSkillContinuous
430 MinigameStat       431 NoviceMagicianLink 432 ChampionDoubleUp  433 KeyDownEnable
434 DebuffHallucination 435 IceAura           436 KnightsAura       437 ZeroAuraStr
438 DeathDance         439 ShadowShield       440 RepeatinCartrige  441 ThrowBlasting
442 DarknessAura       443 ZeroEgoWeaponAlpha 444 MobFlashBang      445 IgnorePriestDispel
446 LimitRecovery      447 LimitHealFactor    448 AdminCritical     449 AdminCriticalDam
450 AdminIgnoreTargetDEF 451 AdminBDR         452 AdminBuffTimeR    453 AdminActionSpeed
454 ATScrollPassive    455 DecFinalDamageDebuff 456 IgnoreFrictionOnOff 457 AnimaLotus
458 BossFieldFinalDamR 459 SeaSerpent         460 SerpentStone      461 SerpentScrew
462 FlameSweep         463 EnhancePiercing    464 EnhanceSniping    465 UltimateSniping
466 UltimatePiercing   467 EnhanceQuadrupleThrow 468 UnwearyingRun  469 CrusaderPanic
470 IceAuraZone        471 HolyWater          472 TriumphFeather    473 SixthAngelsRay
474 WarInTheShade      475 HolyBlood          476 OrbitalExplosion  477 TranscendentLight
478 SixthAssassination 479 SixthAssassinationDarkSight 480 SixthFrozenLightning 481 ShadowBurst
482 BossAggro          483 GrabAndThrow       484 DarkCloud         485 FixedSpeedAndJump
486 UserAroundAttackDebuff 487 UserTrackingAreaWarning 488 SixthStormArrowEx 489 SuperFistEnrageStack
490 WildVulcanAdv      491 WildVulcanAdvStack 492 NaturesBelief     493 LifeDeathControl
494 MissileBarrage     495 RapidFire          496 SacredBastion     497 BuffCraftSmith
498 BuffCraftWeapon    499 BuffCraftTailor    500 BuffCraftWood     501 BuffCraftLeather
502 BuffCraftArcForge
```

**This table is not a list of bits the server may set.** It is what the client *calls* each
index. Which of them the client also *acts* on, and in what unit, is a separate question per
bit - the whole of §2.2's fourth control and every "blind spot" note above is about that gap.
Three entries worth flagging for anyone who reaches for them later:
`103 Invincible`, `251 Invisible` and `312 Translucence` are all plausible-looking homes for
a "hide the character" effect and **none of them is Dark Sight's**; `99` is, and it is the
one with a client function that names itself.
