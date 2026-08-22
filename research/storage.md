# Storage is `0x0572` mode **24**, and the client answers on `0x00F6`

2026-08-22. **No client run was spent on this.** Everything below is static, off
`client-patched\MapleStory.exe`, the client's own WZ archives, and the repo's tools.
Ghidra was not used - another agent held the project lock.

Marks: **[L]** read off a listing, the raw PE bytes, a WZ node or a decrypted string table.
**[D]** derived from something [L] by an argument written down here. **[I]** inferred, or
taken from the v214 reference source, which scored 1 of 8 on a held-out control.

Listings added beside this file:

| | |
|---|---|
| `research/msexe-storage-handler.txt` | `FUN_142159a90` - the whole `0x0572` handler, 749 lines, rip-relative operands resolved |
| `research/msexe-storage-decode.txt` | `FUN_142150c10` - the trunk-block decoder shared by five modes |
| `research/msexe-storage-requests.txt` | the button handler and all five outbound builders |

---

## 0. The short answer

**Nothing in `crates/` knows either opcode exists.** `grep -rn "0x0572\|0x00F6" crates/`
returns one hit, and it is a byte inside a codec test vector. [L]

```
server -> 0x0572   u8 mode = 24, u32 npcTemplateId, then the trunk block
```

opens the window. It is drawn from `UI/Storage.img/Trunk`, and it is the only inbound
opcode that can create it.

The client answers on **`0x00F6`**, whose **first byte is a mode**:

| what the user does | client sends |
|---|---|
| `button:get` (take out) | `0x00F6` `u8 4`, `u8 invType`, `u8 index`, `u16 count` - **5 bytes** |
| `button:put` (store) | `0x00F6` `u8 5`, `u16 bagSlot`, `u32 itemId`, `u16 count` - **9 bytes** |
| `button:sort` | `0x00F6` `u8 6` - **1 byte** |
| `button:outCoin` (mesos out) | `0x00F6` `u8 7`, **`i64 +amount`** - 9 bytes |
| `button:InCoin` (mesos in) | `0x00F6` `u8 7`, **`i64 -amount`** - 9 bytes |
| `button:exit`, or any close | `0x00F6` `u8 8` - **1 byte** |

All [L], read off the builders in `research/msexe-storage-requests.txt`.

Two things that will bite if they are assumed rather than read, and both are §5:

* **The item the client asks to take out is identified by a positional index, not by a slot
  number.** `u8 index` is the item's **0-based position within its inventory type's list, in
  the order the server sent it**. The server's own slot numbers never reach the client.
* **`button:put` sends the player's real 1-based bag slot.** The two grids use the same
  record layout and mean different things in the same field.

And one that will wedge the UI: **every request latches `dlg+0x334`, and only an inbound
`0x0572` clears it.** §7. This is `CLAUDE.md`'s "always answer" rule with a new field name.

Mr. Kim is **NPC template 105**, on map **10000000**, and their `Npc/0000105.img/info` carries
**`trunkPut: 100`** and no `trunkGet`. [L] There are ten such NPCs; §8 lists them.

---

## 1. The instruments, and the control each was made to pass first

| instrument | positive control | result |
|---|---|---|
| `tools/callers.py` | `0x1402fa9a0` must give **96 sites in 15 functions**, 43 of them in `0x140304b20` | reproduced exactly |
| `tools/xref.py` | `--string "UI/Revive.img"` must give one hit, `FUN_1411a3440` | reproduced (`revive.md` §2) |
| `tools/reads.py` | `0x140304100 2` must show the helper reads through `FUN_1403035a0`/`FUN_140303b40` then `raw`/`u8`/`u8` then the `u16` run | reproduced exactly |
| `tools/encodes.py` | `0x141cb6880 1` must show the CTOR at `141cb7eb1` | reproduced |
| `tools/fieldrefs.py` | `0x2f4 --lo 0x141c40000 --hi 0x141d60000 --write` must give 3 hits | reproduced exactly |
| my rip-relative annotator | run on `0x1411a3440` it must print `L'UI/Revive.img'`, `L'town'`, `L'town_center'`, `L'spot'` | all four, at `revive.md` §6's addresses |
| my `COutPacket(imm)` scanner | `0x1E7` must find `0x1411a3db6` **and** a second builder; `0xD1` must find `0x1418284de` | both, matching `revive.md` §6.1/§6.2 |
| my WZ trunk-fee scan | must match `0000105.img` | it did **not**, the first time - §1.1 |
| the two-level jump table decoder | the byte table must name exactly as many slots as fit between it and the target table, and the most common slot must be the address both range checks `ja` to | 27 slots for `0x6C` bytes; slot 26 = `0x141821e24` = both `ja` targets |
| the storage gate keys | their presence bytes must reproduce `crates/net/src/bag.rs`'s `BAG_PRESENCE_BYTE` | **exactly**, from a different key table - §4.2 |

### 1.1 The scan that came back clean and empty

The sweep for other storage NPCs across all 266 templates returned **nothing**, and Mr. Kim
was already known to be a positive. The pattern was `"trunk[GP][ea]t"`, which matches
`trunkGat`/`trunkGet`/`trunkPat`/`trunkPet` and **not `trunkPut`**. Corrected to
`"trunk(Get|Put)"` it returns ten NPCs. The empty result was a property of the search, exactly
as `CLAUDE.md` says, and it was only caught because a positive control existed to run it
against first.

### 1.2 The blind spots, named

* **`tools/reads.py` cannot follow the item decode.** `FUN_140303530` reads `u8 type`, builds
  the object, then calls `obj->vt[0x358](obj, packet)` - an indirect call. reads.py reports
  **one** read there at every depth from 4 to 8. The body behind it is not unknown; it is
  `research/bag-lists.md` §5, already implemented as `net::opcode::equipped_item` (125 bytes)
  and `net::bag::bundle_item` (41, or 49 for stars/bullets). **Do not read the "1 read" as a
  1-byte field.**
