# `0x00AC` — BroadcastMsg, and the scrolling banner is type 4

**2026-08-20.** The owner asked for a scrolling banner across the top of the screen to announce
EXP/meso rate events. This is how the opcode was found and what is and is not established.

Everything below is marked **[L]** (read out of mscw) or **[I]** (inferred, and from what).

## Summary

| | |
|---|---|
| opcode | **`0x00AC`** **[L]** |
| handler | `FUN_142d60d40`, 8292 bytes **[L]** |
| body | `u8 type`, then a per-type tail — see below **[L]** |
| the banner | **type 4** **[L]** |
| type 4's tail | `u8 flag`, then `str message` only when `flag != 0` **[L]** |
| taking it down | type 4 with `flag = 0` and **no string** **[L]** |
| that it *scrolls*, at the *top* | **[I]** — see "What is not established" |

Built by `crates/net/src/broadcast.rs`.

## The candidate table was wrong twice, and how that was caught

`research/msexe-gamestage-opcodes.md` aligns mscw's 273 channel opcodes against two other
versions' enums, anchored on the one confirmed name. It offers **two** entries called
`BroadcastMsg`, one per alignment. Both are wrong, and decompiling them says so:

| candidate | handler | what it actually reads |
|---|---|---|
| `0x00B5` (older alignment) | `FUN_142d92e30`, 33 bytes | one `u32`, then two calls. Not this. |
| `0x00AB` (newer alignment) | `FUN_142d60c70`, 192 bytes | `u32, u32`, and **only if neither is `999999999`**, a `u32` and two `u16` |

That `999999999` is the town-portal "no portal placed" sentinel, and the shape - two map
ids, then a skill and a point - is a town portal. So **mscw's `0x00AB` is TownPortal**, which
the newer alignment places one slot earlier. The alignment is off by one in this
neighbourhood, exactly the decay that file warns about.

TownPortal and BroadcastMsg are adjacent in the enum. One slot on from `0x00AB` is `0x00AC`,
whose handler `FUN_142d60d40` sits **immediately after** `FUN_142d60c70` in the image - the
handlers are adjacent the way the enum entries are. It is 8292 bytes, which is the right
order of magnitude for a packet with thirty-one types and none of the others are.

**This is a case where the reference tree earned its keep by being wrong in a legible way.**
It did not name the opcode; what it did was put two decoys next to the answer, and decoding
the decoys is what located it. The rule in `CLAUDE.md` stands - the tree scored 1 of 8 on a
held-out control, so nothing here rests on it.

## The body **[L]**

```text
u8  type              always                                            142d60d72
u8  flag              types 4 and 26 ONLY                               142d60e48
str message           unless type is 13, 14, 27 or 29; and for types
                      4 and 26, only when flag != 0                     142d60e64
str                   a second string, types 28 and 30 only             142d60ebd
```

The type is bounded at `0x1e`; above that it falls to the same do-nothing target as the
unhandled cases.

**The "no string" set is a bit mask, not a range.** The test is

```c
if ((type < 0x1e) && ((0x28006000 >> (type & 0x1f) & 1) != 0)) { /* no string */ }
```

and `0x28006000` has bits **13, 14, 27, 29** set. Reading that as a comparison chain or a
contiguous range would have been wrong in the quiet way this project keeps paying for - it is
the same shape as the mob-mask mistake in `CLAUDE.md`'s "enumerate before you filter".

### The instruments agree

`tools/reads.py 0x142d60d40 1` lists four reads at `142d60d72`, `142d60e48`, `142d60e64` and
`142d60ebd`, in that order and with those widths - the same four the decompiler shows, at the
same addresses. Field order therefore rests on two instruments, which is the standing rule in
`docs/ghidra.md`.

## Why type 4 is the banner **[L]**

Most types of this packet end up in the chat printer `FUN_1415eca30(text, kind)` that
`research/talking-back.md` decoded for `0x00BB`: type 0 posts with kind 9, type 2 with kind 7,
type 5 with kind 0xb.

**Type 4 touches none of that.** It works on a single global object, `DAT_143acd9a0`, with a
create-and-destroy lifecycle:

```c
case 4:
  if (DAT_143acd9a0) { FUN_142bf3f70(DAT_143acd9a0); FUN_1422a00c0(DAT_143acd9a0); }
  if (flag == 0 || message == NULL || *message == '\0') {
      if (DAT_143acd9a0) { FUN_142bf3f70(DAT_143acd9a0); /* tear down */ }
  } else {
      if (DAT_143acd9a0 == 0) FUN_142db92d0();          // create the singleton
      FUN_14229c3d0(DAT_143acd9a0, message, 0);         // hand it the text
  }
  break;
```

A persistent singleton with a text setter and an explicit teardown is a **banner**. A chat
line needs neither: it is appended to a list and forgotten. And the packet carries a flag
whose only job is to distinguish "show this" from "take it away", which a chat line has no
use for at all.

### Two consequences for anything that sends it

1. **An empty string is a teardown**, because `*message == '\0'` is tested alongside
   `flag == 0`. `net::broadcast::banner("")` therefore emits the teardown form rather than a
   flag-1 packet with a zero-length string.
2. **Re-sending the same text is not idempotent on screen.** The prologue resets the object
   *before* it looks at the flag, so an identical resend restarts whatever the object is
   doing. `world::session::rates` remembers what it last sent for exactly this reason.

## What is NOT established

* **That the banner scrolls, and that it is at the top.** [I], from the packet's shape
  matching the same family's scrolling-header type in other versions, and from the owner having
  asked for a scrolling banner. If it turns out to be a static bar or a popup, the packet and
  the schedule are still right and only the name is wrong.
* **Whether it survives a field change.** Unknown. If the client tears its UI down on
  `SetField`, an event that started before a `!map` would vanish until the next five-minute
  cycle. `world::session::rates` deliberately does **not** re-assert it, because re-asserting
  would restart the scroll on every map change - a visible wrong answer in the common case,
  traded against a silent one in a case nobody has measured. One line of a test run settles
  it.
* **The other thirty types.** Only 0, 2, 4 and 5 were read closely enough to name. Types 26,
  28 and 30 are structurally interesting - 26 shares the flag with the banner, 28 and 30 take
  a second string - and nobody has looked.
* **`FUN_14229c3d0`'s third argument**, sent as 0. It may be a duration, a colour or a
  channel. Not read.
