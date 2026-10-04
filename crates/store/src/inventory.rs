//! The bag: six inventories per character, and what is in each slot.
//!
//! Until this existed the only thing persisted about what a character owned was the
//! `equipment` table - one row per **worn** slot. Taking an item off moved it on screen and
//! wrote nothing down, so the next `SetField` (a portal, a `!map`, a relog) rebuilt the
//! equipped list from those rows and put it straight back on. The owner, 2026-08-19: *"items taken
//! off should persist as is during transitions from map to map."*
//!
//! # Six bags, 1-based slots
//!
//! [`InventoryType`] is the client's own numbering, `1..=6`, the same byte `0x0107` carries.
//! Slot numbering is **1-based**: index 0 is an unused hole in the client's slot arrays (see
//! `net::opcode::PRESENCE_INVENTORY_SIZE`), so a stored slot 0 would be an item nothing can
//! draw. `slot >= 1` is a `CHECK` constraint rather than a convention.
//!
//! Per-bag slot counts stay where they already are - the six `characters.slots_*` columns -
//! because that is what [`net::opcode::Character::inventory_slots`] already reads and what
//! the record already sends. This module adds the missing half: [`Store::set_inventory_slots`],
//! so a bought expansion can actually be written down.
//!
//! # One table for both item shapes, and why
//!
//! An **equip** is unique and carries per-item stats; a **bundle** stacks and carries a
//! quantity. They are one table with a `kind` discriminator and the equip columns nullable,
//! not two tables, for one reason that outweighs the tidiness of two:
//!
//! > **"one slot holds at most one item" is the invariant that matters**, and in one table it
//! > is the `PRIMARY KEY (character_id, inv_type, slot)`. Split across two tables no database
//! > constraint can express it, and the failure it prevents - the same slot occupied twice -
//! > is how an item gets duplicated or eaten.
//!
//! Everything else follows from that: a move is one `UPDATE`, a swap is three, and reading a
//! bag for a field entry is one ordered query rather than a merge of two.
//!
//! The cost is 26 nullable columns that are meaningless on a bundle row. `CHECK` constraints
//! keep the two shapes from being confused (an equip's quantity is always 1), and the row
//! codec is shared with `storage_item` and `equipment` so the three cannot drift.
//!
//! # `equipment` was NOT unified into this table
//!
//! The worn-slot table and an equip sitting in the bag describe the same kind of object, and
//! unifying them would be cleaner. It was deliberately not done: `equipment` is a table that
//! is **currently working against a real client** - it is what dresses the character in the
//! record - and this project has paid for casually re-deriving working things.
//!
//! What was done instead is additive and cannot change any existing behaviour: `equipment`
//! gains the **same 26 nullable stat columns**, by `ALTER TABLE`. No existing column moves, no
//! existing query changes (every one of them names its columns), and the gain is that
//! [`Store::unequip_to_bag`] and [`Store::equip_from_bag`] round-trip an item's per-item stats
//! instead of silently flattening a scrolled item back to its template. Without that column
//! set on both sides, "take it off and put it back on" would be a data-loss bug that nothing
//! downstream could report.
//!
//! # NULL stats mean "derive from the WZ template", not "zero stats"
//!
//! [`ItemKind::Equip`] carries `Option<EquipStats>` and the columns are nullable, because the
//! two are genuinely different states:
//!
//! * `None` - nothing has ever modified this item, so its stats are whatever its
//!   `Character.wz` / `Item.wz` template says. Every row written before scrolling exists is
//!   this. The wire edge builds them with `net::opcode::EquipStats::fresh`.
//! * `Some(all zeros)` - an item that really has no stats.
//!
//! Collapsing them would send an item with no stat lines, which is the exact symptom this
//! project already chased once (STATUS.md, "no stat line at all").
//!
//! # Nothing here authenticates
//!
//! These take a character id and no account, exactly as [`Store::set_character_map`] and
//! `quest.rs` do. The channel connection carries no credentials at all - it is identified only
//! by the migration row it claimed. Storage is the one exception and it is keyed by account
//! **because storage is per account**, not because anything checked one.

use rusqlite::types::Value;
use rusqlite::{params_from_iter, Connection, OptionalExtension};

use net::opcode::{EquipOptions, EquipStatSet, EquipStats};

use crate::db::{Store, INVENTORY_SLOT_COLUMNS};
use crate::error::{Result, StoreError};

// -------------------------------------------------------------------------------------
// Which of the six bags
// -------------------------------------------------------------------------------------

/// The six inventories, numbered the way the client numbers them.
///
/// **[L]** for the numbering: `net::inventory::CLIENT_INVENTORY_MOVE`'s body documents
/// `u8 invType  1 Equip, 2 Consume, 3 Install, 4 Etc, 5 Cash, 6 Decoration`, read off the
/// `0x0107` decode, and `net::inventory::INV_EQUIP` is 1. The names match
/// `net::opcode::INVENTORY_SLOT_ORDER`, which came off the owner's own inventory window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum InventoryType {
    Equip = 1,
    Use = 2,
    Setup = 3,
    Etc = 4,
    Cash = 5,
    Deco = 6,
}

impl InventoryType {
    /// All six, in the order the record sends their slot counts.
    pub const ALL: [InventoryType; net::opcode::INVENTORY_COUNT] = [
        InventoryType::Equip,
        InventoryType::Use,
        InventoryType::Setup,
        InventoryType::Etc,
        InventoryType::Cash,
        InventoryType::Deco,
    ];

    pub fn as_u8(self) -> u8 {
        self as u8
    }

    /// The index into [`net::opcode::Character::inventory_slots`] and
    /// [`crate::db::INVENTORY_SLOT_COLUMNS`]. One less than the wire's number, because the
    /// wire numbers from 1 and the array is 0-based. A test pins the two against
    /// [`net::opcode::INVENTORY_SLOT_ORDER`].
    pub fn index(self) -> usize {
        self as usize - 1
    }

    /// Parse the byte `0x0107` carries. `i16` rather than `u8` because the packet field is an
    /// `i8` and a negative value has to be refusable rather than wrapping into a valid bag.
    pub fn from_wire(value: i16) -> Result<Self> {
        match value {
            1 => Ok(InventoryType::Equip),
            2 => Ok(InventoryType::Use),
            3 => Ok(InventoryType::Setup),
            4 => Ok(InventoryType::Etc),
            5 => Ok(InventoryType::Cash),
            6 => Ok(InventoryType::Deco),
            other => Err(StoreError::InvalidInventoryType { value: other }),
        }
    }

    /// Which bag an item id belongs in, by its leading digit.
    ///
    /// **[D] for equips, [I] for the rest.** `1xxxxxx` being an equip is corroborated in this
    /// client: `world::shops::gender_of` tests `1_000_000..2_000_000` and its gender rule was
    /// audited against 97 hand-transcribed rows with no disagreement. The other four are the
    /// family's usual convention and **nothing in this client has been read to confirm them** -
    /// which is why every write API takes the bag explicitly and this is only a helper.
    pub fn for_item(item_id: u32) -> Option<Self> {
        match item_id / 1_000_000 {
            1 => Some(InventoryType::Equip),
            2 => Some(InventoryType::Use),
            3 => Some(InventoryType::Setup),
            4 => Some(InventoryType::Etc),
            5 => Some(InventoryType::Cash),
            _ => None,
        }
    }
}

// -------------------------------------------------------------------------------------
// What is in a slot
// -------------------------------------------------------------------------------------

/// The `kind` column: 1 an equip, 2 a bundle.
///
/// These are the record's own item-blob type bytes - `net::opcode::EQUIPPED_ITEM_TYPE` is 1
/// and the bundle decode `FUN_140304450` is 2 - so the stored number and the wire's number are
/// the same namespace and no mapping table exists to drift. Type 3 (pet, `FUN_140304550`) has
/// no representation here yet; a pet is not just an item and giving it a half-row would be
/// worse than not having it.
const KIND_EQUIP: i64 = net::opcode::EQUIPPED_ITEM_TYPE as i64;
const KIND_BUNDLE: i64 = 2;

/// An equip, or a stack of something.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemKind {
    /// One equip. `None` stats means **nothing has modified this item** - derive them from
    /// its WZ template at the wire edge. See the module docs; it is not the same as zeros.
    Equip(Option<EquipStats>),
    /// A stack. Always at least 1; a zero-quantity bundle is an empty slot, and an empty slot
    /// is the absence of a row.
    Bundle { quantity: u16 },
}

impl ItemKind {
    /// A fresh equip with no per-item modifications.
    pub const FRESH_EQUIP: ItemKind = ItemKind::Equip(None);

    pub fn quantity(&self) -> u16 {
        match self {
            ItemKind::Equip(_) => 1,
            ItemKind::Bundle { quantity } => *quantity,
        }
    }

    pub fn is_equip(&self) -> bool {
        matches!(self, ItemKind::Equip(_))
    }
}

/// An item, without saying where it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item {
    pub item_id: u32,
    pub kind: ItemKind,
    /// **How many enhancement slots this equip has lost to FAILED scrolls**, and not yet had
    /// returned. Server-only: it is never sent and never read from a packet.
    ///
    /// The owner, 2026-09-09: *"If the item previously had 2 failed scroll slots, the player is
    /// allowed to use 2 clean slate scrolls on the item."* So this is a count, not a flag
    /// about the last action, and `world::scrolls` is the only thing that moves it.
    ///
    /// **It lives on `Item` rather than in `EquipStats` because it must travel.** An item has
    /// no stable row id - the primary key is `(character_id, inv_type, slot)` - so a count
    /// stored beside the slot would be lost the moment the player unequips or moves it.
    /// `remove_item` hands back an `Item` and `add_item` takes one, so a field here survives
    /// every move for free. And it cannot go in `EquipStats`: that struct maps exhaustively
    /// onto both the wire and the 26 stat columns, so a field there would change a packet.
    ///
    /// Always `0` for a bundle. Nothing enforces that in the type because a `CHECK` on the
    /// column is the wrong shape for a value that is legitimately `0` on most rows.
    pub failed_slots: u8,
    /// **Which pet this is**, for a pet item: the `pets.pet_id` row its name, skills and
    /// vitals live on. `None` for every other item, and for a pet that has not been through
    /// [`Store::add_item`] yet (`place_into_bag` numbers it). Server-only, travels with the
    /// row for the same reason `failed_slots` does - it is what tells two Huskies apart.
    ///
    /// The owner, 2026-09-16: *"Two Husky should not share the same name. The pets should in the
    /// background have different ids to identify them apart."*
    pub pet_id: Option<u32>,
    /// **The stats this equip rolled when a mob dropped it** - its own clean base, which an
    /// Innocence reverts to instead of the WZ template. `None` for every item that never
    /// rolled (anything not dropped by a mob, and everything from before item variance),
    /// which reverts to the template exactly as before. Server-only, and it travels with the
    /// row for the same reason `failed_slots` does.
    ///
    /// The owner, 2026-09-24: *"Can we make Innocence Scrolls keep a good base roll?"*
    /// (`world::variance`).
    pub rolled_base: Option<EquipStatSet>,
}

impl Item {
    pub fn equip(item_id: u32) -> Self {
        Item { item_id, kind: ItemKind::Equip(None), failed_slots: 0, pet_id: None, rolled_base: None }
    }

    pub fn bundle(item_id: u32, quantity: u16) -> Self {
        Item { item_id, kind: ItemKind::Bundle { quantity }, failed_slots: 0, pet_id: None, rolled_base: None }
    }

    /// The owner: *"Please do not allow untradeable items to be stored."* Same answer as
    /// [`ItemRules::may_be_stored`]; here so a caller holding an [`Item`] does not have to
    /// reach past it.
    pub fn may_be_stored(&self) -> bool {
        ItemRules::may_be_stored(self.item_id)
    }
}

/// An item and the slot it is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvItem {
    pub inv_type: InventoryType,
    /// **1-based.** See the module docs.
    pub slot: u16,
    pub item: Item,
}

/// Everything a character is carrying, ready for a field entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bag {
    /// The six slot counts, in [`InventoryType::ALL`] order - the same array
    /// [`net::opcode::Character::inventory_slots`] carries.
    pub slots: [u16; net::opcode::INVENTORY_COUNT],
    /// Every occupied slot, ordered by inventory then slot. Stable between two identical
    /// loads, so a record built from it can be diffed against a capture.
    pub items: Vec<InvItem>,
}

impl Bag {
    /// One inventory's contents, slot order.
    pub fn items_in(&self, inv_type: InventoryType) -> impl Iterator<Item = &InvItem> {
        self.items.iter().filter(move |i| i.inv_type == inv_type)
    }

    /// How many slots that bag has.
    pub fn slots_in(&self, inv_type: InventoryType) -> u16 {
        self.slots[inv_type.index()]
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// What a move did. The caller needs to know, because the three cases build different
/// `0x0070` bodies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveOutcome {
    /// The destination was empty and the whole source went into it.
    Moved,
    /// Both slots were occupied and they exchanged.
    Swapped,
    /// Two stacks of the same item merged. `moved` crossed; `remaining` stayed in the source,
    /// and 0 means the source slot is now empty; `destination` is what the destination holds
    /// now. The owner, 2026-09-18: *"it should try to fill the stack first (any remaining after
    /// the full stack will remain at the original position), if the resulting stack is
    /// already full, then it will carry out the swap slots procedure."* - the second half is
    /// [`MoveOutcome::Swapped`].
    Merged { moved: u16, remaining: u16, destination: u16 },
    /// Part of a stack went into an EMPTY destination: a new stack of `moved` there, and
    /// `remaining` (never 0) still in the source. Told apart from [`MoveOutcome::Merged`]
    /// because the client has to be sent an ADD for the new stack, not a new count for a
    /// stack it does not have.
    Split { moved: u16, remaining: u16 },
}

// -------------------------------------------------------------------------------------
// The two restrictions the owner set, as table lookups
// -------------------------------------------------------------------------------------

/// The item properties this crate enforces rules on.
///
/// # Why the ids are baked in rather than read from a file
///
/// Both restrictions are properties the **client ships** - `info/tradeBlock` and `info/quest` -
/// so neither is invented and neither comes from a fan site. They are extracted by
/// `python tools/dump_itemdata.py` into `gm-handbook/itemdata.txt`, which is **generated game
/// data and gitignored**: the repo carries the code to regenerate it, not the content.
///
/// A rule that silently stops being enforced when a gitignored file is missing is not a rule.
/// `world::shops` can afford to load that file at runtime and complain loudly when it is
/// absent, because a missing shop is visible; a missing *restriction* is invisible and fails
/// open, which is the one direction the owner's instruction cannot tolerate.
///
/// So the two **id lists** below - the rules, ~160 integers, no prices, no names, no stats -
/// are generated into this file by `python tools/gen_item_rules.py` and committed, while the
/// 2863-row table they came from is not. The test
/// `the_baked_item_rules_still_match_the_clients_own_table` re-derives them from
/// `gm-handbook/itemdata.txt` whenever that file is present, so drift is caught by the test
/// suite rather than by a player losing an item.
pub struct ItemRules;

impl ItemRules {
    /// Untradeable: `info/tradeBlock`. **May not be put in storage.**
    pub fn trade_blocked(item_id: u32) -> bool {
        Self::TRADE_BLOCKED.binary_search(&item_id).is_ok()
    }

    /// A quest item: `info/quest`. **May not be sold to an NPC** (the owner, goal F). Nothing in
    /// this crate refuses one yet - the sell path is the shop task's - and it is here because
    /// it is the same measurement out of the same generator.
    pub fn quest_item(item_id: u32) -> bool {
        Self::QUEST_ITEMS.binary_search(&item_id).is_ok()
    }

    /// The owner: *"Please do not allow untradeable items to be stored."*
    pub fn may_be_stored(item_id: u32) -> bool {
        !Self::trade_blocked(item_id)
    }

    /// The owner: *"Please do not allow quest items to be sold."*
    pub fn may_be_sold(item_id: u32) -> bool {
        !Self::quest_item(item_id)
    }

    // --- BEGIN GENERATED by tools/gen_item_rules.py - do not edit by hand ---
    /// How many item rows the lists below were distilled from. Provenance only; a test
    /// compares it against the live `gm-handbook/itemdata.txt` when that file exists.
    pub const GENERATED_FROM_ITEMS: usize = 2878;
    /// Item ids with `info/tradeBlock` set, ascending.
    pub const TRADE_BLOCKED: &'static [u32] = &[
        1032021, 1032022, 1302016, 1332017, 1372008, 1452008, 1472022, 2010000,
        2028001, 2028002, 2190000, 2210000, 2210001, 2210002, 2210003, 2210004,
        2210005, 2210006, 2430000, 2430001, 2430002, 2430003, 2430004, 2430005,
        2430006, 2430007, 2430008, 2430009, 2430010, 3010000, 3010001, 3010002,
        3010006, 3010009, 3010010, 3994010, 3994011, 4031082, 4031083,
    ];
    /// Item ids with `info/quest` set, ascending.
    pub const QUEST_ITEMS: &'static [u32] = &[
        1002138, 1302009, 4000000, 4031000, 4031001, 4031002, 4031003, 4031004,
        4031005, 4031006, 4031007, 4031008, 4031009, 4031010, 4031011, 4031012,
        4031013, 4031014, 4031015, 4031016, 4031017, 4031018, 4031019, 4031020,
        4031021, 4031022, 4031023, 4031024, 4031025, 4031026, 4031027, 4031028,
        4031029, 4031030, 4031031, 4031032, 4031033, 4031034, 4031035, 4031036,
        4031037, 4031038, 4031039, 4031040, 4031041, 4031042, 4031043, 4031044,
        4031045, 4031046, 4031047, 4031048, 4031049, 4031050, 4031051, 4031052,
        4031053, 4031054, 4031055, 4031056, 4031057, 4031058, 4031059, 4031060,
        4031061, 4031062, 4031063, 4031064, 4031065, 4031066, 4031067, 4031068,
        4031069, 4031070, 4031071, 4031072, 4031073, 4031074, 4031075, 4031076,
        4031077, 4031078, 4031079, 4031080, 4031081, 4031084, 4031085, 4031086,
        4031087, 4031088, 4031089, 4031090, 4031091, 4031092, 4031093, 4031094,
        4031095, 4031096, 4031097, 4031098, 4031099, 4031100, 4031101, 4031102,
        4031103, 4031104, 4031105, 4031106, 4031107, 4031108, 4031109, 4031110,
        4031111, 4031112, 4031113, 4031114, 4031115, 4031116, 4031117,
    ];
    // --- END GENERATED ---
}

// -------------------------------------------------------------------------------------
// The row codec: one definition, three tables
// -------------------------------------------------------------------------------------

