# The `G`/`H` gate: which premise broke

**Verdict: premise 1 is wrong.** The `ELog` record for site `840` (`0x348`) is *not* evidence
about the connection that uploaded it. The upload is a **file replay** — `FUN_1415dde80`
opens a log file with `OPEN_EXISTING`, reads it, closes it and calls a one-argument API on
the path — so the records it sends were written by an **earlier failure**, not by the
handshake in progress. On the run that produced `login.log`, the `G`/`H` gate **passed**:
`conn+0x48` was non-zero, the `A..F` block *was* parsed, and `G`/`H` read as `1`/`1`.

Everything in `docs/handshake.md`'s "CORRECTION 2026-08-19" section, and the sentence in
`docs/transport.md` about the login connection "taking our IVs and then throwing", is
**wrong for the login connection**. It remains the right explanation for a *channel*
connection, which is where `conn+0x48 == 0` actually happens.

---

## The proof, in five measured steps

Each step is labelled **measured** (read off the binary or off `login.log`) or **inferred**.

### 1. The raise at `0x348` throws, unconditionally — **measured**

```
FUN_140cc2350  @ 0x140cc2350  (105 bytes)
    -> FUN_141804870 at 0x140cc2390, return address 0x140cc2394   (matches the ELog stack)

FUN_141804870  @ 0x141804870 .. 0x14180496a
    1418048cd  CALL 0x141804c20        ; FUN_141804c20(2, "throw CTerminateException", ...)
                                       ;   -> writes the ELog record, with the site id
    141804953  CALL 0x141804380        ; CTerminateException ctor (return addr 0x141804958,
                                       ;   = the ELog stack's FUN_141804870+0xE8 frame)
    141804964  CALL 0x142ef6d4c        ; _CxxThrowException
    141804969  cc                      ; INT3 - the function never returns
```

Disassembling the whole 250 bytes finds **no conditional jump** on that path. So any call
to `FUN_140cc2350` writes the record *and then throws*. The string
`"throw CTerminateException"` is a literal at `0x1433d5e74` passed to the ELog writer, which
is why it appears verbatim in every record.

### 2. `FUN_1415d10e0` cannot catch its own throw — **measured**

`.pdata` for `FUN_1415d10e0` is a single entry, `0x1415d10e0 .. 0x1415d31a1`, with
`EHANDLER|UHANDLER` and handler `0x142ef43c0` (the C++ frame handler). On x64 MSVC a
`catch` is compiled as a **funclet** with its own `RUNTIME_FUNCTION` whose `UNWIND_INFO`
carries `UNW_FLAG_CHAININFO` back to the parent. Scanning all 121002 `.pdata` entries:

| function | chained funclets |
|---|---|
| `FUN_1415d10e0` (`0x1415d10e0..0x1415d31a1`) | **0** |
| `FUN_1415d3990` (the send path) | 0 |
| `0x142c8c8f0` (the window procedure) | 0 |
| **control:** `0x142c8c5b0..0x142c8c601` | **3** |

The image contains 19041 chained entries in total and the control returns three of them, so
the instrument demonstrably produces positives. `FUN_1415d10e0`'s `EHANDLER` flag is the
ordinary "this frame has C++ objects to destroy while unwinding" case, not a `catch`.

**Therefore: if the `0x348` raise fires, nothing after `0x1415d277f` in `FUN_1415d10e0` runs.**

### 3. But the code after it demonstrably ran — **measured, off `login.log`**

```
1415d28bd  CALL 0x1415d5b40           ; packet 0x70
1415d28c2  MOV RAX,[RSP+0x2300]       ; conn
1415d28ca  CMP dword ptr [RAX+0x48],0
1415d28ce  JZ  0x1415d297c            ; channel branch (packet 0x7D)
1415d28d4  MOV RCX,[RSP+0x2300]
1415d28dc  CALL 0x1415d5c20           ; packet 0x71   <-- login branch
1415d28e1  CALL 0x1415dde80           ; packet 0x8F   ELog upload
1415d28e6  CALL 0x1415ddf60           ; packet 0x90   ELog call-stack upload
1415d28eb  CALL 0x1415de040           ; packet 0x91
```

