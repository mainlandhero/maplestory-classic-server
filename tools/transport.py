"""Server -> client packet transport.

Everything here is read out of the client's own receive path, not inferred:

  FUN_1406e9530   framing: two u16s, `len = a ^ b`; >= 0xFF00 escapes to a 32-bit length
  FUN_1406e97e0   header rule: the client drops us unless `(iv >> 16) ^ a == 0xFFFE`
  FUN_1406e99e0   AES over `buf + 4` for `len` bytes, chunked 0x5B0 then 0x5B4
  FUN_140c75880   AES-256-OFB, IV = the 4-byte IV repeated 4x
  FUN_140c78630   the IV evolves once per packet, stock shuffle table

Note the header constant differs by direction: the client *sends* with 0x00DF (measured
across 16/16 captured packets) and *requires* 0xFFFE on what it receives.
"""

import struct

KEY_VA = 0x143A86810
SHUFFLE_VA = 0x143A86890

# The real AES-256 key, read out of the *running* client with tools/dump_runtime.py.
#
# The bytes on disk at KEY_VA are the stock MapleStory key (13 08 06 B4 1B 0F 33 52) and
# are a decoy: Themida overwrites the low byte of all 32 dwords at startup. Only the key
# table is patched - the IV shuffle table beside it is untouched, which is exactly why
# framing, the header constant and the IV chain all worked while everything AES failed in
# both directions.
#
# Verified against eight captures from different sessions: packet 1 decrypts to
# `70 00 02 64 00 00 00`, matching FUN_1415d5b40's `u8 2, u32 100` byte for byte. Stable
# across sessions, so it is a build constant rather than a session secret.
REAL_KEY = bytes.fromhex(
    "0f000000" "1b000000" "c5000000" "46000000" "f3000000" "be000000" "ff000000" "75000000"
)
DECOY_KEY = bytes.fromhex(
    "13000000" "08000000" "06000000" "b4000000" "1b000000" "0f000000" "33000000" "52000000"
)

# Hardcoded comparison in FUN_1415d36c0 / FUN_1415e7090 against FUN_1406e97e0's result.
RECV_CONST = 0xFFFE
# What the client uses on its own outbound packets; kept for decoding captures.
SEND_CONST = 0x00DF

EXTENDED_LEN = 0xFF00


def load_tables(exe=None):
    """The AES key and IV shuffle table, straight out of the client image."""
    import os
    import sys

    here = os.path.dirname(os.path.abspath(__file__))
    sys.path.insert(0, here)
    if exe is None:
        exe = os.path.join(os.path.dirname(here), "client-patched", "MapleStory.exe")

    from dump_va import load_sections, va_to_off

    data, base, sections = load_sections(exe)

    def at(va, n):
        off, _ = va_to_off(va, base, sections)
        return data[off : off + n]

    # The key must come from REAL_KEY, not from the image: the on-disk table is a decoy
    # that the client patches at startup. The shuffle table *is* genuine on disk.
    return REAL_KEY, at(SHUFFLE_VA, 256)


# --------------------------------------------------------------------------------------
# AES-256, in pure Python.
#
# Deliberately dependency-free: the probe is launched by test-one.ps1 through whichever
# `python` is on PATH, and pycryptodome not being installed there once killed the serve
# thread mid-test. The client then dropped the connection because the *server* had gone
# away, which looked exactly like a rejected header. One block per packet, so speed is
# irrelevant here.
#
# The S-box is generated the same way the client generates it (FUN_140c759a0): GF(2^8)
# log/antilog with the 0x1b polynomial, then the affine transform with constant 0x63.
# --------------------------------------------------------------------------------------


