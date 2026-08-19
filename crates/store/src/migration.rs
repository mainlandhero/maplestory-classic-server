//! The handoff from the login/character-select server to a channel.
//!
//! The two are separate processes and share nothing but this database, so a migration has
//! to be written down somewhere both can see. That is the whole job of this module.
//!
//! # What a migration is, and what it is not
//!
//! **It is not authentication.** The token that actually travels is a `u32` - that is all
//! the client's migration packet has room for - and a 32-bit value handed out in sequence
//! is not a secret. What this table buys is *single use*: a migration is claimed exactly
//! once, so a replayed handoff cannot put a second connection into the world as the same
//! character. That is a real property and worth having; it is not the same as proving who
//! is on the far end. Say so when reporting.

use rand::RngCore;
use rusqlite::OptionalExtension;

use crate::db::Store;
use crate::error::{Result, StoreError};

/// A migration the channel server has accepted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimedMigration {
    pub account_id: i64,
    pub character_id: u32,
    pub world_id: u32,
    pub channel_id: u32,
}

/// How long a minted migration stays claimable.
///
/// The client reconnects immediately - the measured gap between the login socket closing
/// and the new connection arriving was **under a millisecond** - so this is generous by
/// three orders of magnitude and exists only so an abandoned migration does not sit
/// claimable forever.
pub const MIGRATION_TTL_SECS: i64 = 60;

impl Store {
    /// Mint a migration and return the seed to put in the packet.
    ///
    /// The seed is random rather than derived from the character id. Deriving it would
    /// make the value predictable *and* make two migrations of the same character collide
    /// on the primary key, so the second would fail exactly when a player re-enters the
    /// world - which is the common case, not an edge one.
    pub fn create_migration(
        &self,
        account_id: i64,
        character_id: u32,
        world_id: u32,
        channel_id: u32,
    ) -> Result<u32> {
        let conn = self.conn();
        let now = Store::now();

        // Retry on collision rather than trusting 32 bits to be unique. Ten attempts is
        // far past the point where a collision means something else is wrong.
        for _ in 0..10 {
            let seed = random_seed();
            let inserted = conn.execute(
                "INSERT OR IGNORE INTO migrations
                     (seed, account_id, character_id, world_id, channel_id, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![seed, account_id, character_id, world_id, channel_id, now],
            )?;
            if inserted == 1 {
                return Ok(seed);
            }
        }
        Err(StoreError::MigrationSeedExhausted { tries: 10 })
    }

    /// Claim a migration. Returns `None` if there is no unconsumed, unexpired one.
    ///
    /// The consume is inside the `UPDATE`'s `WHERE`, so two connections racing the same
    /// seed cannot both win: SQLite reports one row changed to exactly one of them.
    pub fn claim_migration(&self, seed: u32) -> Result<Option<ClaimedMigration>> {
        let conn = self.conn();
        let now = Store::now();
        let changed = conn.execute(
            "UPDATE migrations SET consumed_at = ?2
              WHERE seed = ?1 AND consumed_at IS NULL AND created_at >= ?3",
            rusqlite::params![seed, now, now - MIGRATION_TTL_SECS],
        )?;
        if changed == 0 {
            return Ok(None);
        }
        conn.query_row(
            "SELECT account_id, character_id, world_id, channel_id
               FROM migrations WHERE seed = ?1",
            rusqlite::params![seed],
            |row| {
                Ok(ClaimedMigration {
                    account_id: row.get(0)?,
                    character_id: row.get::<_, i64>(1)? as u32,
                    world_id: row.get::<_, i64>(2)? as u32,
                    channel_id: row.get::<_, i64>(3)? as u32,
                })
            },
        )
        .optional()
        .map_err(Into::into)
    }

    /// Drop consumed and expired migrations. Returns how many went.
    pub fn purge_migrations(&self) -> Result<usize> {
        let conn = self.conn();
        let cutoff = Store::now() - MIGRATION_TTL_SECS;
        Ok(conn.execute(
            "DELETE FROM migrations WHERE consumed_at IS NOT NULL OR created_at < ?1",
            rusqlite::params![cutoff],
        )?)
    }
}

/// A non-zero random `u32`.
///
/// Zero is excluded so that "the client sent no seed" and "the client sent seed 0" are
/// distinguishable in a log - an all-zero field is what an unwritten buffer looks like.
fn random_seed() -> u32 {
    let mut rng = rand::rngs::OsRng;
    loop {
        let seed = rng.next_u32();
        if seed != 0 {
            return seed;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::opcode::Character;

    fn seeded_character(name: &str) -> Character {
        Character { name: name.to_string(), ..Character::default() }
    }

    fn store_with_character() -> (Store, i64, u32) {
        let store = Store::open_in_memory().unwrap();
        let account_id = store.create_account("maplecw", "correct horse battery").unwrap();
        let id = store
            .create_character(account_id, 0, &seeded_character("Wanderer"))
            .unwrap()
            .id;
        (store, account_id, id)
    }

    #[test]
    fn a_minted_migration_can_be_claimed_once() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration(account_id, id, 0, 0).unwrap();

        let claimed = store.claim_migration(seed).unwrap().expect("first claim wins");
        assert_eq!(claimed.character_id, id);
        assert_eq!(claimed.account_id, account_id);

        assert_eq!(
            store.claim_migration(seed).unwrap(),
            None,
            "a replayed migration must not put a second connection into the world"
        );
    }

    #[test]
    fn an_unminted_seed_is_never_claimable() {
        let (store, _, _) = store_with_character();
        assert_eq!(store.claim_migration(0xDEADBEEF).unwrap(), None);
    }

    #[test]
    fn seeds_are_never_zero_so_an_empty_field_is_distinguishable() {
        let (store, account_id, id) = store_with_character();
        for _ in 0..32 {
            assert_ne!(store.create_migration(account_id, id, 0, 0).unwrap(), 0);
        }
    }

    /// Deriving the seed from the character id would collide the second time a player
    /// enters the world, which is the ordinary case.
    #[test]
    fn the_same_character_can_migrate_more_than_once() {
        let (store, account_id, id) = store_with_character();
        let first = store.create_migration(account_id, id, 0, 0).unwrap();
        let second = store.create_migration(account_id, id, 0, 0).unwrap();
        assert_ne!(first, second);
        assert!(store.claim_migration(first).unwrap().is_some());
        assert!(store.claim_migration(second).unwrap().is_some());
    }

    #[test]
    fn purging_clears_claimed_migrations() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration(account_id, id, 0, 0).unwrap();
        store.claim_migration(seed).unwrap().unwrap();
        assert_eq!(store.purge_migrations().unwrap(), 1);
        assert_eq!(store.claim_migration(seed).unwrap(), None);
    }

    /// A deleted character must not leave a claimable migration behind.
    #[test]
    fn deleting_the_character_removes_its_migrations() {
        let (store, account_id, id) = store_with_character();
        let seed = store.create_migration(account_id, id, 0, 0).unwrap();
        assert!(store.delete_character(account_id, id).unwrap());
        assert_eq!(store.claim_migration(seed).unwrap(), None);
    }
}
