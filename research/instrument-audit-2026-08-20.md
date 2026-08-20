# Instrument audit — 2026-08-20

Two shared instruments in `tools/` were answering confidently and wrongly. Both are fixed.
This file records what the bugs were, the positive control for each fix, and **every claim
in `research/`, `docs/`, `STATUS.md` and `ROADMAP.md` that rested on them, re-run against
the fixed tools**.

Files changed by this pass: `tools/callers.py`, `tools/rtti.py`, and this file. Nothing in
`research/` was edited — several of those files are owned by agents that are still running.
Where a claim needs a correction, it is listed here with its file and line for the owner to
apply.

Instruments used: the fixed `tools/callers.py`, the fixed `tools/rtti.py`, `capstone` for
every disassembly quoted, and one batch script that reproduces `callers.py`'s three scans
over a whole list of addresses in one pass. No Ghidra (another agent holds the lock), no
client run.

---

## 1. `tools/callers.py` counted `call` and nothing else

### The bug

It scanned executable sections for `0xE8` (`call rel32`), attributed each hit to its
`.pdata` function, and printed a count. Three things reach a function in this image and it
saw one of them:

| entry mode | what it looks like | old tool |
|---|---|---|
| `call rel32` | `E8 xx xx xx xx` | found |
| **tail `jmp rel32`** | `E9 xx xx xx xx` at the end of a function or a shim | **invisible** |
| **a qword pointer in data** | a vtable or function-table slot, loaded and called indirectly | **invisible** |

This is the same blind spot `tools/reads.py` was fixed for on 2026-08-19. `callers.py` was
not fixed with it. `tools/dispatchers.py`, `tools/xref.py` and
`tools/find_handler_table.py:build_callgraph` were all checked during this audit and
**already scan `E8` and `E9`** — `callers.py` was the only one-mode instrument in `tools/`.

### The fix

`callers.py` now runs three scans and reports them as three separate kinds, because
"called from", "tail-jumped from" and "there is a pointer to this in a table" are three
different facts and the third has no return address at all. Added along with it, because
a byte scan is not a disassembly:

* every hit inside a `.pdata` function is re-checked against a capstone sweep of that
  function (merging contiguous `.pdata` entries first, or the sweep starts mid-function
  and desyncs);
* every hit that fails that check, and every hit in no `.pdata` function, gets a
  **convergence score** — how many of 32 earlier start addresses decode onto the hit
  exactly. Measured: 12 known-real sites score 22–31/32; `0xE8`/`0xE9` bytes picked out of
  `.rsrc`, which is not code, score 0, 0, 0, 0, 9, 24. Good, not perfect — it is printed
  as a number and labelled as corroboration, never used to drop a hit;
* a hit in no `.pdata` function also reports the **`0xCC`-bounded block** it sits in, which
  is the address to ask the tool about next (see §3.1, where that turned one line of output
  into a new call chain);
* a pointer hit reports its enclosing pointer run, the class name if an MSVC RTTI locator
  sits at the run head, and otherwise **the nearest address inside the run that code takes
  with a `lea`** — which is what turns a meaningless "+0x720 into a 477-slot run" into
  "vtable `0x14327e1d8` +0x358".

CLI and behaviour for the old invocation are unchanged: `python tools/callers.py <hex>
[exe]`, and the first line is still `%#x: %d call site(s) in %d function(s)` with the same
numbers, so anything quoting the documented control still reads identically.

### Positive controls, run

**The documented control still reproduces exactly** (this is what proves the call scan was
not disturbed):

```
$ python tools/callers.py 0x1402fa9a0
0x1402fa9a0: 96 call site(s) in 15 function(s)
    ...
    0x140304b20     43 call site(s)   first 0x140304e49  last 0x1403091e7
    ...
```

96 sites, 43 in `0x140304b20` — the numbers `research/channel-select.md:362` and
`research/channel-two-greyed.md:344` both record.

**Control 1 — the tail `jmp` at `0x1429b934b`:**

