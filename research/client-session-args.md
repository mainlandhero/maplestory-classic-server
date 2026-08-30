# Can the client carry a session? Launch arguments vs. the identity field

Asked by the owner, 2026-08-29: *"figure out a way so that the Nexon client itself accepts a
session from us. Have we truly exhausted all options? ... Are there no way we can hijack the
STEAMSTART or NEXONSTART arguments?"*

Labels: **[L]** measured on the wire or in a log, **[D]** derived from the binary,
**[I]** inferred.

## The short answer

**No launch argument can carry a session, and `NEXONSTART` does not exist.** But the premise
underneath the question is right, and the previous write-ups were incomplete:

> **The client has a session-identity field, it is encoded into `0x0073` as a string, and it
> has a working setter that this build never calls.** `FUN_142c503c0`. Zero callers.

So it is not that the client cannot carry a session. It is that **nothing populates it**.
That distinction changes what the cheapest fix is, and it makes the fix much cheaper than
the standing assumption (see "What it would cost", below - no packet is forged, so the
channel's chained AES is never involved).

## 1. `NEXONSTART` is not in the binary — [L]

```text
NEXONSTART   ascii=0  utf16=0        <- also "NexonStart", "nexonstart": 0
STEAMSTART   ascii=1  utf16=0        <- positive controls, all found where
WEBSTART     ascii=1  utf16=0           docs/launch-protocol.md predicted
-NXLDEBUG    ascii=1  -NXLPTS 1  -NXL 3  GAMELAUNCHING 1  IPPORT 1  GFN 1
```

The complete keyword set is the seven already in `docs/launch-protocol.md`, all in one
`.rdata` block at `0x143493028`. The search instrument is verified by six positive controls
in the same run, so the zero is a real absence and not a property of the search.

## 2. The read/encode map — [D]

`param_1` of the parser `FUN_142c94bd0`. The config is a **stack local in `main`**
(`FUN_142c42f30`'s `local_5c8[208]`), so it has no global to scan; it is reachable only as a
parameter, and `main` passes it to exactly four functions. All four are audited below.

| cfg | set from | who reads it | reaches an encode? |
|---|---|---|---|
| `+0x18` | IP, every mode | the connect path directly | no - it *is* the destination |
| `+0x20` | port, every mode | the connect path directly | no |
| `+0x28` | *never set by the parser* | ctor -> acctmgr `+0x22bc` | no |
| `+0x30` | *never set by the parser* | ctor -> acctmgr `+0x1f8` | no |
| **`+0x38`** | **the mode keyword** | ctor -> `session+0x68` | **YES - `0x0073` field 1** |
| `+0x40`,`+0x48` | *never set by the parser* | `FUN_142e14bc0`, game-log init | no |
| `+0x4c` | *never set by the parser* | ctor -> `session+0x150` | no |
| **`+0x80`** | `-NXL`/`-NXLPTS`/`STEAMSTART` t1 | `FUN_142c95ef0` <- `FUN_142c50390` | **NO - dead, see below** |
| **`+0x88`** | `STEAMSTART` t3 (upper-cased) | ctor -> `session+0x1c0` | **NO - dead** |
| **`+0x90..0xb8`** | six tokens, every mode | `FUN_142c95f20` | **NO - dead** |
| `+0xc0` | region, `-NXL`/`STEAMSTART` t2 | ctor -> acctmgr `+0x28d0` | **NO - no `w_str`** |
| `+0xc8` | `-NXLPTS` flag | `FUN_142c95f70` | **NO - dead** |

**Of everything the command line can set, exactly one value reaches the wire: the mode
`u32`.** It is already 5 and has been in every capture ever taken.

### How "dead" was established, and why it is stronger than the old claim

`docs/launch-protocol.md` rested on Ghidra `Xrefs` for one accessor and **named its own blind
spot**: *"The one path not audited is `FUN_142c926e0` (the 9408-byte startup, which owns the
parser call) reading the array directly."* Both halves are now closed.

**The blind spot, closed.** Every `param_1` touch in `FUN_142c926e0` (`research/msexe-startup.c`)
is in the zero-init prologue at lines 253-279, *before* the parser call at line 324. The only
thing done with `cfg+0x90` is `_eh_vector_constructor_iterator_(param_1+0x12, 8, 6, ...)` -
constructing the six-element array, not reading it. The startup does later overwrite
`cfg+0x18`/`+0x20` (IP/port) from another source at lines 952/965, which is worth knowing but
is not a session path.

**The accessors, enumerated rather than searched.** New tool `tools/callrange.py` asks the
enumerating form of the question - *"who calls anything at all in the accessor cluster"* -
because the leaf accessors are 4-5 byte functions with no `.pdata` entry and cannot be listed
any other way. Its positive control (`0x1402fa9a0 0x1402fa9a1` -> 96 sites, 43 in
`0x140304b20`) reproduces `tools/callers.py`'s documented control exactly.

```text
call rel32 into [0x142c95b00, 0x142c96400): 11 sites, 10 distinct targets
  +0x28 +0x38 +0x30 +0x4c +0x88 +0xc0   <- FUN_142c43db0, the session ctor
  +0x38 +0x40 +0x48                     <- FUN_142e14bc0, game-log init
  +0x58                                 <- FUN_142c50330
  +0x80                                 <- FUN_142c50390   (which itself has 0 callers)
  +0x90                                 <- NOTHING
```

The instrument demonstrably finds ten sibling accessors in that exact range and does not find
`FUN_142c95f20`. That is an internal positive control, in the same run, at the same
addresses.

Then `tools/callers.py` (call + tail `jmp` + qword pointer, three scans) on each terminal:

| accessor | field | call | tail jmp | pointers |
|---|---|---|---|---|
| `FUN_142c95f20` | `cfg+0x90` six tokens | **0** | **0** | **0** |
| `FUN_142c50390` | `cfg+0x80` | **0** | **0** | **0** |
| `FUN_142c50800` | `session+0x1c0` (`cfg+0x88`) | **0** | **0** | **0** |
| `FUN_142c95f70` | `cfg+0xc8` | **0** | **0** | **0** |
| `FUN_142c50400` | `session+0x1b8` | 2 (`FUN_141b21ea0`) | 0 | 0 |
| `FUN_142c50300/310/330/370` | siblings | 1 each | 0 | 0 |

The siblings in the same 0x100-byte run all return callers. The dead ones are dead.

The session vtable is only **two** entries (`PTR_FUN_1434923c0` -> `0x142c89f00`,
`0x142c89ec0`), so none of these is virtual and "no direct callers, therefore virtual" - the
mistake `CLAUDE.md` records - is excluded here by dumping the table rather than assuming.

### `FUN_142c9f2d0` is `_strupr`, not a decode — [D]

The brief hoped this was the strongest hint about what `STEAMSTART` argv[3] is meant to be.
It is 86 bytes and it upper-cases the string in place:

```c
_Str = FUN_14019bd40(param_1, 0, 1);   // writable buffer
_strupr(_Str);
FUN_14019c870(param_1, len);           // set length
```

Exactly the idiom the parser already applies to token 0. It exists so that argv[3] can be
compared against `GFN` case-insensitively. No hash, no base64, no length check.

### `STEAMSTART` argv[3] is a two-value enum, not a free slot — [D]

Even ignoring that `session+0x1c0` is dead, argv[3] cannot hold arbitrary text. After the
upper-case it is compared against `GFN` and against `"-"`; the result sets `iVar14` to 1 or
0, and **IP and port are then read from `token(iVar14+3)` and `token(iVar14+4)`**. Pass
anything else and argv[3] doubles as the server IP. So the only usable values are `GFN` and
`-`.

`cfg+0x80` (`-NXL`/`STEAMSTART` token 1) *is* free-form and validated only for non-emptiness.
It is the best slot the launch line has - and its single reader has no callers.

## 3. The session identity: found, encoded, and never set — [D] + [L]

`0x0073`'s field sequence, from `research/msexe-packet-fields.txt` (the instrument that lists
a builder's calls in order):

```text
0x0073  FUN_141b21ea0
        FUN_142c4a810    -> session+0x68    the launch mode
        FUN_1406ede20    w_raw
        FUN_142c50400    -> session+0x1b8   THE IDENTITY
        FUN_1406edc80    w_str             <-- it is encoded, as a string
        FUN_1406ede20    w_raw              the 16-byte GUID
        u32                                 a tick
        FUN_1406ed610    SEND
```

So the identity is not merely stored somewhere near the packet; it is handed straight to the
string encoder. Same builder emits it in `0x0080` and `0x00D0`.

Every reference to `session+0x1b8` in the whole 52 MB `.text`, after filtering to the session
class and to the 132 functions that touch the singleton `DAT_143ac1898`:

```text
142c440ce  mov  [rdi+0x1b8], r14      FUN_142c43db0   ctor - zeroes it
142c444df  mov  rcx, [rdi+0x1b8]      FUN_142c44350   dtor - frees it (ptr-0x10 idiom)
142c50418  mov  rbp, [rcx+0x1b8]      FUN_142c50400   getter - the 0x0073 field
142c503cd  add  rcx, 0x1b8            FUN_142c503c0   SETTER  <-- 0 callers
```

`FUN_142c503c0` is `SetSessionIdentity(this, src)`: it computes `this+0x1b8` and calls the
string-assign helper `FUN_14019a260(dest, src)`. The argument order is not assumed - the
sibling getter `FUN_142c50800` calls the same helper as `FUN_14019a260(out, this+0x1c0)`,
which fixes `(dest, src)` from a function whose direction is already known.

**It writes through a computed pointer, which is exactly the `mob+0x42c` shape `CLAUDE.md`
warns about**: a `[reg+0x1b8]` write-scan cannot see it, which is why every previous pass
concluded the field was "read, never computed". It is not never-computed. It is
never-*called*.

Sharpest form of the negative, and the one that does not depend on any address-range guess:
of the 32 `add/lea reg, 0x1b8` pointer hand-offs anywhere in `.text`, **`FUN_142c503c0` is
the only one followed by a call to the string-assign helper.** It is the only string
assignment into any `+0x1b8` in the binary.

### The wire agrees, 72 times — [L]

Every `0x0073` ever captured, deduplicated on `(timestamp, opcode, body)` per `CLAUDE.md`
rather than by file:

```text
423 log files across previous-runs/ and research/fixtures/
 72 distinct 0x0073 EVENTS
  2 distinct bodies

  x57  05000000 0000 d843ae4c5617b6ae9cd200000000764d 00000000   <- this machine
  x15  05000000 0000 aabbccddeeffdeadbeef0000000076 4d00000000   <- the smoke test's synthetic
       ^mode 5  ^^^^ u16 length = 0
```

**The length prefix is `0000` in all 72.** The identity has been empty in every capture this
project has ever taken, and the only thing that varies between the two bodies is the
machine GUID - which is why an earlier pass correctly called this field per-machine rather
than per-launch.

## 4. Things the previous write-ups got wrong or could not support

* **The `-SessionTokens` measurement of 2026-08-18 is not reproducible.** `docs/login-server.md`
  reports it as decisive, and it is almost certainly right, but the **archive begins
  2026-08-19** - the launcher was still deleting logs when that run happened. Its evidence is
  gone. It is now corroborated by static analysis that is independent of it (section 2), so
  the conclusion stands; the original measurement cannot be re-checked. This is precisely the
  failure mode `CLAUDE.md`'s "a run's output is evidence" section was written about.
* **Every capture is mode 5.** All 72 bodies begin `05000000`. `STEAMSTART` (mode 4),
  `WEBSTART` (3) and `IPPORT`/`GAMELAUNCHING` (2) have **never been run against a live
  server**. So "STEAMSTART would not populate the identity" is **[D]**, from the dead-slot
  analysis - not **[L]**.
* **`0x0073` is login-socket only — [L].** It appears in 103 archived files and *every one* is
  a port-8484 login capture; **zero** channel captures. Three of those files have "world" in
  the name and are login logs - the `CLAUDE.md` lesson that a fixture's name says what its
  author was looking at. The channel's own hello is `0x007D` (99 bytes: character id at
  offset 8, then MAC and machine id). **Filling the identity authenticates the login socket,
  not the game socket.**

