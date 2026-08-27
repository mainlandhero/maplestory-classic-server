# `!locker 1` killed the client: what threw, and whether the Cash-tab item body is wrong

2026-08-26. The owner: *"The !buy command worked, but `!locker 1` seems to have caused the client
to exit."*

Evidence: `world.log` and `client-patched\maplecw-hook.log` as they stood after that run, plus
`client-exit.log`. Nothing was moved or deleted. The run's `world.log` is already archived as
`research/fixtures/cash-shop-buy-request-captured-world.log`.

---

## 0. The answer

**Reading (a).** The item body is wrong for *this item id*, and the client threw when the
**pet tooltip** rendered it.

`5000001` is a **pet**. The client's own item classifier `FUN_1401b1040` returns **3 (pet)**
for every id in `5000000..=5009999` and **2 (bundle)** for everything else in the 5xxxxxx
range. We sent the item with a leading type byte of **2**, so the client's factory
`FUN_1403095e0` built a **126-byte bundle object**. The tooltip, however, does not ask the
object what it is - it re-derives the class from the **item id** and dispatched to the
**pet** tooltip `FUN_14266f2d0`, which read a secured `u16` at `item+0x7a` **and its checksum
at `item+0x7e` - four bytes past the end of a 126-byte allocation.** The checksum verifier
`FUN_1401b00e0` refused, threw `ZException`, nobody caught it, and the process exited with
`0xE06D7363`.

What is **not** wrong: the missing `u64` cash serial and the expiry `FILETIME` named in the
brief. The client's base decode makes `hasCashSN = 0` a first-class case and explicitly zeroes
the serial field on that branch, and we do send the expiry. Neither is implicated. **[L]**

Both classes are exactly sized to their last field, which is what makes the overread certain:

| class | type byte | allocation | last field ends at |
|---|---|---|---|
| bundle (`FUN_1402f7cd0`) | 2 | `0x7e` = 126 | `+0x7a` u32 -> `0x7e` |
| pet (`FUN_1402f8b40`) | 3 | `0xce` = 206 | `+0xca` u32 -> `0xce` |

---

## 1. What threw

`0xE06D7363` is the MSVC C++ EH code. Throws #1..#6 are the routine startup pair that the
control already settled; **#7 and #8 are unique to this death**, and they are a *different
exception type* from #1..#6 - the `ThrowInfo` pointer on the stack is `0x143a3b118` for #7/#8
and `0x143a3b0c0` for #1..#6. **[L]**

Every frame below is confirmed two ways: `tools/pdata_lookup.py` puts the address inside the
function, and the *data* addresses in the same stack scan are the exact rip-relative operands
of the instructions in that function. That second check is what makes the walk a measurement
rather than a guess, because the hook's stack line is a raw scan and carries stale slots.

```
23:36:23.880  C++ THROW #7  tid 127992
  0x143a3b118   ThrowInfo for ZException   <- lea rdx,[rip+0x223673a] at 0x1418049d7
  0x1433d5e98   "throw ZException"         <- lea rdx,[rip+0x1bd14f7] at 0x14180499a
  0x142ef6ddc   _CxxThrowException, the instruction after RaiseException(0xE06D7363)
  0x1418049e8   int3 after `call _CxxThrowException` in the ZException throw helper
  0x1433d5e74   "HR"                       <- lea rax,[rip+0x1bd14b4] at 0x1418049b9
  0x1401b0174   return address after `call 0x1401b00e0` -- THE SECURED-VALUE VERIFIER
  0x143271f04   "-"                        <- lea rcx,[rip+0x30c1d95] at 0x1401b0168
  0x142671621   return address after `call 0x1401b00e0` from FUN_14266f2d0 + 0x2351
  0x143acedf0   the static sprintf scratch buffer at 0x141803bda
  0x142a23947   inside the client's own log-file appender FUN_142a23830
```

Named:

