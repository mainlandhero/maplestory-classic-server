# Skill points: the extended-SP table, which pool is which job, and how to put one on screen

Written 2026-08-27, from a static pass over `client-patched/MapleStory.exe` plus every
`0x007C` and `0x013B` body in `research/fixtures/`. **No client run was spent on this** and
none is needed to settle questions 1–4; question 5 has one residual that does need a launch
and it is named in §11.

Labels: **[L]** read off a listing, a capture or this client's own data; **[D]** derived from
two or more [L] facts; **[I]** inferred, and nothing on this machine confirms it.

Prior work this builds on and does not repeat: `research/skills.md` (the `presence[8]` record
block, `0x0081`, the `0x013B` latch), `research/charstat-layout.md` §4 (the SP fork's exact
condition and the stat block's byte offsets), `research/level-up.md` §4 (the `0x007C` mask),
`research/job-advancement.md` (`0x007C` bit 5 is the job change and the client animates it
itself).

---

## 0. Answer up front

| question | answer |
|---|---|
| What is the extended-SP block on the wire? | **`u8 count`, then `count` × (`u8 jobLevel`, `u32 amount`)** — `1 + 5n` bytes. Identical in the character record and in `0x007C`. **[L]**, §1 |
| What identifies a pool? | A **job level (tier)**, `0`–`10`, **not** a job id. The lookup returns 0 immediately for any key above `10`. **[L]**, §2 |
| Which pool is 1st job? | **Key `1`.** 2nd job is **`2`**, 3rd `3`, 4th `4`, beginner `0`. Derived from `FUN_140286e90`, the job→tier function, decoded in full. **[D]**, §2 |
| How does the client decide which pool a `+` click spends from? | It doesn't spend anything. It **displays** `GetSP(job)` for the tab's job and greys the `+` button when that is `≤ 0`; the request carries only `skillId` and `count`. The server must charge `tier(skillId / 10000)` itself. **[L]**, §3 |
| Then why does a Beginner's `+` work today with an empty SP table? | Because for **any tier-0 job the client computes SP itself**: `min(10, level) − 1 − (levels in skills 1000, 1001, 1002)`. The pools are not read at all. **[L]**, §4 — and an archived run matches it to the point, §4.2 |
| Which packet updates SP mid-session? | **`0x007C`, mask bit 15 (`0x8000`)**, and the extended encoding is fully legal there. **[L]**, §5 |
| Is it a delta? | **No — it is a total replacement.** The extended arm calls the list-clear helper first, so every `0x007C` carrying SP must carry **every** pool. **[L]**, §5.1 |
| Can `0x0081` carry SP? | **No.** The `0x0081` handler contains no reference to `charstat+0xd7` or `+0xef` — verified negative with a positive control inside the same function. **[L]**, §6 |
| Does the client decrement a pool after a successful skill-up? | **No.** The only two things that ever add a pool entry are the two packet decoders; nothing else writes the list. Without a `0x007C` the number on screen never moves for a real job. **[L]**, §6 |
| Has this server ever sent SP? | **Never.** Of the 439 outbound `0x007C` in `research/fixtures/`, the 391 with a body line decode to 9 distinct masks and **not one has bit 15**; the other 48 are accounted for. `character_stat_block` writes `u8 0`. **[L]**, §5.3 |
| What must a job advancement send? | `0x007C` with **bit 5 (job) and bit 15 (SP) in the same packet**, SP encoded for the **new** job. §7 — and that last clause contradicts a doc block on disk, §8. |

---

## 1. The extended-SP block, byte for byte

### 1.1 In the character record — `FUN_1402cb0d0`

`FUN_140302e30` (the stat block) reads its own job back out at `0x1403030b0` and forks. The
extended arm is `0x1403031e2`:

```text
1403031e2  lea rdx,[rbx+0x93] / xor ecx,ecx / call 0x1402f7010   ; sp = 0, reads NOTHING
1403031f0  lea rcx,[rbx+0xd7]                                    ; &charstat->spPools
140303200  call 0x1402cb0d0                                      ; the pool table
```

`FUN_1402cb0d0(list, packet)`, `0x1402cb0d0 .. 0x1402cb1e8`, **[L]** every line:

```text
1402cb0ee  call 0x1402fe400        ; clear the list first
1402cb0f8  READ u8   count         ; 0 -> straight to the total store
   count x {
1402cb123  READ u8   jobLevel      -> ebx
1402cb12e  READ u32  amount        -> edi
1402cb1ac  call 0x140300b60        ; push_back the obfuscated (jobLevel, amount) pair
1402cb1b1  add ebp, edi            ; running sum of the amounts
   }
1402cb1cf  mov [rsi+0x18], ebp     ; list->total = sum   (== charstat+0xef)
```

The plain arm, for a job the fork does not accept, is one `u16` at `0x14030314a` into
obfuscated `+0x93`, and it then **frees the pool list and zeroes `+0xe7`, `+0xdf`, `+0xdb`,
`+0xef`** at `0x1403031c6`–`0x1403031da`. **[L]**

### 1.2 In `0x007C` — the same shape, inlined

`FUN_1402cbb50`, the `StatChanged` decoder. The bit is **15**, and the bytes say so:
`0x1402cbd8f` is `0f ba e5 0f` = `bt ebp, 0x0f`. **[L]**

```text
1402cbd8f  bt  ebp, 0xf / jae 1402cbecf        ; SP bit clear -> skip
1402cbd99  mov edx,[rbx+0x37] / lea rcx,[rbx+0x33] / call 0x1401ab420   ; job, out of the record
1402cbda8  call 0x1403024c0                    ; == net::opcode::uses_extended_sp
1402cbdaf  je  1402cbeb2                       ; plain -> one u16 at 1402cbeb5 -> +0x93/+0x97
1402cbdba  lea rcx,[rbx+0xd7]
1402cbdc6  call 0x1402fe400                    ; CLEAR THE WHOLE LIST
1402cbdd1  READ u8   count
   count x {
1402cbdf4  READ u8   jobLevel
1402cbdff  READ u32  amount
1402cbe80  call 0x140300b60                    ; push_back
1402cbe85  add r12d, esi                       ; sum
   }
1402cbea4  mov [rbx+0xef], r12d                ; total
```

**Identical field order, identical widths, identical destination.** One builder serves both.

### 1.3 The list object, and why the offsets are not guesses

`FUN_1402fe400` zeroes `+0x10`, `+8`, `+4`; `FUN_140300b60` increments `[list+4]` and links
through `[list+0x10]`; `FUN_1402cb0d0` stores the sum at `[list+0x18]`. Against
`charstat+0xd7` that is:

| charstat | list | what | **[L]** from |
|---|---|---|---|
| `+0xd7` | `+0x00` | allocator/head-of-object | — |
| `+0xdb` | `+0x04` | entry count | `140300c73 inc [rbp+4]` |
| `+0xdf` | `+0x08` | head | `1402fe40a mov rbx,[rcx+8]` |
| `+0xe7` | `+0x10` | tail | `140300cc3 mov [rbp+0x10], rsi` |
| `+0xef` | `+0x18` | **sum of every pool's amount** | `1402cb1cf`, `1402cbea4` |

and the **charstat constructor** `FUN_1402f78c0` zeroes exactly `+0xdb`, `+0xdf`, `+0xe7`,
`+0xef` in a row at `0x1402f7a59`–`0x1402f7a6d`. Three independent sites agreeing on the same
four offsets. **[L]**

`FUN_140300b60` is **push_back**, not push_front (`140300cb4 mov [rdi-0x20], rbx` links after
the tail). The lookup in §2 walks from the head, so **wire order is search order and the first
entry with a given key wins.** Send each key once. **[D]**

### 1.4 What a builder needs

```text
extended (uses_extended_sp(job) == true):
    u8  count                      0..255
    count x {
        u8   jobLevel              0..10   (see §2; >10 is stored and never read)
        u32  amount                little-endian
    }
plain (uses_extended_sp(job) == false):
    u16 sp                                 (read SIGNED on the way out - 1407e4e30 cwde)
```

Wire cost `1 + 5n`. `crates/net/src/stats.rs::Sp` and `crates/net/src/combat.rs::SpChange`
already encode exactly this and both are correct. **Nothing about the encoding needs to
change; what is missing is a value in it.**

---

## 2. Which pool is which job — settled, with the function decoded

The pool is fetched by `FUN_1402cb030(list, u8 key)`:

```text
1402cb03f  movzx esi, dl
1402cb042  cmp   dl, 0xa
1402cb045  ja    1402cb098        ; key > 10  ->  RETURN 0
1402cb047  rbx = [list+8]         ; head; null -> return 0
   walk: obf_get_u8(node+0, node+4) == key ?
1402cb0aa  found: tail-jmp obf_get_u32(node+8, node+0x10)   ; the amount
```

**[L].** A key above `10` can never be read back. **A job id as the key returns 0 for every
job in the game**, and that is the single most expensive mistake available here, because the
table is still stored and still counted into `+0xef` — so the "unspent points" indicator (§6.3)
would light up while every skill window read 0.

The key comes from `FUN_140286e90(job)`. It has no `.pdata` entry, so `tools/listing.py`
refuses it; disassembled by hand from `0x140286e90` to its last `ret` at `0x140286eff`
(112 bytes, three returns, no calls):

```text
140286e90  edx = job / 1000
140286eaa  if job == (job/1000)*1000        -> return 0     ; 0, 1000, 2000, 3000
140286eae  if 800000 <= job <= 800099       -> return 0
140286eb9  edx = job / 100
140286ece  if job == (job/100)*100          -> return 1     ; 100, 200, 300, 400, 500
140286ed9  r8d = job % 10
140286ef7  if (job % 10) <= 2               -> return (job % 10) + 2
140286efd  otherwise                        -> return 0
```

**[L]**, and it lands exactly on MapleStory's explorer numbering:

| job | tier / pool key | |
|---|---:|---|
| `0` (Beginner) | 0 | but see §4 — tier 0 never reads a pool |
| `100 200 300 400 500` | **1** | **first job** |
| `110 120 130`, `210 220 230`, … | **2** | **second job** |
| `111 121 131`, `211 221 231`, … | **3** | third job |
| `112 122 132`, `212 222 232`, … | **4** | fourth job |

**[D]** — the mapping is arithmetic from an [L] listing; the *names* ("first job") come from
the client's own quest text, already quoted in `research/job-advancement.md` §1–2.

### 2.1 A second, independent site treats the key the same way

`FUN_142d3f500` is the **builder for outbound `0x0420`** (`research/msexe-send-opcodes.txt`
line 1878). At `0x142d40b5a` it sets `esi = r15d` — the tier index — and then remaps only for
Dual Blade (`job − 430 < 10`, tested at `0x142d40bd3`): index 3 → key 2, index 5 → key 3,
index 6 → key 4. It then calls the same `FUN_1402cb030(charstat+0xd7, sil)` at `0x142d40e19`.
**[L]** Two callers, two different ways of producing the key, both small tier indices.

### 2.2 The fork predicate is the same in all three places

`FUN_1403024c0` (record + `0x007C`) and the chain inside `FUN_1407e4d70` are the same set.
The accessor's version replaces the `job−400` mask with a call to `FUN_140823400`, and that
function is `job−400 ∈ {0,10,11,12,20,21,22}` **or** `430 ≤ job ≤ 439` — i.e. the 400 family
plus Dual Blade, which is what the other two spell inline. **[L]**, disassembled at
`0x140823400`. So there is **no class for which the display and the decoder disagree about
the encoding** — a Rogue-shaped trap that had to be checked and is not there.

---

## 3. What actually puts a number on screen, and what greys the `+` button

One function answers both: `FUN_1407e4d70(job, charstat)` — call it `GetSP(job)`.
`0x1407e4d70 .. 0x1407e4f2e`. **Five callers, all enumerated** (`tools/callers.py`; 0 tail
jumps, 0 pointers):

| caller | what it does with the result | **[L]** |
|---|---|---|
| `0x1425837b0` @ `142583875` | formats it with `"%d"` (`0x143274298`) next to the resource named `"sp"` (`0x14328c85c`) — **the number in the skill window** | `14258386f mov ecx,[rsi+0x2e8]` |
| `0x142584d80` @ `142584db9` | `test r14,r14 / jle` at `0x142584e4f`; **`≤ 0` disables** the row's `BtSpUp` (`+0x11d8`) and `BtSpUpAll` (`+0x11e8`) through `vtable[0x70]` | the enable/disable is `142584e9b` |
| `0x142586d7f` @ `142586dcb` | the `+` **click**: `count = min(room, GetSP)` (`cmp esi,eax / cmovl`), then `FUN_142d4bd80` sends `0x013B` | `142586dd0`–`142586ddd` |
| `0x142586e10` @ `142586ed0` | the same for the second button | |
| `0x1411b13a0` @ `1411b14c9` | walks a `u32` array of job ids calling `GetSP` on each and raises a flag if any is `> 0` | `1411b14c4` loop |

The argument is a **job id**: `0x1411b13a0` feeds it a list of job ids, and the skill window
feeds it `[wnd+0x2e8]`, which `FUN_1407b37f0` fills from a `Skill.wz` job record at
`0x1407b382f`. **[D]**

The extended path of `GetSP` is three instructions:

```text
1407e4e3c  mov ecx, ebx              ; the job
1407e4e3e  call 0x140286e90          ; -> tier
1407e4e45  lea rcx,[rdi+0xd7]        ; the pool list
1407e4e5b  jmp 0x1402cb030           ; -> the pool's amount
```

**[L].** So the number the skill window prints **is** the pool whose key is
`tier([wnd+0x2e8])`, and the `+` button is greyed exactly when that pool is 0. **[D]:** with an
empty table and a tier ≥ 1 job, the tab reads 0 and every `+` is dead — which is
*"Currently I get none."* The one step that is **[I]** is that `[wnd+0x2e8]` is the *selected
tab's* job rather than some other job in the window's list; §11 item 1.

---

## 4. The Beginner is a special case, and it is why the `+` button already works

### 4.1 Tier 0 never reads a pool

`GetSP` takes its first branch — `0x1407e4e60` — when `job % 1000 == 0`, which includes
**job 0**. That branch ignores the argument and ignores the pool list entirely:

```text
1407e4e6e  r8 = charstat[0x1039]                 ; the skill-level hash map
             ebx = level(1000) + level(1001) + level(1002)   ; walked bucket by bucket
1407e4f0f  eax = obf_get_u32(charstat+0x27)      ; the character LEVEL
1407e4f14  ecx = min(10, level)
1407e4f1e  ecx -= ebx
1407e4f25  return ecx - 1
```

**[L], every instruction.**

```text
Beginner SP = min(10, level) − 1 − (Three Snails + Recovery + Nimble Feet levels)
```

capped at **9**, which is exactly the 3+3+3 those skills can absorb. **No packet can raise
it.** Sending a pool with key `0` is stored, is summed into `+0xef`, and is never read by the
skill window.

### 4.2 An archived run measures the formula to the point

`research/fixtures/quest-forfeit-0151-action3-on-the-wire-world.log`, 03:26:31–03:26:42:
**eight** `0x013B` requests, each answered with `0x0081`, raising 1000 to 3, 1001 to 3 and
1002 to 2 — **8 points spent, then the clicks stop.** The level-up to 10 is at 03:26:50,
*after* the last click, so the character was **level 9** throughout.

```text
min(10, 9) − 1 − 0  =  8            spent: 8
```

**[L]** both halves. The whole run happened with an **empty** SP table on the wire (§5.3) and
the button still worked — which is the positive control for "tier 0 does not read the pools",
and simultaneously the reason nobody has noticed the pools were never sent.

> **This kills a hypothesis that would otherwise have cost a launch.** "The `+` button works,
> therefore the SP plumbing works" is false: it works *only* for job 0, and for the same
> reason it will keep working after job advancement on the **Beginner tab** while the
> Magician tab shows 0.

---

## 5. `0x007C` bit 15 is the only way to move a real job's SP

### 5.1 It replaces, it does not add

The extended arm calls `FUN_1402fe400(charstat+0xd7)` at `0x1402cbdc6` **before** reading the
count, and `FUN_1402fe400` walks the chain, deletes every node and zeroes count/head/tail.
**[L]** So:

* every `0x007C` that sets bit 15 must carry **all** pools the character has, not the one that
  changed;
* a `0x007C` with bit 15 and `count = 0` **wipes SP to zero** — `Sp::empty_extended()` is a
  destructive statement, not a no-op;
* `+0xef` is then overwritten with this packet's sum, so it cannot drift.

The record path clears too — `FUN_1402cb0d0`'s **first** act is `call 0x1402fe400` at
`0x1402cb0ee`, with `rcx` still holding the list (`0x1402cb0eb mov rsi,rcx` does not disturb
it). **[L]** And on top of that, all three record decodes reached from the stage module
(`0x141b26432`, `0x141b29649`, `0x141b36b5e`) construct a fresh charstat via
`FUN_14108e7f0 → FUN_1402f78c0` immediately before decoding, so the list starts empty anyway.
**Neither packet can ever accumulate duplicate pools.** **[D]**

### 5.2 Nothing in the handler tests bit 15 — and the redraw hangs off bit 5

The decoder returns the mask in `eax`; `FUN_142d54780` puts it in `edi` and spills it to
`[rbp-0x60]` (`0x142d548a2`, `0x142d548a4`), reloading it at `0x142d54916`, `0x142d5499f` and
`0x142d55b52`. **Every** test against it, enumerated across the whole listing rather than
searched for:

```text
142d5491f  bt   edi, 0x10          ; LEVEL
142d549d6  test edi, 0x40030       ; LEVEL|JOB|EXP|MESO
142d549f0  test dil, 0x30          ; LEVEL|JOB      (when byte 1 is non-zero)
142d549fe  and  eax, 0x40000       ; MESO
142d55b59  and  r15d, 0x30         ; LEVEL|JOB
142d5617c  bt   r12d, 0xa          ; HP
```

**[L]. There is no test of bit 15 anywhere**, so SP has no effect, sound or dialog of its own.

The last-but-one line is the useful one:

```text
142d55b59  and r15d, 0x30 / je 142d55efd
142d55b63  mov rcx,[rip+0xd74a96]        ; = 0x143aca600, THE SKILL WINDOW singleton
142d55b6f  mov edx, -1
142d55b74  call 0x1425840a0              ; the skill window's ROW BUILDER
```

**[L]** — `0x142d55b6a + 0xd74a96 = 0x143aca600`, the singleton `research/skills.md` §5.2
identifies as the skill window, and `FUN_1425840a0` is the row builder that same file names
(it `lea`s `UI/Skill.img/entry/BtSpUp` at `0x1425844ab`).

So: **a `0x007C` carrying the LEVEL or JOB bit rebuilds the skill window's rows; an SP-only
`0x007C` does not.** The advancement packet in §7.3 carries JOB, so it rebuilds — which is what
is wanted anyway. For a plain skill-up (SP only) the window is refreshed by
`FUN_1425837b0`/`FUN_142584d80` instead, and when *that* runs is **[I]** — §11 item 2.

### 5.3 This server has never sent it — enumerated, not eyeballed

Every `-> 0x007C` line in `research/fixtures/*.log` (439 of them), with the mask decoded from
bytes 3..6 of the body:

```text
  mask 0x00000020  x1     JOB                     <- the one !job, body 01 00 01 20000000 6400 0000 00 00
                                                     (job 100, subJob 0, no trailers - 13 bytes)
  mask 0x00000400  x156   HP
  mask 0x00001000  x3     MP
  mask 0x00001400  x13    HP|MP
  mask 0x00004040  x7     STR|AP
  mask 0x00010000  x72    EXP
  mask 0x00010400  x1     HP|EXP
  mask 0x00017c10  x17    LEVEL|HP|MAXHP|MP|MAXMP|AP|EXP   <- the level-up bundle
  mask 0x00040000  x121   MESO
                          ...and NOTHING with 0x8000.
```

391 bodies parsed; the other 48 are `hit by mob` / `idle regen` / `+N exp` / `used item` lines
in two files whose body lines were stripped, and no code path in `crates/world/` sets `sp` on
a `StatChange` (grep: the only `sp: Some(...)` in the workspace are four unit tests in
`crates/net/src/combat.rs`). **[L]**

> **The instrument lied once and was caught.** The first version of this scan looked for a
> `body <hex>` line after each `0x007C` **before** checking whether it had already run into the
> next packet's line, and it reported one packet with bit 15 set, mask `0x0003e811`. That was
> the `0x013B` body `940e8711 e8030000 03000000` — bytes 3..6 of a *skill-up request*. Reordering
> two lines removed it. A mask that appears exactly once (`0x20`, the single `!job`) proves the
> scan can see a one-off, so the zero for `0x8000` is a verified negative and not an empty one.

---

## 6. `0x0081` cannot carry SP, and the client never decrements a pool

### 6.1 The handler does not touch the list — verified negative

`tools/rangescan.py 0xd7 0x142d30000 0x142d60000` → **one** site, `142d40e0e`, in the `0x0420`
builder. `0xef` over the same range → **zero**. The `0x0081` handler `FUN_142d57f20` lives
inside that range and is not among them.

*Positive control, in the same call:* `rangescan.py 0x1039 0x142d50000 0x142d60000` returns
three sites and one of them is `142d57fc1 lea rax,[rsi+0x1039]` — **inside `FUN_142d57f20`**.
The scan can see into the function it is reporting nothing about. **[L]**

### 6.2 Nothing anywhere adds or edits a pool except the two decoders

* `tools/callers.py 0x140300b60` (push): **2 call sites**, `0x1402cb1ac` (record) and
  `0x1402cbe80` (`0x007C`). 0 tail jumps, 0 pointers.
* `tools/rangescan.py 0xd7 0x140001000 0x1433c5e0a` (whole `.text`, 52 MB): **52 sites**, of
  which the `lea`-on-a-real-register ones are the two decoders, the record decoder, the
  client's own `0x007C` **encoder** `FUN_1402cb5a0` at `0x1402cb739`, `GetSP`, the `0x0420`
  builder, and three destructors.
* `tools/rangescan.py 0xef` over the same range: **38 sites**; the `dword` ones are the two
  decoder stores, the constructor's zero, the record's plain-branch zero, and four reads.

**[D]: a skill point is spent only when the server says so.** For job 0 the number falls
because the *skill level* rose and the formula subtracts it; for any tier ≥ 1 the number does
not move at all until a `0x007C` rewrites the table. A server that answers `0x013B` with
`0x0081` alone will let the same point be spent for the rest of the session.

### 6.3 One more consumer of the total, for completeness

`FUN_142d029d0(user)` returns a bitmask: `2` if AP > 0, `4` if SP > 0 — and for an extended
job it tests **`charstat+0xef`, the raw total**, not a pool (`142d02a12`). That is the "you
have unspent points" indicator. Consequence: a beginner's derived SP never lights it, and a
pool with an unreachable key (>10, or a tier the character does not have) lights it forever.
**[L]** for the reads, **[I]** for the name.

---

## 7. Answering `0x013B`, and what a job advancement must send

### 7.1 The request names no pool

`<- 0x013B 940e8711 e8030000 03000000` = `u32 tick, u32 skillId, u32 count`. **[L]**, and all
**twelve** `0x013B` bodies in `research/fixtures/` are skill `1000`/`1001`/`1002` with count 1
or 3 — enumerated, `grep -h 0x013B research/fixtures/*.log | grep "byte body"`. There is no
pool field and there never will be: the client already knows which tab it is on.

**The server picks the pool: `tier(skillId / 10000)`, via the §2 table.** Skill `2001002` →
job 200 → tier 1. Skill `1000` → job 0 → tier 0, which is the client's own budget and not a
pool at all.

### 7.2 The reply, for the client to end up in the right state

Two packets, in this order, on every **successful** skill-up of a tier ≥ 1 skill:

```text
1.  0x0081  ChangeSkillRecordResult   clear_request_latch = 1, show_effect = 1,
                                      one entry: (skillId, newLevel, masterLevel, expiry)
2.  0x007C  StatChanged               mask = 0x8000, extended table with EVERY pool,
                                      the charged one already decremented
```

`crates/net/src/skills.rs::change_skill_record_result` builds (1) and
`crates/net/src/stats.rs::StatChange { sp: Some(Sp::Extended(..)) }` builds (2). Neither needs
changing.

**A wrong pool is silent, never fatal.** `FUN_1402cb030` returns 0 for a key it cannot find and
0 for a key above 10; nothing faults, nothing throws, the number is simply wrong or the button
is simply grey. What *is* fatal is getting the **encoding** wrong — a plain `u16` where the
client expects a counted table, or the reverse, desynchronises every byte after it in a packet
with no length prefix. `Sp::matches_job` is the guard and it must be asked about the job the
record will hold **after** this packet, not before (§8).

Refusals stay exactly as they are today: `0x0081` with `clear_request_latch = 1` and no
entries. The latch (`user+0x2330`) is what makes an unanswered `0x013B` disable a whole class
of requests for the session — `research/skills.md` §4.2.

### 7.3 Job advancement

`research/job-advancement.md` established that `0x007C` bit 5 is the job change and that the
client plays `JobChanged` itself. What it could not say — because the pool encoding was not
decoded — is that **bit 5 alone leaves the new job's tab at 0 SP with its `+` buttons greyed**,
which is precisely the owner's report. The advancement is **one** packet:

```text
01              excl_request_sent   (clears user+0x2330; harmless if unset)
00              secondary
01              context_flag
20 80 00 00     mask = JOB | SP = 0x8020
C8 00           job    = 200
00 00           subJob = 0
01              sp pool count = 1
01              jobLevel 1        <- FIRST JOB
01 00 00 00     amount 1
00              charm    absent
00              recovery absent
```

**19 body bytes**, and the first 13 of them are byte-for-byte the shape of a packet that has
already been on the wire and worked: `!job 100` sent `01 00 01 20000000 6400 0000 00 00`
(§5.3), which is this with the SP block removed. So the only untested bytes in the whole
advancement are the six of the pool table.

Ordering inside the packet is not a choice: the client reads values in ascending bit order, so
job (bit 5) is written to `charstat+0x33` at `0x1402cbbf8`–`0x1402cbc04` **before** the SP fork
reads it back at `0x1402cbd99`. Both `0` and `200` are on the extended side of the fork, so
this particular advancement cannot desynchronise even if the rule were got backwards — but
`jobs::sp_encoding_changes` exists for the advancements where it can.

**Nothing else is needed for the skills themselves to appear.** The skill window's rows come
from the client's own `Skill.wz`, looked up by job id through `FUN_1407b2690` — the server
sends no catalogue. The `presence[8]` record block and `0x0081` carry only *levels*. **[D]**

---

## 8. Two things on disk that this pass contradicts

**1. `crates/net/src/combat.rs`, the `job_for_sp` doc block, is wrong and is labelled [L].**

> *"It does **not** use a job sent in this packet, so if a packet changes both job and SP the
> client decodes SP with the **old** job."*

It uses the **new** job. `0x1402cbbef test bpl,0x20` is the JOB arm; it stores the job into
obfuscated `charstat+0x33` at `0x1402cbc04`, and the SP fork loads `charstat+0x33` at
`0x1402cbd99` — 405 bytes later, straight-line, with no back edge between them. `stats.rs`'s
`Sp::matches_job` note says the opposite of `combat.rs` and **`stats.rs` is the correct one.**
The `debug_assert!` in `combat.rs::stat_changed` would fire on the wrong side of the fork for
any advancement that crosses it. **[L]** — I did not edit either file; `crates/` is not mine.

**2. `crates/store/src/db.rs` and `crates/net/src/opcode.rs` both say the pool list is
undecoded.** `db.rs`: *"a pool list nobody has decoded"*; `opcode.rs` on `Character::exp`: the
same. That is now false in both places, and `stats.rs::SpPool::job_level`'s *"**What
`SpPool::job_level` must contain is NOT established**"* is answered by §2.

---

## 9. The owner's requirement, as numbers

Restating it against what the client will actually read. `L` is the character's level.

| tier | pool key | entitlement | note |
|---|---:|---|---|
| Beginner (job 0) | — | `min(10, L) − 1`, capped at 9 | **client-computed, unchangeable.** Do not try to grant it; do not refuse a beginner skill-up on an SP count the server invented, because the screen will disagree |
| 1st job (100/200/300/400/500) | **1** | `1 + 3 × max(0, min(L, 30) − 10)` | `1` at the moment of advancement, `+3` per level from 11 to 30 |
| 2nd job (`x10`/`x20`/`x30`) | **2** | `1 + 3 × max(0, min(L, 70) − 30)` | same shape; the upper bound is the owner's to choose, 70 is the classic 3rd-job level |

*"The player should not lose skill points if they job advance late"* falls straight out: the
entitlement is a function of `L`, not of when the advancement happened, so a level-25 beginner
who advances gets `1 + 3×15 = 46` in pool 1 immediately. **Nothing in the client caps it** —
the amount is a `u32` and only the display and the button gate read it. Keep it below `2^31`;
the `+` handler compares it signed (`cmp esi,eax / cmovl`).

**Spent points must be subtracted server-side and the whole table resent**, because §6 shows
the client never decrements. The natural store shape is one row per `(character_id, tier)`
holding *spent*, with the entitlement recomputed from level on every send — that way a level-up
adds 3 without a migration and a re-derivation can never disagree with itself.

---

## 10. Instruments, and the control for each

| instrument | what it answered | control that was run |
|---|---|---|
| `tools/listing.py` | every listing quoted here | its own docstring control — `0x140304100` prints `140304138 raw`, `140304144 u8`, `140304183 u8`, then the `u16` run, matching `tools/reads.py 0x140304100 2` line for line. Run first, before anything below |
| `tools/reads.py` (loader) | the `.pdata` extents and the read-primitive table | contains `0x1406e8fb0` and `0x142d23ef0` — the repo copy, not a scratchpad shadow; every command was run with the repo as cwd |
| `tools/fieldrefs.py` | — | docstring control run (`0x2f4`, `0x141c40000..0x141d60000`, `--write` → exactly `141c4d261`, `141c4e6ee`, `141cb7ef3`). Used only to prove the scanner works; the whole-image runs went through `rangescan.py` |
| `tools/rangescan.py` | `0xd7` / `0xef` over all 52 MB of `.text`; `0x1039`, `0x2e8`, `0x308` over the skill module | positive: `0x1039` in `0x142580000..0x1425a0000` returns the 13 sites `research/skills.md` §2.2 already names, including `14258bc0c` and `142592920`. For the `0x0081` negative, `0x1039` was re-run over `0x142d50000..0x142d60000` and found `142d57fc1` **inside the function being reported empty** |
| `tools/callers.py` | callers of `GetSP` (5), `FUN_1402cb030` (1 + 1 tail jmp), `FUN_140300b60` (2), `FUN_1402fe400` (8), `FUN_1403094b0` (5) | it reports tail `jmp` and data pointers separately, and it earned that here twice: `FUN_1402cb030`'s **only** reference from `GetSP` is a tail `jmp` at `1407e4e5b`, and `FUN_1425837b0` has **zero** call sites and is reached through the vtable at `0x143470a70`. A `call`-only scan would have called both dead |
| hand disassembly (capstone, repo loader) | `FUN_140286e90`, `FUN_140823400` — both `.pdata`-less, both refused by `listing.py` | each was read to its `ret`; `FUN_140286e90` is 112 bytes with three returns and no calls, so there is nothing off-screen |
| `tools/dump_va.py` | `0f ba e5 0f` at `0x1402cbd8f` = `bt ebp,0xf`; the `"%d"` and `"sp"` strings | the two strings decode as UTF-16 in `.rdata` at the RIP targets the listing computes |
| a `0x007C` mask census over `research/fixtures/` | §5.3 | see the box in §5.3 — the first version of the scan produced a **false positive** and was fixed; a mask with a single occurrence (`0x20`) proves it can see a one-off |

**`tools/rtti.py` names none of these classes** and was not used. Every class name here
("the skill window", "the charstat") is a description, and none came from the v214 tree.

### What the v214 reference contributed, and what it did not

Nothing was taken from `C:\Users\user\Desktop\ModernMapleSource`. Two candidates it would have
suggested were **killed against this image**:

* *"the extended-SP key is an index into a fixed-size remaining-SP array"* — it is a **tier**,
  and `FUN_1402cb030` rejects anything above 10 rather than indexing;
* *"SP arrives with the skill-up result"* — `0x0081` does not touch the list at all (§6.1).

---

## 11. What I did NOT establish

1. **Which job `[wnd+0x2e8]` holds after a job advancement.** The only write to it in
   `0x142580000..0x142598000` is the constructor's zero (`142582e1e`), plus `FUN_1407b37f0`
   writing through the pointer `FUN_142584fa0` passes at `142584ffd` — and that pointer is fed
   `[wnd+0x308][0]`, the **first** element of a vector whose producer I could not find. The
   displacement scan cannot separate classes, and the one push-shaped candidate (`FUN_142587b80`)
   turned out to belong to a different vtable. So whether the Magician tab is selected, and
   therefore whether the SP number is `GetSP(200)` or `GetSP(0)`, is **[I]**. This is the one
   thing in this document that needs a launch.
2. **Whether an SP-only `0x007C` redraws an already-open skill window.** The handler's own
   rebuild is gated on `mask & 0x30` (§5.2), which excludes bit 15, so the number would have to
   be redrawn by `FUN_1425837b0` — a vtable method (slot `+0x98` of `0x1434709d8`) whose call
   site in the base class I did not find. Sending SP alongside job or level sidesteps the
   question entirely, and a job advancement does that anyway. It matters only for a plain
   skill-up with the window already open; step 2 of the test plan measures it for free.
3. **Which of `FUN_1403094b0`'s five call sites is the `SetField` path.** Three of them
   (`0x141b26432`, `0x141b29649`, `0x141b36b5e`, all in the stage module) construct a fresh
   charstat first; `0x142dd992e` belongs to opcode `0x0167` and `0x14108bf73` decodes into a
   caller-supplied object. Since `FUN_1402cb0d0` clears the list itself (§5.1) this does not
   change any conclusion — it is recorded so nobody re-derives it.
4. **What `FUN_142cc0370` checks**, and `[skillInfo+0xb6] == 0` / `[skillInfo+0x3c] <= 0` — the
   other three conditions on the `+` click. Unchanged from `research/skills.md` §7 item 8. All
   of them passed in the captured run, which is all that has ever been needed.
5. **What `800000..800099` is.** Both `FUN_140286e90` and `GetSP` carve that range out as
   tier 0, and `FUN_142584fa0` computes `[a global] + 0xc3500` at `0x142585033`, so it is a
   real family of something. I did not chase it and nothing here depends on it.
6. **The 4th-job / Dual Blade tiers beyond key 4.** `FUN_1402cb030` allows keys up to 10 and
   `FUN_142d3f500` remaps a Dual Blade's indices; what keys 5–10 are for is not established and
   nothing in this project needs them.
7. **Whether the beginner formula's `min(10, level)` is what MapleStory intends.** It is what
   *this* client computes. If the owner wants a beginner to hold more than 9 points, the answer is
   that no packet can do it.

---

## Wire it like this

`crates/world/src/session/` belongs to the coordinator; none of the following was done, and
nothing outside this file was touched.

### Step 1 — the record, so SP survives a relog

`crates/net/src/opcode.rs::character_stat_block` writes `out.push(0)` for the extended branch.
That becomes the §1.4 table. **The two constants beside it move with it:**

```rust
pub const STAT_BLOCK_MAP_ID_AT: usize = 84;   // + 5 * pools.len()
pub const STAT_BLOCK_LEN: usize      = 108;   // + 5 * pools.len()
```

`character_stat_block` already carries `debug_assert_eq!(out.len(), stat_block_map_id_at(chr.job))`,
so this fails loudly in tests rather than quietly on the wire — which is the only reason the
map id has ever landed in the right place. `python tools/channel_smoke.py` exercises the record
over an independent transport and is where to catch it before a launch.

### Step 2 — the store

`characters` has no `sp` column and `character_skills` has no tier. One row per
`(character_id, tier)` holding **points spent** is enough; the entitlement comes from `level`
(§9) and the wire value is `entitlement(level, tier) − spent`. Same `ALTER TABLE ADD COLUMN`
guard shape as `add_experience_column`, for the same reason.

### Step 3 — charge the right pool in `on_skill_up`

`crates/world/src/session/skills.rs` currently refuses everything outside `BEGINNER_SKILLS`.
When first-job skills are allowed, the charge is `tier(skill_id / 10000)` and the reply becomes
the **pair** in §7.2. Every effect must hang off the same transition — `CLAUDE.md`'s
"a guard whose answer is ignored is not a guard", and this handler has three effects to keep in
step (the level row, the `0x0081`, the `0x007C`) where the quest bug had two.

For `skill_id / 10000 == 0` charge **nothing**: the client's budget is the §4.1 formula and a
server-side counter can only disagree with the screen.

### Step 4 — job advancement sends one packet, not two

The 19 bytes in §7.3. `gm_job` in `crates/world/src/session/gm.rs` currently sends bit 5 alone;
adding `sp: Some(Sp::Extended(vec![SpPool { job_level: 1, amount: 1 }]))` to the same
`StatChange` is the whole change, and it is also the cheapest possible test of this document.

### The one launch worth spending, and what each outcome means

**One variant at a time.** Steps 1–3 can all wait; step 4 alone is decisive.

| # | do | watch for | what it means |
|---|---|---|---|
| 1 | `!job 200` **with the SP pool added to the same `0x007C`** — nothing else changed. Then open the skill window | the number beside `sp` on the **Magician** tab | **`1`, and `+` is not greyed** → §2 is right end to end and the rest is store work. **`0` with `+` greyed** → §11 item 1: the window is reading a different job. **Client dies** → the encoding desynchronised, which §7.3 says cannot happen for 0→200, so §8 item 1 is what to re-check first |
| 1b | only if step 1 shows `0`: close the skill window and re-open it | does the number become `1` | `1` on re-open → the rebuild at `0x142d55b74` did not reach the label, and SP must be sent with the window shut. Still `0` → §11 item 1, and the next question is which job `[wnd+0x2e8]` holds — a static one, not another launch |
| 2 | in the same run: click `+` on a Magician skill (the server will refuse it — `on_skill_up` only knows the three beginner skills) | `world.log` for `<- 0x013B` with a `2xxxxxx` skill id | **the request went out** → the button was genuinely live and the client's own count came from `GetSP`. Nothing in the log → the button only *looked* enabled. This costs no extra launch and it separates "the number is right" from "the gate is right" |
| 3 | in the same run: open the **Beginner** tab | `min(10, level) − 1 − spent` | the §4.1 formula, on screen. It must be **unchanged** by step 1. If the pool leaked into it, §4 is wrong |
| 4 | only after step 3 of *Wire it like this* is done: click `+` on a Magician skill, then **click it again** | does the second click raise the level too | one click proves the pool; **two prove the `0x007C` reply decremented it**. First works, second silently does nothing → the `0x007C` was not sent or charged the wrong key, and §6.2 says the client will never fix that by itself |

Step 1's failure modes are three different answers, not one, and 1b tells the two survivable
ones apart without a second launch. That is the whole point of running it in that order.

---

## 12. The pools were granted at advancement and then WIPED by the next SetField - 2026-09-14

seedling, 2026-09-14: *"job advancing to Bowman at level 12, the game did not grant them the 7
SP that they need because they're over leveled."*

Measured, not inferred. The advancement `0x007C` was byte-perfect and the log proves it:
`world-ch0.log` 01:27:56.268, character 218 (purr, account 3), body
`01 00 01 20800000 2c01 0000 01 01 07000000 00 00` - mask `JOB|SP` = `0x8020`, one pool, tier
`1`, amount `7`. `entitlement(First, 12) = 1 + 3×2 = 7`, so both the amount and the encoding
were right, and §7.3's shape "has been on the wire and worked". So the grant itself was **not**
the bug - which is why "over leveled" was a red herring; the server computes the retroactive
total correctly.

What wiped it is `character_stat_block` (`crates/net/src/opcode.rs`): on the extended-SP branch
it writes **`out.push(0); // no SP pools`**. Every login record and every SetField record
therefore carries an empty SP table, and §5.1 established that the extended arm **clears the
whole list before reading it**. So the record does not merely omit SP - it *zeroes* it. purr
advanced at 01:27:56 (7 SP, shown), walked through portal `out02` at 01:28:01 (`SetField`), and
the pool went to 0 about five seconds later. A relog or a channel change does the same. That is
exactly the "did not grant" the player saw: they got the points, then the very next field entry
took them away, before they had spent any (the live DB confirms it - `character_skill_spend`
empty, no Bowman skills, only the three beginner skills at 3/3/3).

The store already had the right model - `skill_points_available(tier) = entitlement(tier, level)
− spent`, purr's spent is 0, so available is 7 - and `session::skills::skill_point_reply`
already builds the correct all-pools `0x007C` and was sent after every skill-up. It was simply
never sent after a SetField. **This is the "built is not wired" failure exactly.**

The fix mirrors the keymap restore, which rides after every SetField for the same reason (its
manager belongs to the stage the SetField rebuilds): `skill_point_reply` now rides after every
SetField too, in `session/field.rs::go_to_map` (portal walks, revive, taxi) and in the
login-time SetField in `session/mod.rs` (login, channel change). `skill_point_reply` gained a
reached-tier gate - `tier_for_job(chr.job)` - because `pool_entitlement` returns a level's
worth for any tier, so without it a level-12 **beginner** refreshed after a SetField would be
handed 7 first-job points they never advanced into.

The stat block itself still sends `0`, corrected a beat later by the `0x007C`. Threading the
real pools into the shared record builder would be the tidier fix, but that builder is
byte-perfect and feeds ~10 call sites (the character-select list included, where SP is not
shown); the post-SetField packet is the proven, lower-risk pattern and the same one the keymap
uses. If SP is ever seen to flicker to 0 on a field change, the stat block is where to move it.

Fixture: `research/fixtures/seedling-bowman-lvl12-sp7-granted-then-setfield-wipes-world.log`.
