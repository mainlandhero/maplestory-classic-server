# The 180-second family is anti-cheat, the six readable writers are switched OFF, and the one that runs is virtualised

The owner, 2026-09-08: *"Are they doing anything important that affects whatever the client is doing?
Or are they simply anti-cheat behavior that we don't need?"*

**They are anti-cheat**, and this file names the module from its own strings. But the answer to
"can we NOP the write" is not the one the question expects, and the reason is worth more than the
answer: **the five (six) readable writers have never fired in any session we have a dump of**, and
**the tick that has actually been observed running has a Themida-virtualised body with no
instruction to NOP.**

Tags: **[L]** read off this client's listing, a crash dump or an archived hook log, **[D]** derived,
**[I]** inferred. No client run was spent on any of it.

---

## 0. What is new, in one table

| claim | status |
|---|---|
| the subsystem is cheat detection and telemetry | **[L]** — its own strings name three cheat tools |
| `FUN_140ca3130(opcode, &a, &b)` is a **report-to-server** packet builder | **[L]** |
| three detectors set the enable flags of three of the five writers, one-to-one | **[L]** |
| there is a **sixth** readable writer the earlier pass missed, and **twelve** more on the same and the 240 s clock whose bodies are virtualised | **[L]** |
| every readable writer's enable flag is **0** in four separate crash dumps | **[L]** |
| the one tick observed allocating, `FUN_140c93c80`, passes its gate in every dump and is **virtualised** | **[L]** |
| the overrun is what the code says: `&v[0] + K` with `K/4` far past a `n`-element array | **[L]** |
| the 240 s control in the previous file was weaker than it read | **[L]** — all three 240 s ticks are in this same module, just virtualised |

---

## 1. The object: it is a `ZArray<int>`, and nothing ever reads the field

`FUN_140ca61d0(pOut, n)` — 20 bytes of prologue, four merged `.pdata` entries [L]:

```
mov  qword ptr [rcx], 0                 ; out = null
test edx, edx / je ...                  ; n == 0 -> empty
lea  rcx, [rip + 0x2e306b0]             ; -> 0x143AD68A0, the pool context
lea  rdx, [rdi*4 + 8]                   ; n*4 + 8 bytes
call 0x14019b780                        ; pool allocate
add  rax, 8
mov  qword ptr [rbx], rax               ; out = buffer
mov  qword ptr [rax - 8], rdi           ; count at buffer[-8]
```

`0x143AD68A0` is **the same pool context the sentry watches** — `POOL SENTRY ARMED: pool ctx
0x143ad68a0` is in every armed hook log [L]. So these allocations come out of the pool the whole
investigation has been walking.

`FUN_140ca22a0(pArray, i)` bounds-checks `i` against `[data-8]`, calls the throw helper
`0x142e54290` with `ecx = 0xbc` on failure, and returns `data + i*4` [L]. `FUN_140ca6ea0` is
`mov rax, rcx; ret` — a `/OPT:ICF`-folded stateless-allocator ctor; its result goes into `r8` and
`FUN_140ca61d0` never reads `r8` [L]. So the source idiom is `ZArray<int> v(n);` and nothing more.

**What reads `+0x90`, `+0x94`, `+0xc0`, `+0xcc`, `+0xe4`, `+0x220`? Nothing does, and nothing can.**
The offsets are not fields of an object. They are distances from a heap buffer that is allocated at
the top of the tick and freed at the bottom, and between those two points the *only* thing that
happens is the load / modify / store. `v[0..n-1]` is never written and never read. Divide the
offsets by four and they are element indices in an array whose length the same function just set:

| function | array | writes | as an index | operation |
|---|---|---|---|---|
| `FUN_140c93530` | `ZArray<int>(5)` | `&v[0] + 0x90` | **v[36]** | `inc` |
| `FUN_140c936a0` | `ZArray<int>(5)` | `+ 0x94` | **v[37]** | `inc` |
| `FUN_140c93810` | `ZArray<int>(6)` | `+ 0xc0` | **v[48]** | `dec` |
| `FUN_140c93a50` | `ZArray<int>(7)` | `+ 0xcc` | **v[51]** | `add 2` |
| `FUN_140c93b70` | `ZArray<int>(6)` | `+ 0xe4` | **v[57]** | `dec` |
| `FUN_140c93930` | `ZArray<int>(6)` | `+ 0x220` | **v[136]** | `dec` |

