"""Talk to maplecw-world the way the client does, to prove the channel path works.

Uses `tools/transport.py` - the INDEPENDENT Python implementation of the framing, the IV
chain and AES - so a pass is two implementations agreeing rather than the Rust checking
itself. No client launch is spent.

**Rewritten 2026-08-19.** The previous version asserted a byte-shift cipher and a
"POLARITY CHECK" log block, both of which were removed when the channel was measured to be
AES like the login connection. It would have failed every check for the wrong reason - a
stale instrument reporting a real regression.

    python tools/channel_smoke.py
    python tools/channel_smoke.py --set-field-probe

The second form starts the server with the SetField delivery probe on and checks the
packet it sends back - framing, AES, opcode and the field offsets - against the Python
decoder. That is the check worth doing BEFORE spending one of the owner's client launches on it.
"""
import os
import socket
import sqlite3
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

# The map the planted character stands on. Not 1, deliberately: a check that passes on the
# default would not notice the builder ignoring the character and emitting a default one.
MAP_ID = 104_040_000
CLIENT_IV = 0x52307801        # J, the chain the client encrypts with
SERVER_IV = 0x52307802        # K, the chain it decrypts with

key, shuffle = transport.load_tables()

tmp = tempfile.mkdtemp(prefix="maplecw-ch-")
db = os.path.join(tmp, "ch.db")
port = free_port()
subprocess.run([os.path.join("target", "release", "maplecw-useradd.exe"), "--db", db, "maplecw"],
               input="correct horse battery staple\n", text=True, capture_output=True)


def plant_character_and_migration(dbpath, character_id, world_id=0, channel_id=0):
    """Put a character and an unconsumed migration in the store, the way the login server
    would at character select.

    Without this the probe has nothing to claim and correctly falls back to the minimal
    record - which is worth checking too, but it is not the packet a real client gets.
    Writing the rows here rather than driving the whole login flow keeps this script to
    one server; the cost is that a schema change breaks it loudly, which is the right way
    round.
    """
    con = sqlite3.connect(dbpath)
    account_id = con.execute("SELECT id FROM accounts LIMIT 1").fetchone()[0]
    con.execute(
        "INSERT INTO characters (id, account_id, world_id, name, gender, skin, face, hair,"
        " level, job, strength, dexterity, intelligence, luck, hp, max_hp, mp, max_mp, ap,"
        " map_id, created_at)"
        " VALUES (?,?,?,?,0,0,20000,30000,1,0,12,5,4,4,50,50,5,5,0,?,0)",
        (character_id, account_id, world_id, "SmokeChar", MAP_ID),
    )
    con.execute(
        "INSERT INTO migrations (seed, account_id, character_id, world_id, channel_id,"
        " created_at, consumed_at) VALUES (?,?,?,?,?,?,NULL)",
        (0x1234_5678, account_id, character_id, world_id, channel_id,
         int(time.time())),
    )
    con.commit()
    con.close()


logpath = os.path.join(tmp, "world.log")
PROBE = "--set-field-probe" in sys.argv

# With the probe on, the point is the packet, so give the server a character to answer
# about. With it off, the point is that an unclaimed hello is refused in plain words, so
# leave the store empty. Each mode plants exactly what it is testing.
if PROBE:
    plant_character_and_migration(db, CHARACTER_ID)

logf = open(logpath, "w")
cmd = [os.path.join("target", "release", "maplecw-world.exe"), "--db", db,
       "--bind", "127.0.0.1:%d" % port]
if PROBE:
    cmd.append("--set-field-probe")
proc = subprocess.Popen(cmd, stdout=logf, stderr=subprocess.STDOUT)
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
    # The channel is ASYMMETRIC: we send AES and receive the byte subtract from the
    # client, but the client RECEIVES the byte subtract - so read our own stream the way
    # the client does, by subtracting iv[0], not with AES. Measured 2026-08-19: sending
    # AES made the client dispatch opcode 0x406C, our ciphertext minus iv[0].
    server_iv = struct.pack("<I", SERVER_IV)
    sock.settimeout(1.5)
    inbox = bytearray()
    try:
        while True:
            data = sock.recv(4096)
            if not data:
                break
            inbox += data
    except socket.timeout:
        pass
    while len(inbox) >= 4:
        a = int.from_bytes(inbox[0:2], "little")
        length = a ^ int.from_bytes(inbox[2:4], "little")
        if len(inbox) < 4 + length:
            break
        want = (((int.from_bytes(server_iv, "little") >> 16) & 0xFFFF)
                ^ transport.RECV_CONST) & 0xFFFF
        raw = bytes(inbox[4:4 + length])
        del inbox[:4 + length]
        plain = bytes((b - server_iv[0]) & 0xFF for b in raw)
        replies.append({"opcode": int.from_bytes(plain[:2], "little"), "body": plain,
                        "header_ok": a == want})
        server_iv = transport.next_iv(server_iv, shuffle)
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
if PROBE:
    check("it claimed the minted migration for the character",
          "claimed the migration for character %d" % CHARACTER_ID in log)
