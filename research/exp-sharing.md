# EXP sharing: who gets what, and which colour the line is

**The owner, 2026-08-20**, with a screenshot of a yellow `You received EXP (+2)`:

> *"the EXP gained for killing is actually incorrect because they are yellow. There's two
> different color lines for this because if I was the person who dealt majority damage, I
> should see a white line of EXP gained. If I was not the person who dealt majority damage, I
> would only get a % portion of the EXP that belonged to the mob (over-damage of a mob's HP
> does not count towards % sharing), and that line would be yellow. If I'm in a party, when I
> kill a mob, I get 70% of the EXP and all other party members receive a share of the 30% EXP
> equally (provided that they are not AFK) My majority share line would be white, and my party
> members should see a yellow line of 'You received party EXP'"*

## The rule

| | |
|---|---|
| majority damage | the **whole** of the mob's EXP, on a **white** line |
| anything less | a **share proportional to damage dealt**, on a **yellow** line |
| over-damage | **does not count.** A 500-damage hit on a mob with 3 HP left is credited 3 |
| in a party, the killer | **70%** |
| in a party, everyone else | the remaining **30%**, split equally, AFK members excluded |
| a party member's line | yellow, and reads `You received party EXP` |

## What is built

`crates/world/src/fields.rs` — `LiveMob::damage_by` accumulates `(character, damage)` with
each contribution capped at the mob's remaining HP, so over-damage is discarded at the moment
it happens rather than corrected later. `Fields::hurt` returns `Hurt::Died(Vec<DamageShare>)`,
highest share first, with **exactly one** entry flagged `majority` — ties broken by who hit
the mob first, which is stable, so the same fight scores the same way twice.

`DamageShare::cut_of(exp)` gives the majority holder the whole amount and everyone else their
fraction, floored at 1 so that a contributor who earned a share is never paid nothing.

`crates/world/src/session/combat.rs` — `award_kill_experience` pays **every** contributor:
itself directly, with `white = majority`, and everyone else across the channel's message bus.
See the next section for how the second half works and what it still does not prove.

## What stopped being missing, and what is still NOT built

### The queue exists now, and the paragraph that asked for it was a prediction that came true

This section used to say there was *"no way to push a packet into another player's thread"*.
That is no longer true. `crates/world/src/broadcast.rs` (commits `ee67d58`, `4674202`) gives
every connection a mailbox on a per-channel `Bus`: a session appends to another connection's
mailbox and returns, and that connection drains its own mail on its own thread, from
`Session::handle` and from `Session::tick`. No second writer ever touches a socket.

The argument that used to sit here is worth converting rather than deleting, because it was
right. It said the scrolling banner's trick — every session computing its own screen state
from shared storage, `crates/world/src/session/rates.rs` — *"does not work here, because an
EXP award is an event rather than a state: a session cannot look up 'did I earn something
200 ms ago' without a queue to look in."* A queue is exactly what was then built, and the bus
carries **two channels** for exactly that distinction:

| | |
|---|---|
| `Bus::publish` / `Bus::drain` | a finished `Reply` to **everyone on a map**. A state, and it says the same thing to every observer |
| `Bus::send_to_character` / `Bus::drain_events` | an `Event` to **one character**. A fact, and the recipient's session builds the packet from it |

EXP has to be the second one, and the reason is not the queue — it is the packet. `0x007C`
carries the recipient's **own new total and own new level**, and only the recipient's session
can compute those: it needs that character's row, its own `config.exp_curve.award`, its own
`store::save_character_progress`. A `0x007C` built by the killing session would show a
contributor somebody else's total and somebody else's level, and the level is the number a
level-up effect hangs off, so the error would not stay quiet. That is why
`broadcast::Event` is an `enum` and not a `Vec<u8>`: `Event::Experience { amount, why, white }`
carries the *amount*, and the bus is deliberately unable to express a finished EXP packet.

### What is delivered — wired 2026-08-29, and what that does and does not prove

The loop is closed in code. Read off the tree rather than assumed:

| | |
|---|---|
| producer | `session/combat.rs::award_kill_experience` pays *itself* directly, then for every other contributor calls `bus().send_to_character(share.character, Event::Experience { amount: share.cut_of(worth), why, white: share.majority })` |
| transport | the character's mailbox on the channel's `Bus`. Nothing supersedes — two shares of two kills are two events |
| consumer | `session/multiplayer.rs::collect_mail` drains the events and runs each through `Session::award_experience`, the **same** function a local kill uses, so the `0x007C` and the `0x0089` are built from the recipient's own record |
| when | `collect_mail` runs at the end of `Session::handle` and at the top of `Session::tick`, so a share waits at most one tick even for a player standing still |

