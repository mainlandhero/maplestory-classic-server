# `0x00F3`: what the client sends back, and how to hold a conversation

**Written 2026-08-19. No Ghidra.** `capstone` against `client-patched\MapleStory.exe`, the
repo's `tools/callers.py`, `tools/dataref.py`, `tools/pdata_lookup.py`, plus **three real
`0x00F3` bodies in `world.log` whose provoking `0x055B` is known byte for byte**. Every
address can be re-derived that way.

Labels: **[L]** read from the listing or a capture, **[D]** derived from two or more [L]
facts, **[I]** inferred (anything from the v214 tree is a candidate only).

> Companions: `research/npc-dialogue.md` (the `0x055B` head and the Say body) and
> `research/npc-click.md` (`0x00F2`, and the client-side menu that picks `0x0151`).
> **Nothing in either is retracted here.** §9 lists what this corroborates and the two
> places where it sharpens a claim.

---

## 0. Answer up front

| question | answer |
|---|---|
| `0x00F3` body | **`u32 handle, u8 messageType, <per type>`**. For Say: `u32 echo, str text, u8 action`. [L] |
| which field says what was pressed | the **trailing `u8`**, and it is a **signed `i8`**: `1` / `0` / `-1`. [L] |
| can the server tell Next from OK | **No. The client deliberately collapses them.** `BtOK` (id 1) is *remapped to `0x2001`* before it reaches the dialog result (`142a59fb5`). Both send `1`. [L] |
| so how does the server page | by remembering whether the box it just sent had `next = 1`. The result byte says "advance", not "which button". [D] |
| the yes/no type | **message type 3** (`BtYes` / `BtNo`), and **type `0x10`**, the same handler with quest-flavoured captions (`BtQYes` / `BtQNo`, or `BtQStart` / `BtQAfter`). Result: **`1` = Yes, `0` = No, `-1` = closed.** [L] |
| are `handle` / `echo` validated | **No.** Each is read once and written once, into the answer. No compare, no range test, no index. The server may choose them freely — **but only type 0 echoes them; type 3 sends `0`.** [L] |
| must the server answer `0x00F3` | **No, and nothing is left latched.** The dialog is destroyed and the script-manager latch released before the packet is sent. `0x00F3` is **not** one of the 37 setters of `player->[0x2330]`. [L] |
| what the owner actually pressed | **the OK button on a plain Say box.** Whatever the art said, a type-0 Say with `next = 0` has exactly `BtOK` + `BtClose`. There was no Accept button on the wire. [D] |

---

## 1. The capture, decoded against the builder

### 1.1 What went out, and it parses with nothing left over

`world.log 20:29:10.267`, the Robin reply — the whole 42-byte body re-parsed against the
client's read order from `npc-dialogue.md` §2.3 / §2.4:

```
00000000              u32  handle        = 0
00                    u8   -             (discarded on the Say path)
08000000              u32  speaker       = 8   (Robin's template)
00                    u8   hasOverride   = 0
00                    u8   messageType   = 0   (Say)
0000                  u16  flags         = 0
00                    u8   -             -> [ui+0x2a4]
00000000              u32  echo          = 0
1000                  u16  length        = 16
48 65 6c 6c 6f 21 20 49 27 6d 20 23 70 38 23 2e   "Hello! I'm #p8#."
00                    u8   prev          = 0
00                    u8   next          = 0
00000000              u32  -             -> [ui+0x790]
                                                        42 bytes, 0 remaining
```

**That is the first end-to-end confirmation of the `0x055B` layout against a packet the
client accepted and drew.** [L] The Heena packet (`20:27:34.439`) is the same shape with
`speaker = 1` and a 162-byte string (its outbound body is truncated in `world.log`, so only
its `0x00F3` was re-parsed).

> **The whole chain in this document was then executed as code against both Robin captures**
> — parse the sent `0x055B`, decide the style, decide which buttons the layout draws, apply
> the `OnButtonClicked` remap, apply the answer arithmetic, and compare to the real
> `0x00F3`. Seven checks each: zero bytes left over on both sides, `handle` echoed, `echo`
> echoed, text echoed byte for byte, message-type echo `0`, and **action byte `1` predicted
> from "the user pressed `BtOK`"**. All pass. The model is not just consistent with the
> capture, it *predicts* it. [D]

### 1.2 Which builder produced the answer — settled, not guessed

