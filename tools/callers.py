#!/usr/bin/env python3
"""Every way control can reach a function: `call`, tail `jmp`, and pointers to it in data.

`xref.py --va` matches code that takes an address (`lea`, `mov imm64`); `dataref.py`
matches code that reads or writes a global. Neither matches a direct CALL, so both report
**0 references** for a function with dozens of call sites - and on 2026-08-19 that clean
zero was very nearly believed. Verify with a positive control before trusting a negative:

    python tools/callers.py 0x1402fa9a0     # expect 96 sites, 43 of them in 0x140304b20
    python tools/callers.py 0x140302e30

## Three entry modes, because a scan for `E8` alone answered wrong twice

Until 2026-08-20 this file scanned for `E8` (`call rel32`) and nothing else. That is the
same blind spot `tools/reads.py` was fixed for on 2026-08-19, and it produced two stated
conclusions that were wrong:

**1. Tail `jmp rel32` (`E9`).** `FUN_1429bafb0` was written up as having *zero direct
callers, therefore virtual*. It is a tail-`jmp` target from `FUN_1429b9300` at
`0x1429b934b` - an ordinary call, just one whose return address belongs to the caller's
caller. The `0x007C` mask decoder was written up as having *one* caller; there is also a
tail `jmp` to it at `0x14104a8b0`.

**2. A qword pointer in data.** An `E8`+`E9` scan still reports **zero** for
`FUN_140304100`, which a real client run shows executing four times. It is reached through
a function-pointer slot in `.rdata` (`0x14327e530`) - a table entry, loaded and called
indirectly. Nothing in `.text` names it at all.

So all three are enumerated and reported as **distinct kinds**, because they are three
different facts:

* `call`      - a call site. There is a return address; the caller resumes after it.
* `tail jmp`  - a tail call. **No return address of its own**; it returns to whoever
                called the jumping function. A stack walk will not show the jumper.
* `data ptr`  - a qword holding this function's address. Whoever loads that slot can call
                it, and no instruction anywhere mentions the function. Follow up with
                `python tools/dataref.py <slot VA>` to find the code that loads it.

## What this still cannot see, and how it lies

* **Byte scanning is not disassembly.** `0xE8`/`0xE9` are perfectly ordinary ModRM and
  immediate bytes, so a match may be nothing. Every hit inside a `.pdata` function is
  therefore re-checked by disassembling that function and confirming the hit lands on a
  real instruction boundary. Hits that fail the check are printed with a `?` and kept -
  a failed check usually means the linear sweep desynced on a jump table, and dropping
  them would be exactly the silent filter this file exists to warn about. Judge a `?` by
  reading the listing.
* **A pointer in an executable section** is usually a `mov reg, imm64` operand rather
  than a table slot. Those are labelled; `tools/xref.py` is the better tool for them.
* **A run of pointers is not necessarily a vtable.** Where an MSVC RTTI locator sits
  immediately before the run this prints the class name and the `+offset`; where it does
  not, the run head is just the start of the contiguous pointer block and the offset is
  only meaningful against whatever base the code actually loads.
* **A fourth mode is not covered: the MSVC switch table**, which stores 4-byte RVAs, not
  qwords. Scanning for those was tried and is not worth a kind of its own here - every
  function's own `.pdata` record contains its RVA, so the scan returns one guaranteed
  self-hit and, for the handlers checked, nothing else. `tools/find_switch_tables.py` is
  the tool for that shape.
* Computed targets - `call qword [rax+0x18]`, anything Themida virtualised - leave no
  trace here at all. A total of zero across all three kinds still means "look again with
  a debugger", not "unreachable".
"""
import bisect
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rtti import load_pe          # noqa: E402
from dataref import parse_pdata   # noqa: E402

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64
    _md = Cs(CS_ARCH_X86, CS_MODE_64)
except ImportError:  # pragma: no cover - verification degrades, hits are still reported
    _md = None

IMAGE_SCN_MEM_EXECUTE = 0x20000000


