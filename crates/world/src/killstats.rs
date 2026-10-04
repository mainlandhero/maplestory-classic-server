//! **Every kill and what it dropped, counted in memory and written in batches.**
//!
//! The owner, 2026-10-01: *"Make this data cached so that it doesn't burden the server or the
//! database too much. This data at worst can be 30 or 60 minutes out of date."* So a kill costs
//! a hash-map update under one mutex and nothing else. [`spawn_flusher`] writes what has
//! accumulated every [`FLUSH_EVERY`] as one transaction (`store::killstats`), and purges buckets
//! older than seven days every [`PURGE_EVERY`].
//!
//! **What a crash costs**: at most one flush interval of counts, because they were only in
//! memory. That is the trade the owner asked for, and it is stated rather than hidden.

use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use store::killstats::KillBatch;

/// How often a channel writes its counts. Well inside the owner's 30 minutes, so the page's own
/// cache is the only staleness anyone sees.
pub const FLUSH_EVERY: Duration = Duration::from_secs(5 * 60);
/// How often the seven-day purge runs.
pub const PURGE_EVERY: Duration = Duration::from_secs(60 * 60);

fn pending() -> &'static Mutex<KillBatch> {
    static PENDING: OnceLock<Mutex<KillBatch>> = OnceLock::new();
    PENDING.get_or_init(|| Mutex::new(KillBatch::default()))
}

/// One kill on this channel: `template` killed by `character`, and what actually fell.
pub fn note_kill(template: u32, character: u32, dropped: &[crate::droptables::Rolled]) {
    let items: Vec<(u32, u32)> = dropped.iter().map(|r| (r.item_id, r.quantity)).collect();
    pending().lock().unwrap_or_else(|e| e.into_inner()).note(store::Store::unix_now(), template, character, &items);
}

/// One Companion's Magic Box opened by `character`, and the prizes it gave. Counted in the same
/// tables under the box's own id, [`crate::magicbox::BOX`], as though the box were a monster
/// and opening it a kill; `crate::dropweb::live_json` names that row `"box"` and keeps it out
/// of the page's kill and player totals.
pub fn note_box_opened(character: u32, prizes: &[(u32, u16)]) {
    let items: Vec<(u32, u32)> = prizes.iter().map(|&(i, q)| (i, u32::from(q))).collect();
    pending().lock().unwrap_or_else(|e| e.into_inner()).note(store::Store::unix_now(), crate::magicbox::BOX, character, &items);
}

/// One finished Forest or Deep Forest course by `character`, and the prizes its goal gave -
/// counted like a box, under [`crate::jumpquest::STATS_ROW`]; `crate::dropweb::live_json` names
/// that row `"jq"`. The quest item is not a prize and is not counted.
pub fn note_jump_quest(character: u32, prizes: &[(u32, u16)]) {
    let items: Vec<(u32, u32)> = prizes.iter().map(|&(i, q)| (i, u32::from(q))).collect();
    pending().lock().unwrap_or_else(|e| e.into_inner()).note(store::Store::unix_now(), crate::jumpquest::STATS_ROW, character, &items);
}

/// How many times `item` has been counted from `template` and not yet written. For tests.
#[cfg(test)]
pub fn pending_drops(template: u32, item: u32) -> u64 {
    let held = pending().lock().unwrap_or_else(|e| e.into_inner());
    held.drops.iter().filter(|((_, t, i), _)| *t == template && *i == item).map(|(_, (d, _))| d).sum()
}

/// Write everything counted so far, and start counting afresh. A failed write puts the counts
/// back, so the next flush carries them.
pub fn flush(store: &store::Store) {
    let batch = std::mem::take(&mut *pending().lock().unwrap_or_else(|e| e.into_inner()));
    if batch.is_empty() {
        return;
    }
    let kills: u64 = batch.kills.values().sum();
    match store.flush_kill_stats(&batch) {
        Ok(()) => crate::server::log(&format!("   killstats: wrote {kills} kill(s) to the seven-day table")),
        Err(e) => {
            crate::server::log(&format!("   killstats: could not write {kills} kill(s), keeping them for the next flush: {e}"));
            let mut held = pending().lock().unwrap_or_else(|e| e.into_inner());
            merge(&mut held, batch);
        }
    }
}

fn merge(into: &mut KillBatch, from: KillBatch) {
    for (k, v) in from.kills {
        *into.kills.entry(k).or_default() += v;
    }
    into.killers.extend(from.killers);
    for (k, (d, q)) in from.drops {
        let e = into.drops.entry(k).or_default();
        e.0 += d;
        e.1 += q;
    }
}

/// The background writer: a flush every [`FLUSH_EVERY`], a purge every [`PURGE_EVERY`] and one
/// at start, so a server that was down for a week starts clean.
pub fn spawn_flusher(store: Arc<store::Store>) {
    purge(&store);
    std::thread::spawn(move || {
        let mut since_purge = Duration::ZERO;
        loop {
            std::thread::sleep(FLUSH_EVERY);
            flush(&store);
            since_purge += FLUSH_EVERY;
            if since_purge >= PURGE_EVERY {
                since_purge = Duration::ZERO;
                purge(&store);
            }
        }
    });
}

fn purge(store: &store::Store) {
    match store.purge_kill_stats(store::Store::unix_now()) {
        Ok(0) => {}
        Ok(n) => crate::server::log(&format!("   killstats: purged {n} row(s) older than seven days")),
        Err(e) => crate::server::log(&format!("   killstats: purge failed: {e}")),
    }
}

/// Kills of `template` counted and not yet written - for tests of the call sites.
#[cfg(test)]
pub fn pending_kills(template: u32) -> u64 {
    pending().lock().unwrap_or_else(|e| e.into_inner()).kills.iter().filter(|((_, t), _)| *t == template).map(|(_, k)| k).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A failed write keeps the counts: merging a batch back adds to whatever arrived meanwhile.
    #[test]
    fn merging_a_batch_back_adds_up() {
        let mut held = KillBatch::default();
        held.note(3_600, 2, 200, &[(0, 5)]);
        let mut back = KillBatch::default();
        back.note(3_600, 2, 201, &[(0, 4), (4_000_001, 1)]);
        merge(&mut held, back);
        assert_eq!(held.kills[&(3_600, 2)], 2);
        assert_eq!(held.killers.len(), 2);
        assert_eq!(held.drops[&(3_600, 2, 0)], (2, 9));
        assert_eq!(held.drops[&(3_600, 2, 4_000_001)], (1, 1));
    }
}
