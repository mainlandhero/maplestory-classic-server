# Client launch interface

Reverse-engineered from `MapleStory.exe` (Ghidra). The command-line parser is
**`FUN_142c94bd0`** at `0x142c94bd0` - every launch token is referenced from that one
function. Full decompilation in `research/msexe-argparser.c`; the config accessors it
feeds are in `research/msexe-launchmode.c` and `research/msexe-launchconfig.c`.

Signature is effectively:

```c
void parse_command_line(LaunchConfig *cfg /*param_1*/, CommandLine *cmd /*param_2*/);
```

## Tokenising

```c
FUN_1401d1140(cmd, &DAT_14349302c);          // set delimiters
FUN_142ca0710(&out, &cmd, index);            // fetch token[index]
```

`DAT_14349302c` is the two bytes `0x22 0x20` - i.e. **`"` and space** are the delimiter
set. So arguments split on whitespace, and quotes are separators rather than grouping
characters.

Token 0 is upper-cased (`_strupr`) before dispatch, so the mode keyword is
case-insensitive. Everything else is taken verbatim.

## Recognised keywords

One contiguous `.rdata` block holds them all, dumped with `tools/dump_va.py 0x143493028`:

```text
143493028  6e 00 00 00 22 20 00 00 57 45 42 53 54 41 52 54  n..." ..WEBSTART
143493038  00 00 00 00 2d 4e 58 4c 00 00 00 00 00 00 00 00  ....-NXL........
143493048  2d 4e 58 4c 44 45 42 55 47 00 00 00 00 00 00 00  -NXLDEBUG.......
143493058  2d 4e 58 4c 50 54 53 00 53 54 45 41 4d 53 54 41  -NXLPTS.STEAMSTA
143493068  52 54 00 00 47 46 4e 00 47 41 4d 45 4c 41 55 4e  RT..GFN.GAMELAUN
143493078  43 48 49 4e 47 00 00 00 4b 52 00 00 00 00 00 00  CHING...KR......
143493088  31 30 2e 39 2e 32 2e 31 33 33 00 00 00 00 00 00  10.9.2.133......
143493098  34 34 2e 32 33 34 2e 31 36 36 2e 31 36 31 00 00  44.234.166.161..
1434930a8  34 34 2e 32 33 34 2e 31 36 37 2e 31 36 33 00 00  44.234.167.163..
1434930b8  68 74 74 70 3a 2f 2f 6d 61 70 6c 65 73 74 6f 72  http://maplestor
1434930c8  79 2e 6e 65 78 6f 6e 2e 6e 65 74 2f 6d 69 63 72  y.nexon.net/micr
1434930d8  6f 2d 73 69 74 65 2f 32 30 37 30 31 00 00 00 00  o-site/20701....
1434930e8  49 50 50 4f 52 54 00 00                          IPPORT..
```

`GFN` is not a top-level keyword: it is a value `STEAMSTART` looks for in its own token 3.

## What each keyword does

`tN` is token N. "session[6]" is the six-pointer array at `cfg+0x90`.

| Keyword | Tokens | `cfg+0x38` | Extra |
|---|---|---|---|
| `WEBSTART` | needs t1 non-empty; t4..t9 -> session[6] | **3** | no IP or port at all |
| `-NXL` | t1 -> `+0x80`, t2 -> `+0xc0` region, t3 -> IP, t4 -> port, t5.. -> session[6] | **5** | |
| `-NXLDEBUG` | t1 -> IP, t2 -> port, t3.. -> session[6] | **5** | leaves `+0x80` and the region untouched |
| `-NXLPTS` | same as `-NXL` | **5** | also `+0xc8 = 1` and `DAT_143a88df8 = 0` |
| `STEAMSTART` | t1 -> `+0x80`, t2 -> `+0xc0` region, t3 -> `+0x88` (upper-cased) | **4** | if `+0x88` is `GFN` or `-`, IP/port shift one slot right |
| `GAMELAUNCHING` | region forced to `KR`; t1 -> IP, t2 -> port, t3..t8 -> session[6] | **2** | **IP whitelisted** |
| `IPPORT` | identical to `GAMELAUNCHING` | **2** | **IP whitelisted** |
| anything else | - | *(unset)* | opens the micro-site, reports error `0x195`, returns |
| empty token 0 | - | *(unset)* | opens the micro-site, reports error `0x1a4`, returns |

