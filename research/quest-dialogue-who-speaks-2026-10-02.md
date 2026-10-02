# Who speaks which quest line - the client's own request builder, 2026-10-02

Players: *"quests are repeating lines in general."* The owner: *"figure out in the WZ data what the
client automatically shows locally before an opcode is sent, so that the server do not repeat the same
quest dialogue when the opcode does reach the server. Do an overall scan for all quests."*

## 1. The finding [L]

`FUN_141f0e4c0` builds every `0x0151` (decompiled 2026-10-02, Ghidra, with `FUN_141f13360`,
`FUN_141f146a0`, `FUN_141f15fb0`, `FUN_141f18c20`, `FUN_141f0e110`). After the special forms - action 6
(`failscript`), action 4 (`startscript`), action 5 (`endscript`) and two dialog-free forms below - it
does the same thing for **both** states:

```text
node = Say/<state>                      FUN_140734b40(quest) then the child named itoa(obj+0x28)
                                        (0 = not started -> "0", 1 = in progress -> "1")
if node exists:
    ask = node["ask"]                   PTR_DAT_143a488f8 -> "ask"
    r = ask ? FUN_141f146a0(...)        the local dialog: every line of the node in the
            : FUN_141f13360(...)        script box (FUN_142a61900 = Say), Yes/No or the #L menu
                                        on the last line, run modal (FUN_142a64400)
    if r < 0: send nothing              closed / No
state 1: sel = FUN_141f15fb0(lines)     a reward pick off the last line; 0x7FFFFFFF = abort
         send 0x0151 action 2 (..., sel)
state 0: send 0x0151 action 1 (..., r)
```

So **the client draws `Say.0` before an accept and `Say.1` before a turn-in**, with their yes/no or
menu, and sends the request only on the positive answer. The `stop` texts (`/1/stop/%s` with
`item | quest | level`) are drawn by `FUN_141f18c20` when a requirement fails, and no turn-in is sent.

`research/quest-dialogue-audit-2026-09-13.md` §1 had *"The completion, `Say.1`, is the server's, in
answer to action 2. [D]"*. **That derivation was wrong.** Its argument (Rain's `#L` box faulting on OUR
packet) shows only that the server's copy faulted, not that the client had not drawn its own. The
quiz case found the same day was this rule seen once; it is every turn-in.

## 2. The two dialog-free forms, and why they never apply here [L]

| quest-info byte | `QuestInfo` key | set on the quest manager | form |
|---|---|---|---|
| `+0x3d` | `autoAccept` | `+0x468` (`FUN_140715800`) | accept sent without drawing `Say.0` |
| `+0x53` | `autoCompleteAction` | `+0x498` (`FUN_1407158a0`) | action 2 body B (`.., u32 -1`) without drawing `Say.1` |
| `+0x3c` | `autoStart` | `+0x450` (`FUN_1407155d0`) | only drops the x/y pair |

Bytes stored by the info loader `FUN_14072a280` (`14072a879`, `14072a8a7`, `14072aa6a`) from the key
names behind `.data` pointers `0x143a45810/18/40`; the sets are filled from those bytes by
`FUN_14071b0e0` (`14071b244`, `14071b265`, `14071b386`). **No quest in this client carries any of the
three keys** - `gm-handbook/questlines.txt`'s whole `QuestInfo` vocabulary is `name, area,
demandSummary, parent, order, selfStart, rewardSummary, selectedMob, resignScript`.

## 3. The scan - all 322 quests

| | quests | the client drew | the server answers |
|---|---:|---|---|
| accept, `Say.0` + `0.yes` | 157 | `Say.0` (7 of them `ask`) | `0.yes` |
| accept, `Say.0`, no `0.yes` | 152 | `Say.0` | the record only |
| accept, no `Say.0` | 13 | nothing | the record only |
| turn-in, `Say.1` + `1.yes` | 140 | `Say.1` and its yes/no | **`1.yes`** (was: `Say.1` again) |
| turn-in, `Say.1`, no `1.yes` | 147 | `Say.1` | **the record only** (was: `Say.1` again) |
| turn-in, quiz (`ask` on `1`) | 11 | the quiz | the record only |
| turn-in, no `Say.1`, `nextQuest` has `Say.0` | 11 | nothing | the next quest's `Say.0`, and starts it |
| turn-in, no `Say.1` | 13 | nothing | the record only |

287 turn-ins repeated their completion lines until this change - 82 of them Community Board quests,
whose weekly donations repeat a yes/no (the live client drop of 2026-10-02 was its Yes taken as a
second accept). `1.yes` after a turn-in is the server's by the same argument as `0.yes` after an
accept (measured on screen 2026-08-20); that half is **[D]**.

## 4. In the code

`session/npc.rs::on_quest_request`: action 2 on a quest with `Say.1` answers `1.yes` (no `ask`) or
nothing. Action 5 (`endscript`, no local dialog) still speaks `Say.1`. The audit test
`no_quest_answers_its_accept_or_turn_in_with_its_own_opening_lines` now asserts no turn-in is answered
with its own `Say.1` (245 non-board quests: 105 `1.yes`, 111 record after the client's line).
