<#
.SYNOPSIS
    Run a single handshake variant against the patched client, then clean up.

.DESCRIPTION
    The decisive signal is the client's on-screen dialog, which nothing here can read,
    so tests are run ONE variant at a time and the result is read off the screen.

    This starts the probe, launches the client, and leaves both running so the dialog
    can be looked at. Run with -Stop when finished to tear everything down.

    The client is started at BelowNormal priority and kept off one CPU core. It renders
    an uncapped login scene and will otherwise saturate the machine while a test sits
    waiting for someone to read a dialog box.

.EXAMPLE
    tools\test-one.ps1 -Variant 0
    tools\test-one.ps1 -Stop
#>
[CmdletBinding()]
param(
    [int]$Variant = 0,
    [switch]$Stop,
    [switch]$List,
    [switch]$Normal,
    [string]$ClientDir,
    [int]$Port = 8484
)

$ErrorActionPreference = 'Stop'
$here = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
$root = Split-Path -Parent $here
if (-not $ClientDir) { $ClientDir = Join-Path $root 'client-patched' }
$exe = Join-Path $ClientDir 'MapleStory.exe'
$probe = Join-Path $here 'handshake_probe.py'
$logFile = Join-Path $root 'probe.log'

function Stop-All {
    # The client ignores Stop-Process (Themida); taskkill is the one that works.
    #
    # Never pipe a native command's stderr here. Under PowerShell 5.1, `2>&1` on an exe
    # wraps each stderr line in an ErrorRecord, which $ErrorActionPreference='Stop' then
    # treats as fatal - so taskkill reporting "process not found" (the normal case on a
    # clean start) would abort the script before it launched anything.
    $prev = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        if (Get-Process MapleStory -ErrorAction SilentlyContinue) {
            taskkill /F /IM MapleStory.exe /T | Out-Null
        }
        Get-CimInstance Win32_Process -Filter "Name like '%python%'" -ErrorAction SilentlyContinue |
            Where-Object { $_.CommandLine -like '*handshake_probe*' } |
            ForEach-Object { taskkill /F /PID $_.ProcessId | Out-Null }
    } finally {
        $ErrorActionPreference = $prev
    }
}

if ($List) { & python $probe --list; return }

if ($Stop) {
    Stop-All
    Write-Host 'stopped client and probe'
    if (Test-Path $logFile) { Write-Host "--- $logFile ---"; Get-Content $logFile }
    return
}

if (-not (Test-Path $exe)) { throw "not found: $exe" }

# Never leave a previous run's client or probe holding the port.
Stop-All
Start-Sleep -Milliseconds 600

$name = (& python $probe --list | Where-Object { $_ -match "^\s*$Variant\s" })
Write-Host "variant $Variant :$name"

Start-Process -FilePath 'python' `
    -ArgumentList @('-u', $probe, '--port', "$Port", '--only', "$Variant") `
    -WorkingDirectory $root `
    -RedirectStandardOutput $logFile `
    -RedirectStandardError (Join-Path $root 'probe.err') `
    -WindowStyle Hidden | Out-Null
Start-Sleep -Milliseconds 800

$p = Start-Process -FilePath $exe -WorkingDirectory $ClientDir `
    -ArgumentList @('-NXLDEBUG', '127.0.0.1', "$Port") -PassThru

# Keep the host usable while the dialog is being read. -Normal skips this and runs the
# client exactly as Windows would start it, in case the throttling ever looks like it is
# affecting behaviour (timing-sensitive protection checks, for instance).
if (-not $Normal) {
    try {
        $p.PriorityClass = 'BelowNormal'
        $cores = [Environment]::ProcessorCount
        if ($cores -gt 2) {
            # Leave core 0 free for everything else.
            $p.ProcessorAffinity = [IntPtr](([long][Math]::Pow(2, $cores) - 1) -band -bnot 1)
        }
    } catch {
        Write-Host "could not lower client priority: $_"
    }
}

Write-Host "client pid $($p.Id) launched; read the dialog, then run: tools\test-one.ps1 -Stop"
