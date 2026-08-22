# The stat-change / EXP path — `0x007C`, and what a level-up actually costs

Written 2026-08-20. **No Ghidra** (another agent held the project lock), **no client run**.
Everything below was read out of `client-patched\MapleStory.exe` with `tools/reads.py`,
`tools/listing.py`, `tools/callers.py`, `tools/dataref.py`, `tools/xref.py` and six
throwaway scripts whose exact source is reproduced in §9 so every number here can be
re-derived without me.

Tags on every claim:

* **[L]** read off the listing / the raw instruction bytes of the PE.
* **[D]** derived by analysis over those bytes.
* **[I]** inferred, or a candidate from the v214 reference — not provable from this client.

> ## THIS IS NOT WIRED
>
> `crates/net/src/stats.rs` compiles and its tests pass, and **nothing sends any of it.**
> `crates/world/` has never referenced this module. On screen it is identical to not
> existing. §8 is the wire-it-like-this section; until someone does that, this entry belongs
> under `STATUS.md`'s **BUILT, and deliberately NOT wired** heading, not under WIRED.

---

## 0. The three answers, up front

| the server wants to say | packet | body | status |
|---|---|---|---|
| "your stats changed" (level, HP/MP, AP/SP, EXP, meso, …) | **`0x007C`** | `u8 u8 u8`, `u32 mask`, values in **ascending bit order**, `u8 [u8]`, `u8 [u32 u32]` | **[L]**, decoded field for field |
| "you gained EXP" | the **same** `0x007C` with bit 16 set | `u64 exp` — the *new total*, not the delta | **[L]** |
| "you levelled up" (the local animation) | **no packet.** The `0x007C` handler plays it itself when the level in the record goes **up** | — | **[L]**, see §5 |
| "you levelled up" (what other players see) | **`0x02AF`** | `u32 charId, u8 effect`, effect **0** = LevelUp | **[L]**, see §6 |
| (there is also an explicit local effect packet) | **`0x02D1`** | `u8 effect`, effect **0** = LevelUp | **[L]**, see §6 — probably redundant, see §7 |

---

## 1. `0x007C` is the stat-change packet, and the routing is read from the binary

`research/msexe-gamestage-cases.txt` already listed `0x007c -> FUN_142d54780`, but that file
is a scrape of *decompiled text*. I re-derived it from the image.

**[L]** `FUN_142cbaa80` (`CWvsContext::OnPacket`) computes its switch like this:

```
142cbab0b  lea  eax,[r14 - 0x70]                    ; index = opcode - 0x70
142cbab0f  lea  rbx,[rip - 0x2cbab16]               ; rbx = 0x140000000
142cbab16  cmp  eax, 0x32a
142cbab1b  ja   142cbbc10                           ; -> default
142cbab23  mov  ecx,[rbx + rax*4 + 0x2cbd9d0]       ; a DENSE 4-byte-RVA table, no byte index
142cbab2a  add  rcx, rbx
142cbab2d  jmp  rcx
```

so the table is at **`0x142cbd9d0`**, 811 entries, `opcode - 0x70`.

**Enumerated, not filtered** (`CLAUDE.md`: "enumerate before you filter"). Walking all 811
entries gives **273 distinct stubs**, of which one — `0x142cbbc10` — occupies **538** slots
and is the `default:` arm. 272 real handlers, which is exactly the "273 labels / 272 bodies"
the decompiled scrape reports. **[L]**

| | |
|---|---|
| entry 12 (`opcode 0x007C`) | `-> 0x142cbab4f` `-> call 0x142d54780` |
| **the positive control** | entry 0 (`opcode 0x0070`) `-> 0x142cbab2f` `-> call 0x142d51930`, the `InventoryOperation` handler this project has *confirmed on screen* |
| aliasing check | `0x142cbab4f` is reached from **exactly one** opcode, `0x7c` |
| direct transfers into `0x142d54780` | **one `call`, no tail `jmp`**, at `0x142cbab55` inside `FUN_142cbaa80`. `tools/callers.py` alone is not enough for this question — see §3 |

So `0x007C` is `FUN_142d54780` and nothing else reaches it. **[L]**

### Independent corroboration that this really is the stat packet

`FUN_142d54780` **snapshots stat fields out of the character record before it decodes
anything**, and compares them afterwards. Seven fields are read in the prologue - `+0x1f`,
`+0x23` and `+0x33` (face, hair, job) as well as these four - and it is these four that are
compared later. **[L]**

