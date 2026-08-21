# All 36 sub-cases of `0x0089`, and where an item line can actually reach the chat log

**2026-08-21.** Companion to **`research/client-messages.md`**, whose §2 left 28 of the 36
sub-cases unread and whose last section asked one question: *is there a sub-case that puts
`<Item> x<quantity> earned. (<Tab>)` into the chat log?* This file enumerates all 36 and
answers it.

Labels are the project's: **[L]** read off this client's listing or its own data · **[D]**
derived from two or more [L] facts · **[I]** inferred, not established.

**No Ghidra** — another agent held the project lock. Everything below came from
`client-patched/MapleStory.exe` through `tools/listing.py`, `tools/reads.py`,
`tools/callers.py`, `tools/dump_va.py` and `tools/dump_stringids.py`, plus one scratch
script that enumerates every call site of the string resolver.

---

## Summary

| | |
|---|---|
| **is there a `0x0089` sub-case that draws `<Item> x<quantity> earned. (<Tab>)` in the chat log?** | **No.** Verified negative — see §4 |
| what the enumeration did find | **sub-case `12`** — `u32 chatType, str text`. The **server** picks the chat category *and* the words. **[L]** |
| the better route, different opcode | **`0x02D1` `UserEffectLocal`, effect `8`** — `u8 count, count × (u32 itemId, i32 quantity, u8 inBag)`. The **client** composes `'%s x%d earned. (%s)'` from string `0x00EC` and posts it to **chat type 6**. **[L]** |
| chat type 6's colour | **`0xFFBBBBBB`** — opaque grey, RGB(187,187,187). Type 7, which `0x00BB` uses today, is `0xFFFFFF00`, **yellow**. **[L]** |
| how many of the 36 post to the chat log | **23** — 10 at type 6, 11 at type 11, 1 at a type the server chooses, 1 at type 7 (counting kind 0 once, at its live type) |
| how many never touch it | **13**, of which **4** (19, 29, 32, 33) have no handler at all |

---

## 1. The table, read out of the PE

`tools/dump_va.py 0x142d44a88 160` gives 36 dword RVAs and then `cc cc cc cc` padding, so
the table is exactly 36 entries and the `cmp eax,0x23 / ja` bound at `0x142d43f14` is
exactly its length. **[L]**

`tools/listing.py 0x142d43ee0` prints the whole 3128-byte function, every arm included, and
`tools/reads.py 0x142d43ee0 1` lists **39 read sites** at exactly the addresses the listing
marks. The two instruments share a loader — that is a **shared blind spot, not
corroboration** — but they are the same loader the repo's positive controls exercise
(`tools/listing.py 0x140304100 | grep READ` reproduces `reads.py 0x140304100 2`'s direct
reads at the same addresses, checked before any of this).

### Every arm

`chat` is a call to `FUN_1415eca30(text, type)` — the chat log — or to `FUN_1415a87a0`, what
that function tail-calls, with the same argument shape. `screen` is `FUN_142572050` on the
singleton `0x143AD30D8`. `dialog` is `FUN_142a26280`.

