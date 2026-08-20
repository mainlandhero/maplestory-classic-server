#!/usr/bin/env python3
"""Print a function's listing, with the packet reads, branches and marks called out.

    python tools/listing.py 0x142784970
    python tools/listing.py 0x142784970 0x1427856fa 0x142785927   # mark two addresses

## Why this exists

The repo could disassemble in three specialised ways and not one general one. `reads.py`
counts packet reads, `rangescan.py` finds `[reg + DISP]` operands, `fieldrefs.py` finds
struct-field uses - all of them answer a question you already know how to ask. When the
question is "the client accepted this packet and drew nothing, so where does it give up",
what you need is the listing itself, with every branch target labelled so the basic blocks
are visible.

It shares `reads.py`'s image loader, its merged `.pdata` extent and its table of the ten
read primitives, so it **cannot disagree with the read counter** about where a function ends
or about what counts as a read. That matters: `CLAUDE.md` records two separate crashes that
came from a walker seeing nine reads where there were ten.

## Verify it before you believe it

    python tools/listing.py 0x140304100 | grep READ

must print reads at `140304138 raw`, `140304144 u8`, `140304183 u8`, then a run of `u16` -
the same direct reads, at the same addresses, that `python tools/reads.py 0x140304100 2`
lists. If it does not, the loader or the extent is wrong and nothing below it is evidence.

## What the columns mean

* A leading `>` marks an instruction that some branch inside this function targets, so the
  basic-block boundaries can be read off without a graph.
* `<<< READ <type>` marks a call **or a tail `jmp`** into one of the ten primitives. A tail
  jump is a read; missing one is how a packet went out four bytes short.
* `[branch]` marks a conditional jump.
* `<<< MARKED` marks an address given on the command line, either as an instruction or as a
  call target - so "which branch skips this call" is a `grep` rather than a hunt.

## What it does not do

No dominator analysis and no data flow. It shows you the branches; deciding which of them a
real run takes is still a watch on the client.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import reads as R  # noqa: E402

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64  # noqa: E402
except ImportError:  # pragma: no cover
    raise SystemExit("capstone is required: pip install capstone")

DEFAULT_EXE = "client-patched/MapleStory.exe"

COND = {
    "jo", "jno", "jb", "jae", "je", "jne", "jbe", "ja", "js", "jns", "jp", "jnp",
    "jl", "jge", "jle", "jg", "jcxz", "jecxz", "jrcxz",
}


def _target(ins):
    """The absolute target of a direct jump or call, or None for an indirect one."""
    try:
        return int(ins.op_str, 0)
    except ValueError:
        return None


def listing(fn, marks, exe=DEFAULT_EXE, out=sys.stdout):
    R._load(exe)
    ext = R.extent(fn)
    if ext is None:
        print("%#x has no .pdata entry - not a function start?" % fn, file=out)
        return 1
    start, end, merged = ext
    note = ("  [%d contiguous .pdata entries merged]" % (merged + 1)) if merged else ""
    print("; %#x .. %#x  (%d bytes)%s" % (start, end, end - start, note), file=out)

    code = R._data[R._foff(start):R._foff(start) + (end - start)]
    md = Cs(CS_ARCH_X86, CS_MODE_64)

    # First pass: every branch target inside the function, so they can be labelled. Done
    # separately because a target is almost always ahead of the branch that names it.
    targets = set()
    for ins in md.disasm(code, start):
        if ins.mnemonic in COND or ins.mnemonic == "jmp":
            t = _target(ins)
            if t is not None and start <= t < end:
                targets.add(t)

    for ins in md.disasm(code, start):
        label = ">" if ins.address in targets else " "
        note = ""
        if ins.mnemonic in ("call", "jmp"):
            t = _target(ins)
            if t in R.PRIM:
                note = "   <<< READ %s%s" % (
                    R.PRIM[t], " (TAIL JMP)" if ins.mnemonic == "jmp" else ""
                )
            elif t is not None and t in marks:
                note = "   <<< MARKED %#x" % t
        if ins.address in marks:
            note += "   <<< MARKED ADDRESS"
        if ins.mnemonic in COND:
            note += "   [branch]"
        print("%s%012x  %-8s %s%s" % (label, ins.address, ins.mnemonic, ins.op_str, note),
              file=out)
    return 0


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 1
    fn = int(sys.argv[1], 0)
    marks = {int(a, 0) for a in sys.argv[2:]}
    return listing(fn, marks)


if __name__ == "__main__":
    sys.exit(main())
