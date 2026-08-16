#!/usr/bin/env python3
"""Find the code that references a string or address, without Ghidra.

String xrefs are the technique that has actually worked on this client - the IAT is
rebuilt so import xrefs are useless, but string references survive. Ghidra can do this,
but a headless run costs minutes and needs the JDK 21 dance; this costs seconds and is
scriptable.

x64 code reaches data two ways, and both are matched here:

    lea  reg, [rip+disp32]     48/4C 8D /r      the common case for a string address
    mov  reg, imm64            48/49 B8+r       used for absolute addresses

`.pdata` (RUNTIME_FUNCTION[]) maps a hit back to its containing function, so results come
out as function addresses rather than raw offsets.

    python tools/xref.py --string "UI/Login.img"       # ascii and utf-16
    python tools/xref.py --va 0x143a86810              # who touches the key table
    python tools/xref.py --string Login.img --callers  # and who calls those functions
"""
import argparse
import bisect
import collections
import struct
import sys

IMAGE_SCN_MEM_EXECUTE = 0x20000000


def load_pe(path):
    with open(path, "rb") as fh:
        data = fh.read()
    e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
    assert data[e_lfanew:e_lfanew + 4] == b"PE\0\0", "not a PE"
    coff = e_lfanew + 4
    n_sections = struct.unpack_from("<H", data, coff + 2)[0]
    size_opt = struct.unpack_from("<H", data, coff + 16)[0]
    opt = coff + 20
    assert struct.unpack_from("<H", data, opt)[0] == 0x20B, "expected PE32+"
    image_base = struct.unpack_from("<Q", data, opt + 24)[0]
    sections = []
    tbl = opt + size_opt
    for i in range(n_sections):
        off = tbl + i * 40
        name = data[off:off + 8].rstrip(b"\0").decode("ascii", "replace")
        vsize, vaddr, rsize, raddr = struct.unpack_from("<IIII", data, off + 8)
        chars = struct.unpack_from("<I", data, off + 36)[0]
        sections.append({"name": name, "vaddr": vaddr, "vsize": vsize,
                         "raddr": raddr, "rsize": rsize, "chars": chars})
    return data, image_base, sections


def off_to_rva(off, sections):
    for s in sections:
        if s["rsize"] and s["raddr"] <= off < s["raddr"] + s["rsize"]:
            return s["vaddr"] + (off - s["raddr"])
    return None


def parse_pdata(data, sections):
    s = next(x for x in sections if x["name"] == ".pdata")
    blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
    ends = {}
    for i in range(0, len(blob) - 11, 12):
        begin, end, _ = struct.unpack_from("<III", blob, i)
        if begin and end > begin:
            ends[begin] = max(ends.get(begin, 0), end)
    return sorted(ends), ends


def owner(rva, starts, ends):
    j = bisect.bisect_right(starts, rva) - 1
    if j < 0:
        return None
    st = starts[j]
    return st if rva < ends.get(st, 0) else None


def scan_refs(data, image_base, sections, targets):
    """target RVA -> list of (site RVA, kind). Scans executable sections only."""
    found = collections.defaultdict(list)
    tset = set(targets)
    for s in sections:
        if not (s["chars"] & IMAGE_SCN_MEM_EXECUTE) or not s["rsize"]:
            continue
        base_rva = s["vaddr"]
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        n = len(blob)
        for i in range(n - 7):
            b0 = blob[i]
            # REX.W lea reg, [rip+disp32]  ->  48/4C 8D <modrm with mod=00 rm=101>
            if (b0 == 0x48 or b0 == 0x4C) and blob[i + 1] == 0x8D:
                modrm = blob[i + 2]
                if (modrm & 0xC7) == 0x05:
                    disp = struct.unpack_from("<i", blob, i + 3)[0]
                    t = base_rva + i + 7 + disp
                    if t in tset:
                        found[t].append((base_rva + i, "lea"))
            # movabs reg, imm64  ->  48/49 B8+r
            elif (b0 == 0x48 or b0 == 0x49) and 0xB8 <= blob[i + 1] <= 0xBF:
                if i + 10 <= n:
                    imm = struct.unpack_from("<Q", blob, i + 2)[0]
                    if imm >= image_base and (imm - image_base) in tset:
                        found[imm - image_base].append((base_rva + i, "movabs"))
    return found


def find_string(data, sections, text, image_base):
    """RVAs of ascii and utf-16 copies of `text` that live in a real section."""
    out = []
    for enc, blob in (("ascii", text.encode()), ("utf16", text.encode("utf-16-le"))):
        start = 0
        while True:
            i = data.find(blob, start)
            if i < 0:
                break
            start = i + 1
            rva = off_to_rva(i, sections)
            if rva is not None:
                out.append((rva, enc))
    return out


def build_callers(data, image_base, sections, starts, ends):
    callers = collections.defaultdict(set)
    for s in sections:
        if not (s["chars"] & IMAGE_SCN_MEM_EXECUTE) or not s["rsize"]:
            continue
        base_rva = s["vaddr"]
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        for i in range(len(blob) - 4):
            if blob[i] in (0xE8, 0xE9):
                rel = struct.unpack_from("<i", blob, i + 1)[0]
                t = base_rva + i + 5 + rel
                j = bisect.bisect_left(starts, t)
                if j < len(starts) and starts[j] == t:
                    o = owner(base_rva + i, starts, ends)
                    if o is not None:
                        callers[t].add(o)
    return callers


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--exe", default="client-patched/MapleStory.exe")
    ap.add_argument("--string", help="string literal to find references to")
    ap.add_argument("--va", help="address to find references to")
    ap.add_argument("--callers", action="store_true",
                    help="also list functions calling each referencing function")
    args = ap.parse_args()

    data, image_base, sections = load_pe(args.exe)
    starts, ends = parse_pdata(data, sections)

    targets = []
    if args.string:
        hits = find_string(data, sections, args.string, image_base)
        print(f"{len(hits)} copies of {args.string!r}:")
        for rva, enc in hits:
            print(f"  {image_base + rva:#x}  {enc}")
        targets = [r for r, _ in hits]
    elif args.va:
        targets = [int(args.va, 16) - image_base]
    else:
        ap.print_help()
        return 1

    refs = scan_refs(data, image_base, sections, targets)
    total = sum(len(v) for v in refs.values())
    print(f"\n{total} code references, in these functions:\n")

    funcs = collections.defaultdict(list)
    for t, sites in refs.items():
        for site, kind in sites:
            o = owner(site, starts, ends)
            funcs[o].append((image_base + t, image_base + site, kind))

    callers = None
    if args.callers:
        callers = build_callers(data, image_base, sections, starts, ends)

    for fn in sorted(funcs, key=lambda x: (x is None, x)):
        label = f"FUN_{image_base + fn:x}" if fn is not None else "(no function)"
        size = ends.get(fn, 0) - fn if fn is not None else 0
        print(f"  {label}  size {size}")
        for t, site, kind in funcs[fn]:
            print(f"      {kind:7} at {site:#x} -> {t:#x}")
        if callers is not None and fn is not None:
            up = sorted(callers.get(fn, ()))
            print(f"      called by {len(up)}: " +
                  ", ".join(f"FUN_{image_base + u:x}" for u in up[:8]))
    return 0


if __name__ == "__main__":
    sys.exit(main())
