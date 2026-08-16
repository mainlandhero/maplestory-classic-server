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

So the text is a **baked bitmap**, drawn from a canvas rather than resolved from a string.

**It is still a real signal.** The owner, who knows the live game: the live client **never** shows
it, and ours shows it **immediately after the splash screen** — before any of the
`0x0073` / `0x0080` / `0x0010` exchange. So it is conditional, and the condition is
evaluated when the login screen is built, not by anything we send. An earlier note here
guessed it might be permanent screen furniture; that is wrong and direct observation
settles it.

Two consequences:

- Nothing in the login *packet* exchange can clear it. Chasing it through opcodes is
  chasing the wrong half of the client.
- Because it appears at screen-construction time, whatever is wrong is already wrong when
  the client opens.

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
