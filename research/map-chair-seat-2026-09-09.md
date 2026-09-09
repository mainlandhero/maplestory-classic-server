# Map chairs: the reply is `0x0252`, and three of the four things it was built on were wrong

2026-09-09. **No Ghidra** (the project lock is held elsewhere), **no client run**, `crates/`
not touched. Instruments: `tools/listing.py`, `tools/reads.py`, `tools/callers.py`,
`tools/dis_at.py`, `tools/dump_va.py`, `tools/pdata_lookup.py`, and three throwaway sweeps
whose source and controls are in section 9.

Tags on every claim: **[L]** read off this client's listing or table bytes, **[D]** derived
from two or more [L], **[I]** inferred.

Companion: `research/chairs-2026-09-08.md` §10-§17 is the state this replaces in part.
`research/user-pool-tables.md` already contained the answer's row and nobody had read it.

---

## 0. The answer

**Opcode `0x0252`, `CUser::OnSitResult`, body `u32 characterId; u8 bSit; if (bSit) u16
seatIndex`.** Seven bytes to seat, five to release. **[L]** for the chain, the widths and the
read count; **[I]** only for the names.

```text
  4  u32  characterId     consumed by the dispatcher at 0x1429bb08d, BEFORE the arm
  1  u8   bSit            0x1428341e2 - non-zero = sit, zero = stand
  2  u16  seatIndex       0x1428341f7 - READ ONLY IF bSit != 0.  ZERO-extended.
  -------
  7  bytes to seat.  5 bytes to stand (the u16 is not read and must not be sent).
```

`tools/reads.py 0x1428341c0 6` reports **exactly two** reads, and `tools/reads.py
0x14290c6e0 6` (everything downstream of it) reports **zero**. The dispatcher's own head
reads exactly one `u32` for any opcode that is not `0x0226`. So the body is 4+1+2 and there
is nothing hidden behind a helper. **[L]**

Three further facts that matter more than the layout:

* **`0x0252` seats the LOCAL player as well as a remote one**, because its dispatcher looks
  the character up with `CUserPool::GetUser`, which checks `pool+0x10` - the local user -
  *before* the remote hash. `0x02AD`'s dispatcher does not: it inlines a hash-only lookup and
  can never reach the local player. That single difference is why the chair relay worked and
  every map-chair attempt did not. **[L]**
* **`0x0252` clears the exclusive-request latch itself** (`FUN_142cc4430(ctx, 0)` writes
  `ctx+0x2330 = 0`), so it is self-unlocking. **[L]**
* **The client re-checks the seat against the player's position and refuses a bad one** - and
  its refusal is to send `0x00DA 0xFFFF`, a stand request. Section 6. **[L]**

**Nothing here has been on a screen.** Section 8 says what to watch and what each outcome
would mean.

---

## 1. `FUN_142d51930` is NOT SetField, and §17.2 is withdrawn in full

The question was "is `FUN_142d51930` the `SetField` handler, the shared character-record
decoder, or something else?" It is **none of the three**: it is inbound **`0x0070`**,
`CWvsContext::OnInventoryOperation`. **[L]**

Two independent instruments say so, neither of them mine:

* `research/msexe-setfield.md` is an entire file arguing it (five arguments, from the object
  it operates on to the mode list to the v214 field-for-field match) and its own title says
  the filename is kept only for continuity.
* `research/msexe-gamestage-cases.txt`, an enumeration of all 273 case labels in
  `FUN_142cbaa80`, has the row `0x0070  FUN_142d51930` - written for an unrelated purpose.

`SetField` (`0x01A0`) is `FUN_142097f80`, confirmed on a live client on 2026-08-19
(`research/msexe-stage-setfield.md`), reached through the *stage*'s `OnPacket`, not the world
dispatcher.

### 1.1 The two `call [rax+0xb8]` sites are an item's quantity setter, not `SetSeat`

`FUN_142d51930` has exactly two `call qword ptr [rax+0xb8]`, at `0x142d524d2` and
`0x142d535aa`. **`rcx` is an item, not a `CUser`.** **[L]**

