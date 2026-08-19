# Clicking an NPC: `0x00F2`, and why Robin said nothing

**Written 2026-08-19. No Ghidra** (the project was locked by another session) and **no client
run.** Everything below is `capstone` against `client-patched\MapleStory.exe`, the repo's
`tools/pdata_lookup.py`, `tools/callers.py`, `tools/dataref.py`, plus `world.log` and the
tables already in `research/`. Every address can be re-derived that way.

Labels: **[L]** read out of the listing or a capture, **[D]** derived from two or more [L]
facts, **[I]** inferred (anything from the v214 tree at `C:\Users\user\Desktop\ModernMapleSource`
is a candidate only — different game version, 1 of 8 on a held-out control).

> Companion document: `research/npc-dialogue.md`, which decodes `0x0151` (the **quest**
> request) and inbound `0x055B` (the script message). This one decodes the packet that
> actually went out on map 40, and finds the branch that chooses between the two. Nothing in
> `npc-dialogue.md` is retracted here; §7 lists what it corroborates.

---

## 0. Answer up front

| question | answer |
|---|---|
| what is `0x00F2`? | **the plain "I clicked this NPC" request.** `u32 npcObjectId, i16 charX, i16 charY, u32 tail`. 12 bytes. [L] |
| is the `1000` our object id? | **Yes, and not by correlation this time.** The field is `[npc+0x190]`, which is written from `0x044F`'s objectId at `141e35f59` (ctor) and `141e36b73` (body decoder). [L] |
| is the `275` our `cy`? | **No — coincidence.** It is the *character's* y, from the same global the `0x0151` builder uses. Robin's `cy` is 275 because the player is standing on the same ground line. [D] |
| what chooses `0x0151` over `0x00F2`? | **A client-side menu**, built from the client's own quest tables. Found: `141e3dbe2..141e3dcfe`. The server has no say in it. [L] |
| what is the reply? | **inbound `0x055B`, message type 0 (Say)** — layout already in `npc-dialogue.md` §3. Speaker = the *template* id for that object id. [D] |
| did an unequip reach the wire? | **No.** The move request is outbound **`0x0107`** (`u32 tick, u8 invType, i16 src, i16 dst, i16 count`) and it is not in the capture. The client dropped the action inside `FUN_142cc5b00` before building anything. [L] |

---

## 1. `0x00F2` field by field

### 1.1 It is in `msexe-send-opcodes.txt` — four sites, two builders

```
0x00F2  242    FUN_141e3c5d0 @ 141e3c5d0  (call at 141e3c740)   <- site A
0x00F2  242    FUN_141e3c5d0 @ 141e3c5d0  (call at 141e3dd19)   <- site B
0x00F2  242    FUN_141e3c5d0 @ 141e3c5d0  (call at 141e3dddd)   <- site C
0x00F2  242    FUN_1428de280 @ 1428de280  (call at 1428de508)   <- site D
```

**All four write the same four fields in the same order** and differ only in where the last
`u32` comes from. [L]

| # | width | site A | site B | site C | site D | value |
|---|---|---|---|---|---|---|
| — | ctor `0x1406ed520`, `edx = 0xF2` | `141e3c740` | `141e3dd19` | `141e3dddd` | `1428de508` | opcode |
| 1 | `u32` `0x1406ed9d0` | `141e3c753` | `141e3dd2d` | `141e3ddf5` | `1428de525` | **`[npc+0x190]` = objectId** |
| 2 | `u16` `0x1406ed940` | `141e3c777` | `141e3dd51` | `141e3de19` | `1428de542` | **character x** |
| 3 | `u16` `0x1406ed940` | `141e3c79c` | `141e3dd76` | `141e3de3e` | `1428de560` | **character y** |
| 4 | `u32` `0x1406ed9d0` | `141e3c7ad` | `141e3dd87` | `141e3de4f` | `1428de57c` | A/B/C: literal `0xFFFFFFFF`; D: `[npc+0x288]` |
| — | send `0x1415d01c0` | `141e3c7b9` | `141e3dd93` | `141e3de5b` | `1428de586` | |

Site A's body verbatim, as an example [L]:

