//! Items lying on the ground: the field's drop table, its ids, its expiry, and pick-up.
//!
//! `crates/net/src/drops.rs` owns the **bytes** - `0x046E` DropEnterField, `0x046F`
//! DropLeaveField, and the fact that a `0x0107` with `dst == 0` is a drop rather than a move.
//! This module owns the **state**: which items are on which map's floor, who may take each
//! one, when it goes away, and what to send about all of that. Everything either file claims
//! about the client is written up in `research/item-drop.md`, tagged **[L]** for read off the
//! listing and **[I]** for inferred.
//!
//! # WIRED since 2026-08-19. This header used to say the opposite.
//!
//! It read *"Nothing calls any of this"*, and that was true on the day it was written.
//! `crate::fields` owns a `DropTable` per map, `session::combat` rolls drops on the death
//! branch, and `session::mod` routes the pick-up range. Drops arc out of corpses on screen.
//!
//! **Left as a correction rather than deleted**, because a stale "not wired" banner is its own
//! hazard: it invites the next reader to go and wire something twice.
//!
//! What unblocks each half:
//!
//! | | blocked on | |
//! |---|---|---|
//! | **dropping** (`0x0107` with `dst == 0` -> `0x0070` + `0x046E`) | **nothing.** Every packet it needs is built and tested | one edit in `Session::on_inventory_move`, which today refuses `dst == 0` with a notice |
//! | **picking up** (`?` -> `0x0070` + `0x046F`) | **the request opcode is not known** | one client run. `world.log` names it in one line |
//! | **field entry** (re-send the floor) | nothing | one call in `Session::on_field_entered` |
//! | **expiry** | nothing | one call in `Session::tick` |
//!
//! **The pick-up opcode is `0x032C`, measured 2026-08-20**, with the drop's object id at
//! body offset 13. It could never have been found statically - the send is behind a `jmp`
//! into `.themida`, whose `SizeOfRawData` is 0, so those bytes are not in the file at any
//! offset. One walk over one drop named it, exactly as `research/item-drop.md` said it would.
//! The six-candidate range below is history now; `PICK_UP_CANDIDATE_FIRST` survives only
//! because a stray sibling opcode should still be answered rather than ignored.
//!
//! **So this module never sees an opcode.** [`DropTable::take`] takes an object id and a
//! character id. When a run names the opcode, the wiring is one match arm that reads a `u32`
//! and calls it; nothing in here changes. [`may_be_the_pick_up_request`] exists so the
//! session can log a candidate loudly on the run that names it.
//!
//! # Always answer
//!
//! An unanswered packet freezes the client's **entire** UI - every button, including the quit
//! prompt - and reads on screen as a crash.
//!
//! `0x0107` is the worst case of it. `FUN_142cc5b00` sets `player+0x2330` to 1 the moment it
//! sends, **on a drop exactly as on a move** (`0x142cc5f01`), and only an inbound `0x0070`
//! clears it. A refusal that sends nothing does not fail one drag; it kills every inventory
//! action for the rest of the session. That happened on 2026-08-19.
//!
//! Every path through this module therefore hands the caller something to send:
//! [`DropTable::drop_item`] returns the `0x0070` **first** and the `0x046E` second,
//! [`drop_refused`] returns the `0x0070` refusal for every path that does not drop, and every
//! [`PickUp`] variant carries either a packet or a [`PickUp::notice`] line.
//!
//! The one hole is honest and is the same hole as above: **a pick-up refusal has no decoded
//! reply**, because the request has no decoded opcode. Whether an unanswered pick-up latches
//! anything is item 4 of `research/item-drop.md` § 8 - argued as "assume yes" because the
//! path runs through `CWvsContext`, the object that owns the `0x2330` latch, and *not* read.
//! Until the run, the caller must send the notice.
//!
//! # Object ids start at 20 000 000, and that is deliberate
//!
//! See [`FIRST_DROP_OBJECT_ID`].
//!
//! # Expiry is server policy, and the number is a guess
//!
//! See [`DROP_LIFETIME_MS`].
//!
//! # The untradeable rule: it gates the pick-up, not the drop
//!
//! The owner: *"Please do not allow untradeable items to be stored."* That rule is
//! `store::ItemRules::may_be_stored`, over the client's own `info/tradeBlock`.
//!
//! **The decision here is that a trade-blocked item may be dropped, and may only ever be
//! picked up by the character who dropped it** - see [`drop_is_locked_to_owner_forever`]. The
//! rule's purpose is that such an item cannot reach a different character; while it is on the
//! floor it has not reached anybody, and the moment it could is the pick-up. So the refusal
//! sits where the transfer would actually happen rather than where it would not.
//!
//! This is a **decision, not a measurement**. The client enforces neither half - `ownType` is
//! read into `drop+0x70` and never tested again (`research/item-drop.md` § 3, read 10) - so
//! the server is the only thing that can. The stricter reading ("an untradeable item may not
//! leave the bag at all") is one branch at the call site plus this function; nothing else
//! would change. And with one character able to be on a field at a time, the two readings are
//! currently **unobservable** on screen.
//!
//! # The position is the caller's, and the server does not have one
//!
//! `0x0107` carries no position, and this server tracks none: nothing parses `0x00D9`, the
//! client's own movement packet. Two real sources already exist and neither is a live walk
//! position - `net::combat::AttackRequest::x`/`y` (**[L]**, and the session already parses
//! them) and `TransferFieldRequest::position` on a portal walk.
//!
//! Until one is chosen, a drop lands wherever the caller says it does. That matters more than
//! it looks: the client's own pick-up sweep is a box of `x-0x19..x+0x19` by `y-0x32..y+0x0a`
//! around the **player**, so an item placed away from the player's feet is drawn and
//! unreachable - and on screen "nothing on the ground" and "on the ground somewhere else"
//! look identical. Name the source before the run or the run cannot discriminate.

use std::collections::BTreeMap;

use crate::Reply;

/// The first drop object id, and the reason it is nowhere near a small number.
///
/// `CLAUDE.md`: *"Never renumber characters from 1. A create reply carrying id 1 made the
/// client silently refuse a transition."* That was a **character** id and this is a drop-pool
/// key, so it is not the same field and nothing has shown this pool cares. It is treated as
/// though it does, because the failure would look the same - a silent nothing, with no fault
/// and no log line - and being far away from small numbers costs nothing at all.
///
/// Twenty million is clear of every other id space this project has:
///
/// | | |
/// |---|---|
/// | character ids | 200 and up (`store::FIRST_CHARACTER_ID`) |
/// | NPC object ids | 1000 and up (`crate::config::Config::load_npcs`) |
/// | mob object ids | 2000 and up, *"so that a map's mobs and its NPCs never collide even if the two pools turn out to share an id space"* |
/// | every item id in this client | up to **5 990 000** - counted in `gm-handbook/itemdata.txt`, which is generated from its own `Item.wz` |
/// | every mob template | up to **9 990 005** in `gm-handbook/mobtemplates.txt`, and 9 990 545 in the client's own special-case list (`net::mob::SPECIAL_TEMPLATE_IDS`) |
///
/// Eight digits also means a drop id in `world.log` cannot be misread as an item id, and that
/// is not cosmetic: **the run that names the pick-up opcode is read out of `world.log` by
/// eye**, and the packet whose opcode is unknown will carry one of these numbers in it.
///
/// # The one id property the client *does* have
///
/// A **duplicate** id inside one field makes `DropEnterField` look the id up at
/// `0x1417a3014`, jump to `0x1417a46cd` and **read nothing else at all** - the drop simply
/// never appears, silently. [`DropTable`] mints monotonically and never reuses an id, which is
/// the whole guard.
///
/// # The mob rule is NOT copied here, on purpose
///
/// `net::mob::OBJECT_ID_MULTIPLE_TO_AVOID` keeps mob ids off 0 and off multiples of 178.
/// That is a property of `FUN_141d33630`, the **mob** decoder. The drop pool's bucket walk at
/// `0x1417ad7fe` divides by `[pool+0x18]`, a bucket count chosen at runtime, so no constant is
/// special to it as far as anything read goes. Copying the mob rule would have produced a
/// number that looked measured and was not.
pub const FIRST_DROP_OBJECT_ID: u32 = 20_000_000;

/// How long a drop lives before the server takes it away. **Three minutes, and it is [I].**
///
/// Nothing in the client expires a drop on any path that was followed. `drop+0x1e8`,
/// `drop+0x1f0` and the `FUN_1403747c0` block are all timestamps *by shape* and none was
/// traced to a timer (`research/item-drop.md` § 8, item 6), and the expiry FILETIME this
/// server sends is deliberately **zero** - the value the client's own two short paths write
/// into `drop+0x158` themselves. So the client will hold a drop forever, and if the server
/// wants one gone it sends `0x046F` itself.
///
/// That makes the number policy rather than protocol - and **the owner set it to two minutes**
/// after watching drops on screen: *"should have a disappear timer of 2 minutes if not
/// picked up within that time period"*. It was three, which was a guess with nothing behind
/// it. This one is a decision by the person who has seen both this server and the real one.
///
/// [`DropTable::with_lifetime`] takes it as a parameter so a test does not have to wait.
pub const DROP_LIFETIME_MS: u64 = 120_000;

/// How long a drop belongs to the character who dropped it. **Fifteen seconds, and it is [I].**
///
/// After this it may be taken by anyone - which is how the games in this family behave, and
/// which is **not** measurable here: the client stores `ownType` at `drop+0x70` and never
/// tests it, so ownership is entirely the server's to enforce or not.
///
/// With one character able to be on a field at a time this window is currently unobservable.
/// It exists because the alternative - no ownership at all - would have to be re-derived the
/// first time a second player exists, and because [`drop_is_locked_to_owner_forever`] needs
/// somewhere to be the exception to.
pub const OWNER_LOCK_MS: u64 = 15_000;

/// The lowest opcode the player's pick-up request can have. **[L]** for the range.
///
/// The outbound pools run mob `0x02FF..~0x0323` and NPC `0x0327..0x0328`; the next claimed
/// opcode is `0x032F`. A disassembling sweep for every `mov edx,imm / call 0x1406ed520` in the
/// image finds **no** builder using `0x0329..0x032E`, and the drop pool sits between the NPC
/// pool and the reactor pool in every version of this enumeration. So the request is one of
/// those six, and its builder is inside the virtualised region.
///
/// The v214 reference's block layout puts it at `0x032C`. That is **[I]** from the instrument
/// `CLAUDE.md` scores at **1 of 8**, and **nothing here picks a value**.
/// **`0x032C` DropPickUpRequest - MEASURED 2026-08-20.**
///
/// The client walked over a drop and the server logged it. `research/item-drop.md` §1
/// predicted `0x032C` from the v214 reference, a source this project scores at 1 of 8 - it
/// was right, and it is now measured. One data point; not a reason to trust that source.
pub const CLIENT_DROP_PICK_UP: u16 = 0x032C;

/// Where the drop's object id sits in a [`CLIENT_DROP_PICK_UP`] body.
///
/// The whole body, from the measured capture cross-checked against the `0x00D9` and `0x00DF`
/// of the same session:
///
/// ```text
/// u8  0
/// u32 tick            milliseconds - six samples against the wall clock agree to ~10 ms
/// u32 0
/// i16 x               byte-identical to the last 0x00D9 path point
/// i16 y
/// u32 dropObjectId    <- offset 13, the only field the server needs
/// u32 0
/// u8  1
/// u32, u32, u32       one observed value each
/// ```
///
/// **[D]/[I], and it can never be [L]**: the builder is inside `.themida`, whose
/// `SizeOfRawData` is 0, so those bytes are not in the file at any offset. This is one of
/// the few things in this repo that static analysis is permanently unable to settle.
pub const PICK_UP_OBJECT_ID_AT: usize = 13;

/// The drop's object id out of a pick-up body, if it is long enough.
pub fn pick_up_object_id(body: &[u8]) -> Option<u32> {
    let at = PICK_UP_OBJECT_ID_AT;
    Some(u32::from_le_bytes(body.get(at..at + 4)?.try_into().ok()?))
}

pub const PICK_UP_CANDIDATE_FIRST: u16 = 0x0329;

/// The highest candidate. See [`PICK_UP_CANDIDATE_FIRST`].
pub const PICK_UP_CANDIDATE_LAST: u16 = 0x032E;