```
142d5481c  lea  rsi,[rdi + 0x27]          ; &level      (charstat-layout.md read #10)
142d5482d  call 0x1401ba9d0               ; oldLevel   -> [rbp+0x250]
142d5485c  lea  rcx,[rdi + 0x5b]          ; &hp         (read #16)
142d54863  call 0x1401ba9d0               ; oldHp      -> [rbp-0x58]
142d5486b  lea  rcx,[rdi + 0x73]          ; &mp         (read #18)
142d54872  call 0x1401ba9d0               ; oldMp      -> r15d
142d5487a  lea  rsi,[rdi + 0x9b]          ; &exp        (read #21, the u64)
142d5488a  call 0x1401a1790               ; oldExp     -> r14   (64-bit getter)
```

Those four offsets are the ones `research/charstat-layout.md` §3 already published as
level / hp / mp / exp. A packet handler whose first act is to take a *before* picture of the
character's level, HP, MP and EXP is the stat-change packet. **[D]**

---

## 2. The head — three `u8`s before the mask

**[L]**, at the very top of `FUN_142d54780`:

```
142d547ad  call 0x1406e8ae0    ; READ u8  #1
142d547b2  test al,al
142d547b4  je   142d547c0
142d547b6  xor  edx,edx
142d547b8  mov  rcx,r14
142d547bb  call 0x142cc4430    ;   -> SetExclRequest(ctx, 0)
142d547c0  mov  rcx,r14
142d547c3  call 0x142cbe730    ; GetCharacterStat(); NULL -> bail to 142d5629a
142d547d4  call 0x1406e8ae0    ; READ u8  #2 -> [rbp-0x54]
142d547e2  call 0x1406e8ae0    ; READ u8  #3 -> [ctx+0x3bbc]
```

| # | what it does | tag |
|---|---|---|
| 1 | **non-zero clears the client's one-request-outstanding latch.** `FUN_142cc4430(ctx, v)` is `mov [rcx+0x2330], edx; [rcx+0x2334] = tick`. `FUN_142cc52a0` — one of the seven clearers `research/npc-click.md` §4.3 names — is byte-for-byte the same shape with a literal `0`. So this is `bExclRequestSent`, and `1` **unlocks**, `0` leaves the latch alone. | **[L]** shape, **[D]** name |
| 2 | stored in a local and used **once**, at `142d549be`: with `0` the code tests `mask & 0x40030` (level, job, exp, meso); with non-zero it tests `mask & 0x30` (level, job) only. It selects which mask bits trigger one UI refresh. v214 sends `false`. | **[L]** the two tests, **[I]** the name |
| 3 | stored at `[ctx+0x3bbc]`. `tools/xref.py --field 0x3bbc --size dword` finds **10 writers**; three of them write the literal `1`, and one of those is **`FUN_142caa4e0`**, the field-entry routine. v214 sends `1`. | **[L]** destination, **[I]** meaning |

> **An instrument correction that belongs in `research/npc-click.md`, not here.** That file
> says `player->[0x2330]` has "only 7" clearers, found by a byte scan. **`FUN_142cc4430` is
> an eighth**, and the scan could not see it because the value comes from a *register*, not
> an immediate — the same "wrong shape" failure `CLAUDE.md` warns about. It has **241 call
> sites**, so the real clearer count is much larger than seven. I did not re-derive that
> file's conclusions; I am flagging its instrument. **[L/D]**

---

## 3. The mask and the values — `FUN_1402cbb50`

`142d5489d  call 0x1402cbb50` with `rcx` = the character-stat record, `rdx` = the packet.
It **returns the mask in `eax`**, which the handler keeps in `edi` for the rest of the
function. **[L]**

### Who else reaches the mask decoder — and a correction I had to make to my own draft

`tools/callers.py 0x1402cbb50` reports **1 call site**, this one, and I nearly wrote "it is
not shared with anything". That is **wrong**, and the tool is why: `callers.py` scans for
`0xE8` (`call rel32`) only. A **tail `jmp`** into a function is invisible to it — the same
gap `tools/reads.py`'s own docstring documents as failure mode 3, and the subject of an
already-open task in this repo.

Re-scanning `.text` for **both** `E8` and `E9 rel32` against the address: **[L]**

| target | `call` | `jmp` |
|---|---:|---:|
| `FUN_142d54780` (the `0x007C` handler) | **1** | **0** |
| `FUN_1402cbb50` (the mask decoder) | 1 | **1**, at `0x14104a8b0` |
| `FUN_1427863f0` (the effect handler) | 2 | 0 |

