# Why the character is naked, and the route that dresses it

**Written 2026-08-19, entirely statically. No client run was spent.**

The blocker `research/equip-block.md` recorded - "item decode is a vtable call at `+0x330`
and the item classes carry no RTTI, so the layout cannot be read linearly" - **is gone**.
The vtable was found from the constructor, the decode method behind it was read off the
listing, and the whole equipped block is now a concrete byte layout. Section 3 is buildable.

Three things in `equip-block.md` were wrong and are corrected here (section 6).

---

## 1. The ranked routes

| # | route | what it costs | status |
|---|---|---|---|
| **1** | **Build the equipped list in the `SetField` character record**, gated by `presence[2]` | ~127 bytes per equip + 11 bytes of framing; one new function in `crates/net` | **Fully read. Section 3 is the layout.** Recommended |
| 2 | The five inbound opcodes that decode an item at runtime - `0x00AC`, `0x00AD`, `0x0149`, `0x015F`, `0x0160` | same item body as route 1, plus decoding each packet's own framing | Item body is the *same* work; their outer framing is undecoded and it is **not established** that any of them writes the equipped array |
| 3 | Deliberate underrun + `ELog` to locate a mis-sized field | one client run | Now a **debugging** tool for route 1, not a discovery route |
| — | `0x0138`, `0x0107`, `0x0114` | — | **Ruled out. Section 5.** |

**Why route 1 rather than route 2, given both need the same item body.** Route 2's appeal
was containment: the character record has no resynchronisation point, so a wrong field width
breaks world entry outright, whereas a bad standalone packet only breaks itself. That is
real, but route 2 buys it by adding an *undecoded* outer framing on top of the same item
body - so it trades one unknown for two. And route 1 has a cheap pre-flight the record has
always had: `python tools/channel_smoke.py` decodes the server's real bytes over the
independent Python transport before any launch.

**Route 2 is the right fallback** if a launch shows the record is rejected, because it lets
the same item bytes be tested without risking world entry. Its five handlers are named in
section 5.4.

---

## 2. How the block is reached, and the trap in the presence byte

`FUN_140304b20`'s gate at **`0x1403061a0`** carries key `0x143abedb0` (table entry 6), whose
CRT initialiser at `0x140023442` is `MOV byte ptr [0x143abedb2],1` - **presence byte 2**.
**[L]**, read from the initialiser, same method that settled `presence[0]`.

Entry 6 gates two regions. The first, `[0x140305794, 0x140305d7d)`, has **no packet reads**
- it is a post-pass over already-decoded items, and only for item ids in
`[1660000, 1670000)` (`ADD EAX,0xffe6aba0 / CMP EAX,0x2710` at `0x1403057c6`). It runs
harmlessly on an empty inventory. **[L]**

The second, `[0x1403061c9, 0x14030661c)`, is the one that reads.

### The trap: presence byte 2 opens three lists, not one

The region calls two helpers after the equipped loop, and **both re-gate on the same
presence byte**:

| helper | gate key | presence byte | reads when open |
|---|---|---|---|
| `FUN_14030b6f0(closure, 1)` | `FUN_1403023d0(out, 1)` -> `0x143abdb20` | **2** | one `u16`-terminated list |
| `FUN_14030b9e0(closure, 1)` | `FUN_1403023d0(out, 1)` -> `0x143abdb20` | **2** | **three** `u16`-terminated lists |

**[L]** for both: `0x14030b722 MOV EDX,R15D` (= `param_2` = 1) then `CALL 0x1403023d0` in
`FUN_14030b6f0`; in `FUN_14030b9e0` `EDX` is copied to `EDI` and `[RSP+0x3c]` at `0x14030ba0a`
and **never reloaded** before `0x14030ba1a CALL 0x1403023d0`, so the argument is still 1.
`FUN_1403023d0`'s `case 1` is `0x143abdb20` **[L]**, and that key's initialiser at
`0x140022952` is `MOV byte ptr [0x143abdb22],1` - **byte 2** **[L]**.

`FUN_14030b9e0` runs its read loop for outer index 2, 3 and 4 only: `R15 = (param_2-5 <= 1)`
= 0 at `0x14030ba4d`, and it proceeds only where `FUN_140255790(index) == R15`, and
`FUN_140255790` returns 1 for index 0 and 1 and 0 otherwise **[L]**. Hence three lists.

