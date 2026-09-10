# A script portal sends `0x014A`, not `0x00D1` - 2026-09-10

The owner, the morning after the Ellinia Station portal was "fixed": *"The portal in Ellinia to go
to Ellinia Station still currently does not exist."*

## What the fix of 2026-09-09 did, and why it changed nothing on screen

`tools/dump_portals.py` gained the `script` column, `world::scriptportals` derived the
destination of `pt_10002000_in03` from the unique reverse link, and `Config::load_portals`
folded it into the same `(map, portal) -> (map, portal)` table a walk uses. All of that is
correct and all of it is still there. It assumed the client would ask through the ordinary
transfer-field request `0x00D1`, the way it does for a `pt 2` door. It does not.

## What is measured [L]

**The capture.** `world.log`, 2026-09-10, Cobalt standing on the spot (portal 38 is at
`(819, -3072)`):

```text
11:27:53.035 <- 0x014A UNKNOWN, 11 byte body 000400696e30333a0303f4
11:27:53.926 <- 0x014A UNKNOWN, 11 byte body 000400696e30333f0302f4

  00              u8      0
  0400 696e3033   str     "in03"
  3a03            i16     826      (second press 831)
  03f4            i16     -3069    (second press -3069)
```

Nothing followed either one. The server named the opcode UNKNOWN and, having no handler,
answered nothing.

**It is not one run.** Every `0x014A` in `world.log`, `previous-runs/` and
`research/fixtures/`, deduplicated by body: **twelve**, across three runs
(2026-08-22, 2026-08-30, 2026-09-08 and today), every one carrying `"in03"`, x within
`818..833`, y within `-3071..-3067`. The one portal anybody has ever tried this on is this one.

**The builder agrees.** `research/msexe-send-opcodes.txt` names two builders for `0x014A`,
`FUN_1428b2330` and `FUN_1428b4550`; `research/msexe-packet-fields.txt` gives the first one's
encode sequence as `u8, FUN_1406edc80 (str), FUN_1406ed940, FUN_1406ed940` - a byte, a
length-prefixed string and two 16-bit fields. That is the captured shape, read from the
client rather than from the capture.

**Which portals send it.** The four Free Market doors are `pt 7` and the two derived script
portals are `pt 8`:

```text
010001040.img  21  market00  pt 7  script market01
010004000.img  24  market00  pt 7  script market02
020001010.img   9  market00  pt 7  script market03
080002000.img  16  out00     pt 7  script market00
010002000.img  38  in03      pt 8  script pt_10002000_in03
010002071.img   9  down00    pt 8  script pt_10002071_down
```

So the Free Market wiring of 2026-09-09 was dead in exactly the same way, and step TF of the
plan could not have passed. Nobody had reached it yet.

## The sibling, named but not handled: `0x014C`

The same log has `0x014C` at 11:27:41, `00 | "h_under00" | "h_top00" | i16 458 | i16 181 |
i16 -924 | i16 -2472 | 00`, and five archived bodies with the same layout (`up00 -> in03`,
`up00 -> out00`, `in01 -> in01_1`). Two portal names and two positions: the client hopping
between a map's own `pt 1` hidden portals, telling the server after the fact. It is named in
`net::names` so the log stops calling it UNKNOWN, and left unanswered on purpose: the client
completed every one of those hops itself, and the session carried on each time, so it does
not latch. Answering it with a `SetField` would re-enter the map under the player.

## What is inferred [I]

That an unresolved `0x014A` should be answered with the stat-changed unlock rather than
silence. Twelve unanswered ones did not freeze the client, so it may not latch at all - but
the project rule is to answer, and the unlock is the answer every other unhandled request
gets.

## Wired

`net::portalscript::parse_portal_script`, `session::field::on_portal_script`, dispatched from
`Session::handle`. Resolution is shared with `on_transfer_field` - Free Market door first,
then the portal table - so a script portal and a walked door cannot drift apart. Test plan
step TK(a), then TF.
