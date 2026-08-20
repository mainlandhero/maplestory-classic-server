# Quest progress: what counts a kill, what the client shows, and what an item quest needs

**Established 2026-08-20, statically, from `client-patched/MapleStory.exe` and the client's
own `Quest.wz`. No client run was spent.** Nothing here has been on a wire yet; §9 says
exactly what a run would settle.

Labels, as everywhere in `research/`: **[L]** read off this client's listing or its own data,
**[D]** derived from two or more [L] facts, **[I]** inferred.

Companion documents: `research/quest-state.md` (the record blocks and the `0x0089` packet
itself), `research/npc-dialogue.md` (the outbound `0x0151` and its six tags).

## The headline

| | |
|---|---|
| where requirements live | **already dumped.** `Check.<state>.mob.<n>.{id,count}` and `Check.<state>.item.<n>.{id,count}`, in `gm-handbook/questlines.txt` since 2026-08-19 |
| the progress packet | **`0x0089` sub-case 1, state 1** - the same packet that accepts a quest. There is no separate progress opcode |
| how a count is encoded | a **string**: three zero-padded decimal characters per mob requirement, concatenated in slot order. Four snails is the three bytes `"004"` |
| what the client draws | `#a<questId*10 + slot+1>#` in `QuestInfo.demandSummary`, expanded with the format string **`"%d / %d"`** |
| item quests | **need no running count at all.** The client counts the bag itself, live, every time it checks |

**The thing that was missing is one string.** Everything else - the packet, the record
blocks, the accept - was already built and wired.

## 1. Where the requirements come from - nothing had to be scraped

`Check.<state>` carries them and `tools/dump_quests.py` has been emitting them all along.
The census over all 322 quests, from `gm-handbook/questlines.txt`: **[L]**

| key | uses | states it appears in |
|---|---:|---|
| `mob.<n>.id` / `.count` | 63 / 63 quests | **state 1 only**, all 232 rows |
| `item.<n>.id` / `.count` | 216 quests | 794 rows in state 1, 4 in state 0 |
| `npc` | 640 | 320 in state 0, 320 in state 1 |
| `lvmin` | 406 | - |

`Check.0` gates **accepting** the quest, `Check.1` gates **completing** it. Every kill
requirement in this client is a completion requirement.

**Sam's Suggestion is quest 1006** and it is the whole bug in four rows: **[L]**

```text
1006  Check  0.npc          9      Sam starts it
1006  Check  1.npc          11     someone else finishes it
1006  Check  1.mob.0.id     2
1006  Check  1.mob.0.count  10
1006  Act    1.exp          30
1006  Act    1.item.0.id    2010002   x3
```

### 1.1 What was added to `tools/dump_quests.py`

The data was already there; what was **not** there was the **slot index**, and the slot
index is the field the wire format is addressed by. `dump_quests.py` now also writes
`gm-handbook/questreq.txt`:

```text
# questId  state  kind  slot  id       count
1005       1      item  0     4000000  3
1005       1      mob   0     1        3
1006       1      mob   0     2        10
1009       1      mob   0     3        10
1009       1      mob   1     4        10
1009       1      mob   2     5        5
```

416 rows - 87 mob, 329 item - across 224 quests.

It is a separate file rather than a query over `questlines.txt` for a reason worth writing
down: **`questlines.txt` sorts its rows by the dotted path as a string**, so `mob.10` sorts
between `mob.0` and `mob.2`. No quest ships ten mob requirements (the most is three) so the
current file is not wrong - but a consumer that recovers slot order by sorting it is one
data change away from being wrong in the quiet direction, and this is precisely the field
that must not be quietly wrong. Here the slot is a column.

## 2. The progress string, read off the client

`FUN_14070cb70(questData, charData, questId, slot)` is the client's "how many of mob *slot*
has this character killed for this quest" accessor. Its default path reads the answer out of
the progress string in the started-quest map (`charData+0x1273`, node `+0x18`): **[L]**

