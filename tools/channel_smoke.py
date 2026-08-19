"""Talk to maplecw-world the way the client does, to prove the channel path works.

Uses `tools/transport.py` - the INDEPENDENT Python implementation of the framing, the IV
chain and AES - so a pass is two implementations agreeing rather than the Rust checking
itself. No client launch is spent.

**Rewritten 2026-08-19.** The previous version asserted a byte-shift cipher and a
"POLARITY CHECK" log block, both of which were removed when the channel was measured to be
AES like the login connection. It would have failed every check for the wrong reason - a
stale instrument reporting a real regression.

    python tools/channel_smoke.py
"""
import os
import socket
import struct
import subprocess
import sys
import tempfile
import time

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, "tools")
import transport  # noqa: E402

fails = []


def check(label, ok, detail=""):
    print(("  PASS  " if ok else "  FAIL  ") + label + (" - " + detail if detail else ""))
    if not ok:
        fails.append(label)


def free_port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


# The character id a real client sent in 0x007D, from the capture in
# crates/world/src/session.rs. Nothing has minted a migration for it here, so the server
# should say so plainly rather than accepting it - that honest negative is the check.
CHARACTER_ID = 204
CLIENT_IV = 0x52307801        # J, the chain the client encrypts with
SERVER_IV = 0x52307802        # K, the chain it decrypts with

key, shuffle = transport.load_tables()

tmp = tempfile.mkdtemp(prefix="maplecw-ch-")
db = os.path.join(tmp, "ch.db")
port = free_port()
subprocess.run([os.path.join("target", "release", "maplecw-useradd.exe"), "--db", db, "maplecw"],
               input="correct horse battery staple\n", text=True, capture_output=True)
logpath = os.path.join(tmp, "world.log")
logf = open(logpath, "w")
proc = subprocess.Popen([os.path.join("target", "release", "maplecw-world.exe"), "--db", db,
                         "--bind", "127.0.0.1:%d" % port],
                        stdout=logf, stderr=subprocess.STDOUT)
replies = []
try:
    sock = None
    for _ in range(50):
        try:
            sock = socket.create_connection(("127.0.0.1", port), timeout=2)
            break
        except OSError:
            time.sleep(0.1)
    if sock is None:
        raise SystemExit("channel server never accepted a connection")

    sock.settimeout(3)
    head = sock.recv(2)
    n = struct.unpack("<H", head)[0]
    body = b""
    while len(body) < n:
        body += sock.recv(n - len(body))

    check("the channel greeting body is 20 bytes, not the login server's 46", n == 20, "%d" % n)
    g = struct.unpack_from("<H", body, 0)[0]
    h = struct.unpack_from("<I", body, 2)[0]
    check("G is 1 - the gate that raised 'the client is outdated'", g == 1, "G=%d" % g)
    check("H is 1", h == 1, "H=%d" % h)
    ilen = struct.unpack_from("<H", body, 6)[0]
    check("I is the empty string, so atoi(I) == 0", ilen == 0, "len=%d" % ilen)
    j, k = struct.unpack_from("<I", body, 8)[0], struct.unpack_from("<I", body, 12)[0]
    check("J and K are the IV seeds", (j, k) == (CLIENT_IV, SERVER_IV),
          "J=%#x K=%#x" % (j, k))
    check("L is 1", body[16] == 1, "L=%d" % body[16])

    # Send as the client does: AES-256-OFB on the J chain, header constant 0x00DF.
    iv = struct.pack("<I", CLIENT_IV)

    def send(payload):
        global iv
        frame = transport.header(iv, len(payload), transport.SEND_CONST)
        sock.sendall(frame + transport.ofb(payload, iv, key))
        iv = transport.next_iv(iv, shuffle)

    send(transport.packet(0x0070, bytes([2]) + struct.pack("<I", 100)))
    hello = struct.pack("<II", 0, 0) + struct.pack("<I", CHARACTER_ID) + bytes(24)
    send(transport.packet(0x007D, hello))

    # Anything coming back is on the K chain. There is nothing to read yet; this decodes
    # whatever appears so that the first real reply is checked the moment it exists.
    decoder = transport.ClientDecoder(SERVER_IV, key, shuffle)
    sock.settimeout(1.5)
    try:
        while True:
            data = sock.recv(4096)
            if not data:
                break
            for pkt in decoder.feed(data):
                replies.append(pkt)
    except socket.timeout:
        pass
    sock.close()
finally:
    proc.terminate()
    proc.wait(timeout=5)
    logf.close()

log = open(logpath, encoding="utf-8", errors="replace").read()

check("the server framed both packets rather than erroring", "framing" not in log.lower())
check("it decrypted and named the environment report", "0x0070" in log)
check("it decrypted the migration hello", "0x007D" in log)
check("it read the character id out of the hello body", str(CHARACTER_ID) in log,
      "expected %d in the log" % CHARACTER_ID)
check("it refuses a character with no minted migration, and says why",
      "no unconsumed migration" in log)

print()
print("the server said:")
for line in log.splitlines():
    if any(w in line for w in ("greeting", "<-", "->", "migration", "connection from")):
        print("   " + line)

print()
if replies:
    print("the server sent %d packet(s) back, decrypted on the K chain:" % len(replies))
    for pkt in replies:
        print("   opcode %#06x  %s" % (struct.unpack_from("<H", pkt, 0)[0], pkt[2:].hex(" ")))
else:
    print("the server sent nothing back - expected today: this stage is still UNDECODED,")
    print("and crates/world answers nothing on purpose. When a reply is added, it is")
    print("decoded above and this is where it gets checked.")

print()
print("FAILED: " + ", ".join(fails) if fails else "all checks passed")
sys.exit(1 if fails else 0)
