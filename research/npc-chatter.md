# NPC idle chatter: the balloon, who chooses the line, and the packet that carries it

**Written 2026-08-19. No Ghidra** (two other sessions hold or may take the project lock) and
**no client run.** Everything below is `capstone` against `client-patched\MapleStory.exe`,
the repo's `tools/callers.py`, `tools/dataref.py`, `tools/xref.py`, `tools/pdata_lookup.py`,
`tools/dump_va.py`, the repo's own `wz-dump`, the logs in `research/fixtures/`, and the
documents already in `research/`. Every address can be re-derived that way.

Labels: **[L]** read out of the listing, the WZ, or a capture. **[D]** derived from two or
more [L] facts. **[I]** inferred / candidate. Nothing here comes from the v214 tree; the
structure it would have predicted is noted in §9 only as after-the-fact agreement.

> Companions: `research/npc-spawn.md` (`0x044F`), `research/npc-click.md` (`0x00F2`),
> `research/npc-dialogue.md` (`0x0151`, `0x055B`). One row of `npc-spawn.md` is corrected in
> §8.2; nothing else is retracted.

---

## 0. Answer up front

| question | answer |
|---|---|
| Is the balloon client-side or server-sent? | **Both halves are real, and the split is the whole story.** The **text** is client-side: the client already has every line of `String.wz/Npc.img` in memory. The **trigger** is a packet: the balloon is drawn *only* from inbound **`0x0453`**, and the client cannot draw one on its own. [L] |
| So is the owner's belief right? | **Half right, and the useful half.** "The client already knows how to display it" — yes, completely: it holds the strings, the conditions, the balloon art and a 5000 ms display. "The server just needs to send the proper packet" — yes, and the packet is `0x0453`, **not** a field of `0x044F`. No field of the `0x044F` body turns chatter on. [D] |
| the opcode | **inbound `0x0453`** → `FUN_141e421f0`. Body after the pool's `u32 objectId`: **`i8 nAction, i8 nChatIdx, u32 unused`** — 6 bytes, 10 in total. [L] |
| Does the client ask first? | **Yes — outbound `0x0327`**, `u32 objectId, u8 nAction, u8 nChatIdx, u32`. The client picks a random emotion and a random line every 3–9 s and *asks*. **`0x0327` has never appeared in any of our captures**, and why is the one thing §7 could not settle. [L] |
| Where is the cooldown? | In the binary: `FUN_141e46d40` sets `[npc+0x230] = rand() % 6000 + 3000` ms. The client-side interval is **3000–8999 ms, uniform**. [L] |
| Is the client's order sequential? | **No — it is random**, twice over (`rand()%n` for the group, `rand()%n` for the line). There is no ordering state anywhere. [L] |
| Can the server get "in order"? | **Yes, and it is the server's to choose.** `nChatIdx` is an index the *server* supplies; the client obeys it. Sequential order is a server-side counter. [D] |
| Does the client expand `#p8#`? | **On the click-dialogue path, yes** — `0x055B`'s text setter reaches the `#`-token expander. **On the balloon path, unverified.** §6. |
| Are the `n`/`f`/`w`/`h` prefixes a flat list? | **No.** They are four *separate* `speak` arrays on four different WZ animation nodes, and each carries its own **condition** block. The fan site's flat ordering is a concatenation artefact. Corrected from the client's own WZ in §1. [L] |

---

## 1. The data, corrected from the WZ rather than the fan site

The brief carried the grouping as [I]. It is now [L]. `wz-dump cat Npc_000.wz 0000008.img`,
flattened (canvases elided):

```
/info/dcMark        = 1
/info/speak/0..3    = 'n0' 'n1' 'n2' 'n3'
/info/script/0/script = 'npc_8'
/finger/speak/0..2  = 'f0' 'f1' 'f2'
/wink/speak/0..1    = 'w0' 'w1'
/heart/speak/0      = 'h0'
```

So the prefixes are **not** a naming convention inside one list. Each is the `speak` array of
a different **animation node**: `info` (the default/stand state), `finger`, `wink`, `heart`.
`d0`/`d1` appear in no `speak` array at all, which is exactly why they are the click
dialogue and not chatter. The fan site's `n0 n1 n2 n3 f0 f1 f2 w0 w1 h0` is the four arrays
concatenated in WZ order — a listing order, not a play order. [L]

