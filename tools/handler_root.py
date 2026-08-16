#!/usr/bin/env python3
"""Find the dispatcher entry point above a function, before aiming a walk at it.

`crates/grap-stub`'s opcode walk arms an `int3` on a target and asks which opcode reaches
it. That only works if the target is a **dispatcher entry** - a function nothing in the
binary calls directly, because the only caller is the Themida-virtualised dispatcher.

Aim it at a callee instead and the walk is worthless in a way that looks like a clean
miss: the entry handler runs, takes some branch that our synthetic body did not satisfy,
and never reaches the function being watched. That cost a full 1498-opcode run against
`FUN_141b307b0`, whose real entry is `FUN_141b25f30` one level up.

So: climb the call graph until callers run out, and walk for *that*.

    python tools/handler_root.py 0x141b307b0
    python tools/handler_root.py 0x141b307b0 --depth 8
"""
import argparse
import collections
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from find_handler_table import build_callgraph, load_pe, parse_pdata  # noqa: E402


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("address", help="function VA, e.g. 0x141b307b0")
    ap.add_argument("--exe", default="client-patched/MapleStory.exe")
    ap.add_argument("--depth", type=int, default=8)
    args = ap.parse_args()

    data, base, sections = load_pe(args.exe)
    starts, ends = parse_pdata(data, sections)
    graph = build_callgraph(data, base, sections, starts, ends)
    callers = collections.defaultdict(set)
    for c, cs in graph.items():
        for x in cs:
            callers[x].add(c)

    want = int(args.address, 16) - base
    roots = []
    frontier = {want}
    seen = {want}
    for level in range(args.depth):
        nxt = set()
        for f in sorted(frontier):
            up = callers.get(f, set())
            size = ends.get(f, 0) - f
            if not up:
                roots.append(f)
                print(f"{'  ' * level}FUN_{base + f:x}  size {size:6}  "
                      f"<-- DISPATCHER ENTRY (no direct callers)")
            else:
                print(f"{'  ' * level}FUN_{base + f:x}  size {size:6}  "
                      f"called by {len(up)}")
            nxt |= up - seen
            seen |= up
        if not nxt:
            break
        frontier = nxt

    print()
    if not roots:
        print("No dispatcher entry found within --depth. Either it is deeper, or this "
              "function is reached indirectly (a vtable), which the walk cannot follow.")
        return 1
    if roots == [want]:
        print(f"{args.address} is itself a dispatcher entry - safe to walk for directly.")
    else:
        print("Walk for these instead, not the address you gave:")
        for r in roots:
            print(f"    -Probe <from>-<to>@{base + r:x}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
