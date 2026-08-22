# Equipping killed the client. What the listing says, and what it does not.

**2026-08-21.** Written against `world.log` / `client-patched\maplecw-hook.log` /
`client-exit.log` of the 02:07:57–02:11:58 run, and against a fresh listing and
decompilation of `FUN_142d51930` and everything mode 2 calls.

That run is kept as `research/fixtures/equip-into-empty-hat-slot-kills-client-{world,hook,
exit,login}.log`; **cite those, not the live files**, because the live ones are a rolling
buffer. The comparison run is `previous-runs/world-20260820-012657.log`, which has not been
promoted to a fixture and will roll — it is the only capture of 6 successful equips and 7
successful unequips and it is worth keeping.

New files this pass produced, all `//===`-headed:

| file | what is in it |
|---|---|
| `research/msexe-invop-0070.txt` | `DumpAsm` of the whole handler, `142d51930 .. 142d546f0`, bounded by `.pdata` |
| `research/msexe-invslot-access.c` | `FUN_1402e3cd0` GetItem, `FUN_1402e4c20` SetItem, `FUN_1402e5020`, `FUN_14030ee00`, `FUN_142cbe730` |
| `research/msexe-equip-apply.c` | the seventeen functions mode 2 reaches: the slot resolver, the ZRef primitives, the two UI calls, the allocator and the deallocator |
| `research/msexe-equip-avatar.c` | `FUN_14019b4e0` (the deallocator that faulted), `FUN_1428336c0`, `FUN_142ce51b0`, `FUN_140391940` |
| `research/msexe-equip-item-dtor.c` | `FUN_1402fb5c0` (the equip's scalar-deleting destructor, vtable[0] of `0x14327e1d8`), `FUN_1402cc180` (the factory, and the item's `0x467`-byte size), `FUN_1402f7da0` (the type-1 constructor) |

---

## 0. The headline, before the detail

**The packet we sent is correct.** Every field, every width, the entry count and the single
trailing byte all match what `FUN_142d51930` reads, checked instruction by instruction
against the listing, and the byte-for-byte difference between the fatal equip and a
*known-good* equip from 2026-08-20 is **two bytes: `newPos`**. **[L]**

```
2026-08-20 05:20:46  0100010000000002010400 f5ff 00   worked   (bag 4 -> worn -11)
2026-08-21 02:11:58  0100010000000002010400 ffff 00   killed   (bag 4 -> worn  -1)
```

**The worn slot the item went into cannot be the cause.** The worn inventory is a **fixed,
inline array of 31 `ZRef`s inside the character-data object**, `charData + 0x1a8 + slot*0x10`,
item pointer at `+8`. Nothing about it is allocated, grown, or sparse; slot 1 is as real as
slot 11 and both the getter and the setter bounds-check to `1..=31`. There is **no path that
reads or frees a slot the client never allocated**, because there is no such thing as an
unallocated worn slot. **[L]** — section 2.

**What faulted is a free of a block larger than 0x80 bytes, through one specific
deallocator, and it is not the item object.** `0x14019b58e` is the return address of the
`call rbx` at `0x14019b58c` inside `FUN_14019b4e0`; that branch is taken only when the block
header at `ptr-8` says the block exceeds `0x80` bytes — smaller ones are pushed onto an
internal free list at `DAT_143ad6908` and never leave the image. **[L]** The equipped-item
object is `0x467` = 1127 bytes, but its destructor frees it through
`thunk_FUN_140205820 -> FUN_14019bb50`, a **different** wrapper, so the freed block was not
an item. **[D]** And this is a different deallocator from the 2026-08-20 22:46 heap death,
which faulted in `FUN_14019bb6a`. **[L]**

Even so, the fault line names the walk and not the damage — exactly what
`research/heap-corruption.md` says it always will.

So the honest verdict is:

> **The equip is a *candidate trigger* for the pre-existing `0xC0000374`, not a demonstrated
> new bug, and nothing in the packet needs changing.** The one thing that has genuinely
> changed is that this bug may finally have a **deterministic reproducer**, which
> `heap-corruption.md` says outright it has never had. Section 6 is a single one-variant run
> that settles it. **[D]**

