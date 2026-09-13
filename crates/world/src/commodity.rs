//! The cash shop's sale list, keyed by **SN** - the serial the shop trades in, not the item id.
//!
//! The owner, 2026-08-24: *"we need a way to add Leaf Points (NX) in our server so we can attempt to
//! make purchases in the Cash Shop so we can finish that entire transaction flow."*
//!
//! # The shop's key is the SN, and that is not a detail
//!
//! `Commodity.img` has 159 sale rows over **146 distinct item ids**: the same item is sold at
//! several counts, prices and durations, and each of those is its own SN. So a server that
//! looked a purchase up by item id could not tell "1 Megaphone for 100" from "11 Megaphones
//! for 1000" - they are `130200000` and `130200001`, item `5070000` both times. The client's
//! own help text calls the field *"CommoditySN corresponding to the cash item ID"*
//! (`0x143425161`). `research/cash-shop-items.md`.
//!
//! # This table is not sent to the client
//!
//! The client ships `Commodity.img` and reads it itself, which is why `SetCashShop`'s
//! modified-commodity count is zero (`net::cashshop::set_cash_shop`) - that list is a *delta*.
//! This copy exists so the **server** can price a purchase and name what was bought, and the
//! two are the same file, so they cannot disagree about what is for sale.
//!
//! # Generated data, gitignored, loaded at run time
//!
//! `gm-handbook/commodity.txt`, from `python tools/dump_commodity.py`. Same arrangement as
//! `gm-handbook/itemdata.txt` and the rest: the repo carries the code, the client carries the
//! content. A missing file is not fatal - the shop then refuses every purchase with a reason
//! rather than failing to start - and the banner says which of the two states we are in.

use std::collections::BTreeMap;
use std::path::Path;

/// One sale row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Commodity {
    /// The shop's key. `92000000..=160300005` in this build.
    pub sn: u32,
    /// What the buyer receives.
    pub item_id: u32,
    /// How many of it. `1` on 149 of the 159 rows; the others are 3, 5, 10 and 11.
    pub count: u16,
    /// **LEAF POINTS**, whatever the WZ property name says.
    ///
    /// `research/cash-shop-items.md` labels this column "NX" because that is what
    /// `Commodity.img` calls it. The screen disagrees: every price tag in the shop reads
    /// `LP`, and a client holding 10,000 NX and 0 LP refused a purchase itself and sent no
    /// `0x03E1` at all. `!lp` funds it; `!nx` fills a different, displayed, unspendable field.
    ///
    /// `0`, `100`, `700` or `1000` here - and the 21 rows priced `0` are **exactly** the 21
    /// that are switched off, so nothing buyable in this client is free.
    pub price: u32,
    /// **DAYS** the item lasts, not minutes and not hours. `0` means no expiry.
    ///
    /// The unit is worth stating out loud: `CLAUDE.md` records three separate bugs this month
    /// that were a correct number in the wrong unit, and this is a field a server would
    /// naturally assume was seconds.
    pub period_days: u16,
    /// `0` male, `1` female, `2` either. Only two rows in this build are not `2`, both
    /// swimsuits, which is what made the encoding readable at all.
    ///
    /// **Not enforced yet.** It is carried so that a refusal can be written when there is a
    /// character gender to compare it against, and named here rather than silently dropped.
    pub gender: u8,
    /// Switched on. 21 of the 159 rows are not.
    pub on_sale: bool,
    /// The item's name, for the log line and the GM acknowledgement.
    pub name: String,
}

/// What a walk over a payload found when it asked which `u32`s are real serials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SerialMatch {
    /// Exactly one offset held a known SN. This is the answer.
    One { offset: usize, sn: u32 },
    /// No offset held one - the payload is not a buy, or the table is empty.
    None,
    /// **Several offsets did.** Reported rather than resolved: picking the first would be a
    /// coin flip dressed up as a decode, and the run that produced it is the evidence that
    /// settles which offset is right.
    Several(Vec<(usize, u32)>),
}

/// `sn -> Commodity`.
#[derive(Debug, Clone, Default)]
pub struct CommodityTable {
    rows: BTreeMap<u32, Commodity>,
    source: String,
    problems: usize,
}

/// Columns in `gm-handbook/commodity.txt`, in order. The name is last and is allowed to
/// contain commas, so the split has to stop counting at this many fields.
const COLUMNS: usize = 24;

