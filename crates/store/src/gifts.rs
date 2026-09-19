//! **Gift Drops** - items a GM queues for a character or for every account, claimed through
//! the Maple Administrator's dialogue.
//!
//! The owner, 2026-09-18, with a screenshot of the modern client's Gift Drop window: *"Can you take
//! a look and see if a gift drop functionality exists in Classic Client so I can send items to
//! players as compensation if necessary?"* It does not - the window is later than this client -
//! and the Cash Shop locker was refused for it (*"the Cash Shop should not handle items that
//! are not Cash Items"*), so: *"Can we do it via our usual MapleStory Administrator, but this
//! time it is via !giftdrop, and our usual show NPC chat dialogue."* Then: *"I want all gifts
//! to expire in 7 days if unclaimed. Additionally, there should also be a !giftall command
//! which gives all accounts (not character) an item. The player can claim it on any character
//! they want."* And: *"The !giftall command also gives items in giftdrop that expires in 7
//! days if unclaimed."*
//!
//! A gift is a row here until it is claimed, refused, or [`GIFT_TTL_SECS`] old. It is addressed
//! either to one **character** (`!giftdrop`) or to one **account** (`!giftall` writes one row
//! per account); an account's gift is offered to whichever of its characters is playing and is
//! settled for the whole account by the one that claims it. Nothing is deleted: an expired or
//! settled row stays for the record.
//!
//! **Every effect hangs off the transition** (`CLAUDE.md`, the Heena rule): [`Store::settle_gift`]
//! is the one write that turns a pending row into a claimed or refused one, and it reports
//! whether it did. The session hands the item over only on `true`, so a second click on a box
//! that was already answered gives nothing, and two characters of one account racing for an
//! account gift can only both succeed if one of them has a time machine.

use rusqlite::{Connection, OptionalExtension};

use crate::db::Store;
use crate::error::Result;

/// **Seven days.** The owner: *"I want all gifts to expire in 7 days if unclaimed."* Written on the
/// row at queue time (`expires_at`), so a later change to this constant does not move a gift
/// already promised.
pub const GIFT_TTL_SECS: i64 = 7 * 24 * 60 * 60;

/// Who a gift is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GiftTarget {
    Character(u32),
    Account(i64),
}

/// One queued gift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gift {
    pub id: i64,
    pub target: GiftTarget,
    pub item_id: u32,
    pub count: u16,
    /// What the Administrator says above the reward. May be empty.
    pub message: String,
    /// Who queued it - the GM's character name, for the log and the box.
    pub sender: String,
    /// Unix seconds.
    pub queued_at: i64,
    /// Unix seconds; the row stops being offered at this instant.
    pub expires_at: i64,
}

impl Gift {
    /// Whole days left before it expires, rounded up, never below 0.
    pub fn days_left(&self, now: i64) -> i64 {
        ((self.expires_at - now).max(0) + 86_399) / 86_400
    }
}

pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS gifts (
            id           INTEGER PRIMARY KEY AUTOINCREMENT,
            -- Exactly one of the two is set: a gift for one character, or for every
            -- character of one account.
            character_id INTEGER,
            account_id   INTEGER,
            item_id      INTEGER NOT NULL,
            count        INTEGER NOT NULL,
            message      TEXT    NOT NULL DEFAULT '',
            sender       TEXT    NOT NULL DEFAULT '',
            queued_at    INTEGER NOT NULL,
            expires_at   INTEGER NOT NULL,
            -- NULL while pending. At most one of the two is set once settled.
            claimed_at   INTEGER,
            -- The character that claimed or refused it; the account gift's record of which.
            settled_by   INTEGER,
            refused_at   INTEGER
        );
        CREATE INDEX IF NOT EXISTS gifts_by_character ON gifts (character_id, claimed_at, refused_at);
        CREATE INDEX IF NOT EXISTS gifts_by_account   ON gifts (account_id, claimed_at, refused_at);
        "#,
    )?;
    Ok(())
}

const GIFT_COLUMNS: &str = "id, character_id, account_id, item_id, count, message, sender, queued_at, expires_at";

fn gift_from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<Gift> {
    let character: Option<i64> = r.get(1)?;
    let account: Option<i64> = r.get(2)?;
    Ok(Gift {
        id: r.get(0)?,
        target: match (character, account) {
            (Some(c), _) => GiftTarget::Character(c as u32),
            (None, Some(a)) => GiftTarget::Account(a),
            (None, None) => GiftTarget::Character(0),
        },
        item_id: r.get::<_, i64>(3)? as u32,
        count: r.get::<_, i64>(4)?.clamp(1, i64::from(u16::MAX)) as u16,
        message: r.get(5)?,
        sender: r.get(6)?,
        queued_at: r.get(7)?,
        expires_at: r.get(8)?,
    })
}

