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

## Interpretation

The client is missing something the Nexon launcher normally supplies — most likely a
session/auth handoff (Passport or Steam), since `WEBSTART` and `STEAMSTART` are exactly
the launcher's entry modes and both exit cleanly rather than crashing.

**GameGuard neutralisation remains unverified**, not failed: we have not yet reached the
point in startup where it would run.

## Next

Find the early-exit path in Ghidra and see what it tests for:

- Locate the argument parser via the string constants (`WEBSTART`, `IPPORT`,
  `GAMELAUNCHING`) and follow their cross-references.
- Identify the check made immediately after argument parsing — registry key, named pipe,
  environment variable, mutex, or file left by the launcher.
- No client-side log or dump file is produced on exit, so the answer has to come from
  static analysis or runtime observation (e.g. Process Monitor) rather than client logs.
