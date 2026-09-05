//! **Registering an account and recovering a password, each behind a single-use code.**
//!
//! The owner, 2026-09-05: *"There should be a !registrationcode and !recoverycode <email>/<username>
//! command ingame, which outputs a 1 time use 8 character alphanumeric code (uppercase) to
//! allow clients to either register an account with us or set a new password for an existing
//! account. ... Registration takes a username, email and password, ensures that passwords are
//! of at least 8 characters with numbers and letters. Recovery requires either the username or
//! email of the account, as well as the admin generated registration code."*
//!
//! The codes come from `store::codes`; a GM mints them in game (`!registrationcode`,
//! `!recoverycode`) or an administrator does with `maplecw-useradd`. This module is the
//! service end: it is what the launcher's Register and Forgot-password screens talk to.
//!
//! # Everything that can be checked is checked BEFORE the code is spent
//!
//! `store::codes` is explicit that a redeem is atomic with respect to its own row and nothing
//! else: once it returns `true` the code is burnt, whatever happens next. So the order here is
//! the order of least regret. The username's shape, the email's shape, the password policy,
//! and whether the name or email is already taken are all decided first, against nothing but
//! the request; only then is the code redeemed, and only then is the account created. A typo
//! costs a second try, not a second code. The one thing that cannot be checked first is the
//! code itself, because checking it *is* spending it.
//!
//! What remains: the account creation can still fail after the redeem (the name was taken in
//! the instant between the check and the insert). That is reported as `Failed` with a sentence
//! that says the code is spent, which is the honest shape of it.
//!
//! # Recovery never burns a code against the wrong account
//!
//! A recovery code names an account. The request names one too, by email or username. If they
//! disagree - a typo in the identity - the code must survive, so the redeem is
//! `Store::redeem_recovery_code_for`, whose `WHERE` clause includes the account: a code for
//! somebody else is simply a code that does not match, and it is still live afterwards. An
//! identity that names no account is answered the same way as a wrong code, so this endpoint
//! is not a way to find out which emails are registered.
//!
//! # The failure budget
//!
//! Every refused code counts against `crate::ratelimit`, per peer and overall. The codes are
//! eight characters now, and that limiter is what makes eight enough - its module doc has the
//! arithmetic.

use serde::{Deserialize, Serialize};
use store::{Store, StoreError};

use crate::AuthService;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    /// The registration (invite) code a GM minted. Hyphens, spaces and case are irrelevant.
    pub code: String,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RegisterResponse {
    Ok { account_id: i64, username: String },
    /// Unknown, expired or already used. One answer for all three, on purpose.
    InvalidCode,
    UsernameTaken,
    InvalidUsername { message: String },
    InvalidEmail { message: String },
    WeakPassword { message: String },
    /// This peer, or the service as a whole, has failed too many codes recently.
    TooManyAttempts,
    /// Something after the checks went wrong. The message says whether the code was spent.
    Failed { message: String },
}

