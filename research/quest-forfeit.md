# Giving a quest up

**The forfeit is `0x0151` action 3, it is already in two captures, and it went unanswered
both times.** Neither `0x01ED` nor `0x01A5` has anything to do with quests.

Labels: **[L]** read off this client's listing or off a capture, **[D]** derived from two or
more [L] facts, **[I]** inferred. Every negative below names the blind spot it rests on.

---

## 0. The one-paragraph answer

| | |
|---|---|
| the request | `0x0151`, body `03 <u32 questId>`, **5 bytes**, no NPC, no position **[L]** |
| the builder | `FUN_142d9ad50`, the *only* one, three UI call sites **[L]** |
| in a capture? | **yes, twice** - `03 e9030000` (quest 1001), 2026-08-20 at `03:31:18.936` and again at `03:45:59.236` after a relog **[L]** |
| does the client clear its own journal? | **no.** The started-map erase has exactly two callers and neither is on this path **[L]** |
| what the server must send | `0x0089` sub 1, `u32 questId`, state **0**, `u8 forgetCompletion` - `net::quest::quest_forgotten`, which already exists **[L]** |
| can a *completed* quest be forfeited? | **no.** The client refuses to build the packet when the id is not in its started map. A chain reset needs a GM command **[L]** |
| `0x01ED` | the client's own log/usage channel, 104 builder sites, first `u32` is a **kind** not a count **[D]** |
| `0x01A5` | a periodic counter flush: `u32 countA, A*{u32,u32}, u32 countB, B*{u32,u32,u32}` **[L]**, verified against six captured bodies |

---

## 1. The request: `0x0151` action 3

### 1.1 The builder, `FUN_142d9ad50`

`research/msexe-send-opcodes.txt` ("opcodes passed to `1406ed520` in EDX") lists **nine**
`0x0151` call sites in **four** functions. `FUN_142d9ad50` is one of them, and it is the whole
function. **[L]**

```text
142d9ad75  mov  edx, 1
142d9ad7a  mov  rcx, [rip+0xd23cff]
142d9ad81  call 0x141b1f960          ; a modal/UI gate - non-zero -> return, send nothing
142d9ad91  call 0x142cbe730          ; rax = charData  (player->[0x2358])
142d9ad96  mov  r8, [rax + 0x1273]   ; the STARTED-quest hash map's bucket array
142d9ada6  mov  ecx,[rax + 0x127b]   ;   and its bucket count
142d9adb1  div  rcx                  ; questId % buckets
142d9adc1  cmp  [rax + 0x10], ebx    ; walk the chain looking for questId
142d9adcd  jne  142d9adc1
142d9adcf  jmp  142d9ae56            ; NOT FOUND -> return, send nothing
142d9add4  call 0x1408f6690          ; an empty string
142d9adef  call 0x142ce5f80          ; the local quest effect  (player, questId, 1, "", 0)
142d9ae14  call 0x1424f0160          ; a UI refresh, behind a null-checked global
142d9ae19  mov  edx, 0x151
142d9ae23  call 0x1406ed520          ; COutPacket(0x0151)
142d9ae29  mov  dl, 3
142d9ae30  call 0x1406ed840          ; u8  3
142d9ae35  mov  edx, ebx
142d9ae3c  call 0x1406ed9d0          ; u32 questId
142d9ae46  call 0x1415d01c0          ; send
142d9ae51  call 0x1406ed610          ; dtor
```

`charData+0x1273` is the started-quest map - the same field `crates/net/src/quest.rs`
documents from the decoder side, arrived at here from a completely different function. **[L]**

`tools/encodes.py 0x142d9ad50 2` agrees independently: `CTOR, w_u8, w_u32, SEND` and nothing
else. Two field writes, five bytes.

> **Instrument note.** `tools/encodes.py`'s table maps `0x1406ED610` to `SEND`. In this
> function the send is `0x1415d01c0` at `142d9ae46` and `1406ed610` is the **destructor** one
> instruction later. It does not affect the field count - there are two writes either way -
> but a body length taken from that label alone would be counting a destructor as a field.

### 1.2 Three callers, all UI

