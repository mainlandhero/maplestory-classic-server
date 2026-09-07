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
        api-ms-win-core-synch-l1-2-0    IPHLPAPI (login and world only)

    Every one of those ships with Windows. That list is now ASSERTED by the check below
    rather than eyeballed - it fails on any name not in it, in either direction, so a new
    dependency cannot slip through the way IPHLPAPI did.

    THE CLIENT SIDE IS A DIFFERENT STORY AND THE DIFFERENCE WAS MISSED. maplecw-launcher.exe
    and grap64.dll both import VCRUNTIME140.dll, so a client machine DOES need the Visual C++
    Redistributable. See docs\client-machine-checklist.md; tools\installer\install.ps1
    refuses to install without it.

    Cost: about 120 KB per executable. SQLite was
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
# **The sign-in certificate travels with the server, and that is what makes the client need no
# configuration.** crates/auth/src/tls.rs mints a self-signed certificate only when auth-cert.pem
# is ABSENT and reloads it otherwise, and crates/launcher/build.rs compiles that certificate's
# fingerprint into every launcher. So a server deployed without these two files would mint a new
# one and every launcher built here would refuse to sign in - correctly, because a pin that
# accepted a certificate it did not recognise would not be a pin.
#
# auth-key.pem is the sign-in service's PRIVATE KEY. It is self-signed, it authenticates nothing
# but this service, and it never leaves the owner's own machines - but it is a private key in a zip
# and it is named as one here rather than left to be discovered.
$certFiles = @('auth-cert.pem', 'auth-key.pem', 'auth-cert-fingerprint.txt')
$missingCert = @()
foreach ($c in $certFiles) {
    $src = Join-Path $repo $c
    if (Test-Path $src) { Copy-Item $src $stage -Force } else { $missingCert += $c }
}
if ($missingCert.Count -gt 0) {
    Write-Host ''
    Write-Host ("THE SIGN-IN CERTIFICATE IS NOT IN THIS PACKAGE: {0}" -f ($missingCert -join ', ')) -ForegroundColor Red
    Write-Host '  The server will mint a NEW self-signed certificate on first start, and every' -ForegroundColor Red
    Write-Host '  launcher built from this checkout has the OLD fingerprint compiled in, so no' -ForegroundColor Red
    Write-Host '  client will be able to sign in. Start the sign-in service once here to create' -ForegroundColor Red
    Write-Host '  them, then re-package - or pass the new fingerprint to install.ps1 by hand.' -ForegroundColor Red
    Write-Host ''
} else {
    Write-Host '  sign-in certificate included - clients need no fingerprint configuration'
}
Copy-Item (Join-Path $here 'installer\start-server.ps1')  $stage -Force
Copy-Item (Join-Path $here 'installer\start-servers.cmd') $stage -Force
Copy-Item (Join-Path $here 'installer\SERVER-README.txt') $stage -Force

# ---------------------------------------------------------------- verify what we shipped
# Not decoration. The whole point of this script is "no dependencies", and asserting it is
# one command - where believing it costs a trip to the other machine to find out.
Write-Host ''
Write-Host 'checking the shipped binaries import nothing outside Windows...' -ForegroundColor Cyan

# Every name here ships with Windows. The list is ASSERTED, not merely scanned for
# known-bad names: the green line below claims "and nothing else", and a check that only
# greps for VCRUNTIME cannot support that sentence - a new dependency on some DLL that is
# not on a clean box would have sailed straight through it. Anything unlisted fails here,
# named, which is the right place to decide whether it is safe to add.
$WindowsDlls = @(
    'kernel32.dll', 'advapi32.dll', 'ws2_32.dll', 'ntdll.dll',
    'bcrypt.dll', 'bcryptprimitives.dll', 'iphlpapi.dll',
    'api-ms-win-core-synch-l1-2-0.dll'
)
$bad = @()
foreach ($b in $binaries) {
    # pe_import_dlls.py is stdlib-only ON PURPOSE. Its richer sibling pe_imports.py needs
    # `pefile`, which is installed in WISP's per-user site-packages and is NOT visible to
    # the elevated window this script is run from - so on 2026-09-06 this very step died
    # with ModuleNotFoundError, after a full static build. `python -s tools/pe_imports.py`
    # reproduces it without elevation.
    $dlls = & python (Join-Path $here 'pe_import_dlls.py') (Join-Path $stage "bin\$b") 2>&1
    if ($LASTEXITCODE -ne 0) {
        # A reader that failed must never read as a clean result. The old form captured
        # stderr into the same variable it then searched for forbidden names, so a
        # traceback contains no 'VCRUNTIME' and would have passed as proof of a clean
        # binary had anything swallowed the error.
        Write-Host ($dlls | Out-String) -ForegroundColor Red
        Fail "could not read the imports of $b - the check did NOT run, so nothing here is verified"
    }
    foreach ($line in $dlls) {
        $dll = ("$line".Split(' ')[0]).Trim().ToLower()
        if (-not $dll) { continue }
        if ($WindowsDlls -notcontains $dll) { $bad += "$b imports $dll" }
    }
}
if ($bad) {
    Write-Host 'FAILED - the shipped binaries import something a clean Windows box may not have:' -ForegroundColor Red
    $bad | Sort-Object -Unique | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    if (($bad -join ' ') -match 'vcruntime|msvcp|api-ms-win-crt') {
        Fail 'the static build did not take effect - these come from the Visual C++ Redistributable'
    }
    Fail @"
an import outside the known-good list. If it really does ship with Windows, add it to
`$WindowsDlls in this script and say why. If it does not, the target box needs it installed
and this package is no longer self-contained.
"@
}
Write-Host ("  clean - {0}" -f ($WindowsDlls -join ', ')) -ForegroundColor Green
Write-Host '  every one of those ships with Windows; nothing else is imported.' -ForegroundColor Green

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
Write-Host '     8080 is the SIGN-IN port. If it is taken, start-servers.cmd takes'
Write-Host '     -AuthPort <n> and the launcher has a "Sign-in port" box to match.'
Write-Host '  4. Double-click start-servers.cmd. THAT WINDOW IS THE SERVER.'
Write-Host '     It binds 0.0.0.0 and works out the host each client must dial on its'
Write-Host '     own - LAN address for LAN clients, the discovered public address for'
Write-Host '     internet ones. -Advertise <ip> pins one. login.log prints the decision.'
Write-Host '  5. NOTHING to do about the certificate. This package carries auth-cert.pem, so'
Write-Host '     the server presents the SAME certificate the launchers were built against,'
Write-Host '     and every launcher has that fingerprint compiled in. Clients need no'
Write-Host '     certificate configuration at all.'
Write-Host '     If the server ever starts WITHOUT auth-cert.pem it mints a new one and no'
Write-Host '     client will sign in - correctly, because the pin refuses what it does not'
Write-Host '     recognise. Fix by restoring the file, or install.ps1 -AuthFingerprint <new>.'
Write-Host ''
Write-Host 'THEN ON EACH CLIENT MACHINE:' -ForegroundColor Yellow
Write-Host '  the launcher needs the SERVER IP, not 127.0.0.1. Type it into the'
Write-Host '  launcher (or pass install.ps1 -ServerIp) and it is remembered after the'
Write-Host '  first successful Start Game.'
Write-Host '  The outbound block that keeps the patched client off the internet is now'
Write-Host '  written BY THE LAUNCHER at Start Game, scoped to the address that launch'
Write-Host '  resolved - so a LAN server and a CNAME both work with no exception to add'
Write-Host '  by hand. It used to block ALL outbound and need one; that is no longer so.'
Write-Host '  See SERVER-README.txt.'
