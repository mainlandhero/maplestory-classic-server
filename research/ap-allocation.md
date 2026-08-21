# Ability points: two requests, six stats, and the latch that ate the other two clicks

Written 2026-08-21, static pass plus two existing captures. **No client run was spent on
this.** Ghidra was held by another agent throughout; everything below came from
`tools/listing.py`, `tools/callers.py`, `tools/dump_va.py` and the already-exported text in
`research/`.

Every claim is tagged **[L]** (read off this client's listing, its `.rdata`, or a capture),
**[D]** (derived from two or more [L]) or **[I]** (inferred, usually from the v214 reference,
which is a different game version and scored 1 of 8 against a held-out control).

Code: **`crates/net/src/abilityup.rs`** (new, 10 unit tests, all passing). **`crates/net/src/lib.rs`
was deliberately not touched** - the coordinator adds `pub mod abilityup;`. **Nothing is
wired.** See [Wire it like this](#wire-it-like-this).

---

## 0. The short version

| question | answer | |
|---|---|---|
| Is `0x0139` the AP request? | Yes - but it is the **bulk** one, and it is not the only one | **[L]** |
| What does a single `+` click send? | **`0x0138`**, `u32 tick, u32 statMask`, **8 bytes**, no amount field | **[L]** |
| Is the request mask the same layout as `0x007C`'s? | **Yes, six for six**, and named by the client's own string ids | **[L]** |
| Which bits exist? | `0x40` STR, `0x80` DEX, `0x100` INT, `0x200` LUK, `0x800` **max**HP, `0x2000` **max**MP. Six, and the switch bails on everything else | **[L]** |
| What is `count`? | a **pair count**. `(end-begin) >> 3` over 8-byte elements, written by a loop, so the body is `8 + 8n` | **[L]** |
| Can it ever be longer than 16 bytes? | Not from this client's UI - the one caller pushes exactly one pair | **[D]**, blind spot named in §7 |
| Why did three clicks produce one packet? | a **one-request-outstanding latch** at `ctx+0x2330`. The client stops sending until the server answers | **[L]** |
| What must the server send? | one `0x007C` carrying the **new stat total** *and* the **new AP**, with byte 0 non-zero | **[D]** |
| What does a refusal look like? | the same `0x007C` with only the AP bit - correcting the client's belief *and* clearing the latch | **[D]** |

---

## 1. There are two requests. `STATUS.md` names one of them

The stat window's button handler is `FUN_141152da2`. It is a flat chain of twelve
name comparisons, six stats x two kinds, and the two kinds go to **two different builders**.
**[L]**:

```text
141152da8  "hpup"      edx=0x800   -> call 0x142d4baa0        ; single
141152dcd  "hpupall"   edx=0x800   -> call 0x141153070        ; bulk
141152ded  "mpup"      edx=0x2000  -> call 0x142d4baa0
141152e15  "mpupall"   edx=0x2000  -> call 0x141153070
141152e35  "strup"     edx=0x40    -> call 0x142d4baa0
141152e5d  "strupall"  edx=0x40    -> call 0x141153070
141152e7d  "dexup"     edx=0x80    -> call 0x142d4baa0
141152ea5  "dexupall"  edx=0x80    -> call 0x141153070
141152ec5  "intup"     edx=0x100   -> call 0x142d4baa0
141152eea  "intupall"  edx=0x100   -> call 0x141153070
141152f07  "lukup"     edx=0x200   -> call 0x142d4baa0
141152f2c  "lukupall"  edx=0x200   -> call 0x141153070
```

The names are UTF-16 at `0x143386050`..`0x143386128`, dumped with `tools/dump_va.py`. The
`...all` arms all `jmp 141152f44`, which is the single `call 0x141153070` at `141152f47`.

`FUN_142d4baa0` builds **`0x0138`**. `FUN_141153070` opens the amount dialog and then calls
`FUN_142d4bbc0`, which builds **`0x0139`**.

> **A server that answers only `0x0139` still looks broken to anyone who clicks `+`.** That
> is the practical consequence of this section and it is the reason it is first.

### 1.1 Direction disambiguation, because `0x0138` is already in the logs

`0x0138` **outbound** is `UserAvatarModified`, which this server already sends - it appears in
five fixtures under that name. Inbound and outbound opcode spaces are separate. The two
constants in `abilityup.rs` are **client -> server only**. **[L]**

---

## 2. `0x0138` - one point. Eight bytes

`FUN_142d4baa0(ctx, statMask)`, whole body. **[L]**

```text
142d4bacc  call 0x142cc42d0        ; latch/alive/cooldown gate, 500 ms  -> 0 means BAIL
142d4badc  call 0x142cbeca0        ; nonzero -> system message 0x50e and BAIL
142d4bb24  call 0x142cbe730        ; -> charstat
142d4bb36  call 0x1401ab420        ; AP = obfuscated u16 at charstat+0x8b, key at +0x8f
142d4bb3b  test ax, ax
142d4bb3e  jle  142d4bb8f          ; AP <= 0 -> SEND NOTHING AT ALL
142d4bb40  mov  edx, 0x138 ; call 0x1406ed520     ; COutPacket(0x138)
142d4bb50  call 0x1429e3ef0 ; w_u32               ; tick
142d4bb61  mov  edx, edi    ; w_u32               ; the stat mask
142d4bb72  call 0x1415d01c0                       ; SendPacket
142d4bb7f  mov  edx, 1 ; call 0x142cc4430          ; SET THE LATCH
```

**Body: `u32 tick, u32 statMask`. Exactly 8 bytes. There is no amount field - the amount is
1 by construction.** **[L]**

The `AP <= 0` bail matters for testing: **once the client believes it has 0 AP, the `+`
button sends nothing and produces no packet to answer.** That is correct behaviour, not a
bug - but only if the server told it the new AP. **[D]**

---

## 3. `0x0139` - N points, as a counted list of pairs

`FUN_142d4bbc0(ctx, vector*)`, the send half. **[L]**

```text
142d4bcb7  mov  edx, 0x139 ; call 0x1406ed520    ; COutPacket(0x139)
142d4bcc7  call 0x1429e3ef0 ; w_u32              ; tick
142d4bcd8  mov  rdx,[rdi+8] ; sub rdx,[rdi]      ; end - begin
142d4bcdf  sar  rdx, 3                           ; / 8  ->  ELEMENT COUNT
142d4bce8  w_u32                                 ; count
>142d4bd00 mov  edx,[rbx]   ; w_u32              ; element +0: statMask
 142d4bd0c mov  edx,[rbx+4] ; w_u32              ; element +4: amount
 142d4bd19 add  rbx, 8 ; cmp rbx,rsi ; jne 142d4bd00
142d4bd27  call 0x1415d01c0                      ; SendPacket
142d4bd34  mov  edx, 1 ; call 0x142cc4430         ; SET THE LATCH
```

**Body: `u32 tick, u32 count, count x (u32 statMask, u32 amount)`. `8 + 8n` bytes.** **[L]**

> **`research/msexe-packet-fields.txt` gets this wrong, and predictably.** Its row is
> `0x0139 FUN_142d4bbc0 FUN_1429e3ef0,u32,u32,u32,u32` - a flat four `u32`s, because that dump
> flattens the loop into one iteration. It happens to match every capture, which is exactly
> the failure `CLAUDE.md` describes for `tools/encodes.py`: *"misses fields written by a loop,
> so a body length from it alone is short and confident."* The listing was read instead.

### 3.1 The two real captures

Both in `previous-runs/`, and they are the only two `0x0139` bodies in the repo. **[L]**

```text
world-20260821-001440.log  04:13:51.922  <- 0x0139  dfbdfe0c 01000000 40000000 1e000000
world-20260820-233248.log  03:32:12.356  <- 0x0139  a7a5d80c 01000000 40000000 2d000000
                                                    ^tick    ^count=1 ^0x40=STR ^30 / ^45
```

30 and 45 are the numbers the owner typed. That is the only field in either body chosen by a
person, which is what makes this a decode rather than a plausible reading of some bytes.
Both are pinned in `abilityup.rs`'s tests.

### 3.2 What builds the vector, and why `count` is always 1 in practice

`FUN_141153070(ctx, statMask)`, the amount dialog. Tail, **[L]**:

```text
1411533b7  AP = getter(charstat+0x8b)          ; twice, into r14d and edx
1411533fd  r8d = 1                             ; the number box's MINIMUM
1411533fa  r9d = AP                            ; the number box's MAXIMUM
141153405  call 0x142a62cc0                    ; configure it
141153417  call 0x14177f640 ; cmp eax,1 / jne  ; run modal; 1 = OK
141153429  call 0x142a643b0 -> eax             ; the number the user typed
14115342e  xorps xmm0,xmm0 / movdqu [rsp+0x78] ; empty vector
14115343b  [rsp+0x58] = r12d                   ; pair.first  = the stat mask
141153440  [rsp+0x5c] = eax                    ; pair.second = the amount
141153450  call 0x141156430                    ; push ONE pair
14115345d  call 0x142d4bbc0                    ; -> 0x0139
```

So the dialog clamps the amount to `1..=AP` **as the client believes AP to be**, and pushes
exactly one pair. `tools/callers.py 0x142d4bbc0` reports **1 call site, 0 tail jmps, 0 pointers
in the image**, so within the readable image this is the only producer of a `0x0139`. **[D]** -
blind spot in §7.

### 3.3 The gate before the send does **not** stop an over-spend

`142d4bc4d..142d4bcb1`, read literally. **[L]**

```text
142d4bc54  begin == end            -> return, send nothing
142d4bc73  ebx = SUM of every element's amount
142d4bc8c  ap = getter(charstat+0x8b)
142d4bc94  test ax,ax  / jg  send  ; AP > 0     -> send
142d4bc98  test ebx,ebx/ jg  send  ; total > 0  -> send
142d4bcb1  cmp ecx,ebx / jl  return             ; only reachable when AP<=0 AND total<=0
```

Both positive tests jump **to** the send, so the final `AP < total` check is only reachable
when AP and the total are both non-positive. **The client does not refuse to ask for more
points than it has.** The *dialog* clamps; the *builder* does not re-check. Whatever a
hand-built or stale-state request contains, **the server is the only thing that can say no.**
**[L]** for the instructions, **[D]** for "therefore the server must enforce".

---

## 4. The stat mask table, and the hypothesis it was meant to test

`FUN_141153070`'s head switches on the mask and loads one string id per arm. **[L]**

```text
1411530de  cmp r12d, 0x200 / ja 141153125 / je 14115311c
1411530e9  eax = r12d
1411530ec  sub eax, 0x40  / je  141153113
1411530f1  sub eax, 0x40  / je  14115310a
1411530f6  cmp eax, 0x80  / jne 1411534cc   <- bail
141153125  cmp r12d, 0x800  / je  141153144
14115312e  cmp r12d, 0x2000 / jne 1411534cc <- bail
```

The `lea rcx,[rip+d]` in each arm resolves into `.rdata`, plain ASCII (`tools/dump_va.py`):

| mask | string at | string id | `0x007C` field |
|---:|---|---|---|
| `0x0040` | `0x143386210` | `SID_MSCW_STAT_STR` | `charstat+0x3b`, **u16** |
| `0x0080` | `0x143386228` | `SID_MSCW_STAT_DEX` | `charstat+0x43`, **u16** |
| `0x0100` | `0x143386240` | `SID_MSCW_STAT_INT` | `charstat+0x4b`, **u16** |
| `0x0200` | `0x143386258` | `SID_MSCW_STAT_LUK` | `charstat+0x53`, **u16** |
| `0x0800` | `0x1433861e0` | `SID_MSCW_STAT_HP` | `charstat+0x67`, **u32** - **MAX** hp |
| `0x2000` | `0x1433861f8` | `SID_MSCW_STAT_MP` | `charstat+0x7f`, **u32** - **MAX** mp |

The dialog's prompt string is `SID_MSCW_ASK_STATUP_AP_AMOUNT` at `0x143386270`, which is what
identifies `FUN_141153070` as the amount dialog rather than by its shape.

### 4.1 The hypothesis holds, six for six

The `0x007C` mask is decoded by `FUN_1402cbb50`; `research/msexe-statdecode-1402cbb50.txt`
enumerates every conditional bit test in address order. **[L]**

```text
1402cbc1b  test bpl, 0x40   -> READ u16 -> lea rdx,[rbx+0x3b]    STR
1402cbc38  test bpl, bpl/jns-> READ u16 -> lea rdx,[rbx+0x43]    DEX     (bit 7)
1402cbc54  bt   ebp, 8      -> READ u16 -> lea rdx,[rbx+0x4b]    INT
1402cbc71  bt   ebp, 9      -> READ u16 -> lea rdx,[rbx+0x53]    LUK
1402cbc8e  bt   ebp, 0xa    -> READ u32 ->        [rbx+0x5b]     HP      (current)
1402cbcc4  bt   ebp, 0xb    -> READ u32 ->        [rbx+0x67]     MAX HP
1402cbcfa  bt   ebp, 0xc    -> READ u32 ->        [rbx+0x73]     MP      (current)
1402cbd30  bt   ebp, 0xd    -> READ u32 ->        [rbx+0x7f]     MAX MP
1402cbd6c  bt   ebp, 0xe    -> READ u16 -> lea rdx,[rbx+0x8b]    AP
```

**The AP request's mask is the same bit layout as `0x007C`'s.** STR is `0x40` in both.
All six accepted values line up. **[L]**

Three things fall out, and the third is the one that would have cost a run:

1. **`crates/net/src/stats.rs`'s names move from [I] to [L] for these six.** That file says the
   names of `STR`, `DEX`, `INT`, `LUK`, `MAX_HP`, `MAX_MP` "come from the v214 reference … and
   are **[I]**". This client's own `SID_MSCW_STAT_*` strings name them. The remaining names in
   `bits` (`SKIN`, `FACE`, `HAIR`, `LEVEL`, `JOB`, `HP`, `MP`, `AP`, `SP`, `EXP`, `FAME`,
   `MESO`) are untouched by this pass and stay as they were.
2. **HP and MP do have bits here - but they are the MAX bits.** The button says HP; the mask
   is `0x800` = `MAX_HP` -> `charstat+0x67`. **Current** HP is `0x400` and the request cannot
   name it at all. This is the same shape as the three unit bugs in `CLAUDE.md` § "The unit,
   not the arithmetic", one level up: the name on the button is not the name of the field.
3. **The widths differ inside the same reply.** STR/DEX/INT/LUK are `u16`; maxHP/maxMP are
   `u32`. `0x007C` has no length prefix and no resync point, so a `u32` written where the
   client reads a `u16` pushes every later value two bytes late. `ApStat::apply_to` exists so
   this cannot be done by hand.
4. **The client assigns, it does not add.** Every arm calls `FUN_1402f7010(value, ptr)` with
   the value straight off the wire. So the reply carries the character's **new total**, never
   the points just spent. **[L]**

---

## 5. Why three clicks produced one packet: the latch

Both builders open with `FUN_142cc42d0(ctx, 500, 0)` and both end with
`FUN_142cc4430(ctx, 1)`. **[L]**

```text
FUN_142cc4430(ctx, v):
  142cc4439  [ctx+0x2330] = v
  142cc4444  [ctx+0x2334] = now

FUN_142cc42d0(ctx, minMillis, flag) -> 1 = allowed:
  142cc42da  [ctx+0x2338] != 0            -> 0
  142cc42e8  [ctx+0x2330] != 0            -> 0        <- THE LATCH
  142cc42f1  flag == 0 and charstat+0x5b (HP) <= 0 -> 0
  142cc4317  now - [ctx+0x2334] < minMillis -> 0      <- 500 ms
```

So after one AP request the client sets `[ctx+0x2330] = 1` and **will not build another one
until a server packet clears it.** The second and third clicks construct nothing, send
nothing, and put nothing on screen - which is precisely what the owner reported.

### 5.1 What clears it

The **first byte of `0x007C`**, `research/msexe-statchanged-142d54780.txt`. **[L]**

```text
142d547ad  call 0x1406e8ae0        ; u8 bExclRequestSent
142d547b2  test al, al
142d547b4  je   142d547c0          ; zero -> the latch is left alone
142d547b6  xor  edx, edx
142d547bb  call 0x142cc4430        ; [ctx+0x2330] = 0
```

`crates/net/src/stats.rs` already defaults `excl_request_sent` to `true`, and its doc comment
already describes this call. **One `0x007C` both applies the stat and unblocks the next
click.** **[L]**

There is also **`0x0142`**, whose arm is at `0x142cbd140`. It is this client's analogue of
v214's `EXCL_REQUEST`, and the detail matters because it is easy to read backwards. **[L]**

```text
142cbd143  call 0x1406e8ae0 ; u8
142cbd148  cmp  al, 1
142cbd14a  jne  142cbd17c   -------+   ; not 1 -> straight to the merge point
142cbd14c  ...string 0x106, UI...  |   ; 1 -> also show a message, then fall through
>142cbd17c  xor edx, edx      <----+   ; THE MERGE POINT
142cbd17e  mov rcx, rdi
142cbd181  call 0x142cc4430             ; [ctx+0x2330] = 0, on BOTH paths
```

The `jne` is not a guard on the clear - `142cbd17c` is the join, so **the latch is cleared
whatever the byte is**; the byte only selects whether a message is shown as well. (I nearly
retracted this section on a first reading that stopped at the `jne`.) So `0x0142` with body
`00` is a valid latch-clear that touches no stat. **It is not needed**, because the reply has
to carry stats anyway; it is written down so nobody hunts for it later, and because it is the
fallback probe in §9.

### 5.1a Four other opcodes clear it, and this server sends none of them

`tools/fieldrefs.py 0x2330` over the whole image (130 hits, 666 939 resync points) finds four
**inlined** clears inside the channel dispatcher `FUN_142cbaa80` itself, each
`xor ebx,ebx / mov [rdi+0x2330], ebx / [rdi+0x2334] = now`. Resolving the dispatcher's jump
table at `0x142cbd9d0` maps every one to an **exact** table entry - no nearest-match. **[L]**

| arm | opcode | do we send it? |
|---|---|---|
| `0x142cbae80` | **`0x00B7`** | no |
| `0x142cbbd90` | **`0x00F8`** | no |
| `0x142cbbe82` | **`0x00F9`** | no |
| `0x142cbcf32` | **`0x018F`** | no |

A grep of `crates/net/src` and `crates/world/src` finds none of the four anywhere except as
coincidental bytes inside `codec.rs`'s cipher tables. **So nothing this server currently sends
clears the latch by accident** - which is what makes the §9 test discriminating. **[D]**

### 5.1b **Field entry clears it too**, and that changes how to run the test

`FUN_142caa4e0` - the field-entry routine, the same function `crates/net/src/stats.rs` already
names for its write of `1` to `[ctx+0x3bbc]` - writes the latch **twice**, at `142caa7e4` and
`142caaca1`, both `mov [rdi+0x2330], r15d` with `r15d` zeroed at `142caa527` and not
reassigned in between. The `[rdi+0x3bbc] = 1` at `142caacf7` is the same `rdi`, which is what
confirms this is the CWvsContext and not some other class that happens to have a `+0x2330`.
**[L]**

> **So changing map clears the latch.** A player who clicks `+`, gets no answer, then walks
> through a portal can click again and it *will* send. That is a real behaviour, and it is
> also a way to invalidate the §9 test by accident - which is why §9 now says not to move
> between the two clicks.

### 5.2 The corroborating measurement, and what it does *not* prove

Both runs that contain a `0x0139` contain **exactly one**, and both sessions continued for
another 35 s and 49 s afterwards with ordinary mob traffic. `STATUS.md` records three attempts
across those two runs, so at least one attempt produced no packet. **[D]**

What that measurement does **not** establish on its own is that the latch is the reason -
The owner could have clicked once in one run and twice in the other and the second one could have
failed for some other cause. The mechanism above is [L] from the listing; the count is [L]
from the captures; the join is **[D]**.

Both captures were then checked properly, and the answer is **different in the two runs**.
**[L]**

| | `world-20260820-233248` | `world-20260821-001440` |
|---|---|---|
| last `SetField` before the request | 03:30:14, ~2 min earlier | 04:11:11, ~2m40 earlier |
| so field entry cleared it in between? | **no** | **no** |
| `0x007C` sent **after** the request | **none at all** | **one**, at 04:14:03 |
| what that `0x007C` was | - | **idle regen**, `+1 hp -> 146/146` |
| latch state for the rest of the run | **stayed set** for 35 s | **cleared 11 s after the request** |

So run 2 is a clean confirmation - nothing cleared the latch and nothing more was sent. **Run
1 is not**: an idle-regen `0x007C` reopened the window 11 s later, and the session then ran
another 37 s with no second `0x0139`, which means the owner simply did not click again after that
point. **The latch therefore explains *some* of the missing clicks, not provably all three.**
That is a weaker claim than §5 first made, and it is the honest one.

> **The consequence is much bigger than the retraction.** `crates/world/src/session/regen.rs`
> sends a `0x007C` every 10 s of standing still, with `excl_request_sent` defaulting to
> `true`. **Idle regeneration is already clearing this latch by accident.** That is benign in
> normal play - the UI un-wedges itself - but it is a **confound that can fake a passing
> test**, and §9 is written around it.

---

## 6. What the server must send

### 6.1 The confirm

**One `0x007C`, byte 0 non-zero, carrying the new stat total and the new AP.**

* **The new stat total**, because the client never changes its own record. Neither builder
  writes anything to `charstat`; both only *read* AP. So until a `0x007C` arrives the window
  shows the old STR. **[L]** (no writes in either listing) + **[D]**.
* **The new AP**, for two reasons that bite differently:
  * the `0x0138` path bails when it believes AP is 0, and the `0x0139` dialog clamps its
    maximum to what it believes AP is. A stale AP lets the player keep spending points that
    are gone. **[L]**
  * `STATUS.md` and `stats.rs` both already treat `0x007C` bit 14 as the only channel for AP.
* **Byte 0 non-zero**, or the latch stays set and the *next* click is dead. **[L]**

`FUN_142cbefd0` at `142d54a2b` is called unconditionally after the mask block, whatever bits
were set, so the stat window is refreshed by any `0x007C`. The `test edi, 0x40030` block at
`142d549d6` only decides the value of one flag argument, not whether the refresh happens.
**[L]** - so no extra "redraw" packet is needed. Whether the specific labels repaint was not
read; that is what the client test in §9 is for.

### 6.2 The refusal - and "Always answer" is absolute here

`CLAUDE.md`: *an unanswered packet freezes the client's entire UI and reads on screen as a
crash.* This one is worse than the general case, because the latch means the failure is
**silent and permanent for the session** rather than obvious.

**The refusal chosen here is a `0x007C` carrying the character's *current* AP and nothing
else** - 11 bytes. It does three things at once:

1. clears the latch, so the UI is alive again;
2. tells the client the truth about how many points it has, which is the thing that let it ask
   for too many in the first place;
3. changes no stat, so a bad request cannot move a number.

The minimum that satisfies "always answer" is the **empty** `0x007C`,
`StatChange::new().build()` = `[1, 0, 1, 0, 0, 0, 0, 0, 0]`, 9 bytes - the shape
`stats.rs`'s `an_empty_change_still_frames` test already pins. That is the fallback for a body
that does not even parse, where there is no stat to be truthful about.

**No dedicated "stat up failed" packet was found, and that is a searched-for negative rather
than an assumed one**: `research/msexe-gamestage-opcodes.md`'s table of the whole
`0x70..0x19f` range has no entry whose body is a failure code near `0x0138`/`0x0139`, and
v214 - which is only a candidate - also has none: its handler answers a bad request with
`chatPopup(...)` plus `dispose()`, and `dispose()` is `exclRequest()`, an empty latch-clear.
**[I]** for the reference, **[L]** for the table.

### 6.3 The policy numbers, which are [I]

Not in this client. Kept in one named table, `abilityup::policy`, per `CLAUDE.md`'s rule for
exactly this case:

| | value | source |
|---|---:|---|
| max HP per AP | 20 | v214 `UserStatHandler` uses `20`. **[I]** |
| max MP per AP | 20 | same. **[I]** |
| STR/DEX/INT/LUK per AP | 1 | the request's own `amount`. **[L]** |

Whether spending AP on HP should also raise **current** HP by the same amount is **not
established** and is a separate decision - the request cannot name current HP, so if the
server wants the bar to move it must set bit `0x400` itself.

---

## 7. What I did NOT establish

* **A second producer of `0x0139` inside `.themida`.** `tools/callers.py` scans the image, and
  `.themida`'s `SizeOfRawData` is **0** - there are no file bytes to scan, which is the same
  reason `research/` says the pick-up opcode's builder can never be confirmed. So "the body is
  always 16 bytes" is **[D] with a named blind spot**: a caller in the packed section could
  push more than one pair. `parse_ability_mass_up` therefore parses the general `8 + 8n` form
  rather than requiring 16.
* **A *complete* list of what clears `ctx+0x2330`.** Largely answered - §5.1a and §5.1b - and
  the remaining gap is now specific rather than vague. What was done: `tools/fieldrefs.py
  0x2330` over the whole image (**130 hits, 666 939 resync points**), then the dispatcher's
  jump table at `0x142cbd9d0` resolved to name the four inline arms **exactly**, with the
  table calibrated against a known control first (§8).
  What that still cannot see, and both matter:
  * `tools/fieldrefs.py` and `tools/rangescan.py` **silently drop `rbp`-based operands**
    (`STATUS.md`'s instrument table, "Not fixed - work around it and say you did"), so a
    clearer written `[rbp+0x2330]` is invisible to the sweep entirely;
  * a clear made *through the setter* rather than by a direct store shows up only as one of
    `FUN_142cc4430`'s **241 call sites in 204 functions**, which were not individually checked
    for `edx = 0`.

  So the clearers named are [L] and real; **"these are the only ones" is not claimed.**
* **What `FUN_142cbeca0` is.** It is a virtual call `[vtbl+0x38]` feeding `FUN_14085ab00`, and
  a non-zero result makes both builders show system message `0x50e` and send nothing. Probably
  a "you are in a trade/mini-room" style block. Not decoded.
* **Whether the stat window repaints the STR and AP labels** on a `0x007C` that carries no
  level/job/exp/meso bit. The refresh call is unconditional; what it repaints was not read.
  §9 is the one test that separates this from everything else.
* **Whether `0x0138` was ever sent in a real session.** No capture in this repo contains one,
  in either run. The 8-byte shape is [L] from the builder; the test that exercises it uses a
  synthesised body and says so.
* **What the tick is for.** Written by `FUN_1429e3ef0` in every builder in the image. Not
  echoed anywhere by any reply this project sends.

---

## 8. How this was measured, and the controls

| instrument | control run | result |
|---|---|---|
| `tools/listing.py` | its own documented control, `listing.py 0x140304100 \| grep READ` | printed `140304138 raw`, `140304144 u8`, `140304183 u8`, then the `u16` run - exactly as its docstring requires |
| `tools/callers.py` | asked about the **known** `0x013B` skill-up builder `0x142d4bd80` first | 1 call site, in `0x142586d7f` - inside `0x14258xxxx`, the skill-window module `research/skills.md` identified independently. Then asked about `0x142d4bbc0` |
| `tools/dump_va.py` | n/a - it returned printable ASCII string ids at every address, which is its own control | |
| `research/msexe-packet-fields.txt` | found `0x0139` -> `FUN_142d4bbc0` | and was then **contradicted** by the listing on the loop; see §3 |
| the dispatcher jump table at `0x142cbd9d0` | **calibrated before use**: `stats.rs` documents opcode `0x7C` -> index 12 -> stub `0x142cbab4f`. Read as image-base-relative RVAs that is exactly what index 12 gives; read as table-base-relative it gives `0x14597851f`, which is not even in `.text` | so §5.1a's four opcodes are exact table entries, not nearest matches |
| `tools/switch_cases.py` | **misused, and the empty result was discarded rather than reported.** It takes a decompiled `.c` path and a function name; it was handed a VA and returned nothing. An empty result was the only possible outcome, so it is not evidence of anything - `CLAUDE.md` § "Verify the instrument before believing it" | replaced by the jump-table read above |

Everything was run with the repo as the working directory, per `CLAUDE.md` § "The scratchpad
shadows the real tools".

**The cargo result is from an isolated copy, not the shared `target/`.** Four other agents
were running. `crates/net/src` was copied to the session scratchpad, `pub mod abilityup;` was
added **to the copy**, and `cargo test --target-dir ./target` was run there: **324 unit tests
and 10 doctests, 0 failed**, of which 10 are this module's. The repo's own `lib.rs` and
`target/` were not touched, so this result cannot have raced another agent's build - and
equally, it does **not** prove the module compiles inside the workspace. The coordinator's
run after adding the `mod` line is the one that counts.

---

## Wire it like this

`crates/net/src/lib.rs` needs one line, and **this agent did not add it**:

```rust
pub mod abilityup;
```

Then in `crates/world/src/session/` - the coordinator's file, not this agent's - two arms.
The single-point one is not optional; it is what a normal `+` click sends.

```rust
use net::abilityup::{self, ApStat};
use net::stats::{StatChange, STAT_CHANGED};

// dispatch
abilityup::CLIENT_ABILITY_UP      => self.ability_up(body, abilityup::parse_ability_up),
abilityup::CLIENT_ABILITY_MASS_UP => self.ability_up(body, abilityup::parse_ability_mass_up),

fn ability_up(&mut self, body: &[u8], parse: fn(&[u8]) -> Option<abilityup::AbilityUpRequest>)
    -> Vec<Reply>
{
    let chr = /* the live character */;

    // EVERY path below ends in a 0x007C. An unanswered request leaves ctx+0x2330 set and
    // the stat window is dead for the rest of the session - see research/ap-allocation.md 5.
    let refuse = |ap: u16, why: &str| Reply {
        opcode: STAT_CHANGED,
        body: StatChange { ap: Some(ap), ..Default::default() }.build(),
        what: format!("StatChanged: AP request refused ({why}); re-sending ap={ap}. \
                       Byte 0 clears the ctx+0x2330 latch - without it the client never \
                       sends another AP request this session"),
    };

    let Some(req) = parse(body) else {
        return vec![refuse(chr.ap, "body did not parse")];
    };
    if !req.is_well_formed() {
        return vec![refuse(chr.ap, "unknown stat mask or zero amount")];
    }
    if req.total_points() > u32::from(chr.ap) {
        return vec![refuse(chr.ap, "not enough AP")];   // the client does NOT check this
    }

    let mut change = StatChange::default();             // excl_request_sent is already true
    for e in &req.entries {
        let stat = e.stat().expect("is_well_formed");
        let n = e.amount;
        let new_total = match stat {
            ApStat::Str   => { chr.str   += n as u16; u32::from(chr.str) }
            ApStat::Dex   => { chr.dex   += n as u16; u32::from(chr.dex) }
            ApStat::Int   => { chr.int_  += n as u16; u32::from(chr.int_) }
            ApStat::Luk   => { chr.luk   += n as u16; u32::from(chr.luk) }
            ApStat::MaxHp => { chr.max_hp += n * abilityup::policy::MAX_HP_PER_AP; chr.max_hp }
            ApStat::MaxMp => { chr.max_mp += n * abilityup::policy::MAX_MP_PER_AP; chr.max_mp }
        };
        stat.apply_to(&mut change, new_total);          // right field, right WIDTH
        chr.ap -= n as u16;
    }
    change.ap = Some(chr.ap);                           // bit 14. Not optional - see 6.1
    self.store.save_character_progress(chr).ok();

    vec![Reply { opcode: STAT_CHANGED, body: change.build(), what: /* ... */ }]
}
```

**The exact `0x007C` fields to set**, and nothing else:

| field | value | why |
|---|---|---|
| `excl_request_sent` | `true` (the default) | clears `ctx+0x2330`. Without it the next click is dead |
| `secondary`, `context_flag` | `0`, `1` (the defaults) | unchanged from every other `0x007C` this server sends |
| `strength`/`dexterity`/`intelligence`/`luck` | the **new total**, `u16` | via `ApStat::apply_to`, which picks the width |
| `max_hp`/`max_mp` | the **new total**, `u32` | mask `0x800`/`0x2000` are the **MAX** fields |
| `ap` | the **new remaining AP** | bit 14. The client's `+` bails at 0 and the dialog clamps to this |
| everything else | `None` | in particular **not** `hp`/`mp` - `0x400`/`0x1000` are current HP/MP and the request cannot name them |

`crates/store` already persists `ap`, and `save_character_progress` already writes the four
stats, so nothing new is needed there. Three notes for whoever wires it:

* `chr.str/dex/int/luk` are `u16` in the record - the four stat fields on the wire are `u16`
  too, so no cast beyond `u16::from`.
* the character's **current** HP/MP are *not* raised by this handler. If the bar should move
  when AP goes into HP, set `change.hp` as well - deliberately left out because the request
  cannot ask for it and raising it is a policy decision.
* the 500 ms cooldown in `FUN_142cc42d0` starts at the *reply*, so two very fast clicks after
  a confirm can still be dropped client-side. That is the client's rate limiter, not a bug.

---

## 9. The one client test that separates the hypotheses

Everything above is static. One run settles it, and **the second click is the whole test** -
not the first.

> Enter the world with at least 5 AP (level up once, or `!exp` to a level), open the stat
> window, and click the **`+` beside STR** twice, **within about three seconds of each
> other**. Nothing else - do not walk through a portal, and do not stand still for ten
> seconds between the clicks.

Two clicks, because one click cannot distinguish "the reply works" from "the reply is
irrelevant and the client did it itself", and because the *second* click is the one the latch
would have eaten.

**The two timing rules are not fussiness; each one can fake a pass.** [L], §5.1b and §5.2:

* **a map change clears the latch** (`FUN_142caa4e0` writes `0` to it twice), so a portal walk
  between the clicks makes the second one send whatever our reply did;
* **idle regeneration clears the latch**, because `regen.rs` sends a `0x007C` every 10 s of
  standing still and `excl_request_sent` defaults to `true`. This is not hypothetical - it is
  exactly what happened in `world-20260821-001440.log`, 11 s after the owner's `0x0139`.

  Ten seconds of standing still therefore reopens the window on its own, and a second click
  after that would succeed **even if our AP reply were completely broken**. Three seconds is
  inside the regen interval; the 500 ms client cooldown is the only other constraint, so
  anything from ~1 s to ~8 s apart is fine.

**The free cross-check, whatever happens:** `grep 0x007C world.log` and look at what sits
between the two `<- 0x0138` lines. If the only `0x007C` there is ours, the test is clean. If a
regen `0x007C` slipped in, the run does not discriminate and the second click proves nothing -
re-run it faster rather than believing it.

| what the owner sees | what it means | next step |
|---|---|---|
| STR goes up by 1 and AP down by 1 on the **first** click, **and again on the second** | The whole chain works: `0x0138` is the single-click opcode, the mask table is right, and byte 0 of `0x007C` really does clear the latch. | done; `-> STATUS.md` |
| the **first** click updates, the **second** does nothing | The reply reached the client but did **not** clear the latch. Check `world.log`: exactly one `<- 0x0138`. Byte 0 of our `0x007C` is being sent as `0`, or something re-sets `ctx+0x2330`. §5.1 is the place to look, and `0x0142` with any one-byte body is the fallback probe - its clear is at a merge point and runs whatever the byte is. | one-line fix, re-test |
| **nothing** happens on either click, and `world.log` shows **no** `<- 0x0138` at all | The client believes AP is **0** - `142d4bb3e` bails before building anything. The AP in the `SetField` record is wrong, not the handler. | check the stat window's AP number before clicking; it is on screen |
| **nothing** happens, but `world.log` **does** show `<- 0x0138` answered with a `0x007C` | The reply is being sent and ignored. The mask/width mapping is wrong, or the stat window does not repaint on a mask without `0x40030` bits. **Reopen the stat window** - if the new STR is there, it is a repaint problem, not a decode problem, and that distinction is free. | §6.1's unread question |
| the client freezes | An unanswered packet, per `CLAUDE.md`. The last inbound line in `world.log` with nothing after it names it. | |

**Do not test the bulk button in the same run.** It is a different opcode with a different
body, it goes through a modal dialog, and `CLAUDE.md` § "test one variant at a time" exists
because changing two things at once already produced one unexplained crash here. Once the `+`
works, the bulk button is the second run - and its expected result is the same `0x007C` with a
larger jump.

**Nothing authenticates.** The game socket carries no credentials, and none of this changes
that.