`tools/callers.py 0x142d9ad50` - which now reports calls, tail `jmp`s **and** data pointers,
after the 2026-08-20 fix:

```text
0x142d9ad50: 3 call site(s) in 3 function(s)
    0x14240ff30   0x1424101c4
    0x1424c85e0   0x1424c8743
    0x14255d040   0x14255d3f9
0x142d9ad50: 0 tail jmp site(s)
0x142d9ad50: 0 qword pointer(s) to it in the image
```

So the forfeit has exactly three entry points and they are all buttons. **[L]**

### 1.3 The action census - and why tag 3 was missed

`crates/net/src/script.rs` documents **six** tags and its `QuestRequest::action` field says
*"which of the six builder sites sent this"*. That six came from enumerating the six `0x0151`
sites inside **one** function, `FUN_141f0e4c0`. There are nine sites in four functions. **[L]**

| tag | builder | site | body after the `u8` | bytes |
|---:|---|---|---|---:|
| 0 | `FUN_141f18c20` | `141f1a05a` | `u32 q, u32 err, ...` - the "cannot do that" report | var |
| 1 | `FUN_141f0e4c0` | `141f0ed61` | `u32 q, u32 n, [i16 x, i16 y], u32 sel` | 17 or 13 |
| 2 | `FUN_141f0e4c0` | `141f0e7a3`, `141f0eca0` | `u32 q, u32 n, ...` | 17 or 13 |
| **3** | **`FUN_142d9ad50`** | **`142d9ae23`** | **`u32 q`** | **5** |
| 4 | `FUN_141f0e4c0` | `141f0e835` | `u32 q, u32 n, i16 x, i16 y` | 13 |
| 5 | `FUN_141f0e4c0` | `141f0e8d8` | `u32 q, u32 n, i16 x, i16 y` | 13 |
| 6 | `FUN_141f0e4c0` | `141f0e6f5` | `u32 q, u32 n, i16 x, i16 y` | 13 |
| 7 | `FUN_14289dd90` | `14289f22d` | `u32 <not a quest id - `r15d`, unidentified>` | 5 |

This is exactly the failure mode `CLAUDE.md` names: *searching a known list for the wrong
set*. The list was six sites in one function; the set is nine in four.

**The bound on this table, stated rather than implied.** `msexe-send-opcodes.txt` resolves
**1881 of 1894** `1406ed520` call sites; **13** pass a computed opcode and show as `????`.
None of the thirteen (`1409fe0a0`, `140ca2d90`, `140ca30c0`, `140ca3130`, `140ca3320`,
`140db9610`, `1413106e0`, `141d3b880`, `1428c8120`, `1428d2b70`, `14292ede0`, `1429976c0`,
`142dbb600`) is in the quest UI, but a tenth `0x0151` site could in principle hide in one of
them. **[D]**

### 1.4 The captures - [L], and they are the owner's complaint

Two runs on 2026-08-20, now preserved out of the rolling buffer:

```text
research/fixtures/quest-forfeit-0151-action3-on-the-wire-world.log
  03:26:18.005  SetField   quests: 2 started / 0 completed
  03:30:17.966  0x0089     quest 1000 completed, quest 1001 accepted
  03:31:18.936  <- 0x0151  5 byte body  03 e9030000        <- THE FORFEIT, quest 1001
  03:31:18.937     "is not answered yet"

research/fixtures/quest-forfeit-0151-action3-resent-after-relog-world.log
  03:45:49.968  SetField   quests: 2 started / 1 completed   <- 1001 IS STILL STARTED
  03:45:59.236  <- 0x0151  5 byte body  03 e9030000        <- and they did it AGAIN
  03:45:59.236     "is not answered yet"
```

The second run's opening `SetField` is the proof that nothing stuck: it is the book the first
run ended with, forfeit and all.

**The forfeit does not freeze anything.** After the unanswered `03:45:59.236` the client kept
sending `0x00D9` movement, and at `03:46:13.219` it sent a `0x0107` inventory move that the
server answered normally. So this is not one of the requests that latches `player->[0x2330]`
(`research/npc-click.md` §4.3). **[L]** - which is a licence to take one's time getting the
reply right, not a licence to keep not sending it.

