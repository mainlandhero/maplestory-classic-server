# Quest state: where it lives in the character record, and the packet that changes it

**Established 2026-08-19, statically, from `client-patched/MapleStory.exe`.** Nothing here
has been in front of the client yet; the last section says exactly what a run would settle.

Labels, as everywhere in `research/`: **[L]** read off this client's listing, **[D]** derived
from two or more [L] facts, **[I]** inferred (including anything from the v214 reference,
which is a different game version and scored 1 of 8 against a held-out control).

## The headline

| | |
|---|---|
| **`presence[9]`** | the **started**-quest list in the character record: `u8 bulk, u16 count, {u32 questId, str progress}` |
| **`presence[14]`** | the **completed**-quest list: `u8 bulk, u16 count, {u32 questId, raw8 completedAt}` |
| **`0x0089`, first `u8` = `1`** | the quest-result packet: `u32 questId, u8 state, <payload by state>` |

The two record blocks go **after the equipped list and before the final ungated `u8`**, in
that order - started first.

> **The character record has no length prefix and no resync point.** One wrong width
> silently desynchronises everything after it, and the visible symptom is an undressed
> character or no world entry at all - **not** a wrong quest list. Every byte below is
> either read off the decoder or read off the client's own *encoder*; nothing in the two
> record blocks is guessed.

## 1. How the blocks were found, and the instrument

Method, in order:

1. Read the two candidate regions of `FUN_140304b20` straight out of
   `research/msexe-charrecord-full.txt` (which is a capstone listing of the whole decoder).
2. Take the non-primitive functions those regions call and ask `tools/callers.py` who else
   calls them. **That is the step that did the work**: one function, `FUN_142d5b750`, calls
   three of them and is reached from the inbound opcode dispatcher.
3. Confirm every helper reads **no** packet bytes, with `tools/reads.py` at depth 3, so the
   block costs exactly the reads in the listing and not one byte more.
4. Cross-check the whole thing against the client's **own encoder** for the same record.

**The instruments were controlled before being believed.**

* `python tools/callers.py 0x1402fa9a0` reports **43 call sites in `0x140304b20`**, which is
  the number `research/charrecord-loops.md` §5 arrived at independently. Positive control
  passes.
* `python tools/reads.py 0x140304100 2` shows the equipped-item decoder reading **both**
  through `FUN_1403035a0`/`FUN_140303b40` **and** directly - the mix its docstring requires.
  Positive control passes.
* The disassembly used here is capstone over the PE, bounded by `.pdata` - not a byte scan
  for `0xE8`. That distinction is the one that shipped a short `0x0231` body on 2026-08-19.

## 2. `presence[9]` - the started quests

Gate at `0x1403074a3`, key `0x143abf360`, guarded region `[0x1403074c4, 0x14030756a)`, **6
reads**, loops #21 and #22. Those numbers match `research/charrecord-loops.md` §5 row `#19`
exactly, which is a second, independently-derived list agreeing. **[L]**

The presence byte, read out of the key's CRT initialiser rather than assumed **[L]**:

```asm
140023690  SUB  RSP,0x28
140023694  XOR  EDX,EDX
140023696  LEA  RCX,[0x143abf360]          ; the key
14002369d  CALL 0x140302c70                ; memset(key, 0, 100)
1400236a2  MOV  byte ptr [0x143abf369],1   ; 0x369 - 0x360 = byte 9
```

The body, read for read. Addresses are where the client calls the primitive:

```text
1403074c7  u8    bulk          -> SETNZ SIL; if non-zero, FUN_1402e0fe0(charData) CLEARS the list
1403074e1  u16   count
   loop, count times:
1403074f3    u32 questId       (thunk 0x1406e8f00)
140307501    str progress      (u16 BYTE count then bytes)
             -> FUN_1402e0c30(charData, questId, &progress, bulk)
   if bulk == 0:
14030753b  u16   removeCount
   loop, removeCount times:
140307553    u32 questId       -> FUN_1402e0ec0(charData, questId)   -- an erase
```

**`bulk` is a length-changing flag with no resync point after it**, exactly like
`hasOverride` in `0x055B`: with `bulk != 0` the trailing `u16` is *not read*.

The three helpers name the collection **[L]**:

