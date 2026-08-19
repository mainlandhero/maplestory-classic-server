# Is map 1 loadable from the client's own WZ data?

Settled before spending a client launch, so that a failed run cannot be blamed on missing
field data. Tags: **[L]** read from a file, **[D]** derived from something read, **[I]**
inferred.

---

## 1. Headline — YES

**`Map/Map/Map0/000000001.img` exists.** [L]

| | |
|---|---|
| archive | `client-patched\Data\Map\Map\Map0\Map0_000.wz` (23,169,165 bytes, WZ v779, hash `0x0000E73A`) |
| node | `000000001.img`, size 21,532, offset `0x302E` |
| parses | yes — `wz-dump cat` produced 44,051 bytes of JSON, no error |

Note the name is **nine** digits, not eight. The client's own format string is
`Map/Map/Map%d/%09d.img` [L] (`research/msexe-fieldload.c:107`,
`PTR_u_Map_Map_Map_d__09d_img_143a45d78`; also found as a raw UTF-16 literal at file offset
`0x327F200` in `client-patched\MapleStory.exe`). The first `%d` is
`mapid / 100000000` [L] (visible in the same decompiled call). Map 1 therefore resolves to
`Map/Map/Map0/000000001.img` — exactly the node above. [D]

The same file is present, byte-identical, in the live install:
`C:\Nexon\Library\maplestorycw\appdata\Data\Map\Map\Map0\Map0_000.wz` — SHA-256
`566BB560D51C0FECD0C0012CF80225100672866891DECC6A10446B255F6A8A7B` on both. [L] So it does
not matter which of the two `Data` trees the launched client reads.

---

## 2. What is inside it

Top-level nodes of `000000001.img`: [L]

```
info (34 keys)   back (6)   life (2)   0..7 (layers)   reactor (0)
foothold (67)    ladderRope (1)        miniMap         portal (5)
```

### Spawn point — present, and it *is* index 0

All five portals, verbatim: [L]

| idx | pn | pt | x | y | tm | tn |
|---|---|---|---|---|---|---|
| **0** | **sp** | **0** | **-189** | **437** | 999999999 | "" |
| 1 | sp | 0 | -140 | 445 | 999999999 | "" |
| 2 | sp | 0 | -94 | 452 | 999999999 | "" |
| 3 | sp | 0 | -118 | 452 | 999999999 | "" |
| 4 | out00 | 2 | 1116 | 365 | 10 | in00 |

Our record sends `portal = 0`. Portal **0** exists, is type **0**, and is named **`sp`** —
so whether the client treats that byte as a portal *index* or as a spawn-point *id*, it
lands on the same node here. [D] (I did not verify which of the two the client actually
does; for map 1 the distinction is moot.)

Sanity check that the spawn is over ground: interpolating every foothold segment whose
x-span covers x = -189 gives exactly one candidate, `page2/grp0/id43`, at **y = 485** —
48 px below the portal. [D] Portals 1-3 likewise each sit 33-40 px above a foothold at
y = 485. A character dropped there falls a few pixels and lands.

### Footholds — present

**67 footholds** in three groups (`0/3` ×21, `2/0` ×43, `2/6` ×3). [L]
x-range -264 … 1254, y-range 50 … 750, inside the map's VR box
(`VRLeft -254, VRTop -310, VRRight 1244, VRBottom 675`). [L]

### Other data the field pulls in — all present [L]

| dependency | where | present |
|---|---|---|
| `info.mapMark = MushroomVillage` | `Map/MapHelper.img/mark` (19 marks) | yes |
| tile set `tile_grassySoil` | `Map/Tile/Tile_000.wz` | yes |
| back set `back_grassySoil` | `Map/Back/Back_000.wz` | yes |
| obj sets `obj_acc1`, `obj_connect`, `obj_door`, `obj_guide`, `obj_house`, `obj_houseGS` | `Map/Obj/Obj_000.wz` | all yes |
| life `0000001` (Heena), `0000002` (Sera) | `Npc/Npc_000.wz` → `0000001.img`, `0000002.img` | both yes |
| miniMap `_outlink` → `Map/Map/Map0/_Canvas/000000001.img/miniMap/canvas` | that archive, that node, 93×61 canvas | resolves |

---

## 3. Is map 1 a placeholder or otherwise a bad choice? — No. It is the *best* choice.

