# Buffs: `0x013C` is the skill-use request, `0x007D` is the grant, and CTS bit 92 is Speed

The owner, 2026-08-22: *"I tried activating Nimble Feet, but the server did not give me the Nimble
Feet buff for the duration that the skill indicated (30 seconds)."*

Labels are the project's: **[L]** read off this client's listing, its WZ, or a capture ·
**[D]** derived from two or more [L] facts · **[I]** inferred, not established. There is a
["What I did NOT establish"](#what-i-did-not-establish) section and it names its own blind
spots. **No Ghidra** (another agent holds the lock), **no `cargo`**, **no client run.**

Listings dropped beside this file:

| file | what |
|---|---|
| `research/msexe-tempstatset-142d563d0.txt` | the **`0x007D`** handler - `CUserLocal::OnTemporaryStatSet` |
| `research/msexe-secondarystat-140a165f0.txt` | `CSecondaryStat::DecodeForLocal` - the 124-byte mask, plus a census of all 476 stat blocks |
| `research/msexe-secondarystat-remote-140a46e50.txt` | the remote sibling, same mask, short list |
| `research/msexe-speed-is-cts92-1429755a0.txt` | **the proof that CTS bit 92 is Speed** |
| `research/msexe-sendcensus-142d19260.txt` | the `0x013D` builder |
| `research/msexe-sendcount-142d19070.txt` | `m[opcode][skillId]++`, the thing `0x013D` reports |

---

## 0. The short version

| question | answer |
|---|---|
| What does the client send when a buff skill is used? | **`0x013C`**, once, 51 bytes. `u32 skillId, u32 skillLevel, u32 tick, u32 crcLevel, u32 crcSkill, …, u16 x, u16 y, …` **[L]** |
| Then what is `0x013D`? | **Not a buff packet at all.** It is a 30-second anti-cheat census: *"since the last reset I sent opcode X with skill-id Y this many times."* Proven by an exact count match against `world.log`. **[L]** |
| Direction of `0x013D`? | Client → server, **reporting**. It is built from a map the client increments on **send**, in `FUN_142d19070`. **[L]** |
| Which packet grants a temporary stat? | **`0x007D` TemporaryStatSet.** Body: `raw[124]` bit mask, then per set bit `{ value, u32 reason, u32 duration }`, then a 13-or-14-byte tail. **[L]** |
| Which bit is Speed? | **bit 92**, decimal. The client's own refusal *"Nimble Feet cannot be used while another Speed increase effect is active"* is raised 59 bytes after it reads `secStat+0x5cc`, and `+0x5cc` is exactly what the decoder's setter for bit 92 writes. **[D], from two [L] halves** |
| Duration unit | **milliseconds.** The client stores `tExpire = tick + duration` and the tick is milliseconds, measured off the wire. **[D]** |
| What Nimble Feet does | `speed +10` at every level; `time` = **10 / 20 / 30 seconds** at level 1 / 2 / 3; `mpCon` 4 / 7 / 10; `cooltime` 180 s. **[L]** |
| Why nothing happened | The server has **never sent `0x007D`**, and `0x013C` is logged `UNKNOWN` and dropped. |

---

## 1. The brief's premise was a coincidence, and checking it was free

The brief said:

> `0x013D` arrives EXACTLY every 30 seconds … **Nimble Feet's duration is 30 seconds.** That
> is the whole reason to look here.

Both halves are true and the connection between them is not. From `world.log` of the
2026-08-22 02:51 run: **[L]**

```text
02:50:28.826 <- 0x013D  9 bytes  01 00000000 00000000
02:50:59.146 <- 0x013D  9 bytes  00 00000000 00000000
02:51:11.157 <- 0x013B  skill 1002 raised 0 -> 3        <- Nimble Feet learned HERE
02:51:12.248 <- 0x013C  51 bytes, skill 1002 level 3    <- Nimble Feet USED here
02:51:29.171 <- 0x013D  25 bytes … ea030000 …           <- 1002 first appears
```

The 30-second tick is **30.32 s and 30.03 s apart before the skill was learned**, with an
empty body. It is a heartbeat that was already running. Nimble Feet's level-3 `time` being
30 s is a second, unrelated 30.

This is `CLAUDE.md`'s *"the thing you are comparing against may never have been a control"*
again, and one `grep` of the log that was already on disk settled it.

---

## 2. `0x013D` decoded in full - it is a send-counter census

### 2.1 The builder

`FUN_142d19260`, the single builder for opcode `0x13D`
(`research/msexe-send-opcodes.txt` line 380). Encode calls, in order, from the listing: **[L]**

```text
142d192d0  mov edx,0x13d ; COutPacket ctor
142d192e6  w_u8 (bl)                       ; 1 if the counter map was reset this call
142d19305  w_raw4 [user+0x4068]            ; std::map::_Mysize  = number of opcodes
           for each node of the std::map<int, std::map<int,int>> at user+0x4060:
142d1932e    w_raw4 node+0x20              ;   pair.first  = the OPCODE
142d19349    w_raw4 node+0x30              ;   inner map _Mysize = number of skill ids
             for each node of the inner std::map<int,int>:
142d1936e      w_raw4 inner+0x1c           ;     pair.first  = the SKILL ID
142d19381      w_raw4 inner+0x20           ;     pair.second = the COUNT
142d197ea  w_raw4 [rsp+0x58]               ; _Mysize of a second, locally built map
           for each node of it:
142d1980f    w_raw4 inner+0x1c             ;   opcode
142d19822    w_raw4 inner+0x20             ;   value
142d198ed  SendPacket
```

`node+0x20` / `node+0x30` and `inner+0x1c` / `inner+0x20` are the MSVC `std::map` node
layout (`_Left,_Parent,_Right` at 0/8/0x10, `_Color,_Isnil` at 0x18/0x19, then `_Myval`), and
`_Myhead`/`_Mysize` at `user+0x4060`/`+0x4068` is the map object itself. **[D]**

### 2.2 The layout

```text
u8    reset          1 if the counter map was cleared on this tick
u32   opcodeCount
opcodeCount x {
    u32  opcode
    u32  entryCount
    entryCount x { u32 skillId, u32 count }
}
u32   decreasedCount
decreasedCount x { u32 opcode, u32 total }
```

Checked against both observed shapes, exactly: **[L]**

```text
25 bytes  00 | 01000000 | 3c010000 01000000 | ea030000 01000000 | 00000000
          ^    1 opcode   opcode 0x13C, 1 entry   skill 1002 x1     no decreases

41 bytes  00 | 02000000 | df000000 01000000 | 00000000 07000000
                          opcode 0x0DF, 1 entry  skill 0    x7
                        | 3c010000 01000000 | ea030000 01000000 | 00000000
                          opcode 0x13C, 1 entry  skill 1002 x1     no decreases
```

**`0x13c` = 316 is the opcode `0x013C`, not a stat and not a duration.** The brief's
"`0x13c` = 316 appears in both and is unexplained" is explained.

### 2.3 The control that makes this a measurement, not a reading

`0x00DF` is `CLIENT_MELEE_ATTACK`. The 41-byte body claims **7**. Counting the log: **[L]**

```
$ grep -cE "^[0-9:.]+ <- 0x00DF" world.log
7
$ grep -cE "^[0-9:.]+ <- 0x013C" world.log
1
```

Seven melee attacks, all before 03:00:31 when the 41-byte body first appears; one skill use.
Both numbers land in the packet. The inner key for `0x013C` is **1002**, the skill id; for
`0x00DF` it is **0**, a plain swing with no skill.

### 2.4 What increments it

`FUN_142d19070(user, opcode, skillId)` - `research/msexe-sendcount-142d19070.txt`: **[L]**

```text
142d190eb  lea rcx,[rbx+0x4060] ; FUN_1411f6df0(&map, &opcode)   -> map[opcode]
142d19143  FUN_14028c520(inner, &skillId)                        -> inner[skillId]
142d1914d  inc dword [rax]                                       -> ++
```

and it counts **only** these opcodes, from its own jump table at `0x142d19184` plus the
compare chain: **[L]**

```text
0x00DF 0x00E0 0x00E1 0x00E2 0x00E3 0x00E4   (the attack family)
0x013C                                      (skill use)
0x0141  0x0247  0x0273
```

Everything else returns without counting - which is why `0x013B`, `0x01A5`, `0x00D9` and
`0x032C` never appear in the census. The whole thing is gated on
`FUN_14090d2c0(0x1AE)`; when that returns true the maps are wiped
(`FUN_142d199c0`) instead.

### 2.5 So the server should not answer it

**The server must not reply to `0x013D`.** It is telemetry with no reply builder. Measured:
**22 unanswered `0x013D`** and one unanswered `0x013C` across a ten-minute session, and the
client kept playing to the end of the log. **[L]** - so these two are not in the class
`CLAUDE.md`'s *"always answer"* rule is about.

What it *is* good for: a free desync check. If our records say the character cast skill S
`n` times and the census says something else, one of us is wrong.

---

## 3. `0x013C` - the skill-use request

One sample, 02:51:12.248, 51 bytes. **[L]**

```text
ea030000 03000000 8279d911 8a326c9d d78ec0e4 00 01000000 00000000 00000000 6206 8702 00
80e8da8f 01000000 00 ffffffff
```

`0x013C` has **sixteen** builders (`research/msexe-send-opcodes.txt`); six of them share a
header writer, **`FUN_14073aa60`**, which writes exactly this, off a 0x2D-byte struct: **[L]**

| offset in struct | wire | value seen | reading |
|---|---|---|---|
| `+0x00` | `u32` | 1002 | **skillId** |
| `+0x04` | `u32` | 3 | **skillLevel** |
| `+0x08` | `u32` | `0x11d97982` | **client tick, milliseconds** |
| `+0x0c` | `u32` | `0x9d6c328a` | per-skill-level checksum |
| `+0x10` | `u32` | `0xe4c08ed7` | per-skill checksum |
| `+0x14` | `u8` | 0 | |
| `+0x18` | `u32` | 1 | |
| `+0x1c` | `u32` | 0 | |
| `+0x20` | `u32` | 0 | |
| `+0x24` | `u16` | 1634 | **x** |
| `+0x28` | `u16` | 647 | **y** |
| `+0x2c` | `u8` | 0 | |

Two independent checks that this is the right header and not a coincidence:

* **Position.** `+0x24`/`+0x28` read 1634 / 647. The `0x00D9` movement packets bracketing
  02:51:12.248 end at `62 06 87 02` = x 1634, y 647. **[L]**
* **The two checksums.** `+0x0c` = `8a326c9d` and `+0x10` = `d78ec0e4`. Both appear in
  `0x01A5` bodies for skill 1002 - `d78ec0e4` in the first list at 02:51:04.216 and
  `8a326c9d` in the second at 02:51:11.177. The client keeps a per-skill and a
  per-skill-level checksum and echoes them in the use request. **[D]**

The last 13 bytes (`80e8da8f 01000000 00 ffffffff` under one split) are the tail the specific
builder adds after the shared header. **Not resolved** - see
[what I did NOT establish](#what-i-did-not-establish).

**The server only needs `+0x00` and `+0x04`.**

`0x013C` is also the packet the client's own guard refuses to send. `FUN_1429755a0`, at
`0x142975989`, reads the Speed stat and its reason and raises string `0x14DA`
*"Nimble Feet cannot be used while another Speed increase effect is active"* if Speed is
already set from a different skill. **[L]** So a second cast during the buff never reaches us.

### `0x01A5` while we are here

Same family, unanswered, sent right after every `0x013B` skill-up: **[D]**

```text
u32 n1 ; n1 x { u32 skillId, u32 skillChecksum }
u32 n2 ; n2 x { u32 skillId, u32 level, u32 levelChecksum }
```

which divides all three observed bodies exactly (20, 28 and 48 bytes) and whose checksums are
the two dwords `0x013C` echoes. It is the client telling us what it thinks its skill levels
are. Useful later as a desync check; nothing needs it now.

---

## 4. What Nimble Feet actually is

`target/release/wz-dump.exe cat "client-patched/Data/Skill/Skill_000.wz" 000.img`, skill
`0001002`, canvases stripped: **[L]**

```json
"0001002": {
  "info": { "type": 10 },
  "level": {
    "1": { "mpCon": 4,  "time": 10, "speed": 10, "cooltime": 180 },
    "2": { "mpCon": 7,  "time": 20, "speed": 10, "cooltime": 180 },
    "3": { "mpCon": 10, "time": 30, "speed": 10, "cooltime": 180 }
  },
  "masterLevel": 3, "processtype": 6, "additional_process": { "0": 114 }
}
```

* **The stat it moves is `speed`, and it is `10` at every level** - only the duration scales.
  The client's string table has `Speed: +%d` (id 906), so it is a bonus, not an absolute. **[D]**
* **`time` is in seconds**: 10 / 20 / 30, and the owner said the skill "indicated 30 seconds" at
  the level they had, which is level 3. **[L] + the screen.**
* `cooltime` 180 is seconds too; the server should refuse a recast inside it. Note the
  client also refuses on its own (section 3), so a too-early recast will simply never arrive.

The two siblings, for completeness: `0001000` Three Snails is `type 2`, an attack
(`fixdamage` 15/25/40, consumes item 4000001/2/4). `0001001` Recovery is `type 31`,
`time` 30 s, `x` 4/8/12, `cooltime` 120.

---

## 5. `0x007D` - the packet that grants a temporary stat

### 5.1 It is `0x007D`, and there are three independent reasons

1. **The client's own case table.** `research/msexe-gamestage-cases.txt`, extracted from the
   channel-stage switch `FUN_142cbaa80`: `0x007d  FUN_142d563d0`. **[L]**
2. **Two confirmed anchors, one slot either side.** `0x007C` → `FUN_142d54780` is
   `StatChanged`, which this server already sends and which draws EXP and HP on screen;
   `0x0081` → `FUN_142d57f20` is `ChangeSkillRecordResult`, which this server already sends
   and which raises skill levels. `0x007D` and `0x007E` are the two slots between them, and
   the candidate table in `research/msexe-gamestage-opcodes.md` calls them `TemporaryStatSet`
   and `TemporaryStatReset`. **[D]**
3. **The handler itself.** `FUN_142d563d0` reads a 124-byte bit mask and then, per set bit, a
   value / a reason / a duration that it adds to the current time. That is a temporary-stat
   set whatever anyone calls it. **[L]** - and this one does not depend on the reference tree.

`0x007E` → `FUN_142d56f80` reads `u8, u8, u8, raw[124]` and clears stats. **[L]**

**Watch the collision.** `docs/opcodes.md` already lists `0x007D` as the **client's migration
hello** (character id at body offset 8). That is the *inbound* number; opcode spaces are
per-direction, and `research/msexe-gamestage-opcodes.md` says the same thing - the channel
dispatcher has no case that could answer the client's `0x007D`. Both are true. Give the new
outbound constant a name that cannot be confused with `MIGRATE_IN`.

### 5.2 The mask is 124 bytes and the bit order is big-endian *inside each u32*

`FUN_140a165f0` (the decoder) at `0x140a16630`: `mov r8d, 0x7c ; call 0x1406e9170` - a raw
read of **124 bytes**. The constructor `FUN_1402c24f0` zeroes exactly 0x7C. The handler then
loops `cmp dword [rbp+rax*4+0xc0], 0` for `rax = 0..29` and checks `[rbp+0x138]` separately -
**31 u32 words**. Three independent statements of the same size. **[L]**

The bit test, `FUN_1402bf6d0(mask, idx)` - `research/msexe-speed-is-cts92-1429755a0.txt`: **[L]**

```asm
1402bf6d3  cmp  edx, 0x3e0          ; idx >= 992 -> false
1402bf6e1  mov  eax, edx
1402bf6e3  mov  ecx, 0x1f
1402bf6e8  and  eax, 0x1f
1402bf6eb  sub  ecx, eax            ; shift = 31 - (idx & 31)
1402bf6ef  shr  rax, 5              ; word  = idx >> 5
1402bf6f3  mov  eax, [r8 + rax*4]   ; a NATIVE little-endian u32 load
1402bf6f7  shr  eax, cl
1402bf6f9  and  eax, 1
```

So: **word `idx >> 5`, read as a little-endian u32 out of the 124 raw bytes, bit
`1 << (31 - (idx & 31))`.** 992 bits total. Bit 0 is the *most significant* bit of word 0.

In wire-byte terms the same thing is: byte `4*(idx>>5) + 3 - ((idx>>3)&3)`, bit `7-(idx&7)`.

### 5.3 The per-stat body

Every one of the 407 standard blocks in `FUN_140a165f0` is byte-identical in shape
(`research/msexe-secondarystat-140a165f0.txt` census - 407 blocks of exactly 87 instructions):
**[L]**

```text
if (bit idx is set in the mask):
    value    = (mask & GROUP_int) ? u32 : (i16 sign-extended)     <- see 5.5
    reason   = u32          ; the skill id, or item id, that granted it
    duration = u32          ; ADDED to the current tick and stored as an expiry
```

The expiry maths, at the top of the decoder and inside each block: **[L]**

```asm
140a1665b  call [rip+0x284c74f]     ; a tick
140a16661  mov  [rsp+0x28], eax     ; now
...
140a16703  call 1406e8c20           ; duration = u32
140a16708  mov  [rsp+0x24], eax
140a16710  mov  ecx, [rsp+0x28]
140a16714  add  ecx, eax            ; expire = now + duration
140a16722  call <per-stat expire setter>
```

**The unit is milliseconds. [D]**, from three things and one of them is a wire measurement:

* the client's tick accessor `FUN_1429e3ef0` returns a cached counter; two packets 1.091 s
  apart carried `0x11d9754a` and `0x11d97982`, a difference of **1080** - milliseconds. **[L]**
* the only clock this PE imports is **`timeGetTime`** (WINMM, one import, `tools/pe_imports.py`),
  which is milliseconds. **[L]**
* `time` in `Skill.wz` is in seconds, so the server multiplies by 1000. Sending 30 would put
  the expiry 30 **ms** away and the icon would flash and vanish - which is a distinguishable
  outcome, and the test plan uses it.

Each stat's value is stored obfuscated as a 12-byte triple `{ key, rol(key^value,5), checksum }`
at `secStat + D`, its reason at `D+0xC` and its expiry at `D+0x18`. **[L]** - e.g. bit 92 is
`0x5cc / 0x5d8 / 0x5e4`. This is why a plain scan for "who reads the speed field" comes back
almost empty: reads go through `FUN_1401ba9d0(&field, checksum)` by address.

### 5.4 The tail, after the per-stat list

From `FUN_142d563d0` - `research/msexe-tempstatset-142d563d0.txt`: **[L]**

```text
u16   tDelay                    -> 142d56521
u8    (kept as an int)          -> 142d5652c
u8    flag1                     -> 142d5653a
u8    flag2                     -> 142d56547
u8    flag3                     -> 142d56555
u8    flag4                     -> 142d56563
u8    flag5                     -> 142d56571
u8    ONLY IF (mask & GROUP_A)  -> 142d56854, gated on FUN_140878790(mask)
u32                             -> 142d5690c   (only bit 0 is tested)
u8    -> user+0x3bbc            -> 142d56916
```

13 bytes, or 14 if that one conditional fires. **Send them all zero and the ambiguity cannot
hurt**: every field is fixed-width, so both parses read zeros either way.

`0x007E`'s conditional extras are gated on bits **27** and **411** only, so for a mask that
contains neither, its body is exactly `u8, u8, u8, raw[124]` = **127 bytes**. **[L]**

### 5.5 The one thing the file cannot tell us: short or int

`FUN_140a10800(secStat, mask)` decides the width, and **all 407 blocks call the same function
with the same constant**:

```text
FUN_140a10800:  t = mask AND *(0x143ac37e0)      ; FUN_14080fb80 = 124-byte AND
                return !(t is all zero)          ; FUN_14080f5b0 = "is all zero"
caller:         if (result) value = u32   else   value = i16 (cwde, sign-extended)
```

So the decision is **packet-wide**: if the mask intersects the constant at `0x143ac37e0`,
every value in the packet is a `u32`; otherwise every value is a sign-extended `i16`. **[L]**

`0x143ac37e0` is in `.data` (`0x143a41000..0x143ae3aa8`) and **this executable is
Themida-packed, so `.data` at rest is not the runtime content.** The evidence that it is
garbage rather than a real group: the 124 bytes there have **320 of 992 bits set (32%)**, and
the sibling constants at `0x143ac2380`, `0x143abd780` and `0x143aade60` are 34%, and the
qword at `0x143ac1898` - which `FUN_1429e3ef0` dereferences as an object pointer - is
likewise random. A semantic group mask would be a few percent. **[D]**

The composite is built at startup by `FUN_14003f380` as `OR` of two other `.data` constants
(`FUN_1402c2560` is a 124-byte OR), and those are equally unreadable. So this is not a
searching failure; it is a section that is not present in the file.

**Best available guess: `i16`.** The reference tree's equivalent set (`isEncodeInt()`) is 26
stats and does not contain Speed, so a Speed-only mask would not intersect. That is **[I]**
from a source that scores 1 of 8, so the test plan makes the run decide it, and makes the two
outcomes look different on screen.

---

## 6. CTS bit **92** is Speed

This is the load-bearing number and it did not come from the reference tree. It came from the
client's own English sentence.

**Step 1 [L].** `python tools/dump_stringids.py --grep Nimble` - the instrument's documented
positive control (`--id 1331` → `'[Welcome] Welcome to MapleStory!!'`) passes first:

```text
[ 5338] 0x14DA  'Nimble Feet cannot be used while another Speed increase effect is active.'
```

**Step 2 [L].** The client resolves a string by loading its id as an immediate. A scan of
every executable section for `mov r32, 0x14DA` in all sixteen register encodings finds
**exactly one** site: `0x1429759c4`, inside `FUN_1429755a0`.

**Step 3 [L].** The 59 bytes immediately above it (`research/msexe-speed-is-cts92-1429755a0.txt`).
`FUN_1401ba9d0(ptr, checksum)` is the de-obfuscating getter - `rol([ptr+4],5) ^ [ptr]`, with a
tamper check against the third word:

```asm
142975986  call qword ptr [rax + 0x48]     ; rax = user->GetSecondaryStat()
142975989  lea  rcx, [rax + 0x5cc]         ; the VALUE triple of ...
142975990  mov  edx, dword ptr [rax+0x5d4] ; ... its checksum
142975996  call 0x1401ba9d0                ; the de-obfuscating getter
14297599b  test eax, eax
14297599d  je   0x1429759d9                ; zero -> no speed buff -> allowed
1429759a8  lea  rcx, [rax + 0x5d8]         ; the REASON triple of the same stat
1429759b5  call 0x1401ba9d0
1429759ba  cmp  r15d, eax                  ; same skill id -> allowed
1429759bd  je   0x1429759d9
1429759bf  mov  edx, 0xb
1429759c4  mov  ecx, 0x14da                ; "...another Speed increase effect is active."
1429759cf  call 0x1415ecc10
```

**Step 4 [L].** `0x5cc` and `0x5d8` are exactly what `FUN_140a165f0`'s block for bit index
**92** writes, through `FUN_140c3ba60` (`mov [rbx+0x5cc], eax`) and `FUN_140c40b00`
(`mov [rbx+0x5d8], eax`). The immediate was read straight out of the bytes rather than off a
parser, because it is the one number everything else rests on:

```text
140a17e2e  ba 5c 00 00 00                mov  edx, 0x5c              ; 92
140a17e33  48 8d 8c 24 40 47 00 00       lea  rcx, [rsp + 0x4740]    ; the received mask
140a17e3b  e8 90 78 8a ff                call 0x1402bf6d0            ; bit test
...
140a17e7e                                call 0x140c3ba60            ; value  -> +0x5cc
140a17ebf                                call 0x140c40b00            ; reason -> +0x5d8
```

**Two halves found separately, meeting on four offsets** - `0x5cc`, `0x5d4`, `0x5d8`, `0x5e0` -
where the second of each pair is the first plus 8, the checksum word of the same triple.

Corroboration, weaker but independent: in the remote decoder `FUN_140a46e50` bit 92 is
**the first of only two sign-extended single bytes** (`READ u8 ; movsx eax, al`, no reason
field - the other is bit 213 at `0x140a47082`), and in the reference tree the remote encoding
of `Speed` is a bare `encodeByte` with no reason. **[I]** - and the reference is a different
version, so this corroborates and does not decide.

Bit 92 is **not** one of the 65 stats with extra conditional fields, so a Speed-only packet
has no per-stat extras. **[L]**

---

## 7. WIRE IT LIKE THIS

Nothing below is wired. `crates/net/` has no temporary-stat module; `crates/world/src/session/`
logs `0x013C` as `UNKNOWN` and drops it.

### 7.1 The outbound packet, byte for byte

**"Give this character Nimble Feet (1002) at level 3 for its WZ duration":**

```text
opcode  0x007D

body, 152 bytes:

  offset   0 .. 123   the mask, 124 bytes, all zero EXCEPT
           offset 8   = 0x08                     <- bit 92, and nothing else
                        (word 92>>5 = 2 -> bytes 8..11 as LE u32 = 0x00000008,
                         because 1 << (31 - (92 & 31)) = 1 << 3)

  offset 124 .. 125   i16   10        0a 00      <- speed, from Skill.wz level/3/speed
  offset 126 .. 129   u32   1002      ea 03 00 00 <- reason: the skill id
  offset 130 .. 133   u32   30000     30 75 00 00 <- duration, MILLISECONDS = time * 1000

  offset 134 .. 151   18 zero bytes               <- the tail; see below
```

The full non-mask part on the wire is `0a 00 ea 03 00 00 30 75 00 00` followed by 18 zeros.

**Why 18 zeros and not 13.** The tail is 13 bytes, or 14 if one conditional fires
(section 5.4), and if the width guess in section 5.5 is wrong the client consumes 2 extra
bytes for the value. 13 + 1 + 2 = 16, and 18 leaves margin. Every tail field is fixed-width
and every byte is zero, so **all four parses read the same zeros and none of them runs off
the end of the packet.** Do not put a non-zero byte in the tail until section 5.5 is settled.

General form, for any stat and level:

```rust
// mask: 31 little-endian u32 words, bit set with  words[i >> 5] |= 1 << (31 - (i & 31))
// value:  i16 (see 5.5), reason: u32 skill id, duration: u32 milliseconds
const CTS_SPEED: u32 = 92;
let duration_ms = wz_time_seconds * 1000;
```

The **reset** packet, if we want to expire it explicitly rather than letting the client's own
`tExpire` run out - opcode **`0x007E`**, 127 bytes:

```text
u8 0, u8 0, u8 0, then the same 124-byte mask with bit 92 set
```

(safe for bit 92: `0x007E`'s conditional reads are gated on bits 27 and 411 only.)

### 7.2 What to do with the inbound packets

| in | do |
|---|---|
| **`0x013C`** | read `u32 skillId @0`, `u32 level @4`. Check the character actually has that skill at >= that level (`store`), and that its cooldown has expired. Deduct `mpCon` and send the existing `0x007C` StatChanged for MP. Send `0x007D` as above. Record the expiry; send `0x007E` when it passes. **Every effect must hang off the "the cast was allowed" transition**, not off the request - `CLAUDE.md`'s Heena-quest rule; a repeat cast must not re-pay anything. |
| **`0x013D`** | **log only, never reply.** Optionally cross-check `map[0x013C][skillId]` against our own cast count and warn on a mismatch. |
| **`0x01A5`** | log only. Optionally cross-check the reported skill levels against the store. |

The client already plays the cast animation and the client already refuses a second cast while
Speed is held by another skill, so neither needs a packet from us.

**Nothing here authenticates.** The channel socket carries no credentials; a buff is granted
because a packet arrived on the socket.

### 7.3 The run, and what each outcome MEANS

One variant. Send the packet in 7.1 once, from a GM command, while the character is standing
in a field. Watch the buff tray (top right) and the walking speed.

| what the owner sees | what it means | next |
|---|---|---|
| **Buff icon appears, counts down ~30 s, character visibly walks faster** | Everything in this document is right: opcode, bit order, bit 92 = Speed, `i16` value, milliseconds. | Wire it for real, add MP cost and cooldown |
| **Icon appears and vanishes within a second** | Opcode, mask and bit are right; the **value is a `u32`, not an `i16`** (section 5.5). Under a u32 parse this packet's duration reads as 0. | Resend with the value as `u32 10` - shifts the two following fields by 2 bytes, nothing else changes |
| **Icon appears and stays 30 s but the character does not move faster** | Packet is right, **bit 92 is not Speed**. | Sweep 89, 90, 91, 93, 94, 95 one per 10 s and report which one changes the walk |
| **Nothing at all, client keeps playing** | `0x007D` is not TemporaryStatSet, or the mask bit order is inverted. | Try the mask with the bit computed as `1 << (idx & 31)` instead of `1 << (31 - (idx & 31))` |
| **Client freezes or dies** | The tail is short and a read ran off the end. | Copy `world.log`, `client-patched\maplecw-hook.log` and `client-exit.log` into `research/fixtures/` first; then count `0x007D` in `world.log` against the dispatch lines in the hook log - a missing dispatch line means the handler was entered and never came back |

The *second* and *third* rows are the reason to watch both the icon **and** the feet. Watching
only the icon would pass on a wrong bit; watching only the speed would pass on nothing.

Rendered for the launcher, one line: *"Type the buff GM command. Does a buff icon appear top
right, does it count down for 30 seconds, and does the character walk faster? Report all three
separately."*

---

## What I did NOT establish

Each of these names the blind spot that produced it.

* **`i16` vs `u32` for the stat value.** Section 5.5. The deciding constant is in `.data`,
  which is Themida-packed, so the file does not contain it. This is not a search that came
  back empty - it is a section that is not there. The run decides it, and the two outcomes
  look different on screen.
* **Whether the conditional `u8` in the tail fires for bit 92.** Same packed constant
  (`0x143ac2380`, via `FUN_140878790`). Neutralised by sending the tail as zeros, not solved.
* **The last 13 bytes of `0x013C`.** One sample, and I did not identify which of the sixteen
  builders produced it. Two splits fit the length (`u32,u8,u32,u32` and `u32,u32,u8,u32`) and
  one sample cannot separate them. The server does not need them.
* **The names of the other 409 stat bits.** Only bit 92 is named, and only because the client
  ships an English sentence about Nimble Feet. The reference tree's `localOrders` sequence
  does *not* align 1:1 with this client's decode order - the run 83..95 is 13 entries where
  v214 has 14, and the block after it is 4 where v214 has 5 - so the offset is not constant
  and any name read off it is a guess. I did not use it for bit 92 and it should not be used
  for the next one either.
* **Which stat bit `Jump` is.** Not looked for. The same string-table trick may work
  (`--grep Jump`) and is one command.
* **What `[user+0x3bbc]`, `tDelay`, and the five tail booleans do.** They are read and stored;
  I did not follow them.
* **`FUN_140878790`/`FUN_140a10800`'s groups, and the array at `0x143abd320`.** All in packed
  `.data`. Dumped as 124-byte masks they are 32-34% dense, which is noise, not a group.
* **`tools/fieldrefs.py 0x4060` over the whole image returned an empty result with exit 0**
  after 15 minutes. The scoped run
  (`python tools/fieldrefs.py 0x4060 --lo 0x142d19000 --hi 0x142d1a000`) returns 8 correct
  hits and `tools/rangescan.py` agrees with it hit for hit, so the instrument works when
  scoped. **I did not diagnose the empty whole-image run, and it must not be read as a
  negative.** The answer it would have given was found another way (`tools/callers.py` and the
  packet-fields census), so nothing here rests on it.

## Instrument controls run before believing anything above

* `python tools/reads.py 0x140304100 2` → the documented mix of helper and direct reads at
  `140304138 raw`, `140304144 u8`, `140304183 u8`, then the `u16` run. **Passes.**
* `python tools/dump_stringids.py --id 1331` → `'[Welcome] Welcome to MapleStory!!'`.
  **Passes** - and this is the instrument the Speed answer rests on.
* `python tools/rangescan.py 0x4060 0x142d19000 0x142d1a000` and
  `python tools/fieldrefs.py 0x4060 --lo … --hi …` → identical 8 hits. **Agree.**
* The `0x013D` decode was checked against a count of the same event in a second file
  (`grep -c "<- 0x00DF" world.log` = 7 = the number in the packet), which is
  `CLAUDE.md`'s "count the same event in two logs" and is the only reason section 2 is [L]
  rather than [I].
