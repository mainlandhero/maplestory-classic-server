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
    Running scripts is disabled on this machine, so invoke it through -File with a
    per-process bypass (this changes no machine setting):

    powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-one.ps1" -Variant 0 -Normal
    powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-one.ps1" -Stop
#>
[CmdletBinding()]
param(
    [int]$Variant = 0,
    [switch]$Stop,
    [switch]$List,
    [switch]$Normal,
    [ValidateSet('header', 'ping', 'sweep')]
    [string]$Reply,
    [string]$Opcode = '0xFFFF',
    [string]$SweepFrom = '0x0000',
    [string]$SweepTo = '0x1000',
    [double]$SweepDelay = 0.15,
    [int]$Pad = 0,
    [string]$Skip = '',
    [string]$PingFirst = '',
    [double]$PingWait = 10,
    [double]$QuietBefore = 5,
    [string]$HookLog = '',
    # Hex range like '0000-1000'. Walks the inbound opcode space in-process rather than
    # sending packets, which is bounded by the client's tolerance for unknown opcodes.
    [string]$Probe = '',
    # Extra launch tokens after <ip> <port>. -NXLDEBUG routes tokens 3 onward into the
    # config's six-slot session array at +0x90, which is what the launcher normally fills.
    [string[]]$SessionTokens = @(),
    # Hex body for the replied/swept opcode, e.g. '650000' for the login result
    # (u8 result 0x65, then a u16-length empty string).
    [string]$Body = '',
    [string]$PingBody = '',
    # Client opcode to answer the instant it arrives, e.g. 0x0080.
    [string]$ReplyTo = '',
    [string]$ClientDir,
    [int]$Port = 8484
)

$ErrorActionPreference = 'Stop'
$here = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
$root = Split-Path -Parent $here
if (-not $ClientDir) { $ClientDir = Join-Path $root 'client-patched' }
$exe = Join-Path $ClientDir 'MapleStory.exe'
# NOT $probe: PowerShell variable names are case-insensitive, so that would
# collide with the -Probe parameter and silently overwrite it.
$probePy = Join-Path $here 'handshake_probe.py'
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

if ($List) { & python $probePy --list; return }

if ($Stop) {
    Stop-All
    # Clear the walk marker here too. It is a one-shot experiment, and leaving it behind
    # would silently turn the next ordinary run into a probe.
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.probe') -ErrorAction SilentlyContinue
    # taskkill cannot touch the elevated client from a normal shell, and it says so on
    # stderr where it is easy to miss. Check rather than claim success: with the probe's
    # ExitProcess detour in place the client no longer dies on its own, so a survivor
    # here will hold port 8484 and quietly break the next run.
    Start-Sleep -Milliseconds 400
    if (Get-Process MapleStory -ErrorAction SilentlyContinue) {
        Write-Host ''
        Write-Host 'CLIENT IS STILL RUNNING - taskkill was denied (it runs elevated).' -ForegroundColor Red
        Write-Host 'Kill it from an elevated shell, or the next run will fight it for the port:'
        Write-Host '    taskkill /F /IM MapleStory.exe /T'
        Write-Host ''
    } else {
        Write-Host 'stopped client and probe'
    }
    if (Test-Path $logFile) { Write-Host "--- $logFile ---"; Get-Content $logFile }
    return
}

if (-not (Test-Path $exe)) { throw "not found: $exe" }

# Never leave a previous run's client or probe holding the port.
Stop-All
Start-Sleep -Milliseconds 600

$name = (& python $probePy --list | Where-Object { $_ -match "^\s*$Variant\s" })
Write-Host "variant $Variant :$name"

$probeArgs = @('-u', $probePy, '--port', "$Port", '--only', "$Variant")
if ($Reply) {
    $probeArgs += @('--reply', $Reply, '--opcode', $Opcode, '--pad', "$Pad",
                    '--quiet-before', "$QuietBefore")
    # These apply to every reply mode, not just sweep. Gating them on sweep meant a
    # -ReplyTo run silently dropped its gate packet and its body.
    if ($Body) { $probeArgs += @('--body', $Body) }
    if ($PingBody) { $probeArgs += @('--ping-body', $PingBody) }
    if ($ReplyTo) { $probeArgs += @('--reply-to', $ReplyTo) }
    if ($PingFirst) {
        $probeArgs += @('--ping-first', $PingFirst, '--ping-wait', "$PingWait")
    }
    if ($Reply -eq 'sweep') {
        $probeArgs += @('--sweep-from', $SweepFrom, '--sweep-to', $SweepTo,
                        '--sweep-delay', "$SweepDelay")
        if ($Skip) { $probeArgs += @('--skip', $Skip) }
        Write-Host "will sweep opcodes $SweepFrom..$SweepTo at ${SweepDelay}s each (pad $Pad)"
    }
    if ($ReplyTo) {
        Write-Host "will send $PingFirst as the gate, then answer $ReplyTo with $Opcode body=$Body"
    } elseif ($Reply -ne 'sweep') {
        Write-Host "will answer the client with: $Reply"
    }
}

