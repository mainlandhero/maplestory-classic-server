//! Turning a TCP byte stream into discrete packets.
//!
//! TCP gives no message boundaries: a single read may return half a packet, or three
//! packets plus a fragment. [`Framer`] buffers bytes and yields complete, decrypted
//! bodies, so servers never have to reason about partial reads.

use crate::codec::{Cipher, HEADER_LEN, MAX_PACKET_LEN};
use crate::error::{NetError, Result};

/// Where the framer is in the header/body cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramerState {
    /// Waiting for the 4-byte header.
    Header,
    /// Header decoded; waiting for this many body bytes.
    Body(usize),
}

/// Accumulates stream bytes and emits whole packet bodies.
pub struct Framer<C: Cipher> {
    cipher: C,
    buf: Vec<u8>,
    state: FramerState,
    max_len: usize,
}

impl<C: Cipher> Framer<C> {
    /// The cipher this framer wraps.
    ///
    /// Exposed so a caller can read the IV that the *next* packet will be transformed with,
    /// before `next_packet` rolls it. The channel server needs that to show a body under
    /// both cipher polarities while the direction is unsettled.
    pub fn cipher(&self) -> &C {
        &self.cipher
    }

    pub fn new(cipher: C) -> Self {
        Self {
            cipher,
            buf: Vec::with_capacity(4096),
            state: FramerState::Header,
            max_len: MAX_PACKET_LEN,
        }
    }

    /// Override the maximum accepted body length.
    pub fn with_max_len(mut self, max_len: usize) -> Self {
        self.max_len = max_len;
        self
    }

    pub fn state(&self) -> FramerState {
        self.state
    }

    pub fn buffered(&self) -> usize {
        self.buf.len()
    }

    pub fn cipher_mut(&mut self) -> &mut C {
        &mut self.cipher
    }

    /// Feed freshly-read bytes in.
    pub fn feed(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    /// Pop the next complete packet body, decrypted, or `Ok(None)` if more bytes are
    /// needed. Call in a loop until it returns `None` — one read can carry several
    /// packets.
    pub fn next_packet(&mut self) -> Result<Option<Vec<u8>>> {
        loop {
            match self.state {
                FramerState::Header => {
                    if self.buf.len() < HEADER_LEN {
                        return Ok(None);
                    }
                    let mut header = [0u8; HEADER_LEN];
                    header.copy_from_slice(&self.buf[..HEADER_LEN]);
                    let len = self.cipher.decode_len(header)?;
                    if len > self.max_len {
                        return Err(NetError::PacketTooLarge { len, max: self.max_len });
                    }
                    self.buf.drain(..HEADER_LEN);
                    self.state = FramerState::Body(len);
                    // Loop rather than return: the body may already be buffered.
                }
                FramerState::Body(len) => {
                    if self.buf.len() < len {
                        return Ok(None);
                    }
                    let mut body: Vec<u8> = self.buf.drain(..len).collect();
                    self.cipher.decrypt(&mut body)?;
                    self.state = FramerState::Header;
                    return Ok(Some(body));
                }
            }
        }
    }

    /// Encrypt a body and return header-prefixed bytes ready to write to the socket.
    pub fn frame(&mut self, body: &[u8]) -> Vec<u8> {
        let mut body = body.to_vec();
        let header = self.cipher.encrypt(&mut body);
        let mut out = Vec::with_capacity(HEADER_LEN + body.len());
        out.extend_from_slice(&header);
        out.extend_from_slice(&body);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::PlainCipher;

    fn framed(bodies: &[&[u8]]) -> Vec<u8> {
        let mut f = Framer::new(PlainCipher);
        let mut out = Vec::new();
        for b in bodies {
            out.extend_from_slice(&f.frame(b));
        }
        out
    }

    #[test]
    fn reads_a_single_packet() {
        let wire = framed(&[b"hello"]);
        let mut f = Framer::new(PlainCipher);
        f.feed(&wire);
        assert_eq!(f.next_packet().unwrap().as_deref(), Some(&b"hello"[..]));
        assert_eq!(f.next_packet().unwrap(), None);
    }

    #[test]
    fn reads_several_packets_from_one_read() {
        let wire = framed(&[b"one", b"two", b"three"]);
        let mut f = Framer::new(PlainCipher);
        f.feed(&wire);
        assert_eq!(f.next_packet().unwrap().as_deref(), Some(&b"one"[..]));
        assert_eq!(f.next_packet().unwrap().as_deref(), Some(&b"two"[..]));
        assert_eq!(f.next_packet().unwrap().as_deref(), Some(&b"three"[..]));
        assert_eq!(f.next_packet().unwrap(), None);
    }

    #[test]
    fn reassembles_packets_split_byte_by_byte() {
        // The pathological case: TCP delivers one byte at a time.
        let wire = framed(&[b"alpha", b"beta"]);
        let mut f = Framer::new(PlainCipher);
        let mut got: Vec<Vec<u8>> = Vec::new();
        for byte in &wire {
            f.feed(&[*byte]);
            while let Some(p) = f.next_packet().unwrap() {
                got.push(p);
            }
        }
        assert_eq!(got, vec![b"alpha".to_vec(), b"beta".to_vec()]);
    }

    #[test]
    fn handles_a_split_inside_the_header() {
        let wire = framed(&[b"payload"]);
        let mut f = Framer::new(PlainCipher);
        f.feed(&wire[..2]); // half a header
        assert_eq!(f.next_packet().unwrap(), None);
        f.feed(&wire[2..]);
        assert_eq!(f.next_packet().unwrap().as_deref(), Some(&b"payload"[..]));
    }

    #[test]
    fn rejects_an_oversized_length() {
        let mut f = Framer::new(PlainCipher).with_max_len(16);
        f.feed(&1000u32.to_le_bytes());
        assert!(matches!(
            f.next_packet(),
            Err(NetError::PacketTooLarge { len: 1000, max: 16 })
        ));
    }

    #[test]
    fn empty_body_does_not_stall_the_stream() {
        let wire = framed(&[b"", b"after"]);
        let mut f = Framer::new(PlainCipher);
        f.feed(&wire);
        assert_eq!(f.next_packet().unwrap().as_deref(), Some(&b""[..]));
        assert_eq!(f.next_packet().unwrap().as_deref(), Some(&b"after"[..]));
    }
}
