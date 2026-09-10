//! **Portals the client resolves by running a script**, not by following a target map.
//!
//! The owner, 2026-09-09, standing on the spot in Ellinia: *"Ellinia should also have a portal to
//! go to Ellinia Station right here, but this portal does not exist where I expect it."*
//!
//! It exists. `Map.wz` gives Ellinia's portal 38 as
//!
//! ```text
//! {"pn": "in03", "pt": 8, "x": 819, "y": -3072, "tm": 999999999, "tn": "",
//!  "script": "pt_10002000_in03", ...}
//! ```
//!
//! `tm` is `999999999`, the "no target" value - the destination lives in **`script`**, and
//! `tools/dump_portals.py` read only `pn` and `tm`. So the row reached the server looking
//! exactly like a spawn point, and standing on it did nothing.
//!
//! # The size of the problem, enumerated rather than sampled
//!
//! **41 portals of 3679 carry a script, across 9 distinct names.** Every one of them was
//! silently flattened to a spawn point before the `script` column existed:
//!
//! ```text
//!   pt_10002000_in03          1   Ellinia -> Ellinia Station          IMPLEMENTED
//!   pt_10002071_down          1   Tree Tunnel -> Sleepywood           IMPLEMENTED
//!   market01 / 02 / 03        3   Henesys / Perion / El Nath -> Free Market
//!   market00                  1   Free Market -> the town you came from
//!   rand_ola                 29   Ola Ola's random exits
//!   PQ_01_nextstage_portal    5   Kerning PQ stage advance
//!   Zakum05                   1   the Zakum door
//! ```
//!
//! # Only the two that are DERIVED are implemented
//!
//! A script portal's destination is not in its own row, so it has to come from somewhere. For
//! these two it comes from the map data itself: **exactly one portal in the entire game names
//! each of them as its target**, and a portal pair is symmetric.
//!
//! ```text
//!   10002090, 2, out00, 10002000, in03      the only row naming 10002000/in03
//!   10006060, 9, east00, 10002071, down00   the only row naming 10002071/down00
//! ```
//!
//! So `in03` goes to `10002090/out00` and `down00` to `10006060/east00`, and both are read
//! off the client's own data rather than remembered from playing the game.
//!
//! **The other seven have no such reverse link** - checked, all four market targets come back
//! empty - because their destination genuinely depends on state the row cannot hold:
//! `market00` returns you to whichever town you entered from, `rand_ola` picks at random,
//! and the PQ and Zakum portals are gated on party and boss state.
//!
//! # The four `market*` scripts are now handled, and NOT by adding them here
//!
//! The owner asked for the Free Market next, with the state that makes it work: *"The server should
//! keep track of which town the user entered from, and then when the user leaves the Free
//! Market, it should return them to the proper portal which they have entered from."*
//!
//! That is a destination which depends on the player, so it cannot be a row in
//! [`DESTINATIONS`] no matter how the table is shaped - the key is a script name and the
//! answer is per-character. `crate::freemarket` holds the rule, `store::fieldreturn` holds the
//! memory, and `session::field` resolves them before this table is consulted. The remaining
//! three - `rand_ola`, `PQ_01_nextstage_portal` and `Zakum05` - are still unresolved.

/// Where one script portal leads: `(map, arrival portal name)`.
///
/// **Derived from the unique reverse link**, not from memory of the game - see the module
/// docs. A test re-derives both from `gm-handbook/portals.txt` so a re-dump that moved either
/// destination fails here rather than in a player's client.
pub const DESTINATIONS: [(&str, u32, &str); 2] = [
    ("pt_10002000_in03", 10_002_090, "out00"),
    ("pt_10002071_down", 10_006_060, "east00"),
];

/// The scripts this server knows about and deliberately does **not** resolve **here**, with
/// why.
///
/// Listed so that "nothing happened" on one of them is a known gap rather than a mystery, and
/// so the count in the module docs can be checked against the data.
///
/// **The four `market*` scripts are handled, just not by this module.** Their destination is
/// per-player state, which is precisely what a table keyed on a script name cannot hold -
/// `crate::freemarket` and `store::fieldreturn` own them, and `session::field` resolves them
/// before the static lookup. They stay listed here because a reader asking "why is
/// `market01` not in `DESTINATIONS`" deserves the answer at the place they are looking.
pub const UNIMPLEMENTED: [(&str, &str); 5] = [
    ("market00", "the Free Market's exit - per-player state, handled by crate::freemarket"),
    ("market01", "town -> Free Market - handled by crate::freemarket, which also remembers the way back"),
    ("market02", "town -> Free Market; see market01"),
    ("market03", "town -> Free Market; see market01"),
    ("rand_ola", "Ola Ola picks an exit at random"),
];

/// Where `script` leads, if this server resolves it.
pub fn destination(script: &str) -> Option<(u32, &'static str)> {
    DESTINATIONS.iter().find(|(s, _, _)| *s == script).map(|&(_, m, p)| (m, p))
}