Every branch requires **token 1 to be non-empty** before it sets a mode. `WEBSTART` on its
own therefore sets nothing and the client exits cleanly - which is exactly the behaviour
recorded in `research/client-launch.md`.

### The mode-2 IP whitelist

`GAMELAUNCHING` and `IPPORT` `strcmp` the IP against six literals before setting the mode:

```text
10.9.2.131   10.9.2.132   10.9.2.133
44.234.166.161   44.234.167.163   44.234.163.43
```

An IP outside that set opens `http://maplestory.nexon.net/micro-site/20701`, reports error
`0x16d` (`GAMELAUNCHING`) or `0x18a` (`IPPORT`), and returns **without setting a mode**.
This is why `IPPORT 127.0.0.1 8484` was recorded as a crash: it never reached mode 2 at
all. The gate is a hard `strcmp` chain - there is no hostname lookup and no bypass short of
patching, but the addresses are ordinary routable/RFC1918 ones, so a server that *answers
on one of them* would reach mode 2 with no client patch.

## The config struct

Offsets into `param_1`. Accessors are 4-5 byte leaf functions, so `.pdata` has no entry for
most of them; they were found by byte-scanning for `mov eax,[rcx+disp]; ret` in
`0x142c95c00..0x142c96400`.

| Offset | Accessor | Meaning |
|---|---|---|
| `+0x18` | `FUN_142c95c20` | **server IP** (string) - *no caller in readable code* |
| `+0x20` | `FUN_142c95c50` | **server port** (string) - *no caller in readable code* |
| `+0x28` | `FUN_142c95c80` | u32 -> `FUN_142cb8410(DAT_143aa84a0, ...)` |
| `+0x30` | `FUN_142c95ca0` | string -> `FUN_142cb5f70(DAT_143aa84a0, ...)` |
| `+0x38` | `FUN_142c95c90` | **launch mode** |
| `+0x40` | `FUN_142c95cd0` | string, game-log init |
| `+0x48` | `FUN_142c95d00` | u32, game-log init |
| `+0x4c` | `FUN_142c95d10` | u8 -> `session+0x150` |
| `+0x50`, `+0x54`..`+0x58` | `FUN_142c95d20`.. | u32 and five u8 - *no callers*, not set by the parser |
| `+0x80` | `FUN_142c95ef0` | `-NXL`/`-NXLPTS`/`STEAMSTART` token 1. One caller, `FUN_142c50390`, which **itself has no callers** |
| `+0x88` | `FUN_142c95f80` | `STEAMSTART` token 3 -> `session+0x1c0` |
| `+0x90` | `FUN_142c95f20(cfg,out,i)` | **the six session tokens** - *no caller in readable code* |
| `+0xc0` | `FUN_142c95fb0` | region string -> `FUN_142ce96b0(DAT_143aa84a0, ...)` |
| `+0xc8` | `FUN_142c95f70` | `-NXLPTS` flag - *no caller in readable code* |

The IP and port accessors having no callers is not surprising: the connect path reads the
strings directly rather than through them.

**The six session tokens have no reader.** Ghidra's `Xrefs` returned none for
`FUN_142c95f20` in a run where the sibling accessors *did* return callers, so the
instrument was working. That agrees with the 2026-08-18 wire measurement, where six
distinguishable tokens produced a byte-identical `0x0073`. Two independent instruments, one
conclusion: **a launcher token cannot reach the server through `+0x90`.** The one path not
audited is `FUN_142c926e0` (the 9408-byte startup, which owns the parser call) reading the
array directly - but a read that never reaches the wire does not change the design.

## The mode reaches exactly one field

```text
argv[0] keyword
  -> cfg+0x38                      (the parser, FUN_142c94bd0)
  -> FUN_142c95c90(cfg)            (2 callers only: the session ctor, and the game-log init)
  -> session+0x68                  (FUN_142c43db0, the session constructor)
  -> FUN_142c4a810(session)        (15 call sites in 11 functions)
```

`FUN_142c43db0` writes `session+0x68` once, from the config, and nothing else in readable
code writes it. **The launch keyword is the sole determinant of the mode** - which is why
`grap-stub`'s `-Session mode=2` has to poke `session+0x68` directly.

