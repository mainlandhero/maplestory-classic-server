<#
.SYNOPSIS
    Build a self-contained MapleCW payload for a machine that has nothing on it.

.DESCRIPTION
    Produces a directory (and optionally a .zip) carrying everything a bare test machine
    needs: the servers, the launcher, the GameGuard stub, the client with its WZ data, the
    generated game tables and the authored server data.

    Run from an elevated PowerShell 5.1 window on the development box:

      powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\make-installer.ps1"

    The result is roughly 900 MB staged. Most of it is `client\Data` - Sound is 159 MB and
    Map is 128 MB - and both are already-compressed formats, so do not expect the zip to be
    much smaller than the folder.

    TWO SHAPES. The default carries BOTH halves - it is what a bare machine that runs the
    servers AND plays needs, which is the case this script was written for. `-ClientOnly`
    builds a PLAYER's payload instead: the launcher, the stub and the client, and none of the
    server executables, the gm-handbook or the authored data. The owner, 2026-09-07: "since the
    clients themselves are not GMs, I don't think they need the gm-handbook or any of the
    maplecw-auth, login, useradd, or world exes." Nothing in `crates/launcher` references
    gm-handbook or data\ at all, so that is a removal rather than a gamble. install.ps1
    detects which shape it was given and skips account creation for a client, because
    `maplecw-useradd` writes to a LOCAL database and a player's accounts live on the server.

    WHAT GOES IN THE PAYLOAD, AND WHAT DOES NOT

      maplecw-launcher.exe    at the ROOT, and that is not cosmetic - see below
      grap64.dll              the GameGuard stub, also at the root
      bin\            maplecw-login, maplecw-world, maplecw-chat (the hub), maplecw-auth, maplecw-useradd
      client\         MapleStory.exe, its DLLs and Data\ - the WZ archives
      gm-handbook\    game tables generated from the WZ; the world server refuses to do
                      most of its job without them, and they are gitignored, so they have
                      to be staged from the dev box rather than regenerated on the target
      data\           authored server data - shops, drops, quest scripts, the EXP curve
      install.ps1     what the target machine runs
      start-server.ps1

    Deliberately excluded: `maplecw.db` (it holds argon2id password hashes and characters -
    real state, and the target creates its own), every `maplecw-hook.*` marker and log,
    `previous-runs\`, crash dumps, and the screenshots the client leaves beside itself.

    WHY THE LAUNCHER SITS AT THE ROOT AND THE SERVERS DO NOT

    The launcher finds its own way around: with no configuration at all it looks for
    `client\`, `maplecw.db` and `grap64.dll` **beside its own executable**, and only then
    falls back to a config file or to walking up for a dev checkout. Putting it in `bin\`
    with `client\` as a sibling would break that detection and leave the whole install
    depending on `maplecw-launcher.toml` being written correctly - a single point of failure
    on a machine nobody can debug remotely. At the root it finds everything by itself and the
    config file becomes what it should be: an override for the things that cannot be
    inferred, like which server to reach.

    The servers stay in `bin\` because nothing looks for them; `start-server.ps1` names them.

    THE CLIENT IS SHIPPED UNSTUBBED, ON PURPOSE

    `client-patched\` on this machine has GameGuard already neutralised: `grap64.dll` is our
    stub, the real one is `grap64.dll.orig`, and `grap\` has been renamed `grap.disabled`.
    The payload undoes that naming, so the installed client starts in the state Nexon
    shipped and the launcher's Start Game button does the stubbing itself. That keeps one
    code path for neutralising GameGuard instead of two, and it means a payload can be
    diffed against a fresh install.

    THE CLIENT AND ITS WZ DATA ARE NEXON'S

    This packages files the owner already owns so their own test machines can run them. It is not
    a redistribution channel - keep the payload on machines you own.

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\make-installer.ps1" -NoClient
    Stage everything except the 900 MB of client, to check the script itself in seconds.
#>
[CmdletBinding()]
param(
    [string]$OutDir,
    [switch]$SkipBuild,
    [switch]$NoZip,
    [switch]$NoClient,
    # A payload for a PLAYER's machine: the launcher, the stub and the client, and nothing
    # else. No server executables, no gm-handbook, no authored server data - see the comment
    # on $binaries. install.ps1 detects the difference and skips account creation, which is a
    # server-side step. Roughly 470 MB rather than 900 MB, almost all of it the client's WZ.
    [switch]$ClientOnly
)

$ErrorActionPreference = 'Stop'

$here = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
$repo = Split-Path -Parent $here
if (-not $OutDir) { $OutDir = Join-Path $repo 'out' }

$clientSrc = Join-Path $repo 'client-patched'
$stage     = Join-Path $OutDir 'MapleCW'
$zipPath   = Join-Path $OutDir 'MapleCW-setup.zip'

function Fail([string]$msg) { throw $msg }


# **The identity every one of these binaries prints at its own startup.**
#
# store::buildstamp puts `sha256 <16 hex>` in the first line each server logs, so a deployment
# can be checked against the artifact it came from by comparing two short strings - no build,
# no version number, no trusting a file date that unzipping may have rewritten. The owner,
# 2026-09-13, after fixes turned out not to be running on the deployed server: "as part of
# startup, all of the processes should include a build time from now on."
#
# Lowercase and truncated to 16 to MATCH what the process prints. Get-FileHash returns
# uppercase; a comparison that requires the reader to notice that is not a comparison.
function Show-BuildIdentity([string]$dir, [string[]]$names) {
    Write-Host 'build identity - each of these prints the same digest in its own first log line:' -ForegroundColor Cyan
    foreach ($n in $names) {
        $p = Join-Path $dir $n
        if (-not (Test-Path $p)) { continue }
        $f = Get-Item $p
        $d = (Get-FileHash $p -Algorithm SHA256).Hash.ToLower().Substring(0, 16)
        Write-Host ("   {0,-24} {1,10:N0} bytes  {2:yyyy-MM-dd HH:mm:ss}  sha256 {3}" -f `
            $n, $f.Length, $f.LastWriteTimeUtc, $d)
    }
    Write-Host '   (times are UTC, the same zone the startup line uses)'
}

