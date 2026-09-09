# Omok and Match Cards: the in-game move protocol

2026-09-09. Static only, from `client-patched/MapleStory.exe` (base `0x140000000`). No Ghidra,
no client run, no code written. Builds on `research/trade-2026-09-09.md`, which decoded the
miniroom envelope; the room-open payload and the roomType identification are
`research/miniroom-rooms-2026-09-09.md`'s (another agent) and are **not** re-derived here
except where a game field sits inside them.

Claims are tagged **[L]** read off this client's listing / string table, **[D]** derived from
tagged facts, **[I]** inferred.

**Headline.** Both games ride the same `0x017E` / `0x0575` mode space as trade, in the range
`0x11`-`0x23`, and **the range in the brief was neither complete nor exclusive** - the two
move packets are `0x1F` (Omok) and `0x23` (Match Cards), and `0x20` and `0x23` are outside
`0x11`-`0x1C`. The two games use **different mode numbers for their moves** and each game's
inbound table treats the other's move mode as a silent no-op. **The server deals the Match
Cards layout** - the client allocates an array and fills it in one `raw` read straight from
the packet, and has no other write path into it. And **`0x1E` means different things in the
two directions**: outbound it is a 4-byte "my timer expired", inbound it is a 5-byte
"turn changed".

**No mini-game packet has ever been captured.** A grep of every archived `world*.log` in
`previous-runs/` and `research/fixtures/` for `0x017E` returns only the trade create
(`000000000100000000`) and two trade invites. Everything below is static. [L]

---

## 0. Instruments, and three of them were wrong

Positive controls run before anything was believed:

* `python tools/reads.py 0x140304100 2` - passes (mixed direct + helper reads). [L]
* String-id immediate scan: reproduces `0x1F6`/`0x1F9` at `FUN_141c16eb0` and `FUN_141e95720`,
  the two sites `trade-2026-09-09.md` read by hand. [L]
* String-ids-in-a-VA-window sweep over `141c16eb0..141c17400`: returns exactly
  `0x1F6`, `0x1F7`, `0x1F9`, `0x1EF`, `0x5D9`, `0x17E`, `0x20` - the dialog set already known
  for that function. [L]
* `0x017E` builder enumeration: reproduces `trade-2026-09-09.md`'s
  `141826e40` block (`w_raw4 0`, `w_raw4 1`, `w_u8`, SEND, Init, `w_raw4 5`, `w_u32`, SEND)
  instruction for instruction. [L]

### 0.1 `research/msexe-send-opcodes.txt` is NOT a full enumeration

The brief describes it as "a full enumeration of all 1894 outbound builders by opcode". For
`0x017E` it lists **38** `COutPacket` ctor sites. A byte scan for `mov edx, 0x17E` with every
hit re-decoded and then confirmed by a following call to `1406ED520`/`1406EE160` finds
**52** (51 ctor + 1 Init), matching `trade-2026-09-09.md`'s count. [L]

Four of the missing ones are load-bearing here: `141c1cdc1` (`FUN_141c1cd30`), `141c1c6e8`
(`FUN_141c1c6c0`), `141c1ceb4` (`FUN_141c1ce30`), `141e9c081` (`FUN_141e9bff0`). `grep` for
those addresses in that file returns nothing. **Anything that used that file as a complete
set is missing ~27% of this opcode's senders.** [L]

### 0.2 `tools/reads.py` comes back 20 bytes short on the room-open tail

`python tools/reads.py 0x141c1b080 6` reports four direct reads and **no** helper reads. The
listing shows the per-slot loop calling `0x1402D1B50`, which is: [L]

```
1402d1b50  mov rax, rdx / mov r8d, 0x14 / mov rdx, rcx / mov rcx, rax / jmp 0x1406e9170
```

A tail `jmp` into the `raw` reader with **length 0x14 = 20 bytes**. `reads.py` cannot descend
into it because `0x1402D1B50` has **no `.pdata` entry** (`tools/dis_at.py`'s own documented
case), so the walk stops at the call. This is `reads.py`'s docstring failure mode 3 happening
to `reads.py` again, in a new way: not a missing primitive, a missing *function extent*.
**Every read count through a `.pdata`-less thunk in this image is suspect.**

### 0.3 A vtable-slot correction to `trade-2026-09-09.md`

That file lists the trade class's slots `+0x168 .. +0x1C8` and one at `+0x1D8`+. Two problems: [L]

* The room object carries **three** vtables (`[room+0]`, `[room+8]`, `[room+0x18]`, set at
  `141c14612`/`141e93250`). Vtable 1 is only `0x1D8` bytes long (the next `lea` is
  `+0x1D8` further on), so **every slot from `+0x1D8` upward in that table is really
  vtable 2**, whose `this` is `room+8`. Reading `[room+0x1764]` off a vtable-2 method is
  reading `room+0x176c`.
