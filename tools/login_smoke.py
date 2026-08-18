"""Talk to crates/login the way the client does, and check what comes back.

This is a *stand-in client*, not a stand-in server. It exists because the only other
oracle for the login server is the owner launching the real client from an elevated shell and
reading a screen, and that costs a manual run per change. This costs nothing and uses
`tools/transport.py` - an implementation written independently of the Rust one, from the
client's own receive path - so agreement between the two is real evidence rather than a
Rust module agreeing with itself.

What it proves, and what it does not:

  PROVES   the greeting parses, the framing and cipher interoperate in both directions,
           and the server answers each request with the opcodes and codes it should.
  DOES NOT the client's *reaction*. Only a launch shows whether a character is drawn
           correctly. Every packet-level fact in this repo that turned out to be wrong
           was wrong about a body the client read differently, not about a byte count.

Usage:

    python -u tools/login_smoke.py                      # the whole sequence
    python -u tools/login_smoke.py --name Smoke01       # pick the character name
    python -u tools/login_smoke.py --list-only          # just log in and list

Exit code 0 means every check passed; 1 means one did not, and the failing check is named.
"""

import argparse
import os
import socket
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import transport  # noqa: E402

DATA_WZ_PATCH = 0x0032
ACCOUNT_INFO = 0x0000
WORLD_LIST = 0x000B
LOGIN_RESULT = 0x0010
CHECK_NAME_RESULT = 0x0014
CREATE_RESULT = 0x0015
ENTER_CREATION_RESULT = 0x05F4

CLIENT_LOGIN_REQUEST = 0x0080
CLIENT_CHECK_NAME_REQUEST = 0x0081
CLIENT_LEAVE_WORLD_REQUEST = 0x0082
CLIENT_CREATE_REQUEST = 0x008A
CLIENT_ENTER_CREATION_REQUEST = 0x00A8

NAME_AVAILABLE = 0x00
NAME_ALREADY_USED = 0x7A

failures = []


def check(label, ok, detail=""):
    print(("  PASS  " if ok else "  FAIL  ") + label + ((" - " + detail) if detail else ""))
    if not ok:
        failures.append(label)
    return ok


class Peer:
    """One connection, with both cipher chains.

    The two directions are separate chains with separate constants, and neither is
    interchangeable with the other - a fact that took a while to establish and is worth
    keeping visible here.
    """

    def __init__(self, host, port, timeout):
        self.key, self.shuffle = transport.load_tables()
        self.sock = socket.create_connection((host, port), timeout=timeout)
        self.sock.settimeout(timeout)
        self.buf = bytearray()
        self.pending = []

        body = self._read_greeting()
        self.client_tx_iv, self.client_rx_iv = parse_greeting_ivs(body)
        # We encrypt with the IV the client would decrypt with, and vice versa.
        self.send_iv = struct.pack("<I", self.client_tx_iv)
        self.recv_iv = struct.pack("<I", self.client_rx_iv)

    def _read_greeting(self):
        while len(self.buf) < 2:
            self._fill()
        (length,) = struct.unpack("<H", bytes(self.buf[:2]))
        while len(self.buf) < 2 + length:
            self._fill()
        body = bytes(self.buf[2 : 2 + length])
        del self.buf[: 2 + length]
        return body

    def _fill(self):
        chunk = self.sock.recv(8192)
        if not chunk:
            raise SystemExit("the server closed the connection")
        self.buf += chunk

    def send(self, opcode, payload=b""):
        packet = struct.pack("<H", opcode) + payload
        head = transport.header(self.send_iv, len(packet), transport.SEND_CONST)
        self.sock.sendall(head + transport.ofb(packet, self.send_iv, self.key))
        self.send_iv = transport.next_iv(self.send_iv, self.shuffle)

    def recv(self, want=1):
        """Read until `want` whole packets are decoded. Returns [(opcode, body), ...]."""
        while len(self.pending) < want:
            self._decode()
            if len(self.pending) < want:
                self._fill()
        out, self.pending = self.pending[:want], self.pending[want:]
        return out

    def drain(self, wait=0.5):
        """Whatever the server has sent by now.

        It reads the socket first. An earlier version only decoded bytes already in the
        local buffer, so a packet sitting in the OS receive queue read as "nothing sent" -
        and because the packet was still there on the next read, every check after it was
        offset by one and failed. A drain that does not drain is worse than no drain.
        """
        previous = self.sock.gettimeout()
        self.sock.settimeout(wait)
        try:
            while True:
                chunk = self.sock.recv(8192)
                if not chunk:
                    break
                self.buf += chunk
        except socket.timeout:
            pass
        finally:
            self.sock.settimeout(previous)
        self._decode()
        out, self.pending = self.pending, []
        return out

    def _decode(self):
        while True:
            if len(self.buf) < 4:
                return
            a, b = struct.unpack("<HH", bytes(self.buf[:4]))
            length = a ^ b
            head = 4
            if length >= transport.EXTENDED_LEN:
                if len(self.buf) < 8:
                    return
                length = struct.unpack("<I", bytes(self.buf[4:8]))[0] ^ a
                head = 8
            if len(self.buf) < head + length:
                return

            expected = ((int.from_bytes(self.recv_iv, "little") >> 16) & 0xFFFF)
            expected = (expected ^ transport.RECV_CONST) & 0xFFFF
            if a != expected:
                raise SystemExit(
                    "header mismatch: the IV chains have desynced, so nothing after this "
                    "is readable (got 0x%04X, expected 0x%04X)" % (a, expected)
                )
            payload = bytes(self.buf[head : head + length])
            del self.buf[: head + length]
            plain = transport.ofb(payload, self.recv_iv, self.key)
            self.recv_iv = transport.next_iv(self.recv_iv, self.shuffle)
            opcode = struct.unpack("<H", plain[:2])[0] if len(plain) >= 2 else None
            self.pending.append((opcode, plain[2:]))

    def close(self):
        self.sock.close()


