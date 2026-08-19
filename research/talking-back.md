# Talking back: a chat line, and a logout

Two questions of the same kind — *how does the server tell the client something?* — raised
from the owner's run of 2026-08-19.

Everything below was read from `client-patched/MapleStory.exe` with capstone plus the
repo's non-Ghidra tools, and from `world.log`. **No Ghidra**, no client run.

Labels: **[L]** read from the listing or a capture · **[D]** derived from labelled facts ·
**[I]** inferred, not established.

---

## Summary

| | opcode | body | status |
|---|---|---|---|
| **a line in the chat window** | **`0x00BB`** | `u8 force, str text` | **[L]** — handler read end to end, 184 bytes |
| **log out (answer to `0x01BE`)** | **`0x0106`** | `str reason` (**must be non-empty**) | **[L]** — handler read end to end, cross-checked against the jump table |

And one thing that is not a packet but changes priorities:

> **After the client sends `0x01BE`, `SetField` stops working.** `0x01BE`'s builder sets
> `world->[0x33f4] = 1`, and the very first thing the `SetField` handler does — before it
> reads anything but the 8-byte FILETIME — is test that byte and return. Only `0x0106`
> clears it. So the owner's session after clicking Log Out was not merely unresponsive to the
> button: it could no longer be warped anywhere either, silently. **[L]**

---

# 1. Putting text in the chat window

## 1.1 The answer

**Inbound `0x00BB`**, handled by `FUN_142d95630` (184 bytes, `0x142d95630 .. 0x142d956e8`).

```
u8    force        0 = show only if nothing has been shown since the last field entry
                   non-zero = always show
str   text         u16 length, then bytes.  Must be non-empty or nothing happens.
```

Nothing else is read. The whole handler, in order: **[L]**

```asm
142d95648  call 1406e8ae0                  ; u8 -> eax          "force"
142d95650  cmp  dword [rdi+0x28c8], 0      ; the once-per-field latch
142d95657  je   142d9565d                  ;   latch clear -> always proceed
142d95659  test eax, eax
142d9565b  je   142d956d8                  ;   latch set AND force==0 -> return, silently
142d95665  call 1406e9050                  ; str -> world->[0x28c0]  (old one released first)
142d956ac  mov  rax, [rdi+0x28c0]
142d956b3  test rax, rax / je  142d956d8   ; NULL   -> return
142d956b8  cmp  byte [rax], 0 / je 142d956d8 ; empty -> return
142d956bd  mov  edx, 7
142d956c2  lea  rcx, [rdi+0x28c0]
142d956c9  call 1415eca30                  ; <- the chat line
142d956ce  mov  dword [rdi+0x28c8], 1      ; latch
```

`rdi` is the dispatcher's `this`, i.e. `DAT_143aa84a0`, the world object that
`research/msexe-gamestage-dispatch.md` already identifies. **[L]**

**The smallest body that puts a plain server notice on screen:**

```
01  <u16 len>  <text bytes>
```

e.g. `!map 9000000` refused, 25 characters:

```
01 19 00 4d 61 70 20 39 30 30 30 30 30 30 20 64 6f 65 73 20 6e 6f 74 20 65 78 69 73 74
^^ force=1
   ^^^^^ u16 length 0x0019
```

**Always send `force = 1`.** With `force = 0` the *first* line after each field entry
shows and every later one is dropped without a sound: `world->[0x28c8]` is reset to 0 by
`FUN_142caa4e0`, which `docs/opcodes.md` already records as the function field entry runs.
**[L]** — the reset site is `142caa769`, `mov dword [rdi+0x28c8], edi` with `edi = 0`.

## 1.2 How this was found — the "type 7" instrument

Not by guessing at shapes. `research/msexe-gamestage-cases.txt` gives all 273 inbound
cases; I built the **body shape of every one of them** (read primitives in address order,
inline cases from `research/msexe-gamestage-dispatch.c`, forwarded cases by disassembling
the handler) and then looked for what actually prints.

