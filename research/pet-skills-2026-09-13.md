# The pet skill table, read out of the client - 2026-09-13

The owner: *"I just tried using an Auto HP skill on the Husky pet. Also the Husky should by default
come with Meso Magnet and Item Pouch. Currently it is missing the Item Pouch skill by default."*

## 1. The eleven skills, in the client's own order

`FUN_141ed1ad0` turns a skill index into its name string: `cmp edx, 0xa` then a jump table, one
arm per skill, each `mov edx, <string id>`. **[L]**

| index | string | name | the pet-image key that declares it | sold as |
|---|---|---|---|---|
| 0 | `0x9D1` | Item Pouch | `info/pickupItem` | - (default) |
| 1 | `0x9D2` | Auto HP Potion Pouch | `info/consumeHP` | `5190000` |
| 2 | `0x9D4` | Expanded Auto Move | `info/longRange` | `5190003` |
| 3 | `0x9D5` | Auto Move | `info/sweepForDrop` | `5190002` (its own key says `dropSweep`) |
| 4 | `0x9D3` | Auto MP Potion Pouch | `info/consumeMP` | `5190001` |
| 5 | `0x9D7` | Ignore Item | | |
| 6 | `0x9D8` | Auto Buff | | |
| 7 | `0x9D9` | Auto Feed and Movement Skill | | |
| 8 | `0x9DA` | Fatten Up | | |
| 9 | `0x9DB` | Pet Shop Skill | | |

**This is not the reference server's order** - its `PetSkill` enum puts `EXPANDED_AUTO_MOVE`
second and `AUTO_HP` at `0x20`. The bit is `1 << index` (**[I]**; the order is **[L]** and the
tooltip is the test), so `net::bag`'s constants are now the client's.

**Meso Magnet (`0x9DC`) is in no arm of that table.** It is innate: with every skill key cleared
it still showed and Item Pouch did not, which is exactly the owner's observation. **[D]**

## 2. What each skill item carries

`Item/Cash/0519.img`, the four the Cash Shop sells at 100 LP under the Pets tab
(SN `160300002..5`, category 6 / sub 3):

```text
5190000  Auto HP Potion Skill        cash 1, consumeHP 1, add 1
5190001  Auto MP Potion Skill        cash 1, consumeMP 1, add 1
5190002  Auto Move Skill             cash 1, dropSweep 1, add 1
5190003  Expanded Auto Move Skill    cash 1, longRange 1, add 1
```

`add 1` is the instruction: using one **adds** its key's skill to the pet it targets. So learning
is two halves - the pet's image must declare the skill and the item body's mask must have its bit -
and a purchase has to set both. The declaration is WZ, which cannot change at runtime, so a
learnable skill has to be declared on every pet up front and gated by the mask alone. **That is
the design decision the feature needs and it is not built.**

## 3. Using one sent nothing

`world.log` for the session the owner tried it in has **no inbound packet** that could be the use: the
only unanswered opcodes are `0x0202` (the pet's moves, now forwarded), `0x02EB`, `0x0408`, `0x0205`
and `0x00ED`, all at login or field entry, none after. So the client refused locally or opened a UI
that went nowhere. What it would send is still unknown; `0x0148` (`u32, u32, u32, u8`, two builders)
remains the best candidate and has never been seen on the wire.

## 4. What changed

* `net::bag`'s skill bits are the client's numbering, not the reference's.
* `PET_SKILLS_LEARNED_AT_START` is **Item Pouch**, so it reads `(Learned)` rather than
  "unregistered".
* The installer puts `info/pickupItem 1` back on every pet - Nexon's own value, which the previous
  round had cleared - and leaves `sweepForDrop` and `longRange` at `0`, because those two are the
  Auto Move skills the shop sells.

So a fresh pet has **Meso Magnet and Item Pouch**, both usable, and advertises nothing else.

**The install is pending**: the client was running when this was written, so
`python tools\backport_install.py --install` has to be re-run with it closed.
