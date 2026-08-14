# Client reconnaissance — `mscw`

Static analysis of `C:\Nexon\Library\maplestorycw\appdata`, 2026-08-14.
Nothing in the original install was modified.

## Identity

| | |
|---|---|
| PDB path | `c:\build\src\mscw_game\Bin64\MapleStory.pdb` |
| Codename | **mscw** |
| Architecture | x86-64 |
| `MapleStory.exe` | 76,702,712 bytes |
| Patch manifest build | `1786489625` ≈ **Jan 2026** |
| Manifest | `patchdata/59822.manifest.hash` → `321d2f36…` (zlib JSON, 520 entries, UTF-16 base64 paths) |
| Language | English (verified: `String.wz` mob names read "Blue Snail", "Orange Mushroom") |
| **WZ data version** | **779** (hash `0x0000E73A`) |

## Architecture: classic Wvs, modernized

The binary retains the canonical MapleStory C++ class names, so the traditional
server topology still applies:

- `COutPacket`, `CInPacket` — packet framing
- `CWvsContext` — client-side game context
- `CLoginQueueDlg` — login queue

Version-check strings (the login server must satisfy these):
```
First Connect : nClientVersion_Temp : %d
Version Check / Version OK
OK. Allowed Version. %d ( %d~%d )
Low Version. Launch Patch %d ( %d~%d )
High Version. Error. %d ( %d~%d )
Error: Client Version is Higher : %d > %d
```
Note the accepted-range form (`%d ( %d~%d )`) — the client tolerates a *span* of
versions, which may give us slack during Stage 2.

## Network endpoints found in the binary

Leftover Nexon dev/live tables — useful as format references, not as targets.

| Role | Address | Port |
|---|---|---|
| Login | `10.9.2.131`, `.132`, `.133` (tag `LIVE`) | **8484** |
| Login (KR) | `175.207.3.196`, `.238`, `.239` | |
| Channels | `192.168.128.75` … `.84` | **5160** |
| AWS | `54.180.211.235`, `43.200.157.16` | 24200 |
| Other | `44.234.163.43`, `44.234.166.161`, `44.234.167.163` | |

Also present: `D:\DevPatch\z_BinaryOn_{Login,Middle,Etc}_%s.bat` — Nexon's internal
deploy scripts, confirming the Login/Middle/Etc server split.

## Launch flags

The client is normally started by the Nexon launcher, not directly:

```
GAMELAUNCHING  WEBSTART  STEAMSTART  IPPORT  autologin  skiplogo
-NXL  -NXLDEBUG  -NXLPTS
```
Debug/QA flags also present: `noquest`, `debugwnd`, `showcode`, `fastskillui`,
`skilldemo`, `liedetector`, `mtload`, `rprd`, `recorduol`.

**`IPPORT` is the most promising lever** — it appears adjacent to the server address
table and likely lets the launcher hand the client an explicit IP/port, which would let
us point at `127.0.0.1` without patching the host table. Verify in Stage 1.

Auth backends: `CNMLoginNexonPassportFunc`, `CNMGetNexonPassportFunc`, `steam_api64.dll`.

## Protection surface

| Component | Files | Notes |
|---|---|---|
| **nProtect GameGuard** | `grap\BlackCat64.sys` (4.0 MB kernel driver), `grap\NGService.exe`, `grap-core64.aes` (40 MB), `grap-communicator64.aes` (38 MB), `grap-updater.aes` (20 MB), `grap64.dll` (13 MB) | Kernel anti-cheat. Blocks debuggers and packet capture. Installed as a service via `NGService_Install.bat`. |
| **Nexon packet security** | `MapleSecurePC64.dll` (2.7 MB), `jypc.dll` | Packet encryption / anti-tamper. Blocks plaintext classic-MapleStory crypto. |
| Overlay | `NxOverlay\` (CEF 153 MB `libcef.dll`, `DwarfAxe.exe`) | In-game browser; likely disableable. |
| Analytics | `NexonAnalytics64.dll` (6.2 MB) | Telemetry; should be stubbed for a local server. |
| Misc | `ZLZ64.dll`, `WzMss.dll`, `PCOM.dll`, `ResMan.dll`, `NameSpace.dll`, `Canvas.dll`, `Gr2D_DX11.dll`, `Shape2D.dll`, `Sound.dll` | Engine/support. `ZLZ` historically holds WZ/crypto key material — worth a look. |
| TLS | `libssl-3-x64.dll`, `libcrypto-3-x64.dll`, `libcurl.dll` | OpenSSL 3 — web/auth calls. |

Web endpoints: `public.api.nexon.com`, `private.api.nexon.com`,
`maplestory.nexon.com`, `maplestory.nexon.net`, `www.nexon.com/account/en/reset-password`.

## Implications for the server

1. **WZ data is solved** — see `docs/wz-format.md`. 100% of 9,994 images parse.
2. **The hard problem is the protection layer**, not the data. GameGuard is kernel-level
   and `MapleSecurePC64` wraps the socket layer.
3. **No public reference exists for this build.** v83-era emulators (HeavenMS, Cosmic)
   share the *architecture* but none of the opcodes or crypto. Opcode discovery must be
   empirical, from a client we control.
4. Next lever to pull: confirm whether `IPPORT` + a GameGuard-free launch path is enough
   to get the client talking to `127.0.0.1:8484`.
