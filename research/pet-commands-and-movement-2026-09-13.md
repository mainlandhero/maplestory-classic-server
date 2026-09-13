# Pet movement, pet commands, and a pet that advertises nothing - 2026-09-13

The owner, four things: *"broadcast player pet movement so other people can see pets moving even if it
is not their own"*; *"If those skills are not yet active, make sure that the pet do not have those
lines. Same with the dye line. The pets start with nothing learned, and the player has to purchase
those skills in the Cash Shop and choose the pet as a target"*; *"The pet commands are actually
regular chat messages ... if those messages match as one of the pet commands, then the pet should
respond accordingly"*; *"Please check the client WZ for the actions that the pet can do and the
lines it says."*

Tags: **[L]** read off the client, **[D]** derived, **[I]** inferred, **[R]** the reference server.

## 1. The WZ has all of it, in three places

`tools/dump_pets.py` joins them into `gm-handbook/petcommands.txt` - **2405 rows across 12 pets**:

| where | what |
|---|---|
| `Item/Pet/<id>.img/interact/<n>` | `command` (`c1`..), `prob` (percent), `l0`/`l1` (the pet-level band), `inc` (closeness), and `success.0` / `fail.0`, each an `act` and numbered line KEYS |
| `String/PetCommand.img/<id>/<cN>` | the words, pipe-separated: `bad\|no\|badgirl\|badboy` |
| `String/PetDialog.img/<id>/<key>` | the line, e.g. `c1_s1` -> "Bark bark!" |

The `act` names are nodes of the pet's own image - `rest0`, `nap`, `cry`, `dung`, `chat`, `prone`,
`rise`, `hand`, `angry`, `tedious`, `stand1` - so the **client** owns the animation. One word has
one entry per level band: the Husky's `sit` is entry 0 at levels 1..9 with `prob 40`, entry 1 at
10..19 with `prob 60`, and so on to `prob 85`. The Husky's vocabulary: `sit`, `bad|no|badgirl|badboy`,
`stupid|ihateyou|baddog|dummy`, `poop`, `talk|chat|say|bark`, `up|stand|rise`, `down`, `hand`,
`iloveyou`.

## 2. The commands ride on ordinary chat

Measured: typing `bad` sent `0x00E7 CLIENT_CHAT` and nothing else - no pet packet at all. So the
chat line is unchanged and the pet's answer is a **second** thing the server sends beside it:
`Session::pet_command_replies`, called from the plain-chat arm of `on_chat`.

Matching is the **whole message**, trimmed and case-insensitive. Not a prefix and not a substring:
`sit` is a command and `sit down over there` is a sentence.

The packet is **`0x0279`**. `FUN_141ec3fa0` reads `u8, u8, str` and calls
`FUN_141ec6680(pet, command1, command2, message, 0)`, whose first instruction on the arguments is
`test r8d, r8d` - so `command2` is a flag **[L]**. `command1` is the `interact` index and
`command2` is success/fail **[I]**: the client looks the animation up in its own image, which is
why no animation name is on the wire.

**Pets are level 1** and stay there - this server keeps no closeness - so the first band always
answers. `inc` is carried through the table so that adding closeness is one change.

## 3. Movement

`0x0202` is the pet's own move report: **504 of them after a summon and 0 before**, head
`u32 petIdx, u32 tick, u8` and then the movement path, whose first point is the exact spot the
server placed the pet **[L]**. `Session::on_pet_move` forwards the path **byte for byte** inside
`0x0278` (`FUN_141ec3f20` hands everything after the pet index to `FUN_141d598b0`, the path applier
`research/user-pool-tables.md` names for remote characters), published to the map and **not**
returned to the owner, whose client drew the walk itself.

## 4. A pet advertises nothing it has not learned

The tooltip prints one line per skill the pet IMAGE declares, and the `(Learned)` /
*"This is an unregistered pet."* half of it comes from ANDing that against the item body's
`petSkill` mask (`FUN_14266f2d0`). So a fresh pet that declares skills necessarily says
"unregistered" for each. Both halves are now empty:

* the installer writes `pickupItem 0, sweepForDrop 0, longRange 0` onto every pet - the two
  vacuum keys an earlier build added, and Nexon's own `pickupItem`;
* `net::bag::PET_SKILLS_LEARNED_AT_START` is `0`.

The dye line is already gone by `PET_HUE_UNDYED` (`-1`), from the previous round.

**Not built:** the Cash Shop pet-skill items that would set the mask (and re-declare the skill),
feeding, naming, dyeing, closeness and levelling. Until they exist a pet has no skills at all,
which is the state the owner asked for and is why the vacuum is inert for now.

## 5. Unverified on screen

Every part of section 2 and 3. What a run says: the pet walking on a second client; a command word
making it act and speak; a sentence not doing so; and the tooltip carrying **no** skill lines and
no dye line.
