# Where things stand — 2026-08-14

Pick-up notes for the next session. See `ROADMAP.md` for the plan and `docs/` for the
specs.

## Working right now

```bash
cargo test --release          # 47 tests, zero clippy warnings
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
| `crates/net` | Packet reader/writer + stream framer, pluggable cipher. 16 tests. |
| `crates/store` | SQLite accounts/sessions. argon2id, per-password salt, hashed single-use tokens. 21 tests. |
| `crates/auth` | Local HTTP auth server (loopback only) + `maplecw-useradd`. Verified end to end. |
| `crates/grap-stub` | No-op `grap64.dll`. GameGuard never starts; no service, no kernel driver. |
| Client copy | `client-patched/` — original install untouched, firewalled outbound. |

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

## The open problem

**Untested as of this note.** All four handshake gates have been located and a greeting
that satisfies every one is built, but it has not been run against the client yet.

The gate that had been hiding: `FUN_1415d10e0` line 606 rejects the connection unless
fields `G == 1` **and** `H == 1`, raising the *same* `0x22000007` "client is outdated"
error as a version mismatch. It runs unconditionally, so it failed every probe we ever
sent — the version sweep never got a chance to matter. Full table in `docs/handshake.md`.

### Next steps, in order

1. **Run the greeting** — `tools\test-one.ps1 -Variant 0`. Success looks like *no dialog*,
   plus the probe logging 16 raw bytes then packet `0x70`. Any dialog means one more gate
   remains; decode it via `docs/client-messages.md`.
2. **Decode the client's own packets.** Once past the greeting the client talks first:
   16 raw bytes from `conn+0x50`, then `0x70`, then `0x71` (login) or `0x7d` (game).
   Packet layouts and the writer API are in `docs/handshake.md`.
3. **Identify the 16 bytes at `conn+0x50`** — sent before any framed packet, so the server
   must expect them. Plausibly the session key the WEBSTART fields carry, which would also
   unblock the launcher.

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
