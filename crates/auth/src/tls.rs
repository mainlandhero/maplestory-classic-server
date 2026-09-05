//! The sign-in service's own certificate, made on first start and kept beside the database.
//!
//! # Self-signed, and pinned rather than trusted
//!
//! There is no certificate authority here. The service generates one self-signed certificate
//! the first time it runs, writes it and its key beside the database, and writes the
//! certificate's SHA-256 to [`tlspin::FINGERPRINT_FILE`] in the same directory. That
//! fingerprint is what every launcher pins - `crates/tlspin` is the one definition both ends
//! use - so the certificate's name, its dates and who signed it are all irrelevant. What
//! matters is that it is **this** certificate, and that the private key stays on this box.
//!
//! # Regenerating it breaks every client, on purpose
//!
//! Delete `auth-cert.pem` and `auth-key.pem` and the next start makes a new pair with a new
//! fingerprint, and every launcher pinned to the old one refuses to sign in - with a message
//! that says the fingerprint did not match, which is exactly what should happen when a server
//! turns up with a certificate its clients were not told about. That is the same behaviour an
//! attacker would trigger, and it is not distinguishable from here, which is the point.
//!
//! # The private key is a file beside a database of password hashes
//!
//! Stated rather than hidden. It has the same protection the database has - the box's own
//! file permissions - and anyone who can read one can read the other. That is the right
//! level for this project; a hardware token is not.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

pub const CERT_FILE: &str = "auth-cert.pem";
pub const KEY_FILE: &str = "auth-key.pem";

/// A loaded (or freshly made) certificate and key, and the fingerprint clients must pin.
pub struct Identity {
    cert: CertificateDer<'static>,
    key: PrivateKeyDer<'static>,
    pub fingerprint: tlspin::Fingerprint,
    pub cert_path: PathBuf,
    /// True when this start made the files rather than reading them - the banner says so,
    /// because a new fingerprint means every client's pin is now wrong.
    pub created: bool,
}

/// Load the identity from `dir`, generating it first if it is not there.
///
/// Always rewrites the fingerprint file, so it can never describe a certificate other than
/// the one on disk beside it.
pub fn ensure_identity(dir: &Path) -> io::Result<Identity> {
    let cert_path = dir.join(CERT_FILE);
    let key_path = dir.join(KEY_FILE);
    let created = !(cert_path.is_file() && key_path.is_file());
    if created {
        // The names are cosmetic: nothing checks them. "localhost" is there so a browser
        // pointed at the health endpoint on the server box complains about the issuer
        // rather than the name, which is the more useful complaint.
        let certified = rcgen::generate_simple_self_signed(vec![
            "maplecw-auth".to_string(),
            "localhost".to_string(),
        ])
        .map_err(|e| io::Error::other(format!("could not generate a certificate: {e}")))?;
        std::fs::write(&cert_path, certified.cert.pem())?;
        std::fs::write(&key_path, certified.signing_key.serialize_pem())?;
    }
    let cert = CertificateDer::from_pem_file(&cert_path)
        .map_err(|e| io::Error::other(format!("{}: {e}", cert_path.display())))?;
    let key = PrivateKeyDer::from_pem_file(&key_path)
        .map_err(|e| io::Error::other(format!("{}: {e}", key_path.display())))?;
    let fingerprint = tlspin::Fingerprint::of_der(cert.as_ref());
    std::fs::write(dir.join(tlspin::FINGERPRINT_FILE), format!("{fingerprint}\n"))?;
    Ok(Identity { cert, key, fingerprint, cert_path, created })
}

impl Identity {
    /// The server side of TLS 1.3 with this certificate. No client certificates.
    pub fn server_config(&self) -> io::Result<Arc<rustls::ServerConfig>> {
        let config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![self.cert.clone()], self.key.clone_key())
            .map_err(|e| {
                io::Error::other(format!("the certificate and key do not make a TLS identity: {e}"))
            })?;
        Ok(Arc::new(config))
    }

    /// What the console says at startup. The fingerprint line is the one an operator copies.
    pub fn banner(&self) -> Vec<String> {
        let mut lines = Vec::new();
        if self.created {
            lines.push(format!(
                "TLS: NEW certificate generated at {} - every client must pin the fingerprint below",
                self.cert_path.display()
            ));
        } else {
            lines.push(format!("TLS: certificate {}", self.cert_path.display()));
        }
        lines.push(format!("TLS: fingerprint {}", self.fingerprint));
        lines.push(format!(
            "     Put it in maplecw-launcher.toml on every client machine as\n       auth_fingerprint = \"{}\"\n     or copy {} from beside the database to beside the launcher.\n     install.ps1 -AuthFingerprint <that value> does it at install time.",
            self.fingerprint,
            tlspin::FINGERPRINT_FILE
        ));
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("maplecw-auth-tls-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn a_first_start_generates_and_a_second_start_reloads_the_same_certificate() {
        let dir = scratch("reload");
        let first = ensure_identity(&dir).unwrap();
        assert!(first.created);
        assert!(dir.join(CERT_FILE).is_file());
        assert!(dir.join(KEY_FILE).is_file());
        let written = std::fs::read_to_string(dir.join(tlspin::FINGERPRINT_FILE)).unwrap();
        assert_eq!(tlspin::Fingerprint::parse(&written).unwrap(), first.fingerprint);

        let second = ensure_identity(&dir).unwrap();
        assert!(!second.created, "the files were there, so nothing is regenerated");
        assert_eq!(second.fingerprint, first.fingerprint, "same certificate, same pin");
        assert!(second.server_config().is_ok(), "the pair on disk is a usable TLS identity");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn deleting_the_pair_makes_a_new_fingerprint() {
        let dir = scratch("regen");
        let first = ensure_identity(&dir).unwrap().fingerprint;
        std::fs::remove_file(dir.join(CERT_FILE)).unwrap();
        std::fs::remove_file(dir.join(KEY_FILE)).unwrap();
        let second = ensure_identity(&dir).unwrap();
        assert!(second.created);
        assert_ne!(second.fingerprint, first, "a new key pair is a new pin, and every client notices");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_banner_carries_the_full_fingerprint_and_the_config_key() {
        let dir = scratch("banner");
        let id = ensure_identity(&dir).unwrap();
        let text = id.banner().join("\n");
        assert!(text.contains(&id.fingerprint.to_string()));
        assert!(text.contains("auth_fingerprint ="));
        assert!(text.contains("NEW certificate"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
