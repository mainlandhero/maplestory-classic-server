# The cash shop STAGE: `0x01A3` in, `0x00D1` out, and no migrate needed

2026-08-24. Static only — **no client run was spent on this.** `research/cash-shop.md` ends at
part seven with the client sending `0x00D5`, being told "not available", and the question
*"what do we answer it with?"* open. This file answers that.

Tags: **[L]** read out of a file, **[D]** derived from something read, **[I]** inferred.

---

## 0. The short version

| | | |
|---|---|---|
| enters the cash shop | **`0x01A3`** | **[L]** — read out of the stage dispatcher, not aligned against a reference |
| its handler | `FUN_14209AD60` | [L] |
| its stage class | ctor `FUN_140D71CB0`, OnPacket `FUN_140D734E0` | [L] |
| **no migrate is required** | the *field* stage chains `0x1A0..0x1A3` to the base dispatcher at `0x141821FEE` | [L] |
| it also clears `[ctx+0x2330]` | `FUN_142CBE8F0`, called by all four stage handlers, writes 0 at `0x142CBE918` | [L] |
| the client then sends | `0x03E0` (query cash), `0x03E1` (everything else, `u8` sub-op), `0x00D1` (leave) | [L] |
| the client then accepts | `0x05AD`, `0x05AE`, `0x05B9`, `0x05BA` — **only these four** | [L] |
| leaving | client sends **`0x00D1`**, empty body, and drops to a null stage; answer with `0x01A0 SetField` | [L] for the send, [I] for the answer |
| the wallet | `0x05AD`: `u32 nxCredit, u32 maplePoint, u32 <read, range-checked, discarded>` | [L] |

**`research/msexe-gamestage-opcodes.md`'s candidate range `0x01A1..0x01AA` is closed.** The
block is exactly four slots wide — `0x01A0..0x01A3` — because that is the width of the `cmp`
in the base dispatcher, and each slot has a named handler. `SetField = 0x01A0` was already
confirmed on a live client (`research/msexe-stage-setfield.md`); the other three are now read
out of the same ladder.

**Two instrument defects were found on the way and are reported in §10.** One of them —
an eleventh packet-read primitive `tools/reads.py` does not know about, with **71 call sites
in 35 functions** — is not specific to the cash shop and affects existing conclusions.

---

## 1. The instruments, and their controls

Every tool below was run against its own documented positive control **first**, in the repo
working directory, before any negative result here was believed.

| instrument | control | result |
|---|---|---|
| `tools/reads.py` | `0x140304100 2` must show reads through `FUN_1403035a0` *and* `FUN_140303b40` *and* directly | passed — `0x14030411a`, `0x140304126`, then `0x140304138 raw` |
| `tools/listing.py` | `0x140304100 \| grep READ` must match `reads.py`'s addresses | passed — `140304138 raw`, `140304144 u8`, `140304183 u8`, then the `u16` run |
| `tools/encodes.py` | `0x141cb6880 1` must show CTOR at `141cb7eb1` and SEND at `141cb8365` | passed, both |
| `tools/callers.py` | `0x1402fa9a0` must give 96 call sites | passed — **96 sites in 15 functions** |
| `tools/dataref.py` | `0x143aa84a0` must give 5459 references | passed — **5459** |
| `tools/dump_stringids.py` | id 2334 must decrypt to *"You cannot go into the cash shop…"* | passed, exactly |
| scratch `qscan.py` (qwords in data) | `0x141820080` must appear in 21 slots and `0x141b25f30` in 1, at `0x1433fd7a0` | passed — 21 and 1, same address as `research/npc-spawn.md` |
| scratch `fnstrings.py` | `0x1411ab7b0` must list the status-bar button names incl. `CashShop` and `Inven` | passed — 18 of the 19; see the blind spot below |

Blind spots that bit, named because the negatives below rest on them:

* **`fnstrings.py` drops strings shorter than 4 characters.** The control printed 18 of
  `research/cash-shop.md`'s 19 button names; the missing one is `Key`. Any 1–3 character
  literal is invisible to it.
* **`tools/xref.py` and `tools/dataref.py` both return 0 for `UI/CashShopUI.img`** at every
  8-byte offset of the literal. That is not evidence nothing uses it: `xref.py` matches
  `lea`/`mov imm64`, `dataref.py` matches a list of `mov`-family opcodes, and **neither list
  contains `0F 10`/`0F 11` (`movups`)**, which is how MSVC inlines a 34-byte UTF-16 literal.
  The identification in §3 therefore does **not** rest on a string xref.
* **The Etc WZ cannot be grepped.** `Data/Etc/Etc_000.wz` begins
  `"Package file v1.0 Copyright 2002 Wizet, ZMS"` and holds no plaintext node names — an
  ASCII grep for `Commodity` returns 0 and is structurally incapable of returning anything
  else. `gm-handbook/commodity.txt` (179 rows) is the instrument that can see it.

Ghidra was not used; the project holds a single-process lock on it.

---

## 2. The stage dispatcher, read byte for byte

`FUN_142097EE0` is the base `CStage::OnPacket`. It has **no `.pdata` entry of its own** —
`tools/pdata_lookup.py` reports "falls in no function (nearest ends `0x142097EB8`)" — which
is why `tools/listing.py` refuses it and why nobody had disassembled it. Read directly out
of the image: [L]

```asm
142097ee0  sub  edx, 0x1a0
142097ee6  je   142097f1b        ; 0x1A0
142097ee8  sub  edx, 1
142097eeb  je   142097f0f        ; 0x1A1
142097eed  sub  edx, 1
142097ef0  je   142097f03        ; 0x1A2
142097ef2  cmp  edx, 1
142097ef5  jne  142097f27        ; anything else -> ret
142097ef7  add  rcx, -0x18       ; 0x1A3
142097efb  mov  rdx, r8
142097efe  jmp  14209ad60
142097f03  add  rcx, -0x18 ; mov rdx,r8 ; jmp 14209b380     ; 0x1A2
142097f0f  add  rcx, -0x18 ; mov rdx,r8 ; jmp 14209b070     ; 0x1A1
142097f1b  add  rcx, -0x18 ; mov rdx,r8 ; jmp 142097f80     ; 0x1A0  = SetField
142097f27  ret
```

