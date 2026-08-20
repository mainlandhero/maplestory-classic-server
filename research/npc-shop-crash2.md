# The shop UI's art is not in this client's WZ. The server cannot fix it.

Written 2026-08-20 after runs 1 and 2 (12 rows, then 1 row). **No client run was made for
this file** — everything below is the two captures already on disk, the PE, `tools/*.py`,
and `wz-dump`. The Ghidra project was **not** opened; nothing here needed it.

Markers: **[L]** read off a listing or a capture, **[D]** derived from two or more [L]
facts, **[I]** inferred.

---

## 0. The answer, up front

| question | answer |
|---|---|
| What is the client missing? | **`UI/UIWindow2.img` does not exist in this client's `Data` tree.** The shop window's constructor loads `UI/UIWindow2.img/Shop2/backgrnd`; `CreateLayout` loads eight more `Shop2/*` nodes. All nine are absent. **[L]** |
| What kills it? | The resource load returns a failing HRESULT, `_com_issue_errorex` throws `_com_error`, and the unwinder runs a cleanup funclet over a stack local the throw prevented from ever being initialised. **[D]** |
| Why are rows not the variable? | The client dies **inside the constructor**, at `140d22867`, *before* `140d22888 call 0x140d23030` — the row decoder is never reached. One row and twelve rows produce the identical failure because no row byte is ever read. **[L]** |
| Is a dialog context required? | **No, and this kills the leading suspicion.** `140d22820 test rcx,rcx / je 0x140d2284a` — a **null** `0x143AA8520` takes the *allocate* path. A script conversation is not a precondition. **[L]** |
| Can the server change any byte to fix this? | **No.** Nothing in `0x0560` selects a resource path. The only server-side lever is not to send `0x0560` at all. **[D]** |
| Was the `0x0560` handler even entered? | It was entered and **never returned**, in **both** runs. **[L]**, §1 |

---

## 1. The measurement nobody had made: the handler never returned

`crates/grap-stub/src/hook.rs:260` calls `tramp(conn, view)` and logs the numbered dispatch
line **after** it returns (`hook.rs:283`). `research/npc-shop-crash.md` §1.1 already
established that a missing numbered line means the handler did not return, not that a write
was lost.

**Neither run has a numbered line for `0x0560`.** [L]

| | run 1 (12 rows) | run 2 (1 row) |
|---|---|---|
| last numbered dispatch | `1107 opcode=0x044F` at `07:51:19.101` | `225 opcode=0x044F` at `09:46:28.876` |
| `0x0560` on the wire | `11:51:21.098` (= hook `07:51:21.098`) | `13:46:30.960` (= hook `09:46:30.960`) |
| numbered line for it | **none** | **none** |
| later packets (`0x0453`) | sent `07:51:22.117`, `07:51:26.669` — **no numbered lines** | sent `09:46:32.994` — **no numbered line** |
| fault | `07:51:24.656` at `0x140ce89f7` | `09:46:33.824` at `0x140ce89d6` |

The counters are complete, so the absence is real: run 1 numbers 0..1107 with no gaps
(1108 lines), run 2 numbers 0..225 (226 lines). The world clock runs exactly 4 h ahead of
the hook clock throughout both captures, which is how the two files are aligned above.

> The client stopped dispatching packets **the moment `0x0560` arrived** and never resumed.

## 2. One throw in the whole session has a different stack, and it is ours

Run 1 logged seven C++ throws. **Throws #3, #4, #5 and #6 share an identical `<-TEXT`
tail and the client survived all four** — they are the routine, caught throw:

```
... 0x142ef3b78<-TEXT  0x140403f6c<-TEXT 0x14023c567<-TEXT 0x1401a5934<-TEXT 0x141e80dae<-TEXT
```

**Throw #7, at `07:51:21.108` — 10 ms after our `0x0560` — is the only one with a different
tail:** [L]

```
... 0x142ef3b78<-TEXT  0x142bfae9d<-TEXT 0x142f0492d<-TEXT 0x142ef4541<-TEXT 0x143ad68f8(?) 0x14177f13a<-TEXT
```

That is a positive and a negative control **in one capture**: the frames the two share are
the exception machinery, and the frames they do not share name the raiser.

Both of throw #7's distinctive `<-TEXT` frames land exactly on the shop's construction
path, and both are **return addresses of specific call instructions**, not approximate:

| frame | `.pdata` | what it is |
|---|---|---|
| `0x14177f13a` | `FUN_14177f0e0 + 0x5a` | the instruction after `14177f135 call 0x142bfac60` **[L]** |
| `0x142bfae9d` | `FUN_142bfac60 + 0x23d` | the instruction after `142bfae98 call 0x142ef3ad0` **[L]** |
| `0x142ef3b78` | `FUN_142ef3ad0 + 0xa8` | the instruction after `142ef3b72 call [rip+0x36f560]`, the raise **[L]** |

Run 2 logged no throw. **That is an instrument artefact and it has since been fixed** — the
run-2 binary predates `THROW_LOG_EARLY_MAX`, added in commit `4e5450c` at 09:51, five
minutes after the run. Do not read run 2's silence as "no throw".

## 3. The path, read off the listing

`research/msexe-shop-openhandler.txt`, `-ctor.txt`, `-uiinit.txt`, `-uibase-setup.txt`.

### 3.1 The handler's non-empty-shop arm is nine instructions of real work **[L]**

```asm
140d22819  mov  rcx, [rip + 0x2d85d00]   ; -> 0x143AA8520, the current-dialog global
140d22820  test rcx, rcx
140d22823  je   0x140d2284a              ; NULL -> allocate.  THIS IS THE PATH WE TAKE
...
140d2284a  mov  edx, 0x14e8
140d22856  call 0x14019b780              ; allocate 0x14e8
140d22862  je   0x140d22871              ; alloc failed -> rsi = 0 -> silent drop
140d22867  call 0x140d22310              ; <<<< THE CONSTRUCTOR. Control never comes back.
140d22876  je   0x140d22fb5              ; ctor returned NULL -> silent drop
140d22888  call 0x140d23030              ; the row decoder - NEVER REACHED
```

Three early exits, and **all three are silent drops, not crashes**. The crash is not an
early exit; it is the `call` at `140d22867` not returning. **[D]**

`0x143AA8520` is the same global `0x055F` checks (`research/npc-shop.md` §3), computed from
both `140d22819` and `140d22841` and agreeing. A null value is *fine* here — it selects the
allocate arm. **The shop does not need a dialog context, and §2 of the brief's suspicion
list is answered "no".** [L]

### 3.2 The constructor's last act is a resource load **[L]**

```asm
140d2241f  call 0x14177f0e0              ; base-UI init
   rdx = 0x14336c940 = L"UI/UIWindow2.img/Shop2/backgrnd"   (dumped, not inferred)
   [rsp+0x28] = 4    -> the same literal 0x055F requires as its dialogKind
```

`14177f0e0` immediately calls `142bfac60`, which is a COM automation call:

```asm
142bfac95  mov  rsi, [rip + 0xee23bc]    ; -> 0x143AE3058, the ResMan singleton (runtime-set)
142bfadf4  mov  rax, [rsi]               ; its vtable
142bfadf7  mov  r10, [rax + 0x48]
142bfae85  call r10                      ; GetObject(path-as-BSTR-VARIANT, ...)
142bfae88  test eax, eax
142bfae8a  jns  0x142bfae9d              ; HRESULT >= 0 -> carry on
142bfae98  call 0x142ef3ad0              ; <<<< THROW.  Return address = 0x142bfae9d
```

`FUN_142ef3ad0` is `_com_issue_errorex`: it `QueryInterface`s for `ISupportErrorInfo`, calls
`InterfaceSupportsErrorInfo` (`[rax+0x18]`), `GetErrorInfo`, then `_com_raise_error` — the
standard `comdef.h` shape, and the last of those returns to `0x142ef3b78`. **[L]**

The surrounding DLLs confirm the layer: `ResMan.dll`, `NameSpace.dll`, `Canvas.dll`,
`PCOM.dll` all ship beside the exe, `cmp word ptr [...], 8` is `VT_BSTR`, and
`add rcx,-4; call [rip+...]` is `SysFreeString`.

### 3.3 The fault is the unwinder, and the funclet names its own victim **[L]**

Both runs' fault stacks are byte-identical apart from the fault address:

```
0x14308e3ed<-TEXT 0x143ae2d20(?) 0x142f04320<-TEXT 0x14308e3db<-TEXT 0x14374e6fc(?)
0x142efba4e<-TEXT 0x14308e3db<-TEXT 0x1437949ba(?) 0x14374e6fc(?) ... 0x142ef8e98<-TEXT
```

`0x14308e3db` is the first of **49 contiguous MSVC cleanup funclets**
(`research/msexe-shop-unwind-funclets.txt`), each `push rbp / sub rsp,0x20 / mov rbp,rdx /
lea rcx,[rbp+N] / call <dtor> / ret`. The first one is:

