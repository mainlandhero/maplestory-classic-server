# Grounding the eight guessed client -> server opcode names

Static analysis only; the client was not run. Everything below is read out of
`MapleStory.exe` (Ghidra project `research/ghidra/msexe`) and checked against packet
bodies already captured in `login.log` and `research/fixtures/`.

Seven of the eight are now **ESTABLISHED** from their builder functions. One
(`0x00BC`) is **UNKNOWN**, and the search that failed is documented with the control
that proves the failure is a property of the instrument and not evidence of absence.

**Five of the eight invented names are wrong.** Two of those are wrong in a way that
would actively mislead a reader of the log: `0x007A CLIENT_DISCONNECT_NOTICE` is a
*completion* report, not a disconnect, and `0x00C0 CLIENT_HELLO` is a *login failure*
report that only exists on the error path.

## Verdicts

| opcode | invented name | verdict | what it actually is | corrected name |
|---|---|---|---|---|
| 0x0070 | CLIENT_VERSION_ECHO | **ESTABLISHED** | multiplexed report, subtype in byte 0; version 100 is a client literal, not an echo | `CLIENT_ENV_REPORT` |
| 0x0071 | CLIENT_VERSION_DETAIL | **ESTABLISHED** | version pair + 4 config flags + crypto self-test result + 30-byte blob | `CLIENT_ENV_DETAIL` |
| 0x0079 | CLIENT_CHARACTER_REPORT | **ESTABLISHED** | load timings for the selected character, + two SYSTEMTIMEs | `CLIENT_LOAD_TIMING_REPORT` |
| 0x007A | CLIENT_DISCONNECT_NOTICE | **ESTABLISHED** | "all four background tasks finished", with each task's duration | `CLIENT_TASK_TIMING_REPORT` |
| 0x00A6 | CLIENT_ASSET_TICK | **ESTABLISHED** | numbered status/error code with a code-dependent payload | `CLIENT_STATUS_CODE` |
| 0x00BF | CLIENT_READY | **ESTABLISHED** | sent when the login title screen is constructed | `CLIENT_TITLE_SCREEN_READY` |
| 0x00C0 | CLIENT_HELLO | **ESTABLISHED** | auth failure: launch mode + passport error code | `CLIENT_AUTH_FAILURE_REPORT` |
| 0x00BC | CLIENT_LOCALE | **UNKNOWN** | builder not readable; body is three identical u32 = 1033 | *(no name — log as UNKNOWN)* |

## The writers, for reading the field lists below

| function | writes |
|---|---|
| `FUN_1406ed520(buf, op)` | begin packet |
| `FUN_1406ed840(buf, v)` | u8 |
| `FUN_1406ed9d0(buf, v)` | u32 |
| `FUN_1406edc80(buf, str)` | **u16 length, then that many bytes** (confirmed: `05 00 "Hello"`) |
| `FUN_1406ede20(buf, p, n)` | n raw bytes |
| `FUN_141b41d00(buf, p)` | 4 raw bytes (thin wrapper over `FUN_1406ede20`) |
| `FUN_1415d01c0` / `FUN_1415d3990` | send |

---

## 0x0070 — ESTABLISHED

**Builders:** `FUN_1415d5b40` @ `1415d5b40` (subtype 2 — this is the 45-byte packet we
captured), `FUN_1415d8b20` @ `1415d8b20` (subtype 4), `FUN_140c8fdc0` @ `140c8fdc0`
(subtype 5), and `FUN_140caae50` @ `140caae50` (subtype 13). `FUN_1415d5b40` is called
from the handshake `FUN_1415d10e0` at `1415d28bd`, 31 bytes before the `0x0071` call.

`FUN_1415d5b40` writes `u8 = 2`, `u32 = 100`, then delegates the rest to either
`FUN_140c7b560` or `FUN_140c7b230` — chosen by `FUN_140739170(5)`, which is a
**1-in-5 random sampler** (`rand() % 5 == 0`). Both helpers write the same ten u32s.
The 45-byte capture maps exactly:

```
02            u8  subtype
64000000      u32 100                       <- hardcoded literal 100, NOT an echo
00000000      u32 local_50c  (FUN_140c8c1b0 / FUN_140c80c20)
00000000      u32 local_508  (FUN_140c84e80)
02000000      u32 FUN_1402add30()  == constant 2
23d00523      u32 nonce ^ K1
6860a49c      u32 0 ^ nonce ^ K2
808dd95e      u32 0 ^ nonce ^ K3
56495b31      u32 (0 ^ nonce) - K4
b930795c      u32 (0 ^ nonce) + K5          <- carries a 1-in-200 sampled flag
00000000      u32
00000000      u32
```

`nonce = FUN_140738db0(0, 0x7fffffff)`, fresh per packet. **The "20 bytes" that looked
like a SHA-1 is not a hash** — it is five small integers masked against that nonce.

The 46-byte capture (`01 6400...`) is subtype 1 and comes from a builder that is not
statically visible, but it is proven to carry the *same* masked block: realigned one
byte later, its first three masked u32s XOR-agree with the 45-byte capture
(`f0^f1 = bfa1b04b` and `f0^f2 = 7ddc5da3` in **both** captures). Fields 4 and 5 use
`-K4`/`+K5` rather than XOR, so they legitimately differ.

**Why the old name is wrong:** twice. `0x0070` is not one message but a subtype-
multiplexed one (four subtypes located: 2, 4, 5, 13, each with a different body), and
`100` is a literal compiled into `FUN_1415d5b40`. **If we changed the protocol version
we send, the client would still send 100.** Nothing here echoes us.

Corrected name: **`CLIENT_ENV_REPORT`** (subtype in byte 0).

## 0x0071 — ESTABLISHED

**Builder:** `FUN_1415d5c20` @ `1415d5c20`, called from the handshake `FUN_1415d10e0`
at `1415d28dc`. The 49-byte capture maps completely:

```
01                    u8  literal 1
01000000              u32 literal 1
64000000              u32 literal 100
00                    u8  literal 0
00                    u8  FUN_142cb8610(cfg)  == *(u32*)(cfg+0x22BC)
00                    u8  *(u8*)(cfg+0x2520)
00                    u8  *(u8*)(cfg+0x2524)
00                    u8  *(u8*)(cfg+0x2528)
01                    u8  FUN_1415dcc90()  == AES round-trip self-test, 1 = passed
1e000000 <30 bytes>   FUN_1415dc6f0: u32 length + that many bytes
```

`FUN_1415dcc90` generates a random string, encrypts it with `FUN_1415efc80`, decrypts
it with `FUN_1415efa80` under the same key handle, and compares — a **crypto
self-test**. `FUN_1415dc6f0` -> `FUN_1415dd590` decrypts a stored value under a second
key handle into a byte vector, or substitutes `DAT_143ace350` (all zeros) if that
self-test failed; `FUN_1415dea70` then writes it as `u32 length` + bytes.

**Why the old name is wrong:** the same premise error as `0x0070` — `1` and `100` are
compiled-in literals, not a reflection of the version triple we send. The packet is a
client environment/integrity statement.

Corrected name: **`CLIENT_ENV_DETAIL`**.

## 0x0079 — ESTABLISHED

**Builder:** `FUN_141b0ecc0` @ `141b0ecc0` (no callers — reached via vtable or from the
virtualised region). Exact field list:

```
FUN_1406ed9d0(pkt, param_1)                     u32   character id
FUN_1406edc80(pkt, param_2)                     str   character name (u16 len + bytes)
FUN_1406ed9d0(pkt, FUN_141b0ec20(0..3))         4x u32  four task durations, ms
FUN_1406ed9d0(pkt, DAT_143ad1ef4 - DAT_143ad1ef0) u32 a fifth duration, ms
FUN_1406ede20(pkt, &DAT_143ad1ed0, 0x10)        16B   SYSTEMTIME
FUN_1406ede20(pkt, &DAT_143ad1ee0, 0x10)        16B   SYSTEMTIME
```

`FUN_141b0ec20(i)` reads `(&DAT_143ad1ec0)[i]`, an array of four u32 written *only* by
`FUN_141b0ebb0(i, v)`, which is called *only* from the worker `FUN_141b0ef00` at five
sites, each of the form `FUN_141b0ebb0(slot, GetTickCount() - startTick)`. So the four
values are **elapsed milliseconds for four background tasks**.

