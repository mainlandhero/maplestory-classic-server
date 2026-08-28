# MapleCW

A local, testing-only server emulator for the MapleStory **`mscw`** client
(build ≈ Jan 2026, WZ data version **779**), written in Rust.

> **Scope.** This is reverse engineering of a locally-installed client for
> interoperability and testing, run entirely on one machine. It is not distributed and
> is never pointed at official servers. The original install at
> `C:\Nexon\Library\maplestorycw` is treated as **read-only reference** — all client
> patching happens on a separate copy.
>
> **No game content is in this repository.** No client binary, no `.wz` archive, no extracted
> asset — `.gitignore` keeps `client-patched/` and `gm-handbook/` out, and the latter is
> regenerated from the reader's own install. What is here is source, protocol notes, and
> packet captures against a private localhost server.
>
> Captured logs have the capturing machine's **MAC address and machine id redacted**, with
> length-preserving placeholders. See
> [`research/fixtures/README.md`](research/fixtures/README.md) — those particular bytes are
> not data.

**Start here:** [STATUS.md](STATUS.md) — current state and next steps.
[ROADMAP.md](ROADMAP.md) has the staged plan.

## Status

| Stage | Goal | State |
|---|---|---|
| 0 | Foundation: recon, WZ parser, protection map | **done** — 9,994/9,994 images parse |
| 1 | Patched client that launches and talks to localhost | **done** — `-NXLDEBUG 127.0.0.1 8484` connects, GameGuard never loads |
| 2 | Handshake | **done** — framing, the AES key, and the asymmetric channel cipher |
| 2.5 | Auth server | **done** |
| 3 | Login server → character select | **done** — list, create, delete, persistence |
| 4 | Channel server → walk a map | **done** — portals, footholds, NPCs, chat |
| 5 | Cash shop | **done** — the window, the wallet, a purchase into the locker |
| 6 | Mobs, drops, EXP, levelling, inventory, storage, quests | **done** — all seen on screen |
| 7 | Skills | first job for all four branches; buffs, SP and the skill book work |
| 8 | NPC shops | the classic `0x055D` counter draws and sells |

**Open**, and each written up rather than hand-waved: attack skills cost no MP and no hit is
validated yet; a heap wild-write kills a long session; and the client draws its own damage
number as `1` for any mob without an attack node.

Claims are tagged **[L]** read off the listing or a capture, **[D]** derived, **[I]** inferred.
The distinction is load-bearing, and [`CLAUDE.md`](CLAUDE.md) is largely a catalogue of the
times it was got wrong.

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
