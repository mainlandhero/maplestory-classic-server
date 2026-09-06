<#
.SYNOPSIS
    Install MapleCW on a machine that has nothing on it.

.DESCRIPTION
    Run this from an ELEVATED PowerShell window, from inside the unzipped payload:

      powershell -ExecutionPolicy Bypass -File "C:\path\to\MapleCW\install.ps1"

    It copies the payload into place, creates the database, creates the first account,
    writes the launcher's configuration, adds the firewall rule that keeps the patched
    client off the internet, and puts a shortcut on the desktop.

    Elevation is needed for exactly one step - the firewall rule. Everything else works
    without it, and -NoFirewall skips that step and the requirement with it. Do not skip
    it casually: the client still contains Nexon's endpoints and a full TLS stack, and
    running a *modified* client with live internet access is how a machine gets flagged.

    Nothing here is authenticated at the game socket. The launcher checks a password
    (argon2id) before it stakes a claim saying which account is playing; the game socket
    itself carries no credentials, so anything that can reach the login port is served as
    whichever account the claim names. That is true of this project everywhere and it is
    true here.
#>
[CmdletBinding()]
param(
    [string]$InstallDir = 'C:\MapleCW',
    [string]$Account,
    [string]$Email,
    [string]$ServerIp = '127.0.0.1',
    [int]$Port = 8484,
    # The sign-in service's port on the SERVER. Must match the -AuthPort the server was
    # started with; 8080 is the default there too. Something else already holding 8080 on
    # the server box is a real and common case, and the two sides have to agree or the
    # launcher's sign-in simply never connects.
    [int]$AuthPort = 8080,
    # The sign-in server's certificate fingerprint, as its console prints it:
    # "sha256:<64 hex>". REQUIRED before this machine can sign in - the launcher refuses to
    # send a password to a server it has not been told to trust. Also readable from
    # auth-cert-fingerprint.txt beside the server's database.
    [string]$AuthFingerprint,
    [switch]$NoFirewall,
    [switch]$NoShortcut,
    [switch]$NoAccount,
    # Install even though the Visual C++ runtime is missing. See the preflight below: the
    # launcher and the GameGuard stub both import VCRUNTIME140.dll, so without it nothing
    # here starts. Only pass this if you are installing now and fetching the runtime later.
    [switch]$SkipRuntimeCheck
)

$ErrorActionPreference = 'Stop'
$payload = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.MyCommand.Path }

function Test-Elevated {
    $id = [Security.Principal.WindowsIdentity]::GetCurrent()
    (New-Object Security.Principal.WindowsPrincipal($id)).IsInRole(
        [Security.Principal.WindowsBuiltInRole]::Administrator)
}

Write-Host ''
Write-Host '=== MapleCW install ===' -ForegroundColor Cyan
Write-Host "payload  $payload"
Write-Host "target   $InstallDir"
Write-Host ''