`0x14104a8b0` is the last instruction of `FUN_14104a890`, a five-instruction wrapper —
`GetCharacterStat()` off a global, then `jmp FUN_1402cbb50` with the caller's packet in
`rdx`. Its one caller is `FUN_141072ec0`, a 992-byte routine whose **first read is a `str`**
and which then fans out into a dozen unrelated decoders. That is the shape of a
debug/replay harness, not a packet handler, and it has no direct callers of its own. **[D]**

**So `0x007C` remains the only opcode in `FUN_142cbaa80`'s table that reaches the mask
decoder**, but the reason is the jump-table enumeration in §1, not a call count.

> **The E8+E9 scan has its own blind spot, and the control proves it.** Run against
> `FUN_140304100` — the equipped-item decoder that
> `research/fixtures/…mob-body-faults-client-hook.log` shows firing **four times** on a real
> run — it reports **zero** `call`s and **zero** `jmp`s, because that function is reached
> through `vtable+0x358`. **Neither `callers.py` nor this scan can see an indirect call.**
> Every count in the table above is a count of *direct* transfers and nothing more. **[L]**

`tools/reads.py 0x1402cbb50 3` reports **7 contiguous `.pdata` entries merged**,
`0x1402cbb50..0x1402cbf71`, 1057 bytes, **24 read sites**. `tools/listing.py` on the same
address agrees address-for-address — the two instruments share a loader and an extent, so
they cannot disagree, which is why I also checked both against `reads.py`'s own documented
positive control (`0x140304100`, the equipped-item decoder) before trusting either.

### The mask is a `u32`, and that is a real divergence from the reference

```
1402cbb6e  48 8b ca              MOV  RCX,RDX
1402cbb71  e8 aa d0 41 00        CALL 0x1406e8c20      ; the u32 reader
1402cbb76  8b e8                 MOV  EBP,EAX
1402cbb78  a8 01                 TEST AL,1             ; bit 0
```

**[L]** `0x1406e8c20` is the `u32` primitive (`tools/reads.py` `PRIM`). v214's
`WvsContext.statChanged` writes `encodeLong(mask)` — **8 bytes**. mscw reads **4**. Sending
eight would push every value four bytes late and desynchronise the whole body, and there is
no length prefix and no resync point inside it.

### Every bit test in the function, enumerated in address order

Not "the bits I expected", *every* conditional bit test between the mask read and the `ret`:

| # | test | at | bit |
|---|---|---|---|
| 1 | `test al,1` | `1402cbb78` | 0 |
| 2 | `test bpl,2` | `1402cbb92` | 1 |
| 3 | `test bpl,4` | `1402cbba3` | 2 |
| 4 | `test bpl,0x10` | `1402cbbb9` | 4 |
| 5 | `test bpl,0x20` | `1402cbbef` | 5 |
| 6 | `test bpl,0x40` | `1402cbc1b` | 6 |
| 7 | `test bpl,bpl / jns` | `1402cbc38` | 7 |
| 8-18 | `bt ebp, 8 .. 0x12` | `1402cbc54` … `1402cbf36` | 8 … 18 |

**There is no test for bit 3, and none for any bit above 18.** The instruction after the
bit-18 block is the stack-cookie check and `ret`. So bits 3 and 19..31 are *never examined*
and their values are *never read* — setting one of them in the mask changes nothing and
sending a value for one desynchronises the body. **[L]**

### The layout, in wire order

`param_1` is the character-stat record; the destinations are the same offsets
`research/charstat-layout.md` §3 established for the `SetField` block, which is what names
most of these fields.

