# The Mac client

The owner, 2026-10-02: *"not only do we need to release a windows client setup, we also need to
have a mac client for users running mac with all of the features from the launcher, the
gameguard stub, so that they can connect to our local server."*

**Every client release ships both:** `out\MapleCW-setup-windows.zip` for Windows and
`out\MapleCW-setup-mac.zip` for macOS, from one `tools\make-installer.ps1 -ClientOnly` run
(`CLAUDE.md`, "Standing constraints"); `tools\package-server.ps1`'s `MapleCW-server.zip` is the
third release file. **The Mac zip holds only `MapleCW.app`** (the owner's choice), so its
`README-mac.txt` is inside the app at `Contents/Resources/` - the install steps, the Gatekeeper
one above all, have to reach players with the zip:

```bash
xattr -dr com.apple.quarantine /Applications/MapleCW.app
```

**PLAYED ON A MAC, 2026-10-02** (Apple Silicon): launcher, sign-in over the LAN, Start Game, the
login screen with no "trouble logging in" dialog, character select and into the world - with the
bundled Wine 11.0 + DXVK-macOS, nothing of Nexon's. A player's whole setup is dragging the app
into Applications and ONE Terminal line (`xattr` + `codesign`, both part of macOS). Everything
below is kept for how it got there; a verdict down there may be overturned up here.

**Originally built 2026-10-02 before any Mac run.** Everything below the line "what is measured" is the
design and the static evidence it rests on. The first Mac launch is a measurement, and
[the checklist at the end](#the-first-mac-run-what-to-watch-for-and-what-each-outcome-means)
says what each outcome means.

## What Nexon's Mac client is, read from the copy in `client-mac/`

`client-mac/MapleStory Launcher.app` (gitignored; the owner's copy of what Mac players install)
is a zip-stored app bundle, 412 MB, 6 058 entries. What it is, from its own files:

* **A CrossOver OEM build.** `Contents/SharedSupport/maplestoryna/etc/maplestoryna.conf`:
  `ProductVersion 25.0.1.38870`, `BuildTag oem-maplestory-na-ngl-0.0.32`. Wine's PE DLLs for
  `x86_64-windows` and `i386-windows`, the `x86_64-unix` side (`ntdll.so`, `winemac.so`,
  `winevulkan.so`, ...), MoltenVK, GStreamer, and CrossOver's Perl tooling (`bin/wine` is the
  CrossOver wrapper, `lib/perl/CXBottle.pm` its bottle logic).
* **A managed bottle template**, `support/maplestory` (`support/default` is a symlink to it):
  `cxbottle.conf` says `WineArch win64`, `Template win10_64`, `Updater wineprefixcreate`. Its
  `drive_c` holds only the **Nexon Launcher** (`drive_c/Nexon/Launcher/nexon_launcher.exe`).
  **The game itself is not in the app**; the Nexon Launcher downloads it into the player's
  private bottle, `~/Library/Application Support/MapleStoryNA/Bottles/maplestory`.
* **The game is the Windows `MapleStory.exe`.** `bin/run_maplestory` runs
  `wine ... --bottle default --workdir "$MS_LAUNCH_DIR" "$MS_LAUNCH_APP"` with the directory,
  executable and arguments the Windows Nexon Launcher wrote into `C:\.ms-launch-args`.
  `drive_c/.mappings.ini` names `MapleStory.exe` as the app.
* **The Mac anti-cheat is native, beside Wine, not inside it.** `MapleStory Classic
  World.app` is a 130 KB helper (Developer ID: Nexon America) linking
  `@rpath/ngsx.framework` - NGS-X 1.5.0.3, whose `Resources/grap-core` is the Mac engine of
  the same GameGuard the Windows `grap64.dll` talks to. The helper's strings: it starts NGS-X
  (`initNgsxManager`, `OnInit/OnRun/OnDetect`, a `com.nexon.ngsx.daemon` it may ask for
  administrator rights to enable), exports `MS_GUID` and a socket `ms_<GUID>.sock`, then runs
  `bin/run_maplestory` with `CX_WINEWRAPPER_ALT_LOADER_SOCKET` so the Windows process is
  hosted by the helper. `ngsxKillBottlesCallback` is how a detection ends the game.

So **a Mac player's client is the same Windows client** running in Wine, and GameGuard on a
Mac is a native process that the Windows-side `grap64.dll` reaches through the helper. Our stub
replaces that `grap64.dll`, so nothing ever asks for NGS-X - the same position as on Windows,
where the stub means `NGService.exe` and `BlackCat64.sys` never start.

## What the MapleCW Mac client is

**Not a port. The Windows launcher, the Windows stub and the Windows client, in Wine.** One
code path and one set of patches, which is the whole reason this is cheap enough to ship.
**Self-contained since 2026-10-02**: the owner, *"make it so that the MapleCW.app doesn't need the
original MapleStory Launcher.app"* - with an open-source Wine, always used (their choice, over
copying Nexon's CrossOver runtime, which would redistribute CodeWeavers' proprietary build).

```text
MapleCW.app/Contents/MacOS/MapleCW          tools/mac/MapleCW, a bash script
MapleCW.app/Contents/Resources/MapleCW.icns the game's Orange Mushroom icon, copied at package
                                           time from client-mac/ (never committed)
MapleCW.app/Contents/Resources/wine/        Gcenx's macOS build of WineHQ stable 11.0, unmodified,
                                           minus Wine Mono; README-WINE.txt is its licence notice
MapleCW.app/Contents/Resources/payload/     the -ClientOnly payload, byte for byte:
    maplecw-launcher.exe                       the static-CRT launcher every package ships
    grap64.dll                                 the stub
    client/                                    MapleStory.exe and Data\
    .payload-stamp                             digests of those three
```

**The Wine build is downloaded once onto the packaging machine and never committed:**
`wine-stable-11.0_1-osx64.tar.xz` from `github.com/Gcenx/macOS_Wine_builds` release `11.0_1`,
185 303 032 bytes, SHA-256 `b50dc50e...82388`, into `out\wine\`. `tools/make_mac_client.py` pins
the digest and refuses any other file. It streams the archive into the app keeping executable
bits and the 67 relative symlinks, leaves out `share/wine/mono` (236 MB of .NET neither program
uses), and checks the result. Why stable and not the weekly devel builds: the runtime should not
move under players between releases; 11.0 is also the Wine CrossOver 26 is built on.

What the script does at every start:

1. **Install the payload** into `~/Library/Application Support/MapleCW/MapleCW` when the app's
   `.payload-stamp` differs from the installed one (a first run, or a newer zip). The launcher
   writes beside itself - markers, logs, `maplecw-launcher.remembered.toml`, its own
   self-update - and an app bundle is the wrong place for that: an unsigned app opened from
   Downloads runs from a read-only translocated copy. `maplecw-launcher.remembered.toml` and
   `maplecw-launcher.toml` survive a reinstall. Between releases the launcher's self-update and
   the client patch keep everything current, exactly as on Windows.
2. **Use the bundled Wine**, `Contents/Resources/wine/bin/wine` - a self-locating loader that
   links only `libSystem`, run in place from the bundle. Prefix `~/Library/Application
   Support/MapleCW/prefix`, 64-bit. On the first start the script runs `wineboot --init` and
   writes the one Direct3D setting Nexon's bottle carries, `csmt = 0` (CSMT disabled; its other
   key, `cb_access_map_w`, is CrossOver-only). Always exported: `WINEDLLOVERRIDES=mscoree=`
   (without Mono, Wine would otherwise offer to download it) and `WINEDEBUG=fixme-all`.
3. **Start the launcher**: `wine <install>/maplecw-launcher.exe`, from `<install>`.

Test-only overrides, all environment variables (`launchctl setenv` for a Finder launch):
`MAPLECW_D3D_RENDERER=gl|vulkan` (wined3d's backend; the build carries MoltenVK, so `vulkan` is
the second route if the default GL one draws wrongly), `MAPLECW_WINE` (another Wine binary), and
`MAPLECW_USE_NEXON=1` - **Nexon's CrossOver, the route the first Mac runs used**, kept so a
rendering problem can be compared against the runtime Nexon tuned. That route clones Nexon's
bottle template with `CX_BOTTLE_PATH` pointed at MapleCW's folder (`CXBottle::setup_bottle_wineprefix`
finds no private bottle there, falls back to the managed template, and CrossOver makes a private
copy with `wineprefixcreate --ref-dir`), and deliberately omits `--enable-alt-loader`, which
would hand the process to the helper that starts NGS-X.

From there it is the Windows flow: sign in, Start Game, the stub, the markers, `-NXLDEBUG <ip>
<port>`, the hook. `crate::wine` (launcher) and `hook::wine_version` (stub) detect Wine by
`ntdll!wine_get_version`, and both put a line in their log saying so.

### Measured on the owner's Mac, 2026-10-02, with the bundled Wine

* **The launcher window opens** (the glutin patch holds under plain Wine too).
* **Local Network:** unsigned, the app was silently refused the LAN (`OS Error 10065`, no prompt -
  CrossOver's Developer-ID-signed Wine had prompted). `codesign --force --deep --sign -
  /Applications/MapleCW.app` on the Mac fixed it: sign-in and the server probe went through.
  **Not yet built into the packaging** - a signature cannot be made on the Windows packaging box
  without a new tool, so for now the player runs that one command (README-mac.txt).
* **The stub, the hook and every patch armed** under Wine 11 (`install_once: running under Wine
  11.0`, identity, guard pages). Then the client died ~1 s in with no window.
* **Why: Direct3D 11 feature level 11_0.** The hook log's last throw is in `FUN_142c4d020`, the
  `Gr2D_DX11` device-NULL abort (`research/msexe-client-opcodes.md`). With `warn+d3d11`: both
  wined3d renderers fail `D3D11CreateDevice` at "Failed to create command stream" (OpenGL caps at
  4.1; MoltenVK has no geometry shaders), CSMT on or off. `Gr2D_DX11.dll` asks for exactly one
  level, `0xb000` (11_0), and all 203 of its shaders are SM 5.0 (141 ps, 59 vs, 3 cs) - so a lower
  level is not an option. [L] for the request and the shaders; [I] that the throw is that abort.
* **So the app now carries DXVK-macOS** (Gcenx, DXVK 1.10.3, D3D11 over Vulkan/MoltenVK, 11_0
  without geometry shaders), installed into the prefix's system32 and enabled for `MapleStory.exe`
  only via `AppDefaults\MapleStory.exe\DllOverrides`. Pinned by SHA-256 in
  `tools/make_mac_client.py`; `MAPLECW_NO_DXVK=1` reverts. **Unmeasured - the next Mac run.**

### What the switch to plain Wine costs

* **Nexon's runtime was proven with this client; this one is not.** CrossOver carries CodeWeavers'
  macOS Direct3D work, and the client draws through Direct3D 11 (`Gr2D_DX11.dll`). Whether the
  game renders under upstream wined3d on macOS's OpenGL 4.1 is the first new question; the
  launcher window itself is egui over GL and already opened under CrossOver.
* **The zip grows** by the Wine build: ~600 MB unpacked, about 200 MB compressed.

### What differs from Windows, on purpose

| | Windows | Mac |
|---|---|---|
| outbound firewall rule | `netsh advfirewall`, scoped to `MapleStory.exe` | **none** - Wine has no Windows Firewall and its `netsh` would accept the command and block nothing, so the launcher logs `firewall rule NOT applied ... This client CAN reach the internet` instead of a false success (`wine::firewall_note`) |
| elevation | UAC on `MapleStory.exe` and the launcher | none; Wine ignores `requireAdministrator` |
| the Nexon gate byte, the stub, `grap\` -> `grap.disabled` | at Start Game | the same, in the installed copy |
| `previous-runs`-style evidence | beside the client | the same files, in `~/Library/Application Support/MapleCW/MapleCW/client`, plus `mac-launch.log` (the script and Wine) |

**The firewall gap is a real loss of hygiene, not of stability.** The reachability check that
crashed a firewalled client is answered by the hook's `1415db360:ret`, not by the firewall. A
per-application block on macOS needs administrator rights and a `pf` anchor; that is a separate
decision from shipping a client and is not made here. An in-process block (the hook already
hooks `connect` in `netwatch`) would work on both platforms and is the better candidate if it
is wanted.

## What is measured, and what is not

**Measured, 2026-10-02, on the Windows dev box:**

* `tools/make_mac_client.py --self-test`: the zip marks `Contents/MacOS/MapleCW` as Unix 0755
  with LF line endings, every directory 0755, the plist parses, a server payload is refused.
* `tools/mac/MapleCW` under Git Bash against a fake app and a fake CrossOver tree: first
  install, an unchanged second start (no copy), a new stamp (re-copy, the remembered toml
  survives), the exact wrapper command line and `CX_BOTTLE_PATH`, and the no-Wine failure.
* `crate::wine::detect()` returns `None` on Windows (the negative control - a `Some` there would
  skip the firewall on every Windows client).
* `make-installer.ps1 -ClientOnly -NoClient -SkipBuild -NoZip` builds both payloads.

**The first Mac launch, 2026-10-02 (the owner's test machine):** the app started, found Nexon's
runtime, built its bottle and ran `maplecw-launcher.exe` - unmeasured items 1 and 2 below came back
**fine** - and the launcher's own panic dialog then said *"The launcher window could not be
created. glutin error: extension to create ES context with wgl is not present"*. That was item 3:
Wine's `winemac.drv` refuses any 3.2+ GL context that is not forward-compatible (the string is in
the shipped `winemac.so`), glutin asks for core 3.3 without the flag, and eframe's ES fallback does
not exist under Wine. Fixed by a one-line, Wine-gated patch to a vendored glutin -
`vendor/README.md`. The window opening is the next launch's first check.

**The second launch, same day:** the patched window **opened**. Start Game then self-updated the
launcher to the one the live server publishes - built at 00:35, before the patch - and that one hit
the same dialog. So the fix holds, and it exposed a rule:

> **The server must publish a Mac-capable launcher before a Mac zip is handed out.** Self-update
> replaces the Mac launcher with the server's at Start Game, unconditionally. A server older than
> the zip downgrades the player to a launcher whose window cannot open - and a launcher with no
> window can never update itself again. `tools\package-server.ps1` and the setup zips must come
> from the same build (the launcher digest in both build logs must match), and the server goes
> out first.

**Guarded since the same evening:** under Wine the launcher now refuses a self-update whose bytes
lack `selfupdate::MAC_CAPABLE` (a marker every launcher built with the guard carries, because it
searches for it) - it logs `NOT installing the server's launcher ...` and starts the game with the
launcher that works. The rule above is still the right order; breaking it now costs a warning,
not the player's launcher. Only launchers built with the guard enforce it.

**A note on the third report, for whoever reads these logs next:** `mac-launch.log` said
`installing the payload ... (stamp a55fb605f839ff87)` - the zip built **before** the glutin patch
(the patched one was `8e34fe62...`). The fix had not been tested at all; the wrong zip had. The
app now logs `launcher: sha256 <16 hex>` before every start, to compare with the build log.

Recovery on a Mac that was downgraded: quit, delete
`~/Library/Application Support/MapleCW/MapleCW/.payload-stamp`, open `MapleCW.app` again. The app
reinstalls its bundled launcher; the remembered server and account survive.

**Not measured - none of this has run on a Mac (items 1 and 2 since have, see above):**

1. **macOS's own bash 3.2 and BSD tools.** The script avoids bash 4 features, but it has only
   been run under Git Bash.
2. **CrossOver creating our bottle from the template** with `CX_BOTTLE_PATH` pointed elsewhere.
   Read from `CXBottle.pm`, not run.
3. **The launcher window under Wine.** `eframe` with the `glow` (OpenGL) backend through
   `winemac.so`. CrossOver supports OpenGL core contexts; an egui window there is unobserved.
4. **The hook's single-step on Apple Silicon.** `141b2a280:rdx=0` - the patch that keeps the
   "trouble logging in" dialog away - is an `int3` plus a **trap-flag single-step** to re-arm.
   Wine delivers `EXCEPTION_SINGLE_STEP` on an Intel Mac; whether **Rosetta 2** honours `TF`
   set from a signal context is the largest open question. `1415db360:ret` needs no
   single-step and is not at risk.
5. **The heap mitigations under Wine's heap.** `guardpage`, the pool sentry's repair and
   `freeguard` all work on the client's own pool allocator, which is the same code under Wine
   - but the death they prevent is Windows' `RtlFreeHeap` killing the process on a pointer it
   never issued, and **Wine's heap may not die there at all.** Measured nowhere yet; the kill
   switches are the same files as on Windows.
6. **Rosetta and 4 KB pages.** `guardpage` decommits one 4 KB page per watched allocation.
   Rosetta presents 4 KB pages to x86-64 processes on 16 KB hardware; the cost is unmeasured.

## The first Mac run: what to watch for, and what each outcome means

One variant at a time, defaults first. Each step is a claim that can come back false.

1. **Open `MapleCW.app`.** Within ~2 minutes (first start) the MapleCW launcher window appears.
   - No window, a dialog "MapleCW could not start" -> its text and `mac-launch.log` say which
     step; unmeasured item 1 or 2.
   - No window and no dialog -> the app did not start at all: quarantine (README step 2) or
     the script's mode bit. `ls -l /Applications/MapleCW.app/Contents/MacOS/MapleCW` should
     show `-rwxr-xr-x`.
   - `mac-launch.log` ends in Wine errors about OpenGL/WGL -> item 3; the launcher's window is
     the problem, not the game.
2. **Sign in, Start Game.** The launcher's log pane starts with the build line and then
   **`running under Wine <ver> on Darwin - the Mac client`**. Absent -> the log is not from the
   Mac client, or `wine::detect` is wrong; stop and read nothing else from it. It also has
   **`firewall rule NOT applied`** - expected, see above.
3. **The client's login screen.** The Login button enables and no "trouble logging in" dialog
   appears.
   - The dialog appears -> `maplecw-hook.log`: count the `141b2a280` watch hits. **One hit then
     silence -> the single-step re-arm failed (item 4)**; the fix is a patch that needs no
     re-arm, not a Wine setting. Several hits -> the patch works and the dialog is something
     else.
   - `maplecw-hook.log` has `install_once: running under Wine` -> the stub loaded and knows
     where it is. No hook log at all -> the stub did not load; check `client/grap64.dll` is
     ours (the launcher log's "stub" line).
4. **Character select, enter the world, play 10 minutes.** Compare with Windows: the client on
   Windows has a known 180-second pool corruption that the shipped sentry repairs. A death
   under Wine -> the hook log's last lines (its `CLIENT FAULT` line names the exception and
   address) and the end of `mac-launch.log`; whether Wine's heap dies the same way (item 5) is
   exactly what those answer. There is no `client-exit.log` on a Mac - that is a Windows
   watcher.
5. **Only if 1-4 pass:** turn `guardpage` off for one run (`maplecw-hook.guardpage.off` in
   `client/`) and compare frame smoothness - item 6.
