# The spent-SP ledger: schema, balance, and what the two GM commands must now do

Written 2026-08-28. **A persistence change, not a decode** — no Ghidra, no client run, and
nothing in `crates/net/` or `crates/world/` was touched. It builds on
`research/skill-points.md`, which decoded the wire format and the pool key, and does not
repeat any of it.

Labels: **[L]** measured on this machine; **[D]** derived from [L] facts; **[I]** inferred.

**Built, not wired.** `0x013B` still spends nothing. See *Wire it like this*.

---

## 0. The bug, and the sentence that predicted it

`crates/world/src/session/gm.rs::gm_job`, in its own comment:

> *"The amount is computed from the LEVEL, and nothing is persisted yet. … What is missing is
> a record of what has been SPENT — so until `0x013B` persists, points come back on the next
> advancement."*

`world::skillpoints::entitlement(tier, level)` is a **total owed**, not an increment. That is
the right shape and it stays: it makes advancing late cost nothing and makes re-sending the
pool table idempotent. But a total owed, with nothing recorded against it, is *"everything you
have ever earned"* — so three points into Power Strike, then a relog, returns the three points
**and** keeps the skill. Same shape as the Heena loop.

This adds the other half: `crates/store/src/skillpoints.rs`.

---

## 1. The schema

```sql
CREATE TABLE IF NOT EXISTS character_skill_spend (
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    skill_id     INTEGER NOT NULL,
    tier         INTEGER NOT NULL,   -- the client's pool key, net::stats::tier_for_job
    points       INTEGER NOT NULL,   -- points that LEFT the pool for this skill
    PRIMARY KEY (character_id, skill_id)
);
CREATE INDEX IF NOT EXISTS idx_skill_spend_pool
    ON character_skill_spend(character_id, tier);
```

A whole new table, so a plain `CREATE TABLE IF NOT EXISTS` from `Store::init` is enough — the
opposite of a *column* added to an existing table, which needs the `PRAGMA table_info` guard
`add_experience_column` uses. **No row means nothing spent**, exactly one representation of it,
the same way `quest_state` has exactly one representation of "never started".

### 1.1 Why a stored counter and not a derived total — this is the whole design

The obvious alternative is to derive: every point spent produces a level, so
`spent = SUM(character_skills.level)`. No new table, no migration, nothing to keep in step.

**It is wrong, and `!learn` is why.** `!learn`'s own doc block promises *"every skill of this
job's book on the bar, without spending a single skill point"*. Under derivation that promise
cannot be kept: 24 skills at ceilings of 15–20 is several hundred derived "spends" against a
first-job entitlement that tops out at 61. The command would silently stop being free, and
nothing on screen would say why.

Derivation gets exactly one thing right — `!resetsp` refunds for free, because erasing the
levels erases the derived total. That half is preserved below by other means.

`research/skill-points.md` §9 proposed *"one row per `(character_id, tier)` holding spent"*.
That was a sketch and it is one row too coarse:

* A per-pool counter cannot answer *"what did this one skill cost?"*, so forgetting a single
  skill can only be done by zeroing the whole pool.
* `gm_reset_sp` forgets skills **one at a time** and collects per-skill failures. A blanket
  zero beside a partial failure hands back points for a skill the player still has — the Heena
  loop in new clothes.

Per **skill**, the refund is exactly what that skill cost, and
`Store::forget_skill_and_refund` does the deletion and the refund in **one transaction**, so a
half-run reset is not representable.

The `tier` lives on the row because `crates/store/` has no skill table and cannot map a skill
id to a job book. The caller supplies it, and it is rewritten on every spend so the caller's
current mapping is always the authority.

### 1.2 The invariant this buys

**`points` is never more than the skill's `level`.** A point always produces a level; a level
is only removed together with its points; `!learn` moves the level up without the points, never
the other way. It is a cross-table consistency check that can be run against
`character_skills` at any time — `the_ledger_never_exceeds_the_skill_level` walks it across
every transition rather than at one moment.

---

## 2. The balance

```rust
pub fn balance(entitlement: u32, spent: u32) -> u32 { entitlement.saturating_sub(spent) }
```

