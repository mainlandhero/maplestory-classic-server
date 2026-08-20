//! Cursor over a WZ archive: primitives, obfuscated strings, encrypted offsets.
//!
//! See `docs/wz-format.md`. The `mscw` client uses a **zero** string key, so decoding
//! is the positional XOR mask alone with no AES keystream.

use crate::error::{Result, WzError};

/// Subtracted during offset decryption. Constant across all known WZ versions.
pub const OFFSET_MAGIC: u32 = 0x581C_3F6D;

const ASCII_MASK: u8 = 0xAA;
const UNICODE_MASK: u16 = 0xAAAA;

pub struct WzReader<'a> {
    pub data: &'a [u8],
    pub pos: usize,
    /// Start of the content section; offsets are relative to this.
    pub fstart: u32,
    /// Version hash used to decrypt entry offsets.
    pub version_hash: u32,
}

impl<'a> WzReader<'a> {
    pub fn new(data: &'a [u8], fstart: u32, version_hash: u32) -> Self {
        Self { data, pos: 0, fstart, version_hash }
    }

    pub fn seek(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    fn need(&self, n: usize) -> Result<()> {
        if self.remaining() < n {
            return Err(WzError::UnexpectedEof {
                offset: self.pos,
                need: n,
                have: self.remaining(),
            });
        }
        Ok(())
    }

    pub fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        self.need(n)?;
        let s = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    pub fn u8(&mut self) -> Result<u8> {
        self.need(1)?;
        let v = self.data[self.pos];
        self.pos += 1;
        Ok(v)
    }

    pub fn i8(&mut self) -> Result<i8> {
        Ok(self.u8()? as i8)
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

    pub fn f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.u32()?))
    }

    pub fn f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(self.u64()?))
    }

    /// Compressed int: one signed byte, or `-128` escaping to a full i32.
    pub fn compressed_i32(&mut self) -> Result<i32> {
        let b = self.i8()?;
        if b == i8::MIN {
            self.i32()
        } else {
            Ok(b as i32)
        }
    }

    /// Compressed long: one signed byte, or `-128` escaping to a full i64.
    pub fn compressed_i64(&mut self) -> Result<i64> {
        let b = self.i8()?;
        if b == i8::MIN {
            Ok(self.u64()? as i64)
        } else {
            Ok(b as i64)
        }
    }

    /// Compressed float: `0x80` escapes to a full f32, anything else is 0.0.
    pub fn compressed_f32(&mut self) -> Result<f32> {
        let b = self.u8()?;
        if b == 0x80 {
            self.f32()
        } else {
            Ok(0.0)
        }
    }

    /// Obfuscated string. Negative length = ASCII, positive = UTF-16LE.
    ///
    /// Each branch has its own escape for long strings: ASCII escapes on `-128`,
    /// UTF-16 escapes on `127`. Both then read a full `i32` length. Missing the
    /// UTF-16 escape silently corrupts any string of 127+ characters.
    pub fn string(&mut self) -> Result<String> {
        let tag = self.i8()?;
        if tag == 0 {
            return Ok(String::new());
        }
        if tag < 0 {
            let n = if tag == i8::MIN {
                self.i32()? as usize
            } else {
                (-(tag as i32)) as usize
            };
            let start = self.pos;
            let raw = self.take(n)?;
            let mut out = Vec::with_capacity(n);
            for (k, &b) in raw.iter().enumerate() {
                out.push(b ^ ASCII_MASK.wrapping_add(k as u8));
            }
            // WZ "ASCII" is really Latin-1/CP949 bytes; map 1:1 so no byte is lost.
            let _ = start;
            Ok(out.iter().map(|&b| b as char).collect())
        } else {
            let n = if tag == 127 {
                self.i32()? as usize
            } else {
                tag as usize
            };
            let start = self.pos;
            let raw = self.take(n * 2)?;
            let mut units = Vec::with_capacity(n);
            for k in 0..n {
                let u = u16::from_le_bytes([raw[k * 2], raw[k * 2 + 1]]);
                units.push(u ^ UNICODE_MASK.wrapping_add(k as u16));
            }
            String::from_utf16(&units).map_err(|_| WzError::BadUtf16 { offset: start })
        }
    }

    /// String that may live elsewhere in the archive (used by directory entry type 2
    /// and by image property names).
    ///
    /// `base` is the origin that offsets are relative to. `inline` / `deref` list the
    /// tag bytes that select each mode, since images and directories use different tags.
    pub fn string_at(&mut self, base: usize, inline: &[u8], deref: &[u8]) -> Result<String> {
        let tag = self.u8()?;
        if inline.contains(&tag) {
            self.string()
        } else if deref.contains(&tag) {
            let off = self.u32()? as usize;
            let save = self.pos;
            self.seek(base + off);
            let s = self.string()?;
            self.seek(save);
            Ok(s)
        } else {
            // Unknown tag: treat as inline, which is what the client does in practice.
            self.pos -= 1;
            self.string()
        }
    }

    /// Decrypt a directory entry offset. Must be called with `self.pos` at the
    /// offset field, and consumes it.
    pub fn decrypt_offset(&mut self) -> Result<u32> {
        let field_pos = self.pos as u32;
        let encrypted = self.u32()?;

        let mut o = (field_pos.wrapping_sub(self.fstart)) ^ u32::MAX;
        o = o.wrapping_mul(self.version_hash);
        o = o.wrapping_sub(OFFSET_MAGIC);
        o = o.rotate_left(o & 0x1F);
        o ^= encrypted;
        o = o.wrapping_add(self.fstart.wrapping_mul(2));
        Ok(o)
    }
}

