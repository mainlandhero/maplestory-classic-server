//! The socket loop: framing, logging, and one thread per connection.
//!
//! Nothing about the protocol lives here. Everything this module decides is about bytes
//! and threads, so that [`crate::session`] can stay testable without a socket.

use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use net::{Direction, Framer, MapleCipher};
use store::{Account, Store};

use crate::config::Config;
use net::handshake::{greeting, CLIENT_RX_IV, CLIENT_TX_IV};
use net::names::{body_hex, label};
use crate::session::Session;

/// Timestamped, one line, to stdout.
///
/// **Every line carries a time.** An untimestamped log line in the Python probe once sat
/// under the last timestamped one and read as happening there, and eight client launches
/// went into explaining a connection death that had never happened. Redirect stdout to a
/// file to keep a run.
pub fn log(msg: &str) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = now.as_secs();
    let (h, m, s) = ((secs / 3600) % 24, (secs / 60) % 60, secs % 60);
    println!("{h:02}:{m:02}:{s:02}.{:03} {msg}", now.subsec_millis());
    let _ = std::io::stdout().flush();
}

/// Open the database and resolve the configured account.
///
/// A missing account is fatal rather than auto-created: a server that quietly invents one
/// would put a character list behind a name nobody made, and creating an account here
/// would mean inventing a password, which this project does not do anywhere.
fn open(config: &Config) -> std::io::Result<(Arc<Store>, Account)> {
    let store = Store::open(&config.db_path).map_err(|e| {
        std::io::Error::other(format!("could not open {}: {e}", config.db_path.display()))
    })?;
    let store = Arc::new(store);

    let account = match store.get_account(&config.account) {
        Ok(Some(a)) => a,
        Ok(None) => {
            return Err(std::io::Error::other(format!(
                "no account named {:?} in {} - create one with `maplecw-useradd {}`",
                config.account,
                config.db_path.display(),
                config.account
            )))
        }
        Err(e) => return Err(std::io::Error::other(format!("account lookup failed: {e}"))),
    };
    if !account.enabled {
        return Err(std::io::Error::other(format!(
            "account {:?} is disabled",
            config.account
        )));
    }
    Ok((store, account))
}

/// Print what is stored, without listening.
///
/// Worth having as its own mode: "did the last run actually save the character?" is the
/// question this crate exists to answer, and answering it should not cost a client launch.
pub fn list(config: &Config) -> std::io::Result<()> {
    let (store, account) = open(config)?;
    let characters = store
        .characters_for(account.id, config.world.id)
        .map_err(|e| std::io::Error::other(format!("could not read characters: {e}")))?;

    println!("{} ({})", config.db_path.display(), config.world.name);
    println!("account {:?} (id {})", account.name, account.id);
    if characters.is_empty() {
        println!("  no characters");
        return Ok(());
    }
    for chr in &characters {
        println!(
            "  id {:<5} {:<13} level {:<3} job {:<4} face {} hair {} {} equipped",
            chr.id,
            chr.name,
            chr.level,
            chr.job,
            chr.face,
            chr.hair,
            chr.equips.len()
        );
    }
    Ok(())
}

/// Delete one character by name, so a test can be repeated without hand-editing SQL.
///
/// Ownership is still enforced - the delete is scoped to the configured account - so this
/// cannot reach into someone else's characters just because it runs from a command line.
pub fn delete(config: &Config, name: &str) -> std::io::Result<()> {
    let (store, account) = open(config)?;
    let characters = store
        .characters_for(account.id, config.world.id)
        .map_err(|e| std::io::Error::other(format!("could not read characters: {e}")))?;

    let Some(target) = characters.iter().find(|c| c.name.eq_ignore_ascii_case(name)) else {
        return Err(std::io::Error::other(format!(
            "account {:?} has no character named {name:?}",
            account.name
        )));
    };
    let gone = store
        .delete_character(account.id, target.id)
        .map_err(|e| std::io::Error::other(format!("could not delete: {e}")))?;
    if gone {
        println!("deleted {:?} (id {})", target.name, target.id);
    } else {
        println!("nothing deleted - {:?} is not on this account", target.name);
    }
    Ok(())
}