The user's actual request — *equipping over a worn item should swap it back into the bag* —
is answered and is **not blocked**: mode 2 is already a two-way swap and the packet for it is
the same 14 bytes we send today. Sections 4 and 5.

---

## 1. `FUN_142d51930` mode 2, in full

Entry reads, from `tools/reads.py 0x142d51930` (17 read sites in the whole handler; the
"18" in `research/msexe-setfield.md` is one too many — see section 3):

```
142d51b88  u8   mode        -> 2
142d51b94  i8   invType     -> 1
142d51ba4  i16  oldPos      -> 4       (MOVSX, so signed)
142d52528  i16  newPos      -> -1      (MOVSX, R13D)
```

Registers through the case, off the listing: `R12D = invType`, `R14D = oldPos`,
`R13D = newPos`. **[L]**

### 1.1 There is no "`newPos < 0` arm" and no "`oldPos < 0` arm"

This is the single most useful thing in this document. **Mode 2 has exactly one code path.**
An equip and an unequip are the *same instructions* with the two positions in the other
order. From `142d52566` onwards:

```asm
142d52531  CMP R12D,0x1 / JZ ; CMP R12D,0x6 / JNZ     ; invType == 1 || 6
142d5253d  TEST R14D,R14D / JS                        ;   && (oldPos < 0
142d52542  TEST R13D,R13D / JNS                       ;       || newPos < 0)
142d52547  MOV ESI,0x1 / MOV [RSP+0x34],ESI           ;   -> avatarChanged = 1

142d52557  R9D=R13D(newPos) R8D=R12D RDX=&[RBP-0x48]  ; GetItem(invType, newPos)
142d52566  CALL 0x1402e3cd0                           ;   -> DEST item at [RBP-0x40]
142d5256c  R9D=R14D(oldPos) R8D=R12D RDX=&[RBP+0x8]   ; GetItem(invType, oldPos)
142d5257b  CALL 0x1402e3cd0                           ;   -> SRC  item at [RBP+0x10]
...
142d52bc5  MOV [RBP+0x1e8],RBX          ; RBX = DEST item, into a ZRef at RBP+0x1e0
142d52bfe  LEA R9,[RBP+0x1e0] / R8D=R14D(oldPos)
142d52c13  CALL 0x1402e4c20             ; SetItem(invType, oldPos, DEST item)
142d52c1c  MOV [RBP+0xf8],RBX           ; RBX = SRC item, into a ZRef at RBP+0xf0
142d52c55  LEA R9,[RBP+0xf0] / R8D=R13D(newPos)
142d52c65  CALL 0x1402e4c20             ; SetItem(invType, newPos, SRC item)
```

**Mode 2 is an unconditional two-way swap**: whatever is at `newPos` is written to `oldPos`,
and whatever is at `oldPos` is written to `newPos`. When the destination is empty the "swap"
writes a null `ZRef` into the source slot, which is a plain move. **[L]**

Everything downstream is symmetric in the same way:

| site | equip (oldPos=4, newPos=-1) | unequip (oldPos=-5, newPos=1) |
|---|---|---|
| `142d52547` avatarChanged | 1 | 1 |
| `SetItem(oldPos, dest)` | bag slot 4 ← null | worn slot 5 ← whatever was in bag 1 |
| `SetItem(newPos, src)` | worn slot 1 ← the hat | bag slot 1 ← the coat |
| `FUN_142da62e0(cd, srcItem, x)` | `x = 1` (`newPos < 0`) | `x = 0` |
| `FUN_142da62e0(cd, destItem, y)` | not called, dest null | not called, dest null |

The only genuine asymmetry is that third argument, and it selects between two **string
resource ids** inside `FUN_142da62e0` — `0x1287` when equipping, `0x1288` when unequipping
(`FUN_1408a9e40(&local, 0x1287/0x1288)`). Both branches ran, without a fault, in the
2026-08-20 01:26 run (6 equips, 7 unequips). **[L]**

### 1.2 Where mode 2 allocates and frees

Every allocation and deallocation reachable from mode 2, enumerated from the listing rather
than from the decompiler (the decompiler drops the size argument to the allocator, which is
exactly the field that matters here):

