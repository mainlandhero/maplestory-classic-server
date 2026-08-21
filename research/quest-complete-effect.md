# The quest-finish fanfare: `0x02D1` effect **15**, and the EXP line that was already going out

The owner, after a run on 2026-08-21:

> *"Quest finish still does not trigger the SFX for quest finish (this is a different SFX
> than quest completion). Quest finish does not trigger EXP gain."*

Labels are the project's: **[L]** read off this client's listing, its WZ or a capture ·
**[D]** derived from two or more [L] facts · **[I]** inferred, not established.

**No Ghidra.** Another agent held the project lock. Everything below came from
`client-patched/MapleStory.exe` through `tools/listing.py`, `tools/callers.py`,
`tools/dataref.py`, `tools/pdata_lookup.py`, `tools/dump_stringids.py`,
`target/release/wz-dump.exe`, and this run's own `world.log`.

---

## Summary

| | |
|---|---|
| **the packet** | **`0x02D1` `UserEffectLocal`, body `u8 effect` — one byte.** Remote twin `0x02AF`, `u32 charId, u8 effect` |
| **the effect id** | **`15` (`0x0F`)** — the arm at `0x14278e11d` in `FUN_1427863f0` |
| what it does | plays `Effect/BasicEff.img/QuestClear` **and** `Sound/Game.img/QuestClear` at volume 100 |
| the sound | **is in this client's WZ.** `Sound_001.wz/Game.img/QuestClear`, 7410 bytes, 2411 ms **[L]** |
| the animation | **is NOT in this client's WZ.** `Effect_000.wz/BasicEff.img` has 40 nodes and `QuestClear` is not one of them **[L]** |
| is it a `0x0089` sub-case? | **No.** |
| does the client do it by itself on the quest record? | **No** — see §4 |
| **the EXP line** | **already goes out.** This run's `world.log` line 112: `0x0089` body `03 01 0200000000000000 00 0000000000000000` — kind 3, `white=1`, `exp=2`, `in_chat=0`, `mask=0` **[L]** |

**The brief's premise for part (b) is falsified by the same run's own log.** See §6 — that
section is the important one, because it means the EXP half of the owner's report is *not* a
missing packet and building one would have been building a duplicate.

---

## 1. The sound exists and the client asks for it by name

`target/release/wz-dump.exe cat client-patched/Data/Sound/Sound_001.wz Game.img` — the whole
image, 40 sounds. The quest-relevant ones: **[L]**

```json
"QuestAlert":  { "_sound": true, "bytes": 6942, "duration": 2246 },
"QuestClear":  { "_sound": true, "bytes": 7410, "duration": 2411 },
"questCount":  { "_sound": true, "bytes": 2574, "duration":  796 },
"IncEXP":      { "_sound": true, "bytes": 9282, "duration": 3039 },
"LevelUp":     { "_sound": true, "bytes": 10920, "duration": 3586 },
"JobChanged":  { "_sound": true, "bytes": 8658, "duration": 2818 }
```

Three of those six are **dead assets in this build**: a UTF-16 and ASCII scan of the whole
76 MB image finds `QuestClear` twice, `JobChanged` twice, `LevelUp` eight times — and
**`IncEXP` and `questCount` zero times, in either encoding.** **[L]**

> **The instrument speaks before it is believed.** The same scan, run unchanged, finds
> `PickUpItem` (1), `Portal` (10), `QuestAlert` (1), `BasicEff` (63) and `LevelUp` (8). It is
> not a search that returns nothing; it returns nothing *for those two names*.

So **this client has no EXP-gain sound and no quest-counter sound.** Nothing the server can
send will play them. That matters for the second half of the owner's sentence and is the reason
§6 goes looking somewhere else entirely.

`QuestAlert` appears exactly once, and only inside the longer path
`Effect/BasicEff.img/QuestAlert4/Default` (`lea` at `0x14231da94`) — it is a UI image name,
not a `Sound/Game.img` name. **`QuestClear` is the only quest sound this client can play.**
**[D]**

### The two forms the client holds

```text
0x1432ae518   L"Effect/BasicEff.img/QuestClear"    the animation path
0x1432b1f68   L"QuestClear"                        the bare sound name
```

