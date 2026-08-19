# NPC dialogue: what `0x0151` really is, and what makes an NPC speak

**Written 2026-08-19, entirely from the image on disk and the two captures.** No Ghidra
(the project was locked); the working was done with `capstone` against
`client-patched/MapleStory.exe` plus the repo's own `tools/callers.py`, `tools/dataref.py`,
`tools/xref.py` and `tools/rtti.py`. Every address below can be re-derived that way.

Labels: **[L]** read from the listing or a capture, **[D]** derived from two or more [L]
facts, **[I]** inferred, including from the v214 reference (candidate only).

---

## 0. The headline, and one correction to `STATUS.md`

**`0x0151` is not "NPC click". It is the quest request.** Its leading `u8` is a quest
*action code*, and its two `u32`s are **`questId, npcTemplateId`** — not
`objectId, templateId`. [D]

**The `objectId` claim in `STATUS.md` §2 is wrong and should be retracted.** The evidence
against it is in our own logs:

| capture | what the server sent | what the client sent back |
|---|---|---|
| map 1  | `0x044F` objectId **1000**, template 1 | `0x0151` field1 = **1000**, field2 = 1 |
| map 10 | `0x044F` objectId **1000**, template 3 | `0x0151` field1 = **1002**, field2 = 3 |
| map 20 | `0x044F` objectId **1000**, template 4 | `0x0151` field1 = **1003**, field2 = 4 |
| map 30 | `0x044F` objectId 1000 (t6), **1001** (t7) | `0x0151` field1 = **1005**, field2 = 7 |

`crates/world/src/config.rs:130` assigns `object_id = 1000 + list.len()` **per map**, so
every map's first NPC is 1000. The client answered 1000 / 1002 / 1003 / 1005 for four
different NPCs, and the *same* values in both sessions despite opposite visit orders. So
field1 is neither our objectId nor a session counter — it is a value the client holds per
NPC. The listing says what it is: a **quest id**. [L]+[D]

Field2 *does* match the template we sent, in all four captures. [L]

That the numbers happen to land near 1000 is a coincidence of this map's data — our object
ids start at 1000 and this region's quest ids are 1000-1005.

---

## 1. `0x0151` field by field

### 1.1 Where it is built

`research/msexe-send-opcodes.txt` lists four builders. Three are real, one is the one that
matters:

| builder | call sites | verdict |
|---|---|---|
| **`FUN_141f0e4c0`** | 6 | **the one** — six distinct `0x0151` bodies, verified below |
| `FUN_141f18c20` | 1 | a sibling: `u8, u32, u32, string` — the "cannot do that" report |
| `FUN_14289dd90` | 1 | `u8, u32` (5-byte body) |
| `FUN_142d9ad50` | 1 | `u8, u32` (5-byte body) |

**Verified, not taken from the table** [L]: `FUN_141f0e4c0` has six `mov edx, 0x151` /
`call 0x1406ed520` sites at `141f0e6e9`, `141f0e797`, `141f0e829`, `141f0e8cc`,
`141f0ec94`, `141f0ed61`, and each is followed by a matching run of writes. The write
primitives were confirmed from their own code, not assumed:

| primitive | what it does [L] |
|---|---|
| `0x1406ed520(pkt, op)` | `COutPacket` ctor — stores the opcode at `+0x434` and writes it as a `u16` |
| `0x1406ed840(pkt, v)` | write **u8** (`+0x428` += 1) |
| `0x1406ed940(pkt, v)` | write **u16** (`+0x428` += 2) |
| `0x1406ed9d0(pkt, v)` | write **u32** (`+0x428` += 4) |
| `0x1406edc80(pkt, &s)` | write **string**: `u16` length then that many bytes |
| `0x1406ede20(pkt, p, n)` | write **n raw bytes** |
| `0x1415d01c0(pkt)` | send |

### 1.2 The object the builder serialises

`FUN_142d9ac30` (27 call sites, all UI) allocates 0x68 bytes, runs
`FUN_141f0e110(obj, arg2, arg3)`, then `FUN_141f0e4c0(obj, arg4)`. The constructor is
explicit [L]:

```
141f0e149  mov [rcx+0x20], ebp     ; ebp = arg2
141f0e14c  mov [rcx+0x24], r8d     ; r8d = arg3
...
141f0e3af  call 0x14070fae0        ; a lookup on arg2
141f0e3b8  mov dword [rsi+0x28], 2   ; \
141f0e3c1  mov dword [rsi+0x28], 1   ;  > +0x28 = 2, 1 or 0
141f0e3cc  mov dword [rsi+0x28], edi ; /   (edi = 0)
```

So `obj+0x20`, `obj+0x24` and `obj+0x28` are the three things the packet is built from.
`obj+0x28` is a three-valued state derived from `obj+0x20`. **[L]**

`obj+0x20` is the key to six accessors on one global table —
`FUN_140715180`, `FUN_1407152f0`, `FUN_140715460`, `FUN_1407155d0`, `FUN_140715800`,
`FUN_1407158a0` — and to `FUN_140711d70` (a requirement checker returning an error code)
and `FUN_140734b40`. **[L]**

It is a **quest id**, and the proof is two range tests inside the builder plus the string
table it reaches: [L]

```
141f0eb52  mov  ecx, [r14+0x20]
141f0eb56  lea  eax, [rcx - 0x9c40]   ; 40000
141f0eb5c  cmp  eax, 0x3e7            ; .. 40999   -> a flag
141f0eb6a  lea  eax, [rcx - 0x7563]   ; 30051
141f0eb70  cmp  eax, 0x1c             ; .. 30079   -> a flag
141f0eb7c  mov  rdx, [0x143a488f8]    ; -> "ask"
```

`0x143a488f8` is a slot in a `.rdata` string-pointer table whose neighbours are
**`ask`, `stop`, `/1`, `/1/stop`, `/1/stop/%s`, `BtQYes`, `BtQNo`, `BtQStart`, `BtQAfter`,
`#FUI/QuestIcon.img/5/0#`** [L]. Those are the `Quest.wz` node names and the quest-dialog
button names. `FUN_141f18c20`'s row in `msexe-packet-fields.txt` independently carries the
literals `item | quest | level`.

`obj+0x28` is therefore the **quest state**: 0 = not started, 1 = in progress, 2 = complete.
The values 0/1/2 are [L]; the naming is **[I]** from the family.

### 1.3 Where x and y come from

```
141f0e536  mov rcx, [0x143aa8518]
141f0e53d  add rcx, 8
141f0e541  mov rax, [rcx]
141f0e549  call qword [rax+0x30]     ; out-param -> [rsp+0x58], [rsp+0x5c]
```
Two `LONG`s, written to the packet as `u16`s (`movzx edx, word ptr [rsp+0x58]`). **[L]**

They are the **character's own position**, not the NPC's: the pair `(-256, 215)` in the
`0x0151` at 16:26:02 appears verbatim inside the `0x00D9` movement packet 6 seconds earlier
in the same log (`...00ffd70000...`). **[D]**

### 1.4 The six bodies

`s` = quest state (`obj+0x28`), `q` = `obj+0x20`, `n` = `obj+0x24`, `err` = `FUN_140711d70(...)`.

| # | site | tag | gate [L] | body after `u8 tag` | bytes |
|---|---|---|---|---|---|
| A | `141f0e6e9` | **6** | `err != 0` && (`s == 1` \|\| `err == 0xC`) && `FUN_140715460(q)` | `u32 q, u32 n, u16 x, u16 y` | 13 |
| B | `141f0e797` | **2** | `err == 0` && `s == 1` && `FUN_1407158a0(q)` | `u32 q, u32 n, u32 0xFFFFFFFF` | 13 |
| C | `141f0e829` | **4** | `s == 0` && `FUN_140715180(q)` | `u32 q, u32 n, u16 x, u16 y` | 13 |
| D | `141f0e8cc` | **5** | `s == 1` && `FUN_1407152f0(q)` | `u32 q, u32 n, u16 x, u16 y` | 13 |
| E | `141f0ec94` | **2** | `s == 1`, main path; skipped if `FUN_141f15fb0` returns `0x7FFFFFFF` | `u32 q, u32 n,` **`[u16 x, u16 y` only if `FUN_1407155d0(q) == 0`]**`, u32 sel` | 17 or 13 |
| F | `141f0ed61` | **1** | `s != 1`, main path | `u32 q, u32 n,` **`[u16 x, u16 y` only if `FUN_1407155d0(q) == 0`]**`, u32 sel` | 17 or 13 |

