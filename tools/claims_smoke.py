"""Two people signed in on ONE machine, each served their own account. Over real sockets.

This is the acceptance test for per-launch login claims. It exists because the bug it
covers is invisible to every other instrument here:

    `store::claims::stake_login_claim` used to run `DELETE FROM login_claims` and insert one
    global row. A second sign-in EVICTED the first, and both login connections were then
    served as the second account - full character list, twelve-hour window, no race and no
    migration involved.

`tools/login_smoke.py` cannot see that: it signs one account in. `tools/channel_smoke.py`
cannot either: it never touches the auth service. The failure needs two sign-ins and two
connections from two different processes on one address, which is exactly what this does.

    python -u tools/claims_smoke.py

It builds a throwaway database in a temp directory, creates three accounts (two players and
a fallback), starts `maplecw-auth` and `maplecw-login` on free ports, signs both players in
over real HTTP, registers each player's connecting process, and then has two SEPARATE child
processes each open a real login connection and report which characters they were shown.

**It writes nothing to maplecw.db.** Everything happens in a temp directory that is deleted
on the way out - the rule `login_smoke.py` learned on 2026-08-18, when it created and
deleted characters in the real database.

Binaries come from `target/release`, or `MAPLECW_BIN_DIR` when a running server holds those
files open:

    CARGO_TARGET_DIR=target-smoke cargo build --release --workspace
    MAPLECW_BIN_DIR=target-smoke/release python -u tools/claims_smoke.py

# What each check means when it fails

  "each player's connection is served their OWN characters"
      The headline. A failure here is impersonation: one player is looking at another
      player's character list.

  "a second sign-in does not evict the first"
      The store-level cause. If this fails the claim table is back to one global row.

  "an unregistered connection is NOT guessed at"
      The refusal. With two claims live and a connection nobody can attribute, the server
      must fall back to its --account rather than pick the newest. A failure here means the
      guess is back, and the headline check above would then pass *by luck* on whichever
      player happened to sign in last.
"""
import json
import os
import shutil
import socket
import struct
import subprocess
import sys
import tempfile
import time

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, "tools")
import transport  # noqa: E402

ACCOUNT_INFO = 0x0000
WORLD_LIST = 0x000B
LOGIN_RESULT = 0x0010
CLIENT_LOGIN_REQUEST = 0x0080

PASSWORD = "correct horse battery staple"

fails = []


def check(label, ok, detail=""):
    print(("  PASS  " if ok else "  FAIL  ") + label + ((" - " + detail) if detail else ""))
    if not ok:
        fails.append(label)
    return ok


def free_port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


def bin_dir():
    return os.environ.get("MAPLECW_BIN_DIR") or os.path.join("target", "release")


def exe(name):
    return os.path.join(bin_dir(), name + (".exe" if os.name == "nt" else ""))


# --------------------------------------------------------------------- the login protocol


def parse_greeting_ivs(body):
    """J and K out of the greeting.

    Read field by field rather than by a fixed offset, and copied from
    `tools/login_smoke.py` deliberately: an offset picked by eye is exactly the kind of
    plausible-looking wrong number this project keeps paying for.
    """
    at = 0

    def u16():
        nonlocal at
        v = struct.unpack("<H", body[at:at + 2])[0]
        at += 2
        return v

    def u32():
        nonlocal at
        v = struct.unpack("<I", body[at:at + 4])[0]
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
    u16()          # G
    u32()          # H
    mstr()         # I
    j = u32()
    k = u32()
    return j, k


