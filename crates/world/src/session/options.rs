//! **The client's options, kept per account** - `0x02EB` in, quest ex records out.
//!
//! The owner, 2026-10-02, from player complaints: *"Saving of settings such as audio, HP thresholds,
//! etc on server side per account"*, the HP threshold being the pet's auto-potion one. The
//! client had been sending every change all along and reading them back from two quest-ex
//! stores at field entry; with nothing stored, every login started from the defaults.
//! `net::clientsettings` has the client side; `store::clientsettings` the table.
//!
//! **The pet's auto-potion keys are per character** (the owner, 2026-10-02: *"other characters may
//! want to use different auto potion setup"*) - `net::clientsettings::PER_CHARACTER_KEYS`. A
//! character that has never set one gets the account's row for it, which is where every such
//! key lived before that day, so nobody's threshold resets on the upgrade; from the first
//! change on, the character's own row wins. The potions the pet drinks were already per
//! character: they are keymap options A and B (`store::keymap`).

use super::{Reply, Session};

impl Session {
    /// `0x02EB` - one or more options changed. Stored for the account; nothing goes back,
    /// because the client already shows its own change and the builder sets no latch.
    pub(super) fn on_options_changed(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(sync) = net::clientsettings::parse_options_changed(body) else {
            crate::server::log(&format!("   options: a {} byte 0x02EB that does not parse; not stored", body.len()));
            return Vec::new();
        };
        let Some((account, character)) = self.claimed.as_ref().map(|c| (c.account_id, c.character_id)) else {
            return Vec::new();
        };
        let kept: Vec<(u32, i32)> = sync
            .entries
            .iter()
            .copied()
            .filter(|(k, _)| net::clientsettings::key_name(sync.group, *k).is_some())
            .collect();
        let named: Vec<String> = kept
            .iter()
            .map(|(k, v)| {
                let name = net::clientsettings::key_name(sync.group, *k).unwrap_or("?");
                if net::clientsettings::is_per_character(sync.group, *k) {
                    format!("{name}={v} (character)")
                } else {
                    format!("{name}={v}")
                }
            })
            .collect();
        let (mine, shared): (Vec<(u32, i32)>, Vec<(u32, i32)>) =
            kept.iter().partition(|(k, _)| net::clientsettings::is_per_character(sync.group, *k));
        let result = self
            .store
            .save_client_settings(account, sync.group, &shared)
            .and_then(|()| self.store.save_character_settings(character, sync.group, &mine));
        crate::server::log(&format!(
            "   options: account {account} character {character} group {} - {} of {} kept [{}]{}",
            sync.group,
            kept.len(),
            sync.entries.len(),
            named.join(", "),
            match result {
                Ok(()) => String::new(),
                Err(e) => format!(" - NOT SAVED: {e}"),
            }
        ));
        Vec::new()
    }

    /// The stored options as the two record blocks want them: `(block #28, block #32)`.
    /// Empty for a session with no account, or on a database error - the record then simply
    /// carries no options and the client uses its defaults, as before.
    ///
    /// The account's rows overlaid with the character's own, which win per `(group, key)`.
    pub(super) fn option_records(&self) -> (Vec<(u32, String)>, Vec<(u32, String)>) {
        let Some((account, character)) = self.claimed.as_ref().map(|c| (c.account_id, c.character_id)) else {
            return (Vec::new(), Vec::new());
        };
        let mut rows: std::collections::BTreeMap<(u32, u32), i32> = self
            .store
            .client_settings(account)
            .unwrap_or_default()
            .into_iter()
            .map(|(g, k, v)| ((g, k), v))
            .collect();
        for (g, k, v) in self.store.character_settings(character).unwrap_or_default() {
            rows.insert((g, k), v);
        }
        let rows: Vec<(u32, u32, i32)> = rows.into_iter().map(|((g, k), v)| (g, k, v)).collect();
        net::clientsettings::records(&rows)
    }
}
