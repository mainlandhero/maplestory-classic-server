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
use net::{Direction, Framer, MapleCipher};
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

fn send(
    stream: &mut TcpStream,
    tx: &mut Framer<MapleCipher>,
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

fn connection(mut stream: TcpStream, store: Arc<Store>, config: Arc<Config>) -> std::io::Result<()> {
    stream.set_nodelay(true)?;

    // NOT the login greeting. A channel connection has `conn+0x48 == 0`, so the client
    // skips the greeting's two gated blocks - send it the login greeting and it reads `G`
    // from where `A` sits, gets 0, and raises "The client is outdated". See
    // `net::handshake::channel_greeting` and docs/transport.md.
    let hello = channel_greeting(CLIENT_TX_IV, CLIENT_RX_IV);
    stream.write_all(&hello)?;
    log(&format!("-> channel greeting, {} bytes (no A..F, no version block)", hello.len()));

    // AES-256-OFB, the same as the login connection. **Measured, 2026-08-19.**
    //
    // `docs/transport.md` said a channel used the byte subtract (`FUN_1406ef9f0`), selected
    // by `conn+0x48`. It does not: the two packets the client sent on its first channel
    // connection decode cleanly under AES and under nothing else. Packet 1 came out as
    // `0x0070` with the identical body the client sends on the login connection, and packet
    // 2 as `0x007D` carrying character id 204 and the machine's MAC. Reading the client had
    // said otherwise; the wire is what settled it.
    log("cipher: AES-256-OFB, same as login - measured from the first channel packets");
    let mut rx =
        Framer::new(MapleCipher::new(CLIENT_TX_IV.to_le_bytes(), Direction::ClientToServer));
    let mut tx =
        Framer::new(MapleCipher::new(CLIENT_RX_IV.to_le_bytes(), Direction::ServerToClient));

    let mut session = Session::new(store, config.clone());
    for reply in session.on_connect() {
        send(&mut stream, &mut tx, reply.opcode, &reply.packet(), &reply.what)?;
    }

    // No read timeout. The login server has one so it can re-send the startup gate to a
    // quiet client; there is nothing to nudge a quiet client with here, and waking up to
    // send nothing is just a wakeup.
    let mut buf = [0u8; 8192];
    loop {
        let read = match stream.read(&mut buf) {
            Ok(0) => return Ok(()),
            Ok(n) => n,
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
                    "   {} is not answered yet{} - see crates/world/src/session.rs",
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
    log("This stage is UNDECODED: packets are logged and not answered. See");
    log("  crates/world/src/session.rs before adding a reply.");

    let mut nth = 0u64;
    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                nth += 1;
                let store = store.clone();
                let config = config.clone();
                std::thread::spawn(move || {
                    let peer = stream
                        .peer_addr()
                        .map(|a| a.to_string())
                        .unwrap_or_else(|_| "unknown".to_string());
                    let peer = format!("ch{} #{nth} {peer}", config.channel_id);
                    log(&format!("connection from {peer}"));
                    match connection(stream, store, config) {
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