Ghidra `Xrefs` on `FUN_1415d5b40`, `FUN_1415d5c20`, `FUN_1415dde80` and `FUN_1415ddf60`
returns **exactly one reference each** — the four call sites above — and no data
references, so there is no vtable or function-pointer route to any of them.

`login.log` (02:41:28) contains, in order, `0x0070`, `0x0071`, `0x008F`, `0x0090` — the same
order as the four call sites. And the bodies match the builders field for field:

```
0x0071, 49-byte body:
    01            u8   1
    01000000      u32  1
    64000000      u32  100
    00 00 00 00 01                     five u8s
    1e000000 <30 bytes>                the trailer FUN_1415dc6f0 appends
```

which is `FUN_1415d5c20` exactly. The one other `0x71` builder, `FUN_141b21ea0` (reached only
through a vtable slot at `0x1433fd670`), is ruled out by its body: at `0x141b23f2a` it opens
the packet and immediately writes a **blob** (`FUN_1406ede20`), then a string, then a 16-byte
GUID — it never writes the leading `u8` the observed packet starts with. The other two `0x70`
builders are ruled out the same way: `FUN_1415d8b20` writes `u8 4` and `FUN_140c8fdc0`
writes `u8 5`, against the observed `02`.

So the client executed `0x1415d28dc` — 0x15D bytes *past* the raise, straight-line, in the
same frame — with `conn+0x48 != 0`.

### 4. The parse can only run once per connection — **measured**

`FUN_1415d10e0` has three callers:

| call site | in | arguments |
|---|---|---|
| `0x142c8d6cd` | the window procedure `0x142c8c8f0` | `(conn, 1, 0)` |
| `0x1415d16f1` | itself, line 320 | `(conn, 0, 0)` |
| `0x1415d3445` | `FUN_1415d33c0` (reset/reconnect) | `(conn, 0, 0)` |

The parse block, and everything in steps 1 and 3, live in the `param_2 != 0` branch. Only
the window procedure passes `param_2 != 0`, and it does so from **one** dispatch arm:

```
142c8d68b  MOVZX EDX,AX          ; HIWORD(lParam) = WSA error
142c8d68e  MOVZX R8D,BX          ; LOWORD(lParam) = the network event
142c8d692  TEST EDX,EDX
142c8d696  CALL 0x1415d33c0      ; error != 0 -> reset
142c8d6a0  SUB R8D,1 / JZ  ->    CALL 0x1415d31b0   ; FD_READ   (1)
142c8d6aa  SUB R8D,1 / JZ  ->    CALL 0x1415d5510   ; FD_WRITE  (2)
142c8d6ac  SUB R8D,0xE / JZ ->   0x142c8d6c6        ; FD_CONNECT (0x10)
142c8d6c6  XOR R8D,R8D
142c8d6c9  LEA EDX,[R8+1]
142c8d6cd  CALL 0x1415d10e0      ; FUN_1415d10e0(conn, 1, 0)
142c8d6b2  CMP R8D,0x10 / JNZ -> CALL 0x1415d3530   ; FD_CLOSE  (0x20)
```

**`FD_READ` does not go to the handshake at all.** The greeting is read on the *`FD_CONNECT`*
notification, by the blocking retry loop inside `FUN_1415d10e0` — which is also the
explanation for the 514 ms between our greeting (02:41:27.771) and the client's first packet
(02:41:28.285): `FD_CONNECT` fires before our bytes land, the first `recv` returns
`WSAEWOULDBLOCK` (`0x2733`), and the loop does `Sleep(500)` and retries.