```asm
14070ccb8  lea   rbx, [rax + 0x18]        ; the progress string on the map node
14070ccd8  mov   eax, 0x55555556
14070ccdd  imul  dword ptr [rbx - 8]      ; length / 3
14070cce7  lea   eax, [rdx + rdx*2]       ; (length / 3) * 3
14070ccea  cmp   dword ptr [rbx - 8], eax
14070cced  jne   14070cc8e                ; length NOT a multiple of 3 -> return 0
14070ccfb  mov   rax, [r15 + 0x100]       ; the quest's mob-requirement array
14070cd0c  mov   eax, 0x55555556
14070cd11  imul  edx                      ; length / 3   ...
14070cd1a  cmp   edx, ecx                 ; ... vs the number of mob requirements
14070cd1c  jb    14070cc8e                ; too short -> return 0
14070cd22  lea   r8d, [r14 + r14*2]       ; start = slot * 3
14070cd26  lea   r9d, [r8 + 3]            ; end   = slot * 3 + 3
14070cd34  call  0x14019ce60              ; substr(start, end)
14070cd46  call  0x142f11a94              ; and convert it, base 10
```

**Both primitives were read, not assumed.** `0x14019ce60` bounds-checks `start` and `end`
against the length, computes `end - start` and allocates `that + 0x11` - a substring.
`0x142f11a94` passes `r8d = 0xa` to `0x142f10268` - a base-10 conversion. **[L]**

So the string is `"004"` for four kills of a one-mob quest, `"010007000"` for a three-mob
quest at 10, 7 and 0.

### 2.1 Two failure modes worth mirroring exactly

Both gates answer **zero for every slot**, not for the offending one: **[L]**

* a length that is not a multiple of three (`14070cced`);
* a length shorter than three times the quest's mob count (`14070cd1c`).

A server that reads its own strings more forgivingly than the client does would show
progress in `world.log` and none on screen. `net::quest::kill_count_at` takes the slot
*count* as a parameter for exactly this reason.

**An empty string is legal and reads as zero kills** - it fails the second gate, every
counter reads 0, and nothing faults. That is what `quest_accepted` sends today.

### 2.2 The alternate encoding, which this server does not need

When the mob entry's `+0x0c` is non-zero, `FUN_14070cb70` takes a different path entirely:
it formats the key `"m%d"` from the slot (`14070cbc7`, the ascii literal at `0x14329828c`)
and looks it up in a key/value store through `FUN_1402e01c0`, instead of indexing the
string. **[L]** Which `Quest.wz` node sets `+0x0c` is **not established**; no quest in the
current data needs it, because the string path is what `#a` reaches for every quest checked.
It is written down here so that a quest which one day renders no counter has a named
suspect.

## 3. What the client draws, and why `demandSummary` is the proof

`QuestInfo.demandSummary` is the "what this quest wants" line. Quest 1006's is: **[L]**

```text
#o2# #a10061#
```

and quest 1009's, which has three mob requirements, is

```text
#o3# #a10091#\n#o4# #a10092#\n#o5# #a10093#
```

`FUN_142d9afd0` is the `#a` expander - the only caller of `FUN_14070cb70` reached from the
markup dispatcher `FUN_142a45580`. It takes the number as one integer and splits it: **[L]**

```asm
142d9afee  mov   eax, 0x66666667
142d9aff3  imul  r8d
142d9aff8  sar   esi, 2                  ; esi = n / 10      -> the QUEST ID
142d9b002  lea   eax, [rsi + rsi*4]
142d9b005  add   eax, eax
142d9b007  sub   ebp, eax                ; ebp = n % 10      -> the markup index
142d9b0b3  test  ebp, ebp
142d9b0b5  jle   <give up>               ; the index is 1-BASED
142d9b0b7  lea   eax, [rbp - 1]          ; array slot = index - 1
142d9b0f1  mov   edi, [rdx + rcx*4 - 0x10]  ; = entry[slot] + 4, the REQUIRED count
142d9b102  call  0x14070cb70                ; = the CURRENT count
142d9b10d  mov   rdx, [rip + 0xcaa08c]      ; -> 0x1432a87e8
142d9b119  call  0x14019ba10                ; format
```

The format string at `0x1432a87e8` is **`"%d / %d"`**. **[L]** Its neighbours in `.rdata`
are `"%s / %s"` and `"EXP: %s / %s"`, which is what that block of memory is for.

It is reached through a pointer (`mov rdx,[rip+…]`), not a `lea`, so `tools/xref.py` cannot
see it - the blind spot its own docstring names. It was resolved by computing the target and
reading the qword out of `.data`.

> **This is the answer to "what does the server send so the client shows `1/10`".** The
> client renders both numbers itself. The server supplies one string.

### 3.1 The slot order is `Quest.wz` key order, and quest 10113 proves it

