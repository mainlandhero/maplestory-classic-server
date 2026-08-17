# Where things stand — 2026-08-19 (record built; the create request must be measured)

Pick-up notes for the next session. See `ROADMAP.md` for the plan and `docs/` for the
specs.

## THE GOAL (set 2026-08-18)

**The server processes an entire character creation transaction.** The owner set this after the
masked email landed and the "connection dies" problem turned out not to exist.

That means, end to end and against a real server-side implementation:

1. the client reaches CharSelect with a **character list we sent**;
2. it asks the server to **check a name**, and the server answers;
3. it sends the **create request**, and the server creates the character and answers;
4. the client returns to CharSelect **with the new character in the list**.

### Where things stand

| | |
|---|---|
| Login screen | done, `0x0032` |
| Login button lit and clickable | done, `0x000B` world entry sets `stage+0x108` |
| Masked email on the login screen | **done**, `0x0000` - no client patch needed |
| Transition to character select | done |
| Connection stays up | **not a problem** - the "reset" was our own log format, see the retraction below |
| Character record | **CONFIRMED ON THE WIRE** - the client decoded it and drew the character |
| Character list | **CONFIRMED ON THE WIRE**, `login_result` |
| "Create a character" button | **still dead.** The `0x0010` slot gate is filled in and did not fix it; one unexamined gate remains, `FUN_140c9e3f0` |
| Name check | **both halves decoded**, `check_name_result` built, never sent |
| Create request | **found, and it is Themida-virtualised** - it must be measured |
| Create result | **decoded**, `create_character_result` built, never sent |
| Client exits ~25s after our last packet | looks like an **inbound idle timeout**; `--keepalive` added and verified, untested against the client |
| Valid session | still faked by two client patches |

### Read these first

* **`docs/character.md`** - the whole character transaction: the list, the complete record
  layout, the three inbound opcodes, the NewChar screen, why the create request cannot be
  read statically, and precisely what is *not* established.
* `docs/opcodes.md` - the opcode tables, now including the character set.
* `docs/session.md` - the login exchange and the "trouble logging in" story.
* `research/msexe-charstats.c`, `msexe-charrecord.c`, `msexe-avatarlook.c`,
  `msexe-newchar-ui.c`, `msexe-char-create.c` - the decompilation this rests on.
* `C:\Users\user\Desktop\ModernMapleSource` - a modern-version server source the owner supplied.
  A naming and structure reference, **not** this protocol. See `docs/character.md`.

### The one thing that cannot be read

**The create request is virtualised.** `FUN_141122420` (the NewChar OK button) calls
`FUN_141b3fb10`, which tail-jumps to `FUN_141b2cf30`, which sets up a packet frame and then
`JMP`s into `.themida`. Each has exactly one caller, so that is the only path. Its opcode
and body have to come off the wire - and they will, because **the OK button sends `0x0081`
and the create request back to back in one click**, without waiting for a reply.

### What the 2026-08-19 run settled

The one-character `0x0010` was sent and **worked**: the client decoded a 327-byte record and
character select drew "Maple". Everything from the head of `0x0010` through the end of the
record is now measured rather than static.

Three other things came out of it:

1. **The account name went blank** - `test-charselect.ps1` did not send `0x0000`. The client
   cannot compute that field; leaving the packet out is enough to clear it. Fixed.
2. **"Create a character" still did nothing**, and the cause is now known. `FUN_141b282d0`
   gates it on three `0x0010` tail fields that were inside the zero padding - see
   `docs/character.md`. Fixed: `stage+0xdc = 1`, `stage+0xe0 = 1`, `slotCount = 3`.
3. **The client exits 15-20 seconds after reaching character select.** `probe.log` shows the
   connection Established and idle for 26s after the client's last packet, then a reset - so
   the client ends its own process. This has happened for many sessions and is now the thing
   that limits every run.

### The plan, in order

1. **Test the keepalive.** If the client stops exiting, every later run gets unlimited time
   at character select, which is worth more than any single finding - character creation is
   several clicks and has never fitted in the window.
2. **Find which gate stops the "new" button.** The slot gate is filled in and did not do it.
   `-Probe watch@141b282d0` splits the two remaining explanations - handler never entered
   (so `FUN_140c9e3f0` returned zero) versus entered and refused - in one run. **This needs
   `-Probe` to accept more than one target**, because the slot is held by
   `watch@141b2a280:rdx=0` and without that the login dialog blocks the Login button.
   That is a contained change in `crates/grap-stub/src/probe.rs`; remember
   `tools/setup-client.ps1` afterwards or the client silently keeps the old DLL.
3. **Answer `0x00A8`**, reach the NewChar screen, then capture the create request - it is
   virtualised and can only be measured.
