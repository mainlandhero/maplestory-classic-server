//! The Victoria Island taxis: click a cab, pick a town off a list, pay a flat fare, arrive.
//!
//! The owner, 2026-08-29: *"program the different Taxi NPCs so that the users can click on them
//! and pay a modest fee (500 mesos) to be teleported to those maps that the Taxi are on.
//! Lith Harbor Taxi NPC is actually Lyn, they're the tour guide around Victoria Island.
//! Functionally the same as a Taxi, but their dialogue should be of one of tour guides."*
//!
//! Labels are the project's: **[L]** read off this client's listing, its WZ or a capture,
//! **[D]** derived from two or more [L] facts, **[I]** inferred - policy nothing on this
//! machine can confirm.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials. A ride is sold
//! to whoever holds the socket.
//!
//! # This module is the decision and the words; it is NOT wired
//!
//! It owns the table, the fare, the guards and the text. It knows nothing about `Session`,
//! so every branch below is a unit test rather than a client run. §"WIRE IT LIKE THIS" at
//! the bottom is the patch. `CLAUDE.md`'s *built is not wired*: until something calls
//! [`taxi_for`], a taxi on screen is identical to no taxi at all.
//!
//! # The menu is `0x055B` message type **6**, and my first pass said it did not exist
//!
//! **Retracted, 2026-08-29.** The first version of this module reported *"this client has no
//! select-one-of-N script box"* and built a chain of five yes/no boxes instead. That negative
//! was wrong, and the way it was reached is worth more than the conclusion.
//!
//! The method was right: enumerate all 71 entries of the type table at `0x141f6f9f4`, and for
//! each handler record what it reads from the body and what its `0x00F3` answers with. Two
//! things defeated it, and **both were in the filter, not in the data**:
//!
//! * **Wrong shape.** I looked for a handler that reads a *repeated string* - a count, then N
//!   strings - because that is what "a list of five destinations" looks like from the server
//!   side. Type 6 reads **one string**, exactly like the yes/no box, and the list lives in
//!   `#L<n>#` markup inside it that the client parses into clickable lines. Type 2 *does* read
//!   count-then-N-strings, which is why it looked like the only candidate and why its missing
//!   selection field read as "no menu exists anywhere". `CLAUDE.md`: *"the two worst wrong
//!   answers here both came from searching a known list - one looked for the wrong shape."*
//!
//! * **My write-extractor could only see one branch.** It walked each function linearly and
//!   stopped collecting at the **first** `send`. Type 6's cancel path sends at `141f73987`,
//!   which is laid out *before* the selection path's writes at `141f7398e..141f739a4`, so the
//!   tool reported type 6's answer as `u32,u8,u8,[send]` - the six-byte cancel - and I read
//!   that as "carries no index, therefore identical to type 3". The type `0x13` result from
//!   the same run was right only because its blocks happened to be ordered the other way.
//!   **A linear write-extractor reports whichever branch the compiler laid out first and says
//!   nothing about the rest**, which is the `[reg+disp]` write-scan blind spot wearing new
//!   clothes.
//!
//! And the cheapest check of all was never run: **`research/npc-click.md:196` already had the
//! answer**, `141e3db1b FUN_142a61900(ui, 6, npcTemplateId, &text)  ; type 6 = a list` with
//! `141e3dba6 sel = FUN_142a64400(ui)`, labelled **[L]**, sitting in the repo the whole time.
//! `CLAUDE.md` says to grep the *contents* of `research/`, not to scan for a filename that
//! sounds relevant. Two passes over the binary cost more than one `grep -rn '#L'` would have.
//!
//! ## What type 6 actually is, re-read here rather than taken on report
//!
//! Body, `FUN_141f73740` - **two reads and no others**, the same two as the yes/no box:
//!
//! ```text
//! [u32 speakerOverride, only if flags & 0x04]
//! str  the menu text          u16 BYTE count, then that many bytes
//! ```
//!
//! Answer, read off the listing at `141f73940..141f739a4` **[L]**:
//!
//! ```text
//! 141f73940  call 0x142a64400     ; sel = the line the player clicked
//! 141f73950  COutPacket(0x00F3)
//! 141f7395c  w_u32 0              ; handle - a literal zero, never the one we sent
//! 141f73967  w_u8  6              ; the echoed message type
//! 141f73970  lea ecx,[r14-1] / test ecx, 0xffffdfff   ; true iff r14 is 1 or 0x2001
//!   je  ->  141f7398e  w_u8 1 ; w_u32 sel ; send      -> 10 bytes, a selection
//!   else -> 141f7397c  w_u8 0 ;             send      ->  6 bytes, a cancel
//! ```
//!
//! `r14 == 1 || r14 == 0x2001` is the same acceptance test `research/npc-click.md` records for
//! the client's own menu (`141e3db96 if (r != 1 && r != 0x2001) -> cancelled`). **[L]**
//!
//! The selectable-line format is the client's own literal, `#d#L%d# %s#l#k` - not invented
//! here: `research/msexe-packet-fields.txt` row `0x00F2`, the text `FUN_141e3c5d0` builds for
//! the client's own NPC menu. Corroborated by the data: **33** menus in
//! `gm-handbook/questlines.txt` are written with `#L0#`..`#L4#`. **[L]**
//!
//! ## Why this module carries its own encoder and decoder
//!
//! It was meant to share `crate::jobguide`'s. **That module's type-6 code no longer exists**:
//! while this file was being written, the Phil agent withdrew its menu implementation and
//! rebuilt Phil on a yes/no chain, taking `menu_body`, `parse_menu_reply`, `MenuReply` and
//! `SCRIPT_TYPE_MENU` with it. Compiling against a live agent's surface is what broke, and
//! `CLAUDE.md` already says why: *an agent's files are its files until it reports back.* So
//! [`menu_body`] and [`parse_menu_reply`] here are derived from this file's own read of
//! `FUN_141f73740`, with the addresses above.
//!
//! **The right home for both is `net::script::npc_menu(speaker, text)`**, next to `npc_say`
//! and `npc_ask`, where neither agent's churn can reach them. `crates/net/` is not this
//! agent's to edit. Flagged in §"WIRE IT LIKE THIS" rather than done.
//!
//! ## The one thing nobody has measured, quoted rather than paraphrased
//!
//! The Phil agent withdrew type 6 for a reason it stated precisely, and `CLAUDE.md` says to
//! quote a named blind spot when acting on it rather than to summarise it:
//!
//! > the fill site of `[ui+0x3e0]` was not isolated - `FUN_142a61900` *releases and clears* it
//! > at `142a619f4..142a61a0b`, and whatever repopulates it from the `#`-token expander later
//! > in the same function was not read. So "a server-sent type 6 renders `#L` lines" is
//! > **[D]**, and **nobody has ever watched one draw**.
//!
//! That is a fair statement of the evidence and this module does not contradict it. What it
//! *is* built on is the coordinator's decision to spend the run on the one-box menu, plus
//! three things that are [L]: the answer carries a `u32` selection, the client's own menu is
//! built the same way through the same constructor, and 33 of this client's own authored
//! menus use the markup.
//!
//! **The fallback is one line and it is written down**, so a run that comes back wrong costs
//! an edit rather than a redesign - see §"IF THE LIST DOES NOT RENDER".
//!
//! ## `net::script::parse_script_reply` cannot read a type-6 answer, and fails silently
//!
//! Its type switch has arms for `0x03`/`0x10` and falls through to a **Say**-shaped
//! `u32 echo, str text, u8 action`. Against a 10-byte type-6 body the string's `u16` length
//! wants two bytes that are not there, `PacketReader` errors, and the whole packet is dropped
//! - which the caller turns into silence. That is the same failure that cost Roger's quest.
//! The wiring patch therefore calls the menu decoders **before** `parse_script_reply`.
//!
//! # Line breaks: the two characters `\` and `n`
//!
//! The client's `#`-token expander `FUN_142a45580` treats a token beginning with **`\r`
//! (`142a45b3f cmp byte ptr [rax], 0xd`)** and one beginning with **`\` (`142a45b80 cmp byte
//! ptr [rax], 0x5c`)** identically: both `je 0x142a4d9bd`, which bumps the line counter at
//! `[rbp+0x30]` and calls the line-flush `FUN_142a4eb70`. **[L]**
//!
//! That expander is on this path: `FUN_142a61900`, the text setter every one of these boxes
//! goes through, calls it at `142a61fa1` and `142a6206b`; the Say handler reaches
//! `FUN_142a61900` at `141f6fcb1` and the yes/no handler at `141f7098b`. Verified here, not
//! taken from `research/`. **[L]**
//!
//! Either byte sequence would therefore work, and **the backslash one is used** for two
//! reasons: it is what this client's own content uses - 144 `Say` lines in `Quest.wz`, and
//! not one real `0x0A` anywhere in `String.wz/Npc.img` - and it is what `crate::jobguide`
//! uses, so one client run measures the line break for both features instead of leaving a
//! fault in one saying nothing about the other. There is a test that reads Phil's constant
//! rather than restating it.
//!
//! # The text must be ASCII, and nothing else will say so
//!
//! `net::packet::PacketWriter::str` is `s.chars().map(|c| c as u8)` - **one byte per char,
//! low byte only**. A curly apostrophe in a cab's line would reach the client as `0x19` with
//! no error anywhere. [`menu_text`] is asserted ASCII by a test.

use std::collections::HashSet;

use net::opcode::Character;
use store::Store;

use crate::config::Config;

// ---------------------------------------------------------------------------------------
// The two numbers that are policy
// ---------------------------------------------------------------------------------------

