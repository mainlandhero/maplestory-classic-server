# Miniroom room-open: the per-room-type payload, for all three room types

2026-09-09. Static, from `client-patched/MapleStory.exe` (base `0x140000000`). No Ghidra,
no client run, no code written. Continues `research/trade-2026-09-09.md`, which decoded the
envelope; this file decodes what that one left as "the undecoded part".

Claims are tagged **[L]** read off this client's listing / string table, **[D]** derived
from tagged facts, **[I]** inferred (including anything from `ModernMapleSource`, which is a
different game version).

**Headline, and it is smaller than the brief assumed:** there is **one** per-member payload,
not three. `[vt+0x1C8]` is the *same function* `0x141C423D0` in all three room classes, and
it decodes an **avatar look**, not game state. The per-room-type difference lives in the
*final* virtual call, which is `[vt+0x188]` (not a shared slot), and for **trade it is a bare
`ret` that reads nothing**. The games each append a second member-indexed loop of **20 raw
bytes** per player plus a string and a byte.

And: **`B == 3` is Omok, `B == 4` is Match Cards.** [L]

---

## 0. Instruments, and two of them were wrong

Positive controls run *before* anything below was believed.

| instrument | control | result |
|---|---|---|
| `tools/dis_at.py` | its own docstring control `0x140ca22a0 0x52` vs `listing.py` | **passes**, same 27 instructions |
| `tools/vtable dump` (scratch) | trade vtable `0x143431CC8` must reproduce the six slots `trade-2026-09-09.md` derived independently | **passes**, all six exact |
| immediate scanner (scratch) | `0x17E` must yield the **59** `mov edx,0x17e` that `trade-2026-09-09.md` counted independently | **passes**, exactly 59 |
| `tools/ripstrings.py` | its own docstring control `0x141b2c7c0 934` | **FAILS - see below** |
| `tools/reads.py` | speaks on every other function here | **passes, but is short on two - see below** |

### `ripstrings.py`'s documented positive control is stale

Its docstring says `python tools/ripstrings.py 0x141b2c7c0 934` "must print at least one
decoded string". It prints none. The reason is not the tool: `python tools/listing.py
0x141b2c7c0 | grep -c rip` returns **0** - that function contains no RIP-relative
instruction of any kind, so there is nothing for the tool to resolve. [L] The control cannot
distinguish a working tool from a broken one and never could.

I replaced it with a control of my own before using the tool: on the two game constructors
it decodes `nameTag0`, `nameTag1`, `emptyTag`, `score` out of `.rdata` as UTF-16. [L] That is
the instrument speaking. Everything I report from `ripstrings.py` rests on that, not on the
docstring.

*This is only the control being stale; the tool looks correct. Someone should fix the
docstring, but I was told to write one file and not edit others, so it is recorded here.*

### `reads.py` came back short **twice**, both times for the same structural reason

`reads.py` walks `.pdata` functions. **A call target with no `.pdata` entry is a wall it
does not climb, and it says nothing about having stopped.**

* `python tools/reads.py 0x141c1b080 6` reports **4 reads**. The function also calls
  `0x1402d1b50`, which has **no `.pdata` entry** (`pdata_lookup.py`: "falls in no
  function"). That target is `mov r8d,0x14 / jmp 0x1406e9170` - a **20-byte raw read**, once
  per member. [L] A body built from the tool's four reads would have been **20·K bytes short
  per room**. This is the exact failure mode `CLAUDE.md` records as having killed the client
  twice.
* Same omission in `0x141e99c40` (Omok), which calls the same thunk. [L]

I therefore hand-read the listing for every function below rather than trusting a read
count, and I resolved every call target through `pdata_lookup.py` first.

### `reads.py`'s `gated?` label is conservative, and I checked the branches

Every read in the avatar decoder from `1402ee9b6` onward is marked `gated?`. I read the
branches. The two equip loops are genuinely conditional (they are loops); **everything from
`1402eea4a` to the end of the function is unconditional** - the only branch in that stretch
is the `test eax,eax / jg` at `1402eea80`, whose two arms both converge at `1402eeaa3`. [L]
`trade-2026-09-09.md` explicitly declined to build a packet from `gated?` reads without
reading the branches; this is that reading.

### RTTI cannot name these classes, and there is a measured reason

`tools/rtti.py --list Omok` and `--list Mini` return **0 of 1764** descriptors. That is not a
broken search - `--list CField` returns 3, so the tool speaks. The reason is in the objects:
the complete-object-locator pointer at `vtable-8` is **`0`** for the trade vtable and garbage
(`0x67006e`, ASCII `"n\0g\0"`) for the Match Cards one. [L] These UI classes ship without
usable RTTI. Naming had to come from strings, and did.

---

## 1. Which room type is which

`FUN_141C3D980`, the mode-4 handler, decoded independently of `trade-2026-09-09.md`: [L]

```
141c3d9c7  raw 4 -> A
141c3d9d9  raw 4 -> B          (edi)
141c3df0d  raw 4 -> third      ; read BEFORE the room type switch
141c3df14  ecx = B - 1 ; je -> 141c3df73   ; B==1
141c3df1b  ecx -= 2    ; je -> 141c3df4c   ; B==3
141c3df20  cmp ecx,1   ; jne-> 141c3df9d   ; anything else -> error, no window
                                            ; equal -> falls into 141c3df25 (B==4)
