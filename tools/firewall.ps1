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
    [switch]$AllowLan,
    # Let the client reach ONE public address - the server, when it is out on the internet
    # behind forwarded ports - and keep blocking everything else.
    #
    # -AllowLan is not enough for that case: the server's public address is inside the very
    # ranges it blocks, so an internet client would sit on "Connecting..." exactly as a LAN
    # client did before -AllowLan existed. netsh has no negation, so the remote set is the
    # two ranges on either side of the address (and, with -AllowLan as well, the private
    # gaps too). `-Status` prints the RemoteIP line so the carve-out can be read back.
    [string]$AllowServer
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
    # RemoteIP is the line that says what the block leaves reachable: "Any" is the plain
    # rule, a list of ranges is a LAN or server carve-out. Without it a rule that blocks the
    # server looks identical to one that does not.
    $keep = 'Rule Name|Enabled|Direction|Program|Action|Profiles|RemoteIP'
    $out | Where-Object { $_ -match "^($keep):" } | ForEach-Object { Write-Host "  $_" }
}

# The dotted-quad arithmetic the carve-outs need. netsh takes ranges, not negations, so
# "everything except X" has to be written as the pieces around X.
function ConvertTo-IpNumber([string]$ip) {
    $o = $ip.Split('.') | ForEach-Object { [uint32]$_ }
    return ([uint32]$o[0] -shl 24) -bor ([uint32]$o[1] -shl 16) -bor ([uint32]$o[2] -shl 8) -bor [uint32]$o[3]
}
function ConvertFrom-IpNumber([uint32]$n) {
    return ('{0}.{1}.{2}.{3}' -f (($n -shr 24) -band 255), (($n -shr 16) -band 255), (($n -shr 8) -band 255), ($n -band 255))
}
# Split a list of "a-b" ranges so that $exclude (a single address) is in none of them.
function Remove-AddressFromRanges([string[]]$ranges, [uint32]$exclude) {
    $result = @()
    foreach ($r in $ranges) {
        $lo, $hi = $r.Split('-')
        $l = ConvertTo-IpNumber $lo
        $h = ConvertTo-IpNumber $hi
        if ($exclude -lt $l -or $exclude -gt $h) { $result += $r; continue }
        if ($exclude -gt $l) { $result += ($lo + '-' + (ConvertFrom-IpNumber ($exclude - 1))) }
        if ($exclude -lt $h) { $result += ((ConvertFrom-IpNumber ($exclude + 1)) + '-' + $hi) }
    }
    return $result
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

if ($AllowServer) {
    if ($AllowServer -notmatch '^\d+\.\d+\.\d+\.\d+$') {
        throw "-AllowServer '$AllowServer' is not a dotted IPv4 address"
    }
    $server = ConvertTo-IpNumber $AllowServer
    # Start from "block everything" or, with -AllowLan too, from "block the public internet",
    # and cut the server's address out of whichever range holds it.
    $base = if ($AllowLan) { $PublicOnly.Split(',') } else { @('0.0.0.0-255.255.255.255') }
    $remote = (Remove-AddressFromRanges $base $server) -join ','
    $desc = "Local RE/testing: blocks the patched MapleStory client from everything except " +
            "the server at $AllowServer" +
            $(if ($AllowLan) { ' and private addresses' } else { '' }) + '. Safe to delete.'
    netsh advfirewall firewall add rule `
        name="$RuleName" `
        dir=out `
        program="$ClientExe" `
        action=block `
        enable=yes `
        profile=any `
        remoteip="$remote" `
        description="$desc" | Out-Null
} elseif ($AllowLan) {
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

if ($AllowServer) {
    Write-Host "added outbound block (everything EXCEPT $AllowServer$(if ($AllowLan) { ' and private addresses' })) for:`n  $ClientExe"
    Write-Host '  the server is reachable; Nexon and the rest of the internet stay blocked.'
} elseif ($AllowLan) {
    Write-Host "added outbound block (PUBLIC INTERNET ONLY) for:`n  $ClientExe"
    Write-Host '  private addresses are permitted, so a LAN server is reachable.'
    Write-Host '  Nexon is on the public internet and stays blocked.'
} else {
    Write-Host "added outbound block (ALL remote addresses) for:`n  $ClientExe"
    Write-Host '  loopback is not filtered by Windows Firewall, so a LOCAL server works.'
    Write-Host '  A LAN server will NOT until this is re-added with -AllowLan.'
}
Show-Status