## 5. What it would cost, and why it is cheaper than assumed

The standing worry was that carrying a session means the hook sending a packet, and that the
channel's client-to-server AES is chained per packet so an out-of-band write desyncs
everything after it. **That worry does not apply here.** We would not send anything. We would
write one pointer field in the client's own object and let the client build, encrypt and send
`0x0073` itself. The cipher is never touched.

Timing is comfortable, and this is **one run**, not two — [L]:

```text
maplecw-hook-20260829-095044.log  and  login-20260829-094630.log
```

The login server's clock runs exactly +4 h from the hook's, so the two files look like
different sessions and must be correlated before either is quoted. Three shared events pin
it, to the millisecond:

```text
   login 13:46:16.898  -> 0x0032        hook 09:46:16.898  opcode=0x0032    exact
   login 13:46:21.413  -> 0x0032        hook 09:46:21.416  opcode=0x0032    +3 ms
   login 13:46:23.735  <- 0x0073        hook 09:46:23.738  opcode=0x0000    +3 ms
```

With the offset established, the whole timeline comes from that single pair:

```text
09:46:14.071  connection
09:46:15.546  hook install_once
09:46:15.597  hook active - ".text settled after 50 ms"
09:46:23.735  0x0073 goes out          <-- 8.14 s after the hook is armed
```

