# `0x0224`'s remote `CUser` faults on first use — body offset 41 must be `-1`, not `0`

**2026-09-03. Two full-memory crash dumps, no client run, no Ghidra** (the project lock was
held elsewhere). Instruments: `tools/reads.py`, `tools/listing.py`, `tools/callers.py`,
`tools/pdata_lookup.py`, `tools/dump_va.py`, `tools/rangescan.py`, `tools/dumpwalk.py`
(including its `MiniDump` reader, driven from four scratch probes whose logic is quoted
below), all run with the repo as the working directory.

Tags: **[L]** read off a listing, a register, a dump or a log; **[D]** derived from two or
more [L]; **[I]** inferred.

Companion files, none of them edited here: `research/0x0224-body-walk.md` and
`research/avatar-look-loops.md` (the `REMOTE_STAT_TAIL_LEN` 7→23 fix, now confirmed working)
and `research/0x0224-dump-read-position.md` (the technique this file reuses).
`crates/` was not touched.

---

## 0. The answer

**Body offset 41 — the `u32` this builder calls `fame` — is an index into a per-map array,
and the client's "no entry" value for it is `-1`. We send `0`.**

```text
  1429ce483  call 0x1406e8c20   READ u32          <- body offset 41
  1429ce488  mov  [r14+0x3b28], eax               <- straight into CUser+0x3b28, no transform
```

`CUser+0x3b28` is returned by the virtual at vtable slot `+0x18`
(`0x142834190  mov eax,[rcx+0x3b28] / ret`). **All three call sites of the array accessor in
this binary gate on `cmp eax, -1`** and skip the whole lookup when it is `-1`. With `0` the
client takes the lookup, the array is empty, and the accessor **reports the out-of-range
index and then returns the out-of-range address anyway**:

```text
  14182a14a  mov  rdi,[rcx+0xa8]        ; rcx = the current field  -> the field-info object
  14182a158  mov  r8,[rdi+0x178]        ; the array base           -> 0 (empty)
  14182a15f  test r8,r8 / je            ; base null -> count stays 0
  14182a16c  cmp  ebx,ecx / jb ok       ; 0 < 0 is false           -> falls into the report
  14182a17e  mov  ecx,0xbc / call 0x142e54290    ; writes _ms_report.ini and RETURNS
  14182a18f  lea  rax,[rbx+rbx*2] / shl rax,4 / add rax,8 / add rax,r8
                                        ; 48*0 + 8 + 0 = 8
  140f9295e  mov  rcx,[rax]             ; <<< READ AT ADDRESS 8
```

`param[1]` of the `EXCEPTION_RECORD` is `0x0000000000000008` and `Rax` is `0x8`, in **both**
dumps. **[L]**

**The change is one line in `crates/net/src/userpool.rs`: `w.u32(0)` at body offset 41
becomes `w.i32(-1)`.** Nothing else in the body moves — same width, same offset, same length.

---

## 1. The instruments, and the controls each passed first

* `python tools/reads.py 0x140304100 2` printed its documented control — the helper reads
  through `FUN_1403035a0` / `FUN_140303b40` and the direct reads at `140304138 raw`,
  `140304144 u8`, `140304183 u8`, then a run of `u16`. Run **before** anything else. **[L]**
  `tools/listing.py` shares that loader and extent by construction.
* `tools/dumpwalk.py` prints its own six self-checks (`MDMP` magic, stream directory, module
  list, image base, **pid 1003900 matching the filename**, `Memory64List` covering the file).
  All six PASS on both dumps. **[L]**
* The dump object search used in §4 was verified by the shape of its own answer, not by
  assumption: scanning both processes for the avatar-look vptr returns **three** slots each,
  of which two decode as `CUser`s carrying **different** faces and hairs (§4.2). A broken
  scan returns none or thousands.
* The hook-log sweep in §6 deduplicated **by content hash** before counting, per `CLAUDE.md`.

### One instrument that was a false lead, and how it was caught

`tools/rangescan.py 0x3b28` over the whole of `.text` returns 29 sites. Two of them,
`0x140c4194d  mov [rbx+0x3b28],eax` and `0x140c33dc0  mov edx,[rcx+0x3b28]`, look exactly
like a setter and a getter for this field. They are **not**: the writer's neighbourhood is

```text
  140c4192b  mov [rbx+0x3b20], eax
  140c41934  xor eax, 0xbaadf00d
  140c4193c  ror r8d, 5
  140c4194d  mov [rbx+0x3b28], eax      <- a CHECKSUM word of a protected-int triple
```

— a different class entirely, whose `+0x3b20/+0x3b24/+0x3b28` are one obfuscated value. Read
as a CUser accessor it would have named the wrong thing with a straight face. **[L]**

---

## 2. The faulting instruction, and where its register came from