```text
  142d52221  call 0x1402e3cd0            ; GetItem(charData, &out, invType, pos)
  142d52226  mov  r13, [rax + 8]         ; r13 = the ref-counted item slot
  ...
  142d524c7  mov  rax, [r13]
  142d524cb  movzx edx, r14w             ; the mode-1 UpdateQuantity short
  142d524cf  mov  rcx, r13
  142d524d2  call qword ptr [rax + 0xb8]
  142d524d8  lea  rcx, [r13 + 0x20]      ; and immediately the item-id accessor
  142d524dc  call 0x14019a5d0            ;   FUN_14019a5d0(item+0x20) - the obfuscated id
```

Slot 23 of the Bundle item vtable `0x14327e588` is `+0xb8` = `0x14327e640` =
**`0x1402fffc0`**, which is 33 bytes long **[L]**:

```text
  1402fffc0  push rbx / sub rsp,0x20
  1402fffc6  movzx eax, dx               ; a u16 argument
  1402fffcc  lea  rdx, [rcx + 0x4d]
  1402fffd3  call 0x1402f70a0            ; the ZtlSecure encoder
  1402fffd8  mov  [rbx + 0x51], eax      ; the protected quantity word
```

That is `GW_ItemSlotBundle::SetNumber(u16)`. The Equip vtable's slot 23 is a *different*
function (`0x1402ffff0`), which is what a per-class slot looks like.

**So `chairs-2026-09-08.md` §17.2 is wrong and is withdrawn.** "Both `SetSeat` calls in
`FUN_142d51930` pass a `u16`, which matches `userpool.rs`'s seat at 416" was a **vtable-slot
collision** - the same failure §6.2 of that file records for slot 38 and warns about in its
own words, made again nine sections later. The `u16` really is a `u16`; it is an item count.

The conclusion §17.2 supported - "the only thing that can seat the local player is a packet
carrying a user record with field 416 set" - loses its second leg here and its first leg in
section 4.

---

## 2. `FUN_1428a81d0` is not `SetSeat`. It is `LeaveChair(bool)`, and it always writes -1

This one changes the shape of the whole search, so it is worth its own section.

`tools/callers.py 0x1428a81d0`: **one** direct call site (`0x142883fbd`, inside
`FUN_142882250`, which `lea`s the `CUser` vtable and is therefore construction/init) and
**one** pointer in the whole image - `0x143483298`, which is `0x1434831e0 + 0xb8`, slot 23 of
the `CUser` vtable. **[L]**

The body **[L]**:

```text
  1428a81fd  mov  r14d, edx                  ; THE ARGUMENT
  1428a8206  mov  [rcx + 0x4868], 0
  1428a820d  rcx = [this + 0x3c18]           ; the chair OBJECT
  1428a821c  esi = chairObj->vt[0xa0]() ; ebp = [chairObj + 0x34]
  1428a8254  call 0x141715d80 / 142952c10    ; replace the ref-counted slot at this+0x3c10
  1428a8271  test r14d, r14d
  1428a8274  jne  0x1428a8302                ; <<< non-zero SKIPS the whole visible half
  1428a827a    ... avatar work on [this+0x100], with lea edx,[r14+6] / [r14+0xd] / [r14+0x78]
               ... i.e. the constants 6, 0xd, 0x78, because r14 is 0 on this path
  1428a8302  this->vt[0x128](esi, ebp)
  1428a831b  this->vt[0x130](0xffffffff)     ; SetChair(-1) - a hard-coded immediate
  1428a8324  COutPacket(0xdc) / SendPacket   ; outbound 0x00DC
```

The argument is a **flag**, not a seat: the only thing it is tested against is zero, and
every arithmetic use of it sits on the branch where it is provably 0. The `CUser`
constructor passes `1` (silent); the `0x0318` release passes `0` (with effects). **[D]**

**It can only ever clear the seat.** Every hour §6 and §13 spent looking for "the `SetSeat`
call that carries the index" was looking for something that does not exist, and §8's watch -
the one that killed a client - was armed on the field this function unconditionally sets to
`-1`.

