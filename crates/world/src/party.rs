//! Parties - the pure decision, and the one predicate the rest of the server wants.
//!
//! Full working, every enumeration and every named blind spot: **`research/party.md`**.
//! The wire is **`crates::net::party`**.
//!
//! Labels are the project's: **[L]** read off this client's listing, its WZ, its string
//! table or a capture; **[D]** derived from two or more [L] facts; **[I]** inferred - a
//! policy nothing on this machine can confirm.
//!
//! # Nothing here authenticates
//!
//! As everywhere in this project, the channel socket carries no credentials. A party is
//! created, joined and disbanded on the say-so of whoever holds the socket.
//!
//! # THIS MODULE IS NOT WIRED
//!
//! Nothing calls anything in this file. `crates/world/src/session/`, `fields.rs` and
//! `broadcast.rs` belong to the coordinator and were not touched. `research/party.md`
//! § "WIRE IT LIKE THIS" is the integration. `CLAUDE.md`: *"built is not wired"*.
//!
//! # The one function everything else needs
//!
//! [`Parties::audience`]. *Given a character, who else should see what they see.* It is
//! **total** - a character in no party is their own audience of one - and it is an
//! equivalence relation, which [`tests`] asserts directly rather than by inspection.
//!
//! ```
//! # use world::party::Parties;
//! let parties = Parties::new();
//! assert_eq!(parties.audience(200).as_slice(), &[200]);   // nobody is in a party yet
//! assert!(parties.shares_audience(200, 200));
//! assert!(!parties.shares_audience(200, 201));
//! ```
//!
//! ## It is party membership, NOT co-location
//!
//! `audience` knows nothing about maps, channels or who is logged in. Party drops want
//! `audience(killer)` **intersected with the field**, and the intersection is the caller's:
//! `crate::broadcast::Bus` already holds the map of every subscriber and this module
//! deliberately does not duplicate it. A member on another map is in the audience and must
//! not be sent a field packet.
//!
//! Saying that here rather than in the caller is deliberate. The client's own party strings
//! are full of location rules - *"This can only be given to a party member within the
//! vicinity."*, *"All party members must be on the same channel."*, *"Party invites cannot be
//! sent to '%s''s current location."* **[L]** - so "in my party" and "can see what I see" are
//! two different questions in this client too, and a predicate that quietly answered both
//! would be wrong in the direction that is hardest to notice.
//!
//! # Why drop visibility needs no party support in the client at all
//!
//! `crates/net/src/drops.rs` establishes that `ownType` at `DropEnterField` body offset 27
//! is **stored at `drop+0x70` and never gated on**: *"ownership is entirely the server's job
//! here - the client will happily ask to pick up a drop it does not own"*. **[L]**
//!
//! So making a party see each other's drops is entirely a question of **who the server sends
//! `0x046E` to**. It needs no `0x00A5`, no party window, and no decode of the FlatBuffers
//! request body. That is why this module is a membership registry and a predicate and
//! nothing else - see `research/party.md` § "The smallest slice".
//!
//! # The seam into `crate::mobshare`, which already exists
//!
//! `crate::mobshare::Party::of(character, members)` is the value that module's
//! `may_see_drop` takes, and its doc says the party agent's job is to *produce* it rather
//! than to change a signature. [`Audience`] is exactly that list, so the seam is one line at
//! the call site and **nothing in this file references `mobshare` on purpose** - the
//! dependency runs one way:
//!
//! ```ignore
//! // instead of  mobshare::Party::solo(owner)
//! mobshare::Party::of(owner, parties.audience(owner).as_slice().iter().copied())
//! ```
//!
//! Both types promise the same three things - non-empty, contains the character asked
//! about, no duplicates - so the conversion cannot lose a member or invent one.
//!
//! # The state machine's rules are the client's own strings, not a convention
//!
//! Nine of the eleven rules below are read out of `String.wz` via `tools/dump_stringids.py`,
//! in the contiguous party block `0x0108..0x0132`. Two are marked [I] because nothing in this
//! client fixes them.
//!
//! | rule | evidence |
//! |---|---|
//! | a character is in at most one party | `0x0121` *"Already have joined a party."*, `0x0132` *"You are already in a party."* **[L]** |
//! | you must be in a party to leave it | `0x0124` *"You have yet to join a party."* **[L]** |
//! | only the leader invites, expels and hands over | `0x0126` *"You are not the master of the party."*, resolved by the invite, expel and change-leader **builders** - the client refuses locally before sending **[D]** |
//! | you may not invite yourself | `0x0127` *"You may not invite yourself to the party."* **[L]** |
//! | you may not invite someone already in a party | `0x0122` *"'%s' is already in a party."* **[L]** |
//! | a second invite to the same character is refused | `0x011F` *"You have already invited '%s' to your party."* **[L]** |
//! | you may only expel or promote a member | `0x0128` *"'%s' is not a member of your party."* **[L]** |
//! | a full party refuses a join | `0x0125` *"The party you're trying to join is already in full capacity."* **[L]** |
//! | **the leader leaving DISBANDS the party** | `0x0114` *"You have quit as the leader of the party. The party has been disbanded."* and `0x0115` *"You have left the party since the party leader quit."* **[L]** |
//! | **the leader DISCONNECTING promotes instead** | `0x0AF2` *"Due to the party leader disconnecting from the game, %s has been assigned as the new leader."* **[L]** |
//! | the party holds at most [`MAX_MEMBERS`] | **[I]** - see [`MAX_MEMBERS`] |
//!
//! The two leader rows are the reason this module has both [`Parties::apply`] and
//! [`Parties::disconnect`]: **the client has a different message for each**, so they are
//! different transitions and not one with a flag. Collapsing them would have been the
//! obvious thing to do and the client says not to.
//!
//! # A refusal is returned, not logged
//!
//! [`Parties::apply`] returns `Result<Vec<Effect>, Refusal>`. That shape is
//! `CLAUDE.md`'s Heena-quest lesson made structural: *"every effect hangs off the transition,
//! not off the request"*, and *"a refusal that is reported to no one will be ignored
//! eventually"*. There is no way to reach the effects of a refused request, because on a
//! refusal there is no `Vec<Effect>` to reach.
//!
//! # Every number carries its unit
//!
//! There is one number in this file - [`MAX_MEMBERS`] - and its unit is **characters,
//! including the leader**.

use std::collections::BTreeMap;

/// A character id. Ids start at 200 in this project (`store::FIRST_CHARACTER_ID`); nothing
/// here depends on that.
pub type CharacterId = u32;

/// A party id, as the server assigns them.
pub type PartyId = u32;

/// Why an invite was dropped, as the invitee's client reported it in `0x0183`.
///
/// The client answers an invite with the **outcome code** the leader should be shown
/// (`net::party::invite_answer`): the Decline button, or one of three automatic refusals its
/// own `0x03` handler emits instead of opening a dialog. The state machine only carries the
/// reason; the session turns it into the `0x1B` sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclineReason {
    /// The Decline button (answer 4) - or a disconnect, which reads the same to the leader.
    Refused,
    /// The invitee blocks party invitations (answer 1); no dialog was shown.
    Blocking,
    /// The invitee is already looking at another invitation (answer 2); no dialog.
    Busy,
    /// The invitee's client already holds this very invite (answer 3); no dialog.
    AlreadyInvited,
}

/// The id that means "no party". Never assigned to a real one.
///
/// This is a **server-side convention and nothing more**. `0x00A5` code `0x0E` really does
/// carry a `u32` party id at body offset 1 (`0x1413bbb95`, stored to a global) **[L]**, but
/// nothing establishes that the client treats `0` specially, so this constant is not a claim
/// about the client. It exists so that a `PartyId` in a database column or a packet field can
/// carry "none" without an `Option`.
pub const NO_PARTY: PartyId = 0;

/// The first id [`Parties`] hands out.
pub const FIRST_PARTY_ID: PartyId = 1;

/// The largest party this server will assemble, **in characters, leader included**.
///
/// **[I], and the negative behind it is measured rather than assumed.** Two places in the
/// client that would fix a cap do not:
///
/// * `UI/UI_000.wz/UserList.img/Party` has a `scroll:list_scroll` with `length = 270` and
///   `wheelRange = 289` over a list box of `vector:list_lt (8,57)` to `vector:list_rb
///   (289,297)`. **The member list scrolls**, so the window's height caps nothing. **[L]**
/// * `FUN_1413b8d40`, the routine the create-result arm calls first, reads its bound out of a
///   global (`0x1413b8d44 mov eax,[rip+0x27139b6] / test eax,eax / jle`) and passes it as a
///   runtime count. No fixed array. **[L]**
///
/// **The named blind spot**: those are two places, not an enumeration of the party module. A
/// cap could sit in a function neither of them reaches, and this negative would not see it.
/// What would settle it: enumerate every immediate compared against a member count in
/// `0x1413b8000..0x1413bf000`, the way `research/party.md` §2 enumerates string ids - or,
/// far more cheaply, seven characters and one screen, once this machine can run two clients.
///
/// 6 is the number every MapleStory anyone has played uses. It is **[I]** here because this
/// client has not been asked.
pub const MAX_MEMBERS: usize = 6;

