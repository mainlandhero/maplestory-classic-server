//! Wire framing and pluggable ciphers.
//!
//! The scheme is no longer a guess: it was read out of the client's own receive path and
//! is documented in `docs/transport.md`. [`MapleCipher`] implements it, and
//! `reproduces_the_captured_client_headers` checks it against a real capture.
//!
//! * [`MapleCipher`] — 4-byte header (`len = a ^ b`), then AES-256-OFB over the body with
//!   the IV repeated 4x, chunked `0x5B0` then `0x5B4`, IV rolled once per packet. The
//!   header constant differs per [`Direction`].
//! * [`PlainCipher`] — no encryption, for tests where a cipher only obscures failures.
//!
//! There is **no shanda** in this client's packet path. Note [`AES_KEY`]: the key in the
//! binary is a decoy the client patches at startup, so it must not be "corrected" back.

use crate::error::{NetError, Result};

/// Bytes in the length header that precedes every packet body.
pub const HEADER_LEN: usize = 4;

/// A symmetric, stateful stream cipher over packet bodies.
///
/// Implementations are stateful: MapleStory-style ciphers roll their IV after every
/// packet, so send and receive directions each need their own instance.
pub trait Cipher: Send {
    /// Encrypt a body in place and produce its 4-byte header.
    fn encrypt(&mut self, body: &mut [u8]) -> [u8; HEADER_LEN];

    /// Body length encoded in a header, or an error if the header is not ours.
    fn decode_len(&self, header: [u8; HEADER_LEN]) -> Result<usize>;

    /// Decrypt a body in place.
    fn decrypt(&mut self, body: &mut [u8]) -> Result<()>;
}

/// No encryption: the header is a plain little-endian `u32` length.
///
/// Useful for bring-up and for tests, where encryption only obscures failures.
#[derive(Debug, Default, Clone)]
pub struct PlainCipher;

impl Cipher for PlainCipher {
    fn encrypt(&mut self, body: &mut [u8]) -> [u8; HEADER_LEN] {
        (body.len() as u32).to_le_bytes()
    }

    fn decode_len(&self, header: [u8; HEADER_LEN]) -> Result<usize> {
        Ok(u32::from_le_bytes(header) as usize)
    }

    fn decrypt(&mut self, _body: &mut [u8]) -> Result<()> {
        Ok(())
    }
}

/// The "shanda" byte shuffle that classic clients apply on top of AES.
///
/// **Not part of this client's packet path**, and now proven so: with the real AES key
/// the captured stream decodes to clean packets with no shanda step anywhere. Kept only
/// as reference for other MapleStory-derived clients.
pub mod shanda {
    fn rol(v: u8, n: u32) -> u8 {
        v.rotate_left(n % 8)
    }

    fn ror(v: u8, n: u32) -> u8 {
        v.rotate_right(n % 8)
    }

