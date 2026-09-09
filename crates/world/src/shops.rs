//! NPC shop contents: `data/shops.txt`, resolved to item ids at load time.
//!
//! # Why this file loads *authored* data and every other loader reads *generated* data
//!
//! Everything else in [`crate::config`] comes out of the client's own WZ - portals, NPCs,
//! mobs, equip stats, quest text. Shop contents do not, and that is a measured fact rather
//! than a gap someone has not got to yet: `Data/Etc/Script/Script_000.wz` contains **no
//! images at all**, none of `Etc_000.wz`'s 67 images is a shop list, and `Npc.wz` carries
//! no shop node reachable from an NPC template. STATUS.md goal F has the control for each.
//!
//! So `data/shops.txt` is transcribed by hand from the live game and committed as source.
//! Hand-transcribed game data is exactly what this project has been burned by, so **nothing
//! here trusts it**: every name is resolved against `gm-handbook/items.txt`, which is
//! generated from the client's own `String.wz`, and every resolved id is then required to
//! exist in `gm-handbook/itemdata.txt`. A row that fails either check is **dropped and
//! reported**, never guessed at.
//!
//! # Two prices, and only one of them is in the client
//!
//! | | where it comes from | |
//! |---|---|---|
//! | **buy** - what the NPC charges | `data/shops.txt` | authored; not in the client at all |
//! | **sell** - what the NPC pays | `info/price` via `itemdata.txt` | measured |
//!
//! They are not derivable from each other. Every consumable checked is exactly 10.00x its WZ
//! price (Red Potion 5 -> 50, Meat 8 -> 80, nine of nine), and then Pet Food is 15 -> 35. A
//! rule with a counterexample is not a rule.
//!
//! # The two restrictions the owner set, as table lookups
//!
//! > *"Please do not allow quest items to be sold."* -> [`ItemData::may_be_sold`], from
//! > `info/quest`.
//!
//! > *"Please do not allow untradeable items to be stored."* -> [`ItemData::may_be_stored`],
//! > from `info/tradeBlock`.
//!
//! Both are properties the client ships, so neither is invented and neither comes from a fan
//! site. `may_be_stored` has no caller yet - storage is goal G and does not exist - and it
//! lives here because it is the same table and the same measurement.
//!
//! # What this deliberately does NOT do
//!
//! It does not build the shop packet. The dialog/buy/sell protocol is not decoded in either
//! direction, and it does not resolve an NPC *name* to an NPC *template id* - `shops.txt`
//! names the NPC the way the live UI does, and nothing yet maps that onto the templates in
//! `gm-handbook/npcstrings.txt`. Both are the shop-packet task's, and until they exist this
//! table is data with no wire behind it.

use std::collections::HashMap;
use std::path::Path;

/// Citizenship grades 1..=10, in order, as the live UI names them.
///
/// The rank tag on a `shops.txt` row is one of these plus a `+`, meaning "this grade or
/// higher", so `CITIZENSHIP_GRADES[g - 1]` is grade `g`'s name.
///
/// **The system is the client's own, the numbering is corroborated, the level thresholds are
/// not ours to check.** All 88 citizenship quests are in this client's `Quest.wz` (ids
/// `506000`-`506141`, count matches exactly) and `citizenshipGrade` / `citizenshipTown` are
/// real `Check` keys with 86 uses each - so the concept and the grade axis are **[L]**. The
/// names below are corroborated by being exactly the tags the owner transcribed off the live UI.
/// The grade-to-character-level mapping (Traveler 12, Visitor 17, ... Citizen of Honor 57)
/// is **[I]** from a fan site and is deliberately *not* encoded here: this module carries the
/// grade a row requires, and nothing else. See STATUS.md goal H.
pub const CITIZENSHIP_GRADES: [&str; 10] = [
    "Traveler",
    "Visitor",
    "Helpful Stranger",
    "Recognized Guest",
    "Town Resident",
    "Trusted Neighbor",
    "Distinguished Citizen",
    "Town Patron",
    "Guardian of the Village",
    "Citizen of Honor",
];

/// `"Town Resident+"` -> `Some(5)`. `None` for anything not in [`CITIZENSHIP_GRADES`].
///
/// The trailing `+` is optional on input because it carries no information - a gate is always
/// "this grade or higher" - but it is what the live UI shows and what the file records.
///
/// **`None` must never be treated as "no requirement".** That is the whole failure mode this
/// function exists to make visible: a gate that silently degrades to ungated would let a
/// level-12 character buy a Citizen-of-Honor scroll, and nothing downstream would ever say
/// so. [`ShopTable::load`] drops such a row and reports it.
pub fn citizenship_grade(tag: &str) -> Option<u8> {
    let tag = tag.trim().trim_end_matches('+').trim();
    CITIZENSHIP_GRADES
        .iter()
        .position(|g| g.eq_ignore_ascii_case(tag))
        .map(|i| i as u8 + 1)
}

/// Which characters may wear an equip, decided by the id alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
    Unisex,
    /// Not an equip, so the question does not apply.
    NotAnEquip,
}

