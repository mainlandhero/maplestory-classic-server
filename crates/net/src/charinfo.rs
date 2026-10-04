//! The Character Info window for ANOTHER player - `0x01FC` in, `0x00A2` out.
//!
//! The owner, 2026-09-18: *"When double clicking another player, a similar Character Info window
//! should show for as well for players that are not yourself. I just tried double clicking on
//! Tester2 to display the Character Info window as the owner."* `world-ch0.log` 15:27:47 has the
//! click: `<- 0x01FC, 11 byte body b8102c00 d6000000 00 00 00`, and nothing answered it.
//!
//! The reply was found by a chain that never guessed an opcode: the double-click handler
//! toggles a window singleton, the singleton's one constructor site is the "open window kind
//! 0x11" helper, and that helper is called from dispatcher case `0xA2` - which also clears the
//! `[world+0x2330]` request latch both `0x01FC` builders set. The opcode table's candidate names
//! for `0xA2` were wrong in both columns, and `0x00BE` (the first guess) is the NPC pool.
//! `research/character-info-2026-09-18.md` has every field with its reading address. **[L]**
//! throughout unless a doc comment below says otherwise.

use crate::packet::{PacketReader, PacketWriter};

/// The client asking about a character. Two builders, one body:
///
/// ```text
/// u32 tick
/// u32 characterId      0 when asking by name
/// str name             "" when asking by id
/// u8  petInfo          1 from the "Show Pet Info" entry, 0 from the double-click
/// ```
///
/// Both builders set the shared exclusive-request latch (`[world+0x2330] = 1`, the same one
/// `0x0107` sets), and only a handful of inbound packets clear it - `0x00A2` among them. An
/// unanswered request therefore freezes some 35 other request senders until the next field
/// entry. **Always answer**, with [`character_info_refused`] when nothing else fits.
pub const CLIENT_CHARACTER_INFO_REQUEST: u16 = 0x01FC;

/// The reply. `u32 result` first: `0` shows the window and the rest of the body follows;
/// anything else is read, the latch is cleared, and nothing is shown.
pub const CHARACTER_INFO: u16 = 0x00A2;

/// A parsed [`CLIENT_CHARACTER_INFO_REQUEST`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterInfoRequest {
    pub tick: u32,
    /// `0` means "by name".
    pub character_id: u32,
    pub name: String,
    /// Echoed into the reply's last byte: the pet panel opens with the window.
    pub pet_info: bool,
}

pub fn parse_character_info_request(body: &[u8]) -> Option<CharacterInfoRequest> {
    let mut r = PacketReader::new(body);
    let tick = r.u32().ok()?;
    let character_id = r.u32().ok()?;
    let name = r.str().ok()?;
    let pet_info = r.u8().ok()? != 0;
    Some(CharacterInfoRequest { tick, character_id, name, pet_info })
}

/// The summoned pet's panel: what the window prints beside TYPE / LEVEL / CLOSENESS /
/// FULLNESS, and the pet's worn equip - the hat - for the cell under the pet.
///
/// **The cell is the pet's EQUIP, not the pet.** The owner, 2026-09-18, with the window open on
/// themself: *"The pet window for the owner's pet Lucy should have a top hat showing, but in
/// Character Info that slot is blank"* - the first version sent the pet item itself behind
/// the flag, and the widget drew an empty cell with `ReqLv 0`. The research's row 14
/// (`dwPetWearItemID` in the reference) is the equip's id and row 15a the equip's whole item
/// slot; with no hat both are absent and the cell stays empty. The three numbers the panel
/// PRINTS are these fields, not anything in an item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetPanel {
    pub item_id: u32,
    /// Drawn verbatim beside TYPE.
    pub name: String,
    pub level: u32,
    pub closeness: u32,
    pub fullness: u32,
    /// The hat in the pet-equip slot: its id and its whole equip slot
    /// (`crate::opcode::equipped_item`, type byte included). `None` when the pet wears nothing.
    pub wear: Option<(u32, Vec<u8>)>,
}