`msexe-send-opcodes.txt` lists **57** `0x00F3` builders. I extracted the write sequence
between each one's `COutPacket` ctor and its send (control: the same extractor re-derives
`0x00F2` site A as `u32,u16,u16,u32` / send, matching `npc-click.md` §1.1). Twelve of the 57
write a string. Exactly **one** has the shape `u32, u8, u32, STR, u8`:

```
141f6fd3c  in FUN_141f6fb20   u32,u8,u32,STR,u8   [send]      <- the Say handler's own answer
```

Arithmetic on the 28-byte capture: `4 + 1 + 4 + (2+16) + 1 = 28`. The nearest competitors
are `u32,u8,u8,STR,u8` (25 bytes), `u32,u8,STR,u32` (27) and `u32,u8,STR` (23). **None
fits.** So the captured packet is `141f6fd33..141f6fd9e`. [D]

### 1.3 The field boundary the capture *cannot* settle, and the listing does

A first pass over the capture reads the nine leading zero bytes as `u32 handle, u32 echo,
u8`. The builder says otherwise, and the bytes cannot tell the difference because the server
sent `handle = 0` and `echo = 0` — nine zeroes either way. The listing is unambiguous [L]:

```
141f6fd33  mov  edx, 0xf3
141f6fd3c  call 0x1406ed520      ; COutPacket(0x00F3)
141f6fd42  mov  edx, [rsp+0x5c]  ; = arg2 = 0x055B head field 1
141f6fd4a  call 0x1406ed9d0      ; u32   handle
141f6fd4f  xor  edx, edx
141f6fd55  call 0x1406ed840      ; u8    0            <- LITERAL, the message type
141f6fd5a  mov  edx, [rsp+0x60]  ; = the Say body's field 9
141f6fd62  call 0x1406ed9d0      ; u32   echo
141f6fd67  lea  rdx, [rsp+0x68]  ; = the text ZXString
141f6fd70  call 0x1406edc80      ; str   the text, echoed verbatim
   ... the action byte, §2 ...
141f6fd95  call 0x1406ed840      ; u8    action
141f6fd9e  call 0x1415d01c0      ; send
```

**Field 2 is the echoed message type**, not a spare byte. Its siblings prove it: the same
slot is a literal `1` for type 1 (`141f7003d`), `2` for type 2 (`141f703e7`), `0x11` for
type 0x11 (`141f70d26`), and `3`-or-`0x10` for type 3 (`141f70a5b`, chosen by `cmovne`).
**That byte is how the server knows which box is being answered.** [L]+[D]

The string echo is real, not a coincidence of two matching lengths: `0x1406edc80` is the
`u16`-length string writer and it is handed `[rsp+0x68]`, the very buffer `0x1406e9050`
filled at `141f6fb93`. [L]

---

## 2. The action byte — and why OK and Next are the same value

### 2.1 The arithmetic

```
141f6fd75  cmp   edi, 0x2001      ; 81 ff 01 20 00 00
141f6fd7b  jne   0x141f6fd84      ; 75 07
141f6fd7d  mov   eax, 1           ; b8 01 00 00 00
141f6fd82  jmp   0x141f6fd8e      ; eb 0a
141f6fd84  sub   edi, 0x2000      ; 81 ef 00 20 00 00
141f6fd8a  neg   edi              ; f7 df      CF = (edi != 0)
141f6fd8c  sbb   eax, eax         ; 1b c0      eax = -CF
141f6fd8e  movzx edx, al          ; 0f b6 d0
```
Raw bytes read off the image, not just capstone's rendering. [L] So, with `edi` = the modal
result:

| `edi` | action byte |
|---|---|
| `0x2001` | **1** |
| `0x2000` | **0** |
| anything else | **0xFF** (= `-1` as `i8`) |
| `3` | *no packet is sent at all* — see §6.3 |

### 2.2 The modal result is the button id, through a two-step chain

`ui->vtbl[0x130]` is `FUN_14177f640`, a **blocking modal**: it runs the frame pump
`FUN_142c45e50(&[ui+0x234])` and returns `[ui+0x230]` (or 0 if the dialog vanished from the
modal list). `[ui+0x230]` starts at **0** — the base dialog ctor `FUN_14177edb0` zeroes
`+0x230`/`+0x234` as one qword at `14177ede5`. It is written only by `EndDialog`
= `FUN_14177f850(dlg, code)`: `[dlg+0x230] = code`, `[dlg+0x234] = 1`. [L]

The chain from a click is [L]:

```
button clicked
  -> CUIScriptMsg::OnButtonClicked = FUN_142a59f50(ui, buttonId)
       142a59f60  if (buttonId == 2)  tail-call notify(2)              ; Close, any kind
       142a59f79  switch on [ui+0x2a8]  (the dialog kind, 0..0x1d)
  -> notify = ui->vtbl[0x138] = FUN_142a5a880(ui, code)
       142a5b6ad  call FUN_14177f850(ui, code)                         ; EndDialog
  -> [ui+0x230] = code -> Show() returns it -> the action byte above
```

### 2.3 The remap that the capture caught and a static read would not

`FUN_142a59f50`'s case for **dialog kind 0 (the Say box)**, at `142a59f9d` [L]:

```
142a59f9d  lea  eax, [rbp - 0x2000]     ; rbp = buttonId
142a59fa3  cmp  eax, 1
142a59fa6  jbe  0x142a5a12e             ; 0x2000 or 0x2001 -> notify(buttonId) unchanged
142a59fac  cmp  ebp, 1
142a59faf  jne  0x142a5a66c             ; not OK -> base-class default, no result
142a59fb5  mov  edx, 0x2001             ; <<<< OK is REWRITTEN to Next
142a59fca  jmp  qword ptr [rax + 0x138] ; notify(0x2001)
```

**`BtOK` never reaches the dialog result as `1`. It arrives as `0x2001`.** That is why the
capture's action byte is `01` for a box whose only buttons were OK and END CHAT.

### 2.4 The Say box's four buttons, and what each sends

The layout is `FUN_142a65740` (`UI/UtilDlgEx.img/UtilDlgEx`), selected because
`FUN_142a61900(ui, 0, ...)` stores `[ui+0x2a8] = 0` and `CreateLayout`'s 30-entry table at
`0x142a58d08` maps index 0 to it. [L] Captions come from the `.rdata` pointer table
(`0x143a48410` = `BtPrev`, `+8` = `BtNext`, `+0x10` = `BtOK`, `+0x18` = `BtClose`,
`+0x20` = `BtYes`, `+0x28` = `BtNo`; UTF-16, resolved from the image). [L]

| button | drawn when | control slot | id | notify code | **action byte** |
|---|---|---|---|---|---|
| `BtPrev` | `prev != 0` | `[ui+0x5f8]` | `0x2000` | `0x2000` | **0** |
| `BtNext` | `next != 0` | `[ui+0x608]` | `0x2001` | `0x2001` | **1** |
| `BtOK` | `next == 0` | `[ui+0x608]` | `1` | **`0x2001`** | **1** |
| `BtClose` | `!(flags & 1)` | `[ui+0x618]` | `2` | `2` | **0xFF** |

Read at `142a65b99` (prev test), `142a65cbc` (next test), `142a65e6c` (close test), with the
ids at `142a65c7b`, `142a65d8c`, `142a65e3f`, `142a65f1d`. [L]

**`BtNext` and `BtOK` occupy the same control slot** — they are the same button with two
faces. That is the structural reason for the remap.

> There is a second, parallel construction of the same four buttons at `142a657ee..142a65b94`
> using `FUN_141aa81d0` instead of the control's own `Create`. It is chosen at `142a65775`
> when `[ui+0x2a0]` (the *style*, from `flags & 0x20` / `flags & 0x80`) is 1 or 2. It uses
> **the same ids** (`0x2001` / `1` / `2`) and draws **no Prev button at all**. With
> `flags = 0` the style is 0 and the first construction runs. [L] If you ever set
> `flags & 0x20` or `& 0x80`, you lose Prev.

### 2.5 So: what the server can and cannot read

**Cannot**: distinguish "the user pressed Next" from "the user pressed OK". Both are `1`.
**Cannot**: learn which line the user is on — nothing in `0x00F3` carries an index.

**Can**: distinguish *advance* (`1`) from *back* (`0`) from *end chat* (`0xFF`), and read the
message type in field 2 to know which kind of box is being answered.

Paging therefore works, but the server is **inferring**, not reading: it must remember what
the last box was. Concretely:

* it sent `next = 1` and got `1` → the user pressed Next → send the next line;
* it sent `next = 0` and got `1` → the user pressed OK on the last line → the Say run is over;
* it got `0` → Prev (only reachable if the server set `prev = 1`) → resend the previous line;
* it got `0xFF` → END CHAT → drop the conversation, send nothing.

Say that out loud in the server code, because it is the one place here where the protocol
does not tell you what happened.

---

## 3. The yes/no box: message type **3** (and `0x10`)

### 3.1 How it was found