`FUN_140c93a50` is a **sixth** member the previous pass did not have; it is also the one whose
`add eax, 2` explains the value `2` without appealing to two increments landing on one field.

### The control that says this is not how the client uses that array

`FUN_140ca22a0` has exactly seven callers [L]: the six above and `FUN_1423a4ee0`. The seventh does

```
lea  rcx, [rax + 0x330]      ; a vector that lives inside a real object
mov  edx, r14d               ; a real, computed index
call 0x140ca22a0
mov  edx, dword ptr [rax]    ; and it READS
```

A real index, no constant added, and a read. Three call sites, all the same shape. **The six pass
index 0 and then add a constant before writing** — so the difference is not a property of the
accessor, it is a property of these six functions.

## 2. The subsystem: it names three cheat programs in its own `.rdata`

The previous file recorded, as **[L]**, that no strings are referenced in the five functions, their
caller, or the surrounding 96 KB. The first half is right — none of the six references a string.
**The 96 KB claim is wrong**, and it is wrong in the way `CLAUDE.md` keeps warning about: the
instrument could not have produced a positive.

Two reasons a search over that neighbourhood comes back empty:

* **A linear sweep desyncs.** Disassembling 60 KB from an arbitrary start walks into jump tables
  and never resynchronises, so the `lea` at `0x140c9b823` is never decoded. `tools/clusterstrings.py`
  walks one `.pdata` function at a time from a real entry point.
* **The strings are spliced**, exactly as `docs/ghidra.md` says. `[ROYAL Connector].exe` is stored
  as `5b 0d 52 0d 09 4f 0d 59 41 09 4c 20 43 0d 6f 09 0a 6e ...` — CR and TAB through every other
  character. Any "is this printable ASCII" test rejects it at byte two.

Strip `0x09/0x0a/0x0d` and walk per function, and the cluster speaks [L]:

```
FUN_140c902a0   0x143369308  'Crc Fail Alert!!'
FUN_140c9b810   0x143369358  'Jin64.dll'
FUN_140c9b8e0   0x143369368  '[ROYAL Connector].exe'
FUN_140c9b9b0   0x143369390  'Royal.Secure.Runtime.dll'
```

and the `.rdata` around them holds `kernel32.dll`, `SetModuleHandle`, `AccountId`, `Advapi32.dll`,
`RegOpenKeyTransactedA`, and `SYSTEM\CurrentControlSet\Control\Session Manager\Memory
Management\PrefetchParameters` [L]. Registry probes, module-handle checks, a CRC alert, and the
names of MapleStory botting software. **This is cheat detection.**

### The report path, and how the whole module talks

`FUN_140ca3130(ecx = opcode, rdx = &u32, r8 = &u32)` is a packet builder [L]:

```
call 0x1406ed520   ; COutPacket(opcode)      <- edx = the caller's ecx
call 0x1406ede20   ; Encode(rdx, 4)
call 0x1406ede20   ; Encode(r8, 4)
call 0x1406ef4e0   ; SendPacket
call 0x1406ed610   ; ~COutPacket
```

It has 13 callers, all inside `0x140c93930 .. 0x140c9b9b0` [L], every one of them passing two
compile-time constants:

| caller | opcode | second u32 | what it is |
|---|---|---|---|
| `FUN_140c9b810` | `0x288` | `0x9b2` | reports `Jin64.dll` |
| `FUN_140c9b8e0` | `0x292` | `0x9f2` | reports `[ROYAL Connector].exe` |
| `FUN_140c9b9b0` | `0x29a` | `0xa32` | reports `Royal.Secure.Runtime.dll` |
| `FUN_140c93930` | `0x2a4` | `0xa6c` | a writer, reports once then writes |
| `FUN_140c93a50` | `0x2a9` | `0xa8b` | " |
| `FUN_140c93b70` | `0x2a0` | `0xaaa` | " |
| seven 45-byte stubs | `0x295 0x29d 0x2b0 0x2ba 0x2c0 0x2d0 0x2e2` | `0xb1b 0xb3f 0xb6a 0xdb1 0xd77 0xb94 0xbbe` | bare reports; **zero `.text` callers** |

The second field runs 2482, 2546, 2610, 2668, 2699, 2730, 2843, 2879, 2922, 2964, 3006, 3447, 3505 —
increasing, and in step-31 and step-64 runs that match the byte-for-byte-identical function
templates around them. **[I]** that is `__LINE__` of one generated source file. **[D]** whatever it
is, it is a per-site identifier in a report, not game data.