/// The per-item equip stat columns, in the order [`equip_stat_values`] writes them.
///
/// **This array is the single source for the schema, the writes and the reads**, so a column
/// cannot exist in one and not the others. `inventory`, `storage_item` and `equipment` all get
/// exactly this set - an equip must not lose its stats by being moved between containers.
///
/// # What is here and what is not
///
/// All seventeen of `EquipStatSet` (the tooltip's `+n` lines, and the set a scroll changes),
/// plus the nine `EquipOptions` fields this client's own binary **names** - every one labelled
/// `[L]` in `net::opcode::EquipOptions`.
///
/// Deliberately absent, and named explicitly in [`equip_stat_values`]'s destructure so that
/// adding a field forces the decision again rather than dropping it silently:
///
/// * the twelve `unknown_b*` options - nothing in this binary names them, the only names on
///   offer come from a reference tree that scored 1 of 8 here, and a column named after a
///   guess is worse than no column;
/// * `second_stats` - `net::opcode::EquipStats` says its meaning is not established and to
///   leave it zero;
/// * `timed_stats` - likewise, and it changes the item's wire length, so storing one before
///   anything can produce one buys nothing.
///
/// Adding any of them later is an `ALTER TABLE ADD COLUMN` in [`add_equip_stat_columns`],
/// which is already the idiom here.
pub(crate) const EQUIP_STAT_COLUMNS: [&str; EQUIP_STAT_COLUMN_COUNT] = [
    // EquipStatSet, mask-bit order - the same order net::opcode::EQUIP_STAT_WZ_PROPERTIES uses
    "inc_str",
    "inc_dex",
    "inc_int",
    "inc_luk",
    "inc_mhp",
    "inc_mmp",
    "inc_speed",
    "inc_jump",
    "inc_pad",
    "inc_mad",
    "inc_pdd",
    "inc_mdd",
    "inc_acc",
    "inc_eva",
    "inc_crt",
    "inc_crd",
    "inc_wat",
    // EquipOptions, the nine the client's own binary names
    "remaining_enhancements",
    "attribute",
    "golden_hammer",
    "level_req_reduction",
    "boss_damage_pct",
    "ignore_enemy_def_pct",
    "damage_pct",
    "all_stats_pct",
    "scissor_uses",
];

/// 17 stats + 9 named options.
pub(crate) const EQUIP_STAT_COLUMN_COUNT: usize = 26;

/// The columns that describe an item, shared by `inventory` and `storage_item`.
/// `equipment` carries the stat tail but not `kind`/`quantity` - a worn slot is one equip.
pub(crate) fn item_columns() -> Vec<&'static str> {
    let mut c = vec!["item_id", "kind", "quantity"];
    c.extend_from_slice(&EQUIP_STAT_COLUMNS);
    // **Server-only, and LAST on purpose.** `Item::failed_slots` is not part of `EquipStats`
    // and must not be - that struct maps onto the wire. Appending it after the stat block
    // leaves `equip_stats_from_row`'s indexing untouched, so every stat column is read at
    // exactly the offset it was before.
    c.push(FAILED_SLOTS_COLUMN);
    c.push(PET_ID_COLUMN);
    c.push(ROLLED_BASE_COLUMN);
    c
}

/// The first server-only item column. See [`Item::failed_slots`].
pub(crate) const FAILED_SLOTS_COLUMN: &str = "failed_slots";

/// The second, after it. See [`Item::pet_id`]. Nullable: NULL is every non-pet row.
pub(crate) const PET_ID_COLUMN: &str = "pet_id";

/// The third. See [`Item::rolled_base`]. Nullable TEXT: NULL is "never rolled - the template
/// is its base", and a value is the 17 stats in [`EquipStatSet`] field order, comma-separated.
/// One column rather than seventeen because nothing ever queries a single stat of it.
pub(crate) const ROLLED_BASE_COLUMN: &str = "rolled_base";

/// [`Item::rolled_base`] -> the column. **Exhaustive**: a new stat on `EquipStatSet` stops this
/// compiling until it is decided whether it is stored.
pub(crate) fn rolled_base_text(set: &Option<EquipStatSet>) -> Value {
    let Some(set) = set else { return Value::Null };
    let EquipStatSet {
        inc_str,
        inc_dex,
        inc_int,
        inc_luk,
        inc_mhp,
        inc_mmp,
        inc_speed,
        inc_jump,
        inc_pad,
        inc_mad,
        inc_pdd,
        inc_mdd,
        inc_acc,
        inc_eva,
        inc_crt,
        inc_crd,
        inc_wat,
    } = set;
    let v = [
        inc_str, inc_dex, inc_int, inc_luk, inc_mhp, inc_mmp, inc_speed, inc_jump, inc_pad, inc_mad, inc_pdd, inc_mdd,
        inc_acc, inc_eva, inc_crt, inc_crd, inc_wat,
    ];
    Value::Text(v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","))
}

/// The column -> [`Item::rolled_base`]. Anything that is not exactly 17 numbers reads as
/// `None` - a hand-edited value that does not parse means "revert to the template", which is
/// what the item did before item variance existed, rather than an item with no stats.
pub(crate) fn rolled_base_from(text: Option<String>) -> Option<EquipStatSet> {
    let text = text?;
    let v: Vec<u16> = text.split(',').map(|x| x.trim().parse::<u16>()).collect::<std::result::Result<_, _>>().ok()?;
    let [inc_str, inc_dex, inc_int, inc_luk, inc_mhp, inc_mmp, inc_speed, inc_jump, inc_pad, inc_mad, inc_pdd, inc_mdd, inc_acc, inc_eva, inc_crt, inc_crd, inc_wat] =
        <[u16; 17]>::try_from(v).ok()?;
    Some(EquipStatSet {
        inc_str,
        inc_dex,
        inc_int,
        inc_luk,
        inc_mhp,
        inc_mmp,
        inc_speed,
        inc_jump,
        inc_pad,
        inc_mad,
        inc_pdd,
        inc_mdd,
        inc_acc,
        inc_eva,
        inc_crt,
        inc_crd,
        inc_wat,
    })
}

/// A worn item as the `equipment` row holds it: id, stats, failed slots, rolled base.
pub type WornItem = (u32, Option<EquipStats>, u8, Option<EquipStatSet>);

/// The `equipment` columns a worn item is read back with, and the reader for them.
///
/// **Its own helper because both unequip paths have to carry `failed_slots`**, and a path that
/// forgets it silently throws away a player's Clean Slate credit at the exact moment they take
/// the item off - which would look like the scroll never worked. The same reasoning already
/// applies to the stat block: the comment at the second call site records that it used to
/// select `item_id` alone and flattened a scrolled item on the way back to the bag.
pub(crate) fn worn_columns() -> String {
    format!("{}, {}, {}", EQUIP_STAT_COLUMNS.join(", "), FAILED_SLOTS_COLUMN, ROLLED_BASE_COLUMN)
}

/// [`WornItem`] from a row selected with [`worn_columns`] after `item_id`.
pub(crate) fn worn_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<WornItem> {
    let item_id = u32::try_from(row.get::<_, i64>(0)?).unwrap_or(0);
    let stats = equip_stats_from_row(row, 1)?;
    // Tolerant of a missing value for the same reason `item_from_row` is: a row written before
    // the column existed has no failures, which is what 0 says.
    let failed: i64 = row.get(1 + EQUIP_STAT_COLUMN_COUNT).unwrap_or(0);
    // And no rolled base, which is what NULL says: revert to the template.
    let rolled: Option<String> = row.get(2 + EQUIP_STAT_COLUMN_COUNT).unwrap_or(None);
    Ok((item_id, stats, u8::try_from(failed).unwrap_or(0), rolled_base_from(rolled)))
}

/// The stat columns as SQL declarations. Nullable on purpose: NULL is "no per-item stats
/// stored", which is a different state from zero. See the module docs.
pub(crate) fn equip_stat_declarations() -> String {
    let mut out: String =
        EQUIP_STAT_COLUMNS.iter().map(|c| format!("{c} INTEGER,\n                ")).collect();
    // Server-only; see [`item_columns`]. `NOT NULL DEFAULT 0` unlike the stat columns, and the
    // difference is real: "no per-item stats stored" is genuinely not the same as zero, but
    // "no failed slots recorded" IS zero.
    out.push_str(FAILED_SLOTS_COLUMN);
    out.push_str(" INTEGER NOT NULL DEFAULT 0,\n                ");
    out
}

/// `EquipStats` -> the 26 column values, in [`EQUIP_STAT_COLUMNS`] order.
///
/// **Exhaustive at all three levels.** Adding a field to `EquipStats`, `EquipStatSet` or
/// `EquipOptions` stops this file compiling until somebody decides whether it persists - the
/// same discipline `character.rs` uses, and for the same reason: a field left out of a
/// hand-written mapping is state that silently does not survive a relog.
fn equip_stat_values(stats: &EquipStats) -> [i64; EQUIP_STAT_COLUMN_COUNT] {
    let EquipStats { stats, options, second_stats, timed_stats } = stats;
    // Not stored - see EQUIP_STAT_COLUMNS. Bound rather than `..`-ignored so that a new field
    // on EquipStats is a compile error rather than a silent omission.
    let (_not_stored_second, _not_stored_timed) = (second_stats, timed_stats);

    let EquipStatSet {
        inc_str,
        inc_dex,
        inc_int,
        inc_luk,
        inc_mhp,
        inc_mmp,
        inc_speed,
        inc_jump,
        inc_pad,
        inc_mad,
        inc_pdd,
        inc_mdd,
        inc_acc,
        inc_eva,
        inc_crt,
        inc_crd,
        inc_wat,
    } = stats;

    let EquipOptions {
        remaining_enhancements,
        attribute,
        golden_hammer,
        level_requirement_reduction,
        boss_damage_percent,
        ignore_enemy_def_percent,
        damage_percent,
        all_stats_percent,
        scissor_uses,
        // The twelve the binary does not name. Listed rather than `..`-ignored, so a new
        // option bit cannot slip past unstored.
        unknown_b1: _,
        unknown_b3: _,
        unknown_b4: _,
        unknown_b5: _,
        unknown_b6: _,
        unknown_b9: _,
        unknown_b10: _,
        unknown_b11: _,
        unknown_b12: _,
        unknown_b13: _,
        unknown_b19: _,
        unknown_b20: _,
    } = options;

    [
        i64::from(*inc_str),
        i64::from(*inc_dex),
        i64::from(*inc_int),
        i64::from(*inc_luk),
        i64::from(*inc_mhp),
        i64::from(*inc_mmp),
        i64::from(*inc_speed),
        i64::from(*inc_jump),
        i64::from(*inc_pad),
        i64::from(*inc_mad),
        i64::from(*inc_pdd),
        i64::from(*inc_mdd),
        i64::from(*inc_acc),
        i64::from(*inc_eva),
        i64::from(*inc_crt),
        i64::from(*inc_crd),
        i64::from(*inc_wat),
        i64::from(*remaining_enhancements),
        i64::from(*attribute),
        i64::from(*golden_hammer),
        i64::from(*level_requirement_reduction),
        i64::from(*boss_damage_percent),
        i64::from(*ignore_enemy_def_percent),
        i64::from(*damage_percent),
        i64::from(*all_stats_percent),
        i64::from(*scissor_uses),
    ]
}

/// The 26 columns -> `EquipStats`, or `None` when they are all NULL.
///
/// **All-NULL is "no per-item stats stored"**, not "an item with zero stats". A row where some
/// are NULL and some are not can only be hand-written; the NULLs read as 0 rather than the row
/// being dropped, because an equip that vanishes is far worse than one with a wrong number.
fn equip_stats_from_row(row: &rusqlite::Row<'_>, base: usize) -> rusqlite::Result<Option<EquipStats>> {
    let mut v = [0i64; EQUIP_STAT_COLUMN_COUNT];
    let mut any = false;
    for (i, slot) in v.iter_mut().enumerate() {
        if let Some(n) = row.get::<_, Option<i64>>(base + i)? {
            *slot = n;
            any = true;
        }
    }
    if !any {
        return Ok(None);
    }
    // Saturating rather than wrapping: a hand-edited 70000 in `inc_str` should become the
    // biggest number the field can hold, not a small one.
    let n16 = |i: usize| u16::try_from(v[i]).unwrap_or(u16::MAX);
    let n8 = |i: usize| u8::try_from(v[i]).unwrap_or(u8::MAX);
    let n32 = |i: usize| u32::try_from(v[i]).unwrap_or(u32::MAX);
    Ok(Some(EquipStats {
        stats: EquipStatSet {
            inc_str: n16(0),
            inc_dex: n16(1),
            inc_int: n16(2),
            inc_luk: n16(3),
            inc_mhp: n16(4),
            inc_mmp: n16(5),
            inc_speed: n16(6),
            inc_jump: n16(7),
            inc_pad: n16(8),
            inc_mad: n16(9),
            inc_pdd: n16(10),
            inc_mdd: n16(11),
            inc_acc: n16(12),
            inc_eva: n16(13),
            inc_crt: n16(14),
            inc_crd: n16(15),
            inc_wat: n16(16),
        },
        options: EquipOptions {
            remaining_enhancements: n8(17),
            attribute: n16(18),
            golden_hammer: n32(19),
            level_requirement_reduction: n8(20),
            boss_damage_percent: n8(21),
            ignore_enemy_def_percent: n8(22),
            damage_percent: n8(23),
            all_stats_percent: n8(24),
            scissor_uses: n8(25),
            // Named in full rather than `..Default::default()`: a new option bit must break
            // this line, exactly as it breaks the writer above.
            unknown_b1: 0,
            unknown_b3: 0,
            unknown_b4: 0,
            unknown_b5: 0,
            unknown_b6: 0,
            unknown_b9: 0,
            unknown_b10: 0,
            unknown_b11: 0,
            unknown_b12: 0,
            unknown_b13: 0,
            unknown_b19: 0,
            unknown_b20: 0,
        },
        second_stats: EquipStatSet::default(),
        timed_stats: None,
    }))
}

/// `item_id, kind, quantity` and the 26 stat columns, as bind values.
pub(crate) fn item_values(item: &Item) -> Vec<Value> {
    let mut out = Vec::with_capacity(3 + EQUIP_STAT_COLUMN_COUNT);
    out.push(Value::Integer(i64::from(item.item_id)));
    match &item.kind {
        ItemKind::Equip(stats) => {
            out.push(Value::Integer(KIND_EQUIP));
            out.push(Value::Integer(1));
            match stats {
                Some(stats) => {
                    out.extend(equip_stat_values(stats).into_iter().map(Value::Integer))
                }
                None => out.extend(vec![Value::Null; EQUIP_STAT_COLUMN_COUNT]),
            }
        }
        ItemKind::Bundle { quantity } => {
            out.push(Value::Integer(KIND_BUNDLE));
            out.push(Value::Integer(i64::from(*quantity)));
            out.extend(vec![Value::Null; EQUIP_STAT_COLUMN_COUNT]);
        }
    }
    // Server-only, last, matching `item_columns`. Written for a bundle too - it is always 0
    // there, and a `NOT NULL` column has to be given something.
    out.push(Value::Integer(i64::from(item.failed_slots)));
    out.push(match item.pet_id {
        Some(id) => Value::Integer(i64::from(id)),
        None => Value::Null,
    });
    out.push(rolled_base_text(&item.rolled_base));
    out
}

/// The inverse of [`item_values`], starting at column `base`.
pub(crate) fn item_from_row(row: &rusqlite::Row<'_>, base: usize) -> rusqlite::Result<Item> {
    let item_id: i64 = row.get(base)?;
    let kind: i64 = row.get(base + 1)?;
    let quantity: i64 = row.get(base + 2)?;
    let kind = if kind == KIND_EQUIP {
        ItemKind::Equip(equip_stats_from_row(row, base + 3)?)
    } else {
        // Anything that is not the equip byte is read as a bundle. A `CHECK` keeps the column
        // to 1 or 2, so this can only be reached by a hand-edited database, and a stack of
        // one is a far better failure than an item that disappears.
        //
        // **Except an empty star stack**, which is a real 0 (`Store::spend_ammo`, 2026-10-02):
        // read as 1, every emptied stack would hand its owner one free star back.
        let q = u16::try_from(quantity).unwrap_or(1);
        let empty_stars = q == 0 && u32::try_from(item_id).is_ok_and(net::bag::bundle_has_serial);
        ItemKind::Bundle { quantity: if empty_stars { 0 } else { q.max(1) } }
    };
    // Server-only, immediately after the stat block - `item_columns` appends it there.
    // Tolerant of NULL so a row written before this column existed reads as "no failures",
    // which is the truthful answer for one.
    let failed_slots: i64 = row.get(base + 3 + EQUIP_STAT_COLUMN_COUNT).unwrap_or(0);
    // And the pet id after it, NULL (or absent) for anything that is not a numbered pet.
    let pet_id: Option<i64> = row.get(base + 4 + EQUIP_STAT_COLUMN_COUNT).unwrap_or(None);
    // And the rolled base after that, NULL for anything that never rolled.
    let rolled: Option<String> = row.get(base + 5 + EQUIP_STAT_COLUMN_COUNT).unwrap_or(None);
    Ok(Item {
        item_id: u32::try_from(item_id).unwrap_or(0),
        kind,
        failed_slots: u8::try_from(failed_slots).unwrap_or(0),
        pet_id: pet_id.and_then(|v| u32::try_from(v).ok()),
        rolled_base: rolled_base_from(rolled),
    })
}

// -------------------------------------------------------------------------------------
// Schema
// -------------------------------------------------------------------------------------

/// Create the inventory table. Called from `Store::init`.
///
/// A plain `CREATE TABLE IF NOT EXISTS` is enough **because the table is new** - the same
/// situation `quest.rs` is in, and the opposite of `Store::add_inventory_slot_columns`, where
/// a *column* had to be `ALTER`ed onto a table that already exists in the owner's live database.
/// The two `ALTER` paths this feature does need are [`add_equip_stat_columns`] (on the
/// existing `equipment` table) and `Store::add_meso_column`.
pub(crate) fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(&format!(
        r#"
        -- One row per OCCUPIED slot. An empty slot is the absence of a row, so there is
        -- exactly one representation of it - the same rule quest_state uses for "not
        -- started".
        --
        -- The primary key IS the invariant: one slot holds at most one item. See the module
        -- docs for why both item shapes share this table rather than being split in two.
        CREATE TABLE IF NOT EXISTS inventory (
            character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
            -- 1 Equip, 2 Use, 3 Setup, 4 Etc, 5 Cash, 6 Deco - the client's own numbering,
            -- the byte 0x0107 carries.
            inv_type     INTEGER NOT NULL,
            -- 1-BASED. Index 0 is an unused hole in the client's slot arrays.
            slot         INTEGER NOT NULL,
            item_id      INTEGER NOT NULL,
            -- 1 equip, 2 bundle: the record's own item-blob type bytes.
            kind         INTEGER NOT NULL,
            quantity     INTEGER NOT NULL DEFAULT 1,
            -- Per-item equip stats. NULL means "not stored - derive from the WZ template",
            -- which is different from zero.
            {stats}
            PRIMARY KEY (character_id, inv_type, slot),
            CHECK (inv_type BETWEEN 1 AND 6),
            CHECK (slot >= 1),
            CHECK (kind IN (1, 2)),
            {quantity_check},
            -- An equip is unique; a stack of two equips is not a thing the client can draw.
            CHECK (kind <> 1 OR quantity = 1)
        );

        CREATE INDEX IF NOT EXISTS idx_inventory_character ON inventory(character_id);
        "#,
        stats = equip_stat_declarations(),
        quantity_check = QUANTITY_CHECK,
    ))?;
    // **`inventory` needs the ALTER path too now, and it did not before.** The comment above
    // says a plain `CREATE TABLE IF NOT EXISTS` is enough "because the table is new" - that was
    // true when `inventory` was introduced and stopped being true the moment the owner's live
    // database had one. `CREATE TABLE IF NOT EXISTS` does nothing at all to an existing table,
    // so `failed_slots` would have appeared only in databases built from scratch, and every
    // inventory read on the owner's would have failed with "no such column".
    //
    // `db::tests::remys_real_database_upgrades_in_place` caught exactly that, which is what it
    // is for: it replays their real schema rather than a fresh one.
    add_equip_stat_columns(conn, "inventory")?;
    add_equip_stat_columns(conn, "equipment")?;
    allow_empty_rechargeable_stacks(conn)?;
    Ok(())
}

