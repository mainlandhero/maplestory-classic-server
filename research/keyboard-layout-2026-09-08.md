# The keyboard layout: `0x0199` saves it, and the restore block is 89 slots behind an inverted gate

The owner, 2026-09-08, on the live client: *"I have put Slash Blast on the Ctrl key, Power Strike on
the Shift key, and Iron Body on the A key. I have saved it, it's now performing this function
locally, but I want this to persist."*

Tags: **[L]** read off this client's listing or a capture, **[D]** derived, **[I]** inferred.
No client run was spent - the capture came from the session that was already up.

---

## 1. The save packet: `0x0199`, subtype 0

One inbound `0x0199` in the whole channel log, arriving at the moment CONFIRM was clicked [L]:

```
01:52:44.602 <- 0x0199 UNKNOWN, 29 byte body
0000ffffffffffffffff03 1d01 2a460f00 1e01 28460f00 2a01 29460f00
```

```
u8   subtype   = 0
u8   flag      = 0
u32  a         = 0xFFFFFFFF
u32  b         = 0xFFFFFFFF
u8   count     = 3
count x { u8 key; u8 type; u32 action }
```

29 bytes exactly: `1 + 1 + 4 + 4 + 1 + 3*6`.

### It decodes to precisely what the owner said, which is the control

`key` is a **DirectInput scan code**, not a virtual key:

| key | scan code | action | skill |
|---|---|---|---|
| Ctrl | `0x1D` DIK_LCONTROL | `0x000F462A` = 1001002 | **Slash Blast** |
| Shift | `0x2A` DIK_LSHIFT | `0x000F4629` = 1001001 | **Power Strike** |
| A | `0x1E` DIK_A | `0x000F4628` = 1001000 | **Iron Body** |

and `gm-handbook/skills.txt` gives `1001000 Iron Body`, `1001001 Power Strike`,
`1001002 Slash Blast` [L]. Three keys and three skills, all six matching a statement made
before the packet was opened. `type = 1` is *skill* for all three; other types are unknown
because nothing else has been bound yet.

## 2. The builder, and the thing that will bite: it sends a DELTA

`research/msexe-send-opcodes.txt` lists four `0x0199` builders. Three encode a non-zero
subtype (`mov dl,1` at `0x141a011f7`, `mov dl,2` at `0x141a01287`, `mov dl,3` at
`0x141a01886`). **`FUN_141a0c340` is the one that made this packet** - `xor edx,edx` at
`0x141a0c385` gives subtype 0 [L]:

```
141a0c375  mov  edx, 0x199          ; COutPacket(0x199)
141a0c385  xor  edx, edx / Encode1  ; subtype 0
141a0c391  movzx edx, dil / Encode1 ; the flag
141a0c39f  mov  edx, esi / Encode4  ; a
141a0c3ab  mov  edx, ebp / Encode4  ; b
141a0c3bf  call 0x1406ed640         ; RESERVE one byte for the count, remember the position
141a0c3ce  Encode1(0)               ; the placeholder
           esi = 0 ; rdi = keymapMan
loop:
141a0c3e0  lea  rdx, [rdi + 0x1bd]
141a0c3ea  call 0x1401dea00         ; does this slot differ from the DEFAULT copy?
141a0c3f1  jne  next                ; same -> skip it
141a0c3fc  Encode1(sil)             ; the key index
141a0c409  call 0x1401de8e0         ; {u8 type, u32 action}
           bpl++
next:
141a0c413  add  rdi, 5              ; STRIDE 5
141a0c417  cmp  esi, 0x59           ; 89 SLOTS
141a0c428  call 0x1406ed8e0         ; backfill the count at the reserved position
```

**The packet contains only the keys that differ from the client's default table.** A server that
treats it as a full layout and replaces what it stored will lose every binding the player did not
touch in that one dialog. Merge the deltas into the stored layout; do not replace [D].

The keymap manager holds **two parallel tables**: the live one at `+0`, its defaults at
`+0x1bd` - and `0x1bd = 445 = 89 * 5`, which is the same table twice [D].

A slot is five bytes, `{ u8 type; u32 action }`. `FUN_1401de960` is the clear-to-zero
(`mov byte [rcx],0 ; mov dword [rcx+1],0 ; ret`) [L].

## 3. The restore path, and its gate is INVERTED

