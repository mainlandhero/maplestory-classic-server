# The revive dialog is opened by a packet — `0x0315` — and its buttons send `0x00D1` and `0x01E7`

2026-08-21. **No client run was spent on this.** Everything below is static, off
`client-patched\MapleStory.exe`, the Ghidra project, and the repo's own tools.

Marks: **[L]** read off a listing, a decompilation, the raw PE bytes or a decrypted string
table. **[D]** derived from something [L] by an argument written down here. **[I]** inferred
or taken from the v214 reference source, which scored 1 of 8 on a held-out control.

Listings and decompilation added beside this file:

| | |
|---|---|
| `research/msexe-revive.c` | 24 functions decompiled in one headless run — the handler, the dialog class, the opener/closer, the helpers |
| `research/msexe-revive-0315.txt` | the `0x0315` handler, **both copies**: the out-of-line `FUN_142903cd0` and index `0x50` of `FUN_14289a3a0`'s jump table |
| `research/msexe-revive-buttons.txt` | `FUN_1411a3c90` (the button handler), `FUN_1411a4240`, `FUN_1411a4160`, the opener, the closer, `FUN_1429376c0` |

---

## 0. The short answer

**The client never opens the revive dialog by itself.** It opens it when it receives
inbound opcode **`0x0315`**, and only then, and only if its own HP is already `<= 0`.

```
server -> 0x007C  StatChange hp = 0        the tombstone; the client's HP field
server -> 0x0315  23-byte body             the dialog
```

The owner sees the tombstone because `hp = 0` reached the client. They see no dialog because
`0x0315` never did — **nothing in `crates/` sends it, and nothing in `crates/` knows the
opcode exists.** [L] (`grep -rn "0x0315" crates/` is empty.)

The buttons, when clicked:

| button | label (decrypted string table) | sends |
|---|---|---|
| `button:town` | **"REVIVE IN TOWN"** (id `0x17DE`) | **`0x00D1`** `UserTransferFieldRequest`, `targetField = 0`, **empty portal name**, no x/y — a 25-byte body where an ordinary portal walk is 34 |
| `anibutton:spot` | **"REVIVE ON THE SPOT"** (id `0x17DF`) | **`0x01E7`**, body `u8 0`, `u8 <dialog+0x240>` — 2 bytes |

`anibutton:spot` is **hidden** unless an obfuscated counter on `world[0x2368]+0x200` is
`> 0`, and the town button is re-centred when it is hidden. Nothing this server sends can
set that counter, so in practice **the only thing a revive click will produce is `0x00D1`**.

The server already parses `0x00D1` and would answer that click today by warping the
character **to map 0** with no HP restored. §10 says what to do instead.

---

## 1. The instruments, and the control each was made to pass first

| instrument | positive control | result |
|---|---|---|
| `tools/reads.py` | `0x140304100` depth 2 must show reads through `FUN_1403035a0`/`FUN_140303b40` **and** direct ones | reproduced |
| `tools/encodes.py` | `0x141cb6880` depth 1 must show the `0x2ff` COutPacket ctor at `141cb7eb1` | reproduced |
| `tools/callers.py` | `0x1402fa9a0` must give 96 sites in 15 functions | reproduced exactly |
| `FUN_14289a3a0`'s jump table at `0x14289d660` | index `0xC` must decode to `0x14289a439`, a stub calling `0x1427863f0` — i.e. opcode `0x02D1`, which `research/level-up.md` §6 established by a different route | reproduced |
| `FUN_1427863f0`'s effect table at `0x142791348` | index `0` must decode to `0x14278bd99` (`level-up.md` §6) | reproduced |
| `tools/dataref.py 0x143acaa78` | must find the constructor's store into the dialog global | found, and it is the **only** one |
| my `lea r64,[r64+0x200]` + getter scan | must contain `0x1411a3d94` and `0x1411a4278`, both read by hand out of `tools/listing.py` first | both found |
| my `[reg+disp32]` store scan | run on `0x2358` it must find a store inside `FUN_142ca5c50`, which `research/transfer-field-request.md` §3.4 predicts | found |
| `tools/dump_stringids.py` | ids `0x17DE`/`0x17DF` must decrypt to readable English | "REVIVE IN TOWN" / "REVIVE ON THE SPOT" |

### The blind spots, named

* **`tools/callers.py` cannot see a call through a register, a vtable slot, or the Themida
  VM.** Every "N callers" below is therefore a floor. §2 closes it a second way, from the
  global rather than the call graph.
