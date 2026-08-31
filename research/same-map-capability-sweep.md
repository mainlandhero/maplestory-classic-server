# What else does seeing another player require? The whole set, enumerated

Written 2026-08-31. **No Ghidra** (three sibling agents hold the project lock), **no client
run**, **no `cargo`** (the same agents share `target/`). Everything below was read out of
`client-patched\MapleStory.exe` with `tools/reads.py` and `tools/listing.py`, out of the
pre-made dumps in `research/`, out of `crates/`, and out of the archived runs in
`previous-runs/` and `research/fixtures/`.

Tags on every claim: **[L]** read off a listing, a table, a capture or the source;
**[D]** derived from two or more of those; **[I]** inferred.

This document does **not** re-do the three packets the sibling agents own — `0x0293`
movement, `0x0224`/`0x0225` enter/leave, `0x029E..0x02A1` attacks. It enumerates the space
those three sit in and says what is missing from it.

---

## 0. The three answers that matter most

1. **The claim that no user-pool packet has ever worked is false, and it is written down in
   three places.** `0x02D1` and `0x0315` are both in the user-pool range, both have gone out
   of this server hundreds of times, and both are recorded **working on the owner's screen** in
   `STATUS.md`. Section 2. This changes what the first two-client run is testing.
2. **Two new remote opcodes are identified [L] by reachability with five reproduced positive
   controls**: `0x02AE` redresses a remote player (avatar look) and `0x02B0` puts buffs on
   one (`CSecondaryStat::DecodeForRemote`). Two more are **[D]**: `0x02B1` clears buffs and
   `0x02AD` seats a remote player. Section 4.
3. **The most visible two-client bug is not a missing packet at all.** Every session that
   enters a field is handed `MOB_CHANGE_CONTROLLER` for **every mob on it**, with no
   registry anywhere deciding who controls what. Two players on one map means two
   controllers per mob, both running the wander locally and both reporting `0x02FF`.
   Section 6.

---

## 1. The instruments, and the controls they reproduced first

`CLAUDE.md`: *"Verify the instrument before believing it."* All four run from the repo root.

| instrument | control | result |
|---|---|---|
| `python tools/reads.py 0x140304100 2` | the documented control in `tools/listing.py`'s own docstring | raw @`140304138`, u8 @`140304144`, u8 @`140304183`, then a run of u16 — **reproduced** |
| the reachability sweep (below) | `FUN_1427847a0` reached only from `0x0226` | **reproduced** |
| the same | `FUN_140F32200` (attack header) reached only from `0x029E..0x02A1` | **reproduced** |
| the same | `FUN_14025DA80` (HITINFO) reached only from `0x02A5` | **reproduced** |
| the same | `FUN_1404B2630` (path decoder) reached only from `0x0293` and `0x02F5` | **reproduced** — this is `research/user-pool-tables.md` §3's headline answer, re-derived |
| the same | `FUN_1429ba3e0`, the `0x0224` handler, reaches **all four** of AVATARLOOK / SECSTAT / CHAIROBJ / PETDEC | **reproduced** — `research/user-enter-field.md` names all four inside `CUser::Init` |
| the archive sweep | `<- 0x02FF` = 164 117 distinct events, `-> 0x046E` = 2 005, `-> 0x0070` = 2 595 | **reproduced** |

The last control is the one that makes section 2's negative worth anything: an archive
search that can find 164 117 of something can be believed when it finds two of something
else.

**The scratchpad was not on `sys.path`.** Every script was piped in — `python - < script.py`
— which leaves `sys.path[0]` empty, so the scratchpad's stale `reads.py` / `listing.py`
could not shadow `tools/`. `CLAUDE.md` § "The scratchpad shadows the real tools".

**The archive was deduplicated on events, not files.** `previous-runs/` and
`research/fixtures/` overlap and a fixture is copied while its run is still being written,
so a content hash is not enough either. The key is `(timestamp, direction, opcode,
first 120 bytes of the line)`. 444 files scanned. `CLAUDE.md` § "deduplicate the EVENTS".

### The sweep itself

For each of the 419 `(table, opcode, handler)` rows in
`research/msexe-userpool-tables-handlers.txt` — **315 distinct handler functions**, the same
number `research/user-pool-tables.md` §3 reports, which is the check that this walk covers
the same set — walk the call graph to depth 6 over `reads.calls_of`, which sees a tail `jmp`
as well as a `call`, and report which target functions are reachable. Enumerate, then
classify; do not search for the opcodes you expect.