def parse_greeting_ivs(body):
    """Pull J and K out of the greeting, reading the strings rather than guessing."""
    at = 0

    def u16():
        nonlocal at
        v = struct.unpack("<H", body[at : at + 2])[0]
        at += 2
        return v

    def u32():
        nonlocal at
        v = struct.unpack("<I", body[at : at + 4])[0]
        at += 4
        return v

    def skip(n):
        nonlocal at
        at += n

    def mstr():
        skip(u16())

    u16()          # A
    mstr()         # B
    u32()          # C
    u32()          # D
    skip(2)        # E, F
    g = u16()
    h = u32()
    mstr()         # I
    j = u32()
    k = u32()
    ell = body[at]
    check("greeting gates G=1 H=1 L=1", (g, h, ell) == (1, 1, 1), "got G=%d H=%d L=%d" % (g, h, ell))
    return j, k


def create_payload(name, gender=1, skin=2, hair=30030, items=None):
    """A 0x008A body in the layout measured off the wire."""
    if items is None:
        items = [(1, 21002), (2, 30030), (3, 1040002), (4, 1060002), (5, 1072001), (6, 1302000)]
    p = struct.pack("<H", len(name)) + name.encode("ascii")
    p += struct.pack("<II", 0, 0)          # two discarded u32s
    p += struct.pack("<I", 0)              # race
    p += struct.pack("<H", 0)              # subJob
    p += struct.pack("<IIII", 12, 5, 4, 4)  # str, dex, int, luk
    p += struct.pack("<III", gender, skin, hair)
    p += struct.pack("<I", len(items))
    for category, item in items:
        p += struct.pack("<II", category, item)
    return p


def name_payload(name):
    return struct.pack("<H", len(name)) + name.encode("ascii")


def names_in(body):
    """Character names out of a login result, without decoding the whole record.

    The name is a fixed 13-byte block inside each record, so a run of printable ASCII
    followed by a NUL is a name. This is deliberately loose: the point is to see whether
    a character we created is present, not to re-implement the record reader.
    """
    found, run = [], bytearray()
    for byte in body:
        if 0x30 <= byte <= 0x7A and chr(byte).isalnum():
            run.append(byte)
        else:
            if len(run) >= 4:
                found.append(run.decode("ascii"))
            run = bytearray()
    if len(run) >= 4:
        found.append(run.decode("ascii"))
    return found