* **My `lea`+getter scan only matches the `disp32` encoding** (`mod = 10`). Run on `0x5b`
  it returns **zero**, because `lea rcx,[rax+0x5b]` is a `disp8`. That is a *known* limit,
  not a silent one, and it cannot affect a search for `0x200`, which has no `disp8` form.
* **`tools/fieldrefs.py` was run over two VA windows, not the whole image** —
  `0x142700000..0x142a00000` and `0x141100000..0x141300000`. It matches any `[reg+disp]`
  operand including a `lea`, so the `mob+0x42c` failure mode is covered *inside* those
  windows and not outside them.
* **`FUN_1418283f0` jumps into `.themida` at `0x141828435`.** Any linear walk desyncs
  there — which is exactly why `tools/encodes.py` at depth 6 reported that the town button
  builds no packet. It does; see §6.1. **A clean negative from an encode walk through that
  function is worthless.**

---

## 2. The dialog can be created exactly one way

`UI/Revive.img` is referenced by `FUN_1411a3440`, slot 4 (`+0x20`) of the 66-slot vtable at
`0x14338acf8`. The class's own overrides in that vtable are slots 4, 6, 9, 13, 19, 38 and
65; everything else is folded/shared. [L]

The object is **0x260 bytes**, allocated and constructed at exactly one site:

```asm
142cb5d47  cmp   qword ptr [rip+0xe14d29], 0     ; g_pReviveDlg = 0x143ACAA78
142cb5d4f  jne   142cb5d9e                       ; already open -> return 0
142cb5d51  mov   edx, 0x260
142cb5d56  lea   rcx, [rip+0xe20b43]              ; the pool
142cb5d5d  call  0x14019b780                      ; operator new
142cb5d6c  cmp   dword ptr [rsp+0x70], 0          ; arg7
142cb5d71  setne dl
142cb5d74  mov   rcx, rax
142cb5d77  call  0x1411a2ea0                      ; CUIRevive::CUIRevive(this, bool)
```
[L]

The constructor sets the global itself: `DAT_143acaa78 = param_1`, then installs
`&PTR_FUN_14338acf8`. [L]

**Two independent enumerations agree that there is no other way in:**

* `tools/callers.py 0x1411a2ea0` — **1 call site** (`0x142cb5d77`), 0 tail jmps, **0 qword
  pointers in the image**. So it is not in any vtable and no table entry names it. [L]
* `tools/dataref.py 0x143acaa78` — **13 RIP-relative references, one single write**
  (`0x1411a2f0c`, inside the constructor). The three `mov-imm32` sites are the destructors
  (`FUN_1411a32f0`, `FUN_1411a43c0`, and `0x1411a4320`) storing zero. [L]

These have different blind spots: the first is a call-graph scan, the second a data scan
over the global the object must be published into. A Themida-virtualised caller would still
have to reach `FUN_1411a2ea0`, and `FUN_1411a2ea0` is the only thing that ever stores a
non-zero value into `DAT_143acaa78`.

`FUN_142cb5cb0` — the opener — has **3 call sites**, all enumerated: [L]

| site | what it is |
|---|---|
| `0x14289b77c` in `FUN_14289a3a0` | **the `0x0315` handler** — index `0x50` of the local-user jump table |
| `0x142903e29` in `FUN_142903cd0` | a byte-for-byte **out-of-line duplicate** of that same arm, with **zero callers of its own** |
| `0x142937791` in `FUN_1429376c0` | a re-open path that **requires `DAT_143acaa78 != 0` on entry**, so it cannot be the first creator. Its one caller is `FUN_1428923e0`, a `0x00E5` builder |

---

## 3. `0x0315`, field by field

### 3.1 Why the opcode is `0x0315` [L]

`research/level-up.md` §6 established the routing, and I re-read the two hops that matter:

```
FUN_141820080  CField::OnPacket          0x224..0x39F -> FUN_1429b9300
FUN_1429b9300  the user pool
    1429b9372  lea eax,[rdx-0x2c5]
    1429b9378  cmp eax, 0xd9
    1429b937d  ja  <default>
    1429b93e4  mov rcx,[rdi+0x10]        ; the LOCAL user; skipped if null
    1429b93f2  call 0x14289a3a0
FUN_14289a3a0  index = opcode - 0x2C5, RVA table at 0x14289d660, 0xDA entries
```
[L] — I disassembled `FUN_1429b9300` myself; `tools/callers.py 0x1429b9300` gives its one
caller as `0x141820080`.

Decoding the whole table (not just the entry I wanted — **enumerate before you filter**):

```
idx 0x0C -> 0x14289a439    opcode 0x02D1     <- the control, level-up.md's USER_EFFECT_LOCAL
idx 0x50 -> 0x14289b656    opcode 0x0315     <- the arm containing the call to the opener
idx 0x51 -> 0x14289b7c7    opcode 0x0316     <- the next arm, which bounds it
```

