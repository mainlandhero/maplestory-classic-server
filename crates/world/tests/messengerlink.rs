//! **Maple Chat across two channels, through a real hub** - the deployed server's failure,
//! replayed.
//!
//! `Server Investigation/world-ch1.log` 04:32:58 and `world-ch0.log` 04:33:04 (2026-09-24):
//! Cobalt opened a room on **channel 1**, invited Moth on **channel 0**, and their Accept was
//! answered "result 1 - not here; nothing opens", because the room lived in channel 1's
//! process. The owner: *"Maple Chat should work cross channel, please use the hub code."*
//!
//! Here channel 1 is a raw link client speaking the hub protocol - exactly what another
//! channel process looks like to the hub - and channel 0 is a real `Link`, a real
//! `Fields` replica and a real session. Its own test binary because `world::link::install` is
//! process-wide, as `tests/worldlink.rs` explains.
//!
//! Nothing here authenticates: the link is server-to-server and the channel socket carries no
//! credentials.

use std::io::{Read, Write};
use std::sync::Arc;

use net::opcode::Character;
use store::Store;
use world::config::Config;
use world::fields::Fields;
use world::link::Frame;
use world::session::Session;

fn read_frame(s: &mut std::net::TcpStream) -> Frame {
    let mut len = [0u8; 4];
    s.read_exact(&mut len).unwrap();
    let mut body = vec![0u8; u32::from_le_bytes(len) as usize];
    s.read_exact(&mut body).unwrap();
    Frame::decode(&body).expect("a frame the link understands")
}

fn wait(mut done: impl FnMut() -> bool, what: &str) {
    for _ in 0..100 {
        if done() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
    panic!("timed out waiting for {what}");
}

#[test]
fn an_accept_on_channel_0_joins_a_room_channel_1_opened() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let hub = listener.local_addr().unwrap();
    std::thread::spawn(move || world::link::run_hub(listener));

    // Channel 0: the real thing.
    let store = Arc::new(Store::open_in_memory().unwrap());
    let config = Arc::new(Config::default());
    let fields = Arc::new(Fields::new());
    let link = world::link::Link::connect(hub, 0, world::session::worldlink::link_handler(fields.clone()));
    world::link::install(link.clone());
    wait(|| link.is_connected(), "channel 0 to dial the hub");

    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = Character { name: "Moth".to_string(), map_id: 104_040_000, ..Default::default() };
    let moth = store.create_character(account, 0, &chr).unwrap().id;
    store.create_migration(account, moth, 0, 0).unwrap();
    let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
    assert!(s.claim_for_character(moth).contains("claimed the migration"));
    let _ = s.handle(&world::session::CLIENT_FIELD_ENTERED.to_le_bytes());
    wait(|| link.hosts(moth), "Moth to be announced from channel 0");

    // Channel 1: another process, as the hub sees it. Cobalt plays there and opens a room.
    let mut ch1 = std::net::TcpStream::connect(hub).unwrap();
    ch1.write_all(&Frame::Hello { channel: 1 }.encode()).unwrap();
    const JOSIAH: u32 = 213;
    ch1.write_all(&Frame::Online { character: JOSIAH, name: "Cobalt".into(), account: 9, channel: 1, map: 104_040_000 }.encode())
        .unwrap();
    let seat = net::messenger::Seat { character_id: JOSIAH, name: "Cobalt".into(), look: vec![7; 12] };
    ch1.write_all(&Frame::MessengerRequest { actor: JOSIAH, request: world::messenger::Request::Open { seat } }.encode())
        .unwrap();

    // **Channel 0's replica learns the room it did not make** - the whole point.
    let room = world::messenger::FIRST_ROOM_ID;
    wait(|| fields.messengers().get(room).is_some(), "channel 0's replica to hold channel 1's room");

    // Moth presses Accept on channel 0: mode 7 with the room's id, the bytes their client sent
    // on the deployed server (`0700000001000200`, a room id of that shape).
    let mut accept = net::messenger::CLIENT_MESSENGER.to_le_bytes().to_vec();
    accept.extend_from_slice(&7u32.to_le_bytes());
    accept.extend_from_slice(&room.to_le_bytes());
    let immediate = s.handle(&accept);
    assert!(
        !immediate.iter().any(|r| r.opcode == net::messenger::MESSENGER && r.body.get(8) == Some(&1)),
        "no 'result 1' on the spot: {:?}",
        immediate.iter().map(|r| &r.what).collect::<Vec<_>>()
    );

    // Their window opens from the echo: mode 0 result 0, then the six-seat table with Cobalt in
    // seat 0 and them in seat 1.
    let mut got = immediate;
    let mut now = 1_000;
    wait(
        || {
            now += 100;
            got.extend(s.tick(now));
            got.iter().filter(|r| r.opcode == net::messenger::MESSENGER).count() >= 2
        },
        "Moth's window to open from the hub's echo",
    );
    let messenger: Vec<_> = got.iter().filter(|r| r.opcode == net::messenger::MESSENGER).collect();
    assert_eq!(messenger[0].body, net::messenger::self_enter(room, 0), "{}", messenger[0].what);
    let table = &messenger[1].body;
    assert_eq!(&table[0..4], &room.to_le_bytes(), "the room channel 1 made");
    assert_eq!(&table[4..8], &4i32.to_le_bytes(), "mode 4, all six seats");
    let has = |needle: &[u8]| table.windows(needle.len()).any(|w| w == needle);
    assert!(has(b"Cobalt") && has(&[7; 12]), "seat 0 is Cobalt, with the look channel 1 sent: {}", messenger[1].what);
    assert!(has(b"Moth"), "and Moth has a seat: {}", messenger[1].what);
    assert_eq!(fields.messengers().get(room).unwrap().members(), vec![JOSIAH, moth], "Cobalt in seat 0, Moth in seat 1");

    // Channel 1 hears the Enter too, so it can redraw Cobalt's window.
    loop {
        match read_frame(&mut ch1) {
            Frame::MessengerRequest { actor, request: world::messenger::Request::Enter { room: r, .. } } => {
                assert_eq!((actor, r), (moth, room));
                break;
            }
            _ => continue,
        }
    }
}
