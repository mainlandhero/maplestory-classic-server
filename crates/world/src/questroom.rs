//! Room in the bag for what a quest is about to hand over - checked BEFORE the quest moves.
//!
//! Mint, 2026-09-17 (Discord, relayed by the owner): *"quest continues to complete despite this
//! happening"* - under a yellow line reading `Quest 1008 could not give you item 1002005:
//! inventory 1 is full (30 slots)`. The owner: *"When quests give you items but the inventory of
//! the player is full, the server should use the NPC dialogue and display an appropriate
//! message to say that their bag is full, please make <x> amount of spaces in <y> tab. The
//! quest should not complete if the user has a full inventory."*
//!
//! The completion was recorded, the experience paid, the letter taken back, and the one
//! reward that was the point of the quest was refused by the store and reported as a chat
//! line - the quest was over and the hat was never coming. Same shape as the Signature Style
//! box before 2026-09-10 (`session::cashitem::hand_out`): every effect must hang off one
//! decision made before anything is written, and here that decision is *"is there room for
//! all of it"*.
//!
//! # Counted the way the store will place it
//!
//! [`shortfall`] simulates `store::place_into_bag` per tab rather than counting rows: an
//! equip is a slot; a bundle tops up the stacks of the same item that have room, then takes
//! `ceil(rest / max_stack)` fresh slots; a **take** (a negative quest count) is applied first
//! and frees a slot when it empties a stack - quest 1008 takes the letter (Etc) and gives a
//! hat (Equip), so the two never meet, but a quest that takes ten of something and gives a
//! stack back to the same tab is counted honestly. The comparison is against the tab's real
//! slot count, so a 30-slot Equip tab and a 150-slot Cash tab are each their own limit.
//!
//! What it does NOT know is which of a `prop` pool's items will be drawn - so the caller
//! draws first and passes the chosen rows, and the same rows are what it hands over after
//! the check. The roll is spent either way; a refusal does not re-roll on the next click.

/// One tab that cannot take what the quest gives, and by how many slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shortfall {
    pub tab: store::InventoryType,
    pub slots: u16,
}

/// The conversation path the bag-full box is parked under, so the OK that closes it is
/// answered the way every last box is (silently, conversation cleared) and cannot be taken
/// for the quest's own `Say`. Disjoint from every other prefix `on_script_reply` claims - the
/// test below spells them out.
pub const REFUSAL_PATH: &str = "quest.bagfull";

/// The slots each tab is short by, for `gives` after `takes`, against `bag` as it is now.
/// Empty means everything fits.
///
/// `gives` and `takes` are `(item id, count, tab)`; `max_stack(item)` is `info/slotMax`
/// (`0`/`1` = does not stack), which `store` does not know and `Config::shops` does.
pub fn shortfall(
    bag: &store::Bag,
    gives: &[(u32, u16, store::InventoryType)],
    takes: &[(u32, u16, store::InventoryType)],
    max_stack: impl Fn(u32) -> u16,
) -> Vec<Shortfall> {
    let mut out = Vec::new();
    for tab in store::InventoryType::ALL {
        // (item id, quantity) per occupied slot; an equip is a stack of one that never grows.
        let mut slots: Vec<(u32, u16, bool)> = bag
            .items_in(tab)
            .map(|i| (i.item.item_id, i.item.kind.quantity(), matches!(i.item.kind, store::ItemKind::Equip(_))))
            .collect();
        for &(id, mut count, _) in takes.iter().filter(|(_, _, t)| *t == tab) {
            // From the last stack first, the way `take_quest_item` walks the bag.
            for s in slots.iter_mut().rev() {
                if count == 0 {
                    break;
                }
                if s.0 == id {
                    let taken = s.1.min(count);
                    s.1 -= taken;
                    count -= taken;
                }
            }
            slots.retain(|s| s.1 > 0);
        }
        for &(id, count, _) in gives.iter().filter(|(_, _, t)| *t == tab) {
            if tab == store::InventoryType::Equip || tab == store::InventoryType::Deco {
                for _ in 0..count.max(1) {
                    slots.push((id, 1, true));
                }
                continue;
            }
            let per = max_stack(id).max(1);
            let mut rest = count;
            for s in slots.iter_mut() {
                if rest == 0 {
                    break;
                }
                if s.0 == id && !s.2 && s.1 < per {
                    let fill = (per - s.1).min(rest);
                    s.1 += fill;
                    rest -= fill;
                }
            }
            while rest > 0 {
                let put = rest.min(per);
                slots.push((id, put, false));
                rest -= put;
            }
        }
        let used = slots.len() as u16;
        let have = bag.slots_in(tab);
        if used > have {
            out.push(Shortfall { tab, slots: used - have });
        }
    }
    out
}

