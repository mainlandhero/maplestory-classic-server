//! Signing in, and what that does and does not mean.
//!
//! **The game socket carries no credentials.** The client never sends a user name on it, so
//! nothing the launcher does here authenticates the client. What a successful sign-in buys is
//! narrower and worth stating exactly, because the UI says it out loud:
//!
//! * the password really is verified, against an argon2id hash in the database;
//! * on success the store stakes a **login claim** - a row saying "serve the next game
//!   connection as this account", which the login server reads per connection.
//!
//! So this selects *which account* the next connection is served as. It does not make the
//! connection authenticated, and no amount of work in this crate could: `docs/launcher.md`
//! records the measurement that closed the other route - launch arguments from the third
//! onward do reach the client's session array at `+0x90`, and **nothing puts them on the
//! wire**. Six distinguishable tokens were passed and the outbound `0x0073` came back
//! byte-identical to a run without them. Passing them also broke the run with a "trouble
//! connecting" dialog, which is why [`crate::app`] passes exactly three arguments.

use std::path::{Path, PathBuf};

use store::{AuthOutcome, Store, LOGIN_CLAIM_TTL_SECS};

/// The result of a sign-in attempt, in the shape the UI needs.
///
/// The session token is deliberately **not** carried out of this module. It is a secret, it
/// has already been staked into the claim the login server reads, and the UI has no use for
/// it - so it is dropped where it is created rather than trusted to every later line of code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignIn {
    Ok {
        account_id: i64,
        identity: String,
        ttl_secs: i64,
    },
    /// Wrong password, or no such account. **One outcome, on purpose.**
    BadCredentials,
    Disabled,
    /// The database file is not there. Distinct from every other failure because it is the
    /// one with a fix the user can carry out.
    NoDatabase(PathBuf),
    /// Anything else: the store said so.
    Failed(String),
}

impl SignIn {
    pub fn is_ok(&self) -> bool {
        matches!(self, SignIn::Ok { .. })
    }

    /// The line shown under the buttons.
    pub fn message(&self) -> String {
        match self {
            SignIn::Ok { account_id, identity, ttl_secs } => format!(
                "signed in as {identity} (account {account_id}). The next game connection will \
                 be served as this account; the claim lasts {}.",
                human_duration(*ttl_secs)
            ),
            // `store::AuthOutcome::InvalidCredentials` does not distinguish a bad name from a
            // bad password - telling them apart lets an attacker enumerate account names -
            // so this must not invent a distinction the store refuses to make.
            SignIn::BadCredentials => {
                "that email/account name and password do not match an account".into()
            }
            SignIn::Disabled => "that account is disabled".into(),
            SignIn::NoDatabase(p) => format!(
                "no database at {} - create an account first with maplecw-useradd",
                p.display()
            ),
            SignIn::Failed(e) => format!("sign-in failed: {e}"),
        }
    }
}

/// `43200` is a true answer to "how long does the claim last" and a useless one on screen.
fn human_duration(secs: i64) -> String {
    match secs {
        s if s <= 0 => "no time at all".to_string(),
        s if s % 3600 == 0 && s >= 3600 => {
            let h = s / 3600;
            format!("{h} hour{}", if h == 1 { "" } else { "s" })
        }
        s if s % 60 == 0 && s >= 60 => {
            let m = s / 60;
            format!("{m} minute{}", if m == 1 { "" } else { "s" })
        }
        s => format!("{s} seconds"),
    }
}

