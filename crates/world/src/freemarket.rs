//! **The Free Market's one-way door**, and remembering which town it was entered from.
//!
//! The owner, 2026-09-09: *"Wire the Free Market portals. The server should keep track of which
//! town the user entered from, and then when the user leaves the Free Market, it should return
//! them to the proper portal which they have entered from."*
//!
//! # Why this is not in `crate::scriptportals`
//!
//! That module resolves script portals whose destination is **derived from the map data** -
//! there is exactly one portal naming each of them as a target, so the answer is in the file.
//! These four are the ones it explicitly refused, and the reason it refused them is the reason
//! they need this module: their destination is **per-player state** and no amount of reading
//! `Map.wz` can produce it.
//!
//! ```text
//!   market01   10001040 Henesys Market   portal market00   -> the hall
//!   market02   10004000 Perion           portal market00   -> the hall
//!   market03   20001010 El Nath Market   portal market00   -> the hall
//!   market00   80002000 Free Market Ent. portal out00      -> wherever you came in from
//! ```
//!
//! Checked rather than assumed: **no row anywhere targets any of those four**, which is what
//! made them underivable and is also what makes the exit genuinely dangerous - a player in the
//! hall with nothing remembered has no way out that the map data can supply.
//!
//! # The client offers no fallback either
//!
//! `80002000`'s own `info/returnMap` is **80002000** - itself. So a character who somehow
//! reaches the hall with nothing remembered cannot be sent anywhere by the client's own data,
//! and the server has to choose. See [`FALLBACK`].

/// Every town-side entrance: `(map, portal on that map)`.
///
/// **A list of the three, not a range or a name match.** `market00` is a portal *name* used on
/// three different maps and also the *script* name on a fourth - matching on the name alone
/// would make the hall's own exit look like an entrance and loop the player back into it.
pub const ENTRANCES: [(u32, &str); 3] = [
    (10_001_040, "market00"), // Henesys Market
    (10_004_000, "market00"), // Perion
    (20_001_010, "market00"), // El Nath Market
];

/// The Free Market Entrance itself.
pub const HALL: u32 = 80_002_000;

/// The portal on [`HALL`] that leads back out.
pub const EXIT_PORTAL: &str = "out00";

/// Where an entering player arrives in the hall.
///
/// The exit portal, so they step out of the hall the same way they would step back in - which
/// is what every other portal pair in this game does, and what the two derived script portals
/// in `crate::scriptportals` do.
pub const HALL_ARRIVAL: &str = EXIT_PORTAL;

/// Where the exit sends someone the server has no memory for.
///
/// **A policy, and the client cannot supply one**: `80002000`'s `info/returnMap` is itself.
/// Henesys Market is chosen because it is one of the three real entrances - so the player
/// lands somewhere they could legitimately have come from - and because the hall's own
/// `info/mapMark` is `"Henesys"`, which is the closest thing to a preference the data has.
///
/// It should be unreachable in practice: the memory is written before the warp and persisted,
/// so only a character who was already standing in the hall when this shipped can hit it. It
/// exists because the alternative to a fallback is a player with no way out.
pub const FALLBACK: (u32, &str) = (10_001_040, "market00");

/// Is this portal a way **into** the Free Market?
pub fn is_entrance(map: u32, portal: &str) -> bool {
    ENTRANCES.iter().any(|&(m, p)| m == map && p == portal)
}

/// Is this portal the way **out** of it?
pub fn is_exit(map: u32, portal: &str) -> bool {
    map == HALL && portal == EXIT_PORTAL
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The hall's exit must not read as an entrance.** All four portals are called
    /// `market00` or sit on a map that has one, and a name-only match would send a player
    /// leaving the Free Market straight back into it - a loop with no way out.
    #[test]
    fn the_exit_is_not_mistaken_for_an_entrance() {
        for (map, portal) in ENTRANCES {
            assert!(is_entrance(map, portal), "{map}/{portal}");
            assert!(!is_exit(map, portal), "{map}/{portal} must not read as the exit");
        }
        assert!(is_exit(HALL, EXIT_PORTAL));
        assert!(!is_entrance(HALL, EXIT_PORTAL), "the hall's own out00 is the way OUT");
        // The hall also has a portal literally named market00 in some dumps; neither
        // predicate may fire on the wrong map.
        assert!(!is_entrance(HALL, "market00"));
        assert!(!is_exit(10_001_040, "out00"), "a town's out00 is not the hall's");
    }

    /// The fallback is a real entrance, so a player who lands on it is somewhere they could
    /// have come from rather than somewhere arbitrary.
    #[test]
    fn the_fallback_is_one_of_the_real_entrances() {
        assert!(is_entrance(FALLBACK.0, FALLBACK.1));
    }

    /// **Every constant here matches the real portal table**, so a re-dump that renames or
    /// moves one of these portals fails here rather than turning the door into a no-op.
    #[test]
    fn the_portals_all_exist_in_the_generated_table() {
        let path = std::path::Path::new("../../gm-handbook/portals.txt");
        let Ok(text) = std::fs::read_to_string(path) else { return };
        let rows: Vec<Vec<String>> = text
            .lines()
            .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
            .map(|l| l.split(',').map(|f| f.trim().to_string()).collect())
            .collect();
        let has = |map: u32, portal: &str| {
            rows.iter().any(|f| f[0] == map.to_string() && f[2] == portal)
        };
        for (map, portal) in ENTRANCES {
            assert!(has(map, portal), "{map}/{portal} is not in portals.txt");
        }
        assert!(has(HALL, EXIT_PORTAL), "the hall has no {EXIT_PORTAL}");
        assert!(has(FALLBACK.0, FALLBACK.1));

        // **And the reason this module exists**: not one of them has a static destination,
        // so nothing else could have resolved them. If a re-dump ever gives one a target,
        // this fires and the special case can be deleted.
        for (map, portal) in ENTRANCES.iter().copied().chain([(HALL, EXIT_PORTAL)]) {
            let row = rows
                .iter()
                .find(|f| f[0] == map.to_string() && f[2] == portal)
                .expect("checked above");
            assert_eq!(row[3], "0", "{map}/{portal} has a static target now: {row:?}");
        }
    }
}
