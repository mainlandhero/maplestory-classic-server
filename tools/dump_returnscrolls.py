#!/usr/bin/env python3
"""Every Return Scroll and where it goes, plus the client's own continent partition.

Writes `gm-handbook/returnscrolls.txt`. Generated game data - gitignored, never hand-edited,
same contract as `tools/dump_returnmaps.py` and `tools/dump_itemdata.py`.

**This file exists to be checked against, not only to be read.** `crates/world/src/returnscroll.rs`
bakes the ten scroll rows and a three-band continent function into Rust source, because ten
rows plumbed through a runtime table would need a `Config` field and this feature does not
need one. A baked table is a claim; the test `the_baked_table_still_matches_the_client`
re-reads this file and fails when they drift. Regenerate it and run the world crate's tests
after any client data change.

## What is read, and from where

    Item/<cat>/*_000.wz   <item>/spec/moveTo   the scroll's destination map id.
                                               999999999 - the client's "no literal map id
                                               here" sentinel - means "the nearest town,
                                               resolved from the field I am standing on".
                                               Measured: 10 items in the WHOLE item tree
                                               carry this key, all of them 0203xxxx.
    String/String_000.wz  Consume.img          the scroll's name.
    String/String_000.wz  Map.img              the CONTINENT partition. Its six top-level
                                               children are MapleIsland, VictoriaIsland,
                                               Ossyria, Other, Event and Dev, and every
                                               named map hangs under exactly one of them.
                                               This is the client's own answer to "what is
                                               a continent"; nothing here is inferred from
                                               the shape of the id.

## Why the item scan is the whole item tree and not `0203.img`

`research/return-maps.md` sec. 1 named four return scrolls by reading one image. Enumerating
before filtering is this project's standing rule, so this walks Consume, Etc, Install, Cash,
Special and Pet - 1025 items - and reports how many carry `spec/moveTo`. If a future client
puts a warp item outside `0203`, this finds it and the count in the header changes.

## The controls

Refuses to write unless the item reader can see `spec/hp` and `info/price` (it would
otherwise report "no moveTo anywhere" from a reader that reads nothing), unless the
negative control `spec/moveToZZ` comes back zero (a reader matching loosely), and unless
`Map.img` yields more than one group name (a walker that flattened the categories would
silently destroy the whole continent partition and still print a table).

    python tools/dump_returnscrolls.py
    python tools/dump_returnscrolls.py --out-dir tmp
"""
import argparse
import json
import os
import subprocess
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

