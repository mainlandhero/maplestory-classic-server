//! The **world link**: how the channel processes talk to each other, without the database.
//!
//! The owner, 2026-09-14: *"This message should be broadcasted to all party members across
//! channels and maps as long as the client is online."* Then: *"Do not use the database as
//! a shared bus."* And: *"We can have a chat server hosted on 8483, which handles cross
//! channel chat functions if necessary."*
//!
//! # Shape
//!
//! One process per channel stays as it is (`crate::server`, `docs/deployment.md`). A third
//! process, **`maplecw-chat`** on **8483**, is the hub: every channel process dials it and
//! keeps one TCP connection open for its life. The hub does three things and nothing else:
//!
//! * **relays** character-addressed packets ([`Frame::Deliver`]) to every other channel,
//!   where the one that hosts the character posts it into its own bus;
//! * **keeps the directory** of who is online where ([`Frame::Online`] / [`Frame::Offline`]),
//!   so a newcomer gets the whole picture on [`Frame::Hello`] and a channel that dies takes
//!   its characters off it;
//! * **serialises party requests** ([`Frame::PartyRequest`]): it applies each one to its own
//!   `Parties` and echoes it to *every* channel - the sender included - in one order, so
//!   every channel's replica applies the same sequence and agrees. A channel that connects
//!   late gets [`Frame::PartySnapshot`] first.
//!
//! No frame carries game credentials; the link is server-to-server on the box's own
//! addresses, and the channel socket still carries none either.
//!
//! # Wire
//!
//! `u32 length` then `u8 kind` then the fields, written with the same `PacketWriter` the
//! game packets use (so a `str` is `u16 len + bytes`). Unknown kinds are skipped by length.
//!
//! # What happens when the hub is down
//!
//! The client reconnects with a backoff and re-announces its presence on every connect.
//! While it is down, [`Link::send`] drops frames and says so once per outage; party
//! requests fall back to the channel's local registry (`Session::run_party_request`), which
//! is exactly the behaviour this replaced. Nothing blocks a game connection on the link.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{mpsc, Arc, Mutex};

use net::packet::{PacketReader, PacketWriter};

/// The port the owner named for the chat/world hub.
pub const DEFAULT_HUB_PORT: u16 = 8483;

/// The largest frame the reader will accept. A party chat line is under a kilobyte; this is
/// generous and still refuses a stray connection's nonsense before it allocates.
pub const MAX_FRAME: u32 = 64 * 1024;

/// One message on the link. See the module docs for who sends what.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Frame {
    /// A channel process introducing itself. The hub answers with the directory and the
    /// party snapshot.
    Hello { channel: u32 },
    /// `character` is playing on `channel` under `name`, for `account`.
    Online { character: u32, name: String, account: u32, channel: u32, map: u32 },
    /// `character` left `channel` (log out, channel change, disconnect).
    Offline { character: u32, channel: u32 },
    /// A finished packet for `character`, wherever they are. `what` is the log line.
    Deliver { character: u32, opcode: u16, body: Vec<u8>, what: String },
    /// A party request to be applied everywhere in hub order. `now` is the wall-clock
    /// second the sender stamped, so invite expiry ages identically on every replica.
    PartyRequest { actor: u32, now: i64, request: crate::party::Request },
    /// Every party the hub knows, for a channel that just connected.
    PartySnapshot { parties: Vec<crate::party::Party>, next_id: u32 },
    /// A Maple Chat change to be applied everywhere in hub order - [`crate::messenger`].
    /// The owner, 2026-09-24: *"Maple Chat should work cross channel, please use the hub code."*
    MessengerRequest { actor: u32, request: crate::messenger::Request },
    /// Every Maple Chat room the hub knows, for a channel that just connected.
    MessengerSnapshot { rooms: Vec<crate::messenger::Room>, next_id: u32 },
}

mod kind {
    pub const HELLO: u8 = 1;
    pub const ONLINE: u8 = 2;
    pub const OFFLINE: u8 = 3;
    pub const DELIVER: u8 = 4;
    pub const PARTY_REQUEST: u8 = 5;
    pub const PARTY_SNAPSHOT: u8 = 6;
    pub const MESSENGER_REQUEST: u8 = 7;
    pub const MESSENGER_SNAPSHOT: u8 = 8;
}

mod mreq {
    pub const OPEN: u8 = 0;
    pub const ENTER: u8 = 1;
    pub const LEAVE: u8 = 2;
    pub const DISCONNECT: u8 = 3;
}

fn write_seat(w: &mut PacketWriter, s: &net::messenger::Seat) {
    w.u32(s.character_id);
    w.str(&s.name);
    w.u32(s.look.len() as u32);
    w.bytes(&s.look);
}

fn read_seat(r: &mut PacketReader) -> Option<net::messenger::Seat> {
    let character_id = r.u32().ok()?;
    let name = r.str().ok()?;
    let n = r.u32().ok()? as usize;
    let look = r.bytes(n).ok()?.to_vec();
    Some(net::messenger::Seat { character_id, name, look })
}

