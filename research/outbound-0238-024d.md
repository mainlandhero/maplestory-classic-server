# Outbound `0x0238` and `0x024D` — what they are, and whether the client waits

Written 2026-08-19, **statically, with no client run and without Ghidra** (the project was
locked by another session). Everything below comes from text already on disk:
`research/msexe-setfield-aftermath.c`, `research/msexe-gamestage-dispatch.c`,
`research/msexe-gamestage-cases.txt`, `research/msexe-gamestage-outbound.txt`,
`research/msexe-send-opcodes.txt`, `research/msexe-packet-fields.txt`, and the capture
`research/fixtures/setfield-accepted-client-entered-world-world.log`.

Markers: **[L]** read from a decompilation, listing or capture; **[D]** derived from those;
**[I]** inferred or a candidate.

---

## Answer up front

| question | answer |
|---|---|
| what are they | **unidentified.** Two empty requests fired back-to-back from one site, 30 bytes apart, inside the world object's field-entry reset. No name is established and the reference source gives nothing usable. |
| do they carry anything | **No. Both bodies are empty**, and that is confirmed three ways [L]. |
| does the client block on either | **No blocking is visible at the send site, and none is demonstrated by the one run we have.** A later poll cannot be ruled out — see the named gap in §2.4. |
| is there a reply handler in `FUN_142cbaa80` | **No candidate found**, over the part of the switch that is readable on disk (94 of 273 cases). The search has a working positive control and a stated coverage limit. |
| what to send next run | **Send nothing.** Watch. Reasons in §4. |

---

## 1. Where they are built, and what surrounds them

### 1.1 The site

Both are built and sent by **`FUN_142caa4e0`** (7069 bytes, `142caa4e0..142cac07d`), at
`research/msexe-setfield-aftermath.c:485-491` [L]:

```c
  uVar6 = FUN_1429e3ef0();
  *(undefined4 *)((longlong)param_1 + 0x3244) = uVar6;   // byte 0x3244 <- a millisecond clock
  *(undefined4 *)(param_1 + 0x648) = 1;                  // byte 0x3240 <- 1
  FUN_1406ed520(local_488,0x238);                        // begin packet 0x0238
  FUN_1415d01c0(local_488);                              // send
  FUN_1406ed520(local_8d8,0x24d);                        // begin packet 0x024D
  FUN_1415d01c0(local_8d8);                              // send
  uVar6 = (*DAT_143262db0)();
  *(undefined4 *)((longlong)param_1 + 0x34ec) = uVar6;   // GetTickCount-style stamp
```

`FUN_1406ed520(buf, op)` is *begin packet* and `FUN_1415d01c0` is *send*; both are
established in `research/msexe-client-opcodes.md` [L].

Exact call addresses from the listing-derived table `research/msexe-send-opcodes.txt`:

```
0x0238  568    FUN_142caa4e0 @ 142caa4e0  (call at 142caadc6)
0x024D  589    FUN_142caa4e0 @ 142caa4e0  (call at 142caade4)
```

`142caadc6` and `142caade4` are **30 bytes apart** [L] — consistent with the decompilation's
build/send/build/send with nothing in between. Each opcode has **exactly one** build site in
the whole image among the 1881 resolved of 1894 call sites [L]. (13 call sites pass the
opcode in a register from a caller and are unresolved; and a builder inside the Themida
region would not appear at all. So "one builder each" is bounded by that, not absolute.)

### 1.2 They carry nothing — three instruments agree

1. **The decompilation**: literally nothing between `FUN_1406ed520` and `FUN_1415d01c0` on
   either packet [L].
2. **`research/msexe-packet-fields.txt`**, which lists the field-writer sequence per builder:
   both rows begin at the send with **no** `u8`/`u32`/string/raw primitive
   (`FUN_1406ed840`/`ed9d0`/`edc80`/`ede20`) in the sequence [L].
3. **The wire**: `research/fixtures/setfield-accepted-client-entered-world-world.log` —
   `0x0238 UNKNOWN, 0 byte body` and `0x024D UNKNOWN, 0 byte body` [L].

