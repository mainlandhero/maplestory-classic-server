# Why the chat line renders nothing: the flag byte, and the route it has to survive first

**Round 2, 2026-08-20. Static only - no client run, no Ghidra.** Written beside
`research/user-chat.md`, which is not edited. Where this contradicts that file it says so
explicitly, in "What round 1 got wrong".

Tags: **[L]** read off a listing, a table or a capture; **[D]** derived from one of those;
**[I]** inferred, i.e. a guess with a reason.

---

## The one-line answer

**`crates/net/src/userchat.rs` was sending `0` in a byte the client uses as a bitmask, and
bit 1 of that byte is the balloon.**

```text
0x142784a5b   READ u8   ->  byte [rbp+0x168]        the first u8 after the speaker object
0x142785722   test byte [rbp+0x168], 2
0x142785729   je   0x142785b96                      <- past BOTH FUN_14158f5c0 call sites
```

`rbp` is `lea`'d once at `0x142784980` (`lea rbp,[rsp-0x110]`) and never written again in the
whole 5276-byte function - the only `rbp` destination in the listing - so the slot that the
`u8` lands in at `0x142784a60` is the slot the balloon test reads at `0x142785722`, with no
intervening write (the next two writes to `[rbp+0x168]` are at `0x142785995` and
`0x142785b2c`, both after the test). **[L]**

With bit 1 clear a balloon is not reachable however correct the rest of the body is. That is
now fixed in `userchat.rs`, with a byte-for-byte test. **It is still not sufficient** - see
"the route" below, which has two silent drops in front of the handler that no amount of body
correctness can get past.

---

## 1. The route, end to end, every hop read out of the image

`0x0231` is **not** in the game-stage switch. `FUN_142cbaa80` covers `0x70..0x19f` plus the
two outliers `0x275` and `0x39a` (`research/msexe-gamestage-opcodes.md`, fact 1). The chain
that actually carries it is:

| # | at | what |
|---|---|---|
| 1 | `0x141821e24` | `CField::OnPacket FUN_141820080`: `lea eax,[r9-0x224] / cmp eax,0x17b / ja` -> `mov rcx,[0x143AC1B90]` (**no null check**) `/ call 0x1429b9300`. So `0x224..0x39F` -> the user pool. |
| 2 | `0x1429b932e` | `FUN_1429b9300`: `lea eax,[rdx-0x226] / cmp eax,0x6c / ja` -> **tail `jmp 0x1429bafb0`** at `0x1429b934b`. So `0x226..0x292`. |
| 3 | `0x1429bb08d` | `FUN_1429bafb0` reads the leading `u32` (the characterId) - for every opcode except `0x226`, which has its own preamble at `0x1429bafcc`. |
| 4 | `0x1429bb099` | `call 0x1429b6c90` = `CUserPool::GetUser(id)`. **NULL -> `cmp esi,0x226 / jne 0x1429bb5b0` -> return, having done nothing.** |
| 5 | `0x1429bb100` | `lea eax,[rsi-0x226] / cmp eax,0x50 / ja` then a 0x51-entry table at `0x1429bb5d0`, base `0x140000000`. |
| 6 | index 11 | `0x1429bb134 -> call 0x142784970`. |

**[L]** throughout. Step 6 was re-derived independently of round 1 by reading all 81 RVAs out
of `client-patched/MapleStory.exe` and disassembling each case body; the full table is in
section 6 below. `tools/handler_root.py 0x142784970` prints the same chain from the other
end (`FUN_142784970 <- FUN_1429bafb0 <- FUN_1429b9300 <- FUN_141820080`), which is a second
instrument agreeing rather than the same one twice.

### The route above the pool is proved live by a capture

`previous-runs/world-20260820-012657.log` shows the server sending `0x03C6` (x30), `0x03D2`
(x30), `0x03E4` (x480), `0x044F` (x3) and `0x0453` (x4), and the client answering with
`0x02FF CLIENT_MOB_MOVE` for those mobs. `0x3C6..0x44E` is routed by the **same range chain
in the same function**, four `lea`s further down at `0x141821e66`, and `0x44F..0x468` at
`0x141821e88`. So `FUN_141820080` is entered, its range chain runs, and mobs and NPCs arrive
through it. **[L]** - this rules out "the packet never reaches `CField::OnPacket`".

