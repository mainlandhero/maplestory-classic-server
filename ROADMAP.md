# MapleCW — Local Server Emulator

Goal: a **local, private, testing-only** server for the MapleStory "CW" client shipped in
`C:\Nexon\Library\maplestorycw\appdata`. Long-term target: **fuller gameplay** (login →
world/channel → walk a map → mobs/drops/skills/NPCs/inventory), built in stages so each
stage is usable on its own.

The deliverable is **three parts**, all of which have to be built:

1. **The server software** — auth server, login server, world/channel registry, game
   (channel) server, and the cash shop server, plus the shared codec, game-data, and
   persistence layers. See *Server architecture* below.
2. **Our own launcher** — a small client that authenticates the user against our auth
   server and then starts `MapleStory.exe` in `WEBSTART` mode with the resulting session.
   Nexon's launcher normally does this; the game will not proceed without it.
3. **A patched local client** to talk to it, made from a separate copy so the original
   install is never modified.

**Persistence: SQLite throughout.** Everything is local and single-machine, so there is
no reason for anything heavier.

Scope guardrails:
- Everything stays on the local machine. This is reverse engineering our own client for
  interoperability/testing. It crosses Nexon's ToS, so it is never distributed or used online.
- The **original client install is never modified.** All patching happens on a *separate copy*.

---

## Client reconnaissance (established facts)

Source: static analysis of `MapleStory.exe` (76.7 MB) + patch manifest, 2026-08-14.

- **Codename `mscw`**, 64-bit. PDB path: `c:\build\src\mscw_game\Bin64\MapleStory.pdb`.
- **Build ≈ Jan 2026** (`buildtime 1786489625`).
- **Classic "Wvs" architecture, modernized.** Binary still contains canonical class names:
  `COutPacket`, `CInPacket`, `CWvsContext`, `CLoginQueueDlg`. Server model is the traditional
  MapleStory one: **Login (port 8484)**, **World**, **Channel (port 5160)**.
- **Version-check handshake present**: strings `First Connect : nClientVersion_Temp : %d`,
  `OK. Allowed Version. %d ( %d~%d )`, `Low Version. Launch Patch`, `High Version. Error.`
- **Server address candidates found in binary** (leftover dev/live tables):
  - Login: `10.9.2.131/132/133` port `8484`, tag `LIVE`; also `175.207.3.196/238/239` (KR).
  - Channels: `192.168.128.75..84:5160` (internal pool); `54.180.211.235:24200`,
    `43.200.157.16:24200` (AWS).
- **Launch flags** (client is normally started by the Nexon launcher):
  `GAMELAUNCHING`, `WEBSTART`, `STEAMSTART`, `IPPORT`, `autologin`, `skiplogo`, `-NXL`,
  `-NXLDEBUG`, `-NXLPTS`, plus debug flags `noquest`, `debugwnd`, `showcode`, `fastskillui`…
- **Login backends**: Nexon Passport (`CNMLoginNexonPassportFunc`) + Steam (`steam_api64.dll`).
- **Data = standard WZ**, split layout: `X.ini` (`LastWzIndex|N`) + header `X.wz`
  (`PKG1`, "Package file v1.0 Copyright 2002 Wizet, ZMS") + data blobs `X_000.wz`.
  All 17 trees present: Base, Character, Effect, Etc, Item, Map, Mob, Morph, Npc, Quest,
  Reactor, Skill, Sound, String, TamingMob, UI.

### Protections (the real work is here, not the WZ data)

1. **nProtect GameGuard** — `grap\BlackCat64.sys` (kernel driver), `grap\NGService.exe`,
   encrypted `grap-core64.aes` / `grap-communicator64.aes` / `grap-updater.aes`. Kernel-level
   anti-cheat; blocks debuggers & packet capture, may refuse to run outside Nexon's environment.
2. **MapleSecurePC64.dll** + **jypc.dll** — Nexon packet encryption / anti-tamper. Client will
   not speak plaintext classic-MapleStory crypto until this is understood or neutralized.
3. **No public data for this build** — existing emulators (HeavenMS/Cosmic/…) target
   2009-era v83 clients and **do not apply**. Much of this is now recovered rather than
   unknown: protocol version **100**, the full wire cipher (AES-256-OFB with a key that is
   a decoy on disk), the greeting layout, and the login-stage inbound opcode map. See
   `docs/transport.md`, `docs/handshake.md`, `docs/opcodes.md` and `STATUS.md`.