| kind | target | handler | body after the kind byte | posts where | renders |
|---:|---|---|---|---|---|
| **0** | `142d43f32` | `FUN_142d59360` | `u8 quiet, i8 subMode, …` — see `client-messages.md` §4 | **screen** (14 sites); chat **7** at `142d59673` (string `0xA6`, the meso "portion lost" line); chat **11** at `142d59bf2` (string `0xF5`, a refusal); chat **6** at `142d5987a` — **gated on `fieldType == 0x56`, which no map in this client has** | item name + count, to the **screen** |
| **1** | `142d43f42` | `FUN_142d59e20` | `u32, u8, str, raw, u8` — the quest record, `research/quest-state.md` | none on the walked paths | — |
| **2** | `142d43fa0` | `FUN_142d5ce00` | `u32 itemId` | chat **11** (`142d5ce9a`) | item **name**, no count. String `0xF7` *"[%s] has passed its expiration date…"* |
| **3** | `142d43fb0` | `FUN_142d5da20` | EXP — `client-messages.md` §3 | chat **6** when `in_chat != 0`, else **screen** | — |
| **4** | `142d43fc0` | inline | `u16, u8` | chat **6** (`142d44061`) | SP. Strings `0xC3` / `0xC4` |
| **5** | `142d44070` | inline | `i32` | chat **6** (`142d440fe`) | fame. `0xDF` / `0xE0` |
| **6** | `142d44109` | `FUN_142d5f990` | `u64, u32, str` | chat **6** (`142d5fe88`, via `FUN_1415a87a0`) | mesos. `0xE1` / `0xE4` / `0xE6` |
| **7** | `142d44178` | inline | `u32` | chat **6** (`142d44206`) | guild points. `0x9B0` *"You have earned Guild Points (+%d)"* / `0x9B1` |
| **8** | `142d44211` | `FUN_142d60230` | `u32, u32, u32` | chat **6** ×3 (`142d602bb`, `142d6032e`, `142d603b2`) | contribution. `0x9B2` / `0x9B3` / `0x9B4` / `0x9B5` |
| **9** | `142d44221` | inline | `u32 id` | chat **6** (`142d44246`) | a string looked up by `FUN_140399280` on `[0x143AA8328]` — the **same singleton** the item-name lookups use, but a different accessor. No count |
| **10** | `142d44255` | inline | `u8 count, count × u32 itemId` | chat **11** (`142d442ea`) | item **name**, no count. `0x80D` *"The item [%s] has expired…"* |
| **11** | `142d4433b` | inline | `str text` | chat **11** (`142d4435f`) | **the server's own words**, fixed category |
| **12** | `142d4437f` | inline | **`u32 chatType, str text`** | chat **`chatType`** (`142d443b8`) | **the server's own words, at a server-chosen category.** See §3 |
| **13** | `142d43f52` | `FUN_142d5c7b0` | `u32, str` | chat **11** via a helper (`142d5ae07`, string `0xF91`) | — |
| **14** | `142d43f62` | inline | `u32, str` | **none** — `FUN_1402e1a20`, then `FUN_1413f1760(u32)` | — |
| **15** | `142d443d8` | inline | `u8 count, count × u32 itemId` | chat **11** (`142d4446a`) | item **name**, no count. `0x80F` *"%s's seal has expired."* |
| **16** | `142d444bb` | inline | `u8 count, count × str` | chat **11** (`142d44502`) | **the server's own words**, `count` of them |
| **17** | `142d44528` | inline | `u8 count, count × u32 itemId` | chat **11** (`142d445ba`) | item **name**, no count. `0x80E` |
| **18** | `142d4460b` | inline | `u8 count, count × u32 id` | chat **11** (`142d446ad`) | a name via `FUN_1407b2910` on `[0x143AA84D0]`. `0xCA7` |
| **19** | `142d44a6a` | — | **no handler.** Same target as `type > 35` | — | — |
| **20** | `142d446d5` | inline | *(no read at all)* | chat **11** (`142d44711`) | `0xCAB` *"The Android is not powered…"* |
| **21** | `142d44720` | inline | *(no read at all)* | chat **11** (`142d4475c`) | `0xCAC` *"You recovered some fatigue by resting."* |
| **22** | `142d44119` | `FUN_142d5ff40` | `u32, u32` | chat **6** ×2 (`142d60077`, `142d60086`) | battle points. `0x9BA`..`0x9BD` |
| **23** | `142d44129` | inline | `str, str` | chat **6** ×2 (`142d4414c`, `142d4415a`) | **the server's own words**, two lines, fixed category |
| **24** | `142d4476b` | inline | `u8 which` (0..3) | **dialog** | `0xF25`..`0xF28`, the Return-Stone refusals |
| **25** | `142d447e0` | inline | `u32 exp, u32 fieldBonus` | **screen** ×1 or ×2 | `0xC1` *"You received EXP (+%lld)"* and `0xD3`. A second EXP path that never reaches the chat log |
| **26** | `142d448ef` | inline | `str` | **none** — `FUN_140d84b70(&s, 0)` | — |
| **27** | `142d4491e` | inline | `u32, str` | **none** — `FUN_1402e1ac0` | — |
| **28** | `142d44969` | inline | `u32 id` | chat **11** (`142d449b4`) | a string produced by `FUN_14025ade0` on `[0x143AAA058]`; posts nothing when it is empty |
| **29** | `142d44a6a` | — | **no handler** | — | — |
| **30** | `142d449be` | `FUN_142d60450` | *(no read at all)* | **dialog** | `0x12FB` *"This is an expired item."* |
| **31** | `142d449ce` | inline | *(no read at all)* | **dialog** | `0x14D9` *"This item has expired."* |
| **32** | `142d44a6a` | — | **no handler** | — | — |
| **33** | `142d44a6a` | — | **no handler** | — | — |
| **34** | `142d44a52` | `FUN_142d5d6c0` | `u8, str, str` (+ a third `str`, gated) | **dialog** | — |
| **35** | `142d44a5f` | `FUN_142d975a0` | `u8 type, u32 amount` | chat **6** (`142d97838`, via `FUN_1415a87a0`) | `0x17CD` *"You have gained %s Contribution (+%d)"*; `FUN_1402c8af0(type)` returning false drops it |

