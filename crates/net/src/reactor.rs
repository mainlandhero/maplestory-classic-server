//! Reactors - the breakable boxes. `0x0484` puts one on the field, `0x0478` moves it to its
//! next state, `0x0485` takes it away; `0x032F` is the client hitting one.
//!
//! The owner, 2026-09-13: *"The next quest we need to fix is 'Pio's Collecting Recycled Goods'.
//! The quest itself is expecting the correct items, but the items come out of breakable
//! wooden boxes which we do not spawn right now. We need to spawn them and provide the drops
//! for the Wooden Box."*
//!
//! # Where the numbers come from
//!
//! The client's reactor pool is `FUN_141f2c0a0`, reached from `CField::OnPacket`
//! (`FUN_141820080`, call at `0x141821f5a`). Its switch is `add edx, -0x478 / cmp edx, 0x14`
//! over a 21-entry table at `0x141f2c3e4`, so the pool owns opcodes `0x0478..=0x048C`, and
//! decoding the table gives **[L]**:
//!
//! ```text
//! 0x0478  FUN_141f2c440   reads u32, u8, u16, u16, u16, u8, u32, u32   ChangeState
//! 0x0484  FUN_141f2d9c0   reads u8, u32, u32, u8, u16, u16, u8, str    EnterField
//! 0x0485  FUN_141f2d7f0   reads u32                                    LeaveField
//! 0x048C  FUN_141f2ef50   reads u32, u8, u8, u16, u16, u8              (not sent)
//! ```
//!
//! (`tools/reads.py <fn> 2`.) The reference server's `ReactorPool.java` encodes exactly those
//! widths in exactly that order for `reactorEnterField` - `byte 0, int objectId, int
//! templateId, byte state, position, byte flip, string name` - and for `reactorChangeState`
//! - `int objectId, byte state, position, short delay, byte eventIdx, int stateLength, int
//! ownerId`. **[L]** for the widths from this client, **[R]** for the names; the two agree
//! field for field, which is the strongest a reference reading gets here.
//!
//! The pool was found from its data, not its opcode: the format strings `Reactor/%07d.img`
//! and `Reactor/%s.img` are read through `.data` slots (`tools/dataref.py 0x143a47158`) by
//! the template loaders `FUN_141f45880` / `FUN_141f47130`, whose callers climb to the pool.
//!
//! # The hit, outbound
//!
//! `research/msexe-send-opcodes.txt` lists the client's builders in the reactor pool's code
//! range: `0x032F` from five sites (`FUN_141f30aa0`, `FUN_141f36e20`, ...) whose encode order
//! (`research/msexe-packet-fields.txt`) is `u32, u32, u16, u32` - the classic
//! `ReactorHit`: object id, hit option, delay, skill id. `0x0330` (`FUN_141f31630`, `u32 ...`)
//! is the touch. **[I]** until a capture; the world server logs the body either way.
//!
//! # The Wooden Box itself
//!
//! `Reactor.wz/0000001.img` **[L]**: `info wooden box`; states `0`, `1`, `2`, `3` each with
//! an `event 0 { type 0, state N+1 }` and a `hit` animation; state `4` has no event - broken.
//! So four hits break it. Its placements are `Map.wz/<map>.img/reactor/<n>` with `id
//! 0000001`, `x`, `y`, `reactorTime` (seconds to respawn; 120 on all but three) and `f`.
//! 19 of them on the six Amherst maps (`gm-handbook/reactors.txt`), which is what the fan
//! site says too.
//!
//! Nothing here authenticates; the channel socket carries no credentials.
use crate::packet::{PacketReader, PacketWriter};

/// Server -> client: a reactor changes state (a hit landed, or it broke).
pub const REACTOR_CHANGE_STATE: u16 = 0x0478;
/// Server -> client: a reactor appears on the field.
pub const REACTOR_ENTER_FIELD: u16 = 0x0484;
/// Server -> client: a reactor is removed from the field.
pub const REACTOR_LEAVE_FIELD: u16 = 0x0485;
/// Client -> server: the player hit a reactor. `u32 objectId, u32 hitOption, u16 delay,
/// u32 skillId` by the builder's encode order. **[I]**
pub const CLIENT_REACTOR_HIT: u16 = 0x032F;
/// Client -> server: the player touched a reactor (walked into one that reacts to touch).
/// Not answered; the Wooden Box is hit, not touched.
pub const CLIENT_REACTOR_TOUCH: u16 = 0x0330;

/// One reactor on a field, as `0x0484` describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldReactor {
    pub object_id: u32,
    /// `Reactor.wz` image id: 1 for the Wooden Box.
    pub template_id: u32,
    /// 0 is fresh; each hit advances it; the last state is the broken one.
    pub state: u8,
    pub x: i16,
    pub y: i16,
    /// The WZ `f`: drawn mirrored.
    pub flip: bool,
    /// The WZ `name`, usually empty. Sent as the client reads it.
    pub name: String,
}