**So the length varies for two independent reasons** [L]:

1. the leading `u8` selects one of six entirely different bodies; and
2. inside tags **1** and **2**, the `x,y` pair is written only when
   `FUN_1407155d0(questId)` returns zero — a property of the *quest*, not of the click.
   That is the only place a 17-byte body becomes 13.

`sel` is the trailing `u32`: `FUN_141f13360` / `FUN_141f146a0` for tag 1, `FUN_141f15fb0`
for tag 2. It was **0** in every capture — nothing was chosen, because no dialog was open.

### 1.5 The two captured packets, decoded

```
01 e8030000 01000000 0c04 6d01 00000000        (17)   npcs-visible..., 16:27:21
^  ^        ^        ^    ^    ^
|  q=1000   n=1      x=1036 y=365  sel=0
tag 1  = quest 1000 is NOT in progress and has no start script -> "start quest"

04 ea030000 03000000 8402 d700                 (13)   npcs-visible..., 16:26:53
^  ^        ^        ^    ^
|  q=1002   n=3      x=644 y=215
tag 4  = quest 1002 is not started AND has a start script -> "run the start script"
```

Both the other captures (`01 eb030000 04000000`, `01 ed030000 07000000`) are tag 1 on
quests 1003 and 1005. So of map 1/10/20/30's tutorial NPCs, only map 10's quest carries a
start script. **[D]**

### 1.6 What the action numbers mean

The gate structure alone forces most of it. The names are **[I]**, from the v214/classic
`QuestRequest` enum, and the *ordering* is [L]:

| tag | forced by the listing | candidate name |
|---|---|---|
| 1 | state != in-progress, no start script | `AcceptQuest` / start |
| 2 | state == in-progress | `CompleteQuest`, `sel` = the reward chosen; the early form sends `-1` = "no choice" |
| 4 | state == not-started **and** the quest has a start script | `OpeningScript` |
| 5 | state == in-progress **and** the quest has an end script | `CompleteScript` |
| 6 | a requirement check failed (`err != 0`) | a refusal / lost-item report |

Do not read more into the names than that. What is *measured* is the gate.

---

## 2. The reply: inbound **`0x055B`**, the script message

### 2.1 How it was found, and one thing that had to be ruled out first

The brief suggested looking for an inbound handler shaped `u8, u32, u8, string`. I built a
read-shape extractor over **every** function in the image that calls one of the seven
documented read primitives (2159 of them) and searched. **Exactly one match exists:
`FUN_142dd4f10`, the channel dispatcher's `case 0x0194`.** Reading it, it is not the script
message — it compares the `u32` against `FUN_142cb9550(this)` (our own character id), skips
if equal, and routes the string to `FUN_142d05490(this, 1, otherId, &msg)`. That is a
name-addressed **message/whisper**, not NPC dialogue.

*Instrument check before believing that negative*: the same extractor reproduces
`0x142097f80` (`SetField`) as `raw8,u32,u8,u32,u8,u32,u32,u32,u8,u16,str,str,...`, which
matches the 33-byte head in `research/msexe-stage-setfield.md`, and `0x140304b20` as
`raw,u8,u32,u8,u32,u32,...`, which matches `research/charrecord-loops.md`'s minimum record.
It finds positives.

The real route was the UI. `tools/xref.py --string BtNext` reports **0 code references** —
the documented `lea`-only blind spot — but the strings are reached through a `.rdata`
pointer table, and `tools/dataref.py` on the table slots (`0x143a48410/18/20/28`) names the
cluster `0x142a65740 … 0x142a6cc10`. `FUN_142a65740` loads
**`UI/UtilDlgEx.img/UtilDlgEx`** with `BtPrev / BtNext / BtOK / BtClose` — the NPC script
dialog. [L]