```
141e3c734  mov  edx, 0xf2
141e3c740  call 0x1406ed520          ; COutPacket(0x00F2)
141e3c746  mov  edx, [rsi+0x190]     ; rsi = the CNpc
141e3c753  call 0x1406ed9d0          ; u32
141e3c758  mov  rcx, [0x143aa8518]   ; the player singleton
141e3c75f  add  rcx, 8
141e3c763  mov  rax, [rcx]
141e3c76a  call qword [rax+0x30]     ; GetPos -> out buffer
141e3c76d  movzx edx, word [rax]      ; x
141e3c777  call 0x1406ed940          ; u16
141e3c78a  call qword [rax+0x30]      ; (called twice; same object)
141e3c791  movzx edx, word [rax+4]    ; y
141e3c79c  call 0x1406ed940          ; u16
141e3c7a1  mov  edx, 0xffffffff
141e3c7ad  call 0x1406ed9d0          ; u32
141e3c7b9  call 0x1415d01c0          ; send
```

### 1.2 Field 1 really is our object id [L]+[D]

`[npc+0x190]` is written in exactly two places, and both are the object id from `0x044F`:

* `141e35f59  mov [rsi+0x190], edi` — the CNpc constructor `FUN_141e35ea0(this, template, objectId)`;
* `141e36b73  mov [rcx+0x190], edx` — `FUN_141e36b20(npc, objectId, packet)`, the body
  decoder `research/npc-spawn.md` §1.1 already identified, called at `141e75a35` with the
  `u32` read at `141e7598e`.

The getter `FUN_141e36b10` is `mov eax,[rcx+0x190]; ret` [L], and site D calls it directly.

**This survives the objection that killed the earlier `0x0151` reading.** `config.rs` gives
every map's first NPC id 1000, so a 1000 on the wire proves nothing by correlation — but
here the *code* reads the field the spawn packet wrote. No correlation is involved.

### 1.3 Fields 2 and 3 are the *character's* position, not the NPC's [D]

Sites A/B/C use `mov rcx,[0x143aa8518]; add rcx,8; mov rax,[rcx]; call qword [rax+0x30]` —
**instruction for instruction the same sequence** `npc-dialogue.md` §1.3 decoded inside the
`0x0151` builder at `141f0e536`, and already matched there against a `0x00D9` movement
packet. Two `LONG`s, written as `u16`s. [L]

Site D reads `[rsi+8]`'s vtable slot `0x30` instead — and **`rsi` is the same singleton**:
`1428de5a6 mov byte [rsi+0x5280], 1` is the same store that sites A/C do as
`141e3ddb2 mov rax,[0x143aa8518] / mov byte [rax+0x5280], 1`. So all four sites read the
same object. [D]

Corroboration in our own capture: the byte pair sequence `01 00 13 01` — `(1, 275)` as two
little-endian `i16` — appears **verbatim twice in each** of the `0x00D9` movement packets at
`18:44:01.289` and `18:44:25.753`. [L]

So the `275` is **not** Robin's `cy` being echoed back. The player is standing on the same
foothold row: our own `0x044F` gave Robin `x=69, cy=275, fh=30, rx0=19, rx1=119`, and the
character is at `x=1` on that same ground. Two independent things happen to be 275.

### 1.4 The captured packet, decoded

```
e8030000 0100 1301 ffffffff        (12)   world.log 18:45:53.560
^        ^    ^    ^
|        x=1  y=275 tail = -1
objectId = 1000  -> Robin, template 8   (0x044F sent at 18:44:00.792)
```

### 1.5 The tail `u32`

* Sites A/B/C write the literal `-1`. [L]
* Site D writes `[npc+0x288]` via `FUN_141e646b0` (`mov eax,[rcx+0x288]; ret`). [L]

`[npc+0x288]` is an **animation/action index**: the CNpc ctor sets it to `-2`
(`141e360d9 mov qword [rsi+0x288], 0xfffffffffffffffe`, which also leaves `[+0x28c] = -1`)
and `[+0x290] = -1` (`141e360e4`); it is then filled from `FUN_141e84310(template, 1)` at
`141e3e824` / `141e4c0af` and clamped against `template[0x134]` at `141e3f92f` / `141e4c577`.
[L] `FUN_141e84310` **returns `-1`** when `template[0x138]` is null (`141e84788`) [L], so
`-1` is reachable at site D too.

