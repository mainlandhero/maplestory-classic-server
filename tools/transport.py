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

from Crypto.Cipher import AES

KEY_VA = 0x143A86810
SHUFFLE_VA = 0x143A86890

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

    # FUN_140c76070 reads 8 key words at a stride of 4 dwords; the 24 dwords in between
    # are decoys that are never read.
    words = struct.unpack("<32I", at(KEY_VA, 128))
    key = b"".join(struct.pack("<I", words[i]) for i in range(0, 32, 4))
    return key, at(SHUFFLE_VA, 256)


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
    aes = AES.new(key, AES.MODE_ECB)
    data = bytearray(data)
    pos, remaining, chunk = 0, len(data), 0x5B0
    while remaining > 0:
        block = bytes(iv) * 4
        n = min(chunk, remaining)
        for x in range(n):
            if x % 16 == 0:
                block = aes.encrypt(block)
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
