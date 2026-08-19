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

# The map the planted character stands on. Map 1 so that the NPC table has entries to send
# and the whole field-entry path is exercised. Map 1 is also `Character::default().map_id`,
# so "the SetField carries map 1" alone would not prove the builder used OUR character -
# the character-id check below is what carries that, and the portal move to map 10 makes the
# map field discriminating in the other direction.
MAP_ID = 1

# Where map 1's "out00" leads, per the client's own Map.wz. The portal test sends the named
# form the client really sends, so the generated portal table is exercised too - a hand-typed
# stub here once let a character reach map 10 and get stranded, and the smoke test could not
# have caught it because it was using an explicit target field instead of a name.
PORTAL_TARGET = 10
# What the planted character wears. Slots 5/6/7/11 are what the client's own validator
# FUN_140253980 assigns to a coat, trousers, shoes and a weapon, and they are the values a
# really-created character carries.
EQUIPS = ((5, 1040003), (6, 1060002), (7, 1072003), (11, 1302000))
# An equipped item is 125 bytes with every mask clear and grows from there: each set bit of
# the stat mask adds a u16, and each set bit of the option mask adds its own width. There is
# no length prefix anywhere in the record, so this decoder has to walk it exactly as the
# client does - which is the point of the Python side being an independent implementation.
EQUIPPED_ITEM_BARE_LEN = 125
# The bag. presence[7] switches on a fixed six-turn loop in the record decoder that reads
# one u16 an inventory - the slot count - and the twelve bytes sit between the three
# optional-string flags and the equipped list. That POSITION is what this file really
# checks: the record has no length prefix and no resync point, so twelve bytes in the wrong
# place would leave the equipped list unparseable and the character undressed.
INVENTORY_COUNT = 6
INVENTORY_SIZE_BLOCK_LEN = INVENTORY_COUNT * 2
# 30, not 24: the owner's inventory window is six rows of five, filled, with nothing below the
# fold. 24 is the classic-MapleStory bag and it was carried over from a different game
# version for exactly one commit.
DEFAULT_INVENTORY_SLOTS = 30
# FUN_140303b40's 21 optional fields, in mask-bit order, by width in bytes.
EQUIP_OPTION_WIDTHS = [1, 1, 2, 1, 1, 8, 4, 4, 1, 2, 4, 1, 1, 1, 1, 1, 1, 1, 1, 8, 4]


def equipped_item_len(body, at):
    """Length of the item body starting at `at`, or None if it does not decode."""
    i = at
    if i >= len(body) or body[i] != 1:  # the factory's type byte; 1 is an equip
        return None
    i += 1 + 4                       # type, itemId
    if i > len(body):
        return None
    has_cash_sn = body[i]
    i += 1
    # The cash serial and the raw[8] near the end are mutually exclusive: one or the other,
    # never both, which is why a bare item is 125 either way.
    i += 8 if has_cash_sn else 0
    i += 8 + 4 + 1                   # dateExpire, u32, u8
    if i + 4 > len(body):
        return None
    stat_mask = struct.unpack_from("<I", body, i)[0]
    i += 4
    i += 2 * bin(stat_mask & 0x1FFFF).count("1")
    if i + 4 > len(body):
        return None
    opt_mask = struct.unpack_from("<I", body, i)[0]
    i += 4
    for bit, width in enumerate(EQUIP_OPTION_WIDTHS):
        if opt_mask & (1 << bit):
            i += width
    i += 13 + 1 + 1 + 14             # name, two u8, seven u16
    i += 0 if has_cash_sn else 8     # raw[8], only when there is no cash serial
    i += 32 + 12 + 4 + 6 + 1 + 1     # the two sub-decoders and the fixed tail
    if i + 4 + 1 > len(body):
        return None
    i += 4 + 1                       # a third mask (its optional u16 are not sent) and tailFlag
    return i - at
CLIENT_IV = 0x52307801        # J, the chain the client encrypts with
SERVER_IV = 0x52307802        # K, the chain it decrypts with

key, shuffle = transport.load_tables()

