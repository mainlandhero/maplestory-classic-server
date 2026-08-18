<#
.SYNOPSIS
    Record how the client dies, from outside the process.

.DESCRIPTION
    The ~25 second exit is the priority, and five explanations are already ruled out from
    inside the process by verified instruments: an inbound idle timeout (keepalives flowed
    and it still died), RtlExitUserProcess, NtTerminateProcess, any fault a vectored
    handler can see, and a crash or __fastfail - the Windows Application log holds no error
    for these exits, and that log is known to work because it holds a real MapleStory
    0xc0000005 from 2026-08-14.

    Two explanations survive:

      1. another process calls TerminateProcess on the client - no client code runs at all,
         so no in-process breakpoint can ever fire;
      2. the last thread ends, and the process goes with it - RtlExitUserThread on the last
         thread reaches NtTerminateThread, and the kernel ends the process from there
         without passing through RtlExitUserProcess, which is why ruling that out did not
         rule this out.

    THE DECISIVE MEASUREMENT IS NOT IN THIS SCRIPT. It is the pair of watches on
    ntdll!RtlExitUserThread and ntdll!NtTerminateThread in maplecw-hook.log: those fire on
    path 2 and cannot fire on path 1. What this adds is everything about the death that can
    only be seen from outside, and the name of a suspect if it turns out to be path 1.

    A discarded discriminator, recorded so it is not tried again. "An orderly shutdown
    drains threads, an external kill does not" is false. Measured on 2026-08-17 with two
    control processes: one exited normally with code 42, one was killed with
    TerminateProcess, and both showed 24 live threads in the last sample before they
    vanished. ExitProcess ends every other thread in the kernel, running no user code and
    taking no measurable time, so no sample rate separates them. The thread table is still
    logged - it is data about the client's shape over its life - but it is not a verdict.

    What this does measure:

      * the exit code, which those same controls showed does separate them: the killed one
        came back 0xFFFFFFFF, the orderly one 0x0000002A, the value it chose itself;
      * who holds a handle to the client that carries PROCESS_TERMINATE, sampled while it
        lives, because the killer needs one and it is invisible from inside;
      * whether the client sits in a job object, since a job time limit terminates from the
        kernel and no in-process watch could ever see it;
      * whether the exit tracks wall-clock or CPU time, since a quota is spent, not waited;
      * which processes appeared or vanished around the exit.

    Read-only throughout: it opens the process for query, reads counters, duplicates
    handles with no access requested, and reads event logs.

    RUN THIS ELEVATED. Without it, handles held by services running as SYSTEM cannot be
    duplicated and the suspect list is silently short - the control scan could not reach
    1087 of 1564 process handles from an unelevated shell. Every scan line reports the
    unreachable count so an empty list is never mistaken for "nobody holds one".

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File tools\exit-forensics.ps1 -ClientPid 1234
#>
[CmdletBinding()]
param(
    # Not -Pid: $PID is an automatic variable and PowerShell rejects the parameter name.
    [Parameter(Mandatory = $true)]
    [int]$ClientPid,
    [string]$Log = '',
    [double]$Interval = 0.25,
    # How often to write a heartbeat line even when nothing changed.
    [double]$Heartbeat = 5,
    # How often to re-scan for handle holders. The control scan took 0.4s, so this is
    # affordable; set 0 to skip the scans entirely.
    [double]$HandleScan = 4
)

$ErrorActionPreference = 'Continue'
$here = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
if (-not $Log) { $Log = Join-Path (Split-Path -Parent $here) 'client-exit.log' }

# Same format as maplecw-hook.log and probe.log. Two logs that cannot be lined up are two
# instruments that cannot check each other, and that has cost eight launches once already.
function Write-Line {
    param([string]$Text)
    $stamp = (Get-Date).ToString('HH:mm:ss.fff')
    try { Add-Content -Path $Log -Value "$stamp $Text" } catch { }
}

function Get-Family {
    # The processes worth naming: the client, its protection stack, and Nexon's services.
    try {
        Get-Process -ErrorAction SilentlyContinue |
            Where-Object { $_.ProcessName -match '(?i)maple|nexon|grap|black|secure|guard|npx|ncm' } |
            ForEach-Object {
                $started = try { $_.StartTime.ToString('HH:mm:ss') } catch { '?' }
                "$($_.ProcessName)#$($_.Id)@$started"
            }
    }
    catch { @() }
}

