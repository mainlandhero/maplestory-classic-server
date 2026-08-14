<#
.SYNOPSIS
    Prepare the local test client: neutralise GameGuard without touching the original install.

.DESCRIPTION
    Operates only on the *copy* at -ClientDir. The original install is never read-write.

    What it does:
      1. Backs up the real grap64.dll to grap64.dll.orig (once).
      2. Drops in the no-op stub built from crates/grap-stub.
      3. Renames the grap\ folder to grap.disabled so NGService.exe and the
         BlackCat64.sys kernel driver cannot be launched at all.

    Nothing is installed system-wide: no service is created and no driver is loaded.

.EXAMPLE
    pwsh tools/setup-client.ps1 -Verify
    pwsh tools/setup-client.ps1
    pwsh tools/setup-client.ps1 -Restore
#>
[CmdletBinding()]
param(
    [string]$ClientDir,
    [string]$StubPath,
    [string]$OriginalInstall = 'C:\Nexon\Library\maplestorycw\appdata',
    [switch]$Restore,
    [switch]$Verify
)

$ErrorActionPreference = 'Stop'

# $PSScriptRoot is not reliably populated in the param block across hosts, so the
# repo-relative defaults are resolved here instead.
$here = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
$repo = Split-Path -Parent $here
if (-not $ClientDir) { $ClientDir = Join-Path $repo 'client-patched' }
if (-not $StubPath)  { $StubPath  = Join-Path $repo 'target\release\grap64.dll' }

if (-not (Test-Path $ClientDir)) { throw "Client directory not found: $ClientDir" }
$ClientDir = (Resolve-Path $ClientDir).Path

$grapDll     = Join-Path $ClientDir 'grap64.dll'
$grapBackup  = Join-Path $ClientDir 'grap64.dll.orig'
$grapDir     = Join-Path $ClientDir 'grap'
$grapDirOff  = Join-Path $ClientDir 'grap.disabled'

# Refuse to run against the real install, whatever was passed in.
if ($ClientDir -eq (Resolve-Path $OriginalInstall -ErrorAction SilentlyContinue).Path) {
    throw "Refusing to modify the original install at $OriginalInstall. Point -ClientDir at the copy."
}

function Show-State {
    Write-Host "`nclient: $ClientDir"
    $stubbed = Test-Path $grapBackup
    $ggOff   = Test-Path $grapDirOff
    if (Test-Path $grapDll) {
        $len = (Get-Item $grapDll).Length
        $which = if ($stubbed) { 'STUB' } else { 'original' }
        Write-Host ("  grap64.dll      {0,12:N0} bytes  ({1})" -f $len, $which)
    } else {
        Write-Host '  grap64.dll      MISSING'
    }
    Write-Host ("  grap\ folder    {0}" -f $(if ($ggOff) { 'DISABLED (renamed grap.disabled)' }
                                            elseif (Test-Path $grapDir) { 'present - GameGuard could launch' }
                                            else { 'absent' }))
    $state = if ($stubbed -and $ggOff) { 'GameGuard NEUTRALISED' } else { 'GameGuard ACTIVE' }
    Write-Host "  => $state`n"
}

if ($Verify) { Show-State; return }

if ($Restore) {
    if (Test-Path $grapBackup) {
        Move-Item -Force $grapBackup $grapDll
        Write-Host 'restored original grap64.dll'
    }
    if (Test-Path $grapDirOff) {
        Move-Item -Force $grapDirOff $grapDir
        Write-Host 'restored grap\ folder'
    }
    Show-State
    return
}

if (-not (Test-Path $StubPath)) {
    throw "Stub not found at $StubPath. Build it first:  cargo build --release -p grap-stub"
}

# Sanity-check the stub really is a drop-in before displacing anything.
$stubLen = (Get-Item $StubPath).Length
if ($stubLen -lt 1kb) { throw "Stub at $StubPath looks empty ($stubLen bytes)." }

if (-not (Test-Path $grapBackup)) {
    if (-not (Test-Path $grapDll)) { throw "No grap64.dll in $ClientDir - is this a client directory?" }
    Copy-Item $grapDll $grapBackup
    Write-Host "backed up original grap64.dll -> grap64.dll.orig"
} else {
    Write-Host 'backup already exists, leaving it alone'
}

Copy-Item -Force $StubPath $grapDll
Write-Host ("installed stub grap64.dll ({0:N0} bytes)" -f $stubLen)

if ((Test-Path $grapDir) -and -not (Test-Path $grapDirOff)) {
    Move-Item $grapDir $grapDirOff
    Write-Host 'renamed grap\ -> grap.disabled (NGService.exe and BlackCat64.sys cannot run)'
}

Show-State
Write-Host 'To capture which entry points the client calls, set before launching:'
Write-Host ('  $env:GRAP_STUB_LOG = "{0}\grap-stub.log"' -f $ClientDir)