`python tools/pdata_lookup.py 0x140f9295e` → `fn 0x140f8d910 .. 0x140f94c46`, offset `+0x504e`.
`tools/listing.py 0x140f8d910 0x140f9295e` marks it:

```text
 000140f928ef  mov  rax,[r15]
 000140f928f2  mov  rcx, r15
 000140f928f5  call qword ptr [rax + 0x20]   ; vf20 = 0x1429e1710 = `return [this+0x3f80]`
 000140f928fe  test eax, eax
 000140f92900  je   0x140f92931              ; 0 -> the array branch (this is the one taken)
 ...
>000140f92931  call qword ptr [rdx + 0x18]   ; vf18 = 0x142834190 = `return [this+0x3b28]`
 000140f92934  cmp  eax, -1
 000140f92937  je   0x140f92980              ; <<< -1 SKIPS EVERYTHING BELOW
 000140f92939  call 0x141892840              ; the current field  (named in research/item-drop.md)
 000140f9293e  test rax, rax
 000140f92941  je   0x140f92980
 000140f92943  call 0x141892840
 000140f92948  mov  rdi, rax                 ; rdi = the field
 000140f92951  call qword ptr [rdx + 0x18]   ; the index again
 000140f92954  mov  edx, eax
 000140f92956  mov  rcx, rdi
 000140f92959  call 0x14182a140              ; &field->info->array[idx] + 8
 000140f9295e  mov  rcx, qword ptr [rax]     ; <<< MARKED ADDRESS - the fault
 000140f92961  mov  qword ptr [rbp + 8], rcx
 ...
 000140f92979  mov  esi, ecx                 ; low dword used as an X coordinate
```

**[L]** for every line. The decode was re-derived from raw bytes with no disassembler in the
path first (`tools/dump_va.py 0x140f92920 96` → `... 48 8b 08 ...` at `0x140f9295e` =
`mov rcx,[rax]`), because one run of one disassembler is one opinion.

`FUN_140f8d910` is **avatar action/animation**, not packet handling. Its only string
references are

```text
  140f8fbca  "%d : Invalid Action( Frame Count : %d, riding : %d, bSit : %d )"
  140f927db  "%d : Invalid Action2( Frame Count : %d, riding : %d, bSit : %d )"
  140f92e48  "scale"        140f8dec1  "itLoad"
```

**[L]**, and the sibling `FUN_140f83ac0` (the second of the three accessor call sites) carries
the WZ property name `"fixFrameIdx"`. *The `Invalid Action2` block ends at `0x140f928ba` with
`jmp 0x140f928bf`, and the healthy path enters at `0x140f928bc`; both zero `r14d` and
converge. So the report is **not** on the fault path — it is orientation only, and saying the
fault is "downstream of an invalid action" would be wrong.* **[L]**

### The value it dereferenced

`tools/listing.py 0x14182a140` is 105 bytes and reads, in full:

```text
  rdi = [rcx+0xa8]                 ; the field-info object
  rbx = (i64)edx                   ; the index, SIGN-extended
  r8  = [rdi+0x178]                ; the array base
  ecx = (r8 ? [r8-8] : 0)          ; the element count, ZArray-style at base-8
  if (edx >= 0 && (u32)rbx < ecx)  goto ok
      call 0x142e54290(0xbc, index, count)     ; report, then FALL THROUGH
      r8 = [rdi+0x178]                          ; reload - still null
  ok: return r8 + 48*index + 8
```

**[L]** `0x142e54290` is a reporter, not a guard. Its five `lea rip`-relative operands
resolve to `"Hash"`, `"Line"`, `"Info1"`, `"info2"`, `"LogCallStack5"` — ini key names in the
same `.rdata` pool as `"_ms_report.ini"` and `"_ms_callstack.ini"` **[L]**, so it is the
client's own error-log writer **[D]**. What matters is structural and is **[L]** from the
listing: it gates on `0x142e559e0`, formats through `0x14019ba10`, and its only exit is
`add rsp,0x68 / ret`. **There is no throw and no `__fastfail` in it.** So an out-of-range
index here is not caught — it is logged and then honoured.

`48*0 + 8 + 0 = 8`, and `R8 = 0` in the recorded context because `r8` is untouched between
`add rax, r8` and the fault. **[D]**

---

## 3. The two dumps agree on every register that should agree

`dumps\maplecw-crash-1003900-c0000005-1.dmp` (1.46 GB) and
`dumps\maplecw-crash-990836-c0000005-1.dmp` (1.26 GB), two different processes 0.9 s apart:

| | `1003900` | `990836` |
|---|---|---|
| code / address | `0xC0000005` at `0x140f9295e` | identical |
| `param[0]` (read/write) | **0 = read** | identical |
| `param[1]` (target) | **`0x8`** | identical |
| `Rax` | `0x8` | identical |
| `R8` (array base) | `0x0` | identical |
| `Rbx` | `0xffffffcefffffff8` | identical |
| `Rsp` / `Rbp` | `0x14b6e0` / `0x14b830` | identical |
| `R15` (the `CUser`) | `0x37d913f8` | `0x388b7248` |
| `Rdi` (the field) | `0x385b5ef8` | `0x4488c7d8` |