| site | what | size |
|---|---|---|
| `142d5261e`, `142d5270e` | grow the "items touched" `u32` array | `LEA RDX,[0x8 + RDX*0x4]` — **8 + n*4, correct**, and the element store at `142d52753` is `MOV [RDI + RAX*4], R15D` with `RAX = MOVSXD R14D`. No overrun. **[L]** |
| `142d52649`, `142d52739` | free the old copy of that array | `0x142ef3bb8`, **not** the faulting deallocator |
| `142d5289f`, `142d52acf` | rehash: new bucket array | `n*8`, `n` from the prime table at `DAT_1434956a0` |
| `142d5290a`, `142d52b3a` | rehash: free the old bucket array | `FUN_14019b4e0` |
| `142d5294e`, `142d52b82` | one hash node | `FUN_14036f510(0x18)` |
| `142d54509`, `142d5455d` | end of handler: free both bucket arrays | `FUN_14019b4e0` |
| `142d54572` | end of handler: free the `u32` array | `0x142ef3bb8` |
| inside `FUN_1401abd80` | **an item object**, when its refcount reaches zero | the item's own scalar-deleting destructor → `operator delete` |

The hash block (`142d5275b`–`142d52bb3`) is entered only when `oldPos > 0 || newPos > 0`
**and** the moved item's id equals `this+0x303c` and its position equals `this+0x3040` — a
"currently tracked item" latch. Both halves are client state we cannot see. **[L]** for the
condition, **[I]** for what the latch is.

### 1.3 The refcounting is balanced, and I traced it

`FUN_1402e3cd0` (GetItem) nets **+1** on the item when it finds one and does nothing when it
does not. `FUN_1401e8780(dst, src)` AddRefs `src->ptr`, Releases `dst`, then `dst->ptr =
src->ptr`. `FUN_1401abd80(ref)` decrements and, on the transition to zero, calls
`ref->ptr->vtable[0](ptr, 1)` — the scalar-deleting destructor. **[L]**

For the fatal equip, with `R` the hat's refcount:

```
bag slot 4 holds it                                       R = 1
GetItem(newPos=-1)  -> null, nothing
GetItem(oldPos=4)   -> local_378 = hat                    R = 2
SetItem(oldPos=4, null)   releases bag slot 4             R = 1
AddRef before SetItem(newPos)                             R = 2
SetItem(newPos=-1, hat)   AddRef, then release temp       R = 2
end of case: release local_378                            R = 1  (held by worn slot 1)
```

Balanced, and identical in shape to the unequip. **No item object is destroyed by this
packet.** **[D]** — derived from the four decompiled primitives plus the listing's call order.

---

## 2. Empty destination versus occupied destination

Both the getter and the setter resolve a worn slot through the same arithmetic.

`FUN_1402e3770(charData, invType, pos)` — the slot **resolver** (`research/msexe-equip-apply.c`):

```c
if (invType == 1 && pos < 0) {
    slot = -pos;
    if (FUN_140302620(slot) != 0)          // slot in 3000..3031 / 3100..3131 / 3200..3231
        return FUN_1402de170(charData + 0x5a8, slot);   // a map, for those ranges only
    if (0x1e < (unsigned)(pos + 0x1f)) return 0;        // BOUNDS: slots 1..31 only
    return charData + 0x1a8 + slot * 0x10;              // fixed inline array
}
```

`FUN_1402e4c20` (SetItem) has the byte-identical guard and then
`FUN_1401e8780(charData + 0x1a8 + slot*0x10, ref)`. **[L]**

Three things follow, and they close question 2:

1. **The worn array is inline in the character-data object and is 31 entries of 16 bytes,
   `0x1a8 .. 0x3a8`.** The Decoration array is the next 31 entries, `0x3a8 .. 0x5a8`
   (`charData + 0x3a8 + (-100 - pos)*0x10`, guarded by `0x1e < (unsigned)(pos + 0x83)`), and
   `0x5a8` is the map used for the 3000/3100/3200 ranges. The three regions tile exactly.
   **[L]** Nothing here is allocated, grown, or sparse.

2. **`FUN_140302620(1)` returns 0**, so worn slot 1 takes the plain array path, not the map
   path. **[L]**

