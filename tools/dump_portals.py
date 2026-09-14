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

# NPC names live in String.wz, not Map.wz. The `life` node carries a template id and nothing
# a human can read, so the two have to be joined here - the same archive and the same node
# `tools/dump_npcstrings.py` reads, deliberately, so there is one source for an NPC's name.
STRING_ARCHIVE = os.path.join("client-patched", "Data", "String", "String_000.wz")


def run(*args):
    # Explicit UTF-8: Windows would otherwise decode wz-dump's output with the ANSI
    # code page, and one map image carries bytes cp1252 has no mapping for.
    r = subprocess.run([WZ_DUMP, *args], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        raise SystemExit("wz-dump %s failed (rc=%s): %s"
                         % (" ".join(args), r.returncode, (r.stderr or "").strip()[:200]))
    return r.stdout


def string_names(img):
    """`{template: name}` from String.wz/<img>.

    Returns an empty dict rather than raising if the archive cannot be read: the names are a
    convenience column, and the rest of this dump - portals, footholds, fields, mobs - is
    load-bearing for the server and must still be written. **The caller reports the count**,
    because a name table that silently came back empty is the kind of quiet instrument failure
    this repo keeps paying for: "npcs.txt has no names in it" is only obvious to someone who
    already knew to expect them.
    """
    try:
        out = run("cat", STRING_ARCHIVE, img)
    except SystemExit as e:
        print("WARNING: could not read %s/%s (%s) - names will be missing"
              % (STRING_ARCHIVE, img, e), file=sys.stderr)
        return {}
    names = {}
    for key, entry in json.loads(out).items():
        if not str(key).isdigit() or not isinstance(entry, dict):
            continue
        name = entry.get("name")
        if isinstance(name, str) and name.strip():
            # Commas are fine - the name is the LAST column, so a comma inside it cannot
            # shift a numeric field. Newlines are not: they would split one row into two.
            names[int(key)] = " ".join(name.split())
    return names


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


# client-patched/Data/Reactor - two levels up from Map/Map.
REACTOR_ROOT = os.path.join(os.path.dirname(os.path.dirname(MAP_ROOT)), "Reactor")


def reactor_state_count(template):
    """How many states of `Reactor.wz/<id>.img` carry an `event` - the hits it takes to break.

    The Wooden Box (0000001) has events on states 0..3 and none on 4, so 4. A reactor whose
    image cannot be read counts as 1 so a single hit breaks it rather than nothing happening.
    """
    archive = os.path.join(REACTOR_ROOT, "Reactor_000.wz")
    try:
        node = json.loads(run("cat", archive, "%07d.img" % template))
    except SystemExit:
        return 1
    n = sum(1 for k, v in node.items() if k.isdigit() and isinstance(v, dict) and "event" in v)
    return n or 1


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

    rows, npcs, mobs, field_ids, fields, spawns = [], [], [], [], 0, 0
    reactors = []
    reactor_states = {}
    scripted = 0
    footholds = []
    clocks = []
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
            field_ids.append(map_id)
            # **The wall clock.** A map that draws one declares it as a top-level `clock`
            # node - a placement, `{x, y, width, height}`, and nothing else. The client
            # builds the widget from this at field entry. The server needs the LIST because
            # the packet that sets the time (0x01BC) throws in the client on a map that
            # built no widget, so it may only be sent where this node exists.
            # world::config::Config::clocks.
            clock = node.get("clock")
            if isinstance(clock, dict):
                try:
                    clocks.append((map_id, int(clock.get("x", 0)), int(clock.get("y", 0)),
                                   int(clock.get("width", 0)), int(clock.get("height", 0))))
                except (TypeError, ValueError):
                    clocks.append((map_id, 0, 0, 0, 0))
            # life: type "n" is an NPC, "m" a mob. Mobs are not sent yet.
            for _, l in sorted((node.get("life") or {}).items(),
                               key=lambda kv: int(kv[0]) if kv[0].isdigit() else 0):
                kind = l.get("type")
                if kind not in ("n", "m"):
                    continue
                try:
                    template = int(str(l.get("id", "")).lstrip("0") or "0")
                except ValueError:
                    continue
                if not template:
                    continue
                # `mobTime` is the WZ's own respawn delay in SECONDS for this spawn point,
                # and it is the server's business: the client never respawns a mob, it only
                # renders the ones it is sent. Absent means "the field's ordinary rate", not
                # "never" - a huge number of ordinary spawn points have no mobTime at all,
                # and treating those as permanent deaths empties a map after one pass.
                # -1 in the WZ means the spawn is not respawned automatically.
                row = (map_id, template, int(l.get("x", 0)), int(l.get("cy", l.get("y", 0))),
                       int(l.get("fh", 0)), int(l.get("rx0", 0)), int(l.get("rx1", 0)),
                       int(l.get("f", 0)))
                if kind == "m":
                    row = row + (int(l.get("mobTime", 0)),)
                (npcs if kind == "n" else mobs).append(row)

            # Reactors: the breakable boxes. `reactor/<n>` with `id` (a Reactor.wz image),
            # `x`, `y`, `reactorTime` (seconds to respawn; absent means the field's own
            # default, taken as 120 - the value all but three placements carry), `f` and
            # `name`. The server spawns them (the client only preloads the art), so this is
            # the table it spawns from. The owner, 2026-09-13, Pio's "Collecting Recycled Goods":
            # the quest items come out of Wooden Boxes nobody was placing.
            for key, r in sorted((node.get("reactor") or {}).items(),
                                 key=lambda kv: int(kv[0]) if kv[0].isdigit() else 0):
                try:
                    template = int(str(r.get("id", "")).lstrip("0") or "0")
                except ValueError:
                    continue
                if not template or not key.isdigit():
                    continue
                if template not in reactor_states:
                    reactor_states[template] = reactor_state_count(template)
                reactors.append((map_id, int(key), template, int(r.get("x", 0)), int(r.get("y", 0)),
                                 int(r.get("reactorTime", 120) or 120), int(r.get("f", 0)),
                                 reactor_states[template], str(r.get("name", "") or "")))

            # Footholds: the map's floor geometry, three levels deep as
            # foothold/<layer>/<group>/<id>. The server needs it so a dropped item can be
            # put somewhere a player can actually stand - without it a drop staggered off
            # the edge of a ledge lands inside the terrain and cannot be picked up.
            for layer_name, layer in sorted((node.get("foothold") or {}).items()):
                if not isinstance(layer, dict):
                    continue
                for group_name, group in sorted(layer.items()):
                    if not isinstance(group, dict):
                        continue
                    for fid, f in sorted(group.items(),
                                         key=lambda kv: int(kv[0]) if kv[0].isdigit() else 0):
                        if not isinstance(f, dict) or "x1" not in f:
                            continue
                        try:
                            footholds.append((
                                map_id, int(fid),
                                int(f.get("x1", 0)), int(f.get("y1", 0)),
                                int(f.get("x2", 0)), int(f.get("y2", 0)),
                                int(f.get("prev", 0)), int(f.get("next", 0)),
                            ))
                        except (TypeError, ValueError):
                            continue

            for key, p in sorted((node.get("portal") or {}).items(),
                                 key=lambda kv: int(kv[0]) if kv[0].isdigit() else 0):
                if not isinstance(p, dict):
                    continue
                idx = int(key) if str(key).isdigit() else -1
                name, target = p.get("pn", ""), p.get("tm")
                if not name or idx < 0:
                    continue
                # **A SCRIPT portal is not a spawn point, and it used to be written as one.**
                #
                # The owner, 2026-09-09: "Ellinia should also have a portal to go to Ellinia
                # Station right here, but this portal does not exist where I expect it." It
                # does exist. This dumper threw away the one field that says where it goes.
                #
                # 10002000 portal 38 is {"pn": "in03", "tm": 999999999, "script":
                # "pt_10002000_in03"} - the client resolves it by running a named script
                # rather than by following `tm`. Reading only `pn` and `tm` made it
                # indistinguishable from a spawn point, so the server saw a portal that leads
                # nowhere and did nothing when they stood on it.
                #
                # 41 portals of 3679 carry a script, across 9 distinct names. Enumerated
                # rather than sampled, so the column's cost is known: empty on 3638 rows.
                #
                # Spawn points are still emitted with target 0. The server needs them: an
                # arriving character is placed at the portal NAMED by the source portal's
                # `tn`, and that lookup is by (map, name) -> index regardless of whether the
                # destination portal itself leads anywhere.
                script = p.get("script") or ""
                # Where the portal IS. An arriving character stands here, and the field
                # announces them here to everyone already on the map - until 2026-09-14 that
                # announcement carried the map origin and the newcomer visibly snapped from
                # (0, 0) to the portal on their first step. world::session::field::go_to_map.
                px, py = int(p.get("x", 0)), int(p.get("y", 0))
                if target is None or target == NO_TARGET:
                    spawns += 1
                    if script:
                        scripted += 1
                    rows.append((map_id, idx, name, 0, "", script, px, py))
                    continue
                rows.append((map_id, idx, name, int(target), p.get("tn", "") or "", script, px, py))

    os.makedirs(args.out_dir, exist_ok=True)
    path = os.path.join(args.out_dir, "portals.txt")
    with open(path, "w", encoding="utf-8") as fh:
        fh.write("# map, index, portal, target map, target portal, script, x, y - generated by "
                 "tools/dump_portals.py from the client's Map.wz\n")
        fh.write("# A row with target map 0 and NO script is a spawn point.\n")
        fh.write("# A row with target map 0 and a SCRIPT is a script portal: the client\n")
        fh.write("# resolves it by running that named script rather than by following the\n")
        fh.write("# target, so the server has to know the destination. world::scriptportals.\n")
        for r in rows:
            fh.write("%d, %d, %s, %d, %s, %s, %d, %d\n" % r)
    print("%s: %d portals across %d fields (%d spawn points, kept so an arrival portal can "
          "be resolved by name; %d of those are SCRIPT portals, which DO lead somewhere)"
          % (path, len(rows), fields, spawns, scripted))

    hpath = os.path.join(args.out_dir, "footholds.txt")
    with open(hpath, "w", encoding="utf-8") as fh:
        fh.write("# map, id, x1, y1, x2, y2, prev, next - generated by "
                 "tools/dump_portals.py from the client's Map.wz `foothold` nodes.\n")
        fh.write("# A foothold is one floor SEGMENT between (x1,y1) and (x2,y2). A wall is\n")
        fh.write("# a segment with x1 == x2; those are somewhere to bump into rather than\n")
        fh.write("# somewhere to stand, and the server skips them when placing a drop.\n")
        for r in footholds:
            fh.write("%d, %d, %d, %d, %d, %d, %d, %d\n" % r)
    print("%s: %d footholds across %d maps"
          % (hpath, len(footholds), len({f[0] for f in footholds})))

    npath = os.path.join(args.out_dir, "npcs.txt")
    names = string_names("Npc.img")
    named = 0
    with open(npath, "w", encoding="utf-8") as fh:
        # The old header claimed a `mobTime` column this writer has never emitted -
        # NPCs do not respawn and only the mob rows carry it. Corrected rather than
        # left as a table row nobody counted.
        fh.write("# map, template, x, cy, fh, rx0, rx1, f, name - generated by "
                 "tools/dump_portals.py from Map.wz `life` nodes + String.wz/Npc.img\n")
        # `name` is LAST because it is the only free-text field: NPC names contain
        # commas, so any column after it would be unparseable. The server reads the
        # eight numeric columns and ignores the rest - config.rs `load_npcs`.
        fh.write("# a row with no name means String.wz had no entry for that template\n")
        for r in npcs:
            name = names.get(r[1], "")
            if name:
                named += 1
            fh.write("%d, %d, %d, %d, %d, %d, %d, %d, %s\n" % (r + (name,)))
    distinct = {n[1] for n in npcs}
    print("%s: %d NPCs across %d maps, %d rows named (%d of %d distinct templates)"
          % (npath, len(npcs), len({n[0] for n in npcs}), named,
             len(distinct & set(names)), len(distinct)))
    if npcs and not named:
        print("WARNING: not one NPC resolved a name - check %s" % STRING_ARCHIVE,
              file=sys.stderr)

    # Every map that actually has a field image. This is the authoritative "does this map
    # exist" list and it is NOT the same as String.wz's name table: a survey of this client
    # found 12 ids named but absent and 6 present but unnamed. Sending a character to an id
    # with no field image strands it, and one with no name entry can take the client down a
    # branch that does not return (research/map1-exists.md).
    fpath = os.path.join(args.out_dir, "fields.txt")
    with open(fpath, "w", encoding="utf-8") as fh:
        fh.write("# every map id with a field image in Map.wz - generated by "
                 "tools/dump_portals.py\n")
        for m in sorted(field_ids):
            fh.write("%d\n" % m)
    print("%s: %d fields" % (fpath, len(field_ids)))

    cpath = os.path.join(args.out_dir, "clocks.txt")
    with open(cpath, "w", encoding="utf-8") as fh:
        fh.write("# map, x, y, width, height - generated by tools/dump_portals.py from the "
                 "top-level `clock` node of each Map.wz image.\n")
        fh.write("# Only maps that DECLARE a clock are listed. The server sends 0x01BC (the\n")
        fh.write("# field clock) on entry to these and to no others: the client's set-time\n")
        fh.write("# path throws when the map built no widget. world::config::Config::clocks.\n")
        for r in sorted(clocks):
            fh.write("%d, %d, %d, %d, %d\n" % r)
    print("%s: %d maps declare a wall clock" % (cpath, len(clocks)))

    rpath = os.path.join(args.out_dir, "reactors.txt")
    with open(rpath, "w", encoding="utf-8") as fh:
        fh.write("# map, index, reactorId, x, y, reactorTime, f, breakAt, name - generated by "
                 "tools/dump_portals.py from Map.wz `reactor` nodes + Reactor.wz\n")
        fh.write("# breakAt is the number of states with an `event` in Reactor.wz/<id>.img: hits "
                 "0..breakAt-1 advance the state, and state breakAt is the broken one. "
                 "reactorTime is seconds to respawn. world::config::Config::load_reactors\n")
        for r in reactors:
            fh.write("%d, %d, %d, %d, %d, %d, %d, %d, %s\n" % r)
    print("%s: %d reactor placements across %d maps, %d distinct reactors"
          % (rpath, len(reactors), len({r[0] for r in reactors}), len(reactor_states)))

    mpath = os.path.join(args.out_dir, "mobs.txt")
    mob_names = string_names("Mob.img")
    mob_named = 0
    with open(mpath, "w", encoding="utf-8") as fh:
        fh.write("# map, template, x, cy, fh, rx0, rx1, f, mobTime, name - generated by "
                 "tools/dump_portals.py from Map.wz `life` nodes (type m) + String.wz/Mob.img\n")
        # `name` is LAST for the same reason it is in npcs.txt: it is the only free-text
        # field, so a comma inside it cannot shift a numeric column. `load_mobs` reads
        # fixed indices 0-4 and 8, so the extra column is invisible to the server.
        fh.write("# a row with no name means String.wz had no entry for that template\n")
        for r in mobs:
            name = mob_names.get(r[1], "")
            if name:
                mob_named += 1
            fh.write("%d, %d, %d, %d, %d, %d, %d, %d, %d, %s\n" % (r + (name,)))
    mob_distinct = {m[1] for m in mobs}
    print("%s: %d mob spawns across %d maps, %d rows named (%d of %d distinct templates)"
          % (mpath, len(mobs), len({m[0] for m in mobs}), mob_named,
             len(mob_distinct & set(mob_names)), len(mob_distinct)))
    if mobs and not mob_named:
        print("WARNING: not one mob resolved a name - check %s" % STRING_ARCHIVE,
              file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
