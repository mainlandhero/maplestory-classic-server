# Opcodes

## Where these come from

The client's **outbound** opcodes are recoverable from the binary. `FUN_1406ed520(buf, op)`
begins a packet, so every call site names one opcode, and the calls that follow append its
fields. `tools/ghidra_scripts/DumpPacketFields.java` walks all 1894 call sites and records
the opcode, the field sequence, and every string the builder references;
`tools/label_opcodes.py` turns that into the table below.

The client's **inbound** opcodes cannot be recovered this way at all. The dispatcher
`FUN_1415d60e0` tail-jumps into the `.themida` section, which has no file bytes — see
`docs/transport.md`. Inbound opcodes have to come from the client at runtime, which is what
`handshake_probe.py --reply sweep` is for.

## On published opcode lists

Public MapleStory opcode tables are for other versions and should not be trusted here.
This client's outbound space **starts at `0x0070`** — only `0x0000` sits below it — whereas
classic v62/v83 numbering puts the login packet at `0x01`. The numbers will not transfer.

What *does* transfer is structure: field layouts for a given operation are far more stable
across versions than the opcode numbers. So published lists are worth consulting to
sanity-check a layout we have already derived, not to guess what an opcode means.

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
