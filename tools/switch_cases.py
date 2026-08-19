#!/usr/bin/env python3
"""Extract every case of a decompiled switch, including the ones with inline bodies.

Written because the first pass over `FUN_142cbaa80` matched only cases shaped like

    case 0x70:
      FUN_142d51930(param_1,param_3);
      break;

which is 181 of its **285** labels. The other 104 have their handler inlined, so a
single-call regex drops them silently and the gaps look like unhandled opcodes. They are
not. Do not reason about "which opcodes the client ignores" from a table built that way.

    python tools/switch_cases.py research/msexe-gamestage-dispatch.c FUN_142cbaa80
"""
import argparse
import re
import sys

CASE = re.compile(r"^\s*case (0x[0-9a-fA-F]+|\d+):\s*$")
CALL = re.compile(r"\b(FUN_[0-9a-f]+)\s*\(")
FORWARD = re.compile(r"^\s*(FUN_[0-9a-f]+)\((?:param_1|param_1 \+ -0x18),param_3\);\s*$")
END = re.compile(r"^\s*(break|return|goto \w+);\s*$")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("source")
    ap.add_argument("function", help="e.g. FUN_142cbaa80")
    ap.add_argument("--on", default="param_2",
                    help="the switch variable, so nested switches are not merged in")
    args = ap.parse_args()

    text = open(args.source, encoding="utf-8", errors="replace").read()
    i = text.index(f"// {args.function} @")
    nxt = text.find("//===========================================================\n// FUN_", i + 100)
    body = text[i:nxt if nxt > 0 else len(text)]
    lines = body.splitlines()

    # Only the cases of the OUTER switch on the opcode. Nested switches inside a case
    # body have their own `case` labels on unrelated values - taking all of them made
    # `FUN_142cbaa80` look like it handled opcodes 0x0..0x3 as well.
    depth, want = 0, None
    out, pending, acc = [], [], []
    for line in lines:
        opens, closes = line.count("{"), line.count("}")
        if want is None and ("switch(" + args.on + ")") in line.replace(" ", ""):
            want = depth + opens
            depth += opens - closes
            continue
        inside = want is not None and depth == want
        depth += opens - closes
        if want is not None and depth < want:
            break                          # the outer switch closed
        if not inside:
            continue
        m = CASE.match(line)
        if m:
            if acc:                       # a body ended without break: flush it
                out.append((pending, acc)); pending, acc = [], []
            pending.append(int(m.group(1), 0))
            continue
        if not pending:
            continue
        if END.match(line):
            out.append((pending, acc)); pending, acc = [], []
            continue
        acc.append(line)
    if pending:
        out.append((pending, acc))

    n_labels = sum(len(ops) for ops, _ in out)
    print(f"# {len(out)} case bodies, {n_labels} labels, from {args.function}")
    print("# opcode  handler-or-shape")
    for ops, blk in sorted(out, key=lambda g: g[0][0]):
        fwd = None
        for line in blk:
            m = FORWARD.match(line)
            if m:
                fwd = m.group(1)
                break
        if fwd and len([l for l in blk if l.strip()]) == 1:
            shape = fwd
        else:
            calls = []
            for line in blk:
                for c in CALL.findall(line):
                    if c not in calls:
                        calls.append(c)
            shape = "INLINE " + (" ".join(calls[:3]) if calls else "(no calls)")
            if fwd:
                shape = f"{fwd} +INLINE"
        for op in ops:
            print(f"0x{op:04x}  {shape}")


if __name__ == "__main__":
    sys.exit(main())