`FUN_1401de920` is the exact inverse of the encoder - `Decode1 -> [slot+0]`,
`Decode4 -> [slot+1]` [L]. It has **one** call site, and `tools/callers.py` only finds the
function holding it through a **tail jmp**, which is the failure mode `CLAUDE.md` records
costing a session once already:

```
1419ffcf4  call 0x1406e8ae0     ; Decode1 -> bool
1419ffcf9  test al, al
1419ffcfb  jne  0x1419ffd21     ; NON-ZERO -> SKIP the block entirely
1419ffcfd  rdi = table ; esi = 0x59
1419ffd0b  call 0x1401de920     ; one slot
1419ffd10  add  rdi, 5
1419ffd18  jne  loop            ; 89 times
```

**A zero byte means "read all 89 slots". A non-zero byte means "skip".** That is the opposite of
the natural guess and it is the single most likely way to get this wrong. The block is
`1 + 89*5 = 446` bytes, and it is **not a delta** - all 89 slots, in scan-code order.

## 4. What we do about it today: nothing, in either direction

`grep -rn "keymap|key_map|func_key|quickslot" --include=*.rs crates/` returns **nothing** [L].
`0x0199` is not in `crates/net/src/opcode.rs` and the world session logs it as
`UNKNOWN, is not answered yet`.

## 5. The one thing still unknown, and the question that settles it

**Which server-to-client opcode carries the 446-byte block.** The chain up from the decoder is
`0x1401de920 <- 0x1419ffc00 <- (tail jmp) 0x1419ffff0 <- 0x141820080`, and `0x141820080` has
**nine** callers, so it is a shared "restore local state" routine rather than one opcode's
handler [L]. Naming the opcode is a further static pass.

**A false lead worth recording, because it looked convincing.** `set_field_with_character_dressed_quests`
ends with `b.extend_from_slice(&[0u8; 1 + 384])`, whose own comment says the zero u8 "jumps past
the next seven reads" at `0x142098435`. That reads as *"we are sending the byte that skips the
keymap"*. It is not: disassembling `0x142098435` shows the block behind that gate decodes a bool,
a u32, and two 8-byte FILETIMEs - not an 89-slot loop [L]. Different gate, different block.

**And there is a free discriminator that only the owner can read, on a relog they are going to do
anyway.** After logging out and back in, are the three keys:

* **back at their factory bindings** -> the client never received a keymap block at all, so the
  restore packet is one we simply never send. This is the expected outcome **[I]**, because our
  `SetField` tail is 385 zero bytes and the block needs 446.
* **blank / unbound** -> the client *did* reach the loop with our zero gate byte and read 89
  zero slots out of our padding. Then the block IS inside something we already send, we are
  filling it with zeros, and the fix is much smaller than it looks.

The two readings need opposite work and look identical in every log we hold, so this is worth
asking before any code is written.

---

## 6. The restore opcodes: `0x05F1`, `0x05F2`, `0x05F3` [L]

§5 said the opcode was unknown and named the chain up from the decoder as the way to find it.
That chain was the wrong instrument - `0x141820080` has 9 call sites *and* 25 tail-jump sites,
34 entry points, and none of them is in either opcode table. Widening it would only have made
the blind spot bigger.

**Changing the question found it in one step.** `0x1419ffc00` takes its packet in `rcx` and gets
the keymap manager from a singleton, so whoever calls it is passing a packet - which makes
`0x141820080` a packet handler rather than the UI routine its caller count suggested. It is a
large dispatch function, and the call site is 0x1F36 bytes into it. Reading the branch that
selects it [L]:

```
141821fa4  lea  eax, [r9 - 0x5f1]      ; r9 is the opcode
141821fab  cmp  eax, 2
141821fae  ja   <next case>            ; so: opcode in 0x5F1 .. 0x5F3
141821fb0  mov  rdx, rbx               ; the packet
141821fb3  mov  ecx, r9d               ; the opcode
141821fb6  call 0x1419ffff0
```

and `FUN_1419ffff0(opcode, packet)` splits the three [L]:

```
1419ffff4  sub  ecx, 0x5f1 / je  -> 141a0002c   ; 0x5F1
1419fffc  sub  ecx, 1     / je  -> 141a00019    ; 0x5F2
141a00001  cmp  ecx, 1    / jne -> return       ; 0x5F3

141a0002c  mov rcx, rdx ; jmp 0x1419ffc00       ; 0x5F1 -> the 89-slot loop
141a00019  Decode4 -> [0x143AD1068]             ; 0x5F2 -> one u32
141a0000e  Decode4 -> [0x143AD106C]             ; 0x5F3 -> one u32
```

### The control: the save builders read exactly the globals these handlers write

The three `0x0199` subtypes and the three inbound opcodes are a matched set, and the match is
checkable rather than assumed. §2 recorded that subtype 1 (`FUN_141a011c0`) and subtype 2
(`FUN_141a01250`) each encode one u32 read from a global. Resolving all four RIP-relative
displacements [L]:

| | address |
|---|---|
| `0x05F2` handler **writes** | `0x143AD1068` |
| `0x0199` subtype 1 **reads** | `0x143AD1068` |
| `0x05F3` handler **writes** | `0x143AD106C` |
| `0x0199` subtype 2 **reads** | `0x143AD106C` |

Two independent code paths, written years apart from each other in the same binary, agreeing on
an address to the byte. That is what makes this a pairing and not a guess.

### The full protocol, both directions

| client -> server | server -> client | payload |
|---|---|---|
| `0x0199` subtype 0 | **`0x05F1`** | key bindings. **Save is a DELTA, restore is the FULL 89 slots.** |
| `0x0199` subtype 1 | **`0x05F2`** | one `u32` at `0x143AD1068` |
| `0x0199` subtype 2 | **`0x05F3`** | one `u32` at `0x143AD106C` |
| `0x0199` subtype 3 | none found | `u8` preset index, bounded `< 4` by `cmp ebx,4 / jae` at `0x141a01871` |

The preset selector is visible in the client's KEY BINDINGS dialog as **Preset 1 / 2 / 3**, and
subtype 3 is how a switch is reported. Nothing inbound was found for it, so either the server is
not told to restore the *selected* preset, or its handler is elsewhere. **Named as unfinished
rather than guessed at.**

### `0x05F1`'s body, which is the one to get right

```
u8   gate                  ; 0 = READ the block. NON-ZERO = skip and keep defaults.
89 x { u8 type; u32 action }
```

`1 + 89*5 = 446` bytes. The gate is inverted from the natural reading and that is the single
easiest thing to get wrong here; §3 has the listing.

## 7. Confirmed on a screen, 2026-09-08

The owner bound the three skills, clicked CONFIRM, then tested both kinds of relog [L]:

* **logout and back in without closing the client -> the layout STAYS.** The client keeps its
  keymap manager across a character re-entry, and `SetField` is sent on that path.
* **a fresh client -> the layout is back at FACTORY bindings**, not blank.

Together those two settle a question §5 had left open with two readings. If our `SetField` tail's
zero byte reached the 89-slot loop, the re-login would have read 89 zero slots out of our padding
and come back **blank**. It came back unchanged. So `0x05F1` is a packet we have **never sent**,
the client falls back to its own defaults on a fresh process, and the fix is to add a packet
rather than to correct one we are already sending.

## 8. What the server has to do

1. **Parse `0x0199`.** Subtype 0 carries only the keys that differ from the client's defaults, so
   **merge** the deltas into the stored layout; replacing it loses every untouched binding.
   Subtypes 1 and 2 each carry one `u32`; subtype 3 is a preset index.
2. **Store it per character**, alongside the other per-character state.
3. **Send `0x05F1` at login** with gate byte `0` and all 89 slots, plus `0x05F2` / `0x05F3` with
   their `u32`s.

   For a character with nothing stored, **send no packet at all** - not the non-zero keep gate,
   which was the first plan. The keep gate is a real packet and the client's no-op path behind
   it (`0x1419ffd21` onward, past the `jne` that skips the loop) **has not been read**. The
   server sends nothing today and the client is demonstrably fine on a seven-hour session, so
   silence is the measured-good behaviour and the keep form would be an unmeasured one adopted
   to save a branch. `net::keymap::restore` returns `Option` for exactly this reason, and
   `keymap_init_keep` is kept only because the inverted gate byte is the thing most likely to
   be got backwards later.

Nothing here is wired yet - `grep -rn "keymap|key_map|func_key|quickslot" --include=*.rs crates/`
still returns nothing.
