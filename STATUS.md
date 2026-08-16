# Where things stand — 2026-08-16

Pick-up notes for the next session. See `ROADMAP.md` for the plan and `docs/` for the
specs.

## Working right now

```bash
cargo test --release          # 49 tests green
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
| `crates/net` | **The client's real wire cipher**, verified against captures. 19 tests. |
| `crates/store` | SQLite accounts/sessions. argon2id, per-password salt, hashed single-use tokens. 21 tests. |
| `crates/auth` | Local HTTP auth server (loopback only) + `maplecw-useradd`. Verified end to end. |
| `crates/grap-stub` | No-op `grap64.dll`; GameGuard never starts. Plus a working **in-process dispatcher hook**. |
| Client copy | `client-patched/` — original install untouched, firewalled outbound. |
| Tooling | `handshake_probe.py` decodes the client's live stream; `dump_runtime.py` reads its memory. |

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

### The login exchange, and where it stands

The client **logs in by itself** - no button press. After the gate it sends:

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

### Next step - set up, not yet tried

```
powershell -ExecutionPolicy Bypass -File "<repo>\tools\test-one.ps1" -Reply ping
  -Opcode 0x0010 -Body 000000 -Pad 256 -PingFirst 0x0032 -PingBody 00
  -ReplyTo 0x0080 -QuietBefore 4 -HookLog on
```

`0x0032` passes the data gate; the moment the client sends `0x0080` it gets `0x0010` with
result `0`, an empty message, and 256 zero bytes to satisfy the success path's reads.

If the dialog persists, watch `FUN_141b267c0` (`-Probe watch@141b267c0`) to see whether it
is returning zero, rather than guessing at the body again.

### The session question, still open

The owner, who knows the live game: the login form is vestigial, the client already holds a
session when it connects, and **"Having trouble logging in?" means the session is
invalid**. In the live client the Login ID field is pre-filled with a masked account email;
in ours it is empty. The password field's "press the Login Button to proceed" is a
placeholder and appears in the live client too - it is not a symptom.

Tried and inconclusive: six distinguishable tokens in config `+0x90` (the array
`-NXLDEBUG` fills from token 3 onward, `WEBSTART` from tokens 4-9) produced a
**byte-identical** client stream - same `0x0073` body, same empty `0x0080`. So either they
are not the identity source or they never arrived; `test-one.ps1` now echoes the real
command line so the next run can tell those apart.

Unexplained and worth decoding: the constant 20-byte tail of `0x0073`, and why `0x0080`
carries no body. The `CNM*` session interface lives in `nexon_api_x64.dll` /
`nmcogame64.dll`, both unpacked and far easier to read than the exe.

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