/// One town's line of the **CITIZENSHIP tab** - row 17a's `{u32, u32, u32}`.
///
/// The owner, 2026-10-04, with a screenshot of Tester2 looking at a citizen of Henesys:
/// *"Citizenship data cannot be viewed by other players"* - the tab was greyed, because this
/// reply always sent no records.
///
/// **The three words are what the client builds for ITSELF.** Opened on the viewer's own
/// character, the window fills from local data (`FUN_141197a40`) and, for town 1 then town 2,
/// pushes `{FUN_1402c90f0(t), FUN_1402c9150(t), FUN_1402c92a0(t)}` into the same `w+0x3a0`
/// vector this reply fills (`1411988c0..141198960`) - and those three getters read quest
/// 510000's `st<t>`, `gr<t>` and `ct<t>` (`FUN_1402c8870` kinds 0, 1, 2). **[L]** Both paths
/// then apply the same rule to each record: `state == 1` -> `w+0x3c8 = grade` (the badge,
/// `FUN_1411a2da0`), `state != 0` -> `w+0x3c4 = 0`, which un-greys the CITIZENSHIP button
/// (`141195fc2`). So a record built from the same three keys draws what the citizen sees for
/// themself. How the panel lays the vector out was not walked; it is the same vector either
/// way, in the same town order, which is why [`CharacterInfo::towns`] is a fixed pair.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TownRecord {
    /// `st<town>`: 1 active; 0 never signed; anything else signed but not active.
    pub state: u32,
    /// `gr<town>`, 1..10.
    pub grade: u32,
    /// `ct<town>`, the running total.
    pub contribution: u32,
}

/// Row 17's count: the decoder gives up mid-body above 2, and the client's own fill always
/// pushes exactly 2 - town 1 (Henesys), then town 2 (Kerning City).
pub const CHARACTER_INFO_TOWNS: usize = 2;

/// What the window draws for a character who is not the viewer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharacterInfo {
    pub character_id: u32,
    pub name: String,
    pub level: u32,
    pub job: u32,
    pub fame: u32,
    /// `""` for none; drawn beside GUILD as `-` by the client.
    pub guild: String,
    /// `None`: the Show Pet button is greyed and the panel never opens.
    pub pet: Option<PetPanel>,
    /// The request's `petInfo`, echoed: the pet panel opens with the window.
    pub show_pet_panel: bool,
    /// **The ITEM tab**: whole `GW_ItemSlot`s (type byte included), at most
    /// [`CHARACTER_INFO_MAX_ITEMS`] - the decoder returns early past that and the fields
    /// after the list go unread. What the character is wearing: every worn equip, regular and
    /// cash (the owner, 2026-09-18: *"This item list should include the hair, face, equipment and
    /// cash shop cover items that the player is wearing."* - hair and face are not items in
    /// this client's data, so they have no slot to send; see `session/charinfo.rs`).
    pub items: Vec<Vec<u8>>,
    /// **The CITIZENSHIP tab**: town 1, then town 2, as [`TownRecord`]. All zero for a
    /// character who never signed - the button stays greyed, as it does on their own window.
    pub towns: [TownRecord; CHARACTER_INFO_TOWNS],
}

/// Row 16 of the reply: an item count above this makes the decoder give up mid-body.
pub const CHARACTER_INFO_MAX_ITEMS: usize = 32;

/// Length of a [`character_info`] body with no pet, empty guild and a name of `n` bytes:
/// the 60 of `research/character-info-2026-09-18.md` §3 plus the two 12-byte town records.
pub const CHARACTER_INFO_BASE_LEN: usize = 60 + CHARACTER_INFO_TOWNS * 12;