# ---------------------------------------------------------------- preflight
foreach ($needed in @('maplecw-launcher.exe', 'grap64.dll',
                      'bin\maplecw-login.exe', 'bin\maplecw-world.exe',
                      'bin\maplecw-useradd.exe', 'bin\maplecw-auth.exe', 'gm-handbook', 'data')) {
    if (-not (Test-Path (Join-Path $payload $needed))) {
        throw "the payload is incomplete - $needed is missing. Re-run tools\make-installer.ps1."
    }
}
$hasClient = Test-Path (Join-Path $payload 'client\MapleStory.exe')
if (-not $hasClient) {
    Write-Host 'NOTE: this payload carries no client (built with -NoClient).' -ForegroundColor Yellow
    Write-Host '      The servers will install and run; there is nothing to launch.' -ForegroundColor Yellow
    Write-Host ''
}
# The CLIENT side is NOT built with a static C runtime - the server binaries are, and that
# difference was written up as if it covered both. Measured 2026-09-06 with
# `python tools\pe_import_dlls.py`:
#
#     maplecw-launcher.exe   imports VCRUNTIME140.dll and eight api-ms-win-crt-* stubs
#     grap64.dll             imports VCRUNTIME140.dll, api-ms-win-crt-{runtime,heap}
#
# The api-ms-win-crt-* stubs ship with Windows 10 and 11. VCRUNTIME140.dll does NOT - it
# comes from the Visual C++ Redistributable. The development box has it, which is exactly
# why this has never been seen: it works on the machine that built it. On a clean machine
# the launcher dies at startup with a missing-DLL dialog, and the stub - which is loaded
# INSIDE MapleStory.exe - would fail there instead, which is far harder to read.
#
# So this is a hard stop rather than a warning: a warning thirty lines above a dialog on a
# machine nobody is sitting at is not a warning.
if (-not $SkipRuntimeCheck) {
    $vcruntime = Join-Path $env:SystemRoot 'System32\vcruntime140.dll'
    if (-not (Test-Path $vcruntime)) {
        throw @"
this machine is missing the Visual C++ runtime (VCRUNTIME140.dll).

maplecw-launcher.exe and grap64.dll both import it, so the launcher will not start and the
GameGuard stub will not load inside the client. The game's own MapleStory.exe does not need
it, so having the game installed is no guarantee it is here.

Install "Microsoft Visual C++ 2015-2022 Redistributable (x64)" - vc_redist.x64.exe from
Microsoft - then run this again. This installer is safe to re-run; it updates in place.

  -SkipRuntimeCheck   install anyway, and fetch the runtime before playing
"@
    }
}

if (-not $NoFirewall -and -not (Test-Elevated)) {
    throw @"
not elevated, and the firewall rule needs it.
Either re-run this from an elevated PowerShell window, or pass -NoFirewall and add the rule
later with tools\firewall.ps1 -Add. Do not leave a patched client able to reach Nexon.
"@
}