and the `.pdata`-driven unwind is **frame for frame the same in both**:

```text
  00  0x140f9295e  fn 0x140f8d910+0x504e     <- the fault
  01  0x1427cdf7d  fn 0x1427cddf0+0x18d
  02  0x140f8117c  fn 0x140f81103+0x79
  03  0x14276eed6  fn 0x14276ecf0+0x1e6
  04  0x140f80561  fn 0x140f80200+0x361
  05  0x1429dbe7f  fn 0x1429dbe30+0x4f
  06  0x1429b85ee  fn 0x1429b79e0+0xc0e
  07  0x142ce0328  fn 0x142ce0130+0x1f8
  08  0x14181d650+0x50 ... 12  0x142ef4aea  13 kernel32!BaseThreadInitThunk  14 ntdll!RtlUserThreadStart
```

**[L]** `CField::OnPacket` (`FUN_141820080`, frame 06 of the *previous* failure per
`research/0x0224-dump-read-position.md` §5) **is not on this stack**. The fault is on the
per-frame update, not inside the handler — which is also what the hook log says: the handler
returned at 21:35:36.215 (`ret=1`, 1891 µs) and the fault is at 21:35:36.373, **158 ms
later**. **[D]**

### The hook log's own stack is NOT this stack

`0x142f0f0dd / 0x142f0cf52 / 0x142f0aa5d / 0x142f0fac3` — the four addresses in the
`CLIENT FAULT` line — appear in `dumpwalk`'s **stack scan** at `0x14b878..0x14b978`, which is
*above* the faulting `rsp` of `0x14b6e0`, and **none of them is in the `.pdata` unwind**.
Each does resolve to a real function, and `0x142f0fac3` and `0x142f0aa5d` each appear twice,
so they are almost certainly a live exception-dispatch chain — but they are **residue with
respect to the frame that faulted**. Use the unwind above. **[L]**

*(That is not a criticism of the hook: it prints raw stack words that look like code
addresses, and says so.)*

---

## 4. `R15` is the remote `CUser` our packet built, and the control is in the same process

### 4.1 The vtable is unique

Scanning every byte of both `Memory64List`s for the qword `0x143486d88` (the vtable at
`[r15]`) returns **exactly one 8-aligned hit per process**, and it is `r15` itself. **[L]**
So this class — `CUserRemote`, as against the local user's `0x143483400` — has exactly one
live instance, which is what "one other player on the map" means.

### 4.2 Its avatar look is the one we sent, and it differs between the two processes

The look struct sits at `CUser+0x30`; `research/avatar-look-loops.md` §1 has the client
writing equips at `look[0x39 + slot*4]`, and `0x30 + 0x39 = 0x69` reproduces exactly:

```text
                          face      hair      look[0x39+4*slot] for slots 1,5,6,7,11
  1003900  r15+0x59/0x69  20002     30025     1002996 1042999 1062999 1072999 1322999
  990836   r15+0x59/0x69  20001     30032     (Tester2's five, same stride)
```

**[L]** Cobalt in one process, Tester2 in the other — **each client holds the counterpart's
character**, which is the same positive control `research/0x0224-dump-read-position.md` §1
used, arrived at by a different route. This is the packet's object.

### 4.3 The discriminator: the local user in the same process, at the same instant

Scanning for the avatar-look vptr `0x14327d968` finds **three** slots per dump; two of them
are `CUser`s:

| dump | object | vtable | face/hair | **`+0x3b28`** |
|---|---|---|---|---|
| `1003900` | `0x32f8bd78` | `0x143483400` (local) | 20001 / 30032 | **`0xFFFFFFFF` = −1** |
| `1003900` | `0x37d913f8` | `0x143486d88` (remote) | 20002 / 30025 | **`0`** |
| `990836` | `0x37cc20e8` | `0x143483400` (local) | 20002 / 30025 | **`0xFFFFFFFF` = −1** |
| `990836` | `0x388b7248` | `0x143486d88` (remote) | 20001 / 30032 | **`0`** |

**[L]** Same process, same map, same instant: the client's own user carries **−1** and the
one our packet built carries **0**. That is the control `CLAUDE.md` asks for — close to the
subject, and capable of coming back the other way.

### 4.4 Enumerated, not searched for

Comparing the local and remote `CUser`s dword by dword over **`0x4400` bytes**, in **both**
processes, and keeping only offsets where the local holds `0xFFFFFFFF` and the remote holds
`0`:

```text
  1003900   1 offset      990836   1 offset      in both:  +0x3b28
```

