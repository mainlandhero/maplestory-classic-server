#!/usr/bin/env python3
"""Read the mob pool's SECOND jump table out of the image, and name every handler.

    141d32b68  LEA  EAX,[RSI - 0x3d9]
    141d32b6e  CMP  EAX,0x74            ; 0x75 = 117 entries
    141d32b81  MOV  EDX,[RAX + RCX*4 + 0x1d33448]
    141d32b8b  JMP  RDX

Each live stub is `MOV RDX,RDI / MOV RCX,RBX / CALL handler`, i.e. handler(mob, packet).
This prints opcode -> stub -> handler for all 117.
"""
import os
import sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import reads as R
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

EXE = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "client-patched", "MapleStory.exe")
TABLE = 0x141D33448
BASE = 0x140000000
N = 0x75
FIRST_OP = 0x3D9

R._load(EXE)
md = Cs(CS_ARCH_X86, CS_MODE_64)

off = R._foff(TABLE)
import struct
targets = []
for i in range(N):
    rel = struct.unpack_from("<I", R._data, off + i * 4)[0]
    targets.append(BASE + rel)

# The default stub is whichever target appears most often.
from collections import Counter
default = Counter(targets).most_common(1)[0][0]

rows = []
for i, t in enumerate(targets):
    op = FIRST_OP + i
    if t == default:
        rows.append((op, t, None))
        continue
    # decode the stub: expect a CALL within ~8 instructions
    so = R._foff(t)
    handler = None
    for ins in md.disasm(R._data[so:so + 64], t):
        if ins.mnemonic in ("call", "jmp") and ins.op_str.startswith("0x"):
            handler = int(ins.op_str, 16)
            break
        if ins.mnemonic == "ret":
            break
    rows.append((op, t, handler))

print("# default stub = %#x  (%d entries)" % (default, sum(1 for r in rows if r[2] is None)))
for op, t, h in rows:
    if h is None:
        print("0x%03X  ---- default" % op)
    else:
        ext = R.extent(h)
        size = (ext[1] - ext[0]) if ext else 0
        print("0x%03X  stub %#x  handler FUN_%x  (%d bytes)" % (op, t, h, size))