| bit | mask | reads, in order | destination | name | tag |
|---:|---:|---|---|---|---|
| 0 | `0x00001` | `u8`, `u32` | `+0x1a`, `+0x1b` | skin (+ the always-zero `u32` that follows it in the record) | **[L]** offsets, **[I]** name |
| 1 | `0x00002` | `u32` | `+0x1f` | face | **[L]** / **[I]** |
| 2 | `0x00004` | `u32` | `+0x23` | hair | **[L]** / **[I]** |
| **3** | `0x00008` | **nothing — no test exists** | — | — | **[L]** |
| 4 | `0x00010` | `u32` | obf `+0x27` | **level** | **[L]**, see §5 |
| 5 | `0x00020` | `u16`, `u16` | obf `+0x33`, plain `+0x10c` | **job**, then **subJob** | **[L]**, and `+0x33` is proved by the SP fork |
| 6 | `0x00040` | `u16` | obf `+0x3b` | str | **[L]** offset, **[I]** which of the four |
| 7 | `0x00080` | `u16` | obf `+0x43` | dex | same |
| 8 | `0x00100` | `u16` | obf `+0x4b` | int | same |
| 9 | `0x00200` | `u16` | obf `+0x53` | luk | same |
| 10 | `0x00400` | `u32` | obf `+0x5b` | **hp** | **[L]** offset, **[D]** name — see below |
| 11 | `0x00800` | `u32` | obf `+0x67` | maxHp | **[L]** / **[I]** |
| 12 | `0x01000` | `u32` | obf `+0x73` | **mp** | **[L]** / **[D]** |
| 13 | `0x02000` | `u32` | obf `+0x7f` | maxMp | **[L]** / **[I]** |
| 14 | `0x04000` | `u16` | obf `+0x8b` | ap | **[L]** / **[I]** |
| 15 | `0x08000` | **the SP fork**, see §4 | `+0x93` or `+0xd7` | sp | **[L]** |
| 16 | `0x10000` | `u64` | obf `+0x9b` | **exp** | **[L]**, and proved by the consumer in §5 |
| 17 | `0x20000` | `u32` | obf `+0xb3` | fame | **[L]** / **[I]** |
| 18 | `0x40000` | `u64` | obf `+0xbf` | **meso** | **[L]** width/offset, **[I]** name |

"obf `+X`" means the value goes through `FUN_1407386b0(&DAT_143ac1ab0)` /
`FUN_1402f7010` / `FUN_1402f7170` into the three-word `(key, value, checksum)` triple at
`+X`, `+X+4`, `+X+8` — the same ZtlSecure scheme `charstat-layout.md` documents. It is
entirely client-side; nothing about it changes what goes on the wire.