```
$ python tools/callers.py 0x1429bafb0
0x1429bafb0: 0 call site(s) in 0 function(s)

0x1429bafb0: 1 tail jmp site(s) in 1 function(s)
    0x1429b9300      1 tail jmp site(s)   first 0x1429b934b  last 0x1429b934b

0x1429bafb0: 0 qword pointer(s) to it in the image
```

The listing at that address, for the record:

```
0x1429b9346  add   rsp, 0x50
0x1429b934a  pop   rdi
0x1429b934b  jmp   0x1429bafb0
```

**Control 2 — the pointer to `FUN_140304100`:**

```
$ python tools/callers.py 0x140304100
0x140304100: 0 call site(s) in 0 function(s)

0x140304100: 0 tail jmp site(s) in 0 function(s)

0x140304100: 1 qword pointer(s) to it in the image
    0x14327e530  .rdata
        pointer run 0x14327de10..0x14327ecf0 (477 slots), this is +0x720 [slot 228]
        no RTTI locator at the run head
        nearest base that code loads is 0x14327e1d8 (lea at 0x1402f7dbd) -> this slot is
        that base +0x358 [index 107]
```

`0x14327e1d8 + 0x358 = 0x14327e530`, and `0x14327e1d8` is the vfptr the client actually
carries: `research/fixtures/melee-collector-runs-once-per-swing-hook.log:35` and three
other fixtures log `rcx=<obj> [0x4327e1d8]` on every entry to `0x140304100`, with
`called-from=0x140309686`. `research/bag-lists.md:247` already had `0x14327E1D8` and
`0x140304100` in the same row. The static tool now reaches the same place on its own.

**Control 3 — the `0x007C` mask decoder, reported as having 1 caller:**

```
$ python tools/callers.py 0x1402cbb50
0x1402cbb50: 1 call site(s) in 1 function(s)
    0x142d54780      1 call site(s)   first 0x142d5489d  last 0x142d5489d

0x1402cbb50: 1 tail jmp site(s) in 1 function(s)
    0x14104a890      1 tail jmp site(s)   first 0x14104a8b0  last 0x14104a8b0
```

**Control 4 — the RTTI class-naming path**, which the three cases above do not exercise
because none of those tables carries a locator. `0x142e64950` is slot 2 of the
`b2PolygonShape` vtable:

```
$ python tools/callers.py 0x142e64950
0x142e64950: 1 qword pointer(s) to it in the image
    0x14349a1a0  .rdata
        vtable 0x14349a190 (.?AVb2PolygonShape@@) +0x10  [slot 2 of 8]
```

Whole run takes 0.6–1.0 s per address; the old byte-at-a-time loop over 52 MB of `.text`
was replaced with `bytes.find`.

### Blast radius, image-wide

Measured over all 120981 `.pdata` function starts, one pass each:

| | |
|---|---|
| functions with at least one `call` candidate | 40089 |
| functions with at least one tail-`jmp` candidate | 11361 |
| functions with at least one 8-aligned data pointer | 22508 |
| **functions the old tool would call "zero callers" that have a tail jmp** | **6927** |
| **…that have a data pointer** | **21889** |
| **…that have either** | **27909** |
| functions with no entry of any of the three kinds | 52983 |

Byte-scan candidates, so these are upper bounds; the point is the order of magnitude. Any
"0 callers" this tool printed before today covered 40089 functions out of 120981 and was
silent about 27909 more that are entered some other way.

---

## 2. The VA→file-offset helper and `.data`'s BSS tail

### The bug

`.data` in this image is `vaddr 0x3a41000`, `vsize 0xa2aa8`, `rsize 0x67400`. The last
**0x3b6a8 bytes of it — VA `0x143aa8400`..`0x143ae3aa8` — are zero-initialised and are not
in the file at all.** A helper that maps `raddr + (rva - vaddr)` without clamping to
`rsize` walks past the end of `.data`'s raw bytes and into `.pdata`, and returns those
bytes as though they were the data.