/// Listen, and serve every connection until the process is stopped.
pub fn serve(config: Config) -> std::io::Result<()> {
    let config = Arc::new(config);
    let (store, account) = open(&config)?;

    let listener = TcpListener::bind(config.bind)?;
    log(&format!("listening on {}", config.bind));
    // What each channel is advertised as. Under `auto` the host is decided per connection,
    // and the public address is discovered HERE, once, so the banner can print it - then
    // re-checked in the background. `net::advertise`.
    for line in config.advertise.prepare(&config.world.channels) {
        log(&line);
    }
    config.advertise.spawn_refresher(|line| log(&line));
    if config.advertise.mode() == net::advertise::Mode::List {
        for (id, addr) in config.world.channels.iter().enumerate() {
            if addr.ip().is_loopback() && !config.bind.ip().is_loopback() {
                log(&format!("  WARNING: bind is not loopback but channel {id} is listed as loopback, so"));
                log("  an off-box client will be sent back to itself. Use --advertise auto.");
            }
        }
    }
    log(&format!("database {}", config.db_path.display()));
    log(&format!(
        "world {} id {} with {} channel(s)",
        config.world.name, config.world.id, config.world.channel_count()
    ));
    let characters = store.character_count(account.id, config.world.id).unwrap_or(0);
    log(&format!(
        "fallback account {:?} (id {}), {characters} character(s) stored",
        account.name, account.id
    ));
    match store.live_login_claims() {
        Ok(claims) if claims.is_empty() => {
            log("no login claim is live - run maplecw-launcher and sign in")
        }
        Ok(claims) => {
            log(&format!(
                "{} login claim(s) live. A connection this server can attribute to a launch is",
                claims.len()
            ));
            log("  served as that launch's account; one it cannot is served as the fallback");
            log("  below. It is never GUESSED at - serving the newest claim is how one player");
            log("  ends up looking at another player's characters.");
            for c in &claims {
                log(&format!("  claim: {:?} until {}", c.account_name, c.expires_at));
            }
        }
        Err(e) => log(&format!("could not read login claims: {e} - the fallback will be used")),
    }
    log("NOT AUTHENTICATED: the game socket carries no credentials, so anyone who");
    log("  connects is served as whichever account the claim or the fallback names.");
    log("  The launcher checks a password before staking a claim; this socket does not.");
    log("  See docs/launcher.md.");
    log("CLIENT TOKEN (0x0073): this server now READS the identity string the client carries");
    log("  and resolves the account from it - a credential on the wire instead of an");
    log("  inference about a socket. Every 0x0073 is logged with its length, so a run says");
    log("  whether the client carried anything at all.");
    log(&format!(
        "  A MapleCW token is {} characters of uppercase base32. An EMPTY identity is what",
        store::claims::CLIENT_TOKEN_CHARS
    ));
    log("  every capture before 2026-08-29 carried and what every reconnect sends - it is not");
    log("  a refusal, and the weaker rules still decide. A WRONG or SPENT one downgrades the");
    log("  connection to the fallback account, because a credential that can be skipped by");
    log("  presenting a bad one is not a credential.");
    log("  This authenticates the LOGIN socket only. 0x0073 has never appeared on a channel");
    log("  connection, so the game socket is unchanged.");
    if config.bind_migrations {
        log("MIGRATION BINDING IS ON. Every migration is bound to the live login claim's");
        log("  session token and CANNOT be claimed by a connection that presents none.");
        log("  The stock channel server presents none, so unless the hook has been taught");
        log("  to send the token, every character select will be refused and nobody will");
        log("  enter the world. If that is what you are seeing, this flag is why.");
    } else {
        log("MIGRATION BINDING IS OFF (--bind-migrations). A migration is claimed by the");
        log("  character id the connecting client ASSERTS, and nothing checks that the");
        log("  connection has any right to it - so any connection to a channel port can");
        log("  claim any character's pending migration by naming it. No race is needed.");
        log("  The mechanism to close this is built and tested; it needs the hook to send");
        log("  the session token on the channel socket before it can be switched on.");
    }

    // The migration makes the client come back on a second connection, so the log has to
    // say which one a line belongs to - the peer address alone differs only in an ephemeral
    // port, which is easy to misread when two connections interleave.
    let mut nth = 0u64;
    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                nth += 1;
                let store = store.clone();
                let config = config.clone();
                let account = account.clone();
                std::thread::spawn(move || {
                    let peer_addr = stream.peer_addr().ok();
                    let peer = peer_addr
                        .map(|a| a.to_string())
                        .unwrap_or_else(|| "unknown".to_string());
                    let peer = format!("#{nth} {peer}");
                    log(&format!("connection from {peer}"));

                    // WHICH LAUNCH THIS IS, and it is the only per-launch fact available.
                    // The pid comes from the operating system's TCP table keyed by the socket
                    // this server just accepted - the client asserts nothing, and it could
                    // not: the migration seed does not come back (8510 trials, zero hits) and
                    // its identity block is per-machine rather than per-launch (two distinct
                    // bodies across 56 launches). Both measured. `None` for a peer on another
                    // machine, which is honest rather than a failure.
                    let launch_pid = store::peerowner::owning_pid_of(peer_addr);
                    if let Some(addr) = peer_addr {
                        log(&format!("{peer} {}", store::peerowner::describe(addr, launch_pid)));
                    }
                    let evidence = store::ClaimEvidence {
                        // The game socket carries no credential. Written out rather than left
                        // to `..Default::default()` so the sentence sits in the code that
                        // would have to change if it ever stopped being true.
                        token: None,
                        // Nor does the socket, AT THIS POINT. The client's own credential
                        // arrives ~9 s later in `0x0073` - measured, and the accept path has
                        // no packet yet - so it is presented in `Session::on_session_identity`
                        // and cannot be part of the accept-time decision.
                        client_token: None,
                        launch_pid,
                        // A tie-breaker between MACHINES and never the discriminator: two
                        // clients on one box share it. The owner, 2026-08-29: "IP cannot be the
                        // sole discriminator."
                        peer: peer_addr.map(|a| a.ip().to_string()),
                    };

                    // Resolved HERE, per connection, not once at startup - that is what lets
                    // the launcher decide who is playing without restarting the server.
                    let (resolved, why, claim_token_hash) =
                        resolve_account(&store, &account, &evidence);
                    log(&format!("{peer} served as {why}"));
                    log(&format!(
                        "{peer} that decision is PROVISIONAL: if this client carries a one-time \
                         token in 0x0073, it overrides the above - and if it carries a wrong one \
                         this connection is downgraded to the fallback {:?}",
                        account.name
                    ));
                    match connection(
                        stream,
                        store,
                        config,
                        resolved,
                        account,
                        claim_token_hash,
                        launch_pid,
                    ) {
                        Ok(()) => log(&format!("{peer} closed")),
                        Err(e) => log(&format!("{peer} ended: {e}")),
                    }
                });
            }
            Err(e) => log(&format!("accept failed: {e}")),
        }
    }
    Ok(())
}