`DAT_143ad1ee0` is written by `FUN_141b0eb80` from `FUN_1408f66e0`, which computes
`FileTimeToSystemTime((GetTickCount() - base_tick) * 10000 + base_filetime)`. When the
FILETIME base is never initialised (our case — no server supplies it) this renders as
`1601-01-01 00:00:00.xxx`, which is exactly what both captures show, and exactly what
the client's own ELog string in `0x008F` shows (`DATETIME|1601/01/01 00:00:11`).

**Two corrections to the premise.** The name is `u16 len + bytes`, so
`09 00 "TestChar" 44` is length 9 and the character was named **"TestCharD"** — there
is no separate `0x44` field. And `d8070000` is not "2008 the year": it is the fifth
duration (2008 ms; the fixture run shows 6289 ms). The SYSTEMTIME begins at the *next*
byte, `ea070800...` = 2026-08-18.

Corrected name: **`CLIENT_LOAD_TIMING_REPORT`**.

## 0x007A — ESTABLISHED

**Builder:** `FUN_142c4f490` @ `142c4f490` (already in `research/msexe-disconnect007a.c`).
**Caller:** `FUN_141b0ef00` @ `141b0ef00`, the background worker, at `141b0f129`:

```c
if (iStack_38 && iStack_34 && iStack_30 && iStack_2c) {   // all four tasks done
    ...
    *(int *)(param_1 + 0x30) = 2;                          // worker state -> 2
    if (FUN_14090d160(0x8e, 1) == 1)
        FUN_142c4f490(DAT_143ac1898);                      // <-- sends 0x007A
    FUN_141b0eb80(now16);                                  // stamps DAT_143ad1ee0
}
```

Body: `u8 = (session[0xEE] == 0)`, `u8 = FUN_141b0f910()` which is
`*(int*)(session+0x30) == 2` (the worker state just set), and *only if that is true*,
the same four task durations and then `FUN_141b0ec60()` = **their sum**.

The arithmetic settles it beyond doubt — the last u32 is the sum of the preceding four
in every capture:

| capture | four durations | last u32 | sum |
|---|---|---|---|
| `login.log` #1 | 64, 10, 10, 31 | 115 | 115 ✓ |
| `login.log` #2 | 63, 10, 10, 32 | 115 | 115 ✓ |
| `exit-is-fastfail…probe.log` #31 | 83, 18, 15, 70 | 186 | 186 ✓ |

**Why the old name is wrong:** nothing on this path relates to disconnecting. It is
sent exactly once, at the moment the client's four background tasks all complete —
"late in the run" because that is when they finish, not because the client is leaving.
A server that logged this as a disconnect notice would mislead about a healthy event.

Corrected name: **`CLIENT_TASK_TIMING_REPORT`**.

## 0x00A6 — ESTABLISHED

**Builder:** `FUN_142c8b9f0(int code, u32 *detail)` @ `142c8b9f0`.

```c
FUN_1406ed520(pkt, 0xa6);
FUN_1406ed9d0(pkt, code);              // always: u32 code
if (code == 0x10) { u32 detail[2]; u32 detail[6]; }
switch (code) {
  case 1:        FUN_1406edc80(pkt, detail + 4); break;   // u16-len string
  case 6..10:    u32 detail[1];  break;
  case 0xb:      u32 detail[0];  break;
  default:       (nothing more)
}
DAT_143ae0268 = code;                   // remembers the last code
```

Every observed length is accounted for:

| body | code | extra | total |
|---|---|---|---|
| `010000000000` | 1 | empty string (`00 00`) | 6 ✓ |
| `0b00000000000000` | 11 | one u32 | 8 ✓ |
| `12000000` | 18 | none (default arm) | 4 ✓ |

One caller is located: `FUN_142c4d020` @ `142c4d020` sends `FUN_142c8b9f0(0x10, ...)`
when `DAT_143add050` (the `Gr2D_DX11` device) is NULL, and the very next statement is
`FUN_142f27b90(0)`, which does not return. So code `0x10` is a **fatal graphics-init
failure report sent immediately before the client aborts**.

