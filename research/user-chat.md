# The player's own chat: `0x0231`, and how it was found without a single client launch

**Built 2026-08-19, unconfirmed on screen.**

The owner: *"I tried to send 3 chat messages, 'Hello', 'Hello2' and 'Hello3'. The ideal response
is to have a chat bubble pop up above the character's head and I see my own chat message
down in the chat log."*

They saw neither. The capture says why, and it is not subtle: the client sent three `0x00E7`
bodies, the server matched each against `!map`, found no match, and **returned nothing**.

```
23:36:13.562 <- 0x00E7  7438da06 0500 48656c6c6f 03      "Hello"
23:36:15.862 <- 0x00E7  7a41da06 0600 48656c6c6f32 03    "Hello2"
23:36:17.322 <- 0x00E7  3847da06 0600 48656c6c6f33 03    "Hello3"
```

**The client renders nothing for its own chat.** Typing sends `0x00E7` and stops there. Both
halves of what the owner wants - the balloon and the log line - come from one packet coming back.

## The packet

**Inbound `0x0231`. 43 bytes plus the message.**

```text
u32  characterId          read by the dispatcher, before its switch
u8   flag                 0                       0x142784998
str  text                 the message             0x1427849ac
--- the speaker object, FUN_1408d6760, ALWAYS present ---
str                       empty                   0x1408d6782
str                       empty                   0x1408dcba3  } FUN_1408dcb80,
raw 4                     zero                    0x1408dcbee  } also always present
raw 4                     zero                    0x1408dcbff
raw 1                     zero                    0x1408dcc12
raw 4                     zero                    0x1408dcc25
raw 4                     zero                    0x1408dcc38
str                       empty                   0x1408dcc45
raw 4                     zero                    0x1408dcc9a
str                       empty                   0x1408dcca7
--- back in FUN_142784970 ---
u8                        0                       0x142784a5b
u8                        0                       0x142784a69
u8                        0                       0x142784a75
--- the trailing object, FUN_1408da090 ---
raw 4    MUST NOT BE 1    zero                    0x1408da0b6
```

The leading `u32` is read by the dispatcher, before its switch, so it is shared by every
opcode in the `0x226..0x276` range. Everything after it is the handler's. **[L]**

**Nothing in that list is optional.** `FUN_1408d6760` has no branch that skips the
sub-object - its only `je`s are null-pointer cleanup after each string, a pattern that
repeats throughout and which is easy to mistake for gating. Same in `FUN_1408dcb80`.

**The trailing object is stopped for four bytes.** `FUN_1408da090` reads a 4-byte block,
then `CMP r8d,1 / JNE 0x1408da1eb` returns. Any value except 1 ends it there; a 1 would send
it on to a `u8`, a sub-object and a string. **[L]**

What the speaker object's four strings and 21 raw bytes mean is **not established** - they
land in out-pointers at `obj+8`, `+0x10`, `+0x18`, `+0x1c`, `+0x20`, `+0x24`, `+0x28`,
`+0x30`, `+0x38`, `+0x40`. All empty and all zero is the shortest legal encoding. The one
worth trying later is the speaker's **name** in the first string; it is left empty because a
name in a field that turns out to be a title or a medal would render as one.

## The first version killed the client, and the way it was wrong is the point

Sent: `u32, u8, str, u8, u8` - **20 bytes**. The owner typed "Hello David" and the client exited
with **`0xE06D7363`**, an unhandled C++ exception.

The hook log named the exact chain, and unusually **all three return addresses are exact
rather than heuristic** - each is a call site plus five:

| frame | call site | |
|---|---|---|
| `0x142784a58` | `0x142784a53` | `FUN_142784970` -> the speaker object |
| `0x1408d680f` | `0x1408d680a` | `FUN_1408d6760` -> its sub-object |
| `0x1408dcba8` | `0x1408dcba3` | `FUN_1408dcb80` -> `read_str`, and nothing left |

Reconstructed: the client read the `u8`, read the 13-byte message, then took **our two
trailing zeros as an empty string's length**, and had zero bytes left when the next string
was asked for. It threw three times and died.

### Two instrument failures, stacked

1. **Direct-only read counting.** `FUN_142784970` reads through two helpers, and a scan that
   only looks for calls to the eight primitives *inside* the function sees neither. This is
   the third time on this project - `research/mob-spawn.md` records it for `FUN_141cc9410`.
2. **Scanning bytes for `0xE8` instead of disassembling.** `0xE8` is an ordinary ModRM byte.
   At `0x142784a6e` the instruction `MOVZX R13D,AL` is `44 0f b6 e8`; a scanner that treats
   that `e8` as a CALL and skips five bytes lands mid-instruction and **silently loses the
   read at `0x142784a75`**. So even the direct count was short by one.