/// One party.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Party {
    /// Assigned by [`Parties`], never [`NO_PARTY`].
    pub id: PartyId,
    /// The name the client sent with [`net::party::action::CREATE`], or whatever the caller
    /// supplied. The client defaults it to `String.wz` `0x0BAC` *"%s's Party"* with the
    /// creator's name; that default is the **client's**, and this module does not invent one.
    pub name: String,
    /// Always a member of [`Party::members`].
    pub leader: CharacterId,
    /// **Join order, leader first at creation.** Not sorted: the order a party window lists
    /// its members in is the order the server sends them, so throwing it away here would
    /// throw away information the wire needs later.
    pub members: Vec<CharacterId>,
    /// The item pick-up-rights mode: **`1` = Party Leader only, `0` = All** - the client's
    /// own two words for it (`net::party::PartyBlock::leader_only_pickup`). A party starts
    /// at All. The `pickup` button carries no value (its builder hardcodes the payload), so
    /// [`Request::SetPickupRights`] is a **toggle**. Under Party Leader, a party drop may be
    /// taken by the leader (and its killer, who owns it) and by nobody else.
    pub pickup_rights: u8,
}

/// [`Party::pickup_rights`] for "the leader alone picks up party drops".
pub const PICKUP_LEADER_ONLY: u8 = 1;
/// [`Party::pickup_rights`] for "any member picks up party drops".
pub const PICKUP_ALL: u8 = 0;

impl Party {
    /// Whether `who` is in this party.
    pub fn has(&self, who: CharacterId) -> bool {
        self.members.contains(&who)
    }

    /// How many characters are in it, leader included.
    pub fn size(&self) -> usize {
        self.members.len()
    }

    /// Whether another member would exceed [`MAX_MEMBERS`].
    pub fn is_full(&self) -> bool {
        self.members.len() >= MAX_MEMBERS
    }
}

/// What a character asked to do.
///
/// The seven the client can actually send are enumerated in [`net::party::action`], read off
/// the party window's own button dispatcher. This enum is deliberately **not** that list:
/// `Accept` and `Decline` arrive on `0x0183`, not `0x0182`, and
/// [`net::party::action::SET_PICKUP_RIGHTS`] is left out because this module makes no
/// decision about pick-up rights - `research/party.md` §8 item 9 says why, and drop
/// visibility does not depend on it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    /// [`net::party::action::CREATE`].
    Create {
        /// The party's name. Empty is refused; the client always supplies one.
        name: String,
    },
    /// [`net::party::action::INVITE`]. Leader only.
    Invite { target: CharacterId },
    /// The invitee accepting. Arrives on `0x0183`, not `0x0182`.
    Accept { party: PartyId },
    /// The invitee declining. Arrives on `0x0183`.
    Decline { party: PartyId, reason: DeclineReason },
    /// [`net::party::action::LEAVE`]. **If the actor is the leader this disbands the party**
    /// - see the module docs.
    Leave,
    /// [`net::party::action::EXPEL`]. Leader only.
    Expel { target: CharacterId },
    /// [`net::party::action::CHANGE_LEADER`]. Leader only.
    ChangeLeader { target: CharacterId },
    /// [`net::party::action::SET_PICKUP_RIGHTS`]. Leader only. **A toggle**: the client's
    /// button sends no value (`rights` is the request's constant slot, kept for the log), and
    /// each press flips the party between [`PICKUP_ALL`] and [`PICKUP_LEADER_ONLY`].
    SetPickupRights { rights: u8 },
    /// **The actor's connection went away** - a log out, a crash, a dropped socket; NOT a
    /// channel change, which the session tells apart. Never sent by the client; the session
    /// raises it on its way out, through the same hub path as every other request.
    ///
    /// **The actor stays in the party.** The owner, 2026-09-18: *"Only the party leader should be
    /// handed off. The disconnected client should remain in the party. The party should
    /// persist even if all members have disconnected. If the leader position cannot be handed
    /// off to an online player, then the entire party should be disbanded."* So a member's
    /// disconnect changes nothing here; a leader's moves the crown to `successor` - the
    /// **highest-level ONLINE member**, chosen by the session, which knows who is online and
    /// what level they are (this registry knows neither) - and when the session found nobody
    /// online (`None`), the party is disbanded. The registry validates the pick: a successor
    /// who is not another member is treated as none, so a stale or malformed frame cannot
    /// crown a stranger.
    Disconnect { successor: Option<CharacterId> },
}

/// Why a [`Request`] changed nothing.
///
/// A `Refusal` is the *whole* answer: [`Parties::apply`] returns it in place of the effects,
/// so no effect of a refused request can be reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The actor is already in a party. `0x0121` / `0x0132`.
    AlreadyInAParty,
    /// The actor is in no party. `0x0124`.
    NotInAParty,
    /// The actor is in a party but is not its leader. `0x0126`.
    NotTheLeader,
    /// The party already holds [`MAX_MEMBERS`]. `0x0125`.
    PartyIsFull,
    /// You may not invite yourself. `0x0127`.
    TargetIsYourself,
    /// The invitee is already in some party. `0x0122`.
    TargetAlreadyInAParty,
    /// This character has already been invited to this party. `0x011F`.
    TargetAlreadyInvited,
    /// There is no outstanding invite for this (party, character) pair. `0x012D`.
    NoSuchInvite,
    /// The target is not in the actor's party. `0x0128`.
    NotAMember,
    /// A create with an empty name. No client string; the client never sends one.
    NameIsEmpty,
    /// The party id in the request does not exist. `0x012D`.
    NoSuchParty,
}

impl Refusal {
    /// The [`net::party::PARTY_RESULT`] code to answer with.
    ///
    /// **Only four refusals have a code of their own in this client**, and all four are
    /// zero-read arms, so all four are a two-byte body. Everything else falls back to
    /// [`net::party::result::UNKNOWN_ERROR`], which is the client's own default arm and shows
    /// *"Due to an unknown error, your party request failed."* **[L]**
    ///
    /// That is not a gap in the search. `0x0126` *"You are not the master of the party."*,
    /// `0x0127` *"You may not invite yourself"* and `0x0128` *"is not a member of your
    /// party"* are resolved **only by the outbound request builders** - the client refuses
    /// those three locally and never sends the request - so no inbound code carries them and
    /// a server that never sees the request also never has to answer one. `research/party.md`
    /// §3 has the enumeration.
    ///
    /// Every value this returns satisfies `net::party::is_silent_code`, which is asserted in
    /// [`tests`] rather than promised here.
    pub fn result_code(self) -> u8 {
        use net::party::result;
        match self {
            Refusal::AlreadyInAParty => result::CREATE_REFUSED_ALREADY_IN_ONE,
            Refusal::NotInAParty => result::NOT_IN_A_PARTY,
            Refusal::PartyIsFull => result::JOIN_REFUSED_FULL,
            Refusal::NotTheLeader
            | Refusal::TargetIsYourself
            | Refusal::TargetAlreadyInAParty
            | Refusal::TargetAlreadyInvited
            | Refusal::NoSuchInvite
            | Refusal::NotAMember
            | Refusal::NameIsEmpty
            | Refusal::NoSuchParty => result::UNKNOWN_ERROR,
        }
    }

    /// The client's own wording, for a server-side chat line or a log.
    ///
    /// Quoted from `String.wz` locale 0 as `tools/dump_stringids.py` decrypts it. These are
    /// **not** sent as text - the client renders its own copy from the result code - so this
    /// exists so a log line and a screen say the same thing.
    pub fn message(self) -> &'static str {
        match self {
            Refusal::AlreadyInAParty => "Already have joined a party.",
            Refusal::NotInAParty => "You have yet to join a party.",
            Refusal::NotTheLeader => "You are not the master of the party.",
            Refusal::PartyIsFull => {
                "The party you're trying to join is already in full capacity."
            }
            Refusal::TargetIsYourself => "You may not invite yourself to the party.",
            Refusal::TargetAlreadyInAParty => "'%s' is already in a party.",
            Refusal::TargetAlreadyInvited => "You have already invited '%s' to your party.",
            Refusal::NoSuchInvite => {
                "Party cannot be found. Please check the party info once again."
            }
            Refusal::NotAMember => "'%s'is not a member of your party.",
            Refusal::NameIsEmpty => "You cannot make a party",
            Refusal::NoSuchParty => {
                "Party cannot be found. Please check the party info once again."
            }
        }
    }
}

/// How a character stopped being in a party.
///
/// Four values because the client has four messages, not because four felt tidy. **[L]**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Departure {
    /// They chose to. `0x010E` *"You have left the party."*, `0x0111` *"'%s' have left"*.
    Voluntary,
    /// The leader threw them out. `0x010D` / `0x0110`.
    Expelled,
    /// The leader left, so everyone did. `0x0114` / `0x0115`.
    Disbanded,
    /// They dropped their connection. No message of its own for a member; the *leader's*
    /// disconnect has `0x0AF2`.
    Disconnected,
}