The printer is **`FUN_1415eca30(char **text, u16 type)`**. It is the client's universal
"post a UI message" call — **1133 call sites**, so the function alone proves nothing. The
**`type`** argument is what discriminates, and **type 7 is the chat window**:

| evidence | where |
|---|---|
| `string_id(&s, 0x533)` then `FUN_1415eca30(&s, 7)`. String `0x533` decrypts to **`[Welcome] Welcome to MapleStory!!`** | `14209f690`, in `FUN_14209ee50` **[L]** |
| the received `UserChat` message is posted with type 7, gated on one of the packet's trailing flag bytes — the `onlyBalloon` position | `142783c9e`, in `FUN_1427834b0` **[L]**, the gate's meaning **[I]** |
| two more inbound handlers format a string and post it with type 7 | `142d61933`/`142d62239` in `FUN_142d60d40` (opcode `0x00AC`) **[L]** |

The bracketed `[Welcome]` line is the decisive one: that is a chat-log line in this game,
not a dialog, and nothing else in the client would carry it.

Enumerating every `call FUN_1415eca30` in `.text` by a rel32 scan (1133 sites) and reading
the nearest preceding `mov edx, imm32` gives **17 sites with type 7**:

```
1415b9bae in 1415b9670     14209f6a8 in 14209ee50 (the [Welcome] line)
141847231 in 1418463c0     142102b80 in 1421028f0
1418484db in 1418473d0     14210324c in 142102df0
141848dc4 in 1418486b0     142103c89 in 1421033b0
142783c9e in 1427834b0     14210443a in 142104020
1427856fa in 142784970     142ce061a in 142ce0130
142cf16df in 142cf0860     142d59673 in 142d59360
142d61933 in 142d60d40     142d62239 in 142d60d40
142d956c9 in 142d95630   <- 0x00BB
```

**Instrument caveat, stated because it bounds the answer:** of the 1133 sites, **167 load
`edx` from a register**, so my type histogram is a *lower* bound. There may be more type-7
posters than 17. It cannot make `0x00BB` wrong — that site is an immediate — but it means
"`0x00BB` is the only minimal one" is [D], not [L].

## 1.3 The other ways text reaches the chat window

Intersecting the 17 sites with the inbound handler set:

| opcode | handler | shape | what it is |
|---|---|---|---|
| **`0x00BB`** | `FUN_142d95630` | `u8, str` | **a bare notice line. Use this one.** |
| `0x00AC` | `FUN_142d60d40` | `u8, u8, str, str, …` 8420-byte handler | multiplexed; formats a name into the line. Party/guild/friend-shaped **[I]** |
| `0x0089` | `FUN_142d43ee0` → `FUN_142d59360` | 30 reads, `u8,u32,str,u16,…` | large record packet; the chat post is one branch of it |
| `0x0226` | user pool → `FUN_1427847a0` → `FUN_1427834b0` | `u32 userId` (read by the pool) then `u8, u32, str, str, <list>, u8, u8, u8` | **`UserChat`** — the balloon over a character *and* the chat line |

**`0x0226` is the wrong tool for GM feedback** even though it is the true chat packet: the
pool looks the id up with `FUN_1429b6c90` and, when no `CUser` is found, drops the packet
(`FUN_1429bafb0` at `1429bb0a6`). The server would have to spawn the player into the user
pool first, which this project has not done. **[L]**

The user pool is `0x224..0x39F` on singleton `[0x143AC1B90]`, reached from
`CField::OnPacket`; `0x226..0x292` all route through `FUN_1429bafb0`, which reads a `u32`
user id and then dispatches through a 0x51-entry jump table at `0x1429bb5d0`. `0x0226` is
the first entry and is the only opcode the router special-cases before the id read. **[L]**
That ordering — `0x224` enter, `0x225` leave, `0x226` chat — matches the classic
`CUserPool` enum exactly, which is why I am confident `0x0226` is `UserChat` **[D]**.

