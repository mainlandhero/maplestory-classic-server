#!/usr/bin/env python3
"""Every packet field a function reads, in order, counting reads made through helpers.

    python tools/reads.py 0x142784970          # ordered read list, depth 3
    python tools/reads.py 0x140304100 2        # the equipped-item decoder, the control

## Why this exists rather than a grep for `E8`

Three separate mistakes, all of which have shipped a wrong packet on this project:

**1. Direct-only counting.** A function that never calls a read primitive itself can still
read dozens of bytes through a helper. `research/mob-spawn.md` records `FUN_141cc9410` as
"reads NOTHING" when it in fact pulls 57 bytes through `FUN_14085acd0`. This walks call
targets transitively.

**2. Scanning bytes instead of instructions.** A hand-rolled scan for `0xE8` cannot know
where instructions start, and `0xE8` is a perfectly ordinary ModRM byte. In `FUN_142784970`
the sequence `44 0f b6 e8` (`MOVZX R13D,AL`) contains one, and a scanner that treats it as a
CALL and skips five bytes lands mid-instruction and **silently loses the next real read**.
That is how a `0x0231` body went out three fields short and killed the client on
2026-08-19. This disassembles.

**3. An incomplete primitive list, and a call-only walk.** See the note on `PRIM` below:
there are ten, not eight, and a function whose last instruction is `jmp <primitive>` reads a
field that a call-only walker never sees.

All three failures are silent and all three produce a confident, clean, wrong answer - the
exact shape `CLAUDE.md` says to distrust. So this file carries its own positive control:

    python tools/reads.py 0x140304100 2

must show the equipped-item decoder reading through `FUN_1403035a0` and `FUN_140303b40`
**and** directly - a mix of both kinds. If it does not, this tool is broken, not the target.

## Reading the output

`(direct)` is a call (or a tail `jmp`) straight to one of the TEN primitives. `-> READS via helper` is a
call to something that reaches one within the depth limit; the field list after it is what
that subtree touches, NOT necessarily in order and NOT necessarily unconditional.

`gated?` marks a call that sits after a conditional jump inside the function, so it may not
run for every body. **This is a hint, not a dominator analysis** - it flags anything with a
conditional branch anywhere before it, which over-reports. Confirm by reading the listing.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rtti import load_pe            # noqa: E402
from dataref import parse_pdata     # noqa: E402

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
except ImportError:  # pragma: no cover
    raise SystemExit("capstone is required: pip install capstone")

# **TEN, and the count has now been wrong three times.** Six real readers and four bare
# `jmp` thunks. The history is the warning: five (wrong set), then seven, then eight when a
# u16 thunk turned up, then nine when 0x1406e8ee0 turned up - and then TEN, because that
# ninth was found by looking near the known cluster instead of enumerating.
#
# The enumeration that settles it: every `jmp rel32` in the whole of `.text` whose target is
# one of the six real readers, filtered to those preceded by `0xCC` padding (a thunk) rather
# than by a `pop` or `leave` (an ordinary tail jump inside a function). 65 hits, 4 thunks.
# `0x142d23ef0` is 6 MB away from the others, which is exactly why searching a neighbourhood
# missed it. CLAUDE.md: enumerate before you filter.
PRIM = {
    # the six real readers
    0x1406e8ae0: "u8", 0x1406e8b80: "u16", 0x1406e8c20: "u32",
    0x1406e8f10: "u64", 0x1406e9050: "str", 0x1406e9170: "raw",
    # the four thunks
    0x1406e8ee0: "u8", 0x1406e8ef0: "u16", 0x1406e8f00: "u32", 0x142d23ef0: "u32",
}

_md = Cs(CS_ARCH_X86, CS_MODE_64)
_data = _base = _sections = None
_ent = {}


def _load(exe):
    global _data, _base, _sections, _ent
    _data, _base, _sections = load_pe(exe)
    _ent = {f[0]: f for f in parse_pdata(_data, _sections, _base)}


def _foff(va):
    rva = va - _base
    for s in _sections:
        if s["vaddr"] <= rva < s["vaddr"] + max(s["vsize"], s["rsize"]):
            return s["raddr"] + (rva - s["vaddr"])
    return None


def calls_of(fn):
    """[(va, target, seen_conditional_before_it)] for every CALL to a known address.

    Linear sweep with a real decoder, so instruction boundaries are right.
    """
    if fn not in _ent:
        return []
    start, end = _ent[fn][0], _ent[fn][1]
    off = _foff(start)
    if off is None:
        return []
    code = _data[off:off + (end - start)]
    # `start`/`end` are read by the tail-call test below.
    out, conditional = [], False
    for ins in _md.disasm(code, start):
        m = ins.mnemonic
        if m.startswith("j") and m != "jmp":
            conditional = True
        elif m in ("call", "jmp") and ins.op_str.startswith("0x"):
            tgt = int(ins.op_str, 16)
            # A `jmp` counts ONLY when it leaves the function - a tail call. Within the
            # function it is ordinary control flow. Missing tail calls is a real gap: the
            # enumeration above found 61 sites whose last act is `jmp <read primitive>`, and
            # a call-only walker reports every one of those functions as reading nothing.
            if m == "jmp" and start <= tgt < end:
                continue
            if tgt in _ent or tgt in PRIM:
                out.append((ins.address, tgt, conditional))
    return out


_memo = {}


def reaches(fn, depth, stack=frozenset()):
    if fn in PRIM:
        return [PRIM[fn]]
    if depth <= 0 or fn in stack:
        return []
    key = (fn, depth)
    if key in _memo:
        return _memo[key]
    found = []
    for _, tgt, _c in calls_of(fn):
        found.extend(reaches(tgt, depth - 1, stack | {fn}))
    _memo[key] = found
    return found


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 1
    fn = int(sys.argv[1], 16)
    depth = int(sys.argv[2]) if len(sys.argv) > 2 else 3
    exe = sys.argv[3] if len(sys.argv) > 3 else "client-patched/MapleStory.exe"
    _load(exe)
    if fn not in _ent:
        print("%#x has no .pdata entry - not a function start?" % fn)
        return 1
    start, end = _ent[fn][0], _ent[fn][1]
    print("FUN_%x  %#x..%#x (%d bytes)  depth %d" % (fn, start, end, end - start, depth))
    n = 0
    for va, tgt, cond in calls_of(fn):
        mark = "  gated?" if cond else ""
        if tgt in PRIM:
            print("  %#x  READ %-3s (direct)%s" % (va, PRIM[tgt], mark))
            n += 1
        else:
            names = reaches(tgt, depth)
            if names:
                uniq = []
                for x in names:
                    if x not in uniq:
                        uniq.append(x)
                print("  %#x  call %#x -> READS via helper: %s%s"
                      % (va, tgt, " ".join(uniq), mark))
                n += 1
    if n == 0:
        print("  (no reads found - if you expected some, run the positive control first)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
