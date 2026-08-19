# v214 character-record shape — CANDIDATE SKELETON ONLY

> **EVERY LINE IN THIS FILE IS A v214 CANDIDATE, NOT AN mscw FACT.**
>
> Source: `C:\Users\user\Desktop\ModernMapleSource\v214 src` (`net.swordie.ms`, a Swordie-family
> Java server for MapleStory v214). mscw is a *different version of the game*. Nothing here has
> been observed on the mscw client or wire. On this project the same "align against a modern
> reference" method was scored blind against the mscw login opcode range — a control whose
> meanings were already independently decoded — and it scored **1 of 8**, and the one hit was the
> block's first entry, which any alignment gets for free.
>
> Use this for **naming fields and predicting structure**. Do not use it to settle anything.
> The value here is the *shape*: how many blocks, in what order, which are counted, which are
> fixed arrays. That shape is what should be checked against the mscw disassembly.
>
> Field widths shown are the widths **the v214 encoder writes**, read off the source. They are not
> guesses, and they are also not claims about mscw. Where a width could not be determined from the
> source it is left as `?` rather than invented.

## Provenance

| What | File | Line |
|---|---|---|
| SetField packet | `src/main/java/net/swordie/ms/connection/packet/Stage.java` | `setField(...)` |
| The character record | `src/main/java/net/swordie/ms/client/character/Char.java` | 1992 `encode(OutPacket, DBChar)` |
| Section flags enum | `src/main/java/net/swordie/ms/enums/DBChar.java` | — |
| Core stat struct | `src/main/java/net/swordie/ms/client/character/CharacterStat.java` | `encode` |
| Equip inventories | `Char.java` | 2778 `encodeEquips` |
| Damage skins | `Char.java` | 7087 `encodeDamageSkins` |
| VMatrix | `Char.java` | 2714 `encodeMatrixSkills` |
| Hexa | `Char.java` | 9690 / 9847 |
| Boss reward | `Char.java` | 2731 |
| Commerce | `Char.java` | 2749 |

Primitive widths in `connection/OutPacket.java`: `encodeByte`=1, `encodeShort`=2, `encodeInt`=4,
`encodeLong`=8, `encodeFT`=8 (`util/FileTime.encode` writes one long), `encodeString(s)`=2+len
(length-prefixed), `encodeString(s, n)`=exactly n bytes (null-padded, **no length prefix**).

## Marker legend

| Marker | Meaning |
|---|---|
| `[COUNT:w]` | A count field of width `w` immediately prefixing a repeated block. Zero is legal; block collapses to just the count field. |
| `[CAP:w]` | An integer that *looks* like a count but is a constant capacity — it is **not** the length of what follows. Trap. |
| `[FIXED:n]` | Fixed-size, no count. Cannot collapse. `n` = bytes. |
| `[SENTINEL]` | List terminated by a sentinel value (`short 0`), not count-prefixed. Collapses to just the sentinel. |
| `[GATE]` | A boolean byte gating an optional block. Collapses to 1 byte when false. |
| `[JOB]` | Present only for one job family in v214. Absent entirely otherwise — not zero-filled. |

---

## Part 0 — where the record sits inside SetField

`Stage.setField` writes a header, then **if `characterData` is set**: three random ints (damage-calc
seeds), then `chr.encode(outPacket, DBChar.All)` — that call is the character record. Everything
after it in `setField` is trailer, not part of the record.

```
FT  currentTime                       [FIXED:8]
byte unk                              [FIXED:1]
int  channelId - 1                    [FIXED:4]
byte dev                              [FIXED:1]
int  oldDriverID                      [FIXED:4]
byte characterData ? 1 : 2            [FIXED:1]    <-- the fork
int  characterData ? 0 : fieldType    [FIXED:4]
int  field width                      [FIXED:4]
int  field height                     [FIXED:4]
byte characterData                    [FIXED:1]    <-- encoded a second time
short notifierCheck                   [COUNT:2]    -> if >0: string + notifierCheck x string
  if characterData:
    int s1, int s2, int s3            [FIXED:12]   <-- damage-calc seeds
    >>> CHARACTER RECORD (Part 1) <<<
    byte bool1(=1)                    [GATE]       -> byte bool2 [GATE->int], FT, byte, FT, FT, int(-1)
  else:
    byte usingBuffProtector, int fieldId, byte portal, int hp, int 0,
    byte hasPosition [GATE] -> int x, int y
```