* **`tools/xref.py` finds `UI/Storage.img` as a string and reports 0 code references**, and
  `tools/dataref.py` agrees. That is its documented `lea`-only blind spot: the class holds the
  path in a global `ZXString` at `0x143ad6ef0`. The chain was opened from `gridGet_lt` instead,
  which *is* `lea`'d. **A zero from `xref.py` on a UI path here means nothing.**
* **`trunkGet` / `trunkPut` have 0 references from both `xref.py` and `dataref.py`.** So the
  mapping of NPC-template offsets `+0x68`/`+0x6c` onto those two names is **[D]**, argued from
  which dialog string each fee is formatted into (§8), not read off a parser.
* **`tools/fieldrefs.py` cannot tell two classes apart.** Its `+0x2c8`/`+0x2d0` output over
  `0x14214e000..0x142161000` mixes the Trunk dialog with at least three neighbouring classes.
  Only the rows whose containing function is one of the Trunk class's own were used.
* **My `COutPacket` scanner matches only `mov edx, imm32`.** A builder that reaches the ctor
  with the opcode in a register, or one inside `.themida`, is invisible to it. It is used only
  to *enumerate* `0x00F6` builders, and every one it found was then read by hand.

---

## 2. From a UI resource to a packet handler

### 2.1 The window exists, and its buttons name themselves

`target/release/wz-dump.exe cat "client-patched/Data/UI/UI_000.wz" Storage.img` - the image is
`Storage.img`, and its single top-level node is **`Trunk`**. [L]

```
Trunk/ backgrnd  disabled  tradeBlock
       button:get   id 2000      button:put   id 2001
       button:sort  id 2002      button:exit  id 2003
       button:outCoin id 2004    button:InCoin id 2005
       tab:get ID 3000           tab:put ID 3001
       vector:npc_cb (42,77)     vector:avatar_cb (258,77)
       vector:gridGet_lt (9,127) vector:gridPut_lt (217,127)
       vector:scrollBarGet_lt (191,128)  vector:scrollBarPut_lt (398,127)
       vector:trunkMeso (174,343)        vector:myMeso (382,343)
       slotGridWidth 36  slotGridHeight 34  slotRowCount 6  slotColumnCount 5
       scrollBarLength 203  scrollBarWheelRange 278
```

**Two 5x6 grids side by side** - the trunk on the left, the player's inventory on the right -
each with its own scrollbar, plus two meso readouts. 30 cells visible per grid; a larger
storage scrolls.

### 2.2 The chain, with the result of each step

```
python tools/xref.py --string "gridGet_lt"
  -> 1 code reference: FUN_14214f380  (lea at 0x14214f429 -> 0x143432930)          [L]

python tools/callers.py 0x14214f380
  -> 0 calls, 0 tail jmps, 1 qword pointer at 0x143432710
     "pointer run 0x1434326f0..0x143432928 (72 slots), this is +0x20 [slot 4]"     [L]
```

Slot 4 of a 72-slot vtable - the **same slot** `UI/Revive.img` was found in
(`revive.md` §2). `FUN_14214f380` goes on to read `gridPut_lt`, `trunkMeso`, `myMeso`,
`slotGridWidth`, `slotGridHeight`, `slotRowCount`, `slotColumnCount`, `scrollBarGet_lt`,
`scrollBarPut_lt`, `/tradeBlock`, `/disabled` - every node in §2.1, in order. [L]

```
the vtable base 0x1434326f0 is lea'd at 0x14214eda6 and 0x14214f0ad
python tools/pdata_lookup.py 0x14214eda6  -> FUN_14214ed80 (786 bytes)             [L]
```

`FUN_14214ed80` is the **constructor**: it installs three vtables (`0x1434326f0` at `+0`,
`0x143432850` at `+8`, `0x143432928` at `+0x18` - multiple inheritance) and zeroes the object
out to `+0x1508`. [L]

```
python tools/callers.py 0x14214ed80   -> 1 call site, 0 tail jmps, 0 qword pointers
                                         0x142159ce8, inside FUN_142159a90         [L]
python tools/callers.py 0x142159a90   -> 1 call site: 0x141821aa1 in FUN_141820080 [L]
```

`FUN_141820080` is `CField::OnPacket` - **[D]**, the join is from `revive.md` §3.1, which
established it by a different route and which I did not re-derive.

### 2.3 The object can be created exactly one way

```asm
142159cba  mov  rdx, [rip+0x194e85f]     ; g_pTrunkDlg = 0x143AA8520
142159cc1  test rdx, rdx
142159cc4  jne  0x14215a590              ; already open -> return, silently
142159cca  mov  edx, 0x1510              ; sizeof(CUITrunkDlg) = 5392
142159ccf  lea  rcx, [rip+0x197cbca]     ; the pool at 0x143AD68A0
142159cd6  call 0x14019b780              ; operator new  - the same allocator revive.md saw
142159ce5  mov  rcx, rax
142159ce8  call 0x14214ed80              ; CUITrunkDlg::CUITrunkDlg
```
[L]

The constructor has **one** call site, **no** tail jumps and **no** pointer anywhere in the
image, so it is not in a vtable and no table names it. Two enumerations with different blind
spots (a call-graph scan and a `.pdata`-anchored pointer scan) agree.

---

## 3. Why the opcode is `0x0572`

`CField::OnPacket` dispatches in three ranges. The head: [L]

```asm
1418200be  cmp  edx, 0x4b3
1418200c4  jg   0x141821a52              ; > 0x4B3 -> the second table
1418200ca  je   0x141821a41
1418200d0  lea  eax, [rdx-0x1a4]
1418200d6  cmp  eax, 0x7f
1418200d9  ja   0x141821e24              ; not 0x1A4..0x223 -> the next stage
```

The storage call at `0x141821aa1` is past the end of the first table's targets, and lands in
the second: [L]

```asm
141821a52  lea   eax, [rdx-0x4d2]
141821a58  cmp   eax, 0xd9               ; 0x4D2 .. 0x5AB, 218 opcodes
141821a5d  ja    0x141821e24
141821a65  lea   rdx, [rip-0x1821a6c]    ; rdx = image base 0x140000000
141821a6c  movzx eax, byte ptr [rdx+rax+0x18223c4]   ; byte index table, 218 entries
141821a74  mov   ecx, dword ptr [rdx+rax*4+0x1822358] ; target table
141821a7b  add   rcx, rdx
141821a7e  jmp   rcx
```