3. **`pos = -1` is inside the bound, on the last accepted value.** `(-1 + 0x1f) = 0x1e`, and
   the test is `0x1e < x`, so it passes; `pos = 0` gives `0x1f` and is rejected; `pos = -32`
   wraps to `0xffffffff` and is rejected. Slots 1..31. **[L]**

**An empty destination differs from an occupied one only in whether `ZRef.ptr` is null**, and
both `FUN_1402e3cd0` and `FUN_1401abd80` test for null explicitly before touching anything.
There is no read or free of an uninitialised slot. **[L]**

### 2.1 The array is initialised, and the proof is in the character-record decoder

The post-decode pass of `FUN_140304b20` (`research/msexe-charrecord-equip.txt`) walks the
worn array **from slot 1**:

```asm
140305794  MOV R15D,0x1                    ; slot = 1
14030579a  MOV R12,qword ptr [RSP + 0x40]  ; R12 = charData
14030579f  LEA RBX,[R12 + 0x1c0]           ; = 0x1a8 + 1*0x10 + 8  -> worn slot 1's item ptr
1403057b1  MOV RCX,qword ptr [RBX]         ; the item
1403057b4  TEST RCX,RCX / JZ               ; null -> skip
1403057bd  ADD RCX,0x20 / CALL 0x14019a5d0 ; else deobfuscate its itemID
...
140305923  INC R15 / ADD RBX,0x10 / CMP R15,0x1f / JLE
```

Independent confirmation of the `0x1a8 + slot*0x10 (+8)` layout **and** of the 1..31 range,
from a completely different function. **[L]**

And it is the reason "slot 1 held garbage" is not a live hypothesis: **`SetField` ran three
times in this session** (06:08:22, 06:10:02, 06:10:36) and this loop dereferences worn slot 1
on every one of them. If it held garbage the client would have died at map entry, not 100
seconds later. **[D]**

---

## 3. The `avatarChanged` tail: exactly one byte, and we send exactly one

`tools/reads.py 0x142d51930` — the authority, because it disassembles rather than greps,
counts a tail `jmp` into a read primitive, and merges contiguous `.pdata` entries — reports
**17** read sites, 14 direct and 3 through the item-blob helper `FUN_140303530`:

```
142d51973 u8   142d51989 u8   142d51b2b u32  142d51b3b u8      <- header
142d51b88 u8   142d51b94 u8   142d51ba4 u16                    <- entry
142d51bd9 blob(mode 0)   142d531eb blob(mode 5)   142d53917 blob(mode 11)
142d521fe u16 (mode 1)   142d52528 u16 (mode 2)   142d53187 u64 (mode 4)
142d532e0 u32 (mode 7)   142d5342e u16 (mode 8)   142d5380f u16 (mode 10)
142d5219f u8                                                   <- THE TAIL
```

`research/msexe-setfield.md` says 18. It is one over; the site list above is what the
instrument prints and it is the only list any conclusion should rest on. Correcting that
document is left to whoever owns it.

**Mode 2's only tail read is the `i16` at `142d52528`. Nothing else.** And the trailing byte,
straight from the listing:

```asm
142d52168  MOV R12D,dword ptr [RSP + 0x34]   ; avatarChanged
142d52176  DEC R15D / TEST R15D,R15D / JG 0x142d51b80   ; entry loop
142d52187  TEST R12D,R12D
142d5218a  JZ  0x142d53c65                  ; not set -> read nothing more, ever
142d52190  MOV RBX,qword ptr [0x143aa8518]
142d52197  TEST RBX,RBX / JZ 0x142d521af    ; local user null -> read nothing more
142d5219f  CALL 0x1406e8ae0                 ; ONE u8
142d521aa  CALL 0x1428a7f00                 ; -> vtable+0x110(localUser, thatByte)
142d521af  ...                              ; no further read primitive in the function
```

**One `u8`, once, after the loop. Our body is 14 bytes and the client reads 14 bytes.
This is not a short packet.** **[L]**

`FUN_1428a7f00(user, b)` is four instructions of forwarding:
`if (FUN_14279c310() == 0) { p = (*(user+8))->vtable[0x50](user+8); p->vtable[0x110](p, b); }`
— it hands the byte to the avatar object and consumes nothing. Sending `0` is fine. **[L]**

---

## 4. Equipping over a worn item: **one packet, one entry, the client swaps**

