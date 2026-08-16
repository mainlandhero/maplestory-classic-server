//! Inspect and extract WZ archives.
//!
//!   wz-dump info <archive.wz>            header + version + tree summary
//!   wz-dump tree <archive.wz> [depth]    directory listing
//!   wz-dump cat  <archive.wz> <img/path> parse one image to JSON
//!   wz-dump scan <Data dir>              open every tree, report health
//!   wz-dump verify <Data dir>            parse every image in every tree

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
             wz-dump scan <Data dir>"
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
