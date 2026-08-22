# Open the newest client crash dump in WinDbg and run the page-heap analysis.
#
#   powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\analyse-dump.ps1"
#
# WHY A DUMP AND NOT AN ATTACH
#
# `research/protection-surface.md` says of this client: "Runtime anti-debug. Expect debugger
# detection; attaching to a running client is a fight, independent of GameGuard." A dump is
# read after the process is gone and nothing is attached while it runs, so there is nothing
# for the anti-debug to detect. That is the whole reason this script exists instead of a
# `-pn MapleStory.exe` one-liner.
#
# WHAT PRODUCES THE DUMP - NOT WER, AND THAT MATTERS
#
# The hook writes it, from the vectored exception handler in `crates/grap-stub/src/minidump.rs`.
# Windows Error Reporting never produced one and never could: on 2026-08-21 a decoy named
# MapleStory.exe that only dereferences null wrote a 9.4 MB dump into dumps\ with the same
# LocalDumps key armed, while the real client's own 0xC0000005 - 88 minutes after WER was
# switched on - produced nothing at all. The client ships its own crash reporting and a
# process that handles its own faults never reaches WerFault.
#
# Two consequences for reading the file:
#
#   * It is taken FIRST-CHANCE, at the faulting instruction, rather than post-mortem. The
#     faulting thread's stack is intact and has not been unwound.
#   * PAGE HEAP IS NOT ENABLED, so `!heap -p -a` will not have allocation and free stacks to
#     print. It is still worth running - it identifies the block - but the stack at the stop
#     and `!heap -s` are what carry the weight here. Page heap is a separate IFEO setting and
#     is the owner's to turn on; the 418 MB peak working set is the independent evidence it is off,
#     because a page-heap run peaks near 990 MB.
#
# See `research/heap-corruption.md`.

param(
    # A specific dump, or the newest one in dumps/ when omitted.
    [string]$Dump = "",
    # Where to write the analysis.
    [string]$Out = ""
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot

if (-not $Dump) {
    $dir = Join-Path $root 'dumps'
    if (-not (Test-Path $dir)) { throw "no dumps directory at $dir - the launcher creates it, so this means test-server.ps1 has not run" }
    $newest = Get-ChildItem $dir -Filter *.dmp -ErrorAction SilentlyContinue |
              Sort-Object LastWriteTime -Descending | Select-Object -First 1
    if (-not $newest) { throw "no .dmp files in $dir - which is itself a finding. Check client-patched\maplecw-hook.log for a CRASH DUMP line: one saying 'writing' with no matching 'wrote' means the dump attempt died partway, and neither line means the fault was not one the handler matches" }
    $Dump = $newest.FullName
}
if (-not (Test-Path $Dump)) { throw "no such dump: $Dump" }
if (-not $Out) { $Out = Join-Path $root 'dump-analysis.txt' }

Write-Host "dump:   $Dump"
Write-Host ("size:   {0:N0} MB" -f ((Get-Item $Dump).Length / 1MB))
Write-Host "output: $Out"

# Find a debugger. The Store build of WinDbg ships an alias in WindowsApps; the classic SDK
# build ships cdb.exe, which is the one worth having because it takes -c and exits.
$cdb = @(
    "C:\Program Files (x86)\Windows Kits\10\Debuggers\x64\cdb.exe",
    "C:\Program Files\Windows Kits\10\Debuggers\x64\cdb.exe"
) | Where-Object { Test-Path $_ } | Select-Object -First 1

# The commands worth running, in the order that answers the question.
#
#   !analyze -v     WER's own reading, and for a verifier stop it prints the rule that broke
#   !heap -p -a     THE one that matters: for a page-heap block it prints the allocation
#                   stack and the free stack, which is what a post-mortem 0xC0000374 could
#                   never give. Fed the faulting address from the exception record.
#   kb              the stack at the stop
#   .exr / .ecxr    the exception record and its context, in case !analyze is unhelpful
$script = @'
.symfix
.reload
.echo ===== EXCEPTION =====
.exr -1
.echo ===== ANALYZE =====
!analyze -v
.echo ===== STACK =====
kb
.echo ===== PAGE HEAP (the allocation and free stacks) =====
!heap -p -a @$exr_param0
.echo ===== HEAP SUMMARY =====
!heap -s
.echo ===== DONE =====
qd
'@

if ($cdb) {
    Write-Host "using: $cdb"
    $tmp = Join-Path $env:TEMP 'maplecw-dump-cmds.txt'
    Set-Content -Path $tmp -Value $script -Encoding ascii
    & $cdb -z $Dump -logo $Out -cf $tmp
    Write-Host ""
    Write-Host "wrote $Out"
} else {
    Write-Host ""
    Write-Host "cdb.exe is NOT installed - only the Store build of WinDbg is here, and it has"
    Write-Host "no scriptable console mode worth relying on. Open the dump by hand instead:"
    Write-Host ""
    Write-Host "  1. Start WinDbg (the Store one)."
    Write-Host "  2. File -> Open dump file -> $Dump"
    Write-Host "  3. In the command box, run these one at a time and copy the output out:"
    Write-Host ""
    foreach ($line in ($script -split "`n")) {
        $t = $line.Trim()
        if ($t -and -not $t.StartsWith('.echo') -and $t -ne 'qd') { Write-Host "       $t" }
    }
    Write-Host ""
    Write-Host "     !heap -p -a <address> wants the faulting address, which .exr -1 prints"
    Write-Host "     as the first exception parameter."
    Write-Host ""
    Write-Host "     PAGE HEAP IS OFF, so !heap -p -a cannot print allocation and free stacks."
    Write-Host "     The stack at the stop and !heap -s are what carry the weight. STATUS_HEAP_"
    Write-Host "     CORRUPTION is raised at the NEXT allocator walk rather than where the"
    Write-Host "     damage happened, so the fault address names ntdll and not the culprit -"
    Write-Host "     read `kb` for the client frame that was allocating or freeing."
}
