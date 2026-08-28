#!/usr/bin/env python3
"""Which outbound packet builders write a field *inside a loop*?

    python tools/loop_builders.py                    # every builder with a looping encode
    python tools/loop_builders.py --shape u8,u32     # only loops whose body is that
    python tools/loop_builders.py --control          # prove the instrument can see a loop

## Why this exists

`research/msexe-packet-fields.txt` lists the encode calls a builder makes **in address
order**, which flattens a loop: a packet that writes `count` entries of `{u32, u8, u32}`
looks identical to one that writes three fixed fields. Every question of the form "does the
client send a *list* of something" is invisible in that file, and asking it by reading 1793
builders by hand is not a plan.

A loop is visible in the disassembly and nowhere else. This walks each builder once and
reports the encode sites that sit inside a backward branch - i.e. the fields written once
per element rather than once per packet - together with the encode sites outside it, which
are the header.

## The positive control, and why it is not optional

`--control` walks `FUN_1409f6eb0`, the `0x00D9` user-move builder. Movement is the one packet
this repo has decoded end to end (`crates/net/src/usermove.rs`, `research/user-move.md`): it
carries `u8 count` and then that many path elements, so **its encode calls must come back
marked LOOP**. If they do not, this tool cannot see a loop at all and every empty result it
has ever printed was meaningless. It prints:

```text
control 0x1409f6eb0 (0x00D9 user move): header [CTOR,u8,u32,u32,u8,SEND,u8]  LOOP [u8]
```

**That control failed on the first run of this tool, and the failure is the reason `--depth`
defaults to 2.** `FUN_1409f6eb0` writes the header itself and hands the path to
`FUN_141d57c60`; at depth 1 the loop lives entirely in the helper and the one packet known
to carry a list came back flat. A tool that had shipped with depth 1 and no control would
have swept 1793 builders, found nothing, and been believed.

`CLAUDE.md` records five separate instruments here that returned clean, confident, wrong
answers - three of them empty ones. The rule that came out of it is that a search proves it
can find a positive before its silence is worth anything, so `--control` is wired to exit
non-zero when the control does not light up, and the sweep runs it first and refuses to
print a sweep whose control failed.

## What it cannot see

* **An unrolled loop.** A builder that writes eight fixed entries has no backward branch and
  is reported as eight header fields. That is the right answer for the disassembly and the
  wrong answer for the question, so a negative here is "no *looping* builder", never "no
  list-shaped packet".
* **A loop more than one call deep.** `--depth 2` (the default) follows a builder's direct
  callees and no further, so a builder that delegates twice is still missed. Raising it
  costs precision: a deep helper's loop gets attributed to whatever packet reached it.
* Anything Themida virtualised. `0x032C`'s builder is inside `.themida` and has no bytes.
"""
import argparse
import collections
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import reads as R
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

from listing import COND, _target

# Copied from tools/encodes.py rather than imported: that module calls main() at import
# time with no `if __name__` guard, so `from encodes import ENC` runs its CLI and dies on
# sys.argv[1]. Keep the two tables in step - if a primitive is ever added there, add it
# here, because a builder whose only looping write goes through a missing primitive comes
# back "no loop" and looks like a finding.
ENC = {
    0x1406ED520: "CTOR",
    0x1406ED840: "u8",
    0x1406ED940: "u16",
    0x1406ED9D0: "u32",
    0x1406EDBC0: "u64",
    0x1406EDC80: "str",
    0x1406EDE20: "raw",
    0x1406ED610: "SEND",
}

DEFAULT_EXE = "client-patched/MapleStory.exe"
FIELDS = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..",
                      "research", "msexe-packet-fields.txt")

# The 0x00D9 user-move builder. Decoded in research/user-move.md and implemented in
# crates/net/src/usermove.rs, so "it writes a variable-length path" is established
# independently of this tool.
CONTROL = 0x1409F6EB0


_one_memo = {}


def _one(fn, exe):
    """(header, looped, callees) for a single function, or None if it has no .pdata entry.

    A function that has no `.pdata` entry is not the same as one with no encodes - it is a
    function whose bytes are not in the file - so this returns None rather than empty lists
    and every caller has to say which it means.

    Memoised because it is not: the depth-2 sweep walks the same few hundred helpers from
    almost every one of the 1793 builders, and without this the full run does not finish
    inside fifteen minutes.
    """
    if fn in _one_memo:
        return _one_memo[fn]
    r = _one_uncached(fn, exe)
    _one_memo[fn] = r
    return r


