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

    WHAT GOES IN THE PAYLOAD, AND WHAT DOES NOT

      maplecw-launcher.exe    at the ROOT, and that is not cosmetic - see below
      grap64.dll              the GameGuard stub, also at the root
      bin\            maplecw-login, maplecw-world, maplecw-useradd
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
    [switch]$NoClient
)

$ErrorActionPreference = 'Stop'

$here = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
$repo = Split-Path -Parent $here
if (-not $OutDir) { $OutDir = Join-Path $repo 'out' }

$clientSrc = Join-Path $repo 'client-patched'
$stage     = Join-Path $OutDir 'MapleCW'
$zipPath   = Join-Path $OutDir 'MapleCW-setup.zip'

function Fail([string]$msg) { throw $msg }

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
}

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
foreach ($needed in @('maps.txt', 'mobtemplates.txt', 'skills.txt', 'footholds.txt')) {
    if (-not (Test-Path (Join-Path $handbook $needed))) {
        Fail "gm-handbook\$needed is missing - regenerate the handbook before packaging"
    }
}

$dataDir = Join-Path $repo 'data'
if (-not (Test-Path $dataDir)) { Fail "no data\ - shops, drops and quest scripts live there" }

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

if (-not $SkipBuild) {
    Write-Host 'building release binaries...' -ForegroundColor Cyan
    Push-Location $repo
    try {
        & cargo build --release -p login -p world -p auth -p grap-stub -p launcher
        if ($LASTEXITCODE -ne 0) { Fail 'cargo build failed' }
    }
    finally { Pop-Location }
}

$rel = Join-Path $repo 'target\release'
# `To` is the payload-relative directory. The launcher and the stub go to the root because
# that is where the launcher's own path resolution expects them; see the header.
$binaries = @(
    @{ From = 'maplecw-launcher.exe'; To = '.';    Why = 'the launcher the owner signs in with' },
    @{ From = 'grap64.dll';           To = '.';    Why = 'the GameGuard stub the launcher installs' },
    @{ From = 'maplecw-login.exe';    To = 'bin';  Why = 'the login / character-select server' },
    @{ From = 'maplecw-world.exe';    To = 'bin';  Why = 'a channel server' },
    @{ From = 'maplecw-useradd.exe';  To = 'bin';  Why = 'creates the first account' }
)
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
    Copy-Item (Join-Path $rel $b.From) $dst -Force
}
Write-Host ("staged {0} binaries" -f $binaries.Count)

Copy-Item $handbook (Join-Path $stage 'gm-handbook') -Recurse -Force
Copy-Item $dataDir  (Join-Path $stage 'data')        -Recurse -Force
Copy-Item (Join-Path $here 'installer\install.ps1')      $stage -Force
Copy-Item (Join-Path $here 'installer\start-server.ps1') $stage -Force
Copy-Item (Join-Path $here 'installer\README.txt')       $stage -Force

if ($NoClient) {
    Write-Host 'skipping the client (-NoClient)' -ForegroundColor Yellow
} else {
    Write-Host 'copying the client - this is the slow part, about 900 MB...' -ForegroundColor Cyan
    $clientDst = Join-Path $stage 'client'
    # robocopy rather than Copy-Item: it is an order of magnitude faster on a tree this
    # size, and /XF and /XD let the exclusions be stated once rather than filtered after.
    # Exit codes 0-7 are success (8+ is a real failure), which is why $LASTEXITCODE is
    # tested against 8 rather than 0.
    $excludeFiles = @(
        'maplecw-hook.*', 'grap64.dll.orig', 'maplecw.toml',
        'Maple_A_*.jpg', 'grap-stub.log', '*.dmp'
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
Write-Host 'On the target machine: unzip, then from an ELEVATED PowerShell window run' -ForegroundColor Cyan
Write-Host '  powershell -ExecutionPolicy Bypass -File "<unzipped path>\MapleCW\install.ps1"' -ForegroundColor Cyan