4. **Answer `0x0081` with `0x0014`** (`check_name_result(name, NAME_AVAILABLE)`) and the
   create request with `0x0015` (`create_character_result`), then watch the client return to
   CharSelect with the new character.
5. **Then make it real**: persist characters in `crates/store` so the transaction survives a
   restart, which is what "the server processes it" means.

### The next run

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-charselect.ps1"
```

Sends the account record, a world, the terminator, and a login result carrying one character
and the create gate. All four bodies come from `packet-hex`, so none of them can drift from
the builders.

**What to read:** the account name on the login screen, the character at character select,
and then whether "Create a character" moves. `probe.log` will show `0x00A8` if the gate
opened. Expect roughly 20 seconds at character select before the client exits.

### Standing warnings

* **`-Session mode=2` and `-Probe watch@141b2a280:rdx=0` are client patches.** They make the
  normal flow reachable; they do not make the session valid. Say so when reporting.
* **Watch mode now arms from `install()`**, so it no longer races the harness's gate packet.
  If the login dialog ever reappears, check `install: hook active` precedes the first
  dispatch before concluding anything about the client.
* **Every character finding above is static and unmeasured.** Four static chains in this
  project looked equally convincing and were wrong. The record has one thing they did not -
  an independent second decoder that agrees - but that is still not the wire.

## Working right now

```bash
cargo test --release          # 60 tests green
cargo build --release
```

**The client runs and connects to our server:**

```bash
# 1. start a listener/probe
python -u tools/handshake_probe.py --port 8484
# 2. launch the patched client (from client-patched/)
MapleStory.exe -NXLDEBUG 127.0.0.1 8484
```

It connects to `127.0.0.1:8484`, and GameGuard never loads.

## Done

| | |
|---|---|
| `crates/wz` | WZ parser. **9,994/9,994 images** across 102 archives parse. `wz-dump` CLI. |
| `crates/net` | **The client's real wire cipher**, verified against captures, plus the recovered inbound opcodes and their bodies, and `packet-hex` to put a body on a command line without typing it. 28 tests. |
| `crates/store` | SQLite accounts/sessions. argon2id, per-password salt, hashed single-use tokens. 21 tests. |
| `crates/auth` | Local HTTP auth server (loopback only) + `maplecw-useradd`. Verified end to end. |
| `crates/grap-stub` | No-op `grap64.dll`; GameGuard never starts. Plus the **in-process dispatcher hook**, the opcode walk / watch probe, a session monitor and patcher, and a socket watch over `connect`/`closesocket`/`shutdown`. |
| Client copy | `client-patched/` — original install untouched, firewalled outbound. |
| Tooling | `handshake_probe.py` decodes the client's live stream; `dump_runtime.py` reads its memory. |
| Canvas render | `wz-dump canvas` + `tools/wz_png.py` turn WZ canvases into PNGs. **This is how the client's baked UI text gets read** - much of its on-screen wording is pixels, invisible to any string search. Formats 1, 2 and 513. |

## Key facts (do not re-derive)

- **WZ data version 779**, hash `0x0000E73A`, **zero** string key.
- **Network protocol version is 100** — unrelated to 779. Don't conflate them again.
- Launch: **`-NXLDEBUG <ip> <port>`** is the only mode that runs *and* connects.
  `IPPORT` crashes; `WEBSTART` needs six session fields we cannot yet fake.
- Handshake framing: **`u16` little-endian body length, then the body** (length excludes
  itself; the client rewinds over the prefix). Confirmed working.
- `MapleStory.exe` is **Themida**-protected with a rebuilt IAT — do not patch it on disk.
  Find code via **string xrefs**, never import xrefs. That technique has worked four
  times now.
- The client cannot be killed with `Stop-Process`; use `taskkill /F`.

## Transport: SOLVED IN BOTH DIRECTIONS

**The handshake is solved.** `FUN_1415d10e0` line 606 rejects the connection unless fields
`G == 1` **and** `H == 1`, raising the *same* `0x22000007` "client is outdated" error as a
version mismatch, unconditionally — which is why every early version sweep looked
identical. Full table in `docs/handshake.md`.

**The packet transport is solved**, and the client both accepts our frames and has its own
stream fully decoded. See `docs/transport.md`.

```
len     = a ^ b                     # two u16 LE; no byte-swap, unlike classic MapleStory
a       = ((iv >> 16) & 0xFFFF) ^ K # K = 0xFFFE for packets we send, 0x00DF for the client's
payload = AES-256-OFB(key, iv repeated 4x)   # chunks 0x5B0 then 0x5B4
iv       -> stock shuffle table at 0x143A86890, rolled once per packet
```

Our chain seeds from `K`, the **second** u32 of the greeting (`conn+0xec`); the client
transmits on `J`, the first (`conn+0xe8`). Lengths `>= 0xFF00` use an 8-byte header.

### The AES key is a decoy on disk — do not "fix" it

The table at `0x143A86810` holds the **stock** MapleStory key in the file, and the client
overwrites the low byte of all 32 dwords at startup. Only that table — the shuffle table
beside it is untouched, which is exactly why framing, the header constant and the IV chain
were provably correct while everything AES-shaped failed in *both* directions at once.

```
0f 00 00 00  1b 00 00 00  c5 00 00 00  46 00 00 00
f3 00 00 00  be 00 00 00  ff 00 00 00  75 00 00 00
```

Read with `tools/dump_runtime.py` (read-only, needs an elevated shell), stable across
sessions, so it is a build constant. `the_disk_key_is_a_decoy_and_does_not_decrypt` guards
against reverting it.

With it, every captured packet matches its decompiled builder field for field — packet 1 is
`70 00 02 64 00 00 00`, exactly `FUN_1415d5b40`'s `u8 2, u32 100`.

### What the client sends, and what it waits for

Login connection startup, read from the handshake tail (`conn+0x48 != 0` selects it):
**`0x70` version, `0x71` environment, `0x8F`/`0x90`/`0x91` log uploads, optional `0xA1`** —
then the handler *returns*. The hang is in the main loop, waiting on the socket. The eleven
6-byte packets are `0x00A6` carrying an incrementing id.

`0x8F`-`0x91` read a file up to 8 KB, upload it and delete it; they need no reply, and they
are why opening bursts varied 294 to 3393 bytes between runs.

### All earlier sweep results are void

Every sweep predates the key fix, so the client never saw an opcode we intended, and the
scattered exits at `0x0023`, `~0x01DC`, `~0x01F1`, `~0x03C5` were random garbage opcodes
hitting a disconnect handler — none reproduced, and `0x0023` sent alone did nothing.

Note the trap that hid this: **acceptance only proves the header**. A bad payload decrypts
to a random opcode and is silently ignored, not rejected.

### The startup gate is solved - the client reaches its login screen

**Inbound opcode `0x0032`, body `0x00`.** Seven bytes on the wire, and the client goes from
a blank non-responding window to the login screen. Verified with a single packet and no
probe: `flag=0->1 state=0->2`.

It was never a login handshake. The client hashes `Data.wz` into `conn+0x14c`, sends
`0x00A1` carrying that `u32`, and blocks in `recv` **on its UI thread** inside
`FUN_1415e7090`, looping recv -> decrypt -> dispatch until a handler sets the byte at
`conn+0x150`. Only `FUN_1415e5c20` does that, and it is a `Data.wz` patch handler whose
first field is a **zigzag varint** length (`FUN_1406efcc0`):

| length | client does |
|---|---|
| `0` | nothing to patch - sets the flag and carries on |
| `> 0` | expects that many bytes in 64 KB chunks, then writes `Data.wz` |
| `< 0` | deletes `Data.wz` and carries on |

This client ships no `Data.wz` at all (a `Data/` directory instead), so it sends hash `0`
and a varint `0` is the right answer. See `crates/net/src/opcode.rs`.

That also explains the old "26 packet ceiling": every unhandled packet allocates a `0x5b4`
buffer inside that loop and the loop never exits to free them. A leak, not a limit.

### The login exchange

**Superseded in part:** "the client logs in by itself" is true only in **mode 5**
(`-NXLDEBUG`), where a per-frame tick calls the same function the Login button calls. With
`-Session mode=2` the client waits for the button, which is the real flow. See "THE GOAL"
at the top.

After the gate the client sends:

```
0x00C0  05 00 00 00 20 4e 00 00
0x0073  26B  05 00 00 00 00 00 aa bb cc dd ee ff de ad be ef...   <- 20 bytes, constant
0x0080  (empty body)                                              <- the login request
0x007A  01 01 4x 00 00 00 ...
```

then waits **4-7 seconds** and abandons the connection. `0x0073` and `0x0080` are both
built by `FUN_141b21ea0`, the function that loads `UI/Login.img`.

**The reply is inbound `0x0010`**, and this is the structural find of the session: the
login stage's `OnPacket` is `FUN_141b25f30`, and it is an **ordinary readable switch on the
opcode**. The Themida-virtualised dispatcher hands a stage its opcode; the stage dispatches
in plain code. So the whole login-stage opcode map is readable:

```
0x00, 0x0b-0x18, 0x23, 0x25-0x27, 0x29, 0x2b, 0x34-0x39, 0x45-0x48, 0x4a, 0x50, 0x5f, 0x5f4
case 0x10 -> FUN_141b307b0    the login result
```

Watch mode confirmed at runtime that `FUN_141b307b0` **is entered while dispatching
`0x0010`**, so the opcode and the stage are both right.

Body of `0x0010`, from `FUN_141b307b0` and `FUN_1406e9050` (strings are `u16` length then
bytes):

```
u8  result
str message
if result == 0:    u8, 8 bytes, u32, u32, 4B, 4B, 4B, u32, u8,
                   then FUN_14108d290 and FUN_14108bdf0 read further