* `+0x168` (`141C3EF70`), `+0x198` (`141C3F300`), `+0x1A0` (`141C3F7F0`) and `+0x1C8`
  (`141C423D0`) are **identical in all three room types** - they are base-class
  implementations, not "the trade class's". Only `+0x020`, `+0x030`, `+0x048`, `+0x068`,
  `+0x098`, `+0x138`, `+0x170`, `+0x178`, `+0x180`, `+0x188`, `+0x190`, `+0x1B0` differ per
  game. [L]

| slot | trade | **Omok** | **Match Cards** |
|---|---|---|---|
| `+0x178` (inbound default) | `14214A9C0` | **`141E99700`** | **`141C1AD70`** |
| `+0x180` (tail of inbound mode 3) | `14214AA80` | `141E9A2B0` | `141C1B3C0` |
| `+0x188` (tail of the room open) | `141C3F2E0` | `141E99C40` | `141C1B080` |

---

## 1. Which class is which game

`research/miniroom-rooms-2026-09-09.md` owns this question; these two facts fell out of the
work below and corroborate it independently.

* String id `0x05CF` = `"UI/Minigame.img/MatchCards/backgrnd"` has **exactly one** load site
  in the whole image, `141c1473b`, inside **`FUN_141C145F0`** - the class `mode 4`/`B == 4`
  constructs. So **roomType 4 = Match Cards**. [L]
* The `141e9xxxx` class (`FUN_141E93230`, `B == 3`) is the only one of the two whose code
  window loads `0x0206 "You have double-3's."`, `0x0207 "You can't put it there."`,
  `0x05DB "Mushroom"`, `0x05DC "Slime"`, `0x0606 "white/0"`, `0x0607 "black/0"` and
  `0x01FD "Request to withdraw your last move?"`. So **roomType 3 = Omok**. [L]

Below, "Omok" means the `141e9xxxx` family and "Match Cards" the `141c1xxxx` family.

---

## 2. The mode dispatch, enumerated in both directions

### 2.1 Inbound

`FUN_141C3D3E0` reads `u32 mode` and switches on `mode - 3`, 11 entries. Modes 7, 9, 0xA and
**everything outside `3..0xD`** fall to `141c3d791`, which is a virtual call
`[vt+0x178](room, mode /*edx*/, packet /*r8*/)`. [L] So every game mode arrives at the room
object's own handler with the mode still in a register.

Each of those handlers is itself a jump table:

| | function | `add edx, -0x11 / cmp edx, N / ja default` | table | modes covered |
|---|---|---|---|---|
| Omok | `FUN_141E99700` | `N = 0xF` | `0x141E99BF4`, 16 entries | `0x11 .. 0x20` |
| Match Cards | `FUN_141C1AD70` | `N = 0x12` | `0x141C1B028`, 19 entries | `0x11 .. 0x23` |

Both tables decoded in full (`base = 0x140000000`, entries are `add rcx, base` deltas): [L]

| mode | Omok target | Match Cards target |
|---|---|---|
| `0x11` | `141e99860` | `141c1adce` |
| `0x12` | `141e998fa` | `141c1ae6f` |
| `0x13` | default | default |
| `0x14` | default | default |
| `0x15` | `141e99925` | default |
| `0x16` | `141e999bf` -> `FUN_141E9B690` | default |
| `0x17` | default | default |
| `0x18` | default | default |
| `0x19` | `141e999df` | `141c1aeaa` |
| `0x1A` | `141e99a4e` | `141c1af1b` |
| `0x1B` | default | default |
| `0x1C` | `141e99a98` | `141c1af67` -> `FUN_141C1CA70` |
| `0x1D` | `141e999cf` -> `FUN_141E9BB10` | `141c1ae9a` -> `FUN_141C1C7E0` |
| `0x1E` | `141e99b93` | `141c1af77` |
| `0x1F` | **`141e9975c`** | default |
| `0x20` | `141e99b60` | default |
| `0x21`, `0x22` | *(past table end -> default)* | default |
| `0x23` | *(past table end -> default)* | **`141c1adbe`** -> `FUN_141C1C4B0` |

**A mode that lands on `default` is a silent no-op** - it reads nothing and returns. So
sending Omok's `0x1F` to a Match Cards room, or vice versa, does nothing and raises nothing. [L]

Both handlers are reached only after `FUN_141C3D3E0` passes an RTTI check on the single
global "current miniroom dialog" pointer, so an inbound game mode with no room open is also a
silent no-op. [L]

### 2.2 Outbound

Enumerated by finding every `mov edx, 0x17E` (byte scan, each hit re-decoded) that is followed
within 8 instructions by a call to the ctor or `Init` with `edx` unclobbered, then walking each
builder forward tracking `mov [rsp+D], imm` / `lea rdx,[rsp+D]` / `mov r8d, imm` to the
`SendPacket` at `0x1415D01C0`. 52 sites; 26 of them belong to the two games. [L]

