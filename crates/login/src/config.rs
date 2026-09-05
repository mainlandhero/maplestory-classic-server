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
    /// **One address per channel**, and the channel count is `channels.len()`.
    ///
    /// Not a count plus a single address. A channel is its own server process, and the
    /// login server's job at migration time is to hand the client the address of the
    /// channel it is entering - so a channel with no address is a channel nobody can
    /// enter. Making the list the only source of the count means the two cannot drift:
    /// you cannot advertise five channels and run one.
    ///
    /// These are `SocketAddrV4` because the migration packet carries four octets straight
    /// into the client's `sockaddr_in`, so IPv6 is not representable - and they are what
    /// the **client machine** must be able to reach, not what any server bound.
    pub channels: Vec<SocketAddrV4>,
    /// Which channel the login result puts the player on, and which one it migrates to.
    pub channel_id: u32,
}

impl World {
    /// How many channels the world list advertises.
    pub fn channel_count(&self) -> u8 {
        self.channels.len().min(u8::MAX as usize) as u8
    }

    /// Where a channel listens, as the client must reach it.
    pub fn channel_address(&self, channel_id: u32) -> Option<SocketAddrV4> {
        self.channels.get(channel_id as usize).copied()
    }
}

impl Default for World {
    fn default() -> Self {
        World {
            id: 0,
            name: "Scania".to_string(),
            channels: vec!["127.0.0.1:8485".parse().expect("a literal address")],
            channel_id: 0,
        }
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

    /// **Which host the client is told to dial for a channel** - `--advertise`.
    ///
    /// `world.channels` carries the ports and the order. The host written into the migration
    /// packet is decided per connection by [`net::advertise::Advertiser`]: under `auto` a
    /// client on a private network is told the address it reached this server on, and a
    /// client on a public address is told this box's public address. Behind an `Arc` because
    /// the discovered public address is one fact for the whole process.
    pub advertise: std::sync::Arc<net::advertise::Advertiser>,

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
    /// go blank. It is a masked email in the real service.
    ///
    /// **This is the fallback now, not the answer.** It used to say "the accounts table holds
    /// no email, so this is configured until there is something real to show" - and the table
    /// holds one as of 2026-08-28. [`crate::session::Session::display_name`] derives the
    /// masked address from the account the connection is actually being served as, so the
    /// screen follows the launcher's claim instead of a value fixed at startup.
    ///
    /// This value is what an account with **no** email gets, which is every account created
    /// before the column existed. It exists because an empty string draws as a blank line
    /// where a person expects to see themselves, not because a constant is ever right.
    pub display_name: String,

    /// **Bind each minted migration to the launcher sign-in that authorised it.**
    ///
    /// Off, and it must stay off until the channel server can present a token.
    ///
    /// # Why an off switch rather than simply doing the right thing
    ///
    /// A bound migration can only be claimed by a connection presenting a matching session
    /// token - that is the invariant in `store::migration`, and it is what closes the
    /// impersonation hole. **The channel server presents nothing today.** The client does not
    /// carry the credential back: measured 2026-08-29 across 115 `0x007D` hello bodies
    /// against all 74 seeds ever minted, 8510 trials and zero hits, with a passing positive
    /// control. So with this on and nothing else changed, **every migration is refused and
    /// no character can enter the world** - the fix would read on screen as a total outage.
    ///
    /// Turning it on requires the hook to send the token on the channel connection. Until
    /// then this is deliberately, visibly unwired rather than quietly half-applied, and the
    /// startup banner says which state the server is in.
    ///
    /// The mechanism underneath is complete and tested either way; only the switch is off.
    pub bind_migrations: bool,

    pub world: World,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind: "127.0.0.1:8484".parse().expect("a literal address"),
            advertise: std::sync::Arc::new(net::advertise::Advertiser::default()),
            db_path: PathBuf::from("maplecw.db"),
            account: "maplecw".to_string(),
            display_name: "maplecw".to_string(),
            // OFF. See the field's doc block: on, with today's channel server, every
            // migration is refused and nobody can enter the world.
            bind_migrations: false,
            world: World::default(),
        }
    }
}