Decoding **both levels in full** - enumerate before you filter - the byte table names 27
distinct slots, and `(0x18223c4 - 0x1822358) / 4` is exactly 27. Slot 26 is `0x141821e24`,
which is the address *both* range checks already `ja` to, so the decode reproduces the
default from an independent direction. Of the 218 opcodes, **26 are not the default**, and
exactly one reaches the storage handler: [L]

```
opcode 0x0572  idx 160  slot 6  -> 0x141821a9e:   mov rcx, rbx
                                   0x141821aa1:   call 0x142159a90
```

Its neighbours `0x0573`..`0x0579`, `0x057B`, `0x057C` are the same one-line shape into seven
other subsystems, and `0x055A`/`0x05AB` bracket the group. Nothing else reaches
`FUN_142159a90`.

### 3.1 The semantic control, which is the one that matters

The opcode came off a jump table, so it needs corroboration that does not come from a jump
table. The handler's error arms format string-table ids; decrypting them with
`tools/dump_stringids.py`: [L]

| id | text |
|---|---|
| `0x04CA` | *"Storage is full, so you can't store more items."* |
| `0x04D2` | *"Inventory is full, so you can't take out more items."* |
| `0x04D1` | *"Not enough mesos (%d) to store the item."* |
| `0x04CB` | *"You have exceeded the Meso limit."* |
| `0x04CC` | *"You cannot store any more Mesos."* |
| `0x14ED` | *"You cannot use the Storage Room while incapacitated."* |
| `0x01C0` | *"You cannot use the Storage Room while on the waiting list."* |
| `0x04BF` | *"Are you sure you want to recover the item(s) from storage?"* |

Ten of eleven decrypted ids in this one handler name storage explicitly. That is not a
coincidence a wrong table index could produce.

---

## 4. The body of `0x0572` mode 24, field by field

`python tools/reads.py 0x142159a90 4` (identical at depth 6 and 8):

```
0x142159acb  READ u8   (direct)                     <- the mode, first and unconditional
0x142159bf6  READ u8   (direct)  gated               (mode 23 only)
0x142159cf3  READ u32  (direct)  gated               (mode 24 only)
0x142159d07  call 0x142150c10 -> u8 raw u64  gated   (the trunk block)
0x142159d72  call 0x14215ab80 -> u32 u8 u64  gated   (mode 25 only)
0x142159f57  call 0x142150c10                gated   (mode 32's tail)
0x142159f9a  call 0x142150c10                gated   (modes 13/15/19)
0x142159fdf  READ u32  (direct)  gated               (mode 32)
0x14215a110  call 0x142150c10                gated
0x14215a279  READ u32  (direct)  gated               (mode 28)
0x14215a2fa  READ raw  (direct)  gated               (mode 29, 8 bytes)
```

Every branch is on the mode byte, so the layout is per-mode, not a single fixed body.

### 4.1 The mode-24 body

```
off  0   u8     mode = 24
off  1   u32    npcTemplateId          -> dlg+0x2c8       0x142159cf3   [L]
---- FUN_142150c10, the trunk block ------------------------------------
off  5   u8     slots                  -> dlg+0x320       0x142150c55   [L]
off  6   raw[100]  presence mask                          0x142150cb4   [L]
off 106  u64    mesos  ONLY if presence[1]                0x142150ce9   [L]
         then for invType = 1..6, ONLY if that type's presence byte is set:
             u8   count                                   0x142150ec4   [L]
             count x  item blob   (leading u8 type; 125 equip / 41 bundle)
```

**Minimum legal body: 106 bytes** - mode, template id, slot count and a hundred zero bytes.
That is an empty storage box, and it is a complete, legal packet.

`lea r8d,[rax+0x64]` with `eax = 0` at `0x142150caa` is where the **100** comes from; it is
read as `raw`, not as a length-prefixed anything. [L]

### 4.2 Which presence byte opens which block

The mask works exactly as `research/charrecord-presence-map.md` describes: each gate carries
its own 100-byte key, the block runs iff `any(presence & key)`, and every key is all-zero
except one byte written by a CRT initialiser. Storage has **its own key table**, based at
`0x143ad6f10` with stride `0x70`, not the character record's at `0x143abeb10`. [L]

Reading each initialiser's `mov byte ptr [key + b], 1`: [L]

| block | key | initialiser | writes | **presence byte** |
|---|---|---|---|---|
| **mesos** (`u64`) | `0x143ad6f10` | `0x1400db7e0` | `0x143ad6f11` | **1** |
| type 1 Equip | `0x143ad7220` | `0x1400db760` | `0x143ad7222` | **2** |
| type 2 Use | `0x143ad71b0` | `0x1400db740` | `0x143ad71b3` | **3** |
| type 3 Set Up | `0x143ad7140` | `0x1400db7c0` | `0x143ad7144` | **4** |
| type 4 Etc | `0x143ad70d0` | `0x1400db7a0` | `0x143ad70d5` | **5** |
| type 5 Cash | `0x143ad7060` | `0x1400db720` | `0x143ad7066` | **6** |
| type 6 Deco | `0x143ad6ff0` | `0x1400db780` | `0x143ad701c` | **44** |

The last column is **`crates/net/src/bag.rs`'s `BAG_PRESENCE_BYTE = [2, 3, 4, 5, 6, 44]`,
byte for byte** - derived here from a completely different key table, built by different
initialisers, reached from a different packet. Two independent derivations landing on the
same seven-element permutation is the strongest control in this document.

`0x143ad6f80` is the `ja` default for `invType > 6`. It has **no initialiser** - one
reference in the whole image, the `lea` at `0x142150d96` - so its mask stays all-zero and its
gate can never fire, which is the same property `charrecord-presence-map.md` records for its
own entry 0. The loop only runs 1..6, so it is unreachable anyway. [L]

### 4.3 The per-type list, and where the slot number is *not*

