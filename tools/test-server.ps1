<#
.SYNOPSIS
    Run the client against the real login server, with characters that persist.

.DESCRIPTION
    The replacement for test-charselect.ps1. That script drives tools/handshake_probe.py,
    which replays bodies handed to it on a command line; this one starts crates/login,
    which reads and writes a database. The visible difference is the one that matters:
    a character created in one run is still there in the next.

    Everything else is deliberately identical to the run that is known to work - the same
    client patches, the same launch mode, the same exit forensics - so that if a run goes
    wrong, the server is the only thing that changed.

    THE CLIENT PATCHES ARE STILL PATCHES. Nothing here makes the session valid:

      1415db360:ret     skip the server-reachability check. NOT OPTIONAL: without it the
                        client __fastfails after ~37 seconds, because that check overruns
                        its own stack buffer when every address is unreachable. A client
                        bug, on a path that only ever runs because we are firewalled.
      141b2a280:rdx=0   suppress the "trouble logging in" dialog, which otherwise blocks
                        the per-frame tick that enables the Login button.
      mode=2            leave the client's mode-5 auto-login so the button gets a turn.
      create=on         set the protected flag that gates "Create a character". The real
                        service sets this from virtualised code and we have not found the
                        packet that does it.

    The last two are standing in for protocol we have not implemented. See
    docs/launcher.md for which patches retire and when.

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1"
    powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -Stop
#>
[CmdletBinding()]
param(
    [switch]$Stop,
    # Show what is stored without launching anything. Cheap, and it answers "did the last
    # run actually save the character?" without spending a client launch on it.
    [switch]$ListOnly,
    [int]$Port = 8484,
    # The channel server's port. A channel is a separate process: the login server hands
    # the client this address and the client connects to it, so nothing enters the world
    # unless maplecw-world is listening here.
    [int]$ChannelPort = 8485,
    # The account every connection is served as. The game socket carries no credentials,
    # so this is not a login - it decides whose characters appear. Create it first with
    #   .\target\release\maplecw-useradd.exe <name>
    [string]$Account = 'maplecw',
    # What the login screen displays. Server-supplied; the client cannot compute it.
    [string]$DisplayName = 'wisp****@example.com',
    [string]$Database,
    [string]$World = 'Scania',
    # REVERTED 2026-08-19. Dropping mode=2 was tried and the client crashed with an
    # access violation at 0x141177f4a right after the second 0x000B, before the login
    # screen. Server bytes were identical to the known-good run up to the login result, so
    # the regression is client-side: either this or the 141804870 watch, and both changed
    # at once. Back to the configuration that reached character select and migrated.
    #
    # mode=2 still does not do what it was added for - it is applied on 0x0000 dispatch,
    # our reply to the login request, so it lands after the auto-login it should prevent -
    # but it also routes 0x000B to the classic handler, and THAT is load-bearing. Change
    # one of these at a time, not both.
    [string]$Session = 'mode=2,create=on',
    # Four watch slots, all used.
    #
    #   1415db360:ret     skip the reachability check - without it the client __fastfails
    #   141b2a280:rdx=0   suppress the login dialog that blocks the Login button's tick
    #   142ef3e44:hits=8  the /GS site, so a second stack overflow would still be visible
    #   141b36a10:peek=1c0  the create-result handler, reading stage+0x1c0
    #
    # The last slot is the open question. FUN_141b36a10 handles 0x0015 and compares the
    # world id we send against stage+0x1c0; if they differ it returns having registered
    # nothing and shown nothing, which is indistinguishable from the packet never arriving.
    # On 2026-08-18 the client dispatched a successful 0x0015 and stayed on the creation
    # screen, so this reads the value it compared against. It replaces the watch on
    # 142e9ebd0 (the virtualised routine), which should never be entered now that the
    # reachability check is skipped, and was not entered on the last run.
    # 1415db360:ret and 141b2a280:rdx=0 are mandatory - without them the client dies at
    # ~37s and the "trouble connecting" dialog blocks the screen.
    #
    # The 141804870 watch is GONE. It was armed to name which handshake gate raises "the
    # client is outdated", and it never fired - but the client's own uploaded error log
    # (0x0090) carries a full call stack that names the site for free, so the watch bought
    # nothing and was one of two suspects for the crash. `python tools/decode_elog.py`.
    #
    # 141b36f60 is the migration handler; 142ef3e44 is __report_gsfailure, kept because a
    # silent 37s death is the failure mode this project spends the most runs on.
    [string]$Probe = 'watch@1415db360:ret,141b2a280:rdx=0,141b36f60,142ef3e44:hits=8',
    [string]$SessionTokens = '',
    # Answer the migration hello with the fixed head of a SetField, and swap the probe for
    # the two watches that make the answer readable. See research/msexe-stage-setfield.md.
    #
    # This CANNOT put a character in a map - characterData is 0 and the branch that carries
    # a character needs an 18525-byte record decoder nobody has read. It answers exactly one
    # question: does 0x01A0 reach FUN_142097f80? Both of that handler's early returns are
    # silent, so without the watches the run cannot tell an ignored packet from one that
    # never arrived, which is the whole reason for spending the launch.
    [switch]$SetFieldProbe,
    [string]$ClientDir
)