`0x2C5 + 0x50 = 0x0315`. The call to the opener is at `0x14289b77c`, inside
`[0x14289b656, 0x14289b7c7)`. [L]

### 3.2 The reads, cross-checked between two copies of the same code

The handler exists **twice** in the image: inside the switch at `0x14289b656`, and
out-of-line as `FUN_142903cd0`. They are the same code with different register allocation.
Both were read, and both give the same eight reads in the same order — that is the
cross-check `docs/ghidra.md` asks for, and here it is two independent copies rather than
two instruments: [L]

```
FUN_142903cd0                         FUN_14289a3a0 @ 0x14289b656
  142903d0a  READ u32                   14289b669  READ u32
  142903d16  READ u32                   14289b674  READ u32
  142903d20  READ u32                   14289b67e  READ u32
  142903d2b  READ u8                    14289b689  READ u8
  142903d39  READ u32                   14289b697  READ u32
  142903d44  READ u32                   14289b6a2  READ u32
  142903d4f  READ u8                    14289b6ad  READ u8
  142903d61  READ u8                    14289b6bb  READ u8
```

`python tools/reads.py 0x142903cd0 3` reports the same eight and nothing else — no helper
reads, no tail jumps. [L] The Ghidra decompilation of `FUN_142903cd0`
(`research/msexe-revive.c`, first function) reproduces them in the same order.

**Every read is unconditional and above every branch**, so the body is a fixed 23 bytes:

| off | width | name here | what the client does with it |
|---:|---|---|---|
| **0** | `u32` | **`a`** | `and r12d,1 / je exit` at `0x14289b6f9` (`and dword ptr [rsp+0x50],1` in the out-of-line copy) — **if bit 0 is clear the handler returns silently.** Nothing happens, and no telemetry is sent. Set it to `1`. [L] |
| **4** | `u32` | **`b`** | `cmp esi,9 / je exit` — **`9` aborts the handler silently.** Otherwise stored at `localUser+0x4e10` and echoed back in the `0x02C6` telemetry. `FUN_1429376c0` additionally requires `<= 8`. [L] |
| **8** | `u32` | **`c`** | overwritten with `-1` by the client itself when `FUN_142122480(world[0x2380], 1)` is false; then passed to the opener as arg3, **which drops it**. Inert. [L] |
| **12** | `u8` | **`d`** | `!= 0` -> stored at `localUser+0x4e14`; opener arg4, dropped. [L] |
| **13** | `u32` | **`e`** | stored at `localUser+0x4e18`; passed to the opener as `e * 1000`. **Seconds.** [L] |
| **17** | `u32` | **`f`** | stored at `localUser+0x4e1c`; passed as `f * 1000`. **Seconds.** [L] |
| **21** | `u8` | **`g`** | `!= 0` -> opener arg8, dropped. [L] |
| **22** | `u8` | **`h`** | `!= 0` -> opener arg9, dropped. [L] |

`4+4+4+1+4+4+1+1 = 23`.

### 3.3 The unit, and the fields that go nowhere

Two things here are exactly the shape `CLAUDE.md` warns about, so they are stated plainly:

* **`e` and `f` are seconds, not milliseconds.** `imul eax, r12d, 0x3e8` /
  `imul edx, edi, 0x3e8` at `0x14289b742`/`0x14289b749`. [L] Send `5`, not `5000`.
* **The opener consumes only two of its nine arguments.** `FUN_142cb5cb0` reads `param_1`
  (the world singleton) and `param_7`; nothing else. Ghidra renders the constructor call as
  `FUN_1411a2ea0(lVar4, param_7 != 0, param_3, param_4, lVar4)`, but `param_3`/`param_4`
  are in `r8`/`r9`, which four intervening calls have clobbered — that is a decompiler
  artefact, and the listing shows no reload. The **constructor takes two arguments**, and
  the second lands at `dialog+0x240`. [L]

  So `b`, `c`, `d`, `e`, `f`, `g`, `h` **never reach the dialog.** `tools/fieldrefs.py`
  over the CUser code window confirms it from the other side: `localUser+0x4e10` has six
  uses (constructor `= -1`, the two handler copies, the `0x02C6` builder, and two in
  `FUN_1429376c0`), `+0x4e18` has four, and there is **not one use of either in
  `0x141100000..0x141300000`, where the whole dialog class lives.** [L]

  **Consequence: for the purpose of making the dialog appear, only `a` and `b` matter.**
  Everything else can be zero. That is a claim that can come back false, and §11 says how
  it would.

