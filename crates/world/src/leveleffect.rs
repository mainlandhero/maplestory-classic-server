//! **`0x02AF` `UserEffectRemote`, effect `0` — everyone else on the map sees and hears the
//! level-up.**
//!
//! The owner, 2026-09-08: *"the level up sound was not broadcast to other players"*.
//!
//! Tags on every claim: **[L]** read off this client's listing or its WZ, **[D]** derived
//! from two or more [L] facts, **[I]** inferred.
//!
//! # The levelling player's own animation is untouched by this file
//!
//! `session::combat::award_experience` sends `0x007C` (`net::stats::STAT_CHANGED`) with the
//! level bit set, and the `0x007C` handler plays `Effect/BasicEff.img/LevelUp` by itself when
//! the level in the packet is higher than the one the client holds. That is why the bug is
//! *only* about observers, and it is why this module adds a broadcast rather than changing
//! anything the leveller receives. **Nothing here goes to the levelling player** — see
//! [`publish_level_up`] and the `Bus::publish` contract it uses.
//!
//! # The opcode: `0x02AF`, and how it was verified rather than picked
//!
//! Not by grepping for a known list. **Both dispatch tables were enumerated whole**, which is
//! `CLAUDE.md`'s "enumerate before you filter", and the effect handler `FUN_1427863f0` came
//! out of the enumeration exactly twice: **[L]**
//!
//! ```text
//! FUN_1429bb720  remote users   1429bb940 lea eax,[rsi-0x29e] / cmp eax,0x26 / ja
//!                               1429bb951 mov ecx,[r14+rax*4+0x29bbc34] / add rcx,r14 / jmp rcx
//!   39 entries resolved, 0 unresolved
//!   index 0x11 -> arm 0x1429bba36 -> call 0x1427863f0     opcode 0x29e + 0x11 = 0x02AF
//!   and NO other index in the whole table reaches 0x1427863f0
//!
//! FUN_14289a3a0  the local user 14289a3f3 lea eax,[rdx-0x2c5] / cmp eax,0xd9 / ja
//!                               14289a40d mov edx,[rcx+rax*4+0x289d660] / jmp rdx
//!   218 entries resolved, 0 unresolved
//!   index 0x0C -> call 0x1427863f0                        opcode 0x2c5 + 0x0c = 0x02D1
//!   and NO other index in the whole table reaches 0x1427863f0
//! ```
//!
//! `0x02D1` is the **positive control that this handler draws on the owner's screen**, and it is a
//! measured one rather than a static one: `STATUS.md` records *"the quest completion SFX is
//! now working"* (effect 15) and *"I do see 10 in blue above the character"* (effect `0x41`),
//! both through `FUN_1427863f0`. So the remote sibling is not a new handler being trusted for
//! the first time — it is the same function reached by the other door.
//!
//! # The body: `u32 charId, u8 effect`. Five bytes. **[L]**
//!
//! ```text
//! 1429bb745  call 0x1406e8c20        <<< READ u32     the ONLY read before dispatch
//! 1429bb74a  mov  ebp, eax
//! 1429bb74c  mov  r8, [rbx+0xf8]     ; the user-pool hash
//! 1429bb766  div  rcx                ; bucket = charId % [rbx+0x100]
//! 1429bb769  mov  rcx, [r8+rdx*8]
//! 1429bb776  cmp  [rcx+0x10], ebp    ; walk the chain comparing the id
//! 1429bb77b  mov  rcx, [rcx+8]
//! ```
//!
//! The `u32` is consumed before the arm at `0x1429bba36` hands the packet on, so the effect
//! handler's own single read — the `u8` at `0x14278644e` — lands on byte 4. Five bytes in
//! all, which is what [`net::userpool::user_effect_remote`] already builds; this module
//! **delegates to it rather than restating the layout**, so this client cannot grow two
//! builders that drift apart.
//!
//! ## The gate that consumes the id, said plainly
//!
//! `GetUser(charId)` above returns null for an id the receiving client does not hold, and the
//! function then falls straight to its epilogue at `0x1429bbc19` having done nothing — **no
//! error, no sound, nothing on screen**. So this packet is only useful to an observer who has
//! already been sent the leveller's `0x0224` `UserEnterField`, which is what
//! `broadcast::Presence::spawn` does on field entry. A broadcast to a client that never got
//! the spawn is dropped in silence. **[L]** for the null path; **[D]** that our spawn is what
//! satisfies it.
//!
//! # The effect id: `0`, read out of the client and not taken from an enum
//!
//! `research/level-up.md` and `crates/net/src/questeffect.rs` both say effect 0 is the
//! level-up, and a table row written from a quick read is a claim. It was re-derived here,
//! and by a different route than the one those files used — from the **string** back to the
//! code, rather than from the arm forward to the string:
//!
//! ```text
//! "Effect/BasicEff.img/LevelUp" (UTF-16) lives at 0x1432AE4A0, and exactly one .data
//! qword holds that pointer: 0x143A46F48.  tools/dataref.py 0x143a46f48 gives two readers:
//!     read at 0x14278BDC8  in 0x1427863F0      <- the effect handler
//!     read at 0x142D54BDB  in 0x142D54780
//! "LevelUp" (the bare sound name) is at 0x1432B1F40, held by 0x143A48490, read at
//!     0x14278BE09  in 0x1427863F0  and  0x142D54CCF in 0x142D54780
//! ```
//!
//! and `0x14278BDC8` / `0x14278BE09` are both inside the arm the effect switch jumps to for
//! **index 0**: **[L]**
//!
//! ```text
//! 14278bd5d  movzx ebx, byte [rbp+0x80]          ; the effect byte
//! 14278bd7d  cmp   ebx, 0x54 / ja <exit>
//! 14278bd8d  mov   ecx, [rdx + rbx*4 + 0x2791348]
//! 14278bd97  jmp   rcx
//!   table[0] -> 0x14278BD99 .. reads 0x143A46F48 at bdc8 and 0x143A48490 at be09
//! ```
//!
//! **The instrument was checked against a control before any of that was believed.** The same
//! two searches run on `QuestClear` reproduce the pair `research/quest-complete-effect.md`
//! already published — path slot `0x143A46F58`, bare-name slot `0x143A484A0`, both read
//! inside `FUN_1427863F0` — so a search that can find the known answer found this one.
//!
//! An earlier pass of mine scanned the arm for `48 8B 0D` (`mov rcx,[rip+d]`) and came back
//! empty for effect 0 while finding strings for effects 13, 14, 15 and 21. That empty result
//! was a property of the search: effect 0 loads its two strings into **`rdx`** (`48 8B 15`).
//! Recorded because it is this project's most repeated failure and it happened again here.
//!
//! # Two ways the client drops it, neither of them an error
//!
//! Both sit between the handler's two switches, apply to every effect, and effect 0 is
//! **not** one of the three that bypass them: **[L]**
//!
//! ```text
//! 14278bd29  call 0x141892840        ; the current field; NULL -> bl = 1
//! 14278bd4a  call 0x142826340        ; a user-state predicate -> al
//! 14278bd5d..14278bd7d               ; effects 0x4F, 0x50, 0x51 skip the test
//! 14278bd75  test dl,dl / je <exit>  ; either non-zero -> NOTHING HAPPENS
//! ```
//!
//! So an observer whose client is between a `SetField` and its field object does not hear it.
//! That is a missed sound, not a hang: nothing here is a request and nothing is awaited, so
//! `CLAUDE.md`'s **"always answer"** is not in play — this packet is never sent *instead of* a
//! reply, only in addition to one.
//!
//! (While reading that block: `crates/net/src/questeffect.rs`'s doc says the three bypassing
//! effects are `0x4F`, `0x50` and `0x52`. The listing says `0x4F`, `0x50`, `0x51` —
//! `sub ecx,0x4f / je`, `sub ecx,1 / je`, `cmp ecx,1 / je`, so the third is `0x50 + 1`.
//! Nothing in this file depends on it and no code changes; noted so the next reader of that
//! block is not misled. **[L]**)
//!
//! # It has never been on a wire
//!
//! `CLAUDE.md` § "Built is not wired", loudly, because on screen an unwired packet is
//! indistinguishable from one that does not exist.
//!
//! The archive was grepped with a **working positive control** rather than eyeballed:
//! `0x02D1` — the local sibling this whole file leans on — is in **116** of the files under
//! `previous-runs/` and `research/fixtures/`, and `0x0224` `UserEnterField` in **38**, so the
//! search can find a positive. `0x02AF` is in **one**, and it is an artefact, named here so
//! nobody counts it as an observation:
//!
//! * `research/fixtures/sweep-01f2-03c7-exit.log:427` — `01:05:26 >>> opcode 0x02AF +32B`.
//!   That file is `tools/handshake_probe.py`'s login-stage sweep on a fresh socket with a
//!   32-byte junk body, so `CField::OnPacket` was never the active dispatcher and
//!   `FUN_1429bb720` was never reached. It is the same artefact `crate::remoteattack`'s
//!   module docs already name for `0x029E..0x02A1`. **It says nothing about this handler.**
//!
//! So: whether an observer's screen actually plays the animation and the sound is
//! **unmeasured, and stays unmeasured until a two-client run**.

