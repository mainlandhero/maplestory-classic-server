#!/usr/bin/env python3
"""Look for the inbound dispatch table by scoring pointer arrays.

The dispatcher `FUN_1415d60e0` is Themida-virtualised, so the opcode->handler mapping
cannot be read as code. But if the mapping is an *array of function pointers* it is
ordinary initialised data, and virtualising the dispatch loop does nothing to hide it.

The trouble is that .rdata is wall-to-wall C++ vtables, so "a run of code pointers" finds
542 candidates. This adds the discriminator: a packet handler must, directly or a few
calls down, reach one of the packet decode helpers -

    FUN_1406e8380   read u32   (bounds-check, returns 4)
    FUN_1406e82f0   read u8    (bounds-check, returns 1)

A vtable for some UI widget scores near zero on that test; a table of packet handlers
should score near one. Everything here is static:

  * `.pdata` (RUNTIME_FUNCTION[]) gives every function's start and end - no analysis pass
  * scanning `.text` for `E8`/`E9 rel32` gives the call graph
  * a reverse BFS from the decoders gives the set of packet-reading functions

    python tools/find_handler_table.py --depth 3 --min 16
    python tools/find_handler_table.py --show 0x1434... --count 64
"""
import argparse
import bisect
import collections
import struct
import sys

IMAGE_SCN_MEM_EXECUTE = 0x20000000

DECODERS = [0x1406E8380, 0x1406E82F0]


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


def section_by_name(sections, name):
    for s in sections:
        if s["name"] == name:
            return s
    return None


def parse_pdata(data, sections):
    """RUNTIME_FUNCTION[] -> sorted function start RVAs and their ends."""
    s = section_by_name(sections, ".pdata")
    starts, ends = [], {}
    blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
    for i in range(0, len(blob) - 11, 12):
        begin, end, _unwind = struct.unpack_from("<III", blob, i)
        if begin == 0 and end == 0:
            continue
        if end <= begin:
            continue
        starts.append(begin)
        # chunked functions repeat a start; keep the widest end
        ends[begin] = max(ends.get(begin, 0), end)
    starts = sorted(set(starts))
    return starts, ends


def build_callgraph(data, image_base, sections, starts, ends):
    """caller-RVA -> set of callee-RVAs, from direct E8/E9 rel32 in executable sections."""
    graph = collections.defaultdict(set)
    text = [s for s in sections
            if (s["chars"] & IMAGE_SCN_MEM_EXECUTE) and s["rsize"] and s["name"] == ".text"]
    fn_starts = starts
    for s in text:
        base_rva = s["vaddr"]
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        n = len(blob)
        i = 0
        while i < n - 4:
            b = blob[i]
            if b == 0xE8 or b == 0xE9:
                rel = struct.unpack_from("<i", blob, i + 1)[0]
                target = base_rva + i + 5 + rel
                # only keep calls that land exactly on a known function entry
                idx = bisect.bisect_left(fn_starts, target)
                if idx < len(fn_starts) and fn_starts[idx] == target:
                    site = base_rva + i
                    j = bisect.bisect_right(fn_starts, site) - 1
                    if j >= 0:
                        owner = fn_starts[j]
                        if site < ends.get(owner, 0):
                            graph[owner].add(target)
                i += 1
            else:
                i += 1
    return graph


def reverse_reach(graph, seeds, depth):
    """Functions that reach any seed within `depth` calls."""
    callers = collections.defaultdict(set)
    for caller, callees in graph.items():
        for c in callees:
            callers[c].add(caller)
    reached = set(seeds)
    frontier = set(seeds)
    for _ in range(depth):
        nxt = set()
        for f in frontier:
            for c in callers.get(f, ()):
                if c not in reached:
                    reached.add(c)
                    nxt.add(c)
        frontier = nxt
        if not frontier:
            break
    return reached


def exec_ranges(image_base, sections):
    out = []
    for s in sections:
        if s["chars"] & IMAGE_SCN_MEM_EXECUTE:
            lo = image_base + s["vaddr"]
            out.append((lo, lo + max(s["vsize"], s["rsize"])))
    return out