### 3.4 What `dialog+0x240` is, and why it is `0`

The constructor stores its bool argument there (`*(undefined1 *)(param_1 + 0x90) = param_2`,
and `param_1` is `int*`, so `0x90 * 4 = 0x240`). [L] That byte is the **second byte of the
`0x01E7` request**. Its value comes from the opener's `param_7`:

| caller | `param_7` | so `dialog+0x240` |
|---|---|---|
| the `0x0315` handler | literal `0` | **`0`** |
| `FUN_1429376c0` (re-open) | literal `1` | `1` |

So a `0x01E7` produced by a dialog our server opened will carry `00 00`. [D]

---

## 4. The gate is the client's own HP — and it is the HP the server set

Immediately before the opener call: [L]

```asm
14289b6e7  call 0x142cbe730                ; = world[0x2358]
14289b732  lea  rcx,[rax+0x5b]
14289b736  mov  edx,[rax+0x63]
14289b739  call 0x1401ba9d0                ; the obfuscated getter
14289b73e  test eax, eax
14289b740  jg   14289b7aa                  ; HP > 0  -> no dialog, send 0x02C6 mode 0
           ...open the dialog...
```

`FUN_142cbe730` is eight bytes: `mov rax,[rcx+0x2358]; ret`. [L]

That is the same field the `0x007C` handler writes. Joining two files rather than
re-deriving it: `research/user-hit.md` §4 pins HP at `charstat+0x5b` with guard words at
`+0x5f`/`+0x63`, read back through `FUN_1401ba9d0`, and `research/transfer-field-request.md`
§3.2 shows the `SetField` short branch re-seeding `user+0x5b/+0x5f/+0x63` on the object
taken from `world[0x2358]`. **[D]** — the join is mine; each half is [L] in its own file.

**So the ordering is forced: `0x007C` with `hp = 0` must arrive before `0x0315`.** If
`0x0315` arrives while the client still thinks it has HP, the handler takes the `jg` and
reports back with a `0x02C6` whose first field is `0` — see §7, which turns that into a
free measurement rather than a mystery.

---

## 5. The hypothesis, tested

The brief's hypothesis, quoted so the retraction is legible:

> the dialog is opened from the client's own hit path, and the client's own damage value is
> a constant `1` … so by its own arithmetic the player never dies, and the dialog never
> opens. The tombstone the owner sees comes from the server's `hp = 0` in `0x007C`.

**The first clause is false.** The dialog is opened from a packet handler. Nothing on the
hit path can open it: `FUN_1428aa0a0` and `FUN_14288ac30` (the two mob builders,
`user-hit.md` §3.1) do not call `FUN_142cb5cb0`, and `FUN_142cb5cb0`'s complete caller list
is the three sites in §2, none of which is a hit builder. The one hit-adjacent function that
*does* appear anywhere near this machinery is `FUN_1428923e0` — and it calls
`FUN_1429376c0`, which **refuses to run unless the dialog already exists**. [L]

**The second clause is true and is half the answer.** The client's damage arithmetic is
irrelevant because the client does not subtract its own HP at all (`user-hit.md` §4, three
enumerations with different blind spots). The tombstone does come from the server's
`hp = 0`. What was missing was not a bigger damage number; it was a packet.

**The part of the hypothesis worth keeping:** a client-side HP test *is* on the path — it is
just inside the `0x0315` handler rather than inside the hit path, and it reads the
server-authored value. So `hp = 0` is necessary and not sufficient.

---

## 6. What the buttons send

`FUN_1411a3c90` is vtable slot 9 — the button handler, `(this, buttonId)`. It tests the id
against two names, using **two different helpers**, which is itself the tell that the two
controls are of different kinds: [L]

```asm
1411a3cbc  lea rdx,[rip+0x210e645]     ; L"town"   @ 0x1432B2308
1411a3cca  call 0x142aa1a20            ; the plain-button lookup
1411a3d63  lea rdx,[rip+0x210ee1e]     ; L"spot"   @ 0x1432B2B88
1411a3d71  call 0x142aa1b80            ; the ani-button lookup
```

matching `UI/Revive.img`'s `button:town` and `anibutton:spot`. Anything else falls through
to the base class at `0x142bf5d70`. [L]

Both arms end the same way: `FUN_142d0b290(world, 1, 0x7d0)` — which is literally
`world+0x37ac = 1; world+0x37b0 = now + 2000` [L] — and then `FUN_142cb5dc0()`, the closer.
**So every revive click blocks the revive UI for 2000 ms and destroys the dialog.** A second
`0x0315` inside that window is refused with code `4`.

### 6.1 `button:town` -> `0x00D1 UserTransferFieldRequest`