fn write_messenger_request(w: &mut PacketWriter, r: &crate::messenger::Request) {
    use crate::messenger::Request;
    match r {
        Request::Open { seat } => {
            w.u8(mreq::OPEN);
            write_seat(w, seat);
        }
        Request::Enter { room, seat } => {
            w.u8(mreq::ENTER);
            w.u32(*room);
            write_seat(w, seat);
        }
        Request::Leave { room } => {
            w.u8(mreq::LEAVE);
            w.u32(*room);
        }
        Request::Disconnect => {
            w.u8(mreq::DISCONNECT);
        }
    }
}

fn read_messenger_request(r: &mut PacketReader) -> Option<crate::messenger::Request> {
    use crate::messenger::Request;
    Some(match r.u8().ok()? {
        mreq::OPEN => Request::Open { seat: read_seat(r)? },
        mreq::ENTER => {
            let room = r.u32().ok()?;
            Request::Enter { room, seat: read_seat(r)? }
        }
        mreq::LEAVE => Request::Leave { room: r.u32().ok()? },
        mreq::DISCONNECT => Request::Disconnect,
        _ => return None,
    })
}

mod req {
    pub const CREATE: u8 = 0;
    pub const INVITE: u8 = 1;
    pub const ACCEPT: u8 = 2;
    pub const DECLINE: u8 = 3;
    pub const LEAVE: u8 = 4;
    pub const EXPEL: u8 = 5;
    pub const CHANGE_LEADER: u8 = 6;
    pub const SET_PICKUP_RIGHTS: u8 = 7;
    /// `u32 successor` after it; 0 is "none" (a character id is never 0 -
    /// `store::FIRST_CHARACTER_ID` is 200); then `u8 last_online` (1: no other member of the
    /// party is online anywhere, so the party is disbanded whoever the actor is).
    pub const DISCONNECT: u8 = 8;
}

fn write_request(w: &mut PacketWriter, r: &crate::party::Request) {
    use crate::party::{DeclineReason, Request};
    match r {
        Request::Create { name } => {
            w.u8(req::CREATE);
            w.str(name);
        }
        Request::Invite { target } => {
            w.u8(req::INVITE);
            w.u32(*target);
        }
        Request::Accept { party } => {
            w.u8(req::ACCEPT);
            w.u32(*party);
        }
        Request::Decline { party, reason } => {
            w.u8(req::DECLINE);
            w.u32(*party);
            w.u8(match reason {
                DeclineReason::Refused => 0,
                DeclineReason::Blocking => 1,
                DeclineReason::Busy => 2,
                DeclineReason::AlreadyInvited => 3,
            });
        }
        Request::Leave => {
            w.u8(req::LEAVE);
        }
        Request::Expel { target } => {
            w.u8(req::EXPEL);
            w.u32(*target);
        }
        Request::ChangeLeader { target } => {
            w.u8(req::CHANGE_LEADER);
            w.u32(*target);
        }
        Request::SetPickupRights { rights } => {
            w.u8(req::SET_PICKUP_RIGHTS);
            w.u8(*rights);
        }
        Request::Disconnect { successor, last_online } => {
            w.u8(req::DISCONNECT);
            w.u32(successor.unwrap_or(0));
            w.u8(u8::from(*last_online));
        }
    }
}

fn read_request(r: &mut PacketReader) -> Option<crate::party::Request> {
    use crate::party::{DeclineReason, Request};
    Some(match r.u8().ok()? {
        req::CREATE => Request::Create { name: r.str().ok()? },
        req::INVITE => Request::Invite { target: r.u32().ok()? },
        req::ACCEPT => Request::Accept { party: r.u32().ok()? },
        req::DECLINE => {
            let party = r.u32().ok()?;
            let reason = match r.u8().ok()? {
                0 => DeclineReason::Refused,
                1 => DeclineReason::Blocking,
                2 => DeclineReason::Busy,
                3 => DeclineReason::AlreadyInvited,
                _ => return None,
            };
            Request::Decline { party, reason }
        }
        req::LEAVE => Request::Leave,
        req::EXPEL => Request::Expel { target: r.u32().ok()? },
        req::CHANGE_LEADER => Request::ChangeLeader { target: r.u32().ok()? },
        req::SET_PICKUP_RIGHTS => Request::SetPickupRights { rights: r.u8().ok()? },
        req::DISCONNECT => {
            let successor = r.u32().ok()?;
            let last_online = r.u8().ok()? != 0;
            Request::Disconnect { successor: (successor != 0).then_some(successor), last_online }
        }
        _ => return None,
    })
}

