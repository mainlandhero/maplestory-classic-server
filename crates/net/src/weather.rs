//! **`0x01B7` - a map-wide weather effect**, the Sprinkled Chocolate / Flower Petals / Candy /
//! Maple Leaves and Fireworks items (`5120005..5120009`) and their siblings.
//!
//! The owner, 2026-09-30: *"an atmospheric effect, which should display my chosen message with the
//! particular item effect as an atmospheric effect for everyone present in the map for 30
//! seconds then gradually fade out."*
//!
//! # The handler [L]
//!
//! `CField::OnPacket` dispatches `0x01B7` to `FUN_141853820` (`research/msexe-field-cases.txt`
//! row 41; decompiled in `research/msexe-weather-01b7.c`). It reads:
//!
//! ```text
//! u32 itemId                      0 skips everything below
//! str message
//! u32 seconds                     default 200 when the item is 0
//! u8  flag                        non-zero: 0x78 more bytes follow
//! ```
//!
//! and calls `FUN_14185b1c0(field, itemId, message, 0, seconds * 1000, ...)`. That routine reads
//! the item's own `info/path` and `info/floatType` (`floatType` reached through the pointer at
//! `0x143a48340`) - so the art and the way it falls come from the item, and the packet carries
//! only which item, what it says and for how long. `0x00AC` type 12 calls the same routine with
//! a fixed 10 000 ms; this one takes the duration from the wire, which is why it is used.
//!
//! **[I]**: the gradual fade at the end is the client's own - nothing on the wire asks for it.
//!
//! # The request [L]
//!
//! `0x0116`, the owner's own use, 2026-09-30 04:01:07 (`research/fixtures/weather-item-attempt-
//! 2026-09-30-world.log`): `tick, u16 slot 8, u32 5120005, str "the owner's Chocolatey Message:
//! Message for chocolate"` - the client wrote the prefix itself, and there is no byte after the
//! text.

use crate::packet::{PacketReader, PacketWriter};

/// Server -> client: a weather effect on this map.
pub const BLOW_WEATHER: u16 = 0x01B7;

/// How long one lasts - the owner's thirty seconds.
pub const SECONDS: u32 = 30;

/// Is this one of the weather items? Every item in `Item/Cash/0512.img` [L]: `5120000..5120009`
/// and the two GM event ones at `5121000..5121001`.
pub fn is_weather_item(item_id: u32) -> bool {
    item_id / 10_000 == 512
}

/// **The GM weather that goes with a GM's Blessing**: `2023000` Wind -> `5121000` (`GMevent1`),
/// `2023001` Precision -> `5121001` (`GMevent2`). **[L]** from the client's own data: each Cash
/// item in `Item/Cash/0512.img` names its blessing as `info/stateChangeItem`. The weather
/// routine does not apply that item itself - `stateChangeItem` is read only by the item-info
/// loader (`FUN_142ccc3a0`), so the server grants the buff.
pub fn blessing_weather(blessing_item: u32) -> Option<u32> {
    match blessing_item {
        2_023_000 => Some(5_121_000),
        2_023_001 => Some(5_121_001),
        _ => None,
    }
}

/// A decoded weather-item use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatherUse {
    pub slot: u16,
    pub item_id: u32,
    pub text: String,
}

/// Read a `0x0116` weather body (without the opcode). `None` when it is short.
pub fn parse_weather_use(body: &[u8]) -> Option<WeatherUse> {
    let mut r = PacketReader::new(body);
    let _tick = r.u32().ok()?;
    let slot = r.u16().ok()?;
    let item_id = r.u32().ok()?;
    let text = r.str().ok()?;
    Some(WeatherUse { slot, item_id, text })
}

/// The `0x01B7` body: `item`'s effect with `message`, for `seconds`.
pub fn blow_weather(item_id: u32, message: &str, seconds: u32) -> Vec<u8> {
    let mut w = PacketWriter::new();
    w.u32(item_id);
    w.str(message);
    w.u32(seconds);
    w.u8(0); // no 0x78-byte block
    w.into_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_captured_use_decodes_and_the_effect_is_the_shape_the_client_reads() {
        let mut body = hex("9075f00f080005204e00");
        let text = "the owner's Chocolatey Message: Message for chocolate";
        body.extend((text.len() as u16).to_le_bytes());
        body.extend(text.as_bytes());
        assert_eq!(body.len(), 60, "the captured length");
        let u = parse_weather_use(&body).unwrap();
        assert_eq!(u, WeatherUse { slot: 8, item_id: 5_120_005, text: text.into() });

        let b = blow_weather(5_120_005, "Hi", SECONDS);
        let mut r = PacketReader::new(&b);
        assert_eq!(r.u32().unwrap(), 5_120_005);
        assert_eq!(r.str().unwrap(), "Hi");
        assert_eq!((r.u32().unwrap(), r.u8().unwrap()), (30, 0), "thirty seconds, no extra block");
        assert!(r.u8().is_err());
        for id in [5_120_000, 5_120_005, 5_120_009, 5_121_001] {
            assert!(is_weather_item(id), "{id}");
        }
        assert!(!is_weather_item(5_070_000) && !is_weather_item(5_130_000));
    }

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }
}