    // `j` is both the index and an operand in the arithmetic below, so indexing mirrors
    // the reference algorithm; an iterator form would obscure it.
    #[allow(clippy::needless_range_loop)]
    pub fn encrypt(data: &mut [u8]) {
        let len = data.len();
        for _ in 0..3 {
            let mut carry = 0u8;
            for j in (0..len).rev() {
                let mut b = rol(data[j], 3);
                b = b.wrapping_add(j as u8);
                b ^= carry;
                carry = b;
                b = ror(b, (len - j) as u32 & 0xFF);
                b ^= 0xFF;
                b = b.wrapping_add(0x48);
                data[j] = b;
            }
            let mut carry = 0u8;
            for j in 0..len {
                let mut b = rol(data[j], 4);
                b = b.wrapping_add(j as u8);
                b ^= carry;
                carry = b;
                b ^= 0x13;
                b = ror(b, 3);
                data[j] = b;
            }
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn decrypt(data: &mut [u8]) {
        let len = data.len();
        for _ in 0..3 {
            let mut next_carry;
            let mut carry = 0u8;
            for j in 0..len {
                let mut b = rol(data[j], 3);
                b ^= 0x13;
                next_carry = b;
                b ^= carry;
                b = b.wrapping_sub(j as u8);
                data[j] = ror(b, 4);
                carry = next_carry;
            }
            let mut carry = 0u8;
            for j in (0..len).rev() {
                let mut b = data[j].wrapping_sub(0x48);
                b ^= 0xFF;
                b = rol(b, (len - j) as u32 & 0xFF);
                next_carry = b;
                b ^= carry;
                b = b.wrapping_sub(j as u8);
                data[j] = ror(b, 3);
                carry = next_carry;
            }
        }
    }
}

/// This client's wire cipher: AES-256-OFB over the body, IV rolled once per packet.
///
/// Verified against a real capture — see `reproduces_the_captured_client_headers`.
pub struct MapleCipher {
    iv: [u8; 4],
    /// XORed into the header's `a` field. The client uses a *different* value in each
    /// direction, so this is set from [`Direction`] rather than from a version number.
    header_const: u16,
    aes: Box<dyn AesEcb>,
}

/// Which way a stream flows. The client checks the two directions against different
/// constants, so a cipher instance is only valid for one of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Packets we send. `FUN_1406e97e0` computes `(iv >> 16) ^ a` and both receive
    /// loops drop the connection unless the result is `0xFFFE`.
    ServerToClient,
    /// Packets the client sends. Measured across all 16 headers of
    /// `research/fixtures/capture-handshake-ok.log`.
    ClientToServer,
}

impl Direction {
    pub const fn header_const(self) -> u16 {
        match self {
            Direction::ServerToClient => 0xFFFE,
            Direction::ClientToServer => 0x00DF,
        }
    }
}

/// Minimal AES-256 ECB block operation, so the cipher can be tested with a stub.
pub trait AesEcb: Send {
    fn encrypt_block(&self, block: &mut [u8; 16]);
}

struct RealAes {
    key: aes::Aes256,
}

impl AesEcb for RealAes {
    fn encrypt_block(&self, block: &mut [u8; 16]) {
        use aes::cipher::BlockEncrypt;
        let b = aes::cipher::generic_array::GenericArray::from_mut_slice(block);
        self.key.encrypt_block(b);
    }
}

/// This client's real AES-256 key, read out of the **running** process.
///
/// The key table at `0x143A86810` on disk holds the stock MapleStory key
/// (`13 08 06 B4 1B 0F 33 52`) and is a **decoy**: the client overwrites the low byte of
/// all 32 dwords at startup. Only that table is patched — the IV shuffle table beside it
/// is untouched, which is precisely why framing, the header constant and the IV chain all
/// worked while everything AES-shaped failed in both directions.
///
/// Recovered with `tools/dump_runtime.py` and verified against eight captures from
/// different sessions; see `reproduces_the_captured_client_headers` and
/// `decrypts_the_captured_version_packet`.
pub const AES_KEY: [u8; 32] = [
    0x0F, 0x00, 0x00, 0x00, 0x1B, 0x00, 0x00, 0x00, 0xC5, 0x00, 0x00, 0x00, 0x46, 0x00, 0x00, 0x00,
    0xF3, 0x00, 0x00, 0x00, 0xBE, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x75, 0x00, 0x00, 0x00,
];

/// The stock key the binary carries on disk. Kept only so the decoy is documented and
/// nobody "fixes" [`AES_KEY`] back to it.
pub const DECOY_AES_KEY: [u8; 32] = [
    0x13, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0xB4, 0x00, 0x00, 0x00,
    0x1B, 0x00, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x00, 0x33, 0x00, 0x00, 0x00, 0x52, 0x00, 0x00, 0x00,
];

const SHUFFLE: [u8; 256] = [
        0xEC, 0x3F, 0x77, 0xA4, 0x45, 0xD0, 0x71, 0xBF, 0xB7, 0x98, 0x20, 0xFC,
        0x4B, 0xE9, 0xB3, 0xE1, 0x5C, 0x22, 0xF7, 0x0C, 0x44, 0x1B, 0x81, 0xBD,
        0x63, 0x8D, 0xD4, 0xC3, 0xF2, 0x10, 0x19, 0xE0, 0xFB, 0xA1, 0x6E, 0x66,
        0xEA, 0xAE, 0xD6, 0xCE, 0x06, 0x18, 0x4E, 0xEB, 0x78, 0x95, 0xDB, 0xBA,
        0xB6, 0x42, 0x7A, 0x2A, 0x83, 0x0B, 0x54, 0x67, 0x6D, 0xE8, 0x65, 0xE7,
        0x2F, 0x07, 0xF3, 0xAA, 0x27, 0x7B, 0x85, 0xB0, 0x26, 0xFD, 0x8B, 0xA9,
        0xFA, 0xBE, 0xA8, 0xD7, 0xCB, 0xCC, 0x92, 0xDA, 0xF9, 0x93, 0x60, 0x2D,
        0xDD, 0xD2, 0xA2, 0x9B, 0x39, 0x5F, 0x82, 0x21, 0x4C, 0x69, 0xF8, 0x31,
        0x87, 0xEE, 0x8E, 0xAD, 0x8C, 0x6A, 0xBC, 0xB5, 0x6B, 0x59, 0x13, 0xF1,
        0x04, 0x00, 0xF6, 0x5A, 0x35, 0x79, 0x48, 0x8F, 0x15, 0xCD, 0x97, 0x57,
        0x12, 0x3E, 0x37, 0xFF, 0x9D, 0x4F, 0x51, 0xF5, 0xA3, 0x70, 0xBB, 0x14,
        0x75, 0xC2, 0xB8, 0x72, 0xC0, 0xED, 0x7D, 0x68, 0xC9, 0x2E, 0x0D, 0x62,
        0x46, 0x17, 0x11, 0x4D, 0x6C, 0xC4, 0x7E, 0x53, 0xC1, 0x25, 0xC7, 0x9A,
        0x1C, 0x88, 0x58, 0x2C, 0x89, 0xDC, 0x02, 0x64, 0x40, 0x01, 0x5D, 0x38,
        0xA5, 0xE2, 0xAF, 0x55, 0xD5, 0xEF, 0x1A, 0x7C, 0xA7, 0x5B, 0xA6, 0x6F,
        0x86, 0x9F, 0x73, 0xE6, 0x0A, 0xDE, 0x2B, 0x99, 0x4A, 0x47, 0x9C, 0xDF,
        0x09, 0x76, 0x9E, 0x30, 0x0E, 0xE4, 0xB2, 0x94, 0xA0, 0x3B, 0x34, 0x1D,
        0x28, 0x0F, 0x36, 0xE3, 0x23, 0xB4, 0x03, 0xD8, 0x90, 0xC8, 0x3C, 0xFE,
        0x5E, 0x32, 0x24, 0x50, 0x1F, 0x3A, 0x43, 0x8A, 0x96, 0x41, 0x74, 0xAC,
        0x52, 0x33, 0xF0, 0xD9, 0x29, 0x80, 0xB1, 0x16, 0xD3, 0xAB, 0x91, 0xB9,
        0x84, 0x7F, 0x61, 0x1E, 0xCF, 0xC5, 0xD1, 0x56, 0x3D, 0xCA, 0xF4, 0x05,
        0xC6, 0xE5, 0x08, 0x49,
        ];

/// Roll a 4-byte IV forward one packet, with the stock MapleStory shuffle table.
///
/// Shared by every cipher on this transport. `FUN_1406e9a65` picks the *body* transform
/// from the connection type (`conn+0x48`) but the header and the IV chain are the same code
/// either way, so a byte-shift cipher and an AES one roll the IV
/// identically. Keeping one implementation is what makes that true rather than hopeful.
pub fn roll_iv(iv: [u8; 4]) -> [u8; 4] {
    let mut out: [u8; 4] = [0xF2, 0x53, 0x50, 0xC6];
    for &b in iv.iter() {
        let t = SHUFFLE[b as usize];
        out[0] = out[0].wrapping_add(SHUFFLE[out[1] as usize].wrapping_sub(b));
        out[1] = out[1].wrapping_sub(out[2] ^ t);
        out[2] ^= SHUFFLE[out[3] as usize].wrapping_add(b);
        // Note this consumes the *updated* out[0], not the value it had on entry.
        out[3] = out[3].wrapping_sub(out[0].wrapping_sub(t));

        let merged = u32::from_le_bytes(out).rotate_left(3);
        out = merged.to_le_bytes();
    }
    out
}

/// The 4-byte header for a body of `len`, given an IV and a direction constant.
///
/// **The header is never ciphered**, in either mode: the client does `buf + 4` before
/// handing the buffer to the body transform (`FUN_1406e9a65`). That is why a channel
/// connection can be framed correctly before its body cipher is settled.
pub fn header_from_iv(iv: [u8; 4], header_const: u16, len: u16) -> [u8; HEADER_LEN] {
    let a = u16::from_le_bytes([iv[2], iv[3]]) ^ header_const;
    let b = a ^ len;
    let mut header = [0u8; HEADER_LEN];
    header[..2].copy_from_slice(&a.to_le_bytes());
    header[2..].copy_from_slice(&b.to_le_bytes());
    header
}

/// Body length encoded in a header, or an error if it is not ours.
fn len_from_header(header: [u8; HEADER_LEN]) -> Result<usize> {
    let a = u16::from_le_bytes([header[0], header[1]]);
    let b = u16::from_le_bytes([header[2], header[3]]);
    let len = a ^ b;
    // Sanity-check against a wildly wrong cipher/IV rather than allocating garbage.
    if len == 0 || len as usize > MAX_PACKET_LEN {
        return Err(NetError::BadHeader { header, decoded_len: len as usize });
    }
    Ok(len as usize)
}

impl MapleCipher {
    pub fn new(iv: [u8; 4], dir: Direction) -> Self {
        use aes::cipher::KeyInit;
        let key = aes::Aes256::new_from_slice(&AES_KEY).expect("32-byte key");
        Self { iv, header_const: dir.header_const(), aes: Box::new(RealAes { key }) }
    }

