//! Is this copy of the client complete, and does it stand on its own?
//!
//! The owner, 2026-09-13, after a second machine showed *"Nexon Launcher failed to load"* and died
//! before the game window: *"this involves shipping a separate file to Joanne's computer, this
//! is not okay. We need to ship a check integrity function in the MapleCW Launcher."* And,
//! about what the check is for: *"our client should still work if there's no `C:\Nexon` folder
//! ... We should not have any dependencies on the actual install of MapleStory."*
//!
//! So this runs inside the launcher, on every **Start Game**, and its findings land in the
//! launcher's own log - the thing a person on another machine can already copy and paste. No
//! second executable, no script, no Python.
//!
//! # What it measures
//!
//! It reads the PE import tables - normal **and** delay-load - of every `.exe` and `.dll` in
//! the client directory, follows them transitively through the folder's own modules, and
//! resolves every imported name against the search path the client will really use:
//!
//! ```text
//! the client's own directory, then System32, then PATH
//! ```
//!
//! Two things come out of that, and they are different findings:
//!
//! * **missing** - imported by a module in the folder and resolvable nowhere. The client
//!   cannot start, and the loader's own error names no file.
//! * **borrowed** - resolvable, but from neither the folder nor Windows, so it was picked up
//!   off *this machine's* PATH. This is the finding that is invisible on the machine that
//!   works: a module borrowed from a real Nexon install here is simply absent there, and the
//!   folder looks identical on both.
//!
//! Nothing is loaded and nothing is executed: this reads bytes. That matters because the
//! folder holds an anti-cheat module and a repacked executable, and `LoadLibrary`-ing those
//! from the launcher would be measuring a different process than the one that fails.
//!
//! # What a clean result does NOT prove
//!
//! A DLL can be present and still fail to load - wrong architecture, blocked by antivirus, a
//! `DllMain` that returns false, or a delay-load that only fails when first called. Worse,
//! **this cannot see a runtime dependency at all**: `nmconew64.dll` reads
//! `HKLM\SOFTWARE\Nexon\NexonPlug\RootPath` and shells out to an `NMService` executable, and
//! no import table mentions either. (the owner's machine has no such key and the client runs, so
//! that particular path is not required - but it is the shape of thing this check is blind
//! to.) The report says so in its own words rather than claiming the folder is fine.
//!
//! # Why it reports and does not refuse
//!
//! Every other step in [`crate::prepare`] can refuse. This one cannot, deliberately: a false
//! positive here would block a launch on a machine where the client works, which costs more
//! than a false negative - the client is about to fail loudly anyway, and the point of this
//! module is to put a *file name* next to that failure.
//!
//! # Cost
//!
//! Top-level `.exe` and `.dll` only, which is what the loader searches for the executable's
//! own graph; `NxOverlay\` is a CEF subprocess with its own directory and is not on it. About
//! 110 MB is read on this client, once, at Start Game.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::prepare::Level;

/// Names the loader resolves from its own tables rather than from disk. A file by one of
/// these names is not expected to exist and its absence means nothing.
const APISET_PREFIXES: [&str; 2] = ["api-ms-win-", "ext-ms-"];

/// Refuse to read anything larger than this into memory. The client's own executable is 76 MB;
/// this is a guard against a stray disk image in the folder, not a real limit.
const MAX_MODULE_BYTES: u64 = 256 * 1024 * 1024;

/// One import that resolved, but not from anywhere that will exist on another machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Borrowed {
    pub found_at: PathBuf,
    pub importers: BTreeSet<String>,
}

/// What [`check`] found. Rendered by [`Report::lines`]; the fields are public so a test can
/// assert on the finding rather than on its wording.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Report {
    /// How many modules were parsed. A count of 0 or 1 means the walk did not happen, which
    /// is itself a finding - see [`Report::lines`].
    pub modules: usize,
    /// Imported by something in the folder, resolvable nowhere. Name -> who imported it.
    pub missing: BTreeMap<String, BTreeSet<String>>,
    /// Resolved off this machine rather than out of the folder. Name -> where, and who.
    pub borrowed: BTreeMap<String, Borrowed>,
    /// Files that could not be parsed as a 64-bit PE, with the reason. A 32-bit module in a
    /// 64-bit client's folder is a real finding and reaches the log through here.
    pub unreadable: Vec<(String, String)>,
}

impl Report {
    /// Nothing missing and nothing borrowed. Says nothing about whether the client will run.
    pub fn is_clean(&self) -> bool {
        self.missing.is_empty() && self.borrowed.is_empty()
    }

