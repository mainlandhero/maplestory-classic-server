//! **Which host a client is told to connect to** - decided per connection, not at startup.
//!
//! The owner, 2026-09-04: *"If there's any hardcoded IP such as 127.0.0.1, these IP needs to be
//! determined dynamically. I will allow the server software to bind to 0.0.0.0, but the
//! server needs to be able to determine its own public IP address."*
//!
//! # Bind is not advertise
//!
//! A server binds `0.0.0.0` and listens everywhere. But twice in this protocol it writes an
//! address **into a packet** for the client to dial - the migration after character select
//! (`net::opcode::migrate`, four octets straight into the client's `sockaddr_in`) and the
//! answer to Change Channel - and that address has to be one the *client* can reach. On one
//! box `127.0.0.1` works and hides the bug; from a LAN it sends the client back to itself;
//! from the internet it sends it to a LAN address it has never heard of.
//!
//! `--channels` still carries the ports and the order. What this module decides is the
//! **host** in each of those addresses, and under [`Mode::Auto`] it decides it for each
//! connection from two facts the accepted socket already knows:
//!
//! | the client's address is | it is told |
//! |---|---|
//! | loopback, RFC 1918 private, link-local, or CGNAT (`100.64/10`, which is what Tailscale hands out) | **the address it reached this server on** - the accepted socket's local address, which is the right interface even on a box with three |
//! | anything else - a public address | **this box's public address**, discovered at startup by asking an echo service and re-checked every ten minutes |
//!
//! The first row needs no discovery and no configuration, and it covers a LAN, a VPN and a
//! Tailscale-style overlay without knowing which one it is on. The second row is the only one
//! that needs the outside world, because a box behind NAT has no other way to learn the
//! address the internet knows it by - `local_addr()` on a port-forwarded connection is still
//! the LAN address the router translated to.
//!
//! # What this does NOT decide
//!
//! It does not open the router. Port forwarding is a router setting, and it is the operator's
//! call - `tools/installer/SERVER-README.txt` says why this server should not be on the
//! internet as it stands. This module only makes the *advertised* address correct for
//! whichever network the client turns out to be on.
//!
//! # Discovery is a measurement, and it is treated like one
//!
//! Three echo services are asked in turn over plain HTTP; the first answer that parses as a
//! **public** IPv4 address wins. An answer that is private or loopback is refused rather than
//! believed - it is what a captive portal or an interfering proxy returns. A failed discovery
//! is logged and the last good answer is kept; a public peer arriving with no public address
//! known is told the local address, and the log line says so and says it will probably fail,
//! because that is more useful than a silent wrong packet.

use std::fmt;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// The host `--channels` entries carry when they were given as a bare port: "decided by
/// `--advertise`". Never valid on the wire, and [`Advertiser::validate`] refuses it in
/// [`Mode::List`], where nothing would replace it.
pub const PLACEHOLDER_HOST: Ipv4Addr = Ipv4Addr::UNSPECIFIED;

/// How often the public address is re-checked once discovered. Home addresses change on the
/// order of days; ten minutes bounds how long a stale one is advertised after a change.
pub const REFRESH_EVERY: Duration = Duration::from_secs(10 * 60);

/// Per-service budget: connect, then the whole exchange. Three services at this cost is the
/// worst case a startup pays, and it pays it once.
const FETCH_TIMEOUT: Duration = Duration::from_secs(4);

/// Plain-HTTP echo services, asked in this order. Each answers `GET /` with the caller's
/// address as text and nothing else. Plain HTTP on purpose: this workspace has no TLS stack,
/// and the answer is the one thing on the internet that gains nothing from confidentiality -
/// it is the address the request came from.
const ECHO_SERVICES: [&str; 3] = ["checkip.amazonaws.com", "api.ipify.org", "icanhazip.com"];

/// The help text both binaries print for `--advertise`, kept in one place so the two cannot
/// describe the same flag differently.
pub const USAGE: &str = "\
  --advertise MODE what HOST a client is told to connect to for a channel. --channels
                   keeps the ports and the order; this decides the host in each:
                     auto   (default) per connection - a client on a private network
                            is told the address it reached this server on; a client
                            on a public address is told this box's PUBLIC address,
                            discovered at startup and re-checked every 10 minutes
                     list   exactly the hosts written in --channels
                     IPV4   this one host, for every client";

