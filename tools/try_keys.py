"""Sweep AES key/IV interpretations against the captured stream.

The oracle is structural, not a guessed opcode: a correct decryption makes *every*
packet's first two bytes a small, plausible opcode, and makes the eleven 6-byte
packets share one.
"""

import struct
import sys

from Crypto.Cipher import AES

sys.path.insert(0, "tools")
from decrypt_capture import maple_crypt, next_iv, parse_log, shanda_decrypt, split_packets

EXE = "client-patched/MapleStory.exe"
KEY_VA = 0x143A86810
SHUFFLE_VA = 0x143A86890
IMAGE_BASE = 0x140000000
DATA_RVA = 0x3A86810 - 0x3A84C10  # keeps the arithmetic honest below


def at(va, n):
    from dump_va import load_sections, va_to_off

    data, base, sections = load_sections(EXE)
    off, _ = va_to_off(va, base, sections)
    return data[off : off + n]


def keys():
    tbl = at(KEY_VA, 128)
    words = struct.unpack("<32I", tbl)

    # What FUN_140c76070 actually does: param_1[0], [4], [8] ... [0x1c] as u32 words.
    stock_words = [words[i] for i in range(0, 32, 4)]
    yield "stock (dwords at stride 4, LE)", b"".join(struct.pack("<I", w) for w in stock_words)
    yield "stock (dwords at stride 4, BE)", b"".join(struct.pack(">I", w) for w in stock_words)
    yield "stock 8 bytes padded", bytes(w & 0xFF for w in stock_words) + b"\0" * 24

    # The old, wrong reading this repo shipped with: low byte of all 32 dwords.
    yield "all-32 low bytes (old theory)", bytes(w & 0xFF for w in words)

    yield "raw first 32 bytes", tbl[:32]


def score(plain, length):
    """Cheap plausibility: a real packet starts with a small opcode."""
    if len(plain) < 2:
        return -1
    op = int.from_bytes(plain[:2], "little")
    return 1 if op < 0x400 else 0


def main():
    log = sys.argv[1] if len(sys.argv) > 1 else "research/fixtures/capture-handshake-ok.log"
    shuffle = at(SHUFFLE_VA, 256)
    stream = parse_log(log)
    pkts, used = split_packets(stream)
    print(f"capture: {len(stream)} bytes -> {len(pkts)} packets ({used} consumed)\n")

    for seed_name, seed in (("J=0x52307801", 0x52307801), ("K=0x52307802", 0x52307802)):
        for key_name, key in keys():
            if len(key) not in (16, 24, 32):
                continue
            for shanda in (False, True):
                iv = struct.pack("<I", seed)
                ops, good = [], 0
                for hdr, payload in pkts:
                    plain = maple_crypt(payload, iv, key)
                    if shanda:
                        plain = shanda_decrypt(plain)
                    ops.append(int.from_bytes(plain[:2], "little"))
                    good += score(plain, len(payload))
                    iv = next_iv(iv, shuffle)
                tag = f"{seed_name} | {key_name} | shanda={int(shanda)}"
                six = [o for (h, p), o in zip(pkts, ops) if len(p) == 6]
                uniq = len(set(six))
                print(
                    f"{good:2d}/{len(pkts)} small-op  6byte-uniq={uniq:2d}  {tag}\n"
                    f"    ops: {' '.join(f'{o:04X}' for o in ops)}"
                )


if __name__ == "__main__":
    main()