## 1.4 What I could not settle

* **Which tab the line lands in, and its colour.** `FUN_1415eca30` fills a message record
  with five empty strings, a `0xFF` byte and `-1`, then calls `FUN_1415a87a0`. The `0xFF`
  and the `-1` are colour/tab-shaped, but I did not read `FUN_1415a87a0` far enough to say.
  `0x00BB` gives no control over either — it carries one flag and one string.
* **A coloured / pop-up variant.** I did not find a `BroadcastMsg`-style packet with a
  full type switch in the 273 cases. Handlers `0x00C1`–`0x00C7` are a string-heavy cluster
  (`str`, `str`, `u8+str`, `str,str` ×3) that looks like the whisper/find/message family
  and is the place to look next; I did not read them.
* **`0x00AC`'s and `0x0089`'s exact meanings.** Only that they post type-7 lines.

## 1.5 Recommendation

Use **`0x00BB` with `force = 1`** for GM-command feedback. It is the smallest thing in the
inbound space that does exactly what the owner asked for: one packet, one flag, one string, one
line of text, no prerequisites, nothing else touched, and the handler releases the previous
string properly so repeated sends are safe.

---

# 2. Log out

## 2.1 `0x01BE` is the logout request — confirmed, not coincidence

`research/msexe-send-opcodes.txt` line 1120 gives the builder as `FUN_142cfb470`
(`142cfb470 .. 142cfb4f5`, 133 bytes). It writes **no fields at all**, which matches the
capture's 0-byte body: **[L]**

```asm
142cfb493  mov  byte [rcx+0x33f4], 1     ; <- the latch, see 2.4
142cfb49a  mov  edx, 0x1be
142cfb4a4  call 1406ed520                ; OutPacket ctor
142cfb4af  call 1415d01c0                ; send
142cfb4b4  mov  ecx, 1 / call 142aa27f0  ; telemetry stub
```

The confirmation is its caller. `FUN_142d05490` (`142d05490 .. 142d055bf`): **[L]**

```asm
142d054af  mov  edx, 0x17c2
142d054b4  call 1408a9e40                ; string id 0x17C2 = 6082
142d054f0  call 142a269c0                ; a Yes/No dialog
142d054f5  cmp  eax, 6                   ; IDYES
142d054f8  je   142d054ff                ;   ... otherwise do nothing
142d05591  call 142cfb470                ; -> send 0x01BE
```

and `tools/dump_stringids.py --id 6082` gives:

> **`Are you sure you want to logout?`**

(6081 is `Are you sure you want to quit?`, a different button.) A confirm dialog reading
"Are you sure you want to logout?" whose Yes branch is the only thing that sends `0x01BE`
settles it.

A second, corroborating shape match: the character-screen **`0x0082` "leave world"**
builder `FUN_141b3bfd0` ends with the identical three calls —
`OutPacket_ctor(op); send; FUN_142aa27f0(1); dtor` — and carries no body either. `0x01BE`
is the in-game sibling of the packet the login server already answers. **[L]**

### A gap in `msexe-send-opcodes.txt` worth recording

There is a **second** builder for `0x01BE` that the table does not list:
**`FUN_142d3c730`** (`142d3c730 .. 142d3c78d`), byte for byte the same shape
(`mov byte [rcx+0x33f4],1; mov edx,0x1be; ctor; send; dtor`). **[L]** It has **0 `call
rel32` callers and no qword or jump-table reference anywhere in the image** — checked with
`callers.py` and a raw qword/dword-RVA scan against a positive control — so it is reached
from the Themida-virtualised region, the same signature `handler_root.py` documents.

The table's own warning ("absence means the builder is virtualised") turns out to
understate the problem: here the builder is **present in `.text` and still missing from the
table**. Treat `msexe-send-opcodes.txt` as a lower bound on builders, not a census.

## 2.2 The answer the server must send: `0x0106`

