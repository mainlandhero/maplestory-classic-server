"""Send candidate handshakes to the connecting client and record which one it answers.

The client connects and then waits, sending nothing, so the server has to speak first.
The classic MapleStory greeting is unencrypted and shaped like:

    u16   length of everything after this field
    u16   version
    u16   sub-version string length, then that many bytes
    u8[4] recv IV seed
    u8[4] send IV seed
    u8    locale

We do not know that this build still uses that shape, nor which locale byte it wants,
so each incoming connection gets the next variant in the list and we watch for a reply.
A reply of any kind means the framing is close; a silent drop means it is not.

    python tools/handshake_probe.py [--port 8484] [--only N]
"""

import argparse
import itertools
import socket
import sys
import threading
import time

VERSION = 779  # confirmed from the WZ archives


def log(msg: str = "") -> None:
    """Print one line, always timestamped.

    It used to print the message bare, and only a handful of call sites added a time of
    their own. That is how "connection reset by client" - one of the *untimestamped* lines -
    got read as happening immediately after the last timestamped line above it, which is
    where "the connection dies right after 0x007A" came from. It was an artefact of the log
    format: a `Get-NetTCPConnection` poll showed both endpoints Established from well before
    that moment until the client exited half a minute later.

    Every line carries a time now, so no line's position in the file can be mistaken for
    its position in time.
    """
    print(f"{time.strftime('%H:%M:%S')} {msg}" if msg else "", flush=True)


def hexdump(data: bytes, indent: str = "      ") -> str:
    out = []
    for off in range(0, len(data), 16):
        chunk = data[off : off + 16]
        hexpart = " ".join(f"{b:02X}" for b in chunk)
        text = "".join(chr(b) if 0x20 <= b < 0x7F else "." for b in chunk)
        out.append(f"{indent}{off:04X}  {hexpart:<47}  {text}")
    return "\n".join(out)


def frame(body: bytes) -> bytes:
    """u16 little-endian body length, then the body.

    The client reads exactly 2 bytes, takes them as the length, rewinds, and reads that
    many bytes over them - so the length does not count itself and the body starts at
    offset 0. See docs/handshake.md."""
    return len(body).to_bytes(2, "little") + body


def u8(v: int) -> bytes:
    return bytes([v & 0xFF])


def u16(v: int) -> bytes:
    return (v & 0xFFFF).to_bytes(2, "little")


def u32(v: int) -> bytes:
    return (v & 0xFFFFFFFF).to_bytes(4, "little")


def mstr(s: bytes) -> bytes:
    """u16 byte count, then the bytes."""
    return u16(len(s)) + s


def greeting(
    with_gated: bool,
    low: int = 1,
    high: int = 100,
    temp: int = 0,
    flag_l: int = 1,
) -> bytes:
    """Build a greeting from the layout recovered in docs/handshake.md.

    `flag_l` is field L, and it gates everything: the client does

        if (L == 1) { ...proceed... } else { error 0x22000007 }   // "client is outdated"

    so sending L = 0 produces that dialog no matter what the version fields say. That is
    why every version combination looked identical until L was found.
    """
    body = b""
    if with_gated:
        body += u16(0)          # A
        body += mstr(b"")       # B
        body += u32(0)          # C
        body += u32(0)          # D
        body += u8(0)           # E
        body += u8(0)           # F

    # Always present. G packs a flag in bit 0x8000; send it clear.
    body += u16(0)              # G
    body += u32(0)              # H
    body += mstr(b"")           # I
    body += u32(0)              # J
    body += u32(0)              # K
    body += u8(flag_l)          # L  <- must be 1 or the client reports "outdated"

    if with_gated:
        body += u32(low)        # version range low   (must be <= 100)
        body += u32(high)       # version range high  (must be >= 100)
        body += u32(temp)       # nClientVersion_Temp
        body += u8(0)           # M
        body += u8(0)           # N
        body += u8(0)           # O
    return frame(body)