So one TCP connection ⇒ one `FD_CONNECT` ⇒ **one** pass through the parse and the gate.
`login.log` shows exactly one connection (`#1 127.0.0.1:51865`) and one greeting.

Steps 1–4 are jointly inconsistent with the raise having fired on that connection.

### 5. The ELog upload replays a file — **measured (the file I/O); inferred (that it deletes)**

`FUN_1415dde80` (packet `0x8F`) and `FUN_1415ddf60` (packet `0x90`) both do:

```c
path = FUN_142e56b40();                    // a std::string, built once behind InitOnce
FUN_1415ddd10(&blob, path, 0x2000);        // read the file
if (FUN_1415e3fd0(&blob) == 0) { build packet 0x8F/0x90 with the bytes; send; }
```

and `FUN_1415ddd10` is:

```c
CreateFile(path, GENERIC_READ, FILE_SHARE_READ, ..., OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL)
size = GetFileSize(...)
if (size != 0 && size < 0x2000) ReadFile(...)
CloseHandle(...)
(*DAT_143ad55c0)(path)                     // 0x1415dde1f: one argument, the path
```

`DAT_143ad55c0` is a runtime-resolved API slot (this binary's import table is stripped down
to seven names, so it cannot be named statically). It is called on a path immediately after
the file is read and closed, and the same slot is used by `FUN_1415e5c20`, the `Data.wz`
patch handler. `DeleteFileW` is the obvious fit — **inferred**, and not load-bearing: what
matters is only that the contents come off **disk**, so they can predate the connection that
uploads them.

That closes the loop. A run whose handshake dies at the `0x348` raise never reaches
`0x1415d28e1`, so it never uploads its own record; the record sits in the file until the
**next** run gets a clean handshake and uploads it there. `Time1`/`Time2` in the record are
fields the *writer* stamped, not upload times, so their 16 ms spread says nothing about
when the upload happened — that was the trap.

### Bonus: site `1327` (`0x52F`), the "raise site this document has never accounted for"

`0x52F` is passed at exactly two places in `.text`; the relevant one is `0x142c462ec`, inside
the application's message pump (`.pdata` `0x142c45e50..0x142c46b75` — the same function whose
frame `0x142c4612b` sits under `DispatchMessageW` in the uploaded call stack):

```c
TranslateMessage(&msg);
DispatchMessageW(&msg);
if (FUN_142c49fa0(app) != 0) {                 // an HR was recorded during dispatch
    name = FUN_1418039d0(hr);
    if (0x20FFFFFF < hr && hr < 0x2100000F) FUN_142c87860(&DAT_143271f04, 0x527, hr, &name);
    if (0x21FFFFFF < hr && hr < 0x22000011) {
        if (hr == 0x22000001 || hr == 0x22000002) {
            FUN_141804380(&exc, ..., 0x52C, hr);
            _CxxThrowException(&exc, &DAT_143a3bd40);      // fatal
        }
        FUN_142c87900(&DAT_143271f04, 0x52F, hr, &name);   // <-- site 1327, logged only
    }
    FUN_142c879a0(&DAT_143271f04);
}
```

So `840` and `1327` are **one event, reported twice**: the raise inside the handshake, and
the pump's report of the HR after the message that carried it returned. `0x22000007` falls
in the "log it and keep pumping" bucket, which is why the client survived the failure that
wrote those records — but it survived *in the message loop*, with the handshake abandoned.
The pair is therefore the signature of a handshake that **died**, which is the opposite of
what `docs/handshake.md` concluded from it.

---

## Answers to the specific questions

### (a) Where is `conn+0x48` written?

**`FUN_1415d35f0` is not the writer** — measured. The whole function is 113 bytes:

```c
void FUN_1415d35f0(longlong conn) {
  if (FUN_1415e4080(conn) != 0) FUN_142cf4350(FUN_140caa510());
  FUN_1415d5aa0(conn);
  if (*(int *)(conn + 0xc) == 4) FUN_140c91e30();
  *(undefined4 *)(conn + 0xc) = 0;      // <- the only two stores
  *(undefined1 *)(conn + 0x10) = 0;
  FUN_1415e3b60(conn + 0x20);
  FUN_1415db820();
}
```

