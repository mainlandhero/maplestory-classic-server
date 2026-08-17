# Characters: the list, the record, and the creation transaction

Read statically on 2026-08-18 and 2026-08-19. **None of it has been on the wire yet** - say
so when reporting, and see "what is not established" at the end.

## What this client is

**MapleStory Classic World**: a modern MapleStory client engine running classic content.
The owner confirmed it, and the binary says the same thing from both directions - the character
record carries modern fields (`characterIdForLog`, `worldIdForLog`, an extended-SP table)
while the character creation screen is pure classic: four explorer job branches, a
STR/DEX/INT/LUK roll that must total 25, and prev/next pickers for top, bottom, shoes and
weapon.

That mixture is why a single reference version never matches, and why every field below was
taken from this binary rather than from a table.

## A second source, and what it is good for

`C:\Users\user\Desktop\ModernMapleSource` (the owner, 2026-08-19) holds a Swordie-family server
for a modern version. It is **not** this protocol - opcode numbers differ, and its record
has fields this client does not read. What it is good for:

* **Names for fields we had only as offsets.** `CharacterStat.encode` and this client's
  `FUN_140302e30` agree, in order, for the entire head of the record. Two decoders derived
  independently and landing on the same sequence is much stronger evidence than one static
  read, and it is what turned "u32, u32, u32, 13 bytes, u8, u8..." into named stats.
* **Structures to look for.** Its `selectWorldResult` sends a deletion list, an order list
  and then the characters - which is exactly the three-list shape `FUN_14108d290` and
  `FUN_14108bdf0` read. Finding that shape twice is why the list layout is trusted.
* **`src/main/resources/ins.txt`**, a dump of the client-side inbound opcode *names*. Our
  `0x0014`/`0x0015`/`0x0016`/`0x0017`/`0x0018` are its `CheckDuplicatedIDResult`,
  `CreateNewCharacterResult`, `DeleteCharacterResult` and the two reserved-delete results,
  in that order, shifted by ten. Useful for naming, useless for numbering.

**Do not import its numbers.** The opcode offset is not constant across the range: `0x0081`
is `CHECK_DUPLICATE_ID` at -3, but the same -3 applied elsewhere names packets this client
demonstrably does not send.

## CharSelect is not a stage

`FUN_141b3f050(this, screenId, delay)` writes `this + 0x238` and nothing more. Screens
**3 (ClassicIntro), 4 (CharSelect) and 5 (NewChar) are sub-screens of a single login-stage
object**, so every character packet arrives through `FUN_141b25f30`, the login-stage switch.
There is no second `OnPacket` to find, and looking for one wasted a pass.

Two stage fields matter and were previously conflated:

| Field | Meaning |
|---|---|
| `stage+0xd0` | **the current screen id** - `5` is NewChar, and request builders test it |
| `stage+0xd4` | a request is in flight; set by every builder, cleared by its result handler |
| `stage+0x238` | a screen transition is running; builders refuse while it is non-zero |
| `stage+0x220` .. `+0x22f` | four ability points; must sum to `25` |
| `stage+0x230` | the character name, once the server has approved it |

The nine sibling stages, found by scanning `.rdata` for the base-class run every stage
vtable shares (`FUN_141d5f590`) and reading each vtable's `OnPacket` slot:

| OnPacket | Opcodes | What it is |
|---|---|---|
| `FUN_141b25f30` | the login set | **the login stage - all character traffic** |
| `FUN_141b82b00` | `0x51`-`0x6f` | buddy / messenger |
| `FUN_141072ec0` | `0x5ac`-`0x5bf` | - |
| `FUN_141df5940` | `0x1001`-`0x1007` | - |
| `FUN_142097ee0` | (x4) | a shared no-op base |

## The character manager

`FUN_14108…` is the character-manager family, and it is the tell for anything
character-shaped:

| Function | Role |
|---|---|
| `FUN_14108e7f0` | construct an empty character |
| `FUN_1403094b0` | **decode one character record from a packet** |
| `FUN_140302e30` | the stat block inside that record |
| `FUN_1402ee8d0` | the avatar look inside that record |
| `FUN_14108bdf0` | **decode the order list and the character list** |
| `FUN_14108d290` | decode the scheduled-deletion list |
| `FUN_14108d9b0` | look a character up by id |

## The character list is inside `0x0010`

Not a packet of its own. `FUN_141b307b0` reads a head, then calls two sub-readers:

```text
u8   result            0 = success
str  message
u8
8B   FILETIME
u32  worldId           looked up in the list built by 0x000B (FUN_141b2c7c0)
u32  channelId
4B, 4B, 4B
u32
u8
--- FUN_14108d290
u32  deletionCount     then count x (u32 characterId, 8B FILETIME)
--- FUN_14108bdf0
u32  orderCount        then count x u32 characterId
u8   characterCount    then count x character record
--- an undecoded tail: u8, u8, u32, u8, u8, u8, u32, ...
```

**That is why "Create a character" sent nothing.** The client reached CharSelect with an
empty list because our `0x0010` body was zero-padded, so all three counts read as `0`.

Built by `crates::net::opcode::login_result`, with a test that re-reads the bytes the way
the client does and asserts it lands exactly on the padding.

## One character record

`FUN_1403094b0` is the stat block, four fields of its own, then the avatar look. 327 bytes
as we build it. **The readers throw on underrun**, so a record short by one byte is an
exception inside the packet handler, not a rendering glitch.

```text
--- the stat block, FUN_140302e30(record, packet, 0)
u32  characterId
u32  characterIdForLog
u32  worldIdForLog
13B  name                 a fixed 13-byte field, NOT a length-prefixed string
u8   gender
u8   skin
u32                       zero in the reference encoder
u32  face
u32  hair
u32  level
u16  job                  -> +0x33, and the branch below proves it
u16  str, dex, int, luk
u32  hp, maxHp, mp, maxMp
u16  ap
     extended-SP jobs: u8 count, then count x (u8 jobLevel, u32 sp)   [FUN_1402cb0d0]
     everything else:  u16 sp
u64  exp
u32  fame
u32
u8   portal
u16  subJob
u8
8B   FILETIME
u32, u32
--- FUN_1403094b0's own fields
u32, u64, u32, u64
--- the avatar look, FUN_1402ee8d0
u8   gender
u8   skin
u32  face
u32  hair
u32
u8                        read and discarded
u32                       equipment slot 0
u8/u32 pairs              equipment, terminated by slot 0xFF
u8/u32 pairs              a second map, terminated by slot 0xFF
u32, u32, u32, u32
u32                       taken modulo 360
u8
u32
4B, 128B, u32, 13B
```

### The job branch is the proof that the head is right

After `ap`, the client re-reads the `u16` it stored at `+0x33` and bit-tests it against
three literal masks. Decoding them gives **exactly** the explorer job tree:

```text
100 110 111 112 120 121 122 130 131 132     mask 0x1c0701c01 over (job - 100)
200 210 211 212 220 221 222 230 231 232     the same mask over (job - 200)
300 310 311 312 320 321 322                 mask 0x701c01 over (job - 300)
400 410 411 412 420 421 422    plus 430-439
500 510 511 512 520 521 522
```

Job `0` also takes this branch. A field-order mistake anywhere above would put a value at
`+0x33` that does not spell a job tree, so this is a genuine check rather than a
restatement. `crates::net::opcode::uses_extended_sp` implements it, with a test naming the
jobs on both sides of the fork.

### The obfuscation is client-side only

Several `u32`s are stored as `(seed ^ value) >> 5 | (seed ^ value) << 27` with a per-field
seed. **The value on the wire is plain**; this is the client scrambling its own memory, not
a transform to apply. The same pattern appears in `0x0010`'s token handling, where it was
briefly mistaken for a character loop.

## The creation transaction

Three inbound opcodes, all in the login-stage switch:

| Inbound | Handler | Body |
|---|---|---|
| **`0x0014`** | `FUN_141b33f30` | `str name`, `u8 result` - the name check |
| **`0x0015`** | `FUN_141b36a10` | `u8 result`; if `0`: `u32 worldId`, one record, `u8` |
| **`0x0016`** | `FUN_141b34970` | `u32 characterId` - the delete result |

`FUN_141b33f30` is a `switch` on the result byte, so the codes are read, not guessed:

| Code | What the client does |
|---|---|
| `0` | `availableName` confirm dialog; on Yes, stores the name at `stage+0x230` |
| `0x45` | `antimacroTextMismatch` and builds the captcha UI |
| `0x46` | `cannotProcessRequest` |
| `0x47` | `antimacroTextFailTooMuch` |
| `0x79` | `cannotUseThisName` |
| `0x7A` | `alreadyUsedName` |
| `0x7B` | `cannotUseThisName` |

