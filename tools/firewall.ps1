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
    powershell -ExecutionPolicy Bypass -File tools/firewall.ps1 -Status
    powershell -ExecutionPolicy Bypass -File tools/firewall.ps1 -Add
    powershell -ExecutionPolicy Bypass -File tools/firewall.ps1 -Remove
#>
[CmdletBinding()]
param(
    [string]$ClientExe,
    [switch]$Add,
    [switch]$Remove,
    [switch]$Status,
    # Let the client reach PRIVATE addresses, and keep blocking the public internet.
    #
    # Needed the moment the server stops being on this machine. The ordinary rule blocks
    # outbound to RemoteIP=Any, and Windows Firewall does not filter loopback - which is why
    # 127.0.0.1 has always worked and why a LAN server will silently not. The client sits on
    # "Connecting..." and nothing anywhere says the firewall did it.
    #
    # An `action=allow` rule alongside would NOT fix it: Windows evaluates block before
    # allow, so the block still wins. The only way is to narrow what the block covers, which
    # is what this does - four ranges that together are "everything except 10/8, 172.16/12,
    # 192.168/16 and loopback".
    #
    # It is a real widening and it is not the default. The client can now reach any private
    # address, which on a home network is a handful of machines you own. Nexon is on the
    # public internet and stays blocked, which is the property that matters.
    [switch]$AllowLan
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

# Everything EXCEPT 10/8, 172.16/12, 192.168/16 and 127/8. Written as the four gaps between
# them, because netsh has no negation.
$PublicOnly = '1.0.0.0-9.255.255.255,11.0.0.0-126.255.255.255,128.0.0.0-172.15.255.255,' +
              '172.32.0.0-192.167.255.255,192.169.0.0-223.255.255.255'

if ($AllowLan) {
    $desc = 'Local RE/testing: blocks the patched MapleStory client from the public ' +
            'internet, but permits private addresses so it can reach a LAN server. ' +
            'Safe to delete.'
    netsh advfirewall firewall add rule `
        name="$RuleName" `
        dir=out `
        program="$ClientExe" `
        action=block `
        enable=yes `
        profile=any `
        remoteip="$PublicOnly" `
        description="$desc" | Out-Null
} else {
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
}
if ($LASTEXITCODE -ne 0) { throw 'netsh failed to add the rule' }

if ($AllowLan) {
    Write-Host "added outbound block (PUBLIC INTERNET ONLY) for:`n  $ClientExe"
    Write-Host '  private addresses are permitted, so a LAN server is reachable.'
    Write-Host '  Nexon is on the public internet and stays blocked.'
} else {
    Write-Host "added outbound block (ALL remote addresses) for:`n  $ClientExe"
    Write-Host '  loopback is not filtered by Windows Firewall, so a LOCAL server works.'
    Write-Host '  A LAN server will NOT until this is re-added with -AllowLan.'
}
Show-Status