### 2.1 The watch's "stack:" lines in §8 are not a call stack

`§8` recorded `stack: 0x1428a8324<-TEXT 0x142f31839<-TEXT 0x142f0492d<-TEXT`. The
`called-from` is real. The other two are not frames: `0x142f31784` is the CRT's
`__acrt_getptd` (it ends `call [rip+0x330d9f]` around a TLS index) and `0x142f04924` is
**`rand()`** - `imul ecx,[rax+0x28],0x343fd / add ecx,0x269ec3` is the MSVCRT LCG. **[L]**
They are stale return addresses the scan picked up off the stack. Nothing in §8 named the
caller of `LeaveChair`, and reading those two lines as a chain is how `FUN_142d51930` came to
be believed to be SetField's decoder.

---

## 3. Body offset 416 of `0x0224` really is `CUser+0x3c28`, re-derived

Not taken from `research/user-enter-field.md` (whose table the `0x0224` fault file warns is
stale by `REMOTE_STAT_TAIL_LEN`) and not from `userpool.rs`'s comments. Re-derived by
reading the decoder's stores in address order **[L]**:

```text
  1429ce7c5  READ str                       -> 410  (the second of the two strings)
  1429ce80f  READ u32  -> [r14 + 0x2e08]    -> 412
  1429ce81e  READ u16  -> movsx -> [r14 + 0x3c28]      <-- 416, i16, SIGN-extended
  1429ce830  READ u32  -> [r14 + 0x4080]    -> 418
  1429ce843  READ u32  -> [r14 + 0x3b78]    -> 422
  1429ce852  READ u16  cwde                 -> 426
  1429ce85f  READ u16  cwde                 -> 428
  1429ce86c  READ u8   -> [r14 + 0x6e4]     -> 430
  1429ce885  READ u16  -> FUN_142df6c50     -> 431
  1429ce89c  READ u8                        -> 433
  1429ce8b1  READ u8                        -> 434
```

That is `u32, i16, u32, u32, i16, i16, u8, i16, u8, u8` and it matches
`crates/net/src/userpool.rs`'s 412/416/418/422/426/428/430/431/433/434 field for field and
width for width. **[D]**

Two of those rows are anchored to something that can disagree, which is what makes this a
derivation rather than a restatement:

* **426/428 are x and y**, and the character appears in the right place on screen - measured,
  repeatedly. The seat is ten bytes in front of the position, which is exactly what
  `userpool.rs`'s own test computes (`USER_ENTER_FIELD_POS_AT - 10 + shift`).
* **418 lands on `CUser+0x4080`**, and `+0x4080` is independently known to be the chair
  **item id** because the `0x02AD` handler writes its own chair id there
  (`1429d50ac`, `chairs-2026-09-08.md` §15.2). Two unrelated packets agreeing on one field.

Note the **sign**: `movsx ecx, ax` at `0x1429ce823`, so `ffff` on the wire becomes
`0xFFFFFFFF` and `IsSitting`'s `cmp dword [rbx+0x3c28], -1` sees the absent form. `-1` is
correct there and `0` is a valid seat, exactly as `userpool.rs` says.

`+0x3b28` (body offset 41) is a **different** field with its own accessors
(`0x14282b880`/`0x14282b890` getter/setter pair, plus `0x142834190`); it is not the seat.

---

## 4. The leading hypothesis is REFUTED: `0x0224` cannot address the local player

`0x0224`'s handler is `FUN_1429ba3e0` (`callers.py`: one call site, `0x1429b946d`, inside
`FUN_1429b9300`). Before it decodes anything **[L]**:

```text
  1429ba43b  READ u32 -> esi                 ; the character id
  1429ba4a0  READ u32 -> ebx
  1429ba4a7  rdi = [the CWvsContext global]
  1429ba4b5  call 0x142cb9550                ; mov eax,[rcx+0x232c] ; ret
  1429ba4ba  cmp  esi, eax
  1429ba4bc  jne  0x1429ba523                ; NOT me -> allocate 0x4438 bytes, construct,
                                             ;           and only then call FUN_1429ce270
  1429ba4be  ...                             ; ME -> three calls, then jmp 0x1429ba955 = the
                                             ;       epilogue. The record is never decoded.
```

