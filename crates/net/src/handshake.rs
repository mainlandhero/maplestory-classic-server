//! The greeting - the first bytes on the connection, and the server speaks first.
//!
//! Layout and every gate are in `docs/handshake.md`, decoded from `FUN_1415d10e0`. This is
//! the same 48 bytes `tools/handshake_probe.py` has been sending since the handshake was
//! solved; it is the one part of the transport that is finicky and already working, so it
//! is reproduced field for field rather than rederived.

/// The IV the client encrypts **its** stream with - field `J`, landing at `conn+0xe8`.
/// We decrypt what the client sends with this one.
pub const CLIENT_TX_IV: u32 = 0x5230_7801;

/// The IV the client decrypts with - field `K`, landing at `conn+0xec`. We encrypt
/// everything we send with this one.
pub const CLIENT_RX_IV: u32 = 0x5230_7802;

/// The client's own protocol version. Unrelated to the WZ data version of 779, which is a
/// conflation that has already cost a sweep.
pub const PROTOCOL_VERSION: u32 = 100;

fn put_str(out: &mut Vec<u8>, s: &[u8]) {
    out.extend_from_slice(&(s.len() as u16).to_le_bytes());
    out.extend_from_slice(s);
}

/// Build the greeting, length-prefixed and ready to write.
///
/// The gates, each of which produces the *same* "client is outdated" dialog when it fails,
/// which is why they hid each other for so long:
///
/// | field | must be | line |
/// |---|---|---|
/// | `G` | `1`, with bit `0x8000` clear | 606 / 609 |
/// | `H` | `1` | 606 |
/// | `I` | a string parsing to `0` | 438 |
/// | `L` | `1` | 437 |
/// | `high` | `100` on a first connect | 567 |
/// | `low` | `<= 100` | 525 |
/// | `temp` | `0`, which selects the checked path | 562 |
pub fn greeting(client_tx_iv: u32, client_rx_iv: u32) -> Vec<u8> {
    let mut body = Vec::new();
    // Gated on cfg+0x48 != 0, which is set for the login connection.
    body.extend_from_slice(&0u16.to_le_bytes()); // A
    put_str(&mut body, b""); // B
    body.extend_from_slice(&0u32.to_le_bytes()); // C
    body.extend_from_slice(&0u32.to_le_bytes()); // D
    body.push(0); // E
    body.push(0); // F

    // Always present.
    body.extend_from_slice(&1u16.to_le_bytes()); // G, flag bit clear
    body.extend_from_slice(&1u32.to_le_bytes()); // H
    put_str(&mut body, b""); // I, atoi("") == 0
    body.extend_from_slice(&client_tx_iv.to_le_bytes()); // J -> conn+0xe8
    body.extend_from_slice(&client_rx_iv.to_le_bytes()); // K -> conn+0xec
    body.push(1); // L

    // Gated again.
    body.extend_from_slice(&1u32.to_le_bytes()); // version range low
    body.extend_from_slice(&PROTOCOL_VERSION.to_le_bytes()); // version range high
    body.extend_from_slice(&0u32.to_le_bytes()); // nClientVersion_Temp
    body.push(0); // M
    body.push(0); // N

    // Always present.
    body.push(0); // O, locale -> conn+0x0c

    let mut out = Vec::with_capacity(2 + body.len());
    out.extend_from_slice(&(body.len() as u16).to_le_bytes());
    out.extend_from_slice(&body);
    out
}