/// Is this a script this server knows about at all - resolved or not?
pub fn is_known(script: &str) -> bool {
    destination(script).is_some() || UNIMPLEMENTED.iter().any(|(s, _)| *s == script)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn portal_rows() -> Vec<(u32, u32, String, u32, String, String)> {
        let path = std::path::Path::new("../../gm-handbook/portals.txt");
        let Ok(text) = std::fs::read_to_string(path) else { return Vec::new() };
        let mut out = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.split(',').map(str::trim).collect();
            if f.len() < 6 {
                continue;
            }
            let (Ok(map), Ok(idx), Ok(target)) =
                (f[0].parse::<u32>(), f[1].parse::<u32>(), f[3].parse::<u32>())
            else {
                continue;
            };
            out.push((map, idx, f[2].to_string(), target, f[4].to_string(), f[5].to_string()));
        }
        out
    }

    /// **Both destinations are re-derived from the data, not trusted from the constant.**
    ///
    /// The rule is: a script portal's destination is the map and portal of the *unique* row
    /// that names it as a target. This re-runs that derivation over the real file and requires
    /// it to produce exactly what [`DESTINATIONS`] says - so a re-dump that moves either
    /// endpoint fails here rather than in a player's client.
    #[test]
    fn each_destination_is_the_unique_reverse_link_in_the_real_data() {
        let rows = portal_rows();
        if rows.is_empty() {
            return; // generated, gitignored
        }
        for &(script, want_map, want_portal) in &DESTINATIONS {
            // Find the portal that carries this script.
            let source: Vec<_> =
                rows.iter().filter(|r| r.5 == script).collect();
            assert_eq!(source.len(), 1, "{script} is on {} portals", source.len());
            let (src_map, _, src_name, ..) = source[0];

            // Every row naming it as a target. There must be exactly one, or the derivation
            // this whole module rests on does not hold.
            let reverse: Vec<_> =
                rows.iter().filter(|r| r.3 == *src_map && r.4 == *src_name).collect();
            assert_eq!(
                reverse.len(),
                1,
                "{script}: {} rows name {src_map}/{src_name}, so the destination is not \
                 derivable this way",
                reverse.len()
            );
            assert_eq!(
                (reverse[0].0, reverse[0].2.as_str()),
                (want_map, want_portal),
                "{script}'s reverse link moved"
            );
            // And the arrival portal must exist on the destination map, or the client is sent
            // to a portal name nothing resolves.
            assert!(
                rows.iter().any(|r| r.0 == want_map && r.2 == want_portal),
                "{want_map} has no portal named {want_portal}"
            );
        }
    }

    /// **The two really reach `Config::portals`**, which is the half the derivation test
    /// cannot see: a correct table that nothing folds into the link map leaves the portal just
    /// as dead as before. This is the "built is not wired" check.
    #[test]
    fn the_resolved_portals_are_in_the_loaded_link_table() {
        let path = std::path::Path::new("../../gm-handbook/portals.txt");
        if !path.exists() {
            return;
        }
        let (links, index) = crate::config::Config::load_portals(path);
        assert!(links.len() > 1000, "the file loaded: {}", links.len());

        // The owner's portal.
        assert_eq!(
            links.get(&(10_002_000, "in03".to_string())),
            Some(&(10_002_090, "out00".to_string())),
            "standing on Ellinia's in03 must lead to Ellinia Station"
        );
        assert_eq!(
            links.get(&(10_002_071, "down00".to_string())),
            Some(&(10_006_060, "east00".to_string()))
        );
        // The arrival portal resolves to a real index, or the character is placed nowhere.
        assert!(index.contains_key(&(10_002_090, "out00".to_string())));
        assert!(index.contains_key(&(10_006_060, "east00".to_string())));

        // **The control: an unresolved script portal is still dead.** If this started
        // resolving, something is inventing destinations.
        assert_eq!(links.get(&(10_001_040, "market00".to_string())), None);
        // And an ordinary portal is untouched by all of this.
        assert_eq!(
            links.get(&(10_002_090, "out00".to_string())),
            Some(&(10_002_000, "in03".to_string()))
        );
    }

    /// **Every script in the data is one this module has an opinion about.**
    ///
    /// The point is not the 41; it is that a re-dump which introduces a *tenth* script name
    /// fails here instead of silently becoming another portal that does nothing. That is the
    /// failure this module exists because of.
    #[test]
    fn every_script_in_the_real_data_is_accounted_for() {
        let rows = portal_rows();
        if rows.is_empty() {
            return;
        }
        let mut counts: HashMap<&str, usize> = HashMap::new();
        for r in rows.iter().filter(|r| !r.5.is_empty()) {
            *counts.entry(r.5.as_str()).or_default() += 1;
        }
        assert_eq!(counts.values().sum::<usize>(), 41, "the script-portal count moved: {counts:?}");
        assert_eq!(counts.len(), 9, "distinct script names: {counts:?}");
        for name in counts.keys() {
            assert!(
                is_known(name) || matches!(*name, "PQ_01_nextstage_portal" | "Zakum05"),
                "{name} is a script portal nothing here has an opinion about"
            );
        }
        // The two that are implemented really are the ones the docs claim.
        assert!(destination("pt_10002000_in03").is_some());
        assert!(destination("pt_10002071_down").is_some());
        // And the control: an unimplemented one resolves to nothing rather than to a default.
        assert_eq!(destination("market00"), None);
        assert_eq!(destination("rand_ola"), None);
        assert_eq!(destination("nonsense"), None);
    }
}