`docs/opcodes.md` already lists both with an empty field column; the "Canvas / alpha" text in
that table is the nearest string literal in the *builder function*, not a name for the packet.
Do not read it as one.

### 1.3 What `FUN_142caa4e0` is

**`param_1` is the world object `DAT_143aa84a0`** — the same `this` the channel's inbound
dispatcher `FUN_142cbaa80` receives [D]. Evidence:

* Both functions write `param_1 + 0x466` (index form, byte `0x2330`) and
  `param_1 + 0x471` [L].
* Both write the *identical* idiom at that field:
  `*(int *)(param_1 + 0x466) = 0; iVar = FUN_1429e3ef0(); *(int *)((longlong)param_1 + 0x2334) = iVar;`
  — `FUN_142caa4e0:268-270` and `:442-444`, and `FUN_142cbaa80` at four sites [L].
* `FUN_142caa4e0` lives at `0x142ca…`, the same subsystem as `FUN_142ca5c50` (the function
  that *sets* `DAT_143aa84a0`) and `FUN_142caebe0` (called on the world object by the migrate
  handler) — see `research/msexe-gamestage-dispatch.md` [L].

The function itself is a **field-entry reset of the world object** [D]: it constructs about
fifteen sub-manager singletons (`param_1[0x51d]` … `param_1[0x53a]`), zeroes a long run of
state, resets several timers, empties an intrusive list at `param_1[0x6d5]`, creates four
`Canvas` COM objects and sets their alpha from `DAT_143ac87a0 + 0x58/0x5c/0x60/0x64`, and
returns. It contains no loop that waits on anything.

It calls `FUN_1415aafa0(9)` at line 530. The SetField handler `FUN_142097f80` calls the same
function with `3,4,5,6,7` (`research/msexe-stage-setfield.c:612,1744,1908,2042,2177`) [L], so
`FUN_1415aafa0` looks like a phase/progress marker and `FUN_142caa4e0` looks like a later
phase of the same field-load sequence [I].

**`FUN_142097f80` does not call `FUN_142caa4e0`** — it is absent from
`research/msexe-stage-setfield.c` [L]. The caller of `FUN_142caa4e0` is **not determinable
from what is on disk**; only the timing links them (§1.4).

### 1.4 Why they go out at that moment — from the capture

```
06:11:55.730  -> 0x01A0 SetField (characterData=1, minimal record)
06:11:56.152  <- 0x0238, 0 byte body
06:11:56.152  <- 0x024D, 0 byte body        (+422 ms, same millisecond as each other)
~06:11:59.5   CLIENT FAULT 0xC0000005 at 0x140ce89d6   (per STATUS.md §1a)
06:12:01.678  socket closed by the client (os error 10054)
```

So: the client accepted `SetField`, ran the world object's field-entry reset, and about a
third of the way through that reset fired both packets [D]. They are **not** a response to
anything we sent field-by-field; they are unconditional in the reset's straight-line path [L].
Checked specifically: between the function's first line and the send site there is **no
`return`, no no-return call, and exactly one `goto`** — a local forward jump inside a
string-copy block (`LAB_142caa990`, line 336) that lands well before the sends. Once
`FUN_142caa4e0` is entered, both packets go out.

---

## 2. Does the client block on either? — the question that matters

**Short answer: nothing on disk shows a block, the one run we have does not show a block, and
one specific thing could not be checked. Read §2.4 before treating this as settled.**

### 2.1 The send path is fire-and-forget [L]

`FUN_1415d01c0` (171 bytes, `research/msexe-bootloop-exit.c:169`) records the opcode, then:

```c
  lVar3 = FUN_140caa4d0();
  if (lVar3 != 0) { FUN_1415d3990(lVar3,param_1); }
  return;
```

No wait object, no message pump, no return value. It hands the packet to the connection and
returns.

### 2.2 No wait, no re-entry guard, no UI disable at the site [L]