/// Build a [`CHARACTER_INFO`] body. Field order is the decoder's, read at the addresses in
/// `research/character-info-2026-09-18.md` §2, and every field is unconditional except the
/// pet item behind its flag and the two vectors behind their counts: the ITEM tab's worn
/// items and the CITIZENSHIP tab's two town records.
pub fn character_info(info: &CharacterInfo) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(0); // result: show it
    w.u32(info.character_id);
    w.str(&info.name);
    w.u32(info.level);
    w.u32(info.job);
    w.u32(info.fame);
    w.str(&info.guild);
    match &info.pet {
        Some(p) => {
            w.u32(p.item_id);
            w.str(&p.name);
            w.u32(p.level);
            w.u32(p.closeness);
            w.u32(p.fullness);
            w.u32(0); // the frame loader's 4th argument; 0 is what CPet itself passes
            match &p.wear {
                Some((hat_id, hat)) => {
                    w.u32(*hat_id); // the pet's wear item id
                    w.u8(1); // an equip slot follows: the hat, for the cell under the pet
                    w.bytes(hat);
                }
                None => {
                    w.u32(0); // nothing worn
                    w.u8(0);
                }
            }
        }
        None => {
            w.u32(0); // pet item id 0 = no pet
            w.str("");
            w.u32(0);
            w.u32(0);
            w.u32(0);
            w.u32(0);
            w.u32(0);
            w.u8(0); // hasPetItem
        }
    }
    let items = &info.items[..info.items.len().min(CHARACTER_INFO_MAX_ITEMS)];
    w.u32(items.len() as u32); // item count (ITEM tab), 0..32
    for item in items {
        w.bytes(item);
    }
    w.u32(CHARACTER_INFO_TOWNS as u32); // record count (CITIZENSHIP tab), 0..2
    for t in &info.towns {
        w.u32(t.state);
        w.u32(t.grade);
        w.u32(t.contribution);
    }
    w.u8(u8::from(info.show_pet_panel && info.pet.is_some()));
    w.into_vec()
}