impl RegisterResponse {
    pub fn http_status(&self) -> u16 {
        match self {
            RegisterResponse::Ok { .. } => 200,
            RegisterResponse::InvalidUsername { .. }
            | RegisterResponse::InvalidEmail { .. }
            | RegisterResponse::WeakPassword { .. } => 400,
            RegisterResponse::InvalidCode => 401,
            RegisterResponse::UsernameTaken => 409,
            RegisterResponse::TooManyAttempts => 429,
            RegisterResponse::Failed { .. } => 500,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RecoverRequest {
    /// The account's username or email.
    pub identity: String,
    /// The recovery code a GM minted for that account.
    pub code: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum RecoverResponse {
    Ok { username: String },
    /// Unknown, expired, used, or minted for a different account than the identity names.
    /// Also the answer when the identity names no account at all.
    InvalidCode,
    WeakPassword { message: String },
    TooManyAttempts,
    Failed { message: String },
}

impl RecoverResponse {
    pub fn http_status(&self) -> u16 {
        match self {
            RecoverResponse::Ok { .. } => 200,
            RecoverResponse::WeakPassword { .. } => 400,
            RecoverResponse::InvalidCode => 401,
            RecoverResponse::TooManyAttempts => 429,
            RecoverResponse::Failed { .. } => 500,
        }
    }
}

/// The shape of an email address, as far as is worth checking without sending one.
///
/// One `@`, something on each side, a dot in the domain, no whitespace. Deliberately not the
/// RFC: this is a second sign-in identity for a private server, and the cost of refusing an
/// exotic-but-valid address is a support message, while the cost of accepting `wisp` as an
/// email is a name that collides with `Store::validate_name`'s namespace - which is the one
/// thing this must never allow, because the sign-in field takes either.
pub fn validate_email(email: &str) -> Result<(), String> {
    let e = email.trim();
    if e.is_empty() {
        return Err("an email address is required".into());
    }
    if e.len() > 254 {
        return Err("that email address is too long".into());
    }
    if e.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err("an email address cannot contain spaces".into());
    }
    let Some((local, domain)) = e.split_once('@') else {
        return Err("an email address needs an @".into());
    };
    if local.is_empty() || domain.is_empty() || domain.contains('@') {
        return Err("that does not look like an email address".into());
    }
    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return Err("the part after the @ needs a domain like example.com".into());
    }
    Ok(())
}

impl AuthService {
    /// Create an account against a registration code. See the module doc for the order.
    pub fn register(&self, req: &RegisterRequest, peer: Option<&str>) -> RegisterResponse {
        let peer_key = peer.unwrap_or("?");
        if !self.codes.allowed(peer_key) {
            return RegisterResponse::TooManyAttempts;
        }
        let username = req.username.trim();
        let email = req.email.trim();

        // ---- everything that can be decided without touching the code
        match Store::validate_name(username) {
            Ok(()) => {}
            Err(StoreError::InvalidAccountName { reason, .. }) => {
                return RegisterResponse::InvalidUsername { message: reason.to_string() }
            }
            Err(e) => return RegisterResponse::Failed { message: e.to_string() },
        }
        if let Err(message) = validate_email(email) {
            return RegisterResponse::InvalidEmail { message };
        }
        if let Err(message) = store::check_password_policy(&req.password) {
            return RegisterResponse::WeakPassword { message };
        }
        match self.store.get_account_by_identity(username) {
            Ok(Some(_)) => return RegisterResponse::UsernameTaken,
            Ok(None) => {}
            Err(e) => return RegisterResponse::Failed { message: e.to_string() },
        }
        match self.store.get_account_by_identity(email) {
            Ok(Some(_)) => {
                return RegisterResponse::InvalidEmail {
                    message: "that email address is already registered to an account".into(),
                }
            }
            Ok(None) => {}
            Err(e) => return RegisterResponse::Failed { message: e.to_string() },
        }

        // ---- the code. This is the point of no return.
        match self.store.redeem_invite_code(&req.code) {
            Ok(true) => {}
            Ok(false) => {
                self.codes.failed(peer_key);
                return RegisterResponse::InvalidCode;
            }
            Err(e) => return RegisterResponse::Failed { message: e.to_string() },
        }

        let account_id = match self.store.create_account(username, &req.password) {
            Ok(id) => id,
            Err(StoreError::AccountExists { .. }) => {
                return RegisterResponse::Failed {
                    message: "the code was accepted, but that username was taken a moment ago. \
                              The code is spent - ask for another one."
                        .into(),
                }
            }
            Err(e) => {
                return RegisterResponse::Failed {
                    message: format!("the code was accepted but the account could not be created: {e}. The code is spent - ask for another one."),
                }
            }
        };
        // The account exists whatever happens to the email; a refused address must not read
        // as a failed registration. It is logged and the person can sign in by name.
        if let Err(e) = self.store.set_email(username, Some(email)) {
            eprintln!("auth: registered {username:?} (id {account_id}) but could not set its email: {e}");
        }
        println!("auth: registered account {username:?} (id {account_id})");
        RegisterResponse::Ok { account_id, username: username.to_string() }
    }

    /// Set a new password against a recovery code minted for that account.
    pub fn recover(&self, req: &RecoverRequest, peer: Option<&str>) -> RecoverResponse {
        let peer_key = peer.unwrap_or("?");
        if !self.codes.allowed(peer_key) {
            return RecoverResponse::TooManyAttempts;
        }
        if let Err(message) = store::check_password_policy(&req.new_password) {
            return RecoverResponse::WeakPassword { message };
        }
        let account = match self.store.get_account_by_identity(req.identity.trim()) {
            Ok(Some(acc)) => acc,
            Ok(None) => {
                // Indistinguishable from a wrong code, and counted like one: this must not be
                // a way to learn which emails are registered.
                self.codes.failed(peer_key);
                return RecoverResponse::InvalidCode;
            }
            Err(e) => return RecoverResponse::Failed { message: e.to_string() },
        };
        match self.store.redeem_recovery_code_for(&req.code, account.id) {
            Ok(true) => {}
            Ok(false) => {
                self.codes.failed(peer_key);
                return RecoverResponse::InvalidCode;
            }
            Err(e) => return RecoverResponse::Failed { message: e.to_string() },
        }
        if let Err(e) = self.store.set_password(&account.name, &req.new_password) {
            return RecoverResponse::Failed {
                message: format!("the code was accepted but the password could not be set: {e}. The code is spent - ask for another one."),
            };
        }
        println!("auth: password reset for account {:?} (id {}) by recovery code", account.name, account.id);
        RecoverResponse::Ok { username: account.name }
    }
}

#[cfg(test)]
mod email_tests {
    use super::validate_email;

    #[test]
    fn plausible_addresses_pass_and_the_usual_mistakes_are_named() {
        for ok in ["wisp@example.com", "a.b+c@sub.example.co.uk", "X@Y.Z"] {
            assert!(validate_email(ok).is_ok(), "{ok}");
        }
        assert!(validate_email("").unwrap_err().contains("required"));
        assert!(validate_email("wisp").unwrap_err().contains('@'));
        assert!(validate_email("wisp@").unwrap_err().contains("look like"));
        assert!(validate_email("@example.com").unwrap_err().contains("look like"));
        assert!(validate_email("wisp@localhost").unwrap_err().contains("domain"));
        assert!(validate_email("wisp @example.com").unwrap_err().contains("spaces"));
        assert!(validate_email("a@b@c.com").unwrap_err().contains("look like"));
    }

    #[test]
    fn an_account_name_can_never_pass_as_an_email() {
        // The property the sign-in field depends on: the two namespaces cannot collide.
        for name in ["maplecw", "tester_2", "abc"] {
            assert!(validate_email(name).is_err(), "{name}");
        }
    }
}
