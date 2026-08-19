//! What the server needs to know before it can listen.

use std::net::{SocketAddr, SocketAddrV4};
use std::path::PathBuf;

/// The world this login server presents.
///
/// One world for now. The client looks its world up in the list built from `WORLD_LIST`
/// and refuses to proceed if the login result names a world that was never sent, so the
/// id here has to be the id in both places - which is why they come from one struct.
#[derive(Debug, Clone)]
pub struct World {
    pub id: u32,
    pub name: String,
    pub channels: u8,
    /// Which channel the login result puts the player on.
    pub channel_id: u32,
}

impl Default for World {
    fn default() -> Self {
        World { id: 0, name: "Scania".to_string(), channels: 1, channel_id: 0 }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    /// What the socket listens on.
    ///
    /// Defaults to loopback, which is what the probe did and what every run so far has
    /// used. Moving the server to the homelab means passing `0.0.0.0:8484` deliberately -
    /// binding every interface by default would widen exposure without anyone asking for
    /// it. See `docs/deployment.md`, and note that the client machine's firewall rule has
    /// to be reshaped before an off-box server can be reached at all.
    pub bind: SocketAddr,

    /// **The address the client is told to reconnect to when it enters the world.**
    ///
    /// Not the same thing as [`Config::bind`], and the difference is the whole reason this
    /// field exists. `bind` is where *this process* listens; `advertise` is what goes into
    /// the migration packet, so it has to be an address the **client machine** can reach.
    /// On loopback they are the same; the moment the server moves to the homelab they are
    /// not, and a server that advertises its own bind address sends the client to itself.
    ///
    /// It is `SocketAddrV4` rather than `SocketAddr` on purpose: the migration packet
    /// carries four octets straight into the client's `sockaddr_in`, so an IPv6 address is
    /// not representable and should fail at the type level rather than at runtime.
    pub advertise: SocketAddrV4,

    /// The SQLite file. Characters live here, and this is the whole point of the crate.
    pub db_path: PathBuf,

    /// **Which account every connection is served as.**
    ///
    /// The game socket carries no credentials, so there is nothing to authenticate with
    /// yet. This is a stand-in, and it is deliberately a required piece of configuration
    /// rather than a silent default, so nobody can read a run and think a login happened.
    pub account: String,

    /// What the login screen displays as the account name.
    ///
    /// Server-supplied: the client cannot compute it, and leaving it out makes the field
    /// go blank. It is a masked email in the real service. The accounts table holds no
    /// email, so this is configured until there is something real to show.
    pub display_name: String,

    pub world: World,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind: "127.0.0.1:8484".parse().expect("a literal address"),
            advertise: "127.0.0.1:8484".parse().expect("a literal address"),
            db_path: PathBuf::from("maplecw.db"),
            account: "maplecw".to_string(),
            display_name: "maplecw".to_string(),
            world: World::default(),
        }
    }
}