**It does not rule out steps 4-6**, and that is the honest state of it: in every log in
`previous-runs/`, **no packet in `0x224..0x39F` has ever been observed to do anything**. The
only one ever sent is the single `0x0231` of 2026-08-19 02:07:34 (`world-20260819-220806.log`,
`UserChat: 204 (TestCharD) says "Hello David"`), and that one drew nothing. The user-pool
branch of the chain is entirely unexercised.

---

## 2. The flag byte, in full

`FUN_142784970` tests three separate bits of the `u8` read at `0x142784a5b`:

| bit | test | at | clear -> | what it selects |
|---|---|---|---|---|
| 0 | `test al, 1` | `0x142784ece` | `je 0x142785537` | a chat post through `FUN_1415a8b80` (10 arguments, `0x14278551c`) |
| 1 | `test byte [rbp+0x168], 2` | `0x142785722` | `je 0x142785b96` | **the balloon** |
| 2 | `test al, 4` | `0x142785537` | `je 0x14278554f` | a post through `FUN_1415ed1c0(.., 0x1f)` at `0x14278554a` |

All **[L]**, from `python tools/listing.py 0x142784970`.

**The same convention, at the same field position, in the sibling handler.** `0x0226` ->
`FUN_1427847a0` -> `FUN_1427834b0`. `FUN_1427847a0` reads its own first post-speaker `u8`
into `r14b` at `0x14278485e` and passes it as the 5th argument (`mov byte [rsp+0x20], r14b`
at `0x1427848d0`). In `FUN_1427834b0` the 5th argument is `[rbp+0x100]` (entry `rsp = R`,
seven pushes, `lea rbp,[rsp-0xa0]` -> `rbp = R-0xd8`, so `rbp+0x100 = R+0x28`), and it is
tested:

```text
142783aa0  test al, 1                       (al = movzx [rbp+0x100] at 142783a99)
142783af1  test al, 4
142783dcf  test byte [rbp+0x100], 2   ->  je 0x142784300, past all three FUN_14158f5c0 sites
```

**[L]** Two independently decoded handlers, one bitmask, bit 1 is the balloon in both. That
is the strongest thing in this document: it is not an alignment against a reference tree, it
is the same test twice in the client's own code.

### What the client puts in the equivalent byte when it speaks

The outbound `0x00E7` has two builders in the image (`research/msexe-send-opcodes.txt` lines
152-153; both were read, not just the named one):

```text
FUN_1418cd030:                                FUN_141824980:
  1418cd219  mov edx, 0xe7                      141824a26  mov edx, 0xe7
  1418cd223  call 0x1406ed520   COutPacket      141824a2b  call 0x1406ed520
  1418cd229  call 0x1429e3ef0   a tick          141824a31  call 0x1429e3ef0
  1418cd235  call 0x1406ed9d0   encode u32      141824a3d  call 0x1406ed9d0
  1418cd242  call 0x1406edc80   encode str      141824a4a  call 0x1406edc80
  1418cd247  mov dl, 3          <-- CONSTANT    141824a4f  movzx edx, sil   <-- an ARGUMENT
  1418cd24e  call 0x1406ed840   encode u8       141824a58  call 0x1406ed840
```

**[L]** One builder hardcodes `3`; the other takes the byte from its third parameter
(`r8b`, saved to `esi` at `0x1418249a0`). **A field one caller passes as a parameter and
another hardcodes to `3` is a flag byte, not a chat-tab index**, and `3` is bit 0 plus bit 1.

Every capture agrees the byte is `03`: `research/user-chat.md`'s three 2026-08-19 bodies
(`...48656c6c6f 03`) and `world-20260820-012657.log`'s `!map 40`
(`193415080700216d617020343003`).

**That the inbound `0x0231` flag uses the same encoding as the outbound `0x00E7` flag is
[D], not [L].** Nothing has read a server-shaped `0x0231` into this client. What is measured
is (a) inbound bit 1 is the balloon, in two handlers, and (b) the client's own outbound
constant has bit 1 set.

---

## 3. What the handler does with the body, corrected