Both are reached through slots in the `.data` resource-string table:

```text
[0x143a46f58] -> 0x1432ae518    'Effect/BasicEff.img/QuestClear'
[0x143a484a0] -> 0x1432b1f68    'QuestClear'
```

Neighbours in the same table, dereferenced out of the PE, place it exactly: **[L]**

```text
[0x143a46f00] 'Sound/Game.img/'                          <- the prefix FUN_1429f14c0 applies
[0x143a46f48] 'Effect/BasicEff.img/LevelUp'
[0x143a46f50] 'Effect/BasicEff.img/JobChanged'
[0x143a46f58] 'Effect/BasicEff.img/QuestClear'           <- here
[0x143a46f60] 'Effect/BasicEff.img/CraftingLevelUp'
[0x143a46f68] 'Effect/BasicEff.img/CraftingUnlock'
[0x143a46f70] 'Effect/BasicEff.img/CitizenshipGet'
[0x143a46f78] 'Effect/BasicEff.img/CitizenshipGradeUp'
```

`python tools/dataref.py 0x143a46f58` and `... 0x143a484a0` each give **exactly two**
readers, and they are the same two functions: **[L]**

```text
0x14278e180 / 0x14278e196   in FUN_1427863f0    <- the packet-driven effect handler
0x1428a18d0 / 0x1428a18e6   in FUN_1428a1460    <- NOT quests; see below
```

**`FUN_1428a1460` is a red herring and it was checked rather than assumed.** Its only caller
is `FUN_14289a3a0` index `0x13`, i.e. opcode **`0x02D8`**; it reads `raw8, u8, u16` and its
arms load string ids `0x9F5`/`0x9F6`/`0x9F9`, which decrypt to *"The Auto-Pickup Skill has
been added to %s's list of skills."* and siblings. It is the **pet-skill** notice reusing the
same fanfare. **[L]**

---

## 2. The packet: `0x02D1` local, `0x02AF` remote, effect **15**

Routing is already in `research/level-up.md` §6 and `crates/net/src/stats.rs`, and it was
re-derived here rather than copied: **[L]**

```asm
; FUN_14289a3a0 - the LOCAL user's packet switch
14289a3f3  lea  eax, [rdx - 0x2c5]
14289a3f9  cmp  eax, 0xd9
14289a3fe  ja   <default>
14289a40d  mov  edx, [rcx + rax*4 + 0x289d660]
14289a417  jmp  rdx
   ...
14289a439  mov  rdx, r13 / mov rcx, r15 / call 0x1427863f0     ; index 0x0C = opcode 0x02D1
14289a5be  mov  rdx, r13 / mov rcx, r15 / call 0x1428a1460     ; index 0x13 = opcode 0x02D8
```

`FUN_1427863f0` reads **one `u8`** at `0x14278644e` and then runs **two** switches on it.
`research/level-up.md` documented the second and not the first; both matter, and the first
one is what proves the body is a single byte.

```asm
142786482  lea  ecx, [rbx - 8]                 ; FIRST switch, on effect-8
1427864a8  cmp  ecx, 0x45
1427864ab  ja   0x14278bd20                    ; out of range -> straight to the second switch
1427864b4  movzx eax, byte [0x142791300 + idx] ; two-level MSVC table
1427864bc  mov  ecx, dword [0x14279129c + eax*4]
1427864c6  jmp  rcx
   ...
14278bd20  <every arm of the first switch falls through or jmps here>
14278bd5d  movzx ebx, byte [rbp+0x80]          ; the saved effect byte
14278bd7d  cmp  ebx, 0x54 / ja <exit>
14278bd8d  mov  ecx, dword [0x142791348 + ebx*4]   ; SECOND switch, index = effect
14278bd97  jmp  rcx
```

**Effect 15 takes the first switch's *default* arm** — byte-table index 24, target
`0x14278bd20` — so it performs **no packet read at all** before the second switch. **[L]**
The whole body of `0x02D1` is therefore `u8 effect`, one byte, and `0x02AF` is
`u32 charId, u8 effect`, five.

### The arm, in full

