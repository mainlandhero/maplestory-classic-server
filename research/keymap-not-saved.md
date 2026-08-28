# The keyboard layout is not saved, and nothing on either side ever tried to save it

2026-08-28, the owner: *"The client should be able to save their skills on keyboard layout. Upon
logout then subsequent login, this customization is completely gone and went unsaved. I've
tried this a few times in past runs."*

## The answer

**Not implemented, on either half.** This is not a bug in something that exists; there is no
code anywhere in the workspace that has ever touched a key mapping.

| half | what it would be | what exists |
|---|---|---|
| save | receive the client's key-change packet, write it to the database | nothing |
| restore | send the stored mapping back at field entry so the client can apply it | nothing |

Measured, not inferred:

* `keymap|key_map|keybind|funckey|func_key|quickslot|quick_slot`, case-insensitive, over the
  whole repository, matches **three files and all three are prose** - `STATUS.md`,
  `research/cash-shop.md`, `research/msexe-gamestage-opcodes.md`. **Zero matches in any of
  the seven crates.**
* The schema has 14 tables (`accounts`, `sessions`, `characters`, `equipment`, `inventory`,
  `quest_state`, `character_skills`, `character_skill_spend`, `storage`, `storage_item`,
  `cash_wallet`, `cash_locker`, `server_rates`, `migrations`). None of them stores a key
  mapping, and no migration adds one.

## The client does not keep it locally either, which is why the server has to

The client's local settings block is readable as plain ASCII in the image, one
null-terminated name per entry, at file offset ~53 118 400. Enumerated in full it is
sound, graphics, chat and UI options:

```text
goWhisper goFriend goMessenger goExchange goSoulMP goParty goGuildINVITE goAllianceINVITE
goGuildTALK goAllianceTALK goFriendInvite FriendOnlineNotice solErdaMiniUIOnOff goBuffAlign
goBuffMin goFollowRequest soHPFlash soMPFlash soAutoConsumePetHP soSEVol soSEMute
soAndroidMute soSSEVol soSSEMute soVoiceVol soVoiceMute soMasterVol soMasterMute soMSEVol
soMSEMute soScreenShot soShotFormat soShotAuto soShotConti soShotContiCount soMobInfo
soQuickSlotNum soDamageEffect soVSync fontSizeType fontColorWhisper fontColorFriend
fontColorGuild fontColorAlliance goMySkillAlpha goOtherSkillAlpha magUI cEnable cSense
cDpad soTremble uiAlpha soPetBackword soCombatMessage soAvatarMegaphone soPopupUIChatWnd
soShowChatTimeStamp soQuickSlotEffect UseWindowedHotkey GraphicQuality_*
```

**`soQuickSlotNum` is the number of quick-slot rows, not their contents**, and there is no
entry for a key mapping anywhere in the block. So the client has nowhere local to put it.

That matches the screen, which is the stronger evidence of the two: the owner has watched the
customisation vanish across a relog several times. Per `CLAUDE.md`, the screen wins.

## The client has never sent us one - in 162 distinct captures

`crates/world/src/server.rs:144` writes the `<- opcode, N byte body` line **before**
`session.handle`, so an opcode the server has no handler for is still logged in full. The
instrument cannot be silent about an unknown packet; verified by reading the call site, and
by the 40-odd `UNKNOWN` opcodes that do appear in the census.

Deduplicated by content hash - 168 files in `previous-runs/` and `research/fixtures/` are
**162 distinct captures** - the client's entire inbound repertoire is 90 opcode/label pairs,
and every one of them is either named or accounted for by an existing investigation.
Nothing has a key-mapping shape:

* no body of `4 + 9n` or `8 + 9n` bytes on any unexplained opcode
* the rare opcodes - the ones that appear in six runs or fewer - are all identified: login
  stage (`0x0071`, `0x0073`, `0x007A`, `0x0080`, `0x00A1`, `0x00A6`, `0x00BF`, `0x00C0`),
  party (`0x0182`), cash shop (`0x03E0`, `0x03E1`), buff cancel (`0x013F`), shop (`0x00F5`)
