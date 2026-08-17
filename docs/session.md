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

So the text is a **baked bitmap**. It has since been found and rendered:

**`/Notice/text/loginTroubleAskSupport`** in `Login.img` (215x86, canvas format 1):

> Having trouble logging in?
> Try logging in at maplestory.nexon.net
> or visit the Nexon homepage
> to view support options.

Read it yourself with:

```bash
./target/release/wz-dump.exe canvas client-patched/Data/UI/_Canvas/_Canvas_000.wz \
    Login.img <outdir> "Notice/text/login"
python tools/wz_png.py <outdir>
```

`/Notice/text/` is the client's **entire baked message table** - ~170 canvases on the same
parchment frame, named (`incorrectPassword`, `notRegisteredID`, `blockedID`,
`accountSuspended`, `loginAlready`, `loginTimeout`, `unableLogOnToGameSvr`, ...) and
numbered (`28` = "You have entered an invalid login ID", `32` = "The server is under
maintenance", `38` = "GameGuard has been updated"). The numeric keys are **not** the login
result codes - there is no `101`/`0x65` entry - so the login result resolves through the
named keys.

### SOLVED: `FUN_141b2a280` raises it

Found by logging the return address at a watch on the notice display:
`called-from=0x141b2a61e`, inside `FUN_141b2a280(stage, code, flag)`.

```c
if ((code + 1U < 0xe) && ((0x2681U >> (code + 1U & 0x1f) & 1) != 0)) {
    ... show loginTroubleAskSupport ...
}
```

`0x2681` has bits 0, 7, 9, 10 and 13 set and the index is `code + 1`, so **codes -1, 6, 8,
9 and 12** produce this dialog. `code == 0` returns 1 - success. Everything else maps to a
specific notice:

| code | notice |
|---|---|
| **-1, 6, 8, 9, 12** | **`loginTroubleAskSupport`** |
| 2, 3 | `blockedID` |
| 4 | `incorrectPassword` |
| 5 | `notRegisteredID` |
| 7 | `loginAlready` |
| 0x0A | `loginTimeout` |
| 0x0B | `notAdult` |
| 0x0D | `blockedIPAddr` |
| 0x0E | `invalidRegion` |
| 0x0F | `notRegisteredAccount` |
| 0x11, 0x87 | `accountNotVerified` |
| 0x44 | `cannotProcessRequest` |
| 0x52 | `notVerifiedEmail` |
| 0x88 | `invalidRegion` |
| 0x8B | `outOfServiceRegion` |

**Measured, live: the code is 12** - a generic failure with no specific notice - and
`called-from` is `0x144c05eb2`, inside `.themida`. **The caller is virtualised**, and the VM
runs on its own stack, so a 16-slot stack scan at the call found no `.text` frame. *Who*
decides code 12 cannot be answered by reading or by walking back from the call.

`FUN_141b2a280` is a near-duplicate of `FUN_141b267c0`: same mapping, different function.

#### Why every scan missed it, and the lesson

`FUN_141b2a280` never takes the string's *address*. It **copies the literal inline**, eight
bytes at a time, with RIP-relative `mov` from `0x1433d5d98`. `tools/xref.py` matches `lea`,
so it reported three references - all later proven never entered by watch - while the real
raiser was invisible to every scan built on it.

**A "0 references" result from `xref.py` means "nothing takes its address", not "nothing
uses it".** That warning now leads the tool's own docstring.

#### Two theories that were measured and disproved

Recorded because each looked convincing on paper, and because the pattern matters more than
either result:

| Theory | Disproved by |
|---|---|
| `FUN_141b267c0` (login result codes) | watch armed before the login screen; dialog appeared; **never entered** |
| `FUN_1415d9210` (`+0x2270` bit 2, `+0x227c`) | live read: `+0x2270 = 0x00`, bit clear, so it returns before raising anything |
| `FUN_141804140` (error code -> notice name) | watch: **never entered** |

Each was a plausible chain built from a matching string plus a matching default case.
**Measure before building on a static chain.**

## What actually enables the Login button

**One byte: `stage + 0x108`.** The owner: the Login button starts *disabled* in an invalid
session, and becomes clickable in a valid one.

`FUN_14112a720`, the `ClassicIntro` tick, does exactly that:

```c
if (FUN_141b2a160(stage) != '\0') {          // FUN_141b2a160 = *(u8 *)(stage + 0x108)
    FUN_142aa2010(this + 0x240, L"login", 1);  // enable the control named "login"
    ...
}
```

`FUN_142aa2010(container, name, value)` looks a control up by name and calls its vtable
slot `+0x70`. The screen builder `FUN_141129930` calls it with **`0`** at construction, so
the button is born disabled and this tick is the only thing that turns it on.

**`stage + 0x108` is set by the world-list handler**, one line above the append:

```c
*(undefined1 *)(param_1 + 0x108) = 1;
piVar10 = (int *)FUN_141b44520(param_1 + 0x100, 0xffffffff);
```

So the list (`+0x100`) and the flag (`+0x108`) are written together by inbound **`0x000B`**,
which we have never sent. Only a real world entry does this - the terminator branch
(`worldId < 0`) returns before either.

That makes `0x000B` the single highest-value packet outstanding: it enables the button, it
populates the list the login result searches, and it is the one thing the client has been
missing since it first reached the login screen.

**The account field is separate.** In the same tick, `FUN_142cb83a0(DAT_143aa84a0, &s)` -
which is just `DAT_143aa84a0 + 0x22f8` - is rendered into the control at `+0x258`
(`textAccount`) when non-empty. Note the object: `DAT_143aa84a0`, *not* the `DAT_143ac1898`
that carries the `0x0073` identity. The same object also holds the world id (`+0x2258`) and
channel id (`+0x2260`) the login result compares against.

## Stage ids

Read out of `FUN_141127730`, which registers each stage id against a screen name - data,
not inference:

| Stage | Screen |
|---|---|
| 1 | Title |
| 2 | WorldSelect |
| 3 | **ClassicIntro** |
| 4 | **CharSelect** |
| 5 | NewChar |

Two independent checks agree with the owner's description of the live client: the classic Login
button does `FUN_141b3f050(stage, 4, 600)` - straight to CharSelect - and mode 5 sends us to
stage 3, the legacy login screen.

`FUN_141b21ea0` picks the stage on entry:

```c
if (FUN_1411284d0() == 2)                       -> stage 2, sends 0x0073 + 0x0080
else if (session+0x68 == 5)                     -> stage 3, sends 0x0073 + 0x0080   <- us
else                                            -> stage 1, sends nothing
```

**The login result does not advance the stage.** Both of its transitions are
`FUN_141b3f050(stage, 3, 800)` - re-entering stage 3, the stage the client is already in
when it sends `0x0080`. That is useful: answering `0x0010` keeps the connection alive
without skipping past the login screen.

### The Login button behaves differently in mode 5

```c
if (FUN_141b3fd10(stage))  FUN_141b3ff10(stage);          // mode == 5  <- us
else                       FUN_141b3f050(stage, 4, 600);  // -> CharSelect
```

Only the second path is readable. `FUN_141b3ff10` calls `FUN_141b2ba60(stage, 0x50, 0, 0, 0)`
and raises *"unable to log on to game server"* if it returns 0 - and **`FUN_141b2ba60` is
virtualised**: 50 bytes of prologue then `jmp 0x144C74CD1`, inside `.themida`. It cannot be
decompiled, only observed.

So there are two ways to reach CharSelect by clicking Login:

1. **Observe the mode-5 path.** Enable the button, click it, and read what goes out. One
   run, and it costs nothing extra because the packets are already built.
2. **Leave mode 5.** `session+0x68` is a `u32` at `[0x143ac1898] + 0x68`; with any other
   value the button takes the readable `FUN_141b3f050(stage, 4, 600)` straight to
   CharSelect. `crates/grap-stub` already patches memory in-process. Note this **sidesteps**
   the question rather than answering it - it makes the client behave, it does not make the
   session valid - and it also switches `0x000B` to the classic handler, so it should be
   done *after* the world list lands.

## Two login screens, and we are probably on the wrong one

`Login.img` contains two:

| Node | Contents | Which client |
|---|---|---|
| `Title_new` | `nexonID`, `mapleID`, `PW`, `BtLogin`, `BtEmailSave`, **`BtEmailLost`**, **`BtPasswdLost`** | matches the owner's live screenshots — masked account email, active Login button |
| `ClassicIntro` | `textAccount`, `textPassword`, `check_saved`, **`button:find_id`**, **`button:find_pw`** | the legacy user/password screen |

`FUN_141129930` — the builder we have read — uses `L"textAccount"`, `L"textPassword"` and
`L"check_saved"`, so **it builds `ClassicIntro`**. A "having trouble logging in?" label is
the kind of thing that sits above `find_id` / `find_pw`.

**Unproven:** that the bitmap belongs to `ClassicIntro` is inference, not yet a reading of
the pixels. It is the most likely explanation for "live never shows it, ours always does",
but it should be confirmed before more is built on it — see "Next" below.

### How the screen is chosen

`FUN_141b27da0(stage, stageId)` shows or hides seven screens, gating each on
`FUN_141128090(stageId, index)`. **`ClassicIntro` is index 4** — its constructor
`FUN_1411297e0` installs the vtable at `0x143383648`, which holds both `FUN_141129930` (the
UI builder) and `FUN_14112a570` (its button handler).

`FUN_141128090(stageId, index)` is a `std::map` lookup on `DAT_143aca3d8`: stage id ->
a list of screen indices, asking whether `index` is in the list. The map is **built at
runtime**, so the table is not readable straight out of the file; finding what populates
`DAT_143aca3d8` gives the whole stage-to-screen mapping at once.

## Better oracles than the screen

These do not replace the prompt as evidence — see above, it is a real signal — but they are
readable without anyone interpreting a screenshot, which matters when each run costs a
manual launch:

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

## Next - the account name is server-supplied

**The masked email is a packet field, not a launcher handoff.** The login screen renders
`DAT_143aa84a0 + 0x22f8` into `textAccount` when it is non-empty (`FUN_14112a720`), and that
field is written by `FUN_142cb8370(obj, str)`, which then calls `FUN_141128960(4)` to
refresh the UI. Its **only two callers are login-stage packet handlers**:

| Inbound | Handler |
|---|---|
| **`0x0000`** | `FUN_141b2dd00` (4475 bytes - also one of the three functions referencing `loginTroubleAskSupport`) |
| **`0x0012`** | `FUN_141b2ee90` |

So the client expects the server to tell it who it is. That reframes the whole session
question: rather than faking a launcher handoff, send `0x0000`.

Found by scanning `.text` for the disp32 `0x22f8` and filtering to the account-manager
range - **not** by `xref.py`, which finds only `lea` and would have missed a struct-offset
store the same way it missed `FUN_141b2a280`.

**`0x0000` is now decoded** - see `docs/opcodes.md`. It is a second, fuller login result:
`u8 result`, `str message`, the *same* result gate as `0x0010` (`FUN_141b267c0`), and then
about twenty fields ending in the account name. `net::opcode::account_info` builds it.

Still open, in order:

1. **Send `0x0000`** and find out whether the account name appears. It is built and tested
   but has never been on the wire.
2. **Decode `0x0012`** (`FUN_141b2ee90`, 1868 bytes) - the fallback if `0x0000` turns out to
   be the wrong one of the two. Same opening, same gate, different middle.
3. ~~Find the character-list packet.~~ **Done** - it is inside `0x0010`, decoded by
   `FUN_14108bdf0`. The whole character transaction is in **`docs/character.md`**.
   (`FUN_141b28570` was nominated for this and is *not* it - a static guess that was never
   read before being written down.)
4. **Only if those fail:** the `CNM*` interface in `nexon_api_x64.dll` / `nmcogame64.dll`.
   Both are unpacked. This was the standing assumption for weeks and is now the *fallback*,
   because the account name turning out to be server-supplied suggests the session may be
   too.

Settled and no longer worth pursuing:

* ~~**The client never migrates.**~~ **Retracted 2026-08-18.** That rested on a `connect`
  hook logging nothing - but the hook has never logged a `CONNECT` line at all, including
  for the connection to our own server, which certainly happened. Until the hook is shown
  to work, its silence is not evidence. `netwatch` now self-tests at install; read that
  line before reading its output. The migration question is open again, and so is whether
  a channel server or an address field is needed.
* `DAT_143ac1898 + 0x1b8` (the `0x0073` identity string) is still empty and still unwritten
  by anything we can find, but it did not stop login and is no longer the lead.

## RETRACTED: the connection was never dying

**There was no close to explain.** A `Get-NetTCPConnection` poll showed *both* endpoints
`Established` from before the supposed death right through to the client exiting half a
minute later. The whole thing came from two defects in `handshake_probe.py`'s own logging:
`log()` did not timestamp, so the untimestamped `connection reset by client` line - which
fires when the receive loop exits, i.e. when the **client** exits - sat under the last
timestamped line and read as happening there; and `connection lasted 0.0s after the reply
was sent` subtracted `time.time()` from itself in `--reply-to` mode, printing `0.0s` on
every run ever done. Both are fixed.

Everything below was written while chasing that phantom. It is kept because the findings
about `0x007A` are correct and useful, but the framing - "the close" - is not.

### `0x007A` is a loading-complete report

**`0x007A` is a loading-complete report, and the close follows it.** The client's last
packet before dropping the connection is `0x007A`, body
`01 01 43 00 00 00 0b 00 00 00 0a 00 00 00 22 00 00 00 7a 00 00 00`. Its builder is
`FUN_142c4f490`, and its only caller is `FUN_141b0ef00` - the boot task loop, which drives
four load phases (bits 1, 2, 4, 8 of `+0x34`) and, **once all four have finished**, emits
this packet carrying `FUN_141b0ebb0`'s per-phase durations. The five `u32`s are
milliseconds: 67, 11, 10, 34, 122.

So the close arrives on the heels of *loading finishing*, not of a quiet socket. Those two
have been confounded because in every run so far they happen at the same moment, and
"~8s idle timeout" was written down on the strength of the timing alone. It is at best
unproven.

`FUN_141b0ef00` had **no Ghidra function at all** - it is reached only from virtualised
code, so auto-analysis never created one. `tools/ghidra_scripts/DecompileFunc.java` now
creates a function when none exists, which is worth knowing generally: for this binary, "no
function there" is the normal state for the interesting handlers, not a dead end.

**Next:** `netwatch` now also breakpoints `closesocket` and `shutdown` and logs
`called-from` plus a stack scan. Whoever tears the socket down names the moment, the same
way the `connect` hook settled migration. Unlike the login-failure path this call is
ordinary networking teardown and is unlikely to be virtualised, so expect a `.text` address
that can be decompiled.