The concrete case: `FUN_14087ec50` indexes the EXP curve at `0x143AC2400`, 121 `u64`s for
levels 1–120. The old mapping gives file offset `0x3ac0800`, which is inside `.pdata`, and
the first six qwords read there are

```
0x001a7da003501b94 0x03501ba4001a80a7 0x001a815c001a80c0
0x001a817003501c24 0x03501c48001a83c9 0x001a8436001a83d0
```

— which are `RUNTIME_FUNCTION` triples (`begin 0x3501b94 end 0x1a7da0 unwind 0x1a80a7`, …),
not EXP values. 120 confident wrong numbers.

### The claim "it lives in `tools/rtti.py` and most tools import it" is half right

Verified rather than assumed, because the blast radius is the point:

* **What is imported from `rtti` is `load_pe`, and only `load_pe`** — by
  `tools/callers.py:18`, `tools/dataref.py:36`, `tools/dispatchers.py:33`,
  `tools/reads.py:49`, `tools/fieldrefs.py:44`. Five tools.
* **`rtti.rva_to_off` is imported by nothing.** Its only caller is `rtti.read_vtable`.
  So the shared-mapper story is not what happened: every tool that needs a VA→offset
  carries **its own copy**, and the copies disagree.
* `rtti.rva_to_off` was not returning `.pdata` bytes — it had the `rsize` guard and
  returned **`None`**. That is a milder version of the same failure and still wrong for
  this project's rules: a `None` that a caller treats as "no data" is a silent answer.

The census of every VA→offset mapper in `tools/`:

| file | mapper | BSS-safe? |
|---|---|---|
| `tools/rtti.py:60` | `rva_to_off` | **now raises** (was silently `None`) — fixed here |
| `tools/callers.py` | `Image.foff` | safe (returns `None`, documented, used only to probe) |
| `tools/dump_va.py:42` | `va_to_off` | safe — returns `(None, name)` for the zero-filled tail |
| `tools/stacks.py:279` | `to_off` | safe — `return raddr + d if d < rsize else None` |
| **`tools/reads.py:118`** | `_foff` | **unguarded** — `max(vsize, rsize)`, no `rsize` clamp |
| **`tools/find_ptr_tables.py:108`** | `show()` | **unguarded** |
| **`tools/find_handler_table.py:218`** | `--show` | **unguarded** |

`reads.py`'s copy is **latent, not live**: `_foff` is only ever called with a function start
in `.text`, where `rsize >= vsize`, so it cannot currently be handed a BSS address. The two
`--show` paths take a user-supplied VA and *can* be pointed at `.data`; point either at
`0x143AC2400` today and it prints `.pdata` records as pointers.

**I did not touch those three files** — `reads.py` is explicitly off-limits and the other
two belong to nobody in this session. Each needs the same three-line clamp.

### The fix

`rtti.rva_to_off` (and a new `rtti.va_to_off(va, image_base, sections)`) now raise:

* `rtti.UninitialisedAddress` — in a section, past its raw bytes;
* `rtti.UnmappedAddress` — in no section.

Both subclass `ValueError`. The message names the section, both sizes, the offset the old
arithmetic *would* have produced, and the section those bytes actually belong to.

### Positive control, run

```
$ python -c "import rtti; ...; rtti.va_to_off(0x143AC2400, base, secs)"
rtti.UninitialisedAddress: rva 0x3ac2400 is 0x81400 bytes into section .data, whose raw
data is only 0x67400 bytes (vsize 0xa2aa8): it is in the zero-initialised tail and has NO
file offset. Mapping it arithmetically gives file offset 0x3ac0800, which is in .pdata -
those bytes belong to another section and are not this address's contents. Read this
address from the running process instead.
```

Negative controls — addresses that must still resolve, and the boundary:

```
0x143a41000  -> 0x3a3f400   .data, first raw byte
0x143aa83f8  -> 0x3aa67f8   .data, last raw qword
0x14327e530  -> 0x327d530   .rdata
0x140304100  -> 0x303700    .text
0x143aa83ff  -> 0x3aa67ff   last raw byte
0x143aa8400  -> raises UninitialisedAddress   first BSS byte
0x150000000  -> raises UnmappedAddress
```

`python tools/rtti.py --list CLogin` and `--vtable` still work; so do `tools/reads.py`'s
documented control, `tools/dataref.py` and every other importer of `load_pe`.

### One thing the control turned up on its own

**`0x143aa84a0` is itself in the BSS tail** — `0x674a0` into `.data`, 0xa0 bytes past the
raw end. That is the singleton quoted 19 times across `research/channel-select.md`,
`research/channel-two-greyed.md` and four listing dumps. Every one of those mentions treats
it as a *pointer slot resolved at run time* (`mov rcx,[0x143aa84a0]`), which is correct and
unaffected, and `tools/dataref.py` — which scans code for RIP-relative operands and never
reads the target — is unaffected too. **No conclusion depends on a static read of it.** But
anyone who ever tries to read its contents from the file will now get a raise instead of
`.pdata` bytes, and that is the point.

---

## 3. The sweep: every caller claim, re-run

### Method, and the instrument fix the sweep itself needed

Grep `research/**.md`, `docs/**.md`, `STATUS.md` and `ROADMAP.md` for
`call site|callers\.py|caller|vtable-only|no direct call|zero call|0 call|virtual` and
collect every hex address that is a `.pdata` function start. Then one pass over the image
computing all three scans for that whole address set at once.

The first version scoped a claim to **one line** and found 173 windows over 157 addresses.
That missed `research/bag-lists.md:293-295`, where the function name is on one line and
"`callers.py` reports 0 callers" is on the next. Re-scoped to a **±2-line window**: 447
windows over 281 addresses. The wider sweep is the one used below. (Recorded because it is
the same failure this whole audit is about — a search that answers cleanly about the wrong
set.)

**The fix cannot change a call-site count.** It only adds tail jmps and pointers. So a
claim is affected if and only if its function has at least one of those. **68 of the 281
addresses do**, and each one was read in context; the ones where the surrounding sentence
is not actually a claim about that function's callers are not listed below.

### 3.1 Conclusions that change