Case `0x0106` of the channel dispatcher `FUN_142cbaa80`. Read from the **jump table**
(`0x142cbd9d0`, `0x32b` entries, index = `opcode - 0x70`, entries are offsets from
ImageBase — `0x0106` → `0x142cbc169`) and then out of the listing, so the mapping does not
rest on the decompiler: **[L]**

```asm
142cbc169  mov  ecx, 1 / call 142aa2810     ; telemetry stub
142cbc17b  call 1406e9050                   ; str -> [rsp+0x70]     <- the ONLY read
142cbc181  test rcx, rcx      / je  ...     ; NULL  -> do nothing
142cbc18b  cmp  byte [rcx], 0 / je  ...     ; empty -> do nothing
142cbc1ad  lea  rcx, [rdi+0x31f0]           ; world->[0x31f0] = the string
142cbc1db  call 142d18520(world, 2)         ; "leaving, reason 2"  -> outbound 0x0422
142cbc1e3  call 142d3c670(world)            ; <- the teardown
```

The decompiler (`research/msexe-gamestage-dispatch.c` line 913) agrees field for field, and
the packet-read count is **one** in both. That is the cross-check `CLAUDE.md` asks for.

```
Body of 0x0106:

str   reason       u16 length, then bytes.  MUST be non-empty.
```

**Smallest working body:** a one-character string.

```
01 00 2e            ; len 1, "."
```

A real one is no harder and is probably what the field is for:

```
0a 00 4c 6f 67 67 65 64 20 6f 75 74      ; len 10, "Logged out"
```

**An empty string is a no-op.** The handler returns without doing anything at all, which
would look exactly like the packet not being understood.

## 2.3 What `0x0106` actually does — and the reconnect question

`FUN_142d3c670` (`142d3c670 .. 142d3c727`, 183 bytes), in order: **[L]**

```asm
142d3c698  call 14252d560(world->[0x29e0])
142d3c6bc  call 1425fd4f0(world->[0x29f0])
142d3c6ca  call 142c48ca0(DAT_143AC1898, 0)   ; connection subsystem — see below
142d3c6db  call 14019b780(&pool, 0x278)       ; allocate a 0x278-byte object
142d3c6ed  call 141b219e0(obj)                ; construct it
142d3c6f8  call 14209ee50(obj, 0)             ; install it as the current stage
142d3c6fd  mov  byte [rbx+0x33f4], 0          ; clear the logout latch
142d3c722  jmp  142d0eda0(world)
```

**The object it constructs is the login stage.** `FUN_141b219e0` installs three vtable
pointers — `0x1433FD668`, `0x1433FD6C8` and two more — and all three tables contain
`FUN_141B25F30`, the login stage's `OnPacket`, **at the same address** `0x1433FD7A0`
(`0x1433fd540 + 0x260` = `0x1433fd668 + 0x138` = `0x1433fd6c8 + 0xd8`). They are the
multiple-inheritance sub-vtables of one class, and that class is the login stage this
project already identified. **[L]** for the arithmetic, **[D]** for the conclusion.

`FUN_142d3c670` has exactly **three** callers: `FUN_142c42f30` (`main`), case `0x0106`, and
`FUN_142cd98a0`. And outside the login subsystem's own startup (`FUN_141b73e40`,
`FUN_141b740b0`), **`FUN_142d3c670` is the only function in the image that constructs a
login stage.** **[L]** That is what makes `0x0106` the right answer rather than a
plausible one.

### Reconnect, or tear down in place?

**Tear down in place. The answer starts no new connection.** Three independent lines: **[L]**