/// Equips encode gender in the id: `(id / 1000) % 10` is 0 male, 1 female, 2+ unisex.
///
/// **Measured in this client, not taken from the family's usual shape.** `data/shops.txt`
/// carries 97 rows with an explicit `(M)` or `(F)` tag, **90 of them transcribed off the live
/// UI**, and there is **not one** whose tag disagrees with the gender its id implies - `Green
/// Bennis Chainmail (M)` is 1040033 and `(F)` is 1041040. The client agrees from the other
/// side too: its own `String.wz` ships `Natalie's Fashion Box (M)` / `(F)` at 2430001 /
/// 2430002 with the suffix baked into the name itself. `python tools/check_shops.py
/// --gender-audit` re-runs the check and is the control for every tagged row.
///
/// A tag is a hint that narrows, **not** an assertion about the item: what makes the live UI
/// add one is not established (`Archer Pants (M)` is unisex and tagged; `Green Hunter's
/// Pants` is male-only and is not), so a tag on an unambiguous name is accepted and ignored.
pub fn gender_of(item_id: u32) -> Gender {
    if !(1_000_000..2_000_000).contains(&item_id) {
        return Gender::NotAnEquip;
    }
    match (item_id / 1000) % 10 {
        0 => Gender::Male,
        1 => Gender::Female,
        _ => Gender::Unisex,
    }
}

impl Gender {
    /// Does this id satisfy a `(M)` / `(F)` tag? Unisex and non-equips satisfy both.
    fn satisfies(self, tag: Gender) -> bool {
        matches!(self, Gender::Unisex | Gender::NotAnEquip) || self == tag
    }
}

/// One item's measured properties, from `tools/dump_itemdata.py`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ItemData {
    /// `info/price` - the **sell** price, what an NPC pays. Not the buy price.
    pub price: u32,
    /// `info/quest`. A quest item.
    pub quest: bool,
    /// `info/tradeBlock`. Untradeable.
    pub trade_block: bool,
    /// `info/slotMax` - stack size. **0 means the property is absent**, which is every equip
    /// (equips do not stack) and most consumables in this client - 290 of 2785 items carry
    /// one. 0 is not "cannot hold any".
    pub slot_max: u16,
    /// `info/unitPrice`, in **thousandths of a meso per unit** - the recharge price of a
    /// throwing star or bullet. `0` for everything that is not rechargeable, which is every
    /// item outside `207xxxx` and `233xxxx`.
    ///
    /// **[L]**, read out of `Item/Consume/0207.img` on 2026-09-06: Subi 0.3, Wolbi 0.4,
    /// Mokbi 0.5, Kumbi 0.6, Tobi 0.7, Steely 0.8, Ilbi 0.9, Hwabi 1.0, and 1.0 for the
    /// three event stars. Kept as an integer so this struct stays `Eq`; the wire wants an
    /// IEEE double and `net::classicshop` converts at the one place it is written.
    pub unit_price_milli: u32,
}

impl ItemData {
    /// The owner: *"Please do not allow quest items to be sold."*
    pub fn may_be_sold(&self) -> bool {
        !self.quest
    }

    /// The owner: *"Please do not allow untradeable items to be stored."*
    ///
    /// **The rule that binds is `store::storage`'s, not this one.** `Store::store_item` and
    /// its siblings refuse a trade-blocked item from their own baked id list, so a caller
    /// cannot get round it by not asking. This copy exists because it is the same
    /// measurement out of the same dump, and because a shop that could offer to store one
    /// should not draw the option in the first place - the same shape as `may_be_sold`,
    /// which keeps a quest item out of the Sell tab rather than only refusing the sale.
    pub fn may_be_stored(&self) -> bool {
        !self.trade_block
    }
}

/// One row of one shop, after its name has been resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShopItem {
    pub item_id: u32,
    /// The name as `shops.txt` spells it, tag and all. Kept for logging: an id in a log line
    /// is unreadable and this is the only place the two are side by side.
    pub name: String,
    /// What the NPC charges. Authored, undiscounted.
    pub buy_price: u32,
    /// What the NPC pays, from the client. Copied here so a sell needs no second lookup.
    pub sell_price: u32,
    /// The citizenship grade 1..=10 a character needs, if this row is gated.
    pub min_grade: Option<u8>,
    pub quest_item: bool,
    pub trade_blocked: bool,
}

/// One NPC's shop.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Shop {
    /// As the live UI names the NPC. **Not** a template id; see the module docs.
    pub npc: String,
    /// "Grocer", "Armor Seller", ... Free text from the transcription.
    pub role: String,
    /// The map as the owner wrote it down, e.g. "Victoria Road: Perion Weapon Store". A label,
    /// not a map id - the transcription names streets, and matching those onto `Map.wz`
    /// entries is a separate job with its own ambiguity.
    pub map_label: String,
    pub items: Vec<ShopItem>,
}