Every mode is written as a **4-byte `w_raw`**, never `w_u32` - same bytes, but a scan for
`w_u32` after the ctor finds nothing (`trade-2026-09-09.md` says the same).

---

## 3. Omok

Board is **15 x 15**. The click hit-test in `FUN_141E95590` walks `r8d = 0x1a, +0x18, < 0x182`
(15 columns, each 24 px, 14 px hit width) against the mouse x, and `eax = 0x2f, +0x18, < 0x197`
(15 rows) against the mouse y, and the two loop counters become the two fields sent. [L]

### 3.1 Placing a stone - mode `0x1F`, 13 bytes, identical both ways

**Outbound**, `FUN_141E95590` at `141e95675` (mouse click, `edx == 0x202` = left-button-down),
and again from `FUN_141E9BF40` at `141e9bf6d` (same packet, coordinates passed in as a qword): [L]

```
141e95647  [rsp+0x30] = r10d       ; x index, 0..14   (outer loop, bounds the mouse x)
141e9564c  [rsp+0x34] = edx        ; y index, 0..14   (inner loop, bounds the mouse y)
141e95675  ctor 0x17E
141e9567b  w_raw 4  <- 0x1F
141e95698  w_raw 8  <- [rsp+0x38]  ; the two dwords above
141e956ad  w_u8     <- byte [rbp+0x1aec]   ; rbp = room+8, so room+0x1af4 = MY stone type
141e956c3  SEND
```

The client's only pre-send gates are `room+0x1b00 != 0` (game running) and `room+0x1afc != 0`
(my turn). **It does not check occupancy and it does not check the double-three rule.** [L]

**Inbound**, `FUN_141E99700` case `0x1F` at `141e9975c`: [L]

```
141e9976c  raw 8 -> room+0x1b30 (x), room+0x1b34 (y)
141e99774  u8    -> stone type
141e99790  call 141ea1eb0(room, x, y, type)          ; draw the stone
141e99795  if (type == room+0x1af4) ++room+0x1b38    ; my stone count
           room+0x1afc = (type != room+0x1af4)       ; -> it is now my turn
           room+0x1b18 = 0x7530                      ; 30 000 ms turn clock
141e997d7  type == 1 -> "Mushroom" (0x5DB) ; else -> "Slime" (0x5DC)
```

| # | width | field |
|---|---|---|
| 1 | `u32` | mode = `0x1F` |
| 2 | `u32` | x, 0..14 |
| 3 | `u32` | y, 0..14 |
| 4 | `u8` | stone type: **1 = Mushroom (moves first), 2 = Slime** |

**Body = 13 bytes**, both directions. [L]

### 3.2 Move rejected - inbound mode `0x20`, 8 bytes

`141e99b60`: `raw 4` -> a code; `== 0x22` shows `0x0206 "You have double-3's."`, anything else
shows `0x0207 "You can't put it there."`. [L]

| # | width | field |
|---|---|---|
| 1 | `u32` | mode = `0x20` |
| 2 | `u32` | reason; `0x22` = double-three, any other value = illegal square |

**Body = 8 bytes.** [L] Outbound `0x20` does not exist. **So the server owns every Omok rule.** [D]

### 3.3 Undo - outbound `0x15` / `0x16`, inbound `0x15` / `0x16`

* **Outbound `0x15`** (`141e958c0`, `FUN_141E95720`): request to withdraw the last move.
  Guarded by `0x1FD "Request to withdraw your last move?"` and `0x1FB "You can only request a
  handicap once per game."`. Body = `u32 0x15` = **4 bytes**. [L]
* **Inbound `0x15`** (`141e99925`): the opponent asked. Raises `0x1FC "Your oppentent has
  requested to withdraw their/their last move."` and answers with **outbound `0x16`**,
  `u32 0x16`, `u8 (dialogResult == 6)` = **5 bytes** (`141e9992e`..`141e999ab`). [L]
* **Inbound `0x16`** (`FUN_141E9B690`): [L]

```
141e9b6bd  u8 accepted
   accepted == 0 -> 0x1FE "Your opponent denied your request."   ; body ends here
   accepted != 0:
141e9b6d0  u8 count            ; number of stones to remove
141e9b6db  u8 turnSlot
           repeat `count` times: pop room+0x618, erase the stone, and
                                 if it was mine, --room+0x1b38
141e9ba51  room+0x1afc = (turnSlot == room+0x2f8)   ; my slot
           room+0x1b18 = 0x7530
```

| variant | fields | bytes |
|---|---|---|
| denied | `u32 0x16`, `u8 0` | **5** |
| accepted | `u32 0x16`, `u8 1`, `u8 count`, `u8 turnSlot` | **7** |

Match Cards has no undo: its table sends `0x15` and `0x16` to `default`, and its button
handler `FUN_141C16EB0` never builds either. [L]

### 3.4 Game start - inbound mode `0x1C`, 5 bytes