/// Could this inbound opcode be the player's pick-up request?
///
/// **This is a filter for a measurement, not an answer.** It is true for all six candidates
/// and false for everything else, so a session that logs it loudly turns the next run into the
/// one line of `world.log` that settles the question.
///
/// It is deliberately **false for `0x0301`**. That is a *mob* telling the server it walked
/// onto a drop (`net::drops::CLIENT_MOB_DROP_PICK_UP`), it carries a mob object id where a
/// character id would go, and it nearly shipped as the player's request.
pub fn may_be_the_pick_up_request(opcode: u16) -> bool {
    (PICK_UP_CANDIDATE_FIRST..=PICK_UP_CANDIDATE_LAST).contains(&opcode)
}

/// Is this item's owner the only character who may ever pick it up?
///
/// **The untradeable decision, in one function.** `info/tradeBlock` says the item may not
/// reach a different character; a drop only becomes a transfer at the pick-up, so that is
/// where the rule is enforced. See the module docs for the argument and for the stricter
/// reading if the owner wants it.
///
/// `store::ItemRules::trade_blocked` is the client's own table, generated by
/// `tools/gen_item_rules.py` out of `info/tradeBlock` - 39 ids, not a fan site.
pub fn drop_is_locked_to_owner_forever(item_id: u32) -> bool {
    store::ItemRules::trade_blocked(item_id)
}

/// Is the client asking for everything in the slot?
///
/// **This chooses which `0x0070` goes out, and it no longer chooses whether to refuse.**
/// Until 2026-09-09 a partial drop was rejected outright, on the stated grounds that mode 1
/// UpdateQuantity was not built. It *was* built - `net::inventory::inventory_quantity`, mode
/// read at `142d521fe`, **[L]** - and the guard outlived its own reason by long enough for
/// the refusal text to be the only thing still asserting it. The owner: *"I see that partial drop
/// is not implemented, I also need this implemented please."*
///
/// `true` means the slot empties and the reply is mode 3 REMOVE; `false` means some stay and
/// the reply is mode 1 carrying the remainder. Getting that backwards is the failure worth
/// naming: a mode 3 on a partial drop empties the slot on screen while the store still holds
/// the rest, and the player reads it as having lost them.
///
/// `requested` is `net::drops::drop_count`, which already turns the `-1` a non-bundle carries
/// into 1, and the caller clamps it to what is actually in the slot.
pub fn whole_slot_is_leaving(requested: u16, in_slot: u16) -> bool {
    requested >= in_slot
}

/// The `0x0070` that refuses a drop - and it is **still a reply**.
///
/// `nCount` is 0, so the client moves nothing, and `bExclRequestSent` is 1, which is what
/// clears `player+0x2330` and keeps the inventory UI alive. Identical bytes to
/// `Session::inventory_refused`; it is repeated here so that no path through this module's API
/// can end with the caller holding nothing to send.
pub fn drop_refused(m: &net::inventory::InventoryMove, why: &str) -> Vec<Reply> {
    vec![Reply {
        opcode: net::inventory::INVENTORY_OPERATION,
        body: net::inventory::inventory_rejected(),
        what: format!(
            "InventoryOperation: REFUSING the DROP of invType {} slot {} (count {}) with \
             nCount 0 - {why}. Nothing leaves the bag, and bExclRequestSent = 1 clears the \
             +0x2330 latch, without which every later inventory action is dropped before it \
             is built.",
            m.inv_type, m.src, m.count
        ),
    }]
}

// -------------------------------------------------------------------------------------
// What the server remembers about one item on the floor
// -------------------------------------------------------------------------------------

/// How far apart to place two drops from the same kill, in pixels.
///
/// **The owner, with a screenshot of the live server:** *"the items that drop should also be
/// slightly staggered from each other (if it drops 3 items, it should drop like the
/// screenshot from live server)"*. Three items on exactly the same pixel render as one.
///
/// Policy, `[I]` - the real server's spacing was not measured, only seen. Twenty pixels is
/// a little under an item icon's width, so a row of them overlaps slightly the way the
/// screenshot does rather than lining up like a fence.
pub const DROP_STAGGER_PX: i16 = 20;

/// One item lying on a map's floor.
///
/// The item is a `store::Item`, whole, rather than an id and a count. That is the fidelity
/// that makes a round trip honest: `ItemKind::Equip(Some(stats))` carries the item's **rolled**
/// per-item stats and `ItemKind::Bundle { quantity }` carries the real stack, so what goes back
/// into the bag on a pick-up is what came out of it. Flattening an equip to its id here would
/// be a silent scroll-eater, and `crates/store` went to some trouble - 26 nullable columns on
/// both the bag and the `equipment` table - so that it would not happen anywhere else either.
///
/// A `None` in the stats means *"nothing has ever modified this item, derive from the WZ
/// template"*, which is a different state from all-zero stats and must stay `None` across the
/// round trip. It does: this stores the `store::Item` unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiveDrop {
    /// The drop pool's hash key, minted by [`DropTable`]. Never 0, never reused.
    pub object_id: u32,
    /// Which map's floor. The pool is destroyed and rebuilt empty on every field entry, so a
    /// drop belongs to a field and has to be re-sent to anyone entering it.
    pub map_id: crate::fields::FieldKey,
    /// The item itself, with its per-item stats or its quantity.
    pub item: store::Item,
    /// Which bag it came out of, and which bag it goes back into.
    ///
    /// Stored rather than re-derived from the item id. `store::InventoryType::for_item` would
    /// usually agree, and if it ever did not, the item would come back in a different bag from
    /// the one it left - a bug that looks like the item vanishing.
    pub inv_type: store::InventoryType,
    /// The character who dropped it. See [`OWNER_LOCK_MS`] and
    /// [`drop_is_locked_to_owner_forever`].
    pub owner_id: u32,
    /// The party this drop belongs to, or `0` for none. The owner, 2026-09-05: *"All members of a
    /// party should see all drops killed by members of the party ... Once someone leaves the
    /// party, they can no longer pick up the party's drops unless they were the killer."* So
    /// a party drop may be taken by the `owner_id` (the killer) **or by anyone who is a member
    /// of this party at the moment of the pick-up** - [`LiveDrop::may_be_taken_by`] is given
    /// the live roster, so a member who has since left is no longer on it and loses the drop,
    /// while the killer keeps it through `owner_id`.
    pub party_id: u32,
    /// A drop anyone on the map may see and take. The owner: *"If a player drops an item on the
    /// ground, anyone in the map should be able to see it and pick it up."* Set on a player's
    /// own ground drop; **an untradeable item overrides it** and stays owner-only, because
    /// that rule gates the transfer and a public floor is still a transfer waiting to happen.
    pub public: bool,
    /// Where it is lying. See the module docs: the server does not track a player position,
    /// so this is exactly as good as whatever the caller passed.
    pub x: i16,
    /// See [`LiveDrop::x`].
    pub y: i16,
    /// Session milliseconds, from `Session::clock_ms`.
    ///
    /// **Not an `Instant`.** The session's whole clock is a `u64` of milliseconds passed into
    /// `tick`, precisely so that every exchange in the world server is a pure function of
    /// state and time and can be a unit test. A wall clock in here would make expiry the one
    /// thing that could not be tested without sleeping.
    /// The amount, when this is a **meso** drop rather than an item. `0` means an item.
    ///
    /// A meso drop still carries an `item`, and that item is **never read** - every consumer
    /// tests [`LiveDrop::is_meso`] first. It is a placeholder, not a claim that mesos are
    /// item id 0.
    pub meso: u32,
    pub dropped_at_ms: u64,
    /// Where the drop's arc **starts** - the mob's own position for a kill, and the item's
    /// resting place for anything a player put down by hand.
    ///
    /// The owner, 2026-08-20, watching a snail die: *"the item currently drop out too fast"*. It
    /// did, because this used to be the resting place in every case, so the arc had zero
    /// length and the icon simply appeared. See [`DROP_FLIGHT_MS`].
    pub source_x: i16,
    /// See [`LiveDrop::source_x`].
    pub source_y: i16,
    /// The `u32 delay` of `0x046E`'s source block, in milliseconds.
    pub delay_ms: u32,
    /// **Did a mob's death put this here?** The owner, 2026-09-13: every pet is a vacuum pet
    /// *"provided that they are from a mob death drop"*. True only for [`DropTable::drop_from_mob`]
    /// called with [`DropFromMob::from_mob`] set - the kill path; a reactor's drop, a player's
    /// ground drop and a coin drop are false. It is sent to the client as the drop's
    /// `canBePickedUpByPet` byte, so the pet does not try, and [`DropTable::take_by_pet`]
    /// refuses if it does anyway.
    pub from_mob: bool,
}

/// How long a mob's drop takes to arc from the corpse to where it lands.
///
/// The owner asked for *"like a 0.5 second delay from mob dying to item start to drop"*. This is
/// the client's own field - `0x046E`'s `u32` after `srcX`/`srcY`, read only for enter types
/// 0, 1 and 3 - rather than the server holding the packet back for half a second. The client
/// already knows how to animate the arc, and a server-side delay would need the tick to
/// release queued drops and would still leave the arc instantaneous.
///
/// **[I] that the field is a duration and not a start-delay.** Both readings are satisfied by
/// this number, so the run does not have to distinguish them to look right; if the drop still
/// snaps into place, this is the first thing to change.
pub const DROP_FLIGHT_MS: u32 = 500;

impl LiveDrop {
    /// The item template id.
    pub fn item_id(&self) -> u32 {
        self.item.item_id
    }

    /// How many are in this drop. Always 1 for an equip.
    ///
    /// **The wire never carries it.** `0x046E` has no quantity field at all - a stack of five
    /// is one icon on the floor - so the count exists only here until the pick-up puts it back
    /// in a bag.
    pub fn quantity(&self) -> u16 {
        self.item.kind.quantity()
    }

    /// When this drop stops existing, given the table's lifetime.
    pub fn expires_at_ms(&self, lifetime_ms: u64) -> u64 {
        self.dropped_at_ms.saturating_add(lifetime_ms)
    }

    /// May `character_id` take this drop at `now_ms`?
    ///
    /// Who may pick this drop up. `party_members` is the drop's party's **current** roster,
    /// resolved by the caller at pick-up time (empty for a solo or public drop).
    ///
    /// In order:
    /// * the owner always may - the killer, or the player who put it down;
    /// * a trade-blocked item is owner-only forever, and that **overrides** party and public -
    ///   the rule gates the transfer, and party or floor is still a transfer;
    /// * a public drop (a player's ground drop) may be taken by anyone;
    /// * a party drop may be taken by anyone on `party_members` now - so a member who left is
    ///   gone from that list and refused, while the killer still passes on `owner_id` above;
    /// * otherwise the 15-second owner lock applies and then it is free to anyone.
    pub fn may_be_taken_by(
        &self,
        character_id: u32,
        now_ms: u64,
        owner_lock_ms: u64,
        party_members: &[u32],
    ) -> bool {
        if character_id == self.owner_id {
            return true;
        }
        if drop_is_locked_to_owner_forever(self.item_id()) {
            return false;
        }
        if self.public {
            return true;
        }
        if self.party_id != 0 && party_members.contains(&character_id) {
            return true;
        }
        now_ms >= self.dropped_at_ms.saturating_add(owner_lock_ms)
    }

    /// The packet-layer view of this drop.
    ///
    /// `source_object_id` is the owner's character id, which is what makes the item fly out of
    /// the player's feet on [`net::drops::ENTER_FLOATING`].
    /// Is this a bag of coins rather than an item? See [`LiveDrop::meso`].
    pub fn is_meso(&self) -> bool {
        self.meso > 0
    }

    pub fn field_drop(&self) -> net::drops::FieldDrop {
        let base = if self.is_meso() {
            net::drops::FieldDrop::money(self.object_id, self.meso, self.owner_id, self.x, self.y)
        } else {
            net::drops::FieldDrop::item(self.object_id, self.item_id(), self.owner_id, self.x, self.y)
        };
        // The arc. Both constructors default the source to the resting place and the delay to
        // zero, which is right for an item a player set down and wrong for one a mob dropped.
        net::drops::FieldDrop {
            source_x: self.source_x,
            source_y: self.source_y,
            delay: self.delay_ms,
            pet_may_take: self.from_mob,
            ..base
        }
    }