| address | what it is | how it is known |
|---|---|---|
| `0x142ef6d4c` | `_CxxThrowException` | `mov edi,0x19930520` / `mov ecx,0xE06D7363` / `RaiseException` **[L]** |
| `0x141804970` | `throw ZException(file, line, code, msg)` | passes `"throw ZException"` and a `ThrowInfo` to the above; `int3` after **[L]** |
| `0x141804b50` | the ELog message formatter it calls first | it passes `0x143296d2c` = `"ELog"` on to `FUN_141804dc0`; the `"%s%s%s"` in the same block is at `0x143296d50` **[L]** |
| `0x1418039d0` | error-code -> name table | `0x20000000`/`0x21000000+n`/`0x22000000+n`; code `5` is **not** in it, so it takes the `"Etc:0x%X"` default at `0x1433d5c30` **[L]** |
| `0x1401b00e0` | **secured-`u16` decode with integrity check** | see below **[L]** |
| `0x14266f2d0` | the **pet** item tooltip body, 13 834 bytes | reached only from `FUN_142694130+0x946`, on the classifier's type-3 arm **[L]**; "pet tooltip" as a name is **[I]** |
| `0x142a23830` | appends a line to a text file (`CreateFileW` GENERIC_WRITE, seek END, `WriteFile`) | **[L]** |

`FUN_1401b00e0`, in full, because everything turns on it:

```
0001401b00e8  movzx r11d, byte [rcx]        ; key0
0001401b00ec  movzx r10d, byte [rcx+2]      ; enc0
0001401b00f1  xor   al, r11b                ; out byte 0 = enc0 ^ key0
0001401b00fc  movzx r8d,  byte [rcx+1]      ; key1
0001401b0101  movzx r9d,  byte [rcx+3]      ; enc1
0001401b010a  xor   al, r8b                 ; out byte 1 = enc1 ^ key1
0001401b0111  xor   r11d, 0xbaadf00d
0001401b0118  ror   r11d, 5
0001401b011c  add   r11d, r10d
0001401b011f  xor   r11d, r8d
0001401b0122  ror   r11d, 5
0001401b0126  add   r11d, r9d
0001401b012e  cmp   r11d, edx               ; edx = the stored checksum
0001401b0131  je    ok
              ...  throw ZException, code 5, "-":83 ...
0001401b0189  movzx eax, word [rsp+0x40]    ; the decoded u16
```

Signature: `u16 GetSecured(const u8 enc[4], u32 checksum)`. Its matched writer is
`FUN_1402f70a0` - two RNG key bytes, `enc[i] = key[i] ^ val[i]`, and the identical
`0xBAADF00D / ror 5 / add` accumulator returned as the checksum. Encoder and decoder agree
instruction for instruction. **[L]**

The call site that threw:

```
00014267160e  mov  rax, qword [rbp+0x208]   ; arg3 = the item object
000142671615  lea  rcx, [rax+0x7a]          ; the 4 encoded bytes
000142671619  mov  edx, dword [rax+0x7e]    ; the checksum
00014267161c  call 0x1401b00e0
```

`rbp+0x208` is arg3 of `FUN_14266f2d0` (`lea rbp,[rsp-0x1a8]` after 8 pushes puts the arg
home area at `rbp+0x1f0`). At the one call site, arg3 is `FUN_140192f80(r12)`, which returns
its own argument unchanged - so **arg3 is the item slot object**, the same `r12` whose secured
item id at `r12+0x20` the caller had just decoded. **[L]**

Throw #8, 2 ms later, is a second `ZException` through a different wrapper
(`FUN_142c879a0` -> the same `FUN_141804970`), with the same `ThrowInfo` and the same `"HR"`
tag. Its caller is outside the 16-slot scan window, so it is **not** identified. Most likely
the unwind/report path throwing on the way out - **[I]**, and nothing here depends on it.

---

## 2. Why it threw: the class the factory built is not the class the UI read

Two independent dispatches, and they disagree.

**The factory uses the type byte on the wire.** `FUN_1403095e0` reads a `u8` at `0x1403095fb`
and allocates from one of three pools, then calls `vtable+0x358` (the class `Decode`): **[L]**

| type byte | pool | allocator | size | ctor | decode |
|---:|---|---|---:|---|---|
| 1 | `ctx+0` | `FUN_14030ddb0` | - | - | equip |
| 2 | `ctx+0x970` | `FUN_14030db00` | `mov edx,0x7e` | `FUN_1402f7cd0` | `FUN_140304450` |
| 3 | `ctx+0x12e0` | `FUN_14030e340` | `mov edx,0xce` | `FUN_1402f8b40` | `FUN_140304550` |

**The UI re-derives the class from the item id.** `FUN_142694130` (5 561 bytes, 98 call sites
in 92 functions - the shared "build the item tooltip" helper; it is the function that carries
the strings `"addTooltip_tuc"` and `"addTooltip_tucCnt"`) decodes the item's secured id from
`item+0x20`, calls `FUN_1401b1040`, and switches: **[L]**

```
1 -> 0x14269541e        2 -> 0x142694a80        3 -> 0x14266f2d0        else -> 0x142645170
```

