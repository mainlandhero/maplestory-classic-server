<#
.SYNOPSIS
    Name the driver behind "System is at 100% CPU". Read-only, changes nothing.

.DESCRIPTION
    The owner, 2026-08-22: "my computer has been getting pretty slow with all of these tests.
    Most of the time it is because System is taking up 100% CPU."

    Task Manager's "System" row is process id 4 - the kernel's own process, which hosts a
    thread for nearly every loaded driver. Saying "System is busy" is therefore like saying
    "a program is busy": true, and it names nothing. This samples System's threads twice,
    subtracts, and resolves each busy thread's start address to the driver that owns it, so
    the answer is a file name.

    IT IS READ-ONLY. It enumerates drivers and reads thread times. It changes no setting,
    stops no service and writes nothing outside its own output.

    RUN IT ELEVATED. Per-thread CPU times on process 4 come back EMPTY from an unelevated
    shell - not zero, empty - so an unelevated run would report every thread as idle, which
    is exactly the confident wrong answer this repo keeps warning about. The script checks
    for that and refuses rather than printing it.

    "SYSTEM" AND "SYSTEM INTERRUPTS" ARE DIFFERENT ROWS. This measures process 4 only. If
    Task Manager's busy row says *System interrupts*, that is DPC/ISR time, it belongs to no
    process, and none of the numbers below will account for it - the script says so at the
    end rather than letting a small total read as "nothing found".

.PARAMETER Seconds
    Sampling window. 10 is usually enough; use 30 if the load comes in bursts.

.PARAMETER Top
    How many threads to list.

.PARAMETER SelfTest
    Point the same sampling code at THIS process, burn two seconds of CPU, and check it is
    seen. Needs no elevation, and it is the control that makes a quiet result trustworthy:
    a sampler that always returns zero is indistinguishable from an idle machine.

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\system-cpu.ps1"

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\system-cpu.ps1" -Seconds 30
#>
[CmdletBinding()]
param(
    [double]$Seconds = 10,
    [int]$Top = 12,
    # Run the sampling arithmetic against THIS process, which always has readable thread
    # times, and check it reports the CPU that was deliberately burned. Needs no elevation.
    # A sampler that always returns zero looks exactly like a quiet machine, so this is the
    # control that has to pass before the real number means anything.
    [switch]$SelfTest
)

$ErrorActionPreference = 'Continue'

if (-not ('MapleCW.Drivers' -as [type])) {
    Add-Type -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace MapleCW
{
    public class Driver
    {
        public ulong Base;
        public string Name;
    }

    public static class Drivers
    {
        [DllImport("psapi.dll", SetLastError = true)]
        static extern bool EnumDeviceDrivers(IntPtr[] image_base, uint cb, out uint needed);

        [DllImport("psapi.dll", SetLastError = true, CharSet = CharSet.Unicode)]
        static extern int GetDeviceDriverBaseNameW(IntPtr image_base, StringBuilder name, int size);

        public static List<Driver> Enumerate()
        {
            var list = new List<Driver>();
            uint needed;
            EnumDeviceDrivers(new IntPtr[0], 0, out needed);
            if (needed == 0) return list;
            var bases = new IntPtr[needed / IntPtr.Size + 16];
            if (!EnumDeviceDrivers(bases, (uint)(bases.Length * IntPtr.Size), out needed))
                return list;
            int count = (int)(needed / IntPtr.Size);
            for (int i = 0; i < count; i++)
            {
                if (bases[i] == IntPtr.Zero) continue;
                var sb = new StringBuilder(260);
                if (GetDeviceDriverBaseNameW(bases[i], sb, sb.Capacity) == 0) continue;
                list.Add(new Driver { Base = (ulong)bases[i].ToInt64(), Name = sb.ToString() });
            }
            list.Sort(delegate(Driver a, Driver b) { return a.Base.CompareTo(b.Base); });
            return list;
        }
    }
}
'@
}

$drivers = [MapleCW.Drivers]::Enumerate()

# ------------------------------------------------------------------ self-checks first
# A resolver that silently returns nothing prints "unknown" beside every thread, which reads
# as "the tool found nothing" rather than "the tool is broken". Both controls must pass.
Write-Host '== SELF-CHECKS  (if these fail the tool is wrong, not the machine)' -ForegroundColor Cyan
if ($drivers.Count -eq 0) {
    Write-Host '  [FAIL] EnumDeviceDrivers returned nothing. Every line below would say "unknown".'
    Write-Host '         Run this ELEVATED and try again.'
    exit 1
}
Write-Host ("  [PASS] {0} loaded drivers enumerated" -f $drivers.Count)

