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
