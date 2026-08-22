"""Resolve every Tile, Obj and Back reference a map makes, and say which ones are missing.

    python tools/check_map_resources.py 10001050 10000013

Run it with the repo as the working directory - `CLAUDE.md`'s "the scratchpad shadows the
real tools" applies to this file as much as any other.

WHY THIS EXISTS

On 2026-08-22 the client died with an **access violation reading `[0 + 0x3530]`** 328 ms into
loading map 10001050 (Henesys Park), inside the `0x01A0` handler. A null dereference during a
map load is the shape of a resource the map names and the archive does not have - and this
client is a **trimmed** "Classic World" build, so a map referring to art that was cut is a
real possibility rather than a hypothetical one.

WHAT IT CHECKS, AND WHAT IT CANNOT

It checks that every reference **resolves to a node**:

  * each tile layer's `info/tS` -> `Tile/<tS>.img`, then `<u>/<no>` inside it;
  * each object's `oS` -> `Obj/<oS>.img`, then `<l0>/<l1>/<l2>/<no>`;
  * each background's `bS` -> `Back/<bS>.img`, then `back/<no>` or `ani/<no>`.

It **cannot** tell you that a node which exists is loadable - a canvas whose bitmap is
corrupt resolves fine here and still kills the client. So a clean result is "no missing
reference", never "this map is fine", and the script says so in its own output rather than
leaving the reader to remember it.

POSITIVE CONTROL, RUN EVERY TIME

A deliberately bogus reference is injected and must be reported missing. Without it a bug in
the path walk produces "0 missing" for every map, which is exactly the confident empty answer
`CLAUDE.md` keeps warning about.
"""

import argparse
import json
import os
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
WZ_DUMP = os.path.join(ROOT, "target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
DATA = os.path.join(ROOT, "client-patched", "Data", "Map")

_CACHE = {}


def run(*args):
    out = subprocess.run([WZ_DUMP, *args], capture_output=True)
    if out.returncode != 0:
        raise SystemExit("wz-dump %s failed: %s" % (args, out.stderr.decode("utf-8", "replace")))
    return out.stdout.decode("utf-8", "replace")


def image(archive, name):
    """One image as a dict, cached. `None` if the archive does not carry it."""
    key = (archive, name)
    if key not in _CACHE:
        try:
            _CACHE[key] = json.loads(run("cat", archive, name))
        except SystemExit:
            _CACHE[key] = None
    return _CACHE[key]


def node_at(root, path):
    """Walk `path` (a list of keys) through nested dicts. `None` at the first miss."""
    cur = root
    for k in path:
        if not isinstance(cur, dict) or k not in cur:
            return None
        cur = cur[k]
    return cur


def map_archive(map_id):
    """`Map<N>/Map<N>_000.wz` for a 9-digit id, plus the image name."""
    name = "%09d.img" % map_id
    group = name[0]
    return os.path.join(DATA, "Map", "Map" + group, "Map%s_000.wz" % group), name


def check(map_id, inject_bogus=False):
    archive, img = map_archive(map_id)
    field = image(archive, img)
    if field is None:
        return ["the field image %s is not in %s" % (img, archive)], 0

    tile_wz = os.path.join(DATA, "Tile", "Tile_000.wz")
    obj_wz = os.path.join(DATA, "Obj", "Obj_000.wz")
    back_wz = os.path.join(DATA, "Back", "Back_000.wz")

    missing, checked = [], 0

    def resolve(kind, archive_path, img_name, path, what):
        nonlocal checked
        checked += 1
        node = image(archive_path, img_name)
        if node is None:
            missing.append("%s: image %s is absent (%s)" % (kind, img_name, what))
            return
        if node_at(node, path) is None:
            missing.append("%s: %s has no %s (%s)" % (kind, img_name, "/".join(path), what))

    for layer_key, layer in sorted(field.items()):
        if not layer_key.isdigit() or not isinstance(layer, dict):
            continue
        t_set = (layer.get("info") or {}).get("tS")
        for tk, tile in sorted((layer.get("tile") or {}).items()):
            if not isinstance(tile, dict):
                continue
            u, no = tile.get("u"), tile.get("no")
            if t_set is None or u is None or no is None:
                continue
            resolve("tile", tile_wz, "%s.img" % t_set, [str(u), str(no)],
                    "layer %s tile %s" % (layer_key, tk))
        for ok, obj in sorted((layer.get("obj") or {}).items()):
            if not isinstance(obj, dict):
                continue
            o_set = obj.get("oS")
            parts = [obj.get("l0"), obj.get("l1"), obj.get("l2"), obj.get("no")]
            if o_set is None or any(p is None for p in parts):
                continue
            resolve("obj", obj_wz, "%s.img" % o_set, [str(p) for p in parts],
                    "layer %s obj %s" % (layer_key, ok))

    for bk, back in sorted((field.get("back") or {}).items()):
        if not isinstance(back, dict):
            continue
        b_set, no, ani = back.get("bS"), back.get("no"), back.get("ani")
        if not b_set or no is None:
            continue  # an empty bS is a blank layer, not a reference
        group = "ani" if str(ani) == "1" else "back"
        resolve("back", back_wz, "%s.img" % b_set, [group, str(no)], "back %s" % bk)

    if inject_bogus:
        resolve("CONTROL", tile_wz, "tile_thisDoesNotExist.img", ["0", "0"],
                "deliberately bogus - MUST be reported")

    return missing, checked


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("maps", nargs="+", type=int)
    args = ap.parse_args()
    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)

    control, _ = check(args.maps[0], inject_bogus=True)
    if not any(m.startswith("CONTROL") for m in control):
        raise SystemExit("POSITIVE CONTROL FAILED: a bogus reference was not reported. "
                         "The path walk is broken and every result below would read clean.")
    print("positive control: a bogus tile reference IS reported. The walk can say no.\n")

    for map_id in args.maps:
        missing, checked = check(map_id)
        print("map %-10d %4d references checked, %d missing" % (map_id, checked, len(missing)))
        for m in missing:
            print("    MISSING  %s" % m)
    print("\nA clean result means NO MISSING REFERENCE. It does not mean the map loads: a node"
          "\nthat exists can still carry art the client cannot decode, and this script cannot"
          "\nsee that.")


if __name__ == "__main__":
    main()