Four opcodes, four handlers, nothing else. The `add rcx,-0x18` is the this-adjust: the
stage's `OnPacket` lives in a **one-entry vftable at object offset `+0x18`**, which is why
`qscan` finds `FUN_142097EE0` in exactly four `.rdata` slots (`0x1433750A0`, `0x1433F84D0`,
`0x143400F70`, `0x14342BD28`) — four stage classes that do not override it. [L]

The four handlers are structurally identical: read an 8-byte clock base, decode the full
character record, switch the client to a mode, allocate a stage object, run its constructor
over the *same* packet, and install it with `CStage::SetStage` (`FUN_14209EE50`).

| opcode | handler | `FUN_1415AAFA0(mode)` | object size | constructor | its OnPacket | opcodes that stage accepts |
|---|---|---|---|---|---|---|
| `0x01A0` | `FUN_142097F80` | 3, or 7 | (field) | — | `FUN_141820080` | `0x1A4..0x5F3`, the whole field protocol |
| `0x01A1` | `FUN_14209B070` | **5** | `0x70` | `FUN_140F33B70` | `FUN_140F34440` | `0x5C0`, `0x5C1` |
| `0x01A2` | `FUN_14209B380` | **6** | `0x68` | `FUN_1411DAF30` | `FUN_1411DB710` | `0x5C2`, `0x5C3` |
| **`0x01A3`** | **`FUN_14209AD60`** | **4** | `0x1A0` | **`FUN_140D71CB0`** | **`FUN_140D734E0`** | **`0x5AD`, `0x5AE`, `0x5B9`, `0x5BA`** |

All [L]; the mode argument is the immediate at `0x14209B168` / `0x14209B46D` / `0x14209AE55`
and `0x142098B4D` / `0x14209AB79`.

`0x01A1` and `0x01A2` are the **auction house**, not the cash shop: the `0x01A1` class's
methods reference `UI/UIAuction.img/main/backgrnd` and the `0x01A2` class's `.rdata`
neighbourhood holds `startBid`, `/bid`, `/rebid`. [L]

### 2.1 The field stage chains to it — so there is no migrate

```asm
141821fdc  lea  eax, [r9 - 0x1a0]
141821fe3  cmp  eax, 3
141821fe6  ja   141821ff8
141821fe8  mov  edx, r9d ; mov rcx, rsi
141821fee  call 142097ee0
```

`FUN_141820080` is `CField::OnPacket`. **While the player is standing in a map, `0x01A3` sent
down the ordinary channel socket enters the cash shop.** [L] The login stage does the same
thing from the other side (`0x141B26567`), which is how `SetField` was confirmed on a live
client. Nothing on this path closes a socket, mints a seed, or looks at an address.

The owner's original architecture guess — *"transitioning to the cash shop is most likely similar
to transitioning to another channel, we probably need a dedicated cash shop server"* — is
**not what this client requires**. A dedicated server is a deployment choice, not a protocol
one. [D]

---

## 3. Why `0x01A3` is the cash shop

Four independent readings, in decreasing strength.

**(a) Its OnPacket's own opcode arms. [L]** `FUN_140D734E0`'s `0x5AD` arm reads three `u32`s,
range-checks all three, and hands the first two to two dedicated setters. Those two setters,
`FUN_140D85BC0` and `FUN_140D85B70`, have **four and two call sites in the whole image**
(`tools/callers.py`, control passed) and every one of them is inside this class. That is a
wallet.

**(b) Its `.rdata` literal pool. [D]** The class's four vftables occupy
`0x14336DDE0..0x14336DF18`. Immediately **before** them:

```text
14336dda0  55 00 49 00 2f 00 43 00 61 00 73 00 68 00 53 00   U.I./.C.a.s.h.S.
14336ddb0  68 00 6f 00 70 00 55 00 49 00 2e 00 69 00 6d 00   h.o.p.U.I...i.m.
14336ddc0  67 00 00 00 00 00 00 00 2f 00 63 00 61 00 6d 00   g......./.c.a.m.
14336ddd0  65 00 72 00 61 00 50 00 6f 00 73 00 00 00 00 00   e.r.a.P.o.s.....
14336dde0  <vftable 1>  ...
```

and immediately **after** them:

```text
14336df18  6e 65 77 00 73 61 6c 65 00 ...                    new, sale
14336df20  78 67 61 00 68 64 00 ...                          xga, hd
14336df30  63 6f 6d 6d 6f 64 69 74 79 53 4e 00               commoditySN
14336df40  45 74 63 2f 43 61 73 68 53 68 6f 70 43 61 74 65   Etc/CashShopCate
14336df50  67 6f 72 79 2e 69 6d 67 00                        gory.img
14336df60  43 61 73 68 50 75 72 63 68 61 73 65 64 00         CashPurchased
14336df70  63 68 65 63 6b 00 00 00 40 2d 2d 00               check, @--
```

Literal-pool adjacency is a translation-unit fact, not a use fact, hence **[D]**. It is
corroborated by (d).

**(c) The `@--` literal is actually used, and it is a cash-shop rule. [L]** In the `0x5AD`
arm, `0x140D73687 lea r8,[rip+0x25fa8ea]` resolves to `0x14336DF78` = `"@--"`, and the four
bytes at `[this+0x68]` (the account name, fetched by `FUN_142CB6610` in the constructor) are
compared against it; on a match the first wallet value is overwritten with **0**. So the pool
in (b) is this class's pool.

