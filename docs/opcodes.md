# Opcodes

## Where these come from

The client's **outbound** opcodes are recoverable from the binary. `FUN_1406ed520(buf, op)`
begins a packet, so every call site names one opcode, and the calls that follow append its
fields. `tools/ghidra_scripts/DumpPacketFields.java` walks all 1894 call sites and records
the opcode, the field sequence, and every string the builder references;
`tools/label_opcodes.py` turns that into the table below.

The client's **inbound** opcodes cannot be recovered the same way: the dispatcher
`FUN_1415d60e0` tail-jumps into the `.themida` section, which has no file bytes — see
`docs/transport.md`. But they are not out of reach, and **blind sweeping is retired**.

Two routes, cheapest first:

1. **Read the stage's `OnPacket` switch.** The virtualised dispatcher only *routes*: it
   hands a stage its opcode, and the stage dispatches in ordinary code. `FUN_141b25f30` is
   the login stage's, and it is a plain `switch` naming every login-stage opcode at once
   (see "The login stage" below). Always look for this first.
2. **Walk the opcode space inside the client**, when no readable switch covers it —
   `crates/grap-stub/src/probe.rs`, driven by `-Probe` on `tools/test-one.ps1`. It rewrites
   the opcode of one captured packet and re-dispatches, covering the whole enum in a single
   client launch. Aim it with `tools/handler_root.py`; the ways it can silently lie are
   listed in `STATUS.md`.

## On published opcode lists

Public MapleStory opcode tables are for other versions and should not be trusted here.
This client's outbound space **starts at `0x0070`** — only `0x0000` sits below it — whereas
classic v62/v83 numbering puts the login packet at `0x01`. The numbers will not transfer.

What *does* transfer is structure: field layouts for a given operation are far more stable
across versions than the opcode numbers. So published lists are worth consulting to
sanity-check a layout we have already derived, not to guess what an opcode means.

The inbound numbering is its own enum and does not track the outbound one: the reply to
outbound `0x00A1` is inbound `0x0032`, and the reply to outbound `0x0080` is inbound
`0x0010`.

## Method and its limits

A referenced string only counts as evidence when it is **specific** — appearing under at
most three distinct opcodes. Without that filter the table fills with noise: the GM command
handler `FUN_1418d1040` alone references hundreds of strings (`/kill`, `/summon`, ...) and
would smear them across every opcode it builds. Labels below are the client's own strings,
so treat them as strong hints about the *area* an opcode belongs to rather than as its
formal name. Some are genuinely ambiguous (`prepare`, `point`, `enter`).

"Leading fields" is the most common `u8`/`u32` prefix written by that opcode's builders —
a structural fingerprint, useful for recognising a packet by shape.


