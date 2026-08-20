//! Packet primitives, framing, and pluggable ciphers shared by every MapleCW server.
//!
//! Two layers, deliberately separate:
//!
//! * [`PacketWriter`] / [`PacketReader`] — packet *bodies*. These are well understood
//!   (little-endian, `u16`-length-prefixed strings) and independent of any cipher, so
//!   they can be built and tested now.
//! * [`Cipher`] — the wire framing. [`MapleCipher`] implements this client's actual
//!   scheme, read out of its receive path and checked against a real capture; see
//!   `docs/transport.md`. [`PlainCipher`] stays available for tests.
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

pub mod bag;
pub mod channel;
pub mod codec;
pub mod combat;
pub mod drops;
pub mod error;
pub mod handshake;
pub mod inventory;
pub mod mob;
pub mod mobmove;
pub mod names;
pub mod notice;
pub mod npcchat;
pub mod opcode;
pub mod packet;
pub mod quest;
pub mod script;
pub mod session;
pub mod shop;
pub mod stats;
pub mod userchat;
pub mod userhit;
pub mod usermove;

pub use codec::{
    shift_body, ByteShiftCipher, Cipher, Direction, MapleCipher, PlainCipher, Shift,
    HEADER_LEN, MAX_PACKET_LEN,
};
pub use error::{NetError, Result};
pub use opcode::{data_wz_up_to_date, zigzag_varint, DATA_WZ_PATCH, LOGIN_OK, LOGIN_RESULT};
pub use packet::{PacketReader, PacketWriter};
pub use session::{Framer, FramerState};
