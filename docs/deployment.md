# Running the server and the client on different machines

Written 2026-08-17, when the owner said the server will eventually live on a homelab box. Nothing
here is built yet - this is the design the next stage should be written against, so that
"move it to the homelab" is a config change rather than a rewrite.

Everything marked **measured** comes from a run; everything marked **needs measuring** is a
guess with a way to check it.

## The one thing that will break first

`tools/firewall.ps1` adds a **blanket outbound block** scoped to
`client-patched\MapleStory.exe`, and its own docstring says why that has been harmless:

> Loopback is not filtered by Windows Firewall, so 127.0.0.1 traffic to our own local server
> is unaffected.

**That stops being true the moment the server is on another machine.** Our game traffic
becomes ordinary outbound traffic and gets caught by the same rule that blocks Nexon. The
client would fail to connect at all, and it would look like the server being down.

The rule has to keep blocking everything *except* the homelab. Windows Firewall applies
block rules ahead of allow rules, so adding an allow rule alongside the block does not work -
the block still wins. The workable shape is a block rule whose remote address is the
complement of the server:

```text
remoteip=0.0.0.0-<server-1>,<server+1>-255.255.255.255
```

Ugly, but exact, and `netsh advfirewall firewall show rule ... verbose` prints it back so it
can be read and checked. `firewall.ps1` should grow a `-AllowServer <ip>` parameter that
generates this, and `-Status` should print which address is carved out.

**The twenty Nexon addresses stay blocked**, which is the point - and which means
`-SkipNetCheck` is still required. See `STATUS.md`: the client `__fastfail`s when its
reachability check finds none of them, and moving our server does not change that.

## Bind address is not advertise address

The trap in every multi-machine game server: a service binds `0.0.0.0` but hands the client
an address to connect to, and that address is whatever was convenient on the server. On the
same box `127.0.0.1` works and hides the bug; from another machine it fails.

So every service needs **two** addresses in its config:

| setting | meaning |
|---|---|
| `bind` | what the socket listens on, e.g. `0.0.0.0:8484` |
| `advertise` | what gets written *into packets* for the client to dial, e.g. `192.168.1.50` |

**Where this bites us specifically is not yet known**, and it is worth finding out before
building. The world list (`0x000B`) we send today carries no IP - the body is a world id, a
name, a channel count and channel names (`docs/opcodes.md`). In MapleStory the channel
address travels with *migration*, and our candidate for that packet is inbound `0x0011`
(`SelectCharacterResult`), which is Stage 4 and unimplemented.

**Needs measuring:** whether `0x0011`, or whatever answers the select-character request
`0x0078`, carries an IP and port. Decode it before designing around it. If it does, that
field is the first consumer of `advertise`.

## Credentials never travel on the game socket

The Maple protocol's cipher is obfuscation, not security: the key is recoverable from the
running client, and we recovered it. Anything sensitive on that socket is in the clear for
practical purposes.

`crates/auth` already exists and already has the right shape:

| endpoint | who calls it | for |
|---|---|---|
| `POST /login` `{username, password}` | the **launcher** | exchange credentials for a session token |
| `POST /consume` `{token}` | the **login server** | validate once, single-use |
| `POST /verify` `{token}` | anything | check without consuming |
| `GET /health` | monitoring | liveness |

Tokens are 32 bytes from the OS CSPRNG and the database stores only their hash; passwords
are argon2id with a per-password salt. Nothing in the crate logs a password or a token.

**That flow was: launcher → auth → token → client launch argument → game socket → login
server → `/consume`. The middle of it does not exist and cannot be built.** Measured
2026-08-18 and recorded in `docs/launcher.md`: `-NXLDEBUG` does route launch arguments into
the client's session array at config `+0x90`, but **nothing puts them on the wire** - six
distinguishable tokens were passed and outbound `0x0073` came back byte-identical to a run
without them. Passing them also broke the run with a "trouble connecting" dialog. So there is
no transport for a token on the game socket, and `/consume` has nothing to consume.

**What replaced it, 2026-08-28**, is not a smaller version of the same idea. The launcher
authenticates and then writes a **login claim** to the database; the login server resolves an
account from that claim on every accept. See `docs/launcher.md`. The password still never
touches the game protocol - but neither does anything else, and the honest statement is that
the game socket is unauthenticated rather than weakly authenticated.