Second-switch table `0x142791348`, 85 entries, index **15** → `0x14278e11d`, extent
`[0x14278e11d, 0x14278e1a7)`. **[L]**

```asm
14278e11d  mov  rdi, [rip+0x1331ccc]      ; [0x143ABFDF0] - the effect-layer singleton
14278e12f  lea  rdx, [r15 + 0xe58]
14278e13d  call 0x1407073c0
14278e145  lea  rcx, [r15 + 8]
14278e150  call 0x1409397b0               ; -> r9, the position/owner
14278e17d  xor  r8d, r8d
14278e180  mov  rdx, [rip+0x12b8dd1]      ; [0x143A46F58] = L"Effect/BasicEff.img/QuestClear"
14278e18a  call 0x140e16070               ; PLAY THE ANIMATION
14278e18f  xor  r8d, r8d
14278e192  lea  edx, [r8 + 0x64]          ; volume 100
14278e196  mov  rcx, [rip+0x12ba303]      ; [0x143A484A0] = L"QuestClear"
14278e19d  call 0x1429f14c0               ; PLAY THE SOUND
14278e1a2  jmp  0x14279102e               ; the common exit
```

`tools/listing.py` reports **zero** `READ` sites inside that extent. **[L]**

**`FUN_1429f14c0` is the `Sound/Game.img` player**, and that is measured rather than named:
its first act is `1429f14ec mov rsi,[rip+0x1055a0d]` = `[0x143A46F00]` = `'Sound/Game.img/'`,
which it concatenates with `rcx`. `edx` is the volume. **[L]** It has 81 call sites in 62
functions image-wide.

**The sound call is unconditional after the animation call.** There is no branch between
`0x14278e18a` and `0x14278e19d` and nothing tests the animation's return value. **[L]** That
is the whole reason §5's missing-art problem does not sink this.

### The effect ids that could be named, from this client only

Resolving every `[rip+disp]` operand in `FUN_1427863f0` to a `.data` slot and dereferencing:
**[L]**

| effect | arm | resource |
|---:|---|---|
| **0** | `0x14278bd99` | `Effect/BasicEff.img/LevelUp` + sound `LevelUp` |
| **13** | `0x14278e028` | sound `Portal` only, no animation |
| **14** | `0x14278e040` | `Effect/BasicEff.img/JobChanged` + sound `JobChanged` |
| **15** | `0x14278e11d` | **`Effect/BasicEff.img/QuestClear` + sound `QuestClear`** |
| 17 | `0x14278ee66` | `Effect/ItemEff.img/%d` |
| 21 | `0x14278f2c1` | sounds `EnchantSuccess` / `EnchantFailure` |
| 83 | `0x142790e6e` | `Effect/BasicEff.img/CitizenshipGet` |
| 84 | `0x142790f4c` | `Effect/BasicEff.img/CitizenshipGradeUp` |

> **Corroboration, labelled [I] because it is the reference and the reference scored 1 of 8.**
> The v83-era `UserEffect` enum runs `… PLAY_PORTAL_SE(7), JOB_CHANGED(8), QUEST_COMPLETE(9) …`
> — Portal, JobChanged, QuestClear *consecutive*, in that order, with LevelUp at 0. mscw has
> the same three consecutive in the same order at 13, 14, 15, with LevelUp at 0. A constant
> `+6` across a preserved run. Every number in the table above is read out of this client's
> own jump table and does **not** depend on that.

### Two preconditions that make it silently do nothing

Both sit between the two switches, at `0x14278bd29`..`0x14278bd7d`, and both apply to
**every** effect including LevelUp. **[L]** for the branches, **[I]** for the names.

```asm
14278bd29  call 0x141892840          ; the current field
14278bd31  je   0x14278bd45          ;   NULL -> bl = 1
14278bd3b  call 0x14182ffd0          ;   else bl = field->[0xa8]->[0x2d9]   (a field-info byte)
14278bd4a  call 0x142826340          ; a user-state predicate -> al
14278bd4f  jne  ...                  ; al != 0 -> dl = 0
14278bd53  jne  ...                  ; bl != 0 -> dl = 0
14278bd75  test dl,dl / je <exit>    ; dl == 0 -> NOTHING HAPPENS
```

