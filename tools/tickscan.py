#!/usr/bin/env python3
"""Enumerate the client's timed tasks and classify what each one does when it fires.

    python tools/tickscan.py 180000          # the 180 s clock
    python tools/tickscan.py 60000 --all     # a control clock

Every timed task in this client goes through `FUN_1408fcaa0(last, interval, now)`
(`sub r8d,ecx; cmp r8d,edx; seta al; ret` - measured), so "which tasks run on interval N"
is a bounded, enumerable question rather than a text search. This walks each `.pdata`
function that loads the interval as an immediate and reports:

* `alloc` - it calls the ZArray ctor `0x140ca61d0`
* `oob w[K]` - it takes `&v[0]` from `0x140ca22a0` and then writes at `&v[0] + K` bytes,
  i.e. element `K/4` of an array whose length the same function just set
* `vm` - control leaves `.text` by a `jmp` into a Themida section, so the body is
  virtualised and **nothing below the jump is readable**. This is the state a caller-scan
  reports as "zero callers"; it is not absence
* `report` - it calls the telemetry sender `0x140ca3130`

## Enumerate before you filter

The point of the classification is that a scan for *the known shape* answers only about
the shape. `FUN_140c93c80` fires on the same clock, allocates the same array and was the
one **actually observed running** - and it has none of the shape, because its body is a
`jmp` into `.themida` at `0x140c93cd2`. A search for the load/modify/store pattern misses
it and returns a clean, confident five.

## Verify it before you believe it

    python tools/tickscan.py 180000

must list `FUN_140c93530` with `oob w[0x90]` and `FUN_140c93c80` with `vm`. If the second
is missing the Themida detection is broken and every negative below it is worthless.

## Blind spots

* `.themida`/`.boot` are **zero bytes on disk**. A task whose whole body is virtualised has
  nothing here to find, so this can only ever undercount.
* The interval must appear as an immediate. One computed at runtime is invisible.
"""
import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from rtti import load_pe  # noqa: E402
from dataref import parse_pdata  # noqa: E402
from dis_at import va_to_off  # noqa: E402

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64  # noqa: E402
except ImportError:
    sys.exit("capstone not installed")

DEFAULT_EXE = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                           "client-patched", "MapleStory.exe")
ELAPSED = 0x1408FCAA0
ZARRAY_CTOR = 0x140CA61D0
ZARRAY_AT = 0x140CA22A0
REPORT = 0x140CA3130
RIP = re.compile(r"rip ([+-]) (0x[0-9a-f]+)")


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    interval = int(sys.argv[1], 0)
    data, ib, secs = load_pe(DEFAULT_EXE)
    vm_ranges = [(ib + s["vaddr"], ib + s["vaddr"] + max(s["vsize"], s["rsize"]))
                 for s in secs if s["name"].lower() in (".themida", ".boot", ".vm_sec")]
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    fns = parse_pdata(data, secs, ib)
    rows = []
    for start, end in fns:
        off, sect = va_to_off(start, ib, secs)
        if off is None or sect != ".text":
            continue
        blob = data[off:off + (end - start)]
        if bytes([0xBA]) + interval.to_bytes(4, "little") not in blob:
            continue          # mov edx, imm32
        ins_list = list(md.disasm(blob, start))
        has_elapsed = False
        tags = []
        at_pending = False
        for ins in ins_list:
            if ins.mnemonic in ("call", "jmp") and ins.op_str.startswith("0x"):
                t = int(ins.op_str, 16)
                if t == ELAPSED:
                    has_elapsed = True
                elif t == ZARRAY_CTOR:
                    tags.append("alloc")
                elif t == ZARRAY_AT:
                    at_pending = True
                elif t == REPORT:
                    tags.append("report")
                elif ins.mnemonic == "jmp" and any(lo <= t < hi for lo, hi in vm_ranges):
                    tags.append("vm@0x%x" % ins.address)
                    break
            elif at_pending and ins.mnemonic == "add" and ins.op_str.startswith("rax, 0x"):
                tags.append("oob w[%s]" % ins.op_str.split(", ")[1])
                at_pending = False
        if has_elapsed:
            rows.append((start, end - start, tags))
    print("; interval %d ms (0x%x): %d .text functions" % (interval, interval, len(rows)))
    for start, size, tags in rows:
        print("  FUN_%x  %4d bytes  %s" % (start, size, ", ".join(tags) or "-"))


if __name__ == "__main__":
    main()