/// Full 32-bit version hash: `sum = sum*32 + ch + 1` over the decimal digits.
pub fn version_hash(version: u16) -> u32 {
    let mut sum: u32 = 0;
    for ch in version.to_string().bytes() {
        sum = sum.wrapping_mul(32).wrapping_add(ch as u32).wrapping_add(1);
    }
    sum
}

/// The `encVer` byte a given version produces, for matching against the file header.
pub fn enc_version(version: u16) -> u16 {
    let h = version_hash(version);
    let x = 0xFFu32
        ^ ((h >> 24) & 0xFF)
        ^ ((h >> 16) & 0xFF)
        ^ ((h >> 8) & 0xFF)
        ^ (h & 0xFF);
    x as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_779_matches_client() {
        // Values confirmed against the shipped archives; see docs/wz-format.md.
        assert_eq!(version_hash(779), 0x0000_E73A);
        assert_eq!(enc_version(779), 34);
    }

    #[test]
    fn ascii_string_decodes_without_key() {
        // "Character" as it appears in Base.wz.
        let raw = [0xF7u8, 0xE9, 0xC3, 0xCD, 0xDF, 0xCF, 0xCC, 0xC4, 0xD4, 0xC0];
        let mut r = WzReader::new(&raw, 0x3C, version_hash(779));
        assert_eq!(r.string().unwrap(), "Character");
    }

    #[test]
    fn utf16_string_escapes_at_127() {
        // A 130-char UTF-16 string must escape via tag 127 + i32 length. Reading the
        // tag as the length instead silently corrupts every long string in QuestData.
        let n: usize = 130;
        let text: String = std::iter::repeat_n('A', n).collect();
        let mut raw = vec![127u8];
        raw.extend_from_slice(&(n as i32).to_le_bytes());
        for (k, ch) in text.chars().enumerate() {
            let enc = (ch as u16) ^ 0xAAAAu16.wrapping_add(k as u16);
            raw.extend_from_slice(&enc.to_le_bytes());
        }
        let mut r = WzReader::new(&raw, 0, 0);
        assert_eq!(r.string().unwrap(), text);
    }

    #[test]
    fn ascii_string_escapes_at_neg128() {
        let n: usize = 200;
        let text: String = std::iter::repeat_n('x', n).collect();
        // Both escapes store a positive count; only the tag's sign picks the encoding.
        let mut raw = vec![0x80u8];
        raw.extend_from_slice(&(n as i32).to_le_bytes());
        for k in 0..n {
            raw.push(b'x' ^ 0xAAu8.wrapping_add(k as u8));
        }
        let mut r = WzReader::new(&raw, 0, 0);
        assert_eq!(r.string().unwrap(), text);
    }

    #[test]
    fn compressed_int_escapes_to_i32() {
        let raw = [0x05u8, 0x80, 0x01, 0x02, 0x03, 0x04];
        let mut r = WzReader::new(&raw, 0, 0);
        assert_eq!(r.compressed_i32().unwrap(), 5);
        assert_eq!(r.compressed_i32().unwrap(), 0x0403_0201);
    }
}