The array could plausibly have been sorted by template id. It is not. **[D]**

| quest | slots, in order | its own `demandSummary` |
|---|---|---|
| 1009 | mobs 3, 4, 5 | `#o3# #a10091#` `#o4# #a10092#` `#o5# #a10093#` |
| 10000 | mobs 6, 7 | `#o6# #a100001#` `#o7# #a100002#` |
| 10002 | mobs 16, 19 | `#o16# #a100021#` `#o19# #a100022#` |
| **10113** | mobs **18, 17** | `#o18# #a101131#` `#o17# #a101132#` |

10113 is the discriminating case: its key order and its numeric order disagree, and the
markup follows the **key** order. Five quests checked, one of them decisive.

### 3.2 Three digits is enough

The largest `count` in any `Check.<state>.mob.*` in this client is **200**. **[L]**
`net::quest::kill_progress` saturates at 999 rather than widening a field, because a
four-character field would fail §2.1's multiple-of-three gate and zero *every* counter in
the quest, not just the one that overflowed.

## 4. The mob requirement array, as the client holds it

`QuestData+0x100`, entries of **20 bytes** (`add rdi, 0x14` at `1407128de`, `142743665`).
Read across three independent consumers: **[L]**

| field | what | where it is read |
|---|---|---|
| `+0x00` | the mob template id **[I]** | not read by any of the three; see below |
| `+0x04` | the **required** count | `1407128ce`, `14274365d`, `142d9b0f1` |
| `+0x08` | the `Check` **state** this entry came from | `140712875`, filtered against the state argument |
| `+0x0c` | selects the `"m%d"` key/value encoding of §2.2 | `14070cbb4` |
| `[array - 8]` | the entry count | everywhere |

**`+0x00` is [I].** None of the three consumers reads it, because none of them needs to -
they are all indexed by slot. It does not matter on the wire either: slots are positional
and the server takes the template ids from `Quest.wz`, where they are unambiguous. What
*is* load-bearing is §3.1's ordering claim, and that rests on the data, not on this field.

## 5. Item quests need no running count - the client asks the bag

Two independent consumers do the same thing with an item requirement, and neither touches
the progress string: **[L]**

```asm
; FUN_140711e50, the requirement checker
140712220  mov   edx, [r14 + rbx]        ; the ITEM ID out of the entry
140712224  mov   rcx, [rsp + 0x38]       ; charData
140712229  call  0x1403eb020             ; -> two counters, via a visitor at r9
14071222e  mov   edx, [r14 + rbx + 0x10] ; the REQUIRED count
140712241  cmp   r9d, edx / jge          ; and compare

; FUN_142743510, the progress renderer, on its "item" node
142743b5a  mov   edx, [r12 + rbx]
142743b61  call  0x1403eb020
142743b66  mov   edx, [r12 + rbx + 0x10]
142743b73  cmp   r8d, edx / jge
```

The item entry is **32 bytes**: `+0x00` id **[L]**, `+0x10` count **[L]**, `+0x18` the Check
state **[L]**.

And the string's own length gate (§2, `14070cd1a`) counts **only** the mob array. There is
no item field in the progress string and nothing reads one. **[D]**

This is corroborated from the data side by `demandSummary`, where an item line is
`#i<id>:# #t<id>:# #c<id># / <count>` - icon, name, **a live count**, and the required
number as *literal text*. Compare the mob line, which is `#o<id># #a<n>#` with no literal
number in it at all: the mob line needs `#a` precisely because its current count has nowhere
else to come from. **[D]**

> **So the answer to question 4 is: no running count, and no per-pick-up server work.**
> An item quest is a pure function of the bag at the moment it is checked. The server's only
> obligation is to evaluate the same predicate when the player turns the quest in, because
> the server must not trust the client's word for it.

`FUN_1403eb020` is characterised here as "count how many of this item the character holds".
That is **[D]**, not [L]: it is called with an item id and a visitor, its output is compared
against a required count at two unrelated call sites, and one of those sites is the function
that renders `#c`. Its internals were not walked.

## 6. Sending the update: the same packet as the accept

`0x0089` sub-case 1, state **1**, carrying the string. `research/quest-state.md` §5 has the
packet; two things about the **state-1 arm** matter here and are new.

