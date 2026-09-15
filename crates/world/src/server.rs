//! One channel's socket loop: framing, logging, one thread per connection.
//!
//! This deliberately mirrors `login::server` rather than sharing it. The two loops look
//! alike today and are about to stop: login nudges a quiet client with the startup gate on
//! a read timeout, and a channel must not; login greets and immediately sends a packet, and
//! a channel greets and waits. Factoring them together now would mean threading the
//! differences back out through a trait for the sake of forty shared lines.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, OnceLock};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use net::handshake::{channel_greeting, CLIENT_RX_IV, CLIENT_TX_IV};
use net::names::{body_hex, label, opcode_name};
use net::{ByteShiftCipher, Direction, Framer, MapleCipher, Shift};
use store::Store;

use crate::config::Config;
use crate::session::{Session, CLIENT_MIGRATION_HELLO};

/// Timestamped, one line, to stdout. Same rule as the login server: an untimestamped line
/// once read as happening where it sat in the file and cost eight client launches.
pub fn log(msg: &str) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = now.as_secs();
    let (h, m, s) = ((secs / 3600) % 24, (secs / 60) % 60, secs % 60);
    say(&format!("{h:02}:{m:02}:{s:02}.{:03} {msg}", now.subsec_millis()));
}

// ---------------------------------------------------------------------------------------
// Where the log goes, and how big it may get
// ---------------------------------------------------------------------------------------

/// A log file is rolled once it passes this. The owner, 2026-09-14: *"introduce automatic log
/// rotate at 50mb so logs do not go out of control."*
pub const LOG_ROTATE_BYTES: u64 = 50 * 1024 * 1024;

/// How many rolled files are kept beside the live one: `world-ch0.log.1` is the newest,
/// `.5` the oldest, and the sixth is deleted. Five rolls at 50 MB is 300 MB of channel log
/// at most, per channel - a bounded quantity, which is the point.
pub const LOG_KEEP: usize = 5;

/// The file every line goes to once [`install_log_file`] has run. Until then, and in every
/// test, lines go to stdout as they always did.
static LOG_SINK: OnceLock<std::sync::Mutex<LogSink>> = OnceLock::new();

struct LogSink {
    path: std::path::PathBuf,
    file: std::fs::File,
    written: u64,
}

impl LogSink {
    fn open(path: &std::path::Path) -> std::io::Result<Self> {
        let file = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
        let written = file.metadata().map(|m| m.len()).unwrap_or(0);
        Ok(Self { path: path.to_path_buf(), file, written })
    }

    /// `path.4` -> `path.5`, ..., `path` -> `path.1`, then a fresh `path`. A rename that
    /// fails is skipped rather than fatal: the worst case is one roll's worth of overwrite,
    /// and a logger must never take the server down.
    fn rotate(&mut self) {
        let _ = self.file.flush();
        let name = |n: usize| {
            let mut p = self.path.clone().into_os_string();
            p.push(format!(".{n}"));
            std::path::PathBuf::from(p)
        };
        let _ = std::fs::remove_file(name(LOG_KEEP));
        for n in (1..LOG_KEEP).rev() {
            let _ = std::fs::rename(name(n), name(n + 1));
        }
        let _ = std::fs::rename(&self.path, name(1));
        if let Ok(fresh) = Self::open(&self.path) {
            *self = fresh;
        }
    }

    fn write_line(&mut self, line: &str) {
        let bytes = line.len() as u64 + 1;
        if self.written + bytes > LOG_ROTATE_BYTES {
            self.rotate();
            let _ = writeln!(self.file, "(rolled at {} bytes - the previous {} are beside this file as .1 .. .{})", LOG_ROTATE_BYTES, LOG_KEEP, LOG_KEEP);
        }
        if writeln!(self.file, "{line}").is_ok() {
            self.written += bytes;
        }
        let _ = self.file.flush();
    }
}

/// **Send every line from here on to `path`, rolling it at [`LOG_ROTATE_BYTES`].**
///
/// `--log-file`. Until this was added the channel wrote to stdout and the launch script
/// redirected that to `world-chN.log`, which meant nothing in the process could roll the
/// file - the handle belongs to whoever opened it. Owning the file is what makes a size
/// limit possible at all. Stdout stays for whatever the process prints before this runs,
/// and for a run that never passes the flag, which is every test.
pub fn install_log_file(path: &std::path::Path) -> std::io::Result<()> {
    let sink = LogSink::open(path)?;
    let _ = LOG_SINK.set(std::sync::Mutex::new(sink));
    Ok(())
}

/// One raw line, untimestamped, to the log file if one is installed and to stdout if not.
/// The startup banner uses this so it lands in the same file as everything after it.
pub fn say(line: &str) {
    if let Some(sink) = LOG_SINK.get() {
        sink.lock().unwrap_or_else(|e| e.into_inner()).write_line(line);
    } else {
        println!("{line}");
        let _ = std::io::stdout().flush();
    }
}

