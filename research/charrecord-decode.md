# The character record: `FUN_140304b20`

The block `SetField` carries when `characterData = 1`. 18525 bytes of decoder, and the last
thing between the project and a character standing on a map.

Companion files, written in parallel and each owned by a different pass:
`charrecord-loops.md` (the loop census and the straight-line spine),
`charrecord-reuse.md` (what `crates/net` already builds), `charrecord-v214-shape.md`
(candidate names from the reference server - **candidates only**, see the control score in
`research/msexe-gamestage-opcodes.md`).

## The instrument, before anything read with it

The decompiled body reports **122** packet reads. The disassembly reports **117**. That gap
had to be resolved before either could be used, because five unaccounted reads is five
fields in the wrong place.

* The listing covers `0x140304b20..0x14030937e` with **zero discontinuities** and no bad
  instruction data - so it is complete, and Themida has not hidden anything here.
* Per decoder, the two agree exactly on `u8` (16), `u16` (34), `string` (7) and `raw` (11).
  **The entire discrepancy is `u32`: 54 decompiled against 49 in the listing.**
* There are **no tail jumps** to a decoder, so the listing is not missing call-shaped edges.

So the decompiler emitted five `FUN_1406e8c20` calls more than once - the usual artifact of
reconstructing control flow that reaches one call site from several predecessors.
**The listing is authoritative: 117 reads.** Take the field *order* from the listing and the
field *meaning* from the decompilation, which is the same split that worked for `SetField`.

## Signature

```c
FUN_140304b20(void *user, void *scratch112, CInPacket *packet, int zero, int unknown5)
```

`SetField` calls it as `FUN_140304b20(FUN_1420a3080(0), local_b8, packet, 0)`. `local_b8` is
a **112-byte stack buffer** in the caller, not a mask from the wire - there is no way to ask
the client for a smaller record from outside.

## There IS a presence mask, and it is field 1

> **CORRECTED.** This section previously said there was no mask, on the strength of the
> function containing **zero** `BT` instructions and zero `TEST reg,imm`. Both counts are
> accurate and the conclusion was wrong: **the mask is not a bitfield.** It is the 100-byte
> array read as field 1, one **byte** per flag, tested with `CMP byte ptr [reg],0`. An
> instrument that only looks for bit tests cannot see a byte array, and "zero hits" read as
> "no mask" instead of "wrong shape". The claim was committed before it was checked against
> anything that could have contradicted it.

What is actually there:

```asm
140304e46  MOV   RCX,R14          ; the 100-byte array from field 1
140304e49  CALL  0x1402fa9a0      ; -> RAX = &array[0]
140304e4e  MOV   ECX,ESI
140304e50  CMP   byte ptr [RAX],0
140304e53  JNZ   140304e64        ; a set flag -> run the block
140304e55  INC   ECX
140304e57  INC   RAX
140304e5a  CMP   ECX,0x64         ; 0x64 = 100, the array length
140304e5d  JC    140304e50
140304e5f  JMP   140304fa6        ; nothing set -> SKIP the block
140304e64  ...
140304e71  CALL  0x140302e30      ; the stat decoder, the block this gate guards
```

**43 gates** of this shape appear across the function, each bounded by `0x64`, matching the
43 calls to the accessor `FUN_1402fa9a0`. So the record is **not** a fixed sequence: the
100 bytes select what follows, and an all-zero array makes the client skip.

**Two independent lines agree on this.** The reference server's `Char.encode` opens with
**100 x `byte 1`** as its presence signal - found in a separate pass that had not seen this
disassembly. A 100-byte opener on both sides is not a coincidence.

**What is not yet settled**: the index-to-block mapping. Each gate starts its scan at
`ECX = ESI`, and `ESI` is reassigned repeatedly through the function, so the gates are
probably *not* all testing the same thing - but "each gate owns a distinct flag index" is
an inference, not something read yet. Do not build a mask on it until the loop census
settles which index guards which block.

## The head, read off the listing

```asm
140304b6f  XORPS XMM0,XMM0 / XOR EAX,EAX
140304b74  MOVUPS [RDX+0x00 .. 0x50],XMM0    ; zero 0x60 bytes of the caller's scratch
140304b8b  MOV   [RDX+0x60],EAX              ; and 4 more -> 0x64 = 100 bytes zeroed
140304b8e  LEA   R8D,[RAX + 0x64]            ; RAX is 0 here, so this is the constant 100
140304b95  CALL  0x1406e9170                 ; read 100 RAW BYTES into the scratch
140304ba7  CALL  0x1406e8ae0                 ; u8  -> [user + 0x1001]
140304bba  CALL  0x1406e8c20                 ; u32 -> if > 0, added to a tick from
140304bc5                                    ;        [0x143262db0]: a duration, not a count
```

**Field 1 is a fixed 100-byte raw block.** The decompiler renders the size as computed
(`RAX + 0x64`) and it is not - `RAX` is zeroed two instructions earlier. The zeroing of
exactly `0x64` bytes of the scratch immediately before the read is the corroboration: the
buffer and the field are the same 100 bytes.

## Every raw read, and none of them is variable

The 11 `FUN_1406e9170` calls, with the constant that reaches `R8D`:

| address | bytes |
|---|---|
| `140304b95` | **100** (the head block above) |
| `140304cc6`, `140304d1f`, `140304e2d`, `140304ff5`, `140306c1c` | 8 each |
| `140306ce2`, `140306cf7`, `140306dcf`, `140307042`, `1403075df` | 8 each |

**100 + 10x8 = 180 bytes of raw fields, all fixed.** Ten 8-byte reads in a MapleStory record
are almost certainly `FILETIME` expiry stamps, which is a guess about *meaning*; that they
are 8 bytes each and not variable is read from the listing.

## What this settles for the build

* The 100-byte array is the lever, and it is a much better one than loop counts: it gates
  whole blocks rather than shortening them.
* Nothing in the record is length-prefixed at the top level, so a wrong width anywhere
  desyncs everything after it. There is no resynchronisation point.
* `FUN_140302e30`, the stat decoder we already build for the character list, is called at
  `140304e71` - after the first 11 reads. So the head above is what precedes known ground.
