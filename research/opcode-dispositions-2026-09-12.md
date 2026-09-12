# Every inbound opcode has a disposition — 2026-09-12

The owner: *"Please handle all of the opcodes."*

## 1. What "unhandled" meant, enumerated before filtering

Both servers log `<opcode> <NAME> is not answered ...` whenever a handler returns nothing, and
`UNKNOWN` when `net::names` has no row. Over the last 40 archived channel logs, deduplicated
on `(timestamp, opcode, body)` and counting only packets whose **next line** says `is not
answered yet` (the loose grep also caught `0x02FF` and `0x00E5`, which are answered and whose
*reply* lines contain the word):

| opcode | events | body | what it is | evidence |
|---|---|---|---|---|
| `0x013D` | 627 | 25 B, every 30 s | send-counter census | `research/buffs.md` §2 — **must not be answered** |
| `0x02F4` | 482 | 12 B: u32, u32, i32 | telemetry, a few per session | `research/cash-shop.md` telemetry table; builder `FUN_142937ba0`; meaning undecoded |
| `0x01ED` | 344 | 4..56 B, first u32 a kind | the client's log channel | `research/cash-shop.md` §3.5: 104 builder sites |
| `0x0420` `0x0421` `0x0423` | 32 each | text records: timestamp string, char id, name | the leaving-the-field burst | `research/cash-shop-migrate.md`: `0x0422` = `FUN_142d18520(ctx, reason, 0)` **[L]** |
| `0x0422` | 32 | u32 reason, str | "I am leaving the field, reason N" | same; reason 4 = Cash Shop click, 2 = channel change |
| `0x0426` | 32 | 20 B | closes the burst | same burst, same timestamps |
| `0x0425` | 6 | 75..5226 B: u32 count, records of {u32, str path, u32×3} | a resource census (`Sound/Mob.img/…/Die`, `Map/Obj/…`) | shape from the bodies; builder `FUN_140216c50` (reached by pointer) |
| `0x00B8` | 30 | u8 0/1 | a toggle, a few per session | `research/mob-combat.md` §14 |
| `0x02DE` | 2/run | u8 | once per field entry | `research/mob-combat.md`: 252 of these against 251 `0x00DC` |
| `0x0184` | 2/run | 12 B | once per field entry | `research/npc-click.md` table: `FUN_142defbc0` → body from `FUN_142df2760` |
| `0x0194` | 2/run | u8 | once per field entry | same table: `FUN_142defc50` |
| `0x01A5` | 3 | lists of {skillId, checksum} | skill checksums after every skill-up | `research/buffs.md` "`0x01A5` while we are here" |
| `0x01C1` | 7 | u32 charId, str name | **undecoded** | builder `FUN_142927ac0`, one caller inside the 22 KB `FUN_1428923e0` |
| `0x01B9` | 27 | empty | **undecoded** | builder `FUN_142db77a0`, three callers, no strings |
| `0x0226` | 1 | 10 B | **undecoded** | builder `FUN_1428f4eb0` |
| login: `0x0070 0x0071 0x0073 0x0079 0x007A 0x00A6 0x00BF 0x00BC` | every login | — | environment, timings, status codes, identity, title-ready, and the undecoded `0x00BC` | `research/msexe-client-opcodes.md` |

**Every one is a one-way report.** They arrived unanswered hundreds of times with the client
playing on, and none is in `net::dropmoney::LATCHING_REQUESTS`. `0x0199` (keymap) and
`0x00E5` (user hit), which also appeared in older logs as unanswered, have been handled since;
their counts came from logs that predate the handlers.

## 2. A second finding from the same sweep: seventeen handled opcodes had no name

Resolving every constant the world dispatcher matches and diffing against `net::names`:
`0x00D2 0x00DA 0x00DB 0x00E5 0x00F5 0x0104 0x010E 0x0111 0x0125 0x0138 0x0139 0x013B 0x0143
0x017E 0x0199 0x01E7 0x0453` all had arms, modules and doc blocks - and logged as `UNKNOWN`
because nobody added the row. That is the worse kind of gap: a correctly handled packet that
reads, in the log, as one nobody has looked at.

## 3. What changed

* `net::names`: rows for all of §1 and §2, each carrying its shape and its evidence. The
  three undecoded reports are named `CLIENT_UNDECODED_*` and added to `never_truncate`, so
  they stop reading as new but still log whole. `0x00BC` stays unnamed - that decision and
  its reason were already recorded in the table.
* `net::names::is_client_report(op)`: the one-way set, login and channel.
* Both servers: an empty reply to a report logs `is a client report; nothing is expected
  back`; anything else with no reply keeps `is not answered yet` (and `UNKNOWN` when
  nameless). Two silences that used to look the same now do not.
* `world::session::reports`: an explicit dispatcher arm for the report set, **after** the
  exclusive-request whitelist, returning nothing; `0x0422` logs its reason and `0x0425` its
  count.
* Tests: every dispatched opcode is named; every report is named (bar `0x00BC`) and none
  latches; every report body from the archive is answered with nothing and panics on none of
  them, including truncated `0x0422`/`0x0425`.

## 4. What this does not claim

The three `CLIENT_UNDECODED_*` reports and `0x02F4` are named by shape and provenance, not
meaning. `0x0425`'s record layout is read off two bodies, not the builder. The reasons of
`0x0422` other than 2 and 4 are unmapped; the new log line is where they will show up.

**The check on the next launch:** `grep -c UNKNOWN login.log world.log world-ch1.log` should
be **0**; any hit is a packet this project has genuinely never seen. `grep "client report"`
lists the correct silences.