```text
0x142784998  u8    -> [rsp+0x78]      only use: `neg dword [rsp+0x78] / sbb r8d,r8d /
                                       and r8d,0xa` at 0x1427854d0 - it picks 0 or 10 for
                                       FUN_1415ed1c0, INSIDE the bit-0 block. [L]
0x1427849ac  str   -> [rsp+0x70]      the message
0x142784a53  FUN_1408d6760            the speaker object, 33 bytes
0x142784a5b  u8    -> [rbp+0x168]     THE FLAG BYTE
0x142784a69  u8    -> r13d            tested `test r13d,r13d` at 0x14278506d - inside the
                                       bit-0 block
0x142784a75  u8    -> [rbp+0x160]     reloaded into r13d at 0x142785086 - inside the bit-0
                                       block
0x142784b05  FUN_1408da090            the trailing object, 4 bytes while != 1
```

`tools/reads.py 0x142784970 3` reproduces exactly this list, and the sizes were re-checked
(`r8d` immediates at `0x1408dcbe2`, `..bf3`, `..c04`, `..c17`, `..c2a`, `..c89`, `..ccee`):
4, 4, 1, 4, 4, 4 and the tail-`jmp` 4. **The 47-byte overhead in `userchat.rs` is right and
was not touched.** [L]

**The trailing object's dword is read again by the handler**, at `0x142785064`
(`cmp dword [r14+0x10], 1 / je 0x1427851cb`) - but that instruction is inside the bit-0
block, so with bit 0 clear the value is consumed and never looked at. Round 1 only knew
about the `cmp r8d,1` inside `FUN_1408da090`.

### The two outlets, and which one the flag controls

* **The chat-log line** is at `0x1427856fa` - `mov edx,7 / lea rcx,[rbp-0x38] / call
  0x1415eca30`. It is gated **only** by `cmp dword [rbp-0x40], 0` at `0x1427856eb`, where
  `[rbp-0x40]` is the return of `FUN_1408bef90` at `0x142784db4`. Every path out of the
  bit-0 / bit-2 branches converges at `0x14278554f` and reaches this test, so **the flag byte
  does not gate the chat-log line**. [L]
* **The balloon** is at `0x142785927` **and at `0x142785ac1`** - two `FUN_14158f5c0` call
  sites, not one. Both are past `0x142785722`. Between the flag test and the first of them
  sit four `FUN_141715e10(user, id)` probes with ids `0x1a`, `0x1b`, `0x1c`, `0x22`; the
  first two route to `FUN_142870250` and `jmp 0x142785b96` **without** reaching a balloon,
  the second two route to the `0x142785ac1` site, and all-false reaches `0x142785927`. [L]
  What `FUN_141715e10` tests is **[I]**: the shape is a per-user status/buff query.

### The chat-log line is assembled as `"<name> : <message>"`

`0x142784c47`: `FUN_14019ba10(&[rsp+0x68], [0x143a44e50], [r15+0x10d8], [rsp+0x70])`, and
`[0x143a44e50]` holds `0x1432a89f0` = **`"%s : %s"`** (dumped). `FUN_1427834b0` loads the
same pointer at `0x142783793`. So `r15 = the CUser` and **`user+0x10d8` is a `char*` to the
speaker's name**, sixteen bytes past the id at `user+0x10d0`. Name **[D]** (position plus
role in the format); the format string and the offsets **[L]**.

Note the consequence, which was not obvious: the string handed to the type-7 chat window at
`0x1427856f6` is `[rbp-0x38]`, the *output of `FUN_1408bef90`* - **not** the `"%s : %s"`
string, which goes to the balloon (`0x142785713`) and to the bit-2 outlet
(`0x142785541`). So "the name we send in the speaker object" is not what appears in the chat
log; the client uses its own `CUser`'s name.

---

## 4. The gates in front of the render sites, ranked by how plausible each is

