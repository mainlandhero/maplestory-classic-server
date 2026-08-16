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