use crate::broadcast::{Bus, SubscriberId};
use crate::session::Reply;

/// **`0x02AF` `UserEffectRemote`** — `u32 charId, u8 effect`.
///
/// Restated from [`net::userpool::USER_EFFECT_REMOTE`] rather than aliased, so a reader of
/// this file sees the number; pinned equal to it by a test below.
pub const USER_EFFECT_REMOTE: u16 = net::userpool::USER_EFFECT_REMOTE;

/// **Effect `0`** — `Effect/BasicEff.img/LevelUp` plus the `LevelUp` sound. See the module
/// docs for the two `.data` slots and their readers.
pub const EFFECT_LEVEL_UP: u8 = net::stats::EFFECT_LEVEL_UP;

/// The base of `FUN_1429bb720`'s jump table: `1429bb940 lea eax,[rsi-0x29e]`. **[L]**
///
/// Here so the test below can assert `0x29E + 0x11 == 0x02AF` against the arithmetic the
/// client actually does, rather than against a number copied from a comment.
pub const REMOTE_TABLE_BASE: u16 = 0x029E;

/// Index `0x11` in that table — the arm at `0x1429bba36`, `call 0x1427863f0`. **[L]**
pub const REMOTE_TABLE_INDEX: u16 = 0x11;

/// The finished packet: *"character `char_id` just levelled"*, for every screen but theirs.
///
/// Five bytes, built by [`net::userpool::user_effect_remote`]. **Never send this to the
/// player who levelled** — a client does not hold itself in its remote user pool, so the
/// `GetUser` at `0x1429bb769` would return null and the packet would be dropped in silence.
/// [`publish_level_up`] is the only intended caller and it excludes the publisher by
/// construction.
pub fn level_up_remote(char_id: u32) -> Reply {
    Reply {
        opcode: USER_EFFECT_REMOTE,
        body: net::userpool::user_effect_remote(char_id, EFFECT_LEVEL_UP),
        what: describe(char_id),
    }
}