`141e99a98`: [L]

```
141e99aaf  u8 -> ebx
141e99aba  eax = room+0x2f8                       ; my slot
           room+0x1afc = (ebx != eax)             ; my turn
           room+0x1af4 = (ebx == eax) ? 2 : 1     ; MY stone type
           room+0x1b18 = 0x7530 ; room+0x1b38 = 0 ; room+0x1b08 = 0
           room+0x1b00 = 1                        ; game running
```

| # | width | field |
|---|---|---|
| 1 | `u32` | mode = `0x1C` |
| 2 | `u8` | **the slot of the player who does NOT move first** (see below) |

**Body = 5 bytes.** [L]

The naming is [D], and it is worth being exact because it is the one field a server will get
backwards: the client sets *my* stone type to **2** and *my turn* to **false** when the byte
equals my own slot, and type **1** / turn **true** otherwise. Since type 1 is drawn as
Mushroom and moves first, **the byte names the second player**. [D] Match Cards' `0x1C` uses
the same polarity (section 4.3), so this is a property of the protocol, not of Omok.

### 3.5 Turn change - inbound mode `0x1E`, 5 bytes

`141e99b93`: `u8 turnSlot`; `room+0x1afc = (room+0x2f8 == turnSlot)`; clock reset to `0x7530`. [L]

| # | width | field |
|---|---|---|
| 1 | `u32` | mode = `0x1E` |
| 2 | `u8` | **the slot whose turn it now is** |

**Body = 5 bytes.** [L] **Note the polarity is the opposite of `0x1C`'s byte.** A server that
uses one convention for both hands the turn to the wrong player, silently. [D]

### 3.6 Result - inbound mode `0x1D`

`FUN_141E9BB10`. `tools/reads.py 0x141e9bb10 6` finds exactly two `u8` reads, the second
gated: [L]

```
141e9bb23  u8 -> room+0x1b24        ; kind
   kind == 1 -> "It's a tie." (0x1F4) + 'Draw' (0x5D5)   ; body ends
   kind != 1:
141e9bb81  u8 -> room+0x1b20        ; winner slot
           room+0x1af8 = (winner == room+0x2f8) ? room+0x1af4 : 3 - room+0x1af4
           winner == my slot -> "You win." (0x1F3) + 'Win' (0x5D6), room+0x1af4 = 2
           else               -> "You lost." (0x1F5) + 'Loose' (0x5D7)
```

| variant | fields | bytes |
|---|---|---|
| draw | `u32 0x1D`, `u8 1` | **5** |
| win/lose | `u32 0x1D`, `u8 kind != 1`, `u8 winnerSlot` | **6** |

I have **not** established what a `kind` other than 0 and 1 means; nothing in this handler
branches on it beyond `== 1`. [L]

---

## 4. Match Cards

Board is **4x3 (12 cards), 5x4 (20) or 6x5 (30)** - strings `0x0626`/`0x0627`/`0x0628`, and
`FUN_141C1B080` maps a byte `0`/`1`/`2` to card counts `0xC`/`0x14`/`0x1E` and to grid widths
`4`/`5`/`6` at `room+0x184c`. [L]

### 4.1 The deal - inbound mode `0x1C`. THE SERVER SHUFFLES

`FUN_141C1CA70`, in order: [L]

```
141c1ca93  u8 -> ebp                       ; turn seed, section 4.3
141c1ca9e  u8 -> edi ; room+0x17b4 = edi   ; CARD COUNT
           free the old room+0x610
           if (edi != 0) allocate edi*4 + 8; room+0x610 = ptr + 8; [ptr] = edi
141c1cb04  edi = room+0x17b4 << 2          ; byte length = count * 4
141c1cb4b  rdx = room+0x610
141c1cb58  raw(edi bytes) -> room+0x610    ; <<< THE LAYOUT
141c1cb60  room+0x1764 = (ebp != room+0x2f8)
141c1cc4a  room+0x176c = 1                 ; game running
```

| # | width | field |
|---|---|---|
| 1 | `u32` | mode = `0x1C` |
| 2 | `u8` | turn seed - the slot that does **not** move first (section 4.3) |
| 3 | `u8` | card count: **12, 20 or 30** |
| 4 | `u32 * count` | the face value of every card, in board order |

**Body = 6 + 4 * count bytes** = **54** (12 cards), **86** (20) or **126** (30). [L]

**The client neither generates nor shuffles the layout.** `room+0x610` has exactly two write
paths in the whole `141c14000..141c30000` window: this `raw` read, and
`[room+0x610][i] = 0xFFFFFFFF` when a pair is removed in `FUN_141C1C4B0`. The allocation at
`141c1caee` and the `raw` at `141c1cb58` are straight-line with no branch between them, so
the array is never observed uninitialised. [L] **The server must shuffle.** [D]