**The named blind spot.** `calls_of` cannot see a call through a vtable slot. `CUser::Init`
decodes the chair object by `o->vtable[0x18](o, pkt)` (`research/user-enter-field.md` §6),
so a handler that reached one of these decoders *only* virtually is invisible here. Every
negative below inherits that. **[D]**

---

## 2. The route into the user pool is proved live ON SCREEN — and three files say it is not

### What the archive actually contains

Deduplicated on events, over 444 archived logs, every opcode in `0x0224..0x039F` that has
ever appeared in either direction: **[L]**

| direction | opcode | distinct events | what it is |
|---|---|---|---|
| `->` | **`0x02D1`** | **227** | `UserEffectLocal` — quest-clear fanfare (effect 15) and the blue recovery number (effect `0x41`) |
| `->` | **`0x0315`** | **9** | `ShowReviveDialog` |
| `->` | `0x0231` | **2** | `UserChat` — 2026-08-19 02:07:34 *and* 2026-08-29 05:33:03 |
| `<-` | `0x0226` `0x0238` `0x024D` `0x02B2` `0x02DE` `0x02EB` `0x02F4` `0x02F5` `0x02F6` `0x02FF` `0x0327` `0x032C` | 37 … 164 117 | client → server; a **different namespace**, per `crates/net/src/userpool.rs`'s own warning |

`0x02D1` and `0x0315` are both routed by `CField::OnPacket` → `FUN_1429b9300` → the local
branch at `0x1429b93f2 call 0x14289a3a0` → table D. Read it in
`research/msexe-userpool-onpacket-1429b9300.txt`. **[L]**

And both are recorded working on the owner's screen, in `STATUS.md`: **[L]**

* line 608 — *"the quest completion SFX is now working"* — `0x02D1` effect 15
* line 611 — *"the dialog appears, including for a character who logged in already dead"* — `0x0315`
* line 615 — *"I do see 10 in blue above the character"* — `0x02D1` effect `0x41`

`USER_EFFECT_LOCAL` has **five production call sites** in `crates/world/src/session/`
(`npc.rs` ×3, `recovery.rs`, `regen.rs`). This is not a one-off.

### The three sentences that are wrong

**`crates/net/src/userpool.rs`, lines 19-22:**

> *"**No packet in `0x224..0x39F` has ever been observed to do anything.** The only one ever
> sent by this server is a single `0x0231` on 2026-08-19 (`world-20260819-220806.log`, …),
> and it drew nothing."*

Wrong three ways: `0x02D1` (227 events), `0x0315` (9 events), and `0x0231` went out **twice**
— the second is `previous-runs/world-20260829-013454.log` line 8514, character 214
(`Tester2`) saying `"hello"`, *after* the balloon-flag fix in `research/user-chat-round2.md`
landed. Nothing in `STATUS.md` records whether that one drew a balloon, so it is unmeasured
rather than negative. **[L]**

**`crates/world/src/session/multiplayer.rs`, module docs:**

> *"**Nothing here has been on the wire.** No packet in `0x224..0x39F` has ever been observed
> to do anything in any archived run, so every claim below is static."*

Same claim, same falsification. It also drives the file's advice about what the first run
should watch.

**`research/revive.md`, lines 30-32:**

> *"They see no dialog because `0x0315` never did — **nothing in `crates/` sends it, and
> nothing in `crates/` knows the opcode exists.** [L] (`grep -rn "0x0315" crates/` is
> empty.)"*

True the day it was written; false since 2026-08-22, when revive was built and confirmed.
The file needs a SUPERSEDED marker, not an edit — it is the only place the reasoning
survives.

### Why this matters for the two-client run rather than being pedantry

The hop that was never proved is now three hops, and only the last one is open:

```text
CField::OnPacket 141821e24   0x224..0x39F -> the pool singleton    PROVED (0x02D1, 0x0315)
FUN_1429b9300    1429b9372   0x2C5..0x39E -> FUN_14289a3a0         PROVED (0x02D1, 0x0315)
                             gated on [pool+0x10] != 0 (localUser) PROVED - it is non-null
FUN_1429b9300    1429b935e   0x293..0x2C4 -> FUN_1429bb720         same block, same function
FUN_1429bb720    1429bb745   u32 charId, then GetUser(charId)      *** UNPROVED ***
```

So the whole remote family rests on exactly one question: **does `0x0224` put a `CUser` in
the pool under our id?** Not on the routing, not on the framing, not on the field
dispatcher. `research/user-enter-field.md` §5 already names the six gates in front of that
allocation, and `crates/world/src/session/multiplayer.rs` already says to watch
`0x1429ba60b`. That watch is now the *only* thing the first run has to establish; everything
else in the chain has drawn on screen. **[D]**