| # | at | condition | verdict |
|---|---|---|---|
| A | `0x1429b6ca7` | `cmp qword [0x143AC1B90], 0 / je -> return NULL` **inside GetUser** | **round 1 missed this one.** The user-pool singleton being null makes `GetUser` return NULL *before* it dereferences anything - so a null pool is a silent drop with **no fault**, exactly what was seen |
| B | `0x1429bb0ac` | `GetUser` returned NULL (`test rax,rax` at `0x1429bb0a1`) and the opcode is not `0x226` -> `jne 0x1429bb5b0`, return | plausible: needs `[pool+0x10]`'s `CUser` to carry id 204, or a hash entry for it |
| C | `0x142785722` | flag bit 1 clear | **certain, and it was true** - this is the finding |
| D | `0x142784b14` | `[0x143AC2F58]` is null -> allocate, `FUN_1408bd7f0`, `jmp` epilogue | plausible; `0x143AC2F58` has 112 references and one initialiser (`FUN_1408bd820`) |
| E | `0x142784b69` | `FUN_1415abcc0()` **and** `FUN_1415ab8b0()` both non-zero -> epilogue | plausible, unread |
| F | `0x142784d84` | `FUN_142d01050(world, userId)` non-zero - block/mute list | skipped entirely when the vtable call `[user+0x50]` returns non-zero at `0x142784d5b`, which is the shape of `IsLocalUser` **[I]** |
| G | `0x1427856eb` | `FUN_1408bef90` returned 0 | costs the chat-log line only |

A and B are the two that produce *nothing at all* - no balloon, no line, no crash, no packet
error. C costs the balloon and leaves the line. G costs the line and leaves the balloon.

### Who fills `pool+0x10`

`FUN_1429b5540` is `CUserPool::CUserPool`: it stores `this` into `0x143AC1B90` at
`0x1429b5563`, **zeroes `[this+0x10]`** at `0x1429b5574`, and builds a 31-bucket hash at
`+0xf8`/`+0x100`. It has one call site, in `FUN_142d347d0`. **[L]**

`pool+0x10` is written in exactly one place in the whole pool address range: `0x1429b59c7`,
`mov qword [rdi+0x10], rbx`, inside `FUN_1429b5780` - a `shared_ptr` assignment
(`lock inc qword [rbx+0x18]` two instructions earlier). `FUN_1429b5780` has **one** caller,
`0x141819ee6` inside `FUN_141818ae0`, an 8713-byte method in the `CField` subsystem. **[L]**
So the local user is installed into the pool during field setup, which demonstrably happens.
Whether the `CUser` it installs carries id 204 is **not established** and is what the watch
below reads.