It resets the *locale/state* field at `+0x0c` (which `FUN_1415d10e0` writes from field `O`
at line 638), not `+0x48`.

**`FUN_1415d10e0` never writes `conn+0x48` either** — measured. A displacement scan over its
whole 8385-byte range for stores with `disp8 == 0x48` finds only two entries, both
`mov [rsp+…+0x48], ecx` (SIB base = RSP), i.e. stack traffic. So `conn+0x48` is **constant
for the entire invocation**: the value the two parse gates use at lines 338 and 390 is the
same value tested at `0x1415d28ca`.

**Its value on the login connection is non-zero** — measured, from the wire: the observed
`0x0071` can only be sent from `0x1415d28dc`, which is the `!= 0` arm of `CMP [RAX+0x48],0`.
Combined with the paragraph above, that is a direct measurement that both gated blocks were
parsed on this connection.

**Where it is set is still unknown.** A `disp8` store to `[reg+0x48]` with a non-`rsp`/`rbp`
base does not occur anywhere in the connection module (`0x1415d0000..0x1415f0000`) except
`0x1415e243c`, which lies outside any `.pdata` function, and a `disp32` form of the same
store does not occur anywhere in `.text` at all — that last scan has a **positive control**:
switching the target displacement to `0x140` finds exactly the two `conn+0x140 = 1` stores at
`0x1415d2530` and `0x1415d2688` that `FUN_1415d10e0` is known to make. So the field is most
likely initialised in bulk (a constructor `memset`/struct copy) or through a base register
form my scan excludes. Cheapest way to settle it: one client run with
`crates/grap-stub`'s `watch@1415d10e0:peek=48`.

### (b) Is the raise in the same scope as the parse?

**Yes — same scope, straight-line, no jump table involved.** The decompiler's brace structure
is correct here, and the raw disassembly confirms both the condition and the fall-through:

```
1415d2754  CALL 0x1415dd2d0                  ; line 604, FUN_1415dd2d0()
1415d2759  CMP dword ptr [RSP+0x128],0x1     ; H  == local_21d0[0]
1415d2761  JNZ 0x1415d276d                   ; -> raise
1415d2763  MOVZX EAX,word ptr [RSP+0x60]     ; G  == local_2298[0]
1415d2768  CMP EAX,0x1
1415d276b  JZ  0x1415d2784                   ; -> skip the raise
1415d276d  MOV R8D,0x22000007
1415d2773  MOV EDX,0x348
1415d2778  LEA RCX,[0x143271f04]
1415d277f  CALL 0x140cc2350
1415d2784  CMP dword ptr [RSP+0x114],0x0     ; line 609, local_21e4 (the 0x8000 flag)
```

The stack-slot mapping is exact: Ghidra's `local_NNNN` sits at `RSP + (0x22F8 - NNNN)`, so
`local_2298 -> [RSP+0x60]`, `local_21d0 -> [RSP+0x128]`, `local_21e4 -> [RSP+0x114]`. All
three match. There is **no** path that reaches `0x1415d276d` with `G`/`H` still at their
line 327–328 zero-initialisation: the only way into the gate is the fall-through from
`0x1415d2754`, which is inside the `local_2270 != 0` branch, i.e. after the parse. (The
`JMP 0x1415d2754` at `0x1415d26ea` is the "skip the `0x33b` version raise" jump from a few
lines earlier — still inside the same branch.)

The apparent inconsistency in the source, that a call which never returns is followed by
more code, is just MSVC not knowing `FUN_140cc2350` is `noreturn`: the throw is two frames
deeper.

### (c) How many times is `FUN_1415d10e0` called per connection?