**Note on the `characterData ? 1 : 2` byte:** v214 also documents `4` for a bossing variant in a
source comment. Unverified even within v214.

---

## Part 1 — THE CHARACTER RECORD (`Char.encode`, mask = `DBChar.All`)

**Critical structural note.** `DBChar` is a 64-bit section-flag enum, but **the mask value is never
written to the packet**. Verified by grepping the whole tree for `encodeLong(mask` / `mask.get()`
reaching an encoder — no hit in `Char.encode`. Instead the record opens with a **100-byte array of
`1`s**, which is v214's section-presence signal. `setField` always passes `DBChar.All`, so in
practice **every `isInMask` branch below is taken**. If mscw's decoder truly has no presence mask,
that 100-byte array is the closest v214 analogue and is the first thing to look for — or its
absence is the first structural divergence.

Blocks are numbered `C##`. The `[mask]` tag records which `DBChar` flag gates the block in v214;
with `All` they are all live.

| # | Block | Shape |
|---|---|---|
| C01 | section flag array | **`[FIXED:100]`** — 100 × `byte 1`. No count. |
| C02 | combat orders | `byte` `[FIXED:1]` |
| C03 | pet skill cooltimes | **`[FIXED:12]`** — `MAX_PET_AMOUNT`=3 × `int`, padded with `-2`. No count. |
| C04 | unknown scalars | `byte`, `byte`, `int`, `byte` `[FIXED:7]` |
| C05 | **CharacterStat** `[Character]` | `[FIXED:~254]` — see Part 2. The name is here. |
| C06 | friend record count | `byte` — **`[COUNT:1]` with no block after it.** A bare count. Trap: looks like a list header, isn't. |
| C07 | blessing of fairy | `byte` `[GATE]` → `string` (2+len) |
| C08 | blessing of empress | `byte` `[GATE]` → `string` (2+len) |
| C09 | ultimate explorer (deprecated) | `byte false` `[FIXED:1]` |
| C10 | unknown | `short 0`, `FT MIN` `[FIXED:10]` |
| C11 | damage-skin capacity | `int 35` — **`[CAP:4]`**, `GameConstants.DAMAGE_SKIN_MAX_SIZE`. **Not** the count of C12. |
| C12 | damage skins | `[COUNT:4]` + n × (`int` skinID, `int` itemID, `FT`) = 16 B/entry; **then `[FIXED:48]`** — three unconditional triples (active / premium / new), each (`int`,`int`,`FT`). |
| C13 | money `[Money]` | `long` `[FIXED:8]` |
| C14 | exp-consume items `[ItemSlotConsume\|ExpConsumeItem]` | `[COUNT:4]` + n × (`int`,`int`,`int`,`long`) = 20 B/entry. **v214 sends 0.** |
| C15 | shop buy limit `[ItemSlotConsume\|ShopBuyLimit]` | `[COUNT:4]` + n × (5×`int`, 2×`long`) = 36 B/entry. **v214 hardcodes 0.** |
| C16 | inventory sizes `[InventorySize]` | **`[FIXED:24]`** — 6 × `int` (equip, consume, install, etc, cash, decoration). No count. |
| C17 | unknown | 3 × `byte 0` `[FIXED:3]` |
| C18 | **equip inventories** `[ItemSlotEquip]` | `[SENTINEL]` × 24 + 2 `byte`. **Minimum 50 B.** See Part 3. |
| C19 | consume inventory `[ItemSlotConsume]` | `[SENTINEL]` — n × (`short` bagIndex, `Item`) then `short 0` |
| C20 | install inventory `[ItemSlotInstall]` | `[SENTINEL]` |
| C21 | etc inventory `[ItemSlotEtc]` | `[SENTINEL]` |
| C22 | cash inventory `[ItemSlotCash]` | `[SENTINEL]` |
| C23 | bag data — consume | `[COUNT:4]` (v214 hardcodes 0, no loop body written) |
| C24 | bag data — install | `[COUNT:4]` (hardcoded 0) |
| C25 | bag data — etc | `[COUNT:4]` (hardcoded 0) |
| C26–C30 | special bag data ×5 | 5 × `[COUNT:4]` (all hardcoded 0) |
| C31 | boss reward `[ItemSlotEtc]` | `[COUNT:4]` + n × (`long`,`int`,`int`,`long`,`long`,`FT`) = 32 B/entry |
| C32 | core aura `[CoreAura]` | `[COUNT:4]` + n × (`int`, `FT`). **v214 sends 0.** |
| C33 | unknown `[Unk40000000]` | `[COUNT:4]` + n × (`long`, `FT`). **v214 sends 0.** |
| C34 | **skill record** `[SkillRecord]` | Compound, see below. |
| C34a | — flag | `byte 1` `[FIXED:1]` |
| C34b | — skills | `[COUNT:2]` + n × (`int` id, `int` level, `FT`) **plus a conditional `int` masterLevel when `isSkillNeedMasterLevel(id)`** — ⚠️ **variable-width entries**, per-entry width depends on the skill ID. |
| C34c | — hyper-stat preset index | `byte` `[FIXED:1]` |
| C34d | — hyper stats | **3 fixed iterations**, each `[COUNT:4]` + n × (`int`,`int`,`int`). Loop trip count is fixed at 3; the counts inside are collapsible. |
| C34e | — own link skill | `int` charId, `int` originalLinkSkillID, `int` level `[FIXED:12]` |
| C34f | — flag | `byte 1` `[FIXED:1]` |
| C34g | — account link skills | `[COUNT:2]` + n × (`int`,`int`,`int`) |
| C34h | — stacking link skills | `[COUNT:2]` + n × (`int`, `short`) |
| C34i | — flag | `byte 1` `[FIXED:1]` |
| C34j | — linked skill preset | `[COUNT:4]` + n × `int` |
| C34k | — trailer | `int 0`, `int 0` `[FIXED:8]` |
| C35 | skill cooltimes `[SkillCooltime]` | `[COUNT:2]` + n × (`int` id, `int` seconds) |
| C36 | skill alarms `[SkillAlarmInfo]` | **`[FIXED:54]`** — three parallel arrays of `MAX_INDEX`=6: 6×`int` skillId, 6×`byte` enable, 6×`int` key. No count. |
| C37 | quest records `[QuestRecord]` | `byte removeAllOldEntries(=1)` `[GATE-inverse]`; `[COUNT:2]` + n × (`int` key, `string` value); then `[COUNT:2]`(=0); then *(only if removeAll==false)* `[COUNT:2]` + n×`int`; then `[COUNT:2]` + n × (`string`,`string`) |
| C38 | quests complete `[QuestComplete]` | `byte removeAllOldEntries(=1)`; `[COUNT:2]` + n × (`int` key, `FT`); *(only if removeAll==false)* `[COUNT:2]` + n×`int` |
| C39 | minigame records `[MinigameRecord]` | `[COUNT:2]` + n × MiniGameRecord. **v214 sends 0.** |
| C40 | couple/friend/marriage `[CoupleRecord]` | **3 × `[COUNT:2]`, all hardcoded 0.** Collapses to 6 B. |
| C41 | **map transfer** `[MapTransfer]` | **`[FIXED:164]`** — four fixed arrays with no counts: 5 + 10 + 13 + 13 = 41 × `int` (teleport rock, VIP, premium VIP, hyper). |
| C42 | **familiar codex** `[FamiliarCodex]` | **`[FIXED:144]`** — `byte[44]`, then 5×3 `int` (60 B), then 5 × `FAMILIAR_BADGE_SLOTS`(8) `byte` (40 B). No counts anywhere. |
| C43 | familiars `[Familiar]` | `byte` `[GATE]` + `[COUNT:4]` |
| C44 | familiar codex list `[FamiliarCodex]` | `byte` `[GATE]` + `[COUNT:4]` |
| C45 | quest record ex `[QuestRecordEx]` | `[COUNT:2]` + n × (`int`, `string`) |
| C46 | new year cards `[NewYearCard]` | `[COUNT:2]` + n × (`int`, `short`) |
| C47 | bNxRecordAccessAuth | `byte 1` `[FIXED:1]` — gates C48 in v214 |
| C48 | unknown `[Unk10000000000]` | `[COUNT:4]` + n × (`int`, `string`) |
| C49 | unknown `[Unk100000000000]` | `[COUNT:4]` + n × (`int`, `int`) |
| C50 | wild hunter info `[WildHunterInfo]` | **`[JOB]`** Wild Hunter only. `byte` + **`[FIXED:20]`** 5 × `int` captured mob. **Emits nothing at all for other jobs** — not a zero-length placeholder. |
| C51 | zero info `[ZeroInfo]` | **`[JOB]`** Zero only. `short` bitmask + masked fields. Nothing for other jobs. |
| C52–C54 | shop buy limit ×3 `[ShopBuyLimit]` | 3 × `short 0` — three `[COUNT:2]` with no loop body written. |
| C55 | unknown | `[COUNT:4]` (comment says the body would be 4×`int`) |
| C56 | unknown (new in 263) | `int 0` `[FIXED:4]` |
| C57 | **stolen skills** `[StolenSkills]` | **`[FIXED:64]`** — 16 × `int`, **always emitted**, zero-filled for non-Phantom. Not a `[JOB]` block. |
| C58 | **chosen skills** `[ChosenSkills]` | **`[FIXED:20]`** — 5 × `int`, always emitted, zero-filled for non-Phantom. |
| C59 | character potential `[CharacterPotential]` | **3 fixed iterations** (presets), each `[COUNT:2]` + n × (`byte`,`int`,`byte`,`byte`) = 7 B/entry. |
| C60 | soul collection `[SoulCollection]` | `[COUNT:2]` + n × (`int`,`int`) |
| C61 | honor `[Character]` | `int 1` (deprecated level), `int` honorExp `[FIXED:8]` |
| C62 | unknown `[Unk200000000]` | `byte` `[GATE]` → `[COUNT:2]` + n × (`short` category, `[COUNT:2]` + m × (`int`,`int`)) — **nested count**. |
| C63 | return effect `[ReturnEffectInfo]` | `byte` `[GATE]` → Equip + `int` |
| C64 | dress-up `[DressUpInfo]` | **`[FIXED:22]`** — `int` face, `int` hair, `int` clothe, `byte` skin, `int`, `byte`, `int`. Always emitted (default-constructed for non-Angelic-Buster). |
| C65 | active damage skin `[ActiveDamageSkin]` | `int`, `int`, `FT`, `string`, `int` `[FIXED:22 + strlen]` |
| C66 | memorial cube `[MemorialCubeInfo]` | `byte false` `[GATE]` |
| C67 | memorial flame `[MemorialFlameInfo]` | `byte false` `[GATE]` |
| C68 | like point `[LikePoint]` | **`[FIXED:16]`** — `int`, `FT`, `int` |
| C69 | runner game record `[RunnerGameRecord]` | **`[FIXED:28]`** — 4×`int`, `long`, `int` (all `-1`) |
| C70 | unknown `[Unk8000000000000]` | `[COUNT:4]` + n × (`int`, 3×`byte`); then trailer `int`, `long`, `byte`, `byte` `[FIXED:14]` |
| C71 | world-share quests | **unconditional** (no mask) `[COUNT:2]` + n × (`int`, `string`) |
| C72 | monster collection `[MonsterCollection]` | `[COUNT:2]` + n × (`int`, `string`) |
| C73 | unknown | `byte 1`, `short 0` `[FIXED:3]` |
| C74 | text equip info | `[COUNT:4]` + n × (`int`, `string`) |
| C75 | unknown `[Unk10000000000000]` | `[COUNT:2]` + n × (`int`,`int`) |
| C76 | **V-Matrix** `[VMatrix]` | `[COUNT:4]` + n × MatrixRecord; **then `[CAP:4]`** (`MAX_NODE_SLOTS`=30) **followed by `[FIXED:390]`** — 30 × (`int` nodeID, `int` slot, `int` level, `byte` unlocked) = 13 B × 30. The leading `int 30` is a constant, not a count. **Largest single forced block in the record.** |
| C77 | hexa skills | **unconditional** `[COUNT:4]` + n × HexaSkill; then `byte 0`, `int 0` `[FIXED:5]` |
| C78 | hexa stats | **unconditional** `[COUNT:4]` + n × (~9 `int` + save data) |
| C79 | hexa skills 2 | `int 0`, `byte 0`, `int 0` `[FIXED:9]` |
| C80 | union artifacts | **unconditional** `[COUNT:4]` + n × (`int`, `int`, UnionArtifact) |
| C81 | unknown | `[COUNT:4]` + n × `int` |
| C82 | achievements `[Achievement]` | `int` accId, `int` charId, `int 0`, `int -1`, `int 0` `[FIXED:20]`; then `[COUNT:4]` + n × AchievementRank |
| C83 | boss reward (**second time**) `[ItemSlotEtc]` | `[COUNT:4]` + n × 32 B — same encoder as C31, emitted twice in the same record. |
| C84 | **unknown 3×42 flag matrix** | **`[FIXED:132]`** — 3 iterations of { `byte`, `byte`, 42 × (`byte` gate → 6 B if set) }. Trip counts are hardcoded 3 and 42, no counts on the wire. Minimum 3 × 44 = 132 B. |
| C85 | emoticons `[Familiar]` | `[COUNT:4]` + n × `int`; then `[COUNT:4]` + n × `short` (tabs); then `short 0` |
| C86 | unknown | `int 0` `[FIXED:4]` |
| C87 | commerce record `[Unk0x4000000000000000]` | `byte` `[GATE]`; `[COUNT:2]` + n × (`byte`,`int`,`int`); `[COUNT:2]` + n × (`int`,`int`,`FT`) |
| C88 | new year card flag `[NewYearCard]` | `byte 0` `[FIXED:1]` |
| C89 | new year card lists `[NewYearCard]` | `[COUNT:4]` + n × (`short`,`short`); `[COUNT:4]` + n × (`short`,`int`) |
| C90 | unknown `[Unk20000000000]` | `[COUNT:2]` + n × (`short`,`short`) |
| C91 | **red leaf info** `[RedLeafInfo]` | **`[FIXED:48]`** — `int` accId, `int` charId, `int 4`, `int 0`, `byte[32]`. Last thing in the record. |