The hook already does strictly harder things than this: it resolves a singleton pointer at
runtime (`session: object at 0x5fd6da8 is now readable`), writes fields inside client objects
(`mode=2,create=on`), sets `int3`+VEH watches on arbitrary addresses, and **forces register
values on entry** (`probe: will FORCE rdx=0x0 on every entry to 0x141b2a280`). Writing a
string field is within its existing, exercised capability — **[D]** from the hook log, not
**[L]** for this specific field.

The one real hazard: `session+0x1b8` holds a client-allocator string, and the destructor does
`ptr-0x10; FUN_14019f2c0(...)`. A block that did not come from the client's allocator will
fault at shutdown. Either allocate with the client's own `FUN_14019b600(&DAT_143ad6a30, len+0x11)`
(the idiom the parser uses), or call the dead-but-correct `FUN_142c503c0`, which does the
allocation properly.

**This is a client patch standing in for a real session and must be labelled as one**, in
`docs/session.md`'s own words. It does not make the client authenticate; it makes the client
transmit a string we chose.

## 6. Experiments, ranked by cost

### Zero launches

**E1 — server-side, today.** Teach `crates/login` to read `0x0073`'s identity string and bind
it, and exercise it with `tools/login_smoke.py` sending a non-empty identity. This proves our
half of the transaction without a client at all, and it is a prerequisite for E2 being
readable: if the server does not log the field, E2's outcome cannot be distinguished from
"the server ignored it".