# ---------------------------------------------------------------- preflight
# Every one of these is something whose absence produces a payload that installs fine and
# then does not work, which is the expensive kind of failure: it is found on the target
# machine, by hand, rather than here in one second.

if (-not $NoClient) {
    if (-not (Test-Path $clientSrc)) { Fail "no client at $clientSrc" }
    if (-not (Test-Path (Join-Path $clientSrc 'MapleStory.exe'))) {
        Fail "$clientSrc has no MapleStory.exe"
    }
    if (-not (Test-Path (Join-Path $clientSrc 'Data'))) {
        Fail "$clientSrc has no Data\ - the WZ archives are the point of the payload"
    }
    # **The Data\ that ships is the HYBRID one** - the classic archives with the Signature
    # Style Collection merged in (tools\backport_install.py: the 206 items, the box under its
    # classic id, the weapon-cover links, Himmel's ItemEff, the hair-hats' slot type, the face
    # coupons' family). robocopy takes whatever sits there, and nothing else proves that what
    # sits there is the CURRENT build of that script rather than an earlier install. So the
    # script rebuilds into a scratch directory and requires every installed archive to hash
    # equal to the fresh build, and gm-handbook to have been regenerated after the install.
    # The owner, 2026-09-12: "Make sure everything we worked on is release-able to the server and
    # client packages." Two minutes; it is the one check that catches a stale client payload.
    Write-Host 'release check: the installed hybrid archives against a fresh build...' -ForegroundColor Cyan
    & python (Join-Path $here 'backport_install.py') --check
    if ($LASTEXITCODE -ne 0) {
        Fail @"
the installed Data\ is not the current backport build (see the lines above). With the client
closed:
  python "$repo\tools\backport_install.py" --install
"@
    }
    # And the quest archive: tools\quest_patch.py lifts the Maple Island quests' Beginner-only
    # rule, which the client enforces from its own data (the owner, 2026-09-25).
    & python (Join-Path $here 'quest_patch.py') --check
    if ($LASTEXITCODE -ne 0) {
        Fail @"
the installed Quest\QuestData archive is not the current quest patch. With the client closed:
  python "$repo\tools\quest_patch.py" --install
"@
    }
}