class Image(object):
    """The bits of the PE every scan below needs, loaded once."""

    def __init__(self, path):
        self.data, self.base, self.sections = load_pe(path)
        self.funcs = parse_pdata(self.data, self.sections, self.base)
        self.starts = [f[0] for f in self.funcs]
        self.ends = {f[0]: f[1] for f in self.funcs}
        self.exec_ranges = [(self.base + s["vaddr"],
                             self.base + s["vaddr"] + max(s["vsize"], s["rsize"]))
                            for s in self.sections if s["chars"] & IMAGE_SCN_MEM_EXECUTE]
        self._decoded = {}

    # -- addresses ---------------------------------------------------------
    def section_of(self, va):
        rva = va - self.base
        for s in self.sections:
            if s["vaddr"] <= rva < s["vaddr"] + max(s["vsize"], s["rsize"]):
                return s
        return None

    def foff(self, va):
        """File offset of a VA, or None when it is outside the raw bytes.

        Deliberately None-returning rather than raising: this is used to probe addresses
        that may legitimately be unmapped (walking off the end of a pointer run). For the
        raising version - the one to use when a wrong answer would be believed - see
        `rtti.va_to_off`.
        """
        s = self.section_of(va)
        if s is None:
            return None
        d = va - self.base - s["vaddr"]
        return s["raddr"] + d if d < s["rsize"] else None

    def qword(self, va):
        off = self.foff(va)
        if off is None or off + 8 > len(self.data):
            return None
        return struct.unpack_from("<Q", self.data, off)[0]

    def is_code(self, va):
        return any(lo <= va < hi for lo, hi in self.exec_ranges)

    # -- .pdata ------------------------------------------------------------
    def containing(self, va):
        i = bisect.bisect_right(self.starts, va) - 1
        return self.funcs[i][0] if i >= 0 and self.funcs[i][0] <= va < self.funcs[i][1] else None

    def merged_extent(self, fn):
        """`fn`'s entry plus any entries that abut it, in both directions.

        One function can own several `.pdata` entries - they are unwind records, not
        function boundaries. Disassembling from a mid-function entry start desyncs, so
        the instruction-boundary check walks back to the first entry of the run.
        """
        i = bisect.bisect_left(self.starts, fn)
        start = self.starts[i]
        while i > 0 and self.ends[self.starts[i - 1]] == start:
            i -= 1
            start = self.starts[i]
        end = self.ends[start]
        j = bisect.bisect_right(self.starts, start)
        while j < len(self.starts) and self.starts[j] == end:
            end = self.ends[self.starts[j]]
            j += 1
        return start, end

    def instruction_starts(self, fn):
        if fn in self._decoded:
            return self._decoded[fn]
        out = set()
        if _md is not None:
            start, end = self.merged_extent(fn)
            off = self.foff(start)
            if off is not None:
                code = self.data[off:off + (end - start)]
                for ins in _md.disasm(code, start):
                    out.add(ins.address)
        self._decoded[fn] = out
        return out


CONVERGE_DEPTH = 32
CONVERGE_REAL = 20        # measured: 12 known-real sites scored 22..31 out of 32
CONVERGE_DOUBTFUL = 8     # 0xE8/0xE9 bytes picked out of `.rsrc` scored 0, 0, 0, 0, 9, 24


def converge(img, va, depth=CONVERGE_DEPTH):
    """How many of `depth` earlier start points decode onto `va` exactly.

    The anchor for a hit that sits in no `.pdata` function - a thunk, an adjustor thunk,
    the forwarder tables in the `0x143......` region - where there is no function start
    to decode from. x86 linear sweep resynchronises within a few instructions, so a real
    instruction boundary is reached from nearly every start and a byte that only looks
    like `E8` from one particular alignment is reached from almost none.

    Corroboration, not proof: one `0xE9` inside `.rsrc` scored 24/32. Read the listing
    before a number turns into a claim.
    """
    if _md is None:
        return None
    hits = 0
    for back in range(1, depth + 1):
        off = img.foff(va - back)
        if off is None:
            continue
        for ins in _md.disasm(img.data[off:off + back + 16], va - back):
            if ins.address == va:
                hits += 1
                break
            if ins.address > va:
                break
    return hits


def converge_note(img, va):
    n = converge(img, va)
    if n is None:
        return "unverified: capstone not installed"
    if n >= CONVERGE_REAL:
        verdict = "real"
    elif n >= CONVERGE_DOUBTFUL:
        verdict = "DOUBTFUL - read the listing"
    else:
        verdict = "PROBABLY A BYTE-SCAN COINCIDENCE"
    return "decode converges %d/%d (%s)" % (n, CONVERGE_DEPTH, verdict)


CC_BLOCK_LIMIT = 128


def cc_block_start(img, va):
    """Start of the `0xCC`-bounded block containing `va`, or None if there isn't one.

    A hit with no `.pdata` entry is usually inside a small shim that the linker padded
    with `int3` on both sides - `mov rcx,[rcx+0x78] / test / je / xor r8d,r8d / jmp <fn>`
    is the recurring shape here. Reporting the JMP's address alone is useless to whoever
    wants to know what reaches it; the block start is the address to ask about next.
    """
    for back in range(0, CC_BLOCK_LIMIT):
        off = img.foff(va - back - 1)
        if off is None:
            return None
        if img.data[off] == 0xCC:
            return va - back
    return None


