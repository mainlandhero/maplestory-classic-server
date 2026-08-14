//! Parser for Wizet WZ archives as shipped with the `mscw` MapleStory client.
//!
//! This client uses **version 779** (hash `0x0000E73A`) and a **zero string key**,
//! both determined empirically — see `docs/wz-format.md`.
//!
//! On disk each data tree is split: `<Tree>.wz` is a stub index and
//! `<Tree>_000.wz` is the real standalone archive. Open the `_000` file.
//!
//! ```no_run
//! # fn main() -> Result<(), wz::WzError> {
//! let ar = wz::Archive::open("Data/String/String_000.wz")?;
//! for node in &ar.root.children {
//!     println!("{} ({} bytes)", node.name, node.size);
//! }
//! # Ok(()) }
//! ```

pub mod archive;
pub mod error;
pub mod prop;
pub mod reader;

pub use archive::{Archive, Header, Node, NodeKind};
pub use error::{Result, WzError};
pub use prop::{parse_image, to_json, Property, Value};
pub use reader::{enc_version, version_hash, WzReader};

/// The version used by this client.
pub const CLIENT_VERSION: u16 = 779;

/// Convenience: the `_000` archive path for a data tree directory.
pub fn tree_archive(data_dir: &std::path::Path, tree: &str) -> std::path::PathBuf {
    data_dir.join(tree).join(format!("{tree}_000.wz"))
}