**One offset in 4 352, in two processes.** **[L]** Widening the filter to *any* offset where
the local holds the **same nonzero value in both processes** while the remote holds `0` gives
nine:

```text
  +0x0200 0xffff0000   +0x03e4 0xffffff00   +0x0e5c 1   +0x3b28 0xffffffff
  +0x3ed0 1   +0x41cc 1   +0x42f8 1   +0x4328 1   +0x43a0 1
```

and **`+0x3b28` is the only one of the nine that `FUN_1429ce270` writes at all** — the other
eight never appear as a destination in `research/msexe-userpool-userinit-1429ce270.txt`, so
they are `CUserLocal`-vs-`CUserRemote` class differences, not absent-form errors in our body.
**[L]** This is the enumeration `CLAUDE.md` asks for before filtering: the sweep could have
returned a list of fields to fix, and it returned one.

---

## 5. What the field is, and what is only a guess

**Measured [L]:**

* `CUser+0x3b28` ← body offset 41, `u32`, read at `0x1429ce483`, stored raw at `0x1429ce488`.
* It is the return value of vtable slot `+0x18`.
* It indexes `[[currentField + 0xa8] + 0x178]`, elements **48 bytes**, and what is read is
  the qword at **element+8**, which the sibling at `0x140f869ce` immediately treats as a
  `POINT`: `mov eax,[rbp+0x30]` then `neg eax` under a facing flag — the **x** of a
  coordinate pair.
* `[currentField + 0xa8]` is the field-info object already named in
  `research/quest-complete-effect.md:215`, `research/recovery-number.md:477` and
  `research/transfer-field-request.md:135` — and `transfer-field-request` reaches it through
  **the same type descriptor `0x143a87ec8`**, which is an independent confirmation of the
  `0x141892840` chain.
* `-1` is the client's "none": **three of three** call sites of `0x14182a140`
  (`0x140f869c9`, `0x140f92959`, `0x140f9745b` — `tools/callers.py` reports 3 calls, 0 tail
  jumps, 0 data pointers) are immediately preceded by `cmp eax,-1 / je <skip>`.

**Inferred [I], and not needed for the fix:** that the array is the map's **seat / chair
list** — a per-map table of positions, empty on map 40, with the user's seat index `-1` when
not sitting. The supporting circumstantial facts are that the enclosing function's own error
string carries `bSit`, and that `vf20` (`[CUser+0x3f80]`) selects a *different* anchor source
(`[CUser+0x3b18]`, non-null in both dumps) when it is set. **No listing in this pass proves
the name**, and the fix does not depend on it.

**Explicitly not settled:** what fills that array. If a map with a non-empty table were used,
index `0` would silently resolve to a real entry and pin every remote avatar to it rather
than crashing — so this bug is **map-dependent in its symptom and not in its cause**. That is
an argument for `-1` regardless of the name.

### One loose end found in passing, offered as a lead only

`research/user-enter-field.md` row 416 maps that `i16` to `CUser+0x3c28`. In both dumps
`+0x3c28` reads **100** for the local user *and* **100** for the remote, while the builder
writes `0` there. Either the row is wrong or something overwrites it after `Init`. Not
touched here, and it is not on the fault path.

---

## 6. Is it this packet, or is it this session?

`CLAUDE.md`'s discriminator, run first. Sweeping every `*hook*.log` in `previous-runs/` and
`research/fixtures/` — **128 files, 100 distinct by content hash**:

```text
  logs containing 0x140f9295e             : 1
  logs containing an `opcode=0x0224` line : 1
  logs containing both                    : 1   <- this run
```

**[L]** The fault address appears in **1 of 100** distinct hook logs, and it is the only run
in the archive where a `0x0224` **dispatch line exists at all** — which is itself consistent,
because the dispatch line is written on *return* and the two earlier `0x0224` runs died
inside the handler. So the address is new, it appeared exactly when the handler started
completing, and both clients hit it. **[D]**

What this does *not* rule out: this is still **two observations from one run**. Two processes
is better than one, and the byte-identical registers make coincidence implausible, but a
second run is the thing that would make it a measurement.

---

## 7. The change, in `crates/net/src/userpool.rs` (not applied)

One line, in `user_enter_field`:

```rust
    w.u8(chr.gender); //          40
    // **41 -> CUser+0x3b28, and it is NOT fame.** `1429ce483` reads it and `1429ce488`
    // stores it raw; the virtual at vtable+0x18 (`0x142834190 mov eax,[rcx+0x3b28]`)
    // hands it to `0x14182a140`, which indexes `[[field+0xa8]+0x178]` - a per-map array
    // of 48-byte entries. All THREE call sites of that accessor gate on `cmp eax,-1`, so
    // -1 is the client's "no entry". Zero is a live index: on a map whose array is empty
    // the accessor reports the violation, returns `0 + 48*0 + 8`, and `0x140f9295e
    // mov rcx,[rax]` reads address 8. Measured in two crash dumps, where the client's own
    // local CUser holds -1 at this offset and the one this builder made holds 0.
    // research/0x0224-remote-user-first-use-fault.md
    w.i32(-1); //                 41
    w.u32(0); //                  45  name-tag mark
