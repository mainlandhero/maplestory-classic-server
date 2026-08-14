# MapleCW

A local, testing-only server emulator for the MapleStory **`mscw`** client
(build ≈ Jan 2026, WZ data version **779**), written in Rust.

> **Scope.** This is reverse engineering of a locally-installed client for
> interoperability and testing, run entirely on one machine. It is not distributed and
> is never pointed at official servers. The original install at
> `C:\Nexon\Library\maplestorycw` is treated as **read-only reference** — all client
> patching happens on a separate copy.

**Start here:** [STATUS.md](STATUS.md) — current state and next steps.
[ROADMAP.md](ROADMAP.md) has the staged plan.

## Status

| Stage | Goal | State |
|---|---|---|
| 0 | Foundation: recon, WZ parser, protection map | **done** — 9,994/9,994 images parse |
| 1 | Patched client that launches and talks to localhost | **done** — `-NXLDEBUG 127.0.0.1 8484` connects, GameGuard never loads |
| 2 | Handshake | **in progress** — framing confirmed, client parses our body, rejection cause open |
| 2.5 | Auth server + our own launcher | auth server **done**; launcher blocked on the `WEBSTART` session fields |
| 3 | Login server → character select | not started |
| 4 | Channel server → walk a map | not started |
| 5 | Cash shop server | not started |
| 6+ | Mobs, drops, skills, NPCs, inventory | not started |

## Layout

```
crates/wz/      WZ archive parser + `wz-dump` CLI
docs/           format and protocol documentation
research/       reverse-engineering notes
```

## Build

```bash
cargo build --release
```

## `wz-dump`

The data trees are split: `<Tree>.wz` is a stub index and `<Tree>_000.wz` is the real
archive. Point the tool at the `_000` file.

```bash
# header, detected version, tree summary
cargo run --release -p wz --bin wz-dump -- info  "<Data>/String/String_000.wz"

# directory listing
cargo run --release -p wz --bin wz-dump -- tree  "<Data>/Mob/Mob_000.wz" 1

# decode one image to JSON
cargo run --release -p wz --bin wz-dump -- cat   "<Data>/String/String_000.wz" Mob.img

# open every archive under a Data dir and report health
cargo run --release -p wz --bin wz-dump -- scan  "<Data>"

# deep check: parse every image in every archive
cargo run --release -p wz --bin wz-dump -- verify "<Data>"
```

where `<Data>` is `C:\Nexon\Library\maplestorycw\appdata\Data`.

## Key findings so far

- **WZ version 779**, hash `0x0000E73A`, and a **zero string key** — no AES on strings,
  a significant simplification over GMS/KMS builds. See [docs/wz-format.md](docs/wz-format.md).
- The client keeps the classic **Login (8484) / World / Channel (5160)** topology and the
  original `COutPacket`/`CInPacket`/`CWvsContext` class structure.
- The real obstacles are **nProtect GameGuard** (kernel driver) and **MapleSecurePC64**
  (packet crypto), not the data format. See [research/client-recon.md](research/client-recon.md).
