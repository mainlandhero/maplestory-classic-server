//! Which pets each pet equip fits - `gm-handbook/petequips.txt`, from `tools/dump_petequips.py`.
//!
//! The owner, 2026-10-04, with Tester2's Character Info open on a Lil Frieren wearing a Blue Top
//! Hat: *"If a pet equipment is invalid for the current pet, it should not show in pet info"*.
//! Their own Deco window draws that hat's cell red; the client knows it does not fit.
//!
//! A pet equip's image `Character/PetEquip/<id>.img` has one child per pet it has art for,
//! named by the pet's item id. The Blue Top Hat (1802006) has 5000000..5000010 - the classic
//! pets - and Lil Frieren (5002828) is not among them. **[L]** for the data; that the client's
//! red cell tests the same child is **[I]** (the predicate was not walked).

use std::collections::HashMap;
use std::path::Path;

/// Equip item id -> the pet item ids it fits.
pub type PetEquips = HashMap<u32, Vec<u32>>;

/// Read `gm-handbook/petequips.txt`. A missing or unreadable file gives an empty table.
///
/// Rows are `equipId, pet pet pet ..., name`. A row whose numbers do not all parse is skipped:
/// an equip we could not read must behave like one we have never heard of ([`fits`] says
/// yes), not like one that fits no pet.
pub fn load_pet_equips(path: &Path) -> PetEquips {
    let mut out = HashMap::new();
    let Ok(text) = std::fs::read_to_string(path) else { return out };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split(',').map(str::trim).collect();
        if f.len() < 2 {
            continue;
        }
        let Ok(equip) = f[0].parse::<u32>() else { continue };
        let pets: Option<Vec<u32>> = f[1].split_whitespace().map(|p| p.parse::<u32>().ok()).collect();
        let Some(pets) = pets else { continue };
        out.insert(equip, pets);
    }
    out
}

/// Whether `equip` fits the pet `pet_item_id`. **An equip with no row fits** - an ungenerated
/// table or an equip this client does not list must not hide anything that used to show.
pub fn fits(table: &PetEquips, equip: u32, pet_item_id: u32) -> bool {
    table.get(&equip).map_or(true, |pets| pets.contains(&pet_item_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_classic_hat_fits_a_classic_pet_and_not_lil_frieren() {
        let dir = std::env::temp_dir().join(format!("maplecw-petequips-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join("petequips.txt");
        std::fs::write(
            &p,
            "# generated\n\
             1802006, 5000000 5000001 5000002 5000003 5000004 5000005 5000006 5000007 5000008 5000009 5000010, Blue Top Hat\n\
             1803148, 5002828, Lil Frieren's Staff\n\
             1802099, 5000000 x, a row that does not parse\n",
        )
        .unwrap();
        let t = load_pet_equips(&p);
        assert_eq!(t.len(), 2, "the unparsable row is skipped, not defaulted");
        assert!(fits(&t, 1_802_006, 5_000_006), "the Blue Top Hat on a Husky");
        assert!(!fits(&t, 1_802_006, 5_002_828), "the Blue Top Hat on Lil Frieren: no art, does not fit");
        assert!(fits(&t, 1_803_148, 5_002_828), "Lil Frieren's own staff");
        assert!(!fits(&t, 1_803_148, 5_000_006));
        assert!(fits(&t, 1_802_099, 5_002_828), "unknown equip: shown, as before the table");
        assert!(load_pet_equips(Path::new("no/such/petequips.txt")).is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
