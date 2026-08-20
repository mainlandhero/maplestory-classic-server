#!/usr/bin/env python3
"""Scan a VA range for `[reg + DISP]` operands, naming the containing .pdata function.

    python rangescan.py 0x2e4 0x141c40000 0x141d60000

Disassembles (never byte-greps) and resyncs one byte at a time on undecodable input.
research/mob-behaviour.md section 1 documents this instrument's positive controls:
`+0x2f4` must return `141cb7ef3 MOV [R12+0x2f4],EAX` and `+0x960` the constructor's
`MOV byte [RSI+0x960],0` - both read by hand first.
"""
import os, sys, bisect
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import reads as R
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

R._load(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "client-patched", "MapleStory.exe"))
md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True

DISP = int(sys.argv[1], 0)
LO = int(sys.argv[2], 0)
HI = int(sys.argv[3], 0)


def owner(va):
    i = bisect.bisect_right(R._starts, va) - 1
    while i >= 0:
        st = R._starts[i]
        if R._ent[st][1] > va:
            return st
        i -= 1
    return None


off = R._foff(LO)
blob = R._data[off:off + (HI - LO)]
pos = 0
n = 0
while pos < len(blob):
    got = False
    for ins in md.disasm(blob[pos:], LO + pos):
        got = True
        for o in ins.operands:
            if o.type == 3 and o.mem.disp == DISP and o.mem.base != 0 and o.mem.index == 0:
                fn = owner(ins.address)
                print("  %x  %-44s  FUN_%s" % (ins.address, "%s %s" % (ins.mnemonic, ins.op_str),
                                               ("%x" % fn) if fn else "?"))
                n += 1
                break
        pos = ins.address - LO + ins.size
    if not got:
        pos += 1
print("== %d site(s) for disp %#x in %#x..%#x ==" % (n, DISP, LO, HI))