**Why `+0x5b` = hp and `+0x73` = mp is [D] rather than [I].** At `142d548ef..142d54911` the
handler calls `FUN_140fd31f0(uiGlobal, hpRecovery, mpRecovery, oldHp, oldMp)` — arg2 and
arg3 are the two `u32`s the *recovery* tail just read, arg4 is the snapshot of `+0x5b` and
arg5 the snapshot of `+0x73`. The pairing (`hpRecovery` beside `+0x5b`, `mpRecovery` beside
`+0x73`) is what makes it hp and mp rather than maxHp and maxMp. It is still an argument
about argument order, so it is **[D]** and not **[L]**. `charrecord-reuse.md` **D5** ("which
obfuscated `u32` is which of hp/maxHp/mp/maxMp") is *narrowed* by this, not closed.

**Meso is not in the `SetField` stat block.** `+0xbf` appears nowhere in
`charstat-layout.md` §3's 29 reads. So `0x007C` bit 18 is currently the **only** way this
server can put a meso balance into the client — the character record cannot carry one.
**[L]** for the absence (the §3 table is a full enumeration of that function's reads),
**[I]** for the name `meso`.

### The tail, after the mask block returns

```
142d548aa  READ u8            ; A
142d548b1  je 142d548cf       ;   if A == 0, skip
142d548b6  READ u8            ;   B      -> FUN_1428a7f00(global, B)   (gated on a non-null global)
142d548d2  READ u8            ; C
142d548d9  je 142d54919       ;   if C == 0, skip
142d548de  READ u32           ;   hpRecovery
142d548e8  READ u32           ;   mpRecovery
```

**[L]** Two optional trailers. v214 calls them `charmOld` and `updateCovery` and encodes
them in exactly this shape — `u8 flag; if flag u8 value;` then `u8 flag; if flag u32 u32;`
— which is corroboration of the *shape*, not of the names. **Sending `0, 0` is always
valid** and is what every packet this project needs will do.

### The whole body, as a picture

```
u8   bExclRequestSent      1 = clear the client's outstanding-request latch
u8   ?                     0   (selects which mask bits refresh one UI element)
u8   ?                     1   (stored at ctx+0x3bbc; field entry also writes 1)
u32  mask                  bits 0..18 only; bit 3 is not decoded
     <values, ascending bit order, widths from the table above>
u8   hasCharm              0
     [u8 charm]
u8   hasRecovery           0
     [u32 hpRecovery] [u32 mpRecovery]
```

---

## 4. The SP fork, and the trap in it

Bit 15 does **not** simply read a `u16`. **[L]**

```
1402cbd8f  bt   ebp,0xf
1402cbd93  jae  1402cbecf                 ; SP bit clear -> skip
1402cbd99  mov  edx,[rbx + 0x37]
1402cbd9c  lea  rcx,[rbx + 0x33]          ; the JOB, read back OUT OF THE RECORD
1402cbda0  call 0x1401ab420               ; the obfuscated getter
1402cbda5  movsx ecx,ax
1402cbda8  call 0x1403024c0               ; uses_extended_sp(job)
1402cbdad  test eax,eax
1402cbdaf  je   1402cbeb2                 ; 0 -> the plain branch
   extended:  READ u8 count, then count x (READ u8 jobLevel, READ u32 amount)
              each pushed into the list at record+0xd7; the amounts summed into +0xef
   plain:     1402cbeb5  READ u16  -> obf +0x93
```

**`FUN_1403024c0` is exactly `net::opcode::uses_extended_sp`.** I decoded its bytes and
compared: `job == 0`; `job-100` and `job-200` bit-tested against the 64-bit mask
`0x1c0701c01` (bits 0,10,11,12,20,21,22,30,31,32); `job-300`, `job-400`, `job-500`
bit-tested against `0x701c01` (bits 0,10,11,12,20,21,22); plus `430 <= job <= 439`. Same
masks, same ranges, same answer for every input. **[L]** The Rust function can be reused
verbatim.

> ### The trap
>
> The fork reads the job **out of the record**, and bit 5 has already written it **earlier in
> the same packet**. So a `0x007C` that carries *both* job and sp must encode the sp in the
> form that matches the **new** job, not the old one. This is exactly the shape of bug that
> desynchronises a body with no resync point. **[L+D]**

---

## 5. The level-up animation is **client-side**, and this is measured

`FUN_142d54780`, after the mask block returns: **[L]**

```
142d54ae2  mov  edx,[r13 + 0x2f]
142d54ae6  mov  rcx,r15                   ; r15 = record + 0x27  (the level)
142d54ae9  call 0x1401ba9d0               ; newLevel
142d54aee  cmp  eax,[rbp + 0x250]         ; vs the snapshot taken BEFORE the decode
142d54af4  jle  142d55b52                 ; not higher -> nothing happens
142d54afa  cmp  qword [rip + 0xd53a16],0  ; a UI global must exist
142d54b02  je   142d55b52
142d54b08  test r12d,r12d                 ; r12d = FUN_142cf1c20(ctx), a "busy UI" test
142d54b0b  jne  142d551b2                 ; -> an alternate presentation path
...
142d54bdb  mov  rdx, qword [rip + 0xcf2366]     ; = [0x143a46f48]
142d54be6  call 0x140dc12e0
```

**[L] `[0x143a46f48]` holds `0x1432ae4a0`, which is the UTF-16 string
`Effect/BasicEff.img/LevelUp`.** I dereferenced it out of the PE. The three qwords after it
are `Effect/BasicEff.img/JobChanged`, `.../QuestClear`, `.../CraftingLevelUp`, and further
along `.../CitizenshipGet` and `.../CitizenshipGradeUp` — Classic World's own effects.

`tools/dataref.py 0x143a46f48` gives **two** readers: `0x142d54bdb` (this one) and
`0x14278bdc8` (the packet-driven effect handler, §6). That is the whole set.

> **[L+D] The client plays the level-up animation by itself, from inside the `0x007C`
> handler, whenever the level in the character-stat record goes UP.** No effect packet is
> involved. The owner's assumption in `STATUS.md` goal D ("the client owns the animation") is
> now evidence rather than assumption — for the *local* player.

### The EXP indicator is client-side too, and it names read #21 as `exp`

```
142d5491f  bt   edi,0x10                  ; mask bit 16
142d54923  jae  142d549a4
142d54925  cmp  qword [rip+0xd7499c],rax  ; the indicator object must exist
142d5492e  ...
142d54938  call 0x1401ba9d0               ; newLevel
142d54946  cmp  ebx,eax                   ; oldLevel == newLevel ?
142d54948  jne  142d54961
142d54954  call 0x1401a1790               ; newExp  (64-bit getter on record+0x9b)
142d5495c  sub  rdi,r14                   ; delta = newExp - oldExp
142d5495f  jmp  142d54990
142d54961: ... newExp again, then
142d54985  call 0x14087ec50               ; expNeededForLevel(oldLevel)
142d5498a  sub  rdi,r14
142d5498d  add  rdi,rax                   ; delta = newExp - oldExp + needed(oldLevel)
142d54990  mov  rdx,rdi
142d5499a  call 0x140fd3110               ; NOT "show +N EXP" - see the note below
```

> **CORRECTED 2026-08-22: `0x140fd3110` does not show anything.** It is **106 bytes** and its
> whole body is `totalExpGained += delta` on the object at `0x143AC92C8` - confirmed with
> `tools/dataref.py` rather than by reading the arithmetic. That is the same accumulator
> object `FUN_140fd31f0` writes its hp/mp recovery totals into, and that function turned out
> to be a **statistics counter** too: running totals, effective-versus-wasted healing, per-hour
> averages, an hour-boundary reset, and no renderer anywhere on the path.
>
> This matters beyond a label. §5 of this document uses that call as evidence that *"the EXP
> indicator is client-side too"* - and that conclusion now rests on a counter rather than on a
> drawing call. The indicator may still be client-side; nothing here shows it.
>
> The **drawing** call for a floating number is `FUN_142771360(pUser, N, ...)`, which forks on
> the sign of `N` to choose digit set 2 (`NoBlue`) or 3 (`NoViolet`). `research/recovery-number.md`.

**[D]** A handler that computes `newExp - oldExp` and, when the level changed, adds the EXP
required for the old level, is computing "how much EXP did you just gain". The field it
computes it from is `record+0x9b` — mask bit 16. **That closes the naming of bit 16 as
`exp` from a consumer, not from the reference.**

**The server never sends an EXP delta and never sends an EXP message.** It sends the new
total; the client subtracts.

---

## 6. The effect packets — `0x02D1` local, `0x02AF` remote

Both reach the same handler, `FUN_1427863f0`, whose **first read is a `u8` effect type**
(`14278644e`). Routing, all **[L]**:

```
CField::OnPacket FUN_141820080     0x224..0x39F  ->  FUN_1429b9300   (the user pool)
  FUN_1429b9300:
     0x224, 0x225                     inline
     0x226..0x292                  -> FUN_1429bafb0
     0x293..0x2C4                  -> FUN_1429bb720      REMOTE: reads u32 charId first
     0x2C5..0x39E                  -> FUN_14289a3a0(ctx->localUser, op, pkt)   LOCAL
```

* `FUN_14289a3a0`: `index = opcode - 0x2c5`, table `0x14289d660`. **Index `0xC` → the stub
  at `0x14289a439` → `call 0x1427863f0`.** `0x2c5 + 0xc = ` **`0x02D1`**.
* `FUN_1429bb720`: reads **`u32 charId`** at `1429bb745`, looks the user up in the pool's
  hash map, then `index = opcode - 0x29e`, table `0x1429bbc34`. **Index `0x11` → the stub at
  `0x1429bba36` → `call 0x1427863f0`.** `0x29e + 0x11 = ` **`0x02AF`**.
  `tools/reads.py 0x1429bb720 1` shows **exactly one direct read** before the dispatch, so
  the `u32 charId` is the whole prefix.

### Which effect id is LevelUp

`FUN_1427863f0` has two switches on the effect byte. The second, at `14278bd8d`, is
`ecx = [0x142791348 + effectType*4]` for `effectType <= 0x54`. Enumerating that table:
**index `0` → `0x14278bd99`**, and `0x14278bd99..0x14278bedf` is the arm that reads
`[0x143a46f48]` = `Effect/BasicEff.img/LevelUp` and ends `jmp 0x14279102e` (the common
exit). **[L]**

**That arm contains no packet read.** `tools/reads.py 0x1427863f0 3` lists 239 read sites;
the nearest either side of the arm are `0x14278ba65` and `0x14278bf3c`, both outside it.
**[L]**

So:

```
0x02D1   u8 effect                      effect 0 = LevelUp     -> 1-byte body
0x02AF   u32 charId, u8 effect          effect 0 = LevelUp     -> 5-byte body
```

`0x02AF` bails silently if `charId` is not a user currently in the pool.

---

## 7. What I did NOT establish

This section is the point of the document. Read it before building on anything above.

1. **Nothing here has been on the wire.** Not one byte. Every claim is static analysis over
   the PE; no client run was made and none is scheduled by me. `0x007C` has never been sent
   by this server.

2. **Whether `0x02D1` is redundant or additive.** The `0x007C` handler plays the LevelUp
   effect itself (§5) and `0x02D1` effect 0 plays the same effect. I did **not** establish
   whether sending both plays it twice, plays it once, or interacts badly. **Send `0x007C`
   alone first.** If the animation does not appear, `0x02D1` is the next single variant to
   try — one at a time, per `CLAUDE.md`.

3. **The two unnamed head bytes.** Byte 2's *effect* is read (`142d549be` picks
   `mask & 0x40030` vs `mask & 0x30`) but I did not chase what that UI refresh is. Byte 3's
   destination `[ctx+0x3bbc]` is read but I did not find a single *reader* of it — only ten
   writers. Both values are taken from v214 (`0` and `1`) and are **[I]**.

4. **str/dex/int/luk, maxHp/maxMp, ap, fame, meso are named from the reference.** What is
   measured is the *order* and the *width*; the record offsets they land in are the same
   offsets `charstat-layout.md` assigns those names, and that file marks several of them
   `[I]` too. If str and dex were swapped in both documents, nothing here would catch it.
   The cheap on-screen test: send a `0x007C` that sets **one** stat to a distinctive value
   and read the stat window.

5. **The extended-SP pool's `u8` key.** The extended branch reads `count x (u8, u32)` and
   sums the `u32`s. `charstat-layout.md` calls the `u8` a "job level". **I did not establish
   what value the client expects there**, and it matters the moment the server actually
   *awards* SP to a beginner (job 0 takes the extended branch). The empty table — `u8 0`,
   nothing else — is the only extended encoding this project has ever sent, and the
   character record already sends it, so it is the safe one.

6. **The EXP-to-next-level curve cannot be read out of the PE.** `FUN_14087ec50(level)` is
   `if (level > 120) return INT64_MAX; return qword[0x143AC2400 + clamp(level,1,120)*8]`.
   **[L]** But `0x143AC2400` is at RVA `0x3ac2400`, which is `0x81400` into `.data` — and
   `.data` has `vsize 0xa2aa8` but **`rsize 0x67400`**. The table is in the *uninitialised
   tail*: zero on disk, filled at run time. **[L]**

   > **An instrument trap, recorded because it produced a confident wrong answer for ten
   > minutes.** A naive `foff()` that maps VA→file offset using `max(vsize, rsize)` walks
   > straight past the end of `.data`'s raw bytes and starts reading **`.pdata`**. It
   > returned 120 plausible-looking 8-byte "EXP values" that were in fact
   > `RUNTIME_FUNCTION` triples. The tell was the period-3 structure. **Any VA→file-offset
   > helper used on `.data` in this image must clamp to `rsize` and report BSS as absent.**
   > The same helper is correct on `.text` and `.rdata` (both have `rsize >= vsize`), which
   > is why it has never bitten before.

   Leads for whoever wants the curve: `tools/dataref.py 0x143ac2400` gives 7 references, of
   which `0x14087eca6`, `0x14087ed71` and `0x1408813b0` are consumers and `0x14003aeb8` sits
   outside any `.pdata` function. Or read it out of the *running* process with the hook —
   that is one `-Probe` peek at `0x143AC2400`, far cheaper than static work.

7. **HP/MP per level.** Not looked at. It is the number `STATUS.md` goal D specifically warns
   about taking from the reference server.

8. **`0x02AF`'s other effect ids** and the first switch (`effectType - 8`, table
   `0x142791300`) were not enumerated. Only effect `0` was established.