* `FUN_142caa4e0` continues for roughly two-thirds of its body after the second send — four
  Canvas constructions, a list drain, `FUN_141418c20`, `FUN_1411fc3d0`, `FUN_142d3ca60` —
  and returns normally. There is no loop, no sleep, and no blocking call after the sends.
* The two packet objects are stack buffers destroyed at the very end of the function
  (`FUN_1406ed610(local_8d8); FUN_1406ed610(local_488);`, lines 1194-1195). That is RAII
  scope, **not** a wait.
* **The known blocking idiom in this client is absent here.** The delete-character builder
  blocks by setting `stage+0xd4` before its send and *early-returning at the top of the same
  builder* while the latch is set (STATUS.md, "DONE: delete a character"). `FUN_142caa4e0`
  has **no guard on `0x3240` at its entry** — lines 184-190 read no such field [L]. Whatever
  `0x3240 = 1` means, it does not gate this function.

### 2.3 The `0x3240 / 0x3244` pair is the same shape as three sibling pairs, and all three are pollers

The decompiler mixes index form (`param_1 + N`, ×8) with byte form (`(longlong)param_1 + N`).
Three independent sites confirm the convention and the layout:

| flag (index → byte) | timestamp (byte) | written by | value written |
|---|---|---|---|
| `0x466` → `0x2330` | `0x2334` | inbound `0xb7`, `0xb8`/`0xf5`, `0xf8`, `0xf9`, and `FUN_142caa4e0` ×2 | flag `= 0`, time `= now` |
| `0x63a` → `0x31d0` | `0x31d4` | inbound `0xb7` | flag `= byte from packet` |
| `0x64e` → `0x3270` | `0x3274` | inbound `0x135` | flag `= bool`, time `= now - 10000` |
| **`0x648` → `0x3240`** | **`0x3244`** | **`FUN_142caa4e0` only** | **flag `= 1`, time `= now`** |

Each flag sits exactly 4 bytes below its timestamp, in all four cases [L]. The `0x2330/0x2334`
pair is the **keepalive**: inbound `0xb7`/`0xb8`/`0xf5`/`0xf8`/`0xf9` all reset it to
`count = 0; lastContact = now` [L]. The `0x3270/0x3274` pair is an "is this event window
open" bool plus a *last-checked* stamp deliberately set to `now - 10000` so the next poll
fires immediately [L].

**[I] So the family is (state, last-polled-at), not (request-pending, sent-at).** Under that
reading, `0x3240 = 1; 0x3244 = now` means "this feature is now active; the next periodic tick
is due one interval from now", and an unanswered request would produce a *resend on a timer*,
not a freeze. Note the contrast the same function makes with itself two lines later — it
writes `[0x3548] = now - 600000` and `[0x50b] = now - 300000`, the "fire immediately" form.
Writing plain `now` at `0x3244` **delays** the next tick.

This is an inference from three siblings. It is not proof, and `1` is not `0`.

### 2.4 The gap, stated plainly

**Nothing anywhere on disk reads or clears `0x3240` or `0x3244`.** Searched: every `.c`,
`.txt` and `.md` in `research/` and `docs/`, for `0x648`, `0x3240`, `0x3244`, the decimal
forms, and the neighbouring offsets `0x646`, `0x647`, `0x649`, `0x64a`, `0x323c`, `0x3248`,
`0x324c`. One hit each for `0x648` and `0x3244` — both the write in
`msexe-setfield-aftermath.c` itself.

**The instrument works.** The same grep over `research/msexe-gamestage-dispatch.c` returns 23
distinct world-object field writes, including the immediate neighbours `0x31d4`, `0x3274`,
`0x2334`, `0x23d4`, `0x2fa4`, `0x34ac`, `0x63a`, `0x64e` [L]. So the silence at `0x3240` is a
real absence in the text I have, not a broken search.