`FUN_1401b1040` is the item-slot classifier. Divide by 1 000 000, index the 6-entry table at
`0x1401b1094`: **[L]**

```
1xxxxxx -> 1        2xxxxxx,3xxxxxx,4xxxxxx -> 2        6xxxxxx -> 1
5xxxxxx -> if (id - 5000000) < 10000  ->  3   else  ->  2
```

`5000001 - 5000000 = 1 < 10000`, so **the tooltip took the type-3 arm on a type-2 object.**

The pet decode `FUN_140304550` is the *only* thing that ever writes `+0x7a`/`+0x7e` as a
secured pair: **[L]**

```
00014030462e  call 0x1406e8b80              <<< READ u16 off the wire
000140304633  lea  rdx, [rsi+0x7a]
00014030463a  call 0x1402f70a0              ; encode into +0x7a..+0x7d, return checksum
000140304642  mov  dword [rsi+0x7e], eax    ; checksum at +0x7e..+0x81
```

In the object we actually made, `+0x7a..+0x7d` is the bundle's plain trailing `u32` - which
our body sends as `0` - and `+0x7e..+0x81` **is not part of the allocation at all**. The
bundle constructor's last write is `MOV [RSI+0x7a],EAX` and the allocation is `0x7e` bytes, so
the object ends exactly there. **[L]**

The arithmetic is decidable: for four zero bytes the verifier computes
`ror(ror(0xBAADF00D,5),5) = 0x036EAB7C`, and compares it against whatever four bytes follow a
126-byte heap block. A match is a 1-in-2^32 accident. **This throw is deterministic, not a
coincidence of corruption.** **[L]** for the constant, **[I]** that no heap layout makes those
four bytes `0x036EAB7C`.

Note the direction: this is an out-of-bounds **read**, not a write. It cannot be the source of
the heap-corruption family this project has been chasing, and that family is not needed to
explain it.

---

## 3. Is the server's Cash-tab item body wrong?

**Yes - but not in the way the brief guessed.** Three separate claims, separated:

### 3.1 The cash serial: NOT wrong

The base decode `FUN_1403035a0` reads `u8 hasCashSN`, and the zero branch is explicit:

```
000140303787  call 0x1406e8ae0      <<< READ u8   hasCashSN
00014030378c  test al, al
00014030378e  je   0x1403037a4
000140303790  mov  r8d, 8
000140303796  lea  rdx, [rbx+0x38]
00014030379d  call 0x1406e9170      <<< READ raw[8]   -> +0x38
0001403037a2  jmp  0x1403037ac
0001403037a4  mov  qword [rbx+0x38], 0               ; <- the client zeroes it itself
```

`hasCashSN = 0` with no serial is one of the two encodings the client accepts, and it is what
we send. **[L]** Sending a serial would have made the body 8 bytes *longer*, not fixed
anything.

### 3.2 The expiry FILETIME: NOT wrong, and NOT missing

`0x1403037b9` reads `raw[8]` into `+0x40` unconditionally, and our 41-byte body carries
`00 80 05 bb 46 e6 17 02` = `net::opcode::ITEM_NEVER_EXPIRES` in exactly that position.
It is the same constant the client itself has baked at `0x143296d38`. **[L]**

### 3.3 The type byte and the whole body shape: WRONG for this id

Our blob, byte for byte (from `world.log`, `0x0070` body offset 11 onward):

```
02              type = 2, bundle                          <-- WRONG, must be 3 for 5000000..5009999
41 4b 4c 00     itemId 5000001
00              hasCashSN
00 80 05 bb 46 e6 17 02   dateExpire            -> +0x40
00 00 00 00     u32                             -> +0x48
00              u8                              -> +0x4c
01 00           u16 quantity = 1                -> +0x4d/+0x51   (a pet's NAME starts at +0x4d)
00 x13          owner name buffer               -> +0x6d
00 00           u16 attribute                   -> +0x55/+0x59
00              u8                              -> +0x5d
00 00 00 00     u32                             -> +0x7a
                                                    41 bytes
```

`crates/world/src/session/inventory.rs::item_blob` has exactly two arms, `Equip` and
`Bundle`, because `store::ItemKind` has exactly two variants. Every non-equip item is emitted
as a type-2 bundle, pet ids included. `crates/net/src/inventory.rs:187` already records the
right rule - *"The blob's own leading u8 selects the decode: 1 equip, 2 bundle, 3 pet"* - and
`research/bag-lists.md` §5.3 already found the pet decode and set it aside as *"not needed"*.
It became needed the moment `!buy`/`!locker` could hand a 5000xxx id to the bag.

