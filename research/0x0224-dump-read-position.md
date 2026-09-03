# The `0x0224` reader's state at the moment it threw, read out of both crash dumps

**2026-09-03. Two full-memory dumps, no client run, no Ghidra** (another process held the
lock). Everything here comes from `tools/dumpwalk.py` against
`dumps\maplecw-crash-970152-c0000005-1.dmp` (1.26 GB) and
`dumps\maplecw-crash-970748-c0000005-1.dmp` (1.40 GB), plus three scratch scripts whose
logic is quoted in full below.

Tags: **[L]** read out of the dump or a listing; **[D]** derived from two or more [L];
**[I]** inferred.

Companion files, neither edited here: `research/avatar-look-loops.md` (the static walk,
owned by another agent — §6 below carries one correction to its §4.4) and
`research/0x0224-body-walk.md`. Nothing under `crates/` was touched.

---

## 0. The answer

* **The reader had consumed 672 of 675 body bytes (Tester2) / 671 of 674 (Cobalt) and was
  asking for 4 more, with 3 left.** Both dumps, identical condition. **[L]**
* **The read that threw was 4 bytes, not thousands.** It is `FUN_1406e8c20`, the `u32`
  primitive, whose own listing is `cmp edi, 4 / jb <raise>`. There is no single oversized
  read anywhere in this failure. **[L]**
* **The implausible number is one level up: an element count of `0x21000000` = 553 648 128
  (Tester2) / `0x22000000` = 570 425 344 (Cobalt)**, read by `FUN_140862470` from **body
  offset 204 / 203**. The loop then read `u32`s four bytes at a time. **[L]** — recovered
  from the loop counter still sitting in the saved `rdi`, not inferred from the layout.
* **It completed 116 iterations and threw on the 117th**, in both processes. 467 body bytes
  follow the count field in both bodies, and `467 = 4x116 + 3`. **[L] + [D]**
* **Both dumps agree on every number that should be the same and differ only where the two
  bodies differ.** Same reader-object stack address, same frame addresses, same remaining =
  3, same 116 iterations, byte-identical call chain. **[L]**
* The `0xC0000005` is **downstream**: a refcount release run by the unwinder on a pointer
  that was never initialised, six frames out from the throw. §7. **[L]**

---

## 1. The positive control, first, because it is the thing that could have stopped this

The brief's own condition: *if the known body bytes are not in the dump, say so and stop.*

Searching every byte of both `Memory64List`s for the two bodies in
`research/fixtures/0x0224-bodies-that-killed-both-clients.txt` (24- and 25-byte header
needles, then the full body compared byte-for-byte at each hit):

| dump | Cobalt body (674 B) | Tester2 body (675 B) |
|---|---|---|
| `970152` | **0 hits** | **2 hits** — `0x6195576`, `0x2e885086` |
| `970748` | **2 hits** — `0x6133d86`, `0x2c944d46` | **0 hits** |

**[L]** Each client holds only the *counterpart's* body, which is what `UserEnterField`
means, and each holds it exactly twice. `970152` is Cobalt's process, `970748` is
Tester2's. The search is not blind: it returns hits, it returns them in the right dump, and
it returns none in the wrong one.

Both copies are immediately preceded by `24 02` — opcode `0x0224` little-endian — and four
more bytes before that:

```text
0x2e885078  a9 02 00 00 00 00 00 00   <- 0x2a9 = 681
0x2e885080  ce ac 6b ae               <- 4-byte packet header
0x2e885084  24 02                     <- opcode 0x0224
0x2e885086  d6 00 00 00 ...           <- the body, 675 bytes
```

`681 = 4 + 2 + 675`. **[L]**

---

## 2. The reader object, and why the field offsets are not taken on trust

`research/buffs-underflow.md` already names `+0x18` length and `+0x24` position. That is a
claim from a listing quoted in another file, so it was re-derived from **the image as
mapped in this dump** (`dumpwalk.py --dis`, which reads the packer-unpacked bytes):