/// `inventory`'s quantity rule: at least one, **except a throwing star or bullet stack, which may
/// be empty** - `Store::spend_ammo` keeps it at 0 so it can be recharged (the owner, 2026-10-02).
/// `item_id / 10000` is integer division on an INTEGER column, the same `207`/`233` test as
/// `net::bag::bundle_has_serial`.
pub(crate) const QUANTITY_CHECK: &str = "CHECK (quantity >= 1 OR (kind = 2 AND item_id / 10000 IN (207, 233)))";

/// The rule every `inventory` table was created with until 2026-10-02.
const OLD_QUANTITY_CHECK: &str = "CHECK (quantity >= 1)";

/// **Rebuild a deployed `inventory` under [`QUANTITY_CHECK`].** SQLite cannot alter a CHECK in
/// place and `CREATE TABLE IF NOT EXISTS` does nothing to a table that exists, so the owner's live
/// table keeps refusing an empty star stack until this runs - and the live database is not the
/// repo's (memory: `maplecw-live-db-migrations`), so it has to happen on open.
///
/// SQLite's documented rebuild: the table's own stored `CREATE` with the one rule swapped,
/// under a new name; every row copied across by column order (same columns, same order);
/// the old table dropped, the new one renamed, its indexes re-made from their own stored SQL.
/// One transaction, so a half-rebuilt table cannot exist. Keyed on the old rule's text, so
/// it runs once and is a no-op on every later open. Nothing references `inventory`, so no
/// foreign key can dangle mid-rebuild.
fn allow_empty_rechargeable_stacks(conn: &Connection) -> Result<()> {
    let sql: Option<String> = conn
        .query_row("SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'inventory'", [], |r| r.get(0))
        .optional()?;
    let Some(sql) = sql else { return Ok(()) };
    if !sql.contains(OLD_QUANTITY_CHECK) {
        return Ok(());
    }
    let indexes: Vec<String> = {
        let mut stmt = conn.prepare(
            "SELECT sql FROM sqlite_master WHERE type = 'index' AND tbl_name = 'inventory' AND sql IS NOT NULL",
        )?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
        rows.collect::<std::result::Result<Vec<_>, _>>()?
    };
    // The table name is the first "inventory" in its own CREATE - nothing before it says it.
    let name_at = sql.find("inventory").expect("the stored CREATE names its table");
    let rebuilt = format!(
        "{}inventory_rebuild{}",
        &sql[..name_at],
        &sql[name_at + "inventory".len()..].replacen(OLD_QUANTITY_CHECK, QUANTITY_CHECK, 1)
    );
    let tx = conn.unchecked_transaction()?;
    tx.execute_batch(&rebuilt)?;
    tx.execute_batch(
        "INSERT INTO inventory_rebuild SELECT * FROM inventory;
         DROP TABLE inventory;
         ALTER TABLE inventory_rebuild RENAME TO inventory;",
    )?;
    for index in indexes {
        tx.execute_batch(&index)?;
    }
    tx.commit()?;
    Ok(())
}

/// Item ids that changed under the client's feet: `(old, new)`. Every open rewrites every
/// row in every table that carries an item id, so a database written before the change
/// keeps working after it - and the live server's database is not the repo's, so this
/// cannot be a one-off edit of a file (memory: `maplecw-live-db-migrations`).
///
/// * `5222221 -> 5681599`, 2026-09-12: the Signature Style Collection box. Nexon's id is in
///   family 522, which the classic client does not open on double-click (the owner: the box sent
///   no packet at all); the set coupons' family 568 is opened, so the backport now installs
///   the box under 5681599 (`tools/backport_install.py` `BOX_ID`,
///   `world::signaturestyle::COLLECTION`). A box bought before that sat in a bag as 5222221 -
///   an item the client can no longer name or open.
pub const ITEM_ID_RENAMES: &[(u32, u32)] = &[
    (5_222_221, 5_681_599),
    // The eight face coupons, 2026-09-12: the classic client's Beauty Coupon dialog opens for
    // hair coupons in 2540000..2549999 and face coupons in 2890000..2890999 (six other
    // thousand-wide ranges are androids and skins), never for 2897xxx - so a face coupon
    // sat in the Use tab and its double-click sent nothing. They wear 2890907..2890914 now.
    (2_897_007, 2_890_907),
    (2_897_008, 2_890_908),
    (2_897_009, 2_890_909),
    (2_897_010, 2_890_910),
    (2_897_011, 2_890_911),
    (2_897_012, 2_890_912),
    (2_897_013, 2_890_913),
    (2_897_014, 2_890_914),
    // The three hair-hats, 2026-09-18: the client reads the fourth digit of an equip's id
    // as its gender and 6 is female, so Aura / Linie / Lügner Hair (Hat) refused every male
    // character. They wear 1007910..1007912 now (digit 7 is unisex); the installer copies
    // the property image under the new name. `world::signaturestyle::HAIR_HAT_IDS`.
    (1_006_910, 1_007_910),
    (1_006_911, 1_007_911),
    (1_006_912, 1_007_912),
];

/// The tables that carry an item id, all of which [`rename_item_ids`] visits. `equipment`
/// (worn items) is here because the invariant is "every table with the column", not
/// "every table a box could be in" - the test below derives the list from the schema and
/// fails the moment a fifth table appears without being added.
pub const ITEM_ID_TABLES: &[&str] = &["inventory", "equipment", "cash_locker", "storage_item", "pets", "gifts", "effect_item", "trade_escrow", "shop_escrow"];

/// Apply [`ITEM_ID_RENAMES`] to every table in [`ITEM_ID_TABLES`]. Runs on every open.
pub(crate) fn rename_item_ids(conn: &Connection) -> Result<()> {
    for &(old, new) in ITEM_ID_RENAMES {
        for table in ITEM_ID_TABLES {
            conn.execute(
                &format!("UPDATE {table} SET item_id = ?1 WHERE item_id = ?2"),
                rusqlite::params![new, old],
            )?;
        }
    }
    Ok(())
}

/// Give an existing table the [`EQUIP_STAT_COLUMNS`] tail.
///
/// **`equipment` already exists in the owner's database**, so this cannot be a `CREATE TABLE IF NOT
/// EXISTS` - that does nothing at all to a table that is already there, and the columns would
/// appear only in databases created from scratch. Same reasoning, same idiom and the same
/// `PRAGMA table_info` guard as `Store::add_inventory_slot_columns`; `ALTER TABLE ADD COLUMN`
/// raises "duplicate column name" rather than being idempotent, and the whole schema is
/// applied on **every** open.
///
/// Nullable with no default, so every existing row reads back as "no per-item stats stored" -
/// which is exactly what those rows are.
pub(crate) fn add_equip_stat_columns(conn: &Connection, table: &str) -> Result<()> {
    let mut existing = std::collections::HashSet::new();
    {
        let mut stmt = conn.prepare(&format!("PRAGMA table_info({table})"))?;
        let names = stmt.query_map([], |row| row.get::<_, String>(1))?;
        for name in names {
            existing.insert(name?);
        }
    }
    for column in EQUIP_STAT_COLUMNS {
        if existing.contains(column) {
            continue;
        }
        conn.execute(&format!("ALTER TABLE {table} ADD COLUMN {column} INTEGER"), [])?;
    }
    // **The server-only column gets the same treatment, and for the same reason.** Both
    // `inventory` and `equipment` already exist in the owner's live database, so a
    // `CREATE TABLE IF NOT EXISTS` does nothing to them and the column would appear only in
    // databases built from scratch - which is the failure this function was written for.
    //
    // `NOT NULL DEFAULT 0` is safe on an `ALTER`: SQLite backfills every existing row with the
    // default, and 0 is the truthful value for an item that predates the column.
    if !existing.contains(FAILED_SLOTS_COLUMN) {
        conn.execute(
            &format!(
                "ALTER TABLE {table} ADD COLUMN {FAILED_SLOTS_COLUMN} INTEGER NOT NULL DEFAULT 0"
            ),
            [],
        )?;
    }
    // The pet id, 2026-09-16, nullable: NULL is the truthful value for every row that is
    // not a pet, and for a pet bought before pets were numbered - `pets::create_tables`
    // numbers those on the next open.
    if !existing.contains(PET_ID_COLUMN) {
        conn.execute(&format!("ALTER TABLE {table} ADD COLUMN {PET_ID_COLUMN} INTEGER"), [])?;
    }
    // The rolled base, 2026-09-24, nullable: NULL is the truthful value for every item that
    // predates item variance - it never rolled, and the template is its base.
    if !existing.contains(ROLLED_BASE_COLUMN) {
        conn.execute(&format!("ALTER TABLE {table} ADD COLUMN {ROLLED_BASE_COLUMN} TEXT"), [])?;
    }
    Ok(())
}

// -------------------------------------------------------------------------------------
// Reads
// -------------------------------------------------------------------------------------

/// The SELECT list for an inventory row: placement first, then the shared item columns.
fn inventory_select() -> String {
    format!("inv_type, slot, {}", item_columns().join(", "))
}

/// Everything a character is carrying, on a caller-supplied connection.
///
/// The connection is a parameter rather than taken from the `Store` so that `storage.rs` can
/// read a bag and a storage box **inside one transaction**. Moving an item between the two is
/// the one operation here that touches two tables, and doing it in two transactions is how an
/// item ends up in both places or neither.
pub(crate) fn read_bag(conn: &Connection, character_id: u32) -> Result<Bag> {
    let slots = conn.query_row(
            &format!(
                "SELECT {} FROM characters WHERE id = ?1",
                INVENTORY_SLOT_COLUMNS.join(", ")
            ),
            [i64::from(character_id)],
            |row| {
                Ok([
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ])
            },
        );
    // A character that is not in the table has no bag rather than an error: the caller is
    // mid-field-entry and must still answer. Its counts are the default bag.
    let slots: [u16; net::opcode::INVENTORY_COUNT] = match slots {
        Ok(s) => s,
        Err(rusqlite::Error::QueryReturnedNoRows) => net::opcode::default_inventory_slots(),
        Err(e) => return Err(e.into()),
    };

    let mut stmt = conn.prepare(&format!(
        "SELECT {} FROM inventory WHERE character_id = ?1 ORDER BY inv_type, slot",
        inventory_select()
    ))?;
    let rows = stmt.query_map([i64::from(character_id)], |row| {
        let inv_type: i64 = row.get(0)?;
        let slot: i64 = row.get(1)?;
        Ok((inv_type, slot, item_from_row(row, 2)?))
    })?;
    let mut items = Vec::new();
    for row in rows {
        let (inv_type, slot, item) = row?;
        // A row whose inv_type the enum cannot represent is dropped rather than guessed at.
        // The CHECK makes it unreachable except by hand-editing, and the same rule quest.rs
        // uses applies: a shorter list beats a wrong one.
        let Ok(inv_type) = InventoryType::from_wire(inv_type as i16) else { continue };
        items.push(InvItem { inv_type, slot: slot as u16, item });
    }
    Ok(Bag { slots, items })
}

/// One inventory slot, on a caller-supplied connection. See [`read_bag`].
pub(crate) fn read_slot(
    conn: &Connection,
    character_id: u32,
    inv_type: InventoryType,
    slot: u16,
) -> Result<Option<Item>> {
    Ok(conn
        .query_row(
            &format!(
                "SELECT {} FROM inventory
                  WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
                item_columns().join(", ")
            ),
            rusqlite::params![i64::from(character_id), inv_type.as_u8(), slot],
            |row| item_from_row(row, 0),
        )
        .optional()?)
}

impl Store {
    /// Everything a character is carrying, plus its six slot counts.
    ///
    /// **This is the field-entry call.** One query for the counts and one for the contents;
    /// both are ordered, so two identical loads produce identical bytes.
    pub fn bag(&self, character_id: u32) -> Result<Bag> {
        read_bag(&self.conn(), character_id)
    }

    /// One inventory's contents, slot order.
    pub fn bag_items(&self, character_id: u32, inv_type: InventoryType) -> Result<Vec<InvItem>> {
        Ok(self.bag(character_id)?.items.into_iter().filter(|i| i.inv_type == inv_type).collect())
    }

    /// What is in one slot, if anything.
    pub fn inventory_slot(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        slot: u16,
    ) -> Result<Option<Item>> {
        read_slot(&self.conn(), character_id, inv_type, slot)
    }

    /// How many slots one bag has. A character with no row gets the default bag, the same
    /// answer [`Store::bag`] gives.
    pub fn inventory_slots(&self, character_id: u32, inv_type: InventoryType) -> Result<u16> {
        Ok(self.bag(character_id)?.slots[inv_type.index()])
    }

    /// The lowest free slot in a bag, or `None` when it is full.
    pub fn free_slot(&self, character_id: u32, inv_type: InventoryType) -> Result<Option<u16>> {
        let bag = self.bag(character_id)?;
        Ok(lowest_free(&bag, inv_type))
    }
}

/// The lowest 1-based slot in `inv_type` with nothing in it.
pub(crate) fn lowest_free(bag: &Bag, inv_type: InventoryType) -> Option<u16> {
    let taken: std::collections::HashSet<u16> =
        bag.items_in(inv_type).map(|i| i.slot).collect();
    (1..=bag.slots_in(inv_type)).find(|s| !taken.contains(s))
}

/// Put an item into a bag on a caller-supplied connection, stacking onto existing slots first.
///
/// `bag` is a snapshot the caller already read; this keeps it up to date so a sequence of
/// placements does not work from a stale picture. Returns every slot it touched, in order.
///
/// **Nothing partial.** If the whole quantity does not fit, this returns `BagFull` and the
/// caller's transaction rolls back - a half-completed purchase would leave the client and the
/// database disagreeing about what was bought.
pub(crate) fn place_into_bag(
    conn: &Connection,
    character_id: u32,
    bag: &mut Bag,
    inv_type: InventoryType,
    item: &Item,
    max_stack: u16,
) -> Result<Vec<InvItem>> {
    let slots = bag.slots[inv_type.index()];
    let full = || StoreError::BagFull { inv_type: inv_type.as_u8(), slots };
    let mut changed = Vec::new();

    // **A pet is one slot and one identity, never a stack.** Two Huskies are two rows with
    // two `pet_id`s, whatever `max_stack` says; a pet arriving without a number (bought, or
    // from a locker row written before pets were numbered) gets a fresh `pets` row here, and
    // one that has a number keeps it - and is re-homed to this character if it changed hands.
    if net::inventory::is_pet(item.item_id) {
        let mut pet = *item;
        pet.pet_id = Some(match pet.pet_id {
            Some(id) => {
                crate::pets::adopt(conn, id, character_id, item.item_id)?;
                id
            }
            None => crate::pets::new_pet(conn, character_id, item.item_id)?,
        });
        let slot = lowest_free(bag, inv_type).ok_or_else(full)?;
        set_slot(conn, character_id, inv_type, slot, &pet)?;
        let placed = InvItem { inv_type, slot, item: pet };
        bag.items.push(placed);
        return Ok(vec![placed]);
    }
    // **An empty star stack is still an item** (`Store::spend_ammo`, the owner 2026-10-02): picked
    // up off the floor it takes a slot of its own at 0, the way it left one. Without this the
    // loop below would place "nothing" and report success, and the stack would vanish.
    if item.kind == (ItemKind::Bundle { quantity: 0 }) && net::bag::bundle_has_serial(item.item_id) {
        let slot = lowest_free(bag, inv_type).ok_or_else(full)?;
        set_slot(conn, character_id, inv_type, slot, item)?;
        let placed = InvItem { inv_type, slot, item: *item };
        bag.items.push(placed);
        return Ok(vec![placed]);
    }
    let mut remaining = match item.kind {
        ItemKind::Equip(_) => {
            let slot = lowest_free(bag, inv_type).ok_or_else(full)?;
            set_slot(conn, character_id, inv_type, slot, item)?;
            let placed = InvItem { inv_type, slot, item: *item };
            bag.items.push(placed);
            return Ok(vec![placed]);
        }
        ItemKind::Bundle { quantity } => quantity,
    };
    // **A star or bullet stack arrives as ONE item and stays one** (the owner, 2026-10-03: a
    // dropped star goes down *"along with its ammo information"*). Whatever count it carries
    // lands in one slot - a pickup, a withdrawal, a gift - rather than being cut at `max_stack`
    // into several stacks a recharge would each fill: one item in, one item out.
    if net::bag::bundle_has_serial(item.item_id) {
        let slot = lowest_free(bag, inv_type).ok_or_else(full)?;
        set_slot(conn, character_id, inv_type, slot, item)?;
        let placed = InvItem { inv_type, slot, item: *item };
        bag.items.push(placed);
        return Ok(vec![placed]);
    }
    let cap = max_stack.max(1);

    // **A star or bullet stack never takes in another.** The owner, 2026-10-03: *"pick ups of
    // stars items such as Wolbis should result in a separate item stack in the player's
    // inventory instead of adding to an existing slot."* A rechargeable stack is one item with
    // its own serial (`net::bag::bundle_has_serial`) that is topped up only by a recharge
    // (`Store::recharge_slot`), so whatever arrives - a pickup, a purchase, a gift - lands in a
    // slot of its own.
    if cap > 1 && !net::bag::bundle_has_serial(item.item_id) {
        // Top up existing stacks first, lowest slot first, the way the client's own
        // auto-arrange fills a bag.
        let existing: Vec<(u16, u16)> = bag
            .items_in(inv_type)
            .filter(|i| i.item.item_id == item.item_id && !i.item.kind.is_equip())
            .map(|i| (i.slot, i.item.kind.quantity()))
            .collect();
        for (slot, have) in existing {
            if remaining == 0 {
                break;
            }
            let room = cap.saturating_sub(have);
            if room == 0 {
                continue;
            }
            let take = room.min(remaining);
            remaining -= take;
            let topped = Item::bundle(item.item_id, have + take);
            set_slot(conn, character_id, inv_type, slot, &topped)?;
            if let Some(e) = bag.items.iter_mut().find(|i| i.inv_type == inv_type && i.slot == slot)
            {
                e.item = topped;
            }
            changed.push(InvItem { inv_type, slot, item: topped });
        }
    }

    while remaining > 0 {
        let slot = lowest_free(bag, inv_type).ok_or_else(full)?;
        let take = cap.min(remaining);
        remaining -= take;
        let placed = Item::bundle(item.item_id, take);
        set_slot(conn, character_id, inv_type, slot, &placed)?;
        bag.items.push(InvItem { inv_type, slot, item: placed });
        changed.push(InvItem { inv_type, slot, item: placed });
    }
    Ok(changed)
}

/// Take `count` out of an inventory slot on a caller-supplied connection.
///
/// Returns what was taken. Refuses rather than clamping when the slot holds less than asked
/// for: the client is the thing drawing the number, and handing it a different one silently is
/// how the two ends stop agreeing.
pub(crate) fn take_from_bag(
    conn: &Connection,
    character_id: u32,
    inv_type: InventoryType,
    slot: u16,
    count: Option<u16>,
) -> Result<Item> {
    let Some(item) = read_slot(conn, character_id, inv_type, slot)? else {
        return Err(StoreError::SlotEmpty { slot });
    };
    let have = item.kind.quantity();
    // **An emptied star stack** (`Store::spend_ammo` keeps rechargeables at 0): selling or
    // dropping it takes the empty stack, whatever count the client names - there is nothing
    // to count, and refusing would leave a slot nobody can ever clear.
    if have == 0 {
        conn.execute(
            "DELETE FROM inventory WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
            rusqlite::params![i64::from(character_id), inv_type.as_u8(), slot],
        )?;
        return Ok(Item::bundle(item.item_id, 0));
    }
    let want = count.unwrap_or(have).max(1);
    if want > have {
        return Err(StoreError::NotEnoughItems { slot, item_id: item.item_id, have, want });
    }
    if want == have {
        conn.execute(
            "DELETE FROM inventory WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
            rusqlite::params![i64::from(character_id), inv_type.as_u8(), slot],
        )?;
    } else {
        set_slot(conn, character_id, inv_type, slot, &Item::bundle(item.item_id, have - want))?;
    }
    Ok(match item.kind {
        ItemKind::Equip(_) => item,
        // A pet is a bundle of one that carries its identity; `want == have == 1` for it, so
        // the whole row leaves and the number leaves with it.
        ItemKind::Bundle { .. } if item.pet_id.is_some() => item,
        ItemKind::Bundle { .. } => Item::bundle(item.item_id, want),
    })
}

