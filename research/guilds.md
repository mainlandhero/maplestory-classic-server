# Guilds: every entry point ships, the window does not

2026-08-28, the owner: *"The guild thing looks very interesting, are there anything regarding
guilds in this client?"*

**Yes - a lot, and it is the Shop2 shape again.** The client carries guild code, guild GM
commands, guild chat options, a guild button on the status bar and a tab that literally says
**Guild**. What it does not carry is the guild window's art. Anything that makes the client
try to open it is in the same position as the Buy Back tab that crashed it on 2026-08-27.

## The tab says Guild and there is no panel behind it

`UI/UI_000.wz` `UserList.img` - the Friend/Party window - has a four-tab strip and **three**
sub-windows:

```text
tab:mainTab   normal/0..3 and selected/0..3      four tabs
top-level     backgrnd, layer:backgrnd2, textBox, tab:mainTab,
              vector:childwnd_lt, vector:childwnd_rb, number,
              Buddy, Party, Blacklist            three panels
```

The tab labels are bitmaps, not strings, so they were rendered rather than guessed
(`wz-dump canvas` on `_Canvas_000.wz`, then `tools/wz_png.py`):

| tab | label | panel node in `UserList.img` |
|---|---|---|
| 0 | Buddy | `Buddy` |
| 1 | Party | `Party` |
| 2 | Blacklist | `Blacklist` |
| 3 | **Guild** | **missing** |

Note the ordering: the *bitmaps* run Buddy, Party, Blacklist, Guild, so Guild is tab **3**,
not the index 2 that classic MapleStory uses. Read the images, not the convention.

**The instrument has a positive control.** The same search over `UserList.img` that fails to
find `"Guild"` does find `"Buddy"`, `"Party"` and `"Blacklist"` - so the absence is the file's,
not the grep's.

Widening it: **782 images across 17 archives**, and the only image in the whole install whose
name contains "guild" or "alliance" is `Etc/guildCommon.img`, which is a stub -

```json
{ "skill_cooltime_minute_by_join_date": 1440 }
```

one number, no member list layout, no emblem art, no mark backgrounds.

## What *does* ship, and it is nearly everything except the window

Scanning all 67 UI images for a guild or alliance key:

| image | node | what it is |
|---|---|---|
| `StatusBar3.img` | `button:guild`, `button:GuildCastle` | the status bar's guild buttons |
| `StatusBar3.img` | `check:talkGuild`, `check:inviteGuild`, `button:talkGuildColor` | guild chat and invite toggles, and their colour swatch |
| `StatusBar3.img` | `check:talkAlliance`, `check:inviteAlliance`, `button:talkAllianceColor` | the alliance equivalents |
| `ContextMenu.img` | `BtGuild` | right-click a player -> guild action |
| `CharacterInfo.img` | `vector:guildName` | where a guild name is drawn under a character |
| `UIToolTip.img` | `Guild` | a guild tooltip frame |
| `Login.img` | `cannotDeleteGuildmaster` | the notice that stops a guild master deleting their character |

`research/cash-shop.md` already recorded `GuildCastle` as **"0 code reference(s),
StatusBar3.img only"**, which is the same observation from the other side.

In the executable:

* **RTTI classes** - `CGuildCommonDataMan`, `CGuildRankingDlg@UI@Guild`,
  `CGuildAttendCheckDlg@UI@Guild`, `CSearchDlg@Guild`, plus `CAllianceSkillMan` and
  `CAllianceUIDataMan`. (Only `--list` is usable; `--vtable` cannot resolve a MapleStory
  class in this image - see `tools/rtti.py`.)
* **~20 GM commands**: `/guild`, `/GuildUI`, `/guildname`, `/guildid`, `/leaveGuild`,
  `/noGuild`, `/guildMark`, `/setGuildMark`, `/guildSkill`, `/setguildskill`,
  `/useguildskill`, `/guildbattleskillreset`, `/guildContents`, `/incGuildContentsPoint`,
  `/setGuildContentsPoint`, `/setGuildContentsLastRank`, `/reloadGuildContentsLastRank`,
  `/guildRankNextSeason`. These are **outbound** - the client asks, a server has to answer.