The same applies to chat, which is where it bites hardest — see the table's top row.

---

## 3. THE TABLE — what a second player would notice, ranked

Ranked by what the owner sees first with two clients on one map. "Decoded" means a body layout
exists somewhere in `research/`; "built" means `crates/net` can produce the bytes; "wired"
means something in `crates/world` actually sends them.

| # | capability | packet(s) | decoded | built | wired | what it would cost |
|---|---|---|---|---|---|---|
| 1 | **Mobs obey two masters** | `0x03D2` `MobChangeController` — *sent, and wrongly* | n/a | yes | **yes, to everyone** | A controller registry in `Fields`. Not a missing packet: `session/field.rs:120` and `session/combat.rs:516` grant `CONTROL_NORMAL` for every mob to **every** arriving session. Two players = two simulations of every mob. **[L]** |
| 2 | **Chat over another player's head** | `0x0231` (`net::userchat::USER_CHAT`) | `research/user-chat-round2.md`, incl. the balloon flag bit | **yes** | **no — local echo only** | One `Bus::publish` in `gm.rs::say_out_loud`, which today returns the line to the speaker alone. The cheapest row in this table. **[L]** |
| 3 | **Mobs move on the observer's screen** | `0x03D9` `MobMove` | `crates/net/src/mobmove.rs` | **yes, tested** | **no** | One `Bus::publish` in `on_mob_move`. Its doc block says the blocker is that "this server has no field-occupancy registry" — that stopped being true on 2026-08-29. **[L]** |
| 4 | **Drops appear and disappear for everyone** | `0x046E` `DropEnterField`, `0x046F` `DropLeaveField` | `research/item-drop.md`, `research/drop-placement.md` | **yes** | **no** | The drop *table* is already shared (`Fields::with_drops`); only the packets are per-connection. Publish the enter on a kill and the leave on a take. **[L]** |
| 5 | **Item flying into another player's hand** | `0x046F` leaveType **2** (`drop_picked_up_by_character`) | `crates/net/src/drops.rs`, `research/pick-up-latch.md` | **yes** | **no** | Same publish as row 4. The builder's own `what` string already says *"Broadcast this to the field INCLUDING the picker"* and nothing does. **[L]** |
| 6 | **A remote player being hit by a mob** | **`0x02A5`** `UserHitRemote` | `research/user-hit.md` §5.3 — `u32 charId` + the client's own 147-byte HITINFO | **no builder** | **no** | Echo the inbound `0x00E5` body behind the `charId`. Require **≥ 147**, never `== 147` (`user-hit.md` §8.5). `crates/net` has no HITINFO type and `userhit.rs` has only the inbound parser. **[L]** |
| 7 | **A remote player levelling up** | **`0x02AF`** `UserEffectRemote`, `u32 charId, u8 effect`, effect 0 | `research/level-up.md` §6 | **yes** — `net::userpool::user_effect_remote` | **no — zero call sites in `crates/world`** | One `Bus::publish` beside the existing level-up. Five bytes. The player's own animation is client-side from `0x007C`, so this is *only* for observers. **[L]** |
| 8 | **A remote player changing equipment** | **`0x02AE`** — see §4.1 | **this document, [L]** | no | no | `u32 charId`, `u8`, then the same compact avatar look `net::opcode::avatar_look` already builds, then ~14 gated reads. The most expensive body in this table. **[L]/[D]** |
| 9 | **Buffs showing on a remote player** | **`0x02B0`** set — see §4.2 | **this document, [L]** | partly — `net::buff::stat_mask` builds the 124-byte mask | no | `u32 charId` + the 124-byte mask + values + `u16` + `u8`. **The value list is `CSecondaryStat::DecodeForRemote`'s SHORT list, not `0x007D`'s** — `research/msexe-secondarystat-remote-140a46e50.txt` says so in its first line. Copying the local encoder is the trap. **[L]** |
| 10 | **Buffs ending on a remote player** | **`0x02B1`** reset — see §4.3 | **this document, [D]** | partly — same mask builder | no | `u32 charId` + a 124-byte mask + `u8`. No values. **[D]** |
| 11 | **Emote / face expression** | **`0x02A6`** — see §4.5 | **this document, [D]** | no | no | `u32 charId, u32 emotion, u32 duration, u8 flag`. Three fields; the handler is 105 bytes. Local twin `0x02C5`. **[D]** |
| 12 | **Sitting on a chair** | **`0x02AD`** — see §4.4 | **this document, [D]** | no | no | `u32 charId, u32, u32, u8?`. Reaches the same chair/miniroom factory the enter-field body's offset 435 uses — so it is chair **or** miniroom and this sweep cannot separate them. **[D]** |
| 13 | **Mobs dying on the observer's screen** | **unknown** — `crates/net` has no mob-leave opcode at all | no | no | no | Unenumerated. A killed mob is removed from the shared table but no packet tells the other client. Do not guess an id. |
| 14 | **A remote player dying** | **unknown** | no | no | no | No handler in the pool was identified as a death. In this client the tombstone is client-side off `0x007C` for the *local* player (`research/user-hit.md` §6.2); whether an observer gets anything at all is unestablished. Say "unknown", not a number. |
| 15 | **Pets / familiars on a remote player** | **no handler in the whole pool** | n/a | no | no | The sweep found `FUN_141EB9760`, the pet decoder, reachable from `0x0224` (the control) and from **no table handler at all**. So a remote pet arrives only inside the enter-field body, or through a vtable this walk cannot see. **[D]** |

