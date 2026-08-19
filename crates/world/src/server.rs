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

use net::handshake::{greeting, CLIENT_RX_IV, CLIENT_TX_IV};
use net::names::{body_hex, label, opcode_name};
use net::{Direction, Framer, MapleCipher};
use store::Store;

use crate::config::Config;
use crate::session::{seed_candidates, Session, CLIENT_MIGRATION_HELLO};

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

    // The same 48 bytes the login server sends. It is the transport's greeting, not
    // login's, which is why it lives in `net::handshake` - if a channel ever needs
    // different bytes that will be a finding, not a config option.
    let hello = greeting(CLIENT_TX_IV, CLIENT_RX_IV);
    stream.write_all(&hello)?;
    log(&format!("-> greeting, {} bytes", hello.len()));

    let mut rx = Framer::new(MapleCipher::new(CLIENT_TX_IV.to_le_bytes(), Direction::ClientToServer));
    let mut tx = Framer::new(MapleCipher::new(CLIENT_RX_IV.to_le_bytes(), Direction::ServerToClient));

    let mut session = Session::new(store, config);
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

/// Search a `0x007D` body for the migration seed and try to claim it.
///
/// The field's position is unknown, so this is a search over every place a
/// `(key, length, word)` triple could sit. Every candidate is logged with its offset, and
/// the first one the store recognises wins - which means a run either finds the seed and
/// says where it was, or lists what it tried. Both outcomes are useful; a silent failure
/// would not be.
fn describe_hello(session: &mut Session, payload: &[u8]) {
    let candidates = seed_candidates(payload);
    log(&format!(
        "   MIGRATION HELLO: {} candidate seed position(s) in {} bytes",
        candidates.len(),
        payload.len()
    ));
    for &(at, seed) in &candidates {
        log(&format!("     offset {at:3}: {seed:#010x}"));
    }
    for &(at, seed) in &candidates {
        let note = session.claim(seed);
        if session.claimed().is_some() {
            log(&format!("   SEED FOUND AT OFFSET {at} - {note}"));
            return;
        }
    }
    log("   no candidate matched a pending migration - the seed is elsewhere in this body, \
         or the tail transform differs in this direction. Read the body hex above.");
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