```asm
1406e8c20  mov  [rsp+0x10], rbx
1406e8c25  mov  [rsp+8], rcx          ; <- the reader object is spilled here
1406e8c2a  push rdi                   ; <- the caller's rdi is spilled here
1406e8c2b  sub  rsp, 0x50
1406e8c32  mov  edi, [rcx+0x18]       ; length
1406e8c35  sub  edi, [rcx+0x24]       ; minus position = bytes remaining
1406e8c38  mov  rax, [rcx+0x10]       ; buffer
1406e8c5b  mov  r8d, [rax-8]          ; the buffer's own element count, at buf-8
1406e8c70  mov  ecx, [rbx+0x24]
1406e8c75  add  rax, [rbx+0x10]       ; read address = buffer + position
1406e8c79  cmp  edi, 4                ; <<< THE REQUESTED LENGTH IS 4
1406e8c7c  jb   1406e8c91             ; fewer than four remain -> raise
1406e8c7e  mov  eax, [rax]
1406e8c80  add  ecx, 4
1406e8c83  mov  [rbx+0x24], ecx       ; position += 4, ONLY on the success path
1406e8c91  mov  edx, 0x26             ; the raise path
1406e8c96  lea  rcx, [rsp+0x28]       ; the exception object, at entry_rsp-0x30
1406e8ca0  lea  rdx, [rip+0x3352471]  ; -> 0x143a3b118, the ThrowInfo
1406e8cac  call 142ef6d4c             ; _CxxThrowException
1406e8cb1  int3
```

**[L]** Two things this buys beyond confirming the offsets:

* `mov r8d, [rax-8]` means the buffer carries **its own length eight bytes before it**. That
  is a check the object has to pass, not a description of it — and it is what the object
  search below is built on.
* The raise path **does not write `[rbx+0x24]`**, so the position left in memory *is* the
  position at the throw. Nothing edited it afterwards.

### The search, and why an empty result would have been real

Rather than looking for a pointer to a known buffer, the whole 44 232-byte stack was walked
for **any** offset `P` satisfying an identity that can fail:

```text
[P+0x10] is a readable pointer   AND   u32 at ([P+0x10] - 8)  ==  u32 at [P+0x18]
```

That is the client's own consistency rule between a buffer and its length. Results:

| dump | candidates | with opcode `0x0224` at buffer+4 |
|---|---|---|
| `970152` | 3 | **1** — stack `0x14e920` |
| `970748` | 5 | **1** — stack `0x14e920` |

**[L]** The rejected candidates are visibly junk — across the two dumps their `datalen`
fields read 32, 16, 373 644, 380 948 and 1 344 752, all with `rawseq` 0 and position 0, and
none has `0x0224` at buffer+4. The survivor is at the **same stack address in both
processes**.

### The two objects, side by side

| field | offset | `970152` (holds Tester2's body) | `970748` (holds Cobalt's body) |
|---|---:|---|---|
| buffer | `+0x10` | `0x2e885080` | `0x2c944d40` |
| length | `+0x18` | **681** | **680** |
| rawseq | `+0x1c` | `0xacce` | `0xd2ad` |
| datalen | `+0x20` | 677 | 676 |
| **position** | `+0x24` | **678** | **677** |
| **remaining** | | **3** | **3** |
| buffer's own count at `buf-8` | | 681 | 680 |
| `buffer[0:2]` | | `ce ac` | `ad d2` |

**[L]** Four internal checks all pass and none was assumed: `length == buf[-8]`;
`datalen == length - 4`; `rawseq` equals the first two header bytes; `length == 4 + 2 +
len(body)`; and the body at `buffer+6` is byte-for-byte the fixture.

### What the position points at in our body

Body starts at `buffer + 6`, so:

| | position | body offset | body length | bytes left |
|---|---:|---:|---:|---:|
| Tester2 (in `970152`) | 678 | **672** | 675 | 3 |
| Cobalt (in `970748`) | 677 | **671** | 674 | 3 |

**[D]** In both cases the cursor died **three bytes from the end of the body**, deep inside
`USER_ENTER_FIELD_PAD_LEN`'s 128 trailing zeros. The padding was consumed, not skipped.

---

## 3. Tying that object to the throw, rather than assuming it

The object above is the right *shape*. What makes it the object the throwing call was
holding is frame arithmetic, and it closes exactly.