141c3df25  new(0x1860) ; call 0x141c145f0   <- B == 4
141c3df4c  new(0x1b40) ; call 0x141e93230   <- B == 3
141c3df7f  new(0x1678) ; call 0x142146d90   <- B == 1
141c3dfeb  [room+0x304] = B
141c3dff4  [room+0x308] = third
141c3e000  call 0x141c3ed00
```

That reproduces the brief's table exactly, from the listing, so the starting point is sound.

### `B == 4` is **Match Cards** [L]

The constructor `FUN_141C145F0` **loads three Match Cards WZ paths itself**:

| site | string id | text |
|---|---|---|
| `141c1473b` | `0x05CF` | `UI/Minigame.img/MatchCards/backgrnd` |
| `141c14b99` | `0x05D0` | `UI/Minigame.img/MatchCards/card` |
| `141c14e84` | `0x05D1` | `UI/Minigame.img/MatchCards/number` |

`0x05CF` has **2** decoded references in the whole image and one of them is this
constructor; `0x05E3` (`UI/Minigame.img/MatchCards/effect%d`) has **1**, at `141c1d7a1`, in
the same code family. [L]

Independent second line of evidence, from the room-open payload itself: the class's
`[vt+0x188]` handler ends by reading a `u8` into `[room+0x17a0]` and switching on it: [L]

```
value 0 -> [+0x184c]=4, [+0x17b4]=0x0c   ; 12 cards
value 1 -> [+0x184c]=5, [+0x17b4]=0x14   ; 20 cards
value 2 -> [+0x184c]=6, [+0x17b4]=0x1e   ; 30 cards
```

12 / 20 / 30 are the three Match Cards board sizes, and the code immediately formats string
`0x05CD` = `back%d` with that value to pick the board canvas. [L]

### `B == 3` is **Omok** [L]

The constructor references no Omok path string (there is no `UI/Minigame.img/Omok/*` in the
table at all - I enumerated every `Minigame` string). So the constructor cannot settle it and
I went to the vtable instead.

`[vt+0x178]` of the `B == 3` class is `FUN_141E99700`. That one function loads: [L]

| string id | text |
|---|---|
| `0x05DB` | `Mushroom` |
| `0x05DC` | `Slime` |
| `0x01F8` | *"Your opponent requests a tie."* |
| `0x01FA` | *"Your opponent denied your request for a tie."* |
| `0x01FC` | *"Your oppentent has requested to withdraw their/their last move."* |
| `0x0206` | *"You have double-3's."* |
| `0x0207` | *"You can't put it there."* |

`Mushroom` and `Slime` are the two Omok stone types. *"You have double-3's"* is the
**gomoku forbidden-move rule**; *"You can't put it there"* is illegal stone placement;
*"withdraw their/their last move"* is undo. None of these has any meaning in a card-matching
game.

**The set is exclusive.** Scanned image-wide, `0x0206` and `0x0207` are referenced in the
`141E9xxxx` family **and nowhere in `141C1xxxx`**; `0x01FD` (*"Request to withdraw your last
move?"*) and `0x01FB` (*"You can only request a handicap once per game."*) likewise. [L] The
Match Cards counterpart `[vt+0x178] = 0x141C1AD70` carries `0x01F8`/`0x01FA` (the tie
strings, which both games have) and **not** `0x0206`/`0x0207`. [L]

So: **`B == 1` trade, `B == 3` Omok, `B == 4` Match Cards.** [L] The `B == 1` assignment is
unchanged from `trade-2026-09-09.md`.

---

## 2. The three vtables, and what is actually shared

Resolved from each constructor's `mov [this], rax` and verified against the trade vtable
whose slots were already known:

| class | constructor | object size | vtable |
|---|---|---|---|
| trade | `142146D90` | `0x1678` | `0x143431CC8` |
| Omok | `141E93230` | `0x1B40` | `0x143415400` |
| Match Cards | `141C145F0` | `0x1860` | `0x143405628` |

Slot-by-slot over the interesting range: [L]

| slot | trade | Omok | Match Cards | |
|---|---|---|---|---|
| `+0x160` | `141c3d0f0` | `141c3d0f0` | `141c3d0f0` | shared |
| `+0x168` | `141c3ef70` | `141c3ef70` | `141c3ef70` | shared |
| `+0x170` | `14214a900` | `141e996d0` | `141c1ad40` | differs |
| `+0x178` | `14214a9c0` | `141e99700` | `141c1ad70` | differs |
| `+0x180` | `14214aa80` | `141e9a2b0` | `141c1b3c0` | differs |
| **`+0x188`** | **`141c3f2e0`** | **`141e99c40`** | **`141c1b080`** | **differs - the final call** |
| `+0x190` | `14214aaa0` | `141e9a5e0` | `141c1b6f0` | differs |
| `+0x198` | `141c3f300` | `141c3f300` | `141c3f300` | shared |
| `+0x1a0` | `141c3f7f0` | `141c3f7f0` | `141c3f7f0` | shared |
| **`+0x1c8`** | **`141c423d0`** | **`141c423d0`** | **`141c423d0`** | **shared - ALL THREE** |

**The brief's premise that `[vt+0x1C8]` has to be decoded three times is false.** [L] One
decode covers all three room types. That is the single most useful fact in this file.

---

## 3. `[vt+0x1C8]` = `FUN_141C423D0` - it is an avatar look, once per member

`FUN_141C423D0(room, slot, packet)`. Its only packet contact is one call: [L]

```
141c42553  xor r9d, r9d
141c42556  lea r8, [rsp+0x70]        ; out: a name/string buffer
141c4255b  mov rdx, r12              ; the packet
141c4255e  lea rcx, [rbp-0x40]
141c42562  call 0x1402EE8D0          ; <-- the whole payload
```

**Unconditional.** Between the function entry and `141c42562` the only branches are
`141c42509`, `141c4251a` and `141c42542`, and all three merely skip calls to error reporters
(`142e52dd0`, `142e54290`); every path converges on `141c42550`. [L] Everything after the
call (`[vt+0x170]`, the `slot < 8` bound, the `0x140f7f030` draw) touches the packet not at
all - `[vt+0x170]` is passed two stack locals, never `packet`. [L]

`FUN_141C3ED00` calls this only for `0 <= slot < 8`, both checks *before* the call. [L]

### `FUN_1402EE8D0` - the wire format

Read primitives verified individually rather than trusted from `reads.py`'s table: [L]

* `0x1406e8ae0` u8 (`cmp edi, 1` underflow guard), `0x1406e8b80` u16 (`cmp edi, 2`),
  `0x1406e8c20` u32 (`cmp edi, 4`)
* `0x1406e8f00` is literally `jmp 0x1406e8c20` - a **u32 thunk**, 4 bytes
* `0x1406e9170` raw, count in `r8d`, dest in `rdx`
* `0x1406e9050` str: `movzx r8d, word ptr [rdx]` then `add [rbx+0x24], len+2` -
  **u16 length prefix, advance = 2 + len**

```
1402ee8f3  u8       -> look+0x20
1402ee8fe  u8       -> look+0x21   (widened to u32)
1402ee90c  u32      -> look+0x25
1402ee917  u32      -> look+0x29
1402ee922  u32      -> look+0x1bd
           ; zeroes look+0x39 .. look+0x139 (0x100 B = two 32-dword equip arrays). no reads.
1402ee994  u8       -> READ AND DISCARDED  (al dead before the next call)
1402ee99c  u32      -> look+0x39            (equip array index 0)
--- EQUIP LOOP A ---
1402ee9a7  u8 slot
           while slot != 0xFF:
1402ee9b6    u32 value
             if (u8)(slot-1) <= 0x1e and FUN_140253980(value,2,slot,1):
                 look[0x39 + slot*4] = value      ; the value is READ either way
1402ee9e8    u8 slot
--- EQUIP LOOP B ---   (identical shape, different destination array)
1402ee9f7  u8 slot
           while slot != 0xFF:
1402eea06    u32 value  ->  look[0xb9 + slot*4]
1402eea3b    u8 slot
--- TAIL, all unconditional (branches read) ---
1402eea4a  u32      -> look+0x2d
1402eea55  u32      -> look+0x31
1402eea60  u32      -> look+0x35
1402eea6b  u32      -> look+0x1c1
1402eea79  u32      -> look+0x1c5   ; stored as (v > 0 ? v % 360 : 0)  -- 0x168 = 360, degrees
1402eeaac  u8       -> look+0x1c9   ; stored as a bool (setne)
1402eeabf  u32      -> look+0x1ca
1402eeada  raw 4    -> look+0x1b9
1402eeaef  raw 0x80 -> look+0x139   ; 128 bytes
1402eeaf7  u32      -> look+0x1d2   ; via the 0x1406e8f00 thunk
1402eeb12  raw 0xd  -> look+0x1d6   ; 13 bytes
```

**Terminator is exactly `0xFF`** in both equip loops (`cmp al, 0xff`), not "any negative". [L]
The equip-slot validity test gates only the *store*, never the *read*, so the byte count does
not depend on the item ids being valid. [L]

**Avatar byte count** (`E` = total entries across both equip loops):

```
head            19
loop A        1 + 5*nA
loop B        1 + 5*nB
tail           174
              ------------------------
              195 + 5*E        bytes     [D, from the widths above]
```

With no equips at all: **195 bytes per member.**

### Candidate names, all [I]

`ModernMapleSource`'s `AvatarLook.encode` is a **different version** (it has three equip
loops; this client has two), so this is candidate naming only. The **head aligns
field-for-field**, including a detail worth noting: the reference emits a byte it comments
`// ignored`, in exactly the position where this client reads a byte and **discards** it.

| offset | reference field | |
|---|---|---|
| `+0x20` | gender | [I] |
| `+0x21` | skin | [I] |
| `+0x25` | a constant `0` filler | [I] |
| `+0x29` | face | [I] |
| `+0x1bd` | job | [I] |
| (discarded u8) | the `// ignored` byte | [I] |
| `+0x39` | hair (equip index 0) | [I] |
| `+0x2d` `+0x31` `+0x35` | weapon sticker / weapon / sub-weapon | [I] |

Past the equip loops the two versions diverge and I offer no names. **The offsets are [L];
the names are [I] and must not be relied on.**

---

## 4. `FUN_141C3ED00` - the member loop

Re-read from the listing; agrees with `trade-2026-09-09.md` and adds the bounds. [L]

```
141c3ed25  u8  -> [room+0x2fc]                    unconditional
141c3ed38  u8  -> [room+0x2f8]                    unconditional
           ; 141c3ed48: r13 = room+0x310, r12 = r13+0x1c0  (0x1c0 = 8 * 0x38)
           ; 141c3ed70..141c3ee4f clears the 8 member slots. READS NOTHING.
141c3ee58  u8 slot
           if (slot & 0x80)      -> break     (test al,al / js)
           if ((u32)slot >= 8)   -> break     (cmp rdi,8 / jae)
141c3ee87    call [vt+0x1c8](room, slot, packet)   ; section 3, 195+5E bytes
141c3ee97    u32 -> member[slot] + 0x00           (id)
141c3eea9    str -> member[slot] + 0x08           (name, u16-prefixed)
141c3eeeb    u16 -> member[slot] + 0x10
             ++[room+0x300]
141c3ef00  u8 slot ; loop while non-negative
141c3ef10  rbx = <global 0x143AA8520> if an RTTI-ish type check passes, else 0
141c3ef61  jmp qword ptr [rax + 0x188]            ; TAIL CALL, rcx=rbx, rdx=packet
```

**The member-loop terminator is any byte outside `0..7`** - `0x80..0xFF` breaks on the sign
test, `0x08..0x7F` on the bound. `0xFF` by convention. [L]

`[room+0x2f8]` is used elsewhere as a **member slot index**, bounded `0..7` and used to reach
`[room + 0x2f8*0x38 + 0x330]`, the member's avatar object (`FUN_141C3D0F0`, the shared
`[vt+0x160]`). It is most likely "my own slot in this room". [D] `[room+0x2fc]` is compared
against member counts in `FUN_141C3CEF0` / `FUN_141C3D020`. [D] I did not pin either
further; neither affects the byte layout.

---

## 5. The final call at `141c3ef3b` is `[vt+0x188]`, and it differs per room type

Both globals in that epilogue resolve to **`0x143AA8520`** (`141c3ef17 + 0x1e69609` and
`141c3ef3b + 0x1e695e5`), the current-miniroom-dialog pointer `trade-2026-09-09.md` already
names. So the tail call is `room->vt[0x188](room, packet)`. [L]

All three `+0x188` handlers begin by calling `0x141C3F8B0`, which is two instructions -
`mov eax,[rcx+0x2f8] / ret` - a **getter, not a read**. [L] It forks on the byte decoded at
`141c3ed38`. **Both arms converge before any packet read**, so the fork does not change the
wire layout. [L]

### Trade: `0x141C3F2E0` reads **nothing**

```
000141c3f2e0  ret
000141c3f2e3  int3
```

A bare `ret`. [L] The trade room-open packet **ends at the member-loop terminator**. This is
what makes a complete trade byte count possible.

### Omok `0x141E99C40` and Match Cards `0x141C1B080`: identical shape

```
  u8 slot ; if negative -> exit loop        (test al,al / js)   -- NOTE: no upper bound
    ; per member:
    call 0x141c30d00(room + 0x338 + slot*0x38)     ; no reads (verified)
    call 0x1402d1b50([room + 0x340 + slot*0x38], packet)
         -> mov r8d,0x14 ; jmp 0x1406e9170          ; RAW 20 BYTES
    u8 slot ; loop while non-negative
  str  -> Omok [room+0x1b10] / Match Cards [room+0x1780]
  u8   -> Omok [room+0x1b28] / Match Cards [room+0x17a0]
```

`0x338 - 0x310 = 0x28`, so this second loop indexes **the same 8-slot, `0x38`-stride member
array** as `FUN_141C3ED00`; the game state is the sub-object at `member+0x28`. [D]

**The Match Cards trailing `u8` is the board size** - `0`→12 cards, `1`→20, `2`→30
(section 1). [L] The Omok trailing `u8` is stored at `[room+0x1b28]` and I found **no
reader**: `tools/rangescan.py 0x1b28 0x141e93000 0x141ea0000` returns exactly **1** site,
the write itself. [L] **That negative is weak and I am flagging it as such** - `rangescan.py`
matches `[reg+disp]` only, and `CLAUDE.md` records the `mob+0x42c` case where precisely this
shape of scan missed a setter reached through an `lea`'d pointer. Treat the Omok trailing
byte as "one byte, meaning undetermined", not as "unused".

### Two things the server author must know about these two loops

1. **The game loop has no upper bound on `slot`.** The member loop in `FUN_141C3ED00` checks
   `slot < 8`; this one checks **only the sign bit**, then computes
   `room + 0x338 + slot*0x38`. A slot byte of `0x08..0x7F` indexes past the array. [L] Send
   only `0..7`, terminate with `0xFF`.
2. **Opening a game room makes the client send a packet.** When `[room+0x2f8] == 0`, both
   game handlers build and send outbound `0x017E` with `u32 mode = 0x0A, u8 1`
   (`141c1b121` Match Cards, `141e99cf8` Omok). [L] By this project's standing rule an
   unanswered packet freezes the entire UI, so **mode `0x0A` must be handled** before a game
   room is opened. Trade does not do this.

---

## What the server must send

Wire order, top to bottom. `L` = byte length of a name after its u16 prefix. `E` = total
equip entries in a member's two equip loops (0 is legal). `M` = members. `K` = game-state
entries.

### Common envelope - inbound `0x0575`, all three types

| # | width | value | |
|---|---|---|---|
| 1 | `u32` | mode = **4** | [L] |
| 2 | `u32` | A = **0** (any other A is a notice code and reads no further) | [L] |
| 3 | `u32` | B = room type: **1** trade, **3** Omok, **4** Match Cards | [L] |
| 4 | `u32` | third -> `[room+0x308]`; **meaning not determined** | [L] present, [ ] meaning |
| 5 | `u8` | -> `[room+0x2fc]` | [L] |
| 6 | `u8` | -> `[room+0x2f8]`; a member slot index, `0..7` | [L] width, [D] meaning |

**= 18 bytes.**

### Then, for each member (repeat `M` times), then one terminator

| # | width | value | |
|---|---|---|---|
| a | `u8` | slot, **must be `0..7`** | [L] |
| b | — | **avatar look, `195 + 5*E` bytes** - exact layout in section 3 | [L] |
| c | `u32` | character id | [L] |
| d | `str` | name: `u16 len` + `len` bytes | [L] |
| e | `u16` | -> `member+0x10` | [L] width, [ ] meaning |
| | `u8` | **terminator, once after the last member: `0xFF`** | [L] |

Per member = **`204 + 5*E + L`** bytes. [D]

### Then, the per-room-type tail

**Trade (`B == 1`): nothing. `[vt+0x188]` is `ret`.** [L]

**Omok (`B == 3`) and Match Cards (`B == 4`), identical layout:**

| # | width | value | |
|---|---|---|---|
| f | `u8` | slot, **`0..7`** (repeat f+g `K` times) | [L] |
| g | `raw 20` | that member's game state - **20 opaque bytes** | [L] |
| | `u8` | **terminator: `0xFF`** | [L] |
| h | `str` | `u16 len` + `len` bytes | [L] |
| i | `u8` | Match Cards: board size **0**=12 / **1**=20 / **2**=30 cards. Omok: undetermined | [L] / [L]+[ ] |

Tail = **`4 + 21*K + L2`** bytes. [D]

### Totals

| | body bytes | |
|---|---|---|
| trade, generic | `18 + Σ(204 + 5E + L) + 1` | [D] |
| **trade, 1 member, no equips, name length L** | **`223 + L`** | [D] |
| Omok / Match Cards, generic | `18 + Σ(204 + 5E + L) + 1 + 4 + 21K + L2` | [D] |
| **game, 1 member, K=1, no equips** | **`248 + L + L2`** | [D] |

Worked example - trade, one member named `GoodTest` (L = 8), no equips: **231 body bytes**,
of which **195 are the avatar look**. [D]

### The one field I will not guess

The **20-byte game state** at (g) is `raw 20` and this client never decodes it in
`FUN_1402D1B50` - it memcpys it into `member+0x30`'s object. [L] Whatever parses it lives
behind `[vt+0x170]`/`[vt+0x180]` per class and I did not decode it. **Twenty zero bytes is
the safe opening value**; it is the right *length*, which is the part that would kill the
client if wrong. What those 20 bytes mean is open.

---

## What I could NOT determine, and what would settle each

1. **The 20-byte per-member game state (both games).** Length is [L] and exact; contents are
   opaque here. *Settled by:* decoding `[vt+0x180]` (`141e9a2b0` Omok / `141c1b3c0` Match
   Cards), which is where the move handlers read the same sub-object. No client run needed.
2. **The meaning of the avatar look's fields past the equip loops** - `+0x1c1`, the mod-360
   `+0x1c5`, the bool `+0x1c9`, `+0x1ca`, and the three raw blocks (4 / 128 / 13 bytes).
   Offsets and widths are [L]; meanings are not. `ModernMapleSource` diverges from this
   client after the equip loops (it has three loops, this has two) so it cannot answer.
   *Settled by:* a capture of any packet carrying an avatar look - `0x0044` user-enter-field
   almost certainly does. **That costs no client run**: grep the archived `world.log`s.
3. **The third `u32` (`[room+0x308]`).** Read [L], destination [L], meaning not established.
   *Settled by:* `tools/rangescan.py 0x308` across the three class ranges.
4. **The Omok trailing `u8` (`[room+0x1b28]`).** Written, no reader found - and per section 5
   **that negative is weak**, because `rangescan.py` is blind to access through an `lea`'d
   pointer. *Settled by:* changing the question rather than widening the scan - decode
   `[vt+0x170]`/`[vt+0x190]` of the Omok class and look for what consumes it.
5. **`[room+0x2fc]` and `[room+0x2f8]`.** Widths [L]; `0x2f8` as a slot index is [D] from its
   use in `FUN_141C3D0F0`; `0x2fc` is compared against counts and is probably a member count
   or capacity [I]. Neither changes the layout.
6. **`member+0x10` (the `u16`).** Read and stored [L]; no consumer traced.
7. **Whether any of this is right on the wire.** Everything here is static. Nothing in this
   file has been tested against a running client, and per `CLAUDE.md` the cheap check is
   `python tools/channel_smoke.py`, not a launch.

## One process note

While I was working, my scratchpad file `vt.py` was **overwritten by content I did not
write** - a different, functional-looking vtable dumper. Other untracked files appeared in
the tree at the same time (`research/beauty-2026-09-09.md`, `tools/dump_beauty.py`), so the
likely explanation is simply another agent working concurrently and sharing the scratchpad.

I did not use the overwritten file. Every vtable number above comes from a freshly-named
script whose control reproduced the six trade slots `trade-2026-09-09.md` derived
independently. Flagging it because an instrument that is silently the wrong version is the
failure this repo has paid for most often, and because it means **the scratchpad is not
private this session** - anything left there by this work should be assumed shared.

I touched nothing under `crates/`, edited no existing research file, and ran no `git add`
or `cargo` command.