So **setting `presence[2]` costs four extra `u16` terminators** beyond the equipped list
itself. Sending `presence[2] = 1` and forgetting them desynchronises the record.

`FUN_14030b6f0` can be skipped instead: the region's leading `u8` (below) is tested at
`0x1403062dd` and a **non-zero** value skips that call **[L]**. `FUN_14030b9e0` cannot be
skipped.

### The last region is skipped, and that is measured

`0x140306304 CMP dword ptr [RBP+0x3118],0x0 / JZ 0x140306615` guards the rest of the region.
`[RBP+0x3118]` is `FUN_140304b20`'s **4th argument**, established in
`research/charrecord-flag7.md` from the prologue, and the call site passes **0**. So
`[0x140306311, 0x140306613)` - the second copy of the 1660000-range post-pass - does not run.
**[D]**

---

## 3. The wire layout

### 3.1 Where it goes in the record

I walked `FUN_140304b20`'s full 18660-byte listing mechanically: every `CALL 0x1402fa9a0`
gate site, the key it loads, the `JNZ` into its body and the `JMP` past it; then listed every
packet-read call site not inside a closed gate's `[body, skip)` range, in address order.
It finds **43 gates**, which matches `charrecord-presence-map.md`.

**Instrument control:** with `presence = {0}` it reproduces the 224-byte record already on
the wire exactly - `raw[100]`, `u8`, `u32`, `u8` (loop-1 count), `u32` (loop-2 count), `u8`
(bool), the stat block, four `u8` at `0x140304e79`/`e8c`/`ee9`/`f4c`, and **one final ungated
`u8` at `0x140308b3f`**. The only extra rows it prints are the loop and optional-string
bodies that a zero count or a zero flag skips, exactly as `crates/net` documents them.

With `presence = {0, 2}` the *only* change is that the entry-6 reads appear **between the
three string flags and the final ungated `u8`**. **[L]**

```text
off   len  what                                             value
  0   100  presence array                    byte 0 = 1, byte 2 = 1, rest 0
100    11  the six head fields                              0
111   108  the character stat block (map id at its off 84)  character_stat_block
219     1  u8  -> dword [param_1+0x118b]                    0
220     1  u8  optional-string flag A                       0
221     1  u8  optional-string flag B                       0
222     1  u8  optional-string flag C                       0
---------- entry-6 gate at 0x1403061a0 fires here ----------
223     1  u8  flagA          0x1403061cc                   0
224   ...  the equipped list, below
  +     2  u16 0x0000  terminator of the equipped list
  +     2  u16 0x0000  FUN_14030b6f0's list   (present only when flagA == 0)
  +     6  u16 0x0000 x3  FUN_14030b9e0's three lists
---------- entry-6 region ends ----------
  +     1  u8  the final ungated read at 0x140308b3f        0
```

With four equips at 125 bytes each the record is **743 bytes** (today it is 224).

### 3.2 The equipped list

```text
repeat:
    u16  slot                0x1403061fc (first) / 0x1403062c9 (subsequent)   [L]
    if slot == 0: stop
    <item body>              FUN_1403095e0 at 0x14030621e                     [L]
```

The item is stored at `record + 0x1a8 + slot*0x10` **only if `1 <= slot <= 31`**
(`LEA EAX,[RCX-1] / CMP EAX,0x1e / JA` at `0x140306229`) **[L]**. A slot outside that range
is still *decoded* and then discarded - so cash-equip slots cost bytes and achieve nothing.
Send only 1..31.

`TestCharD`'s stored slots 5, 6, 7, 11 are exactly this numbering: `FUN_140253980`, the
client's own slot validator, requires `itemId/10000 == 104 -> slot 5`, `106 -> 6`, `107 -> 7`,
and weapons -> slot 11 **[L]**. Those are the four values already in the database.

### 3.3 The item body

`FUN_1403095e0(out, packet, pool)` reads a **`u8` type** and dispatches **[L]**, listing at
`research/msexe-equipdecode.txt`:

```text
type 1 -> FUN_14030ddb0(pool,        &item)   object size 0x467, ctor FUN_1402f7da0
type 2 -> FUN_14030db00(pool+0x970,  &item)   ctor FUN_1402f7cd0
type 3 -> FUN_14030e340(pool+0x12e0, &item)   ctor FUN_1402f8b40
other  -> item = null, no further read
then      (*(item->vtbl + 0x358))(item, packet)
```

**The vtables, with a positive control.** `FUN_1402f7da0` stores its vtable with
`48 8d 05 <disp> / 48 89 07` at `0x1402f7dbd`, giving **`0x14327E1D8`**; `FUN_1402f7cd0`
gives `0x14327E588`. The control that these are the item vtables: `vtable+0x88` is the
`GetType` that `FUN_14030ca50` switches on, and it reads `b8 01 00 00 00 c3`
(`return 1`) for type 1 and `b8 02 00 00 00 c3` for type 2. **[L]**

Therefore **`vtable+0x358` for type 1 is `FUN_140304100`** (qword at `0x14327E530`), and for
type 2 `FUN_140304450`. **[L]**

Full type-1 body, read off `research/msexe-itemslot-equip-decode.txt`,
`msexe-itemslot-base.txt`, `msexe-itemslot-b40.txt`, `msexe-itemslot-800.txt`,
`msexe-itemslot-cce00.txt`, `msexe-itemslot-cd090.txt`. Every row is **[L]** unless marked.

```text
len  read at      what                                          send
---- ----------- --------------------------------------------- ---------------------------
  1  1403095fb   u8   item type                                 1
---- FUN_1403035a0, the base decode ---------------------------------------------------
  4  1403035c5   u32  itemId                                    1040003 / 1060002 / ...
  1  140303787   u8   hasCashSN                                 0
 (8) 14030379d   u64  cash item serial   ONLY if hasCashSN != 0 omitted
  8  1403037b9   u64  dateExpire                                see "values" below
  4  1403037c1   u32  -> +0x48                                  0
  1  1403037cc   u8   -> +0x4c (bool)                           0
---- FUN_140303b40(this+0x62) --------------------------------------------------------
  4  14030381d   u32  statMask   (FUN_140303800)                0
 (…)             17 optional u16, one per bit 0..16 of statMask omitted
  4  140303b66   u32  optMask                                   0
 (…)             21 optional fields, bits 0..20 of optMask      omitted
---- back in FUN_140304100 -----------------------------------------------------------
 13  140304138   raw[13] -> +0x55, byte +0x62 forced to 0       0 x13  (a char[13] name)
  1  140304144   u8   -> protected blob at +0x3af               0
  1  140304183   u8   -> protected blob at +0x3b7               0
  2  1403041c2   u16  -> +0x3bf                                 0
  2  1403041df   u16  -> +0x3c7                                 0
  2  1403041fc   u16  -> +0x3cf                                 0
  2  140304219   u16  -> +0x3d7                                 0
  2  140304236   u16  -> +0x3e7                                 0
  2  140304253   u16  -> +0x3ef                                 0
  2  140304270   u16  -> +0x3df                                 0
  8  14030429e   raw[8] -> +0x4d   ONLY if hasCashSN == 0       0 x8
 32  1403042b7   FUN_1402cce00: raw8, raw8, u32, u32, u32, u32  0 x32
 12  1403042c6   FUN_1402cd090: raw8, u32                       0 x12
  4  1403042ce   u32  -> +0x23e                                 0
  2  1403042dc   u16  -> +0x3f7                                 0
  2  1403042f9   u16  -> +0x3ff                                 0
  2  140304316   u16  -> +0x407                                 0
 (…) 14030435e   FUN_1402cb4f0  ONLY if itemId/10000 == 166     omitted (ours are 104/106/107/130)
  1  14030436d   u8   -> protected blob at +0x303               0
  1  1403043b1   u8   -> protected blob at +0x30b               0
  4  1403043f6   u32  mask (FUN_140303800 at this+0x26b)        0
 (…)             17 optional u16 for that mask                  omitted
  1  1403043fe   u8   tailFlag                                  0
 (…) 140304411   another FUN_140303800  ONLY if tailFlag != 0   omitted
```