impl Store {
    /// Queue `count` of `item_id` for `target`, expiring [`GIFT_TTL_SECS`] after `now`.
    /// Returns the gift's id.
    pub fn queue_gift(
        &self,
        target: GiftTarget,
        item_id: u32,
        count: u16,
        message: &str,
        sender: &str,
        now: i64,
    ) -> Result<i64> {
        let (character, account) = match target {
            GiftTarget::Character(c) => (Some(i64::from(c)), None),
            GiftTarget::Account(a) => (None, Some(a)),
        };
        let conn = self.conn();
        conn.execute(
            "INSERT INTO gifts (character_id, account_id, item_id, count, message, sender, queued_at, expires_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![character, account, item_id, i64::from(count.max(1)), message, sender, now, now + GIFT_TTL_SECS],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// **`!giftall`**: one gift per account that exists right now. Returns how many were
    /// queued. An account created afterwards is not owed one - the command is a snapshot,
    /// said in its ack.
    pub fn queue_gift_for_every_account(
        &self,
        item_id: u32,
        count: u16,
        message: &str,
        sender: &str,
        now: i64,
    ) -> Result<usize> {
        let accounts: Vec<i64> = {
            let conn = self.conn();
            let mut stmt = conn.prepare("SELECT id FROM accounts ORDER BY id")?;
            let ids = stmt.query_map([], |r| r.get::<_, i64>(0))?;
            ids.collect::<std::result::Result<Vec<_>, _>>()?
        };
        for a in &accounts {
            self.queue_gift(GiftTarget::Account(*a), item_id, count, message, sender, now)?;
        }
        Ok(accounts.len())
    }

    /// Every gift `character_id` (of `account_id`) can claim at `now`: its own and its
    /// account's, unsettled and unexpired, oldest first.
    pub fn pending_gifts(&self, character_id: u32, account_id: i64, now: i64) -> Result<Vec<Gift>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(&format!(
            "SELECT {GIFT_COLUMNS} FROM gifts
             WHERE (character_id = ?1 OR account_id = ?2)
               AND claimed_at IS NULL AND refused_at IS NULL AND expires_at > ?3
             ORDER BY id"
        ))?;
        let rows = stmt.query_map(rusqlite::params![character_id, account_id, now], gift_from_row)?;
        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    /// The oldest claimable gift, if any.
    pub fn next_gift(&self, character_id: u32, account_id: i64, now: i64) -> Result<Option<Gift>> {
        Ok(self.pending_gifts(character_id, account_id, now)?.into_iter().next())
    }

    /// **The transition.** Mark gift `id` claimed (or refused) by `character_id` of
    /// `account_id` at `now`. `true` only if the row was pending, unexpired, and addressed to
    /// that character or that account; a second call, an expired row, or somebody else's gift
    /// writes nothing and returns `false`.
    pub fn settle_gift(&self, id: i64, character_id: u32, account_id: i64, claimed: bool, now: i64) -> Result<bool> {
        let column = if claimed { "claimed_at" } else { "refused_at" };
        let n = self.conn().execute(
            &format!(
                "UPDATE gifts SET {column} = ?1, settled_by = ?2
                 WHERE id = ?3 AND (character_id = ?2 OR account_id = ?4)
                   AND claimed_at IS NULL AND refused_at IS NULL AND expires_at > ?1"
            ),
            rusqlite::params![now, character_id, id, account_id],
        )?;
        Ok(n == 1)
    }

    /// A gift by id, settled, expired or not - for the log after a settle.
    pub fn gift(&self, id: i64) -> Result<Option<Gift>> {
        Ok(self
            .conn()
            .query_row(&format!("SELECT {GIFT_COLUMNS} FROM gifts WHERE id = ?1"), [id], gift_from_row)
            .optional()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Queue two for a character and one for another; read them back oldest first; settle
    /// one; the settled one is gone from the pending list, a second settle is refused, and
    /// somebody else cannot settle it.
    #[test]
    fn a_gift_is_pending_until_exactly_one_settle_by_its_owner() {
        let store = Store::open_in_memory().unwrap();
        let a = store.queue_gift(GiftTarget::Character(200), 1_302_000, 1, "Sorry about the crash", "Wisp", 1_000).unwrap();
        let b = store.queue_gift(GiftTarget::Character(200), 2_000_000, 50, "", "Wisp", 1_001).unwrap();
        let _other = store.queue_gift(GiftTarget::Character(201), 2_000_000, 1, "", "Wisp", 1_002).unwrap();
        let pending = store.pending_gifts(200, 1, 1_500).unwrap();
        assert_eq!(pending.iter().map(|g| g.id).collect::<Vec<_>>(), vec![a, b], "oldest first, only mine");
        assert_eq!(pending[0].message, "Sorry about the crash");
        assert_eq!(pending[0].expires_at, 1_000 + GIFT_TTL_SECS);
        assert_eq!(pending[0].days_left(1_500), 7);
        assert_eq!(store.next_gift(200, 1, 1_500).unwrap().unwrap().id, a);

        assert!(!store.settle_gift(a, 201, 2, true, 2_000).unwrap(), "not 201's gift");
        assert!(store.settle_gift(a, 200, 1, true, 2_000).unwrap(), "claimed");
        assert!(!store.settle_gift(a, 200, 1, true, 2_001).unwrap(), "a second click claims nothing");
        assert!(!store.settle_gift(a, 200, 1, false, 2_001).unwrap(), "and cannot be refused after");
        assert_eq!(store.next_gift(200, 1, 2_100).unwrap().unwrap().id, b, "the next one is up");
        assert!(store.settle_gift(b, 200, 1, false, 2_002).unwrap(), "refused");
        assert!(store.next_gift(200, 1, 2_100).unwrap().is_none());
        assert_eq!(store.pending_gifts(201, 2, 2_100).unwrap().len(), 1, "201's is untouched");
        assert_eq!(store.gift(a).unwrap().unwrap().item_id, 1_302_000, "settled rows stay readable");
    }

    /// **Seven days, then gone.** Pending at six days and change, absent at seven, and a
    /// settle at seven writes nothing - the row itself stays.
    #[test]
    fn a_gift_expires_seven_days_after_it_was_queued() {
        let store = Store::open_in_memory().unwrap();
        let g = store.queue_gift(GiftTarget::Character(200), 2_000_000, 1, "", "Wisp", 1_000).unwrap();
        assert_eq!(store.pending_gifts(200, 1, 1_000 + GIFT_TTL_SECS - 1).unwrap().len(), 1);
        assert_eq!(store.gift(g).unwrap().unwrap().days_left(1_000 + GIFT_TTL_SECS - 1), 1);
        assert!(store.pending_gifts(200, 1, 1_000 + GIFT_TTL_SECS).unwrap().is_empty(), "expired");
        assert!(!store.settle_gift(g, 200, 1, true, 1_000 + GIFT_TTL_SECS).unwrap(), "too late to claim");
        assert!(store.gift(g).unwrap().is_some(), "the row is kept for the record");
    }

    /// **`!giftall` is one gift per account, claimable on any of its characters, once.** Two
    /// accounts, one with two characters: both characters see it, the first to claim settles
    /// it for both, and the other account's copy is untouched.
    #[test]
    fn an_account_gift_is_seen_by_every_character_of_the_account_and_settled_once() {
        let store = Store::open_in_memory().unwrap();
        let wisp = store.create_account("wisp", "correct horse battery").unwrap();
        let mint = store.create_account("mint", "correct horse battery").unwrap();
        let n = store.queue_gift_for_every_account(2_000_000, 5, "Thanks for testing", "Wisp", 1_000).unwrap();
        assert_eq!(n, 2);
        let mine = store.pending_gifts(200, wisp, 1_500).unwrap();
        let alt = store.pending_gifts(201, wisp, 1_500).unwrap();
        assert_eq!(mine.len(), 1);
        assert_eq!(mine, alt, "both of the account's characters see the same row");
        assert_eq!(mine[0].target, GiftTarget::Account(wisp));
        assert_eq!(store.pending_gifts(300, mint, 1_500).unwrap().len(), 1, "the other account has its own");

        assert!(!store.settle_gift(mine[0].id, 300, mint, true, 2_000).unwrap(), "not mint's row");
        assert!(store.settle_gift(mine[0].id, 201, wisp, true, 2_000).unwrap(), "claimed on the alt");
        assert!(store.pending_gifts(200, wisp, 2_100).unwrap().is_empty(), "and gone for the main too");
        assert!(!store.settle_gift(mine[0].id, 200, wisp, true, 2_100).unwrap(), "once");
        assert_eq!(store.pending_gifts(300, mint, 2_100).unwrap().len(), 1, "mint's still waits");
    }
}