### 1.5 `crate::script::parse_quest_request` cannot read it

It reads a fixed **9-byte** head (`u8 action, u32 questId, u32 npcTemplateId`). A forfeit is
five bytes and carries no NPC, so the second `u32` fails and the function returns `None` -
and `Session::on_quest_request` turns `None` into `Vec::new()`. **Silence.** That is not a
prediction; the two captures say "is not answered yet" on exactly that path.

`crates/net/src/questforfeit.rs` has this as a regression test.

---

## 2. The client cannot clear its own journal. The reply is not optional.

`FUN_1402e0ec0` is the client's **erase from the started-quest map** - named in
`crates/net/src/quest.rs` from the `0x0089` side. `tools/callers.py 0x1402e0ec0`:

```text
0x1402e0ec0: 3 call site(s) in 2 function(s)
    0x140304b20   0x14030755d              the character-record decoder
    0x142d5b750   0x142d5b8df, 0x142d5c557 the 0x0089 sub-1 handler (states 0 and 2)
0x1402e0ec0: 0 tail jmp site(s)
0x1402e0ec0: 0 qword pointer(s) to it in the image
```

**Two functions, and neither is on the forfeit path.** `FUN_142d9ad50` does not call it, and
neither does `FUN_142ce5f80`, the local effect it invokes on the way (35 call sites, no
`1402e0xxx` among them). **[L]**

> **The blind spot, named.** `tools/callers.py` reports three kinds of reachability - `call`,
> tail `jmp`, and a pointer in data - and all three are zero here beyond the two named
> functions. What it still cannot see is a *computed* call through a register loaded from
> something other than a static pointer. That is the same shape as the `lea`'d-pointer store
> that `CLAUDE.md` records; it is narrower here, because a virtual call would need the
> address in data and there is none. **[D]**, not [L], on "no third path exists".

So: **pressing "give up" changes nothing on screen until the server answers.** The row stays,
which is exactly what the owner saw, and re-clicking just sends the packet again.

### 2.1 What to send

`net::quest::quest_forgotten(quest_id, forget_completion)` - already written, already tested,
never wired. `0x0089` sub 1: `01 <u32 questId> 00 <u8 forgetCompletion>`, 7 bytes.

* `forgetCompletion = 0` - erase from the **started** map only. The right answer to a
  player-initiated forfeit.
* `forgetCompletion = 1` - **also** erase from the completed map (`FUN_1402e3550`), making a
  finished quest offerable again. The client never asks for this.

`crates/store`'s `forget_quest(character_id, quest_id)` already removes the row and is already
tested (`forgetting_a_quest_removes_the_row_entirely`). **The storage half exists.**

### 2.2 A forfeit cannot reset a *completed* quest, and that matters for the goal

`FUN_142d9ad50` returns without building anything when the id is not in the started map -
three separate exits to the epilogue at `142d9ae56`: `142d9ada0` (no map at all), `142d9adbb`
(the bucket is empty) and `142d9adcf` (the chain ran out). A completed quest is not in that
map. **[L]**

So "give up" can undo an in-progress quest and nothing else. Re-running a chain from the top -
which is the actual goal here, *"the entire quest chain has to be reset"* - needs the server
to volunteer `quest_forgotten(id, true)` for the completed ones. There is no client request
for it and there never will be. See §6.

This also explains a detail of the owner's report. In
`research/fixtures/equip-into-empty-hat-slot-kills-client-world.log` they had **completed** 1000
and 1001 by `06:08:34`; there is no action-3 packet anywhere in that run, because by then the
client would have refused to send one.

### 2.3 One consequence worth deciding on purpose

Quest 1001's `Act.0.item.0` hands over the mirror (`4031000`) on **accept**. A forfeit does
not take it back - nothing in the packet says to - so accepting again would hand over a
second one. That is a server policy question, not a protocol one, and it should be a
deliberate choice rather than a surprise on the second run.

---

## 3. `0x01ED` and `0x01A5` are not the forfeit

The brief's framing was that both "carry the quest id". They carry `1000`, `1001` and `1002`,
and in Mushroom Town those are also **the first three NPC object ids this server hands out**
and **the first three quest ids in `Quest.wz`**. The coincidence is the whole trap.

