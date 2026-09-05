# The claim index — where each fact is derived, where it is restated, and where a restatement lost its hedges

**Survey only. Nothing outside this file was changed.** 2026-09-04.

The corpus: `research/` holds **144 `.md`** (68 362 lines) and **282 `.c`/`.txt`** dumps;
`crates/` holds **152 `.rs`** (116 768 lines) whose doc blocks restate much of it; `docs/`
holds a third layer of **13 `.md`** (4 573 lines). 111 of the 144 research files are cited
by name from `crates/`.

## How to read this file

* **Owner** = the place the fact was *derived*, or the `crates/` doc block if the fact is a
  constant/offset/layout the code depends on. Everything else is a copy.
* **STRENGTHENED** = a copy states the fact *harder* than its owner — dropped guards,
  `[I]`/`[D]` promoted to `[L]`, "sometimes" → "always", conditional → unconditional,
  candidate → fact. **These are the traps.** A copy that keeps its hedges is a nuisance;
  one that loses them shapes a design and costs a revert.
* Confidence is labelled per row. Where two statements may not be the same claim, that is
  said rather than merged.

### The one structural finding that shapes everything below

**This duplication is semantic, not textual.** An 8-word-shingle containment pass over all
144 research files found exactly **two** pairs above 5 % overlap (`second-job-fields.md` /
`second-job.md` at 6.2 %, `damage-number-draw.md` / `damage-number-suppress.md` at 5.5 %).
So a copy-paste detector, a similarity dedupe, or an eye for repeated paragraphs **will
find none of this.** The same fact is restated in different words, in a different file,
under a different confidence tag. Only a claim-by-claim comparison finds it, which is why
this index is a list of claims rather than a list of files.

> ### Line numbers into `STATUS.md` and `tools/test-server.ps1` are STALE BY DESIGN
>
> Both files were being edited by another agent while this survey ran (`git status` showed
> them modified: +490/−151 across the two). **Every `STATUS.md:NNN` and
> `test-server.ps1:NNN` below has drifted**, and re-checking at the end found the claims
> still present but moved — e.g. the `skilltable::book()` row is now `STATUS.md:491`, its
> self-contradiction now `:377`; the cash-shop heading now `:2289`; the heap rows now `:808`
> and `:2432`; the `-HeapFix` comment now `test-server.ps1:796-797`.
> **Grep for the sentence, not the line.**
>
> **One row is already fixed and must not be re-reported:** *"Skill points are not persisted
> when spent"* is now struck through and marked **CLOSED** at `STATUS.md:488`. §2.13's
> `crates/world/src/skillpoints.rs:22-27` copy — the one that instructs the next agent to
> report it to the owner — was **not** closed with it, and is still live. This is the same split
> the whole index is about, caught in the act.

**Count: 74 duplicated facts catalogued. 29 are STRENGTHENED or stale copies. 20 are live
contradictions.** Coverage and gaps are in §7.

---

# 1. The top ten by blast radius

Ranked by how many places consume the fact and how badly a wrong copy bites. Three of these
are **instruments**, which is why they rank above facts that are merely wrong in more files:
a wrong instrument makes new wrong facts.

| # | fact | blast radius | verdict |
|---|---|---|---|
| **1** | **How many packet-read primitives are there?** | **101 files cite `tools/reads.py`** | Stated as **5, 7, 8, 10 and 11** in five places. The owner's own header disagrees with its own table. §2.1 |
| **2** | **`research/msexe-send-opcodes.txt` names every opcode the client builds** | `CLAUDE.md` elevates this file as *the* control against the `0x00D5` class of mistake | **It misses at least five opcodes the client demonstrably sends, and says nothing about it.** The cause is documented in a different file. §2.2 |
| **3** | `mob+0x8b4` / `0x03F0`'s hp field: **percentage or absolute?** | the field behind the 27 %-bar bug | `net/src/combat.rs:47` module header still says **absolute**; the fix is 550 lines below in the same file. §2.3 |
| **4** | The channel-migrate opcode is **`0x001B`** | a **paste-ready Rust `const`** in a file the shipping code points readers at | Measured `0x001A` on 2026-08-21; `research/change-channel-reply.md:338` still declares `0x001B`. §2.4 |
| **5** | **Seven `crates/` modules declare "THIS MODULE IS NOT WIRED" and all seven have production call sites** | the whole "built is not wired" safety rule | 7/7 stale, mechanically verified. §2.5 |
| **6** | `0x044F` byte 20 carries facing, byte 21 the action | the arrangement that **killed the client on 2026-08-21** | Reverted in code; `research/npc-appear.md:479-489` **still recommends it as its own change**, `[I]` promoted to `[L]`. §2.6 |
| **7** | `CONTROL_RELEASE` "despawns" | shaped the mob-sharing design once already | **Three unfixed copies remain in the file that was fixed**, one tagged `[L]`. §2.7 |
| **8** | `REMOTE_STAT_TAIL_LEN` and the zero-tail offset base | the `0x0224` body; got two clients killed | Stated as **0, 7 and 23**; four places give body offsets with the base dropped. §2.8 |
| **9** | The classic-shop row is **157 bytes** | killed the owner's client three times | The `+6` condition is dropped four times inside its own owner file. §2.9 |
| **10** | The cash shop click **"sends nothing"** | cost three sessions, named in `CLAUDE.md` | Six live statements still say it, incl. the **title** of `cash-shop.md`. §2.10 |

Just below the line, and each worth reading: `0x0467` still called *"a template preload
list"* and tagged `[L]` 267 lines below its own correction (§2.11); `skilltable::book()`
(§2.12); the skill-point ledger doc block that **tells the next agent to report a fixed bug
to the owner** (§2.13); the `689 / 2 / 14` counts in the file `CLAUDE.md` names (§2.14); and the
`0x0073` census, wrong in **twelve** places (§2.15).

---

# 2. STRENGTHENED AND STALE COPIES — the traps

## 2.1 The packet-read primitive table — five cardinalities, and the owner contradicts itself

**Highest blast radius in the repo.** `CLAUDE.md` records that a short read walk *"shipped a
truncated chat packet and killed the client twice"*, and that a five-address list was one of
the two worst wrong answers this project has produced.

| where | says | note |
|---|---|---|
| **OWNER** `tools/reads.py:57-60` (header) | *"**TEN**… **Six real readers** and four bare `jmp` thunks"* | **stale** |
| **OWNER** `tools/reads.py:67-85` (`PRIM` dict) | **11 entries: 7 real readers + 4 thunks** | the data |
| `tools/reads.py:68` (label) | `# the SEVEN real readers` — sits above **six** entries, with the 7th (`0x1406e8fb0` f64) added below under *"The eleventh entry, added 2026-08-24"* | internally inconsistent |
| `tools/listing.py:19`, `:33` | *"its table of the **ten** read primitives"*, *"one of the **ten** primitives"* | **stale**, copied from the header |
| `crates/net/src/script.rs:11-13` | *"cross-checked against the **seven** read primitives"* and then names a **different set of seven** — it includes the `0x1406e8f00` u32 thunk and **omits `0x1406e8fb0` (f64)** | **the dangerous one** |
| `crates/net/src/opcode.rs:1925` | *"no other call to any of the **eight** read primitives"* | stale |
| `crates/net/src/mob.rs:406` | `0x1406e8ef0` is *"an **eighth** packet-read primitive **the project's table does not list**"* | **false** — it is in `PRIM` |
| `CLAUDE.md` | *"five decoder addresses, when there are seven"* | historical record, correct as history |

**Why the `script.rs` copy is the trap and not just untidy:** `reads.py:72-81` says of the
f64 reader that *"**Every read walk that crossed one came back EIGHT BYTES SHORT,
silently**"*, across **71 call sites in 35 functions including `CWvsContext::OnPacket`
itself**. A walk validated against `script.rs`'s list of seven is exactly that walk.

**Owner should be `tools/reads.py`'s `PRIM` dict and nothing else.** Every prose count is a
copy of a number that has changed four times. Confidence: **high** — read directly out of
all five files.

## 2.2 `research/msexe-send-opcodes.txt` — the control that cannot see five of its subjects

`CLAUDE.md` elevates this file by name, as *the* fix for the mistake that cost three
sessions: *"when a user says an action produced no packet, grep for the specific opcode the
action's own code builds — `research/msexe-send-opcodes.txt` names it."*

**It does not name all of them.** Cross-checking all 44 `CLIENT_*` constants in `crates/`:

| opcode | const | in the listing? |
|---|---|---|
| `0x0076` | `CLIENT_SELECT_WORLD` (`opcode.rs:382`) | **absent** |
| `0x0078` | `CLIENT_SELECT_CHARACTER_REQUEST` (`opcode.rs:1341`) | **absent** |
| `0x008A` | `CLIENT_CREATE_CHARACTER_REQUEST` (`opcode.rs:506`) | **absent** |
| `0x032C` | `CLIENT_DROP_PICK_UP` (`world/src/drops.rs:186`) | **absent** |
| `0x00D1` | present, but both builders it lists are the wrong ones — `research/transfer-field-request.md:14` names the real builder `FUN_1418283f0`, **which appears nowhere in the file** |

The file's header admits *"1881 resolved of 1894 call sites"* and 13 `????` rows, and
`FUN_1418283f0` is not among them. **The cause is documented, but in the wrong file:**
`crates/world/src/drops.rs:203-207` explains that `0x032C`'s builder *"is inside `.themida`,
whose `SizeOfRawData` is 0, so those bytes are not in the file at any offset."* The listing
sees only `mov edx,imm / call 0x1406ed520`; a virtualised or register-fed builder is
invisible to it.

**Why this ranks second.** Every other row in this index is a fact that is wrong somewhere.
This is the *instrument* `CLAUDE.md` nominates for proving a negative, and it returns a
**silent false negative** for at least five opcodes the client demonstrably sends. It is the
exact shape the rest of `CLAUDE.md` warns about — *"an instrument that is silently the wrong
version is worse than one that is missing. A missing tool raises; a stale one answers."*
**One header line on the listing fixes it.** Confidence: high (the absences were checked by
extraction; the `.themida` explanation is quoted from `drops.rs`).

## 2.3 `mob+0x8b4` — "absolute HP" survives in the module header of the file that fixed it