The values are read back at `141c2644a` (`mov r8d, [rax + rsi*4]`, rax = `room+0x610`,
rsi = card index) and used to pick the card face image when a card is opened, so they are
face values and not indices into something else. [L]
Consequence worth stating plainly: **every card's face value is in the client's memory from
the moment of the deal.** [D]

### 4.2 Turning a card - mode `0x23`

**Outbound**, `FUN_141C1D680(room, edx = cardIdx, r8d = firstFlag)`, called twice from the
board click handler `FUN_141C16CD0`: [L]

```
141c16dab  FUN_141c1d680(room, edi, 1)   when room+0x17b8 == 0   ; first card of the pair
141c16dde  FUN_141c1d680(room, edi, 0)   when room+0x17b8 == 1   ; second card
```
```
141c1d6b4  w_raw 4 <- 0x23
141c1d6d1  w_u8    <- r8d      ; firstFlag
141c1d6de  w_u8    <- edx      ; card index
```

| # | width | field |
|---|---|---|
| 1 | `u32` | mode = `0x23` |
| 2 | `u8` | 1 = this is the first card of the pair, 0 = the second |
| 3 | `u8` | card index, `0 .. count-1` |

**Body = 6 bytes.** [L] Gates before it: `room+0x1764 != 0` (my turn), `room+0x176c == 1`
(game running), `room+0x17b8 < 2` (fewer than two turned), and the hit-tested index must be
`< room+0x17b4` and not already removed. [L]

**Inbound**, `FUN_141C1C4B0`, and the body length depends on field 2: [L]

```
141c1c4cb  u8 -> firstFlag
141c1c4d7  u8 -> idxB
   firstFlag != 0:  141c262d0(room, idxB, 0)      ; open it face up
                    room+0x17ac = idxB            ; remember it
                    return                         ; NOTHING ELSE IS READ
   firstFlag == 0:
141c1c4e9  u8 -> idxA                              ; the FIRST card of the pair
141c1c51a  u8 -> r                                 ; the result, see below
           if (r <  room+0x2fc)  -> NO MATCH
           if (r >= room+0x2fc)  -> MATCH
```

MATCH path: open `idxB`; `141c2a140(room, idxA)`; `room+0x610[idxA] = room+0x610[idxB] = -1`;
`++room[0x17c8 + (r - room+0x2fc)*4]`; play `UI/Minigame.img/MatchCards/effect%d`; tally;
`room+0x17ac = room+0x17b0 = -1`; show `'Match'` (`0x5D3`); clock `0x2710`. **`room+0x1764`
(my turn) is left alone - the same player keeps the turn.** [L]

NO MATCH path: `141c262d0(room, idxB, 1)`; `room+0x17b0 = idxB`;
`room+0x1764 = (r != room+0x2f8)`; show `'NoMatch'` (`0x5D4`); clock `0x2d50`. **The two cards
are not flipped back here** - they flip on the following inbound `0x1E`. [L]

| variant | fields | bytes |
|---|---|---|
| first card | `u32 0x23`, `u8 1`, `u8 idx` | **6** |
| second card | `u32 0x23`, `u8 0`, `u8 idxB`, `u8 idxA`, `u8 result` | **8** |

**What `result` means.** `room+0x2fc` is the **first `u8` of the room-open payload**, i.e. a
value the server itself chose, and it is used elsewhere as the loop bound over the room's
member seats (`FUN_141C3CEF0` at `141c3cf13`, `FUN_141C3D020` at `141c3d043`). The score array
it indexes has exactly two entries, `room+0x17c8` and `room+0x17cc`, read as a pair by
`FUN_141C20CF0` ('Tallying'). [L] So with the conventional seat count of **2**: [D]

| `result` | meaning |
|---|---|
| `0` | no match; slot 0 moved, turn passes to slot 1 |
| `1` | no match; slot 1 moved, turn passes to slot 0 |
| `2` | match scored by slot 0, slot 0 keeps the turn |
| `3` | match scored by slot 1, slot 1 keeps the turn |

The comparison is *signed* against `room+0x2fc`, so a server that sends a different seat count
in the room open shifts this whole encoding. [L]

### 4.3 Turn change - inbound mode `0x1E`, 5 bytes

`141c1af77`: `u8 turnSlot`; `room+0x1764 = (room+0x2f8 == turnSlot)`; then, if
`room+0x17ac >= 0`, flip `room+0x17ac` and `room+0x17b0` back face-down and set
`room+0x17b8 = 0`. [L]

| # | width | field |
|---|---|---|
| 1 | `u32` | mode = `0x1E` |
| 2 | `u8` | the slot whose turn it now is |

**Body = 5 bytes.** [L] Same opposite-polarity warning as Omok: `0x1C`'s byte is
`(byte != mySlot) -> my turn`, `0x1E`'s is `(byte == mySlot) -> my turn`. [L]

### 4.4 Result - inbound mode `0x1D`

