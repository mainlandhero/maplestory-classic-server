# Why the character is naked, and what dressing it needs

**The owner, 2026-08-19, from the screen:** the character stands on map 1 correctly but wears
nothing, while character select shows its four equips. Stored for `TestCharD` (id 204):
slot 5 `1040003`, slot 6 `1060002`, slot 7 `1072003`, slot 11 `1302000`.

## There is no shortcut: `SetField` does not carry the compact avatar look

The character **list** record dresses its characters with `FUN_1402ee8d0` - the compact
`(slot u8, item u32)` pairs terminated by `0xFF` that `crates/net` already builds and that a
live client has accepted. The obvious hope was that the `SetField` record carries the same
block. **It does not.** **[L]**

* `FUN_1402ee8d0` does not appear anywhere in `research/msexe-charrecord-full.c`, the full
  3548-line decompilation of the record decoder `FUN_140304b20`.
* `python tools/callers.py 0x1402ee8d0` gives 17 call sites in 16 functions. `FUN_1403094b0`
  - the character-list path - is one. `FUN_140304b20` is **not**.

So the field appearance is derived from the **equipped list**, and that block has to be
built properly.

## What is actually missing: the EQUIPPED list, not "the inventory"

**Corrected by the owner, 2026-08-19.** An earlier version of this note, and a report to them,
treated the empty inventory UI and the naked character as one problem. They are not:

* **Worn** items are equipped. They do **not** occupy an inventory slot.
* The **inventory** is the bag - what the character is *carrying*. Ours is genuinely empty,
  and showing it empty is **correct behaviour, not a bug.**

So there is nothing to fix about the inventory tabs, and dressing the character would not
put anything in them. The thing to build is the **equipped** list specifically.

That also explains the shape found below, and makes it a much better fit: the entry-6 gated
region reads a `u8` and then **two** `u16`-terminated loops. Two lists, read back to back,
before any bag - which is exactly `equipped` and `equipped cash`. The bag, if it is carried
at all in this record, is a later list and very likely a different presence flag.

## The block is gated by presence byte 2

**[D]** Entry 6 in the key table maps to **presence byte 2**
(`research/charrecord-presence-map.md`). **[I]** The reference's `DBChar.ItemSlotEquip` is
`0x4`, i.e. **bit 2** - the same ordinal logic that turned out to be right for
`Character = 0x1` / bit 0 / `presence[0]`, which is now confirmed on screen. Two lines
agreeing, but the reference half stays `[I]`.

Entry 6 gates **two** regions, and only one of them touches the wire **[L]**:

| region | packet reads |
|---|---|
| `[0x140305794, 0x140305d7d)` | **none.** Every `CALL` target enumerated; no `0x1406e8xxx`/`0x1406e9xxx` among them. Object construction and map insertion. |
| `[0x1403061c9, 0x14030661c)` | `u8` at `0x1403061cc`, `u16` at `0x1403061fc`, `u16` at `0x1403062c9`, plus `0x14030cbc0` x2, `0x14030b560` x2, `0x14030ca50` x1 |

**[I]** A `u8` followed by two `u16`-terminated loops each running an item sub-decoder is the
shape of "equipped" plus "equipped cash" - the two halves of the equip inventory. Shape only.

## The blocker: item decode is virtual, and items carry no RTTI

`FUN_14030b560` does **not** read the packet. It takes an already-constructed item and
inserts it into a map, and it reaches the item through a **vtable call** **[L]**:

```c
puVar5 = (undefined8 *)(**(code **)(*plVar7 + 0x330))();
```

So items are polymorphic and the decode lives behind a virtual method. `tools/rtti.py`
finds **1764** type descriptors but **no** `GW_ItemSlotEquip`, `ItemSlot`, `Equip` or
`Inven` class - the item classes carry no RTTI, exactly like the stage classes. The layout
therefore cannot be read off linearly the way the stat block was.

**Next step when this is picked up:** find the item vtables by their construction sites
(`0x14019b780` allocations inside the entry-6 regions give the object sizes), then read the
method at vtable `+0x330`.

## One thing I nearly wrote down and could not prove

The decompiled C contains a `do { ... } while (lVar17 < 0x20)` loop - 32 iterations, which
is the right order for an equip-slot table. **I could not place it in either region.** The
decompilation carries no addresses, and a grep of the no-reads region for a `0x20` loop bound
returns only `ADD RCX,0x20` and `CMP qword ptr [RAX+0x20],RDX` - struct offsets, not a trip
count. Per `docs/ghidra.md`, field order comes from the listing and not the decompiler; this
is that rule catching a structural inference before it became a fact.