/// What the caller has to make happen.
///
/// An [`Effect`] describes a transition that **has already been applied** to the registry.
/// Nothing here sends anything; the caller turns each one into packets and database writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    /// A new party exists. `leader` is in it and in nothing else.
    Created { party: PartyId, leader: CharacterId },
    /// `target` now has an outstanding invite to `party`. Tell `target`.
    Invited { party: PartyId, from: CharacterId, target: CharacterId },
    /// The invite is gone without a join - declined, or overtaken by events.
    InviteDropped { party: PartyId, target: CharacterId, reason: DeclineReason },
    /// `who` is now a member of `party`. Tell everyone in it, `who` included.
    Joined { party: PartyId, who: CharacterId },
    /// `who` is no longer a member. `remaining` is the party **after** the departure, and is
    /// empty exactly when [`Effect::Disbanded`] follows.
    Departed {
        party: PartyId,
        who: CharacterId,
        how: Departure,
        remaining: Vec<CharacterId>,
    },
    /// The party's leader changed. Both ids are still members.
    LeaderChanged { party: PartyId, from: CharacterId, to: CharacterId },
    /// The party's item pick-up-rights mode changed. Tell every member so their window agrees.
    PickupRightsChanged { party: PartyId, rights: u8 },
    /// The party no longer exists. `members` is who was in it at the moment it ended,
    /// **including** anyone whose [`Effect::Departed`] is in the same batch, so the caller has
    /// one list to notify and does not have to reassemble it.
    Disbanded { party: PartyId, members: Vec<CharacterId> },
}

/// Everyone who should be shown whatever one character is shown.
///
/// Guaranteed: **non-empty**, sorted ascending, free of duplicates, and it always contains
/// the character it was asked about. Those four are what make it safe to use as the argument
/// to a broadcast without a second check, and [`tests`] asserts all four.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Audience(Vec<CharacterId>);

impl Audience {
    /// Sorted, deduplicated, never empty.
    pub fn as_slice(&self) -> &[CharacterId] {
        &self.0
    }

    /// Whether `who` is in it.
    pub fn contains(&self, who: CharacterId) -> bool {
        self.0.binary_search(&who).is_ok()
    }

    /// How many characters. At least 1.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Always `false` - an [`Audience`] is never empty. Present because clippy asks for it
    /// next to [`Audience::len`], and because a caller that writes `if a.is_empty()` should
    /// see it is dead rather than wonder.
    pub fn is_empty(&self) -> bool {
        false
    }

    /// Whether this character has nobody to share with. The common case.
    pub fn is_solo(&self) -> bool {
        self.0.len() == 1
    }

    /// Everyone except `who`. The list to broadcast to when `who` gets their own reply.
    pub fn others(&self, who: CharacterId) -> Vec<CharacterId> {
        self.0.iter().copied().filter(|&c| c != who).collect()
    }

    /// Iterate.
    pub fn iter(&self) -> std::slice::Iter<'_, CharacterId> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a Audience {
    type Item = &'a CharacterId;
    type IntoIter = std::slice::Iter<'a, CharacterId>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Every party this channel knows about.
///
/// Pure data and pure decisions: no database, no packets, no sessions, no clock. The caller
/// owns it - `Fields` wraps its state in a `Mutex` and this is meant to sit beside it.
#[derive(Debug, Clone, Default)]
pub struct Parties {
    parties: BTreeMap<PartyId, Party>,
    /// Which party each character is in. The **only** index; `parties` is the truth and this
    /// is kept in step by [`Parties::apply`] alone. [`Parties::check_invariants`] proves it.
    of: BTreeMap<CharacterId, PartyId>,
    /// Outstanding invites, `(party, invitee) -> the unix second it was minted`. An invitee
    /// may hold several at once - the client's `0x011F` refusal is per *(party, character)*,
    /// not per character. **[L]**
    ///
    /// The timestamp is what [`Parties::expire_invites`] ages out. It arrived 2026-09-05:
    /// The owner, *"the client fades the party invitation out after 30 seconds of client inaction,
    /// I don't want the server to keep waiting for a reply it will never get."* Before it a
    /// faded invite lived forever, and a leader could never re-invite that character - the
    /// state machine kept answering [`Refusal::TargetAlreadyInvited`].
    invites: BTreeMap<(PartyId, CharacterId), i64>,
    next_id: PartyId,
}

/// How long the server keeps an outstanding invite before it lapses, in seconds.
///
/// The client fades the invite dialog after **~30 s** of inaction (the owner, 2026-09-05 - **[I]**,
/// an observation not read off the client). This is deliberately **longer** than that, because
/// the two failures are not symmetric: expire too EARLY and an invitee who clicks Accept just
/// as the dialog fades gets a silent "no such invite" for an action that looked valid; expire
/// too LATE and the only cost is that the leader's re-invite is refused with the truthful
/// "already invited" for a few extra seconds until it lapses. So err long. Once the dialog is
/// gone no accept can arrive, so holding past the fade only affects the re-invite window.
pub const INVITE_TTL_SECS: i64 = 60;

impl Parties {
    /// An empty registry.
    pub fn new() -> Self {
        Parties { next_id: FIRST_PARTY_ID, ..Default::default() }
    }

    // --- the predicate ------------------------------------------------------------------

    /// **Given a character, who else should see what they see.**
    ///
    /// The party they are in, or just themselves. Total: no `Option`, no panic, defined for
    /// every `u32` including ids that have never existed.
    ///
    /// Remember that this is membership and **not co-location** - see the module docs.
    pub fn audience(&self, who: CharacterId) -> Audience {
        match self.of.get(&who).and_then(|id| self.parties.get(id)) {
            Some(p) => {
                let mut v = p.members.clone();
                if !v.contains(&who) {
                    // Unreachable while the invariant holds; still cheaper than a panic.
                    v.push(who);
                }
                v.sort_unstable();
                v.dedup();
                Audience(v)
            }
            None => Audience(vec![who]),
        }
    }

    /// Whether `a` and `b` share an audience.
    ///
    /// An equivalence relation: reflexive for every id, symmetric, and transitive because
    /// party membership is a partition. [`tests`] checks all three against a populated
    /// registry rather than leaving them as a claim in a doc comment.
    pub fn shares_audience(&self, a: CharacterId, b: CharacterId) -> bool {
        a == b || (self.of.get(&a).is_some() && self.of.get(&a) == self.of.get(&b))
    }

    // --- reading ------------------------------------------------------------------------

    /// The party `who` is in.
    /// Every party, for the world link's snapshot. Invites are transient and not included.
    pub fn snapshot(&self) -> Vec<Party> {
        self.parties.values().cloned().collect()
    }

    /// The id the next created party will get, so a replica mints the same one.
    pub fn next_id(&self) -> PartyId {
        self.next_id
    }

    /// **Replace everything with a snapshot from the hub.** A channel that connects late (or
    /// reconnects) has a replica that may have missed requests; the hub's copy is the truth,
    /// and from here on both apply the same echoed sequence. Outstanding invites are dropped
    /// - an invite raced across a reconnect fades on the client anyway.
    pub fn restore(&mut self, parties: Vec<Party>, next_id: PartyId) {
        self.parties.clear();
        self.of.clear();
        self.invites.clear();
        for p in parties {
            for m in &p.members {
                self.of.insert(*m, p.id);
            }
            self.parties.insert(p.id, p);
        }
        self.next_id = next_id.max(FIRST_PARTY_ID);
    }

    pub fn party_of(&self, who: CharacterId) -> Option<&Party> {
        self.of.get(&who).and_then(|id| self.parties.get(id))
    }

    /// The party `who` is in, by id.
    pub fn party_id_of(&self, who: CharacterId) -> Option<PartyId> {
        self.of.get(&who).copied()
    }

    /// A party by id.
    pub fn party(&self, id: PartyId) -> Option<&Party> {
        self.parties.get(&id)
    }

    /// Whether `who` is the leader of the party they are in.
    pub fn is_leader(&self, who: CharacterId) -> bool {
        self.party_of(who).is_some_and(|p| p.leader == who)
    }

    /// Whether `target` holds an outstanding invite to `party`. Does **not** age: call
    /// [`Parties::expire_invites`] first if the answer must exclude lapsed ones.
    pub fn has_invite(&self, party: PartyId, target: CharacterId) -> bool {
        self.invites.contains_key(&(party, target))
    }

    /// Every outstanding invite `target` holds, oldest id first.
    pub fn invites_for(&self, target: CharacterId) -> Vec<PartyId> {
        self.invites.keys().filter(|(_, c)| *c == target).map(|(p, _)| *p).collect()
    }

    /// **Drop every invite older than [`INVITE_TTL_SECS`], and say which went.**
    ///
    /// This is the answer to the owner's timeout: a faded invite is not a decline, so it emits **no
    /// [`Effect`]** - the leader is told nothing, because the client already removed the dialog
    /// on its own and there is no "invite lapsed" packet to send. Contrast [`Parties::decline`],
    /// which is an active refusal and does tell the leader. The returned list is for the log,
    /// so a run can show an invite lapsing rather than a re-invite mysteriously succeeding.
    ///
    /// [`Parties::apply`] calls this at every entry, so a re-invite or an accept always sees an
    /// aged-out invite as gone; a caller may also call it on a timer to reclaim memory.
    pub fn expire_invites(&mut self, now: i64) -> Vec<(PartyId, CharacterId)> {
        let mut gone = Vec::new();
        self.invites.retain(|key, minted| {
            let live = now - *minted < INVITE_TTL_SECS;
            if !live {
                gone.push(*key);
            }
            live
        });
        gone
    }

