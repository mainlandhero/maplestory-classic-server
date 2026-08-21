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

`crates/world/src/session/combat.rs` — `award_kill_experience` finds *this* character in the
list and pays its cut, with `white = majority`.

## What is NOT built, and why

**A share is only ever paid to the connection that killed the mob.** The split names every
contributor, but there is no way to push a packet into another player's thread: a channel is a
process and each connection is a thread with its own socket (`crates/world/src/server.rs`).
So a second player who helped kill something gets nothing until there is a way to deliver it.
`Fields::hurt` already returns the whole split; **only the delivery is missing**, and that is
the same gap the scrolling banner works around by having every session compute its own screen
state from shared storage. The same trick does not work here, because an EXP award is an event
rather than a state — a session cannot look up "did I earn something 200 ms ago" without a
queue to look in.

**Parties do not exist**, so 70/30, the AFK exclusion, and `You received party EXP` are not
implemented. The wording matters when they are: it is a different string id from the ordinary
one, so it is a different message, not the same message in another colour.

## The colour byte

`0x0089` type 3, first field, `dst+0x00`. `142d5f014 cmp dword [rbp+0xb0], ebx / je` picks
`r8d = 4` when it is zero and `r8d = 0` when it is not, and `r8d` is `FUN_142572050`'s third
argument. **[L]** for the branch.

**The owner's screenshot establishes one half of the meaning.** The build that produced it sent
`white = 0`, and the line drew **yellow**. So `0 -> r8d = 4 -> yellow` is now **[L]**, where
before this the module said outright that what the byte does on screen was not established.

That `1` gives white is **[I]**: it is the only other branch, and it is what the field is for
in every related client, but nobody has seen it. The next run settles it in one kill.