### 1.1 Where the client puts them

`FUN_141e79060` (0x141e79060..0x141e826ca, 38506 bytes) is the NPC template parser — it is
the function that reaches `speak`, `/speak`, `imitateFace`, `special`, `hideName`,
`moveAbility`, `Npc/%d.img/info/button/%d` and ~120 other WZ node names. [L] It produces:

| template member | contents | how it is filled |
|---|---|---|
| `template+0xE8` | `ZArray<ZXString>` — the **resolved text** of `info/speak` (n0…n3 for Robin) | loop at `0x141e7b1a0`: for each `info/speak/i`, `FUN_142a08a10(&out, "String/Npc.img", npcId, key)`, then `FUN_140814ce0(template+0xe8, i)` [L] |
| `template+0xF0` | the parallel **condition** array for those lines | same block |
| `template+0x108` | `ZArray` of **0x38-byte speak groups**, one per animation node | `FUN_141e8be50(template+0x108, i)` at `0x141e7fb2e` (element size 0x38 proved by `imul rcx, rax, 0x38` at `0x141e5b1c5`) [L] |
| `template+0x110` | `int[]` — group index → **animation/action index** | `FUN_141e8d000(template+0x110, &idx)` at `0x141e7fc1a` [L] |
| `template+0x128` | count of groups marked `special` in the WZ; these are **excluded** from the client's idle pick and get no `+0x110` entry | `inc [template+0x128]` at `0x141e7ff9f`, `add [template+0x118], -4` at `0x141e7ffaf` [L] |

The 0x38-byte speak group:

```
+0x00  ZXString  name          the animation node name ("finger", "wink", "heart", …)
+0x08  ZArray<ZXString>  lines the resolved <anim>/speak texts
+0x18  u8        imitateFace   from the WZ `imitateFace` (0x141e7fd11)
+0x20  condition block         built by FUN_141e826d0 (job / Quest / condition%d /
                               citizenshipTown / citizenshipGrade / dateStart / dateEnd /
                               dayOfWeek / state / key / value)
```
All [L]. **This answers brief question 3 directly: the groups are condition-gated, not a
flat list.** `FUN_141e826d0` is the condition parser and `FUN_141e83b30` / `FUN_141e78630`
evaluate them at display time against the character record and the NPC's position.

---

## 2. The one function that draws a balloon

`FUN_141e3b510(npc, ZXString* line)` — 0x141e3b510..0x141e3c5c4, 4276 bytes. It is the
balloon. Evidence: [L]

* three creation sites (`0x141e3c2a2`, `0x141e3c48e`, `0x141e3c516`) all call
  `FUN_14158f5c0`, and the first WZ path `FUN_14158f5c0` loads is
  **`UI/ChatBalloon.img/`** (`0x14158f776`);
* all three pass the same two constants as the 5th and 6th arguments:
  **`0x1388` = 5000** and `0x3E9` = 1001;
* it reaches `/name`, `/town`, `/grade`, `/imitatedjob` via `FUN_142ef6a98` (a substring
  search), i.e. it does its own small substitution pass on the line.

That 5000 is **[L] as a constant, [I] as "the balloon's on-screen lifetime in ms"** — I did
not trace the parameter inside `FUN_14158f5c0` (the argument lands at `rbp+0x4c0` in that
frame and I found no direct read there). It is the only time-shaped number on the path.

### 2.1 Its entry guards, and why they are all already satisfied

```
141e3b543  if ([npc+0x270] == 0)                     return
141e3b54f  if ([npc+0x294] != 0)                     return
141e3b55b  if ([npc+0x248] && FUN_140d5e740(...))    return    ; a balloon is already up
141e3b574  if ([npc+0x288] == -2)                    return    ; animation not initialised yet
```
[L]. `[npc+0x270]` is set to **1 by the CNpc constructor** (`0x141e360a6`,
`mov dword [rsi+0x270], 1`) [L]; `[npc+0x294]` is the field `0x044F` read 21 keys off and we
send 0; `[npc+0x288]` starts at `-2` and the per-frame update fixes it at `0x141e3e824`. So
nothing we currently send blocks the balloon.

### 2.2 `FUN_141e3b510` has exactly two callers, and both are inside the `0x0453` handler

