# Amherst Department Store (map 1013) kills the client — and it is not the NPC packet

Written 2026-08-19, after the run preserved as
`research/fixtures/amherst-1013-heap-corruption-{world,hook,login,exit}.log`.
**No Ghidra** (another agent holds the project lock), **no client run** — everything here
comes from that one capture plus the PE, `wz-dump`, and the capstone tools in `tools/`.

Markers: **[L]** read out of a log or the image, **[D]** derived from those, **[I]** inferred.

---

## 0. Answer up front

| question | answer |
|---|---|
| Did the `0x044F` NPC packet for template 21 reach its handler? | **No. It was never dispatched.** [L] |
| Where did the client die? | **Inside `FUN_142097f80`, the `0x01A0` SetField handler**, after it emitted outbound `0x02DE` and before it reached `FUN_142defbc0` at `0x14209aa61`, which sends `0x0184`. [L] |
| Is the NPC body implicated? | **Not in the way STATUS.md assumed.** It cannot have caused the fault, because its handler had not run. It remains a candidate for having *written* a latent overflow earlier — see §4. |
| Are both crashing maps "places with shops"? | **No.** Lucy (template 21) is a grocer; Adobis (template 1117, map 20001075) is the Zakum quest NPC and sells nothing. **The shop lead is dead as stated.** [L] |
| Best remaining lead | Map 1013 is the **only** map visited in the whole session with **no `miniMap` node** while `hideMinimap = 0`, and the last call before the packet that never went out pokes the minimap UI object. **[I], correlation only.** §5 |

---

## 1. The measurement, and why it is trustworthy

### 1.1 The instrument

`crates/grap-stub/src/hook.rs` logs its numbered dispatch line **after** the trampoline
returns:

```rust
let ret = tramp(conn, view);       // the real handler
...
log(&format!("{n:5} opcode=0x{opcode:04X} elapsed_us={micros:.1} ret={ret} ..."));
```

and `log()` opens the file, writes one line and closes it (`hook.rs:200`). **There is no
buffering to lose the last line.** A missing numbered line therefore means the handler never
returned — not that the write was lost.

### 1.2 The counts

| opcode | sent by the server (`world.log`) | completed in the client (`maplecw-hook.log`) |
|---|---:|---:|
| `0x03C6` mob spawn | 52 | **52** ← the control: the counter drops nothing |
| `0x01A0` SetField | 8 | **7** |
| `0x044F` NpcEnterField | 11 | **10** |

The one missing `0x01A0` is map 1013's. The one missing `0x044F` is template 21's. **[L]**

### 1.3 Ordering: `0x01A0` always completes before its `0x044F`

Every field entry in the session, six of them with NPCs, in the hook log:

```
22:28:20.222   555 opcode=0x01A0    22:28:20.223   556 opcode=0x044F
22:29:27.456  1077 opcode=0x01A0    22:29:27.458  1078 opcode=0x044F
22:29:41.934  1082 opcode=0x01A0    22:29:41.935  1083 opcode=0x044F
22:29:53.084  1086 opcode=0x01A0    22:29:53.084  1087/1088 opcode=0x044F
22:30:58.053  1111 opcode=0x01A0    22:30:58.053  1112/1113/1114 opcode=0x044F
```

Inbound packets are **queued, not nested**: on map 1010 the server put three `0x044F` on the
wire at `02:30:58.047`, six milliseconds *before* the `0x01A0` handler returned, and the
client dispatched none of them until it had. **[L]**

So on map 1013 the `0x044F` arrived at `02:31:07.901`, the `0x01A0` handler never returned,
and **the NPC handler could not have run.** **[D]**

### 1.4 An independent corroboration from the wire alone

Every successful field entry emits the same client→server sequence:

```
0x00DC  →  0x02DE  →  0x0184  →  0x0194  →  0x0070
```

Census for the whole session: **8 × `0x00DC`, 8 × `0x02DE`, 7 × `0x0184`, 7 × `0x0194`**.
Map 1013 got as far as `0x02DE` and stopped. **[L]**

`0x0184` is sent by `FUN_142defbc0`, called at `0x14209aa61`; `0x0194` by `FUN_142defc50` at
`0x14209aa69`. Both are inside `FUN_142097f80` (`tools/callers.py`, and
`research/msexe-send-opcodes.txt`). So this is a second, completely independent instrument
saying the same thing: **the client died inside the SetField handler, in its tail.**

### 1.5 The last 400 microseconds

