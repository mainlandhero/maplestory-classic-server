//! **Which process owns the far end of a local TCP connection.**
//!
//! # Why this exists
//!
//! `crates/store/src/claims.rs` keys a login claim per launch. To use one, the login server
//! has to match an accepted connection to the launch that staked it, and the two measurements
//! in `crate::migration`'s module docs say the client cannot help:
//!
//! * **the seed does not come back** - 8510 trials across 115 archived `0x007D` hello bodies
//!   and all 74 seeds ever minted, plain, both endiannesses and the XOR form the decompiler
//!   predicts, zero hits, with the character id at offset 8 passing as a positive control on
//!   all 115;
//! * **the identity block is per-machine, not per-launch** - two distinct `0x0073` bodies
//!   across 56 archived launches, and one of those two is the smoke test's synthetic
//!   `aabbccddeeff`.
//!
//! So nothing the client sends distinguishes one launch from another, and the address does not
//! either. The owner, 2026-08-29: *"The login MapleCW Launcher needs to be able to potentially
//! handle multiple connections from the same IP as well, IP cannot be the sole
//! discriminator."*
//!
//! What is left is a fact the **operating system** knows and neither side of the socket can
//! assert: which process opened the connection. The launcher starts `MapleStory.exe` and
//! therefore knows its process id; this module lets the server recover the same number from an
//! accepted socket. `tools/client-sockets.ps1` already does exactly this with
//! `Get-NetTCPConnection`'s `OwningProcess`; this is the same query without a PowerShell.
//!
//! # What it is, and the three things it is not
//!
//! It **is** a per-launch discriminator that works for several clients on one machine, which
//! is the owner's deployment and the case an address cannot separate.
//!
//! 1. **It is not authentication.** It says which process opened a socket, not who is at the
//!    keyboard. `CLAUDE.md`'s standing constraint is untouched: the game socket carries no
//!    credentials.
//! 2. **It is same-machine only.** A peer on another host has no row in this machine's TCP
//!    table, so the answer is `None` - correctly, and not as a failure. Off-box deployments
//!    fall back to the address rule and then to ambiguity.
//! 3. **It is not spoofable by the client, and that is the whole point.** The pid comes from
//!    the kernel's table keyed by the socket the server itself accepted. A process cannot
//!    register somebody else's socket. An attacker who can already inject into the victim's
//!    process does not need this.
//!
//! # Why it lives in `store`
//!
//! It is a socket concern in a database crate, which is the wrong shelf. The reason is
//! narrow: `crates/login` and `crates/world` both need it, both already depend on `store`, and
//! neither `crates/net` nor either server was this session's to restructure. If the crate
//! layout is ever revisited, this belongs beside the framing code.
//!
//! # The instrument checks itself
//!
//! `CLAUDE.md`: *"A dump writer that has never written a dump is exactly the kind of instrument
//! this file keeps warning about"*, and *"a constant that came from reading a header is a
//! claim, not a fact"*. Every struct below was written from `tcpmib.h` and every one of them
//! is a claim - the layout, the two byte orders, and which of the two rows per connection is
//! the client's.
//!
//! So [`self_test`] opens a real loopback connection inside this process and asserts the
//! lookup returns `std::process::id()`. It fails loudly if any of those claims is wrong, and
//! `the_lookup_finds_this_process_own_connection` runs it. That control is close to the
//! subject - it is the same call, on this machine, against a socket of exactly the shape the
//! login server will hand it - which is the property `CLAUDE.md` says a useless control lacks.

use std::net::{IpAddr, SocketAddr};

/// The process that owns the far end of `peer`, if it is on this machine.
///
/// `peer` is the **remote** endpoint of a socket this server accepted - which is the client's
/// *local* endpoint, and therefore what the TCP table lists in its local columns. Passing the
/// server's own local address instead finds the server's pid, which is a plausible-looking
/// wrong answer, so the direction matters: use `TcpStream::peer_addr`.
///
/// `None` means "not attributable", and it has several honest causes that a caller must treat
/// the same way: the peer is on another host, the connection has already closed, the table
/// could not be read, or this is not Windows. It is never a reason to refuse a connection.
pub fn owning_pid(peer: SocketAddr) -> Option<u32> {
    platform::owning_pid(peer)
}

