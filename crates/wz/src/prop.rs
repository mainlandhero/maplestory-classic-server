//! Image (`.img`) property deserialization.
//!
//! An image is a tree of named properties. Property *names* and string *values* use
//! offset-reference encoding relative to the start of the image.

use crate::error::{Result, WzError};
use crate::reader::WzReader;

/// Name tags: inline vs. dereference. Images and directories use different tag bytes.
const NAME_INLINE: &[u8] = &[0x00, 0x04, 0x73];
const NAME_DEREF: &[u8] = &[0x01, 0x1B];

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    String(String),
    /// `Shape2D#Vector2D`
    Vector(i32, i32),
    /// `UOL` — a symbolic link to another node.
    Uol(String),
    /// A nested property list.
    Object(Vec<Property>),
    /// `Canvas` — image data. Pixels are left unparsed here: decoding them needs an
    /// inflater, and this crate is deliberately dependency-free. What is kept is enough
    /// for someone else to do it — the metadata plus **where the payload starts**, as an
    /// offset into the image bytes that `Archive::image_bytes` returns.
    ///
    /// `wz-dump canvas` exports those slices; `tools/wz_png.py` turns them into PNGs.
    /// Reading the client's baked UI text is what that is for: a lot of this client's
    /// on-screen wording exists only as pixels, and is invisible to any string search.
    Canvas {
        width: i32,
        height: i32,
        format: i32,
        data_off: usize,
        data_len: usize,
        children: Vec<Property>,
    },
    /// `Sound_DX8`
    Sound { data_len: usize, duration: i32 },
    /// `Shape2D#Convex2D`
    Convex(Vec<Value>),
    /// `RawData` — opaque blob (embedded fonts, colour LUTs). Length-delimited by the
    /// enclosing extended block, so the payload is skipped rather than interpreted.
    RawData { data_len: usize },
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) | Value::Uol(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::Short(v) => Some(v as i64),
            Value::Int(v) => Some(v as i64),
            Value::Long(v) => Some(v),
            Value::Float(v) => Some(v as i64),
            Value::Double(v) => Some(v as i64),
            _ => None,
        }
    }

    pub fn children(&self) -> Option<&[Property]> {
        match self {
            Value::Object(c) | Value::Canvas { children: c, .. } => Some(c),
            _ => None,
        }
    }

    /// Walk to a descendant by `/`-separated path.
    pub fn get(&self, path: &str) -> Option<&Value> {
        let mut cur = self;
        for part in path.split('/').filter(|p| !p.is_empty()) {
            let kids = cur.children()?;
            cur = &kids.iter().find(|p| p.name == part)?.value;
        }
        Some(cur)
    }
}

/// Parse a whole image. `data` must be exactly the image's bytes; offsets inside
/// are relative to its start.
pub fn parse_image(data: &[u8]) -> Result<Value> {
    let mut r = WzReader::new(data, 0, 0);
    // Images begin with a name-encoded type string, normally "Property".
    let tag = r.u8()?;
    if tag == 0x73 || tag == 0x1B || tag == 0x01 || tag == 0x00 {
        r.seek(0);
        let ty = r.string_at(0, NAME_INLINE, NAME_DEREF)?;
        if ty != "Property" {
            // Some images are a bare extended object; re-read as one.
            r.seek(0);
            return read_extended(&mut r, 0, data.len());
        }
    } else {
        r.seek(0);
        return read_extended(&mut r, 0, data.len());
    }
    // u16 reserved, then the property list.
    r.u16()?;
    Ok(Value::Object(read_property_list(&mut r, 0)?))
}

fn read_property_list(r: &mut WzReader, base: usize) -> Result<Vec<Property>> {
    let count = r.compressed_i32()?;
    if !(0..=500_000).contains(&count) {
        return Err(WzError::BadPropertyType {
            kind: 0xFF,
            offset: r.pos,
        });
    }
    let mut out = Vec::with_capacity(count.min(1024) as usize);
    for _ in 0..count {
        let name = r.string_at(base, NAME_INLINE, NAME_DEREF)?;
        let value = read_value(r, base)?;
        out.push(Property { name, value });
    }
    Ok(out)
}

fn read_value(r: &mut WzReader, base: usize) -> Result<Value> {
    let start = r.pos;
    let kind = r.u8()?;
    Ok(match kind {
        0x00 => Value::Null,
        0x02 | 0x0B => Value::Short(r.i16()?),
        0x03 | 0x13 => Value::Int(r.compressed_i32()?),
        0x14 => Value::Long(r.compressed_i64()?),
        0x04 => Value::Float(r.compressed_f32()?),
        0x05 => Value::Double(r.f64()?),
        0x08 => Value::String(r.string_at(base, NAME_INLINE, NAME_DEREF)?),
        0x09 => {
            let len = r.u32()? as usize;
            let end = r.pos + len;
            let v = read_extended(r, base, end)?;
            r.seek(end); // extended blocks are length-delimited; trust the length
            v
        }
        other => {
            return Err(WzError::BadPropertyType {
                kind: other,
                offset: start,
            })
        }
    })
}