if result == 0x83: two more u32
```

**Result `0` is success.** `FUN_141b267c0(this, result, 0, ...)` raises the error dialog,
and the proceed branch is `cVar6 != 0 && result == 0`. `0x65`/`0x67` are *not* success -
they take a different branch that re-sends `0x0080`. Misreading them as success cost three
runs of the same dialog.

Corroborated independently: non-zero results are **error message IDs**, resolved through
`FUN_141803cd0` in `docs/client-messages.md`. `0x65` is 101, *"You have been disconnected
from the login server"* - exactly the dialog that replying `0x65` produced.

### DONE - the login result is accepted, the client reaches character select

`0x0010` with `body = 00 00 00` + 256 zero bytes ran the success path to completion and the
client's UI **advanced to character select**. Compare `0x65`, which dropped the connection
in 0.0 s with no follow-up. Fixture:
`research/fixtures/reply-0010-result0-advanced-to-charselect.log`.

Full field list in `docs/opcodes.md`. Two fields matter beyond filler: the `u32` world id
and `u32` channel id, which the client looks up in a world list it does not yet have.

### The whole login stage has two variants, and we are in mode 5

**Check this before decoding any login-stage handler.** Several open with

```c
if (FUN_142c4a810(DAT_143ac1898) == 5) { <other handler>(...); return; }
```

`session+0x68` is **5** in our client - transmitted as the first `u32` of `0x0073`, captured
as `05 00 00 00`. Mode 5 is what **`-NXLDEBUG`** sets, which is how we launch. So the
mode-5 branch is always the live one and the handler the switch names first is dead code
for us. `0x000B`, the login flow, and the Login button all fork this way. Decoding the
wrong side costs a full analysis pass. Table in `docs/opcodes.md`.

### The Login button is enabled by one byte, and `0x000B` sets it

The owner: the button starts **disabled** in an invalid session. `FUN_14112a720`, the
`ClassicIntro` tick, enables the control named `"login"` only when
`FUN_141b2a160(stage)` - that is, `*(u8 *)(stage + 0x108)` - is non-zero. The screen
builder `FUN_141129930` creates it disabled.

`stage+0x108` is written by the **world-list handler**, one line above the list append:

```c
*(undefined1 *)(param_1 + 0x108) = 1;
piVar10 = (int *)FUN_141b44520(param_1 + 0x100, 0xffffffff);
```

So **inbound `0x000B` enables the button**, populates the list the login result searches,
and is the one thing missing since the client first reached the login screen. Only a real
world entry does it - the terminator branch returns before both writes.

The account field is a different object: `FUN_142cb83a0` is `DAT_143aa84a0 + 0x22f8`,
rendered into `textAccount` when non-empty. `DAT_143aa84a0` also holds world id `+0x2258`
and channel id `+0x2260`; it is **not** the `DAT_143ac1898` that carries the `0x0073`
identity.

### MILESTONE - login screen -> Login button -> character select

**Reached 2026-08-17.** The owner clicked a lit Login button, the client played its animated
transition into character select, and "Create a character" was the blocker - the goal set
at the start of the day.

The recipe, all four parts needed together:

```
powershell -ExecutionPolicy Bypass -File "<repo>\tools\test-one.ps1" -Reply ping
  -Opcode 0x0032 -Body 00 -PingFirst 0x0032 -PingBody 00 -QuietBefore 4 -HookLog on
  -Session mode=2 -Probe watch@141b2a280:rdx=0 -ReplyTo 0x0080
  -ReplySeq "000b:0006005363616e6961000000000108005363616e69612d30000000000000000000000000000000"