This is the question `crates/world/src/session/inventory.rs` currently refuses on, with a
comment pointing here. The answer is in section 1.1 and it is **[L]**:

> **`nCount = 1`, one mode-2 entry, the same 14 bytes as an equip into an empty slot.**
> `142d52c13` writes the item that was at `newPos` into `oldPos`, and `142d52c65` writes the
> item that was at `oldPos` into `newPos`. The client puts the displaced item back into the
> source bag slot **by itself**, in the same entry.

The worry recorded in that comment — *"a two-entry `0x0070` collides with itself in both
possible orders"* — is correct, and it is the reason there must **not** be a second entry.
Two mode-2 entries `(4 → -1)` then `(-1 → 4)` would swap and then swap straight back.

What changes when the destination is occupied is only that more of the same code runs:
both `local_3c8` and `local_378` are non-null, so both item ids are appended to the
"items touched" array (`142d5261e` **and** `142d5270e`), both get a
`FUN_142ce53e0` refresh and, under `avatarChanged`, both get a `FUN_142d9bfd0` and a
`FUN_142da62e0` — the destination item with `param_3 = (oldPos < 0)`, i.e. `0`, which is the
"unequipped" string. Still one trailing byte. Still 14 bytes. **[L]**

The server's only remaining job is to make its own database say the same thing, which
`Store::equip_from_bag` already does.

**Not established [I]:** whether a real server also sends a `0x007C` StatChanged after an
equip. Nothing in this handler requires one, and the client recomputes its own stats from the
worn array; but that is an argument from absence in one function, not a measurement.

---

## 5. WIRE IT LIKE THIS

Both cases are the same fourteen bytes. `EQUIP_SLOT` is the worn slot as a **negative**
`i16`; `BAG_SLOT` is the 1-based Equip-tab slot as a positive `i16`.

### (a) Equip into an empty worn slot — and (b) equip over an occupied one

```
off  size  value        field                     why
---  ----  -----------  ------------------------  ----------------------------------------
  0  u8    0x01         bExclRequestSent          142d51973. Non-zero runs FUN_142cc4430,
                                                  which clears player+0x2330. A 0 here
                                                  kills every later inventory action.
  1  u8    0x00         (unknown)                 142d51989. Held to the end of the handler;
                                                  non-zero fires UI message 0x58f.
  2  i32   0x00000001   nCount                    142d51b2b, MOVSXD. FOUR bytes, not one.
  6  u8    0x00         notRemoveAddInfo          142d51b3b. Only mode 3 reads it.
--- entry 0 ---------------------------------------------------------------------------
  7  u8    0x02         mode = Move               142d51b88
  8  i8    0x01         invType = Equip           142d51b94, MOVSX
  9  i16   BAG_SLOT     oldPos  (e.g. 0x0400 = 4) 142d51ba4, MOVSX
 11  i16   EQUIP_SLOT   newPos  (e.g. 0xffff = -1)142d52528, MOVSX
--- tail ------------------------------------------------------------------------------
 13  u8    0x00         addMovementInfo           142d5219f. Present because
                                                  (invType==1||6) && (oldPos<0||newPos<0)
                                                  is true at 142d52531..142d52547.
                                                  Body length 14.
```

Concretely, bag slot 4 into worn slot 1 (the packet already sent, and it is right):

```
01 00 01 00 00 00 00 02 01 04 00 ff ff 00
```

An unequip is the same fourteen bytes with the two positions exchanged — `net::inventory::
inventory_move_result` already builds all of these correctly, including dropping the trailing
byte for a bag-to-bag move where `move_changes_the_avatar` is false. **Nothing in
`crates/net/src/inventory.rs` needs to change.**

The only server-side change section 4 licenses is in
`crates/world/src/session/inventory.rs`: **delete the "worn slot is occupied" refusal** and
let `Store::equip_from_bag` run, because the wire half needs no second entry. That is a
change for whoever owns that file; this document does not touch `crates/`.

---

## 6. So what killed it — and how to find out in one launch

### 6.1 What is established

* The client entered `FUN_142d51930` and did not return: the hook writes its dispatch line
  *after* the handler returns, twenty-two `opcode=0x0070` lines exist in this run, and the
  equip has none. **[L]** The instrument discriminates — twenty-two positives in the same
  file.
