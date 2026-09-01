//! **Party requests, answered.**
//!
//! The owner, 2026-09-01: *"There are already party UI elements in the game."* There are - the
//! client ships the whole window, `UI_000.wz/UserList.img/Party` with all eight buttons and a
//! scrollable member list, `PartyHP.img` with the gauge, and 89 party strings. **[L]**
//! `research/party.md`.
//!
//! # Why this file exists before parties do
//!
//! Because `0x0182` was reaching this server and **going unanswered**. One archived capture
//! has it, from the owner pressing Create:
//!
//! ```text
//!   16:48:31.299 <- 0x0182 UNKNOWN, 68 byte body 1000000000000a000e00...
//!   16:48:31.299    0x0182 UNKNOWN is not answered yet
//! ```
//!
//! `CLAUDE.md`'s first expensive rule: *an unanswered packet freezes the client's entire UI -
//! every button, including the quit prompt - and reads on screen as a crash.* A refusal is an
//! answer; silence is not. So this file answers every party request, today, whether or not
//! there is a party system behind it.
//!
//! # What it answers with, and why that exact code
//!
//! `0x00A5` `PARTY_RESULT` carries a `u8 code` into a 45-entry jump table at `0x1413bcc50`.
//! **Sixteen of the twenty-nine non-default arms read nothing after the code**, and neither
//! does the default. **[L]** `net::party::refusal` will only build a code whose arm is one of
//! those, so this cannot produce a packet the client would read fields out of and run off the
//! end of.
//!
//! [`net::party::request_failed`] is the default arm - *"Due to an unknown error, your party
//! request failed."* Two bytes past the opcode, and the handler returns normally.
//!
//! # What is NOT here yet
//!
//! Membership. `crate::party::Parties` is written and tested and needs a home on the channel
//! that outlives a session; that holder is being added to `Fields` in a separate pass. Until
//! it lands, every request is refused **specifically** rather than ignored, and the request is
//! logged with what it actually asked for - so the first two-client run produces a record of
//! which buttons the window sends even though none of them works yet.

use crate::Reply;

/// What an action byte is called, for the log line only.
///
/// The seven names are the client's own: `FUN_1411c9540`, the party window's button
/// dispatcher, compares the UTF-16 literals `create`, `invite`, `expel`, `leave`, `pickup`
/// and `leader`. **[L]** `research/party.md`.
///
/// An unknown byte is printed as unknown rather than mapped to the nearest thing - six of the
/// seven actions have never been seen on this wire, so the first run to produce one must not
/// have it silently relabelled.
fn action_name(action: u8) -> &'static str {
    use net::party::action;
    match action {
        action::CREATE => "create",
        action::LEAVE => "leave",
        action::SET_PICKUP_RIGHTS => "pickup rights",
        action::INVITE => "invite",
        action::JOIN_REQUEST => "join request",
        action::EXPEL => "expel",
        action::CHANGE_LEADER => "change leader",
        _ => "UNKNOWN - not one of the seven the party window sends",
    }
}