Effects `0x4F`, `0x50` and `0x52` bypass the gate; **15 does not**. So:

* **send it only from a settled field.** No field object → `bl = 1` → the effect is dropped
  in silence. This is the same rule `research/quest-state.md` already imposes on `0x0089`.
* `FUN_142826340` returns non-zero when `FUN_141715f80(user)` is true (a morph/transform
  state of `0x1a`, `0x1b`, `0x1c` or `0x20`), when two virtual predicates fire, or on two
  specific field ids. In an ordinary Mushroom Town session all of those are false. **[L]** for
  the reads; that they are false in practice is **[I]**.

---

## 3. The animation is not in this client's WZ, and the sound is

`Effect_000.wz/BasicEff.img` has **40** top-level nodes and **none of them is `QuestClear`**:
**[L]**

```text
Assaulter Buff BuffIconEffect CameraMode CitizenshipGet CitizenshipGradeUp
CraftingLevelUp CraftingUnlock DoubleJump EmptyCanvas Enchant Flying ItemSkill
JobChanged LevelUp LevelUp2 NoBlue0..NoViolet1 PvpLevelUp Summoned Teleport
Transform TransformOnLadder archerDoubleJump
```

`Effect/_Canvas/_Canvas_000.wz/BasicEff.img` has 35 and also has no `QuestClear`. A
case-insensitive grep for `quest` over the whole decoded image returns 0. **[L]**

So of the seven `Effect/BasicEff.img/*` paths the client holds, **QuestClear is the only one
whose node was cut from the archive.** LevelUp, JobChanged, CraftingLevelUp, CraftingUnlock,
CitizenshipGet and CitizenshipGradeUp are all present.

`Effect_000.wz/Quest.img` exists (two animations, `0` and `1`), and **nothing references it**
— a scan for the literal `Effect/Quest.img` in either encoding finds no such string, while
the same scan finds `Effect/BasicEff.img/QuestClear` and `Effect/BasicEff.img/QuestAlert4/Default`.
It is a leftover asset. **[L]**

### Will the missing node crash the client? **[D] — no, and here is the chain**

This is the same shape as the Shop2 fault (`UI/UIWindow2.img/Shop2/backgrnd` absent →
`_com_issue_errorex` → the unwinder faults), so it was traced rather than waved at.

```asm
; FUN_140e16070(layer, path, ...)
140e16169  call 0x140dc12e0               ; resolve
140e1616f  mov  rbx, [rbp-0x40]
140e16176  jne  0x140e1619d               ; non-null -> carry on
140e16196  xor  eax, eax / jmp <return>   ; NULL -> return 0, no throw

; FUN_140dc12e0
140dc1316  call 0x14090de10               ; VARIANT lookup; on a failing HRESULT it
                                          ; ZEROES the out-variant and clears it - no throw
140dc1330  call 0x1401a5040               ; QueryInterface wrapper; a null input returns
                                          ; E_NOINTERFACE, and E_NOINTERFACE is the ONE
                                          ; failure code 1401a50dc explicitly does NOT throw on
140dc1373  mov  rbx, [rbp-0x70]
140dc137a  je   0x140dc1516               ; NULL -> clean exit
140dc138d  call [rax+0x40]                ; only reached with a RESOLVED node
140dc1392  jns  0x140dc13a5
140dc13a0  call 0x142ef3ad0               ; _com_issue_errorex - the throw, on a resolved
                                          ; node whose COM call fails
```

**The throw is on the "found it but it misbehaved" path, not the "not found" path**, and
three separate null checks stand between a missing node and it.

> **Named blind spot, per `CLAUDE.md`.** The lookup itself is a virtual call —
> `14090de8b call [rax+0x48]` through the WZ provider's vtable — and I did not disassemble
> the callee. If *that* raises a C++ exception rather than returning a failing HRESULT, the
> C++ code above never runs and this [D] is worth nothing. Everything on this side of it is
> written for the not-found case; that is an argument from how the caller is written, which
> is strong and is not proof.

