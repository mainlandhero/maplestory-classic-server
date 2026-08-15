"""Turn the raw builder dump into a table of labelled outbound opcodes.

`DumpPacketFields.java` records, per call site, the opcode, the sequence of field writes,
and every string the builder function references. Most of those strings are noise - big
functions (the GM command handler especially) reference hundreds - so a string only counts
as evidence when it is *specific*: it appears under at most a handful of distinct opcodes.

Ground truth for this client, as opposed to guessing from published opcode lists for other
versions. Only the outbound half can be labelled this way; the inbound dispatcher lives in
the `.themida` section and has no file bytes.
"""

import collections
import re
import sys

BAD_CHARS = set("/%#\\")


def parse(path):
    rows = []
    with open(path, encoding="utf-8", errors="replace") as fh:
        for line in fh:
            m = re.match(r"^(0x[0-9A-F]{4})\s+(\S+)\s+(.*?)(?:\s+// (.*))?$", line.rstrip("\n"))
            if m:
                rows.append((m.group(1), m.group(2), m.group(3), m.group(4)))
    return rows


def name_like(s):
    """Short, identifier-ish: a name rather than a path, format string or command."""
    s = s.strip()
    if not 2 <= len(s) <= 28:
        return False
    if any(c in s for c in BAD_CHARS):
        return False
    if len(s.split()) > 3:
        return False
    return bool(re.match(r"^[A-Za-z][A-Za-z0-9_: ]*$", s))


def label(rows, max_spread=3):
    per_op = collections.defaultdict(collections.Counter)
    for op, _fn, _fields, strs in rows:
        if not strs:
            continue
        for s in {x.strip() for x in strs.split("|")}:
            if name_like(s):
                per_op[op][s] += 1

    spread = collections.Counter()
    for op in per_op:
        for s in per_op[op]:
            spread[s] += 1

    out = {}
    for op, cnt in per_op.items():
        cands = [s for s in cnt if spread[s] <= max_spread]
        if cands:
            out[op] = sorted(cands, key=lambda s: (-cnt[s], s))[:3]
    return out


def shapes(rows):
    """The most common field signature per opcode, as a structural fingerprint."""
    per_op = collections.defaultdict(collections.Counter)
    for op, _fn, fields, _strs in rows:
        prim = [f for f in fields.split(",") if f in ("u8", "u32")]
        if prim:
            per_op[op][",".join(prim[:8])] += 1
    return {op: c.most_common(1)[0][0] for op, c in per_op.items()}


def main():
    path = sys.argv[1] if len(sys.argv) > 1 else "research/msexe-packet-fields.txt"
    rows = parse(path)
    lab = label(rows)
    shp = shapes(rows)

    print(f"# {len(rows)} call sites, {len(lab)} opcodes with a specific string label\n")
    print("| Opcode | Label | Leading fields |")
    print("|---|---|---|")
    for op in sorted(lab):
        print(f"| `{op}` | {' / '.join(lab[op])} | `{shp.get(op, '')}` |")


if __name__ == "__main__":
    main()