/// What a ride costs, anywhere, for anyone. **The owner's number** - *"a modest fee (500
/// mesos)"*. **[I]**, and deliberately one constant rather than a per-route table: the
/// classic game charged by distance and by cab class, and nothing in this client's data
/// carries either figure, so a distance table here would be invention wearing a
/// spreadsheet.
pub const FARE_MESOS: u32 = 500;

/// What a **ferry** crossing costs. **[I]**, and a different number from [`FARE_MESOS`] on
/// purpose: a cab crosses a town, the ferry crosses a continent, and charging the same for
/// both would make the 500 meaningless. Nothing in this client carries either figure.
pub const FERRY_FARE_MESOS: u32 = 1_000;

/// What a VIP cab charges for the run to Ant Tunnel Park.
///
/// The owner, 2026-09-09: *"VIP Cabs should transport players to Dungeon: Ant Tunnel Park for
/// 10,000 mesos."* Twenty times a regular cab, which is the point of the tier - and unlike
/// [`FARE_MESOS`] it is a number that was chosen rather than inferred, so it is stated here
/// once and read by the two rows that use it.
pub const VIP_FARE_MESOS: u32 = 10_000;

/// The line break: the **two characters** backslash and `n`.
///
/// Not a real `0x0A`. See the module doc for the listing - a token beginning with `\` and one
/// beginning with `\r` reach the same line-flush arm - and for the corroboration, which is
/// that this client's own `Quest.wz` writes 144 `Say` lines this way and `String.wz/Npc.img`
/// contains no real `0x0A` at all. `crate::jobguide` uses the same two bytes, so one client
/// run measures the line break for both features at once.
pub const LINE_BREAK: &str = "\\n";

// ---------------------------------------------------------------------------------------
// The table
// ---------------------------------------------------------------------------------------

/// **Which network of stops a row belongs to.**
///
/// [`destinations`] is derived from [`TAXIS`] rather than typed out, which is what stops a
/// hand-written route matrix from losing a row - `CLAUDE.md` records a character walking into
/// map 10 on 2026-08-19 and being unable to get out because every row leading back was
/// missing. **That derivation is exactly why this field has to exist:** without it, adding the
/// three ferry rows would silently put Orbis and El Nath on every Victoria cab's menu, and a
/// 500-meso taxi from Henesys to another continent is not a feature anybody asked for.
///
/// So a row's destinations are the *other rows in its own network*, and the two networks meet
/// only where a row is deliberately placed in both - which none is today.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    /// The six towns of Victoria Island, joined by cabs. Eight rows.
    Victoria,
    /// **The Ossyria line**: Ellinia Station, Orbis, El Nath. Three rows.
    ///
    /// This exists because **the two continents are not connected by any portal**. A
    /// breadth-first walk of `gm-handbook/portals.txt` gives a 223-map component containing
    /// Lith Harbor and an 87-map component containing Orbis and El Nath, and they do not
    /// touch. **[L]** `research/third-job.md` §4. The link in the real game is a ship, and a
    /// ship is not a portal - so without this the third-job instructors in El Nath cannot be
    /// reached by any means a player has.
    Ossyria,
    /// **The VIP cabs' one dungeon run.** The owner, 2026-09-09: *"the VIP Cabs should behave
    /// differently from normal Taxi Cabs. VIP Cabs should transport players to Dungeon: Ant
    /// Tunnel Park for 10,000 mesos."*
    ///
    /// A network of its own rather than an extra stop on [`Network::Victoria`], because
    /// [`destinations`] builds a network's menu out of the OTHER taxis' home maps - and Ant
    /// Tunnel Park has no taxi standing in it. Putting it on the Victoria list would also
    /// offer it from every regular cab at 500 mesos, which is the opposite of what was asked.
    Dungeon,
}

/// Where a [`Network::Dungeon`] cab goes.
///
/// `10005070` is **Ant Tunnel Park**, from `gm-handbook/maps.txt` - the client's own name for
/// it, not a guess at which of the eight Ant Tunnel maps was meant.
pub const DUNGEON_STOPS: [u32; 1] = [10_005_070];

/// How an NPC talks. Same mechanism, different fare, different stops - different words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Voice {
    /// A cab driver. Brisk, transactional.
    Cab,
    /// Lyn. The owner: *"they're the tour guide around Victoria Island ... their dialogue should be
    /// of one of tour guides."*
    ///
    /// Their register is not invented: `String.wz/Npc.img/900003` gives their exactly two
    /// lines, `idle0` **"Welcome to Classic World!"** and `idle1` **"Are you enjoying your
    /// adventure so far?"** - second person, welcoming, exclamatory - and they have no `d0` at
    /// all, so clicking their today prints *"This server has no dialogue for NPC template
    /// 900003 yet."* **[L]** The lines below are written to sit beside those two.
    TourGuide,
    /// The **Platform Usher** at the Orbis Ticketing Booth. Their own shipped lines are already
    /// about exactly this, which is why they were chosen and not invented:
    ///
    /// ```text
    ///  1001  idle0  "Orbis Station is huge. I'll take you to the station platform, so talk
    ///                to me."
    ///  1001  idle1  "The platform is different according to the final destination. Go
    ///                through me to use the platforms!"
    /// ```
    ///
    /// **[L]** The second line is a description of a destination menu, written by Nexon.
    Ferryman,
    /// **Eurek the Alchemist - one NPC, two continents.**
    ///
    /// They are the whole reason this line works without inventing anybody, and the client did
    /// all of it:
    ///
    /// * their own and only `d0` is *"I'm Eurek the Alchemist, and I wander all over the world
    ///   of MapleStory. I'm just stopping here for a while."* **[L]**
    /// * `Map.wz` places them **twice** - Sleepywood, 10005000, and El Nath, 20001000. They are
    ///   the only NPC in this client placed on both continents. **[L]**
    /// * they carry **zero** quest rows, unlike Jade, Fox, Scadur, Alcaster and Mr. Park, so
    ///   giving them a menu swallows nothing a player would otherwise have had.
    ///
    /// So the man who says they wander the world is the one who takes you across it, and both
    /// of their placements are used. That they are not a ticket seller is the **[I]** in this
    /// table; the alternative was an NPC nobody can reach on one side and no NPC at all on
    /// the other.
    ///
    /// **The client's real El Nath clerk is Aileen, template 1003** - ferry-seller lines,
    /// *"Please purchase a ticket from me and get on board"*, and **no placement anywhere in
    /// `Map.wz`**. **[L]** They could not be used without a way to put an NPC on a map, which
    /// this server does not have.
    Wanderer,
}

/// One taxi NPC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Taxi {
    /// The `Npc.wz` **template** id - what `0x055B`'s speaker field wants, and what
    /// `Session::on_npc_click` has already mapped the click's *object* id back to.
    pub template: u32,
    /// The name `String.wz/Npc.img` gives this template. For log lines only.
    pub name: &'static str,
    /// The map this NPC stands on. **[L]** `gm-handbook/npcs.txt`.
    ///
    /// It is also **the stop this row IS** - ride to this row and you arrive here. The two
    /// were never separate fields because they have never differed: an NPC who sells passage
    /// somewhere sells it *from where they are standing*.
    pub home_map: u32,
    pub voice: Voice,
    /// Which set of stops this row belongs to. See [`Network`].
    pub network: Network,
    /// What a ride from this row costs. [`FARE_MESOS`] for a cab, [`FERRY_FARE_MESOS`] for the
    /// ferry - carried per row rather than read from a constant, so the two cannot be confused
    /// at the one place that actually moves the money.
    pub fare: u32,
}