> **I could not tell the four sites apart from the wire.** I tried: the tail was the only
> candidate discriminator and `-1` is reachable from all four. What the owner's observation *does*
> rule out is **site B**, which is only reached after a modal menu has been shown and
> dismissed with a selection (§2). Sites A, C and D all send silently. **[D]**

---

## 2. The branch: how a click picks `0x0151` or `0x00F2`

`FUN_1428de280(player, ctl)` is the click handler. `npc = ctl->[8]`
(`FUN_1429542a0` is `mov rax,[rcx+8]` with an assert). It has **5 callers** —
`1428923e0`, `1428b1bc0`, `1428b7240`, `1428ec500`, `142927830` — all in the UI band, and
four of the five call no packet-read primitive at all (`142927830` calls one; control: the
`0x055B` head reader `141f6f350` shows 9). `FUN_141e3c5d0` has **exactly one caller: this
one.** [L, `callers.py`]

```
FUN_1428de280(player, ctl):
  npc = ctl->[8]
  if (!FUN_141e3ff10(npc))                             -> nothing            1428de2e8
  if (142cc1cf0 || 142cc1e30 || 142da1270 || 142d16c70) -> beep, nothing      1428de2f5..1428de349
  if (!FUN_141e41c30(npc, x, y))                        -> message 0x1257     1428de3a8
  if (FUN_1428de600(player, ...))                       -> nothing            1428de429
  if (FUN_141e39b50(npc))    <<-- THE FORK              -> FUN_141e3c5d0(npc) 1428de441 / 1428de455
  tid = npc->[0x198]->[0]                               ; the WZ template id
  if (tid == 0xF719B)                                   -> FUN_142dcb430      1428de46c
  if (FUN_1401d3220(tid))                               -> open a UI          1428de498
  else                                                  -> **0x00F2, site D** 1428de4fe
```

`FUN_141e39b50(npc)` is `npc->[0x1f0] != NULL && *(u32*)(npc->[0x1f0] - 8) != 0` [L] — i.e.
**"this NPC has a non-empty script name"**. That string is filled *by the client, at
construction*, from a hash map keyed by template id:

```
141e362dc  mov  rax, [rsi+0x198]          ; template
141e362e3  lea  r8,  [rsi+0x1f0]          ; &npc->scriptName
141e362ea  mov  edx, [rax]                ; templateId
141e362ec  mov  rcx, [0x143aa9d98]        ; the quest-data singleton
141e362f3  call 0x140719360               ; hash lookup, key%[+0x228] over [+0x220]
```
[L] `[0x143aa9d98]` is the quest singleton: `dataref.py` reports 413 references and
`FUN_1407155d0` — one of the six quest-table accessors `npc-dialogue.md` §1.2 named — is
among them. [L]

### 2.1 Inside `FUN_141e3c5d0` — the menu, and the only route to `0x0151`

```
141e3c610  if (FUN_141b1f960(g, 1))            -> return          ; nothing sent
141e3c629  if (!global || !global2)            -> return
141e3c645  if (npc->[0x270] == 0)              -> return
141e3c652  if (npc->[0x294] != 0)              -> return
141e3c65f  local = copy of template->[0x178]
141e3c680  FUN_142da3ac0(g, &out, &npc->[0x1f0])   ; resolve the script name
141e3c6ad  FUN_142da38d0(g, ..., &out, 0)          ; -> Etc/ScriptInfo.img
141e3c715  if (len(npc->[0x1f8]) != len(out))  -> 141e3c9a3 (the long path below)
141e3c72b  if (local is a non-empty C string)  -> **0x00F2, site A**  and return
                                               else return silently

... the long path ...
141e3da57  arr = [rbp-0x80]                    ; ZArray<int> of quest ids for this NPC
141e3da66  if (arr empty && local empty)       -> 141e3de99  (another UI path)
141e3da78  if (arr == NULL || arr.count == 0)  -> **0x00F2, site C**
141e3dab5  ... build the menu text ...
141e3db1b  FUN_142a61900(ui, 6, npcTemplateId, &text)   ; type 6 = a list
141e3db34  FUN_142a5ee30(ui)
141e3db50  r = ui->vtbl[0x130](ui)             ; MODAL - the menu is on screen here
141e3db56  if (npc->[0x270] == 0)              -> error dialog, return
141e3db96  if (r != 1 && r != 0x2001)          -> return           ; cancelled
141e3dba6  sel = FUN_142a64400(ui)
141e3dbb5  if (arr == NULL)                    -> **0x00F2, site B**
141e3dbc2  if (sel >= arr.count)               -> **0x00F2, site B**
141e3dbd8  q = arr[sel]
141e3dbe2  found = q in npc->[0x200] | npc->[0x208] | npc->[0x210]
141e3dc9d  if (!found)                         -> return           ; nothing sent
141e3dce8  FUN_142d9ac30([0x143aa84a0], q, npcTemplateId, 0)  -> **0x0151**
```
All [L].