`tools/callers.py 0x141e3b510` → **2 call sites in 1 function**: `0x141e42516` and
`0x141e42eff`, both in `FUN_141e421f0`. [L]
`pe ptr-slot scan` → **0 vtable slots**, so there is no indirect route either. [L]

**That is the decisive negative.** No timer, no click handler, no field loader, nothing in
the client can put a chat balloon over an NPC except the handler for inbound `0x0453`.
Instrument control: the same `callers.py` reports 3 sites for `FUN_141e5af80` and 24 for
`FUN_14158f5c0` in this session, and the same ptr-slot scan finds `FUN_141e3fb60`'s vtable
slot at `0x1434138b0` — so both instruments speak. [L]

---

## 3. Inbound `0x0453`, field by field

Routed by `CField::OnPacket` → the NPC pool `FUN_141e75800` (`0x44F..0x468` block) →
`FUN_141e75aa0`, which reads the **`u32 objectId`** itself, resolves the CNpc through the
pool hash, and jumps through a 20-entry table at `0x141e75cb8`. I recovered that table by
value: [L]

| opcode | handler | reads (address order) |
|---|---|---|
| **`0x0453`** | **`FUN_141e421f0`** | **`u8, u8, u32`** |
| `0x0454` | `FUN_141e433a0` | `u8` |
| `0x0455` | `FUN_141e43bb0` | — |
| `0x0456` | `FUN_141e43bd0` | — (32-byte stub) |
| `0x0457` | `FUN_141e43c50` | — (29-byte stub) |
| `0x0458` | `FUN_141e43cd0` | `u32` |
| `0x0459` | `FUN_141e43d40` | `u32, u32, u8` |
| `0x045A` | `FUN_141e43e10` | `str, u32, u8` |
| `0x045B` | `FUN_141e44a00` | `u8, u8` |
| `0x045C` | `FUN_141e44a60` | `u32, u32` |
| `0x045D` | `FUN_141e44ab0` | `u8, u32` |
| `0x045E` | `FUN_141e44af0` | — |
| `0x045F` | `FUN_141e432c0` | — |
| `0x0460` | `FUN_141e44b10` | `str` |
| `0x0461` | `FUN_141e44d00` | `u32, u32` |
| `0x0462` | `FUN_141e44d50` | `u32, u32` |
| `0x0463` | `FUN_141e44dc0` | `u32, u32` |
| `0x0464` | `FUN_141e44e10` | `str, u32, u32, u32, u32` |
| `0x0465` | `FUN_141e430c0` | `u8, u32, {str, u32}×n` |
| `0x0466` | `FUN_141e42ff0` | `str, u32, u8, u8` |

`FUN_141e421f0` reads exactly three fields — verified mechanically by scanning its whole
`.pdata` range for `call rel32` to the five documented read primitives
(`research/msexe-packet-readers.c`): three hits, at `0x141e42243`, `0x141e4224e`,
`0x141e4225f`. [L]

### 3.1 The body

```
opcode 0x0453
  u32  objectId     read by the pool (FUN_141e75aa0); must already be in the NPC pool
  i8   nAction      movsx at 141e42248 -> SIGNED
  i8   nChatIdx     movsx at 141e42253 -> SIGNED
  u32  (unused on both chat paths)
```
6 bytes of handler body, 10 including the pool's object id. [L]

### 3.2 What the two fields mean, read off the branches

```
141e4221b  if ([npc+0x1a8] != 0) {              ; a "special action" is running
141e42224      if ([npc+0x170] == 0) return
141e42234      [npc+0x170] = 0 ; return         ; <- this packet ONLY clears the request latch
           }
141e4226a  if (nAction == -1) goto 141e42e6a    ; chat only, no animation change
141e42275  if (nAction <  0)  return
141e42294  if ((u32)(nAction-3) >= groupCount) return
141e422e1  [npc+0x1a4] = nAction                ; switch to that animation
```
[L]

* **`nAction == -1`** → `0x141e42e6a`: `FUN_141e842c0(template, &lines, charData, npcPos,
  &nChatIdx)` builds the **condition-filtered** list from `template+0xE8`/`+0xF0`, then
  `lines[nChatIdx]` → `FUN_141e3b510`. This is the `info/speak` (`n0…n3`) path. Bounds are
  checked at `0x141e42edc`; an out-of-range index silently draws nothing. [L]
