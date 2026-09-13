# Summoning a pet - 2026-09-13

The owner: *"I tried summoning the Husky pet, but the pet does not come out."*

Tags: **[L]** read off the client's bytes or listing; **[I]** inferred; **[R]** the reference server.

## 1. The click

`world.log` 18:02:29 and 18:02:30: `<- 0x0147 UNKNOWN, 6 byte body 509a1814 0100` / `f29d1814 0100`.
Nothing handled it. `tools/encodes.py 0x142d4ced0` (the builder `msexe-send-opcodes.txt` names for
`0x0147`): `CTOR, w_u32, w_u16, SEND` - **`u32 tick, u16 slot`** **[L]**. Slot 1 is where the Husky
sat in the Cash tab. The reference's `handleUserActivatePetRequest` reads `tick, short slot` (and a
`bossMode` byte this client does not write) and **toggles** on the item's `activeState` **[R]**.

## 2. The answer, and how it was found

Pet packets in the reference are per-user (`encodeInt(ownerId)` first). This client's per-user
router is table A of `research/user-pool-tables.md` (`0x226..0x276` dense) plus four out-of-table
ranges. None of the dense rows read a string and a `u64`; the first range, `0x277..0x27E`, goes to
`FUN_142795b20`, which reads `u32 petIdx`, looks the pet up (`FUN_1427703d0` - index 0 only), and
switches (`research/msexe-pet-onpacket.c`):

```text
0x0277  user vtable +0x98          PetActivated   (local FUN_1428a01a0, remote FUN_1429d6150)
0x0278  FUN_141ec3f20  u32 u16 u8
0x0279  FUN_141ec3fa0  u8 str
0x027a  FUN_141ec4050  str
0x027b  FUN_141ec4660  str
0x027c  FUN_141ec5750  u32
0x027d  FUN_141ec5980  u16 u8
0x027e  FUN_141ec4780  u8 u32
```

The vtables: `0x1434831e0` is the one `CUser` in a solo process (`research/chairs-2026-09-08.md`) -
the local user; `0x143486b70` the remote. Both `+0x98` handlers read `u32 petIdx` (must be 0),
`u8 activated`, and if set `u8 init` then `CPet::Init` (`FUN_141eb9760`, reached from a pointer slot
to `Item/Pet/` - `tools/xref.py` sees `lea` only, so the slot was found by a rip-relative scan). Its
read order, `tools/listing.py 0x141eb9760 | grep READ` **[L]**:

```text
u32 itemId, str name, raw8 serial, u16 x, u16 y, u8 moveAction, u16 foothold,
u32, u32, u16, u16, u8, u8
```

The last six are named from the reference's `Pet.encode` and older builds' tail **[I]**: hue,
itemId again, wonderGrade, giantRate, nameTag, chatBalloon. `activated = 0` reads nothing further
and calls the pet setter with null - the put-away.

## 3. What the server does now

`net::pet`, `session/pet.rs`. A `0x0147` finds the pet at that Cash slot; summons it with `0x0277`
to this client and the map (position = last reported, foothold under it); re-sends the item at its
slot with `active = 1` and a pairing serial (the store keeps no cash serials, so
`pet_serial(character, item)` is used on both sides); closes the request with the empty `0x0070`
`bExclRequestSent` this codebase uses as its unlock. The same click again puts it away. Every field
entry re-sends the pet. The record's pet body carries the `active` byte from the session, so a relog
puts the pet away, as the reference's `initPets` would for a row that says inactive.

## 4. Not built

Pet movement, feeding, naming, and the rest of `0x0278..0x027E`. The client's own pet sends are not
decoded: `0x0148` (`u32,u32,u32,u8`, two builders) and `0x0149` (empty) are named as pet requests
of unknown meaning; whatever the client sends after a summon is the next thing to read.

## 5. Unverified

Everything in section 3 is built to the client's read order and is **unverified on screen** - the
launch is plan step TO(u). The three things a run can say: the pet appears (the shape is right); the
client dies at the summon (the pet body is wrong somewhere in the six tail fields); the pet appears
but the item does not show as summoned (the pairing serial is not what the item holds).
