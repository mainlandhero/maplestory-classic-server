//! **The live server's status in Discord** - one message, edited in place.
//!
//! The owner, 2026-09-30: *"I want to be able to integrate with discord ... This URL should be
//! configurable and only used on the live server instead of the test server. It should display
//! information about current server status and online members in each channel as well as current
//! up time and patch version."* And: *"Make sure that webhook only posts the status once and then
//! continuously update that same message ID."*
//!
//! # Where it runs, and why only on the live server
//!
//! In the world hub (`maplecw-chat`), which already holds the one cross-channel list of who is
//! online where. It runs **only** when the hub is started with `--discord-webhook-file PATH`;
//! the packaged `start-server.ps1` passes it when `discord-webhook.txt` is in the server folder,
//! and `tools/test-server.ps1` never passes it - so a test server cannot post, whatever files lie
//! around the repository.
//!
//! # One message, forever
//!
//! The first run `POST`s the message and writes its id to `discord-status-message.txt` beside
//! the webhook file. Every refresh after that - and every later start - `PATCH`es that id. A new
//! message is posted **only** when Discord answers the edit with 404, i.e. somebody deleted it.
//! A network failure, a rate limit or a Discord outage keeps the id and tries again next minute;
//! it never posts a second message.
//!
//! # Offline for maintenance
//!
//! The owner, 2026-09-30: *"Upon server shutdown, it should also update the message with that server
//! is offline for maintenance."* The stop script kills processes outright, which runs no code in
//! them, so the message is turned red from two places, both **edits of the same message** -
//! [`Reporter::publish_offline`] never posts:
//!
//! * `maplecw-chat --discord-offline --discord-webhook-file PATH`, a one-shot run that
//!   `start-server.ps1` makes before it stops the servers (`-Stop`, and the `finally` that runs
//!   when its window's Ctrl+C ends the wait);
//! * the hub's own console handler, for a window closed with X, a logoff or a Windows shutdown.
//!
//! # What it says
//!
//! Status, uptime (of the hub, which starts and stops with the channels), each channel's players
//! by name, the total, the server build (the hub's own `store::buildstamp`: build time and
//! digest) and the client patch the sign-in server publishes (`client-patch-version.txt`, which
//! `maplecw-auth` writes beside itself). The embed's timestamp is the last refresh, so a stopped
//! server reads as a message that has stopped moving.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use webhook::{json_str, Edited, Webhook};

/// How often the message is refreshed.
pub const REFRESH_SECS: u64 = 60;

/// Discord's limit on one embed field's value.
const FIELD_MAX: usize = 1024;

/// One refresh's facts.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Status {
    pub uptime_secs: u64,
    /// channel (0-based) -> (connected now, the names on it, sorted).
    pub channels: BTreeMap<u32, (bool, Vec<String>)>,
    pub build: String,
    pub client_patch: Option<String>,
    /// RFC 3339, the embed's timestamp.
    pub now_rfc3339: String,
}

/// `3d 4h 12m`, `4h 12m`, `12m`.
pub fn uptime_text(secs: u64) -> String {
    let (d, h, m) = (secs / 86_400, secs % 86_400 / 3_600, secs % 3_600 / 60);
    match (d, h) {
        (0, 0) => format!("{m}m"),
        (0, _) => format!("{h}h {m}m"),
        _ => format!("{d}d {h}h {m}m"),
    }
}

/// `2026-09-30T14:02:05Z` for a unix second.
pub fn rfc3339(unix: i64) -> String {
    let day = unix.div_euclid(86_400);
    let secs = unix.rem_euclid(86_400);
    format!("{}T{:02}:{:02}:{:02}Z", store::dailyperks::utc_date(day), secs / 3_600, secs % 3_600 / 60, secs % 60)
}