/// Every shop, plus everything that went wrong loading them.
#[derive(Debug, Clone, Default)]
pub struct ShopTable {
    pub shops: Vec<Shop>,
    /// Every item's measured properties, by id - **not just the ones a shop sells**.
    ///
    /// It is kept here because this is where it is already loaded, and because the two
    /// callers that need it are the same file's shop rows and the inventory move that has to
    /// know whether two items in a bag stack. `slot_max` is `0` for an item with no
    /// `info/slotMax`, which is every equip; see [`ItemData::slot_max`].
    pub item_data: HashMap<u32, ItemData>,
    /// One line per row that could not be loaded, ready to print.
    ///
    /// **Nothing is dropped quietly.** A shop row that cannot be resolved is a row that
    /// would otherwise sell nothing, or worse sell the wrong item, and the whole reason
    /// `data/shops.txt` holds names rather than ids is so that failure is visible. A default
    /// that silently does nothing has already cost this project a client launch.
    pub problems: Vec<String>,
}

/// `name -> ids`, from `gm-handbook/items.txt`. Several ids can share a name.
pub fn load_item_names(path: &Path) -> HashMap<String, Vec<u32>> {
    let mut out: HashMap<String, Vec<u32>> = HashMap::new();
    let Ok(text) = std::fs::read_to_string(path) else { return out };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((id, name)) = line.split_once(',') else { continue };
        let (Ok(id), name) = (id.trim().parse::<u32>(), name.trim()) else { continue };
        if !name.is_empty() {
            out.entry(name.to_string()).or_default().push(id);
        }
    }
    out
}

/// `id -> ItemData`, from `gm-handbook/itemdata.txt`.
///
/// A row with the wrong column count is skipped rather than partially read: a half-filled
/// row would put a wrong price or a wrong quest flag into a rule that has no second check
/// behind it. Same rule as [`crate::config::Config::load_equips`].
pub fn load_item_data(path: &Path) -> HashMap<u32, ItemData> {
    let mut out = HashMap::new();
    let Ok(text) = std::fs::read_to_string(path) else { return out };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split(',').map(str::trim).collect();
        // Five columns until 2026-09-06, six since (`unitPrice`, a float). A five-column
        // file is an older generation and still loads; its stars simply cannot be recharged,
        // which the shop log says per row.
        if f.len() < 5 {
            continue;
        }
        let n: Vec<Option<u32>> = f[..5].iter().map(|x| x.parse::<u32>().ok()).collect();
        if n.iter().any(Option::is_none) {
            continue;
        }
        let v: Vec<u32> = n.into_iter().map(Option::unwrap).collect();
        // `0.3` -> 300. Rounded, not truncated: 0.3 is not exactly representable and
        // `(0.3 * 1000.0) as u32` is 299 on some paths, which would be the "unit, not the
        // arithmetic" class of error one notch down.
        let unit_price_milli = f
            .get(5)
            .and_then(|x| x.parse::<f64>().ok())
            .filter(|p| p.is_finite() && *p >= 0.0)
            .map(|p| (p * 1000.0).round() as u32)
            .unwrap_or(0);
        out.insert(
            v[0],
            ItemData {
                price: v[1],
                quest: v[2] != 0,
                trade_block: v[3] != 0,
                slot_max: u16::try_from(v[4]).unwrap_or(u16::MAX),
                unit_price_milli,
            },
        );
    }
    out
}

/// Why a shop row's name did not become exactly one item id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameProblem {
    /// No item in the client carries this name, tagged or untagged.
    Unknown,
    /// Several ids carry it and nothing in the row picks between them.
    Ambiguous(Vec<u32>),
    /// A `(M)` / `(F)` tag that no candidate id satisfies. The tag is wrong, or the
    /// gender-from-id convention does not hold for this item - either way it is a finding,
    /// not a missing item, and it is worth a different message.
    TagMatchesNothing(Vec<u32>),
}

/// One shop-row name -> one item id.
///
/// The name is looked up **whole first**, and only then is a trailing `(M)` / `(F)` peeled
/// off and used to filter. That order matters: the client ships at least one item whose real
/// name ends in `(M)` (`Natalie's Fashion Box (M)`, 2430001), and stripping first would
/// mangle it into a filter over a name that does not exist.
pub fn resolve_item_name(
    name: &str,
    names: &HashMap<String, Vec<u32>>,
) -> Result<u32, NameProblem> {
    if let Some(ids) = names.get(name) {
        return match ids.as_slice() {
            [one] => Ok(*one),
            many => Err(NameProblem::Ambiguous(many.to_vec())),
        };
    }
    let Some((base, tag)) = split_gender_tag(name) else { return Err(NameProblem::Unknown) };
    let Some(ids) = names.get(base) else { return Err(NameProblem::Unknown) };
    let fit: Vec<u32> = ids.iter().copied().filter(|i| gender_of(*i).satisfies(tag)).collect();
    match fit.as_slice() {
        [one] => Ok(*one),
        [] => Err(NameProblem::TagMatchesNothing(ids.clone())),
        many => Err(NameProblem::Ambiguous(many.to_vec())),
    }
}

/// `"Green Bennis Chainmail (M)"` -> `("Green Bennis Chainmail", Male)`.
fn split_gender_tag(name: &str) -> Option<(&str, Gender)> {
    let base = name.strip_suffix(')')?;
    let (base, tag) = base.rsplit_once('(')?;
    let tag = match tag {
        "M" => Gender::Male,
        "F" => Gender::Female,
        _ => return None,
    };
    Some((base.trim_end(), tag))
}