**Expected outcome on screen: the fanfare plays, and no animation is drawn.** The owner asked for
the SFX, so that is the deliverable; the missing art is worth telling them *before* the run so
"I heard it but saw nothing" reads as success rather than half a failure.

---

## 4. It is not `0x0089`, and it is not client-side

The brief asked for a clear answer among three options. It is **(i), a dedicated opcode.**

**Not a sub-case of `0x0089`.** The 36-entry table at `0x142d44a88` is fully enumerated in
`research/client-messages.md` §2, and no entry reaches `FUN_1427863f0`, `0x140e16070` or
`0x1429f14c0`. Independently: the sound player `FUN_1429f14c0` has 81 call sites in **62**
functions, and none of the `0x0089` family (`FUN_142d43ee0`, `FUN_142d59360`,
`FUN_142d59e20`, `FUN_142d5b750`, `FUN_142d5da20`) is among them. **[L]**

**Not something the client does by itself when the quest record changes state.** The quest
record handler `FUN_142d59e20` (2688 bytes, 101 calls) and its state machine `FUN_142d5b750`
(3795 bytes, 129 calls) contain **no call to `0x140e16070`, `0x1429f14c0` or `0x1427863f0`**.
`FUN_142d5b750` makes exactly one string-table call, at `0x142d5bc98`, and it is gated on
`questId == 0x1964 || questId == 0x1ecc` — two hardcoded ids — posting string `0x11C6` into
the chat log at category `0xB`. **[L]**

> **The negative's blind spot, stated so it can be quoted.** I read the *direct* callee list
> of those two functions, not the transitive one; a callee three levels down could in
> principle reach a sound. What makes the negative hold anyway is that the check was also run
> from the other end: `tools/dataref.py` finds **exactly two** readers of the `QuestClear`
> sound-name slot `0x143a484a0` in the whole image, and neither is in the quest module. For
> the client to play this sound from the quest record, it would have to reach one of those
> two sites, and both are inside packet handlers for `0x02D1`/`0x02AF` and `0x02D8`.

**One loose end, honestly unresolved.** String id `0x043E` decrypts to **`'Quest Complete!'`**
and is loaded at `0x14180a27f` inside `FUN_1418099f0`, whose only caller is `FUN_141809530`.
That function loads a contiguous run of ids `0x426`..`0x444`, of which **only `0x43E` is
non-empty in this build** — the shape of a UI window's label table, not an event. I did not
establish what draws it or whether any packet can trigger it. **[I]** that it is the quest
journal; **[L]** that the string exists and that the function loads it.

---

## 5. What the server sends today, measured

From this run's `world.log`, lines 105-114, verbatim including bodies: **[L]**

```text
06:08:34.717 <- 0x0151  02 e9030000 01000000 43ffe501 ffffffff      COMPLETE, quest 1001, npc 1
06:08:34.719 -> 0x0089  01 e9030000 02 62eb876a00000000             quest record: 1001 -> state 2
06:08:34.719 -> 0x0070  01 00 01000000 0003040100                   InventoryOperation REMOVE
06:08:34.719 -> 0x007C  01 00 01 00000100 0200000000000000 00       StatChanged, mask bit 16, exp = 2
06:08:34.719 -> 0x0089  03 01 0200000000000000 00 0000000000000000  EXP MESSAGE, white = 1
06:08:34.719 -> 0x055B  ...                                          the NPC's next line
```

Five packets. **No `0x02D1` anywhere in the file** — `grep` returns nothing — which is
exactly why there is no sound.

---

## 6. The EXP line is **already** being sent, and the brief's premise for it is wrong

> The task this document answers said: *"no `0x0089` kind 3 EXPERIENCE line is sent for the
> quest EXP. Compare a mob kill, which does send `0x0089` kind 3."*

**That is false, and the file it cites disproves it.** `world.log` line 112, quoted above:

```text
-> 0x0089   body 03 01 0200000000000000 00 0000000000000000
                 ^kind 3  ^white=1  ^exp=2     ^in_chat=0  ^mask=0        19 bytes
```

and line 2520, a mob kill in the *same* session:

```text
-> 0x0089   body 03 01 c800000000000000 00 0000000000000000
                 ^kind 3  ^white=1  ^exp=200   ^in_chat=0  ^mask=0        19 bytes
```

**Byte-for-byte the same shape, differing only in the amount.** Both are
`net::message::exp_gained(n, true)`, pushed by `Session::award_experience`
(`crates/world/src/session/combat.rs` ~line 441), which
`apply_quest_completion_rewards` already calls at `crates/world/src/session/npc.rs:234`
with `white = true`. **[L]**

So: **there is nothing to build for part (b).** The body the brief asked me to confirm is
correct and is in `crates/net/src/message.rs` with tests on every field, and it is wired.

### White or yellow for quest EXP?

**It is a policy choice and the client has no opinion.** The byte selects
`FUN_142572050`'s third argument and nothing else: **[L]**

```asm
142d5f014  cmp  dword [rbp+0xb0], ebx     ; ebx = 0
142d5f01a  je   142d5f021                 ;   white == 0 -> r8d = 4
142d5f01c  xor  r8d, r8d                  ;   white != 0 -> r8d = 0
142d5f027  call 0x142572050
```

`white = 0 -> r8d = 4 -> yellow` is **[L]**, from the owner's own screenshot of a yellow
`You received EXP (+2)` (`research/exp-sharing.md`). That `1` gives white is recorded two
different ways in this repo and the two disagree: `STATUS.md`'s CONFIRMED table says it was
seen on screen; `research/exp-sharing.md`, written later the same day, says it is **[I]** and
"nobody has seen it". **I did not resolve that**, and it does not need resolving here: the
server already sends `white = 1` for a quest reward, and the owner's own rule — *"if I was the
person who dealt majority damage, I should see a white line"* — makes a quest payout white by
construction, since it is not a share of anybody else's kill.

### Is there a "quest" bonus-mask bit?

**No.** The 28 bits `FUN_1408cfcf0` tests are enumerated in `research/client-messages.md` §3
with the string each one draws. The only two mentioning quests are: **[L]**

```text
0x40000   0x00D5  '%% additional Party Quest EXP gained from the item (+%lld)'
```

— a *party quest* item bonus, not an ordinary quest — and the field at `dst+0x28`, which
draws string `0x00DE` `'The next %d completed quests will include additional Event Bonus EXP.'`
(loaded at `142d5e9bf`). That second one is a promo counter, not a label, and it is
**unreachable with `in_chat = 0`**: `1408cfd7e` only reads `dst+0x24` when `in_chat != 0`, and
`in_chat != 0` routes the whole line into the chat log, which is the opposite of what the owner
asked for. `crates/net/src/message.rs` calls that field `quest_bonus` in a doc comment; the
name is misleading and this paragraph is the correction.

**So: no mask bit, keep `mask = 0`.** The line reads `You received EXP (+2)` whether the two
came from a snail or from Heena, and there is no wording in this client's string table that
would distinguish them.

### Then what is "Quest finish does not trigger EXP gain"?

Not a missing packet. Three readings survive, and none of them costs a client run to
distinguish because two of them are already answered:

1. **They mean an EXP-gain sound or effect.** *Impossible in this client.* `IncEXP` is in
   `Sound/Game.img` but the string `IncEXP` does not occur anywhere in the executable, in
   either encoding, while the same scan finds `PickUpItem`, `Portal` and `QuestAlert`. The
   client never asks for it. **[L]**
2. **They mean the on-screen line did not appear.** The packet went out with the same body as
   the mob-kill line in the same session. If the kill line drew and this one did not, the
   difference is not in this packet — and the next thing to look at is the `0x055B` script
   box that follows it 0 ms later, not the EXP builder. **[I]**, and it is a *question*, not
   a finding.
3. **They mean 2 EXP is invisible.** Quest 1001's `Act.1` pays `exp 2`; the mob kills in the
   same log paid 200 each. **[L]** from `gm-handbook`/the log.

**The cheap discriminator, costing no launch:** temporarily raise one quest's `complete_exp`
to something unmissable and watch whether the line appears. If it does, this was (3).

---

## 7. WIRE IT LIKE THIS