**Where the 41 bytes fall short: they are 26 bytes short, and every field after the base is at
the wrong offset.** The type-3 body the client wants, read off `FUN_140304550` with
`tools/reads.py 0x140304550 3` and the listing (**[L]** for every row):

```
  1  u8    type = 3                                    1403095fb
  ---- base, FUN_1403035a0, 18 bytes when hasCashSN = 0 ----
  4  u32   itemId                                      1403035c5
  1  u8    hasCashSN                                   140303787
 [8] raw   cashSN, only when hasCashSN != 0            14030379d  -> +0x38
  8  raw   dateExpire                                  1403037b9  -> +0x40
  4  u32                                               1403037c1  -> +0x48
  1  u8    (stored as a bool)                          1403037cc  -> +0x4c
  ---- pet, FUN_140304550, 48 bytes ----
 13  raw   pet name, NOT length-prefixed               140304577  -> +0x4d  raw[13]
  1  u8    (level)                                     14030457f  -> secured +0x5a, cksum +0x5e
  2  u16   (tameness)                                  1403045b5  -> secured +0x62, cksum +0x66
  1  u8    (repleteness)                               1403045cc  -> secured +0x6a, cksum +0x6e
  8  raw   (dateDead)                                  14030460f  -> +0x82  raw[8]
  2  u16                                               140304617  -> secured +0x72, cksum +0x76
  2  u16                                               14030462e  -> secured +0x7a, cksum +0x7e   <<< THE FIELD THAT THREW
  4  u32                                               140304645  -> secured +0x8a, cksum +0x92
  2  u16                                               14030467e  -> secured +0x96, cksum +0x9a
  1  u8                                                14030469b  -> secured +0x9e, cksum +0xa2
  4  u32                                               1403046da  -> secured +0xa6, cksum +0xae
  2  u16                                               140304713  -> secured +0xb2, cksum +0xb6
  2  u16                                               140304730  -> secured +0xba, cksum +0xbe
  4  u32                                               14030474d  -> secured +0xc2, cksum +0xca
                                                       total 67 bytes (75 with a cash serial)
```

The last checksum ends at `0xca + 4 = 0xce`, which is the pet allocation size to the byte -
an independent check that this field walk is complete and correctly ordered. The field
*names* in brackets are **[I]**, taken from the MapleStory pet layout; the offsets, sizes,
order and secured/plain distinction are all **[L]**.

Two ways to stop the crash, and only one of them needs the table above:

1. **Cheap and safe:** refuse to put an id in `5000000..=5009999` in a bag at all - in
   `gm_locker`, or in `item_blob`, or in the store. Nothing else in the game hands out a pet
   today; `!buy` is the only door and it is a GM command.
2. **Correct:** add a `store::ItemKind::Pet` and a `net::bag::pet_item` that emits the 67-byte
   type-3 body above. Note this also needs the tooltip to survive it, which is untested.

---

## 4. (a) or (b): the evidence says (a), and there is a control

`world.log` and the hook log agree on the shape, and the discriminating facts are:

* **The `0x0070` handler completed.** `202 opcode=0x0070 elapsed_us=149.0 ret=1` at
  23:36:20.510. The dispatch line is written on *return*, so the decode itself was fine -
  which is expected: a type-2 body is a valid type-2 body. The crash is downstream of the
  decode.
* **The throw is not inside any handler.** Dispatch #203 returned at 23:36:22.551 and #204 at
  23:36:23.061; the throw is at 23:36:23.880, **819 ms after the last handler returned**, on
  the same thread (127992). The two `0x0453` we sent at 23:36:24.593 and 23:36:26.628 have no
  dispatch line at all. So the throw came out of the client's own update/render loop, not out
  of packet processing. **[L]**
* **The stack lands in a tooltip builder, reached only through the item-id classifier's pet
  arm.** Section 2.
* **The control: `ADD: item 5xxxxxx` appears exactly once in the entire archive, and it is
  this run.** Grepping every `.log` in `previous-runs/` and `research/fixtures/` for
  `ADD: item 5[0-9]{6}` returns **1** hit - 23:36:20.500, the one that died. `into Cash slot`
  returns the same single hit. Every other `0x0070` ADD this project has ever sent carried a
  `1xxxxxx` equip (129-byte blob) or a `2xxxxxx`/`4xxxxxx` bundle (41-byte blob), and none of
  those is in the pet range. **The first time a pet-range id was ever put in this client's bag
  is the run that died.** **[L]**
