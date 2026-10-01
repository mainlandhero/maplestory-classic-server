//! Who is on whose friend list, and who has asked.
//!
//! The owner, 2026-09-21: *"Tester2 just tried adding the owner as a friend, but nothing showed up on
//! The owner's screen."* `research/friends-2026-09-21.md` is the decode; the wire is
//! [`net::friends`]; the handler is `world::session::friends`.
//!
//! # Two rows per friendship, and that is deliberate
//!
//! A friendship is stored as **two directed rows**, one per side, each with its own state and
//! its own group name. It is the shape the client wants - each side sees the other in a group
//! *it* chose - and it makes the pending half trivial: the requester's row is
//! [`FriendState::Requested`] and the target's is [`FriendState::Pending`] until they answer.
//!
//! A single undirected row would have to encode "who asked" in a column and then answer
//! "what does the owner see" with a `CASE`, which is the same information wearing a disguise.
//!
//! # The guards are asked and their answer is used
//!
//! `CLAUDE.md`'s Heena rule: every effect hangs off the transition. [`Store::request_friend`]
//! returns a [`FriendRequestOutcome`] and the session sends the client's own sentence for each
//! one; nothing is written on any refusal.

use rusqlite::Connection;

use crate::{Result, Store};

/// Where one directed row stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendState {
    /// This character asked; the other side has not answered.
    Requested,
    /// The other side asked this character. **This is the row that owes an answer.**
    Pending,
    /// Both sides agreed.
    Accepted,
}

impl FriendState {
    fn as_i64(self) -> i64 {
        match self {
            FriendState::Requested => 0,
            FriendState::Pending => 1,
            FriendState::Accepted => 2,
        }
    }

    fn from_i64(v: i64) -> FriendState {
        match v {
            0 => FriendState::Requested,
            2 => FriendState::Accepted,
            _ => FriendState::Pending,
        }
    }
}

/// One row of somebody's friend list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Friend {
    /// The other character.
    pub friend_id: u32,
    /// Their name, as the characters table has it now - not as it was when the row was
    /// written, so a rename cannot leave a stale name on a list.
    pub name: String,
    /// The group this side filed them under.
    pub group: String,
    pub state: FriendState,
}

/// What [`Store::request_friend`] did, or why it did nothing.
///
/// Every refusal has a sentence the client already owns - see [`net::friends::FriendNotice`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FriendRequestOutcome {
    /// Written: the requester is `Requested`, the target `Pending`.
    Asked,
    /// The target had already asked *this* character, so the two rows were settled at once.
    /// Both sides are now `Accepted`.
    AcceptedTheirs,
    /// Already on the list.
    AlreadyFriends,
    /// This character already asked and is still waiting.
    AlreadyAsked,
    /// The other side already asked and is waiting for THIS character to answer.
    TheyAreWaiting,
    /// Nobody by that name.
    NoSuchCharacter,
    /// Adding yourself.
    Yourself,
    /// This character's list is full.
    YourListIsFull,
    /// The other character's list is full.
    TheirListIsFull,
}

/// How many accepted friends one character may hold. **The owner set this to 50 on 2026-09-22.**
///
/// The client has its own idea of a list size - `0x2F` writes one to `ctx+0x118b` - but that
/// arm also pops *"Your friends list has increased by %d slots! Your wallet is %d Mesos
/// lighter"*, so it is the **purchase** result and not a way to state a capacity at login.
/// This is the server-side ceiling, and it is the one that decides: the 51st request is
/// refused with the client's own *"Your buddy list is full."* / *"The user's buddy list is
/// full"*, both of which exist in its string table.
///
/// **The window header reads `[n/0]`** until whatever normally writes `ctx+0x118b` is found,
/// so the count on screen and this number do not agree yet. That is a display gap, not a
/// limit gap - the refusal fires at 50 either way.
pub const FRIEND_LIST_LIMIT: usize = 50;

/// Create the table. Called from the schema on every open, so it must be idempotent.
pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS character_friends (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            friend_id    INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            group_name   TEXT    NOT NULL DEFAULT 'Default Group',
            state        INTEGER NOT NULL,
            asked_at     INTEGER NOT NULL DEFAULT 0,
            PRIMARY KEY (character_id, friend_id)
        );
        "#,
    )?;
    Ok(())
}