**The insert is unconditional and non-bulk.** `FUN_142d5b750`'s state-1 arm runs the
requirement checker `FUN_140711d70` at `142d5bd98` and stores only whether it returned zero
- it does **not** gate the insert on it - then inserts through `FUN_1402e0eb0` at
`142d5be0c`, whose fourth argument is zero, which is the delta insert that queues the
changed quest id for the UI. **[L]** So the same packet accepts a quest and updates it, and
sending it repeatedly is safe.

**An unchanged string is invisible, not merely wasteful.** The arm saves the old string off
the map node before inserting and then compares: **[L]**

```asm
142d5bde9  add   rdx, 0x18            ; the OLD progress string
142d5bdf2  call  0x14019a260          ;   saved to [rsp+0x60]
142d5be0c  call  0x1402e0eb0          ; insert the new one
142d5be14  mov   rdx, [rsp + 0x60]
142d5be19  cmp   rcx, rdx
142d5be1c  je    142d5c04c            ; identical -> skip everything that follows
```

`net::quest::apply_kill` returns `None` rather than an unchanged string for this reason: a
caller that sends anyway cannot tell a working counter from a broken one.

**The two ways `0x0089` silently does nothing still apply** (`quest-state.md` §5): the
singleton flag bit, and a null `charData` before the first `SetField`. Do not send this with
or just before a `SetField`.

## 7. Completion - what is settled and where I stopped

**What marks a quest complete on the client** is settled and already built: `0x0089` state
**2** with the 8-byte FILETIME, which inserts into the completed map *and* erases from the
started one, plus the `presence[14]` record block on the next `SetField`.
`net::quest::quest_completed`. **[L]**, `quest-state.md` §3 and §5.

**What the client sends when the player returns to the NPC** is `0x0151`, and which of the
six tags depends on the client's own view of the state and on `err = FUN_140711d70(...)`.
From `research/npc-dialogue.md` §1.4, all **[L]**:

| tag | gate |
|---|---|
| **6** | `err != 0` && (`state == 1` \|\| `err == 0xC`) && `FUN_140715460(q)` |
| **2** | `err == 0` && `state == 1` && `FUN_1407158a0(q)` |
| **2** | `state == 1`, the main path, when neither of the above fired |

`FUN_140711d70` loops the quest's `Check` entries and returns the first non-zero error;
`FUN_140711e50` is the per-entry evaluator, and its mob arm is

```asm
1407128c4  mov   rax, [rsi + 0x100]
1407128ce  cmp   r14d, [rdi + rax + 4]   ; kills so far  vs  required
1407128d3  jl    1407128e7
1407128e7  mov   r12d, 6                 ; -> error 6
```

so **error 6 is "not enough kills"**. **[L]** The item arm produces 2 and 7 the same way.

**Where I stopped.** `FUN_140715460(questId)` decides whether tag 6 is sent at all, and it
resolves to "the string at `QuestData+0x148` is non-empty" (`140715525 mov rax,[rdi+0x148]`
/ `140715531 cmp byte [rax],0`). **[L]** *Which* `Quest.wz` node lands at `+0x148` is
**not established**. That fork matters:

* if it is the `stop` node, the client sends **tag 6** when the player turns up short, and
  the server should answer with `Say.<state>.stop.mob.<n>` - quest 1006 has exactly that
  line, *"I'm sure I told you to defeat 10 …"*;
* if it is something rarer - `failscript` is only in 4 quests - the client falls through to
  the main path and sends **tag 2** even at 4/10, and the refusal is entirely the server's.

**Either way the server must validate on tag 2 rather than trusting the client**, so this
does not block anything; it only decides whether a `stop` line is also worth sending. It is
one more static session to settle, or it will settle itself on the first run that turns a
quest in short.

Applying `Act.<state>` on completion - `exp`, `money`, `item.<n>` - is entirely server work
and is untouched here. The rows are in `gm-handbook/questlines.txt`.

## 8. What is in `crates/net/src/quest.rs` now

Builders and parsers only; nothing is wired.