| where | says | tag |
|---|---|---|
| **OWNER** `research/mob-hp-bar.md:94-153` §2 | it is a **PERCENTAGE** 0..100 — constructor writes `100.0`, gauge does `value/100*width`, `hpNoticePerNum` is 0 on **all 193** templates | `[L]` |
| **ENFORCED** `crates/net/src/combat.rs:578-602` | `hp_percent`, plus a passing test `the_hp_field_is_a_percentage_not_an_absolute` at `:1051-1062` | — |
| **STALE** `crates/net/src/combat.rs:47` | *"`mob+0x8b4` is the client's copy of a mob's **absolute HP**"* — **module header**, unqualified | — |
| **STALE** `crates/net/src/combat.rs:119` | *"`FUN_141cbb320` **divides** to get the bar percentage"* — owner `:133-135` says it returns `mob[0x8b4]` **directly** when `hpNoticePerNum == 0`, which it is on every template | — |
| **STALE** `research/mob-combat.md:807` (§12 heading, tagged `[L]`), `:857-877` | *"`hp` is absolute, and `template+0x100` is maxHP"*, *"Send the mob's true remaining HP"*. **No retraction, and the file contains no occurrence of `mob-hp-bar`, `hpNoticePerNum` or "percentage" in §12** | `[L]` on a `[D]` sub-claim |
| **THIN** `crates/net/src/names.rs:160` | `"MOB_HP_CHANGE (u32 objectId, u32 hp, u8 showBar)"` — **no unit at all**, in the human-readable opcode table | — |

`CLAUDE.md`: *"When a number reaches the client and draws wrongly, suspect the unit before
the arithmetic."* This is that field. Confidence: **high** (verified `combat.rs:47` myself).

## 2.4 The channel-migrate opcode — a wrong number in a paste-ready `const`

| where | says |
|---|---|
| **OWNER (measured)** `crates/world/src/session/field.rs:456-474` | `MIGRATE_COMMAND_CHANNEL: u16 = 0x001A`, with the **354 ms vs 64 µs** dispatch-time comparison quoted inline — the long one is a socket teardown, and the client reconnected to `127.0.0.1:8486` |
| corroborated | `research/cash-shop-migrate.md:359`, `:440`, `:620`; `STATUS.md:950`, `:1598`, `:1789`, `:1883-1888` |
| **STALE** `research/change-channel-reply.md:28` | *"The strongest candidate is **`0x001B`**"* |
| **STALE** `research/change-channel-reply.md:244` | `mscw MigrateCommand = 0x0016 + 5 = 0x001B` |
| **STALE, and this is the dangerous one** `research/change-channel-reply.md:338` | **`pub const CHANNEL_MIGRATE_COMMAND: u16 = 0x001B;`** — inside a ready-to-paste Rust block |
| **STALE** `research/change-channel-reply.md:368` | same number |

**This is the inverse of a strengthened copy and it is worse.** The file was written
carefully: `:244` says *"[I] … must not be written down as [L]"*, and `:468` says outright
*"Do not let §7.1's `0x001B` harden into a fact because it is written in a code block."*
**The experiment it proposed was run on 2026-08-21 and won.** Nobody came back to the file.
And `crates/world/src/session/field.rs:459` points readers *at* it — *"`research/change-channel-reply.md`
established everything about this packet except its opcode"* — so the citation trail from
the shipping code lands a reader on the wrong number, in a code block, four lines from a
sentence warning them not to trust it.

**One CORRECTION banner at `change-channel-reply.md:1` closes it.** §0-§3 (the handler
`FUN_1415d8c00`, the 7-byte body, the connect-funnel enumeration, the `0x00D2` `[L]` upgrade
at `:276-295`) are still the owner of everything except the number and must be kept.
Confidence: high.

## 2.5 Seven "NOT WIRED" banners, seven sets of production callers — **mechanically verified**

`CLAUDE.md` § *Built is not wired* makes these banners the safety mechanism, and
`crates/world/src/drops.rs:16` and `crates/world/src/mobshare.rs:17` both already warn that
*"a stale 'not wired' banner is its own hazard, because it invites the next reader to"*
re-do finished work. Every banner in the tree is now that hazard:

| module declaring itself unwired | a real production call site |
|---|---|
| `crates/net/src/abilityup.rs:7` "# NOT WIRED" | `world/src/session/ability.rs:70`, `:81`; `session/gm.rs:887` |
| `crates/net/src/party.rs:9-13` "Nothing calls anything in this file" | `world/src/session/party.rs:54`, `:101`, `:148` |
| `crates/net/src/stats.rs:6-10` "Nothing in `crates/world/` sends any of this" | `world/src/session/ability.rs:111`, `:207`; `session/buff.rs:146` |
| `crates/world/src/party.rs:15-19` "Nothing calls anything in this file" | `world/src/fields.rs:211`, `:226`, `:246`; `session/party.rs:89`, `:126` |
| `crates/world/src/secondjob.rs:15-19` | `world/src/session/combat.rs:590`, `:592`; `session/npc.rs:183`, `:1020` |
| `crates/world/src/itemrecovery.rs:13` | `world/src/session/consume.rs:110`; `session/regen.rs:162`, `:163` |
| `crates/world/src/mobattack.rs:7` | `world/src/session/combat.rs:129` |

Method: enumerate every module whose head matches `# (THIS MODULE IS )?NOT WIRED`, then
grep `<module>::<fn>` across `crates/` excluding the module itself, `lib.rs` (a bare `mod`
declaration) and `tests.rs`, and excluding doc-comment lines. Confidence: **high** — call
sites read individually.

Non-module "unwired" claims in `research/` that the same check falsifies: `mob-share.md:47`
(`0x03D9` *"no production caller"* → `world/src/session/combat.rs:94`),
`remote-move-verification.md:14-19` (banner *"STILL NOT ON A WIRE"*, `0x0293` *"never been
sent"* → `session/multiplayer.rs:413`), `world/src/damage.rs:19` (*"Nothing here is wired"*
→ `session/combat.rs:1043`, `:1276`), `mob-attack-skills.md:294`, `item-recovery.md:7`
and `:323`, `second-job.md:10`, `skill-point-ledger.md:10`, `first-job-buffs.md:423`
(`jobbuffs` has 2 call sites), plus `taxi` (10), `jobguide` (7), `remoteattack` (4) and
`userpool` (6), each of which is declared unwired somewhere in `research/`.

## 2.6 `0x044F` bytes 20/21 — a reverted change is still recommended, with `[I]` promoted to `[L]`

| where | says | tag |
|---|---|---|
| **OWNER (measured)** `crates/net/src/opcode.rs:1540-1580` (builder) and `:2980-3019` (test) | ships `b[20] = 0`, `b[21] = u8::from(npc.f == 0)`. The comment carries the two Sera bodies differing in exactly these two bytes, the `0x009E`, two C++ throws and `CLIENT FAULT 0xC0000005`, and ends *"when a static report contradicts what the screen did, **the screen wins**"* | measured |
| original, correctly hedged | `research/npc-spawn.md:304` — read 7 = *"`f` (facing)"* | `[I]` |
| **STRENGTHENED** `research/npc-appear.md:479-489` | *"They disagree, and the byte shapes back `npc-spawn.md`… **[L]** for the stores… **Recommended, as its own change: send `0` for byte 21 and the facing bit for byte 20.**"* | `[L]` / `[D]` |

`research/npc-appear.md:11` carries a banner saying it is *"superseded in two places by
`research/npc-fade.md`"* — **and §7.1 is not one of the two.** So the file that recommends
the arrangement that killed the client on 2026-08-21 **still recommends it**, as a discrete
actionable change, with `[I]` promoted to `[L]` on the way. `opcode.rs:2980` names
`research/npc-appear.md` as its source; `npc-appear.md` does not name the outcome.

This is the brief's `CONTROL_RELEASE` shape exactly, one file over, and still open.
Confidence: high.

## 2.7 `CONTROL_RELEASE` — three unfixed copies remain in the file that was fixed

The case from the brief. The retraction landed at `crates/net/src/mobmove.rs:122-165`; the
flat copies 180 lines below were missed because
`research/control-release-does-it-despawn.md:366-369` listed the sites to fix and **did not
list `341-352`**.

| where | text | status |
|---|---|---|
| **OWNER** `research/control-release-does-it-despawn.md:1, 5, 56, 176, 187` | *"does not despawn a mob in the field"*; the erase is guarded by the in-field flag; unreachable for every mob this server creates | `[D]` |
| `crates/net/src/mobmove.rs:122-165` | correct — retraction written in place | fixed |
| **STRENGTHENED** `crates/net/src/mobmove.rs:341-352` | *"Level 0: **remove the mob from the client's pool**… it is a despawn: … erases the mob from all three pool containers. **[L]**"*, *"deleting the mob is what happens"* | **guards dropped, conditional made unconditional, tagged `[L]`** |
| `crates/net/src/mobmove.rs:250` | *"here **despawns**"*, on `mob_change_controller`, the non-spawning form | stale |
| `crates/net/src/mobmove.rs:261-262`, `:351` | `debug_assert!` message *"level 0 despawns the mob rather than releasing it"* | stale |
| `crates/net/src/mobmove.rs:765-766` | doc *"Level 0 is a despawn"*; test named `release_is_five_bytes_and_is_a_despawn` | stale |

**Live consumer:** `crates/world/src/session/combat.rs:330` calls `mob_release_controller`
in production for control rotation, and the doc block on that exact function still says it
deletes the mob. Note `:250`/`:351` on `mob_change_controller_**spawning**` **is** correct —
that path really does erase (§5 of the owner). Confidence: **high**.

## 2.8 `REMOTE_STAT_TAIL_LEN` and the zero-tail offset base

Two coupled claims. `crates/net/src/userpool.rs:1034-1039` records that *"The tail was 7 and
it is 23, and the sixteen it was missing **killed two clients**."*

**(a) The tail length is stated as 0, 7 and 23.**

| value | where | status |
|---|---|---|
| **23** — OWNER | `crates/net/src/userpool.rs:482` (doc + assert `:1040`); derived `research/avatar-look-loops.md:21-23`, `:183`, `:242-244`; independently re-confirmed `research/remote-stat-mask-gating.md:24-37` | current |
| **7** | `research/user-enter-verification.md:625`, `:644-645`, `:658`, `:660`, `:782`, `:20` — including an `assert_eq!(REMOTE_STAT_TAIL_LEN, 7, "…all unconditional")` and a test named `the_remote_stat_block_is_131_bytes_not_124` | **superseded, no banner** |
| **0** | `research/user-enter-field.md:172`, `:26`, `:221`, `:525`, `:564` — *"raw 124 … then job at 179"*, *"Total minimum: **508** bytes"* | **superseded twice, no banner** |

Both stale files state their number as `[L]`.

**(b) The offsets in the owner's table are zero-tail, and four copies drop the base.**
`crates/net/src/userpool.rs:1007-1009` names the convention; `:497-500` holds the correct
constants (`426 + REMOTE_STAT_TAIL_LEN`). The copies:

| statement | where | true absolute |
|---|---|---|
| `/// Body offset 426.` (x) | `crates/net/src/userpool.rs:355` | 449 |
| `/// Body offset 428.` (y) | `crates/net/src/userpool.rs:357` | 451 |
| `/// Body offset 430, +0x6e4.` (move_action) | `crates/net/src/userpool.rs:359` | 453 |
| *"`move_action` is body offset **430**"* — stated four times, and **queued into a doc block it proposes to add to `userpool.rs`** | `research/remote-facing.md:6`, `:36`, `:144`, `:293` | 453 |