Walking up: `FUN_142a58880` is that class's `CreateLayout`, a 30-way switch on
`[this+0x2a8]`; its ctor is `FUN_142a57d30` (vtable `0x143489e08`); and 39 of the ctor's 110
callers also read packets. **Every one of them in the `0x141f6f…0x141f76` band is a script
message type handler.** [L]

### 2.2 The routing

`CField::OnPacket` = `FUN_141820080`, the same dispatcher that already delivers our working
`0x044F`: [L]

```
141821f84  lea  eax, [r9 - 0x55b]
141821f8b  cmp  eax, 1
141821f8e  ja   ...
141821f93  mov  rcx, [0x143acedb0]        ; the script-manager singleton
141821f9a  call 0x141f6f320
```

```c
FUN_141f6f320(scriptMan, opcode, packet) {
    if (opcode == 0x55B) return FUN_141f6f350(scriptMan, packet);   // ScriptMessage
    if (opcode == 0x55C) return FUN_1412808a0(scriptMan, packet);
    return;
}
```

**There is no null check on `[0x143acedb0]` at the call site** [L]. It is non-null on a live
field, because `FUN_142caa4e0` — the field-enter routine that builds the `0x0238`/`0x024D`
pair the client sends ~420 ms after every `SetField` — dereferences it unguarded at
`142caac73` and calls `FUN_141f6f200(it)`, the script-manager **reset**. So:

* the singleton is alive once the character is on a map **[D]**; and
* **the script state is torn down on every field entry** — a `0x055B` sent before or during
  a `SetField` is destroyed by the reset **[L]**.

### 2.3 `0x055B` head, field by field

`FUN_141f6f350(this, CInPacket*)`, read in this order [L]:

| # | prim | at | goes to | meaning |
|---|---|---|---|---|
| 1 | `u32` | `141f6f382` | `r15d` | **[?] a handle.** The client echoes it verbatim as the first `u32` of its `0x00F3` answer; every other message type writes `0` there instead. Meaning **not established** — treat it as a correlation token you choose. Passed only to types 0 and 0x42..0x46. |
| 2 | `u8` | `141f6f38d` | `ebp` | passed to the type handler and **never read** on the Say or type-1 paths [L] |
| 3 | `u32` | `141f6f398` | `esi` | **speaker NPC template id** — see below |
| 4 | `u8` | `141f6f3a2` | — | presence flag for field 5 |
| 5 | `u32` | `141f6f3b1` | `r14d` | **only if #4 != 0.** If it is `> 0` *and* a global gate holds (`FUN_142cc1e30`), it **replaces** field 3 (`cmovg esi, r14d`) |
| 6 | `u8` | `141f6f3e6` | `r12d` | **message type**, `0..0x46`, 71-entry jump table at `0x141f6f9f4` |
| 7 | `u16` | `141f6f3f2` | `r14d` | **flags**: `0x04` = an extra speaker `u32` in the body; `0x20`/`0x40` pick the dialog style; `0x06` sets `[ui+0x6b8]` |
| 8 | `u8` | `141f6f3fe` | `edx` | stored at `[ui+0x2a4]` by the ctor |

