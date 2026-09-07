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

/// Compile `grap64.dll` into the launcher, so one executable can neutralise GameGuard on a
/// machine that has nothing else on it.
///
/// Cargo has no stable way to depend on another crate's **binary artifact**, so this is a
/// file lookup rather than a real dependency. It looks for the profile being built first and
/// then release, because a debug launcher run out of the repo should still install the stub
/// that `tools/setup-client.ps1` and `make-installer.ps1` use - and `crates/launcher/src/
/// paths.rs` pins `STUB_PROFILE` to release for exactly that reason.
///
/// A miss is **not** a build failure: it would mean a fresh checkout could not compile the
/// launcher until it had compiled something else first, and the on-disk `stub_path` still
/// works. It is a `cargo:warning`, because a launcher that silently shipped without its stub
/// is the failure this whole feature exists to prevent.
fn embed_stub() {
    println!("cargo:rerun-if-changed=build.rs");
    // Declares the cfg so `unexpected_cfgs` does not warn on every build. Without it rustc
    // suggests "consider using a Cargo feature instead", which would be the wrong shape: a
    // feature is something a person turns on, and this is something the build DISCOVERS.
    println!("cargo:rustc-check-cfg=cfg(has_embedded_stub)");
    let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") else { return };
    let target = std::path::Path::new(&manifest)
        .parent() // crates/
        .and_then(|p| p.parent()) // repo root
        .map(|p| p.join("target"));
    let Some(target) = target else { return };

    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "release".into());
    let mut tried = Vec::new();
    for dir in [target.join(&profile), target.join("release")] {
        let candidate = dir.join("grap64.dll");
        println!("cargo:rerun-if-changed={}", candidate.display());
        if candidate.is_file() {
            // `include_bytes!` needs a literal, so the path travels as an env var the source
            // reads with `env!`.
            println!("cargo:rustc-env=MAPLECW_EMBEDDED_STUB={}", candidate.display());
            println!("cargo:rustc-cfg=has_embedded_stub");
            return;
        }
        tried.push(candidate.display().to_string());
    }
    println!(
        "cargo:warning=launcher: no grap64.dll found, so the built-in GameGuard stub is NOT \
         compiled in and this launcher needs one on disk. Looked in: {}. Fix with: cargo build \
         --release -p grap-stub",
        tried.join(", ")
    );
}

/// Compile the sign-in service's certificate fingerprint into the launcher, so a client
/// machine needs no certificate configuration at all.
///
/// # Why
///
/// The owner, 2026-09-07: *"I would like to modify the launcher to automatically assume we will use
/// the current cert. The server will use the same cert as the test machine ... that way no
/// manual configuration is needed on the client's files."*
///
/// Until now a pin had to reach every client by hand - `install.ps1 -AuthFingerprint <value>`,
/// or a `auth-cert-fingerprint.txt` copied beside the launcher - and a machine that missed
/// that step could not sign in at all. Baking the value in makes the ordinary deployment
/// need nothing, and every on-disk source still **overrides** it (`crates/launcher/src/paths.rs`
/// `resolve_fingerprint`), so a different server is still a config key away.
///
/// # This is a pin, not a relaxation
///
/// The launcher refuses any certificate that does not match. Baking a value in changes *where
/// the pin comes from*, never whether there is one: a build with no fingerprint file produces
/// a launcher with **no** baked pin, which behaves exactly as it does today - it looks on disk
/// and refuses sign-in if it finds nothing. There is no path here that accepts an unknown
/// certificate.
///
/// # The failure mode this creates, stated because it is new
///
/// `crates/auth/src/tls.rs` generates a self-signed certificate **only when `auth-cert.pem` is
/// absent** and reloads it otherwise, so the value is stable for as long as those two files
/// survive. If the server is ever deployed without them it mints a new one, and every launcher
/// built before that refuses to sign in. That is the correct behaviour for a pin and it is
/// why `tools/package-server.ps1` ships the certificate with the server.
fn embed_fingerprint() {
    println!("cargo:rustc-check-cfg=cfg(has_baked_fingerprint)");
    let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") else { return };
    let Some(root) = std::path::Path::new(&manifest).parent().and_then(|p| p.parent()) else {
        return;
    };
    let file = root.join("auth-cert-fingerprint.txt");
    println!("cargo:rerun-if-changed={}", file.display());

    let Ok(text) = std::fs::read_to_string(&file) else {
        println!(
            "cargo:warning=launcher: no {} at the repo root, so NO certificate pin is compiled \
             in and every client machine will need one on disk or in its config. Start the \
             sign-in service once to write it.",
            file.display()
        );
        return;
    };
    let text = text.trim().to_string();
    // Shape-checked here rather than trusted: a launcher that baked in a typo would refuse
    // every sign-in with a pin nobody could find the source of. The real parse still happens
    // at run time in `tlspin::Fingerprint::parse`, which is the authority.
    let body = text.strip_prefix("sha256:").unwrap_or("");
    if body.len() != 64 || !body.bytes().all(|b| b.is_ascii_hexdigit()) {
        println!(
            "cargo:warning=launcher: {} does not hold `sha256:` followed by 64 hex characters, \
             so NO certificate pin is compiled in. Found {:?}",
            file.display(),
            text.chars().take(24).collect::<String>()
        );
        return;
    }
    println!("cargo:rustc-env=MAPLECW_BAKED_FINGERPRINT={text}");
    println!("cargo:rustc-cfg=has_baked_fingerprint");
}

fn main() {
    embed_stub();
    embed_fingerprint();

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