`0x1406e8cb1` (the `int3` after `_CxxThrowException`) sits at stack **`0x14d488`** in both
dumps. From the prologue above, a call pushes at `entry_rsp - 0x60`, so
**`entry_rsp = 0x14d4e8`**. Every slot that address predicts is present:

| predicted | address | value found | meaning |
|---|---|---|---|
| return address | `[0x14d4e8]` | `0x1408624d0` | return of `0x1408624cb call 0x1406e8c20` |
| `rcx` spill (`entry+8`) | `[0x14d4f0]` | **`0x14e920`** | **the reader object above** |
| `rdi` spill (`entry-8`) | `[0x14d4e0]` | `0x20ffff8c` / `0x21ffff8c` | the caller's loop counter |
| exception object (`entry-0x30`) | `0x14d4b8` | `0x26` | the `mov edx, 0x26` message id |

and the `EXCEPTION_RECORD` for the C++ throw, found by scanning the stack for
`0xE06D7363`, names that same object:

```text
0x14d370  code 0xe06d7363  npar 4
          param[0] 0x19930520      EH magic
          param[1] 0x14d4b8        <- the exception object at entry_rsp-0x30
          param[2] 0x143a3b118     <- exactly the ThrowInfo at 0x1406e8ca0's lea
          param[3] 0x140000000
```

**[L], in both dumps, at identical addresses.** Five independent predictions, five hits.
This is the frame that threw, and its `rcx` was the `0x0224` reader.

`0x1408624d0` is the return address of the call at `0x1408624cb`, which is inside
`FUN_140862470`'s **element loop**, not its header reads — that matters for §4.

---

## 4. The count, recovered from the loop counter

`FUN_140862470`, disassembled in full from the dump:

```asm
140862470  ...                          ; rcx = out struct (rsi), rdx = CInPacket (rbp)
140862491  mov [rcx+0x18], rax          ; vector end = begin (no reserve anywhere)
140862498  call 1406e8c20               ; u32 -> [rsi+0]
1408624a2  call 1406e8c20               ; u32 -> [rsi+4]
1408624ad  call 1406e8c20               ; u32 -> [rsi+8]
1408624b8  call 1406e8c20               ; u32 COUNT
1408624bd  test eax, eax / jle 140862500
1408624c6  mov  edi, eax                ; rdi = iterations remaining
1408624c8  mov  rcx, rbp                ; loop head
1408624cb  call 1406e8c20               ; <<< THREW HERE, one u32 per iteration
1408624de  mov  [rdx], eax              ; push_back
1408624f5  sub  rdi, 1
1408624f9  jne  1408624c8
```

**[L]** `{ u32; u32; u32; u32 count; u32 items[count]; }`. `rdi` at the failing call is
therefore **the number of iterations still to run**, and it is the value the primitive
pushed at `entry_rsp-8`.

That gives one equation with one unknown. If the count field sits at buffer offset `P`, the
elements occupy `P+4 .. P+4+4N` and the last one ended at the observed final position, so

```text
N = (final_position - 4 - P) / 4        and        u32_at(P) == rdi + N
```

Every `P` in the buffer was tested. A wrong `P` simply does not satisfy the equality, so an
empty result would have been a real negative:

| | `rdi` | solutions | count field at | value read | N (iterations done) |
|---|---:|---:|---|---:|---:|
| `970152` (Tester2) | `0x20ffff8c` = 553 648 012 | **1** | buffer 210 = **body 204** | `0x21000000` = **553 648 128** | **116** |
| `970748` (Cobalt) | `0x21ffff8c` = 570 425 228 | **1** | buffer 209 = **body 203** | `0x22000000` = **570 425 344** | **116** |

**[L]** One solution each, and the same `N` in both.

The arithmetic closes independently against the bodies themselves: 675 − 208 = **467** and
674 − 207 = **467**, and `467 = 4x116 + 3`. **[D]** The loop was always going to stop three
bytes short whatever the count was, because 467 is not a multiple of four.

### What the count field actually overlapped

```text
Tester2  buffer[210..213] = 00 00 00 21     Cobalt  buffer[209..212] = 00 00 00 22
                                  ^^                                        ^^
              low byte of 0x4E21 = 20001                 low byte of 0x4E22 = 20002
```

