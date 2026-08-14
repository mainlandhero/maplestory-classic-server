//! WZ archive: header, version detection, directory tree.

use std::path::{Path, PathBuf};

use crate::error::{Result, WzError};
use crate::reader::{enc_version, version_hash, WzReader};

pub const MAGIC: &[u8; 4] = b"PKG1";

#[derive(Debug, Clone)]
pub struct Header {
    pub fsize: u64,
    pub fstart: u32,
    pub copyright: String,
    pub enc_ver: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Directory,
    Image,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub kind: NodeKind,
    pub offset: u32,
    pub size: i32,
    pub checksum: i32,
    pub children: Vec<Node>,
}

impl Node {
    pub fn is_dir(&self) -> bool {
        self.kind == NodeKind::Directory
    }

    /// Walk to a node by `/`-separated path.
    pub fn get(&self, path: &str) -> Option<&Node> {
        let mut cur = self;
        for part in path.split('/').filter(|p| !p.is_empty()) {
            cur = cur.children.iter().find(|c| c.name == part)?;
        }
        Some(cur)
    }

    pub fn count_recursive(&self) -> (usize, usize) {
        let (mut dirs, mut imgs) = (0, 0);
        for c in &self.children {
            match c.kind {
                NodeKind::Directory => dirs += 1,
                NodeKind::Image => imgs += 1,
            }
            let (d, i) = c.count_recursive();
            dirs += d;
            imgs += i;
        }
        (dirs, imgs)
    }
}

pub struct Archive {
    pub path: PathBuf,
    pub data: Vec<u8>,
    pub header: Header,
    pub version: u16,
    pub version_hash: u32,
    pub root: Node,
}

impl Archive {
    /// Open an archive, detecting the version automatically.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::open_inner(path.as_ref(), None)
    }

    /// Open an archive with a known version (skips detection).
    pub fn open_with_version(path: impl AsRef<Path>, version: u16) -> Result<Self> {
        Self::open_inner(path.as_ref(), Some(version))
    }

    fn open_inner(path: &Path, version: Option<u16>) -> Result<Self> {
        let data = std::fs::read(path).map_err(|e| WzError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
        let header = parse_header(&data)?;

        let version = match version {
            Some(v) => v,
            None => detect_version(&data, &header)?,
        };
        let vhash = version_hash(version);

        let mut r = WzReader::new(&data, header.fstart, vhash);
        r.seek(header.fstart as usize + 2);
        let children = read_directory(&mut r, &data)?;

        Ok(Self {
            path: path.to_path_buf(),
            header,
            version,
            version_hash: vhash,
            root: Node {
                name: path
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                kind: NodeKind::Directory,
                offset: 0,
                size: 0,
                checksum: 0,
                children,
            },
            data,
        })
    }

    /// Raw bytes of an image node.
    pub fn image_bytes(&self, node: &Node) -> Result<&[u8]> {
        let start = node.offset as usize;
        let end = start + node.size.max(0) as usize;
        if node.kind != NodeKind::Image || end > self.data.len() {
            return Err(WzError::OutOfBounds {
                name: node.name.clone(),
                offset: node.offset,
                size: node.size,
                len: self.data.len(),
            });
        }
        Ok(&self.data[start..end])
    }
}

pub fn parse_header(data: &[u8]) -> Result<Header> {
    if data.len() < 16 {
        return Err(WzError::UnexpectedEof {
            offset: 0,
            need: 16,
            have: data.len(),
        });
    }
    let magic: [u8; 4] = [data[0], data[1], data[2], data[3]];
    if &magic != MAGIC {
        return Err(WzError::BadMagic { found: magic });
    }
    let fsize = u64::from_le_bytes(data[4..12].try_into().unwrap());
    let fstart = u32::from_le_bytes(data[12..16].try_into().unwrap());

    let end = data[16..]
        .iter()
        .position(|&b| b == 0)
        .map(|p| 16 + p)
        .unwrap_or(data.len());
    let copyright = String::from_utf8_lossy(&data[16..end]).into_owned();

    let fs = fstart as usize;
    if fs + 2 > data.len() {
        return Err(WzError::UnexpectedEof {
            offset: fs,
            need: 2,
            have: data.len().saturating_sub(fs),
        });
    }
    let enc_ver = u16::from_le_bytes([data[fs], data[fs + 1]]);

    Ok(Header { fsize, fstart, copyright, enc_ver })
}