// ---------------------------------------------------------------------------------------
// Chatter: the packets that are 97% of a busy log and never decisive on their own
// ---------------------------------------------------------------------------------------

/// **Log every mob move and every ack as its own line**, the way every run before 2026-09-14
/// did. Off by default; `--log-chatter` / `-LogChatter` turns it on for a run where the mob
/// paths ARE the subject.
///
/// The owner, 2026-09-14, looking at a 184 MB channel log: *"Can we analyze the junk and stop
/// logging those junk and focus on the important things?"* Measured on that file
/// (`Crash Investigation/previous-runs/world-20260912-220618.log`, 818 113 lines): `<- 0x02FF`
/// was 56.0% of the bytes, `-> 0x03E4` 29.8%, and their `body` lines 11.1% - **97% in one
/// conversation**, 261 682 mob moves in one session. The client's periodic `<- 0x0070` was the
/// next 1.7%. Everything anyone has ever read a decision off - user moves, drops, attacks,
/// SetField, the pet - was the remaining 1.3%.
///
/// Those lines have also been evidence: the mob-move lag, the drop site, the pet's foothold
/// all came out of `0x02FF` bodies. So they are not deleted, they are **counted**, and the
/// count is printed once a minute per opcode and direction. A run that needs the bodies asks
/// for them; `research/fixtures/` keeps the ones that already settled something.
pub static LOG_CHATTER: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// The routine, high-volume packets that are summarised instead of logged line by line.
/// **Direction matters**: outbound `0x0070` is an InventoryOperation and is always logged;
/// only the client's own inbound `0x0070` environment report is chatter.
pub fn is_chatter(inbound: bool, opcode: u16) -> bool {
    if inbound {
        matches!(opcode, net::mobmove::MOB_MOVE_REQUEST | 0x0070)
    } else {
        matches!(opcode, net::mobmove::MOB_CTRL_ACK)
    }
}

/// How often the chatter counts are printed.
const CHATTER_SUMMARY_SECS: u64 = 60;

static CHATTER: OnceLock<std::sync::Mutex<Chatter>> = OnceLock::new();

#[derive(Default)]
struct Chatter {
    counts: std::collections::BTreeMap<(bool, u16), u64>,
    since: Option<Instant>,
}

/// Count one chatter packet, and print the summary when a minute has passed. Returns
/// `true` when the caller should NOT log the packet itself.
fn count_chatter(inbound: bool, opcode: u16) -> bool {
    if LOG_CHATTER.load(std::sync::atomic::Ordering::Relaxed) || !is_chatter(inbound, opcode) {
        return false;
    }
    let chatter = CHATTER.get_or_init(|| std::sync::Mutex::new(Chatter::default()));
    let mut c = chatter.lock().unwrap_or_else(|e| e.into_inner());
    *c.counts.entry((inbound, opcode)).or_insert(0) += 1;
    let since = *c.since.get_or_insert_with(Instant::now);
    if since.elapsed().as_secs() >= CHATTER_SUMMARY_SECS {
        let parts: Vec<String> = c
            .counts
            .iter()
            .map(|((inb, op), n)| format!("{} {} x{n}", if *inb { "<-" } else { "->" }, label(*op)))
            .collect();
        c.counts.clear();
        c.since = Some(Instant::now());
        drop(c);
        log(&format!(
            "chatter, last {CHATTER_SUMMARY_SECS} s (counted, not logged - pass --log-chatter for the lines): {}",
            parts.join(", ")
        ));
    }
    true
}

/// How often a quiet connection wakes up to let the session send something.
///
/// Short enough that a 6-second chatter interval lands within about a tenth of a
/// second of when it is due, long enough that an idle connection is not spinning. It
/// is **not** the chatter interval - that lives in `session::CHATTER_INTERVAL_MS`,
/// because it is a game decision and this is a socket one.
///
/// **Was 500 until 2026-08-29, and what moved it is multiplayer.** This is the
/// wakeup that lets a session collect its mail (`crate::broadcast`), and mail is
/// how one player's movement reaches another. A player standing still sends
/// nothing, so this timeout is the *only* thing waking their connection up -
/// at 500 ms another player's walk arrived in half-second jumps. The cost of
/// 100 ms is ten wakeups a second per connection doing a drop sweep, a respawn
/// check and an empty drain; the benefit is that a broadcast waits at most a
/// tenth of a second.
const TICK_MS: u64 = 100;

/// The one clock the whole channel runs on. See the long note at its use in [`connection`].
///
/// A `OnceLock`, so it is set once - by `serve` at startup, or by the first connection if a
/// test drives `connection` directly - and every caller after that reads the same origin.
/// [`connection`] measures `now_ms` from it, which is what makes a drop's lifetime, a mob's
/// respawn and a drop's owner lock agree across the players who share a field.
static PROCESS_ORIGIN: OnceLock<Instant> = OnceLock::new();