### The precondition every one of rows 6-12 shares, and it is not obvious

Everything in `0x0226..0x02C4` is dispatched **after** a `CUserPool::GetUser(charId)`, and a
null result returns having done nothing (`research/user-chat-round2.md` §1 step 4). **[L]**

That includes row 2. **`0x0231` chat is dropped in silence unless the speaker is already in
the receiving client's pool** — which for the *local echo* means the client's own `CUser`
must answer to our character id, and nothing has ever established that it does. So the two
archived `0x0231` sends have a second explanation besides a body bug, and it is the one
`user-chat-round2.md` §"gate B" already names.

The practical consequence: **broadcast chat to other players is now more likely to work than
the local echo ever was**, because `0x0224` is wired and a remote speaker really will be in
the receiver's pool. That inverts the order this feature was tried in.

### One correction to `crates/net/src/userpool.rs`'s `USER_CHAT` doc block

> *"**It is the one opcode in `0x0226..0x0292` that survives a null `GetUser`** … which is
> why `research/user-chat-round2.md` recommends `0x0231` over it until a player is really in
> the pool."*

The first half is right and the causal link is backwards. Surviving a null `GetUser` is an
argument **for** `0x0226`, not against it. `user-chat-round2.md` recommends staying on
`0x0231` for unrelated cost reasons — *"it costs an unexplained `u32`, an unexplained
discarded string and a `% 27` test"* — and lists `0x0226` as *"a live alternative"*. Worth
fixing because the sentence as written would stop someone reaching for `0x0226` in exactly
the situation it was designed for.

---

## 4. The new identifications, with their evidence

Each of these is a reachability result from the sweep in §1, which reproduced five
independent positive controls before any of them were believed.

### 4.1 `0x02AE` is `UserAvatarModified` — redress a remote player **[L]**

Table C, handler `FUN_1429d5290`, 888 bytes across 7 merged `.pdata` entries.

**Of 315 distinct handler functions, exactly two reach `FUN_1402EE8D0`** — the compact
avatar-look reader that `0x0107`, `0x0114`, `0x0138` and `CUser::Init` all use, and that
`net::userpool::USER_ENTER_FIELD_LOOK_AT` is the offset of:

```text
table C  opcode 0x02AE  handler 0x1429d5290   <- REMOTE
table D  opcode 0x0331  handler 0x14290d4e0   <- LOCAL
```

A remote/local pair, exactly like `0x02AF`/`0x02D1` and `0x0293`/`0x02F5`. The read walk:

```text
off  read at        field
  0  1429bb745      u32 charId       <- FUN_1429bb720, before the dispatch
  4  1429d52b6      u8               a flag or a count; NOT gated
  5  1429d5363      call 0x1402ee8d0 -> the compact avatar look (u8, u32, raw)
 ...  1429d53dd onward: 14 further read sites, EVERY ONE of them gated
```

The tail is the expensive part and it is not decoded here. **[L]** for the identification
and the first two fields; the tail is **unread**.

### 4.2 `0x02B0` sets a remote player's buffs **[L]**

Table C, handler `FUN_1429d62f0`, 516 bytes, three read sites and no more:

```text
1429d6326  call 0x140a46e50   -> raw(124) then u16/u32/u8 values
1429d632e  READ u16
1429d6339  READ u8
```