def in_exec(va, ranges):
    return any(lo <= va < hi for lo, hi in ranges)


def scan_tables(data, image_base, sections, ranges, handlers, minimum):
    """Every aligned pointer run, scored by how many entries are packet readers."""
    hits = []
    for s in sections:
        if s["rsize"] == 0 or (s["chars"] & IMAGE_SCN_MEM_EXECUTE):
            continue
        if s["name"] in (".pdata", ".rsrc", ".idata"):
            continue
        base_va = image_base + s["vaddr"]
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        start = None
        entries = 0
        good = 0
        last = None
        for i in range(0, len(blob) - 7, 8):
            q = struct.unpack_from("<Q", blob, i)[0]
            ok = q != 0 and in_exec(q, ranges)
            if ok:
                if start is None:
                    start = i
                    entries = good = 0
                entries += 1
                last = i
                if (q - image_base) in handlers:
                    good += 1
            elif q == 0 and start is not None:
                continue
            else:
                if start is not None and entries >= minimum:
                    hits.append((base_va + start, entries, good,
                                 (last - start) // 8 + 1, s["name"]))
                start, entries, good, last = None, 0, 0, None
        if start is not None and entries >= minimum:
            hits.append((base_va + start, entries, good,
                         (last - start) // 8 + 1, s["name"]))
    return hits


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--exe", default="client-patched/MapleStory.exe")
    ap.add_argument("--depth", type=int, default=3, help="call depth from a decoder")
    ap.add_argument("--min", type=int, default=16, help="fewest entries to score a run")
    ap.add_argument("--top", type=int, default=30)
    ap.add_argument("--show", help="dump entries at this VA instead of scanning")
    ap.add_argument("--count", type=int, default=64)
    args = ap.parse_args()

    data, image_base, sections = load_pe(args.exe)
    ranges = exec_ranges(image_base, sections)

    starts, ends = parse_pdata(data, sections)
    print(f"image base {image_base:#x}   {len(starts)} functions in .pdata")

    graph = build_callgraph(data, image_base, sections, starts, ends)
    edges = sum(len(v) for v in graph.values())
    print(f"call graph: {len(graph)} callers, {edges} direct edges")

    seeds = [va - image_base for va in DECODERS]
    for va, rva in zip(DECODERS, seeds):
        known = bisect.bisect_left(starts, rva) < len(starts) and \
            starts[bisect.bisect_left(starts, rva)] == rva
        print(f"  decoder {va:#x}  known function: {known}")

    handlers = reverse_reach(graph, seeds, args.depth)
    print(f"packet readers within depth {args.depth}: {len(handlers)} functions "
          f"({100.0 * len(handlers) / max(1, len(starts)):.1f}% of all)")

    if args.show:
        va = int(args.show, 16)
        for s in sections:
            lo = image_base + s["vaddr"]
            if lo <= va < lo + max(s["vsize"], s["rsize"]):
                off = s["raddr"] + (va - lo)
                for i in range(args.count):
                    q = struct.unpack_from("<Q", data, off + i * 8)[0]
                    tag = ""
                    if q and in_exec(q, ranges):
                        tag = "READER" if (q - image_base) in handlers else "code"
                    elif q == 0:
                        tag = "-"
                    print(f"  [{i:4}] @{va + i * 8:#x}  {q:#018x}  {tag}")
                return 0
        print(f"VA {va:#x} not in any section")
        return 1

    hits = scan_tables(data, image_base, sections, ranges, handlers, args.min)
    # rank by how packet-shaped the run is, then by size
    hits.sort(key=lambda h: (-(h[2] / h[1]), -h[1]))
    print(f"\n{len(hits)} runs of >= {args.min} pointers; "
          f"best reader-density first:\n")
    print(f"  {'VA':<16} {'entries':>7} {'readers':>7} {'density':>8} {'span':>6}  section")
    for va, entries, good, span, sect in hits[:args.top]:
        print(f"  {va:#014x} {entries:7} {good:7} {good / entries:8.2f} {span:6}  {sect}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
