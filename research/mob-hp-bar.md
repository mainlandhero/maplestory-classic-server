# The mob health bar: `0x03F0` carries a PERCENTAGE, and `template+0x100` is not max HP

The owner: *"the Snail should have 45 HP, when I attack it for 18 HP, the health bar is
definitely not at the right percentage."*

Labels: **[L]** read off this client's listing, its `Mob.wz`, or a capture · **[D]**
derived from two or more [L] facts · **[I]** inferred, not established.

No Ghidra, no client run.

---

## The answer in one line

The server's arithmetic is right and the **units are wrong**. `0x03F0`'s `hp` field is not
an absolute HP; it is HP on a **0..100 scale**, and the client already knows the maximum.
For a snail at 27/45 the server must send **60**, not 27. **[L]**

`crates/net/src/combat.rs`'s `mob_hp_change` doc block is wrong in two places and this
document replaces both:

| the doc says | what the listing says |
|---|---|
| *"`template+0x100` is the mob's max HP"* | `template+0x100` is **`hpNoticePerNum`**, and **no mob in this client has it** |
| *"send the mob's true remaining HP"* | send the percentage; the absolute number is what `mob+0x8b4` is *converted into*, not what it holds |

---

## 1. The two template fields, named from the parser

`FUN_14047d990` is the `Mob.wz` template parser (44 762 bytes). Each property is a
`FUN_1401e4330(node, &out, name)` lookup on a name string, followed by a store into the
template. Reading the name beside each store: **[L]**

| offset | width | `Mob.wz` `info/` property |
|---|---|---|
| `+0x20` | **qword** | **`maxHP`** — `140480346 mov [r13+0x20], rax` |
| `+0x60` | dword | (the id `FUN_1404afb40`/`FUN_1404af330` override table is keyed on) |
| `+0x81` | byte | **`boss`** |
| `+0xfc` | dword | **`broadcastMobHP`** |
| `+0x100` | dword | **`hpNoticePerNum`** |
| `+0x105` | byte | **`HPgaugeHide`** |
| `+0x17c` | byte | **`partyBonusMob`** |
| `+0x380` | dword | **`HPgaugeShow`** |

`maxHP`'s name resolves through a global string slot
(`14048030b mov rdx,[rip+0x35c7d4c]` -> `0x143A48050` -> `0x1432b1660` = UTF-16 `maxHP`,
with `maxMP` and `flySpeed` following it in the same literal pool). The gate names resolve
the same way or from a direct `lea`. **[L]**

**So `template+0x20` and `template+0x100` are not two spellings of one number. They are
`maxHP` and `hpNoticePerNum`, two different `Mob.wz` properties.**

### `hpNoticePerNum` does not exist in this client — enumerated, not searched

`target/release/wz-dump.exe cat` over **all 193 images** in
`client-patched/Data/Mob/Mob_000.wz`, counting every key that appears in any `info` node:

```
  level                   193      elemAttr                141      revive       6
  maxHP                   193      undead                   89      noregen      6
  maxMP                   193      attack                   74      invincible   5
  bodyAttack              193      mpRecovery               41      thumbnail    3
  PADamage                193      link                     37      notAttack    1
  PDDamage                193      hpRecovery               36      noFlip       1
  MADamage                193      skill                    28      PDRate       1
  MDDamage                193      firstAttack              15      MDRate       1
  acc                     193      mobType                  13      fixedDamage  1
  eva                     193      boss                     13      selfDestruction 1
  pushed                  193      flySpeed                 10      default      1
  summonType              191      category                  9
  fs                      187      speed                   172
  exp                     184      hideLevel               183
```

**39 keys enumerated, and `hpNoticePerNum`, `broadcastMobHP`, `hpTagColor` and
`hpTagBgcolor` are not among them.** That is the shape `CLAUDE.md` demands: the same scan
that returns zero for `hpNoticePerNum` returns 193 for `maxHP` and 13 for `boss`, so the
zero is a property of the data and not of the search. **[L]**