| # | file:line | the claim | re-run | verdict |
|---|---|---|---|---|
| 1 | `research/user-chat.md:128` | "`FUN_1429bafb0` … has **zero direct callers** — it is virtual" | 0 calls, **1 tail jmp** (`0x1429b934b`, in `FUN_1429b9300`), **0 pointers anywhere in the image** | **Wrong on both halves.** It has a caller, and it is *not* virtual — nothing points at it. Already retracted at `research/user-chat-round2.md:270-280`; `user-chat.md` still says it |
| 2 | `research/setfield-fault-shape.md:16`, `STATUS.md:1828` | "`FUN_140ce89c0` has 5 call sites"; "**both real callers** … end `LEA RCX,[RSP+N]` / `CALL` / epilogue" | 5 calls **+ 5 tail jmps** (`0x143000b64`, `0x143029f2b`, `0x143045d0b`, `0x1430464f7`, `0x14313c881`; converge 22–27/32) | **Incomplete: 10 entries, not 5.** The five new ones are adjustor thunks of the shape `lea rcx,[rdx+X] / jmp 0x140ce89c0` in the `0x143……` forwarder tables — i.e. *virtual* entries into the same destructor. The "both real callers have the same shape" observation is untouched; the sentence "**any** early exit from either function faults at the same address" now has more than two functions to say it about |
| 3 | `STATUS.md:1654`, `research/naked-character.md:378` | "**No inbound opcode reaches `FUN_140f80140`** by direct call — all 23 callers checked against the 273-case table" | 23 calls **+ 5 tail jmps** | **Not refuted, but the stated evidence no longer covers the entry set.** Four of the five are `0xCC`-bounded shims (`mov rcx,[rcx+0x78] / test / je / xor r8d,r8d / jmp 0x140f80140`) at `0x14109d440`, `0x14109d460`, `0x1420dd920`, `0x1429cb550`; the fifth is `FUN_14129c230`. **A new call chain exists that nobody walked**: `FUN_142d012e0` → `FUN_142797be0` @ `0x142797df5` → shim `0x1420dd920` → `FUN_140f80140`. Neither `FUN_142797be0` nor `FUN_14129c230` is in `research/msexe-gamestage-cases.txt` (grep returns 0 against a file with 271 `FUN_` rows), so the conclusion survives on what I can measure — but "all 23 callers checked" should read "23 callers and 5 tail-jmp entries, of which the `142d012e0` chain was walked" |
| 4 | `research/mob-spawn.md:738` | "`tools/callers.py` gives `FUN_1409c50a0` **six** call sites" | 6 calls **+ 4 `.rdata` pointer slots** — `0x143304100`, `0x143304628`, `0x14348c608`, `0x14348d728`, and every one of them is **+0x118 (index 35)** from a base that code `lea`s | **Incomplete.** It is a virtual as well as a called function, in four sibling tables at the same slot. That is a stronger statement than the six call sites |
| 5 | `research/npc-click.md:232` / `research/npc-dialogue.md:79` | "`FUN_142d9ac30`'s **26** call sites" / "(27 call sites, all UI)" | **27 calls in 26 functions**, + **9 tail jmps**, each in its own `.pdata` function (`0x14142f803`, `0x142233f10`, `0x142237310`, `0x142255a60`, `0x14225d8f0`, `0x142261420`, `0x1422eeb90`, `0x1423ffda0`, `0x142c243c0`) | 27 is the site count, 26 the function count — the two documents were counting different things. **36 entries total.** "The other 25 are UI" is now "the other 35" |
| 6 | `research/talking-back.md:283` | "`FUN_142d3c670` has **exactly three** callers" | 3 calls + **1 tail jmp** from a `0xCC`-padded thunk at `0x142cfb380` (converge 31/32) | **Incomplete but the conclusion stands.** The thunk itself has 0 calls, 0 tail jmps, 0 pointers and no `.pdata` record — dead code. The three-caller argument is not weakened |
| 7 | `research/npc-spawn.md:74` | "`callers.py 0x141e36b20` → **4** call sites, all inside `0x141e75800..0x141e762b0`" | 4 calls + **1 tail jmp at `0x141e75e07`** (in `FUN_141e75d86`) | **Count is 5, the substance holds** — `0x141e75e07` is inside the quoted range, so "all inside the NPC pool's own packet code" survives |
| 8 | `research/user-chat-round2.md:318` | "`FUN_14276df20` … **417 call sites in 289 functions**" | 417 / 289 confirmed, + **1 tail jmp** (`0x141c6cd22` in `FUN_141c6ccf0`). Four of the 417 fail the in-function boundary check and score 24–29/32, i.e. real | 418 entries. Immaterial to the argument, listed for completeness |
| 9 | `docs/login-server.md:419` and `:450` | "`FUN_14003fb80` (643 B) … **0** — another orphan, **same shape as the setter**"; "`FUN_14003fb80` remains an orphan of the same shape as the setter" | 0 calls, 0 tail jmps, **1 qword pointer at `0x143266350`** — slot 1611 of a 7392-slot function-pointer table at `0x1432630f8`, every neighbouring slot of which is also a distinct `.pdata` function start | **Not the same shape as the setter.** Re-run for the contrast: the setter `FUN_140c9e230` is **0 / 0 / 0** — its headline conclusion survives, and is now stronger than when it was written because it also survives a tail-`jmp` scan. `FUN_14003fb80` is not an orphan; it is in a table. Caveat that keeps it honest: **no `lea` anywhere in the image targets any address inside that 59 KB table**, so how it is indexed is unknown |
| 10 | `research/npc-spawn.md:79` | "`FUN_1418224c0`, `FUN_141b7c960` … are all absent" from the 87 | unchanged as a statement about that list, but note `FUN_1418224c0` has **62** pointer slots and `FUN_141b7c960` **64** — both are heavily-instantiated virtuals | no change; recorded so the next reader does not mistake "not in that list" for "not reached" |

