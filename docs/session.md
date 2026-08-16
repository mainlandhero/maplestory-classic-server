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

### Which result codes raise it

`FUN_141b267c0(stage, result, ...)` is the map, and it switches on `result + 1`:

| Result | Notice |
|---|---|
| **-1, 6, 8, 9** | **`loginTroubleAskSupport`** |
| 3 | `blockedID` |
| 4 | `incorrectPassword` |
| 5 | `notRegisteredID` |
| 7 | `loginAlready` |

So the prompt is a **login result dialog**: the client believes it got a failing login
result of -1, 6, 8 or 9. No caller passes -1 as an immediate (checked with
`FindConstArgCalls`), so the value arrives in a variable.

**Unresolved, and the top question for next session:** the owner sees this dialog *immediately
after the splash screen*, before we have sent anything but the `0x0032` gate - and `0x0032`
is handled by `FUN_1415e5c20`, which never touches this path. Something is reaching
`FUN_141b267c0` with a failing code before any login exchange. Two of its callers,
`FUN_141b2b120` (a 31-byte wrapper passing the code straight through) and `FUN_141b2ae80`,
have **no callers and are in no vtable**, so they are reached only through the virtualised
dispatcher and cannot be traced statically.

The way to settle it is to **observe**: hook `FUN_141b267c0` and log `param_2` and when it
fires. That names the code, and the table above names the failure. `crates/grap-stub`
already does inline hooks; today's watch mode only reports *whether* a function ran, so it
needs to also capture an argument.

**It is a real signal, not decoration.** The owner, who knows the live game: the live client
**never** shows it, and ours shows it **immediately after the splash screen**. An earlier
note here guessed it might be permanent screen furniture; that is wrong, and the render
above settles it - this is the failure dialog from the login result table.

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

## Next

Ordered by what settles the most per unit of work, and none of it needs a client launch:

1. **Read the pixels.** `crates/wz` parses every canvas node's metadata but deliberately
   skips pixel data (`prop.rs`: "Pixels are left unparsed"). Decoding them - zlib plus a
   handful of pixel formats - would let us *read the client's baked UI text*, which is the
   exact capability gap that made this prompt unfindable in the first place. It would say
   definitively which node carries "having trouble logging in", and therefore which screen
   we are on. Useful far beyond this one question.
2. **Find what populates `DAT_143aca3d8`**, the stage -> screen-index map. That yields the
   full mapping in one read, including which stage shows `Title_new` rather than
   `ClassicIntro`.
3. **Find the writer of `DAT_143ac1898 + 0x1b8`.** It is a single field; whatever fills it
   is the launcher handoff. The `CNM*` interface in `nexon_api_x64.dll` /
   `nmcogame64.dll` is the likely home, and both are unpacked and far easier to read than
   the Themida-wrapped exe.
4. **Write that field directly** from `crates/grap-stub`, which already runs in-process
   with inline-hook and memory-patch capability. Pointing `+0x1b8` at a string we own
   settles whether the empty identity changes the screen choice, without solving the
   handoff first.

## What the empty identity does *not* block

Worth stating plainly, because it was assumed for a while: an empty identity **did not stop
the client from logging in**. It still built and sent `0x0073` *and* `0x0080`, and it
accepted a `result = 0` login reply and advanced its UI to character select. At this stage
the client is blocked on what we send it, not on its own state.

So the identity is a real gap, but it is not what stops the *packet* flow - the missing
world list (`0x000B`) is that.

It remains the best candidate for the **screen** choice, though, and that is a separate
question decided before any packet is sent.