### 3.1 The positive control that kills it

`research/fixtures/character-on-map1-playable-world.log`, **2026-08-19**, contains **zero**
occurrences of `0x0151` or the word `quest` - no quest packet in either direction, and the
quest subsystem did not exist yet. It contains, at `14:01:40.449`:

```text
0x01ED  20 bytes  01000000 01000000 e8030000 01000000 0bccceb9
0x01ED  16 bytes  02000000 01000000 e8030000 6987f2eb
```

**byte-for-byte identical** to the two the brief quotes. The same pair appears in
`dressed-in-world-npc-click-00f2`, `attack-with-mobs-present-zero-targets`,
`channel-list-shows-two-but-unselectable`, `amherst-1013-heap-corruption` and
`equip-into-empty-hat-slot-kills-client` - six runs across three days, always the same bytes,
always within a second of field entry (0.70 s in the equip run, 0.74 s in the relog run,
measured off the `SetField` line in each). A value that is constant across a run with no
quests and a run with three cannot be a quest id. **[L]**

That is also the answer to the brief's first question - *"the two `0x01ED`s arrive before
quest 1000 was accepted; explain that"*. They arrive before it because they have nothing to do
with it.

### 3.2 The same key twice in one packet

`research/fixtures/quest-forfeit-0151-action3-resent-after-relog-world.log`, `03:45:50.706`:

```text
0x01ED  56 bytes  01000000 04000000
                  e8030000 01000000 0bccceb9
                  e8030000 03000000 2cadaa3c     <- 1000 again, second key 3
                  e9030000 03000000 1f7b52ff
                  ea030000 02000000 c0710232
```

A quest journal cannot list a quest twice. The 12-byte entry is a **composite key**, and the
client's own map node says so: it descends only when *both* halves compare
(`142d17e08 cmp [rdx+0x1c],eax / jne / 142d17e0d cmp [rdx+0x20],r11d`). **[L]**

### 3.3 And the keys are not even in the quest range

`equip-into-empty-hat-slot-kills-client-world.log:6758` (`06:11:52.223`), byte-identical to
`attack-with-mobs-present-zero-targets-world.log:3479` on a different day:

```text
0x01A5  24 bytes  02000000 00cf7b05 1611a5ac  10f67b05 1611a5ac  00000000
                           ^0x057BCF00        ^0x057BF610
```

92 131 584 and 92 140 048, sharing one value. The client ships 322 quests. **[L]**

### 3.4 `0x01A5`: the exact layout, from its only builder

`FUN_142d17960` is the **only** function in the image that builds `0x01A5` (two call sites,
both inside it). It flushes two containers hanging off one singleton: **[L]**

```text
142d17a0f  mov  edx, 0xc351 / mov ecx, 0x159 / call 0x14090d160   ; a config value
142d17a3f  imul ... sar 6                     ; cap  = value / 1000   -> the max entries
142d17a6c  call 0x1408fc980                   ; a throttle on the tick
142d17a79  mov  edx, 0x1a5 / call 0x1406ed520 ; COutPacket(0x01A5)

142d17a90  call 0x1406ed640    ; r15 = the CURRENT OFFSET
142d17a9e  call 0x1406ed9d0    ; u32 0                      <- placeholder count A
  loop over [singleton+0x1338], looking each key up in the map at [singleton+0x1310]
    142d17c58  call 0x1406ed9d0   ; u32 key
    142d17c63  call 0x1406ed9d0   ; u32 value
    142d17c68  inc  [rsp+0x20]
142d17d2c  call 0x1406eda30    ; POKE the real count back at offset r15

142d17d38  call 0x1406ed640    ; r13 = the current offset
142d17d46  call 0x1406ed9d0    ; u32 0                      <- placeholder count B
  loop over [singleton+0x1348], looking each PAIR up in the map at [singleton+0x1320]
    142d17f38  call 0x1406ed9d0   ; u32 key1
    142d17f44  call 0x1406ed9d0   ; u32 key2
    142d17f4f  call 0x1406ed9d0   ; u32 value
    142d17f54  inc  r15d
```

So the body is

