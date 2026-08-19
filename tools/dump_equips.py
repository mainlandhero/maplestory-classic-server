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
COLUMNS = ["tuc"] + STAT_KEYS + ["tradeBlock"]


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


def main():
    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)

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
            # An item with nothing to say costs a row and tells the server nothing.
            if not any(values):
                continue
            rows.append((int(stem), values))
            found += 1
        print("  %-10s %d" % (slot, found))

    rows.sort()
    os.makedirs("gm-handbook", exist_ok=True)
    path = os.path.join("gm-handbook", "equips.txt")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# itemId, " + ", ".join(COLUMNS) + "\n")
        fh.write("# generated by tools/dump_equips.py from the client's own Character.wz.\n")
        fh.write("# Rows with every column zero are omitted. Column set is enumerated from\n")
        fh.write("# the data, not assumed: this client has incWAT and NO incPAD.\n")
        for item_id, values in rows:
            fh.write("%d, %s\n" % (item_id, ", ".join(str(v) for v in values)))

    print()
    print("%s: %d equips" % (path, len(rows)))
    if skipped:
        print("  (%d images could not be read)" % skipped, file=sys.stderr)
    blocked = sum(1 for _, v in rows if v[COLUMNS.index("tradeBlock")])
    print("  %d carry tradeBlock - the rest are freely tradeable" % blocked)


if __name__ == "__main__":
    main()
