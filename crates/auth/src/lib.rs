//! Local authentication service.
//!
//! Stands in for Nexon Passport: our launcher logs in here, receives a short-lived
//! session token, and hands that token to the game. The login server later validates
//! the same token, so both sides agree on who the player is.
//!
//! Defaults to a loopback bind; an installed server box passes `--bind 0.0.0.0`. Either way
//! it speaks **TLS 1.3 with its own certificate**, which every launcher pins by fingerprint -
//! `crate::tls` makes the certificate, `crates/tlspin` defines the pin, and `crate::http`
//! says why no HTTP library is underneath.

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use store::{AuthOutcome, Store};

pub mod clientpatch;
pub mod http;
pub mod ratelimit;
pub mod register;
pub mod tls;
#[cfg(test)]
mod register_tests;

pub use register::{RecoverRequest, RecoverResponse, RegisterRequest, RegisterResponse};

/// Default port.
///
/// **8480 since 2026-09-07, and it used to be 8080.** The owner, having found 8080 held by something
/// else on their own machine: *"Can we also make the sign-in port default on both the server and
/// client payloads? So the server doesn't have to change anything in start-servers and neither
/// do the clients in the launcher."* 8080 is one of the most contended ports on a developer
/// machine - proxies, dev servers, IIS Express - and every collision cost the same two manual
/// steps, one on the server and one on every client, with a failure in between that reads as
/// *"received corrupt message of type InvalidContentType"* rather than *"that port is taken"*.
///
/// 8480 sits beside the game's 8484-8486 without colliding, so one glance at a firewall rule
/// covers all of them.
///
/// **`launcher::paths::DEFAULT_AUTH_PORT` must equal this**, and a test in the launcher asserts
/// it against this constant rather than against a literal. The two sides disagreeing is not a
/// hypothetical: it is exactly what a client sees when only one of them is changed.
pub const DEFAULT_PORT: u16 = 8480;

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
        /// **The handle the launcher uses to register the client's process id.**
        ///
        /// Not a credential and not the session token - see [`store::StakedClaim`]. Its only
        /// power is `POST /launch`, and that is what makes two launchers on one machine two
        /// distinguishable claims rather than one evicting the other.
        ///
        /// Empty when the claim could not be staked, which is the same case the log line
        /// below names. An empty string here means the launch cannot be registered and the
        /// connection will be resolved by address or not at all; the launcher says so.
        launch_id: String,
        /// **The one-time token the CLIENT is meant to carry**, 26 characters of uppercase
        /// base32 (`store::claims::CLIENT_TOKEN_CHARS`).
        ///
        /// This is the field the launcher writes where the hook can read it - the repo already
        /// writes `maplecw-hook.session` / `.probe` / `.dumpdir` beside the client for exactly
        /// this kind of hand-off. The hook writes it into the client's session object
        /// (`session+0x1b8`), and the client puts it in `0x0073` itself.
        ///
        /// The owner, 2026-08-29: *"using the client to pass a session should be what we aim for
        /// instead of inference."* Every other field the login server resolves by is an
        /// inference about a socket; this one is a claim the client makes and the server
        /// checks.
        ///
        /// **It authenticates the login socket only.** `0x0073` has never appeared on a
        /// channel connection - 103 archived files, all port 8484. `CLAUDE.md`'s standing
        /// constraint about the game socket is untouched.
        ///
        /// Empty for the same reason `launch_id` is: the claim could not be staked. An empty
        /// string means there is no credential to write and the launcher should say so rather
        /// than writing a blank file - a blank identity is indistinguishable on the wire from
        /// a stock client, which is exactly the ambiguity this exists to remove.
        ///
        /// Spent on first presentation. Pressing Login again mints a new one and kills this.
        client_token: String,
    },
    /// One response for both a bad name and a bad password, so the API cannot be used
    /// to discover which accounts exist.
    InvalidCredentials,
    Disabled,
}

/// The answer to `POST /launch`.
///
/// Three outcomes rather than a boolean, because *"the handle named no live claim"* and *"the
/// database would not write"* need different sentences on the launcher's screen and only one
/// of them has a fix the person can carry out.
#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum LaunchResponse {
    /// The process is now bound to the claim. A connection from it resolves to that account.
    Bound { pid: u32 },
    /// The launch id named no live claim: it is stale, the sign-in expired, or Login was
    /// pressed again since. **Not fatal** - the client can still be started, and if this is
    /// the only sign-in on the box it will be served correctly anyway. It is reported because
    /// with a second player signed in it is the difference between playing and falling back.
    UnknownLaunch,
    /// The store refused the write. Distinct from `UnknownLaunch`: nothing is stale, something
    /// is broken.
    Failed { message: String },
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
    /// The failure budget for registration and recovery codes. `crate::ratelimit`.
    codes: ratelimit::Limiter,
    /// The canonical client this server patches launchers up to, when `--client-dir` was
    /// given. `None` means this server publishes no client: the two `/client/*` endpoints
    /// answer 503, and a launcher told to block on an unconfirmed version will say so rather
    /// than guess. `crate::clientpatch`.
    client: Option<Arc<crate::clientpatch::ClientPatchSource>>,
}