impl Frame {
    /// The frame with its length prefix, ready to write.
    pub fn encode(&self) -> Vec<u8> {
        let mut w = PacketWriter::new();
        match self {
            Frame::Hello { channel } => {
                w.u8(kind::HELLO);
                w.u32(*channel);
            }
            Frame::Online { character, name, account, channel, map } => {
                w.u8(kind::ONLINE);
                w.u32(*character);
                w.str(name);
                w.u32(*account);
                w.u32(*channel);
                w.u32(*map);
            }
            Frame::Offline { character, channel } => {
                w.u8(kind::OFFLINE);
                w.u32(*character);
                w.u32(*channel);
            }
            Frame::Deliver { character, opcode, body, what } => {
                w.u8(kind::DELIVER);
                w.u32(*character);
                w.u16(*opcode);
                w.u32(body.len() as u32);
                w.bytes(body);
                w.str(what);
            }
            Frame::PartyRequest { actor, now, request } => {
                w.u8(kind::PARTY_REQUEST);
                w.u32(*actor);
                w.i64(*now);
                write_request(&mut w, request);
            }
            Frame::MessengerRequest { actor, request } => {
                w.u8(kind::MESSENGER_REQUEST);
                w.u32(*actor);
                write_messenger_request(&mut w, request);
            }
            Frame::MessengerSnapshot { rooms, next_id } => {
                w.u8(kind::MESSENGER_SNAPSHOT);
                w.u32(*next_id);
                w.u32(rooms.len() as u32);
                for room in rooms {
                    w.u32(room.id);
                    for seat in &room.seats {
                        match seat {
                            Some(s) => {
                                w.u8(1);
                                write_seat(&mut w, s);
                            }
                            None => {
                                w.u8(0);
                            }
                        }
                    }
                }
            }
            Frame::PartySnapshot { parties, next_id } => {
                w.u8(kind::PARTY_SNAPSHOT);
                w.u32(*next_id);
                w.u32(parties.len() as u32);
                for p in parties {
                    w.u32(p.id);
                    w.str(&p.name);
                    w.u32(p.leader);
                    w.u8(p.pickup_rights);
                    w.u8(p.members.len() as u8);
                    for m in &p.members {
                        w.u32(*m);
                    }
                }
            }
        }
        let payload = w.into_vec();
        let mut out = Vec::with_capacity(4 + payload.len());
        out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        out.extend_from_slice(&payload);
        out
    }

    /// Decode one payload (the bytes after the length prefix). `None` for an unknown kind or
    /// a short body - the caller skips the frame by its length either way.
    pub fn decode(payload: &[u8]) -> Option<Frame> {
        let mut r = PacketReader::new(payload);
        Some(match r.u8().ok()? {
            kind::HELLO => Frame::Hello { channel: r.u32().ok()? },
            kind::ONLINE => Frame::Online {
                character: r.u32().ok()?,
                name: r.str().ok()?,
                account: r.u32().ok()?,
                channel: r.u32().ok()?,
                map: r.u32().ok()?,
            },
            kind::OFFLINE => Frame::Offline { character: r.u32().ok()?, channel: r.u32().ok()? },
            kind::DELIVER => {
                let character = r.u32().ok()?;
                let opcode = r.u16().ok()?;
                let n = r.u32().ok()? as usize;
                let body = r.bytes(n).ok()?.to_vec();
                let what = r.str().ok()?;
                Frame::Deliver { character, opcode, body, what }
            }
            kind::PARTY_REQUEST => {
                let actor = r.u32().ok()?;
                let now = r.i64().ok()?;
                let request = read_request(&mut r)?;
                Frame::PartyRequest { actor, now, request }
            }
            kind::MESSENGER_REQUEST => {
                let actor = r.u32().ok()?;
                let request = read_messenger_request(&mut r)?;
                Frame::MessengerRequest { actor, request }
            }
            kind::MESSENGER_SNAPSHOT => {
                let next_id = r.u32().ok()?;
                let n = r.u32().ok()? as usize;
                let mut rooms = Vec::with_capacity(n.min(1024));
                for _ in 0..n {
                    let id = r.u32().ok()?;
                    let mut room = crate::messenger::Room { id, ..Default::default() };
                    for slot in room.seats.iter_mut() {
                        if r.u8().ok()? != 0 {
                            *slot = Some(read_seat(&mut r)?);
                        }
                    }
                    rooms.push(room);
                }
                Frame::MessengerSnapshot { rooms, next_id }
            }
            kind::PARTY_SNAPSHOT => {
                let next_id = r.u32().ok()?;
                let n = r.u32().ok()? as usize;
                let mut parties = Vec::with_capacity(n.min(1024));
                for _ in 0..n {
                    let id = r.u32().ok()?;
                    let name = r.str().ok()?;
                    let leader = r.u32().ok()?;
                    let pickup_rights = r.u8().ok()?;
                    let count = r.u8().ok()? as usize;
                    let mut members = Vec::with_capacity(count);
                    for _ in 0..count {
                        members.push(r.u32().ok()?);
                    }
                    parties.push(crate::party::Party { id, name, leader, members, pickup_rights });
                }
                Frame::PartySnapshot { parties, next_id }
            }
            _ => return None,
        })
    }
}

/// Read one length-prefixed frame. `Ok(None)` at a clean end of stream.
fn read_frame(stream: &mut impl Read) -> std::io::Result<Option<Vec<u8>>> {
    let mut len = [0u8; 4];
    match stream.read_exact(&mut len) {
        Ok(()) => {}
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    }
    let n = u32::from_le_bytes(len);
    if n > MAX_FRAME {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("frame of {n} bytes")));
    }
    let mut body = vec![0u8; n as usize];
    stream.read_exact(&mut body)?;
    Ok(Some(body))
}

