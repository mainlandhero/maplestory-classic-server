#!/usr/bin/env python3
"""Every instruction with a `[reg + DISP]` memory operand - i.e. every use of a struct field.

    python tools/fieldrefs.py 0x960                                   # whole image
    python tools/fieldrefs.py 0xb0 --lo 0x141c40000 --hi 0x141d60000  # one class's code
    python tools/fieldrefs.py 0x2f4 --lo 0x141c40000 --hi 0x141d60000 --write

## Why this exists

`xref.py --va` matches code that takes an address, `dataref.py` matches a RIP-relative
global, `callers.py` matches a direct CALL. **None of them matches a struct field**, and
`research/mob-spawn.md` section 11.4 had to hand-roll this scan to find what writes
`mob+0x2b8`. It found the answer - two writers in the whole image - and then the same
question came back three more times for `mob+0x960`, `mob+0x2f4` and `mob+0xcd0`.

## The two ways this kind of scan lies, and what is done about them

**A linear capstone sweep stops silently at the first undecodable byte.** `.text` needs
about 20 000 resync points and `.boot` about 646 000; without resyncing, everything after
the first bad byte is never examined and the scan reports a confident zero. This resyncs one
byte at a time, and prints the resync count to stderr so a suspicious zero is visible.

**A displacement is a class fact, not an offset fact.** `[rcx+0x2b8]` on a `CMob` and
`[rcx+0x2b8]` on the COM interface at `mob+8` are *different fields*, and reading one as the
other is exactly the mistake `research/mob-spawn.md` section 11.7 retracts. This tool cannot
tell them apart. **Establish which subobject `this` is before believing any row.**

## Positive control

    python tools/fieldrefs.py 0x2f4 --lo 0x141c40000 --hi 0x141d60000 --write

must print exactly two rows inside `0x141c4cee0` (the mob base constructor, at `141c4d261`
and `141c4e6ee`) and one at `141cb7ef3` inside `0x141cb6880`. All three were read by hand
first. If a run of this tool cannot reproduce them, the tool is broken, not the target.

`rsp`/`rbp`/`rip`-based operands are dropped: those are stack frames and globals, not fields.
"""
import argparse
import bisect
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rtti import load_pe          # noqa: E402
from dataref import parse_pdata   # noqa: E402

try:
    import capstone
except ImportError:  # pragma: no cover - the message is the point
    sys.exit("capstone is not installed; `pip install capstone`")

SKIP_BASES = (
    capstone.x86.X86_REG_RSP,
    capstone.x86.X86_REG_RBP,
    capstone.x86.X86_REG_RIP,
    capstone.x86.X86_REG_ESP,
    capstone.x86.X86_REG_EBP,
    0,
)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("disp", help="the field offset, e.g. 0x2b8")
    ap.add_argument("--exe", default="client-patched/MapleStory.exe")
    ap.add_argument("--lo", help="only report hits at or above this VA")
    ap.add_argument("--hi", help="only report hits below this VA")
    ap.add_argument("--sections", default=".text,.boot")
    ap.add_argument("--write", action="store_true",
                    help="only instructions whose FIRST operand is the memory operand, "
                         "i.e. stores - the question 'who sets this field'")
    args = ap.parse_args()

    disp = int(args.disp, 16)
    lo = int(args.lo, 16) if args.lo else 0
    hi = int(args.hi, 16) if args.hi else (1 << 63)
    want = set(args.sections.split(","))

    data, base, sections = load_pe(args.exe)
    funcs = parse_pdata(data, sections, base)
    starts = [f[0] for f in funcs]

    def owner(va):
        i = bisect.bisect_right(starts, va) - 1
        if i >= 0 and funcs[i][0] <= va < funcs[i][1]:
            return funcs[i][0]
        return None

    md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
    md.detail = True
    hits = resyncs = 0
    for s in sections:
        if s["name"] not in want:
            continue
        secva = base + s["vaddr"]
        start, end = s["raddr"], s["raddr"] + s["rsize"]
        # Narrow the DISASSEMBLY to the requested VA window, not just the reporting. A whole
        # -image sweep is ~18.5M instructions and minutes of wall clock; the same scan bounded
        # to one class's code is seconds, which is the difference between a tool that gets used
        # and one that gets hand-rolled again.
        if lo > secva:
            start = min(end, s["raddr"] + (lo - secva))
            secva = lo
        if hi < base + s["vaddr"] + s["rsize"]:
            end = max(start, min(end, s["raddr"] + (hi - (base + s["vaddr"]))))
        blob = data[start:end]
        mv = memoryview(blob)          # slicing `bytes` to resync is O(n^2) and never finishes
        n = len(blob)
        cur = 0
        while cur < n:
            got = False
            for ins in md.disasm(mv[cur:], secva + cur):
                cur = ins.address - secva + ins.size
                got = True
                if not (lo <= ins.address < hi):
                    continue
                ops = ins.operands
                for i, op in enumerate(ops):
                    if op.type != capstone.x86.X86_OP_MEM or op.mem.disp != disp:
                        continue
                    if op.mem.base in SKIP_BASES:
                        continue
                    # "Writes the field" = the memory operand is the destination. Capstone
                    # puts the destination first, but `cmp`/`test`/`push`/`bt` also lead with
                    # a memory operand they only READ - and letting those through is how a
                    # `--write` list grows six rows that are not writers at all.
                    if args.write and (i != 0 or ins.mnemonic in READ_ONLY_DEST):
                        continue
                    o = owner(ins.address)
                    print("%09x  %-9s %-44s in %s" % (
                        ins.address, ins.mnemonic, ins.op_str,
                        ("%#x" % o) if o else "-  (no .pdata entry)"))
                    hits += 1
                    break
            if not got:
                resyncs += 1
                cur += 1

    sys.stderr.write("%d hit(s), %d resync point(s)\n" % (hits, resyncs))
    return 0


if __name__ == "__main__":
    sys.exit(main())
