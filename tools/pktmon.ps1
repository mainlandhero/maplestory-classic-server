<#
.SYNOPSIS
    Capture the loopback traffic between the client and our probe, at packet level.

.DESCRIPTION
    Every user-mode way to end a TCP connection has been watched and ruled out: on the
    game socket there is no closesocket until exit, no shutdown ever, and no
    WSASendDisconnect, WSACleanup or setsockopt at all - on hooks that self-test and are
    read back after planting. The client's socket handle polls VALID throughout. Our own
    probe only exits its loop on a real EOF or reset, and holds for 300s. Yet a TCP reset
    arrives about 0.3s after the client sends 0x007A, every single run.

    That makes it a network-layer event, and pktmon is the tool that can see one. Two
    things it gives us that no API hook can:

      * the TCP flags on the wire, so "reset" stops being an inference from a Python
        exception and becomes an observed RST with a direction;
      * DROP events with a reason and the component that dropped them, which is how a
        filter driver - antivirus, or something Nexon ships - would show up.

    Loopback traffic does not traverse a NIC, so the capture uses `--comp all`.

.EXAMPLE
    # 1. In an ELEVATED shell:
    powershell -ExecutionPolicy Bypass -File tools\pktmon.ps1 -Start

    # 2. In a normal shell, the usual one-variant run:
    powershell -ExecutionPolicy Bypass -File tools\test-one.ps1 ...

    # 3. Back in the elevated shell, once the client has closed:
    powershell -ExecutionPolicy Bypass -File tools\pktmon.ps1 -Stop

.NOTES
    Needs elevation. Captures only TCP on the probe's port, so it records this experiment
    and nothing else on the machine - no browsing, no other applications.
#>
param(
    [switch]$Start,
    [switch]$Stop,
    [int]$Port = 8484,
    [string]$OutDir = (Join-Path $PSScriptRoot '..\research\pktmon')
)

$ErrorActionPreference = 'Stop'

function Assert-Elevated {
    $id = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($id)
    if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
        throw "pktmon needs an elevated shell. Re-run this from 'Run as administrator'."
    }
}

$OutDir = [IO.Path]::GetFullPath($OutDir)
$etl = Join-Path $OutDir 'maplecw.etl'
$txt = Join-Path $OutDir 'maplecw.txt'

if (-not $Start -and -not $Stop) {
    Write-Host "usage: pktmon.ps1 -Start | -Stop  [-Port 8484]"
    Write-Host "  -Start  arm a TCP filter on the port and begin capturing (elevated)"
    Write-Host "  -Stop   stop, convert to text, and summarise the resets"
    exit 2
}

Assert-Elevated
if (-not (Test-Path $OutDir)) { New-Item -ItemType Directory -Force $OutDir | Out-Null }

if ($Start) {
    # A stale filter from an earlier run would silently widen or narrow the capture.
    & pktmon filter remove | Out-Null
    & pktmon filter add "MapleCW" -t TCP -p $Port | Out-Null
    Write-Host "filter: TCP, port $Port only"

    if (Test-Path $etl) { Remove-Item $etl -Force }
    # --comp all because loopback never reaches a NIC; --pkt-size 0 keeps whole packets so
    # the TCP header, and therefore the RST flag, is actually present; --type all so drops
    # are recorded alongside flows, since a drop with a reason is the point of this.
    & pktmon start --capture --comp all --type all --pkt-size 0 --file-name $etl
    Write-Host ""
    Write-Host "capturing to $etl"
    Write-Host "now run test-one.ps1 in a normal shell, then re-run this with -Stop"
    exit 0
}

& pktmon stop | Out-Null
& pktmon filter remove | Out-Null
Write-Host "stopped; filters cleared"

if (-not (Test-Path $etl)) { throw "no capture at $etl - was -Start run?" }

if (Test-Path $txt) { Remove-Item $txt -Force }
& pktmon etl2txt $etl --out $txt --verbose 5 | Out-Null
Write-Host "wrote $txt"

# A summary, so the interesting lines do not have to be found by eye in a large file.
# Resets and drops are the whole question; everything else is the connection working.
$lines = Get-Content $txt
$resets = $lines | Select-String -Pattern 'RST' -SimpleMatch
$drops = $lines | Select-String -Pattern 'Drop' -SimpleMatch
Write-Host ""
Write-Host ("packets logged : {0}" -f $lines.Count)
Write-Host ("lines with RST : {0}" -f $resets.Count)
Write-Host ("lines with Drop: {0}" -f $drops.Count)
if ($resets.Count) {
    Write-Host ""
    Write-Host "--- resets ---"
    $resets | Select-Object -First 20 | ForEach-Object { $_.Line }
}
if ($drops.Count) {
    Write-Host ""
    Write-Host "--- drops (reason and component are what matter) ---"
    $drops | Select-Object -First 20 | ForEach-Object { $_.Line }
}
