#!/usr/bin/env python3
"""Disassemble a raw VA range, for code `.pdata` does not bound.

    python tools/dis_at.py 0x140ca6ea0 0x60
    python tools/dis_at.py 0x140caf150 0x80 --exe client-patched/MapleStory.exe

`tools/listing.py` needs a `.pdata` RUNTIME_FUNCTION to know where a function ends, and
leaf functions with no unwind data have none - `python tools/pdata_lookup.py 0x140ca6ea0`
answers "falls in no function". That is the normal state for a small leaf, not evidence
that the address is not code. This prints whatever bytes are there, bounded by a length
you give, and stops at the first `ret`/`jmp`-then-`int3` run if `--stop` is passed.

## Verify it before you believe it

    python tools/dis_at.py 0x140ca22a0 0x52

must print the same 27 instructions, at the same addresses, that
`python tools/listing.py 0x140ca22a0` prints - ending `lea rax, [r8 + rbx*4]` /
`pop rdi` / `ret`. If it does not, the loader is wrong and nothing below it is evidence.

## Blind spots

* A linear sweep desyncs on inline jump tables and on data embedded in code. An
  implausible instruction stream after a `jmp` is usually that, not obfuscation.
* Nothing here follows a branch. This is a window, not a function.
"""
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from rtti import load_pe  # noqa: E402

try:
    from capstone import Cs, CS_ARCH_X86, CS_MODE_64  # noqa: E402
except ImportError:
    sys.exit("capstone not installed: python -m pip install capstone")

DEFAULT_EXE = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                           "client-patched", "MapleStory.exe")


def va_to_off(va, image_base, sections):
    rva = va - image_base
    for s in sections:
        if s["rsize"] and s["vaddr"] <= rva < s["vaddr"] + s["rsize"]:
            return s["raddr"] + (rva - s["vaddr"]), s["name"]
    for s in sections:
        if s["vaddr"] <= rva < s["vaddr"] + max(s["vsize"], s["rsize"]):
            return None, s["name"]        # inside a section, past its raw bytes
    return None, None


def dis(va, length, exe=DEFAULT_EXE, stop=False, out=sys.stdout):
    data, image_base, sections = load_pe(exe)
    off, sect = va_to_off(va, image_base, sections)
    if off is None:
        print("; 0x%x: no raw bytes (section %s) - zero on disk, unpacked at runtime"
              % (va, sect), file=out)
        return
    blob = data[off:off + length]
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    md.detail = False
    print("; 0x%x .. 0x%x  (%d bytes, section %s)" % (va, va + length, length, sect),
          file=out)
    seen_ret = False
    for ins in md.disasm(blob, va):
        print(" %012x  %-8s %s" % (ins.address, ins.mnemonic, ins.op_str), file=out)
        if stop:
            if ins.mnemonic in ("ret", "jmp"):
                seen_ret = True
            elif seen_ret and ins.mnemonic == "int3":
                break
            elif ins.mnemonic != "int3":
                seen_ret = False


def main():
    if len(sys.argv) < 3:
        sys.exit(__doc__)
    va = int(sys.argv[1], 0)
    length = int(sys.argv[2], 0)
    exe = DEFAULT_EXE
    if "--exe" in sys.argv:
        exe = sys.argv[sys.argv.index("--exe") + 1]
    dis(va, length, exe=exe, stop="--stop" in sys.argv)


if __name__ == "__main__":
    main()