| | |
|---|---|
| `PROGRESS_FIELD_WIDTH`, `MAX_PROGRESS_FIELD_VALUE` | 3, and 999 |
| `kill_progress(&[u32]) -> String` | counts to the wire string |
| `kill_count_at(&str, slot, slots) -> u32` | the client's read, **including both of its zero-everything failure modes** |
| `kill_counts(&str, slots) -> Vec<u32>` | all slots at once |
| `count_kill(&str, slot, slots, required) -> Option<String>` | one kill; `None` = nothing changed = send nothing |
| `apply_kill(&QuestRequirements, &str, template) -> Option<String>` | one kill against one quest |
| `MobRequirement`, `ItemRequirement`, `QuestRequirements` | with `progress_slots`, `fresh_progress`, `kills_met`, `items_met` |
| `QuestRequirementTable::parse(&str)` | `gm-handbook/questreq.txt`, plus `quests_for_mob` |

Tests pin the bytes: `"004"`, the full `0x0089` body for Sam's fourth snail, both malformed
string gates, the saturation, 10113's slot order, and a drift test that re-parses the real
generated file and asserts every mob slot is contiguous, in state 1, and fits in three
digits.

## 9. Wire it like this

`crates/world/src/session/` and `crates/store/` were deliberately not touched.

**a. Load the table once, beside the other `gm-handbook` tables.**
`crates/world/src/bin/world_server.rs` already builds paths for `mobtemplates.txt`,
`mobs.txt`, `equips.txt` and friends; add `gm-handbook/questreq.txt` the same way and put
`net::quest::QuestRequirementTable::parse(&text)` on the config the session already carries.
Missing file = an empty table = today's behaviour, which is the right failure.

**b. Count the kill where the drops and the EXP are already counted.**
`crates/world/src/session/combat.rs`, inside `on_attack`'s death branch - the
`if left.is_none()` block that already calls `drops_from_kill` and `award_experience`.
`template` is in scope one line above.

```rust
if left.is_none() {
    out.extend(self.drops_from_kill(template, target.object_id, died_at, chr_id, map));
    let worth = self.config.mob_exp.get(&template).copied().unwrap_or(0);
    out.extend(self.award_experience(u64::from(worth), "a kill"));
    out.extend(self.count_kill_for_quests(chr_id, template));   // <- new
}
```

and the new method, which is the whole feature:

```rust
fn count_kill_for_quests(&mut self, chr_id: u32, template: u32) -> Vec<Reply> {
    let mut out = Vec::new();
    for quest_id in self.config.quest_reqs.quests_for_mob(template) {
        let Some(reqs) = self.config.quest_reqs.get(*quest_id) else { continue };
        // Only quests this character has actually started.
        let Ok(Some(row)) = self.store.quest_row(chr_id, *quest_id) else { continue };
        if row.state != store::quest::QuestState::InProgress { continue }
        let Some(next) = net::quest::apply_kill(reqs, &row.progress, template) else {
            continue;   // no requirement, or already at the target - send NOTHING
        };
        if self.store.set_quest_progress(chr_id, *quest_id, &next).is_err() { continue }
        out.push(Reply {
            opcode: net::quest::MESSAGE,
            body: net::quest::quest_record(
                *quest_id,
                &net::quest::QuestProgress::InProgress { progress: next },
            ),
            what: format!("quest {} progress", quest_id),
        });
    }
    out
}
```

**A template can belong to several quests** - template 13 is named by six - so this loops
rather than stopping at the first match. `quests_for_mob` returns them all.

**c. Accept with a right-sized string.** `on_quest_request`'s `QUEST_ACTION_START` arm in
`crates/world/src/session/npc.rs` currently sends `quest_accepted`, which is the empty
string. That works (§2.1) but it means two shapes exist. Prefer:

```rust
let fresh = self.config.quest_reqs.get(req.quest_id)
    .map(|r| r.fresh_progress())
    .unwrap_or_default();
self.store.set_quest_progress(id, req.quest_id, &fresh).ok();
// then quest_record(req.quest_id, &QuestProgress::InProgress { progress: fresh })
```

**d. Validate on `QUEST_ACTION_COMPLETE` instead of trusting the tag.** The client may send
tag 2 without meeting the requirements (§7):

```rust
// `crates/store` has no "how many of item X" call, and this is the only caller that
// wants one - so count it from the Bag it already returns rather than adding an API
// for one use. `items` is every occupied slot, so one pass is enough.
let bag = self.store.bag(chr_id)?;
let held = |item_id: u32| -> u32 {
    bag.items
        .iter()
        .filter(|i| i.item.item_id == item_id)
        .map(|i| u32::from(i.item.kind.quantity()))
        .sum()
};
let ok = match self.config.quest_reqs.get(req.quest_id) {
    None => true,                                   // no requirements to fail
    Some(r) => r.kills_met(&row.progress) && r.items_met(held),
};
```

