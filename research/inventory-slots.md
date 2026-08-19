# The bag: how the client is told how many inventory slots it has

**Built 2026-08-19. Unconfirmed on screen.** Everything below is read off the client's own
listing unless it says otherwise, and the one thing that matters most - whether this is why
unequip does nothing - is explicitly *not* established.

The owner: *"make sure that the game provides a default amount of inventory slots to new
characters. The client already assumes a number, but the game provides a functionality to
increase the max slots."*

## The headline

`presence[7]` switches on **six `u16` inventory sizes** in the character record, sitting
between the three optional-string flags and the equipped list. Twelve bytes. Each value is
a **slot count**.

```
  0  100  presence array                  byte 0, byte 2 and now byte 7
100   11  head counts and flags           0
111  108  the character-stat block
219    4  a u8 and three string flags     0
223   12  SIX u16 INVENTORY SIZES         <- this document
235   ..  the equipped list
  ..   1  one ungated u8
```

## How it was found

`FUN_140304b20` runs a loop of exactly **six** turns over six pointers based at
`char+0x5d8`, stepping `8` a turn:

| | |
|---|---|
| `0x140305de8` | `ADD RSI,0x5d8` - the base |
| `0x140305def` | `MOV R15D,0x6` - the trip count, a literal |
| `0x14030608a` | `ADD RSI,0x8` - one pointer a turn |
| `0x14030608e` | `SUB R15,0x1 / JNZ 0x140305e01` |

Inside it, gate entry 13 - key `0x143abf0c0`, whose CRT initialiser sets byte **7** - guards
exactly one packet read: the `u16` at `0x140305e48`. A sweep of the whole loop body,
`0x140305de8` to `0x140306098`, finds **no other call to any of the eight read primitives**.
So this presence byte costs six `u16` and nothing else. **[L]**

That sweep is the instrument, and it was checked the way `CLAUDE.md` asks: it was run over
the eight primitives from `docs/ghidra.md`, the table that was itself corrected on
2026-08-19 when a five-name grep missed two of them. Its positive controls are in the same
function - the `u8` at `0x1403061cc` and the `u16` at `0x1403061fc`, both of which the sweep
does find, both of which are bytes we already send.

## What the value means

```asm
140305e0f  MOV EAX,dword ptr [RAX + -0x8]   ; count, from the array header
140305e12  DEC EAX                          ; default = count - 1
;   --- gate entry 13, presence[7] ---
140305e48  CALL 0x1406e8b80                 ; V = read_u16()
140305e50  MOV dword ptr [RSP + 0x60],EDX   ; V replaces the default
;   ---
140305e65  CMP EDX,EAX
140305e67  JZ  0x140305e7a                  ; already that size - nothing to do
140305e69  INC EDX
140305e75  CALL 0x14030ee00                 ; resize the array to V + 1
```

The array becomes `V + 1` elements and the client then walks slot indices `0..=V`
(`CMP EBX,[RSP+0x60] / JLE` at `0x140306078`). So **`V` is the slot count**, and index 0 is
the unused hole that makes MapleStory's slots 1-based. **[L]**

The walk itself reads nothing from the packet: it loads `[slot + 8]`, skips on null, and
otherwise runs a virtual. So a larger `V` costs `V` null checks and an allocation, and
nothing else. That is why `MAX_INVENTORY_SLOTS` (100) is **our** cap and not the client's -
nothing in the decoder bounds `V` at all.

## The order of the six

From the **jump table at `0x1403093bc`**, which picks each turn's gate key from the loop
counter. Read out of `client-patched/MapleStory.exe` directly, by resolving the VA to a file
offset through the PE section table:

| turn | jump target | key | gate entry | presence byte | what it is |
|---:|---|---|---:|---:|---|
| 0 | `0x140305e90` | `0x143abedb0` | 6 | **2** | the equip inventory **[D]** |
| 1 | `0x140305e99` | `0x143abed40` | 5 | **3** | use **[I]** |
| 2 | `0x140305ea2` | `0x143abecd0` | 4 | **4** | set-up **[I]** |
| 3 | `0x140305eab` | `0x143abec60` | 3 | **5** | etc **[I]** |
| 4 | `0x140305eb4` | `0x143abebf0` | 2 | **6** | cash **[I]** |
| 5 | `0x140305ebd` | `0x143abeb80` | 1 | **44** | **Deco** **[D]** |

The key VAs check out against the table base independently: entries are `0x143abeb10 +
0x70 * k`, and `research/charrecord-presence-map.md` lists entry 6 at `0x143abedb0`, entry 5
at `0x143abed40` and entry 1 at `0x143abeb80` from a different measurement. **[L]**

**Byte 2 is measured**: it is the byte that switches on the equipped-item list
(`research/naked-character.md`), which is what makes turn 0 the equip inventory.

**The rest of the names came off the screen, not the binary.** The owner's inventory window has
exactly six tabs - `Equip`, `Use`, `Set Up`, `Etc`, `Cash`, `Deco` - against a loop whose
trip count is the literal `6`, with the first five in the order the reference server's
`DBChar` ordinals 2..6 predict. **[D]**

That is worth noticing as a method point. This table said "not identified at all" for turn
5, and **no amount of further reading would have fixed it**: the name is not in the decoder,
it is in the UI. One screenshot settled what a day of listing work could not.