def _build_sbox():
    alog = [1] * 256
    x = 1
    for i in range(1, 256):
        x = (x ^ ((x << 1) & 0xFF) ^ (0x1B if x & 0x80 else 0)) & 0xFF
        alog[i] = x
    log = [0] * 256
    for i in range(255):
        log[alog[i]] = i

    sbox = [0] * 256
    for i in range(256):
        inv = 0 if i == 0 else alog[255 - log[i]]
        acc, rot = inv, inv
        for _ in range(4):
            rot = ((rot << 1) | (rot >> 7)) & 0xFF
            acc ^= rot
        sbox[i] = acc ^ 0x63
    return sbox


SBOX = _build_sbox()


def _xtime(a):
    a <<= 1
    return (a ^ 0x1B) & 0xFF if a & 0x100 else a


def expand_key(key):
    """AES-256 key schedule: Nk = 8, Nr = 14, 60 words."""
    nk, nr = 8, 14
    w = [list(key[4 * i : 4 * i + 4]) for i in range(nk)]
    rcon = 1
    for i in range(nk, 4 * (nr + 1)):
        t = list(w[i - 1])
        if i % nk == 0:
            t = t[1:] + t[:1]
            t = [SBOX[b] for b in t]
            t[0] ^= rcon
            rcon = _xtime(rcon)
        elif i % nk == 4:
            t = [SBOX[b] for b in t]
        w.append([w[i - nk][j] ^ t[j] for j in range(4)])
    return w


def encrypt_block(block, w):
    nr = 14
    s = list(block)
    for c in range(4):
        for r in range(4):
            s[4 * c + r] ^= w[c][r]

    for rnd in range(1, nr + 1):
        s = [SBOX[b] for b in s]
        shifted = list(s)
        for r in range(1, 4):
            for c in range(4):
                shifted[4 * c + r] = s[4 * ((c + r) % 4) + r]
        s = shifted
        if rnd != nr:
            for c in range(4):
                a = s[4 * c : 4 * c + 4]
                t = a[0] ^ a[1] ^ a[2] ^ a[3]
                s[4 * c + 0] = a[0] ^ t ^ _xtime(a[0] ^ a[1])
                s[4 * c + 1] = a[1] ^ t ^ _xtime(a[1] ^ a[2])
                s[4 * c + 2] = a[2] ^ t ^ _xtime(a[2] ^ a[3])
                s[4 * c + 3] = a[3] ^ t ^ _xtime(a[3] ^ a[0])
        for c in range(4):
            for r in range(4):
                s[4 * c + r] ^= w[4 * rnd + c][r]
    return bytes(s)


def next_iv(iv, shuffle):
    out = bytearray(b"\xf2\x53\x50\xc6")
    for i in range(4):
        b = iv[i]
        s = shuffle[b]
        out[0] = (out[0] + (shuffle[out[1]] - b)) & 0xFF
        out[1] = (out[1] - (out[2] ^ s)) & 0xFF
        out[2] ^= (shuffle[out[3]] + b) & 0xFF
        out[3] = (out[3] - (out[0] - s)) & 0xFF
        val = out[0] | (out[1] << 8) | (out[2] << 16) | (out[3] << 24)
        val = ((val >> 0x1D) | (val << 3)) & 0xFFFFFFFF
        for j in range(4):
            out[j] = (val >> (8 * j)) & 0xFF
    return bytes(out)


def ofb(data, iv, key):
    """AES-256-OFB, restarted at every chunk boundary (0x5B0 then 0x5B4)."""
    w = expand_key(key)
    data = bytearray(data)
    pos, remaining, chunk = 0, len(data), 0x5B0
    while remaining > 0:
        block = bytes(iv) * 4
        n = min(chunk, remaining)
        for x in range(n):
            if x % 16 == 0:
                block = encrypt_block(block, w)
            data[pos + x] ^= block[x % 16]
        pos += n
        remaining -= n
        chunk = 0x5B4
    return bytes(data)


def header(iv, length, const=RECV_CONST):
    """The 4- (or 8-) byte frame header the client will accept."""
    a = (((int.from_bytes(iv, "little") >> 16) & 0xFFFF) ^ const) & 0xFFFF
    if length < EXTENDED_LEN:
        return struct.pack("<HH", a, a ^ length)
    # FUN_1406e9530: a short length of >= 0xFF00 means a 32-bit length follows.
    return struct.pack("<HH", a, a ^ 0xFFFF) + struct.pack("<I", length ^ a)


