# Himmel's cape had no effect - 2026-09-12

The owner: *"There is a cape for Himmel's outfit. Himmel's cape should actually have an effect,
but this effect currently does not appear in our version of the game."*

Tags: **[L]** read off the client's bytes, its listing or a WZ tree; **[D]** derived; **[I]**
inferred.

## 1. The cape is its effect

`Character/Cape/01103918.img` (Himmel's Blessing) has `info` and thirty-odd action nodes,
and every frame is a **1x1** canvas. **[L]** The garment draws nothing; what a player sees
is the *item effect*, which MapleStory keeps in a different archive:
`Effect/ItemEff.img/<itemId>/effect/...`. The string desc agrees: *"Some outfit items may
appear behind this cape when equipped."*

## 2. The classic client has the loader and not the data

Format strings in the client, and who reaches them **[L]**:

| string | how it is reached | by |
|---|---|---|
| `Effect/ItemEff.img/%d/%s` (UTF-16, `0x1432aed90`) | `.data` pointer slot `0x143a47080` (`tools/dataref.py`) | five functions: three 97-byte wrappers that append `effect` (`FUN_140dbc260`, `FUN_1420f81b0`, `FUN_142880f00`, called from avatar code), `FUN_1420a90f0` (`effect_`), and **`FUN_141178760`** - called from `FUN_141177e80` and `FUN_141179970`, the character-select slot filler and placer |
| `Effect/ItemEff.img/%d` (UTF-16, `0x1432aed60`) | slot `0x143a47078` | `FUN_1427863f0` (the UserEffect switch, arm 17), `FUN_142818c70` (`removeEffect`), two effect-layer functions |
| `Effect/ItemEff.img/%d/effect/` (ASCII) | `lea` | `FUN_140f9e5b0`, whose callers are the preview window (`/Tab/Preview`, `/ActionComboBox`) |

The `%d/%s` readers take an item id and a child name, and the avatar and select-screen
callers mean **the client dresses worn-item effects from the worn list on its own** - no
packet carries them (the reference server's `SetActiveEffectItem` is a later addition for
choosing between several). **[D]**

And the classic `Effect/Effect_000.wz` holds **26 images and no `ItemEff.img`** - BasicEff,
DropItemEff, SetEff (13 bytes), PetEff... **[L]** The loader asks for an image that is not
there and draws nothing. Pure missing data.

## 3. What the modern client ships for 1103918

`Effect/Effect_000.wz/ItemEff.img` (5.2 MB, 2174 entries), key `1103918` **[L]**:

```text
effect
  fixed 0, z 10, action 1, actionExceptRotation 1
  stand1/0   canvas, origin (38,141), z 0, no delay
             _outlink Effect/_Canvas/ItemEff.img/1103930/1103918/effect/stand1/0
  stand2/0   UOL ../stand1/0
```

One frame, standing poses only. The pixels are in `Effect/_Canvas/_Canvas_052.wz/ItemEff.img`
- a single **244 MB** image - under holder `1103930` (its own twelve 107x104 `default` frames
plus our one **81x143** frame). **[L]**

## 4. What is installed

`tools/backport_install.py` step 1c: for every set item with an ItemEff node, `merge` the node
onto a **new** classic `ItemEff.img` in `Effect_000.wz`, and `merge` the holder subtrees the
outlinks name onto a new `_Canvas/ItemEff.img`. Built from the pristine `.bak` bases; on disk:

```text
Effect_000.wz          ItemEff.img   360 bytes    1103918/effect/...
_Canvas/_Canvas_000.wz ItemEff.img   45 787 bytes 1103930/{effect/default/0..11, 1103918/effect/stand1/0}
```

Both archives verify image by image.

## 5. Not established

* Whether the classic loader wants `effect/stand1/0` or `effect/default/0` - the action-keyed
  form is what Nexon ships and the loader takes a child name, so it is sent as is. If the cape
  slot fills and nothing glows, re-key `stand1` -> `default` (one `patch` line).
* Whether the effect persists while walking: the node has no walk frames. That would be the
  data, not this client.
* Anything on a screen. Plan step TO(h).
