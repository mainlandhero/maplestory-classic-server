#!/usr/bin/env python3
"""Every skill's per-level numbers, from the client's own `Skill.wz` and `String.wz`.

Writes `gm-handbook/skills.txt`.  Same shape as `tools/dump_commodity.py` and
`tools/dump_itemdata.py`: generated game data, gitignored, never hand-edited.

    python tools/dump_skills.py                     # writes into gm-handbook/
    python tools/dump_skills.py --out-dir tmp

Run it WITH THE REPO AS THE WORKING DIRECTORY (it chdirs there itself).

WHAT THIS IS FOR
================
Before this file the repo had **no skill data at all** - no dump, no table, nothing.
`net::buff::buff_level` models exactly one skill (Nimble Feet, 1002) and answers everything
else with *"This server does not grant skill N's effect yet."*  Every number a skill needs -
what it costs, how long it lasts, how hard it hits - is in the client's own archive and this
reads it out.

WHERE THE NUMBERS LIVE, AND THE THREE PLACES ONE CAN HIDE
=========================================================
`Skill/Skill_000.wz` holds one image per **skill book** (`200.img` is the Magician's first
job) and each image has a `skill` node whose children are the skills, keyed by the id
**zero-padded to seven digits** (`2001002`, and `0001002` for Nimble Feet).  Under a skill:

    level/<n>/...     the per-level numbers.  This is where almost everything is.
    common/...        LEVEL-INVARIANT numbers, stored as STRINGS.  22 skills have one.
    <top level>       masterLevel, psd, processtype, elemAttr, weapon...  scalars.

**A tool that reads only `level/<n>` loses data and does not say so.**  Energy Bolt's only
statement of its reach is `common/range = "350"`; there is no per-level `range` on it at all.
So `common` is MERGED into every level row of that skill, and:

* the level node wins where both define a key - which happens exactly 40 times, all of them
  Flash Jump's `x`/`y`, and the run prints that count;
* every column filled from `common` is named in that row's `fromCommon` cell, so a merged
  value is never mistaken for one the level node carried.

AND THE NUMBERS ARE NOT ALL NUMBERS
===================================
The same property is an integer on one skill and a **quoted string** on another - `mpCon` is
a string on 61 level nodes, `x` on 100, `mastery` on 1 (Fire Arrow level 30, `"10"`).  A
reader that keeps only `int` comes back short, clean and confident, which is the failure
`CLAUDE.md` describes under "the scratchpad shadows the real tools".  Every numeric column is
coerced through `num()` and **the run prints how many cells were coerced**; a value that will
not coerce is reported by skill and property rather than silently blanked.

THE UNITS, AND HOW EACH ONE IS KNOWN
====================================
`CLAUDE.md`: three bugs this month were a correct number in the wrong unit.  The client
states its own units in `String.wz/Skill.img/<id>/h<level>` - the tooltip line the player
reads - so every level row carries that text in its last column and the unit claims below are
checked against it rather than assumed:

| column | unit | how that is known |
|---|---|---|
| `mpCon`, `hpCon` | **flat points** | Nimble Feet L1 `mpCon 4` <-> tooltip "MP -4" |
| `time`, `subTime`, `time2` | **SECONDS** | Nimble Feet L1 `time 10` <-> "for 10 sec"; `net::buff` multiplies by 1000 because the wire is MILLISECONDS |
| `cooltime` | **SECONDS** | Nimble Feet `cooltime 180` <-> "Cooldown: 3 min." |
| `mad` | magic attack **percent** | Energy Bolt L1 `mad 90` <-> "Basic Attack 90". The client's word is "Basic Attack", not "%", so the percent reading is [I] - the formula is not decoded here |
| `damage` | physical attack percent | never on the same skill as `mad`: 47 skills carry `damage`, 14 carry `mad`, **0 carry both** |
| `mastery` | **a LEVEL 1..10, NOT a percent** | "Mastery level 1" in the tooltip of every skill that has it, magician and warrior alike. What that level does to the damage spread is not in `Skill.wz` |
| `attackCount` | attacks per cast | Magic Claw `attackCount 2` <-> "attack an enemy twice" |
| `mobCount` | targets per cast | |
| `range`, `ltX/ltY/rbX/rbY` | **client pixels** | a rectangle around the caster; sign included |
| `indiePdd`, `indieMdd` | **flat defence points** | Magic Armor L1 `indiePdd 40` <-> "Weapon Def. +40" |
| `mmpR`, `mhpR`, `indieMhpR` | **percent** of the maximum | Max MP Increase L1 `mmpR 10` <-> "Max MP +10%" |
| `x`, `y` | **skill-specific** - read the tooltip | Magic Guard `x 30` is "Replace 30% of HP damage"; Improved MP Recovery `x 1` is "1% of Max MP", `y 5` is "+5% from items" |
| `prop` | percent chance | |
| `speed` | flat speed points | Nimble Feet `speed 10` <-> "speed +10" |

`x` and `y` deserve the warning twice: **they mean a different thing on every skill** and
nothing in the WZ says which.  The tooltip column is the only thing in this file that does.

WHAT THIS CANNOT SAY
====================
* **What the server has to send.**  This is the client's authored data; which of it the
  client applies on its own and which needs a packet is not in `Skill.wz`.  The client image
  does contain the property names as UTF-16 literals (`mmpR`, `indiePdd`, `mad`, `mastery`,
  `mpCon`, `cooltime`), so client code CAN look them up by name - but a string being present
  is not a read, and this tool cannot promote it to one.
* **Any damage formula.**  `mad` and `mastery` are inputs to a function nobody here has
  decoded; `research/damage-formula.md` says magic damage is a different path entirely.
* **What `type`, `processtype`, `skillType` or `additional_process` mean.**  They are emitted
  raw.  One correlation is strong enough to write down and it is in the output header:
  every one of the 16 skills with `processtype 113` has **no `time` at any level** and its
  own description says the effect is switched on and off by re-use.
* **Animation.**  `icon`, `effect`, `hit`, `ball`, `action` frames and every other canvas are
  skipped by name (see `CANVAS_KEYS`), not dropped by accident.

THE INSTRUMENT CHECKS ITSELF
============================
Four positive controls run before anything is written, and each names a node that is known to
be present, so an empty output has to be the data's fault rather than the parser's.  The first
one is the important one: **Nimble Feet's three levels must come out 10/20/30 seconds at
4/7/10 MP with a 180 s cooldown**, which is what `crates/net/src/buff.rs` carries and what the
repo has confirmed on a client.  If this extraction disagrees with that, the extraction is
wrong.

The second control exists because this exact bug was hit while writing the tool: the
String.wz key is the **zero-padded** id, and looking up `str(int(id))` silently returns
nothing for every beginner skill.  A missing name is invisible in the output, so it is
asserted instead.

Two whole-archive censuses are printed rather than asserted, because they are measurements:

* the level count of every skill against the `[Master Level: N]` its own tooltip states -
  **163 agree, 0 disagree, 13 state none** in this build, which is what makes `maxLevel` a
  measurement and not a convention;
* every image stem against the job derived from the skill ids inside it.
"""
import argparse
import json
import os
import re
import subprocess
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