| Opcode | Label | Leading fields |
|---|---|---|
| `0x007C` | identifyVerficationFailed | `u32` |
| `0x0080` | GC:SelectWorld / accountno / nexonsn | `u32` |
| `0x0081` | useAllAP | `` |
| `0x00A0` | GC:SelectWorld / accountno / nexonsn | `u8` |
| `0x00B7` | maple_hf / maple_hf2 | `` |
| `0x00D4` | YoYoLogSet / mableDice | `u8` |
| `0x00D9` | Portal | `u8,u32,u32,u8` |
| `0x00DF` | Melee / User Force Atom / User Melee Target | `u8,u8,u32,u32,u8,u32,u32,u8` |
| `0x00E0` | Melee / Shoot / User Box2D | `u8,u8,u8,u32,u32,u8,u32,u32` |
| `0x00E1` | Magic / User Magic | `` |
| `0x00E2` | User Body | `u8,u8,u32,u32,u32,u32,u8,u8` |
| `0x00E3` | User Dot | `u8,u8,u32,u32,u32,u32,u8,u8` |
| `0x00E5` | DelayFever | `` |
| `0x00F5` | point | `u8` |
| `0x00F6` | InCoin / exit / outCoin | `u8` |
| `0x0116` | Meso / path / stateChangeItem | `u32,u32,u32,u32` |
| `0x013C` | Grenade Bomb / User Common 2 / User Flying | `u32,u32,u32,u32,u32` |
| `0x013E` | User Flying New | `u32,u32,u32,u32,u32` |
| `0x014A` | Portal | `u8,u8` |
| `0x014B` | Portal | `u8` |
| `0x0151` | count / default / gender | `u8,u32,u32,u8,u32,u32` |
| `0x016D` | repeat / scale | `u32,u32` |
| `0x017B` | streetName | `u8,u32` |
| `0x017F` | collect | `u32` |
| `0x0180` | collect / storeclose | `u8` |
| `0x0189` | mableDice / YoYoLogSet / abstract | `u32,u32` |
| `0x019C` | abstract / clear / fieldinfo | `u8,u32,u32,u32` |
| `0x01A3` | delete / rankingonoff / setranking | `u32,u32,u32,u32` |
| `0x01AF` | gauge | `` |
| `0x01BA` | abstract / clear / delay | `` |
| `0x01D1` | prepare | `u32,u32,u32` |
| `0x01E2` | atch2 / catch / failed | `u32` |
| `0x01ED` | enter / Count / Disabled | `u32,u32` |
| `0x0214` | Summon Attack / Summon Basic | `u32,u32,u32,u32,u8,u8,u8,u8` |
| `0x0227` | CryptCATAdminAcquireContext2 / IsWow64Process | `u32,u32,u8,u8,u8,u32,u32,u32` |
| `0x0238` | Canvas / alpha | `` |
| `0x0248` | gather / regist | `u32,u32` |
| `0x024D` | Canvas / alpha | `` |
| `0x025C` | DelayFever | `` |
| `0x025F` | CreateDropItemFailed | `u32,u32` |
| `0x0266` | TossedEffect | `` |
| `0x028D` | get target | `` |
| `0x02AF` | onoff | `u32` |
| `0x02B1` | Melee / Magic / User Shoot | `u32,u32` |
| `0x02C8` | enter | `` |
| `0x02D1` | prepare | `u32,u32` |
| `0x0313` | flyingbutterfly | `u32` |
| `0x0318` | prepare | `u32,u32` |
| `0x0343` | Invalid LogType / Tooltip / remove | `u32` |
| `0x0396` | point | `u32` |
| `0x03B2` | LimenWaterFallHit | `` |
| `0x03B4` | Thunder Attack Collision | `` |
| `0x03BE` | finish / gameover / myturn | `` |
| `0x03C2` | Piece / yutskill | `` |
| `0x03F5` | item | `u32,u32,u32` |
| `0x03F6` | item | `u32,u32,u32` |
| `0x0406` | enter | `u32` |
| `0x040F` | Caution | `` |
| `0x04FE` | YoYoLogSet / mableDice | `u32,u32,u32,u32,u32,u32,u32` |

## The interesting ones for login

`0x0080` and `0x00A0` both reference **`GC:SelectWorld`**, **`accountno`** and
**`nexonsn`**, and `0x0080`'s builder also pulls in `String/Consent.img/Consent`. Those are
the account / world-select packets — the ones the client will send once it reaches the
login and world-select screens. `0x007C` (`confirmDeleteCharacterPermanently`) confirms the
`0x0070`-`0x0080` band is the account and character-management area.

Full per-call-site detail, including the unfiltered strings, is in
`research/msexe-packet-fields.txt`.

## The login connection's startup sequence

Read from the tail of `FUN_1415d10e0` (the handshake handler), which is normal readable
code. `conn+0x48` selects the path, and it is **non-zero for the login connection** — the
same flag that selects AES over the byte-subtract cipher:

```c
FUN_1415d5b40(param_1);                 // 0x70   version report (u8 2, u32 100)
if (*(int *)(param_1 + 0x48) == 0) {    // game/channel connection - NOT our path
    ... builds 0x7D, then 0xB5 ...
} else {                                // login connection - ours
    FUN_1415d5c20(param_1);             // 0x71   environment report
    FUN_1415dde80();                    // 0x8F   \
    FUN_1415ddf60();                    // 0x90    > three integrity reports
    FUN_1415de040();                    // 0x91   /
    if (FUN_1415e2e70(param_1 + 0x150)) // conditional
        ... builds 0xA1 ...
}
*(undefined1 *)(param_1 + 0x145) = 1;
return local_21ac;                      // returns 1 - the handler does NOT block
```

So on our connection the client sends **`0x70`, `0x71`, `0x8F`, `0x90`, `0x91`** and
optionally `0xA1`, then the handshake handler *returns*. The hang is therefore not inside
the handshake at all — the client is back in its main loop waiting on the socket.

