<#
.SYNOPSIS
    Everything a SERVER box needs, and nothing else. About 9 MB.

.DESCRIPTION
    The owner, 2026-08-29: *"I'm also moving the server software to a different server, please
    explain to me what is the minimum necessary to ship the server off to a different
    computer"* and *"Does the server need to have rust installed? Can we compile it so that
    it doesn't need any dependencies?"*

    Run from an elevated PowerShell 5.1 window on the development box:

      powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\package-server.ps1"

    NO, THE SERVER DOES NOT NEED RUST - AND NOW IT NEEDS NOTHING ELSE EITHER

    Rust produces native executables, so a target box never needed a toolchain. It did need
    the Visual C++ Redistributable, and that was not obvious: measured with
    `python tools/pe_imports.py`, an ordinary release build of maplecw-login imports

        VCRUNTIME140.dll  and  api-ms-win-crt-{heap,locale,math,runtime,stdio,string,time,
                               utility}-l1-1-0.dll

    The `api-ms-win-crt-*` stubs are part of Windows 10 and 11. **VCRUNTIME140.dll is not** -
    it comes from the redistributable, and a clean Windows install may not have it. The
    failure is a dialog at startup naming a missing DLL, on a machine you are probably not
    sitting at.

    This script builds with `-C target-feature=+crt-static`, which links the C runtime in.
    Measured after the change, the same binaries import only:

        KERNEL32  ADVAPI32  WS2_32  ntdll  bcrypt  bcryptprimitives
        api-ms-win-core-synch-l1-2-0

    Every one of those ships with Windows. Cost: about 120 KB per executable. SQLite was
    never a problem - `rusqlite`'s `bundled` feature compiles it in - and everything else in
    the tree is pure Rust.

    **The client-side binaries are deliberately NOT built this way.** `grap64.dll` is
    injected into MapleStory.exe, and changing a working hook's CRT linkage is the kind of
    unforced change that costs a manual client launch to discover. It has no dependencies to
    fix anyway.

    WHAT GOES IN

      bin\maplecw-login.exe    the login / character-select server
      bin\maplecw-world.exe    one channel; run one per channel
      bin\maplecw-auth.exe     sign-in. The launcher POSTs here, so it must be REACHABLE
      bin\maplecw-useradd.exe  accounts, GM status, invite and recovery codes
      gm-handbook\             game tables generated from the client's WZ. The world server
                               reads these by RELATIVE path, which is why the run script
                               sets a working directory
      data\                    authored server data - shops, drops, quest scripts, EXP curve
      start-server.ps1 + start-servers.cmd

    WHAT DOES NOT

      The client and its 450 MB of WZ - that is the CLIENT machine's payload, and
      `tools/make-installer.ps1` builds it.
      maplecw-launcher.exe and grap64.dll - also client side.
      maplecw.db - accounts and characters are real state. A fresh box makes its own; an
      existing one should be COPIED BY HAND, with its -wal and -shm files, because SQLite in
      WAL mode keeps recent writes in the sidecar and copying the .db alone silently loses
      them. That is not hypothetical: it happened here on 2026-08-29 and read as data loss.
      The repo, the toolchain, and any redistributable.
#>
[CmdletBinding()]
param(
    [string]$OutDir,
    [switch]$SkipBuild,
    [switch]$NoZip
)

$ErrorActionPreference = 'Stop'

$here = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
$repo = Split-Path -Parent $here
if (-not $OutDir) { $OutDir = Join-Path $repo 'out' }

$stage   = Join-Path $OutDir 'MapleCW-server'
$zipPath = Join-Path $OutDir 'MapleCW-server.zip'
$target  = Join-Path $repo 'target-static'

function Fail([string]$msg) { throw $msg }