```
02:31:07.482  ->  0x01A0 SetField, portal "in02" -> map 1013 portal 1
22:31:07.490      WATCH 0x140304100 x4  (the four equipped items decoded - the record is fine)
22:31:07.899      C++ THROW #13
02:31:07.900  <-  0x00DC  CLIENT_FIELD_ENTERED
02:31:07.901  ->  0x044F  template 21, object 1000        (never dispatched)
02:31:07.901  <-  0x02DE                                   (the last thing the client ever said)
22:31:07.902      WATCH 0x1415db360   - identical registers to the map-1010 entry that worked
02:31:07.973      socket reset
22:31:08.196      EXIT 0xC0000374
```

The map-1010 entry ten seconds earlier ran the *same* two watches with the *same* register
values and returned five milliseconds later. **[L]**

---

## 2. Our packet is not different

The `0x01A0` body for 1013 is **1201 bytes and differs from the map-1010 body that worked in
five bytes**:

| offset | 1010 | 1013 | what |
|---:|---|---|---|
| 1..3 | `c6 51 ee` | `a7 47 f4` | the server clock base |
| 0xF0 | `f2 03` | `f5 03` | the map id, 1010 → 1013 |
| 0xF4 | `00` | `01` | the spawn-portal id, 0 → 1 |

**[L]**, byte-diffed from `world.log`.

Two things this eliminates:

* **The spawn-portal id is not it.** Portal ids 1, 3 and 4 have all been sent on successful
  portal walks in earlier captures (`grep 'SetField, portal' research/fixtures/*.log`), and
  map 1013 genuinely has a portal 1 (`out00`) in `gm-handbook/portals.txt`. **[L]**
* **The record decoded.** `0x140304100`, the type-1 equip decoder, fired four times from
  `0x140309686` — once per stored item — 400 ms before the crash. **[L]**

Every foothold id we send exists in its map's foothold table, checked against `Map.wz`:
1010 → 79/40/36 of 114, 1013 → 5 of 35, 20001075 → 159 of 312. **[L]**

---

## 3. The shop lead, as stated, is dead

`STATUS.md` and the brief both rest on "both maps are places with shops".

* Map 1013's NPC is **template 21 = Lucy**, and `data/shops.txt:95` does carry them:
  `shop: Lucy | Grocer | Maple Road (Beginner Zone)`. **[L]**
* Map 20001075's NPC is **template 1117 = Adobis**, "Does anyone want to try the Zakum
  Quest?" — the Zakum quest giver. **They own no shop and appears nowhere in
  `data/shops.txt`.** **[L]**

So the two maps do not share a shop. They share *having exactly one NPC in the `life`
node* — but so do 80001000, 90040200 and 90040203, all of which were entered without
incident in the same session. **[L]**

### 3.1 And the NPC templates themselves do not differ

Extracted every leaf path (digit keys normalised to `#`) from `Npc.wz` for the two crashing
templates and for ten templates the client loaded successfully this session
(8, 9, 17, 18, 19, 20, 800003, 900011, 900013, 900016), then intersected:

* paths present in **both** crashers and in **no** working template: **none**
* paths present in **every** working template and in **neither** crasher: **none**

`info` keys do not separate them either (21 and 1117 both have only `speak`; so do the
working 17 and 20). Nor does `z`, nor per-animation `speak`, nor `_outlink`. **[L]**

**The NPC template's WZ shape does not explain the twin.**

---

## 4. What walking the `0x044F` decode actually found

Done anyway, because the map-1010 NPCs *did* decode and a latent overflow there is the one
way the NPC body could still be guilty (see §6).

`python tools/reads.py 0x141e36b20 3` — positive control
`python tools/reads.py 0x140304100 2` returns its usual 20-plus lines first — gives exactly
the twenty reads `research/npc-spawn.md` §4 already tabulates, in the same order. Our
builder in `crates/net/src/opcode.rs::npc_enter_field` fills them all. **No field is missing
and no field is surplus.** **[L]**

The fields we leave zero are reads 5, 6, 12, 13, 14, 15, 16, 17, 18, the `raw[8]` (19), 20,
21 and the empty string (22). Of these:

| read | shape | risk |
|---|---|---|
| 19 `raw[8]` | **the one length-driven copy in the body** | the length is `mov edi,8`, an immediate in the client at `0x141e37e66`/`0x141e37ee6`. **We cannot drive it.** [L] |
| 20 `u32` | `0` means "use `[template+0x2a4]`" | benign by construction [D] |
| 21 `u32` | consumed only when non-zero | benign [D] |
| 22 `str` | non-empty opens a further block | we send length 0, so the block is skipped [D] |
| 9 `fh` | hash lookup, **result not null-checked** | every id we send is real (§2), so it never misses [L] |