**(d) The `910xxxxx` gate matches this client's own cash data. [L]** `FUN_142D466C0` walks
the commodity list and requires `_wtoi(entry+0x28) / 10000 == 910`
(`0x142D46764 mov eax,0x68db8bad` … `cmp edx,0x38e`). `gm-handbook/cashpackage.txt` — dumped
from this client's `Etc_000.wz/CashPackage.img` — lists package item ids **9100000, 9100001**.
Independent of the binary, same numbers.

### 3.1 There is a second, *unreachable* cash shop in this build

`FUN_141072EC0` is a 20-case jump-table dispatcher on `opcode - 0x5AC`, i.e. `0x5AC..0x5BF`,
sitting in one vftable slot at `0x143378290`. `research/msexe-gamestage-dispatch.md`'s range
table attributes `0x05ac..0x05bf` to it and does not mention `FUN_140D734E0` — that row is
incomplete, not wrong.

The constructor that installs that vftable is `FUN_1410368A0`, and
`tools/callers.py 0x1410368a0` reports **zero call sites, zero tail jumps, zero data
pointers** — with the 96-site control passing in the same session. Its sibling
`FUN_141036EC0` has one caller, `FUN_141060AC0`, which itself is reached only through two
tail jumps from an address with no `.pdata` entry.

> **[D], with the blind spot stated:** `callers.py` cannot see an indirect call through a
> register or through the Themida-virtualised loop, and says so itself. What is established
> is that **nothing in the image names `FUN_1410368A0`**, and that the one constructor
> `0x01A3` does name is `FUN_140D71CB0`. Treat the 20-case dispatcher as dead art in the
> same sense as `StatusBar3.img` and `UI/UIWindow2.img/Trunk` — a candidate, not a finding.

Practical consequence: **build against `FUN_140D734E0`'s four opcodes.** If a reply lands in
one of the other sixteen it will be silently dropped, exactly like `0x0468` at the NPC pool.

---

## 4. The body of `0x01A3`, field by field

The whole chain, in read order. Every address is a call to a read primitive or to a function
that makes one; nothing else in these functions touches the packet.

```
FUN_14209AD60  (the handler)
  raw[8]   server clock base -> FUN_1408F67D0            14209ad98   [L]
           (identical to SetField's field 0 at 142097fcb)
  <character record>  FUN_140304B20(user, scratch, pkt, 0)  14209ade8 [L]

FUN_140D71CB0  (the CCashShop constructor, called at 14209aef5 with rdx = the same packet)
  u8       read, not stored                              140d71efd   [L]
  u8       read, not stored                              140d71f02   [L]
  u8       read, not stored                              140d71f0d   [L]

FUN_142D46A20  (called at 140d71f73, rcx = CWvsContext, rdx = the packet)
  u16      nModifiedCommodity                            142d46be0   [L]
           -- zero here jumps the entire loop (je 142d4721c)
    per entry:
      u32    commoditySN                                 142d46c53   [L]
      <commodity body>  FUN_140227F90                    142d46ff4   (SN already known)
                        FUN_140227F90                    142d4714a   (SN is new)
  u16      nNotice                                       142d47287   [L]
           -- zero skips the loop (je 142d47388)
    per entry:
      u32    key                                         142d472a4   [L]
      str    text                                        142d472b3   [L]
  u32      nSpecial                                      142d474fa   [L]
           -- <= 0 skips the loop (jle 142d47608)
    per entry:
      u32    itemId    MUST satisfy itemId/1000 == 5533  142d47513   [L]
             -- on a mismatch the loop BREAKS OUT (jne 142d47608) and
                the remaining entries are NOT consumed
      u32    n                                           142d4753d   [L]
      n x u32                                            142d47594   [L]
```

Notes that matter:

* **There is no fixed head beyond the FILETIME.** `SetField` reads `u32 channel, u8, u32, u8,
  u32, u32, u32, u8 characterData, u16 stringCount` before its character record
  (`research/msexe-stage-setfield.md` §"The fixed head"). `0x01A3` reads **none of that** —
  the FILETIME is followed immediately by `FUN_140304B20`. [L]
* The character record decoder is *the same function* `SetField` uses at `0x14209842D`, with
  the same fourth argument (`xor r9d,r9d`). Whatever body already satisfies `SetField` will
  satisfy this. [L]
* `FUN_142D46A20` is not exclusive to this packet — `tools/callers.py` gives three callers:
  `FUN_140D71CB0` (here), `FUN_140D73AB0`, and `FUN_141057A40` (in the unreachable modern
  UI). The read sequence above is what it consumes wherever it is called. [L]
* `5533xxx` matches **nothing** in this client's data — `gm-handbook/items.txt` has no
  `5533***` row and `commodity.txt` has no such itemId. Send `nSpecial = 0`. [L]

### 4.1 The commodity body, `FUN_140227F90`

2015 bytes across 5 merged `.pdata` entries, **58 reads that `reads.py` counts and at least
one it does not** (see §10). It is a flag-driven optional-field decoder: the first two reads
are unconditional, everything after is gated.

```
u64                                        140227fbe   [L]  unconditional
u32                                        140227fd9   [L]
... 56 further gated reads, u8/u16/u32 ...
f64  8 bytes  -> item+0x90                 140228609   [L]  NOT counted by reads.py
```

**This file does not claim to have decoded `FUN_140227F90`.** It is only entered when
`nModifiedCommodity > 0`, and the minimum legal `SetCashShop` sends `nModifiedCommodity = 0`,
so it is off the critical path. Decoding it is what a "modified/limited-time price" feature
would need, and it is one `listing.py` pass when somebody wants that.

### 4.2 The two self-terminate gates, and why an empty body passes both