/// The log line. Says what it is *for*, because an unwired-looking packet in `world.log` is
/// how the last three of these were misread.
pub fn describe(char_id: u32) -> String {
    format!(
        "UserEffectRemote: character {char_id} levelled - effect {EFFECT_LEVEL_UP} \
         (Effect/BasicEff.img/LevelUp + the LevelUp sound) on every OTHER screen on this map. \
         The leveller's own animation comes from their 0x007C and is not this packet."
    )
}

/// **Tell everyone else on `from`'s map that `char_id` levelled up.**
///
/// Returns how many other connections were standing on that map at the moment of the
/// broadcast — the count `broadcast.rs` exists to make checkable, and the cheapest possible
/// evidence that a fan-out went to the right set. `0` is the ordinary solo case and is not an
/// error.
///
/// # Three things it deliberately does not do
///
/// * **It does not reach the leveller.** `Bus::publish` skips `from`. That is what keeps this
///   from double-playing the animation the `0x007C` already triggers, and it is enforced by
///   the bus rather than by a filter here, because a filter here is the thing that gets
///   forgotten.
/// * **It does not supersede.** `None` for the supersede key: two level-ups are two events,
///   the same rule that keeps two swings from rendering as one hit. A player who levels twice
///   while an observer is stalled should be seen to level twice.
/// * **It returns rather than erroring when the publisher is in no field.** A session that
///   has left its field — mid-channel-change, or at character select — has no map to broadcast
///   on. There is nothing to answer here, so there is nothing to fail: this is a side effect
///   of a level-up, never a reply to a request. `CLAUDE.md` § "always answer" is satisfied by
///   the `0x007C` that the caller sends regardless of what this does.
pub fn publish_level_up(bus: &Bus, from: SubscriberId, char_id: u32) -> usize {
    let Some(map) = bus.map_of(from) else {
        // Not in a field. Nobody to tell, and nothing owed to anyone.
        return 0;
    };
    let others = bus.others_on(from, map);
    bus.publish(from, map, level_up_remote(char_id), None);
    others
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::broadcast::Presence;

    fn presence(character: u32, map: u32) -> Presence {
        Presence {
            character,
            map,
            spawn: Reply { opcode: 0x0224, body: vec![], what: format!("spawn {character}") },
            farewell: Reply { opcode: 0x0225, body: vec![], what: format!("bye {character}") },
        }
    }

    /// The routing, asserted against the client's own arithmetic rather than against a
    /// number copied out of a comment. `0x29E + 0x11`, index `0x11` being the one arm in the
    /// whole 39-entry table that calls `0x1427863f0`.
    #[test]
    fn the_opcode_is_the_remote_tables_effect_entry() {
        assert_eq!(USER_EFFECT_REMOTE, 0x02AF);
        assert_eq!(REMOTE_TABLE_BASE + REMOTE_TABLE_INDEX, USER_EFFECT_REMOTE);
        // And it is NOT the local one. Sending 0x02D1 to an observer would be routed by
        // FUN_14289a3a0, which reads nothing before dispatching - so the charId would be
        // eaten as the effect byte and some other effect would play.
        assert_ne!(USER_EFFECT_REMOTE, net::stats::USER_EFFECT_LOCAL);
        assert_eq!(net::stats::USER_EFFECT_LOCAL, 0x02D1);
    }

    /// This module must never grow a second wire format. Both `net` builders and this one
    /// have to agree byte for byte.
    #[test]
    fn the_body_is_not_forked_from_nets_builder() {
        let r = level_up_remote(207);
        assert_eq!(r.body, net::userpool::user_effect_remote(207, EFFECT_LEVEL_UP));
        assert_eq!(r.body, net::stats::user_effect_remote(207, EFFECT_LEVEL_UP));
        assert_eq!(USER_EFFECT_REMOTE, net::stats::USER_EFFECT_REMOTE);
        assert_eq!(EFFECT_LEVEL_UP, net::questeffect::EFFECT_LEVEL_UP);
    }

    /// `u32 charId` then `u8 effect`, little-endian, five bytes - the read at `1429bb745`
    /// and the read at `14278644e` and nothing else.
    ///
    /// The id is a real one. `CLAUDE.md`: ids start at 200, and a reply carrying 1 has bitten
    /// this project before, so a test that only ever passes single-digit ids would not notice
    /// a builder that truncated to a byte.
    #[test]
    fn the_body_is_the_char_id_then_effect_zero() {
        let r = level_up_remote(200);
        assert_eq!(r.body.len(), 5);
        assert_eq!(u32::from_le_bytes(r.body[0..4].try_into().unwrap()), 200);
        assert_eq!(r.body[4], 0, "Effect/BasicEff.img/LevelUp is table index 0");
        assert_eq!(EFFECT_LEVEL_UP, 0);
        // A four-byte id, so a `as u8` anywhere on the path fails here rather than on screen.
        assert_eq!(level_up_remote(0x1234_5678).body, vec![0x78, 0x56, 0x34, 0x12, 0]);
    }

    /// **The effect id is 0, and that is the fact worth pinning.**
    ///
    /// This used to assert `EFFECT_LEVEL_UP <= 0x54` against the client's
    /// `14278bd7d cmp ebx,0x54 / ja <exit>` bound. Clippy was right to refuse it: the constant
    /// is `0`, the minimum of its type, so the comparison is **always true** and could not have
    /// failed if the id were wrong. That is `CLAUDE.md`'s "a test that pins what the code
    /// already does is not a check", in one line.
    ///
    /// So this asserts the id itself - which can disagree, and would if somebody read the
    /// effect table differently - and exercises the bound against a sibling that is *not* the
    /// type's minimum, so the bound assertion is a real one.
    #[test]
    fn the_effect_id_is_zero_and_the_switch_bound_holds_for_a_sibling_too() {
        const BOUND: u8 = 0x54; // 14278bd7d cmp ebx,0x54 / ja <exit>
        assert_eq!(
            EFFECT_LEVEL_UP, 0,
            "the level-up effect is index 0 of the client's effect table - Effect/BasicEff.img/\
             LevelUp is reached by the switch's case 0 arm at 0x14278bd99"
        );
        // Non-vacuous: quest-clear is 15, so this comparison has two possible answers.
        assert!(
            net::questeffect::EFFECT_QUEST_CLEAR <= BOUND,
            "a sibling effect must also sit inside the switch bound"
        );
    }

    /// **The bug, in one test.** Someone levels; the other player on the map hears it and
    /// the leveller does not get a second copy.
    ///
    /// Both halves are asserted in the same call, because "the observer got it" would pass
    /// against a version that also sent it to the leveller - and that version would play the
    /// leveller's animation twice, since their `0x007C` already plays it once.
    #[test]
    fn the_other_player_on_the_map_is_told_and_the_leveller_is_not() {
        let bus = Bus::new();
        let leveller = bus.join();
        let observer = bus.join();
        bus.enter_field(leveller, presence(200, 104_040_000));
        bus.enter_field(observer, presence(201, 104_040_000));
        // Drop the two arrival spawns so what follows is only what this test published.
        let _ = bus.drain(leveller);
        let _ = bus.drain(observer);

        assert_eq!(publish_level_up(&bus, leveller, 200), 1, "one other player on the map");

        let mail = bus.drain(observer);
        assert_eq!(mail.len(), 1, "exactly one packet: {mail:?}");
        assert_eq!(mail[0].opcode, 0x02AF);
        assert_eq!(mail[0].body, vec![200, 0, 0, 0, 0], "charId 200, effect 0");
        assert!(
            bus.drain(leveller).is_empty(),
            "the leveller's own animation is client-side off 0x007C; a second copy here \
             would be dropped by GetUser at 1429bb769 at best, and double-play at worst"
        );
    }

    /// A player on another map hears nothing - **and the control is in the same test**, so an
    /// empty result cannot be a function that publishes nothing at all.
    #[test]
    fn a_player_on_another_map_hears_nothing_but_one_on_this_map_does() {
        let bus = Bus::new();
        let leveller = bus.join();
        let same_map = bus.join();
        let far_away = bus.join();
        bus.enter_field(leveller, presence(200, 104_040_000));
        bus.enter_field(same_map, presence(201, 104_040_000));
        bus.enter_field(far_away, presence(202, 100_000_000));
        for who in [leveller, same_map, far_away] {
            let _ = bus.drain(who);
        }

        assert_eq!(publish_level_up(&bus, leveller, 200), 1, "far_away is not on this map");

        assert_eq!(bus.drain(same_map).len(), 1, "the control: this map does receive");
        assert!(bus.drain(far_away).is_empty(), "and map 100000000 does not");
    }

    /// Two level-ups are two packets. A supersede key here would coalesce them and an
    /// observer who was stalled would see one animation for two levels.
    #[test]
    fn two_level_ups_are_two_packets() {
        let bus = Bus::new();
        let leveller = bus.join();
        let observer = bus.join();
        bus.enter_field(leveller, presence(200, 1));
        bus.enter_field(observer, presence(201, 1));
        let _ = bus.drain(observer);

        publish_level_up(&bus, leveller, 200);
        publish_level_up(&bus, leveller, 200);

        assert_eq!(bus.drain(observer).len(), 2, "an event, not a state");
    }

    /// Alone on the map: zero recipients, no panic, and **nothing queued anywhere**. The
    /// ordinary case, and the one every single-client run has been.
    #[test]
    fn levelling_alone_tells_nobody_and_is_not_an_error() {
        let bus = Bus::new();
        let leveller = bus.join();
        let elsewhere = bus.join();
        bus.enter_field(leveller, presence(200, 1));
        bus.enter_field(elsewhere, presence(201, 2));
        let _ = bus.drain(elsewhere);

        assert_eq!(publish_level_up(&bus, leveller, 200), 0);
        assert!(bus.drain(leveller).is_empty());
        assert!(bus.drain(elsewhere).is_empty(), "and it did not leak to another map");
    }

    /// A session that is in no field - at character select, or between a channel change's two
    /// halves - has no map to broadcast on. It reports zero and posts nothing, rather than
    /// picking a map or failing. Nothing is owed to a caller here: the `0x007C` that carries
    /// the level goes out either way.
    #[test]
    fn a_publisher_with_no_field_posts_nothing() {
        let bus = Bus::new();
        let on_the_map = bus.join();
        let between_fields = bus.join();
        bus.enter_field(on_the_map, presence(200, 1));
        bus.enter_field(between_fields, presence(201, 1));
        bus.leave_field(between_fields);
        let _ = bus.drain(on_the_map);

        assert_eq!(publish_level_up(&bus, between_fields, 201), 0, "no map, no broadcast");
        assert!(bus.drain(on_the_map).is_empty(), "and it did not fall back to some map");

        // The control, so the zero above is about the missing field and not about the bus
        // being inert: from a connection that IS in a field, the same call lands.
        assert_eq!(publish_level_up(&bus, on_the_map, 200), 0, "nobody else is left here");
        let watcher = bus.join();
        bus.enter_field(watcher, presence(202, 1));
        let _ = bus.drain(watcher);
        assert_eq!(publish_level_up(&bus, on_the_map, 200), 1);
        assert_eq!(bus.drain(watcher).len(), 1);
    }

    /// Three observers, one broadcast, three copies - and the count says so. A fan-out that
    /// reached one of the three would otherwise look identical from any single mailbox.
    #[test]
    fn every_other_player_on_the_map_gets_one_copy() {
        let bus = Bus::new();
        let leveller = bus.join();
        bus.enter_field(leveller, presence(200, 7));
        let watchers: Vec<_> = (0..3)
            .map(|n| {
                let id = bus.join();
                bus.enter_field(id, presence(201 + n, 7));
                id
            })
            .collect();
        for w in &watchers {
            let _ = bus.drain(*w);
        }
        let _ = bus.drain(leveller);

        assert_eq!(publish_level_up(&bus, leveller, 200), 3);
        for w in &watchers {
            let mail = bus.drain(*w);
            assert_eq!(mail.len(), 1);
            assert_eq!(mail[0].opcode, USER_EFFECT_REMOTE);
            assert_eq!(mail[0].body, vec![200, 0, 0, 0, 0]);
        }
    }

    /// The `what` string is what a reader of `world.log` sees, and the last three packets of
    /// this shape were misread as telemetry. It has to name the opcode's purpose and say the
    /// leveller is not the audience.
    #[test]
    fn the_log_line_says_what_it_is_for() {
        let what = level_up_remote(200).what;
        assert!(what.contains("UserEffectRemote"), "{what}");
        assert!(what.contains("200"), "{what}");
        assert!(what.contains("OTHER"), "{what}");
    }
}
