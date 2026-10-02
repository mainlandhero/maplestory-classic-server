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
[STATUS-history.md](STATUS-history.md) has the finished goals, kept for the method.

## What you need that is not in this repository

The repository is code only. To run a server you supply the game and the toolchain yourself:

| what | why | where it comes from |
|---|---|---|
| **MapleStory Classic World** (`mscw`) from the second closed online test (**COT#2**), installed | the client, and the source of every game-data table the server loads | the Nexon Launcher. The server is built against **WZ data version 779** (client build of January 2026); a later patch may need [`docs/wz-changes.md`](docs/wz-changes.md) |
| **Windows 10 or 11, x64** | the client, the launcher and the scripts are Windows-only | - |
| **Rust** (stable, MSVC toolchain) | builds the servers, the launcher and the client hook | [rustup](https://rustup.rs), plus the **Visual Studio Build Tools** "Desktop development with C++" workload for the linker |
| **Python 3.10 or later** | generates the game-data tables from your client | [python.org](https://www.python.org). Standard library only; `capstone` is needed only by the reverse-engineering tools |
| **Visual C++ 2015-2022 Redistributable (x64)** | the launcher and the injected hook import `VCRUNTIME140.dll` | Microsoft (`vc_redist.x64.exe`); usually already installed |
| **Windows PowerShell 5.1**, run **as administrator** | the scripts; the game client itself demands elevation | built into Windows |

Not needed to run a server: Ghidra and a JDK (reverse engineering only, [`docs/ghidra.md`](docs/ghidra.md)),
and the modern MapleStory client (only `tools/backport_signature_style.py` reads it, to add the
backported cosmetics).

## Getting started: a server on your own machine

The paths below assume the repository is at `C:\MapleCW` and the client is installed at
`C:\Nexon\Library\maplestorycw`; substitute your own. Open **PowerShell as administrator**.

**1. Make a copy of the client to patch.** The original install is never modified - everything
patches the copy in `client-patched\`, whose [README](client-patched/README.md) shows what
belongs there. Only that README is tracked; the client itself is gitignored.

```powershell
robocopy "C:\Nexon\Library\maplestorycw\appdata" "C:\MapleCW\client-patched" /E
```

**2. Build everything.**

```powershell
cd C:\MapleCW
cargo build --release
```

**3. Generate the game data** (`gm-handbook\`: maps, portals, mobs, items, skills, quests, ...)
from your copy of the client. Without it the server starts with empty tables.

```powershell
python C:\MapleCW\tools\build_handbook.py
```

**4. Optional: the quest patch.** Lets every class do the Maple Island quests (the client
otherwise limits them to Beginners). Run it with the client closed; the release packager
requires it.

```powershell
python C:\MapleCW\tools\quest_patch.py --install
```

**5. Create your account, and make it a GM** - see [Managing accounts](#managing-accounts) for
the rules and everything else the tool does.

```powershell
C:\MapleCW\target\release\maplecw-useradd.exe --db C:\MapleCW\maplecw.db Wisp --email wisp@example.com
C:\MapleCW\target\release\maplecw-useradd.exe --db C:\MapleCW\maplecw.db --gm Wisp
```

**6. Start it.**

```powershell
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1"
```

That rebuilds what changed, installs the hook into `client-patched\` (GameGuard is replaced by a
no-op stub and its folder disabled - nothing is installed system-wide), starts the sign-in
service, the login server, two channels and the world hub, and opens the launcher. Sign in with
the account from step 5 and press **Start Game**. Add `-ServersOnly` to start only the servers
and run `target\release\maplecw-launcher.exe` yourself.

What runs where - on `127.0.0.1` under this script, except the drop page:

| port | service | log |
|---|---|---|
| 8480 | sign-in (`maplecw-auth`, TLS) | `auth.log` |
| 8484 | login (`maplecw-login`) | `login.log` |
| 8485, 8486 | channels 0 and 1 (`maplecw-world`) | `world-ch0.log`, `world-ch1.log` |
| 8483 | world hub (`maplecw-chat`), loopback only | `chat-hub.log` |
| 8481 | the drop-table web page - on **all interfaces** (`--drops-web none` turns it off) | - |

**Check it without the client:** `python C:\MapleCW\tools\channel_smoke.py` drives a real channel
over an independent Python implementation of the protocol and should print `all checks passed`.

**Playing from other machines:** `tools\package-server.ps1` builds a self-contained server
(`out\MapleCW-server.zip`) and `tools\make-installer.ps1` the client payload; see
[`docs/deployment.md`](docs/deployment.md), [`docs/server-machine-checklist.md`](docs/server-machine-checklist.md)
and [`docs/client-machine-checklist.md`](docs/client-machine-checklist.md).

**Nothing authenticates the game socket.** The launcher signs in over TLS and the login server
ties the client to that sign-in, but the game protocol itself carries no credentials. Run it on
a network you trust.

## Managing accounts

Accounts live in the server's database (`maplecw.db`, beside the repository or the packaged
server) and are managed with **`maplecw-useradd`**, which `cargo build --release` puts in
`target\release\` and the server package ships in `bin\`. Every command takes **`--db <path>`**;
without it the tool opens `maplecw.db` in the current directory, which in an administrator window
is `C:\Windows\System32` - so always pass it. The servers can keep running while you use it.

```powershell
$useradd = "C:\MapleCW\target\release\maplecw-useradd.exe"
$db      = "C:\MapleCW\maplecw.db"
```

**Create an account.** The password is asked for twice at a hidden prompt (or read from piped
input) and is never accepted on the command line, where it would land in shell history. Only an
argon2id hash is stored.

```powershell
& $useradd --db $db Wisp                              # name only
& $useradd --db $db Wisp --email wisp@example.com     # with an email (put --email AFTER the name)
```

* **Name:** 3 to 24 characters, letters, digits and `_` only.
* **Password:** at least 8 characters. Players registering through the launcher must also use at
  least one letter and one digit.
* **Email** is optional. The launcher's sign-in box accepts either the name or the email.

**Give or take GM rights** - the `!` commands (`!map`, `!item`, `!exp`, `!job`, `!heal`, ...).
A GM flag says which *account* may use them; it is authorisation, not authentication.

```powershell
& $useradd --db $db --gm Wisp
& $useradd --db $db --no-gm Wisp
```

**Everything else:**

| command | what it does |
|---|---|
| `--list` | every account: id, name, email, state (enabled or disabled), GM, last login |
| `--passwd <name\|email>` | set a new password (the old one is not asked for) |
| `--email <name> <address>` | set an email; `""` clears it |
| `--disable <name>` / `--enable <name>` | stop or allow an account signing in |
| `--claims` | who is signed in through a launcher, one row per launch |
| `--clear-claims` | forget them all, if a crashed launcher left an account marked as playing |

**Players on other machines** have no database and no `maplecw-useradd`, so they register in the
launcher instead, with a single-use code you give them:

```powershell
& $useradd --db $db --registration-code               # valid 7 days: the player enters it on Register
& $useradd --db $db --recovery-code Wisp              # valid 24 hours: lets that account set a new password
& $useradd --db $db --codes                           # how many of each are still live
```

A GM can mint the same codes in game with `!registrationcode` and `!recoverycode <name|email>`.
The code is printed once and only its hash is kept, so copy it before closing the window.

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

## License

The code in this repository is released under the **PolyForm Noncommercial License 1.0.0** - see
[`LICENSE`](LICENSE). In short:

* **Free for any noncommercial purpose** - personal use, study, research, hobby projects, and use
  by charities, schools and public bodies. You may copy, modify and share it.
* **No commercial use.** Running it as a paid service, selling it, or using it for a business
  purpose needs separate permission from the copyright holder.
* **Credit is required.** Anyone who passes on any part of it must include the license terms
  (or their URL) and this line from the top of `LICENSE`:
  `Required Notice: Copyright (c) 2026 mainlandhero (https://github.com/mainlandhero/maplestory-classic-server)`

This makes the project **source-available, not "open source"** in the OSI sense - the Open
Source Definition does not allow a ban on commercial use.

What the license does **not** cover, because it is not this project's to license:

* **MapleStory itself** - the client, its WZ data, art, sound and text belong to their owners.
  Nothing here ships the client; `gm-handbook/` and `client-patched/` are generated locally
  and gitignored.
* **Decompiled client excerpts** under `research/` (`msexe-*.c` and the listings quoted in the
  notes) are reverse-engineering notes about software this project does not own.
* **Data gathered from other sites** (`data/drops.txt` and others name their source in their own
  headers) - check the source's terms before reusing it outside this project.