/// The shared origin, initialising it on first read.
fn process_origin() -> Instant {
    *PROCESS_ORIGIN.get_or_init(Instant::now)
}

/// How often a channel connection refreshes the presence lease it holds.
///
/// **Not every tick.** `TICK_MS` is 100, and renewing there would be ten `UPDATE`s a second
/// per connection for a value nothing reads more than once a minute. Fifteen seconds against
/// `store::PRESENCE_LEASE_SECS` of sixty is four chances to miss one before an account frees
/// itself underneath a player who is still in the map.
const PRESENCE_RENEW_MS: u64 = 15_000;

/// How often a connection asks whether the server wants it gone.
///
/// Two seconds, not every tick: the question is a SELECT on an indexed primary key, but it is
/// asked by every connection on the channel and the answer changes at human speed - somebody
/// pressing Login. Two seconds is the delay between that press and the old client's socket
/// closing, which is well inside the time the new client spends loading.
const KICK_POLL_MS: u64 = 2_000;

/// **How a connection ended**, so the accept loop can say who ended it.
///
/// The owner, 2026-09-13: *"Make sure every disconnect initiated by the server is logged clearly
/// in the server channel logs."* The point of making this a type rather than a log call at
/// the point of decision is that a future disconnect cannot be added without choosing one of
/// these arms, and both arms are logged. A bare `return Ok(())` now means "the client went
/// away", and nothing else can borrow that meaning by accident.
#[derive(Debug)]
enum Close {
    /// The socket closed from the far end: the player quit, changed channel, or crashed.
    Client,
    /// **The server ended it**, for the reason given. Always logged, always as a disconnect.
    Server(String),
}

fn send(
    stream: &mut TcpStream,
    tx: &mut Framer<ByteShiftCipher>,
    to: &str,
    opcode: u16,
    packet: &[u8],
    what: &str,
) -> std::io::Result<()> {
    let framed = tx.frame(packet);
    stream.write_all(&framed)?;
    if count_chatter(false, opcode) {
        return Ok(());
    }
    // `to` is who this connection is serving - `Wisp#215` - so a reply line says whose
    // screen it lands on without the reader having to find the hello above it.
    log(&format!("-> [{to}] {} {what}", label(opcode)));
    log(&format!("   body {}", body_hex(opcode, &packet[2.min(packet.len())..])));
    Ok(())
}