`BtYes`/`BtNo` are `0x143a48430`/`0x143a48438`. `tools/dataref.py` gives two readers each,
both in `FUN_142a66070` (control: `BtNext` returns 6 readers across 4 functions, so the tool
speaks). `FUN_142a66070` is **index 1** of `CreateLayout`'s table, i.e. `[ui+0x2a8] == 1`.
Enumerating the `msgType` argument at all **134** call sites of `FUN_142a61900` gives the
handlers that pass `1`: `FUN_141f70820`, `FUN_141f70b30`, `FUN_141f75960` — reached from
`0x055B` message types **3, 0x10, 0x11, 0x1b, 0x1c**. [L]

Type **1** is *not* the yes/no box: `FUN_141f6fe40` passes `0` (`141f6ff9e`), i.e. the Say
layout. `npc-dialogue.md` §2.4 called it "an ask-style dialog"; it is a Say variant that
neither echoes the handle nor carries an `echo` field.

### 3.2 Type 3's buttons

`FUN_142a66070` [L]:

| button | drawn when | slot | id | action byte |
|---|---|---|---|---|
| Yes | always | `[ui+0x5f8]` | **6** | **1** |
| No | always | `[ui+0x608]` | **7** | **0** |
| Close | `!(flags & 1)` | `[ui+0x618]` | `2` | **0xFF** |

Ids at `142a66630` / `142a66709` / `142a667e4`. `FUN_142a59f50`'s case for kind 1
(`142a5a122`) accepts exactly `{6, 7}` and passes them through unchanged. The answer maps
them at `141f70a74`: `edi == 6 -> 1`, `edi == 7 -> 0`, else `0xFF`. [L]
**No remap here** — a yes/no box is unambiguous.

The captions depend on two things [L]:

```
142a660ab  if ([ui+0x2d4] == 0)          -> BtYes / BtNo          (message type 3)
142a660b4  else if (flags & 0x10)        -> BtQStart / BtQAfter
                                    else -> BtQYes / BtQNo        (message type 0x10)
142a66123  if (flags & 0x100)            -> BtQCYes / BtQCNo      (overrides all of the above)
```

`[ui+0x2d4]` is a literal baked into the dispatch stub: `0` for type 3 (`141f6f515`), `1`
for type 0x10 (`141f6f53d`). Those are exactly the `BtQYes, BtQNo, BtQStart, BtQAfter`
strings `npc-dialogue.md` §1.2 found sitting next to `ask` and `stop` in the quest string
table. [L]

### 3.3 Type 3's body — it is shorter than Say's

`FUN_141f70820` makes exactly **two** reads [L]:

| # | prim | at | meaning |
|---|---|---|---|
| 9 | `u32` | `141f70869` | **only if `flags & 0x04`** — overrides the speaker id |
| 10 | `str` | `141f70879` | the text (`u16` byte length, then that many bytes) |

No `echo`, no `prev`/`next` (it calls `FUN_142a62cb0` instead of `FUN_142a62bf0`, leaving
both zero), no trailing `u32`. Full packet:

```
opcode 0x055B

  xx xx xx xx   u32  handle          NOT echoed by type 3 - see §4
  00            u8   -               discarded
  tt tt tt tt   u32  speakerTemplate the NPC's Npc.wz template
  00            u8   hasOverride     0
  03            u8   messageType     3  (or 0x10 for the BtQYes/BtQNo captions)
  00 00         u16  flags           0. bit 0x04 adds a speaker u32; bit 0x01 hides Close
  00            u8   -               -> [ui+0x2a4]
  ll ll         u16  length          BYTE count
  ...           str  the question
```

Its answer is **6 bytes**: `u32 0, u8 3, u8 action` (`u32 0, u8 0x10, u8 action` for 0x10).
Confirmed by the shape extractor: `141f70a4a in FUN_141f70820 -> u32,u8,u8`.

### 3.4 the owner's "Accept" was not an Accept

The Heena box was `messageType = 0`, `flags = 0`, `prev = 0`, `next = 0` — from the server's
own bytes in `world.log`. That layout draws **`BtOK` and `BtClose`**, nothing else, and the
answer it produced is byte-identical in structure to Robin's. Whatever the artwork read,
**the click was OK on a Say box, and the server learned only "advance"**. [D]

To get a real accept/decline pair the server has to send **type 3** (or **0x10**), and then
the `0` / `1` in the action byte means No / Yes with no ambiguity.

---

## 4. `handle` and `echo`: choose them freely, but do not rely on them

Both are read once and written once. Neither is compared, range-checked, or used as an
index. [L]