`0x8F`, `0x90` and `0x91` have byte-identical builder signatures
(`FUN_140a00790, u32, FUN_140a00790, FUN_1415e2f80, FUN_1406ede20`) — a u32 followed by a
binary blob, three times over. That shape says client-integrity or file-hash reporting, and
it is the most likely thing the server is expected to acknowledge before the client will
proceed to the login screen.

The `0x7D` path, which sends a 16-byte blob from `DAT_143262960`, is **game-connection
only** and never runs for us. Worth knowing so it is not chased.

## 0x8F / 0x90 / 0x91 are log uploads — and they explain the noisy captures

All three builders are identical bar their source, and the source is a **file path**
(`FUN_142e56b40`, `FUN_142e56bc0`, `FUN_142c4ad20` each return an MSVC `std::string`).
`FUN_1415ddd10(buf, path, 0x2000)` then:

```c
thunk_FUN_1408e8c70(local_68, param_2, 3, 0x80, 1, 0x80000000, 0, 0);  // CreateFile,
                                                                       // OPEN_EXISTING,
                                                                       // GENERIC_READ
uVar1 = FUN_1401bd210(local_68);                  // file size
if ((uVar1 != 0) && (uVar1 < param_3)) { ... }    // read it, if under 0x2000
local_a8 = (*DAT_143ad55c0)(param_2);             // delete the file
```

and the packet is `opcode + u32 length + that many bytes` (`FUN_140a00790` reads the length
stored at `*p - 8`; `FUN_1415e2f80` returns the pointer). Each is skipped entirely unless
`FUN_1415e3fd0` says the buffer is non-empty.

So the client **reads up to 8 KB from three files, uploads them, and deletes them**. These
are log uploads.

Two consequences:

1. **This is the confounder** that made two sweeps look like they had drawn replies. Log
   size varies with whatever the previous session left behind — the client's 4th packet
   (chain position 3, header `a=0xE35B` every run) was 10 bytes in one run and **2764** in
   another, and the whole opening burst ranged 294 to 3393 bytes. Repeatedly killing the
   client with `taskkill` is what grows them. It is self-limiting: the files are deleted
   once uploaded, which is why the 3393-byte run was followed by a 294-byte one.
2. **They need no acknowledgement.** An earlier hypothesis here — that `0x8F`-`0x91` were
   integrity reports the server must ack before the login screen appears — is wrong. They
   are fire-and-forget diagnostics.

**It waits on none of those.** The startup block ends with `0x00A1`, a `Data.wz` hash, and
the client then blocks in `recv` **on its UI thread** until the server answers it — see
`docs/transport.md`. An earlier version of this file guessed `0x70`/`0x71` were what it
waited on; they are fire-and-forget like the log uploads.

## The login stage, read from its OnPacket switch

`FUN_141b25f30(this, opcode, packet)` is the login stage's `OnPacket`, and its `switch` is
ordinary decompilable code. That is the inbound opcode map for this stage:

```
0x00, 0x0b-0x18, 0x23, 0x25-0x27, 0x29, 0x2b, 0x34-0x39, 0x45-0x48, 0x4a, 0x50, 0x5f, 0x5f4
```

with **`case 0x10` → `FUN_141b307b0`, the login result** — confirmed at runtime by watch
mode, which saw that function entered while dispatching `0x0010`. Its body is `u8 result`
then a `u16`-length string; **result `0` is success**, and `0x65`/`0x67` take a different
branch that merely re-sends `0x0080`.

The other cases in that switch are unlabelled but free to read the same way, and the same
trick should work for every other stage.

## The login stage has two variants, and we are in the second one

**Read this before decoding any login-stage handler.** Several handlers open with

```c
if (FUN_142c4a810(DAT_143ac1898) == 5) { <other handler>(this, packet); return; }
```

`FUN_142c4a810(obj)` is just `*(u32 *)(obj + 0x68)`, and `DAT_143ac1898` is the session
object. **That field is `5` in our client**, and it is not a guess: it is transmitted as the
first `u32` of outbound `0x0073`, where we captured `05 00 00 00`.

Mode 5 is what **`-NXLDEBUG`** selects — the launch mode this project uses for everything.
So every mode-5 branch is the live one, and the classic branch beside it is dead code for
us. Known forks so far:

| Site | mode != 5 | mode == 5 |
|---|---|---|
| `0x000B` handler `FUN_141b2fac0` | classic world list | **`FUN_141b31ff0`** |
| login flow `FUN_141b21ea0` | stage 2 | stage 3 |
| Login button `FUN_14112a570` | fade to stage 4 | `FUN_141b3ff10` |

The trap: decompile the *first* handler the switch names, decode it carefully, and you have
decoded a function this client never calls. Check for the `== 5` fork first, every time.

## 0x0010 — the login result. Confirmed accepted.

Sent as the answer to outbound `0x0080`. Body:

```
u8   result        0 = success
str  message       u16 length, then bytes
// result == 0 continues:
u8
8B   server time       -> _DAT_143ac3120, paired with a local tick baseline
u32  world id          -> compared against session+0x2258
u32  channel id        -> compared against session+0x2260
4B   world id          -> DAT_143ac2040
4B   world TYPE        -> DAT_143ac2044; 1=normal 2=reboot 3=burning 4=challenge, 0 skips
4B                     -> _DAT_143ac2160
u32
u8
     then FUN_14108d290 and FUN_14108bdf0 read further sub-records
u8, u8, u32, u8, u8, u8, u32
// result == 0x83 instead reads: two more u32
```

**Verified working.** `body = 00 00 00` plus 256 zero bytes made the client run the success
path to completion and advance its UI to character select. Compare `0x65`, which dropped the
connection in 0.0 s with no follow-up. Fixtures:
`research/fixtures/reply-0010-result0-advanced-to-charselect.log` (success) and
`reply-0010-correct-order-no-effect.log` (the `0x65` run).

Note the world/channel comparison: when the pair we send differs from `session+0x2258` /
`+0x2260`, the client calls `FUN_141b2c7c0(stage, world, channel, 0)`, which **searches the
world list at `stage+0x100`**. That list is populated only by `0x000B`. Send the login
result without a world list and the client arrives at character select with no world.

## 0x0000 and 0x0012 — the account name (the masked email)

Both handlers call `FUN_142cb8370(obj, str)`, which writes `DAT_143aa84a0 + 0x22f8` and then
calls `FUN_141128960(4)` to refresh the UI. `FUN_14112a720` renders that field into
`textAccount` whenever it is non-empty. **So the masked email on the login screen is
server-supplied**, not a launcher handoff — which is the opposite of what was assumed for
weeks.

| Inbound | Handler | Size |
|---|---|---|
| **`0x0000`** | `FUN_141b2dd00` | 4475 bytes |
| **`0x0012`** | `FUN_141b2ee90` | 1868 bytes |

Found by scanning `.text` for the disp32 `0x22f8` and filtering to the account-manager
range. **Not** by `xref.py` — a struct-offset store is not a `lea`.

### `0x0000` is a second login result, and a fuller one

Decoded 2026-08-17. The shape is unmistakable once read: `u8 result`, `str message`, and
then a gate on the result before any of the account fields are touched.