# **The handbook and data\ are the WORLD SERVER's, so a -ClientOnly payload needs neither**
# and must not fail for their absence. Nothing in crates/launcher reads either one.
if (-not $ClientOnly) {
$handbook = Join-Path $repo 'gm-handbook'
if (-not (Test-Path $handbook)) {
    Fail @"
no gm-handbook\ - the world server reads its game tables from there and it is gitignored.
Regenerate on this machine first:
  python "$repo\tools\dump_names.py"
  python "$repo\tools\dump_portals.py"
  python "$repo\tools\dump_itemdata.py"
"@
}
foreach ($needed in @('maps.txt', 'mobtemplates.txt', 'skills.txt', 'footholds.txt', 'reactors.txt')) {
    if (-not (Test-Path (Join-Path $handbook $needed))) {
        Fail "gm-handbook\$needed is missing - regenerate the handbook before packaging"
    }
}

$dataDir = Join-Path $repo 'data'
if (-not (Test-Path $dataDir)) { Fail "no data\ - shops, drops and quest scripts live there" }
}

# ---------------------------------------------------------------- build
# A running server holds its own executable open, and cargo cannot replace a file Windows has
# locked. The failure it produces is "Access is denied. (os error 5)" against a path in
# target\release, which says nothing about servers and reads like a permissions problem. Worse
# is the -SkipBuild case: that succeeds and quietly packages whatever build was there when the
# server started, so a payload can ship a login server months older than the code.
$running = Get-Process -Name 'maplecw-login', 'maplecw-world', 'maplecw-launcher' `
    -ErrorAction SilentlyContinue
if ($running) {
    $names = ($running | ForEach-Object { "$($_.ProcessName) (pid $($_.Id))" }) -join ', '
    $stopLine = "  powershell -ExecutionPolicy Bypass -File `"$repo\tools\test-server.ps1`" -Stop"
    if ($NoClient) {
        # -NoClient is the documented dry run - "check the script itself in seconds" - and its
        # output is explicitly not a shippable payload, so staleness cannot reach a machine.
        # Warn rather than fail, otherwise the one mode meant for testing this script is the
        # one mode that cannot run while a server happens to be up.
        Write-Host "WARNING: servers are running: $names" -ForegroundColor Yellow
        Write-Host '  -NoClient, so this is a dry run and the binaries may be stale.' -ForegroundColor Yellow
        Write-Host '  Do NOT ship this staging directory.' -ForegroundColor Yellow
        Write-Host $stopLine -ForegroundColor Yellow
    } else {
        Fail @"
these are running and hold their own executables open: $names

Stop them before packaging, or the payload ships a stale build:
$stopLine
"@
    }
}

$rel = Join-Path $repo 'target\release'
# Where a -ClientOnly launcher is built. A SEPARATE target dir, the same reasoning
# tools\package-server.ps1 gives: RUSTFLAGS is part of cargo's fingerprint, so sharing
# target\ would make every switch a full rebuild AND would leave target\release holding a
# statically linked launcher that the dev scripts then run without anyone noticing.
$staticTarget = Join-Path $repo 'target-static'

if (-not $SkipBuild) {
    Write-Host 'building release binaries...' -ForegroundColor Cyan
    Push-Location $repo
    try {
        & cargo build --release -p login -p world -p auth -p grap-stub -p launcher
        if ($LASTEXITCODE -ne 0) { Fail 'cargo build failed' }

        # **The launcher, again, with a STATIC C runtime - for EVERY package since 2026-09-16.**
        #
        # It used to be the -ClientOnly payload's alone: that payload ships no install.ps1,
        # so nothing checks for the Visual C++ redistributable before the launcher runs -
        # the launcher checks it itself (crates\launcher\src\stub.rs), which is only
        # reachable if the launcher does not need the redistributable to START.
        #
        # Now the launcher UPDATES ITSELF from the server (launcher::selfupdate), and the
        # server publishes the static-CRT copy tools\package-server.ps1 builds into this same
        # target-static tree. Every launcher a player holds must therefore BE that build, or
        # the first Start Game swaps it for the static one - harmless, but a package whose
        # launcher replaces itself on first run is a package whose build identity lies. One
        # launcher build, one digest: the setup zip, the client payload and the server's
        # bin\ alike.
        {
            #
            # grap64.dll is deliberately NOT built this way. It is injected into
            # MapleStory.exe, and tools\package-server.ps1's header is explicit that changing
            # a working hook's CRT linkage is an unforced change that costs a manual client
            # launch to discover. So the stub still needs the redistributable, and the check
            # above is what tells a player so in words instead of a missing-DLL dialog.
            Write-Host '  and the launcher again with a STATIC C runtime, so it starts on a' -ForegroundColor Cyan
            Write-Host '  machine with no Visual C++ redistributable and can say so' -ForegroundColor Cyan
            $env:RUSTFLAGS = '-C target-feature=+crt-static'
            $env:CARGO_TARGET_DIR = $staticTarget
            & cargo build --release -p launcher
            $ok = $LASTEXITCODE -eq 0
            Remove-Item Env:\RUSTFLAGS -ErrorAction SilentlyContinue
            Remove-Item Env:\CARGO_TARGET_DIR -ErrorAction SilentlyContinue
            if (-not $ok) { Fail 'the static-runtime launcher build failed' }
        }
    }
    finally {
        Pop-Location
        Remove-Item Env:\RUSTFLAGS -ErrorAction SilentlyContinue
        Remove-Item Env:\CARGO_TARGET_DIR -ErrorAction SilentlyContinue
    }
}

