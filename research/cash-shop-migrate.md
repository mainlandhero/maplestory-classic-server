# What the server must reply to `0x00D5`, and why it is not a migrate

2026-08-23. `research/cash-shop.md` part seven ends with *"§4 stops being a footnote… the answer
a real server gives is a **migrate**."* This file is the answer to that, and **the migrate turns
out to be optional.** The packet the client is actually waiting for is `0x01A3 SetCashShop`, and
it is dispatched on the connection the client is already using.

Research only. **No Rust was written and nothing under `crates/` was touched.** No Ghidra: the
project lock was held elsewhere, so everything below is `tools/listing.py`, `tools/reads.py`,
`tools/callers.py`, `tools/fieldrefs.py`, `tools/dataref.py`, `tools/xref.py`,
`tools/dump_va.py`, `tools/dump_stringids.py`, `tools/rtti.py`, two scratch scanners, and two
archived runs in `research/fixtures/`.

Labels: **[L]** read off the listing, a capture or the string table. **[D]** derived from two or
more [L]. **[I]** inferred, including anything from `ModernMapleSource`.

---

## 0. The one-line answer

**`0x01A3`, and its body is `u64 FILETIME`, the same character record `0x01A0` carries, three
discarded `u8`s, and a cash-shop commodity block.** `[L]` for the field order, `[D]` for the
opcode number - §4 gives three independent discriminators and the control that separates them.

**A migrate is not required.** `0x01A0..0x01A3` are the four cases of one stage forwarder,
`FUN_142097ee0`, and `CField::OnPacket` chains into it - which is exactly why this server's
`!map` `0x01A0` already works mid-session on a live channel socket. The `0x01A3` handler
**never reads its `this`**, so it does not care which stage forwarded it. `[L]`

If a separate cash-shop process is wanted later, the migrate is `0x001A` - the same seven-byte
socket-level packet a channel change uses, with **no destination field of any kind** - and the
new server still has to send `0x01A3` afterwards. So `0x001A` is a deployment choice; `0x01A3`
is the protocol.

---

## 1. Instruments, each with its documented control run first

Per `CLAUDE.md`, and because two of these carry negatives.

| instrument | control | result |
|---|---|---|
| `python tools/listing.py 0x140304100` | reads at `140304138 raw`, `140304144 u8`, `140304183 u8`, then a run of `u16` | exactly that `[L]` |
| `python tools/reads.py 0x140304100 2` | must show reads **through** `FUN_1403035a0`/`FUN_140303b40` **and** directly | both kinds present `[L]` |
| `python tools/callers.py 0x1402fa9a0` | 96 call sites in 15 functions, 43 of them in `0x140304b20` | exactly that `[L]` |
| `python tools/fieldrefs.py 0x2f4 --lo 0x141c40000 --hi 0x141d60000 --write` | three rows: `141c4d261`, `141c4e6ee`, `141cb7ef3` | exactly that `[L]` |
| `python tools/xref.py --string "UI/Login.img"` | a large, obviously-live string | 67 copies `[L]` |
| `python tools/dump_stringids.py --id 2334` | `0x091E` must be the cash-shop refusal `cash-shop.md` §5 already quoted | *"You cannot go into the cash shop. Please try again later."* `[L]` |
| the scratch string-scan used in §4 | run it on `FUN_142097f80`, the known `SetField` handler | `FIELD%d`, `onUserEnter`, `onFirstUserEnter`, `fieldScript` `[L]` |

**`tools/rtti.py` cannot answer anything here and it is worth saying so.** It parses 1764 type
descriptors and `--list Cash`, `--list Shop` and `--list Stage` all return **0**; the control
`--list Login` returns exactly one (`CLoginQueueDlg`). The stage vtables in this build have no
`CompleteObjectLocator` at `vtable[-8]` - `0x1433f83b0` holds `0x100`, not a pointer - so **RTTI
cannot name a stage class in this client.** Every class identification below is behavioural.
`[L]`

---

## 2. The request side, and one field nobody had looked at

`research/cash-shop.md` parts five to seven established: the `CashShop` arm of `FUN_1411ab7b0`
tail-jumps to `FUN_142caee70`; that function has six gates and then builds `0x00D5`; the body is
`u32 tick, u8`; it latches `[ctx+0x2330]`; and an inbound `0x0070` with `nCount = 0` clears it.
All measured. None of that changes.

What is new is the instruction **eight bytes before the packet is built**: `[L]`

