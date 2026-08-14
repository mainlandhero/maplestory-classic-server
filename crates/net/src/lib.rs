//! Packet primitives, framing, and pluggable ciphers shared by every MapleCW server.
//!
//! Two layers, deliberately separate:
//!
//! * [`PacketWriter`] / [`PacketReader`] — packet *bodies*. These are well understood
//!   (little-endian, `u16`-length-prefixed strings) and independent of any cipher, so
//!   they can be built and tested now.
//! * [`Cipher`] — the wire framing. This client's scheme is **not yet known**
//!   (`MapleSecurePC64.dll` wraps the socket layer), so the concrete cipher is a
//!   swappable choice. [`PlainCipher`] enables end-to-end development today;
//!   [`MapleCipher`] is the classic scheme kept as the first hypothesis to test.
//!
//! ```
//! use net::{PacketWriter, PacketReader};
//! let mut w = PacketWriter::with_opcode(0x0001);
//! w.str("Scania").u8(1);
//!
//! let buf = w.into_vec();
//! let mut r = PacketReader::new(&buf);
//! assert_eq!(r.u16().unwrap(), 0x0001);
//! assert_eq!(r.str().unwrap(), "Scania");
//! ```

pub mod codec;
pub mod error;
pub mod packet;
pub mod session;

pub use codec::{Cipher, MapleCipher, PlainCipher, HEADER_LEN, MAX_PACKET_LEN};
pub use error::{NetError, Result};
pub use packet::{PacketReader, PacketWriter};
pub use session::{Framer, FramerState};