# The static launcher replaces the ordinary one in EVERY package, and ONLY that one file.
#
# **Not by repointing $rel**, which is what the first version of this did: only the LAUNCHER is
# built in the static tree, so $rel pointing there made the very next step fail looking for
# grap64.dll - which is deliberately still built the ordinary way and lives in target\release.
# One override for one file. Always, since 2026-09-16 - see the build step above.
$staticExe = Join-Path $staticTarget 'release\maplecw-launcher.exe'
if (-not (Test-Path $staticExe)) { Fail "$staticExe is missing - build without -SkipBuild" }
if ($ClientOnly) {
    if (Test-Path $staticExe) {
        Write-Host "  the launcher will come from $staticExe"
    } elseif (-not $SkipBuild) {
        Fail "the static-runtime launcher is missing at $staticExe"
    } else {
        Write-Host '  NO static-runtime launcher found (-SkipBuild) - shipping the ordinary' -ForegroundColor Yellow
        Write-Host '  one, which needs the Visual C++ redistributable to START and therefore' -ForegroundColor Yellow
        Write-Host '  cannot tell a player that it is missing.' -ForegroundColor Yellow
        $staticExe = $null
    }
}
# `To` is the payload-relative directory. The launcher and the stub go to the root because
# that is where the launcher's own path resolution expects them; see the header.
$binaries = @(
    @{ From = 'maplecw-launcher.exe'; To = '.';    Why = 'the launcher the owner signs in with' },
    @{ From = 'grap64.dll';           To = '.';    Why = 'the GameGuard stub the launcher installs' },
    @{ From = 'maplecw-login.exe';    To = 'bin';  Why = 'the login / character-select server' },
    @{ From = 'maplecw-world.exe';    To = 'bin';  Why = 'a channel server' },
    # The hub joined the server on 2026-09-15 and start-server.ps1 has dialed it since, but
    # this list never learned of it - a setup-based install started WITHOUT it (the script
    # tests for the file and carries on), so cross-channel parties, party chat and whispers
    # were silently absent there. Found 2026-09-18 by hashing the two zips against each other.
    @{ From = 'maplecw-chat.exe';     To = 'bin';  Why = 'the world hub the channels dial for parties, party chat and whispers across channels' },
    @{ From = 'maplecw-useradd.exe';  To = 'bin';  Why = 'creates the first account' },
    @{ From = 'maplecw-auth.exe';     To = 'bin';  Why = 'the sign-in service the launcher posts to' }
)
# **-ClientOnly drops everything a player has no use for.** The owner, 2026-09-07: "since the
# clients themselves are not GMs, I don't think they need the gm-handbook or any of the
# maplecw-auth, login, useradd, or world exes."
#
# They are right, and the launcher agrees: nothing in crates/launcher references gm-handbook or
# data\ at all - it needs client\, grap64.dll and a server to reach. What goes with them is
# account creation, because that runs maplecw-useradd against a LOCAL maplecw.db, and on a
# client machine there is no database: accounts live on the server.
#
# The full payload stays the default. It is what a bare machine that runs BOTH halves needs,
# which is the case this script was written for and the one the owner's own test box is.
if ($ClientOnly) {
    $binaries = $binaries | Where-Object { $_.To -eq '.' }
}
foreach ($b in $binaries) {
    if (-not (Test-Path (Join-Path $rel $b.From))) {
        Fail ("target\release\{0} is missing ({1}). Build without -SkipBuild." -f $b.From, $b.Why)
    }
}