else:
    check("it refuses a character with no minted migration, and says why",
          "no unconsumed migration" in log)

print()
print("the server said:")
for line in log.splitlines():
    if any(w in line for w in ("greeting", "<-", "->", "migration", "connection from")):
        print("   " + line)

SET_FIELD = 0x01A0
HEAD = 33
if PROBE:
    print()
    print("set-field probe checks:")
    check("the probe answered the migration hello", len(replies) == 1,
          "%d replies" % len(replies))
    if replies:
        pkt = replies[0]
        op = pkt["opcode"]
        body = pkt["body"][2:]
        check("the frame header carries the constant the client requires (0xFFFE)",
              pkt["header_ok"])
        check("the reply is SetField", op == SET_FIELD, "%#06x" % op)
        check("the body is at least the 33-byte fixed head", len(body) >= HEAD,
              "%d bytes" % len(body))
        if len(body) >= HEAD:
            clock = struct.unpack_from("<Q", body, 0)[0]
            # A Windows FILETIME for a date this century, sanity-checked as a range rather
            # than a value, so the check does not go stale tomorrow.
            check("offset 0 is a plausible FILETIME, not zero",
                  116444736000000000 < clock < 160000000000000000, "%d" % clock)
            check("offset 8 is the channel id", struct.unpack_from("<I", body, 8)[0] == 0)
            check("offset 17, the tree-reset byte, is 0", body[17] == 0)
            check("offset 30, characterData, is 1 - the record branch; the other one "
                  "faults this client", body[30] == 1, "%d" % body[30])
            check("offset 31, the string count, is 0",
                  struct.unpack_from("<H", body, 31)[0] == 0)
            # The character record. Offsets are from the first byte the record decoder
            # FUN_140304b20 reads, which is the head plus the three u32s the SetField
            # handler consumes first.
            rec = HEAD + 12
            presence = body[rec:rec + 100]
            check("the presence array is 100 bytes of record", len(presence) == 100)

            # presence[0] is gate entry 7, the character-stat block. Every OTHER flag must
            # stay clear: each one that is set pulls in a block nobody has built, and the
            # record has no length prefix to resynchronise on, so one stray flag desyncs
            # everything after it. research/charrecord-presence-map.md has all 40.
            check("presence[0] is set, so the stat block decodes", presence[0] == 1,
                  "presence[0]=%d" % presence[0])
            stray = [i for i, b in enumerate(presence) if b and i != 0]
            check("no other presence flag is set", not stray, "also set: %s" % stray[:6])

            # The six head fields between the array and the gate are counts and flags the
            # client uses to SKIP. A non-zero byte here pulls in loops that read.
            head_fields = body[rec + 100:rec + 111]
            check("the record's counts and flags between the array and the gate are zero",
                  not any(head_fields),
                  "non-zero at record+%s" % [100 + i for i, b in enumerate(head_fields) if b])

            # The stat block, and the field this whole exercise is about. The map id sits
            # at stat-block offset 84 on the extended-SP branch (85 on the plain one), so
            # accept either rather than assuming the job - research/charstat-layout.md.
            stat = rec + 111
            at84 = struct.unpack_from("<I", body, stat + 84)[0]
            at85 = struct.unpack_from("<I", body, stat + 85)[0]
            check("the map id is a real map at stat-block offset 84 or 85, not 0",
                  at84 > 0 or at85 > 0,
                  "offset 84 = %d, offset 85 = %d - 0 is not a map" % (at84, at85))

            # The record's own id fields, which say we sent THIS character and not a
            # default one. Stat-block offsets 0 and 4 are both the character id.
            id0 = struct.unpack_from("<I", body, stat)[0]
            check("the stat block carries the claimed character id", id0 == CHARACTER_ID,
                  "%d, wanted %d" % (id0, CHARACTER_ID))

            check("the body outlasts the traced read path (33+12+224+1)",
                  len(body) > rec + 224 + 1, "%d bytes" % len(body))
elif replies:
    check("the probe is off, so nothing should come back", False,
          "%d unexpected replies" % len(replies))

print()
if replies:
    print("the server sent %d packet(s) back, decrypted on the K chain:" % len(replies))
    for pkt in replies:
        print("   opcode %#06x  %s" % (pkt["opcode"], pkt["body"][2:].hex(" ")))
else:
    print("the server sent nothing back - expected today: this stage is still UNDECODED,")
    print("and crates/world answers nothing on purpose. When a reply is added, it is")
    print("decoded above and this is where it gets checked.")

print()
print("FAILED: " + ", ".join(fails) if fails else "all checks passed")
sys.exit(1 if fails else 0)