**Why the old name is wrong:** these are not asset ticks. The leading u32 is a numbered
status/error code in a general reporting channel that also carries fatal errors. The
"mostly-incrementing small integers" during startup are progress codes, but naming the
opcode after that one usage hides that the same opcode reports crashes.

Corrected name: **`CLIENT_STATUS_CODE`**.

## 0x00BF — ESTABLISHED

**Builder:** `FUN_1415daff0` @ `1415daff0` — opcode only, no fields, sent via
`FUN_1415d3990`. **Single caller:** `FUN_141b5bb50` @ `141b5bb50`, which constructs the
login title screen: it loads `UI/Login.img/Title.new/backgrd`
(`PTR_s_UI_Login_img_Title_new_backgrd_143a44338`), measures and centres it, and ends
with:

```c
FUN_1418060c0(param_1, w, h, u"UI/Login.img/Title.new/backgrd", 0x271a, ...);
if (DAT_143a88df8 != 0)
    FUN_1415daff0(DAT_143ac18a0);      // <-- sends 0x00BF
```

`CLIENT_READY` is the only one of the eight that is not actively misleading, but it is
under-specified: the event is specifically *the login title screen has been built*, and
it is conditional on `DAT_143a88df8`.

Corrected name: **`CLIENT_TITLE_SCREEN_READY`**.

## 0x00C0 — ESTABLISHED

**Builder:** `FUN_141b2a660` @ `141b2a660`, the login submit handler. The packet is
built **only on the failure branch**:

```c
local_d18 = FUN_141d60eb0(user, pass, 0xc9, 0);      // auth call
if (local_d18 == 0) { FUN_141d60fa0(local_418); }    // success: no 0x00C0
else {
    local_d14 = 9;                                    // map error -> dialog message id
    if (local_d18 == 20000 || local_d18 == 0x4e35) local_d14 = 0xc;
    else if (local_d18 == 0x4e22 || local_d18 == 0x4e3e) local_d14 = 8;
    ... (0x4e23, 0x4e26, 0x4e27, 0x4e39, 0x4e3a, 0x4e3b, 0x4e3c, 0x4e45, 0x4e6c) ...
    FUN_1406ed520(pkt, 0xc0);
    local_d08 = FUN_142c4a810(FUN_1415e3d00());       // session->[0x68]
    FUN_141b41d00(pkt, &local_d08);                   // 4 raw bytes
    FUN_1406ed9d0(pkt, local_d18);                    // u32 error code
    FUN_1415d01c0(pkt);
    FUN_141b2a280(param_1, local_d14, 0);             // show the error dialog
}
```

`FUN_1415e3d00()` returns `DAT_143ac1898` (the session) and `FUN_142c4a810(p)` returns
`*(u32 *)(p + 0x68)` — **exactly the launch mode**, confirming the premise from the
outbound `0x0073` side independently.

The observed body `05000000 204e0000` therefore reads: launch mode 5, auth error
`0x4E20` = **20000**. The whole error-code table (20000, 0x4E22..0x4E6C) is the Nexon
Passport range, and 20000 maps to dialog message id `0xC`.

**Why the old name is wrong, and why it matters:** this is not a greeting. It exists
only when authentication fails, and its second field is a diagnosis. Every time the log
said "CLIENT_HELLO" the client was in fact reporting error 20000 — a failure that was
being read as a handshake step. The leading `5` is the launch mode, which is why it
looked like a hello.

Corrected name: **`CLIENT_AUTH_FAILURE_REPORT`**.

## 0x00BC — UNKNOWN

No builder found. What was tried, and the control for each:

**1. The existing table.** `research/msexe-send-opcodes.txt` resolves 1881 of 1894
call sites of `FUN_1406ed520` and has no `0x00BC` entry. Not evidence on its own.

**2. All 13 unresolved (`????`) builders were decompiled.** Every one takes the opcode
as a *parameter* (`FUN_1406ed520(pkt, param_1)`) — that is why they are unresolved.
None of them writes a 12-byte body: `FUN_140ca3130`, `FUN_140ca3320`, `FUN_1413106e0`,
`FUN_141d3b880`, `FUN_142dbb600` write 4+4 bytes; `FUN_1409fe0a0` writes 4;
`FUN_140ca30c0` writes a string. So `0x00BC` is not one of these.

