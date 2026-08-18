"""Find a function's real bounds from the PE exception table.

Ghidra does not have a function for every address in this binary - `getFunctionContaining`
returned null for the epilogue that raised the stack-cookie failure, and DecompileFunc then
created a function starting at the epilogue, which decompiles to nothing useful.

x64 PE files carry .pdata: an array of RUNTIME_FUNCTION {beginRVA, endRVA, unwindRVA},
sorted, covering every function with unwind data. That gives exact bounds without
disassembling anything, and it is authoritative - the linker wrote it.

    python tools/pdata_lookup.py 142e9fe03 [more VAs...]
"""
import bisect
import struct
import sys


def load(path):
    data = open(path, "rb").read()
    pe = struct.unpack_from("<I", data, 0x3C)[0]
    opt = struct.unpack_from("<H", data, pe + 20)[0]
    nsec = struct.unpack_from("<H", data, pe + 6)[0]
    base = struct.unpack_from("<Q", data, pe + 24 + 24)[0]
    sec_off = pe + 24 + opt
    sections = []
    for i in range(nsec):
        o = sec_off + i * 40
        name = data[o:o + 8].rstrip(b"\0").decode("latin1")
        vsize, va, rawsize, raw = struct.unpack_from("<IIII", data, o + 8)
        sections.append((name, va, vsize, raw, rawsize))
    return data, base, sections


def main():
    path = r"client-patched\MapleStory.exe"
    args = sys.argv[1:]
    if args and args[0].lower().endswith(".exe"):
        path = args.pop(0)
    data, base, sections = load(path)
    pdata = next((s for s in sections if s[0] == ".pdata"), None)
    if pdata is None:
        print("no .pdata section")
        return
    _, va, vsize, raw, rawsize = pdata
    n = min(vsize, rawsize) // 12
    begins = []
    entries = []
    for i in range(n):
        b, e, u = struct.unpack_from("<III", data, raw + i * 12)
        if b == 0 and e == 0:
            continue
        begins.append(b)
        entries.append((b, e, u))
    print("%d RUNTIME_FUNCTION entries, image base %#x" % (len(entries), base))

    for arg in args:
        target = int(arg, 16)
        rva = target - base
        i = bisect.bisect_right(begins, rva) - 1
        if i < 0:
            print("%#x: before the first entry" % target)
            continue
        b, e, u = entries[i]
        if rva >= e:
            print("%#x: falls in no function (nearest starts %#x, ends %#x)"
                  % (target, base + b, base + e))
            continue
        print("%#x  ->  function %#x .. %#x  (%d bytes, offset +%#x)"
              % (target, base + b, base + e, e - b, rva - b))


if __name__ == "__main__":
    main()
