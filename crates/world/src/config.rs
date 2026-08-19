//! What one channel server needs to know before it can listen.

use std::net::SocketAddr;
use std::path::PathBuf;

/// One channel of one world.
///
/// A channel is a process, not a thread: the login server hands the client an address and
/// the client connects to it, so every channel needs its own listener and its own
/// advertised address. `world_id` and `channel_id` are here so the log says which channel
/// a line belongs to, and so a migration minted for channel 1 is not claimed by channel 2.
#[derive(Debug, Clone)]
pub struct Config {
    /// What this channel listens on.
    pub bind: SocketAddr,

    /// The SQLite file. Shared with the login server - that is how the migration handoff
    /// crosses the process boundary.
    pub db_path: PathBuf,

    pub world_id: u32,
    pub channel_id: u32,

    /// Answer the migration hello with the fixed head of a `SetField`, and nothing after.
    ///
    /// **Off by default, and it should stay off except during a run that is measuring
    /// something.** It cannot put a character in a map - `characterData` is `0` and the
    /// branch that does carry a character calls an 18525-byte record decoder nobody has
    /// read yet. What it is for is one question static analysis cannot answer: `SetField`'s
    /// handler has two silent early returns, so "the client did nothing" and "the client
    /// never received it" look identical from here. Send this with a watch armed on
    /// `142097f80` and the two become distinguishable.
    ///
    /// See `research/msexe-stage-setfield.md`.
    pub set_field_probe: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind: "127.0.0.1:8485".parse().expect("a literal address"),
            db_path: PathBuf::from("maplecw.db"),
            world_id: 0,
            channel_id: 0,
            set_field_probe: false,
        }
    }
}
