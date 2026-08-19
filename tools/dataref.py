#!/usr/bin/env python3
"""Find RIP-relative references to a global, including the ones `xref.py` cannot see.

`tools/xref.py` matches `lea reg,[rip+disp32]` and `mov reg,imm64` - i.e. code that takes
a global's *address*. Code that *reads or writes the global itself* uses the same
RIP-relative form with a different opcode:

    mov  rax, [rip+disp32]     48 8B 05 ..    a read
    mov  [rip+disp32], rax     48 89 05 ..    a write
    cmp  qword [rip+disp32], 0 48 83 3D .. 00 a test

xref.py's own docstring records that this blind spot cost several sessions once already,
on `FUN_141b2a280`. This closes it for globals.

    python tools/dataref.py 0x143aa84a0
    python tools/dataref.py 0x143aa84a0 --writes    # only stores

Any byte offset is a candidate start, so this can hallucinate an instruction out of data.
Every hit is attributed to its containing `.pdata` function and hits outside one are
flagged; a lone unattributed hit is a candidate, not a finding.
"""
import argparse
import bisect
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rtti import load_pe  # noqa: E402

# (opcode byte, how many bytes follow the disp32 before the instruction ends, label)
# Only forms whose disp32 sits immediately after the modrm byte.
OPS = {
    0x8B: (0, "read"),
    0x89: (0, "write"),
    0x8D: (0, "lea"),
    0x3B: (0, "cmp"),
    0x39: (0, "cmp"),
    0x85: (0, "test"),
    0x01: (0, "add"),
    0x03: (0, "add"),
    0x2B: (0, "sub"),
    0x33: (0, "xor"),
    0x83: (1, "grp1-imm8"),      # cmp/add/... qword [rip+d], imm8
    0x81: (4, "grp1-imm32"),
    0xC7: (4, "mov-imm32"),
    0xFF: (0, "grp5"),           # call/jmp/inc qword [rip+d]
}
REX = {0x48, 0x49, 0x4C, 0x4D, 0x40, 0x41, 0x44, 0x45}


def parse_pdata(data, sections, base):
    for s in sections:
        if s["name"] == ".pdata":
            blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
            out = []
            for i in range(0, len(blob) - 11, 12):
                b, e, _ = struct.unpack_from("<III", blob, i)
                if b and e > b:
                    out.append((base + b, base + e))
            out.sort()
            return out
    return []


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("target", help="global VA, e.g. 0x143aa84a0")
    ap.add_argument("--exe", default="client-patched/MapleStory.exe")
    ap.add_argument("--writes", action="store_true", help="stores only")
    args = ap.parse_args()
    target = int(args.target, 16)

    data, base, sections = load_pe(args.exe)
    funcs = parse_pdata(data, sections, base)
    starts = [f[0] for f in funcs]

    def containing(va):
        i = bisect.bisect_right(starts, va) - 1
        return funcs[i][0] if i >= 0 and funcs[i][0] <= va < funcs[i][1] else None

    hits = []
    for s in sections:
        if not (s["chars"] & 0x20000000) or not s["rsize"]:
            continue
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        secbase = base + s["vaddr"]
        for i in range(len(blob) - 10):
            b0 = blob[i]
            if b0 in REX:
                op, modrm_at = blob[i + 1], i + 2
                head = 2
            else:
                op, modrm_at = b0, i + 1
                head = 1
            if op not in OPS:
                continue
            modrm = blob[modrm_at]
            if modrm & 0xC7 != 0x05:          # mod=00, rm=101 -> RIP-relative
                continue
            disp = struct.unpack_from("<i", blob, modrm_at + 1)[0]
            tail, label = OPS[op]
            end = secbase + i + head + 1 + 4 + tail
            if end + disp == target:
                hits.append((secbase + i, label, containing(secbase + i), head == 2))

    # A REX-prefixed instruction at V also decodes as an un-prefixed one at V+1, so every
    # real hit is reported twice. Keep the REX reading and drop the shadow.
    rex_at = {va for va, _, _, rex in hits if rex}
    hits = [(va, label, fn) for va, label, fn, rex in hits if rex or va - 1 not in rex_at]

    if args.writes:
        hits = [h for h in hits if h[1] == "write"]
    print(f"{target:#x}: {len(hits)} RIP-relative reference(s)")
    for va, label, fn in hits:
        home = f"{fn:#x}" if fn else "(outside any .pdata function)"
        print(f"    {label:10} at {va:#x}  in {home}")


if __name__ == "__main__":
    sys.exit(main())