impl Store {
    /// One character's list, accepted and pending alike, lowest id first.
    ///
    /// Ordered for the same reason [`Store::skills`] is: a packet built twice has to be
    /// byte-identical, and a `HashMap` walk is not.
    pub fn friends(&self, character_id: u32) -> Result<Vec<Friend>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT f.friend_id, c.name, f.group_name, f.state
               FROM character_friends f
               JOIN characters c ON c.id = f.friend_id
              WHERE f.character_id = ?1
              ORDER BY f.friend_id",
        )?;
        let rows = stmt.query_map([i64::from(character_id)], |row| {
            Ok(Friend {
                friend_id: row.get::<_, i64>(0)? as u32,
                name: row.get(1)?,
                group: row.get(2)?,
                state: FriendState::from_i64(row.get(3)?),
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// The requests this character has not answered yet.
    pub fn friend_requests_waiting(&self, character_id: u32) -> Result<Vec<Friend>> {
        Ok(self
            .friends(character_id)?
            .into_iter()
            .filter(|f| f.state == FriendState::Pending)
            .collect())
    }

    /// Ask to be `target`'s friend.
    ///
    /// **Every refusal is a refusal**: nothing is written unless the answer is [`Asked`] or
    /// [`AcceptedTheirs`]. The two rows go in one transaction, so a list can never hold half
    /// a friendship.
    ///
    /// [`Asked`]: FriendRequestOutcome::Asked
    /// [`AcceptedTheirs`]: FriendRequestOutcome::AcceptedTheirs
    pub fn request_friend(
        &self,
        character_id: u32,
        target_id: u32,
        group: &str,
    ) -> Result<FriendRequestOutcome> {
        if character_id == target_id {
            return Ok(FriendRequestOutcome::Yourself);
        }
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let state = |a: u32, b: u32| -> Option<FriendState> {
            tx.query_row(
                "SELECT state FROM character_friends WHERE character_id = ?1 AND friend_id = ?2",
                rusqlite::params![i64::from(a), i64::from(b)],
                |row| row.get::<_, i64>(0),
            )
            .ok()
            .map(FriendState::from_i64)
        };
        let count = |who: u32| -> usize {
            tx.query_row(
                "SELECT COUNT(*) FROM character_friends WHERE character_id = ?1 AND state = 2",
                [i64::from(who)],
                |row| row.get::<_, i64>(0),
            )
            .unwrap_or(0) as usize
        };
        let outcome = match state(character_id, target_id) {
            Some(FriendState::Accepted) => FriendRequestOutcome::AlreadyFriends,
            Some(FriendState::Requested) => FriendRequestOutcome::AlreadyAsked,
            // They asked first and this is the answer: settle both rows.
            Some(FriendState::Pending) => FriendRequestOutcome::AcceptedTheirs,
            None if count(character_id) >= FRIEND_LIST_LIMIT => FriendRequestOutcome::YourListIsFull,
            None if count(target_id) >= FRIEND_LIST_LIMIT => FriendRequestOutcome::TheirListIsFull,
            None => FriendRequestOutcome::Asked,
        };
        match outcome {
            FriendRequestOutcome::Asked => {
                upsert(&tx, character_id, target_id, group, FriendState::Requested)?;
                upsert(&tx, target_id, character_id, "Default Group", FriendState::Pending)?;
            }
            FriendRequestOutcome::AcceptedTheirs => {
                upsert(&tx, character_id, target_id, group, FriendState::Accepted)?;
                upsert(&tx, target_id, character_id, "Default Group", FriendState::Accepted)?;
            }
            _ => {}
        }
        tx.commit()?;
        Ok(outcome)
    }

    /// Answer a request. `accept` settles both rows; a refusal deletes both.
    ///
    /// Returns `false` when there was no request to answer - and then nothing is written,
    /// which is the whole point of returning it.
    pub fn answer_friend_request(&self, character_id: u32, other_id: u32, accept: bool) -> Result<bool> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let waiting: Option<i64> = tx
            .query_row(
                "SELECT state FROM character_friends WHERE character_id = ?1 AND friend_id = ?2",
                rusqlite::params![i64::from(character_id), i64::from(other_id)],
                |row| row.get(0),
            )
            .ok();
        if waiting.map(FriendState::from_i64) != Some(FriendState::Pending) {
            return Ok(false);
        }
        if accept {
            for (a, b) in [(character_id, other_id), (other_id, character_id)] {
                tx.execute(
                    "UPDATE character_friends SET state = 2 WHERE character_id = ?1 AND friend_id = ?2",
                    rusqlite::params![i64::from(a), i64::from(b)],
                )?;
            }
        } else {
            for (a, b) in [(character_id, other_id), (other_id, character_id)] {
                tx.execute(
                    "DELETE FROM character_friends WHERE character_id = ?1 AND friend_id = ?2",
                    rusqlite::params![i64::from(a), i64::from(b)],
                )?;
            }
        }
        tx.commit()?;
        Ok(true)
    }

    /// Take a friend off both lists. `false` when they were not on it.
    pub fn remove_friend(&self, character_id: u32, other_id: u32) -> Result<bool> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let gone = tx.execute(
            "DELETE FROM character_friends WHERE character_id = ?1 AND friend_id = ?2",
            rusqlite::params![i64::from(character_id), i64::from(other_id)],
        )?;
        tx.execute(
            "DELETE FROM character_friends WHERE character_id = ?1 AND friend_id = ?2",
            rusqlite::params![i64::from(other_id), i64::from(character_id)],
        )?;
        tx.commit()?;
        Ok(gone > 0)
    }

    /// Move a friend into another group on this character's list only.
    pub fn set_friend_group(&self, character_id: u32, other_id: u32, group: &str) -> Result<bool> {
        let changed = self.conn().execute(
            "UPDATE character_friends SET group_name = ?3
              WHERE character_id = ?1 AND friend_id = ?2",
            rusqlite::params![i64::from(character_id), i64::from(other_id), group],
        )?;
        Ok(changed > 0)
    }
}

fn upsert(
    conn: &Connection,
    character_id: u32,
    friend_id: u32,
    group: &str,
    state: FriendState,
) -> Result<()> {
    conn.execute(
        "INSERT INTO character_friends (character_id, friend_id, group_name, state, asked_at)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(character_id, friend_id) DO UPDATE SET state = ?4",
        rusqlite::params![
            i64::from(character_id),
            i64::from(friend_id),
            group,
            state.as_i64(),
            Store::unix_now()
        ],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two() -> (Store, u32, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let mut ids = Vec::new();
        for name in ["Wisp", "Tester2"] {
            let chr = net::opcode::Character { name: name.to_string(), ..Default::default() };
            ids.push(store.create_character(account, 0, &chr).unwrap().id);
        }
        (store, ids[0], ids[1])
    }

    /// A request writes two directed rows: the asker waits, the asked owes an answer.
    #[test]
    fn a_request_leaves_one_side_waiting_and_the_other_owing_an_answer() {
        let (store, wisp, tester) = two();
        assert_eq!(
            store.request_friend(tester, wisp, "Default Group").unwrap(),
            FriendRequestOutcome::Asked
        );
        let theirs = store.friends(tester).unwrap();
        assert_eq!(theirs.len(), 1);
        assert_eq!((theirs[0].friend_id, theirs[0].state), (wisp, FriendState::Requested));
        assert_eq!(theirs[0].name, "Wisp", "the name comes from the characters table");
        let waiting = store.friend_requests_waiting(wisp).unwrap();
        assert_eq!(waiting.len(), 1);
        assert_eq!(waiting[0].name, "Tester2");

        // Every refusal refuses, and writes nothing.
        assert_eq!(
            store.request_friend(tester, wisp, "G").unwrap(),
            FriendRequestOutcome::AlreadyAsked
        );
        assert_eq!(store.request_friend(tester, tester, "G").unwrap(), FriendRequestOutcome::Yourself);
        assert_eq!(store.friends(tester).unwrap().len(), 1, "nothing was added");

        // The other side asking back IS the acceptance.
        assert_eq!(
            store.request_friend(wisp, tester, "Default Group").unwrap(),
            FriendRequestOutcome::AcceptedTheirs
        );
        for who in [wisp, tester] {
            let list = store.friends(who).unwrap();
            assert_eq!(list.len(), 1);
            assert_eq!(list[0].state, FriendState::Accepted);
        }
        assert_eq!(
            store.request_friend(wisp, tester, "G").unwrap(),
            FriendRequestOutcome::AlreadyFriends
        );
    }

    /// Accept settles both rows; refuse deletes both; answering nothing changes nothing.
    #[test]
    fn answering_a_request_moves_both_rows_or_neither() {
        let (store, wisp, tester) = two();
        assert!(!store.answer_friend_request(wisp, tester, true).unwrap(), "nothing to answer");

        store.request_friend(tester, wisp, "Default Group").unwrap();
        assert!(store.answer_friend_request(wisp, tester, true).unwrap());
        assert!(store.friends(wisp).unwrap()[0].state == FriendState::Accepted);
        assert!(store.friends(tester).unwrap()[0].state == FriendState::Accepted);
        // An accepted row is not a pending one: the same answer again does nothing.
        assert!(!store.answer_friend_request(wisp, tester, true).unwrap());

        // Refuse: both rows go.
        store.remove_friend(wisp, tester).unwrap();
        store.request_friend(tester, wisp, "Default Group").unwrap();
        assert!(store.answer_friend_request(wisp, tester, false).unwrap());
        assert!(store.friends(wisp).unwrap().is_empty());
        assert!(store.friends(tester).unwrap().is_empty(), "the asker's row goes too");

        // Groups are per side.
        store.request_friend(tester, wisp, "Guildies").unwrap();
        store.answer_friend_request(wisp, tester, true).unwrap();
        assert_eq!(store.friends(tester).unwrap()[0].group, "Guildies");
        assert_eq!(store.friends(wisp).unwrap()[0].group, "Default Group");
        assert!(store.set_friend_group(wisp, tester, "Testers").unwrap());
        assert_eq!(store.friends(wisp).unwrap()[0].group, "Testers");
        assert_eq!(store.friends(tester).unwrap()[0].group, "Guildies", "only one side moved");

        // And removing says whether it removed anything.
        assert!(store.remove_friend(wisp, tester).unwrap());
        assert!(!store.remove_friend(wisp, tester).unwrap());
    }
}
