<#
.SYNOPSIS
    Record every socket the patched client opens, for its whole lifetime.

.DESCRIPTION
    Answers one question: after login, does the client connect to a SECOND endpoint?

    In MapleStory the login server and the channel server are normally different ports, so
    a migration shows up as a new socket to a different port. If no second socket ever
    appears, the client is not migrating and the connection close is just its own timeout -
    which changes what needs building next.

    This exists instead of `client-sockets.ps1` because that samples three times on demand,
    and the interesting window here is a few seconds long while someone is busy clicking
    through a login screen. Racing that by hand loses. This polls from before the client
    starts until after it exits, and logs only changes, so the whole lifetime fits on a
    page.

    Started automatically by `test-one.ps1 -Sockets`; there is no need to run it in a
    second shell.

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File tools\watch-sockets.ps1 -Out sockets.log
#>
[CmdletBinding()]
param(
    [string]$Out = 'sockets.log',
    # How long to wait for the client to appear before giving up.
    [int]$WaitSeconds = 60,
    [int]$IntervalMs = 250,
    [int]$ProbePort = 8484
)

# maplecw-sockets  <- marker so test-one.ps1 can find and stop this process by command line

$ErrorActionPreference = 'Continue'
function Note([string]$msg) {
    $line = "[{0:HH:mm:ss.fff}] {1}" -f (Get-Date), $msg
    Add-Content -Path $Out -Value $line -Encoding utf8
}

Set-Content -Path $Out -Value '' -Encoding utf8
Note 'waiting for MapleStory.exe'

$deadline = (Get-Date).AddSeconds($WaitSeconds)
$procs = $null
while ((Get-Date) -lt $deadline) {
    $procs = Get-Process MapleStory -ErrorAction SilentlyContinue
    if ($procs) { break }
    Start-Sleep -Milliseconds $IntervalMs
}
if (-not $procs) { Note 'client never appeared'; return }

$ids = @($procs.Id)
Note ("client pid(s) " + ($ids -join ', '))

$last = ''
while ($true) {
    if (-not (Get-Process -Id $ids -ErrorAction SilentlyContinue)) {
        Note 'client exited'
        break
    }

    # Both endpoints, not just the client's. The TCP *state* of each half is what says who
    # closed first, and that is now the open question:
    #
    #   client in CloseWait      -> our end sent FIN first
    #   client in FinWait1/2     -> the client sent FIN first
    #   the pair vanishes with no intermediate state -> a reset
    #
    # pktmon cannot answer this because it never sees loopback traffic at all, so this poll
    # is the only view of the connection's TCP state we have.
    $conns = Get-NetTCPConnection -ErrorAction SilentlyContinue |
        Where-Object {
            $ids -contains $_.OwningProcess -or
            $_.LocalPort -eq $ProbePort -or $_.RemotePort -eq $ProbePort
        } |
        Sort-Object LocalPort, RemotePort

    $now = ($conns | ForEach-Object {
        "$($_.State) $($_.LocalAddress):$($_.LocalPort) -> $($_.RemoteAddress):$($_.RemotePort)"
    }) -join ' | '

    # Only on change: a 250ms poll over a 30s run would otherwise be 120 identical blocks.
    if ($now -ne $last) {
        if (-not $now) {
            Note 'no TCP sockets owned by the client'
        } else {
            Note 'sockets:'
            foreach ($c in $conns) {
                $remote = "$($c.RemoteAddress):$($c.RemotePort)"
                $tag = ''
                # Anything off the probe port is the migration this script was written to
                # detect. The states are the newer question - see the comment above.
                if ($c.RemotePort -ne 0 -and $c.RemotePort -ne $ProbePort -and
                    $c.LocalPort -ne $ProbePort) {
                    $tag = '   <<< NOT our probe port'
                }
                $who = if ($ids -contains $c.OwningProcess) { 'client' } else { 'probe ' }
                Note ("    {0} {1,-12} {2}:{3} -> {4}{5}" -f `
                    $who, $c.State, $c.LocalAddress, $c.LocalPort, $remote, $tag)
            }
        }
        $last = $now
    }
    Start-Sleep -Milliseconds $IntervalMs
}
