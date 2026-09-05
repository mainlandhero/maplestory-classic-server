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

    python -u tools/login_smoke.py --spawn     # the whole sequence, throwaway database
    python -u tools/login_smoke.py --list-only # read-only, against a server already running

**Use --spawn.** These checks CREATE AND DELETE CHARACTERS, so pointing them at a running
server writes to whatever database it opened. --spawn builds a throwaway database in a temp
directory, creates an account in it, starts a server on a free port, runs everything there,
and deletes the lot. Without it the mutating checks refuse to run, because the alternative
is what happened on 2026-08-18: this script created and deleted characters in the real
maplecw.db. --list-only is always safe - it logs in and reads.

Exit code 0 means every check passed; 1 means one did not, and the failing check is named.
"""

import argparse
import os
import shutil
import socket
import struct
import subprocess
import sys
import tempfile
import time

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
CLIENT_DELETE_REQUEST = 0x008B
CLIENT_SELECT_CHARACTER_REQUEST = 0x0078

MIGRATE_COMMAND = 0x0011
DELETE_RESULT = 0x0016
DELETE_OK = 0x00
DELETE_FAILED = 0x06
CREATE_INSUFFICIENT_SLOT = 0x09

MIGRATE_OK = 0x00
MIGRATE_REFUSED = 0x0A
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


def first_character_id(body):
    """The first id out of a login result's display-order list.

    Counted from the layout in crates/net/src/opcode.rs rather than guessed - an offset
    picked by eye read 1700031589, which is what a wrong offset looks like:

      u8 result | str message | u8 | 8B FILETIME | u32 world | u32 channel
      | 4B 4B 4B | u32 | u8            -> 37 bytes when the message is empty
      | u32 deletionCount (0)          -> 41
      | u32 orderCount, then that many u32 ids
    """
    message_len = struct.unpack("<H", body[1:3])[0]
    at = 37 + message_len
    deletions = struct.unpack("<I", body[at:at + 4])[0]
    at += 4 + deletions * 12          # each is a u32 id and an 8-byte FILETIME
    order = struct.unpack("<I", body[at:at + 4])[0]
    if order == 0:
        return None
    return struct.unpack("<I", body[at + 4:at + 8])[0]


def select_payload(character_id, pic="."):
    """The select-character body: a leading u32, the PIC as a string, then the id."""
    pic = pic.encode()
    return (struct.pack("<I", 0)
            + struct.pack("<H", len(pic)) + pic
            + struct.pack("<I", character_id)
            + bytes([0]))


def migrate_tail_word_forward(raw, key, offset):
    """The client's in-place transform for one aligned word of the migration tail.

    Transcribed from FUN_141b36f60, independently of crates/net - so agreement between
    this and the Rust builder is two implementations agreeing, not one checking itself.
    """
    m = 0xFFFFFFFF
    t = (((key ^ raw) + 0x369F144D + (key >> 7)) & m) ^ 0xAAAABBBB
    return (t - ((offset * key) & m)) & m


def check_migration(body, expect_ip, expect_port, expect_id):
    """Read the migration packet back the way FUN_141b36f60 does."""
    check("the migration is accepted", body[0] == MIGRATE_OK,
          "result 0x%02X" % body[0])
    message_len = struct.unpack("<H", body[1:3])[0]
    check("the migration message is empty", message_len == 0, "len %d" % message_len)
    at = 1 + 2 + message_len + 1
    ip = ".".join(str(b) for b in body[at:at + 4])
    port = struct.unpack("<H", body[at + 4:at + 6])[0]
    cid = struct.unpack("<I", body[at + 6:at + 10])[0]
    check("the migration carries the CHANNEL address, not the login server's",
          (ip, port) == (expect_ip, expect_port), "%s:%d" % (ip, port))
    check("the migration names the character we chose", cid == expect_id, "id %d" % cid)

    special = struct.unpack("<I", body[at + 14:at + 18])[0]
    check("the migration does not ask for SpecialServerInfo.img", special == 0,
          "field is %#x" % special)

    # The tail: key, length, then `length` obfuscated bytes.
    tail = at + 22 + 21
    key, blob_len = struct.unpack("<II", body[tail:tail + 8])
    raw = struct.unpack("<I", body[tail + 8:tail + 8 + 4])[0]
    check("the migration tail is one aligned word", blob_len == 4, "length %d" % blob_len)
    check("the migration body is exactly what the client reads", len(body) == tail + 12,
          "%d bytes, expected %d" % (len(body), tail + 12))
    seed = migrate_tail_word_forward(raw, key, 0)
    # The seed is minted in the database now, so its value is not predictable here. What
    # is checkable is that it decodes to something a channel could claim: non-zero, and
    # stable across a re-read of the same packet.
    check("the seed decodes to a non-zero value", seed != 0, "decoded %#010x" % seed)
    return seed


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


def free_port():
    with socket.socket() as probe:
        probe.bind(("127.0.0.1", 0))
        return probe.getsockname()[1]


class Throwaway:
    """A temp database, an account in it, and a server - all torn down on the way out.

    Exists so the safe way to run the mutating checks is also the easy way. They were
    pointed at the real maplecw.db once, which created and deleted characters in it.
    """

    def __init__(self, root):
        self.dir = tempfile.mkdtemp(prefix="maplecw-smoke-")
        self.db = os.path.join(self.dir, "smoke.db")
        self.port = free_port()
        # A channel is a separate process and a separate port. Nothing listens on it in
        # the smoke test - the point here is that the login server hands out the right
        # address, not that the world answers.
        self.channel_port = free_port()
        # `target/release` unless MAPLECW_BIN_DIR says otherwise. A server left running
        # from a client launch holds these files open, and on Windows that makes
        # `cargo build --release` fail with "Access is denied" - so without the override
        # this suite would silently check the binary from BEFORE the change. Same escape
        # hatch as tools/channel_smoke.py, and the same reason.
        built = os.environ.get("MAPLECW_BIN_DIR") or os.path.join(root, "target", "release")
        useradd = os.path.join(built, "maplecw-useradd.exe")
        login_exe = os.path.join(built, "maplecw-login.exe")
        for path in (useradd, login_exe):
            if not os.path.exists(path):
                raise SystemExit("%s is missing - run: cargo build --release" % path)

        r = subprocess.run([useradd, "--db", self.db, "maplecw"],
                           input="correct horse battery staple" + chr(10),
                           text=True, capture_output=True)
        if r.returncode != 0:
            raise SystemExit("throwaway account failed: " + r.stderr.strip())

        self.log = open(os.path.join(self.dir, "login.log"), "w")
        # --fallback-account, by name: this script is a stand-in client that stakes no
        # launcher claim, and login is enforced by default since 2026-09-05 - without the
        # flag every connection here would be refused with a login failure, which is the
        # server working, not the thing under test.
        self.proc = subprocess.Popen(
            [login_exe, "--db", self.db,
             "--fallback-account", "maplecw",
             "--bind", "127.0.0.1:%d" % self.port,
             "--channels", "127.0.0.1:%d" % self.channel_port],
            stdout=self.log, stderr=subprocess.STDOUT)
        for _ in range(50):
            if self.proc.poll() is not None:
                raise SystemExit("the throwaway server exited immediately")
            try:
                socket.create_connection(("127.0.0.1", self.port), timeout=0.2).close()
                break
            except OSError:
                time.sleep(0.1)
        else:
            raise SystemExit("the throwaway server never accepted a connection")
        print("throwaway server on port %d, channel 0 advertised at %d, database %s"
              % (self.port, self.channel_port, self.db))

    def close(self):
        self.proc.terminate()
        try:
            self.proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            self.proc.kill()
        self.log.close()
        shutil.rmtree(self.dir, ignore_errors=True)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--port", type=int, default=8484)
    ap.add_argument("--spawn", action="store_true",
                    help="start a server on a throwaway database and test against it")
    ap.add_argument("--i-know-this-writes-to-a-live-server", action="store_true",
                    dest="allow_live", help="run the mutating checks against --port anyway")
    ap.add_argument("--name", default="Smoke01", help="character to create")
    ap.add_argument("--list-only", action="store_true", help="log in and list, create nothing")
    ap.add_argument("--check-quiet", action="store_true",
                    help="say nothing for a while and check the startup gate is repeated")
    ap.add_argument("--channel-port", type=int, default=8485,
                    help="where channel 0 is advertised; --spawn picks its own")
    ap.add_argument("--timeout", type=float, default=5.0)
    args = ap.parse_args()

    throwaway = None
    if args.spawn:
        root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        throwaway = Throwaway(root)
        args.host, args.port = "127.0.0.1", throwaway.port
        args.channel_port = throwaway.channel_port
    elif not args.list_only and not args.allow_live:
        raise SystemExit(
            "refusing to run: these checks create and delete characters, and port %d may be"
            " a live server." % args.port + chr(10)
            + "  --spawn      test against a throwaway database (do this)" + chr(10)
            + "  --list-only  read without writing" + chr(10)
            + "  --i-know-this-writes-to-a-live-server  override")

    try:
        return run(args)
    finally:
        if throwaway is not None:
            throwaway.close()


def run(args):

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

    # 0x0073, EXACTLY as the stock client sends it.
    #
    # This is the compatibility check for the client-token credential added on 2026-08-29.
    # The login server now reads the identity string out of this packet and resolves the
    # account from it, and the identity is EMPTY in all 72 bodies this project has ever
    # captured - so "empty means nothing was presented" is the property every existing
    # client depends on. If it ever counted as a presentation, the server would refuse it,
    # downgrade the connection to --account, and on screen that reads as "my characters
    # vanished" rather than as a session bug.
    #
    # The body is the real one from previous-runs/login-20260829-094630.log.
    # `tools/claims_smoke.py` section 6 is where a NON-empty token is exercised; it needs two
    # accounts and the auth service, which this script does not run.
    captured_0073 = bytes.fromhex("050000000000d843ae4c5617b6ae9cd200000000764d00000000")
    peer.send(0x0073, captured_0073)
    unanswered = peer.drain(wait=0.4)
    check("0x0073 is not answered - the client does not block on it and never has",
          not unanswered,
          "got " + (", ".join("0x%04X" % op for op, _ in unanswered) or "nothing"))
    after_identity = names_in(login(peer)[3][1])
    check("an EMPTY 0x0073 identity leaves the account exactly as it was",
          after_identity == existing,
          "before: %s; after: %s" % (existing or ["(none)"], after_identity or ["(none)"]))

    if args.list_only:
        peer.close()
        return report()

    # Leaving the world is answered the same way, every time. A one-shot answer left the
    # client on "Connecting..." forever when a world was picked a second time.
    #
    # It stops at the world-list TERMINATOR and sends no login result, and that is
    # deliberate: the terminator transitions the client to screen 2, WorldSelect, and a login
    # result after it drags it to screen 4, CharSelect. That is why "Choose another world"
    # used to land back on the character screen.
    peer.send(CLIENT_LEAVE_WORLD_REQUEST)
    again = [op for op, _ in peer.recv(3)]
    check("leave-world is answered with the world list and NO login result",
          again == [ACCOUNT_INFO, WORLD_LIST, WORLD_LIST],
          "got " + ", ".join("0x%04X" % o for o in again))
    check("leave-world ends on the terminator, so the client stays on WorldSelect",
          again and again[-1] == WORLD_LIST, "last was 0x%04X" % (again[-1] if again else 0))

    # Picking a world on the WorldSelect screen. Unanswered, the client sits on
    # "Connecting to server..." forever - exactly what happened the first time that
    # screen became reachable.
    peer.send(0x0076, bytes.fromhex("00000000020000007f000001") + b"\x00\x00")
    picked = [op for op, _ in peer.recv(4)]
    check("selecting a world is answered with the character list",
          picked == [ACCOUNT_INFO, WORLD_LIST, WORLD_LIST, LOGIN_RESULT],
          "got " + ", ".join("0x%04X" % o for o in picked))

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
    if opcode == CREATE_RESULT and body and body[0] == CREATE_INSUFFICIENT_SLOT:
        # Not a failure: the account is full and the server said so, which is the
        # three-character limit working. Anything after this would measure nothing.
        check("a full account is refused with the slot code", True,
              "%d already: %s" % (len(existing), ", ".join(existing)))
        peer.close()
        return report()
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

    # Entering the world, while the character still exists. The client looks the id up in
    # its own map and skips everything in silence on a miss, so a refusal that says so is
    # the only way that case is ever visible.
    peer.send(CLIENT_SELECT_CHARACTER_REQUEST, select_payload(0xDEADBEEF))
    opcode, body = peer.recv(1)[0]
    check("selecting a character we do not own is refused, not ignored",
          opcode == MIGRATE_COMMAND and body[0] == MIGRATE_REFUSED,
          "0x%04X code 0x%02X" % (opcode, body[0] if body else 0xFF))

    channel_port = args.channel_port
    mine = first_character_id(replies[3][1])
    if mine is None:
        check("a character id to migrate", False, "the login result listed none")
    else:
        peer.send(CLIENT_SELECT_CHARACTER_REQUEST, select_payload(mine))
        opcode, body = peer.recv(1)[0]
        check("selecting our own character is answered with a migration",
              opcode == MIGRATE_COMMAND, "0x%04X" % opcode)
        if opcode == MIGRATE_COMMAND:
            first = check_migration(body, args.host, channel_port, mine)

            # Single use: entering the world twice mints two migrations, and the second
            # must not reuse the first seed - a replayed handoff would otherwise put a
            # second connection into the world as the same character.
            peer.send(CLIENT_SELECT_CHARACTER_REQUEST, select_payload(mine))
            opcode, body = peer.recv(1)[0]
            second = check_migration(body, args.host, channel_port, mine)
            check("a second migration mints a different seed", first != second,
                  "%#010x then %#010x" % (first or 0, second or 0))

    # Delete is one u32 and nothing else - the confirmation is a client-side dialog, so the
    # server cannot tell a confirmed delete from a forged one and ownership is all there is.
    # A refusal must use code 6 specifically: every other non-zero code falls through the
    # client's switch to the branch that removes the character from the list anyway.
    peer.send(CLIENT_DELETE_REQUEST, struct.pack("<I", 0xDEADBEEF))
    opcode, body = peer.recv(1)[0]
    check("a delete we do not own is refused with the code that does NOT delete",
          opcode == DELETE_RESULT and body[-1] == DELETE_FAILED,
          "0x%04X code 0x%02X" % (opcode, body[-1] if body else 0xFF))

    ours = first_character_id(replies[3][1])
    peer.send(CLIENT_DELETE_REQUEST, struct.pack("<I", ours or 0))
    opcode, body = peer.recv(1)[0]
    deleted = opcode == DELETE_RESULT and body[-1] == DELETE_OK
    check("our own character is deleted", deleted,
          "0x%04X code 0x%02X (id %s)" % (opcode, body[-1] if body else 0xFF, ours))
    if deleted:
        after = names_in(login(peer)[3][1])
        check("the deleted character is gone from the login result", args.name not in after,
              "names seen: " + (", ".join(after) or "(none)"))
        peer.send(CLIENT_CHECK_NAME_REQUEST, name_payload(args.name))
        check("its name is free again", peer.recv(1)[0][1][-1] == NAME_AVAILABLE)

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