def handshake(
    low: int = 1,
    high: int = 100,
    temp: int = 0,
    g: int = 1,
    h: int = 1,
    sub: bytes = b"",
    iv_recv: int = 0x52307801,
    iv_send: int = 0x52307802,
    locale: int = 0,
) -> bytes:
    """The greeting with every known gate satisfied.

    Constraints, all read straight off `research/msexe-handshake.c`:

      G == 1 and H == 1     line 606: `(H != 1) || (G != 1)` -> 0x22000007, the SAME
                            "outdated" dialog as a version mismatch. This check sits
                            *outside* the version block and runs unconditionally, so it
                            fired on every earlier test and hid the version result.
      G & 0x8000 clear      line 609: the flag bit raises 0x22000001.
      atoi(I) == 0          line 438: with G == 1 a non-zero sub-version diverts into
                            the patcher path, which throws.
      L == 1                line 437: the gate found earlier.
      high == 100           first connect (line 567) wants exactly 100; second connect
      low  <= 100           (line 525) wants low < 101 && high > 99. high=100, low=1
      temp == 0             satisfies both, and temp=0 selects the checked path.

    J and K land in the connection object at +0xe8/+0xec and look like the send/recv
    seeds, so they get plausible values rather than zero.
    """
    body = b""
    # --- gated on cfg+0x48 != 0 (the login connection) ---
    body += u16(0)              # A
    body += mstr(b"")           # B   (overwritten by I below)
    body += u32(0)              # C
    body += u32(0)              # D
    body += u8(0)               # E
    body += u8(0)               # F
    # --- always present ---
    body += u16(g & 0x7FFF)     # G   must be 1, flag bit clear
    body += u32(h)              # H   must be 1
    body += mstr(sub)           # I   must parse to 0
    body += u32(iv_recv)        # J   -> conn+0xe8
    body += u32(iv_send)        # K   -> conn+0xec
    body += u8(1)               # L   must be 1
    # --- gated again ---
    body += u32(low)            # version range low
    body += u32(high)           # version range high
    body += u32(temp)           # nClientVersion_Temp
    body += u8(0)               # M
    body += u8(0)               # N
    # --- always present ---
    body += u8(locale)          # O   -> conn+0x0c; 4 and 5 take special paths
    return frame(body)


def classic(version: int, sub: bytes, recv_iv: bytes, send_iv: bytes, locale: int) -> bytes:
    body = (
        version.to_bytes(2, "little")
        + len(sub).to_bytes(2, "little")
        + sub
        + recv_iv
        + send_iv
        + bytes([locale])
    )
    return len(body).to_bytes(2, "little") + body


def all_u32_are(value: int) -> bytes:
    """Every u32 slot set to the same value.

    Sweeping the version triple's offset one step at a time costs one client launch per
    step. Setting *every* u32 to 100 covers all candidate positions in a single test:
    whichever slot the client actually reads as the version range will read 100, which
    is the value that takes its "Version OK" branch. If the dialog still appears, the
    version fields are not inside the structure we are sending at all - a much more
    useful thing to learn than one more offset being wrong.
    """
    body = b""
    body += u16(value)      # A
    body += mstr(b"100")    # B  (string, parsed as a number by the client)
    body += u32(value)      # C
    body += u32(value)      # D
    body += u8(0)           # E
    body += u8(0)           # F
    body += u16(value)      # G  (flag bit left clear)
    body += u32(value)      # H
    body += mstr(b"100")    # I
    body += u32(value)      # J
    body += u32(value)      # K
    body += u8(1)           # L  (must be 1)
    body += u32(value)      # low
    body += u32(value)      # high
    body += u32(value)      # temp
    body += u8(0) + u8(0) + u8(0)
    return frame(body)


def truncated_after_l(flag_l: int) -> bytes:
    """The gated + always blocks only, ending immediately after field L.

    Deliberately too short for the version block that follows.
    """
    body = b""
    body += u16(0) + mstr(b"") + u32(0) + u32(0) + u8(0) + u8(0)   # A B C D E F
    body += u16(0) + u32(0) + mstr(b"") + u32(0) + u32(0)          # G H I J K
    body += u8(flag_l)                                             # L
    return frame(body)


def all_fields_passing() -> bytes:
    """Every field set to a value that should satisfy its check, simultaneously.

    u8 fields -> 1   (field L must be 1; the others are flags)
    u32 fields -> 100 (the version range brackets the client's version of 100)
    u16 fields -> 100 (G's flag bit stays clear at this value)
    strings   -> "100"

    If this still fails, the field *types* or their order are wrong, not just one
    value - which is a different and more useful conclusion than another near-miss.
    """
    body = b""
    body += u16(100) + mstr(b"100") + u32(100) + u32(100) + u8(1) + u8(1)   # A B C D E F
    body += u16(100) + u32(100) + mstr(b"100") + u32(100) + u32(100)        # G H I J K
    body += u8(1)                                                          # L
    body += u32(100) + u32(100) + u32(100)                                 # low high temp
    body += u8(1) + u8(1)                                                  # M N (gated)
    body += u8(1)                                                          # O (always)
    return frame(body)


