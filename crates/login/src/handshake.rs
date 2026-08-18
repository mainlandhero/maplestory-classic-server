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
}
