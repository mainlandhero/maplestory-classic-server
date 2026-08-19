#!/usr/bin/env python3
"""Extract every portal's destination and every field's NPCs from Map.wz.

The server has to answer the client's transfer-field request (`0x00D1`) with the map the
portal actually leads to. That mapping is not the server's to invent - it is in the client's
own data, one `portal` node per field image:

    portal/<n>/pn   the portal's name, which is what 0x00D1 carries
    portal/<n>/tm   the target map      (999999999 means "none" - a spawn point)
    portal/<n>/tn   the target portal on that map

Written 2026-08-19 after a hand-typed two-row stub let a character walk from map 1 to map 10
and then get stuck there: every portal out of map 10 was "not in the table", so the server
re-sent map 10 and the client bounced back where it started.

The same pass emits the `life` node's NPCs, because the client cannot spawn them itself -
its field loader walks `life` only to preload art, and the only code that builds a populated
NPC takes a packet (`research/npc-spawn.md`). So every NPC on every map is the server's to
send, and this is where the data comes from.

    python tools/dump_portals.py                 # writes gm-handbook/{portals,npcs}.txt
    python tools/dump_portals.py --out-dir tmp

Output is game data regenerated from the client, so it is gitignored deliberately - the repo
carries the code, not the content. Same arrangement as `tools/dump_names.py`.
"""
import argparse
import json
import os
import subprocess
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

MAP_ROOT = os.path.join("client-patched", "Data", "Map", "Map")
WZ_DUMP = os.path.abspath(
    os.path.join("target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
)

# The client's own "no target" sentinel. A portal with this tm is a spawn point, not a door.
NO_TARGET = 999999999


def run(*args):
    # Explicit UTF-8: Windows would otherwise decode wz-dump's output with the ANSI
    # code page, and one map image carries bytes cp1252 has no mapping for.
    r = subprocess.run([WZ_DUMP, *args], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        raise SystemExit("wz-dump %s failed (rc=%s): %s"
                         % (" ".join(args), r.returncode, (r.stderr or "").strip()[:200]))
    return r.stdout


def archives():
    """Every Map<N>_000.wz under the Map root, in name order."""
    out = []
    for entry in sorted(os.listdir(MAP_ROOT)):
        d = os.path.join(MAP_ROOT, entry)
        if not os.path.isdir(d):
            continue
        for f in sorted(os.listdir(d)):
            if f.endswith("_000.wz"):
                out.append(os.path.join(d, f))
    return out


def images(archive):
    """Image names in an archive, from `wz-dump tree`."""
    for line in run("tree", archive, "1").splitlines():
        if "[IMG]" in line:
            yield line.split("[IMG]", 1)[1].split()[0]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-dir", default="gm-handbook")
    args = ap.parse_args()

    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)

    rows, npcs, fields, spawns = [], [], 0, 0
    for archive in archives():
        for image in images(archive):
            stem = image[:-4] if image.endswith(".img") else image
            if not stem.isdigit():
                continue
            map_id = int(stem)
            try:
                node = json.loads(run("cat", archive, image))
            except SystemExit:
                print("  ! could not read %s" % image, file=sys.stderr)
                continue
            fields += 1
            # life: type "n" is an NPC, "m" a mob. Mobs are not sent yet.
            for _, l in sorted((node.get("life") or {}).items(),
                               key=lambda kv: int(kv[0]) if kv[0].isdigit() else 0):
                if not isinstance(l, dict) or l.get("type") != "n":
                    continue
                try:
                    template = int(str(l.get("id", "")).lstrip("0") or "0")
                except ValueError:
                    continue
                if not template:
                    continue
                npcs.append((map_id, template, int(l.get("x", 0)), int(l.get("cy", l.get("y", 0))),
                             int(l.get("fh", 0)), int(l.get("rx0", 0)), int(l.get("rx1", 0)),
                             int(l.get("f", 0))))

            for key, p in sorted((node.get("portal") or {}).items(),
                                 key=lambda kv: int(kv[0]) if kv[0].isdigit() else 0):
                if not isinstance(p, dict):
                    continue
                idx = int(key) if str(key).isdigit() else -1
                name, target = p.get("pn", ""), p.get("tm")
                if not name or idx < 0:
                    continue
                # Spawn points are emitted too, with target 0. The server needs them: an
                # arriving character is placed at the portal NAMED by the source portal's
                # `tn`, and that lookup is by (map, name) -> index regardless of whether the
                # destination portal itself leads anywhere.
                if target is None or target == NO_TARGET:
                    spawns += 1
                    rows.append((map_id, idx, name, 0, ""))
                    continue
                rows.append((map_id, idx, name, int(target), p.get("tn", "") or ""))

    os.makedirs(args.out_dir, exist_ok=True)
    path = os.path.join(args.out_dir, "portals.txt")
    with open(path, "w", encoding="utf-8") as fh:
        fh.write("# map, index, portal, target map, target portal - generated by "
                 "tools/dump_portals.py from the client's Map.wz\n")
        for r in rows:
            fh.write("%d, %d, %s, %d, %s\n" % r)
    print("%s: %d portals across %d fields (%d of them spawn points, kept so an arrival "
          "portal can be resolved by name)" % (path, len(rows), fields, spawns))

    npath = os.path.join(args.out_dir, "npcs.txt")
    with open(npath, "w", encoding="utf-8") as fh:
        fh.write("# map, template, x, cy, fh, rx0, rx1, f - generated by "
                 "tools/dump_portals.py from the client's Map.wz `life` nodes\n")
        for r in npcs:
            fh.write("%d, %d, %d, %d, %d, %d, %d, %d\n" % r)
    print("%s: %d NPCs across %d maps"
          % (npath, len(npcs), len({n[0] for n in npcs})))
    return 0


if __name__ == "__main__":
    sys.exit(main())
