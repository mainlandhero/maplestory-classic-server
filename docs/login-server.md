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
./target/release/maplecw-login.exe --delete NAME      # clear one character and retest
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

## SOLVED: the create reply the client would not act on

**The character id.** The first real run created `TestChar` and the client stayed on the
creation screen, having dispatched the reply (`ret=1`, no fault, no C++ throw). The body was
reconstructed and diffed against the reply that transitioned the client on 2026-08-17:
identical across all 353 bytes **except offsets 5 and 9**, the two copies of the id - `1`
against `200`. Ids now start at 200 (`FIRST_CHARACTER_ID`, seeded through `sqlite_sequence`),
and the owner confirmed creation works on screen.

**Do not renumber characters from 1.** Why the client cares is not established - only that
it does - so the constant is documented where it is set rather than explained away.

Two method notes worth keeping:

* **A number means nothing without a baseline.** The handler's 111.9 microseconds looked like
  proof it had bailed out early, until the neighbours were checked: `0x0010`, which builds
  the entire character select screen, took 333. Sub-millisecond was simply normal.
* **`login.log` did not record bodies**, so that reply had to be rebuilt by hand to find a
  two-byte difference. It records them now, capped at 96 bytes, and that immediately paid
  for itself by capturing the select-character flow below.

## The three-character limit needed no new code

Confirmed on screen: with three characters stored, "Create a character" is disabled. Nothing
was added for it - `login_result` already sends a truthful list and `slotCount`, and
`FUN_141b282d0` computes the free slot from those two. **Telling the truth was the
implementation.** The server-side refusal was already there and tested, so a fourth character
cannot be stored either way.

`create=on` remains, and it is a **different** gate - it enables the button *at all*, which
the real service sets from virtualised code. Removing it is finding that packet, not
enforcing a limit.

## Captured: the select-character flow

Entering the world sends three packets nothing answers yet. First read, from one capture:

```text
0x0078  73B  u32 0 | str "." (placeholder PIC) | u32 203   <- the character id
              | u8 0 | str "AA-BB-CC-DD-EE-FF, 00-00-..." | str "AABBCCDDEEFF_DEADBEEF"
0x0079  67B  u32 203 | str "TestCharC" | u32s | SYSTEMTIME 2026-08-18 13:36:32
0x00BC  12B  09040000 09040000 09040000  - 1033 three times
```

`0x0078` carrying the character id makes it the select-character request, and its reply the
migration packet - the `0x0011` candidate, which is Stage 4 and where an *advertise* address
would first be needed (`docs/deployment.md`). The client sat on "Connecting..." because
nothing answered, which is expected with no channel server.

## Still standing on client patches## Still standing on client patches

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

* **Settle the transition above.** One launch, and it now measures two things at once.
* Confirm persistence on screen: create, close, relaunch, see it in the list.

### Two goals the owner set on 2026-08-18

**Real sessions, so more than one account can be served.** `--account` serving everyone is
fine for one tester and wrong for two. The blocker is that the game socket carries no
credentials. The order of work:

1. **Measure whether a launch-argument token reaches us.** `-NXLDEBUG` routes arguments 3
   onward into the client config's six-slot session array at `+0x90`. `0x0073` is now
   decoded into the log - mode, identity string, machine tail - and `test-server.ps1` takes
   `-SessionTokens`, so any launch answers it as a side effect.
2. **If tokens arrive:** the launcher authenticates against `crates/auth`, passes the
   single-use token, and the login server resolves the account through `/consume`. Half of
   that already exists.
3. **If they do not:** one login server per account on its own port. Crude, works today,
   needs no protocol.

Either way `Session` should take its account from a resolver rather than from config.

**The three-character limit, enforced rather than patched.** Three separate things, and
conflating them is how a workaround becomes permanent:

* **Server-side: done.** `create_character` refuses past `CHARACTER_SLOTS` with
  `CREATE_INSUFFICIENT_SLOT`, pinned by `a_full_account_is_refused_with_the_slot_code`. A
  fourth character cannot be stored whatever the client does.
* **Client-side: driven by what we send, and untested at three.** `FUN_141b282d0` computes
  `slotCount - stage+0xe4 - 1`, clamps at zero and checks whether that slot is occupied,
  raising `insufficientCharacterSlot` if it is. We send a truthful list and slot count, so
  it should work - worth a deliberate check once creation transitions.
* **`create=on` is a different gate.** It forces the flag that enables the button *at all*,
  which the real service sets from virtualised code. Removing that is finding the packet,
  not enforcing a limit, and it is the honest remaining item.
* The slot count should become a property of the account rather than a constant in
  `crates/net`.

### Also open

* Deletion over the protocol. `maplecw-login --delete NAME` exists for testing and
  `crates/store` enforces ownership in the statement, but the client's delete *request*
  opcode has not been identified. The result is `0x0016`, body a `u32` character id.