The town arm calls `FUN_1418283f0`, which `research/transfer-field-request.md` §0/§1
identifies as **the `0x00D1` builder** — the one missing from
`research/msexe-send-opcodes.txt`. I re-read the two bytes that matter rather than trusting
the file:

```
1418284de   ba d1 00 00 00      mov edx, 0xd1
1418284e3   48 8d 4c 24 30      lea rcx,[rsp+0x30]
1418284e8   e8 33 50 ec fe      call 0x1406ed520      ; COutPacket(pkt, 0xD1)
```
[L]

The call site, read off the listing at `0x1411a3d0f..0x1411a3d32`: [L]

| arg | value on the revive path | value on the ordinary portal walk (`transfer-field-request.md` §1) |
|---|---|---|
| 1 `rcx` | the field stage | the field stage |
| 2 `edx` | **`0`** — the `targetField` | **`-1`** |
| 3 `r8` | **`0`** — the portal-name pointer | a real portal name |
| 4 `r9b` | `0` | `0` |
| 5 | `0` | (sets `user+0x49`) |
| 6 | `FUN_142cc4350(world, 500) != 0` | |
| 7 | `1` | |
| 8 | **`1`** — wire offset 14 | `0` |
| 9 | `0` | `0` |

And I re-derived the two consequences of `arg3 = 0` directly, because they change the body
length: [L]

```asm
141828583  lea    rsi,[rip+0x1c8a567]   ; -> 0x1434B2AF1, and the byte there is 00
14182858a  test   r14, r14              ; r14 = arg3
14182858d  cmovne rsi, r14              ; ... so with arg3 = 0 the name is the EMPTY string
141828685  call   0x1406edc80           ; EncodeStr
14182869f  test   r14, r14
1418286a2  je     0x1418286eb           ; ... and BOTH position Encode2s are skipped
```

So the revive-in-town `0x00D1` is, using `transfer-field-request.md` §2.1's offsets for the
14-byte integrity preamble (**which I did not re-derive — [D] from that file**):

```
off  0  u32  100            literal
off  4  u16  <counter>      client integrity
off  6  u64  0
off 14  u8   1              <- 0 on an ordinary portal walk
off 15  u8   <stage byte>
off 16  u32  0              <- targetField; 0xFFFFFFFF on an ordinary portal walk
off 20  u16  0              <- empty portal name
off 22  u8   0              hard-coded
off 23  u8   0
off 24  u8   0
                            25 bytes, where a portal walk is 34
```

**Three independent discriminators on the wire**, any one of which separates a revive click
from a portal walk: the body length, `targetField == 0` vs `0xFFFFFFFF`, and the empty
portal name. §10 recommends not relying on any of them.

### 6.2 `anibutton:spot` -> `0x01E7`, and why you will probably never see it

```asm
1411a3d7e  mov  rcx,[rip+...]           ; the world singleton
1411a3d8e  mov  rax,[rcx]
1411a3d91  call qword ptr [rax+0x30]    ; world vtable slot 6 = `mov rax,[rcx+0x2368]; ret`
1411a3d94  lea  rcx,[rax+0x200]
1411a3d9b  mov  edx,[rax+0x208]
1411a3da1  call 0x1401ba9d0             ; obfuscated getter
1411a3da8  jle  <do nothing>            ; count <= 0 -> the click is swallowed
1411a3db6  mov  edx, 0x1e7
1411a3dbb  lea  rcx,[rsp+0x50]
1411a3dc0  call 0x1406ed520             ; COutPacket(pkt, 0x01E7)
1411a3dc6  xor  edx, edx
1411a3dcd  call 0x1406ed840             ; u8 0
1411a3dd2  movzx edx, byte ptr [rdi+0x240]
1411a3dde  call 0x1406ed840             ; u8 dialog+0x240
1411a3de8  call 0x1415d01c0             ; SendPacket
```
[L]

**Body: `u8 0`, `u8 <dialog+0x240>` — two bytes, and both are `0` for a dialog our server
opened** (§3.4).

`FUN_1411a4240` builds the identical `0x01E7` behind the identical gate, without the
button-name test — the second of the two `0x01E7` builders
`research/msexe-send-opcodes.txt` lists. [L] `FUN_1411a4160` is the gate on its own:
`bool { return decode(world[0x2368]+0x200) > 0; }`. [L]

**The same counter decides whether the button is drawn at all.** In `FUN_1411a3440`
(vtable slot 4, the create): [L]

* the `town` button gets its caption from string id `0x17DE`; if the counter is `< 1` the
  code jumps to `LAB_1411a3608`, which reads the `town_center` vector out of `UI/Revive.img`
  and **moves the town button there**;
