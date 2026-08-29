<#
.SYNOPSIS
    Start the MapleCW servers on an installed machine.

.DESCRIPTION
    The installed counterpart of tools\test-server.ps1. That script builds from source,
    installs the hook and launches the client; this one only starts the servers, because on
    an installed machine the launcher is what starts the client.

      powershell -ExecutionPolicy Bypass -File "C:\MapleCW\start-server.ps1"

    Or just double-click `start-servers.cmd`, which is the same thing without needing a
    shell open first - Windows opens a double-clicked .ps1 in an editor rather than running
    it, which is the only reason that wrapper exists.

    **This window is the server.** It starts them attached to its own console and then waits,
    so closing it takes them with it and there is no stop script to forget. `-Stop` remains
    for the one case a close cannot cover: killing this process from Task Manager sends no
    console event and runs no cleanup, so the children would survive that.

    Measured 2026-08-28 rather than assumed, in both directions: a child started with
    `-NoNewWindow` is gone after its console window is closed, and one started with
    `-WindowStyle Hidden` is still running. The second half is the control, and it is why the
    old script needed a separate stop step.

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
    foreach ($n in @('maplecw-login', 'maplecw-world', 'maplecw-auth')) {
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
$watched = @()
$login = Start-Process -FilePath (Join-Path $bin 'maplecw-login.exe') -WorkingDirectory $root `
    -ArgumentList $loginArgs -PassThru -NoNewWindow `
    -RedirectStandardOutput (Join-Path $root 'login.log') `
    -RedirectStandardError  (Join-Path $root 'login.log.err')
Write-Host "login server  pid $($login.Id)  $($Bind):$Port  fallback account '$Account'"
$watched += $login

# THE SIGN-IN SERVICE, and on an installed box it must be REACHABLE.
#
# The launcher on a client machine has no database to read, so it signs in over HTTP to this
# service. Bound to $Bind - which defaults to 0.0.0.0 here, unlike the dev script - because a
# loopback bind would mean only this machine could ever log in.
#
# THE PASSWORD CROSSES THE WIRE IN PLAIN TEXT. That is stated in crates/launcher/src/http.rs
# and in docs/deployment.md, and it is the price of being installable at all. This is a test
# server on a network you control; do not put it on the internet.
$authArgs = @('--db', "$db", '--bind', "$Bind", '--port', '8080')
$auth = Start-Process -FilePath (Join-Path $bin 'maplecw-auth.exe') -WorkingDirectory $root `
    -ArgumentList $authArgs -PassThru -NoNewWindow `
    -RedirectStandardOutput (Join-Path $root 'auth.log') `
    -RedirectStandardError  (Join-Path $root 'auth.log.err')
Write-Host "sign-in       pid $($auth.Id)  $($Bind):8080  <- the launcher signs in here"
$watched += $auth

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
    $watched += $w
}

Write-Host ''
Write-Host 'THIS WINDOW IS THE SERVER. Close it to stop.' -ForegroundColor Green
Write-Host '  Now double-click maplecw-launcher.exe, sign in, and press Start Game.'
Write-Host '  The launcher decides which account plays; --account above is only the'
Write-Host '  fallback for a connection that arrives before anyone has signed in.'
Write-Host ''

# THE WAIT, and it is what makes closing this window enough.
#
# The servers were started with -NoNewWindow, so they share THIS console. Closing it sends
# CTRL_CLOSE_EVENT to every process attached, children included, and Windows gives them five
# seconds to go. Ctrl+C comes out of Start-Sleep as a terminating error instead, which is
# what the `finally` is for.
#
# "Gracefully" is worth being exact about, because nothing here runs a shutdown routine: it
# means nothing is lost. The server flushes stdout on every log line, so the logs are complete
# to the last thing that happened, and SQLite is in WAL mode, which is crash-safe by
# construction. A terminated server loses no state and no evidence.
try {
    while ($true) {
        Start-Sleep -Seconds 1
        $dead = @($watched | Where-Object { $_.HasExited })
        if ($dead.Count -gt 0) {
            Write-Host ''
            Write-Host 'A SERVER EXITED ON ITS OWN - that is not you closing the window.' -ForegroundColor Red
            foreach ($d in $dead) {
                Write-Host ("  {0} (pid {1}) exit code {2}" -f $d.ProcessName, $d.Id, $d.ExitCode) -ForegroundColor Red
            }
            Write-Host '  The usual cause is an account that does not exist. Read login.log' -ForegroundColor Red
            Write-Host ("  and login.log.err in {0}" -f $root) -ForegroundColor Red
            break
        }
    }
}
finally {
    Write-Host ''
    Write-Host 'stopping the servers...' -ForegroundColor Cyan
    Stop-All
    Write-Host 'stopped.' -ForegroundColor Green
}
