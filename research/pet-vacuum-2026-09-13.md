# Every pet is a vacuum pet, and the client does the vacuuming - 2026-09-13

The owner: *"turn every pet into a vacuum pet, similar to a modern maple Luna Petite pet, which sucks up
loot in a radius around them provided that they are from a mob death drop"*, then: *"I want to
offload most of the pet driven operations on the client. Please figure it out on the client side."*

Tags: **[L]** read off the client's bytes or listing; **[I]** inferred; **[R]** the reference server.

## 1. The split, as this client has it

* The **client** decides what a pet reaches for and when, and asks with a pick-up request. The
  **server** performs the pick-up: ownership, lifetime, the bag write, and the `0x046F` leave
  that removes the drop for everybody - for a pet, leave type **5** (`u32 charId, u32 petId`,
  `net::drops::leave_type::PET_PICKUP`, read at `0x1417b0d2c`) so the item flies into the pet.
* The drop's `0x046E` carries a per-drop **`canBePickedUpByPet`** byte (`drop+0x160`) **[L]** -
  the switch that says which drops a pet may go for.

## 2. The client's own knobs - data, not code

The client names, as UTF-16 WZ keys, `pickupItem`, `pickupAll`, `sweepForDrop`, `longRange`,
`ignorePickup`, `multiPet`, `noRevive`, `autoBuff`, `consumeHP`, `consumeMP`, `hungry` **[L]**.
Their pointer slots are read by the pet loader `FUN_1403e54e0` and by the pet itself
`FUN_141ed4490` (19 KB; found by a rip-relative scan for the slots - `tools/xref.py` sees `lea`
only). The client's string table has *"auto-loot function"*, *"Item Pouch"*, *"Meso Magnet"*.

The modern archive (`C:\Nexon\Library\maplestory\appdata\Data\Item\Pet\Pet_000.wz`, 1561
pets) says what a vacuum pet **is** in data: **370** pets carry `pickupItem 1, sweepForDrop 1,
longRange 1` (first: `5000040.img`); `pickupAll`/`pickupMeso`/`ignorePickup` appear on none. The
classic eleven carry `pickupItem 1` already.

So the installer's step 4c now writes `sweepForDrop 1` and `longRange 1` onto every classic pet
beside the `life 0` / `permanent 1` it already wrote. **What "sweep" and "long range" do on
screen - the radius, whether it is the pet's or the screen's - is Nexon's code in
`FUN_141ed4490`'s family and is not read; it is measured by the launch.** [I] that this build's
pet code acts on them: the loader stores them, and 370 modern pets ship them.

## 3. The server's half

* `LiveDrop::from_mob` - true only for the kill path (`combat.rs` sets `DropFromMob::from_mob`;
  the reactor path sets it false; a player's ground drop and a coin drop are false). It goes out
  as the drop's `canBePickedUpByPet` byte, so the pet leaves everything else alone.
* `DropTable::take_by_pet` - the ownership rules of `take`, plus `from_mob`, and the type-5 leave.
  A pet asking for a non-mob drop gets `PickUp::NotForPets`: the unlock alone, no chat line.
* `Session::on_pick_up` - the player's shape first (id at byte 13, measured); if that names no
  live drop and a pet is out, the pet's shape (id at byte **17** - the reference's
  `PET_DROP_PICK_UP_REQUEST`: `u32 petIdx, u8 fieldKey, u32 tick, u32, i16 x, i16 y, u32 dropId,
  u32` **[R]**). The builder is in `.themida` like the player's, so the offset and the opcode
  are settled by the first capture; the log line *"pet pick-up: 0x.... names live drop"* is it.

## 4. Unverified, and what the run says

Plan step TO(u). With the Husky out and a mob killed nearby: the pet goes to the drop and it
lands in the bag with no click - the trio works and the request shape held; the pet goes to the
drop and nothing happens, with an inbound opcode in `0x0329..0x032E` logged as naming no live
drop - the offset is not 17, paste the body; the pet never moves toward drops - the keys are not
what this build's pet code reads, and `FUN_141ed4490` is the next thing to decompile.