* **Local settings**: `goGuildINVITE`, `goGuildTALK`, `fontColorGuild`, and the alliance
  pair, in the same block as the sound and graphics options.
* **Messages**: `0x00CB` *"Guild Know-How Bonus EXP (+%lld)"* (`research/client-messages.md`),
  and message subcase **7** is guild points - `0x9B0` *"You have earned Guild Points (+%d)"*
  (`research/message-subcases.md`).
* WZ item/skill fields `reqGuildLevel`, `expGuild`, `psdEXPGuild`.

## Half the guild feature was never on the game socket at all

These are **not** packets and chasing them is chasing a web service:

```text
CNMGetMyGuildListExFunc     CNMDownloadGuildMarkFileFunc
CNMGuildChatMessageEvent    CNMGuildOnlineInfoEvent / ExEvent
CNMMonitorGuildOnlineInfoFunc / ExFunc
```

with JSON field names beside them - `guildlistcnt`, `guildmembercnt`, `guildmembers`,
`guildmemberaft`, `guildmarkimage`, `guildmarkcustom`, `guildmaster`, `guilddestroytype`,
`targetguildid`, `recommendguildid`, `advertisedguildflag`, `guildStandAlone`.

`research/protection-surface.md` already names the other half: **`WzMss.dll` - SOAP web
services (guild boards, notices, consult)**. So guild chat, the member list, the mark image
and the guild board go over HTTP to Nexon endpoints this install will never reach, and no
amount of work on our socket substitutes for them.

This is the single most important thing on this page. It is the same mistake as
`Custom_KeySettingChange`: a guild-shaped string is not evidence of a guild-shaped packet.

## Candidate opcodes, and why they are only candidates

`research/msexe-gamestage-opcodes.md`'s column B puts guild traffic at `0x00A7`
(GuildRequest), `0x00A8` (GuildResult), `0x00A9` (AllianceResult), with `0x00B2`/`0x00B3` as
the other column's guess. Read shapes from `tools/reads.py` over the switch:

| opcode | handler | reads |
|---|---|---|
| `0x00A7` | `FUN_142defd40` | 37 fields, 11 of them strings |
| `0x00A8` | `FUN_142de58b0` | 5: `raw,u32,str,u32,u32` |
| `0x00A9` | `FUN_142ddfd10` | **165 fields**, heavily multiplexed |
| `0x00AC` | `FUN_142d60d40` | 36 fields; `research/talking-back.md` already calls it "party/guild/friend-shaped **[I]**" |

Consistent with a guild result, and consistent with several other things. **The column-B
names are `[I]` and that whole table scores 1 of 9 on its own controls** - `CLAUDE.md` is
explicit that a table row written from a quick read is a claim. Nothing here is measured.

## What this means in practice

**Do not build toward opening the guild window.** Every path that would show it - the fourth
UserList tab, `button:guild`, `BtGuild` - ends at art that is not in the install, and this
client's response to that has already been measured once: `0x055E` type 10 selected a tab
`UI/UIShop.img/Shop` does not have and **killed the client**. `research/npc-shop-crash2.md`
lists Trunk, MemberShop, MiracleCube, AP/SP Reset, ExOptTransfer and GuildBoard together as
"none reachable in this client", and the guild window belongs on that list.

What *is* reachable, and would show on screen without opening any missing window:

* **A guild name under the character.** `CharacterInfo.img` has `vector:guildName` and the
  character info window already works. This needs the name in whatever packet carries it -
  unknown, and worth one grep of the character record before assuming it needs a new opcode.
* **Guild chat as a chat channel.** `check:talkGuild` and `fontColorGuild` are status-bar
  state; guild chat is a colour and a prefix on a line the chat window already draws. But
  `CNMGuildChatMessageEvent` says the real client routes it through the web service, so
  check which path this client actually takes before building it.

Both are small. Neither is worth starting before the two questions above are answered, and
both answers are static - no client run.