    /// `0x046E`, as the reply that puts it on screen.
    ///
    /// `enter_type` must be [`net::drops::ENTER_FLOATING`] or [`net::drops::ENTER_INSTANT`]:
    /// `drop+0x61`, the gate every pick-up sweep in the client tests, is set for those two
    /// values and no others, so any other enter type produces a drop that is drawn and can
    /// never be collected.
    pub fn enter_reply(&self, enter_type: u8) -> Reply {
        let body = net::drops::drop_enter_field(&self.field_drop(), enter_type);
        Reply {
            opcode: net::drops::DROP_ENTER_FIELD,
            body,
            what: format!(
                "DropEnterField: object id {} = item {} x{} on map {} at ({}, {}), owner {}, \
                 enterType {enter_type}. The enter type is the pick-up gate (drop+0x61 is set \
                 for 1 and 2 only), and a REUSED object id would make the client read the id \
                 and stop.",
                self.object_id,
                self.item_id(),
                self.quantity(),
                self.map_id,
                self.x,
                self.y,
                self.owner_id
            ),
        }
    }
}

/// What a dead mob left behind, already rolled.
///
/// `meso > 0` makes it a bag of coins and `item` is then a placeholder that nothing reads -
/// see [`LiveDrop::meso`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropFromMob {
    /// The map the mob died on.
    pub map_id: crate::fields::FieldKey,
    /// The character who killed it, and therefore who owns the drop while the lock lasts.
    pub owner_id: u32,
    /// The item, or a placeholder when `meso > 0`.
    pub item: store::Item,
    /// Which bag it goes into when picked up. Derived from the item id, not sent on the wire.
    pub inv_type: store::InventoryType,
    /// The amount when this is mesos; `0` for an item drop.
    pub meso: u32,
    /// Where it lands. Staggered outward from the corpse when several drop at once.
    pub x: i16,
    /// See [`DropFromMob::x`].
    pub y: i16,
    /// The corpse itself - where the arc starts, **before** the stagger is applied.
    ///
    /// Separate from `x` on purpose: with both equal the arc has zero length and the icon
    /// appears at rest, which is what the owner saw.
    pub source_x: i16,
    /// See [`DropFromMob::source_x`].
    pub source_y: i16,
    /// Session milliseconds, for expiry.
    pub now_ms: u64,
    /// The killer's party, or `0`. When set, every current member of it may take the drop
    /// (and the caller shows it to them) - see [`LiveDrop::party_id`].
    pub party_id: u32,
    /// Whether a mob's death is what dropped this (the kill path) rather than a reactor.
    /// See [`LiveDrop::from_mob`].
    pub from_mob: bool,
}

/// The result of a player dropping an item on the floor: the inventory `0x0070` for the
/// dropper, and the `0x046E` that puts it on the ground.
///
/// They go to different audiences and that is the whole reason this is not a `Vec`: the
/// `removed` clears the dropper's own inventory latch and is theirs alone, while the `enter`
/// is shown to **everyone on the map**, because a player's ground drop is public (the owner,
/// 2026-09-05). The session sends `removed` to itself and broadcasts `enter`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedDrop {
    pub object_id: u32,
    pub removed: Reply,
    pub enter: Reply,
}

/// The result of a player dropping **mesos**: just the `0x046E`.
///
/// Deliberately a different type from [`PlacedDrop`] rather than one with an `Option`. A bag
/// drop's `removed` is not optional - forgetting it leaves the dropper's inventory latched -
/// and a meso drop has no bag slot to remove from at all. Two shapes, so a call site cannot
/// silently take the wrong path, and the balance packet the caller must still send is named
/// in [`DropTable::drop_money`]'s docs rather than left to be remembered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacedMoney {
    pub object_id: u32,
    pub enter: Reply,
}

/// One accepted meso drop, as the caller describes it.
///
/// **The mesos are already gone from the character when this is called**, the same division
/// [`DropFromBag`] uses: the store write belongs to the session, and this module has no
/// database access, which is what lets its tests run without one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropMoneyOnGround {
    /// The map the character is standing on.
    pub map_id: crate::fields::FieldKey,
    /// Who dropped it. Becomes the owner and the `sourceObjectId`.
    pub character_id: u32,
    /// How many mesos. **Always positive** - the call site refuses zero and negative, because
    /// `0x0143`'s amount is a signed `i32` that the client's own check lets through negative.
    pub meso: u32,
    /// Where it comes to rest, already resolved onto a foothold by the caller.
    pub x: i16,
    pub y: i16,
    /// **Where the player actually was** when they dropped it, before the foothold snap.
    ///
    /// See [`arc_from`]. Equal to `(x, y)` for someone standing on the ground, and higher up
    /// for someone on a ladder or in mid-air - which is the case the owner reported.
    pub from_x: i16,
    /// See [`DropMoneyOnGround::from_x`].
    pub from_y: i16,
    pub now_ms: u64,
}

/// The arc a hand-made drop travels: from where the player is to where it comes to rest.
///
/// The owner, 2026-09-09: *"Whenever the character is on a ladder, the mesos drop location is
/// incorrect. It doesn't come out of the player's current ladder location, but more closer to
/// the ground."*
///
/// Both hand-drop paths resolved the player's position onto the foothold **below** them and
/// then used that one point as the resting place *and* as the arc's source. That snap is
/// deliberate and stays - a drop left hanging where a jumping player happened to be is drawn
/// outside the client's pick-up box and cannot be collected. What was wrong is using the
/// snapped point as the **source**: it gives the arc zero length, so the coins are simply
/// drawn on the ground and never appear to leave the player.
///
/// The client already knows how to animate this - it is the same `srcX`/`srcY`/`delay` block
/// a mob's drop uses, and the reason a snail's loot arcs out of the corpse instead of
/// appearing. So: **source is where the player is, resting place is the ground beneath.**
///
/// Returns the source and the delay. A player standing on the ground gets `(x, y)` and `0`,
/// which is byte-for-byte what this sent before, so the common case is unchanged.
pub fn arc_from(from: (i16, i16), rest: (i16, i16)) -> ((i16, i16), u32) {
    if from == rest {
        (rest, 0)
    } else {
        (from, DROP_FLIGHT_MS)
    }
}

/// One accepted drop, as the caller describes it.
///
/// **The item is already out of the bag when this is called.** The store write belongs to the
/// session, which owns the `Store` handle and the transaction, exactly as
/// `research/item-drop.md` § 7.1 sketches it: take it out, then record it here. This module
/// deliberately has no database access, which is also what lets every test in it run without
/// one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropFromBag {
    /// The map the character is standing on.
    pub map_id: crate::fields::FieldKey,
    /// Who dropped it. Becomes the owner and the `sourceObjectId`.
    pub character_id: u32,
    /// Which bag it came out of.
    pub inv_type: store::InventoryType,
    /// The 1-based slot it came out of, for the `0x0070` mode 3.
    ///
    /// Must be positive: mode 3 on a negative slot makes the body's length depend on
    /// client-side state the server cannot see (`net::inventory::inventory_removed`). Dragging
    /// straight off the character - `src < 0`, `dst == 0` - is refused at the call site,
    /// because that path has never been tested.
    pub slot: u16,
    /// The item, exactly as the store handed it back.
    pub item: store::Item,
    /// **Where the player actually was** when they dropped it, before the foothold snap.
    /// See [`arc_from`] - this is what makes the item fall from a ladder rather than appear
    /// on the ground.
    pub from_x: i16,
    /// See [`DropFromBag::from_x`].
    pub from_y: i16,
    /// **How many are still in that slot afterwards.** `None` means the slot is now empty.
    ///
    /// This is what makes a partial drop possible, and it decides which `0x0070` goes out:
    /// `None` is mode 3 REMOVE, `Some(n)` is mode 1 UPDATE QUANTITY carrying `n`. Sending a
    /// mode 3 when the server kept some would empty the slot on screen while the store still
    /// held items - a desync the player reads as losing them.
    ///
    /// `Some(0)` is not a legal way to say "empty": the client's mode 1 draws the number it is
    /// given, and a zero would leave a slot showing 0 rather than clearing. A debug assertion
    /// in [`DropPool::drop_item`] catches it.
    pub remaining_in_slot: Option<u16>,
    /// Where it lands. See the module docs - the server has no player position today.
    pub x: i16,
    /// See [`DropFromBag::x`].
    pub y: i16,
    /// Session milliseconds.
    pub now_ms: u64,
}

/// What happened when someone tried to pick a drop up.
///
/// **Every variant needs an answer on the wire**, and only two of them can supply one: the
/// request's own opcode is not decoded, so there is no "pick-up refused" packet to build. Use
/// [`PickUp::replies`] for what *is* known and [`PickUp::notice`] for the rest, and read the
/// module docs on why a pick-up that goes unanswered may or may not latch the UI.
///
/// `Taken` is much larger than the other variants because it carries a whole [`LiveDrop`],
/// and boxing it would put an allocation on the one path that always succeeds. One of these
/// exists at a time, on the stack, for the length of one pick-up.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PickUp {
    /// It is theirs. Put `drop.item` into `drop.inv_type` and send `leave` to the field.
    ///
    /// The drop is already out of the table. If putting it in the bag then fails - a full
    /// bag, a database error - the caller must **not** send `leave`, and should put the drop
    /// back with [`DropTable::restore`]; a leave for a drop that is still there is how an item
    /// disappears from the world entirely.
    Taken {
        /// Everything needed to put it back in a bag: the item with its stats or its
        /// quantity, and which bag it came from.
        drop: LiveDrop,
        /// `0x046F` leaveType 2, carrying the picking character's id, which is what plays the
        /// "flies into the character" animation.
        leave: Reply,
    },
    /// No drop by that id. Someone else took it, it expired, or the id was never ours.
    Unknown {
        /// What was asked for.
        object_id: u32,
    },
    /// It is somebody else's for another `opens_in_ms`.
    NotYours {
        /// What was asked for.
        object_id: u32,
        /// Whose it is.
        owner_id: u32,
        /// How long until anyone may take it.
        opens_in_ms: u64,
    },
    /// Trade-blocked, and this is not the owner. It never opens up. See the module docs.
    Untradeable {
        /// What was asked for.
        object_id: u32,
        /// The only character who may take it.
        owner_id: u32,
    },
    /// A pet asked, and the drop is not a mob's. The drop went out with `canBePickedUpByPet`
    /// clear, so a well-behaved client never sends this; the answer is the unlock alone, with
    /// no line on screen - a pet brushing past a coin should not be a chat message each time.
    NotForPets {
        /// What was asked for.
        object_id: u32,
    },
    /// It was past its lifetime. It has been swept, and `leave` says so.
    Expired {
        /// What was asked for.
        object_id: u32,
        /// `0x046F` leaveType 0 - it fades rather than flying into anyone.
        leave: Reply,
    },
}

impl PickUp {
    /// The drop, if it was taken. This is what goes back in the bag.
    pub fn taken(&self) -> Option<&LiveDrop> {
        match self {
            PickUp::Taken { drop, .. } => Some(drop),
            _ => None,
        }
    }

    /// The packets this outcome produces, for the field.
    ///
    /// Empty for the three refusals, because **no refusal packet exists to build** - see the
    /// type's own docs. Those three carry a [`PickUp::notice`] instead.
    pub fn replies(&self) -> Vec<Reply> {
        match self {
            PickUp::Taken { leave, .. } => vec![leave.clone()],
            PickUp::Expired { leave, .. } => vec![leave.clone()],
            _ => Vec::new(),
        }
    }

    /// One line for the player, for the outcomes that have no packet of their own.
    ///
    /// `None` when [`PickUp::replies`] already answers. Otherwise the caller should send it as
    /// a chat notice (`Session::notice`) - it is not the reply the client is waiting for,
    /// because that reply's opcode is unknown, but it is *something*, and silence is the one
    /// answer this project has proven costs a session.
    pub fn notice(&self) -> Option<String> {
        match self {
            PickUp::Taken { .. } => None,
            PickUp::Unknown { .. } => Some("That item is no longer there.".to_string()),
            PickUp::NotYours { opens_in_ms, .. } => Some(format!(
                "That item belongs to someone else for another {} seconds.",
                opens_in_ms.div_ceil(1000)
            )),
            PickUp::Untradeable { .. } => {
                Some("That item cannot be picked up by anyone but its owner.".to_string())
            }
            PickUp::NotForPets { .. } => None,
            PickUp::Expired { .. } => Some("That item is no longer there.".to_string()),
        }
    }

    /// A refusal that owes the client the unlock and nothing to read: the pet case.
    pub fn is_silent_refusal(&self) -> bool {
        matches!(self, PickUp::NotForPets { .. })
    }

