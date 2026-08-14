#[derive(Debug, thiserror::Error)]
pub enum NetError {
    #[error("packet underflow at offset {offset}: need {need} bytes, {have} remaining")]
    PacketUnderflow {
        offset: usize,
        need: usize,
        have: usize,
    },

    #[error("bad packet header {header:02X?} (decoded length {decoded_len})")]
    BadHeader {
        header: [u8; 4],
        decoded_len: usize,
    },

    #[error("packet of {len} bytes exceeds the {max} byte limit")]
    PacketTooLarge { len: usize, max: usize },

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, NetError>;
