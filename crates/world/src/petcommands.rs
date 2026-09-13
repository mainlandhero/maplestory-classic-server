//! What a pet does when you type at it.
//!
//! The owner, 2026-09-13: *"The pet commands are actually regular chat messages. They will still
//! happen as regular chat messages in the game, but if those messages match as one of the pet
//! commands, then the pet should respond accordingly."*
//!
//! Measured, not assumed: typing `bad` produced an ordinary `0x00E7 CLIENT_CHAT` and no pet
//! packet at all, so the chat line stays exactly as it is and the pet's answer is a **second**
//! thing the server sends beside it.
//!
//! # The rule is the client's own data
//!
//! `tools/dump_pets.py` joins the three places the client keeps this and writes
//! `gm-handbook/petcommands.txt`; this loads it. Per pet and per `interact` entry:
//!
//! * **`words`** - the chat text that triggers it, pipe-separated (`bad|no|badgirl|badboy`).
//! * **`l0`..`l1`** - the pet level band this entry serves. One word has an entry per band, so
//!   the same `sit` gets a different trick and better odds as the pet grows.
//! * **`prob`** - the percent chance of success.
//! * **`inc`** - the closeness the pet gains. Recorded here, **not yet applied**: this server
//!   keeps no per-pet closeness, so a pet never levels and always uses the level-1 band.
//! * the success and fail **lines**, one of which is picked at random, and the `interact`
//!   **index**, which is what `net::pet::pet_action` sends - the client owns the animation and
//!   looks it up in the pet's own image.
//!
//! # Matching
//!
//! The whole message, trimmed, compared case-insensitively against each word. Not a prefix and
//! not a substring: `sit` is a command and `sit down over there` is a sentence, and a player
//! chatting normally must not set the pet off. A message that matches nothing is just chat.

use std::collections::HashMap;
use std::path::Path;

/// One outcome line of one `interact` entry.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    success: bool,
    text: String,
}

/// One `interact` entry: a command, the level band it serves, and its two outcomes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Entry {
    index: u8,
    words: Vec<String>,
    prob: u32,
    l0: u32,
    l1: u32,
    /// The closeness this would add. Unused until pets have closeness; see the module docs.
    inc: u32,
    lines: Vec<Line>,
}

/// What the pet should do about a chat message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetResponse {
    /// The `interact` entry, which is the byte the action packet carries.
    pub index: u8,
    pub success: bool,
    /// The line the pet says.
    pub text: String,
    /// The animation the client will play. Carried for the log only - the client picks it up
    /// from its own WZ using `index` and `success`.
    pub act: String,
    /// The closeness this earns. Not applied yet.
    pub inc: u32,
}

/// Every pet's command table, from `gm-handbook/petcommands.txt`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PetCommands {
    by_pet: HashMap<u32, Vec<Entry>>,
    /// The `act` of each `(pet, index, success)`, for the log line.
    acts: HashMap<(u32, u8, bool), String>,
}

impl PetCommands {
    /// Load the table. A missing file is an empty table and every message stays plain chat -
    /// the same degradation every other generated table in this crate has.
    pub fn load(path: &Path) -> Self {
        let Ok(text) = std::fs::read_to_string(path) else { return Self::default() };
        Self::parse(&text)
    }