Both raise `0x22000006` through `FUN_141804870`, which contains the literal
`"throw CTerminateException"` — this is a hard client death, not a log line. [L]

**Gate 1 — `FUN_142D45C60(ctx, 1)` at `0x140D71EA3`.** Fails if `FUN_14022C5D0` returns 0.
`FUN_14022C5D0` reads `mileageRate`, `onlyMileage`, `originalPrice`, `discount`, `bombSale`,
`forcedCategory`, `forcedSubCategory`, `SubstituteSN`, `gameWorld`, `possibleTrading`,
`couponType`, `Country` — it is the **`Etc/Commodity.img` loader**, property for property the
same list `gm-handbook/commodity.txt`'s header documents. [L] So this gate is about the
client's own WZ, not about our packet, and `commodity.txt` proves this build ships 179 rows
for it to find. **[D]** that it therefore passes; only a run can make that **[L]**.

**Gate 2 — `FUN_142D466C0(ctx, buf)` at `0x142D47435`.** Walks `[ctx+0x2CF0]`. Its first act
is `FUN_1410673E0(vec)`, which is `vector::size()` (`mov rax,[rcx]; test; jne; ret` /
`mov eax,[rax-8]`), and **`size == 0` branches straight to `0x142D469F9: mov eax,1; ret`**.
[L] An empty commodity list passes. So does one whose entries all satisfy the `910xxxxx`
rule.

---

## 5. What the client sends once it is inside

`research/msexe-send-opcodes.txt` maps every `COutPacket` construction site to its opcode;
the cash shop's block is `0x03DF..0x03E7`, plus `0x00D1`.

| opcode | body | builders | what it is |
|---|---|---|---|
| `0x00D1` | **empty** | `FUN_140D73BF0`, `FUN_14107DFC0` | leave — see §7 |
| `0x03DF` | empty | `FUN_14104AE10` | — |
| **`0x03E0`** | **empty** | `FUN_140D73C80`, `FUN_14104B050`, `FUN_14104D450` | **query cash** — see below |
| **`0x03E1`** | `u8 sub-op` + payload | 22 sites | **everything else** |
| `0x03E2` | `str` | `FUN_14104B0E0` | — |
| `0x03E3` | `u8, str, str, u8 [, str]` | `FUN_14104A8C0`, `FUN_141067B10` | coupon-shaped [I] |
| `0x03E4` | several forms; one is `u32, str, str` | `FUN_1410455F0` | — |
| `0x03E5` | `u8, u32, str` | `FUN_14107E060` | — |
| `0x03E6` | `u32 [, raw]` | `FUN_14103C900`, `FUN_14103CCF0`, `FUN_14103CE00` | — |
| `0x03E7` | empty | `FUN_14104AF60` | — |

All bodies from `tools/encodes.py` at depth 1. Only `0x00D1`, `0x03E0` and the six `0x03E1`
builders that live inside the **reachable** class (`0x140D7xxxx`) are on the live path; the
`0x14103xxxx..0x14107xxxx` builders belong to the class §3.1 could not find a caller for. [D]

### 5.1 `0x03E0` is a request with a measured reply

```asm
140d73ca2  cmp  byte [rcx+0x74], 0        ; a request is already in flight -> return false
140d73ca6  jne  140d73cf8
140d73ca8  call [rip+...]                 ; GetTickCount
140d73cb0  mov  edx, [rbx+0x90]           ; last time asked
140d73cbe  cmp  ecx, 0xea60               ; 60 000 ms
140d73cc4  jle  140d73cf8                 ; too soon -> return false
140d73cc6  mov  edx, 0x3e0 ; CTOR ; <no fields> ; SEND
140d73ce0  mov  byte [rbx+0x74], 1        ; in-flight latch
140d73ce4  mov  [rbx+0x90], edi           ; stamp
```

and the `0x5AD` arm ends with `0x140D736DC mov byte [rbx+0x74], sil` where `sil = 0`. **So
`0x03E0` is the request and `0x05AD` is its reply**, measured on both ends of the same latch,
throttled to once per 60 s. [L] Every `0x03E1` builder sets the same `[this+0x74]` latch, so
`0x05AD` — or a `0x5AE` failure — is also what releases the UI after a purchase.

**Who calls it** — `tools/callers.py` gives exactly two sites, and neither is the
constructor: [L]

* `0x1410BCE42`, inside `FUN_1410BCDA0` — the cash shop window's **`checkCash`** button;
* `0x142C8CD4A`, inside `FUN_142C8C8F0`, guarded by two `vt+0xd0` type queries on the current
  stage — i.e. *"if the current stage is a cash shop, ask for cash"*. That function itself has
  zero callers by any of the three scans, which is the signature of an update loop reached
  indirectly. **[D]**

So `0x03E0` is a **poll, not an on-entry handshake**. It nevertheless fires almost
immediately after entry, because the constructor zeroes the stamp
(`0x140D71D57 mov qword [rdi+0x90], rbp`, `rbp = 0`) and `FUN_140D73C80` short-circuits the
60 s test when the stamp is zero (`0x140D73CB8 je 140d73cc6`). [L]

**Do not depend on it.** Nothing in this client *requires* the server to wait for `0x03E0`
before sending `0x05AD`; sending the wallet unprompted right after `SetCashShop` is safe and
removes the timing question. **[D]**

### 5.2 `0x03E1`'s sub-ops, from the reachable class

The first `u8` after each CTOR, read off the listing (the immediate, not inferred):