# ---------------------------------------------------------------- copy
if (Test-Path $InstallDir) {
    Write-Host "$InstallDir already exists - updating in place, leaving maplecw.db alone" -ForegroundColor Yellow
} else {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

Write-Host 'copying...' -ForegroundColor Cyan
# /XF maplecw.db protects an existing database on a re-install: accounts and characters are
# real state and a payload must never overwrite them.
& robocopy $payload $InstallDir /E /NFL /NDL /NJH /NJS /NP /R:1 /W:1 /XF 'maplecw.db' 'install.ps1' | Out-Null
if ($LASTEXITCODE -ge 8) { throw "robocopy failed with code $LASTEXITCODE" }

$bin      = Join-Path $InstallDir 'bin'
$clientDir= Join-Path $InstallDir 'client'
$db       = Join-Path $InstallDir 'maplecw.db'
$useradd  = Join-Path $bin 'maplecw-useradd.exe'
# At the root, beside client\, maplecw.db and grap64.dll. That is exactly the layout the
# launcher detects on its own, so it works even if the config file below is never written.
$launcher = Join-Path $InstallDir 'maplecw-launcher.exe'

# ---------------------------------------------------------------- the first account
# Created by prompting, never by inventing one: an account with a password nobody chose is
# a login that looks real and is not. `maplecw-useradd` reads the password from a hidden
# prompt and refuses to take one as an argument, so it cannot end up in shell history.
if (-not $NoAccount) {
    Write-Host ''
    Write-Host '--- first account ---' -ForegroundColor Cyan
    if (-not $Account) {
        $Account = Read-Host 'Account name (3-24 chars, letters, digits and underscore)'
    }
    if (-not $Email) {
        $Email = Read-Host 'Email for this account (optional - press Enter to skip)'
    }

    $existing = & $useradd --db "$db" --list 2>&1 | Out-String
    if ($existing -match ("(?m)^\s*\d+\s+" + [regex]::Escape($Account) + "\s")) {
        Write-Host "account '$Account' already exists - leaving it alone" -ForegroundColor Yellow
    } else {
        Write-Host "You will be asked for a password twice. It is hashed with argon2id;"
        Write-Host "nothing here stores it in plain text."
        if ($Email) {
            & $useradd --db "$db" $Account --email $Email
        } else {
            & $useradd --db "$db" $Account
        }
        if ($LASTEXITCODE -ne 0) { throw 'could not create the account' }
    }
    Write-Host ''
    Write-Host 'More accounts at any time:' -ForegroundColor Cyan
    Write-Host "  & `"$useradd`" --db `"$db`" <name> --email <address>"
} else {
    Write-Host 'skipping account creation (-NoAccount)' -ForegroundColor Yellow
    Write-Host "create one before launching:  & `"$useradd`" --db `"$db`" <name>"
}

# ---------------------------------------------------------------- launcher config
# The launcher already finds client\, maplecw.db and grap64.dll beside itself, so this file
# is an OVERRIDE rather than a requirement. It is written for the one thing detection cannot
# infer - which server to reach - and the paths are included so `--print-paths` shows an
# answer that came from configuration rather than from a guess.
#
# Paths are written LITERALLY, with single backslashes.
#
# The first version of this doubled them with `-replace '\\', '\\'`, on the assumption that
# the reader needed TOML escaping. It does not - it is a deliberately literal key = "value"
# reader, because a real TOML parser would reject "C:\Users\..." as a bad \U escape. And the
# `-replace` was wrong in its own right: `\` is not special in a .NET replacement string, so
# that expression turns C:\MapleCW\client into C:\\MapleCW\\client. Measured in 5.1, not
# assumed. Win32 collapses the duplicate separators so it happened to work, which is the
# worst kind of bug - it would have sat here looking fine.
# Written as a comment when it was not given, so the file itself says what is missing.
$pinLine = if ($AuthFingerprint) {
    "auth_fingerprint = `"$($AuthFingerprint.Trim())`""
} else {
    "# auth_fingerprint = `"sha256:...`"   <- REQUIRED. The sign-in server prints it at startup."
}
$toml = @"
# Written by install.ps1. The launcher reads this from beside its own executable.
# Paths are literal - single backslashes, no escaping.
client_dir = "$clientDir"
db_path    = "$db"
stub_path  = "$(Join-Path $InstallDir 'grap64.dll')"
server_ip  = "$ServerIp"
port       = "$Port"
auth_port  = "$AuthPort"
$pinLine
"@
Set-Content -Path (Join-Path $InstallDir 'maplecw-launcher.toml') -Value $toml -Encoding ascii
Write-Host ''
Write-Host "wrote $InstallDir\maplecw-launcher.toml"
if (-not $AuthFingerprint) {
    Write-Host ''
    Write-Host 'NO CERTIFICATE FINGERPRINT WAS GIVEN, so this machine cannot sign in yet.' -ForegroundColor Yellow
    Write-Host '  The launcher refuses to send a password to a server it has not been told to'
    Write-Host '  trust. The server window prints "TLS: fingerprint sha256:..." at startup;'
    Write-Host '  either re-run this with -AuthFingerprint <that value>, or edit'
    Write-Host ("  {0}\maplecw-launcher.toml and set auth_fingerprint." -f $InstallDir)
}

# ---------------------------------------------------------------- firewall
if ($NoFirewall) {
    Write-Host 'skipping the firewall rule (-NoFirewall)' -ForegroundColor Yellow
    Write-Host '  the patched client can reach the internet until you add it.' -ForegroundColor Yellow
} elseif (-not $hasClient) {
    Write-Host 'no client in this payload, so no firewall rule to scope to one'
} else {
    $ruleName = 'MapleCW - block patched client outbound'
    $clientExe = Join-Path $clientDir 'MapleStory.exe'
    $null = netsh advfirewall firewall show rule name="$ruleName" 2>&1
    if ($LASTEXITCODE -eq 0) {
        netsh advfirewall firewall delete rule name="$ruleName" | Out-Null
    }
    # **The block must leave the server reachable, and -ServerIp says where the server is.**
    # Windows evaluates block rules before allow rules, so an allow alongside would not help;
    # the block itself is narrowed to "everything except the server". Loopback needs no
    # carve-out (Windows Firewall does not filter it); a private server address opens the
    # three private ranges (the machines on your own network, Nexon is not among them); a
    # public address opens exactly that one address. netsh has no negation, so the remote
    # set is written as the gaps around what is allowed.
    function ConvertTo-IpNumber([string]$ip) {
        $o = $ip.Split('.') | ForEach-Object { [uint32]$_ }
        return ([uint32]$o[0] -shl 24) -bor ([uint32]$o[1] -shl 16) -bor ([uint32]$o[2] -shl 8) -bor [uint32]$o[3]
    }
    function ConvertFrom-IpNumber([uint32]$n) {
        return ('{0}.{1}.{2}.{3}' -f (($n -shr 24) -band 255), (($n -shr 16) -band 255), (($n -shr 8) -band 255), ($n -band 255))
    }
    $ip = $ServerIp.Trim()
    $isLoopback = $ip -like '127.*'
    $isPrivate = ($ip -like '10.*') -or ($ip -like '192.168.*') -or
        ($ip -match '^172\.(1[6-9]|2[0-9]|3[01])\.')
    $remote = $null
    if ($isLoopback) {
        $remote = $null
        $shape = 'ALL remote addresses (loopback is never filtered, so a local server works)'
    } elseif ($isPrivate) {
        $remote = '1.0.0.0-9.255.255.255,11.0.0.0-126.255.255.255,128.0.0.0-172.15.255.255,' +
                  '172.32.0.0-192.167.255.255,192.169.0.0-223.255.255.255'
        $shape = "the public internet only - private addresses such as $ip are reachable"
    } elseif ($ip -match '^\d+\.\d+\.\d+\.\d+$') {
        $n = ConvertTo-IpNumber $ip
        $parts = @()
        if ($n -gt 0) { $parts += ('0.0.0.0-' + (ConvertFrom-IpNumber ($n - 1))) }
        if ($n -lt [uint32]::MaxValue) { $parts += ((ConvertFrom-IpNumber ($n + 1)) + '-255.255.255.255') }
        $remote = $parts -join ','
        $shape = "everything except the server at $ip"
    } else {
        throw "-ServerIp '$ServerIp' is not a dotted IPv4 address; the firewall rule cannot be scoped to it"
    }
    if ($remote) {
        netsh advfirewall firewall add rule name="$ruleName" dir=out action=block `
            program="$clientExe" enable=yes profile=any remoteip="$remote" | Out-Null
    } else {
        netsh advfirewall firewall add rule name="$ruleName" dir=out action=block `
            program="$clientExe" enable=yes profile=any | Out-Null
    }
    if ($LASTEXITCODE -ne 0) { throw 'could not add the firewall rule' }
    Write-Host "firewall rule added, scoped to $clientExe" -ForegroundColor Green
    Write-Host "  it blocks $shape."
    Write-Host '  If the server moves, re-run this with the new -ServerIp, or tools\firewall.ps1 -Add -AllowServer <ip>.'
}

# ---------------------------------------------------------------- shortcut
if (-not $NoShortcut) {
    try {
        $lnk = Join-Path ([Environment]::GetFolderPath('Desktop')) 'MapleCW Launcher.lnk'
        $ws = New-Object -ComObject WScript.Shell
        $s = $ws.CreateShortcut($lnk)
        $s.TargetPath = $launcher
        $s.WorkingDirectory = $InstallDir
        $s.Description = 'Sign in and start MapleCW'
        $s.Save()
        Write-Host "desktop shortcut -> $lnk"
    } catch {
        Write-Host "could not create the shortcut: $_" -ForegroundColor Yellow
    }
}

# ---------------------------------------------------------------- done
Write-Host ''
Write-Host '=== installed ===' -ForegroundColor Green
Write-Host ''
Write-Host 'Start the servers (leave this window open):' -ForegroundColor Cyan
Write-Host "  powershell -ExecutionPolicy Bypass -File `"$InstallDir\start-server.ps1`""
Write-Host ''
Write-Host 'Then run the launcher, sign in, and press Start Game:' -ForegroundColor Cyan
Write-Host "  `"$launcher`""
Write-Host ''
Write-Host 'The launcher signs you in against the database and marks which account is'
Write-Host 'playing. The game socket itself carries no credentials - anything that can'
Write-Host 'reach the login port is served as that account. Local testing only.'