/// A refusal: `u32 result != 0`, four bytes, nothing else. The client clears the latch and
/// shows nothing. For a character that does not exist or a name that does not resolve.
pub fn character_info_refused() -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(1);
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The observed request, byte for byte: `b8102c00 d6000000 0000 00` - tick, Tester2's
    /// id, an empty name, petInfo 0.
    #[test]
    fn the_double_click_on_tester2_parses_as_by_id_without_the_pet_panel() {
        let body = [0xb8, 0x10, 0x2c, 0x00, 0xd6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let r = parse_character_info_request(&body).unwrap();
        assert_eq!(r, CharacterInfoRequest { tick: 0x2c10b8, character_id: 214, name: String::new(), pet_info: false });
        assert!(parse_character_info_request(&body[..10]).is_none(), "the petInfo byte is read, not assumed");
    }

    /// The minimum body is 60 bytes plus the two strings, laid out exactly as §3 of the
    /// research writes it, and the refusal is one non-zero u32.
    #[test]
    fn the_no_pet_body_is_sixty_bytes_plus_the_strings_in_the_decoders_order() {
        let info = CharacterInfo {
            character_id: 214,
            name: "Tester2".into(),
            level: 8,
            job: 0,
            fame: 0,
            items: Vec::new(),
            guild: String::new(),
            pet: None,
            show_pet_panel: false,
            towns: Default::default(),
        };
        let b = character_info(&info);
        assert_eq!(b.len(), CHARACTER_INFO_BASE_LEN + "Tester2".len());
        assert_eq!(&b[..4], &[0, 0, 0, 0], "result 0");
        assert_eq!(u32::from_le_bytes(b[4..8].try_into().unwrap()), 214);
        assert_eq!(u16::from_le_bytes(b[8..10].try_into().unwrap()), 7, "name length");
        assert_eq!(&b[10..17], b"Tester2");
        assert_eq!(u32::from_le_bytes(b[17..21].try_into().unwrap()), 8, "level");
        assert_eq!(u32::from_le_bytes(b[21..25].try_into().unwrap()), 0, "job");
        assert_eq!(u32::from_le_bytes(b[25..29].try_into().unwrap()), 0, "fame");
        assert_eq!(&b[29..31], &[0, 0], "empty guild");
        assert_eq!(u32::from_le_bytes(b[31..35].try_into().unwrap()), 0, "no pet");
        assert_eq!(b[b.len() - 1], 0, "pet panel closed");
        let records = b.len() - 1 - 4 - 24;
        assert_eq!(&b[records..records + 4], &[2, 0, 0, 0], "two town records, as the client's own fill pushes");
        assert!(b[records + 4..b.len() - 1].iter().all(|&x| x == 0), "never signed: both all zero, the button stays greyed");
        assert_eq!(character_info_refused(), vec![1, 0, 0, 0]);
    }

    /// With a pet: the item id gates the button, the three numbers are what the panel
    /// prints, the hat's id and whole equip slot follow when the pet wears one (nothing and
    /// a clear flag when it does not), and the panel flag is echoed only when there is a pet.
    #[test]
    fn a_summoned_pet_rides_as_its_numbers_and_its_hat() {
        let hat = crate::opcode::equipped_item(1_802_006, &crate::opcode::EquipStats::default());
        let info = CharacterInfo {
            character_id: 215,
            name: "Wisp".into(),
            level: 12,
            job: 200,
            fame: 0,
            items: Vec::new(),
            guild: String::new(),
            pet: Some(PetPanel { item_id: 5_000_006, name: "Husky".into(), level: 3, closeness: 250, fullness: 90, wear: Some((1_802_006, hat.clone())) }),
            show_pet_panel: true,
            towns: Default::default(),
        };
        let b = character_info(&info);
        let at = 4 + 4 + 2 + 4 + 4 + 4 + 4 + 2; // through the empty guild
        assert_eq!(u32::from_le_bytes(b[at..at + 4].try_into().unwrap()), 5_000_006);
        let after_name = at + 4 + 2 + 5;
        let nums: Vec<u32> = (0..5).map(|i| u32::from_le_bytes(b[after_name + i * 4..after_name + i * 4 + 4].try_into().unwrap())).collect();
        assert_eq!(nums, vec![3, 250, 90, 0, 1_802_006], "level, closeness, fullness, loader arg, the hat's id");
        let flag = after_name + 20;
        assert_eq!(b[flag], 1, "an equip slot follows");
        assert_eq!(&b[flag + 1..flag + 1 + hat.len()], &hat[..], "the hat's whole equip slot, type byte first");
        let tail = &b[flag + 1 + hat.len()..];
        assert_eq!(&tail[..8], &[0, 0, 0, 0, 2, 0, 0, 0], "no items, two town records");
        assert_eq!(tail.len(), 8 + 24 + 1);
        assert_eq!(*tail.last().unwrap(), 1, "panel open");

        let bare = character_info(&CharacterInfo { pet: Some(PetPanel { wear: None, ..info.pet.clone().unwrap() }), ..info.clone() });
        assert_eq!(&bare[after_name + 16..after_name + 21], &[0, 0, 0, 0, 0], "no hat: id 0, no slot");

        let closed = character_info(&CharacterInfo { show_pet_panel: true, pet: None, ..info });
        assert_eq!(*closed.last().unwrap(), 0, "no pet: the panel flag is not echoed, the client would refuse it anyway");
    }

    /// A Henesys citizen of grade 5 with 4150 contribution who once signed in Kerning City
    /// (frozen, state 2): row 17 carries exactly `st1 gr1 ct1`, then `st2 gr2 ct2`, the words
    /// the client's own fill reads out of quest 510000 for town 1 then town 2.
    #[test]
    fn the_citizenship_tab_carries_both_towns_in_the_clients_own_order() {
        let info = CharacterInfo {
            character_id: 214,
            name: "Tester2".into(),
            level: 30,
            job: 200,
            fame: 1,
            items: Vec::new(),
            guild: String::new(),
            pet: None,
            show_pet_panel: false,
            towns: [
                TownRecord { state: 1, grade: 5, contribution: 4150 },
                TownRecord { state: 2, grade: 3, contribution: 900 },
            ],
        };
        let b = character_info(&info);
        assert_eq!(b.len(), CHARACTER_INFO_BASE_LEN + "Tester2".len());
        let at = b.len() - 1 - 24 - 4;
        let words: Vec<u32> = (0..7).map(|i| u32::from_le_bytes(b[at + i * 4..at + i * 4 + 4].try_into().unwrap())).collect();
        assert_eq!(words, vec![2, 1, 5, 4150, 2, 3, 900], "count, then town 1, then town 2");
        assert_eq!(&b[at - 4..at], &[0, 0, 0, 0], "the item count before it");
    }
}