```

`i32` exists on `PacketWriter` (`crates/net/src/packet.rs:72`). **No offset moves**: same
width, same position, `USER_ENTER_FIELD_MIN_LEN` unchanged at 531.

### Tests

Nothing existing asserts body offset 41, so nothing breaks. What *should* be added, and the
reason is `CLAUDE.md`'s "a test that pins what the code already does is not a check": assert
the four bytes at `41 + name.len()` are `0xFF 0xFF 0xFF 0xFF`, **anchored** the way
`there_is_no_miniroom_and_no_chair` now is — the gender byte at `40 + name.len()` is the
nearest field with a value the test chooses, so a wrong anchor fails loudly instead of the
zero-check passing on nothing.

```rust
    let body = user_enter_field(&someone("Wanderer", &[]), RemoteAt::default());
    let p = 40 + "Wanderer".len();
    assert_eq!(body[p], 0, "the gender byte anchors the offset below it");
    assert_eq!(
        i32::from_le_bytes(body[p + 1..p + 5].try_into().unwrap()),
        -1,
        "CUser+0x3b28 is a field-array index; 0 is a live index into an empty array"
    );
```

### Keep `USER_ENTER_FIELD_PAD_LEN` for one more run

Same reasoning as last time and it has now paid once: if a *second* wrong absent form exists,
the pad is what keeps the reader inside the buffer long enough to see it. Remove it after a
launch that does not die.

### The watch worth arming

`0x14182a140` (the accessor) with `0x140304100:hits=200` as the positive control. If it is
**never entered**, the index reached the client as `-1` and the guard at `0x140f92937` did
its job. If it is entered, print `edx`: that is the index the client believes, and it says
whether the byte landed where we think it does.

---

## 8. What would refute this

* A dump in which the local `CUser`'s `+0x3b28` is **not** `-1`. Two processes agree; a third
  disagreeing would mean `-1` is this session's accident rather than the client's convention.
* `tools/callers.py 0x14182a140` returning a fourth call site that is **not** `-1`-gated.
  Three of three today, with zero tail jumps and zero data pointers, so an indirect call
  would be the way this is wrong.
* The next run dying at `0x140f9295e` again. That would mean the byte did not land at offset
  41 — and the accessor watch above says so directly, without another dump.
* The next run dying somewhere *else* on the update tick. That is the outcome the pad is kept
  for, and it would mean this file found one of two, not one of one.

## 9. Named blind spots

1. **`CUserLocal` and `CUserRemote` are different classes**, so §4.4's dword diff assumes a
   shared `CUser` base layout across `0x4400` bytes. The look struct, face, hair and all five
   equips land at identical offsets in both, which is evidence for it, not proof.
2. **The diff only catches `-1` vs `0` and "same nonzero vs 0".** An absent form that should
   be some other value the local user happens not to hold — a per-character number, or one
   the local user also has as `0` — is invisible to it.
3. **The array is not named.** §5 says why, and the fix does not need it. Anyone who names it
   should say what `[field+0xa8]+0x178` is filled from; nothing in this pass opened that.
4. **`0x140f8d910` was not walked end to end.** It is 29 494 bytes and only the ~200 bytes
   around the fault, plus its five string references, were read. Another packet-fed field
   could be misused elsewhere in it.
5. **`0x142e54290`'s gate at `0x142e559e0` was not opened**, so whether the bounds violation
   was actually written anywhere on this run is unknown, and which file it would land in is
   **[D]** from a string pool rather than **[L]** from an operand. It does not matter to the
   finding — the return path is unconditional — but if an `_ms_report.ini` or
   `_ms_callstack.ini` exists beside the client it is worth one look, and the array id the
   call passes is `0xbc` = 188.
6. **Nothing here has been on the wire.** No `0x0224` carrying `-1` at offset 41 has ever
   been sent.

---
---

# Part 2 — the `-1` shipped, the fault did not move, and Part 1 named the wrong field

**2026-09-03, later the same day. Two more full-memory dumps, no client run of my own, no
Ghidra.** `dumps\maplecw-crash-992840-c0000005-1.dmp` and
`dumps\maplecw-crash-1010344-c0000005-1.dmp`; logs in
`research/fixtures/offset41-minus-one-same-fault-0x140f9295e-hook.log` and its `-world.log`.

`crates/` was not touched.

## P2.0 The three questions, answered

* **Is `RAX` still 8?** **Yes**, in both new dumps. **[L]**
* **Is the index still 0?** **Yes** — and it is forced by arithmetic before any object is
  read: the accessor returns `r8 + 48*idx + 8`, and `RAX = 8` with `R8 = 0` admits only
  `idx = 0`. An index of `-1` would have produced `0xFFFFFFFFFFFFFFD8`. **[D]**
* **Same instruction, or merely the same address?** **The same instruction with the same
  register state.** Every register matches the two old dumps except the three that hold heap
  objects:

```text
                 old 1003900 / 990836        new 992840 / 1010344
  Rip            0x140f9295e                 0x140f9295e     same
  Rax            0x8                         0x8             same
  R8             0x0                         0x0             same
  Rbx            0xffffffcefffffff8          same            same
  Rsp / Rbp      0x14b6e0 / 0x14b830         same            same
  R9,R11,R12,R13,R14                         all same
  param[0]/[1]   0 (read) / 0x8              same            same
  unwind         14 frames                   frame-for-frame identical
  Rsi/Rdi/R15    heap pointers               different (different processes)