Start-Process -FilePath 'python' `
    -ArgumentList $probeArgs `
    -WorkingDirectory $root `
    -RedirectStandardOutput $logFile `
    -RedirectStandardError (Join-Path $root 'probe.err') `
    -WindowStyle Hidden | Out-Null
Start-Sleep -Milliseconds 1200

# Never launch the client against a dead probe. A crashed serve thread closes the socket,
# and the client then drops because the server vanished - which reads exactly like the
# client rejecting our packet. That cost a full test cycle once; check instead.
$errFile = Join-Path $root 'probe.err'
$probeAlive = Get-CimInstance Win32_Process -Filter "Name like '%python%'" -ErrorAction SilentlyContinue |
    Where-Object { $_.CommandLine -like '*handshake_probe*' }
$errText = if (Test-Path $errFile) { (Get-Content $errFile -Raw) } else { '' }
if ((-not $probeAlive) -or ($errText -and $errText.Trim())) {
    Write-Host ''
    Write-Host 'PROBE DID NOT START CLEANLY - not launching the client.' -ForegroundColor Red
    if ($errText.Trim()) { Write-Host $errText.Trim() }
    if (Test-Path $logFile) { Get-Content $logFile | Select-Object -Last 5 }
    Stop-All
    return
}

# The dispatcher hook lives in our grap64.dll stub, which the client loads itself, so
# enabling it is just an environment variable the child process inherits.
if ($HookLog) {
    if (Test-Path $HookLog) { Remove-Item $HookLog -Force }
    # Marker file beside the client, read by the hook at startup.
    New-Item -ItemType File -Path (Join-Path $ClientDir 'maplecw-hook.enable') -Force | Out-Null
    # The DLL appends, and it writes beside the client rather than to -HookLog when the
    # env var does not propagate. Clear it, or two runs concatenate and the second looks
    # like a continuation of the first - which already caused one wrong conclusion.
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.log') -ErrorAction SilentlyContinue
    $env:MAPLECW_HOOK_LOG = $HookLog
    # Also enable the stub's own call log, next to it. If neither file appears, the DLL
    # is not running our code at all; if only this one does, the hook install is at fault.
    $env:GRAP_STUB_LOG = "$HookLog.stub"
    if (Test-Path $env:GRAP_STUB_LOG) { Remove-Item $env:GRAP_STUB_LOG -Force }
    Write-Host "dispatcher hook enabled -> $HookLog (stub log -> $HookLog.stub)"
} else {
    Remove-Item Env:\MAPLECW_HOOK_LOG -ErrorAction SilentlyContinue
    Remove-Item (Join-Path $ClientDir 'maplecw-hook.enable') -ErrorAction SilentlyContinue
}

# -Probe walks the opcode space from inside the client instead of over the wire. The
# marker holds the range; the hook reads it once, on the first packet it sees.
$probeMarker = Join-Path $ClientDir 'maplecw-hook.probe'
if ($Probe) {
    if (-not $HookLog) { throw '-Probe needs -HookLog: the walk reports through the hook log.' }
    Set-Content -Path $probeMarker -Value $Probe -Encoding ascii
    Write-Host "in-process opcode walk enabled, range $Probe"
} else {
    Remove-Item $probeMarker -ErrorAction SilentlyContinue
}

# -NXLDEBUG puts token 1 at the server IP, token 2 at the port, and **tokens 3 onward
# into config +0x90** - the same six-slot session array WEBSTART fills from tokens 4-9
# (docs/launch-protocol.md). We have always launched with none of them, so that array is
# empty, and the live client's login screen shows a pre-filled account ID where ours shows
# nothing. Passing six distinguishable tokens maps slot -> on-screen field in one launch.
$launchArgs = @('-NXLDEBUG', '127.0.0.1', "$Port")
if ($SessionTokens) {
    $launchArgs += $SessionTokens
    Write-Host ("session tokens (config +0x90): " + ($SessionTokens -join ' '))
}

# Start-Process (ShellExecute) is required: the client has an elevation manifest, and
# CreateProcess - which is what UseShellExecute = false uses - fails with "requires
# elevation". ShellExecute in turn will not carry $env: into the child, which is why the
# hook is switched on by a marker file instead.
$p = Start-Process -FilePath $exe -WorkingDirectory $ClientDir `
    -ArgumentList $launchArgs -PassThru

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

# Read the command line back off the running process rather than trusting what we meant
# to pass. A run where the session tokens silently failed to arrive looks exactly like a
# run where they arrived and did nothing, and those call for opposite next steps.
$actual = (Get-CimInstance Win32_Process -Filter "ProcessId = $($p.Id)" -ErrorAction SilentlyContinue).CommandLine
if ($actual) { Write-Host "launched: $actual" }
else { Write-Host 'launched: (could not read the command line back)' }

Write-Host "client pid $($p.Id) launched. When done, read the screen, then run:"
Write-Host "  powershell -ExecutionPolicy Bypass -File `"$PSCommandPath`" -Stop"
