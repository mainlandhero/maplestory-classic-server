# The pet tooltip's two wrong lines, and where pet chat commands go - 2026-09-13

The owner, with the Husky's tooltip open: *"The pet does not pick up items despite it saying that the
skill is applied but unregistered"*, *"My pet is not dyed, the pet should not have that line"*, and
*"The pets' chat commands does not work."*

Tags: **[L]** read off the client, **[D]** derived, **[I]** inferred, **[R]** the reference server.

## 0. First: the pet IS drawn now

`world.log` for that session settles the earlier question. **Opcode `0x0202` arrives 504 times
after the summon and exactly 0 times before it** (`23:21:07.569` is the `0x0277`; the first
`0x0202` is `23:21:08.070`). Its body is a movement block whose first point is `x 0x0136`,
`y 0x0112` - 310, 274 - the position we put the pet at. So `0x0202` is the **pet's move report**,
the pet is alive, walking and on screen, and the visibility gauntlet of
`research/pet-not-drawn-2026-09-13.md` is passing. That research note's gate hunt is closed; what
remains from it is that `0x0202` is unanswered, so **other players are never told the pet moved**.

## 1. "This is an unregistered pet." - `petSkill` is a registration mask, and we sent 0

The tooltip builder is `FUN_14266f2d0` (found by scanning `.text` for the string ids as `mov r32,
imm32`, the technique `research/client-messages.md` names). Its per-skill loop **[L]**:

```c
local_250 = FUN_1426dc0d0(skillNode);      // a u16 taken from the ITEM
local_250 = local_250 & skillBit;          // ANDed with this skill's bit
...
if (local_250 == 0)  FUN_1408a9e40(&s, 0x9E5);   // "(Learned)"
else                 FUN_1408a9e40(&s, 0x9E6);   // "This is an unregistered pet."
```

So the WZ keys the installer writes (`pickupItem`, `sweepForDrop`, `longRange`) decide which
skills are **listed**; a separate per-pet `u16` decides whether each is **usable**. That `u16` is
the pet item body's `petSkill` (`FUN_140304550` read at `14030462e`), and this server sent `0` -
so every skill read "unregistered", and an unregistered Item Pouch does not pick anything up.

`net::bag::PET_SKILLS_GRANTED` now carries `ITEM_PICKUP | EXPANDED_AUTO_MOVE | AUTO_MOVE`. That
the mask lives in this field is **[D]**; the bit *numbering* is the reference's `PetSkill` enum
and is **[R]/[I]**. The tooltip is its own test: the three lines become `(Learned)` if the
numbering is right, and stay "unregistered" if it is not.

## 2. "Your pet has been dyed!" - `petHue` 0 means dyed, not undyed

Same builder, line 2092 **[L]**:

```c
if (-1 < FUN_1401ba9d0(item + 0xa6, *(u32 *)(item + 0xae))) {
    ... string 0x102A, "Your pet has been dyed!" ...
}
```

`FUN_1401ba9d0` is the client's ordinary **obfuscated-int reader** - `mov r8d,[rcx]`,
`mov ebx,[rcx+4]`, `rol ebx,5`, `xor ebx,r8d`, `xor r8d,0xBAADF00D`, `ror r8d,5` - so what the
test compares is the stored value itself. **A hue of 0 is "dyed with colour 0"; only a negative
hue is undyed**, which is exactly what the reference annotates on its own field (`Pet.encode`:
`encodeInt(getHue()); // -1`). `net::bag::PET_HUE_UNDYED` is `0xFFFFFFFF`, in the item body and
in the `0x0277` activation.

## 3. Pet chat commands ride on ORDINARY CHAT, and the server has to answer them

Measured, not guessed. Typing `bad` produced **`0x00E7 CLIENT_CHAT`**, body
`046e4115 0300 626164 03` = tick, `u16` length 3, `"bad"`, tab 3 - and this server answered it the
way it answers any chat, with a `0x0231` balloon reading "bad". No pet packet was sent by the
client at all: `0x0148` and `0x0149`, the two undecoded pet sends, appear **nowhere** in the log.

So the command list in the tooltip (`Lv. 1+: bad, baddog, ... sit, stupid`; `Lv. 10+: bark, chat,
down, say, talk`; `Lv. 20+: rise, stand, up`) is acted on **server-side**: the server recognises
the word while a pet is out and broadcasts the pet's action instead of a chat line.

The packet to send is in the same family as the activation - the `0x277..0x27E` sub-dispatcher
`FUN_142795b20`, whose rows this server has already enumerated:

```text
0x0279  FUN_141ec3fa0   u8, str      after the router's charId and the dispatcher's petIdx
0x027a  FUN_141ec4050   str
```

which lines up with the reference's `PetPacket.action(id, petID, command1, command2, sMsg)` and
`actionSpeak(id, petID, sMsg)` **[R]**. **Not built.** What is still needed: the command-word to
action-number table (the tooltip's own list is the vocabulary; where the client keeps it is not
yet found), and one capture to confirm whether `0x0279` takes one command byte or two.

## 4. What changed

`net::bag::pet_item_with_state` now writes `PET_SKILLS_GRANTED` and `PET_HUE_UNDYED`; the
`0x0277` activation carries the same hue. Nothing else. Both are **unverified on screen** - the
tooltip is the check, and plan step TO(v) asks for it.