impl ShopTable {
    /// Read `data/shops.txt` and resolve every row against the two generated tables.
    ///
    /// A missing file is not fatal and never has been in this crate - the server runs with no
    /// shops and says so - but it is not silent either: the caller is expected to print
    /// [`Self::problems`], and [`Self::is_empty`] is what the startup warning tests.
    pub fn load(shops: &Path, item_names: &Path, item_data: &Path) -> ShopTable {
        let names = load_item_names(item_names);
        let data = load_item_data(item_data);
        let mut table = ShopTable { item_data: data.clone(), ..ShopTable::default() };

        if names.is_empty() {
            table.problems.push(format!(
                "no item name table at {} - NO shop row can be resolved. Regenerate with: \
                 python tools/dump_names.py",
                item_names.display()
            ));
        }
        if data.is_empty() {
            table.problems.push(format!(
                "no item data at {} - sell prices and the quest/tradeBlock rules are \
                 unavailable. Regenerate with: python tools/dump_itemdata.py",
                item_data.display()
            ));
        }
        let Ok(text) = std::fs::read_to_string(shops) else {
            table.problems.push(format!(
                "no shop file at {} - every NPC shop will be empty",
                shops.display()
            ));
            return table;
        };

        for (lineno, raw) in text.lines().enumerate() {
            let lineno = lineno + 1;
            let line = raw.trim_end();
            if line.trim().is_empty() || line.trim_start().starts_with('#') {
                continue;
            }
            if let Some(header) = line.strip_prefix("shop:") {
                let mut parts = header.split('|').map(str::trim);
                table.shops.push(Shop {
                    npc: parts.next().unwrap_or_default().to_string(),
                    role: parts.next().unwrap_or_default().to_string(),
                    map_label: parts.next().unwrap_or_default().to_string(),
                    items: Vec::new(),
                });
                continue;
            }
            // The name only, not the shop: every message below names the NPC, and holding a
            // borrow on `table.shops` while pushing to `table.problems` would not compile.
            let Some(npc) = table.shops.last().map(|s| s.npc.clone()) else {
                table.problems.push(format!(
                    "{}:{lineno}: an item row before any `shop:` header - dropped: {}",
                    shops.display(),
                    line.trim()
                ));
                continue;
            };

            let cells: Vec<&str> =
                line.trim_start_matches('\t').split('\t').map(str::trim).collect();
            if cells.len() < 2 {
                table.problems.push(format!(
                    "{}:{lineno} [{npc}]: not a `name<tab>price` row - dropped: {}",
                    shops.display(),
                    line.trim()
                ));
                continue;
            }
            let name = cells[0];
            let Ok(buy_price) = cells[1].replace(',', "").parse::<u32>() else {
                table.problems.push(format!(
                    "{}:{lineno} [{npc}] {name}: price {:?} is not a number - dropped",
                    shops.display(),
                    cells[1]
                ));
                continue;
            };

            // The gate is parsed BEFORE the name, and an unrecognised grade drops the row.
            // Loading it ungated would turn a Citizen-of-Honor item into one anybody can
            // buy, which is the failure nothing downstream could ever report.
            let rank = cells.get(2).copied().unwrap_or("").trim();
            let min_grade = if rank.is_empty() {
                None
            } else {
                match citizenship_grade(rank) {
                    Some(g) => Some(g),
                    None => {
                        table.problems.push(format!(
                            "{}:{lineno} [{npc}] {name}: rank {rank:?} is not a citizenship \
                             grade - DROPPED rather than sold ungated. Known grades: {}",
                            shops.display(),
                            CITIZENSHIP_GRADES.join(", ")
                        ));
                        continue;
                    }
                }
            };

            let item_id = match resolve_item_name(name, &names) {
                Ok(id) => id,
                Err(NameProblem::Unknown) => {
                    table.problems.push(format!(
                        "{}:{lineno} [{npc}] {name}: no item of this name in the client's \
                         String.wz - dropped. `python tools/check_shops.py` lists these",
                        shops.display()
                    ));
                    continue;
                }
                Err(NameProblem::Ambiguous(ids)) => {
                    table.problems.push(format!(
                        "{}:{lineno} [{npc}] {name}: {} items share this name ({}) and \
                         nothing picks between them - dropped rather than guessed. A (M)/(F) \
                         tag settles a gender pair",
                        shops.display(),
                        ids.len(),
                        join_ids(&ids)
                    ));
                    continue;
                }
                Err(NameProblem::TagMatchesNothing(ids)) => {
                    table.problems.push(format!(
                        "{}:{lineno} [{npc}] {name}: the (M)/(F) tag matches none of {} - \
                         the tag is wrong, or gender-from-id does not hold for this item",
                        shops.display(),
                        join_ids(&ids)
                    ));
                    continue;
                }
            };

            let Some(measured) = data.get(&item_id).copied() else {
                table.problems.push(format!(
                    "{}:{lineno} [{npc}] {name}: resolved to {item_id}, which has no row in \
                     the item table - the client has the NAME but not the item. Dropped: a \
                     sell price and the quest/tradeBlock rules are unavailable for it",
                    shops.display()
                ));
                continue;
            };

            // Not a drop. The owner's rule is about the SELL direction, and a shop that stocks a
            // quest item is a transcription worth looking at rather than a rule violation -
            // as it happens no shop in the file does, and this line is what would say so.
            if measured.quest {
                table.problems.push(format!(
                    "{}:{lineno} [{npc}] {name} ({item_id}) is a QUEST item. Stocked, but it \
                     may never be sold back - ItemData::may_be_sold",
                    shops.display()
                ));
            }

            let item = ShopItem {
                item_id,
                name: name.to_string(),
                buy_price,
                sell_price: measured.price,
                min_grade,
                quest_item: measured.quest,
                trade_blocked: measured.trade_block,
            };
            table.shops.last_mut().expect("checked by the `npc` binding above").items.push(item);
        }
        table
    }

