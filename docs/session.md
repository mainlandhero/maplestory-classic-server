# The client's session identity

The goal this file serves: **make the client hold a session it considers valid**, so it
stops presenting the "having trouble logging in" state and proceeds past login.

## What "having trouble logging in" actually is

It is **not** a client message and **not** a dialog string. It was searched for and is
absent from all four places text can live:

| Searched | Result |
|---|---|
| the exe's 6165 XOR-encrypted messages (`tools/dump_stringids.py --grep`) | no match |
| `UI/Login.img` — all 260 string values | no match |
| `String.wz` `StringTable.img` | no match |
| raw ASCII **and** UTF-16 across every file in `client-patched/` | only the `troubled` facial emote |

So the text is a **baked bitmap**. That matters for how it is used as evidence: a canvas can
be permanent furniture on the screen rather than a state readout. `Login.img/ClassicIntro`
carries `button:find_id` and `button:find_pw`, which is exactly what such a label sits
above.

**Do not use it as the oracle.** Use the wire instead — see below.

## Better oracles than the screen

Two signals are readable without anyone interpreting a screenshot:

1. **The session identity is transmitted in cleartext** as the second field of outbound
   `0x0073`. Length zero means the client has no identity. See the decode below.
2. **Connection lifetime after a reply.** A rejected login result drops the connection in
   0.0 s with no follow-up; an accepted one runs the success path first. That difference is
   already in the probe log.

## 0x0073, fully decoded

Built by `FUN_141b21ea0`, alongside the body-less `0x0080`. 26 bytes, which matches the
captured length exactly:

```
u32   FUN_142c4a810(DAT_143ac1898)   -> session+0x68     the launch mode
str   FUN_142c50400(DAT_143ac1898)   -> session+0x1b8    THE SESSION IDENTITY
16B   (*DAT_143262960)(...)                              a GUID
u32   (*DAT_143262958)()                                 a tick counter
```

Captured: `05 00 00 00 | 00 00 | aa bb cc dd ee ff de ad be ef ... `

- `05` — launch mode 5, i.e. `-NXLDEBUG`. See `docs/opcodes.md`, "The login stage has two
  variants".
- `00 00` — **a zero-length string. The client has no session identity.** This is the same
  emptiness the blank Login ID field shows, visible on the wire.

This also retires an old open question. The "constant 20-byte tail" of `0x0073` is not
session data at all: it is the 16-byte GUID plus the 4-byte counter.

## Where the identity comes from

`FUN_142c50400(obj, out)` is a plain accessor:

```c
char *s = *(char **)(obj + 0x1b8);
if (s == NULL || *s == '\0') s = "";
// ... copies s into out
```

So the entire identity is one `char *` at **`DAT_143ac1898 + 0x1b8`**, and it is null or
empty. Nothing in the login path derives it — it is read, never computed.

Ruled out as its source: the six `+0x90` launcher tokens. `-NXLDEBUG` accepts them from
token 3 onward (`tools/test-one.ps1 -SessionTokens`), and six distinguishable values
produced a byte-identical client stream — same `0x0073` body, same empty identity.
`test-one.ps1` echoes the real command line, so this was not a case of the arguments
failing to arrive.

Still to try, cheapest first:

1. **Find the writer of `+0x1b8`** statically. It is a single field; whatever fills it is
   the launcher handoff we have been looking for. The `CNM*` interface in
   `nexon_api_x64.dll` / `nmcogame64.dll` is the likely home, and both are unpacked and far
   easier to read than the Themida-wrapped exe.
2. **Write the field directly.** `crates/grap-stub` already runs in-process with a working
   inline-hook and memory-patch capability. Pointing `+0x1b8` at a string we own would
   settle whether the empty identity actually blocks anything, without solving the handoff
   first.

## What the empty identity does *not* block

Worth stating plainly, because it was assumed for a while: an empty identity **did not stop
the client from logging in**. It still built and sent `0x0073` *and* `0x0080`, and it
accepted a `result = 0` login reply and advanced its UI to character select. At this stage
the client is blocked on what we send it, not on its own state.

So the identity is a real gap, but it is not the thing standing between us and character
select. The missing world list (`0x000B`) is.