**Total: 125 bytes**, and it is 125 whichever way `hasCashSN` goes - the 8 bytes it controls
are read either by the base decode into `+0x38` or by the equip decode into `+0x4d`. **[D]**

**The three bitmasks are the whole reason this is tractable.** `FUN_140303800` is
`u32 mask` followed by up to 17 `u16`, one per bit **[L]**; `FUN_140303b40` is that, then a
second `u32 mask` with 21 optional fields of mixed width **[L]**:

```text
bit  0 u8   bit  1 u8   bit  2 u16  bit  3 u8   bit  4 u8   bit  5 u64  bit  6 u32
bit  7 u32  bit  8 u8   bit  9 u16  bit 10 u32  bit 11 u8   bit 12 u8   bit 13 u8
bit 14 u8   bit 15 u8   bit 16 u8   bit 17 u8   bit 18 u8   bit 19 u64  bit 20 u32
```

All-zero masks read nothing beyond the mask itself. That is what collapses a modern equip
record from "several hundred bytes of unknown fields" to 125.

### 3.4 The values, which is where the NPC lesson applies

Layout is settled; **values are not**. The NPC body was structurally perfect and produced
nothing because `isEnabled` and `alpha` were zero. The equivalents here, ranked:

1. **`dateExpire`, the `u64` at `+0x40`.** Zero is `1601-01-01` as a FILETIME - an item that
   expired 400 years ago. Every MapleStory server sends a "permanent" sentinel instead;
   the usual value is `0x00_00_C9_2A_69_C0_00_00` (150842304000000000, i.e. 2079-01-01).
   **[I]** - the constant is from convention, not from this binary, and I did **not** find a
   client-side expiry check. It costs nothing to send, so send it.
2. **The two date-shaped `raw[8]` fields inside `FUN_1402cce00`** and the one inside
   `FUN_1402cd090`. Same shape, same argument, lower confidence about what they are. Try
   zeros first; they are the second thing to change.
3. Everything else can stay zero: every mask is a "read more" switch, and the 13-byte block
   is a fixed-size name buffer whose terminator the client writes itself (`+0x62 = 0`).

**A corroboration worth recording, because it says the whole reading is right.** The *type-2*
decode `FUN_140304450` reads an extra `raw[8]` only when
`itemId - 0x1f95f0 < 10000 || itemId - 0x238d90 < 10000` - that is **2070000..2079999** and
**2330000..2339999**, which in MapleStory are throwing stars and bullets, the two consumable
families that carry a serial. Real game semantics fell out of the listing without being
looked for.

### 3.5 Build order

1. Set `presence[2] = 1` alongside `presence[0] = 1`.
2. Replace the record's trailing `[0u8; 5]` with `[0u8; 4]`, the equipped block, and
   `[0u8; 1]`.
3. Equipped block = `u8 flagA(0)`, then for each equip `u16 slot` + the 125-byte item, then
   `u16 0`, then `u16 0` (b6f0), then `u16 0` x3 (b9e0).
4. `python tools/channel_smoke.py --set-field-probe` **before** any launch: it decodes the
   real server's real bytes with the independent Python transport, so a width error shows up
   there. Add an assertion that the record's byte at offset 2 is 1 and that the block is
   where the walk says it is.
5. One launch. What to watch, and what each outcome means:
   * **Character dressed** - done.
   * **No fault, still naked, Equipment window still empty** - the layout is right and a
     *value* is wrong; go to section 3.4 item 1.
   * **No fault, still naked, but the Equipment window now lists items** - the items decoded
     and the *avatar* is not being rebuilt; that is a different (and much smaller) problem.
   * **Client faults, or freezes at "Connecting..."** - the record desynchronised. The
     client's readers throw on underrun and the throw is reported in `ELog` (`0x008F`/`0x0090`)
     with section-relative RVAs; `tools/pdata_lookup.py` turns them into functions, which
     names the field that was mis-sized.

---

## 4. What the character-select screen does differently, since it works

