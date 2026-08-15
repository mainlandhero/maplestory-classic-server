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

**The handshake is solved.** The client accepts our greeting with no error dialog and
proceeds to a white window, waiting for a login server that does not exist yet. It sent
294 bytes in 16 packets — captured in `research/fixtures/capture-handshake-ok.log`.

The gate that had been hiding: `FUN_1415d10e0` line 606 rejects the connection unless
fields `G == 1` **and** `H == 1`, raising the *same* `0x22000007` "client is outdated"
error as a version mismatch. It runs unconditionally, so it failed every probe we ever
sent — the version sweep never got a chance to matter. Full table in `docs/handshake.md`.

**Transport is decoded** (`docs/transport.md`): classic 4-byte header with
`length = LOWORD ^ HIWORD`, `a = ((iv[3]<<8)|iv[2]) ^ 0x00DF`, and the stock IV shuffle.
Seeding the chain with the `J` we sent predicts **16/16 observed headers exactly**.

### The open problem

**Payload decryption.** The cipher is stock in every part we can read — AES-256, OFB, the
classic MapleStory key (`13 08 06 B4 1B 0F 33 52`, read from `0x143A86810` with a stride
of 4 dwords; the interleaved bytes are decoys), stock S-box, IV repeated 4×, `0x5B0`/`0x5B4`
chunking. Yet the captured payloads do not decrypt. Key, IV, endianness and Shanda
variants have all been ruled out — see the list in `docs/transport.md`.

### Next steps, in order

1. **Decompile `FUN_1415d9cd0`** (reached via `FUN_1415d5aa0`, which runs *before*
   `FUN_1415d5b40`). If it sends a packet, then packet 1 is not opcode `0x70` and the
   known-plaintext test that all the ruling-out relied on was invalid.
2. **Decompile the send counterpart of `FUN_1415d36c0`** — the one reading `conn+0xe8` —
   and read the header construction directly instead of inferring `0x00DF`.
3. Then implement the login server. Note the **game/channel connection uses mode 2**, a
   plain `byte - iv` subtract rather than AES, so it is far cheaper to talk to.

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
