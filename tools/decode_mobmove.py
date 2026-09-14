#!/usr/bin/env python3
"""Decode every `0x02FF` mob-move report in a world log, and ASSERT its total length.

    python tools/decode_mobmove.py world-ch0.log
    python tools/decode_mobmove.py                      # defaults to world-ch0.log

## Why this exists

`research/mob-behaviour.md` section 11 claims a byte layout for `0x02FF`. A layout claim with
no artefact beside it is exactly what `STATUS.md` says this project keeps paying for, so this
is the artefact: it walks every captured body field by field in the client's own encode order
and checks that head + path + tail lands **exactly** on the body length. On the capture of
2026-08-19 that is 30 of 30, at four different path-element counts.

An "N/M" result below 100% means the layout is wrong, not that the capture is odd. A wrong
field width shows up as a constant offset on every body.

## The layout, and where each field is read

head, `FUN_141cb6880`'s encode order, `141cb7ec6` .. `141cb82f0`  -> 46 bytes
path, `FUN_141d57c60` -> `FUN_1404b2000`; read side `FUN_141d598b0` -> `FUN_1404b2630`
      -> 14 + 21*n, then a nibble list **only this packet carries** (`141c82000 XOR R8D,R8D`
      means `0x03D9` does not read it)
tail, `141cb83e7` .. `141cb8626`                                   -> 29 bytes

The Rust side is `crates/net/src/mobmove.rs::parse_mob_move`, and it carries two of these
bodies as fixtures.
"""
import os
import re
import struct
import sys

BODY_RE = re.compile(r"0x02FF\b.*?(\d+) byte body ([0-9a-fA-F]+)")


class R:
    def __init__(s, b):
        s.b, s.o = b, 0

    def take(s, n):
        if s.o + n > len(s.b):
            raise IndexError("body ran short at %d" % s.o)
        v = s.b[s.o:s.o + n]
        s.o += n
        return v

    def u8(s):
        return s.take(1)[0]

    def u16(s):
        return struct.unpack("<H", s.take(2))[0]

    def i16(s):
        return struct.unpack("<h", s.take(2))[0]

    def u32(s):
        return struct.unpack("<I", s.take(4))[0]


def decode(b):
    r = R(b)
    obj = r.u32()               # 141cb7ec6  mob+0x3a0
    move_id = r.u16()           # 141cb7f05  deobf(mob+0x2f0) + 1
    packed = r.u8()             # 141cb7f20  -> 0x03D9 offset 4
    action = r.u8()             # 141cb7f31  -> 0x03D9 offset 5; 0xFF = none
    r.take(8)                   # 141cb7f44
    r.take(2)                   # 141cb7f55, 141cb7f65
    n1 = r.u8()                 # 141cb7f80/91   mob+0x8c8
    r.take(n1 * 4)              # 141cb7ff2 + 141cb803a
    n2 = r.u8()                 # 141cb8063/71   mob+0x8d0
    r.take(n2 * 2)              # 141cb80ca
    gate = r.u32()              # 141cb80e9      mob+0x10b0
    if gate:
        r.take(11 * 4)          # 141cb810c .. 141cb81e4
    r.take(1 + 5 * 4 + 1)       # 141cb820e .. 141cb82f0
    head_end = r.o

    path_start = r.o
    r.take(4)                   # 1404b2650
    x = r.i16()                 # 1404b265b
    y = r.i16()                 # 1404b2675
    r.take(4)                   # 1404b2690, 1404b269c
    n = r.i16()                 # 1404b26a9, signed
    if n < 0:
        raise ValueError("negative element count %d" % n)
    r.take(n * 21)
    path_end = r.o
    nib = r.u8()                # 141d5991f, gated on FUN_141d598b0's arg 3
    r.take((nib + 1) // 2)      # 141d59936

    r.take(1 + 5 * 4 + 1 + 4 + 1 + 1 + 1)   # the tail, 141cb83e7 .. 141cb8626
    return dict(obj=obj, move_id=move_id, packed=packed, action=action, x=x, y=y,
                elems=n, head=head_end, path=(path_start, path_end), end=r.o,
                level=b[len(b) - 3])


def main():
    path = sys.argv[1] if len(sys.argv) > 1 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), "..", "world-ch0.log")
    text = open(path, encoding="utf-8", errors="replace").read()
    bodies = [(int(n), h) for n, h in BODY_RE.findall(text)]
    if not bodies:
        raise SystemExit("no 0x02FF bodies in %s - is this the right log?" % path)

    ok = 0
    for want, hexs in bodies:
        b = bytes.fromhex(hexs)
        if len(b) != want:
            print("BAD  transcription: log says %d bytes, hex is %d" % (want, len(b)))
            continue
        try:
            d = decode(b)
        except (IndexError, ValueError) as e:
            print("BAD  len=%-4d %s" % (len(b), e))
            continue
        good = d["end"] == len(b)
        ok += good
        print("%s len=%-4d obj=%-5d move=%-3d packed=%#04x action=%#04x pos=(%d,%d) "
              "elems=%d head=%d path=%d..%d level=%d"
              % ("OK " if good else "LEN", len(b), d["obj"], d["move_id"], d["packed"],
                 d["action"], d["x"], d["y"], d["elems"], d["head"], d["path"][0],
                 d["path"][1], d["level"]))

    print()
    print("%d/%d bodies decode to exactly their length" % (ok, len(bodies)))
    counts = sorted({len(bytes.fromhex(h)) for _, h in bodies})
    print("body sizes seen: %s  (the 21-byte path-element ladder)" % counts)
    if ok != len(bodies):
        raise SystemExit(1)


main()