def variants():
    """Candidate greetings, cheapest/most-likely first.

    Includes deliberate controls. If random bytes produce exactly the same client
    behaviour as a well-formed greeting, the client is not parsing our format at all
    and the version is not the variable to sweep."""
    yield ("G=1 H=1 L=1, high=100 (all known gates satisfied)", handshake())
    yield ("G=1 H=1, second-connect range low=1 high=200", handshake(low=1, high=200))
    yield ("G=1 H=1, locale O=4", handshake(locale=4))
    yield ("G=1 H=1, locale O=5", handshake(locale=5))

    yield ("ALL fields passing (u8=1, u32=100)", all_fields_passing())

    # Discriminator: both the L gate and the version mismatch raise error 0x22000007
    # and so show the SAME "outdated" dialog. Truncating right after L separates them:
    #   L accepted -> the version reads run off the end, the reader throws -> silent exit
    #   L rejected -> the dialog appears as before
    yield ("DISCRIMINATOR: L=1, body truncated after L", truncated_after_l(1))
    yield ("DISCRIMINATOR: L=0, body truncated after L", truncated_after_l(0))

    yield ("L=1, low=1 high=100 temp=0", greeting(True))
    yield ("L=1, low=1 high=200 temp=100", greeting(True, low=1, high=200, temp=100))
    yield ("L=1, low=100 high=100 temp=100", greeting(True, low=100, high=100, temp=100))
    yield ("ALL u32 fields = 100 (L=1)", all_u32_are(100))

    # Gated blocks confirmed present: those variants reached the client's version
    # check and produced its "client is outdated" dialog, which is the Launch Patcher
    # branch (high > 100). Sweep toward high == 100, which is the "Version OK" branch.
    for (low, high, temp) in (
        (1, 100, 0),
        (100, 100, 0),
        (1, 100, 1),
        (0, 100, 0),
        (100, 100, 100),
        (1, 100, 100),
        (99, 101, 0),
        (1, 200, 0),
    ):
        yield (f"low={low} high={high} temp={temp}", greeting(True, low=low, high=high, temp=temp))

    # CONTROL: not a valid greeting under any format.
    yield ("CONTROL random 16 bytes", bytes(range(0x40, 0x50)))
    # CONTROL: plausible length prefix, garbage payload.
    yield ("CONTROL len-prefixed garbage", (14).to_bytes(2, "little") + bytes(14))
    recv_iv = bytes([0x52, 0x30, 0x78, 0x01])
    send_iv = bytes([0x52, 0x30, 0x78, 0x02])

    # Locale byte differs per region: 8 = GMS, 1 = KMS, 2 = MSEA, 5 = Tespia.
    for locale in (8, 1, 2, 5, 0):
        yield (
            f"classic v{VERSION} sub='1' locale={locale}",
            classic(VERSION, b"1", recv_iv, send_iv, locale),
        )
    # The launch parser defaults the region string to "KR"; try that as sub-version.
    yield (
        f"classic v{VERSION} sub='KR' locale=1",
        classic(VERSION, b"KR", recv_iv, send_iv, 1),
    )
    # Some builds send an empty sub-version.
    yield (
        f"classic v{VERSION} sub='' locale=8",
        classic(VERSION, b"", recv_iv, send_iv, 8),
    )
    # Version could be sent negated, as the header does in some regions.
    yield (
        f"classic v{0xFFFF ^ VERSION} (negated) sub='1' locale=8",
        classic(0xFFFF ^ VERSION, b"1", recv_iv, send_iv, 8),
    )


def make_cipher(client_recv_iv: int):
    import transport

    key, shuffle = transport.load_tables()
    return transport.ServerCipher(client_recv_iv, key, shuffle)