* **`handle`** (head field 1, read at `141f6f382` into `r15d`). Every use of `r15d` in
  `FUN_141f6f350`: the type-0 stub (`141f6f4c9`) and the five stubs for types `0x42..0x46`
  (`141f6f977/987/997/9a7/9b7`). Nine `r15` mentions in the whole function, all of them
  moves. Inside the Say handler it is stored at `141f6fb53` and read at `141f6fd42` —
  those two lines are its complete lifetime.
* **`echo`** (Say body field 9, read at `141f6fb6b`): stored at `141f6fb70`, read at
  `141f6fd5a`. Two lines, nothing else.

**But the asymmetry matters more than the freedom.** Only **type 0** echoes them; type 1
writes `0` (`141f70032`), type 2 writes `0`, type 3 and 0x10 write `0` (`141f70a50`), type
0x11 writes `0` (`141f70d1b`). The capture cannot distinguish "the client echoed our 0"
from "the client wrote a literal 0" — the server only ever sent 0 — but the listing does,
and it says both are true depending on the type.

**Consequence for the state machine:** you may key a Say conversation on `handle`, but the
moment you send a yes/no the correlation token disappears. **Key the conversation on the
session, not on a token**, and use the echoed message type in field 2 as the sanity check.

---

## 5. How a conversation ends — and the answer to "is this poisoning later requests?"

### 5.1 Nothing waits for a reply to `0x00F3`

After the send the Say handler does `FUN_140da25d0(scriptMan + 0x10)` (`141f6fda3`), which
drops the script manager's reference to the dialog, then unwinds; the re-entrancy latch
`[scriptMan+8]` is cleared at the common exit `141f6f9c2` (`mov [rdi+8], r13d`, `r13d = 0`).
No timer, no retry, no pending state. [L] **If the server never answers a `0x00F3`, the box
simply closes and nothing else happens.**

### 5.2 `0x00F3` is *not* one of the 37 `player->[0x2330]` setters

This was the sharpest risk in the brief, and the answer is clean. A byte scan of `.text` for
the displacement `0x2330` finds 194 raw occurrences; decoding each as an instruction and
bucketing by band [L]:

| band | stores/tests of `+0x2330` |
|---|---|
| script-message handlers `0x141f6f000..0x141f78000` | **0** |
| script UI band `0x14127d000..0x141282000` | **0** |
| `CUIScriptMsg` band `0x142a55000..0x142a90000` | **0** |
| **control**: `FUN_142cc5b00`, the known setter | **2** (`142cc5b5d` test, `142cc5f01` store) |

The control fires, so the scan speaks. **The NPC-dialogue path never arms the
one-request-outstanding latch.** An unanswered `0x00F3` costs a dead conversation, not a
silently poisoned client. `npc-click.md` §4.3's hazard is real but does not apply here.

### 5.3 The one thing that *does* end a conversation badly

Field entry. `FUN_142caa4e0 -> FUN_141f6f200(scriptMan)` tears the script state down. Never
send `0x055B` with or just before a `SetField`. (Unchanged from `npc-dialogue.md` §4.)

### 5.4 One box at a time — a sharpening of `npc-dialogue.md` §2.3

`npc-dialogue.md` says the latch "is set and cleared inside one dispatch, so the server
needs no lock". The set (`141f6f490`) and clear (`141f6f9c2`) are indeed one dispatch — but
**that dispatch blocks inside the modal `Show()` for as long as the box is on screen.** So
the latch is held for the whole conversation step, not for a few microseconds. The busy
branch discards everything except type `0x47` and writes an ELog line
(`141f6f416: ecx = 0x21000003`, `edx = 0xa3`). [L]

**I could not settle whether a `0x055B` arriving mid-modal is discarded or merely queued.**
That turns on whether the modal pump re-enters packet dispatch. `FUN_142cbaa80` has zero
direct callers (it is reached virtually), a 3-level direct-call walk from `FUN_142c45e50`
does not reach `FUN_1415d59b0`, and the socket function the pump *does* reach,
`FUN_1415d60e0`, is **virtualised** — `1415d60f1 jmp 0x144add569`, into the Themida VM. A
static answer is not available by this route. [L]

The safe rule is the same under either outcome and it is what a request/response state
machine wants anyway: **send one `0x055B`, wait for its `0x00F3`, then send the next.**

Also note that message type `0x47` — the documented "force close" — acts on the global UI
at `[0x143aca0f0]` via `FUN_1410dea80`, whose answer builder `FUN_1410de9a0` writes message
types `0x42..0x46`. That is a **different dialog family**, not `CUIScriptMsg`. Treat "0x47
closes the script box" as unproven. [L]+[D]

