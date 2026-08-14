use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum WzError {
    #[error("io error on {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("not a WZ archive: expected magic PKG1, found {found:02X?}")]
    BadMagic { found: [u8; 4] },

    #[error("unexpected end of data at offset {offset} (need {need} bytes, {have} available)")]
    UnexpectedEof {
        offset: usize,
        need: usize,
        have: usize,
    },

    #[error("unknown directory entry type {kind} at offset {offset}")]
    BadEntryType { kind: u8, offset: usize },

    #[error("unknown property type {kind} at offset {offset}")]
    BadPropertyType { kind: u8, offset: usize },

    #[error("unknown extended property {name:?} at offset {offset}")]
    BadExtendedType { name: String, offset: usize },

    #[error("entry {name:?} points out of bounds (offset {offset}, size {size}, file {len})")]
    OutOfBounds {
        name: String,
        offset: u32,
        size: i32,
        len: usize,
    },

    #[error("could not determine version: no candidate matched encVer {enc_ver}")]
    VersionNotFound { enc_ver: u16 },

    #[error("invalid UTF-16 in string at offset {offset}")]
    BadUtf16 { offset: usize },
}

pub type Result<T> = std::result::Result<T, WzError>;