**Nothing on the wire depends on those names.** The server sends every inventory the same
size, so a wrong name costs a comment and not a byte - which is why the table was safe to
ship with a hole in it rather than a guess in it.

## Why this might be the unequip bug - and why that is a candidate, not a finding

With `presence[7]` clear, the size used is `count - 1` from `0x140305e0f`. The two
instructions before it are:

```asm
140305e01  MOV RAX,qword ptr [RSI]
140305e06  TEST RAX,RAX
140305e09  JNZ 0x140305e0f
140305e0b  MOV EAX,EDI          ; EDI is 0 - a null array counts as zero
```

so a **null** array gives `count = 0` and therefore `V = -1`. The very next thing done with
it is `CMP dword ptr [RSP+0x60],0x0 / JL 0x140306087` at `0x140305f09`, which skips that
inventory's entire slot walk. A negative bag has no slots at all. **[L]** for the
arithmetic.

**What is NOT established is whether these arrays are null at decode time.** That lives in
the constructor of the character-data object, which has not been read. So:

> The claim is: *if* the arrays start null or empty, a server that never sends `presence[7]`
> leaves every inventory at "no slots", and a client with no slots has nowhere to put an
> unequipped item - which is what the owner saw on 2026-08-19, where dragging the Undershirt off
> produced nothing on the wire at all. That is a **[I]** with a mechanism, not a measurement.

The cheap way to settle it is the run, not more reading - and the screenshot below
makes the right probe a number BELOW the default rather than above it.

## What the server does

* `Character::inventory_slots: [u16; 6]`, defaulting to **30**.
* Persisted per character, one column each (`characters.slots_equip` and friends), added
  by a guarded `ALTER TABLE` because `CREATE TABLE IF NOT EXISTS` does nothing to a table
  that already exists. A character created before the columns existed reads back the
  default through the column default, and there is a test that inserts such a row and
  proves it. Rows written during the one commit that defaulted to 24 are repaired to 30
  on open - see the note beside that UPDATE for why it is safe now and has to go later.
* Per-column rather than one number, because buying slots is per-tab in this game and a
  single column could not express a character who has bought Use slots and not Etc ones.
  Nothing yet *sells* slots; the storage is what makes that a later feature rather than a
  schema change.
* `--inventory-slots N` overrides all six for one run. A test lever, and the reason it
  exists is in the next section.

## 30, and the ambiguity the screenshot did not resolve

The default was 24 for one commit. **24 is the classic MapleStory bag**, and it came from
the reference server rather than from anything measured here - the same failure this
document already records twice. The owner's window is **five wide and six tall**, thirty cells
filled with nothing below the fold, so 30 is the number.

> **What the screenshot does NOT settle: whether those thirty cells are the array or the
> window.** The panel has a scrollbar. A viewport drawn 5x6 with a scrollbar behind it would
> look identical at any slot count of 30 or more, so "thirty cells" is consistent both with
> "the bag holds 30" and with "the bag holds something else and 30 is what fits". **[I]**
>
> It matters for one reason: if the arrays already held 30 before we ever sent
> `presence[7]`, then the null-array reasoning above is **not** why unequip fails, and that
> hypothesis needs dropping rather than defending.

## How to test it, and why a number ABOVE 30 is the wrong probe

A run at 32 barely moves anything visible: the grid still draws 5x6 and only the scrollbar's
range changes, which is exactly the observation a viewport would fake. **Go under instead.**

**`-InventorySlots 10`.** Then the two readings come apart:

| what you see | what it means |
|---|---|
| ~10 usable cells and the rest dead or gone | the field is read, and the grid is a viewport |
| still thirty usable cells | the field is **not** reaching the array - `presence[7]` is wrong |
| the character is undressed, or no world entry | the twelve bytes are in the wrong place |

Then a second run at the default 30 puts the bag back. The other things worth reading in the
same run:

| what to look at | what it means |
|---|---|
| the other five tabs | all six are sent equal; if **one** differs, the order above is wrong |
| dragging an equip off | a `0x0107` in `world.log` means the slot count was the whole problem |

That last row is the important one and it is why a failure here is loud rather than subtle:
the record has **no length prefix and no resync point**, so twelve bytes in the wrong place
makes the equipped list behind them garbage. The failure mode is an undressed character or
no world entry, not a wrong number.

The watch `140305e48:peek=24` prints the packet read cursor at each of the six reads.
**Expect six lines, each exactly 2 apart.** That criterion is origin-independent - it does
not matter what the cursor counts from, only that the client took twelve contiguous bytes
where we put twelve contiguous bytes.

## Checked and not assumed

* **The equipped list did not move relative to its own gate.** The gate at `0x1403061a0` is
  after loop #6 at `0x140305de8`, which is why the twelve bytes go before it. Confirmed by
  `tools/channel_smoke.py`, which parses the equipped block at both candidate stat-block
  lengths and requires exactly one to be well formed - it would fail if the block had moved.
* **`presence[7]` appears in exactly one row** of the 40-row gate table, so setting it opens
  nothing else.
* **The dynamic per-turn gate does not add reads.** With `presence[2]` set, turn 0 also
  enters the block at `0x140305f05` and runs its slot walk. That walk calls no read
  primitive, so a larger bag costs iterations and not bytes.
