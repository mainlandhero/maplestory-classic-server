<#
.SYNOPSIS
    Attach exit forensics to every client, no matter who started it.

.DESCRIPTION
    `client-exit.log` is the file the test plan tells the owner to read FIRST when a client goes
    away. On 2026-09-09 two clients exited after 13.5 minutes and that file was three days
    stale, so there was no exit code, no thread census and no "vanished during the run" line
    for the event we were trying to explain.

    The reason was structural, not a mistake on the night. `test-server.ps1` starts
    `exit-forensics.ps1` in exactly one place - immediately after IT launches the client -
    and attaches it to that one pid. But `-ServersOnly` returns into a wait loop long before
    that code, and `start-servers.cmd` says so in its own header: *"The CLIENT is not started
    here. Double-click maplecw-launcher.exe after this"*. So on the path every launch now
    actually takes, the monitor was never started and the file could never be written.

    That is the same shape as the WER LocalDumps lesson in `CLAUDE.md`: a step in the plan
    that could only ever come back empty, which looks identical to a step that ran and found
    nothing.

    This watches for client processes instead of being handed one. It polls for
    `MapleStory.exe`, and the first time it sees a pid it spawns `exit-forensics.ps1` against
    it. Whoever started the client - the launcher, this script, a double-click - it gets
    covered.

.NOTES
    **One log per pid, and that is not tidiness.** The old single `client-exit.log` was
    deleted at launch and written by one monitor, so with two clients up the second would
    have overwritten the first's answer. `CLAUDE.md` already records this for the hook log -
    "two clients share one hook log, so chairprobe needs --pid". Here each client gets
    `client-exit-<pid>.log` and nothing is ever deleted.

    Started hidden by `test-server.ps1 -ServersOnly` before its wait loop. It exits on its
    own when `-ParentPid` goes away, so closing the server console takes it with it rather
    than leaving a poller behind for the next boot to find.
#>
param(
    [string]$Root = (Split-Path -Parent $PSScriptRoot),
    # The server console. When it goes, so does this.
    [int]$ParentPid = 0,
    [double]$Interval = 1.0,
    # Matched against the process name, without the extension.
    [string]$ClientName = 'MapleStory'
)

$here = $PSScriptRoot
$forensics = Join-Path $here 'exit-forensics.ps1'
if (-not (Test-Path $forensics)) {
    Write-Host "client-exit-watch: $forensics is missing; nothing to attach"
    exit 1
}

# Pids already covered. A pid can be reused by Windows after the process ends, but not while
# the original is alive, and a monitor that has already exited will simply be replaced.
$seen = @{}
$attached = 0

while ($true) {
    if ($ParentPid -gt 0) {
        $parent = Get-Process -Id $ParentPid -ErrorAction SilentlyContinue
        if (-not $parent) { break }
    }

    $clients = @(Get-Process -Name $ClientName -ErrorAction SilentlyContinue)
    foreach ($c in $clients) {
        if ($seen.ContainsKey($c.Id)) { continue }
        $seen[$c.Id] = $true
        $log = Join-Path $Root ("client-exit-{0}.log" -f $c.Id)
        # -PassThru so a monitor that fails to start is visible rather than assumed.
        $m = Start-Process -FilePath 'powershell' -WindowStyle Hidden -PassThru -ArgumentList @(
            '-ExecutionPolicy', 'Bypass', '-File', "`"$forensics`"",
            '-ClientPid', $c.Id, '-Log', "`"$log`""
        )
        if ($m) {
            $attached = $attached + 1
            Write-Host ("client-exit-watch: attached to client pid {0} -> {1} (monitor pid {2})" -f $c.Id, $log, $m.Id)
        } else {
            Write-Host ("client-exit-watch: FAILED to attach to client pid {0} - that client's exit will not be measurable" -f $c.Id)
        }
    }

    Start-Sleep -Seconds $Interval
}

Write-Host ("client-exit-watch: parent {0} is gone, stopping after attaching to {1} client(s)" -f $ParentPid, $attached)