---

## Part 2 — `CharacterStat.encode` (block C05) expanded

This is the block that carries **type, id, and name**. Fixed-size except for two job-conditional
fields and the trailing burning strings.

```
int    characterId                 4
int    characterIdForLog           4
int    worldIdForLog               4
string name, FIXED 13 BYTES       13   <-- encodeString(s,13): null-padded, NO length prefix
byte   gender                      1
byte   skin                        1
int    0                           4
int    face                        4
int    hair                        4    (mixed-hair-colour math applied server-side)
int    level                       4
short  job                         2
short  str / dex / int / luk       8
int    hp / maxHp / mp / maxMp    16
short  ap                          2
short  sp                          2    <-- OR ExtendSP [COUNT:1] + n x (byte,int) for extended-SP jobs
long   exp                         8
int    pop                         4
int    wp (Zero weapon point)      4
int    gachExp                     4
int    posMap (field id)           4
byte   portal                      1
int    0                           4
short  subJob                      2
int    defFaceAcc                  4   <-- [JOB] Demon/Xenon/BeastTamer/Ark/HoYoung ONLY; absent otherwise
byte   0                           1
FT     (MIN_TIME)                  8
int    charisma/insight/will/craft/sense/charm exp   24
--- NonCombatStatDayLimit (inline, [FIXED:37]) ---
int    charisma/insight/will/craft/sense/charm      24
byte   charmByCashPR                1
FT     lastUpdateCharmByCashPR      8
int    todayYYMMDD                  4
--- end ---
int    pvpExp                      4
byte   pvpGrade                    1
int    pvpPoint                    4
byte   0                           1
byte   pvpModeType                 1
int    eventPoint                  4
int    0                           4
int    0                           4
--- encodeBurning (inline, [FIXED:29 + 4 strings]) ---
FT     startDate                   8
FT     endDate                     8
int    minLevel / maxLevel / type 12
byte   burningType                 1
int    0                           4
string ""                          2+
int    0                           4
string "" / "" / ""                6+
int    0 / 0                       8
--- end ---
int    0                           4
```