**But the coverage is bounded, and this is the honest limit.** Of the dispatcher's 273 cases,
**179 forward to a handler function that is not in this file** and only **94 are inlined**
(counted from `research/msexe-gamestage-cases.txt`). The search therefore proves only that
*no inlined inbound case, and no code in the dispatcher body itself, touches `0x3240`*. A
forwarded handler receives the same world object as its first argument and could clear it.

**It also does not cover the client's own per-frame tick**, which is where a poller or a
timeout would live and which is not decompiled anywhere in `research/`.

### 2.5 What the one run actually shows [L]

After sending both and receiving no answer, the client kept executing for about **3.4
seconds**, then faulted at `FUN_140ce89c0+0x16` — a refcounted release dereferencing a
non-null invalid pointer, already diagnosed in STATUS.md §1a as the consequence of the
all-zero record putting the character on map `0`. It **did not resend** either packet in that
window, and it **did not stall waiting** — it progressed to a different failure entirely.

That is evidence against an immediate freeze. It is **not** evidence against a slow one: the
client died of an unrelated cause before any plausible timeout could expire, so the run cannot
distinguish "no timeout" from "a timeout longer than 3.4 s".

### 2.6 Verdict

**No block is demonstrable, and no block is visible in any code I can read.** The send is
fire-and-forget, the sending function does not guard on the latch it sets, the latch matches a
family of pollers rather than a family of request latches, and the client demonstrably kept
running without a reply. The residual risk is confined to (a) one of the 179 forwarded inbound
handlers clearing `0x3240`, and (b) a per-frame tick that reads it — neither of which exists in
text on disk.

---

## 3. Is there a reply handler in `FUN_142cbaa80`?

**No candidate identified.**

### 3.1 The numbering does not help, and it would be a trap to let it

`0x0238`/`0x024D` are **outbound** opcodes. The channel's inbound switch spans `0x0070..0x019f`
plus the two strays `0x0275` and `0x039a` — 273 cases, and `0x0238` and `0x024D` are simply not
inbound values at all [L]. The outbound set runs `0x007A..0x03F8` across 175 opcodes [L]. These
are two different enums; a numeric near-match between them means nothing.

### 3.2 What I searched instead, and what it covers

A reply to a request issued *by the world object* would be handled by a case in
`FUN_142cbaa80` acting on that same object, and would plausibly write near the state the
request set. So:

* **`0x3240` / `0x3244` are never written by the dispatcher body or by any of its 94 inlined
  cases** [L]. Positive control as in §2.4.
* **Both stray cases were read in full** and excluded [L]:
  * `case 0x275` — reads `u8`, `u32`; if the `u8` is zero it *sends* `0x17e`. Touches no field
    on the world object.
  * `case 0x39a` — reads 8 raw bytes into `param_1[0x831]` (byte `0x4188`) and a `u32` into
    `param_1[0x832]`. Nowhere near `0x3240`.
* The remaining **179 forwarded handlers are not on disk**, so nothing here rules them out.

### 3.3 One thing worth knowing before anyone is tempted to guess

`FUN_142cbaa80`'s switch has **no `default:` body** [L]. An opcode it does not recognise falls
straight to the tail: a `DAT_143ac87a0` housekeeping call, the `0x121..0x126` sub-switch under
`DAT_143ad1850`, and an append to a ~20-entry opcode ring buffer at `param_1[0x74c]`. So an
*invented* inbound opcode that happens to match no case is silently ignored — no dialog, no
fault, no log. **The danger is not the miss. It is the hit**: an opcode that does match a case
runs a real handler against a body we made up, and the client then reads fields we never wrote.

Also worth remembering: `FUN_1415d59b0` drops every game-stage packet in silence when
`DAT_143aa84a0` is null (`research/msexe-gamestage-dispatch.md`), so "no visible effect" can
never be read as "the client rejected it".

### 3.4 The reference source gives nothing here

