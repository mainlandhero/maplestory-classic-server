# CH.2 is drawn and cannot be clicked — what feeds the predicate, and where the value comes from

Written 2026-08-20, after the run in which the Change Channel dialog showed **two rows for
the first time**. Companion to `research/channel-select.md`; it does not replace it, and it
corrects one thing in it (§4 below).

**No Ghidra** — another session held the project lock. Everything here is `tools/listing.py`,
`tools/callers.py`, `tools/rangescan.py`, `tools/fieldrefs.py`, a raw capstone range dump for
the three leaves that have no `.pdata` entry, plus `login.log` from the run itself. Every
instrument's positive control is in §8.

Labels: **[L]** read off the listing, a capture or the WZ. **[D]** derived from two or more
[L]. **[I]** inferred, including anything from the v214 reference.

Verbatim listings are in the three dumps written beside this file:

| | |
|---|---|
| `research/msexe-channel-enable-predicate.txt` | `FUN_142cb9510`, `FUN_142cb9490`, the OK handler, the mouse hit test |
| `research/msexe-channel-list-transfer.txt` | the mode fork, the **live** decoder's channel loop, `FUN_141b2c7c0`, `FUN_142cb8e10` |
| `research/msexe-channel-draw-and-send.txt` | the Draw four-way decision, OnCreate, the age gate, the `0x00D2` builder |

---

## 1. The answer in one line

**`singleton+0x2cc8` is a `dword` array with one entry per channel, it is filled only from
the 4th trailing `u8` of that channel's `WORLD_LIST` entry, and we send `0`.** Draw, the
mouse hit test and the Change button all read the *same* entry through the *same* function
and all three treat `0` as "not usable". **[D]**

There is nothing else feeding it. The value is not computed, not derived from the user
count, and not defaulted anywhere: it is the wire byte, zero-extended. **[L]**

---

## 2. What actually feeds `FUN_142cb9510`

### 2.1 The array's shape — read, not assumed

`FUN_142cb9510` has **no `.pdata` entry**, so `listing.py` refuses it; this is a raw range
dump at that VA. **[L]**

```
000142cb9510  85 d2                    test edx, edx
000142cb9512  78 18                    js   0x142cb952c        ; idx < 0        -> 0
000142cb9514  48 8b 89 c8 2c 00 00     mov  rcx, [rcx+0x2cc8]  ; the array
000142cb951b  48 85 c9                 test rcx, rcx
000142cb951e  74 0c                    je   0x142cb952c        ; null           -> 0
000142cb9520  3b 51 f8                 cmp  edx, [rcx-8]       ; length at [-8]
000142cb9523  73 07                    jae  0x142cb952c        ; out of bounds  -> 0
000142cb9525  48 63 c2                 movsxd rax, edx
000142cb9528  8b 04 81                 mov  eax, [rcx+rax*4]   ; return arr[idx]
000142cb952b  c3                       ret
000142cb952c  33 c0                    xor  eax, eax
000142cb952e  c3                       ret
```

So, all **[L]**, and stated explicitly because `CLAUDE.md` records this project getting the
shape of a mask wrong in both directions:

* **It is not a bitmask and there is no bit test.** `mov eax,[rcx+rax*4]` is an indexed load.
* **It is not a byte array.** The scale is `*4` and the load is a `dword`.
* **The index is the channel index**, sign-checked and bounds-checked against a length stored
  in the four bytes *before* the buffer.
* **`rcx` is the singleton at `0x143AA84A0`.** RIP-resolved twice, from `142a316ef` in the OK
  handler and `142a3176d` in the mouse hit test — both land on the same VA. **[L]**

### 2.2 What writes it

`FUN_142cb8e10` is the only function that fills it, and it does not build the array — it
**steals a `std::vector<int>`'s buffer**: **[L]**

```
142cb8ece  mov rcx, [rsp+0x98]      ; arg6
142cb8ed6  mov rax, [rcx]           ; the vector's data pointer
142cb8ed9  mov [rdi+0x2cc8], rax    ; -> the array
142cb8ee0  mov [rcx], rbx           ; and the vector is emptied
```

