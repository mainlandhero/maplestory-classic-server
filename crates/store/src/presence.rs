//! **Who is playing right now**, so the same account cannot be logged in twice.
//!
//! The owner, 2026-09-08: *"the server should not allow the same account to login twice, there
//! should be an existing message to say that the account is already logged in."* The client
//! has that message baked into its own WZ - `Login.img /Notice/text/loginAlready`, rendered
//! and read: **"That ID is already logged in. Please try again later"** - and login result
//! **7** is the code that selects it. So the whole of the server side is this file plus one
//! branch in `login::session`.
//!
//! # The failure this design exists to avoid is NOT the double login
//!
//! It is the lockout. **This client crashes, often, and mid-session** - that is the subject of
//! most of `research/`. A naive "account is logged in" flag is set on login and cleared on
//! logout, and a crash never reaches the logout. Every player who crashed would then be locked
//! out of their own account until something cleared the flag by hand, and with
//! [`crate::claims::LOGIN_CLAIM_TTL_SECS`] at twelve hours the obvious "clear it when the
//! claim expires" would mean **twelve hours**. That is far worse than the bug being fixed, and
//! it would be discovered by a player rather than by us.
//!
//! So presence is not a flag. It is a **lease**:
//!
//! * it is **held by a connection**, and released the moment that connection ends - including
//!   when it ends because the process died. A crashed client's socket is closed by the
//!   operating system, so the server's `read` returns `0` or `10054` within milliseconds and
//!   the serving thread runs the release on its way out. `world.log` records exactly that:
//!   `ended: An existing connection was forcibly closed by the remote host. (os error 10054)`;
//! * it **expires on its own** after [`PRESENCE_LEASE_SECS`] if nothing renews it, so even a
//!   server process that is killed outright cannot leave an account held forever;
//! * and it is keyed on **the client process, not the connection**, which is what makes the
//!   legitimate reconnects free. See below.
//!
//! # The holder is the CLIENT PROCESS, and that is the load-bearing choice
//!
//! One launch of this client makes **more than one connection**, and a lease keyed on the
//! socket would refuse the player's own reconnect as "already logged in":
//!
//! | event | what happens on the wire |
//! |---|---|
//! | Log Out, Choose another world | the login socket is dropped and a **new** one opened (`CLAUDE.md`: two `0x0010`s in one launch) |
//! | character select | the login socket closes and a **channel** connection opens |
//! | Change Channel | the channel socket closes and another channel's opens |
//!
//! Every one of those is the same `MapleStory.exe`. [`holder_key`] therefore builds the key
//! from the process the **operating system** attributes the accepted socket to
//! ([`crate::peerowner::owning_pid_of`]) - never from anything the connection asserts - so all
//! four rows above re-take *their own* lease and nothing is refused.
//!
//! Two clients started from one launcher sign-in are two processes, and that is exactly the
//! case the owner asked to refuse. It is also why the key is not the login claim's token: since
//! 2026-09-08 that token is deliberately reusable for the life of the claim
//! ([`crate::claims`]), so keying on it would make the two clients indistinguishable and the
//! feature would do nothing.
//!
//! **A connection that cannot be identified at all holds nothing.** `holder_key` returns
//! `None` when there is no pid and no address, and the caller must then skip enforcement and
//! say so in its log. An unidentifiable connection must never be able to lock an account out -
//! that is the same trade `login::server::resolve_account` makes when the claim lookup fails.
//!
//! # What "already logged in" is measured against, and what it is not
//!
//! It is measured against a **live lease**. It is not measured against a login claim (which
//! outlives any session by design), not against a session token, and not against the presence
//! of characters. `CLAUDE.md`'s standing constraint is untouched: nothing here authenticates
//! anything. A lease says "a client process the OS attributes to this socket is currently
//! being served as this account", which is a statement about sockets, not about people.

use rusqlite::{Connection, OptionalExtension};
use std::sync::Arc;

use crate::db::Store;
use crate::error::Result;

