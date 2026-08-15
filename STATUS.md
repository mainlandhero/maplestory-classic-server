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

### Transport is SOLVED IN BOTH DIRECTIONS

The last blocker was the AES key. The table at `0x143A86810` holds the stock MapleStory key
**on disk and is a decoy** — the client overwrites the low byte of all 32 dwords at startup.
Only that table; the IV shuffle table beside it is untouched, which is exactly why framing,
the header constant and the IV chain were provably right while everything AES-shaped failed
in *both* directions. Real key (`tools/dump_runtime.py`, stable across sessions):

```
0f 00 00 00  1b 00 00 00  c5 00 00 00  46 00 00 00
f3 00 00 00  be 00 00 00  ff 00 00 00  75 00 00 00
```

**Do not "correct" this back to the stock key** — a regression test guards it.

The probe now decrypts the client's stream live and logs it as `opcode + fields`.
All sweep results predate this and are void: the client never saw an opcode we intended.

### Server → client (previously)

**2026-08-15: the client accepted a frame we built and stayed connected.** A
`--reply header` run (valid header, body withheld, so nothing is dispatched) did not drop
the connection. That validates the header rule, the `0xFFFE` constant, the `K` IV seed and
the shuffle table in one shot. We can now talk to the client.


`tools/transport.py` builds packets the client will accept, and reproduces **all 16
captured client headers byte-exactly**:

```
a = ((iv >> 16) & 0xFFFF) ^ 0xFFFE      # 0xFFFE is hardcoded in the client
b = a ^ length                          # no byte-swap, unlike classic MapleStory
payload = AES-256-OFB(stock key, iv repeated 4x)
```

on the `conn+0xec` chain (the **second** u32 of the greeting, `K`), evolving once per
packet. Lengths `>= 0xFF00` use an 8-byte header with a 32-bit length.

**The AES key is the stock one.** `13 00 00 00 08 00 00 00 06 00 00 00 B4 …` — eight key
*words* at a stride of 4 dwords from `0x143A86810`; the 24 dwords interleaved between them
are decoys. `decrypt_capture.py` used to fold the decoys in, which silently invalidated
every sweep ever run against it. Don't reintroduce that.

### The open problem — and why it is not blocking

**Client → server payloads still do not decrypt**, now with the correct key and after
re-running every sweep (key forms, both IV chains × 24 positions, keystream alignment 0-8,
Shanda, mode 2). Scored on zero-richness rather than a guessed opcode: mean 0.22 zeros
over 1296 trials, best 2 — pure chance.

The reason is now known. `FUN_1415d3990` (the send entry point, called by both `0x70` and
`0x71` senders) hits `halt_baddata()` partway through, and `FUN_1415d60e0` — the step run
immediately after decryption in both receive loops — is a 22-byte stub that is *entirely*
`halt_baddata()`. **Themida has obfuscated exactly the two functions that would show the
extra transform.** Every primitive we can read is stock; more decompiling will not help.

Reading the client's traffic would take dynamic analysis. It is not needed to drive the
client, so it should not hold up the login server.

### Next steps, in order

1. ~~Validate the send direction against the client.~~ **Done — confirmed 2026-08-15.**
2. `-Reply sweep` to find the inbound opcode the client is waiting for. This also tests
   the payload path for the first time, since a sweep sends complete encrypted packets:
   if the very first one drops the connection, the AES/OFB side is wrong; surviving many
   opcodes means it is right. Use `-Pad` to append zero bytes after the opcode — a bare
   2-byte packet makes any handler that reads a body underflow, which can end the sweep
   on its first *handled* opcode.

   The **inbound** opcode space is the one thing we cannot recover statically — the
   dispatcher lives in `.themida`, which has no file bytes — so it has to come from the
   client. The sweep logs one timestamped line per opcode, so the last line printed
   before a disconnect or a UI change names the opcode responsible:

   ```powershell
   powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-one.ps1" -Variant 0 -Normal -Reply sweep -SweepFrom 0x0000 -SweepTo 0x0200 -Pad 32
   ```

3. Then implement the login server. Note the **game/channel connection uses mode 2**, a
   plain `byte - iv` subtract rather than AES, so it is far cheaper to talk to.

The **outbound** opcode map is already recovered: 657 distinct opcodes in
`research/msexe-send-opcodes.txt`, from `tools/ghidra_scripts/DumpOpcodes.java`.

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