```asm
142150ec4  READ u8                      ; count for this type, r13d
142150edd  xor   r12d, r12d             ; index = 0, reset PER TYPE
>142150ef0
142150f02  call  0x140303530            ; one item blob
...
1421511d3  call  0x14215afd0            ; append a 0x30-byte record
1421511de  mov   [rec+0x00], itemId
1421511e3  mov   [rec+0x04], quantity
1421511e6  mov   [rec+0x08], r12d       ; <- the index, 0-based, within THIS type
1421512fd  inc   r12d
```
[L]

**There is no slot field on the wire.** The client's identity for a stored item is the
position it arrived in. `[rec+0x08]` is what `button:get` sends back (§5.1).

A type whose gate is **clear** is not cleared - the decoder detaches the old list at
`0x142150d1c` and, for each clear type, re-appends the old records verbatim including
`[rec+0x08]` (`mov ecx,[rdi-0x18] / mov [rax+8],ecx` at `0x142150e60`), filtered by
`itemId/1000000 == invType`. [L] So a partial update is possible - but see §9.2 for why it
should not be used.

### 4.4 The item blob is code that already exists

`FUN_140303530` is the same factory `research/bag-lists.md` and
`crates/net/src/inventory.rs` already document: [L]

```asm
140303543  READ u8                      ; item type: 1 equip, 2 bundle, 3 pet
140303550  call 0x1402cc180             ; construct by type
140303578  call qword ptr [rax+0x358]   ; the type's own Decode  <- reads.py stops here
```

So **storage reuses the bag's item encoding exactly.** `net::opcode::equipped_item(id,
&stats)` and `net::bag::bundle_item(id, qty, attr, owner)` both already emit a complete blob
*including the leading type byte*, and both are already on the wire in `0x0107` mode 0. There
is no second encoder to write and no reason to write one.

---

## 5. What the client sends

`FUN_142150a70` is **vtable slot 9** - the same slot the revive dialog's button handler
occupied. It tests the button name and dispatches: [L]

```asm
142150aa3  lea rdx,[rip+...] ; L'get'      -> FUN_142157ea0
142150ac6  lea rdx,[rip+...] ; L'put'      -> FUN_142158460
142150ae9  lea rdx,[rip+...] ; L'sort'     -> inline
142150b48  lea rdx,[rip+...] ; L'exit'     -> this->vt[0x138](this, 2)   = slot 39
142150b71  lea rdx,[rip+...] ; L'outCoin'  -> FUN_142158c50
142150b91  lea rdx,[rip+...] ; L'InCoin'   -> FUN_142158e40
```

matching `UI/Storage.img/Trunk`'s six button nodes. My `COutPacket` scanner finds **18**
`0x00F6` builders in the whole image, and there are no modes outside `{3, 4, 5, 6, 7, 8}`.
Nine sit in this class (`0x14214f2c4`, `0x142150b09`, `0x142158361`, `0x142158ae3`,
`0x142158bf7`, `0x142158dcb`, `0x142158fc5`, `0x142159bb5`, `0x142159c46`) and nine in the
second, undrawable class of §10. [L]

`0x142158bf7` is `FUN_142158bd0`, a standalone "send sort" with the identical latch check and
**zero callers, zero tail jmps and zero pointers** - an out-of-line duplicate of the inline
sort arm, the same shape `revive.md` §2 found in `FUN_142903cd0`. [L] It is listed here so
that finding it later does not look like a second sort protocol.

### 5.1 `button:get` -> mode 4

```asm
142158361  mov edx, 0xf6  / call 0x1406ed520      ; COutPacket(0x00F6)
142158370  mov dl, 4      / call w_u8             ; u8  4
14215837b  mov edi,[rbp-0x80] / movzx edx,dil     ; u8  invType   = FUN_1403e8af0(itemId)
14215838b  movzx edx, byte ptr [r12+8]            ; u8  index     <- [rec+0x08]
14215839a  movzx edx, word ptr [rsp+0x60]         ; u16 count
1421583ac  call 0x1415d01c0                       ; SendPacket
1421583b1  mov dword [r13+0x334], 1               ; latch
1421583b8  mov dword [r13+0x330], edi             ; remember the type
```
[L]

`r12 = [dlg+0x2e0] + dlg[0x318]*0x30` - the selected record of the **trunk** grid, so `+8` is
§4.3's positional index. `[rsp+0x60]` is `1`, or the result of the quantity prompt (string
`0x04C5`, *"How many do you want to take out?"*) when the stack is bigger than one. [L]

**`u8 index` takes only the low byte** (`movzx edx, byte ptr`), so the wire cannot express an
index above 255 even though the client stores a `dword`.

### 5.2 `button:put` -> mode 5

```asm
142158ae3  mov edx, 0xf6  / call 0x1406ed520
142158af2  mov dl, 5      / call w_u8             ; u8  5
142158afd  movzx edx, word ptr [rsp+0x78]         ; u16 bagSlot
142158b0b  mov edx, dword ptr [rsp+0x7c]          ; u32 itemId
142158b18  movzx edx, word ptr [rbp-0x80]         ; u16 count
142158b25  call 0x1415d01c0
142158b2e  mov dword [r13+0x334], 1
```
[L]

Here `rbx = [dlg+0x2e8] + dlg[0x31c]*0x30` - the **inventory** grid - and `[rsp+0x78]` is
`[rbx+8]`, `[rsp+0x7c]` is `[rbx+0]` (`0x14215850d`/`0x142158516`). [L]

**And `+8` means something different in this list.** `FUN_142151640` fills the inventory grid
by walking the player's bag: [L]

```asm
1421516a1  mov  edi, 1                            ; slot = 1
1421516ef  call 0x14030ccb0                       ; slots in this bag
1421516f4  cmp  eax, edi / jl exit
>142151700 mov  r9d, edi / mov r8d, esi
14215170d  call 0x1402e3cd0                       ; item at (invType, slot)
142151788  mov  dword [rec+8], edi                ; <- the REAL 1-based bag slot
```

So the same field is a positional index on the trunk side and a real slot on the inventory
side. Reading one as the other is the whole shape of §4.3's warning.

Note there is **no inventory type** in the put request - only the item id. The server derives
the type the same way the client does: `itemId / 1_000_000` (`FUN_1403e8af0` is a magic-number
divide by 1000000). Its one special case: when the quotient is `1` and the id is in
`1000000..1999999`, `5000000..5999999` or `6000000..6999999`, it consults the item data and
returns **6** if a flag at `+0x18` is set. [L] For the ids this server hands out today the
quotient is the answer; the exception is named rather than smoothed over.

### 5.3 `outCoin` / `InCoin` -> mode 7, one signed `i64`

Both build the **same mode**. The only difference is the sign: [L]

```asm
; outCoin - take mesos OUT of storage          ; InCoin - put mesos IN
142158dbd  test rbx, rbx / jle skip            142158fad  lea rax,[rbx-1]
142158dc2  cmp  rbx, [rsi+0x328] / jg skip     142158fb1  movabs rcx, 0x746a5287fe
                                               142158fbb  cmp  rax, rcx / ja skip
                                               142158fc0  cmp  rbx, r14 / jg skip