class Peer:
    """One login connection with both cipher chains, the same shape login_smoke.py uses."""

    def __init__(self, host, port, timeout=10.0):
        self.key, self.shuffle = transport.load_tables()
        self.sock = socket.create_connection((host, port), timeout=timeout)
        self.sock.settimeout(timeout)
        self.buf = bytearray()
        self.pending = []
        body = self._read_greeting()
        tx, rx = parse_greeting_ivs(body)
        self.send_iv = struct.pack("<I", tx)
        self.recv_iv = struct.pack("<I", rx)

    def _read_greeting(self):
        while len(self.buf) < 2:
            self._fill()
        (length,) = struct.unpack("<H", bytes(self.buf[:2]))
        while len(self.buf) < 2 + length:
            self._fill()
        body = bytes(self.buf[2:2 + length])
        del self.buf[:2 + length]
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

    def _decode(self):
        """Frame, check the header against the IV chain, decrypt, advance.

        The header check is not decoration: once the two chains desync nothing after it is
        readable, and without this the failure surfaces as a garbage opcode several packets
        later. Same guard as `tools/login_smoke.py`.
        """
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
                    "header mismatch: the IV chains have desynced, so nothing after this is "
                    "readable (got 0x%04X, expected 0x%04X)" % (a, expected)
                )
            payload = bytes(self.buf[head:head + length])
            del self.buf[:head + length]
            plain = transport.ofb(payload, self.recv_iv, self.key)
            self.recv_iv = transport.next_iv(self.recv_iv, self.shuffle)
            opcode = struct.unpack("<H", plain[:2])[0] if len(plain) >= 2 else None
            self.pending.append((opcode, plain[2:]))

    def recv(self, want=1):
        while len(self.pending) < want:
            self._decode()
            if len(self.pending) < want:
                self._fill()
        out, self.pending = self.pending[:want], self.pending[want:]
        return out

    def close(self):
        try:
            self.sock.close()
        except OSError:
            pass


def names_in(body):
    """Character names out of a login result. Deliberately loose - the same reader
    login_smoke.py uses, and for the same reason: the question is which names are present,
    not how the record is laid out."""
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


def characters_seen(host, port):
    """Open a login connection, log in, and report the character names it was shown."""
    peer = Peer(host, port)
    try:
        peer.recv(1)                       # the unprompted startup gate, 0x0032
        peer.send(CLIENT_LOGIN_REQUEST)
        replies = peer.recv(4)
        opcodes = [op for op, _ in replies]
        if opcodes != [ACCOUNT_INFO, WORLD_LIST, WORLD_LIST, LOGIN_RESULT]:
            return {"error": "unexpected reply sequence " + repr(opcodes)}
        return {"names": names_in(replies[3][1])}
    finally:
        peer.close()


# ------------------------------------------------------------------- the child-process mode
#
# The whole point is that the two connections come from two DIFFERENT processes, because the
# process is the discriminator. Re-invoking this file is the cheapest way to get one: it
# needs no extra fixture on disk and the pid `Popen` reports really is the pid that owns the
# socket, with no shell wrapper in between.
#
# It waits for a line on stdin before connecting, so the parent can register the pid FIRST.
# Without that the connection could arrive before the claim knew about it, and the test would
# be measuring a race rather than the mechanism.

if len(sys.argv) > 1 and sys.argv[1] == "--connect":
    host, port = sys.argv[2], int(sys.argv[3])
    sys.stdin.readline()
    print(json.dumps(characters_seen(host, port)), flush=True)
    raise SystemExit(0)


# --------------------------------------------------------------------------- the auth client


def http_post(port, path, body):
    request = (
        "POST %s HTTP/1.1\r\nHost: 127.0.0.1:%d\r\nContent-Type: application/json\r\n"
        "Content-Length: %d\r\nConnection: close\r\n\r\n%s" % (path, port, len(body), body)
    )
    s = socket.create_connection(("127.0.0.1", port), timeout=30)
    try:
        s.sendall(request.encode())
        out = b""
        while True:
            chunk = s.recv(8192)
            if not chunk:
                break
            out += chunk
    finally:
        s.close()
    text = out.decode("utf-8", "replace")
    head, _, payload = text.partition("\r\n\r\n")
    status = head.splitlines()[0] if head else ""
    try:
        return status, json.loads(payload)
    except ValueError:
        return status, {"raw": payload}


# ------------------------------------------------------------------------------- the fixture