**The subtraction does not happen in the store, and cannot.** `world` depends on `store`; the
arrow only goes one way, so `crates/store/` cannot call `world::skillpoints::entitlement`. The
entitlement is therefore a **parameter** on every call in the new module.

That is the honest split rather than a workaround: the store knows what was spent, the world
knows what was earned, and the subtraction belongs where both are in scope.

`saturating_sub`, not a checked subtraction. An over-spent pool is a state that can legitimately
exist — a rule change, an entitlement that shrank, a hand-edited row — and the answer to *"how
many may I still spend"* is then **zero**, never an error and certainly never `u32::MAX`.
`world::skillpoints::top_up` saturates in the same direction and for the same reason.

### 2.1 The API

| call | what it is for |
|---|---|
| `balance(entitlement, spent) -> u32` | the pure arithmetic, no database |
| `Store::skill_points_spent(chr, tier) -> u32` | one pool's total |
| `Store::skill_points_spent_by_tier(chr) -> Vec<(u8, u32)>` | every pool, tier order, for building the `0x007C` table |
| `Store::skill_points_available(chr, tier, entitlement) -> u32` | the two above, subtracted |
| `Store::spend_and_raise_skill(chr, skill, tier, entitlement, count) -> SkillUp` | **what `0x013B` calls.** Charge and level in one transaction |
| `Store::spend_skill_points(...) -> SpendOutcome` | charge only. Not for `0x013B` |
| `Store::forget_skill_and_refund(chr, skill) -> u32` | one skill: delete and refund, one transaction |
| `Store::forget_all_skills_and_refund(chr) -> Refunded` | **what `!resetsp` calls.** All of it, one transaction |
| `Store::skill_point_ledger(chr) -> Vec<SpendRow>` | diagnostics and tests |

`SpendOutcome` is `#[must_use]`, and that is load-bearing rather than decorative: it caught
**twelve** discarded answers in this module's own tests on the first `clippy -D warnings` run,
which is precisely the shape of the Heena bug (three call sites captured the store's answer
into a log string and carried on). Those twelve are now a `must_spend` helper that asserts the
setup spend was granted.

### 2.2 Tier 0 is a success that charges nothing

`research/skill-points.md` §4.1, **[L]** every instruction: for a tier-0 job the client computes
SP itself as `min(10, level) - 1 - (levels in 1000, 1001, 1002)` and never reads a pool. A
server-side counter there could only disagree with the screen.

So tier 0 returns `SpendOutcome::NotCharged` — a **success** that writes no row — rather than a
refusal. Refusing would break every beginner skill-up for a caller that forgot to branch;
"nothing to record" cannot. Beginner clicks behave exactly as they do today.

A tier **above 10** is refused: `FUN_1402cb030` is `cmp dl, 0xa / ja return-0`, so those points
would vanish from every screen while lighting the unspent-points indicator at `charstat+0xef`
forever.

---

## 3. What the two GM commands must now do

### 3.1 `!learn` — nothing. That is the requirement.

It grants levels through `set_skill_level` and **must not** touch the ledger. Its doc block
already promises the grant is free; the stored counter is what lets that stay true.
`a_granted_level_costs_no_skill_points` is the assertion: 55 levels of skills, zero points out
of the pool. Under a derived total the same character would have read 55 spent.

### 3.2 `!resetsp` — its chat line stays true, but for a different reason

Today the line reads:

> *"The points come back on their own — this server computes the pool from your LEVEL rather
> than tracking a balance, so a forgotten skill is the whole refund."*

The **second half stays true and the first half becomes false.** There is a balance now, and a
forgotten skill is still the whole refund because the forget and the refund are one
transaction. The wording has to change or it becomes exactly the kind of comment `CLAUDE.md`
warns about — a sentence describing a guarantee that lives somewhere else. Suggested:

> *"Forgot N skill(s): … — and P skill point(s) went back into the pool. A forgotten skill is
> the whole refund: the points are returned in the same transaction that erases the level, so
> there is no state where you keep the skill and get the points."*

**The implementation must change too.** The current per-skill loop over `set_skill_level(.., 0)`
would erase the levels and leave the ledger charged, so the points would *not* come back. Two
correct shapes, both provided:

* `forget_all_skills_and_refund(chr) -> Refunded { skills, points }` — one transaction, all or
  nothing. This is the one to use.
* `forget_skill_and_refund(chr, skill_id) -> u32` — per skill, if the per-skill failure
  reporting in the current loop is worth keeping. Each iteration is atomic on its own.

What must **not** happen is a loop of `set_skill_level(.., 0)` followed by a blanket zero of
the pool: one failed forget in that shape hands back points for a skill the player still has.

`forgetting_everything_empties_both_tables_and_refunds_both_pools` asserts both counts —
`skills` and `points` — because they are genuinely independent here (`!learn`'s skills have no
ledger rows), and a test that checks one of two effects gives false confidence about the other.

---

## 4. Migration: the character who already has 7 points in Magic Claw

**They keep the 7 levels and their pool reads full. Once.**

The table is new and starts empty, so every existing character has no ledger rows, which reads
as `spent = 0`. A one-time amnesty, bounded by the entitlement and self-closing the moment they
spend anything.

The alternative was to backfill `points` from `character_skills.level`, and it is wrong for the
same reason derivation is wrong — with a measured example sitting in the owner's own database:

```text
maplecw.db, read 2026-08-28 from a copy:
  212  Idiot    level 11  job   0   1000@3, 1001@3, 1002@3    <- tier 0, never charged
  213  Cobalt   level 14  job 100   1000001 @ level 15
```

`world::skillpoints::entitlement(Tier::First, 14)` is `1 + 3 x 4 = 13`. **Cobalt has 15 levels
in a skill out of a pool that has only ever been worth 13** — those levels came from `!learn`,
which was free by contract. Backfilling would bill them 15 against 13, saturate their pool at zero,
and leave it there until they ran `!resetsp`. **[L]** on the rows, **[D]** on the entitlement.

Charging for the past to avoid one free round in the present is the worse trade, and it is the
one that cannot be undone.

`Idiot` is unaffected in either direction: job 0 is tier 0, the client computes that pool
itself, and this ledger never records it.

The migration is covered three ways, deliberately, because a schema change that only ever runs
against files the test suite created is the case a broken migration still passes:

| test | what it opens |
|---|---|
| `a_character_who_already_has_skills_is_not_stranded` | in-memory, synthetic |
| `a_database_written_before_the_ledger_upgrades` | a real file with the table dropped back out |
| `remys_real_database_gains_the_ledger_without_stranding_anyone` | a **copy** of `maplecw.db`, sidecars included — skipped if absent |

The last one ran: 3 characters, every pool full, every skill level intact, a spend works on the
upgraded file, and a second open is a no-op. **[L]**

---

## 5. Wire it like this — `0x013B`

**None of this was done.** `crates/world/src/session/` belongs to the coordinator.

### Step 1 — pick the tier, in `on_skill_up`

```rust
let tier = net::stats::tier_for_job((req.skill_id / 10000) as u16);
```

`research/skill-points.md` §7.1: the request carries no pool field and never will — the client
already knows which tab it is on, and the server charges `tier(skillId / 10000)`. Skill
`2001005` → job 200 → tier **1**. Skill `1000` → job 0 → tier **0**, the client's own budget.

### Step 2 — the entitlement, from `world::skillpoints`

```rust
let entitlement = match tier {
    0 => 0,                                                     // never reached: NotCharged
    1 => skillpoints::entitlement(skillpoints::Tier::First,  chr.level),
    2 => skillpoints::entitlement(skillpoints::Tier::Second, chr.level),
    _ => 0,   // no rule exists above 2nd job - see §6
};
```

### Step 3 — one call, and return early on the refusal

Replace the existing `set_skill_level` with:

```rust
let up = self.store.spend_and_raise_skill(chr.id, req.skill_id, tier, entitlement, granted)?;
if let store::SpendOutcome::Refused(why) = up.spend {
    return vec![self.skill_reply(
        net::skills::skill_up_refused(net::skills::SkillUpRefusal::NotYours),
        format!("skill {} not raised: {why}", req.skill_id),   // SpendRefusal implements Display
    )];
}
// only past here: the level is up.level, the charge happened, and both are committed
```