def _one_uncached(fn, exe):
    R._load(exe)
    ext = R.extent(fn)
    if ext is None:
        return None
    start, end, _ = ext
    off = R._foff(start)
    if off is None:
        return None
    code = R._data[off:off + (end - start)]
    md = Cs(CS_ARCH_X86, CS_MODE_64)

    sites, callees = [], []
    back = []           # (target, source) of every backward branch inside the function
    for ins in md.disasm(code, start):
        if ins.mnemonic in ("call", "jmp"):
            t = _target(ins)
            if t in ENC:
                sites.append((ins.address, ENC[t]))
            elif t is not None and not (start <= t < end):
                callees.append((ins.address, t))
        if ins.mnemonic in COND or ins.mnemonic == "jmp":
            t = _target(ins)
            if t is not None and start <= t <= ins.address:
                back.append((t, ins.address))

    def in_loop(a):
        return any(lo <= a <= hi for lo, hi in back)

    header = [(a, k) for a, k in sites if not in_loop(a)]
    looped = [(a, k) for a, k in sites if in_loop(a)]
    return header, looped, [(a, t, in_loop(a)) for a, t in callees]


def encodes_in_loops(fn, exe=DEFAULT_EXE, depth=2, _stack=frozenset()):
    """(header, looped) - encode kinds written once per packet, and once per element.

    **Depth matters and the default is 2 for a measured reason.** The `0x00D9` move builder
    `FUN_1409f6eb0` writes its header itself and hands the variable-length path to
    `FUN_141d57c60`, so at depth 1 the one packet in this repo that is known to carry a list
    comes back with an empty loop. A builder is only as flat as its inlining, and this
    client inlines inconsistently.

    A callee's encodes are treated as looping if the callee loops **or** if the call site
    itself is inside a loop in the caller - that is what makes a per-element helper show up.

    Returns None when the function has no `.pdata` entry.
    """
    r = _one(fn, exe)
    if r is None:
        return None
    # Copy: `_one` is memoised, and the extends below would otherwise grow the cached
    # lists, so the second builder to reach a shared helper would inherit the first
    # builder's fields. That is a silent wrong answer, not a crash.
    header, looped, callees = list(r[0]), list(r[1]), r[2]
    if depth > 1:
        for site, tgt, site_in_loop in callees:
            if tgt in _stack or tgt in ENC:
                continue
            sub = encodes_in_loops(tgt, exe, depth - 1, _stack | {fn})
            if sub is None:
                continue
            sub_header, sub_looped = sub
            looped.extend(sub_looped)
            # The helper writes fixed fields, but the caller calls it once per element.
            (looped if site_in_loop else header).extend(sub_header)
    header.sort()
    looped.sort()
    return header, looped


def shape(entries):
    """The encode kinds in address order, as a comma string - 'u32,u8,u32'."""
    return ",".join(k for _, k in entries)


def control(exe):
    r = encodes_in_loops(CONTROL, exe)
    if r is None:
        print(f"CONTROL FAILED: {CONTROL:#x} has no .pdata entry", file=sys.stderr)
        return False
    header, looped = r
    ok = bool(looped)
    print(f"control {CONTROL:#x} (0x00D9 user move): "
          f"header [{shape(header)}]  LOOP [{shape(looped)}]")
    if not ok:
        print("CONTROL FAILED: the move builder's path loop was not seen. This tool cannot "
              "see a loop; ignore every result it prints.", file=sys.stderr)
    return ok


def builders():
    """(opcode, function VA) from the field dump, deduplicated, in file order."""
    out, seen = [], set()
    for line in open(FIELDS):
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split()
        if len(parts) < 2 or not parts[0].startswith("0x") or not parts[1].startswith("FUN_"):
            continue
        va = int(parts[1][4:], 16)
        key = (parts[0], va)
        if key in seen:
            continue
        seen.add(key)
        out.append((parts[0], va))
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--exe", default=DEFAULT_EXE)
    ap.add_argument("--shape", help="only report loops whose body is exactly this, e.g. u8,u32")
    ap.add_argument("--depth", type=int, default=2)
    ap.add_argument("--control", action="store_true", help="run the control and stop")
    args = ap.parse_args()

    ok = control(args.exe)
    if args.control:
        return 0 if ok else 1
    if not ok:
        return 1
    print()

    rows, no_pdata = [], 0
    for op, va in builders():
        r = encodes_in_loops(va, args.exe, args.depth)
        if r is None:
            no_pdata += 1
            continue
        header, looped = r
        if not looped:
            continue
        body = shape(looped)
        if args.shape and body != args.shape:
            continue
        rows.append((op, va, shape(header), body))

    print(f"{len(rows)} builder(s) with a looping encode"
          + (f", shape {args.shape!r}" if args.shape else "")
          + f"   ({no_pdata} had no .pdata entry and were skipped)\n")
    for op, va, head, body in sorted(rows):
        print(f"  {op}  FUN_{va:x}   header [{head}]   LOOP [{body}]")
    return 0


if __name__ == "__main__":
    sys.exit(main())