DATA_ROOT = os.path.join("client-patched", "Data")
STRING_WZ = os.path.join(DATA_ROOT, "String", "String_000.wz")
WZ_DUMP = os.path.abspath(
    os.path.join("target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
)

ITEM_CATEGORIES = ["Consume", "Etc", "Install", "Cash", "Special", "Pet"]

# The client's own "there is no literal map id here" sentinel. The same number
# `portal/<n>/tm` uses for a spawn point and `info/forcedReturn` uses for "this field ejects
# nobody" - see `research/return-maps.md` sec. 1. On a scroll it means "resolve the
# destination from the field the character is standing on".
NO_MAP = 999999999

# String.wz/Map.img's six top-level children, in the order the archive lists them. Only the
# first three are places a character can stand and travel between; `Other` is the Free
# Market, the job-advancement dungeons and the party quest, `Event` is event stages and
# `Dev` is the three test maps.
CONTINENTS = ["MapleIsland", "VictoriaIsland", "Ossyria", "Other", "Event", "Dev"]


def run(*args):
    # Explicit UTF-8: Windows would otherwise decode wz-dump's output with the ANSI code
    # page, and some images carry bytes cp1252 has no mapping for.
    r = subprocess.run([WZ_DUMP, *args], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        return None
    return r.stdout


def images(archive):
    for line in (run("tree", archive, "1") or "").splitlines():
        if "[IMG]" in line:
            yield line.split("[IMG]", 1)[1].split()[0]


def item_archives():
    out = []
    for cat in ITEM_CATEGORIES:
        d = os.path.join(DATA_ROOT, "Item", cat)
        if not os.path.isdir(d):
            continue
        for f in sorted(os.listdir(d)):
            if f.endswith("_000.wz"):
                out.append(os.path.join(d, f))
    return out


def scalars(node):
    """`info` and `spec` scalars of one item node, prefixed so the namespaces cannot merge.

    `info/time` and `spec/time` are different properties; flattening them lets one win
    silently. Same prefix trick `tools/dump_itemdata.py` uses.
    """
    out = {}
    for section in ("info", "spec"):
        blk = node.get(section)
        if isinstance(blk, dict):
            for k, v in blk.items():
                if not isinstance(v, (dict, list)):
                    out[section + "/" + str(k).strip()] = v
    return out


def walk_items(node, out, keys, root_id=None):
    """Emit id -> scalars for every item in `node`. Two archive shapes, one walker."""
    if not isinstance(node, dict):
        return
    if root_id is not None and ("info" in node or "spec" in node):
        s = scalars(node)
        out[root_id] = s
        for k in s:
            keys[k] = keys.get(k, 0) + 1
        return
    for key, value in node.items():
        if not isinstance(value, dict):
            continue
        if key.isdigit() and ("info" in value or "spec" in value):
            s = scalars(value)
            out[int(key)] = s
            for k in s:
                keys[k] = keys.get(k, 0) + 1
            continue
        walk_items(value, out, keys)


def item_names():
    """id -> name from String.wz's Consume.img. Best effort; an empty table is not fatal."""
    out = {}
    txt = run("cat", STRING_WZ, "Consume.img")
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
            if key.isdigit() and isinstance(value.get("name"), str):
                out[int(key)] = (value["name"].replace("\t", " ")
                                 .replace("\n", " ").replace("\r", " ").strip())
                continue
            walk(value)

    walk(root)
    return out


def map_groups():
    """(mapId -> group, mapId -> name) from String.wz's Map.img.

    The group is the IMAGE'S OWN top-level child name. Nothing here is derived from the
    numeric shape of the id - that is the whole point of reading this image.
    """
    groups, names = {}, {}
    txt = run("cat", STRING_WZ, "Map.img")
    if not txt:
        return groups, names
    try:
        root = json.loads(txt)
    except ValueError:
        return groups, names

    def walk(node, group):
        if not isinstance(node, dict):
            return
        for key, value in node.items():
            if not isinstance(value, dict):
                continue
            if key.isdigit() and isinstance(value.get("mapName"), str):
                groups[int(key)] = group
                names[int(key)] = (value["mapName"].replace("\t", " ")
                                   .replace("\n", " ").replace("\r", " ").strip())
                continue
            walk(value, group)

    for group, node in root.items():
        walk(node, group)
    return groups, names


# --- the band rule crates/world/src/returnscroll.rs bakes -------------------------------
#
# Not a guess at what the ids mean: it is fitted to this client's own partition above and
# then scored against every named map. The script prints the score, and it must be 0
# disagreements or the Rust constant is wrong.
def band(map_id):
    if map_id < 1000000:
        return "MapleIsland"
    if 10000000 <= map_id <= 19999999:
        return "VictoriaIsland"
    if 20000000 <= map_id <= 29999999:
        return "Ossyria"
    return None          # Other / Event / Dev - not a continent a character travels between


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-dir", default="gm-handbook")
    args = ap.parse_args()

    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)

    # --- items -------------------------------------------------------------------------
    items, keys = {}, {}
    for archive in item_archives():
        for image in images(archive):
            txt = run("cat", archive, image)
            if not txt:
                print("unreadable: %s %s" % (archive, image), file=sys.stderr)
                continue
            try:
                node = json.loads(txt)
            except ValueError:
                print("bad json: %s %s" % (archive, image), file=sys.stderr)
                continue
            stem = image[:-4] if image.endswith(".img") else image
            walk_items(node, items, keys,
                       root_id=int(stem) if stem.isdigit() and len(stem) > 4 else None)

    print("items scanned: %d" % len(items))
    print("item reader controls:")
    for key, want in (("spec/hp", "positive"), ("info/price", "positive"),
                      ("spec/moveTo", "the subject"), ("spec/moveToZZ", "must be 0")):
        print("   %-16s %5d   %s" % (key, keys.get(key, 0), want))
    if not keys.get("spec/hp") or not keys.get("info/price"):
        raise SystemExit("the item reader found no spec/hp or no info/price - it is blind, "
                         "and an empty moveTo result would mean nothing")
    if keys.get("spec/moveToZZ"):
        raise SystemExit("the negative control matched - the reader is matching loosely")

    scrolls = {i: int(s["spec/moveTo"]) for i, s in items.items() if "spec/moveTo" in s}
    if not scrolls:
        raise SystemExit("no item in this client carries spec/moveTo - that is not credible "
                         "with the controls above passing; stop and look at the walker")

    # --- maps --------------------------------------------------------------------------
    groups, names = map_groups()
    seen = sorted(set(groups.values()))
    print("\nString.wz/Map.img: %d named maps in %d groups: %s"
          % (len(groups), len(seen), ", ".join(seen)))
    if len(seen) < 2:
        raise SystemExit("Map.img came back as one group - the walker flattened the "
                         "categories and the continent partition is gone")
    unexpected = [g for g in seen if g not in CONTINENTS]
    if unexpected:
        print("   NOTE: group(s) this client has and the tool did not expect: %s"
              % unexpected, file=sys.stderr)

    # --- the band rule, scored ---------------------------------------------------------
    real = {"MapleIsland", "VictoriaIsland", "Ossyria"}
    wrong = [(m, groups[m], band(m)) for m in sorted(groups)
             if (groups[m] if groups[m] in real else None) != band(m)]
    print("\nband rule vs String.wz over %d named maps: %d disagreement(s)"
          % (len(groups), len(wrong)))
    for m, g, b in wrong[:20]:
        print("   %-11d String.wz=%-15s band=%s" % (m, g, b))
    if wrong:
        raise SystemExit("the band rule baked into crates/world/src/returnscroll.rs "
                         "disagrees with this client - fix the Rust, do not fix this script")

    # The discriminator worth printing every run: `Other` is the one group a rule keyed on
    # the id's first two digits would split three ways.
    lead = {}
    for m, g in groups.items():
        lead.setdefault(g, set()).add(m // 1000000)
    print("leading 1e6 digits per group: %s"
          % {g: sorted(v) for g, v in sorted(lead.items())})

    itemnames = item_names()

    os.makedirs(args.out_dir, exist_ok=True)
    path = os.path.join(args.out_dir, "returnscrolls.txt")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# kind\ta\tb\tname\n")
        fh.write("# generated by tools/dump_returnscrolls.py from the client's own\n")
        fh.write("# Item.wz spec/moveTo and String.wz Map.img. TAB separated, because map\n")
        fh.write("# and item names contain commas.\n")
        fh.write("#\n")
        fh.write("# scroll <itemId> <moveTo> <name>\n")
        fh.write("#     moveTo %d is the sentinel: NOT a map id. It means 'the\n" % NO_MAP)
        fh.write("#     nearest town', resolved from the field the character stands on -\n")
        fh.write("#     use gm-handbook/returnmaps.txt's reviveMap column for that.\n")
        fh.write("#     %d of %d items in the whole item tree carry spec/moveTo.\n"
                 % (len(scrolls), len(items)))
        fh.write("# map <mapId> <group> <name>\n")
        fh.write("#     the group is String.wz/Map.img's own top-level child. Only\n")
        fh.write("#     MapleIsland, VictoriaIsland and Ossyria are continents a\n")
        fh.write("#     character travels between; Other, Event and Dev are not.\n")
        fh.write("#     %d named maps. A field image with no row here has no group at\n"
                 % len(groups))
        fh.write("#     all - there are 6 of those, 80003000..80003500.\n")
        for i in sorted(scrolls):
            fh.write("scroll\t%d\t%d\t%s\n" % (i, scrolls[i], itemnames.get(i, "")))
        for m in sorted(groups):
            fh.write("map\t%d\t%s\t%s\n" % (m, groups[m], names.get(m, "")))

    print("\n%s: %d scroll(s), %d map group(s)" % (path, len(scrolls), len(groups)))
    for i in sorted(scrolls):
        dest = scrolls[i]
        where = "NEAREST TOWN (sentinel)" if dest == NO_MAP else "%-10d %-16s %s" % (
            dest, band(dest) or "(not a continent)", names.get(dest, ""))
        print("   %-9d %-34s -> %s" % (i, itemnames.get(i, ""), where))
    return 0


if __name__ == "__main__":
    sys.exit(main())