**≈254 bytes** for a plain job with empty burning strings (arithmetic over the v214 encoder, not an
mscw claim). Two job-conditional widenings: extended-SP jobs replace `short sp` with a
count-prefixed list, and five job families insert an extra `int defFaceAcc`.

**AvatarLook is NOT in the character record.** `AvatarData.encode` (CharacterStat + `encodeUnk` +
AvatarLook [+ Zero look]) exists and is used for the *character-select list*, but `Char.encode`
calls only `getAvatarData().getCharacterStat().encode(...)`. In v214 the in-field look is
reconstructed from the equipped-inventory block (C18), not from a look struct. If mscw's record
carries an AvatarLook inline, that is a divergence from v214, not a match to it.

---

## Part 3 — `encodeEquips` (block C18) expanded

**No counts anywhere.** Every sub-list is a run of `(short bagIndex, Item…)` terminated by
`short 0`. There are **24 sentinel terminators and 2 gate bytes**, so an entirely empty
equipment set still costs **50 bytes**.

Order: `byte onlyEquipped`; equipped(BPBase..BPEnd, non-cash) → sentinel; equip-inventory
(potentialed only) → sentinel; then Evan, Mech, Bits, MBP, Arc, AUS, Haku slot ranges → sentinel
each; three bare sentinels (v263 slots 8–10); Totem → sentinel; bare sentinel (12); two bare
sentinels (item ranges 20001–20048, 20049–20051); `byte onlyEquipped` again; cash-equipped
(bagIndex − 100) → sentinel; decoration inventory → sentinel; AP, DU, Zero ranges → sentinel each;
three bare sentinels (16–18).