/// Who this connection is served as, and the sentence that explains it.
///
/// **This is the resolver `docs/launcher.md` asked for**, and it is the whole of what makes
/// more than one account usable on one machine. Until now the account came from `--account`
/// and was resolved once in [`serve`], so the process could only ever be one player.
///
/// The order is: a live login claim, else the configured fallback. A claim is staked by
/// `maplecw-launcher` after it verifies a password (argon2id), and it says "serve the next
/// game connection as this account".
///
/// # Three things this is not
///
/// 1. **It is not authentication.** The game socket carries no credentials - the client never
///    sends a username - so anything that connects to this port gets whatever claim is live.
///    The launcher authenticates a *person* before staking the claim; the socket is still
///    open. Say so when reporting progress.
/// 2. **It does not consume the claim.** The client opens a *second* login connection after
///    "Log Out" and "Choose another world" - `CLAUDE.md` records two `0x0010`s in one launch -
///    and a single-use claim would drop that second connection back to the fallback account.
///    On screen that reads as "my characters vanished when I logged out", which is a much
///    worse bug than the one it would be guarding against.
/// 3. **It does not fail the connection.** A claim naming an account that has since been
///    deleted or disabled falls back rather than refusing, because an unanswered connection
///    freezes the client's entire UI. The log line says which one was used and why, every
///    time, so a run is never ambiguous about whose characters are on screen.
fn resolve_account(
    store: &Store,
    fallback: &Account,
    evidence: &store::ClaimEvidence,
) -> (Account, String, Option<String>) {
    use store::ClaimResolution;

    let outcome = match store.resolve_login_claim(evidence) {
        Ok(outcome) => outcome,
        Err(e) => {
            return (
                fallback.clone(),
                format!(
                    "account {:?} (id {}) from --account; the claim lookup FAILED: {e}",
                    fallback.name, fallback.id
                ),
                None,
            )
        }
    };
    // Built before the match, because `ClaimResolution::why` is where the three sentences
    // live and two of them describe outcomes that are easy to summarise into silence.
    let why = outcome.why();

    match outcome {
        ClaimResolution::Resolved(resolved) => {
            match store.get_account(&resolved.claim.account_name) {
                Ok(Some(acc)) if acc.enabled => (acc, why, resolved.token_hash),
                // The claim resolved to nothing usable. `resolve_login_claim` already joins
                // to `accounts` and skips disabled ones, so reaching here means the row
                // changed underneath us between the two statements - rare, and not worth a
                // refusal.
                _ => (
                    fallback.clone(),
                    format!(
                        "account {:?} (id {}) - a claim named {:?} but it is no longer usable",
                        fallback.name, fallback.id, resolved.claim.account_name
                    ),
                    None,
                ),
            }
        }
        // BOTH of the remaining outcomes fall back, and they are DIFFERENT events with
        // different fixes - "nobody has signed in" and "two people have and this connection
        // could not be told apart". `why` says which, every time, which is the whole reason
        // the store returns three variants instead of an Option. "from --account" is kept
        // verbatim because `tests::with_no_claim_...` greps the line for it.
        ClaimResolution::NoClaim | ClaimResolution::Ambiguous { .. } => (
            fallback.clone(),
            format!("account {:?} (id {}) from --account; {why}", fallback.name, fallback.id),
            None,
        ),
    }
}