| helper | what it does |
|---|---|
| `FUN_1402e0fe0` | clears `charData+0x1273` (a hash map), `+0x128b` and `+0x12a3` |
| `FUN_1402e0c30` | inserts `{key = questId at node+0x10, string at node+0x18}` into `charData+0x1273`; with the 4th argument zero it first diffs the old string and queues the id on `+0x128b` |
| `FUN_1402e0ec0` | erases `questId` from `charData+0x1273` |

`charData` is `world+0x2358` - `FUN_142cbe730` is literally `mov rax,[rcx+0x2358]; ret`, and
that is the same offset `STATUS.md` already records as "`0x00` on the first `SetField` and
non-zero on every later one". **[L]**

## 3. `presence[14]` - the completed quests

Gate at `0x14030757b`, key `0x143abf3d0`, region `[0x140307596, 0x140307633)`, **6 reads**,
loops #23 and #24 - again matching `charrecord-loops.md` row `#20`. Initialiser at
`0x140023670` writes `[0x143abf3de]` - `0x3de - 0x3d0` = byte **14**. **[L]**

```text
140307599  u8    bulk          -> if non-zero, FUN_1402e1020(charData) clears +0x1347/+0x135f/+0x1377
1403075b3  u16   count
   loop:
1403075c8    u32 questId
1403075df    raw(8) completedAt      (R8D = 8, a compile-time constant at 1403075cf)
             -> FUN_1402e33f0(charData, questId, &completedAt, bulk)
   if bulk == 0:
140307608  u16   removeCount
   loop:
140307619    u32 questId  -> FUN_1402e3550(charData, questId)
```

`FUN_1402e3340(charData, questId)` is `isCompleted` and `FUN_1402e3390(charData, questId,
&out)` is `isCompleted + fetch the time from node+0x14`; the second has **31 call sites in 23
functions**, which is the quest UI reading this collection back. **[L]**

**The 8 bytes are a Windows FILETIME. [I].** What is [L] is that it is 8 raw bytes copied
verbatim into `node+0x14` and handed back out as one qword. No call site examined here
formats it, so a wrong value costs a wrong date at worst and **cannot** desynchronise the
record - the width is a compile-time 8 either way. `net::opcode::ITEM_NEVER_EXPIRES` is the
same convention, and it is labelled [I] there for the same reason.

## 4. Why these are quests and not something else

Four independent links, no one of which would be enough alone.

**4a. The client's own encoder writes the same two blocks, keyed on the same two presence
bytes.** `FUN_1402e5a30` is the mirror of `FUN_140304b20` - 38 calls to the same gate
helper `FUN_1402fa9a0` - and it uses a **different key table**. Its started-quest gate is at
`0x1402e719e` with key `0x143abe0d0`, whose initialiser at `0x140022ba0` sets
`[0x143abe0d9]` = byte **9**; its completed-quest gate is at `0x1402e7414` with key
`0x143abe140`, initialiser `[0x143abe14e]` = byte **14**. Two tables, two sets of
initialisers, same two byte indices. **[L]**

And the encoder settles the wire shape beyond argument:

```asm
; bulk started-quest block, FUN_1402e5a30
1402e71c5  MOV   DL,1
1402e71c7  CALL  0x1406ed840          ; write_u8(1)          <- the bulk flag IS 1
1402e71cc  MOVZX EDX,word [r15+0x127f]
1402e71d7  CALL  0x1406ed940          ; write_u16(count)
;   per entry, walking the +0x1273 map:
1402e727e  CALL  0x1406ed9d0          ; write_u32(node+0x10) = questId
1402e728d  CALL  0x1406edc80          ; write_str(node+0x18) = progress
1402e72aa  JMP   ...                  ; and NOTHING else - no second u16
```

```asm
; bulk completed-quest block
1402e7474  MOV   DL,1
1402e7476  CALL  0x1406ed840          ; write_u8(1)
1402e747b  MOVZX EDX,word [r15+0x1353]
1402e7486  CALL  0x1406ed940          ; write_u16(count)
1402e752d  CALL  0x1406ed9d0          ; write_u32(questId)
1402e753f  CALL  0x1406ede20 (R8D=8)  ; write_raw(&node+0x14, 8)
```

Its non-bulk arms write `u8 0`, the add list, **then** a second `u16` and the erase list -
which is precisely the decoder's other branch. Encoder and decoder agree field for field.
**[L]**