**The gate is `FUN_141b267c0(stage, result, 0, message)` — the same function that turns a
non-zero [`0x0010`](#0x0010--the-login-result-confirmed-accepted) into a named dialog.** So
`0x0000` and `0x0010` share their error vocabulary; `docs/client-messages.md` applies to
both. It returns "proceed" for result `0` **and** result `12`, and raises a notice for
everything else.

**No mode fork.** Unlike `0x000B`, `FUN_141b2dd00` does not branch on `session+0x68 == 5`,
so this is the live handler in mode 5 and mode 2 alike — one fewer thing to get wrong.

```text
u8   result           0 = success
str  message          the text the failure dialogs display
u8   verifyState      0 or 1 proceed; 2 or 3 raise "accountHasNotBeenVerified";
                      anything else raises loginTroubleAskSupport
u32                   read and discarded
--- fields below are read only once the gate passes ---
str  loginName        -> account+0x48
u64                   read and discarded
u32  accountId
u8
u32  flags            bit 21 calls FUN_140d2d4e0 — keep it clear
u32, u8, str, u32     the trailing u32 -> account+0x22b8
u8, u8, 8B, 8B        the two 8-byte fields go through a FILETIME conversion
u32, str, u32         the trailing u32 -> account+0x28e0
u8                    read and discarded
u8                    -> stage+0x1a4
u8                    -> stage+0xdc
8B                    -> account+0x2324
str  accountName      -> account+0x22f8, the string the login screen displays
```

Built by `net::opcode::account_info`, with a test that re-reads the body the way
`FUN_141b2dd00` does. Note the failure mode a length check would *not* catch: the readers
throw only on underrun, so a body that is misaligned but long enough runs to completion and
quietly hands the account-name setter the wrong bytes.

Three sub-records exist for non-zero results and are not built: `result == 2` reads
`u8 reason, 8 bytes, str` and produces the account-locked messages (reason `99` = five bad
attempts, `199` = ten), and `0x2d` / `0x8d` are one-off notices.

### `0x0012` is its shorter sibling

Same opening, same gate — but with `param_3 = 1`, so a failure also sets `stage+0xf0` — and
no `verifyState` byte or locked-account sub-record. Its middle differs field for field, and
it ends the same way, with the account name. Kept as the fallback if `0x0000` turns out to
be the wrong one of the two.

## Characters — see `docs/character.md`

Decoded 2026-08-17, statically. Summary:

| Direction | Opcode | Meaning |
|---|---|---|
| in | `0x0010` | the login result **carries the character list**, via `FUN_14108bdf0`: `u8 count`, then `count` records |
| out | `0x0081` | name check request — one string |
| in | `0x0014` | name check result — `str name`, `u8 result` |
| in | `0x0015` | create result — `u8 result`; on `0`, `u32` + one character record + `u8` |
| in | `0x0016` | delete result — `u32 characterId` |
| out | ? | **the create request, still unfound** |

The record itself is `FUN_1403094b0` -> `FUN_140302e30`. **CharSelect is a sub-screen of the
login stage**, not a stage of its own, so all of this arrives through `FUN_141b25f30`.

## 0x000B — the world list

One packet per world; a final packet whose first byte has the high bit set closes the list.
Read from `FUN_141b31ff0`, the **mode-5** handler (see the fork warning above).

```
u8    worldId
      // if worldId < 0 (high bit set, e.g. 0xFF) this is the TERMINATOR:
      //     u8  flag      -> stage+0x1a8
      //     u8  hasNotice -> non-zero shows a message
      //     ... and the entry fields below are NOT read
str   worldName
u8    flag
str   eventDescription
u8    flag
u8    channelCount
      repeat channelCount:
          str  channelName
          u32
          u8
          u8
          u8
          u8
u16   balloonCount
      repeat balloonCount:
          u16  x
          u16  y
          str  message
u32
u8    hasExtra        // non-zero: FUN_1408e4210 reads a further sub-record
```

The entry is appended by `FUN_141b44520(stage + 0x100, -1)` — the same list
`FUN_141b2c7c0` later searches by world id, which is what ties this packet to the login
result.

## 0x0011 — the migration packet ("GameIn")

**Identified statically 2026-08-19, no client run.** `case 0x11` of the login-stage switch
is `FUN_141b36f60` (4821 bytes), decompiled in `research/msexe-loginstage-cases.c`; the
helpers are in `research/msexe-migrate.c` and `research/msexe-migrate-helpers.c`.

### Why this is the migration packet

Three independent things say so, and one of them is a proper discriminator:

1. It builds a **`sockaddr_in`** — `sin_family = 2`, `sin_port = htons(...)`,
   `sin_addr = <a packet field>`, `sin_zero = 0` — and hands it to `FUN_142caa360`.
2. `htons` (`DAT_143262e48`) appears **exactly once** in every decompiled login-stage case
   handler, and that once is here. The other thirteen handlers in the same file build no
   address at all.
3. On the success path it calls `FUN_1429f14c0(PTR_u_GameIn_143a47c08, 100)` — the string
   **`GameIn`**.

It is the only login-stage case that can move the client to another server, so it is the
answer to `0x0078` (select character) by elimination as much as by shape.

### The packet readers, pinned exactly

Read off the primitives themselves rather than inferred from use:

| Function | Reads |
|---|---|
| `FUN_1406e8ae0` | **u8** (`pos += 1`) |
| `FUN_1406e8b80` | **u16** (`pos += 2`) |
| `FUN_1406e8c20` | **u32** (`pos += 4`) |
| `FUN_1406e8f10` | **u64** (`pos += 8`) |
| `FUN_1406e9050` | **string**: `u16 len`, then `len` bytes (`pos += len + 2`) |
| `FUN_1406e9170(p, dst, n)` | **n raw bytes** |
| `FUN_1406e8ee0` | **u8** - a bare `JMP`, invisible to a search for the target |
| `FUN_1406e8ef0` | **u16** - a bare `JMP 0x1406e8b80` |
| `FUN_1406e8f00` | **u32** - a bare `JMP 0x1406e8c20` |
| `FUN_142d23ef0` | **u32** - a bare `JMP`, and **6 MB from the other nine** |
| `FUN_1406e9b20` | **not a reader** — `mov eax,[rcx+0x24]; ret`, the current position |

**Ten, and this table listed seven until 2026-08-20.** The count has been wrong three times
and each fix came from enumerating rather than searching. `tools/reads.py` is the authority;
if it and this table disagree, this table is stale. A tail `jmp` into any of them **is** a
read - missing one shipped a chat packet four bytes short and killed the client.

`FUN_1406e9b20` decompiles to an empty body because Ghidra has no function there; the four
bytes are the answer.

### Head — decoded and straight-line

```text
u8    result          0 proceeds. Gated by FUN_141b267c0(stage, result, 0, &message),
                      the same gate as 0x0000 and 0x0010
str   message         shown in the dialog the gate raises on a non-zero result
u8                    read on every path, unused on the success path
```

Result codes `0x27`, `0x37`, `0x43`, `0x80`, `0x0c`, `0x22` and `0x8e` are intercepted
*before* the gate and each raises its own dialog, then returns. The stage records success
as `*(u32 *)(stage + 0xd8) = (result == 0)`.

### Payload — straight-line, only reached when the gate passes

```text
u32    ip             copied straight into sockaddr_in.sin_addr, so the four octets go on
                      the wire in order: 127.0.0.1 is 7f 00 00 01
u16    port           the client calls htons() on it, so write it little-endian as usual
u32    characterId    looked up with FUN_14108cae0 — a red-black-tree find over the map at
                      DAT_143ac9890. On a miss it returns a sentinel and the caller's
                      `*record == id` test fails, which skips the entire action block.
                      So this must be a character the login result already sent.
u32    a              -> FUN_1408414d0(a, b)  (DAT_143ac2040)
u32    b              -^  **non-zero makes the client load `Etc/SpecialServerInfo.img`**
u32    c              -> FUN_140842250(&c)    (_DAT_143ac2160)
u8     flags          bit 0 -> FUN_142cb9590, bit 1 -> FUN_142cb95a0
u32                   -> FUN_142cb95b0
u8                    -> FUN_142cb95c0
u8                    read, discarded
u32                   read, discarded
u8                    read, discarded
u8                    read, discarded
u8[8]
u32    key
u32    length         **must be <= the bytes remaining**, or FUN_1406e8460 throws
<length bytes>        obfuscated; see below
```

### The obfuscated tail

After `length`, the client copies the next `length` bytes out, transforms them, and copies
them **back over the same offset** without advancing the position — so reading simply
continues over the now-plain bytes. The transform is arithmetic on values that are
themselves in the packet, so there is no unknown key material and it is invertible:

```text
for each aligned u32 at byte offset 4n, while 4n + 4 <= length:
    w = (((key ^ w) + 0x369F144D + (key >> 7)) ^ 0xAAAABBBB) - (4n * key)

for each remaining byte at offset i:
    b = ((((key >> 1) ^ b) + 0x37 + (key >> 7)) ^ 0xAB) - (i * key)
```

with `key` the `u32` immediately before `length`. Both lines are truncating 32-bit and
8-bit arithmetic respectively.

**There is a second, nested pass** immediately after, structurally identical, with the roles
rotated: the previous `length` becomes the key, a length derived from the position delta
becomes the count, and the previous `key` (or the current position, when it is `0xFFFFFFFF`)
becomes the start offset.

### Settled on the wire, 2026-08-19

A migration packet built to this decode **moved a real client to a real channel**: the
client closed the login socket, connected to the advertised address, accepted the channel
greeting and began talking. The layout above is confirmed by behaviour.

The `u32` in the tail is stashed at `DAT_143ac80b0` and `FUN_1415d10e0` does write it into
outbound `0x007D` — but it is **not present in the `0x007D` the client actually sends**.
Do not build a handoff on it. The client identifies itself there by **character id**; see
`crates/world/src/session/`.

---

## The ELog covers the channel, but travels on the login socket

Worth knowing before hunting a channel-side fault. Captured 2026-08-19 in
`research/fixtures/leave-world-and-elog-login.log`:

```text
ELog|10|VERSION|100|DATETIME|2026/08/19 16:48:05|FID|30|LastUseName||State|3
|Time1|90478870|Time2|90477067|WID|0|Channel|0|NAME|TestCharD|JOB|0
|Socket|127.0.0.1:8485|0|184|AccountId|0|
```

`FID|30` is the map the character was standing on and `127.0.0.1:8485` is the channel - yet
this arrived as `0x008F` on the **login** connection. So the ELog does report channel-side
problems; it just does not travel on the channel. **Keep `login.log` in view when chasing a
fault in the world**, and note the client keeps the login connection open for the whole
session, so it is there to receive it.

The trailing `|0|184|AccountId|0|` follows the same shape as the `ELog|2` capture's
`|0|121|throw CTerminateException|...`, so `AccountId` reads as the thing being reported and
`0` as its value. **[I]** - not chased.

## The channel connection, 2026-08-19

Everything above is the **login** connection. These are the channel's, and they are on a
different enum - numeric proximity between the two means nothing.

**Almost all of them named themselves.** The owner used a feature, the capture showed what went
out, and the id and template values in the body matched data the *server* had assigned. That
has now identified five requests and cost no static analysis at all. It is the cheapest
instrument this project has.

### Inbound - what the server sends

| opcode | name | body | status |
|---|---|---|---|
| `0x01A0` | `SetField` | 33-byte head, three `u32`s, then the character record - **235 bytes undressed, 759 wearing four items with stats** | **works, and the character is dressed on screen.** The equipped list was confirmed 2026-08-19. What is *not* working is the item **tooltips**: both stat masks read back as zero in the Equipment window while the avatar is correct, so something in that window renders a different object. `research/naked-character.md`, `research/equip-stats.md` §11 |
| `0x044F` | `NpcEnterField` | fixed **64 bytes** | **works** - NPCs on screen. Two of its fields being zero (`enabled`, `alpha`) made every NPC invisible while the layout was perfect. `research/npc-spawn.md` |
| `0x055B` | `ScriptMessage` | 14-byte head, then per message type; type 0 `Say` is 26 bytes plus the text | **built 2026-08-19, unconfirmed on screen** - answers `0x0151`. Never send one with or just before a `SetField`: field entry runs `FUN_142caa4e0`, which resets the script manager and tears the dialog down silently. `research/npc-dialogue.md` |
| `0x0138` | `UserAvatarModified` | `u32` character id, then the compact avatar look | **dead code in the client, and no longer sent.** The apply is guarded by a call to `0x1407f5ce0`, which is three bytes - `33 c0 c3`, `xor eax,eax; ret` - then `TEST/JZ`, so the branch is always taken. **Corrected 2026-08-19:** this row used to say "the handler reaches its apply but the apply's loop never runs. Measured." The measurement (a watch that never fired) was right; the *explanation* was wrong, and the real one needs no run. `research/naked-character.md` §5.1 |
| `0x03C6` | mob enter field | `u8, u32 objectId, u8, u32 templateId, u8`, a 20-byte block, then a **variable-length** movement path | **not built** - `research/mob-spawn.md` |

### Outbound - what the client sends

**Added or corrected 2026-08-19.** Each of these came from the client naming its own request
in a capture, which has now identified seven and cost no static search at all.

| opcode | what | body | answered? |
|---|---|---|---|
| `0x0453` | **NpcChat**, inbound - the idle-chatter balloon | `u32 objectId, i8 nAction, i8 nChatIdx, u32`. 10 bytes. `nAction = -1` talks without changing animation and indexes the WZ `info/speak` group | **sent** - the server's only unsolicited packet. The client holds the text and the 5-second display; only the index goes on the wire. Ordering is ours, cadence is the client's own `rand() % 6000 + 3000` |
| `0x00BB` | **ChatNotice**, inbound - a line in the chat window | `u8 force, str text`. **Always send `force = 1`**: with `0` only the first line after each field entry appears | **sent** - `!map` uses it to say why it refused |
| `0x0106` | **LogOutResult**, inbound | one **non-empty** string; an empty one is a no-op | **sent** - and answering it is not optional: see `0x01BE` |
| `0x0420`-`0x0426` | **the client's own world-state dump**, six packets arriving together once per session | `0x0421` is 1115 bytes with the character id, the name and **our four item ids in equipped-slot order**; `0x0420` carries the NPC object ids we assigned; `0x0426` is 20 bytes starting `ffffffffffffffff` | **no**, and undecoded. A free read-back instrument: it says what the client *thinks* it has |
| `0x00F3` | **the client's answer to a script message** | `u32 handle, u32 echo, u8, u16-length string, u8`. **The string is the text the server sent**, echoed back byte for byte - confirmed at two different lengths in one capture (16 bytes for Robin's `d0`, 162 for quest 1000's opening) | **no**, and this is what stops a quest conversation dead: pressing Accept sends this and nothing comes back |
| `0x01BE` | **log out**, empty body | zero bytes, sent the instant the owner clicked Log Out | **yes**, with `0x0106` - and it is not optional. Its builder sets `world->[0x33f4] = 1`, and the `SetField` handler returns early while that byte is set, so **an unanswered Log Out makes every later `SetField` vanish in silence**. Any observation made after a Log Out click in the same session is invalid |
| `0x0107` | inventory move / unequip | `u32 tick, u8 invType, i16 srcSlot, i16 dstSlot, i16 count`. **Negative slots mean equipped**, read not assumed | never reaches the wire - the client drops it at one of six gates inside its own builder. `research/npc-click.md` §4 |
| `0x00D2` | change channel | `u8 targetChannel` (**0-based**), a `u32`, and a shared 14-byte preamble. Field *set* is read; field *order* is derived and unsettled | **no**. Reachable only once a channel row is selectable |



| opcode | what | body | answered? |
|---|---|---|---|
| `0x007D` | migration hello | character id at offset 8 | yes |
| `0x00D1` | transfer field (portal) | fully decoded, `research/transfer-field-request.md` | yes |
| `0x00DC` | **field entered** | empty. **Once per `SetField`, every time** - this is the per-field marker | yes, with NPCs |
| `0x0238` / `0x024D` | entered the world | empty. **First field entry only**, never again - not a per-field marker | no |
| `0x0151` | **quest request** | `u8 action, u32 questId, u32 npcTemplateId, [i16 x, i16 y], [u32 selection]`. Builder `FUN_141f0e4c0`. **Not an "NPC click", and the first `u32` is a quest id, not an object id** - see the retraction below | **yes, since 2026-08-19** - answered with a `0x055B` Say, so the NPC speaks. That is text on screen and nothing more: no quest-result packet has been found, so **no quest state advances** |
| `0x00E7` | **chat** | `u32`, `u16`-length string, `u8` | no |
| `0x0082` | **leave world** - BOTH "Choose another world" and "Back" on the character screen send this, empty body | | **yes**, already |
| `0x0182` | **party create** | 68 bytes carrying a length-prefixed party name | no |
| `0x00D9` | movement | every ~510 ms, coordinate-shaped | no |
| `0x0076` | **select world** | 171 bytes: local IP, then CPU, OS, memory, timezone, country, locale as length-prefixed strings. **Login connection** | **yes** - unanswered it hangs the client on "Connecting to server..." |
| `0x00F2` | **NPC click, the no-quest path** | `u32 npcObjectId, i16 charX, i16 charY, u32` - 12 bytes. Field 1 is `[npc+0x190]`, the object id **we** assigned; fields 2-3 are the **character's** position, not the NPC's, and the first reading of this had that wrong | **yes** - a `0x055B` Say, spoken by the template the object id maps back to. `research/npc-click.md` |
| `0x013D`, `0x00B8`, `0x02EB`, `0x01ED`, `0x0408`, `0x0184`, `0x0194`, `0x01A5`, `0x02DE`, `0x00ED`, `0x02B2` | undecoded | | no |

**None of the unanswered ones has ever caused a freeze**, so none is a blocking request -
which is why the "always answer" rule has not bitten on the channel the way it did on login.

### Routing

`research/msexe-gamestage-dispatch.md` has the full table. The one worth knowing here is
**`FUN_141820080`**, `CField::OnPacket`, covering `0x1a4..0x5ab` and range-chaining to about
15 pool sub-dispatchers - the NPC pool at `0x44F..0x468` and the mob pool at `0x3C6..0x44E`
among them.