`FUN_140A46E50` is **`CSecondaryStat::DecodeForRemote`** — the name is in
`research/msexe-secondarystat-remote-140a46e50.txt`'s first line, and it is the same
function `net::userpool::REMOTE_STAT_MASK_LEN`'s 124 bytes are read by inside
`CUser::Init`. **It is reached by exactly one table handler in 315: this one.** **[L]**

So the body is `u32 charId`, the 124-byte mask, the values the mask selects, a `u16` and a
`u8`. **[L]** for the shape; the meaning of the trailing `u16`/`u8` is unread.

**The trap, and it is the one this repo keeps paying for.** The remote decoder's value list
is *short* — the file's own subtitle is *"same 124-byte mask, short list"*. `net::buff`'s
`temporary_stat_set` encodes for the **local** `0x007D` with a 64-byte tail. The mask
builder `net::buff::stat_mask` is reusable; the value encoder is **not**, and a value list
copied from the local side desynchronises a body with no length prefix in it anywhere.

### 4.3 `0x02B1` clears a remote player's buffs **[D]**

Table C, handler `FUN_1429d6500`. It does not reach `DecodeForRemote`. What it does:

```text
1429d6581  lea  r8d,[rbx + 0x7c]        ; rbx is 0 here -> 124
1429d658f  call 0x1406e9170             ; READ raw, 124 bytes
1429d65e0  cmp  dword [rax + rdx*4], ebx   ; walk 30 dwords of the mask
1429d65ee  cmp  dword [rax + 0x78], ebx    ; ... and the 31st = 124 bytes exactly
1429d65f6  READ u8
```

A 124-byte mask with no value list, walked bit by bit, then one byte. That is a **reset**,
and it sits at `set + 1` exactly as the local pair `0x007D`/`0x007E` does. **[D]** — the
length and the absence of a value decoder are **[L]**; "reset" is derived from those two
plus the pairing.

### 4.4 `0x02AD` gives a remote player a chair or a miniroom **[D]**

Table C, handler `FUN_1429d4fd0`, 658 bytes, reads `u32 u32 u8?` after the `charId`.

**Of 315 handlers, exactly one reaches `FUN_141712040`** — the factory `CUser::Init` calls
at body offset 435, which `research/user-enter-field.md` labels *"chair/miniroom present"*.
The control passes: `FUN_1429ba3e0` (the `0x0224` handler) reaches it too.

That one label covers two features, and this sweep cannot separate them, so the row says
both. **[D]**

### 4.5 `0x02A6` is the emote, and this is the weakest row here **[D]**

Table B index 8, handler `FUN_1427862e0`, 105 bytes, shared with **local `0x02C5`** — the
same remote/local pairing that `0x02AF`/`0x02D1` has. It reads `u32 a, u32 b, u8 c` and
calls `FUN_14282D710(user, a, b, c)`, whose whole body is:

```text
14282d727  test r9d,r9d / je         ; c == 0 takes the other branch
14282d732  mov  [rcx + 0x1280], edx  ; a  -> a state id on the CUser
14282d741  call [rip+0xa35669]       ; GetTickCount
14282d749  mov  [rbx + 0x1284], eax  ; + max(b,0) -> its expiry tick
```

An id, a duration that becomes an expiry, and a flag that decides whether this is a set.
That is the shape of a timed per-user expression. **[D]**, and the reason it is not [L] is
worth stating: **the reachability instrument failed to discriminate here.** I targeted
`FUN_14282D710` expecting it to be emote-specific and **23 handlers reach it**, including
`0x02A5`. It is a generic `CUser` method. The identification therefore rests on the read
shape plus the local twin, which is the same class of evidence `CLAUDE.md` warns produced
*"a template preload list"* for `0x0467`. Treat it as a candidate.

---

## 5. The complete remote range, all 50 opcodes, classified

`0x0293..0x02C4`. **37 have a handler; 13 are accepted by `CField::OnPacket`, have their
`charId` read and their user looked up, and then fall off both tables into the epilogue.**
They will not freeze the client — the handler returns — but nothing happens. **[L]**

