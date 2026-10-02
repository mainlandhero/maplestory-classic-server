# The client's options live on the server: `0x02EB` out, quest ex records in

2026-10-02. Static, off `client-patched\MapleStory.exe` and the Ghidra project
(`research/msexe-client-options.c`), plus the live server's own captures and the client's
registry. **No client run.**

The owner, from player complaints: *"Saving of settings such as audio, HP thresholds, etc on server
side per account"*, and *"HP thresholds means pet auto pot HP thresholds"*.

Marks: **[L]** read off a listing, a decompilation, the PE bytes or a capture. **[D]** derived.
**[I]** inferred.

## 1. The answer

* The client keeps its options in an object with **two `std::map<int,int>` groups**, group 0
  at `+0x408` and group 1 at `+0x418` (each with a shadow at `+0x428` / `+0x438`).
  `FUN_140426d40(group, key)` is the "the server holds this" test: group 0 keys `< 0x41`,
  group 1 keys `< 0x15`. **[L]**
* **Every change goes out as `0x02EB`**: `u32 group, u8 count, count x (u32 key, i32 value)`.
  Six builders (`FUN_141601390` whole group, `141601440` one key, `1416018a0` two keys,
  `141601d10`, `141601dc0`, `1411f4f70`). **[L]** Every archived body is exactly `5 + 8n`:
  13, 21, 109, 173, 389, 397, 405 bytes. **[L]**
* **They are read back at field entry from quest ex records**, not from any options packet:
  * group 1: `FUN_1415f7140` → `FUN_141604660` → `FUN_140426ed0`. That turns the key into its
    name (`FUN_140427040`, table `0x143287b10`), then looks it up in quest **101563**, then
    **101790** (`DAT_1432874e8`) through `FUN_1402E01C0`, which is the character's `+0x12BB`
    map: record block #28, the citizenship one. **[L]**
  * group 0: `FUN_140426d60` does the same with names from `FUN_140427230` (table
    `0x1432874f0`) in quests **368, 369, 370, 481** (`DAT_1432874d8`) through `FUN_1402E0430`,
    which is the `+0x12D3` map: record block #32, presence byte 19. **[L]**
  * The value is the record's `key=value;` text, and the first quest that has the key wins.
    **[L]**
* Both readers run from `0x141818ae0` (field entry), after the SetField record has been decoded.
  **[L]** for the call; **[D]** that the record's maps are filled by then.

## 2. Why nothing persisted

**The registry does not hold them.** `HKLM\SOFTWARE\WOW6432Node\Wizet\MapleStoryClassic` on a
machine that has played for weeks has:
* one `g0_<world>_<charId>` key per character, holding window positions, quest alarms and
  auction filters;
* at the root: screen mode, resolution, `soBGMVol`, `soBGMMute`.

There is no `soHPFlash`, no `soAutoConsumePetHP` and no effect volume anywhere. **[L]** Those
are server-held by design, the server never stored them, so every field entry read nothing and
fell to the default (`flHP` 10).

## 3. The names [L]

Group 1, game options, quest 101563:

```text
00 alWh  01 chFr  02 alMe  03 alExch  04 shMd  05 shNk  06 soulUI  07 alPa  08 alGu  09 alAl
0a chGu  0b chAl  0c alFr  0d frOnNot 0e soErUI 0f acpHP 10 flHP   11 flMP  12 alFol 13 bufAual
14 bufMin   (15 = COUNT)
```

Group 0, system options, quests 368/369/370/481:

```text
00 vBG1 01 mBG1 02 vE1 03 mE1 04 mAnd1 05 vSE1 06 mSE1 07 vSV1 08 mSV1 09 vM1 0a mM1 0b vME1
0c mME1 0d fLoc1 0e fType1 0f sAuto1 10 sCon1 11 sNum1 12 mobInf1 13 qsEff1 14 damEff1 15 vSync1
16 fSize2 17 fcW2 18 fcF2 19 fcG2 1a fcA2 1b aMine2 1c aOther2 1d magUI1 1e trem2 1f aUI2
20 pBack2 21 simItm2 22 cmbMsg2 23 avMega2 24 chPos2 25 chTime2 26 petHP2 27 qsTime2 28 wrnHP3
29 wrnMP3 2a wndHtK1 2b..35 gq*3 36 damAmt3 37 silBo3 38 silTy3 39 silTh3 3a mlUpAc2 3b mlUpTy2
3c aMyPet 3d aUrPet 3e aMyAnd 3f aUrAnd 40 infItm   (41 = COUNT)
```

* **The pet's auto-potion threshold is `flHP` / `flMP`.** `FUN_1415f7140` reads `soHPFlash` as
  `FUN_141604660(.., key 0x10, fallback 0x28 = wrnHP3, .., default 10, min 0, max 0x13)`, and
  likewise `soMPFlash` with 0x11 / `wrnMP3`. **[L]** That this is the threshold Auto HP drinks
  at is **[I]**: it is the modern client's HP-warning setting, and the archived pair values
  (2..16) sit in its 0..19 range.
* `acpHP` (0x0f) is read as `soAutoConsumePetHP`. **[L]**

## 4. What the server does (`net::clientsettings`, `session/options.rs`)

* Stores each `(group, key) = value` per **account** (`store::clientsettings`).
* At every SetField:
  * group 1 goes in block #28 as quest 101563 = `flHP=7;flMP=3;...`;
  * group 0 goes in block #32 (presence 19), one quest per name suffix (`...1` → 368,
    `...2` → 369, `...3` → 370, the rest → 481).
* **Block #32 is after the record's final ungated byte.** The decoder listing has
  `0x140308b3f READ u8` (ungated), then gates at `b63, be0, cf3, d31, d79, dba, df3, e40`, all
  presence bytes this server never sets. Then the gate at `0x140308e79` reads `u16` at `e95`,
  `u32` at `ea4` and `str` at `eb2`, and calls `FUN_1402e1a20`. **[L]** So the block is
  appended after that byte.
* **Never on a wire when it shipped.** `--system-options off` (`start-server.ps1
  -NoSystemOptions`) drops block #32 alone if a login fails to decode.

## 5. What would refute this

* A client that does not keep the HP warning after a relog with block #28 carrying
  `flHP` → the reader is not reached at field entry, or reads a different quest.
* A login that fails to decode only when an account has system options → the block #32
  placement is wrong. `--system-options off` is the test: it must make that login work.