    pub fn is_empty(&self) -> bool {
        self.shops.iter().all(|s| s.items.is_empty())
    }

    /// Total rows loaded across every shop.
    pub fn item_count(&self) -> usize {
        self.shops.iter().map(|s| s.items.len()).sum()
    }

    /// The shop an NPC keeps, by the name `shops.txt` gives it.
    ///
    /// Case-insensitive because the transcription and any future caller are two different
    /// hands. This is **not** how the game will look a shop up - that needs an NPC template
    /// id and the mapping does not exist yet (see the module docs) - it is how a test and a
    /// `!shop` style command can.
    pub fn by_npc(&self, npc: &str) -> Option<&Shop> {
        self.shops.iter().find(|s| s.npc.eq_ignore_ascii_case(npc))
    }

    /// The maximum quantity one purchase may ask for. **Row offset 29.**
    ///
    /// The client uses it as the ceiling in its "How many?" box and rejects anything above
    /// it, so **`0` on the wire makes every quantity fail**. That matters here because
    /// `info/slotMax` is *absent* on 2495 of the 2785 items in `gm-handbook/itemdata.txt`,
    /// Red Potion included - the client cannot supply this number on its own.
    ///
    /// So the fallbacks are a **server policy, [I]**, not a client fact, and they live in
    /// one place so there is one thing to change when a run disagrees.
    pub fn max_per_purchase(&self, item_id: u32) -> u16 {
        self.max_stack(item_id)
    }

    /// **How many of this item fit in one bag slot.**
    ///
    /// The same rule as [`Self::max_per_purchase`] and the same numbers - a purchase is
    /// limited by the stack it has to land in - but named for the question a pick-up asks, so
    /// that a caller stacking an item has something to reach for that is not about shops.
    ///
    /// **This is the answer to "why do Garnet Ores not stack".** The owner, 2026-08-20. Two
    /// callers - the drop pick-up and `!item` - wrote their own version of this as
    /// `slot_max.max(1)`, which turns the **2495 items with no `info/slotMax`** into
    /// one-per-slot. That is right for the 1760 of them that are equips and wrong for the
    /// **187 Etc, 285 Use and 263 Cash** items that are not: Garnet Ore is `slotMax 0` and
    /// stacks perfectly well in the real game.
    ///
    /// The policy was already here and already correct; what was wrong is that two places did
    /// not call it. Hence one function with two names rather than three copies of a rule.
    pub fn max_stack(&self, item_id: u32) -> u16 {
        // **The two repurposed scrolls override the client's own number**, the owner 2026-09-09:
        // *"can we make all of these items stackable up to a 100 please?"*
        //
        // It has to be an override rather than a data edit for two reasons. `gm-handbook/` is
        // generated and must never be hand-edited - the next dump would silently undo it. And
        // the client's `info/slotMax` describes what those items *were*: `4031065` and
        // `4031066` both carry `slotMax = 1`, which is right for the quest props they are in
        // this client's data and wrong for the scrolls `crate::scrolls` turns them into.
        //
        // **[I], and the risk is on the client side - with one data point already against it.**
        // Nothing has been measured about how this client draws a stack larger than an item's
        // own `slotMax`. `4001009` Event Trophy carries `slotMax = 0` and was dropped from the
        // feature on 2026-09-09 precisely because the owner found it *"does not stack"* on screen -
        // so the override did not save that one. Whether `slotMax = 1` behaves differently from
        // `slotMax = 0` here is exactly the open question, and it is worth one look: stack a
        // few, then try splitting them. 161 of the 359 Etc items already carry `slotMax = 200`
        // if a swap turns out to be the answer.
        if crate::scrolls::Scroll::from_item_id(item_id).is_some() {
            return crate::scrolls::STACK_LIMIT;
        }
        match self.item_data.get(&item_id).map(|d| d.slot_max) {
            Some(n) if n > 0 => n,              // [L] from info/slotMax
            _ if item_id / 1_000_000 == 1 => 1, // an equip: one at a time
            _ => 100,                           // [I] a policy number
        }
    }
}