The `[buf-8]` length `FUN_142cb9510` reads is that vector's own length word — which is why
the array length is exactly the channel count and needs no separate field. **[D]**

Argument numbering: the callee does `push rdi; sub rsp,0x60`, so callee `[rsp+0x90]` is arg5
and `[rsp+0x98]` is arg6, and the caller sets `[rsp+0x20]`=arg5, `[rsp+0x28]`=arg6 at
`141b2cac9`/`141b2cabc`. **[D]**

The whole-image sweeps for the displacement, and what they found, are in §8.

### 2.3 What fills the vector

`FUN_141b2c7c0`, the transfer, at `141b2ca84`-`141b2caa2`: **[L]**

```
141b2ca84  call FUN_141b41c00      ; &chan[i]
141b2ca89  mov  eax, [rax+0x18]    ; chan+0x18
141b2ca99  call FUN_141b41ba0      ; &vecB[i]
141b2caa2  mov  [rax], ecx         ; vecB[i] = chan[i]+0x18
```

`FUN_141b41ba0` ends `lea rax, [r8+rbx*4]` — **stride 4**, matching the `*4` in the reader.
**[L]** The sibling `FUN_141b41c00` is the same function over the channel vector.

`vecA` (`singleton+0x2cc0`) is the identical shape one field earlier: `chan[i]+0x14`. **[L]**

### 2.4 The chain, end to end, in the code that actually runs

```
WORLD_LIST channel entry, 4th trailing u8
  -> chan+0x18            FUN_141b31ff0 @ 141b325d0     (the MODE-5 decoder - see §4)
  -> vecB[i]              FUN_141b2c7c0 @ 141b2caa2
  -> singleton+0x2cc8     FUN_142cb8e10 @ 142cb8ed9
  -> FUN_142cb9510(i)     -> non-zero enables the row
```

---

## 3. Where the value comes from on the wire — and what we put there

**It is a field we already send, and it is the last byte of each channel entry.** **[L]**

`login.log` from the run of 2026-08-20 (the one whose dialog showed two rows), the `0x000B`
body as it went out:

```
00 0600 5363616e6961 00 0000 00 02
   0800 5363616e69612d30  00000000  00 00 00 00
   0800 5363616e69612d31  00000000  00 01 00 00
   0000 00000000 00
```

Per channel: `str name`, `u32 userCount`, then `u8 worldId`, `u8 channelIndex`, `u8`, `u8`.
The **fourth** is the one `FUN_142cb9510` reads, and it is `00` for both channels. **[L]**

`crates/net/src/opcode.rs` emits it as `out.extend_from_slice(&[world_id, i, 0, CHANNEL_ENABLED])`
with `pub const CHANNEL_ENABLED: u8 = 0`. **[L]**

**It is not computed client-side.** Three things rule that out, each read rather than assumed:

* The decoder stores the byte `movzx`-widened into `chan+0x18` and does nothing else with it. **[L]**
* `FUN_141b2c7c0` reads exactly two fields of the channel struct, `+0x14` and `+0x18`. The
  `u32` user count at `chan+0x08` is never transferred to the dialog's singleton — so a
  population figure cannot be the source. (`research/channel-select.md` §7 established this
  by enumerating every memory operand in the function; nothing here contradicts it.) **[L]**
* Nothing else in the image writes `singleton+0x2cc8` except the constructor, the destructor
  and `FUN_142cb8e10` — §8. **[L]**

### 3.1 What the byte probably *means*

The third byte (`chan+0x14` -> `+0x2cc0` -> `FUN_142cb9490`) is almost certainly the
**adult-channel flag**, and that is a [D] from the client rather than an [I] from the
reference:

* `FUN_1418287f0`, the `0x00D2` builder, refuses when `FUN_142cb9490(target) != 0` **unless**
  `FUN_142a082a0(FUN_142cb84a0(singleton), 0x12)` is true. `FUN_142cb84a0` is
  `mov eax,[rcx+0x2250]`, `FUN_142a082a0` is `xor eax,eax / cmp ecx,edx / setge al`. So the
  condition is `singleton+0x2250 >= 18`. **[L]**
* The dialog's OnCreate stores the same comparison in `dialog+0x2a8` at `142a2e811`. **[L]**
* `0x12` is 18. A per-channel flag that is only passable by an account over 18 is an adult
  channel. **[D]**
* The v214 reference encodes exactly three trailing bytes — `worldId`, `channelId`,
  `adultChannel` — which names the first three and **has no counterpart for the fourth**. **[I]**

So the fourth byte is this build's own extra field with no reference analogue, and its only
observed consumer is the row-enable predicate. Beyond "non-zero enables the row" its
**semantics are not established**, and §9 says so plainly.

### 3.2 `singleton+0x2250` is `-1` and nothing observed ever changes it

`FUN_142ca5c50` (the constructor) writes `0xffffffff` at `142ca6391`. A bounded scan of
`0x142c00000..0x142d40000` for displacement `0x2250` finds exactly three sites: that store,
the getter `FUN_142cb84a0`, and a setter `FUN_142cb83d0`. **`FUN_142cb83d0` has zero direct
callers and zero pointer-table references**, verified against a control that finds both call
sites of its sibling getter (§8). **[L]/[D]**

Consequence: `dialog+0x2a8` is `0`, the `FUN_142cb9490` gate is **live** in both Draw and the
mouse test, and **the third byte must stay `0`** — a non-zero third byte would grey the row
via `channel1` and refuse the `0x00D2` outright, with no way to satisfy the age check.

---

## 4. A correction: `channel-select.md` §3.1 read the dead body of the decoder

`FUN_141b2fac0` — the login stage's `case 0x0b` — forks in its first eight instructions: **[L]**

```
141b2fae0  mov  ecx, 1
141b2fae5  call FUN_142aa2810
141b2faea  mov  rcx, [rip+0x1f91da7]   ; -> 0x143AC1898, the session pointer
141b2faf1  call FUN_142c4a810          ; = mov eax,[rcx+0x68]   the MODE
141b2faf6  cmp  eax, 5
141b2faf9  jne  141b2fb0b              ; the non-mode-5 body
141b2fafb  mov  rdx, rdi / mov rcx, r15
141b2fb01  call FUN_141b31ff0          ; MODE 5 -> here
141b2fb06  jmp  141b3020e
```

**Our client is mode 5** — `login.log`, every run: `session identity: mode=5`. **[L]** So the
channel loop at `141b30060`, which `channel-select.md` §3.1 quotes, **never executes**. It is
the same trap `maplecw-mode5-login-fork.md` and §9.4 of that document both record, one
function earlier in the chain.

**The conclusion survives**, which is why this is a correction and not a retraction. The live
decoder `FUN_141b31ff0` has an identical channel loop at `141b32540`-`141b325d9`: **[L]**

| read | primitive | destination |
|---|---|---|
| `141b32558` | `str` | `chan+0x00` |
| `141b32593` | `u32` | `chan+0x08` |
| `141b3259e` | `u8` | `chan+0x0c` |
| `141b325ac` | `u8` | `chan+0x10` |
| `141b325ba` | `u8` | `chan+0x14` |
| `141b325c8` | `u8` | **`chan+0x18`** |

and it appends into the same place: `141b322c5 lea rcx,[rsi+0x100]` -> `FUN_141b44520` ->
the world entry in `[rbp-0x30]`, then `141b32545 lea rcx,[r13+0x28]` -> `FUN_141b443d0`
appends the channel. `FUN_141b2c7c0` searches that same `this+0x100` vector and reads
`world+0x28`. **[L]**

Two identical bodies is what the client does everywhere here; the point of writing it down is
that **the addresses in `channel-select.md` §3.1 are not addresses a watch would ever hit.**

---