(The scan that found it: `python tools/rangescan.py 0x10 0x1429b5000 0x1429d0000`, with the
tool's documented positive control checked first - `python tools/rangescan.py 0x2f4
0x141c40000 0x141d60000` must name `141cb7ef3`, and it did, exactly once.)

---

## 5. The candidate list, enumerated rather than filtered

| candidate | status |
|---|---|
| **the opcode is wrong** - the game-stage dispatcher drops it | **[L] no.** `0x0231` never goes near `FUN_142cbaa80`; it is routed by `CField::OnPacket` at `0x141821e24` and index 11 of the table at `0x1429bb5d0` resolves to `FUN_142784970`. Two instruments agree (the table read, `handler_root.py`) |
| **the body is wrong** - a width, a missing field | **[L] no.** `tools/reads.py` at depth 3 reproduces the seven-entry list and every raw size; 47 + text is right |
| **the body is wrong** - a value that gates rendering | **[L] YES. This is it.** The flag byte at `0x142784a5b` went out as `0`; bit 1 is the balloon |
| **handled and deliberately dropped** - a gate inside the handler | open, ranked D/E/F above. Not measured |
| **dropped before the handler** - unknown speaker | open, ranked A/B above. **Not measured, and round 1's §1 covered only half of it** |
| **renders but is invisible** | **[D] no**, for the balloon: it is not a colour or a layer, it is a `je` past the call site. For the chat line, the type-7 window is the one that shows `[Welcome] Welcome to MapleStory!!` in every capture (`research/talking-back.md`), so it is on screen |
| **more than one chat packet** | **[L] yes, and `0x0231` is one of the two.** See below |

### There are exactly two chat packets in the user pool, and one balloon-only one

Every one of the 81 table entries was resolved to its case body and its call target, then
each target tested for reachability of the chat printer `FUN_1415eca30` and the balloon
factory `FUN_14158f5c0` at depth 3, with `FUN_142784970` as the positive control (it must
reach both; it does):

| opcode | handler | reaches |
|---|---|---|
| **`0x0226`** | `FUN_1427847a0` -> `FUN_1427834b0` | chat **and** balloon |
| **`0x0231`** | `FUN_142784970` | chat **and** balloon |
| `0x0232` | `FUN_142785E20` | **balloon only** - the function immediately after ours in the image |
| `0x0236`, `0x0238`-`0x023f`, `0x0242`, `0x0246`, `0x0247`, `0x0249`, `0x024c`-`0x024e`, `0x0250` | | chat only |

Also read out of the table, and worth having: `0x0227`-`0x0230`, `0x0237`, `0x024f`,
`0x025b`, `0x0265` and `0x0275` **share the `default` case** at `0x1429bb53f` ->
`FUN_142795b20`. Ten consecutive opcodes with no handler is a hole in this client's enum, not
a hole in the reading. (The default case reaches both printers at depth 3; that is a
reachability superset through a generic helper, not a claim that it renders chat.)

`0x0226` remains a live alternative, and it is *not* just "the same thing with a name". Its
dispatcher preamble at `0x1429bafcc` reads a `u32`, and **if that `u32` is zero reads a
second one** (`0x1429bb001` / `0x1429bb00c`), then does a `% 27` test on it before `GetUser`;
its handler reads `u8, u32, str, str, ...` and **discards the first string entirely** (it is
written to `[rsp+0x40]` and only ever freed, at `0x142784937`). Untested and unsent.

---

## 6. What round 1 got wrong

`research/user-chat.md` is not edited. These are the specific claims this round falsifies or
corrects, with what the original evidence actually was.

**1. "`FUN_1429bafb0` ... has zero direct callers - it is virtual."** *(user-chat.md line
128, and the same phrasing in `userchat.rs`'s old doc comment.)*

**Falsified.** It is reached by a **tail `jmp` from `FUN_1429b9300` at `0x1429b934b`**, which
is itself called from `CField::OnPacket` at `0x141821e3c`. It is not in any vtable and it is
not virtual.

The evidence for "virtual" was `tools/callers.py` returning zero, and **`tools/callers.py`
counts `call` only** - `python tools/callers.py 0x1429bafb0` still prints
`0 call site(s) in 0 function(s)` today. `tools/handler_root.py 0x142784970` prints
`FUN_1429bafb0 ... called by 1` for the same function in the same minute. **This is the same
instrument bug that shipped the chat packet four bytes short**: a tail `jmp` into a function
is a caller, `tools/reads.py` was fixed for exactly this in the read primitives, and
`callers.py` was not. A clean confident zero from a scan of the wrong shape.

Nothing downstream of that claim was wrong - the table, the index and the handler are all
correct - but "it is virtual" was an inference dressed as a finding, and it is the kind that
sends the next person looking for a vtable that does not exist.

**2. "§1 The dispatcher can drop the packet before the handler exists"** - correct, and
**incomplete in a way that matters**. It quotes `FUN_1429bafb0` from `0x1429bb08d` and treats
`GetUser` as a lookup that can only fail by not finding the id. `FUN_1429b6c90`'s **first
instruction pair** is `cmp qword [0x143AC1B90], 0 / je -> return NULL` at `0x1429b6c9a`, so
it also fails when the pool singleton is null - and `CField::OnPacket` passes that same
global as `rcx` **without a null check** (`0x141821e35`), so a null pool is a silent NULL
return rather than the fault you would expect. Two distinct failures, one symptom.

**3. "§3 The two render sites"** names the balloon site as `0x142785927`. There are **two**
`FUN_14158f5c0` call sites reachable from this handler, `0x142785927` and `0x142785ac1`, and
which one runs depends on the four `FUN_141715e10` probes. Round 1's run watched one of two
- and CLAUDE.md's own rule is enumerate before you filter.

**4. "What is not known: the four u8s ... one of the trailing ones suppresses the chat-log
line so that only the balloon shows. That is [I]."** Now **[L]**, and **the polarity was
backwards**: the first trailing `u8` is a bitmask that *enables* outlets rather than
suppressing them, and it is the **balloon** that it gates, not the log line. Sending `0`
asked for neither. The log line is gated by something else entirely (`FUN_1408bef90`'s
return).

**5. The watch recipe in §4 could not have been run.** It reads
`watch@142784970,14276df20:peek=10d0`. Two problems, either fatal:

* It names only those two targets, so it **drops `1415db360:ret` and `141b2a280:rdx=0`**.
  `tools/test-server.ps1` lines 17-21 and 101 call both mandatory - without the first the
  client `__fastfail`s on the reachability check, without the second the login dialog blocks
  the button that reaches character select. The run would never get to a chat box. The
  launcher's own comment at line 161 warns that replacing the probe string "replaces ALL FOUR
  slots, which silently drops `140304100`" - the same trap, one slot over.
* `FUN_14276df20` is `mov eax,[rcx+0x10D0] / ret`, **417 call sites in 289 functions**. With
  the default cap of 32 it would spend its budget and disarm itself in the first frames after
  arming, long before anyone types - and `probe.rs`'s own docstring names that exact failure
  ("a thread-exit function in a client that recycles threads would spend the cap early ...
  which reads exactly like a function that never ran").

**6. The evidence for "both were watched; both were zero" is no longer in the repo.** No file
under `research/fixtures/`, `previous-runs/` or `client-patched/` mentions `142784970`,
`1427856fa` or `142785927`. `previous-runs/` holds only `login-*` and `world-*`, never a hook
log, and no chat fixture was ever copied across. That is CLAUDE.md's "copy anything that
settles a question into `research/fixtures/`" not being done. The claim may well be true; it
is currently unauditable, and the run below re-measures it anyway.

---

## 7. What I did NOT establish

* **Whether `FUN_142784970` runs at all.** Everything in section 4 rows A and B is unmeasured.
  A fixed flag byte changes nothing if the handler is never entered.
* **What `[localuser+0x10D0]` holds.** Round 1's section 5 established that the id is a
  constructor argument, not a later patch; it did not establish the value, and neither did I.
* **What `FUN_1408bef90` is or when it returns zero.** 5607 bytes, COM-ish imports, and it
  opens with `call [import] / sub eax,[rbx] / cmp eax,0x2710 / jb` - a 10-second throttle
  against a timestamp in `[0x143AC2F58+0]`. It is called from seven functions including
  `FUN_14285ff30`, which is the `0x226`-with-no-user fallback. A throttle in front of the
  chat-log gate is worth knowing about before believing any single-message test of the log
  line. **Unread.**
* **What `0x143AC2F58` and the pair `0x143ACAB80`/`0x143ACAB70` are.** Named, not read.
* **What `FUN_141715e10(user, 0x1a|0x1b|0x1c|0x22)` tests.** The two `true` branches
  bypass the balloon entirely, so if the balloon still does not appear with bit 1 set, this
  is the next thing to read.
* **What bits 0 and 2 render.** Bit 1 is measured; bits 0 and 2 select `FUN_1415a8b80` and
  `FUN_1415ed1c0(.., 0x1f)` and nothing here read either.
* **Whether `0x0226` would work better.** Its extra `u32`, its discarded first string and its
  `% 27` preamble are all unexplained.
* **Anything about a second player.** The local echo is still a local echo.

---

## 8. Ranked list of what to change

**1. Send `3` in the flag byte at `0x142784a5b`.** *Done in `crates/net/src/userchat.rs`;
`cargo test -p net` is 232 passed / 0 failed and `cargo clippy --workspace --all-targets` is
silent.* Support: `0x142785722` `test byte [rbp+0x168], 2 / je 0x142785b96`; the same test at
`0x142783dcf` on the same argument in `FUN_1427834b0`; and the client's own `mov dl, 3` at
`0x1418cd247`. `user_chat_with_flags` exists so the value can be bisected without editing
this file again, and `CHAT_FLAG_BALLOON` / `CHAT_FLAG_CHAT_WINDOW` / `CHAT_FLAG_BIT2` name
the bits.

   *If that run faults*: `3` opens **two** code paths that have never executed (bit 0's
   `FUN_1415a8b80` block, which is also where the trailing object's dword starts to matter,
   and bit 1's balloon). `user_chat_with_flags(id, text, CHAT_FLAG_BALLOON)` is the smaller
   half and is the fallback. Neither path reads the packet, so an underrun is not the
   failure mode to expect.

**2. Confirm the handler is entered, before spending another run on the body.** Section 4
rows A and B. The watch in section 9 answers it in the same run as change 1, at no extra
cost. If it never fires, everything below this line is premature and the next work is
`0x0224` (`FUN_1429ba3e0`) - putting our own character into the user pool - not more chat
bytes.

**3. If the handler runs and the balloon appears but the chat-log line does not**, the gate
is `[rbp-0x40]` at `0x1427856eb`, i.e. `FUN_1408bef90` returned 0. Watch `0x1427856fa`
directly; if it never fires, read `FUN_1408bef90`, starting with the `cmp eax,0x2710`
throttle at `0x1408befdd`.

**4. If the handler runs and nothing appears**, bisect gates D and E with one watch each:
`0x142784b41` (past the `[0x143AC2F58]` null test) and `0x142784b6f` (past the
`FUN_1415abcc0`/`FUN_1415ab8b0` pair). Both are round 1's addresses and both still stand.

**5. Try the speaker's name in the speaker object's first string.** Lower priority than it
looks: the chat-log line takes its name from `[user+0x10d8]`, the client's own `CUser`, not
from anything in our body.

**6. Do not switch to `0x0226` yet.** It is a real second chat packet, but it costs an
unexplained `u32`, an unexplained discarded string and a `% 27` test, and `0x0231` has
already been proved to be accepted without a crash.

**Instrument fix, out of scope here but load-bearing:** `tools/callers.py` counts `call`
only. `tools/reads.py` was fixed for exactly this in August and `callers.py` was not, and it
has already produced one wrong conclusion in `research/user-chat.md` ("it is virtual"). Every
"N call sites" number in `research/` that a conclusion rests on is suspect until it is fixed
and the affected claims are re-run.

---

## 9. The watch recipe - one free slot, and what to put in it

**Only one slot is free.** `probe.rs` has four (`WATCH_SLOTS = 4`) and
`tools/test-server.ps1` spends three of them on every run by convention:
`1415db360:ret` and `141b2a280:rdx=0` are mandatory (lines 17-21, 101) and
`140304100:hits=200` is the positive control. `client-patched/maplecw-hook.log` from
2026-08-20 01:31 shows exactly that arrangement in slots 0, 1 and 3.

```text
watch@1415db360:ret,141b2a280:rdx=0,142784970:peek=10d0:hits=20,140304100:hits=200
```

**`140304100:hits=200` is the positive control. If it logs no lines, the hook never armed and
nothing else in that log proves anything** - including a silent `142784970`.

`rcx` at `0x142784970` is the `CUser` that `GetUser` returned (`0x1429bb137 mov rcx, rbx`),
and `FUN_14276df20` is `mov eax,[rcx+0x10D0] / ret` on that same class - so `peek=10d0` reads
the user's own id out of the object the client chose, without watching the 417-call-site
accessor. One slot, two answers.

Run it with change 1 in place - the flag byte is the one variant, the watch is observation.
The owner should type one short line (no `!`) once the character is on the map, and report the
screen.

| what the log and the screen say | what it means |
|---|---|
| no `140304100` lines at all | the hook never armed. **Discard the whole run**; nothing below is evidence |
| `142784970` **never fires**, `140304100` did | the dispatcher dropped it: gate A or B. `GetUser(204)` returned NULL - either the pool singleton `[0x143AC1B90]` is null or no `CUser` carries 204. Next step is `0x0224`, not more chat bytes |
| `142784970` fires, `[rcx+0x10d0]=u32:0x000000cc`, **and a balloon appears** | done. The flag byte was the whole story, and `3` is the value |
| `142784970` fires, id is `0xcc`, **balloon but no chat-log line** | gate G: `FUN_1408bef90` returned 0. Go to change 3 |
| `142784970` fires, id is `0xcc`, **nothing on screen** | gates D/E/F. Go to change 4 |
| `142784970` fires and `[rcx+0x10d0]` is **something other than 204** | that is the answer on its own - send that id. It would mean the client's local `CUser` is numbered independently of the stat block |
| the client dies | note the fault code before anything else. `0xE06D7363` would mean a read past the end, which the byte-for-byte test says cannot come from this change; a `0xC0000005` inside `0x1427849xx`-`0x142785xxx` means bit 0 or bit 1 opened a path that dereferences something we did not supply - retry with `CHAT_FLAG_BALLOON` alone |

The second run, whatever this one says, should put `1427856fa:hits=20` or `142785927:hits=20`
in the free slot - but only after this one, because "did the handler run" makes every one of
those observations interpretable and nothing else does.