fn read_extended(r: &mut WzReader, base: usize, end: usize) -> Result<Value> {
    let start = r.pos;
    let ty = r.string_at(base, NAME_INLINE, NAME_DEREF)?;
    Ok(match ty.as_str() {
        "Property" => {
            r.u16()?;
            Value::Object(read_property_list(r, base)?)
        }
        "Canvas" => {
            r.u8()?; // unused
            let has_children = r.u8()? == 1;
            let children = if has_children {
                r.u16()?;
                read_property_list(r, base)?
            } else {
                Vec::new()
            };
            let width = r.compressed_i32()?;
            let height = r.compressed_i32()?;
            let format = r.compressed_i32()? + r.u8()? as i32;
            r.u32()?; // unused
            // The declared length counts a leading byte that is not part of the payload,
            // so the pixels start one byte after this field and run `data_len` bytes.
            let data_len = r.i32()?.saturating_sub(1).max(0) as usize;
            let data_off = r.pos + 1;
            Value::Canvas { width, height, format, data_off, data_len, children }
        }
        "Shape2D#Vector2D" => Value::Vector(r.compressed_i32()?, r.compressed_i32()?),
        "Shape2D#Convex2D" => {
            let n = r.compressed_i32()?.max(0);
            let mut items = Vec::with_capacity(n.min(1024) as usize);
            for _ in 0..n {
                items.push(read_extended(r, base, end)?);
            }
            Value::Convex(items)
        }
        "Sound_DX8" => {
            r.u8()?;
            let data_len = r.compressed_i32()?.max(0) as usize;
            let duration = r.compressed_i32()?;
            Value::Sound { data_len, duration }
        }
        "UOL" => {
            r.u8()?;
            Value::Uol(r.string_at(base, NAME_INLINE, NAME_DEREF)?)
        }
        "RawData" => {
            // The caller bounds this block, so consume whatever is left of it.
            let data_len = end.saturating_sub(r.pos);
            r.seek(end.min(r.data.len()));
            Value::RawData { data_len }
        }
        _ => {
            return Err(WzError::BadExtendedType {
                name: ty,
                offset: start,
            })
        }
    })
}

/// Render a value as JSON. Canvas pixel data and sound payloads are summarized,
/// not embedded.
pub fn to_json(v: &Value, indent: usize) -> String {
    let pad = "  ".repeat(indent);
    let pad2 = "  ".repeat(indent + 1);
    match v {
        Value::Null => "null".into(),
        Value::Short(x) => x.to_string(),
        Value::Int(x) => x.to_string(),
        Value::Long(x) => x.to_string(),
        Value::Float(x) => fmt_num(*x as f64),
        Value::Double(x) => fmt_num(*x),
        Value::String(s) | Value::Uol(s) => json_str(s),
        Value::Vector(x, y) => format!("{{ \"x\": {x}, \"y\": {y} }}"),
        Value::Sound { data_len, duration } => {
            format!("{{ \"_sound\": true, \"bytes\": {data_len}, \"duration\": {duration} }}")
        }
        Value::RawData { data_len } => {
            format!("{{ \"_rawdata\": true, \"bytes\": {data_len} }}")
        }
        Value::Convex(items) => {
            let inner: Vec<String> = items.iter().map(|i| to_json(i, indent + 1)).collect();
            format!("[{}]", inner.join(", "))
        }
        Value::Canvas { width, height, format, data_len, children, data_off: _ } => {
            let mut parts = vec![
                format!("{pad2}\"_canvas\": true"),
                format!("{pad2}\"width\": {width}"),
                format!("{pad2}\"height\": {height}"),
                format!("{pad2}\"format\": {format}"),
                format!("{pad2}\"bytes\": {data_len}"),
            ];
            for c in children {
                parts.push(format!(
                    "{pad2}{}: {}",
                    json_str(&c.name),
                    to_json(&c.value, indent + 1)
                ));
            }
            format!("{{\n{}\n{pad}}}", parts.join(",\n"))
        }
        Value::Object(props) => {
            if props.is_empty() {
                return "{}".into();
            }
            let inner: Vec<String> = props
                .iter()
                .map(|p| {
                    format!(
                        "{pad2}{}: {}",
                        json_str(&p.name),
                        to_json(&p.value, indent + 1)
                    )
                })
                .collect();
            format!("{{\n{}\n{pad}}}", inner.join(",\n"))
        }
    }
}

fn fmt_num(x: f64) -> String {
    if x.is_finite() {
        let s = x.to_string();
        if s.contains('.') || s.contains('e') {
            s
        } else {
            format!("{s}.0")
        }
    } else {
        "null".into()
    }
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