* The fault is inside the client's deallocator, on its **large-block** path.
  `0x14019b58e` is the return address of `call rbx` at `0x14019b58c` in `FUN_14019b4e0`.
  That branch is reached only when the block header at `ptr-8` says the block is **larger
  than 0x80 bytes**; anything smaller is pushed onto an internal free list at
  `DAT_143ad6908` and never leaves the image. **[L]**

  ```asm
  14019b573  TEST ECX,ECX / JNS 0x14019b52b     ; size class >= 0 -> internal free list
  14019b577  MOV  RBX,qword ptr [RIP+0x3939fb2] ; DAT_143ad5530
  14019b57e  CALL qword ptr [RIP+0x3939fb4]     ; DAT_143ad5538 -> a heap handle
  14019b584  MOV  RCX,RAX / MOV R8,RDI / XOR EDX,EDX
  14019b58c  CALL RBX                           ; (handle, 0, ptr-8)
  14019b58e  JMP  0x14019b5eb                   ; <- the address in the fault line
  ```

  Both `DAT_143ad5530` and `DAT_143ad5538` are **uninitialised `.data`**
  (`python tools/dump_va.py 0x143ad5530 32` says so), i.e. resolved at run time by the
  packer, so they cannot be named statically. Calling them `HeapFree`/`GetProcessHeap` is
  **[D]** from the argument shape `(handle, 0, block)` plus the fact that the exception
  raised at this return address was `STATUS_HEAP_CORRUPTION` **in `ntdll.dll`**. Nothing in
  the argument below depends on the names, only on "this is the only path in this function
  that leaves the image".

> **CORRECTED 2026-08-22 by the first crash dump this project ever captured**
> (`research/heap-corruption-dump.md`). Two things above need amending, and the first is a
> piece of reasoning rather than a fact.
>
> **1. "This branch is only reached for blocks larger than `0x80`, therefore the freed object
> was one of the large ones" is unsound.** The branch is chosen from the **header**, not from
> the block. In the dump, the header of the block being freed read
> `0x0000000100000020` where it should have read `0x20` - a stray `1` in the high dword - and
> that pushed a **32-byte** allocation off the internal free-list path and into `RtlFreeHeap`.
> So the branch says *the header claimed large*, which is a different statement, and the size
> argument that follows from it cannot be used to identify the object. **[L]**, from a measured
> header.
>
> **2. `DAT_143ad5530` and `DAT_143ad5538` are now named, not inferred.** Read out of the live
> image in the dump they are **`kernel32!HeapFree`** and **`kernel32!GetProcessHeap`** - the
> order the listing above uses. The `[D]` reasoning from the argument shape `(handle, 0, block)`
> was right; it is now **[L]**, and the note that they "cannot be named statically" is true only
> of the on-disk image.
>
> **3. And the death was not heap corruption at all.** The Windows heap chain was intact - 156
> consecutive blocks stepped, every header checksum valid - and the address the allocator
> complained about lands inside a **live, BUSY** 270 352-byte block. `RtlFreeHeap` refused a bad
> free rather than discovering damage. The failure type is `8`, "block not busy". Page heap
> would not have helped here and that is worth saying plainly: it guards Windows heap blocks,
> and the damaged header is a slot inside a client-allocator arena it cannot see into.

* This is a **different** free from the 2026-08-20 22:46 heap death, which faulted in
  `FUN_14019bb6a`. **[L]**
* **The block was not an equipped-item object.** `FUN_1402cc180` allocates a type-1 item as
  `FUN_14019b780(&DAT_143ad68a0, 0x467)` — 1127 bytes — and its scalar-deleting destructor
  `FUN_1402fb5c0` (vtable[0] of `0x14327e1d8`) releases it with
  `thunk_FUN_140205820(this, 0x467)`, which is `0x140205820 -> call 0x14019bb50`. A
  different wrapper from the one that faulted. **[L]** for both call chains.