/// Join `data/shops.txt`'s NPC **names** onto the template ids the client sends.
///
/// **This was the last mile.** `data/shops.txt` names Lucy the way the live UI does, the
/// `0x00F2` click carries a template id, and until now nothing connected the two - so a
/// fully decoded shop packet had nobody to send it to. `gm-handbook/npcstrings.txt` carries
/// `21 name Lucy`, which is the join.
///
/// Returns `template -> index into shops`, plus a line per problem, ready to print. Two
/// things go in that list rather than being resolved silently:
///
/// * a shop whose NPC name matches **no** template - the shop can never open
/// * a name that matches **several** templates - every one of them gets the shop, because a
///   name collision in this data is usually the same character standing in two maps, and
///   refusing would be worse than opening the right shop in both places. The line says how
///   many, so a genuine collision is visible rather than assumed away.
pub fn resolve_npc_templates(
    table: &ShopTable,
    npc_strings: &HashMap<u32, crate::config::NpcStrings>,
) -> (HashMap<u32, usize>, Vec<String>) {
    let mut out = HashMap::new();
    let mut problems = Vec::new();
    for (i, shop) in table.shops.iter().enumerate() {
        let matches: Vec<u32> = npc_strings
            .iter()
            .filter(|(_, s)| s.name.eq_ignore_ascii_case(&shop.npc))
            .map(|(id, _)| *id)
            .collect();
        if matches.is_empty() {
            problems.push(format!(
                "shop \"{}\" ({}): no NPC template in gm-handbook/npcstrings.txt is named that, so it can never open",
                shop.npc, shop.map_label
            ));
            continue;
        }
        if matches.len() > 1 {
            problems.push(format!(
                "shop \"{}\": {} templates share that name ({}) - all of them get it",
                shop.npc,
                matches.len(),
                join_ids(&matches)
            ));
        }
        for template in matches {
            out.insert(template, i);
        }
    }
    (out, problems)
}