$ErrorActionPreference = 'Stop'

# -SetFieldProbe swaps two of the four watch slots. The other two are not negotiable:
# 1415db360:ret and 141b2a280:rdx=0 keep the client alive and unblocked, and dropping
# either kills it at ~37s. What goes is 141b36f60, the migration handler - the channel's
# own log already proves the migration, because the client connects and sends 0x007D - and
# 142ef3e44, __report_gsfailure, whose failure mode is still visible in client-exit.log as
# an exit at ~37s, just less precisely.
#
# The two free slots moved on 2026-08-19. 142097f80 and 142cfb500 have both already done
# their job - the run of that day caught SetField entering its handler while dispatching
# 0x01A0, with the latch measured at 0x00 - so re-arming them would re-answer a settled
# question. The open question is now one step further in:
#
#   140304b20  the character-record decoder. Fires if SetField reached the record branch.
#   140302e30  the character-STAT decoder, the block presence[0] is supposed to switch on.
#
# Those two discriminate cleanly, which is the point:
#   neither fires        -> SetField never reached the record path
#   only 140304b20       -> the record decoded but the gate SKIPPED the stat block, i.e.
#                           presence[0] is the wrong byte
#   both                 -> the gate fired and the stats decoded; anything still wrong is
#                           downstream of the map id, not the presence array
#
# 140302e30 also has a built-in positive control. It has exactly two callers in the whole
# image (python tools/callers.py 0x140302e30): this record path, and the character-LIST
# path. So it should fire once at character select, on the login connection, well before
# the migration. If it fires there and NOT after SetField, the probe is demonstrably armed
# and working and the gate genuinely did not open - a silent negative that means something,
# which is the thing this project keeps having to prove the hard way.
#
# An explicit -Probe still wins, so a run can be aimed somewhere else without editing this.
if ($SetFieldProbe -and -not $PSBoundParameters.ContainsKey('Probe')) {
    $Probe = 'watch@1415db360:ret,141b2a280:rdx=0,140304b20,140302e30'
}

$here = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
$root = Split-Path -Parent $here
if (-not $ClientDir) { $ClientDir = Join-Path $root 'client-patched' }
if (-not $Database) { $Database = Join-Path $root 'maplecw.db' }
$exe = Join-Path $ClientDir 'MapleStory.exe'
$loginExe = Join-Path $root 'target\release\maplecw-login.exe'
$worldExe = Join-Path $root 'target\release\maplecw-world.exe'
$userAdd = Join-Path $root 'target\release\maplecw-useradd.exe'
$serverLog = Join-Path $root 'login.log'
$worldLog = Join-Path $root 'world.log'

function Stop-All {
    # Never pipe a native command's stderr under PowerShell 5.1: it wraps each line in an
    # ErrorRecord, which $ErrorActionPreference='Stop' then treats as fatal, so a taskkill
    # reporting "process not found" - the normal case - would abort the script.
    $prev = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        # The client ignores Stop-Process (Themida); taskkill is the one that works.
        if (Get-Process MapleStory -ErrorAction SilentlyContinue) {
            taskkill /F /IM MapleStory.exe /T | Out-Null
        }
        if (Get-Process maplecw-login -ErrorAction SilentlyContinue) {
            taskkill /F /IM maplecw-login.exe | Out-Null
        }
        if (Get-Process maplecw-world -ErrorAction SilentlyContinue) {
            taskkill /F /IM maplecw-world.exe | Out-Null
        }
    } finally {
        $ErrorActionPreference = $prev
    }
}

if ($Stop) {
    Stop-All
    # Leave no marker behind: a stale probe marker would silently turn the next ordinary
    # run into an instrumented one.
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.probe') -ErrorAction SilentlyContinue
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.enable') -ErrorAction SilentlyContinue
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.session') -ErrorAction SilentlyContinue
    Write-Host 'stopped client and login server'
    exit 0
}

Push-Location $root
try {
    & cargo build --release -p login -p store -p grap-stub
    if ($LASTEXITCODE -ne 0) { throw 'build failed' }
}
finally { Pop-Location }

