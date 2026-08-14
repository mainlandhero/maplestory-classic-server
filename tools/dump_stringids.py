"""Dump and decrypt the client's numeric string-ID table from MapleStory.exe.

The client resolves user-facing messages by integer ID (`FUN_1408a9e40(dest, id)`).
The text is stored XOR-encrypted, which is why a plaintext search for a message finds
nothing.

Layout (`FUN_1408aa850`):

    locale_tables = (u64 *) 0x143A563F8      // one pointer per locale
    entries       = (u64 *) locale_tables[locale]
    entry         = entries[id]
    seed          = *(u8 *) entry            // first byte: per-string key schedule
    ciphertext    = cstring at entry + 1

Key schedule (`FUN_1408aa600`), starting from a 16-byte base key:

    if seed >= 8:  rotate the key left by ((seed >> 3) % 16) *bytes*
    then           rotate the whole key left by (seed & 7) *bits*

Decryption (`FUN_1408aadb0`) is repeating-key XOR with one quirk: a byte that would
decrypt to NUL keeps the key byte instead, so the string never gains a terminator.

    python tools/dump_stringids.py [--locale N] [--grep TEXT] [--id N]
"""

import argparse
import struct

import pefile

EXE = r"C:/Nexon/Library/maplestorycw/appdata/MapleStory.exe"
LOCALE_TABLE_VA = 0x143A563F8
BASE_KEY_VA = 0x1432BA470
KEY_LEN = 0x10
MAX_IDS = 0x1815


class Image:
    def __init__(self, path: str):
        self.pe = pefile.PE(path, fast_load=True)
        self.base = self.pe.OPTIONAL_HEADER.ImageBase
        self.data = open(path, "rb").read()

    def off(self, va: int):
        try:
            return self.pe.get_offset_from_rva(va - self.base)
        except Exception:
            return None

    def u64(self, va: int):
        o = self.off(va)
        if o is None or o + 8 > len(self.data):
            return None
        return struct.unpack_from("<Q", self.data, o)[0]

    def read(self, va: int, n: int):
        o = self.off(va)
        return None if o is None else self.data[o : o + n]

    def cbytes(self, va: int, limit: int = 4096):
        o = self.off(va)
        if o is None:
            return None
        end = self.data.find(b"\x00", o, o + limit)
        return None if end < 0 else self.data[o:end]


def derive_key(base: bytes, seed: int) -> bytes:
    """Rotate the base key per the client's schedule."""
    key = bytearray(base)
    n = len(key)
    if seed == 0 or n == 0:
        return bytes(key)

    if seed >= 8:
        r = (seed >> 3) % n
        if r:
            key = bytearray(key[r:] + key[:r])  # rotate left by whole bytes

    bits = seed & 7
    if bits:
        sh = 8 - bits
        carry = (key[0] >> sh) if n >= 2 else 0
        out = bytearray(n)
        for i in range(n):
            nxt = 0 if i == n - 1 else (key[i + 1] >> sh)
            out[i] = ((key[i] << bits) & 0xFF) | nxt
        out[n - 1] |= carry
        key = out
    return bytes(key)


def decrypt(cipher: bytes, key: bytes) -> bytes:
    out = bytearray(len(cipher))
    for i, c in enumerate(cipher):
        k = key[i % len(key)]
        p = c ^ k
        if p == 0:  # never introduce a NUL
            p = k
        out[i] = p
    return bytes(out)


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--locale", type=int, default=0)
    ap.add_argument("--grep", type=str, default=None)
    ap.add_argument("--id", type=int, default=None)
    ap.add_argument("--max", type=int, default=MAX_IDS)
    ap.add_argument("--all-locales", action="store_true")
    args = ap.parse_args()

    img = Image(EXE)
    base_key = img.read(BASE_KEY_VA, KEY_LEN)
    if not base_key:
        raise SystemExit("could not read the base key")

    locales = range(8) if args.all_locales else [args.locale]
    for loc in locales:
        table_va = img.u64(LOCALE_TABLE_VA + loc * 8)
        if not table_va or img.off(table_va) is None:
            continue

        ids = [args.id] if args.id is not None else range(args.max)
        rows, total = [], 0
        for sid in ids:
            entry = img.u64(table_va + sid * 8)
            if not entry:
                continue
            seed_b = img.read(entry, 1)
            if not seed_b:
                continue
            cipher = img.cbytes(entry + 1)
            if cipher is None:
                continue
            text = decrypt(cipher, derive_key(base_key, seed_b[0]))
            try:
                s = text.decode("utf-8")
            except UnicodeDecodeError:
                s = text.decode("latin1")
            total += 1
            if args.grep and args.grep.lower() not in s.lower():
                continue
            rows.append((sid, s))

        print(f"=== locale {loc}: {total} strings (table 0x{table_va:X}) ===")
        limit = len(rows) if (args.grep or args.id is not None) else 40
        for sid, s in rows[:limit]:
            shown = s if len(s) < 240 else s[:240] + "..."
            print(f"  [{sid:5d}] 0x{sid:04X}  {shown!r}")
        if not args.grep and args.id is None and total > limit:
            print(f"  ... ({total} total; use --grep to filter)")
        print()


if __name__ == "__main__":
    main()