```asm
14308e3db  push rbp
14308e3e1  mov  rbp, rdx            ; rdx = the establisher frame, supplied by the unwinder
14308e3e4  lea  rcx, [rbp + 0x50]
14308e3e8  call 0x140ce89c0         ; <<<< the function that faults
14308e3ed  add  rsp, 0x20           ; <<<< the frame in BOTH fault stacks
```

The `(?)` frames are `.rdata`/`.data`, and two of them are `UNWIND_INFO` records:
`0x14374e6fc` is `FUN_1415d6a50`'s unwind record +0x10, and `0x1437949ba` is
`FUN_14181fcd0`'s +0x32. Those are values an **unwinder** holds, not call frames. **[L]**

That closes `research/setfield-fault-shape.md` from the other end. It said the local was at
`[RSP+0x50]` in `FUN_1415d6a50`; the funclet says `[rbp+0x50]`, and the unwind record in the
stack says `FUN_1415d6a50`. Two independent facts, same frame. So:

> `0x140ce89d6` means **"an exception unwound through `FUN_1415d6a50` before its `+0x50`
> local was initialised"**. It still says nothing about which subsystem threw — but this
> time we have the throw.

Unwind flags, read from `.xdata` (control: `FUN_142bfac60` builds BSTRs around a throwing
call and must show cleanup — it does):

| function | flags |
|---|---|
| `FUN_142bfac60` | `UHANDLER` (cleanup) — the control |
| `FUN_14177f0e0` | **none** — passes the exception straight through |
| `FUN_140d22310` (ctor) | `UHANDLER` |
| `FUN_140d225f0` (`0x0560`) | `EHANDLER` + `UHANDLER` |
| `FUN_141820080` (`CField::OnPacket`) | `EHANDLER` + `UHANDLER` |

**This matters for our own innocence.** There are two `EHANDLER` frames between the throw
and `crates/grap-stub`'s Rust trampoline, so the exception does not have to unwind through
Rust to reach a handler. Our hook is very unlikely to be what turns a caught exception into
a fatal one. **[D]** It is not proof — `EHANDLER` says a handler *exists*, not that it
matched — but the direction is clear and it was worth checking.

---

## 4. `UIWindow2.img` is not in the WZ

### 4.1 The instrument, and the search that could not work

A raw byte-grep for `UIWindow2` over the `.wz` files finds nothing — **and it also finds
nothing for `UIWindow`, which is certainly there.** The positive control fails, so the grep
is not evidence of anything. WZ names are encoded; only the decoder can read them.

`wz-dump` (prebuilt `target/release/wz-dump.exe`, 2026-08-16 — **no `cargo` was run**, so no
build race with the other agents) over **all 205 archives, 10 021 images**, at depth 8: [L]

```
client-patched/Data/Etc/Language/es/UI/UI_000.wz   [IMG] UIWindow.img
client-patched/Data/UI/UI_000.wz                   [IMG] UIWindow.img
client-patched/Data/UI/_Canvas/_Canvas_000.wz      [IMG] UIWindow.img
```

Three hits for `UIWindow`, **zero for `UIWindow2`**. The positive control is in the same
output. This is an enumeration of every image in every archive, not a search of a known list.

`UIWindow.img` itself has exactly two top-level nodes, `Quest` and `FloatNotice` — no `Shop`.

