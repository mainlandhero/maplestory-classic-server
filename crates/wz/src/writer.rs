//! Writing WZ archives and images - the inverse of `archive.rs` and `prop.rs`.
//!
//! Built 2026-09-10 to put the modern client's "Signature Style Collection" assets into the
//! classic client's data (`backport/signature-style/README.md`). Two facts make it small:
//!
//! * **An image is self-contained.** Every offset inside an image is relative to the image's
//!   own first byte (`parse_image` reads with base 0), and nothing in it depends on the
//!   archive version. So an image from the v271 client is copied into a v779 archive byte
//!   for byte. Only the archive layer - header, directory, encrypted offsets, checksums -
//!   has to be produced here.
//! * **The directory checksum is the plain byte sum** of the image, as `i32`. Checked on
//!   `client-patched/Data/Character/Longcoat/Longcoat_000.wz`: the first four entries'
//!   stored checksums equal the wrapping sum of their bytes.
//!
//! Images DO have to be re-serialised when they are edited - a string table gaining names,
//! an item image gaining nodes - and [`Owned`] is the editable form: the same tree as
//! [`crate::Value`] with canvas payloads carried as bytes rather than as offsets into a
//! source buffer, so trees from two different archives can be merged.
//!
//! **Every encoding here mirrors one decode in `reader.rs` / `prop.rs`**, and the tests
//! round-trip real images from the classic client through parse -> owned -> serialise ->
//! parse and require the JSON to be identical. A writer that only agrees with itself is
//! the kind of instrument `CLAUDE.md` warns about.

use std::collections::BTreeMap;

use crate::archive::{parse_header, Archive, MAGIC};
use crate::error::{Result, WzError};
use crate::prop::{parse_image, Property, Value};
use crate::reader::{enc_version, version_hash, WzReader, OFFSET_MAGIC};

const COPYRIGHT: &str = "Package file v1.0 Copyright 2002 Wizet, ZMS";
const ASCII_MASK: u8 = 0xAA;
const UNICODE_MASK: u16 = 0xAAAA;

/// Byte-level encoders. Each one is the inverse of the `WzReader` method of the same name.
pub struct WzWriter {
    pub buf: Vec<u8>,
}