/// `0x0484`. `u8 0, u32 objectId, u32 templateId, u8 state, i16 x, i16 y, u8 flip, str name`.
pub fn reactor_enter_field(r: &FieldReactor) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u8(0); //                141f2db10
    w.u32(r.object_id); //     141f2db23
    w.u32(r.template_id); //   141f2db2e
    w.u8(r.state); //          141f2db4d
    w.i16(r.x); //             141f2db5e
    w.i16(r.y); //             141f2dd23
    w.u8(u8::from(r.flip)); // 141f2dee3
    w.str(&r.name); //         141f2defc
    w.into_vec()
}

/// `0x0478`. `u32 objectId, u8 state, i16 x, i16 y, u16 delay, u8 eventIdx, u32 stateLength,
/// u32 ownerId`. `delay` is the client's own number from its hit packet, echoed; `event_idx`
/// is which `event` of the previous state fired - the box has one, index 0.
pub fn reactor_change_state(
    object_id: u32,
    state: u8,
    x: i16,
    y: i16,
    delay: u16,
    event_idx: u8,
    state_length: u32,
    owner_id: u32,
) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(object_id);
    w.u8(state);
    w.i16(x);
    w.i16(y);
    w.u16(delay);
    w.u8(event_idx);
    w.u32(state_length);
    w.u32(owner_id);
    w.into_vec()
}

/// `0x0485`. The handler reads the `u32 objectId` and nothing else at depth 3; the
/// reference sends `byte state, position` after it, and surplus bytes are never looked at
/// (the frame carries its own length), so they are sent for the sake of the one reading
/// this client's decoder could add later.
pub fn reactor_leave_field(object_id: u32, state: u8, x: i16, y: i16) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(object_id);
    w.u8(state);
    w.i16(x);
    w.i16(y);
    w.into_vec()
}

/// A decoded [`CLIENT_REACTOR_HIT`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReactorHit {
    pub object_id: u32,
    pub hit_option: u32,
    pub delay: u16,
    pub skill_id: u32,
}

/// Parse the body after the opcode. The object id is required; the rest reads as zero
/// when a shorter body arrives, so the first capture cannot be refused for its tail.
pub fn parse_reactor_hit(body: &[u8]) -> Option<ReactorHit> {
    let mut r = PacketReader::new(body);
    let object_id = r.u32().ok()?;
    let hit_option = r.u32().unwrap_or(0);
    let delay = r.u16().unwrap_or(0);
    let skill_id = r.u32().unwrap_or(0);
    Some(ReactorHit { object_id, hit_option, delay, skill_id })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wooden_box() -> FieldReactor {
        FieldReactor {
            object_id: 6000,
            template_id: 1,
            state: 0,
            x: 610,
            y: 259,
            flip: false,
            name: String::new(),
        }
    }

    /// The widths the client reads at 0x141f2db10..0x141f2defc, in that order.
    #[test]
    fn enter_field_is_the_eight_reads_of_fun_141f2d9c0() {
        let b = reactor_enter_field(&wooden_box());
        assert_eq!(b.len(), 1 + 4 + 4 + 1 + 2 + 2 + 1 + 2, "and an empty name");
        assert_eq!(b[0], 0);
        assert_eq!(u32::from_le_bytes(b[1..5].try_into().unwrap()), 6000);
        assert_eq!(u32::from_le_bytes(b[5..9].try_into().unwrap()), 1);
        assert_eq!(b[9], 0, "state");
        assert_eq!(i16::from_le_bytes(b[10..12].try_into().unwrap()), 610);
        assert_eq!(i16::from_le_bytes(b[12..14].try_into().unwrap()), 259);
        assert_eq!(b[14], 0, "flip");
        assert_eq!(&b[15..17], &[0, 0], "u16 length 0, no name bytes");
    }

    #[test]
    fn change_state_is_the_eight_reads_of_fun_141f2c440() {
        let b = reactor_change_state(6000, 4, 610, 259, 150, 0, 0, 215);
        assert_eq!(b.len(), 4 + 1 + 2 + 2 + 2 + 1 + 4 + 4);
        assert_eq!(b[4], 4, "the new state");
        assert_eq!(u16::from_le_bytes(b[9..11].try_into().unwrap()), 150, "the delay echoed");
        assert_eq!(u32::from_le_bytes(b[16..20].try_into().unwrap()), 215, "the owner");
    }

    #[test]
    fn leave_field_leads_with_the_one_read_the_handler_makes() {
        let b = reactor_leave_field(6000, 4, 610, 259);
        assert_eq!(u32::from_le_bytes(b[0..4].try_into().unwrap()), 6000);
        assert_eq!(b.len(), 9);
    }

    #[test]
    fn a_hit_parses_and_a_short_hit_still_names_its_reactor() {
        let mut body = Vec::new();
        body.extend_from_slice(&6000u32.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        body.extend_from_slice(&150u16.to_le_bytes());
        body.extend_from_slice(&0u32.to_le_bytes());
        assert_eq!(
            parse_reactor_hit(&body),
            Some(ReactorHit { object_id: 6000, hit_option: 0, delay: 150, skill_id: 0 })
        );
        assert_eq!(parse_reactor_hit(&body[..4]).map(|h| h.object_id), Some(6000));
        assert_eq!(parse_reactor_hit(&body[..3]), None);
    }
}