// -------------------------------------------------------------------------------------
// Writes
// -------------------------------------------------------------------------------------

impl Store {
    /// Put an item in a named slot, replacing whatever was there.
    ///
    /// The blunt instrument: use it for a load, a fixture, or a correction. Ordinary play goes
    /// through [`Store::add_item`], [`Store::move_item`] and [`Store::remove_item`], which
    /// check the things this does not.
    pub fn set_inventory_slot(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        slot: u16,
        item: &Item,
    ) -> Result<()> {
        let slots = self.inventory_slots(character_id, inv_type)?;
        check_slot(slot, slots)?;
        let conn = self.conn();
        set_slot(&conn, character_id, inv_type, slot, item)
    }

    /// Empty a slot. `false` if there was nothing in it.
    pub fn clear_inventory_slot(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        slot: u16,
    ) -> Result<bool> {
        let n = self.conn().execute(
            "DELETE FROM inventory WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
            rusqlite::params![i64::from(character_id), inv_type.as_u8(), slot],
        )?;
        Ok(n > 0)
    }

    /// Put an item in the first free slot of a bag, stacking onto an existing one first.
    ///
    /// **This is the shop-purchase and mob-drop call.** It returns every slot it touched, in
    /// the order it touched them, so the caller can build one `0x0070` per change without a
    /// second read.
    ///
    /// `max_stack` is the item's `info/slotMax`, which this crate does **not** know: it is in
    /// `gm-handbook/itemdata.txt`, which `world` already loads (`world::shops::ItemData`), and
    /// baking a 2785-row table in here to avoid one parameter would be committing extracted
    /// game data. `0` or `1` means "does not stack" and every unit takes its own slot.
    ///
    /// An equip never stacks whatever `max_stack` says.
    pub fn add_item(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        item: &Item,
        max_stack: u16,
    ) -> Result<Vec<InvItem>> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let mut bag = read_bag(&tx, character_id)?;
        let changed = place_into_bag(&tx, character_id, &mut bag, inv_type, item, max_stack)?;
        tx.commit()?;
        Ok(changed)
    }

    /// **Spend ammunition from a stack**: `count` thrown or fired out of one slot. Returns what is
    /// left in it.
    ///
    /// Like [`Store::remove_item`], except that a **rechargeable** stack - throwing stars and
    /// bullets, `net::bag::bundle_has_serial` - that reaches zero **stays in its slot at 0**.
    /// The owner, 2026-10-02: *"When stars reach 0, it should remain in the player's inventory
    /// because they should be able to recharge them at any general store."* An empty stack is
    /// still a stack, and only a **recharge** of that slot fills it (`Store::recharge_slot`);
    /// since 2026-10-03 a purchase or pickup of the same star takes a slot of its own
    /// (`place_into_bag`). Anything else that hits zero leaves the slot, as before.
    pub fn spend_ammo(&self, character_id: u32, inv_type: InventoryType, slot: u16, count: u16) -> Result<u16> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let Some(item) = read_slot(&tx, character_id, inv_type, slot)? else {
            return Err(StoreError::SlotEmpty { slot });
        };
        let have = item.kind.quantity();
        if count > have {
            return Err(StoreError::NotEnoughItems { slot, item_id: item.item_id, have, want: count });
        }
        let left = have - count;
        if left == 0 && !net::bag::bundle_has_serial(item.item_id) {
            tx.execute(
                "DELETE FROM inventory WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
                rusqlite::params![i64::from(character_id), inv_type.as_u8(), slot],
            )?;
        } else {
            set_slot(&tx, character_id, inv_type, slot, &Item::bundle(item.item_id, left))?;
        }
        tx.commit()?;
        Ok(left)
    }

    /// Take `count` out of a slot. `None` takes the whole slot.
    ///
    /// Returns what was taken. Refuses rather than clamping when the slot holds less than was
    /// asked for - the client is the thing drawing the number, and handing it a different one
    /// silently is how the two ends stop agreeing.
    pub fn remove_item(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        slot: u16,
        count: Option<u16>,
    ) -> Result<Item> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let taken = take_from_bag(&tx, character_id, inv_type, slot, count)?;
        tx.commit()?;
        Ok(taken)
    }

    /// Move within one bag: the plain move, the swap, and the stack merge.
    ///
    /// **This is what answers `0x0107` for a bag-to-bag drag.** `count` is the packet's own
    /// field (`None` for the `-1` a non-bundle carries); `max_stack` is the item's
    /// `info/slotMax`, supplied by the caller for the reason [`Store::add_item`] gives.
    ///
    /// Whole thing in one transaction, and the intermediate slot is 0 - which no real item can
    /// occupy, because slots are 1-based - so a swap cannot trip the primary key halfway
    /// through. SQLite cannot defer a non-foreign-key constraint, so a parking slot is the
    /// only way to do this without deleting a row and hoping the insert lands.
    pub fn move_item(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        src: u16,
        dst: u16,
        count: Option<u16>,
        max_stack: u16,
    ) -> Result<MoveOutcome> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let bag = read_bag(&tx, character_id)?;
        let slots = bag.slots[inv_type.index()];
        check_slot(src, slots)?;
        check_slot(dst, slots)?;
        // A drag onto its own slot. Refused rather than answered `Moved`, because "the move
        // succeeded and nothing changed" and "the move happened" build the same `0x0070` and
        // the caller cannot tell them apart afterwards.
        if src == dst {
            return Err(StoreError::SlotOccupied { slot: dst });
        }
        let find = |slot: u16| {
            bag.items_in(inv_type).find(|i| i.slot == slot).map(|i| i.item)
        };
        let Some(source) = find(src) else { return Err(StoreError::SlotEmpty { slot: src }) };
        let target = find(dst);

        // **A star or bullet stack is one item: it moves whole and never merges.** The owner,
        // 2026-10-03: *"The star should occupy an entire slot, no matter the ammo count"*, and
        // separate star items must not *"stack into 1 item"*. Dropped on another stack of the
        // same star it swaps, the same as two different items; dragged to an empty slot it goes
        // whole, whatever count the client named - a split would mint a second stack that a
        // recharge then fills.
        if net::bag::bundle_has_serial(source.item_id) {
            let outcome = if target.is_some() {
                swap_slots(&tx, character_id, inv_type, src, dst)?;
                MoveOutcome::Swapped
            } else {
                tx.execute(
                    "UPDATE inventory SET slot = ?4
                      WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
                    rusqlite::params![i64::from(character_id), inv_type.as_u8(), src, dst],
                )?;
                MoveOutcome::Moved
            };
            tx.commit()?;
            return Ok(outcome);
        }

        let outcome = match target {
            // Empty destination: one UPDATE, and the whole stack goes unless a partial count
            // was asked for.
            None => {
                let moving = count.unwrap_or(source.kind.quantity()).max(1);
                let have = source.kind.quantity();
                if moving > have {
                    return Err(StoreError::NotEnoughItems {
                        slot: src,
                        item_id: source.item_id,
                        have,
                        want: moving,
                    });
                }
                if moving == have {
                    tx.execute(
                        "UPDATE inventory SET slot = ?4
                          WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
                        rusqlite::params![i64::from(character_id), inv_type.as_u8(), src, dst],
                    )?;
                    MoveOutcome::Moved
                } else {
                    // A partial move off a stack: the source keeps the rest.
                    set_slot(
                        &tx,
                        character_id,
                        inv_type,
                        src,
                        &Item::bundle(source.item_id, have - moving),
                    )?;
                    set_slot(
                        &tx,
                        character_id,
                        inv_type,
                        dst,
                        &Item::bundle(source.item_id, moving),
                    )?;
                    MoveOutcome::Split { moved: moving, remaining: have - moving }
                }
            }
            Some(target)
                if target.item_id == source.item_id
                    && !target.kind.is_equip()
                    && !source.kind.is_equip()
                    && max_stack > 1 =>
            {
                // Same item, both bundles: merge as much as the destination has room for and
                // leave the rest behind. That is what the client does, and a swap here would
                // look to the player like the stacks refused to combine.
                let have = source.kind.quantity();
                let asked = count.unwrap_or(have).max(1);
                if asked > have {
                    return Err(StoreError::NotEnoughItems {
                        slot: src,
                        item_id: source.item_id,
                        have,
                        want: asked,
                    });
                }
                let room = max_stack.saturating_sub(target.kind.quantity());
                let moved = room.min(asked);
                if moved == 0 {
                    // The destination is already at slotMax. Nothing merges, so this is the
                    // swap the player would expect from dropping one full stack on another.
                    swap_slots(&tx, character_id, inv_type, src, dst)?;
                    MoveOutcome::Swapped
                } else {
                    let remaining = have - moved;
                    set_slot(
                        &tx,
                        character_id,
                        inv_type,
                        dst,
                        &Item::bundle(target.item_id, target.kind.quantity() + moved),
                    )?;
                    if remaining == 0 {
                        tx.execute(
                            "DELETE FROM inventory
                              WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
                            rusqlite::params![i64::from(character_id), inv_type.as_u8(), src],
                        )?;
                    } else {
                        set_slot(
                            &tx,
                            character_id,
                            inv_type,
                            src,
                            &Item::bundle(source.item_id, remaining),
                        )?;
                    }
                    MoveOutcome::Merged { moved, remaining, destination: target.kind.quantity() + moved }
                }
            }
            Some(_) => {
                swap_slots(&tx, character_id, inv_type, src, dst)?;
                MoveOutcome::Swapped
            }
        };
        tx.commit()?;
        Ok(outcome)
    }

    /// **Consolidate Item: pour every stack of the same item into the earlier stacks of it,
    /// up to `max_stack`.** The owner, 2026-09-18: *"This button should make sure that all items
    /// that can be stacked without violating their max stack size should be done."*
    ///
    /// Slots are walked in ascending order and a later stack is poured into the earliest
    /// stack of the same item with room, so what remains after this is: every stack of an
    /// item but the last at `max_stack`, the last holding the remainder, in the slots the
    /// earliest stacks occupied - and then **every remaining stack slides up to the first
    /// free slots**, in the order it had. The owner, same day: *"Consolidate items should also
    /// additionally move all items to take the first available slots in the inventory. Such
    /// as that blue potion should be consolidated upwards to be below the red potion."*
    ///
    /// Only bundles take part. An equip never stacks, a pet is a bundle with a `pet_id` and
    /// two pets are two pets whatever their item id (`place_into_bag` says the same), and an
    /// item whose `max_stack` is 0 or 1 is left exactly as it is. `max_stack` is a callback
    /// for the reason [`Store::add_item`] gives: `info/slotMax` lives in the world crate.
    ///
    /// One transaction, and the returned list is **every row that changed**, in the order
    /// the writes - and the `0x0070`s - have to happen: the new quantities (mode 1) and the
    /// emptied slots (mode 3) first, then the slides (mode 2) in ascending destination order,
    /// so each destination is already empty when its move lands, on disk and on screen. Empty
    /// when nothing changed, which the session still answers - the request latches the
    /// client.
    pub fn consolidate_bag(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        max_stack: &dyn Fn(u32) -> u16,
    ) -> Result<Vec<StackChange>> {
        self.rearrange_bag(character_id, inv_type, max_stack, &|stacks| plan_consolidation(stacks))
    }

    /// **Sort Items: consolidate, then put the tab in order - biggest stack first, then by
    /// name.** The owner, 2026-09-18: *"Sort Items should sort by quantity, then name."* The
    /// order is [`plan_sort`]'s; `name_of` is the item name table, which lives in the world
    /// crate for the same reason `max_stack` does. Same transaction shape and the same
    /// change list as [`Store::consolidate_bag`], plus the swaps that put the survivors in
    /// order.
    pub fn sort_bag(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        max_stack: &dyn Fn(u32) -> u16,
        name_of: &dyn Fn(u32) -> String,
    ) -> Result<Vec<StackChange>> {
        self.rearrange_bag(character_id, inv_type, max_stack, &|stacks| plan_sort(stacks, name_of))
    }

    /// The shared transaction under [`Store::consolidate_bag`] and [`Store::sort_bag`]: read
    /// the tab as [`Stack`]s, let `plan` say what changes, write exactly that, in order.
    fn rearrange_bag(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        max_stack: &dyn Fn(u32) -> u16,
        plan: &dyn Fn(&[Stack]) -> Vec<StackChange>,
    ) -> Result<Vec<StackChange>> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let bag = read_bag(&tx, character_id)?;
        let stacks: Vec<Stack> = bag
            .items_in(inv_type)
            .map(|i| Stack {
                slot: i.slot,
                item_id: i.item.item_id,
                quantity: i.item.kind.quantity(),
                cap: if i.item.kind.is_equip() || i.item.pet_id.is_some() {
                    1
                } else {
                    max_stack(i.item.item_id)
                },
            })
            .collect();
        let changes = plan(&stacks);
        for c in &changes {
            match *c {
                StackChange::Quantity { slot, quantity } => {
                    tx.execute(
                        "UPDATE inventory SET quantity = ?4
                          WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
                        rusqlite::params![i64::from(character_id), inv_type.as_u8(), slot, quantity],
                    )?;
                }
                StackChange::Emptied { slot } => {
                    tx.execute(
                        "DELETE FROM inventory
                          WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
                        rusqlite::params![i64::from(character_id), inv_type.as_u8(), slot],
                    )?;
                }
                StackChange::Moved { from, to } => {
                    tx.execute(
                        "UPDATE inventory SET slot = ?4
                          WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
                        rusqlite::params![i64::from(character_id), inv_type.as_u8(), from, to],
                    )?;
                }
                StackChange::Swapped { a, b } => swap_slots(&tx, character_id, inv_type, a, b)?,
            }
        }
        tx.commit()?;
        Ok(changes)
    }

    /// Resize one bag. Returns the count actually stored, which is clamped to
    /// `net::opcode::MIN_INVENTORY_SLOTS ..= net::opcode::MAX_INVENTORY_SLOTS`.
    ///
    /// **A shrink that would strand an item is refused**, not truncated: the row would still
    /// be in the table but past the end of the bag, so the client would never draw it and the
    /// player would have lost it with nothing to see. Clearing the slot first is the caller's
    /// decision to make, loudly.
    pub fn set_inventory_slots(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        slots: u16,
    ) -> Result<u16> {
        let slots = slots.clamp(net::opcode::MIN_INVENTORY_SLOTS, net::opcode::MAX_INVENTORY_SLOTS);
        let highest: Option<i64> = self.conn().query_row(
            "SELECT MAX(slot) FROM inventory WHERE character_id = ?1 AND inv_type = ?2",
            rusqlite::params![i64::from(character_id), inv_type.as_u8()],
            |row| row.get(0),
        )?;
        if let Some(highest) = highest {
            if highest as u16 > slots {
                return Err(StoreError::SlotOutOfRange { slot: highest as u16, slots });
            }
        }
        let column = INVENTORY_SLOT_COLUMNS[inv_type.index()];
        self.conn().execute(
            &format!("UPDATE characters SET {column} = ?2 WHERE id = ?1"),
            rusqlite::params![i64::from(character_id), slots],
        )?;
        Ok(slots)
    }
}

/// One occupied slot as [`plan_consolidation`] sees it: where, what, how many, and how many
/// of it fit in one slot (`1` for anything that does not stack).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stack {
    pub slot: u16,
    pub item_id: u32,
    pub quantity: u16,
    pub cap: u16,
}

/// One row [`Store::consolidate_bag`] changed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackChange {
    /// The stack in `slot` now holds `quantity` (more, for a stack that was poured into;
    /// fewer, for one that was partly poured out).
    Quantity { slot: u16, quantity: u16 },
    /// The stack in `slot` was poured out entirely and the slot is empty.
    Emptied { slot: u16 },
    /// The stack in `from` slid up to `to`, the first slot free once everything before it had
    /// slid. Always `to < from`, and always listed after every `Quantity` and `Emptied`.
    Moved { from: u16, to: u16 },
    /// The stacks in `a` and `b` - **both occupied** - changed places. Only [`plan_sort`]
    /// produces these, after every `Moved`, so by then the tab has no gaps and every swap is
    /// between two full slots. On the wire it is the same mode-2 move a drag onto an occupied
    /// slot is answered with, which the client draws as a swap (measured: every drag-swap
    /// since 2026-08).
    Swapped { a: u16, b: u16 },
}

/// **The consolidation, as arithmetic.** Walks `stacks` in slot order; each stack is poured,
/// as far as it goes, into the earlier stacks of the same item that still have room under
/// `cap`; then whatever is left slides up so the occupied slots are `1..=n` in the order they
/// had. Pure, so every branch is a unit test; [`Store::consolidate_bag`] writes what this
/// returns. Quantities and emptied slots first, in slot order, then the slides in ascending
/// destination order - see [`StackChange::Moved`].
pub fn plan_consolidation(stacks: &[Stack]) -> Vec<StackChange> {
    let mut work: Vec<Stack> = stacks.to_vec();
    work.sort_by_key(|s| s.slot);
    let before: Vec<u16> = work.iter().map(|s| s.quantity).collect();
    for later in 1..work.len() {
        // **Star and bullet stacks are never poured** (the owner, 2026-10-03: each is its own
        // item, and separate ones must not *"stack into 1 item"*). They only slide up.
        if work[later].cap <= 1 || net::bag::bundle_has_serial(work[later].item_id) {
            continue;
        }
        for earlier in 0..later {
            if work[later].quantity == 0 {
                break;
            }
            let e = work[earlier];
            if e.item_id != work[later].item_id || e.cap <= 1 || e.quantity == 0 {
                continue;
            }
            let room = e.cap.saturating_sub(e.quantity);
            let moved = room.min(work[later].quantity);
            if moved == 0 {
                continue;
            }
            work[earlier].quantity += moved;
            work[later].quantity -= moved;
        }
    }
    let mut out: Vec<StackChange> = work
        .iter()
        .zip(before)
        .filter(|(s, was)| s.quantity != *was)
        .map(|(s, _)| {
            if s.quantity == 0 {
                StackChange::Emptied { slot: s.slot }
            } else {
                StackChange::Quantity { slot: s.slot, quantity: s.quantity }
            }
        })
        .collect();
    // The slide: the k-th surviving stack belongs in slot k. Ascending, so by the time a
    // stack moves, every slot below its destination holds something that has already slid.
    // **An emptied star stack survives** (`Store::spend_ammo`): it was 0 before the pour and
    // is a real row, so it slides like any stack - only a stack the POUR emptied is gone.
    let was_empty: std::collections::HashSet<u16> =
        stacks.iter().filter(|s| s.quantity == 0).map(|s| s.slot).collect();
    let mut next: u16 = 1;
    for s in work.iter().filter(|s| s.quantity > 0 || was_empty.contains(&s.slot)) {
        if s.slot != next {
            out.push(StackChange::Moved { from: s.slot, to: next });
        }
        next += 1;
    }
    out
}