`FUN_142d9ac30` is the exact function `npc-dialogue.md` §1.2 identified: it allocates 0x68,
runs `FUN_141f0e110(obj, questId, npcTemplateId)` and then `FUN_141f0e4c0(obj, 0)` — the
`0x0151` builder. So `obj+0x20 = q` and `obj+0x24 = npcTemplateId`, which is exactly what
that document derived from the capture. [D]

The three arrays `npc->[0x200] / [0x208] / [0x210]` are nulled in the ctor
(`141e3600a`, `141e36011`, `141e36018`) and filled by **`FUN_141e40010`**, which calls
`FUN_14070fae0` (the quest lookup) and `FUN_140711d70` (the requirement checker) — both
named in `npc-dialogue.md` §1.2. [L] So they are **quest-id lists per state**. [D]

The same function reaches the literals `#fUI/UtilDlgEx.img/UtilDlgEx/list0#`…`/list3#`,
`Etc/ScriptInfo.img` and `#d#L%d# %s#l#k` (`msexe-packet-fields.txt`, row `0x00F2`) [L] —
`#L%d#` is the selectable-line markup, which is why "menu" is [D] rather than [I].

### 2.2 What that means for the server

**The choice is made entirely inside the client, off `Quest.wz` and `Etc/ScriptInfo.img`.**
Nothing the server sent selects it, and no field of `0x044F` selects it either. On maps
1/10/20/30 the tutorial NPCs have quests in the client's own tables, so the quest path was
taken; Robin (template 8) has none, so the plain talk request went out.