/// **How long a lease survives with nothing renewing it. Sixty seconds.**
///
/// This is the *worst case a crashed player waits*, and it is only reached when the release
/// could not run at all - the server process was killed, or the client died in the one gap
/// where no connection owns the lease (see [`PresenceGuard::hand_over`]). The ordinary crash
/// costs **zero** wait: the socket closes, the serving thread's guard drops, the row goes.
///
/// The number is bounded below by the renewal cadence of the two servers that hold leases:
/// the login server wakes a quiet connection every 4 s (`login::server::QUIET_AFTER`) and a
/// channel every 100 ms (`world::server::TICK_MS`). Sixty seconds is fifteen missed login
/// renewals, so a live session cannot expire by accident.
///
/// It is bounded above by the handover gap - login socket closes, channel connection arrives -
/// which was measured at **under a millisecond** (`crate::migration::MIGRATION_TTL_SECS`).
/// Sixty seconds is four orders of magnitude of slack on that, and it is the same order as the
/// migration's own TTL, which is the other thing that has to survive the same gap.
pub const PRESENCE_LEASE_SECS: i64 = 60;

/// A lease that is currently held, as read back from the database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PresenceHolder {
    pub account_id: i64,
    /// The opaque key from [`holder_key`]. Not a secret and not a credential - it is derived
    /// from the OS's view of the socket, so it is only ever compared, never trusted.
    pub holder: String,
    /// A sentence for the log saying where the holder is, e.g. `"login connection #3 from
    /// 127.0.0.1"`. Written for a person reading `login.log`, never parsed.
    pub whence: String,
    pub since: i64,
    pub last_seen: i64,
}

impl PresenceHolder {
    /// How long ago this lease was last renewed, in seconds.
    pub fn idle_secs(&self, now: i64) -> i64 {
        now.saturating_sub(self.last_seen)
    }

    /// The sentence a refusal should log. Says who holds it, how stale it is, and - the part
    /// that matters when a person is reading this at two in the morning - **how long the wait
    /// is**, because a refusal that does not bound the wait reads as a permanent lockout.
    pub fn why_refused(&self, now: i64) -> String {
        let idle = self.idle_secs(now);
        format!(
            "REFUSED - account id {} is ALREADY LOGGED IN by {} (holder {:?}, held for {} s, \
             last seen {idle} s ago). Answered with login result 7, which draws the client's \
             own loginAlready notice: \"That ID is already logged in. Please try again \
             later\". If that other client is gone, this clears by itself {} s from its last \
             packet - no manual step, and a client that CRASHES frees it immediately because \
             the operating system closes its socket",
            self.account_id,
            self.whence,
            self.holder,
            now.saturating_sub(self.since),
            PRESENCE_LEASE_SECS.saturating_sub(idle).max(0)
        )
    }
}

/// What [`Store::hold_presence`] did.
///
/// Deliberately not a `bool`. `CLAUDE.md`: *"A refusal that is reported to no one will be
/// ignored eventually."* The refusal carries the holder so the caller's log line can say who
/// has it and when it frees, and so the caller cannot accidentally treat a refusal as a
/// success by forgetting a `!`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PresenceOutcome {
    /// The lease is now this holder's. Either it was free, or it was already ours, or the
    /// previous holder's lease had gone stale and was reclaimed.
    Held {
        /// True when a stale lease was taken over rather than a free row claimed. Worth
        /// logging: it is the only visible sign that a previous session died without
        /// releasing, and it is the number to watch if anybody ever reports a wait.
        reclaimed_stale: bool,
    },
    /// Somebody else's live lease. **Refuse**, with login result 7.
    TakenBy(PresenceHolder),
}

impl PresenceOutcome {
    pub fn is_held(&self) -> bool {
        matches!(self, PresenceOutcome::Held { .. })
    }
}