/// Build the greeting for a **game channel** connection.
///
/// A channel is not a login server on another port. `conn+0x48` is the connection *type* -
/// non-zero for login, zero for a channel - and `FUN_1415d10e0` reads it three times, each
/// read changing the wire:
///
/// | read | when `conn+0x48 == 0` (a channel) |
/// |---|---|
/// | greeting parse | the leading `A..F` block is **not read** |
/// | version block | `low`, `high`, `temp` are **not read** |
/// | `FUN_1406e9a65` | the body cipher is a byte shift, not AES |
///
/// So this greeting is [`greeting`] with both optional blocks removed. Sending the login
/// greeting to a channel makes the client read `G` from where `A` sits - our `A` is
/// `00 00`, so it reads `G = 0`, fails `G == 1 && H == 1`, and raises source line 840 with
/// `0x22000007`: **"The client is outdated"**. That is measured, from the client's own
/// uploaded error log, and it is what ended the first run to enter the world.
///
/// **This shape is derived from the parse, not yet confirmed on screen.** What is certain
/// is that the login greeting is wrong here; that this is right is the best reading of
/// `FUN_1415d10e0` and nothing more until a run says so.
///
/// See `docs/transport.md`.
pub fn channel_greeting(client_tx_iv: u32, client_rx_iv: u32) -> Vec<u8> {
    let mut body = Vec::new();
    // No A..F block: a channel connection does not read one.
    body.extend_from_slice(&1u16.to_le_bytes()); // G, flag bit 0x8000 clear
    body.extend_from_slice(&1u32.to_le_bytes()); // H
    put_str(&mut body, b""); // I, atoi("") == 0
    body.extend_from_slice(&client_tx_iv.to_le_bytes()); // J -> conn+0xe8
    body.extend_from_slice(&client_rx_iv.to_le_bytes()); // K -> conn+0xec
    body.push(1); // L
    // No version block either: `low`, `high` and `temp` are not read on this path.
    body.push(0); // M
    body.push(0); // N
    body.push(0); // O, locale -> conn+0x0c

    let mut out = Vec::with_capacity(2 + body.len());
    out.extend_from_slice(&(body.len() as u16).to_le_bytes());
    out.extend_from_slice(&body);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact bytes in `docs/handshake.md`, which is the greeting the client has
    /// accepted on every run since the handshake was solved. If this test fails the
    /// server will not get past the first packet, and the symptom on screen is an
    /// "outdated client" dialog that names no cause.
    #[test]
    fn reproduces_the_greeting_the_client_accepts() {
        let want: Vec<u8> = vec![
            0x2E, 0x00, // length = 46
            0x00, 0x00, // A
            0x00, 0x00, // B, empty string
            0x00, 0x00, 0x00, 0x00, // C
            0x00, 0x00, 0x00, 0x00, // D
            0x00, // E
            0x00, // F
            0x01, 0x00, // G
            0x01, 0x00, 0x00, 0x00, // H
            0x00, 0x00, // I, empty string
            0x01, 0x78, 0x30, 0x52, // J
            0x02, 0x78, 0x30, 0x52, // K
            0x01, // L
            0x01, 0x00, 0x00, 0x00, // version low
            0x64, 0x00, 0x00, 0x00, // version high = 100
            0x00, 0x00, 0x00, 0x00, // temp
            0x00, // M
            0x00, // N
            0x00, // O
        ];
        assert_eq!(greeting(CLIENT_TX_IV, CLIENT_RX_IV), want);
    }

    #[test]
    fn the_length_prefix_excludes_itself() {
        let g = greeting(CLIENT_TX_IV, CLIENT_RX_IV);
        let declared = u16::from_le_bytes([g[0], g[1]]) as usize;
        assert_eq!(declared, g.len() - 2);
    }

    /// The channel greeting is the login one with the two `conn+0x48`-gated blocks cut.
    ///
    /// That is **26** bytes: `A..F` is 14 - `A` u16, `B` a length-prefixed empty string so
    /// two bytes of prefix and none of body, `C` and `D` u32, `E` and `F` u8 - and the
    /// version block is 12. Getting this wrong by the two bytes of `B`'s prefix is exactly
    /// the kind of slip that would shift every field after it, which is what this asserts.
    #[test]
    fn the_channel_greeting_drops_exactly_the_two_gated_blocks() {
        let login = greeting(CLIENT_TX_IV, CLIENT_RX_IV);
        let channel = channel_greeting(CLIENT_TX_IV, CLIENT_RX_IV);
        assert_eq!(channel.len(), login.len() - 26, "expected 26 bytes fewer");
        // And the surviving fields still add up: G 2, H 4, I 2, J 4, K 4, L 1, M/N/O 3.
        assert_eq!(channel.len(), 2 + 20);
    }

    /// The whole point: `G` must be the first field a channel reads, and it must be 1.
    /// When this was 0 - which is what the login greeting's `A` field looks like from here -
    /// the client raised line 840 and showed "The client is outdated".
    #[test]
    fn the_channel_greeting_starts_at_g_with_the_value_the_gate_wants() {
        let g = channel_greeting(CLIENT_TX_IV, CLIENT_RX_IV);
        let body = &g[2..];
        assert_eq!(u16::from_le_bytes([body[0], body[1]]), 1, "G");
        assert_eq!(body[0] & 0x80, 0, "G's 0x8000 bit must be clear");
        assert_eq!(u32::from_le_bytes([body[2], body[3], body[4], body[5]]), 1, "H");
    }

    /// The IVs are what the cipher chains are seeded from, so a misplaced one breaks the
    /// transport silently rather than loudly.
    #[test]
    fn the_channel_greeting_carries_the_ivs_where_the_client_reads_them() {
        let g = channel_greeting(0xAABB_CCDD, 0x1122_3344);
        let body = &g[2..];
        // G(2) + H(4) + I(2, empty) = 8 bytes before J.
        assert_eq!(&body[8..12], &0xAABB_CCDDu32.to_le_bytes(), "J");
        assert_eq!(&body[12..16], &0x1122_3344u32.to_le_bytes(), "K");
        assert_eq!(body[16], 1, "L");
    }

    /// The length prefix must describe the body, or the client's read loop never completes.
    #[test]
    fn the_channel_greeting_length_prefix_matches_its_body() {
        let g = channel_greeting(CLIENT_TX_IV, CLIENT_RX_IV);
        assert_eq!(u16::from_le_bytes([g[0], g[1]]) as usize, g.len() - 2);
    }
}