function Get-Sample {
    param($Proc)
    $s = [pscustomobject]@{
        At = Get-Date; Threads = -1; Handles = -1; Cpu = [TimeSpan]::Zero; Ws = 0; Tids = @()
    }
    try { $Proc.Refresh() } catch { }
    try { $s.Threads = $Proc.Threads.Count } catch { }
    try { $s.Handles = $Proc.HandleCount } catch { }
    try { $s.Cpu = $Proc.TotalProcessorTime } catch { }
    try { $s.Ws = $Proc.WorkingSet64 } catch { }
    try { $s.Tids = @($Proc.Threads | ForEach-Object { $_.Id }) } catch { }
    return $s
}

function Get-ThreadTable {
    param($Proc)
    $rows = @()
    try {
        foreach ($t in $Proc.Threads) {
            $state = try { "$($t.ThreadState)" } catch { '?' }
            $why = ''
            try { if ("$($t.ThreadState)" -eq 'Wait') { $why = "/$($t.WaitReason)" } } catch { }
            $rows += "$($t.Id):$state$why"
        }
    }
    catch { return '(threads unreadable)' }
    return ($rows -join ' ')
}

$proc = $null
try { $proc = Get-Process -Id $ClientPid -ErrorAction Stop } catch {
    Write-Line "client $ClientPid was already gone when the watcher opened it: $_"
    exit
}

# Touch the handle now. ExitCode is unreadable once the last handle to a dead process
# closes, and .NET only opens one lazily - by which time there may be nothing left to open.
$handleOk = $true
try { $null = $proc.Handle } catch { $handleOk = $false }

$elevated = $false
try {
    $elevated = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()
    ).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}
catch { }

$startedAt = try { $proc.StartTime } catch { Get-Date }
$first = Get-Sample $proc
$noHandle = if ($handleOk) { '' } else { ' (no query handle - the exit code will be unreadable; run elevated)' }
Write-Line "watching $($proc.ProcessName) pid $ClientPid, started $($startedAt.ToString('HH:mm:ss.fff')), $($first.Threads) threads, $($first.Handles) handles$noHandle"
Write-Line "this shell is $(if ($elevated) { 'elevated' } else { 'NOT elevated - handles held by SYSTEM services cannot be duplicated and the suspect list will be short' })"
Write-Line "threads at start: $(Get-ThreadTable $proc)"

# A job object with PerProcessUserTimeLimit or an end-of-job timer kills its processes
# without running a single instruction in them - exactly the shape of an exit no
# in-process breakpoint can catch. Membership alone is not a finding: modern Windows puts
# plenty of ordinary processes in jobs, and both 2026-08-17 controls reported True. It is
# a lead only if the exit also reproduces at a constant lifetime or a constant CPU time.
try {
    if (-not ('MapleCW.Job' -as [type])) {
        $sig = '[System.Runtime.InteropServices.DllImport("kernel32.dll", SetLastError = true)]' + "`n" +
        'public static extern bool IsProcessInJob(System.IntPtr process, System.IntPtr job, out bool result);'
        Add-Type -Namespace MapleCW -Name Job -MemberDefinition $sig
    }
    $inJob = $false
    if ([MapleCW.Job]::IsProcessInJob($proc.Handle, [IntPtr]::Zero, [ref]$inJob)) {
        Write-Line "in a job object: $inJob (common, and true for both controls - only a lead if the lifetime is also constant)"
    }
    else {
        Write-Line "in a job object: could not be determined (win32 error $([System.Runtime.InteropServices.Marshal]::GetLastWin32Error()))"
    }
}
catch { Write-Line "in a job object: the check itself failed: $_" }

# Whoever kills the client needs a handle to it carrying PROCESS_TERMINATE, and that call
# happens in the other process where nothing of ours can see it. Sampling the holders while
# the client lives is the only way to have a suspect ready if the in-process watches stay
# silent. The launching shell legitimately holds one (Start-Process -PassThru), so it is
# expected in this list and is not the suspect.
$scanOk = $false
if ($HandleScan -gt 0) {
    try {
        . (Join-Path $here 'handle-holders.ps1')
        $scanOk = $true
    }
    catch { Write-Line "handle-holder scanning unavailable: $_" }
}