* **`nAction >= 3`** → group `nAction - 3` of `template+0x108`; its own line array is
  condition-filtered by `FUN_141e83d60`, and `nChatIdx` indexes that. [L]
* `nChatIdx < 0` → no balloon, animation change only (`test edi,edi; js` at `0x141e42ecf`).
* **`nChatIdx` indexes the *filtered* list, not the raw WZ array.** `FUN_141e842c0` /
  `FUN_141e78630` take `&nChatIdx` by pointer and may rewrite it. With no conditions in the
  WZ (Robin has none) filtered == raw, but that equality is an assumption for any NPC that
  does have them. [D]

**So `nAction = -1, nChatIdx = k` is a pure "say line k", and it is the smallest possible
experiment.** The `0x044F` bodies already on the wire need no change at all.

### 3.3 The neighbours, so nobody sends the wrong one

`0x0465` and `0x0466` are the **special-action** pair, not chatter. Both end at
`FUN_141e5af80(npc, ZXString* groupName, u32 durationMs, u8 flag)`, which looks a speak
group up **by name** through `template+0x108` (loop at `0x141e5afd0..0x141e5b0db`, stride
0x38, name compared with `FUN_142f100b0`), sets `[npc+0x1a4] = groupIndex + 3`,
`[npc+0x1a8] = 1`, and — if the duration is non-zero — `[npc+0x428] = 1` with an expiry tick
at `[npc+0x42c] = now + duration` (`0x141e5b33a`). [L]

`0x0465` is the list form: `u8 flag, u32 count, then count × {str name, u32 durationMs}`,
pushed onto a `std::list` at `[npc+0x1d0]` (size at `[npc+0x1d8]`, entries
`{+0x10 name, +0x18 duration, +0x1c flag}`), and the per-frame update pops one at a time
(`FUN_141e3fb60` at `0x141e3fd39`, pop at `FUN_141e6f900`). [L] It **drains** — `FUN_141e6f900`
is pop-front-and-free, nothing re-queues — so a list plays once and stops.

**These do not draw a balloon.** `FUN_141e5af80` never calls `FUN_141e3b510`. And worse, once
`[npc+0x1a8]` is 1 the `0x0453` handler takes the early-out above and **swallows the chat**.
So: do not mix `0x0465`/`0x0466` with `0x0453` chatter.

`0x0460` (`str`) and the **trailing string of `0x044F`** (npc-spawn.md read 22) are the same
thing: both build a rendered-text object with `FUN_140dc12e0` and hand it to
`FUN_141e5b5e0`, which stores it at `[npc+0x4f0]` (call sites `0x141e38dfe` in the `0x044F`
decoder and `0x141e44c91`/`0x141e44cb6` in the `0x0460` handler). [L] That is a *persistent*
per-NPC text field, not the balloon — `FUN_141e5b5e0` never reaches `FUN_141e3b510` or
`UI/ChatBalloon.img`. Worth knowing because it is the one `0x044F` field that looked like a
chatter switch and is not.

---

## 4. The client half: it picks, and it asks

There **is** a client-side idle-chatter driver. It just does not draw anything; it sends a
request and waits.

`FUN_141e3e220` (0x141e3e220..0x141e3fb57) is the CNpc per-frame update — no packet reads,
two `GetTick` calls, in two vtables (`0x143413860`, `0x143413970`). [L] Its tail:

```
141e3e773  if ([npc+0x230] > 0) [npc+0x230] -= 30      ; <- the cooldown, in ms
141e3e786  if ([npc+0x170] != 0) goto done             ; <- request outstanding: STOP
141e3e78f  if ([template+0x28] && [npc+0x1ac] && r12==-1)
141e3e7bb      FUN_141e46d90(npc, &nAction, &nChatIdx)      ; branch A, result ignored
           else
141e3e7d3      if (!FUN_141e46d90(npc, &nAction, &nChatIdx)) goto done   ; branch B
141e3e7ef  FUN_141e4b620(npc, nAction, nChatIdx, 0)    ; -> sends 0x0327
```
All [L].

### 4.1 The picker, `FUN_141e46d90`