* `0x0070` cannot be hiding it: inbound it is only ever 45 or 46 bytes

**The one caveat, and it is the whole caveat:** nobody has recorded *changing a key during a
captured run*. So this is "no keymap packet was ever captured", which is a weaker claim than
"the client does not send one". `CLAUDE.md` is explicit that those are different sentences.
See the measurement below, which settles it for free.

## No handler in the channel stage's switch decodes a key table

`FUN_142cbaa80` has 273 labels / 272 bodies; **179 have an out-of-line `FUN_` handler** and
`tools/reads.py` at depth 2 was run over every one of them. A `FuncKeyMappedInit` would read
a count and then loop over `(u8 type, u32 action)` pairs, so the shape to look for is
`u32,u8,u32` or `u8,u32`. Eight handlers match, and all eight were read:

| opcode | handler | what it actually is |
|---|---|---|
| `0x0084` | `FUN_142d58bb0` | two fields, no loop |
| `0x008B` | `FUN_142cd8bd0` | id -> user-pool lookup (`FUN_1429b5cf0`), then two setters. No loop |
| `0x00B6` | `FUN_142d94670` | two fields, no loop |
| `0x00FB` | `FUN_142cf22d0` | two fields, no loop |
| `0x010B` | `FUN_142da1140` | two fields, no loop |
| `0x012D` | `FUN_142d95480` | a UI result: message id `0x1252`, branches on -1/0/1 |
| `0x0172` | `FUN_142db6e10` | gated on skill id `0x188b4` = 100532 |
| `0x018C` | `FUN_142cd94e0` | two fields, no loop |

**The blind spot, stated rather than hidden: 94 of the 273 cases have inline bodies** and the
sweep did not cover them. The case table lists at most three calls for an inline body, so it
cannot rule them out either. This negative therefore covers 179/273 of the switch and no
more.

## Two instruments to distrust

* **`tools/rtti.py --vtable` returns "0 locator(s)" for every class**, including
  `CLoginQueueDlg` - the class its own docstring names as the example. It is structurally
  incapable of returning a vtable in this build. `--list` works and was used above. This is
  the `CLAUDE.md` positive-control rule paying off: the empty answer looked like a finding.
* The client has **no `KeyConfig` or `FuncKey` string** and **no `.img/...Key...` UI path**
  in the image, so neither is a way in. It does have `Custom_KeySettingChange` and
  `Custom_KeySettingSnapshot` - but those sit in the block with `Custom_ClientDevLog` and
  `Custom_PrivateServerLog`, next to `registerKeymapInfo` / `enqueueJsonBodyUserLogEx`, so
  they are **Nexon telemetry event names, not game packets**. Chasing them would be chasing
  a log upload.
* RTTI has no `CFuncKeyMappedMan` and no `CUIKeyConfig`; it does have `CUIQuickSlot`,
  `CUIStatusbar`, `KeyProcessor` and `CSequencedKeyMan`. 1764 descriptors, so absence here
  means "not polymorphic", not "not present".

## The measurement that settles it, and costs no launch of its own

Ride it along on any run:

> Open the keyboard-settings window, **drag one skill onto an empty key**, close the window.
> Then log out and back in.

Then, on the run's `world.log`:

```text
python - <<PY
# opcodes in this run that appear in no other archived run
PY
```

or simply diff the opcode census of that run against the 90-pair repertoire above.

| outcome | what it means | what to do |
|---|---|---|
| a **new opcode** appears at the moment the window closes | the client does report key changes. Its body is the layout, and the same layout inverted is the restore packet | implement both halves; the save half is then measured, not guessed |
| **no new opcode** | the client never volunteers it, so the mapping can only arrive from the server | the save half needs a different trigger found statically, and the restore half needs the 94 inline cases swept |

Write down which one happened. Both are useful and they need opposite work - this is the
same "separate *this thing* from *this session*" discriminator that `CLAUDE.md` records
paying twice.