/// **The key a lease is held under: the client process, as the operating system sees it.**
///
/// `pid` must come from [`crate::peerowner::owning_pid_of`] on a socket this server accepted -
/// never from a packet body. `peer_ip` is the fallback for an off-box peer, which has no row
/// in this machine's TCP table.
///
/// `None` means *this connection cannot be identified at all*, and the caller must then not
/// enforce. See the module docs: an unidentifiable connection that could hold a lease would be
/// a way to lock somebody out of their account by connecting.
///
/// # Why the address is an acceptable fallback and is still not a discriminator
///
/// The owner, 2026-08-29: *"IP cannot be the sole discriminator."* It is not one here. A lease is
/// **per account**, so two off-box clients sharing an address only collide when they are
/// signed in as the *same account* - which is precisely the state this whole file refuses.
/// Two different accounts behind one NAT hold two different rows and never meet.
pub fn holder_key(pid: Option<u32>, peer_ip: Option<&str>) -> Option<String> {
    match (pid, peer_ip) {
        (Some(pid), _) => Some(format!("pid:{pid}")),
        (None, Some(ip)) if !ip.is_empty() => Some(format!("addr:{ip}")),
        _ => None,
    }
}

/// Create the presence table. Called from `Store::init` on every open, so it must be
/// idempotent - and from every entry point below, for the reason `crate::claims::create_tables`
/// gives: a missing table on the login path turns into an `Err`, and a login server that
/// answers an error instead of a reply freezes the client's entire UI.
pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- ONE ROW PER ACCOUNT. `account_id` is the PRIMARY KEY, so "one login per account"
        -- is a property of the schema rather than of a code path somebody can forget to run.
        --
        -- There is no `logged_in` flag anywhere, deliberately. A flag is set on login and
        -- cleared on logout, and this client crashes - so the clear would not run and the
        -- player would be locked out of their own account. `last_seen` is a LEASE: it is
        -- refreshed by the live connection and the row is dead once it is
        -- PRESENCE_LEASE_SECS old, whatever happened to the process that wrote it.
        CREATE TABLE IF NOT EXISTS account_presence (
            account_id INTEGER PRIMARY KEY REFERENCES accounts(id) ON DELETE CASCADE,
            holder     TEXT    NOT NULL,
            whence     TEXT    NOT NULL,
            since      INTEGER NOT NULL,
            last_seen  INTEGER NOT NULL
        );

        -- The liveness predicate, for the sweep that prunes dead rows.
        CREATE INDEX IF NOT EXISTS idx_account_presence_seen ON account_presence(last_seen);
        "#,
    )?;
    Ok(())
}

