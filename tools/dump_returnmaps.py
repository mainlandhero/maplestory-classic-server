#!/usr/bin/env python3
"""Every field's `returnMap`, `forcedReturn` and `town` flag, out of the client's own Map.wz.

Writes `gm-handbook/returnmaps.txt`. Generated game data - gitignored, never hand-edited,
same contract as `tools/dump_portals.py`, `tools/dump_mobs.py` and `tools/dump_names.py`.

**Why the server needs it.** A revive has to warp the character somewhere, and "the nearest
town" is not the server's to invent - it is one key per field image:

    info/returnMap      the field's nearest-town anchor. Present on all 426 fields, never
                        the sentinel, a town on 388 of them, and NOT reachable by any
                        portal from its own field on 316 of the 401 fields that have a
                        door - so it is a teleport target, not a map exit. That much is
                        read. That it is specifically the DEATH destination is derived,
                        not read: the WZ never names the death event. The argument is in
                        `research/return-maps.md` sec. 2 and it rests on
                        `Return Scroll - Nearest Town` (2030000) carrying 999999999 in
                        `spec/moveTo` where every other return scroll carries a town id.
    info/forcedReturn   an EJECT target for a field a character must not be left standing
                        in - a mid-flight cabin, a PQ stage, a timed depot, an instanced
                        dungeon. 999999999 means "none", which is 354 of 426 fields. NOT
                        the revive destination: absent on 354 fields where a death still
                        has to go somewhere, and where present its target is a non-town
                        54 times out of 72.
    info/town           1 on a town field, 0 otherwise. Only those two values occur. The
                        flag is coarser than "town square" - shop interiors carry it too.

`research/return-maps.md` is the write-up: what each field means, the counts, and the
resolve rule the server should use.

**The output is TAB separated, not comma separated**, unlike `portals.txt` and its
siblings. That is deliberate: this file carries map NAMES, and one of them is
`The Resting Spot, Pig Park`. A comma-separated row would split it in the wrong place.

    python tools/dump_returnmaps.py
    python tools/dump_returnmaps.py --out-dir tmp
"""
import argparse
import json
import os
import subprocess
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