`remote-facing.md` is the **newest file in the whole survey** (2026-09-04) and is the one
about to push the un-based number into the code. The good pattern, for contrast, is
`research/0x0224-remote-user-first-use-fault.md:596-602`, which writes `body[470..472]
(builder comment 416, i16)` and shows the arithmetic.

**(c) The builder's own comments use three bases and run backwards.** In
`crates/net/src/userpool.rs::user_enter_field`: `:587-589` true; `:633-637` true absolute;
`:643-645` (job/sub-job) the **retired 7-byte-tail numbering**; `:648-660` zero-tail. Read
top to bottom the comments go `197 → 201 → 186 → 188 → 190 → 187 → 382`. The `186/188/190`
match `research/user-enter-verification.md:643-645`'s superseded proposal exactly.

Confidence: **high** on (a) and (c) (arithmetic and direct reads); **high** on (b).

## 2.9 The 157-byte classic-shop row — the condition dropped four times inside its own owner

| where | statement | condition present? |
|---|---|---|
| **OWNER** `research/classic-shop-rows.md:290` | field 41a, `raw 8` at `+0x40`, ***only when `itemId / 10000` is 207 or 233*** (throwing stars, bullets) | **yes** |
| `research/classic-shop-rows.md:576` | *"ROW, **157 bytes** for an ordinary non-rechargeable item"* | partial |
| `research/classic-shop-rows.md:636` | *"Total … **336 bytes**, `19 + 2 + 157 + 158`"* | **no** |
| `research/classic-shop-rows.md:726` | run plan: *"last outbound length should be **`21 + 157n`**"* | **no** |
| `research/classic-shop-rows.md:727` | *"the **157-byte row** are both right"* | **no** |
| **the copy that shipped** | `crates/net/src/classicshop.rs:106` — `CLASSIC_ROW_LEN = 157`, unconditional | **no** |

**Cost, `research/shop-packet-rejected.md:13-22` (2026-08-30, `[D]`):** Mina and Luna both
stock Subi Throwing Stars `2070000`; every such row left the client 6 bytes short; the read
primitive threw reason `0x26`; that is the `0x009E` that killed the owner's client three times on
demand. *"Size was never involved. A 21 687-byte shop was fine and a 5 673-byte one dies."*

**Code is fixed** (`classicshop.rs:111-113`, `:212-214` `wire_len`, tests at `:577-598`).
**`classic-shop-rows.md` was never amended** — it contains no `009E`, no `2070000` and no
reference to `shop-packet-rejected.md` — and `classicshop.rs:3-4`, `:103`, `:219`, `:432`
names it as the authority the golden vector came from.

## 2.10 The cash shop "sends nothing" — six live statements, and `CLAUDE.md` names this one

`CLAUDE.md` §*"the filter was a timestamp"*: this was reported back to the owner **three times
across three sessions**, and `0x00D5` was in `world.log` every time.

