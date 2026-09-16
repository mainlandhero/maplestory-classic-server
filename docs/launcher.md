# The launcher

Written 2026-08-17 as a design. **Built 2026-08-28** as `crates/launcher` /
`maplecw-launcher`, after the owner asked for "a separate simplified login client that launches
before the regular MapleStory client to establish account and session… We need to have more
than 1 account to be able to connect to this server", and then for the shape: "input of email
and password and server IP… two buttons, Login then Start Game. Login validates the session
and then enables the Start Game button. The Start Game button should invalidate the game
guard, stub the grap.dll and do whatever it needs to do."

The original brief was "a minimal launcher for the client to perform any patching you need to
do in order to neutralize GameGuard and skip net check when the client starts, so it is more
professional looking".

It replaces `tools/test-charselect.ps1` for *using* the client; that script stays as the
instrumented harness for *investigating* it, and the two should keep working side by side.
`tools/test-server.ps1 -Launcher` starts the servers and hands over to it.

**Read "How more than one account actually works" below before anything else** - it is the
part that changed, and the two options this document used to end on were both wrong.

## What it does, in order

1. **Find and verify the client.** Hash `MapleStory.exe` and compare against a known build.
   This is not ceremony: every patch we apply is at an **absolute virtual address**
   (`0x1415db360`, `0x141b2a280`), so applying them to a different build writes into
   whatever happens to be there. Refuse to launch an unrecognised build and say so.
   **Built, 2026-09-13, as `crates/launcher/src/integrity.rs`** - though not the hash half.
   It reads the PE import tables (normal and delay-load) of every module in the folder,
   follows them through the folder's own modules, and resolves each name against the client's
   real search path. It reports two things a person on another machine cannot otherwise
   discover: a DLL that is **missing**, and a DLL that is **borrowed** from that machine's
   PATH rather than shipped - the one that is invisible on the box where the client works.
   29 modules, 56 ms on the real client. It reports and never refuses, and it runs on every
   Start Game; `--check-client [folder]` asks without launching. Blind to `LoadLibrary` and
   registry lookups, and it says so in its own output.
2. **Install the stub.** Copy `grap64.dll` into the client directory if it is missing or
   stale. This is what neutralises GameGuard - the client loads our DLL instead of the real
   one, and no service or driver starts.
3. **Write the patch config** the hook reads at startup (see below).
4. **Authenticate.** `POST /login {username, password}` to the auth service over TLS; receive
   a session token. Show the service's own error on failure - "wrong password" and "auth
   service unreachable" must not look the same.
5. **Launch.** `ShellExecute` (the client has an elevation manifest, so this raises UAC) with
   `-NXLDEBUG <server> <port>` plus the session token.
6. **Get out of the way**, leaving a way to see the hook log if something went wrong.