## 5. Draw and click read the SAME predicate

This was the third question and the answer is unambiguous.

`tools/callers.py` gives `FUN_142cb9510` **six** call sites in six functions, and a
whole-image scan for its 8-byte little-endian VA finds it in **no** pointer table — so the
six are all of them (§8). Every one of the six tests the result with a bare `test eax, eax`;
there is no enum and no magic value. **[L]**

| caller | role | on zero |
|---|---|---|
| `FUN_142a308a0` @ `142a312b9` | **Draw** | `je 142a313d8` -> loads `[r15+0x2c8]` = `channel3`, the disabled art |
| `FUN_142a31750` @ `142a31810` | **the mouse hit test** | `je 142a31854` -> return; the click is swallowed |
| `FUN_142a316e0` @ `142a31712` | **the Change/OK button** | `je 142a31738` -> no `0x00D2` is built |
| `FUN_142a31c60` @ `142a31cdf` | keyboard navigation | `jne` to accept; zero keeps scanning |
| `FUN_142a31d30` @ `142a31e19` | keyboard navigation | same |
| `FUN_142a31e50` @ `142a31e6f` | "is this row selectable" | `je` -> returns `0` |

So **there is no split to exploit**: a row cannot be made clickable while staying grey, and
making it cream necessarily makes it clickable. One byte moves all six.

### 5.1 Which bitmap each row gets, and why the owner's screenshot is exactly right

Draw's four-way decision, in order, `r12d` = row index, `r15` = the dialog: **[L]**

```
142a31168  cmp r12d,[r15+0x298]    ; == selection  -> [r15+0x2c0] channel2  BLUE
142a31287  cmp r12d,[r15+0x29c]    ; == current    -> [r15+0x2b8] channel1  grey
142a31294  cmp dword [r15+0x2a8],0
142a312a5  call FUN_142cb9490      ; +0x2cc0 non-zero -> [r15+0x2b8] channel1  grey
142a312b9  call FUN_142cb9510      ; == 0          -> [r15+0x2c8] channel3  grey, DISABLED
                                   ; else          -> [r15+0x2b0] channel0  cream
```

OnCreate sets `selection := current` (`142a30662`). **[L]** The character was in the world on
channel 0 and `SetField` carries `config.channel_id` into `singleton+0x2260`
(`142098065`/`14209806f` -> `FUN_142cb91e0`), so `current == 0`.

Therefore row 0 = selection = **blue**, row 1 = not selection, not current, `+0x2cc0[1]==0`,
`+0x2cc8[1]==0` = **`channel3`, disabled**. That is the owner's screenshot line for line, and it is
a real corroboration rather than a restatement: had `current` been `1` — which it *is*
between login and the first `SetField`, because `FUN_142cb8e10` stores the login result's
channel id — the colours would have been the other way round. **[D]**

**Both rows are unclickable today, for two different reasons.** CH.1 is excluded because it
is the current channel (`142a31803 cmp ebx,[rdi+0x294] / je`); CH.2 is excluded because the
predicate returns zero. Only the second is a bug.

### 5.2 The swallowed click, measured

`previous-runs/world-20260820-012657.log` is the long session from the run in which the owner
opened the dialog and clicked CH.2. It logs **24 distinct inbound opcodes and not one
`0x00D2`**. **[L]**

That negative has a control that makes it worth something: **`0x00D1` is in the same capture**
(`10202: <- 0x00D1 CLIENT_TRANSFER_FIELD ... 64000000 9109...`). `0x00D1` is built by the
sibling of `FUN_1418287f0` and carries the same 14-byte `FUN_140c7b890` preamble — its first
four bytes are literally `64000000`, the `100` that `channel.rs::PREAMBLE_MAGIC` records. So
the logger is demonstrably not blind to that packet family, and `0x00D2`'s absence is the
click being swallowed inside the client, exactly at `142a31817`, rather than a logging gap.
**[D]**

---

## 6. Why §0's retraction cannot repeat itself — enumerated, not eyeballed