**Every effect hangs off `up.spend`, not off the request.** The `0x0081`, the chat notice and
the `0x007C` below all sit after that early return, not each behind their own condition —
separately is how one gets missed.

`granted` keeps its existing clamp (`req.count.min(ceiling - level)`); the store charges exactly
that and raises the level by exactly that, so the two cannot disagree. `up.level` is the new
level for the `0x0081` entry.

**Which refusal byte to send is the coordinator's call and is not settled here.**
`SkillUpRefusal::NotYours` is what the handler already sends for "could not save the skill";
whether a client with an empty pool would rather see `AtMaxLevel` is a screen question this pass
did not measure. Whatever it is, it must still **clear the latch** — an unanswered `0x013B`
disables `user+0x2330`'s whole request family for the session.

### Step 4 — the `0x007C`, or the number on screen never moves

`research/skill-points.md` §6.2, **[L]**: **the client never decrements a pool itself.** Nothing
but the two packet decoders ever writes the list. So a `0x013B` answered with `0x0081` alone
leaves the same point spendable for the rest of the session, and this whole ledger would be
invisible — the database right and the screen wrong, which is the harder failure to diagnose.

After a successful, *charged* skill-up (skip it for `NotCharged` — tier 0 has no pool):

```rust
let mut pools = Vec::new();
for (t, tier_enum) in [(1u8, Tier::First), (2u8, Tier::Second)] {
    let owed = skillpoints::entitlement(tier_enum, chr.level);
    let left = self.store.skill_points_available(chr.id, t, owed)?;
    if owed > 0 { pools.push(net::stats::SpPool { job_level: t, amount: left }); }
}
// mask bit 15, extended encoding
StatChange { sp: Some(net::stats::Sp::Extended(pools)), ..Default::default() }
```

**It replaces, it does not add** (§5.1 of the other document): the extended arm clears the whole
list before reading the count, so every SP-bearing `0x007C` must carry **every** pool, not the
one that changed. A `count = 0` wipes SP to zero — `Sp::empty_extended()` is destructive, not a
no-op.

Guard the encoding with `Sp::matches_job(job)` exactly as `gm_job` already does; the wrong shape
desynchronises every byte after it in a packet with no length prefix.

### Step 5 — `gm_job` sends the same subtraction

`gm_job` currently builds its pools from `entitlement` alone. One line each:

```rust
let amount = crate::skillpoints::entitlement(tier, chr.level);
let amount = self.store.skill_points_available(chr.id, wire_tier, amount)?;
```

Without this, an advancement still hands the spent points back — which is the reported bug, in
the one command most likely to be typed right after spending.

### Step 6 — `gm_reset_sp`

Swap the `set_skill_level(.., 0)` loop for `forget_all_skills_and_refund`, report both counts,
and update the chat line (§3.2). Then send the `0x007C` from step 4, or the refunded points do
not appear until something else moves the table.

---

## 6. What this does NOT establish

1. **Anything above 2nd job.** `world::skillpoints::Tier` has two variants and no rule exists
   for tiers 3–4, so step 2's `_ => 0` refuses every spend there. Nothing can reach 3rd job on
   this server today, and inventing an entitlement would be a number nobody asked for — but the
   day a 3rd job exists, that arm is where it breaks, silently and in the direction of "the
   button does nothing".
2. **Whether the client's `+` button and this ledger will ever disagree.** The button is greyed
   from `GetSP(job)`, which reads the *last table the server sent*. Keep step 4 and they cannot
   drift; drop it and they will, with the server right and the screen wrong.
3. **Any of this on a screen.** No client run was spent. Every claim here is about the database
   and the arithmetic; the wire half is `research/skill-points.md`'s and is still unconfirmed
   for the SP bit — no `0x007C` with bit 15 has ever been sent by this server.
4. **Concurrency beyond one transaction.** The read of the pool and the write of the row are one
   transaction, so two spends cannot both see the same balance. There is no test for that — a
   racing test is flaky, and the guarantee is SQLite's rather than ours.
5. **What a refund should do to a skill the player earned *and* was granted.** A skill at level
   20 with 4 charged points refunds 4 and loses all 20, because forgetting is all-or-nothing per
   skill. That is what `!resetsp` means; a partial unlearn does not exist and nobody has asked
   for one.