def scan_rel32(img, target, opcode):
    """[(site VA, section name)] for every `opcode rel32` whose target is `target`.

    Byte scan, not disassembly - see the caveat in the module docstring. `bytes.find`
    does the work in C; the old byte-at-a-time loop walked 52 MB in Python.
    """
    needle = bytes([opcode])
    hits = []
    for s in img.sections:
        if not (s["chars"] & IMAGE_SCN_MEM_EXECUTE) or not s["rsize"]:
            continue
        blob = img.data[s["raddr"]:s["raddr"] + s["rsize"]]
        secbase = img.base + s["vaddr"]
        start = 0
        while True:
            i = blob.find(needle, start)
            if i < 0 or i + 5 > len(blob):
                break
            start = i + 1
            rel = struct.unpack_from("<i", blob, i + 1)[0]
            va = secbase + i
            if va + 5 + rel == target:
                hits.append((va, s["name"]))
    return hits


def scan_pointers(img, target):
    """[(slot VA, section, aligned)] for every qword in the image equal to `target`.

    Every section with raw bytes is scanned, executable ones included, because
    under-reporting is the failure this tool has already made twice. A hit in an
    executable section is almost always a `mov reg, imm64` operand and is labelled as
    such rather than dropped.
    """
    needle = struct.pack("<Q", target)
    hits = []
    for s in img.sections:
        if not s["rsize"]:
            continue
        blob = img.data[s["raddr"]:s["raddr"] + s["rsize"]]
        secbase = img.base + s["vaddr"]
        start = 0
        while True:
            i = blob.find(needle, start)
            if i < 0:
                break
            start = i + 1
            hits.append((secbase + i, s["name"],
                         (secbase + i) % 8 == 0,
                         bool(s["chars"] & IMAGE_SCN_MEM_EXECUTE)))
    return hits


def pointer_run(img, slot):
    """(head VA, count) of the contiguous run of code pointers `slot` belongs to."""
    head = slot
    while True:
        prev = img.qword(head - 8)
        if prev is None or not img.is_code(prev):
            break
        head -= 8
    tail = slot
    while True:
        nxt = img.qword(tail + 8)
        if nxt is None or not img.is_code(nxt):
            break
        tail += 8
    return head, (tail - head) // 8 + 1


def lea_targets_in(img, lo, hi):
    """{VA: [lea site, ...]} for every `lea reg,[rip+d]` landing in [lo, hi].

    This is what turns "+0x720 into a 477-slot pointer run" into "+0x358 from the base
    the code actually loads". MSVC writes a vfptr with `lea rax,[rip+vtable]` in the
    constructor, so the addresses inside a run that code takes are its real table bases -
    which matters here because these tables carry no RTTI locator to mark their heads.

    Byte-scanned, so a hit is a candidate; the range is narrow enough that a coincidence
    is unlikely but not impossible. `mov reg, imm64` bases are not matched.
    """
    out = {}
    for s in img.sections:
        if not (s["chars"] & IMAGE_SCN_MEM_EXECUTE) or not s["rsize"]:
            continue
        blob = img.data[s["raddr"]:s["raddr"] + s["rsize"]]
        secbase = img.base + s["vaddr"]
        start = 1
        while True:
            i = blob.find(b"\x8d", start)
            if i < 1 or i + 6 > len(blob):
                break
            start = i + 1
            if blob[i + 1] & 0xC7 != 0x05:        # mod=00, rm=101 -> RIP-relative
                continue
            rex = 1 if 0x40 <= blob[i - 1] <= 0x4F else 0
            va = secbase + i - rex
            tgt = va + rex + 6 + struct.unpack_from("<i", blob, i + 2)[0]
            if lo <= tgt <= hi:
                out.setdefault(tgt, []).append(va)
    return out


def rtti_class_at(img, head):
    """Class name if an MSVC CompleteObjectLocator sits at `head - 8`, else None."""
    col = img.qword(head - 8)
    if col is None:
        return None
    off = img.foff(col)
    if off is None or off + 0x18 > len(img.data):
        return None
    sig, _off, _cd, td, _cls, self_rva = struct.unpack_from("<IIIIII", img.data, off)
    if sig != 1 or self_rva != col - img.base:
        return None
    tdo = img.foff(img.base + td)
    if tdo is None:
        return None
    name = img.data[tdo + 0x10:tdo + 0x110].split(b"\0")[0]
    return name.decode("ascii", "replace") or None