```text
u32 countA,  countA * { u32 key, u32 value }
u32 countB,  countB * { u32 key1, u32 key2, u32 value }
```

and **the leading `u32` really is a count** - not because it looks like one, but because it is
written as a zero placeholder and back-patched with `1406eda30` after the loop. That was the
brief's second question ("a leading count is the obvious reading and therefore the one to
check"), and this is the check.

**Verified against six captured bodies at five different lengths** - 20, 24, 24, 28, 32 and
68 - in `crates/net/src/questforfeit.rs`'s tests. The 68-byte one
(`quest-forfeit-0151-action3-resent-after-relog-world.log`, `03:45:51.735`) is the strongest:
three of each, so both counts and both entry widths are exercised at once.

The entries are drained as they are reported (`142d17c4b call 0x142d3b8f0` on the lookup map),
which is why a later flush in the same run carries fewer keys than the one before it - visible
at `03:45:51.735` (three pairs) vs `03:46:01.756` (one pair, a fresh one).

### 3.5 `0x01ED`: a log channel, and the first `u32` is a kind

**104 builder call sites** across the whole image (`msexe-send-opcodes.txt`), including the
Meso Market window, the game-stage dispatcher `FUN_142cbaa80`, and ~60 near-identical stubs in
`0x142d0f000..0x142d16000`. Almost all have the shape `w_raw(fixed struct) -> FUN_1406ef4e0 ->
dtor`, and `FUN_1406ef4e0` is `mov rdx,rcx / mov rcx,[rip+0x33d23b6] / test / jne 0x1415d3990`
- "send on that socket **if there is one**". **[L]**

A representative stub, `FUN_142d119e0`: gated on
`FUN_1408fcaa0([this+0x39f0], 0x493e0 /* 300 000 ms */, tick)` - a **five-minute** throttle -
and then it sweeps a struct for `[x+0x47c0] == 1`, `[x+0x47ac] == 1`, `[x+0x47d4] == 1`,
**resetting each to 0** as it goes. That is a "which features did the user touch since the last
report" sweep. **[L]** for the mechanics, **[D]** for the word *telemetry*.

Four kinds are in captures, with four unrelated bodies: **[L]**

| kind | body | seen |
|---|---|---|
| `1` | `u32 count`, `count * {u32,u32,u32}` | every run, at field entry |
| `2` | `u32 count`, `count * {u32,u32}` | every run, same millisecond |
| `0x44` | `u32, u32, u32, str, u32, pairs..., str "10", ..., str "11", ...` | `channel-list-shows-two-but-unselectable`, `17:22:38` |
| `0x60` | *empty* | `character-on-map1-playable`, `14:02:07` |

**What is not established:** what the keys and values *mean*. They are stable across runs for
the same in-game activity, which is what a content hash does. Naming them was not needed to
answer the question that was asked, and guessing would be the third time this project has
attached a confident label to a `0x01ED` body.

---

## 4. `0x0151` action 6 - the second question, and it has a clean answer

**A tag-6 request cannot be provoked for quest 1002. Ever.** The `_ => "0"` arm in
`crates/world/src/session/npc.rs` is not reachable by a tag 6 on that quest, so the fix
`STATUS.md` item 8 proposes would be answering a packet the client cannot send.

### 4.1 The gate, in full

`FUN_141f0e4c0`, `141f0e6b2` onward. `s` = the quest state at `obj+0x28` (0 = not started,
1 = in progress, 2 = complete), `err` = `FUN_140711d70(...)`, the requirement checker. **[L]**

```text
141f0e6b2  call 0x140711d70          ; err
141f0e6bb  je   141f0e778            ; err == 0  -> the normal path (tags 1/2/4/5)
141f0e6c1  cmp  [r14+0x28], 1
141f0e6c6  je   141f0e6d1            ; s == 1    -> maybe tag 6
141f0e6c8  cmp  eax, 0xc
141f0e6cb  jne  141f0e778            ; s != 1 and err != 12 -> IGNORE the failure entirely
141f0e6d1  mov  edx, [r14+0x20]
141f0e6dc  call 0x140715460          ; -> QuestData+0x148, non-empty?
141f0e6e3  je   141f0e769            ; empty -> FUN_141f18c20(obj, err): a LOCAL error dialog
141f0e6e9  ... build 0x0151 tag 6 ...
```

