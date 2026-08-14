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
    print(msg, flush=True)


def hexdump(data: bytes, indent: str = "      ") -> str:
    out = []
    for off in range(0, len(data), 16):
        chunk = data[off : off + 16]
        hexpart = " ".join(f"{b:02X}" for b in chunk)
        text = "".join(chr(b) if 0x20 <= b < 0x7F else "." for b in chunk)
        out.append(f"{indent}{off:04X}  {hexpart:<47}  {text}")
    return "\n".join(out)


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


def variants():
    """Candidate greetings, cheapest/most-likely first.

    Includes deliberate controls. If random bytes produce exactly the same client
    behaviour as a well-formed greeting, the client is not parsing our format at all
    and the version is not the variable to sweep."""
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


def serve(port: int, only: int | None) -> None:
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

        conn.settimeout(8.0)
        try:
            conn.sendall(payload)
        except OSError as e:
            log(f"[{port}] send failed: {e}")
            conn.close()
            continue

        total = b""
        try:
            while True:
                data = conn.recv(4096)
                if not data:
                    break
                total += data
                log(f"[{port}] *** CLIENT REPLIED with {len(data)} bytes ***")
                log(hexdump(data))
        except socket.timeout:
            pass
        except ConnectionResetError:
            log(f"[{port}] connection reset by client")
        finally:
            conn.close()

        if total:
            log(f"[{port}] VERDICT: client answered {len(total)} bytes to [{name}]")
        else:
            log(f"[{port}] VERDICT: no reply to [{name}]")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, default=8484)
    ap.add_argument("--only", type=int, default=None, help="send only variant N")
    args = ap.parse_args()

    t = threading.Thread(target=serve, args=(args.port, args.only), daemon=True)
    t.start()
    try:
        while True:
            time.sleep(0.5)
    except KeyboardInterrupt:
        log("stopped")


if __name__ == "__main__":
    main()