    /// Inject a different AES implementation (tests use a stub).
    pub fn with_aes(iv: [u8; 4], dir: Direction, aes: Box<dyn AesEcb>) -> Self {
        Self { iv, header_const: dir.header_const(), aes }
    }

    /// The header the client will accept for a body of `len`, without rolling the IV.
    ///
    /// Used for framing-only probes, where a header is sent and the body withheld.
    pub fn peek_header(&self, len: u16) -> [u8; HEADER_LEN] {
        header_from_iv(self.iv, self.header_const, len)
    }

    pub fn iv(&self) -> [u8; 4] {
        self.iv
    }

    /// AES over the body in 1460-byte chunks, IV repeated to fill each block.
    fn transform(&self, data: &mut [u8]) {
        let mut iv16 = [0u8; 16];
        for (i, slot) in iv16.iter_mut().enumerate() {
            *slot = self.iv[i % 4];
        }

        let mut pos = 0usize;
        // The first chunk is shortened by the 4-byte header, matching the client.
        let mut chunk = 1456usize;
        while pos < data.len() {
            let end = (pos + chunk).min(data.len());
            let mut block = iv16;
            for i in pos..end {
                if (i - pos).is_multiple_of(16) {
                    self.aes.encrypt_block(&mut block);
                }
                data[i] ^= block[(i - pos) % 16];
            }
            pos = end;
            chunk = 1460;
        }
    }

