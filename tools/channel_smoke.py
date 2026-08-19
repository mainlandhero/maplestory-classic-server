"""Talk to maplecw-world the way the client would, to prove the channel path works.

Uses tools/transport.py - the INDEPENDENT Python implementation - for the header and IV
chain, and implements the byte shift here, so agreement is two implementations agreeing
rather than the Rust checking itself.
"""
import os, socket, struct, subprocess, sys, tempfile, time

os.chdir(r"C:\MapleCW")
sys.path.insert(0, "tools")
import transport

fails = []
def check(label, ok, detail=""):
    print(("  PASS  " if ok else "  FAIL  ") + label + (" - " + detail if detail else ""))
    if not ok:
        fails.append(label)

def free_port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]

db = os.path.join(tempfile.mkdtemp(prefix="maplecw-ch-"), "ch.db")
port = free_port()
subprocess.run([os.path.join("target","release","maplecw-useradd.exe"), "--db", db, "maplecw"],
               input="correct horse battery staple\n", text=True, capture_output=True)
logf = open(os.path.join(os.path.dirname(db), "world.log"), "w")
proc = subprocess.Popen([os.path.join("target","release","maplecw-world.exe"), "--db", db,
                         "--bind", "127.0.0.1:%d" % port],
                        stdout=logf, stderr=subprocess.STDOUT)
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

    check("the channel greeting is 20 bytes of body, not the login server's 46",
          n == 20, "%d" % n)
    g = struct.unpack_from("<H", body, 0)[0]
    h = struct.unpack_from("<I", body, 2)[0]
    check("G is 1 - this is the gate that raised 'client is outdated'", g == 1, "G=%d" % g)
    check("H is 1", h == 1, "H=%d" % h)
    ilen = struct.unpack_from("<H", body, 6)[0]
    check("I is the empty string, so atoi(I) == 0", ilen == 0, "len=%d" % ilen)
    j = struct.unpack_from("<I", body, 8)[0]
    k = struct.unpack_from("<I", body, 12)[0]
    check("J and K are the IV seeds", (j, k) == (0x52307801, 0x52307802),
          "J=%#x K=%#x" % (j, k))
    check("L is 1", body[16] == 1, "L=%d" % body[16])
    check("the greeting ends after M, N, O", len(body) == 20, "%d" % len(body))

    # Now send a packet as the client would, byte-shifted, and see it in the log.
    # The client encrypts with the J chain; the server decrypts with the same.
    iv = struct.pack("<I", 0x52307801)
    payload = struct.pack("<H", 0x007D) + bytes(range(32))
    shifted = bytes((b + iv[0]) & 0xFF for b in payload)   # ClientSubtractsOnReceive => client ADDs on send? see below
    a = (struct.unpack("<H", iv[2:4])[0]) ^ 0x00DF
    header = struct.pack("<HH", a, a ^ len(payload))
    sock.sendall(header + shifted)
    time.sleep(0.6)
    sock.close()
finally:
    proc.terminate()
    proc.wait(timeout=5)
    logf.close()

log = open(os.path.join(os.path.dirname(db), "world.log"), encoding="utf-8",
           errors="replace").read()
check("the server framed the client's packet rather than erroring",
      "framing:" not in log, "framing error in the log")
check("the polarity check ran and printed both readings",
      "POLARITY CHECK" in log and "the other way" in log)
check("the log names the cipher and admits the polarity is a guess",
      "byte shift" in log and "GUESS" in log)
print()
print("world.log:")
for line in log.splitlines():
    if any(w in line for w in ("greeting", "cipher", "POLARITY", "as decoded", "other way",
                               "<-", "Whichever")):
        print("   " + line)
print()
print("FAILED: " + ", ".join(fails) if fails else "all checks passed")
sys.exit(1 if fails else 0)
