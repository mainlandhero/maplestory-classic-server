//! What each chair restores, from `gm-handbook/chairs.txt`.
//!
//! The owner, 2026-09-08: *"my character just sat in a chair, but the idle recovery did not adjust
//! to match the chair's recovery stats."* `research/mp-regen.md` §5.1 had recorded why:
//! *"This server has no chairs."* It has eleven now.
//!
//! # Absent is zero, and one chair proves it matters
//!
//! `tools/dump_chairs.py` writes a `0` wherever the client's WZ has no property at all, and
//! the **Blue Seal Cushion** (`3010008`) is why that is not a detail: it has no `recoveryHP`
//! node, only `recoveryMP`. A loader that defaulted a missing HP figure to the common value
//! of 30 would invent HP regeneration for a chair the client says gives none.
//!
//! ```text
//! 3010000..3010007   30 HP           3010008    0 HP, 10 MP
//! 3010009, 3010010   20 HP, 5 MP
//! ```
//!
//! # A missing file is not an error
//!
//! `gm-handbook/` is generated and gitignored, so a clean checkout has no chairs table. Same
//! rule as `world::shops::load_item_data`: an empty map, and the effect is that chairs restore
//! nothing extra - which is exactly the behaviour before this module existed.

use std::collections::HashMap;
use std::path::Path;

/// One chair's idle-recovery bonus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Chair {
    /// Extra HP per idle tick. The Red Chair's tooltip: *"Restores an additional 30 HP every
    /// 10 seconds"* - and `regen::REGEN_EVERY_MS` is 10 000, so this is per tick with no
    /// scaling. **The units agree by measurement, not by assumption.**
    pub recovery_hp: u32,
    /// Extra MP per idle tick.
    pub recovery_mp: u32,
    /// The level the client requires to use it. Recorded; the server does not enforce it,
    /// because the client already refuses and a second check would only disagree.
    pub req_level: u32,
}

impl Chair {
    /// A chair that restores nothing is still a chair. Used to keep "unknown id" and
    /// "restores nothing" distinguishable at the call site.
    pub fn is_inert(self) -> bool {
        self.recovery_hp == 0 && self.recovery_mp == 0
    }
}

/// Read `gm-handbook/chairs.txt`. A missing or unreadable file gives an empty table.
///
/// Rows are `itemId, recoveryHP, recoveryMP, reqLevel, name`. A row that does not parse is
/// skipped rather than defaulted, for the reason `shops::load_item_data` gives: a chair whose
/// numbers we could not read must behave like a chair we have never heard of, not like one
/// that restores zero.
pub fn load_chairs(path: &Path) -> HashMap<u32, Chair> {
    let mut out = HashMap::new();
    let Ok(text) = std::fs::read_to_string(path) else { return out };
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split(',').map(str::trim).collect();
        if f.len() < 4 {
            continue;
        }
        let n: Vec<Option<u32>> = f[..4].iter().map(|x| x.parse::<u32>().ok()).collect();
        if n.iter().any(Option::is_none) {
            continue;
        }
        let v: Vec<u32> = n.into_iter().map(Option::unwrap).collect();
        out.insert(
            v[0],
            Chair { recovery_hp: v[1], recovery_mp: v[2], req_level: v[3] },
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// Each call gets its own file. The first version of this helper reused one name per
    /// process, and `cargo test` runs these in parallel threads - two tests clobbered each
    /// other's table and both failed for a reason that had nothing to do with the loader.
    fn table(body: &str) -> HashMap<u32, Chair> {
        use std::sync::atomic::{AtomicU32, Ordering};
        static N: AtomicU32 = AtomicU32::new(0);
        let mut p = std::env::temp_dir();
        p.push(format!(
            "maplecw-chairs-{}-{}.txt",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        let mut f = std::fs::File::create(&p).unwrap();
        f.write_all(body.as_bytes()).unwrap();
        drop(f);
        let out = load_chairs(&p);
        let _ = std::fs::remove_file(&p);
        out
    }

    #[test]
    fn a_missing_file_is_an_empty_table_not_a_panic() {
        assert!(load_chairs(Path::new("no/such/chairs.txt")).is_empty());
    }

    #[test]
    fn the_real_rows_load() {
        let t = table(
            "# itemId, recoveryHP, recoveryMP, reqLevel, name\n\
             3010005, 30, 0, 5, Red Chair\n\
             3010009, 20, 5, 10, Henesys Resident's Chair\n",
        );
        assert_eq!(t[&3_010_005], Chair { recovery_hp: 30, recovery_mp: 0, req_level: 5 });
        assert_eq!(t[&3_010_009], Chair { recovery_hp: 20, recovery_mp: 5, req_level: 10 });
    }

    /// The Blue Seal Cushion: MP only, and its zero HP must survive the round trip rather than
    /// being defaulted to the value every other chair has.
    #[test]
    fn a_chair_with_no_hp_recovery_stays_at_zero() {
        let t = table("3010008, 0, 10, 0, Blue Seal Cushion\n");
        let c = t[&3_010_008];
        assert_eq!(c.recovery_hp, 0);
        assert_eq!(c.recovery_mp, 10);
        assert!(!c.is_inert());
    }

    #[test]
    fn a_malformed_row_is_skipped_not_defaulted() {
        let t = table("3010005, thirty, 0, 5, Red Chair\n3010004, 30, 0, 5, Green Chair\n");
        assert!(!t.contains_key(&3_010_005), "an unreadable row must not become a zero chair");
        assert!(t.contains_key(&3_010_004));
    }
}