I surveyed **all 426 field images** in the tree (423 in `Map0_000.wz`, 3 in
`Map9_000.wz`) rather than checking a shortlist. All 426 parsed, zero failures. [L]

Across the whole set: [L]

| property | count out of 426 |
|---|---|
| has `info/link` | **0** |
| zero footholds | **0** |
| missing portal key `0` | **0** |
| portal `0` with `pt != 0` | **0** |
| has `onFirstUserEnter` | **0** |
| has `onUserEnter` | **1** (only `80000000`, `PQ_01_entered`) |
| has `fieldScript` | **0** |
| `fieldLimit != 0` | 129 |

So there are no link-maps and no script-gated entries anywhere in this client's data, and
*every* map has a type-0 portal at index 0. Map 1 is not special in any of those ways. [D]

Map 1's own `info`: [L]

```
town 1   cloud 0   fieldLimit 0   fieldLimit2 0   fieldType 0   swim 0   fly 0
mobRate 1.0   returnMap 1   forcedReturn 999999999   moveLimit 0   quarterView 0
fieldScript ""   onFirstUserEnter ""   onUserEnter ""   bgm Bgm00/FloralLife
```

`returnMap = 1` points at itself and `forcedReturn = 999999999` is the "none" sentinel, both
normal for a town. [I]

Positive reasons map 1 is a good first target: [D]

- **0 mobs and 0 reactors.** `life` holds two NPCs and nothing else. The server has to send
  no mob-spawn and no reactor state for the field to look correct.
- **`town = 1`**, `fieldLimit = 0`, no scripts — nothing the server must implement to let a
  character stand there.
- It is the lowest-numbered map and the one the client's own String tree lists first.

Only 6 maps in the whole client are 0-mob **and** 0-reactor **and** `town = 1` in the
low-id Maple Island block: **1, 10, 20, 21, 60, 61**. [D] If map 1 ever needs a
substitute, **10 ("Mushroom Town")** is the closest equivalent — same properties, one spawn
portal, 41 footholds — and map 1's own `out00` portal already targets it (`tm = 10`), so
both ends of that link have data. But there is no evidence-based reason to prefer it.

Maps to **avoid** for a first run, on this data: `40/41/42` and the `1000x` block (34-66
mobs each), `1000` and `101x` (reactors), and `80000000` (the one `onUserEnter`). [D]

---

## 4. Is `String.wz`'s `mapName` for 1 reachable the way the client reaches it?

**The entry exists and is unambiguous.** [L]

```
String/Map.img / MapleIsland / 000000001
    streetName = "Maple Road"
    mapName    = "Mushroom Town - West Entrance"
```

`String_000.wz` → `Map.img` has six categories (`MapleIsland`, `VictoriaIsland`, `Ossyria`,
`Other`, `Event`, `Dev`) and the id keys are **zero-padded to 9 digits**, same as the Map
archive. Id `000000001` appears in exactly one category. [L]

What I can show about the client's own path: the exe carries the UTF-16 literals
`String/Map.img` (offset `0x32AAC40`), `mapName` (`0x32B0BE0`) and a bare `%09d`
(`0x32831C8`) [L], and `research/msexe-stage-setfield.c:538` calls
`FUN_1403999e0(DAT_143aa8328, &out, <map id>, "mapName")` — a lookup keyed by the integer
map id with the property name passed in [L]. That the helper zero-pads the id to 9 digits
before indexing `String/Map.img` is **[I]** — it is the only formatting that matches the
on-disk keys, and the bare `%09d` literal is consistent with it, but I did not read the
helper's body (Ghidra was unavailable for this task).

Practically: `gm-handbook/maps.txt` line 1 reads `1, Mushroom Town - West Entrance`, and
`tools/dump_names.py` reaches it by the same recursive descent, converting `000000001` →
`1`. [L]

---

## Instrument verification

A negative here would have been worthless without these, so they are stated explicitly.

**Positive control — WZ walker.** Map **40**, "Snail Hunting Ground I" from
`gm-handbook/maps.txt`: `000000040.img` is present (68,895 bytes) and `wz-dump cat` returned
a full `info` block (`bgm Bgm00/RestNPeace`, `returnMap 60`, `mapMark MushroomVillage`). [L]
The walker can find a map I knew should be there.

**Bulk control.** All **426** images in the Map tree were parsed end-to-end; **0** parse
failures. [L] The walker is not silently skipping images.