/// **The sort, as arithmetic.** [`plan_consolidation`] first - merge, then slide, so the
/// survivors sit in `1..=n` - and then the survivors are put in order by a sequence of swaps:
/// for each slot `k` from 1, if it does not already hold the stack that belongs there, swap
/// it with the slot that does. At most `n - 1` swaps, each between two occupied slots.
///
/// **The order** - the owner, 2026-09-18: *"Sort Items should sort by quantity, then name."*
/// Largest stack first (`quantity` descending), then name A to Z (case-insensitive), then
/// item id, then the slot the stack was in - so the result is total and two clicks agree.
/// "Largest first" is the reading taken of "by quantity"; it is one comparator here and
/// nothing else knows the direction.
pub fn plan_sort(stacks: &[Stack], name_of: &dyn Fn(u32) -> String) -> Vec<StackChange> {
    let mut out = plan_consolidation(stacks);
    // Replay the merge and the slide to know what sits where before the swaps.
    let mut work: Vec<Stack> = stacks.to_vec();
    work.sort_by_key(|s| s.slot);
    for c in &out {
        match *c {
            StackChange::Quantity { slot, quantity } => {
                work.iter_mut().find(|s| s.slot == slot).expect("planned from these").quantity = quantity;
            }
            StackChange::Emptied { slot } => work.retain(|s| s.slot != slot),
            StackChange::Moved { from, to } => {
                work.iter_mut().find(|s| s.slot == from).expect("planned from these").slot = to;
            }
            StackChange::Swapped { .. } => unreachable!("consolidation never swaps"),
        }
    }
    work.sort_by_key(|s| s.slot);
    // `work[k - 1]` is what slot k holds now; `wanted` is what it should hold.
    let mut wanted: Vec<Stack> = work.clone();
    wanted.sort_by(|x, y| {
        y.quantity
            .cmp(&x.quantity)
            .then_with(|| name_of(x.item_id).to_lowercase().cmp(&name_of(y.item_id).to_lowercase()))
            .then_with(|| x.item_id.cmp(&y.item_id))
            .then_with(|| x.slot.cmp(&y.slot))
    });
    // Stacks are told apart by the slot they held after the slide, which is unique.
    let mut now: Vec<u16> = work.iter().map(|s| s.slot).collect(); // now[k-1] = id of stack in slot k
    for (k, want) in wanted.iter().enumerate() {
        let here = now[k];
        if here == want.slot {
            continue;
        }
        let other = now.iter().position(|&id| id == want.slot).expect("every stack is somewhere");
        now.swap(k, other);
        out.push(StackChange::Swapped { a: (other + 1) as u16, b: (k + 1) as u16 });
    }
    out
}

/// Insert-or-replace one inventory slot. Takes anything that derefs to a `Connection`, so the
/// same statement serves a bare connection and a transaction.
pub(crate) fn set_slot(
    conn: &Connection,
    character_id: u32,
    inv_type: InventoryType,
    slot: u16,
    item: &Item,
) -> Result<()> {
    let columns = item_columns();
    let assignments: Vec<String> =
        columns.iter().enumerate().map(|(i, c)| format!("{c} = ?{}", i + 4)).collect();
    let placeholders: Vec<String> = (4..4 + columns.len()).map(|i| format!("?{i}")).collect();
    let sql = format!(
        "INSERT INTO inventory (character_id, inv_type, slot, {cols})
         VALUES (?1, ?2, ?3, {ph})
         ON CONFLICT(character_id, inv_type, slot) DO UPDATE SET {set}",
        cols = columns.join(", "),
        ph = placeholders.join(", "),
        set = assignments.join(", "),
    );
    let mut values = vec![
        Value::Integer(i64::from(character_id)),
        Value::Integer(i64::from(inv_type.as_u8())),
        Value::Integer(i64::from(slot)),
    ];
    values.extend(item_values(item));
    conn.execute(&sql, params_from_iter(values))?;
    Ok(())
}

/// Exchange two occupied slots.
///
/// Read both, delete both, write both back the other way round. Not two `UPDATE`s: the
/// primary key **is** `(character_id, inv_type, slot)`, so the first update would collide with
/// the row the second one is about to move, and SQLite cannot defer a primary-key check the
/// way it can a foreign key. A parking slot is not available either - `CHECK (slot >= 1)`
/// applies to updates too, so there is no value outside the bag to park in.
///
/// The caller always runs this inside a transaction, which is what makes delete-then-insert
/// safe: a failure between the two rolls the whole thing back rather than eating both items.
fn swap_slots(
    conn: &Connection,
    character_id: u32,
    inv_type: InventoryType,
    a: u16,
    b: u16,
) -> Result<()> {
    let read = |slot: u16| -> Result<Item> {
        Ok(conn.query_row(
            &format!(
                "SELECT {} FROM inventory
                  WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
                item_columns().join(", ")
            ),
            rusqlite::params![i64::from(character_id), inv_type.as_u8(), slot],
            |row| item_from_row(row, 0),
        )?)
    };
    let (first, second) = (read(a)?, read(b)?);
    conn.execute(
        "DELETE FROM inventory
          WHERE character_id = ?1 AND inv_type = ?2 AND slot IN (?3, ?4)",
        rusqlite::params![i64::from(character_id), inv_type.as_u8(), a, b],
    )?;
    set_slot(conn, character_id, inv_type, b, &first)?;
    set_slot(conn, character_id, inv_type, a, &second)?;
    Ok(())
}

/// Slots are 1-based and bounded by the container's own count.
pub(crate) fn check_slot(slot: u16, slots: u16) -> Result<()> {
    if slot < 1 || slot > slots {
        return Err(StoreError::SlotOutOfRange { slot, slots });
    }
    Ok(())
}

// -------------------------------------------------------------------------------------
// Worn equipment <-> the bag. THE goal-I operation.
// -------------------------------------------------------------------------------------

/// One worn slot, with whatever per-item stats it carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EquippedItem {
    /// The avatar's equipment slot, not an inventory slot.
    pub slot: u8,
    pub item_id: u32,
    /// `None` means "not stored - derive from the WZ template", which is every row written
    /// before this module existed.
    pub stats: Option<EquipStats>,
}

/// What an equip actually did: what went on, and what it pushed off.
///
/// The second field exists because the client has to be told about **both** movements. A
/// swap that reports only the item going on leaves the client holding an item the server
/// has moved underneath it, and this project has already learned once what a client and a
/// server disagreeing about an inventory slot looks like on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Equipped {
    /// The item now worn in the slot.
    pub equipped: u32,
    /// What was worn before, now sitting in the bag slot the new item came out of.
    /// `None` when the worn slot was empty - the ordinary case.
    pub displaced: Option<u32>,
}

impl Store {
    /// The worn slots, with their stats. `characters_for` returns the `(slot, item_id)` pairs
    /// the avatar look needs; this is the same rows with the stat tail.
    /// One worn item, with the server-only failed-slot count. For `world::scrolls`.
    ///
    /// `equipped_items` deliberately does not carry `failed_slots` - it feeds the avatar and
    /// the wire, where the count has no business - so the scroll path reads the row itself.
    pub fn worn_item(
        &self,
        character_id: u32,
        equip_slot: u8,
    ) -> Result<Option<WornItem>> {
        let conn = self.conn();
        let got = conn
            .query_row(
                &format!(
                    "SELECT item_id, {} FROM equipment WHERE character_id = ?1 AND slot = ?2",
                    worn_columns()
                ),
                rusqlite::params![i64::from(character_id), i64::from(equip_slot)],
                worn_from_row,
            )
            .optional()?;
        Ok(got)
    }

    /// Write a worn item's stats and failed-slot count back. For `world::scrolls`.
    ///
    /// **An UPDATE, not an upsert.** A scroll may only ever change an item that is already on
    /// the character; creating a row here would put an item on somebody who never equipped
    /// one. `Ok(false)` means the slot was empty and nothing was written, which the caller
    /// must treat as a refusal rather than a success.
    pub fn set_worn_equip(
        &self,
        character_id: u32,
        equip_slot: u8,
        stats: &EquipStats,
        failed_slots: u8,
    ) -> Result<bool> {
        let conn = self.conn();
        let assignments: Vec<String> = EQUIP_STAT_COLUMNS
            .iter()
            .enumerate()
            .map(|(i, c)| format!("{c} = ?{}", i + 3))
            .collect();
        let sql = format!(
            "UPDATE equipment SET {}, {FAILED_SLOTS_COLUMN} = ?{} \
             WHERE character_id = ?1 AND slot = ?2",
            assignments.join(", "),
            3 + EQUIP_STAT_COLUMN_COUNT
        );
        let mut values = vec![
            Value::Integer(i64::from(character_id)),
            Value::Integer(i64::from(equip_slot)),
        ];
        values.extend(equip_stat_values(stats).into_iter().map(Value::Integer));
        values.push(Value::Integer(i64::from(failed_slots)));
        let n = conn.execute(&sql, params_from_iter(values))?;
        Ok(n > 0)
    }

    pub fn equipped_items(&self, character_id: u32) -> Result<Vec<EquippedItem>> {
        let conn = self.conn();
        let mut stmt = conn.prepare(&format!(
            "SELECT slot, item_id, {} FROM equipment WHERE character_id = ?1 ORDER BY slot",
            EQUIP_STAT_COLUMNS.join(", ")
        ))?;
        let rows = stmt.query_map([i64::from(character_id)], |row| {
            Ok(EquippedItem {
                slot: row.get(0)?,
                item_id: row.get::<_, i64>(1)? as u32,
                stats: equip_stats_from_row(row, 2)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// **Take an item off, and write it down.** The whole of the owner's goal I in one call.
    ///
    /// Deletes the `equipment` row and inserts the equip bag row in **one transaction**, so
    /// there is no instant where the item is in both places or neither. `dst` is the bag slot
    /// the client asked for; `None` picks the lowest free one.
    ///
    /// Per-item stats travel with the item, which is the reason `equipment` was given the same
    /// columns; see the module docs.
    /// **Destroy a worn item outright.** For a cursed scroll that failed.
    ///
    /// `Ok(false)` means the slot was already empty and nothing was deleted, which the caller
    /// must treat as a refusal rather than a success - the same contract as
    /// [`Store::set_worn_equip`], and for the same reason: the two answers look identical from
    /// the outside and only one of them means the player lost an item.
    ///
    /// **There is deliberately no bag-slot equivalent here.** A bagged item is removed with
    /// [`Store::remove_item`], which already exists and already reports what it took.
    ///
    /// This is the only path in the workspace that deletes an item a character is *wearing*,
    /// and it exists because 52 of this client's 208 scrolls carry a non-zero `cursed`. It is
    /// not reachable from any Scroll of Secrets or Treasure Scroll - neither can fail into a
    /// destroy - so a call here always came from `0x0125`.
    pub fn destroy_worn_equip(&self, character_id: u32, equip_slot: u8) -> Result<bool> {
        let conn = self.conn();
        let n = conn.execute(
            "DELETE FROM equipment WHERE character_id = ?1 AND slot = ?2",
            rusqlite::params![i64::from(character_id), i64::from(equip_slot)],
        )?;
        Ok(n > 0)
    }

    pub fn unequip_to_bag(
        &self,
        character_id: u32,
        equip_slot: u8,
        dst: Option<u16>,
    ) -> Result<InvItem> {
        self.unequip_to_tab(character_id, equip_slot, InventoryType::Equip, dst)
    }

    /// [`Store::unequip_to_bag`] with the destination tab chosen: a cash equip comes off
    /// into the **Deco** tab (worn slots 101 and up), an ordinary one into Equip.
    ///
    /// The owner, 2026-09-12: none of the Übel outfit would go on - the client sent the moves
    /// from the Deco tab (`invType 6`, destination `-105`..) and the server refused them as
    /// "a bag-to-bag move needs two positive slots", because both halves here were welded
    /// to the Equip tab.
    pub fn unequip_to_tab(
        &self,
        character_id: u32,
        equip_slot: u8,
        inv_type: InventoryType,
        dst: Option<u16>,
    ) -> Result<InvItem> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let bag = read_bag(&tx, character_id)?;
        let slots = bag.slots[inv_type.index()];
        let dst = match dst {
            Some(slot) => {
                check_slot(slot, slots)?;
                if bag.items_in(inv_type).any(|i| i.slot == slot) {
                    return Err(StoreError::SlotOccupied { slot });
                }
                slot
            }
            None => lowest_free(&bag, inv_type)
                .ok_or(StoreError::BagFull { inv_type: inv_type.as_u8(), slots })?,
        };

        let worn = tx
            .query_row(
                &format!(
                    "SELECT item_id, {} FROM equipment WHERE character_id = ?1 AND slot = ?2",
                    worn_columns()
                ),
                rusqlite::params![i64::from(character_id), equip_slot],
                worn_from_row,
            )
            .optional()?;
        let Some((item_id, stats, failed_slots, rolled_base)) = worn else {
            return Err(StoreError::SlotEmpty { slot: u16::from(equip_slot) });
        };
        tx.execute(
            "DELETE FROM equipment WHERE character_id = ?1 AND slot = ?2",
            rusqlite::params![i64::from(character_id), equip_slot],
        )?;
        let item = Item { item_id, kind: ItemKind::Equip(stats), failed_slots, pet_id: None, rolled_base };
        set_slot(&tx, character_id, inv_type, dst, &item)?;
        tx.commit()?;
        Ok(InvItem { inv_type, slot: dst, item })
    }

    /// Put an item on, **displacing whatever is already there into the slot it came from**.
    ///
    /// The reverse of [`Store::unequip_to_bag`], and it round-trips the stats in both
    /// directions.
    ///
    /// # This used to refuse, and refusing was the bug
    ///
    /// The owner, 2026-08-21: *"Equipping another equipment (while having a current equipment in
    /// the same slot) should swap the current equipment with the one being replaced."*
    ///
    /// The doc that stood here said a swap "needs the free bag slot the swap frees up, which
    /// is the caller's arithmetic, not this crate's" - and that is simply wrong. **A swap
    /// needs no free slot at all**: the worn item goes into `src`, the very slot the incoming
    /// item is vacating in the same transaction. There is no instant in which either item is
    /// in two places or in neither, which is the whole reason this is one transaction and not
    /// an unequip followed by an equip.
    ///
    /// [`Equipped::displaced`] is `None` when the worn slot was empty, which is the ordinary
    /// case and the only one that existed before.
    pub fn equip_from_bag(&self, character_id: u32, src: u16, equip_slot: u8) -> Result<Equipped> {
        self.equip_from_tab(character_id, InventoryType::Equip, src, equip_slot)
    }

    /// [`Store::equip_from_bag`] with the source tab chosen. The Deco tab's cash equips go
    /// on through here, into worn slots 101 and up; a displaced item lands back in the same
    /// tab and slot the incoming one left, exactly as for Equip.
    pub fn equip_from_tab(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        src: u16,
        equip_slot: u8,
    ) -> Result<Equipped> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let Some(item) = read_slot(&tx, character_id, inv_type, src)? else {
            return Err(StoreError::SlotEmpty { slot: src });
        };
        let ItemKind::Equip(stats) = item.kind else {
            return Err(StoreError::NotAnEquip { item_id: item.item_id });
        };

        // Whatever is on that slot now, read WITH its stats rather than just its id: it is
        // about to travel back into the bag and a scrolled item must not come back flattened.
        // The old code selected `item_id` alone because all it did with the answer was
        // refuse.
        let worn = tx
            .query_row(
                &format!(
                    "SELECT item_id, {} FROM equipment WHERE character_id = ?1 AND slot = ?2",
                    worn_columns()
                ),
                rusqlite::params![i64::from(character_id), equip_slot],
                worn_from_row,
            )
            .optional()?;
        let displaced = worn.as_ref().map(|(item_id, _, _, _)| *item_id);
        if worn.is_some() {
            tx.execute(
                "DELETE FROM equipment WHERE character_id = ?1 AND slot = ?2",
                rusqlite::params![i64::from(character_id), equip_slot],
            )?;
        }
        tx.execute(
            "DELETE FROM inventory WHERE character_id = ?1 AND inv_type = ?2 AND slot = ?3",
            rusqlite::params![i64::from(character_id), inv_type.as_u8(), src],
        )?;
        // `worn_columns` rather than the stat columns alone: **putting an item ON has to carry
        // `failed_slots` too.** Writing only the stat block would let the column fall back to
        // its `DEFAULT 0`, so a player who scrolled an item, took it off and put it back on
        // would silently lose every Clean Slate they had earned on it.
        let columns = worn_columns();
        let placeholders: Vec<String> =
            (4..4 + EQUIP_STAT_COLUMN_COUNT + 2).map(|i| format!("?{i}")).collect();
        let mut values = vec![
            Value::Integer(i64::from(character_id)),
            Value::Integer(i64::from(equip_slot)),
            Value::Integer(i64::from(item.item_id)),
        ];
        match stats {
            Some(stats) => {
                values.extend(equip_stat_values(&stats).into_iter().map(Value::Integer))
            }
            None => {
                values.extend(vec![Value::Null; EQUIP_STAT_COLUMN_COUNT])
            }
        }
        values.push(Value::Integer(i64::from(item.failed_slots)));
        values.push(rolled_base_text(&item.rolled_base));
        tx.execute(
            &format!(
                "INSERT INTO equipment (character_id, slot, item_id, {columns})
                 VALUES (?1, ?2, ?3, {})",
                placeholders.join(", ")
            ),
            params_from_iter(values),
        )?;
        // The displaced item lands in `src`, which the DELETE above has just emptied. This
        // is why a swap needs no free slot.
        if let Some((item_id, stats, failed_slots, rolled_base)) = worn {
            set_slot(
                &tx,
                character_id,
                inv_type,
                src,
                &Item { item_id, kind: ItemKind::Equip(stats), failed_slots, pet_id: None, rolled_base },
            )?;
        }
        tx.commit()?;
        Ok(Equipped { equipped: item.item_id, displaced })
    }
}

// -------------------------------------------------------------------------------------
// Mesos
// -------------------------------------------------------------------------------------

impl Store {
    /// What the character is carrying. A character with no row has none.
    /// **Buy: pay and receive, in one transaction.** The whole of a shop purchase.
    ///
    /// Two calls - `add_mesos` then `add_item` - is the wrong shape here and it is worth
    /// saying why: a bag that turns out to be full after the mesos have gone leaves the player
    /// short with nothing to show, and the reverse order leaves them with a free item. Both
    /// halves are one transaction, so a purchase either happens or does not.
    ///
    /// `max_stack` is the item's `info/slotMax`; see [`Store::add_item`] for why it is a
    /// parameter. `price` is the **total**, not per unit - the discount arithmetic (goal H's
    /// citizenship grades) belongs to the caller that knows the grade.
    pub fn buy_item(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        item: &Item,
        max_stack: u16,
        price: u32,
    ) -> Result<Vec<InvItem>> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        // The bag is checked first, so the common refusal costs nothing and the meso write is
        // never the thing that gets rolled back.
        let mut bag = read_bag(&tx, character_id)?;
        let changed = place_into_bag(&tx, character_id, &mut bag, inv_type, item, max_stack)?;
        adjust_mesos(&tx, character_id, -i64::from(price))?;
        tx.commit()?;
        Ok(changed)
    }

    /// **Recharge one star or bullet stack to `slot_max`, and pay for it, in one transaction.**
    ///
    /// Only the slot the player pointed at moves - not the lowest partial stack of the same
    /// id, which is where [`Store::buy_item`] would put the units. Returns the slot as it now
    /// is. Refuses an empty slot, a different item, a non-rechargeable id and a full stack.
    pub fn recharge_slot(
        &self,
        character_id: u32,
        slot: u16,
        item_id: u32,
        slot_max: u16,
        price: u32,
    ) -> Result<InvItem> {
        let inv_type = InventoryType::Use;
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let Some(held) = read_slot(&tx, character_id, inv_type, slot)? else {
            return Err(StoreError::SlotEmpty { slot });
        };
        let have = held.kind.quantity();
        if held.item_id != item_id || !net::bag::bundle_has_serial(item_id) || have >= slot_max {
            return Err(StoreError::BagFull { inv_type: inv_type.as_u8(), slots: slot });
        }
        let full = Item { kind: ItemKind::Bundle { quantity: slot_max }, ..held };
        set_slot(&tx, character_id, inv_type, slot, &full)?;
        adjust_mesos(&tx, character_id, -i64::from(price))?;
        tx.commit()?;
        Ok(InvItem { inv_type, slot, item: full })
    }

    /// **Sell: hand over and be paid, in one transaction.** Returns the new meso balance.
    ///
    /// The owner: *"Please do not allow quest items to be sold."* Enforced here, at the API, and
    /// the item stays in the bag when it is refused.
    ///
    /// `unit_price` is the item's `info/price` - the client's own **sell** price, what an NPC
    /// pays. It is a parameter for the same reason `max_stack` is: it lives in
    /// `gm-handbook/itemdata.txt`, which `world` loads and this crate does not.
    pub fn sell_item(
        &self,
        character_id: u32,
        inv_type: InventoryType,
        slot: u16,
        count: Option<u16>,
        unit_price: u32,
    ) -> Result<u32> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let Some(peek) = read_slot(&tx, character_id, inv_type, slot)? else {
            return Err(StoreError::SlotEmpty { slot });
        };
        if !ItemRules::may_be_sold(peek.item_id) {
            return Err(StoreError::ItemMayNotBeSold { item_id: peek.item_id });
        }
        let taken = take_from_bag(&tx, character_id, inv_type, slot, count)?;
        let paid = i64::from(unit_price) * i64::from(taken.kind.quantity());
        let balance = adjust_mesos(&tx, character_id, paid)?;
        tx.commit()?;
        Ok(balance)
    }

    /// What the character is carrying. A character with no row has none.
    pub fn mesos(&self, character_id: u32) -> Result<u32> {
        let n: Option<i64> = self
            .conn()
            .query_row(
                "SELECT mesos FROM characters WHERE id = ?1",
                [i64::from(character_id)],
                |row| row.get(0),
            )
            .optional()?;
        Ok(n.unwrap_or(0).max(0) as u32)
    }

    pub fn set_mesos(&self, character_id: u32, mesos: u32) -> Result<()> {
        self.conn().execute(
            "UPDATE characters SET mesos = ?2 WHERE id = ?1",
            rusqlite::params![i64::from(character_id), i64::from(mesos)],
        )?;
        Ok(())
    }

    /// Add (or, with a negative delta, spend) mesos and return the new balance.
    ///
    /// **A spend that would go below zero is refused and changes nothing.** The whole read and
    /// write is one transaction, so two concurrent purchases cannot both see the same balance
    /// and both succeed - which is the shape of every duplication bug in this class.
    pub fn add_mesos(&self, character_id: u32, delta: i64) -> Result<u32> {
        let mut conn = self.conn();
        let tx = conn.transaction()?;
        let next = adjust_mesos(&tx, character_id, delta)?;
        tx.commit()?;
        Ok(next)
    }
}

/// Move a character's meso balance on a caller-supplied connection, refusing to go below zero.
///
/// The read and the write are one statement pair on one connection, so a purchase can do the
/// bag and the payment in a single transaction - see [`Store::buy_item`].
pub(crate) fn adjust_mesos(conn: &Connection, character_id: u32, delta: i64) -> Result<u32> {
    let have: i64 = conn
        .query_row(
            "SELECT mesos FROM characters WHERE id = ?1",
            [i64::from(character_id)],
            |row| row.get(0),
        )
        .optional()?
        .unwrap_or(0);
    let next = have + delta;
    if next < 0 {
        return Err(StoreError::NotEnoughMesos {
            have: have.max(0) as u32,
            want: delta.unsigned_abs().min(u64::from(u32::MAX)) as u32,
        });
    }
    let next = next.min(i64::from(u32::MAX));
    conn.execute(
        "UPDATE characters SET mesos = ?2 WHERE id = ?1",
        rusqlite::params![i64::from(character_id), next],
    )?;
    Ok(next as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use net::opcode::Character;

    fn store_with_character() -> (Store, i64, u32) {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = store
            .create_character(
                account,
                0,
                &Character { name: "Bagger".into(), ..Character::default() },
            )
            .unwrap();
        (store, account, chr.id)
    }

    fn dressed() -> Character {
        Character {
            name: "Dressed".into(),
            equips: vec![(5, 1040002), (6, 1060002), (7, 1072001), (11, 1302000)],
            ..Character::default()
        }
    }

    /// The wire's numbering and this crate's array index are the same six things.
    #[test]
    fn the_six_bags_line_up_with_the_records_own_order() {
        assert_eq!(InventoryType::ALL.len(), net::opcode::INVENTORY_COUNT);
        for (i, t) in InventoryType::ALL.iter().enumerate() {
            assert_eq!(t.index(), i);
            assert_eq!(t.as_u8() as usize, i + 1);
            assert!(crate::db::INVENTORY_SLOT_COLUMNS[i].ends_with(
                net::opcode::INVENTORY_SLOT_ORDER[i]
            ));
        }
        // `0x0107` carries the type as an i8, so a negative has to be refusable.
        assert_eq!(InventoryType::from_wire(1).unwrap(), InventoryType::Equip);
        assert!(InventoryType::from_wire(0).is_err());
        assert!(InventoryType::from_wire(7).is_err());
        assert!(InventoryType::from_wire(-1).is_err());
    }

    /// **Goal I.** An item taken off is in the bag afterwards, and it is not worn.
    #[test]
    fn an_unequipped_item_lands_in_the_bag_and_stays_there() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = store.create_character(account, 0, &dressed()).unwrap();

        let moved = store.unequip_to_bag(chr.id, 6, None).unwrap();
        assert_eq!(moved.item.item_id, 1060002);
        assert_eq!(moved.inv_type, InventoryType::Equip);
        assert_eq!(moved.slot, 1, "the lowest free slot");

        // The worn list no longer has it - which is what stops the next SetField re-dressing
        // the character, the exact thing the owner reported.
        let worn = store.characters_for(account, 0).unwrap();
        assert_eq!(worn[0].equips, vec![(5, 1040002), (7, 1072001), (11, 1302000)]);

        // And it is in the bag, still there on a fresh read.
        let bag = store.bag(chr.id).unwrap();
        assert_eq!(bag.items.len(), 1);
        assert_eq!(bag.items[0], moved);
        assert!(bag.items[0].item.kind.is_equip());
    }

    /// Taking an item off and putting it back on preserves its per-item stats.
    ///
    /// This is the whole reason `equipment` was given the stat columns rather than being left
    /// alone. Without them the round trip silently flattens a scrolled item.
    #[test]
    fn an_equip_keeps_its_stats_across_the_bag_and_back() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = store.create_character(account, 0, &dressed()).unwrap();

        // Scroll it while it is worn.
        let scrolled = EquipStats {
            stats: EquipStatSet { inc_pad: 9, inc_str: 4, ..EquipStatSet::default() },
            options: EquipOptions {
                remaining_enhancements: 5,
                scissor_uses: net::opcode::NO_SCISSOR_RESTRICTION,
                ..EquipOptions::default()
            },
            ..EquipStats::default()
        };
        store.unequip_to_bag(chr.id, 5, Some(3)).unwrap();
        store
            .set_inventory_slot(
                chr.id,
                InventoryType::Equip,
                3,
                &Item { item_id: 1040002, kind: ItemKind::Equip(Some(scrolled)), failed_slots: 0, pet_id: None, rolled_base: None },
            )
            .unwrap();

        store.equip_from_bag(chr.id, 3, 5).unwrap();
        let worn = store.equipped_items(chr.id).unwrap();
        let coat = worn.iter().find(|e| e.slot == 5).expect("it went back on");
        assert_eq!(coat.stats, Some(scrolled), "the scrolls survived the round trip");

        // And back off again, still intact.
        let back = store.unequip_to_bag(chr.id, 5, None).unwrap();
        assert_eq!(back.item.kind, ItemKind::Equip(Some(scrolled)));
    }

