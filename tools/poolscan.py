#!/usr/bin/env python3
"""Scan ONLY the mob-pool handler subtrees (both jump tables) for a `[reg+DISP]` operand.

    python poolscan.py 0x2f4 2

Fast, and it answers the one question that matters: does ANY mob-pool packet handler touch
this field? The whole-image version is dispscan.py.

POSITIVE CONTROL: run it with 0x960. research/mob-behaviour.md section 4.2 read
`141d34ca0 MOV [rcx+0x960],..` inside FUN_141d34a70, the 0x3D2 handler, by hand. If 0x960
does not name that site, this tool is broken and its negatives are worthless.
"""
import os, sys, struct, bisect
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import reads as R
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
from collections import Counter

EXE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "client-patched", "MapleStory.exe")
R._load(EXE)
md = Cs(CS_ARCH_X86, CS_MODE_64)
md.detail = True

DISP = int(sys.argv[1], 0)
DEPTH = int(sys.argv[2], 0) if len(sys.argv) > 2 else 2


def table(addr, base, n, first_op):
    off = R._foff(addr)
    tg = [base + struct.unpack_from("<I", R._data, off + i * 4)[0] for i in range(n)]
    default = Counter(tg).most_common(1)[0][0]
    out = {}
    for i, t in enumerate(tg):
        if t == default:
            continue
        so = R._foff(t)
        for ins in md.disasm(R._data[so:so + 64], t):
            if ins.mnemonic in ("call", "jmp") and ins.op_str.startswith("0x"):
                out[first_op + i] = int(ins.op_str, 16)
                break
            if ins.mnemonic == "ret":
                break
    return out


handlers = {}
handlers.update(table(0x141D31184, 0x140000000, 0x13, 0x3C6))
handlers.update(table(0x141D33448, 0x140000000, 0x75, 0x3D9))
handlers[0x3C6] = 0x141D33630
byfn = {}
for op, h in handlers.items():
    byfn.setdefault(h, []).append(op)
byfn.setdefault(0x141D30E80, []).append(0x3C6)
byfn.setdefault(0x141D32B30, []).append(0x3D9)

deep = {}
for h, ops in list(byfn.items()):
    stack = [(h, 0)]
    seen = set()
    while stack:
        fn, d = stack.pop()
        if fn in seen or d > DEPTH or R.extent(fn) is None:
            continue
        seen.add(fn)
        deep.setdefault(fn, set()).update(ops)
        for _va, tgt, _g in R.calls_of(fn):
            stack.append((tgt, d + 1))

print("# %d opcodes, %d distinct handlers, %d functions in the depth-%d subtree"
      % (len(handlers), len(byfn), len(deep), DEPTH))

found = 0
for fn in sorted(deep):
    ext = R.extent(fn)
    if not ext:
        continue
    off = R._foff(ext[0])
    blob = R._data[off:off + (ext[1] - ext[0])]
    pos = 0
    while pos < len(blob):
        got = False
        for ins in md.disasm(blob[pos:], ext[0] + pos):
            got = True
            for o in ins.operands:
                if o.type == 3 and o.mem.disp == DISP and o.mem.base != 0 and o.mem.index == 0:
                    ops = sorted(deep[fn])
                    print("  %x  %-40s  FUN_%x   opcode(s) %s"
                          % (ins.address, "%s %s" % (ins.mnemonic, ins.op_str), fn,
                             " ".join("0x%03X" % o for o in ops[:8])))
                    found += 1
                    break
            pos = ins.address - ext[0] + ins.size
        if not got:
            pos += 1
print("== %d site(s) for disp %#x inside the mob-pool handler subtree ==" % (found, DISP))