/// [`owning_pid`] for a caller holding the `Option` that `TcpStream::peer_addr` produces.
pub fn owning_pid_of(peer: Option<SocketAddr>) -> Option<u32> {
    peer.and_then(owning_pid)
}

/// A sentence for a log line, saying what was found **and what it means when nothing was**.
///
/// Written here because the empty case is the one that gets logged as nothing at all, and
/// `CLAUDE.md`'s repeated complaint is about answers nobody reports.
pub fn describe(peer: SocketAddr, pid: Option<u32>) -> String {
    match pid {
        Some(pid) => format!("{peer} is owned by process {pid} on this machine"),
        None => format!(
            "{peer} could not be attributed to a local process - it is either on another \
             machine, already closed, or this is not Windows. The connection is resolved by \
             address, or by being the only live claim, or not at all"
        ),
    }
}

/// **Prove the lookup works, on this machine, right now.**
///
/// Opens a loopback listener, connects to it from this process, and asks who owns the client
/// end. The answer has to be this process. Returns `Err` with what it actually saw, so a
/// failure names the wrong number rather than merely being false.
///
/// Run by the test below, and worth calling from a server's startup banner if the pid rule is
/// ever suspected: an instrument that has never produced a positive is exactly the shape this
/// project keeps being bitten by.
pub fn self_test() -> std::result::Result<u32, String> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")
        .map_err(|e| format!("could not bind a loopback listener: {e}"))?;
    let addr = listener
        .local_addr()
        .map_err(|e| format!("the listener has no address: {e}"))?;
    let client = std::net::TcpStream::connect(addr)
        .map_err(|e| format!("could not connect to {addr}: {e}"))?;
    // Accepting keeps the connection established while the table is read; without it the row
    // can be in SYN_SENT and disappear under us.
    let accepted = listener
        .accept()
        .map_err(|e| format!("could not accept: {e}"))?
        .0;

    let client_local = client
        .local_addr()
        .map_err(|e| format!("the client socket has no local address: {e}"))?;
    let seen_from_server = accepted
        .peer_addr()
        .map_err(|e| format!("the accepted socket has no peer address: {e}"))?;
    // The two must agree, or the rest of the test is measuring the wrong socket.
    if client_local != seen_from_server {
        return Err(format!(
            "the client's local address {client_local} is not what the server sees as its peer \
             ({seen_from_server})"
        ));
    }

    let me = std::process::id();
    match owning_pid(seen_from_server) {
        Some(pid) if pid == me => Ok(pid),
        Some(pid) => Err(format!(
            "the table attributed {seen_from_server} to process {pid}, but this process is {me} \
             - the row layout or a byte order is wrong"
        )),
        None => Err(format!(
            "the table did not list {seen_from_server} at all, although this process has it \
             open. Either GetExtendedTcpTable failed or the address family is not handled"
        )),
    }
}

#[cfg(windows)]
mod platform {
    use super::*;

    // `TCP_TABLE_CLASS::TCP_TABLE_OWNER_PID_ALL`. Listeners are included and simply never
    // match a peer endpoint, so there is nothing to gain from the narrower class and one less
    // constant to get wrong.
    const TCP_TABLE_OWNER_PID_ALL: u32 = 5;
    const AF_INET: u32 = 2;
    const AF_INET6: u32 = 23;
    const NO_ERROR: u32 = 0;
    const ERROR_INSUFFICIENT_BUFFER: u32 = 122;