`FUN_141C1C7E0` is structurally identical to Omok's `FUN_141E9BB10` - `u8 kind`, `== 1` is a
draw and ends the body, otherwise a second `u8 winnerSlot` compared against `room+0x2f8`.
`tools/reads.py 0x141c1c7e0 6` finds exactly those two reads. Fields land at `room+0x179c`
and `room+0x1798`. [L] Byte counts as in section 3.6: **5** (draw) or **6**.

---

## 5. What both games share

### 5.1 The bodiless control modes

Confirmed by walking each builder to its `SendPacket` and by the inbound handlers reading
nothing. The dialog string pins the meaning in every case marked [L]. [L]

| mode | out | in | body | meaning |
|---|---|---|---|---|
| `0x0C` | yes | via `[vt+0x168]` | out **4** | leave / close the room |
| `0x11` | yes | yes | **4** | request a tie (`0x1F9 "Will you request a tie?"`) |
| `0x12` | yes | yes | out **5**, in **4** | out: answer, `u8 accepted`. in: tie refused (`0x1FA`) |
| `0x13` | yes | no | **4** | forfeit (`0x1F6 "Are you sure you want to give up?"`) |
| `0x15` | Omok | Omok | **4** | request undo (`0x1FD`) |
| `0x16` | Omok | Omok | out **5**, in 5/7 | out: answer, `u8 accepted`. in: section 3.3 |
| `0x17` | yes | no | **4** | set "leave after this game" (`0x1FF`) |
| `0x18` | yes | no | **4** | clear it (`0x0200`) |
| `0x19` | yes | yes | **4** | ready ON (sets `room+0x1774` / `room+0x1b08` = 1) |
| `0x1A` | yes | yes | **4** | ready OFF |
| `0x1B` | yes | no | **4** | expel the other player (`0x1F7 "Will you expel the user?"`) |
| `0x1C` | yes | yes | out **4** | out: start the game. in: sections 3.4 / 4.1 |
| `0x1E` | yes | yes | out **4** | **out: my turn clock hit zero.** in: turn change |

The `0x12` / `0x16` answer byte is `sete dl` on `dialogResult == 6`, i.e. **1 = Yes**
(`141e998d3`, `141c1ae45`, `141e9999b`). [L]

Outbound `0x1E` is built in `FUN_141C1BF70` / `FUN_141E9AE60` (the per-frame tick that owns
the `'Timer'` string `0x5D8`) only when the countdown at `room+0x1790` / `room+0x1b18` reaches
zero. Body is `u32 0x1E` and nothing else. [L] **Inbound `0x1E` is 5 bytes. Do not echo it.** [D]

Outbound `0x0A` (`u32 0x0A`, `u8 1`, **5 bytes**) is sent from the room-open tail
(`141c1b121`, `141e99d10`) but **only when `room+0x2f8 == 0`**, i.e. by the room's owner. [L]
Its meaning is not established.

### 5.2 Modes handled by the shared base class

These are the same code for trade and both games, so they are not game-specific, but a server
running a mini-game still has to produce them. `trade-2026-09-09.md` left them undecoded. [L]

| inbound mode | slot | body |
|---|---|---|
| `0x0C` | `[vt+0x168]` = `141C3EF70` | `u32 mode`, `u8 slot`, `u32 reason` = **9 bytes** |
| `0x0D` | `[vt+0x1A0]` = `141C3F7F0` | `u32 mode`, `u32` -> `room+0x308` = **8 bytes** |
| `3` (user entered) | tail `[vt+0x180]` | `141E9A2B0` / `141C1B3C0` read **nothing** extra (`reads.py` depth 6, both empty; positive control passes) |

### 5.3 The game-specific tail of the room-open packet

`FUN_141C3ED00` (mode 4, `A == 0`) ends with a tail `jmp [vt+0x188]`. That is `FUN_141E99C40`
for Omok and `FUN_141C1B080` for Match Cards, and **both read the same shape**: [L]

```
repeat:
   u8 slot                       ; negative terminates
   raw 20                        ; via the 1402D1B50 thunk - the read reads.py misses
str  roomTitle                   ; -> room+0x1780
u8   gameSpec
```

* Match Cards `gameSpec`: `0` -> 12 cards / 4 wide, `1` -> 20 / 5, `2` -> 30 / 6
  (`141c1b283`, `141c1b295`..`141c1b321`). **This, not the deal, is where the board size
  arrives.** [L]
* Omok `gameSpec`: stored to `room+0x1b28` and **never read** - a scan for `[reg+0x1b28]`
  over `0x141c00000..0x141f00000` finds that one write and no read. The byte must still be
  sent, but its value appears not to matter. [L] (Blind spot: a read through a pointer, or outside that range, would
  not be seen. The room object is only manipulated inside these classes, so I think the range
  is right, but the negative is [L] only for that range.)

