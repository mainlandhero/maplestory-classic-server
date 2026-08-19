#!/usr/bin/env python3
"""Find the client's inbound packet dispatchers by call-graph shape, not by guessing.

A dispatcher mostly does not read the packet - it switches on the opcode and forwards the
`CInPacket*` to a handler. The handlers do read it, through five known primitives:

    FUN_1406e8ae0  u8      FUN_1406e8b80  u16     FUN_1406e8c20  u32
    FUN_1406e8f00  u32 (a thunk to the above)     FUN_1406e8f10  u64
    FUN_1406e9050  string  FUN_1406e9170  n raw bytes

So: a **handler** is any function that calls one of those. A **dispatcher** is a function
that calls many distinct handlers and reads no packet fields of its own. That shape is
visible in the direct call graph alone, which matters here because the packet dispatcher
proper is Themida-virtualised and the stage classes carry no RTTI - neither the call graph
above them nor the type system can name them.

    python tools/dispatchers.py                 # rank every candidate
    python tools/dispatchers.py --min 8         # only strong ones

**This scans for E8/E9 rel32 at every byte offset**, so it can hallucinate a call out of
data that happens to encode one. Every edge is therefore attributed through `.pdata`, and
a site outside any known function is dropped rather than guessed at. Treat the ranking as
a list of candidates to decompile, never as a finding on its own.
"""
import argparse
import bisect
import collections
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rtti import load_pe  # noqa: E402

# **Seven entries, not five.** A census of every `0x1406e8xxx`/`0x1406e9xxx` call target in
# two decoded handlers turned up two more after the first five had been used for weeks:
# `0x1406e8f10` reads a **u64**, and `0x1406e8f00` is a bare `JMP 0x1406e8c20` - a thunk to
# the u32 reader that a grep for the target address cannot see. Missing them made a field
# census short by nine reads in one function and produced a confident, wrong story about the
# decompiler duplicating call sites. Enumerate the call targets before trusting a list.
DECODERS = {
    0x1406E8AE0: "u8",
    0x1406E8B80: "u16",
    0x1406E8C20: "u32",
    0x1406E8F00: "u32 (thunk to 0x1406e8c20)",
    0x1406E8F10: "u64",
    0x1406E9050: "str",
    0x1406E9170: "raw",
}


def parse_pdata(data, sections, base):
    for s in sections:
        if s["name"] == ".pdata":
            blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
            out = []
            for i in range(0, len(blob) - 11, 12):
                b, e, _ = struct.unpack_from("<III", blob, i)
                if b and e > b:
                    out.append((base + b, base + e))
            out.sort()
            return out
    return []


def build_graph(data, sections, base, funcs):
    starts = [f[0] for f in funcs]
    startset = set(starts)
    out = collections.defaultdict(set)

    def containing(va):
        i = bisect.bisect_right(starts, va) - 1
        return funcs[i][0] if i >= 0 and funcs[i][0] <= va < funcs[i][1] else None

    for s in sections:
        if not (s["chars"] & 0x20000000) or not s["rsize"]:
            continue
        blob = data[s["raddr"]:s["raddr"] + s["rsize"]]
        secbase = base + s["vaddr"]
        for op in (0xE8, 0xE9):
            i = -1
            while True:
                i = blob.find(bytes([op]), i + 1)
                if i < 0 or i + 5 > len(blob):
                    break
                rel = struct.unpack_from("<i", blob, i + 1)[0]
                site = secbase + i
                target = site + 5 + rel
                if target in startset:
                    home = containing(site)
                    if home is not None and home != target:
                        out[home].add(target)
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--exe", default="client-patched/MapleStory.exe")
    ap.add_argument("--min", type=int, default=6, help="minimum distinct handlers called")
    ap.add_argument("--top", type=int, default=60)
    args = ap.parse_args()

    data, base, sections = load_pe(args.exe)
    funcs = parse_pdata(data, sections, base)
    graph = build_graph(data, sections, base, funcs)
    print(f"{len(funcs)} functions in .pdata, {sum(len(v) for v in graph.values())} call edges")

    handlers = {f for f, cs in graph.items() if cs & DECODERS.keys()}
    print(f"{len(handlers)} functions read packet fields directly")

    # A dispatcher mostly forwards, but it does not do so purely: the login stage's
    # OnPacket reads a byte inline for two of its 34 cases. Excluding every function that
    # touches a decoder therefore excluded the one dispatcher already known to be real,
    # and the sweep returned zero. Rank by handlers called and let the reader judge.
    scored = []
    for f, cs in graph.items():
        n = len(cs & handlers)
        if n >= args.min:
            scored.append((n, f, len(cs)))
    scored.sort(reverse=True)
    print(f"{len(scored)} dispatcher candidates (>= {args.min} distinct handlers)\n")
    for n, f, total in scored[:args.top]:
        mark = " *reads packet itself" if f in handlers else ""
        print(f"  {f:#x}   {n:3} handlers of {total:3} callees{mark}")


if __name__ == "__main__":
    sys.exit(main())