Per-item payload (`Item.encode` / `Equip.encode`) is itself branchy — cash items carry an extra
`long`, symbols carry an extra `short/int/short`, androids carry an android block, and the trailing
enhancement blocks are gated by computed masks. **This is not a fixed-width record.**

---

## Part 4 — the answers you asked for

### How many top-level blocks

**91 top-level blocks** (C01–C91) in `Char.encode` under `DBChar.All`, plus the `Stage.setField`
header (10 fields + 1 count) before it and a trailer after it.

Because `setField` always passes `DBChar.All`, **the mask never suppresses anything** — every
block above is on the wire in v214.

### Count-prefixed and therefore collapsible

**64 count-prefixed points**, spread over the blocks below. Every one takes a zero and collapses to
just the count field; v214 itself hardcodes zero at many of them (C15, C23–C30, C32, C33, C39, C40,
C52–C55, C87, C89 are written as literal `0` with a dead loop body, so a zero there is not
speculative — it is what the reference server sends).

Widths: `[COUNT:4]` at C12, C14, C15, C23, C24, C25, C26–C30, C31, C32, C33, C34d(×3), C34j, C43,
C44, C48, C49, C55, C70, C74, C76a, C77, C78, C80, C81, C82, C83, C85(×2), C89(×2).
`[COUNT:2]` at C34b, C34g, C34h, C35, C37(×3), C38(×1), C39, C40(×3), C45, C46, C52–C54, C59(×3),
C60, C62, C71, C72, C75, C87(×2), C90. That is 36 four-byte + 28 two-byte = **64**.