```

| Part | Why it is needed |
|---|---|
| `0x0032` gate | releases the startup loop; login screen appears |
| `0x000B` world **entry** | sets `stage+0x108`, which is what enables the Login button |
| `-Session mode=2` | leaves mode 5 so the per-frame tick stops auto-logging-in and the button gets a turn |
| `-Probe watch@141b2a280:rdx=0` | suppresses the "trouble logging in" dialog, which otherwise **blocks the tick** and stops the button ever being enabled |

**No terminator.** With the mode patched, a second `0x000B` is handled by the classic
`FUN_141b2fac0`, whose terminator transitions to WorldSelect - a screen this service does
not use.

**Two of those four are client-side patches.** They make the client's normal flow
reachable; they do **not** make the session valid. Describe results accordingly.

**Everything after the login screen was offline.** The client closed the connection at
8.4s - immediately after sending `0x007A`, its loading-complete report - so the Login click,
the transition and the "Create a character" clicks all happened with no server attached,
and **sent nothing**. Those transitions are purely client-side.

**Adding the login result back did not keep the connection alive.** It *was* dispatched and
handled (`2 opcode=0x0010 ... ret=1`), and the client closed 0.42s later, exactly as it does
without one. So the close is not a rejection of our reply.

### RETRACTED: "the client does not migrate" was never established

This section used to read "SETTLED: the client does not migrate", on the strength of a
`connect` hook that logged **nothing**. That was the same silent-negative mistake the repo
warns about everywhere else, and it took two runs to notice.

**The hook has never logged a `CONNECT` line at all — including for the connection to
`127.0.0.1:8484`, which certainly happened.** So "no connect was logged" says nothing
about the client's behaviour until the hook is shown to work. It may be that the client
reaches its socket through a path `ws2_32!connect`/`WSAConnect` do not cover; it may be
that the hook is simply broken. Either way the migration question is **open**, and so is
everything that was inferred from it.

`netwatch` now runs a **self-test** at install: it makes its own loopback `connect` and
`closesocket` and reports whether its handler caught them.

```
netwatch: SELF-TEST ok - 2 of our own calls were caught, so a later absence of lines is a
real negative
```

Read that line before reading anything else from this hook. `SELF-TEST FAILED` means every
negative it reports is worthless.

**What is still true:** `tools/watch-sockets.ps1` saw only one socket, and no second
endpoint was ever observed. That is weak evidence for one connection, not proof.

**What was inferred from the retracted claim, and is now unsupported:**

* that no channel server is needed;
* that the unread fields in the login result cannot be a server address.
* **The close is *not* explained.** It was recorded here as an ~8s idle timeout; that was
  inferred from timing alone and the timing has a second explanation - see "CORRECTION"
  above. What is settled is only that no reconnect follows it.

### The real gap: the client has no character list

Also from that run: **the Login click, the transition to character select, and every
"Create a character" click sent nothing at all.** Not one packet, while the socket was still
open. So those controls are not blocked by a dead connection - the client has nothing to
send because it was never told what characters exist.

That is the next thing to find: the character-list packet. The client reached character
select without one, which is why the screen is inert.

**Both of those are now answered - see `docs/character.md`.** The list is inside `0x0010`
(`FUN_14108bdf0`: `u8 count`, then records), and `FUN_141b28570` turned out **not** to
consume a character list at all; it is a 290-byte state check that was nominated here on
the strength of where it is called from, and never read.

### After that: build the server side

Client-side patching has taken this as far as it goes - the flow is reachable and the
blocker is now that **nothing is answering**. Character select and character creation need
real handlers, which is the `login server to character select` milestone in `ROADMAP.md`.

### SOLVED - `FUN_141b2a280` raises the prompt

`called-from=0x141b2a61e` at a watch on the notice display named it.
`FUN_141b2a280(stage, code, flag)` shows `loginTroubleAskSupport` for **codes -1, 6, 8, 9
and 12** (`0x2681` bit-tested at `code + 1`); `code == 0` is success. Full code -> notice
table in `docs/session.md`, and the whole baked dialog table in `docs/client-notices.md`.

It is a near-duplicate of `FUN_141b267c0` - same mapping, different function.

**Why it hid for three sessions, and the lesson:** it never takes the string's address. It
**copies the literal inline** with RIP-relative `mov`. `tools/xref.py` matches `lea`, so it
reported three references, all of which were then proven never entered - and the real raiser
was invisible to every scan built on it. A "0 references" result means *nothing takes its
address*, not *nothing uses it*. That warning is now at the top of `xref.py`.

Independently corroborated: an exhaustive render of all 170 `/Notice/` canvases found no
duplicate node and no numeric twin, so the dialog on screen is definitely this one.

### The code is 12, and the caller is virtualised

```
WATCH #1: 0x141b2a280 ENTERED  rdx=0xc (as i32 12)  r8=0x1  called-from=0x144c05eb2
```

* **`code = 12`** - in the trouble set, and a *generic* failure: no specific notice maps to
  it, unlike 4 (`incorrectPassword`) or 5 (`notRegisteredID`). The client is not reporting a
  named reason, it is reporting "login did not succeed".
* **`r8 = 1`** - the flag argument, which sets `stage+0xf0 = 1`.
* **`called-from = 0x144c05eb2` is inside `.themida`.** The immediate caller is virtualised
  and cannot be decompiled.

No packet carried this. Only `0x0032` was ever dispatched, so the client generated code 12
on its own.

**The stack walk is a dead end, and that is settled.** A 0x400-byte, 16-slot scan of the
stack at the call found exactly one image address: the VM return address itself.

```
stack: 0x144c05eb2(vm)
```

No `.text` frames at all. Themida runs the VM on **its own stack**, so the caller chain is
not there to find and widening the scan only reads more VM stack. **Who decided code 12
cannot be answered by walking back from the call.**

Two ways forward, and they answer different questions.

**1. What does the client do if it believes login succeeded?** `FUN_141b2a280` returns
success for code `0`, so rewriting the code at its entry answers that in one run.

```
powershell -ExecutionPolicy Bypass -File "<repo>\tools\test-one.ps1" -Reply ping
  -Opcode 0x0032 -Body 00 -PingFirst 0x0032 -PingBody 00 -QuietBefore 4
  -HookLog on -Probe watch@141b2a280:rdx=0 -Session watch