/// One timestamped line on stdout - `HH:MM:SS.mmm [peer] what` by convention - which the
/// launch scripts redirect to `auth.log`. The same shape the login and channel servers use,
/// so the three logs read alike.
pub fn log(msg: &str) {
    use std::io::Write;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
    let secs = now.as_secs();
    let (h, m, s) = ((secs / 3600) % 24, (secs / 60) % 60, secs % 60);
    println!("{h:02}:{m:02}:{s:02}.{:03} {msg}", now.subsec_millis());
    let _ = std::io::stdout().flush();
}

impl AuthService {
    pub fn new(store: Arc<Store>) -> Self {
        Self::with_limiter(store, ratelimit::Limiter::default())
    }

    /// [`AuthService::new`] with a chosen code-failure budget, for tests that need a small one.
    pub fn with_limiter(store: Arc<Store>, codes: ratelimit::Limiter) -> Self {
        Self { store, codes, client: None }
    }

    /// Publish a canonical client from this server. See [`crate::clientpatch`].
    pub fn with_client_patches(
        mut self,
        client: Arc<crate::clientpatch::ClientPatchSource>,
    ) -> Self {
        self.client = Some(client);
        self
    }

    /// The canonical client, if this server publishes one.
    pub fn client_patches(&self) -> Option<&Arc<crate::clientpatch::ClientPatchSource>> {
        self.client.as_ref()
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
    ///
    /// # A sign-in no longer evicts anybody else's
    ///
    /// It used to. `store::claims` kept one global row and `stake_login_claim` ran
    /// `DELETE FROM login_claims` before inserting, so a second person signing in through this
    /// endpoint removed the first person's claim **and both login connections were then served
    /// as the second account**. Measured through this method on 2026-08-29:
    ///
    /// ```text
    /// after otter: claim=Some("otter")
    /// after owl:   claim=Some("owl")
    /// login connection 1 served as Some("owl")
    /// login connection 2 served as Some("owl")
    /// ```
    ///
    /// The claim is now per launch, and the `launch_id` handed back here is how the launcher
    /// tells the server which client process belongs to it. `peer` is recorded as a
    /// tie-breaker for two *machines*; it is never the discriminator, because two clients on
    /// one machine share it.
    pub fn login(&self, req: &LoginRequest, peer: Option<&str>) -> LoginResponse {
        let from = peer.unwrap_or("?");
        match self.store.authenticate_identity(&req.username, &req.password) {
            Ok(AuthOutcome::Ok { account_id, token }) => {
                log(&format!("[{from}] sign-in OK: {:?} is account {account_id}", req.username));
                let (launch_id, client_token) = match self.store.stake_login_claim_with(
                    account_id,
                    &token,
                    store::LOGIN_CLAIM_TTL_SECS,
                    peer,
                ) {
                    Ok(staked) => {
                        // Said out loud, because it invalidates a credential somebody may
                        // still be holding: a claim this sign-in replaced belongs to an
                        // earlier launch of the SAME account, whose client token stops
                        // matching from here on. Silence would make a client that was working
                        // a minute ago look like a server fault.
                        if staked.superseded > 0 {
                            eprintln!(
                                "auth: account {account_id} signed in again - {} earlier live \
                                 claim(s) for this account replaced. Any client still holding \
                                 one of those launch credentials will no longer be recognised; \
                                 no other account is affected",
                                staked.superseded
                            );
                        }
                        (staked.launch_id, staked.client_token)
                    }
                    Err(e) => {
                        eprintln!(
                            "auth: signed in account {account_id} but could NOT stake the login \
                             claim: {e} - the game will be served as the fallback --account"
                        );
                        (String::new(), String::new())
                    }
                };
                LoginResponse::Ok {
                    account_id,
                    username: req.username.clone(),
                    token,
                    expires_in: store::SESSION_TTL_SECS,
                    launch_id,
                    client_token,
                }
            }
            Ok(AuthOutcome::Disabled) => {
                log(&format!("[{from}] sign-in REFUSED: {:?} is disabled", req.username));
                LoginResponse::Disabled
            }
            Ok(AuthOutcome::InvalidCredentials) => {
                // The name is logged, the password is not - and the line is the same for an
                // unknown name and a wrong password, so the log does not enumerate accounts.
                log(&format!("[{from}] sign-in REFUSED: bad credentials for {:?}", req.username));
                LoginResponse::InvalidCredentials
            }
            Err(e) => {
                // Log the cause for us; tell the caller nothing beyond "no".
                eprintln!("auth: login error for {:?} from {from}: {e}", req.username);
                LoginResponse::InvalidCredentials
            }
        }
    }

    /// **Bind the client process the launcher just started to the claim it signed in with.**
    ///
    /// This is the whole of what makes several clients on one machine work. The launcher is
    /// the only party that knows the pid, because it started the process; the login server
    /// independently recovers the same number from an accepted socket
    /// (`store::peerowner::owning_pid`) and matches the two. The pid is not a secret and is not
    /// trusted as an assertion - a process cannot make the operating system attribute somebody
    /// else's socket to it.
    ///
    /// Refusals are values rather than logs. `CLAUDE.md`: *"A refusal that is reported to no
    /// one will be ignored eventually"* - the launcher shows this line, so an unregistered
    /// launch is visible before the client is on screen rather than after it turns out to be
    /// playing the wrong account.
    pub fn bind_launch(&self, launch_id: &str, pid: u32) -> LaunchResponse {
        if launch_id.trim().is_empty() || pid == 0 {
            // pid 0 is the System Idle Process and can never own a socket, so it is an
            // unfilled field rather than a value - the same reasoning `migration::random_seed`
            // uses for never minting zero.
            return LaunchResponse::UnknownLaunch;
        }
        match self.store.bind_launch_pid(launch_id, pid) {
            Ok(true) => LaunchResponse::Bound { pid },
            Ok(false) => LaunchResponse::UnknownLaunch,
            Err(e) => {
                eprintln!("auth: could not bind a launch pid: {e}");
                LaunchResponse::Failed { message: "the claim store refused the write".into() }
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
        let resp = svc.login(
            &LoginRequest { username: "player_one".into(), password: "hunter2hunter2".into() },
            Some("127.0.0.1"),
        );
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
        let a = svc.login(
            &LoginRequest { username: "player_one".into(), password: "nope-nope-nope".into() },
            Some("127.0.0.1"),
        );
        let b = svc.login(
            &LoginRequest { username: "ghost_account".into(), password: "nope-nope-nope".into() },
            Some("127.0.0.1"),
        );
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
        let LoginResponse::Ok { token, .. } = svc.login(
            &LoginRequest { username: "player_one".into(), password: "hunter2hunter2".into() },
            None,
        ) else {
            panic!("login failed");
        };
        assert!(matches!(svc.verify(&token), VerifyResponse::Ok { .. }));
        assert!(matches!(svc.consume(&token), VerifyResponse::Ok { .. }));
        assert!(matches!(svc.consume(&token), VerifyResponse::Invalid));
    }

    #[test]
    fn login_response_never_serialises_the_password() {
        let svc = service();
        let resp = svc.login(
            &LoginRequest { username: "player_one".into(), password: "hunter2hunter2".into() },
            Some("127.0.0.1"),
        );
        let json = serde_json::to_string(&resp).unwrap();
        assert!(!json.contains("hunter2hunter2"), "password leaked into {json}");
    }

    // ------------------------------------------------------------------------------------
    // THE EVICTION BUG. Everything below was measured failing through this exact method.
    // ------------------------------------------------------------------------------------

    fn two_player_service() -> AuthService {
        let s = Store::open_in_memory().unwrap();
        s.create_account("otter", "hunter2hunter2").unwrap();
        s.create_account("owl", "hunter2hunter2").unwrap();
        AuthService::new(Arc::new(s))
    }

    fn sign_in(svc: &AuthService, who: &str, peer: &str) -> (String, String) {
        let LoginResponse::Ok { token, launch_id, .. } = svc.login(
            &LoginRequest { username: who.into(), password: "hunter2hunter2".into() },
            Some(peer),
        ) else {
            panic!("{who} must be able to sign in");
        };
        (token, launch_id)
    }

    /// **The bug, stated as a test at the level it was measured.**
    ///
    /// Against the old `DELETE FROM login_claims`, signing owl in removed otter's claim and
    /// every login connection then resolved to owl. Both halves are asserted: otter's claim
    /// still exists, and owl's sign-in did not make it disappear.
    #[test]
    fn a_second_sign_in_does_not_evict_the_first() {
        let svc = two_player_service();
        sign_in(&svc, "otter", "127.0.0.1");
        assert_eq!(svc.store().live_login_claims().unwrap().len(), 1);

        sign_in(&svc, "owl", "127.0.0.1");
        let live = svc.store().live_login_claims().unwrap();
        assert_eq!(live.len(), 2, "owl's sign-in deleted otter's claim");
        let names: Vec<&str> = live.iter().map(|c| c.account_name.as_str()).collect();
        assert!(names.contains(&"otter"), "{names:?}");
        assert!(names.contains(&"owl"), "{names:?}");
    }

    /// **And neither connection is served as the other.** This is the assertion the old code
    /// could not pass in any form: it answered every connection with owl.
    ///
    /// Same address for both - `127.0.0.1` - because that is the owner's machine and the case an
    /// address cannot separate. The client process is what separates them.
    #[test]
    fn two_clients_on_one_machine_each_resolve_to_their_own_account() {
        use store::ClaimEvidence;

        let svc = two_player_service();
        let (_, otter_launch) = sign_in(&svc, "otter", "127.0.0.1");
        let (_, owl_launch) = sign_in(&svc, "owl", "127.0.0.1");

        assert_eq!(svc.bind_launch(&otter_launch, 4242), LaunchResponse::Bound { pid: 4242 });
        assert_eq!(svc.bind_launch(&owl_launch, 9999), LaunchResponse::Bound { pid: 9999 });

        for (pid, want) in [(4242u32, "otter"), (9999, "owl")] {
            let r = svc
                .store()
                .resolve_login_claim(&ClaimEvidence::with_launch_pid(pid).from_peer("127.0.0.1"))
                .unwrap();
            let got = r.resolved().unwrap_or_else(|| panic!("pid {pid}: {}", r.why()));
            assert_eq!(got.claim.account_name, want, "{}", r.why());
        }
    }

    /// With nothing registered, two sign-ins are **ambiguous rather than resolved to the
    /// newest**. The login server then answers with its fallback account - a degrade, and
    /// never one player being served as another.
    #[test]
    fn two_sign_ins_with_no_launch_registered_refuse_to_guess() {
        let svc = two_player_service();
        sign_in(&svc, "otter", "127.0.0.1");
        sign_in(&svc, "owl", "127.0.0.1");
        assert!(
            svc.store().current_login_claim().unwrap().is_none(),
            "answering here at all is how one player gets served as another"
        );
    }

    /// A sign-in hands back a launch handle, and it is not the session token. If those two
    /// were ever the same value the launcher would be carrying a game credential again, which
    /// is the thing `launcher::http` was written to stop.
    #[test]
    fn a_sign_in_hands_back_a_launch_handle_that_is_not_the_token() {
        let svc = two_player_service();
        let (token, launch_id) = sign_in(&svc, "otter", "127.0.0.1");
        assert!(!launch_id.is_empty());
        assert_ne!(launch_id, token);
        let (_, second) = sign_in(&svc, "owl", "127.0.0.1");
        assert_ne!(launch_id, second, "two launches must not share a handle");
    }

    /// A stale or unknown handle is reported, not silently ignored. The launcher shows this,
    /// so an unregistered launch is visible before the client is on screen.
    #[test]
    fn an_unknown_launch_handle_is_reported_rather_than_accepted() {
        let svc = two_player_service();
        sign_in(&svc, "otter", "127.0.0.1");
        assert_eq!(svc.bind_launch("not-a-handle", 4242), LaunchResponse::UnknownLaunch);
        assert_eq!(svc.bind_launch("", 4242), LaunchResponse::UnknownLaunch);
        // pid 0 is the System Idle Process and can never own a socket: an unfilled field.
        let (_, launch_id) = sign_in(&svc, "owl", "127.0.0.1");
        assert_eq!(svc.bind_launch(&launch_id, 0), LaunchResponse::UnknownLaunch);
        // The control: the same handle with a real pid does bind, so the refusals above are
        // about what was passed and not about binding being broken.
        assert_eq!(svc.bind_launch(&launch_id, 7), LaunchResponse::Bound { pid: 7 });
    }

    /// The launch handle goes out over the wire, so it must be in the JSON - and the password
    /// must still not be. A field that is documented but not serialised is the "built is not
    /// wired" failure in one line.
    #[test]
    fn the_launch_handle_is_in_the_json_and_the_password_is_not() {
        let svc = two_player_service();
        let resp = svc.login(
            &LoginRequest { username: "otter".into(), password: "hunter2hunter2".into() },
            Some("127.0.0.1"),
        );
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("launch_id"), "{json}");
        assert!(!json.contains("hunter2hunter2"), "{json}");
        let LoginResponse::Ok { launch_id, .. } = &resp else { panic!("{resp:?}") };
        // **Non-empty FIRST.** `json.contains("")` is true of every string, so without this
        // the assertion below passes against a response that carries no handle at all - which
        // is exactly what an injection that stopped returning one produced, and it passed.
        // That is the vacuous-control failure `CLAUDE.md` keeps recording.
        assert!(!launch_id.is_empty(), "the sign-in produced no handle at all: {json}");
        assert!(json.contains(launch_id.as_str()), "the handle must actually reach the client");
    }
}
