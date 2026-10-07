# Mob status indices, read off the client - Poison is 23

The owner, 2026-10-06, after Poison Breath's burst was built without its mark: *"Can you figure it
out by decompiling?"* Yes. Everything below is **[L]**, from `MapleStory.exe` in
`research/ghidra`.

## The instrument: the client's own name table

`.rdata` holds the mob status **names** at `0x143289d20..` (`PDR`, `MDR`, `ArcMage2Stack`, `Stun`,
`Freeze`, `BeforeFreeze`, `Poison`, `Seal`, `Darkness`, ... `Burned`; `PAD` and `MAD` are shared
strings at `0x143272658`). Two functions beside the `0x03E6` decoder use them -
`FUN_140464530` (23 037 bytes) and `FUN_14045e980` (23 453 bytes). Each is a chain of blocks:
compare the argument with one name, then read **that status's fields** off the stat object
(`R14`). So a name block names the fields, and the decoder `FUN_14046fba0` names which mask bit
fills them.

| name | string | block in `FUN_140464530` | fields read | decoder bit that writes them | index |
|---|---|---|---|---|---|
| `PAD` | `0x143272658` | first block | `+0x5c/+0x60/+0x64` | `TEST [R15],0x80000` | **12** |
| `PDR` | `0x143289d20` | `140464657` | `+0x6c/+0x70/+0x74` | `TEST [R15],0x40000` | **13** |
| `Stun` | `0x143289d38` | `140464d12` | `+0xe8/+0xec/+0xf0` | `TEST [R15],0x800` | **20** |
| `Freeze` | `0x143289d40` | `140464e10` | `+0xf4/+0xf8/+0xfc` | `TEST [R15],0x400` | **21** |
| `Poison` | `0x143289d58` | `140464fff` | `+0x114/+0x118/+0x11c` | `TEST [R15],0x100` (`1404700a2`) | **23** |

Index = word x 32 + (31 - bit). The decoder's per-index field offsets were tabulated from
`research/msexe-mobmove.txt` (its whole listing).

**Controls.** `PAD = 12` and `PDR = 13` were already being sent for Disorder on an inference
from the reference's order (`net::mobstat` module docs); the name table now says the same thing
from the client. And the stat-set handler `FUN_141cbe8f0` compares **index 21's reason**
(`+0xf8`) with `0x21e3d3` = 2221011, Freezing Breath - the skill that freezes, on the index the
table calls `Freeze`.

Every row agrees with the v214 reference's order **shifted by one** (`PAD 11`, `Stun 19`,
`Freeze 20`, `Poison 22` there). The reference's **extras** do not shift cleanly, so the extras
below come from the listing only.

## Poison's shape

Block: `i32 value, i32 reason, i16 duration / 500 ms`, like every fixed status.

**One extra `u32`**, read after the word-1 and word-2 extras (`BT EAX,0x8` on word 0 at
`140471f34`), stored at `+0x120`. PDR's extra (`140471d03`) comes before it. Nothing else in the
extras section is gated on bit 23.

## What the client does with it

`FUN_141c72e50` is a **client-side poison tick**:

* returns unless `now - mob+0x50c >= 1000` - once a second; the stat-set handler stamps `+0x50c`
  when a status in its first mask group arrives;
* returns if the value `+0x114` is 0;
* `FUN_140499ea0(template, reason)` - the mob's per-template skill list. It answers *allowed* for
  a template with no list, so ordinary mobs pass;
* the number it draws is the value `+0x114` (`FUN_141d15f00(mob+0x3c0)` -> `+0x114`);
* `FUN_1429b6c90(DAT_143ac1b90, [+0x120])` - **`CUserPool::GetUser`** (named in
  `research/map-chair-seat-2026-09-09.md`) - so the extra is the **poisoner's character id**;
* `FUN_141c98cf0(mob, value, ...)` draws it, then `mob+0x50c = now`.

The client draws the number; it does not change the HP. The server's own ticks do that
(`world::session::poisonbreath`).

## Built

`net::mobstat::POISON = 23`, its extra on `MobStatus::extra`, byte-for-byte test
`poison_byte_for_byte`. Poison Breath sends it per poisoned mob, value = the server's per-tick
number, reason `2101004`, duration `dotTime`, extra = the caster's character id.

**Not yet seen on a screen.** What a live cast should show: a poison mark on the mob, and a
number over it once a second for five seconds equal to the HP the bar loses each second.