**`0x0015`'s success path** decodes the new character with `FUN_1403094b0`, registers it, and
calls `FUN_141b3f050(stage, 4, 0x14a)` - back to CharSelect with the new character in the
list. Its failure path raises a notice and returns to the same screen, so a non-zero result
is how the server refuses.

Two neighbours in the same family, not yet needed: `0x0017` (`FUN_141b359e0`) and `0x0018`
(`FUN_141b365f0`), both `u32 characterId` plus a per-character call.

### The NewChar screen

`FUN_141122420` is its button dispatcher, and it names every control:

```text
(unnamed, the OK button)   cancel
gender_prev / gender_next  face_prev / face_next
hairstyle_prev / _next     haircolor_prev / _next    skincolor_prev / _next
top_prev / _next           bottom_prev / _next
shoes_prev / _next         weapon_prev / _next
str_prev / _next           dex_prev / _next          int_prev / _next   luk_prev / _next
check_name                 edit_name (the text field)
```

The four AP pickers go through `FUN_141b3df70(stage, index, delta)`, which clamps each stat
to 4..12 and refuses any increment that would take the total over 25.

### The outbound side

| Outbound | Builder | Body | Pairs with |
|---|---|---|---|
| **`0x0081`** | `FUN_141b28950` | one string: the name | **`0x0014`** |
| `0x0082` | `FUN_141b3bfd0` | empty - leave world; clears the world list | - |
| `0x008B` | `FUN_141b28750`, `FUN_141b2cb70` | `u32 characterId` - select | - |
| `0x008C` | `FUN_141b2d860` | `u32 characterId` - delete, after `confirmDeleteCharacterPermanently` | `0x0016` |
| `0x008D` | `FUN_141b2da30` | `u32 characterId` - cancel a scheduled delete | - |

**`0x0081` ↔ `0x0014` is confirmed by more than adjacency.** `FUN_141b28950` sets
`stage+0xd4 = 1` after sending, and `FUN_141b33f30` opens by writing `stage+0xd4 = 0`. Same
flag, set by the request and cleared by the response. `FUN_141b28950` also refuses to send
unless the screen is `5`, no request is in flight, and the four AP total 25 - otherwise it
raises `useAllAP` locally. It validates the name itself too, raising `cannotUseThisName`
without sending.

## The create request is virtualised - stop looking for it statically

This is settled, and it cost a full pass to establish, so do not repeat the search.

```text
FUN_141122420   the NewChar OK button
  -> FUN_141b28950     sends 0x0081, returns 1
  -> FUN_141b3fb10     33 bytes: guard on stage+0x238 == 0, stage+0xd0 == 5, UI object set
       -> JNZ 0x141b2cf30          a tail jump, which is why Ghidra showed "bad instruction"
            FUN_141b2cf30          sets up a 0x558-byte frame, then:
              JMP 0x144c8b251      into .themida
```

Both functions have exactly one caller each, so this is the only path, and there is no
second one to find. `FUN_141b2cf30`'s body is Themida VM bytecode - everything Ghidra prints
past the `JMP` is garbage decoded from data.

**Consequences:**

* The create opcode and its field order **cannot be read**. They have to be measured.
* That is why no scan found it. `research/msexe-send-opcodes.txt` resolves 1881 of 1894
  `FUN_1406ed520` call sites and the create request is in neither group: its `FUN_1406ed520`
  call is inside the VM.
* Ruled out by reading them, and still ruled out: `0x0074`, `0x0075`, `0x0082`, `0x008B`,
  `0x008C`, `0x008D`, `0x00A8`, `0x00A9`, `0x00C0`, plus every other login-range builder.

**How to measure it.** The OK button sends `0x0081` **and** the create request back to back
in one click, without waiting for a reply - the two calls are consecutive statements in the
same `if`. So a single client run with the probe attached captures the create request in
full: reach CharSelect, open character creation, fill it in, click OK, and read the second
outbound packet out of `probe.log`.

`tools/transport.py` used to truncate any body over 24 bytes to its first 16, which would
have thrown away exactly this packet while the run still looked successful. It now spells
out bodies up to 256 bytes.

## What is *not* established

* **The create request.** Opcode and body, both. See above - it must come off the wire.
* **The tail of `0x0010` past the character list.** We pad it with zeros, which is what the
  accepted reply did. `LOGIN_RESULT_TAIL_PAD` is that padding.
* **Everything above is unmeasured.** Four static chains in this project looked this
  convincing and were wrong. Send it before believing it.
