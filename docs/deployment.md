# Running the server and the client on different machines

Written 2026-08-17, when the owner said the server will eventually live on a homelab box, as the
design the next stage should be written against, so that "move it to the homelab" is a config
change rather than a rewrite. Sections say when they were built; the rest is still design.

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
can be read and checked. **Built 2026-09-06:** `firewall.ps1 -Add -AllowServer <ip>` generates
exactly this (combined with `-AllowLan` it also opens the private ranges), `-Status` prints the
`RemoteIP` line so the carve-out can be read back, and the shipped `install.ps1` derives the
same shape from its `-ServerIp` - loopback blocks everything, a private address blocks the
public internet only, a public address blocks everything but that one address. The two
checklists, `docs/server-machine-checklist.md` and `docs/client-machine-checklist.md`, say
which to run.

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

**Built 2026-09-04, and the packet question is answered.** Two packets carry a channel
address: the login migration `0x0011` (`net::opcode::migrate`) and the Change Channel answer
`0x001A`, both four octets straight into the client's `sockaddr_in`, IPv4 only. Both servers
take `--advertise`, and the rule lives in one place, `crates/net/src/advertise.rs`:

| the client's address is | it is told |
|---|---|
| loopback, RFC 1918 private, link-local, CGNAT `100.64/10` | the address it reached the server on - the accepted socket's local end |
| public | the box's public address, discovered from an echo service at startup and re-checked every ten minutes |

So `bind` stays `0.0.0.0` and nothing is configured for a LAN, a VPN or a Tailscale overlay -
the accepted socket already knows which interface the client reached. Only the internet case
needs discovery, because a NATed box cannot learn its public address any other way, and a
failed discovery is logged (and printed against every migration) rather than guessed at.
`--advertise <ip>` pins one host; `--advertise list` is the pre-2026-09-04 behaviour, kept as
the control. `--channels` now takes bare ports, so the installed `start-server.ps1` carries no
address at all.

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

**Built 2026-09-05: that service is TLS 1.3 to a pinned certificate.** The owner: *"We should not
be sending passwords in plain text."* The service generates a self-signed certificate beside
its database on first start (`crates/auth/src/tls.rs`) and prints its SHA-256; the launcher
accepts exactly that certificate (`crates/tlspin`, one definition of the fingerprint for both
ends) and **refuses to send a password when it has no pin**. `install.ps1 -AuthFingerprint`,
`auth_fingerprint` in `maplecw-launcher.toml`, or `auth-cert-fingerprint.txt` beside the
launcher; a dev checkout finds the dev service's file at the repo root. `tiny_http` went with
the change - its TLS feature pins rustls 0.20, end-of-life with CVE-2024-32650 unfixed - and
the five endpoints are served by a bounded hand-rolled HTTP/1.1 loop over rustls 0.23. The
alternative the owner first suggested, sending the argon2 hash instead of the password, was not
done: it is pass-the-hash, and it turns a database leak into instant login for every account.

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

* ~~**`crates/auth` needs a bind that is not hardcoded loopback** and TLS once it is off-box.~~
  Both done: `--bind 0.0.0.0` and TLS with a pinned self-signed certificate (no CA needed -
  see the "Built 2026-09-05" paragraph above).
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
  launcher  ── TLS, pinned cert ───▶ auth        (crates/auth,   TCP 8480)
  client    ── Maple protocol ─────▶ login       (crates/login,  TCP 8484)
                                     channel N   (crates/world,  TCP 8485 + N)
                                       │ dials, loopback only
                                       ▼
                                     hub         (crates/world::link, maplecw-chat, TCP 8483)
                                     store       (crates/store, SQLite file)
```

The hub (2026-09-15) is what crosses channels: each channel process keeps one TCP
connection to `maplecw-chat`, which relays character-addressed packets to the channel that
hosts the character, keeps the directory of who is online where, and serialises party
requests so every channel's party replica applies the same sequence. It is server-internal -
bound to loopback, never forwarded - and a channel that cannot reach it runs alone, per
channel, as before. It is not a database and holds nothing that survives a restart; the
channels re-announce their players when it comes back.

```text
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

## The client payload carries the hybrid WZ archives

Since 2026-09-10 the client's `Data\` is not Nexon's classic data as shipped: 26 archives
are rebuilt by `tools\backport_install.py` from the pristine originals (kept beside each as
`.bak`) with the Signature Style Collection merged in - 206 modern items, the box under a
classic id family, the weapon covers' type links, Himmel's worn-item effect (a new
`Effect/ItemEff.img`), the hair-hats' slot type and the face coupons' id family. **Every one
of those is client data, not server code**, so a server release without the matching client
release, or the reverse, is a mismatch the wire cannot detect: the server hands out an item id
the client has no node for, or prices a row the client's Cash Shop does not show.

What keeps the two halves together:

* `make-installer.ps1` runs `backport_install.py --check` before it copies the client: a
  fresh build into a scratch directory, and every installed archive must hash equal to it.
  A stale install fails the packaging, on the dev box, in two minutes.
* `--install` regenerates `gm-handbook\` (items, equips, item data, commodity, the store's
  item rules) after the copy, and both packagers refuse a handbook older than the installed
  `String_000.wz`. The world server debits and names from those tables.
* The `.bak` originals are excluded from the client payload; they are the dev box's undo.
* Anything that renumbers an item (`RENAMES` in the installer) has a twin in
  `store::inventory::ITEM_ID_RENAMES`, run on every database open - so a server upgrade
  renumbers what players already hold, on the homelab box too.

So the release order is: install the backport (client closed), package the server, package
the client, ship both. Shipping one of them is not a release.

**Every change the installer makes, and how to make them again on a new Classic World version,
is `docs/wz-changes.md`.**

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