def build_reply(kind: str, opcode: int, cipher, pad: int = 0, body: bytes | None = None):
    """A server->client frame, built with the rules read out of the client's recv path.

    "header" is the sharpest transport test available: a valid header that declares a
    payload we never send. If the header is good the client simply waits for the rest,
    so nothing is dispatched and an unknown opcode cannot muddy the result; if the
    header is bad it calls FUN_1415d33c0 and drops us straight away.
    """
    import transport

    if kind == "header":
        return cipher.peek_header(100), "valid 4-byte header declaring 100 bytes, body withheld"
    if kind in ("ping", "sweep"):
        # An explicit body is for answering a handler whose format we have actually read,
        # rather than probing one we have not. The login result (opcode 0x0010) is
        # `u8 result, u16-length string` per FUN_141b307b0, and no amount of zero padding
        # produces a *specific* result code.
        if body is not None:
            # `--pad` after an explicit body appends zeros. The login success path reads a
            # u8, 8 bytes, two u32s, three 4-byte fields, a u32, a u8 and then two further
            # sub-readers - spelling all of that out as hex makes the command unreadable,
            # and zeros are the right filler anyway: fixed-width fields read 0 and
            # length-prefixed strings read empty.
            full = body + b"\x00" * pad
            frame = cipher.encode(transport.packet(opcode, full))
            return (frame, f"opcode 0x{opcode:04X} body={body.hex(' ')}"
                    + (f" +{pad} zero bytes" if pad else ""))
        # A bare 2-byte packet makes any handler that reads a body underflow, which can
        # end a sweep on its first *handled* opcode. Zero padding lets more handlers run
        # to completion: fixed-width fields read 0, and length-prefixed strings read empty.
        frame = cipher.encode(transport.packet(opcode, b"\x00" * pad))
        return frame, f"opcode 0x{opcode:04X}" + (f" +{pad}B" if pad else "")
    raise SystemExit(f"unknown reply kind: {kind}")


def parse_reply_seq(spec: str):
    """`--reply-seq 000b:0006..,000b:ff0000,0010:000000/256` -> [(opcode, body, pad)].

    A single reply is not enough once handlers depend on each other. The login result
    makes the client look up its world in a list only inbound 0x000B fills, so the answer
    to one request is three packets, in order. Sending them across three client launches
    would not even be equivalent - the list lives on the stage and the client does not
    survive to be relaunched into the same state.

    `/N` appends N zero bytes, the same filler rule as `--pad`.
    """
    out = []
    for item in spec.split(","):
        item = item.strip()
        if not item:
            continue
        head, _, pad_txt = item.partition("/")
        op_txt, sep, body_txt = head.partition(":")
        if not sep:
            raise SystemExit(f"--reply-seq entry needs OPCODE:BODY, got {item!r}")
        try:
            out.append((int(op_txt, 16),
                        bytes.fromhex(body_txt.replace(" ", "")),
                        int(pad_txt or 0)))
        except ValueError as exc:
            raise SystemExit(f"--reply-seq entry {item!r}: {exc}") from exc
    if not out:
        raise SystemExit("--reply-seq was empty")
    return out