**4b. `charData+0x1273` is read by the quest module.** A scan for the 4-byte displacement
`0x1273` finds 120 hits in 89 `.pdata` functions; among them are `FUN_14070fe30` (5 sites),
`FUN_140711b40` and `FUN_140711e50` - the three `.pdata` functions immediately surrounding
**`FUN_140711d70`**, which `crates/net/src/script.rs` already documents as the quest
**requirement checker** that the `0x0151` builder consults. `FUN_141f0e110` is in the same
block as `FUN_141f0e4c0`, the `0x0151` builder itself, and also reads `+0x1273`. **[D]**

> **A displacement scan is a byte-pattern scan and can hallucinate an operand out of data**,
> which is exactly the failure `tools/dataref.py`'s docstring warns about. So the four hits
> this argument rests on were **disassembled individually** at `0x1407100bd`, `0x14071117f`,
> `0x141f0e2c8` and `0x140711ffe`. All four are real `mov reg, [reg+0x1273]` loads, and all
> four are immediately followed by the same `div [reg+0x127b]` bucket lookup - one hash map,
> four readers. **[L]**

**4c. The quest-result packet handler drives the same collections**, and its middle state
calls `FUN_140711d70` by name - see §5.

**4d. The shapes match the packet.** The record's started entry is `{u32 id, str}` and its
completed entry is `{u32 id, raw8}`; the update packet's state 1 carries a string and its
state 2 carries 8 raw bytes, for the same id. **[D]**

## 5. The quest-result packet: `0x0089`, sub-case `1`

**This packet had never been found in either direction.** It is:

```text
opcode 0x0089
u8   1                 the sub-case; FUN_142d43ee0 reads it and indexes a 36-entry table
u32  questId
u8   state             0 = not started, 1 = in progress, 2 = completed
  state 0:  u8   alsoForgetCompletion
  state 1:  str  progress
  state 2:  raw8 completedAt
```

**Routing, all [L].** `FUN_142cbaa80` (the game-stage dispatcher, `0x70..0x39a`) case `0x89`
calls `FUN_142d43ee0`. That function reads one `u8`, bounds it at `0x23`, and jumps through
the table at **`0x142d44a88`** (36 dword RVAs, base `0x140000000`, read straight out of the
PE). Entry `1` is `0x142d43f42`, which is `mov rdx,rsi / mov rcx,r14 / call 0x142d59e20`.

`tools/reads.py 0x142d59e20 3` reports exactly five reads and no others, transitively:

```text
0x142d59e74  READ u32     questId
0x142d59e7f  READ u8      state
0x142d59f23  READ str     (state == 1)
0x142d59f68  READ raw     (state == 2, R8D = r14+6 = 8)
0x142d59f77  READ u8      (state == 0)
```

The three payload reads are mutually exclusive: `cmp r14d,1 / jne`, `cmp r14d,2 / jne`,
`test r14d,r14d / jne` at `0x142d59f15`, `0x142d59f56`, `0x142d59f6f`. **[L]**

### What each state does

`FUN_142d5b750(user, state, questId, &payload, ...)` is the state machine; `edi = state`
forks three ways at `0x142d5b868`. **[L]**

| state | what the client does |
|---:|---|
| **0** | erase `questId` from the started map (`FUN_1402e0ec0`); if the trailing `u8` is non-zero, erase it from the completed map too (`FUN_1402e3550`) |
| **1** | run **`FUN_140711d70`** - the quest requirement checker - then insert `{questId, progress}` into the started map via `FUN_1402e0eb0`, which is `xor r9d,r9d / jmp FUN_1402e0c30`, i.e. the non-bulk insert |
| **2** | insert `{questId, completedAt}` into the completed map (`FUN_1402e33f0`) **and** erase it from the started map (`FUN_1402e0ec0`) |

State 1 reaching `FUN_140711d70` is the single most convincing line in this document: that is
the function `research/npc-dialogue.md` identified from the *other* end, as the requirement
gate the client's own `0x0151` builder consults before choosing tag 1 over tag 6.

### Two preconditions that make it silently do nothing

Both are at the top of `FUN_142d59e20`, and both `return` without touching anything. **[L]**

```asm
142d59e47  MOV  EDX,1
142d59e4c  MOV  RCX,[0x143abea80]
142d59e53  CALL 0x141b1f960          ; test dword [rcx+4], edx / setne al  -- a flag test
142d59e5a  JNE  0x142d5a885          ; bit 0 set -> DO NOTHING
142d59e60  MOV  RCX,R15
142d59e63  CALL 0x142cbe730          ; charData = world+0x2358
142d59e6b  JE   0x142d5a885          ; null -> DO NOTHING
```