Twice at most, and **the parse runs at most once**. See step 4 above. In detail:

* `FD_CONNECT` → `FUN_1415d10e0(conn, 1, 0)` — this is the one that reads and parses the
  greeting.
* **All three call sites pass `param_3 = 0`** (`XOR R8D,R8D` at `0x142c8d6c6`, and the two
  literal `(param_1, 0, 0)` calls). So the connection always takes the **"Second Connect"**
  branch, `low <= 100 <= high`, and the "First Connect" branch at line 558 onward — including
  the `high == 100` check `docs/handshake.md` lists as a gate — is **dead code on every path
  reachable from `.text`**. Only `high >= 100` and `low <= 100` are ever enforced.
* The self-recursion at line 320 and `FUN_1415d33c0`'s call both pass `param_2 == 0`, which
  enters the *connect* branch and returns at line 257 — no read, no parse, no gate, no
  `0x70`.
* A second framed message could not be re-parsed as a greeting anyway: our `0x0032` frame
  starts `CE AD` (`a = (K >> 16) ^ 0xFFFE = 0xADCE`), and `0xADCE > 0x5B4`, so the length
  check at line 304 aborts to `local_2270 = 0` and the recursion.

So there is no "called again with a short buffer" path. **Inferred but tight**: the only
data-carrying entry is `FD_CONNECT`, and a socket gets one of those.

### (d) Is the cipher-mode `param_1+0x48` the same object as `FUN_1415d10e0`'s `conn`?

**Yes — measured.** `FUN_1406e99e0(buf, iv, mode)` has exactly two callers, and both read
`+0x48` off a pointer they also hand to `FUN_1415d33c0`, which hands it unchanged to
`FUN_1415d10e0`:

```c
// FUN_1415d36c0(conn)                          call at 0x1415d3903
if (*(int *)(conn + 0x48) == 0) mode = 2; else mode = 1;
...
FUN_1406e88d0(local_38, conn + 0xb0);
FUN_1406e99e0(local_38, *(undefined4 *)(conn + 0xec), mode);
...
FUN_1415d33c0(conn, 0, 0);                   // on a framing error

// FUN_1415e7090(conn)                          call at 0x1415e789f
type = *(int *)(conn + 0x48);
FUN_1406e88d0(local_588, conn + 0xb0);
FUN_1406e99e0(local_588, *(undefined4 *)(conn + 0xec), 2 - (uint)(type != 0));
...
FUN_1415d33c0(conn, 0, 0);
```

`FUN_1415d33c0(p, …)` calls `FUN_1415d10e0(p, 0, 0)` with the pointer unchanged (no offset
arithmetic anywhere in the chain), so the `+0x48` read for the cipher mode is literally the
same field the parse gates read. Two independent corroborations of the struct identity:
both functions also use `conn+0xec`, which is where `FUN_1415d10e0` line 427 stores field
`K`; and both use `conn+0xb0`, `conn+0x70`, `conn+0x150` — the same object `FUN_1415d10e0`
touches at `+0x150`.

`docs/transport.md`'s claim is therefore **correct**, including the object identity. (Its
`FUN_1406e9a65` reference is an address *inside* `FUN_1406e99e0`, which is where the mode
argument is finally branched on; the selection itself is in the two callers.)

---

## What this changes

1. **The login handshake is not failing.** Delete the "CORRECTION 2026-08-19" verdict in
   `docs/handshake.md` and the "Why the login connection also logs it" paragraph. The
   greeting in `crates/net/src/handshake.rs` satisfies every gate, `conn+0x48 != 0`, and both
   gated blocks are parsed. `docs/transport.md`'s last section ("Why 'the transport works'
   never proved the handshake was right") is wrong as written — the transport working *and*
   the handshake tail running are the same evidence, because `0x70`/`0x71`/`0x8F`/`0x90` are
   all emitted after the gate.
