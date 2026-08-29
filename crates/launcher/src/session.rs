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

use crate::http::{self, AuthReply};

/// The result of a sign-in attempt, in the shape the UI needs.
///
/// The session token is deliberately **not** carried out of this module - in fact it never
/// reaches this machine at all now. The auth service stakes the login claim itself, because
/// the launcher on a client machine cannot: there is no database there to write to.
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
    /// The sign-in service could not be reached, or did not answer usefully. Distinct from a
    /// refusal because it is the one with a fix the person can carry out.
    Unreachable(String),
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
            // bad password - telling them apart lets an attacker enumerate account names - so
            // this must not invent a distinction the service refuses to make.
            SignIn::BadCredentials => {
                "that email or account name and password do not match. (The server will not \
                 say which of the two was wrong.)"
                    .to_string()
            }
            SignIn::Disabled => "that account is disabled.".to_string(),
            SignIn::Unreachable(why) => why.clone(),
        }
    }
}

/// Sign in against the server's auth service.
///
/// **Over the network, always - there is no local-database shortcut.** The owner, 2026-08-29:
/// *"everyone installs this differently, on a client machine you won't have access to the
/// project or the database."* Two auth paths would mean the dev box exercised the file and
/// every other machine the socket, and the socket would be the one nobody tried.
///
/// `crate::http` carries the warning that matters: the password crosses the wire in plain
/// text, which is acceptable for a test server on a network you control and nothing else.
pub fn sign_in(host: &str, auth_port: u16, identity: &str, password: &str) -> SignIn {
    if identity.trim().is_empty() {
        return SignIn::BadCredentials;
    }
    match http::login(host, auth_port, identity, password) {
        AuthReply::Ok { account_id, .. } => SignIn::Ok {
            account_id,
            identity: identity.to_string(),
            ttl_secs: LOGIN_CLAIM_TTL_SECS,
        },
        AuthReply::InvalidCredentials => SignIn::BadCredentials,
        AuthReply::Disabled => SignIn::Disabled,
        AuthReply::Failed(why) => SignIn::Unreachable(why),
    }
}

/// The claim lifetime, for the message only.
///
/// Duplicated from `store` rather than depended on: the launcher no longer needs the store at
/// all, and keeping a database crate in a client-machine binary to read one integer would be
/// carrying rusqlite onto machines that have no database. If it ever drifts the message is
/// slightly wrong, which is the cheapest possible failure here.
const LOGIN_CLAIM_TTL_SECS: i64 = 12 * 3600;

/// `43200` reads badly on screen. This says "12 hours".
///
/// Whole units only where they divide exactly, so "90 seconds" stays 90 seconds rather than
/// becoming "1 minute" and quietly losing half of itself.
fn human_duration(secs: i64) -> String {
    if secs <= 0 {
        return "no time at all".to_string();
    }
    let plural = |n: i64, unit: &str| {
        if n == 1 { format!("1 {unit}") } else { format!("{n} {unit}s") }
    };
    if secs % 3600 == 0 {
        plural(secs / 3600, "hour")
    } else if secs % 60 == 0 {
        plural(secs / 60, "minute")
    } else {
        plural(secs, "second")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_identity_is_refused_without_touching_the_network() {
        // Not a round trip: an empty box is a mistake, and making the server say so would
        // cost an argon2id verify and a message that reads as if the server were at fault.
        assert_eq!(sign_in("127.0.0.1", 1, "   ", "whatever"), SignIn::BadCredentials);
    }

    #[test]
    fn a_success_names_the_account_and_the_claim_lifetime() {
        let msg = SignIn::Ok {
            account_id: 2,
            identity: "tester@example.test".into(),
            ttl_secs: LOGIN_CLAIM_TTL_SECS,
        }
        .message();
        assert!(msg.contains("tester@example.test"), "{msg}");
        assert!(msg.contains("account 2"), "{msg}");
        assert!(msg.contains("12 hours"), "43200 reads badly on screen: {msg}");
    }

    /// The refusal must not invent a distinction the server refuses to make. Telling a bad
    /// name from a bad password is an account-enumeration oracle, which is why
    /// `AuthOutcome::InvalidCredentials` is one variant on the server side too.
    #[test]
    fn a_refusal_does_not_say_which_half_was_wrong() {
        let msg = SignIn::BadCredentials.message();
        let lower = msg.to_ascii_lowercase();
        assert!(lower.contains("do not match"), "{msg}");
        assert!(!lower.contains("no such account"), "{msg}");
        assert!(!lower.contains("wrong password"), "{msg}");
    }

    #[test]
    fn an_unreachable_server_shows_the_reason_it_was_given() {
        let msg = SignIn::Unreachable("could not reach the sign-in service".into()).message();
        assert!(msg.contains("could not reach"), "{msg}");
    }

    #[test]
    fn no_message_ever_mentions_a_token() {
        // The token never reaches this machine now - the service stakes the claim itself -
        // but the assertion stays, because a secret that cannot be printed is worth pinning.
        for s in [
            SignIn::Ok { account_id: 1, identity: "wisp".into(), ttl_secs: 3600 },
            SignIn::BadCredentials,
            SignIn::Disabled,
            SignIn::Unreachable("nope".into()),
        ] {
            assert!(!s.message().to_ascii_lowercase().contains("token"), "{:?}", s);
        }
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