| opcode | tbl | handler | reads (depth 4, `research/user-pool-tables.md` §9) | what it is |
|---|---|---|---|---|
| `0x0293` | C | `0x1429d2e70` | path decoder | **MOVE** — wired ✔ |
| `0x0294`..`0x029D` | – | – | – | **dropped** (10 opcodes) |
| `0x029E`..`0x02A1` | B | `0x1429d2ee0` | u16, header, damage list | **ATTACK** ×4 — wired ✔ (`0x00E2` body attack still unmapped) |
| `0x02A2` | B | `0x1429d4100` | `u32 u8 u16? u8?` | unnamed. Shape matches a skill-prepare (`skillId, level, action, speed`) **[I]** |
| `0x02A3` | B | `0x1429d4390` | `u8 u8 u32? u16? u8?` | unnamed; reaches the skill-effect renderer (`SkillID:%d`, `coverField`) **[D]** |
| `0x02A4` | B | `0x1429d4680` | `u32 u32` | unnamed |
| `0x02A5` | B | `0x1429d48c0` | 147-byte HITINFO | **USER HIT** — decoded, no builder |
| `0x02A6` | B | `0x1427862e0` | `u32 u32 u8` | **emote [D]** §4.5. Local twin `0x02C5` |
| `0x02A7` | B | `0x142786350` | `u32 u32` | unnamed. Local twin `0x02D0` |
| `0x02A8` | B | `0x1429d4f20` | `u32` | unnamed, 32 bytes |
| `0x02A9` | B | `thunk 0x14277e150` | – | unnamed thunk (`msexe-userpool-enter.c` names it; the tables dump has no `.pdata` for it) |
| `0x02AA` | C | `0x1429d6f50` | `u32? u32?` | unnamed |
| `0x02AB` | – | – | – | **dropped** |
| `0x02AC` | B | `0x1429d4f60` | `u32 u32 u32` | unnamed |
| `0x02AD` | C | `0x1429d4fd0` | `u32 u32 u8?` | **chair / miniroom [D]** §4.4 |
| `0x02AE` | C | `0x1429d5290` | `u8`, avatar look, 14 gated | **AVATAR MODIFIED [L]** §4.1. Local twin `0x0331` |
| `0x02AF` | B | `0x1427863f0` | `u8 effect` | **EFFECT** — built, unwired. Local twin `0x02D1` **(this one works on screen)** |
| `0x02B0` | C | `0x1429d62f0` | mask+values, `u16`, `u8` | **BUFF SET [L]** §4.2 |
| `0x02B1` | C | `0x1429d6500` | 124-byte mask, `u8` | **BUFF RESET [D]** §4.3 |
| `0x02B2` | C | `0x1429d5610` | `u32 u32` | unnamed. *The client sends a `<- 0x02B2` — 75 archived events — but the two directions are unrelated namespaces* |
| `0x02B3` | C | `0x1429d5710` | `u32 str u16? u8? …` | unnamed; a name or a message |
| `0x02B4` | C | `0x1429d5a40` | `str` | unnamed; one string, 115 bytes |
| `0x02B5` | C | `0x1429d5ac0` | `u16 u8 u16 u8 u32 …` | unnamed |
| `0x02B6` | C | `0x1429d7020` | `u32 u32 u8` | unnamed |
| `0x02B7` | B | `0x1429d5f70` | `u32` | unnamed |
| `0x02B8` | – | – | – | **dropped** |
| `0x02B9` | C | `0x1429d70f0` | **0 reads** | unnamed; body is the `charId` alone |
| `0x02BA` | – | – | – | **dropped** |
| `0x02BB` | B | `0x1429d6010` | `u32 f64 u32 u8` | unnamed — **the only `f64` in the whole remote range** |
| `0x02BC` | B | `0x1429d6100` | `u32` | unnamed |
| `0x02BD` | B | `0x1429d7130` | `u32` | unnamed |
| `0x02BE` | C | `0x1429ddb20` | `u64` | unnamed |
| `0x02BF` | B | `0x1429de1b0` | 10 sites | unnamed; reaches the skill-effect renderer **[D]** |
| `0x02C0` | B | `0x1429deb60` | `u32 u32 u8` | unnamed |
| `0x02C1` | B | `0x1429deae0` | `u32 u8 u32 u32 u8 u32 u32 u32` | unnamed |
| `0x02C2` | C | `0x1429d7170` | `str u16? u16?` | unnamed |
| `0x02C3` | C | `0x1429d7220` | `u8` | unnamed |
| `0x02C4` | B | `0x1429d5d90` | 12 `u32`-ish | unnamed. The one opcode whose table-B arm exits straight past table C |

### Two structural facts that constrain any future guess

* **`0x039F` is routed to the pool and matches none of the four sub-ranges.** `CField::OnPacket`
  tests `cmp eax,0x17b` (→ `0x39F`) but the local branch tests `cmp eax,0xd9` (→ `0x39E`).
  One opcode falls through everything. **[L]**