### The client's own classifier

```c
undefined1 FUN_1401e7bf0(int mode) { return mode == 3 || mode == 4 || mode == 5; }
```

So the client splits the world into **"a launcher started me" (3, 4, 5)** and **"I was
started directly" (2, or unset)**. Called from `FUN_1415d10e0`, `FUN_142c46b80`, and
`FUN_141b2dd00` (the `0x0000` login-result handler, on the *failure* branch only).

Other sites test `mode == 5` specifically, via the one-line predicate
`FUN_141b3fd10() { return FUN_142c4a810(DAT_143ac1898) == 5; }`.

### Mode 5 has a startup prerequisite

In `main` (`FUN_142c42f30`), immediately after the session is constructed:

```c
if ((mode == 5) && (FUN_140d90d70() == '\0')) {
    FUN_141804a70(local_820, 0x23000001);
    _CxxThrowException(...);                  // does not return
}
```

Our client runs under `-NXLDEBUG`, so `FUN_140d90d70()` is already returning non-zero for
us. Worth knowing that mode 5 alone carries this gate.

## Which mode to launch with

**`-NXLDEBUG` is not a debug mode.** It writes the same `cfg+0x38 = 5` as `-NXL`, which is
what the real Nexon Launcher passes. The `DEBUG` suffix changes only *which argv slots* map
to IP and port; it selects no different code path, sets no debug flag, and relaxes no
check. Switching to `-NXL` would change exactly two things: `+0x80` would be set (its only
reader has no callers) and the **region string** at `+0xc0` would be ours to choose instead
of whatever the config constructor left there.

`-NXLPTS` differs from `-NXL` by `+0xc8 = 1` (nothing reads it) and `DAT_143a88df8 = 0`,
whose single reader is `FUN_141b5bb50` - the login-screen draw, where a non-zero value adds
one call to `FUN_1415daff0(DAT_143ac18a0)`. Cosmetic. **`-NXLPTS` buys nothing.**

That leaves mode 2 as the only keyword-reachable behaviour change, and it is not the change
we want. The login screen's buttons fork on `mode == 5` in `FUN_14112a570`:

```c
if (button == "login") {
    if (mode == 5) FUN_141b3ff10();              // begin login
    else           FUN_141b3f050(stage, 4, 600); // fade to screen 4 over 600 ms
}
else if (button == "quit") {
    if (mode == 5) FUN_141b2d4a0(session, 0, 0); // otherwise nothing happens
}
```

`FUN_141b3ff10` is the login request; on failure it raises `unableLogOnToGameSvr`.
`FUN_141b3f050` is a screen transition, and screen **4 is world select** - which matches
what happened when `-Session mode=2` was patched in and a world-list terminator sent the
client to WorldSelect.

So **mode 2 restores the classic WorldSelect -> ChannelSelect flow; it does not make the
client authenticate.** The account identity still is not in the client, so mode 2 does not
help multi-account - it costs two screens `crates/login` would have to implement, and it
kills the login-screen Quit button, which reads as a freeze.

**Conclusion: stay on `-NXLDEBUG`.** The only launch-line change with any prospect of
mattering is `-NXL <anything> <region> <ip> <port>`, purely to control the region string,
and only if something is ever traced to the region.

## Call chain

Traced by cross-reference; each function has exactly one caller:

```text
FUN_142c42f30   (main; owns the config as local_5c8)
  |- FUN_142c926e0   (startup, 9408 bytes)  - receives config as param_1
  |    `- FUN_142c94bd0  (the parser above)
  |- FUN_142e14bc0(config)      consumers, called straight after parsing
  `- FUN_142c43db0(_, config)   (the session constructor)
```

The config is a **stack local in main**, not a global, so Ghidra does not propagate a
struct type into the consumers.

## Where the session was assumed to go - superseded

The binary carries a **Nexon Client Manager** interface (`CNMLoginNexonPassportFunc`,
`CNMGSGetSessionInfoFunc`, ...) from `nexon_api_x64.dll` / `nmcogame64.dll`, and the
`WEBSTART` tokens were assumed to be handed to it. That is now the *fallback*, not the
plan: the masked account email on the login screen is written only by the `0x0000` and
`0x0012` packet handlers, so **the account name is server-supplied**, not a launcher
handoff. See `docs/session.md`.
