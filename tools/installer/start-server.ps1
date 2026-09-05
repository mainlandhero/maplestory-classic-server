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

    THE HOST A CLIENT IS TOLD TO DIAL IS DECIDED PER CONNECTION, NOT HERE. -Bind is what
    the sockets listen on and stays 0.0.0.0. The host written into the migration packet
    comes from -Advertise: `auto` (the default) tells a LAN or VPN client the address it
    reached this box on, and an internet client this box's PUBLIC address, discovered at
    startup and re-checked every ten minutes; an IPv4 address pins one host for everyone.
    This script used to build the channel list from -Bind, which meant the default
    advertised 0.0.0.0 and a LAN install needed -Bind <lan ip> to work at all. The first
    lines of login.log print what was decided and, under auto, the public address found.
#>
[CmdletBinding()]
param(
    [string]$Bind = '0.0.0.0',
    # auto | list | an IPv4 address. See the header; crates\net\src\advertise.rs owns the rule.
    [string]$Advertise = 'auto',
    [int]$Port = 8484,
    [int]$ChannelPort = 8485,
    [int]$Channels = 2,
    # Kept so an old command line is told what changed rather than silently ignored.
    [string]$Account,
    # Serve a connection that cannot be tied to a launcher sign-in as THIS account instead of
    # refusing it. Off by default - login is enforced. Dev and smoke-test use only.
    [string]$FallbackAccount,
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

# LOGIN IS ENFORCED. A connection the login server cannot tie to a launcher sign-in is
# refused with a login failure - it is not served anybody's characters. This script used to
# pick the FIRST account in the database as a fallback and serve every unattributable
# connection as that person, which on a forwarded port meant a stranger reaching 8484 was
# served as the administrator. The owner, 2026-09-05: "enforce login". -FallbackAccount <name>
# turns the old behaviour back on by hand, and the login server's banner says so when it is.
if ($Account) {
    Write-Host '-Account no longer selects a fallback. Login is enforced; use -FallbackAccount <name> to serve unattributable connections as one account (dev/test only).' -ForegroundColor Yellow
}
$listing = & (Join-Path $bin 'maplecw-useradd.exe') --db "$db" --list 2>&1 | Out-String
if (-not ($listing -match '^\s*\d+\s+\S+')) {
    throw "no accounts in $db - create your own (and make it GM) with maplecw-useradd before starting the servers; players then register with codes you mint"
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

# Bare PORTS, deliberately no host. The host is decided per connection by --advertise (see
# the header); writing $Bind here is what used to advertise 0.0.0.0 to every client.
$channelList = (0..($Channels - 1) | ForEach-Object { "$($ChannelPort + $_)" }) -join ','

# Built on its own line rather than inline in -ArgumentList. That is not style: an inline
# array parses fine and hands the server one mangled argument, and a syntax check does not
# catch it. tools\test-server.ps1 carries the same note for the same reason.
$loginArgs = @(
    '--bind', "$($Bind):$Port",
    '--db', "$db",
    '--channels', "$channelList",
    '--advertise', "$Advertise"
)
if ($FallbackAccount) { $loginArgs += @('--fallback-account', "$FallbackAccount") }
$watched = @()
$login = Start-Process -FilePath (Join-Path $bin 'maplecw-login.exe') -WorkingDirectory $root `
    -ArgumentList $loginArgs -PassThru -NoNewWindow `
    -RedirectStandardOutput (Join-Path $root 'login.log') `
    -RedirectStandardError  (Join-Path $root 'login.log.err')
$enforced = if ($FallbackAccount) { "FALLBACK '$FallbackAccount' - anything reaching this port is served as it" } else { 'login enforced - unattributable connections are refused' }
Write-Host "login server  pid $($login.Id)  $($Bind):$Port  $enforced"
Write-Host "              channels advertised as: $Advertise  (login.log's first lines say what each client is told)"
$watched += $login

# THE SIGN-IN SERVICE, and on an installed box it must be REACHABLE.
#
# The launcher on a client machine has no database to read, so it signs in over HTTP to this
# service. Bound to $Bind - which defaults to 0.0.0.0 here, unlike the dev script - because a
# loopback bind would mean only this machine could ever log in.
#
# TLS, to a certificate the service makes for itself beside the database on first start.
# Every launcher pins its fingerprint, which is printed below once the service has written
# it. This block used to say the password crossed the wire in plain text; since 2026-09-05
# it does not - crates/auth/src/tls.rs and crates/tlspin.
$authArgs = @('--db', "$db", '--bind', "$Bind", '--port', '8080')
$auth = Start-Process -FilePath (Join-Path $bin 'maplecw-auth.exe') -WorkingDirectory $root `
    -ArgumentList $authArgs -PassThru -NoNewWindow `
    -RedirectStandardOutput (Join-Path $root 'auth.log') `
    -RedirectStandardError  (Join-Path $root 'auth.log.err')
Write-Host "sign-in       pid $($auth.Id)  $($Bind):8080  <- the launcher signs in here (TLS)"
$watched += $auth

# The fingerprint every client must pin. The service writes it beside the database within a
# moment of starting; waited for rather than assumed, so the line below is the real value and
# not a stale file from a certificate that has since been regenerated.
$pinFile = Join-Path $root 'auth-cert-fingerprint.txt'
$pinStamp = if (Test-Path $pinFile) { (Get-Item $pinFile).LastWriteTime } else { [datetime]::MinValue }
$waited = 0
while ($waited -lt 50 -and -not ((Test-Path $pinFile) -and (Get-Item $pinFile).LastWriteTime -gt $pinStamp)) {
    Start-Sleep -Milliseconds 100
    $waited++
}
if (Test-Path $pinFile) {
    $pin = (Get-Content $pinFile -Raw).Trim()
    Write-Host ''
    Write-Host 'EVERY CLIENT MACHINE MUST PIN THIS CERTIFICATE FINGERPRINT:' -ForegroundColor Cyan
    Write-Host ("    {0}" -f $pin) -ForegroundColor Cyan
    Write-Host '  install.ps1 -AuthFingerprint <it>, or auth_fingerprint = "<it>" in'
    Write-Host '  maplecw-launcher.toml beside the launcher. A launcher without it refuses'
    Write-Host '  to sign in. See SERVER-README.txt step 5.'
    Write-Host ''
} else {
    Write-Host '  (the sign-in service has not written auth-cert-fingerprint.txt yet - read auth.log)' -ForegroundColor Yellow
}

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
        '--advertise', "$Advertise",
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
Write-Host '  The launcher decides which account plays. A client that did not come'
Write-Host '  through a launcher sign-in is refused at the login screen.'
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