Checked `C:\Users\user\Desktop\ModernMapleSource\v214 src\…\handlers\header\InHeader.java`
(client → server): `568` is `SET_MAX_GAUGE`, tagged `// v265.3`, and **`589` has no entry at
all** — 580..593 are absent from the enum, verified with a control that found 511..518 [L].
Different game version, and the outbound enum's offset against mscw's is not established
(the inbound block alignment scored **1 of 9** on held-out controls, per
`research/msexe-gamestage-dispatch.md`). **`SET_MAX_GAUGE` is not a candidate worth carrying
forward** [I]; it is recorded only so nobody spends the lookup twice.

---

## 4. Recommendation for the next launch: send nothing, and watch

**Answer neither.** Concretely:

1. **Do not add a reply.** There is no candidate opcode. Picking one out of 273 cases would be
   a guess, and §3.3 says a lucky hit is worse than a miss — it would run a real handler on a
   fabricated body and move the client into a state nobody has read. That is exactly the
   failure mode `crates/world/src/session.rs:17` already warns about.

2. **The "always answer" rule does not obviously apply here, and that is the point.** Every
   case where it bit — `0x0081`, `0x0082`, the delete request — was a packet the client sent
   **in response to a user clicking something**, with a latch that disabled the UI until the
   answer arrived. These two are sent unconditionally from an initialisation routine, with the
   user touching nothing, and the client kept running without them. The rule's precondition is
   not met by the evidence we have.

3. **Change one thing.** The next run's variable is the map id at stat-block offset 84
   (STATUS.md §1a-ii). Adding a speculative reply in the same run would make an unexplained
   outcome uninterpretable — which is the mistake this project has already paid for once.

4. **What to watch, and what each outcome means.** Say this to the owner before they launch:

   | on screen / in the log | reading |
   |---|---|
   | the map draws, `0x0238`/`0x024D` still unanswered, client responsive | **these two are not blocking. Close the question.** |
   | `0x0238` and/or `0x024D` **arrive again** in `world.log` after a gap | they are polled, and `0x3244` is a "last polled" stamp — §2.3 confirmed. Note the interval; still not a freeze. |
   | the client is **alive but every button is dead**, including the quit prompt, and `world.log`'s last inbound line is one of these two | **that is the freeze**, and this document is wrong. Reopen §2.4 immediately. |
   | the client faults again at `0x140ce89d6` | still the map/teardown path, unrelated to these two. |

5. **Keep logging them in full.** They are `UNKNOWN` in `crates/net`, which is what makes
   `world.log` print the whole body. Leave them unnamed — an invented name here would be the
   fifth wrong one (`research/msexe-client-opcodes.md` records five of eight guessed names as
   wrong, two of them actively misleading).

---

## 5. What could not be determined, and the cheapest way to settle each

| open question | what would settle it | cost |
|---|---|---|
| Who reads/clears `0x3240` and `0x3244`? | Ghidra `dataref`/xref on `DAT_143aa84a0 + 0x3240`, or decompile the world object's per-frame tick. Note `tools/xref.py` is documented as missing read/write forms — use `tools/dataref.py`. | no client run |
| Does any of the **179 forwarded** inbound handlers write `0x3240`? | Decompile the 179 and grep, or one targeted xref as above. | no client run |
| What calls `FUN_142caa4e0`? | Xrefs on `142caa4e0`. If it comes back empty with a working control, the caller is virtualised and static reading stops. | no client run |
| Are they resent on a timer? | The next launch answers it for free — `world.log` records every inbound packet. | free |
| What are they *for*? | Unresolved. Neither the image nor the reference names them. An in-process watch on `142caadc6` would confirm the site but not the meaning. | not worth a run |

---

## Two traps for the next reader

* **`stage+0x238` in `docs/character.md` is not this opcode.** That is a *field offset* on the
  **login stage** object — the "a screen transition is running" latch that builders refuse
  while non-zero. A grep for `0x238` hits both. They are unrelated.
* **"Canvas / alpha" is not a name.** `docs/opcodes.md` and `research/msexe-packet-fields.txt`
  attach it to both opcodes; it is the nearest string literal inside `FUN_142caa4e0`, which
  constructs four `Canvas` COM objects several hundred lines *after* the sends. Both packets
  are empty and neither has anything to do with a canvas.