/// Verify a password and stake the login claim.
///
/// Slow on purpose: argon2id is the point. Call it off the UI thread.
pub fn sign_in(db_path: &Path, identity: &str, password: &str) -> SignIn {
    // **Check the file before opening it.** `Store::open` creates a database that is not
    // there, so without this a mistyped path would produce an empty database and every
    // sign-in would come back "wrong password" - which is exactly the failure this project
    // keeps writing down: a clean, confident answer from an instrument pointed at nothing.
    if !db_path.is_file() {
        return SignIn::NoDatabase(db_path.to_path_buf());
    }

    let store = match Store::open(db_path) {
        Ok(s) => s,
        Err(e) => return SignIn::Failed(format!("could not open {}: {e}", db_path.display())),
    };

    let outcome = match store.authenticate_identity(identity, password) {
        Ok(o) => o,
        Err(e) => return SignIn::Failed(e.to_string()),
    };

    match outcome {
        AuthOutcome::InvalidCredentials => SignIn::BadCredentials,
        AuthOutcome::Disabled => SignIn::Disabled,
        AuthOutcome::Ok { account_id, token } => {
            match store.stake_login_claim(account_id, &token, LOGIN_CLAIM_TTL_SECS) {
                // The claim itself is not carried out of here: nothing in the UI needs it,
                // and it is derived from the token.
                Ok(_claim) => SignIn::Ok {
                    account_id,
                    identity: identity.to_string(),
                    ttl_secs: LOGIN_CLAIM_TTL_SECS,
                },
                Err(e) => SignIn::Failed(format!("could not stake the login claim: {e}")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::TempDir;

    #[test]
    fn a_missing_database_is_reported_as_such_and_never_created() {
        // `Store::open` would happily create one, and then every password would be wrong for
        // a reason nobody could see.
        let t = TempDir::new("nodb");
        let db = t.path().join("maplecw.db");
        let outcome = sign_in(&db, "wisp", "hunter2");
        assert_eq!(outcome, SignIn::NoDatabase(db.clone()));
        assert!(!db.exists(), "sign_in created a database that was not there");
    }

    #[test]
    fn the_four_failures_do_not_read_alike() {
        let messages = [
            SignIn::BadCredentials.message(),
            SignIn::Disabled.message(),
            SignIn::NoDatabase(PathBuf::from(r"C:\x\maplecw.db")).message(),
            SignIn::Failed("disk is on fire".into()).message(),
        ];
        let unique: std::collections::HashSet<_> = messages.iter().collect();
        assert_eq!(unique.len(), 4, "{messages:?}");
        assert!(messages[2].contains(r"C:\x\maplecw.db"), "{}", messages[2]);
        assert!(messages[3].contains("disk is on fire"), "{}", messages[3]);
    }

    #[test]
    fn bad_credentials_does_not_claim_to_know_which_half_was_wrong() {
        let msg = SignIn::BadCredentials.message();
        let lower = msg.to_ascii_lowercase();
        assert!(
            !lower.contains("no such account") && !lower.contains("wrong password"),
            "the store refuses to distinguish these; the UI must not invent it: {msg}"
        );
    }

    #[test]
    fn the_success_line_names_the_account_and_says_what_the_claim_does() {
        let msg = SignIn::Ok {
            account_id: 7,
            identity: "wisp@example.com".into(),
            ttl_secs: 120,
        }
        .message();
        assert!(msg.contains("wisp@example.com"), "{msg}");
        assert!(msg.contains('7'), "{msg}");
        assert!(msg.contains("2 minutes"), "{msg}");
        // It has to say what the claim actually does, or "signed in" reads as "authenticated".
        assert!(msg.contains("served as this account"), "{msg}");
    }

    #[test]
    fn the_success_line_does_not_carry_a_token() {
        // The token is a secret and this type deliberately has nowhere to put one.
        let msg = SignIn::Ok {
            account_id: 1,
            identity: "wisp".into(),
            ttl_secs: LOGIN_CLAIM_TTL_SECS,
        }
        .message();
        assert!(!msg.to_ascii_lowercase().contains("token"), "{msg}");
    }

    // ---- against a real database ----
    //
    // Everything above this line is about shapes and wording. These drive the actual store:
    // argon2id, the claim row, and the account the login server would then resolve. Without
    // them the whole sign-in path is untested end to end, which is the first thing anyone
    // touches and the last thing that gets covered.

    fn db_with_two_accounts(dir: &Path) -> PathBuf {
        let path = dir.join("maplecw.db");
        let store = Store::open(&path).expect("open a fresh database");
        store.create_account("player_one", "correct horse battery").unwrap();
        store.create_account("player_two", "correct horse battery").unwrap();
        store.set_email("player_one", Some("wisp@example.test")).unwrap();
        path
    }

    #[test]
    fn signing_in_stakes_a_claim_the_login_server_would_resolve() {
        let t = TempDir::new("signin-ok");
        let db = db_with_two_accounts(t.path());

        let out = sign_in(&db, "player_two", "correct horse battery");
        let SignIn::Ok { account_id, .. } = out else {
            panic!("expected a successful sign-in, got {out:?}");
        };

        // The claim is the whole point: it is what `login::server::resolve_account` reads.
        let store = Store::open(&db).unwrap();
        let claim = store.current_login_claim().unwrap().expect("a claim was staked");
        assert_eq!(claim.account_id, account_id);
        assert_eq!(claim.account_name, "player_two");
    }

    #[test]
    fn signing_in_by_email_works_and_names_the_account_that_owns_it() {
        let t = TempDir::new("signin-email");
        let db = db_with_two_accounts(t.path());

        let out = sign_in(&db, "wisp@example.test", "correct horse battery");
        assert!(out.is_ok(), "{out:?}");

        let store = Store::open(&db).unwrap();
        let claim = store.current_login_claim().unwrap().unwrap();
        assert_eq!(claim.account_name, "player_one");
    }

    #[test]
    fn signing_in_again_as_someone_else_moves_the_claim() {
        // This is the feature the owner asked for, in one test: more than one account on one
        // machine, swapped without restarting anything.
        let t = TempDir::new("signin-swap");
        let db = db_with_two_accounts(t.path());

        assert!(sign_in(&db, "player_one", "correct horse battery").is_ok());
        assert!(sign_in(&db, "player_two", "correct horse battery").is_ok());

        let store = Store::open(&db).unwrap();
        assert_eq!(
            store.current_login_claim().unwrap().unwrap().account_name,
            "player_two"
        );
    }

    #[test]
    fn a_wrong_password_stakes_nothing() {
        let t = TempDir::new("signin-bad");
        let db = db_with_two_accounts(t.path());

        assert_eq!(sign_in(&db, "player_one", "not-the-password"), SignIn::BadCredentials);

        let store = Store::open(&db).unwrap();
        assert!(
            store.current_login_claim().unwrap().is_none(),
            "a refused sign-in must not decide who plays"
        );
    }

    #[test]
    fn an_unknown_identity_is_refused_without_saying_it_is_unknown() {
        let t = TempDir::new("signin-unknown");
        let db = db_with_two_accounts(t.path());
        // Same variant as a wrong password, so the UI cannot leak which accounts exist.
        assert_eq!(sign_in(&db, "nobody@example.test", "whatever"), SignIn::BadCredentials);
    }

    #[test]
    fn a_disabled_account_is_refused_and_says_so() {
        let t = TempDir::new("signin-disabled");
        let db = db_with_two_accounts(t.path());
        Store::open(&db).unwrap().set_enabled("player_one", false).unwrap();

        assert_eq!(sign_in(&db, "player_one", "correct horse battery"), SignIn::Disabled);
        assert!(Store::open(&db).unwrap().current_login_claim().unwrap().is_none());
    }

    #[test]
    fn the_claim_survives_being_read_twice() {
        // The log-out reconnect, from the launcher's side. `CLAUDE.md` records two 0x0010s
        // in one launch; if reading consumed the claim the second connection would land on
        // a different account and it would read as characters going missing.
        let t = TempDir::new("signin-twice");
        let db = db_with_two_accounts(t.path());
        assert!(sign_in(&db, "player_one", "correct horse battery").is_ok());

        let store = Store::open(&db).unwrap();
        let first = store.current_login_claim().unwrap().unwrap();
        let second = store.current_login_claim().unwrap().unwrap();
        assert_eq!(first.account_name, second.account_name);
    }

    #[test]
    fn the_claim_lifetime_reads_as_a_duration_not_as_43200() {
        assert_eq!(human_duration(LOGIN_CLAIM_TTL_SECS), "12 hours");
        assert_eq!(human_duration(3600), "1 hour");
        assert_eq!(human_duration(120), "2 minutes");
        assert_eq!(human_duration(60), "1 minute");
        assert_eq!(human_duration(90), "90 seconds");
        assert_eq!(human_duration(0), "no time at all");
        assert_eq!(human_duration(-5), "no time at all");
    }
}
