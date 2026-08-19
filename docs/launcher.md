# The launcher

Written 2026-08-17. Not built yet. The owner asked for "a minimal launcher for the client to
perform any patching you need to do in order to neutralize GameGuard and skip net check when
the client starts, so it is more professional looking", and noted it probably needs the
username and password, since the real client uses a validated session.

This is the design. It replaces `tools/test-charselect.ps1` for *using* the client; that
script stays as the instrumented harness for *investigating* it, and the two should keep
working side by side.

## What it does, in order

1. **Find and verify the client.** Hash `MapleStory.exe` and compare against a known build.
   This is not ceremony: every patch we apply is at an **absolute virtual address**
   (`0x1415db360`, `0x141b2a280`), so applying them to a different build writes into
   whatever happens to be there. Refuse to launch an unrecognised build and say so.
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
| `mode=2` | patches the login-stage mode byte | was believed to stop the auto-login | **possibly already retired.** Measured 2026-08-17: it is applied ~100ms *after* the client has already auto-logged in, so it cannot be doing that job. Test whether anything breaks without it - a cheap experiment now that runs are cheap |

A launcher that silently applies all five is worse than one that says what it did. Print the
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

### What is left, and neither is pretty

1. **One login server per account, each on its own port**, with the launcher choosing the
   port after it authenticates. Needs no protocol at all, works today, and the owner has said
   testing-grade is acceptable for now. The account stops being global configuration and
   becomes per-instance, which is the only property that actually matters for testing.
2. **Write the identity string from `grap-stub`.** `0x0073`'s second field is a `char *` at
   `DAT_143ac1898+0x1b8`, sent as a **zero-length string because nothing computes it**. The
   stub is already in-process and could write a token there before the packet is built. That
   would carry a real token to a real `/consume` - but it is a **client patch standing in for
   a session**, and it belongs in the patch inventory above with a retirement column, not
   described as authentication.

Either way `Session` should take its account from a resolver rather than from `Config`, so
whichever route wins is one function.

**Nothing here is authenticated today**, and the login server says so at startup. Do not
describe it otherwise until `/consume` is gating the login result.

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