Both produce a clean, confident, wrong number - the exact shape `CLAUDE.md` says to distrust.
**`tools/reads.py` now exists so neither can happen again**: it disassembles with capstone,
walks helpers transitively, and carries its own positive control.

Its one weakness, documented in the tool: the `gated?` marker over-reports, because it flags
anything after any conditional jump and string cleanup emits `je` pairs constantly. Confirm a
guard by reading it.

## How it was found

Two independently derived lists, intersected. Neither was built for this.

**List one: the seventeen chat-window printers.** `research/talking-back.md` established
that `FUN_1415eca30(char**, u16 type)` is the string printer and **type 7 is the chat
window**, by way of `FUN_14209ee50` posting string id `0x533` - `[Welcome] Welcome to
MapleStory!!`, the line visible in every capture. A `rel32` sweep of all **1132** call sites
finds **17** that load type 7 within 24 bytes of the call. `notice::CHAT_NOTICE` (`0x00BB`)
came out of that sweep, so the instrument has a confirmed positive: the count reproduced
exactly when re-run for this. **[L]**

**List two: the eighteen balloon creators.** `research/npc-chatter.md` established
`FUN_14158f5c0` as the balloon factory - the first WZ path it loads is `UI/ChatBalloon.img/`
at `0x14158f776`. `tools/callers.py` gives **24 call sites in 18 functions**. **[L]**

**The intersection is two functions**, `0x1427834b0` and `0x142784970`. Only the second
decodes a packet:

| | reads | |
|---|---|---|
| `FUN_142784970` | `u8, str, <speaker object>, u8, u8, u8, <trailing object>` | **one** message string |
| `FUN_1427847a0` | `u8, str, str, <speaker object>, u8, u8, u8` | **two** strings |

`0x0226` is **not the simpler option**: it carries the identical `FUN_1408d6760` speaker
object, at `0x142784856`. Its extra string is the only difference that matters.

Both are reached from one dispatcher, `FUN_1429bafb0`, which normalises the opcode with
`LEA EAX,[RSI-0x226]` / `CMP EAX,0x50` at
`0x1429bb100` before jumping through a table at `0x1429bb5d0`. Reading that table's 81 RVAs
out of the exe:

> **Corrected 2026-08-20.** This said `FUN_1429bafb0` "has **zero direct callers** - it is
> virtual". Both halves were wrong, and the cause was the instrument: `tools/callers.py`
> scanned for `0xE8` only, so a tail `jmp` was invisible to it. It is reached by a **tail
> `jmp` at `0x1429b934b`, in `FUN_1429b9300`**, and it has **no pointer anywhere in the
> image** - so it is not virtual either. The tool now reports calls, tail jumps and data
> pointers as three distinct kinds; `research/instrument-audit-2026-08-20.md` lists
> everything the old blind spot touched.

| index | opcode | case | handler |
|---:|---|---|---|
| 0 | `0x0226` | `0x1429bb124` | `FUN_1427847a0` - two strings |
| **11** | **`0x0231`** | `0x1429bb134` | **`FUN_142784970`** - the one we send |

**[L]** throughout: every step is a table or a call target read out of
`client-patched/MapleStory.exe`, not a name from the reference tree.

## What is not known

* **The four `u8`s** - one before the text and three after. In this game family the leading
  one is an admin/GM marker and one of the trailing ones suppresses the chat-log line so that
  only the balloon shows. That is **[I]**; nothing here read what they do. All go out as `0`,
  which asks for the ordinary case of both balloon and log line.
* **`0x0226`, the two-string form.** A packet carrying a name *and* a message is the shape of
  a whisper, and the dispatcher gives it special setup at its entry (`CMP EDX,0x226 / JNZ`
  at `0x1429bafcb`, guarding two `u32` reads the other opcodes skip). Untested, so unsent.
* **Whether the path to the trailing object is truly unconditional.** It was checked by
  disassembling `0x142784a7a`..`0x142784b05`: the only branches are a null check on a
  `0x30`-byte allocation and a null check on `alloc + 0x10`. Neither can be taken in
  practice, so the object is on the path. **[D]** rather than **[L]**, because "an
  allocation never fails" is an assumption.

  **Sending every field is the safe direction either way.** A client that reads fewer bytes
  than arrive never looks at the rest; one that reads *more* than arrive throws on underrun.
  A field that turns out to be gated off costs a wasted byte; a field left out costs the
  session - and did.

## What the server does now

Anything typed that does not begin with `!` is echoed back as `0x0231` attached to the
speaker's own character id. A `!` line that is not a known command gets a `0x00BB` notice
saying so, rather than vanishing - the same reasoning as the refused `!map`.

**It is a local echo, not a broadcast.** It goes back to the one connection that spoke,
because the server has no concept of a second player in a field yet.