// ---------------------------------------------------------------------------------------
// The client side: one per channel process
// ---------------------------------------------------------------------------------------

/// Who is online where, as this process last heard from the hub.
#[derive(Debug, Default, Clone)]
pub struct Directory {
    pub by_id: HashMap<u32, Entry>,
}

/// One online character.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub account: u32,
    pub channel: u32,
    /// The map the character was last announced on. Re-announced on every field entry, so
    /// this follows a portal walk within a few hundred milliseconds. `!track` reads it.
    pub map: u32,
}

impl Directory {
    /// The character called `name`, case-insensitively, if anyone by that name is online.
    pub fn find_by_name(&self, name: &str) -> Option<(u32, &Entry)> {
        self.by_id.iter().find(|(_, e)| e.name.eq_ignore_ascii_case(name)).map(|(id, e)| (*id, e))
    }
}

/// What the link hands the process when a frame arrives.
pub type Handler = Arc<dyn Fn(Frame) + Send + Sync>;

/// A channel process's connection to the hub. Reconnects on its own; never blocks a caller.
pub struct Link {
    channel: u32,
    tx: Mutex<Option<mpsc::Sender<Vec<u8>>>>,
    connected: AtomicBool,
    /// The characters this process hosts, re-announced on every connect.
    local: Mutex<HashMap<u32, (String, u32, u32)>>,
    dropped: AtomicU64,
    pub directory: Mutex<Directory>,
}

impl Link {
    /// Dial `addr` in the background and keep dialling. `handler` runs on the reader thread
    /// for every frame that arrives, so it must be quick and must not block on the game loop.
    pub fn connect(addr: SocketAddr, channel: u32, handler: Handler) -> Arc<Link> {
        let link = Arc::new(Link {
            channel,
            tx: Mutex::new(None),
            connected: AtomicBool::new(false),
            local: Mutex::new(HashMap::new()),
            dropped: AtomicU64::new(0),
            directory: Mutex::new(Directory::default()),
        });
        let me = link.clone();
        std::thread::Builder::new()
            .name("world-link".into())
            .spawn(move || me.run(addr, handler))
            .expect("spawn the world-link thread");
        link
    }

