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

The client **parses our handshake body** — a structured payload draws a specific error
dialog where random bytes draw silence. But it rejects us with:

> "The client is outdated. Please download the latest client from maplestory.nexon.net"

**This is not the version check.** Sending a body with *every* `u32` set to 100 still
produced it, which `FUN_1415d10e0`'s logic cannot do. The dialog text also exists nowhere
as plaintext in any client binary, so it is looked up by **error code**.

### Next steps, in order

1. **Map the error codes to messages.** Find what `FUN_1429e4fa0` and the
   `FUN_141804870` / `FUN_1415e0*` family do with a numeric code, and which code yields
   the outdated text. Codes seen: `0x195`, `0x1a4`, `0x23d`, `0x243`, `0x327`, `0x33b`,
   `0x3dc`, `0x3df`, `0x3e3`, `0x3e6`. This names the real failing check.
2. **Enable the client's own log.** `FUN_14019cfe0` writes `MapleStory.LOG` (exe path,
   suffix `LOG`, `ZtlLog` mutex) and the version branches log their real numbers. It is
   gated behind `FUN_140933f30`. Enabling it replaces guesswork with direct observation
   and is probably the highest-value single move.
3. Consider whether the rejection happens **before** our data — e.g. a version-file
   fetch. `nmcogame64.dll` exports `NMCO_SetVersionFileUrlA/W`, and the client is
   firewalled, so such a check would fail closed.

### Testing loop that works

The client's dialog is the oracle; it is not visible to the agent. Run **one variant at
a time** (`--only N`) and have the owner report what they see. That loop found the framing and
disproved the version-check theory.

## Housekeeping

- Firewall rule `MapleCW - block patched client outbound` is **active**. Remove with
  `pwsh tools/firewall.ps1 -Remove` (needs elevation).
- `client-patched/` has the GameGuard stub installed; `pwsh tools/setup-client.ps1
  -Restore` puts the real DLL back.
- Ghidra projects in `research/ghidra/` (~1.2 GB, gitignored). `msexe`, `grap64`,
  `mssecure`, `nexoncm` are all analysed — reuse them rather than re-importing.
