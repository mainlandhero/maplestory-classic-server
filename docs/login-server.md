# The login server

`crates/login`. Written 2026-08-18, replacing `tools/handshake_probe.py` as the thing the
client talks to. The probe stays for packet capture and for testing hypotheses that need a
body typed on a command line; it is no longer the server.

**What changed that matters:** characters persist. A character created in one run is in the
character list of the next, because it is a row in a database rather than a command-line
argument. Everything else here is in service of that.

## Running it

```bash
cargo build --release -p login
./target/release/maplecw-useradd.exe maplecw          # once; prompts for a password
./target/release/maplecw-login.exe --list             # what is stored, without listening
./target/release/maplecw-login.exe                    # listen on 127.0.0.1:8484
```

With the client, from an **elevated** shell:

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1"
```

That builds, installs the hook into `client-patched/`, starts the server, applies the
client patches and launches the client. `-Stop` tears it down; `-ListOnly` prints the
stored characters and launches nothing.

| file | what is in it |
|---|---|
| `login.log` | every packet both ways, and **what each reply was** — read this first |
| `client-patched\maplecw-hook.log` | client patches and faults |
| `client-exit.log` | how the client died; `0` is a hand-close |

## The shape

Three modules, split so the protocol can be tested without a socket:

| module | holds |
|---|---|
| `handshake` | the greeting. 48 bytes, unencrypted, server speaks first |
| `session` | the whole protocol as a **pure state machine**: bodies in, bodies out |
| `server` | the socket loop, framing, the log. Nothing protocol-shaped |

`session` having no socket and no clock is the point: every exchange measured against the
real client is a unit test, and a change can be checked without a client launch. A launch
costs the owner a manual, elevated run and the only oracle is what they see on screen.

## What it answers

| client sends | server sends | notes |
|---|---|---|
| — | `0x0032` | the startup gate, unprompted at connect **and** again after 4s of quiet |
| `0x00A1` | `0x0032` | the same gate, when the client asks for it |
| `0x0080` login | `0x0000`, `0x000B`, `0x000B`, `0x0010` | account name, world, terminator, character list |
| `0x0082` leave world | the same four | **standing**, not one-shot |
| `0x00A8` open creation | `0x05F4` | |
| `0x0081` check name | `0x0014` | now **truthful** — see below |
| `0x008A` create | `0x0015` | persists, then replies from what was stored |
| anything else | nothing | version and environment reports and log uploads want no reply |

### Always answer

**An unanswered request freezes the client's entire UI** — every button, including the OK
on the quit prompt. That is what "Check" did before `0x0081` was answered and what "Choose
another world" did before `0x0082` was, and both times it read as a crash.

So no path in `session` returns an error instead of a reply. A database failure becomes a
*refusal the client can render*, and the reason travels in the reply's log label. That rule
is a test: `every_request_the_client_blocks_on_gets_an_answer`.

### The name check is the first honest answer

The harness replied "available" to every name, including names it had already handed out.
The server checks the database. Names are unique across the **whole service**, not per
account — the client's request carries no account to scope by — and the comparison is
case-insensitive.

A name is refused as *not allowed* if it is not 4–12 ASCII alphanumerics. Twelve is not
arbitrary: the character record writes the name into a fixed 13-byte block.

### The reply is built from what was stored

Not from the request. That is deliberate — sending the request back would hide exactly the
class of bug this crate exists to fix, where the screen after creation shows something the
next launch will not.

## What is **not** authenticated

**Nothing on this socket proves who the player is.** The client's login request carries no
credentials; the login form is vestigial, and the server supplies both the login result and
the account name. So `--account` decides whose characters every connection sees, and two
different people connecting are the same account.

The server says so at startup, in its own log:

```text
serving every connection as account "maplecw" (id 1), 1 character(s) stored
NOT AUTHENTICATED: the game socket carries no credentials, so anyone who
  connects is served as that account. See docs/launcher.md.
```

Closing this is the launcher work in `docs/launcher.md`: authenticate over HTTPS against
`crates/auth`, receive a single-use token, and get the token to this server so it can call
`/consume`. Whether the client will carry that token in `0x0073` is **not yet measured** —
`docs/deployment.md` has the experiment. Until `/consume` is gating the login result, do not
describe a session here as authenticated.

## Storage

`crates/store/src/character.rs`, two tables:

* `characters` — one row per character, columns matching the protocol's `Character` field
  for field, `name` unique and case-insensitive, `account_id` cascading from `accounts`.
* `equipment` — `(character_id, slot, item_id)`, cascading from `characters`.

The row maps to `net::opcode::Character` **by exhaustive destructure in both directions**,
so adding a protocol field breaks the build until a column exists. A separate storage struct
would have meant mapping nineteen fields by hand, and a field missed in that mapping is a
stat that silently does not persist — the same shape as the avatar-look bug, which a
round-trip test could not see.

The character id on the wire is the database rowid, so it is distinct per character. That is
a protocol requirement, not tidiness: a canned reply that sent id 200 twice made the client
silently drop the second character.

## How it was checked without launching the client

`tools/login_smoke.py` is a **stand-in client**. It speaks the real transport through
`tools/transport.py` — an implementation written independently of the Rust one, from the
client's own receive path — so agreement between them is evidence rather than one module
agreeing with itself.

```bash
python -u tools/login_smoke.py --check-quiet
python -u tools/login_smoke.py --list-only
```

Verified 2026-08-18, against a fresh database: greeting accepted, gate delivered (and
repeated to a quiet client), login answered with four packets, creation permitted, a free
name available and the same name taken immediately after creating it, create accepted, and
the new character present in the next login result. Then the server was **stopped and
restarted** and a new connection still listed the character.

**What it does not prove: the client's reaction.** Only a launch shows whether the character
is drawn, and drawn correctly. Every packet-level fact in this repo that turned out to be
wrong was wrong about a body the client read differently, not about a byte count.

Two instrument bugs it caught on itself, both worth remembering:

* the server binary under test was **stale** — `cargo test` does not refresh
  `target/release/maplecw-login.exe`, so the running server was a build old enough to
  predate the feature being tested. `cargo build --release` before every run.
* an early `drain()` decoded only bytes already in its local buffer and never read the
  socket, so a packet sitting in the OS receive queue read as "nothing sent" — and because
  it was still there next time, every check after it was offset by one and failed.

## Still standing on client patches

The server is real; the run around it is not yet. `test-server.ps1` still applies:

| patch | why |
|---|---|
| `1415db360:ret` | **not optional** — the client `__fastfail`s after ~37s otherwise. A client bug |
| `141b2a280:rdx=0` | suppresses the "trouble logging in" dialog, which blocks the tick that enables Login |
| `mode=2` | leaves mode-5 auto-login so the button gets a turn |
| `create=on` | sets the flag gating "Create a character" — the real service does this from virtualised code |

The last two stand in for protocol we have not found. `docs/launcher.md` has the retirement
column.

## Next

* Confirm on screen, with a launch: create a character, close, relaunch, and see it in the
  list. That is the one claim only the client can settle.
* Delete. `crates/store` has `delete_character` with an ownership clause, and the client has
  a delete result (`0x0016`, body is a `u32` character id), but the request opcode has not
  been identified and nothing is wired up.
* Character slots are a constant `3`. The client computes the free slot from it and the list
  length, and the server refuses a create past it — but the number should come from the
  account eventually.
* The account gap above, which is Stage 3.5.