/// Every taxi in this client, and there are exactly eight rows.
///
/// **Provenance, per row: `gm-handbook/npcs.txt`, generated by `tools/dump_portals.py` from
/// `Map.wz`'s `life` nodes joined onto `String.wz/Npc.img`. [L]** The whole table is one
/// grep - `grep -E ", (Regular Cab|VIP Cab|Lyn)$" gm-handbook/npcs.txt` - and every template
/// below appears on exactly **one** map in that file, which is what makes `home_map` a
/// property of the NPC rather than a guess:
///
/// ```text
/// 10000000, 104,    1957, -133, 340, 1907, 2007, 0, VIP Cab
/// 10000000, 900003,  804,  316, 166,  761,  841, 0, Lyn
/// 10001000, 200,     127,  274, 143,   77,  177, 0, Regular Cab
/// 10002000, 301,     117,  -36, 963,   67,  167, 0, Regular Cab
/// 10002000, 302,     740,-2693, 641,  690,  790, 0, VIP Cab
/// 10003000, 400,    1796,  156, 201, 1746, 1846, 0, Regular Cab
/// 10004000, 500,     776, 1875, 447,  726,  826, 0, Regular Cab
/// 10005000, 600,    -282,   75,  40, -332, -232, 0, Regular Cab
/// ```
///
/// **The VIP cabs are no longer regular cabs**, and that was a decision rather than a
/// discovery. This block used to read *"a second price tier is a decision nobody has made,
/// and inventing one here would put a number on screen that no one chose"* - so it was
/// recorded and left alone. The owner made it on 2026-09-09: *"VIP Cabs should transport players
/// to Dungeon: Ant Tunnel Park for 10,000 mesos."* They are [`Network::Dungeon`] at
/// [`VIP_FARE_MESOS`] now.
///
/// **Lyn is a taxi and the Lith Harbor VIP Cab is one too**, so Lith Harbor has two. That is
/// what the data says; both work, and they differ only in voice.
pub const TAXIS: &[Taxi] = &[
    // ---- the Victoria Island cabs, unchanged -------------------------------------------
    Taxi { template: 104, name: "VIP Cab", home_map: 10_000_000, voice: Voice::Cab, network: Network::Dungeon, fare: VIP_FARE_MESOS },
    Taxi { template: 900_003, name: "Lyn", home_map: 10_000_000, voice: Voice::TourGuide, network: Network::Victoria, fare: FARE_MESOS },
    Taxi { template: 200, name: "Regular Cab", home_map: 10_001_000, voice: Voice::Cab, network: Network::Victoria, fare: FARE_MESOS },
    Taxi { template: 301, name: "Regular Cab", home_map: 10_002_000, voice: Voice::Cab, network: Network::Victoria, fare: FARE_MESOS },
    Taxi { template: 302, name: "VIP Cab", home_map: 10_002_000, voice: Voice::Cab, network: Network::Dungeon, fare: VIP_FARE_MESOS },
    Taxi { template: 400, name: "Regular Cab", home_map: 10_003_000, voice: Voice::Cab, network: Network::Victoria, fare: FARE_MESOS },
    Taxi { template: 500, name: "Regular Cab", home_map: 10_004_000, voice: Voice::Cab, network: Network::Victoria, fare: FARE_MESOS },
    Taxi { template: 600, name: "Regular Cab", home_map: 10_005_000, voice: Voice::Cab, network: Network::Victoria, fare: FARE_MESOS },

    // ---- the Ossyria line ---------------------------------------------------------------
    // Three stops, and the ONLY way a player reaches the third-job instructors: Victoria
    // Island and Orbis/El Nath are separate portal components with nothing between them, and
    // the link in the real game is a ship rather than a portal. `research/third-job.md` §4.
    //
    //   10005000, 605,  Eurek the Alchemist   Sleepywood             [L] gm-handbook/npcs.txt
    //   20000010, 1001, Platform Usher        Orbis Ticketing Booth  [L]
    //   20001000, 605,  Eurek the Alchemist   El Nath                [L]
    //
    // **Eurek is one NPC standing on two continents**, which is why `taxi_for` takes the map
    // as well as the template - and why they are the line rather than a compromise on it. See
    // `Voice::Wanderer`.
    //
    // **The obvious rows are NOT here, and the reason is reachability.** Joel (322) and Cherry
    // (323) stand at *Ellinia Station*, 10002090, and their shipped lines are exactly about a
    // ship to Orbis - but **nothing in the entire archive has a portal targeting 10002090**.
    // Ellinia's own `in03`, which is the station door in the retail game, has `tm = 0`: no
    // destination. **[L]** So a ferry on Joel would be a ferry nobody can walk to, which is
    // the same failure as a warden behind a door that does not open. The Orbis Ticketing Booth
    // is different and was checked separately: `20000000 top00 -> 20000010` is a real portal,
    // so the Platform Usher is reachable and is used.
    Taxi { template: 605, name: "Eurek the Alchemist", home_map: 10_005_000, voice: Voice::Wanderer, network: Network::Ossyria, fare: FERRY_FARE_MESOS },
    Taxi { template: 1001, name: "Platform Usher", home_map: 20_000_010, voice: Voice::Ferryman, network: Network::Ossyria, fare: FERRY_FARE_MESOS },
    Taxi { template: 605, name: "Eurek the Alchemist", home_map: 20_001_000, voice: Voice::Wanderer, network: Network::Ossyria, fare: FERRY_FARE_MESOS },
];

/// Is this NPC a taxi? `None` means "not one of mine - carry on down the click chain".
/// Is this NPC, **on this map**, a taxi? `None` means "not one of mine - carry on down the
/// click chain".
///
/// # Why the map is part of the question
///
/// It did not used to be, and for the eight cabs it makes no difference: each of those
/// templates appears on exactly one map in `gm-handbook/npcs.txt`, and `on_npc_click` resolves
/// a click against `config.npcs[chr.map_id]`, so the character is standing on `home_map` by
/// construction.
///
/// **Eurek the Alchemist is placed twice** - Sleepywood and El Nath. **[L]** Matching on the
/// template alone would make them a ferry port in Sleepywood as well, offering a ride to
/// Ellinia Station and Orbis but *not* to El Nath, because `destinations` excludes a row's own
/// `home_map` and their row's home is El Nath. That is a wrong menu on the wrong continent, and
/// the map is what forecloses it.
pub fn taxi_for(template: u32, map: u32) -> Option<&'static Taxi> {
    TAXIS.iter().find(|t| t.template == template && t.home_map == map)
}

/// Where this taxi will take you, in a fixed order. **The `#L` number is the index into
/// this vector**, so the menu and the routing cannot disagree about what "2" meant.
///
/// **Derived, not typed: a taxi's destinations are the towns that have taxis.** [`TAXIS`] is
/// the only table, so there is no second list to fall out of step with it, and the whole
/// route matrix is this one function rather than 30 hand-written rows. `CLAUDE.md` records
/// what a hand-typed destination table costs - a character walked into map 10 on 2026-08-19
/// and could not get out, because every row that would have led back was missing.
///
/// The order is **ascending map id**, which is the client's own ordering and puts the six
/// towns in the order Victoria Island numbers them. Deterministic, so a test can name a
/// selection number.
///
/// The taxi's own map is excluded: offering a ride to where the player is standing is a
/// 500-meso no-op. That also makes "never warp to the current map" structural rather than a
/// check somebody has to remember - the NPC only exists on `home_map`, and
/// `Session::on_npc_click` resolves the click against `config.npcs[chr.map_id]`, so the
/// character is on `home_map` by construction.
pub fn destinations(taxi: &Taxi) -> Vec<u32> {
    // **A dungeon cab's stops are a list, not the other cabs' home maps.** Ant Tunnel Park
    // has no taxi standing in it, so the derivation below would give it an empty menu.
    if taxi.network == Network::Dungeon {
        return DUNGEON_STOPS.iter().copied().filter(|&m| m != taxi.home_map).collect();
    }
    let mut maps: Vec<u32> = TAXIS
        .iter()
        // **Its own network, and only its own.** Without this the three Ossyria rows would
        // appear on every Victoria cab's menu - a 500-meso taxi to another continent - and
        // the ferry would offer six Victoria towns it does not sail to. See `Network`.
        .filter(|t| t.network == taxi.network)
        .map(|t| t.home_map)
        .filter(|&m| m != taxi.home_map)
        .collect();
    maps.sort_unstable();
    maps.dedup();
    maps
}

/// The destination a menu number names, or `None` for a number this taxi never offered.
///
/// **`0xFFFFFFFE` is a real thing the client sends**, on its own special path
/// (`141f739b4  cmp edi, -2`), and it is not a line the server wrote. It lands here as
/// `None` like any other out-of-range number, which is the only safe reading.
pub fn destination(taxi: &Taxi, selection: u32) -> Option<u32> {
    destinations(taxi).get(usize::try_from(selection).ok()?).copied()
}

// ---------------------------------------------------------------------------------------
// Telling a taxi's menu answer from Phil's
// ---------------------------------------------------------------------------------------

/// The `Conversation::path` that marks a live taxi menu.
///
/// # This is the discriminator, and it is load-bearing
///
/// **Phil's menu and a taxi's menu are the same packet type**, so a type-6 `0x00F3` on its
/// own says nothing about who asked. `crate::jobguide::parse_menu_reply` decodes any type-6
/// answer and cannot tell them apart either - by design; it is a decoder, not a router.
///
/// The two are separated by session state, and the separation only works because Phil
/// **clears** `self.conversation` when they send their menu while a taxi **sets** it to this
/// path. So:
///
/// * a taxi answer arrives with a taxi conversation parked -> the taxi branch claims it;
/// * a Phil answer arrives with no conversation at all -> the taxi branch declines, and
///   Phil's branch claims it.
///
/// **The taxi branch must therefore run first**, because it is the one with a precondition.
/// Phil's claims every type-6 body it is shown, so putting it first would eat taxi answers.
/// §"WIRE IT LIKE THIS" spells out the order; there is a test for both directions.
///
/// A quest path is `"0"`, `"0.yes"` or `"0.no"` and a plain talk is `""`, so nothing the
/// existing state machine produces collides with this.
pub const MENU_PATH: &str = concat!("taxi.", "menu");

/// The prefix every taxi conversation path starts with.
///
/// Kept as its own constant because `crate::jobguide`'s tests assert that Phil's prefix and
/// this one cannot collide - a good cross-module check, and one that should keep working
/// through either module's redesigns. [`MENU_PATH`] is built from it so the two cannot drift.
pub const PATH_PREFIX: &str = "taxi.";

/// Is this conversation a taxi menu waiting for an answer?
///
/// Exact rather than prefixed: `"taxi."` alone, or some future `"taxi.something-else"`, is
/// not a live menu and must not be routed as one.
pub fn is_taxi_path(path: &str) -> bool {
    path == MENU_PATH
}

// ---------------------------------------------------------------------------------------
// The words
// ---------------------------------------------------------------------------------------

/// A map's name from the client's own `String.wz`, via `gm-handbook/maps.txt`.
///
/// Falls back to the id, which is honest: a destination the name table does not know is
/// still a real map, and printing `10001000` beats printing `unnamed`.
fn map_name(config: &Config, map_id: u32) -> String {
    config.map_names.get(&map_id).cloned().unwrap_or_else(|| map_id.to_string())
}

/// What the taxi says above the list.
pub fn header(taxi: &Taxi) -> String {
    let fare = taxi.fare;
    match taxi.voice {
        Voice::Cab => format!(
            "Where to? It's #b{fare} mesos#k anywhere on the island - just say the word."
        ),
        // Their own shipped `idle0`, then the offer in their own register.
        Voice::TourGuide => format!(
            "Welcome to Classic World! I'm Lyn, and I'd love to show you around Victoria \
             Island. The tour is only #b{fare} mesos#k - where shall we start?"
        ),
        // Written to sit beside Joel's own shipped line about a ticket for the ship, and the
        // Platform Usher's about the platform differing by destination.
        Voice::Ferryman => format!(
            "Passage is #b{fare} mesos#k. Where are you sailing to?"
        ),
        // Eurek's own `d0` is *"I wander all over the world of MapleStory."* This is that
        // sentence turned into an offer, which is the whole reason they are the El Nath port.
        Voice::Wanderer => format!(
            "I wander all over the world, and I know the roads off this mountain. \
             #b{fare} mesos#k and I will see you to one of them - which?"
        ),
    }
}

