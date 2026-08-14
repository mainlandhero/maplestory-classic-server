# Client bring-up log

Experiments against `client-patched/` (a copy; the original install is never touched).
The client is firewalled outbound so it cannot reach Nexon — see `tools/firewall.ps1`.

Client file version: **1.1.0.0**.

## Result summary

| Arguments | Outcome |
|---|---|
| *(none)* | `0xC0000005` access violation |
| `GAMELAUNCHING` | `0xC0000005` |
| `GAMELAUNCHING <ip> <port>` | `0xC0000005` |
| `GAMELAUNCHING skiplogo` | `0xC0000005` |
| `IPPORT <ip> <port>` | `0xC0000005` |
| `<ip> <port>` (positional) | `0xC0000005` |
| **`WEBSTART`** | **clean exit 0** |
| **`WEBSTART <ip> <port>`** | **clean exit 0** |
| **`STEAMSTART`** | **clean exit 0** |
| **`-NXL`** | **clean exit 0** |
| **`-NXLDEBUG`** | **clean exit 0** |

Reproduce with `tools/try-launch.ps1`.

A clean exit is a much better signal than a crash: the client parsed the argument,
initialised, decided a prerequisite was missing, and shut down deliberately. The
crashing forms are presumably reading argv slots that were never populated.

Faulting module for the crashes is `MapleStory.exe` itself at offset `0x20A520F`
(inside `.text`), not a DLL.

## GameGuard is *not* the current blocker

Established by two experiments:

1. **Control test.** Restoring the genuine `grap64.dll` (keeping `grap.disabled` so
   `NGService.exe` and `BlackCat64.sys` still could not run) produced the **identical**
   `0xC0000005`. The stub is therefore not the cause of the crash.
2. **The stub is never called.** No entry in `grap-stub.log` from any run — including
   the clean-exit ones.

The stub itself is proven good, independently: loading it with `LoadLibraryW` succeeds,
`GetProcAddress(ordinal 9)` resolves, and `DllMain` logs `PROCESS_ATTACH`.

So the client exits **before it ever initialises GameGuard**. Note this also means
Themida is *not* binding the static `grap64.dll!#9` import at load time the way the
normal Windows loader would — consistent with its IAT being rebuilt at runtime.

## BREAKTHROUGH: the client runs and connects to us

```
MapleStory.exe -NXLDEBUG 127.0.0.1 8484
```

**The client stays running and opens a TCP connection to `127.0.0.1:8484`.**

Captured by `tools/listen.py`:

```
[8484] 01:56:14 *** CONNECTION from 127.0.0.1:65272 ***
[8484] no further data (total 0 bytes); holding open
```

Three things follow from this.

**1. `-NXLDEBUG <ip> <port>` is the way in.** It is the only mode that both survives and
connects. Per `docs/launch-protocol.md` it writes token 1 to `cfg+0x18` (IP) and token 2
to `cfg+0x20` (port) and sets mode **5** — the same address fields as `IPPORT`, but a
path the client actually tolerates. `IPPORT` still crashes; `WEBSTART` still exits
cleanly even with ten arguments supplied, so its six payload fields are checked for
content we cannot yet fake.

**2. The client sends nothing and waits — the server speaks first.** It connected and
then sat there having sent **0 bytes**. That matches the classic MapleStory handshake,
where the server opens with an unencrypted greeting carrying the version, the two IV
seeds, and a locale byte, and only then does the client reply. So Stage 2 begins by
*sending*, not by waiting to decode something.

**3. GameGuard never initialises on this path.** `grap-stub.log` is empty across every
run, and the client reaches the network regardless. The no-GameGuard route works: no
service, no kernel driver, and nothing installed system-wide.

Note the process cannot be killed with `Stop-Process` (access denied) — Themida's
self-protection. Use `taskkill /F` or let it exit on its own.

## Interpretation

`WEBSTART` still exits cleanly even when given ten arguments, so its six payload fields
(tokens 4-9) are validated for content, not merely presence. Those are the launcher's
session identifiers, and faking them needs either the real format or a path that skips
the check — which `-NXLDEBUG` appears to be.

**GameGuard neutralisation is effectively confirmed** for this path: the client reaches
the network with the stub in place and the driver folder disabled.

## Next

Stage 2, and the server has to talk first:

- Send a candidate handshake on connect (version 779, two 4-byte IV seeds, locale byte)
  and watch whether the client replies or drops.
- A reply means the framing is right and `crates/net`'s `MapleCipher` hypothesis can be
  tested against real bytes; silence or a drop means the format differs and
  `MapleSecurePC64` has to be reversed first.
- Keep `WEBSTART` in view as the eventual production path, since that is what our
  launcher will use once the six fields are understood.