**Still asserting it:** `research/cash-shop.md:1` (**the title**: *"the client never asked,
and that is the whole finding"*), `:13` (*"No packet was sent"* — the literal sentence
`CLAUDE.md` quotes), `:19`, `:63`, `:113` (heading *"five deliberate clicks, five packets,
none of them new"* — the exact "nothing *new* arrived ≠ this thing did not arrive" error),
and `STATUS.md:1617` (heading). The retraction is at `research/cash-shop.md:381-500`,
**380 lines below the title, with no forward pointer at the top**; `:436` even names the
problem — *"Parts one and two of this file both assert 'no packet was sent' and both rest on
that single unexamined grouping."*

**Corrected copies, all consistent, no strengthening:** `research/cash-shop.md:381-500`
(owner, with the 5-run/2-run control table at `:426-431`), `crates/net/src/cashshop.rs:1-27`
and `:57-63`, `crates/world/src/session/cashshop.rs:1-20`, `research/cash-shop-migrate.md:1`,
`cash-shop-stage.md:4`, `cash-shop-items.md:3`, `CLAUDE.md:166-186`, `STATUS.md:1844-1849`.

**Two fixture names are themselves false claims, confirmed against their own bodies:**
`research/fixtures/cash-shop-click-sent-nothing-world.log` has **one** `<- 0x00D5` at line
6961; `cash-shop-five-clicks-zero-packets-world.log` has **one** at line 1669.
*(Method note for anyone re-checking: grep `"<- 0x00D5"`, not `00d5` — the byte pair
`00d5fd06` occurs inside `0x00D9` movement paths and gives ~89 false hits.)*

## 2.11 `0x0467` — still "a template preload list", tagged `[L]`, below its own correction

`CLAUDE.md` names this row: *"`research/` said `0x0467` was 'a template preload list', and it
is `SetNpcScriptable` — `u32 templateId; str script; u32 dateStart; u32 dateEnd` per entry.
**A table row written from a quick read is a claim.** Two separate briefs sent an agent after
that row before anybody counted its reads."*

| where | says | status |
|---|---|---|
| **OWNER** `research/npc-preload.md:1`, `:42-124` | `SetNpcScriptable`, body decoded field by field; listing at `research/msexe-npc-0467.txt` | correct |
| `research/npc-spawn.md:212`, `:238-254` | corrected in place by an independent pass, 2026-08-22 | correct |
| **STALE** `research/npc-spawn.md:479` | *"`0x0467` (a template preload list) and `0x00BE` … are optional; neither creates anything. **[L]**"* | **wrong, tagged `[L]`, 267 lines below its own correction in the same file** |
| **STALE** `research/npc-dialogue.md:294` | *"the NPC template loader (`0x0467`'s preload list feeds it)"* | wrong |
| `research/same-map-capability-sweep.md:317` | quotes the wrong description and says *"Treat it as a candidate"* | hedged copy, fine |
| **STALE meta-claim** `research/npc-preload.md:525` | *"`crates/net/src/opcode.rs` **and** `research/npc-spawn.md` §3.1 both describe `0x0467` as a template preload list"* | `0x0467` appears **nowhere** in `opcode.rs` today; the only `crates/` mention is `net/src/userpool.rs:7`, citing it correctly as the cautionary example |

`npc-spawn.md:479` is very likely the row `CLAUDE.md` means. **A correction applied to a
file's §3.1 did not reach the same file's §9.** Confidence: high on the facts; medium-high
that `:479` is the specific row `CLAUDE.md` refers to.

## 2.12 `skilltable::book()` "does not filter `invisible`" — fixed in code, alive in seven places

**Ground truth:** `crates/world/src/skilltable.rs:100` `pub invisible: bool`; `:322` parses
column 24; **`:402-411` `book()` filters `!s.invisible`** with its own comment saying so.

Still asserting the defect: `research/second-job.md:295-300` and `:509`,
`research/second-job-fields.md:326`, `research/third-job.md:425`,
`crates/world/src/secondjob.rs:169-172` and `:1341` (a **test doc block**), and
`STATUS.md:276-277` under *Still open*.

**`STATUS.md` contradicts itself:** line 195 *"The three invisible third-job skills |
already filtered"* vs line 276 *"`skilltable::book()` **will offer** the thirteen"*. Line
195 is right.

The underlying census (13 invisible = 13 unnamed, exact, both directions) is sound and
owned by `research/second-job.md:305-311` `[L]`, enforced and tested at
`crates/world/src/secondjob.rs:149-176`, `:1312-1338`. **Only the rider is dead.**

## 2.13 "Skill points are not persisted when spent" — and one copy tells the next agent to report it

**Ground truth:** `crates/store/src/skillpoints.rs` is the ledger, tested at `:710-729`
(*"Spent points must not come back on an advancement"*), called from
`crates/world/src/session/skills.rs:158-172` (`spend_and_raise_skill`) in one transaction,
pool computed as `entitlement − spent` at `:246-253`.

| where | text |
|---|---|
| `crates/world/src/skillpoints.rs:22-27` | *"# What is still missing: **SPENDING** … a point the player spends comes back on the next advancement. **Say that when reporting**"* |
| `research/skill-point-ledger.md:10` | *"**Built, not wired.** `0x013B` still spends nothing."* |
| ~~`STATUS.md`~~ | **CLOSED during this survey** — now struck through and marked `**CLOSED**` at `STATUS.md:488` by a concurrent agent |

The `world/skillpoints.rs:27` copy does not merely record a stale fact — it **instructs the
next agent to tell the owner something that is no longer true** — and it **survived the edit that
closed the `STATUS.md` copy.** Two of the three copies are still live.

## 2.14 The skill-id counts — the wrong numbers are still printed in the file `CLAUDE.md` names

`research/attack-skill-id.md:29-33` still prints the table `447 + 233 + 9 = 689` swings,
**2** Three Snails, **14** Magic Claws. Verified by reading it. Line 106 admits *"the wrong
counts in section 1"* exist but **never corrects the table and never states the real
numbers**, and is internally inconsistent with its own `:69` (*"433 of the 434 bodies"*).

**Corrected copies:** `crates/net/src/attack.rs:47-52` (*"Both are exactly doubled … the
Three Snails control is **one packet**, and anything that leans on it should say so"*) and
`STATUS.md:365-367`. Real counts: **426 / 1 / 7**.

**And the corrected number may itself be an overcount.** `attack.rs:40-45` produced its
`434`-body corpus by **file-hash** dedup. `CLAUDE.md:113-119` and
`research/archive-sweep-2026-08-29.md:385-394` both say a content hash is *not enough* —
eleven fixture/previous-run pairs are the same run and hash differently. `archive-sweep`
re-ran with `(timestamp, opcode, body)` event dedup: **849 raw → 787 file-hash → 527
events**. Every *"in all 434 bodies"* downstream inherits the overcount:
`world/src/remoteattack.rs:296`, `:364-368`, `:486`, `:704`; `session/combat.rs:292`;
`research/remote-attack-verification.md:261`; `research/mob-combat.md:123`, `:199`.

## 2.15 The `0x0073` census — "72 bodies, 2 distinct" is wrong in twelve places

| | |
|---|---|
| **OWNER (corrected)** | `research/archive-sweep-2026-08-29.md:116`, `:178-194` — **one body across 67 launches**, not 72/2. The 15 "smoke-test synthetic" bodies are **real client launches** whose MACs were replaced by commit `4f6b448`; **six of the 15 are fixture copies of runs also in `previous-runs/`** — the same packet, same millisecond, counted twice |
| **still says 72 / 2** | `research/client-session-args.md:185-189`, `:193`, `:206`, `:332`; `crates/grap-stub/src/identity.rs:29`, `:179`; `crates/login/src/session.rs:909`, `:972`, `:1000`; `crates/store/src/claims.rs:76`, `:210`, `:357`, `:2408` |

The **conclusion** — the identity has never been non-empty — survives. **Every count
downstream of it is inflated**, and `client-session-args.md:185-189` still labels 15 real
launches as smoke-test output. No cross-reference to the correction was added to any of the
twelve sites.

This is `CLAUDE.md`'s *"fixtures are copies, so counting across both double-counts"* rule
firing again, in the domain the rule was written about. Confidence: high.

**One more from the same file, and it is a *strengthened* sentence rather than a count:**
`research/client-session-args.md:214` reads *"**Filling the identity authenticates the login
socket, not the game socket.**"* — declarative. Nothing has been filled: §6's experiment E2
is unrun (`:279-291`, *"When E2 is run"*), `:263-265` says *"It does not make the client
authenticate; it makes the client transmit a string we chose"*, and §7 (`:306-308`) makes the
whole claim conditional on E2 succeeding. Line 214 is the one sentence a grep would lift, and
it reads as a completed fact about a standing constraint.

*(Checked and clean: no file in any domain claims the game socket carries credentials.
`research/change-channel-reply.md:317-318` restates the constraint correctly and extends it
to the channel case.)*

## 2.16 Other strengthened copies

| # | fact | owner | strengthened copy | confidence |
|---|---|---|---|---|
| a | The client's contact-damage `1`: **224 captures are 224 observations of the same case, not a sweep** | `research/damage-number-two-numbers.md:114-117` explicitly | `world/src/session/combat.rs:1183-1187` restates it as *"`PADamage` runs from 3 to 287 — a **96x spread**"*, i.e. a sweep | high |
| b | *"The client DOES compute contact damage; it is thrown away"* | **retracted 22 min later** by `research/damage-number-two-numbers.md:147-171` — `[rdi+0xe8]` is `fixedBodyAttackDamage`, **zero of 193 mob images carry it**, so the gate is never reached and the patch is inert | `crates/grap-stub/src/hitnumber.rs:8-39` is still built on it; `:19` still mislabels `[rdi+0xe8]` *"the mob's attack power"*; `:77-81` still lists as *unestablished* a question that is settled. Live `-ClientHitNumberPatch` switch. Restated as the justification for a live damage rule at `session/combat.rs:1046-1060` | high |
| c | CTS bit 92 = movement Speed | `research/buffs.md:32` — *"**[D]**, from two [L] halves"*, with the falsifying prediction written down at `:551` | `crates/net/src/buff.rs:112-116` says **[L]** *"from `FUN_1429755a0`"* — but that function **is** the `[D]` chain. The real `[L]` is a screen (`STATUS.md:663`). Tag defensible, evidence named under it is the weaker half. **Fix both ends.** | high |
| d | `MAX_MEMBERS = 6` | `research/party.md:364-380` `[I]` with its blind spot named; `party-result-0x00A5.md:805-807` promotes to `[L]` **but scopes it**: *"That settles the **packet**; whether the server should also cap at six is a consequence"* | `research/party-request-payloads.md:33`, `:365-399` drops the packet/server scope: *"`MAX_MEMBERS = 6` is now **[L]** … can come off `party.md` §5's [I] list"*. Its "two independent legs" **share an object** (leg 2 walks the identical array leg 1 walks) — `CLAUDE.md`'s shared-blind-spot rule | high |
| e | `mob-to-player-damage-packet.md` §6/§7 leaves **two readings** open, *"the single biggest thing left open"* | same file `:390-407` | same file `:18` (§0 answer table) states Reading (1) flat and unhedged. **Turned out correct**, so no harm — flagged as the pattern, not an error of fact | medium |
| f | The v214 reference is **candidates only**, control score 1 of 8 | `research/charrecord-presence-map.md:84-88` — *"this is **[I]** corroboration, not evidence — it could not have overturned the initialisers"* | `research/charrecord-decode.md:92-94` restates the same agreement as *"Two independent lines agree… not a coincidence"* — no `[I]`, no control score at the point of use | medium (tone, not a number) |
| g | `avatar_look` wire shape | `research/avatar-look-loops.md:76-84`; the code eight lines below is correct | `crates/net/src/opcode.rs:1024` summary drops the `u32 0` between skin and face, the discarded `u8`, the **second** `0xFF`, and the 174-byte tail. `avatar-look-loops.md:14-17` exists *because* someone believed there was one terminator | high |
| h | *"The wild write's damaged-slot count tracks session length at about one per 250 s"* | `research/heap-third-dump.md:22-30` tagged it `[D]` on three points; killed at `heap-corruption-2026-08-27.md:158`; replaced at `heap-crash-pattern.md:338` (in-field clock, 298 s/slot, CV 0.19) | `research/job-instructors.md:600-601` restates it **flat, no tag, no hedge**, in an out-of-scope file | high |
| i | `research/cash-shop-stage.md:493-494`: *"**Every** `0x05AE` arm that clears the latch puts a message on screen"* | corrected at `research/cash-shop-buy-done.md:45-49`: the enumeration covered only the six **inline** arms and missed the two that **delegate** — `0x19` (`FUN_140D7F8A0`) and `0x1B` (`FUN_140D80410`) clear the latch and call neither message function. Verified twice, two instruments | `cash-shop-stage.md:493-494` **uncorrected in place**. Its §6.2.2 appends corrections for other things and never touches this sentence. Carried correctly at `crates/net/src/cashshop.rs:536-544`. **Precision: the adjacent *"there is no silent refusal"* at `:492` IS confirmed — do not fix both** | high |
| j | `FUN_140304b20` read count | `research/charrecord-decode.md:14-44`: **126** (122 and 117 were both instrument error — 5 thunked `u32` + 4 unenumerated `u64`) | `research/charrecord-reuse.md:198` hedges honestly (*"a figure I could not verify"*); **`research/charrecord-v214-shape.md:360-363` hardens it**: *"was **measured** at 117 packet reads"*, and draws a structural conclusion from it. The conclusion survives 117→126; the number does not | high |
| k | *"The client asks for **2.3 GB** and the `u32` primitive throws"* | `research/0x0224-dump-read-position.md:289-308` — recovered from saved `rdi` in two crash dumps: *"the client **never demands more than 4 bytes** … no allocation of any size"*; 116 iterations completed, threw on the 117th; `467 = 4×116 + 3` in both processes | `research/avatar-look-loops.md:27`, `:201`, `:205`, `:230` — *"**[D], and it reproduces in both bodies**"*, plus a **proposed code comment** carrying it. `research/0x0224-body-walk.md:27` restates *"2 281 701 376 bytes wanted"*. **`crates/net/src/userpool.rs:614-622` already rejects the phrasing — the code caught this and the research file did not get updated** | high |
| l | `research/setfield-fault-shape.md:16`: *"`FUN_140ce89c0` has **5 call sites**"* | `research/instrument-audit-2026-08-20.md:300` — *"**Incomplete: 10 entries, not 5**"* (5 calls + 5 tail-`jmp` adjustor thunks) | `setfield-fault-shape.md` unedited since 2026-08-19, no back-reference; its `:50-53` scope *"whenever one of **these two functions**"* is now under-scoped | medium |
| m | `research/heap-wild-write.md:77`: pool lock word at `ctx + i*8 + 0x28` | `research/heapfix-did-not-hold.md:99`, `:120` — `ctx + **i*16** + 0x28`, from the listing; matched by `tools/poolchain.py:13-18` (fixed 2026-08-30) and `crates/grap-stub/src/poolsentry.rs:154-166` | only the **lock stride** is stale; the other four rows agree in all four places. Harmless in dumps (locks read zero); **not harmless for `poolsentry.rs`, which reads a live pool** | high |
| n | `research/heap-corruption.md:49`: *"`0x143ad68a0` is the heap handle"* | `research/heap-wild-write.md:69-72` `[D]` — *"it is **not** a heap handle, it is the **pool context**"* | `heap-corruption.md` carries no note; `heapfix-did-not-hold.md:11` and `heap-wild-write.md:3` both explicitly **decline to edit it**. Old label also at `net/src/opcode.rs:1669`, `STATUS.md:1375`, `research/skills.md:358` | high |
| n2 | The `0x0238`/`0x024D` spawn trigger: *"when they appear, the NPC pool exists and is empty — that is the moment to send the spawns"* | `research/npc-spawn.md:441-447` **`[D]`** | `research/npc-chatter.md:336` restates it as *"still applies"* — **`[D]` dropped, presented as a working precondition**. **Falsified** by `crates/world/src/session/mod.rs:95-103`: `0x0238` arrives on the **first field entry only**; three portal transitions produced none, so a trigger on it can never fire after a portal walk | high |
| n3 | The game-stage dispatcher's opcode range | `research/msexe-gamestage-opcodes.md:24-26` — labels are `0x70..0x19f`, **271 of 304 slots**, **plus two outliers** `0x275` and `0x039a` = 273 | `research/msexe-gamestage-dispatch.md:7`, `:119`, `:140`, `:170` flatten it to a contiguous-looking *"273 cases, `0x70..0x39a`"*. `research/change-channel-reply.md:34` had to spend a section correcting a brief that inherited the range from this family. `outbound-0238-024d.md:235-236` and `user-chat-round2.md:38` state it correctly | high |
| n4 | `0x0089`'s 36-entry sub-case table | `research/message-subcases.md` enumerates **all 36** | `crates/net/src/quest.rs:208-216` still says *"the other 35 are **unread** and this module does not guess at them"* | medium |
| o | `crates/net/src/drops.rs:440-468`, `the_pickup_request_is_not_decoded()`: *"**The cheap way to finish this is a measurement** … that is **[I]** and the same reference scored 1 of 8"* | the measurement **was made**: `research/item-drop.md:24` + `research/pick-up-latch.md:24` — `0x032C`, 34-byte body, object id at offset **13**, measured on a client 2026-08-20, `[L]` | a *weakening*, not a strengthening — but it is dead weight in the module a reader hits first and it prescribes work already done. `crates/world/src/drops.rs:179-181` shows the right pattern (keeps the `[I]` history *and* the `MEASURED` line adjacent) | high |

---

# 3. CONTRADICTIONS

Ranked. "Settled" means the evidence in the files decides it.

| # | the disagreement | sides | settled? |
|---|---|---|---|
| **C1** | Is `mob+0x8b4` percent or absolute? | `net/src/combat.rs:47` = absolute · `research/mob-hp-bar.md:94-153` `[L]` = percent | **Yes — percent.** The same file's `hp_percent` and its passing test implement percent. `:47` is pre-2026-08-20 text that survived the fix |
| **C2** | Has the client→server "I was hit" packet been found? | `net/src/combat.rs:611-616` (*"has **not** been found … `0x00E5` … **is not it**"*) and `world/src/damage.rs:17` · vs `research/touch-damage.md:1`, `research/user-hit.md`, `crates/net/src/userhit.rs`, `crates/net/src/mobdamage.rs:9-11`, `session/combat.rs:1001` (*"`0x00E5` — the client says the player took damage. **Apply it.**"*) | **Yes — it is `0x00E5`.** Two doc blocks in `crates/net/src` contradict each other; `combat.rs` is ~8 days stale and sits above code the fact matters to |
| **C3** | Can the server make the client's own damage number correct? | `research/damage-number-suppress.md:28` (*"**No.** … the negative is **measured, not argued**"*), `:546`; `research/damage-number-draw.md:28` · vs the **2026-08-29 client run** (`session/combat.rs:1207-1217`; the owner: *"mob damage numbers are now agreeing"*) | **Yes — by a client run, against the measured negative.** Neither file carries a correction banner. `CLAUDE.md`'s *"Not found is not not there… the screen wins"* recurring on the same page |
| **C4** | `mob+0x3a0` — object id, or template pointer? | `crates/net/src/mobmove.rs:394-397` `[L]` = object id (ctor + `141c4ffb4 MOV [RSI+0x3a0],EBX`); `research/mob-behaviour.md:850`; `world/src/drops.rs:49-50` (test asserts 2000) · vs `research/mob-spawn.md:786-790` §11.10 `[L]` = *"both template pointers"* | **Yes — object id.** `mob-behaviour.md` §11.1 round-trips 60 captured `0x02FF` bodies and matches mob 2000 against an independently decoded `0x00DF`; a 64-bit pointer's low dword would not. §11.10's *first* half can still stand. **Not certain the `141c8108d` reading is about the same object — flagged, not merged** |
| **C5** | Does `mob+0xa88` gate the body rect? | `research/mob-gates-arm-c.md:30`, `:284`, `:370-383`, `:442` `[D]` = *"Two gates, one field"*, `141cb4645` returns writing nothing · vs `research/mob-a88.md:21`, `:218-265` `[L]` = it is a **branch into an 8 KB alternative** that writes the rect at `141cb6358` | **Leans `mob-a88.md`** — it disassembles the branch target, and the real fix (body offset 91, `mob+0xd64` size scale) was confirmed by the owner on screen (`touch-damage.md:8-11`). `mob-gates-arm-c.md` contains **zero** references to `mob-a88.md`. Confidence on *which* is right: medium-high |
| **C6** | Does the stat block contain a map field? | `research/charrecord-flag7.md:347-370` `[D]` = *"There is no map id in this block"*, and `:280` says send *"zeros elsewhere"* · vs `research/charstat-layout.md:342-343` `[L+D]` = map id is read #23 at block offset **84**, three independent lines | **Yes — there is a map field.** `crates/net/src/opcode.rs:1134-1135` writes it; `:1000-1001` and `:1787-1789` record that map `0` never loaded. **Why this one is worse than a normal stale line: `opcode.rs:1804` says *"Layout from `research/charrecord-flag7.md`"* — the code's own pointer sends a reader to the file that denies the field exists.** `flag7.md`'s commit is *later* the same day than `charstat-layout.md`'s, so §6 survived a later edit un-retracted |
| **C7** | `0x0183` op byte | `crates/net/src/party.rs:104-108` and `research/party.md:295-297`, `:521` = *"the op byte the client writes **is `0x1B`**"* · vs `research/party-request-payloads.md:38-39`, `:591-599` `[L]` = `0x1B` **only** for the arm answering `0x00A5` code `0x03`; the code `0x06` arm writes **`0x1C`** (`0x1413bb3a1`) | **Yes — payloads.md.** Four builder sites, two dispatchers. It is filed explicitly as *"a correction to `research/party.md` §3 and `crates/net/src/party.rs`'s doc"*. **Published and never applied, and it names the file by path** |
| **C8** | `poolsentry.rs`'s own tallies | one doc block, three sets: `:9-11` *"Seven dumps… **14 of 14**… 9 damaged in 537 696 vs 0 in 589 240"* · `:96-97` *"across **six** dumps, 1 153 752 slots, **10** non-`slot` headers"* · `:116`, `:1685` *"0 of **693 272**"* | **No.** `537 696 + 589 240 = 1 126 936 ≠ 1 153 752`; `9 ≠ 10 ≠ 14`; `589 240 ≠ 693 272`. The six-dump set matches its owner `heapfix-did-not-hold.md:229-231` exactly. **The seven-dump set and "14 of 14" have no derivation anywhere in `research/`** — the only trace is `poolsentry.rs:44` "dump 319924", a pid in no research file. `:356` prints *"the value in 14 of 14 observations"* into the runtime log. May be counting damaged *headers* vs *slots*; **cannot resolve, flagging** |
| **C9** | `0x00D1` body length | `research/transfer-field-request.md:53` `[L]` = **34** (a capture); `research/revive.md:45`, `:671` = a **25**-byte revive body vs 34 for a portal walk; `world/src/session/field.rs:670-672` correct · vs `world/src/session/mod.rs:703-705` and `session/tests.rs:5607-5611` = *"A portal walk sends **35** bytes"* (the test appends `[0u8; 35]`) | **34 is right** (measured). Small blast radius — the dispatcher only tests empty-vs-not — but two places assert a wrong literal. The 34↔35 pair may be an opcode-inclusion confusion rather than a real disagreement; **not certain** |
| **C10** | `MAX_INVENTORY_SLOTS` | `research/inventory-slots.md:71` = *"**100** is **our** cap"* · vs `crates/net/src/opcode.rs:2000` = **125**, and `:1997` *"It was 100 for one commit, which was mine and arbitrary"* | **Yes — 125.** `bag.rs:427` asserts against it; `tools/test-server.ps1:827` calls 125 the maximum. `inventory-slots.md:71` is a stale literal in the file the code names as its owner |
| **C11** | `crash-14090a6f0.md:88` *"**the ten** `0xC0000374` faults already in the archive"* | vs **its own table** at `:30`, `:32` — 10 + 5 = **15** across two ntdll bases; and `heap-crash-pattern.md:15` = **14** deaths event-deduplicated | **`heap-crash-pattern.md` wins** (it deduplicates on events, per `CLAUDE.md`). `:88` is inconsistent with its own table two paragraphs above. Related: `:89` says *"`heap-wild-write.md` has been chasing it across **ten dumps**"* — that file is a **two-dump** file; the ten-dump chase lives in three other files |
| **C12** | `MAX_MEMBERS` leg count | *"two independent legs"* (`party-request-payloads.md:33`) vs *"three independent ways"* (`party-result-0x00A5.md:806`) | **No.** They cite partly different immediates. Combined there are ≥4 sites, nobody has written the union down, and payloads.md's two share one array walk. **Say so rather than picking** |
| **C13** | Dressed record length | **755**: `opcode.rs:1803`, `:4337` (assert), `charrecord-flag7.md:266` · **743**: `opcode.rs:2845` (*"the whole record"*), `:4325` (a **doc contradicting its own assertion three lines below**), `naked-character.md:282` | **755.** 743 omits the 12-byte `presence[7]` inventory-size block. Low severity |
| **C14** | *"`0x143AD68A0` free entry point"* address | `poolsentry.rs:44` calls the second free `0x14019bb50` · `heapfix-did-not-hold.md:72`, `:112` puts the **fault** at `0x14019bbf3` in `0x14019bb6a..0x14019bc5c`, with `0x14019bb50` the true entry 26 bytes before `.pdata` start | **Same function, both right.** Used interchangeably across files; worth one sentence somewhere |

| **C15** | channel-migrate opcode | `research/change-channel-reply.md:28`, `:244`, `:338`, `:368` = `0x001B` · vs `crates/world/src/session/field.rs:474` = `0x001A` | **Yes — `0x001A`.** Measured: 354 ms dispatch, socket teardown, client reconnected to `127.0.0.1:8486`. §2.4 |
| **C16** | `0x044F` byte 20/21 | `research/npc-appear.md:488` · vs `crates/net/src/opcode.rs:1540-1580`, `:2980` | **Yes — the code.** The other arrangement was shipped once and killed the client. §2.6 |
| **C17** | `0x0467` | `research/npc-spawn.md:479` **`[L]`** + `npc-dialogue.md:294` · vs `research/npc-preload.md:42-124` | **Yes — `SetNpcScriptable`.** Body decoded field by field; listing at `research/msexe-npc-0467.txt`. §2.11 |
| **C18** | `0x0073` archive count | 12 sites = **72 bodies / 2 distinct** · vs `research/archive-sweep-2026-08-29.md:116` = **67 launches / 1 body** | **Yes — 67/1.** The sweep event-deduplicates; six of the 15 "synthetic" bodies are fixture copies of runs also in `previous-runs/`. §2.15 |
| **C19** | `0x00AC` | `research/broadcast-banner.md:1-20` = **`BroadcastMsg`**, banner is type 4, `[L]`, **shipped** as `crates/net/src/broadcast.rs:78` · vs `research/talking-back.md:130` = *"party/guild/friend-shaped"* `[I]`, restated at `research/guilds.md:117` | **Yes — `BroadcastMsg`.** `guilds.md` (2026-08-28) is **eight days later** than `broadcast-banner.md` (2026-08-20) and still quotes the superseded `[I]` as a guild candidate. Correctly tagged `[I]`, so not strengthened — but stale |
| **C20** | `0x0011` | `crates/net/src/opcode.rs:1388` = `MIGRATE_COMMAND` · vs `research/msexe-gamestage-opcodes.md:64`, `:72-79` = **`SelectCharacterResult`**, field-for-field, with v214's `MigrateCommand` at `0x11` called a coincidence | **Not a defect — listed so nobody "fixes" it.** `opcode.rs:1370-1387` carries the whole disagreement, names the reference name, says why the local name is kept, and says *"nothing rests on it."* **This is the model every row in §2 and §3 should be closed to.** Related and unsettled: `0x0000` `ACCOUNT_INFO` vs `CheckPasswordResult`, `0x0010` `LOGIN_RESULT` vs `SelectWorldResult`, `0x0012` `ACCOUNT_INFO_ALT` vs `AccountInfoResult` — numbers agree, only labels differ, but `0x0012`'s research name is what `opcode.rs` calls `0x0000`, which invites a reader to merge two packets the research says are distinct |

### Two research tables whose left-hand column is deliberately WRONG

The domain's worst grep traps. A row quoted without its verdict column is a confidently
wrong name, and neither table can be made safe by editing a row — only by a header.

* `research/msexe-gamestage-opcodes.md:101-110` — the **control table**. Column 2 is what blind alignment *predicted* (`0x0010 SetCharacterID`, `0x0011 MigrateCommand`, `0x0012 AliveReq`, `0x0014 AuthenCodeChanged`, `0x0015 AuthenMessage`, `0x0016 SecurityPacket`); column 4 says **"wrong"** on **7 of 8**. This is the source of `CLAUDE.md`'s *"scored 1 of 8"*.
* `research/msexe-client-opcodes.md:18-27` — column 2 is the **invented name being retired** (`0x0070 CLIENT_VERSION_ECHO`, `0x007A CLIENT_DISCONNECT_NOTICE`, `0x00C0 CLIENT_HELLO`); column 6 is the corrected name (`CLIENT_ENV_REPORT`, `CLIENT_TASK_TIMING_REPORT`, `CLIENT_AUTH_FAILURE_REPORT`). The file itself says two of them *"would actively mislead a reader of the log"*.
* `research/msexe-gamestage-opcodes.md:252-670` — the **273-row A/B candidate table**, correctly hedged at `:212` (*"Treat every row as ±10 entries and never as exact"*) and `:245-247`. Every single-row quote from it loses that. `research/broadcast-banner.md:22-38` and `research/guilds.md:120-127` both quote from it and **both re-state the hedge** — the right behaviour.

### One opcode constant with no derivation at all

`crates/net/src/opcode.rs:1703` — `pub const SET_FIELD: u16 = 0x01A0;` — is the **only opcode
constant in that file with no doc block**, against the module's own rule at `:11` (*"Anything
added here should say how it was established, because none of it can be re-derived by reading
the binary"*). The derivation exists and is strong: `research/msexe-stage-setfield.md:3-29`,
CONFIRMED ON A LIVE CLIENT 2026-08-19 (`WATCH: 0x142097f80 ENTERED … while dispatching opcode
0x01A0`), with the earlier candidate derivation at `msexe-gamestage-opcodes.md:215-241`.
Neither is cited from the constant, so a reader arriving from the code side sees an unsourced
number for the packet the entire world stage hangs off.

Same file, four lines apart: `SET_FIELD_NO_CHARACTER_DATA` (`:1755`) calls the short branch a
*"delivery probe"* while `:1759` says *"The short branch **faults this client**, measured."*
Both are true (`msexe-stage-setfield.md:22-29` — the fault is branch-mismatch, not
opcode-mismatch), but quoted apart they read as a contradiction.

### Two contradictions that dissolve on inspection — recorded so nobody "fixes" them

* **`heap-corruption-dump.md:27`** *"exactly **one** damaged header"* vs `heap-wild-write.md:144` *"**2**"*. **Both right, scope dropped.** A's scope is *one 264 KB arena* — the qualifier is on `:26` and is lost in the summary bullet at `:27` and in the headline.
* **`heap-corruption-dump.md:18`** *"**This is not heap corruption.**"* (restated `equip-crash.md:410` and in commit `1eb8346`'s subject) vs everything from `heap-wild-write.md` onward. **Both right, headline flattens.** A means *"the Windows heap chain is intact; `RtlFreeHeap` refused a bad pointer"*. Quoted bare it reads as *"the heap is fine"*. **This is the highest-risk single sentence in the corpus for the `CONTROL_RELEASE` failure mode** — a true statement whose scope is invisible at the quoting site.
* **`protection-surface.md:43-45`** *"Themida checksums the image. Patching bytes… should be assumed to fail."* is about the image **on disk** and does not say so; `heapfix.rs:93-95` writes into mapped `.text` and *"that one has never been checksummed"*. Not a contradiction — an unqualified sentence that is cited elsewhere (`heap-corruption.md:136`) as a general anti-tamper claim.

---

# 4. Constants and layouts duplicated but CONSISTENT — the ownership map

These need no edit. Recorded so the next agent does not re-derive them, and so a future
change knows every site it must touch.

## 4.1 Structural DRY hazards (values agree; nothing pins them together)

| what | where | hazard |
|---|---|---|
| **`STAT_CHANGED: u16 = 0x007C`** | defined **twice**: `crates/net/src/combat.rs:114` and `crates/net/src/stats.rs:60`, each with its own independent `[L]` derivation | two definitions, **no assert pinning them equal** |
| **`MESSAGE: u16 = 0x0089`** | defined **twice**: `crates/net/src/message.rs:69` and `crates/net/src/quest.rs:216`. `message.rs:64` says the duplication is **deliberate** — *"rather than aliased so a reader of either module sees the number"* | deliberate, but still unpinned |
| Eight more opcode numbers carry two const names in two modules | `0x007D`, `0x0080`, `0x0081`, `0x0138`, `0x0224`, `0x0226`, `0x0293`, `0x029E` | mostly legitimate in/out namespacing — **but nothing declares that, so a reader cannot tell which pairs are intentional.** Worth one comment each |
| **`AP_PER_LEVEL = 5`** | `crates/world/src/expcurve.rs:124` (owner, `[I]` at `:77-88`) and a **second local literal** at `crates/world/src/jobs.rs:522` | `expcurve.rs:338` asserts `g.ap == 5` for every job, so the check exists |
| **Six opcode/name tables, and none of them is canonical** | `crates/net/src/names.rs` (derives from the consts — **cannot** disagree on numbers, the one good design), `docs/opcodes.md`, `research/msexe-client-opcodes.md`, `research/msexe-gamestage-opcodes.md`, `research/msexe-send-opcodes.txt` (§2.2) | only `names.rs` is structurally prevented from drifting |
| **`crates/net/src/opcode.rs` is NOT the opcode table** | it holds **25** of the ~**127** `const …: u16 = 0x….` declarations in `crates/`; the rest are spread across **30** other files (`net/src/{channel,combat,mob,mobmove,npcchat,script,shop,userchat,userpool,cashshop,broadcast,notice,buff,drops,inventory,stats,skills,storage,useitem,party,revive,questforfeit,classicshop,abilityup,message,quest}.rs`, `world/src/{drops.rs,session/field.rs,session/mod.rs}`, `login/src/session.rs`, `grap-stub/src/session.rs`) | **any future instruction to "check it against the opcode table" that greps only `opcode.rs` returns a clean, confident, ~80 %-incomplete answer.** Grep `crates/**/*.rs` for `const …: u16 = 0x` instead. *(My own regex found 121, the domain audit found 127 — the gap is declaration-style variants I did not match, which is itself the point.)* |
| **The write-primitive table** | `tools/encodes.py` `ENC` dict (instrument, owner) · `crates/net/src/attack.rs:100-110` (`FIELD_WIDTHS` doc) · **~15 scattered restatements** in doc comments across `drops.rs`, `mobmove.rs`, `party.rs`, `questforfeit.rs`, `skills.rs`, `quest.rs`, `shop.rs`, `userchat.rs`, `combat.rs`, `world/drops.rs`, `identity.rs` | values agree everywhere I checked; the count of restatements is the hazard |

## 4.2 Consistent, hedges intact — no action

| fact | owner | note |
|---|---|---|
| Bag position is unsigned `u16`; `0` is the terminator so can never address a slot; keep iff `1 ≤ pos ≤ slotMax`; **an out-of-range position is decoded and thrown away** | `research/bag-lists.md:106-138` §2, every bullet `[L]` | `crates/net/src/bag.rs:106-111`, `:130-137` keeps **every** qualifier including *"the bytes are consumed and the item vanishes"*. **The model copy in the whole survey** |
| Storage: take-out is a **0-based positional index**, put is a **1-based bag slot**, same field offset | `research/storage.md:51-53` | `crates/net/src/storage.rs:22-24` carries both qualifiers and the always-answer rule |
| Character record: 100 raw bytes, bytewise-AND gate against a per-gate key with exactly one byte set | `research/charrecord-presence-map.md:12-56` `[L]` from CRT initialisers | agreed and attributed at `charrecord-decode.md:63-102`, `charrecord-loops.md:283-286`, `charstat-layout.md:457-459`, `equip-block.md:52`, `opcode.rs:1832-1837` |
| Stat block field order and widths (29 reads, 108 bytes) | `research/charstat-layout.md:164-195` `[L]` per-read addresses | three files agree on order and width; **only the map-id naming diverges** (C6) |
| Extended-SP fork; equip item body = **125** bytes either way; the 21 mask-2 bit widths | `charrecord-reuse.md:100-110`; `naked-character.md:191-235`, `:242-244` | `equip-stats.md:181-182` states it **re-read them off the listing rather than trusting the earlier table** — model behaviour |
| `0x00D9`/`0x0293` path head | `research/user-move.md:23-37` (1082 bodies) | `remote-move-verification.md:157` says it **did not trust** `user-move.md` and re-derived — model behaviour |
| HITINFO = **147 bytes**, *"≥147, never ==147"* | `research/user-hit.md:63` + `user-hit-remote-0x02A5.md:170` (two independent code paths) | qualifier survives at `userhit.rs:22`, `userpool.rs:196`, `docs/opcodes.md:68`. `userpool.rs:1238` is **legitimately stricter** — different function |
| `0x02D1` effect `0x41`: `i32 amount, i32 delayMs, i32 id`, 15 bytes, **ms** | `research/recovery-number.md:346`, `:539-541` | identical in six places incl. the unit |
| CTS mask length **124** | `research/buffs.md:290-293` (three independent client reads) | `buff.rs:38` correctly says *"stated three independent ways"* |
| Job level gates **10 / 30 / 70** | `world/jobs.rs:62` `[D]`; `world/secondjob.rs:117-126` `[L]`; `world/thirdjob.rs:87-94` `[I]` | **the confidence tags survive every hop** — the best-behaved constant family found |
| SP pool key is a **tier 0..=10, not a job id** | `research/skill-points.md:26`, `:418` `[D]` | six copies, every one repeats the *"a job id reads an empty pool"* consequence |
| mastery is **1..=10, not a percentage**; skill damage is a **percent** | `research/damage-formula.md:43-44` `[L]` | `meowdb-combat-formulas.md:1-22` is exemplary: names its owner, quantifies the error (*"a 5.5x error"*), says *"where the two files disagree, `damage-formula.md` is right"* |
| Heap: `0x14019b504` patch bytes; the four-bucket table off `0x14019b7f0`; `0x143AD68A0` = pool context; chunk identity `n*(slot+8)+8`; type 8 = entry / type 9 = caller pointer | `heapfix.rs:76-81`; `heap-wild-write.md:82-89`; 8 sites; `poolchain.py:29-32`; `dumpwalk.py:1000-1020` | identical everywhere, and `dumpwalk.py` **refuses** the bad decode rather than printing a plausible one |
| `FIRST_CHARACTER_ID = 200` | `crates/store/src/db.rs:45` — **defined once** | restated correctly in ~10 places. **No file states ids start at 1.** Clean |
| *"Nothing authenticates; the game socket carries no credentials"* | `CLAUDE.md` standing constraint | restated consistently in 8 places. Clean |
| *"The dispatch line is written on handler **return**"* | `research/npc-shop-crash.md` §1.1 `[L]` | 5 restatements, no strengthening; only line-number drift in the citations |

*One unverified block, and it is the largest:* the per-level buff arrays at
`crates/net/src/buff.rs:670-724` and `crates/net/src/jobbuffs.rs:213-360` — **~200
duplicated numbers** also stated in `research/first-job-buffs.md` and
`research/magician-first-job.md`. Lengths are compile-time asserted; **values were not
cross-checked by anyone in this survey.** `buff.rs:684-686`'s note *"Level 20 is 600, not
585"* suggests at least one row has already bitten someone.

---

# 5. DEAD WEIGHT

## 5.1 The big one: a second copy of the whole repo

`.claude/worktrees/elegant-diffie-14dffe/` — **1.2 GB**, a git worktree at commit `40dcbb1`
(**2026-08-28**, seven days stale), containing **112 research `.md` files of which 90 differ
from `main`**. Every recursive grep from the repo root finds each claim **twice, at two
different commits**, and the older copy has none of the corrections in this index. Two of
the seven domain audits tripped over it independently. `git worktree list` shows it; if the
branch is finished, `git worktree remove` reclaims 1.2 GB and removes the hazard.

## 5.2 Unreferenced dumps

**86 of the 282** `.c`/`.txt` dumps in `research/` are referenced by name from **nothing** —
no `.md`, `.rs`, `.py` or `.ps1` anywhere in the tree. 11 MB total for all 282. Selected:
all six `gsfail-*`, `msexe-aes.c`, `msexe-aescore.c`, `msexe-aesinit.c`, `msexe-crypto.c`,
`msexe-pktcrypto.c`, `msexe-cipher2.c` (the cipher work is settled and lives in
`docs/transport.md`), `msexe-main.c`, `msexe-waitloop.c`, `msexe-stagereg.c`,
`msexe-loginui.c`, `msexe-loginresult.c`, `msexe-loginflag.c`, `msexe-charlist.c`,
`msexe-worldlist.c`, `nmco-dispatch.c`, `msexe-framing.c`, `msexe-packetwriter.c`,
`msexe-sendloop.c`, `msexe-sendpath.c`, `msexe-recvtail.c`, `msexe-teardown.c`,
`msexe-quitpath.c`, `msexe-socket-closer.c`, `msexe-stringtable.c`, `msexe-strkey.c`.
*(Full list reproducible: for each dump, grep its stem across the tree excluding itself.)*
**Caveat before deleting: a dump is cheap to keep and expensive to regenerate, and Ghidra
is a single-process lock.** These are listed as *unreferenced*, which is not the same as
*useless*.

## 5.3 Research `.md` files referenced by nothing at all

12 of 144, cited by no other `.md`, no `.rs`, no tool, not `STATUS.md`:
`archive-sweep-2026-08-29`, `cash-shop-migrate`, `client-launch`, `client-recon`,
`crash-14090a6f0`, `msexe-handshake-gh-gate`, `remote-facing`, `remote-move-verification`,
`shop-packet-rejected`, `skill-point-ledger`, `user-hit-remote-0x02A5`, `world-names`.

**Four of these are orphan OWNERS — the pattern that produced §2.7.** They hold the
derivation of a fact the code depends on, and nothing points at them:

| orphan | owns | consumed by |
|---|---|---|
| `research/shop-packet-rejected.md` | the `0x009E` / `+6` rechargeable row diagnosis | `crates/net/src/classicshop.rs` (§2.9) |
| `research/remote-facing.md` | `move_action = (action << 1) | facing` | `userpool.rs:359`, `:382`; `usermove.rs:291`, `:512`; `session/mod.rs:274`; `multiplayer.rs:1456` |
| `research/skill-point-ledger.md` | the `character_skill_spend` schema | `crates/store/src/skillpoints.rs` |
| `research/archive-sweep-2026-08-29.md` | the **event-deduplicated** archive counts | contradicts the file-hash counts in `attack.rs` (§2.14) |

`research/mob-a88.md` is a fifth: cited by **nothing**, and it owns `mob+0xd64` / body offset
91 which `crates/net/src/mob.rs:99`, `:515`, `:588`, `:838` depends on (`MOB_SCALE_UNSCALED`).

## 5.4 Files superseded entirely, or describing an abandoned approach

| file | verdict |
|---|---|
| `research/npc-shop.md` (711 lines) | **Superseded and actively misleading.** Built entirely on `0x0560`, which `classic-shop-opcode.md:16-21` established is the wrong one of two shop windows — its art `UI/UIWindow2.img/Shop2` is **absent from this client's WZ** and it killed the client twice. Contains **no `0x055D` anywhere** and **no supersession banner**. Its §9 run plan at `:673` still tells the owner to click Lucy and expect a shop window. Meanwhile `world/src/session/tests.rs:885` asserts *"0x0560 must never go out again"* |
| `research/heap-corruption.md` (2026-08-21) | **Superseded.** Title says *"four sightings"* (now 14 deaths); `:3` says the fault *"cannot be diagnosed by a post-mortem stack"* (six post-mortems have since diagnosed it); `:49` heap-handle error uncorrected (C-row n); `:61-107` is the **page-heap recipe**, an abandoned approach — `heap-corruption-dump.md:400` says page heap **cannot see** slots inside the client arena, which was the whole reason it was turned on. Only live content is the `0xC0000421` result and the WER correction, both duplicated in `minidump.rs:1-32` and `CLAUDE.md` |
| `research/cash-shop-migrate.md` (625 lines) | **Largely superseded** one day later by `cash-shop-stage.md`. Cited by **zero** files under `crates/`. Its `:588` prescribes *"reply with `0x0070` then `0x01A3`"* — `stage.md:793-796` `[L]` shows `0x01A3` clears the latch itself, so the `0x0070` is unnecessary, and the shipped code omits it. Its `:582` (*"`0x01A3` has never executed in this project"*) was falsified on a client 2026-08-25/26. §2-§7 may retain unique value; not assessed |
| `research/inventory-slots.md` §§ at `:3`, `:126-135`, `:153-198` | **Abandoned experiment.** Still says *"Unconfirmed on screen"*, still prescribes *"**Go under instead. `-InventorySlots 10`**"* and a 3-row outcome table for a run that has been made and **could not answer the question it was designed for** (`opcode.rs:2002-2006`). `MIN_INVENTORY_SLOTS = 30` makes 10 unsendable. `tools/test-server.ps1:820-828` is current; the research file behind it is not — the §"two places" failure with a third place |
| `research/user-enter-verification.md` (790 lines) | **Half dead, no banner.** §8.1 (`:620-666`) prescribes four superseded numbers and a test name. §1.3a is still live and still cited by `userpool.rs:611`. **Needs a banner, not deletion** |
| `research/user-enter-field.md` (586 lines) | **Stale on totals, live on structure.** `:26`/`:221`/`:564` say 508; `:486-488` says *"the whole user-pool branch is unexercised"*, false since 2026-09-03. Its own §2.2 table is still the owner of the field sequence and every downstream file indexes against it — **must stay, needs one line saying the numbers are zero-tail** |
| `research/damage-number-suppress.md`, `research/damage-number-draw.md` §0/§6 | **Superseded, no banner** (C3). `damage-number-draw.md` §1-§4 (the `0x02D1`/`0x41` renderer path) is still good and referenced by live code |
| `crates/grap-stub/src/hitnumber.rs` | **Abandoned approach**, retracted 22 min after it was written (§2.16b) and moot since 2026-08-29. Still a live, documented launcher switch |
| `crates/net/src/combat.rs:608-651` (`touch_damage`, `player_hp_after`) | **Orphaned.** Zero production callers; its formula competes with `world::damage::incoming_damage`, which is what runs. `world/src/damage.rs:771` notes the duplication |
| `research/mob-target-gates.md` §4/§6 | Superseded by `mob-collector-callsites.md` → `mob-gates-arm-c.md`. Correct top banner; body is dead weight |
| `research/protection-surface.md` (2026-08-14) | Largely historical; its `grap64.dll` and `IPPORT` verdicts were resolved by the shipped hook. The one sentence anyone still quotes is the one needing a qualifier |
| `research/exp-sharing.md` (136 lines) | Thin — a requirements record plus a wiring map; its one factual claim beyond the owner's quote is stale (*"Parties do not exist"*, §6 below) |
| `research/change-channel-reply.md` §4.1 / §7 (`:238-268`, `:330-380`) | **Superseded** — the `0x001B` derivation and its paste-ready code block were settled against by measurement on 2026-08-21 (§2.4). §0-§3 is still the owner of everything except the number and must be kept |
| `research/npc-appear.md` §7.1 (`:449-489`) | **Abandoned approach, not marked.** The change was made and reverted the same day. Its `:11` banner names two superseded sections and this is not one of them (§2.6). §3.1 and §"the same epoch" *are* correctly marked |
| `research/msexe-setfield.md` | **Not dead — but the filename is a trap.** It is a decode of `0x0070 = InventoryOperation` and says so at `:1-3` (*"It is not SetField"*). A grep for "setfield" returns an inventory document. `research/msexe-setfield.c` carries the same misnomer by its own admission (`:13-14`) |
| `research/guilds.md:117` | Stale citation of `talking-back.md`'s `[I]` for `0x00AC`, eight days after `broadcast-banner.md` settled it `[L]` and shipped it (C19) |
| `research/npc-preload.md:519-527` ("One documentation change, no bytes") | The change it asks for was made in `npc-spawn.md` — **partially**, `:479` was missed — and its claim about `opcode.rs` is no longer true (§2.11) |
| `dumps\` on disk | **None of the six analysed heap dumps still exist.** Every heap conclusion is now re-derivable only from the write-ups. Not an error — worth knowing before anyone plans a re-run |

## 5.5 Naming hazards

* `docs/client-messages.md` and `research/client-messages.md` are **different subjects**
  (string-table decryption vs the `0x0089` right-hand message area). A citation of
  *"client-messages.md"* is ambiguous. Not a duplicated claim.
* `research/heap-wild-write.md:103` cites `research/fixtures/heap-second-dump-same-stack-different-type.log`, **which does not exist** — it was renamed to `heap-second-dump-same-damage-value.log`. A dangling path in the file that owns the correction.
* `research/mob-behaviour.md:838` says `tools/encodes.py`; `crates/net/src/mobmove.rs:382` says `scratchpad/encodes.py`. The file is at `tools/encodes.py`, and `CLAUDE.md` § *The scratchpad shadows the real tools* is specifically about this.

---

# 6. Cross-file corrections that were published and never landed

Mechanical pass: every heading matching `correction|retract|supersed` that **names another
research file**, checked for whether the target file mentions the correcting file anywhere.

| correcting file | target | landed in target? |
|---|---|---|
| `0x0224-dump-read-position.md:289` | `avatar-look-loops.md` | **no** (§2.16k) |
| `cash-shop-buy-done.md:617` | `cash-shop-stage.md` | **no** (§2.16i) |
| `channel-two-greyed.md:182` | `channel-select.md` | **no** |
| `mob-behaviour.md:585` | `mob-spawn.md` | **no** |
| `npc-appear.md:86` (*"`npc-chatter.md` §8.2 must be **un-retracted**"*) | `npc-chatter.md` | **no** |
| `npc-preload.md:131` | `npc-spawn.md` | **partly** — see below |
| `naked-character.md:407` | `equip-block.md` | yes |
| `npc-fade.md:155` | `npc-appear.md` | yes |
| `recovery-number.md:169` | `level-up.md` | yes |
| `touch-damage.md:56` | `mob-combat.md` | yes |

The `npc-preload.md` → `npc-spawn.md` row reads *"partly"* because the correction reached
that file's §3.1 and **not** its §9 — the full case is §2.11, and it is the one `CLAUDE.md`
names. It is the most instructive row in this table: **a correction can land in the right
file and still miss the claim**, so "did the target file get updated?" is not the same
question as "is the wrong sentence gone?", and only the second one is worth asking.

**Also stale, and the same class:**

| claim | still asserted at | corrected by |
|---|---|---|
| *"Parties do not exist"* | `research/exp-sharing.md:109`; `world/src/session/combat.rs:850` | `crates/world/src/party.rs`, `session/party.rs`, dispatched `session/mod.rs:809`; create has been on the wire (`party-result-0x00A5.md:811`). **The conclusion (70/30 unimplemented) is still true; the reason is not** |
| *"a sibling agent's AP handlers exist but are not dispatched"* | `research/job-advancement.md:27` | dispatched at `session/mod.rs:772-776`. Half-defused by *"as this was written"* |
| *"the branch every character takes sends a pool list nobody has decoded"* | `crates/store/src/db.rs:436-438` | decoded at `research/skill-points.md` §2, implemented as `net::stats::SpPool`. The sibling half of this correction **was** applied — `crates/net/src/combat.rs:810-820` carries a `# CORRECTED 2026-08-27` block. **That block is the model for how every row in §2 and §3 should be closed** |
| `docs/opcodes.md:580` — `0x0107` *"**never reaches the wire**"* | | falsified: `<- 0x0107` inbound in ≥5 fixtures; parsed at `net/src/inventory.rs:31`, handled in `world/src/session/inventory.rs`. Reads as *"the unequip path is dead"* |
| `docs/opcodes.md:85` — `0x01A3` *"delete / rankingonoff"* | | it is **`SetCashShop`** (`cashshop.rs:158`, `cash-shop-stage.md:13-16` `[L]`) |
| `docs/opcodes.md:71-72` — `0x00F5` "point", `0x00F6` "InCoin / exit / outCoin" | | `0x00F5` is the **classic shop request** (`classicshop.rs:75`); `0x00F6` is the **storage request, first byte a mode** (`storage.md:33-45`) |
| `docs/opcodes.md:575` — `0x0560` *"sent, and it kills the client. Off by default."* | | shops are **on by default** on `0x055D` |
| `research/storage.md:23` — *"Nothing in `crates/` knows either opcode exists"* | | `net/src/storage.rs` and `world/src/session/storage.rs` exist. Cosmetic |
| `tools/test-server.ps1:788-795` (`-HeapFix` comment) — *"Six damaged slots across three dumps"*, *"the old one-per-250-s is falsified"*, *"if the client still dies with `0xC0000374`, `heap-wild-write.md` is wrong somewhere"* | | **it did die with it on**, and `heapfix-did-not-hold.md:320` says the opposite: *"The chain in `heap-wild-write.md` is **confirmed**, not falsified."* **This is a THIRD copy of the test plan** — the `.NOTES` block (`:40`, `:639-645`) and the `Write-Host` block (`:1642`) are both current; nobody knew to update the parameter comment |
| `crates/grap-stub/src/heapfix.rs:56-57` and **`:142`** | | the same falsified prediction, and `:142` is a **runtime log string** that writes the wrong sentence into `maplecw-hook.log` on every armed run |
| `STATUS.md:491`, `:626-628`, `:2095` | | three sentences `heap-crash-pattern.md` §10 already flagged (10 deaths → **14**; the 250 s rate *"FALSIFIED"* → re-derived at 298 s/slot CV 0.19; *"`-HeapFix` has never been on in a crashed run"* → it was, dump 217724). **`heap-crash-pattern.md`'s own pointers into `STATUS.md` have drifted** (it cites `:298` and `:1902`; they are now `:491` and `:2095`), and **`STATUS.md` contains no reference to `heap-crash-pattern.md` at all** |

---

# 7. Coverage, method, and what this index does not cover

**Method.** Seven domain audits (mob, heap/crash, damage, character record/avatar,
shops/inventory/drops, quests/jobs/skills/party, opcodes/stage) plus a mechanical pass by
the coordinator. Greps targeted the shapes that duplicate — hex addresses
`0x14[0-9a-f]{7}`, opcodes `0x[0-9A-F]{4}`, byte offsets `+0x[0-9a-f]+`, `[L]`/`[D]`/`[I]`
tags, unit words, invariant words — then read around the hits. Nothing was read end to end.

**One instrument failure inside this survey, recorded because this file is about exactly
that.** An early grep here was written as
`grep -rn "0x0467" research/*.md crates --include=*.rs`. `--include` applies to every
argument, so the `.md` files were **silently excluded** and the search returned one hit
instead of nine. It was caught only because a domain report named a file the grep had not
returned. `CLAUDE.md`: *"a silent negative is usually a property of your search."* Re-run
with the two searches separated.

**Not verified against the binary.** No Ghidra, no `cargo`, no client run. Every verdict is
documents-and-code against each other, plus arithmetic. Where a domain audit says a claim is
`[L]`, that is the *file's* label, not a re-measurement.

**A second instrument caveat, from the opcode domain, and it bounds a headline number.** The
opcode→name comparison was done by a scripted extraction that handles `| 0xNNNN | … | Name |`
and inline forms but **misses prose forms** like *"the handler for X, which the table calls
Y"*. **The opcode-disagreement count in §3 is a lower bound, not an enumeration.** A complete
pass wants a table-aware parser per file, not one regex — and the 273-row candidate table at
`msexe-gamestage-opcodes.md:252-670` was **not** checked row by row against `crates/`.

**Known gaps, ranked by what a reader is most likely to want and not find here:**

1. **~200 duplicated buff numbers** — `net/src/buff.rs:670-724` and `net/src/jobbuffs.rs:213-360` against `research/first-job-buffs.md`, `magician-first-job.md` and `gm-handbook/skills.txt`. Lengths asserted, **values never cross-checked**. Largest unaudited duplication in the repo.
2. **The cash-shop interiors** — the 32-byte queue record, the `[stage+0x120]` kind values, the `0x05AE` field tables across `cash-shop-actions.md`, `cash-shop-buy-done.md`, `cash-shop-cash-inventory.md`. Dense and likely to restate each other; not diffed.
3. `research/equip-stats.md` (1171 lines) §§4-9, 11 — not audited for internal duplication.
4. `research/mob-spawn-selection.md`, and the gate numbering shared across `mob-collector-callsites.md` / `mob-gates-arm-c.md` / `mob-target-gates.md` — unaudited and duplication is likely.
5. `research/user-pool-tables.md` (1120 lines) — confirmed to have **zero** overlap with the character-record set, but its own internal census was not audited.
6. `research/level-up.md`, `ap-allocation.md`, `mp-regen.md`, `job-instructors.md`, `skills.md` — greps only, no narrative sweep.
7. The `224 / 198 / 257 / 331` archived `0x00E5` counts (see below) were **not re-measured**.
8. `crates/store/` and `crates/world/src/broadcast.rs` were not diffed against the record/look layout; a fifth restatement of those offsets could live there.

**One open numeric question worth a line of its own.** Four different counts of archived
`0x00E5` user-hit events are in circulation: **224** / 22 files (`damage-number-draw.md:399-402`,
no dedup) · **198** / 160 logs (`damage-number-two-numbers.md:131-136`, **copied verbatim into
`grap-stub/src/hitnumber.rs:38` and `session/combat.rs:1054`**) · **257** events, 435 raw
(`mob-attack-skills.md:55`, `:94-97` — the only one produced by the `CLAUDE.md`-sanctioned
`(timestamp, opcode, body)` event key) · **331** events / 530 files
(`user-hit-remote-0x02A5.md:56`). Archive growth explains 224 → 257 → 331. **198 is the
anomaly** — taken the day *after* the 224 count, from **160 logs against 22**, and smaller.
Nothing reconciles the four, and **the smallest and least-supported number is the one in
shipping code.**

---

# 8. If only ten things get fixed

In this order. Each is a single edit or a small set.

**The four instruments first — a wrong instrument makes new wrong facts:**

1. **`tools/reads.py:57-60`** — the header says TEN/six real readers; the dict has ELEVEN/seven. Fix the header, then `tools/listing.py:19`, `:33`, `crates/net/src/script.rs:11-13`, `crates/net/src/opcode.rs:1925`, `crates/net/src/mob.rs:406`. **101 files cite this instrument.** (§2.1)
2. **`research/msexe-send-opcodes.txt`** — one header line saying it cannot see a builder inside `.themida` or one fed from a register, and naming the five known absences. This is the control `CLAUDE.md` nominates for proving a negative. (§2.2)
3. **`research/change-channel-reply.md:1`** — a CORRECTION banner. `:338` is a paste-ready `const CHANNEL_MIGRATE_COMMAND = 0x001B`, the shipping code points readers at this file, and it is `0x001A`. (§2.4)
4. **`research/npc-appear.md:479-489`** — mark §7.1 superseded. It still recommends, as its own change, the byte arrangement that killed the client on 2026-08-21. (§2.6)

**Then the facts, by blast radius:**

5. **`crates/net/src/combat.rs:47`** and `:119` — the module header still calls `mob+0x8b4` absolute. The corrected text is 550 lines below in the same file. (§2.3)
6. **The seven stale NOT WIRED banners** (§2.5) — each is one line, and the rule they implement is the one `CLAUDE.md` says has already caught two subsystems.
7. **`crates/net/src/mobmove.rs:341-352`**, `:250`, `:261-262`, `:765-766` — the three `CONTROL_RELEASE` copies the retraction missed, one of which is `[L]`; and `research/npc-spawn.md:479`, the `0x0467` row `CLAUDE.md` names, also `[L]`, also below its own correction. (§2.7, §2.11)
8. **`research/remote-facing.md`** — stop `430` reaching a code doc block without its base; add one line to `research/user-enter-field.md` §2.2 saying its table is zero-tail; and **`research/classic-shop-rows.md:576`, `:636`, `:726`, `:727`** — note that the row is `157 + 6` for `itemId / 10000 ∈ {207, 233}`. Both codebases are already right. (§2.8, §2.9)
9. **`research/cash-shop.md`** — a banner at line 1; the title and four more lines assert what the same file retracts at `:381`. Rename the two fixtures whose names are false against their own bodies. Then **`research/attack-skill-id.md:29-33`** (`426 / 1 / 7`) and **`crates/world/src/skillpoints.rs:22-27`**, which instructs the next agent to report a fixed bug to the owner. (§2.10, §2.13, §2.14)
10. **`git worktree remove .claude/worktrees/elegant-diffie-14dffe`** if that branch is done — 1.2 GB, 90 divergent research files, and every root-level grep currently answers twice. (§5.1)

**One thing that is not a fix but a template.** `crates/net/src/combat.rs:810-820` carries a
`# CORRECTED 2026-08-27` block: the old claim, the new one, the date, and why. So does
`crates/net/src/opcode.rs:1370-1387` for the `0x0011` naming disagreement, and
`research/meowdb-combat-formulas.md:1-22`, which names its owner, quantifies its own error
and says which file wins. **Every row in §2 and §3 wants that shape.** The corrections in
this index all already exist; what is missing is that they are somewhere else.

**The pattern behind all ten, and the only thing worth generalising:** in every case the
correction exists, is right, and is *somewhere else*. Nothing in this index needed new
research. It needed the correction and the claim to be in the same place.