    /// How many parties exist.
    pub fn len(&self) -> usize {
        self.parties.len()
    }

    /// Whether no party exists.
    pub fn is_empty(&self) -> bool {
        self.parties.is_empty()
    }

    // --- the transitions ----------------------------------------------------------------

    /// Apply one request. **Nothing changes on a [`Refusal`].**
    ///
    /// The `Err` arm is not an error in the "something went wrong" sense - it is the ordinary
    /// answer to *"may I?"*, and the caller answers it with
    /// `net::party::refusal(refusal.result_code())`. `CLAUDE.md`'s **always answer** rule: a
    /// refused request still needs a reply, and there is no path here that returns nothing.
    pub fn apply(
        &mut self,
        now: i64,
        actor: CharacterId,
        request: Request,
    ) -> Result<Vec<Effect>, Refusal> {
        // Every entry ages invites first, so a re-invite sees a faded one as gone and an
        // accept of a lapsed invite is `NoSuchInvite` rather than a join. The dropped list is
        // discarded here; the session logs it via its own `expire_invites` call. See the
        // field doc and `expire_invites`.
        self.expire_invites(now);
        match request {
            Request::Create { name } => self.create(actor, name),
            Request::Invite { target } => self.invite(now, actor, target),
            Request::Accept { party } => self.accept(actor, party),
            Request::Decline { party, reason } => self.decline(actor, party, reason),
            Request::Leave => self.leave(actor),
            Request::Expel { target } => self.expel(actor, target),
            Request::ChangeLeader { target } => self.change_leader(actor, target),
            Request::SetPickupRights { rights } => self.set_pickup_rights(actor, rights),
            Request::Disconnect { successor } => Ok(self.disconnect_to(actor, successor)),
        }
    }

    fn create(&mut self, actor: CharacterId, name: String) -> Result<Vec<Effect>, Refusal> {
        if name.trim().is_empty() {
            return Err(Refusal::NameIsEmpty);
        }
        if self.of.contains_key(&actor) {
            return Err(Refusal::AlreadyInAParty);
        }
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        self.parties.insert(
            id,
            Party { id, name, leader: actor, members: vec![actor], pickup_rights: PICKUP_ALL },
        );
        self.of.insert(actor, id);
        // A character who was invited somewhere and then made their own party cannot still
        // hold those invites: accepting one would put them in two parties at once. Dropping
        // them here rather than at Accept is the same rule as `record_quest_forfeit`'s -
        // enforce it where the state changes, not at the caller.
        let dropped = self.drop_invites_to(actor);
        let mut out = vec![Effect::Created { party: id, leader: actor }];
        out.extend(dropped);
        Ok(out)
    }

    fn invite(
        &mut self,
        now: i64,
        actor: CharacterId,
        target: CharacterId,
    ) -> Result<Vec<Effect>, Refusal> {
        if actor == target {
            return Err(Refusal::TargetIsYourself);
        }
        let party = self.leader_party(actor)?;
        if self.parties[&party].is_full() {
            return Err(Refusal::PartyIsFull);
        }
        if self.of.contains_key(&target) {
            return Err(Refusal::TargetAlreadyInAParty);
        }
        // `apply` expired stale invites before this ran, so a key still here is genuinely
        // live - a real double-invite, not a faded one. Stamp the fresh one with `now`.
        if self.invites.contains_key(&(party, target)) {
            return Err(Refusal::TargetAlreadyInvited);
        }
        self.invites.insert((party, target), now);
        Ok(vec![Effect::Invited { party, from: actor, target }])
    }

    fn accept(&mut self, actor: CharacterId, party: PartyId) -> Result<Vec<Effect>, Refusal> {
        if !self.invites.contains_key(&(party, actor)) {
            return Err(Refusal::NoSuchInvite);
        }
        if self.of.contains_key(&actor) {
            return Err(Refusal::AlreadyInAParty);
        }
        let Some(p) = self.parties.get_mut(&party) else {
            // The party was disbanded while the invite was outstanding. Clear the stale
            // invite so the same click cannot be refused twice for the same reason.
            self.invites.remove(&(party, actor));
            return Err(Refusal::NoSuchParty);
        };
        if p.is_full() {
            return Err(Refusal::PartyIsFull);
        }
        p.members.push(actor);
        self.of.insert(actor, party);
        let mut out = vec![Effect::Joined { party, who: actor }];
        // Joining consumes this invite and voids every other one, for the same reason
        // create does.
        self.invites.remove(&(party, actor));
        out.extend(self.drop_invites_to(actor));
        Ok(out)
    }

    /// The leader toggles the party's item pick-up-rights mode. The client gates the button
    /// on being the leader and so does this, so a non-leader request is `NotTheLeader` rather
    /// than a silent success. The request's own byte is ignored: the button hardcodes it.
    fn set_pickup_rights(&mut self, actor: CharacterId, _sent: u8) -> Result<Vec<Effect>, Refusal> {
        let party = *self.of.get(&actor).ok_or(Refusal::NotInAParty)?;
        let p = self.parties.get_mut(&party).ok_or(Refusal::NoSuchParty)?;
        if p.leader != actor {
            return Err(Refusal::NotTheLeader);
        }
        p.pickup_rights = if p.pickup_rights == PICKUP_LEADER_ONLY { PICKUP_ALL } else { PICKUP_LEADER_ONLY };
        let rights = p.pickup_rights;
        Ok(vec![Effect::PickupRightsChanged { party, rights }])
    }

    fn decline(
        &mut self,
        actor: CharacterId,
        party: PartyId,
        reason: DeclineReason,
    ) -> Result<Vec<Effect>, Refusal> {
        if self.invites.remove(&(party, actor)).is_none() {
            return Err(Refusal::NoSuchInvite);
        }
        Ok(vec![Effect::InviteDropped { party, target: actor, reason }])
    }

    fn leave(&mut self, actor: CharacterId) -> Result<Vec<Effect>, Refusal> {
        let party = *self.of.get(&actor).ok_or(Refusal::NotInAParty)?;
        let leader = self.parties[&party].leader;
        if leader == actor {
            // The leader leaving disbands. `0x0114` / `0x0115`. **[L]**
            return Ok(self.dissolve(party, Departure::Disbanded, Some(actor)));
        }
        Ok(self.remove_member(party, actor, Departure::Voluntary))
    }

    fn expel(
        &mut self,
        actor: CharacterId,
        target: CharacterId,
    ) -> Result<Vec<Effect>, Refusal> {
        if actor == target {
            // Expelling yourself is Leave, and Leave by the leader disbands. Routing it here
            // would quietly disband a party from a button that says "expel".
            return Err(Refusal::NotAMember);
        }
        let party = self.leader_party(actor)?;
        if !self.parties[&party].has(target) {
            return Err(Refusal::NotAMember);
        }
        Ok(self.remove_member(party, target, Departure::Expelled))
    }

    fn change_leader(
        &mut self,
        actor: CharacterId,
        target: CharacterId,
    ) -> Result<Vec<Effect>, Refusal> {
        let party = self.leader_party(actor)?;
        if actor == target {
            return Err(Refusal::NotAMember);
        }
        if !self.parties[&party].has(target) {
            return Err(Refusal::NotAMember);
        }
        self.parties.get_mut(&party).expect("checked above").leader = target;
        Ok(vec![Effect::LeaderChanged { party, from: actor, to: target }])
    }

    /// A character's connection went away, and the session found no successor - see
    /// [`Request::Disconnect`]. Kept as the no-successor form for callers without a session.
    pub fn disconnect(&mut self, who: CharacterId) -> Vec<Effect> {
        self.disconnect_to(who, None)
    }

    /// [`Request::Disconnect`], applied. `who` keeps their seat whatever happens. A leader
    /// hands the crown to `preferred` when that is another member; with no valid successor
    /// the party is dissolved ([`Departure::Disbanded`], the leader named first). A member's
    /// disconnect is no change at all - not even a `Departed`, because nobody departed.
    pub fn disconnect_to(&mut self, who: CharacterId, preferred: Option<CharacterId>) -> Vec<Effect> {
        let mut out = self.drop_invites_to(who);
        let Some(party) = self.of.get(&who).copied() else { return out };
        if self.parties[&party].leader != who {
            return out;
        }
        let successor = preferred.filter(|c| *c != who && self.parties[&party].has(*c));
        match successor {
            Some(next) => {
                self.parties.get_mut(&party).expect("looked up above").leader = next;
                out.push(Effect::LeaderChanged { party, from: who, to: next });
            }
            None => out.extend(self.dissolve(party, Departure::Disbanded, Some(who))),
        }
        out
    }

    // --- shared machinery ---------------------------------------------------------------

