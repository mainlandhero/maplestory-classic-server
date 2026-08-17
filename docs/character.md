# Characters: the list, the record, and the creation transaction

Everything here was read statically on 2026-08-18. **None of it has been on the wire yet** -
say so when reporting, and see the "what is not established" section at the end.

## CharSelect is not a stage

`FUN_141b3f050(this, screenId, delay)` writes `this + 0x238` and nothing more. Screens
**3 (ClassicIntro), 4 (CharSelect) and 5 (NewChar) are sub-screens of a single login-stage
object**, so every character packet arrives through `FUN_141b25f30`, the login-stage switch.
There is no second `OnPacket` to find, and looking for one wasted a pass.

The nine sibling stages, found by scanning `.rdata` for the base-class run every stage
vtable shares (`FUN_141d5f590`) and reading each vtable's `OnPacket` slot:

| OnPacket | Opcodes | What it is |
|---|---|---|
| `FUN_141b25f30` | the login set | **the login stage - all character traffic** |
| `FUN_141b82b00` | `0x51`-`0x6f` | buddy / messenger |
| `FUN_141072ec0` | `0x5ac`-`0x5bf` | - |
| `FUN_141df5940` | `0x1001`-`0x1007` | - |
| `FUN_142097ee0` | (×4) | a shared no-op base |

## The character manager

`FUN_14108…` is the character-manager family, and it is the tell for anything
character-shaped:

| Function | Role |
|---|---|
| `FUN_14108e7f0` | construct an empty character |
| `FUN_1403094b0` | **decode one character record from a packet** |
| `FUN_140302e30` | the stat block inside that record |
| `FUN_14108bdf0` | **decode a whole character list** |
| `FUN_14108d9b0` | look a character up by id |
| `FUN_14108c210`, `FUN_14108cd40`, `FUN_14108c500`, `FUN_14108cc50`, `FUN_14108d140`, `FUN_14108d260` | per-character operations |

## The character list is inside `0x0010`

Not a packet of its own. `FUN_14108bdf0` is called from the login result's success path -
the part `docs/opcodes.md` used to describe only as "two further sub-readers":

```text
u32
u32
...
u8    count
      repeat count times:
        FUN_14108e7f0()      construct
        FUN_1403094b0()      decode one character (below)
```

**That is why "Create a character" sends nothing.** The client reached CharSelect with an
empty list because our `0x0010` body is zero-padded, so `count` read as 0.

## One character record - `FUN_1403094b0`

```text
FUN_140302e30(record, packet, 0)     the stat block, below
u32
u64
u32
u64
```

### The stat block - `FUN_140302e30(record, packet, 0)`

The `param_3 == 0` branch is the one `FUN_1403094b0` uses. Field offsets are into the
record, and they are contiguous and unaligned, which is a good cross-check that the read
order is right:

```text
u32   -> +0x00   character id
u32   -> +0x04
u32   -> +0x08
13B   -> +0x0c   name, a fixed 13-byte field (not a length-prefixed string)
u8    -> +0x19
u8    -> +0x1a
u32   -> +0x1b
u32   -> +0x1f
u32   -> +0x23
u32              -> obfuscated into +0x27 / +0x2b (see below)
... continues: several u32 blocks, a u64, a 4-iteration loop, u8, u8, 8 bytes, u32, u32
```

**The obfuscation is client-side only.** After reading a `u32` the client does
`(seed ^ value) >> 5 | (seed ^ value) << 27` and stores that. The value **on the wire is
plain**; this is the client scrambling its own memory, not a transform we have to apply.
The same pattern appears in `0x0010`'s token handling, where it was briefly mistaken for a
character loop.

## The creation transaction

Three opcodes, all in the login-stage switch, and the order is the client's:

| Inbound | Handler | Body |
|---|---|---|
| **`0x0014`** | `FUN_141b33f30` | `str name`, `u8 result` - **the name check**, switched on the result |
| **`0x0015`** | `FUN_141b36a10` | `u8 result`; if `0`: `u32`, one character record, `u8` |
| **`0x0016`** | `FUN_141b34970` | `u32 characterId` - the delete result |

`0x0014` is the one the owner identified from the live game: the client will not let creation
proceed until the server has approved the name.

**`0x0015`'s success path** decodes the new character with `FUN_1403094b0`, registers it,
and calls `FUN_141b3f050(stage, 4, 0x14a)` - back to CharSelect with the new character in
the list. Its failure path raises a notice through `FUN_141b4ac80` and returns to the same
screen, so a non-zero result is how the server refuses.

Two neighbours in the same family, not yet needed: `0x0017` (`FUN_141b359e0`) and `0x0018`
(`FUN_141b365f0`), both `u32 characterId` plus a per-character call.

### The outbound side

Outbound opcodes are statically readable - every `FUN_1406ed520(buf, op)` call site names
one - so these were read rather than guessed:

| Outbound | Builder | Body | Pairs with |
|---|---|---|---|
| **`0x0081`** | `FUN_141b28950` | one string: the name | **`0x0014`** |
| `0x00A8` | `FUN_141b282d0` | one string | - |
| `0x00A9` | `FUN_14108d8d0` | `u32`, `u32 count`, `count × u32` | - |
| `0x008B` | `FUN_141b28750`, `FUN_141b2cb70` | `u32` (a character id) | - |
| `0x008C` / `0x008D` | `FUN_141b2d860` / `FUN_141b2da30` | - | - |

**`0x0081` ↔ `0x0014` is confirmed by more than adjacency.** `FUN_141b28950` sets
`stage + 0xd4 = 1` after sending, and `FUN_141b33f30` - the `0x0014` handler - opens by
writing `stage + 0xd4 = 0`. Same flag, set by the request and cleared by the response.
`FUN_141b28950` also validates the name locally first and raises `cannotUseThisName`
without sending, so a rejected name never reaches the server.

**The create request has not been found yet.** Ruled out by reading them: `0x0074`, `0x0075`,
`0x0082`, `0x008B`, `0x008C`, `0x008D`, `0x00A8`, `0x00A9`, `0x00C0`. The next place to look
is the **NewChar screen (screen 5)** and whatever its Create button calls - the same route
that found the Login button's handler. It must carry a full character spec (name, job,
face, hair, skin, starting stats and items), so it will be a builder with many
`FUN_1406ed840`/`FUN_1406ed9d0`/`FUN_1406edc80` writes - which is a cheap thing to grep for
across `research/msexe-send-opcodes.txt` once the screen's handler names a candidate.

## What is *not* established

* **The rest of the stat block.** The head is solid to `+0x23`; past that it is several
  `u32` blocks, a `u64`, a four-iteration loop, then `u8, u8, 8B, u32, u32`. Building a
  character needs all of it, because the readers throw on underrun and a misaligned record
  would corrupt every field after the mistake without any error.
* **The client's outbound side.** Which opcodes it *sends* for name-check and create have
  not been read. They are statically readable - every `FUN_1406ed520(buf, op)` call site
  names one - so this is cheap, and it is what the server has to answer.
* **Everything above is unmeasured.** Three separate static chains in this project looked
  this convincing and were wrong. Send it before believing it.