    /// **The failed-slot count survives the same round trip**, and it is a separate test
    /// because it travels by a separate mechanism.
    ///
    /// `Item::failed_slots` is server-only - it is deliberately NOT in `EquipStats`, so the
    /// test above would pass with the count being silently reset on every equip. It nearly
    /// was: the `equipment` INSERT wrote only the stat columns, so the `DEFAULT 0` would have
    /// taken over the moment a player put the item back on, and every Clean Slate they had
    /// earned on it would be gone. On screen that reads as "the scroll did nothing".
    ///
    /// The owner, 2026-09-09: *"If the item previously had 2 failed scroll slots, the player is
    /// allowed to use 2 clean slate scrolls on the item."* That promise only holds if the
    /// count outlives the bag.
    ///
    /// **The rolled base rides the same mechanism** (`Item::rolled_base`, 2026-09-24): an
    /// Innocence reverts a mob-dropped equip to it, so losing it on an equip would turn a good
    /// roll into the template the first time the player scrolled it.
    #[test]
    fn the_failed_slot_count_survives_the_bag_and_back() {
        let rolled = EquipStatSet { inc_pdd: 23, inc_str: 2, ..EquipStatSet::default() };
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = store.create_character(account, 0, &dressed()).unwrap();

        store.unequip_to_bag(chr.id, 5, Some(3)).unwrap();
        store
            .set_inventory_slot(
                chr.id,
                InventoryType::Equip,
                3,
                &Item {
                    item_id: 1040002,
                    kind: ItemKind::Equip(Some(EquipStats::default())),
                    failed_slots: 2,
                    pet_id: None,
                    rolled_base: Some(rolled),
                },
            )
            .unwrap();
        // Read straight back out of the bag first, so a failure here separates "the bag lost
        // it" from "the equip round trip lost it".
        let in_bag = store.bag_items(chr.id, InventoryType::Equip).unwrap();
        let row = in_bag.iter().find(|i| i.slot == 3).expect("it is in the bag");
        assert_eq!(row.item.failed_slots, 2, "the bag row kept the count");
        assert_eq!(row.item.rolled_base, Some(rolled), "and the rolled base");

        store.equip_from_bag(chr.id, 3, 5).unwrap();
        assert_eq!(store.worn_item(chr.id, 5).unwrap().unwrap().3, Some(rolled), "worn, it is still there");
        let back = store.unequip_to_bag(chr.id, 5, None).unwrap();
        assert_eq!(back.item.failed_slots, 2, "on, then off, and the count is still 2");
        assert_eq!(back.item.rolled_base, Some(rolled), "on, then off, and the rolled base too");

        // And a malformed value reads as "never rolled", not as an item with no stats.
        assert_eq!(rolled_base_from(Some("1,2,3".into())), None);
        assert_eq!(rolled_base_from(Some(match rolled_base_text(&Some(rolled)) {
            Value::Text(t) => t,
            v => panic!("{v:?}"),
        })), Some(rolled), "the text round-trips");
    }

    /// Equipping over a worn item **swaps** it into the slot the new one came from.
    ///
    /// The owner, 2026-08-21: *"Equipping another equipment (while having a current equipment in
    /// the same slot) should swap the current equipment with the one being replaced."* This
    /// call used to return `SlotOccupied`, on the reasoning that a swap needs a free bag slot
    /// and that finding one was the caller's job. It needs no free slot: the outgoing item
    /// takes the incoming one's place, inside the same transaction.
    #[test]
    fn equipping_over_a_worn_item_swaps_the_two() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = store.create_character(account, 0, &dressed()).unwrap();

        // A second coat in bag slot 4, while 1040002 is worn in slot 5.
        store
            .set_inventory_slot(chr.id, InventoryType::Equip, 4, &Item::equip(1040001))
            .unwrap();

        let done = store.equip_from_bag(chr.id, 4, 5).unwrap();
        assert_eq!(done.equipped, 1040001);
        assert_eq!(done.displaced, Some(1040002), "the coat that came off");

        let worn = store.equipped_items(chr.id).unwrap();
        assert_eq!(worn.iter().find(|e| e.slot == 5).unwrap().item_id, 1040001);