def login(peer):
    """Send the login request and return the four replies."""
    peer.send(CLIENT_LOGIN_REQUEST)
    replies = peer.recv(4)
    opcodes = [op for op, _ in replies]
    check(
        "login request is answered with account, world, end, result",
        opcodes == [ACCOUNT_INFO, WORLD_LIST, WORLD_LIST, LOGIN_RESULT],
        "got " + ", ".join("0x%04X" % o for o in opcodes),
    )
    return replies


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--port", type=int, default=8484)
    ap.add_argument("--name", default="Smoke01", help="character to create")
    ap.add_argument("--list-only", action="store_true", help="log in and list, create nothing")
    ap.add_argument("--check-quiet", action="store_true",
                    help="say nothing for a while and check the startup gate is repeated")
    ap.add_argument("--timeout", type=float, default=5.0)
    args = ap.parse_args()

    print("connecting to %s:%d" % (args.host, args.port))
    peer = Peer(args.host, args.port, args.timeout)
    print("greeting accepted; client_tx_iv=0x%08X client_rx_iv=0x%08X"
          % (peer.client_tx_iv, peer.client_rx_iv))

    # The startup gate arrives unprompted.
    gate = peer.recv(1)
    check("startup gate 0x0032 arrives unprompted", gate[0][0] == DATA_WZ_PATCH,
          "got 0x%04X" % gate[0][0])
    check("the gate says nothing to patch", gate[0][1] == b"\x00", gate[0][1].hex())

    if args.check_quiet:
        # The gate releases a recv loop on the client's UI thread. A client that missed it
        # shows a blank window, which looks exactly like the server not running - so the
        # repeat is worth proving rather than assuming.
        print("  saying nothing for 5s...")
        import time
        time.sleep(5.0)
        repeated = peer.drain()
        check("the startup gate is repeated to a quiet client",
              [op for op, _ in repeated] == [DATA_WZ_PATCH],
              "got " + (", ".join("0x%04X" % op for op, _ in repeated) or "nothing"))

    replies = login(peer)
    existing = names_in(replies[3][1])
    print("  characters already stored: %s" % (", ".join(existing) or "(none)"))

    if args.list_only:
        peer.close()
        return report()

    # Leaving the world is answered the same way, every time. A one-shot answer left the
    # client on "Connecting..." forever when a world was picked a second time.
    peer.send(CLIENT_LEAVE_WORLD_REQUEST)
    again = [op for op, _ in peer.recv(4)]
    check("leave-world is answered like a login", again == [ACCOUNT_INFO, WORLD_LIST, WORLD_LIST, LOGIN_RESULT],
          "got " + ", ".join("0x%04X" % o for o in again))

    peer.send(CLIENT_ENTER_CREATION_REQUEST, b"\x01\x00\x2e")
    opcode, body = peer.recv(1)[0]
    check("creation screen is permitted", opcode == ENTER_CREATION_RESULT and body == b"\x00\x00",
          "0x%04X %s" % (opcode, body.hex()))

    already_there = args.name in existing

    peer.send(CLIENT_CHECK_NAME_REQUEST, name_payload(args.name))
    opcode, body = peer.recv(1)[0]
    check("name check answers about the name that was asked",
          opcode == CHECK_NAME_RESULT and body[:2 + len(args.name)] == name_payload(args.name),
          body.hex())
    if already_there:
        check("a name already stored reads as taken", body[-1] == NAME_ALREADY_USED,
              "code 0x%02X" % body[-1])
        print("  %r already exists, so creation is skipped" % args.name)
        peer.close()
        return report()
    check("a free name reads as available", body[-1] == NAME_AVAILABLE, "code 0x%02X" % body[-1])

    peer.send(CLIENT_CREATE_REQUEST, create_payload(args.name))
    opcode, body = peer.recv(1)[0]
    check("create is accepted", opcode == CREATE_RESULT and body[0] == 0,
          "0x%04X result 0x%02X" % (opcode, body[0] if body else 0xFF))

    peer.send(CLIENT_CHECK_NAME_REQUEST, name_payload(args.name))
    opcode, body = peer.recv(1)[0]
    check("the name is taken immediately after creating it", body[-1] == NAME_ALREADY_USED,
          "code 0x%02X" % body[-1])

    replies = login(peer)
    now = names_in(replies[3][1])
    check("the new character is in the login result", args.name in now,
          "names seen: " + (", ".join(now) or "(none)"))
    peer.close()
    return report()


def report():
    print()
    if failures:
        print("%d check(s) FAILED: %s" % (len(failures), "; ".join(failures)))
        return 1
    print("all checks passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