**Counts, so the table can be checked against itself.** 36 rows.

* **4 have no handler** — 19, 29, 32, 33, all pointing at `142d44a6a`, the same target as
  `type > 0x23`.
* **23 reach the chat log**: 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 16, 17, 18, 20,
  21, 22, 23, 28, 35. Of those, **10 at type 6** (3 only when its `in_chat` byte is set),
  **11 at type 11**, kind 0 at type 7 and type 11 on its live paths, and kind **12** at a
  type the server sends.
* **13 never reach it**: 1, 14, 19, 24, 25, 26, 27, 29, 30, 31, 32, 33, 34.
* **4 raise a dialog** (24, 30, 31, 34); **1 posts only to the screen** (25); kind 0 posts to
  the screen as well as the chat log.

23 + 13 = 36, which is the check.

### The five neighbours that are not arms

`client-messages.md` §5 lists `FUN_142d5f7c0`, `FUN_142d5f8b0`, `FUN_142d600d0`,
`FUN_142d60150` and `FUN_142d60400` as type-6 posters "in the `0x0089` family". They are
real `.pdata` entries sitting between the sub-handlers, and `tools/callers.py` reports
**zero** for all three of its kinds — no `call`, no tail `jmp`, no pointer in data — for
every one of them. **[L]** They are not reachable from the table above. Whether they are
dead code or reached by a computed call is **not established**; `callers.py`'s own docstring
names computed targets as the thing it cannot see.

---

## 2. Sub-case 12 — the only arm whose chat category the server chooses

```asm
142d4437f  mov   rcx, rsi
142d44382  call  0x1406e8c20      ; <<< READ u32   the chat category
142d44387  mov   ebx, eax
142d44389  mov   r14d, 0xb
142d4438f  cmp   eax, 0x24
142d44392  cmova ebx, r14d        ; > 0x24  ->  forced to 11
142d44396  lea   rdx, [rbp + 0x77]
142d4439a  mov   rcx, rsi
142d4439d  call  0x1406e9050      ; <<< READ str   the text
142d443a3  mov   rcx, [rbp + 0x77]
142d443a7  test  rcx, rcx / je    ; null  -> nothing
142d443ac  cmp   byte [rcx], 0 / je   ; empty -> nothing
142d443b1  movzx edx, bx
142d443b8  call  0x1415eca30      ; the chat log, at the category just read
```

Both reads are **unconditional** and there are no others in the arm. **[L]** Body:

```text
u8   12
u32  chatType      142d44382    0..0x24; anything above becomes 11
str  text          142d4439d    u16 length then bytes; empty draws nothing
```

`crates/net/src/notice.rs` records that `0x00BB`'s colour and tab "are not controllable".
**This arm makes them controllable** — it is the same `FUN_1415eca30`, with the type coming
off the wire instead of being hard-coded to 7.

---

## 3. Why chat type 6 is grey, and type 7 is not

`FUN_1415b6c00(out, type, flags)` is the colour table. `lea eax,[rdx-1]` then
`cmp eax,0x23 / ja`, so the index is `type - 1` and the table at `0x1415b6d60` has 36 dword
entries. **[L]** Reading them and the arms they point at:

```text
type  1..5   dynamic - read out of a config object at +0x174, +0x178, +0x17c, +0x180
type  6      mov edx, 0xFFBBBBBB     <- opaque grey, RGB(187,187,187)
type  7      mov edx, 0xFFFFFF00     <- yellow                      (0x00BB uses this)
type  8      mov edx, 0xFFFFF080
type  9      mov edx, 0xFF60CEFF
type 10      mov edx, 0xFF000000
type 11      mov edx, 0xFFFFAFAF     <- 723 of 1133 call sites use type 11
type 12      mov edx, 0xFF003F7F
type 13      mov edx, 0xFF770042
type 14      mov edx, 0xFFFFFFFF     (the default arm; also every out-of-range type)
type 34..36  mov edx, 0xFFFF0000
```

**So the owner's "gray text" is chat type 6, `0xFFBBBBBB`, and the `0x00BB` line they get today is
yellow.** **[L]** for both constants.

`FUN_1415a3010`, the chat list's own routine, calls the colour table at `0x1415a31e1` with
`edx = r13d` (the type) and `r8d = 0`. The `r8d == 0x2710` override at `0x1415b6c26` is only
taken when the caller passed a type ≥ 10000, which `FUN_1415eca30` never does. **[L]**

> **The named blind spot.** The colour-table call sits on the `else` of
> `0x1415a31bc  test rdi,rdi / je 0x1415a31d9`, where `rdi = *(arg8)`. When that pointer is
> non-null a virtual call at `0x1415a31c9` supplies the colour instead. I traced arg8 back to
> `&[rbp-0x69]` inside `FUN_1415a87a0` (`0x1415a8891`) and **did not** establish what
> `FUN_1415eca30`'s empty-object arguments leave there. If it is ever non-null for an
> ordinary post, the whole colour table is dead code — which is implausible but unmeasured.
> **[D]**, not [L], for "the line will render grey"; **[L]** only for the constant.

---

## 4. The verified negative: no `0x0089` sub-case draws the item line into the chat log

Three of the four criteria for the owner's line are met by several arms — the wording exists
(`0x00EC`), several arms resolve an item **name**, and eighteen arms post to the chat log —
but no `0x0089` arm does all of it. The decisive test is *which functions load string
`0x00EC`*, because that string is the sentence.

**The instrument, and its positive control.** A scratch script enumerates **every** call site
of the string resolver `FUN_1408a9e40` by disassembling inside each `.pdata` extent — 6994
sites — and back-resolves the `edx` immediate at each. It reports the ones it cannot resolve
rather than dropping them: **46**. Its positive control is `client-messages.md`'s own:
string `0x533` resolves to exactly **one** site, in `FUN_14209ee50`, the `[Welcome]` poster
that `research/talking-back.md` named independently. **[L]**

Result:

```text
0x00EC   142d597f1  in FUN_142d59360     the 0x0089 type-0 drop pick-up
0x00EC   14278aeff  in FUN_1427863f0     the 0x02D1 / 0x02AF user-effect handler
0x00EC   14278b52f  in FUN_1427863f0     the same, a second arm
0x00ED   142d597bb  /  14278aee5  /  14278b516     the same three
0x00EE   142d597d6  in FUN_142d59360     only there
```