* **Under throttling, `0x029E..0x02A1` go to `FUN_1429DF530`, which reads nothing.** A remote
  user over 100 packets per second stops having their attacks drawn while everything table C
  handles keeps working. `research/user-pool-tables.md` §2. Relevant the moment a rebroadcast
  storm exists. **[L]**

---

## 6. Outside the user pool: what a second player needs that has nothing to do with `CUserPool`

The user-pool enumeration answers "how do I draw the other player". It does not answer "why
do we see different worlds". Four things, all measured in `crates/`:

### 6.1 Both clients are told they control every mob **[L]**

`crates/world/src/session/field.rs:118-124` sends `MOB_CHANGE_CONTROLLER` with
`CONTROL_NORMAL` for every live mob to **every** session on field entry, and
`session/combat.rs:516` does the same on every respawn. `grep -rni controller
crates/world/src/fields.rs` is **empty** — there is no registry deciding who controls what.

The client that holds control runs the wander locally and reports each path as `0x02FF`.
Two controllers means two independent simulations of the same mob and two `0x02FF` streams
for it. This is the row the owner will see first and it is not a missing packet — it is a packet
being sent to the wrong number of people.

### 6.2 The bus has exactly two production publishers **[L]**

`grep` for `bus().publish` outside tests: `multiplayer.rs:202` (user move) and
`multiplayer.rs:277` (user attack). Plus `enter_field`, `leave_field`, and
`send_to_character` for the EXP share in `combat.rs:607`. **Everything else in this server
is a `Reply` to the connection that asked**, including every packet that describes a
field-wide fact: mob movement, mob death, drops appearing, drops being taken.

### 6.3 Two files still say the registry does not exist **[L]**

* `crates/world/src/session/combat.rs`, `on_mob_move` doc: *"this server has no
  field-occupancy registry: a `Session` is one connection and knows of no other. … Wiring it
  needs the player list that `config::spawn_capacity`'s `players_here = 1` is also waiting
  on."*
* `crates/world/src/session/gm.rs`, `say_out_loud` doc: *"This is a **local echo, not a
  broadcast** … There is nobody else on the field to send it to yet - the server has no
  concept of a second player in a field - and saying so here is cheaper than rediscovering
  it when there is."*

Both were true until 2026-08-29. `crate::broadcast::Bus` is that registry and
`Bus::others_on(id, map)` is that player count. `config::spawn_capacity`'s hard-coded
`players_here = 1` is a third consumer of the same fix.

### 6.4 The state is shared; only the packets are not **[L]**

`Fields` already holds mobs (`fields.rs:166`) and the drop table (`fields.rs:172`) per map
per channel. So two players *do* fight the same mobs and *do* compete for the same drops —
they simply are not told when the other one changes something. That is the cheap direction
to be wrong in, and it means rows 3-5 of the table are each one `Bus::publish`, not a data
model change.

---

## 7. What I did NOT establish

Read this before building on anything above.

1. **Nothing in §4 has been on the wire.** No client run. Every new identification is static.
2. **Vtable dispatch is invisible to the sweep.** `calls_of` reads `call` and tail `jmp`
   only. `CUser::Init` decodes its chair object through `o->vtable[0x18](o, pkt)`, so a
   handler reaching one of these decoders *only* virtually would not appear. Every negative
   in this document — including "only `0x02AE` redresses a player" and "no handler decodes a
   pet" — carries that blind spot. **[D]**
3. **`0x02A6` is a candidate, not a finding.** §4.5 says why, including the instrument that
   failed to discriminate.
4. **The tails of `0x02AE` and `0x02B0` are unread.** 14 gated reads and a `u16`/`u8`
   respectively. Neither is safe to build from what is here.
5. **The 22 unnamed remote opcodes stay unnamed.** Their read shapes are in §5. I did not
   guess ids for chair-vs-miniroom, for death, or for mob-leave, and the brief is right that
   a wrong opcode is how this client dies.
6. **Whether the two archived `0x0231`s drew a balloon is unmeasured**, not negative.
   `STATUS.md` records no outcome for the 2026-08-29 one, and the hook log was not checked
   for a `FUN_142784970` dispatch line — which is exactly the "count the same event in two
   logs" check that would settle it without a launch.
7. **I did not classify table A's 65 remaining handlers.** `0x0226` and `0x0231` are chat;
   the other 63 read shapes are in `research/user-pool-tables.md` §9 and are almost certainly
   trade / miniroom / party / guild, which the brief scopes out. Saying "almost certainly" is
   **[I]** and nothing here rests on it.