    /// A log line naming the outcome, for `world.log`.
    pub fn what(&self) -> String {
        match self {
            PickUp::Taken { drop, .. } => format!(
                "pick-up: character took drop {} = item {} x{}",
                drop.object_id,
                drop.item_id(),
                drop.quantity()
            ),
            PickUp::Unknown { object_id } => {
                format!("pick-up: drop {object_id} is not on this server's floor")
            }
            PickUp::NotYours { object_id, owner_id, opens_in_ms } => format!(
                "pick-up: drop {object_id} belongs to character {owner_id} for another {opens_in_ms} ms"
            ),
            PickUp::Untradeable { object_id, owner_id } => format!(
                "pick-up: drop {object_id} is trade-blocked, so only character {owner_id} may take it"
            ),
            PickUp::NotForPets { object_id } => format!(
                "pick-up: drop {object_id} did not come from a mob, so a pet may not take it"
            ),
            PickUp::Expired { object_id, .. } => {
                format!("pick-up: drop {object_id} had already expired and has been swept")
            }
        }
    }
}

// -------------------------------------------------------------------------------------
// The table
// -------------------------------------------------------------------------------------

/// Every drop this server has on the floor, on every map.
///
/// # One table across maps rather than one per field
///
/// The client's pool is per field - destroyed and rebuilt empty on every field entry - so a
/// drop is re-sent to whoever enters, and [`DropTable::field_entry`] is the query that does
/// it. Keeping every map in one table is what makes an item still be there when the player
/// walks back through the portal, and it makes the id counter monotonic across the whole
/// server rather than per field, which is strictly stronger than the uniqueness the pool
/// actually requires.
///
/// Keyed by object id because that is what the client addresses a drop by, and a `BTreeMap`
/// rather than a `HashMap` so that iteration order is the id order - a test that pins packet
/// order stays pinned.
#[derive(Debug, Clone)]
pub struct DropTable {
    next_object_id: u32,
    lifetime_ms: u64,
    owner_lock_ms: u64,
    live: BTreeMap<u32, LiveDrop>,
    /// Packets this table has produced that are addressed to a **character**, not to
    /// whoever called. See [`Addressed`] and [`DropTable::take_addressed`].
    outbox: Vec<Addressed>,
}

/// **A finished packet with an address on it.**
///
/// The connection that produces a packet about a drop is often not who it is owed to. The
/// clearest case is expiry: every session on a map ticks, whichever ticks first removes the
/// drop from the shared table, and the fade is owed to **everyone who was sent the `0x046E`**
/// - the owner, their party, or the whole field for a public ground drop. The owner, 2026-09-18:
/// *"When a party loot expires for the client that killed the monster, other clients in the
/// party who share the visual for that drop do not see the expired drop disappear ... If
/// anyone drops items publicly and that item disappears, it should disappear for everyone
/// who can see it."* Until then the fade went to the owner alone, so a party member - or a
/// bystander looking at a public drop - kept drawing an item that no longer existed, and a
/// click on it was refused as unknown.
///
/// This module holds no bus and no session, deliberately - that is what makes every branch in
/// it a unit test. So it names the recipient and `crate::fields::Fields::with_drops` posts it,
/// which is the only place that holds both the table and the bus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Addressed {
    /// The map the packet is about, and **everyone on it** is the recipient
    /// (`Bus::publish_to_map`). Not optional: a `0x046E`/`0x046F` names a position in a
    /// field's own drop pool, so delivering one to a character who has walked away would put
    /// a phantom item on a map it was never dropped on.
    ///
    /// The audience of a drop is not recorded per drop - it is the owner, then the party as
    /// it stood when the drop landed, then anyone who walked in while it was public - and it
    /// does not need to be: a client whose pool never held the object id ignores a leave for
    /// it, exactly as it ignores a movement packet for a mob it was never shown. The pick-up
    /// leave in `session/ground.rs::take_leaves_to_field` goes to the whole field for the
    /// same reason.
    pub map_id: crate::fields::FieldKey,
    pub reply: Reply,
}

impl Default for DropTable {
    fn default() -> Self {
        Self::new()
    }
}

impl DropTable {
    /// An empty table with the default [`DROP_LIFETIME_MS`] and [`OWNER_LOCK_MS`].
    pub fn new() -> Self {
        Self::with_lifetime(DROP_LIFETIME_MS, OWNER_LOCK_MS)
    }

    /// An empty table with an explicit lifetime and owner lock, both in milliseconds.
    pub fn with_lifetime(lifetime_ms: u64, owner_lock_ms: u64) -> Self {
        DropTable {
            next_object_id: FIRST_DROP_OBJECT_ID,
            lifetime_ms,
            owner_lock_ms,
            live: BTreeMap::new(),
            outbox: Vec::new(),
        }
    }

    /// **Take everything this table has addressed to a character**, leaving it empty.
    ///
    /// Called by `crate::fields::Fields::with_drops` after every closure, because that is the
    /// only place holding both this table and the bus. A caller that forgets loses the
    /// packets - which is why there is exactly one caller and it is in a function every path
    /// already goes through, rather than a step each call site has to remember.
    pub fn take_addressed(&mut self) -> Vec<Addressed> {
        std::mem::take(&mut self.outbox)
    }

    /// How many packets are waiting for an owner. For tests and for a log line.
    pub fn addressed_len(&self) -> usize {
        self.outbox.len()
    }

    /// How long a drop lives here.
    pub fn lifetime_ms(&self) -> u64 {
        self.lifetime_ms
    }

    /// How long a drop belongs to whoever dropped it.
    pub fn owner_lock_ms(&self) -> u64 {
        self.owner_lock_ms
    }

    /// How many drops are on the floor, everywhere.
    pub fn len(&self) -> usize {
        self.live.len()
    }

    /// Is the floor empty everywhere?
    pub fn is_empty(&self) -> bool {
        self.live.is_empty()
    }

    /// One drop by its object id.
    pub fn get(&self, object_id: u32) -> Option<&LiveDrop> {
        self.live.get(&object_id)
    }

    /// Every drop on one map, in id order.
    pub fn on_field(&self, map_id: crate::fields::FieldKey) -> impl Iterator<Item = &LiveDrop> {
        self.live.values().filter(move |d| d.map_id == map_id)
    }

    /// The id the next drop will get. For a log line, and for a test.
    pub fn next_object_id(&self) -> u32 {
        self.next_object_id
    }

    /// Continue the id sequence from `next` instead of [`FIRST_DROP_OBJECT_ID`].
    ///
    /// Exists so a table rebuilt mid-session - or one day, restored across a restart - does
    /// not hand out an id it has already used. Values below [`FIRST_DROP_OBJECT_ID`] are
    /// raised to it rather than accepted: an id of 1 is the failure this whole numbering
    /// exists to avoid.
    pub fn resume_ids_from(&mut self, next: u32) {
        self.next_object_id = next.max(FIRST_DROP_OBJECT_ID);
    }

    /// Mint the next id: monotonic, never 0, never one that is currently live.
    ///
    /// The wrap at `u32::MAX` is four billion drops away and is handled anyway, because the
    /// consequence of getting it wrong is the silent one: a duplicate id makes
    /// `DropEnterField` read the id and stop, so the item never appears and nothing is logged.
    fn mint_object_id(&mut self) -> u32 {
        loop {
            let id = self.next_object_id;
            self.next_object_id = match id.checked_add(1) {
                Some(next) => next,
                None => FIRST_DROP_OBJECT_ID,
            };
            if id >= FIRST_DROP_OBJECT_ID && !self.live.contains_key(&id) {
                return id;
            }
        }
    }

    /// **Put an item on the ground**, and build the whole answer to the `0x0107`.
    ///
    /// The caller has already taken the item out of the bag; see [`DropFromBag`].
    ///
    /// # The order of the two replies is not cosmetic
    ///
    /// The `0x0070` goes **first**. It is what clears `player+0x2330`, the latch
    /// `FUN_142cc5b00` sets on every send, and until it arrives the client refuses to build
    /// the player's next inventory request at all. The `0x046E` follows and is one-way - the
    /// client sends nothing back for it, so a drop appearing is not evidence that anything
    /// else works.
    ///
    /// Mode 3 Remove carries **no tail bytes**, and no `avatarChanged` byte is appended: for
    /// mode 3 that flag depends on client-side state the server cannot see
    /// (`research/msexe-setfield.md`), which is why the slot must be a positive bag slot.
    /// A mob died and left something on the floor.
    ///
    /// **Not [`DropTable::drop_item`], and the difference is the inventory packet.** A bag
    /// drop must send `0x0070` first, because that reply is what clears the client's
    /// `+0x2330` latch; a mob drop never touched the bag, so sending one would refuse an
    /// inventory request the player never made. This returns the `0x046E` alone.
    ///
    /// Takes an already-rolled result rather than a table, so the randomness stays in
    /// `crate::droptables` and everything here remains deterministic and testable.
    ///
    /// `owner_id` is the character who landed the killing blow: it is who the drop belongs to
    /// for [`OWNER_LOCK_MS`], which is the only reason a mob drop needs an owner at all.
    ///
    /// Returns the object id as well as the packet, because the caller may have to
    /// **re-address** it: the drop belongs to the top damager, who is often not the killer
    /// and may not be on this map at all. See [`DropTable::readdress`] and
    /// `crate::mobshare::drop_audience`.
    pub fn drop_from_mob(&mut self, d: DropFromMob) -> (u32, Reply) {
        let object_id = self.mint_object_id();
        let drop = LiveDrop {
            object_id,
            map_id: d.map_id,
            item: d.item,
            inv_type: d.inv_type,
            owner_id: d.owner_id,
            party_id: d.party_id,
            public: false,
            x: d.x,
            y: d.y,
            meso: d.meso,
            dropped_at_ms: d.now_ms,
            // The arc starts on the corpse and ends where the stagger put the item, over
            // DROP_FLIGHT_MS. Before this both ends were the same point and the icon simply
            // appeared - the owner: "the item currently drop out too fast".
            source_x: d.source_x,
            source_y: d.source_y,
            delay_ms: DROP_FLIGHT_MS,
            from_mob: d.from_mob,
        };
        let enter = drop.enter_reply(net::drops::ENTER_FLOATING);
        self.live.insert(object_id, drop);
        (object_id, enter)
    }

    /// **Give an already-minted drop to somebody else**, and rebuild its enter packet.
    ///
    /// `None` if there is no such drop.
    ///
    /// Exists because "who owns this" is decided by a walk the table cannot do: the owner's rule
    /// is *"the one who dealt the most damage (without counting over-damage) will see the
    /// drops"*, the ranking comes from `LiveMob::shares()`, and whether a given candidate is
    /// still on this map is a question only `crate::broadcast::Bus` can answer - by
    /// attempting the delivery. So the caller mints the drop, then walks the ranking,
    /// re-addressing until one lands. `crate::mobshare::drop_audience` is the ranking and
    /// `session/combat.rs::drops_from_kill_for` is the walk.
    ///
    /// The **object id does not change**, so a re-address before the first delivery is
    /// invisible to every client: nothing has been sent about this drop yet.
    ///
    /// `owner_id` is also what [`LiveDrop::may_be_taken_by`] enforces, so this moves the
    /// pick-up right along with the visibility rather than leaving the two disagreeing.
    pub fn readdress(&mut self, object_id: u32, owner_id: u32) -> Option<Reply> {
        let drop = self.live.get_mut(&object_id)?;
        drop.owner_id = owner_id;
        Some(drop.enter_reply(net::drops::ENTER_FLOATING))
    }

