//! Saving and restoring the player's key bindings.
//!
//! `net::keymap` owns the wire format and `store::keymap` owns the rows; this is the join.
//! `research/keyboard-layout-2026-09-08.md` has the full working.
//!
//! # `0x0199` is answered with nothing, and that is measured rather than assumed
//!
//! `CLAUDE.md`'s oldest rule is *always answer* - an unanswered packet freezes the client's
//! whole UI. `0x0199` is one of the ones that does not latch, and the evidence is the capture
//! this feature came from: it arrived at 01:52:44 in the archived channel log and **the client
//! went on playing for hours afterwards**, through several more field entries, with nothing
//! sent back.
//!
//! It is also absent from `net::dropmoney::LATCHING_REQUESTS`, and that list is worth checking
//! rather than trusting, because a search that finds nothing is usually a broken search. It
//! contains `0x0143` and `0x01FD` - two of the three opcodes `CLAUDE.md` records as having
//! frozen a client - so it can speak. And it contains **`0x0197` and `0x019D`**: the immediate
//! neighbours on both sides latch, and `0x0199` does not. That is a much sharper negative than
//! a bare absence.
//!
//! So this returns no reply. If that is ever wrong, the symptom is a UI that freezes the
//! instant CONFIRM is clicked, and the fix is the nine-byte unlock the catch-all arm sends.
//!
//! # Four tables, and the controller is one of them - 2026-09-14
//!
//! The owner: *"Whenever there are customization to keybindings in the controller settings, it is
//! not getting saved properly, and when clients switch maps, their controller settings are
//! completely screwed up."* A delta names its table (`net::keymap` §4); this module stored
//! all of them as the keyboard's and sent the controller's back as *keep*, which the client
//! reads as "keep the reset to keyboard preset 0". Every SetField therefore handed the
//! controller a keyboard layout and put the controller's buttons on scan codes. Now the row
//! carries the table and the `0x05F1` sends all four tables READ.

use net::keymap::{Binding, Change, Slot};
use store::keymap::{KeyBinding, KeymapOption};

use super::{Reply, Session};

impl Session {
    /// The player clicked CONFIRM in KEY BINDINGS, or changed a preset.
    pub(super) fn on_keymap_change(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            // No character to attribute the layout to. Dropping it is right - there is
            // nowhere to put it - but say so, because a silent drop here would look exactly
            // like the store failing.
            crate::server::log("   keymap: 0x0199 arrived with no claimed character; not stored");
            return Vec::new();
        };
        let Some(change) = net::keymap::parse_change(body) else {
            crate::server::log(&format!(
                "keymap: 0x0199 did not decode ({} byte body) - refused whole rather than \
                 partially applied, so the player's stored layout is unchanged",
                body.len()
            ));
            return Vec::new();
        };

        match change {
            Change::Bindings { table, bindings } => {
                let rows: Vec<KeyBinding> = bindings
                    .iter()
                    .map(|b| KeyBinding { preset: table, key: b.key, kind: b.slot.kind, action: b.slot.action })
                    .collect();
                let which = if table == net::keymap::CONTROLLER_TABLE {
                    "the CONTROLLER table".to_string()
                } else {
                    format!("keyboard preset {table}")
                };
                match self.store.apply_keymap_delta(chr.id, &rows) {
                    Ok(n) => crate::server::log(&format!(
                        "keymap: {} binding(s) for {which} from character {} ({}) merged, {} row(s) \
                         changed. This is a DELTA against the client's shadow of that one table, \
                         not a full layout.",
                        rows.len(),
                        chr.id,
                        chr.name,
                        n
                    )),
                    Err(e) => crate::server::log(&format!("keymap: storing {} binding(s) failed: {e}", rows.len())),
                }
            }
            Change::OptA(v) => self.store_keymap_option(chr.id, KeymapOption::A, v),
            Change::OptB(v) => self.store_keymap_option(chr.id, KeymapOption::B, v),
            Change::Preset(p) => {
                // No inbound opcode was found for the preset selector, so storing it would be
                // storing something we can never send back. Logged rather than written, so
                // the day the opcode is found there is a record of how often it is used.
                crate::server::log(&format!(
                    "keymap: character {} selected preset {p}. NOT stored - no server-to-client \
                     opcode for the preset selector has been found, so it could not be restored. \
                     research/keyboard-layout-2026-09-08.md section 6.",
                    chr.id
                ));
            }
        }
        Vec::new()
    }

    fn store_keymap_option(&self, character_id: u32, option: KeymapOption, value: u32) {
        if let Err(e) = self.store.set_keymap_option(character_id, option, value) {
            crate::server::log(&format!("keymap: storing option {option:?} failed: {e}"));
        }
    }

    /// The packets that put a saved layout back, sent straight after `SetField`.
    ///
    /// Ordering matters and is not arbitrary: a reply sequence is a script the client walks in
    /// order, and the keymap manager is part of the stage this `SetField` is building. These
    /// go **after** it, never before.
    ///
    /// **Returns nothing at all for a character with nothing to restore**, which is today's
    /// behaviour exactly. `net::keymap::restore` explains why that is preferred over sending
    /// the keep-gate form: the client is measured-good under silence, and the keep form's
    /// no-op path in the client has not been read.
    pub(super) fn keymap_replies(&self) -> Vec<Reply> {
        let Some(chr) = self.claimed_character() else {
            return Vec::new();
        };
        let stored = self.store.keymap(chr.id).unwrap_or_default();
        let bindings: Vec<(u8, Binding)> = stored
            .iter()
            .map(|b| (b.preset, Binding { key: b.key, slot: Slot { kind: b.kind, action: b.action } }))
            .collect();
        let controller = bindings.iter().filter(|(t, _)| *t == net::keymap::CONTROLLER_TABLE).count();

        let mut out = Vec::new();
        match net::keymap::restore(Some(&bindings)) {
            Some(body) => out.push(Reply {
                opcode: net::keymap::KEYMAP_INIT,
                body,
                what: format!(
                    "FuncKeyMappedInit: four tables, all READ with {} slots each - keyboard \
                     presets 0..2 and the CONTROLLER table, each the image's factory with this \
                     character's saved bindings on top ({} keyboard, {controller} controller); \
                     quickslots not sent. One table alone was rejected by the client (0x009E) \
                     on 2026-09-12; a keep gate on the controller table handed it a keyboard \
                     layout on 2026-09-14. Nothing authenticates.",
                    net::keymap::SLOT_COUNT,
                    bindings.len() - controller
                ),
            }),
            // Not an error, and the two reasons need different work, so say which.
            None if !bindings.is_empty() => crate::server::log(&format!(
                "   keymap: character {} has {} saved binding(s) that were NOT restored - \
                 net::keymap::CLIENT_DEFAULT_LAYOUT is not measured yet, so the untouched \
                 slots would go out as zeros and unbind the keyboard. Run \
                 tools/keymapdump.py --rust against a running client and paste the table in.",
                chr.id,
                bindings.len()
            )),
            None => {}
        }

        for (option, opcode) in [
            (KeymapOption::A, net::keymap::KEYMAP_OPT_A),
            (KeymapOption::B, net::keymap::KEYMAP_OPT_B),
        ] {
            if let Ok(Some(v)) = self.store.keymap_option(chr.id, option) {
                out.push(Reply {
                    opcode,
                    body: net::keymap::keymap_opt(v),
                    what: format!("FuncKey option {option:?} = {v}. Nothing authenticates."),
                });
            }
        }
        out
    }
}