`ctx+0x232c` is a character id by construction - it is compared against one - and it is the
*local* one, because the comparison's only purpose in an "a user entered the field" handler
is to reject yourself. **[D]** (Its setter `0x142cb9560` has two callers, `0x1415d8be7` and
`0x141b37fa1`; the second is in the login-stage code region.)

So "send the player their own `0x0224` with the seat index at 416" is not risky-but-untested.
**It cannot work.** The handler consumes three `u32`s and returns. Everything after that -
the character record, the seat, the 500-odd bytes - is never read.

That also disposes of the fear in `chairs-2026-09-08.md` §17.3 ("do it on a run where a
second client is expendable"): a self-addressed `0x0224` is not dangerous, it is inert. It is
also useless.

---

## 5. What actually reaches the seat: `0x0252`

### 5.1 The complete enumeration that made the search finite

Everything that can write `CUser+0x3c28`, from a `.text`-wide capstone sweep with
byte-by-byte resync (20 034 resyncs, four positive controls - see section 9) **[L]**:

| site | what |
|---|---|
| `0x142769d49` | `mov [rsi+0x3c28], 0xffffffff` - the initialiser |
| `0x1429ce826` | the `0x0224` decoder, body offset 416 |
| `0x1428341a0` | **`SetChair`** - `mov [rcx+0x3c28], edx ; ret` |
| `0x14088f478`, `0x14089f12b`, `0x14294d2af` | other classes (two `movsd` of a double, one ZtlSecure key triple) |

and the field is **read in exactly one place in `.text`**: `0x14283403e`, inside `IsSitting`.
So `SetChair` was the only door, and it is virtual-only.

`SetChair` appears at index 38 of **five** vtables and has **zero** direct callers, which is
where §6 stopped. What breaks the deadlock is going at it from the *argument* rather than the
call site: a sweep of every `call qword ptr [reg+0x130]` in `.text` (**375** sites) that
prints the last instruction to define `EDX` before each one. Almost all are `xor edx, edx` or
a small immediate. Working down the ones with a variable `EDX` finds **`0x14290c7d1`**, and
its containing function is the answer.

### 5.2 `FUN_14290c6e0` is `CUser` vtable slot 39 - the sit result

`callers.py 0x14290c6e0`: zero calls, zero tail jumps, **one** pointer, at `0x143483318` =
`0x1434831e0 + 0x138`. Read directly out of the `CUser` vtable **[L]**:

| slot | VA | what |
|---|---|---|
| `+0x58` | `0x142889030` | `xor eax,eax ; ret` - a predicate that is constant **false** here |
| `+0xb8` | `0x1428a81d0` | `LeaveChair(bool)` (section 2) |
| `+0x120` | `0x142834020` | `IsSitting` |
| `+0x130` | `0x1428341a0` | `SetChair(int)` |
| **`+0x138`** | **`0x14290c6e0`** | **`OnSitResult(int bSit, int seatIndex)`** |

```text
  14290c6e9  rsi = [the CWvsContext global]
  14290c6fd  call 0x142cc4430(ctx, 0)        ; ctx+0x2330 = 0  -> THE EXCLUSIVE-REQUEST UNLOCK
  14290c702  test ebx, ebx                   ; ebx = bSit
  14290c704  jne  0x14290c731                ;   non-zero -> the sit path
  ; ---- bSit == 0 : stand ----
  14290c70c  call [this + 0x120]             ; IsSitting()
  14290c714  je   return                     ;   not sitting -> do nothing at all
  14290c72a  jmp  [this + 0xb8](this, 0)     ;   LeaveChair(0) -> SetChair(-1), sends 0x00DC
  ; ---- bSit != 0 : sit ----
  14290c740  call 0x141892840                ; the current CField
  14290c74a  call 0x14182a0d0(field, ebp)    ; the seat's POINT: x=[rax], y=[rax+4]
  14290c76d  call [this+8]->vt[0x30]         ; my own position
             ... the gate, section 6 ...
  14290c7c9  mov  rax,[this] / mov edx, ebp / mov rcx, this
  14290c7d1  call qword ptr [rax + 0x130]    ; *** SetChair(seatIndex) ***
  14290c7d9  (gate failed) call 0x142cd3a60(ctx, 0)   ; build outbound 0x00DA / 0xFFFF
```

`FUN_142cd3a60` is the client's own stand-up request builder - the same one
`chairs-2026-09-08.md` §11.1 dissected. **[L]**

### 5.3 The packet handler, and the opcode

`callers.py 0x1428341c0` -> one call site, `0x1429bb34a`, inside **`FUN_1429bafb0`** - the
`0x0226..0x0292` dispatcher. **[L]** The handler is 95 bytes and sits **32 bytes after
`SetChair` in the binary** (`0x1428341a0` then `0x1428341c0`), in the same accessor block as
`IsSitting`:

```text
  1428341d3  call [this_vt + 0x58]           ; 0x142889030 -> 0 for a plain CUser: passes
  1428341d8  jne  return                     ; a subclass that returns non-zero drops it
  1428341e2  READ u8  -> edi                 ; bSit
  1428341ea  r8d = 0xffffffff                ; the default seat
  1428341f2  je   0x142834200                ; bSit == 0 -> DO NOT READ THE u16
  1428341f7  READ u16 -> movzx r8d, ax       ; seatIndex, ZERO-extended
  142834208  call [this_vt + 0x138](edi, r8d)
```

The opcode: `FUN_1429bafb0` normalises with `lea eax,[rsi-0x226] / cmp eax,0x50` at
`0x1429bb100` and jumps through the table at `0x1429bb5d0`. The table entry holding
`0x029bb344` (the thunk that calls `0x1428341c0`) is at `0x1429bb680` = index **44 = 0x2c**.
`0x0226 + 44` = **`0x0252`**. **[L]**

The arithmetic is controlled against two answers established for other reasons, per the
method `chairs-2026-09-08.md` §12.1 used:

* index **0** is `0x0226`, which the dispatcher special-cases by value at its head
  (`cmp edx, 0x226`) - so the base is right;
* index **11** is `0x0231`, `USER_CHAT`, which `crates/net/src/userchat.rs` names off the same
  table for reasons that have nothing to do with chairs.

And `research/user-pool-tables.md` line 763 - written 2026-08-29, for an unrelated
enumeration - already had the row, unnamed:

```text
| `0x0252` | 0x2c | `0x1429bb344` | `0x1428341c0` | 95 | 2 | u8? u16? |  |
```

**The answer had been sitting in `research/` for eleven days under a name that did not say
"chair".** That is the `CLAUDE.md` rule about fixture names, applied to a table row: grep the
contents, not the titles.

### 5.4 Why this reaches the local player and `0x02AD` never could

The two user-pool sub-dispatchers look users up **differently**, and this is the whole
explanation of the last two days **[L]**:

```text
FUN_1429bafb0   0x0226..0x0292   1429bb099  call 0x1429b6c90 = CUserPool::GetUser
FUN_1429bb720   0x0293..0x02C4   1429bb74c  r8 = [pool+0xf8] ... the hash, INLINE
```

`CUserPool::GetUser` (`FUN_1429b6c90`):

```text
  1429b6c9a  cmp  [0x143AC1B90], 0 / je -> NULL      ; the pool singleton
  1429b6ca9  rcx = [pool + 0x10]                     ; THE LOCAL USER
  1429b6cb2  call 0x14276df20                        ; mov eax,[rcx+0x10d0] ; ret - its id
  1429b6cb7  cmp  eax, ebx / je -> return the local user
  1429b6cca  ... only then the 31-bucket hash of remote users ...
```

`FUN_1429bb720` never touches `pool+0x10`. So **every opcode in `0x0293..0x02C4` -
`0x02AD` included - is structurally incapable of addressing the local player**, and every
opcode in `0x0226..0x0292` can address either. That is why the `0x02AD` relay made Tester2
see Cobalt in a chair (measured, §16) while nothing we sent ever moved Cobalt's own state.

`research/user-chat-round2.md` §4 already established that `pool+0x10` is filled during field
setup (`0x1429b59c7`, one writer, one caller, in the `CField` subsystem). **[L]**

`0x0252` is also **not** claimed by the world dispatcher: `research/msexe-gamestage-cases.txt`
(272 bodies / 273 labels from `FUN_142cbaa80`) contains exactly one case in the whole `0x02xx`
band, `0x0275`. **[D]**, and the named blind spot is that inlined cases are under-counted in
that file - so this is "not found there", not "proven absent".

---

## 6. The client re-validates the seat, and its refusal is a stand request

This is the part that will bite if the numbers are off, so it is worth writing exactly.
`ebx`/`ecx` are the seat's x/y from `CField::GetSeatPos`; `edx`/`eax` are the player's
**[L]**:

```text
  14290c769  r12d = seatX + 0x0a
  14290c770  ecx  = seatX - 0x0a
  14290c775  cmp ecx, myX / jg  fail          ; require seatX - 10 <= myX
  14290c779  cmp myX, r12d / jge fail         ; require myX < seatX + 10
  14290c75d  r15d = seatY - 0x1e ; r14d = seatY + 0x1e
  14290c781  cmp r15d, myY / jg  fail         ; require seatY - 30 <= myY
  14290c786  cmp myY, r14d / jge fail         ; require myY < seatY + 30
  fail:      call 0x142cd3a60(ctx, 0)         ; send outbound 0x00DA 0xFFFF
```

Two consequences worth naming before a run:

* **Echoing the index the client just sent is safe by construction.** The client picked it
  with `FUN_14182a220(field, myPos)` at `0x1428b5a43` in its own request builder, so it was
  within tolerance one packet ago.
* **Broadcasting `0x0252` to bystanders carries a real hazard.** On the fail path `rsi` is
  the *global* `CWvsContext`, so a bystander whose copy of the sitter's position is stale
  makes **its own** player send `0x00DA 0xFFFF`. A bystander could stand up because someone
  else sat down. **[D]** - the code is [L], the scenario is derived. If that shows up on a
  two-client run, send `0x0252` to the sitter only and find another way to replicate.

---

## 7. Answers to the four questions, in order

1. **`FUN_142d51930` is inbound `0x0070`, `OnInventoryOperation`** - neither SetField nor the
   character-record decoder. `SetField` is `0x01A0` -> `FUN_142097f80` (stage `OnPacket`),
   and the shared record decoder is `FUN_140304b20`. Section 1.
2. **Neither of its `[rax+0xb8]` calls is `SetSeat`.** `rcx` is a `GW_ItemSlot` from
   `GetItem`, and slot 23 of the Bundle vtable is a `u16` quantity setter that ZtlSecure-
   encodes into `item+0x4d/+0x51`. The `u16`s are the mode-1 and mode-8 quantities, not a
   seat. Our `SetField` builder writes neither. Section 1.1.
3. **Yes - offset 416 is `CUser+0x3c28`**, re-derived from the decoder's read order and
   anchored on x/y and on `+0x4080`. It is an `i16`, sign-extended. Section 3.
4. **Yes, there is such a path, and it is `0x0252`.** The honest form of the old answer
   ("only a whole user record can carry a seat") was wrong in both directions: the record
   cannot do it for the local player at all, and a 7-byte packet can. Sections 4 and 5.

---

## 8. What is NOT established, and how the next run reads

**Not on a screen.** Every step above is static. What is measured is only the negative:
The owner's client sends `0x00DA` with `1800`/`1900` and stays standing.

Specifically unverified:

* that `bSit` non-zero means sit rather than some other discriminator - it is read [L],
  its *meaning* is [D] from what the two branches do;
* that the seat index we echo is in the same numbering as the one the client sent. It should
  be: the client got it from `FUN_14182a220(field, pos)` and the reply feeds
  `FUN_14182a0d0(field, idx)`, and both index the field's seat array. **[D]**, untested;
* that `[vt+0x58]` is 0 for whatever concrete `CUser` class the local player has at that
  moment. It is 0 for `0x1434831e0`, the vtable `tools/chairprobe.py` has measured live in
  every run. **[D]**
* whether `0x0252` needs the `0x02AD` relay as well for other players to see the *pose*.
  `0x0252` sets the seat on every client that receives it, so it probably does not.
  **[I]**

**Reading the run** - the owner clicks a Henesys bench, the server answers
`0x0252 = <charId> 01 <seat u16>`:

| on screen / in the log | what it means |
|---|---|
| the character sits on the bench | done - and `chairprobe` should show `+0x3c28 = 24` |
| nothing happens, **and a `0x00DA ffff` arrives within a frame** | the packet was decoded and the **position gate failed** - the seat index or the seat array is the problem, not the opcode |
| nothing happens and **no** `0x00DA` at all | `GetUser` dropped it: either the id is not the local `CUser`'s `+0x10d0`, or `[vt+0x58]` is non-zero. `research/user-chat-round2.md`'s watch procedure applies unchanged |
| the client dies | the body is wrong, and it is wrong about a *value*, not a length - the length is `reads.py`-counted at depth 6 |

The middle row is the one worth arming for: it is a **different** outcome from "nothing
happened", it costs one grep of `world.log`, and it is the difference between "the opcode is
wrong" and "the index is wrong". `chairs-2026-09-08.md`'s own history is three sessions of
"nothing happened" being filed as one object.

---

## 9. The instruments, and the controls each passed first

* **`tools/reads.py`** - control run first: `python tools/reads.py 0x140304100 2` printed its
  documented output (helpers `0x1403035a0` / `0x140303b40`, then `140304138 raw`,
  `140304144 u8`, `140304183 u8`, then the run of `u16`). Only then were `0x1428341c0` and
  `0x14290c6e0` counted, at depth **6**.
* **`tools/callers.py`** - used for `0x1428a81d0`, `0x1428341a0`, `0x1429ce270`,
  `0x1429ba3e0`, `0x1428341c0`, `0x14290c6e0`. Its own docstring control was not re-run;
  every negative it returned here (`SetChair`: 0 calls) was *corroborated by finding the
  pointer instead*, which is the tool's third mode working.
* **A `.text` sweep for `[reg+DISP]` operands**, resyncing one byte at a time (20 034
  resyncs). Run for `0x3c28` and `0x3b28`. **Four positive controls for `0x3c28`**, all
  known before the run: the initialiser `0x142769d49`, the `0x0224` store `0x1429ce826`,
  `SetChair` `0x1428341a0`, and `IsSitting`'s read `0x14283403e`. All four present. It exists
  because `tools/fieldrefs.py 0x3c28` over the whole image did not finish inside ten minutes
  and returned an empty file twice; **an empty result from a tool that did not finish is not
  a negative**, and it was not used as one.
* **A `.text` sweep for `call qword ptr [reg+0x130]` / `[reg+0x138]`**, same resync, printing
  the last `EDX`-defining instruction before each site. 375 slot-38 sites (§6.3 of the chairs
  file counted 372 with a different sweep - close enough that neither is obviously broken,
  and the disagreement is recorded rather than resolved).
* **Its named blind spot**, because it is the reason this nearly failed: the first pass
  filtered to call sites whose containing `.pdata` function also calls a read primitive.
  `FUN_14290c6e0` **takes no packet** - it is handed two integers - so it was invisible to
  that filter and only appeared when the filter was dropped. That is `CLAUDE.md`'s "enumerate
  before you filter", and the filter here was a plausible one.
* **`tools/dump_va.py`** for the vtables and both jump tables; every table index quoted above
  was read as raw bytes and cross-checked against a thunk disassembled with
  `tools/dis_at.py`.
* All of it run with the repo as the working directory, and the two throwaway sweeps piped in
  as `python - < script` so `sys.path[0]` is empty and the scratchpad's stale copies of
  `reads.py`/`dis.py` cannot shadow `tools/`.

**Nothing here authenticates.** `0x0252` names a character id and the client applies it to
whoever that id resolves to; the server would have to check that the sender owns it, and that
the seat is really in that map.
