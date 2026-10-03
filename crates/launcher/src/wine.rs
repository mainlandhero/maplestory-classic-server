//! **Is this launcher running under Wine?** - the Mac client.
//!
//! The owner, 2026-10-02: *"not only do we need to release a windows client setup, we also need
//! to have a mac client for users running mac with all of the features from the launcher, the
//! gameguard stub, so that they can connect to our local server."*
//!
//! Nexon's own Mac build of Classic World (`MapleStory Launcher.app`) is a CrossOver bottle: the
//! **Windows** `MapleStory.exe` under Wine, with a native helper that starts the Mac anti-cheat
//! (`ngsx.framework`, `grap-core`) beside it. So the Mac client is not a port. It is THIS
//! launcher, THIS `grap64.dll` and the same client, run in a Wine bottle by
//! `tools/mac/MapleCW` (`docs/mac-client.md`). One code path, one set of patches.
//!
//! What has to know the difference is small, and this module is how it asks:
//!
//! * **the outbound firewall rule.** `netsh advfirewall` has no meaning in a Wine bottle - and
//!   Wine's `netsh` is a stub that does not fail, so asking it would log "firewall rule applied"
//!   for a rule that does not exist. That is the dangerous direction: a hardening measure
//!   reported as present when it is absent. [`firewall_note`] is what is logged instead.
//! * **the log**, which a player pastes back. A Mac log has to say it is one.
//!
//! Detection is the documented one: Wine's `ntdll.dll` exports `wine_get_version`, and
//! Windows' does not. Nothing is guessed from paths or environment variables.

use std::ffi::{c_char, c_void, CStr};

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleW(name: *const u16) -> *mut c_void;
    fn GetProcAddress(module: *mut c_void, name: *const c_char) -> *mut c_void;
}

/// What Wine says about itself, when it is there.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Wine {
    /// `wine_get_version()`, e.g. `"9.0"`. CrossOver reports its Wine base here.
    pub version: String,
    /// `wine_get_host_version()`'s system name - `"Darwin"` on a Mac, `"Linux"` elsewhere.
    /// `None` when the export is missing (very old Wine).
    pub host: Option<String>,
}

impl Wine {
    /// True on macOS. Only a Mac is a supported Wine host; Linux is "it may work".
    pub fn is_mac(&self) -> bool {
        self.host.as_deref() == Some("Darwin")
    }

    /// The one line every log starts with when this is the Mac client.
    pub fn line(&self) -> String {
        match &self.host {
            Some(_) if self.is_mac() => {
                format!("running under Wine {} on Darwin - the Mac client", self.version)
            }
            Some(h) => format!("running under Wine {} on {h} - not a supported host", self.version),
            None => format!("running under Wine {} (host unknown)", self.version),
        }
    }
}

/// `Some` under Wine, `None` on Windows. Cheap: two `GetProcAddress` calls.
pub fn detect() -> Option<Wine> {
    let name: Vec<u16> = "ntdll.dll".encode_utf16().chain(Some(0)).collect();
    // SAFETY: plain Win32 lookups; ntdll is mapped in every process, and both exports, when
    // present, are documented Wine extensions with exactly these C signatures.
    unsafe {
        let ntdll = GetModuleHandleW(name.as_ptr());
        if ntdll.is_null() {
            return None;
        }
        let get_version = GetProcAddress(ntdll, c"wine_get_version".as_ptr());
        if get_version.is_null() {
            return None;
        }
        let get_version: extern "C" fn() -> *const c_char = std::mem::transmute(get_version);
        let version = cstr(get_version()).unwrap_or_else(|| "(unknown version)".to_string());

        let host_fn = GetProcAddress(ntdll, c"wine_get_host_version".as_ptr());
        let host = if host_fn.is_null() {
            None
        } else {
            let host_fn: extern "C" fn(*mut *const c_char, *mut *const c_char) =
                std::mem::transmute(host_fn);
            let mut sysname: *const c_char = std::ptr::null();
            let mut release: *const c_char = std::ptr::null();
            host_fn(&mut sysname, &mut release);
            cstr(sysname)
        };
        Some(Wine { version, host })
    }
}

unsafe fn cstr(p: *const c_char) -> Option<String> {
    if p.is_null() {
        None
    } else {
        Some(CStr::from_ptr(p).to_string_lossy().into_owned())
    }
}

/// What the Start Game log says **instead of** writing a firewall rule under Wine.
///
/// A warning, and worded as what is true: nothing is blocking this client's outbound
/// connections. The reachability check that used to crash the client without network access
/// is answered by the hook (`1415db360:ret`), not by the firewall, so this costs hygiene, not
/// stability. A per-application block on a Mac needs administrator rights and a `pf` anchor,
/// which is a separate decision from shipping a client - `docs/mac-client.md`.
pub fn firewall_note(wine: &Wine) -> String {
    format!(
        "firewall rule NOT applied: this is Wine {} ({}), which has no Windows Firewall - \
         Wine's netsh would accept the command and block nothing. This client CAN reach the \
         internet. docs/mac-client.md",
        wine.version,
        wine.host.as_deref().unwrap_or("host unknown")
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The negative control, on the machine the tests run on.** The suite runs on Windows,
    /// where `ntdll` has no `wine_get_version`; a detector that said `Some` here would skip the
    /// firewall on every Windows client. The positive half - `Some` under Wine - can only be
    /// measured on a Mac, and `docs/mac-client.md` lists it as the first thing the Mac log
    /// must show.
    #[test]
    fn windows_is_not_wine() {
        assert_eq!(detect(), None, "this test machine is Windows; detect() must say so");
    }

    #[test]
    fn the_mac_line_says_it_is_the_mac_client() {
        let w = Wine { version: "9.0".into(), host: Some("Darwin".into()) };
        assert!(w.is_mac());
        assert!(w.line().contains("Wine 9.0 on Darwin - the Mac client"), "{}", w.line());
        let l = Wine { version: "9.0".into(), host: Some("Linux".into()) };
        assert!(!l.is_mac());
        assert!(l.line().contains("not a supported host"), "{}", l.line());
    }

    #[test]
    fn the_firewall_note_says_the_client_is_not_blocked() {
        let w = Wine { version: "9.0".into(), host: Some("Darwin".into()) };
        let note = firewall_note(&w);
        assert!(note.starts_with("firewall rule NOT applied"), "{note}");
        assert!(note.contains("CAN reach the internet"), "{note}");
    }
}
