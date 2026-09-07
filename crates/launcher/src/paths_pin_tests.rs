//! Where the certificate pin comes from, in order - `crate::paths::resolve_fingerprint`.
//!
//! Its own file because the order is the whole behaviour: a config key the operator wrote
//! beats a file, a file beats nothing, and a bad value is reported rather than treated as
//! nothing - a typo must never turn into a refused sign-in with no explanation.

use std::path::PathBuf;

use crate::paths::resolve_from;

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("maplecw-launcher-pin-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// **With nothing on disk, the answer now depends on how this launcher was built**, and both
/// outcomes are asserted here rather than one of them being assumed.
///
/// This test failed the moment `build.rs` started baking a pin in, which is the test doing its
/// job: the behaviour genuinely changed. What must NOT change is that there is no third
/// outcome - a launcher either has a pin or refuses to sign in, and never accepts an unknown
/// certificate because it could not find one.
#[test]
fn with_nothing_pinned_the_answer_is_the_baked_pin_or_a_refusal() {
    let dir = scratch("none");
    let layout = resolve_from(&dir);

    match crate::paths::baked_fingerprint() {
        Some(baked) => {
            let expected = tlspin::Fingerprint::parse(baked).expect("build.rs shape-checks it");
            assert_eq!(
                layout.auth_fingerprint,
                Some(expected),
                "a build with a compiled-in pin must use it when nothing is on disk"
            );
            assert!(
                layout.auth_fingerprint_from.contains("compiled into this launcher"),
                "the report must say where the pin came from: {}",
                layout.auth_fingerprint_from
            );
            assert!(layout.report().contains(&expected.to_string()));
        }
        None => {
            assert!(layout.auth_fingerprint.is_none());
            assert!(
                layout.auth_fingerprint_from.contains("NOT PINNED"),
                "{}",
                layout.auth_fingerprint_from
            );
            assert!(layout.report().contains("NOT PINNED"), "{}", layout.report());
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
}

/// **A file on disk still outranks the compiled-in pin.** This is what keeps a shipped
/// launcher pointable at a different server without a rebuild, and it is the property most
/// likely to be broken by someone tidying the fallback chain later.
#[test]
fn a_file_on_disk_outranks_the_compiled_in_pin() {
    let Some(baked) = crate::paths::baked_fingerprint() else {
        return; // nothing to outrank in a build without one
    };
    let baked = tlspin::Fingerprint::parse(baked).unwrap();
    let dir = scratch("outranks");
    let other = tlspin::Fingerprint::of_der(b"a different server's certificate");
    assert_ne!(other, baked, "the test needs two distinct fingerprints");

    std::fs::write(dir.join(tlspin::FINGERPRINT_FILE), format!("{other}\n")).unwrap();
    let layout = resolve_from(&dir);
    assert_eq!(layout.auth_fingerprint, Some(other), "the file must win");
    assert!(
        layout.auth_fingerprint_from.ends_with(tlspin::FINGERPRINT_FILE),
        "{}",
        layout.auth_fingerprint_from
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_fingerprint_file_beside_the_launcher_is_picked_up() {
    let dir = scratch("file");
    let fp = tlspin::Fingerprint::of_der(b"the dev service's certificate");
    std::fs::write(dir.join(tlspin::FINGERPRINT_FILE), format!("{fp}\n")).unwrap();
    let layout = resolve_from(&dir);
    assert_eq!(layout.auth_fingerprint, Some(fp));
    assert!(
        layout.auth_fingerprint_from.ends_with(tlspin::FINGERPRINT_FILE),
        "{}",
        layout.auth_fingerprint_from
    );
    assert!(layout.report().contains(&fp.to_string()));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_config_key_wins_over_the_file_and_a_bad_key_is_a_problem_not_an_absence() {
    let dir = scratch("config");
    let in_file = tlspin::Fingerprint::of_der(b"what the file says");
    let in_config = tlspin::Fingerprint::of_der(b"what the operator wrote");
    std::fs::write(dir.join(tlspin::FINGERPRINT_FILE), format!("{in_file}\n")).unwrap();
    let toml = dir.join(crate::config::CONFIG_FILE_NAME);
    std::fs::write(&toml, format!("auth_fingerprint = \"{in_config}\"\n")).unwrap();
    let layout = resolve_from(&dir);
    assert_eq!(layout.auth_fingerprint, Some(in_config), "the operator's explicit value wins");
    assert!(layout.config_applied.iter().any(|k| k == "auth_fingerprint"), "{:?}", layout.config_applied);

    std::fs::write(&toml, "auth_fingerprint = \"sha256:not-a-fingerprint\"\n").unwrap();
    let layout = resolve_from(&dir);
    assert_eq!(layout.auth_fingerprint, Some(in_file), "a bad key falls through to the file");
    assert!(
        layout.config_problems.iter().any(|p| p.contains("auth_fingerprint")),
        "the typo is reported: {:?}",
        layout.config_problems
    );
    let _ = std::fs::remove_dir_all(&dir);
}