/// How long a client may say nothing before it is sent the startup gate again.
///
/// Four seconds, which is what the harness used on every successful run.
const QUIET_AFTER: Duration = Duration::from_secs(4);

/// One connection, start to finish.
fn connection(
    mut stream: TcpStream,
    store: Arc<Store>,
    config: Arc<Config>,
    account: Account,
    fallback: Account,
    claim_token_hash: Option<String>,
    launch_pid: Option<u32>,
) -> std::io::Result<()> {
    stream.set_nodelay(true)?;

    // The client speaks nothing until it has been greeted, and the greeting is plain.
    let hello = greeting(CLIENT_TX_IV, CLIENT_RX_IV);
    stream.write_all(&hello)?;
    log(&format!("-> greeting, {} bytes", hello.len()));

    // Two chains, two IVs, and they are not interchangeable: the client encrypts with the
    // one it read from field J and decrypts with the one from field K, and each direction
    // XORs a different constant into the header.
    let mut rx = Framer::new(MapleCipher::new(CLIENT_TX_IV.to_le_bytes(), Direction::ClientToServer));
    let mut tx = Framer::new(MapleCipher::new(CLIENT_RX_IV.to_le_bytes(), Direction::ServerToClient));

    // The address is recorded on any migration this connection mints. Defence in depth and
    // an audit trail - **not** a discriminator: two clients on one machine share an address.
    let local_addr = stream.local_addr().ok();
    let session = match stream.peer_addr() {
        Ok(addr) => Session::new(store, config, account).with_peer(addr.ip().to_string()),
        Err(_) => Session::new(store, config, account),
    };
    // The server's end of the socket. On a 0.0.0.0 bind it is the interface this client
    // actually reached, which is what a directly-connected client is told to dial for its
    // channel. `net::advertise`.
    let session = match local_addr {
        Some(local) => session.with_local_addr(local),
        None => session,
    };
    // The claim THIS connection resolved to, not whichever is newest. See the note on
    // `Session::with_claim_token_hash`.
    //
    // The fallback and the pid are here for `0x0073`: the client's own credential arrives
    // mid-connection, so the session needs to be able to re-resolve the account when it does -
    // upwards to the claim the token names, or **downwards** to the fallback when the token is
    // refused. The pid is what lets the same client process re-present a spent token; it comes
    // from the OS's TCP table at accept time, never from the client.
    let mut session = session
        .with_claim_token_hash(claim_token_hash)
        .with_fallback(fallback)
        .with_launch_pid(launch_pid);
    for reply in session.on_connect() {
        send(&mut stream, &mut tx, &reply.opcode, &reply.packet(), &reply.what)?;
    }

    // Not a keepalive - the client wants no heartbeat, and that was measured. This only
    // exists so a quiet client can be nudged with the startup gate again.
    stream.set_read_timeout(Some(QUIET_AFTER))?;

    let mut buf = [0u8; 8192];
    loop {
        let read = match stream.read(&mut buf) {
            Ok(n) => n,
            // Windows reports a read timeout as TimedOut, Unix as WouldBlock.
            Err(e) if matches!(e.kind(), ErrorKind::TimedOut | ErrorKind::WouldBlock) => {
                for reply in session.on_quiet() {
                    send(&mut stream, &mut tx, &reply.opcode, &reply.packet(), &reply.what)?;
                }
                continue;
            }
            Err(e) => return Err(e),
        };
        if read == 0 {
            return Ok(());
        }
        rx.feed(&buf[..read]);
        loop {
            let body = match rx.next_packet() {
                Ok(Some(body)) => body,
                Ok(None) => break,
                // A framing error means the stream is no longer decodable, so there is
                // nothing to recover to - close rather than resynchronise on garbage.
                Err(e) => return Err(std::io::Error::other(format!("framing: {e}"))),
            };
            let opcode = body
                .get(..2)
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .unwrap_or(0xFFFF);
            let payload = &body[2.min(body.len())..];
            log(&format!(
                "<- {}, {} byte body {}",
                label(opcode),
                payload.len(),
                body_hex(opcode, payload)
            ));

            if let Some(note) = crate::session::describe(opcode, payload) {
                log(&format!("   {note}"));
            }

            let replies = session.handle(&body);
            // Drained BEFORE the replies go out, because `0x0073`'s notes explain which
            // account the very next packet's character list belongs to. A handler that
            // produces no reply produces no `Reply::what` line, and `0x0073` deliberately
            // produces no reply - without this its whole decision would be invisible.
            for note in session.take_notes() {
                log(&format!("   {note}"));
            }
            if replies.is_empty() {
                log(&format!(
                    "   {} is not answered by this server{}",
                    label(opcode),
                    if net::names::opcode_name(opcode).is_none() {
                        " - and it is UNKNOWN, so the full body is above"
                    } else {
                        ""
                    }
                ));
            }
            for reply in replies {
                // **A reply that asks to be late is late.** Only the character list does, and
                // `session::CHARACTER_LIST_PAUSE_MS` is the whole argument for why.
                //
                // Blocking this thread is correct rather than lazy: one thread serves one
                // login connection, and the point is that THIS client sees a gap between the
                // world list and its character list. Nobody else is waiting on it.
                if reply.pause_ms > 0 {
                    log(&format!(
                        "   waiting {} ms before {} - see login::session::CHARACTER_LIST_PAUSE_MS",
                        reply.pause_ms,
                        label(reply.opcode)
                    ));
                    std::thread::sleep(std::time::Duration::from_millis(reply.pause_ms));
                }
                send(&mut stream, &mut tx, &reply.opcode, &reply.packet(), &reply.what)?;
            }
        }
    }
}