Three more counts exist in the encoder but are **not reached** on v214's own settings: one each in
C37 and C38 behind `if (!removeAllOldEntries)` (the flag is hardcoded `true`), and the nested inner
count in C62 (reached only when its outer count is non-zero). `[COUNT:1]` appears at C06 (bare, no
block) and inside ExtendSP.

Three traps in this list:
- **C06** is a count with **no repeated block after it**. Emitting a block there would desync.
- **C11** (`int 35`) and **C76b** (`int 30`) are **`[CAP]`, not counts** — constants that precede a
  block whose real length is unrelated (C12) or fixed (C76b). Reading them as counts desyncs.
- **C34b** entries are **variable width** — a conditional extra `int` per skill, decided by skill ID.

### Fixed-size blocks we are forced to emit in full

These have no count and cannot be collapsed. Sorted by v214 size:

| Bytes | Block | What it is |
|---|---|---|
| **390** | C76b | V-Matrix node slots — 30 × (`int`,`int`,`int`,`byte`), behind a constant `int 30` |
| **~254** | C05 | CharacterStat (name lives here, as a 13-byte null-padded field) |
| **164** | C41 | Map-transfer rocks — 5 + 10 + 13 + 13 ints |
| **144** | C42 | Familiar codex — `byte[44]` + 15 ints + 40 bytes |
| **132** | C84 | The 3 × 42 flag matrix |
| **100** | C01 | The section-flag array (100 × `byte 1`) |
| **64** | C57 | Stolen skills — 16 ints, emitted for every job |
| **54** | C36 | Skill alarms — 3 parallel arrays of 6 |
| **50** | C18 | Equip-inventory scaffolding — 24 sentinels + 2 bytes (minimum, empty) |
| **48** | C12b | Three unconditional damage-skin triples |
| **48** | C91 | Red leaf info — 4 ints + `byte[32]` |
| **28** | C69 | Runner game record |
| **24** | C16 | Inventory sizes — 6 ints |
| **22** | C64 | Dress-up info |
| **22** | C65 | Active damage skin |
| **20** | C58 | Chosen skills — 5 ints |
| **16** | C68 | Like point |
| **12** | C03 | Pet cooltimes — 3 ints |

