//! **The sign-in service's certificate fingerprint: printed by the server, pinned by the
//! launcher, defined once here.**
//!
//! The owner, 2026-09-05: *"We should not be sending passwords in plain text."* The fix chosen was
//! TLS with a pinned certificate rather than a CA: the server makes its own self-signed
//! certificate on first start and prints the SHA-256 of it; every client machine is told that
//! one value and accepts **that certificate and no other**. There is no certificate authority
//! to trust, no hostname to match, and nothing an attacker on the path can substitute without
//! also stealing the server's private key.
//!
//! # Why a separate crate
//!
//! `crates/auth` computes the fingerprint when it writes `auth-cert-fingerprint.txt`;
//! `crates/launcher` parses that text and compares it against the certificate the connection
//! presents. Two implementations of "what is a fingerprint" that drifted by one byte would
//! refuse every sign-in with a message blaming the certificate. One implementation, two users.
//!
//! # What pinning does and does not protect
//!
//! It protects the password and the launch token in transit, and it makes an active
//! man-in-the-middle fail the handshake before a byte of the password is sent. It does not
//! make the *game* socket authenticated - that socket still carries no credentials, and
//! `CLAUDE.md` says to say so whenever reporting progress.

use std::fmt;
use std::path::Path;
use std::sync::Arc;

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{DigitallySignedStruct, Error, SignatureScheme};
use sha2::{Digest, Sha256};

/// The file the sign-in service writes beside its database, and the launcher reads beside
/// itself (or, in a dev checkout, at the repo root - which is the same directory the dev
/// service writes to).
pub const FINGERPRINT_FILE: &str = "auth-cert-fingerprint.txt";

/// The prefix every printed fingerprint carries, so a value pasted from the wrong place - an
/// SSH key, a git hash - fails to parse instead of failing to match.
pub const PREFIX: &str = "sha256:";

/// SHA-256 of a certificate's DER encoding - the bytes on the wire, not the PEM text.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Fingerprint([u8; 32]);

impl Fingerprint {
    pub fn of_der(der: &[u8]) -> Self {
        Fingerprint(Sha256::digest(der).into())
    }

    /// Parse what a person pasted: `sha256:` optional and case-insensitive, colons between
    /// bytes allowed (the `openssl x509 -fingerprint` shape), whitespace ignored.
    pub fn parse(text: &str) -> Result<Self, String> {
        let t = text.trim();
        let t = if t.len() >= PREFIX.len() && t[..PREFIX.len()].eq_ignore_ascii_case(PREFIX) {
            &t[PREFIX.len()..]
        } else {
            t
        };
        let hex: String = t.chars().filter(|c| *c != ':' && !c.is_whitespace()).collect();
        if hex.len() != 64 {
            return Err(format!(
                "a fingerprint is 64 hex characters (32 bytes); this has {} - it should look like {PREFIX}<64 hex>",
                hex.len()
            ));
        }
        let mut out = [0u8; 32];
        for (i, byte) in out.iter_mut().enumerate() {
            let pair = &hex[2 * i..2 * i + 2];
            *byte = u8::from_str_radix(pair, 16)
                .map_err(|_| format!("{pair:?} at character {} is not hex", 2 * i))?;
        }
        Ok(Fingerprint(out))
    }

