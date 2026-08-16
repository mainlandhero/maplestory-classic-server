# Where things stand — 2026-08-15

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

**Inbound opcode `0x0032`, body `0x00`.** Seven bytes on the wire, and the client goes
from a blank non-responding window to the login screen.

It was never a login handshake. On connect the client hashes `Data.wz` into `conn+0x14c`,
sends `0x00A1` carrying that `u32`, and then blocks in `recv` **on its UI thread** inside
`FUN_1415e7090`, looping recv -> decrypt -> dispatch until a handler sets the byte at
`conn+0x150`. Only `FUN_1415e5c20` does that, and it is a `Data.wz` patch handler. The
body is a zigzag varint length (`FUN_1406efcc0`):

| length | client does |
|---|---|
| `0` | nothing to patch - sets the flag and carries on |
| `> 0` | expects that many bytes in 64 KB chunks, then writes `Data.wz` |
| `< 0` | deletes `Data.wz` and carries on |

This client has no `Data.wz` at all - it ships a `Data/` directory - so it sends hash `0`
and a varint `0` reply is the right answer. Verified with a single packet and no probe:
`flag=0->1 state=0->2`, then the login screen. `crates/net/src/opcode.rs`.

That also explains the "26 packet ceiling": every unhandled packet allocates a `0x5b4`
buffer inside that loop and the loop never exits to free them. It was a leak, not a limit.

### How inbound opcodes get found, because it is not by reading

The dispatcher is virtualised and handler addresses appear nowhere as data - not in the
image, not in a gigabyte of live memory. Sweeping over the wire costs about two opcodes
per launch, because live handlers take the client down.

So `crates/grap-stub/src/probe.rs` walks the space **inside** the client: it snapshots one
captured packet, rewrites its opcode, and re-dispatches, watching `conn+0x150`. The whole
enum in one launch. A vectored handler catches decoder throws and access violations and
resumes the loop; `ExitProcess`/`TerminateProcess` are detoured so a handler cannot end
the run. `-Probe <from>-<to>` on `test-one.ps1`.

**Snapshot before the dispatch, never after.** The dispatcher consumes the opcode and
moves the cursor 4 -> 6, so an after-snapshot replays with the cursor past the opcode and
dispatches "opcode 0" every time - a silent no-op that reports as an empty range. The walk
now reports how many calls advanced the cursor, and says outright when the answer is
"broken", not "empty".

### Next steps

The client is at the login screen and still talking: after `0x0032` it sends `0x00BF`,
`0x00C0` (`05 00 00 00 20 4e 00 00`) and another run of `0x00A6` with ids 19-28.

1. **Capture a login attempt.** Have the owner type credentials and press Login while the probe
   holds the connection. Whatever opcode that produces is the next thing to answer, and it
   is outbound - so `docs/opcodes.md` can name its fields.
2. **Answer it** from `crates/store`, which already has argon2id accounts and single-use
   session tokens, plus `maplecw-useradd`.
3. **Find the reply opcode** the same way, with `-Probe` narrowed around a candidate band.

### Traps that cost time — do not re-learn these

* The **on-disk AES key is a decoy**; read the real one from a running client. A regression
  test guards against reverting it.
* **Accepting a packet only proves the header.** A bad payload decrypts to a random opcode
  and is silently ignored, not rejected — which is what made a broken cipher look fine for
  six sweeps.
* `tools/test-one.ps1` runs the **bare system Python** with no third-party packages; keep
  the probe path stdlib-only, and check `probe.err` is empty before believing any negative.
* The client needs **elevation**, so it must launch via ShellExecute, which does **not**
  propagate `$env:` — hence the hook's marker file.
* The client's opening burst varies **294 to 3393 bytes** because `0x8F`-`0x91` upload and
  delete log files. Wait for silence (`-QuietBefore`) before attributing anything to a reply.
* Scripts here are invoked as
  `powershell -ExecutionPolicy Bypass -File "<abs path>"`; the bare path will not run.

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