def group(img, hits):
    by_fn = {}
    for va, _sec in hits:
        by_fn.setdefault(img.containing(va), []).append(va)
    return by_fn


def report_code_hits(img, target, hits, noun):
    by_fn = group(img, hits)
    print("%#x: %d %s(s) in %d function(s)" % (target, len(hits), noun, len(by_fn)))
    for fn in sorted(by_fn, key=lambda f: (f is None, f or 0)):
        rows = sorted(by_fn[fn])
        home = ("%#x" % fn) if fn else "(outside any .pdata function)"
        flags = []
        detail = []          # (va, why) printed one per line
        if fn is None:
            # No unwind record to decode from - a thunk, an adjustor thunk, or noise.
            # Every one of these gets its own convergence score; none is dropped.
            for va in rows:
                block = cc_block_start(img, va)
                where = ("in the 0xCC-bounded block at %#x - RUN THIS TOOL ON THAT "
                         "ADDRESS to find what reaches the shim" % block) \
                    if block is not None and block != va else \
                    ("0xCC-padded (thunk)" if block == va else
                     "no 0xCC boundary within %d bytes" % CC_BLOCK_LIMIT)
                detail.append((va, "no .pdata entry; %s; %s"
                               % (where, converge_note(img, va))))
        else:
            if fn == target:
                flags.append("SELF - intra-function control flow, not an entry")
            ok = img.instruction_starts(fn)
            if not ok:
                flags.append("boundary check unavailable: %s"
                             % ("capstone not installed" if _md is None
                                else "the function did not decode"))
            else:
                for va in rows:
                    if va not in ok:
                        detail.append((va, "not an instruction boundary in the linear "
                                           "sweep of %#x; %s" % (fn, converge_note(img, va))))
        note = ("   [%s]" % "; ".join(flags)) if flags else ""
        print("    %-14s %3d %s(s)   first %#x  last %#x%s"
              % (home, len(rows), noun, rows[0], rows[-1], note))
        for va, why in detail:
            print("        %#x  %s" % (va, why))


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 1
    args = [a for a in sys.argv[1:] if not a.startswith("-")]
    target = int(args[0], 16)
    exe = args[1] if len(args) > 1 else "client-patched/MapleStory.exe"

    img = Image(exe)

    calls = scan_rel32(img, target, 0xE8)
    jmps = scan_rel32(img, target, 0xE9)
    ptrs = scan_pointers(img, target)

    report_code_hits(img, target, calls, "call site")
    print()
    report_code_hits(img, target, jmps, "tail jmp site")
    print()

    print("%#x: %d qword pointer(s) to it in the image" % (target, len(ptrs)))
    for va, sec, aligned, in_code in sorted(ptrs):
        bits = [sec]
        if not aligned:
            bits.append("UNALIGNED - probably not a table slot")
        if in_code:
            bits.append("in an executable section - probably a `mov reg, imm64`; see xref.py")
        elif aligned:
            head, count = pointer_run(img, va)
            cls = rtti_class_at(img, head)
            if cls:
                bits.append("vtable %#x (%s) +%#x  [slot %d of %d]"
                            % (head, cls, va - head, (va - head) // 8, count))
            else:
                bits.append("pointer run %#x..%#x (%d slots), this is +%#x [slot %d]"
                            % (head, head + (count - 1) * 8, count, va - head, (va - head) // 8))
                bits.append("no RTTI locator at the run head")
                bases = lea_targets_in(img, head, head + (count - 1) * 8)
                below = [t for t in bases if t <= va]
                if below:
                    b = max(below)
                    bits.append("nearest base that code loads is %#x (lea at %s) -> "
                                "this slot is that base +%#x [index %d]"
                                % (b, ", ".join("%#x" % x for x in sorted(bases[b])[:3]),
                                   va - b, (va - b) // 8))
        print("    %#x  %s" % (va, "\n        ".join(bits)))
    if ptrs:
        print("    -> a pointer is not a call site. `python tools/dataref.py <slot VA>` "
              "finds the code that loads it.")

    total = len(calls) + len(jmps) + len(ptrs)
    if total == 0:
        print()
        print("NOTHING FOUND. Before concluding 'unreachable', run the documented control:")
        print("    python tools/callers.py 0x1402fa9a0   # 96 call sites, 43 in 0x140304b20")
        print("An indirect call through a register or a virtualised dispatcher leaves no "
              "trace in any of the three scans above.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