Template 2, the snail, in full:

```
MADamage 0   MDDamage 0   PADamage 3   PDDamage 0   acc 33   bodyAttack 1   eva 0
exp 2   fs 10.0   hideLevel 0   level 1   maxHP 45   maxMP 30   pushed 1
speed -65   summonType 1   undead 0
```

`maxHP = 45`, exactly what `gm-handbook/mobtemplates.txt` and `tools/dump_mobs.py` already
carry. **`tools/dump_mobs.py` reads the right property and needs no change.** **[L]**

---

## 2. `mob+0x8b4` is a percentage, and the constructor proves it

```asm
141c4eb57  mov      rax, [rsi + 0x3a8]        ; the template
141c4eb5e  mov      ecx, [rax + 0x100]        ; hpNoticePerNum
141c4eb64  test     ecx, ecx
141c4eb66  je       141c4eb72
141c4eb68  movd     xmm0, ecx / cvtdq2pd      ;   present -> use it
141c4eb70  jmp      141c4eb7a
141c4eb72  movsd    xmm0, [rip+0x16292fe]     ;   absent  -> 100.0
141c4eb7a  cvttsd2si eax, xmm0
141c4eb7e  mov      [rsi + 0x8b4], eax        ; current
141c4eb84  mov      [rsi + 0x8b8], eax        ; the maximum on the same scale
141c4eb8a  mov      dword [rsi + 0x8bc], -1
```

The constant at `0x141c4eb7a + 0x16292fe = 0x143277E78` is **`100.0`**, read out of the
image. **[L]** So a freshly constructed mob has `mob+0x8b4 = mob+0x8b8 = 100`, and it is
full — which is only true if the field is a percentage.