Body: `u8 6, u32 questId, u32 npcTemplateId, i16 x, i16 y` - **13 bytes**, the same shape as
tags 4 and 5. **[L]**

### 4.2 `QuestData+0x148` is `failscript` - settled, with a listing

`research/quest-progress.md` §7 stopped here: *"Which `Quest.wz` node lands at `+0x148` is not
established"*, with two forks. It is the second fork, and this closes it.

`tools/xref.py --string` finds each of the three script node names exactly once in the image,
all three referenced from one loader, `FUN_14072b230`, and the loader stores each into a
different field: **[L]**

```text
14072b5d0  lea r8, ["startscript"]   ...  14072b62a  mov [rbx + 0x138], rax
14072b66e  lea r8, ["endscript"]     ...  14072b6c8  mov [rbx + 0x140], rax
14072b70c  lea r8, ["failscript"]    ...  14072b759  mov [rbx + 0x148], ...
```

and the three predicates read exactly those three fields:

| field | predicate | gates |
|---|---|---|
| `+0x138` startscript | `FUN_140715180` (`140715245`) | tag **4** |
| `+0x140` endscript | `FUN_1407152f0` (`1407153b5`) | tag **5** |
| `+0x148` **failscript** | `FUN_140715460` (`140715525`) | tag **6** |

The instrument spoke three times before it was believed: all three strings were found, all
three references resolved, all three offsets differ. Corroborated from the wire: quest 1002
has `Check.0.startscript = q1002s` and the client sent **tag 4** for it
(`04 ea030000 03000000 ...`, five times across two runs).

### 4.3 So which quests can send a tag 6

`gm-handbook/questlines.txt` has `failscript` on **four quests only**: **20002, 20102, 20202,
20302**, all as `Check.1.failscript` (`q20002f` and friends). **[L]**

Quest 1002 has none. Quest 1000 has none. Quest 1001 has none. **No capture contains a tag 6
because no quest anybody has tested can produce one**, and that is a property of the data, not
of the testing.

The four that can are the siblings of the `Proof of Qualification` set `STATUS.md` item 7
already flags as the cheapest next quests.

### 4.4 How the server tells `Check.0` from `Check.1`

**From its own quest table, because the packet does not say.** The tag-6 body carries a quest
id, an NPC template and the player's position - no state byte. What the *gate* establishes is
the rule: **[L]**

* the quest is in progress (`s == 1`) - so the failure is `Check.1`; **or**
* the quest is not started and `err == 0xC` exactly - so it is `Check.0`, and only for that one
  error code.

A server that keeps quest state (this one does) reads the same fork off its own row: **started
-> `Check.1`, absent -> `Check.0`.** No inference needed and no client field to trust.

`err == 0` is by far the more common way a short turn-in reaches the server: a quest with no
`failscript` falls through to the normal path and sends **tag 2** or **tag 5** even at 4/10
kills. So `research/quest-progress.md`'s conclusion stands and is now the *only* case:
**the server must validate requirements on tag 2 / tag 5 itself.** The client will not do it
for it, and tag 6 is not a substitute.

### 4.5 What to do with STATUS item 8

* Do **not** point the `_ => "0"` arm at `Say."1".stop.item.0` for tag 6 on quest 1002. It
  cannot arrive.
* The right home for `Say.<state>.stop.*` is the **tag 2 / tag 5** path, gated on the server's
  own requirement check - which is where "Eat the `#r#t2010000##k` I gave you..." belongs when
  Roger is clicked without the apple.
* Tag 6, if it is ever handled, should run the quest's `failscript` (`q20002f`, ...) - a
  script body this client does not ship at all (`Script.wz` is a 63-byte header with zero
  entries, `STATUS.md` 2026-08-21), so it would be authored in `data/quest-scripts.txt` like
  Roger's.
* Whatever the arm does, it must still **answer**. `_ => "0"` at least says something; a
  `return Vec::new()` does not.

---

## 5. WIRE IT LIKE THIS

