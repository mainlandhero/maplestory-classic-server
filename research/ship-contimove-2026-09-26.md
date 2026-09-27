# The station ship: `0x01BF` and `0x01C0`, read 2026-09-26

The owner: *"When the boat 'arrives' at xx:x5 to the station so players can board, there should be an
opcode that the server can send to animate the boat arriving ... All other times, the player
should be sent the opcode that animates the boat leaving the station."*

Decompilation: `research/msexe-contimove.c` (`FUN_140d6cc90`, `FUN_140d6cda0`, `FUN_140d6aba0`,
`FUN_140d6a610`, `FUN_140d69b90`, `FUN_140d6cb50`). Labels: **[L]** read off the listing or WZ,
**[D]** derived from two [L] facts, **[I]** inferred.

## The map says it is a ship field [L]

`Map0_000.wz/010002090.img` (Ellinia Station):

```text
info/fieldType = 2
shipObj { shipObj "Map/Obj/obj_vehicle.img/ship/ossyria/99", x 1545, x0 2100, y -195,
          z 0, f 0, tMove 15, shipKind 0 }
```

`020000022.img` (To Orbis) is also `fieldType 2`, with `shipKind 1` and no `x0`. Four maps in
the client have `fieldType 2` (`research/client-messages.md`). `020000010` (Orbis Ticketing
Booth) is `fieldType 0` with no `shipObj`.

## How it was found

The key names are UTF-16 strings reached only through a pointer table: `shipObj` at
`0x1432b11d8` is pointed to from `0x143a47d60`, `shipKind` from `0x143a47d68`, `tMove` from
`0x143a47d70` and `x0` from `0x143a47d80` [L]. `tools/xref.py --string shipObj` finds **0**
references because nothing `lea`s the string. `tools/dataref.py` on the pointer slots finds one
reader: `FUN_140d69b90`, the ship loader [L]. Its only caller is `FUN_140d6cb50`, which is slot 1
of the vtable at `0x1433d8250` [L]. Beside it, `FUN_140d6cc90` tail-jumps to `CField::OnPacket`
(`FUN_141820080`) - it is one of the 25 tail-jump callers `tools/callers.py` lists [L]. That is
the ship field's own packet handler.

## `FUN_140d6cc90` - the ship field's OnPacket [L]

```text
140d6cca2  sub ecx, 0x1bf ; je  -> 0x01BF  (jmp FUN_140d6cda0 with this-0x18)
140d6ccae  cmp ecx, 1     ; je  -> 0x01C0  (inline)
140d6ccc0  jmp 0x141820080         everything else: CField::OnPacket
```

On any other field, `0x01BF` and `0x01C0` land in `CField::OnPacket`'s default arm
(`research/msexe-field-cases.txt` rows 49-50), which range-checks `op - 0x224` and does nothing
for an opcode below `0x224` [L]. A stray one on the wrong map is ignored.

## `0x01C0` - ship state: `u8 state, u8 flag` [L]

Two `Decode1` (`FUN_1406e8ae0`), then a 7-entry table at `0x140d6cd74`:

| state | arm | effect |
|---|---|---|
| 0, 1, 6 | `140d6ccf9` | if `shipKind == 0`: `FUN_140d6aba0` - **arrive** |
| 2, 5 | `140d6cd38` | if `shipKind == 0`: `FUN_140d6a610` - **leave** |
| 3, 4 | `140d6cd1e` | if `shipKind == 1 && flag == 1`: `FUN_140d6b130`; then as 2/5 |

`shipKind` is the field's `+0x110` (read as `[rbx+0xf8]` from the `+0x18` subobject) [L].

## `0x01BF` - ship move: `u8 type, u8 state` [L]

`FUN_140d6cda0`: `Decode1` type, `type - 7` checked `<= 5`, a table at `0x140d6cfcc`:

| type | reads | effect |
|---|---|---|
| 8 | `u8`, must be 2 | `FUN_140d6a610` - **leave** |
| 10 | `u8`: 4 or 5 | 4: `FUN_140d6b130` plus a sound effect (the string at id `0x58d`); 5: `FUN_140d6bdb0` |
| 12 | `u8`, must be 6 | `FUN_140d6aba0` - **arrive** |
| 7, 9, 11 | - | nothing |

## Which one is arrive [L] / [D]

Both routines start with `if (ship.layer != 0 && ship.+0x20 == 0)`, then play `Whistle`
(`PTR_u_Whistle_143a48fd0`), then position the layer and start a timed move. The duration is
`ship.+0x34 * 1000` ms [L].

The loader stores its keys in read order: `shipKind -> +0x20`, `x -> +0x24`, `y -> +0x2c`,
`f -> +0x38`, `tMove -> +0x34`, and, only when `shipKind == 0`, `x0 -> +0x28` [L, by read order
against the pointer-slot reads at `140d69d58`, `140d6a07b` and `140d6a2dc`].

* `FUN_140d6aba0` places the layer at `(+0x28, +0x2c)` = `(x0, y)` and moves it to `(+0x24,
  +0x2c)` = `(x, y)`. At the station: from 2100, past the right edge (`VRRight 1910`), in to
  1545, over 15 s. **Arrive.** [D]
* `FUN_140d6a610` places it at `(x, y)` and moves it to `(x0, y)`. **Leave.** [D]

So the ship only animates at a `shipKind 0` field - the station - and both packets are
animations, not placements: every one plays the whistle and the 15-second slide.

## What the server sends (`net::ship`, `session/boat.rs`)

* **On entry to the station:** `0x01C0 [1, 0]` while the ship is in (from `:x5:00` until
  the departure), `0x01C0 [2, 0]` at any other time.
* **At the moment it changes:** `0x01BF [12, 6]` to everyone on the station at `:x5:00`, and
  `0x01BF [8, 2]` at the departure.

None of this has been seen on a screen. The one [I] worth watching is whether the ship is
drawn before any packet arrives. If it stands docked on entry and then slides out, the leave
animation plays from the dock, which is what the owner asked for. If it is invisible until a packet
comes, the same packets still animate it.