/// One selectable line, in **the client's own format**: `#d#L%d# %s#l#k`.
///
/// Not invented and not re-derived - it is the literal `FUN_141e3c5d0` uses to build the
/// client's *own* NPC menu (`research/msexe-packet-fields.txt`, row `0x00F2`), and
/// [`crate::jobguide::menu_line`] uses the same one for Phil. **[L]**
pub fn menu_line(selection: u32, name: &str) -> String {
    format!("#d#L{selection}# {name}#l#k")
}

/// The whole menu string: the header, a blank line, then one [`menu_line`] per destination.
///
/// **The blank line is the client's own layout**, not a preference: every one of the 33
/// authored menus in `gm-handbook/questlines.txt` is written `<question> \n\n#L0# ...` with a
/// single break between the options after that. Quest 1013's is the canonical one. **[L]**
pub fn menu_text(taxi: &Taxi, config: &Config) -> String {
    let mut out = header(taxi);
    out.push_str(LINE_BREAK);
    for (selection, map) in destinations(taxi).into_iter().enumerate() {
        out.push_str(LINE_BREAK);
        out.push_str(&menu_line(selection as u32, &map_name(config, map)));
    }
    out
}

/// The fare could not be taken. **`have` is what the store said, not what anyone assumed.**
fn cannot_afford(taxi: &Taxi, have: u32) -> String {
    let fare = taxi.fare;
    match taxi.voice {
        Voice::Cab => format!(
            "The fare is #b{fare} mesos#k and you're carrying #b{have}#k. \
             Come back when you've got it."
        ),
        Voice::TourGuide => format!(
            "Oh dear - the tour is #b{fare} mesos#k and you only have #b{have}#k. \
             Go and see a little more of the island, and come back when you can afford it!"
        ),
        Voice::Ferryman => format!(
            "Passage is #b{fare} mesos#k and you have #b{have}#k. No ticket, no crossing."
        ),
        Voice::Wanderer => format!(
            "#b{fare} mesos#k is what it costs, and you carry #b{have}#k. \
             The mountain will still be here when you can pay."
        ),
    }
}

/// The destination is not somewhere this client can be sent. Says nothing about money,
/// because no money moved.
fn cannot_go(taxi: &Taxi, map_name: &str) -> String {
    match taxi.voice {
        Voice::Cab => format!("I can't run to {map_name} today. No charge."),
        Voice::TourGuide => format!(
            "I'm terribly sorry - the road to {map_name} is closed today. \
             I haven't taken anything from you!"
        ),
        Voice::Ferryman => {
            format!("No ship is sailing for {map_name} today. Your mesos are your own.")
        }
        Voice::Wanderer => {
            format!("The road to {map_name} is shut. I have taken nothing from you.")
        }
    }
}

/// The selection named no line this taxi offered - including the client's own `-2`.
fn no_such_stop(taxi: &Taxi, count: usize) -> String {
    match taxi.voice {
        Voice::Cab => format!("I don't run that route. There are only {count} stops on my list."),
        Voice::TourGuide => format!(
            "Goodness, I don't know that one! There are only {count} stops on the tour - \
             click me again and take another look."
        ),
        Voice::Ferryman => {
            format!("Nothing sails there. There are {count} ports on this line.")
        }
        Voice::Wanderer => format!("I know {count} roads from here, and that is not one."),
    }
}

// ---------------------------------------------------------------------------------------
// The packet out
// ---------------------------------------------------------------------------------------

/// `0x055B` message type **6**: the list box. Re-exported from its home in `net::script`.
///
/// # This used to be a second copy, and the file said so
///
/// The encoder, the decoder, `MenuReply` and this constant were all written out again here
/// because `crates/net/` was another agent's to edit at the time, and the module doc above
/// says *"the right home for both is `net::script::npc_menu`"*. They now live there, and
/// these four names are re-exports so nothing that used them had to change.
///
/// The reason to collapse it rather than leave the note: `crate::secondjob` needed a menu
/// too, and a **third** copy of a packet head is how three features drift apart one bug at a
/// time.
pub use net::script::{MenuReply, MENU_FIXED_LEN, SCRIPT_TYPE_MENU};

/// Decode a `0x00F3` that is answering a [`SCRIPT_TYPE_MENU`] box. See
/// [`net::script::parse_menu_reply`], which owns the working.
pub fn parse_menu_reply(body: &[u8]) -> Option<MenuReply> {
    net::script::parse_menu_reply(body)
}

/// The `0x055B` **body** for a menu. See [`net::script::npc_menu`], which owns the working.
pub fn menu_body(speaker_template: u32, text: &str) -> Vec<u8> {
    net::script::npc_menu(speaker_template, text)
}

// ---------------------------------------------------------------------------------------
// The step
// ---------------------------------------------------------------------------------------

/// What the session must do next. **Every arm is exhaustive and only one of them carries a
/// map id.**
///
/// That is the whole point of the type, and it comes straight out of `CLAUDE.md`'s Heena
/// section: *"Every effect hangs off the transition, not off the request."* The Heena quest
/// paid out twice per click because the payout sat outside the `match` on the store's answer.
/// Here the destination is **unreachable** unless the fare was actually taken - there is no
/// field on any other arm to read it from - so a teleport cannot be performed beside a
/// refusal by accident. A caller has to invent a map id to get it wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// Put the menu on screen and park `Conversation::path` at [`MENU_PATH`].
    Menu { text: String },
    /// **The fare has been taken and the database says so.** Send the character to `map_id`
    /// and tell them the balance moved.
    Ride { map_id: u32, map_name: String, fare: u32, balance: u32 },
    /// Say `text` on a plain OK box and drop the conversation. `why` is the log label.
    ///
    /// **A refusal is a reply.** `CLAUDE.md`'s *always answer*: this arm carries "you cannot
    /// afford it" and "I cannot take you there", and neither may be silence.
    Refused { text: String, why: String },
    /// The player closed the box. **Send nothing** and drop the conversation.
    ///
    /// Silence is correct here and it is measured, not hoped: `0x00F3` is **not** one of the
    /// 37 setters of the client's one-request-outstanding latch `player->[0x2330]`, and the
    /// dialog is destroyed and the latch released before the packet is even sent
    /// (`research/script-reply.md` §5.1, §5.2). **[L]** Answering a deliberate Close with
    /// another box the player must also close is not politeness, it is a second click.
    Cancelled,
}

/// Open the conversation: the menu.
///
/// `None` when this taxi has nowhere to go, which cannot happen with the shipped [`TAXIS`]
/// (every row has five destinations) but would if the table were ever cut to one town. A
/// menu with no lines is a dead end on screen.
pub fn opening(taxi: &Taxi, config: &Config) -> Option<Step> {
    if destinations(taxi).is_empty() {
        return None;
    }
    Some(Step::Menu { text: menu_text(taxi, config) })
}

/// The player answered a taxi's menu.
///
/// `body` is the raw `0x00F3` payload. Returns `None` when it is **not** a type-6 answer, so
/// the caller falls through to the ordinary script path rather than treating an unrelated
/// reply as a cancel. Decoding is [`crate::jobguide::parse_menu_reply`] - one decoder, shared.
pub fn on_reply(
    store: &Store,
    config: &Config,
    character_id: u32,
    taxi: &Taxi,
    body: &[u8],
) -> Option<Step> {
    let reply = parse_menu_reply(body)?;
    Some(route(store, config, character_id, taxi, &reply))
}

/// The decision, given an already-decoded answer. Split out so a test can drive it without
/// hand-building bodies, and so [`on_reply`] is only the decode plus this.
pub fn route(
    store: &Store,
    config: &Config,
    character_id: u32,
    taxi: &Taxi,
    reply: &MenuReply,
) -> Step {
    let Some(selection) = reply.selection else {
        return Step::Cancelled;
    };
    let dests = destinations(taxi);
    let Some(map_id) = destination(taxi, selection) else {
        return Step::Refused {
            text: no_such_stop(taxi, dests.len()),
            why: format!(
                "taxi {} ({}): selection {selection} names no stop; this taxi offers {} (a -2 is the client's own special path, 141f739b4). NO FARE TAKEN",
                taxi.template,
                taxi.name,
                dests.len()
            ),
        };
    };
    board(store, config, character_id, taxi, map_id)
}