# A kernel address like 0xfffff801`183c1430 is NEGATIVE as an Int64, and [uint64] on a
# negative number throws rather than reinterpreting. Round-trip the bits instead.
function ConvertTo-U64 {
    param($Address)
    [System.BitConverter]::ToUInt64([System.BitConverter]::GetBytes([int64]$Address), 0)
}

function Resolve-Driver {
    param([uint64]$Address)
    # Highest base at or below the address. Binary search over a list sorted at build time.
    $lo = 0; $hi = $drivers.Count - 1; $best = $null
    while ($lo -le $hi) {
        $mid = [int](($lo + $hi) / 2)
        if ($drivers[$mid].Base -le $Address) { $best = $drivers[$mid]; $lo = $mid + 1 } else { $hi = $mid - 1 }
    }
    if ($best) { return $best.Name }
    return 'unknown'
}

$sys = Get-Process -Id 4 -ErrorAction SilentlyContinue
if (-not $sys) {
    Write-Host '  [FAIL] process 4 is not readable at all.'
    exit 1
}
# The kernel image must resolve, or the address arithmetic is wrong and every answer is too.
$kernelThread = $sys.Threads | Select-Object -First 1
$kernelName = Resolve-Driver (ConvertTo-U64 $kernelThread.StartAddress.ToInt64())
if ($kernelName -notmatch '^(ntoskrnl|ntkrnlmp)') {
    Write-Host ("  [WARN] the first System thread resolved to {0}, not the kernel image. " -f $kernelName)
    Write-Host '         Not fatal - some System threads do belong to drivers - but if EVERY'
    Write-Host '         line below names the same odd module, distrust the mapping.'
} else {
    Write-Host ("  [PASS] a System thread resolves to the kernel image ({0})" -f $kernelName)
}

# Per-thread CPU on process 4 is empty, not zero, without elevation. -SelfTest reads THIS
# process instead, so it is allowed past this gate.
$probe = $sys.Threads | Where-Object { $null -ne $_.TotalProcessorTime } | Select-Object -First 1
if (-not $probe -and -not $SelfTest) {
    Write-Host '  [FAIL] per-thread CPU times are unreadable. THIS SHELL IS NOT ELEVATED.'
    Write-Host '         An unelevated run reports every thread as idle, which looks exactly'
    Write-Host '         like a quiet machine. Refusing rather than printing that.'
    exit 1
}
if ($probe) { Write-Host '  [PASS] per-thread CPU times are readable (this shell is elevated)' }
Write-Host ''

# ------------------------------------------------------------------ sample
# One function, used by the real measurement and by -SelfTest, so the control exercises the
# arithmetic that produces the answer rather than a copy of it.
function Measure-ThreadCpu {
    param([int]$ProcessId, [double]$ForSeconds, [switch]$Busy)

    $before = @{}
    foreach ($t in (Get-Process -Id $ProcessId).Threads) {
        try {
            $before[[int]$t.Id] = @{
                Cpu   = $t.TotalProcessorTime.TotalSeconds
                Start = (ConvertTo-U64 $t.StartAddress.ToInt64())
            }
        } catch { }
    }
    $t0 = Get-Date
    if ($Busy) {
        # Burn real CPU on this thread rather than sleeping, so the delta has something to
        # find. `while` on a stopwatch, not a fixed iteration count, keeps it machine-independent.
        $spin = [Diagnostics.Stopwatch]::StartNew()
        while ($spin.Elapsed.TotalSeconds -lt $ForSeconds) { $null = 1 }
    } else {
        Start-Sleep -Seconds $ForSeconds
    }
    $elapsed = ((Get-Date) - $t0).TotalSeconds

    $rows = @()
    foreach ($t in (Get-Process -Id $ProcessId).Threads) {
        $id = [int]$t.Id
        try { $now = $t.TotalProcessorTime.TotalSeconds } catch { continue }
        $was = if ($before.ContainsKey($id)) { $before[$id].Cpu } else { 0 }
        $delta = $now - $was
        if ($delta -le 0) { continue }
        $addr = if ($before.ContainsKey($id)) { $before[$id].Start } else { ConvertTo-U64 $t.StartAddress.ToInt64() }
        $rows += [pscustomobject]@{
            Tid     = $id
            Seconds = $delta
            Percent = 100.0 * $delta / $elapsed
            Driver  = Resolve-Driver $addr
            Start   = $addr
        }
    }
    return @{ Rows = $rows; Elapsed = $elapsed }
}

