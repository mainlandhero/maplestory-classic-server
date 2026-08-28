# Removing the client's own `1` over the player's head

The owner, 2026-08-28: *"I do see two numbers now, the 1 and the actual damage number. I really need
this 1 to go away to make this portion perfect."*

**Yes, it can be removed, and there are three independent ways to do it.** The one that costs
nothing but a packet is a **server-sent client console command**; the one with no dependency on
anything is a **five-byte patch** the hook can already make.

The route that would have made the stub *correct* instead of gone — `forcedStatPresent` — is
enumerated in §5 and it is **dead**: ten of its twelve `u32` fields are read by accessor
functions that **nothing in the image calls**, and each of those accessors' mob-template
fallbacks has exactly one caller — its own dead accessor.

Tags: **[L]** read off a listing, a table dump or a capture; **[D]** derived from two or more
[L]; **[I]** inferred.

---

## 0. The answers, up front

| question | answer |
|---|---|
| **What draws the `1`?** | `FUN_1428aa0a0`'s single `call 0x142771360` at `0x1428aca14`. There is exactly one in that 14 497-byte function. **[L]** |
| **What gates it?** | `GetOption(0xAE, 0) == 0` **and** `user+0x544a != 0`. Either failing skips 0x356 bytes, the renderer call included. **[L]** |
| **Why is `user+0x544a` non-zero, when the earlier pass found one writer that no one ever calls?** | Because there is a **second** write it could not see: `142883687 mov qword [rsi+0x5448], 0x10001`, a *wider store at a lower displacement*. §2. **[L]** |
| **Can the server clear it?** | **Yes.** Inbound `0x00EA` runs its string through the client's own slash-command dispatcher, and the command **`/hitdamagetest 0`** writes `user+0x544a = 0`. §3, §4. **[L]** for the mechanism, **[D]** for the permission gate. |
| **Can the server make the stub correct instead?** | **No.** §5, and the negative is measured, not argued. |
| **Cheapest client-side patch?** | `0x1428aca14`: `E8 47 49 EC FF` → `90 90 90 90 90`. §6. **[L]** |
| **Does any of this break our own `0x02D1` violet number?** | No. The `0x02D1` queue drain `FUN_14281e390` is one of the **fifteen** renderer callers with no `0x544a` gate. **[L]** |

---

## 1. Instruments, and the control each one passed

Every number below came from a tool made to speak first, run with the repo as the working
directory (`CLAUDE.md` § *The scratchpad shadows the real tools*). The one script I wrote was
piped in on stdin (`python - <<'PY'`), which leaves `sys.path[0]` empty, with
`sys.path.insert(0, os.path.abspath("tools"))` naming the repo's copy explicitly. I verified
`tools/reads.py` is the 9 605-byte version carrying both `0x142d23ef0` and `0x1406e8fb0`.