**Positive control — exe string scanner.** Before trusting any "not found" from the binary
scan, I required it to recover a string I already knew from `research/msexe-fieldload.c`:
out of 8,726 UTF-16 literals it found `Map/Map/Map%d/%09d.img`, one hit, at `0x327F200`. [L]

**Discrimination control — name vs. data are genuinely independent here.** Cross-referencing
the 432 named ids in `String/Map.img` against the 426 field images: [D]

- **12 ids are named but have no field image**: `90040111`–`90040113`, `90040121`–`90040123`,
  `90040131`–`90040133`, `90040141`–`90040143` (all "Hidden Street: Up and Up <Level n>").
- **6 field images have no name**: `80003000`, `80003100`, `80003200`, `80003300`,
  `80003400`, `80003500`.

So the instrument does return "named but absent" and "present but unnamed" when those are
the truth. Map 1 came back **present in both**. That is the check the task was really
asking for — a name in `String.wz` is not field data, and here the two were tested
separately and both passed.

**Negative control attempted, and why it is weak.** `104040000` is absent from `Map0_000.wz`
[L] — but it is also absent from `String/Map.img` [L], and there is no `Map1` directory at
all (the tree holds only `Map0` and `Map9`) [L]. Every id in the archive buckets correctly:
423 ids with `id/100000000 == 0` (max `90050001`) and 3 with `== 9` [D], so no `Map1..Map8`
directory is missing — this client simply has no Victoria/Ossyria-numbered fields on disk
under those ids. `104040000` is a control for "id not in this client" rather than for
"named but no data"; the 12 `9004xxxx` ids above are the real control for that.

---

## What I could not determine

- **Whether the client's `%09d` map-name lookup is the one at `0x32831C8`.** Marked [I]
  above. Settling it needs the body of `FUN_1403999e0`, i.e. Ghidra, which was held by
  another process for this task.
- **Whether the SetField `portal` byte is a portal index or a spawn-point id.** Not read
  from the client. Irrelevant for map 1 (portal 0 is the `pt = 0` `sp` node either way) but
  it will matter the first time a map is chosen where those differ.
- **Which `Data` tree the launched client actually opens** (`client-patched\Data` vs
  `C:\Nexon\...\appdata\Data`). Not determined — but the Map and String archives are
  SHA-256 identical between the two [L], so the answer to every question above is the same
  for either.
- Nothing here says the *protocol* is right. This rules out one failure mode only: if the
  run fails, it will not be because map 1's field data is missing.

---

## Addendum from the main session: the "needs Ghidra" question, answered

`FUN_1403999e0` was already in `research/msexe-fieldload.c`, and its body settles the
open item above. It is **not** a String.wz name lookup - it is a generic
"get property `key` for map `id`" helper with a fallback:

```c
local_res18[0] = param_3;                      // the map id
local_res20   = param_4;                       // the key, e.g. "mapName"
FUN_1403fcd90(&local_a0, &local_res8);         // the primary lookup
if (local_res8 == NULL || *(char *)local_res8 == '\0') {
    // MISS -> build the field image path and read the map itself
    FUN_1401c21c0(&local_e0, PTR_u_Map_Map_Map_d__09d_img_143a45d78,
                  (ulonglong)local_res18[0] / 100000000);
    ...
}
```

So `SetField`'s `FUN_1403999e0(ctx, &out, mapid, "mapName")` looks the name up first and only
opens `Map/Map/Map%d/%09d.img` when that misses or returns an empty string. **[L]**

For map 1 this matters twice over, and both are already confirmed above: the primary lookup
**hits** (`String/Map.img/MapleIsland/000000001` -> `"Mushroom Town - West Entrance"`), so the
fallback is not even reached; and if it ever were, `Map/Map/Map0/000000001.img` exists and
parses. The `%09d` padding is now **read** rather than inferred for the image path.

**One hazard on the fallback path only.** After the miss it does:

```c
if (DAT_143add058 == (IUnknown *)0x0) { FUN_142ef3ac0(0x80004003); }   // does not return
```

`0x80004003` is `E_POINTER`, and Ghidra marks the call as non-returning. That is a **fatal**
path if that global is null. It is not on map 1's path, because map 1's name lookup hits -
but it is worth knowing that a map id with no `mapName` entry can take the client somewhere
that does not return. **[L]** for the code, **[I]** for what `DAT_143add058` is.