        // The old coat is in slot 4 - the one the new coat vacated - and nowhere else.
        let bag = store.bag(chr.id).unwrap();
        let equips: Vec<_> =
            bag.items.iter().filter(|i| i.inv_type == InventoryType::Equip).collect();
        assert_eq!(equips.len(), 1, "one item moved, one item arrived, none were cloned");
        assert_eq!(equips[0].slot, 4);
        assert_eq!(equips[0].item.item_id, 1040002);
    }

    /// The swap carries the displaced item's per-item stats back into the bag.
    ///
    /// The old code read `item_id` alone from `equipment`, because all it did with the answer
    /// was refuse. Selecting the stat tail is the difference between a scrolled item
    /// surviving a swap and coming back flattened - the same failure the unequip round trip
    /// above exists to prevent.
    #[test]
    fn a_swap_does_not_flatten_the_item_it_displaces() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        let chr = store.create_character(account, 0, &dressed()).unwrap();

        let scrolled = EquipStats {
            options: EquipOptions {
                remaining_enhancements: 5,
                scissor_uses: net::opcode::NO_SCISSOR_RESTRICTION,
                ..EquipOptions::default()
            },
            ..EquipStats::default()
        };
        // Put a scrolled coat ON, via the bag, then equip a plain one over it.
        store.unequip_to_bag(chr.id, 5, Some(1)).unwrap();
        store
            .set_inventory_slot(
                chr.id,
                InventoryType::Equip,
                1,
                &Item { item_id: 1040002, kind: ItemKind::Equip(Some(scrolled)), failed_slots: 0, pet_id: None, rolled_base: None },
            )
            .unwrap();
        store.equip_from_bag(chr.id, 1, 5).unwrap();
        store
            .set_inventory_slot(chr.id, InventoryType::Equip, 2, &Item::equip(1040001))
            .unwrap();

        let done = store.equip_from_bag(chr.id, 2, 5).unwrap();
        assert_eq!(done.displaced, Some(1040002));
        let back = store.inventory_slot(chr.id, InventoryType::Equip, 2).unwrap().unwrap();
        assert_eq!(back.kind, ItemKind::Equip(Some(scrolled)), "the scrolls survived the swap");
    }

    /// NULL stats are "derive from the template", not "an item with no stats".
    #[test]
    fn stats_that_were_never_stored_read_back_as_none_not_as_zeros() {
        let (store, _, chr) = store_with_character();
        store
            .set_inventory_slot(chr, InventoryType::Equip, 1, &Item::equip(1040002))
            .unwrap();
        let back = store.inventory_slot(chr, InventoryType::Equip, 1).unwrap().unwrap();
        assert_eq!(back.kind, ItemKind::Equip(None));

        // Explicit zeros are a different, storable state.
        store
            .set_inventory_slot(
                chr,
                InventoryType::Equip,
                2,
                &Item { item_id: 1040002, kind: ItemKind::Equip(Some(EquipStats::default())), failed_slots: 0, pet_id: None, rolled_base: None },
            )
            .unwrap();
        let zeroed = store.inventory_slot(chr, InventoryType::Equip, 2).unwrap().unwrap();
        assert_eq!(zeroed.kind, ItemKind::Equip(Some(EquipStats::default())));
        assert_ne!(zeroed.kind, back.kind, "None and Some(zeros) must not collapse");
    }

    /// Every stat column round-trips. A column silently missing from the writer or the reader
    /// is exactly the failure the exhaustive destructure exists to prevent, and a test that
    /// only checks two fields would not see it.
    #[test]
    fn every_stored_stat_survives_the_round_trip() {
        let (store, _, chr) = store_with_character();
        let stats = EquipStats {
            stats: EquipStatSet {
                inc_str: 1,
                inc_dex: 2,
                inc_int: 3,
                inc_luk: 4,
                inc_mhp: 5,
                inc_mmp: 6,
                inc_speed: 7,
                inc_jump: 8,
                inc_pad: 9,
                inc_mad: 10,
                inc_pdd: 11,
                inc_mdd: 12,
                inc_acc: 13,
                inc_eva: 14,
                inc_crt: 15,
                inc_crd: 16,
                inc_wat: 17,
            },
            options: EquipOptions {
                remaining_enhancements: 18,
                attribute: 19,
                golden_hammer: 20,
                level_requirement_reduction: 21,
                boss_damage_percent: 22,
                ignore_enemy_def_percent: 23,
                damage_percent: 24,
                all_stats_percent: 25,
                scissor_uses: 26,
                ..EquipOptions::default()
            },
            ..EquipStats::default()
        };
        store
            .set_inventory_slot(
                chr,
                InventoryType::Equip,
                1,
                &Item { item_id: 1302000, kind: ItemKind::Equip(Some(stats)), failed_slots: 0, pet_id: None, rolled_base: None },
            )
            .unwrap();
        let back = store.inventory_slot(chr, InventoryType::Equip, 1).unwrap().unwrap();
        assert_eq!(back.kind, ItemKind::Equip(Some(stats)));
        // 1..=26, one per column, so a duplicated or transposed column name shows up as a
        // wrong number rather than passing.
        assert_eq!(equip_stat_values(&stats).to_vec(), (1..=26).collect::<Vec<i64>>());
        assert_eq!(EQUIP_STAT_COLUMNS.len(), EQUIP_STAT_COLUMN_COUNT);
        let mut names: Vec<&str> = EQUIP_STAT_COLUMNS.to_vec();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), EQUIP_STAT_COLUMN_COUNT, "a column name is duplicated");
    }

    /// Slots are 1-based; slot 0 is the client's unused hole and must not be storable.
    #[test]
    fn slot_zero_is_not_a_slot() {
        let (store, _, chr) = store_with_character();
        assert!(matches!(
            store.set_inventory_slot(chr, InventoryType::Use, 0, &Item::bundle(2000000, 1)),
            Err(StoreError::SlotOutOfRange { slot: 0, .. })
        ));
        let slots = store.inventory_slots(chr, InventoryType::Use).unwrap();
        assert!(matches!(
            store.set_inventory_slot(chr, InventoryType::Use, slots + 1, &Item::bundle(2000000, 1)),
            Err(StoreError::SlotOutOfRange { .. })
        ));
    }

    #[test]
    fn a_bundle_stacks_up_to_its_slot_max_then_takes_a_second_slot() {
        let (store, _, chr) = store_with_character();
        let changed = store
            .add_item(chr, InventoryType::Use, &Item::bundle(2000000, 250), 100)
            .unwrap();
        assert_eq!(changed.len(), 3, "100 + 100 + 50");
        let bag = store.bag(chr).unwrap();
        let counts: Vec<u16> =
            bag.items_in(InventoryType::Use).map(|i| i.item.kind.quantity()).collect();
        assert_eq!(counts, vec![100, 100, 50]);

        // Adding more tops the part-full slot up first.
        store.add_item(chr, InventoryType::Use, &Item::bundle(2000000, 60), 100).unwrap();
        let counts: Vec<u16> = store
            .bag(chr)
            .unwrap()
            .items_in(InventoryType::Use)
            .map(|i| i.item.kind.quantity())
            .collect();
        assert_eq!(counts, vec![100, 100, 100, 10]);
    }

    /// **Consolidate, as arithmetic.** 50 / 80 / 90 of one item at cap 100 becomes 100 / 100
    /// / 20 in place; two singles of another become 2 and an empty slot; an equip, a pet and
    /// a cap-1 item are untouched; a stack already full is untouched and reported as nothing;
    /// then everything after the emptied slot slides up one, in order.
    #[test]
    fn consolidation_pours_later_stacks_into_earlier_ones_up_to_the_cap_then_slides_up() {
        let s = |slot, item_id, quantity, cap| Stack { slot, item_id, quantity, cap };
        let plan = plan_consolidation(&[
            s(1, 4_000_000, 50, 100),
            s(2, 4_000_000, 80, 100),
            s(3, 4_000_001, 1, 100),
            s(4, 4_000_001, 1, 100),
            s(5, 4_000_000, 90, 100),
            s(6, 1_302_000, 1, 1), // an equip
            s(7, 1_302_000, 1, 1), // another of the same equip: still two rows
            s(8, 4_000_002, 1, 1), // slotMax 1 - the two never merge
            s(9, 4_000_002, 1, 1),
            s(10, 4_000_003, 100, 100), // already full
            s(11, 4_000_003, 100, 100),
        ]);
        assert_eq!(
            plan,
            vec![
                StackChange::Quantity { slot: 1, quantity: 100 },
                StackChange::Quantity { slot: 2, quantity: 100 },
                StackChange::Quantity { slot: 3, quantity: 2 },
                StackChange::Emptied { slot: 4 },
                StackChange::Quantity { slot: 5, quantity: 20 },
                StackChange::Moved { from: 5, to: 4 },
                StackChange::Moved { from: 6, to: 5 },
                StackChange::Moved { from: 7, to: 6 },
                StackChange::Moved { from: 8, to: 7 },
                StackChange::Moved { from: 9, to: 8 },
                StackChange::Moved { from: 10, to: 9 },
                StackChange::Moved { from: 11, to: 10 },
            ]
        );
        assert!(plan_consolidation(&[]).is_empty());
        assert!(plan_consolidation(&[s(1, 4_000_000, 3, 100)]).is_empty(), "one stack in slot 1: nothing to do");
        // Slot order, not row order: the EARLIER slot receives, and what is left slides.
        assert_eq!(
            plan_consolidation(&[s(9, 4_000_000, 10, 100), s(2, 4_000_000, 10, 100)]),
            vec![StackChange::Quantity { slot: 2, quantity: 20 }, StackChange::Emptied { slot: 9 }, StackChange::Moved { from: 2, to: 1 }]
        );
        // The owner's screen: the blue potion two rows down slides up to sit after the last item.
        assert_eq!(
            plan_consolidation(&[s(1, 2_000_000, 54, 100), s(2, 2_010_000, 21, 100), s(11, 2_000_002, 100, 100)]),
            vec![StackChange::Moved { from: 11, to: 3 }]
        );
    }

    /// **Sort, as arithmetic.** The owner's Use tab (arrows 325 and 350, red 54, orange 21, scroll
    /// 6, apple 3, orange 7, blue 100) plus a second red stack to merge: after the merges
    /// (the two reds, the two oranges) and the slide, the swaps put the biggest stack first
    /// and break ties by name.
    #[test]
    fn sort_merges_slides_then_puts_the_biggest_stack_first_and_ties_by_name() {
        let s = |slot, item_id, quantity| Stack { slot, item_id, quantity, cap: 100 };
        let name = |id: u32| -> String {
            match id {
                2_060_000 => "Arrow for Bow",
                2_061_000 => "Arrow for Crossbow",
                2_000_000 => "Red Potion",
                2_010_000 => "Orange",
                2_040_000 => "Scroll",
                2_010_001 => "Apple",
                2_000_002 => "Blue Potion",
                _ => "?",
            }
            .to_string()
        };
        let plan = plan_sort(
            &[
                s(1, 2_060_000, 325),
                s(2, 2_061_000, 350),
                s(3, 2_000_000, 54),
                s(4, 2_010_000, 21),
                s(5, 2_040_000, 6),
                s(6, 2_010_001, 3),
                s(7, 2_010_000, 7),
                s(11, 2_000_002, 100),
                s(12, 2_000_000, 46), // merges into slot 3 -> red 100
            ],
            &name,
        );
        // Replay the plan on a model of the tab and check the END state, which is the claim;
        // the exact swap sequence is an implementation detail.
        let mut tab: std::collections::BTreeMap<u16, (u32, u16)> = [
            (1, (2_060_000, 325)), (2, (2_061_000, 350)), (3, (2_000_000, 54)), (4, (2_010_000, 21)),
            (5, (2_040_000, 6)), (6, (2_010_001, 3)), (7, (2_010_000, 7)), (11, (2_000_002, 100)),
            (12, (2_000_000, 46)),
        ]
        .into_iter()
        .collect();
        let mut phase = 0; // 0 quantities/empties, 1 slides, 2 swaps - never backwards
        for c in &plan {
            match *c {
                StackChange::Quantity { slot, quantity } => { assert_eq!(phase, 0); tab.get_mut(&slot).unwrap().1 = quantity; }
                StackChange::Emptied { slot } => { assert_eq!(phase, 0); tab.remove(&slot).unwrap(); }
                StackChange::Moved { from, to } => {
                    assert!(phase <= 1); phase = 1;
                    let v = tab.remove(&from).unwrap();
                    assert!(tab.insert(to, v).is_none(), "a slide lands on an empty slot");
                }
                StackChange::Swapped { a, b } => {
                    assert!(phase <= 2); phase = 2;
                    let (x, y) = (tab.remove(&a).expect("occupied"), tab.remove(&b).expect("occupied"));
                    tab.insert(a, y);
                    tab.insert(b, x);
                }
            }
        }
        let rows: Vec<(u16, u32, u16)> = tab.iter().map(|(k, (id, q))| (*k, *id, *q)).collect();
        assert_eq!(
            rows,
            vec![
                (1, 2_061_000, 350), // Arrow for Crossbow
                (2, 2_060_000, 325), // Arrow for Bow
                (3, 2_000_002, 100), // Blue Potion   - 100 ties with red: "Blue" before "Red"
                (4, 2_000_000, 100), // Red Potion
                (5, 2_010_000, 28),  // Orange - the two stacks merged first
                (6, 2_040_000, 6),   // Scroll
                (7, 2_010_001, 3),   // Apple
            ]
        );
        assert!(plan.iter().filter(|c| matches!(c, StackChange::Swapped { .. })).count() <= 7, "at most n - 1 swaps");
        // Already sorted: nothing at all.
        assert!(plan_sort(&[s(1, 2_061_000, 350), s(2, 2_060_000, 325)], &name).is_empty());
        // A tie on quantity AND name (two full stacks of one item) keeps slot order.
        assert!(plan_sort(&[s(1, 2_010_000, 100), s(2, 2_010_000, 100)], &name).is_empty());
    }

    /// **And on the rows.** The transaction writes exactly the plan: the quantities land, the
    /// emptied slot is gone, the equip and the pet are where they were, and a second call
    /// finds nothing to do.
    #[test]
    fn consolidate_bag_writes_the_plan_and_is_idempotent() {
        let store = Store::open_in_memory().unwrap();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = store.create_character(account, 0, &Character { name: "Sorter".into(), ..Default::default() }).unwrap();
        let etc = InventoryType::Etc;
        for (slot, item) in [
            (1u16, Item::bundle(4_000_000, 50)),
            (2, Item::bundle(4_000_000, 80)),
            (3, Item::bundle(4_000_001, 1)),
            (4, Item::bundle(4_000_001, 1)),
            (5, Item::bundle(4_000_000, 90)),
            (9, Item::bundle(4_000_004, 7)),
        ] {
            set_slot(&store.conn(), chr.id, etc, slot, &item).unwrap();
        }
        set_slot(&store.conn(), chr.id, InventoryType::Equip, 1, &Item::equip(1_302_000)).unwrap();
        set_slot(&store.conn(), chr.id, InventoryType::Equip, 2, &Item::equip(1_302_000)).unwrap();
        let cap = |_: u32| 100u16;
        let changes = store.consolidate_bag(chr.id, etc, &cap).unwrap();
        assert_eq!(changes.len(), 7, "{changes:?}");
        let rows: Vec<(u16, u32, u16)> = store
            .bag_items(chr.id, etc)
            .unwrap()
            .into_iter()
            .map(|i| (i.slot, i.item.item_id, i.item.kind.quantity()))
            .collect();
        assert_eq!(rows, vec![(1, 4_000_000, 100), (2, 4_000_000, 100), (3, 4_000_001, 2), (4, 4_000_000, 20), (5, 4_000_004, 7)]);
        assert_eq!(store.bag_items(chr.id, InventoryType::Equip).unwrap().len(), 2, "equips are not stacks");
        assert!(store.consolidate_bag(chr.id, etc, &cap).unwrap().is_empty(), "nothing left to merge");
        assert!(store.consolidate_bag(chr.id, InventoryType::Equip, &cap).unwrap().is_empty());

        // Sort the same tab: 100 / 100 / 2 / 20 / 7 -> 100, 100, 20, 7, 2 with the swaps
        // written through `swap_slots`, and a second sort finds nothing.
        let name = |id: u32| format!("item {id}");
        let changes = store.sort_bag(chr.id, etc, &cap, &name).unwrap();
        assert!(changes.iter().all(|c| matches!(c, StackChange::Swapped { .. })), "{changes:?}");
        let rows: Vec<(u16, u32, u16)> = store
            .bag_items(chr.id, etc)
            .unwrap()
            .into_iter()
            .map(|i| (i.slot, i.item.item_id, i.item.kind.quantity()))
            .collect();
        assert_eq!(rows, vec![(1, 4_000_000, 100), (2, 4_000_000, 100), (3, 4_000_000, 20), (4, 4_000_004, 7), (5, 4_000_001, 2)]);
        assert!(store.sort_bag(chr.id, etc, &cap, &name).unwrap().is_empty());
    }

    /// Equips never stack, whatever `max_stack` says.
    #[test]
    fn two_equips_take_two_slots() {
        let (store, _, chr) = store_with_character();
        store.add_item(chr, InventoryType::Equip, &Item::equip(1040002), 200).unwrap();
        let second = store.add_item(chr, InventoryType::Equip, &Item::equip(1040002), 200).unwrap();
        assert_eq!(second[0].slot, 2);
        assert_eq!(store.bag(chr).unwrap().items.len(), 2);
    }

    /// A bag that cannot take the whole purchase takes none of it. A partial add would leave
    /// the client and the database disagreeing about what was bought.
    #[test]
    fn an_add_that_does_not_fit_changes_nothing() {
        let (store, _, chr) = store_with_character();
        let slots = store.inventory_slots(chr, InventoryType::Etc).unwrap();
        // Fill every slot but one.
        for slot in 1..slots {
            store
                .set_inventory_slot(chr, InventoryType::Etc, slot, &Item::bundle(4000001, 1))
                .unwrap();
        }
        let before = store.bag(chr).unwrap();
        assert!(matches!(
            store.add_item(chr, InventoryType::Etc, &Item::bundle(4000019, 5), 1),
            Err(StoreError::BagFull { .. })
        ));
        assert_eq!(store.bag(chr).unwrap(), before, "nothing was committed");
    }

    #[test]
    fn a_move_into_an_empty_slot_moves_and_into_a_full_one_swaps() {
        let (store, _, chr) = store_with_character();
        store.set_inventory_slot(chr, InventoryType::Etc, 1, &Item::bundle(4000000, 3)).unwrap();
        assert_eq!(
            store.move_item(chr, InventoryType::Etc, 1, 5, None, 200).unwrap(),
            MoveOutcome::Moved
        );
        assert!(store.inventory_slot(chr, InventoryType::Etc, 1).unwrap().is_none());
        assert_eq!(
            store.inventory_slot(chr, InventoryType::Etc, 5).unwrap().unwrap().kind.quantity(),
            3
        );

        store.set_inventory_slot(chr, InventoryType::Etc, 2, &Item::bundle(4000001, 7)).unwrap();
        assert_eq!(
            store.move_item(chr, InventoryType::Etc, 2, 5, None, 200).unwrap(),
            MoveOutcome::Swapped
        );
        let a = store.inventory_slot(chr, InventoryType::Etc, 2).unwrap().unwrap();
        let b = store.inventory_slot(chr, InventoryType::Etc, 5).unwrap().unwrap();
        assert_eq!((a.item_id, a.kind.quantity()), (4000000, 3));
        assert_eq!((b.item_id, b.kind.quantity()), (4000001, 7));
    }

    #[test]
    fn two_stacks_of_the_same_item_merge_and_a_full_destination_swaps_instead() {
        let (store, _, chr) = store_with_character();
        store.set_inventory_slot(chr, InventoryType::Use, 1, &Item::bundle(2000000, 40)).unwrap();
        store.set_inventory_slot(chr, InventoryType::Use, 2, &Item::bundle(2000000, 70)).unwrap();
        assert_eq!(
            store.move_item(chr, InventoryType::Use, 1, 2, None, 100).unwrap(),
            MoveOutcome::Merged { moved: 30, remaining: 10, destination: 100 }
        );
        assert_eq!(
            store.inventory_slot(chr, InventoryType::Use, 2).unwrap().unwrap().kind.quantity(),
            100
        );
        assert_eq!(
            store.inventory_slot(chr, InventoryType::Use, 1).unwrap().unwrap().kind.quantity(),
            10
        );
        // The destination is now at slotMax, so the same drag has to swap rather than
        // silently do nothing.
        assert_eq!(
            store.move_item(chr, InventoryType::Use, 1, 2, None, 100).unwrap(),
            MoveOutcome::Swapped
        );
        assert_eq!(
            store.inventory_slot(chr, InventoryType::Use, 1).unwrap().unwrap().kind.quantity(),
            100
        );
    }

    /// A merge that empties the source leaves no row behind - an empty slot is the absence of
    /// a row, so a zero-quantity one would be a second representation of the same thing.
    #[test]
    fn a_merge_that_empties_the_source_removes_the_row() {
        let (store, _, chr) = store_with_character();
        store.set_inventory_slot(chr, InventoryType::Use, 1, &Item::bundle(2000000, 5)).unwrap();
        store.set_inventory_slot(chr, InventoryType::Use, 2, &Item::bundle(2000000, 5)).unwrap();
        assert_eq!(
            store.move_item(chr, InventoryType::Use, 1, 2, None, 100).unwrap(),
            MoveOutcome::Merged { moved: 5, remaining: 0, destination: 10 }
        );
        assert!(store.inventory_slot(chr, InventoryType::Use, 1).unwrap().is_none());
    }

    #[test]
    fn taking_more_than_the_slot_holds_is_refused_rather_than_clamped() {
        let (store, _, chr) = store_with_character();
        store.set_inventory_slot(chr, InventoryType::Use, 1, &Item::bundle(2000000, 3)).unwrap();
        assert!(matches!(
            store.remove_item(chr, InventoryType::Use, 1, Some(4)),
            Err(StoreError::NotEnoughItems { have: 3, want: 4, .. })
        ));
        let took = store.remove_item(chr, InventoryType::Use, 1, Some(2)).unwrap();
        assert_eq!(took.kind.quantity(), 2);
        assert_eq!(
            store.inventory_slot(chr, InventoryType::Use, 1).unwrap().unwrap().kind.quantity(),
            1
        );
        store.remove_item(chr, InventoryType::Use, 1, None).unwrap();
        assert!(store.inventory_slot(chr, InventoryType::Use, 1).unwrap().is_none());
    }

    /// A bought expansion survives a relog - which is the only thing that makes it a purchase.
    #[test]
    fn a_slot_expansion_persists_and_is_clamped_to_the_clients_range() {
        let (store, _, chr) = store_with_character();
        assert_eq!(store.set_inventory_slots(chr, InventoryType::Use, 48).unwrap(), 48);
        assert_eq!(store.inventory_slots(chr, InventoryType::Use).unwrap(), 48);
        // And only that one moved.
        assert_eq!(
            store.inventory_slots(chr, InventoryType::Etc).unwrap(),
            net::opcode::DEFAULT_INVENTORY_SLOTS
        );
        // The owner's floor and ceiling.
        assert_eq!(
            store.set_inventory_slots(chr, InventoryType::Use, 1).unwrap(),
            net::opcode::MIN_INVENTORY_SLOTS
        );
        assert_eq!(
            store.set_inventory_slots(chr, InventoryType::Use, 9000).unwrap(),
            net::opcode::MAX_INVENTORY_SLOTS
        );
    }

    /// Shrinking a bag past an occupied slot would strand the item where the client can never
    /// draw it. Refused, loudly.
    #[test]
    fn a_shrink_that_would_strand_an_item_is_refused() {
        let (store, _, chr) = store_with_character();
        store.set_inventory_slots(chr, InventoryType::Etc, 60).unwrap();
        store.set_inventory_slot(chr, InventoryType::Etc, 55, &Item::bundle(4000000, 1)).unwrap();
        assert!(matches!(
            store.set_inventory_slots(chr, InventoryType::Etc, 40),
            Err(StoreError::SlotOutOfRange { slot: 55, slots: 40 })
        ));
        assert_eq!(store.inventory_slots(chr, InventoryType::Etc).unwrap(), 60);
    }

    /// A purchase is one transaction: a bag that cannot take the item does not take the mesos
    /// either, and a purse that cannot pay does not hand over the item.
    #[test]
    fn a_purchase_that_fails_on_either_half_changes_neither() {
        let (store, _, chr) = store_with_character();
        store.set_mesos(chr, 100).unwrap();

        // Cannot pay: the item must not arrive.
        assert!(matches!(
            store.buy_item(chr, InventoryType::Use, &Item::bundle(2000000, 1), 100, 101),
            Err(StoreError::NotEnoughMesos { have: 100, .. })
        ));
        assert!(store.bag(chr).unwrap().is_empty(), "an unpaid item must not arrive");
        assert_eq!(store.mesos(chr).unwrap(), 100);

        // Cannot carry: the mesos must not leave.
        let slots = store.inventory_slots(chr, InventoryType::Etc).unwrap();
        for slot in 1..=slots {
            store
                .set_inventory_slot(chr, InventoryType::Etc, slot, &Item::bundle(4000001, 1))
                .unwrap();
        }
        assert!(matches!(
            store.buy_item(chr, InventoryType::Etc, &Item::bundle(4000019, 1), 1, 10),
            Err(StoreError::BagFull { .. })
        ));
        assert_eq!(store.mesos(chr).unwrap(), 100, "an undelivered item must not be charged");

        // And the successful case moves both.
        let got = store
            .buy_item(chr, InventoryType::Use, &Item::bundle(2000000, 4), 100, 40)
            .unwrap();
        assert_eq!(got[0].item.kind.quantity(), 4);
        assert_eq!(store.mesos(chr).unwrap(), 60);
    }

    /// **The owner's other restriction.** A quest item cannot be sold, and the refusal leaves it in
    /// the bag rather than eating it.
    #[test]
    fn a_quest_item_cannot_be_sold_and_the_refusal_keeps_it() {
        let (store, _, chr) = store_with_character();
        let quest_item =
            *ItemRules::QUEST_ITEMS.first().expect("the client ships 119 quest items");
        store
            .set_inventory_slot(chr, InventoryType::Etc, 1, &Item::bundle(quest_item, 3))
            .unwrap();
        assert!(matches!(
            store.sell_item(chr, InventoryType::Etc, 1, None, 5),
            Err(StoreError::ItemMayNotBeSold { item_id }) if item_id == quest_item
        ));
        assert_eq!(
            store.inventory_slot(chr, InventoryType::Etc, 1).unwrap().unwrap().kind.quantity(),
            3
        );
        assert_eq!(store.mesos(chr).unwrap(), 0);

        // A plain item does sell, and the price is per unit.
        store.set_inventory_slot(chr, InventoryType::Etc, 2, &Item::bundle(4000001, 6)).unwrap();
        assert_eq!(store.sell_item(chr, InventoryType::Etc, 2, Some(4), 5).unwrap(), 20);
        assert_eq!(
            store.inventory_slot(chr, InventoryType::Etc, 2).unwrap().unwrap().kind.quantity(),
            2
        );
    }

    #[test]
    fn mesos_never_go_negative_and_a_refused_spend_changes_nothing() {
        let (store, _, chr) = store_with_character();
        assert_eq!(store.mesos(chr).unwrap(), 0);
        assert_eq!(store.add_mesos(chr, 500).unwrap(), 500);
        assert!(matches!(
            store.add_mesos(chr, -501),
            Err(StoreError::NotEnoughMesos { have: 500, .. })
        ));
        assert_eq!(store.mesos(chr).unwrap(), 500, "the refusal committed nothing");
        assert_eq!(store.add_mesos(chr, -500).unwrap(), 0);
        store.set_mesos(chr, 12345).unwrap();
        assert_eq!(store.mesos(chr).unwrap(), 12345);
    }

    /// Deleting a character takes its bag with it, the way it already takes its equipment and
    /// its quests.
    #[test]
    fn deleting_a_character_takes_its_bag() {
        let (store, account, chr) = store_with_character();
        store.set_inventory_slot(chr, InventoryType::Etc, 1, &Item::bundle(4000000, 1)).unwrap();
        // Prove the row exists first - "no orphans" is also what a failed insert looks like.
        assert_eq!(inventory_rows(&store), 1);
        store.delete_character(account, chr).unwrap();
        assert_eq!(inventory_rows(&store), 0, "ON DELETE CASCADE should have taken it");
    }

    #[test]
    fn one_character_never_sees_anothers_bag() {
        let (store, _, first) = store_with_character();
        let other = store.create_account("someone_else", "correct horse battery").unwrap();
        let second = store
            .create_character(
                other,
                0,
                &Character { name: "Second".into(), ..Character::default() },
            )
            .unwrap()
            .id;
        store.set_inventory_slot(first, InventoryType::Etc, 1, &Item::bundle(4000000, 1)).unwrap();
        assert!(store.bag(second).unwrap().is_empty());
        assert_eq!(store.bag(first).unwrap().items.len(), 1);
    }

    /// The generated id lists must match the client's own item table.
    ///
    /// **This is the drift instrument** for the baked rules. `gm-handbook/itemdata.txt` is
    /// generated and gitignored, so this test is skipped where it is absent - the same shape
    /// `world::shops`'s real-file tests use. Regenerate with:
    ///
    /// ```text
    /// python tools/dump_itemdata.py
    /// python tools/gen_item_rules.py
    /// ```
    #[test]
    fn the_baked_item_rules_still_match_the_clients_own_table() {
        let path = std::path::Path::new("../../gm-handbook/itemdata.txt");
        if !path.exists() {
            return; // generated data, gitignored - tools/dump_itemdata.py
        }
        let text = std::fs::read_to_string(path).unwrap();
        let (mut blocked, mut quest, mut rows) = (Vec::new(), Vec::new(), 0usize);
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            // Five columns until 2026-09-06, six since (`unitPrice`, a float this test does
            // not read). Only the first five are integers.
            if f.len() < 5 {
                continue;
            }
            let n: Vec<u32> = match f[..5].iter().map(|x| x.parse::<u32>()).collect() {
                Ok(v) => v,
                Err(_) => continue,
            };
            rows += 1;
            if n[2] != 0 {
                quest.push(n[0]);
            }
            if n[3] != 0 {
                blocked.push(n[0]);
            }
        }
        // The instrument speaks before it is believed: these are the numbers STATUS.md goal F
        // and tools/dump_itemdata.py both report, so a parser that silently read nothing would
        // fail here rather than agreeing with an empty baked list.
        // 2785 was the classic client alone. 2863 is with the 78 Signature Style Collection
        // items backported into client-patched/Data on 2026-09-10 (backport/signature-style);
        // 2871 with the four collaboration pets and their four weapons (2026-09-17). 2878 with
        // the seven backported scrolls (2026-10-01): Pure Clean Slate x4, Chaos, Innocence
        // (as 2049190) and the Lucky Day Scroll - none of them quest or tradeBlock items.
        assert_eq!(rows, 2878, "the item table changed size");
        assert_eq!(blocked.len(), 39, "39 items carry tradeBlock");
        assert_eq!(quest.len(), 119, "119 items carry quest");

        assert_eq!(
            ItemRules::TRADE_BLOCKED,
            blocked.as_slice(),
            "the baked trade-block list is stale - run: python tools/gen_item_rules.py"
        );
        assert_eq!(
            ItemRules::QUEST_ITEMS,
            quest.as_slice(),
            "the baked quest-item list is stale - run: python tools/gen_item_rules.py"
        );
        assert_eq!(ItemRules::GENERATED_FROM_ITEMS, rows);
    }

    /// The baked lists have to be sorted, because the lookup binary-searches them. An unsorted
    /// list would answer "not blocked" for a blocked item and nothing would say so.
    #[test]
    fn the_baked_lists_are_sorted_and_the_lookup_agrees_with_a_linear_scan() {
        assert!(ItemRules::TRADE_BLOCKED.windows(2).all(|w| w[0] < w[1]));
        assert!(ItemRules::QUEST_ITEMS.windows(2).all(|w| w[0] < w[1]));
        for &id in ItemRules::TRADE_BLOCKED {
            assert!(ItemRules::trade_blocked(id));
            assert!(!ItemRules::may_be_stored(id));
        }
        for &id in ItemRules::QUEST_ITEMS {
            assert!(ItemRules::quest_item(id));
            assert!(!ItemRules::may_be_sold(id));
        }
        // A positive control for the negative: an id that is in neither list.
        let free = 2000000;
        assert!(!ItemRules::TRADE_BLOCKED.contains(&free));
        assert!(ItemRules::may_be_stored(free) && ItemRules::may_be_sold(free));
    }

    fn inventory_rows(store: &Store) -> i64 {
        store
            .conn()
            .query_row("SELECT COUNT(*) FROM inventory", [], |row| row.get(0))
            .unwrap()
    }
}