```
if (animation on [npc+0x168] still running)      -> 0
if ([npc+0x230] > 0)                             -> 0        ; cooldown
if (FUN_14158f2f0([npc+0x40]))                   -> 0
if (npc is in a special action)                  -> 0
FUN_141e46d40(npc)                                            ; re-arm the cooldown
esi = count(template+0x108) - [template+0x128]                ; selectable emotion groups
eax = count(template+0xE8)                                    ; info/speak lines
ebp = esi + eax ; if (ebp == 0) -> 0
r   = FUN_1407386b0(&DAT_143ac1ab0)              ; the RNG
idx = (r % 50) % ebp                                          ; 141e46e8b..141e46ea4
if (idx < esi) {                                              ; an emotion group
    action = template[0x110][idx] + 3
    if (action == [template+0x64]) -> the else branch         ; the shop action is skipped
    lines  = group(idx).lines
    chat   = (rand() % 50) % count(lines)                     ; 141e46ef1..141e46f0f
} else {                                                      ; an info/speak line
    action = -1
    chat   = idx - esi
}
```
All [L].

**Two `rand()`s, no counter, no cursor.** The client's own ordering is random. The `% 50`
before the `% n` is verbatim in the listing (`mul 0x51EB851F; shr 4; imul 0x32; sub`) — a
`% 50` — so the RNG's usable range here is 0..49, which also means an NPC with more than 50
choices would never reach the tail ones. [L]

### 4.2 The cooldown, read directly

```
FUN_141e46d40(npc):
  141e46d50  r = FUN_1407386b0(&DAT_143ac1ab0)
  141e46d5d  r %= 0x1770                       ; 6000
  141e46d6d  r += 0xBB8                        ; 3000
  141e46d74  [npc+0x230] = r
```
[L] — **3000 to 8999**, uniform, re-rolled on every pick.

**The units are milliseconds, and that is derived rather than assumed.** `[npc+0x230]` is
decremented by `0x1E` = 30 per update (`0x141e3e77d`), and the *same* constant decrements
`[npc+0x1b8]` (`0x141e3fc75`), which is loaded from the WZ `delay` of an animation frame —
Robin's `stand/0` carries `delay: 5000`. WZ delays are milliseconds, so one update step is
30 ms and `[npc+0x230]` is a millisecond countdown. [D]

`[npc+0x230]` is not written by the CNpc constructor, so it starts at 0 from the zeroed
allocation and the **first** attempt happens on the first update after spawn. [D]

### 4.3 What it sends: outbound `0x0327`

`FUN_141e4b620(npc, nAction, nChatIdx, arg4)`:

```
141e4b70f  mov  edx, 0x327
141e4b719  call 0x1406ed520          ; COutPacket(0x0327)
141e4b71f  mov  edx, [rdi+0x190]     ; the NPC's objectId (the field 0x044F wrote)
141e4b72a  call 0x1406ed9d0          ; u32
141e4b72f  movzx edx, r14b           ; nAction
141e4b738  call 0x1406ed840          ; u8
141e4b73d  movzx edx, r15b           ; nChatIdx
141e4b746  call 0x1406ed840          ; u8
141e4b74b  mov  edx, r12d            ; arg4 (0 from the idle path; a duration from FUN_141e5af80)
141e4b753  call 0x1406ed9d0          ; u32
141e4b798  call 0x1415d01c0          ; send
141e4b79e  mov  dword [rdi+0x170], 1 ; <- LATCH
```
[L]. `research/msexe-send-opcodes.txt` independently lists `0x0327` for `FUN_141e4b620` at
`141e4b719` — a second instrument agreeing. [L]

**`[npc+0x170] = 1` is an "always answer" latch of exactly the kind
`npc-click.md` §4.3 warned about.** It is cleared in one place on this path: the `0x0453`
handler's early-out at `0x141e42234`. Until the server answers, that NPC never asks again.
The `0x00F2`/`0x0151` precedent does not apply here — those were measured to be non-blocking;
this one provably latches. [L]

---

## 5. What the server should send

### 5.1 The minimum experiment — one packet, no change to `0x044F`

