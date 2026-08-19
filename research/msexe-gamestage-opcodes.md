# The channel stage's 273 opcodes: candidate names

**Status: one confirmed name, 272 candidates.** `0x0070` is `InventoryOperation`, read out
of mscw. Everything else in the table is an order-preserving alignment against *other
versions'* enums, anchored on that one name, and its accuracy decays with distance from it.
Read "The two controls" before using any name here.

Rebuilt 2026-08-19 against the corrected 273-label case table. The first draft used a
181-row extract that silently dropped every case with an inlined body; conclusions that
rested on the gaps in that file have been re-derived, and one of them changed.

## Sources

| | |
|---|---|
| mscw case list | `research/msexe-gamestage-cases.txt` - **273 labels / 272 bodies** from `FUN_142cbaa80`, `tools/switch_cases.py`, brace-depth aware so nested switches are not merged in |
| mscw dispatcher body | `research/msexe-gamestage-dispatch.c` |
| mscw `0x0070` decode | `research/msexe-setfield.md`, `research/msexe-setfield.c` - the anchor |
| reference (older) | `ModernMapleSource/v214 src/src/main/resources/ins.txt` - a **v214** client's inbound enum, `BEGIN_x`/`END_x` block markers |
| reference (newer) | `.../src/main/java/net/swordie/ms/handlers/header/OutHeader.java` - Swordie's own list, annotated **v265** |

## Read out of mscw (facts, not candidates)

1. The switch's labels are `0x70..0x19f` **271 of 304 slots**, then two outliers `0x275`
   and `0x39a`. "0x70..0x39a" is the switch's min/max, not its extent.
2. The 33 unclaimed slots inside `0x70..0x19f`, as runs:
   **`0x71-0x7a` (10)**, `0x90`, `0xaf-0xb1` (3), `0xf4`, `0xf7`, `0x102`, `0x105`, `0x119`,
   **`0x121-0x128` (8)**, `0x16c`, `0x175-0x176` (2), `0x184`, `0x188`, `0x19d`.
   The ten-slot run at the very top is the load-bearing one; see below.
3. `0x121..0x126` are **not** cases of the outer switch. They are dispatched from its
   **tail**, conditionally: `if (DAT_143ad1850 != 0) switch(op) { 0x121..0x126 }`, six
   consecutive opcodes to one object (`FUN_141a3c...`-`FUN_141a3e...`). `0x127` and `0x128`
   are handled nowhere in this function.
4. `0xb8` and `0xf5` share one body, and it is a no-op prologue
   (`switchD_142cbab2d_caseD_b8`): reset `param_1+0x466`, three `FUN_1429e3ef0` calls, fall
   through. They consume no packet bytes.
5. The tail pushes every opcode into a **20-entry ring buffer** at `param_1[0x74c]` - a
   "last 20 packets received" debug history.
6. Case `0x18b` contains the literal `"BossFirstClearRecord Is Empty ( bossID[%d]
   worldID[%d]"`, reads `u32 bossID, u32 worldID`, then a vector of strings, and logs each.
   **Neither reference has a name resembling it** - mscw's enum contains at least one packet
   neither knows.
7. Case `0x162` reads a `u32`, recomputes it locally, and on mismatch **sends outbound
   `0x175`**. Case `0x275` reads `u8, u32` and on `u8 == 0` **sends outbound `0x17e`**.
   Those are the only two request/response pairs in the switch.
8. Fifty-four cases read the packet inline; those read sequences are in the table's last
   column and are the only mscw-side evidence any candidate name can be checked against.


## The premise this file was started on is wrong, and the login range proves it

The brief said mscw is a *reduced* v214 - same order, entries deleted, values compressed.
The login range says the opposite, at least in the SOCKET block.

`docs/opcodes.md` has eight login-stage opcodes decoded out of the client. Lining their
**shapes** up against Swordie's builders (`Login.java`, `ClientSocket.java`) identifies them
by field order, not by number:

| mscw | decoded shape (from mscw) | matching v214 name | v214 value |
|---|---|---|---|
| `0x0000` | `u8 result, str msg, u8 verifyState, u32, str loginName, ...` | CheckPasswordResult | `0x00` |
| `0x000B` | one world per packet, high-bit id terminates | WorldInformation | `0x01` |
| `0x0010` | `u8, str, u8, FT, u32 world, u32 channel, ... + character list` | SelectWorldResult | `0x06` |
| `0x0011` | `u8, str, u8, u32 ip, u16 port, u32 charId, ... u8[8], u32 key, u32 len, <len>` | SelectCharacterResult | `0x07` |
| `0x0012` | shorter sibling of `0x0000`, ends with the account name | AccountInfoResult | `0x08` |
| `0x0014` | `str name, u8 result` | CheckDuplicatedIDResult | `0x0A` |
| `0x0015` | `u8 result`, then one character record | CreateNewCharacterResult | `0x0B` |
| `0x0016` | `u32 characterId` | DeleteCharacterResult | `0x0C` |

Two of these overturn an existing label:

* **`0x0011` is `SelectCharacterResult`, not `MigrateCommand`.** Swordie's
  `Login.selectCharacterResult` is `u8 loginType, str, u8 errorCode, u32 ip, u16 port, u32
  charId, ...` and ends `u64, u32, u32 length, <length bytes>` - which is
  `docs/opcodes.md`'s `0x0011` field for field, obfuscated tail included. Swordie's
  `ClientSocket.migrateCommand` is only `u8 succeed, 4B ip, u16 port, u32` with **no result
  string at all**. The name in `docs/opcodes.md` is a coincidence of numbering: v214's
  `MigrateCommand` really is `0x11`, and that is exactly the trap this file exists to
  measure.
* **`0x0010` is `SelectWorldResult`.** `Login.selectWorldResult` is `u8 code, str, u8, FT,
  u32 worldId, u32 channel-1, u32, u32, u32, u32, u8, u8`, then the character list - the
  same order `docs/opcodes.md` records.

So mscw's SOCKET block is `CheckPasswordResult` at `0x00`, then **ten mscw opcodes that do
not exist in v214**, then `WorldInformation` at `0x0B` and one-to-one from there.
**mscw is v214 plus insertions, not v214 minus deletions.** Independently: mscw's outbound
`0x007D` is the migration hello, and v265's `InHeader.MIGRATE_IN` is `125 = 0x7D` exactly -
mscw's numbering sits nearer v265 than v214.



## The two controls, and what they score

### Control 1: the login range

The method the brief asked for is: assume mscw keeps v214's block order, line the two
enumerations up block by block, allow deletions. Run blind on the login block that means
mscw's SOCKET starts where v214's SOCKET starts and runs one-to-one - i.e. the **identity**
map.

