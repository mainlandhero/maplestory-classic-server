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
    for (id, addr) in config.world.channels.iter().enumerate() {
        log(&format!("channel {id} advertised to the client at {addr}"));
        if addr.ip().is_loopback() && !config.bind.ip().is_loopback() {
            log("  WARNING: bind is not loopback but this channel is advertised as one, so");
            log("  an off-box client will be sent back to itself. See docs/deployment.md.");
        }
    }
    log(&format!("database {}", config.db_path.display()));
    log(&format!(
        "world {} id {} with {} channel(s)",
        config.world.name, config.world.id, config.world.channel_count()
    ));
    let characters = store.character_count(account.id, config.world.id).unwrap_or(0);
    log(&format!(
        "serving every connection as account {:?} (id {}), {characters} character(s) stored",
        account.name, account.id
    ));
    log("NOT AUTHENTICATED: the game socket carries no credentials, so anyone who");
    log("  connects is served as that account. See docs/launcher.md.");

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
                    let peer = stream
                        .peer_addr()
                        .map(|a| a.to_string())
                        .unwrap_or_else(|_| "unknown".to_string());
                    let peer = format!("#{nth} {peer}");
                    log(&format!("connection from {peer}"));
                    match connection(stream, store, config, account) {
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

    let mut session = Session::new(store, config, account);
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