/// The names, comma-separated, cut to fit a field with "and N more".
fn names_value(names: &[String]) -> String {
    if names.is_empty() {
        return "No one online".to_string();
    }
    let mut out = String::new();
    for (i, n) in names.iter().enumerate() {
        let piece = if out.is_empty() { n.clone() } else { format!(", {n}") };
        let rest = names.len() - i;
        if out.len() + piece.len() + 20 > FIELD_MAX {
            out.push_str(&format!(" and {rest} more"));
            break;
        }
        out.push_str(&piece);
    }
    out
}

/// The webhook body: one embed.
pub fn status_json(s: &Status) -> String {
    let total: usize = s.channels.values().map(|(_, n)| n.len()).sum();
    let any_up = s.channels.values().any(|(up, _)| *up);
    let (state, colour) = if any_up { ("🟢 Online", 0x2ECC71) } else { ("🟠 Hub up, no channels connected", 0xE67E22) };
    let field = |name: &str, value: &str, inline: bool| {
        format!("{{\"name\":{},\"value\":{},\"inline\":{inline}}}", json_str(name), json_str(value))
    };
    let mut fields = vec![
        field("Status", state, true),
        field("Uptime", &uptime_text(s.uptime_secs), true),
        field("Players online", &total.to_string(), true),
    ];
    for (ch, (up, names)) in &s.channels {
        let title = if *up { format!("Channel {} ({})", ch + 1, names.len()) } else { format!("Channel {} (offline)", ch + 1) };
        let value = if *up { names_value(names) } else { "Not connected".to_string() };
        fields.push(field(&title, &value, false));
    }
    fields.push(field("Server build", &s.build, true));
    fields.push(field("Client patch", s.client_patch.as_deref().unwrap_or("unknown"), true));
    format!(
        "{{\"username\":\"MapleCW\",\"allowed_mentions\":{{\"parse\":[]}},\"embeds\":[{{\"title\":\"MapleCW Server Status\",\
         \"color\":{colour},\"fields\":[{}],\"footer\":{{\"text\":\"Refreshed every minute - if this time stops moving, the server is down\"}},\
         \"timestamp\":{}}}]}}",
        fields.join(","),
        json_str(&s.now_rfc3339)
    )
}

/// The body for "offline for maintenance": red, no channels, the build and patch it was on.
pub fn offline_json(build: &str, client_patch: Option<&str>, now_rfc3339: &str) -> String {
    let field = |name: &str, value: &str, inline: bool| {
        format!("{{\"name\":{},\"value\":{},\"inline\":{inline}}}", json_str(name), json_str(value))
    };
    let fields = [
        field("Status", "🔴 Offline for maintenance", false),
        field("Players online", "0", true),
        field("Server build", build, true),
        field("Client patch", client_patch.unwrap_or("unknown"), true),
    ];
    format!(
        "{{\"username\":\"MapleCW\",\"allowed_mentions\":{{\"parse\":[]}},\"embeds\":[{{\"title\":\"MapleCW Server Status\",\
         \"color\":{},\"description\":\"The server is offline for maintenance. This message turns green again when it is back.\",\
         \"fields\":[{}],\"footer\":{{\"text\":\"Went offline\"}},\"timestamp\":{}}}]}}",
        0xE74C3C,
        fields.join(","),
        json_str(now_rfc3339)
    )
}

/// The server build, from the running executable: `2026-09-30 14:02 UTC - a1b2c3d4e5f6a7b8`.
pub fn build_text() -> String {
    let b = store::buildstamp::stamp();
    let when = b.modified.map(|m| rfc3339(m).replace('T', " ")[..16].to_string() + " UTC");
    match (when, b.digest) {
        (Some(w), Some(d)) => format!("{w} - {d}"),
        (Some(w), None) => w,
        (None, Some(d)) => d,
        (None, None) => b.version.to_string(),
    }
}

/// The hub side: the webhook, the one message id, and the files that keep it.
pub struct Reporter {
    hook: Webhook,
    id_file: PathBuf,
    patch_file: PathBuf,
    message_id: Option<String>,
    seen: BTreeSet<u32>,
    started: std::time::Instant,
    build: String,
}