class ServerCipher:
    """Encodes packets in the direction the client reads (its conn+0xec chain)."""

    def __init__(self, iv, key, shuffle):
        self.iv = struct.pack("<I", iv) if isinstance(iv, int) else bytes(iv)
        self.key = key
        self.shuffle = shuffle

    def encode(self, payload):
        frame = header(self.iv, len(payload)) + ofb(payload, self.iv, self.key)
        self.iv = next_iv(self.iv, self.shuffle)
        return frame

    def peek_header(self, length):
        """A header without consuming the IV - for framing-only tests."""
        return header(self.iv, length)


def packet(opcode, body=b""):
    return struct.pack("<H", opcode) + body


# --------------------------------------------------------------------------------------
# Live decoding of the client -> server stream.
# --------------------------------------------------------------------------------------

# Opcodes decoded from real captures and matched against their decompiled builders.
CLIENT_OPCODES = {
    0x0070: "version report (FUN_1415d5b40)",
    0x0071: "environment report (FUN_1415d5c20)",
    0x008F: "log upload A (FUN_1415dde80)",
    0x0090: "log upload B (FUN_1415ddf60)",
    0x0091: "log upload C (FUN_1415de040)",
    0x00A1: "handshake-tail notify",
    0x009E: "periodic status (FUN_142c4ef20) - client-initiated, not a reply",
    0x00A6: "enumeration entry",
    0x00B5: "version mismatch report",
    0x007D: "game-connection hello",
}


def describe(body):
    """A short, honest rendering of a packet body: no invented field structure."""
    if len(body) < 2:
        return ""
    rest = body[2:]
    bits = []
    if len(rest) == 4:
        bits.append(f"u32={int.from_bytes(rest, 'little')}")
    elif len(rest) == 1:
        bits.append(f"u8={rest[0]}")
    elif 0 < len(rest) <= 24:
        bits.append("body=" + rest.hex(" "))
    elif rest:
        bits.append(f"body={len(rest)}B {rest[:16].hex(' ')}...")
    return "  ".join(bits)


class ClientDecoder:
    """Reassembles and decrypts the client's stream, which is framed exactly like ours.

    Handles TCP coalescing and splits: `feed` takes whatever `recv` returned and yields
    only whole packets. The IV rolls once per packet, so a desync is unrecoverable — hence
    `header_ok`, which reports whether the frame's `a` matched the IV we expected.
    """

    def __init__(self, iv, key, shuffle):
        self.iv = struct.pack("<I", iv) if isinstance(iv, int) else bytes(iv)
        self.key = key
        self.shuffle = shuffle
        self.buf = bytearray()
        self.count = 0

    def feed(self, data):
        self.buf += data
        out = []
        while True:
            if len(self.buf) < 4:
                return out
            a = int.from_bytes(self.buf[0:2], "little")
            b = int.from_bytes(self.buf[2:4], "little")
            length = a ^ b
            head = 4
            if length >= EXTENDED_LEN:
                if len(self.buf) < 8:
                    return out
                length = int.from_bytes(self.buf[4:8], "little") ^ a
                head = 8
            if len(self.buf) < head + length:
                return out

            expected = (((int.from_bytes(self.iv, "little") >> 16) & 0xFFFF) ^ SEND_CONST) & 0xFFFF
            payload = bytes(self.buf[head : head + length])
            del self.buf[: head + length]

            plain = ofb(payload, self.iv, self.key)
            self.iv = next_iv(self.iv, self.shuffle)
            self.count += 1
            out.append({
                "n": self.count - 1,
                "opcode": int.from_bytes(plain[:2], "little") if len(plain) >= 2 else None,
                "body": plain,
                "header_ok": a == expected,
            })
