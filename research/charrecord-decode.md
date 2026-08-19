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

## No presence mask

Checked explicitly, because if the record began with a flags word selecting optional blocks
the job would collapse. **It does not.** The function contains **zero** `BT` instructions and
zero `TEST reg,imm` against a mask. The sequence is fixed; the only lever is loop counts,
which is what `charrecord-loops.md` is for.

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

* The minimum record is **not** collapsible below 180 bytes of raw fields plus the
  straight-line spine plus the stat block.
* Nothing in the record is length-prefixed at the top level, so a wrong width anywhere
  desyncs everything after it. There is no resynchronisation point.
* `FUN_140302e30`, the stat decoder we already build for the character list, is called at
  `140304e71` - after the first 11 reads. So the head above is what precedes known ground.