/// Find the version by matching `encVer`, then confirming that every root entry
/// decodes to an in-bounds offset.
///
/// Many trees ship a 63-byte stub whose root entry count is 0 (the real content
/// lives in a sibling archive). There is nothing to validate against in that case,
/// so any `encVer`-consistent version is accepted.
pub fn detect_version(data: &[u8], header: &Header) -> Result<u16> {
    let mut first_match: Option<u16> = None;

    for v in 0u16..=u16::MAX {
        if enc_version(v) != header.enc_ver {
            continue;
        }
        first_match.get_or_insert(v);

        let mut r = WzReader::new(data, header.fstart, version_hash(v));
        r.seek(header.fstart as usize + 2);
        if let Ok(children) = read_directory_shallow(&mut r, data) {
            if children.is_empty() {
                // Empty archive: offsets carry no signal, so stop probing.
                return Ok(crate::CLIENT_VERSION);
            }
            if children.iter().all(|n| (n.offset as usize) < data.len()) {
                return Ok(v);
            }
        }
    }

    first_match.ok_or(WzError::VersionNotFound {
        enc_ver: header.enc_ver,
    })
}

/// Entry names are stored inline (tags 3/4) or at an absolute position (tag 2).
fn read_entry_header(r: &mut WzReader, data: &[u8]) -> Result<Option<(String, NodeKind)>> {
    let start = r.pos;
    let tag = r.u8()?;
    match tag {
        // Padding / unused slot.
        1 => {
            r.take(10)?;
            Ok(None)
        }
        // Name and real type live elsewhere in the file.
        2 => {
            let rel = r.i32()?;
            let at = (r.fstart as i64 + 1 + rel as i64) as usize;
            if at >= data.len() {
                return Err(WzError::UnexpectedEof {
                    offset: at,
                    need: 1,
                    have: 0,
                });
            }
            let real = data[at - 1];
            let mut nr = WzReader::new(data, r.fstart, r.version_hash);
            nr.seek(at);
            let name = nr.string()?;
            let kind = if real == 3 {
                NodeKind::Directory
            } else {
                NodeKind::Image
            };
            Ok(Some((name, kind)))
        }
        3 | 4 => {
            let name = r.string()?;
            let kind = if tag == 3 {
                NodeKind::Directory
            } else {
                NodeKind::Image
            };
            Ok(Some((name, kind)))
        }
        other => Err(WzError::BadEntryType {
            kind: other,
            offset: start,
        }),
    }
}

fn read_entries(r: &mut WzReader, data: &[u8]) -> Result<Vec<Node>> {
    let count = r.compressed_i32()?;
    if !(0..=100_000).contains(&count) {
        return Err(WzError::BadEntryType {
            kind: 0,
            offset: r.pos,
        });
    }
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let Some((name, kind)) = read_entry_header(r, data)? else {
            continue;
        };
        let size = r.compressed_i32()?;
        let checksum = r.compressed_i32()?;
        let offset = r.decrypt_offset()?;
        out.push(Node {
            name,
            kind,
            offset,
            size,
            checksum,
            children: Vec::new(),
        });
    }
    Ok(out)
}

/// One level only — used by version detection so a wrong guess fails fast.
fn read_directory_shallow(r: &mut WzReader, data: &[u8]) -> Result<Vec<Node>> {
    read_entries(r, data)
}

/// Full recursive tree.
fn read_directory(r: &mut WzReader, data: &[u8]) -> Result<Vec<Node>> {
    let mut nodes = read_entries(r, data)?;
    for n in &mut nodes {
        if n.kind == NodeKind::Directory && (n.offset as usize) < data.len() {
            let mut sub = WzReader::new(data, r.fstart, r.version_hash);
            sub.seek(n.offset as usize);
            // A malformed subtree should not sink the whole archive.
            n.children = read_directory(&mut sub, data).unwrap_or_default();
        }
    }
    Ok(nodes)
}