After the NPCs are on the field (i.e. anywhere the existing `0x044F` batch already works —
`npc-spawn.md` §6.2's `0x0238`/`0x024D` trigger still applies), send:

```
opcode 0x0453
  E8 03 00 00     u32 objectId    1000  (the id the matching 0x044F used)
  FF              i8  nAction     -1    = "no animation change, just talk"
  00              i8  nChatIdx    0     = the first eligible info/speak line -> Robin's n0
  00 00 00 00     u32 unused
```
10 bytes of body. Expected on screen: a balloon over Robin reading
*"Be careful! There are monsters around here."* for about 5 seconds.

### 5.2 "In order, on a cooldown"

The ordering is **ours**. Keep a per-(field, npc) cursor and a timer; on each tick send
`0x0453` with `nAction = -1` and `nChatIdx = cursor % lineCount`, then advance. For Robin's
`info/speak` that yields `n0 → n1 → n2 → n3 → n0 …`.

`lineCount` is the number of `info/speak` entries in `Npc.wz/<id>.img` — 4 for Robin.
Regenerating it belongs in the `gm-handbook` dump alongside the portals and names; the
strings themselves are **not** needed on the server, only the count, because the client
resolves the text from its own WZ.

Choose the cooldown to match the client's own feel: **3–9 s uniform**, which is what
`FUN_141e46d40` does. Nothing forces that; the client applies whatever arrives, immediately.

### 5.3 If the f/w/h lines are wanted too

Send `nAction = 3 + groupIndex` and `nChatIdx` = the index within that group's `speak`
array. **The group index is a gap** — it is the order in which `FUN_141e79060` enumerates the
template's animation nodes, which I did not establish (§7). Robin's groups are `finger`,
`wink`, `heart` plus whatever the `info`/stand node contributes, and the mapping from index
to node is unread. Start with `nAction = -1` and only chase this if the owner wants the emotions.

### 5.4 Two things not to do

* **Do not send `0x0465` or `0x0466` alongside chatter.** They set `[npc+0x1a8] = 1` and the
  `0x0453` handler then swallows the chat entirely (§3.3). [L]
* **Do not put text in `0x044F`'s trailing string** expecting a balloon. It lands in
  `[npc+0x4f0]` and is a different subsystem (§3.3). [L]

### 5.5 And one thing worth doing anyway

If `0x0327` ever does arrive (§7), it **must** be answered with a `0x0453` for the same
object id, or that NPC is latched silent forever. Echoing the client's own `nAction` and
`nChatIdx` back is the faithful behaviour; overriding `nChatIdx` with the server's cursor is
how "in order" is imposed while still letting the client drive the timing.

---

## 6. `#p8#`

`#p8#` is a name substitution: `p` = NPC, `8` = the template id, expanded to `String.wz/
Npc.img/8/name` = "Robin".

**On the click-dialogue path the client expands it. [L]**
`FUN_142a45580` (0x142a45580..0x142a4e7db) is the client's `#`-token expander: it holds the
literals `#Cred`, `#Cgreen`, `#Cblue`, `#Cviolet`, `#Cgray`, `#Cyellow`, `#Corange`,
`#Coceanblue` and the node names `npc`, `face`, `avatar`, `illu`, and it dispatches on a
token id through a jump table (`0x142a46b35`). One of its cases,
`0x142a4c7b5`, calls **`FUN_141e781f0(&out, npcTemplateId, 1)`** — the NPC-name getter, one
of only six functions in the image that reach `String/Npc.img`, and the only one that also
reaches `name`. [L]
`FUN_142a61900` — the `CUIScriptMsg` text setter that `npc-dialogue.md` §2.4 identified as
the `0x055B` **Say** path — calls `FUN_142a45580` at `0x142a61fa1` and `0x142a6206b`. [L]

**So the server may send `d0`/`d1` verbatim, `#p8#` included, in `0x055B`.** No expansion on
our side. That also settles it for the placeholder click text the server sends today.

**On the balloon path it is unverified. [I]** `FUN_141e3b510` does its own substitution pass
for `/name`, `/town`, `/grade` and `/imitatedjob` only, and neither it nor `FUN_14158f5c0`
calls `FUN_142a45580` at one call level (checked directly). I did not chase deeper. It does
not matter for Robin — none of `n0…h0` contains a `#` — but do not assume it for an NPC whose
`speak` lines do.

Instrument note: `tools/xref.py --string "#p"` is useless here (the tokens are parsed
character by character, not as literals) and `--string "String/Npc.img"` returns **0 code
references** because every reader goes through the `.data` pointer slot `0x143a46e00`. The
pointer-following scan returns 6 functions. This is `npc-spawn.md` §7.1's blind spot again.

---

## 7. What I could not settle, and the next instrument

**Why `0x0327` has never appeared in a capture.** That is the load-bearing gap, and I want to
be plain that it is one.

Measured: across every `research/fixtures/*world*.log` and the current `world.log`, the
client-to-server opcode histogram is
`0x00D9 ×418, 0x00B8 ×107, 0x013D ×66, 0x00A6 ×66, 0x0070 ×61, 0x01ED ×43, 0x02DE ×37,
0x00DC ×37, 0x0194 ×36, 0x0184 ×36, 0x00D1 ×22, 0x02EB ×21, 0x00F2 ×15, …` and **`0x0327`
does not occur once**. [L] The grep instrument is verified: the same command finds `0x00F2`,
which `npc-click.md` decoded from these very logs.

The gates that could explain it, in the order I would test them:

1. **No controller.** The classic shape of this protocol is that only the client that *owns*
   an NPC sends the move/chat request. `0x0451` (`npc-spawn.md` §3.1) is the controller
   grant — `u8 flag; u32 id`, and `flag != 0` sets `[obj+0x38] = 2` rather than or-ing bit 1.
   **Our server has never sent `0x0451`.** I looked for the controller test and did **not**
   find one: a scan of every byte-sized operation on `[reg+0x38]` in `0x141e30000..0x141ea0000`
   returns 16 functions, all in the pool (`0x141e75800`…`0x141e76500`) plus `FUN_141e3b510`,
   and **neither `FUN_141e3e220` nor `FUN_141e46d90` is among them**. So on the evidence I
   have, there is no `[npc+0x38]` controller gate on this path — but the gate could live on
   the *caller* side (whatever walks the pool and invokes the virtual update), which I did
   not read. **[I], and the cheapest thing to try.**
2. **`FUN_141e46d90`'s first gate**, `FUN_142b590d0([npc+0x168])` — "the current animation is
   still running". Robin's `stand/0` is a single frame with `delay: 5000` and an outlink;
   whether that reads as permanently animating is unknown.
3. **`FUN_14158f2f0([npc+0x40])`** — unread.
4. **The update may never run.** `FUN_141e3e220` has 0 direct callers and two vtable slots; I
   did not identify the slot index or find the pool loop that calls it.

**The next instrument is a runtime watch, and it is cheap.** `client-patched\maplecw-hook.log`
already emits `WATCH` lines. Arm three:

* **`0x141e3e786`** — reached ⇒ the update runs and the latch is clear. Log `rdi+0x230`
  (cooldown) and `rdi+0x170` (latch).
* **`0x141e46e6d`** — reached ⇒ the picker got past every gate; `ebp` there is the number of
  choices it found. Not reached ⇒ one of the four gates in §4.1, and which one is visible
  from where it stopped.
* **`0x141e4b70f`** — reached ⇒ it built and sent `0x0327` and the capture is what is wrong.

Remember the arming race (`maplecw-hook-arming-race`): no `WATCH` line can also mean the hook
was not armed. Arm well before the spawn and confirm with a known-hot address first.

**Do not let that gap block the goal.** §5.1 does not depend on any of it: `0x0453` is
unsolicited-safe — the handler's only precondition is that the object id is in the pool and
`[npc+0x1a8]` is 0, both of which hold today.

### 7.1 Smaller things left open

* The **group index → animation node** mapping (§5.3).
* Whether `FUN_141e78630`'s filter can renumber `nChatIdx` when conditions exist (§3.2).
* The 5000 constant's exact role (§2).
* The other 16 opcodes in the `0x0453..0x0466` block are shape-only; only `0x0453`,
  `0x0465`, `0x0466`, `0x0460` were read.
* `#`-token expansion on the balloon path (§6).

---

## 8. Corrections and cross-checks against the existing documents

### 8.1 Nothing in `npc-spawn.md`, `npc-click.md` or `npc-dialogue.md` is contradicted

`npc-spawn.md`'s `0x044F` layout, the pool routing, the `0x0451` reading and the
"the field loader cannot reach the pool" negative all survive and are used above.
`npc-click.md`'s `[npc+0x190] = objectId` is what `0x0327` and `0x0453` key on, which is a
third independent confirmation of that identification. `npc-dialogue.md`'s `0x055B` Say path
gains the token-expander fact in §6.

### 8.2 One row of `npc-spawn.md` §4 needs re-checking

Row 14 reads "`u8` → `[npc+0x270]` — unknown". **`[npc+0x270]` is not written anywhere in
`FUN_141e36b20`.** A scan of the decoder's whole `.pdata` range for `[reg+0x270]` in the
disp32 form returns two hits, `0x141e36f3d` and `0x141e36fa4`, and both are `[rbp+0x270]` —
stack slots, since that function's `rbp` is a frame pointer (`lea rbp,[rsp+0xa0]` at
`0x141e36b31`) and the CNpc is in `r12`. One of them is
`mov dword [rbp+0x270], 0x7d0`, a dword constant, which cannot be a `u8` packet read at all.
The scan's positive control is the CNpc constructor's own `mov dword [rsi+0x270], 1` at
`0x141e360a6`, which it does find. [L]