---

## 6. Reference: the answer shape of every script message type worth using

### 6.1 The `0x055B` message-type table (71 entries at `0x141f6f9f4`)

Re-derived independently: entry `i` is `0x140000000 + tbl[i]`; 25 entries point at the common
exit and do nothing, at exactly the indices `npc-dialogue.md` §2.4 lists. [L]

| type | handler | dialog kind | body after the head | `0x00F3` answer |
|---|---|---|---|---|
| **0** Say | `FUN_141f6fb20` | 0 | `u32 echo, [u32 spk if flags&4], str, u8 prev, u8 next, u32` | `u32 handle, u8 0, u32 echo, str text, u8 action` |
| 1 | `FUN_141f6fe40` | 0 | `str, u8 prev, u8 next, u32 (dropped)` | `u32 0, u8 1, u8 action` |
| 2 | `FUN_141f70110` | 0x11 | `u8, str` | `u32 0, u8 2, u8 action` |
| **3** yes/no | `FUN_141f70820` | **1** | `[u32 spk if flags&4], str` | `u32 0, u8 3, u8 action` |
| **0x10** yes/no, quest captions | `FUN_141f70820` | **1** | same as type 3 | `u32 0, u8 0x10, u8 action` |
| 0x11 | `FUN_141f70b30` | **1** | `u32 (dropped), str` | `u32 0, u8 0x11, u8 action` |
| 0x1b | `FUN_141f75960` | **1** | `u32, str, u32, u32, u32, u32, u8` | `u32, u8, u8 action` |
| 0x42..0x46 | `FUN_141f765d0`… | — | — | these five also receive `handle` |

All bodies [L] from the read primitives in each handler; all answer shapes [L] from the write
extractor of §1.2. `0x1406e8ae0` = `u8`, `0x1406e8b80` = `u16`, `0x1406e8c20` = `u32`,
`0x1406e9050` = string (`u16` **byte** count).

### 6.2 The `flags` word, bit by bit

| bit | effect | at |
|---|---|---|
| `0x01` | **suppresses the Close / END CHAT button** in both layouts | `142a65e6c`, `142a6672c` |
| `0x04` | an extra speaker `u32` in the body (types 0 and 3) | `141f6fb7a`, `141f70860` |
| `0x06` | sets `[ui+0x6b8]`, which shifts the button x positions | `142a62ba6` |
| `0x10` | type 0x10 only: `BtQStart`/`BtQAfter` instead of `BtQYes`/`BtQNo` | `142a660b4` |
| `0x20` | style 1 → the alternate Say construction, **no Prev button** | `141f6fbbe`, `142a65775` |
| `0x80` | style 2 → same alternate construction | `141f6fbcf` |
| `0x100` | yes/no only: `BtQCYes`/`BtQCNo` | `142a66123` |

`FUN_142a62ba0` also ORs bit 0 in by itself when a global gate holds (`142a62bd5`), so the
client can drop the Close button on its own. **Send `flags = 0` unless you want one of
these.** [L]

### 6.3 The `result == 3` branch

Every handler skips the answer entirely if `Show()` returns 3. I could not construct a way
for that to happen to a Say or yes/no box, and I looked three ways [L]:

* no `EndDialog` caller in the image passes a literal 3 (31 call sites enumerated);
* no literal `3` is stored to any `+0x230` (a byte scan of `.text` finds literal stores of
  0, 1, 5, `0x1f`, `0x64`, `0xffffffff` — so the scan can see literal stores);
* no button in any `CreateLayout` layout is created with id 3 (ids found: `1, 2, 6, 7, 0xc,
  0x18, 0x2000, 0x2001, 0x3e8, 0x3ea, 0x3eb`).

That third scan **under-reports** — it matches only `mov r8d, imm` followed by `call reg`
within 8 instructions, and it found no buttons at all for dialog kinds 2, 6, 7, 9, 10, 15,
16 and 18–29, yet `npc-click.md` §2.1 observed kind 6 returning `1` and `0x2001`. So treat
"3 is unreachable" as **[D] with a soft edge**: nothing I can find produces it, and the
consequence if it ever does is a silently unanswered conversation, which §5.1 says is
survivable.

---

## 7. The state machine, spelled out

For a quest whose `Say."0"` is four numbered lines followed by a `yes` / `no` prompt.
`S` = the server's per-session conversation state; `T` = the NPC's template id.

### 7.1 The sequence