    /// `MIB_TCPROW_OWNER_PID`. Six `DWORD`s.
    ///
    /// `dwLocalAddr` is the IPv4 address in **network byte order**, so its native bytes are
    /// already the octets in address order. `dwLocalPort` holds the port in network byte order
    /// in its **low 16 bits**; the documentation says the high 16 may contain uninitialised
    /// data, which is why every read below masks before swapping.
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct TcpRowOwnerPid {
        _state: u32,
        local_addr: u32,
        local_port: u32,
        _remote_addr: u32,
        _remote_port: u32,
        owning_pid: u32,
    }

    /// `MIB_TCP6ROW_OWNER_PID`.
    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Tcp6RowOwnerPid {
        local_addr: [u8; 16],
        _local_scope: u32,
        local_port: u32,
        _remote_addr: [u8; 16],
        _remote_scope: u32,
        _remote_port: u32,
        _state: u32,
        owning_pid: u32,
    }

    #[link(name = "iphlpapi")]
    extern "system" {
        fn GetExtendedTcpTable(
            table: *mut std::ffi::c_void,
            size: *mut u32,
            order: i32,
            af: u32,
            class: u32,
            reserved: u32,
        ) -> u32;
    }

    /// The low 16 bits, network order, as a host `u16`.
    fn port_of(raw: u32) -> u16 {
        ((raw & 0xFFFF) as u16).swap_bytes()
    }

    /// Read the whole table for one address family into a byte buffer.
    ///
    /// Two calls: one to size it, one to fill it. Retried a few times because the table can
    /// grow between the two, which returns `ERROR_INSUFFICIENT_BUFFER` again rather than
    /// failing outright.
    fn read_table(af: u32) -> Option<Vec<u8>> {
        let mut size: u32 = 0;
        for _ in 0..8 {
            let rc = unsafe {
                GetExtendedTcpTable(
                    std::ptr::null_mut(),
                    &mut size,
                    0,
                    af,
                    TCP_TABLE_OWNER_PID_ALL,
                    0,
                )
            };
            if rc != ERROR_INSUFFICIENT_BUFFER && rc != NO_ERROR {
                return None;
            }
            if size == 0 {
                return None;
            }
            // Aligned to 4 by construction: every field in both row types is 4-byte aligned,
            // and a `Vec<u32>` reinterpreted as bytes cannot be less aligned than that.
            let words = (size as usize).div_ceil(4);
            let mut buf: Vec<u32> = vec![0; words];
            let mut have = (words * 4) as u32;
            let rc = unsafe {
                GetExtendedTcpTable(
                    buf.as_mut_ptr().cast(),
                    &mut have,
                    0,
                    af,
                    TCP_TABLE_OWNER_PID_ALL,
                    0,
                )
            };
            match rc {
                NO_ERROR => {
                    let mut bytes = Vec::with_capacity(words * 4);
                    for w in buf {
                        bytes.extend_from_slice(&w.to_ne_bytes());
                    }
                    return Some(bytes);
                }
                // The table grew. `have` now holds the new required size; go round again.
                ERROR_INSUFFICIENT_BUFFER => size = have,
                _ => return None,
            }
        }
        None
    }

    /// `DWORD dwNumEntries` followed by the rows, for both table types.
    fn entries(buf: &[u8], row_size: usize) -> Option<(usize, &[u8])> {
        let count = u32::from_ne_bytes(buf.get(..4)?.try_into().ok()?) as usize;
        let rows = buf.get(4..)?;
        // Trust the shorter of the two rather than the header: a count that overruns the
        // buffer would index out of bounds, and a buffer longer than the count is padding.
        Some((count.min(rows.len() / row_size), rows))
    }

