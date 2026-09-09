//! SQLite storage for accounts and sessions.

use std::sync::Mutex;

use rusqlite::{params, Connection, OptionalExtension};

use crate::error::{Result, StoreError};
use crate::password::{hash_password, verify_password};
use crate::session::{hash_token, new_token, NewSession};

/// How long an issued session stays valid.
pub const SESSION_TTL_SECS: i64 = 15 * 60;

/// One column per inventory, in the order the client reads them.
///
/// Six, not five: the record's sizing loop runs a fixed six turns. What each index is - and
/// which of the names are measured and which are inferred - is
/// [`net::opcode::INVENTORY_SLOT_ORDER`]. The sixth has no known name, so it is called what
/// it is.
///
/// Per-column rather than one number for the whole bag because buying slots is per-tab in
/// this game, and a single column could not express a character who has bought Use slots
/// and not Etc ones.
pub const INVENTORY_SLOT_COLUMNS: [&str; net::opcode::INVENTORY_COUNT] = [
    "slots_equip",
    "slots_use",
    "slots_setup",
    "slots_etc",
    "slots_cash",
    "slots_deco",
];

/// The name `slots_deco` had for one commit, before the sixth inventory had a name.
///
/// Kept so [`Store::add_inventory_slot_columns`] can rename it instead of adding a seventh
/// column beside it and silently losing whatever was in the old one.
const INVENTORY_SLOT_COLUMN_RENAMED_FROM: (&str, &str) = ("slots_sixth", "slots_deco");

/// What the slot columns were created with before the count was measured on screen.
///
/// See the repair in [`Store::add_inventory_slot_columns`].
const SUPERSEDED_INVENTORY_SLOTS: u16 = 24;

/// The id the first character gets. See the note beside the `sqlite_sequence` seed.
pub const FIRST_CHARACTER_ID: u32 = 200;

#[derive(Debug, Clone)]
pub struct Account {
    pub id: i64,
    pub name: String,
    /// The address the launcher's sign-in field accepts, when one is set.
    ///
    /// `None` for every account made before the column existed, and for any account made
    /// with `maplecw-useradd` without one. An account with no email signs in by name; the
    /// two are alternative identities for the same row, never a second credential.
    pub email: Option<String>,
    pub enabled: bool,
    /// May this account use the `!` GM commands?
    ///
    /// **Not a security boundary.** The game socket carries no credentials, so this says which
    /// *account* is allowed, not which *person* is connected - and which account a connection
    /// is served as comes from a launcher claim. It keeps a second account on the machine out
    /// of `!item`; it keeps nothing out of the port.
    pub is_gm: bool,
    pub created_at: i64,
    pub last_login: Option<i64>,
}

/// How many characters of the local part survive masking. `wispplayer@example.com` becomes
/// `wisp****@example.com`, which is the shape the real service shows and the shape
/// `tools/test-server.ps1` has been hard-coding as `-DisplayName` since before there was an
/// email column to build it from.
const MASK_KEEP: usize = 4;

/// The stars. A **fixed** count, deliberately: one star per hidden character would leak the
/// length of the address, which is the one thing masking is for.
const MASK_STARS: &str = "****";

/// Mask an email for the login screen.
///
/// ```text
/// wispplayer@example.com -> wisp****@example.com
/// abc@example.com        -> abc****@example.com     (shorter than the keep length)
/// not-an-email           -> not-****                (no @: mask anyway rather than show it)
/// ```
///
/// The domain is kept whole. That is what the address is recognisable by, and the point of the
/// field is for a person to see their own account rather than for it to be a secret.
pub fn mask_email(email: &str) -> String {
    let (local, domain) = match email.split_once('@') {
        Some((l, d)) => (l, Some(d)),
        // Not an address. Mask the front of it anyway - showing an unrecognised value in full
        // on the login screen is the one behaviour that has no argument for it.
        None => (email, None),
    };
    let kept: String = local.chars().take(MASK_KEEP).collect();
    match domain {
        Some(d) => format!("{kept}{MASK_STARS}@{d}"),
        None => format!("{kept}{MASK_STARS}"),
    }
}

impl Account {
    /// What the client's login screen should display for this account.
    ///
    /// `None` when the account has no email, which is every account made before the column
    /// existed. The caller decides what to show instead - the login server falls back to its
    /// `--display-name`, because leaving the field empty makes the client draw a blank line
    /// where a person expects to see themselves.
    pub fn masked_email(&self) -> Option<String> {
        self.email.as_deref().map(mask_email)
    }
}

/// Outcome of an authentication attempt.
///
/// Deliberately does **not** distinguish "no such account" from "wrong password":
/// telling them apart lets an attacker enumerate valid account names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthOutcome {
    Ok { account_id: i64, token: String },
    InvalidCredentials,
    Disabled,
}

/// The SQLite connection is wrapped in a `Mutex` so `Store` is `Send + Sync` and can
/// be shared across threads via `Arc`. `rusqlite::Connection` is `Send` but not `Sync`,
/// and the servers built on this are concurrent, so serialising access here avoids
/// every caller having to invent its own locking.
pub struct Store {
    conn: Mutex<Connection>,
}