## What to watch on the next run

| what | means |
|---|---|
| a balloon over the head **and** a line in the chat log | `0x0231` is right, and both zero tail bytes are right |
| balloon but **no** log line | one of the two trailing `u8`s is the "balloon only" flag |
| log line but **no** balloon | the balloon has its own guard, like the NPC one at `0x141e3b543` |
| the client dies on the first message | the body is still short - a read past the end throws, and `0xE06D7363` is what that looked like on 2026-08-19 |

---

## Why nothing renders: the gate chain, read end to end (2026-08-19, late)

The run of 2026-08-19 settled that `0x0231` is **accepted** - no crash since the 47-byte fix
- and that `FUN_142784970` reaches **neither** the chat-window print at `1427856fa` **nor**
the balloon at `142785927`. Both were watched; both were zero.

That measurement does **not** say the handler ran and bailed. It is equally consistent with
the handler never being called. This section walks both halves and finds the gates. Every
row is **[L]** unless marked; the listings came from a capstone sweep bounded by the merged
`.pdata` extent (`FUN_142784970` is `0x142784970..0x142785e0c`, one entry, nothing merged),
and the instrument was checked against `FUN_140304100` first, where it reproduced
`tools/reads.py`'s direct reads at the same addresses.

### 1. The dispatcher can drop the packet before the handler exists

`FUN_1429bafb0` does not simply switch. It reads the leading `u32` and **looks the speaker
up in the user pool**, and for `0x0231` a miss is a silent return.

```text
1429bb08d  call 0x1406e8c20        the leading u32 - the characterId
1429bb092  mov  ebx, eax
1429bb094  mov  edx, ebx
1429bb096  mov  rcx, rbp           rbp = this, the user pool
1429bb099  call 0x1429b6c90        CUserPool::GetUser(characterId)
1429bb09e  mov  rbx, rax
1429bb0a1  test rax, rax
1429bb0a4  jne  0x1429bb0bf        found -> go on to the switch
1429bb0a6  cmp  esi, 0x226
1429bb0ac  jne  0x1429bb5b0        NOT 0x226 -> RETURN, having done nothing
1429bb0b2  call 0x14285ff30        0x226 alone gets a fallback
```

**The asymmetry is the point.** `0x0226` has somewhere to go when the speaker is unknown;
`0x0231` does not. A `0x0231` naming a character the client has no user object for produces
no packet error, no exception and no pixel - exactly what the owner saw.

`FUN_1429b6c90` is a lookup by id, not a cast:

```text
1429b6ca9  mov  rcx, [rcx + 0x10]   the LOCAL user
1429b6cb2  call 0x14276df20         its id
1429b6cb7  cmp  eax, ebx            == the requested id?
1429b6cb9  jne  0x1429b6cca         no -> fall through to the hash walk
1429b6cbb  mov  rax, [rdi + 0x10]   yes -> return the local user
...
1429b6cde  mov  rax, rbx / div rcx  bucket = id % [pool+0x100]
1429b6cf0  cmp  [rax + 0x10], ebx   walk the chain, matching the id
1429b6cfe  xor  eax, eax            no match -> NULL
```

And `FUN_14276df20` is six bytes, dumped rather than decompiled:

```text
14276df20  8b 81 d0 10 00 00   MOV EAX, [RCX + 0x10D0]
14276df26  c3                  RET
```

So **the client's local user carries its character id at `user+0x10D0`**, and `0x0231` is
delivered only when the id in our packet equals it. We send `chr.id` - 204 - which is also
what goes into the character stat block at offset 0. Whether `+0x10D0` ends up holding that
value is **not established**; it is written from the character data during `SetField` and
nothing here has read that write.

### 2. If the handler does run, four bails reach neither render site

`FUN_142784970` has exactly one `ret`, at `142785e0b`. Everything that gives up jumps to the
epilogue, and between the last packet read (`142784a75`) and the chat-window print there are
four such jumps:

| at | condition | what bails |
|---|---|---|
| `142784af9` | a `0x30`-byte allocation returned null | not plausible in practice |
| `142784b14` -> `142784b3c` | the global at **`0x143AC2F58`** is null | plausible |
| `142784b69` | `FUN_1415abcc0()` is true **and** `FUN_1415ab8b0()` is non-zero | plausible |
| `142784d84` | `FUN_142d01050(list, id)` is non-zero - the shape of a block/mute list | skipped entirely when the vtable call at `[user+0x50]` returns non-zero |

The two tiny functions in row three, dumped rather than decompiled:

```text
1415abcc0  CMP qword [0x143ACAB80], 0 / SETNE AL / RET      "is that singleton live"
1415ab8b0  MOV RAX,[0x143ACAB70] / TEST / JZ ret / MOV EAX,[RAX+0x3b0] / RET
```

