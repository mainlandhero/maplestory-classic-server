# What a client machine needs, beyond MapleStory.exe and its WZ data

Written 2026-09-06. The game client itself - `MapleStory.exe` and the `Data\` folder with the
`.wz` files - is assumed to be on the machine already, as a **copy** you are willing to have
patched. Everything else is below. The client payload `tools\make-installer.ps1` builds
(`out\MapleCW\`) carries all of it plus `install.ps1`, which does steps 2 to 4 in one go.

## 1. The launcher: `maplecw-launcher.exe`

Built by `cargo build --release -p launcher` into `target\release\`, or taken from
`out\MapleCW\`. It is the whole client-side program: it signs in, patches the client, starts
it, and gets out of the way. It asks for administrator on start because the game client
demands it, and accepting there means the client does not ask again.

**It needs the Visual C++ runtime, and the game does not.** This section said the opposite
until 2026-09-06; that was wrong, and it was wrong in the direction that only shows up on
somebody else's machine. Measured with `python tools\pe_import_dlls.py`:

| binary | imports `VCRUNTIME140.dll`? |
|---|---|
| `maplecw-launcher.exe` | **yes**, plus eight `api-ms-win-crt-*` stubs |
| `grap64.dll` (the injected stub) | **yes** |
| `MapleStory.exe` | no |
| the four server binaries | no - those *are* built with a static C runtime |

The `api-ms-win-crt-*` stubs are part of Windows 10 and 11. `VCRUNTIME140.dll` is not: it
comes from **Microsoft Visual C++ 2015-2022 Redistributable (x64)** (`vc_redist.x64.exe`).
The development box has it, which is why this was never seen - the classic "works on the
machine that built it". On a clean machine the launcher dies at startup with a missing-DLL
dialog; worse, `grap64.dll` is loaded *inside* `MapleStory.exe`, so a missing runtime there
reads as the client failing rather than as a missing dependency. Having MapleStory installed
proves nothing, because the game itself does not import it.

`install.ps1` now refuses to run on a machine without it and says so by name
(`-SkipRuntimeCheck` overrides). Installing by hand: check for
`C:\Windows\System32\vcruntime140.dll` before anything else.

What it does to the client folder at **Start Game**, so nobody has to do it by hand:

| step | what | why |
|---|---|---|
| install the stub | copies `grap64.dll` into the client folder, backing the original up as `grap64.dll.orig` | the stub is compiled into the launcher; a `grap64.dll` on disk beside it is only a convenience |
| disable GameGuard | renames the client's `grap\` folder to `grap.disabled` | so `NGService.exe` and the `BlackCat64.sys` driver can never start |
| write the hook's markers | `maplecw-hook.identity` (the one-time launch token), the multi-client marker, the log path | the login server uses the token to attribute this connection to your sign-in |
| launch | `MapleStory.exe -NXLDEBUG <server ip> <port>` from the client folder | the working directory matters: the hook reads its markers relative to it |

**Point it at the right folder once.** The *Game folder* box, or Browse, picks the
`MapleStory.exe`; the choice is remembered in `maplecw-launcher.remembered.toml` beside the
launcher and used on every later start. Never point it at the original Nexon install.

## 2. Three facts about the server: `maplecw-launcher.toml`

Beside the launcher. `install.ps1` writes it; by hand it is four lines:

```
server_ip        = "203.0.113.5"        the server's LAN address, or its public one from outside
port             = "8484"               the login server
auth_port        = "8080"               the sign-in service - MUST match the server
auth_fingerprint = "sha256:<64 hex>"    REQUIRED - see below
```

**`auth_port` must match the server.** 8080 is a popular port and a server box may already
have it spoken for, in which case its `start-servers.cmd` sets `AUTHPORT` to something else
and every client has to follow. A mismatch is silent: the launcher just never connects.
`install.ps1 -AuthPort <n>` writes it.

**The fingerprint is not optional.** The launcher pins the sign-in server's certificate and
refuses to send a password to a server it has not been told to trust. The value is what the
server window prints on its first start (`TLS: fingerprint sha256:...`) and what sits in
`auth-cert-fingerprint.txt` beside the server's database. Either put it in the toml, or copy
that `.txt` file beside the launcher, or run `install.ps1 -AuthFingerprint <value>`. If the
server's certificate is ever regenerated, this value changes and every client needs the new
one.

The channel address is **not** configured here. The server tells the client which host to
dial when it enters the world, and picks a host the client can actually reach.

## 3. The firewall rule, and the one way it goes wrong

The patched client still carries Nexon's endpoints, so an outbound block scoped to
`MapleStory.exe` is installed on every client machine. **Windows evaluates block rules
before allow rules**, so the block itself has to leave the server reachable; an "allow"
beside it does nothing. Which shape depends on where the server is:

| server is | run (elevated) | what it blocks |
|---|---|---|
| this same machine | `install.ps1` (default), or `tools\firewall.ps1 -Add` | everything - loopback is never filtered, so a local server works |
| on your LAN | `install.ps1 -ServerIp 192.168.x.y`, or `firewall.ps1 -Add -AllowLan` | the public internet only |
| on the internet (forwarded ports) | `install.ps1 -ServerIp <public ip>`, or `firewall.ps1 -Add -AllowServer <public ip>` | everything except that one address |

Symptom of getting this wrong: the client sits on *Connecting...* forever and says nothing.
`firewall.ps1 -Status` prints the rule's `RemoteIP` line, so the carve-out can be read back.

## 4. An account

Nobody is pre-created on a client. Either the server's operator makes the account with
`maplecw-useradd` on the server, or the player registers in the launcher's **Register** tab
with a single-use code the GM mints in game (`!registrationcode`). Passwords are at least 8
characters with a letter and a digit. A forgotten password is a **Forgot password** tab plus
a recovery code (`!recoverycode <email or name>`).

## 5. Ports the client must be able to reach

All TCP, all outbound from the client: **8080** (sign-in), **8484** (login), **8485, 8486**
(the two default channels; one more per extra channel). Nothing uses UDP. On a home router
nothing needs opening on the client's side.

## 6. Not needed on a client, and shipped only by accident

`out\MapleCW\` also contains `bin\`, `data\`, `gm-handbook\` and `start-server.ps1`. Those are
the **server's**. A client machine never reads them; they are in that payload because the
installer used to set up a local server too. Leave them out when copying to a client.

## 7. What is protected, in one sentence

The password crosses only inside TLS to a pinned certificate; the game socket carries no
credential, and the server serves this connection as your account because the launcher's
token, the process that owns the socket, or your address ties it to your sign-in.
