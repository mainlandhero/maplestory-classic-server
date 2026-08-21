//! The MapleCW game world: **one server per channel**.
//!
//! # Why this is a separate crate and a separate process
//!
//! The owner, 2026-08-19: the login/character-select server has to be separate from the game
//! world, and each channel handles migrations to and from character select and to other
//! channels. That is how the real service is shaped, and the first run that migrated a
//! character showed why it matters here and not just in principle:
//!
//! The client entered the world, closed the login socket, and reconnected - and the login
//! server answered that second connection with a *login* greeting and an unprompted
//! `0x0032` startup gate. The client showed **"The client is outdated"** and exited. A
//! login server answering a game connection is wrong by construction, whatever the dialog
//! turns out to be caused by.
//!
//! # What a channel server does *not* do
//!
//! It does not send the `0x0032` startup gate on connect. That packet releases the
//! *startup* loop on the login connection; there is no startup loop here, and sending it
//! was the most likely proximate cause of the rejection above. The greeting is byte
//! identical to login's, because it is the transport's greeting rather than login's -
//! that is why it lives in `net::handshake`.
//!
//! # What is not authenticated, stated plainly
//!
//! A migration seed is a `u32` - all the client's migration packet has room for - so it
//! identifies a pending migration rather than proving who is on the far end. What the
//! store gives is **single use**. See `store::migration`.

pub mod config;
pub mod drops;
pub mod droptables;
pub mod expcurve;
pub mod fields;
pub mod consumables;
pub mod damage;
pub mod footholds;
pub mod jobs;
pub mod server;
pub mod session;
pub mod shops;

pub use config::Config;
pub use server::serve;
pub use session::{Reply, Session};
pub use shops::{ItemData, Shop, ShopItem, ShopTable};