* `0x143abea80` is a singleton with **171** references, among them `FUN_141820080`
  (`CField::OnPacket`). What bit 0 of `+4` means is **not established**; only that when it is
  set this packet is a no-op. **[L]** the test, **[I]** any reading of it.
* `charData` is `world+0x2358`, which `STATUS.md` records as measuring **`0x00` on the first
  `SetField`**. So a quest update sent too early is discarded in silence.

**Practical rule, and it is the same shape as the "never send a script with a `SetField`"
rule that already cost a run:** send `0x0089` only in reply to something the client sent from
a settled field - which is what answering `0x00F3` is.

**Nothing here is a blocking request**, so a `0x0089` that vanishes costs a stale journal and
not a frozen UI.

## 6. Where the blocks go in the record

Against the layout in `net::opcode::character_record_for_set_field_with` **as it stands after
the inventory-size block landed** (offset 223, twelve bytes, `presence[7]`):

```text
off  len  what                                   gate
  0  100  presence array                         bytes 0, 2, 7 and now 9 and 14
100   11  head fields                            -
111  108  the character-stat block               presence[0], gate 0x140304e49
219    4  a u8 and three optional-string flags   presence[0]'s region
223   12  six u16 inventory sizes                presence[7], gate 0x140305e29
235   ..  the equipped list (+ 4 terminators)    presence[2], gate 0x1403061a0
  ..   ..  STARTED quests                        presence[9],  gate 0x1403074a3
  ..   ..  COMPLETED quests                      presence[14], gate 0x14030757b
  ..    1  the final ungated u8                  0x140308b3f
```

Order is not a choice: `0x1403061a0 < 0x1403074a3 < 0x14030757b < 0x140308b3f` and the
decoder is a single straight run of gates. **[L]**

**Nothing lies between the equipped list and the started-quest block for us**, and the reason
needs stating carefully rather than waved at, because the obvious version of it is wrong.

* The five **static** gates in `[0x140306632, 0x1403074a3)` key on presence bytes 44, 21, 27,
  8 and 15 - all clear here. **[L]**, from the 40-row table.
* The two **dynamic** gates in that range (`0x1403068fd`, `0x1403069ef`) select among entries
  0-6, whose presence bytes are 0(none), 44, 6, 5, 4, 3 and **2** - and byte 2 *is* set,
  because it is the equipped list's. So **one of them can fire.** What saves it is that
  `0x1403068fd`'s region reads nothing at all, and `0x1403069ef`'s only read - the `u32` at
  `0x140306a0d` inside the fixed-3 loop #9 - is itself behind a *per-turn* switch-selected
  flag whose three entries are 4, 3 and 2, i.e. presence bytes **4, 5 and 6**, all clear.
  Loop #10's count comes from that same read, so it is zero. **[L]**
* And the decisive evidence is not static at all: **presence[2] is already set today and the
  record already decodes on screen** - a dressed character stands on map 1. Whatever those
  two gates do, they do it identically before and after this change.

**Neither 9 nor 14 is reachable through a dynamic gate**, and each appears in exactly one row
of the 40-row table in `research/charrecord-presence-map.md`, so setting them opens nothing
else. **[D]**

**An empty book still costs 6 bytes**, three per block: `01 00 00`. That is deliberate -
`bulk = 1` *clears* the client's collections, so a character who has dropped a quest server
side stops showing it. `bulk = 0` would leave stale entries behind.

## 7. What the server sends, and what is still a decision rather than a measurement

* **`bulk = 1` in the record.** [D] from the encoder, which writes exactly that in its bulk
  arm. The cost is that `FUN_1402e0c30`'s fourth argument is then non-zero, which skips the
  `+0x128b` "these ids changed" queue and the `+0x12a3` bookkeeping - correct for a snapshot,
  since everything in it is new by construction, but **untested on screen**.
* **The progress string is `""` for a freshly accepted quest.** [I]. The client stores
  whatever arrives; nothing read here parses it. If a quest with a kill counter shows no
  progress, this is the field.
* **`completedAt` is a FILETIME.** [I], §3.
* **The quest ids are `u32` on the wire**, not the `u16` this game family usually uses. [L] -
  `0x1406e8f00` is the u32 thunk, in both loops and in the update packet.

