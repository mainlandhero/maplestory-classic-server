//! The MapleCW login server.
//!
//! This replaces `tools/handshake_probe.py` for everything except packet capture. The
//! probe replayed bodies handed to it on a command line; it could not persist anything,
//! hold account state, or answer a request in a way that depended on what came before -
//! and all three of those are most of what a login server does.
//!
//! # The shape, and why it is split this way
//!
//! * The greeting lives in `net::handshake` - it is the transport's, not login's, and the
//!   channel server needs the identical bytes.
//! * [`session`] is the whole protocol as a **pure state machine**: bodies in, bodies out,
//!   no socket and no cipher. Every exchange measured against the real client is a unit
//!   test there, which is the only way this stays honest without a client launch per
//!   change - and a client launch costs the owner a manual, elevated relaunch.
//! * [`server`] is the socket loop, the framing and the log. It owns nothing about the
//!   protocol beyond wiring.
//!
//! # What is not authenticated, stated plainly
//!
//! **Nothing on this socket proves who the player is.** The client's login request
//! (`0x0080`) carries no credentials - the login form is vestigial, and the server supplies
//! both the login result and the account name. So every connection is served as the account
//! named in [`Config::account`], and two different people connecting are the same account.
//!
//! That is a real gap, not an oversight, and closing it is the launcher work in
//! `docs/launcher.md`: the launcher authenticates over HTTPS, receives a single-use token,
//! and the token reaches this server so it can call `/consume`. Whether the client will
//! carry that token in `0x0073` is **not yet measured** - see `docs/deployment.md`. Until
//! it is, do not describe a session here as authenticated.

pub mod config;
pub mod server;
pub mod session;

pub use config::{Config, World};
pub use server::{delete, list, serve};
pub use session::{Reply, Session};