---

## Strategy: the separate client copy

We produce a *second copy* of the client (`client-patched/`) that:
- launches standalone (no Nexon launcher / no Passport),
- has GameGuard + MapleSecurePC neutralized,
- points its login socket at `127.0.0.1:8484`.

Original `C:\Nexon\Library\maplestorycw\appdata` is read-only reference; never touched.

> **Revised 2026-08-14 after PE analysis.** `MapleStory.exe` is **Themida-protected with
> a rebuilt IAT**, so it checksums itself: *editing bytes in the exe should be assumed to
> fail.* See `research/protection-surface.md`. The plan is therefore **configuration and
> module substitution first, binary patching last**:
>
> 1. **`IPPORT` launch argument** — may redirect the client to `127.0.0.1` with no
>    patching whatsoever. Cheapest test, biggest payoff. Try first.
> 2. **Stub `grap64.dll`** — GameGuard's interface is only two exported functions
>    (`__syscall_Common_Param8/16`). It is a static import so the file must exist, but a
>    no-op replacement is small and well-defined.
> 3. **Block/stub `MapleSecurePC64.dll`** — dynamically loaded, so easier to displace
>    than GameGuard.
> 4. **Network-layer redirect** (`netsh portproxy`) as a client-untouched fallback. Note
>    the client targets hard-coded **IPs**, so a `hosts` file will not work.
>
> The good news: `.text` is 52.8 MB of ordinary unencrypted code, so Ghidra can read the
> game logic. Because the IAT is obfuscated, find the socket code via **string xrefs**,
> not import xrefs.

---

## Staged plan

Each stage ends in something observable.

### Stage 0 — Foundation & analysis  ✅ done
- [x] Reconnaissance of client (done; see above).
- [x] Rust workspace + toolchain (rustc 1.97.1 msvc).
- [x] PE protection-surface map — all 42 binaries surveyed. `MapleStory.exe` and
      `NGService.exe` are Themida-protected; `MapleSecurePC64.dll` and `BlackCat64.sys`
      are packed; `grap64.dll`, `jypc.dll`, `ZLZ64.dll` are **clean and analysable**.
      See `research/protection-surface.md`.
- [x] WZ parser (`crates/wz`): version **779** / hash `0x0000E73A` / **zero** string key.
      Verified across the whole client: **9,994/9,994 images in 102 archives parse**.
- [x] RE tooling: **Ghidra 12.1.2** at `C:\Users\user\Desktop\ghidra_12.1.2_PUBLIC`,
      running headless on JDK 21. Projects in `research/ghidra/` (gitignored).
      `MapleSecurePC64.dll` imported; `MapleStory.exe` analysis is long-running.

### Stage 1 — Client bring-up  ✅ done
- [x] Launch mode: **`IPPORT` crashes**; `-NXLDEBUG <ip> <port>` is the only mode that runs
      *and* connects. `WEBSTART` needs session fields we cannot fake.
- [x] `client-patched/` with a stub `grap64.dll`. **GameGuard never loads**, and Themida
      does not validate it.
- [x] Listener on 8484 capturing the client's first bytes.

### Stage 2 — Crypto & handshake  ✅ done
- [x] Handshake reversed — the gate was fields `G == 1` **and** `H == 1`, not the version.
      See `docs/handshake.md`.
- [x] Framing and cipher in `crates/net`, **verified in both directions**. The on-disk AES
      key is a decoy; the real one is read from the running client. See `docs/transport.md`.

### Stage 2.5 — Auth server + our own launcher
**Premise corrected 2026-08-17.** This stage used to open "the client will not get past
startup without the session handoff its launcher normally performs", on the strength of bare
`WEBSTART` exiting cleanly. That is no longer true of the route we take: **`-NXLDEBUG` puts
the client in mode 5 and it reaches character select with no launcher at all**, which is how
every run this month has worked. A launcher is now wanted for the reasons the owner gave - applying
the client patches without a PowerShell script, and taking a username and password so a
session is validated - not because startup demands one. Design in **`docs/launcher.md`**;
the work is listed under Stage 3.5.