### One launch — the decisive one

**E2 — the hook writes `session+0x1b8` before `0x0073` is built.** Cleanest form: an `int3`
watch on `FUN_142c50400` (the getter, entered while the builder runs); on entry, if
`[rcx+0x1b8]` is null, point it at a client-allocated string. Change **one** thing; nothing
else in the launch line moves.

Write the readings down before spending it:

| outcome | reading |
|---|---|
| `login.log` shows `0x0073` with a non-zero `u16` length and our token | **The client carries a session end to end**, through its own cipher, no forged packet. The game socket still does not - see below. |
| length still `0000`, and the hook log shows the write happening *before* `0x0073` | Something re-zeroes the field after the write. Next step: watch the ctor's store at `0x142c440ce`. |
| length still `0000`, and the hook log shows no write | The watch never fired - an arming problem, not a protocol finding. Compare the two logs' counts, per `CLAUDE.md`. |
| the client dies at shutdown | The string block is malformed; the destructor freed `ptr-0x10` on non-allocator memory. Switch to `FUN_142c503c0`. |

### Not worth a launch

**E3 — `-NXL <tok> <region> <ip> <port>`.** Predicted result: a byte-identical `0x0073` (mode
is still 5; `cfg+0x80`'s reader still has no callers). This would be a *control* for a static
negative that already has two independent instruments behind it. Only worth doing if E2
fails and the static map comes into doubt.

**E4 — `STEAMSTART`.** Same, plus it needs argv[3] to be `GFN` or `-`, and mode 4 changes the
login screen's button behaviour (`docs/launch-protocol.md`). Costs a launch, changes two
things at once, and the slot it fills is dead.

## 7. The thing that still is not solved

`0x0073` is on the **login** socket. Even with E2 succeeding, `CLAUDE.md`'s standing
constraint - *"Nothing authenticates. The game socket carries no credentials"* - is only half
retired.

Worth stating because it may make the remaining half unnecessary: the channel hello `0x007D`
carries the **character id**, and the character id was issued by *our* login server in the
list it sent. A server-side migration binding (character id -> account, minted at migrate,
consumed once, short-lived) closes the game socket without the client carrying anything new.
That is protocol work in `crates/`, not reverse engineering, and it does not need a client
patch at all — which may make it the better target than E2.

## Instruments used, and their blind spots

* `tools/callrange.py` (**new**) - enumerates calls into a VA *range*. Control: 96 sites, 43
  in `0x140304b20`.
* `tools/callers.py` - call + tail `jmp` + qword pointer. Same control, passing.
* `tools/encodes.py` - control passing (`0x2ff` CTOR at `141cb7eb1`, SEND at `141cb8365`).
* `tools/dataref.py` - 132 functions referencing `DAT_143ac1898`.
* Displacement scan over all of `.text` for `0x1b8`, `0x1c0`, `0x28d0`, disassembling rather
  than byte-matching.

**The blind spot all of them share, stated because two agreeing scans are not corroboration
when they share one:** every scan here is `rel32`- or displacement-based. Code inside the
Themida-virtualised region (`.themida`, `.vm_sec`, `.boot`) is invisible to all of them, so
none of these zeros can exclude a virtualised caller.

What *does* cover that gap is the wire: **72 of 72 `0x0073` captures carry a zero-length
identity.** Whatever may exist in virtualised code, it demonstrably never fires under mode 5,
which is the only mode this project has ever launched.