```

This is a **client-side patch** - it makes the client stop concluding it failed; it does not
make the session valid. Say so when reporting results. What it buys is the rest of the
flow: whether the login screen becomes usable, and where the client gets stuck next.

**2. Why the client concludes failure.** The decision is virtualised, but what it *consults*
need not be. The `CNM*` session interface lives in `nexon_api_x64.dll` / `nmcogame64.dll`,
both **unpacked** - readable statically and hookable at their exports. That is the honest
route to a genuinely valid session, and it needs no client runs to start.

### How it was found

A watch on the notice display `FUN_141b4ac80(name, ...)` caught it:

```
WATCH #1: 0x141b4ac80 ENTERED  rcx=0x14d148 [0x057f5ed8] "loginTroubleAskSupport"
          rdx=0x14d100  r8=0x0  r9=0xe
```

Settled by that line: the dialog **is** `loginTroubleAskSupport` (not some similar node),
it is raised through `FUN_141b4ac80`, and the name arrives **intact** - so it is not built
at runtime.

Which leaves a genuine puzzle. Something loaded that name, but:

* the `.rdata` literal at `0x1433d5d98` has exactly **three** code references, and all three
  are proven never entered;
* there is **no pointer-table reference** either - a scan for the qword `0x1433d5d98`
  anywhere in the file finds nothing;
* `rcx` pointed at a **heap** copy (`0x057f5ed8`), not the literal.

The leading explanation is that the caller is **virtualised**: a `lea` inside Themida VM
bytecode is invisible to every static scan we have. If so, static analysis is finished here
and everything further must be measured.

**Next:** watch mode now logs `called-from`, read from `[rsp]` at the breakpoint - the
breakpoint sits on the function's first byte, so the `call` has just pushed the return
address. Re-run the same command; the caller names itself.

```
powershell -ExecutionPolicy Bypass -File "<repo>\tools\test-one.ps1" -Reply ping
  -Opcode 0x0032 -Body 00 -PingFirst 0x0032 -PingBody 00 -QuietBefore 4
  -HookLog on -Probe watch@141b4ac80 -Session watch