Nothing below is written. **This is unwired**, and it should be marked so in `STATUS.md`
until it is not.

### 5.1 `crates/net/src/lib.rs` - the coordinator's line

```rust
pub mod questforfeit;
```

That is the only `lib.rs` change. `crates/net/src/questforfeit.rs` is new and self-contained;
its 14 tests pass, and the whole `net` suite is **348 passed, 0 failed** with it in - run in a
**private target directory**, not the shared one, so it is not the racy kind of `cargo` result
`CLAUDE.md` warns about (`crates/net` copied to a scratch workspace, `cargo test -p net`).

### 5.2 `crates/world/src/session/mod.rs` - route action 3 before the general parser

`CLIENT_QUEST_REQUEST` already dispatches to `on_quest_request`. The forfeit has to be split
off inside it, because `parse_quest_request` returns `None` for a 5-byte body:

```rust
// in Session::on_quest_request, as the FIRST thing it does
if net::questforfeit::is_forfeit(body) {
    return self.record_quest_forfeit(body);
}
```

### 5.3 `crates/world/src/session/npc.rs` (or a new `quest.rs`) - the handler

Written in the shape `record_quest_start` already uses - `Reply` is a plain struct with
`opcode` / `body` / `what`, there is no `Reply::new`, and the character comes from
`self.claimed_character()`:

```rust
/// `0x0151` action 3. The client has already decided; it cannot clear its own journal.
pub(super) fn record_quest_forfeit(&mut self, body: &[u8]) -> Vec<Reply> {
    // Always answer. A body that does not parse still gets a reply from the caller - see
    // the note below - because an unanswered 0x0151 is silence on the wire.
    let Some(req) = net::questforfeit::parse_forfeit_request(body) else { return Vec::new() };
    let Some(chr) = self.claimed_character() else { return Vec::new() };
    let what = match self.store.forget_quest(chr.id, req.quest_id) {
        Ok(true) => format!(
            "quest {} given up by character {} ({}) and the row is gone",
            req.quest_id, chr.id, chr.name
        ),
        Ok(false) => format!(
            "quest {} given up but we had no row for character {}; the record is sent \
             anyway so the client's journal agrees with ours",
            req.quest_id, chr.id
        ),
        Err(e) => format!(
            "quest {} given up but NOT REMOVED ({e}) - the journal clears now and the \
             quest comes back on the next SetField",
            req.quest_id
        ),
    };
    vec![Reply {
        opcode: net::quest::MESSAGE,
        body: net::questforfeit::forfeit_reply(req.quest_id, false),
        what,
    }]
}
```

Three things that are decisions, not details:

1. **`forget_completion = false`.** A forfeit undoes an acceptance; it must not silently wipe
   a completion the player earned.
2. **Send the `0x0089` even when the store had no row** (`Ok(false)`). The client's journal is
   the thing being fixed, and the case where the two books disagree is exactly the case that
   matters.
3. **The store is the only state, and that is checked**: `Session::quest_book` is
   `self.store.quest_book(character_id)` (`crates/world/src/session/npc.rs:624`) and both
   `SetField` sites call it (`field.rs:202`, `mod.rs:561`). There is no in-memory quest cache
   to keep in step - so removing the row *is* the whole persistence half, and a quest that
   reappears after a map change means `forget_quest` returned `Err`, which the `what` string
   above will say in `world.log`.

### 5.4 The GM command that is the actual point

`!questreset <id>` (or `!forgetquest`), in `crates/world/src/session/gm.rs`, in that file's
own idiom:

```rust
// store: remove the row whatever state it is in
self.store.forget_quest(chr.id, id).ok();
// client: state 0 with forget_completion = TRUE, which clears the completed map too
Reply {
    opcode: net::quest::MESSAGE,
    body: net::questforfeit::forfeit_reply(id, true),
    what: format!("GM !questreset: quest {id} back to untouched for character {}", chr.id),
}
```

`store::forget_quest` deletes the row outright - `"Not started" is the absence of a row`,
`crates/store/src/quest.rs:395` - so one call covers started and completed alike on our side,
and `forget_completion = true` covers both on the client's.