/// Take the fare, or say why not. **This is the transition, and it is the only thing that
/// can produce a [`Step::Ride`].**
///
/// The order of the three gates is the order they must run in:
///
/// 1. **The field table.** `Config::map_exists` is fail-open on an empty set, so with no
///    `gm-handbook/fields.txt` every id would pass. `Session::gm_map` refuses out loud in
///    exactly this case rather than failing open, after the owner typed `!map 45` on 2026-08-20
///    and lost a session to a map with no field image. This must not be a weaker guard.
/// 2. **The map itself.** A destination with no field image kills the client.
/// 3. **The money**, last - because charging for a ride we are about to refuse is the one
///    direction of this that takes something and gives nothing.
///
/// `Store::add_mesos` reads and writes in **one transaction** and refuses to go below zero,
/// so the balance cannot be raced by two clicks. The refusal comes back as
/// `StoreError::NotEnoughMesos { have, want }` and is turned into a sentence rather than a
/// log line - `CLAUDE.md`: *"A refusal that is reported to no one will be ignored
/// eventually."*
fn board(store: &Store, config: &Config, character_id: u32, taxi: &Taxi, map_id: u32) -> Step {
    let map_name = map_name(config, map_id);
    if config.fields.is_empty() {
        return Step::Refused {
            text: cannot_go(taxi, &map_name),
            why: format!(
                "taxi {} REFUSED a ride to {map_id}: the field table is empty, so the map could not be checked and a bad id would kill the client. Regenerate gm-handbook/fields.txt with tools/dump_portals.py, or restart the server so it loads. NO FARE TAKEN",
                taxi.template
            ),
        };
    }
    if !config.map_exists(map_id) {
        return Step::Refused {
            text: cannot_go(taxi, &map_name),
            why: format!(
                "taxi {} REFUSED a ride to {map_id}: no field image in this client, so it would strand the character. NO FARE TAKEN",
                taxi.template
            ),
        };
    }
    match store.add_mesos(character_id, -i64::from(taxi.fare)) {
        Ok(balance) => Step::Ride { map_id, map_name, fare: taxi.fare, balance },
        Err(store::StoreError::NotEnoughMesos { have, .. }) => Step::Refused {
            text: cannot_afford(taxi, have),
            why: format!(
                "taxi {} REFUSED a ride to {map_id} ({map_name}): character {character_id} has {have} mesos and the fare is {}. NO FARE TAKEN, NO TELEPORT",
                taxi.template, taxi.fare
            ),
        },
        Err(e) => Step::Refused {
            text: cannot_go(taxi, &map_name),
            why: format!(
                "taxi {} could not charge character {character_id} for a ride to {map_id}: {e}. NO FARE TAKEN, NO TELEPORT",
                taxi.template
            ),
        },
    }
}

/// The `0x055B` a step puts on screen.
///
/// # `Step::Ride` and `Step::Cancelled` deliberately produce NOTHING
///
/// **A script message must never travel with, or just before, a `SetField`.** Field entry
/// runs `FUN_142caa4e0`, which calls the script-manager reset `FUN_141f6f200` at
/// `142caac7a`: the dialog is built and then torn down, silently, with nothing in any log.
/// `CLAUDE.md` says this has already cost a run. So a paid ride confirms itself with the
/// meso change and the new map, and says nothing. A cancel is the player's own Close.
pub fn script_replies(taxi: &Taxi, step: &Step) -> Vec<crate::Reply> {
    match step {
        Step::Menu { text } => vec![crate::Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: menu_body(taxi.template, text),
            what: format!(
                "ScriptMessage MENU (type 6) from taxi {} ({}), {} bytes: {text:?}",
                taxi.template,
                taxi.name,
                MENU_FIXED_LEN + text.len()
            ),
        }],
        Step::Refused { text, why } => vec![crate::Reply {
            opcode: net::script::SCRIPT_MESSAGE,
            body: net::script::npc_say(taxi.template, text, false, false),
            what: format!("ScriptMessage Say from taxi {} ({}): {why}", taxi.template, taxi.name),
        }],
        Step::Ride { .. } | Step::Cancelled => Vec::new(),
    }
}

/// A convenience for the caller's log line: what a ride out of this taxi is worth saying.
pub fn ride_note(taxi: &Taxi, chr: &Character, map_id: u32, map_name: &str, balance: u32) -> String {
    format!(
        "taxi {} ({}) took {} mesos from {} and is sending them to map {map_id} ({map_name}); {balance} left",
        taxi.template, taxi.name, taxi.fare, chr.name
    )
}

/// The set of every map a taxi can send anyone to, for a start-up sanity line if the
/// coordinator wants one. Cheap, and it is the whole blast radius of this feature.
pub fn all_destinations() -> HashSet<u32> {
    TAXIS.iter().map(|t| t.home_map).collect()
}