Plus the loops whose **trip count is fixed even though the contents are counted**: C34d runs
exactly 3 times, C59 runs exactly 3 times. You must emit three counts, not one.

### Minimum record size in v214

Collapsing every count to zero and every gate to false, a v214 character record still costs
**≈1945 bytes**, of which **≈1596 (82%) is the fixed blocks above**. The collapsible parts are
almost all cost-free; the floor is set by C76b, C05, C41, C42, C84 and C01 together (≈1184 B, 61%
of the floor).

### Calibration note for the mscw comparison

The mscw decoder was measured at **117 packet reads**. A v214 record at `DBChar.All` has well over
that at the top level alone — the fixed arrays above account for more than 117 reads by themselves
if the client reads them element-wise. **This strongly suggests mscw's record is a much smaller,
much older ancestor of this structure, not a trimmed version of it.** Treat the v214 list as an
*upper bound on structure* — expect most of C42–C91 to have no mscw counterpart at all, and expect
the mscw order to diverge from v214 well before the end of the list. The parts most likely to
survive backwards into an older client are the head: C01-analogue (or its absence), C05
(CharacterStat), C13 (money), C16 (inventory sizes), C18–C22 (inventories), C34 (skills),
C37/C38 (quests), C40 (couple/friend/marriage counts), C41 (teleport rocks). Everything past that
is v214-era content.