9. **Bit 3 and bits 19..31 are dead in this client.** That is a *verified* negative — the
   enumeration in §3 lists every conditional bit test in the function, in address order,
   and the positive controls are the eighteen tests it did find. But I did not check whether
   some *other* opcode carries `fatigue`, `charismaEXP` and the rest that v214 puts in those
   bits.

---

## 8. Wire it like this

`crates/world/src/session/` belongs to the coordinator; I did not touch it.

**1. `crates/net/src/lib.rs`** — one line, already added:

```rust
pub mod stats;
```

**2. EXP from a mob kill** (the thing goal D is blocked on):

```rust
use net::stats::{self, StatChange};

// the character's NEW total, not the delta - the client subtracts (see section 5)
chr.exp += award;
vec![Reply {
    opcode: stats::STAT_CHANGED,
    body: StatChange::exp(chr.exp).build(),
    what: format!("StatChanged: exp -> {}", chr.exp),
}]
```

**3. A level-up.** One packet, and the animation comes free:

```rust
let mut sc = StatChange::new();
sc.level   = Some(new_level);
sc.exp     = Some(carry_over_exp);
sc.hp      = Some(chr.hp);
sc.max_hp  = Some(chr.max_hp);
sc.mp      = Some(chr.mp);
sc.max_mp  = Some(chr.max_mp);
sc.ap      = Some(chr.ap);
// sp: see research/level-up.md section 7 item 5 before awarding SP to a beginner
let body = sc.build();
```

