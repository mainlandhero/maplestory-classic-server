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

## SETTLED: "Windia" is a hardcoded bitmap in the Change Channel UI

**RETRACTION.** This section previously said the client was "showing world 15" and that this
was "a live signal that one of our packets is not laid out the way we think", with three
candidate causes about `world_list_entry` and `login_result` being misparsed. **All of that
was wrong.** It came from assuming the Change Channel UI draws from
`Login.img/WorldSelect/world/<world id>`, the table above - and the owner said plainly they were
talking about the **Change Channel** dialog, not world select, which is a different UI with
its own assets.

What is actually there **[L]**:

```
UI/_Canvas/_Canvas_000.wz  ChannelChange.img
  Channel/
    world/
      0            <- a bitmap reading "WINDIA", and the ONLY child
    ch/ 0..18      <- the channel number bitmaps
    channel0..3, BtChange, BtCancel, t, c, s
```

`Channel/world` has **exactly one entry**. There is no index to select with, so no world id
we send can change it. The Change Channel dialog in this client build says WINDIA
unconditionally - it is a leftover asset, not a reflection of anything on the wire.

**Nothing is wrong with our packets, and nothing here needs fixing.** Changing it would mean
editing the WZ, which is a client modification and not something this project does for
cosmetics.

## The world-select screen is a third thing again

The owner: *"the 'world' on the world selection is actually 'Classic'... that's going to get
displayed on 'Choose another world'."* So that screen shows neither our name string nor
`WorldSelect/world/0` (which renders SCANIA). `Login.img` has a top-level **`ClassicIntro`**
node which is the obvious candidate **[I]** - not chased, because nothing depends on it.

## The lesson, since this is the second time

The table below is correct and was worth rendering. The mistake was reaching a conclusion
about **which UI** was involved without checking, then attaching a confident causal story to
it. "The client is showing world 15" was never observed - it was inferred from a table that
turned out to belong to a different dialog. Read the asset the user is actually looking at.