// ---------------------------------------------------------------------------------------
//
// # IF THE LIST DOES NOT RENDER
//
// Nobody has watched a server-sent type 6 draw, and the Phil agent's named blind spot (quoted
// in the module doc) is the honest reason why. **`world.log` settles it without anyone having
// to describe the screen**, because the three outcomes have three different byte counts:
//
//   * a **10-byte** `0x00F3` - `xxxxxxxx 06 01 <u32>` - the list rendered, a line was clicked,
//     and that `u32` is the `#L` number. The feature works and the ride follows.
//   * a **6-byte** `0x00F3` - `xxxxxxxx 06 00` - the box came up and the client answered as
//     type 6, so the packet and the dialog kind are right and only the **markup** is wrong.
//   * **no `0x00F3` at all** - the box never appeared. Then it is the type or the body, not
//     the markup.
//
// The last two both fall back the same way, and it is a small change rather than a redesign,
// because everything except the box is shared: the table, `destinations`, `board` and all
// three guards, the fare, and every sentence. Only [`opening`] and [`script_replies`] know
// what shape the question is. Replace `Step::Menu` with the yes/no chain this module carried
// before - one `net::script::npc_ask(taxi.template, text, false)` per destination, Yes rides
// and No advances, `Conversation::path` walking `taxi.0`..`taxi.4` instead of [`MENU_PATH`] -
// and `route` keeps working unchanged, because a selection index is a selection index however
// it was collected.
//
// ---------------------------------------------------------------------------------------
//
// # WIRE IT LIKE THIS
//
// `crates/world/src/session/npc.rs` belongs to the coordinator. The exact patch is in the
// hand-back report; in short:
//
// * `on_npc_click`, in the branch chain beside the shop / storage / instructor checks:
//   `open_taxi_for(template)`, which parks a `Conversation` at `MENU_PATH`.
//
// * `on_script_reply`, **before `parse_script_reply` is called at all** - that function
//   cannot decode a 10-byte type-6 body and drops it silently - and **before Phil's menu
//   branch**, because the taxi branch has a precondition (a parked taxi conversation) and
//   Phil's claims every type-6 body it is shown. See [`MENU_PATH`] for the full argument and
//   `phils_menu_answer_is_not_eaten_by_the_taxi_branch` for the test.
//
// * **`net::script::npc_menu(speaker, text)`**: [`menu_body`] here and
//   `crate::jobguide::menu_body` are the same eight writes. They belong in `crates/net`, and
//   neither agent could put them there. Collapse them.
//
// ---------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Every town this client puts a taxi in, so a test can build a `Config` whose field
    /// table is not empty without depending on `gm-handbook/`, which is gitignored.
    const TOWNS: [u32; 6] =
        [10_000_000, 10_001_000, 10_002_000, 10_003_000, 10_004_000, 10_005_000];

    fn config() -> Config {
        let mut c = Config::default();
        c.fields = TOWNS.into_iter().collect();
        for (id, name) in [
            (10_000_000u32, "Lith Harbor"),
            (10_001_000, "Henesys"),
            (10_002_000, "Ellinia"),
            (10_003_000, "Kerning City"),
            (10_004_000, "Perion"),
            (10_005_000, "Sleepywood"),
        ] {
            c.map_names.insert(id, name.to_string());
        }
        c
    }

    /// A store with one character carrying `mesos`.
    fn store_with(mesos: u32) -> (Store, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = Character { name: "Wanderer".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.set_mesos(id, mesos).unwrap();
        (store, id)
    }

    fn lyn() -> &'static Taxi {
        taxi_for(900_003, 10_000_000).expect("Lyn is a taxi")
    }

    fn henesys_cab() -> &'static Taxi {
        taxi_for(200, 10_001_000).expect("Henesys has a Regular Cab")
    }

    /// A real type-6 `0x00F3`, built the way the client builds it at `141f7398e`/`141f73995`.
    fn menu_answer(selection: u32) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0u32.to_le_bytes()); // handle, a literal 0
        b.push(SCRIPT_TYPE_MENU); //                the echoed message type
        b.push(1); //                               accepted
        b.extend_from_slice(&selection.to_le_bytes());
        b
    }

    /// The six-byte cancel, `141f7397c`.
    fn menu_cancel() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(&0u32.to_le_bytes());
        b.push(SCRIPT_TYPE_MENU);
        b.push(0);
        b
    }

    // -- the table -----------------------------------------------------------------------

    /// The table itself, against the file it came from. **Eleven rows** - eight cabs on the
    /// six Victoria towns, and three ports on the Ossyria line.
    #[test]
    fn the_table_matches_the_npc_dump_it_was_read_from() {
        assert_eq!(TAXIS.len(), 11, "7 cabs, Lyn, and 3 ferry ports - grep gm-handbook/npcs.txt");
        // **A template may now appear twice, but only on different maps.** Eurek does not,
        // today - their Sleepywood placement is deliberately not a row - but the uniqueness
        // that has to hold is `(template, map)`, because that is what `taxi_for` matches on.
        for t in TAXIS {
            assert_eq!(
                TAXIS
                    .iter()
                    .filter(|o| o.template == t.template && o.home_map == t.home_map)
                    .count(),
                1,
                "template {} on map {} appears twice in the table",
                t.template,
                t.home_map
            );
        }

        let victoria: Vec<&Taxi> =
            TAXIS.iter().filter(|t| t.network == Network::Victoria).collect();
        // Six, not eight: the two VIP Cabs moved to `Network::Dungeon` on 2026-09-09.
        assert_eq!(victoria.len(), 6, "the regular cabs and Lyn");
        let mut towns: Vec<u32> = victoria.iter().map(|t| t.home_map).collect();
        towns.sort_unstable();
        towns.dedup();
        // **Still all six towns, and that is the check that matters.** Lith Harbor keeps Lyn
        // and Ellinia keeps Regular Cab 301, so moving the VIP rows out cost no town its
        // service. If it ever does, a town silently loses its taxi and this catches it.
        assert_eq!(towns, TOWNS, "the six Victoria Island towns");
        for t in &victoria {
            assert_eq!(t.fare, FARE_MESOS, "a cab charges the cab fare");
        }

        // **The VIP cabs.** The owner: *"VIP Cabs should transport players to Dungeon: Ant Tunnel
        // Park for 10,000 mesos."*
        let vip: Vec<&Taxi> = TAXIS.iter().filter(|t| t.network == Network::Dungeon).collect();
        assert_eq!(vip.len(), 2, "Lith Harbor and Ellinia have the VIP cabs");
        for t in &vip {
            assert_eq!(t.name, "VIP Cab");
            assert_eq!(t.fare, VIP_FARE_MESOS, "twenty times a regular cab");
            assert_eq!(destinations(t), vec![10_005_070], "Ant Tunnel Park, and only that");
        }
        // And no regular cab reaches it, or the tier is pointless.
        for t in TAXIS.iter().filter(|t| t.network != Network::Dungeon) {
            assert!(!destinations(t).contains(&10_005_070), "row {} offers the dungeon", t.template);
        }

        let ferry: Vec<&Taxi> = TAXIS.iter().filter(|t| t.network == Network::Ossyria).collect();
        assert_eq!(ferry.len(), 3, "Sleepywood, Orbis, El Nath");
        assert_eq!(
            ferry.iter().map(|t| t.home_map).collect::<Vec<_>>(),
            vec![10_005_000, 20_000_010, 20_001_000]
        );
        assert_eq!(ferry.iter().map(|t| t.template).collect::<Vec<_>>(), vec![605, 1001, 605]);
        // **Sleepywood is a stop on BOTH networks, and that is the interchange.** A player
        // cabs to Sleepywood from any Victoria town for the cab fare and crosses from there.
        // Asserted rather than left implicit, because it is the only map where the two
        // networks touch and it is what makes the ferry reachable at all.
        assert_eq!(
            TAXIS.iter().filter(|t| t.home_map == 10_005_000).count(),
            2,
            "Sleepywood has the cab and the ferry"
        );
        assert_eq!(
            TAXIS
                .iter()
                .filter(|t| t.home_map == 10_005_000)
                .map(|t| t.network)
                .collect::<Vec<_>>(),
            vec![Network::Victoria, Network::Ossyria]
        );
        for t in &ferry {
            assert_eq!(t.fare, FERRY_FARE_MESOS, "a crossing costs more than a cab ride");
            assert_ne!(t.fare, FARE_MESOS, "and the two prices must not be the same number");
        }

        assert_eq!(lyn().voice, Voice::TourGuide);
        assert_eq!(henesys_cab().voice, Voice::Cab);
        assert!(taxi_for(1, 10_000_000).is_none(), "Heena is not a taxi");
        // And the map half of the question, which is what Eurek needs: they are a ferry
        // port in El Nath and an ordinary NPC in Sleepywood, on the same template.
        // **Eurek is a port on both of their placements, and nowhere else.** The map argument
        // is what makes that expressible: one template, two rows, two different networks'
        // worth of neighbours.
        assert!(taxi_for(605, 20_001_000).is_some(), "Eurek is a port in El Nath");
        assert!(taxi_for(605, 10_005_000).is_some(), "and in Sleepywood - they cross");
        assert!(taxi_for(605, 10_000_000).is_none(), "but they do not stand in Lith Harbor");
        assert_eq!(all_destinations().len(), 8, "six towns plus Orbis and El Nath");
    }

    /// **Every row stands where the table says, read from the dump rather than from memory.**
    ///
    /// The test above is named *"against the file it came from"* and checks only that the
    /// table agrees with itself. This one opens the file. It exists because the three new rows
    /// were chosen by reading `String.wz` strings, and a template id copied wrong would put a
    /// ferry on an NPC who is not there - which on screen is a click that does nothing, the
    /// hardest failure in this project to tell from a click that was never wired.
    ///
    /// `gm-handbook/` is generated and gitignored, so this degrades to a no-op on a clean
    /// checkout. It therefore asserts a positive control first.
    #[test]
    fn every_row_stands_where_the_table_says() {
        let npcs = std::path::Path::new("../../gm-handbook/npcs.txt");
        let strings = std::path::Path::new("../../gm-handbook/npcstrings.txt");
        if !npcs.exists() || !strings.exists() {
            return;
        }
        let text = std::fs::read_to_string(npcs).expect("npcs.txt");
        let rows: Vec<(u32, u32)> = text
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .filter_map(|l| {
                let f: Vec<&str> = l.split(',').map(str::trim).collect();
                Some((f.first()?.parse().ok()?, f.get(1)?.parse().ok()?))
            })
            .collect();
        assert!(rows.len() > 250, "positive control: {} placements loaded", rows.len());

        let names = std::fs::read_to_string(strings).expect("npcstrings.txt");
        let name_of = |t: u32| -> Option<String> {
            names.lines().find_map(|l| {
                let f: Vec<&str> = l.split('\t').collect();
                (f.len() == 3 && f[0].parse::<u32>().ok()? == t && f[1] == "name")
                    .then(|| f[2].to_string())
            })
        };
        assert_eq!(name_of(322).as_deref(), Some("Joel"), "positive control on the name join");

        for t in TAXIS {
            assert!(
                rows.contains(&(t.home_map, t.template)),
                "{} (template {}) is not placed on map {} in the client",
                t.name,
                t.template,
                t.home_map
            );
            assert_eq!(
                name_of(t.template).as_deref(),
                Some(t.name),
                "template {} is not called {:?} in String.wz",
                t.template,
                t.name
            );
        }

        // **Eurek is placed twice and only one of them is a port.** That is the fact
        // `taxi_for`'s map argument exists for, so it is asserted against the file rather
        // than trusted: if a later dump moves them, this fails here.
        let eurek: Vec<u32> =
            rows.iter().filter(|(_, t)| *t == 605).map(|(m, _)| *m).collect();
        assert_eq!(eurek.len(), 2, "Eurek stands in two towns, got {eurek:?}");
        assert!(eurek.contains(&20_001_000) && eurek.contains(&10_005_000));
        assert!(taxi_for(605, 10_005_000).is_some(), "and BOTH of their placements are ports");

        // **The ferry NPCs the client actually shipped, and why two of them are unused.**
        // Joel (322) sells tickets to Orbis in their own words - and stands at Ellinia Station,
        // 10002090, which NOTHING in the archive has a portal to. Aileen (1003) sells them too
        // and is placed nowhere at all. Both facts are asserted here so that a later dump
        // which fixes either one fails this test and invites the row back in.
        // Reachability is a question about PORTALS, not about placements - Joel is standing
        // there perfectly well, which is exactly what makes the trap a trap.
        let portals = std::path::Path::new("../../gm-handbook/portals.txt");
        if portals.exists() {
            let ptext = std::fs::read_to_string(portals).expect("portals.txt");
            let targets: Vec<u32> = ptext
                .lines()
                .filter(|l| !l.starts_with('#'))
                .filter_map(|l| l.split(',').nth(3)?.trim().parse().ok())
                .collect();
            assert!(targets.len() > 1000, "positive control: {} portal targets", targets.len());
            assert!(
                targets.contains(&20_000_010),
                "positive control: something DOES lead to the Orbis Ticketing Booth"
            );
            assert!(
                !targets.contains(&10_002_090),
                "nothing leads to Ellinia Station - if that ever changes, put Joel in TAXIS"
            );
        }
        assert!(
            !rows.iter().any(|(_, t)| *t == 1003),
            "if Aileen is ever placed, they are the El Nath port and Eurek can stand down"
        );
        assert_eq!(name_of(1003).as_deref(), Some("Aileen"), "they are in String.wz regardless");
        for t in [322u32, 1001] {
            assert_eq!(
                rows.iter().filter(|(_, tpl)| *tpl == t).count(),
                1,
                "template {t} is placed once"
            );
        }
    }

    /// **A row offers the other stops in its own network, and never the other network's.**
    ///
    /// The second half is the one worth having. `destinations` is derived from `TAXIS`, so
    /// adding three rows for another continent would - without `Network` - have put Orbis and
    /// El Nath on every Victoria cab's menu at 500 mesos. This asserts the containment in both
    /// directions rather than just counting.
    #[test]
    fn destinations_are_the_other_stops_in_the_same_network() {
        for t in TAXIS {
            let d = destinations(t);
            let want = match t.network {
                Network::Victoria => 5, // six towns minus its own
                Network::Ossyria => 2,  // three ports minus its own
                Network::Dungeon => DUNGEON_STOPS.len(), // Ant Tunnel Park, and no cab is in it
            };
            assert_eq!(d.len(), want, "row {} on map {}", t.template, t.home_map);
            assert!(!d.contains(&t.home_map), "row {} offers its own map", t.template);
            let mut sorted = d.clone();
            sorted.sort_unstable();
            assert_eq!(d, sorted, "the order is ascending map id");
            // **Every destination is served by a row in this row's own network.** Written
            // as "there exists such a row" rather than "the row on that map is in this
            // network", because Sleepywood hosts one of each - it is the interchange - and
            // the stronger phrasing would fail on the very map the design depends on.
            // **A dungeon stop is deliberately NOT a taxi's home map** - that is the whole
            // reason it needed its own network - so the containment below is asked only of
            // the two networks that are built out of `TAXIS`.
            if t.network != Network::Dungeon {
                for m in &d {
                    assert!(
                        TAXIS.iter().any(|o| o.home_map == *m && o.network == t.network),
                        "row {} offers map {m}, which its own network does not serve",
                        t.template
                    );
                }
            }
        }
        // And the containment, stated as the thing that would actually be a bug: no cab ever
        // names a map that only the ferry reaches.
        let ossyria_only = [20_000_010u32, 20_001_000];
        for t in TAXIS.iter().filter(|t| t.network == Network::Victoria) {
            for m in ossyria_only {
                assert!(
                    !destinations(t).contains(&m),
                    "cab {} offers {m}, which is another continent",
                    t.template
                );
            }
        }
        // Nor does the ferry sell a hop between two Victoria towns - that is the cabs' job.
        for t in TAXIS.iter().filter(|t| t.network == Network::Ossyria) {
            for m in [10_000_000u32, 10_001_000, 10_002_000, 10_003_000, 10_004_000] {
                assert!(!destinations(t).contains(&m), "the ferry does not run to {m}");
            }
        }
        assert_eq!(
            destinations(lyn()),
            vec![10_001_000, 10_002_000, 10_003_000, 10_004_000, 10_005_000],
            "the cabs are exactly as they were"
        );
        // From Sleepywood, Eurek sails to Orbis and El Nath and to no town on their own island.
        let out = taxi_for(605, 10_005_000).expect("Eurek is the Sleepywood port");
        assert_eq!(destinations(out), vec![20_000_010, 20_001_000]);
        // **And from El Nath they can get a player home**, which is the whole reason their second
        // placement is in the table. A line that only ran one way would leave a level-70
        // character with seventeen floors of the Orbis Tower as their only route back.
        let home = taxi_for(605, 20_001_000).expect("Eurek is the El Nath port");
        assert_eq!(destinations(home), vec![10_005_000, 20_000_010]);
        assert!(destinations(home).contains(&10_005_000), "there IS a way back to Victoria");
    }

    // -- the menu packet -----------------------------------------------------------------

    /// **The `#L` number a line advertises is the number that selects it.** A menu whose
    /// markup and whose routing disagree sends people to the wrong town and looks like a
    /// working feature while it does it.
    #[test]
    fn every_menu_line_number_round_trips_to_the_destination_it_names() {
        let cfg = config();
        for t in TAXIS {
            let text = menu_text(t, &cfg);
            for (i, map) in destinations(t).into_iter().enumerate() {
                let sel = i as u32;
                let name = map_name(&cfg, map);
                assert!(
                    text.contains(&menu_line(sel, &name)),
                    "taxi {} has no line #L{sel}# for {name}: {text:?}",
                    t.template
                );
                assert_eq!(
                    destination(t, sel),
                    Some(map),
                    "taxi {} selection {sel} must route to the map its own line names",
                    t.template
                );
            }
            assert_eq!(destination(t, 5), None, "there is no sixth stop");
            assert_eq!(destination(t, 0xFFFF_FFFE), None, "the client's own -2 is not a stop");
        }
    }

    /// **The box goes out as message type 6, at the offset the client reads it from.** Type
    /// 3 would draw a yes/no pair and answer with no selection at all; the whole feature
    /// turns on this byte.
    #[test]
    fn the_menu_goes_out_as_type_six_with_the_speakers_own_template() {
        let cfg = config();
        let Some(step) = opening(lyn(), &cfg) else { panic!("Lyn opens with a menu") };
        let out = script_replies(lyn(), &step);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].opcode, net::script::SCRIPT_MESSAGE);
        let b = &out[0].body;
        assert_eq!(b[9], 0, "hasOverride 0 - no u32 follows, or every field after shifts by 4");
        assert_eq!(b[10], SCRIPT_TYPE_MENU, "141f6f3e6 message type");
        assert_eq!(b[10], 6);
        assert_eq!(&b[5..9], &900_003u32.to_le_bytes(), "141f6f398 - Lyn's own portrait");
        assert_eq!(&b[11..13], &0u16.to_le_bytes(), "flags 0: no body speaker, Close kept");
        let text = menu_text(lyn(), &cfg);
        assert_eq!(&b[14..16], &(text.len() as u16).to_le_bytes(), "u16 BYTE count");
        assert_eq!(b.len(), MENU_FIXED_LEN + text.len(), "two reads and no others");
    }

    /// **The decoder, against the two bodies the client actually builds.**
    ///
    /// Both shapes are read off `141f7397c` (cancel) and `141f7398e`/`141f73995` (accept).
    /// The truncation sweep matters because these come off a socket: a short body must be
    /// refused, and in particular a **truncated accept must not decode as a cancel** - that
    /// would silently turn a malformed packet into "the player pressed Close", which is one
    /// step from a free ride.
    #[test]
    fn the_menu_decoder_reads_both_bodies_the_client_builds() {
        assert_eq!(SCRIPT_TYPE_MENU, 6, "141f73961  mov dl, 6");

        let accept = menu_answer(3);
        assert_eq!(accept.len(), 10, "u32 handle, u8 type, u8 1, u32 selection");
        assert_eq!(parse_menu_reply(&accept), Some(MenuReply { selection: Some(3) }));

        let cancel = menu_cancel();
        assert_eq!(cancel.len(), 6, "u32 handle, u8 type, u8 0");
        assert_eq!(parse_menu_reply(&cancel), Some(MenuReply { selection: None }));

        // Not a type-6 answer: the caller must fall through, not treat it as a cancel.
        let yes_no = [0x00, 0x00, 0x00, 0x00, 0x10, 0x01];
        assert_eq!(parse_menu_reply(&yes_no), None, "a quest yes/no is not ours");
        let say = [0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        assert_eq!(parse_menu_reply(&say), None, "a Say is not ours");

        // Every truncation of the accept is refused rather than mis-read.
        for n in 0..accept.len() {
            let got = parse_menu_reply(&accept[..n]);
            assert_ne!(
                got,
                Some(MenuReply { selection: None }),
                "a {n}-byte accept must not decode as a cancel"
            );
            if n < 6 {
                assert_eq!(got, None, "len {n}");
            }
        }

        // And the packet this module SENDS round-trips with the decoder it also owns: the
        // `#L` number in the text is the number that comes back.
        let cfg = config();
        let text = menu_text(lyn(), &cfg);
        let body = menu_body(lyn().template, &text);
        assert_eq!(body[10], SCRIPT_TYPE_MENU);
        for sel in 0..destinations(lyn()).len() as u32 {
            assert!(text.contains(&format!("#L{sel}#")));
            assert_eq!(parse_menu_reply(&menu_answer(sel)).unwrap().selection, Some(sel));
        }
    }

    /// **The text is ASCII.** `PacketWriter::str` is `chars().map(|c| c as u8)` - one byte
    /// per char, low byte only - so a curly apostrophe would reach the client as `0x19` with
    /// no error anywhere. This is the "unit, not the arithmetic" failure in text form.
    #[test]
    fn every_line_this_module_can_say_is_ascii() {
        let cfg = config();
        for t in TAXIS {
            let mut lines = vec![menu_text(t, &cfg), header(t)];
            lines.push(cannot_afford(t, 120));
            lines.push(cannot_go(t, "Henesys"));
            lines.push(no_such_stop(t, 5));
            for l in lines {
                assert!(l.is_ascii(), "taxi {} says non-ASCII: {l:?}", t.template);
            }
        }
        // **The line break is the one Phil uses, and that is checked against their text, not
        // restated here.** Both menus land on the owner's screen in the same run; if they broke
        // lines differently, a rendering fault in one would say nothing about the other.
        assert_eq!(LINE_BREAK.as_bytes(), b"\\n", "two characters, not a real 0x0A");
        assert_eq!(LINE_BREAK.len(), 2);
        assert!(menu_text(lyn(), &cfg).contains(LINE_BREAK));
        assert!(!menu_text(lyn(), &cfg).contains('\n'), "a real newline is not what this reads");
        // **Read Phil's constant rather than restating it.** Both menus land on the owner's screen
        // in the same run; if they broke lines differently, a rendering fault in one would say
        // nothing about the other, and this test is what notices if that changes.
        assert_eq!(
            LINE_BREAK,
            crate::jobguide::LINE_BREAK,
            "Phil breaks lines differently now, so one client run can no longer measure both"
        );
    }

    // -- the ride ------------------------------------------------------------------------

    /// **The four effects of a ride, counted.** `CLAUDE.md`: a test that checks one of
    /// several effects gives false confidence about the rest, and the Heena turn-in test
    /// passed for a week while the experience doubled beside the fanfare it was counting.
    ///
    /// A ride is: (1) the fare leaves the database, (2) exactly once, (3) the destination
    /// the selection named comes back, and (4) the balance reported is the one the store
    /// now holds.
    #[test]
    fn a_ride_charges_the_fare_once_and_names_the_selected_destination() {
        let (store, id) = store_with(1_200);
        let cfg = config();

        let step = on_reply(&store, &cfg, id, lyn(), &menu_answer(0)).expect("a type-6 answer");
        let Step::Ride { map_id, ref map_name, fare, balance } = step else {
            panic!("selection 0 on a solvent character rides: {step:?}");
        };
        assert_eq!(map_id, 10_001_000, "#L0# out of Lith Harbor is Henesys");
        assert_eq!(map_name, "Henesys");
        assert_eq!(fare, FARE_MESOS);
        assert_eq!(balance, 700, "1200 - 500");
        assert_eq!(store.mesos(id).unwrap(), 700, "the database agrees with the packet");

        // A different line goes to a different town, so the selection is really being read
        // rather than a constant being returned.
        let step = on_reply(&store, &cfg, id, lyn(), &menu_answer(3)).expect("a type-6 answer");
        let Step::Ride { map_id, balance, .. } = step else { panic!("second ride: {step:?}") };
        assert_eq!(map_id, 10_004_000, "#L3# is Perion");
        assert_eq!(balance, 200, "twice is twice - a ride is a charge, not a latch");
        assert_eq!(store.mesos(id).unwrap(), 200);
    }

    /// **Not enough mesos: answered, and nothing else happens.** Three ways this could go
    /// wrong - silence (which reads on screen as a crash), a teleport that was never paid
    /// for, and a partial charge.
    #[test]
    fn a_player_who_cannot_pay_is_told_so_and_is_not_moved_or_charged() {
        let (store, id) = store_with(499);
        let cfg = config();

        let step =
            on_reply(&store, &cfg, id, henesys_cab(), &menu_answer(0)).expect("a type-6 answer");
        let Step::Refused { ref text, ref why } = step else {
            panic!("a broke player gets a refusal, not {step:?}")
        };
        assert!(!text.is_empty(), "a refusal is a REPLY - an empty box is silence with a frame");
        assert!(text.contains("499"), "the sentence says what they actually have: {text:?}");
        assert!(text.contains(&FARE_MESOS.to_string()), "and what it costs: {text:?}");
        assert!(why.contains("NO FARE TAKEN") && why.contains("NO TELEPORT"), "{why}");
        assert_eq!(store.mesos(id).unwrap(), 499, "the refused spend changed nothing");
        // The refusal is a real Say on the wire, not a dropped packet.
        assert_eq!(script_replies(henesys_cab(), &step).len(), 1);
        // And no arm but Ride carries a map, so a caller cannot warp off this even by mistake.
        assert!(!matches!(step, Step::Ride { .. }));
    }

    /// Exactly at the fare: a ride, and the balance lands on zero rather than refusing.
    /// An off-by-one on a `>=` here would make 500 mesos not enough to spend 500 mesos.
    #[test]
    fn exactly_the_fare_is_enough() {
        let (store, id) = store_with(FARE_MESOS);
        let step = on_reply(&store, &config(), id, henesys_cab(), &menu_answer(0)).unwrap();
        assert!(matches!(step, Step::Ride { balance: 0, .. }), "{step:?}");
        assert_eq!(store.mesos(id).unwrap(), 0);
    }

    /// **A map with no field image is refused before the money moves**, and the guard is not
    /// weaker than `Session::gm_map`'s: an empty field table is refused too rather than
    /// failing open. The owner lost a session to `!map 45` on 2026-08-20.
    #[test]
    fn a_destination_the_client_cannot_load_is_refused_and_costs_nothing() {
        let (store, id) = store_with(5_000);

        // The field table knows every town except Ellinia, which is Lyn's #L1#.
        let mut cfg = config();
        cfg.fields.remove(&10_002_000);
        let step = on_reply(&store, &cfg, id, lyn(), &menu_answer(1)).unwrap();
        let Step::Refused { ref why, .. } = step else { panic!("must refuse: {step:?}") };
        assert!(why.contains("no field image"), "{why}");
        assert!(why.contains("NO FARE TAKEN"), "{why}");
        assert_eq!(store.mesos(id).unwrap(), 5_000);

        // And with no table at all, everything is refused rather than waved through -
        // `Config::map_exists` is fail-open, so this case needs its own guard.
        let mut blind = config();
        blind.fields.clear();
        let step = on_reply(&store, &blind, id, lyn(), &menu_answer(0)).unwrap();
        let Step::Refused { ref why, .. } = step else { panic!("must refuse: {step:?}") };
        assert!(why.contains("field table is empty"), "{why}");
        assert_eq!(store.mesos(id).unwrap(), 5_000, "still nothing taken");
    }

    /// **A selection off the end is refused, not charged - and `-2` is a real one.** The
    /// client sends `0xFFFFFFFE` on its own special path (`141f739b4  cmp edi, -2`), which is
    /// not a line the server wrote, and `usize::try_from` alone would not stop a 32-bit
    /// number on a 64-bit build.
    #[test]
    fn a_selection_the_taxi_never_offered_is_refused_without_charging() {
        let (store, id) = store_with(9_999);
        let cfg = config();
        for sel in [5u32, 99, 0xFFFF_FFFE, 0xFFFF_FFFF] {
            let step = on_reply(&store, &cfg, id, lyn(), &menu_answer(sel)).unwrap();
            let Step::Refused { ref text, .. } = step else { panic!("sel {sel}: {step:?}") };
            assert!(!text.is_empty(), "sel {sel} gets words");
        }
        assert_eq!(store.mesos(id).unwrap(), 9_999, "not one meso for four bad selections");
    }

    /// **A cancel sends nothing and costs nothing.** Answering a deliberate Close with
    /// another box the player must also close is a second click, and `0x00F3` needs no
    /// answer - it is not one of the 37 latch setters.
    #[test]
    fn closing_the_menu_sends_nothing_and_charges_nothing() {
        let (store, id) = store_with(1_000);
        let step = on_reply(&store, &config(), id, lyn(), &menu_cancel()).unwrap();
        assert_eq!(step, Step::Cancelled);
        assert!(script_replies(lyn(), &step).is_empty());
        assert_eq!(store.mesos(id).unwrap(), 1_000);
    }

    /// **A paid ride sends no script**, because field entry runs the script-manager reset and
    /// destroys the dialog silently. This project has already lost a run to that.
    #[test]
    fn a_paid_ride_sends_no_script_message() {
        let (store, id) = store_with(1_000);
        let step = on_reply(&store, &config(), id, lyn(), &menu_answer(0)).unwrap();
        assert!(matches!(step, Step::Ride { .. }), "{step:?}");
        assert!(
            script_replies(lyn(), &step).is_empty(),
            "a script sent with a SetField is torn down by FUN_141f6f200 with nothing in any log"
        );
    }

    // -- living beside Phil ---------------------------------------------------------------

    /// **Phil's menu answer and a taxi's are the same packet, and must not eat each other.**
    ///
    /// The only thing that separates them is session state: Phil clears `Conversation`, a
    /// taxi parks one at [`MENU_PATH`]. This pins both halves of that contract - the taxi
    /// path is recognisable, and no quest or plain-talk path collides with it.
    #[test]
    fn phils_menu_answer_is_not_eaten_by_the_taxi_branch() {
        // The decoder cannot tell them apart, and is not supposed to: it is a decoder.
        let body = menu_answer(2);
        assert_eq!(parse_menu_reply(&body).unwrap().selection, Some(2));

        // So the router is the conversation path, and only a live taxi menu matches it.
        assert!(is_taxi_path(MENU_PATH));
        for other in ["", "0", "1", "0.yes", "0.no", "taxi", "taxi.0", "phil"] {
            assert!(!is_taxi_path(other), "{other:?} must not read as a taxi conversation");
        }
        // Phil sends their menu with no conversation parked at all, which is the state the
        // wiring patch checks. Nothing here can produce a taxi path by accident.
        assert_ne!(MENU_PATH, "");
    }

    /// **A guard on the one thing in the wiring patch that cannot be compiled here.**
    ///
    /// `crates/world/src/session/npc.rs` belongs to the coordinator, so the patch that calls
    /// this module exists only as a diff in a report until someone applies it - `CLAUDE.md`'s
    /// *built is not wired*. The single non-obvious thing that diff assumes is that
    /// `&self.store` and `&self.config`, which are `Arc`s, coerce to the `&Store` and
    /// `&Config` these functions take. That much can be checked from inside this file, so it
    /// is, and a signature change that would break the patch breaks this test instead of
    /// breaking the integration.
    #[test]
    fn the_wiring_patch_can_pass_the_sessions_arcs_straight_in() {
        use std::sync::Arc;
        let (store, id) = store_with(1_000);
        let store: Arc<Store> = Arc::new(store);
        let cfg: Arc<Config> = Arc::new(config());
        let step = on_reply(&store, &cfg, id, lyn(), &menu_answer(0)).unwrap();
        assert!(matches!(step, Step::Ride { .. }));
        let _ = opening(lyn(), &cfg);
    }

    // -- the voices ----------------------------------------------------------------------

    /// The menu names every destination and is one box, not five.
    #[test]
    fn the_menu_lists_every_destination_in_one_box() {
        let cfg = config();
        let Some(Step::Menu { text }) = opening(lyn(), &cfg) else { panic!() };
        for map in destinations(lyn()) {
            assert!(text.contains(&map_name(&cfg, map)), "{map} missing from {text:?}");
        }
        assert_eq!(text.matches("#L").count(), 5, "five selectable lines: {text:?}");
        assert_eq!(script_replies(lyn(), &opening(lyn(), &cfg).unwrap()).len(), 1, "ONE box");
    }

    /// **Lyn is a tour guide and the cabs are cabs**, and the difference is in the words
    /// rather than in the mechanism. Their opening is the line `String.wz` already gives them.
    #[test]
    fn lyn_sounds_like_a_tour_guide_and_a_cab_sounds_like_a_cab() {
        let cfg = config();
        let lyn_text = menu_text(lyn(), &cfg);
        let cab_text = menu_text(henesys_cab(), &cfg);

        assert!(lyn_text.starts_with("Welcome to Classic World!"), "{lyn_text:?}");
        assert!(lyn_text.contains("tour"), "{lyn_text:?}");
        assert!(lyn_text.contains("Lyn"), "they introduce themself: {lyn_text:?}");
        assert!(!cab_text.contains("tour"), "a cab does not run tours: {cab_text:?}");
        assert!(cab_text.starts_with("Where to?"), "{cab_text:?}");

        // Same mechanism underneath: same fare, same list format. Kerning City rather than
        // Henesys, because the Henesys cab does not offer its own town - asserting on
        // Henesys here would be asserting the bug `destinations` exists to prevent.
        for t in [&lyn_text, &cab_text] {
            assert!(t.contains(&FARE_MESOS.to_string()), "{t:?}");
            assert!(t.contains("Kerning City"), "{t:?}");
            assert!(t.contains("#d#L0# "), "the client's own line format: {t:?}");
        }
        // And their refusals keep the voice.
        assert!(cannot_afford(lyn(), 10).contains("tour"), "{}", cannot_afford(lyn(), 10));
        assert!(no_such_stop(lyn(), 5).contains("tour"), "{}", no_such_stop(lyn(), 5));
        assert!(!cannot_afford(henesys_cab(), 10).contains("tour"));
    }
}