**Send `0x007C` and nothing else on the first run.** The animation is client-side; adding
`0x02D1` at the same time makes two variants out of one measurement.

**4. The owner's award rule** is world policy, not net. It wants to live beside
`world::config::spawn_capacity` so the one ambiguity is in one named place:

```rust
/// Skill points awarded for REACHING `new_level`.
///
/// The owner, 2026-08-19: "5 ability points and 3 skill points ... if they are under level 10,
/// they only receive 1 skill point per level". STATUS.md reads "under level 10" as the level
/// being REACHED, so 9 -> 10 gives 3. Flip the comparison here if that is wrong. **[I]**
pub const AP_PER_LEVEL: u16 = 5;
pub fn sp_for_level(new_level: u32) -> u32 { if new_level < 10 { 1 } else { 3 } }
```

**5. Other players' level-ups** — `stats::user_effect_remote(char_id, stats::EFFECT_LEVEL_UP)`
to every *other* client on the field. Nothing on this server has a second client yet, so
this is untestable today and should not be wired before something can observe it.

**6. Do not send a `0x007C` with the `sp` bit and the `job` bit in ways that disagree** —
§4. `stats::Sp::matches_job` exists to make that a compile-time-ish check.

---

## 9. Reproducing every number in this document

The five throwaway scripts I used are gone (they lived in `tools/` and I removed them so the
shared directory stays as its owner left it). Each is small enough to reproduce inline. All
of them must be run **from the repo root**, per `CLAUDE.md` § "The scratchpad shadows the
real tools".

