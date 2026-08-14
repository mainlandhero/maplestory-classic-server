# MapleCW — Local Server Emulator

Goal: a **local, private, testing-only** server for the MapleStory "CW" client shipped in
`C:\Nexon\Library\maplestorycw\appdata`. Long-term target: **fuller gameplay** (login →
world/channel → walk a map → mobs/drops/skills/NPCs/inventory), built in stages so each
stage is usable on its own.

The deliverable is **two halves**, both of which have to be built:

1. **The server software** — login server, world/channel registry, game (channel) server,
   and the cash shop server, plus the shared codec, game-data, and persistence layers.
   See *Server architecture* below.
2. **A patched local client** to talk to it, made from a separate copy so the original
   install is never modified.

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
3. **No public data for this build** — version number, opcode table, and crypto handshake
   (AES variant / IV seeds) are all unknown for `mscw`. Existing emulators (HeavenMS/Cosmic/…)
   target 2009-era v83 clients and **do not apply**.

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

### Stage 0 — Foundation & analysis  ← current
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

### Stage 1 — Client bring-up  ← next
Ordered cheapest-first, per the revised strategy above.
- [ ] Test the **`IPPORT`** launch argument on a copied client — can we set the server
      endpoint without patching anything?
- [ ] Determine whether the client will start with GameGuard absent/stubbed, and whether
      Themida validates `grap64.dll`.
- [ ] Make `client-patched/` accordingly (stub `grap64.dll`, block `MapleSecurePC64.dll`).
- [ ] Stand up a bare TCP listener on 8484; capture the raw first bytes the client sends.

### Stage 2 — Crypto & handshake
- [ ] Reverse the initial handshake: version, sub-version/locale, IV seeds, cipher.
- [ ] Implement the framing + cipher in `crates/net`; round-trip a handshake with the client.

### Stage 3 — Login server
- [ ] `crates/login`: version check → (stub) auth → world list → channel select →
      character list → **migration** hand-off to a channel. Reach **character-select**.
- [ ] `crates/world`: world/channel registry the login server advertises, and the
      migration token store both sides validate against.
- [ ] `crates/data`: typed loaders over `crates/wz` (mobs, items, skills, maps, strings).
- [ ] `crates/store`: persistence for accounts and characters (SQLite to start).

### Stage 4 — Channel server / enter world
- [ ] `crates/channel`: accept a migrating client, spawn the character into a map.
      Needs server-side Map WZ (portals, spawns, footholds).
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
    net/        packet framing + crypto              [Stage 2]
    data/       typed game data loaded from WZ       [Stage 3]
    store/      accounts/characters persistence      [Stage 3]
    world/      world+channel registry, migration    [Stage 3]
    login/      login server binary                  [Stage 3]
    channel/    channel/game server binary           [Stage 4]
    cashshop/   cash shop server binary              [Stage 5]
  tools/        CLI utilities (WZ dumping, PE analysis, client setup)
  research/     RE notes, byte fixtures, protection analysis
  docs/         protocol/opcode/data-format documentation as we learn it
```

## Open questions / decisions log
- Server language: **Rust** (chosen). C++ was the alternative.
- WZ string-decryption key: TBD (KMS/zero/custom) — determine empirically in Stage 0.
- RE disassembler: TBD (Ghidra likely).