$familyAtStart = @(Get-Family)
Write-Line "protection/nexon processes at start: $(if ($familyAtStart) { $familyAtStart -join ' ' } else { '(none)' })"
$pidsAtStart = @()
try { $pidsAtStart = @(Get-Process -ErrorAction SilentlyContinue | ForEach-Object { $_.Id }) } catch { }

$prev = $first
$last = $first
$lastBeat = Get-Date
$lastScan = [DateTime]::MinValue
$lastTable = ''
$lastHolders = ''
$peakThreads = $first.Threads

while ($true) {
    Start-Sleep -Milliseconds ([int]($Interval * 1000))
    $exited = $false
    try { $exited = $proc.HasExited } catch { $exited = $true }
    if ($exited) { break }

    $s = Get-Sample $proc
    if ($s.Threads -gt $peakThreads) { $peakThreads = $s.Threads }
    if ($s.Threads -ne $prev.Threads -and $s.Threads -ge 0 -and $prev.Threads -ge 0) {
        $gone = @($prev.Tids | Where-Object { $_ -notin $s.Tids })
        $new = @($s.Tids | Where-Object { $_ -notin $prev.Tids })
        $detail = ''
        if ($gone) { $detail += " ended=$($gone -join ',')" }
        if ($new) { $detail += " started=$($new -join ',')" }
        Write-Line "threads $($prev.Threads) -> $($s.Threads)$detail"
    }
    if (((Get-Date) - $lastBeat).TotalSeconds -ge $Heartbeat) {
        $up = ((Get-Date) - $startedAt).TotalSeconds
        Write-Line ("+{0:N1}s threads={1} handles={2} cpu={3:N2}s ws={4:N0}MB" -f `
                $up, $s.Threads, $s.Handles, $s.Cpu.TotalSeconds, ($s.Ws / 1MB))
        $lastBeat = Get-Date
    }
    if ($scanOk -and ((Get-Date) - $lastScan).TotalSeconds -ge $HandleScan) {
        try {
            $holders = Format-HandleHolder -TargetPid $ClientPid
            if ($holders -ne $lastHolders) {
                Write-Line "handles on the client: $holders"
                $lastHolders = $holders
            }
        }
        catch { Write-Line "handle scan failed: $_" }
        $lastScan = Get-Date
    }
    # Kept fresh so the sample nearest the exit can be printed after the fact.
    $lastTable = Get-ThreadTable $proc
    $prev = $s
    $last = $s
}

$exitAt = Get-Date
$code = $null
$exitTime = $null
try { $code = $proc.ExitCode } catch { }
try { $exitTime = $proc.ExitTime } catch { }
if ($exitTime -and $exitTime -gt $startedAt) { $exitAt = $exitTime }
$life = ($exitAt - $startedAt).TotalSeconds
$sinceLast = ($exitAt - $last.At).TotalSeconds

if ($null -ne $code) {
    Write-Line ("EXIT code 0x{0:X8} ({0}) after {1:N1}s of life, {2:N2}s of CPU" -f `
            $code, $life, $last.Cpu.TotalSeconds)
}
else {
    Write-Line ("EXIT after {0:N1}s of life - the exit code could not be read (no query handle)" -f $life)
}
Write-Line ("last sample was {0:N2}s before the exit: threads={1} (peak {2}) handles={3}" -f `
        $sinceLast, $last.Threads, $peakThreads, $last.Handles)
Write-Line "threads in that sample (data, not a verdict - see the header): $(if ($lastTable) { $lastTable } else { '(not captured)' })"
if ($lastHolders) { Write-Line "handles on the client, last scan: $lastHolders" }

# What the code means. Written down here rather than left to whoever reads the log, so the
# criterion cannot drift afterwards to fit whatever came out.
if ($null -ne $code) {
    # Two traps here, both of which produced a confidently wrong reading before being
    # caught by the killed control on 2026-08-17:
    #
    #   * `-1 -band 0xFFFFFFFF` stays -1 in PowerShell and the [uint32] cast then throws;
    #   * a hex literal that fits in 32 bits is parsed as [int], so `0xFFFFFFFF` in a
    #     switch case is -1 and `0xC0000409` is negative. Every wide case was unreachable
    #     and every exit fell through to the default branch.
    #
    # Formatting to a string first and switching on that has neither problem.
    $u = [BitConverter]::ToUInt32([BitConverter]::GetBytes([int]$code), 0)
    $hex = '0x{0:X8}' -f $u
    $read = switch ($hex) {
        '0xFFFFFFFF' { 'TerminateProcess with -1. That is what .NET Process.Kill passes, and what the 2026-08-17 killed control returned. Someone outside ended it: read the handle scans above for who held PROCESS_TERMINATE.' }
        '0x00000001' { 'exit code 1 - what taskkill /F passes, and also a perfectly ordinary application exit. Weak on its own; the in-process watches decide it.' }
        '0x00000000' { 'exit code 0. Nothing failed. With RtlExitUserProcess ruled out, a clean 0 fits the last-thread path - check whether ntdll!RtlExitUserThread fired in maplecw-hook.log.' }
        '0xC0000409' { '__fastfail (STATUS_STACK_BUFFER_OVERRUN). A security check tripped - this is the code protection stacks use when they detect tampering.' }
        '0xC0000005' { 'an access violation reached the top. A vectored handler should have seen this - if none did, it happened on a thread with no handler, or after ours was removed.' }
        '0x40010004' { 'DBG_TERMINATE_PROCESS - a debugger ended it.' }
        '0xC000013A' { 'STATUS_CONTROL_C_EXIT - a console control event ended it.' }
        '0x00000718' { 'ERROR_NOT_ENOUGH_QUOTA - a job object time limit. Follow the job line above.' }
        default { 'a code with no standard meaning, so it was chosen - either by the client as an ordinary application exit code, or by an outside killer passing a value to TerminateProcess. The orderly 2026-08-17 control returned 0x0000002A this way. Repeating exactly across runs says it was deliberate; only the in-process watches say which side chose it.' }
    }
    Write-Line "READ: $read"
}
Write-Line 'READ: the decisive line is in client-patched\maplecw-hook.log, not here - a WATCH on ntdll!RtlExitUserThread or ntdll!NtTerminateThread means the client ended itself; silence on both means it did not.'

$familyAtExit = @(Get-Family)
Write-Line "protection/nexon processes at exit: $(if ($familyAtExit) { $familyAtExit -join ' ' } else { '(none)' })"
$appeared = @($familyAtExit | Where-Object { $_ -notin $familyAtStart })
$vanished = @($familyAtStart | Where-Object { $_ -notin $familyAtExit })
if ($appeared) { Write-Line "appeared during the run: $($appeared -join ' ')" }
if ($vanished) { Write-Line "vanished during the run: $($vanished -join ' ')" }
try {
    $newcomers = @(Get-Process -ErrorAction SilentlyContinue |
            Where-Object { $_.Id -notin $pidsAtStart -and $_.Id -ne $PID } |
            ForEach-Object { "$($_.ProcessName)#$($_.Id)" })
    if ($newcomers) { Write-Line "processes that started after the client did: $($newcomers -join ' ')" }
}
catch { }

# The Application log was checked by hand after the last three runs and held nothing.
# Doing it here removes the step, and covers System too - a kill by a service leaves its
# trace there rather than in Application.
try {
    $events = Get-WinEvent -MaxEvents 15 -ErrorAction SilentlyContinue -FilterHashtable @{
        LogName = 'Application', 'System'; StartTime = $startedAt.AddSeconds(-5)
    }
    $hits = @($events | Where-Object { $_.LevelDisplayName -in @('Error', 'Critical', 'Warning') })
    if ($hits) {
        foreach ($e in $hits) {
            $msg = ($e.Message -split "`n")[0]
            Write-Line "event: $($e.TimeCreated.ToString('HH:mm:ss')) $($e.LogName)/$($e.ProviderName) $($e.LevelDisplayName) - $msg"
        }
    }
    else {
        Write-Line 'no Error/Warning event in Application or System during this run (that log is known to work: it holds a real MapleStory 0xc0000005 from 2026-08-14)'
    }
}
catch { Write-Line "event log check failed: $_" }