| sub-op | body after the sub-op byte | builder | candidate meaning |
|---|---|---|---|
| `0x02` **or** `0x1F` | `u8, u32, [u8, u8], u32, u32` | `FUN_140D785F0` | buy. The sub-op is a `cmov`: `mov edx,2 / mov eax,0x1f / cmovb edx,eax`, selected by whether a computed id lies in a 10 000-wide window at `0x08ADDAE0`. Both the sub-op **and** the two `u8 0`s at `140d7a4bf`/`140d7a4cd` hang off that same test, so **the two forms differ in length** [L] |
| `0x03` | `u32, str, str` | `FUN_140D7B240` | gift — id, recipient, message [I] |
| `0x0A` | `raw[8], u32, u8, u16` | `FUN_140D74E10` | an operation on one cash item SN |
| `0x0B` | `raw[8], u32, u8, u32` | `FUN_140D75080` | ditto |
| `0x1C` | `raw[8]` | `FUN_140D75480` | ditto, no arguments |
| `0x2B` | `u32` | `FUN_140D7ADC0` | ditto, one id |

The `raw[8]` is written from `lea rdx,[reg+8]` with `r8d = 8` — the cash item's `u64` serial,
matching `FUN_1402D0950`'s first field. [L]

The unreachable class adds `0x04, 0x1B, 0x21, 0x32, 0x36, 0x37, 0x38, 0x3D` and second
copies of `0x03, 0x0A, 0x0B, 0x2B`. Recorded so a stray sub-op is recognisable, not because
it is expected. [D]

> **A caveat on the scratch extractor.** It tracks "last immediate moved to `edx`/`dl`", so
> only the **first** `u8` after each CTOR is trustworthy; the `imm=` column it prints for
> later fields is stale, and for `FUN_140D785F0` it printed `1` where the listing shows the
> `cmov`. Every sub-op number in the table above was re-read by hand from `listing.py`.

---

## 6. What the client accepts once it is inside

`FUN_140D734E0`, 618 bytes, four arms and nothing else: [L]

```asm
140d734fa  sub  edx, 0x5ad ; je 140d7362f     ; 0x5AD
140d73506  sub  edx, 1     ; je 140d7361e     ; 0x5AE
140d7350f  sub  edx, 0xb   ; je 140d7355a     ; 0x5B9
140d73514  cmp  edx, 1     ; jne <ret>        ; 0x5BA, else ignored
```

### 6.1 `0x05AD` — the wallet

```
off 0   u32   -> FUN_140D85BC0(this, v)        140d73636   [L]   "cash"      (id 601 calls it cash)
off 4   u32   -> FUN_140D85B70(this, v)        140d73640   [L]   maple/leaf points
off 8   u32   read, sign-checked, never stored 140d7364a   [L]
```

All three are tested with `js` and **any negative value aborts**: `FUN_140D7C7F0(this, 2)`
(the generic failure message) followed by `FUN_140D73BF0(this)` — which sends `0x00D1` and
drops the stage. So a negative balance ejects the player. [L]

After storing, the arm clears the in-flight latch `[this+0x74] = 0`, refreshes the UI, and —
if `[this+0x120]` is 1 or 6 — runs a follow-up (`FUN_140D785F0` / `FUN_140D74A70`), i.e. the
pending purchase resumes once the balance is known. [L]

### 6.2 `0x05AE` — the multiplexed result

```
u8  nResult    140d7dcbc   [L]
```

`add eax,-3; cmp eax,0x55; ja <ignore>` then a byte index table at `0x140D7E194` into a
22-entry jump table at `0x140D7E13C`. Decoded: **23 of the 86 sub-ops in `3..0x58` do
anything**; the other 63 land on the common exit and are silently ignored. [L]

| sub-op | case body | what it consumes | notes |
|---|---|---|---|
| `0x03` | `140d7dcfd` | `u16 count`, then `count ×` cash-item record (`FUN_1402D0950`) | a cash inventory list |
| `0x04` | `140d7dced` → `FUN_140D7E1F0` | `u8`, `u32`, `u16`, cash-item records, then `4 × u16` | |
| `0x05`, `0x07` | `140d7de31` | **`u8 nReason`** | shows a message, then **sends `0x00D1` and drops the stage** |
| `0x06` | `140d7e103` → `FUN_140D819C0` | `u16`, `raw` | also builds outbound `0x0196` |
| `0x0C` | `140d7de51` → `FUN_140D7E5E0` | `raw`, `u32`, `u16`, `u8` | |
| `0x0D` | `140d7de61` → `FUN_140D7EAE0` | `u8`, `u32`, `raw` | |
| `0x13` | `140d7e0ac` → `FUN_140D81590` | `str`, `u32`, `u16` | |
| `0x14` | `140d7e0b9` | `u8`, `u32` | |
| `0x19` | `140d7de71` → `FUN_140D7F8A0` | `u8`, `u16` | |
| `0x1A`, `0x1C` | `140d7de81` | **nothing** | UI-only |
| `0x1B` | `140d7deaf` → `FUN_140D80410` | `u8`, `raw`, `u32`, `u16` | |
| `0x1D` | `140d7debf` | `raw[8]` | `-1` is a sentinel (`cmp qword,-1`) |
| `0x1E` | `140d7de93` | **`u8 nReason`** | message only, stage survives |
| `0x3C` | `140d7df3c` | `raw` | |
| `0x3D` | `140d7dfc6` | `u16` | |
| `0x3E` | `140d7e110` → `FUN_140D7F020` | `u8`, `raw`, `u32`, `u16` | |
| `0x3F` | `140d7e11d` → `FUN_140D7F550` | `u8`, `u32`, `raw` | |
| `0x55` | `140d7dfea` | `u32`, `u32` | |
| `0x56` | `140d7e01a` | `u8` | |
| `0x57` | `140d7e027` | `u32`, `u32` | |
| `0x58` | `140d7e06f` | `u8`, `u32` | |
| everything else in `3..0x58` | `140d7e128` | nothing | |

Read shapes are `tools/reads.py` at depth 4, so **any of them that crosses `0x1406E8FB0` is
8 bytes short** — `FUN_1402D0950` does, and its corrected layout is in §6.5.

### 6.3 The failure-reason table — how to say "no"

