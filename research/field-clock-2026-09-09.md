# The station clock, 2026-09-09 — diagnosed, not fixed

The owner, with a screenshot of Ellinia Station: *"In the station maps such as Ellinia Station, the
server clock does not seem to work. It just stays on 00:00."*

## What is measured

**The widget is the map's, and the map declares it.** `010002090.img` has a top-level `clock`
node beside `back`, `foothold` and `portal`:

```json
"clock": { "x": 635, "y": -226, "width": 200, "height": 200 }
```

That is a placement, not a time. The client draws the AM/PM display at those coordinates and
waits to be told what to show. **[L]**

**This server sends nothing for it.** `grep -rn "0x004B\|CLOCK\|field_clock" crates/net/src`
comes back empty; there is no clock builder anywhere in the workspace, and no field-entry path
sends one. So `00:00` is not a broken widget or a wrong number — it is a widget that has never
been given a value. **[L]**

That much settles the *shape* of the problem: it is a missing packet, not a bad one.

## What is NOT established, and why the obvious search failed

**Which opcode carries it.** `tools/xref.py --string clock` reports the literal in five places
(three ASCII, two UTF-16) and **0 code references**.

That negative is worth nothing on its own and is recorded as such. `xref.py` matches
`lea reg, [rip+disp32]` and `mov reg, imm64`; a WZ property fetched through a hashed name
table, an interned string, or a wide literal built on the stack is invisible to it. This is
`CLAUDE.md`'s standing rule — a silent negative is usually a property of the search — and the
tool has no positive control here, so **"no code references `clock`" must not be read as "the
client does not read it"**. It plainly does: it places the widget.

`tools/rtti.py --list Clock` is also empty, which is a second instrument agreeing and a second
one with the same blind spot: RTTI only names classes with virtual functions reached by
`dynamic_cast`, so a plain UI struct never appears.

## The next step, and why no capture from the owner can shortcut it

**This is a server→client packet.** No amount of playing produces one, so unlike the Cash-item
opcode — which one click named, because the *client* sends it — a run cannot settle this. It
needs a decode.

The cheapest route that has not been tried: the client must, on field entry, notice `clock` in
the map data and construct the widget. Find **that** construction site rather than the string,
then walk to whatever writes its hour/minute fields; the inbound handler that sets them is the
opcode. Starting points that have worked on this binary before:

* the field-load path already decoded in `research/msexe-fieldload.c` — the same walk that
  reads `portal`, `foothold` and `life` must read `clock`, and it is a sibling of nodes this
  project has already located;
* `research/msexe-gamestage-opcodes.md`'s dispatch table, for an unhandled arm whose handler
  writes two small integers into a UI object.

## What the packet almost certainly looks like, marked as the guess it is

Every MapleStory server sends a `CLOCK` with a leading type byte — `1` for a countdown
(`i32 seconds`) and `2` for a wall clock (`u8 hour, u8 minute, u8 second`). Ellinia Station's
display has an **AM/PM** indicator and four digits, which is the wall-clock shape.

**[I], from the game family and from the screenshot, not from this binary.** It is written down
so the decode has something to disconfirm, and it is explicitly *not* a licence to pick an
opcode and send it: this project's rule is that guessing an opcode moves the client into a
state nobody has read, and the Cash-item opcode was left unwired for exactly one turn rather
than guessed — which turned out to be the right call when the capture named `0x0114` and it
was not any of the obvious candidates.


---

# Second pass, same day: the search was wrong, and the blocker has a name

The first pass above concluded "not established" off two instruments that both returned
nothing. **Both of those negatives were worthless, and one of them was worthless for a reason
the tool documents about itself.**

## The clock IS read, and the reference was invisible to `xref.py`

`xref.py` matches `lea reg,[rip+disp32]` and `mov reg,imm64`. Its own docstring names the
blind spot: *"`mov rax, [rip+disp32]` - a data READ, not an address"* - and that is exactly
how a global interned-string pointer is loaded. So "0 code references" said nothing.

An operand-level scan - every instruction whose rip-relative target resolves into a wanted set,
whatever the mnemonic - with a **positive control** (`0x143414f08` "ladderRope%d", known to be
referenced at `0x141e7dd32`) found the control **and exactly one clock reference**:

```text
0x143a48320  the interned 'clock' pointer   1 reference
      00014184145a  mov  rdi, qword ptr [rip + 0x2206ebf]
```

`0x14184145a` is inside **`FUN_141841430`** (2252 bytes, `0x141841430..0x141841cfc`), which
reads the map's `clock` node. It has exactly one caller, `FUN_1418224c0`. So the widget is
real, it is constructed from the map data, and the earlier "0 references" was a property of
the search.

The other tool agreed and was also worthless: `rtti.py --list Clock` is empty, but RTTI only
names classes with virtual functions reached by `dynamic_cast`, so a plain UI object never
appears. **Two instruments agreeing is not corroboration when they share a blind spot** -
`CLAUDE.md` has a section on precisely this, and it happened again here.

## No channel handler touches it, and that is not a surprise once you look at the range

Every `call`/`jmp` from the whole inbound-handler address space (`0x142c00000..0x142f00000`)
into the clock class's neighbourhood (`0x141840000..0x141846000`) was enumerated: **zero**.

The reason is the dispatcher. `FUN_142cbaa80` - the channel switch, 273 cases - covers
`0x0070..0x019f` plus two outliers, and **stops before `0x01A0`**.
`research/msexe-gamestage-opcodes.md` already records what that means: `0x01A0` SetField and
the whole field-packet block are dispatched by **`CStage::OnPacket`**, and *"that function has
not been found yet; it is the next thing to look for, and `0x01A0` is the case label to look
for inside it."*

So the clock packet is behind the same unfound dispatcher as SetField. **No search inside the
known switch could ever have found it**, which is also why none of the 294 v214 candidate
names in that table is a Clock - the table only covers the switch that stops at `0x019f`.

## What would settle it

Find `CStage::OnPacket`. That is an existing open item in this repo, not a new one, and it
unblocks more than the clock: every field-block packet from `0x01A0` up is behind it.

`tools/find_switch_tables.py --min 40` was tried and is not the way in - it returns 263 runs
dominated by false positives (a 99990-"case" run in a 366-byte function). A jump table for a
sparse opcode block is likely a byte index table plus a smaller RVA run, which that tool's
shape assumption does not cover.

## What this pass DID land

Reading the candidate-name table for the clock turned up `0x007B` = **InventoryGrow**, and the
handler confirms it independently of the name: `FUN_142d54700` reads `u8 invType, u8 slots` and
resizes `charData + 0x5d0 + invType*8` to `slots + 1`. That base and indexing are the **same**
this project derived from the character record for the Equip tab, from a different packet.

It is now `net::inventory::inventory_grow`, and the 5-slot coupons widen the tab on screen
instead of saying "change maps or relog to see them".