    /// Roll the IV forward. Every packet changes it, which is why send and receive
    /// need separate cipher instances.
    fn next_iv(&mut self) {
        self.iv = roll_iv(self.iv);
    }
}

impl Cipher for MapleCipher {
    fn encrypt(&mut self, body: &mut [u8]) -> [u8; HEADER_LEN] {
        // No shanda: the client's receive path runs FUN_1406e99e0 (AES only) and hands
        // the buffer straight to the dispatcher. See docs/transport.md.
        let header = self.peek_header(body.len() as u16);
        self.transform(body);
        self.next_iv();
        header
    }

    fn decode_len(&self, header: [u8; HEADER_LEN]) -> Result<usize> {
        len_from_header(header)
    }

    fn decrypt(&mut self, body: &mut [u8]) -> Result<()> {
        self.transform(body);
        self.next_iv();
        Ok(())
    }
}

/// Upper bound on a single packet body; guards against a bad header causing a huge
/// allocation.
pub const MAX_PACKET_LEN: usize = 16 * 1024 * 1024;

/// Which way one [`ByteShiftCipher`] instance moves a body.
///
/// The client's routine (`FUN_1406ef9f0`) **subtracts**; whichever side subtracts, the
/// other must add.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shift {
    Add,
    Sub,
}

impl Shift {
    /// The operation that undoes this one.
    pub const fn inverse(self) -> Shift {
        match self {
            Shift::Add => Shift::Sub,
            Shift::Sub => Shift::Add,
        }
    }
}

