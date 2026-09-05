# The channel's own migrate reply — `FUN_1415d8c00`

> **ANSWERED 2026-08-21: the opcode is `0x001A`, not `0x001B`.** This file predicts `0x001B`
> in at least three places (§0, §4, §7's code block) and every one of them is tagged **[I]**
> honestly. A ten-candidate sweep settled it: the dispatch line for `0x001A` took **354 ms**
> where its neighbour took 64 us, and the long one is a socket teardown.
>
> **The fact's owner is `crates/world/src/session/field.rs::MIGRATE_COMMAND_CHANNEL`.** The
> predictions below are kept as the working - the alignment reasoning is sound and produced
> a candidate one off - but nothing here should be read as the answer.

**Written 2026-08-21 while holding the Ghidra lock, and it did not need Ghidra.** Everything
below came from `tools/listing.py`, `tools/reads.py`, `tools/callers.py`,
`tools/pdata_lookup.py`, `tools/dump_va.py`, a 4-byte-RVA scanner written for this task, the
`0x00D2` capture in `previous-runs/world-20260821-001633.log`, and the two reference enums.
New listings are saved beside this file as `research/msexe-migrate-command.txt`.

Labels: **[L]** read from the listing or a capture. **[D]** derived from two or more [L].
**[I]** inferred, including anything from `ModernMapleSource`.

---

## 0. The one-line answer

**The reply is not a case of `FUN_142cbaa80` at all.** The channel stage's migrate handler is
**`FUN_1415d8c00`**, a *socket-level* handler in the `CClientSocket` subsystem, dispatched
from the Themida-virtualised packet loop rather than from any stage's `OnPacket`. Its body is
**three fields, seven bytes**:

```text
u8   ok        0 -> log an error and do nothing; non-zero -> migrate
u32  ip        raw network order, exactly the four bytes 0x0011 already carries
u16  port      little-endian, the client htons() it itself
```

**Its opcode number is the only thing still missing, and it cannot be read statically** — see
§4. The strongest candidate is **`0x001B`** and §7 is a one-run test that settles it.

---

## 1. The brief's premise is wrong, and here is the proof rather than a search that found nothing

The task said the reply "is somewhere in `0x70..0x19f`". It is not, and this is a positive
argument, not a failed search.

A migrate has to make the client connect somewhere. The connect funnel is fully enumerated:

```text
FUN_1415d0b40   the connect itself (Themida-virtualised at +0x13: jmp 0x144a8e012)
   ^-- exactly ONE caller image-wide:  FUN_142caa360        [L]
FUN_142caa360   builds the sockaddr, tears the current connection down, calls the above
   ^-- exactly TWO callers image-wide:                      [L]
        FUN_141b36f60   the LOGIN stage's case 0x0011  (the migrate we already send)
        FUN_1415d8c00   <-- everything else
```

Measured with `tools/callers.py`, run **after** its documented positive control
(`python tools/callers.py 0x1402fa9a0` -> 96 call sites in 15 functions, as its docstring
promises). `callers.py` reports three kinds — `call`, tail `jmp`, and qword pointers in data
— and all three were zero for the ones that matter.

Corroborated by a **different** question, per `CLAUDE.md`'s rule that re-running a tool is not
a second opinion: `FUN_1415e2450`, the `sockaddr_in` builder, has exactly two callers —
`FUN_1415d0280` (the client's initial connect) and `FUN_1415d8c00`. **[L]**

`research/talking-back.md` §2.3 reached the same place from a third direction — a `dataref`
scan of the `htons` IAT slot `0x143262e48` finds 9 sites image-wide and **none of them is in
any of the 273 channel handlers**, with `141b37ea5` inside `FUN_141b36f60` as its positive
control.

> **Named blind spot, per `CLAUDE.md`.** `callers.py` cannot see a call made through a
> register, a vtable slot, or the Themida VM, and `FUN_1415d0b40` is itself virtualised. So
> the claim is: *every statically readable path to a socket connect funnels through
> `FUN_142caa360`.* A handler that reached the winsock imports directly, without going
> through this funnel, would be invisible to all three scans — but it would also have to
> re-implement the address plumbing that already exists here.

**So no case in `FUN_142cbaa80` can migrate anything.** That also disposes of the idea that
the answer is one of the "cases that send something": `0x162`->`0x175` and `0x275`->`0x17e`
are the only two request/response pairs in that switch, and neither is this.

---

## 2. `FUN_1415d8c00`, field by field, from the listing

`0x1415d8c00 .. 0x1415d8e6c`, 620 bytes, bounded by `tools/pdata_lookup.py`. Full listing in
`research/msexe-migrate-command.txt`. Signature `f(rcx = connection, rdx = CInPacket)` —
`rcx` is saved to `[rsp+8]` and `rdx` to `[rsp+0x10]` before `sub rsp,0xc8`, so they appear
later as `[rsp+0xd0]` and `[rsp+0xd8]`, and `[rsp+0xd8]` is what every read primitive is
handed. **[L]**

### 2.1 The read count, cross-checked before the field list is believed

`CLAUDE.md`: *field order from the listing, field meaning from the decompiler, and cross-check
the packet-read count first.*

| instrument | result |
|---|---|
| `python tools/listing.py 0x1415d8c00` | `1415d8c2b` u8, `1415d8d2c` u32, `1415d8d3d` u16 |
| `python tools/reads.py 0x1415d8c00 2` | the same three, same addresses, **depth 2** so helpers count |

Both instruments were run against their documented controls first (`0x140304100`: raw, u8,
u8, then a run of u16). **Three reads. There is no fourth field.** **[L]**

### 2.2 The body

| body offset | size | field | what the client does with it |
|---:|---:|---|---|
| 0 | `u8` | **ok** | `1415d8c33 test eax,eax / je 1415d8e3d` — zero takes the error branch |
| 1 | `u32` | **ip** | written **unchanged** into `sockaddr_in.sin_addr` at `1415d8e02`, so it is **network order on the wire**: the bytes are `7f 00 00 01` for 127.0.0.1 |
| 5 | `u16` | **port** | `1415d8dfd call FUN_1415e2450(&sa, port)`, which calls **`htons`** (`[rip+0x1c809d1]` = IAT `0x143262e48`) — so the wire value is **little-endian host order**, `26 21` for 8486 |

`FUN_1415e2450` is 81 bytes and does exactly `sa.sin_family = AF_INET(2); sa.sin_port =
htons(port); sa.sin_addr = <0>; sa+8 = 0`, then `FUN_1415d8c00` overwrites `sin_addr` with the
raw `u32`. **[L]**

**This is the same convention the working login `0x0011` already uses**, which is the cheapest
possible check on the units: the capture's `0x0011` body carries `7f000001` then `2521` for
127.0.0.1:8485, and `2521` little-endian is 8485. **[L]**

Anything after byte 7 is ignored — the cursor simply stops.

### 2.3 The `ok == 0` branch is inert, and that matters for the test in §7

```asm
1415d8e3d  mov r8d, 0x21000002        ; error code
1415d8e43  mov edx, 0x63a             ; line 1594
1415d8e48  lea rcx, [rip+0x1c990b5]
1415d8e4f  call 0x1412825a0           ; -> resolve the code to a string, then log it
1415d8e54  ... stack cookie, ret
```

`FUN_1412825a0` (105 bytes) resolves the code with `FUN_1418039d0`, formats a string, calls
`FUN_1418049f0(ctx, line, code, str)` and frees. **It returns normally — no throw, no
fail-fast, no dialog raiser on this path.** **[L]** So a `<opcode> 00` body is a legal,
side-effect-free "no". That is also the shape a **refusal** must take (§6).

### 2.4 What the handler does on success, in order

```text
1415d8c2b  u8 ok                                          ; 0 -> §2.3 and return
1415d8c3b  FUN_14209ee40()  = lea rax,[0x143AD4840]        ; &g_currentStage  [L]
1415d8c43  FUN_1415e3000(that)                             ; deref, raise if null
1415d8c94  call [stage_vtbl + 0xd0](stage+8, FUN_1415e3c30())   ; IsKindOf(<type at 0x143A88588>)
           if NOT that type:
1415d8cc9    FUN_14019a150(0x68)                           ; allocate a 0x68-byte object
1415d8ce0    FUN_141a3ca80(obj)                            ; ctor: base FUN_142097cf0 + 4 vtables
1415d8d15    FUN_14209ee50(obj, 0)                         ; INSTALL IT AS THE CURRENT STAGE
1415d8d1a  FUN_140caa510() = [0x143AA84A0]                 ; the world object
1415d8d2c  u32 ip
1415d8d3d  u16 port
1415d8d47..1415d8de9   FUN_142c4f8e0(...)                  ; a report: old ip/port from conn+0x50,
                                                           ; new ip/port, plus world+0x2258/0x2260
1415d8dfd  FUN_1415e2450(&sa, port);  sa.sin_addr = ip
1415d8e1a  FUN_142caa360(world, &sa)                       ; <-- THE RECONNECT
1415d8e2c  FUN_142cd3c40(world, 0, 1)                      ; <-- SENDS OUTBOUND 0x0118
1415d8e36  FUN_142c50b90(9)                                ; app/connection state := 9
```

`0x14209ee40` and `0x142c50b90` have **no `.pdata` entry** — they are the `.pdata`-less leaf
getters `research/channel-select.md` §7 warns about — so they were read with
`tools/dump_va.py` instead of a function dump. `0x140caa510` decodes to
`mov rax,[0x143AA84A0]; ret`, the same world singleton `FUN_1415d59b0` gates on, which is an
independent check that the arithmetic is right. **[L]**

**The client sends `0x0118` as part of migrating.** `FUN_142cd3c40` builds
`u32 tick, u8 arg2, u8 arg3, [u32 world+0x31d4 when arg3 != 0]` under opcode `0x118`
(`142cd3c65 mov edx,0x118`), and `research/msexe-send-opcodes.txt:331` already lists it as
the `0x0118` builder. It has two callers: this handler and `FUN_142311bf0`. **[L]** So a
`0x0118` arriving on either channel log is a strong second signature that the migrate ran.

---

## 3. Teardown: **the client does it, and it does it first**

`FUN_142caa360`'s very first act, before it builds any address, is
`142caa387 call FUN_1415d35f0(conn)`. **[L]** That function:

```text
1415d35fe  FUN_1415e4080(conn)   = cmp dword [rcx+0x48],0 / setne al   ; conn+0x48 = the
                                                                      ; login-vs-channel type
1415d3612  if non-zero: FUN_142cf4350(world)                          ; tell the world it is going
1415d361c  FUN_1415d5aa0(conn)                                        ; drain/clear +0x70,+0x88,+0xb0,
                                                                      ; +0x108, then FUN_1415d9cd0
1415d3636  conn+0x0c = 0 ; conn+0x10 = 0
1415d3652  FUN_1415e3b60(conn+0x20)
```

**Measured corroboration, and it is the important half.** In
`previous-runs/login-20260821-001613.log` the login server sends `0x0011` at `04:16:13.769`
and logs `#1 127.0.0.1:58250 closed` at **`04:16:13.777` — eight milliseconds later**. **[L]**
The client closes the socket itself, immediately, on a migrate it understood.

And the negative control is in the failing run: in
`previous-runs/world-20260821-001633.log` the `0x0011` reply goes out at `04:16:27.761` and
ch0 stays open sending mob movement for another **six seconds**. **[L]** That gap — 8 ms
versus 6 s — is on its own a clean, cheap discriminator for §7.

> **So: answer, then leave the socket alone.** Closing it from the server is not merely
> unnecessary, it races the client's own teardown. The failure mode of getting this backwards
> is a hang, and a hang here is indistinguishable on screen from the bug we already have.

---

## 4. The opcode, and why it is not statically readable

`FUN_1415d8c00` has **zero** callers of every kind `tools/callers.py` can see, and — a
question that tool does *not* ask — **zero 4-byte RVA references** anywhere in the image
either. I wrote a scanner for that specifically, because an MSVC switch jump table stores
4-byte RVAs (the login stage's own table at `0x141b265dc` is exactly that shape) and
`callers.py` only looks for 8-byte pointers. The scan's built-in positive control is that it
finds each function's `.pdata` `BeginAddress` entry, so it can speak:

```text
0x1415d8c00  ->  .pdata 0x143b725cc only
0x1415d59b0  ->  .pdata 0x143b72530 only     (the known game-stage dispatch entry)
0x1415d5a00  ->  .pdata 0x143b7253c only
0x1415d5a50  ->  .pdata 0x143b72548 only
```

The three known VM-dispatched entries behave identically, which is what a VM-dispatched
function looks like here. **The opcode is baked into Themida VM bytecode, and `.themida` has
`SizeOfRawData = 0` — there are no file bytes to search.** **[L]**

### 4.1 What the references say, and how far that is worth trusting

mscw's SOCKET block has **eight** opcodes read out of the client (`docs/opcodes.md`), and
against v214's `ins.txt` they line up at a **constant +0x0A**:

| name | v214 | mscw | delta |
|---|---:|---:|---:|
| CheckPasswordResult | 0x00 | 0x00 | 0 |
| WorldInformation | 0x01 | 0x0B | +0x0A |
| SelectWorldResult | 0x06 | 0x10 | +0x0A |
| SelectCharacterResult | 0x07 | 0x11 | +0x0A |
| AccountInfoResult | 0x08 | 0x12 | +0x0A |
| CheckDuplicatedIDResult | 0x0A | 0x14 | +0x0A |
| CreateNewCharacterResult | 0x0B | 0x15 | +0x0A |
| DeleteCharacterResult | 0x0C | 0x16 | +0x0A |

`research/msexe-gamestage-opcodes.md` scored the *blind* form of this method at 1 of 8 and
said why: *"Order-preserving alignment reproduces the interior of a block perfectly once the
block's offset is known, and determines the offset not at all."* Here the offset is not
guessed — it is fixed by seven anchors read out of the client. That is the mode the same file
scores 7/7 in.

Both references then put MigrateCommand at exactly **DeleteCharacterResult + 5**: v214
`0x0C -> 0x11`, and the v265 `OutHeader.java` `26 -> 31`, with the same intervening names
(ReservedDelete, ReservedDeleteCancel, Rename/gap, SetCharacterID). Two independently
numbered references agreeing on a *distance* from a confirmed mscw anchor is worth more than
either agreeing on a value.

> **mscw MigrateCommand = 0x0016 + 5 = `0x001B`. [I], and it is the best-supported [I] in
> this file. It is not [L] and must not be written down as one.**

### 4.2 One mscw-side fact that constrains it, and it is a real constraint

The login stage's own `OnPacket` `FUN_141b25f30` has cases

```text
0x00, 0x0B..0x18, 0x23, 0x25, 0x26, 0x27, 0x29, 0x2B, 0x34..0x39, 0x45..0x48, 0x4A, 0x50, 0x5F
```

(extracted from `research/msexe-loginswitch.c`; case `0x0B` is the world-list decoder
`FUN_141b2fac0` that `research/channel-select.md` §3.1 already named, which is the control
that says the extraction is right). **[L]**

`FUN_1415d8c00` is dispatched by nobody's switch, so the migrate opcode **must** be one the
login stage has no case for. Inside the SOCKET block the caseless runs are:

* `0x01..0x0A` — the ten mscw-only insertions `msexe-gamestage-opcodes.md` identified. A
  v214/v265 name cannot live there.
* **`0x19..0x22` — ten slots, and under the +0x0A map they are exactly v214 `0x0F..0x18`:
  RenameCharacterResult, SetCharacterID, MigrateCommand, AliveReq, PingCheckResult,
  AuthenCodeChanged, AuthenMessage, SecurityPacket, PrivateServerPacket, ChangeSPWResult.**
* `0x24`, `0x28`, `0x2A`, `0x2C..0x33`, `0x3A..0x44`, `0x49`, `0x4B..0x4F` — all *after*
  MigrateCommand's position under the same map.

The gap in the client's own switch is precisely the run of opcodes that in every MapleStory
client are owned by `CClientSocket` rather than by a stage. **[D]** That is the mscw-side
half of the argument, and it is why §7 sweeps `0x19..0x22` rather than guessing one number.

---

## 5. The request, `0x00D2`, is now [L] — the field order was previously [D]

`crates/net/src/channel.rs` records the `0x00D2` field order as unsettled ("call order is not
wire order"). The capture settles it:

```text
04:16:27.760 <- 0x00D2, 19 byte body
   01              u8   target channel, 0-based   <- CH.2 in the dialog
   2d 2c 01 0d     u32  tick
   64 00 00 00 47 58 00 00 00 00 00 00 00 00      the 14-byte FUN_140c7b890 preamble,
                                                  which always opens with the literal 100
```

**Target channel first, preamble last.** `ChangeChannelRequest::parse`'s discriminator picks
this branch correctly today, so nothing changes — but the layout is measured now, not
inferred. **[L]**

---

## 6. What has to precede it: nothing new

The login migrate is paired with `create_migration`, and the question was whether the channel
equivalent needs more. It does not, and the reason is worth stating because the channel-side
migrate body has **no room** for a character id or a seed — it is seven bytes.

* The client's migration hello `0x007D` carries the **character id at body offset 8**, and
  **not** the seed — measured 2026-08-19 and documented in
  `crates/world/src/session/mod.rs::migration_hello_character`. So the handoff was never keyed
  on anything the migrate packet carries.
* `Session::claim_for_character` claims by character id and checks the row's world/channel
  against the channel it landed on. Minting the row with the **target** channel, which
  `on_change_channel` already does, makes that check pass on channel 1.
* Character state is **write-through** (`store.save_character_progress` on every mutation), so
  there is no save-on-disconnect to race. The map and position ch1 will send in its `SetField`
  are already in the database when ch0's socket dies.
* The client's `+0x2260` "current channel" is corrected by ch1's own `SetField` — the `u32` at
  body offset 8 goes straight to the `+0x2260` setter (`research/channel-select.md` §9.9). No
  extra packet.

**So the only thing wrong with `Session::on_change_channel` is the opcode and the body.**

**Nothing here authenticates.** A `u32` seed identifies a pending migration; the game socket
still carries no credentials, and a channel change does not change that.

---

## 7. WIRE IT LIKE THIS

`crates/world/src/session/field.rs::on_change_channel` and
`crates/world/src/session/field.rs::change_channel_refused` are the only two places that
change. **Do not touch `net::opcode::MIGRATE_COMMAND` (`0x0011`) or `net::opcode::migrate`** —
they are the *login* migrate, they work, and they are a different packet.

### 7.1 The new constant and builder (belongs in `crates/net/src/channel.rs`)

> **THE BLOCK BELOW IS WRONG AND IS KEPT ONLY TO SHOW WHAT WAS PREDICTED.** It is
> paste-ready Rust declaring `0x001B`, and the shipping code points readers at this file.
> **The measured opcode is `0x001A`** - a ten-candidate sweep sent them all and the dispatch
> line for `0x001A` took 354 ms where its neighbour took 64 us, because the long one is a
> socket teardown. The owner of that fact is
> `crates/world/src/session/field.rs::MIGRATE_COMMAND_CHANNEL`.
>
> The prediction was honest - tagged **[I]**, from a block alignment - and it was still one
> off, which is what [I] means. Kept rather than deleted because the reasoning that produced
> it is sound and the next `[I]` opcode guess will look exactly like it.

```rust
/// The channel stage's own migrate command, handled by `FUN_1415d8c00` - a socket-level
/// handler, not a case of `FUN_142cbaa80`. `research/change-channel-reply.md`.
///
/// **The number is [I]**, from a +0x0A block alignment anchored on seven mscw-confirmed
/// SOCKET opcodes; the handler and its body are [L]. It cannot be read statically: the
/// dispatch is Themida VM bytecode and `.themida` has no file bytes.
pub const CHANNEL_MIGRATE_COMMAND: u16 = 0x001B; // <- WRONG. SEE BELOW. DO NOT PASTE.

/// Body of a [`CHANNEL_MIGRATE_COMMAND`]. Three fields, seven bytes - `FUN_1415d8c00`
/// reads `u8, u32, u16` and stops. Read count cross-checked between `tools/listing.py`
/// and `tools/reads.py`.
pub fn channel_migrate(addr: std::net::SocketAddrV4) -> Vec<u8> {
    let mut out = Vec::with_capacity(7);
    out.push(1);                                   // ok
    out.extend_from_slice(&addr.ip().octets());    // network order, unchanged into sin_addr
    out.extend_from_slice(&addr.port().to_le_bytes()); // the client htons() it
    out
}

/// A refused channel change: one byte. `FUN_1415d8c00` logs error 0x21000002 at line 0x63A
/// and returns - no throw, no dialog. It is an answer, which is what matters.
pub fn channel_migrate_refused() -> Vec<u8> {
    vec![0]
}
```

For 127.0.0.1:8486 that is exactly **`01 7f 00 00 01 26 21`**.

### 7.2 `on_change_channel`

Keep every guard as it stands — the parse, the claimed-character check, the "no address for
that channel" check, the "already on it" check, and `store.create_migration(account, character,
world, target)`. Change only the two `Reply`s:

```rust
vec![Reply {
    opcode: net::channel::CHANNEL_MIGRATE_COMMAND,
    body:   net::channel::channel_migrate(addr),
    what:   format!("Change Channel: character {} to channel {target} at {addr}, \
                     seed {seed:#010x} - single use, NOT authentication. \
                     OPCODE IS [I]: research/change-channel-reply.md §4", claimed.character_id),
}]
```

and the refusal to `channel_migrate_refused()` under the same opcode.

### 7.3 Ordering against the socket close — the part that produces a hang if it is backwards

1. Write the reply. **Do not close the socket, do not shut down the write half, do not drop
   the session.** §3: `FUN_142caa360` calls `FUN_1415d35f0(conn)` itself, and the login
   capture shows the client closing 8 ms after a migrate it understood.
2. Expect an inbound **`0x0118`** on ch0 within a few milliseconds of the reply
   (`u32 tick, u8, u8, u32`). **Log it and do not error on it.** It is a notification, not a
   request, and the client is already tearing down — but `CLAUDE.md`'s always-answer rule
   means it must not be met with an error path either.
3. Expect ch0's read to return EOF. Treat that as an ordinary disconnect.
4. Channel 1 gets a fresh connection, its greeting, and a `0x007D` hello carrying the
   character id. `claim_for_character` finds the row minted in step 0 and `wrong_channel` is
   false. From there it is ordinary field entry.

### 7.4 The one thing that must be added for the test in §8

A GM command — **`crates/world/src/session/gm.rs` is a coordinator-owned file and I have not
touched it** — of the shape:

```text
!migsweep         send opcodes 0x0019..0x0022 in ascending order, 300 ms apart,
                  each with body  01 7f 00 00 01 <port LE>  followed by 64 zero bytes,
                  where <port> is the address of channel 1 from --channels.
                  Log every one, with its own timestamp, to world.log.
```

The 64 zero bytes are padding, not payload: `FUN_1415d8c00` reads seven bytes and stops, but
if one of the other nine opcodes *is* handled by something, a fixed-width reader gets zeros
instead of running off the end of the packet. **Running off the end throws.** [L], read out of
the u8 primitive itself:

```asm
1406e8af2  mov edi,[rcx+0x18] / sub edi,[rcx+0x24]   ; remaining = end - cursor
1406e8b39  cmp edi, 1
1406e8b3c  jb  1406e8b51
1406e8b51  mov edx, 0x26 / lea rcx,[rsp+0x28] / call 1401bb8b0
1406e8b6c  call 142ef6d4c   ; _CxxThrowException(&exc, ThrowInfo)
1406e8b71  int3             ; never returns
```

and that unwind is the one `research/setfield-fault-shape.md` shows faulting in the client's
own error path. So the padding is a real precaution, not decoration.

Ascending order matters: the first opcode that **is** the migrate closes the socket, so the
rest are never written, and `world.log`'s last line before the close **names the answer**.

---

## 8. THE SINGLE DISCRIMINATING CLIENT TEST

One run. It identifies the opcode *and* performs the channel change if it is in range.

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe -Probe "watch@1415db360:ret,141b2a280:rdx=0,1415d8c00:hits=8,141b36f60,140304100:hits=200:dump=143AC2400/968"
```

Five of the six watch slots, chosen deliberately:

| slot | why it is there |
|---|---|
| `1415db360:ret` | **not negotiable** — without it the client fail-fasts at ~37 s |
| `141b2a280:rdx=0` | **not negotiable** — keeps the client unblocked |
| `1415d8c00:hits=8` | **the measurement.** Fires the instant the migrate handler is entered, before the `u8` is even read |
| `141b36f60` | the **login** migrate handler. It fires on every launch at character select, so it is a second positive control that says *a watch on a migrate handler works in this build* |
| `140304100:hits=200:dump=...` | the standing positive control. No lines from it means the hook never armed and the whole run proves nothing |

**What the owner does:** log in, enter the world, stand still, then type **`!migsweep`** in the
chat box. That is the whole test. (Do *not* also use the Change Channel dialog in this run —
one variant at a time.)

### What each outcome means

| on screen / in the logs | meaning |
|---|---|
| **the world reloads and the character is playing again** — and `world-ch1.log` grows past its 23-line banner, with a connection and a `0x007D` hello | **Solved.** `world.log`'s last outbound line before `ch0 ... closed` names the opcode. Copy all four logs into `research/fixtures/change-channel-migrates-<opcode>-*.log` before `previous-runs/` rolls |
| **ch0 closes within ~50 ms of one of the sweep packets, but nothing ever connects to 8486** — a "dropped session", the client sitting on a dead screen | The opcode is right and the *reconnect* failed. Check the `1415d8c00` watch: **a WATCH line means the handler ran** and the next question is the address (read the port bytes back out of `world.log`) or the firewall rule, which is scoped to the patched exe. **No WATCH line** means something else in `0x19..0x22` closed us — a security or disconnect handler — and the sweep needs narrowing |
| **the client HANGS** — UI dead, the quit prompt's OK dead | Not the migrate. One of the swept opcodes reached a handler that read past the end of a 71-byte body and threw. Read `client-patched\maplecw-hook.log` for a `C++ THROW` and `client-exit.log` for the fault; the last opcode in `world.log` names the culprit, and the next run sweeps the range minus that one. **A hang is an unanswered/unwound packet, never "the migration was refused"** |
| **the character bounces back onto channel 0** — the world reloads but the Change Channel dialog still shows CH.1 as current | The migrate ran with the **wrong address**. The port little-endian is the first suspect: `26 21` is 8486, `25 21` is 8485. `world.log` has the bytes |
| **nothing at all happens and the session carries on normally** | The migrate opcode is not in `0x19..0x22`. The `1415d8c00` watch will be silent and `141b36f60`/`140304100` will not be — which is what makes this a *result* rather than an unarmed instrument. Next sweep: `0x24, 0x28, 0x2A, 0x2C..0x33` (§4.2), still the caseless slots of the SOCKET block |

**The cheapest tell of all, and it needs no hook at all:** the gap between the reply and
`ch0 ... closed`. §3 measured **8 ms** for a migrate the client understood and **six seconds**
for one it did not.

---

## 9. What this file cannot bear

* **The opcode number is [I].** Everything else here — the handler, its three fields, their
  byte order, the teardown order, the `0x0118` it sends — is [L] off the listing or a capture.
  Do not let §7.1's `0x001B` harden into a fact because it is written in a code block.
* **`FUN_1415d0b40` is virtualised**, so "the client closes the socket" is [D] from
  `FUN_1415d35f0`'s body plus the 8 ms login measurement, not from watching a `closesocket`.
* **The `ok == 0` path is inert as far as `FUN_1412825a0` goes** — it logs and returns. What
  `FUN_1418049f0` does with the line beyond logging is unread; it may write the ELog, which
  the client uploads as `0x008F`/`0x0090`. That is a bonus, not a hazard.
* **`0x19..0x22` is a derived range, not an exhaustive one.** It is the only caseless run in
  the SOCKET block that is not the mscw-only insertion block, and both references put
  MigrateCommand inside it. If mscw deleted `RenameCharacterResult`, the answer is `0x1A`; if
  it inserted one more, `0x1C`. The sweep covers all three and six more besides.
* **The `0x00D2` handler `FUN_142cdcef0`** — the game switch's own case at that number, from
  `research/msexe-gamestage-opcodes.md` — was not read. It is *inbound* `0x00D2`, a different
  packet from the client's outbound `0x00D2`; the two share a number and nothing else.