if ($SelfTest) {
    Write-Host '== SELF-TEST: sampling THIS process while burning 2s of CPU' -ForegroundColor Cyan
    $r = Measure-ThreadCpu -ProcessId $PID -ForSeconds 2 -Busy
    $burned = ($r.Rows | Measure-Object -Property Seconds -Sum).Sum
    if (-not $burned) { $burned = 0 }
    Write-Host ("  measured {0:N2}s of CPU across {1} thread(s) in {2:N2}s" -f $burned, $r.Rows.Count, $r.Elapsed)
    if ($burned -lt 1.0) {
        Write-Host '  [FAIL] a busy loop that ran for 2 seconds was not seen. The sampler is broken.'
        exit 1
    }
    Write-Host '  [PASS] the sampler reports CPU that was really burned, so a zero from it means zero.'
    exit 0
}

$cores = [Environment]::ProcessorCount
Write-Host ("sampling process 4 for {0}s on {1} logical cores..." -f $Seconds, $cores)
$measured = Measure-ThreadCpu -ProcessId 4 -ForSeconds $Seconds
$rows = $measured.Rows
$elapsed = $measured.Elapsed

$total = ($rows | Measure-Object -Property Seconds -Sum).Sum
if (-not $total) { $total = 0 }

Write-Host ''
Write-Host ("== SYSTEM (pid 4) used {0:N1}s of CPU in {1:N1}s = {2:N0}% of one core, {3:N0}% of the machine" -f `
    $total, $elapsed, (100 * $total / $elapsed), (100 * $total / ($elapsed * $cores))) -ForegroundColor Cyan
Write-Host ''
Write-Host ("{0,-8} {1,9} {2,8}  {3}" -f 'tid', 'cpu', 'of 1 core', 'driver that owns the thread')
foreach ($r in ($rows | Sort-Object Seconds -Descending | Select-Object -First $Top)) {
    Write-Host ("{0,-8} {1,8:N2}s {2,7:N0}%  {3}  (start 0x{4:x})" -f $r.Tid, $r.Seconds, $r.Percent, $r.Driver, $r.Start)
}

Write-Host ''
$byDriver = $rows | Group-Object Driver | ForEach-Object {
    [pscustomobject]@{ Driver = $_.Name; Seconds = ($_.Group | Measure-Object Seconds -Sum).Sum; Threads = $_.Count }
} | Sort-Object Seconds -Descending | Select-Object -First 8
Write-Host 'by driver:'
foreach ($d in $byDriver) {
    Write-Host ("  {0,8:N2}s  {1,7:N0}% of one core  {2,3} thread(s)  {3}" -f `
        $d.Seconds, (100 * $d.Seconds / $elapsed), $d.Threads, $d.Driver)
}

Write-Host ''
Write-Host 'HOW TO READ THIS' -ForegroundColor Cyan
Write-Host '  ntoskrnl.exe        the kernel itself. Usually the memory manager, the cache'
Write-Host '                      writer or a worker queue - i.e. heavy disk or paging.'
Write-Host '  WdFilter.sys        Windows Defender''s file-system filter. It inspects every'
Write-Host '                      file open. A 2.3 GB target\ dir and 4.8 GB of crash dumps'
Write-Host '                      are exactly what makes it expensive.'
Write-Host '  ntfs.sys / storport / disk / volsnap   the storage stack. Disk-bound.'
Write-Host '  a network filter (pacer, netio, wfp, vmswitch, or an AV''s own)  packet path.'
Write-Host ''
Write-Host 'IF THE TOTAL ABOVE IS SMALL and Task Manager still shows a busy row, the row you'
Write-Host 'are looking at is probably "System interrupts" - DPC and ISR time, which belongs'
Write-Host 'to no process and is NOT counted here. That is a different diagnosis (usually one'
Write-Host 'misbehaving device driver) and this script cannot see it.'