**There is no field in the `0x044F` body whose value we control that drives a length or a
buffer size.** That closes the specific mechanism the brief asked about. **[D]**

---

## 5. The best remaining lead, and it is a correlation, not a mechanism

Every map visited in the session, from `Map.wz`:

| map | `miniMap` node | `hideMinimap` | NPCs in `life` | outcome |
|---|---|---|---:|---|
| 40 | yes | 0 | 2 | ok |
| 80001000 | yes | 0 | 1 | ok |
| 90000000 | yes | 0 | 0 | ok |
| 90040200 | yes | 0 | 1 | ok |
| 90040203 | yes | 0 | 1 | ok |
| 90050000 | yes | 0 | 2 | ok |
| 1010 | yes | 0 | 3 | ok |
| **1013** | **NO** | **0** | 1 | **died** |

**[L]**, all from `wz-dump cat`.

Map 1013 is the only one with no `miniMap` node, and its `info` still says
`hideMinimap = 0` — the client is told to show a minimap for a map that has no minimap data.

Three things line up with that, and they are adjacency rather than proof:

* The **last call in the handler before the `0x0184` that never went out** is
  `0x14209aa55  call 0x14246e7a0` — a small setter on `DAT_143acf088` that writes
  `+0x5c0/+0x5c4/+0x5c8/+0x248` (a mode with `% 100` arithmetic) and tail-jumps
  `FUN_14246e510`. **[L]**, `research/msexe-fieldentry-tail.txt`.
* `FUN_14246e870`, 208 bytes further on in the same object's code, is **the only function in
  the image that reads the WZ property name `miniMap`** — three times, at `0x14246ea7a`,
  `0x14246ec0e`, `0x14246ed0e`. **[L]** (found via `dataref.py` on the `PTR_` slots
  `0x143a48368`/`0x143a48378`; `xref.py --string "miniMap"` returns **0**, which is the
  documented `PTR_`-indirection blind spot, not an absence — control:
  `xref.py --string "Map/Map/Map%d/%09d.img"` returns 2).
* `FUN_14246e870`'s callers are `FUN_1418224c0`, `FUN_14186fad0` and `FUN_14246fd50`, and
  **`FUN_1418224c0` is the field-load chain** that `research/npc-spawn.md` §2 already
  identified. **[L]**

**Against it:** map 20001075 *has* a `miniMap` node. So this does not explain the twin — and
the lookup at `0x14246ea7a` does check its result (`test eax,eax / jns`, then a found-flag at
`0x14246eb16`), so the miss is at least noticed. **[L]**

---

## 6. What `0xC0000374` does and does not license

`STATUS_HEAP_CORRUPTION` is raised **when the allocator next walks the block headers**, not
when the overwrite happens. So the fault site names the *detection*, and a field entry — the
allocation-heaviest thing the client does — is exactly where a latent overflow surfaces.

That leaves two live readings, and the capture does not separate them:

**H1 — local.** Something in the client's construction of field 1013 corrupts the heap. §5 is
the candidate.

**H2 — latent.** Something earlier wrote past a block and 1013's field entry is merely the
first allocator walk to notice. Candidates, in the order they touched the heap before the
crash: the three `0x044F` decodes on map 1010 (templates 17/19/18, ten seconds earlier), the
52 mob-spawn decodes (`0x03C6`, mobs were on in this run), the character record itself, and
the 32 `0x0453` chat balloons.

Against H2, weakly: seven earlier field entries in the same session each walked the heap and
found nothing wrong. For H2 the overwrite would have to have happened after the map-1010
entry, which narrows it to the three NPC decodes, three chat balloons, and the client's own
movement handling.

**Do not write either of these down as the answer.** The 72 ms gap the brief mentions points
neither way, and neither does anything else in this capture.

## 6.1 One thing that is *not* uncertain

`0xC0000374` was the **process exit code**. This project's own note is that a `__fastfail`
returns `0xC0000409` (`STATUS.md`, "SOLVED - the ~37 second exit"). An exit code of
`0xC0000374` is therefore an **unhandled exception**, delivered through SEH — which a
vectored handler sees. **[I], but it is the reading the exit code supports.**

---

## 7. Instruments, and two of them are lying by omission

**1. The probe's fault filter cannot report this crash.** `crates/grap-stub/src/probe.rs`
logs `CLIENT FAULT` only for

```rust
0xC000_0005 | 0xC000_001D | 0xC000_0025 | 0xC000_008C | 0xC000_008E
    | 0xC000_0094 | 0xC000_00FD | 0xC000_0096
```