    /// Whether the hub is reachable right now.
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }

    /// Queue a frame for the hub. Returns `false` (and drops it) while disconnected.
    pub fn send(&self, frame: &Frame) -> bool {
        let guard = self.tx.lock().unwrap_or_else(|e| e.into_inner());
        match guard.as_ref() {
            Some(tx) if tx.send(frame.encode()).is_ok() => true,
            _ => {
                let n = self.dropped.fetch_add(1, Ordering::Relaxed);
                if n == 0 {
                    crate::server::log("   link: the hub is not connected; frames are being dropped (this is said once per outage)");
                }
                false
            }
        }
    }

    /// Remember that `character` plays here, on `map`, and tell the hub. Called on every
    /// field entry, so a map change is a re-announcement and the directory follows it.
    pub fn announce_online(&self, character: u32, name: &str, account: u32, map: u32) {
        self.local
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .insert(character, (name.to_string(), account, map));
        self.send(&Frame::Online { character, name: name.to_string(), account, channel: self.channel, map });
    }

    /// **Everyone online, everywhere this process can see**: the hub's directory (the other
    /// channels) plus this process's own announcements, which the hub echoes to everyone
    /// *else* and so are never in the replica. Sorted by name. The owner, 2026-09-14: *"`!online` -
    /// available to everyone, list all characters that are currently online across all
    /// channels."*
    pub fn everyone(&self) -> Vec<(u32, Entry)> {
        let mut all: HashMap<u32, Entry> = self.directory.lock().unwrap_or_else(|e| e.into_inner()).by_id.clone();
        for (id, (name, account, map)) in self.local.lock().unwrap_or_else(|e| e.into_inner()).iter() {
            all.insert(*id, Entry { name: name.clone(), account: *account, channel: self.channel, map: *map });
        }
        let mut v: Vec<(u32, Entry)> = all.into_iter().collect();
        v.sort_by(|a, b| a.1.name.to_lowercase().cmp(&b.1.name.to_lowercase()));
        v
    }

    /// The character called `name`, case-insensitively, wherever they are - this channel or
    /// another. `None` when nobody by that name is online.
    pub fn find(&self, name: &str) -> Option<(u32, Entry)> {
        self.everyone().into_iter().find(|(_, e)| e.name.eq_ignore_ascii_case(name))
    }

    /// Whether `character` was announced from this process and not taken back.
    pub fn hosts(&self, character: u32) -> bool {
        self.local.lock().unwrap_or_else(|e| e.into_inner()).contains_key(&character)
    }

    /// `character` no longer plays here.
    pub fn announce_offline(&self, character: u32) {
        let was = self.local.lock().unwrap_or_else(|e| e.into_inner()).remove(&character).is_some();
        if was {
            self.send(&Frame::Offline { character, channel: self.channel });
        }
    }

    /// A frame for a character: `true` when it was handed to the hub.
    pub fn deliver(&self, character: u32, reply: &crate::session::Reply) -> bool {
        self.send(&Frame::Deliver {
            character,
            opcode: reply.opcode,
            body: reply.body.clone(),
            what: reply.what.clone(),
        })
    }

    fn run(self: Arc<Self>, addr: SocketAddr, handler: Handler) {
        let mut backoff_ms = 500u64;
        loop {
            match TcpStream::connect_timeout(&addr, std::time::Duration::from_secs(3)) {
                Ok(stream) => {
                    backoff_ms = 500;
                    let _ = stream.set_nodelay(true);
                    crate::server::log(&format!("   link: connected to the hub at {addr} as channel {}", self.channel));
                    self.serve(stream, &handler);
                    crate::server::log(&format!("   link: the hub at {addr} went away; reconnecting"));
                }
                Err(e) => {
                    if backoff_ms == 500 {
                        crate::server::log(&format!(
                            "   link: cannot reach the hub at {addr} ({e}); retrying. Parties and chat stay per-channel until it answers"
                        ));
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(backoff_ms));
            backoff_ms = (backoff_ms * 2).min(10_000);
        }
    }

    fn serve(&self, stream: TcpStream, handler: &Handler) {
        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        let mut writer = match stream.try_clone() {
            Ok(w) => w,
            Err(_) => return,
        };
        let writer_thread = std::thread::spawn(move || {
            for frame in rx {
                if writer.write_all(&frame).is_err() {
                    break;
                }
            }
        });
        *self.tx.lock().unwrap_or_else(|e| e.into_inner()) = Some(tx);
        self.connected.store(true, Ordering::Relaxed);
        self.dropped.store(0, Ordering::Relaxed);
        self.send(&Frame::Hello { channel: self.channel });
        let local: Vec<(u32, String, u32, u32)> = self
            .local
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .map(|(id, (n, a, m))| (*id, n.clone(), *a, *m))
            .collect();
        for (character, name, account, map) in local {
            self.send(&Frame::Online { character, name, account, channel: self.channel, map });
        }

        let mut reader = stream;
        loop {
            match read_frame(&mut reader) {
                Ok(Some(payload)) => {
                    if let Some(frame) = Frame::decode(&payload) {
                        self.note(&frame);
                        handler(frame);
                    }
                }
                Ok(None) => break,
                Err(_) => break,
            }
        }
        self.connected.store(false, Ordering::Relaxed);
        *self.tx.lock().unwrap_or_else(|e| e.into_inner()) = None;
        let _ = writer_thread.join();
    }

    /// Keep the directory replica in step before the handler sees the frame.
    fn note(&self, frame: &Frame) {
        let mut d = self.directory.lock().unwrap_or_else(|e| e.into_inner());
        match frame {
            Frame::Online { character, name, account, channel, map } => {
                d.by_id.insert(*character, Entry { name: name.clone(), account: *account, channel: *channel, map: *map });
            }
            Frame::Offline { character, channel } => {
                if d.by_id.get(character).is_some_and(|e| e.channel == *channel) {
                    d.by_id.remove(character);
                }
            }
            _ => {}
        }
    }
}

// ---------------------------------------------------------------------------------------
// The process's one link
// ---------------------------------------------------------------------------------------

static CURRENT: std::sync::OnceLock<Arc<Link>> = std::sync::OnceLock::new();

/// Make `link` the process's link. Once per process; a second call is ignored.
///
/// A process-wide handle rather than a field on `Fields`, so the link reaches every session
/// without a constructor change - and so the test suite, which never installs one, runs every
/// party and chat path in its local form.
pub fn install(link: Arc<Link>) {
    let _ = CURRENT.set(link);
}

/// The process's link, if one was installed and the hub is connected right now.
pub fn current() -> Option<&'static Arc<Link>> {
    CURRENT.get().filter(|l| l.is_connected())
}

/// The process's link whether or not it is connected - for presence bookkeeping, which has
/// to be remembered across an outage so a reconnect can re-announce it.
pub fn installed() -> Option<&'static Arc<Link>> {
    CURRENT.get()
}

// ---------------------------------------------------------------------------------------
// The hub: maplecw-chat
// ---------------------------------------------------------------------------------------

struct HubConn {
    tx: mpsc::Sender<Vec<u8>>,
    channel: Option<u32>,
}

#[derive(Default)]
struct HubState {
    conns: HashMap<u64, HubConn>,
    /// character -> (entry, the connection that announced it)
    directory: HashMap<u32, (Entry, u64)>,
    parties: crate::party::Parties,
    /// The world's Maple Chat rooms, applied here only so a late channel's snapshot is right.
    messengers: crate::messenger::Rooms,
}

impl HubState {
    fn broadcast(&self, except: Option<u64>, bytes: &[u8]) {
        for (id, c) in &self.conns {
            if Some(*id) != except {
                let _ = c.tx.send(bytes.to_vec());
            }
        }
    }

    fn send_to(&self, id: u64, bytes: Vec<u8>) {
        if let Some(c) = self.conns.get(&id) {
            let _ = c.tx.send(bytes);
        }
    }
}

