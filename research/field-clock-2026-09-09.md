# The station clock, 2026-09-09 — diagnosed, mis-blocked, then found on 2026-09-10

> **Read the third pass at the bottom first.** The first two passes are kept as written
> because the second one is a worked example of believing a stale sentence; the answer is
> `0x01BC`, type 1, `u8 hour, u8 minute, u8 second`, and it is wired.

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


---

# Third pass, 2026-09-10: FOUND. The "blocker" was a stale sentence, and the review caught it

The owner asked for a review of the second pass. Its conclusion - *the clock is behind
`CStage::OnPacket`, which has not been found* - rested on one sentence in
`research/msexe-gamestage-opcodes.md`. That sentence was written on 2026-08-19 and overtaken
**the same day**: `research/msexe-gamestage-dispatch.md` has had `FUN_141820080` =
`CField::OnPacket`, `0x01a4..0x05ab`, in its range table since then, and so has the
project memory. The second pass never looked. Its "zero calls from the inbound-handler space
into the clock code" was scanned over `0x142c00000..0x142f00000`, and the field dispatcher
lives at `0x141820080` - **outside the scanned range**. Enumerate before you filter, again.

## What is measured

**The dense switch.** `FUN_141820080` does `lea eax,[rdx-0x1a4]; cmp eax,0x7f; ja default`
and jumps through a 128-entry table of image-relative RVAs at `0x141822158`. All 128 cases,
each with the handler its stub calls, are in `research/msexe-field-cases.txt`. [L]

**`0x01BC` is the clock.** Its case is `add rcx,-0x18; mov rax,[rcx]; call [rax+0x1d8]` -
vtable slot 59 - and slot 59 in every field vtable is `FUN_1418564d0`. That function reads a
`u8` type, refuses anything above `0x11`, and jumps through an 18-entry table at
`0x141856f9c`. The arms read: [L]

```text
type 0  0x14185652d  u32 seconds      -> FUN_142d98870(global, |seconds|); <= 0 destroys it
type 1  0x141856566  u8 h, u8 m, u8 s -> FUN_1418a7dd0(field+0x220) then FUN_1415ea5c0(widget, h, m, s)
type 2  0x1418565af  u32 seconds      -> builder FUN_141839a60
type 3  0x141856f81  (no arm)
type 4  0x14185662a  u32, u32         -> a gauge, UI resource 0x310
5..17   read; 8, 11, 12, 15 have no arm
```

That is the v214 `ClockType` order - EventTimer 0, HMSClock 1, SecondsClock 2, TimerGauge 4 -
with matching shapes. The reference scores 1 of 8 here, so the shapes carry it.

**The unit is a 24-hour hour.** `FUN_1415ea5c0` multiplies by `0x2aaaaaab` (divide by 12),
stores the quotient's non-zero-ness at `widget+0x288` (PM), `hour % 12` at `+0x28c` with 0
mapped to 12, minute at `+0x290`, second at `+0x294`, and `GetTickCount` at `+0x2a8` so the
widget ticks on its own afterwards. That is the AM/PM display in the screenshot. [L]

**Why it must be gated on the map.** `FUN_1418a7dd0` returns `[holder+8]` and, when that is
null, calls `FUN_142e52ed0(0x431, 0)` - the throw helper this client uses everywhere - and
the type-1 arm calls the setter on the result with no check. A map whose image has no
`clock` node never builds the widget (`FUN_141841430` only runs the construction when
`FUN_14023b360(img, "clock")` returns a node). So the server sends `0x01BC` only on maps
listed in `gm-handbook/clocks.txt`, which `tools/dump_portals.py` now emits from the same
node. [L]

**The widget's home.** The map loader stores it through `FUN_1418a74f0` into the holder at
`field+0x220`; `[field+0x228]` is the raw pointer. Case `0x01C4` calls `FUN_1418396d0`,
which reads no packet and releases exactly that pointer - **DestroyClock**. Case `0x01C5`
takes a `u8` and walks the id-keyed tree at `field+0x230` - destroy a timer by id. Cases
`0x020C` / `0x020D` are vtable slots 83 / 84: `u8 flag` -> `field+0x1c88` / `+0x1c89`, then a
widget setter; the loader reads those two bytes back when it builds the widget, which is how
the two were found. [L]

## What the review did NOT change

`0x007B` InventoryGrow stands: the bytes re-read from the exe this pass are the same two
one-byte reads into `ebx` and `edx`, `add rsi,0x5d0; inc edx; lea rcx,[rsi+rbx*8]`, and
`research/bag-lists.md` line 109 has the same base and stride from the character record.

## Wired

`net::clock::clock_hms` builds `01 hh mm ss`; `session::field::on_field_entered` sends it
after the NPCs when `Config::clocks` lists the map; `world::localtime` supplies **local**
time via `GetLocalTime`, because `server::log` is UTC and a clock on a wall is read against a
wristwatch. Test plan step TK says what each screen outcome means. Not yet seen on screen.