fn send(
    stream: &mut TcpStream,
    tx: &mut Framer<MapleCipher>,
    opcode: &u16,
    packet: &[u8],
    what: &str,
) -> std::io::Result<()> {
    let framed = tx.frame(packet);
    stream.write_all(&framed)?;
    log(&format!("-> {} {what}", label(*opcode)));
    log(&format!("   body {}", body_hex(*opcode, &packet[2.min(packet.len())..])));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use store::LOGIN_CLAIM_TTL_SECS;

    /// Two accounts and a store, which is the situation this whole resolver exists for:
    /// before it, one process could only ever be one player.
    fn two_accounts() -> (Store, Account, Account) {
        let store = Store::open_in_memory().unwrap();
        store.create_account("maplecw", "correct horse battery").unwrap();
        store.create_account("second_one", "correct horse battery").unwrap();
        let fallback = store.get_account("maplecw").unwrap().unwrap();
        let other = store.get_account("second_one").unwrap().unwrap();
        (store, fallback, other)
    }

    #[test]
    fn with_no_claim_a_connection_is_served_as_the_configured_fallback() {
        let (store, fallback, _) = two_accounts();
        let (acc, why, _) = resolve_account(&store, &fallback, &store::ClaimEvidence::none());
        assert_eq!(acc.id, fallback.id);
        assert!(why.contains("--account"), "{why}");
    }

    #[test]
    fn a_claim_overrides_the_fallback() {
        let (store, fallback, other) = two_accounts();
        store.stake_login_claim(other.id, "a-token", LOGIN_CLAIM_TTL_SECS).unwrap();
        let (acc, why, _) = resolve_account(&store, &fallback, &store::ClaimEvidence::none());
        assert_eq!(acc.id, other.id, "the launcher's claim decides, not --account");
        assert!(why.contains("login claim"), "{why}");
    }

    /// **The regression test for logging out.** The client opens a SECOND login connection
    /// after "Log Out" and "Choose another world" - `CLAUDE.md` records two `0x0010`s in one
    /// launch. If resolving consumed the claim, that second connection would fall back to
    /// `--account` and the player would watch their characters turn into someone else's.
    #[test]
    fn resolving_twice_serves_the_same_account_because_a_claim_is_not_consumed() {
        let (store, fallback, other) = two_accounts();
        store.stake_login_claim(other.id, "a-token", LOGIN_CLAIM_TTL_SECS).unwrap();
        let first = resolve_account(&store, &fallback, &store::ClaimEvidence::none()).0;
        let second = resolve_account(&store, &fallback, &store::ClaimEvidence::none()).0;
        assert_eq!(first.id, other.id);
        assert_eq!(second.id, other.id, "the log-out reconnect must not change account");
    }

    #[test]
    fn a_later_claim_replaces_an_earlier_one() {
        let (store, fallback, other) = two_accounts();
        store.stake_login_claim(other.id, "a-token", LOGIN_CLAIM_TTL_SECS).unwrap();
        store.stake_login_claim(fallback.id, "b-token", LOGIN_CLAIM_TTL_SECS).unwrap();
        assert_eq!(resolve_account(&store, &fallback, &store::ClaimEvidence::none()).0.id, fallback.id);
    }

    #[test]
    fn an_expired_claim_falls_back_rather_than_serving_a_stale_account() {
        let (store, fallback, other) = two_accounts();
        store.stake_login_claim(other.id, "a-token", -1).unwrap();
        let (acc, why, _) = resolve_account(&store, &fallback, &store::ClaimEvidence::none());
        assert_eq!(acc.id, fallback.id);
        assert!(why.contains("no launcher claim is live"), "{why}");
    }

    /// A disabled account must not keep playing on a claim staked before it was disabled -
    /// and the fall-back must still answer, because refusing the connection outright would
    /// leave the client with no reply, which freezes its entire UI.
    #[test]
    fn disabling_an_account_mid_claim_falls_back_instead_of_refusing() {
        let (store, fallback, other) = two_accounts();
        store.stake_login_claim(other.id, "a-token", LOGIN_CLAIM_TTL_SECS).unwrap();
        store.set_enabled("second_one", false).unwrap();
        let (acc, _, _) = resolve_account(&store, &fallback, &store::ClaimEvidence::none());
        assert_eq!(acc.id, fallback.id);
    }

    #[test]
    fn clearing_the_claim_returns_the_process_to_its_fallback() {
        let (store, fallback, other) = two_accounts();
        store.stake_login_claim(other.id, "a-token", LOGIN_CLAIM_TTL_SECS).unwrap();
        store.clear_login_claims().unwrap();
        assert_eq!(resolve_account(&store, &fallback, &store::ClaimEvidence::none()).0.id, fallback.id);
    }

    /// The log line is the only way a run says whose characters were on screen, and three of
    /// this project's answers came from reading a log after the fact. It must always name the
    /// account it chose.
    #[test]
    fn every_outcome_names_the_account_it_chose() {
        let (store, fallback, other) = two_accounts();
        let (_, no_claim, _) = resolve_account(&store, &fallback, &store::ClaimEvidence::none());
        assert!(no_claim.contains("maplecw"), "{no_claim}");
        store.stake_login_claim(other.id, "a-token", LOGIN_CLAIM_TTL_SECS).unwrap();
        let (_, claimed, _) = resolve_account(&store, &fallback, &store::ClaimEvidence::none());
        assert!(claimed.contains("second_one"), "{claimed}");
    }
}