def serve(port: int, only: int | None, hold: float, reply: str | None = None,
          opcode: int = 0xFFFF, recv_iv: int = 0x52307802,
          sweep_from: int = 0, sweep_to: int = 0x1000, sweep_delay: float = 0.15,
          pad: int = 0, skip: frozenset = frozenset(),
          body: bytes | None = None, ping_body: bytes | None = None,
          reply_to: int | None = None, reply_seq=None,
          ping_first: int | None = None, ping_wait: float = 10.0,
          quiet_before: float = 5.0, send_iv: int = 0x52307801,
          keepalive: float = 0.0, keepalive_opcode: int = 0x0023,
          answers: dict | None = None) -> None:
    answers = answers or {}
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(("127.0.0.1", port))
    srv.listen(8)
    log(f"[{port}] listening; will send a candidate handshake on each connection")

    cases = list(variants())
    if only is not None:
        cases = [cases[only % len(cases)]]
    cycle = itertools.cycle(cases)

    while True:
        conn, addr = srv.accept()
        name, payload = next(cycle)
        stamp = time.strftime("%H:%M:%S")
        log(f"\n[{port}] {stamp} connection from {addr[0]}:{addr[1]}")
        log(f"[{port}] sending: {name}  ({len(payload)} bytes)")
        log(hexdump(payload))

        conn.settimeout(5.0)
        try:
            conn.sendall(payload)
        except OSError as e:
            log(f"[{port}] send failed: {e}")
            conn.close()
            continue

        # Hold the connection open. Closing it early makes the client's recv return 0,
        # which sends it down its own disconnect path (FUN_1415d10e0 recurses with
        # param_2 = 0 and raises 0x22000001, "You cannot access the game"). That looks
        # exactly like a rejected handshake, so an idle timeout would poison the result.
        total = b""
        deadline = time.time() + hold
        last_data = time.time()
        # "sweep" drives its own sends; the one-shot reply must not also fire.
        replied = reply is None or reply == "sweep"
        decoder = None
        try:
            import transport

            key, shuffle = transport.load_tables()
            decoder = transport.ClientDecoder(send_iv, key, shuffle)
        except Exception as e:  # noqa: BLE001
            log(f"[{port}] !!! could not build the inbound decoder: {e!r}")
        try:
            cipher = None if reply is None else make_cipher(recv_iv)
        except Exception as e:  # noqa: BLE001 - must not look like a client rejection
            log(f"[{port}] !!! PROBE ERROR building the cipher: {e!r}")
            log(f"[{port}] !!! holding the connection open; THIS RUN PROVES NOTHING")
            cipher, replied = None, True
        next_op = sweep_from
        last_sent = None
        sent_at = None
        answered = False
        pinged = ping_first is None
        opened_at = time.time()
        next_send = 0.0
        next_keepalive = None
        keepalives_sent = 0
        last_keepalive_at = 0.0
        if reply is not None:
            # Keep the loop responsive. With the default 5s timeout the loop blocks in
            # recv and cannot notice the client has gone quiet, so a one-shot reply fires
            # late (or after the very data it was meant to provoke).
            conn.settimeout(0.05)
        try:
            while time.time() < deadline:
                # Let the client finish its opening burst, then answer into the quiet.
                ready = total and time.time() - last_data > quiet_before

                # --reply-to mode: send only the gate packet on a timer, and leave the
                # actual answer to the decode loop.
                #
                # Otherwise the timed path races the client and can fire *before* the
                # request it is meant to answer - which is exactly what happened: 0x0010
                # went out 0.12s before the client sent 0x0080, so the reply arrived
                # first and the request went unanswered. Ordering, not timing.
                if reply_to is not None:
                    if ready and not pinged and ping_first is not None:
                        frame, what = build_reply("ping", ping_first, cipher, pad, ping_body)
                        log(f"[{port}] >>> GATE {what}"
                            f" - now waiting for the client to send 0x{reply_to:04X}")
                        conn.sendall(frame)
                        last_sent, sent_at = ping_first, time.time()
                        pinged = True
                    replied = True          # suppress the one-shot timer path entirely

                # Keep the inbound direction alive.
                #
                # Read this before reading anything into a longer-lived client: it proves
                # nothing about whether the session is valid, only whether the client was
                # waiting on us. Started only once the real answer has gone out, so it can
                # never race the reply sequence or muddy the causality of an early close.
                if keepalive and next_keepalive is not None and time.time() >= next_keepalive:
                    frame, what = build_reply("ping", keepalive_opcode, cipher, 0, b"")
                    quiet = time.time() - last_data if total else 0.0
                    log(f"[{port}] >>> KEEPALIVE {what}"
                        f"  ({quiet:.1f}s since the client last sent anything,"
                        f" {time.time() - opened_at:.1f}s into the connection)")
                    conn.sendall(frame)
                    keepalives_sent += 1
                    last_keepalive_at = time.time()
                    next_keepalive = last_keepalive_at + keepalive

                if (reply == "sweep" and reply_to is None and ready
                        and time.time() >= next_send):
                    # Isolate one opcode before the sweep starts, so a reply to it can be
                    # timed against our packet rather than against the connection age.
                    # Folds the causality test and the next sweep range into one run.
                    if not pinged:
                        frame, what = build_reply("ping", ping_first, cipher, pad,
                                                  ping_body)
                        log(f"[{port}] >>> ISOLATED {what}"
                            f" - watching {ping_wait:.0f}s before the sweep starts")
                        conn.sendall(frame)
                        last_sent, sent_at = ping_first, time.time()
                        pinged = True
                        next_send = time.time() + ping_wait
                        continue
                    while next_op in skip and next_op < sweep_to:
                        log(f"[{port}] skipping 0x{next_op:04X}")
                        next_op += 1
                    if next_op >= sweep_to:
                        log(f"[{port}] sweep finished at 0x{sweep_to:04X}")
                        break
                    frame, what = build_reply(reply, next_op, cipher, pad, body)
                    # One line per opcode, timestamped: if the client dies or the UI
                    # changes, the last line printed says exactly where it happened.
                    log(f"[{port}] >>> {what}")
                    conn.sendall(frame)
                    last_sent, sent_at = next_op, time.time()
                    next_op += 1
                    next_send = time.time() + sweep_delay
                elif not replied and ready:
                    frame, what = build_reply(reply, opcode, cipher, pad, body)
                    log(f"[{port}] >>> REPLY: {what}"
                        f"  ({len(frame)} bytes)")
                    log(hexdump(frame))
                    conn.sendall(frame)
                    replied = True
                    reply_at = time.time()
                    last_sent, sent_at = opcode, reply_at
                try:
                    data = conn.recv(4096)
                except socket.timeout:
                    continue
                if not data:
                    log(f"[{port}] client closed the connection")
                    break
                total += data
                last_data = time.time()
                # Attribute the data to the opcode in flight. A reply to one of our
                # packets is a far stronger signal than a UI change, and this is what
                # makes the log say so without hand-correlating timestamps.
                # Elapsed since our packet vs since the connection opened is what
                # separates "the client answered us" from "a keepalive timer fired".
                if last_sent is None:
                    since = ""
                else:
                    since = (f" (after 0x{last_sent:04X}, +{time.time() - sent_at:.2f}s"
                             f", {time.time() - opened_at:.1f}s into the connection)")
                log(f"[{port}] *** CLIENT SENT "
                    f"{len(data)} bytes{since} ***")
                if decoder is not None:
                    import transport

                    for pkt in decoder.feed(data):
                        op = pkt["opcode"]
                        name = transport.CLIENT_OPCODES.get(op, "")
                        # A bad header means the IV chain desynced; every later packet is
                        # then garbage, so say so loudly rather than printing nonsense.
                        flag = "" if pkt["header_ok"] else "  <<< HEADER MISMATCH, IV DESYNC"
                        log(f"[{port}]      <- #{pkt['n']:<3} 0x{op:04X} {name:34}"
                            f" {transport.describe(pkt['body'])}{flag}")

                        # Answer the moment the client asks, rather than on a timer.
                        #
                        # The window is genuinely tight and it moves: the client sent
                        # 0x0080 at +4.4s in one run, +6.1s in another, +7.2s in a third,
                        # and abandoned the connection within a second or two of that. A
                        # fixed delay lost that race repeatedly, and `--quiet-before`
                        # made it worse by additionally requiring a silence that never
                        # came. Reacting to the packet removes the race entirely.
                        if reply_to is not None and op == reply_to and not answered:
                            # One packet or several. The sequence is sent back to back on
                            # the same socket: the client dispatches from a single recv
                            # loop, so ordering on the wire is the ordering it sees.
                            steps = (reply_seq if reply_seq
                                     else [(opcode, body, pad)])
                            for n, (step_op, step_body, step_pad) in enumerate(steps, 1):
                                frame, what = build_reply(
                                    "ping", step_op, cipher, step_pad, step_body)
                                tag = (f" [{n}/{len(steps)}]" if len(steps) > 1 else "")
                                log(f"[{port}] >>> ANSWERING"
                                    f"{tag} 0x{op:04X} with {what}")
                                conn.sendall(frame)
                                last_sent, sent_at = step_op, time.time()
                            answered = True
                            if keepalive:
                                next_keepalive = time.time() + keepalive

                        # Standing answers, applied every time rather than once.
                        #
                        # --reply-to fires a single sequence at one moment, which is right
                        # for the login handshake and wrong for anything the user can do
                        # repeatedly. The client sends 0x00A8 on every click of "Create a
                        # character", so an answer that fires once would look like the
                        # button working the first time and breaking afterwards.
                        for step_op, step_body, step_pad in answers.get(op, []):
                            frame, what = build_reply(
                                "ping", step_op, cipher, step_pad, step_body)
                            log(f"[{port}] >>> ANSWERING 0x{op:04X} with {what}")
                            conn.sendall(frame)
                else:
                    log(hexdump(data))
        except ConnectionResetError:
            log(f"[{port}] connection reset by client")
        finally:
            conn.close()

        # `reply_at` is only ever set by the *timed* reply path. In `--reply-to` mode it
        # does not exist, so this fell back to `time.time()` and printed "connection lasted
        # 0.0s" on every single run - a number that looked like a measurement of the client
        # giving up instantly and was in fact measuring nothing at all.
        if reply is not None and "reply_at" in locals():
            held = time.time() - reply_at
            log(f"[{port}] connection lasted {held:.1f}s after the reply was sent")
        elif sent_at:
            # "our last packet" was accurate until keepalives existed; they do not touch
            # `sent_at`, which tracks the reply for attribution. Say which one this is,
            # because the difference between "25s after the reply" and "25s after anything
            # we sent" is the whole idle-timeout question.
            log(f"[{port}] connection ended {time.time() - sent_at:.1f}s"
                f" after the last packet of the reply sequence")
        if keepalive:
            if keepalives_sent:
                log(f"[{port}] {keepalives_sent} keepalive(s) went out; the last was"
                    f" {time.time() - last_keepalive_at:.1f}s before the close."
                    f" If that gap is well under the lifetime, an inbound idle timeout is"
                    f" not what ended this connection")
            else:
                log(f"[{port}] keepalive was on but never fired - the reply sequence had"
                    f" not gone out yet, so this run says nothing about it")

        if total:
            log(f"[{port}] VERDICT: client answered {len(total)} bytes to [{name}]")
        else:
            log(f"[{port}] VERDICT: no reply to [{name}]")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, default=8484)
    ap.add_argument("--only", type=int, default=None, help="send only variant N")
    ap.add_argument(
        "--hold",
        type=float,
        default=300.0,
        help="seconds to keep each connection open (closing early looks like a rejection)",
    )
    ap.add_argument("--list", action="store_true", help="print the variants and exit")
    ap.add_argument(
        "--reply",
        choices=("header", "ping", "sweep"),
        default=None,
        help="answer the client once its opening burst goes quiet",
    )
    ap.add_argument("--sweep-from", type=lambda s: int(s, 0), default=0)
    ap.add_argument("--sweep-to", type=lambda s: int(s, 0), default=0x1000)
    ap.add_argument("--sweep-delay", type=float, default=0.15)
    ap.add_argument("--reply-to", type=lambda s: int(s, 0),
                    help="client opcode to answer immediately, e.g. 0x0080 - removes the "
                         "timing race a fixed delay creates")
    ap.add_argument("--reply-seq", default=None,
                    help="several packets to send for --reply-to, in order: "
                         "OPCODE:HEXBODY[/PAD],... e.g. 000b:00..,000b:ff0000,0010:000000/256")
    ap.add_argument("--answer", action="append", default=[],
                    help="a standing answer, repeatable: IN=OPCODE:HEXBODY[/PAD],... "
                         "e.g. 00a8=05f4:0000. Unlike --reply-to this fires every time the "
                         "client sends that opcode, which is what anything the user can "
                         "click more than once needs.")
    ap.add_argument("--keepalive", type=float, default=0.0,
                    help="seconds between keepalive packets once the reply has been sent; "
                         "0 disables. Two runs both ended ~25s after our last packet with "
                         "the socket Established and idle, which is what an inbound idle "
                         "timeout looks like. This tests that directly.")
    ap.add_argument("--keepalive-opcode", type=lambda s: int(s, 0), default=0x0023,
                    help="opcode for --keepalive. The default is a genuine no-op: the "
                         "login stage's switch has `case 0x23: break;`, so it is dispatched "
                         "and handled without reading a body or touching any state. An "
                         "unhandled opcode would leak a 0x5b4 buffer per packet instead.")
    ap.add_argument("--body", type=lambda h: bytes.fromhex(h.replace(" ", "")),
                    help="hex body for the replied/swept opcode, instead of zero padding")
    ap.add_argument("--ping-body", type=lambda h: bytes.fromhex(h.replace(" ", "")),
                    help="hex body for the --ping-first packet")
    ap.add_argument("--pad", type=int, default=0,
                    help="zero bytes appended after the opcode")
    ap.add_argument("--ping-first", type=lambda s: int(s, 0), default=None,
                    help="send this opcode alone before the sweep begins")
    ap.add_argument("--ping-wait", type=float, default=10.0)
    ap.add_argument("--send-iv", type=lambda s: int(s, 16), default=0x52307801,
                    help="the J field we sent; the client transmits on this chain")
    ap.add_argument("--quiet-before", type=float, default=5.0,
                    help="seconds of client silence to wait for before sending. The "
                         "client's own startup burst varies hugely between runs "
                         "(294 to 3393 bytes observed), and speaking too early makes "
                         "its late chatter look like a reply")
    ap.add_argument("--skip", default="",
                    help="comma-separated opcodes to skip (e.g. 0x23), the growing "
                         "blacklist of ones that end the connection")
    ap.add_argument("--opcode", type=lambda s: int(s, 0), default=0xFFFF)
    ap.add_argument(
        "--recv-iv",
        type=lambda s: int(s, 16),
        default=0x52307802,
        help="the K field we sent; the client reads with this chain (conn+0xec)",
    )
    args = ap.parse_args()

    if args.list:
        for i, (name, payload) in enumerate(variants()):
            log(f"{i:3d}  {name}  ({len(payload)} bytes)")
        return

    # Preflight before anything launches the client. A tooling failure here used to kill
    # the serve thread mid-test, closing the socket; the client then dropped because the
    # server had vanished, which is indistinguishable from it rejecting our header. Fail
    # loudly and early instead of burning a client run on a wrong answer.
    # Parsed here rather than at the call below so a malformed sequence is rejected before
    # the preflight, not after it.
    seq = parse_reply_seq(args.reply_seq) if args.reply_seq else None
    if seq is not None and args.reply_to is None:
        raise SystemExit("--reply-seq needs --reply-to: it is sent when that opcode arrives")

    answers = {}
    for spec in args.answer:
        incoming, _, outgoing = spec.partition('=')
        if not outgoing:
            raise SystemExit(f"--answer {spec!r} is not IN=OPCODE:HEXBODY")
        try:
            key = int(incoming, 16)
        except ValueError as exc:
            raise SystemExit(f"--answer {spec!r}: {incoming!r} is not hex") from exc
        answers[key] = parse_reply_seq(outgoing)
        log(f"standing answer: 0x{key:04X} -> "
            + ", ".join(f"0x{o:04X}" for o, _, _ in answers[key]))
    if not args.answer:
        log("standing answers: none")

    if args.reply is not None:
        try:
            c = make_cipher(args.recv_iv)
            build_reply("header", args.opcode, c)
            build_reply("ping", args.opcode, c, args.pad)
            for step_op, step_body, step_pad in seq or []:
                build_reply("ping", step_op, c, step_pad, step_body)
            for steps in answers.values():
                for step_op, step_body, step_pad in steps:
                    build_reply("ping", step_op, c, step_pad, step_body)
            if args.keepalive:
                build_reply("ping", args.keepalive_opcode, c, 0, b"")
        except Exception as e:  # noqa: BLE001
            log(f"PREFLIGHT FAILED: {e!r}")
            log("refusing to start - fix this before launching the client")
            raise SystemExit(2)
        log(f"reply preflight OK ({args.reply})"
            + (f", {len(seq)} packets in sequence" if seq else ""))

    # State the keepalive setting whether or not it is on. An absence of KEEPALIVE lines
    # then distinguishes "the flag never arrived" from "it was on and never fired", which
    # is the difference between a broken command line and a real finding.
    if args.keepalive:
        log(f"keepalive: every {args.keepalive:g}s, opcode "
            f"0x{args.keepalive_opcode:04X}, starting once the reply has been sent")
    else:
        log("keepalive: OFF")

    t = threading.Thread(
        target=serve,
        # Keyword arguments, not positional: serve() has grown past a dozen parameters and
        # inserting one in the middle silently shifts every argument after it.
        kwargs=dict(
            port=args.port, only=args.only, hold=args.hold, reply=args.reply,
            opcode=args.opcode, recv_iv=args.recv_iv,
            sweep_from=args.sweep_from, sweep_to=args.sweep_to,
            sweep_delay=args.sweep_delay, pad=args.pad,
            skip=frozenset(int(x, 0) for x in args.skip.split(',') if x.strip()),
            body=args.body, ping_body=args.ping_body, reply_to=args.reply_to,
            reply_seq=seq,
            ping_first=args.ping_first, ping_wait=args.ping_wait,
            quiet_before=args.quiet_before, send_iv=args.send_iv,
            keepalive=args.keepalive, keepalive_opcode=args.keepalive_opcode,
            answers=answers,
        ),
        daemon=True,
    )
    t.start()
    try:
        while True:
            time.sleep(0.5)
    except KeyboardInterrupt:
        log("stopped")


if __name__ == "__main__":
    main()