* the `spot` button gets caption `0x17DF`; if the counter is `< 1`, the code calls
  `FUN_142aa1c30(ui, L"spot", 0)`. That function looks the ani-button up by name with the
  same `FUN_141ad6240` the create used, then calls `FUN_141684970(widget, false)`, which
  hands `false` to **three** of the widget's virtuals — `[widget+8]->vt[0x70]`,
  `widget->vt[0xb0]`, and a tail call to `widget->vt[0x98]`. [L] Calling that trio
  "hide/disable" is [D]; that the button is turned off, and that the town button is
  simultaneously re-centred on `town_center`, is [L].

That counter is one of three obfuscated fields on `world[0x2368]` — `+0x200` with its key at
`+0x208`, `+0x20c`/`+0x214`, `+0x218`/`+0x220`. `world[0x2368]` is allocated and constructed
by the world constructor at `0x142ca649b`, so it is **not null** and the gate cannot fault.
[L]

Naming the counter, **[D]**: the only other function that reads `+0x200` through the getter
is `FUN_1408674e0`, which reads `+0x20c` in the same breath and feeds it to `FUN_1407eca00`
/ `FUN_1407ecbb0` — item-id predicates, the same pair `FUN_142122480` uses on the buff list
— and then **re-keys all three fields**. So the triple is `{count, itemId, ?}`. And the
string table has, at id `0x0857`, *"You have used 1 Respawn Token in order to revive at the
current map. (%d left)"* [L]. **So `+0x200` is the Respawn Token count and `0x01E7` is the
respawn-token revive.** [D]

**Nothing MapleCW sends can set it.** So: the dialog our server opens will show **one
centred button, "REVIVE IN TOWN"**, and the only packet a click can produce is `0x00D1`.
That is a claim that can come back false — if the owner sees two buttons, the counter is being
set by something I have not found, and §11 says what that would mean.

---

## 7. `0x02C6` — the client tells you why it refused, for free

When the handler does **not** end up with a dialog, it calls `FUN_142903e90`, which builds
an outbound packet: [L]

```asm
142903f0e  mov edx, 0x2c6
142903f18  call 0x1406ed520
           w_raw(4 bytes = 01 00 00 00)
           w_u32 x 10
           SendPacket
```

44-byte body. The ten `u32`s, from the decompilation: [L]

| # | value |
|---|---|
| 1 | **mode** — `0` when the client's HP was `> 0`; `2` when the dialog was refused |
| 2, 3 | `0` on this path |
| 4 | **the opener's return code** |
| 5, 6 | two numbers from `localUser+0x100` |
| 7 | **the decoded HP** |
| 8, 9 | two numbers from a field/mob lookup |
| 10 | **field `b` echoed back** (`localUser+0x4e10`) |

The opener's return codes, read off `FUN_142cb5cb0`: [L]

| code | meaning |
|---|---|
| `1` | the current stage is not a field (`FUN_141892840()` returned null) |
| `2` | `FUN_14182ffb0(stage)` — the stage is busy |
| `4` | `world+0x37ac` is set and `world+0x37b0` has not expired — **the 2000 ms UI block from a previous revive click** |
| `5` | created |
| `0` | it was already open |

Note the handler checks the **global**, not the return code: if the dialog exists afterwards
it sends nothing at all. So:

| what the owner sees | what `world.log` shows |
|---|---|
| the dialog | **no `0x02C6`** |
| nothing | `0x02C6` mode `0` -> the client's HP was still positive when `0x0315` arrived |
| nothing | `0x02C6` mode `2`, code `1`/`2`/`4` -> the opener refused, and field 4 says which |
| nothing | **no `0x02C6` either** -> `a & 1` was clear, or `b == 9`; the handler returned before doing anything |

Four outcomes, three of them distinguishable from the log alone, and the fourth
distinguishable because it is the only silent failure. That is the whole test in §11.

---

## 8. The death sequence itself

* **The tombstone is client-side and already works.** `research/user-hit.md` §6.2 enumerated
  ~65 sites that read `charstat+0x5b` and branch on sign — action gates. §6.1 showed the
  `0x007C` HP arm takes the same four instructions for `hp = 0` as for `hp > 0`. The owner seeing
  a tombstone confirms the client draws it off `hp <= 0` with no further packet. Nothing
  here contradicts that, and **the screen wins** anyway.
* **Nothing un-gates those ~65 sites except a positive HP.** So a revive that does not send
  the new HP leaves a character who is standing up and cannot do anything.
* **The dialog is destroyed by the click, not by the reply.** Both button arms call
  `FUN_142cb5dc0()` themselves. The server does not need to close anything.
