# World names are bitmaps, and this client's order is not the classic one

**The owner, 2026-08-19:** *"You say the world is Scania, but the client actually displays the
world is Windia when I opened the change channel UI, why is that"*

## The name we send is never displayed

`world_list_entry` puts a name string on the wire and the login server logs "world Scania",
but **the client does not render that string.** The world name is a **canvas** - a bitmap -
at `UI.wz/Login.img/WorldSelect/world/<world id>`, with the pixels behind an `_outlink` into
`UI/_Canvas/_Canvas_000.wz`. So `--world` is cosmetic on our side only: it changes our log
line and nothing on screen.

This is the hazard `docs/ghidra.md` and [[maplecw-baked-ui-text]] both warn about - "Windia"
appears **nowhere** in the executable as ASCII or UTF-16 (checked, with `mapName` and
`MapleStory` as positive controls), and nowhere in `String.wz`, because it is pixels.

## The table, read by rendering every canvas

Rendered with `wz-dump canvas` + `tools/wz_png.py`. Index is the **world id**.

| id | name | id | name | id | name |
|---:|---|---:|---|---:|---|
| 0 | **SCANIA** | 16 | ELYSIUM | 32 | ENOSIS |
| 1 | BERA | 17 | KRADIA | 33 | NOVA |
| 2 | BROA | 18 | GALICIA | 34 | COSMO |
| 3 | LUNA | 19 | KALLUNA | 35 | ANDROA |
| 4 | ZENITH | 20 | MEDERE | 36 | CHAOS |
| 5 | CROA | 21 | CULVERIN | 37 | TITAN |
| 6 | ARKENIA | 22 | HACERLO | 38 | LEGEND |
| 7 | MARDIA | 23 | FLETA | 39 | ELF |
| 8 | PLANA | 24 | MERIEL | 40 | JUSTICE |
| 9 | STIUS | 25 | LEONA | 41 | RAVEN |
| 10 | UNION | 26 | ASTER | 42 | TEMPEST |
| 11 | DEMETHUS | 27 | DAR | 43 | PANTHEON |
| 12 | YELLONDE | 28 | RYUHO | 47 | ULTIMATE |
| 13 | KASTIRA | 29 | ENOSIS | 50 | AETHER |
| 14 | ELNIDO | 30 | NOVA | 51 | RED |
| **15** | **WINDIA** | 31 | COSMO | | |

(The tail of the sheet also carries PINKBEAN, ARCANE and NOVA as unnumbered extras; the
right-hand columns above are approximate past ~29 and were not needed to answer the
question. **Ids 0-16 are read directly and are the trustworthy part.**)

`Login.img/WorldSelect/BtWorld/release` offers buttons for ids
**0, 1, 3, 4, 5, 10, 16, 29, 43, 44, 50** - so those are the worlds this client's login
screen is built to show.

## What is NOT explained: where 15 comes from

We send world id **0** in `world_list_entry` (the first byte) and in `login_result`. The
client shows **15**. That is a real discrepancy and I have not found its source.

Candidates, none verified:

* Our `world_list_entry` layout was built partly by inference. If the client parses a later
  field of it as the world id for the in-game UI, a misalignment would produce a wrong id
  while the first byte still reads 0.
* `login_result`'s world/channel `u32`s may not be at the offsets we assume.
* The in-game UI may take the world id from the channel connection rather than from login,
  and the channel never sends one.

**It is cosmetic** - it does not affect entering the world, portals, or the channel swap. But
it is a live signal that one of those packets is not laid out the way we think, which is
worth more than the wrong name is.

> **Next step:** find what reads the world id in the Change Channel UI, or bisect by sending
> a distinctive world id (say 4, ZENITH) and seeing what the client then displays. The second
> costs one launch and would say immediately whether the client is reading our field at all.