/// Apply the byte shift in place, with `iv[0]` as the amount.
///
/// Read straight off `FUN_1406ef9f0`: one wrapping 8-bit operation across the whole body,
/// with the amount taken from the **first byte** of the 4-byte IV and held constant for the
/// packet. The client chunks at `0x5B0`/`0x5B4` around the call, but the amount does not
/// vary within a packet, so a flat loop gives the identical result.
pub fn shift_body(body: &mut [u8], iv: [u8; 4], shift: Shift) {
    let amount = iv[0];
    for b in body.iter_mut() {
        *b = match shift {
            Shift::Add => b.wrapping_add(amount),
            Shift::Sub => b.wrapping_sub(amount),
        };
    }
}

/// The game channel's **server -> client** cipher: `FUN_1406ef9f0`, a wrapping byte shift.
///
/// **A channel is asymmetric, and each half was measured separately.**
///
/// | direction | transform |
/// |---|---|
/// | client -> server | AES-256-OFB, the same as login ([`MapleCipher`]) |
/// | **server -> client** | **this**, `out[i] = in[i] - iv[0]` on the client's side |
///
/// So a server **adds** `iv[0]` on send and the client's subtract recovers the plaintext.
///
/// This took three passes to get right, and the middle one was committed as fact:
///
/// 1. Read statically from `conn+0x48`, which selects the transform. Correct.
/// 2. **Retracted** when the first real channel packets decoded under AES - but those were
///    packets the client *sent*, which is the other half of the connection. The retraction
///    generalised one direction to both.
/// 3. Restored 2026-08-19 by arithmetic on a live run: a `SetField` sent under AES was
///    dispatched by the client as opcode `0x406C`, and `0x406C` is exactly that ciphertext
///    with `iv[0] = 0x02` subtracted from each byte.
///
/// The lesson worth keeping: **"the channel is AES" was never one claim.** It was two, and
/// only one of them had been tested.
///
/// See `docs/transport.md`.
pub struct ByteShiftCipher {
    iv: [u8; 4],
    header_const: u16,
    /// What this instance does to a body. One instance per direction, as with `MapleCipher`.
    shift: Shift,
}

impl ByteShiftCipher {
    pub fn new(iv: [u8; 4], dir: Direction, shift: Shift) -> Self {
        Self { iv, header_const: dir.header_const(), shift }
    }