The 20-byte per-slot record is five `u32`. Four of them - offsets `+0x04`, `+0x08`, `+0x0C`,
`+0x10` - are formatted as decimal numbers into the mini-game record panel at
`141c182a9`..`141c18346`; `+0x00` is not drawn there. [L] Given the neighbouring strings
`'win'`/`'draw'`/`'lose'` (`0x5E7`-`0x5E9`) they are the player's win/tie/loss/points record
[I], but **I have not established which field is which** and I am not guessing one.

The rest of the room-open packet belongs to `research/miniroom-rooms-2026-09-09.md`.

### 5.4 Creating a mini-game room

`FUN_142D4ABC0` is the create dialog's OK handler (`142d4b37b`): [L]

```
w_raw 4 <- r14d (= 0)          ; mode 0, the same create as trade
w_raw 4 <- [rbp-0x68]          ; roomType, from FUN_140417E60(menuIndex); the 3 and 4 branches
w_str   <- title
w_u8    <- hasPassword
   if hasPassword != 0: w_str <- password
w_u8    <- gameSpec            ; the board-size byte of section 5.3
```

So the mini-game create is **longer than trade's 9-byte create** (`u32 0, u32 1, u8 0`) - the
server has to switch on `roomType` to know whether a title and a password follow. [D]

---

## 6. Which side decides what

* **Match Cards layout: the server.** [D] Measured in section 4.1 - the array is allocated and
  filled in one `raw` read with no other write path, and read back to choose card images.
  **The server needs a shuffle.**
* **Match Cards match/no-match: the server.** [D] The client is told the answer in the `0x23`
  result byte and never compares two values itself, even though it holds the whole layout.
* **Match Cards board size: the server**, echoed back to both clients in the room-open tail
  after the creator proposed it in the mode-0 create. [L]
* **Omok legality (occupancy, double-three): the server.** [D] The client's only pre-send
  checks are "game running" and "my turn"; illegality comes back as mode `0x20`.
* **Turn order in both games: the server** (`0x1C`, `0x1E`, and the `0x23` result byte /
  `0x16` accepted byte). The client never advances its own turn flag except from a packet. [L]
* **Win/lose/draw in both games: the server** (`0x1D`). The Omok client draws the board but
  never evaluates five-in-a-row - `FUN_141EA1EB0` is 5 763 bytes of drawing and stack
  bookkeeping and builds no packet. [L]
* **Turn clock: shared.** The client counts down locally (`0x7530` Omok, `0x2710`/`0x2d50`
  Match Cards) and, on reaching zero, *reports* it with outbound `0x1E`; the server then
  decides and sends inbound `0x1E`. [L]
* **The score array: the client accumulates it** (`++room[0x17c8 + i*4]` on a Match Cards
  match) from the server's result bytes. The persistent record (section 5.3) comes from the
  server. [L]

---

## What the server must send and accept

Byte counts are **bodies**, excluding the 2-byte opcode - the same convention as
`trade-2026-09-09.md`. Every field is `[L]` unless marked. Nothing here is a guessed body;
where a body is unknown it is in section 8 and is **not** listed.

### Accept, opcode `0x017E` (client -> server)

| mode | game | body | bytes |
|---|---|---|---|
| `0x00` | both | `u32 0`, `u32 roomType (3 or 4)`, `str title`, `u8 hasPassword`, [`str password`], `u8 gameSpec` | var |
| `0x0A` | both | `u32 0x0A`, `u8 1` (owner only) | 5 |
| `0x0C` | both | `u32 0x0C` | 4 |
| `0x11` | both | `u32 0x11` | 4 |
| `0x12` | both | `u32 0x12`, `u8 accepted (1 = yes)` | 5 |
| `0x13` | both | `u32 0x13` | 4 |
| `0x15` | Omok | `u32 0x15` | 4 |
| `0x16` | Omok | `u32 0x16`, `u8 accepted` | 5 |
| `0x17` | both | `u32 0x17` | 4 |
| `0x18` | both | `u32 0x18` | 4 |
| `0x19` | both | `u32 0x19` | 4 |
| `0x1A` | both | `u32 0x1A` | 4 |
| `0x1B` | both | `u32 0x1B` | 4 |
| `0x1C` | both | `u32 0x1C` | 4 |
| `0x1E` | both | `u32 0x1E` | 4 |
| `0x1F` | Omok | `u32 0x1F`, `u32 x (0..14)`, `u32 y (0..14)`, `u8 stoneType` | **13** |
| `0x23` | Match Cards | `u32 0x23`, `u8 isFirstCard`, `u8 cardIndex` | **6** |

### Send, opcode `0x0575` (server -> client)