142158dcb  mov edx, 0xf6                       142158fc5  mov edx, 0xf6
142158ddb  mov dl, 7   / w_u8                  142158fd5  mov dl, 7   / w_u8
142158de7  mov rdx, rbx / w_u64                142158fe1  neg  rbx
                                               142158fe4  mov rdx, rbx / w_u64
```

* **positive = withdraw from storage**, bounded above by `dlg+0x328`, the trunk balance the
  server sent.
* **negative = deposit into storage**, bounded by `1 <= amount <= 499 999 999 999`
  (`0x746A5287FE + 1`) and by the player's own mesos.

**The meso field is 64-bit in both directions.** `dlg+0x328` is written by exactly one
instruction in the image (`0x142150cee`, the `u64` read in §4.1) and read by the outCoin
bound and by mode 24's tail. [L] This agrees with what the repo already ships:
`net::stats::StatChange::meso` is `Option<u64>` and its own test pins bit 18 to a 64-bit
value. [L] `crates/store`'s `StorageBox.mesos` is a `u32`; that is a server-side cap and not
a wire fact, but the **eight bytes must go out as eight bytes**.

### 5.4 `sort` and `exit`

```asm
; sort, inline in the button handler          ; FUN_14214f2a0, vtable slot 39 (+0x138)
142150afc  cmp dword [rbx+0x334],0 / jne      14214f2c4  mov edx, 0xf6
142150b09  mov edx, 0xf6                      14214f2d4  mov dl, 8  / w_u8
142150b19  mov dl, 6  / w_u8                  14214f2e5  SendPacket
142150b2a  SendPacket                         14214f2ef  call 0x14177fef0   ; base close
142150b2f  mov dword [rbx+0x334], 1
```
[L]

**Every close sends `0x00F6 { u8 8 }`** - the `exit` button goes through the same slot-39
override, so there is one notification and not two paths. The client destroys the dialog
itself; the server does not need to close anything.

---

## 6. The inbound mode table, in full

`0x0572`'s handler switches on the first byte: `add ebx,-9 / cmp ebx,0x1a / ja default`, table
at `0x14215a5c0`, 27 entries. **Modes below 9 and above 0x23 fall to `0x14215a512`**, which
raises string `0x04AD` *"An unknown error prevented the trade from completing."* [L]

| mode | arm | what it does |
|---:|---|---|
| **9** | `0x142159d97` | rebuild the **inventory** grid from client-side state. **No packet reads.** |
| **10** | `0x14215a14c` | msgbox `0x04D2` *"Inventory is full, so you can't take out more items."* |
| **11** | `0x14215a18e` | msgbox `0x009D` *"You don't have enough Mesos."* |
| **12** | `0x14215a179` | msgbox `0x04CD` *"Item could not be retrieved because there was an item that could only be acquired once."* |
| **13** | `0x142159fc6` | **rebuild the inventory grid, then re-decode the trunk block.** |
| 14 | - | default |
| **15** | `0x142159fd7` | **re-decode the trunk block** (scroll recalculated) |
| **16** | `0x14215a1a3` | msgbox `0x04D1` *"Not enough mesos (%d) to store the item."*, `%d` = the NPC's `trunkPut` |
| **17** | `0x14215a216` | msgbox `0x04CA` *"Storage is full, so you can't store more items."* |
| 18 | - | default |
| **19** | `0x142159f8e` | **re-decode the trunk block** (scroll **preserved**) |
| 20 | - | default |
| **21** | `0x14215a4bd` | msgbox `0x04CB` *"You have exceeded the Meso limit."* |
| **22** | `0x14215a4d5` | msgbox `0x04CC` *"You cannot store any more Mesos."* |
| **23** | `0x142159b8d` | a handshake: reads `u8`; replies `0x00F6 { u8 3 }`, or `{ u8 8 }` if a dialog is already open |
| **24** | `0x142159c9e` | **OPEN.** `u32 npcTemplateId` + the trunk block |
| **25** | `0x142159d6f` | `u32 count`, `count` x item blob, `u64` - fills a global "recoverable" list at `0x143ad6f00`/`0x143ad6ed0` |
| **26** | `0x14215a22e` | msgbox `0x0DAB` *"Items or mesos cannot be moved."* |
| **27** | `0x14215a246` | msgbox `0x0925` - account inactive |
| **28** | `0x14215a276` | `u32 minutes`, msgbox `0x0926` - different-IP restriction |
| **29** | `0x14215a2ed` | `raw[8]` - a timestamp; msgbox `0x0927`/`0x0928` - trade restricted |
| **30** | `0x14215a437` | msgbox from a global table |
| **31** | `0x14215a4ed` | msgbox `0x01C0` *"You cannot use the Storage Room while on the waiting list."* |
| **32** | `0x142159fdc` | `u32 itemId`, msgbox `0x080D` *"The item [%s] has expired..."*, **then the trunk block** |
| **33** | `0x14215a4a5` | msgbox `0x01BE` *"This function cannot be used right now."* |
| 34 | `0x14215a590` | return, no-op |
| **35** | `0x14215a25e` | msgbox `0x14ED` *"You cannot use the Storage Room while incapacitated."* |

All [L] off `research/msexe-storage-handler.txt`.

Modes **13**, **15**, **19**, **24** and **32** carry the trunk block; they differ only in
whether the inventory grid is rebuilt first and whether the scroll position survives (the
third argument to `FUN_142150c10`, compared at `0x142151346`). **The block itself is
byte-identical in all five.** [L]

---

## 7. The latch - the "always answer" hazard

`dlg+0x334` is a one-request-outstanding latch, and all four request builders honour it: [L]

| builder | checks | sets |
|---|---|---|
| `FUN_142157ea0` get | `0x142157f03  cmp dword [r13+0x334],0 / jne exit` | `0x1421583b1` |
| `FUN_142158460` put | `0x1421584c8  cmp dword [r13+0x334],esi / jne exit` | `0x142158b2e` |
| sort (inline) | `0x142150afc  cmp dword [rbx+0x334],0 / jne exit` | `0x142150b2f` |
| `FUN_142158c50` outCoin | `0x142158c80  cmp dword [rcx+0x334],ebx` | `0x142158dfe` |
| `FUN_142158e40` InCoin | `0x142158e70  cmp dword [rcx+0x334],ebx` | `0x142158ffb` |

It is cleared in exactly one place - the top of the `0x0572` handler, for **any** mode that is
not one of `{23, 24, 25, 30, 31}`: [L]

```asm
142159b19  mov ecx, ebx / sub ecx, 0x17 / je keep      ; 23
142159b20  sub ecx,1 / je keep                         ; 24
142159b25  sub ecx,1 / je keep                         ; 25
142159b2a  sub ecx,5 / je keep                         ; 30
142159b2f  cmp ecx,1 / je keep                         ; 31
142159b34  test rdi, rdi / je return                   ; no dialog -> nothing to clear
142159b5d  mov dword [rdi+0x334], esi                  ; = 0
```

**So an unanswered `0x00F6` leaves the storage window alive and completely inert** - every
button dead until it is closed. Any of modes 10..22, 26..29, 32, 33, 35 clears it, so even a
pure refusal re-enables the UI. Answering with an error is correct; answering with nothing is
not.

`0x00F6 { u8 8 }` (close) is the exception - the dialog is already gone. Log it, do not reply.

---

## 8. The slots, the fees, and the ten keepers

**Slot count: server-supplied, in the packet, one byte.** `dlg+0x320` has exactly **one**
writer in the whole class window - `0x142150c5d`, the `u8` at §4.1 offset 5. [L] Its only
other uses are a read in `FUN_142151cd0` and a bound test at `0x1421539c1`. The client
neither computes it nor takes it from the NPC.

`crates/store::storage::DEFAULT_STORAGE_SLOTS` is `4` and marked **[I], nothing in this
client corroborates it**. That is still true - nothing here measures a *default* - but the
field's width is now measured: **`u8`, so 0..255**, and `MAX_STORAGE_SLOTS = 100` is safely
inside it. The visible grid is 5x6 = 30 cells with a scrollbar, so a box larger than 30
scrolls rather than clipping.

**The fees are client-side, from the NPC template, and the server never sends them.** [L]

```asm
142152c91  mov ecx, dword ptr [r13+0x2c8]   ; the npcTemplateId from mode 24
142152c98  call 0x141e77b70                 ; NPC template lookup (67 sites, incl. the
142152c9d  mov r15, rax                     ;   0x044F handler FUN_141e75800)
...
1421536ac  mov eax,[r15+0x68] -> dlg+0x2cc  ; withdraw fee
1421536b7  mov eax,[r15+0x6c] -> dlg+0x2d0  ; deposit fee
```

`dlg+0x2cc` is used only in the **get** path, formatting `0x04C0` *"Recovering the items will
cost %d mesos"*; `dlg+0x2d0` only in the **put** path and in mode 16's *"Not enough mesos
(%d) to store the item"*. On that evidence `+0x68` is `trunkGet` and `+0x6c` is `trunkPut` -
**[D]**, because the two WZ property strings have zero references from both `xref.py` and
`dataref.py` (§1.2) and no parser was found.

Scanning all **266** NPC templates in `Npc_000.wz` for `trunkGet`/`trunkPut`: [L]

| template | name | `trunkPut` | `trunkGet` |
|---:|---|---:|---|
| **105** | **Mr. Kim** | **100** | absent |
| 220 | Mr. Lee | 100 | absent |
| 307 | Mr. Park | 100 | absent |
| 405 | Mr. Hong | 100 | absent |
| 505 | Mr. Wang | 100 | absent |
| 604 | Mr. Oh | 100 | absent |
| 704 | Cave Fairy's Storage | 100 | absent |
| 1011 | Trina | 100 | absent |
| 1110 | Mr. Thalj | 150 | absent |
| 800009 | Scrooge | 100 | absent |

**Ten storage keepers, all charging to deposit and none to withdraw.** The client only
*warns*; it never deducts. The server owns the 100 mesos.

### 8.1 Why Mr. Kim does nothing today

`crates/world/src/session/npc.rs:589`, `on_npc_click`: the `0x00F2` click resolves the object
id to a template, tries `open_shop_for(template, ...)`, and otherwise starts a one-line
conversation and calls `say_line(0)`. [L] Mr. Kim is template **105** on map **10000000**, and
`gm-handbook/npcstrings.txt` gives them `idle0`, `idle1`, `info0`, `info1` and **no `dN`
line**, so `npc_line` falls through to its placeholder. **There is no branch anywhere in
`crates/` that could send `0x0572`.** [L]

---

## 9. Account or character?

**The client cannot answer this, and this document will not pretend otherwise.**

`0x0572` carries no account id, no character id, and no owner of any kind - the mode-24 body
is a template id, a slot count, a 100-byte mask, a meso balance and item blobs (§4.1). The
requests carry a mode and positions. There is nothing on the wire, in either direction, that
distinguishes a per-account box from a per-character one. A search for such a field is a
search that can only come back empty, so its emptiness is worth nothing.

What can be said:

* **[L]** `crates/store/src/storage.rs` keys both tables on `account_id`, and its own doc
  block records the owner's instruction of 2026-08-19: *"all of the characters of a particular
  account share this inventory."* That is a user requirement, and it is the only real
  evidence in this repo.
* **[L]** The one storage refusal that names an owner names the **account**: mode 27's string
  `0x0925`, *"Your account has been inactive for a while, so items and mesos cannot be
  transferred."* An account-scoped lockout on a storage window is consistent with an
  account-scoped box, and inconsistent with nothing.
* **[I]** This game family stores the trunk per account in every server tree I am aware of.
  Per `CLAUDE.md` that is a candidate, not a fact.

So: **keep `account_id`.** It is what the owner asked for, the store already implements it, and
the client neither confirms nor contradicts it. If it ever needs settling, the measurement is
behavioural, not static: deposit on character A, log to character B, open storage. One
launch, one bit.

### 9.1 Nothing here authenticates

The channel connection carries no credentials. The `account_id` that would key a storage box
comes from the migration row and from nothing else - the same standing caveat `store`'s own
crate docs carry, restated because a storage box is the first thing in this server where
"whose is it" has consequences.

### 9.2 Send the whole box, every time

The decoder *can* be driven incrementally (§4.3), and it should not be. The wire index in a
`get` is positional (§5.1), so the server has to be able to reconstruct exactly the ordering
it last sent. Sending all seven gates on every trunk-block reply makes that reconstruction a
pure function of the database:

> for inventory type `T`, wire index `i` is the `i`-th row of
> `storage.items` filtered to `item_id / 1_000_000 == T`, **in ascending `slot` order**.

A partial update makes it a function of history instead, and history is exactly what a
reconnect loses. 106 bytes plus the items is not worth optimising.

---

## 10. Two things I found and did not chase

**A second, live UI class speaks the identical `0x00F6` protocol.** Nine of the 18 `0x00F6`
builders live at `0x14233c5e0`..`0x142345904`, with the same mode set `{3,4,5,6,7,8}`, hanging
off a vtable at `0x14344f930`. Its resource path is **`UI/UIWindow2.img/Trunk`** (strings at
`0x14344fb70`, `0x14344fbb0`), and `UIWindow2.img` **is not present in either
`client-patched/Data/UI/UI.wz` or `UI_000.wz`** - the only `UIWindow` image this client ships
is `UIWindow.img`. [L] So it is a newer skin whose art is missing, and it cannot draw. Two of
its functions (`0x1423452d0`, `0x14233c5e0`'s siblings) have zero callers, zero tail jmps and
zero pointers - dead, the way `FUN_142903cd0` was dead in `revive.md` §2.

**I did not enumerate what dispatches to it.** `0x0572` does not - `FUN_142159a90` has one
call site, and no other arm of the `0x4D2` table lands in `0x14233xxxx`. That is a bounded
negative: it says `0x0572` does not reach that class, not that nothing does.

**Mode 25's list is not the trunk.** `FUN_14215ab80` reads `u32 count`, `count` item blobs
and a trailing `u64`, into globals at `0x143ad6f00`/`0x143ad6ed0` that `FUN_142150c10` also
walks. Strings `0x04BF`/`0x04C0` (*"recover the item(s) from storage"*, *"Recovering the items
will cost %d mesos"*) suggest an item-recovery list. **Its purpose is a lead, not a layout** -
the reads are [L], the meaning is not established, and nothing in §11 depends on it.

---

## 11. WIRE IT LIKE THIS

Nothing below is wired. `crates/` contains no reference to `0x0572` or `0x00F6`. [L]

### 11.1 The constants

```rust
pub const STORAGE_RESULT: u16   = 0x0572;   // server -> client
pub const CLIENT_STORAGE: u16   = 0x00F6;   // client -> server, first byte is a mode