```

**[L]** So this is not a fourth call site, not a different register, and not a different
address arriving at the same instruction. It is the *same* failure, unchanged.

**And Part 1's static enumeration is not the thing that was wrong.** The three `cmp eax,-1`
gates are real, the accessor is real, `-1` is the sentinel. **What was wrong is which byte of
`0x0224` feeds the field.**

## P2.1 The mistake: `r15` is not `param_1`. It is `param_1 + 0x100`.

The instrument that found it is the one that did not exist last time — **the old dumps
against the new ones.** Diffing the remote `CUser` of the *same character* across the two
runs, over `0x4400` bytes, keeping only dwords that went `0 -> 0xFFFFFFFF`, which is exactly
what changing body offset 41 from `0` to `-1` should do:

```text
  Cobalt  (face 20002/hair 30025)   309 of 4352 dwords differ    0 -> FFFFFFFF at: +0x3a28
  Tester2 (face 20001/hair 30032)   742 of 4352 dwords differ    0 -> FFFFFFFF at: +0x3a28
```

**[L]** One offset each, the same one, and it is **`0x100` below `+0x3b28`**.

`FUN_1429ce270`'s store was read correctly — raw bytes at `0x1429ce488` are
`41 89 86 28 3b 00 00` = `mov [r14+0x3b28], eax`, and `0x1429ce2ab mov r14, rcx` makes `r14`
the incoming `this`. **[L]** So `r14 + 0x3b28 == r15 + 0x3a28`, and therefore

> **`r15 = param_1 + 0x100`.** `r15` is a **base-subobject pointer**, `0x100` bytes into the
> `CUser`. `[param_1] = 0x143486b70` is the primary vtable; `[param_1+0x100] = 0x143486d88`
> is the second one, and that is the one Part 1 read.

Tested against fields whose values I can predict, at **both** candidate bases:

| `research/user-enter-field.md` row | at `r15 + off` | at `param_1 + off` |
|---|---|---|
| body 12 → `+0x406c` level | 0, 0 | **18** (Cobalt), **8** (Tester2) |
| body 179 → `+0x40c0` job | 0, 0 | **200** (Cobalt), **0** (Tester2) |
| body 41 → `+0x3b28` | 0 in all four dumps | **`-1` in the NEW, `0` in the OLD** |
| body 416 → `+0x3c28` | 100 in all four dumps | **`0` in all four** |

**[L]** Level and job are character-specific, differ between the two characters, and are
identical for the same character across runs — a table that could have come back all-zero and
did not. The `+0x3b28` row tracks the one thing we changed. `param_1` is the right base and
`r15` is not.

**So the virtual `0x142834190 mov eax,[rcx+0x3b28]` — whose `this` is `r15` — reads
`param_1 + 0x3c28`, which `research/user-enter-field.md` already names: row 416.**

### What this failure actually was

Not a wrong number. **A right number read in one frame of reference and looked up in a table
indexed by another.** `+0x3b28` off `r15` and `+0x3b28` off `param_1` are different fields
`0x100` apart, and nothing in either listing says which base its column is written against.

This is `CLAUDE.md`'s *"the unit, not the arithmetic"* in a new costume, and it is worse than
a wrong unit because **the cross-check I ran agreed with me.** Part 1 §4.4's enumeration —
"one dword offset in 4 352, in two processes" — was **structurally correct and pointed at the
right field the whole time**: it compared local against remote at a consistently `r15`-based
offset, so its single hit *was* `param_1+0x3c28`. I then took that number to a
`param_1`-indexed table and read off the wrong row. The measurement was right; the label was
wrong; and the label is what shipped.

It also explains, exactly, the loose end Part 1 flagged and could not account for: *"body 416
maps to `+0x3c28` but both dumps read 100 there for local and remote."* `r15 + 0x3c28` is
`param_1 + 0x3d28` — a different field, and `100` was never body 416's value.

**Two corrections to Part 1, and nothing else in it moves:**

* §0, §5 and §7: *"`CUser+0x3b28` ← body offset 41"* is wrong. The store is
  `[param_1+0x3b28] ← body 41`, which is correct — but the **crashing** field is
  `param_1+0x3c28` ← **body offset 416**.
* §4.2's look measurements were `r15`-relative and stand as numbers; the look struct is at
  `param_1+0x130`, not `+0x30`. (It is decoded into a stack buffer at
  `1429ce6a2 lea rcx,[rbp+0x160]` and copied in afterwards, which is why no store names it.)

Everything else — the faulting instruction, the accessor's arithmetic, `RAX=8` implying an
empty array and index 0, the three `-1` gates, the enumeration method — survives unchanged.

## P2.2 The field, and why `-1` is certain this time

`param_1 + 0x3c28`, from **body offset 416**, read as a `u16` and **sign-extended**:

```text
  1429ce81e  call 0x1406e8ef0   READ u16 (the u16 thunk)
  1429ce823  0f bf c8           movsx ecx, ax          <<< SIGN-extended, not zero-extended
  1429ce826  41 89 8e 28 3c     mov [r14+0x3c28], ecx
