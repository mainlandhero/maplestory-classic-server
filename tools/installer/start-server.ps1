<#
.SYNOPSIS
    Start the MapleCW servers on an installed machine.

.DESCRIPTION
    The installed counterpart of tools\test-server.ps1. That script builds from source,
    installs the hook and launches the client; this one only starts the servers, because on
    an installed machine the launcher is what starts the client.

      powershell -ExecutionPolicy Bypass -File "C:\MapleCW\start-server.ps1"

    Leave the window open - closing it stops the servers. -Stop kills any that are running.

    Logs land beside the executables as login.log and world.log (and world-ch<N>.log for
    further channels). The previous run's logs are MOVED into previous-runs\, not deleted:
    a run's output is the most expensive data this project produces, and conclusions have
    died with an overwritten world.log more than once.
#>
[CmdletBinding()]
param(
    [string]$Bind = '0.0.0.0',
    [int]$Port = 8484,
    [int]$ChannelPort = 8485,
    [int]$Channels = 2,
    [string]$Account,
    [switch]$Stop
)

$ErrorActionPreference = 'Stop'
$root = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
$bin  = Join-Path $root 'bin'
$db   = Join-Path $root 'maplecw.db'

function Stop-All {
    foreach ($n in @('maplecw-login', 'maplecw-world')) {
        Get-Process -Name $n -ErrorAction SilentlyContinue | Stop-Process -Force
    }
}

if ($Stop) {
    Stop-All
    Write-Host 'stopped'
    return
}

if (-not (Test-Path $db)) {
    throw @"
no database at $db - create an account first:
  & "$bin\maplecw-useradd.exe" --db "$db" <name>
"@
}

# The fallback account: who a connection is served as when no launcher claim is live. The
# launcher overrides it every time it signs in, so this only matters for a server started
# before anyone has run the launcher.
if (-not $Account) {
    $listing = & (Join-Path $bin 'maplecw-useradd.exe') --db "$db" --list 2>&1 | Out-String
    $first = ($listing -split "`n" | Where-Object { $_ -match '^\s*\d+\s+(\S+)' } |
              Select-Object -First 1)
    if ($first -match '^\s*\d+\s+(\S+)') { $Account = $matches[1] }
}
if (-not $Account) {
    throw "no accounts in $db - create one with maplecw-useradd before starting the servers"
}

Stop-All
Start-Sleep -Milliseconds 300

# Archive rather than delete. See the header, and CLAUDE.md's section on why.
$archive = Join-Path $root 'previous-runs'
New-Item -ItemType Directory -Path $archive -Force | Out-Null
foreach ($log in (Get-ChildItem $root -Filter '*.log' -File -ErrorAction SilentlyContinue)) {
    $stamp = $log.LastWriteTime.ToString('yyyyMMdd-HHmmss')
    Move-Item $log.FullName (Join-Path $archive ("{0}-{1}.log" -f $log.BaseName, $stamp)) -Force
}

$channelList = (0..($Channels - 1) | ForEach-Object { "$($Bind):$($ChannelPort + $_)" }) -join ','

# Built on its own line rather than inline in -ArgumentList. That is not style: an inline
# array parses fine and hands the server one mangled argument, and a syntax check does not
# catch it. tools\test-server.ps1 carries the same note for the same reason.
$loginArgs = @(
    '--bind', "$($Bind):$Port",
    '--db', "$db",
    '--account', "$Account",
    '--channels', "$channelList"
)
$login = Start-Process -FilePath (Join-Path $bin 'maplecw-login.exe') -WorkingDirectory $root `
    -ArgumentList $loginArgs -PassThru -NoNewWindow `
    -RedirectStandardOutput (Join-Path $root 'login.log') `
    -RedirectStandardError  (Join-Path $root 'login.log.err')
Write-Host "login server  pid $($login.Id)  $($Bind):$Port  fallback account '$Account'"

for ($ch = 0; $ch -lt $Channels; $ch++) {
    $chLog = if ($ch -eq 0) { 'world.log' } else { "world-ch$ch.log" }
    # `--set-field-probe` is NOT optional and is always passed here. Its name is a fossil:
    # it now means "the channel answers at all". Without it `Session::handle` returns
    # nothing for every packet, the migration hello goes unanswered, and the client sits on
    # "Connecting..." looking exactly like a server that is not running. That cost a manual
    # launch on 2026-08-20; an installed machine must not be able to reproduce it, so there
    # is deliberately no switch to turn it off.
    $worldArgs = @(
        '--bind', "$($Bind):$($ChannelPort + $ch)",
        '--db', "$db",
        '--channel', "$ch",
        '--channels', "$channelList",
        '--set-field-probe'
    )
    $w = Start-Process -FilePath (Join-Path $bin 'maplecw-world.exe') -WorkingDirectory $root `
        -ArgumentList $worldArgs -PassThru -NoNewWindow `
        -RedirectStandardOutput (Join-Path $root $chLog) `
        -RedirectStandardError  (Join-Path $root "$chLog.err")
    Write-Host "channel $ch      pid $($w.Id)  $($Bind):$($ChannelPort + $ch)  -> $chLog"
}

Write-Host ''
Write-Host 'Servers are up. Now run the launcher, sign in, and press Start Game.' -ForegroundColor Green
Write-Host 'The launcher decides which account plays; --account above is only the fallback'
Write-Host 'for a connection that arrives before anyone has signed in.'
Write-Host ''
Write-Host "Stop them with:  powershell -ExecutionPolicy Bypass -File `"$root\start-server.ps1`" -Stop"