* **A revive click blocks a re-open for 2000 ms** (`FUN_142d0b290(world, 1, 0x7d0)`), so a
  second `0x0315` inside that window returns code `4`.
* **Two `0x02D1` effect ids are revive messages** — from the effect table at `0x142791348`,
  whose index-0 control reproduces `level-up.md`'s answer: [L]

  | effect | arm | string |
  |---|---|---|
  | **26** (`0x1A`) | `0x14278f62c` | *"You have used 1 Respawn Token in order to revive at the current map. (%d left)"* (id `0x0857`) |
  | **34** (`0x22`) | `0x14278f600` | *"You have revived on the current map through the effect of the Spirit Stone."* (id `0x0858`) |

  Effect 26 formats a `%d`, so its arm reads at least one more field. I did **not** count
  the reads in that arm; treat it as a lead, not a layout. [D]

---

## 9. Notes on files I do not own

Nothing here contradicts another file. Three things are worth carrying forward:

1. **`research/user-hit.md` §6.3** — *"I did **not** find what plays the death/tomb sequence,
   and I did not find a revive request or a revive reply."* That negative was correct and
   correctly hedged; the thread it pointed at (the `"dead"` stance strings) is the tombstone,
   which is a different question from the dialog. The dialog is `0x0315`, and it was
   unreachable from where that pass was standing: `FUN_142cb5cb0` has no string reference,
   no RTTI, and its handler is one arm of a 13 864-byte switch.

2. **`research/user-hit.md` §7** named `FUN_14289a3a0` as the one `0x00E5`-family function
   that touches the request latch. It is also **the local user's entire inbound packet
   switch** for `0x2C5..0x39E` — 218 opcodes in one function. Anything that reads it
   linearly (`research/msexe-packet-fields.txt` does) will attribute one case's calls to the
   previous case's send. The `0x00E5` row in that file lists
   `FUN_142cb5cb0` and `FUN_142903e90` among its callees; **those belong to the `0x0315`
   arm, not to `0x00E5`.** That is how I found the handler, so the file earned its keep —
   but its rows are not per-opcode past the first `SendPacket`.

3. **`research/transfer-field-request.md`** — its §2.1 table is what makes §6.1 above a
   layout rather than a guess, and its "*offsets 27-30 are written only when the portal name
   pointer is non-null*" is the sentence that predicts a 25-byte revive request. I confirmed
   the gate (`test r14,r14 / je` at `0x14182869f`) and the empty default string myself; I did
   **not** re-derive the 14-byte preamble or the arg->offset mapping.

---

## 10. WIRE IT LIKE THIS

Nothing below is wired. `crates/` contains no reference to `0x0315`, `0x02C6` or `0x01E7`.
[L]

### 10.1 Send the dialog

Two packets, **in this order**, when the server decides the character has died:

```
0x007C   net::stats::StatChange::hp_only(0)      13 bytes  (user-hit.md 5.1 - already exists)
0x0315   23 bytes:
         01 00 00 00      a = 1     MUST have bit 0 set, or the client silently ignores it
         00 00 00 00      b = 0     MUST NOT be 9; keep it <= 8
         00 00 00 00      c = 0     inert
         00               d = 0
         00 00 00 00      e = 0     SECONDS, not ms
         00 00 00 00      f = 0     SECONDS
         00               g = 0
         00               h = 0
```

Do not send `0x0315` in the same breath as a `SetField` — the opener returns `2` while the
stage is busy, and `4` for 2000 ms after any previous revive click.

**The hook point already exists.** `crates/world/src/session/combat.rs`, in the `0x00E5`
handler, is where `chr.hp = chr.hp.saturating_sub(applied)` runs and the
`StatChange::hp_only(chr.hp)` reply is pushed. [L] The `0x0315` goes in right after it, and
**it hangs off the transition, not off the request**:

```
let died = before > 0 && chr.hp == 0;      // NOT `chr.hp == 0`
...
out.push(hp_only reply);
if died { out.push(Reply { opcode: 0x0315, body: revive_prompt(), ... }); }
```

`before > 0` matters: without it every subsequent hit on an already-dead character re-sends
`0x0315`, the opener returns `4` for 2000 ms after each one, and `world.log` fills with
`0x02C6`.

### 10.2 Answer the click

The button will be **"REVIVE IN TOWN"**, and it arrives as `0x00D1`, which
`Session::on_transfer_field` already handles. **Today that click warps the character to map
`0` with no HP restored**, because `parse_transfer_field` returns
`target_field: Some(0), portal_name: "", position: None` and the handler passes `0`
straight to `go_to_map`. [L] — read out of `crates/world/src/session/mod.rs:115` and
`crates/world/src/session/field.rs:426`.