`FUN_140D7C7F0(this, u8 reason)`, 4968 bytes, `reason-1` indexed into a 127-entry jump table
at `0x140D7D95C`; each case loads a message id and shows it. The mapping is **not linear**.
Decrypted with `tools/dump_stringids.py` (control passed): [L]

| reason | id | text |
|---:|---:|---|
| `0x01` | 600 | Request timed out.\r\nPlease try again. |
| `0x02` | 661 | Due to an unknown error,\r\nthe Cash Shop request has failed. |
| `0x03` | 601 | You don't have enough cash. |
| `0x04` | 612 | You can't buy someone a cash item gift if you're under 14. |
| `0x05` | 613 | You have exceeded the allotted limit of price\r\nfor gifts. |
| `0x06` | 3097 | You cannot send a gift to your own account… |
| `0x07` | 3098 | That character could not be found in this world… |
| `0x08` | 3099 | This item has a gender restriction… |
| `0x09` | 3100 | The gift cannot be sent because\r\nthe recipient's Inventory is full. |
| `0x0A` | 614 | You have too many Cash Items.\r\nPlease clear Cash slot and try again. |
| `0x0B` | 615 | Please check and see\r\nif the name of the character is wrong… |
| `0x0C`, `0x0D` | 661 | (generic) |
| `0x1A` | 1194 | Please check if your inventory is full or not. |
| `0x1F` | 1831 | Items are not available for purchase\r\n at this hour. |
| `0x20` | 1832 | You cannot buy this item\r\nbecause it is sold out. |
| `0x22` | 157 | You don't have enough Mesos. |
| `0x2B` | 616 | You have reached the daily maximum \r\npurchase limit for the Cash Shop. |
| `0x27`,`0x28`,`0x29`,`0x2A`,`0x2C` | 661 | (generic) |

Reasons `0x0E`..`0x26` are coupon messages; the full 127 rows are one re-run of the same
scan. **`nReason = 2` is the safe "no"** — it is the generic error and it is what the
client's own `0x5AD` abort path uses.

> Blind spot: the extractor reads only the **first instruction** at each case head and takes
> a `mov edx, imm32` there. 126 of 127 cases had one; case `0x49` did not and is unmapped.

### 6.4 `0x05B9` and `0x05BA`

```
0x5B9   u8 flag                                   140d7355d   [L]
        flag == 0 -> FUN_1401C2910(pkt, 0)
        flag != 0 -> FUN_1401C2DF0(pkt, &vec)     140d7358d   (a u32 vector; the client
                                                   then frees it without using it)
0x5BA   FUN_1401C3050(pkt) -> count               140d73520   [L]
        count > 0 -> FUN_1410DB190([this+0xa0], count)
```

Both are peripheral. `0x5B9`'s decoded vector is deallocated in both branches of the arm
without being stored anywhere — read from the listing at `0x140D735D8`/`0x140D7361C`. [L]

### 6.5 The cash-item record `FUN_1402D0950`, corrected

Read off the listing rather than from `reads.py`, because `reads.py` misses the 8-byte field
at `+51`:

```
off  +0   raw[8]   -> item+0x20    1402d0972   the cash item serial (u64)
off  +8   u32      -> item+0x28    1402d097a
off +12   u32      -> item+0x2c    1402d0985
off +16   u32      -> item+0x30    1402d0990
off +20   u32      -> item+0x34    1402d099b
off +24   u16      -> item+0x38    1402d09a6
off +26   raw[13]  -> item+0x3a    1402d09bc   13 raw bytes (a fixed-width name field [I])
off +39   raw[8]   -> item+0x47    1402d09ce   FILETIME-shaped
off +47   u32      -> item+0x4f    1402d09d6
off +51   f64 (8)  -> item+0x53    1402d09e1   via 0x1406E8FB0 - see section 10
off +59   u32      -> item+0x5b    1402d09ee
off +63   u32      -> item+0x5f    1402d09f9
off +67   u8       -> item+0x63    1402d0a04
off +68   u8       -> item+0x64    1402d0a0f
off +69   u8       -> item+0x65    1402d0a1a
off +70   u8       flag            1402d0a25   if 0, the record ENDS here (je 1402d0aa6)
          [ u8 type ; <type-dependent blob via vt+0x358> ]   FUN_140303530
```

**71 bytes with the trailing flag set to 0.** [L] Field *meanings* past the serial are not
established — the offsets are.

---

## 7. Getting back out

`FUN_140D73BF0` — 132 bytes, the whole thing: [L]

```asm
140d73c09  mov  edx, 0xd1
140d73c13  call 1406ed520          ; COutPacket(0x00D1)
140d73c1e  call 1415d01c0          ; a per-opcode send census, NOT a packet field (encodes.py
                                   ;  reports no writes in it)
140d73c23  mov  edx, 0x68 ; call 14019b780     ; allocate the neutral 0x68-byte stage
140d73c41  call 141a3ca80                      ; its ctor
140d73c4c  call 14209ee50                      ; CStage::SetStage(neutral, 0)
140d73c57  call 1406ed610          ; SendPacket
```

**`0x00D1` has an empty body.** [L] The client switches to the neutral stage *before* the
send returns, so from that instant it is rendering nothing and waiting.

Where it comes from: `tools/callers.py` gives 11 call sites in 10 functions plus 2 tail jumps,
and all but two are inside the cash-shop class's own abort paths. The one that is a
deliberate user action is **`FUN_1410BCDA0`**, whose `lea rdx,<name>` chain is

```text
exit   checkCash   chargeCash   cartSelectAll   cartDelete   cartBuy
```

— the cash shop window's own buttons, and `exit` is the arm that calls `FUN_140D73BF0`. [L]