    pub fn drop_item(&mut self, d: DropFromBag) -> PlacedDrop {
        debug_assert!(
            d.slot >= 1,
            "mode 3 needs a positive bag slot; an equipped-slot drop is refused at the call site"
        );
        let object_id = self.mint_object_id();
        let (source, delay) = arc_from((d.from_x, d.from_y), (d.x, d.y));
        let drop = LiveDrop {
            object_id,
            map_id: d.map_id,
            item: d.item,
            inv_type: d.inv_type,
            owner_id: d.character_id,
            party_id: 0,
            // A player's own ground drop is public: anyone on the map may see and take it.
            // An untradeable item overrides this in `may_be_taken_by` and stays owner-only.
            public: true,
            x: d.x,
            y: d.y,
            meso: 0, // a bag drop is always an item
            dropped_at_ms: d.now_ms,
            // A player standing on the ground has no arc, and this returns exactly what it
            // used to for them. One on a LADDER does: see `arc_from`.
            source_x: source.0,
            source_y: source.1,
            delay_ms: delay,
            from_mob: false,
        };
        debug_assert!(
            d.remaining_in_slot != Some(0),
            "an emptied slot is None, not Some(0) - mode 1 would draw a slot holding 0"
        );
        let inv_type = d.inv_type.as_u8() as i8;
        // **Which `0x0070` depends on whether anything is left.** Both carry
        // `bExclRequestSent = 1`, which is the byte that clears the client's `+0x2330` latch;
        // whichever one goes out, it must go out, or every later inventory action is dropped
        // before it is built.
        let removed = match d.remaining_in_slot {
            None => Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_removed(inv_type, d.slot as i16),
                what: format!(
                    "InventoryOperation REMOVE: item {} x{} leaves {:?} slot {} for the floor of \
                     map {}. This goes FIRST: bExclRequestSent = 1 is what clears the client's \
                     +0x2330 latch, and until it lands every later inventory action is dropped \
                     before it is built.",
                    drop.item_id(),
                    drop.quantity(),
                    d.inv_type,
                    d.slot,
                    d.map_id
                ),
            },
            Some(left) => Reply {
                opcode: net::inventory::INVENTORY_OPERATION,
                body: net::inventory::inventory_quantity(inv_type, d.slot as i16, left),
                what: format!(
                    "InventoryOperation UPDATE QUANTITY: x{} of item {} leave {:?} slot {} for \
                     the floor of map {}, and {left} stay behind. Mode 1 rather than mode 3 \
                     because the slot is NOT empty - a mode 3 here would clear it on screen \
                     while the store still held {left}.",
                    drop.quantity(),
                    drop.item_id(),
                    d.inv_type,
                    d.slot,
                    d.map_id
                ),
            },
        };
        let enter = drop.enter_reply(net::drops::ENTER_FLOATING);
        self.live.insert(object_id, drop);
        PlacedDrop { object_id, removed, enter }
    }

    /// **Put a player's mesos on the floor.** `0x0143`, the other half of [`Self::drop_item`].
    ///
    /// The owner, 2026-09-09: *"I still cannot drop mesos."* They were right, and it was not a missing
    /// opcode: `world::mesodrop` decoded `0x0143` and then deliberately **refused** it, because
    /// the 09-08 work fixed the *freeze* a refused drop caused and never made the drop happen.
    ///
    /// # It returns ONE reply, and that is the difference from a bag drop
    ///
    /// [`Self::drop_item`] returns a `0x0070` REMOVE as well, because an item leaves a bag
    /// slot. **Mesos are not in a bag slot**, so there is no inventory operation to send and
    /// answering with one would be the "wrong reply" failure `CLAUDE.md` names - it would clear
    /// the latch and leave the client's bag running an entry loop about a slot that never
    /// changed. The caller sends a `StatChanged` carrying the new balance instead, and that is
    /// what clears `player+0x2330`.
    ///
    /// The drop is **public**, like a player's item drop: the owner, 2026-09-05, *"If a player drops
    /// an item on the ground, anyone in the map should be able to see it and pick it up."*
    /// Money has no untradeable override, so this one really is takeable by anyone.
    ///
    /// No arc: `source` is the resting place and the delay is zero, the same choice
    /// [`Self::drop_item`] makes for an item a player sets down rather than one flung off a
    /// corpse.
    pub fn drop_money(&mut self, d: DropMoneyOnGround) -> PlacedMoney {
        debug_assert!(d.meso > 0, "a zero-meso drop is refused at the call site");
        let object_id = self.mint_object_id();
        let (source, delay) = arc_from((d.from_x, d.from_y), (d.x, d.y));
        let drop = LiveDrop {
            object_id,
            map_id: d.map_id,
            // `meso > 0` is what makes this a bag of coins; `item` is the documented
            // placeholder that nothing reads on that path.
            item: store::Item::bundle(0, 1),
            inv_type: store::InventoryType::Etc,
            owner_id: d.character_id,
            party_id: 0,
            public: true,
            x: d.x,
            y: d.y,
            meso: d.meso,
            dropped_at_ms: d.now_ms,
            // Coins fall from the player, not from the ground under them. The owner, on a ladder:
            // *"It doesn't come out of the player's current ladder location"*.
            source_x: source.0,
            source_y: source.1,
            delay_ms: delay,
            from_mob: false,
        };
        debug_assert!(drop.is_meso(), "meso > 0 is what selects the money class");
        let enter = drop.enter_reply(net::drops::ENTER_FLOATING);
        self.live.insert(object_id, drop);
        PlacedMoney { object_id, enter }
    }

    /// **Pick a drop up.** Opcode-agnostic on purpose: this takes an object id, not a packet.
    ///
    /// Whichever of `0x0329..=0x032E` turns out to carry the request, the wiring is one match
    /// arm that reads a `u32` out of the body and calls this. See the module docs.
    ///
    /// A [`PickUp::Taken`] has already been removed from the table. If the caller then cannot
    /// put it in the bag, it must not send the leave packet and should call
    /// [`DropTable::restore`].
    pub fn take(
        &mut self,
        object_id: u32,
        character_id: u32,
        now_ms: u64,
        party_members: &[u32],
    ) -> PickUp {
        self.take_as(object_id, character_id, now_ms, party_members, None)
    }

    /// [`DropTable::take`] for a character's **pet**: the same ownership rules, plus the drop
    /// must be a mob's ([`LiveDrop::from_mob`]), and the leave is type 5 - "flies into the pet".
    pub fn take_by_pet(
        &mut self,
        object_id: u32,
        character_id: u32,
        pet_index: u32,
        now_ms: u64,
        party_members: &[u32],
    ) -> PickUp {
        self.take_as(object_id, character_id, now_ms, party_members, Some(pet_index))
    }

    fn take_as(
        &mut self,
        object_id: u32,
        character_id: u32,
        now_ms: u64,
        party_members: &[u32],
        pet_index: Option<u32>,
    ) -> PickUp {
        let Some(drop) = self.live.get(&object_id).copied() else {
            return PickUp::Unknown { object_id };
        };
        if now_ms >= drop.expires_at_ms(self.lifetime_ms) {
            self.live.remove(&object_id);
            return PickUp::Expired { object_id, leave: fade_reply(&drop, "it had expired") };
        }
        if !drop.may_be_taken_by(character_id, now_ms, self.owner_lock_ms, party_members) {
            if drop_is_locked_to_owner_forever(drop.item_id()) {
                return PickUp::Untradeable { object_id, owner_id: drop.owner_id };
            }
            let opens_at = drop.dropped_at_ms.saturating_add(self.owner_lock_ms);
            return PickUp::NotYours {
                object_id,
                owner_id: drop.owner_id,
                opens_in_ms: opens_at.saturating_sub(now_ms),
            };
        }
        if pet_index.is_some() && !drop.from_mob {
            return PickUp::NotForPets { object_id };
        }
        self.live.remove(&object_id);
        let leave = match pet_index {
            Some(pet) => Reply {
                opcode: net::drops::DROP_LEAVE_FIELD,
                body: net::drops::drop_picked_up_by_pet(object_id, character_id, pet),
                what: format!(
                    "DropLeaveField leaveType 5: character {character_id}'s pet {pet} picked up \
                     drop {object_id} = item {} x{}. Broadcast this to the field INCLUDING the \
                     owner.",
                    drop.item_id(),
                    drop.quantity()
                ),
            },
            None => Reply {
                opcode: net::drops::DROP_LEAVE_FIELD,
                body: net::drops::drop_picked_up_by_character(object_id, character_id),
                what: format!(
                    "DropLeaveField leaveType 2: character {character_id} picked up drop \
                     {object_id} = item {} x{}. The character id is read into drop+0xec and aims \
                     the pick-up animation, so it has to be the real one. Broadcast this to the \
                     field INCLUDING the picker.",
                    drop.item_id(),
                    drop.quantity()
                ),
            },
        };
        PickUp::Taken { drop, leave }
    }

    /// Put a taken drop back, because the caller could not put it in the bag.
    ///
    /// The id it already had is reused deliberately: nothing on the wire ever heard about the
    /// pick-up, since the leave packet is only sent once the bag write has succeeded, so from
    /// the client's side the drop never moved.
    pub fn restore(&mut self, drop: LiveDrop) {
        self.live.insert(drop.object_id, drop);
    }

    /// **Expire old drops.** Call it from `Session::tick`.
    ///
    /// `watching_map_id` is the map the client is standing on. Expired drops there produce a
    /// `0x046F` leaveType 0 (fade); expired drops on any other map are removed **silently**,
    /// because this client's pool holds nothing at all for a field it is not standing in - the
    /// pool is destroyed and rebuilt empty on every field entry.
    ///
    /// Nothing in the client does this by itself on any path that was read. See
    /// [`DROP_LIFETIME_MS`].
    ///
    /// # The return value is **always empty**, and the fades go to the owners
    ///
    /// It used to hand the `0x046F` straight back to the caller, and that was wrong the
    /// moment drops became private to their owner. The table is shared by every session on
    /// the channel and **every one of them ticks**, so whichever ticked first removed the
    /// drop and took the fade: the owner went on drawing an item that no longer existed, and
    /// a bystander was handed a leave packet for an object its pool never held. Neither is
    /// visible from one screen, which is why it survived.
    ///
    /// So every fade is now an [`Addressed`], and `crate::fields::Fields::with_drops` posts
    /// it to **everyone on the drop's map** through `Bus::publish_to_map` - so a fade cannot
    /// land on a field the drop was never on, and it reaches every screen the drop was on:
    /// the owner, the party members who were shown a party drop, and every bystander who was
    /// shown a public ground drop (the owner, 2026-09-18 - see [`Addressed`]). Each reads it out
    /// of its own mailbox on its next `collect_mail`, at most one 100 ms tick later; a client
    /// that never held the object id ignores it.
    ///
    /// **The signature keeps its `Vec<Reply>` so that `session/mod.rs::tick` still
    /// compiles** - that file belongs to the coordinator. The tidier form is
    /// `self.fields.sweep_drops(here, now_ms);` with no `out.extend` at all, and it is in the
    /// report rather than done here.
    ///
    /// A drop on a map **nobody is standing on** is still removed silently: this client's
    /// pool holds nothing for a field it is not in, and `publish_to_character` refuses a
    /// recipient who has walked away, so the two rules agree without a second test.
    pub fn sweep(&mut self, watching_map_id: crate::fields::FieldKey, now_ms: u64) -> Vec<Reply> {
        let _ = watching_map_id;
        let expired: Vec<LiveDrop> = self
            .live
            .values()
            .filter(|d| now_ms >= d.expires_at_ms(self.lifetime_ms))
            .copied()
            .collect();
        for drop in expired {
            self.live.remove(&drop.object_id);
            self.outbox.push(Addressed {
                map_id: drop.map_id,
                reply: fade_reply(&drop, "it reached the end of its lifetime"),
            });
        }
        Vec::new()
    }

    /// **What to send a character who has just entered a field.**
    ///
    /// The drop pool is destroyed and rebuilt empty on every field entry, exactly as the NPC
    /// pool is, so **every drop on the field must be re-sent after every `SetField`** or the
    /// floor comes back bare. That is not a bug the player could distinguish from the item
    /// being gone.
    ///
    /// [`net::drops::ENTER_INSTANT`] rather than `ENTER_FLOATING`: the item is already lying
    /// there and should not re-play its arc, and instant is the other of the two enter types
    /// that set `drop+0x61`, the gate every pick-up sweep in the client tests. It is also
    /// eight bytes shorter, because it skips the source-position block.
    ///
    /// Drops on this map that have already expired are removed here without a leave packet:
    /// the pool being entered is empty, so there is nothing to tell the client about.
    ///
    /// # It is filtered, because the floor is not public
    ///
    /// The owner, 2026-09-01: *"the drops can remain per client."* This used to re-send **every**
    /// drop on the field to whoever walked in, which was right while one character could be
    /// on a map at a time and is a leak the moment drops are owner-scoped: walk in, see
    /// somebody else's loot, and - because the client stores `ownType` at `drop+0x70` and
    /// never tests it again (`net::drops`, **[L]**) - be able to take it, since visibility is
    /// the *only* thing the server controls here.
    ///
    /// `viewer` is the arriving character and `party` is
    /// `crate::mobshare::Party::solo(viewer)` until parties land; `crate::mobshare::may_see_drop`
    /// is the whole predicate and this adds nothing to it.
    pub fn field_entry(
        &mut self,
        map_id: crate::fields::FieldKey,
        now_ms: u64,
        viewer: u32,
        party: &crate::mobshare::Party,
    ) -> Vec<Reply> {
        let stale: Vec<u32> = self
            .live
            .values()
            .filter(|d| d.map_id == map_id && now_ms >= d.expires_at_ms(self.lifetime_ms))
            .map(|d| d.object_id)
            .collect();
        for id in stale {
            self.live.remove(&id);
        }
        self.on_field(map_id)
            .filter(|d| crate::mobshare::may_see_drop(d.owner_id, viewer, party))
            .map(|d| d.enter_reply(net::drops::ENTER_INSTANT))
            .collect()
    }
}

