"""Re-verify the IV chain, then sweep IV positions against the *correct* stock key.

Every earlier sweep ran with the 32-decoy-byte key, so its negative results say
nothing. This redoes them.
"""

import struct
import sys

from Crypto.Cipher import AES

sys.path.insert(0, "tools")
from decrypt_capture import maple_crypt, next_iv, parse_log, shanda_decrypt, split_packets
from dump_va import load_sections, va_to_off

EXE = "client-patched/MapleStory.exe"
STOCK_KEY = bytes.fromhex(
    "13000000" "08000000" "06000000" "b4000000" "1b000000" "0f000000" "33000000" "52000000"
)


def at(va, n):
    data, base, sections = load_sections(EXE)
    off, _ = va_to_off(va, base, sections)
    return data[off : off + n]


def verify_chain(pkts, shuffle, seed):
    """Does the header sequence really follow from this seed?"""
    iv = struct.pack("<I", seed)
    hits = 0
    const = None
    for hdr, _ in pkts:
        a = int.from_bytes(hdr[:2], "little")
        c = (((iv[3] << 8) | iv[2]) ^ a) & 0xFFFF
        if const is None:
            const = c
        if c == const:
            hits += 1
        iv = next_iv(iv, shuffle)
    return hits, const


def main():
    log = "research/fixtures/capture-handshake-ok.log"
    shuffle = at(0x143A86890, 256)
    pkts, used = split_packets(parse_log(log))
    print(f"{len(pkts)} packets, {used} bytes consumed\n")

    for name, seed in (("J", 0x52307801), ("K", 0x52307802)):
        hits, const = verify_chain(pkts, shuffle, seed)
        print(f"chain from {name}: {hits}/{len(pkts)} headers share xor const 0x{const:04X}")

    # Build a long IV chain from each seed and try every position on every packet,
    # looking for the opcodes the postshake decompile says the client sends.
    print("\nsweeping IV positions with the stock key ...")
    wanted = (0x70, 0x71)
    found = []
    for name, seed in (("J", 0x52307801), ("K", 0x52307802)):
        iv = struct.pack("<I", seed)
        chain = []
        for _ in range(40):
            chain.append(iv)
            iv = next_iv(iv, shuffle)
        for pos, ivp in enumerate(chain):
            for pi, (hdr, payload) in enumerate(pkts):
                for shanda in (False, True):
                    plain = maple_crypt(payload, ivp, STOCK_KEY)
                    if shanda:
                        plain = shanda_decrypt(plain)
                    op = int.from_bytes(plain[:2], "little")
                    if op in wanted:
                        found.append((name, pos, pi, shanda, plain))
                        print(
                            f"  HIT seed={name} ivpos={pos} pkt={pi} shanda={int(shanda)} "
                            f"op=0x{op:04X} {plain.hex(' ')}"
                        )
    if not found:
        print("  no hit for opcode 0x70/0x71 anywhere in 40 chain positions")
    return 0


if __name__ == "__main__":
    sys.exit(main())