`FUN_141c83440`, the `0x03F0` handler, is the **only** other writer of that field
(`tools/rangescan.py 0x8b4 0x141c40000 0x141d60000` gives 13 sites; two are stores, the
constructor's and `141c83460`). Its positive control — `+0x2f4` returning
`141c4d261`/`141c4e6ee` in the constructor — reproduces, as
`research/mob-behaviour.md` §1 documents. **[L]**

### Seven readers, one formula

Every reader of `mob+0x8b4` in the mob code range converts it out of that scale, and none
of them uses it as an absolute number: **[L]**

```asm
; FUN_141cbb560 / FUN_141cbb5d0 / FUN_141c755d0 / FUN_141cd2bf0  - "give me absolute HP"
   scale = template->hpNoticePerNum ? (double)hpNoticePerNum : 100.0
   call  FUN_141c8a730                 ; maxHP, scaled by mob+0x8c0
   movsxd rcx, [mob + 0x8b4]
   imul  rax, rcx / cvtsi2sd / divsd xmm0, scale
   ->  absoluteHP = maxHP * mob[0x8b4] / scale

; FUN_141cbb320 / FUN_141cbb3d0 / FUN_141cbb4c0                  - "give me a percentage"
   ecx = template->hpNoticePerNum
   if ecx == 0:  return mob[0x8b4]                  ; already a percentage
   else:         return mob[0x8b4] * 100.0 / ecx
```

`FUN_141c755d0` (`141c757c2`) and `FUN_141cd2bf0` (`141cd374c`) each compute the absolute
number **and** `maxHP` and format both into one string — a `current / max` display. **[L]**

And the consumer end closes it: `FUN_1418fc730`, the only caller of `FUN_141cbb4c0`,
averages the value over a group of mobs and then does

```asm
1418fc887  divsd  xmm1, [rip+0x197b5e9]     ; 0x143277E78 == 100.0
1418fc88f  movd   xmm0, [rsi + 0xf0]        ; a pixel width
1418fc89b  mulsd  xmm1, xmm0
1418fc89f  cvttsd2si edx, xmm1
1418fc8b2  call   [vtable + 0x1a8]          ; set the gauge width
```

**value / 100 * width.** The number `mob+0x8b4` carries is drawn as a percentage. **[L]**

---

## 3. Why the two divisors in `combat.rs` looked like a contradiction

They are not the same number and they never were:

| | field | property | for template 2 |
|---|---|---|---|
| the **spawn** (`0x03C6`) divisor, `FUN_141c8a730` -> `FUN_14047a100` | `template+0x20` | `maxHP` | **45** |
| the **`FUN_141cbb320`** divisor | `template+0x100` | `hpNoticePerNum` | **0** |

`FUN_14047a100`:

```asm
14047a10a  mov  edi, [rcx + 0x60]
14047a112  call 0x1404afb40           ; is there an override for this id?
14047a119  je   14047a12f
14047a11d  call 0x1404af330           ;   yes -> a dword from an override table
14047a12f  mov  rax, [rbx + 0x20]     ;   no  -> the template's maxHP qword
```

The **same** `FUN_1404afb40` predicate appears in the parser: when it is true the parser
writes `template+0x20 = 0` and skips the `maxHP` property entirely
(`1404802e3 call 1404afb40 / test al,al / jne 140480374`, and `140480374 mov [r13+0x20], r12`
with `r12 = 0`). So the two halves are consistent: either the WZ supplies `maxHP` or the
override table does. **[L]**

`FUN_141c8a730` then scales by `mob+0x8c0`, the spawn body's field 46 "HP scale %", unless
it is 0 or 100 — and this server sends 100. **[L]**

---

## 4. What actually happens today, end to end

| step | what the client does |
|---|---|
| constructor | `mob+0x8b4 = mob+0x8b8 = 100` (because `hpNoticePerNum` is absent) |
| `0x03C6` spawn, body offset 50, `u64 hp = 45` | `141c50502 idiv` -> `mob+0xb60 = 45*100/45 = 100`. **Does not touch `mob+0x8b4`.** |
| `0x03F0`, `hp = 27` | `141c83460 mob+0x8b4 = mob+0x8bc = 27` — i.e. **27 %** |

So the bar reads **27 %** where it should read **60 %**. `27` and `60` are both plausible
on screen, which is exactly why "definitely not at the right percentage" is the report
rather than "the bar does not move". **[D]**

### The `0x03F0` fork, and which mobs even reach the boss gauge

```asm
141c83458  call 1406e8c20                 ; u32 hp
141c83460  mov  [rdi+0x8b4], eax          ; UNCONDITIONAL
141c83466  mov  [rdi+0x8bc], eax
141c8346c  call 1406e8ae0                 ; u8 showBar -> sil
141c8347c  mov  rcx, [rdi+0x3a8]          ; the template
141c83483  cmp  byte [rcx+0x81], 0  / je   141c834c7   ; boss           == 0 -> floating number
141c8348c  cmp  byte [rcx+0x105], 0 / jne  141c834c7   ; HPgaugeHide    != 0 -> floating number
141c83495  cmp  dword [rcx+0x380], 0/ jne  141c834c7   ; HPgaugeShow    != 0 -> floating number
141c8349e  cmp  byte [rcx+0x17c], 0 / jne  141c834c7   ; partyBonusMob  != 0 -> floating number
141c834a7  call 140479ea0 / test al,al / je 141c83521  ; a global check -> return
141c834c2  jmp  141cd7b40                                ; the BOSS GAUGE recompute
```

A snail has no `boss` node, so `template+0x81 == 0` and **`0x03F0` never reaches
`FUN_141cd7b40` and never touches `mob+0xb60`.** The other arm pushes `mob+0x8b4` into the
tick-keyed map at `mob+0x6d8` and stamps `mob+0x6c0` when `broadcastMobHP != 0` **or**
`showBar` is set — the timed HP/damage display over the mob. **[L]**

Which means the `showBar` byte `mob_hp_change` already sends as `true` is doing real work:
with `broadcastMobHP` absent on every template, it is the only thing that makes the display
appear at all. **Keep sending `true`.** **[L]** for the branch, **[D]** for the advice.

`mob+0xb60` is the **boss** gauge — `UI/BossMobHP.img/MobGage/...`, string ids
`0x08ED`..`0x08F3` — seeded at spawn from the absolute HP and updated only on the boss path.
For a non-boss it is written once and never read. **[D]**

---

## 5. What the server should send

```
percent = round(hp_after * 100 / max_hp)      clamped to 1..=100 while the mob is alive
```

* **Clamp the low end to 1.** `cvttsd2si` truncates, and a mob on 1/45 gives `2` with
  rounding but `0` if anyone changes the arithmetic to floor with small numbers. A `0`
  reads as an empty bar on a living mob.
* **Clamp the high end to 100.** `mob+0x8b8` is 100 and nothing re-derives it.
* **Do not change the spawn packet.** `0x03C6`'s `hp` is genuinely absolute and 45 is right
  there; it feeds `mob+0xb60` through `maxHP`.
* **Do not send `0x03F0` for a mob that died.** Unchanged from `mob_hit_replies` — the mob
  is being torn down.
* `max_hp` comes from `world::config::MobTemplate::max_hp`, i.e. `tools/dump_mobs.py`'s
  `maxHP`, which is the right property. Nothing about the dump changes.

### Wire it like this

`crates/net/src/combat.rs` is **not** mine to edit and I have not touched it. The change is
one function and one doc block, and it belongs to whoever owns `combat.rs`:

```rust
/// `hp` is a PERCENTAGE on a 0..100 scale - see research/mob-hp-bar.md.
pub fn mob_hp_change(object_id: u32, hp_percent: u32, show_bar: bool) -> Vec<u8> { … }

/// Convert absolute HP to the percentage `0x03F0` carries.
pub fn hp_percent(hp_after: u64, max_hp: u64) -> u32 {
    if max_hp == 0 || hp_after == 0 { return 0; }
    (((hp_after * 100 + max_hp / 2) / max_hp).max(1)).min(100) as u32
}
```

and in `mob_hit_replies`, `mob_hp_change(object_id, hp_percent(hit.hp_after, max_hp), true)`
— which means `mob_hit_replies` needs the template's `max_hp` passed in. That is the only
signature change, and it is the reason this is a "wire it like this" and not an edit: the
call site is in `crates/world/src/session/`.

**The check on the next run:** snail, one hit for 18. Right = the bar at roughly three
fifths. Wrong-in-the-old-way = about a quarter. Wrong-in-a-new-way = full or empty, which
would mean the percentage is being clamped or the mob is not the one being hit.

---

## 6. What I did NOT establish

* **What draws the small bar over an ordinary mob.** I found the *boss* gauge renderer
  (`FUN_1418fc730` -> `FUN_141cbb4c0`, `value/100 * width`) and the timed list
  `mob+0x6d8` / `mob+0x6c0` that the non-boss arm feeds. I did not follow `mob+0x6d8` to a
  drawing call, so "the bar the owner sees is fed by `mob+0x8b4`" rests on `mob+0x8b4` being the
  only HP the non-boss path writes, not on reading the renderer. **[D]**
* **`FUN_140479ea0`** — the global predicate on the boss path. Unread.
* **`mob+0x8bc`**, written to the same value by `0x03F0` and initialised to `-1` by the
  constructor. Unread.
* **`FUN_1404afb40` / `FUN_1404af330`**, the `maxHP` override table keyed on
  `template+0x60`. I read that they exist and gate `template+0x20`; I did not read what is
  in the table or whether any mob in this client hits it. If one does, its `maxHP` is not
  the WZ number and `mobtemplates.txt` would be wrong for that mob alone.
* **`hpNoticePerNum`'s purpose.** Absent everywhere here, so it is untestable against this
  client; the name and the `FUN_141cd7b40` string-building path suggest an HP *notice*
  granularity rather than a bar, and that is **[I]**.
* **Whether any capture shows a `0x03F0` at all.** The change is derived from the listing;
  nothing in `research/fixtures/` contains one.