impl Default for WzWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl WzWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn u8(&mut self, v: u8) {
        self.buf.push(v);
    }

    pub fn u16(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn i16(&mut self, v: i16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn i32(&mut self, v: i32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn u64(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn f64(&mut self, v: f64) {
        self.u64(v.to_bits());
    }

    /// Compressed int: one signed byte unless it needs the `-128` escape to a full i32.
    /// `-128` itself must escape, or the reader takes it as the escape marker.
    pub fn compressed_i32(&mut self, v: i32) {
        if v > i8::MIN as i32 && v <= i8::MAX as i32 {
            self.u8(v as i8 as u8);
        } else {
            self.u8(0x80);
            self.i32(v);
        }
    }

    pub fn compressed_i64(&mut self, v: i64) {
        if v > i8::MIN as i64 && v <= i8::MAX as i64 {
            self.u8(v as i8 as u8);
        } else {
            self.u8(0x80);
            self.u64(v as u64);
        }
    }

    /// Compressed float: a zero is one `0x00` byte; anything else is `0x80` + f32.
    pub fn compressed_f32(&mut self, v: f32) {
        if v == 0.0 {
            self.u8(0);
        } else {
            self.u8(0x80);
            self.u32(v.to_bits());
        }
    }

    /// Obfuscated string, zero key. **ASCII only when every char is below 0x80** - the
    /// reader maps the ASCII branch's bytes 1:1 to chars, so a Latin-1 `\u{dc}` written on
    /// that branch would read back correctly here but the CLIENT decodes those bytes as
    /// CP949, and "Ubel" with an umlaut would draw as something else. UTF-16 is unambiguous.
    pub fn string(&mut self, s: &str) {
        if s.is_empty() {
            self.u8(0);
            return;
        }
        if s.bytes().all(|b| b < 0x80) {
            let n = s.len();
            if n >= 128 {
                self.u8(0x80);
                self.i32(n as i32);
            } else {
                self.u8((-(n as i32)) as i8 as u8);
            }
            for (k, b) in s.bytes().enumerate() {
                self.u8(b ^ ASCII_MASK.wrapping_add(k as u8));
            }
        } else {
            let units: Vec<u16> = s.encode_utf16().collect();
            let n = units.len();
            if n >= 127 {
                self.u8(127);
                self.i32(n as i32);
            } else {
                self.u8(n as u8);
            }
            for (k, u) in units.iter().enumerate() {
                self.u16(u ^ UNICODE_MASK.wrapping_add(k as u16));
            }
        }
    }

    /// A name or string value inside an image, always written inline with the `0x00`
    /// tag (the reader accepts `0x00`, `0x04` and `0x73` as inline). Dereferenced strings
    /// are a space optimisation the original packer used; nothing requires them.
    pub fn image_string(&mut self, s: &str) {
        self.u8(0x00);
        self.string(s);
    }

    /// An extended-type name, written with the `0x73` inline tag the original packer uses
    /// for the first occurrence of each type name.
    pub fn type_string(&mut self, s: &str) {
        self.u8(0x73);
        self.string(s);
    }
}

/// Encrypt an image offset for a directory entry whose offset field sits at `field_pos`.
/// The exact inverse of `WzReader::decrypt_offset`.
pub fn encrypt_offset(field_pos: u32, fstart: u32, version_hash: u32, real: u32) -> u32 {
    let mut o = field_pos.wrapping_sub(fstart) ^ u32::MAX;
    o = o.wrapping_mul(version_hash);
    o = o.wrapping_sub(OFFSET_MAGIC);
    o = o.rotate_left(o & 0x1F);
    o ^ real.wrapping_sub(fstart.wrapping_mul(2))
}

/// The directory checksum: the wrapping byte sum of the image.
pub fn checksum(bytes: &[u8]) -> i32 {
    bytes.iter().fold(0i32, |acc, &b| acc.wrapping_add(b as i32))
}

/// One image going into an archive.
pub struct ImageEntry {
    pub name: String,
    pub bytes: Vec<u8>,
}

/// Build a complete standalone archive: header, one flat directory of images, the images.
///
/// Flat because every `<Tree>_000.wz` this client ships is flat (`wz-dump tree` shows
/// `0 directories` for all of them); subdirectories exist only in `Base.wz`, which is not
/// rewritten. `fstart` is `0x3C`, the same as every shipped archive, because the copyright
/// string is the same length.
pub fn write_archive(version: u16, images: &[ImageEntry]) -> Vec<u8> {
    let vh = version_hash(version);
    let fstart: u32 = (4 + 8 + 4 + COPYRIGHT.len() + 1) as u32; // 0x3C
    debug_assert_eq!(fstart, 0x3C);

    // The directory, with the offset fields left as placeholders and their positions noted.
    let mut dir = WzWriter::new();
    dir.u16(enc_version(version));
    dir.compressed_i32(images.len() as i32);
    let mut offset_fields = Vec::with_capacity(images.len());
    for img in images {
        dir.u8(4);
        dir.string(&img.name);
        dir.compressed_i32(img.bytes.len() as i32);
        dir.compressed_i32(checksum(&img.bytes));
        offset_fields.push(dir.buf.len());
        dir.u32(0);
    }

    // Where each image lands: straight after the directory, in order.
    let mut out = Vec::with_capacity(
        fstart as usize + dir.buf.len() + images.iter().map(|i| i.bytes.len()).sum::<usize>(),
    );
    let mut pos = fstart as usize + dir.buf.len();
    for (i, img) in images.iter().enumerate() {
        let field_pos = fstart as usize + offset_fields[i];
        let enc = encrypt_offset(field_pos as u32, fstart, vh, pos as u32);
        dir.buf[offset_fields[i]..offset_fields[i] + 4].copy_from_slice(&enc.to_le_bytes());
        pos += img.bytes.len();
    }

    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&((pos as u64) - fstart as u64).to_le_bytes()); // fsize
    out.extend_from_slice(&fstart.to_le_bytes());
    out.extend_from_slice(COPYRIGHT.as_bytes());
    out.push(0);
    out.extend_from_slice(&dir.buf);
    for img in images {
        out.extend_from_slice(&img.bytes);
    }
    debug_assert_eq!(out.len(), pos);
    out
}

// ---------------------------------------------------------------------------------------
// Editable image trees
// ---------------------------------------------------------------------------------------

/// An image tree that owns its bytes, so it can be built, merged and serialised.
#[derive(Debug, Clone, PartialEq)]
pub enum Owned {
    Null,
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    Vector(i32, i32),
    Uol(String),
    /// Children in order. A `BTreeMap` would lose the on-disk order, which the client
    /// depends on for frame lists (`0`, `1`, `2` ...).
    Object(Vec<(String, Owned)>),
    Canvas {
        width: i32,
        height: i32,
        format: i32,
        payload: Vec<u8>,
        children: Vec<(String, Owned)>,
    },
    Convex(Vec<Owned>),
}

impl Owned {
    /// Convert a parsed image, pulling canvas payloads out of `image_bytes`.
    ///
    /// Sound and RawData are refused rather than silently dropped: `Value` records neither
    /// payload's position, so they cannot be carried, and none of the equip, item or string
    /// images this was built for contains one.
    pub fn from_value(v: &Value, image_bytes: &[u8]) -> Result<Owned> {
        Ok(match v {
            Value::Null => Owned::Null,
            Value::Short(x) => Owned::Short(*x),
            Value::Int(x) => Owned::Int(*x),
            Value::Long(x) => Owned::Long(*x),
            Value::Float(x) => Owned::Float(*x),
            Value::Double(x) => Owned::Double(*x),
            Value::String(s) => Owned::String(s.clone()),
            Value::Vector(x, y) => Owned::Vector(*x, *y),
            Value::Uol(s) => Owned::Uol(s.clone()),
            Value::Object(props) => Owned::Object(props_from(props, image_bytes)?),
            Value::Canvas { width, height, format, data_off, data_len, children } => {
                let end = data_off + data_len;
                if end > image_bytes.len() {
                    return Err(WzError::UnexpectedEof {
                        offset: *data_off,
                        need: *data_len,
                        have: image_bytes.len().saturating_sub(*data_off),
                    });
                }
                Owned::Canvas {
                    width: *width,
                    height: *height,
                    format: *format,
                    payload: image_bytes[*data_off..end].to_vec(),
                    children: props_from(children, image_bytes)?,
                }
            }
            Value::Convex(items) => Owned::Convex(
                items.iter().map(|i| Owned::from_value(i, image_bytes)).collect::<Result<_>>()?,
            ),
            Value::Sound { .. } => {
                return Err(WzError::BadExtendedType { name: "Sound_DX8".into(), offset: 0 })
            }
            Value::RawData { .. } => {
                return Err(WzError::BadExtendedType { name: "RawData".into(), offset: 0 })
            }
        })
    }

    /// Parse and own an image in one step.
    pub fn parse(image_bytes: &[u8]) -> Result<Owned> {
        Owned::from_value(&parse_image(image_bytes)?, image_bytes)
    }

    pub fn children(&self) -> Option<&Vec<(String, Owned)>> {
        match self {
            Owned::Object(c) | Owned::Canvas { children: c, .. } => Some(c),
            _ => None,
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut Vec<(String, Owned)>> {
        match self {
            Owned::Object(c) | Owned::Canvas { children: c, .. } => Some(c),
            _ => None,
        }
    }

    pub fn get(&self, name: &str) -> Option<&Owned> {
        self.children()?.iter().find(|(n, _)| n == name).map(|(_, v)| v)
    }

    /// Set (replace or append) a direct child.
    pub fn set(&mut self, name: &str, value: Owned) {
        let Some(kids) = self.children_mut() else { return };
        if let Some(slot) = kids.iter_mut().find(|(n, _)| n == name) {
            slot.1 = value;
        } else {
            kids.push((name.to_string(), value));
        }
    }

    /// Set a leaf by `/`-separated path, creating intermediate objects. Used for string
    /// tables: `ClassicWorld/Longcoat/1054555/name`.
    pub fn set_path(&mut self, path: &str, value: Owned) {
        let mut parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty()).collect();
        let Some(leaf) = parts.pop() else { return };
        let mut cur = self;
        for part in parts {
            if cur.get(part).is_none() {
                cur.set(part, Owned::Object(Vec::new()));
            }
            cur = cur
                .children_mut()
                .and_then(|k| k.iter_mut().find(|(n, _)| n == part))
                .map(|(_, v)| v)
                .expect("just inserted");
        }
        cur.set(leaf, value);
    }

    /// Serialise as a whole image (`Property` root).
    pub fn serialize_image(&self) -> Vec<u8> {
        let mut w = WzWriter::new();
        match self {
            Owned::Object(kids) => {
                w.type_string("Property");
                w.u16(0);
                write_list(&mut w, kids);
            }
            other => write_extended_body(&mut w, other),
        }
        w.buf
    }

    /// Every canvas in the tree, as `(path, width, height, format, payload length)`.
    pub fn canvases(&self) -> Vec<(String, i32, i32, i32, usize)> {
        let mut out = Vec::new();
        fn walk(v: &Owned, path: &str, out: &mut Vec<(String, i32, i32, i32, usize)>) {
            if let Owned::Canvas { width, height, format, payload, .. } = v {
                out.push((path.to_string(), *width, *height, *format, payload.len()));
            }
            if let Some(kids) = v.children() {
                for (n, c) in kids {
                    walk(c, &format!("{path}/{n}"), out);
                }
            }
        }
        walk(self, "", &mut out);
        out
    }
}

fn props_from(props: &[Property], image_bytes: &[u8]) -> Result<Vec<(String, Owned)>> {
    props
        .iter()
        .map(|p| Ok((p.name.clone(), Owned::from_value(&p.value, image_bytes)?)))
        .collect()
}

fn write_list(w: &mut WzWriter, kids: &[(String, Owned)]) {
    w.compressed_i32(kids.len() as i32);
    for (name, v) in kids {
        w.image_string(name);
        write_value(w, v);
    }
}

fn write_value(w: &mut WzWriter, v: &Owned) {
    match v {
        Owned::Null => w.u8(0x00),
        Owned::Short(x) => {
            w.u8(0x02);
            w.i16(*x);
        }
        Owned::Int(x) => {
            w.u8(0x03);
            w.compressed_i32(*x);
        }
        Owned::Long(x) => {
            w.u8(0x14);
            w.compressed_i64(*x);
        }
        Owned::Float(x) => {
            w.u8(0x04);
            w.compressed_f32(*x);
        }
        Owned::Double(x) => {
            w.u8(0x05);
            w.f64(*x);
        }
        Owned::String(s) => {
            w.u8(0x08);
            w.image_string(s);
        }
        extended => {
            // 0x09, u32 length of what follows, then the length-delimited body.
            let mut body = WzWriter::new();
            write_extended_body(&mut body, extended);
            w.u8(0x09);
            w.u32(body.buf.len() as u32);
            w.buf.extend_from_slice(&body.buf);
        }
    }
}

fn write_extended_body(w: &mut WzWriter, v: &Owned) {
    match v {
        Owned::Object(kids) => {
            w.type_string("Property");
            w.u16(0);
            write_list(w, kids);
        }
        Owned::Canvas { width, height, format, payload, children } => {
            w.type_string("Canvas");
            w.u8(0);
            if children.is_empty() {
                w.u8(0);
            } else {
                w.u8(1);
                w.u16(0);
                write_list(w, children);
            }
            w.compressed_i32(*width);
            w.compressed_i32(*height);
            // The reader computes `format = cint + u8`; the second byte is the "format2"
            // the client's packer leaves at 0 for every canvas in this data.
            w.compressed_i32(*format);
            w.u8(0);
            w.u32(0);
            // Declared length counts one leading byte that is not pixel data.
            w.i32(payload.len() as i32 + 1);
            w.u8(0);
            w.buf.extend_from_slice(payload);
        }
        Owned::Vector(x, y) => {
            w.type_string("Shape2D#Vector2D");
            w.compressed_i32(*x);
            w.compressed_i32(*y);
        }
        Owned::Convex(items) => {
            w.type_string("Shape2D#Convex2D");
            w.compressed_i32(items.len() as i32);
            for item in items {
                write_extended_body(w, item);
            }
        }
        Owned::Uol(s) => {
            w.type_string("UOL");
            w.u8(0);
            w.image_string(s);
        }
        // Scalars never reach here: write_value handles them before recursing.
        Owned::Null
        | Owned::Short(_)
        | Owned::Int(_)
        | Owned::Long(_)
        | Owned::Float(_)
        | Owned::Double(_)
        | Owned::String(_) => write_value(w, v),
    }
}

/// Every image of an archive as owned bytes, in directory order, keyed by name. The base
/// for a rebuild: copy these verbatim, then replace or add.
pub fn archive_images(ar: &Archive) -> Result<Vec<ImageEntry>> {
    let mut out = Vec::with_capacity(ar.root.children.len());
    for node in &ar.root.children {
        if node.is_dir() {
            return Err(WzError::BadEntryType { kind: 3, offset: node.offset as usize });
        }
        out.push(ImageEntry { name: node.name.clone(), bytes: ar.image_bytes(node)?.to_vec() });
    }
    Ok(out)
}

/// Replace or append images by name, keeping the base order. Appended images are sorted
/// by name among themselves, which is the order every shipped archive uses.
pub fn merge_images(base: Vec<ImageEntry>, mut additions: Vec<ImageEntry>) -> Vec<ImageEntry> {
    let mut by_name: BTreeMap<String, ImageEntry> = BTreeMap::new();
    for a in additions.drain(..) {
        by_name.insert(a.name.clone(), a);
    }
    let mut out: Vec<ImageEntry> = Vec::with_capacity(base.len() + by_name.len());
    for b in base {
        match by_name.remove(&b.name) {
            Some(replacement) => out.push(replacement),
            None => out.push(b),
        }
    }
    out.extend(by_name.into_values());
    out
}

/// Sanity-check that a written archive parses and holds exactly the images it was given.
pub fn verify_archive(bytes: &[u8], version: u16, images: &[ImageEntry]) -> Result<()> {
    let header = parse_header(bytes)?;
    if header.enc_ver != enc_version(version) {
        return Err(WzError::VersionNotFound { enc_ver: header.enc_ver });
    }
    let mut r = WzReader::new(bytes, header.fstart, version_hash(version));
    r.seek(header.fstart as usize + 2);
    let count = r.compressed_i32()?;
    if count as usize != images.len() {
        return Err(WzError::BadEntryType { kind: 0, offset: r.pos });
    }
    for img in images {
        let tag = r.u8()?;
        if tag != 4 {
            return Err(WzError::BadEntryType { kind: tag, offset: r.pos });
        }
        let name = r.string()?;
        let size = r.compressed_i32()? as usize;
        let sum = r.compressed_i32()?;
        let off = r.decrypt_offset()? as usize;
        if name != img.name || size != img.bytes.len() || sum != checksum(&img.bytes) {
            return Err(WzError::OutOfBounds { name, offset: off as u32, size: size as i32, len: bytes.len() });
        }
        if off + size > bytes.len() || &bytes[off..off + size] != img.bytes.as_slice() {
            return Err(WzError::OutOfBounds { name, offset: off as u32, size: size as i32, len: bytes.len() });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::to_json;

    fn classic(tree: &str) -> Option<Archive> {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../client-patched/Data")
            .join(tree);
        if !p.exists() {
            return None;
        }
        Some(Archive::open(p).unwrap())
    }

    /// `encrypt_offset` is the exact inverse of the reader's decrypt, at the positions and
    /// values a real archive uses.
    #[test]
    fn offsets_round_trip_through_the_reader() {
        let vh = version_hash(779);
        for (field_pos, real) in [(0x3Eu32, 0x13BBu32), (0x1000, 0x8874), (0x2_0000, 0x1FF_FFFF)] {
            let enc = encrypt_offset(field_pos, 0x3C, vh, real);
            let mut bytes = vec![0u8; field_pos as usize + 4];
            bytes[field_pos as usize..].copy_from_slice(&enc.to_le_bytes());
            let mut r = WzReader::new(&bytes, 0x3C, vh);
            r.seek(field_pos as usize);
            assert_eq!(r.decrypt_offset().unwrap(), real, "field {field_pos:#x} real {real:#x}");
        }
    }

    /// Every string shape round-trips: empty, short ASCII, long ASCII (escape), short and
    /// long UTF-16 (escape), and the umlaut that must NOT take the ASCII branch.
    #[test]
    fn strings_round_trip_and_umlauts_go_utf16() {
        let cases = [
            String::new(),
            "Property".to_string(),
            "x".repeat(200),
            "\u{dc}bel's Clothes".to_string(),
            "A".repeat(130) + "\u{fc}",
        ];
        for s in &cases {
            let mut w = WzWriter::new();
            w.string(s);
            let mut r = WzReader::new(&w.buf, 0, 0);
            assert_eq!(&r.string().unwrap(), s);
            if s.chars().any(|c| c as u32 >= 0x80) {
                assert!(w.buf[0] as i8 > 0 || w.buf[0] == 127, "non-ASCII must be UTF-16: {s:?}");
            }
        }
        // Compressed ints at both escape edges.
        for v in [0i32, 1, -1, 127, -127, -128, 128, 100_000, i32::MIN, i32::MAX] {
            let mut w = WzWriter::new();
            w.compressed_i32(v);
            let mut r = WzReader::new(&w.buf, 0, 0);
            assert_eq!(r.compressed_i32().unwrap(), v, "{v}");
        }
    }

    /// **The image round trip, on real classic images.** parse -> owned -> serialise ->
    /// parse must give byte-identical JSON, and every canvas payload must come back
    /// unchanged. The serialised bytes are allowed to differ from the original (the packer
    /// dereferences repeated strings; this writer inlines them).
    #[test]
    fn classic_images_round_trip_through_owned() {
        let Some(ar) = classic("Character/Coat/Coat_000.wz") else { return };
        let Some(canvas) = classic("Character/Coat/_Canvas/_Canvas_000.wz") else { return };
        let mut checked = 0;
        for (a, want) in [(&ar, 3usize), (&canvas, 3usize)] {
            for node in a.root.children.iter().take(want) {
                let bytes = a.image_bytes(node).unwrap();
                let owned = Owned::parse(bytes).unwrap();
                let again = owned.serialize_image();
                let reparsed = Owned::parse(&again).unwrap();
                assert_eq!(owned, reparsed, "{}", node.name);
                assert_eq!(
                    to_json(&parse_image(bytes).unwrap(), 0),
                    to_json(&parse_image(&again).unwrap(), 0),
                    "{} JSON differs after the round trip",
                    node.name
                );
                checked += 1;
            }
        }
        assert_eq!(checked, 6);
        // And a canvas image really carried pixels through.
        let node = &canvas.root.children[0];
        let owned = Owned::parse(canvas.image_bytes(node).unwrap()).unwrap();
        assert!(owned.canvases().iter().any(|c| c.4 > 100), "no canvas payload survived");
    }

    /// A string table gains a name by path and reads back with it, everything else intact.
    #[test]
    fn set_path_adds_a_nested_string_leaf() {
        let mut root = Owned::Object(vec![(
            "ClassicWorld".into(),
            Owned::Object(vec![("Coat".into(), Owned::Object(vec![]))]),
        )]);
        root.set_path("ClassicWorld/Longcoat/1054555/name", Owned::String("Frieren's Clothes".into()));
        root.set_path("ClassicWorld/Coat/1040021/name", Owned::String("Blue Sergeant".into()));
        let bytes = root.serialize_image();
        let back = Owned::parse(&bytes).unwrap();
        let name = back.get("ClassicWorld").unwrap().get("Longcoat").unwrap().get("1054555").unwrap().get("name").unwrap();
        assert_eq!(name, &Owned::String("Frieren's Clothes".into()));
        assert!(back.get("ClassicWorld").unwrap().get("Coat").unwrap().get("1040021").is_some());
    }

    /// **An archive written here opens with the ordinary reader**, version-detects to 779,
    /// and hands back every image byte for byte - on real classic images.
    #[test]
    fn a_written_archive_opens_and_matches() {
        let Some(ar) = classic("Character/Accessory/Accessory_000.wz") else { return };
        let images = archive_images(&ar).unwrap();
        assert!(images.len() > 10);
        let bytes = write_archive(779, &images);
        verify_archive(&bytes, 779, &images).unwrap();

        let dir = std::env::temp_dir().join(format!("wz-writer-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("Accessory_000.wz");
        std::fs::write(&path, &bytes).unwrap();
        let again = Archive::open(&path).unwrap();
        assert_eq!(again.version, 779, "version detection must land on 779 from the bytes alone");
        assert_eq!(again.header.fstart, 0x3C);
        assert_eq!(again.header.fsize as usize, bytes.len() - 0x3C);
        assert_eq!(again.root.children.len(), images.len());
        for (node, img) in again.root.children.iter().zip(&images) {
            assert_eq!(node.name, img.name);
            assert_eq!(node.checksum, checksum(&img.bytes));
            assert_eq!(again.image_bytes(node).unwrap(), img.bytes.as_slice());
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Merge keeps base order, replaces by name, and appends new names sorted.
    #[test]
    fn merge_replaces_in_place_and_appends_sorted() {
        let e = |n: &str, b: &[u8]| ImageEntry { name: n.into(), bytes: b.to_vec() };
        let base = vec![e("b.img", b"1"), e("a.img", b"2"), e("c.img", b"3")];
        let add = vec![e("z.img", b"9"), e("a.img", b"22"), e("d.img", b"4")];
        let out = merge_images(base, add);
        let names: Vec<&str> = out.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names, ["b.img", "a.img", "c.img", "d.img", "z.img"]);
        assert_eq!(out[1].bytes, b"22");
    }
}