impl CommodityTable {
    /// Load `gm-handbook/commodity.txt`. A missing file gives an empty table, not an error.
    ///
    /// A row whose SN, item id, count or price will not parse is **dropped whole** rather than
    /// partly read, and counted. A half-read row here would put a wrong price on a purchase,
    /// and there is no second check behind it. Same rule as `world::shops::load_item_data`.
    pub fn load(path: &Path) -> Self {
        let mut out =
            CommodityTable { source: path.display().to_string(), ..CommodityTable::default() };
        let Ok(text) = std::fs::read_to_string(path) else { return out };
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let f: Vec<&str> = line.splitn(COLUMNS, ',').map(str::trim).collect();
            if f.len() != COLUMNS {
                out.problems += 1;
                continue;
            }
            // An EMPTY CELL means the property is absent from the WZ node, which is not the
            // same as zero - but for every field read here, absent and zero mean the same
            // thing to a purchase, so an empty cell parses as 0 rather than dropping the row.
            let num = |s: &str| -> Option<u32> {
                if s.is_empty() { Some(0) } else { s.parse::<u32>().ok() }
            };
            let (Some(sn), Some(item_id), Some(count), Some(price)) =
                (num(f[0]), num(f[1]), num(f[2]), num(f[3]))
            else {
                out.problems += 1;
                continue;
            };
            out.rows.insert(
                sn,
                Commodity {
                    sn,
                    item_id,
                    count: u16::try_from(count).unwrap_or(u16::MAX),
                    price,
                    period_days: num(f[6]).map_or(0, |d| u16::try_from(d).unwrap_or(u16::MAX)),
                    gender: num(f[7]).map_or(2, |g| u8::try_from(g).unwrap_or(2)),
                    on_sale: num(f[8]) != Some(0),
                    name: f[23].to_string(),
                },
            );
        }
        out
    }

    /// One sale row by its serial.
    /// The lowest on-sale serial that sells `item_id`, if any. For the locker listing: a
    /// stored row remembers its item and not the serial it was bought under, and the record
    /// the client reads has a commodity-serial field. `None` for an item nothing sells.
    pub fn serial_for_item(&self, item_id: u32) -> Option<u32> {
        self.rows.values().filter(|c| c.item_id == item_id && c.on_sale).map(|c| c.sn).min()
    }

    pub fn get(&self, sn: u32) -> Option<&Commodity> {
        self.rows.get(&sn)
    }

    /// How many rows are loaded.
    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// Is the table empty - i.e. did the file fail to load?
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// **Which offset in this payload holds a commodity SN.**
    ///
    /// **This settled the layout, and it is kept because it is still the fallback.**
    ///
    /// It was written when no real `0x03E1` had ever been captured: rather than pick one of
    /// the payload's `u32`s and be quietly wrong, it read a `u32` at every byte offset and
    /// asked the client's own sale list which were real serials. One run answered it - three
    /// clicks, three different items, all at offset 7, each resolving to the item the owner said
    /// they had clicked. `net::cashshop::BUY_SERIAL_OFFSET` now holds that.
    ///
    /// The walk stays for two reasons. The builder has a **short arm** that has never been
    /// seen and would put the serial elsewhere; and if the layout ever moves, this is what
    /// notices instead of decoding garbage. It cannot invent an answer - it only ever returns
    /// a serial that is really in `Commodity.img` - and two matches are reported rather than
    /// resolved.
    pub fn identify_serial(&self, rest: &[u8]) -> SerialMatch {
        // **The measured offset first.** Three captured buy requests put the serial at
        // `BUY_SERIAL_OFFSET` and each resolved to the item the owner said they had clicked, so the
        // walk below is no longer the primary decoder - it is the fallback for the short arm
        // of the builder's `cmov`, which has never been seen, and the cross-check that would
        // notice if the layout ever moved.
        if let Some(sn) = net::cashshop::parse_buy_serial(rest) {
            if self.rows.contains_key(&sn) {
                return SerialMatch::One { offset: net::cashshop::BUY_SERIAL_OFFSET, sn };
            }
        }
        let hits: Vec<(usize, u32)> = net::cashshop::u32_candidates(rest)
            .into_iter()
            .filter(|(_, v)| self.rows.contains_key(v))
            .collect();
        match hits.as_slice() {
            [] => SerialMatch::None,
            [(offset, sn)] => SerialMatch::One { offset: *offset, sn: *sn },
            _ => SerialMatch::Several(hits),
        }
    }

    /// Printed on stdout at start-up, in both directions. A banner that is silent when things
    /// are fine cannot be told apart from one that is not being printed.
    pub fn banner(&self) -> String {
        let from = if self.source.is_empty() { "(no file)" } else { self.source.as_str() };
        if self.rows.is_empty() {
            return format!(
                "maplecw-world: cash shop: NO SALE LIST from {from}. Every purchase will be \
                 refused with \"sold out\" - the client still draws its own catalogue, so the \
                 shop will look stocked and nothing will be buyable. Regenerate with: \
                 python tools/dump_commodity.py"
            );
        }
        let on_sale = self.rows.values().filter(|c| c.on_sale).count();
        let free = self.rows.values().filter(|c| c.price == 0).count();
        format!(
            "maplecw-world: cash shop: {} sale rows from {from}, {on_sale} on sale, {free} \
             priced at 0 NX{}",
            self.rows.len(),
            if self.problems > 0 {
                format!(" ({} unreadable line(s))", self.problems)
            } else {
                String::new()
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **`who` is not decoration.** The first version of this helper wrote every case to one
    /// path under the process id, and `cargo test` runs a crate's tests in parallel threads of
    /// **one** process - so two tests raced on the same file and one of them read the other's
    /// rows. It failed as "2 rows expected, 1 found", which reads exactly like a parser bug
    /// and is not one. Each case gets its own file.
    /// A scratch table on disk.
    ///
    /// **The name is not enough to make the path unique, and that made this suite flaky.**
    /// `two-rows` was used by two different tests; cargo runs them on parallel threads in one
    /// process, so both wrote the same file and whichever lost read it half-written - an empty
    /// table, surfacing as `SerialMatch::None` where a row was expected. It passed in isolation
    /// and failed about one full-suite run in ten, which is the worst way for a test to be
    /// wrong. The counter makes collision impossible rather than merely unlikely.
    fn table(who: &str, rows: &str) -> CommodityTable {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let unique = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("maplecw-commodity-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(format!("{who}-{unique}.txt"));
        std::fs::write(&path, rows).unwrap();
        CommodityTable::load(&path)
    }

    const HEADER: &str = "# sn, itemId, count, price, ...\n";
    // Two real rows, copied from gm-handbook/commodity.txt. Note they share an item id and
    // differ only by count and price - which is the whole reason the key is the SN.
    const ONE: &str = "130200000, 5070000, 1, 100, 100, , 0, 2, 1, , 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 302, Megaphone\n";
    const ELEVEN: &str = "130200001, 5070000, 11, 1000, 1100, 1, 0, 2, 1, , 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 302, Megaphone\n";

    /// **Two rows, one item id, and the SN is what tells them apart.**
    #[test]
    fn the_same_item_at_two_prices_is_two_rows() {
        let t = table("two-rows", &format!("{HEADER}{ONE}{ELEVEN}"));
        assert_eq!(t.len(), 2);

        let single = t.get(130200000).expect("the 100 NX row");
        assert_eq!((single.item_id, single.count, single.price), (5070000, 1, 100));
        let bundle = t.get(130200001).expect("the 1000 NX row");
        assert_eq!((bundle.item_id, bundle.count, bundle.price), (5070000, 11, 1000));
        assert_eq!(single.item_id, bundle.item_id, "the item id cannot tell these apart");
        assert_eq!(single.name, "Megaphone");
        assert!(t.get(999).is_none());
    }

    /// An absent cell is 0, and a row that will not parse is dropped whole and counted.
    #[test]
    fn a_broken_row_is_dropped_rather_than_half_read() {
        let t = table("broken", &format!("{HEADER}{ONE}not, a, row\n130200002, oops, 1, 100, , , , , , , , , , , , , , , , , , , , x\n"));
        assert_eq!(t.len(), 1, "only the good row survived");
        assert_eq!(t.get(130200000).unwrap().period_days, 0, "an empty period cell is 0 days");
        assert!(t.banner().contains("unreadable"), "and the banner says so: {}", t.banner());
    }

    /// **A real captured buy resolves at the measured offset**, and the fallback still works.
    #[test]
    fn a_real_buy_resolves_at_the_measured_offset() {
        let t = table("real-buy", &format!("{HEADER}{ONE}{ELEVEN}"));
        // Brown Puppy's shape, with a serial this fixture actually sells.
        let mut real = vec![0x01u8, 0x02, 0, 0, 0, 0, 0];
        real.extend_from_slice(&130200000u32.to_le_bytes());
        real.extend_from_slice(&[0u8; 4]);
        assert_eq!(real.len(), net::cashshop::BUY_PAYLOAD_LEN);
        assert_eq!(
            t.identify_serial(&real),
            SerialMatch::One { offset: net::cashshop::BUY_SERIAL_OFFSET, sn: 130200000 }
        );

        // A payload that does NOT carry a serial at the measured offset still gets walked -
        // that is the fallback for the short cmov arm, which has never been captured.
        let mut short = vec![0u8];
        short.extend_from_slice(&130200001u32.to_le_bytes());
        short.extend_from_slice(&[0u8; 4]);
        assert_eq!(t.identify_serial(&short), SerialMatch::One { offset: 1, sn: 130200001 });
    }

    /// **The serial walk finds the SN wherever it sits, and refuses to guess when two match.**
    #[test]
    fn identify_serial_answers_or_says_it_cannot() {
        let t = table("two-rows", &format!("{HEADER}{ONE}{ELEVEN}"));

        // The short buy form: u8, u32 sn, u32, u32.
        let mut payload = vec![0u8];
        payload.extend_from_slice(&130200000u32.to_le_bytes());
        payload.extend_from_slice(&[0u8; 8]);
        assert_eq!(t.identify_serial(&payload), SerialMatch::One { offset: 1, sn: 130200000 });

        // Nothing that looks like a serial.
        assert_eq!(t.identify_serial(&[0u8; 12]), SerialMatch::None);

        // Two real serials in one payload: reported, not resolved.
        let mut both = 130200000u32.to_le_bytes().to_vec();
        both.extend_from_slice(&130200001u32.to_le_bytes());
        match t.identify_serial(&both) {
            SerialMatch::Several(hits) => assert_eq!(hits.len(), 2, "{hits:?}"),
            other => panic!("two serials must not resolve to one: {other:?}"),
        }
    }

    /// A missing file is an empty table and a banner that says what that costs.
    #[test]
    fn a_missing_file_is_not_an_error() {
        let t = CommodityTable::load(Path::new("no/such/commodity.txt"));
        assert!(t.is_empty());
        assert!(t.banner().contains("NO SALE LIST"), "{}", t.banner());
    }

    /// **The drift check against the client's own data**, when it is present.
    ///
    /// `gm-handbook/` is generated and gitignored, so this is a no-op on a clean checkout -
    /// which is exactly the shape of check `CLAUDE.md` warns can be silently vacuous. It
    /// therefore asserts a positive control first: if the file is there at all, it must have
    /// the 159 rows `research/cash-shop-items.md` measured.
    #[test]
    fn the_real_table_loads_if_it_has_been_generated() {
        let path = Path::new("../../gm-handbook/commodity.txt");
        if !path.exists() {
            return;
        }
        let t = CommodityTable::load(path);
        // 159 is what research/cash-shop-items.md counted in the classic client alone. The
        // 9 more are the Signature Style Collection rows tools/backport_install.py writes
        // into the Special tab (2026-09-10): the box and eight set coupons, SN 120000000..8.
        // ...and 8 more are the pets the classic shop never listed (2026-09-13): SN 160000003..10.
        assert_eq!(t.len(), 176, "159 classic sale rows + 9 backported Special-tab rows + 8 pet rows");
        for (sn, pet) in [(160_000_003u32, 5_000_000u32), (160_000_010, 5_000_010)] {
            let row = t.get(sn).expect("a pet row the installer wrote");
            assert_eq!((row.item_id, row.price, row.period_days, row.on_sale), (pet, 100, 0, true), "permanent, 100 LP, on sale");
        }
        assert_eq!(t.problems, 0, "every row parses");
        let special = t.get(120_000_000).expect("the Signature Style Collection is on sale");
        // The box wears 5681599 in the classic client (family 568 opens on double-click;
        // 522 does not) - crate::signaturestyle::COLLECTION, and backport_install.py's BOX_ID.
        assert_eq!((special.item_id, special.price, special.on_sale), (crate::signaturestyle::COLLECTION, 8_000, true));

        // The price column is NX and the observed set is tiny. If a price ever lands outside
        // it, the column has moved and the whole table is decoding shifted.
        for c in t.rows.values() {
            // 2000 and 8000 are the two Special-tab prices the owner set on 2026-09-10; still a
            // closed set, so a shifted column fails here rather than reading as plausible.
            assert!(
                matches!(c.price, 0 | 100 | 700 | 1000 | 2000 | 8000),
                "{} ({}) priced {} - the price column has moved",
                c.sn,
                c.name,
                c.price
            );
            assert!(
                (92_000_000..=160_300_005).contains(&c.sn) || (120_000_000..=120_000_008).contains(&c.sn),
                "SN {} out of range",
                c.sn
            );
        }
    }
}