impl Store {
    /// Open (creating if needed) a database file.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let conn = Connection::open(path)?;
        Self::init(conn)
    }

    /// In-memory database, for tests.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        Self::init(conn)
    }

    fn init(conn: Connection) -> Result<Self> {
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "foreign_keys", "ON")?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS accounts (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                name          TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                -- argon2id PHC string; never a plain-text or reversible password
                password_hash TEXT    NOT NULL,
                enabled       INTEGER NOT NULL DEFAULT 1,
                created_at    INTEGER NOT NULL,
                last_login    INTEGER
            );

            CREATE TABLE IF NOT EXISTS sessions (
                -- SHA-256 of the token; the token itself is never stored
                token_hash TEXT    PRIMARY KEY,
                account_id INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
                issued_at  INTEGER NOT NULL,
                expires_at INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_account ON sessions(account_id);
            CREATE INDEX IF NOT EXISTS idx_sessions_expiry  ON sessions(expires_at);

            -- One row per character. The column set is deliberately the protocol's
            -- Character field for field, so crates/store/src/character.rs can map a row
            -- to it by exhaustive destructure and a new protocol field becomes a
            -- compile error rather than a stat that silently fails to persist.
            --
            -- name is unique across the WHOLE service, not per account: the client's
            -- name check carries no account to scope by.
            CREATE TABLE IF NOT EXISTS characters (
                id           INTEGER PRIMARY KEY AUTOINCREMENT,
                account_id   INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
                world_id     INTEGER NOT NULL,
                name         TEXT    NOT NULL UNIQUE COLLATE NOCASE,
                gender       INTEGER NOT NULL,
                skin         INTEGER NOT NULL,
                face         INTEGER NOT NULL,
                hair         INTEGER NOT NULL,
                level        INTEGER NOT NULL,
                job          INTEGER NOT NULL,
                strength     INTEGER NOT NULL,
                dexterity    INTEGER NOT NULL,
                intelligence INTEGER NOT NULL,
                luck         INTEGER NOT NULL,
                hp           INTEGER NOT NULL,
                max_hp       INTEGER NOT NULL,
                mp           INTEGER NOT NULL,
                max_mp       INTEGER NOT NULL,
                ap           INTEGER NOT NULL,
                map_id       INTEGER NOT NULL,
                created_at   INTEGER NOT NULL
            );

            -- The avatar's visible equipment, one row per occupied slot. Separate from
            -- characters because the slot set is sparse and grows with the game.
            CREATE TABLE IF NOT EXISTS equipment (
                character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
                slot         INTEGER NOT NULL,
                item_id      INTEGER NOT NULL,
                PRIMARY KEY (character_id, slot)
            );

            -- A character moving from the login/character-select server to a channel.
            --
            -- This table exists because the two are separate processes: the login server
            -- mints the handoff and the channel server is the one that has to believe it,
            -- and they share nothing but the database. A migration is **single use** -
            -- consumed_at is set on the first successful claim - so a replayed handoff
            -- cannot put a second connection into the world as the same character.
            --
            -- The seed is what actually travels in the migration packet, and it is only a
            -- u32, so it is far too small to be a secret on its own. It identifies a
            -- pending migration; it does not authenticate one. Say so when reporting.
            CREATE TABLE IF NOT EXISTS migrations (
                seed         INTEGER NOT NULL,
                account_id   INTEGER NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
                character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
                world_id     INTEGER NOT NULL,
                channel_id   INTEGER NOT NULL,
                created_at   INTEGER NOT NULL,
                consumed_at  INTEGER,
                PRIMARY KEY (seed)
            );

            -- Characters created before the start map was set carry map_id 0, and 0 is
            -- not a map: the game's own String.wz table has no entry for it and the lowest
            -- real id is 1. So a stored 0 means "never assigned" rather than a place, and
            -- repairing it is fixing a placeholder rather than overwriting a decision.
            --
            -- Idempotent by construction, and it cannot touch a character that is anywhere
            -- real. If a legitimate map 0 ever exists this has to go.
            UPDATE characters SET map_id = 1 WHERE map_id = 0;

            CREATE INDEX IF NOT EXISTS idx_characters_account
                ON characters(account_id, world_id);

            -- Character ids start at FIRST_CHARACTER_ID rather than 1.
            --
            -- Not cosmetic, and not superstition: on 2026-08-18 a create reply carrying
            -- id 1 was byte-identical to a reply that had previously transitioned the
            -- client to character select EXCEPT for the two copies of this id, and the
            -- client did not transition. Small ids are the untested value; the working
            -- one was 200. If a measurement later shows the id was innocent this seed
            -- can go, but a real service does not number characters from 1 anyway.
            --
            -- sqlite_sequence already exists because `accounts` is AUTOINCREMENT. It has
            -- no UNIQUE constraint on `name`, so INSERT OR IGNORE would append a SECOND
            -- 'characters' row rather than skip - hence the guarded insert and the
            -- separate raise, which are together idempotent and safe on a database that
            -- already has characters in it.
            INSERT INTO sqlite_sequence (name, seq)
                 SELECT 'characters', 199
                  WHERE NOT EXISTS (SELECT 1 FROM sqlite_sequence WHERE name = 'characters');
            UPDATE sqlite_sequence SET seq = 199 WHERE name = 'characters' AND seq < 199;
            "#,
        )?;
        Self::add_inventory_slot_columns(&conn)?;
        Self::add_meso_column(&conn)?;
        Self::add_experience_column(&conn)?;
        Self::add_account_email_column(&conn)?;
        Self::add_account_gm_column(&conn)?;
        // Whole new tables, so a plain `CREATE TABLE IF NOT EXISTS` is enough - unlike the
        // slot columns above, which had to be ALTERed onto a table that already existed.
        crate::quest::create_tables(&conn)?;
        // `inventory::create_tables` also ALTERs the stat columns onto `equipment`, which is
        // NOT a new table; see the note there.
        crate::inventory::create_tables(&conn)?;
        crate::storage::create_tables(&conn)?;
        crate::cash::create_tables(&conn)?;
        crate::skills::create_tables(&conn)?;
        // The spent half of a skill point. A whole new table, so `CREATE TABLE IF NOT EXISTS`
        // is enough - see `skillpoints::create_tables` for what that means for a character who
        // already has skills learned.
        crate::skillpoints::create_tables(&conn)?;
        crate::rates::create_tables(&conn)?;
        // Which account a credential-less game connection is served as. New table, so
        // `CREATE TABLE IF NOT EXISTS` is enough. `claims.rs` ALSO ensures the table inside
        // each of its three methods, and that belt-and-braces is deliberate rather than
        // redundant: `current_login_claim` runs once per login connection, and a missing
        // table would make it return `Err` - which the login server would have to turn into
        // either a wrong account or no reply at all, and an unanswered packet freezes the
        // client's whole UI.
        crate::claims::create_tables(&conn)?;
        // WHO IS PLAYING RIGHT NOW - the lease behind "that ID is already logged in". New
        // table, so `CREATE TABLE IF NOT EXISTS` is enough, and `presence.rs` ensures it at
        // every entry point for the same reason `claims.rs` does: the read runs on the login
        // path, and a missing table there would become an `Err` where the only safe answers
        // are a wrong one or none at all.
        //
        // **This line is what makes it wired rather than merely working.** `CLAUDE.md`'s
        // "Built is not wired" section is about exactly the state where this is absent.
        crate::presence::create_tables(&conn)?;
        // The two AP-spend counters. ALTERed onto `characters`, which is NOT a new table, so
        // this carries its own PRAGMA guard - see the note in that module.
        crate::abilityspend::create_tables(&conn)?;
        // Invite codes and recovery codes. New tables, so `CREATE TABLE IF NOT EXISTS` is
        // enough. `codes.rs` also ensures them inside each entry point, for the same
        // belt-and-braces reason `claims.rs` does - but this line is the one that makes the
        // module WIRED rather than merely working, and its absence is exactly the state
        // CLAUDE.md calls "built is not wired".
        crate::codes::create_tables(&conn)?;
        // One row per (scope, scope_id, perk) holding the last UTC day it was claimed - the
        // Maple Administrator's daily allowance. A whole new table, so `CREATE TABLE IF NOT
        // EXISTS` is enough. `dailyperks.rs` also ensures it inside every entry point, for the
        // same belt-and-braces reason `claims.rs` and `codes.rs` do; this line is what makes
        // the module WIRED rather than merely working.
        crate::dailyperks::create_tables(&conn)?;
        // The migration credential columns. These are ALTERed onto `migrations`, which is
        // NOT a new table, so the call carries its own PRAGMA guard - see that module.
        // Every claim entry point already calls this; doing it here too makes the module
        // wired rather than merely working.
        crate::migration::ensure_columns(&conn)?;
        Ok(Self { conn: Mutex::new(conn) })
    }

    /// `characters.mesos`, added to `characters` after the fact.
    ///
    /// **Not in the `CREATE TABLE` above, for the same reason the slot columns are not.**
    /// `CREATE TABLE IF NOT EXISTS` does nothing at all to a table that already exists, so a
    /// column added there would appear only in databases created from scratch - and the owner's has
    /// characters in it. `ALTER TABLE ADD COLUMN` is not idempotent (it raises "duplicate
    /// column name") and this schema runs on **every** open, hence the `PRAGMA table_info`
    /// guard.
    ///
    /// Every existing character starts on 0. When the column was added nothing credited mesos,
    /// so 0 was what they already had rather than a value being overwritten. **Meso drops
    /// credit real balances now**, and this stays safe only because of the `PRAGMA` guard: it
    /// never runs against a database that already carries the column. Unlike the slot-count
    /// repair below, this is not a correction and cannot destroy a balance.
    ///
    /// Mesos are **not** a field of `net::opcode::Character`, so this column is deliberately
    /// outside `character.rs`'s exhaustive destructure: adding it there would mean changing a
    /// protocol type from the storage side. `crate::inventory::Store::mesos` and friends are
    /// the API.
    fn add_meso_column(conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("PRAGMA table_info(characters)")?;
        let exists = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<std::result::Result<Vec<_>, _>>()?
            .iter()
            .any(|name| name == "mesos");
        drop(stmt);
        if !exists {
            conn.execute(
                "ALTER TABLE characters ADD COLUMN mesos INTEGER NOT NULL DEFAULT 0",
                [],
            )?;
        }
        Ok(())
    }

    /// `accounts.email`, added to `accounts` after the fact.
    ///
    /// Same shape and same reason as [`Self::add_meso_column`]: `CREATE TABLE IF NOT EXISTS`
    /// does nothing to a table that already exists, and the owner's database has accounts in it,
    /// so a column named in the schema above would exist only in a fresh database. `ALTER
    /// TABLE ADD COLUMN` raises "duplicate column name" on the second open, hence the guard.
    ///
    /// **Nullable, and every existing account gets NULL.** That is a statement of fact rather
    /// than a default being imposed: nothing has ever recorded an email here, so there is no
    /// value that could be overwritten, and an account without one still signs in by name.
    ///
    /// It exists because the launcher's sign-in field is labelled *email* - the real service
    /// authenticates by email and the login screen shows a masked one - and because
    /// [`Self::validate_name`] allows only `[A-Za-z0-9_]`, so an email-shaped string can
    /// never be an account **name**. Without this column "log in with your email" would be a
    /// field that cannot match anything.
    ///
    /// The uniqueness index is `WHERE email IS NOT NULL`: a plain `UNIQUE` column in SQLite
    /// permits many NULLs, which is the behaviour wanted, but being explicit means the intent
    /// survives someone later making the column `NOT NULL`. `COLLATE NOCASE` because email
    /// addresses are not case-sensitive in the half anyone types.
    fn add_account_email_column(conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("PRAGMA table_info(accounts)")?;
        let exists = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<std::result::Result<Vec<_>, _>>()?
            .iter()
            .any(|name| name == "email");
        drop(stmt);
        if !exists {
            conn.execute("ALTER TABLE accounts ADD COLUMN email TEXT COLLATE NOCASE", [])?;
        }
        // Outside the `if`: an index is idempotent on its own and a database that gained the
        // column before this index existed still needs it.
        conn.execute(
            "CREATE UNIQUE INDEX IF NOT EXISTS idx_accounts_email
                 ON accounts(email) WHERE email IS NOT NULL",
            [],
        )?;
        Ok(())
    }

    /// `accounts.is_gm`, added to `accounts` after the fact.
    ///
    /// The owner, 2026-08-29: *"can you please make GM commands only available to accounts with GM
    /// status? All commands should have this gate for now until otherwise specified."*
    ///
    /// Same `PRAGMA table_info` guard and the same reason as [`Self::add_meso_column`]:
    /// `CREATE TABLE IF NOT EXISTS` does nothing to a table that already exists, and `ALTER
    /// TABLE ADD COLUMN` raises "duplicate column name" on the second open.
    ///
    /// **Every existing account gets `0`, including the one the owner plays.** That is the safe
    /// direction and it is deliberate: a migration that guessed which account should be a GM
    /// would be inventing an authorisation decision. `maplecw-useradd --gm <name>` grants it,
    /// and that is a person choosing rather than a schema assuming.
    ///
    /// **This is not a security boundary and must not be described as one.** The game socket
    /// carries no credentials; the flag says which *account* may use GM commands, and which
    /// account a connection is served as is decided by a launcher claim, not by anything the
    /// client proves. It stops a second account on this machine from using `!item`; it stops
    /// nothing that can reach the port.
    fn add_account_gm_column(conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("PRAGMA table_info(accounts)")?;
        let exists = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<std::result::Result<Vec<_>, _>>()?
            .iter()
            .any(|name| name == "is_gm");
        drop(stmt);
        if !exists {
            conn.execute(
                "ALTER TABLE accounts ADD COLUMN is_gm INTEGER NOT NULL DEFAULT 0",
                [],
            )?;
        }
        Ok(())
    }

    /// `characters.exp`, added to `characters` after the fact.
    ///
    /// Same shape and same reason as [`Self::add_meso_column`]: `CREATE TABLE IF NOT EXISTS`
    /// does nothing to a table that already exists, and the owner's has characters in it, so a
    /// column named in the schema above would exist only in a fresh database. `ALTER TABLE
    /// ADD COLUMN` raises "duplicate column name" on the second open, hence the guard.
    ///
    /// Every existing character starts on 0, and that is a statement of fact rather than a
    /// default being imposed: **nothing has ever awarded a single point of experience**, so
    /// there is no value here that could be overwritten.
    ///
    /// There is no `sp` column beside it. The stat block's SP field forks on the job and the
    /// branch every character here takes sends a pool list nobody has decoded - see the note
    /// on `net::opcode::Character::exp`. Adding a column for a value that cannot be put on
    /// the wire would be storage pretending to be a feature.
    fn add_experience_column(conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("PRAGMA table_info(characters)")?;
        let exists = stmt
            .query_map([], |row| row.get::<_, String>(1))?
            .collect::<std::result::Result<Vec<_>, _>>()?
            .iter()
            .any(|name| name == "exp");
        drop(stmt);
        if !exists {
            conn.execute("ALTER TABLE characters ADD COLUMN exp INTEGER NOT NULL DEFAULT 0", [])?;
        }
        Ok(())
    }

    /// The six inventory slot counts, added to `characters` after the fact.
    ///
    /// **Not in the `CREATE TABLE` above, and that is on purpose.** `CREATE TABLE IF NOT
    /// EXISTS` does nothing at all to a table that already exists, so a column added there
    /// would appear only in databases created from scratch - and the owner's has characters in
    /// it. Every other repair in this schema is an idempotent `UPDATE` for the same reason.
    ///
    /// `ALTER TABLE ADD COLUMN` is not idempotent - it fails with "duplicate column name" -
    /// so the existing columns are read first. `PRAGMA table_info` rather than the
    /// `pragma_table_info` table-valued function, because the pragma works on every SQLite
    /// build and the TVF needs introspection left enabled.
    ///
    /// A character created before this existed gets [`net::opcode::DEFAULT_INVENTORY_SLOTS`]
    /// through the column default, which is the same bag a new one gets.
    fn add_inventory_slot_columns(conn: &Connection) -> Result<()> {
        let mut existing = std::collections::HashSet::new();
        {
            let mut stmt = conn.prepare("PRAGMA table_info(characters)")?;
            let names = stmt.query_map([], |row| row.get::<_, String>(1))?;
            for name in names {
                existing.insert(name?);
            }
        }

        // The sixth inventory got a name once the owner sent a screenshot of the tabs, so its
        // column is renamed rather than replaced - adding `slots_deco` beside a populated
        // `slots_sixth` would read back a default and drop whatever was stored.
        let (from, to) = INVENTORY_SLOT_COLUMN_RENAMED_FROM;
        if existing.contains(from) && !existing.contains(to) {
            conn.execute(
                &format!("ALTER TABLE characters RENAME COLUMN {from} TO {to}"),
                [],
            )?;
            existing.remove(from);
            existing.insert(to.to_string());
        }

        for column in INVENTORY_SLOT_COLUMNS {
            if existing.contains(column) {
                continue;
            }
            conn.execute(
                &format!(
                    "ALTER TABLE characters ADD COLUMN {column} INTEGER NOT NULL DEFAULT {}",
                    net::opcode::DEFAULT_INVENTORY_SLOTS
                ),
                [],
            )?;
        }

        // A one-off repair, in the same spirit as the `map_id = 0` one above and with the
        // same justification: **24 was never a value anybody chose.**
        //
        // The columns were created for one commit with a default of 24 - the classic
        // MapleStory bag, carried over from a different game version - before the owner's
        // screenshot showed this client's window holds thirty. No client ever saw a 24, and
        // nothing in the server can set a slot count deliberately yet, so a stored 24 can
        // only have come from that default.
        //
        // **This has to go the moment anything can buy a slot**, because from then on a 24
        // could be somebody's actual bag. Idempotent until then.
        if net::opcode::DEFAULT_INVENTORY_SLOTS != SUPERSEDED_INVENTORY_SLOTS {
            for column in INVENTORY_SLOT_COLUMNS {
                conn.execute(
                    &format!(
                        "UPDATE characters SET {column} = {} WHERE {column} = {}",
                        net::opcode::DEFAULT_INVENTORY_SLOTS, SUPERSEDED_INVENTORY_SLOTS
                    ),
                    [],
                )?;
            }
        }
        Ok(())
    }

    /// Lock the connection. Poisoning cannot lose data here - the recovered guard is
    /// still a usable connection - so the lock is recovered rather than panicking.
    pub(crate) fn conn(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }

    /// Account names are used in URLs, logs, and the game protocol, so keep them plain.
    /// The rule for an account name. Public since 2026-09-05 so `auth::register` can refuse a
    /// bad name BEFORE spending a registration code, with the same sentence this would give.
    pub fn validate_name(name: &str) -> Result<()> {
        let bad = |reason| {
            Err(StoreError::InvalidAccountName {
                name: name.to_string(),
                reason,
            })
        };
        if name.len() < 3 || name.len() > 24 {
            return bad("must be 3-24 characters");
        }
        if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return bad("only letters, digits and underscore are allowed");
        }
        Ok(())
    }

    /// Create an account. The password is hashed here and never stored as given.
    pub fn create_account(&self, name: &str, password: &str) -> Result<i64> {
        Self::validate_name(name)?;
        let hash = hash_password(password)?;

        let existing: Option<i64> = self.conn()
            .query_row(
                "SELECT id FROM accounts WHERE name = ?1",
                params![name],
                |r| r.get(0),
            )
            .optional()?;
        if existing.is_some() {
            return Err(StoreError::AccountExists {
                name: name.to_string(),
            });
        }

        self.conn().execute(
            "INSERT INTO accounts (name, password_hash, enabled, created_at)
             VALUES (?1, ?2, 1, ?3)",
            params![name, hash, Self::now()],
        )?;
        Ok(self.conn().last_insert_rowid())
    }

    pub fn get_account(&self, name: &str) -> Result<Option<Account>> {
        let acc = self.conn()
            .query_row(
                "SELECT id, name, email, enabled, is_gm, created_at, last_login
                 FROM accounts WHERE name = ?1",
                params![name],
                |r| {
                    Ok(Account {
                        id: r.get(0)?,
                        name: r.get(1)?,
                        email: r.get(2)?,
                        enabled: r.get::<_, i64>(3)? != 0,
                        is_gm: r.get::<_, i64>(4)? != 0,
                        created_at: r.get(5)?,
                        last_login: r.get(6)?,
                    })
                },
            )
            .optional()?;
        Ok(acc)
    }

    /// Resolve an account by **name or email**, whichever the string matches.
    ///
    /// The launcher's sign-in field is labelled *email*, but [`Self::validate_name`] allows
    /// only `[A-Za-z0-9_]`, so an email-shaped string can never be an account name and a
    /// name can never be an email. **The two namespaces cannot collide**, which is what makes
    /// one field for both safe rather than merely convenient: there is no string that is a
    /// valid name and a valid email at once, so this can never have to choose between two
    /// accounts.
    ///
    /// Both comparisons are `COLLATE NOCASE` - `accounts.name` was declared that way and the
    /// email index matches it - so case is not a way to miss your own account.
    pub fn get_account_by_identity(&self, identity: &str) -> Result<Option<Account>> {
        let acc = self
            .conn()
            .query_row(
                "SELECT id, name, email, enabled, is_gm, created_at, last_login
                   FROM accounts
                  WHERE name = ?1 OR email = ?1",
                params![identity],
                |r| {
                    Ok(Account {
                        id: r.get(0)?,
                        name: r.get(1)?,
                        email: r.get(2)?,
                        enabled: r.get::<_, i64>(3)? != 0,
                        is_gm: r.get::<_, i64>(4)? != 0,
                        created_at: r.get(5)?,
                        last_login: r.get(6)?,
                    })
                },
            )
            .optional()?;
        Ok(acc)
    }

    /// Set or clear an account's email.
    ///
    /// `None` clears it. A duplicate is refused by the unique index rather than silently
    /// reseating which account an address signs into.
    pub fn set_email(&self, name: &str, email: Option<&str>) -> Result<()> {
        let n = self.conn().execute(
            "UPDATE accounts SET email = ?2 WHERE name = ?1",
            params![name, email],
        )?;
        if n == 0 {
            return Err(StoreError::NoSuchAccount {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    /// [`Self::authenticate`], but the identity may be a name **or** an email.
    ///
    /// This is what the launcher calls. It resolves the identity to a name and defers to
    /// `authenticate`, so there is exactly one implementation of "check a password and issue
    /// a token" and this cannot drift away from it - in particular it cannot accidentally
    /// skip the `enabled` check or the token write.
    ///
    /// **The unknown-identity path still costs a verify.** `authenticate` already burns a
    /// dummy argon2id verification when the name does not exist, so routing an unresolvable
    /// email to a name that cannot exist keeps the timing indistinguishable. Returning
    /// `InvalidCredentials` directly from here would answer a bad email far faster than a bad
    /// password and hand back exactly the account-enumeration oracle
    /// [`AuthOutcome`] exists to deny.
    pub fn authenticate_identity(&self, identity: &str, password: &str) -> Result<AuthOutcome> {
        let name = match self.get_account_by_identity(identity)? {
            Some(acc) => acc.name,
            // Not a valid account name (spaces are outside `validate_name`'s alphabet), so
            // the lookup inside `authenticate` misses and its dummy verify runs.
            None => " no such identity ".to_string(),
        };
        self.authenticate(&name, password)
    }

    pub fn list_accounts(&self) -> Result<Vec<Account>> {
        // Bind the guard: a prepared statement borrows the connection, so the lock has
        // to outlive it.
        let conn = self.conn();
        let mut stmt = conn.prepare(
            "SELECT id, name, email, enabled, is_gm, created_at, last_login FROM accounts ORDER BY id",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok(Account {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    email: r.get(2)?,
                    enabled: r.get::<_, i64>(3)? != 0,
                    is_gm: r.get::<_, i64>(4)? != 0,
                    created_at: r.get(5)?,
                    last_login: r.get(6)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    /// Grant or revoke GM status.
    ///
    /// Deliberately a separate call from anything that creates an account: granting is a
    /// decision, and a decision should be made in one obvious place rather than fall out of a
    /// default. `maplecw-useradd --gm <name>`.
    pub fn set_gm(&self, name: &str, is_gm: bool) -> Result<()> {
        let n = self.conn().execute(
            "UPDATE accounts SET is_gm = ?2 WHERE name = ?1",
            params![name, i64::from(is_gm)],
        )?;
        if n == 0 {
            return Err(StoreError::NoSuchAccount { name: name.to_string() });
        }
        Ok(())
    }

    /// Is this account allowed to use GM commands? `false` for an account that is not there,
    /// which is the answer that refuses rather than the one that panics.
    pub fn is_gm(&self, account_id: i64) -> Result<bool> {
        Ok(self
            .conn()
            .query_row(
                "SELECT is_gm FROM accounts WHERE id = ?1",
                params![account_id],
                |r| r.get::<_, i64>(0),
            )
            .optional()?
            .map(|v| v != 0)
            .unwrap_or(false))
    }

    pub fn set_enabled(&self, name: &str, enabled: bool) -> Result<()> {
        let n = self.conn().execute(
            "UPDATE accounts SET enabled = ?2 WHERE name = ?1",
            params![name, i64::from(enabled)],
        )?;
        if n == 0 {
            return Err(StoreError::NoSuchAccount {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn set_password(&self, name: &str, password: &str) -> Result<()> {
        let hash = hash_password(password)?;
        let n = self.conn().execute(
            "UPDATE accounts SET password_hash = ?2 WHERE name = ?1",
            params![name, hash],
        )?;
        if n == 0 {
            return Err(StoreError::NoSuchAccount {
                name: name.to_string(),
            });
        }
        // Changing a password invalidates existing sessions - and the login claim with them.
        //
        // The claim is not a credential, but it is a standing instruction to serve the next
        // game connection as this account, and it outlives a session by hours. Leaving it
        // would let the old password's last act survive the password itself, which is exactly
        // what revoking the sessions is for. Scoped to this account, so changing one
        // password cannot drop somebody else out of the game.
        if let Some(acc) = self.get_account(name)? {
            self.revoke_account_sessions(acc.id)?;
            self.clear_login_claims_for(acc.id)?;
        }
        Ok(())
    }

    /// [`Self::set_password`], but the account may be named by its **email** as well.
    ///
    /// The launcher signs in with either identity, so an administrator resetting a password
    /// should be able to name the account the same way - otherwise "log in with your email"
    /// is a half-built idea that works until the day it matters. `maplecw-useradd --passwd`
    /// calls this.
    ///
    /// Unlike [`Self::authenticate_identity`] this does **not** have to hide whether the
    /// account exists: it is an administrative command run by someone holding the database,
    /// not an authentication path, and telling them "no such account" is the useful answer.
    pub fn set_password_by_identity(&self, identity: &str, password: &str) -> Result<()> {
        let account = self.get_account_by_identity(identity)?.ok_or_else(|| {
            StoreError::NoSuchAccount {
                name: identity.to_string(),
            }
        })?;
        self.set_password(&account.name, password)
    }

    /// Authenticate and, on success, issue a session token.
    ///
    /// The password is verified even when the account does not exist, so the two cases
    /// take comparable time and cannot be told apart by an observer.
    pub fn authenticate(&self, name: &str, password: &str) -> Result<AuthOutcome> {
        let row: Option<(i64, String, i64)> = self.conn()
            .query_row(
                "SELECT id, password_hash, enabled FROM accounts WHERE name = ?1",
                params![name],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .optional()?;

        let Some((id, stored, enabled)) = row else {
            // Verify against a dummy so a missing account costs roughly the same work
            // as a wrong password, and the two cannot be told apart by timing.
            let _ = verify_password(password, dummy_hash());
            return Ok(AuthOutcome::InvalidCredentials);
        };

        if !verify_password(password, &stored)? {
            return Ok(AuthOutcome::InvalidCredentials);
        }
        if enabled == 0 {
            return Ok(AuthOutcome::Disabled);
        }

        let now = Self::now();
        self.conn().execute(
            "UPDATE accounts SET last_login = ?2 WHERE id = ?1",
            params![id, now],
        )?;

        let NewSession { token, hash } = new_token();
        self.conn().execute(
            "INSERT INTO sessions (token_hash, account_id, issued_at, expires_at)
             VALUES (?1, ?2, ?3, ?4)",
            params![hash, id, now, now + SESSION_TTL_SECS],
        )?;

        Ok(AuthOutcome::Ok { account_id: id, token })
    }

    /// Resolve a session token to its account, if it is valid and unexpired.
    pub fn validate_session(&self, token: &str) -> Result<Option<Account>> {
        let hash = hash_token(token);
        let now = Self::now();
        let account_id: Option<i64> = self.conn()
            .query_row(
                "SELECT account_id FROM sessions WHERE token_hash = ?1 AND expires_at > ?2",
                params![hash, now],
                |r| r.get(0),
            )
            .optional()?;

        let Some(id) = account_id else {
            return Ok(None);
        };
        let acc = self.conn()
            .query_row(
                "SELECT id, name, email, enabled, is_gm, created_at, last_login
                 FROM accounts WHERE id = ?1 AND enabled = 1",
                params![id],
                |r| {
                    Ok(Account {
                        id: r.get(0)?,
                        name: r.get(1)?,
                        email: r.get(2)?,
                        enabled: r.get::<_, i64>(3)? != 0,
                        is_gm: r.get::<_, i64>(4)? != 0,
                        created_at: r.get(5)?,
                        last_login: r.get(6)?,
                    })
                },
            )
            .optional()?;
        Ok(acc)
    }

    /// Consume a session: valid exactly once. Used for the launcher → game handoff, so
    /// a captured token cannot be replayed.
    pub fn consume_session(&self, token: &str) -> Result<Option<Account>> {
        let acc = self.validate_session(token)?;
        if acc.is_some() {
            self.conn().execute(
                "DELETE FROM sessions WHERE token_hash = ?1",
                params![hash_token(token)],
            )?;
        }
        Ok(acc)
    }

    pub fn revoke_account_sessions(&self, account_id: i64) -> Result<usize> {
        Ok(self.conn().execute(
            "DELETE FROM sessions WHERE account_id = ?1",
            params![account_id],
        )?)
    }

    /// Drop expired rows. Cheap to call periodically.
    pub fn purge_expired_sessions(&self) -> Result<usize> {
        Ok(self.conn().execute(
            "DELETE FROM sessions WHERE expires_at <= ?1",
            params![Self::now()],
        )?)
    }
}

/// An argon2id hash of a random secret, used only to spend comparable time when the
/// account does not exist. Nothing can authenticate against it.
///
/// Computed at runtime rather than hard-coded: a hand-written constant that fails to
/// parse would make `verify_password` return early, quietly removing the timing
/// equalisation this exists to provide.
fn dummy_hash() -> &'static str {
    static DUMMY: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    DUMMY.get_or_init(|| {
        let filler = crate::session::new_token().token;
        hash_password(&filler).expect("hashing a 64-char token cannot fail")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> Store {
        Store::open_in_memory().unwrap()
    }

    /// Opening the same file twice must not fail on the slot columns.
    ///
    /// `ALTER TABLE ADD COLUMN` is not idempotent - it raises "duplicate column name" - and
    /// the whole schema is applied on **every** open, not once. An in-memory store cannot
    /// catch this: it is a fresh database each time, so the second open never happens. This
    /// test needs a real file for exactly that reason.
    ///
    /// It also covers the upgrade the deployed database will actually do: the first open
    /// creates `characters` WITHOUT the columns, because they are not in the `CREATE TABLE`.
    #[test]
    fn reopening_a_database_does_not_re_add_the_slot_columns() {
        let dir = std::env::temp_dir().join(format!("maplecw-reopen-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("reopen.db");
        let _ = std::fs::remove_file(&path);

        let first = Store::open(&path).unwrap();
        let account = first.create_account("wisp", "correct horse battery").unwrap();
        let chr = net::opcode::Character {
            name: "Wanderer".to_string(),
            ..Default::default()
        };
        first.create_character(account, 0, &chr).unwrap();
        drop(first);

        // The second open runs the same schema again. If the guard were missing this is
        // where it would fail, and it would fail for every existing database - the owner's
        // included - rather than in a test.
        let second = Store::open(&path).unwrap();
        let loaded = second.characters_for(account, 0).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(
            loaded[0].inventory_slots,
            [net::opcode::DEFAULT_INVENTORY_SLOTS; net::opcode::INVENTORY_COUNT]
        );
        drop(second);
        let _ = std::fs::remove_file(&path);
    }

    /// **The upgrade path, run against a copy of the owner's real database.**
    ///
    /// Every other schema test starts from a file this test suite created, which is exactly
    /// the case a broken migration still passes: `CREATE TABLE IF NOT EXISTS` does nothing to
    /// a table that already exists, so a column added there works on a fresh file and never
    /// appears in a deployed one. The only way to be sure is to open a database that was
    /// written by an older build.
    ///
    /// Skipped when `maplecw.db` is absent - it is gitignored live state, not a fixture - and
    /// it operates on a **copy**, including the WAL and shm sidecars so the copy is the same
    /// database rather than a truncated one. Nothing here writes to the original.
    #[test]
    fn wisps_real_database_upgrades_in_place() {
        let live = std::path::Path::new("../../maplecw.db");
        if !live.exists() {
            return; // gitignored live state; the rest of the suite covers the synthetic cases
        }
        let dir = std::env::temp_dir().join(format!("maplecw-upgrade-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let copy = dir.join("maplecw.db");
        for suffix in ["", "-wal", "-shm"] {
            let from = live.with_extension(format!("db{suffix}"));
            if from.exists() {
                std::fs::copy(&from, dir.join(format!("maplecw.db{suffix}"))).unwrap();
            }
        }

        let store = Store::open(&copy).expect("the live database must open under the new schema");
        let accounts = store.list_accounts().unwrap();
        assert!(!accounts.is_empty(), "the live database has accounts in it");

        let mut characters = 0;
        for account in &accounts {
            for chr in store.characters_for(account.id, 0).unwrap() {
                characters += 1;
                // The new columns read on a row that predates them. **Not asserted to be
                // zero.** This used to require `mesos == 0` and an empty bag, which was true
                // only because nothing had ever written either - and on 2026-08-20 it went
                // red because the owner ran `!item 1302000` on their own character. A test over
                // live, mutable state must assert what the *upgrade* guarantees, not what
                // the player happens not to have done yet.
                store.mesos(chr.id).unwrap();
                // `chr` came out of `characters_for`, which selects `exp` - so reaching
                // here at all means the new column read on a pre-existing row.
                let _ = chr.exp;
                // The bag reads rather than erroring, and it is sized by the character.
                let bag = store.bag(chr.id).unwrap();
                assert_eq!(bag.slots, chr.inventory_slots);
                assert!(
                    bag.items.iter().all(|i| i.slot >= 1),
                    "slot 0 is the hole that makes slots 1-based; a 0 here is a bad read"
                );
                // The worn slots still read, now with the stat tail.
                //
                // **This used to require every equip to read `stats == None`, and on
                // 2026-09-09 it went red because the owner used `!scroll` on their own character.**
                // That is the *second* time this test asserted a property of the owner's play
                // rather than a property of the upgrade - the meso line above carries the
                // first, from 2026-08-20 - and the lesson written there applies unchanged: a
                // test over live, mutable state must assert what the upgrade guarantees.
                //
                // A stored stat block on a live row is now a legitimate state, so "no stats"
                // is no longer a fact about this file. **The guarantee itself has not been
                // dropped**; it is held where the database can be kept still:
                // `a_database_written_before_mesos_and_the_stat_columns_upgrades` below winds
                // a real schema back and requires `None` rather than zeros, and
                // `inventory::tests::stats_that_were_never_stored_read_back_as_none_not_as_zeros`
                // pins the same thing at the row level. Neither can be perturbed by playing.
                //
                // What is left here is what this test is *for*: the column tail reads at all
                // on rows that predate it, and the list still matches the character record.
                let worn = store.equipped_items(chr.id).unwrap();
                assert_eq!(worn.len(), chr.equips.len(), "the equipped list did not change");
                for e in &worn {
                    // A stored block must be whole: `item_from_row` reads the stat columns and
                    // the failed-slot count at fixed offsets, so a decode that ran off the end
                    // shows up as a panic here rather than as quietly wrong stats in a tooltip.
                    if let Some(stats) = e.stats {
                        let _ = stats.options.remaining_enhancements;
                    }
                }
            }
        }
        assert!(characters > 0, "the live database has a real character in it");

        // And it survives a second open - the schema runs on EVERY open and ALTER TABLE ADD
        // COLUMN is not idempotent.
        drop(store);
        let again = Store::open(&copy).expect("the second open is where a bad ALTER shows up");
        assert_eq!(again.list_accounts().unwrap().len(), accounts.len());
        drop(again);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A character row written before `mesos` existed reads back as zero, and the storage the
    /// new tables need is there. The synthetic twin of the test above, so the suite still
    /// covers the upgrade on a machine with no live database.
    #[test]
    fn a_database_written_before_mesos_and_the_stat_columns_upgrades() {
        let dir = std::env::temp_dir().join(format!("maplecw-oldschema-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("old.db");
        let _ = std::fs::remove_file(&path);

        let account;
        let chr;
        {
            let store = Store::open(&path).unwrap();
            account = store.create_account("wisp", "correct horse battery").unwrap();
            chr = store
                .create_character(
                    account,
                    0,
                    &net::opcode::Character {
                        name: "Oldtimer".to_string(),
                        equips: vec![(5, 1040002)],
                        ..Default::default()
                    },
                )
                .unwrap();
            // Wind the schema back to what an older build wrote: no meso column, no stat tail
            // on `equipment`, and no new tables at all.
            let conn = store.conn();
            conn.execute("ALTER TABLE characters DROP COLUMN mesos", []).unwrap();
            conn.execute("ALTER TABLE characters DROP COLUMN exp", []).unwrap();
            for column in crate::inventory::EQUIP_STAT_COLUMNS {
                conn.execute(&format!("ALTER TABLE equipment DROP COLUMN {column}"), []).unwrap();
            }
            conn.execute("DROP TABLE inventory", []).unwrap();
            conn.execute("DROP TABLE storage_item", []).unwrap();
            conn.execute("DROP TABLE storage", []).unwrap();
        }

        // The upgrade. This is the statement that has to work on a file it did not create.
        let store = Store::open(&path).expect("an older database must upgrade, not fail");
        let loaded = store.characters_for(account, 0).unwrap();
        assert_eq!(loaded.len(), 1, "the character survived");
        assert_eq!(loaded[0].equips, vec![(5, 1040002)]);
        assert_eq!(store.mesos(chr.id).unwrap(), 0, "the meso column arrived with a default");
        assert_eq!(loaded[0].exp, 0, "the exp column arrived with a default");
        assert_eq!(
            store.equipped_items(chr.id).unwrap()[0].stats,
            None,
            "an equip written before the stat columns is 'no stats stored', not zeros"
        );
        // The new containers work on the upgraded file.
        store.add_mesos(chr.id, 250).unwrap();
        assert_eq!(store.mesos(chr.id).unwrap(), 250);
        store
            .set_inventory_slot(
                chr.id,
                crate::inventory::InventoryType::Etc,
                1,
                &crate::inventory::Item::bundle(4000000, 3),
            )
            .unwrap();
        assert_eq!(store.bag(chr.id).unwrap().items.len(), 1);
        assert!(store.storage(account).unwrap().is_empty());

        // Third open: every ALTER in the schema runs again and must be a no-op.
        drop(store);
        let again = Store::open(&path).expect("ALTER TABLE ADD COLUMN is not idempotent");
        assert_eq!(again.mesos(chr.id).unwrap(), 250, "and nothing was reset");
        drop(again);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A character row written before the columns existed still comes back with a bag.
    ///
    /// The column default is what does it, and this proves it rather than assuming it: the
    /// row is inserted by hand into a table the ALTER has not touched yet, which is exactly
    /// the shape of every character already in the owner's database.
    #[test]
    fn a_character_from_before_the_columns_still_gets_the_default_bag() {
        let s = store();
        let account = s.create_account("wisp", "correct horse battery").unwrap();
        {
            let conn = s.conn();
            for column in INVENTORY_SLOT_COLUMNS {
                conn.execute(&format!("ALTER TABLE characters DROP COLUMN {column}"), [])
                    .unwrap();
            }
            conn.execute(
                "INSERT INTO characters (
                     account_id, world_id, name, gender, skin, face, hair, level, job,
                     strength, dexterity, intelligence, luck,
                     hp, max_hp, mp, max_mp, ap, map_id, created_at
                 ) VALUES (?1, 0, 'Oldtimer', 0, 0, 20000, 30000, 1, 0,
                           12, 5, 4, 4, 50, 50, 5, 5, 0, 1, 0)",
                params![account],
            )
            .unwrap();
            Store::add_inventory_slot_columns(&conn).unwrap();
        }

        let loaded = s.characters_for(account, 0).unwrap();
        assert_eq!(loaded.len(), 1);
        assert_eq!(
            loaded[0].inventory_slots,
            [net::opcode::DEFAULT_INVENTORY_SLOTS; net::opcode::INVENTORY_COUNT],
            "a character made before the bag existed came back with no slots"
        );
    }

    #[test]
    fn creates_and_authenticates() {
        let s = store();
        let id = s.create_account("player_one", "hunter2hunter2").unwrap();
        match s.authenticate("player_one", "hunter2hunter2").unwrap() {
            AuthOutcome::Ok { account_id, token } => {
                assert_eq!(account_id, id);
                assert!(!token.is_empty());
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn plain_password_is_never_stored() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        let stored: String = s.conn()
            .query_row(
                "SELECT password_hash FROM accounts WHERE name='player_one'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(!stored.contains("hunter2hunter2"));
        assert!(stored.starts_with("$argon2id$"));
    }

    #[test]
    fn wrong_password_and_unknown_account_look_the_same() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        assert_eq!(
            s.authenticate("player_one", "wrong-password").unwrap(),
            AuthOutcome::InvalidCredentials
        );
        assert_eq!(
            s.authenticate("nobody_here", "wrong-password").unwrap(),
            AuthOutcome::InvalidCredentials
        );
    }

    #[test]
    fn duplicate_names_are_rejected_case_insensitively() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        assert!(matches!(
            s.create_account("PLAYER_ONE", "another-password"),
            Err(StoreError::AccountExists { .. })
        ));
    }

    #[test]
    fn disabled_accounts_cannot_log_in() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        s.set_enabled("player_one", false).unwrap();
        assert_eq!(
            s.authenticate("player_one", "hunter2hunter2").unwrap(),
            AuthOutcome::Disabled
        );
    }

    #[test]
    fn session_validates_then_is_consumed_once() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        let AuthOutcome::Ok { token, .. } =
            s.authenticate("player_one", "hunter2hunter2").unwrap()
        else {
            panic!("login failed");
        };

        assert!(s.validate_session(&token).unwrap().is_some());
        assert!(s.consume_session(&token).unwrap().is_some());
        // Replaying the same token must fail.
        assert!(s.validate_session(&token).unwrap().is_none());
        assert!(s.consume_session(&token).unwrap().is_none());
    }

    #[test]
    fn raw_token_is_not_in_the_database() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        let AuthOutcome::Ok { token, .. } =
            s.authenticate("player_one", "hunter2hunter2").unwrap()
        else {
            panic!("login failed");
        };
        let stored: String = s.conn()
            .query_row("SELECT token_hash FROM sessions", [], |r| r.get(0))
            .unwrap();
        assert_ne!(stored, token);
    }

    #[test]
    fn changing_password_revokes_sessions() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        let AuthOutcome::Ok { token, .. } =
            s.authenticate("player_one", "hunter2hunter2").unwrap()
        else {
            panic!("login failed");
        };
        s.set_password("player_one", "a-brand-new-password").unwrap();
        assert!(s.validate_session(&token).unwrap().is_none());

        // `.is_ok()` was the assertion here, and it checked almost nothing: `authenticate`
        // returns `Ok(AuthOutcome::InvalidCredentials)` for a REFUSED login, so the old line
        // passed whether or not the new password worked - it would have stayed green if
        // `set_password` had written a hash nothing could verify. Match the outcome.
        assert!(
            matches!(
                s.authenticate("player_one", "a-brand-new-password").unwrap(),
                AuthOutcome::Ok { .. }
            ),
            "the new password must actually sign in"
        );
        assert_eq!(
            s.authenticate("player_one", "hunter2hunter2").unwrap(),
            AuthOutcome::InvalidCredentials,
            "and the old one must not"
        );
    }

    #[test]
    fn a_password_can_be_reset_by_email_without_knowing_the_old_one() {
        // What `maplecw-useradd --passwd <email>` does. Nothing anywhere in the reset path
        // takes the previous password: it is an administrative reset against a database the
        // caller already holds, not a change-my-password flow.
        let s = store();
        s.create_account("maplecw", "the-forgotten-one").unwrap();
        s.set_email("maplecw", Some("wispplayer@example.com")).unwrap();

        s.set_password_by_identity("wispplayer@example.com", "a-brand-new-password")
            .unwrap();

        assert!(matches!(
            s.authenticate_identity("wispplayer@example.com", "a-brand-new-password").unwrap(),
            AuthOutcome::Ok { .. }
        ));
        // And by name, because they are two identities for one row.
        assert!(matches!(
            s.authenticate("maplecw", "a-brand-new-password").unwrap(),
            AuthOutcome::Ok { .. }
        ));
        assert_eq!(
            s.authenticate("maplecw", "the-forgotten-one").unwrap(),
            AuthOutcome::InvalidCredentials
        );
    }

    #[test]
    fn resetting_by_an_unknown_identity_says_so() {
        // Unlike authentication, this may distinguish: it is run by someone holding the
        // database, and "no such account" is the useful answer rather than a leak.
        let s = store();
        s.create_account("maplecw", "hunter2hunter2").unwrap();
        assert!(matches!(
            s.set_password_by_identity("nobody@example.test", "whatever-else"),
            Err(StoreError::NoSuchAccount { .. })
        ));
    }

    #[test]
    fn changing_a_password_also_clears_that_accounts_login_claim() {
        // The claim is a standing instruction to serve the next game connection as this
        // account, and it outlives a session by hours. A password is changed when someone
        // has lost control of it, so letting the claim survive would let the old password's
        // last act outlive the password.
        let s = store();
        let id = s.create_account("player_one", "hunter2hunter2").unwrap();
        s.stake_login_claim(id, "a-token", crate::LOGIN_CLAIM_TTL_SECS).unwrap();
        assert!(s.current_login_claim().unwrap().is_some());

        s.set_password("player_one", "a-brand-new-password").unwrap();
        assert!(s.current_login_claim().unwrap().is_none());
    }

    #[test]
    fn changing_one_password_does_not_evict_a_different_account() {
        // Scoped, not `clear_login_claims()`. One person resetting a password must not drop
        // somebody else out of the game.
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        let two = s.create_account("player_two", "hunter2hunter2").unwrap();
        s.stake_login_claim(two, "a-token", crate::LOGIN_CLAIM_TTL_SECS).unwrap();

        s.set_password("player_one", "a-brand-new-password").unwrap();
        assert_eq!(
            s.current_login_claim().unwrap().map(|c| c.account_name),
            Some("player_two".to_string()),
            "player_two was playing and had nothing to do with it"
        );
    }

    #[test]
    fn invalid_names_are_rejected() {
        let s = store();
        assert!(s.create_account("ab", "hunter2hunter2").is_err());
        assert!(s.create_account("has space", "hunter2hunter2").is_err());
        assert!(s.create_account("drop;table", "hunter2hunter2").is_err());
    }

    #[test]
    fn dummy_hash_is_a_usable_argon2_hash() {
        // Regression: a hard-coded constant here was invalid Base64, so
        // verify_password returned early and the timing equalisation for unknown
        // accounts silently did nothing.
        let h = dummy_hash();
        assert!(h.starts_with("$argon2id$"), "got {h}");
        assert!(
            !verify_password("some guess", h).expect("dummy hash must parse")
        );
    }

    #[test]
    fn unknown_account_takes_comparable_time_to_a_wrong_password() {
        use std::time::Instant;
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();

        let t0 = Instant::now();
        s.authenticate("player_one", "wrong-password").unwrap();
        let wrong_pw = t0.elapsed();

        let t1 = Instant::now();
        s.authenticate("nobody_here", "wrong-password").unwrap();
        let unknown = t1.elapsed();

        // Argon2 dominates both paths; without the dummy verify the unknown-account
        // case returns almost instantly. Generous bound to stay stable on CI.
        assert!(
            unknown * 5 > wrong_pw,
            "unknown-account path far faster than wrong-password path \
             ({unknown:?} vs {wrong_pw:?}), which leaks account existence"
        );
    }

    #[test]
    fn short_password_is_rejected_at_creation() {
        let s = store();
        assert!(matches!(
            s.create_account("player_one", "short"),
            Err(StoreError::PasswordTooShort { .. })
        ));
    }

    // ---- email as a second identity for the launcher's sign-in field ----

    #[test]
    fn an_account_starts_with_no_email() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        assert_eq!(s.get_account("player_one").unwrap().unwrap().email, None);
    }

    #[test]
    fn identity_resolves_by_name_or_by_email() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        s.set_email("player_one", Some("wisp@example.test")).unwrap();

        let by_name = s.get_account_by_identity("player_one").unwrap().unwrap();
        let by_mail = s.get_account_by_identity("wisp@example.test").unwrap().unwrap();
        assert_eq!(by_name.id, by_mail.id);
        assert_eq!(by_mail.email.as_deref(), Some("wisp@example.test"));
    }

    #[test]
    fn identity_lookup_ignores_case_in_both_namespaces() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        s.set_email("player_one", Some("the owner@Example.test")).unwrap();
        assert!(s.get_account_by_identity("PLAYER_ONE").unwrap().is_some());
        assert!(s.get_account_by_identity("wisp@EXAMPLE.TEST").unwrap().is_some());
    }

    #[test]
    fn signing_in_by_email_issues_a_token() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        s.set_email("player_one", Some("wisp@example.test")).unwrap();

        let out = s.authenticate_identity("wisp@example.test", "hunter2hunter2").unwrap();
        let AuthOutcome::Ok { account_id, token } = out else {
            panic!("email sign-in should succeed, got {out:?}");
        };
        assert_eq!(account_id, s.get_account("player_one").unwrap().unwrap().id);
        // The token is a real session, not a placeholder.
        assert_eq!(
            s.validate_session(&token).unwrap().map(|a| a.name),
            Some("player_one".to_string())
        );
    }

    #[test]
    fn signing_in_by_email_with_the_wrong_password_is_refused() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        s.set_email("player_one", Some("wisp@example.test")).unwrap();
        assert_eq!(
            s.authenticate_identity("wisp@example.test", "not-the-password").unwrap(),
            AuthOutcome::InvalidCredentials
        );
    }

    #[test]
    fn a_disabled_account_cannot_sign_in_by_email_either() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        s.set_email("player_one", Some("wisp@example.test")).unwrap();
        s.set_enabled("player_one", false).unwrap();
        assert_eq!(
            s.authenticate_identity("wisp@example.test", "hunter2hunter2").unwrap(),
            AuthOutcome::Disabled
        );
    }

    #[test]
    fn two_accounts_cannot_share_one_email() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        s.create_account("player_two", "hunter2hunter2").unwrap();
        s.set_email("player_one", Some("wisp@example.test")).unwrap();
        assert!(
            s.set_email("player_two", Some("wisp@example.test")).is_err(),
            "the unique index must refuse to reseat which account an address signs into"
        );
    }

    #[test]
    fn many_accounts_may_have_no_email() {
        // A UNIQUE column in SQLite permits many NULLs. That is the behaviour wanted -
        // every account that predates the column has one - so pin it rather than trust it.
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        s.create_account("player_two", "hunter2hunter2").unwrap();
        assert!(s.get_account("player_one").unwrap().unwrap().email.is_none());
        assert!(s.get_account("player_two").unwrap().unwrap().email.is_none());
    }

    #[test]
    fn an_unknown_email_still_pays_for_a_password_verification() {
        // Same guarantee as `unknown_account_costs_similar_time_to_wrong_password`, on the
        // email path. Returning InvalidCredentials straight from `authenticate_identity`
        // would answer a bad email far faster than a bad password and hand back exactly the
        // account-enumeration oracle AuthOutcome exists to deny.
        use std::time::Instant;
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        s.set_email("player_one", Some("wisp@example.test")).unwrap();

        let t0 = Instant::now();
        s.authenticate_identity("wisp@example.test", "wrong-password").unwrap();
        let wrong_pw = t0.elapsed();

        let t1 = Instant::now();
        s.authenticate_identity("nobody@example.test", "wrong-password").unwrap();
        let unknown = t1.elapsed();

        assert!(
            unknown * 5 > wrong_pw,
            "unknown-email path far faster than wrong-password path              ({unknown:?} vs {wrong_pw:?}), which leaks which addresses have accounts"
        );
    }

    #[test]
    fn clearing_an_email_frees_it_for_another_account() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        s.create_account("player_two", "hunter2hunter2").unwrap();
        s.set_email("player_one", Some("wisp@example.test")).unwrap();
        s.set_email("player_one", None).unwrap();
        s.set_email("player_two", Some("wisp@example.test")).unwrap();
        assert_eq!(
            s.get_account_by_identity("wisp@example.test").unwrap().unwrap().name,
            "player_two"
        );
    }

    // ---- the masked email the client's login screen shows ----

    #[test]
    fn masking_keeps_four_characters_and_the_whole_domain() {
        // The exact shape tools/test-server.ps1 hard-coded as -DisplayName before there was
        // an email column to build it from, so the screen does not change appearance.
        assert_eq!(mask_email("wispplayer@example.com"), "wisp****@example.com");
    }

    #[test]
    fn the_star_count_does_not_leak_the_length() {
        // One star per hidden character would give away how long the address is, which is the
        // one thing masking exists to hide. Two very different local parts must look alike.
        assert_eq!(mask_email("wispa@example.com"), "wisp****@example.com");
        assert_eq!(mask_email("wispplayerandthensome@example.com"), "wisp****@example.com");
    }

    #[test]
    fn a_local_part_shorter_than_the_keep_length_is_not_padded_out() {
        assert_eq!(mask_email("abc@example.com"), "abc****@example.com");
        assert_eq!(mask_email("a@example.com"), "a****@example.com");
        assert_eq!(mask_email("@example.com"), "****@example.com");
    }

    #[test]
    fn something_that_is_not_an_address_is_still_masked() {
        // Showing an unrecognised value in full on the login screen is the one behaviour with
        // no argument for it.
        assert_eq!(mask_email("not-an-email"), "not-****");
        assert_eq!(mask_email(""), "****");
    }

    #[test]
    fn masking_does_not_split_a_multi_byte_character() {
        // `.chars().take()` rather than a byte slice, which would panic on this input.
        assert_eq!(mask_email("ééééé@example.com"), "éééé****@example.com");
    }

    #[test]
    fn an_account_with_an_email_offers_a_masked_one_and_one_without_offers_none() {
        let s = store();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        assert_eq!(s.get_account("player_one").unwrap().unwrap().masked_email(), None);

        s.set_email("player_one", Some("wispplayer@example.com")).unwrap();
        assert_eq!(
            s.get_account("player_one").unwrap().unwrap().masked_email().as_deref(),
            Some("wisp****@example.com")
        );
    }

    #[test]
    fn the_masked_form_never_contains_the_whole_address() {
        // The property rather than one example: whatever the local part was, it must not
        // survive into what the screen shows.
        for addr in ["wispplayer@example.com", "someone.long@example.co.uk"] {
            let masked = mask_email(addr);
            assert!(!masked.contains(addr), "{addr} -> {masked}");
            assert!(masked.contains('*'), "{addr} -> {masked}");
        }
    }

    #[test]
    fn setting_an_email_on_a_missing_account_is_an_error() {
        let s = store();
        assert!(matches!(
            s.set_email("nobody_here", Some("a@b.test")),
            Err(StoreError::NoSuchAccount { .. })
        ));
    }
}