**Changing the question rather than re-running the tool**, per `CLAUDE.md`: an independent
raw byte scan for `B8+r <id32>` across all of `.text`, with **no** requirement that a
resolver call follow, finds 117 hits for the three ids. Restricted to `edx` — the resolver's
second argument — the only ones outside the three string-table builders
(`FUN_140a10a40`, `FUN_140a165f0`, `FUN_140a9d500`) and unrelated UI code are those same two
functions, plus four look-alikes in the `0x142d4...` range that turn out to be
`FUN_1406ed520` calls — **outbound opcodes** `0x00EC`/`0x00ED`/`0x00EE`, not string ids.
Checked one by one. **[L]**

None of the 46 unresolvable sites is in `FUN_142d43ee0`, `FUN_142d59360`, `FUN_1427863f0` or
any of the eleven sub-handler functions. **[L]**

**So: within `0x0089`, the only code that draws `'%s x%d earned. (%s)'` is type 0 sub-mode 0,
and `client-messages.md` §5 already established that its chat-log site is gated on
`FUN_141829fd0(FUN_141892840()) == 0x56` and that no map in this client has that
`fieldType`.** The negative stands, and the enumeration above is what it rests on.

---

## 5. `0x02D1` effect 8 — the route that does exist

The second function loading `0x00EC` is `FUN_1427863f0`, which `research/level-up.md` and
`research/quest-complete-effect.md` already identify as the handler for **`0x02D1`
`UserEffectLocal`** (`u8 effect`) and **`0x02AF` `UserEffectRemote`** (`u32 charId, u8
effect`).

Its **first** switch is `ecx = effect - 8`, bounded at `0x45`, through a two-level MSVC table
(byte table `0x142791300`, dword table `0x14279129c`):

```asm
142786482  lea   ecx, [rbx - 8]                    ; rbx = the effect byte, read at 14278644e
1427864a8  cmp   ecx, 0x45 / ja 0x14278bd20
1427864b4  movzx eax, byte [0x142791300 + idx]
1427864bc  mov   ecx, dword [0x14279129c + eax*4]
1427864c6  jmp   rcx
```

`byte[0] = 0` and `dword[0] = 0x14278b474`, so **effect 8** is the arm at `0x14278b474`.
**[L]** (The other `0x00EC` site, `0x14278aeff`, is in `dword[9] = 0x14278abcd`, reached from
`byte[39]`, i.e. **effect 47** — an item-obtained announcement behind world-id and
`FUN_142ce70a0` guards that I did not chase. It is a lead, not a finding.)

### Effect 8's body, field by field

```text
u8   effect = 8               14278644e
u8   count                    14278b47e     ** ZERO TAKES A COMPLETELY DIFFERENT BRANCH **
  count times:
    u32  itemId               14278b494
    i32  quantity             14278b4a1     signed; see below
    u8   inBag                14278b4ab     boolean: test al,al / setne
```

`tools/reads.py 0x1427863f0 3` lists exactly these four addresses and no others inside the
arm, at the same addresses `tools/listing.py` marks. The loop back-edge is
`14278b90f sub r15,1 / jne 0x14278b491`, and `0x14278b491` is the `u32 itemId` read, so one
iteration is **9 bytes**. Whole body = **2 + 9n**. **[L]**

**`count == 0` is not "no items".** `14278b488 test eax,eax / je 0x14278b948` goes to a
different body shape entirely — `str` at `14278b952` then `u32` at `14278b9c4`, posted to
the **screen** through `FUN_142572050`. A body with `count = 0` and nothing after it leaves
the client reading a string and a dword off the end of the packet. That is exactly the
underrun `CLAUDE.md` records killing this client twice. **Always send `count >= 1`.** **[L]**

### What each field does

