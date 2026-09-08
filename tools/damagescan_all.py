"""Run tools/damagescan.py over every dump in dumps/ and tabulate.

Sequential on purpose: a client is usually running on this machine and each sweep reads a
1.3 GB file. One at a time, and the per-dump JSON is kept so the table can be rebuilt
without re-reading 46 GB.

    python tools/damagescan_all.py                 # scan, then table
    python tools/damagescan_all.py --table-only    # rebuild the table from the JSON
"""

import glob
import json
import os
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
OUT = os.path.join(ROOT, "research", "damagescan")


def main():
    table_only = "--table-only" in sys.argv
    if not os.path.isdir(OUT):
        os.makedirs(OUT)
    dumps = sorted(glob.glob(os.path.join(ROOT, "dumps", "*.dmp")))
    for d in dumps:
        j = os.path.join(OUT, os.path.basename(d) + ".json")
        if table_only or os.path.exists(j):
            continue
        txt = os.path.join(OUT, os.path.basename(d) + ".txt")
        sys.stderr.write("scanning %s\n" % os.path.basename(d))
        sys.stderr.flush()
        with open(txt, "w") as f:
            subprocess.call([sys.executable, os.path.join(HERE, "damagescan.py"),
                             d, "--json", j], stdout=f, stderr=subprocess.STDOUT,
                            cwd=ROOT)

    rows = []
    for d in dumps:
        j = os.path.join(OUT, os.path.basename(d) + ".json")
        if not os.path.exists(j):
            continue
        try:
            rows.append(json.load(open(j)))
        except ValueError:
            sys.stderr.write("unreadable json for %s\n" % d)

    print("%-46s %6s %5s %8s %7s %7s %4s %4s %4s %4s %4s"
          % ("dump", "alive", "code", "slots", "listed", "scan", "D1", "D2", "D3", "D4",
             "OBJ"))
    for r in rows:
        print("%-46s %6s %5s %8d %7d %7d %4d %4d %4d %4d %4d"
              % (r["dump"][:46], r.get("alive"),
                 ("%x" % r["code"])[-4:] if r.get("code") else "-",
                 r["slots"], r.get("chunks_listed", 0), r.get("chunks_scan", 0),
                 len(r["d1"]), len(r.get("d2_t4", [])), len(r["d3"]),
                 r.get("d4_strict", 0), len(r["objects"])))

    print("")
    print("high-dword values seen at tier 2, across all dumps:")
    from collections import Counter
    hi = Counter()
    for r in rows:
        for k, v in (r.get("d2_hi") or {}).items():
            hi[k] += v
    for k, v in hi.most_common(30):
        print("   %-14s %d" % (k, v))

    print("")
    print("offsets of confirmed damage within the allocation, across all dumps:")
    off = Counter()
    for r in rows:
        for b, tags in r["objects"].items():
            for t in tags:
                if "@" in t:
                    off[t.split("@")[1]] += 1
    for k, v in off.most_common(30):
        print("   %-8s %d" % (k, v))


main()