    /// The report as launcher log lines, in the order they should be read.
    ///
    /// A clean result is **one** line: this runs on every launch, and a check that prints a
    /// paragraph when it has nothing to say trains people to skip it. A finding is verbose,
    /// because it is going to be read once, by someone who cannot see the machine.
    pub fn lines(&self) -> Vec<(Level, String)> {
        let mut out = Vec::new();

        if self.modules == 0 {
            out.push((
                Level::Warn,
                "client integrity: nothing to check - no .exe or .dll in the client folder. \
                 That is not a pass; the check never ran."
                    .to_string(),
            ));
            return out;
        }

        for (name, why) in &self.unreadable {
            out.push((
                Level::Warn,
                format!("client integrity: {name} could not be read as a 64-bit module ({why})"),
            ));
        }

        for (name, who) in &self.missing {
            out.push((
                Level::Error,
                format!(
                    "client integrity: {name} IS MISSING - imported by {}, and it is not in the \
                     client folder, in System32, or on PATH. The client will fail to start and \
                     Windows will not name the file.",
                    joined(who)
                ),
            ));
        }

        for (name, b) in &self.borrowed {
            out.push((
                Level::Warn,
                format!(
                    "client integrity: {name} came from {} - not the client folder and not \
                     Windows, so it is borrowed from THIS machine and will be absent on one \
                     that never installed the game. Imported by {}.",
                    b.found_at.display(),
                    joined(&b.importers)
                ),
            ));
        }

        if self.is_clean() {
            out.push((
                Level::Good,
                format!(
                    "client integrity: {} modules, every imported DLL resolves from the folder \
                     or from Windows, nothing borrowed from this machine. (Imports only - a \
                     module can still be blocked by antivirus, and a runtime LoadLibrary or a \
                     registry lookup is invisible here.)",
                    self.modules
                ),
            ));
        } else {
            out.push((
                Level::Info,
                format!(
                    "client integrity: {} modules read, {} missing, {} borrowed",
                    self.modules,
                    self.missing.len(),
                    self.borrowed.len()
                ),
            ));
        }
        out
    }

