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
/// FULLNESS, and the whole pet item so the panel can build its item widget. The three
/// numbers the panel PRINTS are these fields, not the item's - the client never reads them
/// out of the item - so a caller keeps them equal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetPanel {
    pub item_id: u32,
    /// Drawn verbatim beside TYPE.
    pub name: String,
    pub level: u32,
    pub closeness: u32,
    pub fullness: u32,
    /// `crate::bag::pet_item_with_state(..)`, type byte included.
    pub item: Vec<u8>,
}

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
}

/// Length of a [`character_info`] body with no pet, empty guild and a name of `n` bytes.
pub const CHARACTER_INFO_BASE_LEN: usize = 60;

/// Build a [`CHARACTER_INFO`] body. Field order is the decoder's, read at the addresses in
/// `research/character-info-2026-09-18.md` §2, and every field is unconditional except the
/// pet item behind its flag and the two vectors behind their counts (both sent empty: the
/// ITEM and CITIZENSHIP tabs' data, whose contents are not established).
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
            w.u32(0); // look override: draw the pet the item id names
            w.u8(1); // hasPetItem
            w.bytes(&p.item);
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
    w.u32(0); // item count (ITEM tab), 0..32
    w.u32(0); // record count (CITIZENSHIP tab), 0..2
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
            guild: String::new(),
            pet: None,
            show_pet_panel: false,
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
        assert_eq!(character_info_refused(), vec![1, 0, 0, 0]);
    }

    /// With a pet: the item id gates the button, the three numbers are what the panel
    /// prints, the whole pet item follows its flag byte, and the panel flag is echoed only
    /// when there is a pet to show.
    #[test]
    fn a_summoned_pet_rides_as_its_numbers_and_its_whole_item() {
        let vitals = crate::bag::PetVitals { level: 3, closeness: 250, fullness: 90, skills: 1 };
        let item = crate::bag::pet_item_with_state(5_000_006, "Husky", None, 1, &vitals);
        let info = CharacterInfo {
            character_id: 215,
            name: "Wisp".into(),
            level: 12,
            job: 200,
            fame: 0,
            guild: String::new(),
            pet: Some(PetPanel { item_id: 5_000_006, name: "Husky".into(), level: 3, closeness: 250, fullness: 90, item: item.clone() }),
            show_pet_panel: true,
        };
        let b = character_info(&info);
        let at = 4 + 4 + 2 + 4 + 4 + 4 + 4 + 2; // through the empty guild
        assert_eq!(u32::from_le_bytes(b[at..at + 4].try_into().unwrap()), 5_000_006);
        let after_name = at + 4 + 2 + 5;
        let nums: Vec<u32> = (0..5).map(|i| u32::from_le_bytes(b[after_name + i * 4..after_name + i * 4 + 4].try_into().unwrap())).collect();
        assert_eq!(nums, vec![3, 250, 90, 0, 0], "level, closeness, fullness, loader arg, look override");
        let flag = after_name + 20;
        assert_eq!(b[flag], 1, "hasPetItem");
        assert_eq!(&b[flag + 1..flag + 1 + item.len()], &item[..], "the whole pet item, type byte first");
        assert_eq!(b[flag + 1 + item.len()..], [0, 0, 0, 0, 0, 0, 0, 0, 1], "no items, no records, panel open");

        let closed = character_info(&CharacterInfo { show_pet_panel: true, pet: None, ..info });
        assert_eq!(*closed.last().unwrap(), 0, "no pet: the panel flag is not echoed, the client would refuse it anyway");
    }
}