- [x] **The handoff mechanism, partly.** `-NXLDEBUG` routes launch arguments from the third
      onward into the config's six-slot session array at `+0x90`; `test-one.ps1
      -SessionTokens` exercises it. **Still to measure:** whether that array is what outbound
      `0x0073` transmits, and in which field - which is what decides whether a launcher-issued
      token can gate the login result.
- [ ] `crates/auth`: HTTP/JSON auth server, SQLite accounts, issues short-lived session
      tokens. Stands in for Nexon Passport.
- [ ] `crates/auth` **user-admin CLI** (`maplecw-useradd`): create/list/disable accounts
      and reset passwords, run locally against the SQLite file. Reads the password from
      a prompt or stdin — never from an argument, which would leak it into shell history
      and the process list.

**Credential handling rules (non-negotiable, even though this is local):**
- Passwords are **never stored in plain text** and never stored reversibly.
- Hash with **argon2id** — memory-hard, so it stays expensive to attack. Each password
  gets a **unique random salt**, stored with the hash in PHC string format.
- Verification is constant-time; a wrong username and a wrong password are
  indistinguishable to the caller.
- Passwords are never logged, never echoed to the terminal, and never placed in a
  command-line argument.
- Session tokens are random (from a CSPRNG), short-lived, and stored hashed, so a leaked
  database does not hand over live sessions.
- [ ] `crates/launcher`: authenticates against `crates/auth`, then starts the client. Not
      `WEBSTART` - `-NXLDEBUG <server> <port> <token...>`, the mode we actually use. See
      Stage 3.5 and `docs/launcher.md`.
- [ ] The login server (Stage 3) validates the same session token, so the two agree on
      who the player is.

### Stage 3 — Login server  ← **current**, and mostly done
- [x] Reach the login screen (inbound `0x0032`), the **world list** (`0x000B`, which is also
      what enables the Login button), and **character select**.
- [x] The **account name / masked email** (`0x0000`) — server-supplied, on screen.
- [x] **The character creation transaction, on the wire.** Character list inside `0x0010`,
      `0x00A8`/`0x05F4` to open the screen, name check `0x0081`/`0x0014`, create request
      **`0x008A`** → `0x0015`, and the client returns to character select with the new
      character. Every opcode measured; the create builder is virtualised, so a capture was
      the only route. Full decode in **`docs/character.md`**.
- [x] **The ~37s client exit — solved.** The client ran a server-reachability check over
      twenty hardcoded IPs about 36s after launch; the firewall made all twenty fail, and the
      Themida-virtualised routine handling that overran a 512-byte stack buffer into its own
      `/GS` cookie, so `__report_gsfailure` raised `int 0x29` and the process died with
      `0xC0000409`. A latent client bug on a path that never runs in production.
      `-SkipNetCheck` returns from the check immediately; the client now runs indefinitely.
- [x] **The created character keeps what was picked.** The create reply is built from the
      request rather than replayed, with an incrementing id, and the avatar look's `face`
      and `hair` were being written one field too early — see `docs/character.md`.
- [x] `0x0082` (leave world) answered, and the `0x0080` world sequence made standing, so
      "Choose another world" and re-entering a world both work.
- [x] **`crates/login` — built 2026-08-18**, the priority the owner set on 2026-08-17. A real
      server, no canned bodies, and **characters persist between launches**. Three modules:
      `handshake` (the greeting), `session` (the protocol as a pure state machine — no
      socket, no clock, so every measured exchange is a unit test), `server` (the socket
      loop and the log). `maplecw-login --list` prints what is stored without listening.
      Run it against the client with **`tools/test-server.ps1`**. Full notes in
      **`docs/login-server.md`**.
- [x] **Storage for characters.** `crates/store/src/character.rs`: a `characters` table
      whose columns match the protocol's `Character` field for field, and an `equipment`
      table keyed by character and slot, both cascading from `accounts`. The row maps to
      `net::opcode::Character` by exhaustive destructure in both directions, so a new
      protocol field breaks the build until a column exists. Names are unique across the
      service and case-insensitive, which is what makes the name check truthful.
- [x] **The name check is answered from the database.** The harness replied "available" to
      every name including ones it had already handed out; the server now distinguishes
      available, already used, and not allowed.
- [ ] **Confirm it on screen.** The end-to-end transport and every reply are verified
      against `tools/login_smoke.py`, a stand-in client built on the independent Python
      transport — including a stop-and-restart with the character still listed. What that
      cannot show is the client's *reaction*. One launch settles it: create a character,
      close, relaunch, and look at the list.
- [ ] **The account is configuration, not a login.** The game socket carries no
      credentials, so `--account` decides whose characters every connection sees. Closing
      this is Stage 3.5 — see `docs/launcher.md`.
- [ ] Find what the real service sends to enable character creation. The flag is set by
      `FUN_140c9e230` from **virtualised** code; we currently call it ourselves from
      `grap-stub` (`-Session create=on`), which is a client patch, not the protocol.
- [ ] **Migration is an open question again** — the evidence for "the client never
      migrates" was retracted; see `STATUS.md`.
- [ ] `crates/world`: world/channel registry the login server advertises, and the
      migration token store both sides validate against.
- [ ] `crates/data`: typed loaders over `crates/wz` (mobs, items, skills, maps, strings).
- [ ] Character **deletion**. `crates/store` has `delete_character` with an ownership
      clause in the statement, and the client has a delete result (`0x0016`, body a `u32`
      character id), but the request opcode has not been identified and nothing is wired up.
- [ ] Character **slot count** comes from the constant `3` rather than from the account.

### Stage 3.5 — Off-box deployment and a launcher  ← *added 2026-08-17*
The owner will host this on a homelab box, so the client and the server are **not** the same
machine, and they want a minimal launcher that applies the client patches and takes a
username and password. Designs: **`docs/deployment.md`** and **`docs/launcher.md`**.
- [ ] `firewall.ps1 -AllowServer <ip>`: the blanket outbound block has been harmless only
      because Windows Firewall does not filter loopback. Off-box it blocks our own traffic
      too, so it must become a block whose remote address is the complement of the server.
      The twenty Nexon addresses stay blocked, so `-SkipNetCheck` is still required.
- [ ] Split **bind** from **advertise** in every service config. Whatever packet carries a
      channel address must carry one the *client* can reach; which packet that is is not yet
      established (candidate: `0x0011`, Stage 4).
- [ ] `crates/auth`: configurable bind (it hardcodes `127.0.0.1`) and TLS. Never expose it
      to the internet. Credentials must never travel on the game socket - that cipher is
      obfuscation, not security.
- [ ] **Measure whether the session array at config `+0x90` is what outbound `0x0073`
      transmits**, and in which field. `-NXLDEBUG` fills it from launch arguments 3 onward
      and `-SessionTokens` already exercises it; six distinguishable tokens and one capture
      settles it. This decides whether the launcher's token can gate the login result.
- [ ] `crates/launcher` (Rust + `eframe`/`egui`, one static exe): verify the client build by
      hash before patching — every patch is an absolute VA — install `grap64.dll`, write one
      `maplecw.toml` in place of the marker files, authenticate, launch.
- [ ] Retire patches rather than accumulate them. `suppress_login_dialog` hides a result
      code 12 we should be preventing; `enable_creation` stands in for a packet we have not
      found; `mode=2` may already be unnecessary — it is applied ~100ms *after* the client
      auto-logs in, so test whether anything breaks without it.

### Stage 4 — Channel server / enter world
- [ ] `crates/channel`: accept a migrating client, spawn the character into a map.
      Needs server-side Map WZ (portals, spawns, footholds).
- [ ] **Lead, not yet investigated:** the owner recalls the live service dropping a newly created
      character straight into the starter map. `FUN_141b36a10` does *not* do that — its
      success path ends at character select unconditionally — so a further packet must. The
      candidate is inbound **`0x0011`** (`FUN_141b36f60`, 4821 bytes), which the opcode-name
      mapping calls `SelectCharacterResult`, i.e. the packet that would carry a channel
      address. That is also where the retracted migration question gets settled.
- [ ] Movement, chat. **Walk around a map.**

### Stage 5 — Cash shop server
The cash shop is a **separate server with its own connection**: the client disconnects
from the channel, migrates to the cash shop, and migrates back on exit. It needs its own
handler set, not a menu inside the channel server.
- [ ] `crates/cashshop`: accept the migration, serve the shop UI's item catalogue from
      `Item/Cash` + `String/Cash.img`, and migrate the client back to a channel.
- [ ] Wallet (NX/maple points) in `crates/store`; purchase → cash inventory grant.
- [ ] Cash inventory as a distinct storage area from the normal inventory.

### Stage 6+ — Gameplay systems
- [ ] Mob spawns/AI/damage, drops, loot, inventory, skills, NPCs/shops, quests, parties…
      Driven by extracted WZ data (Mob/Skill/Npc/Quest/Reactor/String).

---

---

## Server architecture

MapleStory is **not** one server process. The client holds **one TCP connection at a
time** and is handed between servers by *migration*: the current server sends the client
an address plus a one-time token, the client disconnects, reconnects to the new address,
and presents the token. Every stage below has to implement that hand-off correctly.

```
   ┌──────────────┐   HTTP    ┌────────────────┐
   │ our launcher │──────────▶│  Auth  server  │  account login, issues a session
   │  (we write)  │◀──────────│   (HTTP/JSON)  │  token; SQLite-backed
   └──────┬───────┘  session  └────────────────┘
          │ starts MapleStory.exe WEBSTART + session
          ▼
                    ┌────────────────┐
   client ─────────▶│  Login  :8484  │  version check, auth, world/channel list,
                    │                │  character list/create/delete
                    └───────┬────────┘
                            │ migrate (ip:port + token)
                            ▼
                    ┌────────────────┐
                    │ Channel :5160+ │  the actual game world: maps, movement,
                    │  (one per ch.) │  mobs, drops, skills, NPCs, parties
                    └───┬────────┬───┘
             migrate    │        │   migrate back
                        ▼        ▲
                    ┌────────────────┐
                    │   Cash Shop    │  separate connection; NX wallet,
                    │                │  catalogue, cash inventory
                    └────────────────┘

        ┌───────────────────────────────────────────────┐
        │ World registry + migration tokens (shared)    │
        │ Persistence: accounts, characters, inventory  │
        │ Game data: loaded from WZ at startup          │
        └───────────────────────────────────────────────┘