// inbound modes worth naming
pub const STORAGE_INVENTORY_REFRESH: u8 = 9;
pub const STORAGE_PUT_OK:            u8 = 13;  // rebuild both grids + trunk block
pub const STORAGE_TRUNK_REFRESH:     u8 = 15;  // trunk block only
pub const STORAGE_OPEN:              u8 = 24;  // u32 npcTemplate + trunk block
pub const STORAGE_INV_FULL:          u8 = 10;
pub const STORAGE_NO_MESOS:          u8 = 11;
pub const STORAGE_NOT_ENOUGH_FEE:    u8 = 16;
pub const STORAGE_FULL:              u8 = 17;
pub const STORAGE_MESO_LIMIT:        u8 = 21;

// outbound modes
pub const REQ_TAKE_OUT: u8 = 4;
pub const REQ_PUT_IN:   u8 = 5;
pub const REQ_SORT:     u8 = 6;
pub const REQ_MESOS:    u8 = 7;   // i64, + = withdraw, - = deposit
pub const REQ_CLOSE:    u8 = 8;

// the presence bytes - the SAME namespace as net::bag::BAG_PRESENCE_BYTE
pub const PRESENCE_MESOS: usize = 1;
// inventory type n uses net::bag::BAG_PRESENCE_BYTE[n - 1]  =  [2, 3, 4, 5, 6, 44]
```

### 11.2 The trunk block builder

```
fn trunk_block(box: &StorageBox) -> Vec<u8>
    u8   box.slots as u8                       // 0..255
    let mut presence = [0u8; 100];
    presence[1] = 1;                           // ALWAYS send the mesos gate
    for t in 1..=6 { presence[BAG_PRESENCE_BYTE[t-1]] = 1; }   // ALWAYS all six - 9.2
    raw  presence                              // exactly 100 bytes
    u64  box.mesos as u64                      // EIGHT bytes - 5.3
    for t in 1..=6 {
        let items: Vec<_> = box.items.iter()
            .filter(|i| inv_type_of(i.item.id) == t)
            .collect();                        // ascending slot order - 9.2
        u8 items.len() as u8                   // 0..255
        for it in items { extend(item_blob(it)) }   // equipped_item / bundle_item
    }