# ---------------------------------------------------------------- stage
if (Test-Path $stage) {
    Write-Host "clearing $stage"
    Remove-Item -Recurse -Force $stage
}
New-Item -ItemType Directory -Path $stage -Force | Out-Null
New-Item -ItemType Directory -Path (Join-Path $stage 'bin') -Force | Out-Null

foreach ($b in $binaries) {
    $dst = if ($b.To -eq '.') { $stage } else { Join-Path $stage $b.To }
    New-Item -ItemType Directory -Path $dst -Force | Out-Null
    # The one override: the launcher comes from the static-CRT tree - the same file the server
    # package publishes for self-update - so it starts on a machine with no Visual C++
    # redistributable and matches the server byte for byte. Everything else, grap64.dll
    # included, comes from the ordinary build.
    $src = if ($staticExe -and $b.From -eq 'maplecw-launcher.exe') {
        $staticExe
    } else {
        Join-Path $rel $b.From
    }
    Copy-Item $src $dst -Force
}
Write-Host ("staged {0} binaries" -f $binaries.Count)
# The launcher is the one binary a PLAYER runs, and its log is what they paste back when
# something goes wrong - so its digest is the way to tell whether they are running the
# build that carries a fix. It prints the same string in its own first log line.
Show-BuildIdentity $stage @('maplecw-launcher.exe')

if ($ClientOnly) {
    # No gm-handbook, no data\, and no server-start scripts: all three are the world server's,
    # and shipping them to a player is 400 MB of game tables they cannot use and a script that
    # would start a second server on their machine.
    # **No install.ps1 and no README either.** The owner, 2026-09-07, once the launcher had taken
    # over the last job the installer did that nothing else could:
    #
    #   the database        gone - a client has none; accounts live on the server
    #   the first account   gone - made in the launcher's REGISTER tab with a GM's code
    #   the server address  the launcher has boxes for it and REMEMBERS them after a
    #                       successful Start Game
    #   the firewall rule   written by the launcher at Start Game, scoped to the address that
    #                       launch resolved - crates/launcher/src/firewall.rs
    #   the runtime check   moved into the launcher - crates/launcher/src/stub.rs - which only
    #                       works because of the STATIC CRT below: a dynamically linked
    #                       launcher would have died before it could report anything
    #   copying to C:\      unzip it wherever you like; the launcher finds client\ beside
    #                       itself
    #   a desktop shortcut  the only thing genuinely lost, and it is a right-click away
    #
    # So: three entries and no instructions to follow. If something here ever needs a step
    # again, the step belongs in the launcher, not in a script beside it.
    Remove-Item (Join-Path $stage 'bin') -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host 'CLIENT-ONLY payload: launcher, stub and client. No servers, no handbook, no' -ForegroundColor Cyan
    Write-Host '  data, no install script, no README - unzip and run the launcher.' -ForegroundColor Cyan
} else {
    Copy-Item $handbook (Join-Path $stage 'gm-handbook') -Recurse -Force
    Copy-Item $dataDir  (Join-Path $stage 'data')        -Recurse -Force
    foreach ($f in @('install.ps1', 'start-server.ps1', 'start-servers.cmd', 'README.txt')) {
        Copy-Item (Join-Path $here "installer\$f") $stage -Force
    }
}

