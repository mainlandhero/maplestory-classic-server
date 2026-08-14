# Protection surface map

PE analysis of every binary in `appdata`, 2026-08-14, via `tools/pe_surface.py`,
`tools/pe_imports.py`, `tools/pe_packing.py`. Read-only; nothing was modified.

## Verdict per binary

### Protected

| Binary | Size | Protection |
|---|---|---|
| **MapleStory.exe** | 76.7 MB | **Themida/WinLicense** (`.themida` 20.9 MB virtual / 0 raw, `.boot` 12.8 MB @ entropy 7.91) + **IAT obfuscation** |
| **NGService.exe** | 4.3 MB | **Themida/WinLicense** (GameGuard service) |
| **MapleSecurePC64.dll** | 2.7 MB | Custom packer: random section names (`ggshcdnj`, `gfhcpfgb`), entropy 7.88–7.93, 2 imports total, 4.5 MB virtual / 512 B raw unpack target |
| **BlackCat64.sys** | 4.1 MB | GameGuard kernel driver, `.text` entropy 7.52 |
| NexonAnalytics64.dll | 6.2 MB | `.text` entropy 7.42 |

### Clean (statically analysable)

`grap64.dll` (13.3 MB), `ZLZ64.dll`, `jypc.dll`, `WzMss.dll`, `Canvas.dll`,
`NameSpace.dll`, `Gr2D_DX11.dll`, `Shape2D.dll`, `Sound.dll`, `ResMan.dll`, `PCOM.dll`,
`MachineIdLib.dll`, `HybridCore64.dll`, `nexon_api_x64.dll`, `nmcogame64.dll`,
`nmconew64.dll`, `nps64.dll`, `steam_api64.dll`, the OpenSSL/curl set, and all of
`NxOverlay\`.

## What Themida actually costs us

The good news is narrower than "the exe is packed":

- **`.text` is 52.8 MB at entropy 6.44 — ordinary, unencrypted x64 code.** The game
  logic is present on disk and disassembles normally. Strings we already recovered
  (version-check messages, `COutPacket`/`CInPacket`, the server address tables) live there.
- Themida here is a **wrapper**, not whole-binary encryption: `.boot` bootstraps, and
  `.themida` is runtime-allocated VM space.

The costs:

1. **IAT is rebuilt at runtime.** All 34 imported DLLs list exactly one import each —
   the giveaway. `WS2_32.dll` statically imports only `gethostbyname`; `connect`, `send`,
   and `recv` are resolved dynamically. **API calls will appear as indirect calls through
   Themida stubs**, so cross-referencing by import name will not find the socket code.
   Locate it via string xrefs instead.
2. **Anti-tamper.** Themida checksums the image. **Patching bytes in `MapleStory.exe` on
   disk should be assumed to fail.** This is the single most important constraint on
   Stage 1 and it rules out the naive "patch the client" plan.
3. **Runtime anti-debug.** Expect debugger detection; attaching to a running client is a
   fight, independent of GameGuard.

## The levers worth trying, in order

**1. `IPPORT` launch argument — no patching at all.**
`IPPORT` sits directly beside the server address table in `.text`, alongside
`GAMELAUNCHING`, `WEBSTART`, `STEAMSTART`, `autologin`. Strongly suggests the launcher
passes an explicit IP/port. If so we redirect to `127.0.0.1:8484` with a command line
and never touch a byte. **Test this first** — it is cheap and, if it works, sidesteps
Themida's anti-tamper entirely.

**2. Stub out `grap64.dll` — GameGuard's interface is tiny.**
`grap64.dll` exports exactly **two** functions:
```
__syscall_Common_Param8
__syscall_Common_Param16
```
It is a *static* import of `MapleStory.exe`, so the file must exist or the process will
not start — but a no-op replacement exporting those two symbols is a small, well-defined
piece of work. Open question: whether Themida verifies the module.

**3. `MapleSecurePC64.dll` is *not* a static import** — it is loaded dynamically. That
makes it easier to block or stub than GameGuard, and it is the packet-crypto layer we
most need out of the way.

**4. Network-layer redirect as a fallback.** The client targets hard-coded **IPs**, not
hostnames, so a `hosts` file will not help. `netsh interface portproxy` (or a local route)
can redirect those IPs to `127.0.0.1` without modifying the client at all.

## Notable clean-DLL findings

- **`ZLZ64.dll`** — exports only `ZLZCreateDeflator` / `ZLZCreateInflator` / `ZLZCloseFilter`.
  It is a **compression** filter (the `ZIStream` type), *not* the key store that the
  old `ZLZ.dll` was in v83-era clients. Likely used for packet and/or WZ compression;
  we will probably need to reimplement it for Stage 2.
- **`jypc.dll`** — 18 exports, all hash-named (`F0e9292417034e96934eadd7e6fedafd8`, …).
  Deliberately obfuscated naming, but the **module itself is unpacked and analysable**.
  Given its pairing with `MapleSecurePC64`, this is a prime target for understanding the
  packet security layer.
- **`WzMss.dll`** — SOAP web services (guild boards, notices, consult). Irrelevant locally;
  should be stubbed so the client does not stall on web calls.
- **`grap64.dll`** — 13 MB but only 2 exports, and unpacked. Best entry point for
  understanding how GameGuard is invoked.

## Consequences for the roadmap

Stage 1 must be **re-planned around not modifying `MapleStory.exe`**:

- Prefer configuration (`IPPORT`) and module substitution (`grap64.dll` stub, blocking
  `MapleSecurePC64.dll`) over binary patching.
- If in-process work proves necessary, favour **runtime injection** over on-disk patching,
  since Themida validates the image but is far weaker against a loaded module.
- Budget real time for **defeating GameGuard's kernel driver**, which no amount of
  user-mode cleverness avoids if the client insists the service is running.
