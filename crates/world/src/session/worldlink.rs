//! The session's end of the world link (`crate::link`): who is online where, packets to a
//! character on another channel, and party requests serialised by the hub.
//!
//! # Why a party request is answered a tick later when the hub is up
//!
//! With the hub connected, `run_party_request` does not apply the request itself: it sends
//! it to the hub and returns nothing. The hub echoes it to every channel in one order; each
//! channel's link thread applies it to its replica (`Fields::parties`), and the channel that
//! hosts the ACTOR queues the outcome here ([`pending_party_outcomes`]). The actor's own
//! session drains that queue on its next `handle` or `tick` - at most `TICK_MS` later, in
//! practice about a millisecond over loopback - and runs the very same `party_effects` it
//! would have run synchronously, with itself as the actor. Every packet to every member then
//! leaves through [`Session::deliver_anywhere`], which is the local bus when the member is
//! here and a [`crate::link::Frame::Deliver`] when they are not.
//!
//! The synchronous path is untouched for a process with no hub (the test suite, a single
//! channel started by hand): same code, same tests, one `if`.

use super::*;

/// Outcomes of hub-echoed party requests, waiting for the actor's session to turn them into
/// packets: `actor -> (described request, outcome)`.
static PENDING: std::sync::Mutex<Vec<(u32, String, Result<Vec<crate::party::Effect>, crate::party::Refusal>)>> =
    std::sync::Mutex::new(Vec::new());

/// **The link thread's side.** Apply one echoed request to this process's replica and, if
/// the actor plays here, queue the outcome for their session. Every channel runs this for
/// every request, in hub order, so the replicas agree.
pub fn apply_echoed_party_request(fields: &crate::fields::Fields, actor: u32, now: i64, request: crate::party::Request) {
    let described = format!("{request:?}");
    let outcome = fields.parties().apply(now, actor, request);
    // Hosted here = announced to the hub from here and not yet taken back (`Link::hosts`).
    let hosted_here = crate::link::installed().is_some_and(|l| l.hosts(actor));
    if hosted_here {
        PENDING.lock().unwrap_or_else(|e| e.into_inner()).push((actor, described, outcome));
    } else {
        crate::server::log(&format!(
            "   party: hub echo applied for character {actor} (not on this channel): {described} -> {}",
            match &outcome {
                Ok(e) => format!("{} effect(s), packets built by their own channel", e.len()),
                Err(r) => format!("refused, {r:?}"),
            }
        ));
    }
}

impl Session {
    /// Tell the hub this character plays here. Idempotent; called on every field entry.
    pub(super) fn announce_online_to_link(&self) {
        let (Some(link), Some(chr), Some(claim)) = (crate::link::installed(), self.claimed_character(), self.claimed()) else {
            return;
        };
        link.announce_online(chr.id, &chr.name, u32::try_from(claim.account_id).unwrap_or(0), chr.map_id);
    }

    /// Tell the hub this character left this channel.
    pub(super) fn announce_offline_to_link(&self) {
        let (Some(link), Some(chr)) = (crate::link::installed(), self.claimed_character()) else { return };
        link.announce_offline(chr.id);
    }

    /// **A packet to a character wherever they are**: this channel's bus first, then the hub.
    /// `false` when neither took it - the character is not online anywhere this process can
    /// reach, and the caller says so in its own words.
    pub(super) fn deliver_anywhere(&self, to: u32, reply: Reply) -> bool {
        if self.bus().publish_to_character_anywhere(to, reply.clone()) {
            return true;
        }
        match crate::link::current() {
            Some(link) => link.deliver(to, &reply),
            None => false,
        }
    }

    /// Drain the outcomes the link thread queued for THIS character and turn them into
    /// packets, exactly as the synchronous path would have. Called from `handle` and `tick`.
    pub(super) fn collect_party_outcomes(&mut self) -> Vec<Reply> {
        let Some(me) = self.claimed_character().map(|c| c.id) else { return Vec::new() };
        let mine: Vec<(String, Result<Vec<crate::party::Effect>, crate::party::Refusal>)> = {
            let mut q = PENDING.lock().unwrap_or_else(|e| e.into_inner());
            if q.is_empty() {
                return Vec::new();
            }
            let (m, rest): (Vec<_>, Vec<_>) = std::mem::take(&mut *q).into_iter().partition(|(a, _, _)| *a == me);
            *q = rest;
            m.into_iter().map(|(_, d, o)| (d, o)).collect()
        };
        let mut out = Vec::new();
        for (described, outcome) in mine {
            out.extend(self.party_outcome_replies(me, &described, outcome));
        }
        out
    }

    /// Send a party request through the hub. `true` when it went; the reply comes back
    /// through [`Session::collect_party_outcomes`].
    pub(super) fn send_party_request_to_hub(&self, actor: u32, now: i64, request: &crate::party::Request) -> bool {
        match crate::link::current() {
            Some(link) => link.send(&crate::link::Frame::PartyRequest { actor, now, request: request.clone() }),
            None => false,
        }
    }
}

/// The handler `crate::server` installs on the link: what this process does with each frame.
pub fn link_handler(fields: std::sync::Arc<crate::fields::Fields>) -> crate::link::Handler {
    std::sync::Arc::new(move |frame: crate::link::Frame| {
        use crate::link::Frame;
        match frame {
            Frame::Deliver { character, opcode, body, what } => {
                let reply = Reply { opcode, body, what: format!("{what} [via the hub]") };
                if !fields.bus().publish_to_character_anywhere(character, reply) {
                    crate::server::log(&format!(
                        "   link: a 0x{opcode:04X} for character {character} arrived but they are not on this channel; dropped"
                    ));
                }
            }
            Frame::PartyRequest { actor, now, request } => {
                apply_echoed_party_request(&fields, actor, now, request);
            }
            Frame::PartySnapshot { parties, next_id } => {
                crate::server::log(&format!("   link: party snapshot from the hub: {} party(ies), next id {next_id}", parties.len()));
                fields.parties().restore(parties, next_id);
            }
            // Online / Offline are folded into the link's directory before this runs.
            Frame::Online { .. } | Frame::Offline { .. } | Frame::Hello { .. } => {}
        }
    })
}