```

**[L]**, from raw bytes *and* from disassembling the live image inside the dump. `movsx`
matters: `w.i16(-1)` writes `FF FF`, which sign-extends to `0xFFFFFFFF`. A zero-extending read
would have produced `65535` and this fix would have failed the same way twice.

Five independent facts say `-1` is the absent value, and two of them are literals in the
client's own code rather than inferences from a dump:

1. **`0x142769d49  mov dword ptr [rsi+0x3c28], 0xFFFFFFFF`** — the client's own initialiser
   writes the literal, between `mov [rsi+0x3c20], rax` and a `lea rdi,[rsi+0x3c30]` /
   zero-fill of `0x30` bytes. **[L]**
2. **`FUN_142834020`** is `bool IsSitting(CUser*)`:
   `return (m_0x3c18 && FUN_141716a90(m_0x3c18)) || m_0x3c28 != -1;` —
   `0x14283403e cmp dword ptr [rbx+0x3c28], -1`. **[L]** That is the `bSit` in the faulting
   function's own `"Invalid Action( ... bSit : %d )"` format string, which is the first thing
   in this whole investigation that ties the field to a name by something other than
   resemblance.
3. All **three** call sites of the accessor gate on `cmp eax,-1` (Part 1 §5). **[L]**
4. The client's own **local** `CUser` holds `0xFFFFFFFF` at `param_1+0x3c28` in **all four
   dumps**, while every one of our packet-built remotes holds `0`. **[L]**
5. `tools/rangescan.py 0x3c28` over the whole of `.text` returns **27 sites**, and the only
   two that are a CUser field write are `0x1429ce826` (the decoder) and `0x142769d49` (the
   `-1` initialiser). **Nothing overwrites it after `Init`.** **[L]**

### The enumeration, redone at the corrected base

Local vs remote, `0x4400` bytes, **both new dumps**, offsets where the local holds `-1` and
the remote holds `0`: **24**, of which **exactly one is written by `FUN_1429ce270` at all** —
`+0x3c28`. The other 23 form a regular ~`0xB0`-stride run from `+0x0f6c` to `+0x21cc` and are
never a destination in `research/msexe-userpool-userinit-1429ce270.txt`; they are
`CUserLocal`-vs-`CUserRemote` state, not absent-form errors in our body. **[L]**
*(The grep's positive control is that the same pattern **does** find `+0x3c28`, so the
negative for the other 23 means something.)*

### And the byte is where the arithmetic says, measured in the client's own memory

Searching `1010344`'s memory for Cobalt's body (`u32 id, u32 id, u32 0, u32 level, u16 6,
"Cobalt"` — a 24-byte needle) returns **one** copy, with `24 02` immediately before it:

```text
  body[ 47.. 51]  (builder comment 41,  u32) = ff ff ff ff   <- our new -1 DID reach the wire
  body[470..472]  (builder comment 416, i16) = 00 00         <- the field that crashes
  body[480..484]  (builder comment 426, x/y) = 00 00 00 00   <- anchor: world.log says (0,0)
```

**[L]** The `x,y` anchor is what makes the `470` arithmetic
(`comment + REMOTE_STAT_TAIL_LEN + name + 5*equips`) a measurement rather than a hope — a
wrong offset model puts the anchor somewhere that is not zero.
*(`992840` holds **0** copies of Tester2's body: its receive buffer had been recycled by fault
time. The search is not blind — it returns exactly one hit in the dump that still has the
buffer and zero in the one that does not, and the needle was rejected once already when I had
the two clients paired the wrong way round.)*

## P2.3 The change

**`crates/net/src/userpool.rs:598`**, one field, **no offset moves** (`i16` stays `i16`):

```rust
    // **416 is the map-seat index, and `0` is a live index.** Read as u16 at `1429ce81e`
    // and SIGN-extended (`1429ce823 movsx ecx,ax`) into `param_1+0x3c28`, which is what
    // `0x142834190` - vtable slot +0x18 on the base subobject at `param_1+0x100` - hands to
    // the bounds-checked accessor `0x14182a140`. All three of its call sites gate on
    // `cmp eax,-1`; the client's own initialiser writes the literal -1 here
    // (`0x142769d49`); and `FUN_142834020` reads it as `IsSitting`. With `0`, on a map whose
    // seat array is empty, the accessor returns `0 + 48*0 + 8` and `0x140f9295e
    // mov rcx,[rax]` reads address 8.
    // research/0x0224-remote-user-first-use-fault.md Part 2.
    w.i16(-1); //                 416  field seat - NOT SITTING
