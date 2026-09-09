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