/// How the host is chosen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Decide per connection - the table in the module doc.
    Auto,
    /// One host for every client, whatever it connected to.
    Fixed(Ipv4Addr),
    /// Exactly what `--channels` says. The pre-2026-09-04 behaviour, kept as the control.
    List,
}

impl Mode {
    /// Parse the `--advertise` value.
    pub fn parse(text: &str) -> Result<Mode, String> {
        let t = text.trim();
        if t.eq_ignore_ascii_case("auto") {
            return Ok(Mode::Auto);
        }
        if t.eq_ignore_ascii_case("list") {
            return Ok(Mode::List);
        }
        match t.parse::<Ipv4Addr>() {
            Ok(ip) if ip.is_unspecified() => {
                Err(format!("--advertise {t}: 0.0.0.0 is a bind address, not one a client can dial"))
            }
            Ok(ip) => Ok(Mode::Fixed(ip)),
            Err(_) => Err(format!("--advertise {t}: expected auto, list, or an IPv4 address")),
        }
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Auto => f.write_str("auto"),
            Mode::Fixed(ip) => write!(f, "{ip}"),
            Mode::List => f.write_str("list"),
        }
    }
}

/// What discovery last found, and when.
#[derive(Debug, Clone)]
struct PublicIp {
    addr: Option<Ipv4Addr>,
    source: &'static str,
    at: Option<Instant>,
    /// The last failure, kept so a log line can say why there is no address rather than
    /// only that there is none.
    last_error: Option<String>,
}

/// The decision maker, shared by every connection of a server process.
pub struct Advertiser {
    mode: Mode,
    public: Mutex<PublicIp>,
}

impl Default for Advertiser {
    fn default() -> Self {
        Advertiser::new(Mode::Auto)
    }
}

impl fmt::Debug for Advertiser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Advertiser")
            .field("mode", &self.mode)
            .field("public", &self.public_ip())
            .finish()
    }
}

