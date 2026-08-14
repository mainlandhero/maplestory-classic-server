<#
.SYNOPSIS
    Try launching the patched client with different argument sets and report which survive.

.DESCRIPTION
    The client is normally started by the Nexon launcher, and a bare launch crashes
    immediately (0xC0000005). The binary contains these argument tokens:

        GAMELAUNCHING  WEBSTART  STEAMSTART  IPPORT  autologin  skiplogo
        -NXL  -NXLDEBUG  -NXLPTS

    This walks candidate argument sets and reports how long each survives, so we can
    find the shape the client expects. Longer survival = got further before failing.

    Assumes tools/firewall.ps1 -Add has been run, so the client cannot reach Nexon.
#>
[CmdletBinding()]
param(
    [string]$ClientDir,
    [int]$WaitSeconds = 8,
    [string]$ServerIp = '127.0.0.1',
    [int]$ServerPort = 8484
)

$ErrorActionPreference = 'Stop'
$here = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
if (-not $ClientDir) { $ClientDir = Join-Path (Split-Path -Parent $here) 'client-patched' }
$exe = Join-Path $ClientDir 'MapleStory.exe'
if (-not (Test-Path $exe)) { throw "not found: $exe" }

# Candidate argument sets, roughly cheapest/most-likely first.
$cases = @(
    @{ name = '(no arguments)';            args = @() },
    @{ name = 'GAMELAUNCHING';             args = @('GAMELAUNCHING') },
    @{ name = 'WEBSTART';                  args = @('WEBSTART') },
    @{ name = 'STEAMSTART';                args = @('STEAMSTART') },
    @{ name = 'GAMELAUNCHING ip port';     args = @('GAMELAUNCHING', $ServerIp, "$ServerPort") },
    @{ name = 'IPPORT ip port';            args = @('IPPORT', $ServerIp, "$ServerPort") },
    @{ name = 'ip port (positional)';      args = @($ServerIp, "$ServerPort") },
    @{ name = 'GAMELAUNCHING skiplogo';    args = @('GAMELAUNCHING', 'skiplogo') },
    @{ name = 'WEBSTART ip port';          args = @('WEBSTART', $ServerIp, "$ServerPort") },
    @{ name = '-NXL';                      args = @('-NXL') },
    @{ name = '-NXLDEBUG';                 args = @('-NXLDEBUG') }
)

Write-Host ("{0,-28} {1,-9} {2,-12} {3}" -f 'arguments', 'survived', 'exit', 'verdict')
Write-Host ('-' * 74)

foreach ($c in $cases) {
    $sw = [Diagnostics.Stopwatch]::StartNew()
    try {
        $p = if ($c.args.Count) {
            Start-Process -FilePath $exe -WorkingDirectory $ClientDir -ArgumentList $c.args -PassThru
        } else {
            Start-Process -FilePath $exe -WorkingDirectory $ClientDir -PassThru
        }
    } catch {
        Write-Host ("{0,-28} {1,-9} {2,-12} {3}" -f $c.name, '-', '-', "launch failed: $_")
        continue
    }

    $exited = $p.WaitForExit($WaitSeconds * 1000)
    $sw.Stop()
    $secs = '{0:N1}s' -f $sw.Elapsed.TotalSeconds

    if ($exited) {
        $code = '0x{0:X8}' -f $p.ExitCode
        $verdict = switch ($p.ExitCode) {
            -1073741819 { 'ACCESS_VIOLATION' }
            -1073741515 { 'MISSING DLL' }
            0           { 'clean exit' }
            default     { '' }
        }
    } else {
        $code = '(running)'
        $verdict = '*** SURVIVED - investigate ***'
        Stop-Process -Id $p.Id -Force -ErrorAction SilentlyContinue
        Start-Sleep -Milliseconds 500
    }
    Write-Host ("{0,-28} {1,-9} {2,-12} {3}" -f $c.name, $secs, $code, $verdict)
    Start-Sleep -Milliseconds 800
}