```text
quantity  > 0   ->  0x00EC  '%s x%d earned. (%s)'          when inBag == 0
                    0x00ED  '%s x%d earned. (%s / Bag)'    when inBag != 0
quantity  < 0   ->  neg, then
                    0x00EF  '%s x%d has been lost. (%s)'   when inBag == 0
                    0x00F0  '%s x%d has been lost (%s / Bag)'
quantity == 0   ->  14278b5ae jns 0x14278b8b7 - the item is skipped in silence
```

**A unit trap, and it is not the same unit as `0x0089`.** `0x0089` type 0 sub-mode 0's third
field is a **three-valued enum** (`0` plain, `1` "and Bag", `2` "/ Bag", anything else
abandons the message). Effect 8's third field is a **boolean** — `test al,al / setne` — and
there is no "and Bag" form at all. Copying `item_slot::*` across would put `2` on the wire
and get the `/ Bag` wording. **[L]**

Two more silent drops, both per-item, both continuing the loop:

* the client resolves the name itself (`FUN_140398ba0` on `[0x143AA8328]`) and skips the item
  when it is null or empty — `14278b4e6` / `14278b4ef`;
* `14278b65e cmp dword [rbp+0x28], 0x3d0df7 / jne` skips item id **4001271** specifically.

### Where it posts

```asm
14278b84c  xor   r9d, r9d
14278b84f  lea   edx, [r9 + 6]           ; type 6
14278b853  lea   r8d, [r9 - 1]           ; -1
14278b857  mov   rcx, [rbp + 0xd8]       ; the composed sentence
14278b85e  call  0x1415a87a0
```

`FUN_1415eca30(text, type)` is a thin wrapper whose whole body ends
`0x1415ecbea xor r9d,r9d / lea r8d,[r9-1] / mov rcx,[rsi] / call 0x1415a87a0` — the same
four arguments, with `byte [rsp+0x28] = 0xff` in both. **[L]** So effect 8's post is a chat
post at **type 6**, identical in shape to `FUN_1415eca30(text, 6)`, differing only in that
the six string slots `FUN_1415eca30` leaves empty are filled — which is what makes the item
name a link.

**Nothing gates it.** `FUN_1427863f0`'s field/state gates (`0x14278bd29 call 0x141892840`,
`0x14278bd4a call 0x142826340`) run **after** arm 8 has already posted — arm 8 ends
`14278b943 jmp 0x14278bd29` — and they only decide whether the *second* switch runs.
Second-switch entry `[8]` is `0x0279102e`, the common exit. **[L]** So effect 8 does its
whole job in the first switch and adds no animation and no sound.

### One thing I did not establish

The composed string buffer `[rbp+0xd8]` is nulled **once**, at `0x14278b474`, outside the
loop; `FUN_14019ba10` writes into it every iteration and the post is inside the loop. If that
routine appends rather than assigns, a `count > 1` body would post growing concatenations.
Every other arm in the family nulls its buffer immediately before a single `FUN_14019ba10`
call, which is the *construct-then-assign* pattern, so assignment is the likely reading —
but it is **[I]**. **Send one item per packet until a run says otherwise.**

---

## 6. What I did NOT establish

* **The meaning of sub-cases 9, 13, 14, 26, 27, 28 and 34.** Their field widths are **[L]**
  from `reads.py`, and where they post is **[L]**; what the fields *mean* is unread.
* **Sub-case 1's full read list.** `reads.py` gives `u32, u8, str, raw, u8` through
  `FUN_142d59e20`; `research/quest-state.md` owns that decode and I did not re-derive it.
* **Effect 47 (`0x2F`) of `0x02D1`.** It reads `u8, u8, u8, u32 itemId, u32 count, u8` and
  reaches the same `'%s x%d earned. (%s)'` chat post, but behind `FUN_142ce70a0`,
  `FUN_1425b2120` and a chain of world-id comparisons (`0x109`, `0x134`, `0x1B1`) that I did
  not resolve. It looks like a server-wide rare-drop announcement. **Do not send it blind.**
