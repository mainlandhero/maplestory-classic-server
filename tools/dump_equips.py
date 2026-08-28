#!/usr/bin/env python3
"""Every equip's stats and upgrade slots, out of the client's own `Character.wz`.

Writes `gm-handbook/equips.txt`. Same shape as `tools/dump_portals.py`: generated game
data, gitignored, never hand-edited.

**Why the server needs this at all.** The 125-byte equipped-item body in the character
record carries the item's stat fields, and a real server fills them from the item template
at creation - `tuc` becomes the remaining upgrade count, and the stat fields come from the
template's `inc*` properties. Sending zeros gives a shirt with no defence and no upgrade
slots, which is what the owner saw on 2026-08-19.

**The column set is enumerated, not assumed.** Running this scan over all 1760 equip images
and counting every scalar `info` property is what produced the list below - and it corrected
an expectation on the way. Two of the names a modern MapleStory server would use are not in
this client at all:

* there is **no `incPAD`**. Weapon attack is **`incWAT`** (202 items). A generator written
  from the family's usual names would have emitted an empty column and missed every weapon.
* `incMMD` appears exactly **once**, against `incMDD`'s 272 - a typo in the game data, not a
  field.

And two keys in the data are literally malformed - `inc\nMHP` and `inc\nMMP`, one occurrence
each, with an embedded newline. Keys are whitespace-stripped so those land in the right
column instead of being silently dropped.

`tradeBlock` is carried because **only 7 of 1760 equips have it**, which is the measured
version of the owner's "that should only apply to some items".

    python tools/dump_equips.py
"""
import json
import os
import subprocess
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

