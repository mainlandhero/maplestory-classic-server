#!/usr/bin/env python3
"""Every crafting recipe, from the client's own `Etc/CraftRecipe.img`.

Writes `gm-handbook/craftrecipes.txt`. Same shape as `tools/dump_chairs.py`: generated game
data, gitignored, never hand-edited.

    python tools/dump_craftrecipe.py

## What the client keeps, and what the server has to agree with

`CraftRecipe.img` is `<profession 0..5> / <craft level 1..10> / <index> / ...`, and the
client loads it at start-up into six vectors - one per profession - in `FUN_1401d36b0`. Each
recipe object's **first int is the key the client puts on the wire**, and the parser builds
it at `1401d3d0c`:

    *piVar11 = (level + profession * 10) * 1000 + index;

So Smithing's first level-1 recipe is `1000`, and Arcforge's fourth level-10 recipe is
`60003`. The `0x02F6` craft request carries that number and nothing else that identifies the
recipe, so **a server that computes the key differently cannot craft anything at all** - it
is not a display detail. `research/crafting-2026-09-21.md` §2.

## The columns

    key, profession, craftLevel, index, skillId, reqSkillLevel, coolTimeSec,
    processTimeMs, meso, additiveItemId, additiveCount, resultItem, resultCount,
    resultExp, ingredients

`ingredients` is `itemId:count` pairs joined with `|`, in the client's own order. A recipe
with no additive writes `0, 0`; the 18 recipes with no `Meso` node write `0`.

## The controls

Three, all of which have to reproduce or the run refuses to write:

* **348 recipes over six professions** - the count as read on 2026-09-21.
* **Smithing level 1 index 0** is key `1000`, five `4010000` into one `4010100`, 100 mesos,
  3 exp. That is the recipe quest 80011 (*Mr. Thunder in Need of an Apprentice*) asks the
  player to run, so it is checkable on a screen and not only in a file.
* **Every `Conditions.SkillCheck.SkillID` is `9200`..`9205` x 10000** and every
  `reqSkillLevel` equals the level node it sits under. A recipe whose skill check disagrees
  with its own level node would mean the key formula above is keyed on the wrong thing.
"""
import json
import os
import subprocess

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

ARCHIVE = os.path.join("client-patched", "Data", "Etc", "Etc_000.wz")
IMAGE = "CraftRecipe.img"
OUT = os.path.join("gm-handbook", "craftrecipes.txt")
WZ_DUMP = os.path.abspath(
    os.path.join("target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
)

#: The count on 2026-09-21. A read that loses a whole profession still looks like a file.
MIN_RECIPES = 300
#: The six profession skills, in profession order - `FUN_1401d24c0`.
PROFESSION_SKILLS = [92000000, 92010000, 92020000, 92030000, 92040000, 92050000]


def key_of(profession, level, index):
    """The client's own recipe key - `1401d3d0c`."""
    return (level + profession * 10) * 1000 + index


def num(node, name):
    v = node.get(name)
    return int(v) if isinstance(v, (int, float)) else 0


def main():
    if not os.path.exists(WZ_DUMP):
        print("no wz-dump at %s - build it first: cargo build --release -p wz" % WZ_DUMP)
        return 2
    if not os.path.exists(ARCHIVE):
        print("no archive at %s" % ARCHIVE)
        return 2

    r = subprocess.run([WZ_DUMP, "cat", ARCHIVE, IMAGE], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        print("wz-dump failed: %s" % (r.stderr or "no output"))
        return 1
    tree = json.loads(r.stdout)

    rows = []
    bad_skill = []
    for prof_name, levels in sorted(tree.items(), key=lambda kv: int(kv[0])):
        profession = int(prof_name)
        for level_name, recipes in sorted(levels.items(), key=lambda kv: int(kv[0])):
            level = int(level_name)
            for index_name, recipe in sorted(recipes.items(), key=lambda kv: int(kv[0])):
                index = int(index_name)
                check = recipe.get("Conditions", {}).get("SkillCheck", {})
                skill_id = num(check, "SkillID")
                req_level = num(check, "reqSkillLevel")
                if skill_id != PROFESSION_SKILLS[profession] or req_level != level:
                    bad_skill.append((profession, level, index, skill_id, req_level))
                results = recipe.get("Results", {})
                ingredients = recipe.get("Ingredients", {})
                pairs = []
                for ing_name in sorted(ingredients, key=int):
                    ing = ingredients[ing_name]
                    pairs.append("%d:%d" % (num(ing, "item"), num(ing, "count")))
                rows.append([
                    key_of(profession, level, index), profession, level, index,
                    skill_id, req_level,
                    num(recipe, "CoolTimeSec"), num(recipe, "ProcessTimeMS"),
                    num(recipe, "Meso"),
                    num(recipe, "AdditiveItemID"), num(recipe, "AdditiveCount"),
                    num(results, "item"), num(results, "count"), num(results, "exp"),
                    "|".join(pairs),
                ])

    if len(rows) < MIN_RECIPES:
        print("REFUSING TO WRITE: found %d recipe(s), expected at least %d. That is the shape"
              % (len(rows), MIN_RECIPES))
        print("of a broken read rather than of a client with no recipes. %s untouched." % OUT)
        return 1
    if bad_skill:
        print("REFUSING TO WRITE: %d recipe(s) whose SkillCheck does not match their own"
              % len(bad_skill))
        print("profession/level node - the key formula is keyed on something else than this")
        print("script assumes. First: %r" % (bad_skill[0],))
        return 1
    control = [r for r in rows if r[0] == 1000]
    if len(control) != 1 or control[0][11] != 4010100 or control[0][14] != "4010000:5":
        print("REFUSING TO WRITE: the control recipe (Smithing lv1 #0 -> 4010100 from five")
        print("4010000) did not reproduce; got %r" % (control,))
        return 1

    os.makedirs(os.path.dirname(OUT), exist_ok=True)
    with open(OUT, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# key, profession, craftLevel, index, skillId, reqSkillLevel, coolTimeSec,"
                 " processTimeMs, meso, additiveItemId, additiveCount, resultItem,"
                 " resultCount, resultExp, ingredients\n")
        fh.write("# generated by tools/dump_craftrecipe.py from the client's own\n")
        fh.write("# Data/Etc/Etc_000.wz `CraftRecipe.img`. Never hand-edit.\n")
        fh.write("# key = (craftLevel + profession * 10) * 1000 + index - the client's own\n")
        fh.write("# formula at 1401d3d0c, and what a 0x02F6 craft request carries.\n")
        fh.write("# ingredients are itemId:count joined with |, in the client's order.\n")
        for row in rows:
            fh.write(", ".join(str(v) for v in row) + "\n")

    print("wrote %s: %d recipe(s)" % (OUT, len(rows)))
    for profession in range(6):
        n = sum(1 for r in rows if r[1] == profession)
        additives = sum(1 for r in rows if r[1] == profession and r[9])
        print("  profession %d (skill %d): %3d recipe(s), %d with an additive"
              % (profession, PROFESSION_SKILLS[profession], n, additives))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