impl super::Session {
    /// Answer `0x0182` `CLIENT_PARTY_REQUEST` and `0x0183` `CLIENT_PARTY_INVITE_ANSWER`.
    ///
    /// **Always answers.** Every path here returns exactly one `0x00A5`, including the one
    /// where the body does not parse - a body this server cannot read is still a request the
    /// client is blocking on, and `net::party::parse_request` returning `None` says nothing
    /// about whether the window is waiting.
    pub(super) fn on_party_request(&mut self, opcode: u16, body: &[u8]) -> Vec<Reply> {
        let who = self
            .claimed_character()
            .map(|c| format!("character {} ({})", c.id, c.name))
            .unwrap_or_else(|| "an unclaimed connection".to_string());

        // The decode is for the LOG, not for the answer: the answer is the same either way
        // until there is a party system. It is worth logging because the archive contains
        // exactly one party packet, and the first two-client run is where the other six
        // actions get seen for the first time.
        let asked = if opcode == net::party::CLIENT_PARTY_REQUEST {
            match net::party::parse_request(body) {
                Some(req) => format!(
                    "action {} ({}), payload tag {}, name {:?}",
                    req.action,
                    action_name(req.action),
                    req.payload_tag,
                    req.name
                ),
                // Not a failure to hide. `parse_request` reads one payload shape - the create
                // that the single archived capture contains - so `None` here is most likely a
                // shape nobody has decoded yet, and the bytes are what the next pass needs.
                None => format!("a {}-byte body this decoder cannot read: {body:02x?}", body.len()),
            }
        } else {
            format!("invite answer, {}-byte body {body:02x?}", body.len())
        };

        vec![Reply {
            opcode: net::party::PARTY_RESULT,
            body: net::party::request_failed(),
            what: format!(
                "PartyResult UNKNOWN_ERROR to {who}: {asked}. There is no party system on this \
                 server yet - crate::party is written and has no home on the channel - so this \
                 is a SPECIFIC refusal rather than silence. An unanswered request freezes the \
                 client's whole UI",
            ),
        }]
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use store::Store;

    use crate::config::Config;
    use crate::fields::Fields;
    use crate::session::Session;

    /// A claimed session, built the way `session::multiplayer`'s tests build theirs.
    fn session() -> Session {
        let store = Arc::new(Store::open_in_memory().unwrap());
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Leader".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.create_migration(account, id, 0, 0).unwrap();
        let config = Arc::new(Config { set_field_probe: true, ..Config::default() });
        let mut s = Session::joining(store, config, Arc::new(Fields::new()));
        s.claim_for_character(id);
        s
    }

    fn hex_body(h: &str) -> Vec<u8> {
        let h: String = h.chars().filter(|c| !c.is_whitespace()).collect();
        (0..h.len() / 2).map(|i| u8::from_str_radix(&h[i * 2..i * 2 + 2], 16).unwrap()).collect()
    }

    /// **Every party request is answered, including one this server cannot decode.**
    ///
    /// The second half is the point. A handler that answers only what it understands leaves
    /// the client blocked on everything else, and six of the seven party actions have never
    /// been seen on this wire.
    #[test]
    fn a_party_request_is_always_answered() {
        let mut s = session();

        // The real captured create, from `previous-runs` - action absent (0 = create),
        // payload tag 5, the string "TestCharD's Party".
        let captured = hex_body(
            "1000000000000a000e000000070008000a000000000000050c00000000000600080004000600\
             000004000000110000005465737443686172442773205061727479000000",
        );
        let out = s.on_party_request(net::party::CLIENT_PARTY_REQUEST, &captured);
        assert_eq!(out.len(), 1, "exactly one answer");
        assert_eq!(out[0].opcode, net::party::PARTY_RESULT);
        assert!(out[0].what.contains("create"), "and it logs what was asked: {}", out[0].what);

        // A body this decoder cannot read is still answered.
        let out = s.on_party_request(net::party::CLIENT_PARTY_REQUEST, &[0xff, 0x00]);
        assert_eq!(out.len(), 1, "an undecodable body is still a blocked client");
        assert_eq!(out[0].opcode, net::party::PARTY_RESULT);

        // An empty body too - the shortest thing the client could possibly send.
        assert_eq!(s.on_party_request(net::party::CLIENT_PARTY_REQUEST, &[]).len(), 1);

        // And the invite answer, which is a different opcode with a different body.
        let out = s.on_party_request(net::party::CLIENT_PARTY_INVITE_ANSWER, &[0x1b, 1, 0, 0]);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].opcode, net::party::PARTY_RESULT);
    }

    /// **The dispatcher actually routes it**, which is the half a handler test cannot show.
    ///
    /// `on_party_request` being correct is worth nothing if `0x0182` still falls through to
    /// the unknown-opcode path, which is exactly where it was before this file existed. So
    /// this goes in through `Session::handle` with the opcode on the front, the way the
    /// socket loop does.
    #[test]
    fn the_dispatcher_routes_both_party_opcodes() {
        let mut s = session();
        for opcode in [net::party::CLIENT_PARTY_REQUEST, net::party::CLIENT_PARTY_INVITE_ANSWER] {
            let mut packet = opcode.to_le_bytes().to_vec();
            packet.extend_from_slice(&[0x10, 0x00, 0x00, 0x00]);
            let out = s.handle(&packet);
            assert_eq!(
                out.iter().filter(|r| r.opcode == net::party::PARTY_RESULT).count(),
                1,
                "{opcode:#06x} must be answered by the dispatcher, not dropped: {out:?}"
            );
        }
        // The control: an opcode this server really does not handle stays unanswered, so the
        // assertion above is about routing and not about `handle` answering everything.
        assert!(
            s.handle(&[0xFE, 0xFF, 0x00]).iter().all(|r| r.opcode != net::party::PARTY_RESULT),
            "0xFFFE is not a party packet"
        );
    }

    /// The refusal is a code whose client-side arm **reads no fields**, so it cannot run off
    /// the end of a two-byte body.
    #[test]
    fn the_refusal_is_a_read_free_code() {
        let body = net::party::request_failed();
        assert_eq!(body.len(), 3, "the opcode's two bytes plus one code byte");
        let code = body[2];
        assert!(
            net::party::is_silent_code(code),
            "code {code} must be one whose handler arm reads nothing"
        );
    }
}
