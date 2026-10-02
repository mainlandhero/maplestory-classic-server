//! **The client's options, kept per account** - `0x02EB` in, quest ex records out.
//!
//! The owner, 2026-10-02, from player complaints: *"Saving of settings such as audio, HP thresholds,
//! etc on server side per account"*, the HP threshold being the pet's auto-potion one. The
//! client had been sending every change all along and reading them back from two quest-ex
//! stores at field entry; with nothing stored, every login started from the defaults.
//! `net::clientsettings` has the client side; `store::clientsettings` the table.

use super::{Reply, Session};

impl Session {
    /// `0x02EB` - one or more options changed. Stored for the account; nothing goes back,
    /// because the client already shows its own change and the builder sets no latch.
    pub(super) fn on_options_changed(&mut self, body: &[u8]) -> Vec<Reply> {
        let Some(sync) = net::clientsettings::parse_options_changed(body) else {
            crate::server::log(&format!("   options: a {} byte 0x02EB that does not parse; not stored", body.len()));
            return Vec::new();
        };
        let Some(account) = self.claimed.as_ref().map(|c| c.account_id) else { return Vec::new() };
        let kept: Vec<(u32, i32)> = sync
            .entries
            .iter()
            .copied()
            .filter(|(k, _)| net::clientsettings::key_name(sync.group, *k).is_some())
            .collect();
        let named: Vec<String> = kept
            .iter()
            .map(|(k, v)| format!("{}={v}", net::clientsettings::key_name(sync.group, *k).unwrap_or("?")))
            .collect();
        let result = self.store.save_client_settings(account, sync.group, &kept);
        crate::server::log(&format!(
            "   options: account {account} group {} - {} of {} kept [{}]{}",
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
    pub(super) fn option_records(&self) -> (Vec<(u32, String)>, Vec<(u32, String)>) {
        let Some(account) = self.claimed.as_ref().map(|c| c.account_id) else { return (Vec::new(), Vec::new()) };
        let rows = self.store.client_settings(account).unwrap_or_default();
        net::clientsettings::records(&rows)
    }
}