### 3.2 Instrument notes in `research/` that are now obsolete

These are not wrong conclusions — they are warnings about the old tool that the fixed tool
has absorbed. Left in place, listed so their owners can retire them.

| file:line | note | status |
|---|---|---|
| `research/channel-select.md:556-558` | "**`callers.py` cannot see a virtual.** … Any '0 callers' from `callers.py` must be followed by a pointer scan before it means anything" | The pointer scan is built in. `python tools/callers.py 0x141b25f30` prints the single `.rdata` slot at `0x1433fd7a0` directly — the same address the hand-rolled scanner found |
| `research/mob-combat.md:465` | "**`tools/callers.py` cannot see a tail call.** `FUN_141d23960` reports **0 callers**" | Now reports the tail jmp at `0x141c9978a`. Confirmed exactly |
| `research/mob-gates-arm-c.md:453` | "`0x141c56e00`: 0 calls, 0 tail jmps, **1 qword pointer, at `0x1434077f8`**" | Reproduced exactly by the tool, in one command. This entry was worked out by hand earlier today |
| `research/level-up.md:133,148` | "`callers.py 0x1402cbb50` reports **1 call site** … and I nearly wrote 'it is'" | Confirmed: 1 call + 1 tail jmp at `0x14104a8b0`. Caught by hand today; now the tool says it |
| `research/npc-shop.md:97-101` | "`callers.py` returns **0 call sites** and `xref.py --va` **0 references** … A scratch qword scanner found it at `0x14336c720`" | The tool now finds `0x14336c720` itself |
| `research/bag-lists.md:293-295` | "`FUN_1402cf5c0` … is vtable-only, so `tools/callers.py` reports 0 callers" | Confirmed and located: one pointer at `0x14327e8e8` |
| `research/npc-chatter.md:446` | "`FUN_141e3e220` has 0 direct callers and two vtable slots" | Confirmed exactly: 0/0/2, slots `0x143413860` and `0x143413970`, each of which is itself a `lea`-taken base |
| `research/user-chat-round2.md:270-280` | the retraction of #1 above, and "**`callers.py` counts `call` only**" | Correct, and now fixed at the source |
| `docs/login-server.md:400`, `STATUS.md:2249`, `STATUS.md:3329` | "`FUN_141b25f30`, a known vtable entry: found in `.rdata` at `0x1433fd7a0`" | Confirmed |
| `research/mob-spawn.md:645` | "**three** indirect call sites at a vtable offset where `FUN_141c81040` lives" | Sharpened: 8 pointer slots, and **all 8 are +0x8 (index 1)** from a `lea`-taken base |

### 3.3 Claims re-run and unchanged

Every one of these was re-run; all three scans agree with what the document says.