class Servers:
    def __init__(self):
        self.dir = tempfile.mkdtemp(prefix="maplecw-claims-smoke-")
        self.db = os.path.join(self.dir, "smoke.db")
        self.auth_port = free_port()
        self.login_port = free_port()
        self.channel_port = free_port()

        for name in ("maplecw-useradd", "maplecw-auth", "maplecw-login"):
            if not os.path.exists(exe(name)):
                raise SystemExit(
                    "%s is missing - run: cargo build --release --workspace" % exe(name)
                )

        # Three accounts. The fallback is a THIRD one, deliberately: if it were one of the
        # two players, a connection that fell back would be indistinguishable from one that
        # resolved correctly, and the headline check would pass for the wrong reason.
        for who, character in (("otter", "OtterOne"), ("owl", "OwlTwo"), ("fallback", None)):
            r = subprocess.run(
                [exe("maplecw-useradd"), "--db", self.db, who],
                input=PASSWORD + "\n", text=True, capture_output=True,
            )
            if r.returncode != 0:
                raise SystemExit("could not create %s: %s" % (who, r.stderr.strip()))
            if character:
                self._make_character(who, character)

        self.auth_log = open(os.path.join(self.dir, "auth.log"), "w")
        self.auth = subprocess.Popen(
            [exe("maplecw-auth"), "--db", self.db, "--port", str(self.auth_port)],
            stdout=self.auth_log, stderr=subprocess.STDOUT,
        )
        self.login_log = open(os.path.join(self.dir, "login.log"), "w")
        self.login = subprocess.Popen(
            [exe("maplecw-login"), "--db", self.db,
             "--account", "fallback",
             "--bind", "127.0.0.1:%d" % self.login_port,
             "--channels", "127.0.0.1:%d" % self.channel_port],
            stdout=self.login_log, stderr=subprocess.STDOUT,
        )
        self._wait(self.auth_port, self.auth, "auth")
        self._wait(self.login_port, self.login, "login")
        print("throwaway auth on %d, login on %d, database %s"
              % (self.auth_port, self.login_port, self.db))

    def _make_character(self, account, name):
        """A character each, straight into the database.

        Written with sqlite3 rather than through the protocol because the point of this
        script is WHICH account a connection is served as, and creating a character over the
        wire would need a connection that has already been resolved - the very thing under
        test.
        """
        import sqlite3
        c = sqlite3.connect(self.db)
        acct = c.execute("SELECT id FROM accounts WHERE name = ?", (account,)).fetchone()[0]
        c.execute(
            "INSERT INTO characters (account_id, world_id, name, gender, skin, face, hair, "
            "level, job, strength, dexterity, intelligence, luck, hp, max_hp, mp, max_mp, "
            "ap, map_id, created_at) "
            "VALUES (?, 0, ?, 0, 0, 20000, 30030, 1, 0, 12, 5, 4, 4, 50, 50, 5, 5, 0, 1, 0)",
            (acct, name),
        )
        c.commit()
        c.close()

    def _wait(self, port, proc, what):
        for _ in range(80):
            if proc.poll() is not None:
                raise SystemExit("the throwaway %s server exited immediately" % what)
            try:
                socket.create_connection(("127.0.0.1", port), timeout=0.2).close()
                return
            except OSError:
                time.sleep(0.1)
        raise SystemExit("the throwaway %s server never accepted a connection" % what)

    def close(self):
        for proc in (self.auth, self.login):
            proc.terminate()
            try:
                proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                proc.kill()
        self.auth_log.close()
        self.login_log.close()
        # The login server's own log is the only record of which account each connection was
        # served as, and it is what a failure has to be read against. Print it before the
        # directory goes.
        try:
            with open(os.path.join(self.dir, "login.log"), encoding="utf-8", errors="replace") as f:
                served = [l.rstrip() for l in f if "served as" in l or "claim" in l.lower()]
            if served:
                print("\nthe login server said:")
                for line in served:
                    print("   " + line)
        except OSError:
            pass
        shutil.rmtree(self.dir, ignore_errors=True)


