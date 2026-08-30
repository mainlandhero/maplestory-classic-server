//! One channel's socket loop: framing, logging, one thread per connection.
//!
//! This deliberately mirrors `login::server` rather than sharing it. The two loops look
//! alike today and are about to stop: login nudges a quiet client with the startup gate on
//! a read timeout, and a channel must not; login greets and immediately sends a packet, and
//! a channel greets and waits. Factoring them together now would mean threading the
//! differences back out through a trait for the sake of forty shared lines.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

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
    println!("{h:02}:{m:02}:{s:02}.{:03} {msg}", now.subsec_millis());
    let _ = std::io::stdout().flush();
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

fn send(
    stream: &mut TcpStream,
    tx: &mut Framer<ByteShiftCipher>,
    opcode: u16,
    packet: &[u8],
    what: &str,
) -> std::io::Result<()> {
    let framed = tx.frame(packet);
    stream.write_all(&framed)?;
    log(&format!("-> {} {what}", label(opcode)));
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
) -> std::io::Result<()> {
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
    let mut session = match stream.peer_addr() {
        Ok(addr) => Session::joining(store, config.clone(), fields).with_peer_addr(addr),
        Err(_) => Session::joining(store, config.clone(), fields),
    };
    for reply in session.on_connect() {
        send(&mut stream, &mut tx, reply.opcode, &reply.packet(), &reply.what)?;
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
    let started = std::time::Instant::now();
    let mut buf = [0u8; 8192];
    loop {
        let read = match stream.read(&mut buf) {
            Ok(0) => return Ok(()),
            Ok(n) => n,
            Err(e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                // Nothing arrived. Give the session the clock and send whatever it owes.
                let now_ms = started.elapsed().as_millis() as u64;
                for reply in session.tick(now_ms) {
                    send(&mut stream, &mut tx, reply.opcode, &reply.packet(), &reply.what)?;
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
            log(&format!(
                "<- {}, {} byte body {}",
                label(opcode),
                payload.len(),
                body_hex(opcode, payload)
            ));

            if opcode == CLIENT_MIGRATION_HELLO {
                describe_hello(&mut session, payload);
            }

            let replies = session.handle(&body);
            if replies.is_empty() {
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
            for reply in replies {
                send(&mut stream, &mut tx, reply.opcode, &reply.packet(), &reply.what)?;
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
    let config = Arc::new(config);
    let store = Arc::new(
        Store::open(&config.db_path)
            .map_err(|e| std::io::Error::other(format!("{}: {e}", config.db_path.display())))?,
    );

    let listener = TcpListener::bind(config.bind)?;
    log(&format!(
        "world {} channel {} listening on {}",
        config.world_id, config.channel_id, config.bind
    ));
    log(&format!("database {}", config.db_path.display()));
    log("NOT AUTHENTICATED: a migration seed is a u32, so it identifies a pending");
    log("  migration rather than proving who is on the far end. It is single-use.");
    if config.set_field_probe {
        log("SET-FIELD PROBE IS ON - which by now means 'the channel answers at all'. The");
        log("  flag is a misnomer kept for the launch line: without it Session::handle");
        log("  returns nothing for every packet, so the migration hello goes unanswered and");
        log("  the client freezes on 'Connecting...'.");
        log("  With it on, this channel answers: the migration hello with a SetField");
        log("  carrying the character's real record (presence[0] the stat block, presence[2]");
        log("  the equipped list), 0x00DC with that field's NPCs and mobs, 0x00D1 with the");
        log("  portal's destination, 0x00E7 with the !map GM command, and 0x0151 with a");
        log("  script Say, 0x0104 with the NPC shop, and 0x00D2 with a channel migration.");
        log("  CONFIRMED on a real client: the dressed character, the bag and its 125 slots,");
        log("  NPC dialogue, mobs spawning and moving, an unequip that survives a map change.");
        log("  NOT yet seen on screen: the shop counter, the quest journal, the channel");
        log("  switch. Dropping an item is refused on purpose - there is nowhere to put it.");
    } else {
        log("Packets are logged and NOT ANSWERED. The stage is no longer undecoded:");
        log("  SetField is inbound 0x01A0, confirmed on a live client, and its 33-byte");
        log("  head is in research/msexe-stage-setfield.md. What is missing is the");
        log("  character record it must carry - see research/charrecord-decode.md.");
        log("  Pass --set-field-probe to send the head alone as a delivery probe.");
    }

    // **One set of fields per channel process**, shared by every connection on it. Mobs
    // keep their positions when a player walks away, a field keeps running with nobody
    // watching, and a second player joining sees the same world as the first.
    // `crate::fields`.
    let fields = Arc::new(crate::fields::Fields::new());

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