```

`item_blob` is **not new code**: `net::opcode::equipped_item(id, &stats)` for an equip,
`net::bag::bundle_item(id, qty, attr, &owner)` for a stack. Both already emit the leading type
byte (§4.4). Writing a second encoder here would be building the thing `bag.rs` exists to
prevent.

Empty box, all gates on: the **block** is `1 + 100 + 8 + 6 = 115` bytes, so a mode-24 body is
`1 + 4 + 115 = 120`. (§4.1's "106 minimum" is the same packet with *no* gates set - legal, but
it means an empty box that also cannot show a meso balance.)

`inventory_quantity` and `inventory_removed` exist beside `inventory_added` in
`crates/net/src/inventory.rs`; a partial withdrawal from a stack is a `quantity` update, not a
remove.

### 11.3 Open it from Mr. Kim's click

The hook point is `crates/world/src/session/npc.rs`, `on_npc_click`, **beside
`open_shop_for`** and before the conversation fallback:

```
if let Some(fee) = storage_keeper(template) {      // the ten templates in 8
    let box = store.storage(account_id)?;
    let mut body = vec![24u8];
    body.extend_from_slice(&template.to_le_bytes());   // u32, NOT the object id
    body.extend(trunk_block(&box));
    self.storage_npc = Some(template);                 // remember it for the fees
    return vec![Reply { opcode: 0x0572, body, .. }];
}
```

Three details that will otherwise cost a launch each:

* **The `u32` is the NPC *template* id, not the object id.** `0x00F2` carries the object id
  (`research/npc-click.md` §0); mode 24's field is fed straight into the template lookup at
  `0x142152c98`. Send the object id and the lookup returns null, the fee block is skipped, and
  the dialog draws with no NPC portrait.
* **Send all 100 mask bytes.** The read at `0x142150cb4` is a fixed `raw[100]`. Ninety-nine
  bytes and the client eats the first item blob as mask.
* **Do not create a storage row just because someone clicked.** `store::storage()` already
  returns the default box for an account with no row; keep it that way, so talking to Mr. Kim
  writes nothing.

### 11.4 Answer the six requests

`u8 mode` first, then:

| mode | body | do | reply |
|---|---|---|---|
| **4** take out | `u8 invType, u8 index, u16 count` | resolve `(invType, index)` by §9.2's rule -> a `slot`; `store::take_item` | `0x0070` `inventory_added`, **then** `0x0572` mode **13** + trunk block |
| **5** put in | `u16 bagSlot, u32 itemId, u16 count` | verify the bag slot really holds `itemId`; charge `trunkPut`; `store::store_item` | `0x0070` `inventory_removed` + `0x007C` mesos, **then** `0x0572` mode **13** |
| **6** sort | (none) | re-slot the box however you like | `0x0572` mode **15** + trunk block |
| **7** mesos | `i64 delta` | `delta > 0` = **withdraw** to the player, `delta < 0` = **deposit** | `0x007C` mesos, **then** `0x0572` mode **15** |
| **8** close | (none) | drop `self.storage_npc` | **none** - log it |
| anything else | | | `0x0572` mode **33** (*"This function cannot be used right now."*) |

**Every refusal must still be a `0x0572`,** and the mode should say why - mode 10 inventory
full, 11 not enough mesos, 16 cannot afford the fee, 17 storage full, 21/22 meso limit. All of
those clear the latch (§7); silence does not.

And the `CLAUDE.md` rule the quest bug taught: **hang each effect off the transition, not off
the request.** `store::store_item` already refuses a trade-blocked item with
`StoreError::ItemMayNotBeStored`. Match on the store's answer once and return the refusal
mode; do not deduct the fee, send the `0x0107` and *then* discover the insert failed.

### 11.5 The three numbers that must not drift

* **`u8` index in mode 4 is positional.** If `store::storage()` ever returns items in a
  different order between two sends, the client's index means a different item. Sort by
  `slot` at the boundary, not by chance.
* **`i64` in mode 7 is signed and sixty-four bits.** A `u32` cast silently turns a deposit
  into a four-billion-meso withdrawal.
* **`u64` mesos in the trunk block.** Same eight bytes, unsigned.

---

## 12. If it does not appear: the single-variant test

One launch, one variant. **Send `0x0572` mode 24 with template `105` and an empty box** the
moment Mr. Kim is clicked.

The client is unusually informative here, because §7 means it either draws or it says so.

| what the owner sees | what `world.log` / `client-patched\maplecw-hook.log` shows | what it means |
|---|---|---|
| the storage window, two grids, "0" in both meso readouts | no inbound `0x00F6` until they click | everything in this document is right |
| **nothing at all**, no dialogue box either | the hook log has **no dispatch line** for `0x0572` | the handler was entered and did not return - a malformed body. Count the bytes: `1 + 4 + 1 + 100` minimum |
| nothing, and the old placeholder dialogue box instead | no `0x0572` in `world.log` | the click never reached the storage branch; `on_npc_click` still falls through to `say_line` |
| the window, but **no NPC portrait** and no fee prompt on a deposit | `0x0572` is on the wire | the `u32` is the object id, not template 105 (§11.3) |
| the window, then it is **inert** - no button responds | one `0x00F6` in `world.log`, no `0x0572` after it | the latch (§7). The server did not answer. This is the failure this document exists to prevent |
| the window with garbage in the grid, or the client dies | `0x0572` present, hook log missing its dispatch line | the mask is not exactly 100 bytes, or an item blob is short. `bag-lists.md` §5 has the two lengths |

Only rows 2 and 6 would mean something in this document is wrong, and both are distinguishable
from the logs alone. Everything else is a measurement.

**The follow-on, same launch, no extra variant:** the owner clicks a bag item, then `put`.
`world.log` should show an inbound **`0x00F6` with a 9-byte body**, first byte `5`, then the
bag slot as a `u16`. If the first byte is `5` and the body is a different length, §5.2 is
wrong about the field widths. If nothing arrives at all, the latch was already set - which
means the server answered nothing earlier in the same session.
