"""Turn `wz-dump canvas` output into PNGs.

Why this exists: a lot of this client's on-screen wording is **baked into bitmaps**, not
stored as text. "Having trouble logging in?" appears nowhere in the executable's 6165
encrypted messages, nowhere in Login.img's string values, and nowhere in the install as
ASCII or UTF-16 - because it is pixels. Without a way to render canvases we cannot read
what the client is actually saying, and analysis turns into guessing from surrounding code.

Split of work: `wz-dump canvas` exports each canvas payload plus a manifest, because it
owns the WZ format; `wz` is deliberately dependency-free and inflating needs zlib. This
side has zlib and zlib-based PNG writing in the standard library, so nothing new is
required anywhere.

    wz-dump canvas <archive.wz> <Image.img> <dir> [node filter]
    python tools/wz_png.py <dir>
"""

from __future__ import annotations

import binascii
import json
import struct
import sys
import zlib
from pathlib import Path

# Canvas pixel formats. Only what this client actually uses is implemented; anything else
# is reported by name rather than guessed at, because a wrong decode produces a plausible
# looking image and that is worse than no image.
#
# Login.img alone is 1014 format 1, 135 format 2 and 1 format 513, so handling only
# BGRA8888 would leave 88% of it unreadable - including the entire /Notice/text/ message
# table, which is where the client's baked dialog wording lives.
BGRA4444 = 1
BGRA8888 = 2
RGB565 = 513

BYTES_PER_PIXEL = {BGRA4444: 2, BGRA8888: 4, RGB565: 2}


def inflate(payload: bytes) -> bytes:
    """Inflate a canvas payload, handling both framings the format allows.

    A payload is normally a single zlib stream. When it is not, it is a sequence of
    length-prefixed blocks that would each be XOR-decrypted with the WZ string key - and
    this client's key is all zeroes (`docs/` and `STATUS.md`), so concatenating them is
    already the plaintext.

    **`decompressobj`, not `decompress`.** These streams carry a zlib header and a complete
    deflate body but no valid adler32 tail, so the strict call raises "incomplete or
    truncated stream" on payloads that are in fact entirely intact - verified by decoding
    one and getting exactly `width * height * 4` bytes out. The caller checks that size,
    which is a stronger guarantee for our purpose than the checksum: a wrong offset or a
    wrong format cannot produce exactly the right number of pixels.
    """
    if payload[:1] == b"\x78":
        return zlib.decompressobj().decompress(payload)

    out = bytearray()
    pos = 0
    while pos + 4 <= len(payload):
        (size,) = struct.unpack_from("<i", payload, pos)
        pos += 4
        if size <= 0 or pos + size > len(payload):
            raise ValueError(f"bad chunk length {size} at offset {pos - 4}")
        out += payload[pos : pos + size]
        pos += size
    return zlib.decompressobj().decompress(bytes(out))


def to_rgba(pixels: bytes, width: int, height: int, fmt: int) -> bytes:
    """Convert decoded pixel data to RGBA, the one layout PNG wants."""
    if fmt not in BYTES_PER_PIXEL:
        raise ValueError(f"unimplemented canvas format {fmt}")

    # Exact, not ">=": this is the check that stands in for the missing adler32. A wrong
    # offset or a misread format cannot land on exactly the right byte count.
    need = width * height * BYTES_PER_PIXEL[fmt]
    if len(pixels) != need:
        raise ValueError(f"got {len(pixels)} bytes, need exactly {need} for {width}x{height}")

    if fmt == BGRA8888:
        # BGRA -> RGBA: swap the blue and red channels.
        buf = bytearray(pixels)
        buf[0::4], buf[2::4] = buf[2::4], buf[0::4]
        return bytes(buf)

    out = bytearray(width * height * 4)
    if fmt == BGRA4444:
        # Two pixels' worth of nibbles per 2 bytes: low byte holds B,G and high holds R,A.
        # Nibble n scales to 8 bits as n*17, which maps 0->0 and 15->255 exactly.
        lo = pixels[0::2]
        hi = pixels[1::2]
        out[0::4] = bytes((b & 0x0F) * 17 for b in hi)  # R
        out[1::4] = bytes((b >> 4) * 17 for b in lo)  # G
        out[2::4] = bytes((b & 0x0F) * 17 for b in lo)  # B
        out[3::4] = bytes((b >> 4) * 17 for b in hi)  # A
    else:  # RGB565 - no alpha channel in the source, so the image is fully opaque.
        for i in range(width * height):
            v = pixels[i * 2] | (pixels[i * 2 + 1] << 8)
            out[i * 4 + 0] = round(((v >> 11) & 0x1F) * 255 / 31)
            out[i * 4 + 1] = round(((v >> 5) & 0x3F) * 255 / 63)
            out[i * 4 + 2] = round((v & 0x1F) * 255 / 31)
            out[i * 4 + 3] = 255
    return bytes(out)


def write_png(path: Path, rgba: bytes, width: int, height: int) -> None:
    """Minimal PNG writer: filter byte 0 per scanline, one IDAT."""
    raw = bytearray()
    stride = width * 4
    for y in range(height):
        raw.append(0)  # filter: none
        raw += rgba[y * stride : (y + 1) * stride]

    def chunk(tag: bytes, data: bytes) -> bytes:
        return (
            struct.pack(">I", len(data))
            + tag
            + data
            + struct.pack(">I", binascii.crc32(tag + data) & 0xFFFFFFFF)
        )

    ihdr = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    path.write_bytes(
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", ihdr)
        + chunk(b"IDAT", zlib.compress(bytes(raw), 6))
        + chunk(b"IEND", b"")
    )


def main() -> int:
    if len(sys.argv) < 2:
        print(__doc__)
        return 2
    root = Path(sys.argv[1])
    keep = sys.argv[2] if len(sys.argv) > 2 else None

    manifest = json.loads((root / "manifest.json").read_text(encoding="utf-8"))
    out_dir = root / "png"
    out_dir.mkdir(exist_ok=True)

    ok = 0
    for entry in manifest:
        if keep and keep not in entry["node"]:
            continue
        name = entry["file"]
        try:
            pixels = inflate((root / name).read_bytes())
            rgba = to_rgba(pixels, entry["width"], entry["height"], entry["format"])
            dest = out_dir / (name[: -len(".bin")] + ".png")
            write_png(dest, rgba, entry["width"], entry["height"])
            ok += 1
        except Exception as exc:  # noqa: BLE001 - report and continue; one bad node
            print(f"  {entry['node']}: {exc}")  # should not stop the rest
    print(f"wrote {ok} PNGs to {out_dir}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