**This counts the bag and not the equipped list**, which is a separate table in
`crates/store`. Whether the client's own `FUN_1403eb020` includes worn items is **not
established** - its `r8d = 0xf` argument looks like an inventory-type mask and was not walked
(§10). It is not hypothetical: **four `Check` item requirements are equip-class ids** -

```text
10402  item 1302009    10410  item 1032002    80012  item 1422003    80015  item 1002084
```

- a sword, an earring, a polearm and a hat, any of which a player might reasonably be
wearing when they walk up to the NPC. 303 of the 329 item requirements are ETC (`4…`), so
this affects four quests out of 224 - but "the quest refuses because you are wearing the
thing it asked for" is a bad enough failure that it is worth either counting both tables or
deciding, deliberately, not to.

and only then `complete_quest` + `quest_completed` + the `Act` rewards. **Always answer**
either way - a refusal is a reply, and `research/npc-dialogue.md` has the `stop` lines to
refuse with.

**e. Do not send a `0x0089` with or just before a `SetField`.** `quest-state.md` §5. On a
field entry the record blocks carry the book instead, which is already wired.

**One thing that must not be got wrong:** the progress string is the *only* place a kill
count lives. `store::set_quest_progress` already exists and already persists it, and
`QuestBook::started` already puts it in the record - so once (b) writes it, the counter
survives a relog for free. If it does not, the bug is in the record block, not in this.

## 10. What I did NOT establish

* **Which `Quest.wz` node is `QuestData+0x148`** - the string whose emptiness decides
  whether the client sends tag 6 rather than tag 2 for an unmet quest (§7). Two readings,
  both consequential for what the server replies with, neither settled.
* **What sets the mob entry's `+0x0c`**, the flag that switches `FUN_14070cb70` to the
  `"m%d"` key/value encoding instead of the string (§2.2). No quest checked needs it.
* **`+0x00` of the mob entry is the template id.** [I] - none of the three consumers reads
  it. Nothing on the wire depends on it (§4).
* **`FUN_1403eb020` counts inventory items.** [D] from two call sites and their comparisons,
  not [L] - its 3109 bytes were not walked (§5). In particular **what its `r8d = 0xf`
  argument selects is unknown**; it looks like an inventory-type mask, and whether it
  includes *worn* equipment decides four quests (§9d). Both call sites pass the same `0xf`,
  so nothing can be inferred by comparing them.
* **Whether the array order for a quest with mob entries in more than one `Check` state
  matches `questreq.txt`'s state-then-key walk.** No quest in this client has them, so the
  question is currently unanswerable and currently harmless.
* **Anything about the other 35 sub-cases of `0x0089`.** Only sub-case 1 is read; that was
  already true in `quest-state.md`.
* **That any of this renders.** Nothing here has been in front of the client. The chain
  string -> `FUN_14070cb70` -> `#a` -> `"%d / %d"` is complete in the listing, and complete
  in the listing is not the same as on screen - which is the whole reason `STATUS.md` keeps
  "WIRED, and NOT yet seen on a screen" as its own heading.
* **The `Act` side of completion** - exp, mesos, reward items. Read out of
  `gm-handbook/questlines.txt`; no code here touches it.

## 11. What a run would settle, one variant at a time

Accept Sam's quest from NPC 9 on map 1, then go to map 40 and kill snails (template 2).

| what to watch | what it means |
|---|---|
| the quest window shows **`0 / 10`** right after accepting | the whole render chain is alive: the record block, `#a`, and `"%d / %d"`. This is the gate - if it shows nothing, no kill will ever show either |
| it becomes **`1 / 10`** on the first kill | the string, the packet and the client's change-diff all work |
| `world.log` shows one `0x0089` per kill and **none** after the tenth | `apply_kill` returning `None` at the target, as designed |
| relog: still `4 / 10` | `set_quest_progress` and the `presence[9]` block agree |
| talk to NPC 11 at 4/10 | **this is the §7 fork.** A `0x0151` with tag **6** means `+0x148` is the `stop` node; tag **2** means the refusal is entirely ours. Either way `world.log` names it, and that is one static question answered for free |

If `0 / 10` never appears but the quest is listed in the journal, the fault is in the
*record* block, not in any of this - `quest-state.md` §8 has that experiment already
written.