**This is the single biggest gap between the local setup and an off-box one.** On one machine
the launcher opens the SQLite file directly and the claim is machine-local state, which is
sound because the only party that can stake one has already authenticated to the launcher. On
a network that stops being true twice over: a claim is **global to the database**, so two
players staking claims would fight over who the next connection is; and anything that can
reach the login port is served as whatever the current claim names.

So for a real multi-machine deployment the remaining work is:

* **`crates/auth` needs a bind that is not hardcoded loopback** - its own doc comment says it
  hardcodes `127.0.0.1` - and TLS once it is off-box. A homelab CA with a certificate for the
  server name is enough. Do not put it on the internet.
* **The launcher needs to authenticate over that service rather than against a local database
  file**, because a client machine will not have one. It has a server IP field already; today
  that field directs the *game* connection only.
* **The claim needs to stop being global.** There is nothing on the game socket to key it by,
  which is the whole difficulty - the honest options are one login port per player, or the
  `grap-stub` identity-string route in `docs/launcher.md`, and neither is pretty.

Until those exist, treat the installed configuration as **one player per server box**.

## Machine identity is not an authorisation input

**Measured:** outbound `0x0073` (session identity) carries a MAC address list and a machine
id - `AA-BB-CC-DD-EE-FF` and `AABBCCDDEEFF_DEADBEEF` on the owner's machine - and outbound
`0x0078` (select character) carries them again.

Record them, do not gate on them. They change with the machine, and the whole point of this
document is that the client machine is not fixed. If they are ever wanted for anything, bind
them to the session token at issue time rather than to the account.

## Topology

Start with **everything on one server box**, because the split that matters is client
machine versus server machine, not service versus service:

```text
  The owner's PC                          homelab
  ---------                          -------
  launcher  ── HTTPS ──────────────▶ auth        (crates/auth,   TCP 8443)
  client    ── Maple protocol ─────▶ login       (crates/login,  TCP 8484)
                                     channel     (crates/channel, later)
                                     store       (crates/store, SQLite file)
```

`crates/store` is SQLite. That is fine for a single box and it is **not** fine over a network
share - SQLite's locking does not survive one. So: while everything is co-located, let the
services share the file; the moment a service moves to its own machine, it stops touching the
file directly and talks to whichever service owns it. Designing `store` behind a trait now
keeps the eventual swap to Postgres cheap, and costs nothing today.

## Configuration

Today the port is a PowerShell parameter and the server address is a client launch argument.
That does not survive a move. Two files:

**Server** - `maplecw.toml` next to the binaries:

```toml
[auth]
bind = "0.0.0.0:8443"
[login]
bind = "0.0.0.0:8484"
advertise = "192.168.1.50"      # what goes into packets, not what we listen on
[store]
path = "/var/lib/maplecw/maplecw.db"
```

**Client** - `launcher.toml` next to the launcher:

```toml
server = "192.168.1.50"
port = 8484
auth = "https://maplecw.homelab.lan:8443"
client = "C:/MapleCW/client-patched"
```

## What stays on the client machine

All of it is client-side and the launcher owns it - see `docs/launcher.md`:

* `grap64.dll`, our GameGuard stub;
* the runtime patches (`-SkipNetCheck`, the login-dialog suppression, the creation flag);
* the firewall rule, which is a property of the client machine.

The server should never need to know which patches are in force. It does need to be told
when a result is only reachable *because* of one - that is a reporting rule for us, not a
protocol feature.

## A checklist for the first off-box run

Cheap to do, and each one has failed for someone before:

1. `GET /health` on the auth service **from the client machine**, not from the server.
2. The firewall rule carves out the server: `netsh ... show rule ... verbose` prints the
   complement ranges.
3. The client still `__fastfail`s without `-SkipNetCheck` - if it stops doing that, the
   firewall carve-out is too wide and the client is reaching Nexon.
4. `probe.log` (or the login server's log) shows the connection arriving from the client's
   LAN address rather than `127.0.0.1`.
5. Kill the connection mid-session and confirm the client's behaviour is the same as it was
   on loopback; a LAN adds real disconnects that loopback never produced.