```

**And revert `crates/net/src/userpool.rs:482` to `w.u32(0)`.** The client's own local `CUser`
holds **`0`** at `param_1+0x3b28`, so `0` was right there all along, and the `-1` Part 1 asked
for is an unjustified change to a field nothing here has characterised. Reverting it is what
keeps this **one variant**: against the last-known baseline the next run then differs in
exactly one field, offset 416. Leaving both would change two.

### Test

Anchored the way `there_is_no_miniroom_and_no_chair` now is — the `u32` at comment-offset 412
is the nearest field the test controls, so a wrong anchor fails loudly instead of a zero check
passing on nothing:

```rust
    let body = user_enter_field(&someone("Wanderer", &[]), RemoteAt::default());
    let at = |zero_tail: usize| zero_tail + REMOTE_STAT_TAIL_LEN + "Wanderer".len();
    assert_eq!(&body[at(412)..at(416)], &[0, 0, 0, 0], "the anchor above the seat index");
    assert_eq!(
        i16::from_le_bytes(body[at(416)..at(418)].try_into().unwrap()),
        -1,
        "map-seat index: sign-extended at 1429ce823, and 0 is a live index into an empty array"
    );
```

### The watch worth arming, unchanged in purpose and now correctly aimed

`0x14182a140` with `0x140304100:hits=200` as the positive control.

* **Not entered** → the `-1` reached `param_1+0x3c28` and the gate at `0x140f92937` held.
  That is the pass.
* **Entered** → print `edx`. `0` means the byte still is not landing there and the offset
  model is wrong somewhere I have not looked; anything else means a third reading.

`FUN_142834020` is the cheaper alternative if only one watch can be armed: it is `IsSitting`,
it is 55 bytes, and it returns `1` for a character nobody has seated.

## P2.4 What would refute this

* The next run dying at `0x140f9295e` with `RAX = 8` again. That would mean body 416 is not
  `param_1+0x3c28` either, and the offset model — not the field — is what needs work. The
  accessor watch says so without another dump.
* `param_1 + 0x3c28` reading anything but `0xFFFFFFFF` in a dump taken after this ships. Five
  independent facts point at `-1`; a sixth disagreeing would break all of them at once.
* A dump in which the local `CUser`'s `param_1+0x3c28` is **not** `-1`. Four dumps agree.

## P2.5 Blind spots, including the one that caused this

1. **A struct-offset table is meaningless without the base pointer it is indexed against**,
   and neither `research/user-enter-field.md` nor
   `research/msexe-userpool-userinit-1429ce270.txt` states one. Its `→ CUser` column is `r14`
   = `FUN_1429ce270`'s `rcx`. Any other listing in `research/` whose `this` is the `+0x100`
   subobject is `0x100` out against it, in either direction, **and both will look plausible.**
   Nothing in this pass audited the rest of that table; only rows 12, 41, 179, 181 and 416
   have now been checked against a dump.
2. **The `0x100` shift itself is [D], not [L]** — derived from `r14+0x3b28 == r15+0x3a28` plus
   the level/job table. I did not find the `add rcx, 0x100` (or the vtable-thunk `sub`) that
   produces `r15`. Finding it would make the shift [L] and would also say *which* base class
   sits at `+0x100`.
3. **The seat naming is still [I]**, though much better supported than in Part 1:
   `FUN_142834020` is a two-term "is sitting" predicate over `m_0x3c18` and `m_0x3c28 != -1`,
   and the faulting function prints `bSit`. What fills the per-map array is still unopened.
4. **`0x1428341a0 mov [rcx+0x3c28], edx`** is a setter thunk whose `this` base I did not
   establish. If its `rcx` is the `+0x100` subobject it writes `param_1+0x3d28` and is
   irrelevant; if it is `param_1` it is a second writer of the seat index. `rangescan` shows
   it is never reached with a literal `-1`, so it cannot be what puts `-1` in the local user,
   but it is the one site of the 27 I cannot place.
5. **Still one run per variant.** Two processes each time, byte-identical registers, and the
   second run agreeing with the first — but no `0x0224` carrying `-1` at offset **416** has
   ever been sent.