```
1.  client -> 0x0151   01 e8030000 01000000 <x> <y> 00000000
                       tag 1, quest 1000, npc template 1
    (or 0x00F2 for a scriptless NPC; map its objectId -> template first)

    S = { npc: T, node: Say.0, line: 0, awaiting: Say }

2.  server -> 0x055B   Say, line 0, prev = 0, next = 1
      00000000  handle       0        (free; only type 0 echoes it)
      00        -
      01000000  speaker      T = 1
      00        hasOverride
      00        messageType  0
      0000      flags
      00        -
      00000000  echo         0        (free)
      llll      length
      ....      Say.0.0
      00        prev         0        <- first line, no way back
      01        next         1        <- "there is more"
      00000000  -

3.  client -> 0x00F3   00000000 00 00000000 <llll+text> 01
                       handle 0, msgType 0, echo 0, the text echoed, action = 1

    action 1 AND the box we sent had next = 1  ->  advance.   S.line = 1

4.  server -> 0x055B   Say, line 1, prev = 1, next = 1
5.  client -> 0x00F3   ... action = 1      (or 0 = Prev -> S.line = 0, resend)
6.  server -> 0x055B   Say, line 2, prev = 1, next = 1
7.  client -> 0x00F3   ... action = 1
8.  server -> 0x055B   Say, line 3, prev = 1, next = 1
9.  client -> 0x00F3   ... action = 1      S.line = 4 -> the Say run is exhausted

10. server -> 0x055B   the yes/no prompt          <- messageType 3, NOT 0
      00000000  handle       0        (type 3 will NOT echo it)
      00        -
      01000000  speaker      T = 1
      00        hasOverride
      03        messageType  3        (0x10 for BtQYes / BtQNo captions)
      0000      flags        0
      00        -
      llll      length
      ....      the question                     <- no prev/next, no trailing u32

    S.awaiting = YesNo

11. client -> 0x00F3   00000000 03 01                      (6 bytes)
                       handle 0, msgType 3, action = 1  ->  YES

    -> server sends Say."0".yes.0 as messageType 0 again, and pages it the same way.
       action = 0 would be NO      -> Say."0".no.0
       action = 0xFF               -> the user closed the box; send nothing.
```

### 7.2 The dispatch the server needs, in words

On inbound `0x00F3`, read `u32 handle`, then `u8 msgType`, then **branch on `msgType`**
before reading anything else — the rest of the body differs per type:

```
msgType 0  (Say):     u32 echo, u16 len + len bytes, u8 action
msgType 3  / 0x10:    u8 action
msgType 1  / 2 / 0x11: u8 action
```

Then, keyed on `S.awaiting`:

| `S.awaiting` | action | meaning | do |
|---|---|---|---|
| Say, sent `next = 1` | `1` | Next | advance one line |
| Say, sent `next = 0` | `1` | OK on the last line | the Say run is over — send the prompt, or end |
| Say, sent `prev = 1` | `0` | Prev | go back one line |
| Say | `0xFF` | END CHAT | drop `S`, send nothing |
| YesNo (3 / 0x10) | `1` | **Yes** | send the `yes` branch |
| YesNo | `0` | **No** | send the `no` branch |
| YesNo | `0xFF` | closed | drop `S`, send nothing |

The action byte is **signed**: decode it as `i8` so `0xFF` reads as `-1`.

Three rules that are cheap to get wrong:

* **Never send the next `0x055B` before the `0x00F3` for the previous one arrives** (§5.4).
* **Never send `0x055B` with or just before a `SetField`** — field entry destroys the script
  state (§5.3).
* **`speakerTemplate` must be a real `Npc.wz` template.** `0x0151` hands you the template
  directly; `0x00F2` hands you the *object* id and you must map it (`npc-click.md` §3.2).

---

## 8. What I could not settle

* **Whether a `0x055B` sent mid-modal is discarded or queued** (§5.4). The deciding function
  is virtualised. The safe rule is unaffected.
* **What message type `0x47` closes.** It acts on `[0x143aca0f0]`, whose answer builder emits
  types `0x42..0x46`. That is not `CUIScriptMsg`. `npc-dialogue.md` §2.4's "force-close the
  open dialog" is a plausible reading of a function that is demonstrably about a *different*
  dialog family, and I would not build on it.
* **Whether `Show()` can ever return 3** (§6.3). Three searches say no; one of them
  under-reports and I have said how.
* **The 25 `dialogKind`s I did not open.** I read kinds 0 and 1 in full and skimmed 3/4/5.
  The `CreateLayout` and button-click tables for all 30 are in §6 / §3, but only kinds 0 and
  1 are traced end to end.