**3. A byte-level scanner, validated against the whole known table.**
`mov edx, imm32` (`BA` + imm) followed within 32 bytes by a direct `call 0x1406ED520`.
Across all **657** opcodes in `msexe-send-opcodes.txt` it recovers **1872 of 1881**
known call sites (the 9 misses are opcodes loaded by a non-`BA` form, e.g. `xor edx,edx`
for opcode 0). **This is the positive control: the instrument demonstrably works.**

- `BA BC 00 00 00` occurs **11 times** in `.text`, and **not once** before a call to
  `FUN_1406ed520`.
- A wider scan (any `mov r32, 0xBC`, any register) surfaced two candidates,
  `141f227b0` and `141f25408`. Both were disassembled (`DumpAsm`) and both are **false
  positives**: the matched bytes are `80 bb bc 00 00 00 00` = `CMP byte ptr [RBX+0xbc],0`,
  and the real opcode at those call sites is `0x332`.
- `mov ecx, 0xBC` before a call to any of the 13 generic helpers: none.

**4. The negative control that makes this honest.** `0x0078`
(`CLIENT_SELECT_CHARACTER_REQUEST`) is a *confirmed, measured, answered* opcode — the
server replies to it and character selection works. It is **equally invisible**: absent
from `msexe-send-opcodes.txt`, and `BA 78 00 00 00` occurs 403 times in `.text` with
**zero** hits before a call to `FUN_1406ed520`. A method that cannot find `0x0078`
cannot be used to conclude anything about whether `0x00BC`'s builder exists.

**5. Why.** The image carries `.themida` (0x13EC000 virtual, 0 raw — materialised at
runtime), `.vm_sec` (0x40000) and `.boot` (0xC37400) sections, and the import table is
only ~100 entries, so almost every API is resolved at runtime. In
`research/fixtures/exit-is-fastfail-c0000409-probe.log` packets #35 (`0x00BC`), #36
(`0x0078`) and #37 (`0x0079`) are emitted in the same instant on the character-select
click. `0x00BC` and `0x0078` are almost certainly built by the same virtualised code —
which is consistent with `0x0078` being unreadable too.

**What is actually known:** the body is 12 bytes, three identical little-endian u32
each equal to `0x00000409` = 1033, sent once, immediately before the select-character
request. 1033 is the Windows LCID for en-US, so "a locale triple" is a *reasonable*
guess — but it is a guess about a value, not a reading of the code, and 1033 is a
plausible value for other things too. Nothing in the client was read that says what
these three fields are.

**Recommendation:** remove the name. Let it fall through to `UNKNOWN` so the body is
logged in full and a future run can be compared against it. If a placeholder is wanted,
it must be marked as unverified, e.g.
`"0x00BC (3x 0x409, meaning unknown, never answered)"` — a description of the bytes, not
a claim about them.

---

## Corrections to the premises in the original table

- **0x0070 / 0x0071 do not echo our version.** `100` (and `1`) are literals compiled
  into `FUN_1415d5b40` and `FUN_1415d5c20`. Changing the version we send will not
  change these packets. Any inference that treated them as a reflection of our
  handshake is unfounded.
- **0x0070's "20 bytes" is not a hash.** It is five u32 masked with a per-packet random
  nonce, proven by the pairwise-XOR agreement between two independent captures.
- **0x0079's name is 9 bytes.** `FUN_1406edc80` writes `u16 length + bytes`; the
  character in that capture was `"TestCharD"`, and the trailing `44` is part of it.
- **0x0079's `d8070000` is not the year 2008.** It is a millisecond duration; the
  SYSTEMTIME starts at the following byte.
- **0x007A is not a disconnect.** It is a completion report; its last field is
  arithmetically the sum of the four preceding ones in all three captures.
- **0x00C0 only exists on the auth-failure path**, and the `0x4E20` we kept seeing is
  error 20000.

## Files produced/used

- Decompiles and disassembly for this pass are in the session scratchpad; the
  load-bearing ones are reproduced inline above.
- Pre-existing: `research/msexe-send-opcodes.txt`, `research/msexe-packet-fields.txt`,
  `research/msexe-disconnect007a.c`, `research/msexe-postshake.c`.
