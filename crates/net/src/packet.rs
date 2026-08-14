//! Packet body primitives — the `COutPacket` / `CInPacket` pair the client uses.
//!
//! Everything is little-endian. Strings are length-prefixed with a `u16` count of
//! bytes; the client's "ASCII" strings are really single-byte characters, so they are
//! mapped 1:1 rather than validated as UTF-8 (game data legitimately contains bytes
//! that are not valid UTF-8).

use crate::error::{NetError, Result};

/// Builds an outgoing packet body. Mirrors `COutPacket`.
#[derive(Debug, Clone, Default)]
pub struct PacketWriter {
    buf: Vec<u8>,
}

impl PacketWriter {
    pub fn new() -> Self {
        Self { buf: Vec::with_capacity(64) }
    }

    /// Start a packet with a 2-byte opcode.
    pub fn with_opcode(opcode: u16) -> Self {
        let mut w = Self::new();
        w.u16(opcode);
        w
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.buf
    }

    pub fn u8(&mut self, v: u8) -> &mut Self {
        self.buf.push(v);
        self
    }

    pub fn i8(&mut self, v: i8) -> &mut Self {
        self.u8(v as u8)
    }

    /// Maple encodes booleans as a single byte.
    pub fn bool(&mut self, v: bool) -> &mut Self {
        self.u8(u8::from(v))
    }

    pub fn u16(&mut self, v: u16) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn i16(&mut self, v: i16) -> &mut Self {
        self.u16(v as u16)
    }

    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn i32(&mut self, v: i32) -> &mut Self {
        self.u32(v as u32)
    }

    pub fn u64(&mut self, v: u64) -> &mut Self {
        self.buf.extend_from_slice(&v.to_le_bytes());
        self
    }

    pub fn i64(&mut self, v: i64) -> &mut Self {
        self.u64(v as u64)
    }

    pub fn bytes(&mut self, v: &[u8]) -> &mut Self {
        self.buf.extend_from_slice(v);
        self
    }

    /// Zero padding.
    pub fn zeros(&mut self, n: usize) -> &mut Self {
        self.buf.resize(self.buf.len() + n, 0);
        self
    }

    /// `u16` byte-length followed by the raw bytes.
    pub fn str(&mut self, s: &str) -> &mut Self {
        let bytes: Vec<u8> = s.chars().map(|c| c as u8).collect();
        self.u16(bytes.len() as u16);
        self.bytes(&bytes)
    }

    /// String padded (or truncated) to exactly `n` bytes, as used by fixed-width
    /// fields such as character names.
    pub fn fixed_str(&mut self, s: &str, n: usize) -> &mut Self {
        let mut bytes: Vec<u8> = s.chars().map(|c| c as u8).take(n).collect();
        bytes.resize(n, 0);
        self.bytes(&bytes)
    }

    /// A 2-D point as a pair of `i16`.
    pub fn pos(&mut self, x: i16, y: i16) -> &mut Self {
        self.i16(x).i16(y)
    }
}

/// Reads an incoming packet body. Mirrors `CInPacket`.
#[derive(Debug, Clone)]
pub struct PacketReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> PacketReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    pub fn rest(&self) -> &'a [u8] {
        &self.data[self.pos..]
    }

    pub fn seek(&mut self, pos: usize) {
        self.pos = pos.min(self.data.len());
    }

    pub fn skip(&mut self, n: usize) -> Result<()> {
        self.take(n).map(|_| ())
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.remaining() < n {
            return Err(NetError::PacketUnderflow {
                offset: self.pos,
                need: n,
                have: self.remaining(),
            });
        }
        let s = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    pub fn i8(&mut self) -> Result<i8> {
        Ok(self.u8()? as i8)
    }

    pub fn bool(&mut self) -> Result<bool> {
        Ok(self.u8()? != 0)
    }

    pub fn u16(&mut self) -> Result<u16> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    pub fn i16(&mut self) -> Result<i16> {
        Ok(self.u16()? as i16)
    }

    pub fn u32(&mut self) -> Result<u32> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    pub fn i32(&mut self) -> Result<i32> {
        Ok(self.u32()? as i32)
    }

    pub fn u64(&mut self) -> Result<u64> {
        let b = self.take(8)?;
        Ok(u64::from_le_bytes([
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
        ]))
    }

    pub fn i64(&mut self) -> Result<i64> {
        Ok(self.u64()? as i64)
    }

    pub fn bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        self.take(n)
    }

    /// `u16` byte-length followed by that many bytes.
    pub fn str(&mut self) -> Result<String> {
        let n = self.u16()? as usize;
        let b = self.take(n)?;
        Ok(b.iter().map(|&c| c as char).collect())
    }

    /// Fixed-width string; trailing NULs are trimmed.
    pub fn fixed_str(&mut self, n: usize) -> Result<String> {
        let b = self.take(n)?;
        let end = b.iter().position(|&c| c == 0).unwrap_or(b.len());
        Ok(b[..end].iter().map(|&c| c as char).collect())
    }

    pub fn pos(&mut self) -> Result<(i16, i16)> {
        Ok((self.i16()?, self.i16()?))
    }

    /// Read the leading opcode without consuming the reader's position permanently.
    pub fn peek_opcode(&self) -> Option<u16> {
        if self.data.len() < 2 {
            return None;
        }
        Some(u16::from_le_bytes([self.data[0], self.data[1]]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_primitives() {
        let mut w = PacketWriter::with_opcode(0x1234);
        w.u8(0xAB)
            .i16(-2)
            .u32(0xDEAD_BEEF)
            .i64(-5)
            .bool(true)
            .str("Scania")
            .fixed_str("Owl", 13)
            .pos(100, -200);

        let buf = w.into_vec();
        let mut r = PacketReader::new(&buf);
        assert_eq!(r.peek_opcode(), Some(0x1234));
        assert_eq!(r.u16().unwrap(), 0x1234);
        assert_eq!(r.u8().unwrap(), 0xAB);
        assert_eq!(r.i16().unwrap(), -2);
        assert_eq!(r.u32().unwrap(), 0xDEAD_BEEF);
        assert_eq!(r.i64().unwrap(), -5);
        assert!(r.bool().unwrap());
        assert_eq!(r.str().unwrap(), "Scania");
        assert_eq!(r.fixed_str(13).unwrap(), "Owl");
        assert_eq!(r.pos().unwrap(), (100, -200));
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn fixed_str_truncates_and_pads() {
        let mut w = PacketWriter::new();
        w.fixed_str("a-very-long-character-name", 13);
        assert_eq!(w.len(), 13);

        let mut w2 = PacketWriter::new();
        w2.fixed_str("ab", 13);
        assert_eq!(w2.as_slice(), b"ab\0\0\0\0\0\0\0\0\0\0\0");
    }

    #[test]
    fn underflow_is_an_error_not_a_panic() {
        let buf = [1u8, 2];
        let mut r = PacketReader::new(&buf);
        assert!(r.u16().is_ok());
        assert!(matches!(r.u8(), Err(NetError::PacketUnderflow { .. })));
    }

    #[test]
    fn high_bytes_survive_string_round_trip() {
        // Game strings are not UTF-8; byte values above 0x7F must not be mangled.
        let s: String = vec![0xC4u8 as char, 0xE9u8 as char].into_iter().collect();
        let mut w = PacketWriter::new();
        w.str(&s);
        let buf = w.into_vec();
        assert_eq!(&buf[2..], &[0xC4, 0xE9]);
        let mut r = PacketReader::new(&buf);
        assert_eq!(r.str().unwrap(), s);
    }
}
