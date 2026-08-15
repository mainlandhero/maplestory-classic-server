"""Decrypt a captured client->server stream from tools/handshake_probe.py's log.

The client uses the classic MapleStory transport, confirmed three ways:

  * framing      4-byte header, length = LOWORD ^ HIWORD
  * IV shuffle   the stock 256-byte table, present verbatim at VA 0x143A86890
  * block size   0x5B4, which also shows up as FUN_1406f1730(0x5b4, ...) in the
                 handshake handler

The AES key is a *customised* variant of the stock one. The client stores it the classic
way - 32 dwords, one key byte in each - at VA 0x143A86810, directly before the shuffle
table. Every fourth byte matches the well-known MapleStory key (13 08 06 B4 1B 0F 33 52);
the bytes that are zero in the stock key have been filled in with other values here.

    python tools/decrypt_capture.py [--log probe.log] [--iv 52307801]
"""

import argparse
import re
import struct

import pefile
from Crypto.Cipher import AES

EXE = r"C:/Nexon/Library/maplestorycw/appdata/MapleStory.exe"
KEY_VA = 0x143A86810
SHUFFLE_VA = 0x143A86890


def read_client_tables():
    pe = pefile.PE(EXE, fast_load=True)
    base = pe.OPTIONAL_HEADER.ImageBase
    data = open(EXE, "rb").read()

    def at(va, n):
        return data[pe.get_offset_from_rva(va - base) : pe.get_offset_from_rva(va - base) + n]

    # 32 dwords, low byte of each is the key byte
    key = bytes(struct.unpack_from("<32I", at(KEY_VA, 128))[i] & 0xFF for i in range(32))
    return key, at(SHUFFLE_VA, 256)


def parse_log(path):
    """Pull the client->server bytes back out of the probe's hexdump."""
    blocks, cur = [], None
    for line in open(path).read().splitlines():
        if "CLIENT SENT" in line:
            cur = bytearray()
            blocks.append(cur)
        elif cur is not None and re.match(r"^ {6}[0-9A-F]{4}  ", line):
            cur += bytes(int(b, 16) for b in line[12:59].split())
        elif cur is not None and not line.startswith("      "):
            cur = None
    return b"".join(bytes(b) for b in blocks)


def split_packets(stream):
    out, off = [], 0
    while off + 4 <= len(stream):
        hdr = stream[off : off + 4]
        ln = int.from_bytes(hdr[:2], "little") ^ int.from_bytes(hdr[2:], "little")
        if off + 4 + ln > len(stream):
            break
        out.append((hdr, stream[off + 4 : off + 4 + ln]))
        off += 4 + ln
    return out, off


def maple_crypt(data, iv, key):
    """AES-256-ECB run as a keystream over the 4-byte IV repeated 4x.

    The first chunk is 0x5B0 bytes and every chunk after it 0x5B4 - an oddity of the
    original implementation that only shows up on packets larger than 1456 bytes.
    """
    aes = AES.new(key, AES.MODE_ECB)
    data = bytearray(data)
    pos, remaining, chunk = 0, len(data), 0x5B0
    while remaining > 0:
        block = iv * 4
        if remaining < chunk:
            chunk = remaining
        for x in range(chunk):
            if x % 16 == 0:
                block = aes.encrypt(bytes(block))
            data[pos + x] ^= block[x % 16]
        pos += chunk
        remaining -= chunk
        chunk = 0x5B4
    return bytes(data)


def next_iv(iv, shuffle):
    """The stock IV evolution."""
    out = bytearray(b"\xf2\x53\x50\xc6")
    for i in range(4):
        b = iv[i]
        s = shuffle[b]
        out[0] = (out[0] + (shuffle[out[1] & 0xFF] - b)) & 0xFF
        out[1] = (out[1] - (out[2] ^ (s & 0xFF))) & 0xFF
        out[2] ^= (shuffle[out[3] & 0xFF] + b) & 0xFF
        out[3] = (out[3] - (out[0] - (s & 0xFF))) & 0xFF
        val = (out[0] | (out[1] << 8) | (out[2] << 16) | (out[3] << 24)) & 0xFFFFFFFF
        val2 = ((val >> 0x1D) | (val << 3)) & 0xFFFFFFFF
        for j in range(4):
            out[j] = (val2 >> (8 * j)) & 0xFF
    return bytes(out)


def rl(b, n):
    n &= 7
    return ((b << n) | (b >> (8 - n))) & 0xFF


def rr(b, n):
    n &= 7
    return ((b >> n) | (b << (8 - n))) & 0xFF


def shanda_decrypt(d):
    d = bytearray(d)
    for j in range(1, 7):
        rem, dl = 0, len(d) & 0xFF
        if j % 2 == 0:
            for i in range(len(d)):
                c = rl((~((d[i] - 0x48) & 0xFF)) & 0xFF, dl)
                nxt = c
                c ^= rem
                rem = nxt
                d[i] = rr((c - dl) & 0xFF, 3)
                dl = (dl - 1) & 0xFF
        else:
            for i in range(len(d) - 1, -1, -1):
                c = rl(d[i], 3) ^ 0x13
                nxt = c
                c ^= rem
                rem = nxt
                d[i] = rr((c - dl) & 0xFF, 4)
                dl = (dl - 1) & 0xFF
    return bytes(d)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--log", default="probe.log")
    ap.add_argument("--iv", default="52307801", help="the u32 we sent as J (or K)")
    ap.add_argument("--no-shanda", action="store_true")
    args = ap.parse_args()

    key, shuffle = read_client_tables()
    print(f"AES key   : {key.hex(' ')}")
    print(f"  stock bytes at 0,4,8..: {bytes(key[i] for i in range(0,32,4)).hex(' ')}")

    stream = parse_log(args.log)
    pkts, used = split_packets(stream)
    print(f"capture   : {len(stream)} bytes -> {len(pkts)} packets ({used} consumed)\n")

    iv = struct.pack("<I", int(args.iv, 16))
    hdr_a = int.from_bytes(pkts[0][0][:2], "little")
    print(f"iv        : {iv.hex(' ')}  (iv[3]<<8|iv[2] = 0x{(iv[3] << 8) | iv[2]:04X})")
    print(f"header a  : 0x{hdr_a:04X}  -> xor constant 0x{((iv[3] << 8) | iv[2]) ^ hdr_a:04X}\n")

    for i, (hdr, payload) in enumerate(pkts, 1):
        plain = maple_crypt(payload, iv, key)
        if not args.no_shanda:
            plain = shanda_decrypt(plain)
        op = int.from_bytes(plain[:2], "little")
        print(f"pkt {i:2d} len={len(payload):3d} iv={iv.hex()} op=0x{op:04X} ({op:5d}) {plain[:24].hex(' ')}")
        iv = next_iv(iv, shuffle)


if __name__ == "__main__":
    main()