## 8. What only a client run can settle

One variant at a time. The record change and the update packet are **separate observables**
and should be separate runs if possible; the record one gates everything, exactly the way
step 0 of the equipment run did.

| what to do | pass | what a failure means |
|---|---|---|
| Enter the world with a character that has **no** quests | character on the map, dressed, as today | **This is the gate.** 6 extra bytes went into the record. A fault or a freeze on "Connecting..." means the block is mis-sized or mis-placed; read the `ELog` (`0x008F`/`0x0090`) and run `tools/pdata_lookup.py` on its RVAs |
| Accept Heena's quest 1000, then open the quest journal | quest 1000 listed as in progress | **Journal empty, no fault** - the `0x0089` was dropped by one of §5's two gates, or `bulk`/`state` is wrong. `world.log` shows whether it went out |
| Relog and re-open the journal | still in progress | **Gone** - the record block is not reaching the collection: `presence[9]` or the block position is wrong |
| Click Heena again | the completion conversation, not the opening one | the server's own state read-back, not a client question |

**A watch that costs nothing and discriminates**: `watch@140307501:peek=24` (the started
block's string read) and `watch@1403075df:peek=24` (the completed block's raw read). Expect
one line per stored quest, and **no line at all** when the character has none - a line with a
count of zero would mean the count `u16` was read as something else.

## 9. Wiring it into `crates/world/src/session.rs`

`session.rs` is the shared integration point and is **deliberately untouched** by this work.
Four edits, all small:

**a. Both `SetField` sites carry the book.** `set_field_with_character_dressed` at lines ~393
and ~815 becomes `set_field_with_character_dressed_quests` with one extra argument:

```rust
let quests = self.claimed
    .as_ref()
    .and_then(|c| self.store.quest_book(c.character_id).ok())
    .unwrap_or_default();
```

Both sites, not one. The bare form at line 815 is what made every item lose its stats after a
`!map` on 2026-08-19; the same trap is live here, and the symptom would be "the journal
empties when you walk through a portal".

**b. `on_quest_request` records the accept.** `QUEST_ACTION_START` (tag 1) *is* the Accept
press - the client has already shown the opening conversation itself. So:

```rust
if req.action == net::script::QUEST_ACTION_START {
    if let Some(id) = self.claimed.as_ref().map(|c| c.character_id) {
        if self.store.start_quest(id, req.quest_id).unwrap_or(false) {
            out.push(Reply { opcode: net::quest::MESSAGE,
                             body: net::quest::quest_accepted(req.quest_id), .. });
        }
    }
}
```

`start_quest` returns `false` if the character already has it, and then **nothing is sent** -
re-announcing an acceptance would reset the client's progress string.

**c. `QUEST_ACTION_COMPLETE` (tag 2) completes it.** `store.complete_quest` returns the unix
time on success and `None` if the quest was not in progress, so the guard and the timestamp
are one call:

```rust
if let Some(at) = self.store.complete_quest(id, req.quest_id).unwrap_or(None) {
    out.push(Reply { opcode: net::quest::MESSAGE,
                     body: net::quest::quest_completed(
                         req.quest_id, net::quest::filetime_from_unix_secs(at)), .. });
}
```

**d. Order the replies: the `0x055B` script first, then the `0x0089`.** Not for the client's
sake - `0x0089` touches nothing the script manager owns - but because the script is the thing
the user is waiting to read.

**What must NOT change:** `req.action` stays the discriminator for which `Say` path to walk.
The client picks tag 1 vs tag 2 from *its own* quest table, and now that the server pushes
`0x0089` that table is right, so re-deriving the path from server state would only add a way
for the two to disagree.

## 10. Things this document does NOT claim

* That the quest journal *UI* reads `+0x1273`. The functions that read it are the quest
  module and the `0x0151` builder; the journal window was not traced. **The claim is about
  the collection the record fills, not about what draws it.**
* That `bulk = 1` is what a real server sends. It is what this client's own encoder writes,
  which is a different and weaker thing - the encoder is used for the client's local state
  dump, not for a server.
* Any meaning for the `0x0089` sub-cases other than `1`. The other 35 are unread.
* That state `1` is called "in progress" anywhere in the binary. The name is [I]; the
  *behaviour* - insert into the started map after a requirement check - is [L].