    /// The IV this instance will use for the **next** packet.
    ///
    /// Exposed so a caller can compute the other polarity's reading of the same body before
    /// the IV rolls - which is how the unresolved direction gets settled from one run.
    pub fn iv(&self) -> [u8; 4] {
        self.iv
    }

    pub fn shift(&self) -> Shift {
        self.shift
    }
}

impl Cipher for ByteShiftCipher {
    fn encrypt(&mut self, body: &mut [u8]) -> [u8; HEADER_LEN] {
        let header = header_from_iv(self.iv, self.header_const, body.len() as u16);
        shift_body(body, self.iv, self.shift);
        self.iv = roll_iv(self.iv);
        header
    }

    fn decode_len(&self, header: [u8; HEADER_LEN]) -> Result<usize> {
        len_from_header(header)
    }

    fn decrypt(&mut self, body: &mut [u8]) -> Result<()> {
        shift_body(body, self.iv, self.shift);
        self.iv = roll_iv(self.iv);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The channel cipher must round-trip against itself with opposite shifts, which is
    /// what a server and a client are.
    #[test]
    fn the_byte_shift_round_trips_between_opposite_shifts() {
        for iv in [[1u8, 2, 3, 4], [0xFF, 0, 0, 0], [0x80, 0x11, 0x22, 0x33]] {
            let mut tx = ByteShiftCipher::new(iv, Direction::ServerToClient, Shift::Add);
            let mut rx = ByteShiftCipher::new(iv, Direction::ServerToClient, Shift::Sub);
            let plain: Vec<u8> = (0..=255u8).collect();
            let mut body = plain.clone();
            tx.encrypt(&mut body);
            assert_ne!(body, plain, "iv {iv:?} produced no change at all");
            rx.decrypt(&mut body).unwrap();
            assert_eq!(body, plain, "iv {iv:?} did not round-trip");
        }
    }

    /// An IV whose first byte is zero shifts by nothing. That is not a bug - it is what
    /// `FUN_1406ef9f0` does - but it means a packet that draws such an IV proves nothing
    /// about polarity, and a test written around one would pass either way.
    #[test]
    fn a_zero_first_iv_byte_is_the_identity() {
        let mut c =
            ByteShiftCipher::new([0, 0xAA, 0xBB, 0xCC], Direction::ClientToServer, Shift::Sub);
        let mut body = vec![1u8, 2, 3];
        c.encrypt(&mut body);
        assert_eq!(body, vec![1, 2, 3]);
    }

    /// Both ciphers roll the IV through the same code, so a channel and a login connection
    /// stay in step packet for packet. If these diverge the transport rots silently.
    #[test]
    fn both_ciphers_roll_the_iv_identically() {
        let iv = [0x01, 0x78, 0x30, 0x52];
        let mut aes = MapleCipher::new(iv, Direction::ServerToClient);
        let mut shift = ByteShiftCipher::new(iv, Direction::ServerToClient, Shift::Add);
        for _ in 0..8 {
            let mut a = vec![0u8; 24];
            let mut b = vec![0u8; 24];
            assert_eq!(aes.encrypt(&mut a), shift.encrypt(&mut b), "headers diverged");
        }
    }

    /// The header does not depend on the body transform: it is computed before the shift,
    /// and the client never ciphers it. This is what lets a channel be framed correctly
    /// while the polarity is still unsettled.
    #[test]
    fn the_header_is_the_same_whichever_way_the_body_shifts() {
        let iv = [0x11, 0x22, 0x33, 0x44];
        let mut add = ByteShiftCipher::new(iv, Direction::ServerToClient, Shift::Add);
        let mut sub = ByteShiftCipher::new(iv, Direction::ServerToClient, Shift::Sub);
        let mut a = vec![9u8; 40];
        let mut b = vec![9u8; 40];
        assert_eq!(add.encrypt(&mut a), sub.encrypt(&mut b));
        assert_ne!(a, b, "the bodies should differ even though the headers match");
    }


    #[test]
    fn shanda_round_trips() {
        let original: Vec<u8> = (0u8..=255).collect();
        let mut data = original.clone();
        shanda::encrypt(&mut data);
        assert_ne!(data, original, "encryption should change the data");
        shanda::decrypt(&mut data);
        assert_eq!(data, original);
    }

    #[test]
    fn shanda_round_trips_odd_lengths() {
        for len in [1usize, 2, 15, 17, 63, 1459, 1460, 1461] {
            let original: Vec<u8> = (0..len).map(|i| (i * 7 % 251) as u8).collect();
            let mut data = original.clone();
            shanda::encrypt(&mut data);
            shanda::decrypt(&mut data);
            assert_eq!(data, original, "failed at len {len}");
        }
    }

    /// Identity "AES" so the framing can be tested independently of the block cipher.
    struct NoopAes;
    impl AesEcb for NoopAes {
        fn encrypt_block(&self, _b: &mut [u8; 16]) {}
    }

    /// The headers the real client actually put on the wire, from
    /// `research/fixtures/capture-handshake-ok.log`, with the payload length of each.
    ///
    /// This is the load-bearing test for the whole transport: reproducing all sixteen
    /// byte-exactly pins the header constant, the IV seed, and the shuffle table at once.
    /// Any of the three being wrong breaks it, usually from the second packet on.
    const CAPTURED: [([u8; 4], u16); 16] = [
        ([0xEF, 0x52, 0xC0, 0x52], 47),
        ([0x6B, 0xFB, 0x58, 0xFB], 51),
        ([0x37, 0xDD, 0x3F, 0xDD], 8),
        ([0x5B, 0xE3, 0x51, 0xE3], 10),
        ([0x9C, 0x9A, 0x9A, 0x9A], 6),
        ([0xAA, 0x99, 0xAC, 0x99], 6),
        ([0x18, 0x3D, 0x1E, 0x3D], 6),
        ([0x76, 0x23, 0x70, 0x23], 6),
        ([0x33, 0x84, 0x35, 0x84], 6),
        ([0x00, 0x8D, 0x06, 0x8D], 6),
        ([0x14, 0x4C, 0x12, 0x4C], 6),
        ([0x9F, 0xC4, 0x99, 0xC4], 6),
        ([0x9C, 0xFA, 0x9A, 0xFA], 6),
        ([0x27, 0x71, 0x21, 0x71], 6),
        ([0xEE, 0x23, 0xE8, 0x23], 6),
        ([0x76, 0xE8, 0x46, 0xE8], 48),
    ];

    /// The client's first packet, captured verbatim off the wire, with the payload
    /// still encrypted. Decrypting it must yield exactly what FUN_1415d5b40 builds:
    /// opcode 0x70, then `u8 2`, then `u32 100`. This is what pins the AES key - the
    /// on-disk key is a decoy and produces noise here.
    const CAPTURED_PKT0: [u8; 47] = [
        0x87, 0xBB, 0x8B, 0x51, 0xE7, 0xF2, 0xE7, 0x78, 0xF9, 0x49, 0xE5, 0xD5, 0xEB, 0x46, 0x2B,
        0x60, 0x00, 0x58, 0x0E, 0xF8, 0x02, 0xB2, 0xA5, 0x06, 0x1B, 0x37, 0x4F, 0x99, 0x4E, 0x2F,
        0x8F, 0xD9, 0x75, 0xFB, 0x8B, 0x7B, 0xE3, 0x54, 0x74, 0xD9, 0x1C, 0xDD, 0x76, 0xBD, 0xE5,
        0x67, 0x19,
    ];

    #[test]
    fn decrypts_the_captured_version_packet() {
        let mut c = MapleCipher::new(0x5230_7801u32.to_le_bytes(), Direction::ClientToServer);
        let mut body = CAPTURED_PKT0.to_vec();
        c.decrypt(&mut body).unwrap();

        assert_eq!(u16::from_le_bytes([body[0], body[1]]), 0x0070, "opcode");
        assert_eq!(body[2], 2, "the u8 FUN_1415d5b40 writes");
        assert_eq!(
            u32::from_le_bytes([body[3], body[4], body[5], body[6]]),
            100,
            "the u32 FUN_1415d5b40 writes"
        );
    }

    #[test]
    fn the_disk_key_is_a_decoy_and_does_not_decrypt() {
        assert_ne!(AES_KEY, DECOY_AES_KEY);
        use aes::cipher::KeyInit;
        let key = aes::Aes256::new_from_slice(&DECOY_AES_KEY).unwrap();
        let mut c = MapleCipher::with_aes(
            0x5230_7801u32.to_le_bytes(),
            Direction::ClientToServer,
            Box::new(RealAes { key }),
        );
        let mut body = CAPTURED_PKT0.to_vec();
        c.decrypt(&mut body).unwrap();
        assert_ne!(u16::from_le_bytes([body[0], body[1]]), 0x0070);
    }

    #[test]
    fn reproduces_the_captured_client_headers() {
        // The J field we sent in the greeting; the client sends on this chain.
        let mut c = MapleCipher::new(0x5230_7801u32.to_le_bytes(), Direction::ClientToServer);
        for (n, (expected, len)) in CAPTURED.iter().enumerate() {
            let got = c.peek_header(*len);
            assert_eq!(
                got, *expected,
                "packet {n}: got {got:02X?}, client sent {expected:02X?}"
            );
            assert_eq!(c.decode_len(got).unwrap(), *len as usize);
            c.next_iv();
        }
    }

    #[test]
    fn server_headers_satisfy_the_clients_check() {
        // FUN_1406e97e0 returns (iv >> 16) ^ a and the caller requires 0xFFFE.
        let mut c = MapleCipher::new(0x5230_7802u32.to_le_bytes(), Direction::ServerToClient);
        for len in [2u16, 6, 47, 1000] {
            let h = c.peek_header(len);
            let a = u16::from_le_bytes([h[0], h[1]]);
            let iv_hi = u16::from_le_bytes([c.iv()[2], c.iv()[3]]);
            assert_eq!(iv_hi ^ a, 0xFFFE, "client would drop us at len {len}");
            assert_eq!(c.decode_len(h).unwrap(), len as usize);
            c.next_iv();
        }
    }

    #[test]
    fn maple_cipher_round_trips_across_packets() {
        let iv = [0x12, 0x34, 0x56, 0x78];
        let mut send = MapleCipher::with_aes(iv, Direction::ServerToClient, Box::new(NoopAes));
        let mut recv = MapleCipher::with_aes(iv, Direction::ServerToClient, Box::new(NoopAes));

        // Several packets in a row: the IV rolls, so both sides must stay in step.
        for n in 1usize..6 {
            let original: Vec<u8> = (0..n * 37).map(|i| (i % 253) as u8).collect();
            let mut body = original.clone();

            let header = send.encrypt(&mut body);
            assert_eq!(recv.decode_len(header).unwrap(), original.len());
            recv.decrypt(&mut body).unwrap();
            assert_eq!(body, original, "mismatch on packet {n}");
        }
    }

    #[test]
    fn plain_cipher_round_trips() {
        let mut c = PlainCipher;
        let original = b"hello world".to_vec();
        let mut body = original.clone();
        let header = c.encrypt(&mut body);
        assert_eq!(c.decode_len(header).unwrap(), original.len());
        c.decrypt(&mut body).unwrap();
        assert_eq!(body, original);
    }

    #[test]
    fn absurd_header_is_rejected() {
        let c = MapleCipher::new([1, 2, 3, 4], Direction::ServerToClient);
        // a ^ b decodes to 0, which is never a valid body length.
        assert!(c.decode_len([0xAA, 0xBB, 0xAA, 0xBB]).is_err());
    }
}