if ($NoClient) {
    Write-Host 'skipping the client (-NoClient)' -ForegroundColor Yellow
} else {
    Write-Host 'copying the client - this is the slow part, about 900 MB...' -ForegroundColor Cyan
    $clientDst = Join-Path $stage 'client'
    # robocopy rather than Copy-Item: it is an order of magnitude faster on a tree this
    # size, and /XF and /XD let the exclusions be stated once rather than filtered after.
    # Exit codes 0-7 are success (8+ is a real failure), which is why $LASTEXITCODE is
    # tested against 8 rather than 0.
    # `*.bak` are the pristine classic archives backport_install.py keeps beside each hybrid
    # one - the dev box's undo, 60 MB of it, and nothing the client or the launcher reads.
    $excludeFiles = @(
        'maplecw-hook.*', 'grap64.dll.orig', 'maplecw.toml',
        'Maple_A_*.jpg', 'grap-stub.log', '*.dmp', '*.bak', '*.bak.bak'
    )
    $excludeDirs = @('previous-runs', 'dumps')
    & robocopy $clientSrc $clientDst /E /NFL /NDL /NJH /NJS /NP /R:1 /W:1 `
        /XF $excludeFiles /XD $excludeDirs | Out-Null
    if ($LASTEXITCODE -ge 8) { Fail "robocopy failed with code $LASTEXITCODE" }

    # Undo our own patching so the payload is the client as shipped. See the header.
    $dstGrap    = Join-Path $clientDst 'grap64.dll'
    $srcOrig    = Join-Path $clientSrc 'grap64.dll.orig'
    $dstGrapDir = Join-Path $clientDst 'grap'
    $dstDisabled= Join-Path $clientDst 'grap.disabled'
    if (Test-Path $srcOrig) {
        Copy-Item $srcOrig $dstGrap -Force
        Write-Host '  restored the original grap64.dll into the payload'
    } else {
        Write-Host '  NOTE: no grap64.dll.orig here, so the payload carries whatever' -ForegroundColor Yellow
        Write-Host '  grap64.dll this machine has. If that is the stub, the installed' -ForegroundColor Yellow
        Write-Host '  client has no original to back up and Start Game will preserve it' -ForegroundColor Yellow
        Write-Host '  as .orig - harmless, but it is no longer a pristine client.' -ForegroundColor Yellow
    }
    if ((Test-Path $dstDisabled) -and -not (Test-Path $dstGrapDir)) {
        Rename-Item $dstDisabled 'grap'
        Write-Host '  renamed grap.disabled back to grap\ in the payload'
    }
}

# ---------------------------------------------------------------- report
$bytes = (Get-ChildItem $stage -Recurse -File | Measure-Object -Property Length -Sum).Sum
Write-Host ''
Write-Host ("staged {0}  ({1:N0} MB, {2:N0} files)" -f $stage, ($bytes / 1MB),
            (Get-ChildItem $stage -Recurse -File).Count) -ForegroundColor Green

if (-not $NoZip) {
    Write-Host 'compressing - expect several minutes and little shrinkage; WZ is already compressed'
    if (Test-Path $zipPath) { Remove-Item $zipPath -Force }
    # ZipFile rather than Compress-Archive: 5.1's cmdlet buffers far too much for a payload
    # this size and has been known to run the host out of memory.
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [System.IO.Compression.ZipFile]::CreateFromDirectory(
        $stage, $zipPath,
        [System.IO.Compression.CompressionLevel]::Optimal, $true)
    $zipMb = (Get-Item $zipPath).Length / 1MB
    Write-Host ("wrote {0}  ({1:N0} MB)" -f $zipPath, $zipMb) -ForegroundColor Green
    Write-Host ("sha256 {0}" -f (Get-FileHash $zipPath -Algorithm SHA256).Hash)
}

Write-Host ''
if ($ClientOnly) {
    # There is no install.ps1 in a client payload any more, so telling somebody to run one
    # would be a command that fails on a file that is not there.
    Write-Host 'On the player''s machine: unzip anywhere, then run maplecw-launcher.exe.' -ForegroundColor Cyan
    Write-Host '  Nothing to install. It asks for administrator (the client requires it),' -ForegroundColor Cyan
    Write-Host '  writes the outbound firewall rule itself, and remembers the server address,' -ForegroundColor Cyan
    Write-Host '  both ports and the account name after the first successful Start Game.' -ForegroundColor Cyan
    Write-Host '  Accounts are made in its REGISTER tab with a code a GM mints (!registrationcode).' -ForegroundColor Cyan
    Write-Host '  If the Visual C++ redistributable is missing it says so in words at Start Game.' -ForegroundColor Cyan
} else {
    Write-Host 'On the target machine: unzip, then from an ELEVATED PowerShell window run' -ForegroundColor Cyan
    Write-Host '  powershell -ExecutionPolicy Bypass -File "<unzipped path>\MapleCW\install.ps1"' -ForegroundColor Cyan
}
