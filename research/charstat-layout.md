# `FUN_140302e30` — the character-stat block, byte for byte

Written 2026-08-19. **No Ghidra** (the project was locked by another process). Everything
below was read from text already in `research/` plus **the instruction bytes of
`client-patched\MapleStory.exe` read directly out of the PE**, which turned out to be the
decisive instrument.

Tags on every claim:

* **[L]** read off a listing / decompilation / the raw instruction bytes.
* **[D]** derived by analysis over those bytes.
* **[I]** inferred or a candidate — not provable from what is on disk.

---

## 0. The instrument, and its positive control

The exe is on disk and Ghidra imported it directly (`docs/ghidra.md`), so the `.text`
bytes can be read without Ghidra. Image base `0x140000000`, `.text` RVA `0x1000` at file
offset `0x600`. **[L]**

Before trusting a single byte I re-derived the ten instructions that
`research/charrecord-decode.md` had already published from a Ghidra listing. All ten match,
and both call targets resolve exactly:

```
140304e46  49 8b ce              MOV  RCX,R14
140304e49  e8 52 5b ff ff        CALL 0x1402fa9a0      <- resolves exactly
140304e4e  8b ce                 MOV  ECX,ESI
140304e50  80 38 00              CMP  byte ptr [RAX],0
140304e53  75 0f                 JNZ  140304e64        <- resolves exactly
140304e55  ff c1                 INC  ECX
140304e57  48 ff c0              INC  RAX
140304e5a  83 f9 64              CMP  ECX,0x64
140304e5d  72 f1                 JC   140304e50        <- resolves exactly
140304e5f  e9 42 01 00 00        JMP  140304fa6        <- resolves exactly
```

Two more controls: `0x1406e8f00` decodes to `e9 1b fd ff ff` = `JMP 0x1406e8c20`, the
documented bare thunk; and `140304b8e 44 8d 40 64` = `LEA R8D,[RAX+0x64]`, the documented
"100 is a constant, not a computed length". **[L]** The instrument reproduces four
independently-published facts, so I trusted it for the bytes nobody had read yet.

---

## 1. Which branch the character-record path takes — **`param_3 == 0`, measured**

This was the open question (`charrecord-reuse.md` **D9**) and it is now closed, from bytes,
in both directions.

**The call site.** `research/charrecord-decode.md` published the address but not the
argument setup. The 13 bytes at `0x140304e64` are: **[L]**

```
140304e64  44 8b 85 18 31 00 00   MOV  R8D, dword ptr [RBP+0x3118]   ; param_3
140304e6b  49 8b d7               MOV  RDX, R15                      ; param_2 = the packet
140304e6e  49 8b cc               MOV  RCX, R12                      ; param_1 = the record
140304e71  e8 ba df ff ff         CALL 0x140302e30                   ; resolves exactly
```

So `param_3` is **not** an immediate. It is a frame slot. The prologue says which one: **[L]**

```
140304b20  44 89 4c 24 20         MOV  [RSP+0x20], R9D     ; spill arg4 to its home slot
140304b25  55 53 56 57 41 54 41 55 41 56 41 57   ; 8 pushes
140304b31  48 8d ac 24 48 cf ff ff   LEA RBP,[RSP-0x30b8]
...
140304b57  4d 8b f8               MOV  R15, R8             ; R15 = arg3 = packet
140304b5f  4c 8b f2               MOV  R14, RDX            ; R14 = arg2 = the 100-byte array
140304b67  4c 8b e1               MOV  R12, RCX            ; R12 = arg1 = the record
```

**[D]** Let `E` be RSP at entry. After 8 pushes RSP = `E - 0x40`, so
`RBP = E - 0x40 - 0x30b8 = E - 0x30f8`. In the Microsoft x64 ABI arg4's home slot is
`[E + 0x20]`, which is `[RBP + 0x3118]` — **exactly the displacement in the instruction**,
and exactly the slot the function's first instruction spills `R9D` into. The arithmetic
also lands `arg1..arg3` on `[RBP+0x3100/0x3108/0x3110]` and the return address on
`[RBP+0x30f8]`, and puts the stack cookie slot `[RBP+0x30a0]` inside the frame — all
consistent.