```

Cross-cutting pieces every server needs: the packet codec (`crates/net`), the WZ-backed
game data (`crates/data`), and persistence (`crates/store`).

## Workspace layout
```
MapleCW/
  crates/
    wz/         WZ archive parser + extractor        [done]
    grap-stub/  no-op grap64.dll, keeps GameGuard off [Stage 1]
    net/        packet primitives, framing, ciphers  [done]
    auth/       HTTP auth server, session tokens     [Stage 2.5]
    launcher/   our launcher: auth -> WEBSTART       [Stage 2.5]
    data/       typed game data loaded from WZ       [Stage 3]
    store/      SQLite persistence (accounts, chars) [Stage 3]
    world/      world+channel registry, migration    [Stage 3]
    login/      login server binary                  [Stage 3]
    channel/    channel/game server binary           [Stage 4]
    cashshop/   cash shop server binary              [Stage 5]
  tools/        CLI utilities (WZ dumping, PE analysis, client setup)
  research/     RE notes, byte fixtures, protection analysis
  docs/         protocol/opcode/data-format documentation as we learn it
```

## Open questions / decisions log

Decided:
- Server language: **Rust**. C++ was the alternative.
- WZ parameters: **version 779**, hash `0x0000E73A`, **zero** string key. Settled
  empirically; 9,994/9,994 images parse.
- RE disassembler: **Ghidra 12.1.2**, headless on JDK 21.
- Anti-cheat: **stub `grap64.dll`** so GameGuard never loads, rather than fighting the
  kernel driver. Original install untouched; work happens on a copy.
- Client patching: **avoid it.** Themida checksums the image, so prefer configuration
  and module substitution.
- **We write our own launcher and auth server** rather than trying to remove the
  client's launcher-session requirement.
- **Persistence: SQLite** everywhere, since this is local and single-machine.

Open:
- **How does the launcher hand the session to the game?** Command line, registry,
  environment, named pipe, or token file. Blocks Stage 2.5; answer via Ghidra string
  xrefs on `WEBSTART`, or Process Monitor on a live launch.
- **What does the client speak on the wire?** `MapleSecurePC64.dll` wraps the socket
  layer. `crates/net` keeps the cipher swappable; the classic AES+shanda scheme is the
  first hypothesis to test.
- Opcode table: entirely unknown for this build, and must be recovered empirically.
- Whether Themida validates a substituted `grap64.dll` — untested, because startup fails
  earlier for unrelated reasons.