**Field 3 is the speaker template id — read, not inferred.** `FUN_141f6fb20` hands it to
`FUN_142a61900(ui, msgType, speakerId, &text)`, which does
`mov [rcx+0x2cc], r8d` (`142a61997`); and `[ui+0x2cc]` is passed straight into
**`FUN_141e77b70`** at `142a7b50d` — the very function `research/npc-spawn.md` identified as
the **NPC template loader** (`0x0467`'s preload list feeds it). The same UI cluster
(`FUN_142a55af0`, via `FUN_142a54340`) formats **`Npc/%07d`**. **[L]+[D]**

There is also a re-entrancy latch, and it is **not** a session lock [L]:

```
141f6f406  cmp  [rdi+8], 0
141f6f40a  je   141f6f490          ; free -> proceed
141f6f410  cmp  r12d, 0x47         ; busy: only type 0x47 is accepted
...
141f6f490  mov  dword [rdi+8], 1   ; take it
...
141f6f9c2  mov  dword [rdi+8], 0   ; common exit: release it
```

It is set and cleared inside one dispatch, so **the server needs no lock and no
acknowledgement before sending another `0x055B`.**

### 2.4 Message type 0 = Say, and its body

Type 0 → `FUN_141f6fb20(this, f1, f2, f3, packet, flags, f8)`. It reads [L]:

| # | prim | at | meaning |
|---|---|---|---|
| 9 | `u32` | `141f6fb6b` | echoed as the third field of the `0x00F3` answer |
| 10 | `u32` | `141f6fb83` | **only if `flags & 0x04`** — overrides the speaker id |
| 11 | `str` | `141f6fb93` | **the text.** `u16` length, then that many **bytes** (`FUN_1406e9050`, `research/msexe-packet-readers.c:80`). The `0x00E7` chat capture confirms the wire form: `05 00 "Hello"` |
| 12 | `u8` | `141f6fb9c` | → `[ui+0x2f8]` — **prev** button |
| 13 | `u8` | `141f6fba8` | → `[ui+0x2fc]` — **next** button |
| 14 | `u32` | `141f6fbb7` | → the `CUIScriptMsg` ctor's 4th argument |

then, in order [L]:

```
ebx = (flags & 0x20) ? 1 : ((flags >> 6) & 2)
obj = alloc(0x7d0)
FUN_142a57d30(obj, ebx, f8, #14)      ; [ui+0x2a0]=ebx  [ui+0x2a4]=f8
FUN_141280700(this+0x10, obj)         ; register it
FUN_142a61900(ui, 0, speaker, &text)  ; [ui+0x2a8]=0 (Say)  [ui+0x2cc]=speaker
FUN_142a62ba0(ui, flags)              ; [ui+0x6b4]=flags
FUN_142a62bf0(ui, prev, next)         ; [ui+0x2f8], [ui+0x2fc]
FUN_142a5ee30(ui)
r = ui->vtbl[0x130](ui)               ; show
if (r != 3) { <answer 0x00F3 immediately> }
```

`[ui+0x2a8] = 0` is what makes `CreateLayout` pick `FUN_142a65740`, i.e.
`UI/UtilDlgEx.img/UtilDlgEx`. **[D]**

The other 70 types are in the same table; 26 of them jump straight to the common exit and do
nothing. Types worth knowing: `1 → FUN_141f6fe40` (`str, u8, u8, u32` — an ask-style
dialog), `2 → FUN_141f70110`, `3 → FUN_141f70820`, `4 → FUN_141f70df0`,
`0x47` = **force-close the open dialog** (`u8 result`, then `FUN_1410dea80` on the global
UI at `0x143aca0f0`) — the only type accepted while the latch is held.

---

## 3. The minimum viable "an NPC says one line", byte by byte

Send **one** packet on the channel connection, after the character is on a map and after the
NPCs have been sent (i.e. anywhere the existing `0x044F` batch already works):

```text
opcode  0x055B                                      (u16 LE on the wire: 5B 05)

body (32 bytes for the text "Hello.")

  00 00 00 00   u32  handle          echoed back in 0x00F3; any value, 0 is fine
  00            u8   -               discarded on the Say path
  01 00 00 00   u32  speakerTemplate 1  = map 1's Heena. MUST be a real Npc.wz template
  00            u8   hasOverride     0 -> no extra u32 follows
  00            u8   messageType     0 = Say
  00 00         u16  flags           0. bit 0x04 would add a speaker u32 to the body
  00            u8   -               -> [ui+0x2a4]
  -- say payload --
  00 00 00 00   u32  echo            echoed back in 0x00F3
  06 00         u16  length          6
  48 65 6c 6c
  6f 2e         str  "Hello."        length bytes, not NUL-terminated, not UTF-16
  00            u8   prev            0 = no Prev button
  00            u8   next            0 = no Next button  (1 gives "Next")
  00 00 00 00   u32  -               -> the CUIScriptMsg ctor
```

flat: `00000000 00 01000000 00 00 0000 00 00000000 0600 48656c6c6f2e 00 00 00000000`

**Values that matter, because the NPC-spawn bug was two zero fields with a perfect layout:**

* **`speakerTemplate` (offset 5) must be a template that exists.** It goes straight into
  `FUN_141e77b70`, the NPC template loader. `0` is not a template. Use the same number
  `gm-handbook/npcs.txt` gave the NPC the player clicked — the *template*, not our objectId.
* `messageType` must be `0`. Anything in the 26 dead slots produces a silent no-op.
* `hasOverride` must be `0` unless you actually append the extra `u32`, and `flags & 0x04`
  must be `0` for the same reason — **both change the body length**, and there is no
  resynchronisation point.
* The text length is a `u16` **byte** count. A zero-length string will render an empty box.
* `prev`/`next` are cosmetic; both zero is valid (an OK-only box).

There is no reason to answer `0x0151` itself first. It is **not a blocking request** —
measured: three of them went unanswered across two sessions with no freeze, and the client
kept sending movement.

---

## 4. Ordering, locks, and what ends the conversation

| question | answer | evidence |
|---|---|---|
| Does anything have to arrive before `0x055B`? | Only that the field exists. `0x055B` rides the same `CField::OnPacket` range chain as `0x044F`, which already works. | [L] `141821f84` |
| Is there a lock the server must respect? | **No.** `[scriptMan+8]` is taken at `141f6f490` and released at the common exit `141f6f9c2` of the same call. | [L] |
| Does the client acknowledge? | **Yes — outbound `0x00F3`**, shape `u32 handle, u8 messageType, <per type>`. For Say the client's own immediate-answer path writes `u32 handle, u8 0, u32 echo, str text, u8 result` (`141f6fd33..141f6fd9e`). Answering it is optional in the same sense `0x0151` is. | [L] |
| What ends it? | The user clicking OK/Close; the client sends `0x00F3` and drops the dialog. The server can force it with message type **`0x47`** (`u8 result`), the only type accepted while the latch is held. | [L] |
| Anything that destroys it? | **Field entry.** `FUN_142caa4e0` calls `FUN_141f6f200(scriptMan)`, which zeroes the latch and releases the UI objects at `+0x18`/`+0x28`. So never send a script with, or just before, a `SetField`. | [L] |

---

## 5. Gaps — stated rather than guessed

* **Head field 1 (the handle) and field 2 are not identified.** Field 2 is provably unused on
  the Say and type-1 paths. Field 1 is only ever echoed. A natural reading is
  `field1 = NPC object id, field2 = speaker type flag` — that is **[I]** and nothing in the
  listing supports it. Since field 1 comes straight back in `0x00F3`, the server can define
  it however it likes.
* **No quest-result / quest-record reply was identified.** Answering `0x0151` so that a
  quest actually *advances* (state 0 → 1) needs a packet that writes the client's quest
  record, and I did not isolate it. Reachability from the case tables into the quest table
  (`FUN_140711d70`, `FUN_14070fae0`) is too diffuse to name one. `0x00FA`, `0x00FC` and
  `0x00AC` are the only inbound cases that reach the *quest request builder* itself
  (`FUN_141f0e110`/`FUN_141f0e4c0`), so one of them is "server asks the client to run a
  quest action" — that is a lead, not a finding.
* **The 71 message types are not enumerated**, only the table location
  (`0x141f6f9f4`, 71 entries) and types 0, 1, 2, 3, 4 and 0x47.
* **The dialog-driven `0x00F3`** (as opposed to the client's immediate-answer path) was not
  traced to a single builder; `FUN_1410de8c0` is the closest candidate and writes
  `u32 [ui+0x2c8], u8 <derived>, u8 result`, whose first field is a *different* member from
  the one the Say handler echoes.
* **A null `[0x143acedb0]` would fault** at `141f6f406` (`cmp [rdi+8], 0`) with no null check
  at the call site. The argument that it is non-null is an unguarded dereference on the
  field-entry path, which is strong but is not a construction trace.

## 6. If a fresh decompilation is worth spending

Two functions would close the biggest gap, and both are large enough that reading them off
the raw listing by hand is slow:

* **`FUN_142d60d40`** — the channel dispatcher's `case 0x00AC`, 8420 bytes, `u8` then a
  30-way switch, and it reaches the quest-request builder. Likely the packet that *opens* a
  quest interaction from the server side.
* **`FUN_142d43ee0`** — `case 0x0089`, 3128 bytes, `u8` then a 36-entry jump table, also
  reaching the quest builder. One of these two is very likely where "quest accepted"
  arrives.