Two adjacent globals, sixteen bytes apart. What they *mean* is **[I] and unread** - the
shape is that of a mode flag ("a cutscene or a modal UI owns the screen"), and that is a
guess, not a finding.

### 3. The two render sites have their own guards, on top of the four

```text
1427856eb  cmp dword [rbp - 0x40], 0
1427856ef  je  0x1427856ff              skip the chat-window post
1427856fa  call 0x1415eca30             <- the chat window, type 7
```

`[rbp-0x40]` is the return of `FUN_1408bef90` at `142784db4`, a formatting call taking a
`0x400`-byte buffer. A zero there costs the log line and nothing else.

```text
1427856ff  lea rcx, [r15 + 0x100]
142785706  call 0x140f8abc0
14278570d  jne  0x142785d86             skip everything below, balloon included
142785927  call 0x14158f5c0             <- the balloon factory
```

So "log line but no balloon" and "balloon but no log line" both have a named mechanism now,
which is more than the guess in the older table below.

### 4. The one-variant test

Two **read-only** watches, one run. Neither changes a byte we send, so this is not two
variants - it is one observation with two probes:

```text
watch@142784970,14276df20:peek=10d0
```

| what happens | what it means |
|---|---|
| `142784970` never fires | the dispatcher dropped it. `GetUser(204)` returned null, and the id we send is not the id the client's local user holds |
| `142784970` fires, and the `peek` dword is **204** | the lookup worked and the handler ran. The bail is one of the four in section 2; the next run watches `142784b41` and `142784b6f` to bisect them |
| `142784970` fires and the `peek` dword is **something else** | that is the answer. Send that id instead, and note what it is - it would mean the client renumbers the local user independently of the stat block |
| neither fires | the packet is not reaching this dispatcher at all, which contradicts `world.log`; re-check that the run actually sent a `0x0231` before believing it |

`peek=<off>` logs the byte and dword at `rcx + off`, and `rcx` at `14276df20` is the local
user - so `peek=10d0` reads the id straight out of the field the accessor returns.
`crates/grap-stub/src/probe.rs` has four watch slots, so both fit with room to spare.

### 5. What writes `user+0x10D0`, and a wrong turn worth recording

A whole-image `tools/fieldrefs.py 0x10d0 --write` sweep - 666 939 resync points, instrument
checked first against its documented control (`0x2f4` in the mob range must return
`141cb7ef3`) - finds **five** writers:

```text
14079ed0a  mov   qword [r14 + 0x10d0], rbp      in 0x14079e660
14089c306  mov   dword [rbx + 0x10d0], r8d      in 0x14089c2d0
141c8168a  movss dword [r14 + 0x10d0], xmm0     in 0x141c813b0
142768f56  mov   dword [rsi + 0x10d0], ebx      in 0x142768ee0
1429c143c  mov   dword [rsi + 0x10d0], eax      in 0x1429c13b0
```

**Only one of those is the user class.** `FUN_142768ee0` is a constructor: it installs four
vtables at `+0`, `+8`, `+0x10` and `+0x100`, then zeroes a contiguous run from `+0x10b0` to
`+0x1102` with `+0x10D0` inside it, and the value it stores there is `ebx = edx`, its **second
argument**. It also sits in the same neighbourhood as the accessor `FUN_14276df20`. So the
client's local user is constructed with its character id, `CUser::CUser(dwId)`, and the id is
decided at creation rather than patched later. It has 7 call sites.

**The wrong turn.** I followed `FUN_1429c13b0` instead, because it is in the same address
range as the user pool (`FUN_1429b6c90`, `FUN_1429bafb0`) and it also takes the value from an
argument. Two hops up, at `141e727d7`, that argument turns out to be
`[rbp-0x6c] + [rcx+0x1c]` - an addition, next to `psrldq`/`movd`/`inc`/`dec` on `xmm6`. That
is coordinate arithmetic, not an id, and `FUN_1429c13b0` is a different class that happens to
use the same offset.

**A displacement match is not a class match.** `fieldrefs.py` finds `[reg + 0x10d0]` whatever
`reg` points at, and five hits across five unrelated classes is exactly what a big binary
should produce. Two more hops and this would have been written up as "the client stores a
computed screen coordinate where the id should be", which is the kind of clean, confident,
wrong answer `CLAUDE.md` is mostly a list of.

**So this is still not settled statically**, and it does not need to be: the watch in section
4 reads the field's actual value out of a running client for the price of a run the owner is
taking anyway. What the sweep did settle is that the id is a constructor argument, so if it
is wrong it is wrong from the moment the user is made - which is a different place to look
than "something overwrote it later", and that is worth knowing before the run rather than
after.
