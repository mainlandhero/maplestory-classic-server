# The presence array: which byte switches on which block

**SETTLED 2026-08-19, statically, from the client's own startup code.** This was the last
open question in `research/charrecord-loops.md` ("What I could NOT determine about the
gates") and the blocker named at the top of `STATUS.md`.

**The headline: `presence[0] = 1` switches on the character-stat block** - the one that
calls `FUN_140302e30`, the stat decoder `crates/net` already builds for the character list.

## How a gate actually works

`FUN_1402fa9a0` is not an accessor. It is a **100-byte bytewise AND** [L]:

```c
undefined8 *FUN_1402fa9a0(longlong arr, undefined8 *out, longlong key) {
    // zeroes out[0..0x63] - 12 qwords plus a dword, exactly 100 bytes
    do { out[i] = arr[i] & key[i]; ... } while (uVar2 < 100);   // 2 x 50, unrolled
    return out;
}
```

and the caller scans the **result** for any non-zero byte:

```asm
140304e38  LEA   R8,[0x143abee20]   ; the key for this gate - entry 7
140304e46  MOV   RCX,R14            ; the 100-byte presence array from field 1
140304e49  CALL  0x1402fa9a0        ; RAX = out, where out[i] = presence[i] & key[i]
140304e50  CMP   byte ptr [RAX],0
140304e53  JNZ   140304e64          ; any byte set -> run the block
140304e5a  CMP   ECX,0x64           ; ... scanned over all 100
140304e5f  JMP   140304fa6          ; all clear -> SKIP the block
```

So each gate carries **its own 100-byte key mask**, and the block runs iff
`any(presence & key)`.

## And every key mask has exactly one bit set

The key table is 41 objects of stride `0x70` based at `0x143abeb10`, in the **uninitialised
tail of `.data`** - zero on disk, built at startup. 43 CRT dynamic initialisers, pointer
array at `.rdata 0x143264a00 .. 0x143264b58` [L], and 40 of them have this exact shape [L]:

```asm
140023290  XOR   EDX,EDX
140023296  LEA   RCX,[0x143abee20]        ; entry 7
14002329d  CALL  0x140302c70              ; memset(entry, 0, 100)   <- verified: 100 bytes
1400232a2  MOV   byte ptr [0x143abee20],1 ; entry+0  -> presence byte 0
```

`FUN_140302c70(p, 0)` writes 100 bytes of its second argument [L] - so the mask is
**all-zero except one byte**. Therefore:

> **gate on entry k fires  <=>  presence[byte_of(k)] != 0**

One byte per gate. Nothing composite, nothing overlapping: 40 entries, **40 distinct
bytes** [D].

**Entry 0 is never written by any initialiser** [D, and see the instrument note below] - its
mask stays all-zero, so a gate that selects entry 0 can never fire. That corroborates the
census's reading that entry 0 is the `JA` default of the dynamic gates' jump table: the
default case is "absent".

## The census's index guess was wrong

`charrecord-loops.md` recorded, honestly labelled `[I] plausible ... but I did not prove
it`, that "key #k reads array byte k". **It does not.** The mapping is a permutation:

| entry | 7 | 8 | 1 | 11 | 14 | 17 |
|---|---|---|---|---|---|---|
| presence byte | **0** | **62** | **44** | **1** | **64** | **8** |

Had that inference been built on, every flag would have been set in the wrong place and the
client would have skipped every block - which is indistinguishable, on screen, from sending
nothing. The gate labels `#k` in the census are correct as *entry* numbers; they are not
byte offsets.

## The reference server agrees, on the one thing it is allowed to vote on

`Char.encode` in the v214 tree writes **100 bytes of `1`** and then, after a handful of
small fixed fields, `if (mask.isInMask(DBChar.Character)) getAvatarData().getCharacterStat().encode(...)`.
`DBChar.Character` is `0x1` - **ordinal 0**, the first flag after `None`. The client's first
gated block is the stat decoder and its byte is **`presence[0]`**, the same ordinal.

Per `CLAUDE.md` the reference is a different game version and scored 1 of 8 against a
held-out control, so this is **[I] corroboration, not evidence** - it could not have
overturned the initialisers. But it was derived from a completely different artefact and it
lands on the same index, and the structural echo either side (100 bytes, then small fixed
fields, then a gated stat block) is hard to get by chance.

## The table

`presence byte` is the one to set. `entry` is the census's `#k`. Regions and read counts are
carried over from `charrecord-loops.md` section 5 unchanged.

| presence byte | entry | key VA | gate call | guarded region | reads | loops |
|---:|---:|---|---|---|---:|---|
| **0** | 7 | `0x143abee20` | `0x140304e49` | `[0x140304e64, 0x140304fa6)` | 7 | - |
| **1** | 11 | `0x143abefe0` | `0x140305082` | `[0x1403050a1, 0x1403050ac)` | 0 | - |
| **2** | 6 | `0x143abedb0` | `0x140305776` | `[0x140305794, 0x140305d7d)` | 0 | - |
| **2** | 6 | `0x143abedb0` | `0x1403061a0` | `[0x1403061c9, 0x14030661c)` | 3 | #7 |
| **3** | 5 | `0x143abed40` | `0x1403050bd` | `[0x140305104, 0x1403051b6)` | 5 | #5 |
| **7** | 13 | `0x143abf0c0` | `0x140305e29` | `[0x140305e45, 0x140305e54)` | 1 | in #6 |
| **8** | 17 | `0x143abf280` | `0x140306d28` | `[0x140306d44, 0x1403073c3)` | 21 | #13-#19 |
| **9** | 19 | `0x143abf360` | `0x1403074a3` | `[0x1403074c4, 0x14030756a)` | 6 | #21,#22 |
| **10** | 21 | `0x143abf440` | `0x140307644` | `[0x140307664, 0x140307801)` | 6 | #25 |
| **11** | 22 | `0x143abf4b0` | `0x140307812` | `[0x140307834, 0x140307b97)` | 6 | - |
| **12** | 23 | `0x143abf520` | `0x140307ba8` | `[0x140307bc1, 0x140307bfc)` | 2 | #26,#27 |
| **14** | 20 | `0x143abf3d0` | `0x14030757b` | `[0x140307596, 0x140307633)` | 6 | #23,#24 |
| **15** | 18 | `0x143abf2f0` | `0x1403073d4` | `[0x1403073f4, 0x140307492)` | 3 | #20 |
| **16** | 24 | `0x143abf590` | `0x140308a36` | `[0x140308a56, 0x140308ab8)` | 3 | #28 |
| **17** | 25 | `0x143abf600` | `0x140308ace` | `[0x140308ae6, 0x140308b3c)` | 3 | #29 |
| **19** | 34 | `0x143abf9f0` | `0x140308e79` | `[0x140308e92, 0x140308ee0)` | 3 | #32 |
| **21** | 15 | `0x143abf1a0` | `0x140306b62` | `[0x140306b89, 0x140306c3c)` | 3 | #11 |
| **22** | 12 | `0x143abf050` | `0x1403050e4` | `[0x140305104, 0x1403051b6)` | 5 | #5 |
| **23** | 28 | `0x143abf750` | `0x140308cf3` | `[0x140308d11, 0x140308d20)` | 0 | - |
| **27** | 16 | `0x143abf210` | `0x140306c4d` | `[0x140306c68, 0x140306d17)` | 3 | #12 |
| **29** | 32 | `0x143abf910` | `0x140308df3` | `[0x140308e11, 0x140308e2f)` | 2 | - |
| **37** | 26 | `0x143abf670` | `0x140308b63` | `[0x140308b81, 0x140308bcf)` | 3 | #30 |
| **39** | 33 | `0x143abf980` | `0x140308e40` | `[0x140308e59, 0x140308e68)` | 0 | - |
| **41** | 27 | `0x143abf6e0` | `0x140308be0` | `[0x140308c01, 0x140308c44)` | 3 | #31 |
| **44** | 1 | `0x143abeb80` | `0x14030525a` | `[0x140305275, 0x1403054df)` | 0 | - |
| **44** | 1 | `0x143abeb80` | `0x140306632` | `[0x140306654, 0x140306830)` | 3 | #8 |
| **49** | 36 | `0x143abfad0` | `0x1403090b5` | `[0x1403090d1, 0x140309114)` | 3 | #35 |
| **52** | 29 | `0x143abf7c0` | `0x140308d31` | `[0x140308d51, 0x140308d68)` | 0 | - |
| **54** | 37 | `0x143abfb40` | `0x140309125` | `[0x140309141, 0x140309156)` | 0 | - |
| **55** | 38 | `0x143abfbb0` | `0x140309167` | `[0x140309181, 0x140309196)` | 0 | - |
| **57** | 39 | `0x143abfc20` | `0x1403091a7` | `[0x1403091c1, 0x1403091d6)` | 0 | - |
| **58** | 40 | `0x143abfc90` | `0x1403091e7` | `[0x140309204, 0x1403092f3)` | 4 | #36,#37 |
| **62** | 8 | `0x143abee90` | `0x140304fb7` | `[0x140304fd1, 0x140304ffa)` | 2 | - |
| **64** | 14 | `0x143abf130` | `0x140306147` | `[0x140306171, 0x14030618f)` | 2 | - |
| **65** | 35 | `0x143abfa60` | `0x140308ef1` | `[0x140308f14, 0x140309099)` | 7 | #33,#34 |
| **66** | 9 | `0x143abef00` | `0x14030500b` | `[0x140305023, 0x140305033)` | 0 | - |
| **67** | 10 | `0x143abef70` | `0x140305044` | `[0x140305061, 0x140305071)` | 0 | - |
| **68** | 30 | `0x143abf830` | `0x140308d79` | `[0x140308d92, 0x140308da9)` | 0 | - |
| **69** | 31 | `0x143abf8a0` | `0x140308dba` | `[0x140308dd3, 0x140308de2)` | 0 | - |

The four **dynamic** gates (`0x14030558a`, `0x140305eea`, `0x1403068fd`, `0x1403069ef`)
select their key through a jump table over entries 0-6 rather than a direct `lea`, so they
are not in the table above. Entries 0-6 map to presence bytes 0(none), 44, 6, 5, 4, 3, 2.

## How this was measured, and how the instrument lied first

Method: scan `.text` for every RIP-relative reference landing anywhere in
`[0x143abeb10, 0x143abfd00)`, attribute each to its `.pdata` function, then read the
initialisers. Joining the 107 `lea` sites inside `FUN_140304b20` against the census's 43
gate call sites matched **39 of 43** - each `lea` exactly `0x11` bytes before its `CALL` -
and **every one of the 39 agreed with the census's own entry label**. The 4 that did not
match are exactly the 4 dynamic gates. Two independently-derived lists agreeing on 39 rows
is what makes this worth trusting.

**`tools/dataref.py` reported 1 write to this table. There are 41.** Its `OPS` table had
`0xC7` (`mov [rip+d], imm32`) but not `0xC6` (`mov byte [rip+d], imm8`), and every flag
initialiser stores its byte with `0xC6`. Worse, `--writes` filtered on the label `"write"`,
so adding the opcode alone still returned nothing - two independent filters, both dropping
the same evidence, both silently.

That is the **third** time on this project that a scan of a known set produced a clean,
confident, wrong number: the `BT` scan that missed a byte-array mask (wrong *shape*), the
five-decoder grep that missed two primitives (wrong *set*), and now this (wrong *opcode*).
Both halves are fixed in `tools/dataref.py`, with a positive control in the docstring.