The seven bare stubs having zero callers is **not** evidence they are dead: their callers are in
`.themida`, which is zero bytes on disk. Named blind spot, and §4 shows the same thing happening
where it can be proved.

### The three detectors arm three of the writers, one-to-one

`tools/dataref.py` on the writers' enable flags [L]:

| enable flag | written by | writer it arms |
|---|---|---|
| `0x143AC7D00` = 1 at `0x140c9b87b` | the `Jin64.dll` detector | `FUN_140c93530` |
| `0x143AC7D10` = 1 at `0x140c9b94b` | the `[ROYAL Connector].exe` detector | `FUN_140c936a0` |
| `0x143AC7D20` = 1 at `0x140c9ba1b` | the `Royal.Secure.Runtime.dll` detector | `FUN_140c93810` |

and each detector, in the same basic block, also sets the writer's `tStart` to
`GetTickCount() + 0x2BF20` — **now plus 180 000 ms** [L]:

```
140c9b87b  mov  dword ptr [rip + ...], 1      ; -> 0x143AC7D00, the enable
140c9b885  mov  dword ptr [rsp + 0x24], 0x9b2
140c9b89f  mov  ecx, 0x288
140c9b8a4  call 0x140ca3130                   ; report it to the server
140c9b8a9  call qword ptr [rip + ...]         ; GetTickCount
140c9b8af  add  eax, 0x2bf20
140c9b8b4  mov  dword ptr [rip + ...], eax    ; -> 0x143AC7D08, the writer's tStart
```

and the writer it arms checks `now > tStart` before it writes [L]. So the design is: **detect a
cheat tool, tell the server, and 180 seconds later start corrupting the heap.** The delay is what
stops a player correlating the crash with the detection. That is a deliberate delayed-sabotage
response, not a bug — and it is the only reading under which six functions that allocate a buffer,
write past it and free it are *intended*.

## 3. Enumerate before you filter: there are 18 of these, not 5

Every timed task in this client goes through `FUN_1408fcaa0(last, interval, now)` —
`sub r8d,ecx; cmp r8d,edx; seta al; ret`, four instructions [L] — so "which tasks run on interval
N" is enumerable. `tools/tickscan.py` walks each `.pdata` function that loads the interval as an
immediate and classifies what it does when it fires:

```
; interval 180000 ms: 16 .text functions
  FUN_140c93530   alloc, oob w[0x90]      FUN_140c93c80   vm@0x140c93cd2
  FUN_140c936a0   alloc, oob w[0x94]      FUN_140c942a0   vm@0x140c9430c
  FUN_140c93810   alloc, oob w[0xc0]      FUN_140c94400   vm@0x140c9446c
  FUN_140c93930   report, alloc, w[0x220] FUN_140c94800   vm@0x140c94860
  FUN_140c93a50   report, alloc, w[0xcc]  FUN_140c949a0   vm@0x140c94a00
  FUN_140c93b70   report, alloc, w[0xe4]  FUN_140c94b70   vm@0x140c94bd0
                                          FUN_140c94d40   vm@0x140c94da0
  FUN_142e0f9e0   -   (the only one outside the module)
                                          FUN_140c94e40   vm@0x140c94eaf
                                          FUN_140c94fa0   vm@0x140c9500f
; interval 240000 ms: 3 .text functions - ALL THREE in this module, ALL THREE vm
  FUN_140c93d60   vm@0x140c93db2
  FUN_140c94560   vm@0x140c945cc
  FUN_140c946c0   vm@0x140c94720
; interval 60000 ms: 22 functions, 2 in this module, neither with the shape
```

**This retracts half of the previous file's control.** "240 s has 3 tick functions and 0 with this
shape" was read as *the shape is specific to 180 s*. It is not: all three 240 s ticks are in this
same anti-cheat module and none shows the shape **because all three are virtualised**. The number
0 was a property of Themida. What survives is the 60 s comparison, and what it actually says is
narrower and more useful: *the shape is specific to this module*, which runs on 180 s and 240 s.

The virtualised ones keep their gates in readable `.text` and jump into the VM only afterwards, so
the pattern is legible even where the body is not [L]:

```
140c93c88  cmp  dword ptr [rip+...], 0     ; -> 0x143AC7D44, LAST
           ...                             ; if (!LAST) LAST = now
140c93ca5  mov  ecx, dword ptr [rip+...]
140c93cab  call 0x1408fcaa0                ; elapsed(LAST, 180000, now)?
140c93cbf  mov  dword ptr [rip+...], eax   ; LAST := now
140c93cc5  cmp  dword ptr [rip + 0x2e34270], 2   ; -> 0x143AC7F3C, a level
140c93ccc  jl   skip
140c93cd2  jmp  0x14492d329                ; <<< into .themida - body gone
```

`0x14492d329` in a crash dump is a textbook Themida VM prologue — `pushfq`, spill `r9`, `movabs
rax, 0x5ff`, key arithmetic on `r13` [L]. There is no instruction there to patch.

## 4. It is `FUN_140c93c80` that runs, and the archived logs prove it

`tools/callers.py` cannot see `FUN_140c93c80` calling the ZArray ctor, because in `.text` that call
no longer exists. The runs can. Every armed hook log carries, on a 180.0 s cadence [L]:

```
19:46:51.408 WATCH #5: 0x140ca61d0 ENTERED on tid 322020 ... rdx=0x5 ... called-from=0x14491cafd
             stack: 0x14491cafd(vm) 0x142ce2030<-TEXT
```

and `0x142ce2030` is the instruction **after** `0x142ce202b: call 0x140c93c80`, inside
`FUN_142ce0130` [L]. So the stack reads, bottom-up: the field update → `FUN_140c93c80` → its
virtualised body → `FUN_140ca61d0(_, 5)`. `FUN_140c93c80` has exactly one call site and no tail
jumps or data pointers [L].

`FUN_142ce0130` references `Effect/BasicEff.img/FeverTime/start` and `enter`, and
`FUN_1428923e0` — which calls three of the readable writers — references
`AdminFixStat [Cri:%d] [CriDam:%d, Option:%d] [IgnoreTargetDEF:%d] [BDR:%d] [BuffTimeR: %d]` [L].
**[I]** these are the per-frame `Update` of the field and of the local user. The anti-cheat ticks
are hosted inside ordinary gameplay update loops, which is why they run at all.

### The allocation is the writer's own event, not a coincidence

Pairing every distinct pool-sentry finding with the nearest preceding ZArray allocation, over every
archived hook log, **deduplicated on `(timestamp, event)` rather than on the file** — `CLAUDE.md`'s
rule, and it matters here: 251 raw allocation lines are **125 distinct events**, 164 raw finding
lines are **96** [L]:

```
in runs where the allocation watch was armed: 58 of 68 findings paired
  gap  min 0.012 s   median 0.074 s   max 0.149 s
  three of the five runs pair 100% (24/24, 18/18, 2/2)
```

The sentry walks every 100 ms, so a gap that never exceeds 149 ms is *one walk*. The damage is
found on the first walk after the allocation, every time. **[D]** the write happens inside the same
tick as the allocation.

## 5. The gates: four crash dumps say the six readable writers have never fired

`tools/dumpwalk.py --raw` over `dumps/`, four dumps (`372984`, `419988`, `374940`, `345148`) [L]:

| writer | its gate | value in all four dumps | passes? |
|---|---|---|---|
| `FUN_140c93530` | `[0x143AC7D00] == 1` | **0** | no |
| `FUN_140c936a0` | `[0x143AC7D10] == 1` | **0** | no |
| `FUN_140c93810` | `[0x143AC7D20] == 1` | **0** | no |
| `FUN_140c93930` | `[0x143AC7D30] != 0` (byte) | **0** | no |
| `FUN_140c93a50` | `[0x143AC7D32] != 0` (byte) | **0** | no |
| `FUN_140c93b70` | `[0x143AC7D3C] != 0` (byte) | **0** | no |

and the second gate the first three share, `[0x143ADC538 + 8] >= 7 / 8 / 9`, reads a `std::list`
whose size is **1** [L]. Two independent closed gates.

Their `LAST` slots are live tick counts (`0x1023D48B`, `0x1023D299`, …) [L], so the ticks *are*
running and the gate is what rejects. **The six readable writers are switched off and have been in
every session we have a dump of. They are not what has been killing the client.**

The virtualised twelve, same method [L]:

| function | gate | dump value | passes |
|---|---|---|---|
| **`FUN_140c93c80`** (180 s) | `[0x143AC7F3C] >= 2` | **2** in three dumps | **YES** |
| **`FUN_140c93d60`** (240 s) | `[0x143AC7F70] >= 1` | **2** in four dumps | **YES** |
| `FUN_140c942a0` | `[0x143AC7F80] >= 2` | 0 | no |
| `FUN_140c94400` | `[0x143AC7F90] >= 2` | 0 | no |
| `FUN_140c94800` | `[0x143AC7D64] >= 2` | 0 | no |
| `FUN_140c949a0` | `[0x143AC7D74] >= 3` | 0 | no |
| `FUN_140c94b70` | `[0x143AC7D94] >= 3` | 0 | no |
| `FUN_140c94d40` | `[0x143AC7DB0] >= 2` | 0 | no |
| `FUN_140c94e40` | `[0x143AC7FE8] >= 2` | 0 | no |
| `FUN_140c94fa0` | `[0x143AC7FF0] >= 2` | 0 | no |
| `FUN_140c94560` (240 s) | `[0x143AC7FA4] >= 2` | 0 | no |
| `FUN_140c946c0` (240 s) | `[0x143AC7FDC] >= 3` | 0 | no |

**Two of eighteen are live, and the 180 s one is the one seen allocating.** `0x143AC7F3C` and its
neighbours (`0x143AC7F28 = 0xCC`, `0x143AC7F2C = 0x12CD84`, `0x143AC7F30 = 0x1FFA28AC`) are
byte-identical across three sessions [L], so **[I]** that block is configuration loaded at startup,
not a per-session detection count.

## 6. The mechanism: the overrun, and the arithmetic error in the negative that refuted it

For the six readable writers the question is closed by reading them: the base is `&v[0]` of an
allocation made four instructions earlier and the offset is a compile-time constant. **There is no
pointer that could be stale.** It is an overrun, by construction [L].

For the live, virtualised writer it is still open — but the negative that was used to rule the
overrun out has an off-by-eight. It tested `damaged − 0x90` for a pool header. `&v[0]` is
`body + 8`, so the buffer body is `damaged − K − 8`. Redone on the overnight death, where the write
landed at `0x3A2F9A8C` (the high dword of `_Right` in a `0x20`-class node at body `0x3A2F9A78`), and
bucket-1 bodies in that dump are `≡ 0x18 (mod 0x20)`:

| K | `damaged − K − 8` | aligned to a slot body? |
|---|---|---|
| `0x90` | `0x3A2F99F4` | no |
| `0x94` | `0x3A2F99F0` | no |
| `0xc0` | `0x3A2F99C4` | no |
| **`0xcc`** | **`0x3A2F99B8`** | **yes — six slots before the victim** |
| `0xe4` | `0x3A2F99A0` | no |
| `0x220` | `0x3A2F9864` | no |

One of six survives the alignment test, and it is eliminated on two other grounds: `FUN_140c93a50`
does `add eax, 2` where the damage is `-1`, and its enable byte is `0`. So the arithmetic is now
right and the readable six are still excluded — but by a *stronger* argument than before, and the
same test is now correctly armed for the next capture.

**Neither hypothesis is excluded for the live writer, and residues cannot separate them**: an
overrun at constant `K` from a constant-size allocation and a stale pointer at a constant field
offset into a recycled slot of the same class both predict a constant residue. What separates them
is **distance to an allocation that has just happened**.

### The test that can observe it, and it needs no guard page

The hook already breaks on `FUN_140ca61d0` and already writes client memory (the sentry's header
repair). Two additions, both small:

1. **Record the buffer, not just the arguments.** The current WATCH logs `rcx = 0x14d408`, which is
   the *stack local*, not the heap block. On return, `*(void**)rcx` is the buffer. Log it with the
   timestamp and `rdx`.
2. **Read the six candidate dwords before and after.** At the return of `FUN_140ca61d0`, snapshot
   `buf + 8 + K` for `K ∈ {0x90, 0x94, 0xc0, 0xcc, 0xe4, 0x220}` **plus two control offsets no
   sibling uses** (say `0x1c0` and `0x300`). Break again on `FUN_140369050` — the ZArray destructor,
   called at the bottom of the same tick with the same `rcx` — and compare.

The readings, written down before the run:

* **exactly one of the six changed, and neither control did** → the overrun is proved, and `K`
  names the offset the virtualised body uses. One line of log ends the question.