* Inside `FUN_142d51930` itself, the only callers of `FUN_14019b4e0` are the four rehash
  frees (`142d51d4b`, `142d523f9`, `142d5290a`, `142d52b3a`) and the two end-of-handler
  bucket frees (`142d5450c`, `142d54560`), all of them `n*8`-byte hash bucket arrays with
  `n` a prime from `DAT_1434956a0` (`2, 3, 5, 7, 11, 13, 17, ...`). For such a block to take
  the ntdll path it must hold **17 buckets or more**. `FUN_14019b4e0` returns on its first
  instruction when passed null, so an empty hash costs nothing. **[L]**

  **Retracted before it was published:** the first draft of this section said the faulting
  free "had already run twenty-two times in this session without incident". That is wrong.
  The bucket arrays are allocated only inside the tracked-item latch block, and the small
  primes mean an early array is under 0x80 anyway; nothing shows those frees ever reached
  ntdll during the twenty-one mode-0 packets. The claim was arithmetic on a table I had not
  read, and the table starts at 2.
* `FUN_14019b4e0` is a general-purpose deallocator, so the faulting call may equally have
  come from anywhere in the handler's dynamic extent — `FUN_1428336c0` (the avatar rebuild),
  `FUN_142ce51b0`, `FUN_142ce5e60`, `FUN_142d9bfd0`. The post-mortem stack has **one** `.text`
  frame and cannot say which. **[L]**

### 6.2 What is not established, and the alternative I could not eliminate

`research/heap-corruption.md` records five sessions killed by `0xC0000374` at 193 s, 213 s,
240 s, 390 s and 482 s, in runs with **no equipping at all**, and its capture diff eliminated
session length, control-ack volume, mob count, field entries and drops as discriminators.
This run died at **240.9 s**, inside that distribution.

So there are two readings and the evidence does not separate them:

| reading | supported by | against |
|---|---|---|
| the equip caused it | no dispatch line; 9 ms after the reply; the whole `avatarChanged` half of the handler — `FUN_142d9bfd0`, `FUN_142da62e0(…, 1)`, the `FUN_142ce51b0` loop, `FUN_142ce5e60`, `FUN_1427e90b0`, `FUN_1428336c0` — ran for the **first time in this session**, because the other 21 `0x0070`s were mode 0 into empty slots and the one mode 3 was a positive Etc slot | nothing in the packet, the mode-2 decode, the worn array, the refcounting or the item blob is wrong by the listing; and that same half ran 13 times in the 2026-08-20 01:26 run with no fault |
| the equip merely walked the heap first | five prior deaths with no equip at all; 240.9 s is an ordinary lifetime here; `0xC0000374` is raised at the walk, never at the damage | the previous packet was 240 ms earlier and the previous *inventory* packet 37 s earlier, so the coincidence is tight |

I checked the item blob too, because "the ADD dispatched cleanly" does not rule out a
malformed *object*. It is clean: the 131-byte body parses to exactly 131 bytes against
`FUN_140303530`/`FUN_140304100`, with `itemID = 1002005`, `bHasSN = 0`, a year-2079 expiry,
stat mask `0x00001400` (bit 10 `incPDD` = 6, bit 12 `incACC` = 1) and option mask
`0x00040001` (bit 0 `remaining_enhancements` = 7, bit 18 `scissor_uses` = 0xFF). Every one of
those lands at a fixed offset in the item object — `item+0x62+8k` and `item+0xfa+...` — with
no array, no count and no index. **[D]**, from `research/equip-stats.md` sections 2 and 3
plus the byte-level parse of `world.log` line 2741.

**One enumerated fact worth recording either way.** Across the whole of `previous-runs/`,
every run that performed a mode-2 move (`world-20260819-220806`, `-222734`,
`world-20260820-012657`, `-013148`, `-234808`) performed **zero** equip ADDs, and every run
that performed an equip ADD (`world-20260820-075127`, `-181822`, `-222440`, `-231300`,
`-233248`, `world-20260821-001440`) performed **zero** moves. **2026-08-21 is the first time
in the archive that a mode-2 move was applied to an item created by a mode-0 ADD blob rather
than by the `SetField` character record.** **[L]**

That was the strongest lead I had, and I could not turn it into a mechanism: both blobs are
built by the same `net::opcode::equipped_item` with the same `EquipStats::fresh`
(`Session::template_stats`, `crates/world/src/session/mod.rs:653`), so the two item objects
should be identical in content. I am recording it because it is a real, enumerated
first-occurrence and because the next person should not have to re-derive it — **not**
because it explains anything.

