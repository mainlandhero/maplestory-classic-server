//! **A channel connection's reads, on their own thread, with no socket timeout.**
//!
//! The owner, 2026-10-02: *"I (Tester) was in a party quest with Cate when I randomly got disconnected
//! from the server."* The server ended the session itself:
//!
//! ```text
//! 04:56:52.143 ch0 #2 ...:52207 ended: Overlapped I/O operation is in progress. (os error 997)
//! ```
//!
//! The session thread used to read with `SO_RCVTIMEO` = 100 ms so it could wake to tick
//! (chatter, regen, buffs, the presence lease). Winsock implements a blocking `recv` on an
//! overlapped socket - which every Rust socket on Windows is - as an overlapped receive plus a
//! wait, and when the timeout expires as data arrives it can hand back `ERROR_IO_PENDING` (997)
//! instead of `WSAETIMEDOUT`. The loop accepted only `WouldBlock`/`TimedOut` as "nothing
//! arrived", so the 997 ended the session. **Measured, not just reasoned:** the client was
//! sending a move every ~510 ms (50.097, 50.597, 51.107, 51.617) and the error landed at
//! ~52.13, the moment the next one was due. The archive has one other 997, 2026-09-18, ending a
//! session the same way. Microsoft's own documentation of `SO_RCVTIMEO` says a timed-out
//! blocking receive leaves the connection in an indeterminate state.
//!
//! So nothing times a socket read any more. This thread does plain blocking reads and passes
//! bytes over a channel; the session waits on the channel with the same tick, and a timeout
//! there is an ordinary channel timeout that cannot touch the socket. The socket is shut down
//! when the [`SocketReader`] drops, which is what unblocks the thread on every return path.

use std::io::Read;
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError};
use std::time::Duration;

/// One thing the reader thread saw.
#[derive(Debug)]
pub enum Inbound {
    /// Bytes, in the order they arrived.
    Data(Vec<u8>),
    /// Nothing arrived within the tick. The session ticks and asks again.
    Idle,
    /// The client closed the connection (a zero-byte read).
    Closed,
    /// The socket failed - os error 10054 is the client crashing.
    Failed(std::io::Error),
}

/// The reading half of a channel connection. Shuts the socket down when dropped.
pub struct SocketReader {
    rx: Receiver<Inbound>,
    socket: TcpStream,
}

impl SocketReader {
    /// Start reading `stream` (a clone of the session's socket) on its own thread.
    pub fn spawn(stream: TcpStream) -> std::io::Result<Self> {
        let socket = stream.try_clone()?;
        let (tx, rx) = mpsc::channel();
        std::thread::Builder::new().name("sock-read".into()).spawn(move || {
            let mut stream = stream;
            let mut buf = [0u8; 8192];
            loop {
                let event = match stream.read(&mut buf) {
                    Ok(0) => Inbound::Closed,
                    Ok(n) => Inbound::Data(buf[..n].to_vec()),
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                    Err(e) => Inbound::Failed(e),
                };
                let last = !matches!(event, Inbound::Data(_));
                if tx.send(event).is_err() || last {
                    return;
                }
            }
        })?;
        Ok(SocketReader { rx, socket })
    }

    /// The next event, waiting at most `tick`. A reader thread that has gone away without
    /// saying why reads as a close.
    pub fn next(&self, tick: Duration) -> Inbound {
        match self.rx.recv_timeout(tick) {
            Ok(event) => event,
            Err(RecvTimeoutError::Timeout) => Inbound::Idle,
            Err(RecvTimeoutError::Disconnected) => Inbound::Closed,
        }
    }
}

impl Drop for SocketReader {
    fn drop(&mut self) {
        // The thread holds its own handle to the socket, so dropping the session's stream does
        // not close the connection; this does, and wakes the blocked read.
        let _ = self.socket.shutdown(Shutdown::Both);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::net::TcpListener;

    fn pair() -> (TcpStream, TcpStream) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let client = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
        let (server, _) = listener.accept().unwrap();
        (server, client)
    }

    const TICK: Duration = Duration::from_millis(50);

    fn data_within(r: &SocketReader, want: usize) -> Vec<u8> {
        let mut got = Vec::new();
        for _ in 0..100 {
            match r.next(TICK) {
                Inbound::Data(b) => got.extend(b),
                Inbound::Idle => {}
                other => panic!("{other:?}"),
            }
            if got.len() >= want {
                break;
            }
        }
        got
    }

    /// Idle ticks, then the bytes, then a close - and a quiet socket is never an error.
    #[test]
    fn idle_ticks_then_data_then_a_close() {
        let (server, mut client) = pair();
        let r = SocketReader::spawn(server).unwrap();
        for _ in 0..5 {
            assert!(matches!(r.next(TICK), Inbound::Idle), "a quiet client is not a disconnect");
        }
        client.write_all(b"hello").unwrap();
        assert_eq!(data_within(&r, 5), b"hello");
        drop(client);
        let end = (0..100).map(|_| r.next(TICK)).find(|e| !matches!(e, Inbound::Idle)).unwrap();
        assert!(matches!(end, Inbound::Closed | Inbound::Failed(_)), "{end:?}");
    }

    /// **Data arriving right at the tick boundary is never lost and never fatal** - the race
    /// that produced os error 997. Many small writes timed to land around the tick.
    #[test]
    fn bytes_landing_on_the_tick_boundary_all_arrive() {
        let (server, mut client) = pair();
        let r = SocketReader::spawn(server).unwrap();
        let writer = std::thread::spawn(move || {
            for i in 0..40u8 {
                std::thread::sleep(Duration::from_millis(49 + u64::from(i % 3)));
                client.write_all(&[i]).unwrap();
            }
            client
        });
        let got = data_within(&r, 40);
        assert_eq!(got, (0..40u8).collect::<Vec<_>>());
        drop(writer.join().unwrap());
    }

    /// Dropping the reader closes the connection, so the client sees it end and the thread
    /// blocked in `read` wakes.
    #[test]
    fn dropping_the_reader_closes_the_connection() {
        let (server, mut client) = pair();
        let r = SocketReader::spawn(server).unwrap();
        drop(r);
        client.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let mut b = [0u8; 1];
        assert!(matches!(client.read(&mut b), Ok(0) | Err(_)), "the client sees the close");
    }
}
