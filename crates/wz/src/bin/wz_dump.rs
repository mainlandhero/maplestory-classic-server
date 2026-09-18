//! Inspect and extract WZ archives.
//!
//!   wz-dump info <archive.wz>            header + version + tree summary
//!   wz-dump tree <archive.wz> [depth]    directory listing
//!   wz-dump cat  <archive.wz> <img/path> parse one image to JSON
//!   wz-dump scan <Data dir>              open every tree, report health
//!   wz-dump verify <Data dir>            parse every image in every tree
//!   wz-dump build <out.wz> <version> <spec.tsv> [base.wz]
//!                                        write an archive: the base's images verbatim, then
//!                                        the spec's copies / merges / string patches
//!
//! `build` spec rows, tab separated:
//!
//!   copy    <image> <src.wz> <src image>
//!   merge   <image> <src.wz> <src image> <k[=k2],...>   top-level keys onto the base image
//!   inline  <image> <src.wz> <src image> <canvas.wz>    copy, with every outlinked canvas
//!                                                       pulled in from <canvas.wz> - the
//!                                                       classic PetEquip shape, no _Canvas
//!   patch   <image> <rows.tsv>     rows: path <TAB> int|str|uol|del|canvas <TAB> value
//!                                  (canvas: value is `w,h,format,<payload file>` - raw WZ
//!                                  pixel payload, zlib-compressed, as `wz-dump canvas` writes)
//!   strings <image> <rows.tsv>     rows: path <TAB> text

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use wz::{Archive, Node, NodeKind};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!(
            "usage:\n  \
             wz-dump info <archive.wz>\n  \
             wz-dump tree <archive.wz> [depth]\n  \
             wz-dump cat  <archive.wz> <image/path>\n  \
             wz-dump canvas <archive.wz> <image> <out dir> [node filter]\n  \
             wz-dump scan <Data dir>\n  \
             wz-dump verify <Data dir>\n  \
             wz-dump build <out.wz> <version> <spec.tsv> [base.wz]"
        );
        return ExitCode::FAILURE;
    }

    let result = match args[0].as_str() {
        "info" if args.len() >= 2 => cmd_info(Path::new(&args[1])),
        "tree" if args.len() >= 2 => {
            let depth = args.get(2).and_then(|d| d.parse().ok()).unwrap_or(2);
            cmd_tree(Path::new(&args[1]), depth)
        }
        "cat" if args.len() >= 3 => cmd_cat(Path::new(&args[1]), &args[2]),
        "canvas" if args.len() >= 4 => cmd_canvas(
            Path::new(&args[1]),
            &args[2],
            Path::new(&args[3]),
            args.get(4).map(|s| s.as_str()),
        ),
        "scan" if args.len() >= 2 => cmd_scan(Path::new(&args[1])),
        "verify" if args.len() >= 2 => cmd_verify(Path::new(&args[1])),
        "build" if args.len() >= 4 => cmd_build(
            Path::new(&args[1]),
            &args[2],
            Path::new(&args[3]),
            args.get(4).map(Path::new),
        ),
        other => {
            eprintln!("unknown or incomplete command: {other}");
            return ExitCode::FAILURE;
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cmd_info(path: &Path) -> wz::Result<()> {
    let ar = Archive::open(path)?;
    let (dirs, imgs) = ar.root.count_recursive();
    println!("file       {}", ar.path.display());
    println!("bytes      {}", ar.data.len());
    println!("copyright  {}", ar.header.copyright);
    println!("fstart     0x{:X}", ar.header.fstart);
    println!("fsize      {}", ar.header.fsize);
    println!("encVer     {}", ar.header.enc_ver);
    println!("version    {}", ar.version);
    println!("hash       0x{:08X}", ar.version_hash);
    println!("tree       {dirs} directories, {imgs} images");
    Ok(())
}

fn cmd_tree(path: &Path, depth: usize) -> wz::Result<()> {
    let ar = Archive::open(path)?;
    println!("{} (v{})", ar.root.name, ar.version);
    print_tree(&ar.root.children, 0, depth);
    Ok(())
}

fn print_tree(nodes: &[Node], level: usize, max: usize) {
    if level > max {
        return;
    }
    for n in nodes {
        let tag = if n.is_dir() { "DIR" } else { "IMG" };
        println!(
            "{}[{tag}] {:<28} size={:<9} off=0x{:X}",
            "  ".repeat(level + 1),
            n.name,
            n.size,
            n.offset
        );
        if n.is_dir() {
            print_tree(&n.children, level + 1, max);
        }
    }
}

fn cmd_cat(path: &Path, img_path: &str) -> wz::Result<()> {
    let ar = Archive::open(path)?;
    let Some(node) = ar.root.get(img_path) else {
        eprintln!("no such node: {img_path}");
        std::process::exit(1);
    };
    if node.kind != NodeKind::Image {
        eprintln!("{img_path} is a directory, not an image");
        std::process::exit(1);
    }
    let bytes = ar.image_bytes(node)?;
    let value = wz::parse_image(bytes)?;
    println!("{}", wz::to_json(&value, 0));
    Ok(())
}

/// Export every canvas payload in one image, plus a manifest describing them.
///
/// The payloads are written exactly as they sit in the archive - still compressed, not
/// decoded. Inflating them needs a zlib implementation, and this crate has no
/// dependencies; `tools/wz_png.py` takes it from here using Python's stdlib `zlib`.
///
/// A `--filter` substring keeps the output to the nodes actually being looked at: a single
/// UI image can hold thousands of canvases and tens of megabytes.
fn cmd_canvas(path: &Path, img_path: &str, out_dir: &Path, filter: Option<&str>) -> wz::Result<()> {
    let ar = Archive::open(path)?;
    let Some(node) = ar.root.get(img_path) else {
        eprintln!("no such node: {img_path}");
        std::process::exit(1);
    };
    let bytes = ar.image_bytes(node)?;
    let value = wz::parse_image(bytes)?;

    io(out_dir, std::fs::create_dir_all(out_dir))?;
    let mut entries: Vec<String> = Vec::new();
    let mut skipped = 0usize;
    collect_canvases(&value, String::new(), &mut |node_path, w, h, fmt, off, len| {
        if let Some(f) = filter {
            if !node_path.contains(f) {
                return Ok(());
            }
        }
        // An offset past the end means the parse and the payload disagree; say so rather
        // than writing a truncated file that looks like a decode failure later.
        if off + len > bytes.len() {
            eprintln!("warning: {node_path} payload runs past the image, skipping");
            skipped += 1;
            return Ok(());
        }
        let name = format!("{}.bin", node_path.trim_matches('/').replace('/', "."));
        let dest = out_dir.join(&name);
        io(&dest, std::fs::write(&dest, &bytes[off..off + len]))?;
        entries.push(format!(
            "  {{\"node\": {}, \"file\": {}, \"width\": {w}, \"height\": {h}, \
             \"format\": {fmt}, \"bytes\": {len}}}",
            json_str(node_path),
            json_str(&name)
        ));
        Ok(())
    })?;

    let manifest = format!("[\n{}\n]\n", entries.join(",\n"));
    let manifest_path = out_dir.join("manifest.json");
    io(&manifest_path, std::fs::write(&manifest_path, manifest))?;
    println!(
        "wrote {} canvases to {}{}",
        entries.len(),
        out_dir.display(),
        if skipped > 0 { format!(" ({skipped} skipped)") } else { String::new() }
    );
    Ok(())
}

/// Walk every `Canvas` in a parsed image, deepest-first path included.
fn collect_canvases(
    v: &wz::Value,
    path: String,
    f: &mut impl FnMut(&str, i32, i32, i32, usize, usize) -> wz::Result<()>,
) -> wz::Result<()> {
    if let wz::Value::Canvas { width, height, format, data_off, data_len, .. } = v {
        if *data_len > 0 {
            f(&path, *width, *height, *format, *data_off, *data_len)?;
        }
    }
    if let Some(children) = v.children() {
        for c in children {
            collect_canvases(&c.value, format!("{path}/{}", c.name), f)?;
        }
    }
    Ok(())
}

/// `WzError::Io` names the path it failed on, so io errors are attributed rather than
/// converted blindly - "io error" with no filename is useless when writing thousands.
/// `build`: assemble an archive from a base plus a spec. The spec is tab-separated, one
/// instruction per line, `#` comments allowed:
///
/// ```text
/// copy     <name>   <src.wz>   <src image>            the image, byte for byte
/// merge    <name>   <src.wz>   <src image>   <k1,k2>  top-level keys k1.. of the source
///                                                     image replace/append into the base's
///                                                     image of that name (empty if absent);
///                                                     `k=k2` takes source key k in as k2
/// strings  <name>   <patch.tsv>                       lines `path<TAB>value` set string
///                                                     leaves in the base's image of that name
/// patch    <name>   <patch.tsv>                       lines `path<TAB>kind<TAB>value`, kind
///                                                     `str`, `int` or `uol` (a link to a
///                                                     sibling path, the client's own way of
///                                                     saying "same as 30"); same base rule,
///                                                     and the base image may be absent
///                                                     (starts empty)
/// ```
///
/// **A `merge`/`strings`/`patch` row starts from the image an EARLIER row of this spec
/// produced, when there is one, and from the base otherwise.** Without that, `copy X` then
/// `patch X` did not layer: both pushed an addition named X, `merge_images` keeps the last,
/// and the copied image was silently replaced by an empty one holding only the patch. Found
/// 2026-09-12 wiring the weapon covers' per-type links onto the copied cover images.
///
/// Everything in the base not named by the spec is copied verbatim. The result is verified
/// against the reader before it is written, and re-opened after.
fn cmd_build(out: &Path, version: &str, spec: &Path, base: Option<&Path>) -> wz::Result<()> {
    use wz::writer::{archive_images, merge_images, verify_archive, ImageEntry, Owned};

    let version: u16 = version.parse().map_err(|_| wz::WzError::VersionNotFound { enc_ver: 0 })?;
    let base_ar = match base {
        Some(p) => Some(Archive::open(p)?),
        None => None,
    };
    let base_images = match &base_ar {
        Some(ar) => archive_images(ar)?,
        None => Vec::new(),
    };
    let base_image = |name: &str| -> Option<&[u8]> {
        base_images.iter().find(|i| i.name == name).map(|i| i.bytes.as_slice())
    };

    let text = io(spec, std::fs::read_to_string(spec))?;
    let mut additions: Vec<ImageEntry> = Vec::new();
    // The image a later row should start from: the newest addition of that name, else the
    // base's. `merge_images` keeps the last addition per name, so a row that starts from a
    // prior addition and pushes its result replaces it, which is the layering intended.
    let starting_image = |additions: &Vec<ImageEntry>, name: &str| -> Option<Vec<u8>> {
        additions
            .iter()
            .rev()
            .find(|i| i.name == name)
            .map(|i| i.bytes.clone())
            .or_else(|| base_image(name).map(|b| b.to_vec()))
    };
    let mut opened: std::collections::HashMap<PathBuf, Archive> = std::collections::HashMap::new();
    for (lineno, line) in text.lines().enumerate() {
        let line = line.trim_end_matches('\r');
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        let bad = || wz::WzError::BadEntryType { kind: 0xEE, offset: lineno + 1 };
        match f[0] {
            "copy" | "merge" => {
                if f.len() < 4 {
                    return Err(bad());
                }
                let (name, src, src_img) = (f[1], PathBuf::from(f[2]), f[3]);
                if !opened.contains_key(&src) {
                    opened.insert(src.clone(), Archive::open(&src)?);
                }
                let ar = &opened[&src];
                let Some(node) = ar.root.get(src_img) else {
                    eprintln!("line {}: {} has no image {src_img}", lineno + 1, src.display());
                    return Err(bad());
                };
                let src_bytes = ar.image_bytes(node)?;
                if f[0] == "copy" {
                    additions.push(ImageEntry { name: name.to_string(), bytes: src_bytes.to_vec() });
                    println!("copy    {name:<16} <- {}/{src_img} ({} bytes)", src.display(), src_bytes.len());
                } else {
                    if f.len() < 5 {
                        return Err(bad());
                    }
                    // `k` takes the source's top-level `k` as `k`; `k=k2` takes it as `k2`.
                    // The rename exists for the Signature Style Collection box: the classic
                    // client opens a Cash item on double-click by its id FAMILY, and 522
                    // (the modern box's) is not one it opens while 568 (the set coupons') is
                    // - so the box's node moves from 0522.img/05222221 to 0568.img/05681599,
                    // its outlinks still pointing at the 0522 canvas, which stays put.
                    let keys: Vec<(&str, &str)> = f[4]
                        .split(',')
                        .map(str::trim)
                        .filter(|k| !k.is_empty())
                        .map(|k| match k.split_once('=') {
                            Some((from, to)) => (from.trim(), to.trim()),
                            None => (k, k),
                        })
                        .collect();
                    let overlay = Owned::parse(src_bytes)?;
                    let start = starting_image(&additions, name);
                    let had_base = start.is_some();
                    let mut target = match &start {
                        Some(b) => Owned::parse(b)?,
                        None => Owned::Object(Vec::new()),
                    };
                    let mut taken = 0;
                    for (from, to) in &keys {
                        let Some(v) = overlay.get(from) else {
                            eprintln!("line {}: {src_img} has no top-level {from}", lineno + 1);
                            return Err(bad());
                        };
                        target.set(to, v.clone());
                        taken += 1;
                    }
                    let bytes = target.serialize_image();
                    println!(
                        "merge   {name:<16} <- {taken} key(s) of {}/{src_img} onto {} ({} bytes)",
                        src.display(),
                        if had_base { "the base image" } else { "an EMPTY image" },
                        bytes.len()
                    );
                    additions.push(ImageEntry { name: name.to_string(), bytes });
                }
            }
            "inline" => {
                // A `copy` whose outlinked canvases are resolved against <canvas.wz> and
                // written inline. For the trees the classic client keeps WITHOUT a
                // `_Canvas` archive - `Character/PetEquip` - a copied stub would point at
                // an image the client has no tree for and draw nothing. 2026-09-17, the
                // four Lil Frieren pet weapons.
                if f.len() < 5 {
                    return Err(bad());
                }
                let (name, src, src_img, canvas) = (f[1], PathBuf::from(f[2]), f[3], PathBuf::from(f[4]));
                for p in [&src, &canvas] {
                    if !opened.contains_key(p) {
                        opened.insert(p.clone(), Archive::open(p)?);
                    }
                }
                let ar = &opened[&src];
                let Some(node) = ar.root.get(src_img) else {
                    eprintln!("line {}: {} has no image {src_img}", lineno + 1, src.display());
                    return Err(bad());
                };
                let mut target = Owned::parse(ar.image_bytes(node)?)?;
                let canvas_ar = &opened[&canvas];
                let mut images: std::collections::HashMap<String, Owned> = std::collections::HashMap::new();
                let mut misses: Vec<String> = Vec::new();
                let filled = target.inline_outlinks(&mut |link: &str| {
                    // `Character/PetEquip/_Canvas/01802653.img/5002828/stand0/0`: the image
                    // is the segment ending in `.img`, the path is what follows it.
                    let Some(at) = link.find(".img/") else {
                        misses.push(link.to_string());
                        return Ok(None);
                    };
                    let (img_part, path) = link.split_at(at + 4);
                    let img_name = img_part.rsplit('/').next().unwrap_or(img_part).to_string();
                    if !images.contains_key(&img_name) {
                        let Some(n) = canvas_ar.root.get(&img_name) else {
                            misses.push(link.to_string());
                            return Ok(None);
                        };
                        images.insert(img_name.clone(), Owned::parse(canvas_ar.image_bytes(n)?)?);
                    }
                    let found = images[&img_name].get_path(path).cloned();
                    if found.is_none() {
                        misses.push(link.to_string());
                    }
                    Ok(found)
                });
                let filled = match filled {
                    Ok(n) => n,
                    Err(e) => {
                        for m in &misses {
                            eprintln!("line {}: {src_img}: outlink resolves to nothing in {}: {m}", lineno + 1, canvas.display());
                        }
                        return Err(e);
                    }
                };
                let bytes = target.serialize_image();
                println!(
                    "inline  {name:<16} <- {}/{src_img}, {filled} canvas(es) pulled in from {} ({} bytes)",
                    src.display(),
                    canvas.display(),
                    bytes.len()
                );
                additions.push(ImageEntry { name: name.to_string(), bytes });
            }
            "strings" | "patch" => {
                if f.len() < 3 {
                    return Err(bad());
                }
                let typed = f[0] == "patch";
                let (name, patch) = (f[1], Path::new(f[2]));
                let mut target = match starting_image(&additions, name) {
                    Some(b) => Owned::parse(&b)?,
                    None if typed => Owned::Object(Vec::new()),
                    None => {
                        eprintln!("line {}: neither the base nor an earlier row has an image {name} to patch", lineno + 1);
                        return Err(bad());
                    }
                };
                let mut n = 0;
                for pl in io(patch, std::fs::read_to_string(patch))?.lines() {
                    let pl = pl.trim_end_matches('\r');
                    if pl.trim().is_empty() || pl.starts_with('#') {
                        continue;
                    }
                    let cols: Vec<&str> = pl.split('\t').collect();
                    let (path, value) = if typed {
                        if cols.len() < 2 {
                            eprintln!("line {}: patch row needs path, kind[, value]: {pl:?}", lineno + 1);
                            return Err(bad());
                        }
                        // `del` takes a leaf AWAY. The modern pets carry `info/chatBalloon`
                        // and `info/nameTag` naming UI nodes the classic client lacks;
                        // rewriting them to some other number is still a lookup that fails.
                        if cols[1] == "del" {
                            if !target.remove_path(cols[0]) {
                                println!("        {name}: del {} - nothing there (fine)", cols[0]);
                            }
                            n += 1;
                            continue;
                        }
                        // `newcanvas`: a canvas node that was NOT there, with its pixels
                        // inline and an `origin` child - `w,h,format,<payload file>,ox,oy`.
                        // The hair and face icons (2026-09-18): this client's Hair and Face
                        // images carry no `info/icon`, so the Character Info ITEM tab could
                        // not list them; the installer renders one from the part's own
                        // canvases. Refuses to overwrite a node that exists - that is what
                        // `canvas` is for, and a silent replace would hide a wrong path.
                        // `w,h,format,<payload file>,ox,oy[,outlink]`: `ox,oy` may be `-,-` for a
                        // pixel node with NO origin (what every `_Canvas` archive node is), and
                        // `outlink` makes the node a 1x1 stub that borrows its pixels - the shape
                        // of every real icon: the stub with the origin in the property image,
                        // the pixels, originless, in `_Canvas`. The engine builds its canvas from
                        // the pixel node, so an origin put on an INLINE canvas moves the picture
                        // (2026-09-18 evening, the tooltip drew a face 32 px low).
                        if cols[1] == "newcanvas" {
                            let Some(cols2) = cols.get(2) else { return Err(bad()) };
                            let parts: Vec<&str> = cols2.splitn(7, ',').collect();
                            if parts.len() < 6 {
                                eprintln!("line {}: newcanvas row wants w,h,format,<payload file>,ox,oy[,outlink]: {pl:?}", lineno + 1);
                                return Err(bad());
                            }
                            let num = |i: usize| parts[i].trim().parse::<i32>().map_err(|_| bad());
                            let (w, h, fmt) = (num(0)?, num(1)?, num(2)?);
                            let origin = if parts[4].trim() == "-" { None } else { Some((num(4)?, num(5)?)) };
                            let payload = io(Path::new(parts[3].trim()), std::fs::read(parts[3].trim()))?;
                            if target.get_path(cols[0]).is_some() {
                                eprintln!("line {}: {name} already has a node at {} - use `canvas` to replace pixels", lineno + 1, cols[0]);
                                return Err(bad());
                            }
                            let mut children = Vec::new();
                            if let Some((ox, oy)) = origin {
                                children.push(("origin".to_string(), Owned::Vector(ox, oy)));
                            }
                            if let Some(link) = parts.get(6).map(|l| l.trim()).filter(|l| !l.is_empty()) {
                                children.push(("_outlink".to_string(), Owned::String(link.to_string())));
                            }
                            target.set_path(cols[0], Owned::Canvas { width: w, height: h, format: fmt, payload, children });
                            n += 1;
                            continue;
                        }
                        // `canvas`: new pixels for a canvas that is already there. The Petite
                        // pets' "P" badge, composited into the icon by the installer.
                        if cols[1] == "canvas" {
                            let Some(cols2) = cols.get(2) else { return Err(bad()) };
                            let parts: Vec<&str> = cols2.splitn(4, ',').collect();
                            if parts.len() != 4 {
                                eprintln!("line {}: canvas row wants w,h,format,<payload file>: {pl:?}", lineno + 1);
                                return Err(bad());
                            }
                            let (w, h, fmt) = (
                                parts[0].trim().parse::<i32>().map_err(|_| bad())?,
                                parts[1].trim().parse::<i32>().map_err(|_| bad())?,
                                parts[2].trim().parse::<i32>().map_err(|_| bad())?,
                            );
                            let payload = io(Path::new(parts[3].trim()), std::fs::read(parts[3].trim()))?;
                            if !target.replace_canvas_pixels(cols[0], w, h, fmt, payload) {
                                eprintln!("line {}: {name} has no canvas at {}", lineno + 1, cols[0]);
                                return Err(bad());
                            }
                            n += 1;
                            continue;
                        }
                        if cols.len() < 3 {
                            eprintln!("line {}: patch row needs path, kind, value: {pl:?}", lineno + 1);
                            return Err(bad());
                        }
                        let v = match cols[1] {
                            "int" => Owned::Int(cols[2].trim().parse().map_err(|_| bad())?),
                            "str" => Owned::String(cols[2].replace("\\n", "\n")),
                            // A UOL: the value is a path relative to the leaf's parent, e.g.
                            // `32<TAB>uol<TAB>30` makes weapon type 32 an alias of type 30 -
                            // exactly how the classic covers (01702001) spell 31/32/33.
                            "uol" => Owned::Uol(cols[2].trim().to_string()),
                            other => {
                                eprintln!("line {}: unknown leaf kind {other:?}", lineno + 1);
                                return Err(bad());
                            }
                        };
                        (cols[0], v)
                    } else {
                        if cols.len() < 2 {
                            return Err(bad());
                        }
                        // The patch file is one line per leaf, so a literal backslash-n in it
                        // stands for the newline the string tables carry.
                        (cols[0], Owned::String(cols[1].replace("\\n", "\n")))
                    };
                    target.set_path(path, value);
                    n += 1;
                }
                let bytes = target.serialize_image();
                println!("{:<7} {name:<16} <- {n} leaves from {} ({} bytes)", f[0], patch.display(), bytes.len());
                additions.push(ImageEntry { name: name.to_string(), bytes });
            }
            other => {
                eprintln!("line {}: unknown instruction {other:?}", lineno + 1);
                return Err(bad());
            }
        }
    }

    let images = merge_images(base_images, additions);
    let bytes = wz::write_archive(version, &images);
    verify_archive(&bytes, version, &images)?;
    if let Some(parent) = out.parent() {
        io(parent, std::fs::create_dir_all(parent))?;
    }
    io(out, std::fs::write(out, &bytes))?;
    // Re-open the file the way the client will, and count.
    let again = Archive::open(out)?;
    println!(
        "wrote {} ({} bytes): v{} {} image(s), reopened as v{} with {} image(s)",
        out.display(),
        bytes.len(),
        version,
        images.len(),
        again.version,
        again.root.children.len()
    );
    if again.version != version || again.root.children.len() != images.len() {
        return Err(wz::WzError::VersionNotFound { enc_ver: again.header.enc_ver });
    }
    Ok(())
}

fn io<T>(path: &Path, r: std::io::Result<T>) -> wz::Result<T> {
    r.map_err(|source| wz::WzError::Io { path: path.to_path_buf(), source })
}

fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Open every `<Tree>_000.wz` under a Data directory and report parse health.
fn cmd_scan(data_dir: &Path) -> wz::Result<()> {
    let mut archives: Vec<PathBuf> = Vec::new();
    collect_archives(data_dir, &mut archives);
    archives.sort();

    println!(
        "{:<34} {:>10} {:>8} {:>7} {:>7}  status",
        "archive", "bytes", "version", "dirs", "images"
    );
    let (mut ok, mut empty, mut failed) = (0, 0, 0);
    for path in &archives {
        let rel = path.strip_prefix(data_dir).unwrap_or(path);
        match Archive::open(path) {
            Ok(ar) => {
                let (dirs, imgs) = ar.root.count_recursive();
                // 63-byte stubs are legitimate: the tree's content lives elsewhere.
                let status = if dirs + imgs == 0 {
                    empty += 1;
                    "empty (stub)"
                } else {
                    ok += 1;
                    "ok"
                };
                println!(
                    "{:<34} {:>10} {:>8} {:>7} {:>7}  {}",
                    rel.display(),
                    ar.data.len(),
                    ar.version,
                    dirs,
                    imgs,
                    status
                );
            }
            Err(e) => {
                failed += 1;
                println!(
                    "{:<34} {:>10} {:>8} {:>7} {:>7}  FAILED: {}",
                    rel.display(),
                    "-",
                    "-",
                    "-",
                    "-",
                    e
                );
            }
        }
    }
    println!(
        "\n{ok} ok, {empty} empty stubs, {failed} failed, {} total",
        archives.len()
    );
    Ok(())
}

/// Deep check: parse every image in every archive. This is the real exercise of the
/// property deserializer, which is far more failure-prone than the directory reader.
fn cmd_verify(data_dir: &Path) -> wz::Result<()> {
    let mut archives: Vec<PathBuf> = Vec::new();
    collect_archives(data_dir, &mut archives);
    archives.sort();

    let (mut total, mut failed) = (0usize, 0usize);
    let mut failures: Vec<(String, String)> = Vec::new();
    let mut by_archive: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for path in &archives {
        let Ok(ar) = Archive::open(path) else { continue };
        let rel = path
            .strip_prefix(data_dir)
            .unwrap_or(path)
            .display()
            .to_string();

        let mut stack: Vec<(&Node, String)> = ar
            .root
            .children
            .iter()
            .map(|n| (n, n.name.clone()))
            .collect();

        while let Some((node, npath)) = stack.pop() {
            if node.is_dir() {
                for c in &node.children {
                    stack.push((c, format!("{npath}/{}", c.name)));
                }
                continue;
            }
            total += 1;
            let outcome = ar
                .image_bytes(node)
                .and_then(|b| wz::parse_image(b).map(|_| ()));
            if let Err(e) = outcome {
                failed += 1;
                *by_archive.entry(rel.clone()).or_default() += 1;
                if failures.len() < 25 {
                    failures.push((format!("{rel}:{npath}"), e.to_string()));
                }
            }
        }
    }

    for (where_, why) in &failures {
        println!("FAIL {where_}\n     {why}");
    }
    if !by_archive.is_empty() {
        println!("\nfailures by archive:");
        let mut rows: Vec<_> = by_archive.into_iter().collect();
        rows.sort_by_key(|r| std::cmp::Reverse(r.1));
        for (archive, n) in rows {
            println!("  {n:>6}  {archive}");
        }
    }
    let pct = if total > 0 {
        (total - failed) as f64 * 100.0 / total as f64
    } else {
        100.0
    };
    println!("\nparsed {total} images across {} archives", archives.len());
    println!("{} ok, {failed} failed ({pct:.3}% success)", total - failed);
    Ok(())
}

fn collect_archives(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            collect_archives(&p, out);
        } else if p
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.ends_with("_000.wz"))
        {
            out.push(p);
        }
    }
}