### 6.3 The one-variant run that settles it

`heap-corruption.md`'s whole problem was that the bug had **no reproducer**. If equipping is
one, that is worth more than any further reading.

**Reproduce the same shape at ~20 s instead of ~240 s.** The database now has character 206
wearing the hat in worn slot 1, so the exact repro is two drags and nothing else:

1. Enter the world. **Do nothing** — no attacking, no picking anything up, no portals.
2. Open the inventory and **drag the hat off worn slot 1** into an empty Equip bag slot.
   That is an unequip, a shape that has worked 7 times before.
3. **Drag it straight back onto the head.** That is byte-for-byte today's fatal packet —
   `0100010000000002010400ffff00` with `oldPos` set to whichever bag slot it landed in.

Do not do anything else in between. One variant.

| what happens | what it means |
|---|---|
| dies within a second or two of step 3, `0xC0000374` | the equip is causal, and the project has a **deterministic reproducer** for the bug that has killed five sessions |
| step 3 works, hat goes on, client stays alive | the equip is not sufficient by itself; 2026-08-21 was the background bug and the packet is exonerated. Then keep playing and see whether it still dies at ~240 s |
| step 2 kills it instead | the unequip is equally implicated and mode 2 as a whole is the trigger, not the equip direction |

The owner runs, from an **elevated** window:

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1"
```

**Watch for and report:** whether step 2 completed (the hat appears in the bag and the head
goes bare), whether step 3 put the hat back on, and whether the window died — and roughly how
many seconds into the run. Which of the two drags it dies on is the whole result.

If it dies at ~20 s, the second run is **full page heap plus a dump** (the dump comes from the hook now, not from WER — see the correction above), both
already written out in `research/heap-corruption.md` — with a 20-second reproducer those stop
being expensive, and `!heap -p -a <addr>` then prints the allocation and free stacks that a
post-mortem `0xC0000374` can never give.

### 6.4 If a watch run is preferred instead

Four free slots (two are permanently spoken for). This brackets the handler so an incomplete
set of lines names the region:

```
-Probe 'watch@1415db360:ret,141b2a280:rdx=0,142d52c13:hits=4,142d52c65:hits=4,142d5219f:hits=4,142d54509:hits=4'
```

| line present | line absent | the crash is |
|---|---|---|
| `142d52c13` | `142d52c65` | inside `SetItem(oldPos, destItem)` |
| `142d52c65` | `142d5219f` | in the post-swap UI calls: `FUN_142ce53e0`, `FUN_142d9bfd0`, `FUN_142da62e0` |
| `142d5219f` | `142d54509` | in the tail: the `FUN_142ce51b0` loop, `FUN_142ce5e60`, or `FUN_1428336c0` |
| `142d54509` | — | at that free itself: the two end-of-handler bucket frees. A watch prints *before* its call, so a line at `142d54509` followed immediately by the exit names that exact `FUN_14019b4e0` call |

I do not have a prediction here worth writing down; that is the point of running it.
**Do not** read an absent line as proof on its own without the positive control:
`142d52c13` firing at all is that control, and if none of the four fires the watch was not
armed (`research/` records the ~4.5 s arming race) rather than the handler being skipped.

---

## 7. Corrections this pass makes to existing documents

* `research/msexe-setfield.md` says the handler has **18** packet-read call sites.
  `tools/reads.py` finds **17** (14 direct + 3 via `FUN_140303530`). The mode table and the
  trailing-byte rule in that document are otherwise confirmed instruction by instruction.
* `research/msexe-setfield.md` names mode 2 "Move". It is a **swap**; "move" is the special
  case where the destination is empty. Nothing built on it is wrong, but the name has already
  cost one refusal in `crates/world/src/session/inventory.rs`.
* The task brief's *"SQLite equipment for character 206 held slots 5, 6, 7, 11 before this and
  slot 1 after"* reads as a replacement. It is not: the table now holds **1, 5, 6, 7, 11**
  (`1002005, 1040003, 1060002, 1072002, 1312000`) and Equip bag slot 4 is gone. The store
  half did the right thing. **[L]**