**`0xC000_0374` is not in that list.** So "no CLIENT FAULT line" is a property of the search,
not evidence that no exception was raised. This is the `CLAUDE.md` "enumerate before you
filter" failure exactly: a filter over a known set, and the case that mattered was outside it.

**2. `tools/decode_elog.py` prints the wrong VA.** It adds `IMAGE_BASE = 0x140000000` to an
address the client reported with the top image-base nibble dropped; the correct addend is
`0x100000000`. Every VA it prints is `0x40000000` too high — e.g. `0x180194df5` where the
real address is `0x140194DF5`. Frame *names* are unaffected. (Not fixed here: `tools/` was
off-limits to this pass.)

**3. `client-exit.log`'s reading is wrong for this code.** It calls `0xC0000374` "a code with
no standard meaning, so it was chosen". It is `STATUS_HEAP_CORRUPTION`.

**Checked and genuinely negative:**

* No `0x008F`/`0x0090` ELog for this crash. The one in `login.log` is dated `02:26:23` /
  `02:27:07`, i.e. **before this client started at `22:27:54`** — it is the replay of an
  earlier failure, and its stack is the client's own C++-throw reporter
  (`ZtlTaskMemReallocImp()+...`, `CallWindowProcW`), not a fault. Positive control: the tool
  decodes it, so it can find one when one is there.
* No Windows Application-log `Application Error` event, no WER report in either
  `ReportQueue`/`ReportArchive`, no `%LOCALAPPDATA%\CrashDumps` entry, and no file written
  under `client-patched/` by the client during the run except the hook log itself.
* The client's own `CrashReportClient.exe` never started (`client-exit.log`'s
  "processes that started after the client did" line).

The client keeps its ELog on disk and uploads it at the **next** launch, so **`login.log` on
the next run should be read first** — if the client managed to write one before the heap
killed it, it is free evidence.

---

## 8. What this changes in `STATUS.md`

The entry "NEW: two maps kill the client, and they look like the same bug" should record:

* the NPC packet was never dispatched, so the `0x044F` body is not the fault site;
* Adobis is not a shop NPC, so "both maps have shops" is wrong;
* the proposed one-variant test ("`-Probe` a watch on the NPC pool's decode entry") would
  answer a question the hook log has already answered.

---

## 9. The one-variant test

**The change, and it is the only one:** make the probe's fault filter report
`0xC000_0374`. `crates/grap-stub/src/probe.rs:945` —

```rust
0xC000_0005 | 0xC000_001D | 0xC000_0025 | 0xC000_008C | 0xC000_008E
    | 0xC000_0094 | 0xC000_00FD | 0xC000_0096 | 0xC000_0374
```

and give that `log!` the `stack_trace(rsp)` the C++-throw branch four lines above already
builds (`rsp` is read from `(*info).context` at `CTX_RSP`). **No protocol byte changes**, so
nothing about what the client is fed is different from the run that crashed.

The VEH is registered by `arm()` (`probe.rs:577`) and `test-server.ps1` arms watches on
every launch, so no extra flag is needed.

**Free, and before anything else:** read `login.log` at the top of the next run. The client
replays its on-disk ELog as `0x008F`/`0x0090` at startup and then deletes it, so if it wrote
one for the 1013 crash it arrives on the *next* launch and nowhere else.

**The route:** reproduce exactly — `!map 1010`, then walk the `in02` portal into 1013.
Nothing else; a different route changes the heap history, which for a corruption bug is the
one thing that must not change.

| outcome | what it means |
|---|---|
| `CLIENT FAULT #1: code=0xc0000374 at <addr>` with a stack | **The answer.** `at` will be in `ntdll` (the allocator that noticed); the stack's `<-TEXT` frames name the client function that was allocating. That plus §5 either confirms or kills the minimap lead in one read. Only the `at=` value is exact — the `stack:` line is a heuristic scan (`STATUS.md` §2g). |
| the client dies with `0xC0000374` and **still no** `CLIENT FAULT` line | The fault is **not** delivered through SEH — it is a `__fastfail`, and §6.1's reading of the exit code is wrong. Retract §6.1 and reach for a watch instead: `0x14209aa61` (does the handler reach the `0x0184` send?) bisects the window from the far end. |
| the client survives 1013 | The intermittency is real and it is **not** a property of map 1013. That kills the "two maps kill the client" framing outright: heap corruption surfaces at the next allocator walk, and field entry is simply the heaviest one. Next probe is then `!map 1011` — the only other map in the Amherst set with no `miniMap` node, one NPC (template 20, Sid) — with `!map 1012` (miniMap present, **zero** NPCs) as its control. |