    /// The party `actor` leads, or the refusal that says why not.
    fn leader_party(&self, actor: CharacterId) -> Result<PartyId, Refusal> {
        let party = *self.of.get(&actor).ok_or(Refusal::NotInAParty)?;
        if self.parties[&party].leader != actor {
            return Err(Refusal::NotTheLeader);
        }
        Ok(party)
    }

    /// Take one member out, and dissolve the party if that emptied it.
    fn remove_member(
        &mut self,
        party: PartyId,
        who: CharacterId,
        how: Departure,
    ) -> Vec<Effect> {
        let mut out = Vec::new();
        let Some(p) = self.parties.get_mut(&party) else { return out };
        p.members.retain(|&c| c != who);
        self.of.remove(&who);
        let remaining = p.members.clone();
        out.push(Effect::Departed { party, who, how, remaining: remaining.clone() });
        if remaining.is_empty() {
            self.parties.remove(&party);
            self.invites.retain(|(pid, _), _| *pid != party);
            out.push(Effect::Disbanded { party, members: vec![who] });
        }
        out
    }

    /// End a party outright.
    fn dissolve(
        &mut self,
        party: PartyId,
        how: Departure,
        first: Option<CharacterId>,
    ) -> Vec<Effect> {
        let Some(p) = self.parties.remove(&party) else { return Vec::new() };
        let mut order: Vec<CharacterId> = Vec::new();
        if let Some(f) = first {
            if p.members.contains(&f) {
                order.push(f);
            }
        }
        order.extend(p.members.iter().copied().filter(|c| Some(*c) != first));

        let mut out = Vec::new();
        let mut left: Vec<CharacterId> = p.members.clone();
        for who in &order {
            self.of.remove(who);
            left.retain(|c| c != who);
            out.push(Effect::Departed {
                party,
                who: *who,
                how,
                remaining: left.clone(),
            });
        }
        self.invites.retain(|(pid, _), _| *pid != party);
        out.push(Effect::Disbanded { party, members: p.members });
        out
    }

    /// Void every invite held by one character.
    fn drop_invites_to(&mut self, who: CharacterId) -> Vec<Effect> {
        let gone: Vec<PartyId> =
            self.invites.keys().filter(|(_, c)| *c == who).map(|(p, _)| *p).collect();
        for p in &gone {
            self.invites.remove(&(*p, who));
        }
        gone.into_iter()
            .map(|party| Effect::InviteDropped { party, target: who, reason: DeclineReason::Refused })
            .collect()
    }

