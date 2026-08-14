<#
.SYNOPSIS
    Block the patched test client from reaching the network. Requires elevation.

.DESCRIPTION
    Adds an outbound Windows Firewall block rule scoped to a single executable:
    client-patched\MapleStory.exe.

    Why: the patched client still contains Nexon's API endpoints and a full
    curl/OpenSSL stack, so on startup it may contact Nexon before we ever reach a
    login screen. Running a *modified* client with live internet access is how a
    machine or account gets flagged. This rule removes that risk.

    Loopback is not filtered by Windows Firewall, so 127.0.0.1 traffic to our own
    local server is unaffected.

    The rule is named so it is easy to find and remove; -Remove undoes it.

.EXAMPLE
    pwsh tools/firewall.ps1 -Status
    pwsh tools/firewall.ps1 -Add
    pwsh tools/firewall.ps1 -Remove
#>
[CmdletBinding()]
param(
    [string]$ClientExe,
    [switch]$Add,
    [switch]$Remove,
    [switch]$Status
)

$ErrorActionPreference = 'Stop'
$RuleName = 'MapleCW - block patched client outbound'

$here = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }
$repo = Split-Path -Parent $here
if (-not $ClientExe) { $ClientExe = Join-Path $repo 'client-patched\MapleStory.exe' }

function Test-Elevated {
    $id = [Security.Principal.WindowsIdentity]::GetCurrent()
    (New-Object Security.Principal.WindowsPrincipal($id)).IsInRole(
        [Security.Principal.WindowsBuiltInRole]::Administrator)
}

# netsh is used rather than Get-NetFirewallRule: the NetSecurity cmdlets return an
# empty set in some environments (they need CIM/WMI), which reads as "no rule" even
# when the rule is present. netsh queries the firewall directly and is reliable.
function Test-RuleExists {
    $null = netsh advfirewall firewall show rule name="$RuleName" 2>&1
    return ($LASTEXITCODE -eq 0)
}

function Show-Status {
    $out = netsh advfirewall firewall show rule name="$RuleName" verbose 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "rule '$RuleName': NOT PRESENT - client is NOT blocked"
        return
    }
    $keep = 'Rule Name|Enabled|Direction|Program|Action|Profiles'
    $out | Where-Object { $_ -match "^($keep):" } | ForEach-Object { Write-Host "  $_" }
}

if ($Status -or (-not $Add -and -not $Remove)) { Show-Status; return }

if (-not (Test-Elevated)) {
    throw "This needs an elevated PowerShell (Run as administrator)."
}

if ($Remove) {
    if (Test-RuleExists) {
        netsh advfirewall firewall delete rule name="$RuleName" | Out-Null
        Write-Host "removed rule '$RuleName'"
    } else {
        Write-Host 'nothing to remove'
    }
    Show-Status
    return
}

# -Add
if (-not (Test-Path $ClientExe)) { throw "Client executable not found: $ClientExe" }
$ClientExe = (Resolve-Path $ClientExe).Path

# Never scope this at the original install.
if ($ClientExe -like '*\Nexon\Library\*') {
    throw "Refusing to target the original install: $ClientExe"
}

if (Test-RuleExists) {
    Write-Host 'rule already exists; removing and re-adding so the path is current'
    netsh advfirewall firewall delete rule name="$RuleName" | Out-Null
}

$desc = 'Local RE/testing: prevents the patched MapleStory client from contacting ' +
        'Nexon. Loopback is unaffected. Safe to delete.'
netsh advfirewall firewall add rule `
    name="$RuleName" `
    dir=out `
    program="$ClientExe" `
    action=block `
    enable=yes `
    profile=any `
    description="$desc" | Out-Null
if ($LASTEXITCODE -ne 0) { throw 'netsh failed to add the rule' }

Write-Host "added outbound block for:`n  $ClientExe"
Show-Status