```asm
142caf05a  mov  ecx, edi                 ; edi = arg2 of FUN_142caee70
142caf05c  test edi, edi
142caf05e  je   142caf06f                ; 0 -> store
142caf060  sub  ecx, 1
142caf063  je   142caf06f                ; 1 -> store
142caf065  sub  ecx, 1
142caf068  je   142caf06f                ; 2 -> store
142caf06a  cmp  ecx, 1
142caf06d  jne  142caf075                ; not 3 -> skip
142caf06f  mov  dword ptr [rbx + 0x2d10], edi     ; <-- rbx is the CWvsContext (rcx on entry)
142caf075  ...
142caf167  xor  r8d, r8d
142caf16a  lea  edx, [r8 + 4]
142caf171  call 0x142d18520              ; the 0x0422 "I am leaving, reason 4" telemetry
142caf176  mov  edx, 0xd5
142caf180  call 0x1406ed520              ; COutPacket(0x00D5)
142caf186  call 0x1429e3ef0 ; 142caf192 w_u32   <- client tick
142caf197  movzx edx, sil   ; 142caf1a0 w_u8    <- arg3, the wire flag
142caf1aa  call 0x1415d01c0              ; SEND
142caf1b7  call 0x142cc4430(ctx, 1)      ; [ctx+0x2330] = 1, [ctx+0x2334] = tick
142caf1bc  mov  dword ptr [rbx + 0x31fc], ebp     ; = 0
```

`FUN_142cc4430` is 55 bytes and is exactly `[rcx+0x2330] = edx; [rbx+0x2334] = tick; …` - the
latch setter, read rather than assumed. `[L]`

`FUN_142d18520(ctx, 4, 0)` builds **`0x0422`** (`u32 reason=4`, then a string). `[L]` That is why
`0x00D5` has always arrived inside the `0x0420..0x0426` burst that `CLAUDE.md` now records as
the burst nobody enumerated: **the burst is not shutdown telemetry, it is the client announcing
that it is leaving the field**, and the Cash Shop click is one of the things that causes it.

### 2.1 The sender's callers, and what the two arguments mean

`tools/callers.py 0x142caee70`: **13 call sites and 5 tail-`jmp` sites**, 18 entries in all.
`[L]` Read at each site, the two arguments take these values:

| `edx` (arg2) -> `[ctx+0x2d10]` | `r8b` (arg3) -> the wire `u8` | sites |
|---|---|---|
| `0` | `0` | the `CashShop` arm of `FUN_1411ab7b0`, and most others |
| `0` | **`1`** | exactly one: `0x142255ae0` (`mov r8b,1; xor edx,edx`), unidentified |
| `2` | `0` | `141a21c4f`, and the two `0xCC`-bounded shims `141a21e68`/`141a21f37` |
| `3` | `0` | `1423b8765`, `1423bac55` |

The Cash Shop button is `edx = 0, r8b = 0`, so `[ctx+0x2d10]` becomes **0** and the wire byte is
**0**. That matches every real capture: `research/fixtures/cash-shop-three-clicks-three-requests-world.log`
has three `0x00D5` bodies, `71bac30500`, `6dcdc30500`, `2be2c30500` - `u32 tick` then `00`. `[L]`

**And the arguments were measured on a real click, not only read off the listing.** The watch on
`FUN_142caee70` in `research/fixtures/cash-shop-latch-clears-0-0-0-hook.log` logs registers on
entry, and all three clicks of that run read: `[L]`

```text
WATCH #1  0x142caee70 ENTERED  [rcx+0x2330]=0x00000000  rdx=0x0  r8=0x0  called-from=0x14168ae7d
WATCH #2  0x142caee70 ENTERED  [rcx+0x2330]=0x00000000  rdx=0x0  r8=0x0  called-from=0x14168ae7d
WATCH #3  0x142caee70 ENTERED  [rcx+0x2330]=0x00000000  rdx=0x0  r8=0x0  called-from=0x14168ae7d
```

`rdx = 0` is the mode, so **`[ctx+0x2d10]` really is written with `0` on a real Cash Shop click**;
`r8 = 0` is the wire byte, which the captured bodies independently confirm. The return address is
the *caller's caller* because `FUN_1411ab7b0` reaches the sender by tail `jmp`, exactly as
`cash-shop.md` part two's listing shows.

### 2.2 The sibling sender names the field

`FUN_142cb6700` builds **`0x00D6`**, takes the same mode argument (it accepts only `2` and `3`),
and raises message `0x091F` on failure. Decrypted out of the client's own table: `[L]`

```text
0x091E  "You cannot go into the cash shop. Please try again later."          <- 0x00D5's gate
0x091F  "You cannot access the Auction House at this time. …"                <- 0x00D6, mode 2/3
0x0920  "You can't move to the New Name Auction right now. …"
0x0922  "You have equipment, skills, or abilities … you can attempt to go to the Cash Shop."
```

`0x0922` is the message `FUN_142caee70` raises at `142caf1d3`/`142caf205`, and it says *"the Cash
Shop"* in so many words. **`0x00D5` is the cash-shop request, out of the client's own strings.**
`[L]`