fn connection(
    mut stream: TcpStream,
    store: Arc<Store>,
    config: Arc<Config>,
    // Every connection on this channel shares one set of fields: mobs keep their
    // positions when a player leaves, and a second player sees the same field.
    fields: Arc<crate::fields::Fields>,
) -> std::io::Result<Close> {
    stream.set_nodelay(true)?;

    // NOT the login greeting. A channel connection has `conn+0x48 == 0`, so the client
    // skips the greeting's two gated blocks - send it the login greeting and it reads `G`
    // from where `A` sits, gets 0, and raises "The client is outdated". See
    // `net::handshake::channel_greeting` and docs/transport.md.
    let hello = channel_greeting(CLIENT_TX_IV, CLIENT_RX_IV);
    stream.write_all(&hello)?;
    log(&format!("-> channel greeting, {} bytes (no A..F, no version block)", hello.len()));

    // **The channel is asymmetric.** Both halves are measured, on different runs:
    //
    // * client -> server is **AES-256-OFB**, same key as login. The two packets the client
    //   sent on its first channel connection decode under AES and nothing else - `0x0070`
    //   with the body it also sends on login, and `0x007D` carrying character id 204.
    // * server -> client is the **byte subtract** `FUN_1406ef9f0` selects from `conn+0x48`,
    //   so we ADD `iv[0]` and the client's subtract recovers the plaintext. A `SetField`
    //   sent under AES was dispatched by the client as opcode `0x406C`, which is that
    //   ciphertext minus `iv[0] = 0x02` byte for byte.
    //
    // The first bullet was briefly written up as covering both directions. It never did:
    // "the channel is AES" was two claims, and only one had been tested.
    log("cipher: ASYMMETRIC - the client SENDS AES-256-OFB and RECEIVES a byte subtract,");
    log("  so we decrypt with AES and encrypt by adding iv[0]. Measured 2026-08-19.");
    let mut rx =
        Framer::new(MapleCipher::new(CLIENT_TX_IV.to_le_bytes(), Direction::ClientToServer));
    // **The channel is asymmetric, and this is measured, not inferred.** The client sends
    // AES-256-OFB and *receives* the byte subtract - `out[i] = in[i] - iv[0]`. Sending it
    // AES produced a body it decoded as opcode 0x406C, which is exactly our AES ciphertext
    // minus iv[0]; the run of 2026-08-19 is the arithmetic. So we ADD on send and its
    // subtract recovers the plaintext.
    let mut tx = Framer::new(ByteShiftCipher::new(
        CLIENT_RX_IV.to_le_bytes(),
        Direction::ServerToClient,
        Shift::Add,
    ));

    // The address this connection came from. The **address** is recorded and never decisive -
    // two clients on one machine share it, and a dual-stack client legitimately arrives as ::1
    // on one socket and 127.0.0.1 on the other. Refusing on it would lock the owner out of their own
    // server; it is here so a suspected impersonation has something to read.
    //
    // **The whole `SocketAddr` is kept, port included, and the port is the load-bearing half.**
    // `Session::claim_for_character` asks the operating system which process owns this socket
    // (`store::peerowner`), and that lookup is keyed on the client's local endpoint - address
    // AND port. Every client on this machine shares `127.0.0.1`, so an address-only lookup
    // matches the first row and returns a confident wrong pid. `with_peer_addr` sets both
    // fields, so this line changes nothing about the log and adds the one fact the attestation
    // needs.
    let local_addr = stream.local_addr().ok();
    let peer_addr = stream.peer_addr().ok();
    // **The same two facts the login server keys a presence lease on**, derived the same way:
    // the process the operating system attributes this socket to, and the address as a
    // fallback for an off-box peer. Both come from the kernel, neither from the client. This
    // is what makes the login connection's lease and this one's the SAME lease - see
    // `store::presence::holder_key`.
    let peer_ip = peer_addr.map(|a| a.ip().to_string());
    let launch_pid = store::peerowner::owning_pid_of(peer_addr);
    let mut session = match peer_addr {
        Some(addr) => Session::joining(store.clone(), config.clone(), fields).with_peer_addr(addr),
        None => Session::joining(store.clone(), config.clone(), fields),
    };
    // The server's end of the socket: on a 0.0.0.0 bind, the interface this client reached,
    // which is what it is told to dial when it changes channel. `net::advertise`.
    if let Some(local) = local_addr {
        session = session.with_local_addr(local);
    }
    for reply in session.on_connect() {
        send(&mut stream, &mut tx, &session.log_tag(), reply.opcode, &reply.packet(), &reply.what)?;
    }

    // A read timeout, and this file used to say a channel needed none. It does now: NPC idle
    // chatter is server-triggered, so there is finally something to say to a quiet client,
    // and this wakeup is what says it.
    //
    // **A timeout is not a disconnect**, and conflating them is the classic way to write
    // this bug: `WouldBlock` and `TimedOut` both mean "nothing arrived", and platforms
    // disagree about which one they raise - Windows tends to `TimedOut` where Unix gives
    // `WouldBlock`. Both are handled, because getting it wrong drops every idle connection
    // after one interval and looks exactly like the client disconnecting.
    stream.set_read_timeout(Some(std::time::Duration::from_millis(TICK_MS)))?;
    // **The channel clock is process-wide, not per-connection**, and that is a bug fix, not a
    // detail. `now_ms` is the milliseconds this clock reads, and it is the ONLY clock the
    // shared field state is timed against: a drop's lifetime, a mob's respawn delay, a drop's
    // owner lock. Those live in `crate::fields`, one set per channel, shared by every
    // connection - but `now_ms` used to be `Instant::now()` captured HERE, per connection, so
    // each player measured the field against how long THEIR OWN client had been connected.
    //
    // On 2026-09-14 that emptied map 30 of every drop within one tick. Three players: one
    // connected at 02:54, another at 02:59. A drop the 02:59 player created was stamped at its
    // clock (~315 s) and expires 120 s later (~435 s); the 02:54 player's tick swept the same
    // shared drop at ITS clock (~634 s), and 634 >= 435, so the drop was gone ~110 ms after it
    // landed - before anyone could pick it up. The quest item "did not drop", the party saw
    // the kill but could not grab it, and it stopped the moment the oldest connection left
    // (03:07:15 in that log; pick-ups worked from 03:07:24). `crate::fields`, `crate::drops`.
    //
    // A single origin for the whole process puts every connection on one timeline, so the
    // clock that stamps a drop and the clock that sweeps it are the same clock. It is
    // monotonic, so nothing here goes backwards. Per-connection things (buffs, regen, chatter)
    // are unaffected: they store `now_ms + duration` and compare with the same `now_ms`, and a
    // shared origin changes only the baseline, not the elapsed differences they rely on.
    let started = process_origin();

    // **THE PRESENCE LEASE, held for as long as this player is in the world.**
    //
    // The login connection took it, then handed it over rather than releasing it, because it
    // closes a second before this one opens - `login::session::hand_presence_to_the_channel`.
    // This connection re-takes it under the SAME key (the client process, as the operating
    // system attributes the socket), so the takeover is a renewal rather than a fight, and
    // then holds it until the socket closes.
    //
    // **Every return path below releases it, and that is the whole reason it is a guard.**
    // There are five of them - a clean close, a read error, a framing error, a write failure
    // in the tick, a write failure in the reply loop - and the one that matters is the read
    // error, because that is the crash: `world.log` records it as `ended: An existing
    // connection was forcibly closed by the remote host. (os error 10054)`. A release written
    // at any one `return` is a release missed at the other four.
    let mut presence: Option<store::PresenceGuard> = None;
    let mut presence_renewed_ms: u64 = 0;
    let presence_holder = store::holder_key(launch_pid, peer_ip.as_deref());

    // **Has anybody asked for this player to be thrown off since they joined?**
    //
    // Set at the migration hello, which is the first and only point at which this connection
    // learns which account it is - the channel socket carries no credentials. The watch stamps
    // that instant, so a kick queued BEFORE this connection joined is not ours: that is what
    // stops the sign-in that authorised this launch from disconnecting it. `store::kick`.
    let mut kicks: Option<store::KickWatch> = None;
    let mut kick_checked_ms: u64 = 0;

    let mut buf = [0u8; 8192];
    loop {
        // Asked here rather than in the timeout branch below, because that branch only runs
        // when nothing arrived: a client that is moving, attacking or chatting can keep the
        // socket busy for a long time, and it is exactly the player who is doing something who
        // needs to be thrown off cleanly.
        if let Some(watch) = kicks {
            let now_ms = started.elapsed().as_millis() as u64;
            if now_ms.saturating_sub(kick_checked_ms) >= KICK_POLL_MS {
                kick_checked_ms = now_ms;
                match watch.poll(&store) {
                    Ok(Some(kick)) => {
                        return Ok(Close::Server(format!(
                            "account {} was asked to be disconnected: {}. The request was made \
                             at unix {} and this connection joined at unix {}",
                            kick.account_id, kick.reason, kick.requested_at, watch.joined_at
                        )));
                    }
                    Ok(None) => {}
                    // A table that will not read must not throw anybody out. Same call this
                    // file already makes about the presence lease.
                    Err(e) => log(&format!(
                        "KICK: could not read the disconnect requests for account {}: {e}. \
                         This connection stays",
                        watch.account_id
                    )),
                }
            }
        }

        let read = match stream.read(&mut buf) {
            Ok(0) => return Ok(Close::Client),
            Ok(n) => n,
            Err(e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                // Nothing arrived. Give the session the clock and send whatever it owes.
                let now_ms = started.elapsed().as_millis() as u64;
                // Keep the lease alive. A player standing still sends nothing at all, and
                // this wakeup is the only thing that runs for them.
                if let Some(guard) = &presence {
                    if now_ms.saturating_sub(presence_renewed_ms) >= PRESENCE_RENEW_MS {
                        presence_renewed_ms = now_ms;
                        let lost = match guard.renew() {
                            Ok(true) => None,
                            Ok(false) => Some(guard.account_id()),
                            Err(e) => {
                                log(&format!("PRESENCE: could not renew the lease: {e}"));
                                None
                            }
                        };
                        // **Re-take it rather than leaving the player unheld.** Two ways to
                        // get here and both are real: the lease genuinely went stale, or a
                        // Change Channel raced - the old channel's guard released a row the
                        // new channel had already re-taken under the same key. Neither is
                        // fatal, both fail OPEN (a second client could log in), and both are
                        // repaired by asking again. A guard whose answer is ignored is not a
                        // guard, and "we lost it" is an answer.
                        if let (Some(account_id), Some(holder)) = (lost, presence_holder.as_deref())
                        {
                            presence = None;
                            let whence =
                                format!("channel {} (re-taken, holder {holder})", config.channel_id);
                            match store::PresenceGuard::hold(
                                store.clone(),
                                account_id,
                                holder,
                                &whence,
                            ) {
                                Ok(Ok(guard)) => {
                                    log(&format!(
                                        "PRESENCE: the lease on account {account_id} had been \
                                         lost and was RE-TAKEN by this connection"
                                    ));
                                    presence = Some(guard);
                                }
                                Ok(Err(who)) => log(&format!(
                                    "PRESENCE: the lease on account {account_id} is held by {} \
                                     and could not be re-taken. This player stays in the world",
                                    who.whence
                                )),
                                Err(e) => log(&format!(
                                    "PRESENCE: could not re-take the lease for account \
                                     {account_id}: {e}"
                                )),
                            }
                        }
                    }
                }
                for reply in session.tick(now_ms) {
                    send(&mut stream, &mut tx, &session.log_tag(), reply.opcode, &reply.packet(), &reply.what)?;
                }
                continue;
            }
            Err(e) => return Err(e),
        };
        rx.feed(&buf[..read]);
        loop {
            let body = match rx.next_packet() {
                Ok(Some(body)) => body,
                Ok(None) => break,
                Err(e) => return Err(std::io::Error::other(format!("framing: {e}"))),
            };
            let opcode = body
                .get(..2)
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .unwrap_or(0xFFFF);
            let payload = body.get(2..).unwrap_or(&[]);
            if !count_chatter(true, opcode) {
                log(&format!(
                    "<- [{}] {}, {} byte body {}",
                    session.log_tag(),
                    label(opcode),
                    payload.len(),
                    body_hex(opcode, payload)
                ));
            }

            if opcode == CLIENT_MIGRATION_HELLO {
                describe_hello(&mut session, payload);
                // The account is known from here on, so this is where the disconnect watch
                // starts. Deliberately NOT tied to the presence lease below: that lease can
                // fail to be taken (somebody else holds it, or the table would not read) and
                // this player is served anyway, so a kick must still reach them.
                if kicks.is_none() {
                    if let Some(claimed) = session.claimed() {
                        let watch = store.watch_for_kicks(claimed.account_id);
                        log(&format!(
                            "   KICK WATCH: account {} joined at unix {}. A disconnect \
                             requested from here on - by a second sign-in on this account, \
                             say - closes this connection within {} ms. One requested BEFORE \
                             this instant belongs to an earlier launch and is ignored",
                            watch.account_id, watch.joined_at, KICK_POLL_MS
                        ));
                        kicks = Some(watch);
                    }
                }
                // The hello is the first and only point at which this connection learns WHICH
                // ACCOUNT it is - the channel socket carries no credentials, and the account
                // comes out of the migration row the hello claimed. So the lease is taken
                // here, and only when a migration was actually claimed: a hello that was
                // refused enters no world and must hold nothing.
                if presence.is_none() {
                    if let (Some(claimed), Some(holder)) =
                        (session.claimed(), presence_holder.as_deref())
                    {
                        let account_id = claimed.account_id;
                        let whence = format!(
                            "channel {} playing character {} (holder {holder})",
                            config.channel_id, claimed.character_id
                        );
                        match store::PresenceGuard::hold(
                            store.clone(),
                            account_id,
                            holder,
                            &whence,
                        ) {
                            Ok(Ok(guard)) => {
                                log(&format!(
                                    "   PRESENCE: account {account_id} is held by {holder} for \
                                     as long as this channel connection lives. It is released \
                                     when the socket closes - including when it closes because \
                                     the client crashed - and expires by itself {} s after the \
                                     last renewal if this process dies outright",
                                    store::PRESENCE_LEASE_SECS
                                ));
                                presence = Some(guard);
                                presence_renewed_ms = started.elapsed().as_millis() as u64;
                            }
                            // Somebody else holds it. NOT a refusal: this player has already
                            // been let into the world by the login server, and throwing them
                            // out here would be a lockout arriving one screen later. It is a
                            // log line, because the only way to reach it is a pid lookup that
                            // disagreed with the login connection's.
                            Ok(Err(who)) => log(&format!(
                                "   PRESENCE: account {account_id} is held by {} rather than by \
                                 this connection, so this connection holds NOTHING. The player \
                                 is still served - refusing here would be a lockout one screen \
                                 after the login server allowed them in",
                                who.whence
                            )),
                            Err(e) => log(&format!(
                                "   PRESENCE: could not take the lease for account \
                                 {account_id}: {e}. The player is served; a table that will \
                                 not read must not keep anybody out"
                            )),
                        }
                    }
                }
            }

            let replies = session.handle(&body);
            if replies.is_empty() {
                // Two different silences. A report is answered with nothing because that
                // is correct (net::names::is_client_report - 0x013D must not be answered);
                // anything else with no reply is a gap, and the line says which.
                if net::names::is_client_report(opcode) {
                    log(&format!(
                        "   {} is a client report; nothing is expected back",
                        label(opcode)
                    ));
                } else {
                    log(&format!(
                        "   {} is not answered yet{} - see crates/world/src/session/",
                        label(opcode),
                        if opcode_name(opcode).is_none() {
                            ", and it is UNKNOWN, so the full body is above"
                        } else {
                            ""
                        }
                    ));
                }
            }
            for reply in replies {
                send(&mut stream, &mut tx, &session.log_tag(), reply.opcode, &reply.packet(), &reply.what)?;
            }
        }
    }
}