def main():
    s = Servers()
    children = []
    try:
        # 1. Both players sign in, over real HTTP, from the same address.
        handles = {}
        for who in ("otter", "owl"):
            status, body = http_post(
                s.auth_port, "/login",
                json.dumps({"username": who, "password": PASSWORD}),
            )
            ok = "200" in status and body.get("status") == "ok"
            check("%s signs in through the auth service" % who, ok, status.strip())
            handles[who] = body.get("launch_id", "")
        check(
            "each sign-in hands back its own launch handle",
            bool(handles["otter"]) and bool(handles["owl"])
            and handles["otter"] != handles["owl"],
            "otter %s, owl %s" % (
                "yes" if handles["otter"] else "MISSING",
                "yes" if handles["owl"] else "MISSING"),
        )

        # 2. The store-level property: the second sign-in did not remove the first.
        r = subprocess.run([exe("maplecw-useradd"), "--db", s.db, "--claims"],
                           capture_output=True, text=True)
        live = r.stdout
        check(
            "a second sign-in does not evict the first",
            "otter" in live and "owl" in live,
            live.strip().replace("\n", " | "),
        )

        # 3. Two child processes, each registered to one player, each opening its own
        #    connection from the SAME address. This is the owner's machine.
        for who in ("otter", "owl"):
            child = subprocess.Popen(
                [sys.executable, "-u", os.path.join("tools", "claims_smoke.py"),
                 "--connect", "127.0.0.1", str(s.login_port)],
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True,
            )
            children.append((who, child))
            status, body = http_post(
                s.auth_port, "/launch",
                json.dumps({"launch_id": handles[who], "pid": child.pid}),
            )
            check(
                "%s's client process is registered with the server" % who,
                body.get("status") == "bound" and body.get("pid") == child.pid,
                "pid %d -> %s" % (child.pid, body.get("status")),
            )

        # 4. Release them one at a time, so the two connections cannot interleave in a way
        #    that makes a failure ambiguous.
        seen = {}
        for who, child in children:
            child.stdin.write("go\n")
            child.stdin.flush()
            line = child.stdout.readline()
            seen[who] = json.loads(line) if line.strip() else {"error": "no answer"}

        otter = seen["otter"].get("names", [])
        owl = seen["owl"].get("names", [])
        crossed = "OwlTwo" in otter or "OtterOne" in owl
        own = check(
            "each player's connection is served their OWN characters",
            "OtterOne" in otter and "OwlTwo" not in otter
            and "OwlTwo" in owl and "OtterOne" not in owl,
            "otter saw %s; owl saw %s" % (otter or ["(none)"], owl or ["(none)"]),
        )
        if not own:
            # The two failures look identical in the summary line and mean opposite things.
            # Saying which is which here is the difference between "go and wire it" and
            # "stop, somebody is being impersonated".
            if crossed:
                print("         ^ IMPERSONATION: one player was served the other's character "
                      "list. This is the bug, live.")
            elif not otter and not owl:
                print("         ^ NOT impersonation - neither player saw anyone's characters, "
                      "so both fell back.")
                print("           That is the REFUSAL working and the RESOLUTION unwired: "
                      "crates/login/src/server.rs")
                print("           still calls store.current_login_claim(), which cannot tell "
                      "two claims apart and")
                print("           correctly answers None. It needs "
                      "store.resolve_login_claim(&ClaimEvidence{..}) with")
                print("           the pid from store::peerowner::owning_pid_of(peer_addr). "
                      "Nobody is impersonatable")
                print("           in this state; nobody can play their own account either.")

        # 5. The refusal. A connection from THIS process is registered to nobody, and with
        #    two claims live it must be served the fallback rather than the newest claim.
        #    Without this the check above could pass by luck for whichever player signed in
        #    last, which is exactly how the old code would have looked on a good day.
        mine = characters_seen("127.0.0.1", s.login_port).get("names", [])
        check(
            "an unregistered connection is NOT guessed at",
            "OtterOne" not in mine and "OwlTwo" not in mine,
            "an unattributable connection saw %s (the fallback account has none)"
            % (mine or ["(none)"]),
        )
    finally:
        for _, child in children:
            try:
                child.kill()
            except OSError:
                pass
        s.close()

    print()
    if fails:
        print("FAILED: " + ", ".join(fails))
        return 1
    print("all checks passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