`channel-select.md` §0 set this byte to `1`, saw an **empty** dialog, and concluded that
"some consumer *between* the decoder and the array drops the entry when the byte is
non-zero". There are exactly two functions between them, and **neither contains a branch that
could drop anything**. Every conditional branch in both loops, enumerated by decoding the
range rather than by reading it: **[L]**

| loop | range | conditional branches |
|---|---|---|
| the live decoder's channel loop | `141b32540`-`141b325df` | `141b32566 je 141b32574`, `141b32584 je 141b32590`, `141b325d9 jg 141b32540` |
| `FUN_141b2c7c0`'s transfer loop | `141b2c9b7`-`141b2caa9` | `141b2c9d6 jae 141b2caa9` |

The two `je`s in the decoder both jump *forward inside the same iteration*, over an
`add rcx,-0x10 / call FUN_14019f2c0` pair — a string release guarded on null. Neither skips a
read, an append or a field store. The third is the loop's back edge. In the transfer loop the
single branch is the loop bound. **No value of any channel field can shorten either list.**

The row count is `len(singleton+0x2cb8)` — `FUN_142cb92a0` is
`mov rax,[rcx+0x2cb8] / test / mov eax,[rax-8]`, landing in `dialog+0x2a0`, which bounds
Draw's row loop at `142a30c51`/`142a3164e` and the hit test's loop at `142a3177c`. **[L]**
That length comes from the *names* vector, which is built by the same unconditional loop. So
the fourth byte **cannot** change how many rows are drawn.

This does not make §0's observation false; it makes it **not about this byte**. §9 of
`channel-select.md` already gave the alternative: the dialog was empty in both runs for the
`(0,0)` reason, and the one populated run was an accident of uninitialised heap. The two
readings are now distinguishable, because the row count is deterministic for the first time.

---

## 7. If the fourth byte is set and CH.2 is still grey

In this order, and none of these needs a client run except the last:

1. **Confirm the byte is on the wire.** `login.log`'s `0x000B` body line; the fourth byte of
   the second channel entry is at a fixed offset in that hex string (§3). Free.
2. **Confirm the transfer ran at all.** If it did not, CH.2 would not be *drawn*; two rows
   being drawn is itself proof that `FUN_142cb8e10` executed. Free — it is the same screenshot.
3. **Watch the predicate.** `-Probe watch@0x142cb9510` reads `edx` (the row) and the return
   value directly and does not care how any of this was encoded. That settles it in one
   launch and is the correct next instrument if the byte does not work.
4. **Watch the store.** `watch@0x142cb8e10` prints `edx` (world id) and `r8d` (channel id) and
   confirms which arguments arrived.

---

## 8. Instruments, and the control each one passed

Every one of these was run **before** the result it produced was believed.

| instrument | positive control | outcome |
|---|---|---|
| `tools/fieldrefs.py` | `0x2f4 --lo 0x141c40000 --hi 0x141d60000 --write` must print `141cb7ef3 mov [r12+0x2f4], eax` | **passed** — 3 hits, 445 resync points, including that exact row |
| `tools/listing.py` | `0x140304100` must show reads at `140304138 raw`, `140304144 u8`, `140304183 u8`, then a run of `u16` | **passed**, identical |
| `tools/callers.py` | `0x1402fa9a0` -> 96 sites, 43 of them in `0x140304b20` | **passed**, exactly those numbers |
| whole-image 8-byte LE pointer scan | `0x141b25f30` must appear once, at file offset `0x33fc7a0` (the login `OnPacket` vtable slot) | **passed**; it also finds `142a31750` and `142a316e0` in the dialog's vtables, so it can see this class's slots |
| raw capstone range dump | `0x142cb9490`, which `listing.py` *can* read, disassembles identically both ways | **passed** — needed because `142cb9510` has no `.pdata` entry and `listing.py` refuses it |

Two negatives that are therefore worth something:

* **`FUN_142cb9510`, `FUN_142cb9490` and `FUN_142cb8e10` appear in no pointer table** — zero
  8-byte LE occurrences each, from a scan that finds the control. So `callers.py`'s six / three
  / one direct call sites are the complete caller sets. This closes the hole §9.7 of
  `channel-select.md` opened ("`callers.py` cannot see a virtual").
* **`FUN_142cb83d0`, the `+0x2250` setter, has no callers of either kind** — and `callers.py`
  finds both call sites of its sibling getter `FUN_142cb84a0` (`141828849`, `142a2e800`), so
  the tool can speak about leaves of this exact shape.

Displacement sweeps for `0x2cc8`:

| scan | result |
|---|---|
| `rangescan.py 0x2cc8 0x142a00000 0x142d40000` | 7 sites: `142ca6cae` (constructor, nulls it), `142ca9a44`/`142ca9a5b` (`FUN_142ca88c0`, the **destructor** — frees `+0x2cd0`, `+0x2cc8`, `+0x2cc0` in a row), `142cb8eb0`/`142cb8ec7`/`142cb8ed9` (`FUN_142cb8e10`), `142cb9514` (`FUN_142cb9510`, attributed `FUN_?` because it is a `.pdata`-less leaf) |
| `rangescan.py 0x2cb8` / `0x2cc0`, same range | 7 and 13 sites; the only stores are in the constructor, the destructor, `FUN_142cb8e10` and `FUN_142ce9600`. (`FUN_1429fbeb0`'s three `lea rcx,[rbp+0x2cc0]` are stack frames — `rangescan` does not drop `rbp`-based operands the way `fieldrefs` does) |
| `fieldrefs.py 0x2cc8 --write`, whole image | **7 writers, 666 939 resync points** — the resync count matches the tool's own documented figure, so the sweep covered the whole image rather than stopping early. Full output in §8.1 |

**A small correction to `channel-select.md` §9.8**, recorded because a shared instrument's
output is being reused: that section says the only touchers besides `FUN_142cb8e10` are "the
constructor and `FUN_142ce9600`, both of which null them". Read off the listing:

* `FUN_142ce9600` is the singleton's own "forget the world list" routine — `+0x2258 = 0`,
  `global+0x18c = 0`, `+0x2260 = 0`, clears `+0x2cb8`, frees and nulls `+0x2cc0` — and it
  **never mentions `+0x2cc8`**. **[L]**
* The function that releases all three is the **destructor `FUN_142ca88c0`** (`142ca9a44`,
  `142ca9a62`, `142ca9a80`), which §9.8 does not list at all. Its single caller is
  `FUN_142d30df0`. **[L]**

Neither changes any conclusion — both are teardown and neither can write a non-zero value —
but the sentence as written is one array too broad and one function short.

### 8.1 The whole-image write sweep, and the three rows that are *not* this class

```
$ python tools/fieldrefs.py 0x2cc8 --write
7 hit(s), 666939 resync point(s)
14088d098  movsd  qword ptr [rsi + 0x2cc8], xmm0      in 0x140886810
140c460bb  mov    dword ptr [rbx + 0x2cc8], eax       in 0x140c460a0
14294aecf  movsd  qword ptr [rsi + 0x2cc8], xmm0      in 0x1429446b0
142ca6cae  mov    qword ptr [rdi + 0x2cc8], rbx       in 0x142ca5c50   the constructor, nulls it
142ca9a5b  mov    qword ptr [rbx + 0x2cc8], r15       in 0x142ca88c0   the destructor, nulls it
142cb8ec7  mov    qword ptr [rdi + 0x2cc8], rbx       in 0x142cb8e10   nulls the old array
142cb8ed9  mov    qword ptr [rdi + 0x2cc8], rax       in 0x142cb8e10   <-- THE ONLY FILL
```

**Three of the seven are the exact trap the task brief warned about** — a displacement match
that is not a class match — and they are discriminable without following any of them, from
the operand alone: `FUN_142cb9510` dereferences `+0x2cc8` as a **pointer** and then indexes
it, so a field written by `movsd` (a `double`) or by `mov dword` cannot be it. Two doubles
and one `dword` drop out; the four qword stores are the constructor, the destructor and the
two halves of `FUN_142cb8e10`'s replace. **[L]/[D]**

So **`142cb8ed9` is the only instruction in the image that puts a non-null array there**, and
`channel-select.md` §9.8's conclusion is reproduced by a second, independent run of a
different tool.

**A displacement is a class fact, not an offset fact**, and the object here was confirmed
three independent ways before any of the above was trusted: the RIP-relative loads in the OK
handler and the mouse hit test resolve to the same global `0x143AA84A0`; `FUN_142cb8e10` is
reached with that global in `rcx` from `FUN_141b2c7c0` (`141b2cae5 mov rcx,[rsp+0xe0]`, from
`FUN_140caa510`); and the constructor/destructor pair that also touch `+0x2cc8` touch
`+0x2cb8`, `+0x2cc0` and `+0x2cd0` beside it, which is the same field group. **[D]**

---

## 9. What I did NOT establish

Stated plainly, because everything above is only worth what this section admits.

* **What the fourth byte *means*.** "Non-zero enables the row" is measured. Whether the field
  is `enabled`, `available`, `visible`, a status enum whose zero happens to be "closed", or
  something else entirely is **not** established. It has no v214 counterpart, so even an [I]
  candidate name is unavailable. If a future capture of a real server's `0x000B` ever exists,
  that is what would settle it.
* **Whether any value other than `1` behaves differently.** All six consumers do
  `test eax,eax`, so *within the client's Change Channel code* every non-zero value is
  identical. That is not the same as saying no other code path reads `chan+0x18`, which is
  the next bullet.
* **Whether `chan+0x18` is read through an *inline* walk of the channel vector.** A
  displacement sweep for `0x18` over the whole image is worthless — the displacement is
  everywhere and a hit says nothing about the class — so this was enumerated through the
  accessors instead, which is as far as it goes but goes further than expected:

  | accessor | what it does | callers |
  |---|---|---|
  | `FUN_141b443d0` | appends an element to `world+0x28` | **2** — `FUN_141b2fac0` @ `141b30068` and `FUN_141b31ff0` @ `141b32549`, i.e. the dead and live decoder bodies and nothing else |
  | `FUN_141b41c00` | `&chan[i]`, **stride `0x20`** (`141b41c4b shl rax,5`) | **3 sites, all in `FUN_141b2c7c0`** |
  | `FUN_141b20d80` | `0 <= channel < len(world+0x28)`, else walks a list at `world+0x58` | **1** — `FUN_141b2c7c0` @ `141b2c8d4` |

  The 32-byte stride is itself a corroboration of the field layout: `str*` at `+0x00`, then
  five `u32` at `+0x08`, `+0x0c`, `+0x10`, `+0x14`, `+0x18`, and four bytes of tail. **[L]**

  What this does **not** rule out is a consumer that iterates the vector inline — a
  `[begin, end)` pointer walk with the stride folded into the loop, which is exactly what the
  balloon list does 90 bytes later at `141b325f0` (`shl r15,4`). Such a loop calls none of the
  three accessors and would not appear above. This is the honest residue of §0's worry, and it
  is the reason §10 asks for one byte and not two.
* **The whole-image *read* sweep for `+0x2cc8` had not finished** when this was written; only
  the `--write` half is reported in §8.1. The read side therefore rests on the bounded
  `rangescan` over `0x142a00000..0x142d40000` (which contains the whole singleton and the
  whole dialog) plus the caller enumeration in §5, not on a whole-image pass. A reader in
  some *other* class at the same displacement would not change anything — the six consumers
  of the value are the six callers of `FUN_142cb9510`, and those are enumerated.
* **What writes `singleton+0x2250`.** Nothing observed does, but the sweep behind that was
  bounded to `0x142c00000..0x142d40000` plus a caller/pointer check, not a whole-image
  `fieldrefs` run. An inlined `mov [reg+0x2250], imm` elsewhere would have been missed.
* **The `0x00D2` body layout.** `crates/net/src/channel.rs` already records that the field
  order is [D] from call order and that no capture exists. Nothing here improves that.
* **What the client does after the send.** `FUN_1418287f0` also passes through
  `FUN_142cc42d0(singleton, 500, 0)`, a rate limit with two busy flags at `+0x2330`/`+0x2338`
  and a timestamp at `+0x2334`. Whether a busy flag latches when the reply never comes is
  **unknown**, and if it does, a single unanswered `0x00D2` could poison later attempts in the
  same session. Read as a risk, not a finding.

---

## 10. The one-variant test

**One constant, one byte, one field.**

`crates/net/src/opcode.rs`:

```rust
pub const CHANNEL_ENABLED: u8 = 0;   // change this to 1. Nothing else.
```

Leave the third byte at `0` — §3.2 shows that with `singleton+0x2250` stuck at `-1` a
non-zero third byte would grey the row through a *different* branch and refuse the `0x00D2`
outright, which would mask the result completely.

**Open the dialog from inside the world, not at character select.** Between the login result
and the first `SetField`, `singleton+0x2260` holds the *priming* channel `1`, so `current`
would be row 1 and the two rows' colours would swap. In the world it is `0`, which is the
state the previous run was observed in.

### What the owner should watch

| observation | what it means |
|---|---|
| **CH.2 turns cream** — pale off-white, distinct from both the blue CH.1 and the grey it is now. RGB of the four 68x20 row bitmaps, rendered out of `_Canvas_000.wz` in `channel-select.md` §4 and not re-measured here: `channel0` (224,222,212) cream, `channel1` (138,138,138) grey, `channel2` (56,168,219) blue, `channel3` (132,131,129) grey | `singleton+0x2cc8[1]` is non-zero and `FUN_142cb9510` returned it. The chain in §2 is right end to end. **CH.1 stays blue** — it is `selection` and that check comes first, so this run changes exactly one row's appearance |
| **click CH.2 once: CH.2 turns blue and CH.1 turns grey** | the mouse gate at `142a31817` let the click through and wrote `selection = 1`. This is the whole reported symptom fixed. CH.1 going *grey* is expected, not a regression — it stops being `selection` and becomes `current` |
| **CH.2 still grey, still unclickable** | the byte is not reaching `+0x2cc8`. Go to §7 — the next instrument is `watch@0x142cb9510`, not another byte |
| **the dialog comes up with no rows at all** | §6 says this cannot be caused by this byte, so it would mean `FUN_142cb8e10` did not run this time — i.e. the §9.9 priming is intermittent, not the byte. Check `login.log` for `channel 1 advertised` first |
| **a fault or a freeze at the character-select screen** | new code ran. Read the ELog (`0x008F`/`0x0090`) at the top of the *next* login |

### STOP THERE. Do not press the Change button.

**If CH.2 becomes clickable, the next thing that happens is a packet nobody answers.**
`FUN_142a316e0` -> `FUN_141892840` -> `FUN_1418287f0` builds and sends **`0x00D2`**, and
`grep` over `crates/world` finds **no reference to `ChangeChannelRequest` or `0x00D2`
anywhere** — the parser exists in `crates/net/src/channel.rs` and nothing calls it. An
unanswered packet freezes the client's entire UI including the quit prompt, which reads on
screen as a crash.

A single click only moves the highlight (`WM_LBUTTONDOWN` -> `[+0x298] = i`); it is
**`WM_LBUTTONDBLCLK`** (`142a3183f`) or the Change button that sends. So:

* **single-click CH.2 — safe, and it is the whole measurement.**
* **double-click, or the Change button — sends `0x00D2` and will freeze the client.** Worth
  doing deliberately *last*, as its own step, if a capture of the `0x00D2` body is wanted:
  `world.log`'s last inbound line names it and would settle the field order
  `channel.rs::parse` currently has to guess. Close the client by hand afterwards.