So the answer to `0x00D1` is **`0x01A0 SetField`**, exactly as it is after a portal warp.
The `characterData` fork matters: `research/msexe-stage-setfield.md` records that the
`characterData == 0` short form **faults** for a client that has just arrived and has no live
field, and a client returning from the cash shop is in that state — its field object was
destroyed by `SetStage`. **[I]**, and it is the single most likely place a first
implementation dies. Send the full form.

---

## 8. Question 4: the wallet, and whether anything gates the cash shop per account

**The wallet arrives only in `0x05AD`.** `FUN_140D85BC0` and `FUN_140D85B70` — the two
setters the `0x5AD` arm feeds — have **4 and 2 call sites in the entire image**
(`tools/callers.py`, 96-site control passed), in exactly two functions, `FUN_140D734E0` and
`FUN_140D7DB60`. `FUN_140D7DB60` reads the same three `u32`s at `0x140D7DB7D`/`0x140D7DB87`/
`0x140D7DB91` and is a `0x5AE` sub-handler reached through the jump table. So:

> **No CWvsContext opcode carries NX or Maple Points in this client.** [L] The candidate
> names `SetMaplePoint` (`0x011A`) and `SetAccountInfo` (`0x0138`) in
> `research/msexe-gamestage-opcodes.md` are alignment guesses; whatever those handlers do,
> they do not reach either setter.

**No per-account "cash shop enabled" flag was found**, and here is the bounded negative:

* The only conditions between the Cash Shop button and the wire are the six exits of
  `FUN_142CAEE70` already enumerated in `research/cash-shop.md` part five — a null global, a
  counter at `[ctx+0x31FC]`, an open-window chain, the exclusive latch, the 500 ms stamp, and
  the quiz flag. None reads an account attribute.
* Inside the stage, the only two gates that can refuse are `FUN_142D45C60` and
  `FUN_142D466C0` (§4.2), and both are about the client's own `Etc/Commodity.img`.
* **The instrument that would have found a packet-borne flag is `tools/callers.py` on the two
  setters plus `tools/reads.py` on the four handlers, both controlled.** What neither can see
  is a field of the **character record** (`FUN_140304B20`, 18 525 bytes) being consulted
  later by cash-shop code. I did not search the character record for such a field, so that
  possibility is open.

---

## 9. What this file does not establish

* **Nothing here has been on a wire.** Every claim is static. `SetField = 0x01A0` is the only
  member of this block with a live confirmation, and it came from a client run in August.
* The **meaning** of the three `u8`s at the head of the constructor's read (`140d71efd`,
  `140d71f02`, `140d71f0d`). They are read and their return values are discarded within
  `FUN_140D71CB0`; a value that matters would have to matter to the reader, and the reader
  drops it. Sending `0 0 0` is safe on that reading, but the reading is **[D]**.
* The field meanings of `FUN_140227F90` (the commodity body) and of the cash-item record past
  its serial. Offsets yes, semantics no.
* Whether the `0x03E1` sub-op names in §5.2 ("buy", "gift") are right. The *shapes* are read;
  the names are **[I]** from the shape and from the failure strings that mention gifts.
* Whether `FUN_1410368A0`'s stage is genuinely dead (§3.1) — three scans found nothing and
  all three share the "indirect call" blind spot.
* **`0x03E2`..`0x03E7`, and 19 of the 23 live `0x05AE` sub-ops, are unnamed.** They are
  enumerated, not decoded.

---

## 10. Two instruments that were wrong, found on the way

### 10.1 There is an ELEVENTH read primitive, and `tools/reads.py` does not know it

`tools/reads.py`'s `PRIM` table carries a comment saying the count "has now been wrong three
times" — five, seven, eight, nine, ten. It is **eleven**.

`0x1406E8FB0` is byte-for-byte the same reader as the known `u64` primitive `0x1406E8F10`:

```asm
1406e8fb0  ...        1406e8f10  ...
1406e9000  mov  ecx, [rbx+0x24]        1406e8f60  mov  ecx, [rbx+0x24]
1406e9005  add  rax, [rbx+0x10]        1406e8f65  add  rax, [rbx+0x10]
1406e9009  cmp  edi, 8                 1406e8f69  cmp  edi, 8
1406e900c  jb   <underflow, msg 0x26>  1406e8f6c  jb   <underflow, msg 0x26>
1406e900e  movsd xmm0, [rax]           1406e8f6e  mov  rax, [rax]
1406e9012  lea  eax, [rcx+8]           1406e8f71  add  ecx, 8
1406e9015  mov  [rbx+0x24], eax        1406e8f74  mov  [rbx+0x24], ecx
```

Same cursor, same 8-byte advance, same underflow message id. It differs only in returning the
value in `xmm0` — it is the `double` reader. `tools/callers.py` (control passed): **71 call
sites in 35 functions.**

Concretely, in `FUN_140227F90`:

```
1402285f1  call 1406e8ae0   <<< READ u8     (counted)
140228609  call 1406e8fb0                   (NOT counted - 8 bytes)
140228622  call 1406e8ae0   <<< READ u8     (counted)
```

**Every read walk that crossed one of those 71 sites came back 8 bytes short per crossing,
silently** — the exact failure mode `CLAUDE.md` records as having shipped a truncated packet
and killed the client twice. `tools/listing.py` shares `reads.py`'s table and is short in the
same places.

I have **not** edited `tools/reads.py` — `tools/` is read-only for this task. The fix is one
line: add `0x1406e8fb0: "f64"` (8 bytes) to `PRIM`. Then re-run anything that rests on a read
count through those 35 functions.

Two of the 35 are worth naming immediately:

* **`FUN_142CBAA80`** — `CWvsContext::OnPacket` itself, one site at `0x142CBCE46`. One of the
  273 channel opcodes carries an 8-byte field the project has never counted. Which one is not
  established; it needs the dispatcher's jump table walked.