* **Whether chat type 6 is a separate visible tab.** `FUN_1415a3010` indexes
  `this + 0x20 + type*24`, one list per type — that is **[L]** and is `client-messages.md`'s
  reading — but which of those lists the player's chat window actually shows is not
  established. If type 6 turns out to be a tab the default view hides, the line would be
  posted and invisible, which on screen is indistinguishable from a dropped packet.
  **The client test in §7 is what separates those two.**
* **Anything on the wire.** Not one byte of this has been sent. No client run was made.
* **`FUN_142d5f7c0` / `FUN_142d5f8b0` / `FUN_142d600d0` / `FUN_142d60150` /
  `FUN_142d60400`** — zero callers of all three kinds, purpose unknown.

---

## 7. WIRE IT LIKE THIS

`crates/net/src/message.rs` now builds all three of these and pins every byte with tests
(`cargo test -p net`: 355 + 10 passed, 0 failed, run in a private `CARGO_TARGET_DIR` because
other agents share `target/`). **Nothing is wired**; nothing in `crates/world` references any
of them.

The coordinator owns `crates/world/src/session/`. Where a quest turn-in awards an item:

```rust
use net::{message, stats};

// The client composes '<Item> x<n> earned. (<Tab>)' itself, looks up both names, and
// posts it to chat category 6 - grey. 11 bytes. NOTE THE OPCODE: 0x02D1, not 0x0089.
replies.push((stats::USER_EFFECT_LOCAL, message::item_gained_in_chat(item_id, count)));
```

and for a line whose words are the server's:

```rust
// 0x0089 sub-case 12: our text, our chat category. This is the packet that makes
// notice.rs's "colour and tab are not controllable" false.
replies.push((message::MESSAGE,
              message::chat_line(message::chat_category::GREY, "…")));
```

`item_gained_in_chat` panics on `count == 0` — deliberately, because a zero count is a
different body shape in the client and a two-byte packet would underrun its reader.
`chat_line` cannot be short: both of sub-case 12's reads are unconditional.

The item **pick-up** stays as it is. `client-messages.md` §5 settled that `0x0089` type 0
sub-mode 0's chat-log copy is unreachable in this client, so `message::item_gained` already goes to
the bottom-right area and only there. Nothing in this file changes that, and the two packets
do not interact.

**The one discriminating client test.** One change, one run:

> Where the quest turn-in currently awards an item, additionally send
> **`net::stats::USER_EFFECT_LOCAL` (`0x02D1`)** with body
> **`net::message::item_gained_in_chat(itemId, count)`** — 11 bytes:
> `08 01 <u32 itemId> <i32 count> 00`. Change nothing else.

| what the owner sees | what it means |
|---|---|
| **`<Item> x<n> earned. (<Tab>)` appears in the chat log, in grey** | Done. Effect 8 is the route, type 6 is grey, and the item link works. Drop the `0x00BB` line for item rewards. |
| the line appears **but in some other colour** | The route is right and §3's named blind spot is the explanation — the per-line colour override at `0x1415a31bc` is being taken. Colour is then a second, separate question. |
| **nothing appears, and nothing else breaks** | Either the item id did not resolve a name (`14278b4e6`), or type 6 is a chat category this client's window does not show. Re-send with `net::message::chat_line(11, "…")` — sub-case 12 at category 11, which is where 723 of the client's own 1133 chat posts go. If *that* shows, type 6 is the problem, not the packet. |
| **the client freezes or dies** | The body went short. The only way that happens with `item_gained_in_chat` is `count = 0`, which the builder refuses — so check the log for the bytes actually sent before believing anything else. |

`world.log` records both directions, and `tools/test-server.ps1` archives the previous run
into `previous-runs/` rather than deleting it. Copy whichever run answers this into
`research/fixtures/`.

**Do not send both `0x02D1` effect 8 and a `0x00BB` line for the same reward in the same
run.** Two changes at once has already produced one unexplained crash on this project.