| file:line | claim | re-run |
|---|---|---|
| `research/channel-select.md:362`, `research/channel-two-greyed.md:344` | control: `0x1402fa9a0` → 96 sites, 43 in `0x140304b20` | 96 / 43 — identical |
| `research/channel-two-greyed.md:229,351-353` | `FUN_142cb9510` six call sites, and "callers.py's six / three / one direct call sites are the complete caller sets" | `0x142cb9510`: 6 calls, **0** tail jmps, **0** pointers. The claim was right and is now reproducible in one command |
| `research/channel-select.md:379` | `FUN_141b2c7c0` has two callers | 2 / 0 / 0 |
| `research/equip-stats.md:833` | "`callers.py 0x14038d3c0` reports **0 call sites**" | 0 / 0 / 0 — a genuine orphan under all three scans |
| `research/equip-block.md:27` | `0x1402ee8d0` → 17 call sites in 16 functions | 17 / 0 / 0 |
| `research/equip-stats.md:225,489,773` | `FUN_1426b20f0` has exactly one call site | 1 / 0 / 0 |
| `research/equip-stats.md:931,934` | `0x1402f7010` 499 sites / 121 fns; `FUN_1402f7da0` 5 sites in 4 fns | unchanged |
| `research/bag-lists.md:33` | `FUN_14030b6f0` has six call sites, all in `FUN_140304b20` | 6 / 0 / 0, all in `0x140304b20` |
| `STATUS.md:3224` | `FUN_141b2b120` and `FUN_141b2ae80` "have no callers and are in no vtable" | both 0 / 0 / 0 — **holds under all three scans**, which is exactly the property that walk-aiming needs |
| `docs/session.md:349` | `FUN_142c4f490`'s only caller is `FUN_141b0ef00` | unchanged |
| `research/npc-shop-crash.md:220` | `FUN_14246e870`'s three callers | unchanged |
| `research/mob-behaviour.md:397` | `FUN_141d57c60`'s 9-caller list | unchanged |
| `research/item-drop.md:81-82` | `FUN_142cc5b00`'s five callers | unchanged |
| `research/mob-target-gates.md:370` | `FUN_1428baa50`'s four callers "also make no direct call into mob code" | claim is about what those functions *call*, not what calls them — unaffected. Noted: three of the four (`0x1418f5300`, `0x1419083c0`, `0x1419987a0`) are themselves pointer-only entries, one slot each |
| `docs/login-server.md:377,383,399-402` | "**The setter genuinely has no caller in readable code**" — `FUN_140c9e230`, 0 direct calls and 0 qword pointers | **0 / 0 / 0.** Holds, and now also survives the tail-`jmp` scan the original check did not run |
| `research/channel-two-greyed.md:354` | "`FUN_142cb83d0`, the `+0x2250` setter, **has no callers of either kind**" | 0 / 0 / 0 — holds under three kinds, and its sibling getter `FUN_142cb84a0` still shows its 2 call sites |
| `research/channel-two-greyed.md:461`, `research/channel-select.md:195` | `FUN_142cb8e10` has a single caller | 1 / 0 / 0 |
| `research/msexe-gamestage-dispatch.md:228` | "`FUN_142097ee0` forwards `0x1a0` to **`FUN_142097f80`**" | **Now visible as a mechanism.** `FUN_142097f80` has 0 calls and **1 tail jmp, at `0x142097f22`** — and the tool resolves that site's `0xCC`-bounded block to **`0x142097ee0`**, the named forwarder, which has no `.pdata` record of its own. `FUN_142097ee0` in turn has 2 call sites (`FUN_141820080` = `CField::OnPacket`, `FUN_141b25f30` = the login `OnPacket`) and 4 vtable slots. The old tool reported the `0x01A0` handler as having zero callers |

`tools/handler_root.py` deserves a line of its own: its "a walk target must have no direct
callers **and be in no vtable**" rule already did a qword scan, and its call graph comes
from `find_handler_table.build_callgraph`, which already handled `E9`. **No walk was aimed
using the broken instrument.**

---

## 4. The sweep: static reads inside `.data`'s BSS tail

Every `.md` line mentioning an address in `0x143aa8400..0x143ae3aa8` — 153 lines across 30
files, plus 637 lines including the raw listing dumps.

**Almost all of them are safe, for the same reason.** Two shapes dominate and neither
reads the file:

* `mov rcx,[0x143ABFE00]` — the address is a *pointer slot*, and the documents describe
  the pointer as resolved at run time. Every singleton in `research/npc-spawn.md:180-191`,
  `research/mob-spawn.md:35`, `research/npc-click.md:62`, `docs/session.md:192`, and the
  `0x143AA84A0` family, is this shape.