So read 14 goes somewhere else. This does not change any advice — the field is still sent as
zero and nothing observable depends on it — but the offset should not be quoted. The wider
lesson is the one `npc-spawn.md` §7 already teaches from the other side: in this client a
`[rbp+N]` is far more often a stack slot than a member, because most of these functions set
`rbp` to a frame pointer.

---

## 9. Instrument notes

* **`tools/xref.py --string` again missed everything that mattered.** `String/Npc.img`,
  `speak` and `Npc/%07d.img` all return **0 code references**; following one level of `.data`
  pointer-slot indirection returns 6, 2 and 4. Positive control before believing any of it:
  `Map/Map/Map%d/%09d.img` → 8 references in 7 functions, matching `npc-spawn.md` §7.1.
* **A section-header unpack bug produced a clean, confident zero.** My first pass used
  `"<IIIIIIHI"` for the tail of `IMAGE_SECTION_HEADER` — one `H` short — so `Characteristics`
  was read two bytes off, no section tested as executable, and the scanner reported **0 hits
  for the positive control**. It looked exactly like "the string is unreferenced". The
  control caught it; without one it would have become a finding. This is the same failure
  mode as the five-vs-seven decoder count.
* **A linear `capstone` sweep stops silently at the first undecodable byte.** Every
  disassembly above used a resync (advance one byte, restart the generator) and printed a
  marker; the counts in this document are from runs with 0–4 resyncs each, all in inter-
  function padding or jump tables.