```

A `called-from` inside `.themida` (roughly `0x144C0000`+) confirms the virtualised-caller
theory. Anything in `.text` names a real function to decompile.

#### Older note, now superseded

**One question, and it has a designed experiment:** what result code reaches
`FUN_141b267c0`, and when? The dialog is raised for result -1, 6, 8 or 9, but the owner sees it
*before* any login exchange, and `0x0032` (handled by `FUN_1415e5c20`) never touches that
path. Two of its callers - `FUN_141b2b120`, a 31-byte wrapper that passes the code straight
through, and `FUN_141b2ae80` - have no callers and are in no vtable, so they are reached
only through the virtualised dispatcher and **cannot be traced statically**.

So observe it. **`-Probe watch@<VA>` now does this**: it reports *every* entry with the
dispatching opcode and the first four integer arguments (`rcx`, `rdx`, `r8`, `r9`), rather
than announcing one hit and disarming. `rdx` is the result code, and the switch above turns
that number into the dialog on screen.

```
powershell -ExecutionPolicy Bypass -File "<repo>\tools\test-one.ps1" -Reply ping
  -Opcode 0x0010 -PingFirst 0x0032 -PingBody 00 -ReplyTo 0x0080 -QuietBefore 4
  -HookLog on -Probe watch@141b267c0
```

Send only the gate, so nothing we send can be the cause: if the dialog still appears, the
failing code came from the client itself. Read `rdx as i32` out of the `***** WATCH #n`
lines in the hook log.

One known limit: it stops logging after 32 hits so a per-frame caller cannot fill the disk.
(The old "can only arm once the hook has seen a dispatch" limit is **gone** - watch now
arms from `install()`, which also removes the race that twice brought the dialog back.)

**The auto-advance is not a bug.** In mode 5 the `ClassicIntro` tick calls
`FUN_141b3ff10` - *the same function the Login button calls* - as soon as `0x000B` sets
`stage+0x108`. `-NXLDEBUG` is a debug launch mode that logs in without the button, which is
why the flow does not match a normal server.