* **a control changed too** → the instrument is measuring ordinary heap churn between the two
  breakpoints, and nothing here is evidence. This is the control that makes the positive worth
  something; without it "a dword changed" says nothing about a pool that is being used by other
  threads.
* **nothing changed at any offset, but a sentry finding follows within 150 ms** → not an overrun of
  *this* buffer. The stale-pointer reading survives, and the next question is what the VM body
  dereferences.

This costs one flag on a run that is already planned, reserves no address space, and — unlike the
guard page — it discriminates between the two hypotheses instead of only catching the write.

## 7. Neutering: what to do, and the risk

**Do not plan on NOPping an instruction.** The writer that is live has no instruction in `.text`;
`FUN_140c93c80` ends at a `jmp` into the VM. The six that do have a NOP-able store are all switched
off and have never fired. Patching them changes nothing that is happening.

What *is* patchable, in decreasing order of how much I would trust it:

**(a) Close the gate from the hook — no code modification.** `FUN_140c93c80` returns immediately
unless `[0x143AC7F3C] >= 2`. Writing `0` there at startup skips the whole body, including the
allocation. This is a four-byte data write to a global the client itself set, of the same kind the
sentry's repair already performs; nothing in `.text` is touched, so no CRC over code can see it.
Do the same for `0x143AC7F70` (the 240 s `FUN_140c93d60`, needs `>= 1`) only if the first is not
enough. **It is also self-verifying**: if the 180 s `WATCH ... 0x140ca61d0 ... rdx=0x5` lines stop
*and* the sentry findings stop, the writer was in this family and is now off. If the allocations
stop and the findings continue, it was not, and a fortnight of guard-page work is aimed wrong.
That is worth knowing on its own.

**(b) Make the call site a no-op.** `0x142CE202B: call 0x140c93c80` is five bytes in `.text`.
Riskier than (a) for no extra benefit.

**(c) NOP the six readable stores.** Correct, harmless, and useless today. Worth doing only as
insurance if a future session ever trips a detector — the day one of `Jin64.dll`,
`[ROYAL Connector].exe` or `Royal.Secure.Runtime.dll` is *believed* present, three more writers
switch on at once.

### The risk, stated plainly

* **"We do not need it" does not mean "safe to remove".** This module reports to the server on
  thirteen distinct opcodes and our server answers none of them, which by `CLAUDE.md`'s oldest rule
  freezes the client's UI. **Measured, and it is not a problem today**: grepping every archived
  `world-*.log` and `login-*.log` for each of `0x0288 0x0292 0x029A 0x02A0 0x02A4 0x02A9 0x0295
  0x029D 0x02B0 0x02BA 0x02C0 0x02D0 0x02E2` gives **0 occurrences of all thirteen**, against a
  positive control of 20 for `0x00D5` in the same files [L] — the specific-opcode grep, not an
  eyeball for something unfamiliar, because that is exactly how the cash-shop answer went wrong
  three times. Consistent with every detector flag being `0`. It becomes a hazard the moment any
  detector fires.
* **Anti-cheat that cannot report can kill the client deliberately.** This module already responds
  to a detection by corrupting the heap on a timer. Assume it can also respond to being interfered
  with, and assume the response is delayed by design.
* **The client is Themida-packed and ships its own crash reporting.** A `.text` edit may be
  checksummed; `Crc Fail Alert!!` is in this very module. Option (a) exists specifically to avoid
  that surface.
* **And nothing here has been tested on a running client.** Every claim in this file is from the
  file on disk, four crash dumps, and archived hook logs.

## 8. What is still unknown

* **What the VM body of `FUN_140c93c80` actually does.** It allocates `ZArray<int>(5)` on the
  writer's cadence [L]; that it then writes past it is **[I]** from its six readable siblings, not
  measured. §6's test settles it.
* **Why a shipped retail client would carry an unconditional heap overrun.** The readable six are
  gated behind cheat detections, which resolves it for them. `FUN_140c93c80`'s gate looks like
  static configuration, so either its VM body contains a detection the `.text` prologue does not, or
  the write is conditional inside the VM. **[I]**, and the honest answer is that it cannot be
  determined from here; the §6 test can, because it observes the write rather than the code.
* **Whether zeroing `0x143AC7F3C` is itself detected.** Cannot be determined statically. The run
  that tests it is the run that answers it.