| instrument | positive control | result |
|---|---|---|
| `tools/listing.py` | `0x140304100` must show `raw@140304138`, `u8@140304144`, `u8@140304183`, then a `u16` run | reproduced exactly |
| `tools/callers.py` | `0x1402fa9a0` must give **96 sites in 15 functions, 43 of them in `0x140304b20`**, 0 tail jmps, 0 data pointers | reproduced exactly |
| `tools/reads.py` | `0x140304100 2` must list the same direct reads plus two helper subtrees | reproduced |
| `tools/rangescan.py` | disp `0x544a` in `0x142000000..0x143000000` must give **11 sites** with `1428ac8b8` among them (`damage-number-draw.md` §5's own control) | reproduced, 11 sites |
| my covering scan (§2) | must re-find all 11 of those as exact-disp hits before any new hit is believed | 11 of 11, plus one more outside the old range |
| **zero-caller results in §5** | **`0x141c8a730`**, a sibling stub in the *same* 48-byte-aligned family, in the *same* 0x300-byte region, reached the same way | **178 call sites in 51 functions** |

That last row is the control that matters most. `CLAUDE.md` § *A positive control has to be close
to the subject*: "callers.py works" was already known and useless. "callers.py finds 178 sites for
the accessor sitting 0xC0 bytes before the ones I am calling dead" is the control that localises
the failure, and it costs one extra invocation.

**Ghidra was not used** (another agent holds the project lock). Everything here is capstone over
`client-patched\MapleStory.exe` through the repo's own tools, plus `research/fixtures/`.

---

## 2. Why `user+0x544a` is non-zero — the write the earlier pass could not see

`research/damage-number-draw.md` §5 reported, carefully and with three named blind spots:

> **Hedged negative, with its blind spot named:** I found no route from a server packet to
> `user+0x544a`. What that search could see: `[reg + 0x544a]` memory operands anywhere in
> `0x142000000..0x143000000`, and packet-read primitives within two call levels of the single
> writer. What it could **not** see: (a) a write through a pointer that was `lea`'d and handed to
> a helper — the exact `mob+0x42c` failure `CLAUDE.md` records, and `rangescan` is structurally
> blind to it; (b) the flag being *copied* from some other field that a packet does populate,
> which a depth-2 read walk from the writer would not reach; (c) anything outside
> `0x142000000..0x143000000`. So: **not found is not not-there.**

I am quoting that in full because `CLAUDE.md` requires the blind spot to be written down before a
hedged negative is acted on. All three were real. **The actual answer was a fourth one nobody had
written down**, and it was sitting where the first three would never look.

### 2.1 The contradiction that had to be resolved first

The earlier pass's own facts do not fit together:

* it found **exactly one write** to `user+0x544a`, in `FUN_1420571b0`, a function with one caller
  and no packet reads in its subtree;
* the gate at `1428ac8b8` requires the byte to be **non-zero to draw**;
* and the owner can see the `1` on screen.

`CLAUDE.md` § *"Not found" is not "not there"*: **when a report's negative contradicts something
already seen on screen, the screen wins.** So a second write had to exist. The question was not
"is there one" but "what shape is it, such that a disp-exact scan cannot see it".

### 2.2 Changing the question, not the range

`rangescan.py` matches `o.mem.disp == DISP` with `o.mem.index == 0`. Three things are invisible to
that and only one of them was on the earlier list:

* a **wider store at a lower displacement** — `mov qword [r+0x5448], imm` writes bytes
  `0x5448..0x544f`, and `disp == 0x544a` never matches;
* an **indexed** form `[base + index*scale + disp]`;
* a store through a pointer computed elsewhere.

So I scanned every `[reg + disp]` operand in the whole `.text`
(`0x140001000..0x14326194a`, from the PE section table) and asked a different question of each
one: **does the operand's byte range cover offset `0x544a`, given its size?**

```
== exact disp 0x544a : 12 ==
  141fd159e  cmp byte ptr [r14 + 0x544a], al      FUN_141fd1260     <- outside the old range
  14205729b  mov byte ptr [rax + 0x544a], bl      FUN_1420571b0     <- the known writer
  14288b41e  cmp byte ptr [r15 + 0x544a], al      FUN_14288ac30
  14288cc51  cmp byte ptr [rsi + 0x544a], r12b    FUN_14288cb40
  14288dac2  cmp byte ptr [r12 + 0x544a], al      FUN_14288d6d0
  14288dc4a  cmp byte ptr [r12 + 0x544a], al      FUN_14288d6d0
  14288df4b  cmp byte ptr [r14 + 0x544a], r15b    FUN_14288de60
  142893aa9  cmp byte ptr [r13 + 0x544a], r15b    FUN_1428923e0
  142893e45  cmp byte ptr [r13 + 0x544a], r15b    FUN_1428923e0
  14289420b  cmp byte ptr [r13 + 0x544a], r15b    FUN_1428923e0
  1428ac8b8  cmp byte ptr [r15 + 0x544a], r14b    FUN_1428aa0a0
  142908505  cmp byte ptr [rsi + 0x544a], al      FUN_142908220

== WIDE operand covering byte 0x544a : 11 ==
  142883687  mov qword ptr [rsi + 0x5448], 0x10001   size=8   FUN_142882250   <<<<<
  (the other ten are all [rsp + 0x5448] stack slots in unrelated functions)

== indexed [base+index*s+disp] covering 0x544a : 0 ==
```

**[L] `142883687 mov qword ptr [rsi + 0x5448], 0x10001`.** Bytes on disk at `0x142883687`:

```
48 c7 86 48 54 00 00   01 00 01 00
REX.W C7 /0  disp32=0x5448   imm32=0x00010001
```

The 8-byte store sign-extends that imm32, so it lays down

```
offset  5448 5449 544a 544b 544c 544d 544e 544f
value     01   00   01   00   00   00   00   00
                    ^^ user+0x544a = 1
```

Closing blind spot (c) at the same time cost nothing and found a **twelfth** compare site,
`141fd159e` in `FUN_141fd1260`, below `0x142000000`. It is another `cmp`, so it does not change
the conclusion — but it does mean the earlier "11 sites" was 11 of 12.

### 2.3 That `rsi` is the same object the gate reads

`FUN_142882250` (`0x142882250..0x142885253`, 12 291 bytes; **1 call site**, `FUN_1429b5780` at
`0x1429b5959`; 0 tail jmps, 0 data pointers) also writes, off the same `rsi`:

* `1428825c9  mov qword ptr [rsi + 0x48b8], r12`
* `142883fe5  movups xmmword ptr [rsi + 0x40e8], xmm0`

and `FUN_1428aa0a0` reads both off `r15`, the register the `0x544a` gate uses:

* `1428ac7d2  mov ecx, dword ptr [r15 + 0x40e8]`
* `1428ac853  mov dword ptr [r15 + 0x48b8], esi`

**[D]** Same object type, from three matching offsets rather than one. That is the identity check;
it is not proof that this particular call constructs the *local* user, which I did not establish.

### 2.4 What the flag therefore is

**[D]** `user+0x544a` is a per-user "draw my own damage number" flag, **initialised to 1** by
`FUN_142882250` and settable to 0 or 1 by exactly one other thing — the `/hitdamagetest` console
command, §3. `user+0x5448`, initialised to 1 in the same store, is its neighbour; I did not chase
what reads it.

---

## 3. The gate, and what clearing it actually removes

### 3.1 The gate, in full

```
1428ac8a4  xor  edx, edx
1428ac8a6  mov  ecx, 0xae
1428ac8ab  call 0x14090d160          ; GetOption(key, default) - see below
1428ac8b0  test eax, eax
1428ac8b2  jne  1428acc0e            ; option 0xAE non-zero -> skip
1428ac8b8  cmp  byte [r15 + 0x544a], r14b     ; r14b = 0
1428ac8bf  je   1428acc0e            ; the byte must be NON-ZERO to draw
...
1428ac9ef  test r12d, r12d
1428ac9f2  js   1428ad551            ; negative damage -> skip (never taken; see §3.3)
1428aca14  call 0x142771360          ; THE draw
```

**[L] `0x1428aca14` is the only `call 0x142771360` anywhere in `FUN_1428aa0a0`** — grepped over the
full 3 585-line listing in `research/msexe-1428aa0a0.txt`, and it agrees with `callers.py
0x142771360`, which reports exactly one site for this function.

**`0x14090d160` is `int GetOption(int key, int defaultValue)`** — bytes at `0x14090d160` decode to a
red-black-tree lookup over the `std::map<int,int>` whose root pointer is the global `0x143AC33E0`
(`_Left/_Parent/_Right` at 0..0x10, `_Isnil` at 0x19, key at 0x1c, value at 0x20), returning
`[node+0x20]` on a hit and **`edx` on a miss**. The call site passes `edx = 0`. **[L]** Five sibling
accessors at `0x14090d1d0/240/2c0/340` read the same global (`tools/dataref.py 0x143AC33E0`: 7
references, all reads, five of them these accessors and two in `FUN_140918760`). The getter has
**268 call sites in 224 functions**, so it is the client's general option table.

### 3.2 Clearing the flag removes a number in **one** of the eight builders, and it is ours

Eleven of the twelve `0x544a` sites are `cmp`s in eight functions. All eight also call
`0x142771360`. The naive reading — "the flag turns off damage numbers everywhere" — is **wrong**,
and the discriminator is the size of the block each `je` skips against where that function's
renderer call sits:

| function | gate | `je` target | skipped | its renderer call(s) | call inside the skip? |
|---|---|---|---|---|---|
| `FUN_141fd1260` | `141fd159e` | `141fd15f6` | 0x58 | `141fd1884` | no |
| `FUN_14288ac30` | `14288b41e` | `14288b483` | 0x65 | `14288c73e` | no |
| `FUN_14288cb40` | `14288cc51` | `14288cca9` | 0x58 | `14288d05c` | no |
| `FUN_14288d6d0` | `14288dac2` | `14288db14` | 0x52 | `14288dd01` | no |
| `FUN_14288d6d0` | `14288dc4a` | `14288dc8e` | 0x44 | `14288dd01` | no |
| `FUN_14288de60` | `14288df4b` | `14288dfab` | 0x60 | `14288e327` | no |
| `FUN_1428923e0` | `142893aa9` | `142893af2` | 0x49 | `142893b11` | no |
| `FUN_1428923e0` | `142893e45` | `142893e90` | 0x4b | `142893eb5` | no |
| `FUN_1428923e0` | `14289420b` | `142894256` | 0x4b | `14289427b` | no |
| **`FUN_1428aa0a0`** | **`1428ac8b8`** | **`1428acc0e`** | **0x356** | **`1428aca14`** | **YES** |
| `FUN_142908220` | `142908505` | `142908566` | 0x61 | `1429086c3` | no |

**[L]** `FUN_1428923e0`'s three renderer sites were enumerated by disassembling
`0x1428923e0..0x142894400` and matching `call 0x142771360`, not by reading `callers.py`'s
`first`/`last` columns — `recovery-number.md` §5's warning box and `CLAUDE.md`'s "do not read
first/last as the site list".

So in ten of the eleven gates the flag chooses **between two ways of computing** a number that is
drawn either way. **Only in `FUN_1428aa0a0` does clearing it remove the draw**, and
`FUN_1428aa0a0` is the builder `damage-number-draw.md` §2.3/§6 established produced all 224
captured `0x00E5` bodies — the one that is drawing the owner's `1`.

That is the best possible shape for this answer: the lever is precise rather than broad.

### 3.3 What is actually drawn, which is not what §5 of the earlier file assumed

`damage-number-draw.md` §2.3 says `1428aca14` is reached with `edx = ebx - r12d = -max(eax,0)`.
That is right, but `eax` is not a mana-shield absorb — it is the whole magnitude:

```
1428ac8f7  call 0x140266ea0        ; (nType=0, nDamage=r12d, &damageSkinOut) -> eax
1428ac915  call 0x1428eb360        ; (pUser, 0, eax) -> eax
1428ac91a  mov  ebx, r12d
1428ac91d  sub  ebx, eax           ; ebx = damage - eax
1428ac91f  cmp  r12d, ebx
1428ac922  cmovl ebx, r12d         ; ebx = min(damage, damage - eax)
...
1428ac9fd  mov  edx, ebx
1428ac9ff  sub  edx, r12d          ; edx = -max(eax, 0)   <- the number that draws
```

`FUN_1428eb360` (196 bytes) switches on `movsx ebx, dl`, matching only 1..4 ('I','F','L','S' event
codes); with `dl = 0` it falls to `1428eb412 mov eax, edi` and **returns its third argument
unchanged**. **[L]** So the drawn magnitude is exactly `FUN_140266ea0(0, damage, &out)`, and
`FUN_140266ea0` opens with `test edx,edx / jle 1402677e5 / mov eax, r12d / ret` — it returns
`nDamage` verbatim when the damage is `<= 0` or the local user (`[0x143AA8518]`) is null. **[L]**

This matters for one reason only: `FUN_140266ea0` has **11 call sites in 8 functions, and they are
exactly the 8 functions holding the `0x544a` compares**. The flag and that function are one
subsystem. **[L]**

---

## 4. The server route: opcode `0x00EA` runs client console commands

This is the direction the brief asked for — *"a flag with one writer still has a caller, and that
caller has callers"* — and it goes further than expected.

### 4.1 The writer is a slash-command handler, and the command is `/hitdamagetest`

`FUN_1420571b0` is one of **133 sibling functions** (all in `0x14204xxxx..0x14205xxxx`) that each
call `0x14083ef30` once at their head with a small byte vector — a permission mask. Its one caller
is `FUN_14202e460` at `0x14203834d`, a **124 196-byte slash-command dispatcher**.

The name string for that entry:

```
14203810a  lea rdx, [rip + 0x13ee197]     ; -> 0x1434262A8
142038111  lea rcx, [rbp + 0x278]
142038118  call 0x140196ed0               ; ZXString::Assign(name, -1)
...
14203830f  lea rcx, [rbp + 0x278]
142038316  call 0x14206f160               ; normalise
14203831b  lea rdx, [rbp + 0x278]
142038322  lea rcx, [rsp + 0x38]          ; the token the user typed
142038327  call 0x14019a570               ; compare
14203832c  test al, al
14203832e  je   14203829d                 ; no match -> next command
142038334  lea rcx, [rsp + 0x30]
142038339  call 0x140726850               ; argument count
14203833e  cmp eax, 1
142038341  jb  14203837d                  ; needs at least one argument
142038343  lea rdx, [rsp + 0x30]          ; the argument vector
142038348  lea rcx, [rsp + 0x48]          ; the permission context
14203834d  call 0x1420571b0
```

`0x1434262A8` in `.rdata` is **`/hitdamagetest`**, and the description used two instructions later
(`142038249 lea rax,[rip+0x13ee068]` → `0x1434262B8`) is **"Test hit damage"**. The neighbouring
pool entries are `/jobexcltestmsg`, then `/hitdamagetest` + "Test hit damage", then
`/hitdamagetestmsg` + "Hit damage test message", then `/getquest`. **[L]**

### 4.2 The argument is a real argument

`0x140726850` is `mov rax,[rcx]; test rax,rax; jne +1; ret; mov eax,[rax-8]; ret` — the element
count. **[L]** And the dispatcher removes the command token from the vector before any handler
runs:

```
14202e712  call 0x14206eda0                    ; release element 0
14202e717  mov  rax, [rsp + 0x30]
14202e71c  mov  rcx, [rax - 8]
14202e720  shl  rcx, 3
14202e724  sub  rcx, rdi
14202e727  lea  r8,  [rax - 8]
14202e72b  add  r8,  rcx
14202e72e  and  r8,  0xfffffffffffffff8
14202e732  lea  rdx, [rdi + 8]
14202e736  mov  rcx, rdi
14202e739  call 0x142ef7ba0                    ; memmove(&v[0], &v[1], (n-1)*8)
14202e73e  mov  rax, [rsp + 0x30]
14202e743  dec  qword ptr [rax - 8]            ; n -= 1
```

**[L]** So `element[0]` inside a handler is the **first parameter**. This is corroborated by the
argument-count checks across the dispatcher: `cmp eax, 1`, `cmp eax, 3`, `cmp eax, 6` — counts of
arguments, not of tokens.

`FUN_1420571b0` then does:

```
142057275  cmp dword [rax - 8], 1
142057279  jb  1420572e2            ; need >= 1 argument
14205727b  mov rcx, [rax]           ; argv[0]
142057283  call 0x142f11a94         ; atoi
142057288  mov edi, eax
14205728a  test edi, edi
14205728c  setne bl
14205728f  mov rax, [rip + 0x1a51282]   ; -> 0x143AA8518, the local CUser
142057299  je  1420572a1
14205729b  mov byte [rax + 0x544a], bl
1420572a1  ... COutPacket(0x0189); Encode4(0x13D); Encode1(bl); SendPacket; ~COutPacket
```

**`/hitdamagetest 0` writes `user+0x544a = 0`. `/hitdamagetest 1` writes it back to 1.** **[L]**
(Encode identities from `tools/encodes.py`'s table: `0x1406ED520` ctor, `0x1406ED9D0` u32,
`0x1406ED840` u8, `0x1415D01C0` SendPacket, `0x1406ED610` dtor.)

The client will therefore send us an **outbound `0x0189` carrying `u32 0x13D` and `u8 flag`.** It is
a plain build-and-send with no latch and nothing waiting on a reply, so it needs no answer — but
`CLAUDE.md`'s *Always answer* rule is about requests the UI blocks on, and I did not verify by
measurement that this one does not. Log it and move on.

### 4.3 The packet that types it for us

`FUN_142d9fa90` — the game-stage handler for **`0x00EA`**, listed in
`research/msexe-gamestage-opcodes.md:363` and `research/msexe-gamestage-cases.txt:112`, dispatched
from `FUN_142cbaa80` at `0x142cbbcc9` with no gate on the case:

```
142d9faa4  call 0x1406e9050        ; READ str   <- the whole payload
142d9faaa  call 0x141892840        ; the chat UI object; null -> bail
142d9faca  lea  rdx, [rip+0x4f5fa7]   ; -> 0x143295A78 = "\r"
142d9fad6  call 0x1408e4350        ; split the string on "\r" into a vector
   for each token:
142d9fb30    lea rdx, [rip+0x6f60c9]  ; -> 0x143495C00 = "> %s"
142d9fb47    call 0x1415eca30         ; echo it to the chat log, colour 11
142d9fb61    lea rdx, [rsp + 0x68]    ; the RAW token
142d9fb66    mov rcx, rdi
142d9fb69    call 0x1418cd030         ; -> 0x1418cd082 call 0x14202e460, the dispatcher
```

**[L]** So a server-sent `0x00EA` string is split on carriage returns, each line is echoed into the
chat window as `> <line>`, **and each line is run through the slash-command dispatcher**.

`research/msexe-gamestage-opcodes.md` names `0x00EA` "ScriptProgressMessage". That row is a claim,
and this is what the code does — `CLAUDE.md` § *"A table row written from a quick read is a claim"*
again. The echo is the message; the dispatch is not in the name.

### 4.4 The one thing that is not settled: the permission gate

`0x14083ef30(ctx, vector)` walks the vector two bytes at a time as `{mask, flags}` and returns 1 iff
`((flags & 2) || (ctx[6] && (flags & 1)))` **and** one of
`ctx[4]&(mask&0x10)`, `ctx[0]&(mask&1)`, `ctx[1]&(mask&2)`, `ctx[2]&(mask&4)`, `ctx[3]&(mask&8)`.
`FUN_1420571b0` passes `{0x07, 0x0D}`, so it needs

> **`ctx[6]` non-zero, and any one of `ctx[0]`, `ctx[1]`, `ctx[2]`.** **[L]**

`ctx` is built at `14202e60e` by `FUN_14083ee10(&ctx, [0x143AA84A0] /* the CWvsContext singleton,
documented across `research/channel-select.md`, `cash-shop.md` and the hook's own session watch */,
[0x143AC18A0])`:

| ctx byte | source |
|---|---|
| `ctx[0]` | `CWvsContext + 0x2294 != 0` |
| `ctx[1]` | bit 0 of `FUN_142d2e710(CWvsContext + 0x2210)`, **or** `+0x2294 != 0` |
| `ctx[2]` | `+0x2298 != 0` or `+0x229c != 0` or `+0x2294 != 0` |
| `ctx[3]` | `+0x229c != 0` |
| `ctx[4]` | `*(u8*)(0x143AC18A0 + 0x144)` |
| `ctx[5]` | `+0x22a4 != 0` |
| `ctx[6]` | `+0x226c != 0` |

**[L]** All from `0x14083ee10`'s listing and the six one-instruction getters at `0x142cb9290`,
`0x142cb84b0`, `0x142cb8660`, `0x142cb8510`, `0x142cb8540`, `0x142cb8560`.

Those fields come off the wire:

* `+0x2294`, `+0x2298`, `+0x229c`, `+0x2210` are stored by `FUN_142cb7e60` (`142cb804f`,
  `142cb8230`, `142cb823f`, `142cb7f0e`), whose **only two callers** are `FUN_141b2dd00` and
  `FUN_141b2ee90` — both in the login-stage band, and both with direct packet reads at their head
  (`u8` then `str`, `tools/reads.py <fn> 2`). **[D]**
* `+0x226c` is stored by `FUN_142cb9210`, whose **one caller** is `FUN_142097f80` at `0x142098082`
  — and `FUN_142097f80` is the **`SetField` handler for inbound `0x01A0`**, named in
  `research/msexe-stage-setfield.md`, `charrecord-reuse.md` and `instrument-audit-2026-08-20.md`.
  **[L]**

**So the capability bits the command needs are set from packets we already send.** What I did
**not** do is decode *which field of which packet* carries each bit, so I cannot hand over a diff.
That is the one open piece of §4 and it is a bounded job: `FUN_142097f80` around `0x142098065`
(`SetField`) for `+0x226c`, and `FUN_141b2dd00`/`FUN_141b2ee90` for `+0x2294`.

### 4.5 What a `0x00EA` costs if the permission is not there

Nothing fatal, as far as anything here shows. The token is echoed to chat either way; if no command
matches, `FUN_1418cd030` continues past `0x1418cd08a` and may send the string as ordinary chat.
`0x00EA` was already put on the wire once, with 32 bytes of junk, in
`research/fixtures/sweep-0024-01c3-reply-0171.log:242`, and the sweep carried on through `0x00EB`,
`0x00EC`… — **but that was the login stage, not the game stage**, so it is not evidence about this
handler. Say so rather than leaning on it.

Two unresolved gates on that path: `FUN_142067170(&str)` at `14202e522` exits the dispatcher early
if it returns true, and I do not know what it tests; and I did not establish what
`FUN_14202e460` returns on a successful command, which is what decides whether the line is also
sent as chat.

---

## 5. `forcedStatPresent`: all twelve `u32`s enumerated, and the route is dead

`damage-number-draw.md` §4/§6.3 named this as the one untried lever. It is enumerable, and the
answer is no.

### 5.1 Where the block lives

`FUN_14085acd0` (`research/msexe-mobforcedstat.txt`) has **0 call sites, 1 tail `jmp`** — from
`FUN_141cc9410` at `0x141cc9496` — and 0 data pointers. That is exactly the `tools/callers.py`
blind spot `CLAUDE.md` records, and the tool now reports it as its own kind. `FUN_141cc9410`
allocates the object and stores it at **`mob + 0xA20`** (`141cc945f mov qword [rbx+0xa20], rax`).
**[L]**

### 5.2 The twelve fields, in wire order

The parser's decode order is the wire order; the struct offsets are not monotonic, so the two
must be listed separately.

| # | wire type | struct offset | reader | template fallback | template offset |
|---:|---|---|---|---|---|
| 0 | `u64` (`0x1406e8f10`) | `+0x08` | `0x141c8a730` | `0x14047a100` | `tpl+0x60` / `tpl+0x20` (max HP) |
| 1 | `u32` | `+0x10` | `0x141c8a7f0` | `0x14047a170` | `tpl+0x28` |
| 2 | `u32` | `+0x14` | `0x141c8a820` | `0x14047a180` | `tpl+0x2c` |
| 3 | `u32` | `+0x18` | `0x141c8a850` | `0x14047a190` | `tpl+0x34` |
| 4 | `u32` | `+0x1c` | `0x141c8a880` | `0x14047a1a0` | `tpl+0x30` |
| 5 | `u32` | `+0x20` | `0x141c8a8b0` | `0x14047a1b0` | `tpl+0x40` |
| 6 | `u32` | `+0x24` | `0x141c8a8e0` | `0x14047a1c0` | `tpl+0x44` |
| 7 | `u32` | `+0x28` | `0x141c8a910` | `0x14047a1d0` | `tpl+0x4c` |
| 8 | `u32` | `+0x2c` | `0x141c8a940` | `0x14047a1e0` | `tpl+0x50` |
| 9 | `u32` | `+0x34` | `0x141c8a9a0` | `0x14047a200` | `tpl+0x54` |
| 10 | `u32` | `+0x38` | `0x141c8a9d0` (**used only when > 0**) | `0x14047a210` | `tpl+0x58` |
| 11 | `u32` | `+0x30` | `0x141c8a970` | `0x14047a1f0` | `tpl+0x48` |
| 12 | `u32` | `+0x48` | **none found** | — | — |
| — | `u8` | `+0x04` | `0x141c8aa00` (returns 1 when the byte is **0**) | — | — |

**[L]** for every row: the parser listing gives the wire order and the struct offsets; each reader
is a 4-instruction stub of the shape `mov rax,[rcx+0xA20]; test rax,rax; je fallback; mov
eax,[rax+OFF]; ret`, dumped from `0x141c8a730..0x141c8aa40`; each template fallback is
`mov eax,[rcx+OFF]; ret`, dumped from `0x14047a100..0x14047a230`.

Note the ninth, tenth and eleventh `u32`s land at `+0x34`, `+0x38`, `+0x30` — the wire order is
**not** struct order, which a decoder that assumes it would get silently wrong. Note also that the
first field is a `u64`, not a `str`: `0x1406e8f10` is the `u64` reader in `tools/reads.py`'s own
table, and the store is `mov qword ptr [RDI + 0x8], RAX`.

**Which one is attack power I could not establish.** The mob template's WZ field names are not in
`research/`, and I did not open the template loader. What I can say is that it does not matter —
see below.

### 5.3 Ten of the twelve readers have no caller of any kind

`tools/callers.py`, which reports `call`, tail `jmp` and data pointers as three separate facts:

```
0x141c8a730  (+0x08, max HP)  178 call sites in 51 functions
0x141c8a970  (+0x30)           23 call sites in 23 functions
0x141c8a7f0  (+0x10)   0 calls, 0 tail jmps, 0 data pointers
0x141c8a820  (+0x14)   0 / 0 / 0
0x141c8a850  (+0x18)   0 / 0 / 0
0x141c8a880  (+0x1c)   0 / 0 / 0
0x141c8a8b0  (+0x20)   0 / 0 / 0
0x141c8a8e0  (+0x24)   0 / 0 / 0
0x141c8a910  (+0x28)   0 / 0 / 0
0x141c8a940  (+0x2c)   0 / 0 / 0
0x141c8a9a0  (+0x34)   0 / 0 / 0
0x141c8a9d0  (+0x38)   0 / 0 / 0
0x141c8aa00  (+0x04)   0 / 0 / 0
```

And the corresponding **mob-template** getters, one level down:

```
0x14047a170 (tpl+0x28) : 1 caller    0x14047a1a0 (tpl+0x30) : 1 caller
0x14047a180 (tpl+0x2c) : 0 callers   0x14047a1b0 (tpl+0x40) : 1 caller
0x14047a190 (tpl+0x34) : 1 caller    0x14047a1c0 (tpl+0x44) : 1 caller
0x14047a1d0 (tpl+0x4c) : 1 caller    0x14047a1e0 (tpl+0x50) : 1 caller
0x14047a200 (tpl+0x54) : 1 caller    0x14047a210 (tpl+0x58) : 2 callers
0x14047a1f0 (tpl+0x48) : 9 call sites in 8 functions
```

Every "1 caller" is **its own dead accessor**. **[D] The client reads none of those ten mob stats,
from the override or from the template.** The only two live ones are max HP (`+0x08`, which the HP
bar uses — `mob-hp-bar.md`) and `+0x30`/`tpl+0x48`.

### 5.4 The blind spot in that negative, named, and closed as far as I could

`callers.py` cannot see an **inlined** copy of a 4-instruction accessor. That is a real risk here
precisely because these stubs are tiny.

Closing it: I scanned the whole `.text` for 8-byte `[reg + 0xA20]` operands — the load that any
inlined copy must begin with. **176 sites in 90 functions**, and none of them is in
`FUN_1428aa0a0`, `FUN_140266ea0`, `FUN_140265f00`, `FUN_1402661b0`, or any function in the
`0x1402xxxxx` damage-calculation module. Corroborating from the other end: `FUN_140265f00` is 673
bytes and its complete call list is `0x1401ba9d0` ×6, `0x14025e540` ×2, `0x140268140`,
`0x140879b10`, `0x140909d50`, `0x140909d80` — no mob accessor, no template accessor. Its damage
seed is `mov eax, dword ptr [r14]` where `r14` is the caller's eighth argument, `&[rbp+0x440]`, a
buffer the *user* side fills.

> **What that scan still cannot see, written down so it can be quoted:** a two-step address
> (`lea rX,[mob+0xA00]` then `[rX+0x20]`); the `mob+0xA20` pointer loaded once into a register or
> local far from its uses and carried; and a read through a `[base+index*scale+disp]` form. Any of
> those would make this negative wrong, and the way to test it is not to re-run this scan —
> `CLAUDE.md`: **changing the question is the second opinion** — but to watch `mob+0xA20` in a live
> process, or to send a non-zero `forcedStatPresent` once and see whether anything moves.

### 5.5 Therefore

**[D] Making the client's stub *correct* via `forcedStatPresent` cannot work**, for a stronger
reason than the 224-body table gave: it is not that the client's number ignores the mob's WZ
`PADamage`, it is that the client contains **no live reader for the mob's attack power at all**,
neither from the override block nor from the template. `damage-number-draw.md` §6.3's counter-
argument — "the client can already read this mob's WZ stats, `mob+0xb60 = hp*100/maxHP` proves the
template is loaded" — is correct and is answered: max HP is one of the **two** template fields with
a live reader.

---

## 6. If a client-side patch is preferred, here are the exact bytes

The hook already writes into this mapped, Themida-packed image at run time —
`crates/grap-stub/src/heapfix.rs` patches `0x14019B504` in the same `.text`, reading the bytes
first, refusing and logging if they differ, `VirtualProtect`, write, read back, log either way.
Use that pattern verbatim. All three sites below were dumped from
`client-patched\MapleStory.exe` today.

**(c) is the recommendation**: it removes exactly one thing.

### (a) The initial value — one byte, kills the stub for every user object

```
VA  0x142883690      RVA 0x2883690
verify at 0x142883687:  48 C7 86 48 54 00 00 01 00 01 00
write     at 0x142883690:  01 -> 00        (imm32 0x00010001 -> 0x00000001)
```

Leaves `user+0x5448 = 1` untouched. **Caveat:** the store executes once, inside `FUN_142882250`;
the patch has to be in before that runs. The hook arms ~4.5 s after connect
(`maplecw-hook-arming-race` memory) and the user object is built at field entry, so it ought to be
comfortable — but it is a timing dependency the other two do not have.

### (b) The gate — six bytes, evaluated per hit, timing-independent

```
VA  0x1428ac8bf      RVA 0x28AC8BF
verify:  0F 84 49 03 00 00      (je 0x1428acc0e)
write:   E9 4A 03 00 00 90      (jmp 0x1428acc0e ; nop)
```

Skips the same 0x356 bytes the flag skips — including the damage-skin lookup at `1428ac8f7` and
the `FUN_1428eb360` call — so it is a bigger change than it looks.

### (c) The renderer call — five bytes, the most surgical **← recommended**

```
VA  0x1428aca14      RVA 0x28ACA14
verify:  E8 47 49 EC FF         (call 0x142771360)
write:   90 90 90 90 90
```

`0x1428aca19 + 0xFFEC4947 = 0x142771360`, so the encoding is confirmed against the target. `eax` is
dead immediately after (`1428aca19 mov rcx, qword ptr [rbp - 0x18]`), and on x64 the caller's
shadow space is allocated in the prologue, so removing the call needs no stack fix-up. Everything
else in the block — the skin, the HP handling, the object built at `[rbp+0x1e8]` — runs unchanged.
**[L]/[D]**

---

## 7. How to spend the one client run

**One variant.** `/hitdamagetest 0` delivered by `0x00EA`, on top of the `0x02D1` effect `0x41`
that already draws the violet number. Payload: the opcode, then a `str` whose text is
`/hitdamagetest 0`. Send it a second or two after field entry, once the chat window exists
(`FUN_142d9fa90` bails at `142d9fab5` if `FUN_141892840()` is null).

Every outcome is a claim that can come back false:

| on screen | reading |
|---|---|
| chat shows `> /hitdamagetest 0`, and the next mob hit draws **only** the violet number | done. §3, §4 confirmed end to end |
| chat shows `> /hitdamagetest 0`, and the `1` is **still there** | the command was found but refused — the §4.4 permission gate. `world.log` will have **no** inbound `0x0189`; that absence is the discriminator, and it is the specific opcode, not "nothing new arrived" |
| chat shows `> /hitdamagetest 0` **and** the line again as ordinary chat, `1` still there | the dispatcher did not match the name. Then the command string is wrong, not the mechanism |
| **no** `> ` line in chat at all | `0x00EA` never reached the handler. Count `0x00EA` in `world.log` and `called`/dispatch lines in `client-patched\maplecw-hook.log` and compare — the hook writes its line on **return** |
| the client freezes or dies | `0x00EA` is server→client and expects no reply, but it does run arbitrary client code. Fall back to §6(c), which touches five bytes and no packet |

The **inbound `0x0189` with body `3D 01 00 00 00`** is the positive control for the whole chain: it
is emitted by `FUN_1420571b0` *after* the write to `user+0x544a`, on the same straight line. If it
arrives, the flag was written. Grep for that opcode specifically —
`CLAUDE.md` § *"nothing new arrived" is a different claim from "this thing did not arrive"*.

---

## 8. What I could **not** establish

* **Which field of which packet sets `CWvsContext + 0x226c`, `+0x2294`, `+0x2298`, `+0x229c`.**
  §4.4 shows the three setter functions are on packet paths — `+0x226c` from `SetField`/`0x01A0`
  (`FUN_142097f80` → `FUN_142cb9210` at `0x142098082`), the other three from two login-stage
  handlers — but I did not decode the offsets. Until that is done, **whether `/hitdamagetest` will
  pass its permission check on the owner's session is [I], not [D].**
* **What `FUN_142067170` tests** at `14202e522`, which can exit the command dispatcher before any
  command is matched; and **what `FUN_14202e460` returns** on success, which decides whether the
  line is also sent as ordinary chat.
* **Which of the twelve forced-stat `u32`s is attack power**, and what the mob template's
  `+0x28..+0x58` fields are called in WZ. §5.2 gives every offset and every reader; the names are
  missing. It does not change §5's conclusion, because ten of the twelve have no live reader at all.
* **Whether `forcedStat +0x48` (the twelfth `u32`) has any reader.** I found none in the accessor
  family; I did not scan for one elsewhere.
* **Whether `FUN_142882250` constructs the *local* user specifically**, or every `CUser`. Three
  matching field offsets (§2.3) establish the type, not the instance.
* **What `user+0x5448` is** — written to 1 by the same store, never looked at.
* **Where the option map at `0x143AC33E0` is populated**, and therefore whether key `0xAE` is a
  second, independent lever. `FUN_140918760` reads the global twice (`1409187f3`, `14091886f`) and
  is the obvious place to start. If `0xAE` can be made non-zero, it suppresses the same draw
  through the same `jne` one instruction earlier — and it would do so without touching a per-user
  flag.
* **Whether inbound `0x00EA` is safe in the game stage.** The one archived send
  (`research/fixtures/sweep-0024-01c3-reply-0171.log:242`) was in the **login** stage with junk
  padding, so it says nothing about this handler. `CLAUDE.md` § *The thing you are comparing
  against may never have been a control*.
* **Whether the client waits on anything after emitting `0x0189`.** The builder is a plain
  ctor/encode/send/dtor with no latch, but I did not measure it.

---

## 9. Reproducing every number here

Repo root as the working directory, always.

```
python tools/listing.py  0x140304100 | grep READ        # control
python tools/callers.py  0x1402fa9a0                    # control: 96 sites, 43 in 0x140304b20
python tools/reads.py    0x140304100 2                  # control

python tools/rangescan.py 0x544a 0x142000000 0x143000000  # control 1428ac8b8; 11 sites
python tools/rangescan.py 0x544a 0x140001000 0x142000000  # the 12th: 141fd159e
python tools/dump_va.py  0x142883680 32   # 48 C7 86 48 54 00 00 01 00 01 00
python tools/dump_va.py  0x1428ac8a4 40   # the gate, both jumps
python tools/dump_va.py  0x1428aca0a 24   # E8 47 49 EC FF at 1428aca14

python tools/callers.py  0x142771360      # 30 sites / 23 fns; only 8 carry a 0x544a gate
python tools/callers.py  0x140266ea0      # 11 sites in the SAME 8 functions
python tools/callers.py  0x1420571b0      # ONE site: 0x14203834d in FUN_14202e460
python tools/callers.py  0x142882250      # ONE site: 0x1429b5959
python tools/callers.py  0x14085acd0      # 0 calls, 1 TAIL JMP at 0x141cc9496
python tools/callers.py  0x141c8a730      # 178 sites - the near control for the zero results
python tools/callers.py  0x141c8a7f0      # 0 / 0 / 0   (and 0x141c8a820 .. 0x141c8a9d0)
python tools/callers.py  0x14047a170      # 1 caller: its own dead accessor
python tools/callers.py  0x14090d160      # 268 sites / 224 fns - the option getter
python tools/dataref.py  0x143AC33E0      # 7 refs, all reads; 5 sibling accessors
python tools/callers.py  0x142d9fa90      # ONE site: 0x142cbbcc9 inside FUN_142cbaa80
python tools/callers.py  0x142cb7e60      # 141b2dd00, 141b2ee90 (both read packets)
python tools/callers.py  0x142cb9210      # ONE site: 0x142098082 in SetField/0x01A0

python tools/dump_va.py  0x1434262A8 48   # "/hitdamagetest" / "Test hit damage"
python tools/dump_va.py  0x143495C00 48   # "> %s"
python tools/dump_va.py  0x14090d160 130  # GetOption(key, default)
python tools/dump_va.py  0x140726850 48   # the argument-count helper
```

The two whole-image scans (the `0x544a` covering scan in §2.2 and the `[reg+0xA20]` scan in §5.4)
are one-off variants of `tools/rangescan.py`, run over `0x140001000..0x14326194a` — the `.text`
extent read from the PE section table, not guessed. The `0x544a` one differs from `rangescan.py`
in exactly one predicate: instead of `o.mem.disp == DISP`, it asks
`disp <= 0x544a < disp + o.size`, and reports indexed forms separately. Its control is that it must
re-find all of `rangescan.py`'s exact-disp hits before any wide hit is believed; it did.