MAP_ROOT = os.path.join("client-patched", "Data", "Map", "Map")
STRING_WZ = os.path.join("client-patched", "Data", "String", "String_000.wz")
WZ_DUMP = os.path.abspath(
    os.path.join("target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
)

# The client's own "no target" sentinel. Same number the portal table uses for `tm`, and the
# same number `Return Scroll - Nearest Town` (2030000) carries in `spec/moveTo`. In every one
# of those three places it means "there is no literal map id here" - for a portal, "not a
# door"; for the scroll, "resolve it from the field I am standing on"; for `forcedReturn`,
# "this field does not eject anybody".
NO_TARGET = 999999999

# How far the transitive walk will follow `returnMap` before calling it a loop. The deepest
# real chain in this client is TWO hops (Grave of Mushmom -> Ant Tunnel Park -> Sleepywood),
# so this is slack, not a tuning knob. A revive that loops forever is a hang; this is what
# stops it.
MAX_HOPS = 8

# Keys this reader must be able to find before any absence it reports is worth anything.
# `version`, `bgm` and `mapMark` are on all 426 field images; `fieldType` is on 379 of them,
# which makes it the useful one - a control that is present on SOME maps proves the reader
# discriminates rather than just returning a constant.
#
# NOTE: `VRLimit` is NOT a key in this client. The viewport bound is four separate keys,
# `VRTop`/`VRBottom`/`VRLeft`/`VRRight`. Using `VRLimit` as a positive control would have
# been a control that could only ever fail - measured, 0 of 426.
CONTROLS = {"version": 426, "bgm": 426, "mapMark": 426, "fieldType": 379}
NEGATIVE_CONTROL = "returnMapZZ"   # must be 0 of 426, or the reader is matching loosely


def run(*args):
    # Explicit UTF-8: Windows would otherwise decode wz-dump's output with the ANSI code
    # page, and one map image carries bytes cp1252 has no mapping for.
    r = subprocess.run([WZ_DUMP, *args], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        return None
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
    for line in (run("tree", archive, "1") or "").splitlines():
        if "[IMG]" in line:
            yield line.split("[IMG]", 1)[1].split()[0]


def map_names():
    """id -> name from String.wz's Map.img. Best effort: an empty table is not fatal."""
    out = {}
    if not os.path.exists(STRING_WZ):
        return out
    txt = run("cat", STRING_WZ, "Map.img")
    if not txt:
        return out
    try:
        root = json.loads(txt)
    except ValueError:
        return out

    def walk(node):
        if not isinstance(node, dict):
            return
        for key, value in node.items():
            if not isinstance(value, dict):
                continue
            if key.isdigit() and isinstance(value.get("mapName"), str):
                out[int(key)] = (value["mapName"].replace("\t", " ")
                                 .replace("\n", " ").replace("\r", " ").strip())
                continue
            walk(value)

    walk(root)
    return out


def revive_map(map_id, info):
    """Where a character who died on `map_id` should come back. Returns (field, hops).

    **The first hop is unconditional.** Not "stop if this field is already a town" - 94 of
    the 115 `town == 1` fields have a `returnMap` pointing somewhere ELSE, because a town's
    shop interiors are themselves flagged `town == 1`. `Southperry Armor Store` (61) is a
    town by the flag and its `returnMap` is `Southperry` (60). A resolver that stops on the
    flag revives the player inside the shop; one that always takes the first hop puts them in
    the town square, which is what the field is for.

    After that first hop it keeps following `returnMap` only while the field it landed on is
    NOT a town - which is what turns `The Grave of Mushmom -> Ant Tunnel Park` into
    `-> Sleepywood`.

    **It never returns "nothing".** A walk that loops or runs out of hops returns the last
    real field it stood on, so a party-quest stage resolves to stage 1 rather than to a
    fallback town half a world away. `(0, -1)` comes back only if `returnMap` names a field
    that is not in this client at all, which is zero fields today - the guard is there
    because a revive with no destination is a hang.
    """
    cur = info.get(map_id)
    if cur is None:
        return 0, -1
    nxt = cur.get("returnMap")
    if nxt is None or nxt == NO_TARGET or nxt not in info:
        return 0, -1
    dest, seen, hops = nxt, {map_id}, 1
    while hops <= MAX_HOPS:
        if info[dest].get("town") == 1:
            return dest, hops
        step = info[dest].get("returnMap")
        if step is None or step == NO_TARGET or step == dest or step in seen \
                or step not in info:
            return dest, hops          # last real field - not a town, but somewhere real
        seen.add(dest)
        dest = step
        hops += 1
    return dest, hops


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-dir", default="gm-handbook")
    args = ap.parse_args()

    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)
    if not os.path.isdir(MAP_ROOT):
        raise SystemExit("%s is missing" % MAP_ROOT)

    info, unreadable, no_info, seen_keys = {}, [], [], {}
    for archive in archives():
        for image in images(archive):
            stem = image[:-4] if image.endswith(".img") else image
            if not stem.isdigit():
                continue
            txt = run("cat", archive, image)
            if txt is None:
                unreadable.append(image)
                continue
            try:
                node = json.loads(txt)
            except ValueError:
                unreadable.append(image)
                continue
            block = node.get("info")
            if not isinstance(block, dict):
                no_info.append(int(stem))
                block = {}
            info[int(stem)] = block
            for k in block:
                seen_keys[k] = seen_keys.get(k, 0) + 1

    total = len(info)
    if not total:
        raise SystemExit("no field images read at all - the reader is broken, not the data")

    # --- verify the instrument before believing anything it says is missing ---------------
    print("field images read: %d (unreadable %d, no info node %d)"
          % (total, len(unreadable), len(no_info)))
    print("positive controls:")
    broken = []
    for key, expect in CONTROLS.items():
        got = seen_keys.get(key, 0)
        flag = "ok" if got else "BLIND"
        if not got:
            broken.append(key)
        print("   %-12s %3d/%d  (expected about %d)  %s" % (key, got, total, expect, flag))
    neg = seen_keys.get(NEGATIVE_CONTROL, 0)
    print("   %-12s %3d/%d  (must be 0)  %s"
          % (NEGATIVE_CONTROL, neg, total, "ok" if neg == 0 else "MATCHING TOO LOOSELY"))
    if broken or neg:
        raise SystemExit("control failed on %s - do not trust this run"
                         % (broken or [NEGATIVE_CONTROL]))

    names = map_names()

    rows, missing_rm, missing_fr, missing_town = [], [], [], []
    for map_id in sorted(info):
        block = info[map_id]
        if "returnMap" not in block:
            missing_rm.append(map_id)
        if "forcedReturn" not in block:
            missing_fr.append(map_id)
        if "town" not in block:
            missing_town.append(map_id)
        return_map = block.get("returnMap", NO_TARGET)
        forced = block.get("forcedReturn", NO_TARGET)
        town = block.get("town", 0)
        try:
            return_map, forced, town = int(return_map), int(forced), int(town)
        except (TypeError, ValueError):
            return_map, forced, town = NO_TARGET, NO_TARGET, 0
        rows.append([map_id, return_map, forced, town, 0, -1,
                     names.get(map_id, "")])

    for row in rows:
        row[4], row[5] = revive_map(row[0], info)

    os.makedirs(args.out_dir, exist_ok=True)
    path = os.path.join(args.out_dir, "returnmaps.txt")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# map\treturnMap\tforcedReturn\ttown\treviveMap\thops\tname\n")
        fh.write("# generated by tools/dump_returnmaps.py from the client's own Map.wz\n")
        fh.write("# info nodes. TAB separated because map names contain commas.\n")
        fh.write("#\n")
        fh.write("# returnMap    READ. The field's nearest-town anchor. On every field,\n")
        fh.write("#              never %d, a town on 388 of 426. That it is the\n" % NO_TARGET)
        fh.write("#              DEATH destination is derived, not read - see\n")
        fh.write("#              research/return-maps.md sec. 2.\n")
        fh.write("# forcedReturn eject target for a field nobody may be left standing in.\n")
        fh.write("#              %d means THERE IS NONE. Not a map id - do not warp to it.\n"
                 % NO_TARGET)
        fh.write("# town         1 if this field is a town, 0 otherwise. No other value\n")
        fh.write("#              occurs in this client.\n")
        fh.write("# reviveMap    DERIVED BY THIS SCRIPT, not read from the WZ. One\n")
        fh.write("#              returnMap hop UNCONDITIONALLY (a shop interior is flagged\n")
        fh.write("#              town==1 and its returnMap is the town square), then keep\n")
        fh.write("#              hopping while the field is not a town, capped at %d.\n"
                 % MAX_HOPS)
        fh.write("#              Always a real field id: a walk that loops stops on the\n")
        fh.write("#              last real field rather than giving up, so a party-quest\n")
        fh.write("#              stage lands on stage 1. 0 with hops -1 would mean the\n")
        fh.write("#              returnMap names a field this client does not have - it\n")
        fh.write("#              happens on no field today, and the server still needs the\n")
        fh.write("#              branch, because a revive with no destination is a hang.\n")
        fh.write("# hops         how many returnMap steps reviveMap took. Always >= 1.\n")
        for r in rows:
            fh.write("%d\t%d\t%d\t%d\t%d\t%d\t%s\n" % tuple(r))

    town_ids = {x[0] for x in rows if x[3] == 1}
    towns = len(town_ids)
    rm_sentinel = sum(1 for r in rows if r[1] == NO_TARGET)
    fr_sentinel = sum(1 for r in rows if r[2] == NO_TARGET)
    rm_is_town = sum(1 for r in rows if r[1] in town_ids)
    unresolved = [r for r in rows if r[5] < 0]
    not_a_town = [r for r in rows if r[5] > 0 and r[4] not in town_ids]
    hops = {}
    for r in rows:
        hops[r[5]] = hops.get(r[5], 0) + 1

    print()
    print("%s: %d fields" % (path, len(rows)))
    print("  named by String.wz            : %d" % sum(1 for r in rows if r[6]))
    print("  town == 1                     : %d" % towns)
    print("  returnMap missing             : %d %s" % (len(missing_rm), missing_rm[:8]))
    print("  returnMap == %d        : %d   <- the sentinel NEVER appears here"
          % (NO_TARGET, rm_sentinel))
    print("  returnMap points at a town    : %d of %d" % (rm_is_town, len(rows)))
    print("  forcedReturn missing          : %d %s" % (len(missing_fr), missing_fr[:8]))
    print("  forcedReturn == %d     : %d   <- 'this field ejects nobody'"
          % (NO_TARGET, fr_sentinel))
    print("  forcedReturn is a real map id : %d" % (len(rows) - fr_sentinel))
    print("  town key missing              : %d %s" % (len(missing_town), missing_town[:8]))
    print("  reviveMap hops                : %s"
          % ", ".join("%s:%d" % ("no destination" if k < 0 else k, v)
                      for k, v in sorted(hops.items())))
    print("  reviveMap lands in a town     : %d of %d" % (len(rows) - len(not_a_town)
                                                          - len(unresolved), len(rows)))
    if not_a_town:
        print("  %d fields revive somewhere REAL that is not flagged a town - the walk\n"
              "  stopped on a self-loop. Every one is a party quest, an event stage or a\n"
              "  test field:" % len(not_a_town))
        for r in not_a_town:
            print("      %-10d -> %-10d %s" % (r[0], r[4], r[6] or "(unnamed)"))
    if unresolved:
        print("  %d fields have NO destination at all - the server MUST have a fallback:"
              % len(unresolved))
        for r in unresolved:
            print("      %-10d %s" % (r[0], r[6] or "(unnamed)"))
    if unreadable:
        print("  (%d images could not be read)" % len(unreadable), file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