**[D]** The high byte of the misread `u32` is the low byte of the avatar look's `face`
value, three bytes further on. Naming that field and deciding what belongs in front of it is
`research/avatar-look-loops.md` §4 and `research/0x0224-body-walk.md`; this file only
measures where the cursor was. The dump's number and their static number agree, and the two
do not share an instrument: one is register and memory state out of a process image, the
other is a listing walk that never opened a dump.

---

## 5. The frozen call chain

The dump's own context is inside the unwinder, so `dumpwalk`'s walk shows EH machinery. The
throwing frames are still on the stack above it. Seeding the unwinder at the raise path
(`rip 0x1406e8c91, rsp 0x14d490`) replays their prologues:

```text
00  rsp 0x14d490   0x1406e8c20 +0x71   u32 read primitive, raise path
01  rsp 0x14d4f0   0x1408624d0         return of 1408624cb call 1406e8c20   (element loop)
02  rsp 0x14d520   0x140a4a113         return of 140a4a10e call 140862470
03  rsp 0x14d6a0   0x1429ce4e9         return of 1429ce4e4 call 140a46e50
04  rsp 0x14dbc0   0x1429ba85b         fn 0x1429ba3e0+0x47b
05  rsp 0x14dcb0   0x1429b9472         fn 0x1429b9300+0x172
06  rsp 0x14dd10   0x141821e41         fn 0x141820080+0x1dc1
07  rsp 0x14e6b0   0x144ada037         fn 0x144ad9fc8+0x6f
08  rsp 0x14e720   (walk ends)
```

**Byte-identical in both dumps, frame for frame and rsp for rsp.** **[L]**

Frames 02 and 03 are `FUN_140a46e50` (`CSecondaryStat::DecodeForRemote`, per
`research/remote-attack-verification.md:497`) and its call site `0x1429ce4e4`, which
`research/user-enter-field.md` §2.2 already records. `tools/callers.py 0x140862470`
independently reports exactly two call sites, `0x140a45ad8` and **`0x140a4a10e`**, and zero
tail-jumps and zero pointers — so frame 02's return address is the only place it could have
come from. **[L]**

`0x14d6b0` — `FUN_140a46e50`'s third-argument home slot, `[rsp+0x190]` — holds `0x14e920`,
the reader object. **[L]**

---

## 6. The correction to `research/avatar-look-loops.md` §4.4

That file's static derivation of *which* field is misread and *what value* comes out is
confirmed here exactly, by a completely different instrument. Two statements in it are
wrong, and only a dump could have caught them:

| its §4.4 says | the dumps say |
|---|---|
| "The **first iteration** of the loop at `0x1408624c8` exhausts the body" | the **117th** does. 116 completed; `rdi` had 553 648 012 / 570 425 228 still to go |
| "**bytes demanded** 2 281 701 376" / "asks the client for 2.3 GB" / "a 2.3 GB demand" | the client never demands more than **4 bytes**. `FUN_1406e8c20` tests `cmp edi, 4`; `FUN_140862470` has no `reserve` and no bulk read. There is no allocation of any size and no single large request anywhere in this failure |

This is not pedantry, and it is the same failure mode `CLAUDE.md` names under *the unit, not
the arithmetic*: "asks for 2.3 GB" predicts a memory spike, an allocation failure, or a
length check somewhere that could be made to reject it. None of those exist. What exists is
a `u32`-at-a-time walk that always dies three bytes from the end of whatever body it is
given — **which is exactly why 128 bytes of padding did not help, and why 1 280 would not
either.** Padding only buys 320 more iterations. **[D]**

---

## 7. The access violation is downstream of all of this

| | `970152` | `970748` |
|---|---|---|
| fault address | `0x140ce89f7` = `FUN_140ce89c0+0x37` | `0x140ce89d6` = `FUN_140ce89c0+0x16` |
| access | **write** (param[0] = 1) | **read** (param[0] = 0) |
| target | `0x660822`, COMMIT MAPPED **READONLY** | `0x8f0998`, **RESERVE** PRIVATE (uncommitted) |
| `rbx` | `0x6607fa` | `0x8f0970` |
| target − `rbx` | **`0x28`** | **`0x28`** |