fn join_ids(ids: &[u32]) -> String {
    ids.iter().map(u32::to_string).collect::<Vec<_>>().join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **Both scrolls stack to 100**, overriding the client's own `slotMax`.
    ///
    /// The owner, 2026-09-09: *"can we make all of these items stackable up to a 100 please?"*
    /// Both carry `info/slotMax = 1` in `gm-handbook/itemdata.txt`, so without the override
    /// they would be one per bag slot. The data file is generated and must never be
    /// hand-edited, which is why this lives in code.
    ///
    /// Whether the client honours it is still **[I]** - see `max_stack`'s own note, and the
    /// Event Trophy that did not stack.
    #[test]
    fn the_two_scrolls_stack_to_one_hundred() {
        let shops = ShopTable::default();
        for id in crate::scrolls::REPURPOSED {
            assert_eq!(shops.max_stack(id), crate::scrolls::STACK_LIMIT, "item {id}");
        }
        // The control: the override is not simply "everything stacks". An equip still does not.
        assert_eq!(shops.max_stack(1_040_002), 1, "an equip is one per slot");
    }

    const SHOPS: &str = "../../data/shops.txt";
    const NAMES: &str = "../../gm-handbook/items.txt";
    const DATA: &str = "../../gm-handbook/itemdata.txt";

    fn index(rows: &[(&str, u32)]) -> HashMap<String, Vec<u32>> {
        let mut out: HashMap<String, Vec<u32>> = HashMap::new();
        for (name, id) in rows {
            out.entry((*name).to_string()).or_default().push(*id);
        }
        out
    }

    #[test]
    fn every_rank_tag_in_the_file_is_a_grade_and_an_unknown_one_is_not() {
        assert_eq!(citizenship_grade("Traveler+"), Some(1));
        assert_eq!(citizenship_grade("Town Resident+"), Some(5));
        assert_eq!(citizenship_grade("Guardian of the Village+"), Some(9));
        assert_eq!(citizenship_grade("Citizen of Honor+"), Some(10));
        // The `+` carries no information - a gate is always "or higher".
        assert_eq!(citizenship_grade("Town Resident"), Some(5));
        assert_eq!(citizenship_grade("  town resident +  "), Some(5));
        // And the failure that matters is a miss, not a crash.
        assert_eq!(citizenship_grade("Town Resident++"), Some(5));
        assert_eq!(citizenship_grade("Honorary Citizen+"), None);
        assert_eq!(citizenship_grade(""), None);
        assert_eq!(CITIZENSHIP_GRADES.len(), 10);
    }

    /// The rule that a bad gate must fail CLOSED. A row whose rank does not parse is
    /// dropped, not loaded ungated - the ungated version is the one nothing could detect.
    #[test]
    fn an_unrecognised_rank_drops_the_row_rather_than_dropping_the_gate() {
        let dir = std::env::temp_dir().join("maplecw-shop-rank-test");
        std::fs::create_dir_all(&dir).expect("temp dir");
        let shops = dir.join("shops.txt");
        std::fs::write(
            &shops,
            "shop: Flint | Scrolls | Henesys\n\
             \tReal Scroll\t100\tTown Resident+\n\
             \tFake Scroll\t100\tHonorary Citizen+\n",
        )
        .expect("write");
        let names = dir.join("items.txt");
        std::fs::write(&names, "2040000, Real Scroll\n2040001, Fake Scroll\n").expect("write");
        let data = dir.join("itemdata.txt");
        std::fs::write(&data, "2040000, 10, 0, 0, 100\n2040001, 10, 0, 0, 100\n").expect("write");

        let table = ShopTable::load(&shops, &names, &data);
        assert_eq!(table.item_count(), 1, "the ungated-by-accident row must not load");
        assert_eq!(table.shops[0].items[0].name, "Real Scroll");
        assert_eq!(table.shops[0].items[0].min_grade, Some(5));
        assert_eq!(table.problems.len(), 1, "and it must be reported, not swallowed");
        assert!(table.problems[0].contains("Honorary Citizen+"), "{:?}", table.problems);
        assert!(table.problems[0].contains("DROPPED"), "{:?}", table.problems);
    }

    #[test]
    fn gender_comes_from_the_id_and_unisex_satisfies_both_tags() {
        assert_eq!(gender_of(1040033), Gender::Male); // Green Bennis Chainmail (M)
        assert_eq!(gender_of(1041040), Gender::Female); // Green Bennis Chainmail (F)
        assert_eq!(gender_of(1062000), Gender::Unisex); // Archer Pants
        assert_eq!(gender_of(2000000), Gender::NotAnEquip); // Red Potion
        assert!(Gender::Unisex.satisfies(Gender::Male));
        assert!(Gender::Unisex.satisfies(Gender::Female));
        assert!(Gender::NotAnEquip.satisfies(Gender::Female));
        assert!(!Gender::Male.satisfies(Gender::Female));
    }

    #[test]
    fn a_gender_tag_picks_between_two_ids_and_a_whole_name_wins_over_a_tag() {
        let names = index(&[
            ("Green Bennis Chainmail", 1040033),
            ("Green Bennis Chainmail", 1041040),
            ("Archer Pants", 1062000),
            ("Natalie's Fashion Box (M)", 2430001),
            ("Natalie's Fashion Box (F)", 2430002),
        ]);
        assert_eq!(resolve_item_name("Green Bennis Chainmail (M)", &names), Ok(1040033));
        assert_eq!(resolve_item_name("Green Bennis Chainmail (F)", &names), Ok(1041040));
        // A tag on a unisex item is harmless.
        assert_eq!(resolve_item_name("Archer Pants (M)", &names), Ok(1062000));
        assert_eq!(resolve_item_name("Archer Pants", &names), Ok(1062000));

        // The client's own name ends in `(M)`. Peeling the tag first would look up
        // "Natalie's Fashion Box", find nothing, and report a real item as missing.
        assert_eq!(resolve_item_name("Natalie's Fashion Box (M)", &names), Ok(2430001));

        // Untagged, the pair is ambiguous and must NOT resolve to the first id.
        assert_eq!(
            resolve_item_name("Green Bennis Chainmail", &names),
            Err(NameProblem::Ambiguous(vec![1040033, 1041040]))
        );
        assert_eq!(resolve_item_name("Green Bennis Chainmale", &names), Err(NameProblem::Unknown));
    }

    /// A tag no candidate satisfies is its own diagnosis: the gender-from-id convention is
    /// measured rather than assumed, so the loader has to be able to say it did not hold.
    #[test]
    fn a_tag_that_matches_nothing_is_reported_as_such_not_as_a_missing_item() {
        let names = index(&[("Red Shark", 1041039)]);
        assert_eq!(
            resolve_item_name("Red Shark (M)", &names),
            Err(NameProblem::TagMatchesNothing(vec![1041039]))
        );
    }

    #[test]
    fn an_unresolvable_row_is_reported_rather_than_silently_skipped() {
        let dir = std::env::temp_dir().join("maplecw-shop-loud-test");
        std::fs::create_dir_all(&dir).expect("temp dir");
        let shops = dir.join("shops.txt");
        std::fs::write(
            &shops,
            "shop: Lucy | Grocer | Maple Road\n\
             \tRed Potion\t50\n\
             \tNot An Item\t50\n\
             \tRed Potion\tfree\n\
             \tPair\t50\n\
             \tNamed But Absent\t50\n",
        )
        .expect("write");
        let names = dir.join("items.txt");
        std::fs::write(
            &names,
            "2000000, Red Potion\n1040000, Pair\n1041000, Pair\n2000009, Named But Absent\n",
        )
        .expect("write");
        let data = dir.join("itemdata.txt");
        std::fs::write(&data, "2000000, 5, 0, 0, 100\n1040000, 1, 0, 0, 0\n1041000, 1, 0, 0, 0\n")
            .expect("write");

        let table = ShopTable::load(&shops, &names, &data);
        assert_eq!(table.item_count(), 1, "only Red Potion is loadable");
        assert_eq!(table.problems.len(), 4, "{:#?}", table.problems);
        let all = table.problems.join("\n");
        assert!(all.contains("Not An Item"), "unknown name reported");
        assert!(all.contains("\"free\""), "unparseable price reported");
        assert!(all.contains("share this name"), "ambiguity reported");
        assert!(all.contains("no row in the item table"), "name-without-item reported");
        // The one that loaded carries the client's sell price, not the authored buy price.
        let potion = &table.shops[0].items[0];
        assert_eq!((potion.buy_price, potion.sell_price), (50, 5));
    }

    #[test]
    fn the_two_restrictions_are_table_lookups() {
        let quest = ItemData { quest: true, ..ItemData::default() };
        let bound = ItemData { trade_block: true, ..ItemData::default() };
        let plain = ItemData { price: 5, slot_max: 100, ..ItemData::default() };
        assert!(!quest.may_be_sold(), "a quest item may not be sold to an NPC");
        assert!(quest.may_be_stored(), "but a quest item is not automatically unstorable");
        assert!(!bound.may_be_stored(), "an untradeable item may not be stored");
        assert!(bound.may_be_sold());
        assert!(plain.may_be_sold() && plain.may_be_stored());
    }

    /// The real file, against the real generated tables. This is the test that would catch a
    /// regenerated `items.txt` renaming something out from under the transcription.
    #[test]
    fn every_row_of_the_real_shop_file_resolves() {
        let (shops, names, data) = (Path::new(SHOPS), Path::new(NAMES), Path::new(DATA));
        if !names.exists() || !data.exists() {
            return; // generated data, gitignored - tools/dump_names.py, dump_itemdata.py
        }
        let table = ShopTable::load(shops, names, data);
        assert_eq!(table.shops.len(), 39, "39 shops were transcribed");
        // 932 since 2026-08-19: seven female rows were restored to Don Hwang and Nuri
        // after the coordinator had deleted them as "duplicates". They were not - they
        // are the female variants, sharing a display name and differing only in id, and
        // the live UI counts (98/98 and 36/36) are what proved it.
        assert_eq!(table.item_count(), 932, "and 932 item rows");
        assert!(
            table.problems.is_empty(),
            "every row must resolve; still open:\n{}",
            table.problems.join("\n")
        );

        // Both halves of the gender pair are stocked by the same NPC and are different items.
        let sam = table.by_npc("Sam").expect("Sam sells armour");
        let male = sam.items.iter().find(|i| i.name == "Green Bennis Chainmail (M)").unwrap();
        let female = sam.items.iter().find(|i| i.name == "Green Bennis Chainmail (F)").unwrap();
        assert_eq!((male.item_id, female.item_id), (1040033, 1041040));
        assert_eq!(male.buy_price, female.buy_price, "same price, different item");

        // The buy price is authored and the sell price is the client's, and for the one item
        // both numbers are known for they are 50 and 5 - a 10x that Pet Food does not share.
        let lucy = table.by_npc("Lucy").expect("Lucy is a grocer");
        let potion = lucy.items.iter().find(|i| i.name == "Red Potion").unwrap();
        assert_eq!((potion.item_id, potion.buy_price, potion.sell_price), (2000000, 50, 5));
        let food = lucy.items.iter().find(|i| i.name == "Pet Food").unwrap();
        assert_eq!((food.buy_price, food.sell_price), (35, 15), "the counterexample to 10x");

        // The owner's restriction, measured over the whole file: no shop stocks a quest item, so
        // "do not allow quest items to be sold" costs nothing here and is still enforced.
        assert!(table.shops.iter().flat_map(|s| &s.items).all(|i| !i.quest_item));

        // The citizenship gate survives the round trip.
        let flint = table.by_npc("Flint").expect("Flint sells scrolls");
        assert!(flint.items.iter().all(|i| i.min_grade.is_some()), "every scroll is gated");
        let lesser = flint.items.iter().find(|i| i.name.starts_with("Topwear")).unwrap();
        assert_eq!(lesser.min_grade, Some(5), "Town Resident+");
        assert!(
            table.shops.iter().flat_map(|s| &s.items).filter(|i| i.min_grade.is_some()).count()
                == 40,
            "40 gated rows across the file"
        );
    }

    /// The item table itself, read back. These are the numbers goal F and goal G turn into
    /// rules, so a change in either census should be deliberate.
    #[test]
    fn the_item_table_carries_the_two_flags_the_restrictions_need() {
        let path = Path::new(DATA);
        if !path.exists() {
            return; // generated data, gitignored - tools/dump_itemdata.py
        }
        let data = load_item_data(path);
        assert!(data.len() > 2500, "only {} items", data.len());

        // 04000000 is the worked example from STATUS.md goal F: quest 1 AND price 0.
        let quest_item = data[&4000000];
        assert!(quest_item.quest && !quest_item.may_be_sold());
        assert_eq!(quest_item.price, 0);
        assert_eq!(quest_item.slot_max, 200);

        // And the neighbours that are not quest items carry the price the same node gives.
        assert_eq!(data[&4000001].price, 1);
        assert_eq!(data[&4000002].price, 2);
        assert!(data[&4000001].may_be_sold());

        // Equips are in here too - they live under Character.wz, not Item.wz - and they do
        // not stack, so slotMax is absent rather than 1.
        let shirt = data[&1040002];
        assert_eq!(shirt.slot_max, 0, "an equip has no slotMax");
        assert!(shirt.may_be_stored());

        let quest = data.values().filter(|i| i.quest).count();
        let blocked = data.values().filter(|i| i.trade_block).count();
        assert_eq!(quest, 119, "items that may not be SOLD");
        assert_eq!(blocked, 39, "items that may not be STORED");
    }
}