    pub fn matches(&self, der: &[u8]) -> bool {
        Self::of_der(der) == *self
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// The first four bytes, for a log line that has to fit on a screen: `sha256:ab12cd34...`.
    pub fn short(&self) -> String {
        format!(
            "{PREFIX}{:02x}{:02x}{:02x}{:02x}...",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

impl fmt::Display for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(PREFIX)?;
        for b in self.0 {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// Read a fingerprint file. `None` when there is no such file; `Some(Err)` when there is one
/// and it does not parse, which is worth reporting rather than treating as absent.
pub fn read_fingerprint_file(path: &Path) -> Option<Result<Fingerprint, String>> {
    let text = std::fs::read_to_string(path).ok()?;
    Some(Fingerprint::parse(&text).map_err(|e| format!("{}: {e}", path.display())))
}

/// A verifier that accepts exactly one certificate and rejects everything else.
///
/// Chains, names, expiry and revocation are all irrelevant to it on purpose: the operator
/// copied this exact certificate's fingerprint from the server's own console, so "is this
/// that certificate" is the whole question. Signatures over the handshake are still checked
/// against the certificate's key, which is what stops a replayed certificate without its key.
#[derive(Debug)]
pub struct PinnedVerifier {
    pin: Fingerprint,
    provider: Arc<CryptoProvider>,
}

impl PinnedVerifier {
    pub fn new(pin: Fingerprint) -> Self {
        PinnedVerifier { pin, provider: Arc::new(rustls::crypto::ring::default_provider()) }
    }
}

impl ServerCertVerifier for PinnedVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, Error> {
        if self.pin.matches(end_entity.as_ref()) {
            Ok(ServerCertVerified::assertion())
        } else {
            Err(Error::General(format!(
                "certificate fingerprint {} does not match the pinned {}",
                Fingerprint::of_der(end_entity.as_ref()),
                self.pin
            )))
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        verify_tls12_signature(message, cert, dss, &self.provider.signature_verification_algorithms)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        verify_tls13_signature(message, cert, dss, &self.provider.signature_verification_algorithms)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.provider.signature_verification_algorithms.supported_schemes()
    }
}

/// A client configuration that trusts the pinned certificate and nothing else.
pub fn client_config(pin: &Fingerprint) -> Arc<rustls::ClientConfig> {
    let config = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(PinnedVerifier::new(*pin)))
        .with_no_client_auth();
    Arc::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DER: &[u8] = b"pretend this is a DER-encoded certificate";

    #[test]
    fn a_fingerprint_prints_with_its_prefix_and_parses_back() {
        let fp = Fingerprint::of_der(DER);
        let text = fp.to_string();
        assert!(text.starts_with("sha256:"), "{text}");
        assert_eq!(text.len(), 7 + 64);
        assert_eq!(Fingerprint::parse(&text).unwrap(), fp);
    }

    #[test]
    fn every_shape_a_person_might_paste_parses_to_the_same_value() {
        let fp = Fingerprint::of_der(DER);
        let hex = fp.to_string()[7..].to_string();
        let upper = hex.to_uppercase();
        let colons: Vec<String> = (0..32).map(|i| hex[2 * i..2 * i + 2].to_string()).collect();
        for form in [
            hex.clone(),
            format!("SHA256:{upper}"),
            format!("  sha256:{hex}\n"),
            colons.join(":"),
            format!("sha256:{}", colons.join(":")),
        ] {
            assert_eq!(Fingerprint::parse(&form).unwrap(), fp, "{form:?}");
        }
    }

    #[test]
    fn the_wrong_length_and_non_hex_are_refused_with_a_sentence() {
        assert!(Fingerprint::parse("sha256:abcd").unwrap_err().contains("64 hex"));
        let bad = format!("sha256:{}", "zz".repeat(32));
        assert!(Fingerprint::parse(&bad).unwrap_err().contains("not hex"));
        assert!(Fingerprint::parse("").is_err());
    }

    #[test]
    fn short_form_is_the_first_four_bytes() {
        let fp = Fingerprint::of_der(DER);
        let s = fp.short();
        assert!(s.starts_with(&fp.to_string()[..7 + 8]), "{s}");
        assert!(s.ends_with("..."));
    }

    #[test]
    fn the_verifier_accepts_the_pinned_certificate_and_refuses_any_other() {
        let pin = Fingerprint::of_der(DER);
        let v = PinnedVerifier::new(pin);
        let name = ServerName::try_from("127.0.0.1").unwrap();
        let now = UnixTime::now();
        assert!(v
            .verify_server_cert(&CertificateDer::from(DER.to_vec()), &[], &name, &[], now)
            .is_ok());
        let other = CertificateDer::from(b"a different certificate".to_vec());
        let err = v.verify_server_cert(&other, &[], &name, &[], now).unwrap_err().to_string();
        assert!(err.contains("does not match the pinned"), "{err}");
        assert!(err.contains(&pin.to_string()), "the message names the pinned value: {err}");
    }

    #[test]
    fn a_fingerprint_file_is_absent_or_parsed_or_reported() {
        let dir = std::env::temp_dir().join(format!("tlspin-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(FINGERPRINT_FILE);
        assert!(read_fingerprint_file(&path).is_none());
        let fp = Fingerprint::of_der(DER);
        std::fs::write(&path, format!("{fp}\n")).unwrap();
        assert_eq!(read_fingerprint_file(&path).unwrap().unwrap(), fp);
        std::fs::write(&path, "garbage").unwrap();
        assert!(read_fingerprint_file(&path).unwrap().is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