Character select dresses its characters from the **compact** look, `FUN_1402ee8d0` -
`(u8 slot, u32 itemId)` pairs terminated by `0xFF`, which `crates/net::avatar_look` already
builds. That reader validates each pair with `FUN_140253980(itemId, slot, 2, 1)` and
**silently drops** any pair that fails **[L]**. Our four pairs pass (they are on screen), so
the slot numbering in the database is confirmed against the client's own table.

`FUN_1402ee8d0` is **not** called from `FUN_140304b20` - `tools/callers.py` gives 17 call
sites in 16 functions and the record decoder is not among them. That part of
`equip-block.md` stands.

---

## 5. Ruled out - and why, so nobody re-tests them

### 5.1 `0x0138` is dead code, at byte level

Previously ruled out by a launch that measured `FUN_1420dd920` never firing, with the
explanation "the loop over `user+0x1200` was empty". **That explanation was wrong, and the
real one is stronger.** The apply is guarded by a function that unconditionally returns 0:

```text
142797de1  8b 8b 0c 03 00 00   MOV  ECX,dword ptr [RBX+0x30c]
142797de6  e8 f5 de 05 fe      CALL 0x1407f5ce0        ; body is  33 c0 c3  =  xor eax,eax; ret
142797deb  85 c0               TEST EAX,EAX
142797ded  74 18               JZ   0x142797e07        ; always taken
142797def..142797e06                                   ; FUN_1420dd920, FUN_1420dd220 - unreachable
```

**[L]** - the `CALL` is a direct `rel32` (verified by decoding the bytes, not by trusting the
decompiler) and `0x1407f5ce0` really is three bytes of `xor eax,eax; ret`. So the loop body
could never have run whatever `user+0x1200` contained, and no other trigger or timing would
change that. `/OPT:ICF` means that 3-byte stub is probably shared across the image; that
does not weaken the conclusion, it only means the original function name is unrecoverable.

### 5.2 `0x0107` (`FUN_142d95ab0`) only logs

It decodes a compact look into a stack object and then does exactly one thing with it:
a 32-iteration loop formatting `"[BP:%02d] %d"` and passing each line to `FUN_1415eca30(.., 0xb)`,
followed by a 31-character separator string. No apply, no user lookup. **[L]**

It is, however, a free **read-back instrument**: it makes the client print the equipment
array it parsed. If the destination of `FUN_1415eca30(.., 0xb)` turns out to be a readable
log, `0x0107` becomes a way to check a compact look without guessing.

### 5.3 `0x0114` (`FUN_142da4820`) does not touch the local avatar

It reads `u32`, five strings, a helper, `u32`, `u8`, then the compact look, then `u8`, and
builds a separate **0x4b8-byte** object via `FUN_140fc4610`, finally calling
`FUN_142da4cf0(param_1, ...)` and setting `param_1+0x3264`/`+0x3268`. It never reaches the
avatar apply. **[D]**

The instrument: a direct-call/tail-jump reachability walk from each handler to
`FUN_140f80140` - the primitive `FUN_1420dd920` tail-jumps into, and the only thing in this
chain that actually re-dresses a rendered avatar. **Control: the walk finds
`0x140f80140 <- 0x1420dd920 <- 0x142797be0 <- 0x142d012e0`, the `0x0138` path**, at depth 4.
From `0x142da4820` (0x0114) it visits 700 functions and finds nothing; from `0x142d95ab0`
(0x0107), 245 functions and nothing.

Bound on that negative, stated honestly: **it follows `call rel32` and tail `jmp rel32`
only.** A vtable call anywhere on the path would be invisible to it. Getting the control to
pass at all required adding tail jumps - the first version reported "no path" for the known
positive, which is exactly the failure mode `CLAUDE.md` warns about.

### 5.4 There is no cheap "dress the avatar" packet at all

`FUN_140f80140` has **23 direct callers**; none of them appears in
`research/msexe-gamestage-cases.txt`, the 273-case inbound table. So no inbound opcode
reaches the avatar-apply primitive by a direct call. **[D]**, with the same bound as above.

Combined with 5.1-5.3: the appearance has to come from the equipped list.

### 5.5 The five runtime item packets - not ruled out, just not decoded

`FUN_1403095e0` (the pooled factory used by the record) has only 4 call sites, all inside
`FUN_140304b20` and its two list helpers. But **`FUN_140303530`** is a second factory - same
`u8 type`, same `vtable+0x358` decode, a global allocator instead of a pool - with **46 call
sites in 38 functions**, and six of those functions are inbound handlers:

| opcode | handler | item decodes |
|---|---|---|
| `0x0070` | `FUN_142d51930` | 3 |
| `0x00AC` | `FUN_142d60d40` | 4 |
| `0x00AD` | `FUN_142dae820` | 1 |
| `0x0149` | `FUN_142da97d0` | 2 |
| `0x015F` | `FUN_142da3f30` | 1 |
| `0x0160` | `FUN_142da42b0` | 1 |

`0x00AC` reads a leading `u8` and switches on it across 31 sub-cases, several of which decode
an item - the shape of an inventory/effect multiplexer. **Nothing about their framing is
decoded and nothing establishes that any of them writes the *equipped* array** rather than
the bag. Recorded so the next pass starts here rather than re-finding it.

---

## 6. Corrections to `research/equip-block.md`

1. **"The gated region reads a `u8` then two `u16`-terminated loops."** There is **one**
   `u16`-terminated loop inline (`0x1403061fc` / `0x1403062c9`). What looked like a second
   loop is `FUN_14030b6f0` and `FUN_14030b9e0`, called at `0x1403062ee` and `0x1403062ff` -
   lambdas over a captured-reference struct, each with its own presence gate, contributing
   **four** more lists. The "equipped + equipped cash" reading was wrong: there is no cash
   list here at all, and slots outside 1..31 are decoded and thrown away.
2. **"`FUN_14030b560` reaches the item through a vtable call at `+0x330`, and that is the
   blocker."** `FUN_14030b560` is not on the decode path. It is the post-pass in the
   *no-reads* region, it only touches items with ids in `[1660000, 1670000)`, and `+0x330` is
   `FUN_1402fbb30` = `return this + 0x242` - an accessor, not a decoder. The decode is at
   **`+0x358`**, and the RTTI dead end never had to be crossed: the vtable comes from the
   constructor, and `vtable+0x88` returning the literal 1/2/3 confirms which class it is.
3. **"A `do { } while (lVar17 < 0x20)` loop I could not place."** Its back-edge is at
   `0x140305923` (`INC R15 / ADD RBX,0x10 / CMP R15,0x1f / JLE 0x1403057b1`) in the
   **no-reads** region - 31 iterations over a `0x10`-stride array whose base is
   `LEA RBX,[R12 + 0x1c0]`. It reads nothing from the packet. (I am not asserting what that
   base is: `+0x1c0` does not line up with the equipped array's `+0x1a8 + slot*0x10`, and the
   provenance of `R12` at that point is worth checking before anyone builds on it.) The
   instinct to refuse to place the loop from the decompiler was right.

The document's central claim - that `SetField` carries no compact avatar look, and that the
appearance must come from the equipped list - is **unchanged and confirmed**.

---

## 7. New files from this pass

| file | what |
|---|---|
| `research/msexe-equipdecode.c` / `.txt` | `FUN_1403095e0` the item factory, `FUN_14030b6f0`, `FUN_14030b9e0`, `FUN_1407f5ce0`, `FUN_1420dd920` |
| `research/msexe-itemalloc.c`, `-t1/-t2/-t3.txt` | the three per-type allocators, `FUN_1403023d0` (the bag presence keys), `FUN_140253980` (the slot validator) |
| `research/msexe-itemslot-decode.c` | `FUN_140304100` equip decode, `FUN_140304450` bundle decode, `FUN_140303530` the second factory |
| `research/msexe-itemslot-equip-decode.txt` | the equip decode **listing** - the authority for field order |
| `research/msexe-itemslot-sub.c`, `-base/-b40/-800/-cce00/-cd090.txt` | every sub-decoder the equip decode calls |
| `research/msexe-invlist-b6f0.txt`, `-b9e0.txt`, `msexe-invkey-23d0.txt` | the two extra list readers and their presence keys |
| `research/msexe-charrecord-full.txt` | the **complete** 18660-byte listing of `FUN_140304b20`, which the gate/read walk needs |
| `research/msexe-invop-00ac.c` | the `0x00AC` handler, for route 2 |