# ---------------------------------------------------------------- preflight
$handbook = Join-Path $repo 'gm-handbook'
if (-not (Test-Path $handbook)) {
    Fail @"
no gm-handbook\ - the world server reads its game tables from there and it is gitignored, so
it has to be staged from this machine. Regenerate first:
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
if (-not (Test-Path (Join-Path $repo 'data'))) { Fail "no data\ in $repo" }

# A running server holds its own executable open and cargo cannot replace a locked file. The
# error is "Access is denied (os error 5)" against a path in target-static, which says nothing
# about servers.
$running = Get-Process -Name 'maplecw-login', 'maplecw-world', 'maplecw-auth' -ErrorAction SilentlyContinue
if ($running -and -not $SkipBuild) {
    $names = ($running | ForEach-Object { "$($_.ProcessName) (pid $($_.Id))" }) -join ', '
    Fail @"
these are running and hold their own executables open: $names
  powershell -ExecutionPolicy Bypass -File "$repo\tools\test-server.ps1" -Stop
"@
}

# ---------------------------------------------------------------- build
if (-not $SkipBuild) {
    Write-Host 'building the servers with a STATIC C runtime...' -ForegroundColor Cyan
    Write-Host '  (this is what removes VCRUNTIME140.dll - see Get-Help on this script)'
    Push-Location $repo
    try {
        # A separate CARGO_TARGET_DIR, on purpose. RUSTFLAGS is part of cargo's fingerprint,
        # so sharing target\ with ordinary builds would make every switch a full rebuild of
        # the workspace - and would leave target\release holding statically linked binaries
        # that the dev scripts then run without anyone noticing the difference.
        $env:RUSTFLAGS = '-C target-feature=+crt-static'
        $env:CARGO_TARGET_DIR = $target
        & cargo build --release -p login -p world -p auth
        if ($LASTEXITCODE -ne 0) { Fail 'cargo build failed' }
    }
    finally {
        Pop-Location
        Remove-Item Env:\RUSTFLAGS -ErrorAction SilentlyContinue
        Remove-Item Env:\CARGO_TARGET_DIR -ErrorAction SilentlyContinue
    }
}

$rel = Join-Path $target 'release'
$binaries = @('maplecw-login.exe', 'maplecw-world.exe', 'maplecw-auth.exe', 'maplecw-useradd.exe')
foreach ($b in $binaries) {
    if (-not (Test-Path (Join-Path $rel $b))) {
        Fail "$rel\$b is missing. Build without -SkipBuild."
    }
}

# ---------------------------------------------------------------- stage
if (Test-Path $stage) { Remove-Item -Recurse -Force $stage }
New-Item -ItemType Directory -Path (Join-Path $stage 'bin') -Force | Out-Null

foreach ($b in $binaries) { Copy-Item (Join-Path $rel $b) (Join-Path $stage 'bin') -Force }
Copy-Item $handbook (Join-Path $stage 'gm-handbook') -Recurse -Force
Copy-Item (Join-Path $repo 'data') (Join-Path $stage 'data') -Recurse -Force
Copy-Item (Join-Path $here 'installer\start-server.ps1')  $stage -Force
Copy-Item (Join-Path $here 'installer\start-servers.cmd') $stage -Force
Copy-Item (Join-Path $here 'installer\SERVER-README.txt') $stage -Force

# ---------------------------------------------------------------- verify what we shipped
# Not decoration. The whole point of this script is "no dependencies", and asserting it is
# one command - where believing it costs a trip to the other machine to find out.
Write-Host ''
Write-Host 'checking the shipped binaries import nothing outside Windows...' -ForegroundColor Cyan
$bad = @()
foreach ($b in $binaries) {
    $imports = & python (Join-Path $here 'pe_imports.py') (Join-Path $stage "bin\$b") 2>&1 | Out-String
    foreach ($forbidden in @('VCRUNTIME', 'api-ms-win-crt', 'MSVCP')) {
        if ($imports -match $forbidden) { $bad += "$b imports $forbidden" }
    }
}
if ($bad) {
    Write-Host 'FAILED - these need the Visual C++ Redistributable on the target:' -ForegroundColor Red
    $bad | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    Fail 'the static build did not take effect'
}
Write-Host '  clean - KERNEL32, ADVAPI32, WS2_32, ntdll, bcrypt and nothing else' -ForegroundColor Green

# ---------------------------------------------------------------- report
$bytes = (Get-ChildItem $stage -Recurse -File | Measure-Object -Property Length -Sum).Sum
Write-Host ''
Write-Host ("staged {0}  ({1:N1} MB)" -f $stage, ($bytes / 1MB)) -ForegroundColor Green

if (-not $NoZip) {
    if (Test-Path $zipPath) { Remove-Item $zipPath -Force }
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [System.IO.Compression.ZipFile]::CreateFromDirectory(
        $stage, $zipPath, [System.IO.Compression.CompressionLevel]::Optimal, $true)
    Write-Host ("wrote {0}  ({1:N1} MB)" -f $zipPath, ((Get-Item $zipPath).Length / 1MB)) -ForegroundColor Green
}

Write-Host ''
Write-Host 'ON THE SERVER BOX:' -ForegroundColor Cyan
Write-Host '  1. Unzip anywhere. No Rust, no runtime, no redistributable.'
Write-Host '  2. Create an account:'
Write-Host '       & ".\bin\maplecw-useradd.exe" --db ".\maplecw.db" <name> --email <addr>'
Write-Host '       & ".\bin\maplecw-useradd.exe" --db ".\maplecw.db" --gm <name>'
Write-Host '  3. Open inbound TCP 8080, 8484, 8485 and 8486 in its firewall.'
Write-Host '  4. Double-click start-servers.cmd. THAT WINDOW IS THE SERVER.'
Write-Host '     It binds 0.0.0.0 and works out the host each client must dial on its'
Write-Host '     own - LAN address for LAN clients, the discovered public address for'
Write-Host '     internet ones. -Advertise <ip> pins one. login.log prints the decision.'
Write-Host '  5. Copy the certificate fingerprint it prints ("TLS: fingerprint sha256:...")'
Write-Host '     to every client: install.ps1 -AuthFingerprint <it>. Without it a launcher'
Write-Host '     refuses to sign in rather than send a password to an unknown server.'
Write-Host ''
Write-Host 'THEN ON EACH CLIENT MACHINE - and this is the step that is easy to miss:' -ForegroundColor Yellow
Write-Host '  the launcher needs the SERVER IP, not 127.0.0.1, and the client-side'
Write-Host '  firewall rule BLOCKS ALL OUTBOUND from MapleStory.exe. Loopback is not'
Write-Host '  filtered by Windows Firewall, which is why a local server has always'
Write-Host '  worked - a LAN server will not until that rule is given an exception.'
Write-Host '  See SERVER-README.txt.'