Same result against the **original, unpatched** client data at
`C:\Nexon\Library\maplestorycw\appdata\Data\UI\`: `UIShop.img` and `UIWindow.img` present,
`UIWindow2.img` absent. So this is not something `client-patched/` did. **[L]**

`Base.wz` holds only `smap.img`, `StandardPDD.img`, `zmap.img` — no UI link table. And
`ResMan.dll`, `NameSpace.dll`, `Canvas.dll`, `PCOM.dll`, `grap64.dll` contain **zero**
occurrences of `UIWindow2`, `UIShop` or `Shop2` in any encoding, where the same scan finds
**595** in `MapleStory.exe` — so there is no string-level path remap in the support DLLs.
**[L]** (A table-driven remap with no literals is not excluded. **[I]** that there is none.)

### 4.2 All nine of the shop's resources are in the missing image

`tools/xref.py --string`, one call per path; control is the ctor's own `backgrnd`, which
comes back as the single `lea` at `0x140d22415`:

| loaded by | path |
|---|---|
| `FUN_140d22310` (ctor) `0x140d22415` | `UI/UIWindow2.img/Shop2/backgrnd` |
| `FUN_140d23fb0` (CreateLayout) `0x140d24131` | `UI/UIWindow2.img/Shop2/select` |
| `0x140d24462` | `UI/UIWindow2.img/Shop2/select2` |
| `0x140d246d3` | `UI/UIWindow2.img/Shop2/BtExit` |
| `0x140d2479b` | `UI/UIWindow2.img/Shop2/BtBuy` |
| `0x140d2485d` | `UI/UIWindow2.img/Shop2/BtSell` |
| `0x140d24b4a` | `UI/UIWindow2.img/Shop2/TabBuy/disabled` |
| `0x140d24e44` | `UI/UIWindow2.img/Shop2/TabBuy/enabled` |
| `0x140d25495` | `UI/UIWindow2.img/Shop2/TabSell` |

**A one-string binary patch cannot fix this.** Nine nodes, and eight of them are in a
function the constructor has not even reached yet.

### 4.3 What the WZ ships instead

`UI/UIShop.img` exists in both `UI_000.wz` and `_Canvas_000.wz`, and its **only** top-level
node is `Shop`, whose children are `backgrnd`, `select`, `disabledBack`, `icon/lock`,
`icon/disabled`, `checkBox/0`, `checkBox/1`, `TabSell/enabled`, `TabSell/disabled`,
`TabBuy/enabled`, `TabBuy/disabled`, `meso`, … — the **classic-layout equivalent** of
`Shop2`. **[L]**

The exe holds two copies of the literal `UI/UIShop.img/Shop` (`0x14341cb68`, `0x14341cff8`)
and one of `UI/UIShop.img/Shop/BtRecharge`. **Who uses them is not established.**
`tools/xref.py` and `tools/dataref.py` both return 0, and an aligned-qword scan for pointer
slots also returns 0 — but that scan's control fails too: `UI/Item.img` (`0x143454200`),
which the bag certainly uses, comes back 0 from all three. So **all three instruments are
blind to whatever reaches these strings**, and "0 references" here is a property of the
search. Do not conclude the classic shop UI is dead code; it is **unmeasured**.

### 4.4 Why the rest of the UI still works

The exe references `UIWindow2.img` in **495 distinct UTF-16 paths and 83 ASCII ones**, and
`UIWindow4/5/6/7/8.img` in another ~350 UTF-16 — none of those images exist either. So the
shop is not unique; it is the first one our server has told the client to open. **[L]**

Of the 159 call sites of `FUN_14177f0e0` in 154 functions, 46 pass a path resolvable from
the listing. Every `UIWindow2/4/8.img` one is a **modern** window — Auction, MesoMarket,
Trunk, MemberShop, MiracleCube, AP/SP Reset, ExOptTransfer, GuildBoard — none reachable in
this client from anything we send. Three of the 46 point at images that **do** exist
(`Login.img`, `LieDetector.img`, `Megaphone.img`), so the code path itself is sound when the
art is present. **[L]**

**The falsification test that matters, and it passed.** The NPC dialogue box *does* render
on screen (`research/fixtures/npcs-visible-quests-clicked-world.log`, and `0x055B` in run 2
of the earlier capture). If `CUIScriptMsg` loaded a `UIWindow2.img` path, everything above
would be wrong. Its constructor is `FUN_142a57d30`, the same handler's zero-row arm calls it
at `140d22686`, and it has **zero string `lea`s and does not call `0x14177f0e0` at all** — it
calls a different base ctor, `0x14177ff90`. **[L]** The dialogue and the shop differ exactly
where this file says they do.

---

## 5. What is still not established

* **The 2.9–3.5 s gap between the throw and the fault.** The throw is at +10 ms; the fault
  at +3.548 s (run 1) and +2.864 s (run 2). Two candidates, and this capture does not
  separate them: **(a)** MSVC's two-phase search walking a stack full of Themida VM frames
  with no usable unwind data, and **(b)** an `EHANDLER` catching, doing something slow, and
  rethrowing. Against (b): if any frame had caught *and returned*, the hook would have logged
  a numbered dispatch line, and it did not (§1). There was exactly **one** throw in that
  window, so there is no retry loop of throws. **[I]** that it is (a).
* **The exact HRESULT.** Nothing on disk carries it. §6 watch 1 gets it.
* **Whether `ResMan` fails a missing node with an error HRESULT rather than S_OK + null.**
  Inferred from the throw landing on the `jns`-failed branch 10 ms after the packet, which is
  strong, but not read from `ResMan.dll`. **[D]**
* **Who uses `UI/UIShop.img/Shop`** — §4.3. Three instruments, three blind zeros, one failed
  control.
* **No ELog.** Run 2's `login.log` carries no `0x008F`/`0x0090`, so run 1 wrote none.
  Control: the same grep finds ELogs in ten other fixtures, including
  `research/fixtures/amherst-1013-heap-corruption-login.log`. A genuine negative. **[L]**

## 6. What to do, and what a run would add

### The fix, and it is server-side by subtraction

**Stop sending `0x0560`.** There is no byte in it that selects a resource path, so no shop
packet this server can build will make the client construct that window. Answer the NPC click
with the `0x055B` dialogue that already works, and leave `net::shop` built-and-unwired with
this file named beside it (`CLAUDE.md`, "Built is not wired" — record it loudly).

Two things **not** to reach for:

* A zero-row `0x0560` (`research/npc-shop.md` §2.2) does avoid the constructor and builds a
  `CUIScriptMsg` instead — but it is strictly worse than `0x055B`: same dialog box, plus a
  `0x0104` sub-op 2 to handle, plus a new untested path. **[D]**
* Patching the nine `lea`s in the client. Eight of the nine target strings do not exist as
  literals in the image, and `UIShop.img/Shop` has no `BtBuy`/`BtSell`/`BtExit`/`select2`
  node to point them at. This is not a small patch. **[D]**

Longer term, the only route to a real shop on this client is authoring a `UIWindow2.img`
with `_outlink`s into `UIShop.img` — and `crates/wz` only reads. That is the owner's call, not
a bug fix.

### If a launch is spent, these three watches make the chain measured

Three slots are free (`1415db360:ret`, `141b2a280:rdx=0`, `140304100:hits=200` stay).
Send the same one-row `0x0560` as run 2 — **change nothing else**.

| # | watch | what it answers |
|---|---|---|
| 1 | `watch@142ef3ad0:hits=20` | `_com_issue_errorex`. `rcx` is the failing **HRESULT**, `called-from` the exact site. **Expect `called-from=0x142bfae9d`.** It fires *before* the throw, so the throw-log window cannot hide it |
| 2 | `watch@14177f0e0:hits=12` | the base-UI init. **`called-from=0x140d22424`** is the shop constructor and nothing else — the path is then known statically from §4.2. It also proves the two earlier ctor calls survived |
| 3 | `watch@140d23030:hits=5` | the row decoder. **Expect zero hits** — that turns "rows are not the variable" from a listing inference into a measurement |

Watch 3 is a negative, so it needs watch 2 as its control: **watch 2 firing proves the
handler reached the constructor**, and watch 3 not firing then means the rows were never
read. Watch 2 firing and watch 1 not firing would falsify §3.2 and send this file back.

All three are function entries, so the probe's int3 lands on an instruction boundary.

**Do not expect the path string in the log.** `deref_wstr(reg)` reads `*reg` and *then*
walks a wide string, i.e. it wants a pointer-to-pointer; `rdx` at `14177f0e0` is the string
itself, so `deref_wstr` will print nothing and `deref(rdx)` will print `[0x00490055]` — the
UTF-16 `"UI"`, which is a weak confirmation at best. `called-from` is the load-bearing
field on watches 2 and 1 alike.

**This run is optional.** The static chain is five exact return addresses plus an
enumeration of 10 021 WZ images, and its outcome is fully predicted. Spend the launch on it
only if the coordinator wants certainty before removing shop support; otherwise apply §6's
fix and spend the launch elsewhere.

---

## 7. What this changes elsewhere

Nothing in `research/npc-shop.md` is falsified. Its §7 open item *"Whether `0x0560` may be
sent while an NPC dialogue is open"* is now partly answered — a **null** dialog global is the
normal, working case (§3.1) — and its §6 test table's row 1, *"A freeze or a fault = the row
is mis-sized"*, is **wrong for this failure**: the fault happens with a correctly sized row,
before any row is read. That is a prediction that did not hold, not a decoding error.

`research/npc-shop-crash.md` is about a different crash (map 1013 heap corruption, exit code
`0xC0000374`) and is untouched. Note the contrast: that one never dispatched its packet;
this one dispatched and never returned, with exit code `0xC0000005`.

`STATUS.md` should record that goal F (shops) is **blocked on client data, not on the
protocol** — the packet is correct and the window it asks for cannot be built.

## 8. Evidence kept

Run 2 was still in the repo root and is now in `research/fixtures/` before it rolled:

```
research/fixtures/shop-1row-still-faults-{world,hook,exit,login}.log
```

Run 1 was already there as `shop-opens-then-client-faults-{world,hook,exit}.log`.

New listings: `research/msexe-shop-openhandler.txt` (the `0x0560` handler),
`-ctor.txt` (the constructor), `-uiinit.txt` (`FUN_14177f0e0`),
`-uibase-setup.txt` (`FUN_142bfac60`, the COM resource call),
`-unwind-funclets.txt` (the 49 cleanup funclets, the first of which faults).