`crates/world/src/session/` belongs to the coordinator and is untouched by this work. The
builder is new and is `crates/net/src/questeffect.rs`; the one shared-file edit is the `mod`
line in `crates/net/src/lib.rs`.

### The one change

In **`crates/world/src/session/npc.rs`**, `record_quest_complete`, after
`apply_quest_completion_rewards` has run — so the journal row, the item lines and the EXP
line are all already queued and the fanfare lands on a finished turn-in:

```rust
out.push(Reply {
    opcode: net::questeffect::USER_EFFECT_LOCAL,          // 0x02D1
    body:   net::questeffect::quest_clear_local(),        // one byte: 0x0F
    what:   format!(
        "UserEffectLocal QuestClear for quest {finished} - plays Sound/Game.img/QuestClear. \
         The animation node Effect/BasicEff.img/QuestClear is NOT in this client's WZ, so \
         expect sound and no picture."
    ),
});
```

Put it inside the `Ok(Some(at))` arm's follow-on path — i.e. only when a completion was
actually recorded. A quest that was not in progress must not play a fanfare.

### The order at quest completion, once wired

| # | packet | why it is here |
|---|---|---|
| 1 | `0x0089` kind 1, state 2 | the journal row. **First**, so the fanfare lands on a book that already says "complete" |
| 2 | `0x0070` | items given / taken |
| 3 | `0x0089` kind 0 | one `item_gained` line per item given |
| 4 | `0x007C` | the new EXP total, mask bit 16 |
| 5 | `0x0089` kind 3 | `You received EXP (+n)`, `white = 1`, `in_chat = 0`, `mask = 0` |
| 6 | **`0x02D1` effect 15** | **the new one.** The `QuestClear` fanfare |
| 7 | `0x055B` | the NPC's next line |

1-5 and 7 are what the server sends today and are unchanged. Only **6** is new.

**Do not add `0x02AF`.** The remote twin is `u32 charId, u8 effect` and is what *other*
players' clients are sent; `research/exp-sharing.md` records that this server has no way to
push a packet into another session's socket, so it would go nowhere. The builder is provided
for when that exists; sending it to the *same* client would look the local user up in its own
remote-user pool, not find itself, and be dropped in silence (`FUN_1429bb720`).

### What to watch on the run, and what each outcome means

**One variant.** Change nothing else in the same launch.

| what happens | what it means |
|---|---|
| the fanfare plays, nothing is drawn | **success, and the expected result.** The sound node is in the WZ and the animation node is not |
| the fanfare plays *and* something is drawn | the animation resolved from somewhere this analysis did not find. Say so - §3 would need correcting |
| nothing at all, no fault | the gate at `0x14278bd47` closed. Check the field-info byte and the user state in §2; `-Probe "watch@14278e11d:hits=8"` shows whether the arm ran |
| the client faults ~seconds later | the [D] in §3 is wrong: the missing WZ node throws after all. **Abandon effect 15**; there is no other route to this sound |

A `watch@1427863f0:hits=8` slot costs nothing and answers "did the packet reach the handler
at all" independently of anything on screen.

---

## 8. What this document does NOT claim

* **That effect 15 has ever been on a wire.** Nothing here has been in front of the client.
  Every claim is static analysis over the PE, the WZ, and one existing `world.log`.
* **That the missing `Effect/BasicEff.img/QuestClear` node is harmless.** That is **[D]** with
  a named blind spot (§3): the WZ lookup's innermost step is a virtual call I did not read.
* **That the on-screen EXP line failed to draw.** The packet is measured; what the owner saw is
  not, and §6 lists three readings rather than picking one.
* **What `white = 1` looks like.** Two files in this repo disagree about whether anyone has
  seen it and I did not adjudicate.
* **What draws `'Quest Complete!'` (string `0x043E`).** Located, not traced (§4).
* **The other 77 effect ids.** Eight of the 85 are named here, from resource strings. The rest
  were enumerated (every arm's extent is known) but not read.
* **That `0x02D1` is called `UserEffectLocal` in this binary.** The name comes from
  `crates/net/src/stats.rs`, which took it from the reference. The *number*, the *routing* and
  the *effect id* are read out of this client.