**To get the click-the-button flow**, write anything but `5` to `[0x143ac1898] + 0x68`
(`session+0x68`) from `grap-stub` once the world list has landed. Then the tick's
auto-login goes false, the button still enables (that happens as a side effect of the
`+0x108` check, independent of mode), and clicking Login takes the readable
`FUN_141b3f050(stage, 4, 600)` straight to CharSelect. Switching modes sends `0x000B` to the
classic handler `FUN_141b2fac0` instead of `FUN_141b31ff0`, which is safe: their read
sequences were compared field by field and are identical. **Be honest about what this is** -
it makes the client follow the normal flow, it does not make the session valid.

### Next step - the world list

The login result makes the client search for its world in the list at `stage+0x100`
(`FUN_141b2c7c0`). Only inbound **`0x000B`** appends to it, and we have never sent one, so
the client arrives at character select with no world. That is the gap, not the empty
character list - an account with no characters legitimately routes to "create a character"
(the owner, who knows the live game).

Format decoded from `FUN_141b31ff0` (the mode-5 handler) and built by
`crates::net::opcode::{world_list_entry, world_list_end}`, with a test that re-reads the
bytes the way the client does.

The run needs **three packets** in answer to `0x0080`: a world entry, the terminator, then
the login result. `-ReplySeq` now does that (`OPCODE:HEXBODY[/PAD],...`), sent back to back
on the same socket. The world-list bytes below are pinned by
`the_one_world_list_we_actually_send_has_these_exact_bytes`, so the command cannot drift
from the decoded format.

```
powershell -ExecutionPolicy Bypass -File "<repo>\tools\test-one.ps1" -Reply ping
  -Opcode 0x0010 -PingFirst 0x0032 -PingBody 00 -ReplyTo 0x0080 -QuietBefore 4 -HookLog on
  -ReplySeq "000b:0006005363616e6961000000000108005363616e69612d30000000000000000000000000000000,000b:ff0000,0010:000000/256"
```

Read the result off the **wire**, not the screen: does the connection outlive the reply,
and does the client send anything it has not sent before? The bitmap on the login screen is
not evidence either way.

### The session identity - see `docs/session.md`

Short version, because two long-standing assumptions turned out to be wrong:

- **"Having trouble logging in" is `/Notice/text/loginTroubleAskSupport`** - a baked bitmap
  in `Login.img`, which is why no string search ever found it. **Solved:** raised by
  `FUN_141b2a280(stage, code, flag)` for codes -1, 6, 8, 9 and 12; measured live as **code
  12**, a generic failure, from a **virtualised** caller. Three other candidates were ruled
  out by measurement first. Full table and the tooling lesson in `docs/session.md`.
  `Login.img` also has **two** login screens (`Title_new`, and `ClassicIntro` = ours, the
  one carrying `find_id`/`find_pw`); `FUN_141129930` builds `ClassicIntro`.
- **The empty identity did not block the login.** The client still sent `0x0073` and
  `0x0080` and accepted a `result = 0` reply. It is a real gap but not the current blocker.
- **The account name is server-supplied** (`0x0000` / `0x0012`), so the session may be too.
  That demotes the launcher-handoff theory this section was built around - see "THE GOAL".

The identity is one `char *` at **`DAT_143ac1898 + 0x1b8`**, read by `FUN_142c50400` and
sent as the second field of `0x0073`, where we captured a **zero-length string**. Nothing
computes it. The six `+0x90` launcher tokens are ruled out. Next: find its writer in
`nexon_api_x64.dll` / `nmcogame64.dll` (both unpacked), or write the field directly from
`grap-stub`, which is already in-process.

The old "constant 20-byte tail of `0x0073`" question is closed: it is a 16-byte GUID plus a
4-byte counter, not session data.

### The opcode walk, and how to aim it

`crates/grap-stub/src/probe.rs` walks the inbound opcode space **inside** the client:
snapshot one captured packet, rewrite its opcode, re-dispatch, watch an oracle. The whole
enum in one launch instead of ~2 opcodes per launch over the wire. Faults are caught by a
vectored handler and the loop resumes; `ExitProcess`, `TerminateProcess`,
`RtlExitUserProcess` and `NtTerminateProcess` are detoured so a handler cannot end the run;
progress is appended to a resume file so a fatal opcode costs one launch, not the search.

`-Probe <from>-<to>[@targetVA][#N]`, or `-Probe watch@<VA>` to observe whether a function
runs at all.

**Aim it with `tools/handler_root.py`, never by hand.** A walk target must be a dispatcher
entry: no direct callers **and in no vtable**. `FUN_141b25f30` has no callers but *is* a
vtable entry, and aiming at it burned a full 3968-opcode run that missed cleanly.

**And time it.** `#N` starts the walk on the Nth dispatched packet. The walk runs inside
whatever loop the client is in, so walking for a login-stage handler before the login
screen exists cannot work no matter what address is used.