CHARACTER_ROOT = os.path.join("client-patched", "Data", "Character")
WZ_DUMP = os.path.abspath(
    os.path.join("target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
)

# Every directory under Character/ that holds equippable items. Face and Hair are
# appearance, not equipment; Afterimage and _Canvas are art.
SLOTS = ["Accessory", "Cap", "Cape", "Coat", "Glove", "Longcoat", "Pants",
         "Ring", "Shield", "Shoes", "Weapon", "PetEquip"]

# In the order the file writes them. Every one of these was observed in the data; see the
# module docstring for the two that were expected and are not there.
STAT_KEYS = ["incSTR", "incDEX", "incINT", "incLUK", "incMHP", "incMMP",
             "incSpeed", "incJump", "incWAT", "incMAD", "incPDD", "incMDD",
             "incACC", "incEVA", "incCRT", "incCRD"]

# What it takes to PUT THE THING ON. Added 2026-08-28 because the file was missing them, and
# `research/damage-formula.md` section 6.4 had recorded that as a real gap: an agent needing
# them had to re-read all 230 weapon images out of the WZ instead.
#
# `reqJob` is a BITMASK, measured rather than assumed: every bow and crossbow is 4, every claw
# 8, every staff 2, no exceptions, and every value in the archive is a subset of those bits.
# **16 never appears**, which independently reproduces "this client has no pirates".
REQ_KEYS = ["reqLevel", "reqSTR", "reqDEX", "reqINT", "reqLUK", "reqJob"]

COLUMNS = ["tuc"] + STAT_KEYS + ["tradeBlock"] + REQ_KEYS


def run(*args):
    # Explicit UTF-8: Windows would otherwise decode wz-dump's output with the ANSI code
    # page, and some images carry bytes cp1252 has no mapping for.
    r = subprocess.run([WZ_DUMP, *args], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        return None
    return r.stdout


def images(archive):
    out = run("tree", archive, "1")
    if not out:
        return
    for line in out.splitlines():
        if "[IMG]" in line:
            yield line.split("[IMG]", 1)[1].split()[0]


def item_names():
    """`itemId -> name`, from the already-generated `gm-handbook/items.txt`.

    The owner, 2026-08-28: *"Can we enhance equips.txt to have item names so I know what to spawn
    in to test"*. The names are already dumped from `String.wz` by `tools/dump_names.py`, so
    this reads that rather than parsing the string archive a second time - one decoder, one
    place for it to be wrong.

    **A missing file is reported, not silently tolerated.** A blank name column would look
    exactly like an item the client does not have, which is the confusion the name is being
    added to remove.
    """
    path = os.path.join("gm-handbook", "items.txt")
    if not os.path.exists(path):
        print("  ! %s is missing - names will be blank. Run: python tools/dump_names.py"
              % path, file=sys.stderr)
        return {}
    out = {}
    with open(path, encoding="utf-8", errors="replace") as fh:
        for line in fh:
            if line.startswith("#"):
                continue
            parts = line.split(",", 1)
            if len(parts) != 2:
                continue
            try:
                out[int(parts[0].strip())] = parts[1].strip()
            except ValueError:
                continue
    return out


def main():
    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)

    names = item_names()

    rows = []
    skipped = 0
    for slot in SLOTS:
        archive = os.path.join(CHARACTER_ROOT, slot, slot + "_000.wz")
        if not os.path.exists(archive):
            print("  ! no archive for %s" % slot, file=sys.stderr)
            continue
        found = 0
        for image in images(archive):
            stem = image[:-4] if image.endswith(".img") else image
            if not stem.isdigit():
                continue
            out = run("cat", archive, image)
            if not out:
                skipped += 1
                continue
            try:
                node = json.loads(out)
            except ValueError:
                skipped += 1
                continue
            # Strip whitespace from keys: two of them carry an embedded newline.
            info = {
                str(k).strip(): v
                for k, v in (node.get("info") or {}).items()
                if not isinstance(v, (dict, list))
            }

            def num(key):
                v = info.get(key, 0)
                try:
                    return int(v)
                except (TypeError, ValueError):
                    return 0

            values = [num(c) for c in COLUMNS]
            name = names.get(int(stem), "")
            # An item with nothing to say costs a row and tells the server nothing - but a
            # NAMED item always has something to say now that a person reads this file to
            # decide what to spawn. Dropping it would be indistinguishable from the client
            # not having the item at all.
            if not any(values) and not name:
                continue
            rows.append((int(stem), values, name))
            found += 1
        print("  %-10s %d" % (slot, found))

    rows.sort()
    # The header claims no name contains a comma. Check it rather than assert it in prose -
    # `CLAUDE.md`: a constant that came from reading is a claim, not a fact. If this ever
    # fires, the name is still last so a parser that takes "everything after field N" keeps
    # working; only a naive split would break.
    commas = [(i, n) for i, _, n in rows if "," in n]
    if commas:
        print("  ! %d name(s) contain a comma, e.g. %r - the header says none do"
              % (len(commas), commas[0]), file=sys.stderr)

    os.makedirs("gm-handbook", exist_ok=True)
    path = os.path.join("gm-handbook", "equips.txt")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# itemId, " + ", ".join(COLUMNS) + ", name\n")
        fh.write("# generated by tools/dump_equips.py from the client's own Character.wz,\n")
        fh.write("# with names from gm-handbook/items.txt (String.wz via dump_names.py).\n")
        fh.write("# NAME IS THE LAST COLUMN, and everything after the last numeric field is\n")
        fh.write("# part of it. No name in this client contains a comma - that is checked,\n")
        fh.write("# not assumed - but do not rely on it when parsing.\n")
        fh.write("# A row is kept if it has a name OR any non-zero value.\n")
        fh.write("# Column set is enumerated from the data, not assumed: this client has\n")
        fh.write("# incWAT and NO incPAD. reqJob is a BITMASK: 1 warrior, 2 magician,\n")
        fh.write("# 4 bowman, 8 thief. 0 means anyone, and 16 never appears.\n")
        for item_id, values, name in rows:
            fh.write("%d, %s, %s\n" % (item_id, ", ".join(str(v) for v in values), name))

    print()
    print("%s: %d equips" % (path, len(rows)))
    if skipped:
        print("  (%d images could not be read)" % skipped, file=sys.stderr)
    blocked = sum(1 for _, v, _n in rows if v[COLUMNS.index("tradeBlock")])
    unnamed = sum(1 for _, _v, n in rows if not n)
    if unnamed:
        print("  %d have no name in items.txt - spawn those by id" % unnamed)
    print("  %d carry tradeBlock - the rest are freely tradeable" % blocked)


if __name__ == "__main__":
    main()