| mode | game | body | bytes |
|---|---|---|---|
| `0x0C` | both | `u32 0x0C`, `u8 slot`, `u32 reason` | 9 |
| `0x0D` | both | `u32 0x0D`, `u32` | 8 |
| `0x11` | both | `u32 0x11` | 4 |
| `0x12` | both | `u32 0x12` | 4 |
| `0x15` | Omok | `u32 0x15` | 4 |
| `0x16` | Omok | `u32 0x16`, `u8 0` | 5 |
| `0x16` | Omok | `u32 0x16`, `u8 1`, `u8 stonesToRemove`, `u8 turnSlot` | 7 |
| `0x19` | both | `u32 0x19` | 4 |
| `0x1A` | both | `u32 0x1A` | 4 |
| `0x1C` | Omok | `u32 0x1C`, `u8 secondPlayerSlot` **[D]** | **5** |
| `0x1C` | Match Cards | `u32 0x1C`, `u8 secondPlayerSlot` **[D]**, `u8 count (12/20/30)`, `u32 face[count]` | **54 / 86 / 126** |
| `0x1D` | both | `u32 0x1D`, `u8 1` (draw) | 5 |
| `0x1D` | both | `u32 0x1D`, `u8 kind != 1`, `u8 winnerSlot` | 6 |
| `0x1E` | both | `u32 0x1E`, `u8 turnSlot` | **5** |
| `0x1F` | Omok | `u32 0x1F`, `u32 x`, `u32 y`, `u8 stoneType (1 or 2)` | **13** |
| `0x20` | Omok | `u32 0x20`, `u32 reason (0x22 = double-three)` | **8** |
| `0x23` | Match Cards | `u32 0x23`, `u8 1`, `u8 cardIndex` | **6** |
| `0x23` | Match Cards | `u32 0x23`, `u8 0`, `u8 secondIdx`, `u8 firstIdx`, `u8 result` **[D]** | **8** |

Plus the room open (mode 4, `A == 0`), whose base part is
`research/miniroom-rooms-2026-09-09.md`'s, ending with the game tail measured here:
`{ u8 slot (negative terminates), raw 20 } *`, `str roomTitle`, `u8 gameSpec`. [L]

Three traps that will not announce themselves:

1. **`0x1E` is 4 bytes outbound and 5 inbound.** Echoing what the client sent leaves the turn
   where it was and the client will never move again.
2. **`0x1C`'s slot byte and `0x1E`'s slot byte have opposite meanings.** `0x1C` names the
   player who does **not** move first; `0x1E` names the player who does.
3. **A mode a game does not handle is a silent no-op**, and so is any game mode arriving with
   no room dialog open. Nothing is drawn and nothing is logged. But `CLAUDE.md`'s standing
   rule still applies: `0x017E` currently goes unanswered in every archived run, and an
   unanswered packet freezes the whole UI.

---

## 7. What I could not determine, and the cheapest thing that would settle it

1. **The five `u32` in the 20-byte per-slot record** (section 5.3). Four are drawn as numbers
   at `141c182a9`/`141c182da`/`141c1830b`/`141c1833c`, in the panel order `+0x10`, `+0x04`,
   `+0x0C`, `+0x08`. *Cheapest:* render the `UI/UIWindow.img/MinigameTable` canvases with
   `tools/wz_png.py` - the labels beside those four fields are baked bitmaps
   (`maplecw-baked-ui-text`), so a render names them in one pass and no client run.
2. **Omok's room-open `gameSpec` byte** is written to `room+0x1b28` and never read in
   `0x141c00000..0x141f00000`. *Cheapest:* `python tools/fieldrefs.py 0x1b28` unscoped
   (minutes, no client run) to close the range blind spot; if it is genuinely dead, any value
   will do.
3. **`kind` values other than 0 and 1 in the `0x1D` result.** Only `== 1` is branched on.
   Nothing static can distinguish "forfeit" from "normal win" here because the client does not
   look. *Cheapest:* a client run with two characters, forfeiting one game - but only after
   everything else works.
4. **The exact value of `room+0x2fc`** that makes the Match Cards `result` encoding above
   correct. It is the server's own first room-open byte, so this is a self-consistency
   requirement rather than an unknown - but the mapping in section 4.2 is [D], not [L].
   *Cheapest:* it falls out of `research/miniroom-rooms-2026-09-09.md`'s room-open decode.
5. **Outbound `0x0A`'s meaning.** Sent by the room owner from the room-open tail with a
   literal `u8 1`. Nothing in this client says what a `0` would mean.
6. **`FUN_141C26230`, the Match Cards hit-test**, bounds the index by `room+0x17b4` but I did
   not decode its grid arithmetic, so I cannot state that index 0 is the top-left. *Cheapest:*
   `python tools/listing.py 0x141c26230` - one read, no client run. I did not do it because
   the server only has to keep the index space consistent with its own deal.
7. **No capture exists for any of this.** The whole file is static. The cheapest confirmation
   is not a client run: answer `0x017E` mode `0x00` with roomType 3, then send mode 4 / `0x1C`,
   and read `world.log` plus `client-patched\maplecw-hook.log` together - a dispatch line
   written on *return* for `0x0575` proves the handler was entered and came back
   (`CLAUDE.md`, "Count the same event in two logs").