/// `0x046F` leaveType 0: the drop fades, with nobody picking it up.
///
/// Leave types 0 and 1 both carry no extra field; 0 is the one that fades rather than
/// vanishing. Types 2, 3 and 5 carry a character id and are built by
/// `net::drops::drop_picked_up_by_character`.
fn fade_reply(drop: &LiveDrop, why: &str) -> Reply {
    Reply {
        opcode: net::drops::DROP_LEAVE_FIELD,
        body: net::drops::drop_leave_field(drop.object_id, net::drops::leave_type::FADE),
        what: format!(
            "DropLeaveField leaveType 0: drop {} = item {} on map {} is gone because {why}. \
             Nothing in the client expires a drop on any path that was read, so this is the \
             server's policy and DROP_LIFETIME_MS is its only source.",
            drop.object_id,
            drop.item_id(),
            drop.map_id
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::opcode::{EquipStatSet, EquipStats};
    use store::{InventoryType, Item, ItemKind};

    const MAP: crate::fields::FieldKey = crate::fields::FieldKey::world(1);
    const WISP: u32 = 200;
    const SOMEBODY_ELSE: u32 = 201;
    /// The sword the owner dragged out of the window on 2026-08-20. Not trade-blocked.
    const SWORD: u32 = 1_302_000;
    /// `store::ItemRules::TRADE_BLOCKED` carries this one, out of the client's own
    /// `info/tradeBlock`.
    const TRADE_BLOCKED_SWORD: u32 = 1_302_016;

    fn dropping(item: Item, now_ms: u64) -> DropFromBag {
        DropFromBag {
            map_id: MAP,
            character_id: WISP,
            inv_type: InventoryType::Equip,
            slot: 1,
            item,
            // The whole slot leaves, which is what every test here was written against.
            // Partial drops have their own tests.
            remaining_in_slot: None,
            x: 473,
            y: 395,
            from_x: 473,
            from_y: 395,
            now_ms,
        }
    }

    /// A **mob** drop, owner WISP, no party. Owner-locked for [`OWNER_LOCK_MS`], which is the
    /// behaviour the owner-lock tests need now that a *player's* ground drop is public and has
    /// no lock at all (the owner, 2026-09-05).
    fn from_mob_owned(item: Item, now_ms: u64) -> DropFromMob {
        DropFromMob {
            from_mob: true,
            map_id: MAP,
            owner_id: WISP,
            item,
            inv_type: InventoryType::Equip,
            meso: 0,
            x: 473,
            y: 395,
            source_x: 473,
            source_y: 395,
            now_ms,
            party_id: 0,
        }
    }

    /// A **party** mob drop, owner WISP (the killer), belonging to party `pid`.
    fn party_mob(item: Item, now_ms: u64, pid: u32) -> DropFromMob {
        DropFromMob { party_id: pid, ..from_mob_owned(item, now_ms) }
    }

    /// A player's own ground drop is public: anyone, at once, with no owner lock. The owner,
    /// 2026-09-05: *"anyone in the map should be able to see it and pick it up."*
    #[test]
    fn a_player_ground_drop_is_public_and_has_no_owner_lock() {
        let mut t = DropTable::with_lifetime(60_000, 15_000);
        t.drop_item(dropping(Item::equip(SWORD), 0));
        let id = t.on_field(MAP).next().unwrap().object_id;
        // A stranger, one millisecond later, with nobody in any party.
        assert!(t.take(id, SOMEBODY_ELSE, 1, &[]).taken().is_some(), "a public drop is free at once");
    }

    /// A trade-blocked item stays owner-only even on a public floor - the untradeable rule
    /// gates the transfer, and the floor is a transfer waiting to happen.
    #[test]
    fn an_untradeable_ground_drop_is_still_owner_only() {
        let mut t = DropTable::with_lifetime(60_000, 15_000);
        t.drop_item(dropping(Item::equip(TRADE_BLOCKED_SWORD), 0));
        let id = t.on_field(MAP).next().unwrap().object_id;
        assert!(matches!(t.take(id, SOMEBODY_ELSE, 1, &[]), PickUp::Untradeable { .. }));
        assert!(t.take(id, WISP, 1, &[]).taken().is_some(), "the owner always may");
    }

    /// A party drop: a current member may take it, a non-member may not, and a member who has
    /// **left** (no longer on the roster passed in) may not - but the killer always can, off
    /// `owner_id`. This is the owner's *"unless they were the killer."*
    #[test]
    fn a_party_drop_is_takeable_by_current_members_and_the_killer_only() {
        const PID: u32 = 7;
        const MEMBER: u32 = 202;
        const OUTSIDER: u32 = 999;
        let mut t = DropTable::with_lifetime(60_000, 15_000);
        // WISP killed it; the party is {WISP, MEMBER}.
        t.drop_from_mob(party_mob(Item::equip(SWORD), 0, PID));
        let id = t.on_field(MAP).next().unwrap().object_id;
        let roster = [WISP, MEMBER];

        // A member, immediately, no owner-lock wait.
        assert!(t.get(id).unwrap().may_be_taken_by(MEMBER, 1, t.owner_lock_ms(), &roster));
        // A non-party bystander: refused (still inside the owner lock).
        assert!(!t.get(id).unwrap().may_be_taken_by(OUTSIDER, 1, t.owner_lock_ms(), &roster));
        // The member LEFT: the roster the caller resolves no longer lists them, so refused -
        // while the killer, WISP, still passes on owner_id.
        assert!(!t.get(id).unwrap().may_be_taken_by(MEMBER, 1, t.owner_lock_ms(), &[WISP]));
        assert!(t.get(id).unwrap().may_be_taken_by(WISP, 1, t.owner_lock_ms(), &[WISP]));
    }

    /// Not `EquipStats::default()`: an item somebody has scrolled.
    fn rolled() -> EquipStats {
        EquipStats {
            stats: EquipStatSet { inc_pad: 17, inc_str: 3, ..EquipStatSet::default() },
            ..EquipStats::default()
        }
    }

    // ------------------------------------------------------------------------------
    // Object ids
    // ------------------------------------------------------------------------------

    /// `CLAUDE.md`: an id of 1 made the client silently refuse a transition once.
    #[test]
    fn object_ids_start_well_clear_of_small_numbers_and_are_never_reused() {
        let mut t = DropTable::new();
        assert_eq!(t.next_object_id(), FIRST_DROP_OBJECT_ID);
        // Read the bound out of the real table rather than copying the number: the highest
        // template id this client has is 9 990 545, and a drop id must be past it.
        let highest_template = *net::mob::SPECIAL_TEMPLATE_IDS.iter().max().unwrap();
        assert!(FIRST_DROP_OBJECT_ID > highest_template, "clear of every mob template id");

        t.drop_item(dropping(Item::equip(SWORD), 0));
        let first = t.on_field(MAP).next().unwrap().object_id;
        assert_eq!(first, FIRST_DROP_OBJECT_ID);

        // Take it back and drop another: the id must NOT come round again. A duplicate makes
        // DropEnterField read the id at 0x1417a3014 and stop, silently.
        assert!(t.take(first, WISP, 1, &[]).taken().is_some());
        t.drop_item(dropping(Item::equip(SWORD), 2));
        let second = t.on_field(MAP).next().unwrap().object_id;
        assert_eq!(second, first + 1, "monotonic, and the freed id is not handed out again");
    }

    #[test]
    fn ids_wrap_past_the_top_without_colliding_with_a_live_drop() {
        let mut t = DropTable::new();
        t.resume_ids_from(u32::MAX);
        t.drop_item(dropping(Item::equip(SWORD), 0));
        assert_eq!(t.on_field(MAP).next().unwrap().object_id, u32::MAX);
        // Wrapped back to the base - and the base is free, so it is used.
        t.drop_item(dropping(Item::equip(SWORD), 0));
        let ids: Vec<u32> = t.on_field(MAP).map(|d| d.object_id).collect();
        assert_eq!(ids, vec![FIRST_DROP_OBJECT_ID, u32::MAX]);
        // A third drop must skip the live base rather than duplicate it.
        t.resume_ids_from(FIRST_DROP_OBJECT_ID);
        t.drop_item(dropping(Item::equip(SWORD), 0));
        let ids: Vec<u32> = t.on_field(MAP).map(|d| d.object_id).collect();
        assert_eq!(ids, vec![FIRST_DROP_OBJECT_ID, FIRST_DROP_OBJECT_ID + 1, u32::MAX]);
    }

    #[test]
    fn resume_never_accepts_a_small_id() {
        let mut t = DropTable::new();
        t.resume_ids_from(1);
        assert_eq!(t.next_object_id(), FIRST_DROP_OBJECT_ID);
        t.resume_ids_from(0);
        assert_eq!(t.next_object_id(), FIRST_DROP_OBJECT_ID);
    }

    // ------------------------------------------------------------------------------
    // Dropping
    // ------------------------------------------------------------------------------

    /// The `0x0070` first, then the `0x046E`. The order is what keeps the UI alive.
    #[test]
    fn a_drop_answers_with_the_remove_first_and_the_enter_second() {
        let mut t = DropTable::new();
        let out = t.drop_item(dropping(Item::equip(SWORD), 1_000));

        assert_eq!(out.removed.opcode, net::inventory::INVENTORY_OPERATION);
        assert_eq!(out.removed.body.len(), net::inventory::INVENTORY_REMOVE_LEN);
        assert_eq!(out.removed.body[0], 1, "bExclRequestSent - this is what clears +0x2330");
        // The 7-byte header is bExclRequestSent, a u8, the u32 nCount and notRemoveAddInfo,
        // so the mode is byte 7 and not byte 5. Counting it by eye got that wrong once.
        assert_eq!(u32::from_le_bytes([out.removed.body[2], out.removed.body[3], out.removed.body[4], out.removed.body[5]]), 1, "nCount");
        assert_eq!(out.removed.body[7], net::inventory::MODE_REMOVE);
        assert_eq!(out.removed.body[8], InventoryType::Equip.as_u8());
        assert_eq!(i16::from_le_bytes([out.removed.body[9], out.removed.body[10]]), 1, "the source slot");

        assert_eq!(out.enter.opcode, net::drops::DROP_ENTER_FIELD);
        let b = &out.enter.body;
        assert_eq!(b.len(), net::drops::DROP_ENTER_FIELD_LEN);
        assert_eq!(b[0], net::drops::DROP_TYPE_ITEM);
        assert_eq!(b[1], net::drops::ENTER_FLOATING, "1 and 2 are the pickable enter types");
        assert_eq!(
            u32::from_le_bytes([b[2], b[3], b[4], b[5]]),
            FIRST_DROP_OBJECT_ID,
            "the pool key"
        );
        assert_eq!(u32::from_le_bytes([b[19], b[20], b[21], b[22]]), SWORD);
        assert_eq!(u32::from_le_bytes([b[23], b[24], b[25], b[26]]), WISP, "ownerId");
        assert_eq!(i16::from_le_bytes([b[28], b[29]]), 473);
        assert_eq!(i16::from_le_bytes([b[30], b[31]]), 395);
    }

    #[test]
    fn the_drop_is_on_the_table_afterwards_with_everything_the_pickup_needs() {
        let mut t = DropTable::new();
        t.drop_item(dropping(Item::bundle(2_000_000, 5), 700));
        let d = *t.on_field(MAP).next().unwrap();
        assert_eq!(d.map_id, MAP);
        assert_eq!(d.owner_id, WISP);
        assert_eq!(d.inv_type, InventoryType::Equip);
        assert_eq!(d.item_id(), 2_000_000);
        assert_eq!(d.quantity(), 5);
        assert_eq!((d.x, d.y), (473, 395));
        assert_eq!(d.dropped_at_ms, 700);
        assert_eq!(t.len(), 1);
        assert!(!t.is_empty());
        assert_eq!(t.get(d.object_id), Some(&d));
    }

    // ------------------------------------------------------------------------------
    // The round trip - the fidelity that matters
    // ------------------------------------------------------------------------------

    /// A scrolled equip must come back the same object, not the same item id.
    #[test]
    fn an_equip_keeps_its_rolled_stats_across_the_round_trip() {
        let scrolled = Item { item_id: SWORD, kind: ItemKind::Equip(Some(rolled())), failed_slots: 0, pet_id: None };
        let mut t = DropTable::new();
        t.drop_item(dropping(scrolled, 0));
        let id = t.on_field(MAP).next().unwrap().object_id;

        let out = t.take(id, WISP, 10, &[]);
        let taken = out.taken().expect("the owner may take it");
        assert_eq!(taken.item, scrolled, "the stats came back byte for byte");
        assert_eq!(taken.inv_type, InventoryType::Equip, "and it knows which bag to go back to");
        match taken.item.kind {
            ItemKind::Equip(Some(s)) => assert_eq!(s.stats.inc_pad, 17),
            other => panic!("the equip was flattened to {other:?}"),
        }
    }

    /// `None` stats mean "derive from the WZ template" and are NOT the same as zeros.
    #[test]
    fn a_fresh_equip_stays_fresh_rather_than_becoming_all_zero_stats() {
        let fresh = Item { item_id: SWORD, kind: ItemKind::FRESH_EQUIP, failed_slots: 0, pet_id: None };
        let mut t = DropTable::new();
        t.drop_item(dropping(fresh, 0));
        let id = t.on_field(MAP).next().unwrap().object_id;
        let taken = *t.take(id, WISP, 1, &[]).taken().unwrap();
        assert_eq!(taken.item.kind, ItemKind::Equip(None), "None must not collapse to Some(0)");
        assert_ne!(taken.item.kind, ItemKind::Equip(Some(EquipStats::default())));
    }

    #[test]
    fn a_bundle_keeps_its_quantity() {
        let stack = Item::bundle(2_000_000, 7);
        let mut t = DropTable::new();
        t.drop_item(dropping(stack, 0));
        let id = t.on_field(MAP).next().unwrap().object_id;
        let taken = *t.take(id, WISP, 1, &[]).taken().unwrap();
        assert_eq!(taken.item, stack);
        assert_eq!(taken.quantity(), 7, "the wire never carried this - only the table has it");
    }

    // ------------------------------------------------------------------------------
    // Picking up
    // ------------------------------------------------------------------------------

    #[test]
    fn a_pick_up_leaves_with_type_2_and_the_real_character_id() {
        let mut t = DropTable::new();
        t.drop_item(dropping(Item::equip(SWORD), 0));
        let id = t.on_field(MAP).next().unwrap().object_id;

        let out = t.take(id, WISP, 5, &[]);
        let replies = out.replies();
        assert_eq!(replies.len(), 1);
        assert_eq!(replies[0].opcode, net::drops::DROP_LEAVE_FIELD);
        let b = &replies[0].body;
        assert_eq!(b.len(), net::drops::DROP_PICKED_UP_LEN);
        assert_eq!(u32::from_le_bytes([b[0], b[1], b[2], b[3]]), id, "the id is read FIRST");
        assert_eq!(b[4], net::drops::leave_type::CHAR_PICKUP);
        assert_eq!(u32::from_le_bytes([b[5], b[6], b[7], b[8]]), WISP, "drop+0xec");
        assert!(out.notice().is_none(), "a taken drop needs no notice");
        assert!(t.is_empty(), "and it is off the floor");
    }

    #[test]
    fn taking_the_same_drop_twice_only_works_once() {
        let mut t = DropTable::new();
        t.drop_item(dropping(Item::equip(SWORD), 0));
        let id = t.on_field(MAP).next().unwrap().object_id;
        assert!(t.take(id, WISP, 1, &[]).taken().is_some());
        let again = t.take(id, WISP, 2, &[]);
        assert_eq!(again, PickUp::Unknown { object_id: id });
        assert!(again.replies().is_empty());
        assert!(again.notice().is_some(), "no packet exists, so there must still be words");
    }

    #[test]
    fn a_drop_that_was_never_ours_is_unknown_rather_than_a_panic() {
        let mut t = DropTable::new();
        let out = t.take(12345, WISP, 0, &[]);
        assert_eq!(out, PickUp::Unknown { object_id: 12345 });
    }

    #[test]
    fn restore_puts_a_taken_drop_back_under_the_same_id() {
        let mut t = DropTable::new();
        t.drop_item(dropping(Item::equip(SWORD), 0));
        let id = t.on_field(MAP).next().unwrap().object_id;
        let taken = *t.take(id, WISP, 1, &[]).taken().unwrap();
        assert!(t.is_empty());
        // The bag was full: the leave packet was never sent, so the client never heard.
        t.restore(taken);
        assert_eq!(t.get(id), Some(&taken));
        assert_eq!(t.len(), 1);
    }

    // ------------------------------------------------------------------------------
    // Ownership, and the untradeable decision
    // ------------------------------------------------------------------------------

    #[test]
    fn the_owner_lock_opens_after_its_window() {
        let mut t = DropTable::with_lifetime(60_000, 15_000);
        // A MOB drop is owner-locked; a player's own ground drop is public and has no lock.
        t.drop_from_mob(from_mob_owned(Item::equip(SWORD), 1_000));
        let id = t.on_field(MAP).next().unwrap().object_id;

        let early = t.take(id, SOMEBODY_ELSE, 5_000, &[]);
        assert_eq!(
            early,
            PickUp::NotYours { object_id: id, owner_id: WISP, opens_in_ms: 11_000 }
        );
        assert!(early.notice().is_some());
        assert_eq!(t.len(), 1, "a refusal must not take it off the floor");

        assert!(t.take(id, SOMEBODY_ELSE, 16_000, &[]).taken().is_some(), "the window has passed");
    }

    /// The owner's rule, at the point where an untradeable item would actually change hands.
    #[test]
    fn a_trade_blocked_drop_never_opens_up_to_anyone_else() {
        assert!(
            drop_is_locked_to_owner_forever(TRADE_BLOCKED_SWORD),
            "positive control: this id really is in the client's tradeBlock list"
        );
        assert!(!drop_is_locked_to_owner_forever(SWORD), "and this one really is not");

        let mut t = DropTable::with_lifetime(600_000, 15_000);
        t.drop_item(dropping(Item::equip(TRADE_BLOCKED_SWORD), 0));
        let id = t.on_field(MAP).next().unwrap().object_id;

        for now in [1_000, 100_000, 500_000] {
            assert_eq!(
                t.take(id, SOMEBODY_ELSE, now, &[]),
                PickUp::Untradeable { object_id: id, owner_id: WISP },
                "long past the owner lock at {now} ms, and still nobody else's"
            );
        }
        // The owner always may, which is the half of the decision that keeps it droppable.
        assert!(t.take(id, WISP, 500_000, &[]).taken().is_some());
    }

    // ------------------------------------------------------------------------------
    // Expiry
    // ------------------------------------------------------------------------------

    /// **The fade is addressed to the drop's whole FIELD, not handed to whoever ticked.**
    ///
    /// Every session on the channel sweeps this table, so before this the first one to tick
    /// took the `0x046F` and the owner went on drawing an item that no longer existed. Then
    /// it went to the owner alone, and a party member or a bystander who had been shown the
    /// drop kept drawing it (the owner, 2026-09-18). The two halves are asserted together, because
    /// a version that simply stopped emitting a fade at all would pass the first half on its
    /// own.
    #[test]
    fn a_drop_expires_and_the_sweep_says_so_exactly_once() {
        let mut t = DropTable::with_lifetime(10_000, 1_000);
        t.drop_item(dropping(Item::equip(SWORD), 1_000));
        let id = t.on_field(MAP).next().unwrap().object_id;

        assert!(t.sweep(MAP, 10_999).is_empty(), "still inside its lifetime");
        assert_eq!(t.addressed_len(), 0, "and nothing addressed to anybody yet");
        assert_eq!(t.len(), 1);

        assert!(
            t.sweep(MAP, 11_000).is_empty(),
            "the caller is never handed somebody else's fade - it goes to the field"
        );
        let out = t.take_addressed();
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].map_id, MAP, "the map - everyone who was shown it, whoever happened to tick - so it cannot land on another field");
        assert_eq!(out[0].reply.opcode, net::drops::DROP_LEAVE_FIELD);
        let b = &out[0].reply.body;
        assert_eq!(b.len(), net::drops::DROP_LEAVE_FIELD_LEN);
        assert_eq!(u32::from_le_bytes([b[0], b[1], b[2], b[3]]), id);
        assert_eq!(b[4], net::drops::leave_type::FADE);
        assert!(t.is_empty());

        assert!(t.sweep(MAP, 99_999).is_empty(), "and it is not announced twice");
        assert_eq!(t.addressed_len(), 0, "not on either channel");
    }

    /// A field nobody is standing on: the drop still goes, and the fade is still addressed to
    /// its map. `Bus::publish_to_map` posts to whoever is there, so a field with nobody on it
    /// simply has no recipients - which is the same rule the old `watching_map_id` filter was
    /// reaching for, enforced one layer up instead of guessed at here.
    #[test]
    fn expiring_on_another_map_is_still_addressed_to_that_map_and_still_removed() {
        let mut t = DropTable::with_lifetime(10_000, 1_000);
        t.drop_item(DropFromBag { map_id: crate::fields::FieldKey::world(40), ..dropping(Item::equip(SWORD), 0) });
        assert_eq!(t.len(), 1);
        assert!(t.sweep(MAP, 20_000).is_empty());
        let out = t.take_addressed();
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].map_id, crate::fields::FieldKey::world(40), "map 40's pool, not the ticking session's map");
        assert!(t.is_empty(), "and the server must not keep believing in it");
    }

    #[test]
    fn picking_up_something_that_has_already_expired_hands_nothing_back() {
        let mut t = DropTable::with_lifetime(10_000, 1_000);
        t.drop_item(dropping(Item::equip(SWORD), 0));
        let id = t.on_field(MAP).next().unwrap().object_id;

        let out = t.take(id, WISP, 10_001, &[]);
        assert!(out.taken().is_none());
        assert!(matches!(out, PickUp::Expired { .. }));
        assert_eq!(out.replies().len(), 1, "the client is still told the drop is gone");
        assert_eq!(out.replies()[0].body[4], net::drops::leave_type::FADE);
        assert!(t.is_empty());
    }

    // ------------------------------------------------------------------------------
    // Field entry
    // ------------------------------------------------------------------------------

    /// The owner's own drops, on their own map. See `only_the_owner_is_shown_the_floor` for the
    /// half this one deliberately does not exercise.
    #[test]
    fn field_entry_re_sends_only_this_maps_drops_and_does_not_re_animate_them() {
        let mut t = DropTable::with_lifetime(100_000, 1_000);
        t.drop_item(dropping(Item::equip(SWORD), 0));
        t.drop_item(dropping(Item::bundle(2_000_000, 3), 0));
        t.drop_item(DropFromBag { map_id: crate::fields::FieldKey::world(40), ..dropping(Item::equip(SWORD), 0) });

        let out = t.field_entry(MAP, 5_000, WISP, &crate::mobshare::Party::solo(WISP));
        assert_eq!(out.len(), 2, "the map 40 drop is somebody else's field");
        for r in &out {
            assert_eq!(r.opcode, net::drops::DROP_ENTER_FIELD);
            assert_eq!(r.body.len(), net::drops::DROP_ENTER_FIELD_LEN_INSTANT);
            assert_eq!(r.body[1], net::drops::ENTER_INSTANT, "already lying there");
        }
        // In id order, so the packet order is stable between runs.
        let ids: Vec<u32> = out
            .iter()
            .map(|r| u32::from_le_bytes([r.body[2], r.body[3], r.body[4], r.body[5]]))
            .collect();
        assert_eq!(ids, vec![FIRST_DROP_OBJECT_ID, FIRST_DROP_OBJECT_ID + 1]);
        assert_eq!(t.len(), 3, "listing the floor does not change it");
    }

    #[test]
    fn field_entry_forgets_a_drop_that_expired_while_nobody_was_looking() {
        let mut t = DropTable::with_lifetime(10_000, 1_000);
        t.drop_item(dropping(Item::equip(SWORD), 0));
        let out = t.field_entry(MAP, 30_000, WISP, &crate::mobshare::Party::solo(WISP));
        assert!(out.is_empty(), "an expired drop must not be re-sent to a fresh pool");
        assert!(t.is_empty());
        assert_eq!(
            t.addressed_len(),
            0,
            "and no fade for it either - the pool being entered is empty, so there is nothing \
             to tell anybody about"
        );
    }

    /// **The floor is private.** Walking in must not show - and therefore must not hand over,
    /// since visibility is the only thing the server controls - somebody else's loot.
    ///
    /// Both directions are asserted in one test. A `field_entry` that returned nothing to
    /// anybody would pass "the bystander sees none of the owner's", and that is a different bug:
    /// an item the owner cannot see either.
    #[test]
    fn only_the_owner_is_shown_the_floor() {
        let mut t = DropTable::with_lifetime(100_000, 1_000);
        t.drop_item(dropping(Item::equip(SWORD), 0));
        t.drop_item(DropFromBag {
            character_id: SOMEBODY_ELSE,
            ..dropping(Item::bundle(2_000_000, 3), 0)
        });

        let wisp = t.field_entry(MAP, 5_000, WISP, &crate::mobshare::Party::solo(WISP));
        assert_eq!(wisp.len(), 1, "the owner sees their own and only their own");
        assert_eq!(
            u32::from_le_bytes([
                wisp[0].body[23],
                wisp[0].body[24],
                wisp[0].body[25],
                wisp[0].body[26]
            ]),
            WISP,
            "ownerId - the same offset a_drop_answers_with_the_remove_first pins"
        );

        let other =
            t.field_entry(MAP, 5_000, SOMEBODY_ELSE, &crate::mobshare::Party::solo(SOMEBODY_ELSE));
        assert_eq!(other.len(), 1, "and the other player sees theirs");
        assert_eq!(
            u32::from_le_bytes([
                other[0].body[23],
                other[0].body[24],
                other[0].body[25],
                other[0].body[26]
            ]),
            SOMEBODY_ELSE
        );

        assert_eq!(t.len(), 2, "listing the floor still does not change it");

        // A party sees both, which is the seam and the reason the predicate takes one.
        let together = crate::mobshare::Party::of(WISP, [SOMEBODY_ELSE]);
        assert_eq!(t.field_entry(MAP, 5_000, WISP, &together).len(), 2);
    }

    #[test]
    fn an_empty_field_sends_nothing_at_all() {
        let mut t = DropTable::new();
        assert!(t.field_entry(MAP, 0, WISP, &crate::mobshare::Party::solo(WISP)).is_empty());
        assert!(t.sweep(MAP, 0).is_empty());
        assert_eq!(t.addressed_len(), 0);
    }

    // ------------------------------------------------------------------------------
    // Always answer, and the opcode that is not known
    // ------------------------------------------------------------------------------

    #[test]
    fn a_refused_drop_is_still_a_packet() {
        let m = net::inventory::InventoryMove { tick: 0, inv_type: 1, src: 1, dst: 0, count: 1 };
        let out = drop_refused(&m, "there is nowhere to put it");
        assert_eq!(out.len(), 1, "silence here kills the inventory UI for the session");
        assert_eq!(out[0].opcode, net::inventory::INVENTORY_OPERATION);
        assert_eq!(out[0].body, net::inventory::inventory_rejected());
        assert_eq!(out[0].body.len(), net::inventory::INVENTORY_REJECTED_LEN);
        assert_eq!(out[0].body[0], 1, "bExclRequestSent = 1 even on a refusal");
    }

    /// Every non-taken outcome must leave the caller holding *something* to send.
    #[test]
    fn every_pick_up_outcome_gives_the_caller_something_to_send() {
        let mut t = DropTable::with_lifetime(10_000, 15_000);
        // Mob drops: owner-locked, so a stranger gets NotYours. A player's own ground drop is
        // public now and would not.
        t.drop_from_mob(from_mob_owned(Item::equip(SWORD), 0));
        t.drop_from_mob(from_mob_owned(Item::equip(TRADE_BLOCKED_SWORD), 0));
        let ids: Vec<u32> = t.on_field(MAP).map(|d| d.object_id).collect();

        let outcomes = vec![
            t.take(999_999, WISP, 1, &[]),                  // Unknown
            t.take(ids[0], SOMEBODY_ELSE, 1, &[]),          // NotYours
            t.take(ids[1], SOMEBODY_ELSE, 1, &[]),          // Untradeable
            t.take(ids[0], WISP, 50_000, &[]),              // Expired
            t.take(ids[1], WISP, 1, &[]),                   // Taken
        ];
        for o in &outcomes {
            assert!(
                !o.replies().is_empty() || o.notice().is_some(),
                "{o:?} left the caller with nothing to send"
            );
            assert!(!o.what().is_empty(), "and nothing to write in world.log");
        }
        // And all five shapes really were exercised.
        assert!(matches!(outcomes[0], PickUp::Unknown { .. }));
        assert!(matches!(outcomes[1], PickUp::NotYours { .. }));
        assert!(matches!(outcomes[2], PickUp::Untradeable { .. }));
        assert!(matches!(outcomes[3], PickUp::Expired { .. }));
        assert!(matches!(outcomes[4], PickUp::Taken { .. }));
    }

    /// The six candidates, and the one packet that keeps being mistaken for them.
    #[test]
    fn the_pick_up_candidate_range_is_the_gap_and_excludes_the_mob_packet() {
        for op in PICK_UP_CANDIDATE_FIRST..=PICK_UP_CANDIDATE_LAST {
            assert!(may_be_the_pick_up_request(op), "{op:#06x} is in the unclaimed gap");
        }
        assert_eq!(PICK_UP_CANDIDATE_LAST - PICK_UP_CANDIDATE_FIRST + 1, 6);
        assert!(!may_be_the_pick_up_request(0x0328), "the NPC pool's last opcode");
        assert!(!may_be_the_pick_up_request(0x032F), "the next claimed one");
        assert!(
            !may_be_the_pick_up_request(net::drops::CLIENT_MOB_DROP_PICK_UP),
            "0x0301 is a MOB picking a drop up and carries a mob object id"
        );
        assert!(!may_be_the_pick_up_request(net::inventory::CLIENT_INVENTORY_MOVE));
    }

    #[test]
    fn a_partial_stack_is_refused_rather_than_half_dropped() {
        assert!(whole_slot_is_leaving(10, 10));
        assert!(whole_slot_is_leaving(1, 1), "an equip");
        assert!(!whole_slot_is_leaving(3, 10), "3 of 10 leaves 7, so mode 1 rather than mode 3");
        assert!(whole_slot_is_leaving(5, 0), "an empty slot cannot hold anything back");
    }

    /// **A drop made on a ladder falls from the player, not from the ground under them.**
    ///
    /// The owner, 2026-09-09: *"Whenever the character is on a ladder, the mesos drop location is
    /// incorrect. It doesn't come out of the player's current ladder location, but more closer
    /// to the ground."*
    ///
    /// Both halves are asserted, because the fix is a pair and either alone would be wrong:
    /// the item must still come to REST on the ground (or it is drawn outside the client's
    /// pick-up box and cannot be collected) and its arc must START at the player.
    #[test]
    fn a_drop_from_a_ladder_arcs_down_from_the_player() {
        // 200 px up a ladder, landing on the floor below.
        let (source, delay) = arc_from((473, 195), (473, 395));
        assert_eq!(source, (473, 195), "the arc starts where the player is");
        assert_eq!(delay, DROP_FLIGHT_MS, "and it takes time, or it is not an arc");

        // **The control: someone standing on the ground is unchanged.** This is the common
        // case and it must stay byte for byte what it was, or every ordinary drop grows an
        // animation nobody asked for.
        let (source, delay) = arc_from((473, 395), (473, 395));
        assert_eq!(source, (473, 395));
        assert_eq!(delay, 0, "no arc for a player already on the floor");
    }

    /// The same, through the real `drop_item`, so the wiring is covered and not just the
    /// helper - a correct `arc_from` that nothing calls would pass the test above.
    #[test]
    fn the_drop_paths_carry_the_arc_into_the_packet() {
        let mut t = DropTable::default();
        let on_ladder = DropFromBag {
            from_y: 195, // up a ladder; `dropping` rests it at y 395
            ..dropping(Item::equip(SWORD), 0)
        };
        let placed = t.drop_item(on_ladder);
        let live = t.get(placed.object_id).unwrap();
        assert_eq!((live.x, live.y), (473, 395), "it still lands on the floor");
        assert_eq!(live.source_y, 195, "but it comes out of the ladder");
        assert_eq!(live.delay_ms, DROP_FLIGHT_MS);

        // And the money path, which is the one the owner was looking at.
        let placed = t.drop_money(DropMoneyOnGround {
            map_id: MAP,
            character_id: WISP,
            meso: 100,
            x: 473,
            y: 395,
            from_x: 473,
            from_y: 195,
            now_ms: 0,
        });
        let live = t.get(placed.object_id).unwrap();
        assert!(live.is_meso());
        assert_eq!((live.x, live.y), (473, 395));
        assert_eq!(live.source_y, 195, "the coins come out of the ladder");
        assert_eq!(live.delay_ms, DROP_FLIGHT_MS);
    }

    /// The whole shape of the run this unblocks, in one test.
    #[test]
    fn drop_walk_away_come_back_and_pick_it_up() {
        let mut t = DropTable::new();
        let sword = Item { item_id: SWORD, kind: ItemKind::Equip(Some(rolled())), failed_slots: 0, pet_id: None };

        // The owner drags the sword out of the window on map 1.
        let answer = t.drop_item(dropping(sword, 1_000));
        assert_eq!(answer.removed.opcode, net::inventory::INVENTORY_OPERATION);
        assert_eq!(answer.enter.opcode, net::drops::DROP_ENTER_FIELD);
        let id = t.on_field(MAP).next().unwrap().object_id;

        // `!map 40` and back. The pool is rebuilt empty both times.
        let solo = crate::mobshare::Party::solo(WISP);
        assert!(t.field_entry(crate::fields::FieldKey::world(40), 2_000, WISP, &solo).is_empty());
        let back = t.field_entry(MAP, 3_000, WISP, &solo);
        assert_eq!(back.len(), 1, "the sword must still be lying there");
        assert_eq!(back[0].body[1], net::drops::ENTER_INSTANT);

        // And it is still the same object, with the same stats.
        let taken = *t.take(id, WISP, 4_000, &[]).taken().unwrap();
        assert_eq!(taken.item, sword);
        assert!(t.is_empty());
    }

    /// **A mob drop can change hands before anyone has seen it**, which is what lets the
    /// caller walk the damage ranking and stop at the first candidate still on the map.
    ///
    /// Three effects, because the packet is not the only one: the `0x046E`'s `ownerId`, the
    /// table's own record, and the pick-up rule that hangs off it. Asserting only the first
    /// would pass against a version that showed the drop to the top damager and still let the
    /// killer take it.
    #[test]
    fn a_mob_drop_can_be_re_addressed_before_anybody_has_been_told() {
        let mut t = DropTable::with_lifetime(100_000, 15_000);
        let (id, first) = t.drop_from_mob(DropFromMob {
            from_mob: true,
            map_id: MAP,
            owner_id: WISP,
            item: Item::equip(SWORD),
            inv_type: InventoryType::Equip,
            meso: 0,
            x: 100,
            y: 395,
            source_x: 100,
            source_y: 395,
            now_ms: 0,
            party_id: 0,
        });
        assert_eq!(id, FIRST_DROP_OBJECT_ID, "the id comes back so it can be re-addressed");
        let owner_at = |r: &Reply| u32::from_le_bytes([r.body[23], r.body[24], r.body[25], r.body[26]]);
        assert_eq!(owner_at(&first), WISP);

        let second = t.readdress(id, SOMEBODY_ELSE).expect("the drop is on the floor");
        assert_eq!(owner_at(&second), SOMEBODY_ELSE, "the packet names the new owner");
        assert_eq!(
            u32::from_le_bytes([second.body[2], second.body[3], second.body[4], second.body[5]]),
            id,
            "and it is the SAME object - nothing has been sent about it yet, so no client can \
             tell this happened"
        );
        assert_eq!(t.get(id).unwrap().owner_id, SOMEBODY_ELSE, "the table agrees");
        // ...and so does the pick-up, which is the effect a packet-only assertion would miss.
        assert!(t.get(id).unwrap().may_be_taken_by(SOMEBODY_ELSE, 1, t.owner_lock_ms(), &[]));
        assert!(!t.get(id).unwrap().may_be_taken_by(WISP, 1, t.owner_lock_ms(), &[]));

        assert!(t.readdress(999_999, WISP).is_none(), "a drop that is not there is None");
    }
}