---

## 3. The whole `CStage::OnPacket` block, enumerated

`FUN_142097ee0` has **no `.pdata` entry** - it is a forwarder inside another function's range,
which is why `tools/listing.py` refuses it and it had to be disassembled directly. Its entire
body: `[L]`

```asm
142097ee0  sub edx, 0x1a0
142097ee6  je  142097f1b          ; 0x1A0 -> rcx-0x18, rdx=packet, jmp 0x142097f80
142097ee8  sub edx, 1
142097eeb  je  142097f0f          ; 0x1A1 -> jmp 0x14209b070
142097eed  sub edx, 1
142097ef0  je  142097f03          ; 0x1A2 -> jmp 0x14209b380
142097ef2  cmp edx, 1
142097ef5  jne 142097f27          ; anything else -> ret
                                  ; 0x1A3 -> jmp 0x14209ad60
```

**Four cases. There is no fifth.** `[L]`

`tools/callers.py 0x142097ee0` puts it in **four vtable slots** and gives it two direct callers:
`FUN_141b25f30` (the login stage's `OnPacket`) and `FUN_141820080` (`CField::OnPacket`). One of
those four slots is `0x1433f84d0`, and `0x1433f84d0` is precisely the pointer that
`FUN_141a3ca80` writes to `[obj+0x18]`: `[L]`

```asm
141a3ca80  push rbx ; sub rsp,0x20 ; mov rbx,rcx
141a3ca89  call 0x142097cf0            ; base ctor
141a3ca8e  lea rax,[rip+…] -> 0x1433F83B8 ; [obj+0x00]
141a3ca98  lea rax,[rip+…] -> 0x1433F83F8 ; [obj+0x08]
141a3caa3  lea rax,[rip+…] -> 0x1433F84D0 ; [obj+0x18]  <- slot 0 is FUN_142097ee0
141a3caae  lea rax,[rip+…] -> 0x1433F84D8 ; [obj+0x20]
```

`FUN_141a3ca80` is the 0x68-byte transition stage the **migrate handler installs**
(`research/change-channel-reply.md` §2.4, `1415d8cc9 FUN_14019a150(0x68)` ->
`1415d8ce0 FUN_141a3ca80` -> `1415d8d15 FUN_14209ee50(obj, 0)`). `[L]`

So: **after any migrate, the only four packets the client can act on are `0x01A0..0x01A3`**, and
each of them installs a different stage.

### 3.1 The block's far edge, which is what fixes the numbering

`CField::OnPacket`'s jump table is at `0x141822158`, dense, indexed by `opcode - 0x1A4` over
`0..0x7f`. Slot 0 (`0x01A4`) has a real handler; **slots 1..7 (`0x01A5..0x01AB`) all point at the
default**, `0x141821e24`. `[L]` The default is the head of a range chain, and every range in it
was extracted:

```text
0x224-0x39F  0x3A0-0x3C5  0x3C6-0x44E  0x44F-0x468  0x469-0x46D  0x46E-0x46F  0x470-0x472
0x473-0x475  0x476-0x477  0x478-0x48C  0x48D-0x48F  0x55B-0x55C  0x5F1-0x5F3  0x55D-0x55E
0x1A0-0x1A3  0x51-0x6F    0x55F-0x560  0x600-0x602  0x612-0x61A  0x490-0x491  0x606-0x611
0x5DE-0x5E8  0x544-0x559  0x603-0x605  0x621-0x625
```

**`0x01A5` is in none of them**, and `FUN_142cbaa80`'s 273 labels stop at `0x019F`
(`research/msexe-gamestage-cases.txt`, last three rows `0x019f`, `0x0275`, `0x039a`). `[L]`

> **Named blind spot.** The third singleton dispatcher, `FUN_14177b7e0` on `DAT_143ace378`,
> has never been read (`maplecw-inbound-dispatch`). `0x01A5` could be one of its cases. So
> *"`0x01A5` is unhandled"* is `[D]`, not `[L]`, and the missing evidence is one function dump.

That the block is `0x01A0..0x01A3` and `CField::OnPacket` begins at `0x01A4` lines up exactly
with both references, where `END_STAGE` is immediately followed by `BEGIN_FIELD =
TransferFieldReqIgnored`. `[D]` mscw has **four** entries where v214 and v265 have three
(`SetField`, `SetFarmField`/`SET_AUCTION_FIELD`, `SetCashShop`) - which is consistent with mscw
carrying **both** the farm field and the auction field. `[I]`

---

## 4. Which of the four is the cash shop, with the control that separates them

The three non-`SetField` handlers are byte-for-byte the same shape - read `raw 8`, decode a
character record with `FUN_140304b20`, allocate, construct, install - and differ in exactly one
call each: `[L]`

| opcode | handler | object size | stage ctor | app state set at the end |
|---|---|---:|---|---|
| `0x01A0` | `FUN_142097f80` | - | (`SetField`) | `FUN_142c50b90(3)` |
| `0x01A1` | `FUN_14209b070` | `0x70` | `FUN_140f33b70` | `FUN_142c50b90(5)` |
| `0x01A2` | `FUN_14209b380` | `0x68` | `FUN_1411daf30` | `FUN_142c50b90(6)` |
| `0x01A3` | `FUN_14209ad60` | `0x1a0` | `FUN_140d71cb0` | `FUN_142c50b90(4)` |

Three discriminators point at `0x01A3`, and they are independent of each other.

### 4.1 `[ctx+0x2d10]` closes a request/response pair

`0x01A3` is the only one of the four that reads the field the `0x00D5` sender wrote: `[L]`

```asm
14209aec4  mov  rcx, [rip+0x1a0d5d5]     ; -> the CWvsContext singleton 0x143AA84A0
14209aecb  call 0x142cc42c0              ; = mov eax,[rcx+0x2d10] ; ret
14209aed0  mov  [rsp+0x68], eax
14209aed4  mov  edx, 0x1a0               ; allocate
14209aee0  call 0x14019b780
14209aef5  call 0x140d71cb0(obj, packet) ; ctor - and it READS from the packet
14209aeff  lea  rdx, [rsp+0x68]          ; <-- the mode goes into the new stage
14209af07  call 0x14209ee50              ; install it
```

`0x01A0`, `0x01A1` and `0x01A2` all pass a `&[rsp+0x68]` that was just set to **zero**
(`mov dword [rsp+0x68], edi` with `edi` xor'd). Only `0x01A3` fills it from the context. `[L]`

**And that field has exactly two touchers in the whole image.** Two instruments, different
algorithms, same answer:

* `python tools/fieldrefs.py 0x2d10` (whole image, resyncing linear sweep): **10 hits**,
  666 939 resync points;
* a scratch byte-locate for the little-endian disp32 `10 2d 00 00` across `.text`, then a
  boundary check by disassembling from each hit's `.pdata` function start: **39 raw byte hits,
  10 real instructions** - the same ten.

```text
140865894  lea   rcx,[rdi+0x2d10]        140862520      other class
140873572  lea   rdx,[rbx+0x2d10]        1408674e0      other class
14088d140  movsd [rsi+0x2d10],xmm0       140886810      a DOUBLE - other class
140c4476b  mov   [rbx+0x2d10],eax        140c44750      other class: it writes an obfuscated
                                                        triplet +0x2d10/+0x2d14/+0x2d18
14294af77  movsd [rsi+0x2d10],xmm0       1429446b0      a DOUBLE - other class
142caf06f  mov   [rbx+0x2d10],edi        142caee70   <- THE 0x00D5 BUILDER
142cc42c0  mov   eax,[rcx+0x2d10]        (leaf, no .pdata)  <- the getter
142fb4df0  lea   rcx,[rdx+0x2d10]        (no .pdata)    adjustor thunk table
142fb4e5c  lea   rcx,[rdx+0x2d10]        (no .pdata)    adjustor thunk table
142fb5473  lea   rcx,[rdx+0x2d10]        (no .pdata)    adjustor thunk table
```

`python tools/callers.py 0x142cc42c0` -> **1 call site, 0 tail jumps, 0 pointers**, and the one
call site is `0x14209aecb` inside `FUN_14209ad60`. `[L]`

> So on the `CWvsContext`, `+0x2d10` is written by one function and read by one function, and
> they are **the `0x00D5` request builder and the `0x01A3` handler**. `[D]`

> **Named blind spots.** (a) `.themida` has `SizeOfRawData = 0`, so a VM-resident access to
> `+0x2d10` has no file bytes and neither scan can see it. (b) A displacement is a class fact,
> not an offset fact - `[rbx+0x2d10]` on `FUN_140c44750`'s object is a different field, and the
> reason to say so is positive rather than absent: that function writes `+0x2d10`, `+0x2d14` and
> `+0x2d18` as an `xor 0xBAADF00D` / `ror 5` obfuscated triplet, which is a pattern the
> `CWvsContext` accesses do not use.

### 4.2 Only `0x01A3` reads anything after the character record

`tools/reads.py <handler> 3`, all three: `[L]`

```text
FUN_14209b070 (0x1A1)   raw@14209b0a8 ; FUN_140304b20@14209b0f8         <- and nothing else
FUN_14209b380 (0x1A2)   raw@14209b3b8 ; FUN_140304b20@14209b413         <- and nothing else
FUN_14209ad60 (0x1A3)   raw@14209ad98 ; FUN_140304b20@14209ade8 ;
                        FUN_140d71cb0@14209aef5 -> u8 u16 u32 u64 str
```

`tools/reads.py` at **depth 4** on the three constructors: `FUN_140f33b70` and `FUN_1411daf30`
report *"no reads found"*; `FUN_140d71cb0` reports three direct `u8`s at `140d71efd`,
`140d71f05`, `140d71f0d` plus a subtree through `FUN_142d46a20`. `[L]`

The reference half of this, and it is `[I]`: in `ModernMapleSource`'s `Stage.java`,
`setAuctionField` is *"FILETIME, character record"* and nothing else, while `setCashShop` is
*"FILETIME, character record, byte, byte, byte, byte, then a cash-shop blob"*. **The only member
of the STAGE block that writes anything after the record is the cash shop**, and the only mscw
handler that reads anything after the record is `0x01A3`.

### 4.3 The stage's own `OnPacket` names it outright, and the siblings are the control

Each stage class's packet entry is slot 0 of the vtable at `[obj+0x18]`. Read out of the three
constructors and dereferenced: `[L]`

```text
0x01A1 stage  vtable 0x1433727a0 -> FUN_140f34440   handles 0x5C0, 0x5C1
0x01A2 stage  vtable 0x14338e138 -> FUN_1411db710   handles 0x5C2, 0x5C3
0x01A3 stage  vtable 0x14336def8 -> FUN_140d734e0   handles 0x5AD, 0x5AE, 0x5B9, 0x5BA
```

Scanning every readable `lea reg,[rip+d]` string target reachable from each, at depth 2, with the
**same** scan on all three so the comparison is a control and not a hunt: `[L]`

```text
0x01A1  FUN_140f34440   error, boxResult, questChime, casher
                        and via FUN_140f5c3c0:  "Current Stage : %d", "Field : %d",
                        "FieldType : %d", "ReturnMap : %d", "Town Map"     -> a FIELD
0x01A2  FUN_1411db710   error, boxResult, questChime, casher,
                        "Etc/GlobalMarketData.img"                          -> the AUCTION HOUSE
0x01A3  FUN_140d734e0 -> FUN_1410db190:
                        mileageRate, onlyMileage, token, Bonus, originalPrice,
                        discount, bombSale, forcedCategory, forcedSubCategory,
                        favorType, WSLimitMax                               -> the CASH SHOP
```

`originalPrice`, `discount`, `bombSale`, `mileageRate` and `forcedCategory` are Cash Shop
commodity fields. **The instrument discriminates** - it returns a field for one sibling, a market
for another, and a price list for the third - which is what makes the third result evidence
rather than a coincidence.

`UI/CashShopUI.img` corroborates weakly and is recorded only so nobody re-walks it: the utf-16
copy at `0x14336dda0` sits immediately before `FUN_140d71cb0`'s vtable group
(`0x14336dde0..0x14336df00`), i.e. in the same COMDAT neighbourhood. That is suggestive, not
probative, and it is not what settles anything above.

> **`0x01A3` is `SetCashShop`; `0x01A2` is the auction field; `0x01A1` is the farm field.**
> The *behaviour* is `[L]`. The *number* is `[D]` - three discriminators agreeing, none of them
> a name lifted from a reference.

---

## 5. Question 1: which inbound opcode does `0x00D5` expect

**`0x01A3`.** Not `0x0011`, and `0x0011` cannot even be dispatched here - it is a login-stage
case of `FUN_141b25f30` and sits below the game switch's `0x70` floor. That is not a new finding;
`crates/world/src/session/field.rs` already records it costing a channel change and a character
that could not attack.

`0x001A` - the socket-level migrate `FUN_1415d8c00`, measured on 2026-08-21 - **is** dispatchable
from a channel connection and will move the client to another address. But it is not an answer to
`0x00D5` in any sense the client can see (§7), and it is not required (§10.1).

---

## 6. Question 2: the `0x001A` body, for the cash shop and for a channel change

**They are the same packet, and there is no branch.** `FUN_1415d8c00` is 620 bytes and
`tools/listing.py` marks exactly three reads in it: `[L]`

```text
1415d8c2b  u8   ok      test eax,eax / je 1415d8e3d
1415d8d2c  u32  ip      written unchanged into sockaddr_in.sin_addr -> NETWORK order on the wire
1415d8d3d  u16  port    handed to FUN_1415e2450, which htons() it   -> LITTLE-ENDIAN on the wire
```

`tools/reads.py 0x1415d8c00 2` lists the same three at the same addresses. **There is no fourth
field, no destination flag, no character id and no seed.** The handler then installs the
transition stage, calls `FUN_142caa360` (the reconnect), sends outbound `0x0118`, and sets the
app state to 9. `[L]`

The re-run of the connect-funnel enumeration, because a claim this load-bearing should not rest
on a file from two days ago: `[L]`

```text
tools/callers.py 0x1415d0b40  -> 1 call site:  FUN_142caa360
tools/callers.py 0x142caa360  -> 2 call sites: FUN_1415d8c00, FUN_141b36f60
tools/callers.py 0x1415d8c00  -> 0 of every kind (VM-dispatched, as expected)
```

> **Named blind spot, unchanged from `research/change-channel-reply.md` §1.** `callers.py` cannot
> see a call through a register, a vtable slot or the Themida VM. The claim is *every statically
> readable path to a socket connect funnels through `FUN_142caa360`*, and `FUN_1415d0b40` is
> itself virtualised.

So the answer to question 2 is: **the cash-shop path reads the same body as the channel path,
because there is only one body.** The migrate carries nothing that could distinguish them.

---

## 7. Question 3: how the client knows to enter the cash shop

**Purely the first packet the new server sends.** Three measurements, none of them leaving room
for a flag:

1. **Not in the migrate.** Three reads, no destination field (§6). `[L]`
2. **Not in the hello.** `0x007D` is built by `FUN_1415d10e0` at `0x1415d29f6` in one
   straight-line sequence - `u32` from `FUN_142cb9240`, `u32` from `FUN_142cb8470`, `u32` from
   `FUN_142cb9550`, `raw[16]`, `u8`, `u8`, `u32`, `u32 [conn+0x68]`, then the obfuscated tail.
   The only conditional between the `COutPacket` and the send picks a *value* for one `u32`
   (`-1`, `-2`, or a version read out of a struct); it does not add or remove a field. `[L]`
3. **It is the packet.** After a migrate the current stage is `FUN_141a3ca80`'s object, whose
   packet entry is `FUN_142097ee0`, whose entire vocabulary is `0x01A0..0x01A3` - one field, one
   farm field, one auction field, one cash shop (§3, §4). `[L]`

**What this means for the coordinator: one listener is enough.** There is nothing in the migrate
or the hello for a second listener to key on - a dedicated cash-shop port would have to be told
which pending migration it is serving by the database anyway, exactly the way
`claim_sole_migration_for_channel` already does it. And because `CField::OnPacket` chains into
`FUN_142097ee0` (`141821fdc lea eax,[r9-0x1a0] / cmp eax,3 / 141821fee call 0x142097ee0`, `[L]`,
already recorded in `research/classic-shop-opcode.md` §2), the **existing** channel connection can
carry `0x01A3` with no migrate at all.

One more fact that makes the no-migrate route cheap rather than merely possible: `[L]`

```asm
14209ad60  mov [rsp+8], rbx      ; the rcx home slot, used for rbx
14209ad65  mov [rsp+0x18], rbp
...
14209ad87  mov rbp, rdx          ; the packet
14209ad95  mov rcx, rbp          ; <-- rcx overwritten, never once read
```

**`FUN_14209ad60` never reads its `this`.** It works out of globals and the packet, so it does
not care whether `FUN_142097ee0` was entered from the transition stage or from `CField`.

---

## 8. Question 4: what the client sends on the new connection

**Measured, for a channel migrate**, in `research/fixtures/channel-migrate-is-0x001a-ch1.log` -
the 2026-08-21 run where the migrate actually fired: `[L]`

```text
17:49:53.354  connection from ch1 #1 127.0.0.1:60342
17:49:53.354  -> channel greeting, 22 bytes
17:49:53.354  <- 0x0070  45-byte env report   0264000000000000000000000002000000abde…
17:49:53.356  <- 0x007D  99-byte migration hello
              0000000000000000 017f0000 aabbccddeeffdeadbeef 0000 0000764d0000 …
              u32 world=0   u32 channel=0   u32 "character id" = 0x00007f01 = 32513
```

Then it waits. Three points:

* **It is the same `0x007D`**, because §7.2 shows one unconditional builder. A cash-shop migrate
  cannot send a different hello. `[D]`
* **The character id in it is not usable after a channel migrate.** `32513` is `01 7f 00 00`
  read as a little-endian `u32` - the first four bytes of the migrate body we sent. This is
  already recorded in `crates/world/src/session/mod.rs`, which is why
  `claim_sole_migration_for_channel` exists. Anything built on a cash-shop migrate inherits that
  problem verbatim, and it is a second reason to prefer the no-migrate route.
* **The `0x0070` that arrives first is the env report, not a reply**, and it is unrelated to the
  `0x0070` this server sends to clear `[ctx+0x2330]` - same opcode, opposite direction.

---

## 9. Question 5: the server-driven refusal

**There is no dedicated "the cash shop is unavailable" packet, and the migrate's own refusal is
worse than useless.**

`0x001A` with `ok = 0` reaches this, and this is the whole branch: `[L]`

```asm
1415d8e3d  mov r8d, 0x21000002
1415d8e43  mov edx, 0x63a
1415d8e48  lea rcx, [rip+0x1c990b5]
1415d8e4f  call 0x1412825a0        ; resolve the code to a string and log it
1415d8e54  ...stack cookie, ret
```

It logs and returns. **No dialog, no chat line, nothing on screen** - and, decisively, it does
not touch `[ctx+0x2330]`. A `0x00D5` answered with `0x001A 00` would leave the exclusive-request
latch set, which `research/pick-up-latch.md` §2.2.2 shows also gates the pick-up sweep. That is
the exact failure `cash-shop.md` part seven fixed.

The `0x01A5` slot - `TransferChannelReqIgnored` under the block alignment of §3.1 - **is not
handled anywhere this pass can see**: it is a default slot in `CField::OnPacket`'s dense table, no
range in that function's chain covers it, and `FUN_142cbaa80`'s cases stop at `0x019F`. `[D]`,
with the `FUN_14177b7e0` blind spot named in §3.1. Sending it would most likely be inert.

**So the refusal that works is the one already shipping**, and it is measured rather than
argued: `inventory_rejected()` - a `0x0070` with `nCount = 0` - plus a chat line.
`research/fixtures/cash-shop-latch-clears-0-0-0-hook.log` recorded `[ctx+0x2330]` as **0, 0, 0**
across three clicks where the unanswered run read **0, 1, 1**. `[L]` Nothing in this file
suggests changing it; §10 only says what to send when the answer is yes.

---

## 10. WIRE IT LIKE THIS

Nothing below is wired. `crates/` contains no reference to `0x01A3`, `0x001A` for the cash shop,
or `FUN_142d46a20`'s block. `[L]` `crates/world/src/session/cashshop.rs` answers `0x00D5` with a
latch-clear and a chat line, and that stays as the refusal path.

### 10.1 Do not build a second listener first

The migrate is a deployment choice, not a protocol requirement (§7). The route with the fewest
moving parts, and the one that can be tested in a single client run:

```text
<- 0x00D5  u32 tick, u8 flag                     (the click; flag is 0 for the Cash Shop)
-> 0x01A3  SetCashShop, on the SAME socket        (§10.2)
```

If a separate process is wanted later, it becomes:

```text
<- 0x00D5
-> 0x001A  01 <ip[4] network order> <port u16 LE>  + 64 zero bytes of padding
           ... do NOT close the socket; the client tears it down itself, measured at 8 ms
<- 0x0118  u32 tick, u8, u8, u32   - a notification. Log it, never error on it.
   ch0 read returns EOF
   the cash-shop listener gets: greeting, 0x0070 env report, 0x007D hello (§8)
-> 0x01A3  SetCashShop
```

`0x001A`'s body is byte-identical to the channel one - `crates/world/src/session/field.rs`'s
`migrate_candidates` already builds it, padding included. For 127.0.0.1:8486 it is
`01 7f 00 00 01 26 21` plus padding. **Reuse that function; do not write a second builder.**

### 10.2 The `0x01A3` body

```text
u64   FILETIME                          14209ad98  raw 8 -> FUN_1408f67d0
<character record>                      14209ade8  FUN_140304b20(charObj, &out, packet, 0)
u8                                      140d71efd  return value DISCARDED
u8                                      140d71f05  return value DISCARDED
u8                                      140d71f0d  return value DISCARDED
<cash-shop commodity block>             140d71f73  FUN_142d46a20(world, packet)
```

**The head is `0x01A0`'s head minus 25 bytes.** `SetField` sends a 33-byte head
(`net::opcode::set_field_head`) and then three `u32` before the record; `0x01A3` sends **only the
8-byte FILETIME** and goes straight into the record. Getting that wrong shifts every field of the
record and the client will decode garbage or throw. `[L]`

The character record is decoded by **the same `FUN_140304b20`** `SetField` uses, with the same
argument shape (`r9d = 0`, `[rsp+0x20] = 0`), so
`net::opcode::set_field_with_character`'s record blob is reusable as-is. `[D]` - the fifth
argument at the `0x01A0` site is `r12d` rather than a literal `0`, and I did not trace what
`r12d` holds there. If the record decodes wrongly, that is the first thing to check.

The three `u8`s can be anything - their return values go nowhere - so send `00 00 00`. `[L]`

**The commodity block is a real decode job and it is not optional.** `FUN_142d46a20` is called
whenever the world singleton is non-null, which in game it always is (`140d71f12 mov rbx,
[rip+0x2d36587]` resolves to `0x143AA84A0`, tested at `140d71f19`). `[L]` `tools/reads.py
0x142d46a20 3` gives eleven reads - first a `u16` after a loop over `world+0x2ce8`, then `u32`,
two subtrees through `FUN_140227f90` (`u64 u32 u16 u8`), then `u16, u32, str`, then four `u32`.
Decoding it is the next piece of work and this file does not do it.

### 10.3 Ordering and the latch

1. **Clear `[ctx+0x2330]` either way.** `0x01A3` was not checked for a latch clear and there is no
   reason to assume one. Sending `inventory_rejected()`'s `0x0070` *before* the `0x01A3` costs
   one packet and is the difference between "the cash shop failed" and "the cash shop failed and
   the button and the pick-up sweep are dead for the session".
2. **Do not close the socket** on the migrate route. `FUN_142caa360`'s first act is
   `FUN_1415d35f0(conn)` - the client tears its own connection down, measured at 8 ms after the
   login `0x0011` and never after a reply it did not understand.
3. **`[ctx+0x31fc]`.** `FUN_142caee70` zeroes it on send, and `FUN_142cb6700` (the `0x00D6`
   *leave* request) refuses with message `0x091F` when it disagrees with the mode. Nothing in
   `FUN_14209ad60` writes it - the writers are `FUN_142cf4400`, `FUN_142cf4440` (which sets it to
   `4`), `FUN_142cb6700` and `FUN_142ce0130`, none of which is on this path. `[L]` **So "leaving
   the cash shop" may not work even when entering it does**, and that is unread rather than
   broken.

---

## 11. The client experiment, if `0x01A3` is built and does not work

`0x01A3`'s handler has, as far as anything here can tell, **never executed in this project**.
That is the same situation `research/channel-select.md` §9.10 flagged before the channel-list fix,
and it earned that section's warning.

One variant, one run, and it does not need a second listener or a migrate:

**Reply to `0x00D5` with `0x0070` (latch clear) then `0x01A3`, on the same socket.** Nothing else
changes. The owner clicks Cash Shop **once**, waits three seconds, and reports.

| what the screen does | what it means |
|---|---|
| **the Cash Shop opens, or opens empty/broken** | `0x01A3` is `SetCashShop` and the no-migrate route works. Everything left is the commodity block (§10.2) |
| **the world goes away and the character is nowhere** | the stage was installed and the cash shop UI had nothing to draw. Still a win: `0x01A3` is the right opcode. Read `world.log` for a `0x00D6` (the leave request) - if it arrives, the stage is live |
| **nothing at all, and the button still works on the next click** | the packet was dispatched and the handler bailed. `FUN_14209ad60` bails only if `FUN_1420a3080(0)` returns null - watch `142cc42c0:hits=4` and `14209ad60:hits=4`; a WATCH line on the first proves the handler was entered *and* that `[ctx+0x2d10]` was read |
| **the client freezes, UI and quit prompt dead** | the record or the commodity block ran off the end of the body and threw (`1406e8b51` -> `_CxxThrowException` -> `int3`). Pad the body: the frame carries its own length, so surplus bytes are ignored and a short body is fatal |
| **the client faults** | new code ran. Read `client-patched\maplecw-hook.log` for `CLIENT FAULT` and the ELog `0x008F`/`0x0090` first |

**The cheapest tell needs no hook**: `world.log` writing one dispatch line per inbound opcode *on
handler return*. A `0x01A3` with **no** dispatch line means the client entered the handler and
never came back - which is the same instrument that found the equip crash and the `0x001A`
migrate.

---

## 12. What this file cannot bear

* **`0x01A3` is `[D]`, not `[L]`.** No capture of a `SetCashShop` exists. Three discriminators
  agree (a closed `+0x2d10` data flow, the only handler that reads past the record, and cash-shop
  commodity node names in its stage's own `OnPacket`), and the sibling scan is a control that
  separates the three - but nothing has been seen on screen. Do not let §10's code block harden
  it.
* **The commodity block is undecoded.** `FUN_142d46a20` has eleven reads at depth 3 and walks two
  arrays on the world singleton before the first of them. §10.2 names the entry point and stops.
* **`0x01A5` "is unhandled" is `[D]` with a live blind spot** - `FUN_14177b7e0`, the third
  singleton dispatcher, has never been read.
* **`FUN_140304b20`'s fifth argument** is a literal `0` on the `0x01A3` path and `r12d` on the
  `0x01A0` path. I did not trace `r12d`. If the record decodes wrongly this is the first suspect,
  and it is one listing away.
* **`0x001A` remains the channel migrate's measured opcode** and nothing here re-opens it. What
  this file adds is that it carries no destination information, so it cannot *be* the answer to
  `0x00D5` - only a way to move the client before answering.
* **`tools/rtti.py` was useless here and the reason is structural**, not a bad query: the stage
  vtables in this build carry no RTTI locator at `vtable[-8]`. Anyone reaching for class names in
  this part of the client should expect the same and skip straight to behaviour.