    /// Every invariant this type promises, checked. Returns the first violation.
    ///
    /// It exists because the `of` index is a second copy of the truth, and `CLAUDE.md` is
    /// explicit that a comment describing a guarantee is not the guarantee. Every test that
    /// mutates the registry calls it afterwards.
    pub fn check_invariants(&self) -> Result<(), String> {
        for (id, p) in &self.parties {
            if p.id != *id {
                return Err(format!("party {id} carries id {}", p.id));
            }
            if p.members.is_empty() {
                return Err(format!("party {id} is empty and still exists"));
            }
            if !p.members.contains(&p.leader) {
                return Err(format!("party {id}'s leader {} is not a member", p.leader));
            }
            let mut seen = p.members.clone();
            seen.sort_unstable();
            seen.dedup();
            if seen.len() != p.members.len() {
                return Err(format!("party {id} lists a character twice"));
            }
            for m in &p.members {
                if self.of.get(m) != Some(id) {
                    return Err(format!("member {m} of party {id} is not indexed to it"));
                }
            }
        }
        for (c, id) in &self.of {
            match self.parties.get(id) {
                None => return Err(format!("character {c} indexed to missing party {id}")),
                Some(p) if !p.has(*c) => {
                    return Err(format!("character {c} indexed to party {id} without being in it"))
                }
                _ => {}
            }
        }
        for (id, c) in self.invites.keys() {
            if !self.parties.contains_key(id) {
                return Err(format!("invite to missing party {id} for {c}"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A fixed "now" for the tests that do not care about invite expiry; the ones that do
    // pass their own. Any value works: a fresh invite is minted at NOW and NOW - NOW < TTL.
    const NOW: i64 = 1_000_000;
    const A: CharacterId = 200;
    const B: CharacterId = 201;
    const C: CharacterId = 202;
    const D: CharacterId = 203;

    /// Build a party of `n` characters led by `A`, ids 200.. . Panics on any refusal, so a
    /// rule change that breaks the fixture fails loudly rather than silently shrinking it.
    fn party_of(n: usize) -> (Parties, PartyId) {
        let mut p = Parties::new();
        p.apply(NOW, A, Request::Create { name: "A's Party".into() }).unwrap();
        let id = p.party_id_of(A).unwrap();
        for i in 1..n {
            let who = A + i as u32;
            p.apply(NOW, A, Request::Invite { target: who }).unwrap();
            p.apply(NOW, who, Request::Accept { party: id }).unwrap();
        }
        p.check_invariants().unwrap();
        (p, id)
    }

    // --- the predicate ------------------------------------------------------------------

    #[test]
    fn a_character_in_no_party_is_their_own_audience_of_one() {
        let p = Parties::new();
        let a = p.audience(A);
        assert_eq!(a.as_slice(), &[A]);
        assert!(a.is_solo());
        assert!(a.contains(A));
        assert!(!a.is_empty());
        assert_eq!(a.others(A), Vec::<CharacterId>::new());
    }

    #[test]
    fn the_audience_of_an_id_that_has_never_existed_is_still_that_id() {
        // Totality. `audience` is called on whatever character id a packet carried.
        let p = Parties::new();
        for who in [0u32, 1, u32::MAX] {
            assert_eq!(p.audience(who).as_slice(), &[who]);
        }
    }

    #[test]
    fn the_audience_of_a_member_is_the_whole_party_sorted_and_deduplicated() {
        let (p, _) = party_of(3);
        let a = p.audience(B);
        assert_eq!(a.as_slice(), &[A, B, C]);
        assert_eq!(a.len(), 3);
        assert!(!a.is_solo());
        assert_eq!(a.others(B), vec![A, C]);
        // Sorted and deduplicated, asserted rather than assumed.
        let mut sorted = a.as_slice().to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted, a.as_slice());
    }

    #[test]
    fn shares_audience_is_an_equivalence_relation() {
        let (p, _) = party_of(3);
        let everyone = [A, B, C, D, 999];
        for &x in &everyone {
            assert!(p.shares_audience(x, x), "not reflexive at {x}");
            for &y in &everyone {
                assert_eq!(
                    p.shares_audience(x, y),
                    p.shares_audience(y, x),
                    "not symmetric at ({x}, {y})"
                );
                for &z in &everyone {
                    if p.shares_audience(x, y) && p.shares_audience(y, z) {
                        assert!(p.shares_audience(x, z), "not transitive at ({x},{y},{z})");
                    }
                }
            }
        }
    }

    #[test]
    fn audience_and_shares_audience_agree() {
        // Two ways of asking the same question must not drift. This is the check that would
        // have caught the `of` index going stale.
        let (p, _) = party_of(3);
        for x in [A, B, C, D] {
            for y in [A, B, C, D] {
                assert_eq!(
                    p.audience(x).contains(y),
                    p.shares_audience(x, y),
                    "({x}, {y})"
                );
            }
        }
    }

    // --- create -------------------------------------------------------------------------

    #[test]
    fn create_makes_a_one_member_party_led_by_its_creator() {
        let mut p = Parties::new();
        let out = p.apply(NOW, A, Request::Create { name: "A's Party".into() }).unwrap();
        let id = p.party_id_of(A).unwrap();
        assert_eq!(out, vec![Effect::Created { party: id, leader: A }]);
        let party = p.party(id).unwrap();
        assert_eq!(party.members, vec![A]);
        assert_eq!(party.leader, A);
        assert_eq!(party.name, "A's Party");
        assert!(p.is_leader(A));
        assert_eq!(p.len(), 1);
        p.check_invariants().unwrap();
    }

    #[test]
    fn create_refuses_a_character_already_in_a_party_and_changes_nothing() {
        let (mut p, id) = party_of(2);
        let before = p.clone();
        let err = p.apply(NOW, B, Request::Create { name: "B's Party".into() }).unwrap_err();
        assert_eq!(err, Refusal::AlreadyInAParty);
        // Every observable, not just the one the refusal is about.
        assert_eq!(p.len(), before.len());
        assert_eq!(p.party_id_of(B), Some(id));
        assert_eq!(p.party(id), before.party(id));
        p.check_invariants().unwrap();
    }

    #[test]
    fn create_refuses_an_empty_name() {
        let mut p = Parties::new();
        assert_eq!(
            p.apply(NOW, A, Request::Create { name: "   ".into() }).unwrap_err(),
            Refusal::NameIsEmpty
        );
        assert!(p.is_empty());
    }

    #[test]
    fn party_ids_start_at_one_and_never_repeat() {
        let mut p = Parties::new();
        p.apply(NOW, A, Request::Create { name: "A".into() }).unwrap();
        let first = p.party_id_of(A).unwrap();
        assert_eq!(first, FIRST_PARTY_ID);
        assert_ne!(first, NO_PARTY);
        p.apply(NOW, A, Request::Leave).unwrap();
        p.apply(NOW, A, Request::Create { name: "A again".into() }).unwrap();
        assert_eq!(p.party_id_of(A), Some(first + 1));
    }

    #[test]
    fn creating_a_party_voids_every_invite_the_creator_held() {
        let (mut p, id) = party_of(1);
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        assert!(p.has_invite(id, B));
        let out = p.apply(NOW, B, Request::Create { name: "B's Party".into() }).unwrap();
        let b_party = p.party_id_of(B).unwrap();
        assert_eq!(
            out,
            vec![
                Effect::Created { party: b_party, leader: B },
                Effect::InviteDropped { party: id, target: B, reason: DeclineReason::Refused },
            ]
        );
        assert!(!p.has_invite(id, B));
        p.check_invariants().unwrap();
    }

    // --- invite / accept / decline ------------------------------------------------------

    #[test]
    fn invite_records_an_invite_and_nothing_else() {
        let (mut p, id) = party_of(1);
        let out = p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        assert_eq!(out, vec![Effect::Invited { party: id, from: A, target: B }]);
        assert!(p.has_invite(id, B));
        // An invite is not a membership. Both halves, because checking one gives false
        // confidence about the other.
        assert_eq!(p.party_id_of(B), None);
        assert_eq!(p.party(id).unwrap().members, vec![A]);
        assert_eq!(p.audience(B).as_slice(), &[B]);
        p.check_invariants().unwrap();
    }

    #[test]
    fn only_the_leader_may_invite() {
        let (mut p, _) = party_of(2);
        assert_eq!(
            p.apply(NOW, B, Request::Invite { target: C }).unwrap_err(),
            Refusal::NotTheLeader
        );
        assert!(p.invites_for(C).is_empty());
    }

    #[test]
    fn you_may_not_invite_yourself() {
        let (mut p, _) = party_of(1);
        assert_eq!(
            p.apply(NOW, A, Request::Invite { target: A }).unwrap_err(),
            Refusal::TargetIsYourself
        );
    }

    #[test]
    fn you_may_not_invite_someone_already_in_a_party() {
        let (mut p, _) = party_of(2);
        p.apply(NOW, C, Request::Create { name: "C's Party".into() }).unwrap();
        assert_eq!(
            p.apply(NOW, A, Request::Invite { target: C }).unwrap_err(),
            Refusal::TargetAlreadyInAParty
        );
    }

    #[test]
    fn a_second_invite_to_the_same_character_is_refused() {
        let (mut p, id) = party_of(1);
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        assert_eq!(
            p.apply(NOW, A, Request::Invite { target: B }).unwrap_err(),
            Refusal::TargetAlreadyInvited
        );
        assert!(p.has_invite(id, B));
        assert_eq!(p.invites_for(B), vec![id]);
    }

    #[test]
    fn a_character_may_hold_invites_from_two_parties_at_once() {
        // `0x011F` refuses a repeat invite from the SAME party. Two parties inviting the
        // same person is a different thing and the client has no string against it.
        let (mut p, one) = party_of(1);
        p.apply(NOW, C, Request::Create { name: "C's Party".into() }).unwrap();
        let two = p.party_id_of(C).unwrap();
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        p.apply(NOW, C, Request::Invite { target: B }).unwrap();
        assert_eq!(p.invites_for(B), vec![one, two]);
    }

    #[test]
    fn accept_joins_and_voids_the_other_invites() {
        let (mut p, one) = party_of(1);
        p.apply(NOW, C, Request::Create { name: "C's Party".into() }).unwrap();
        let two = p.party_id_of(C).unwrap();
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        p.apply(NOW, C, Request::Invite { target: B }).unwrap();

        let out = p.apply(NOW, B, Request::Accept { party: one }).unwrap();
        assert_eq!(
            out,
            vec![
                Effect::Joined { party: one, who: B },
                Effect::InviteDropped { party: two, target: B, reason: DeclineReason::Refused },
            ]
        );
        assert_eq!(p.party_id_of(B), Some(one));
        assert_eq!(p.party(one).unwrap().members, vec![A, B]);
        assert!(p.invites_for(B).is_empty());
        assert_eq!(p.audience(A).as_slice(), &[A, B]);
        p.check_invariants().unwrap();
    }

    #[test]
    fn accept_without_an_invite_is_refused() {
        let (mut p, id) = party_of(1);
        assert_eq!(
            p.apply(NOW, B, Request::Accept { party: id }).unwrap_err(),
            Refusal::NoSuchInvite
        );
        assert_eq!(p.party(id).unwrap().members, vec![A]);
    }

    #[test]
    fn accept_into_a_full_party_is_refused_and_the_invite_survives() {
        let (mut p, id) = party_of(MAX_MEMBERS);
        let outsider = A + MAX_MEMBERS as u32;
        // The party filled up between the invite and the acceptance. The invite was legal
        // when it was sent, so this is the only place the cap can be enforced.
        p.invites.insert((id, outsider), NOW);
        assert_eq!(
            p.apply(NOW, outsider, Request::Accept { party: id }).unwrap_err(),
            Refusal::PartyIsFull
        );
        assert_eq!(p.party(id).unwrap().size(), MAX_MEMBERS);
        assert!(p.has_invite(id, outsider));
        p.check_invariants().unwrap();
    }

    #[test]
    fn inviting_into_a_full_party_is_refused_before_the_invite_is_recorded() {
        let (mut p, id) = party_of(MAX_MEMBERS);
        let outsider = A + MAX_MEMBERS as u32;
        assert_eq!(
            p.apply(NOW, A, Request::Invite { target: outsider }).unwrap_err(),
            Refusal::PartyIsFull
        );
        assert!(!p.has_invite(id, outsider));
    }

    /// Disbanding takes the invites with it, so `NoSuchParty` is not what the invitee sees.
    ///
    /// This test was written expecting [`Refusal::NoSuchParty`] and it was **wrong**: both
    /// paths that end a party already clear its invites, and `check_invariants` enforces
    /// that no invite outlives its party. So [`Refusal::NoSuchParty`] is a guard against a
    /// state the invariant forbids and is unreachable today - kept because `accept` has to
    /// be total, removed the moment something can reach it. Saying so beats deleting the
    /// branch and rediscovering the need for it.
    #[test]
    fn disbanding_a_party_makes_a_pending_accept_read_as_no_such_invite() {
        let (mut p, id) = party_of(1);
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        p.apply(NOW, A, Request::Leave).unwrap(); // the leader leaves: the party disbands
        assert!(p.party(id).is_none());
        assert!(!p.has_invite(id, B));
        assert_eq!(
            p.apply(NOW, B, Request::Accept { party: id }).unwrap_err(),
            Refusal::NoSuchInvite
        );
        // Idempotent: the same click twice gives the same honest answer.
        assert_eq!(
            p.apply(NOW, B, Request::Accept { party: id }).unwrap_err(),
            Refusal::NoSuchInvite
        );
        p.check_invariants().unwrap();
    }

    #[test]
    fn decline_drops_exactly_one_invite() {
        let (mut p, one) = party_of(1);
        p.apply(NOW, C, Request::Create { name: "C's Party".into() }).unwrap();
        let two = p.party_id_of(C).unwrap();
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        p.apply(NOW, C, Request::Invite { target: B }).unwrap();
        let out = p.apply(NOW, B, Request::Decline { party: one, reason: DeclineReason::Refused }).unwrap();
        assert_eq!(out, vec![Effect::InviteDropped { party: one, target: B, reason: DeclineReason::Refused }]);
        assert_eq!(p.invites_for(B), vec![two]);
        assert_eq!(
            p.apply(NOW, B, Request::Decline { party: one, reason: DeclineReason::Refused }).unwrap_err(),
            Refusal::NoSuchInvite
        );
    }

    // --- pick-up rights -----------------------------------------------------------------

    #[test]
    fn the_leader_sets_pick_up_rights_and_it_is_stored() {
        let (mut p, id) = party_of(2);
        // A toggle: All -> Party Leader -> All, whatever byte the button's constant payload
        // carried.
        let out = p.apply(NOW, A, Request::SetPickupRights { rights: 1 }).unwrap();
        assert_eq!(out, vec![Effect::PickupRightsChanged { party: id, rights: PICKUP_LEADER_ONLY }]);
        assert_eq!(p.party(id).unwrap().pickup_rights, PICKUP_LEADER_ONLY);
        let out = p.apply(NOW, A, Request::SetPickupRights { rights: 1 }).unwrap();
        assert_eq!(out, vec![Effect::PickupRightsChanged { party: id, rights: PICKUP_ALL }]);
        assert_eq!(p.party(id).unwrap().pickup_rights, PICKUP_ALL, "back to All");
    }

    #[test]
    fn a_non_leader_cannot_set_pick_up_rights() {
        let (mut p, _id) = party_of(2);
        assert_eq!(
            p.apply(NOW, B, Request::SetPickupRights { rights: 2 }).unwrap_err(),
            Refusal::NotTheLeader
        );
    }

    #[test]
    fn setting_pick_up_rights_with_no_party_is_refused_not_a_panic() {
        let mut p = Parties::new();
        assert_eq!(
            p.apply(NOW, A, Request::SetPickupRights { rights: 1 }).unwrap_err(),
            Refusal::NotInAParty
        );
    }

    // --- leave / expel / disband --------------------------------------------------------

    #[test]
    fn a_member_leaving_leaves_the_party_standing() {
        let (mut p, id) = party_of(3);
        let out = p.apply(NOW, B, Request::Leave).unwrap();
        assert_eq!(
            out,
            vec![Effect::Departed {
                party: id,
                who: B,
                how: Departure::Voluntary,
                remaining: vec![A, C],
            }]
        );
        assert_eq!(p.party(id).unwrap().members, vec![A, C]);
        assert_eq!(p.party_id_of(B), None);
        assert_eq!(p.audience(B).as_slice(), &[B]);
        assert_eq!(p.audience(A).as_slice(), &[A, C]);
        p.check_invariants().unwrap();
    }

    #[test]
    fn the_leader_leaving_disbands_the_party_for_everyone() {
        // `0x0114` "You have quit as the leader of the party. The party has been disbanded."
        // and `0x0115` "You have left the party since the party leader quit." **[L]**
        let (mut p, id) = party_of(3);
        let out = p.apply(NOW, A, Request::Leave).unwrap();
        assert_eq!(
            out,
            vec![
                Effect::Departed {
                    party: id,
                    who: A,
                    how: Departure::Disbanded,
                    remaining: vec![B, C],
                },
                Effect::Departed {
                    party: id,
                    who: B,
                    how: Departure::Disbanded,
                    remaining: vec![C],
                },
                Effect::Departed {
                    party: id,
                    who: C,
                    how: Departure::Disbanded,
                    remaining: vec![],
                },
                Effect::Disbanded { party: id, members: vec![A, B, C] },
            ]
        );
        assert!(p.is_empty());
        for who in [A, B, C] {
            assert_eq!(p.party_id_of(who), None);
            assert_eq!(p.audience(who).as_slice(), &[who]);
        }
        p.check_invariants().unwrap();
    }

    #[test]
    fn the_last_member_leaving_disbands_the_party() {
        let (mut p, id) = party_of(1);
        let out = p.apply(NOW, A, Request::Leave).unwrap();
        assert_eq!(out.len(), 2, "one Departed, one Disbanded");
        assert!(matches!(out[0], Effect::Departed { who: A, .. }));
        assert_eq!(out[1], Effect::Disbanded { party: id, members: vec![A] });
        assert!(p.is_empty());
        p.check_invariants().unwrap();
    }

    #[test]
    fn leaving_when_you_are_in_no_party_is_refused() {
        let mut p = Parties::new();
        assert_eq!(p.apply(NOW, A, Request::Leave).unwrap_err(), Refusal::NotInAParty);
    }

    #[test]
    fn only_the_leader_may_expel_and_only_a_member() {
        let (mut p, _) = party_of(3);
        assert_eq!(
            p.apply(NOW, B, Request::Expel { target: C }).unwrap_err(),
            Refusal::NotTheLeader
        );
        assert_eq!(
            p.apply(NOW, A, Request::Expel { target: D }).unwrap_err(),
            Refusal::NotAMember
        );
        // Expelling yourself is refused rather than routed to Leave: a leader clicking
        // "expel" on their own row must not silently disband the party.
        assert_eq!(
            p.apply(NOW, A, Request::Expel { target: A }).unwrap_err(),
            Refusal::NotAMember
        );
        assert_eq!(p.party_of(A).unwrap().members, vec![A, B, C]);
    }

    #[test]
    fn expel_removes_the_target_and_says_how() {
        let (mut p, id) = party_of(3);
        let out = p.apply(NOW, A, Request::Expel { target: C }).unwrap();
        assert_eq!(
            out,
            vec![Effect::Departed {
                party: id,
                who: C,
                how: Departure::Expelled,
                remaining: vec![A, B],
            }]
        );
        assert_eq!(p.audience(C).as_slice(), &[C]);
        p.check_invariants().unwrap();
    }

    // --- leadership ---------------------------------------------------------------------

    /// **A disconnecting leader hands the crown to the session's pick and keeps their seat;
    /// with no pick the party ends; a bad pick counts as none.** The owner, 2026-09-18: only the
    /// leader is handed off, the disconnected client stays in the party, and a party whose
    /// crown cannot go to an online player is disbanded.
    #[test]
    fn a_disconnecting_leader_hands_over_and_stays_or_the_party_ends() {
        let (mut p, id) = party_of(3);
        let members = p.party(id).unwrap().members.clone();
        let (leader, second, third) = (members[0], members[1], members[2]);
        // The chosen successor leads; the old leader is STILL a member.
        let effects = p.apply(0, leader, Request::Disconnect { successor: Some(third) }).unwrap();
        assert_eq!(effects, vec![Effect::LeaderChanged { party: id, from: leader, to: third }]);
        assert_eq!(p.party(id).unwrap().leader, third);
        assert_eq!(p.party(id).unwrap().members, vec![leader, second, third], "nobody left");
        p.check_invariants().unwrap();
        // A successor who is not a member, or is the leader themself, is no successor: disband.
        let mut q = p.clone();
        let effects = q.apply(0, third, Request::Disconnect { successor: Some(9_999) }).unwrap();
        assert!(effects.iter().any(|e| matches!(e, Effect::Disbanded { party, .. } if *party == id)), "{effects:?}");
        assert!(q.party(id).is_none());
        q.check_invariants().unwrap();
        let mut q = p.clone();
        assert!(q.apply(0, third, Request::Disconnect { successor: Some(third) }).unwrap().iter().any(|e| matches!(e, Effect::Disbanded { .. })));
        // No successor at all: disband, the leader named first, everyone Departed.
        let effects = p.apply(0, third, Request::Disconnect { successor: None }).unwrap();
        let departed: Vec<CharacterId> = effects.iter().filter_map(|e| if let Effect::Departed { who, how: Departure::Disbanded, .. } = e { Some(*who) } else { None }).collect();
        assert_eq!(departed[0], third, "the leader first");
        assert_eq!(departed.len(), 3);
        assert!(p.party(id).is_none());
        assert!(p.apply(0, 9_999, Request::Disconnect { successor: None }).unwrap().is_empty(), "a stranger changes nothing");
        p.check_invariants().unwrap();
    }

    /// A member's disconnect changes nothing - no departure, no crown - whatever `successor`
    /// says; the party persists with every seat filled, even with everyone offline.
    #[test]
    fn a_disconnecting_member_stays_and_nothing_moves() {
        let (mut p, id) = party_of(3);
        let members = p.party(id).unwrap().members.clone();
        let (leader, second, third) = (members[0], members[1], members[2]);
        assert!(p.apply(0, second, Request::Disconnect { successor: Some(third) }).unwrap().is_empty());
        assert!(p.apply(0, third, Request::Disconnect { successor: None }).unwrap().is_empty());
        assert_eq!(p.party(id).unwrap().leader, leader);
        assert_eq!(p.party(id).unwrap().members, vec![leader, second, third]);
        p.check_invariants().unwrap();
    }

    #[test]
    fn change_leader_moves_the_leadership_and_keeps_everyone() {
        let (mut p, id) = party_of(3);
        let out = p.apply(NOW, A, Request::ChangeLeader { target: C }).unwrap();
        assert_eq!(out, vec![Effect::LeaderChanged { party: id, from: A, to: C }]);
        assert!(p.is_leader(C));
        assert!(!p.is_leader(A));
        assert_eq!(p.party(id).unwrap().members, vec![A, B, C]);
        p.check_invariants().unwrap();
    }

    #[test]
    fn change_leader_is_refused_for_a_non_leader_a_non_member_and_yourself() {
        let (mut p, _) = party_of(3);
        assert_eq!(
            p.apply(NOW, B, Request::ChangeLeader { target: C }).unwrap_err(),
            Refusal::NotTheLeader
        );
        assert_eq!(
            p.apply(NOW, A, Request::ChangeLeader { target: D }).unwrap_err(),
            Refusal::NotAMember
        );
        assert_eq!(
            p.apply(NOW, A, Request::ChangeLeader { target: A }).unwrap_err(),
            Refusal::NotAMember
        );
        assert!(p.is_leader(A));
    }

    #[test]
    fn after_handing_over_the_new_leader_can_invite_and_the_old_one_cannot() {
        let (mut p, _) = party_of(2);
        p.apply(NOW, A, Request::ChangeLeader { target: B }).unwrap();
        assert_eq!(
            p.apply(NOW, A, Request::Invite { target: C }).unwrap_err(),
            Refusal::NotTheLeader
        );
        assert!(p.apply(NOW, B, Request::Invite { target: C }).is_ok());
    }

    // --- disconnect ---------------------------------------------------------------------

    #[test]
    fn a_member_disconnecting_keeps_their_seat() {
        let (mut p, id) = party_of(3);
        assert!(p.disconnect(B).is_empty());
        assert!(p.is_leader(A));
        assert_eq!(p.party(id).unwrap().members, vec![A, B, C]);
        p.check_invariants().unwrap();
    }

    /// The no-successor form: the session found nobody online to take the crown, so the
    /// party ends, the leader named first. (With a successor: `disconnect_to`, tested above.)
    #[test]
    fn the_leader_disconnecting_with_no_successor_disbands() {
        let (mut p, id) = party_of(3);
        let out = p.disconnect(A);
        assert!(matches!(out[0], Effect::Departed { who: A, how: Departure::Disbanded, .. }), "{out:?}");
        assert!(matches!(out.last(), Some(Effect::Disbanded { party, .. }) if *party == id));
        assert!(p.is_empty());
        p.check_invariants().unwrap();
    }

    #[test]
    fn a_lone_leader_disconnecting_disbands_and_does_not_promote_a_ghost() {
        let (mut p, id) = party_of(1);
        let out = p.disconnect(A);
        assert_eq!(out.len(), 2);
        assert!(matches!(out[0], Effect::Departed { who: A, .. }));
        assert_eq!(out[1], Effect::Disbanded { party: id, members: vec![A] });
        assert!(p.is_empty());
        p.check_invariants().unwrap();
    }

    #[test]
    fn disconnecting_a_character_in_no_party_does_nothing_and_does_not_panic() {
        let mut p = Parties::new();
        assert!(p.disconnect(A).is_empty());
        assert!(p.disconnect(u32::MAX).is_empty());
    }

    #[test]
    fn disconnecting_drops_the_invites_that_character_held() {
        let (mut p, id) = party_of(1);
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        let out = p.disconnect(B);
        assert_eq!(out, vec![Effect::InviteDropped { party: id, target: B, reason: DeclineReason::Refused }]);
        assert!(!p.has_invite(id, B));
        p.check_invariants().unwrap();
    }

    #[test]
    fn disbanding_a_party_clears_the_invites_it_had_outstanding() {
        let (mut p, id) = party_of(2);
        p.apply(NOW, A, Request::Invite { target: C }).unwrap();
        assert!(p.has_invite(id, C));
        p.apply(NOW, A, Request::Leave).unwrap();
        assert!(!p.has_invite(id, C));
        assert!(p.invites_for(C).is_empty());
        p.check_invariants().unwrap();
    }

    // --- the wire contract --------------------------------------------------------------

    #[test]
    fn every_refusal_maps_to_a_result_code_the_client_can_take_with_an_empty_body() {
        // This is the "always answer" rule, checked. A refusal whose code needs fields
        // would leave the caller with nothing safe to send.
        for r in [
            Refusal::AlreadyInAParty,
            Refusal::NotInAParty,
            Refusal::NotTheLeader,
            Refusal::PartyIsFull,
            Refusal::TargetIsYourself,
            Refusal::TargetAlreadyInAParty,
            Refusal::TargetAlreadyInvited,
            Refusal::NoSuchInvite,
            Refusal::NotAMember,
            Refusal::NameIsEmpty,
            Refusal::NoSuchParty,
        ] {
            let code = r.result_code();
            assert!(
                net::party::is_silent_code(code),
                "{r:?} maps to {code:#04x}, whose arm reads fields"
            );
            assert!(net::party::refusal(code).is_some(), "{r:?}");
            assert!(!r.message().is_empty());
        }
    }

    #[test]
    fn max_members_is_the_number_the_fixture_can_actually_reach() {
        // If MAX_MEMBERS is ever changed, this fails unless the machinery agrees with it -
        // rather than the constant and the code drifting apart quietly.
        let (p, id) = party_of(MAX_MEMBERS);
        assert_eq!(p.party(id).unwrap().size(), MAX_MEMBERS);
        assert!(p.party(id).unwrap().is_full());
        assert_eq!(p.audience(A).len(), MAX_MEMBERS);
    }

    // --- the property that matters most -------------------------------------------------

    #[test]
    fn no_sequence_of_requests_leaves_a_character_in_two_parties() {
        // A small deterministic walk over every transition, checking the invariants after
        // each step. Not a fuzzer - it is a fixed script, so a failure is reproducible.
        let mut p = Parties::new();
        let script: Vec<(CharacterId, Request)> = vec![
            (A, Request::Create { name: "A".into() }),
            (A, Request::Invite { target: B }),
            (B, Request::Accept { party: 1 }),
            (C, Request::Create { name: "C".into() }),
            (C, Request::Invite { target: B }),
            (B, Request::Accept { party: 2 }),
            (A, Request::ChangeLeader { target: B }),
            (A, Request::Leave),
            (B, Request::Leave),
            (C, Request::Invite { target: D }),
            (D, Request::Accept { party: 2 }),
            (C, Request::Expel { target: D }),
            (C, Request::Leave),
            (D, Request::Create { name: "D".into() }),
        ];
        for (actor, req) in script {
            let _ = p.apply(NOW, actor, req);
            p.check_invariants().expect("invariant broken");
            // The one thing that must never be true, checked after every single step.
            let mut seen: BTreeMap<CharacterId, PartyId> = BTreeMap::new();
            for (id, party) in &p.parties {
                for m in &party.members {
                    assert!(seen.insert(*m, *id).is_none(), "{m} is in two parties");
                }
            }
        }
    }
}

#[cfg(test)]
mod invite_expiry_tests {
    use super::*;

    const NOW: i64 = 1_000_000;
    const A: CharacterId = 200;
    const B: CharacterId = 201;

    /// A party with a leader, for the expiry tests. Actor A leads party `id`.
    fn led_party() -> (Parties, PartyId) {
        let mut p = Parties::new();
        let out = p.apply(NOW, A, Request::Create { name: "A's Party".into() }).unwrap();
        let id = match out[0] {
            Effect::Created { party, .. } => party,
            ref other => panic!("{other:?}"),
        };
        (p, id)
    }

    /// **The owner's timeout.** An invite left unanswered past the TTL lapses: the invitee can no
    /// longer accept it, and - the point - the leader can invite them again.
    #[test]
    fn an_unanswered_invite_lapses_and_frees_a_re_invite() {
        let (mut p, id) = led_party();
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        assert!(p.has_invite(id, B));

        // One second before the deadline it is still live: a re-invite is refused, an accept
        // would work.
        let almost = NOW + INVITE_TTL_SECS - 1;
        assert_eq!(
            p.apply(almost, A, Request::Invite { target: B }).unwrap_err(),
            Refusal::TargetAlreadyInvited,
        );

        // At the deadline it lapses. The re-invite now succeeds - the state this whole change
        // exists to unstick - and the fresh invite is stamped at the new time.
        let after = NOW + INVITE_TTL_SECS;
        let out = p.apply(after, A, Request::Invite { target: B }).unwrap();
        assert_eq!(out, vec![Effect::Invited { party: id, from: A, target: B }]);
        assert!(p.has_invite(id, B));
    }

    /// A lapsed invite cannot be accepted: it reads as `NoSuchInvite`, not a silent join.
    #[test]
    fn a_lapsed_invite_cannot_be_accepted() {
        let (mut p, id) = led_party();
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        assert_eq!(
            p.apply(NOW + INVITE_TTL_SECS, B, Request::Accept { party: id }).unwrap_err(),
            Refusal::NoSuchInvite,
        );
    }

    /// An accept that lands just before the deadline still joins - the generous TTL is so a
    /// late-but-valid click is honoured rather than silently refused.
    #[test]
    fn an_accept_just_before_the_deadline_still_joins() {
        let (mut p, id) = led_party();
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        let out = p.apply(NOW + INVITE_TTL_SECS - 1, B, Request::Accept { party: id }).unwrap();
        assert!(out.iter().any(|e| matches!(e, Effect::Joined { who, .. } if *who == B)));
    }

    /// **A lapse is not a decline.** Expiry emits no effect and tells nobody - the client
    /// faded its own dialog - whereas a decline returns `InviteDropped` so the leader hears
    /// "denied". `expire_invites` reports the dropped pair for the log and nothing else.
    #[test]
    fn expiry_is_silent_where_a_decline_speaks() {
        let (mut p, id) = led_party();
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        let dropped = p.expire_invites(NOW + INVITE_TTL_SECS);
        assert_eq!(dropped, vec![(id, B)], "reported for the log");
        assert!(!p.has_invite(id, B));
        // Re-invite and DECLINE instead: that one does produce an effect.
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        let out = p.apply(NOW, B, Request::Decline { party: id, reason: DeclineReason::Refused }).unwrap();
        assert_eq!(out, vec![Effect::InviteDropped { party: id, target: B, reason: DeclineReason::Refused }]);
    }

    /// Expiry leaves a still-live invite alone, and the invariant holds across it.
    #[test]
    fn expiry_spares_a_fresh_invite_and_keeps_the_invariant() {
        let (mut p, id) = led_party();
        p.apply(NOW, A, Request::Invite { target: B }).unwrap();
        assert!(p.expire_invites(NOW + INVITE_TTL_SECS - 1).is_empty());
        assert!(p.has_invite(id, B));
        p.check_invariants().unwrap();
    }
}