---

## 8. Reproducing every number

All from the repo root. Scripts piped in (`python - < s.py`) so `sys.path[0]` is empty.

```
python tools/reads.py 0x140304100 2            # the control - run this FIRST
python tools/reads.py 0x1429d5290 4            # 0x02AE - the avatar look at 1429d5363
python tools/reads.py 0x1429d62f0 4            # 0x02B0 - call 0x140a46e50
python tools/reads.py 0x1429d4fd0 4            # 0x02AD
python tools/listing.py 0x1429d6500            # 0x02B1 - lea r8d,[rbx+0x7c] at 1429d6581
python tools/listing.py 0x1427862e0            # 0x02A6
python tools/listing.py 0x14282d710            # its sink: +0x1280 id, +0x1284 expiry
```

The reachability sweep, over every row of `research/msexe-userpool-tables-handlers.txt`:

```python
import re, sys
sys.path.insert(0, "tools")
import reads as R
R._load("client-patched/MapleStory.exe")
TARGETS = {0x1404B2630:"PATHDEC", 0x140A46E50:"SECSTAT", 0x1402EE8D0:"AVATARLOOK",
           0x140F32200:"ATKHDR", 0x14025DA80:"HITINFO", 0x1427847A0:"CHATBODY",
           0x141712040:"CHAIROBJ", 0x141EB9760:"PETDEC"}
memo = {}
def reaches(fn, depth, stack=frozenset()):
    if fn in TARGETS: return {TARGETS[fn]}
    if depth <= 0 or fn in stack: return set()
    k = (fn, depth)
    if k in memo: return memo[k]
    memo[k] = set(); got = set()
    for _, t, _c in R.calls_of(fn):          # call AND tail jmp
        if t in TARGETS: got.add(TARGETS[t])
        got |= reaches(t, depth - 1, stack | {fn})
    memo[k] = got; return got

# CONTROL FIRST: the 0x0224 handler must reach four of the eight targets.
assert reaches(0x1429ba3e0, 6) >= {"AVATARLOOK", "SECSTAT", "CHAIROBJ", "PETDEC"}

pat = re.compile(r"^([ABCD])\s+(0x[0-9a-f]{4})\s+0x[0-9a-f]{2}\s+0x[0-9a-f]+\s+(0x[0-9a-f]+)")
for line in open("research/msexe-userpool-tables-handlers.txt", encoding="utf-8",
                 errors="replace"):
    m = pat.match(line)
    if not m: continue
    got = reaches(int(m.group(3), 16), 6)
    if got: print(m.group(1), m.group(2), " ".join(sorted(got)))
```

Expected output, in full — `control OK`, then **12 rows**, eight of which are the reproduced
controls. This block was run verbatim; it is a transcript, not a description:

```text
control OK
A 0x0226 CHATBODY        <- control
B 0x029e ATKHDR          <- control
B 0x029f ATKHDR          <- control
B 0x02a0 ATKHDR          <- control
B 0x02a1 ATKHDR          <- control
B 0x02a5 HITINFO         <- control
D 0x02f5 PATHDEC         <- control
D 0x0331 AVATARLOOK      <- the LOCAL twin of the new finding
C 0x0293 PATHDEC         <- control
C 0x02ad CHAIROBJ        <- NEW, section 4.4
C 0x02ae AVATARLOOK      <- NEW, section 4.1
C 0x02b0 SECSTAT         <- NEW, section 4.2
```

The archive sweep, event-deduplicated across `previous-runs/` and `research/fixtures/`:

```python
import glob, re, collections
files = glob.glob("previous-runs/*.log") + glob.glob("research/fixtures/*.log")
pat = re.compile(r"^(\d\d:\d\d:\d\d\.\d\d\d)\s+(->|<-)\s+(0x[0-9A-Fa-f]{4})\s*(.*)$")
ev = collections.defaultdict(set)
for f in files:
    for line in open(f, encoding="utf-8", errors="replace"):
        m = pat.match(line.strip())
        if m:
            ts, d, op, rest = m.groups()
            ev[(d, int(op, 16))].add((ts, rest[:120]))   # the EVENT, not the file
assert len(ev[("<-", 0x02FF)]) > 100000    # positive control: the search works
for (d, op), s in sorted(ev.items(), key=lambda kv: kv[0][1]):
    if 0x0224 <= op <= 0x039F:
        print(d, hex(op), len(s))
```

`444` files, and the only `->` rows inside the range are `0x0231` (2), `0x02D1` (227) and
`0x0315` (9).