```python
# VA -> file offset, CLAMPED TO rsize. This is the version that does not lie about .data.
import sys; sys.path.insert(0, "tools")
from rtti import load_pe
data, base, sections = load_pe("client-patched/MapleStory.exe")
def foff(va):
    rva = va - base
    for s in sections:
        if s["vaddr"] <= rva < s["vaddr"] + s["vsize"]:
            off = rva - s["vaddr"]
            if off >= s["rsize"]:
                raise ValueError("%#x is in %s's BSS tail - zero on disk" % (va, s["name"]))
            return s["raddr"] + off
    raise ValueError("%#x is in no section" % va)
```

```python
# the whole 0x70-based jump table of FUN_142cbaa80, enumerated
import struct
TABLE = 0x142CBD9D0
for i in range(0x32A + 1):
    print(hex(0x70 + i), hex(base + struct.unpack_from("<I", data, foff(TABLE + i*4))[0]))
```

```python
# every DIRECT transfer into a function - call AND tail jmp. tools/callers.py sees E8 only.
# It cannot see an indirect call: run it on 0x140304100 and it reports zero, for a function
# a real client run shows executing four times through vtable+0x358.
import struct
def direct_refs(target):
    out = {0xE8: [], 0xE9: []}
    for s in sections:
        if not (s["chars"] & 0x20000000) or not s["rsize"]:
            continue
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        secbase = base + s["vaddr"]
        for i in range(len(blob) - 5):
            if blob[i] in out:
                rel = struct.unpack_from("<i", blob, i + 1)[0]
                if secbase + i + 5 + rel == target:
                    out[blob[i]].append(secbase + i)
    return out
```

The rest are repo tooling, unchanged:

```
python tools/reads.py    0x140304100 2          # the documented positive control - run this FIRST
python tools/listing.py  0x140304100 | grep READ  # must agree with it address-for-address
python tools/reads.py    0x142d54780 3          # 0x007C: 9 direct reads + FUN_1402cbb50
python tools/listing.py  0x1402cbb50            # the mask decoder, all 18 bit tests
python tools/callers.py  0x142d54780            # 1 call site, in FUN_142cbaa80
python tools/dataref.py  0x143a46f48            # the two readers of the LevelUp effect path
python tools/xref.py --va 0x143a49020           # positive control for the pointer chase:
                                                #   1 reference, the published PTR_s_mapName
```
