"""Bare TCP listener that logs whatever the client sends.

Output is flushed on every line: when this is captured to a file, buffering will
otherwise hide a connection that did happen.

Stage 1/2 instrument: we do not know the wire format yet, so this makes no attempt to
speak it. It accepts connections, hexdumps the first bytes, and holds the socket open
so the client is not tripped up by an instant close.

    python tools/listen.py [port ...]        # defaults to the client's login+channel ports
"""

import socket
import sys
import threading
import time


def log(msg: str = "") -> None:
    """Print and flush. A captured log that silently buffers looks like nothing
    happened, which is exactly the wrong signal when hunting for a connection."""
    print(msg, flush=True)

DEFAULT_PORTS = [8484, 5160]


def hexdump(data: bytes, indent: str = "      ") -> str:
    lines = []
    for off in range(0, len(data), 16):
        chunk = data[off : off + 16]
        hexpart = " ".join(f"{b:02X}" for b in chunk)
        text = "".join(chr(b) if 0x20 <= b < 0x7F else "." for b in chunk)
        lines.append(f"{indent}{off:04X}  {hexpart:<47}  {text}")
    return "\n".join(lines)


def serve(port: int) -> None:
    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    try:
        srv.bind(("127.0.0.1", port))
    except OSError as e:
        log(f"[{port}] could not bind: {e}")
        return
    srv.listen(4)
    log(f"[{port}] listening")

    while True:
        conn, addr = srv.accept()
        stamp = time.strftime("%H:%M:%S")
        log(f"\n[{port}] {stamp} *** CONNECTION from {addr[0]}:{addr[1]} ***")
        conn.settimeout(10.0)
        total = 0
        try:
            while True:
                data = conn.recv(4096)
                if not data:
                    log(f"[{port}] client closed after {total} bytes")
                    break
                total += len(data)
                log(f"[{port}] received {len(data)} bytes:")
                log(hexdump(data))
        except socket.timeout:
            log(f"[{port}] no further data (total {total} bytes); holding open")
            try:
                time.sleep(5)
            except KeyboardInterrupt:
                pass
        except ConnectionResetError:
            log(f"[{port}] connection reset after {total} bytes")
        finally:
            conn.close()


def main() -> None:
    ports = [int(a) for a in sys.argv[1:]] or DEFAULT_PORTS
    threads = []
    for p in ports:
        t = threading.Thread(target=serve, args=(p,), daemon=True)
        t.start()
        threads.append(t)
    log("listening; Ctrl-C to stop")
    try:
        while True:
            time.sleep(0.5)
    except KeyboardInterrupt:
        log("\nstopped")


if __name__ == "__main__":
    main()