* **A raw byte search for a displacement hallucinates.** Searching `.text` for the four bytes
  `f0 04 00 00` "found" `[npc+0x4f0]` inside `0f 88 f0 04 00 00` — a `js rel32`. Requiring the
  preceding byte to be a modrm with `mod=10, rm!=100` removed it and kept every real hit,
  including the control (`FUN_141e5b5e0`, 6 sites).
* **`switch_cases.py`-style case tables were not used here**; the 20-entry NPC-pool table was
  recovered by value from `0x141e75cb8` and cross-checked against the `sub`/`je` ladder that
  precedes it.
* **The v214 tree was not consulted.** After the fact, the shape it would have predicted —
  `CP_NpcMove`/`LP_NpcMove` carrying `nAction` and `nChatIdx` — does match. That agreement is
  worth exactly nothing as evidence and is recorded only so a future session does not mistake
  it for corroboration; every number above came out of `mscw`.

---

## 10. One-paragraph summary for STATUS

NPC idle chatter is **server-triggered and client-rendered**. The client already holds every
line of `String.wz/Npc.img` (resolved at template load into `template+0xE8` for `info/speak`
and into condition-carrying 0x38-byte groups at `template+0x108` for `finger`/`wink`/`heart`),
holds `UI/ChatBalloon.img`, and shows a balloon for ~5 s — but the only code path that
creates one is the handler for inbound **`0x0453`** (`FUN_141e3b510` has two callers, both
inside it, and no vtable slot). The packet is `u32 objectId, i8 nAction, i8 nChatIdx, u32`;
`nAction = -1, nChatIdx = k` says line *k* with no animation change. No field of `0x044F`
turns chatter on. The client has its own driver that picks a **random** group and a **random**
line every **3000–8999 ms** (`FUN_141e46d40`: `rand()%6000 + 3000`) and asks the server with
outbound **`0x0327`**, latching `[npc+0x170]` until answered — but `0x0327` has never appeared
in any capture and I could not establish why (`0x0451` controller grant is the leading
candidate). Ordering and cadence are therefore **ours to choose**: a per-NPC cursor plus a
timer emitting `0x0453`. `#p8#` is expanded by the client on the `0x055B` dialogue path, so
`d0`/`d1` can be sent verbatim.