/// What the NPC says. One sentence per short tab, in the words the owner asked for.
pub fn refusal_text(short: &[Shortfall]) -> String {
    let mut text = String::from("Your bag is full, so I can't give you what I promised yet.");
    for s in short {
        let plural = if s.slots == 1 { "space" } else { "spaces" };
        text.push_str(&format!(
            " Please make {} {plural} in your {} tab",
            s.slots,
            net::bag::BAG_TAB_NAMES[s.tab.index()]
        ));
        text.push('.');
    }
    text.push_str(" Then come and talk to me again.");
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use store::InventoryType as T;

    fn bag_with(slots: [u16; 6], items: &[(T, u16, u32, u16)]) -> store::Bag {
        store::Bag {
            slots,
            items: items
                .iter()
                .map(|&(t, slot, id, qty)| store::InvItem {
                    inv_type: t,
                    slot,
                    item: if t == T::Equip { store::Item::equip(id) } else { store::Item::bundle(id, qty) },
                })
                .collect(),
        }
    }

    /// **Mint's case.** Equip tab 30 of 30, quest 1008 takes the letter (Etc) and gives one
    /// hat (Equip): the Etc take frees nothing in Equip, and Equip is short by exactly one.
    #[test]
    fn a_full_equip_tab_is_short_by_one_for_one_hat_and_the_etc_take_does_not_help() {
        let items: Vec<(T, u16, u32, u16)> =
            (1..=30).map(|s| (T::Equip, s, 1_302_000, 1)).chain([(T::Etc, 1, 4_031_002, 1)]).collect();
        let bag = bag_with([30, 30, 30, 30, 150, 150], &items);
        let short = shortfall(&bag, &[(1_002_005, 1, T::Equip)], &[(4_031_002, 1, T::Etc)], |_| 1);
        assert_eq!(short, vec![Shortfall { tab: T::Equip, slots: 1 }]);
        assert_eq!(
            refusal_text(&short),
            "Your bag is full, so I can't give you what I promised yet. Please make 1 space in your Equip tab. Then come and talk to me again."
        );
        // One slot free: fits, nothing to say.
        let bag = bag_with([31, 30, 30, 30, 150, 150], &items);
        assert!(shortfall(&bag, &[(1_002_005, 1, T::Equip)], &[(4_031_002, 1, T::Etc)], |_| 1).is_empty());
    }

    /// **Stacks count the way the store fills them.** A Use tab full of 30 stacks, one of
    /// them 96 of 100 Red Potions: a gift of 4 tops it up and fits; 5 needs a slot there is
    /// not. A gift of 250 into an empty tab of 2 slots is three stacks, short by one.
    #[test]
    fn a_bundle_tops_up_its_own_stacks_before_it_needs_a_slot() {
        let mut items: Vec<(T, u16, u32, u16)> = (1..=29).map(|s| (T::Use, s, 2_000_001, 100)).collect();
        items.push((T::Use, 30, 2_000_000, 96));
        let bag = bag_with([30, 30, 30, 30, 150, 150], &items);
        let m = |_: u32| 100u16;
        assert!(shortfall(&bag, &[(2_000_000, 4, T::Use)], &[], m).is_empty(), "96 + 4 = one full stack");
        assert_eq!(shortfall(&bag, &[(2_000_000, 5, T::Use)], &[], m), vec![Shortfall { tab: T::Use, slots: 1 }]);
        let empty = bag_with([30, 2, 30, 30, 150, 150], &[]);
        assert_eq!(shortfall(&empty, &[(2_000_000, 250, T::Use)], &[], m), vec![Shortfall { tab: T::Use, slots: 1 }], "250 = 100 + 100 + 50");
        // An item that does not stack: every unit is a slot.
        assert_eq!(shortfall(&empty, &[(4_031_000, 3, T::Use)], &[], |_| 1), vec![Shortfall { tab: T::Use, slots: 1 }]);
    }

    /// A take in the same tab frees the slot it empties, and only that: taking 1 of a stack
    /// of 5 frees nothing; taking all 5 frees one.
    #[test]
    fn a_take_frees_a_slot_only_when_it_empties_the_stack() {
        let mut items: Vec<(T, u16, u32, u16)> = (1..=29).map(|s| (T::Etc, s, 4_000_000, 100)).collect();
        items.push((T::Etc, 30, 4_031_002, 5));
        let bag = bag_with([30, 30, 30, 30, 150, 150], &items);
        let give = [(4_000_001, 1, T::Etc)];
        assert_eq!(shortfall(&bag, &give, &[(4_031_002, 1, T::Etc)], |_| 100), vec![Shortfall { tab: T::Etc, slots: 1 }]);
        assert!(shortfall(&bag, &give, &[(4_031_002, 5, T::Etc)], |_| 100).is_empty());
    }

    /// Two short tabs are two sentences, and the plural is right.
    #[test]
    fn the_text_names_every_short_tab() {
        let short = [Shortfall { tab: T::Equip, slots: 2 }, Shortfall { tab: T::Etc, slots: 1 }];
        let text = refusal_text(&short);
        assert!(text.contains("Please make 2 spaces in your Equip tab."), "{text}");
        assert!(text.contains("Please make 1 space in your Etc tab."), "{text}");
        assert!(text.ends_with("Then come and talk to me again."), "{text}");
    }

    /// The parked path shares no prefix with any other the reply handler claims.
    #[test]
    fn the_refusal_path_cannot_be_confused_with_any_other_menu() {
        for other in [
            crate::signaturestyle::RECEIPT_PATH,
            crate::shanks::ANNOUNCE_PATH,
            crate::shanks::ASK_PATH,
            crate::jobs::ASK_PATH,
            "package.frieren:",
            "scroll.",
            "jobguide.menu",
        ] {
            assert!(!REFUSAL_PATH.starts_with(other) && !other.starts_with(REFUSAL_PATH), "{other}");
        }
        assert!(!REFUSAL_PATH.starts_with(|c: char| c.is_ascii_digit()), "a quest Say path starts with a digit");
    }
}
