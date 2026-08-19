# The player's own chat: `0x0231`, and how it was found without a single client launch

**Built 2026-08-19, unconfirmed on screen.**

The owner: *"I tried to send 3 chat messages, 'Hello', 'Hello2' and 'Hello3'. The ideal response
is to have a chat bubble pop up above the character's head and I see my own chat message
down in the chat log."*

They saw neither. The capture says why, and it is not subtle: the client sent three `0x00E7`
bodies, the server matched each against `!map`, found no match, and **returned nothing**.

```
23:36:13.562 <- 0x00E7  7438da06 0500 48656c6c6f 03      "Hello"
23:36:15.862 <- 0x00E7  7a41da06 0600 48656c6c6f32 03    "Hello2"
23:36:17.322 <- 0x00E7  3847da06 0600 48656c6c6f33 03    "Hello3"
```

**The client renders nothing for its own chat.** Typing sends `0x00E7` and stops there. Both
halves of what the owner wants - the balloon and the log line - come from one packet coming back.

## The packet

**Inbound `0x0231`.**

```text
u32  characterId
u8   flag          0
str  text          u16 length, then bytes
u8   tail A        0
u8   tail B        0
```

The leading `u32` is read by the dispatcher, before its switch, so it is shared by every
opcode in the `0x226..0x276` range. The four fields after it are read by the handler.

## How it was found

Two independently derived lists, intersected. Neither was built for this.

**List one: the seventeen chat-window printers.** `research/talking-back.md` established
that `FUN_1415eca30(char**, u16 type)` is the string printer and **type 7 is the chat
window**, by way of `FUN_14209ee50` posting string id `0x533` - `[Welcome] Welcome to
MapleStory!!`, the line visible in every capture. A `rel32` sweep of all **1132** call sites
finds **17** that load type 7 within 24 bytes of the call. `notice::CHAT_NOTICE` (`0x00BB`)
came out of that sweep, so the instrument has a confirmed positive: the count reproduced
exactly when re-run for this. **[L]**

**List two: the eighteen balloon creators.** `research/npc-chatter.md` established
`FUN_14158f5c0` as the balloon factory - the first WZ path it loads is `UI/ChatBalloon.img/`
at `0x14158f776`. `tools/callers.py` gives **24 call sites in 18 functions**. **[L]**

**The intersection is two functions**, `0x1427834b0` and `0x142784970`. Only the second
decodes a packet:

| | reads | |
|---|---|---|
| `FUN_142784970` | `u8, str, u8, u8` | **one** string |
| `FUN_1427847a0` | `u8, str, str, u8, u8, u8` | **two** strings |

Both are reached from one dispatcher, `FUN_1429bafb0`, which has **zero direct callers** - it
is virtual - and which normalises the opcode with `LEA EAX,[RSI-0x226]` / `CMP EAX,0x50` at
`0x1429bb100` before jumping through a table at `0x1429bb5d0`. Reading that table's 81 RVAs
out of the exe:

| index | opcode | case | handler |
|---:|---|---|---|
| 0 | `0x0226` | `0x1429bb124` | `FUN_1427847a0` - two strings |
| **11** | **`0x0231`** | `0x1429bb134` | **`FUN_142784970`** - the one we send |

**[L]** throughout: every step is a table or a call target read out of
`client-patched/MapleStory.exe`, not a name from the reference tree.

## What is not known

* **The three `u8`s.** In this game family the one before the text is an admin/GM marker and
  one of the two after it suppresses the chat-log line so that only the balloon shows. That
  is **[I]** - nothing here read what they do. All three go out as `0`, which asks for the
  ordinary case of both balloon and log line.
* **`0x0226`, the two-string form.** A packet carrying a name *and* a message is the shape of
  a whisper, and the dispatcher gives it special setup at its entry (`CMP EDX,0x226 / JNZ`
  at `0x1429bafcb`, guarding two `u32` reads the other opcodes skip). Untested, so unsent.
* **Whether the four reads inside `FUN_142784970` are unconditional.** They sit at
  `0x142784998`, `0x1427849ac`, `0x142784a5b`, `0x142784a69` - a straight run near the top -
  but no guard-interval pass was done, so a gate is possible.

  **Sending all four is the safe direction either way.** A client that reads fewer bytes than
  arrive never looks at the rest; one that reads *more* than arrive throws on underrun. A
  field that turns out to be gated off costs a wasted byte; a field left out costs the
  session.

## What the server does now

Anything typed that does not begin with `!` is echoed back as `0x0231` attached to the
speaker's own character id. A `!` line that is not a known command gets a `0x00BB` notice
saying so, rather than vanishing - the same reasoning as the refused `!map`.

**It is a local echo, not a broadcast.** It goes back to the one connection that spoke,
because the server has no concept of a second player in a field yet.

## What to watch on the next run

| what | means |
|---|---|
| a balloon over the head **and** a line in the chat log | `0x0231` is right, and both zero tail bytes are right |
| balloon but **no** log line | one of the two trailing `u8`s is the "balloon only" flag |
| log line but **no** balloon | the balloon has its own guard, like the NPC one at `0x141e3b543` |
| the client dies on the first message | the body is short - a read past the end throws, and `0xC0000374` is what that looked like on 2026-08-19 |