/// Run the hub on `listener` forever, with no Discord status.
pub fn run_hub(listener: TcpListener) {
    run_hub_with_status(listener, None)
}

/// Run the hub on `listener` forever. `maplecw-chat` calls this; `status`, when given, is the
/// live server's Discord message (`crate::discordstatus`), refreshed from the hub's own roster.
pub fn run_hub_with_status(listener: TcpListener, status: Option<crate::discordstatus::Reporter>) {
    let state = Arc::new(Mutex::new(HubState { parties: crate::party::Parties::new(), ..Default::default() }));
    if let Some(mut reporter) = status {
        let state = state.clone();
        std::thread::spawn(move || {
            // A few seconds first, so the channels have dialled in before the first refresh.
            std::thread::sleep(std::time::Duration::from_secs(5));
            loop {
                let (roster, connected) = {
                    let s = state.lock().unwrap_or_else(|e| e.into_inner());
                    let roster: Vec<(u32, String)> = s.directory.values().map(|(e, _)| (e.channel, e.name.clone())).collect();
                    let connected: Vec<u32> = s.conns.values().filter_map(|c| c.channel).collect();
                    (roster, connected)
                };
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs() as i64)
                    .unwrap_or(0);
                let status = reporter.status(&roster, &connected, now);
                match reporter.publish(&crate::discordstatus::status_json(&status)) {
                    Ok(what) => crate::server::log(&format!(
                        "discord: {what} - {} player(s) on {} channel(s)",
                        roster.len(),
                        connected.len()
                    )),
                    Err(e) => crate::server::log(&format!("discord: {e}")),
                }
                std::thread::sleep(std::time::Duration::from_secs(crate::discordstatus::REFRESH_SECS));
            }
        });
    }
    let mut next = 1u64;
    for incoming in listener.incoming() {
        let Ok(stream) = incoming else { continue };
        let id = next;
        next += 1;
        let _ = stream.set_nodelay(true);
        let peer = stream.peer_addr().map(|a| a.to_string()).unwrap_or_default();
        let (tx, rx) = mpsc::channel::<Vec<u8>>();
        let mut writer = match stream.try_clone() {
            Ok(w) => w,
            Err(_) => continue,
        };
        std::thread::spawn(move || {
            for frame in rx {
                if writer.write_all(&frame).is_err() {
                    break;
                }
            }
        });
        state.lock().unwrap_or_else(|e| e.into_inner()).conns.insert(id, HubConn { tx, channel: None });
        crate::server::log(&format!("hub: connection #{id} from {peer}"));
        let state = state.clone();
        std::thread::spawn(move || {
            let mut reader = stream;
            while let Ok(Some(payload)) = read_frame(&mut reader) {
                let Some(frame) = Frame::decode(&payload) else { continue };
                hub_handle(&state, id, frame, &payload);
            }
            hub_drop(&state, id);
        });
    }
}

fn hub_handle(state: &Mutex<HubState>, from: u64, frame: Frame, raw: &[u8]) {
    let mut bytes = Vec::with_capacity(4 + raw.len());
    bytes.extend_from_slice(&(raw.len() as u32).to_le_bytes());
    bytes.extend_from_slice(raw);
    let mut s = state.lock().unwrap_or_else(|e| e.into_inner());
    match frame {
        Frame::Hello { channel } => {
            if let Some(c) = s.conns.get_mut(&from) {
                c.channel = Some(channel);
            }
            crate::server::log(&format!(
                "hub: connection #{from} is channel {channel}; sending {} online character(s) and {} party(ies)",
                s.directory.len(),
                s.parties.len()
            ));
            let entries: Vec<Vec<u8>> = s
                .directory
                .iter()
                .map(|(id, (e, _))| {
                    Frame::Online { character: *id, name: e.name.clone(), account: e.account, channel: e.channel, map: e.map }.encode()
                })
                .collect();
            for e in entries {
                s.send_to(from, e);
            }
            let snapshot = Frame::PartySnapshot { parties: s.parties.snapshot(), next_id: s.parties.next_id() }.encode();
            s.send_to(from, snapshot);
            let rooms = Frame::MessengerSnapshot { rooms: s.messengers.snapshot(), next_id: s.messengers.next_id() }.encode();
            s.send_to(from, rooms);
        }
        Frame::Online { character, name, account, channel, map } => {
            crate::server::log(&format!("hub: {name} ({character}) online on channel {channel}, map {map}"));
            s.directory.insert(character, (Entry { name, account, channel, map }, from));
            s.broadcast(Some(from), &bytes);
        }
        Frame::Offline { character, channel } => {
            if s.directory.get(&character).is_some_and(|(e, _)| e.channel == channel) {
                crate::server::log(&format!("hub: character {character} offline from channel {channel}"));
                s.directory.remove(&character);
            }
            s.broadcast(Some(from), &bytes);
        }
        Frame::Deliver { character, opcode, .. } => {
            // To the channel that hosts the character when the directory knows it, else to
            // everyone but the sender - the host tries its bus and the rest ignore it.
            let host = s.directory.get(&character).map(|(_, conn)| *conn);
            match host {
                Some(conn) if conn != from => s.send_to(conn, bytes),
                Some(_) => {}
                None => {
                    crate::server::log(&format!(
                        "hub: 0x{opcode:04X} for character {character}, who is not in the directory; offered to every other channel"
                    ));
                    s.broadcast(Some(from), &bytes);
                }
            }
        }
        Frame::PartyRequest { actor, now, request } => {
            // Applied here only so a late channel's snapshot is right; the outcome is
            // computed by every channel from the same echo.
            let outcome = s.parties.apply(now, actor, request.clone());
            crate::server::log(&format!("hub: party request by {actor}: {request:?} -> {}", match &outcome {
                Ok(effects) => format!("{} effect(s)", effects.len()),
                Err(r) => format!("refused: {r:?}"),
            }));
            s.broadcast(None, &bytes);
        }
        Frame::PartySnapshot { .. } => {}
        Frame::MessengerRequest { actor, request } => {
            // The same contract as a party request: applied here for the snapshot, echoed to
            // EVERY channel (the sender included) so every replica applies the same sequence.
            let outcome = s.messengers.apply(actor, request.clone());
            crate::server::log(&format!("hub: maple chat request by {actor}: {request:?} -> {outcome:?}"));
            s.broadcast(None, &bytes);
        }
        Frame::MessengerSnapshot { .. } => {}
    }
}