* **No quest-result packet is identified**, so answering the yes/no still will not advance
  the client's quest state from 0 to 1. `npc-dialogue.md` §5 already flags this and names
  `FUN_142d60d40` (`case 0x00AC`) and `FUN_142d43ee0` (`case 0x0089`) as the leads. Unchanged.
* **`[ui+0x2b4]`**, which `FUN_142a62bf0` bumps by `0x12` when `prev || next` and
  `FUN_142a6fae0(ui)` is positive. It smells like an in-box text-scroll counter. Not chased.

---

## 9. Against the companion documents

**Corroborated**

* `npc-dialogue.md` §2.3 / §2.4's head and Say body: now confirmed against a **real 42-byte
  `0x055B` the client accepted**, parsing to zero remainder (§1.1). That is the strongest
  evidence either document has.
* §2.3's "the handle is passed only to types 0 and `0x42..0x46`": re-derived from every use
  of `r15d` in `FUN_141f6f350` — six moves, at exactly those six stubs (§4).
* §2.4's 71-entry table and its 25 dead slots at the listed indices: reproduced
  independently (§6.1).
* §2.4's field 12 / 13 = prev / next: confirmed by the layout tests at `142a65b99` /
  `142a65cbc` and by the capture (§2.4).
* `npc-click.md` §4.3's `player->[0x2330]` latch: real, and **not** on this path (§5.2).

**Sharpened, not retracted**

* §2.3's "the server needs no lock and no acknowledgement before sending another `0x055B`".
  The latch really is one dispatch — but the dispatch blocks for the life of the box. Use a
  strict request/response discipline (§5.4).
* §4's "the client acknowledges — for Say the client's own **immediate-answer path** writes
  `u32 handle, u8 0, u32 echo, str text, u8 result`". That is not an immediate-answer path:
  `vtbl[0x130]` is a blocking modal, so this *is* the normal answer, and the capture proves
  it fires (§1.2). The field list was right.
* §5's "the dialog-driven `0x00F3` was not traced to a single builder; `FUN_1410de8c0` is
  the closest candidate". `FUN_1410de8c0` belongs to the `0x42..0x46` family, not to
  `CUIScriptMsg`; there is no separate dialog-driven builder for Say. `FUN_141f6fb20` is it.

---

## 10. Instrument notes

* **A capture with all-zero fields cannot fix a field order.** The nine leading zeroes of
  `0x00F3` parse three different ways. The builder settled it in one read (§1.3). Where a
  measurement and a listing overlap on zeroes, the listing is the instrument.
* **Naming a script `dis.py` shadows the stdlib `dis` module** and breaks `import capstone`
  with `AttributeError: module 'dis' has no attribute 'COMPILER_FLAG_NAMES'`. Renamed to
  `mdis.py`.
* **`0x1406e8f00` is a 5-byte thunk: `jmp 0x1406e8c20`.** Disassembling it without `.pdata`
  bounds runs past the thunk into the *next* function at `0x1406e8f10`, which advances the
  packet cursor by **8**. That very nearly produced a "correction" to `npc-dialogue.md`
  §2.3 row 5 claiming head field 5 is a `u64`. It is a `u32`; the document is right. Bound
  every dump by `.pdata` — including the ones that look too small to matter.
* **An anchored regex against annotated listing lines silently matches nothing.** The write
  extractor of §1.2 used `re.search(r'call\s+(0x[0-9a-f]+)$')` while `mdis.py` appends
  `; <<< W_u32` to exactly the lines that matter. It reported an empty write sequence for
  **all 57** builders — a clean, confident, uniform wrong answer. Fixed, and it now runs a
  positive control (`0x00F2` site A) before printing anything.
* **A sweep for a common structure offset drowns.** `+0x230` matches 1681 sites in `.text`,
  overwhelmingly `[rbp+0x230]` stack frames. The useful filter was not the offset but the
  *class*: the two hits adjacent to `FUN_14177f640` were the answer.
* **`tools/callers.py` cannot see virtual dispatch** and says so honestly with a `0`:
  `FUN_142cbaa80` and `FUN_1415d59b0` both report zero callers. Do not read that as dead
  code — it is how every `OnPacket` in this binary is reached.
* Controls that were run before any negative was reported: `0x00F2` site A reproduced
  instruction for instruction; `callers.py` on `0x1402fa9a0`; `dataref.py` on `BtNext`
  (6 hits) before believing `BtYes` (2 hits); `FUN_142cc5b00` for the `+0x2330` scan.