# Where the server binaries are. `target/release` unless MAPLECW_BIN_DIR says otherwise.
#
# The override exists because a server left running from a client launch holds
# `target/release/maplecw-world.exe` open, and on Windows that makes `cargo build
# --release` fail outright with "Access is denied" - so the suite would be checking the
# binary from BEFORE the change it is meant to verify, or not running at all. Building to
# a side target directory and pointing this at it leaves the running servers alone:
#
#     CARGO_TARGET_DIR=target-smoke cargo build --release --workspace
#     MAPLECW_BIN_DIR=target-smoke/release python tools/channel_smoke.py --set-field-probe
BIN_DIR = os.environ.get("MAPLECW_BIN_DIR") or os.path.join("target", "release")


def binary(name):
    return os.path.join(BIN_DIR, name + ".exe" if os.name == "nt" else name)


tmp = tempfile.mkdtemp(prefix="maplecw-ch-")
db = os.path.join(tmp, "ch.db")
port = free_port()
subprocess.run([binary("maplecw-useradd"), "--db", db, "maplecw"],
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
    # Equipment, so the avatar-look check below is not vacuous. Slots and item ids are the
    # ones a real created character carries.
    for slot, item in EQUIPS:
        con.execute(
            "INSERT INTO equipment (character_id, slot, item_id) VALUES (?,?,?)",
            (character_id, slot, item),
        )
    con.commit()
    con.close()


def parse_equipped(body, at):
    # u8 flagA, then (u16 slot, 125-byte item)* until a zero slot.
    if at >= len(body) or body[at] != 0:
        return None
    i = at + 1
    worn = []
    while True:
        if i + 2 > len(body):
            return None
        slot = struct.unpack_from("<H", body, i)[0]
        i += 2
        if slot == 0:
            break
        if not 1 <= slot <= 31:
            return None
        n = equipped_item_len(body, i)
        if n is None or i + n > len(body):
            return None
        item = body[i:i + n]
        worn.append((slot, struct.unpack_from("<I", item, 1)[0], item))
        i += n
    # Four more u16 terminators: presence[2] gates FUN_14030b6f0's one list and
    # FUN_14030b9e0's three as well as the equipped list itself. Omitting them
    # desynchronises everything after, silently.
    if i + 8 > len(body) or any(body[i:i + 8]):
        return None
    return worn, i + 8


logpath = os.path.join(tmp, "world.log")
PROBE = "--set-field-probe" in sys.argv

# With the probe on, the point is the packet, so give the server a character to answer
# about. With it off, the point is that an unclaimed hello is refused in plain words, so
# leave the store empty. Each mode plants exactly what it is testing.
if PROBE:
    plant_character_and_migration(db, CHARACTER_ID)

logf = open(logpath, "w")
# --mobs because the server does NOT send mobs by default any more: the body faulted a
# real client on 2026-08-19 (0xC0000005 at 0x141c810b0, mob+0x2b8 null). The builder is
# still covered here so a byte-level regression in it cannot pass unnoticed, but this
# suite proves the BYTES, not that the client accepts them - and on that it is wrong.
cmd = [binary("maplecw-world"), "--db", db, "--mobs",
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

    if PROBE:
        # 0x00DC, the client's per-field-entry marker - measured as arriving once after
        # every SetField, the first migration and every portal walk alike. NOT 0x0238, which
        # the capture shows arrives only on the very first field entry.
        send(transport.packet(0x00DC, b""))

        # !map typed into the All tab. The prefix is `!` and not `/` because the client
        # never transmits a slash line - it parses those itself and swallows unknown ones.
        # Chat is fire-and-forget, so a command that does nothing is safe.
        # A map that ACTUALLY EXISTS. 104040000 was used here until the /map guard landed
        # and refused it - correctly: this client has only Map0 and Map9, so that id has no
        # field image at all. The guard failing this check is the guard working.
        text = b"!map 40"
        chat = bytes(4) + struct.pack("<H", len(text)) + text + b""
        send(transport.packet(0x00E7, chat))

        # And the field-entry marker again, because the real client sends one after
        # EVERY SetField - the mob and NPC pools are destroyed and rebuilt empty on
        # each field entry, so both have to be re-sent every time. Map 1 is the
        # tutorial start and has no mobs at all; map 40 has 40, which is what
        # exercises the mob path here.
        send(transport.packet(0x00DC, b''))

        # A map that does not exist must move nobody. 104040000 has no field image in this
        # client at all, so the guard must refuse it - and refusing means NO extra reply,
        # which is what the reply count below is really asserting.
        bogus = b"!map 104040000"
        send(transport.packet(0x00E7, bytes(4) + struct.pack("<H", len(bogus)) + bogus + b""))

        # Ordinary chat - not a command. The client renders nothing for its own line,
        # so this has to come back as 0x0231 or the player is typing into a void.
        said = b"Hello"
        send(transport.packet(0x00E7, bytes(4) + struct.pack("<H", len(said)) + said
                              + bytes([3])))

        # 0x00D1, a transfer-field request, in the form the client actually sends: no
        # explicit target field (0xFFFFFFFF), a named portal, and coordinates after it. This
        # exercises the generated portal table end to end - map 1's "out00" leads to map 10.
        name = b"out00"
        req = bytearray(27)
        req[16:20] = struct.pack("<I", 0xFFFFFFFF)
        req[20:22] = struct.pack("<H", len(name))
        req = bytes(req[:22]) + name + struct.pack("<HH", 1107, 365) + bytes(3)
        send(transport.packet(0x00D1, req))

        # 0x0151, the quest request - the packet the client sends when an NPC is clicked.
        # These are the exact 17 bytes the owner's client sent for Heena on map 1, out of
        # research/fixtures/npcs-visible-quests-clicked-world.log. Field 1 is a QUEST id and
        # field 2 the NPC template; the first was once recorded as our own object id, and
        # our own logs disproved it.
        send(transport.packet(0x0151, bytes.fromhex("01e8030000010000000c046d0100000000")))

        # 0x00F2, the OTHER NPC click - the one an NPC with no quests produces. These are
        # the exact 12 bytes the owner's client sent for Robin on map 40, from
        # research/fixtures/dressed-in-world-npc-click-00f2-world.log. Which of the two goes
        # out is decided inside the client from Quest.wz, so a server that answers only
        # 0x0151 is silent for every quest-less NPC. Map 1's Heena is object id 1000 here.
        send(transport.packet(0x00F2, bytes.fromhex("e803000001001301ffffffff")))

    # Anything coming back is on the K chain. There is nothing to read yet; this decodes
    # whatever appears so that the first real reply is checked the moment it exists.
    # The channel is ASYMMETRIC: we send AES and receive the byte subtract from the
    # client, but the client RECEIVES the byte subtract - so read our own stream the way
    # the client does, by subtracting iv[0], not with AES. Measured 2026-08-19: sending
    # AES made the client dispatch opcode 0x406C, our ciphertext minus iv[0].
    server_iv = struct.pack("<I", SERVER_IV)
    # NPC idle chatter is the server's only UNSOLICITED packet, and it is the one thing here
    # that cannot be provoked by sending something. It is due 3-9 seconds after field entry
    # (the client's own rand() % 6000 + 3000), so the probe run waits long enough to see at
    # least one - which also exercises the socket loop's read-timeout path end to end. A
    # timeout mishandled as a disconnect would drop the connection here rather than on a
    # client launch.
    #
    # **Read to a DEADLINE, not to a silence.** This loop used to run until the socket went
    # quiet, which worked while every packet was a reply to something. It hangs forever now:
    # idle chatter arrives every 3-9 seconds, so a socket timeout longer than that is never
    # reached and the loop never exits. That is worth knowing beyond this file - any tool
    # that treats silence as "the server is done" is broken by the unsolicited path.
    deadline = time.time() + (11.0 if PROBE else 1.5)
    sock.settimeout(0.5)
    inbox = bytearray()
    while time.time() < deadline:
        try:
            data = sock.recv(4096)
        except socket.timeout:
            continue
        if not data:
            break
        inbox += data
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
    NPC_ENTER_FIELD = 0x044F
    set_fields = [r for r in replies if r["opcode"] == SET_FIELD]
    npcs = [r for r in replies if r["opcode"] == NPC_ENTER_FIELD]

    # 0x0138 UserAvatarModified used to be sent here as a guess at the naked character.
    # It is **dead code in the client**: its apply is guarded by a call to 0x1407f5ce0,
    # three bytes of `xor eax,eax; ret`, then TEST/JZ. The character is dressed by the
    # SetField record instead, and the checks for that are below.
    AVATAR = 0x0138
    check("no UserAvatarModified is sent - 0x0138 is dead code in the client",
          not [r for r in replies if r["opcode"] == AVATAR])

    MOB_ENTER_FIELD = 0x03C6
    mobs = [r for r in replies if r["opcode"] == MOB_ENTER_FIELD]

    # ---- the mobs
    #
    # Map 1 has none - it is the tutorial start - so the first field entry must produce
    # zero, and the second, on map 40, must produce that map's 40. Getting a mob onto the
    # WRONG map is the failure this asymmetry catches.
    # 40 spawn POINTS, but a spawn point is not a mob: the server caps how many are alive
    # at 75% for a solo player, so 30. That figure is [I] from a fan site, not from the WZ -
    # map 40's info node has a mobRate and no capacity of any name. See
    # world::config::spawn_capacity.
    check("map 40 sends 30 mobs, not one per spawn point", len(mobs) == 30,
          "%d" % len(mobs))
    if mobs:
        mb = mobs[0]["body"][2:]
        # 137 = 11-byte head + the 20-byte temp-stat mask + the 106-byte encodeInit. The
        # mask gates all 331 optional reads in FUN_14046fba0, so all-zero costs no bytes.
        check("a mob body is 137 bytes", len(mb) == 137, "%d bytes" % len(mb))
        ids = [struct.unpack_from("<I", r["body"][2:], 1)[0] for r in mobs]
        # A zero object id pulls in an extra u32 and desynchronises the rest; a multiple of
        # 178 takes a branch through a vtable slot on what looks like an exception object.
        check("no mob has object id 0", all(i != 0 for i in ids))
        # Byte 35 is action*2 + facing, and action 0 is what killed the client on
        # 2026-08-19: it takes a callback at 141c50da5 into an interface encodeInit does not
        # build until 0x148 bytes later, so the read lands on a null pointer. Anything with
        # action >= 1 skips it. See net::mob::MOVE_ACTION_MIN_SAFE.
        actions = [r["body"][2:][35] for r in mobs]
        check("no mob is sent with move action 0 - that is the crash",
              all(a >= 2 for a in actions), "byte 35 values %s" % sorted(set(actions)))
        check("no mob's object id is a multiple of 178",
              all(i % 178 != 0 for i in ids), "%s" % [i for i in ids if i % 178 == 0][:4])
        check("every mob on the field has a distinct object id",
              len(set(ids)) == len(ids), "%d ids, %d distinct" % (len(ids), len(set(ids))))
        # Mob ids start at 2000 and NPC ids at 1000, so the two pools cannot collide even if
        # they turn out to share an id space.
        npc_ids = [struct.unpack_from("<I", r["body"][2:], 0)[0]
                   for r in replies if r["opcode"] == 0x044F]
        check("no mob object id collides with an NPC object id",
              not (set(ids) & set(npc_ids)), "%s" % sorted(set(ids) & set(npc_ids))[:4])
        # hp = 0 draws a mob at 0%, and the bar is hp*100/maxHp through an IDIV at
        # 141c50502 with no zero guard. This is the mob's version of the NPC alpha bug.
        # hp is a u64 at body offset 50 in the 137-byte minimum shape; an appear-option
        # block would shift it by 4, so only read it when the body IS the minimum.
        hps = [struct.unpack_from("<Q", r["body"][2:], 50)[0]
               for r in mobs if len(r["body"]) - 2 == 137]
        check("no mob is sent with hp = 0 - that is a mob at 0 percent",
              hps and all(h != 0 for h in hps), "%s" % hps[:3])

    # ---- ordinary chat comes back as a balloon
    #
    # 0x0231, found by intersecting the 17 chat-window printers with the 18 balloon
    # creators: FUN_142784970 is in both, and the jump table at 0x1429bb5d0 puts it at
    # index 11 of the 0x226..0x276 range. research/user-chat.md.
    USER_CHAT = 0x0231
    spoken = [r for r in replies if r["opcode"] == USER_CHAT]
    check("ordinary chat is said back to the speaker", len(spoken) == 1,
          "%d UserChat" % len(spoken))
    if spoken:
        sb = spoken[0]["body"][2:]
        check("the balloon is attached to the character who spoke",
              struct.unpack_from("<I", sb, 0)[0] == CHARACTER_ID,
              "id %d" % struct.unpack_from("<I", sb, 0)[0])
        n = struct.unpack_from("<H", sb, 5)[0]
        check("the text survives the round trip", sb[7:7 + n] == b"Hello",
              "%r" % sb[7:7 + n])
        # FUN_142784970 reads u8, str, u8, u8 - both trailing bytes, unconditionally. A
        # body that stops after the text makes the client read past the end and throw.
        check("both bytes after the text are present", len(sb) == 4 + 1 + 2 + n + 2,
              "%d bytes, wanted %d" % (len(sb), 4 + 1 + 2 + n + 2))

    # ---- the refused !map now says why on screen
    #
    # The drive above sends `!map 104040000`, which has no field image in this client. It
    # must be refused AND explained: a silent refusal is indistinguishable from a command
    # that did not arrive, which is what the owner hit.
    CHAT_NOTICE = 0x00BB
    notices = [r for r in replies if r["opcode"] == CHAT_NOTICE]
    check("the refused !map explains itself in chat", len(notices) == 1,
          "%d notices" % len(notices))
    if notices:
        nb = notices[0]["body"][2:]
        # force = 1. With 0 the client shows only the first line after each field entry and
        # drops the rest, which reads exactly like the feature being broken.
        check("the notice forces itself onto the screen (force = 1)", nb[0] == 1,
              "force = %d" % nb[0])
        n = struct.unpack_from("<H", nb, 1)[0]
        text = nb[3:3 + n].decode("utf-8", "replace")
        check("and it names the map id that was refused", "104040000" in text, text)

    # ---- idle chatter, the unsolicited path
    NPC_CHAT = 0x0453
    balloons = [r for r in replies if r["opcode"] == NPC_CHAT]
    check("the server sends idle chatter without being asked", len(balloons) >= 1,
          "%d balloons in %ds" % (len(balloons), 11))
    if balloons:
        bb = balloons[0]["body"][2:]
        check("a chat balloon is 10 bytes", len(bb) == 10, "%d" % len(bb))
        who = struct.unpack_from("<I", bb, 0)[0]
        # It must address an NPC by the OBJECT id we assigned, not a template - the pool
        # keys on the object id and a template silently addresses nothing.
        check("the balloon names an NPC object id we assigned", who >= 1000,
              "object id %d" % who)
        # nAction is SIGNED: -1 is "talk without changing animation", and only that value
        # indexes the info/speak group the lines come from.
        check("nAction is -1, so it indexes info/speak", bb[4] == 0xFF, "%#04x" % bb[4])
        # In order, wrapping - the ordering is ours because the client has no cursor.
        seen = [r["body"][2:][5] for r in balloons]
        check("the lines advance in order", seen == sorted(seen) or len(seen) == 1,
              "indices %s" % seen)
        # And the connection survived the read timeouts that made room for them.
        check("the connection survived the tick timeouts", True)

    check("the probe answered every request", len(replies) >= 8 + 30 + 2,
          "%d replies: %s" % (len(replies), sorted(set(hex(r["opcode"]) for r in replies))))

    # ---- the NPC the client clicked
    #
    # Always answer: an unanswered request freezes the client's whole UI. This is text on
    # screen only - no quest-result packet is known, so no state advances.
    SCRIPT_MESSAGE = 0x055B
    says = [r for r in replies if r["opcode"] == SCRIPT_MESSAGE]
    check("both NPC-click packets are answered with a script message", len(says) == 2,
          "%d" % len(says))
    if says:
        sb = says[0]["body"][2:]
        # The speaker is at head offset 5 and must be the template the CLIENT named - by
        # construction a real Npc.wz id. Zero is not one, and a template the client did not
        # send would be a guess.
        speaker = struct.unpack_from("<I", sb, 5)[0]
        check("the script speaks as the NPC template the client named", speaker == 1,
              "template %d" % speaker)
        # 0x00F2 hands over the OBJECT id we chose (1000), not a template, while the script
        # message's speaker field wants a template. Sending 1000 through would not fault -
        # the loader result is null-checked - it would just draw a portrait-less box, which
        # is the kind of failure a client run cannot explain.
        clicked = struct.unpack_from("<I", says[1]["body"][2:], 5)[0]
        # And 8 rather than 1 is the stronger check: the character walked to map 40 with
        # !map earlier in this run, and object id 1000 is template 1 on map 1 but template 8
        # on map 40. So this also proves the lookup is scoped to the map the character is
        # actually on, which is what makes a per-map numbering safe.
        check("the no-quest click is answered as the TEMPLATE, not the object id",
              clicked == 8, "speaker %d (1000 would be the object id)" % clicked)
        check("hasOverride is 0, so no u32 follows it and the body does not shift",
              sb[9] == 0, "%d" % sb[9])
        # messageType indexes a 71-entry jump table; 0 is Say. Anything else reads a
        # different body, and there is no resync point after it.
        check("the message type is 0, Say", sb[10] == 0, "type %d" % sb[10])
        text_len = struct.unpack_from("<H", sb, 18)[0]
        check("the text is a u16 BYTE count, and the body is exactly long enough",
              len(sb) == 20 + text_len + 6, "%d bytes, text %d" % (len(sb), text_len))
        check("the text is not empty - an empty Say puts nothing on screen",
              text_len > 0, "%d" % text_len)

    # ---- EVERY SetField must be dressed, not just the first
    #
    # This is the regression the owner hit on 2026-08-19: the migration's record carried the
    # items' stats and every later one - a portal walk, a !map - carried EquipStats::default,
    # all zeros. On screen that is "my items had stats until I used a command", and it sent a
    # whole investigation after a client-side ghost that was never there.
    for i, sf in enumerate(set_fields):
        body = sf["body"][2:]
        rec = HEAD + 12
        if len(body) < rec + 100:
            continue
        check("SetField %d switches the equipped list on" % i, body[rec + 2] == 1,
              "presence[2] = %d" % body[rec + 2])
        # The stat mask of the first equipped item must be non-zero. A record whose items
        # are all bare parses perfectly and looks identical in every other check.
        parsed = [(L, parse_equipped(body, rec + 111 + L + 4 + INVENTORY_SIZE_BLOCK_LEN))
                  for L in (108, 109)]
        good = [r for _, r in parsed if r is not None]
        if good:
            worn = good[0][0]
            masks = [struct.unpack_from("<I", raw, 19)[0] for _, _, raw in worn]
            check("SetField %d carries item stats, not zeros" % i, all(m != 0 for m in masks),
                  "stat masks %s" % [hex(m) for m in masks])

    check("three are SetField - the migration, the /map command and the portal",
          len(set_fields) == 3, "%d" % len(set_fields))
    if len(set_fields) == 3:
        gm = set_fields[1]["body"][2:]
        at = HEAD + 12 + 111 + 84
        check("the GM /map command moved the character to the map it names",
              struct.unpack_from("<I", gm, at)[0] == 40,
              "map %d" % struct.unpack_from("<I", gm, at)[0])
    check("four are NpcEnterField - map 1's Heena and Sera, then map 40's two on the "
          "second field entry", len(npcs) == 4, "%d" % len(npcs))
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
            check("presence[2] is set, so the equipped list decodes", presence[2] == 1,
                  "presence[2]=%d" % presence[2])
            check("presence[7] is set, so the bag gets a size", presence[7] == 1,
                  "presence[7]=%d" % presence[7])
            stray = [i for i, b in enumerate(presence) if b and i not in (0, 2, 7)]
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

            # ---- the equipped list, which is what dresses the character
            #
            # It sits between the three optional-string flags and the final ungated u8, and
            # the stat block before it is 108 bytes on the extended-SP branch and 109 on the
            # plain one. Rather than assume the job, parse at both and require exactly one
            # to be a well-formed block - that IS the discriminator, and a layout error
            # shows up here as "neither parses" rather than as a client fault.
            parsed = [(L, parse_equipped(body, stat + L + 4 + INVENTORY_SIZE_BLOCK_LEN))
                      for L in (108, 109)]
            good = [(L, r) for L, r in parsed if r is not None]
            check("the equipped block parses at exactly one stat-block length",
                  len(good) == 1,
                  "parsed at %s" % [L for L, _ in good])
            if len(good) == 1:
                stat_len, (worn, end) = good[0]

                # The bag, at the offset the equipped parse just settled on. Reading it
                # anywhere else - or at whichever offset happened to look like 24s - would
                # make this check confirm itself.
                sizes = struct.unpack_from("<6H", body, stat + stat_len + 4)
                check("all six inventories get a non-zero slot count",
                      all(v > 0 for v in sizes), "%s" % (sizes,))
                check("the bag is the default size",
                      all(v == DEFAULT_INVENTORY_SLOTS for v in sizes),
                      "%s, wanted six of %d" % (sizes, DEFAULT_INVENTORY_SLOTS))

                check("the equipped list carries every item the character wears",
                      [(sl, it) for sl, it, _ in worn] == list(EQUIPS),
                      "%s, wanted %s" % ([(sl, it) for sl, it, _ in worn], list(EQUIPS)))
                check("every equipped item is at least the 125-byte bare body",
                      all(len(raw) >= EQUIPPED_ITEM_BARE_LEN for _, _, raw in worn),
                      "%s" % [len(raw) for _, _, raw in worn])

                # The stats are the point: a fresh item carries its Character.wz template
                # values as ABSOLUTES, and the tooltip's print guard is on the packet field,
                # so a zero suppresses the line whatever the template says. The Grey T-Shirt
                # (1040002) has incPDD 6 and tuc 7; the starter sword (1302000) has incWAT 17
                # - and incWAT is bit 16, because this client's WZ has no incPDD... no
                # incPAD at all.
                by_id = {item: raw for _, item, raw in worn}
                for item_id, bit, want, what in (
                    (1040003, 10, 6, "the shirt's incPDD"),
                    (1060002, 10, 4, "the trousers' incPDD"),
                    (1302000, 16, 17, "the sword's incWAT (bit 16, NOT incPAD)"),
                ):
                    raw = by_id.get(item_id)
                    if raw is None:
                        continue
                    mask = struct.unpack_from("<I", raw, 19)[0]
                    ok = bool(mask & (1 << bit))
                    got = None
                    if ok:
                        off = 23 + 2 * bin(mask & ((1 << bit) - 1)).count("1")
                        got = struct.unpack_from("<H", raw, off)[0]
                    check("%s reaches the wire as %d" % (what, want), got == want,
                          "mask %#x, value %s" % (mask, got))

                # And the two option fields that between them killed both bad tooltip lines.
                shirt = by_id.get(1040003)
                if shirt is not None:
                    opt_at = 23 + 2 * bin(
                        struct.unpack_from("<I", shirt, 19)[0] & 0x1FFFF).count("1")
                    opt = struct.unpack_from("<I", shirt, opt_at)[0]
                    vals, o = {}, opt_at + 4
                    for bit, width in enumerate(EQUIP_OPTION_WIDTHS):
                        if opt & (1 << bit):
                            vals[bit] = int.from_bytes(shirt[o:o + width], "little")
                            o += width
                    # bit 0 is the remaining upgrade count. It must stay within 0..=tuc:
                    # FUN_14038d3c0 compares the two at 0x14038d41c.
                    check("the shirt's remaining enhancements are its tuc, 7",
                          vals.get(0) == 7, "%s" % vals.get(0))
                    # bit 18 > 0x14 is what stops "Cannot be Traded when equipped", and it
                    # is also why "Scissors Usages Available: 0" printed - one byte, two
                    # lines. 0xFF is the client's own "no restriction" value.
                    check("the shirt is not trade-blocked (scissor uses above 0x14)",
                          vals.get(18, 0) > 0x14, "%s" % vals.get(18))
                # dateExpire is a FILETIME at item offset 6, and zero is 1601-01-01 - an
                # item that expired four centuries ago. It is the top suspect if a run comes
                # back "no fault, still naked", so pin that it is not zero here.
                expiries = [struct.unpack_from("<Q", raw, 6)[0] for _, _, raw in worn]
                check("no equipped item carries dateExpire = 0 (which is 1601-01-01)",
                      all(e != 0 for e in expiries), "%s" % expiries[:2])
                # The three optional-string flags immediately before the block, and the one
                # ungated u8 after it. Both are zero, and the record ends there.
                check("the three optional-string flags before the block are zero",
                      not any(body[stat + stat_len:stat + stat_len + 4]),
                      "%s" % list(body[stat + stat_len:stat + stat_len + 4]))
                # 743 was the bare record. Stats and options add bytes per item, and the
                # exact total is arithmetic on the templates rather than a constant worth
                # pinning - what matters is that the walk ENDS where the record ends.
                check("the record ends where the walk says it does",
                      end + 1 - rec > 743, "%d bytes" % (end + 1 - rec))
                check("the body outlasts the whole record",
                      len(body) > end + 1, "%d bytes, record ends at %d" % (len(body), end + 1))
    # ---- the NPCs the client cannot spawn for itself
    for i, pkt in enumerate(npcs):
        nb = pkt["body"][2:]
        check("NPC %d has the fixed 64-byte body" % i, len(nb) == 64, "%d bytes" % len(nb))
        if len(nb) == 64:
            check("NPC %d carries a real template id, not 0" % i,
                  struct.unpack_from("<I", nb, 4)[0] != 0,
                  "template %d" % struct.unpack_from("<I", nb, 4)[0])
            check("NPC %d has a non-zero foothold" % i,
                  struct.unpack_from("<H", nb, 22)[0] != 0,
                  "fh %d" % struct.unpack_from("<H", nb, 22)[0])
    if len(npcs) >= 2:
        npcs = npcs[:2]  # map 1's, from the first field entry
        ids = [struct.unpack_from("<I", p["body"][2:], 0)[0] for p in npcs]
        # The pool keys on object id: a repeat makes the client return after four bytes and
        # silently drop the NPC, so two NPCs sharing one id would show as one on screen.
        check("the two NPCs have distinct object ids", ids[0] != ids[1], "%s" % ids)
        # Heena stands at negative x. If the builder ever clamps instead of sign-extending,
        # they land on the wrong side of the map, so check the sign survived the wire.
        xs = [struct.unpack_from("<h", p["body"][2:], 8)[0] for p in npcs]
        check("Heena's negative x survived as a signed value", min(xs) < 0, "x = %s" % xs)

    # ---- the portal
    if len(set_fields) == 2:
        moved = set_fields[1]["body"][2:]
        stat = HEAD + 12 + 111
        at84 = struct.unpack_from("<I", moved, stat + 84)[0]
        check("the portal reply is a SetField carrying the TARGET map, not the old one",
              at84 == PORTAL_TARGET, "map %d, wanted %d" % (at84, PORTAL_TARGET))
        check("the portal reply still switches the stat block on",
              moved[HEAD + 12] == 1, "presence[0]=%d" % moved[HEAD + 12])

        # The arrival portal, immediately after the map id. Map 1's "out00" names map 10's
        # "in00", which is index 1 there - so walking that door must NOT put the character
        # back on the spawn (index 0), which is where a login goes.
        arrival = moved[stat + 88]
        check("the character arrives at the matching door, not the map spawn",
              arrival == 1, "portal index %d (0 = spawn, which is the login position)" % arrival)
        first = set_fields[0]["body"][2:]
        check("a fresh login still arrives at the spawn", first[stat + 88] == 0,
              "portal index %d" % first[stat + 88])

        # And the move has to survive a relog, which means it reached the database.
        con = sqlite3.connect(db)
        stored = con.execute("SELECT map_id FROM characters WHERE id=?", (CHARACTER_ID,)).fetchone()[0]
        con.close()
        check("the move was persisted, so a relog puts the character on the new map",
              stored == PORTAL_TARGET, "stored map_id = %s" % stored)
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
