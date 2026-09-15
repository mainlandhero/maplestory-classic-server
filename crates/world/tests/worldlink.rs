//! The world link, end to end in one process: a real `maplecw-chat` hub on an ephemeral
//! port, a real `Link` installed the way `crate::server` installs it, and a real session
//! creating a party through it.
//!
//! This is its own test binary because `world::link::install` is process-wide: with a link
//! installed, every party request in the process goes through the hub, and the unit tests
//! in the library exercise the local path on purpose.
//!
//! Nothing here authenticates - the link is server-to-server and the channel socket still
//! carries no credentials.

use std::sync::Arc;

use net::opcode::Character;
use store::Store;
use world::config::Config;
use world::fields::Fields;
use world::session::Session;

/// The Create body the owner's client sent on 2026-09-14 23:38:49 (`world-ch0.log` line 27650):
/// action 0, tag 5, the name "the owner's Party".
const CREATE_HEX: &str = "1000000000000a000e000000070008000a000000000000050c000000000006000800040006000000040000000c00000057697370277320506172747900000000";

fn hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

#[test]
fn a_party_created_through_the_hub_is_answered_from_the_echo_and_the_hub_knows_it() {
    // The hub, as maplecw-chat runs it, on a free port.
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let hub = listener.local_addr().unwrap();
    std::thread::spawn(move || world::link::run_hub(listener));

    // A channel process's worth of state, and the link `crate::server` would install.
    let store = Arc::new(Store::open_in_memory().unwrap());
    let config = Arc::new(Config::default());
    let fields = Arc::new(Fields::new());
    let link = world::link::Link::connect(hub, 0, world::session::worldlink::link_handler(fields.clone()));
    world::link::install(link.clone());
    for _ in 0..50 {
        if link.is_connected() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    assert!(link.is_connected(), "the link dials the hub");

    // A character, in the field, announced to the hub by its field entry.
    let account = store.create_account("maplecw", "correct horse battery").unwrap();
    let chr = Character { name: "Cobalt".to_string(), map_id: 104_040_000, ..Default::default() };
    let id = store.create_character(account, 0, &chr).unwrap().id;
    store.create_migration(account, id, 0, 0).unwrap();
    let mut s = Session::joining(store.clone(), config.clone(), fields.clone());
    assert!(s.claim_for_character(id).contains("claimed the migration"));
    let _ = s.handle(&world::session::CLIENT_FIELD_ENTERED.to_le_bytes());
    for _ in 0..50 {
        if link.directory.lock().unwrap().by_id.contains_key(&id) || link.hosts(id) {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    assert!(link.hosts(id), "field entry announced the character to the hub");

    // Create: nothing comes back synchronously - the request went to the hub.
    let mut body = net::party::CLIENT_PARTY_REQUEST.to_le_bytes().to_vec();
    body.extend_from_slice(&hex(CREATE_HEX));
    let now = s.handle(&body);
    assert!(
        now.iter().all(|r| r.opcode != net::party::PARTY_RESULT),
        "with the hub up, the answer waits for the echo: {:?}",
        now.iter().map(|r| &r.what).collect::<Vec<_>>()
    );

    // The echo arrives on the link thread, is applied to the replica, and the actor's next
    // tick turns it into the client's 0x0E.
    let mut created = None;
    for i in 0..100 {
        let out = s.tick(1_000 + i * 10);
        if let Some(r) = out.into_iter().find(|r| r.opcode == net::party::PARTY_RESULT) {
            created = Some(r);
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let created = created.expect("the hub's echo became the CREATED reply");
    assert_eq!(created.body[0], net::party::result::CREATED, "{}", created.what);
    let party = u32::from_le_bytes(created.body[1..5].try_into().unwrap());
    assert_eq!(party, world::party::FIRST_PARTY_ID);
    assert_eq!(
        fields.parties().party_of(id).map(|p| p.name.clone()),
        Some("the owner's Party".to_string()),
        "the replica applied the echo"
    );

    // A second, late channel gets the party in its snapshot - the hub applied it too.
    let mut late = std::net::TcpStream::connect(hub).unwrap();
    late.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
    std::io::Write::write_all(&mut late, &world::link::Frame::Hello { channel: 1 }.encode()).unwrap();
    let mut saw_party = false;
    for _ in 0..4 {
        let mut len = [0u8; 4];
        if std::io::Read::read_exact(&mut late, &mut len).is_err() {
            break;
        }
        let mut payload = vec![0u8; u32::from_le_bytes(len) as usize];
        std::io::Read::read_exact(&mut late, &mut payload).unwrap();
        match world::link::Frame::decode(&payload) {
            Some(world::link::Frame::PartySnapshot { parties, .. }) => {
                saw_party = parties.iter().any(|p| p.id == party && p.leader == id && p.name == "the owner's Party");
                break;
            }
            Some(world::link::Frame::Online { character, .. }) => assert_eq!(character, id, "the directory names Cobalt"),
            other => panic!("{other:?}"),
        }
    }
    assert!(saw_party, "the late channel's snapshot carries the party");
}