    /// The same thing as plain text, for `--check-client`.
    pub fn text(&self) -> String {
        self.lines()
            .into_iter()
            .map(|(level, line)| {
                let tag = match level {
                    Level::Error => "ERROR",
                    Level::Warn => "WARN ",
                    Level::Good => "ok   ",
                    Level::Info => "     ",
                };
                format!("{tag} {line}")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn joined(set: &BTreeSet<String>) -> String {
    set.iter().cloned().collect::<Vec<_>>().join(", ")
}

/// Check the folder against the real machine.
pub fn check(client_dir: &Path) -> Report {
    let system_root = PathBuf::from(
        std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string()),
    );
    let mut dirs = vec![client_dir.to_path_buf(), system_root.join("System32")];
    if let Ok(path) = std::env::var("PATH") {
        dirs.extend(std::env::split_paths(&path));
    }
    check_with(client_dir, &dirs, &system_root)
}

/// The whole check, with the search path and the Windows directory handed in.
///
/// Split out for the tests: "borrowed from this machine" is only testable if the test gets to
/// say what "this machine" is, and a test that mutates `PATH` in a process running other tests
/// in parallel is a race, not a test.
pub fn check_with(client_dir: &Path, search_dirs: &[PathBuf], system_root: &Path) -> Report {
    let mut report = Report::default();

    let mut pending: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(client_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && has_module_extension(&path) {
                pending.push(path);
            }
        }
    }
    pending.sort();

    let mut seen: BTreeSet<String> = BTreeSet::new();
    while let Some(path) = pending.pop() {
        let key = key_of(&path);
        if !seen.insert(key) {
            continue;
        }
        let name = file_name(&path);
        let imports = match read_imports(&path) {
            Ok(imports) => imports,
            Err(why) => {
                report.unreadable.push((name, why));
                continue;
            }
        };
        report.modules += 1;

        for dep in imports {
            let lower = dep.to_ascii_lowercase();
            if APISET_PREFIXES.iter().any(|p| lower.starts_with(p)) {
                continue;
            }
            match resolve(&dep, search_dirs) {
                None => {
                    report.missing.entry(lower).or_default().insert(name.clone());
                }
                Some(found) if is_under(&found, client_dir) => {
                    // Recurse only into the folder's own modules. An earlier version of this
                    // walk followed System32's graph too and came back claiming
                    // `hvsifiletrust.dll` and `pdmutilities.dll` were missing on the machine
                    // where the client runs perfectly - ordinary optional imports of SHELL32
                    // and a printing module, resolved by servicing paths this does not model.
                    // An instrument that reports two misses on the known-good control cannot
                    // report a miss on the broken one, so the walk stops at the OS boundary.
                    pending.push(found);
                }
                Some(found) if !is_under(&found, system_root) => {
                    report
                        .borrowed
                        .entry(lower)
                        .or_insert_with(|| Borrowed {
                            found_at: found,
                            importers: BTreeSet::new(),
                        })
                        .importers
                        .insert(name.clone());
                }
                Some(_) => {}
            }
        }
    }

    report
}

fn has_module_extension(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => ext.eq_ignore_ascii_case("exe") || ext.eq_ignore_ascii_case("dll"),
        None => false,
    }
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

fn key_of(path: &Path) -> String {
    std::fs::canonicalize(path)
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .to_ascii_lowercase()
}

fn is_under(path: &Path, root: &Path) -> bool {
    let (Ok(path), Ok(root)) = (std::fs::canonicalize(path), std::fs::canonicalize(root)) else {
        return path.starts_with(root);
    };
    path.starts_with(root)
}

fn resolve(name: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    for dir in dirs {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

// ---------------------------------------------------------------------------------------
// The PE reader. Enough of PE32+ to answer one question, and no more.
// ---------------------------------------------------------------------------------------

/// Every DLL name a module imports: the import directory and the delay-load directory.
fn read_imports(path: &Path) -> Result<BTreeSet<String>, String> {
    let size = std::fs::metadata(path).map_err(|e| e.to_string())?.len();
    if size > MAX_MODULE_BYTES {
        return Err(format!("{size} bytes, larger than the {MAX_MODULE_BYTES} byte limit"));
    }
    let data = std::fs::read(path).map_err(|e| e.to_string())?;
    Pe::parse(&data)?.imports()
}

struct Pe<'a> {
    data: &'a [u8],
    /// `(virtual address, virtual size, raw offset, raw size)` per section.
    sections: Vec<(u32, u32, u32, u32)>,
    /// The data directories, `(rva, size)`.
    dirs: Vec<(u32, u32)>,
}

impl<'a> Pe<'a> {
    fn parse(data: &'a [u8]) -> Result<Pe<'a>, String> {
        if data.len() < 0x40 || &data[..2] != b"MZ" {
            return Err("no MZ header".into());
        }
        let pe = u32at(data, 0x3C).ok_or("truncated DOS header")? as usize;
        if data.len() < pe + 24 || &data[pe..pe + 4] != b"PE\0\0" {
            return Err("no PE signature".into());
        }
        let nsections = u16at(data, pe + 6).ok_or("truncated COFF header")? as usize;
        let optsize = u16at(data, pe + 20).ok_or("truncated COFF header")? as usize;
        let optoff = pe + 24;
        match u16at(data, optoff) {
            Some(0x20B) => {}
            Some(0x10B) => return Err("32-bit (PE32), and this client is 64-bit".into()),
            Some(other) => return Err(format!("unknown optional header magic 0x{other:x}")),
            None => return Err("truncated optional header".into()),
        }

        let ndirs = u32at(data, optoff + 108).ok_or("truncated optional header")? as usize;
        let mut dirs = Vec::with_capacity(ndirs.min(16));
        for i in 0..ndirs.min(16) {
            let at = optoff + 112 + i * 8;
            let (Some(rva), Some(size)) = (u32at(data, at), u32at(data, at + 4)) else {
                break;
            };
            dirs.push((rva, size));
        }

        let mut sections = Vec::with_capacity(nsections);
        for i in 0..nsections {
            let at = optoff + optsize + i * 40;
            let (Some(vsize), Some(va), Some(rsize), Some(raw)) = (
                u32at(data, at + 8),
                u32at(data, at + 12),
                u32at(data, at + 16),
                u32at(data, at + 20),
            ) else {
                break;
            };
            sections.push((va, vsize.max(rsize), raw, rsize));
        }

        Ok(Pe { data, sections, dirs })
    }

    /// A relative virtual address as an offset into the file, or `None` when it falls in a
    /// section's virtual tail - the part that exists only once the loader has mapped it.
    fn off(&self, rva: u32) -> Option<usize> {
        for &(va, vsize, raw, rsize) in &self.sections {
            if rva >= va && rva < va.saturating_add(vsize) {
                let delta = rva - va;
                if delta >= rsize {
                    return None;
                }
                return Some(raw as usize + delta as usize);
            }
        }
        None
    }

    fn cstr(&self, at: usize) -> Option<String> {
        let rest = self.data.get(at..)?;
        let end = rest.iter().position(|&b| b == 0)?;
        std::str::from_utf8(&rest[..end]).ok().map(|s| s.to_string())
    }

    fn imports(&self) -> Result<BTreeSet<String>, String> {
        let mut names = BTreeSet::new();

        // Directory 1: import descriptors, 20 bytes each, DLL name RVA at +12, terminated by
        // an all-zero entry.
        if let Some(&(rva, _)) = self.dirs.get(1) {
            if rva != 0 {
                if let Some(base) = self.off(rva) {
                    for i in 0.. {
                        let at = base + i * 20;
                        let Some(entry) = self.data.get(at..at + 20) else { break };
                        if entry.iter().all(|&b| b == 0) {
                            break;
                        }
                        let Some(name_rva) = u32at(self.data, at + 12) else { break };
                        if let Some(name) = self.off(name_rva).and_then(|o| self.cstr(o)) {
                            names.insert(name);
                        }
                    }
                }
            }
        }

        // Directory 13: delay-load descriptors, 32 bytes each, DLL name RVA at +4. Bit 0 of
        // the attributes at +0 says the addresses really are RVAs; the old linkers that wrote
        // absolute VAs there are not worth decoding for a name we would only log.
        if let Some(&(rva, _)) = self.dirs.get(13) {
            if rva != 0 {
                if let Some(base) = self.off(rva) {
                    for i in 0.. {
                        let at = base + i * 32;
                        let Some(entry) = self.data.get(at..at + 32) else { break };
                        if entry.iter().all(|&b| b == 0) {
                            break;
                        }
                        let (Some(attrs), Some(name_rva)) =
                            (u32at(self.data, at), u32at(self.data, at + 4))
                        else {
                            break;
                        };
                        if attrs & 1 == 0 {
                            continue;
                        }
                        if let Some(name) = self.off(name_rva).and_then(|o| self.cstr(o)) {
                            names.insert(name);
                        }
                    }
                }
            }
        }

        Ok(names)
    }
}

fn u16at(data: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(data.get(at..at + 2)?.try_into().ok()?))
}

fn u32at(data: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(data.get(at..at + 4)?.try_into().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal 64-bit PE that imports the names it is given.
    ///
    /// Hand-built rather than pulled from a crate on purpose: the thing under test is a PE
    /// reader, and generating the input with a second implementation of the same format would
    /// leave both of them agreeing about a layout neither had checked against Windows. This
    /// one is checked against Windows - `the_real_client_folder_is_self_contained` reads the
    /// actual client.
    fn fake_pe(imports: &[&str]) -> Vec<u8> {
        const PE_AT: usize = 0x80;
        const OPT_SIZE: usize = 240;
        const SECTION_RVA: u32 = 0x1000;
        const SECTION_RAW: usize = 0x400;

        // The section's contents: descriptors, then the names they point at.
        let descriptors = (imports.len() + 1) * 20;
        let mut section = vec![0u8; descriptors];
        let mut name_rvas = Vec::new();
        for name in imports {
            name_rvas.push(SECTION_RVA + section.len() as u32);
            section.extend_from_slice(name.as_bytes());
            section.push(0);
        }
        for (i, rva) in name_rvas.iter().enumerate() {
            section[i * 20 + 12..i * 20 + 16].copy_from_slice(&rva.to_le_bytes());
        }

        let mut data = vec![0u8; SECTION_RAW];
        data[..2].copy_from_slice(b"MZ");
        data[0x3C..0x40].copy_from_slice(&(PE_AT as u32).to_le_bytes());
        data[PE_AT..PE_AT + 4].copy_from_slice(b"PE\0\0");
        let coff = PE_AT + 4;
        data[coff..coff + 2].copy_from_slice(&0x8664u16.to_le_bytes()); // AMD64
        data[coff + 2..coff + 4].copy_from_slice(&1u16.to_le_bytes()); // one section
        data[coff + 16..coff + 18].copy_from_slice(&(OPT_SIZE as u16).to_le_bytes());

        let opt = PE_AT + 24;
        data[opt..opt + 2].copy_from_slice(&0x20Bu16.to_le_bytes()); // PE32+
        data[opt + 108..opt + 112].copy_from_slice(&16u32.to_le_bytes()); // 16 data dirs
        // Data directory 1 = imports.
        data[opt + 112 + 8..opt + 112 + 12].copy_from_slice(&SECTION_RVA.to_le_bytes());
        data[opt + 112 + 12..opt + 112 + 16]
            .copy_from_slice(&(descriptors as u32).to_le_bytes());

        let sec = opt + OPT_SIZE;
        data[sec..sec + 6].copy_from_slice(b".idata");
        data[sec + 8..sec + 12].copy_from_slice(&(section.len() as u32).to_le_bytes());
        data[sec + 12..sec + 16].copy_from_slice(&SECTION_RVA.to_le_bytes());
        data[sec + 16..sec + 20].copy_from_slice(&(section.len() as u32).to_le_bytes());
        data[sec + 20..sec + 24].copy_from_slice(&(SECTION_RAW as u32).to_le_bytes());

        data.extend_from_slice(&section);
        data
    }

    fn write(dir: &Path, name: &str, bytes: &[u8]) {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(dir.join(name), bytes).unwrap();
    }

    /// A temp directory that cleans itself up, so the tests need no dev-dependency.
    struct Temp(PathBuf);
    impl Temp {
        fn new(tag: &str) -> Temp {
            let base = std::env::temp_dir().join(format!(
                "maplecw-integrity-{tag}-{}-{:?}",
                std::process::id(),
                std::thread::current().id()
            ));
            let _ = std::fs::remove_dir_all(&base);
            std::fs::create_dir_all(&base).unwrap();
            Temp(base)
        }
        fn path(&self) -> &Path {
            &self.0
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn the_import_table_of_a_module_is_read_back() {
        let temp = Temp::new("read");
        write(temp.path(), "A.dll", &fake_pe(&["B.dll", "KERNEL32.dll"]));
        let imports = read_imports(&temp.path().join("A.dll")).unwrap();
        assert_eq!(
            imports.into_iter().collect::<Vec<_>>(),
            vec!["B.dll".to_string(), "KERNEL32.dll".to_string()]
        );
    }

    /// **The positive control.** `CLAUDE.md`: prove the instrument can find a positive before
    /// believing that it found nothing. A clean report from a checker that cannot report a
    /// miss is the exact failure this project keeps paying for.
    #[test]
    fn a_folder_missing_one_dll_names_it_and_names_who_wanted_it() {
        let temp = Temp::new("missing");
        let client = temp.path().join("client");
        let system = temp.path().join("Windows");
        write(&client, "MapleStory.exe", &fake_pe(&["Canvas.dll"]));
        write(&system.join("System32"), "KERNEL32.dll", &fake_pe(&[]));

        let dirs = vec![client.clone(), system.join("System32")];
        let report = check_with(&client, &dirs, &system);

        assert_eq!(report.modules, 1);
        assert_eq!(report.missing.len(), 1, "{report:?}");
        let who = &report.missing["canvas.dll"];
        assert!(who.contains("MapleStory.exe"), "{who:?}");
        assert!(!report.is_clean());
        assert!(report.text().contains("IS MISSING"), "{}", report.text());
    }

    #[test]
    fn a_complete_folder_is_clean_and_the_walk_follows_it_transitively() {
        let temp = Temp::new("complete");
        let client = temp.path().join("client");
        let system = temp.path().join("Windows");
        // The exe reaches ZLZ64 only through Canvas, so a clean result here also proves the
        // walk recursed rather than stopping at the roots.
        write(&client, "MapleStory.exe", &fake_pe(&["Canvas.dll", "KERNEL32.dll"]));
        write(&client, "Canvas.dll", &fake_pe(&["ZLZ64.dll"]));
        write(&client, "ZLZ64.dll", &fake_pe(&["KERNEL32.dll"]));
        write(&system.join("System32"), "KERNEL32.dll", &fake_pe(&[]));

        let dirs = vec![client.clone(), system.join("System32")];
        let report = check_with(&client, &dirs, &system);

        assert!(report.is_clean(), "{report:?}");
        assert_eq!(report.modules, 3, "all three folder modules were parsed");
    }

    /// The finding that is invisible on the machine that works.
    #[test]
    fn a_dll_found_only_on_this_machines_path_is_reported_as_borrowed() {
        let temp = Temp::new("borrowed");
        let client = temp.path().join("client");
        let system = temp.path().join("Windows");
        let nexon = temp.path().join("Nexon").join("Library");
        write(&client, "MapleStory.exe", &fake_pe(&["nexon_x64.dll"]));
        write(&nexon, "nexon_x64.dll", &fake_pe(&[]));
        write(&system.join("System32"), "KERNEL32.dll", &fake_pe(&[]));

        let dirs = vec![client.clone(), system.join("System32"), nexon.clone()];
        let report = check_with(&client, &dirs, &system);

        assert!(report.missing.is_empty(), "it resolved: {report:?}");
        assert!(!report.is_clean(), "but not from anywhere that ships");
        let borrowed = &report.borrowed["nexon_x64.dll"];
        assert_eq!(borrowed.found_at, nexon.join("nexon_x64.dll"));
        assert!(report.text().contains("borrowed from THIS machine"));
    }

    /// System32 is not borrowed, and the walk does not follow it.
    #[test]
    fn windows_own_modules_are_neither_missing_nor_borrowed() {
        let temp = Temp::new("system");
        let client = temp.path().join("client");
        let system = temp.path().join("Windows");
        write(&client, "MapleStory.exe", &fake_pe(&["USER32.dll"]));
        // USER32 imports something that exists nowhere. It is Windows' module, so its graph
        // is not ours to walk - the earlier version of this walk reported exactly this shape
        // of miss on a machine where the client runs.
        write(&system.join("System32"), "USER32.dll", &fake_pe(&["hvsifiletrust.dll"]));

        let dirs = vec![client.clone(), system.join("System32")];
        let report = check_with(&client, &dirs, &system);

        assert!(report.is_clean(), "{report:?}");
        assert_eq!(report.modules, 1, "only the client's own module was parsed");
    }

    #[test]
    fn api_set_names_are_not_files_and_are_not_missing() {
        let temp = Temp::new("apiset");
        let client = temp.path().join("client");
        let system = temp.path().join("Windows");
        write(
            &client,
            "HybridCore64.dll",
            &fake_pe(&["api-ms-win-crt-heap-l1-1-0.dll", "ext-ms-win-x.dll"]),
        );
        let dirs = vec![client.clone(), system.join("System32")];
        assert!(check_with(&client, &dirs, &system).is_clean());
    }

    #[test]
    fn an_empty_folder_is_reported_as_unchecked_rather_than_as_a_pass() {
        let temp = Temp::new("empty");
        let client = temp.path().join("client");
        std::fs::create_dir_all(&client).unwrap();
        let report = check_with(&client, &[client.clone()], temp.path());
        assert!(report.is_clean(), "there is nothing to be unclean about");
        let (level, line) = report.lines().into_iter().next().unwrap();
        assert_eq!(level, Level::Warn);
        assert!(line.contains("the check never ran"), "{line}");
    }

    #[test]
    fn a_32_bit_module_is_named_rather_than_silently_skipped() {
        let temp = Temp::new("pe32");
        let client = temp.path().join("client");
        let mut bytes = fake_pe(&[]);
        let opt = 0x80 + 24;
        bytes[opt..opt + 2].copy_from_slice(&0x10Bu16.to_le_bytes());
        write(&client, "Old.dll", &bytes);
        let report = check_with(&client, &[client.clone()], temp.path());
        assert_eq!(report.modules, 0);
        assert_eq!(report.unreadable.len(), 1);
        assert!(report.unreadable[0].1.contains("32-bit"), "{report:?}");
    }

    /// The control that is close to the subject: the real folder, on this machine.
    ///
    /// Skipped rather than failed when the client is not there, because this crate is built on
    /// machines that have no client - but on the owner's it is the only test that exercises a
    /// 76 MB packed executable, delay-load descriptors, and the real search path.
    #[test]
    fn the_real_client_folder_is_self_contained() {
        let client = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("client-patched");
        if !client.join("MapleStory.exe").is_file() {
            eprintln!("skipped: no client at {}", client.display());
            return;
        }
        let started = std::time::Instant::now();
        let report = check(&client);
        // Printed, not just asserted: this is the only place the cost of the check on the
        // real client is visible, and it is paid on every Start Game. `cargo test -p launcher
        // integrity -- --nocapture`.
        eprintln!("{} modules in {:?}\n{}", report.modules, started.elapsed(), report.text());
        assert!(report.modules > 20, "the walk did not happen: {report:?}");
        assert!(
            report.missing.is_empty(),
            "the client that runs on this machine is missing something: {:?}",
            report.missing
        );
        assert!(
            report.borrowed.is_empty(),
            "the shipped folder is not self-contained: {:?}",
            report.borrowed
        );
    }
}