**Honest limit:** I traced one of `FUN_142d9ac30`'s **26** call sites. The other 25 are UI
(two of them, `FUN_1428db920` and `FUN_1428dbc20`, are siblings of the click handler in the
same class and are plausibly the quest balloon over an NPC's head). I did not verify that
the earlier `0x0151` captures came through the click path rather than one of those. [I]

---

## 3. The reply: inbound `0x055B`, message type 0

### 3.1 The evidence

1. `0x00F2` carries an NPC object id and the player's position and **nothing else**; the
   client then does nothing further locally on that path. There is no local dialog, no
   timeout, no retry. [L]
2. The only inbound packet in the client that puts an NPC on screen with text is `0x055B`,
   routed by `CField::OnPacket` (`141821f84`) to the script manager `[0x143acedb0]` —
   `npc-dialogue.md` §2.2, §2.3, §2.4. [L]
3. **Independent corroboration this session:** the click handler's own local menu calls
   `FUN_142a61900(ui, 6, npcTemplateId, &text)` at `141e3db1b` [L] — the *same* function,
   with the NPC **template** id in the *same* argument slot that `0x055B`'s handler fills
   from head field 3 (`142a61997 mov [rcx+0x2cc], r8d`). Locally generated and
   server-generated NPC dialogs go through one `CUIScriptMsg`. [D]
4. **`0x00F3` is the script-dialog answer, not just an adjacent number.**
   `msexe-send-opcodes.txt` lists 57 builders for `0x00F3`, and the whole
   `0x141f6f…0x141f76` block is in it — including `FUN_141f6f350` (the `0x055B` head reader)
   and `FUN_141f6fb20` (the Say handler). So `0x00F2`/`0x00F3` are a request/answer pair for
   the same subsystem, and the adjacency is not the argument. [L]
5. **The NPC pool is not an alternative route.** I checked whether the pool's per-NPC command
   block (`0x0453..0x0466`, `FUN_141e75aa0`/`FUN_141e75f50`) could open a dialog: **no calls
   at all** to `0x142a5…`, `0x142a6…`, `0x141f6…`, `0x141f7…`, the packet ctor or the sender.
   *Instrument check:* the same grep finds 1 and 3 calls to the read primitives in those two
   functions, so it was capable of matching. [L]

**Conclusion [D]:** answer `0x00F2` with **inbound `0x055B`, message type 0**, using the
layout already written out byte for byte in `research/npc-dialogue.md` §3.

### 3.2 The one thing the server has to do that `0x0151` did not require

`0x0151` hands you the **template** id directly (its second `u32`). `0x00F2` hands you the
**object** id. `0x055B`'s speaker field is a **template** id — it goes straight into
`FUN_141e77b70`, the `Npc/%07d.img` loader (`npc-dialogue.md` §2.3). So the server must map
`objectId -> template` using the same table it built `0x044F` from. For this capture:
`1000 -> 8`.

Sending the object id as the speaker would not crash — the load result is null-checked at
`142a7b52a` — but the dialog would come up with no portrait.

### 3.3 Not blocking, and no post-send latch

After the send, all four sites do
`if (FUN_1404ba780(templateId)) player->[0x5280] = 1`. **`FUN_1404ba780` is
`xor al,al; ret`** [L] — it always returns 0, so that flag is never set on this path in this
build. Nothing latches. As with `0x0151`, leaving `0x00F2` unanswered does not freeze
anything (measured: it went unanswered at 18:45:53 and the client kept sending `0x00B8`,
`0x01ED` and `0x013D` for another 40 s before the owner closed it — `world.log`).

Two hazards from `npc-dialogue.md` still apply and are worth repeating:

* the script manager is **reset on every field entry** (`FUN_142caa4e0 -> FUN_141f6f200`), so
  never send `0x055B` with or just before a `SetField`;
* `messageType` must be `0`; 25 of the 71 types are dead slots that silently do nothing.

---

## 4. The unequip: nothing reached the wire

### 4.1 The request is outbound `0x0107`

`FUN_142cc5b00(player, invType, srcSlot, dstSlot, count)`, `msexe-send-opcodes.txt` line 271.
Body [L]:

```
142cc5ea3  mov  edx, 0x107
142cc5eac  call 0x1406ed520
142cc5eb2  call 0x1429e3ef0          ; a tick
142cc5ebd  call 0x1406ed9d0          ; u32  tick
142cc5ec2  movzx edx, r12b
142cc5eca  call 0x1406ed840          ; u8   invType      (arg2)
142cc5ecf  movzx edx, r14w
142cc5ed7  call 0x1406ed940          ; i16  srcSlot      (arg3)
142cc5edc  movzx edx, di
142cc5ee3  call 0x1406ed940          ; i16  dstSlot      (arg4)
142cc5ee8  movzx edx, word [rbp+0x460]
142cc5ef3  call 0x1406ed940          ; i16  count        (arg5)
142cc5efc  call 0x1415d01c0          ; send
```

11 bytes. `[rbp+0x460]` resolves to `rsp0+0x28`, the **5th argument's home slot** — the
frame is `lea rbp,[rsp-0x3f8]` then `sub rsp,0x4f8` after 8 pushes. [D]

**The slots are signed and negative means equipped** — read, not assumed [L]:

```
142cc5b8e  ...                        ; (guards, below)
142cc5c7f  test r14d, r14d            ; srcSlot
142cc5c82  js   142cc5c8c             ; negative -> the equipped branch
142cc5c84  test edi, edi              ; dstSlot
142cc5c86  jns  142cc5d0e
...
142cc5d0e  cmp  r12d, 1               ; invType == 1  (EQUIP)
142cc5d1e  cmp  r14d, -0xb            ; srcSlot == -11
142cc5d24  cmp  edi,  -0xb            ; dstSlot == -11   (the weapon-slot special case)
```

So the owner's unequip would have been `0x0107` with `invType = 1`, `srcSlot = -5` (`FB FF`),
`dstSlot` = a free EQUIP slot, `count` = 1.

`0x0107` sits in a dense contiguous request block `0x0105..0x0135` all built by one class in
`0x142cc…`, which is the shape of `CUserLocal`'s request methods. [L]

### 4.2 It is not in the capture, and neither is anything of that shape

`world.log` for that session carries `0x00B8 ×10, 0x013D ×6, 0x01ED ×4, 0x00D9 ×3,
0x02EB ×2, 0x0070 ×2` and one each of `0x0408, 0x02DE, 0x02B2, 0x024D, 0x0238, 0x01A5,
0x0194, 0x0184, 0x00F2, 0x00ED, 0x00DC, 0x007D`. No `0x0107`, and none of those has an
`invType, src, dst` shape — checked against `msexe-packet-fields.txt`:

| opcode | builder | shape |
|---|---|---|
| `0x0184` | `FUN_142defbc0` | delegates the whole body to `FUN_142df2760` |
| `0x0194` | `FUN_142defc50` | `u8` |
| `0x02B2` | `FUN_142d0ec30` | `u32, u32` |
| `0x00ED` | `FUN_142d4dd10` | `u32, u32` |
| `0x00DC` | `FUN_1428a81d0` | empty |
| `0x01ED` | **104 call sites**, overwhelmingly `FUN_1406ede20` raw-byte writes | the client's own log/telemetry channel [I] |
| `0x0408`, `0x02DE` | **absent from the table** | virtualised builder — absence is not evidence they do not exist |

So: **the client swallowed it.** The same answer as the slash commands.

### 4.3 Why — six candidate gates, all before the packet is built

`FUN_142cc5b00` returns `false` without sending at any of these [L]:

| at | test | returns |
|---|---|---|
| `142cc5b50` | `player->[0x2338] != 0` | 0 |
| `142cc5b5d` | `player->[0x2330] != 0` | 0 |
| `142cc5b74` | `player->[0x2358] == NULL` | 0 |
| `142cc5b88` | `FUN_1401ba9d0(&cd[0x5b], cd[0x63]) <= 0` | 0 |
| `142cc5b9e` | `tick - player->[0x2334] < 0x1F4` (500 ms) | 0 |
| `142cc5baf` | `FUN_1428fb0b0(g) == 1` | 0 |

Two of them are worth naming:

* **`player->[0x2330]` is a one-request-outstanding latch.** It is set to `1` immediately
  after the send (`142cc5f01`) together with `[0x2334] = tick`. A byte scan finds
  **37 functions** that set it and only **7** that clear it — and the clearers
  (`142cc52a0`, `142cd8d70`, `142cd8e90`, `142cf3810`, `142d04af0`, `142d04b70`,
  `142cae700`) are **inbound packet handlers**: `FUN_142cc52a0` is literally
  `read(packet); [this+0x2330] = 0; [this+0x2334] = tick`. [L]

  So: **if the server does not answer one of those 37 requests, every later request in that
  class is silently dropped.** That is a live "always answer" hazard for this project. It is
  *not* what happened here — none of the 37 setters' opcodes appears in the capture, so the
  latch was probably never set — but it will bite the first time one of them does.

* **`player->[0x2358]`** is the character/inventory data object: `FUN_142cbe730` is a leaf
  getter for it, and the sibling builder `FUN_142dda360` hands the same pointer to
  `FUN_1401dd300(x, invType)`, which indexes a vector of 16-byte entries at `x+0x146..0x14e`
  [L]. `FUN_1401ba9d0` is the ZtlSecure de-obfuscator (`0xbaadf00d`, rol/ror 5) [L], so
  `cd[0x5b]/cd[0x63]` is an obfuscated `(value, checksum)` pair that must decode `> 0`.
  I did **not** identify which field that is — it is not written by the character-record
  decoder `FUN_140304b20` (`140304b20..140309404` scanned for displacement `0x5b`: 0 hits;
  the same scan hits `14030df66` just outside that range, so the instrument speaks). If it
  is an inventory slot count, a zero in our `SetField` record would explain the whole thing.
  **[I] — a lead, not a finding.**

### 4.4 The next instrument, and it is cheap

`client-patched\maplecw-hook.log` already emits `WATCH` lines. Put a watch on
**`0x142cc5b00`** and on the two exits, **`0x142cc5c16`** (the common bail) and
**`0x142cc5ea3`** (the `mov edx, 0x107` that begins the packet). One drag of the Undershirt
then answers all three questions at once:

* no `WATCH` at `142cc5b00` → the UI never asked; the drop target or the double-click
  binding is the problem, not the protocol;
* `142cc5b00` but then `142cc5c16` → one of the six gates above; log `rsi+0x2330`,
  `rsi+0x2338`, `rsi+0x2358` in the watch to say which;
* `142cc5ea3` → it built and sent, and the capture is what is wrong.

Remember the arming race (`maplecw-hook-arming-race`): no `WATCH` line can also mean the
hook was not armed. Arm it well before the drag and confirm with a line from a known-hot
address first.

---

## 5. What the server should do, concretely

On inbound `0x00F2`:

1. read `u32 objectId` (ignore the `i16 x, i16 y` and the trailing `u32`);
2. look the object id up in the field's NPC list -> **template id**;
3. reply with **`0x055B`**, `messageType = 0`, `speakerTemplate =` that template id, per
   `research/npc-dialogue.md` §3. For this capture:

```
opcode 0x055B
00000000   u32 handle           echoed back in the client's 0x00F3; pick anything
00         u8  -                discarded on the Say path
08000000   u32 speakerTemplate  8  = Robin  (NOT 1000, the object id)
00         u8  hasOverride      0
00         u8  messageType      0 = Say
0000       u16 flags            0
00         u8  -
00000000   u32 echo             echoed back in 0x00F3
0e00       u16 length
"Hello, I'm Robin."             length bytes, not NUL-terminated, not UTF-16
00         u8  prev
00         u8  next
00000000   u32 -
```

Do **not** also answer `0x0151` for the same click — the client will not send both.

---

## 6. Gaps, stated rather than guessed

* **Which of the four `0x00F2` sites fired is not settled** (§1.5). Only site B is excluded,
  and only on the owner's "no dialogue appeared". The packet is identical either way.
* **What `FUN_141e39b50`'s script-name map contains for template 8** was not read. Doing it
  needs the client's `Etc/ScriptInfo.img` / quest data through the WZ tooling, not the PE.
  That is the one measurement that would tell us whether `FUN_141e3c5d0` was entered at all.
* **`cd[0x5b]/cd[0x63]`** (§4.3) is unidentified. If the drag turns out to be blocked at
  `142cc5b88`, that is the field to chase, and it is an obfuscated pair, so the search has to
  look for `FUN_1401ba9d0`'s *writer*, not for a plain store.
* **`0x0408` and `0x02DE`** have no builder in `msexe-send-opcodes.txt`. Per that file's own
  note, absence means the builder is virtualised, not that it is missing. They are unread.
* **The earlier `0x0151` captures were not traced to a call site** — 25 other routes to
  `FUN_142d9ac30` exist (§2.2).
* **`FUN_141e3c5d0`'s `141e3c9a3` long path** was read only along the branches that lead to a
  packet. Its other 4 KB (dialogs, `#L%d#` formatting, `Etc/ScriptInfo.img` lookups) was not.

---

## 7. Instrument notes

* `msexe-send-opcodes.txt` **had `0x00F2` already**, with all four sites. Checking the table
  first, as the brief said, saved the whole search. Its documented trap held too:
  `0x0408`/`0x02DE` are absent and that is a virtualisation result, not a negative.
* **A linear `capstone` sweep stops silently at the first undecodable byte.** A scan for
  writes to `+0x2330` across `.text` returned **0 hits** for a store this session had already
  read by hand at `142cc5f01`. Adding a resync (advance one byte and restart the generator)
  turned that into 62 hits in a 64 KB window. Any whole-`.text` sweep written here in future
  must resync, and must be run against a hit you already know.
* A byte-pattern search is only as good as the encoding you assume. `8? 80 52 00 00` found
  nothing for `player->[0x5280]` because the write is `C6 86 …`, not `8x`. The control
  (`c6 8? 80 52 00 00 01`) found all four known sites.
* `tools/callers.py` behaved (control: `0x1402fa9a0` -> 96 sites in 15 functions) and its
  single-caller answer for `FUN_141e3c5d0` is what collapsed the search space.

---

## 8. What this corroborates in `npc-dialogue.md`

Nothing there is retracted. Three of its claims get independent support from a different
direction:

* **§0 "`0x0151` is the quest request, not NPC click"** — now with the call site that proves
  it: `141e3dcfe`, reached only after a quest id is picked out of a client-side menu.
* **§1.2's `obj+0x20 = questId`, `obj+0x24 = npcTemplateId`** — read directly off the caller:
  `mov edx, ebx` (the quest id from the menu array) and `mov r8d, [rax]` (the template id)
  at `141e3dcf5`/`141e3dcf2`.
* **§2.3's "head field 3 is the speaker template id"** — the client's *local* dialog passes
  the template id in the same argument slot of the same function (`141e3db1b`).
