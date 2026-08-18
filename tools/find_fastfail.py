"""Find `int 0x29` (__fastfail) sites in the client image.

The client exits with 0xC0000409 = STATUS_STACK_BUFFER_OVERRUN, which on x64 is what
`int 29h` produces. It bypasses vectored handlers entirely, which is why every in-process
instrument stayed silent. `__fastfail` is a two-byte instruction, CD 29, so the sites can
be enumerated from the image without running anything.
"""
import struct
import sys

path = sys.argv[1] if len(sys.argv) > 1 else r"client-patched\MapleStory.exe"
data = open(path, "rb").read()

pe = struct.unpack_from("<I", data, 0x3C)[0]
assert data[pe:pe + 4] == b"PE\0\0", "not a PE"
nsec = struct.unpack_from("<H", data, pe + 6)[0]
opt = struct.unpack_from("<H", data, pe + 20)[0]
image_base = struct.unpack_from("<Q", data, pe + 24 + 24)[0]
sec_off = pe + 24 + opt

sections = []
for i in range(nsec):
    o = sec_off + i * 40
    name = data[o:o + 8].rstrip(b"\0").decode("latin1")
    vsize, va, rawsize, raw = struct.unpack_from("<IIII", data, o + 8)
    chars = struct.unpack_from("<I", data, o + 36)[0]
    sections.append((name, va, vsize, raw, rawsize, chars))

print("image base %#x" % image_base)
print("%-10s %-12s %-12s %-12s %s" % ("section", "VA", "vsize", "raw", "exec"))
for name, va, vsize, raw, rawsize, chars in sections:
    print("%-10s %#-12x %-12d %#-12x %s" % (name, image_base + va, vsize, raw,
                                            "X" if chars & 0x20000000 else ""))

print()
total = 0
for name, va, vsize, raw, rawsize, chars in sections:
    if not (chars & 0x20000000):
        continue
    blob = data[raw:raw + rawsize]
    hits = []
    start = 0
    while True:
        i = blob.find(b"\xcd\x29", start)
        if i < 0:
            break
        hits.append(image_base + va + i)
        start = i + 1
    total += len(hits)
    print("%s: %d candidate `int 29h` sites" % (name, len(hits)))
    for h in hits[:60]:
        print("    %#x" % h)
    if len(hits) > 60:
        print("    ... and %d more" % (len(hits) - 60))
print("\ntotal %d" % total)