**Gate the revive branch on the server's own HP, not on the packet's shape.** The wire has
three discriminators (§6.1) and all three are properties of one client build; the server's
`hp == 0` is a fact the server itself owns, and `CLAUDE.md`'s standing rule is that every
effect hangs off the transition. So, in `on_transfer_field`, before the portal lookup:

```
if character.hp == 0 {
    // this 0x00D1 is a revive-in-town click
    let town = return_map_of(character.map_id);   // gm-handbook/returnmaps.txt
    character.hp = 50;
    if character.level > 10 {
        character.exp -= exp_for_level(character.level) / 10;   // clamp at 0
    }
    persist, then go_to_map(character, town, 0, "revive in town");
    also send 0x007C StatChange with the new HP (and the new EXP if it changed);
    return;
}
```

Three details that will otherwise cost a launch each:

* **The new HP must go out.** `go_to_map` builds a `SetField` carrying the stat block, so if
  `character.hp` is 50 before the record is built the bar will be right — but send the
  `0x007C` as well. It is 13 bytes, it is idempotent, and without a positive HP the ~65
  action gates in `user-hit.md` §6.2 stay shut and the character stands in town unable to
  move.
* **The destination is `info/returnMap`.** `research/return-maps.md` (written today, same
  data) establishes it is present on 426 of 426 fields, never the sentinel, and points at a
  `town == 1` field on 388 of them. `gm-handbook/returnmaps.txt` already has the column.
  **Nothing in `crates/` loads that file yet** — `grep -rn "returnmaps\|return_map" crates/`
  is empty. [L]
* **Return early on the refusal.** If the character is not actually dead server-side, fall
  through to the ordinary portal path unchanged. Do not gate the warp and the HP and the EXP
  separately — that is the `complete_quest` mistake in `CLAUDE.md` wearing a new hat.

### 10.3 Answer `0x01E7` anyway

It should never arrive (§6.2), but **always answer**. Two bytes, both `0`. The right reply
is the same revive without the warp: `hp = 50`, same map, `0x007C` with the new HP, and
optionally `0x02D1 { u8 26 }` for the "Respawn Token" message. If one ever *does* arrive, it
means the Respawn Token counter is being set by something this document did not find — say
so loudly rather than quietly handling it.

### 10.4 Log `0x02C6` verbatim

44 bytes. Do not reply — it is a one-way client report. **Log all ten `u32`s**, because
fields 1, 4 and 7 are the entire diagnosis when the dialog does not appear (§7).

---

## 11. If it does not appear: the single-variant test

This does **not** need a sweep. `!migsweep` existed because ten candidate opcodes had to be
tried in one launch; here the opcode is read off a jump table whose index-`0xC` control
reproduces an independently-established answer, and **the client reports its own refusal
code**. One launch, one variant.

**Send `0x007C hp=0` then `0x0315` with the 23 bytes in §10.1, and read `world.log`.**

| what the owner sees | what the log shows | what it means |
|---|---|---|
| the dialog, one centred button "REVIVE IN TOWN" | **no `0x02C6`** | everything in this document is right |
| the dialog, **two** buttons | no `0x02C6` | right, and the Respawn Token counter at `world[0x2368]+0x200` is non-zero for a reason §6.2 did not find. Expect `0x01E7` |
| nothing | `0x02C6`, field 1 = `0` | the client's HP was still `> 0`. The `0x007C` did not land, or `0x0315` overtook it. Field 7 carries the HP the client thinks it has — read it |
| nothing | `0x02C6`, field 1 = `2`, field 4 = `1` | not in a field stage when it arrived. Send it later |
| nothing | `0x02C6`, field 1 = `2`, field 4 = `2` | the stage was busy. Do not send it next to a `SetField` |
| nothing | `0x02C6`, field 1 = `2`, field 4 = `4` | the 2000 ms UI block was up. Something called `FUN_142d0b290` recently |
| nothing | **no `0x02C6` at all** | the handler returned before doing anything: `a & 1` was clear or `b` was `9`. Check the 23 bytes on the wire — or the opcode is not `0x0315`, and §3.1 is wrong |

Only the last row would mean the opcode is wrong, and it is the only row this document
cannot already explain. Every other outcome is a measurement.

**The follow-on, same launch, no extra variant:** the owner clicks "REVIVE IN TOWN". `world.log`
should show an inbound **`0x00D1` with a 25-byte body** (not 34), `targetField = 0`, empty
portal name. If it is 34 bytes with a name, §6.1 is wrong about `arg3`.