impl Reporter {
    /// From the file holding the URL (one line). `Err` says why, without the URL.
    pub fn from_file(path: &Path) -> Result<Reporter, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let hook = Webhook::parse(text.lines().next().unwrap_or("")).map_err(|e| format!("{}: {e}", path.display()))?;
        let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
        let id_file = dir.join("discord-status-message.txt");
        let message_id = std::fs::read_to_string(&id_file).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        Ok(Reporter {
            hook,
            id_file,
            patch_file: dir.join("client-patch-version.txt"),
            message_id,
            seen: BTreeSet::new(),
            started: std::time::Instant::now(),
            build: build_text(),
        })
    }

    pub fn describe(&self) -> String {
        format!(
            "DISCORD STATUS: editing one message on {} every {REFRESH_SECS}s ({}); message id kept in {}",
            self.hook.redacted(),
            self.message_id.as_deref().map_or("none yet - the first refresh posts it".to_string(), |id| format!("message {id}")),
            self.id_file.display()
        )
    }

    /// The facts for this refresh from the hub's roster: `(channel, name)` per online
    /// character, and the channels connected now. A channel seen once and gone since is shown
    /// as offline rather than dropped.
    pub fn status(&mut self, roster: &[(u32, String)], connected: &[u32], now_unix: i64) -> Status {
        self.seen.extend(connected.iter().copied());
        self.seen.extend(roster.iter().map(|(c, _)| *c));
        let mut channels: BTreeMap<u32, (bool, Vec<String>)> =
            self.seen.iter().map(|c| (*c, (connected.contains(c), Vec::new()))).collect();
        for (c, name) in roster {
            channels.entry(*c).or_default().1.push(name.clone());
        }
        for (_, names) in channels.values_mut() {
            names.sort_by_key(|n| n.to_lowercase());
        }
        let client_patch = std::fs::read_to_string(&self.patch_file).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        Status {
            uptime_secs: self.started.elapsed().as_secs(),
            channels,
            build: self.build.clone(),
            client_patch,
            now_rfc3339: rfc3339(now_unix),
        }
    }

    /// **Offline for maintenance** - an EDIT of the one message, never a post: with no message
    /// yet there is nothing in the channel to correct, and a message that was deleted stays
    /// deleted until the next start posts it.
    pub fn publish_offline(&mut self, now_unix: i64) -> Result<String, String> {
        let Some(id) = self.message_id.clone() else {
            return Ok("no status message yet - nothing to mark offline".to_string());
        };
        let patch = std::fs::read_to_string(&self.patch_file).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        let json = offline_json(&self.build, patch.as_deref(), &rfc3339(now_unix));
        match self.hook.edit(&id, &json) {
            Ok(Edited::Ok) => Ok(format!("marked message {id} offline for maintenance")),
            Ok(Edited::Gone) => Ok(format!("message {id} is gone - nothing to mark offline")),
            Err(e) => Err(format!("could not mark message {id} offline: {e}")),
        }
    }

    /// Edit the one message; post it only when there is none yet or Discord says it is gone.
    pub fn publish(&mut self, json: &str) -> Result<String, String> {
        if let Some(id) = self.message_id.clone() {
            match self.hook.edit(&id, json) {
                Ok(Edited::Ok) => return Ok(format!("edited message {id}")),
                Ok(Edited::Gone) => {
                    crate::server::log(&format!("discord: message {id} is gone (deleted in Discord?) - posting a new one"));
                }
                // Anything else is transient: keep the id, try again next refresh. Never a
                // second message.
                Err(e) => return Err(format!("edit of message {id} failed, will retry: {e}")),
            }
        }
        let id = self.hook.post(json).map_err(|e| format!("post failed, will retry: {e}"))?;
        if let Err(e) = std::fs::write(&self.id_file, &id) {
            crate::server::log(&format!(
                "discord: posted message {id} but could NOT save its id to {} ({e}) - a restart will post again",
                self.id_file.display()
            ));
        }
        self.message_id = Some(id.clone());
        Ok(format!("posted message {id}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reporter(dir: &Path) -> Reporter {
        let file = dir.join("discord-webhook.txt");
        std::fs::write(&file, "https://discord.com/api/webhooks/123/tok_en\n").unwrap();
        Reporter::from_file(&file).unwrap()
    }

    fn scratch(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("maplecw-discord-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn uptime_and_timestamps_read_the_way_people_write_them() {
        assert_eq!(uptime_text(59), "0m");
        assert_eq!(uptime_text(3_720), "1h 2m");
        assert_eq!(uptime_text(3 * 86_400 + 4 * 3_600 + 12 * 60), "3d 4h 12m");
        assert_eq!(rfc3339(20_726 * 86_400 + 14 * 3_600 + 2 * 60 + 5), "2026-09-30T14:02:05Z");
    }

    /// Each channel with its players, a channel that went away shown offline, the total, the
    /// build and the client patch - and the names are the hub's, sorted.
    #[test]
    fn the_message_lists_each_channel_and_its_players() {
        let dir = scratch("status");
        let mut r = reporter(&dir);
        std::fs::write(dir.join("client-patch-version.txt"), "0123456789abcdef\n").unwrap();
        let _ = r.status(&[], &[0, 1], 0);
        let s = r.status(&[(0, "mint".into()), (0, "Wisp".into())], &[0], 1_000);
        assert_eq!(s.channels[&0], (true, vec!["mint".to_string(), "Wisp".to_string()]));
        assert_eq!(s.channels[&1], (false, vec![]), "seen before, gone now");
        assert_eq!(s.client_patch.as_deref(), Some("0123456789abcdef"));
        let json = status_json(&s);
        for want in ["\"Channel 1 (2)\"", "\"mint, Wisp\"", "\"Channel 2 (offline)\"", "\"Players online\",\"value\":\"2\"", "\"0123456789abcdef\"", "Online", "\"timestamp\":\"1970-01-01T00:16:40Z\""] {
            assert!(json.contains(want), "{want} in {json}");
        }
        assert!(!json.contains("tok_en"), "the token is never in the body");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The id file is what makes it ONE message across restarts: read at start, written after
    /// a post. And the Debug/describe forms never carry the token.
    #[test]
    fn the_message_id_survives_a_restart_and_the_token_never_prints() {
        let dir = scratch("id");
        let r = reporter(&dir);
        assert_eq!(r.message_id, None);
        std::fs::write(dir.join("discord-status-message.txt"), "987654321\n").unwrap();
        let r = reporter(&dir);
        assert_eq!(r.message_id.as_deref(), Some("987654321"));
        assert!(r.describe().contains("message 987654321") && !r.describe().contains("tok_en"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The offline body is red and says maintenance; with no message id the offline call posts
    /// nothing (it returns before touching the network).
    #[test]
    fn offline_is_red_and_never_posts_a_new_message() {
        let json = offline_json("2026-09-30 14:02 UTC - abc", Some("0123"), "2026-09-30T15:00:00Z");
        assert!(json.contains("Offline for maintenance") && json.contains(&0xE74C3C.to_string()) && json.contains("\"0123\""), "{json}");
        let dir = scratch("offline");
        let mut r = reporter(&dir);
        assert_eq!(r.publish_offline(0), Ok("no status message yet - nothing to mark offline".to_string()));
        assert!(!dir.join("discord-status-message.txt").exists(), "nothing posted, nothing saved");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_long_channel_is_cut_to_fit_discord() {
        let names: Vec<String> = (0..300).map(|i| format!("Player{i:03}")).collect();
        let v = names_value(&names);
        assert!(v.len() <= FIELD_MAX && v.ends_with("more"), "{}", v.len());
        assert_eq!(names_value(&[]), "No one online");
    }
}