Two details worth keeping, because both are the kind that go wrong quietly:

* **The rate is applied once, by the killer.** `exp_for_kill` multiplies before the split, so
  `amount` crosses the bus already scaled and the recipient's session applies only the curve
  and the save. Rates are global (`store::rates`), so once is right — but note that
  `broadcast::Event::Experience`'s own doc block says the receiving session "applies its own
  rate multipliers", which it does not.
* **An undelivered share is logged, not swallowed.** `send_to_character` returns `false` when
  nobody on this channel is playing that character — they logged out, or they are between the
  two halves of a channel change, which `Bus` cannot tell apart. That is ordinary and not
  retryable, and `award_kill_experience` writes a line naming the character and the amount.
  This project has already shipped one guard whose answer nobody read.

The end-to-end path is pinned by `a_helpers_share_of_a_kill_reaches_their_own_session` in
`session/multiplayer.rs`, which asserts the helper's EXP is **still zero** at the moment the
killer is paid and only becomes 30 after the helper's own tick — so a future change that
builds the helper's packet inside the killer's session fails there. Its last assertion (the
killer draining its own mailbox) was **verified by injecting the bug it catches**, dropping
the `s.character != chr_id` filter; nothing else in the suite caught that.

**None of it has been on a screen.** Everything above is unit tests over an in-memory store.
Two clients have never been run against this server at once, and `session/multiplayer.rs`
records that no packet in `0x224..0x39F` — the sighting half that has to work first — has ever
been observed to do anything in any archived run. So the honest state is: *the split is
computed, delivered and applied in code, and nobody has watched a second player's yellow line
appear.*

**Parties now exist, and the party split is implemented** (`Session::party_exp_split`,
2026-09-05; **rule changed 2026-09-06**). When the killer is in a party of two or more, the
damage-share path above is suppressed and this runs instead: the killer keeps **70%** (white),
and every other member **standing on the killer's field** receives **a copy** of the party
share of the whole worth, yellow - not a division of the remaining 30%. The owner, 2026-09-06:
*"30% split copy for party member means killer (70% - 70 EXP), party mem 2-6 (30% each, 30
EXP each), this mob awarded a total of 220 EXP; 50% ... 50 EXP each ... 320."* The share is
the fifth `!setrates` field (`RateKind::Party`, 30% until changed, 0 allowed), so the total
paid out grows with the party rather than summing back to the worth. A member on another map
or offline is not eligible and their copy is not minted; a killer with nobody there keeps 100%.

Two honest gaps, both stated in the code:

* **AFK is not modelled.** The owner's rule excludes AFK members; this server has no idle signal,
  so *"on the field and online"* is the whole eligibility test. `characters_on` (the bus) is
  the query, read once before anyone is paid.
* **There is no distinct `You received party EXP` string in this client.** The dump has
  `0x00C1 "You received EXP"` and the party-quest line `0x00D5`, but no plain party-EXP
  string, so the member's line is the ordinary one in **yellow** - which is this client's
  party-EXP presentation. If a distinct id turns up in a capture, `why: "party EXP"` in the
  bus event is where to switch it.

## The colour byte

`0x0089` type 3, first field, `dst+0x00`. `142d5f014 cmp dword [rbp+0xb0], ebx / je` picks
`r8d = 4` when it is zero and `r8d = 0` when it is not, and `r8d` is `FUN_142572050`'s third
argument. **[L]** for the branch.

**The owner's screenshot establishes one half of the meaning.** The build that produced it sent
`white = 0`, and the line drew **yellow**. So `0 -> r8d = 4 -> yellow` is now **[L]**, where
before this the module said outright that what the byte does on screen was not established.

That `1` gives white is **[I]**: it is the only other branch, and it is what the field is for
in every related client, but nobody has written down having seen it.

**And "the next run settles it in one kill" has been sitting here while the runs went past.**
Every solo kill sends `white = 1` — `award_kill_experience` passes `mine.majority`, and a lone
contributor is always the majority — and the server has logged `WHITE (majority damage)` in
**34 distinct archived world logs by content hash** (fewer distinct *runs* than that, because
a fixture copied mid-write hashes differently from its `previous-runs/` original;
`CLAUDE.md`). Dozens of opportunities, not one.

So this is not waiting on a build, and never was. `world.log` records what the server **sent**,
not what was **drawn**, so no amount of grepping can close it — it needs one sentence from
someone in front of the screen. It costs no dedicated run: the next kill of any run already
does it, and the only thing missing is the question being asked.