| mscw | identity alignment says | truth (read out of mscw) | |
|---|---|---|---|
| `0x0000` | CheckPasswordResult | CheckPasswordResult | correct |
| `0x000B` | CreateNewCharacterResult | WorldInformation | wrong |
| `0x0010` | SetCharacterID | SelectWorldResult | wrong |
| `0x0011` | MigrateCommand | SelectCharacterResult | wrong |
| `0x0012` | AliveReq | AccountInfoResult | wrong |
| `0x0014` | AuthenCodeChanged | CheckDuplicatedIDResult | wrong |
| `0x0015` | AuthenMessage | CreateNewCharacterResult | wrong |
| `0x0016` | SecurityPacket | DeleteCharacterResult | wrong |

**Control score: 1 of 8 (12.5%), and the one hit is the block's first entry, which any
alignment gets for free.** On the seven entries where the method had to do work it scored
zero.

Give the method **one** anchor inside the block - "`0x000B` is WorldInformation" - and the
remaining seven fall out correctly, 7/7. That is the real shape of the result:

> Order-preserving alignment reproduces the *interior* of a block perfectly once the block's
> offset is known, and determines the offset not at all.

### Control 2: the game range, `0x0070`

`research/msexe-setfield.md` decoded `FUN_142d51930` down to the assembly, independently of
this alignment: it is `CWvsContext::OnInventoryOperation` / `ModifyInventoryItem` - header
`u8 bExclRequestSent, u8, i32 nCount`, then `nCount` entries, each resolving to "find the
item at *(inventory type, slot)* and put something there / take it away / renumber it", then
a UI refresh. Its 13 modes map one-for-one onto v214's `InventoryOperation`. It touches no
map, field, portal or channel id.

| method | says `0x0070` is | |
|---|---|---|
| identity against v214 (the blind method) | `AllianceResult` | wrong |
| block-anchored (mscw `MAP` in `0x60..0x6f`, `CHARACTERDATA` from `0x70`) | `InventoryOperation` | **correct** |

**Combined blind score: 1 of 9.** The block-anchored variant scores 1 of 1 on this control,
but it had to be told where the block starts, which is the same free parameter as before.

What control 2 *does* buy: it fixes the block premise. `FUN_142cbaa80` is
`CWvsContext::OnPacket`, mscw's CHARACTERDATA block begins at `0x0070`, and the 16-slot hole
`0x60..0x6f` that neither dispatcher claims is the `MAP` block (v214 has 17 entries there).
It also kills, independently, the standing suspicion that `0x0070` is `SetField`.

## The ten-slot hole, and how v265 settles the offset

With the block start fixed, one question remains: what sits in `0x71..0x7a`?

* If mscw ran 1:1 with v214 from `0x70` (**alignment A**, offset `+0x39`), those ten slots
  are `InventoryGrow, StatChanged, TemporaryStatSet, TemporaryStatReset, ForcedStatSet,
  ForcedStatReset, ChangeSkillRecordResult, ChangeStealMemoryResult,
  UserDamageOnFallingCheck, PersonalShopBuyCheck`. **`StatChanged` and `TemporaryStatSet`
  cannot be unhandled in `CWvsContext::OnPacket`** - they are the two most frequently sent
  server packets in the game.
* If mscw inserted ten opcodes after `InventoryOperation` and resumed at `0x7b`
  (**alignment B**, offset `+0x43`), `0x7c` is `StatChanged` and nothing important is
  orphaned.

The v265 reference decides it. Swordie's `OutHeader` `CWvsContext` block reads:

```text
INVENTORY_OPERATION  = 144        <- mscw 0x70 = 112
   (145 .. 154, ten slots)        <- mscw 0x71..0x7a, ten slots
INVENTORY_GROW       = 155        <- mscw 0x7b = 123
```

**A ten-slot gap in exactly the same place, and a constant delta of -32 across it.** That is
a structural fingerprint, not a numerical coincidence: the ten opcodes mscw has after
`InventoryOperation` are ten opcodes v265 also has and v214 does not.

Two supporting checks, both cheap:

* Aligning v265's `CWvsContext` names against v214's `CHARACTERDATA` names **by name**
  matches 175 entries with **zero order inversions**. The enum's relative order really is
  stable across ~50 versions, which is the premise this whole method rests on.
* `0x8c`'s handler is **188,565 bytes**, by far the largest in the switch. B calls it
  `Message` - the multiplexed message packet, one of the biggest handlers in any MapleStory
  client. A calls it `SetTamingMobInfo`.

**Alignment A is dead at the block start. Alignment B is right there.**

## But B is wrong at the block end, and that is the answer to "where is SetField"

Run B to the end of the block and it predicts `MacroSysDataInit` - the last entry of
CHARACTERDATA in *both* references - at mscw `0x1a9`, leaving ten CHARACTERDATA entries
(`0x1a0..0x1a9`, including `NeedClientResponse` and `CharacterModified`) with no case.
Alignment A instead lands `MacroSysDataInit` exactly on `0x19f`, the switch's last in-block
case.

Both are true if mscw **deleted about ten entries somewhere in the middle of the block**:

```text
mscw CHARACTERDATA = v214 CHARACTERDATA
                     + 10 (the v265-era insertion after InventoryOperation)
                     - 10 (classic-content removals, position unknown)
                   = 304 slots = 0x0070 .. 0x019F exactly
```

That is why `0x70..0x19f` came out to exactly 304 slots against v214's exactly 304 names -
not a coincidence, a cancellation. And it is directly visible in the table: the mapping
must track **B at the top** and **A at the bottom**. Where it changes over is not located,
and the spot checks do not line up neatly:

| mscw | observed in mscw | favours |
|---|---|---|
| `0x8c` | 188,565-byte handler | B (`Message`) |
| `0xb3` | `u32 count`, then `{u32 id, str name}` into a name cache | B (`AllianceResult` load) |
| `0xd0`, `0xd4` | both `str name, u32, u32, u8` into one UI (`FUN_1425d0140` on `DAT_143acaf20`) | A (`FamilyInfoResult` / `FamilyJoinAccepted` carry names; B's `MiniMapOnOff` cannot) |
| `0x121..0x126` | six consecutive opcodes to one UI object | A (`ReturnEffect*` / `*CubeResult` - one cube UI; B's six are unrelated) |
| `0x13e`, `0x13f`, `0x140` | three **byte-identical** bodies: `u8`, string-table lookup, log | B (`GetLotteryResult, CheckProcess, CompleteNpcSpeechSuccess` - three `u8 result` notices; A's `LikePoint, SignErrorAck, AskAfterErrorAck` is not a uniform run) |

So the crossover is not simply "before `0x121`" - `0x13e..0x140` still reads better as B.
**Treat every row as +/- 10 entries and never as exact.** The only thing the two columns
agree on is `0x0070`, and that one is confirmed.

### SetField

In both references `BEGIN_STAGE = SetField` is the entry immediately after
`END_CHARACTERDATA`. With the block ending at `0x019F`:

> **`SetField` is `0x01A0`** - *candidate from v214/v265, not read out of mscw.*
>
> Range if the block runs a few unhandled slots past the switch's last case:
> **`0x01A0` .. `0x01AA`**. `SetFarmField` and `SetCashShop` follow it;
> `BEGIN_FIELD / TransferFieldReqIgnored` is about `0x01A3`.

It is **outside `FUN_142cbaa80`**, one slot past its last case, in `CStage::OnPacket` -
which is why the channel dispatcher has no case that could answer the client's `0x007D`
migration hello. That function has not been found yet; it is the next thing to look for,
and `0x01A0` is the case label to look for inside it.

The two outliers corroborate the block boundary rather than contradicting it: `0x275` and
`0x39a` sit far past `0x1a0`, i.e. inside the `FIELD` block, which is exactly where a
`CWvsContext` dispatcher would be expected to peek at a handful of field-level packets.

### How to check `0x01A0` for the cost of one decode

`SetField` with `characterData = true` opens with an **8-byte FILETIME, `u8`, `u32
channelId-1`, `u8`, `u32`**, and then three `u32` damage-calc seeds before the character
blob - see the Swordie shape at the bottom of this file. A handler that starts by reading 8
raw bytes and stashing them as a time base is unmistakable. If `0x01A0` has no case
anywhere, walk down: `0x01A1`, `0x01A2`, ... to `0x01AA`.

## The table

Column **B** is the better name near the top of the block, column **A** near the bottom;
they are ten entries apart and the crossover is not located. Both are **candidates from
v214**. The last column is the only mscw-side content in the row.

Rows whose body is inlined in the dispatcher are marked `inline:` and list the first
functions the body calls.

| mscw | handler / body | candidate B (block start) | candidate A (block end) | read sequence (mscw) |
|---|---|---|---|---|
| `0x0070` | `FUN_142d51930` | InventoryOperation | = B |  |
| `0x007B` | `FUN_142d54700` | InventoryGrow | MobDropMesoPickup |  |
| `0x007C` | `FUN_142d54780` | StatChanged | BreakTimeFieldEnter |  |
| `0x007D` | `FUN_142d563d0` | TemporaryStatSet | RuneActSuccess |  |
| `0x007E` | `FUN_142d56f80` | TemporaryStatReset | ResultStealSkillList |  |
| `0x007F` | `FUN_142d57ea0` | ForcedStatSet | SkillUseResult |  |
| `0x0080` | `FUN_142d57ee0` | ForcedStatReset | ExclRequest |  |
| `0x0081` | `FUN_142d57f20` | ChangeSkillRecordResult | GivePopularityResult |  |
| `0x0082` | `FUN_142d608f0` | ChangeStealMemoryResult | Message |  |
| `0x0083` | `FUN_142d60aa0` | UserDamageOnFallingCheck | MemoResult |  |
| `0x0084` | `FUN_142d58bb0` | PersonalShopBuyCheck | MapTransferResult |  |
| `0x0085` | `FUN_142d58c40` | MobDropMesoPickup | AntiMacroResult |  |
| `0x0086` | `FUN_142d58c50` | BreakTimeFieldEnter | AntiMacroBombResult |  |
| `0x0087` | `FUN_142d58c60` | RuneActSuccess | InitialQuizStart |  |
| `0x0088` | `FUN_142d59010` | ResultStealSkillList | ClaimResult |  |
| `0x0089` | `FUN_142d43ee0` | SkillUseResult | SetClaimSvrAvailableTime |  |
| `0x008A` | `FUN_142d99ea0` | ExclRequest | ClaimSvrStatusChanged |  |
| `0x008B` | `FUN_142cd8bd0` | GivePopularityResult | StarPlanetUserCount |  |
| `0x008C` | `FUN_142d634c0` | Message | SetTamingMobInfo |  |
| `0x008D` | `FUN_142d91560` | MemoResult | QuestClear |  |
| `0x008E` | `FUN_142d92bd0` | MapTransferResult | EntrustedShopCheckResult |  |
| `0x008F` | `FUN_142dea760` | AntiMacroResult | SkillLearnItemResult |  |
| `0x0091` | `FUN_142d93b40` | InitialQuizStart | AbilityResetItemResult |  |
| `0x0092` | `FUN_142d94300` | ClaimResult | ExpConsumeResetItemResult |  |
| `0x0093` | `FUN_142d94250` | SetClaimSvrAvailableTime | ExpItemGetResult |  |
| `0x0094` | `FUN_142d94280` | ClaimSvrStatusChanged | CharSlotIncItemResult |  |
| `0x0095` | `FUN_142d942b0` | StarPlanetUserCount | CharRenameItemResult |  |
| `0x0096` | `FUN_142d94350` | SetTamingMobInfo | GatherItemResult |  |
| `0x0097` | `FUN_142d93310` | QuestClear | SortItemResult |  |
| `0x0098` | `FUN_142d948e0` | EntrustedShopCheckResult | RemoteShopOpenResult |  |
| `0x0099` | `FUN_142d94d10` | SkillLearnItemResult | PetDeadMessage |  |
| `0x009A` | `FUN_142d94f00` | SkillResetItemResult | CharacterInfo |  |
| `0x009B` | `FUN_142d950f0` | AbilityResetItemResult | PartyResult |  |
| `0x009C` | `FUN_142d95350` | ExpConsumeResetItemResult | PartyMemberCandidateResult |  |
| `0x009D` | `FUN_142d95580` | ExpItemGetResult | UrusPartyMemberCandidateResult |  |
| `0x009E` | `FUN_142d955d0` | CharSlotIncItemResult | PartyCandidateResult |  |
| `0x009F` | `FUN_142d95600` | CharRenameItemResult | UrusPartyResult |  |
| `0x00A0` | `FUN_142d56370` | GatherItemResult | IntrusionFriendCandidateResult |  |
| `0x00A1` | `FUN_142d1cdf0` | SortItemResult | IntrusionLobbyCandidateResult |  |
| `0x00A2` | `FUN_142cd87d0` | RemoteShopOpenResult | ExpeditionRequest |  |
| `0x00A3` | inline: `FUN_141183ec0` | PetDeadMessage | ExpeditionNoti |  |
| `0x00A4` | inline: `FUN_1406e9170 FUN_141351aa0` | CharacterInfo | FriendResult | `raw` |
| `0x00A5` | inline: `FUN_1413bab80` | PartyResult | StarFriendResult |  |
| `0x00A6` | `FUN_142defd30` | PartyMemberCandidateResult | LoadAccountIDOfCharacterFriendResult |  |
| `0x00A7` | `FUN_142defd40` | UrusPartyMemberCandidateResult | GuildRequest |  |
| `0x00A8` | `FUN_142de58b0` | PartyCandidateResult | GuildResult |  |
| `0x00A9` | `FUN_142ddfd10` | UrusPartyResult | AllianceResult |  |
| `0x00AA` | inline: `FUN_14035d880` | IntrusionFriendCandidateResult | TownPortal |  |
| `0x00AB` | `FUN_142d60c70` | IntrusionLobbyCandidateResult | BroadcastMsg |  |
| `0x00AC` | `FUN_142d60d40` | ExpeditionRequest | AswanTimeTableState |  |
| `0x00AD` | `FUN_142dae820` | ExpeditionNoti | IncubatorResult |  |
| `0x00AE` | `FUN_142dafc80` | FriendResult | IncubatorHotItemResult |  |
| `0x00B2` | inline: `FUN_1406e8c20` | GuildResult | AuctionMessage | `u32,u32` |
| `0x00B3` | inline: `(no calls)` | AllianceResult | MarriageRequest | `u32,u32,str` |
| `0x00B4` | `FUN_142d09280` | TownPortal | MarriageResult |  |
| `0x00B5` | `FUN_142d92e30` | BroadcastMsg | WeddingGiftResult |  |
| `0x00B6` | `FUN_142d94670` | AswanTimeTableState | MarriedPartnerMapTransfer |  |
| `0x00B7` | inline: `FUN_1429e3ef0 FUN_142e54b20 FUN_142e54f40` | IncubatorResult | CashPetFoodResult | `u8,u8` |
| `0x00B8` | inline: `(no calls)` | IncubatorHotItemResult | CashPetPickUpOnOffResult |  |
| `0x00F5` | inline: `(no calls)` | ZeroWP | SetSonOfLinkedSkillResult |  |
| `0x00B9` | `FUN_142cd8ee0` | ShopScannerResult | CashPetSkillSettingResult |  |
| `0x00BA` | `FUN_142cd92f0` | ShopLinkResult | CashLookChangeResult |  |
| `0x00BB` | `FUN_142d95630` | AuctionResult | CashPetDyeingResult |  |
| `0x00BC` | `FUN_142d956f0` | AuctionMessage | SetWeekEventMessage |  |
| `0x00BD` | `FUN_142cd9980` | MarriageRequest | SetPotionDiscountRate |  |
| `0x00BE` | inline: `(no calls)` | MarriageResult | BridleMobCatchFail |  |
| `0x00BF` | inline: `FUN_1406e8b80 FUN_140cf1560 FUN_141779560` | WeddingGiftResult | ImitatedNPCResult | `u16,u16` |
| `0x00C0` | inline: `FUN_1406e8ae0` | MarriedPartnerMapTransfer | ImitatedNPCData | `u8` |
| `0x00C1` | `FUN_142cd9bb0` | CashPetFoodResult | JournalAvatar |  |
| `0x00C2` | `FUN_142cd9de0` | CashPetPickUpOnOffResult | LimitedNPCDisableInfo |  |
| `0x00C3` | `FUN_142cda010` | CashPetSkillSettingResult | MonsterBookSetCard |  |
| `0x00C4` | `FUN_142cda260` | CashLookChangeResult | MonsterBookSetCover |  |
| `0x00C5` | `FUN_142cdafc0` | CashPetDyeingResult | HourChanged |  |
| `0x00C6` | `FUN_142cdb120` | SetWeekEventMessage | MiniMapOnOff | `str,str` |
| `0x00C7` | inline: `FUN_1406e9050 FUN_141892840 FUN_14019a260` | SetPotionDiscountRate | ConsultAuthkeyUpdate |  |
| `0x00C8` | `FUN_142cdb4c0` | BridleMobCatchFail | ClassCompetitionAuthkeyUpdate |  |
| `0x00C9` | inline: `FUN_14211df30` | ImitatedNPCResult | WebBoardAuthkeyUpdate |  |
| `0x00CA` | `FUN_142d9dc70` | ImitatedNPCData | SessionValue |  |
| `0x00CB` | inline: `FUN_1406e9050 FUN_1406e8c20` | JournalAvatar | PartyValue | `str,u32` |
| `0x00CC` | inline: `FUN_1406e9050` | LimitedNPCDisableInfo | FieldSetVariable | `str` |
| `0x00CD` | inline: `FUN_1406e8c20 FUN_1406e9050` | MonsterBookSetCard | FieldValue | `u32,str` |
| `0x00CE` | inline: `FUN_1406e8ae0 FUN_1406e9050` | MonsterBookSetCover | BonusExpRateChanged | `u8,str,u8` |
| `0x00CF` | inline: `FUN_141e2c990` | HourChanged | FamilyChartResult |  |
| `0x00D0` | inline: `FUN_1406e9050 FUN_1406e8c20 FUN_1406e8ae0` | MiniMapOnOff | FamilyInfoResult | `str,u32,u32,u8` |
| `0x00D1` | inline: `FUN_1406e9050 FUN_1406e8c20` | ConsultAuthkeyUpdate | FamilyResult | `str,u32,u32,u32` |
| `0x00D2` | `FUN_142cdcef0` | ClassCompetitionAuthkeyUpdate | FamilyJoinRequest |  |
| `0x00D3` | inline: `FUN_1406e9050 FUN_1406e8c20 FUN_1406e8ae0` | WebBoardAuthkeyUpdate | FamilyJoinRequestResult | `str,u32,u32,u8,u32` |
| `0x00D4` | inline: `FUN_1406e9050 FUN_1406e8c20 FUN_1406e8ae0` | SessionValue | FamilyJoinAccepted | `str,u32,u32,u8,u32` |
| `0x00D5` | inline: `FUN_1406e8c20 FUN_1406e8ae0 FUN_1406e9050` | PartyValue | FamilyPrivilegeList | `u32,u32,u32,u32,u8,str` |
| `0x00D6` | `FUN_142cdd720` | FieldSetVariable | FamilyFamousPointIncResult |  |
| `0x00D7` | inline: `FUN_1406e9170 FUN_140909d80 FUN_1407a3230` | FieldValue | FamilyNotifyLoginOrLogout | `raw,raw` |
| `0x00D8` | `FUN_142cddaa0` | BonusExpRateChanged | FamilySetPrivilege |  |
| `0x00D9` | `FUN_142cddf10` | FamilyChartResult | FamilySummonRequest |  |
| `0x00DA` | `FUN_142cde3c0` | FamilyInfoResult | NotifyLevelUp |  |
| `0x00DB` | inline: `FUN_142d9c9f0` | FamilyResult | NotifyWedding |  |
| `0x00DC` | `FUN_142dd22b0` | FamilyJoinRequest | NotifyJobChange |  |
| `0x00DD` | `FUN_142dd43b0` | FamilyJoinRequestResult | SetBuyEquipExt |  |
| `0x00DE` | `FUN_142dd4570` | FamilyJoinAccepted | SetPassenserRequest |  |
| `0x00DF` | `FUN_142de9210` | FamilyPrivilegeList | ScriptProgressMessageBySoul |  |
| `0x00E0` | `FUN_142de9540` | FamilyFamousPointIncResult | ScriptProgressMessage |  |
| `0x00E1` | inline: `FUN_1406e8c20 FUN_1406e9170` | FamilyNotifyLoginOrLogout | ScriptProgressItemMessage | `u32,raw` |
| `0x00E2` | `FUN_142cef640` | FamilySetPrivilege | SetStaticScreenMessage |  |
| `0x00E3` | `FUN_142d96920` | FamilySummonRequest | OffStaticScreenMessage |  |
| `0x00E4` | `FUN_142d96960` | NotifyLevelUp | WeatherEffectNotice |  |
| `0x00E5` | `FUN_142ce7780` | NotifyWedding | WeatherEffectNoticeY |  |
| `0x00E6` | inline: `FUN_1406e8c20 FUN_1402d2c10` | NotifyJobChange | ProgressMessageFont | `u32,u32,u32,u16` |
| `0x00E7` | inline: `FUN_1406e8b80` | SetBuyEquipExt | DataCRCCheckFailed | `u16,u16,u8` |
| `0x00E8` | `FUN_142cdecc0` | SetPassenserRequest | ShowSlotMessage |  |
| `0x00E9` | `FUN_142cdf2b0` | ScriptProgressMessageBySoul | WildHunterInfo |  |
| `0x00EA` | `FUN_142d9fa90` | ScriptProgressMessage | ZeroInfo |  |
| `0x00EB` | `FUN_142d9fc00` | ScriptProgressItemMessage | ZeroWP |  |
| `0x00EC` | `FUN_142d9fcf0` | SetStaticScreenMessage | ZeroInfoSubHP |  |
| `0x00ED` | `FUN_142da08c0` | OffStaticScreenMessage | OpenUICreatePremiumAdventurer |  |
| `0x00EE` | `FUN_142da0950` | WeatherEffectNotice | FieldSetEnterSuccessed |  |
| `0x00EF` | `FUN_142cf1e40` | WeatherEffectNoticeY | ResultInstanceTable |  |
| `0x00F0` | `FUN_142cf1c50` | ProgressMessageFont | CoolTimeSet |  |
| `0x00F1` | `FUN_142d96c10` | DataCRCCheckFailed | ItemPotChange |  |
| `0x00F2` | `FUN_142d96c60` | ShowSlotMessage | ItemCoolTimeChange |  |
| `0x00F3` | `FUN_142d96d90` | WildHunterInfo | SetAdDisplayInfo |  |
| `0x00F6` | `FUN_142d4a670` | ZeroInfoSubHP | SetMapleStyeInfo |  |
| `0x00F8` | inline: `FUN_1429e3ef0 FUN_142e54b20 FUN_142e54f40` | FieldSetEnterSuccessed | ResetBuyLimitcount | `u8` |
| `0x00F9` | inline: `FUN_1429e3ef0 FUN_142e54b20 FUN_142e54f40` | ResultInstanceTable | UpdateUIEventListInfo | `u8` |
| `0x00FA` | inline: `FUN_1406e9170 FUN_142cbefd0` | CoolTimeSet | DojangRanking | `raw` |
| `0x00FB` | `FUN_142cf22d0` | ItemPotChange | DefenseGameResponse_Shop |  |
| `0x00FC` | inline: `FUN_1406e8c20 FUN_142cbefd0` | ItemCoolTimeChange | DefenseGameResponse_Inventory | `u32` |
| `0x00FD` | inline: `FUN_1408c67b0` | SetAdDisplayInfo | ShutdownMessage |  |
| `0x00FE` | inline: `FUN_1414b0970` | SetAdDisplayStatus | ResultSetStealSkill |  |
| `0x00FF` | `FUN_142d95e20` | SetSonOfLinkedSkillResult | SlashCommand |  |
| `0x0100` | `FUN_142d96060` | SetMapleStyeInfo | StartNavigationRequest |  |
| `0x0101` | `FUN_142d96090` | SetBuyLimitCount | FuncKeySetByScript |  |
| `0x0103` | `FUN_142d95790` | UpdateUIEventListInfo | CharacterPotentialReset |  |
| `0x0104` | `FUN_142d95920` | DojangRanking | CharacterHonorExp |  |
| `0x0106` | inline: `FUN_142aa2810 FUN_1406e9050` | DefenseGameResponse_Inventory | AswanResult | `str` |
| `0x0107` | `FUN_142d95ab0` | ShutdownMessage | ReadyForRespawn |  |
| `0x0108` | inline: `FUN_1413ce880` | ResultSetStealSkill | ReadyForRespawnByPoint |  |
| `0x0109` | `FUN_142cf5f20` | SlashCommand | OpenReadyForRespawnUI |  |
| `0x010A` | inline: `(no calls)` | StartNavigationRequest | CharacterHonorGift |  |
| `0x010B` | `FUN_142da1140` | FuncKeySetByScript | CrossHunterCompleteResult |  |
| `0x010C` | `FUN_142da1320` | CharacterPotentialSet | CrossHunterShopResult |  |
| `0x010D` | inline: `(no calls)` | CharacterPotentialReset | SetCashItemNotice |  |
| `0x010E` | `FUN_142da37e0` | CharacterHonorExp | SetSpecialCashItem |  |
| `0x010F` | inline: `(no calls)` | AswanStateInfo | ShowEventNotice |  |
| `0x0110` | `FUN_142cf4f20` | AswanResult | BoardGameResult |  |
| `0x0111` | `FUN_142da9aa0` | ReadyForRespawn | YutGameResult |  |
| `0x0112` | `FUN_142d97010` | ReadyForRespawnByPoint | ValuePackResult |  |
| `0x0113` | `FUN_142da4710` | OpenReadyForRespawnUI | UserUseNaviFlyingResult |  |
| `0x0114` | `FUN_142da4820` | CharacterHonorGift | MapleStyleResult |  |
| `0x0115` | `FUN_142da5190` | CrossHunterCompleteResult | OpenWeddingEx |  |
| `0x0116` | inline: `FUN_1406e8c20 FUN_1406e8ae0 FUN_142cf6d40` | CrossHunterShopResult | BingoResult | `u32,u8` |
| `0x0117` | `FUN_142cfacd0` | SetCashItemNotice | BingoCassandraResult |  |
| `0x0118` | inline: `FUN_1426181c0` | SetSpecialCashItem | UpdateVIPGrade |  |
| `0x011A` | inline: `FUN_1426181a0` | BoardGameResult | SetMaplePoint |  |
| `0x011B` | `FUN_142da51e0` | YutGameResult | SetAdditionalCashInfo |  |
| `0x011C` | `FUN_142cf5290` | ValuePackResult | SetMiracleTime |  |
| `0x011D` | `FUN_142cf5410` | UserUseNaviFlyingResult | HyperSkillResetResult |  |
| `0x011E` | `FUN_142cf5860` | MapleStyleResult | GetServerTime |  |
| `0x011F` | `FUN_142da52e0` | OpenWeddingEx | GetCharacterPosition |  |
| `0x0120` | `FUN_142da5450` | BingoResult | SetFixDamageForTest |  |
| `0x0129` | inline: `FUN_1406e9050 FUN_14019a260 FUN_142cf07e0` | GetCharacterPosition | SetOffStateForOnOffSkill | `str` |
| `0x012A` | `FUN_142cea4b0` | SetFixDamageForTest | IssueReloginCookie |  |
| `0x012B` | `FUN_142cea890` | ReturnEffectConfirm | AvatarPackTest |  |
| `0x012C` | `FUN_142df3650` | ReturnEffectModified | EvolvingResult |  |
| `0x012D` | `FUN_142d95480` | WhiteAdditionalCubeResult | ActionBarResult |  |
| `0x012E` | `FUN_142df3e40` | BlackCubeResult | GuildContentResult |  |
| `0x012F` | inline: `FUN_1406e8c20` | MemorialCubeResult | GuildSearchResult | `u32` |
| `0x0130` | `FUN_142dc72d0` | MemorialCubeModified | BufferFlyResult |  |
| `0x0131` | `FUN_142dc7360` | DressUpInfoModified | HalloweenCandyRankingResult |  |
| `0x0132` | inline: `FUN_14019b780` | ResetOnStateForOnOffSkill | GetRewardResult |  |
| `0x0133` | inline: `FUN_14019b780` | SetOffStateForOnOffSkill | Mentoring |  |
| `0x0134` | inline: `FUN_1406e8c20` | IssueReloginCookie | GetLotteryResult | `u32,u32,u32,u32,u32,u8` |
| `0x0135` | inline: `FUN_1406e9170 FUN_1408f6690 FUN_1429e3ef0` | AvatarPackTest | CheckProcess | `raw,raw` |
| `0x0136` | `FUN_142da6800` | EvolvingResult | CompleteNpcSpeechSuccess |  |
| `0x0137` | inline: `(no calls)` | ActionBarResult | CompleteSpecialCheckSuccess |  |
| `0x0138` | `FUN_142d012e0` | GuildContentResult | SetAccountInfo |  |
| `0x0139` | `FUN_142cff620` | GuildSearchResult | SetGachaponFeverTime |  |
| `0x013A` | inline: `FUN_14019b780` | BufferFlyResult | AvatarMegaphoneRes |  |
| `0x013B` | inline: `FUN_1406e8ae0` | HalloweenCandyRankingResult | AvatarMegaphoneUpdateMessage | `u8` |
| `0x013C` | inline: `FUN_1406e8c20 FUN_1406e9050 FUN_14276df20` | GetRewardResult | AvatarMegaphoneClearMessage | `u32,u32,str` |
| `0x013D` | inline: `FUN_1406e8c20 FUN_1408a9e40 FUN_14019a260` | Mentoring | RequestEventList | `u32` |
| `0x013E` | inline: `FUN_1406e8ae0 FUN_1408a9e40 FUN_1415eca30` | GetLotteryResult | LikePoint | `u8` |
| `0x013F` | inline: `FUN_1406e8ae0 FUN_1408a9e40 FUN_1415eca30` | CheckProcess | SignErrorAck | `u8` |
| `0x0140` | inline: `FUN_1406e8ae0 FUN_1408a9e40 FUN_1415eca30` | CompleteNpcSpeechSuccess | AskAfterErrorAck | `u8` |
| `0x0141` | `FUN_142dadbb0` | CompleteSpecialCheckSuccess | EventNameTagInfo |  |
| `0x0142` | inline: `FUN_1406e8ae0 FUN_142cc4430` | SetAccountInfo | GiveEventNameTag | `u8` |
| `0x0143` | inline: `FUN_1406e8c20` | SetGachaponFeverTime | JobFreeChangeResult | `u32,u32` |
| `0x0144` | `FUN_142da6d70` | AvatarMegaphoneRes | EventLotteryOpen |  |
| `0x0145` | `FUN_142da7550` | AvatarMegaphoneUpdateMessage | EventLotteryResult |  |
| `0x0146` | inline: `(no calls)` | AvatarMegaphoneClearMessage | InvasionSupportSet |  |
| `0x0147` | `FUN_142da58d0` | RequestEventList | InvasionSupportAttackResult |  |
| `0x0148` | `FUN_142da59e0` | LikePoint | InvasionSupportBossKill |  |
| `0x0149` | `FUN_142da97d0` | SignErrorAck | InvasionSupportSetttingResult |  |
| `0x014A` | `FUN_142d9e780` | AskAfterErrorAck | InvasionElapsedTime |  |
| `0x014B` | `FUN_142d4fd50` | EventNameTagInfo | InvasionSystemMsg |  |
| `0x014C` | `FUN_142dea850` | GiveEventNameTag | InvasionBossKeyChange |  |
| `0x014D` | inline: `(no calls)` | JobFreeChangeResult | ScreenMsg |  |
| `0x014E` | `FUN_142d4fe10` | EventLotteryOpen | TradeBlockForSnapshot |  |
| `0x014F` | inline: `FUN_1406e8c20 FUN_1406e8ae0` | EventLotteryResult | LimitGoodsNoticeResult | `u32,u8,u32,u32` |
| `0x0150` | `FUN_142da9c70` | InvasionSupportSet | MonsterBattle |  |
| `0x0151` | `FUN_142da9d00` | InvasionSupportAttackResult | MonsterBattleCombat |  |
| `0x0152` | `FUN_142da9d40` | InvasionSupportBossKill | UniverseBossPossible |  |
| `0x0153` | `FUN_142da9db0` | InvasionSupportSetttingResult | UniverseBossImpossible |  |
| `0x0154` | inline: `FUN_140f62a20` | InvasionElapsedTime | CashShopPreviewInfo |  |
| `0x0155` | `FUN_142d945d0` | InvasionSystemMsg | ChangeSoulCollectionResult |  |
| `0x0156` | `FUN_141174cc0` | InvasionBossKeyChange | SelectSoulCollectionResult |  |
| `0x0157` | `FUN_142d96dd0` | ScreenMsg | UserMasterPieceResult |  |
| `0x0158` | `FUN_142daa080` | TradeBlockForSnapshot | PendantSlotIncResult |  |
| `0x0159` | inline: `FUN_142d23bd0` | LimitGoodsNoticeResult | BossArenaMatchSucess |  |
| `0x015A` | `FUN_142d96e60` | MonsterBattle | BossArenaMatchFail |  |
| `0x015B` | inline: `(no calls)` | MonsterBattleCombat | BossArenaMatchRequestDone | `u32` |
| `0x015C` | `FUN_140d2d4c0` | UniverseBossPossible | UserSoulMatching |  |
| `0x015D` | `FUN_140d2d4d0` | UniverseBossImpossible | Catapult_UpgradeSkill |  |
| `0x015E` | `FUN_140d2d510` | CashShopPreviewInfo | Catapult_ResetSkill |  |
| `0x015F` | `FUN_142da3f30` | ChangeSoulCollectionResult | PartyQuestRankingResult |  |
| `0x0160` | `FUN_142da42b0` | SelectSoulCollectionResult | CoordinationContestInfo |  |
| `0x0161` | `FUN_142d4a7f0` | UserMasterPieceResult | WorldTransferResult |  |
| `0x0162` | inline: `FUN_1406e8c20` | PendantSlotIncResult | TrunkSlotIncItemResult | `u32` |
| `0x0163` | `FUN_142daabd0` | BossArenaMatchSucess | EliteMobWorldMapNotice |  |
| `0x0164` | `FUN_142daad10` | BossArenaMatchFail | RandomPortalWorldMapNotice |  |
| `0x0165` | `FUN_142dad160` | BossArenaMatchRequestDone | WorldTransferHelperNotify |  |
| `0x0166` | `FUN_142dd9720` | UserSoulMatching | EquipmentEnchantDisplay |  |
| `0x0167` | `FUN_142dd9790` | Catapult_UpgradeSkill | TopTowerRankResult |  |
| `0x0168` | `FUN_142dc5e20` | Catapult_ResetSkill | FriendTowerRankResult |  |
| `0x0169` | inline: `FUN_1406e8fb0` | PartyQuestRankingResult | TowerResultUIOpen |  |
| `0x016A` | inline: `FUN_142d2e450 FUN_14019b780 FUN_1406e8ae0` | CoordinationContestInfo | MannequinResult | `u8,u8,u8` |
| `0x016B` | inline: `FUN_1408fcaa0` | WorldTransferResult | IronBoxEvent |  |
| `0x016D` | `FUN_142dc6f00` | EliteMobWorldMapNotice | CreateSwingGame |  |
| `0x016E` | `FUN_142dc70c0` | RandomPortalWorldMapNotice | UserUpdateMapleTVShowTime |  |
| `0x016F` | `FUN_142dcb0d0` | WorldTransferHelperNotify | ReturnToTitle |  |
| `0x0170` | inline: `FUN_1406e9170 FUN_141f1bc00` | EquipmentEnchantDisplay | ReturnToCharacterSelect | `raw` |
| `0x0171` | `FUN_142dc9e90` | TopTowerRankResult | FlameWizardFlameWalkEffect |  |
| `0x0172` | `FUN_142db6e10` | FriendTowerRankResult | FlameWizardFlareBlink |  |
| `0x0173` | `FUN_142db7390` | TowerResultUIOpen | SummonedAvatarSync |  |
| `0x0174` | inline: `FUN_142d0a6e0` | MannequinResult | CashShopEventInfo |  |
| `0x0177` | inline: `FUN_1408f5fa0` | CreateSwingGame | BlackListView |  |
| `0x0178` | inline: `FUN_142130730` | UserUpdateMapleTVShowTime | ScrollUpgradeFeverTime |  |
| `0x0179` | inline: `FUN_1406e8c20` | ReturnToTitle | TextEquipInfo | `u32,u32,u8,u32,u8,raw` |
| `0x017A` | `FUN_142cb6360` | ReturnToCharacterSelect | TextEquipUIOpen |  |
| `0x017B` | `FUN_142d4ffa0` | FlameWizardFlameWalkEffect | UIStarPlanetMiniGameResult |  |
| `0x017C` | inline: `FUN_1406e8ae0 FUN_1406e9170 FUN_1408f6690` | FlameWizardFlareBlink | UIStarPlanetTrendShop | `u8,raw` |
| `0x017D` | `FUN_142d161a0` | SummonedAvatarSync | UIStarPlanetQueue |  |
| `0x017E` | inline: `FUN_1406e8c20` | CashShopEventInfo | UIStarPlanetQueueErr | `u32,str,u32` |
| `0x017F` | inline: `(no calls)` | BlackList | StarPlanetRoundInfo |  |
| `0x0180` | inline: `FUN_140289a70` | UIOpenTest | StarPlanetResult |  |
| `0x0181` | inline: `FUN_140352f40` | BlackListView | BackSpeedCtrl |  |
| `0x0182` | inline: `FUN_1406e8c20 FUN_1406e8ae0` | ScrollUpgradeFeverTime | SetMazeArea | `u32,u8` |
| `0x0183` | inline: `(no calls)` | TextEquipInfo | CharacterBurning | `u32` |
| `0x0185` | `FUN_142d9e760` | UIStarPlanetMiniGameResult | BattleStatCoreAck |  |
| `0x0186` | inline: `FUN_1406e8c20 FUN_1411fda50` | UIStarPlanetTrendShop | GachaponRewardTestResult | `u32,u32,u32` |
| `0x0187` | inline: `(no calls)` | UIStarPlanetQueue | MasterPieceTestRewardResult |  |
| `0x0189` | inline: `FUN_1406e8c20` | StarPlanetRoundInfo | BeautyCouponTestRewardResult | `u32` |
| `0x018A` | `FUN_142d4ff70` | StarPlanetResult | NickSkillExpired |  |
| `0x018B` | inline: `FUN_1406e8c20 FUN_140233d50` | BackSpeedCtrl | RandomMissionResult | `u32,u32` |
| `0x018C` | `FUN_142cd94e0` | SetMazeArea | 12TresureResult |  |
| `0x018D` | inline: `FUN_1408a9e40 FUN_14019a260 FUN_1415eca30` | CharacterBurning | 12TresureJumpHigh |  |
| `0x018E` | inline: `FUN_142128dc0` | BattleStatCoreInfo | ItemCollection_SetFlag | `u8` |
| `0x018F` | inline: `FUN_1406e8ae0` | BattleStatCoreAck | ItemCollection_CheckComplete |  |
| `0x0190` | `FUN_142db50f0` | GachaponRewardTestResult | ItemCollection_SendCollectionList |  |
| `0x0191` | `FUN_142d43be0` | MasterPieceTestRewardResult | ToadsHammerRequestResult |  |
| `0x0192` | `FUN_142d43cf0` | RoyalStyleTestRewardResult | HyperStatSkillResetResult |  |
| `0x0193` | `FUN_142d1ba10` | BeautyCouponTestRewardResult | InventoryOperationResult |  |
| `0x0194` | `FUN_142dd4f10` | NickSkillExpired | GetSavedUrusSkill |  |
| `0x0195` | `FUN_142ddaa60` | RandomMissionResult | SetRolePlayingCharacterInfo |  |
| `0x0196` | `FUN_142ddb8c0` | 12TresureResult | MVP_Alarm |  |
| `0x0197` | inline: `FUN_142267f60` | 12TresureJumpHigh | MonsterCollecion_CompleteReward_Result |  |
| `0x0198` | inline: `FUN_14113d920` | ItemCollection_SetFlag | UserTowerChairSettingResult |  |
| `0x0199` | inline: `FUN_14019bd40 FUN_14019c870 FUN_1404bf3c0` | ItemCollection_CheckComplete | NeedClientResponse |  |
| `0x019A` | `FUN_142d9ed20` | ItemCollection_SendCollectionList | CharacterModified |  |
| `0x019B` | `FUN_142d97560` | ToadsHammerRequestResult | TradeKingShopItem |  |
| `0x019C` | `FUN_142db7b80` | HyperStatSkillResetResult | TradeKingShopRes |  |
| `0x019E` | inline: `FUN_142d24480` | GetSavedUrusSkill | PlatFormar_Oxyzen |  |
| `0x019F` | `FUN_142d968f0` | SetRolePlayingCharacterInfo | MacroSysDataInit |  |
| `0x0275` | inline: `FUN_1406e8ae0 FUN_1406e8c20` | - | - | `u8,u32` |
| `0x039A` | inline: `FUN_1406e9170 FUN_1406e8c20` | - | - | `raw,u32` |


## Swordie packet shapes, for checking the client's read order against

**Reference implementation, v214-era Swordie. Field order is a hypothesis about mscw, not a
fact about it.** The value in these is that field *order* survives version changes far
better than opcode numbers do.

### `Stage.setField` - `net/swordie/ms/connection/packet/Stage.java:21`

```text
FT      current time                    8 bytes
u8      unk (false)
u32     channelId - 1
u8      dev (false)
u32     oldDriverID
u8      characterData ? 1 : 2           // 1 = full entry, 2 = a plain map change
u32     characterData ? 0 : fieldType
u32     field width
u32     field height
u8      characterData
u16     notifierCheck (0)
        // if notifierCheck > 0:  str, then notifierCheck x str

// --- characterData == true (this is the migration case) ---
u32     damage-calc seed 1
u32     damage-calc seed 2
u32     damage-calc seed 3
        <Char.encode(All)>              // see below
u8      bool1 (true)
        u8   bool2 (false)   [ u32 if bool2 ]
        FT   MIN_TIME
        u8   0
        FT   MIN_TIME
        FT   MIN_TIME
        u32  -1

// --- characterData == false (a portal warp inside the channel) ---
u8      usingBuffProtector
u32     field id
u8      portal
u32     hp
u32     0
u8      hasPosition   [ u32 x, u32 y ]

// --- both paths rejoin ---
u8      bool (false)                    [ u32, u32 if set ]
u8      2
u8      0
u32     mobStatAdjustRate (100)
u8      hasFieldCustom                  [ FieldCustom.encode ]
u8      false                           // CWvsContext::OnInitPvPStat
u32     0
u8      false
        <extraTMSSystem buffer>
u8      isExtendSpJob
u64     0
u32     -1
u8      stackEventGauge > 0             [ u32 stackEventGauge ]
        // vonbon fields only: u8 size, then size x str
u32     0                               // CUser::DecodeTextEquipInfo
        <FreezeHotEventInfo>            // CUser::DecodeFreezeHotEventInfo
u32     eventBestFriendAID
u8      bool (true)
        u32 -1, u32 0, u32 0, u32 999999999, u32 999999999, str ""
u32     damageSkinID
str     ""
str     ""
str     ""
        <SunnySunday>
u32     0                               // UI_OPEN(%d)
u32     0
u8      true
u32     0
u32     0
        [ u8 0 if the leading unk was set ]
u32     size (0), then size x u32
        <customNickName>
```

`Char.encode(outPacket, DBChar.All)` - `client/character/Char.java:1992` - opens with a
shape worth knowing because it is unmistakable on the wire:

```text
100 x u8  all 1                           // a 100-byte all-ones prefix
u8        combat orders
3 x u32   pet cool times, -2 when absent  // MAX_PET_AMOUNT
u8 0, u8 0, u32 0, u8 0
          <CharacterStat.encode>
u8        friend record count
u8        hasBlessingOfFairy   [ str ]
u8        hasBlessingOfEmpress [ str ]
u8        false                          // ultimate explorer, deprecated
u16 0, FT MIN_TIME
u32       DAMAGE_SKIN_MAX_SIZE, then damage skins
u64       money
u32       expConsumeItems count, then records
u32       shop-buy-limit count, then 5 x u32 + 2 x u64 each
6 x u32   inventory slot counts (equip, consume, install, etc, cash, decoration)
u8 0, u8 0, u8 0
          equips, then consume / install / etc as (u16 bagIndex, item) lists each
          terminated by u16 0
```

### What the reference server sends after a migrate-in

`MigrationHandler.handleMigrateIn` (`handlers/user/MigrationHandler.java:45`) reads
`u32 worldID, u32 userID, u32 charId, 16 raw bytes machineID`, then does no I/O until it
calls `chr.warp(field, true)`. `Char.warp` (`Char.java:4801`) is the whole reply sequence:

1. **`Stage.setField(..., characterData = true, ...)`** - the packet above. This is the one
   that ends the client's wait.
2. `showProperUI(oldFieldId, newFieldId)`
3. `UserRemote.receiveHP` per party member, both directions, if in a party
4. **`WvsContext.partyResult(PartyResult.load(party))`** - sent unconditionally, even with
   no party
5. `FieldPacket.setAchieveRate` / `FieldPacket.practiceMode`, instances only
6. `WvsContext.guildResult(...)`, guild members only

then, **1500 ms later**, `Char.initialize()` runs and opens with
`WvsContext.hourChange(dayOfWeek, hour)`.

So the reference server's minimum answer to a migrate-in is **one packet**: `SetField` with
`characterData = true`. Everything else is conditional or delayed.

## What this file cannot bear

* **`0x0070` is the only name in the table checked against mscw.** Unanchored, the method
  scores 1 of 9 across both controls, and that one hit was the block's first entry, which
  any alignment gets for free.
* The A/B split is a **ten-name shift**, not a small perturbation, and the crossover between
  them is not located. Reading a name off a row between `0xb3` and `0x121` and building on
  it has roughly a coin-flip's chance of being ten entries wrong.
* **`SetField = 0x01A0` is a candidate, not a finding.** It rests on the block ending where
  the switch's cases end, which is an inference, not a measurement. Budget for `0x01A0
  .. 0x01AA` and test the handler's opening read (8 raw bytes) before building on it.
* The `0x71..0x7a` hole is *not* an artifact of case extraction - the corrected,
  brace-depth-aware table still shows it - but it has not been proven those ten slots are
  absent from the enum rather than present and ignored. Either reading supports alignment B.
* mscw's enum contains at least one packet (`BossFirstClearRecord`, case `0x18b`) that
  neither reference names, so the alignment is not even a subsequence relation.
* Ghidra handler sizes are used above as an argument. `STATUS.md` records that this binary
  regularly gets function boundaries wrong; a 188,565-byte "function" is more likely several
  merged than one enormous handler.