impl Store {
    /// **Take the lease for an account, or find out who has it.**
    ///
    /// One statement pair in one transaction, so two connections racing the same free account
    /// cannot both be told they got it: the `INSERT` names the primary key, and the
    /// `ON CONFLICT` update carries a predicate that only the rightful taker satisfies.
    ///
    /// The lease is taken when the row is free, when it is **already ours** (the reconnect
    /// case - see the module docs), or when the previous holder's lease has gone stale. It is
    /// refused when somebody else's lease is live.
    ///
    /// `whence` is a sentence for the log. It is rewritten on every take, including a renewal
    /// by the same holder, so a reconnect's line names the connection that actually holds it.
    pub fn hold_presence(
        &self,
        account_id: i64,
        holder: &str,
        whence: &str,
    ) -> Result<PresenceOutcome> {
        let now = Store::now();
        let dead_by = now.saturating_sub(PRESENCE_LEASE_SECS);
        let mut conn = self.conn();
        create_tables(&conn)?;
        let tx = conn.transaction()?;

        // What was there BEFORE, so the answer can say whether a stale lease was reclaimed.
        // Read inside the transaction: reading it outside would be a different question.
        let before: Option<(String, i64)> = tx
            .query_row(
                "SELECT holder, last_seen FROM account_presence WHERE account_id = ?1",
                rusqlite::params![account_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;

        let changed = tx.execute(
            "INSERT INTO account_presence (account_id, holder, whence, since, last_seen)
                  VALUES (?1, ?2, ?3, ?4, ?4)
             ON CONFLICT(account_id) DO UPDATE SET
                 holder    = excluded.holder,
                 whence    = excluded.whence,
                 -- `since` is kept when the holder has not changed, so a reconnect does not
                 -- reset how long this player has been on. It is reset when a stale lease is
                 -- taken over, because that is a different session.
                 since     = CASE WHEN account_presence.holder = excluded.holder
                                  THEN account_presence.since ELSE excluded.since END,
                 last_seen = excluded.last_seen
               WHERE account_presence.holder = excluded.holder
                  OR account_presence.last_seen <= ?5",
            rusqlite::params![account_id, holder, whence, now, dead_by],
        )?;

        if changed == 1 {
            let reclaimed_stale = matches!(&before, Some((h, _)) if h != holder);
            tx.commit()?;
            return Ok(PresenceOutcome::Held { reclaimed_stale });
        }

        // The `WHERE` refused the update, so a live lease belonging to somebody else is there.
        // Read it back inside the same transaction rather than reusing `before`: the row that
        // refused us is the row to report, and re-reading is what makes that exact.
        let held = tx
            .query_row(
                "SELECT holder, whence, since, last_seen FROM account_presence
                  WHERE account_id = ?1",
                rusqlite::params![account_id],
                |row| {
                    Ok(PresenceHolder {
                        account_id,
                        holder: row.get(0)?,
                        whence: row.get(1)?,
                        since: row.get(2)?,
                        last_seen: row.get(3)?,
                    })
                },
            )
            .optional()?;
        tx.commit()?;

        Ok(match held {
            Some(h) => PresenceOutcome::TakenBy(h),
            // The row vanished between the failed update and the read - another connection
            // released it. Nothing holds the account, and reporting a refusal we cannot name
            // would be a guess. The caller retries on its next request; a login request is
            // never the only one a client sends.
            None => PresenceOutcome::Held { reclaimed_stale: true },
        })
    }

    /// **Keep a lease alive.** `false` means it is no longer ours - it expired and somebody
    /// else took it - which a caller must report rather than ignore.
    pub fn renew_presence(&self, account_id: i64, holder: &str) -> Result<bool> {
        let now = Store::now();
        let conn = self.conn();
        create_tables(&conn)?;
        Ok(conn.execute(
            "UPDATE account_presence SET last_seen = ?3
              WHERE account_id = ?1 AND holder = ?2",
            rusqlite::params![account_id, holder, now],
        )? == 1)
    }

    /// **Give a lease back.** Scoped to the holder, so a connection can never release
    /// somebody else's lease - which is what would happen if this deleted by account alone and
    /// a stale thread got round to its cleanup after a new session had started.
    ///
    /// `false` is not an error: it means the lease had already expired and been taken, or was
    /// never ours.
    pub fn release_presence(&self, account_id: i64, holder: &str) -> Result<bool> {
        let conn = self.conn();
        create_tables(&conn)?;
        Ok(conn.execute(
            "DELETE FROM account_presence WHERE account_id = ?1 AND holder = ?2",
            rusqlite::params![account_id, holder],
        )? == 1)
    }

    /// The LIVE lease on an account, or `None`. A row past [`PRESENCE_LEASE_SECS`] is not
    /// live and is reported as `None` - the filter is here rather than at the caller for the
    /// same reason `claims::live_rows` puts its expiry in the query.
    pub fn presence_of(&self, account_id: i64) -> Result<Option<PresenceHolder>> {
        let dead_by = Store::now().saturating_sub(PRESENCE_LEASE_SECS);
        let conn = self.conn();
        create_tables(&conn)?;
        Ok(conn
            .query_row(
                "SELECT holder, whence, since, last_seen FROM account_presence
                  WHERE account_id = ?1 AND last_seen > ?2",
                rusqlite::params![account_id, dead_by],
                |row| {
                    Ok(PresenceHolder {
                        account_id,
                        holder: row.get(0)?,
                        whence: row.get(1)?,
                        since: row.get(2)?,
                        last_seen: row.get(3)?,
                    })
                },
            )
            .optional()?)
    }

    /// Drop every lease. What an operator would call after a server restart, and what the
    /// tests use to state "nobody is playing" without waiting a minute.
    pub fn clear_presence(&self) -> Result<usize> {
        let conn = self.conn();
        create_tables(&conn)?;
        Ok(conn.execute("DELETE FROM account_presence", [])?)
    }
}

/// **A held lease, released when this is dropped.**
///
/// The whole point is the `Drop`: `login::server::connection` and `world::server::connection`
/// each have half a dozen `return` paths - a read error, a framing error, a clean close, a
/// write failure - and a release written at any one of them is a release missed at the other
/// five. The one that matters most is the error path, because that is the crash:
/// `10054`, the client process gone, `read` returning immediately.
///
/// It is not the only thing that frees a lease. [`PRESENCE_LEASE_SECS`] is the backstop for
/// the case a `Drop` cannot cover - the server process itself being killed - and the two
/// together are why a crashed player never waits more than a minute and usually waits nothing.
pub struct PresenceGuard {
    store: Arc<Store>,
    account_id: i64,
    holder: String,
    handed_over: bool,
}

impl PresenceGuard {
    /// Take the lease, or report who has it.
    ///
    /// `Err` is a database failure and is **not** a refusal: the caller must carry on serving.
    /// Refusing on a failed read would let a broken table lock every account out, which is the
    /// lockout this design exists to avoid arriving by another door.
    pub fn hold(
        store: Arc<Store>,
        account_id: i64,
        holder: &str,
        whence: &str,
    ) -> Result<std::result::Result<PresenceGuard, PresenceHolder>> {
        Ok(match store.hold_presence(account_id, holder, whence)? {
            PresenceOutcome::Held { .. } => Ok(PresenceGuard {
                store,
                account_id,
                holder: holder.to_string(),
                handed_over: false,
            }),
            PresenceOutcome::TakenBy(who) => Err(who),
        })
    }

    pub fn account_id(&self) -> i64 {
        self.account_id
    }

    pub fn holder(&self) -> &str {
        &self.holder
    }

    /// Keep it alive. `false` means it was lost - report it.
    pub fn renew(&self) -> Result<bool> {
        self.store.renew_presence(self.account_id, &self.holder)
    }

    /// **Stop releasing on drop, because another connection is about to take this lease over.**
    ///
    /// Character select is the case: the login socket closes and the channel connection opens
    /// a moment later, both from the same client process. Releasing in between would open a
    /// window in which a second client could log in as this account, which is the whole thing
    /// being prevented - so the login connection hands the lease across instead, and
    /// [`PRESENCE_LEASE_SECS`] is what closes it if the channel connection never arrives.
    ///
    /// The channel takes it with the **same holder key** (the same client process), so the
    /// takeover is a renewal rather than a fight.
    pub fn hand_over(&mut self) {
        self.handed_over = true;
    }

    /// True once [`Self::hand_over`] has been called. For the tests and for a log line.
    pub fn is_handed_over(&self) -> bool {
        self.handed_over
    }
}

impl Drop for PresenceGuard {
    fn drop(&mut self) {
        if self.handed_over {
            return;
        }
        // A failure here cannot be reported - `Drop` has nowhere to put it - and it does not
        // have to be: the lease expires on its own. That backstop is why this is allowed to
        // swallow the error, and it is the reason PRESENCE_LEASE_SECS is not optional.
        let _ = self.store.release_presence(self.account_id, &self.holder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store_with_two_accounts() -> (Arc<Store>, i64, i64) {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let a = store.create_account("player_one", "correct horse battery").unwrap();
        let b = store.create_account("player_two", "correct horse battery").unwrap();
        (store, a, b)
    }

    /// Age a lease by rewriting `last_seen`, so "a minute passed" costs no minute.
    fn age(store: &Store, account_id: i64, secs: i64) {
        store
            .conn()
            .execute(
                "UPDATE account_presence SET last_seen = last_seen - ?2, since = since - ?2
                  WHERE account_id = ?1",
                rusqlite::params![account_id, secs],
            )
            .unwrap();
    }

    /// **The positive control.** A fresh store answers "nobody is playing" rather than
    /// failing, which proves the table is created on the read path. Every test below asserts
    /// `Some(..)` or a refusal, and neither could be told from a broken query without this.
    #[test]
    fn a_store_with_no_presence_answers_none_rather_than_failing() {
        let (store, a, _) = store_with_two_accounts();
        assert_eq!(store.presence_of(a).unwrap(), None);
        assert_eq!(store.clear_presence().unwrap(), 0);
        assert!(!store.renew_presence(a, "pid:1").unwrap());
        assert!(!store.release_presence(a, "pid:1").unwrap());
    }

    #[test]
    fn holding_an_account_makes_it_readable_as_live() {
        let (store, a, _) = store_with_two_accounts();
        assert!(store.hold_presence(a, "pid:100", "login #1").unwrap().is_held());
        let held = store.presence_of(a).unwrap().expect("the lease is live");
        assert_eq!(held.holder, "pid:100");
        assert_eq!(held.whence, "login #1");
        assert_eq!(held.account_id, a);
    }

    /// **The feature.** A second client process is refused, and the refusal names the holder.
    #[test]
    fn a_second_process_on_the_same_account_is_refused_and_the_first_is_untouched() {
        let (store, a, _) = store_with_two_accounts();
        assert!(store.hold_presence(a, "pid:100", "login #1 from 127.0.0.1").unwrap().is_held());

        let second = store.hold_presence(a, "pid:200", "login #2 from 127.0.0.1").unwrap();
        let PresenceOutcome::TakenBy(who) = second else {
            panic!("a second client process must be refused: {second:?}");
        };
        assert_eq!(who.holder, "pid:100");
        assert!(who.whence.contains("login #1"), "{}", who.whence);

        // AND THE FIRST SESSION IS UNAFFECTED - the effect that a test checking only the
        // refusal would miss. `CLAUDE.md`: a test that checks one of several effects gives
        // false confidence about the rest.
        assert!(store.renew_presence(a, "pid:100").unwrap(), "the first lease must survive");
        assert_eq!(store.presence_of(a).unwrap().unwrap().holder, "pid:100");
    }

    /// Two DIFFERENT accounts never collide, however many clients are on one machine.
    #[test]
    fn two_accounts_are_independent() {
        let (store, a, b) = store_with_two_accounts();
        assert!(store.hold_presence(a, "pid:100", "one").unwrap().is_held());
        assert!(store.hold_presence(b, "pid:200", "two").unwrap().is_held());
        assert!(store.presence_of(a).unwrap().is_some());
        assert!(store.presence_of(b).unwrap().is_some());
    }

    /// **The reconnect, and the reason the holder is the process rather than the socket.**
    ///
    /// "Log Out", "Choose another world", character select and Change Channel all drop a
    /// socket and open another from the same client process. Every one of them must re-take
    /// its own lease rather than meet a refusal.
    #[test]
    fn the_same_process_reconnecting_re_takes_its_own_lease() {
        let (store, a, _) = store_with_two_accounts();
        assert!(store.hold_presence(a, "pid:100", "login #1").unwrap().is_held());
        let since = store.presence_of(a).unwrap().unwrap().since;

        for whence in ["login #2 after Log Out", "channel 0 #1", "channel 1 #1"] {
            let again = store.hold_presence(a, "pid:100", whence).unwrap();
            assert_eq!(
                again,
                PresenceOutcome::Held { reclaimed_stale: false },
                "the player's own reconnect must not be refused ({whence})"
            );
            assert_eq!(store.presence_of(a).unwrap().unwrap().whence, whence);
        }
        // And it is still one session, not four.
        assert_eq!(store.presence_of(a).unwrap().unwrap().since, since);
    }

    /// **THE LOCKOUT REGRESSION.** A lease nothing renewed is dead, and the account is free.
    /// This is the property that stops a crash from locking a player out for as long as the
    /// login claim lives.
    #[test]
    fn a_stale_lease_is_not_live_and_is_reclaimed_by_the_next_client() {
        let (store, a, _) = store_with_two_accounts();
        assert!(store.hold_presence(a, "pid:100", "the client that crashed").unwrap().is_held());
        age(&store, a, PRESENCE_LEASE_SECS + 1);

        assert_eq!(store.presence_of(a).unwrap(), None, "a stale lease is not a live one");
        assert_eq!(
            store.hold_presence(a, "pid:200", "the client after the crash").unwrap(),
            PresenceOutcome::Held { reclaimed_stale: true },
            "a stale lease must not refuse the next login"
        );
        assert_eq!(store.presence_of(a).unwrap().unwrap().holder, "pid:200");
    }

    /// A lease that is one second short of stale still refuses. The boundary in the other
    /// direction, because a test that only proves expiry would pass against a version with no
    /// lease at all.
    #[test]
    fn a_lease_just_inside_the_window_still_refuses() {
        let (store, a, _) = store_with_two_accounts();
        assert!(store.hold_presence(a, "pid:100", "still playing").unwrap().is_held());
        age(&store, a, PRESENCE_LEASE_SECS - 1);
        assert!(store.presence_of(a).unwrap().is_some());
        assert!(
            !store.hold_presence(a, "pid:200", "a second client").unwrap().is_held(),
            "a live lease must still refuse"
        );
    }

    /// Renewing is what keeps a long session alive past the lease.
    #[test]
    fn renewing_keeps_a_lease_alive_past_its_window() {
        let (store, a, _) = store_with_two_accounts();
        assert!(store.hold_presence(a, "pid:100", "playing").unwrap().is_held());
        age(&store, a, PRESENCE_LEASE_SECS - 1);
        assert!(store.renew_presence(a, "pid:100").unwrap());
        // The renewal moved last_seen to now, so ageing by the same amount again is still live.
        age(&store, a, PRESENCE_LEASE_SECS - 1);
        assert!(store.presence_of(a).unwrap().is_some());
        // A renewal by somebody else's holder key changes nothing and says so.
        assert!(!store.renew_presence(a, "pid:999").unwrap());
    }

    /// A release is scoped to the holder: a late thread cannot free a lease that has since
    /// been taken by a new session.
    #[test]
    fn a_release_cannot_free_somebody_elses_lease() {
        let (store, a, _) = store_with_two_accounts();
        assert!(store.hold_presence(a, "pid:100", "first").unwrap().is_held());
        age(&store, a, PRESENCE_LEASE_SECS + 1);
        assert!(store.hold_presence(a, "pid:200", "second").unwrap().is_held());

        assert!(!store.release_presence(a, "pid:100").unwrap(), "the old holder frees nothing");
        assert_eq!(store.presence_of(a).unwrap().unwrap().holder, "pid:200");
        assert!(store.release_presence(a, "pid:200").unwrap());
        assert_eq!(store.presence_of(a).unwrap(), None);
    }

    // ------------------------------------------------------------------------ the guard

    /// **The crash, as a test.** A guard that goes out of scope - which is what a serving
    /// thread does when `read` returns `10054` - frees the account immediately.
    #[test]
    fn dropping_a_guard_frees_the_account_at_once() {
        let (store, a, _) = store_with_two_accounts();
        {
            let guard = PresenceGuard::hold(store.clone(), a, "pid:100", "the doomed client")
                .unwrap()
                .expect("a free account");
            assert_eq!(guard.account_id(), a);
            assert!(store.presence_of(a).unwrap().is_some());
        }
        assert_eq!(
            store.presence_of(a).unwrap(),
            None,
            "a connection that died must not hold the account"
        );
        // And the next login is not refused - the whole point.
        assert!(PresenceGuard::hold(store.clone(), a, "pid:200", "the retry").unwrap().is_ok());
    }

    /// A guard that was handed over does NOT release, because the next connection is taking
    /// it. Character select is this case.
    #[test]
    fn a_handed_over_guard_leaves_the_lease_for_the_next_connection() {
        let (store, a, _) = store_with_two_accounts();
        {
            let mut guard = PresenceGuard::hold(store.clone(), a, "pid:100", "login, migrating")
                .unwrap()
                .expect("a free account");
            guard.hand_over();
            assert!(guard.is_handed_over());
        }
        let held = store.presence_of(a).unwrap().expect("the lease survives the handover");
        assert_eq!(held.holder, "pid:100");
        // The channel connection takes it with the SAME key, so it is a renewal.
        let channel = PresenceGuard::hold(store.clone(), a, "pid:100", "channel 0")
            .unwrap()
            .expect("the same process takes its own lease");
        assert!(channel.renew().unwrap());
    }

    /// **And the handover is still bounded.** If the channel connection never arrives - the
    /// client died in the gap - the lease expires rather than holding the account for the
    /// life of the login claim.
    #[test]
    fn a_handover_that_is_never_collected_expires() {
        let (store, a, _) = store_with_two_accounts();
        {
            let mut guard =
                PresenceGuard::hold(store.clone(), a, "pid:100", "login").unwrap().unwrap();
            guard.hand_over();
        }
        age(&store, a, PRESENCE_LEASE_SECS + 1);
        assert_eq!(store.presence_of(a).unwrap(), None);
        assert!(PresenceGuard::hold(store.clone(), a, "pid:200", "next").unwrap().is_ok());
    }

    /// A guard that could not take the lease is an `Err` carrying the holder, and taking one
    /// must not have been a side effect.
    #[test]
    fn a_refused_guard_carries_the_holder_and_takes_nothing() {
        let (store, a, _) = store_with_two_accounts();
        let _first =
            PresenceGuard::hold(store.clone(), a, "pid:100", "first").unwrap().unwrap();
        let refused = PresenceGuard::hold(store.clone(), a, "pid:200", "second").unwrap();
        let Err(who) = refused else { panic!("the second client must be refused") };
        assert_eq!(who.holder, "pid:100");
        assert_eq!(store.presence_of(a).unwrap().unwrap().holder, "pid:100");
    }

    // ------------------------------------------------------------------------ holder_key

    /// The process wins when the OS could attribute the socket; the address is the off-box
    /// fallback; nothing at all means **do not enforce**.
    #[test]
    fn the_holder_key_prefers_the_process_and_refuses_to_invent_one() {
        assert_eq!(holder_key(Some(4242), Some("127.0.0.1")).as_deref(), Some("pid:4242"));
        assert_eq!(holder_key(Some(4242), None).as_deref(), Some("pid:4242"));
        assert_eq!(holder_key(None, Some("10.0.0.7")).as_deref(), Some("addr:10.0.0.7"));
        // An unidentifiable connection holds NOTHING. If this ever returns `Some`, a
        // connection nobody can name could lock an account out by arriving.
        assert_eq!(holder_key(None, None), None);
        assert_eq!(holder_key(None, Some("")), None);
        // Two processes on one address are two keys - which is the case the owner asked to refuse.
        assert_ne!(holder_key(Some(1), Some("127.0.0.1")), holder_key(Some(2), Some("127.0.0.1")));
    }

    /// The refusal sentence has to bound the wait. A refusal that reads as permanent is how a
    /// twelve-hour lockout gets reported, and the number is the whole reassurance.
    #[test]
    fn the_refusal_sentence_says_how_long_the_wait_is() {
        let now = 1_000_000;
        let who = PresenceHolder {
            account_id: 3,
            holder: "pid:100".into(),
            whence: "login connection #1 from 127.0.0.1".into(),
            since: now - 300,
            last_seen: now - 10,
        };
        let why = who.why_refused(now);
        assert!(why.contains("ALREADY LOGGED IN"), "{why}");
        assert!(why.contains("login result 7"), "{why}");
        assert!(why.contains("loginAlready"), "{why}");
        assert!(why.contains(&format!("{} s from its last packet", PRESENCE_LEASE_SECS - 10)), "{why}");
        assert!(why.contains("no manual step"), "{why}");
        assert_eq!(who.idle_secs(now), 10);
    }

    /// Deleting the account takes its lease with it, through the foreign key. Otherwise a
    /// re-created account id could inherit a lease nobody can release.
    #[test]
    fn deleting_an_account_takes_its_lease() {
        let (store, a, _) = store_with_two_accounts();
        assert!(store.hold_presence(a, "pid:100", "playing").unwrap().is_held());
        store.conn().execute("DELETE FROM accounts WHERE id = ?1", rusqlite::params![a]).unwrap();
        assert_eq!(store.presence_of(a).unwrap(), None);
    }
}