SKILL_WZ = os.path.join("client-patched", "Data", "Skill", "Skill_000.wz")
STRING_WZ = os.path.join("client-patched", "Data", "String", "String_000.wz")
WZ_DUMP = os.path.abspath(
    os.path.join("target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
)

#: Top-level skill properties that are canvases, animation or sub-node lists.  Named here so
#: they are skipped ON PURPOSE and anything else without a column gets REPORTED instead.
CANVAS_KEYS = {
    "icon", "iconMouseOver", "iconDisabled", "effect", "effect0", "effect1", "effect2",
    "effect3", "hit", "ball", "ball0", "ball1", "action", "affected", "affected0", "mob",
    "mob0", "special", "prepare", "finish", "tile", "summon", "state", "teleport",
    "afterimage", "multiAttackInfo", "extraSkillInfo", "cDoor", "mDoor", "Frame",
    "additional_process", "info", "level", "common", "req",
}

#: Level properties that are canvases rather than numbers - a skill may override its hit or
#: ball animation per level.  Skipped for the same reason and in the same explicit way.
CANVAS_LEVEL_KEYS = {"hit", "ball"}

#: Per-skill columns.  `info/*` is flattened in with `infoDot` renamed, because `dot` is also
#: a level property and merging the two namespaces would let one silently win - the same trap
#: `tools/dump_itemdata.py` records for `info/time` versus `spec/time`.
INFO_COLUMNS = [
    ("type", "type"), ("magicDamage", "magicDamage"), ("areaAttack", "areaAttack"),
    ("infoDot", "dot"), ("condition", "condition"), ("massSpell", "massSpell"),
    ("finalAttack", "finalAttack"), ("casterMove", "casterMove"),
]

#: Scalar properties that sit directly on the skill node.  Measured: these are every
#: top-level key in this archive whose value is never a dict.
SKILL_SCALARS = [
    "masterLevel", "psd", "processtype", "skillType", "elemAttr", "elemAttr2",
    "weapon", "weapon2", "weapon3", "weapon4", "invisible", "noBulletConsume",
    "notRemoved", "notAbleNearbyPortal", "showSummonedBuffIcon",
]

#: Every per-level property in this archive, plus the seven that only ever appear in
#: `common`.  `lt`/`rb`/`lt2`/`rb2` are points and are expanded into X/Y columns below, so
#: they are not in this list.  A property in the data that is in neither is REPORTED.
LEVEL_SCALARS = [
    # cost
    "mpCon", "hpCon", "moneyCon", "itemCon", "itemConNo", "bulletConsume",
    # timing - seconds, every one of them
    "time", "time2", "subTime", "cooltime", "updatableTime",
    # attack
    "damage", "mad", "fixdamage", "attackCount", "mobCount", "bulletCount", "mastery",
    "prop", "range", "z",
    "ballDelay", "ballDelay1", "ballDelay2", "ballDelay3", "ballDelay4", "ballDelay5",
    "ballDelay6", "ballDelay7",
    # damage over time
    "dot", "dotTime", "dotInterval",
    # effect magnitudes
    "x", "y", "speed",
    "indiePad", "indieMad", "indiePdd", "indieMdd", "indiePddR", "indieMhpR",
    "indieAcc", "indieEva", "indieSpeed", "indieJump",
    "mhpR", "mmpR", "pddX", "accX", "evaX", "crtX", "crdX",
    "resI", "resF", "resL", "resS",
]

#: Point-valued level properties, emitted as two columns each.
POINT_KEYS = ["lt", "rb", "lt2", "rb2"]

#: Columns whose value is text and must not be coerced to a number.
#: `info/condition` is the word "attack" on four skills, not a number.
TEXT_COLUMNS = {"name", "elemAttr", "elemAttr2", "hs", "action", "req", "fromCommon",
                "tooltip", "condition"}

#: Positive controls.  Each names a node that IS in this archive.
#:
#: `nimble_feet` is the one that matters: these are the numbers `crates/net/src/buff.rs`
#: carries and the repo has confirmed on a client.  If the extraction disagrees with them the
#: extraction is wrong, not the repo.
CONTROLS_NIMBLE_FEET = {
    "1": {"mpCon": 4, "time": 10, "speed": 10, "cooltime": 180},
    "2": {"mpCon": 7, "time": 20, "speed": 10, "cooltime": 180},
    "3": {"mpCon": 10, "time": 30, "speed": 10, "cooltime": 180},
}
CONTROL_NIMBLE_FEET_ID = "0001002"
CONTROL_NIMBLE_FEET_NAME = "Nimble Feet"
#: Energy Bolt's reach is in `common` and NOWHERE else.  Without the merge it disappears.
CONTROL_COMMON = ("2001002", "range", "350")
#: The Magician's first skill book names itself, which is how job 200 is identified without
#: assuming a numbering scheme.
CONTROL_BOOK = ("200", "Introduction to Magic")

MASTER_LEVEL_RE = re.compile(r"Master Level\s*:?\s*(\d+)")


def run(*args):
    """One wz-dump invocation.  Explicit UTF-8: Windows would otherwise decode wz-dump's
    stdout with the ANSI code page and some images carry bytes cp1252 cannot map."""
    r = subprocess.run([WZ_DUMP, *args], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    if r.returncode != 0 or not r.stdout:
        raise SystemExit("wz-dump %s failed (rc=%s): %s"
                         % (" ".join(args), r.returncode, (r.stderr or "").strip()[:300]))
    return r.stdout


def image(archive, name):
    return json.loads(run("cat", archive, name))


def images(archive):
    for line in run("tree", archive, "1").splitlines():
        if "[IMG]" in line:
            yield line.split("[IMG]", 1)[1].split()[0]


def text(value):
    """A WZ string with its markup left alone but its separators made safe.

    Commas would split a column and the file is comma-separated, so they become semicolons
    and the run reports how many cells that happened to - the same bargain
    `tools/dump_commodity.py` strikes for item names carrying a comma.
    """
    if value is None:
        return ""
    s = str(value)
    if len(s) >= 2 and s[0] == '"' and s[-1] == '"':
        s = s[1:-1]
    # The WZ carries a literal backslash-n in descriptions, not a newline.  Both are flattened.
    s = s.replace("\\n", " ").replace("\n", " ").replace("\r", " ")
    s = s.replace(" ", " ")
    return " ".join(s.split())


class Coercions:
    """How many numeric cells arrived as strings, and which ones would not convert.

    Kept as an object rather than a global because the count is REPORTED - a run that
    silently dropped 240 string-valued numbers would look exactly like a clean one.
    """

    def __init__(self):
        self.from_string = 0
        self.failed = []

    def num(self, value, where):
        if value is None or isinstance(value, bool):
            return ""
        if isinstance(value, int):
            return str(value)
        if isinstance(value, float):
            return str(int(value)) if value.is_integer() else str(value)
        s = str(value).strip().strip('"')
        if not s:
            return ""
        try:
            out = str(int(s))
        except ValueError:
            try:
                out = str(int(float(s)))
            except ValueError:
                self.failed.append((where, value))
                return ""
        self.from_string += 1
        return out


def load_skills(archive):
    """{padded id: (image, node)} for every skill in every image of the archive."""
    out, no_skill_node = {}, []
    for name in images(archive):
        node = image(archive, name)
        skills = node.get("skill")
        if not isinstance(skills, dict):
            no_skill_node.append((name, sorted(node.keys())[:6]))
            continue
        for key, skill in skills.items():
            if isinstance(skill, dict):
                out[key] = (name, skill)
    return out, no_skill_node


def load_strings():
    """String.wz's Skill.img, keyed exactly as it is on disk.

    **The keys are the ZERO-PADDED ids** - `0001002`, not `1002` - and three-digit keys are
    skill BOOKS carrying a `bookName`.  Looking a skill up by `str(int(id))` returns nothing
    for every beginner skill and says nothing about it, which is why there is a control.
    """
    return image(STRING_WZ, "Skill.img")


def check_controls(skills, strings, coerce):
    nf = skills.get(CONTROL_NIMBLE_FEET_ID)
    if nf is None:
        raise SystemExit(
            "POSITIVE CONTROL FAILED: Skill.wz has no skill %s (Nimble Feet). The parser or "
            "the archive changed; an empty run would not have meant 'no skills'."
            % CONTROL_NIMBLE_FEET_ID)
    levels = nf[1].get("level") or {}
    for level, want in CONTROLS_NIMBLE_FEET.items():
        got = levels.get(level)
        if not isinstance(got, dict):
            raise SystemExit("POSITIVE CONTROL FAILED: Nimble Feet has no level %s." % level)
        for prop, value in want.items():
            if coerce.num(got.get(prop), "control") != str(value):
                raise SystemExit(
                    "POSITIVE CONTROL FAILED: Nimble Feet level %s reads %s=%r, expected %r. "
                    "crates/net/src/buff.rs carries these numbers and the repo has confirmed "
                    "them on a client, so this extraction is wrong - not the repo."
                    % (level, prop, got.get(prop), value))

    name = (strings.get(CONTROL_NIMBLE_FEET_ID) or {}).get("name")
    if text(name) != CONTROL_NIMBLE_FEET_NAME:
        raise SystemExit(
            "POSITIVE CONTROL FAILED: String.wz names skill %s as %r, expected %r. The key is "
            "the ZERO-PADDED id; str(int(id)) silently misses every beginner skill and a "
            "blank name column would not have said so."
            % (CONTROL_NIMBLE_FEET_ID, name, CONTROL_NIMBLE_FEET_NAME))

    sid, prop, want = CONTROL_COMMON
    got = ((skills.get(sid) or (None, {}))[1].get("common") or {}).get(prop)
    if text(got) != want:
        raise SystemExit(
            "POSITIVE CONTROL FAILED: skill %s has common/%s = %r, expected %r. That node is "
            "the ONLY statement of Energy Bolt's reach - it has no per-level range - so a "
            "run without it would be quietly short one column."
            % (sid, prop, got, want))

    book, want_name = CONTROL_BOOK
    got = text((strings.get(book) or {}).get("bookName"))
    if got != want_name:
        raise SystemExit(
            "POSITIVE CONTROL FAILED: String.wz book %s is %r, expected %r." % (book, got, want_name))

    print("positive controls: Nimble Feet 4/7/10 MP at 10/20/30 s and a 180 s cooldown "
          "(matches crates/net/src/buff.rs), its zero-padded String.wz name, Energy Bolt's "
          "common/range, and book 200 = %r - all four parse" % want_name)


def columns():
    head = ["skillId", "job", "level", "maxLevel", "commonMaxLevel", "name"]
    head += [c for c, _ in INFO_COLUMNS]
    head += SKILL_SCALARS
    head += ["req"]
    head += ["hs"]
    head += LEVEL_SCALARS
    head += [p + axis for p in POINT_KEYS for axis in ("X", "Y")]
    head += ["action", "fromCommon", "tooltip"]
    return head


def build_rows(skills, strings, coerce):
    head = columns()
    rows = []
    skipped, unknown_top, unknown_level = [], {}, {}
    clashes, merged, comma_cells, missing_names, missing_tooltips = 0, 0, 0, [], 0
    job_mismatch = []

    for key in sorted(skills):
        img, node = skills[key]
        try:
            skill_id = int(key)
        except ValueError:
            skipped.append((key, "id is not a number"))
            continue

        for prop in node:
            if prop in CANVAS_KEYS or prop in SKILL_SCALARS:
                continue
            if prop.isdigit():           # skill 2211004 carries four bare numeric children
                unknown_top.setdefault("<numeric child>", 0)
                unknown_top["<numeric child>"] += 1
                continue
            unknown_top.setdefault(prop, 0)
            unknown_top[prop] += 1

        # `job` is DERIVED - id // 10000 - and then checked against the image's own stem,
        # which is an independent statement of the same thing.
        job = skill_id // 10000
        stem = img[:-4] if img.endswith(".img") else img
        if stem.isdigit() and int(stem) != job:
            job_mismatch.append((key, img, job))

        strs = strings.get(key) or {}
        name = text(strs.get("name"))
        if not name:
            missing_names.append(key)

        levels = node.get("level")
        if not isinstance(levels, dict) or not levels:
            skipped.append((key, "no level node - %s" % ", ".join(sorted(node.keys()))))
            continue
        common = {k: v for k, v in (node.get("common") or {}).items()}
        max_level = len(levels)

        per_skill = {"skillId": str(skill_id), "job": str(job), "maxLevel": str(max_level),
                     "commonMaxLevel": coerce.num(common.get("maxLevel"), key),
                     "name": name}
        info = node.get("info") or {}
        for column, prop in INFO_COLUMNS:
            value = info.get(prop)
            per_skill[column] = text(value) if column in TEXT_COLUMNS else \
                coerce.num(value, "%s/info/%s" % (key, prop))
        for prop in SKILL_SCALARS:
            value = node.get(prop)
            per_skill[prop] = text(value) if prop in TEXT_COLUMNS else \
                coerce.num(value, "%s/%s" % (key, prop))
        req = node.get("req") or {}
        per_skill["req"] = "|".join(
            "%d:%s" % (int(k), coerce.num(v, "%s/req" % key))
            for k, v in sorted(req.items(), key=lambda kv: int(kv[0])) if str(k).isdigit())

        for level_key in sorted(levels, key=lambda k: int(k) if k.isdigit() else -1):
            level = levels[level_key]
            if not isinstance(level, dict):
                skipped.append(("%s/%s" % (key, level_key), "level is not a property bag"))
                continue

            from_common = []
            values = dict(level)
            for prop, value in common.items():
                if prop == "maxLevel":
                    continue
                if prop in values:
                    if str(values[prop]) != str(value).strip('"'):
                        clashes += 1
                    continue
                values[prop] = value
                from_common.append(prop)
                merged += 1

            for prop in values:
                if prop in CANVAS_LEVEL_KEYS or prop in LEVEL_SCALARS or prop in POINT_KEYS:
                    continue
                if prop == "hs":
                    continue
                if prop == "action":
                    continue
                unknown_level.setdefault(prop, 0)
                unknown_level[prop] += 1

            row = dict(per_skill)
            row["level"] = level_key
            row["hs"] = text(values.get("hs"))
            row["action"] = text(values.get("action"))
            for prop in LEVEL_SCALARS:
                row[prop] = coerce.num(values.get(prop), "%s/%s/%s" % (key, level_key, prop))
            for prop in POINT_KEYS:
                point = values.get(prop)
                for axis in ("x", "y"):
                    cell = ""
                    if isinstance(point, dict):
                        cell = coerce.num(point.get(axis), "%s/%s/%s" % (key, level_key, prop))
                    row[prop + axis.upper()] = cell
            row["fromCommon"] = "|".join(sorted(from_common))

            hs = row["hs"] or ("h" + level_key)
            tip = text(strs.get(hs))
            if not tip:
                missing_tooltips += 1
            if "," in tip:
                tip = tip.replace(",", ";")
                comma_cells += 1
            row["tooltip"] = tip

            rows.append([row.get(c, "") for c in head])

    stats = dict(skipped=skipped, unknown_top=unknown_top, unknown_level=unknown_level,
                 clashes=clashes, merged=merged, comma_cells=comma_cells,
                 missing_names=missing_names, missing_tooltips=missing_tooltips,
                 job_mismatch=job_mismatch)
    return head, rows, stats


def master_level_census(skills, strings):
    """Level count against the `[Master Level: N]` the skill's own tooltip states.

    This is what makes `maxLevel` a MEASUREMENT.  Two independent authorings of the same
    number - the data and the localised description - and they are compared rather than one
    being assumed to follow from the other.
    """
    agree, disagree, silent = 0, [], 0
    for key, (_, node) in skills.items():
        levels = node.get("level")
        if not isinstance(levels, dict) or not levels:
            continue
        desc = text((strings.get(key) or {}).get("desc"))
        found = MASTER_LEVEL_RE.search(desc or "")
        if not found:
            silent += 1
            continue
        if int(found.group(1)) == len(levels):
            agree += 1
        else:
            disagree.append((key, len(levels), int(found.group(1))))
    return agree, disagree, silent


def write(out_dir, head, rows):
    os.makedirs(out_dir, exist_ok=True)
    path = os.path.join(out_dir, "skills.txt")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# " + ", ".join(head) + "\n")
        fh.write("# generated by tools/dump_skills.py from the client's own\n")
        fh.write("# Data/Skill/Skill_000.wz and Data/String/String_000.wz `Skill.img`.\n")
        fh.write("# NEVER HAND-EDIT: regenerate.\n")
        fh.write("#\n")
        fh.write("# One row = ONE SKILL AT ONE LEVEL. The per-skill columns repeat on every\n")
        fh.write("# row of that skill.\n")
        fh.write("# AN EMPTY CELL MEANS THE PROPERTY IS ABSENT FROM THAT WZ NODE, which is\n")
        fh.write("# not the same as 0.\n")
        fh.write("#\n")
        fh.write("# UNITS - three bugs this month were a correct number in the wrong unit:\n")
        fh.write("#   time, time2, subTime, cooltime  SECONDS. The wire wants MILLISECONDS;\n")
        fh.write("#                          net::buff multiplies by 1000.\n")
        fh.write("#   mpCon, hpCon           flat points.\n")
        fh.write("#   indiePdd, indieMdd     flat defence points ('Weapon Def. +40').\n")
        fh.write("#   mmpR, mhpR, indieMhpR  PERCENT of the maximum, not an amount.\n")
        fh.write("#   mad                    magic attack, on skills that have no `damage`.\n")
        fh.write("#   damage                 physical attack. NO skill carries both.\n")
        fh.write("#   mastery                a LEVEL 1..10, NOT a percentage. The client's\n")
        fh.write("#                          own tooltip says 'Mastery level 3'. What that\n")
        fh.write("#                          does to the damage spread is not in Skill.wz.\n")
        fh.write("#   attackCount/mobCount   attacks per cast / targets per cast.\n")
        fh.write("#   range, ltX..rbY        client pixels, signed, relative to the caster.\n")
        fh.write("#   x, y                   SKILL-SPECIFIC. They mean something different on\n")
        fh.write("#                          every skill and the WZ does not say what. The\n")
        fh.write("#                          `tooltip` column is the only thing here that does.\n")
        fh.write("#\n")
        fh.write("# job          DERIVED as skillId / 10000, then checked against the WZ\n")
        fh.write("#              image the skill was found in. 200 is the Magician's first job.\n")
        fh.write("# maxLevel     the number of `level` children. MEASURED, not assumed: it\n")
        fh.write("#              agrees with the '[Master Level: N]' in the skill's own\n")
        fh.write("#              description for all 163 skills that state one, 0 disagree.\n")
        fh.write("# masterLevel  the WZ property of that name, which only 3 skills carry.\n")
        fh.write("# fromCommon   columns on this row that came from the skill's LEVEL-INVARIANT\n")
        fh.write("#              `common` node rather than from `level/<n>`. Energy Bolt's\n")
        fh.write("#              range is only there; a reader of `level` alone loses it.\n")
        fh.write("# tooltip      String.wz's h-string for this exact level - the line the\n")
        fh.write("#              player reads. Commas in it were turned into semicolons.\n")
        fh.write("#              This is the client's own statement of the units above.\n")
        fh.write("# processtype  emitted raw; meaning not established. One correlation is\n")
        fh.write("#              solid: all 16 skills with processtype 113 have NO `time` at\n")
        fh.write("#              any level and their descriptions say the effect is switched\n")
        fh.write("#              on and off by using the skill again - Magic Guard is one.\n")
        fh.write("# Animation nodes (icon, effect, hit, ball, action frames) are skipped by\n")
        fh.write("# name, not by accident. Anything else with no column is REPORTED by the\n")
        fh.write("# run rather than dropped quietly.\n")
        for row in rows:
            fh.write(", ".join(row) + "\n")
    return path


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out-dir", default="gm-handbook")
    ap.add_argument("--archive", default=SKILL_WZ)
    args = ap.parse_args()

    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)
    for path in (args.archive, STRING_WZ):
        if not os.path.exists(path):
            raise SystemExit("%s is missing" % path)

    skills, no_skill_node = load_skills(args.archive)
    strings = load_strings()
    coerce = Coercions()
    check_controls(skills, strings, coerce)

    head, rows, stats = build_rows(skills, strings, coerce)
    if not rows:
        raise SystemExit(
            "Skill.wz parsed to ZERO level rows. The positive controls passed, so this is "
            "the data and not the parser - but an empty skills.txt would be worse than no "
            "file, so nothing was written. %d skill(s) were skipped: %s"
            % (len(stats["skipped"]),
               "; ".join("%s (%s)" % s for s in stats["skipped"][:20]) or "none at all"))

    path = write(args.out_dir, head, rows)

    jobs = sorted({int(r[head.index("job")]) for r in rows})
    print("%s: %d skills, %d skill-levels, %d columns, jobs %s"
          % (path, len({r[0] for r in rows}), len(rows), len(head), jobs))

    agree, disagree, silent = master_level_census(skills, strings)
    print("  maxLevel vs the tooltip's '[Master Level: N]': %d agree, %d disagree, "
          "%d state none" % (agree, len(disagree), silent))
    for key, count, said in disagree:
        print("    ! %s has %d levels and its description says %d" % (key, count, said))

    print("  %d cell(s) filled from a `common` node; %d level/common value clash(es) "
          "(the level node wins)" % (stats["merged"], stats["clashes"]))
    print("  %d numeric cell(s) arrived as a QUOTED STRING and were coerced - a reader that "
          "kept only ints would be short by that many" % coerce.from_string)
    for where, value in coerce.failed[:20]:
        print("    ! %s = %r would not convert to a number" % (where, value))
    if coerce.failed:
        print("    %d value(s) would not convert" % len(coerce.failed))

    for name, keys in no_skill_node:
        print("  image %s has no `skill` node (top: %s)" % (name, ", ".join(keys)))
    for key, why in stats["skipped"]:
        print("  skipped %s: %s" % (key, why))
    for key, img, job in stats["job_mismatch"]:
        print("  ! skill %s is in %s but its id derives job %d" % (key, img, job))
    if stats["missing_names"]:
        print("  %d skill(s) have no String.wz name: %s"
              % (len(stats["missing_names"]), ", ".join(stats["missing_names"][:12])))
    print("  %d level(s) have no tooltip h-string" % stats["missing_tooltips"])
    if stats["comma_cells"]:
        print("  %d tooltip(s) contained a comma and had it replaced with a semicolon"
              % stats["comma_cells"])
    if stats["unknown_top"]:
        print("  ! skill properties with NO column (emitted nowhere): %s"
              % ", ".join("%s x%d" % kv for kv in sorted(stats["unknown_top"].items())))
    else:
        print("  every top-level skill property has a column or is a named animation node")
    if stats["unknown_level"]:
        print("  ! LEVEL properties with NO column (emitted nowhere): %s"
              % ", ".join("%s x%d" % kv for kv in sorted(stats["unknown_level"].items())))
    else:
        print("  every level property present in the data has a column")
    return 0


if __name__ == "__main__":
    sys.exit(main())