    pub(super) fn owning_pid(peer: SocketAddr) -> Option<u32> {
        match peer.ip() {
            IpAddr::V4(want_ip) => {
                let buf = read_table(AF_INET)?;
                let size = std::mem::size_of::<TcpRowOwnerPid>();
                let (count, rows) = entries(&buf, size)?;
                for i in 0..count {
                    let start = i * size;
                    // SAFETY: `entries` clamped `count` so this range is inside `rows`, and the
                    // buffer came from a `Vec<u32>` so it is 4-aligned, which is this type's
                    // alignment. Read unaligned regardless - it costs nothing here and cannot
                    // be wrong.
                    let row: TcpRowOwnerPid =
                        unsafe { std::ptr::read_unaligned(rows[start..].as_ptr().cast()) };
                    if port_of(row.local_port) != peer.port() {
                        continue;
                    }
                    if std::net::Ipv4Addr::from(row.local_addr.to_ne_bytes()) == want_ip {
                        return Some(row.owning_pid);
                    }
                }
                None
            }
            IpAddr::V6(want_ip) => {
                let buf = read_table(AF_INET6)?;
                let size = std::mem::size_of::<Tcp6RowOwnerPid>();
                let (count, rows) = entries(&buf, size)?;
                for i in 0..count {
                    let start = i * size;
                    // SAFETY: as above.
                    let row: Tcp6RowOwnerPid =
                        unsafe { std::ptr::read_unaligned(rows[start..].as_ptr().cast()) };
                    if port_of(row.local_port) != peer.port() {
                        continue;
                    }
                    if std::net::Ipv6Addr::from(row.local_addr) == want_ip {
                        return Some(row.owning_pid);
                    }
                }
                None
            }
        }
    }

    #[cfg(test)]
    mod layout {
        use super::*;

        /// **The layout is a claim read out of a header, so it is asserted here.**
        ///
        /// `CLAUDE.md` records the cost of not doing this: a test asserted a `MINIDUMP_*`
        /// struct was 24 bytes because the field list said so, and Windows packs it to 16 -
        /// the test agreed with the code and neither agreed with Windows. These two are
        /// unpacked all-`DWORD` structures, so `repr(C)` is right; the assertion is what turns
        /// that from a belief into something that can fail.
        ///
        /// A wrong size here does not raise - it strides through the table at the wrong
        /// interval and reads plausible-looking garbage - which is why `self_test` exists as
        /// well. This catches it at the size; that catches it at the answer.
        #[test]
        fn the_row_structs_are_the_documented_size() {
            assert_eq!(std::mem::size_of::<TcpRowOwnerPid>(), 24);
            assert_eq!(std::mem::align_of::<TcpRowOwnerPid>(), 4);
            assert_eq!(std::mem::size_of::<Tcp6RowOwnerPid>(), 56);
            assert_eq!(std::mem::align_of::<Tcp6RowOwnerPid>(), 4);
        }

        /// The port lives in the low 16 bits, in network order, and the high 16 are
        /// documented as possibly uninitialised. Both halves of that matter: forgetting the
        /// swap gives a wrong port, and forgetting the mask gives a wrong port only on some
        /// machines, which is worse.
        #[test]
        fn the_port_is_masked_and_byte_swapped() {
            // 8484 = 0x2124; network order is 21 24, which little-endian reads as 0x2421.
            assert_eq!(port_of(0x2421), 8484);
            assert_eq!(port_of(0xDEAD_2421), 8484, "the high half must be ignored");
            assert_eq!(port_of(0x5000), 80);
        }
    }
}

#[cfg(not(windows))]
mod platform {
    use super::*;

    /// Not Windows, so there is no `GetExtendedTcpTable` and no attribution.
    ///
    /// Returning `None` is the honest answer and is the same answer a remote peer produces, so
    /// callers already handle it: the address rule and the sole-claim rule still work, and two
    /// clients on one machine are ambiguous rather than confused for each other. This project
    /// only runs on Windows; the arm exists so the crate compiles elsewhere rather than as a
    /// claim that anything has been tested there.
    pub(super) fn owning_pid(_peer: SocketAddr) -> Option<u32> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The positive control.** This proves the struct layout and the two byte orders - every
    /// one of which was written from a header rather than measured.
    ///
    /// **It cannot prove the direction**, and saying so is the point. Both ends of the socket
    /// belong to this process, so a lookup that matched the *remote* columns instead of the
    /// local ones would find the listener's row and return the same pid. That is a real
    /// failure - the login server would attribute every connection to itself, and no launch
    /// would ever resolve - and it is invisible from here.
    /// `a_connection_from_another_process_is_attributed_to_that_process` is the test that can
    /// see it, and it exists because this one cannot.
    #[cfg_attr(not(windows), ignore = "no TCP table to read off Windows")]
    #[test]
    fn the_lookup_finds_this_process_own_connection() {
        match self_test() {
            Ok(pid) => assert_eq!(pid, std::process::id()),
            Err(why) => panic!("{why}"),
        }
    }