* `FUN_1402D0950`, `FUN_1402D0010`, `FUN_1402D0BF0` — the cash-item family, §6.5.

**`FUN_140304B20`, the character record decoder, does not reach it** at depth 4, and neither
does `FUN_1403047A0` (which is one of the 35, and which has zero callers of any kind). [L]
So the character record is not affected, which is the reassuring half.

### 10.2 `research/msexe-gamestage-dispatch.md`'s range table is incomplete at `0x05AC`

That table has one row for `0x05ac..0x05bf` → `FUN_141072EC0`. There are **two** dispatchers
in that range, and the one `0x01A3` actually installs is `FUN_140D734E0`. Not wrong — the
row was written from a scan that found one stage vtable and stopped. Worth a correction if
anyone edits that file.

---

## 11. WIRE IT LIKE THIS

### 11.1 Answering `0x00D5`

Today `crates/world` answers the Cash Shop click with `inventory_rejected()` + a chat line
(`research/cash-shop.md` part seven). Replace that with `0x01A3` on the **same channel
socket**. Do not migrate, do not mint a seed, do not close anything.

`0x01A3` also clears `[ctx+0x2330]` on its own — `FUN_142CBE8F0` is called at `0x14209AE4E`
and writes 0 at `0x142CBE918` — so the `0x0070` that currently clears the exclusive-request
latch becomes unnecessary on this path. [L]

### 11.2 The minimum legal `SetCashShop`

```text
opcode 0x01A3
  u8[8]    FILETIME     server clock base, same value SetField sends at offset 0
  <character record>    byte-for-byte what SetField's characterData==1 branch sends
                        (FUN_140304B20; already built, already proven on a live client)
  u8       0
  u8       0
  u8       0
  u16      0            nModifiedCommodity  -> skips the commodity loop entirely
  u16      0            nNotice             -> skips the notice loop
  u32      0            nSpecial            -> skips the 5533xxx loop
```

**Seven bytes after the character record.** Nothing in that body can trip either
self-terminate gate: gate 2 returns 1 on an empty list (`0x142D469F9`), and gate 1 never
looks at the packet.

### 11.3 Then, immediately, the wallet

The client polls with `0x03E0` (empty body) from its update loop — first pass right after
entry, then at most once per 60 s — and it will not accept a purchase until a reply lands.
**Send this unprompted immediately after `SetCashShop` rather than waiting for the poll**, and
answer the poll with it as well:

```text
opcode 0x05AD
  u32   nxCredit        >= 0   (a negative value ejects the player)
  u32   maplePoints     >= 0
  u32   0               >= 0   read, range-checked, discarded
```

The client stores fields 1 and 2 and clears its in-flight latch. If the account name begins
`"@--"` the client zeroes field 1 on its own — a quirk, not something to work around.

### 11.4 Refusing a purchase

Every `0x03E1` sub-op sets `[this+0x74] = 1` and the UI blocks until something clears it. The
cheapest correct answer is a failure:

```text
opcode 0x05AE
  u8    0x05           nResult  (or 0x07 - identical case body)
  u8    0x02           nReason  -> "Due to an unknown error, the Cash Shop request has failed."
```

Careful: sub-ops `0x05`/`0x07` show the message **and then send `0x00D1` and drop the stage**,
so the player is ejected. To refuse without ejecting, use

```text
opcode 0x05AE
  u8    0x1E           nResult
  u8    0x02           nReason
```

which shows the same message and leaves the client in the cash shop. [L] Reason bytes worth
knowing: `0x03` "not enough cash", `0x20` "sold out", `0x0A` "too many Cash Items",
`0x2B` "daily maximum".

**Both are complete, legal packets**, and either satisfies `CLAUDE.md`'s "always answer".

### 11.5 Leaving

The client sends `0x00D1`, empty body, and is in a null stage from that instant.

```text
answer with 0x01A0 SetField, characterData = 1
```

Do **not** use the `characterData == 0` short form. `research/msexe-stage-setfield.md`
records it faulting at `FUN_1402FA540+0x1C` reached from `0x142098685` for a client with no
live field, and a client that has just left the cash shop has no live field.

### 11.6 The order to build it in, and what falsifies each step

1. **Answer `0x00D5` with the §11.2 body.** *Falsified if:* the client does not draw the cash
   shop window, or dies. Watch `client-patched\maplecw-hook.log` for a dispatch line on
   `0x142097EE0` — if the line is absent the packet never reached the stage; if it is present
   and the client dies inside, the body is wrong. `CLAUDE.md`, "count the same event in two
   logs": the dispatch line is written on **return**.
2. **Send §11.3's `0x05AD` unprompted, right behind it, and expect a `0x03E0` poll in
   `world.log` shortly after.** *Falsified if:* no `0x03E0` ever arrives — then the stage was
   constructed but its update loop never ran, which means the window never came up.
   *Also falsified if:* `0x03E0` arrives repeatedly at ~60 s intervals with no click — that
   would mean `0x05AD` is not clearing `[this+0x74]` and the reply body is wrong.
3. **Read the balance off the screen.** *Falsified if:* it still reads 0 — then the two `u32`
   fields are in the wrong order or the wrong unit. `CLAUDE.md`, "the unit, not the
   arithmetic": nothing in the listing says these are anything but absolute counts, but
   nothing in the listing said `0x03F0`'s HP was a percentage either.
4. **Click Exit; expect `0x00D1`.** *Falsified if:* no `0x00D1` — then `FUN_1410BCDA0` is not
   the live button dispatcher and §7 is wrong.
5. **Answer `0x00D1` with `SetField`.** *Falsified if:* the client stays black.

Steps 1–2 are one launch and settle the whole of §2–§4. Step 1 alone is worth a run: it is
the first time this client will have been asked to change stage on a live channel socket
without a migrate, and that claim — §2.1 — is the load-bearing one for the entire design.