**This is the one that unblocks re-testing a chain**, because the client will never ask to
un-complete a quest (§2.2). `!questreset 1000` then `!questreset 1001` puts Mushroom Town's
first two quests back to untouched without a new character.

Do not send it with, or just before, a `SetField` - `crates/net/src/quest.rs` records both
ways `0x0089` silently does nothing, and a GM command is typed long after field entry, so this
is only a constraint on any code that batches it with a map change.

### 5.5 What must NOT change

`crates/net/src/script.rs`'s `parse_quest_request` should keep rejecting the 5-byte body. It
is documented as reading a fixed 9-byte head; widening it to "maybe there is no NPC" would
make every short or corrupt body parse as *something*, which is how a wrong tag gets acted on.
The split at the dispatcher is the safe shape.

---

## 6. The single discriminating client test

**One change per run.** Wire §5.2 + §5.3 only - not the GM command - and have the owner do this:

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

1. Enter the world on **map 1**. Click **Heena** (the left NPC) and accept quest 1000.
2. Walk east, click **Sera**, hand the mirror over. Quest 1000 completes and **quest 1001
   starts** - this is the confirmed chain, so it costs nothing to reach.
3. Open the **Quest** window, find *"Sera's Mirror"* under the in-progress tab, and press
   **give up**. Confirm the box.
4. Walk back to **Sera** and click them.

| what is on screen | what it means |
|---|---|
| the row **disappears** the moment they confirm, and clicking Sera offers quest 1001 again | **it works.** The reply reached the client and the journal cleared |
| the row disappears, then **comes back after a map change** | the `0x0089` went out but the store still has the row. `world.log` says which: look for `NOT REMOVED` on the reply line (§5.3 note 3) |
| **nothing happens at all**, and `world.log` has `<- 0x0151 ... 5 byte body 03 e9030000` | the packet arrived and the handler did not run. Grep the line after it for whether anything was sent |
| **nothing happens at all**, and `world.log` has **no** 5-byte `0x0151` | the client refused to build it - the quest was not in its started map. Check the `SetField` line's `quests: N started`; if 1001 is not there, the chain in step 2 did not run and this is not a forfeit bug |

The last row is the discriminator that costs nothing, and it is why the test uses quest 1001
rather than 1000: 1001 is the one that is *in progress* at the end of the chain. Quest 1000 is
**completed** by then and its give-up button can never send anything (§2.2) - testing on it
would produce the fourth row and look like a server failure.

Read it with:

```
cd C:\MapleCW
findstr /C:"0x0151" world.log
findstr /C:"quests: " world.log
```

---

## 7. Negatives, each with its blind spot

* **`0x01ED` and `0x01A5` do not carry quest state.** The control is a run with no quest
  system that contains the identical bytes. *Blind spot:* this says the **observed** bodies
  are not quest state; a `0x01ED` kind nobody has captured could be anything.
* **There is no third way to erase a started quest on the client.** `tools/callers.py` reports
  calls, tail `jmp`s and data pointers, and all three are exhausted. *Blind spot:* a call
  through a register loaded from something that is not a static pointer.
* **No `0x0151` tag 6 exists in any capture.** The reason is now positive rather than absent:
  the gate needs `QuestData+0x148` non-empty, that field is `failscript`, and only four quests
  in the client have one - none of them tested. *Blind spot:* the four-quest count comes from
  `gm-handbook/questlines.txt`, a generated dump; if the dump drops a node kind, the count is
  low. The same dump enumerates 20-plus other `Check` keys including all three script names,
  so it is not blind to the shape.
* **The `0x0151` action census is 8 tags over 9 call sites.** *Blind spot, stated in §1.3:*
  13 of 1894 `1406ed520` call sites pass a computed opcode and are unresolved.
* **`resignScript` is not a string in this executable.** `tools/xref.py --string "resignScript"`
  reports **0 copies** in either encoding, while the same tool finds `startscript`,
  `endscript` and `failscript` one copy each - so the search works. `Quest.wz` has exactly one
  `QuestInfo.resignScript` (quest 500009) and the client does not appear to read it by name.
  *Blind spot:* only ASCII and UTF-16LE are searched, and a name assembled at runtime would be
  invisible.