### Traps that cost time - do not re-learn these

**Protocol**

* The **on-disk AES key is a decoy**; read the real one from a running client.
* **Accepting a packet only proves the header.** A bad payload decrypts to a random opcode
  and is silently ignored, not rejected.
* Login result **`0` is success**; `0x65`/`0x67` are a different branch entirely.
* **Check the `session+0x68 == 5` fork before decoding any login-stage handler.** The
  handler the switch names is often a shim that hands off to the mode-5 one, and we are
  always mode 5.
* **The "trouble logging in" prompt is a bitmap, but it *is* a state readout.** It appears
  before any packet exchange, so no reply can clear it and it does not measure the wire.
  For wire questions use the identity string in `0x0073` and how long the connection
  survives a reply; for the prompt, look at which login screen was built.
* The client's opening burst varies **294 to 3393 bytes** because `0x8F`-`0x91` upload and
  delete log files.

**The walk**

* **Snapshot the packet before the dispatch, never after.** The dispatcher consumes the
  opcode and moves the cursor 4 -> 6, so an after-snapshot replays "opcode 0" every time:
  4096 dispatches, no faults, reported as an empty range.
* **One oracle per walk.** With a target armed, `conn+0x150` is *expected* to be set
  already, so consulting it as well reports a hit on the first opcode tested.
* **Aim at dispatcher entries** - no callers *and* no vtable. Use `handler_root.py`.
* **Time the walk** into the phase where the handler exists (`#N`).
* **Append resume records.** `fs::write` truncates first, so dying mid-write leaves an
  empty file and the next launch restarts from zero and dies in the same place.
* The probe detours `ExitProcess`, so the client survives and, being elevated, **cannot be
  killed from a normal shell** - use `taskkill /F /IM MapleStory.exe /T` from an elevated
  one, or the leftover holds port 8484 and the DLL file.

**The harness**

* **Answer on packet arrival, not on a timer.** The client sends `0x0080` at +4.4s, +6.1s
  or +7.2s and gives up seconds later; a fixed delay once fired 0.12s *before* the request
  it was meant to answer. Use `-ReplyTo`.
* `-QuietBefore` also gates timed replies, and the client is rarely quiet for that long.
* PowerShell variable names are **case-insensitive**: a `$probe` local silently ate the
  `-Probe` parameter.
* A parameter that never arrives looks exactly like one that arrives and does nothing -
  `test-one.ps1` echoes the real command line for that reason.
* Scripts are invoked as `powershell -ExecutionPolicy Bypass -File "<abs path>"`.
* **Rebuilding `grap-stub` does not update the client.** `cargo build` writes
  `target/release/grap64.dll`, but the client loads `client-patched/grap64.dll`, and only
  `tools/setup-client.ps1` copies one to the other. Skip it and the run silently uses the
  old hook - the most expensive kind of failure here, because it looks like the new code
  did nothing. Compare hashes if in doubt.

### Testing loop that works

The client's dialog is the oracle; it is not visible to the agent. Run **one variant at
a time** and have the owner report what they see. That loop found the framing, disproved the
version-check theory, and confirmed the `L` gate.

`tools\test-one.ps1` runs one variant: it starts the probe, launches the client at
BelowNormal priority pinned off core 0 (the client otherwise saturates the host while a
test sits waiting for a dialog to be read), and tears both down with `-Stop`.

The probe now **holds the connection open** (`--hold`, default 300s). Closing it early
makes the client's `recv` return 0, which sends it down its own disconnect path
(`FUN_1415d10e0` recurses with `param_2 = 0` → `0x22000001`) and looks exactly like a
rejected handshake.

## Housekeeping

- Firewall rule `MapleCW - block patched client outbound` is **active**. Remove with
  `pwsh tools/firewall.ps1 -Remove` (needs elevation).
- `client-patched/` has the GameGuard stub installed; `pwsh tools/setup-client.ps1
  -Restore` puts the real DLL back.
- Ghidra projects in `research/ghidra/` (~1.2 GB, gitignored). `msexe`, `grap64`,
  `mssecure`, `nexoncm` are all analysed — reuse them rather than re-importing.
- **Ghidra needs JDK 21, not 25.** Under JDK 25 the bundled Felix 7.0.5 aborts with
  `Bundle org.apache.felix.framework [0] The data file must be inside the data dir`.
  Prefix headless runs with:

  ```powershell
  $env:JAVA_HOME="C:\Program Files\Eclipse Adoptium\jdk-21.0.6.7-hotspot"
  $env:PATH="$env:JAVA_HOME\bin;$env:PATH"
  ```

  If it was already run under 25, also delete
  `%APPDATA%\ghidra\ghidra_12.1.2_PUBLIC\osgi\felixcache` (a regenerable script cache).