    pub fn parse(text: &str) -> Self {
        let mut out = Self::default();
        // (pet, index) -> where it sits in by_pet, so the rows of one entry gather.
        let mut at: HashMap<(u32, u8), usize> = HashMap::new();
        for line in text.lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 10 {
                continue;
            }
            let num = |i: usize| f[i].trim().parse::<u32>().ok();
            let (Some(pet), Some(index), Some(prob), Some(l0), Some(l1), Some(inc)) =
                (num(0), num(1), num(3), num(4), num(5), num(6))
            else {
                continue;
            };
            let Ok(index) = u8::try_from(index) else { continue };
            let success = match f[7].trim() {
                "s" => true,
                "f" => false,
                _ => continue,
            };
            let text = f[9].to_string();
            if text.is_empty() {
                continue;
            }
            let list = out.by_pet.entry(pet).or_default();
            let slot = *at.entry((pet, index)).or_insert_with(|| {
                list.push(Entry {
                    index,
                    words: f[2].split('|').map(|w| w.trim().to_lowercase()).filter(|w| !w.is_empty()).collect(),
                    prob,
                    l0,
                    l1,
                    inc,
                    lines: Vec::new(),
                });
                list.len() - 1
            });
            list[slot].lines.push(Line { success, text });
            out.acts.entry((pet, index, success)).or_insert_with(|| f[8].to_string());
        }
        out
    }

    /// How many pets have a table. For the banner.
    pub fn pets(&self) -> usize {
        self.by_pet.len()
    }

    /// How many `interact` entries in total.
    pub fn entries(&self) -> usize {
        self.by_pet.values().map(|v| v.len()).sum()
    }

    /// **What `pet_id` at `level` does about `message`.** `None` when the message is not one of
    /// this pet's commands, which is the ordinary case and means "this was only chat".
    ///
    /// `roll` is any value; the caller's RNG. Its low bits pick success against `prob` and then
    /// which line of that outcome is spoken, so one draw decides both.
    pub fn respond(&self, pet_id: u32, level: u32, message: &str, roll: u64) -> Option<PetResponse> {
        let want = message.trim().to_lowercase();
        if want.is_empty() {
            return None;
        }
        let list = self.by_pet.get(&pet_id)?;
        // The entry for this word whose band holds the level; if the data has no band for it
        // (a level past every `l1`), the highest band is used rather than nothing.
        let matching = || list.iter().filter(|e| e.words.iter().any(|w| *w == want));
        let entry = matching()
            .find(|e| level >= e.l0 && level <= e.l1)
            .or_else(|| matching().max_by_key(|e| e.l1))?;

        let success = (roll % 100) < u64::from(entry.prob);
        let pool: Vec<&Line> = entry.lines.iter().filter(|l| l.success == success).collect();
        // An outcome with no lines in the data still acts - silence beats saying nothing at all,
        // because the animation is the visible half.
        let text = if pool.is_empty() {
            String::new()
        } else {
            pool[((roll / 100) as usize) % pool.len()].text.clone()
        };
        Some(PetResponse {
            index: entry.index,
            success,
            text,
            act: self.acts.get(&(pet_id, entry.index, success)).cloned().unwrap_or_default(),
            inc: entry.inc,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
# petId\tindex\twords\tprob\tl0\tl1\tinc\tkind\tact\ttext
5000006\t0\tsit\t40\t1\t9\t1\ts\trest0\tBark bark!
5000006\t0\tsit\t40\t1\t9\t1\ts\trest0\tBark! BARK BARK!
5000006\t0\tsit\t40\t1\t9\t1\tf\tstand1\tHeh... heh...
5000006\t1\tsit\t60\t10\t19\t1\ts\trest0\tYou want me to sit, like this? Bark?
5000006\t4\tbad|no|badgirl|badboy\t40\t1\t9\t1\ts\tnap\tI am sorry, bark...
";

    fn table() -> PetCommands {
        PetCommands::parse(SAMPLE)
    }

    /// The word picks the entry, the pet's LEVEL picks which band of it, and the roll decides
    /// success against that band's own probability.
    #[test]
    fn a_command_picks_the_entry_for_the_pets_level_band() {
        let t = table();
        assert_eq!(t.pets(), 1);
        assert_eq!(t.entries(), 3);

        // Level 1 is the first band: index 0, 40%.
        let r = t.respond(5_000_006, 1, "sit", 0).unwrap();
        assert_eq!((r.index, r.success), (0, true), "roll 0 < 40");
        assert_eq!(r.act, "rest0");
        assert_eq!(r.text, "Bark bark!");
        // Level 12 is the second band: a different entry and better odds.
        let r = t.respond(5_000_006, 12, "sit", 50).unwrap();
        assert_eq!((r.index, r.success), (1, true), "roll 50 < 60 on the level-10 band");
        // Past every band: the highest is used rather than nothing.
        assert_eq!(t.respond(5_000_006, 99, "sit", 0).unwrap().index, 1);
    }

    #[test]
    fn a_failed_roll_speaks_the_fail_line_and_a_second_line_is_reachable() {
        let t = table();
        let r = t.respond(5_000_006, 1, "sit", 40).unwrap();
        assert_eq!((r.success, r.act.as_str()), (false, "stand1"), "roll 40 is not < 40");
        assert_eq!(r.text, "Heh... heh...");
        // The draw's high half chooses among the outcome's lines.
        assert_eq!(t.respond(5_000_006, 1, "sit", 100).unwrap().text, "Bark! BARK BARK!");
        assert_eq!(t.respond(5_000_006, 1, "sit", 0).unwrap().text, "Bark bark!");
    }

    /// Any of the pipe-separated words, case-insensitively, and **only** the whole message.
    #[test]
    fn every_synonym_matches_and_a_sentence_containing_one_does_not() {
        let t = table();
        for w in ["bad", "no", "badgirl", "BadBoy", "  bad  "] {
            assert_eq!(t.respond(5_000_006, 1, w, 0).unwrap().index, 4, "{w}");
        }
        for not in ["sit down over there", "badly", "", "hello", "nobody"] {
            assert!(t.respond(5_000_006, 1, not, 0).is_none(), "{not:?} is chat, not a command");
        }
        assert!(t.respond(5_000_001, 1, "sit", 0).is_none(), "another pet's table is not this one's");
    }

    /// A missing file is an empty table and every message stays plain chat.
    #[test]
    fn no_table_means_every_message_is_only_chat() {
        let t = PetCommands::load(Path::new("no/such/petcommands.txt"));
        assert_eq!(t.pets(), 0);
        assert!(t.respond(5_000_006, 1, "sit", 0).is_none());
    }

    /// The generated table, when it is there: the Husky knows `sit`, and every row's index fits
    /// the byte the action packet carries.
    #[test]
    fn the_generated_table_answers_the_huskys_own_commands() {
        let path = Path::new("../../gm-handbook/petcommands.txt");
        if !path.exists() {
            return; // generated, gitignored - python tools/dump_pets.py
        }
        let t = PetCommands::load(path);
        assert!(t.pets() >= 11, "the eleven classic pets: {}", t.pets());
        let r = t.respond(5_000_006, 1, "sit", 0).expect("the Husky knows sit");
        assert!(!r.text.is_empty());
        assert!(!r.act.is_empty());
        for word in ["bad", "no", "stupid", "poop", "talk", "up"] {
            assert!(t.respond(5_000_006, 30, word, 0).is_some(), "the Husky knows {word}");
        }
        assert!(t.respond(5_000_006, 1, "hello there", 0).is_none());
    }
}