* the character-record flag table at `0x143abeb10`, stride `0x70`, all through
  `research/charrecord-presence-map.md`, `charrecord-flag7.md`, `inventory-slots.md`,
  `quest-state.md`, `naked-character.md` and `bag-lists.md`. Those byte values were read
  **out of the CRT initialiser code** (`0x1400232a2 MOV byte ptr [0x143abee20],1`), not out
  of the file. That is the correct method for a BSS address and it is unaffected.
  `research/charrecord-presence-map.md:39` and `charrecord-flag7.md:288` both say so
  explicitly ("in the **uninitialised** …", "that address has **no file bytes**").

| file:line | what it says | verdict |
|---|---|---|
| `research/level-up.md:455-477` | the EXP curve at `0x143AC2400` is in the uninitialised tail, cannot be read statically, and a naive `foff()` "returned 120 plausible-looking 8-byte 'EXP values' that were in fact `RUNTIME_FUNCTION` triples" | **Already correct, and it is the record of this bug.** The wrong numbers were caught before they were published. Its instruction — "Any VA→file-offset helper used on `.data` in this image must clamp to `rsize` and report BSS as absent" — is what `tools/rtti.py` now does |
| `STATUS.md:166` | "121 `u64`s at `0x143AC2400`, in the BSS tail of `.data` — **zero on disk**, so static analysis cannot ever read it. One `-Probe` peek" | Correct, unchanged |
| everything else in the sweep | pointer slots and initialiser-derived flag bytes | unaffected |

**No published conclusion had to be retracted for bug 2.** The one case that produced wrong
numbers caught itself, which is why the bug was known well enough to be handed to this
audit in the first place.

---

## 5. `STATUS.md` and `CLAUDE.md`

Not edited — flagged for the coordinator.

* **`STATUS.md:1654`** — "No inbound opcode reaches `FUN_140f80140` (the apply primitive)
  by direct call — all 23 callers checked against the 273-case table." The conclusion is
  not refuted, but there are **28 entries, not 23**, and a call chain
  (`FUN_142d012e0` → `FUN_142797be0` → shim `0x1420dd920` → `FUN_140f80140`) that the
  original check never saw. See §3.1 #3.
* **`STATUS.md:1828`** — "both real callers of `FUN_140ce89c0`". There are five more
  entries, all adjustor thunks. See §3.1 #2.
* **`STATUS.md:3224`, `:3329`, `:2249`** — re-run and **confirmed** under all three scans.
* **`CLAUDE.md`** — nothing to retract. Its "Re-run your own analysis when you fix a shared
  instrument" section named `reads.py` and 2026-08-19; `callers.py` was the sibling that
  was not fixed that day, and this audit is the re-run. If a line is added, the honest one
  is that **`callers.py` was the last single-mode scanner in `tools/` and it stayed broken
  for a day after its sibling was fixed** — which is the section's own thesis.

---

## 6. What is still invisible

Written down so the next "zero" is read correctly.

* **A computed target** — `call qword [rax+0x358]`, anything Themida virtualised — leaves
  no trace in any of the three scans. `0x140304100` is exactly this: the pointer is
  findable, the code that loads it is not named anywhere in `.text`.
* **The MSVC switch table**, which stores 4-byte RVAs rather than qwords. Tried during this
  audit: every function's own `.pdata` record contains its RVA, so a dword scan returns one
  guaranteed self-hit, and for the six handlers checked it returned nothing else.
  `tools/find_switch_tables.py` is the tool for that shape.
* **Byte scanning is not disassembly.** The convergence score is corroboration; one `0xE9`
  inside `.rsrc` scored 24/32. Read the listing before a number becomes a claim.
* **`tools/reads.py`, `tools/find_ptr_tables.py` and `tools/find_handler_table.py` still
  carry unguarded VA→offset mappers** (§2). Two of them are reachable from a command line
  with a `.data` address.

## 7. Tests

`tools/` has **no Python test suite** — no `test_*.py`, no `conftest.py`, no `pytest.ini`.
The only `test-*` files there are the three PowerShell launchers. So there is no coverage to
keep green, and nothing in this pass was verified by a test: the controls in §1 and §2 are
the verification, and they are reproducible from the command lines quoted.