**[L]** The two fault addresses differ, and that difference is not two bugs:

```asm
140ce89c0  mov  rdi, rcx
140ce89cd  mov  rbx, [rcx+8]        ; the referent
140ce89d1  test rbx, rbx
140ce89d4  je   140ce8a26           ; null -> nothing to release
140ce89d6  mov  rdx, [rbx+0x28]     ; <<< 970748 faulted here (read)
140ce89de  cmp  rax, 0xffffe        ; sanity range on the refcount
140ce89f0  mov  rax, -1
140ce89f7  lock xadd [rbx+0x28], rax ; <<< 970152 faulted here (write)
```

**Same field, `+0x28`, in both.** `970152` happened to read a value inside the sanity range
and died on the interlocked write two instructions later. This is a **refcount release**
(`InterlockedDecrement`), reached from an unwind funclet:

```asm
14308e3db  push rbp / sub rsp,0x20 / mov rbp, rdx
14308e3e4  lea  rcx, [rbp+0x50]     ; rdx = establisher frame
14308e3e8  call 140ce89c0
```

The establisher frame handed to it is **`0x14e6b0`** (the fault context's `rbp`/`rdx`), which
§5's chain shows is **frame 07** — six frames out from the throw. So by the time this ran the
unwinder had already left `FUN_140862470` and `FUN_140a46e50` entirely. The pointer being
released is the local at `0x14e700`:

```text
0x14e700  08 56 40 06 ...   (970152)   |  c8 ad 41 06 ...   (970748)
0x14e708  fa 07 66 00 ...   = 0x6607fa |  70 09 8f 00 ...   = 0x8f0970
```

**[L]** Non-zero — so the null guard passed — and not a valid object in either process
(`0x6607fa` is not even 16-aligned and lands in a read-only file mapping; `0x8f0970` lands in
reserved, uncommitted address space). **[D]** The referent slot was never set to anything
real, which is what the brief's working theory predicted; whether that is uninitialised stack
or a half-written assignment is **[I]** and this file does not settle it.

Frame 07's function cannot be named: `0x144ad9fc8` disassembles into garbage from its own
`.pdata` start, i.e. it is in the Themida region. **[L]** Nothing above rests on naming it.

**The practical consequence:** the client cannot report its own packet underflow, because it
dies during the unwind of it. The `0xE06D7363` exit code that `research/buffs-underflow.md`
saw for `0x007D` and `0x007E` is what this *should* have looked like.

---

## 8. Two instrument notes, both near-misses

* **`dumpwalk.py --scan` defaults to 0x4000 bytes above `rsp` and does not reach any of
  this.** The live throw frame is at `0x14d488`, just under 20 KB above the faulting `rsp` of
  `0x1485f0`. The default window shows eleven copies of `0x1406e8cb1` from the exception
  dispatcher's saved contexts and **none of them is the live one**. Scan the whole committed
  stack (`0x145338..0x150000` here) or the answer is confidently wrong.
* **One coincidence nearly went into this file as a finding.** `[0x14e6b0 + 0x190]` holds
  `0x14e920`, which matches `FUN_140a46e50`'s `mov rdx,[rsp+0x190]` exactly and looked like
  proof that the unwinding frame was the decoder itself. It is not: enumerating every stack
  slot holding `0x14e920` gives **30** of them, so a hit at any particular offset is worth
  nothing. `0x140a4a113` occurs **once**, at `0x14d518`, which puts `FUN_140a46e50`'s `rsp`
  at `0x14d520` — 4.5 KB below `0x14e6b0`. Enumerating first is what caught it.

---

## 9. What would refute this

* A dump in which the reader object at `0x14e920` shows a position that is **not** three
  short of its length. Two dumps agreeing is a measurement; a third disagreeing would mean
  the object was misidentified.
* A count-solve returning **zero or two** solutions. It returned exactly one in each dump,
  over ~170 candidate offsets.
* `467` failing to equal `4N + 3` for the `N` recovered from `rdi`. It holds for both bodies
  with the same `N`, and the two halves come from different places — `467` from the fixture,
  `N` from a saved register.
