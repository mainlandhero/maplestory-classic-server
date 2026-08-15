"""Structural sweep: IV position x keystream alignment, scored without guessing opcodes.

A correct MapleStory plaintext is zero-rich (u32 fields holding small values); a wrong
key/IV/alignment gives uniform random bytes. Over a 47-byte packet the expected zero
count is 0.18 when wrong, so anything >= 4 is a loud signal.
"""

import struct
import sys

from Crypto.Cipher import AES

sys.path.insert(0, "tools")
from decrypt_capture import next_iv, parse_log, split_packets
from dump_va import load_sections, va_to_off

EXE = "client-patched/MapleStory.exe"
STOCK_KEY = bytes.fromhex(
    "13000000" "08000000" "06000000" "b4000000" "1b000000" "0f000000" "33000000" "52000000"
)


def at(va, n):
    data, base, sections = load_sections(EXE)
    off, _ = va_to_off(va, base, sections)
    return data[off : off + n]


def keystream(iv, key, n):
    """OFB keystream: block = AES(block), starting from iv repeated 4x."""
    aes = AES.new(key, AES.MODE_ECB)
    block = bytes(iv) * 4
    out = b""
    while len(out) < n:
        block = aes.encrypt(block)
        out += block
    return out[:n]


def main():
    shuffle = at(0x143A86890, 256)
    pkts, _ = split_packets(parse_log("research/fixtures/capture-handshake-ok.log"))

    chains = {}
    for name, seed in (("J", 0x52307801), ("K", 0x52307802)):
        iv = struct.pack("<I", seed)
        c = []
        for _ in range(24):
            c.append(iv)
            iv = next_iv(iv, shuffle)
        chains[name] = c

    results = []
    for pi, (hdr, payload) in enumerate(pkts):
        for cname, chain in chains.items():
            for pos, iv in enumerate(chain):
                ks = keystream(iv, STOCK_KEY, len(payload) + 16)
                for off in range(0, 9):
                    plain = bytes(c ^ k for c, k in zip(payload, ks[off:]))
                    zeros = plain.count(0)
                    results.append((zeros, pi, len(payload), cname, pos, off, plain))

    results.sort(key=lambda r: -r[0])
    print("top scorers (zeros / packet / len / chain / ivpos / ksoff):")
    for zeros, pi, ln, cname, pos, off, plain in results[:15]:
        print(f"  {zeros:2d}z  pkt{pi:2d} len={ln:3d} {cname} ivpos={pos:2d} ksoff={off}  {plain[:20].hex(' ')}")

    # Baseline: what does chance look like here?
    big = [r for r in results if r[2] >= 47]
    avg = sum(r[0] for r in big) / max(len(big), 1)
    print(f"\nmean zeros over {len(big)} trials on >=47-byte packets: {avg:.2f}")
    best = max(big, key=lambda r: r[0])
    print(f"best on a big packet: {best[0]} zeros  pkt{best[1]} {best[3]} ivpos={best[4]} ksoff={best[5]}")


if __name__ == "__main__":
    main()
