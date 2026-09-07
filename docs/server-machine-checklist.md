# What goes to the server computer

Written 2026-09-06. Everything here is what `tools/package-server.ps1` builds and what
`tools/installer/SERVER-README.txt` (shipped inside the package) explains at length. This is
the short list; that README is the long one.

## 1. One zip, built on this machine

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\package-server.ps1"
```

produces **`out\MapleCW-server.zip`** (about 8 MB). Copy that one file. Its contents:

| in the zip | what it is |
|---|---|
| `bin\maplecw-login.exe` | login and character select, listens on **TCP 8484** |
| `bin\maplecw-world.exe` | one channel per process, **TCP 8485, 8486** (two channels by default; `-Channels N` adds one port each) |
| `bin\maplecw-auth.exe` | sign-in over TLS, **TCP 8480**; the launcher talks to this |
| `bin\maplecw-useradd.exe` | accounts, GM flag, registration and recovery codes from the console |
| `gm-handbook\` | game tables generated from the client's WZ (maps, mobs, items, portals, footholds, quests). The world server reads these by relative path |
| `data\` | authored server data: `drops.txt`, `exp-curve.txt`, `npc-dialogue.txt`, `quest-scripts.txt`, `shops.txt` |
| `start-server.ps1`, `start-servers.cmd` | the run script; the `.cmd` is the double-click. **That window is the server.** |
| `SERVER-README.txt` | the full operator notes: setup, ports, the two things that go wrong, the security posture |

The binaries are built with the C runtime linked in and import only DLLs that ship with
Windows. **Nothing has to be installed on the server box** - no Rust, no .NET, no Visual C++
Redistributable, no database engine (SQLite is compiled in). The packaging script asserts the
whole import list, not just the absence of `VCRUNTIME140.dll`, so a new dependency on
something a clean box may lack fails the packaging run rather than the deployment.

**This is true of the server only.** The launcher and `grap64.dll` are not built that way and
do need the Visual C++ Redistributable - see [the client checklist](client-machine-checklist.md).

## 2. Not in the zip, and deliberately

| | why |
|---|---|
| the game client and its WZ data | the **client** machine's payload |
| `maplecw-launcher.exe`, `grap64.dll` | client side too |
| `maplecw.db` | made on the server on first use. If you want THIS machine's accounts and characters over there, see §4 |
| `auth-cert.pem`, `auth-key.pem`, `auth-cert-fingerprint.txt` | generated on the server's **first start**, beside the database. Do not copy this machine's; the server should have its own |

## 3. On the server, in order

1. Unzip anywhere (`C:\MapleCW-server` is fine).
2. Make your own account and give it GM (leading `&` required in PowerShell):
   ```
   & ".\bin\maplecw-useradd.exe" --db ".\maplecw.db" <name> --email <address>
   & ".\bin\maplecw-useradd.exe" --db ".\maplecw.db" --gm <name>
   ```
   Everybody else registers through the launcher with a single-use code you mint in game
   (`!registrationcode`) or here (`--registration-code`).
3. Open the inbound ports on the server's own Windows Firewall:
   ```
   netsh advfirewall firewall add rule name="MapleCW server" dir=in action=allow protocol=TCP localport=8480,8484,8485,8486
   ```
4. Double-click `start-servers.cmd`. Leave the window open; closing it stops everything.
5. Read the fingerprint the window prints on the first start:
   ```
   TLS: fingerprint sha256:<64 hex characters>
   ```
   It is also in `auth-cert-fingerprint.txt` beside the database. **Every client needs this
   value** (see the client checklist). Delete the two `.pem` files and the next start mints a
   new pair, and every client's pin stops matching until it is updated.
6. On the router, forward **TCP 8480, 8484, 8485 and 8486** to the server (one more per extra
   channel). Nothing uses UDP.

The server needs no address configured. It binds `0.0.0.0` and decides **per connection**
what host to tell the client: a LAN, VPN or Tailscale client is told the address it reached
the server on, an internet client is told the box's public address, which the server
discovers at startup (`checkip.amazonaws.com`, `api.ipify.org`, `icanhazip.com`, in turn) and
re-checks every ten minutes. `login.log` says what it found in its first lines; if it says
`UNKNOWN` (no internet at startup), pin it:
```
powershell -ExecutionPolicy Bypass -File ".\start-server.ps1" -Advertise <public ip>
```

## 4. Bringing this machine's database

Stop the servers on both machines first. Copy **all three**:

```
maplecw.db   maplecw.db-wal   maplecw.db-shm
```

SQLite runs in WAL mode; the last writes live in the `-wal` file until a checkpoint. Copying
the `.db` alone opens fine and is simply missing the newest accounts and characters, which on
2026-08-29 read as corruption for several minutes.

## 5. What is and is not protected, in one paragraph

Sign-in is TLS to a certificate every launcher pins, and a wrong password is refused. The
game socket carries no credential: a connection is served as an account only if the server
can tie it to a live launcher sign-in (the one-time token the launcher hands the client, the
process that owns the socket on the same machine, or the address), and a connection it cannot
attribute is refused with a login failure. Forwarded to the internet, that attribution is the
whole posture; `SERVER-README.txt`'s last section says the same at length.
