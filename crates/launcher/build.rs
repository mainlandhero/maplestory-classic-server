//! Embed the UAC manifest, so double-clicking the launcher elevates.
//!
//! # Why this is a linker argument and not a crate
//!
//! The usual way is `embed-resource` or `winres`, which compile a `.rc` and hand the linker a
//! `.res`. Both are build-dependencies, and this workspace is deliberately thin - `grap-stub`
//! has none at all and declares its Win32 by hand. MSVC's linker embeds a manifest directly
//! with `/MANIFEST:EMBED /MANIFESTINPUT:<file>`, which needs nothing but the file.
//!
//! # What happens on a non-MSVC toolchain
//!
//! Nothing. The flags are only emitted for an MSVC target; a GNU or non-Windows build gets an
//! executable with no manifest, which still runs and simply does not elevate. That is the
//! right failure: the manifest is a Windows deployment detail, not a correctness one, and
//! `cargo test` on any host must keep working.
//!
//! # It is not silent about being skipped
//!
//! A manifest that quietly failed to embed would produce a launcher that runs unelevated,
//! hands the client a second UAC prompt, and looks like the feature was never built - the
//! shape `CLAUDE.md` calls "built is not wired". So the skip prints a `cargo:warning`, which
//! shows up in the build output rather than nowhere.

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    if target_os != "windows" {
        return; // Not a warning: a non-Windows build is not trying to elevate anything.
    }
    if target_env != "msvc" {
        println!(
            "cargo:warning=launcher: /MANIFEST:EMBED needs the MSVC linker (target_env is \
             {target_env:?}), so the UAC manifest was NOT embedded. The launcher will run \
             unelevated and the client will raise its own UAC prompt."
        );
        return;
    }

    let dir = std::env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    let manifest = std::path::Path::new(&dir).join("launcher.manifest");
    if !manifest.is_file() {
        println!(
            "cargo:warning=launcher: {} is missing, so no UAC manifest was embedded.",
            manifest.display()
        );
        return;
    }

    // `-bins` rather than the unsuffixed form: a manifest belongs on the executable, and
    // applying link args to every artifact would put it on test binaries too.
    println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
    println!("cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}", manifest.display());
    // The elevation, and it MUST be here rather than in launcher.manifest. The linker always
    // generates its own UAC fragment and merges it with the input; two fragments naming
    // different levels is `manifest authoring error c1010001` and `LNK1327`, not an override.
    // Measured by trying it. The manifest file carries the same note.
    println!("cargo:rustc-link-arg-bins=/MANIFESTUAC:level='requireAdministrator' uiAccess='false'");
    println!("cargo:rerun-if-changed=launcher.manifest");
    println!("cargo:rerun-if-changed=build.rs");
}
