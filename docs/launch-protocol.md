# Client launch interface

Reverse-engineered from `MapleStory.exe` (Ghidra, 2026-08-14). The command-line parser is
**`FUN_142c94bd0`** at `0x142c94bd0` — every launch token is referenced from that one
function. Full decompilation in `research/msexe-argparser.c`.

Signature is effectively:

```c
void parse_command_line(LaunchConfig *cfg /*param_1*/, CommandLine *cmd /*param_2*/);
```

## Tokenising

```c
FUN_1401d1140(cmd, &DAT_14349302c);          // set delimiters
FUN_142ca0710(&out, &cmd, index);            // fetch token[index]
```

`DAT_14349302c` is the two bytes `0x22 0x20` — i.e. **`"` and space** are the delimiter
set. So arguments split on whitespace, and quotes are separators rather than grouping
characters.

Token 0 is upper-cased (`_strupr`) before dispatch, so the mode keyword is
case-insensitive. Everything else is taken verbatim.

## Recognised modes

Dispatch is a chain of comparisons against these constants (addresses are where each
string lives):

| String | Address |
|---|---|
| `WEBSTART` | `0x143493030` |
| `-NXL` | `0x14349303c` |
| `-NXLDEBUG` | `0x143493048` |
| `-NXLPTS` | `0x143493058` |
| `STEAMSTART` | `0x143493060` |
| `GFN` | `0x14349306c` |
| `GAMELAUNCHING` | `0x143493070` |
| `KR` (default region) | `0x143493080` |
| `IPPORT` | `0x1434930e8` |

## The config struct

Offsets into `param_1`, as written by the parser:

| Offset | Meaning |
|---|---|
| `+0x18` | **server IP** (string) |
| `+0x20` | **server port** (string) |
| `+0x38` | **launch mode** (int) — see table below |
| `+0x80` | set from token 1 by `-NXL` |
| `+0x90` | array of **6** token pointers (`0x30` bytes / 8) |
| `+0xc0` | region code; defaults to the literal **`KR`** |

### Mode values written to `+0x38`

| Mode keyword | Tokens consumed | `+0x38` |
|---|---|---|
| `GAMELAUNCHING`, `IPPORT` | t1 → `+0x18`, t2 → `+0x20` | **2** |
| `WEBSTART` | requires t1 non-empty; t4…t9 → `+0x90[0..6]` | **3** |
| *(no recognised keyword)* | — | **4** |
| `-NXL` | t1 → `+0x80`, t2 → `+0xc0`, t3 → `+0x18`, t4 → `+0x20`, t5… → `+0x90` | **5** |
| `-NXLDEBUG` | t1 → `+0x18`, t2 → `+0x20`, t3… → `+0x90` | **5** |

## Two conclusions that shape the plan

**1. `IPPORT <ip> <port>` really does set the server endpoint.** The parser writes
token 1 to `+0x18` and token 2 to `+0x20`, which are the address and port fields. This
confirms the redirect lever found during PE analysis — the client can be pointed at
`127.0.0.1` by argument, with no patching.

After storing them, the code compares the IP against the built-in Nexon addresses
(`10.9.2.131/132/133`, `44.234.166.161`, `44.234.167.163`, `44.234.163.43`), presumably
to select a region or flag. An unrecognised IP takes the other branch; whether that
matters is untested.

**2. `WEBSTART` explains the clean exit we observed.** The branch first requires
**token 1 to be non-empty**, and only then reads **tokens 4 through 9** into the
`+0x90` array and sets mode 3. Launched as bare `WEBSTART`, token 1 is empty, so the
whole block is skipped, no mode is set, and the client exits without doing anything —
exactly the behaviour in `research/client-launch.md`.

So **`WEBSTART` expects roughly ten arguments**, of which six (indices 4–9) are captured
as the session payload. Identifying those six fields is the remaining work for
`crates/launcher`.

## Call chain

Traced by cross-reference; each function has exactly one caller:

```
FUN_142c42f30   (main; owns the config as local_5c8)
  └─ FUN_142c926e0   (startup, 9408 bytes)  — receives config as param_1
       └─ FUN_142c94bd0  (the parser above)
  └─ FUN_142e14bc0(config)     ─┐ consumers, called straight after parsing
  └─ FUN_142c43db0(_, config)  ─┘
```

The config is a **stack local in main**, not a global, so Ghidra does not propagate a
struct type into the consumers and their decompilation does not show `+0x90` / `+0x38`
reads directly. Recovering the field *semantics* therefore needs the struct defined in
Ghidra and re-propagated, or runtime observation.

## Where the session actually goes

The binary carries a **Nexon Client Manager** interface — function-pointer names such as:

```
CNMLoginNexonPassportFunc     CNMGetNexonPassportFunc     CNMLoginPassportFunc
CNMGSGetSessionInfoFunc       CNMGSSessionCreatedEvent    CNMGSSessionEstablishedEvent
```

These belong to `nexon_api_x64.dll` / `nmcogame64.dll`, both of which are **unpacked and
analysable** (see `research/protection-surface.md`). So the `WEBSTART` tokens are most
likely identifiers handed to the Client Manager, which owns the actual session, rather
than credentials the game validates itself.

That makes `nmcogame64.dll` the place to look for the six fields' meaning — a smaller,
unprotected target than the 76 MB Themida-wrapped exe.

## Settled since

- **`-NXLDEBUG <ip> <port>` is the launch mode we use.** It runs *and* connects, and every
  session since has been driven through it. `IPPORT` still crashes and is not worth
  revisiting while `-NXLDEBUG` works.
- **The six `+0x90` session tokens have been passed and changed nothing observable.**
  `-NXLDEBUG` takes them from token 3 onward (`tools/test-one.ps1 -SessionTokens`), and six
  distinguishable values produced a byte-identical client stream — same `0x0073` body, same
  body-less `0x0080`. So they are either not the account identity or they never arrived;
  `test-one.ps1` now echoes the real command line so the next run can distinguish those.

## Next

- Analyse `nmcogame64.dll` / `nexon_api_x64.dll` for the `CNM*` session interface, which is
  where the `WEBSTART` identifiers are consumed. Both are unpacked and far easier to read
  than the Themida-wrapped exe. This is the live thread: the client's login ID field is
  blank where the live client shows a masked account email.
- Decode the constant 20-byte tail of outbound `0x0073`, which is the only session-shaped
  payload the client sends.