**So `param_3` of the stat decoder is `arg4` of `FUN_140304b20`.**

**The caller passes 0.** At the `SetField` call site: **[L]**

```
142098414  44 89 64 24 20      MOV  [RSP+0x20], R12D     ; arg5
142098419  45 33 c9            XOR  R9D, R9D             ; arg4 = 0
14209841c  48 8b 7d c8         MOV  RDI,[RBP-0x38]
142098420  4c 8b c7            MOV  R8, RDI              ; arg3 = packet
142098423  48 8d 95 f0 02 00 00 LEA RDX,[RBP+0x2f0]      ; arg2 = the 112-byte scratch
14209842a  48 8b ce            MOV  RCX, RSI             ; arg1
14209842d  e8 ee c6 26 fe      CALL 0x140304b20
```

**[L]** All five decompiled call sites agree — `msexe-stage-setfield.c:496,1892,2028,2161`
and `msexe-gamestage-dispatch.c:1389` all pass a literal `0` as the fourth argument.

> **[L+D] The `SetField` record path takes the `param_3 == 0` branch — the same branch as
> the character list.** `charrecord-reuse.md` **D9** ("a non-zero `param_3` would make the
> client discard the id") is retired: `param_3` is zero, the client stores the id, and our
> habit of putting `chr.id` at offset 0 is correct rather than decorative.

**One correction to a file on disk.** `charrecord-loops.md` §8.4 calls `[RBP+0x3118]` "a
client-side local ... written from client state around `0x140304e64`". **[L]** It is neither
a local nor written there: it is arg4's home slot, written by the function's *first*
instruction and only *read* at `0x140304e64`. The other five references to it in that
function are all `CMP dword ptr [RBP+0x3118],0` — arg4 is a mode flag tested in five more
places. That does not change any count in that document.

---

## 2. Every call in `FUN_140302e30`, enumerated before filtering

**[L]** `0x140302e30 .. 0x1403034e0`, 1712 bytes. I enumerated **every** `E8 rel32` whose
target lands in `.text` rather than grepping for the seven known primitives — the mistake
`docs/ghidra.md` warns about twice. 61 call sites, **15 distinct targets**, none of them
overlapping a previous call's bytes, and every target begins with a plausible prologue:

| target | x | what |
|---|---|---|
| `0x1406e8ae0` | 6 | **u8** |
| `0x1406e8b80` | 8 | **u16** |
| `0x1406e8c20` | 21 | **u32** |
| `0x1406e8f10` | 1 | **u64** |
| `0x1406e9170` | 3 | **raw** |
| `0x1402f7010` | 8 | obfuscating store, 16-bit input |
| `0x1402f7170` | 1 | obfuscating store, 64-bit input |
| `0x1407386b0` | 6 | obfuscation key generator (`&DAT_143ac1ab0`) |
| `0x1401ab420` | 1 | reads back the job at `+0x33` for the SP fork |
| `0x1402cb0d0` | 1 | the extended-SP table sub-decoder |
| `0x14019b780`, `0x142e52ed0`, `0x142ef3bb8`, `0x142ef44b0`, `0x142f04924` | 1 each | alloc, assert, memcpy thunk, cookie check, key byte |

**[L] Neither `0x1406e9050` (string) nor `0x1406e8f00` (the u32 thunk) is called.** This
function contains **no variable-length string and no thunked read** — the two things that
made the census of `FUN_140304b20` wrong twice. 39 primitive call sites total, split across
the two `param_3` branches.

**[L]** The fork itself:

```
140302e5c  45 85 c0   TEST R8D,R8D
140302e5f  74 58      JZ  140302eb9        ; param_3 == 0  -> the branch we use
140302e61             ...                  ; param_3 != 0  falls through
140302eb7  eb 6a      JMP 140302f23        ; and rejoins here
```

**[D]** The two branches are `[140302e61, 140302eb7]` (9 reads) and
`[140302eb9, 140302f23]` (9 reads); common code runs from `0x140302f23`, whose first read is
at `0x140302f26`. Per execution: **9 + 21 = 30 primitive reads** (29 when the extended-SP
branch is taken, which skips one `u16`).

---

## 3. The layout — `param_3 == 0`, in wire order

Offsets are byte offsets from the start of the block. The **Offset** column is for an
**extended-SP job with zero SP pools** (what we send); the parenthesised value is for a
plain-`u16`-SP job, where everything after the fork shifts by +1.

`FUN_1402f7010(v, dst)` and `FUN_1402f7170(v, dst)` store `v` obfuscated at `dst` and return
a companion word stored just after. "obf(k,v,c)" means the value is spread over three record
words — key, value, checksum — via `FUN_1407386b0(&DAT_143ac1ab0)`.

| # | addr | primitive | width | offset | destination (decompiler) |
|---:|---|---|---:|---:|---|
| 1 | `140302eb9` | u32 | 4 | 0 | `*param_1` → `+0x00` |
| 2 | `140302ec3` | u32 | 4 | 4 | `param_1[1]` → `+0x04` |
| 3 | `140302ece` | u32 | 4 | 8 | `param_1[2]` → `+0x08` |
| 4 | `140302ee3` | raw | **13** | 12 | `FUN_1406e9170(pkt, param_1+3, 0xd)` → `+0x0c` |
| 5 | `140302eeb` | u8 | 1 | 25 | `+0x19` |
| 6 | `140302ef6` | u8 | 1 | 26 | `+0x1a` |
| 7 | `140302f01` | u32 | 4 | 27 | `+0x1b` |
| 8 | `140302f10` | u32 | 4 | 31 | `+0x1f` |
| 9 | `140302f1b` | u32 | 4 | 35 | `+0x23` |
| 10 | `140302f26` | u32 | 4 | 39 | obf → `+0x27`,`+0x2b`,`+0x2f` |
| 11 | `140302f56` | u16 | 2 | 43 | `FUN_1402f7010` → `+0x33`, companion `+0x37` |
| 12 | `140302f6d` | u16 | 2 | 45 | → `+0x3b`, `+0x3f` |
| 13 | `140302f84` | u16 | 2 | 47 | → `+0x43`, `+0x47` |
| 14 | `140302f9b` | u16 | 2 | 49 | → `+0x4b`, `+0x4f` |
| 15 | `140302fb2` | u16 | 2 | 51 | → `+0x53`, `+0x57` |
| 16 | `140302fc9` | u32 | 4 | 53 | obf → `+0x5b`,`+0x5f`,`+0x63` |
| 17 | `140302ff9` | u32 | 4 | 57 | obf → `+0x67`,`+0x6b`,`+0x6f` |
| 18 | `140303029` | u32 | 4 | 61 | obf → `+0x73`,`+0x77`,`+0x7b` |
| 19 | `140303059` | u32 | 4 | 65 | obf → `+0x7f`,`+0x83`,`+0x87` |
| 20 | `14030308f` | u16 | 2 | 69 | `FUN_1402f7010` → `+0x8b`, `+0x8f` |
| — | **SP fork** | — | 1 or 2 or 1+5n | 71 | see §4 |
| 21 | `140303208` | **u64** | 8 | 72 (73) | `FUN_1402f7170` → `+0x9b`, companion `+0xab` |
| 22 | `140303225` | u32 | 4 | 80 (81) | obf → `+0xb3`,`+0xb7`,`+0xbb` |
| 23 | `14030325e` | u32 | 4 | **84 (85)** | `local_68` → mangled into the 12-byte heap object at `+0xfb`; counter at `+0xf3` — **the map, see §5** |
| 24 | `140303441` | u8 | 1 | 88 (89) | `+0x10b` |
| 25 | `14030344f` | u16 | 2 | 89 (90) | `*(undefined2 *)(param_1 + 0x43)` → `+0x10c` |
| 26 | `14030345e` | u8 | 1 | 91 (92) | `+0x10e`, widened to `u32` |
| 27 | `14030347a` | raw | **8** | 92 (93) | stack buf → `(*DAT_1432625b0)(buf, param_1+0x112)` |
| 28 | `140303494` | u32 | 4 | 100 (101) | `local_5c` — the **high** dword of the value at `+0x126` |
| 29 | `1403034a0` | u32 | 4 | 104 (105) | `local_60` — the **low** dword → `(*DAT_1432625b0)(&local_60, param_1+0x126)` |

**[L]** Both raw sizes are compile-time immediates, read off the bytes:
`140302edd 41 b8 0d 00 00 00` = `MOV R8D,0xD` (13) and
`14030346b 41 b8 08 00 00 00` = `MOV R8D,8`. Neither is variable.

**[L] Reads 28/29 are high-half-first.** The bytes are unambiguous:

```
140303494  CALL u32
140303499  89 44 24 2c        MOV [RSP+0x2c], EAX      ; first read -> +0x2c
14030349d  48 8b cd           MOV RCX, RBP
1403034a0  CALL u32
1403034a5  48 8d 93 26 01 00 00  LEA RDX,[RBX+0x126]
1403034ac  89 44 24 28        MOV [RSP+0x28], EAX      ; second read -> +0x28
1403034b0  48 8d 4c 24 28     LEA RCX,[RSP+0x28]       ; the 8-byte span starts at +0x28
1403034b5  ff 15 f5 f0 f5 02  CALL [DAT_1432625b0]
```

The span handed to the converter is `[RSP+0x28 .. RSP+0x30)`, whose **low** dword is the
**second** read. This confirms `charrecord-reuse.md` **D2** at the byte level, and it is the
same converter used for the 8 raw bytes at offset 92, so **[I]** both are very likely
`FILETIME` (`{DWORD low; DWORD high;}`) — the *ordering* is [L], the *name* is [I].

### For the record: the `param_3 != 0` branch

**[L]** `140302e61` u32 (discarded), `140302e69` u32 → `+0x04`, `140302e74` u32 → `+0x08`,
`140302e8a` raw 13 → `LEA RDX,[RSP+0x38]`, a **stack** buffer (discarded), `140302e92` u8
(discarded), `140302e9a` u8 (discarded), `140302ea2`/`140302eaa`/`140302eb2` u32 (discarded).
Same nine widths, same order, same 39 bytes. Not the path in use; recorded so nobody has to
read it again.

---

## 4. The variable part — the SP fork, and its exact condition

**[L]** `1403030b0 CALL 0x1401ab420` with `LEA RCX,[RBX+0x33]` and `MOV EDX,[RBX+0x37]` —
the client reads its own **job** field back out and switches on it. The gate is a genuine
bit test here (unlike `FUN_140304b20`'s byte array):

```
140303133  81 c1 0c fe ff ff   ADD  ECX, -0x1f4     ; job - 500
140303139  83 f9 16            CMP  ECX, 0x16
14030313c  77 09               JA   140303147
14030313e  0f a3 ca            BT   EDX, ECX
140303141  0f 82 9b 00 00 00   JB   1403031e2       ; -> extended-SP branch
140303147  48 8b cd            MOV  RCX, RBP
14030314a  e8 31 5a 3e 00      CALL u16             ; -> plain u16 sp
```

**[L]** The full condition, from the decompilation, matching `opcode.rs:720`
(`uses_extended_sp`): job `== 0`, or `job-100` / `job-200` in mask `0x1c0701c01` (bits
0,10,11,12,20,21,22,30,31,32), or `job-300` / `job-400` / `job-500` in mask `0x701c01`
(bits 0,10,11,12,20,21,22), or `430 <= job <= 439`.

**Extended branch** (`1403031e2`) — **[L]** `XOR ECX,ECX` then `FUN_1402f7010(0, +0x93)`, so
`sp` is set to **zero without reading anything**, then `FUN_1402cb0d0(param_1+0xd7, packet)`.
**[L]** `FUN_1402cb0d0` (`msexe-newchar-scan.c:1957`) is `u8 count`, then `count x (u8, u32)`,
summing the `u32`s into `param_1+0x18` — which is why the `u32` is the SP amount and the `u8`
the job level, not the reverse. **Wire cost: `1 + 5n` bytes.**

**Plain branch** — one `u16` at `14030314a`. **Wire cost: 2 bytes.** It also walks and frees
the extended-SP list at `+0xdf` and zeroes `+0xdb`/`+0xe7`/`+0xef`.

### Totals

**[D]** Fixed portion outside the fork: **71 bytes before + 36 bytes after = 107**.

| case | total |
|---|---|
| extended-SP job, 0 pools | **108** |
| extended-SP job, n pools | **108 + 5n** |
| plain-SP job | **109** |

Nothing else in this function is conditional or variable. **[L]** No string reads, no
count-prefixed loops, no early returns.

---

## 5. The map / field id — **read #23, offset 84**

This is the question the task exists for, so here is the whole chain rather than the
conclusion.

**Step 1 — where read #23 goes. [L]** `FUN_140302e30` reads a `u32` at `0x14030325e` into
`local_68` and then, instead of storing it plainly, does this:

* increments a counter at `param_1 + 0xf3`, and every `0x6f` (111) decodes reallocates a
  **12-byte heap object** whose pointer lives at `param_1 + 0xfb`;
* writes a key byte from `FUN_142f04924()` to `obj+4`;
* seeds `obj[8] = 0x65, obj[9] = 0x9a` (little-endian `0x9a65`);
* runs a 4-byte XOR chain with the constant `0x2a` over `local_68`, folding each byte into a
  rolling 16-bit checksum at `obj+8` with a 3-bit rotate.

**[D]** That is the heaviest anti-tamper treatment in the whole block — heavier than the six
`0xbaadf00d` fields — and it protects exactly one `u32`.

**Step 2 — the getter is the exact inverse. [L]** `FUN_1402fa540(int *param_1)`
(`msexe-presence-gate.c:89`) loads `*(param_1 + 2)` — for an `int*`, byte offset **`+8`** —
and unmangles four bytes with the *same* `0x2a` chain, the *same* `0x9a65` seed and the
*same* 3-bit rotate, then compares its recomputed checksum against `obj+8` and reports
tampering on mismatch. Encoder and decoder are byte-for-byte the same scheme on the same
object. **[D]** So `FUN_1402fa540(record + 0xf3)` returns precisely the `u32` that read #23
put in.

**Step 3 — what the client does with it. [L]** `msexe-stage-setfield.c:532`, immediately
after `FUN_140304b20` returns:

```c
piVar14 = (int *)(uVar33 + 0xf3);          // uVar33 is arg1 of FUN_140304b20,
iVar7 = FUN_1402fa540(piVar14);            //   i.e. param_1 of FUN_140302e30
if (local_448 != iVar7) {                  // the value changed
    uVar8 = FUN_1402fa540(piVar14);
    FUN_1403999e0(DAT_143aa8328, &local_2c8, uVar8, PTR_s_mapName_143a49020);
    FUN_1408b0600(&local_2d0, "\tM\rA\nP", &DAT_14337b3c8);
    ...
}
```

**[L] I dereferenced `PTR_s_mapName_143a49020` out of the PE.** It points at `0x1432b3ec0`
in `.rdata`, which holds the ASCII string **`mapName`** (followed by `fixWidth`, `story`,
`m1`, `m2`, `m3`). And `DAT_14337b3c8` holds `" \r\n\t"`, the separator set that turns the
obfuscated literal `"\tM\rA\nP"` into **`MAP`**. So the value is used as the **key for a
`mapName` resource lookup**.

**[D]** `uVar33` is the same pointer chain the whole way: the `SetField` stage passes it as
arg1, `FUN_140304b20` keeps it in R12, and `140304e6e MOV RCX,R12` makes it `param_1` of
`FUN_140302e30`. Same object, verified at the instruction level in §1.

**Step 4 — an independent second line. [L]** `msexe-stage-setfield.c:403-412`, the
`characterData == 0` branch — the short "same character, new map" form:

```c
piVar14 = (int *)(uVar33 + 0xf3);
uVar8 = FUN_1402fa540(piVar14);     // read the CURRENT value
FUN_142d01d40(lVar24, uVar8);
local_3e8 = FUN_1406e8c20(param_2); // read a u32 from the wire
...                                 // and write it back through the same +0xf3/+0xfb slot
                                    // (identical counter + 0x6f reallocation)
```

**[D]** On a packet form whose entire purpose is to move a character to a new map, the `u32`
it reads goes into **the identical secure slot** that read #23 fills. Two different packet
shapes, one storage location, and a `mapName` lookup hanging off it.

> ### Conclusion
>
> **[L+D, three independent lines] The map / field id is read #23: the `u32` at
> `0x14030325e`, at byte offset 84 of the block** (85 for a plain-SP job; in general
> `83 + spBytes`). It is not stored plainly — it goes through the obfuscated 12-byte object
> at `record+0xfb`, read back by `FUN_1402fa540(record+0xf3)`.

### Pinning the ordinal, from the listing rather than the decompiler

The decompiler reorders, so all three of these are counted over the **call sites in address
order** in §2's enumeration, not over the decompiled statements. **[L/D]**

| question | answer |
|---|---|
| address of the call | `0x14030325e` |
| ordinal among **all** reads executed on the `param_3 == 0` path | **23rd of 29** |
| ordinal among **`FUN_1406e8c20` (u32) calls** on that path | **13th of 15** |
| byte offset from the start of the stat block | **84** (85 for a plain-SP job) |
| bytes read **after** it, to the end of the block | **6 reads, 20 bytes** |

**[D]** The "23rd of 29" counts the SP fork as *not* a read, which is correct for the
extended-SP path (the fork reads nothing in this function — `XOR ECX,ECX`, then the
sub-decoder `FUN_1402cb0d0` does its own reads). For a **plain-SP** job the `u16` at
`0x14030314a` is a read in this function, so the map becomes the **24th of 30** and the
offset shifts to 85. The u32 ordinal (13th) is unaffected either way.

**[D] Nothing is read between read #22 (offset 80) and read #23.** The only call in between
is `0x140303233 FUN_1407386b0`, the obfuscation-key generator — not a packet read. The
calls between #23 and #24 (`0x1403032a2 FUN_14019b780` alloc, `0x1403032c6 FUN_142ef3bb8`,
`0x1403032cb FUN_142f04924` key byte) are the reallocation and mangling machinery, also not
reads. **[L]** Verified against the full enumeration in §2, which found no eighth primitive.

**After it, to the end of the block** — reads #24-#29, **20 bytes**: `u8` (offset 88),
`u16` (89), `u8` (91), `raw 8` (92), `u32` high (100), `u32` low (104), ending at **108**.
>
> This is stronger than a reference-source name: the string `mapName` is a literal in the
> client's own `.rdata`, and the `characterData == 0` branch corroborates it independently.
> **[I]** What I have *not* proved is that no *other* field also carries a map id — I did not
> enumerate consumers of the remaining unnamed reads (#2, #3, #7, #8, #9, #24, #25, #26).

**Consistent with the measured crash. [L]** `msexe-stage-setfield.md:23` records the client
faulting at `FUN_1402fa540+0x1c` on the `characterData == 0` branch. **[L]** `+0x1c` is
`1402fa55c  4c 8b 51 08  MOV R10,[RCX+8]` — the load of the secure-object pointer at
`record+0xfb`. **[I]** The natural reading is that the slot was never populated because
`FUN_140302e30` never ran on that branch; I did not prove which operand was bad, so treat the
causal story as inference and the faulting instruction as fact.

**Retiring D3.** `charrecord-reuse.md` **D3** flagged `map_id` at trailer offset 120 as "the
least-supported placement in everything we build". That was right, and the placement is
wrong — see §6.

---

## 6. Diff against `crates/net/src/opcode.rs`

`character_record()` at `opcode.rs:803`. **Same order, same widths, 29 for 29.** Comparing
the builder's stat-block statements against the table in §3:

| client read | our statement | verdict |
|---|---|---|
| 1-3 u32,u32,u32 | `chr.id`, `chr.id`, `world_id` | order + width agree |
| 4 raw 13 | `put_fixed(name, 13)` | agree |
| 5,6 u8,u8 | `gender`, `skin` | agree |
| 7,8,9 u32 x3 | `0`, `face`, `hair` | agree |
| 10 u32 | `level` | agree |
| 11 u16 | `job` | agree |
| 12-15 u16 x4 | `str,dex,int,luk` | agree |
| 16-19 u32 x4 | `hp,max_hp,mp,max_mp` | agree |
| 20 u16 | `ap` | agree |
| fork | `if uses_extended_sp {u8 0} else {u16 0}` | agree, and the condition matches the bit masks |
| 21 u64 | `0` (exp) | agree |
| 22 u32 | `0` (fame) | agree |
| **23 u32 — the map** | **`0`** | **WRONG VALUE — see below** |
| 24 u8 | `0` (portal) | agree |
| 25 u16 | `0` (subJob) | agree |
| 26 u8 | `0` | agree |
| 27 raw 8 | `0u64` (FILETIME) | agree |
| 28 u32 | `0` | agree on width; this is the **high** half |
| 29 u32 | `0` | agree on width; this is the **low** half |

**No framing disagreement anywhere.** Length checks out too: `opcode.rs:1817` asserts 327
bytes for the default character, of which the stat block is 108 — matching §4 exactly.

### The one real disagreement

**[L]** `opcode.rs:845` writes `chr.map_id` into the `u32` at **record offset 120** — the
third field of `FUN_1403094b0`'s trailer, client destination `+0x325`, which
`FUN_1403094b0` then copies to `+0x308`.

**[L] What we currently put at offset 84 is `opcode.rs:834`** — a bare
`out.extend_from_slice(&0u32.to_le_bytes());`, the only statement in the stat block with no
trailing comment, sitting between `// fame` (833) and `// portal` (835). It is a literal
zero and the struct has no field feeding it. **So the map id the client will look up is
hardcoded to 0**, and `mapName` for field 0 is what the probe run was actually asking the
client to resolve.

The one-line fix is to make line 834 carry `chr.map_id` for the `SetField` path. **[I]** I
would not simply move it off line 845 until someone establishes what offset 120 is — see
point 2 below.

Three consequences, in order of cost:

1. **For `SetField`, the map is at offset 84, not 120.** The trailer is read by
   `FUN_1403094b0`, which is **not** on the `SetField` path at all
   (`charrecord-reuse.md` **D1**). A `SetField` that carries the map at 120 carries it
   nowhere.
2. **[I] Offset 120 may be the map for the character *list* as well**, in which case our
   builder has had it in the wrong slot all along and nothing noticed because character
   select never renders a map. I have **not** established what offset 120 is; the client
   copies `+0x325` to `+0x308` and I did not chase either consumer. Do not "fix" the list
   path on the strength of this document.
3. **Reads 28/29 are high-dword-first** (§3). Harmless while we send zeros; wrong the moment
   a real timestamp goes there.

### The gate, and why none of this has been on the wire yet

**[L]** `research/charrecord-presence-map.md` (written in parallel with this pass) settles
the gate: `FUN_1402fa9a0` is a 100-byte bytewise AND against a per-gate key mask, each key
is built at startup as 100 zero bytes with exactly one set to `1`, and the gate guarding
this block — entry 7, key at `0x143abee20` — sets byte 0. **So `presence[0] = 1` switches
the stat block on**, and `1` is sufficient because the key byte is exactly `0x01`.

**[D] This corrects something I wrote earlier in this document.** The accepted-minimal-record
run (`research/fixtures/setfield-accepted-client-entered-world-world.log`) sent **every
presence flag clear**, so the gate at `0x140304e49` scanned 100 zero bytes and jumped to
`0x140304fa6`. **`FUN_140302e30` never executed on that run.** Its own log note — *"the
record is all zeros, so the map is 0 - and 0 is not a map"* — is right about the outcome but
not the mechanism: offset 84 was not read as zero, it was **never read at all**, and the
secure slot at `record+0xfb` was never populated. That is also why that run is not evidence
for or against any offset in this document.

> **Nothing in this table has been on the wire.** Every byte here is static analysis. The
> first run that sets `presence[0] = 1` is the first test of all 108 bytes at once — and per
> `CLAUDE.md`, that is one variant, so change nothing else on that launch.

### What this does not settle

**[I]** Everything past the 108-byte stat block on the `SetField` path is still unknown —
`FUN_140304b20` has 126 reads of its own and only the first 11 precede this block
(`charrecord-decode.md`, `charrecord-loops.md`). This document covers the shared function and
nothing else. `charrecord-reuse.md` **D4** (face/hair naming), **D5** (which obfuscated `u32`
is which of hp/maxHp/mp/maxMp) and **D6** (portal) are all untouched by this pass — none of
those reads has an identified consumer, and I did not look for one.

---

## Addendum from the main session: the fields immediately after the map id

Read from `research/msexe-charstats.c`, straight after the loop that encodes the map id into
the obfuscated slot (`} while (uVar10 < 4);`):

```c
uVar5  = FUN_1406e8ae0(param_2);  *(byte *)  (param_1 + 0x10b) = uVar5;   // "portal", offset 88
uVar7  = FUN_1406e8b80(param_2);  *(ushort *)(param_1 + 0x43)  = uVar7;   // subJob,   offset 89
bVar18 = FUN_1406e8ae0(param_2);  *(uint *)  (param_1 + 0x10e) = bVar18;  // u8 -> uint, offset 91
         FUN_1406e9170(param_2, local_58, 8);
         (*DAT_1432625b0)(local_58, param_1 + 0x112);                     // raw 8,    offset 92
local_5c = FUN_1406e8c20(param_2);                                        // u32,      offset 100
local_60 = FUN_1406e8c20(param_2);
         (*DAT_1432625b0)(&local_60, param_1 + 0x126);                    // u32,      offset 104
```

**[L]** The `portal` byte is **stored plainly** at `record + 0x10b` and is not consumed
inside the stat decoder. Whether the client later treats it as a **portal index** or as a
**spawn-point id** is **still unsettled** - `tools/xref.py --field 0x10b --size byte` finds
17 writes across 13 functions, but `0x10b` is a generic struct offset so most of those are
unrelated types, and it does not search reads at all. **That instrument does not answer this
question; do not read its output as if it did.**

It is **moot for map 1**, where portals 0-3 are all type-0 `sp` spawns
(`research/map1-exists.md`), so `portal = 0` lands on the spawn either way. It will matter
on a map where the index and the id differ.
