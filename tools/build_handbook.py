#!/usr/bin/env python3
"""Generate `gm-handbook/` - every game-data table the servers load - from your own client.

    cargo build --release -p wz          # the WZ reader the dump tools call
    python tools/build_handbook.py       # from the repository root

The repository carries the code that reads the client, never the client's data, so a fresh
clone has no `gm-handbook/` and the servers start with empty tables (no maps, no mobs, no
items). Each `tools/dump_*.py` writes one or two of the files; this runs them all, in an order
that respects the one dependency between them (several read `items.txt`, which
`dump_names.py` writes), and then checks that every file the servers read exists.

Reads `client-patched/Data`, the patched COPY of the client (README: Getting started). Writes
only `gm-handbook/`, which is gitignored. Standard library only.
"""
import os
import subprocess
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# dump_names.py first: dump_equips.py and others read the items.txt it writes.
TOOLS = [
    "dump_names.py",        # maps, items, mob names, item descriptions (String.wz)
    "dump_portals.py",      # portals, fields, footholds, NPC and mob spawns, reactors, clocks
    "dump_itemdata.py",     # item data, consumables
    "dump_equips.py",       # equips
    "dump_commodity.py",    # the Cash Shop's rows
    "dump_skills.py",
    "dump_mobs.py",         # mob templates
    "dump_mobskills.py",
    "dump_scrolls.py",
    "dump_summon_sacks.py",
    "dump_chairs.py",
    "dump_petequips.py",    # which pets each pet equip fits
    "dump_craftrecipe.py",
    "dump_beauty.py",       # hair and face ids
    "dump_npcstrings.py",
    "dump_pets.py",         # pet commands
    "dump_quests.py",       # quest lines and requirements
    "dump_returnmaps.py",
    "dump_returnscrolls.py",
]

# What the servers read (crates/world/src/bin/world_server.rs, crates/login, the package check).
REQUIRED = [
    "maps.txt", "items.txt", "itemdata.txt", "itemdesc.txt", "equips.txt", "consumables.txt",
    "commodity.txt", "portals.txt", "fields.txt", "footholds.txt", "npcs.txt", "mobs.txt",
    "mobtemplates.txt", "mobskills.txt", "reactors.txt", "clocks.txt", "skills.txt",
    "scrolls.txt", "summonsacks.txt", "chairs.txt", "craftrecipes.txt", "beauty.txt",
    "npcstrings.txt", "petcommands.txt", "questlines.txt", "questreq.txt", "returnmaps.txt",
]


def main():
    os.chdir(REPO)
    wz = os.path.join("target", "release", "wz-dump" + (".exe" if os.name == "nt" else ""))
    if not os.path.exists(wz):
        sys.exit("no %s - build it first:  cargo build --release -p wz" % wz)
    data = os.path.join("client-patched", "Data")
    if not os.path.isdir(data):
        sys.exit("no %s - copy your client to client-patched\\ first (README: Getting started)" % data)
    for tool in TOOLS:
        print("== %s" % tool, flush=True)
        r = subprocess.run([sys.executable, os.path.join("tools", tool)])
        if r.returncode != 0:
            sys.exit("%s failed (exit %d) - nothing after it was run" % (tool, r.returncode))
    missing = [f for f in REQUIRED if not os.path.exists(os.path.join("gm-handbook", f))]
    if missing:
        sys.exit("gm-handbook/ is missing: %s" % ", ".join(missing))
    print("gm-handbook/ is complete: %d files" % len(REQUIRED))


if __name__ == "__main__":
    main()
