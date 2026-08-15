<#
.SYNOPSIS
    Show every socket the patched client has open, while it is running.

.DESCRIPTION
    Tests whether the white-screen hang is actually a *network* wait on something other
    than our probe. The firewall rule "MapleCW - block patched client outbound" blocks the
    client from reaching any external host; Windows Firewall does not filter loopback, so
    our probe on 127.0.0.1 still works while everything outbound is silently dropped.

    If the client is stuck waiting on an external endpoint - an auth service, a CDN, a
    telemetry host - it will hang forever no matter which opcodes we send it, and that
    would explain every null result from the opcode sweeps.

    Run this WHILE the client is hanging.

      SYN_SENT to a non-loopback address  -> it is blocked on an external host
      only ESTABLISHED to 127.0.0.1:8484  -> it is genuinely waiting on our protocol

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\client-sockets.ps1"
#>
[CmdletBinding()]
param([int]$Samples = 3, [int]$IntervalMs = 1500)

$procs = Get-Process MapleStory -ErrorAction SilentlyContinue
if (-not $procs) {
    Write-Host 'MapleStory.exe is not running - start a test first.' -ForegroundColor Yellow
    return
}

foreach ($p in $procs) {
    Write-Host "client pid $($p.Id)  threads=$($p.Threads.Count)  responding=$($p.Responding)"
}
$ids = $procs.Id

for ($i = 1; $i -le $Samples; $i++) {
    Write-Host ""
    Write-Host "--- sample $i of $Samples ---"

    # Get-NetTCPConnection carries the owning PID, which netstat only gives with -o.
    $conns = Get-NetTCPConnection -ErrorAction SilentlyContinue |
        Where-Object { $ids -contains $_.OwningProcess }

    if (-not $conns) {
        Write-Host '  no TCP sockets owned by the client'
    } else {
        $conns |
            Select-Object State,
                @{n = 'Local'; e = { "$($_.LocalAddress):$($_.LocalPort)" } },
                @{n = 'Remote'; e = { "$($_.RemoteAddress):$($_.RemotePort)" } },
                @{n = 'Loopback'; e = {
                    $_.RemoteAddress -eq '127.0.0.1' -or $_.RemoteAddress -eq '::1' -or
                    $_.RemoteAddress -eq '0.0.0.0' -or $_.RemoteAddress -eq '::' } } |
            Sort-Object Loopback, State |
            Format-Table -AutoSize | Out-String | Write-Host

        $external = $conns | Where-Object {
            $_.RemoteAddress -ne '127.0.0.1' -and $_.RemoteAddress -ne '::1' -and
            $_.RemoteAddress -ne '0.0.0.0' -and $_.RemoteAddress -ne '::' }
        if ($external) {
            Write-Host '  *** EXTERNAL socket(s) present - the client is reaching outside ***' -ForegroundColor Red
            foreach ($e in $external) {
                Write-Host "      $($e.State)  ->  $($e.RemoteAddress):$($e.RemotePort)"
            }
            Write-Host '      SYN_SENT here means the firewall is dropping it and the client is stuck.'
        } else {
            Write-Host '  all sockets are loopback - the hang is not an external network wait.'
        }
    }

    if ($i -lt $Samples) { Start-Sleep -Milliseconds $IntervalMs }
}