1. `FUN_142d3c670` builds no `sockaddr` and never calls `FUN_142caa360`, the connect.
2. `FUN_142caa360` has exactly **two** callers in the whole image — `FUN_141b36f60` (the
   login stage's `0x0011` migrate) and `FUN_1415d8c00` — and neither is reachable from
   `FUN_142d3c670`.
3. A `dataref` scan for the `htons` IAT slot `0x143262e48` finds **9** sites image-wide.
   None is in any of the 273 channel handlers. (Positive control: the scan finds
   `141b37ea5` in `FUN_141b36f60`, the migrate this project already documented.)

So the shape is: the client destroys the world, builds a fresh login stage in-process, and
carries on. **[D]**

**What I could not settle: whether the channel socket is closed.** `FUN_142c48ca0` sets a
global to `0xA` and then, 74 bytes in, does `jmp 0x144ec0366` — into the Themida region.
Static reading stops there. So I cannot say whether the client keeps the channel socket, or
resumes on the login socket it has held open all session (`docs/opcodes.md` records the
`ELog` arriving on `login.log` at 16:48 *during* a channel session, so that socket is
alive), or opens something new.

**That is the one thing a client run should watch**, and it is cheap: send `0x0106` and see
which log the next packet lands in — `login.log` or `world.log`.

### The alternative, if the in-place teardown misbehaves

**`FUN_1415d8c00`** (`1415d8c00 .. 1415d8e6c`) *is* a genuine channel-side migrate handler:
**[L]**

```
u8   ok        ; 0 -> raises 0x21000002 at line 0x63A and does nothing
u32  ip
u16  port
```

then it installs a fresh stage, builds the `sockaddr` and calls `FUN_142caa360` — a real
reconnect. **Its opcode is unknown**: like `FUN_142cd98a0` it has no callers, no vtable
slot and no jump-table entry, so it is dispatched from the virtualised loop. If the
`0x0106` teardown leaves the client talking to the wrong socket, finding this opcode is
the next move.

## 2.4 The `0x33f4` latch — and why this is more urgent than a dead button

`0x01BE`'s builder sets `world->[0x33f4] = 1`. A byte scan for that displacement across
`.text` finds **17** instructions, of which four matter: **[L]**

| where | what |
|---|---|
| `142ca752d` in `FUN_142ca5c50` | the world-object constructor, initialises it to 0 |
| `142cfb493` in `FUN_142cfb470` | **set to 1** when `0x01BE` is sent |
| `142d3c749` in `FUN_142d3c730` | **set to 1** by the second `0x01BE` builder |
| `142d3c6fd` in `FUN_142d3c670` | **cleared** — the `0x0106` teardown, and nothing else |

and one leaf getter at `0x142cfb500` (`movzx eax, byte [rcx+0x33f4]; ret`) which has
**no `.pdata` entry at all** — the exact class of function `CLAUDE.md` warns a
`.pdata`-attributed scan drops. Its single caller is the one that matters:

```asm
; FUN_142097f80 — the SetField handler, 0x01A0
142097fcb  call 1406e9170(pkt, &buf, 8)   ; the 8-byte FILETIME
142097fdc  mov  r14, [DAT_143aa84a0]
142097fea  je   14209acea                 ; no world object -> return
142097ff3  call 142cfb500                 ; world->[0x33f4]
142097ff8  test al, al
142097ffa  jne  14209acea                 ; LOGOUT PENDING -> return
```

`14209acea` is the epilogue: cookie check, restore, `ret`. **[L]**

So from `20:30:42.904` onward in that session, **every `SetField` the server sent would
have been read for 8 bytes and dropped in silence** — no dialog, no error, no movement.
Any experiment run after a Log Out click in the same session is invalid.

This is the same failure *class* as `research/npc-click.md`'s `player->[0x2330]` latch —
set by the request, cleared only by the answer, failing silently and later — but on a
different field and with a worse blast radius, because `SetField` is the packet this
project depends on. `0x01BE` is **not** one of the 37 setters of `[0x2330]`; it has its
own.

## 2.5 The server can also *ask* the client to log out

Case **`0x0137`** (jump table → `0x142cbc7f0`), zero-body: **[L]**

```asm
142cbc7f0  cmp  dword [rdi+0x33f0], 0
142cbc7f7  je   142cbc898              ; -> (*DAT_143ad5718)(0)
   else:   build an empty string, then
142cbc88e  call 142cfb470              ; -> the client sends 0x01BE
```

Not needed for the owner's button, but worth knowing: `0x0137` is the "please log out now" push,
and it makes the client set its own `0x33f4` latch. **Do not send it without following up
with `0x0106`**, or the session ends up in exactly the stuck state of 2026-08-19.

## 2.6 What else to expect on the wire

During the teardown, `FUN_142d18520(world, 2, NULL)` builds and sends outbound **`0x0422`**:
`u32 reason` (= 2) then a `str` from `FUN_142d43c80`. **[L]** It is gated —
`FUN_141892840()` must be non-null and `world->[0x332d]` must be 0 — so it may or may not
appear. **[D]** It is a report, not a request; nothing suggests it needs an answer, but
per the project's standing rule, watch for a freeze the first time it shows up unanswered.

## 2.7 What I could not settle

* **Whether the `0x0106` string is ever displayed.** It is stored at `world->[0x31f0]`.
  The only other code touching that field is the world constructor, the world destructor,
  `FUN_142cf4350` (which frees it and zeroes it — called from the `0x0082` "leave world"
  builder and from `FUN_1415d35f0`, a socket-teardown path), and two leaf getters at
  `142cf42c1` and `142cf42e0` that have **no `call rel32` callers at all**. So statically
  it is written, freed, and read only from code I cannot reach. If it turns out to raise a
  dialog on the login screen, send something presentable rather than `"."`.
* **The opcode of `FUN_142cd98a0`**, which does exactly what case `0x0106` does — same
  string read, same non-empty gate, same `FUN_142d18520(…,2)` then `FUN_142d3c670`. It is
  virtualised-dispatched. There are therefore **at least two** opcodes that log the client
  out; `0x0106` is the one that is pinned.
* **Whether the channel socket is closed** (2.3).
* **What `world->[0x33f0]` is** — the gate on case `0x0137`.

---

# 3. Instrument notes

Three of these cost something during this session and are worth carrying forward.

**`tools/xref.py --string` returns 0 for strings it should find.** Run against
`Npc/%07d.img` — documented in `research/npc-spawn.md` as referenced by four functions — it
reports **0 code references**. The reason is the one `npc-spawn.md` already recorded: this
client reaches most literals through a `PTR_` qword slot in `.data`, and `xref.py` matches
only `lea` and `mov imm64`. **A `0 references` result from `xref.py` on a string in this
binary means nothing until the control is run.** The working form is to scan `.data` for
the string's qword (`Npc/%07d.img` → one slot at `0x143a46b38`) and take references to
*that*.

**A displacement scanner must start its resync window at 2 bytes, not 3.** My first pass
over `+0x33f4` returned 12 instructions and **missed the write in `FUN_142cfb470`** — the
very instruction the search existed to find — because `mov byte [rcx+disp32], imm8` is
`C6 81 <disp32> <imm8>`, putting the displacement only 2 bytes into the instruction, and
the window started at 3. It came back clean and confident and wrong. Widening to 2 found
17. This is the fourth entry in the project's list of scans that return a tidy wrong
number by looking for the wrong shape.

**A linear capstone sweep must resync, and several of these functions need it.**
`FUN_142d91560` (handler for `0x008D`) reads `u32, str` and then does
`jmp 0x144f1b71e` — out of `.text` and into the Themida region — at its 6th instruction.
`FUN_142c48ca0` does the same 74 bytes in. Every dump above was produced by a sweep that
emits a `.byte` line and advances one byte on an undecodable opcode rather than stopping,
and every function was bounded by `.pdata` first.

**Method that worked, for reuse:** the inbound equivalent of
`research/msexe-packet-fields.txt` — the read-primitive sequence of all 273 channel
handlers — was cheap to build (inline cases from the decompiled switch, forwarded cases by
disassembling the handler) and is what turned "find the chat packet" from a search into a
filter. It is worth committing as a tool if this kind of question comes up again.