    /// **The control that distinguishes the two directions.**
    ///
    /// A connection made by a *different* process, accepted here. The pid the table reports
    /// must be the child's, not this process's - which is exactly the claim the same-process
    /// self-test above is structurally unable to check, and exactly the mistake that would
    /// make the login server resolve nothing while looking like it worked.
    ///
    /// `curl.exe` is used because it ships in `System32` on Windows 10 1803 and later, makes
    /// one TCP connection, and needs no shell wrapper - so `Child::id()` really is the process
    /// that owns the socket. Its absence is a hard failure rather than a skip: a control that
    /// quietly does not run is the thing `CLAUDE.md` warns about most.
    #[cfg(windows)]
    #[test]
    fn a_connection_from_another_process_is_attributed_to_that_process() {
        let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".into());
        let curl = std::path::PathBuf::from(&system_root).join("System32").join("curl.exe");
        if !curl.is_file() {
            panic!(
                "no {} - the direction control cannot run, so local-vs-remote is unverified",
                curl.display()
            );
        }

        let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a loopback listener");
        let port = listener.local_addr().expect("an address").port();
        let mut child = std::process::Command::new(&curl)
            .args([
                "-s",
                "--max-time",
                "5",
                &format!("http://127.0.0.1:{port}/"),
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("curl.exe must start");
        let child_pid = child.id();

        let accepted = listener.accept().expect("curl must connect").0;
        let peer = accepted.peer_addr().expect("the accepted socket has a peer");
        let found = owning_pid(peer);

        // Tear the child down before asserting, so a failure does not leave a process behind.
        let _ = child.kill();
        let _ = child.wait();
        drop(accepted);

        assert_eq!(
            found,
            Some(child_pid),
            "the peer endpoint {peer} must be attributed to the process that opened it \
             ({child_pid}), not to this one ({}). Matching the table's REMOTE columns instead \
             of its local ones produces exactly this, and the same-process self-test cannot \
             see it",
            std::process::id()
        );
        assert_ne!(found, Some(std::process::id()), "that is the accepting side, not the peer");
    }

    /// **The negative control**, so "it returns None" can be told from "it returns None for
    /// everything". Port 0 is never a live local port, and the test above proves the same call
    /// does find a real one.
    #[test]
    fn an_endpoint_nobody_owns_is_not_attributed() {
        let nowhere: SocketAddr = "127.0.0.1:0".parse().unwrap();
        assert_eq!(owning_pid(nowhere), None);
    }

    /// A peer on another machine has no row here. Not a failure - the caller falls back to the
    /// address rule - but the log line has to say which of the two it was.
    #[test]
    fn an_off_box_peer_is_reported_as_unattributable_rather_than_silently_missing() {
        let remote: SocketAddr = "203.0.113.7:54321".parse().unwrap();
        assert_eq!(owning_pid(remote), None);
        let msg = describe(remote, None);
        assert!(msg.contains("another"), "{msg}");
        assert!(msg.contains("203.0.113.7"), "{msg}");
    }

    #[test]
    fn a_found_pid_is_named_in_the_log_line() {
        let peer: SocketAddr = "127.0.0.1:5000".parse().unwrap();
        let msg = describe(peer, Some(4242));
        assert!(msg.contains("4242"), "{msg}");
        assert!(msg.contains("127.0.0.1:5000"), "{msg}");
    }

    /// The `Option`-taking wrapper must not invent an answer for a socket with no peer.
    #[test]
    fn no_peer_address_means_no_pid() {
        assert_eq!(owning_pid_of(None), None);
    }
}
