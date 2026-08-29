//! Local authentication service.
//!
//! Stands in for Nexon Passport: our launcher logs in here, receives a short-lived
//! session token, and hands that token to the game. The login server later validates
//! the same token, so both sides agree on who the player is.
//!
//! Deliberately **binds to localhost only** — this is a single-machine test server and
//! has no business listening on a network interface.

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use store::{AuthOutcome, Store};

pub mod http;

/// Default port. Arbitrary, just not one the game itself uses.
pub const DEFAULT_PORT: u16 = 8080;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LoginResponse {
    Ok {
        account_id: i64,
        username: String,
        token: String,
        expires_in: i64,
    },
    /// One response for both a bad name and a bad password, so the API cannot be used
    /// to discover which accounts exist.
    InvalidCredentials,
    Disabled,
}

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum VerifyResponse {
    Ok { account_id: i64, username: String },
    Invalid,
}

/// Shared service state.
pub struct AuthService {
    store: Arc<Store>,
}

impl AuthService {
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }

    /// Authenticate, issue a token, and **stake the login claim**.
    ///
    /// The owner, 2026-08-29: *"everyone installs this differently, on a client machine you won't
    /// have access to the project or the database."*
    ///
    /// That is why the claim is staked HERE rather than in the launcher. The launcher used to
    /// open `maplecw.db` directly, which works on the one machine that has it and cannot work
    /// anywhere else - a client machine has no repo, no database and no server binaries. The
    /// only thing it can reach is this service, so this service has to do both halves: check
    /// the password, and record who is playing.
    ///
    /// **The identity may be an account name or an email.** `authenticate_identity` resolves
    /// either, and routes an unresolvable one through the same dummy argon2id verify so a bad
    /// address does not answer faster than a bad password.
    ///
    /// A failed claim does **not** fail the login. The password really was right, and telling
    /// someone their credentials are wrong because a table would not write is a lie about the
    /// thing they can act on. It is logged and the token is still issued.
    pub fn login(&self, req: &LoginRequest) -> LoginResponse {
        match self.store.authenticate_identity(&req.username, &req.password) {
            Ok(AuthOutcome::Ok { account_id, token }) => {
                if let Err(e) = self.store.stake_login_claim(
                    account_id,
                    &token,
                    store::LOGIN_CLAIM_TTL_SECS,
                ) {
                    eprintln!(
                        "auth: signed in account {account_id} but could NOT stake the login \
                         claim: {e} - the game will be served as the fallback --account"
                    );
                }
                LoginResponse::Ok {
                    account_id,
                    username: req.username.clone(),
                    token,
                    expires_in: store::SESSION_TTL_SECS,
                }
            }
            Ok(AuthOutcome::Disabled) => LoginResponse::Disabled,
            Ok(AuthOutcome::InvalidCredentials) => LoginResponse::InvalidCredentials,
            Err(e) => {
                // Log the cause for us; tell the caller nothing beyond "no".
                eprintln!("auth: login error for {:?}: {e}", req.username);
                LoginResponse::InvalidCredentials
            }
        }
    }

    /// Check a token without spending it. Used for diagnostics.
    pub fn verify(&self, token: &str) -> VerifyResponse {
        match self.store.validate_session(token) {
            Ok(Some(acc)) => VerifyResponse::Ok {
                account_id: acc.id,
                username: acc.name,
            },
            Ok(None) => VerifyResponse::Invalid,
            Err(e) => {
                eprintln!("auth: verify error: {e}");
                VerifyResponse::Invalid
            }
        }
    }

    /// Spend a token. The login server calls this so a captured token cannot be
    /// replayed.
    pub fn consume(&self, token: &str) -> VerifyResponse {
        match self.store.consume_session(token) {
            Ok(Some(acc)) => VerifyResponse::Ok {
                account_id: acc.id,
                username: acc.name,
            },
            Ok(None) => VerifyResponse::Invalid,
            Err(e) => {
                eprintln!("auth: consume error: {e}");
                VerifyResponse::Invalid
            }
        }
    }

    pub fn store(&self) -> &Store {
        &self.store
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> AuthService {
        let s = Store::open_in_memory().unwrap();
        s.create_account("player_one", "hunter2hunter2").unwrap();
        AuthService::new(Arc::new(s))
    }

    #[test]
    fn login_succeeds_and_issues_a_token() {
        let svc = service();
        let resp = svc.login(&LoginRequest {
            username: "player_one".into(),
            password: "hunter2hunter2".into(),
        });
        match resp {
            LoginResponse::Ok { token, username, .. } => {
                assert_eq!(username, "player_one");
                assert!(!token.is_empty());
            }
            other => panic!("expected Ok, got {other:?}"),
        }
    }

    #[test]
    fn bad_password_and_unknown_user_are_indistinguishable() {
        let svc = service();
        let a = svc.login(&LoginRequest {
            username: "player_one".into(),
            password: "nope-nope-nope".into(),
        });
        let b = svc.login(&LoginRequest {
            username: "ghost_account".into(),
            password: "nope-nope-nope".into(),
        });
        assert!(matches!(a, LoginResponse::InvalidCredentials));
        assert!(matches!(b, LoginResponse::InvalidCredentials));
        // Serialised form must match too, or the difference leaks over the wire.
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            serde_json::to_string(&b).unwrap()
        );
    }

    #[test]
    fn token_verifies_then_is_consumed_once() {
        let svc = service();
        let LoginResponse::Ok { token, .. } = svc.login(&LoginRequest {
            username: "player_one".into(),
            password: "hunter2hunter2".into(),
        }) else {
            panic!("login failed");
        };
        assert!(matches!(svc.verify(&token), VerifyResponse::Ok { .. }));
        assert!(matches!(svc.consume(&token), VerifyResponse::Ok { .. }));
        assert!(matches!(svc.consume(&token), VerifyResponse::Invalid));
    }

    #[test]
    fn login_response_never_serialises_the_password() {
        let svc = service();
        let resp = svc.login(&LoginRequest {
            username: "player_one".into(),
            password: "hunter2hunter2".into(),
        });
        let json = serde_json::to_string(&resp).unwrap();
        assert!(!json.contains("hunter2hunter2"), "password leaked into {json}");
    }
}
