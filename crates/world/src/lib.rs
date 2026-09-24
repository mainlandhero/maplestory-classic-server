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

pub mod broadcast;
pub mod chairs;
pub mod commodity;
pub mod crafting;
pub mod config;
/// The Maple Administrator's three once-a-day favours, and the words they say about them.
/// The gate itself is `store::dailyperks`; the grants are in `session::npc`.
pub mod dailyperks;
pub mod giftdrop;
pub mod dropsite;
pub mod drops;
pub mod droptables;
pub mod expcurve;
pub mod fields;
pub mod advbuffs;
pub mod firstjob;
pub mod firsttime;
pub mod consumables;
pub mod cosmetics;
pub mod damage;
pub mod footholds;
pub mod freemarket;
pub mod itemrecovery;
pub mod jobguide;
pub mod jobs;
pub mod leafcoupons;
pub mod logprune;
pub mod magicbox;
pub mod mobskills;
pub mod leveleffect;
pub mod link;
pub mod magic;
pub mod mesodrop;
pub mod mobattack;
pub mod mobshare;
pub mod party;
pub mod petcommands;
pub mod petlevel;
pub mod questitems;
pub mod questroom;
pub mod remoteattack;
pub mod returnscroll;
pub mod salon;
pub mod scrollnpc;
pub mod scriptportals;
pub mod scrolls;
pub mod secondjob;
pub mod shanks;
pub mod server;
pub mod serverclock;
pub mod session;
pub mod skillpoints;
pub mod skilltable;
pub mod slotcoupons;
pub mod shops;
pub mod signaturestyle;
pub mod taxi;
pub mod thirdjob;

pub use config::Config;
pub use server::serve;
pub use session::{Reply, Session};
pub use shops::{ItemData, Shop, ShopItem, ShopTable};