/// Read the client's `0x007D` and claim the migration it belongs to.
///
/// **Decoded from a real capture, 2026-08-19.** The seed the migration packet handed over
/// does **not** come back here - it is absent from the body, plainly and under the
/// obfuscated-block search. What the client sends instead is its **character id**, at
/// offset 8, followed by the same MAC and machine id it puts in `0x0073`.
///
/// So the handoff is keyed on the character, and the single-use migration row is what makes
/// it safe rather than the seed being secret. Which was always the honest description of a
/// `u32` anyway.
fn describe_hello(session: &mut Session, payload: &[u8]) {
    match crate::session::migration_hello_character(payload) {
        Some(id) => {
            log(&format!("   MIGRATION HELLO: character id {id}"));
            log(&format!("   {}", session.claim_for_character(id)));
        }
        None => log(&format!(
            "   MIGRATION HELLO: {} bytes, too short to hold a character id - read the hex \
             above",
            payload.len()
        )),
    }
}

/// Listen on one channel until the process is stopped.
pub fn serve(config: Config) -> std::io::Result<()> {
    // Start the shared channel clock at process startup, so the first drop is timed against a
    // baseline that predates it rather than against whenever the first player happened to
    // connect. See [`connection`] and [`process_origin`].
    let _ = process_origin();
    let config = Arc::new(config);
    let store = Arc::new(
        Store::open(&config.db_path)
            .map_err(|e| std::io::Error::other(format!("{}: {e}", config.db_path.display())))?,
    );

    // **WHICH BUILD IS THIS?** First line out, before anything that could fail.
    //
    // The owner, 2026-09-13, after an afternoon's fixes turned out not to be running on the
    // deployed server and the only way to tell was to reason about its behaviour: *"as
    // part of startup, all of the processes should include a build time from now on"*.
    // It is the executable's own file - time, size and digest - so it moves whenever the
    // linker rewrites the binary, including for a dependency-only change. `store::buildstamp`
    // says why a compile-time constant would have been stale in exactly this case.
    log(&store::buildstamp::line());

    let listener = TcpListener::bind(config.bind)?;
    log(&format!(
        "world {} channel {} listening on {}",
        config.world_id, config.channel_id, config.bind
    ));
    log(&format!("database {}", config.db_path.display()));
    // What a Change Channel answer names as the host - decided per connection under `auto`;
    // the public address is discovered here, once, and re-checked in the background.
    for line in config.advertise.prepare(&config.channels) {
        log(&line);
    }
    config.advertise.spawn_refresher(|line| log(&line));
    log("NOT AUTHENTICATED: a migration seed is a u32, so it identifies a pending");
    log("  migration rather than proving who is on the far end. It is single-use.");
    if config.answer_packets {
        log("THIS CHANNEL ANSWERS PACKETS - the default since 2026-09-14. It used to need");
        log("  --set-field-probe, a flag every caller passed and whose absence cost a launch;");
        log("  --silent-channel is the way to turn answering off now, and it is only for");
        log("  eliminating the channel as a variable.");
        log("  This channel answers: the migration hello with a SetField");
        log("  carrying the character's real record (presence[0] the stat block, presence[2]");
        log("  the equipped list), 0x00DC with that field's NPCs and mobs, 0x00D1 with the");
        log("  portal's destination, 0x00E7 with the !map GM command, and 0x0151 with a");
        log("  script Say, 0x0104 with the NPC shop, and 0x00D2 with a channel migration.");
        log("  CONFIRMED on a real client: the dressed character, the bag and its 125 slots,");
        log("  NPC dialogue, mobs spawning and moving, an unequip that survives a map change.");
        log("  NOT yet seen on screen: the shop counter, the quest journal, the channel");
        log("  switch. Dropping an item is refused on purpose - there is nowhere to put it.");
    } else {
        log("--silent-channel: packets are logged and NOT ANSWERED. The stage is decoded:");
        log("  SetField is inbound 0x01A0, confirmed on a live client, and its 33-byte");
        log("  head is in research/msexe-stage-setfield.md. What is missing is the");
        log("  character record it must carry - see research/charrecord-decode.md.");
        log("  --silent-channel was passed, so nothing here will answer. Drop it to play.");
    }

    // **One set of fields per channel process**, shared by every connection on it. Mobs
    // keep their positions when a player walks away, a field keeps running with nobody
    // watching, and a second player joining sees the same world as the first.
    // `crate::fields`.
    let fields = Arc::new(crate::fields::Fields::new());

    // **The world link**, for anything that has to cross channels: the party registry, party
    // chat, an invite to a character on the other channel. One TCP connection to
    // `maplecw-chat`, dialled in the background and re-dialled for the life of the process;
    // a channel that cannot reach the hub says so once and runs alone. `crate::link`.
    match config.link {
        Some(addr) => {
            let link = crate::link::Link::connect(addr, config.channel_id, crate::session::worldlink::link_handler(fields.clone()));
            crate::link::install(link);
            log(&format!("world link: dialling the hub at {addr} (--link none runs this channel alone)"));
        }
        None => log("world link: OFF (--link none); parties and party chat stay on this channel"),
    }

    let mut nth = 0u64;
    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                nth += 1;
                let store = store.clone();
                let config = config.clone();
                let fields = fields.clone();
                std::thread::spawn(move || {
                    let peer = stream
                        .peer_addr()
                        .map(|a| a.to_string())
                        .unwrap_or_else(|_| "unknown".to_string());
                    let peer = format!("ch{} #{nth} {peer}", config.channel_id);
                    log(&format!("connection from {peer}"));
                    match connection(stream, store, config, fields) {
                        Ok(Close::Client) => log(&format!("{peer} closed")),
                        // **The one line that says the server did it.** Every deliberate
                        // disconnect on this channel comes through here, in these words, so
                        // "did we drop them or did they drop?" is a grep rather than an
                        // inference from a missing line.
                        Ok(Close::Server(why)) => log(&format!(
                            "{peer} DISCONNECTED BY THE SERVER: {why}"
                        )),
                        // Not server-initiated: the socket failed or the client sent
                        // something unframeable. Left distinct on purpose - os error 10054 is
                        // the client crashing, and calling that a server disconnect would
                        // bury the real ones.
                        Err(e) => log(&format!("{peer} ended: {e}")),
                    }
                });
            }
            Err(e) => log(&format!("accept failed: {e}")),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The clock two players share is one clock.** This is the whole of the 2026-09-14 drop
    /// fix: every connection reads `now_ms` from [`process_origin`], and if two reads returned
    /// two different origins, a drop stamped by one player and swept by another would be timed
    /// against clocks that disagree - which is exactly what emptied map 30 within one tick when
    /// the origin was a per-connection `Instant::now()`.
    #[test]
    fn the_channel_clock_is_one_shared_origin() {
        let first = process_origin();
        let second = process_origin();
        assert_eq!(first, second, "two connections must measure now_ms from the same instant");
        // And a moment later it still has not moved: the origin is fixed, not re-sampled.
        std::thread::sleep(std::time::Duration::from_millis(5));
        assert_eq!(process_origin(), first);
        // now_ms is elapsed since that fixed origin, so it only grows.
        let a = process_origin().elapsed().as_millis();
        std::thread::sleep(std::time::Duration::from_millis(5));
        assert!(process_origin().elapsed().as_millis() >= a);
    }

    /// **The log rolls at the limit and keeps five.** The sink is driven directly with its
    /// byte count set just under the line, because writing 50 MB in a unit test is not a
    /// test of anything. Six rolls: `.1` is the newest, `.5` the oldest, the sixth is gone.
    #[test]
    fn the_log_file_rolls_at_the_limit_and_keeps_five() {
        let dir = std::env::temp_dir().join(format!("maplecw-logroll-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("world-ch0.log");
        let mut sink = LogSink::open(&path).unwrap();
        sink.write_line("first line, before any roll");
        for n in 1..=6 {
            sink.written = LOG_ROTATE_BYTES - 1;
            sink.write_line(&format!("line that forces roll {n}"));
        }
        let live = std::fs::read_to_string(&path).unwrap();
        assert!(live.contains("rolled at"), "the fresh file says it was rolled: {live}");
        assert!(live.contains("forces roll 6"), "the line that forced the roll goes in the NEW file");
        for n in 1..=LOG_KEEP {
            assert!(path.with_extension(format!("log.{n}")).exists(), "roll .{n} kept");
        }
        assert!(!path.with_extension(format!("log.{}", LOG_KEEP + 1)).exists(), "the sixth is deleted");
        // Six rolls, five kept: the file that held the very first line was rolled first and
        // is the one that fell off the end. `.5`, the oldest survivor, holds roll 1.
        let oldest = std::fs::read_to_string(path.with_extension(format!("log.{LOG_KEEP}"))).unwrap();
        assert!(oldest.contains("forces roll 1"), "{oldest}");
        let newest = std::fs::read_to_string(path.with_extension("log.1")).unwrap();
        assert!(newest.contains("forces roll 5"), "{newest}");
        let everything: String = (1..=LOG_KEEP)
            .map(|n| std::fs::read_to_string(path.with_extension(format!("log.{n}"))).unwrap())
            .chain(std::iter::once(live))
            .collect();
        assert!(!everything.contains("first line, before any roll"), "the sixth-oldest is gone");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The chatter set is exactly the three, and direction matters: an OUTBOUND 0x0070 is an
    /// InventoryOperation and must never be summarised away.
    #[test]
    fn chatter_is_three_packets_and_outbound_0x0070_is_not_one_of_them() {
        assert!(is_chatter(true, net::mobmove::MOB_MOVE_REQUEST));
        assert!(is_chatter(true, 0x0070));
        assert!(is_chatter(false, net::mobmove::MOB_CTRL_ACK));
        assert!(!is_chatter(false, 0x0070), "the inventory operation");
        assert!(!is_chatter(true, net::usermove::CLIENT_USER_MOVE), "user moves have been evidence too often");
        assert!(!is_chatter(false, net::opcode::SET_FIELD));
    }
}