* Corroborating but weak: `client-exit.log` shows CPU going from 17.91 s at +226.2 s to
  18.64 s at +231.4 s and the working set 386 -> 394 MB across the throw, and a thread started
  at 23:36:21.614 - i.e. the client loaded something new in the seconds before it died. That
  is consistent with a tooltip pulling in pet resources; it is not evidence on its own. **[I]**

Against (b) - "234 s of session, something else fired": nothing in the corruption family's
signature is present. The heap deaths in this project raise `0xC0000005` and are logged as
`CLIENT FAULT`; this is `0xE06D7363`, a deliberate C++ throw, from a verifier that names its
own reason, on an offset that a single static chain predicts in advance. The 3.37 s delay,
which is the one thing that superficially favours (b), is fully accounted for by the tooltip
being a **hover**-triggered build rather than a per-frame one.

**The one link I have not measured is exactly that: what makes the tooltip run.** That
`FUN_142694130` is *the item tooltip builder* rests on two strings inside it
(`"addTooltip_tuc"`, `"addTooltip_tucCnt"`) and on `FUN_14266f2d0` opening with a
(arg1, arg2) result cache at `[rcx+0x48]/[rcx+0x80]/[rcx+0x84]` - the shape of a "don't rebuild
what you already built". That is **[I]**, not **[L]**. I have not traced a mouse-move handler
down to it, and I did not open Ghidra (another agent holds the lock). If it turns out to run
on the inventory *draw* rather than on hover, the 3.37 s needs a different explanation and
this section is weaker - but sections 1-3 do not move.

### The one cheap client observation

One launch, one variant, at **~40 s of client life** so "this thing" is separated from "this
session" the way `CLAUDE.md` requires:

> `!item 5000001` (or `!nx`/`!buy` then `!locker 1`), then **do not move the mouse for 30 s**,
> then hover the Cash tab's slot 1.

| what happens | what it means |
|---|---|
| dies within seconds of the ADD, mouse untouched | (a), and the trigger is the inventory *draw*, not the tooltip - my `[I]` on `FUN_142694130` is wrong about the trigger but right about the path |
| survives 30 s untouched, dies within ~1 s of the pointer reaching the icon | (a) confirmed **and** the tooltip trigger confirmed. This is the outcome the static chain predicts |
| survives both, for minutes | the chain in §2 is wrong somewhere and (b) is back. Re-open it |

A second run, only if the first is ambiguous: the same thing with `!item 5010000` - one digit
outside the classifier's pet window, so `FUN_1401b1040` returns 2 and the *bundle* tooltip
runs on a bundle object. It must **not** die. That turns `id - 5000000 < 10000` from a
listing into an on-screen measurement.

---

## 5. Loose ends, named

* **Throw #8 is not attributed.** Its wrapper is `FUN_142c879a0`; its caller was outside the
  scan window. Assumed to be the unwind/report path - **[I]**, unverified.
* **The ELog line the client wrote is not on disk anywhere I looked.** `FUN_141804970` formats
  an ELog entry (tag `"ELog"` at `0x143296d2c`, category `0xf`, file `"-"`, line `83`,
  code `5` which is *not* in `FUN_1418039d0`'s name table so it renders as `Etc:0x5`) and
  `FUN_142a23830` appends it to a file. Nothing under `client-patched\` or
  `C:\Nexon\Library\maplestorycw` was modified after 23:30 except our own hook files, and no
  `0x008F`/`0x0090` ELog upload reached `world.log` - the client died before it could send.
  If that file can be found, it carries the exception's own text.
* **The meaning of the u16 at pet `+0x7a`** is not decoded. The tooltip immediately does
  `and ax, FUN_1426dc0d0(0)` - an 11-entry bit-mask table `{1,0x20,2,4,0x40,8,0x80,0x100,
  0x200,0x400}` - and pairs it with `FUN_1426dc320`, an 11-entry field selector on a
  *different* object. A flag word of some kind. Not needed for anything above.
* **`FUN_1403095e0`'s equip arm** (type 1, pool at `ctx+0`) was not sized; only types 2 and 3
  were, because only those two are in this chain.
* Ghidra was not used - the project lock is held elsewhere. Everything here is from
  `tools/pdata_lookup.py`, `tools/listing.py`, `tools/reads.py`, `tools/callers.py`,
  `tools/rtti.py` and `tools/dump_va.py`, all run with the repo as the working directory.