impl Advertiser {
    pub fn new(mode: Mode) -> Self {
        Advertiser {
            mode,
            public: Mutex::new(PublicIp { addr: None, source: "", at: None, last_error: None }),
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// The public address as last discovered, if any. `None` in [`Mode::Fixed`] and
    /// [`Mode::List`], which never look.
    pub fn public_ip(&self) -> Option<Ipv4Addr> {
        self.public.lock().map(|p| p.addr).unwrap_or(None)
    }

    /// Refuse a configuration that cannot produce a dialable address.
    ///
    /// Only [`Mode::List`] can: a bare-port channel carries [`PLACEHOLDER_HOST`], and in this
    /// mode nothing replaces it, so the client would be told to dial `0.0.0.0`.
    pub fn validate(&self, channels: &[SocketAddrV4]) -> Result<(), String> {
        if self.mode != Mode::List {
            return Ok(());
        }
        for (id, ch) in channels.iter().enumerate() {
            if ch.ip().is_unspecified() {
                return Err(format!(
                    "--advertise list needs a host for every channel, and channel {id} was \
                     given as a bare port ({}). Write host:port, or use --advertise auto",
                    ch.port()
                ));
            }
        }
        Ok(())
    }

    /// Discover the public address once, synchronously, and describe the whole arrangement.
    ///
    /// Called from a server's startup, before it listens, so the banner can say what each
    /// channel will be advertised as. Returns log lines rather than logging, because this
    /// crate has no logger and the two servers each have their own.
    pub fn prepare(&self, channels: &[SocketAddrV4]) -> Vec<String> {
        let mut lines = Vec::new();
        match self.mode {
            Mode::Auto => {
                lines.push(
                    "advertise: AUTO - each client is told the host it can actually reach:".to_string(),
                );
                lines.push(
                    "  a client on a private, loopback, link-local or CGNAT address is told the"
                        .to_string(),
                );
                lines.push("  address it reached this server on; a client on a public address is told".to_string());
                lines.push("  this box's public address, below.".to_string());
                match self.discover() {
                    Ok((ip, source)) => {
                        lines.push(format!("  public address: {ip} (from {source}, re-checked every {} min)", REFRESH_EVERY.as_secs() / 60));
                    }
                    Err(e) => {
                        lines.push(format!("  public address: UNKNOWN - {e}"));
                        lines.push(
                            "  A client arriving from a public address will be told this box's LOCAL"
                                .to_string(),
                        );
                        lines.push(
                            "  address, which it almost certainly cannot reach. LAN and VPN clients are"
                                .to_string(),
                        );
                        lines.push("  unaffected. To pin one by hand: --advertise <ipv4>.".to_string());
                    }
                }
            }
            Mode::Fixed(ip) => {
                lines.push(format!("advertise: every client is told {ip} (--advertise {ip})"));
            }
            Mode::List => {
                lines.push("advertise: exactly the hosts in --channels (--advertise list)".to_string());
            }
        }
        for (id, ch) in channels.iter().enumerate() {
            let host = match self.mode {
                Mode::Auto => "host decided per connection".to_string(),
                Mode::Fixed(ip) => ip.to_string(),
                Mode::List => ch.ip().to_string(),
            };
            lines.push(format!("  channel {id}: port {}, {host}", ch.port()));
        }
        lines
    }

    /// Ask the echo services now. Updates the cache on success; on failure keeps whatever was
    /// there and records the error.
    pub fn discover(&self) -> Result<(Ipv4Addr, &'static str), String> {
        let mut errors = Vec::new();
        for service in ECHO_SERVICES {
            match fetch_public_ip(service) {
                Ok(ip) => {
                    if let Ok(mut p) = self.public.lock() {
                        p.addr = Some(ip);
                        p.source = service;
                        p.at = Some(Instant::now());
                        p.last_error = None;
                    }
                    return Ok((ip, service));
                }
                Err(e) => errors.push(format!("{service}: {e}")),
            }
        }
        let why = errors.join("; ");
        if let Ok(mut p) = self.public.lock() {
            p.last_error = Some(why.clone());
        }
        Err(why)
    }

    /// Keep the public address current in the background.
    ///
    /// Only [`Mode::Auto`] has anything to refresh. The thread logs a change of address and
    /// each failure, with the age of the value still being advertised, so a stale address is
    /// visible in the log rather than only in a client that cannot connect.
    pub fn spawn_refresher(self: &Arc<Self>, log: impl Fn(String) + Send + 'static) {
        if self.mode != Mode::Auto {
            return;
        }
        let me = Arc::clone(self);
        std::thread::Builder::new()
            .name("advertise-refresh".into())
            .spawn(move || loop {
                std::thread::sleep(REFRESH_EVERY);
                let before = me.public_ip();
                match me.discover() {
                    Ok((ip, source)) if Some(ip) != before => {
                        log(format!(
                            "advertise: public address CHANGED {} -> {ip} (from {source})",
                            before.map(|b| b.to_string()).unwrap_or_else(|| "unknown".into())
                        ));
                    }
                    Ok(_) => {}
                    Err(e) => {
                        let age = me
                            .public
                            .lock()
                            .ok()
                            .and_then(|p| p.at)
                            .map(|t| format!("{} min old", t.elapsed().as_secs() / 60))
                            .unwrap_or_else(|| "never discovered".into());
                        log(format!(
                            "advertise: public address re-check FAILED ({e}); still advertising {} ({age})",
                            before.map(|b| b.to_string()).unwrap_or_else(|| "nothing".into())
                        ));
                    }
                }
            })
            .ok();
    }

    /// The address to write into a packet for this connection, and the sentence for the log.
    ///
    /// `listed` is the `--channels` entry - its port is always kept. `peer` and `local` are
    /// the accepted socket's two ends; either may be unknown, and a session built in a test
    /// has neither.
    pub fn address_for(
        &self,
        listed: SocketAddrV4,
        peer: Option<IpAddr>,
        local: Option<IpAddr>,
    ) -> (SocketAddrV4, String) {
        let (host, why) = decide(&self.mode, *listed.ip(), peer, local, self.public_ip());
        (SocketAddrV4::new(host, listed.port()), why)
    }
}

/// The rule, as a pure function so every row of the table is a unit test.
pub fn decide(
    mode: &Mode,
    listed: Ipv4Addr,
    peer: Option<IpAddr>,
    local: Option<IpAddr>,
    public: Option<Ipv4Addr>,
) -> (Ipv4Addr, String) {
    match mode {
        Mode::Fixed(ip) => (*ip, format!("{ip}, fixed by --advertise")),
        Mode::List => (listed, format!("{listed}, as listed in --channels")),
        Mode::Auto => {
            let peer4 = peer.and_then(as_ipv4);
            let local4 = local.and_then(as_ipv4).filter(|l| !l.is_unspecified());
            match (peer4, local4) {
                (Some(p), Some(l)) if reaches_us_directly(p) => (
                    l,
                    format!("{l}, the address this client reached us on - it is at {p}, which can reach it directly"),
                ),
                (Some(p), None) if reaches_us_directly(p) => (
                    listed,
                    format!("{listed}, as listed - the client at {p} could reach us directly but this socket's own address is unknown"),
                ),
                // From here on the peer is a public address.
                (Some(p), local4) => match public {
                    Some(pubip) => (
                        pubip,
                        format!("{pubip}, this box's public address - the client is at public address {p}"),
                    ),
                    None => {
                        let fallback = local4.unwrap_or(listed);
                        (
                            fallback,
                            format!(
                                "{fallback} - the client is at PUBLIC address {p} and this box's public address is UNKNOWN (discovery failed), so it is being told a local address it probably CANNOT reach. Pass --advertise <ipv4>"
                            ),
                        )
                    }
                },
                (None, Some(l)) => {
                    (l, format!("{l}, the address this client reached us on; its own address is unknown"))
                }
                (None, None) => (
                    listed,
                    format!("{listed}, as listed - neither end of this connection is known (a test, or an IPv6 socket)"),
                ),
            }
        }
    }
}

/// Can a client at this address dial an address on our side of the network directly?
///
/// Loopback, RFC 1918 private, link-local and CGNAT `100.64.0.0/10`. The last is what
/// Tailscale and carrier NAT hand out; a peer there reached us over an overlay or a
/// carrier's network, and the address it reached is the one to hand back.
pub fn reaches_us_directly(peer: Ipv4Addr) -> bool {
    let o = peer.octets();
    peer.is_loopback()
        || peer.is_private()
        || peer.is_link_local()
        || (o[0] == 100 && (64..=127).contains(&o[1]))
}

/// IPv4, including an IPv4-mapped IPv6 address (`::ffff:a.b.c.d`, what a dual-stack listener
/// reports for an IPv4 peer). A real IPv6 address is `None`: the packet has four octets.
fn as_ipv4(ip: IpAddr) -> Option<Ipv4Addr> {
    match ip.to_canonical() {
        IpAddr::V4(v4) => Some(v4),
        IpAddr::V6(_) => None,
    }
}

/// One `--channels` entry: `host:port`, or a bare `port` meaning "host decided by
/// `--advertise`" (stored as [`PLACEHOLDER_HOST`]).
pub fn parse_channel(text: &str) -> Result<SocketAddrV4, String> {
    let t = text.trim();
    if !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit()) {
        return t
            .parse::<u16>()
            .ok()
            .filter(|p| *p != 0)
            .map(|p| SocketAddrV4::new(PLACEHOLDER_HOST, p))
            .ok_or_else(|| format!("{t:?} is not a port (1..=65535)"));
    }
    t.parse::<SocketAddrV4>().map_err(|e| format!("{t:?}: {e} (IPv4 host:port, or a bare port)"))
}

/// The whole comma-separated `--channels` value.
pub fn parse_channels(text: &str) -> Result<Vec<SocketAddrV4>, String> {
    text.split(',').map(parse_channel).collect()
}

/// Ask one echo service. Plain HTTP/1.1, `Connection: close`, read to end.
fn fetch_public_ip(host: &str) -> Result<Ipv4Addr, String> {
    // Resolve and prefer an IPv4 endpoint: an answer from a service reached over IPv6 would be
    // this box's IPv6 address, which the packet cannot carry.
    let mut addrs = (host, 80u16).to_socket_addrs().map_err(|e| format!("resolve: {e}"))?;
    let addr = addrs
        .find(|a| a.is_ipv4())
        .ok_or_else(|| "resolved to no IPv4 address".to_string())?;
    let mut stream = TcpStream::connect_timeout(&addr, FETCH_TIMEOUT).map_err(|e| format!("connect: {e}"))?;
    stream.set_read_timeout(Some(FETCH_TIMEOUT)).ok();
    stream.set_write_timeout(Some(FETCH_TIMEOUT)).ok();
    let request = format!(
        "GET / HTTP/1.1\r\nHost: {host}\r\nUser-Agent: maplecw-advertise\r\nAccept: text/plain\r\nConnection: close\r\n\r\n"
    );
    stream.write_all(request.as_bytes()).map_err(|e| format!("send: {e}"))?;
    let mut raw = Vec::new();
    stream.take(64 * 1024).read_to_end(&mut raw).map_err(|e| format!("read: {e}"))?;
    let text = String::from_utf8_lossy(&raw);
    parse_echo_response(&text)
}

/// The HTTP response of an echo service, reduced to the address it carried.
///
/// Split out so the parse can be tested against captured shapes without a socket. The
/// status must be 200; the body is scanned for the first IPv4 token rather than trusted
/// whole, which makes a chunked body or a trailing newline irrelevant.
fn parse_echo_response(text: &str) -> Result<Ipv4Addr, String> {
    let (head, body) = text
        .split_once("\r\n\r\n")
        .or_else(|| text.split_once("\n\n"))
        .ok_or_else(|| "no HTTP header terminator in the response".to_string())?;
    let status = head.lines().next().unwrap_or("");
    if !(status.starts_with("HTTP/1.") && status.contains(" 200")) {
        return Err(format!("status {status:?}"));
    }
    let ip = first_ipv4_in(body).ok_or_else(|| format!("no IPv4 address in the body {body:?}"))?;
    if !is_public(ip) {
        return Err(format!("{ip} is not a public address - a proxy or captive portal answered"));
    }
    Ok(ip)
}

/// A usable public address: not private, loopback, link-local, CGNAT, unspecified,
/// broadcast, multicast or documentation space.
fn is_public(ip: Ipv4Addr) -> bool {
    let o = ip.octets();
    !(reaches_us_directly(ip)
        || ip.is_unspecified()
        || ip.is_broadcast()
        || ip.is_multicast()
        || ip.is_documentation()
        || o[0] == 0)
}

/// The first token of digits and dots that parses as an IPv4 address.
pub fn first_ipv4_in(text: &str) -> Option<Ipv4Addr> {
    text.split(|c: char| !(c.is_ascii_digit() || c == '.'))
        .filter(|t| !t.is_empty())
        .find_map(|t| t.parse::<Ipv4Addr>().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    const LISTED: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    const LAN: Ipv4Addr = Ipv4Addr::new(192, 168, 1, 20);
    const PUBLIC: Ipv4Addr = Ipv4Addr::new(203, 0, 113, 5);

    fn v4(a: [u8; 4]) -> Option<IpAddr> {
        Some(IpAddr::V4(Ipv4Addr::from(a)))
    }

    // ---------------------------------------------------------------- the table, row by row

    #[test]
    fn a_lan_peer_is_told_the_address_it_reached_us_on() {
        let (ip, why) = decide(&Mode::Auto, LISTED, v4([192, 168, 1, 77]), v4([192, 168, 1, 20]), Some(PUBLIC));
        assert_eq!(ip, LAN, "{why}");
        assert_ne!(ip, PUBLIC, "a LAN client must not be sent round the router");
    }

    #[test]
    fn a_loopback_peer_is_told_loopback_which_is_the_dev_box_unchanged() {
        let (ip, _) = decide(&Mode::Auto, LISTED, v4([127, 0, 0, 1]), v4([127, 0, 0, 1]), Some(PUBLIC));
        assert_eq!(ip, LISTED);
    }

    #[test]
    fn a_tailscale_peer_is_told_the_overlay_address_it_used() {
        let (ip, _) = decide(&Mode::Auto, LISTED, v4([100, 101, 5, 9]), v4([100, 64, 0, 3]), Some(PUBLIC));
        assert_eq!(ip, Ipv4Addr::new(100, 64, 0, 3));
    }

    #[test]
    fn a_public_peer_is_told_the_public_address() {
        let (ip, why) = decide(&Mode::Auto, LISTED, v4([198, 51, 100, 9]), v4([192, 168, 1, 20]), Some(PUBLIC));
        assert_eq!(ip, PUBLIC, "{why}");
    }

    #[test]
    fn a_public_peer_with_no_public_address_known_is_told_local_and_the_sentence_says_it_will_fail() {
        let (ip, why) = decide(&Mode::Auto, LISTED, v4([198, 51, 100, 9]), v4([192, 168, 1, 20]), None);
        assert_eq!(ip, LAN);
        assert!(why.contains("UNKNOWN"), "{why}");
        assert!(why.contains("CANNOT reach"), "{why}");
    }

    #[test]
    fn an_ipv4_mapped_ipv6_peer_is_treated_as_the_ipv4_it_is() {
        let peer: IpAddr = "::ffff:192.168.1.77".parse().unwrap();
        let local: IpAddr = "::ffff:192.168.1.20".parse().unwrap();
        let (ip, _) = decide(&Mode::Auto, LISTED, Some(peer), Some(local), Some(PUBLIC));
        assert_eq!(ip, LAN);
    }

    #[test]
    fn a_session_with_no_addresses_gets_the_listed_host() {
        // Every unit test that builds a Session without a socket lands here, which is why
        // the login and world suites did not change shape.
        let (ip, why) = decide(&Mode::Auto, LISTED, None, None, Some(PUBLIC));
        assert_eq!(ip, LISTED, "{why}");
    }

    #[test]
    fn an_unspecified_local_address_is_never_handed_out() {
        // A bare-port channel carries 0.0.0.0 and a socket can, in principle, report it.
        let (ip, _) = decide(&Mode::Auto, PLACEHOLDER_HOST, v4([198, 51, 100, 9]), v4([0, 0, 0, 0]), Some(PUBLIC));
        assert_eq!(ip, PUBLIC);
    }

    #[test]
    fn fixed_ignores_everything_else() {
        let (ip, _) = decide(&Mode::Fixed(PUBLIC), LISTED, v4([127, 0, 0, 1]), v4([127, 0, 0, 1]), None);
        assert_eq!(ip, PUBLIC);
    }

    #[test]
    fn list_is_the_old_behaviour() {
        let (ip, _) = decide(&Mode::List, LISTED, v4([198, 51, 100, 9]), v4([192, 168, 1, 20]), Some(PUBLIC));
        assert_eq!(ip, LISTED);
    }

    // ---------------------------------------------------------------- reachability

    #[test]
    fn direct_reachability_covers_the_four_private_families_and_nothing_public() {
        for yes in ["127.0.0.1", "10.0.0.1", "172.16.5.5", "172.31.255.1", "192.168.0.1", "169.254.1.1", "100.64.0.1", "100.127.255.254"] {
            assert!(reaches_us_directly(yes.parse().unwrap()), "{yes}");
        }
        for no in ["8.8.8.8", "172.32.0.1", "100.63.255.255", "100.128.0.1", "203.0.113.5", "1.1.1.1"] {
            assert!(!reaches_us_directly(no.parse().unwrap()), "{no}");
        }
    }

    // ---------------------------------------------------------------- the flag and the list

    #[test]
    fn the_flag_parses_its_three_forms_and_refuses_a_bind_address() {
        assert_eq!(Mode::parse("auto").unwrap(), Mode::Auto);
        assert_eq!(Mode::parse(" LIST ").unwrap(), Mode::List);
        assert_eq!(Mode::parse("203.0.113.5").unwrap(), Mode::Fixed(PUBLIC));
        assert!(Mode::parse("0.0.0.0").unwrap_err().contains("bind address"));
        assert!(Mode::parse("::1").is_err(), "four octets on the wire");
        assert!(Mode::parse("homelab").is_err());
    }

    #[test]
    fn a_bare_port_becomes_a_placeholder_host_and_a_host_port_is_kept() {
        assert_eq!(parse_channel("8485").unwrap(), SocketAddrV4::new(PLACEHOLDER_HOST, 8485));
        assert_eq!(parse_channel(" 192.168.1.20:8486 ").unwrap(), "192.168.1.20:8486".parse().unwrap());
        assert!(parse_channel("0").is_err());
        assert!(parse_channel("70000").is_err());
        assert!(parse_channel("[::1]:8485").is_err(), "IPv6 cannot go in the packet");
        assert_eq!(parse_channels("8485, 8486").unwrap().len(), 2);
    }

    #[test]
    fn list_mode_refuses_a_placeholder_and_auto_accepts_it() {
        let chans = vec![SocketAddrV4::new(PLACEHOLDER_HOST, 8485)];
        let err = Advertiser::new(Mode::List).validate(&chans).unwrap_err();
        assert!(err.contains("channel 0"), "{err}");
        assert!(Advertiser::new(Mode::Auto).validate(&chans).is_ok());
        assert!(Advertiser::new(Mode::Fixed(PUBLIC)).validate(&chans).is_ok());
    }

    #[test]
    fn address_for_keeps_the_listed_port() {
        let a = Advertiser::new(Mode::Fixed(PUBLIC));
        let (addr, _) = a.address_for("127.0.0.1:8486".parse().unwrap(), None, None);
        assert_eq!(addr, SocketAddrV4::new(PUBLIC, 8486));
    }

    // ---------------------------------------------------------------- the echo parse

    /// A routable address for the echo-parse tests. `PUBLIC` above is `203.0.113.5`, which
    /// is RFC 5737 documentation space: fine for `decide`, which takes what it is given, and
    /// **refused by `is_public`** - the first version of these tests used it and failed.
    /// The filter was right and the fixture was wrong; `documentation_space_is_refused`
    /// pins that so the next fixture cannot repeat it quietly.
    const ROUTABLE: Ipv4Addr = Ipv4Addr::new(93, 184, 216, 34);

    #[test]
    fn a_plain_echo_body_parses_with_or_without_a_trailing_newline() {
        let r = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 14\r\n\r\n93.184.216.34\n";
        assert_eq!(parse_echo_response(r).unwrap(), ROUTABLE);
        let r2 = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\n93.184.216.34";
        assert_eq!(parse_echo_response(r2).unwrap(), ROUTABLE);
    }

    #[test]
    fn a_chunked_echo_body_parses_because_the_scan_ignores_chunk_framing() {
        // The chunk size "d" is hex and has no dots, so it is not an address; the scan
        // moves on to the token that is.
        let r = "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nd\r\n93.184.216.34\r\n0\r\n\r\n";
        assert_eq!(parse_echo_response(r).unwrap(), ROUTABLE);
    }

    #[test]
    fn documentation_space_is_refused_like_a_private_answer() {
        let r = "HTTP/1.1 200 OK\r\n\r\n203.0.113.5\n";
        assert!(parse_echo_response(r).unwrap_err().contains("not a public address"));
    }

    #[test]
    fn a_non_200_or_a_private_answer_is_refused() {
        let r = "HTTP/1.1 503 Service Unavailable\r\n\r\n203.0.113.5\n";
        assert!(parse_echo_response(r).unwrap_err().contains("503"));
        let portal = "HTTP/1.1 200 OK\r\n\r\n<html>10.0.0.1 please log in</html>";
        assert!(parse_echo_response(portal).unwrap_err().contains("not a public address"));
        let empty = "HTTP/1.1 200 OK\r\n\r\n";
        assert!(parse_echo_response(empty).unwrap_err().contains("no IPv4 address"));
    }

    #[test]
    fn the_scanner_finds_the_first_real_address_and_skips_near_misses() {
        assert_eq!(first_ipv4_in("v1.2 built 2026.09.04 addr 203.0.113.5 ok").unwrap(), PUBLIC);
        assert_eq!(first_ipv4_in("999.1.1.1 then 8.8.8.8").unwrap(), Ipv4Addr::new(8, 8, 8, 8));
        assert!(first_ipv4_in("nothing here").is_none());
    }

    /// **The one test here that touches the network**, ignored by default so the suite stays
    /// hermetic. Run it by hand before trusting a "public address: UNKNOWN" banner:
    ///
    /// ```text
    /// cargo test -p net --release -- --ignored discovery_reaches_an_echo_service --nocapture
    /// ```
    ///
    /// It proves the fetch path end to end against a live service - resolve, connect, the
    /// hand-rolled request, the parse - which is the part the hermetic tests cannot.
    #[test]
    #[ignore = "asks a live echo service; run by hand"]
    fn discovery_reaches_an_echo_service() {
        let a = Advertiser::new(Mode::Auto);
        let (ip, source) = a.discover().expect("no echo service answered - is this box online?");
        println!("public address {ip} from {source}");
        assert!(is_public(ip));
        assert_eq!(a.public_ip(), Some(ip), "the cache holds what discovery found");
    }

    #[test]
    fn prepare_in_list_mode_touches_no_network_and_names_each_channel() {
        let a = Advertiser::new(Mode::List);
        let lines = a.prepare(&["192.168.1.20:8485".parse().unwrap(), "192.168.1.20:8486".parse().unwrap()]);
        assert!(lines[0].contains("--advertise list"), "{}", lines[0]);
        assert!(lines.iter().any(|l| l.contains("channel 1: port 8486, 192.168.1.20")), "{lines:?}");
        assert_eq!(a.public_ip(), None);
    }
}