fn hub_drop(state: &Mutex<HubState>, id: u64) {
    let mut s = state.lock().unwrap_or_else(|e| e.into_inner());
    let channel = s.conns.remove(&id).and_then(|c| c.channel);
    let gone: Vec<(u32, u32)> = s
        .directory
        .iter()
        .filter(|(_, (_, conn))| *conn == id)
        .map(|(c, (e, _))| (*c, e.channel))
        .collect();
    for (character, ch) in &gone {
        s.directory.remove(character);
        let f = Frame::Offline { character: *character, channel: *ch }.encode();
        s.broadcast(None, &f);
        // A channel that died takes its players out of their Maple Chat rooms too, or everyone
        // else keeps drawing a seat nobody is in. Echoed like any request, so every channel
        // tells the members it hosts.
        if s.messengers.room_of(*character).is_some() {
            let _ = s.messengers.apply(*character, crate::messenger::Request::Disconnect);
            let f = Frame::MessengerRequest { actor: *character, request: crate::messenger::Request::Disconnect }.encode();
            s.broadcast(None, &f);
        }
    }
    crate::server::log(&format!(
        "hub: connection #{id} (channel {channel:?}) closed; {} character(s) taken offline",
        gone.len()
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::party::{DeclineReason, Request};

    /// Every frame round-trips, including every party request variant - the link's only
    /// job is to carry these unchanged, and an asymmetry here would desync the replicas.
    #[test]
    fn every_frame_round_trips() {
        let frames = vec![
            Frame::Hello { channel: 1 },
            Frame::Online { character: 213, name: "Cobalt".into(), account: 1, channel: 0, map: 0 },
            Frame::Offline { character: 213, channel: 0 },
            Frame::Deliver { character: 214, opcode: 0x01B1, body: vec![1, 2, 3, 0xd6], what: "a line".into() },
            Frame::PartyRequest { actor: 213, now: 1_789_000_000, request: Request::Create { name: "Wisp's Party".into() } },
            Frame::PartyRequest { actor: 213, now: 1, request: Request::Invite { target: 214 } },
            Frame::PartyRequest { actor: 214, now: 2, request: Request::Accept { party: 1 } },
            Frame::PartyRequest { actor: 214, now: 3, request: Request::Decline { party: 1, reason: DeclineReason::Busy } },
            Frame::PartyRequest { actor: 214, now: 4, request: Request::Leave },
            Frame::PartyRequest { actor: 213, now: 5, request: Request::Expel { target: 214 } },
            Frame::PartyRequest { actor: 213, now: 6, request: Request::ChangeLeader { target: 214 } },
            Frame::PartyRequest { actor: 213, now: 7, request: Request::SetPickupRights { rights: 1 } },
            Frame::PartyRequest { actor: 213, now: 8, request: Request::Disconnect { successor: Some(214), last_online: false } },
            Frame::PartyRequest { actor: 213, now: 9, request: Request::Disconnect { successor: None, last_online: true } },
            Frame::MessengerRequest {
                actor: 213,
                request: crate::messenger::Request::Enter {
                    room: 0x1_0001,
                    seat: net::messenger::Seat { character_id: 219, name: "Moth".into(), look: vec![9; 40] },
                },
            },
            Frame::MessengerRequest { actor: 213, request: crate::messenger::Request::Leave { room: 0x1_0001 } },
            Frame::MessengerRequest { actor: 213, request: crate::messenger::Request::Disconnect },
            Frame::MessengerSnapshot {
                rooms: vec![crate::messenger::Room {
                    id: 0x1_0001,
                    seats: [Some(net::messenger::Seat { character_id: 213, name: "Cobalt".into(), look: vec![1] }), None, None, None, None, None],
                }],
                next_id: 0x1_0002,
            },
            Frame::PartySnapshot {
                parties: vec![crate::party::Party { id: 1, name: "P".into(), leader: 213, members: vec![213, 214], pickup_rights: 1 }],
                next_id: 2,
            },
        ];
        for f in frames {
            let bytes = f.encode();
            let n = u32::from_le_bytes(bytes[..4].try_into().unwrap()) as usize;
            assert_eq!(n, bytes.len() - 4, "the length prefix covers the payload");
            let mut cursor = std::io::Cursor::new(bytes.clone());
            let payload = read_frame(&mut cursor).unwrap().unwrap();
            assert_eq!(Frame::decode(&payload), Some(f.clone()), "{f:?}");
        }
        assert_eq!(Frame::decode(&[99]), None, "an unknown kind is None, not a panic");
        assert_eq!(Frame::decode(&[]), None);
        let mut short = std::io::Cursor::new(vec![0xff, 0xff, 0xff, 0x7f]);
        assert!(read_frame(&mut short).is_err(), "an absurd length is refused before it allocates");
    }

    /// The hub end to end on a loopback port: two "channels" connect, one announces a
    /// character, the other sees it; a party request from one is echoed to both in order and
    /// a late third connection gets the snapshot; a delivery for the announced character goes
    /// to its host only; a dropped connection takes its characters offline everywhere.
    #[test]
    fn the_hub_relays_directory_party_order_and_deliveries() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        std::thread::spawn(move || run_hub(listener));

        let dial = |channel: u32| {
            let mut s = TcpStream::connect(addr).unwrap();
            s.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
            s.write_all(&Frame::Hello { channel }.encode()).unwrap();
            s
        };
        let next = |s: &mut TcpStream| Frame::decode(&read_frame(s).unwrap().unwrap()).unwrap();

        let mut a = dial(0);
        assert_eq!(next(&mut a), Frame::PartySnapshot { parties: vec![], next_id: crate::party::FIRST_PARTY_ID }, "an empty world on Hello");
        assert_eq!(next(&mut a), Frame::MessengerSnapshot { rooms: vec![], next_id: crate::messenger::FIRST_ROOM_ID }, "and no Maple Chat rooms");
        let mut b = dial(1);
        assert!(matches!(next(&mut b), Frame::PartySnapshot { .. }));
        assert!(matches!(next(&mut b), Frame::MessengerSnapshot { .. }));

        a.write_all(&Frame::Online { character: 213, name: "Cobalt".into(), account: 1, channel: 0, map: 0 }.encode()).unwrap();
        assert_eq!(next(&mut b), Frame::Online { character: 213, name: "Cobalt".into(), account: 1, channel: 0, map: 0 }, "b hears a's character");

        // A party request from b is echoed to a AND b, and a late c gets the state.
        let create = Frame::PartyRequest { actor: 213, now: 10, request: Request::Create { name: "P".into() } };
        b.write_all(&create.encode()).unwrap();
        assert_eq!(next(&mut a), create);
        assert_eq!(next(&mut b), create);
        let mut c = dial(2);
        assert_eq!(next(&mut c), Frame::Online { character: 213, name: "Cobalt".into(), account: 1, channel: 0, map: 0 }, "the directory first");
        match next(&mut c) {
            Frame::PartySnapshot { parties, next_id } => {
                assert_eq!(parties.len(), 1);
                assert_eq!(parties[0].leader, 213);
                assert_eq!(next_id, crate::party::FIRST_PARTY_ID + 1);
            }
            other => panic!("{other:?}"),
        }
        assert!(matches!(next(&mut c), Frame::MessengerSnapshot { ref rooms, .. } if rooms.is_empty()));

        // A delivery for 213 reaches a (its host) and not c.
        let line = Frame::Deliver { character: 213, opcode: 0x01B1, body: vec![7], what: "hi".into() };
        c.write_all(&line.encode()).unwrap();
        assert_eq!(next(&mut a), line);
        c.set_read_timeout(Some(std::time::Duration::from_millis(300))).unwrap();
        assert!(read_frame(&mut c).is_err(), "nothing for c: the read times out");

        // **Maple Chat across channels** (2026-09-24): a room opened for 213 is echoed to every
        // channel, the sender included, so channel 1 can seat an Accept for a room channel 0 made.
        let seat = net::messenger::Seat { character_id: 213, name: "Cobalt".into(), look: vec![1, 2, 3] };
        let open = Frame::MessengerRequest { actor: 213, request: crate::messenger::Request::Open { seat } };
        a.write_all(&open.encode()).unwrap();
        assert_eq!(next(&mut a), open);
        assert_eq!(next(&mut b), open);
        assert_eq!(next(&mut c), open);

        // a drops: 213 goes offline for everyone still connected - and leaves the room, or
        // everyone else would keep drawing a seat nobody is in.
        drop(a);
        b.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
        assert_eq!(next(&mut b), Frame::Offline { character: 213, channel: 0 });
        assert_eq!(next(&mut b), Frame::MessengerRequest { actor: 213, request: crate::messenger::Request::Disconnect });
    }
}