# The account has to exist. Creating one here would mean inventing a password, and this
# repo does not do that - it prompts, and it hashes.
if (-not (Test-Path $Database)) {
    Write-Host ''
    Write-Host "No database at $Database. Create the account first:"
    Write-Host "  $userAdd --db `"$Database`" $Account"
    throw 'no database'
}

if ($ListOnly) {
    & $loginExe --list --db $Database --account $Account --world $World
    exit $LASTEXITCODE
}

# Rebuilding grap-stub does not update the client: cargo writes target/release/grap64.dll
# and the client loads client-patched/grap64.dll. Skipping this is the most expensive kind
# of failure here, because the run looks normal and the new hook code simply is not there.
& powershell -ExecutionPolicy Bypass -File (Join-Path $here 'setup-client.ps1')
if ($LASTEXITCODE -ne 0) { throw 'setup-client.ps1 failed - the client would run the old hook' }

Stop-All
Start-Sleep -Milliseconds 300

Remove-Item $serverLog -Force -ErrorAction SilentlyContinue
$server = Start-Process -FilePath $loginExe -WorkingDirectory $root -PassThru `
    -WindowStyle Hidden `
    -ArgumentList @(
        '--db', "`"$Database`"", '--bind', "127.0.0.1:$Port",
        # One address per channel. The client connects to this when it enters the world,
        # so it must be reachable from the *client* machine - loopback here, a LAN address
        # once the server moves to the homelab.
        '--channels', "127.0.0.1:$ChannelPort",
        '--account', $Account, '--display-name', "`"$DisplayName`"", '--world', $World
    ) `
    -RedirectStandardOutput $serverLog -RedirectStandardError "$serverLog.err"

Remove-Item $worldLog -Force -ErrorAction SilentlyContinue
$worldArgs = @('--db', "`"$Database`"", '--bind', "127.0.0.1:$ChannelPort", '--channel', '0')
if ($SetFieldProbe) { $worldArgs += '--set-field-probe' }
$worldSrv = Start-Process -FilePath $worldExe -WorkingDirectory $root -PassThru `
    -WindowStyle Hidden `
    -ArgumentList $worldArgs `
    -RedirectStandardOutput $worldLog -RedirectStandardError "$worldLog.err"
Write-Host "channel 0 on 127.0.0.1:$ChannelPort (pid $($worldSrv.Id)), log $worldLog"

# Never launch the client against a dead server. A server that exited - a missing account
# is the usual reason - leaves the client on "Connecting..." forever, which looks like a
# client problem and is not.
Start-Sleep -Milliseconds 700
if ($server.HasExited) {
    Write-Host ''
    Write-Host '--- the login server exited immediately ---'
    if (Test-Path "$serverLog.err") { Get-Content "$serverLog.err" | Write-Host }
    if (Test-Path $serverLog) { Get-Content $serverLog | Write-Host }
    throw 'the login server did not start'
}
Write-Host "login server pid $($server.Id) -> $serverLog"
Get-Content $serverLog -ErrorAction SilentlyContinue | ForEach-Object { Write-Host "  $_" }

# The hook is switched on by marker files, because ShellExecute does not carry $env: into
# the child - a launcher will replace all of this with one config file (docs/launcher.md).
$hookLog = Join-Path $ClientDir 'maplecw-hook.log'
New-Item -ItemType File -Path (Join-Path $ClientDir 'maplecw-hook.enable') -Force | Out-Null
Remove-Item $hookLog -Force -ErrorAction SilentlyContinue
$env:MAPLECW_HOOK_LOG = $hookLog
Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.probe') -Value $Probe -Encoding ascii
Set-Content -Path (Join-Path $ClientDir 'maplecw-hook.session') -Value $Session -Encoding ascii
Write-Host "client patches: $Probe"
Write-Host "session patches: $Session"
if ($SetFieldProbe) {
    Write-Host ""
    Write-Host "SET-FIELD RUN. The client is answered with a FULL character record:" -ForegroundColor Cyan
    Write-Host "  characterData=1, presence[0] set so the character-stat block decodes,"
    Write-Host "  and that block carries the character's real map id."
    Write-Host ""
    Write-Host "  ONE thing changed since the run that faded to black: the record now" -ForegroundColor Cyan
    Write-Host "  switches the stat block on and carries a map. Nothing else moved."
    Write-Host ""
    Write-Host "In client-patched\maplecw-hook.log, two WATCH lines decide this:"
    Write-Host "  no WATCH lines at all        the hook never armed. Re-run; conclude nothing."
    Write-Host "  140302e30 at CHARACTER SELECT is the positive control - it means the probe"
    Write-Host "                               is armed and working, whatever happens later."
    Write-Host "  140304b20 but NOT 140302e30  the record decoded and the gate SKIPPED the"
    Write-Host "                               stat block: presence[0] is the wrong byte."
    Write-Host "  BOTH, after the migration    the gate opened and the stats decoded."
    Write-Host ""
    Write-Host "Then: is there a CLIENT FAULT line?" -ForegroundColor Cyan
    Write-Host "  fault at 140ce89d6   unchanged from last run - the map still did not load."
    Write-Host "  fault ELSEWHERE      it got further; the address says how far."
    Write-Host "  NO fault at all      the record was accepted end to end."
    Write-Host ""
    Write-Host "  This run CAN put a character on screen. That is the thing to watch for." -ForegroundColor Yellow
    Write-Host "  Unlike every previous run, 'nothing visible' is now a FAILURE, not the" -ForegroundColor Yellow
    Write-Host "  expected result." -ForegroundColor Yellow
}

# ShellExecute is required: the client has an elevation manifest, and CreateProcess fails
# with "requires elevation".
$launchArgs = @('-NXLDEBUG', '127.0.0.1', "$Port")
if ($SessionTokens) {
    $launchArgs += ($SessionTokens -split '\s+' | Where-Object { $_ })
    Write-Host "session tokens (config +0x90): $SessionTokens"
    Write-Host '  -> read login.log for the 0x0073 body and look for them'
}
$p = Start-Process -FilePath $exe -WorkingDirectory $ClientDir `
    -ArgumentList $launchArgs -PassThru

$exitLog = Join-Path $root 'client-exit.log'
Remove-Item $exitLog -ErrorAction SilentlyContinue
Start-Process -FilePath 'powershell' -WindowStyle Hidden -ArgumentList @(
    '-ExecutionPolicy', 'Bypass', '-File', "`"$(Join-Path $here 'exit-forensics.ps1')`"",
    '-ClientPid', $p.Id, '-Log', "`"$exitLog`""
) | Out-Null

# Keep the host usable while a dialog is being read; the client otherwise saturates it.
try {
    $p.PriorityClass = 'BelowNormal'
    $cores = [Environment]::ProcessorCount
    if ($cores -gt 2) {
        $p.ProcessorAffinity = [IntPtr](([long][Math]::Pow(2, $cores) - 1) -band -bnot 1)
    }
} catch {
    Write-Host "could not lower client priority: $_"
}

# Read the command line back off the running process rather than trusting what we meant to
# pass. A parameter that never arrived looks exactly like one that arrived and did nothing.
$actual = (Get-CimInstance Win32_Process -Filter "ProcessId = $($p.Id)" -ErrorAction SilentlyContinue).CommandLine
if ($actual) { Write-Host "launched: $actual" } else { Write-Host 'launched: (command line unreadable)' }

Write-Host ''
Write-Host 'On screen:'
if ($SetFieldProbe) {
    Write-Host '  1. wait out the splash, then click Login.'
    Write-Host '  2. pick an existing character and enter the world.'
    Write-Host '  3. you should hear the enter-success sound, then the screen fades to black.'
    Write-Host '     Both of those already happened last run - they are not the news.'
    Write-Host '  4. WAIT about 40 seconds without touching anything. Last run the client'
    Write-Host '     faulted about 3.4s after the fade, so the interesting window is short,'
    Write-Host '     but let it sit in case it survives.'
    Write-Host ''
    Write-Host '  The question this run answers: does the map LOAD?' -ForegroundColor Cyan
    Write-Host '    - a character standing on a map, however broken it looks - that is the'
    Write-Host '      goal, and anything at all drawn after the fade counts. Say what you see:'
    Write-Host '      terrain, a character sprite, a UI bar, a loading bar, anything.'
    Write-Host '    - black screen then exit, same as last time - the map still did not load.'
    Write-Host '    - any dialog: report the exact wording, it names the failure.'
    Write-Host '    - a freeze with the UI unresponsive means an unanswered packet, NOT a'
    Write-Host '      crash - read world.log for the last inbound line with nothing after it.'
    Write-Host ''
    Write-Host '  Something visible after the fade is now the target, not a surprise.' -ForegroundColor Yellow
} else {
    Write-Host '  1. click Login. Any character created in an EARLIER run should be there.'
    Write-Host '  2. create one. Check the name first - a name already used is now refused'
    Write-Host '     by the server rather than always accepted.'
    Write-Host '  3. close the client, run this script again, and click Login. The character'
    Write-Host '     should still be listed. That is the whole point of this run.'
}
Write-Host ''
Write-Host 'Logs:'
Write-Host "  $serverLog                 every packet both ways, and what each reply was"
Write-Host "  $hookLog   client patches and faults"
Write-Host "  $exitLog          how the client died; 0 is a hand-close"
Write-Host ''
Write-Host "Then: powershell -ExecutionPolicy Bypass -File `"$PSCommandPath`" -Stop"