**0, since 2026-09-16 - before all of the above at Start Game: update itself.** The owner: *"The
launcher that we have should have the ability to patch itself should we need to."* The server
publishes its own copy of `maplecw-launcher.exe` (`bin\` of the server package,
`maplecw-auth --launcher`, endpoints `/launcher/manifest` and `/launcher/file` -
`auth::launcherpatch`). The launcher hashes its own executable, and when the two differ it
downloads the server's, verifies size and SHA-256 before writing, renames the running
executable to `.old`, renames the new one onto its path, starts it with `--updated-from
<old>` and closes; the new one deletes the old file once the process has gone
(`launcher::selfupdate`). Sign in again in the new window - the claim belonged to the old
process and the password is never on disk. A server that publishes no launcher is a warning,
not a refusal: an old server must not lock players out of a launcher that was working.
**The update carries the stub with it** (2026-09-16): `grap64.dll` beside the launcher used
to win over the copy compiled in, so a guard-page change shipped in a new launcher reached no
installed client until the setup zip was re-run. Now the newer of the two wins - the file if
it was modified after the launcher's executable (a developer's fresh build), otherwise the
launcher's copy, which is written over the file (`stub::resolve`, "rewritten with the
launcher's copy" in the log). A self-updated launcher is newer than everything beside it.

## Replace the marker files with one config

The hook is switched on today by files written next to the client by PowerShell:
`maplecw-hook.enable`, `maplecw-hook.probe`, `maplecw-hook.session`, and it reads
`MAPLECW_HOOK_LOG` from the environment when that survives the launch. That grew out of
debugging - including one bug where the log landed in a file called `on` because the
environment *did* survive - and it is not what a launcher should write.

One file, `maplecw.toml`, next to the client:

```toml
[patch]
skip_net_check = true       # the client bug; see below
suppress_login_dialog = true
enable_creation = true      # a stand-in for protocol we have not implemented
[log]
path = "maplecw-hook.log"
level = "normal"            # "verbose" turns on the watch/probe machinery
```

`grap-stub` reads it in `install_once`. Keep the existing marker files working while the
harness still uses them - retire them when `test-one.ps1` moves over, not before.

## The patch inventory, and how each one retires

This matters more than it looks. Some of these are permanent workarounds for a client defect;
others are us standing in for a server we have not written. They should not be treated alike,
and the config should say which is which.

| patch | what it does | why | retires when |
|---|---|---|---|
| `grap64.dll` stub | GameGuard never starts | the whole project depends on it | never - this is the point |
| `skip_net_check` | returns from `FUN_1415db360` on entry | **client bug**: a firewalled reachability check overruns its own stack buffer and `__fastfail`s. `docs/` and `STATUS.md` have the full story | never, unless the client is allowed to reach Nexon - which it must not be |
| `suppress_login_dialog` | forces `rdx=0` into `FUN_141b2a280` | the "trouble logging in" dialog blocks the tick that enables Login | **when we find what the server should send instead.** The client computes result code 12; something in our login reply is wrong or missing, and this hides it rather than fixing it |
| `enable_creation` | calls `FUN_140c9e230` | the real service sets that flag from virtualised code | **when we find the packet that sets it** - genuine remaining protocol work |
| `mode=2` | patches the login-stage mode byte | was believed to stop the auto-login | **possibly already retired.** Measured 2026-08-17: it is applied ~100ms *after* the client has already auto-logged in, so it cannot be doing that job. Test whether anything breaks without it - a cheap experiment now that runs are cheap. **But note the next row: with the mode at 2, the login handler that runs never refills the select screen** |
| `selectfill` (`grap_stub::session::refresh_select_after_dispatch`) | after every `0x0010` dispatch, if the character-select UI already exists, calls the client's own `FUN_141177e40(selectUi)` | **the blank character-select screen, confirmed fixed 2026-09-12.** On a fast start the client builds the select UI 30 ms after the login request, inside the `0x0032` dispatch, from an *empty* character list, and the mode-2 login handler never refills it; the mode-5 handler makes exactly this call. Nine of nine blank logins measured were this ordering; four of four rescued. `research/charselect-avatar-fade-race.md` | **never while `mode=2` stands**, and probably never: the refill is the client's own code path. Off for one launch with `selectfill=off` (a pin). Never watch `141177e40` in `-Probe` - the int3 is the byte its guard reads |

A launcher that silently applies all six is worse than one that says what it did. Print the
list at startup and write it to the log, so a result is never reported without the patches
that made it reachable.

## Keep the patching at runtime

All of it is applied in-process by `grap-stub` (an `int3`, a vectored handler, a byte
rewritten in memory). The alternative - a patched `MapleStory.exe` on disk - is tempting for
a launcher and should be resisted:

* the on-disk client stays byte-identical to what Nexon shipped, so the build hash in step 1
  keeps meaning something;
* patches are versioned with our code rather than baked into a 76 MB binary;
* restoring the original client is deleting one DLL and one config.

**And we know runtime patching is safe here**: the `-NoPatch` control run died at 36.70s with
no patches at all, against 36.96s and 36.89s with the full set. Nothing in the client reacts
to them.

One improvement worth making while moving this into a launcher: `skip_net_check` currently
uses the diagnostic mechanism - an `int3`, an exception, a single-step re-arm on every call.
For a permanent patch, writing `0xC3` (`ret`) at `FUN_1415db360`'s entry is a better fit: the
function returns void, so a bare `ret` is a correct no-op, and it costs no exception at all.
Keep `:ret` for investigation; ship the byte.

## The session token: the route this design assumed is dead

**Measured 2026-08-18, and it changes this document.** `-NXLDEBUG` does route launch
arguments from the third onward into the client config's six-slot session array at `+0x90`.
**But nothing puts them on the wire.** Six distinguishable tokens were passed and outbound
`0x0073` came back byte-identical to a run without them:

```text
26B  05000000 0000 aabbccddeeff deadbeef 00000000 764d0000 0000
     mode=5   ""   MAC          machine id
```

No token text anywhere in the capture. So **the launcher cannot hand the server a token
through `0x0073`**, and the "launcher authenticates, passes the token, login server calls
`/consume`" chain has no transport.

**Passing those tokens also broke the run** - a "trouble connecting" dialog immediately after
the splash, from a path that is *not* the `FUN_141b2a280` we suppress (its watch was armed
and never fired). Do not pass them in ordinary runs.

### What was left, and neither was taken

1. **One login server per account, each on its own port**, with the launcher choosing the
   port after it authenticates.
2. **Write the identity string from `grap-stub`.** `0x0073`'s second field is a `char *` at
   `DAT_143ac1898+0x1b8`, sent as a **zero-length string because nothing computes it**. The
   stub is already in-process and could write a token there before the packet is built. That
   would carry a real token to a real `/consume` - but it is a **client patch standing in for
   a session**, and it belongs in the patch inventory above with a retirement column, not
   described as authentication.

The one true sentence in that pair was the one after it: *"Either way `Session` should take
its account from a resolver rather than from `Config`, so whichever route wins is one
function."* That is what was built, and it turned out that once the resolver existed neither
option was needed.

### Sign out revokes the claim - 2026-09-16

The owner: *"Can we make sign-out button actually revoke the claim please"* and *"Make sure it also
disables the 'Start Game' button once signed out."* Until then the button was local: it cleared
the sign-in and said in its own comment that it could not revoke anything. Now the sign-in
keeps the **session token** the service issued (in memory only, `Debug`-redacted), and Sign
out does three things in this order: drops the sign-in synchronously - Start Game is gated on
it, so the button greys the instant Sign out is pressed, before any network - then `POST
/logout {token}` in a worker (`auth::AuthService::logout` ->
`store::clear_login_claim_for_token`), then removes the client credential file this launch
wrote into the game folder, since the claim it belonged to is gone. The answer is one of three
sentences: revoked (N claims), unknown (expired or superseded - nothing to do), or failed
(unreachable or a server that predates `/logout` - the claim expires on its own). A client
already in the world keeps playing: its session was minted at its own login. A client from
that sign-in that has not connected yet is not served as the account.

## How more than one account actually works

**A login claim in the shared database, and per-connection resolution.** Built 2026-08-28.

```text
launcher                              login server
--------                              ------------
authenticate_identity(email|name, pw)
  -> argon2id verify, session token
stake_login_claim(account_id, token)
  -> "serve the next connection as this"
                                      accept()
                                      resolve_account()  <- reads the claim, per connection
                                      Session::new(.., that account)
```

`store::claims` holds it, `login::server::resolve_account` reads it, and `--account` survives
as the fallback for a connection that arrives before anyone has signed in.

**Why this beats one-server-per-port.** That option needed N processes on N ports for N
accounts, the launcher had to know the mapping, and adding an account meant editing a
launch script. This needs one process, one port, and no restart to swap accounts. It also
composes with the installer: a target machine runs one pair of servers whoever is playing.

Three properties that are load-bearing, and the second one is the one that would have been
got wrong:

* **The claim is not consumed on read.** The client opens a *second* login connection after
  "Log Out" and "Choose another world" - `CLAUDE.md` records two `0x0010`s in one launch - so
  a single-use claim would drop that connection back to the fallback account. On screen that
  is "my characters vanished when I logged out", which is worse than the replay it would be
  guarding against. `resolving_twice_serves_the_same_account_because_a_claim_is_not_consumed`
  is the regression test and it is named to say so.
* **A stale claim falls back rather than refusing.** An account deleted or disabled while a
  claim named it resolves to the fallback and the connection is answered. Refusing would
  leave the client with no reply, and an unanswered packet freezes its entire UI.
* **Every connection logs which account it chose and why.** `world.log` and `login.log` are
  the only record of whose characters were on screen, and three of this project's answers
  came from reading a log after the fact.

**The email is an identity, not a credential.** `accounts.email` is a nullable unique column;
`Store::get_account_by_identity` matches name **or** email. The two namespaces cannot collide
because `validate_name` allows only `[A-Za-z0-9_]`, so no string is a valid name and a valid
email at once - which is what makes one sign-in field safe rather than merely convenient. An
unresolvable identity is still routed through argon2id's dummy verify, so a bad email does
not answer faster than a bad password and hand back an account-enumeration oracle.

### And it is still not authentication

`resolve_account` decides **which** account a credential-less connection is served as. It
does not check anything about the connection, because there is nothing on the wire to check:
the game socket carries no credentials and `0x0073` has been measured carrying none. The
launcher verifies a *person* before staking a claim; anything that can reach the login port
is then served as that account. The login server says so at startup, the launcher says so on
screen, and the installer's README says so. Do not describe it otherwise until something is
gating the login result.

## Shipping it: the installer

The owner, 2026-08-28: *"we'll need to ship an installer too with all of these client files.
Including the wz files since the test machines will not have any client files and it needs to
start from 0."*

`tools/make-installer.ps1` stages a payload on the dev box; `tools/installer/install.ps1`
runs on the target. What goes in, what does not, and why the client is staged **unstubbed**
is documented in the packaging script's own header rather than repeated here.

Two things worth knowing before running either:

* **`gm-handbook/` must be regenerated before packaging.** It is gitignored game data, the
  world server reads its tables from it, and a payload without it installs cleanly and then
  cannot load a map. The packaging script refuses rather than shipping one.
* **`--set-field-probe` is passed unconditionally** by `start-server.ps1`, with no switch to
  turn it off. Without it `Session::handle` returns nothing for every packet and the client
  sits on "Connecting..." looking exactly like a server that is not running - a failure that
  has already cost one manual launch, and one an installed machine must not be able to
  reproduce.

The client and its WZ data are Nexon's. The payload exists so machines the owner owns can run
files they own; it is not a distribution channel.

## Implementation notes

* **Rust with `eframe`/`egui`.** One static executable, no runtime to install, same workspace
  and same toolchain as everything else. A `crates/launcher` binary.
* **Elevation:** the client's own manifest raises UAC on `ShellExecute`. The launcher itself
  does not need to be elevated to *launch* it - but it does to write the firewall rule, so
  either ship that as a separate one-time setup step or have the launcher request elevation
  only when the rule is missing. Prefer the former: a launcher that demands admin every time
  is the opposite of professional.
* **Never store the password.** Keep the token if a "remember me" is wanted; it is
  single-use, so it buys little - a refresh token is a later feature, not a first one.
* **Show the log path on failure.** The three logs and what is in each are listed in
  `STATUS.md`; the launcher should surface the hook log, since that is where a failed patch
  reports.

## What this does not cover

Patching the client's *appearance* - title, version string, the Nexon branding on the login
screen. Much of that text is baked into WZ canvases as pixels rather than strings
(`tools/wz_png.py` renders them), so changing it is WZ work, not launcher work. Out of scope
until the server is real.