#[cfg(test)]
mod item_id_rename_tests {
    use super::*;

    /// A box bought under Nexon's id, in a bag, a cash locker and a storage box, reads back
    /// under the classic client's id after the open-time rewrite - and a second run changes
    /// nothing, because the live server re-runs it on every start.
    #[test]
    fn a_box_under_the_old_id_is_rewritten_in_every_item_table_and_the_rewrite_is_idempotent() {
        let store = crate::Store::open_in_memory().unwrap();
        let account = store.create_account("maplecw", "correct horse battery").unwrap();
        let chr = net::opcode::Character { name: "Wanderer".to_string(), ..Default::default() };
        let id = store.create_character(account, 0, &chr).unwrap().id;
        store.add_item(id, InventoryType::Cash, &Item::bundle(5_222_221, 1), 1).unwrap();
        {
            let conn = store.conn();
            // The other two tables, written directly: what matters is the rewrite, not the
            // API that would normally put a row there.
            // kind 2 = a bundle, the box's kind; slot 1 is the first locker/storage slot.
            for table in ["cash_locker", "storage_item"] {
                conn.execute(
                    &format!("INSERT INTO {table} (account_id, slot, item_id, kind, quantity) VALUES (?1, 1, 5222221, 2, 1)"),
                    rusqlite::params![account],
                )
                .unwrap();
            }
            rename_item_ids(&conn).unwrap();
            rename_item_ids(&conn).unwrap(); // idempotent
            for table in ["inventory", "cash_locker", "storage_item"] {
                let old: i64 = conn.query_row(&format!("SELECT count(*) FROM {table} WHERE item_id = 5222221"), [], |r| r.get(0)).unwrap();
                let new: i64 = conn.query_row(&format!("SELECT count(*) FROM {table} WHERE item_id = 5681599"), [], |r| r.get(0)).unwrap();
                assert_eq!((old, new), (0, 1), "{table}");
            }
            // `equipment` is visited too; nothing is worn here, so it simply stays empty.
            let worn: i64 = conn.query_row("SELECT count(*) FROM equipment", [], |r| r.get(0)).unwrap();
            assert_eq!(worn, 0);
        }
        let cash: Vec<u32> = store.bag(id).unwrap().items_in(InventoryType::Cash).map(|i| i.item.item_id).collect();
        assert_eq!(cash, vec![5_681_599]);
    }

    /// Every table with an `item_id` column is in [`ITEM_ID_TABLES`] - a new table that
    /// carries item ids would otherwise be skipped by the rename in silence.
    #[test]
    fn every_table_with_an_item_id_column_is_renamed() {
        let store = crate::Store::open_in_memory().unwrap();
        let conn = store.conn();
        let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type = 'table'").unwrap();
        let tables: Vec<String> = stmt.query_map([], |r| r.get(0)).unwrap().map(|r| r.unwrap()).collect();
        let mut with_item_id = Vec::new();
        for t in &tables {
            let mut cols = conn.prepare(&format!("PRAGMA table_info({t})")).unwrap();
            let names: Vec<String> = cols.query_map([], |r| r.get(1)).unwrap().map(|r| r.unwrap()).collect();
            if names.iter().any(|n| n == "item_id") {
                with_item_id.push(t.clone());
            }
        }
        with_item_id.sort();
        let mut expected: Vec<String> = ITEM_ID_TABLES.iter().map(|s| s.to_string()).collect();
        expected.sort();
        assert_eq!(with_item_id, expected, "tables carrying item_id vs the rename list");
    }

    // -- empty star stacks (the owner, 2026-10-02) ------------------------------------------

    fn rogue(store: &Store) -> u32 {
        let account = store.create_account("wisp", "correct horse battery").unwrap();
        store.create_character(account, 0, &net::opcode::Character { name: "Pebble".into(), ..Default::default() }).unwrap().id
    }

    /// **A star stack that runs out stays at 0; an arrow quiver that runs out is gone.** Every
    /// other path still treats a slot that reaches zero as empty.
    #[test]
    fn spending_the_last_star_keeps_the_stack_and_the_last_arrow_does_not() {
        let store = Store::open_in_memory().unwrap();
        let id = rogue(&store);
        store.set_inventory_slot(id, InventoryType::Use, 1, &Item::bundle(2_070_000, 3)).unwrap();
        store.set_inventory_slot(id, InventoryType::Use, 2, &Item::bundle(2_060_000, 1)).unwrap();
        assert_eq!(store.spend_ammo(id, InventoryType::Use, 1, 3).unwrap(), 0);
        assert_eq!(store.inventory_slot(id, InventoryType::Use, 1).unwrap(), Some(Item::bundle(2_070_000, 0)));
        assert_eq!(store.spend_ammo(id, InventoryType::Use, 2, 1).unwrap(), 0);
        assert_eq!(store.inventory_slot(id, InventoryType::Use, 2).unwrap(), None, "arrows have no recharge");
        assert!(store.spend_ammo(id, InventoryType::Use, 1, 1).is_err(), "nothing to spend from an empty stack");
        // A potion can never be stored at 0 - the CHECK still says so.
        assert!(store.set_inventory_slot(id, InventoryType::Use, 3, &Item::bundle(2_000_000, 0)).is_err());
    }

    /// **The empty stack round-trips the floor and the shop**: taking it (a drop or a sale) clears
    /// the slot whatever count is named; putting it back (a pickup) takes a free slot of its
    /// own at 0 rather than vanishing. More of the same star does NOT fill it - the owner,
    /// 2026-10-03, every star stack is its own - and a recharge of that slot does.
    #[test]
    fn an_empty_star_stack_can_be_taken_put_back_and_refilled() {
        let store = Store::open_in_memory().unwrap();
        let id = rogue(&store);
        store.set_inventory_slot(id, InventoryType::Use, 1, &Item::bundle(2_070_001, 0)).unwrap();
        assert_eq!(store.remove_item(id, InventoryType::Use, 1, Some(1)).unwrap(), Item::bundle(2_070_001, 0));
        assert_eq!(store.inventory_slot(id, InventoryType::Use, 1).unwrap(), None);
        let placed = store.add_item(id, InventoryType::Use, &Item::bundle(2_070_001, 0), 500).unwrap();
        assert_eq!(placed.len(), 1, "a pickup of an empty stack lands somewhere");
        assert_eq!(placed[0].item, Item::bundle(2_070_001, 0));
        let slot = placed[0].slot;
        let more = store.add_item(id, InventoryType::Use, &Item::bundle(2_070_001, 200), 500).unwrap();
        assert_eq!(more.len(), 1);
        assert_ne!(more[0].slot, slot, "more of the same star takes a slot of its own");
        assert_eq!(store.inventory_slot(id, InventoryType::Use, slot).unwrap(), Some(Item::bundle(2_070_001, 0)), "the empty stack is untouched");
        store.add_mesos(id, 1_000).unwrap();
        let full = store.recharge_slot(id, slot, 2_070_001, 500, 200).unwrap();
        assert_eq!((full.slot, full.item), (slot, Item::bundle(2_070_001, 500)), "the recharge fills the slot it names");
    }

    /// **A star pickup never adds to a stack already in the bag** (the owner, 2026-10-03, about
    /// Wolbi), partial or not - while a potion still does. And a recharge tops up the slot it
    /// names, not the lowest partial stack of the same star.
    #[test]
    fn a_star_pickup_is_its_own_stack_and_a_recharge_fills_the_slot_named() {
        let store = Store::open_in_memory().unwrap();
        let id = rogue(&store);
        store.set_inventory_slot(id, InventoryType::Use, 1, &Item::bundle(2_070_001, 40)).unwrap();
        store.set_inventory_slot(id, InventoryType::Use, 2, &Item::bundle(2_000_000, 10)).unwrap();
        let star = store.add_item(id, InventoryType::Use, &Item::bundle(2_070_001, 3), 500).unwrap();
        assert_eq!(star.iter().map(|r| (r.slot, r.item)).collect::<Vec<_>>(), vec![(3, Item::bundle(2_070_001, 3))]);
        assert_eq!(store.inventory_slot(id, InventoryType::Use, 1).unwrap(), Some(Item::bundle(2_070_001, 40)), "the partial stack is untouched");
        let potion = store.add_item(id, InventoryType::Use, &Item::bundle(2_000_000, 5), 100).unwrap();
        assert_eq!(potion.iter().map(|r| (r.slot, r.item)).collect::<Vec<_>>(), vec![(2, Item::bundle(2_000_000, 15))], "potions still stack");

        store.add_mesos(id, 1_000).unwrap();
        let before = store.mesos(id).unwrap();
        let full = store.recharge_slot(id, 3, 2_070_001, 500, 199).unwrap();
        assert_eq!((full.slot, full.item), (3, Item::bundle(2_070_001, 500)));
        assert_eq!(store.inventory_slot(id, InventoryType::Use, 1).unwrap(), Some(Item::bundle(2_070_001, 40)), "not the lower stack");
        assert_eq!(store.mesos(id).unwrap(), before - 199);
        assert!(store.recharge_slot(id, 3, 2_070_001, 500, 1).is_err(), "a full stack is refused");
        assert!(store.recharge_slot(id, 2, 2_000_000, 100, 1).is_err(), "a potion is not recharged");
        assert!(store.recharge_slot(id, 1, 2_070_001, 500, 1_000_000).is_err(), "too few mesos");
        assert_eq!(store.inventory_slot(id, InventoryType::Use, 1).unwrap(), Some(Item::bundle(2_070_001, 40)), "and a refusal moves nothing");
    }

    /// **Consolidating keeps an empty star stack** as a stack that slides, not a hole another
    /// stack can slide into.
    #[test]
    fn consolidation_keeps_an_empty_star_stack() {
        let s = |slot, item_id, quantity, cap| Stack { slot, item_id, quantity, cap };
        assert_eq!(
            plan_consolidation(&[s(2, 2_070_000, 0, 500), s(5, 2_000_000, 10, 100)]),
            vec![StackChange::Moved { from: 2, to: 1 }, StackChange::Moved { from: 5, to: 2 }],
            "the empty stack slides up and holds slot 1; the potion sits after it"
        );
    }

    /// **A deployed `inventory` is rebuilt under the new rule, once, with every row, column and
    /// index intact.** The table here is created with the exact pre-2026-10-02 CHECK, holding a
    /// star stack and an equip; after `create_tables` an empty star stack is accepted, an empty
    /// potion still refused, the rows read back unchanged, the index is back, and a second open
    /// is a no-op.
    #[test]
    fn a_deployed_inventory_table_is_rebuilt_to_allow_empty_star_stacks() {
        let store = Store::open_in_memory().unwrap();
        let id = rogue(&store);
        store.set_inventory_slot(id, InventoryType::Use, 1, &Item::bundle(2_070_000, 37)).unwrap();
        store.set_inventory_slot(id, InventoryType::Equip, 1, &Item::equip(1_302_000)).unwrap();
        {
            // Put the old rule back by rebuilding the table the other way, the way the live
            // database still has it.
            let conn = store.conn();
            let sql: String = conn
                .query_row("SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'inventory'", [], |r| r.get(0))
                .unwrap();
            assert!(sql.contains(QUANTITY_CHECK), "a fresh table has the new rule");
            let old = sql.replacen(QUANTITY_CHECK, OLD_QUANTITY_CHECK, 1).replacen("inventory", "inventory_old", 1);
            conn.execute_batch(&old).unwrap();
            conn.execute_batch(
                "INSERT INTO inventory_old SELECT * FROM inventory; DROP TABLE inventory;
                 ALTER TABLE inventory_old RENAME TO inventory;",
            )
            .unwrap();
            let refused = conn.execute(
                "UPDATE inventory SET quantity = 0 WHERE character_id = ?1 AND inv_type = 2 AND slot = 1",
                [i64::from(id)],
            );
            assert!(refused.is_err(), "positive control: the old rule refuses an empty star stack");
            create_tables(&conn).unwrap();
            create_tables(&conn).unwrap(); // the second open: nothing left to do
            let sql: String = conn
                .query_row("SELECT sql FROM sqlite_master WHERE type = 'table' AND name = 'inventory'", [], |r| r.get(0))
                .unwrap();
            assert!(sql.contains(QUANTITY_CHECK) && !sql.contains(OLD_QUANTITY_CHECK), "{sql}");
            let index: i64 = conn
                .query_row("SELECT count(*) FROM sqlite_master WHERE type = 'index' AND name = 'idx_inventory_character'", [], |r| r.get(0))
                .unwrap();
            assert_eq!(index, 1, "the index came back");
        }
        assert_eq!(store.inventory_slot(id, InventoryType::Use, 1).unwrap(), Some(Item::bundle(2_070_000, 37)));
        assert_eq!(store.inventory_slot(id, InventoryType::Equip, 1).unwrap().map(|i| i.item_id), Some(1_302_000));
        assert_eq!(store.spend_ammo(id, InventoryType::Use, 1, 37).unwrap(), 0, "and now it may reach 0");
        assert!(store.set_inventory_slot(id, InventoryType::Use, 2, &Item::bundle(2_000_000, 0)).is_err());
    }
}