2. **The channel-connection theory survives intact.** `conn+0x48 == 0` really does skip
   `A..F`, really does make the client read `G` from where we put `A`, and really does raise
   `0x348`. The channel greeting shape proposed in `docs/transport.md` is still the right
   thing to build — and now there is a positive prediction to test it against: if it is
   wrong, the *next* run's `0x008F` will carry a fresh `840`/`1327` pair; if it is right,
   `0x008F` will be absent or empty.
3. **`decode_elog.py` needs a health warning.** It reports what the *previous* failure was,
   not the current one. Two consequences: a clean run can carry a stale pair, and a run that
   fails leaves nothing to read until the run after it. Since the file is (very likely)
   deleted on upload, the reliable protocol is: get a clean run to drain the log, then do the
   experiment, then get another clean run to read the verdict.
4. **The record's `Socket|127.0.0.1:8484` does not identify the failing socket.** It cannot,
   given the record can outlive the process. It is a stored field like `WID`, `Channel` and
   `JOB`, all of which are also at their null values in these records.

## Still unknown

* **Which run wrote the `840`/`1327` pair.** The record's `Socket` field says `8484`, which
  points at a probe-era run (`handshake_probe.py` listens on 8484 and sent `G = 0` for
  weeks) rather than at the migrated-channel run on 8485 — but that field is unreliable per
  point 4, so this is not settled. It does not matter for the conclusion.
* **Where `conn+0x48` is written.** See (a). One `watch@1415d10e0:peek=48` run settles both
  the value and, with `-Session`, the writer.
* **Whether `DAT_143ad55c0` is `DeleteFileW`.** Only reachable dynamically; the imports are
  resolved at runtime. Not load-bearing.
* **Where the `CTerminateException` from the wndproc is actually caught.** Neither
  `FUN_1415d10e0`, nor the wndproc `0x142c8c8f0`, nor `FUN_142c45e50` has a catch funclet, so
  it happens outside all three (the `DispatchMessageW` callback boundary is the likely place).
  The pump's site-`0x52F` report proves the HR is recorded and the loop continues; the exact
  catch site is not needed for anything here.
* **The second `0x0070`** (02:41:28.800, body starts `01 64000000`) matches none of the three
  known `0x70` builders (`FUN_1415d5b40` writes `u8 2`, `FUN_1415d8b20` writes `4`,
  `FUN_140c8fdc0` writes `5`). `DumpOpcodes` resolved 1881 of 1894 call sites, so a fourth
  builder is probably among the 13 it could not. Unrelated to this question, noted so it is
  not rediscovered.

## Instruments and their controls

Per the project rule that a negative from an unproven instrument means nothing:

| instrument | control | control result |
|---|---|---|
| `.pdata` funclet scan (new, scratch) | parent `0x142c8c5b0..0x142c8c601`, known to have funclets | **3 found** of 19041 image-wide |
| `disp32` struct-store scan (new, scratch) | target `0x140` inside `FUN_1415d10e0` | **2 found**, exactly the two `conn+0x140 = 1` stores |
| `mov r8d,<hr>; mov edx,<site>` scan (new, scratch) | must find the sites already known in `FUN_1415d10e0` | **16 found**, including `0x23d 0x243 0x2df 0x327 0x33b 0x348 0x34e 0x351` at the expected addresses |
| Ghidra `Xrefs` | `FUN_140cc2350`, expected to be widely called | 23 callers, including both `FUN_1415d10e0` sites |
| `disp8` struct-store scan | — | found a genuine `mov dword [rcx+0x48],0x3a` at `0x1415e7c34`, so it can hit; but it is noisy and its negative for the connection object is weaker than the `disp32` one |

The `mov r8d/mov edx` scan is pattern-limited: a site that materialises either constant
differently would not appear. That limitation does not affect the conclusion, because the
uploaded call stack independently pins the raise to `0x1415d2784` — the return address of
the `call` at `0x1415d277f`.

**Cost: zero client runs.**
