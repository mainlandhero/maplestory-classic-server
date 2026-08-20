#!/usr/bin/env python3
"""Every quest the client ships, out of its own `Quest.wz`.

Writes three files into `gm-handbook/`. Generated game data, gitignored, never hand-edited -
same contract as `dump_portals.py`, `dump_equips.py`, `dump_mobs.py` and
`dump_npcstrings.py`.

| file | what it is for |
|---|---|
| `quests.json` | the readable tree; one image per quest, all four nodes |
| `questlines.txt` | the same data flat, one row per scalar leaf. What a human greps |
| `questreq.txt` | **only** the `mob` and `item` requirements, with the client's own slot index. What the server reads |

The third exists because the **slot index is load-bearing on the wire** and is not
recoverable from `questlines.txt` by sorting - see `requirements()` below, and
`research/quest-progress.md` for what the slot addresses.

**JSON, not a flat table, because the data is a tree.** Every other generator here writes
TSV or CSV because its data is rows; a quest's `Say` node is a conversation with branches
(`yes`, `no`, `stop`, `lost`, `ask`) and flattening it would lose the shape the server has
to walk.

## What is in it

`Quest/QuestData/QuestData_000.wz` has **322 images, and all 322 carry all four nodes**:

| node | what |
|---|---|
| `QuestInfo` | `name`, `parent`, `area`, and the numbered journal entries |
| `Check` | requirements per state - `npc`, `lvmin`, `job`, and more |
| `Act` | what happens - rewards, `nextQuest` |
| `Say` | the conversation. Numbered lines per state, plus `yes` / `no` / `stop` / `lost` / `ask` branches |

The branch census across all 322: `0` 607, `stop` 312, `yes` 298, `1` 221, `no` 149,
`2` 77, `lost` 44, `3` 29, `ask` 18, and a thin tail to `7`.

Quest 1000 is "Borrowing Sera's Mirror": `Check.0.npc = 1` starts it, `Check.1.npc = 2`
finishes it, `Act.1.nextQuest = 1001`. That matches the `0x0151` a real client sent on
2026-08-19 - quest id 1000, npc template 1 - which is the cross-check that these ids are the
same namespace the protocol uses.

## The text carries markup

`#b`/`#k` are colour codes, `#p1#` substitutes NPC template 1's name, `#i4031000#` inlines an
item icon. **It is emitted raw.** Whether the client expands these is not established, and
sending them unexpanded makes the screen answer the question - the same reason
`tools/dump_npcstrings.py` leaves `#p8#` alone.

    python tools/dump_quests.py
"""
import json
import os
import subprocess
import sys

os.chdir(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

QUEST_ARCHIVE = os.path.join("client-patched", "Data", "Quest", "QuestData",
                             "QuestData_000.wz")
WZ_DUMP = os.path.abspath(
    os.path.join("target", "release", "wz-dump") + (".exe" if os.name == "nt" else "")
)


def run(*args):
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


def requirements(quests):
    """The `mob` and `item` rows of every `Check` state, with the client's own **slot index**.

    ## Why this is a separate file and not a `questlines.txt` query

    The slot index is load-bearing on the wire and it is not recoverable from
    `questlines.txt` by sorting. A kill quest's progress string is a run of three-digit
    fields, one per entry of the client's mob array, addressed by **position in that
    array** - `FUN_14070cb70` takes `substr(index*3, index*3+3)` and `atoi`s it. Get the
    position wrong and the client counts the wrong monster, silently.

    `questlines.txt` sorts its rows by the dotted path **as a string**, so `mob.10` lands
    between `mob.0` and `mob.2`. No quest ships ten mob requirements today (the most is
    three) so nothing is wrong in the current file - but a consumer that reconstructs slot
    order by sorting that file is one data change away from being wrong, and it would be
    wrong in the quiet direction. Here the slot is a column.

    ## How the slot is computed, and the one assumption in it

    The client holds a single flat array per kind on the quest object - the mob array is
    `QuestData+0x100`, entries 20 bytes, `+0x00` id, `+0x04` count, `+0x08` the Check state
    it came from - covering **every** state, filtered at use time by comparing `+0x08`. So
    the slot is the position in that whole-quest array, not the position within a state.

    This walks states in ascending numeric order and keys in ascending numeric order inside
    each. That is only *assumed* to match the loader's append order when a quest carries mob
    entries in more than one state - and **no quest does**: all 232 `Check.<state>.mob.*`
    rows in this client are state 1. Item rows are 794 in state 1 and 4 in state 0, and the
    same caveat applies to them with the same practical answer.
    """
    rows = []
    for qid, q in quests.items():
        check = q.get("Check")
        if not isinstance(check, dict):
            continue
        states = sorted((s for s in check if str(s).isdigit()), key=int)
        for kind in ("mob", "item"):
            slot = 0
            for state in states:
                node = check[state]
                if not isinstance(node, dict):
                    continue
                group = node.get(kind)
                if not isinstance(group, dict):
                    continue
                for key in sorted((k for k in group if str(k).isdigit()), key=int):
                    entry = group[key]
                    if not isinstance(entry, dict):
                        continue
                    if "id" not in entry or "count" not in entry:
                        continue
                    rows.append((int(qid), int(state), kind, slot,
                                 int(entry["id"]), int(entry["count"])))
                    slot += 1
    rows.sort(key=lambda r: (r[0], r[2], r[3]))
    return rows


def write_requirements(path, rows):
    nl, tab = chr(10), chr(9)
    head = [
        "# questId" + tab + "state" + tab + "kind" + tab + "slot" + tab + "id" + tab
        + "count   (TAB separated)",
        "# generated by tools/dump_quests.py from the client's own Quest.wz. Never hand-edit.",
        "# kind is mob | item. state is the Check state: 0 gates ACCEPTING the quest,",
        "# 1 gates COMPLETING it. Every mob row in this client is state 1.",
        "#",
        "# slot is the index of this entry in the client's per-quest array for that kind,",
        "# counting across all states. For mob rows it is the field index in the quest's",
        "# progress string: the count lives at characters [slot*3, slot*3+3), zero padded,",
        "# read by FUN_14070cb70 as substr then atoi. Item rows carry no progress field -",
        "# the client counts the bag itself (FUN_1403eb020) every time it checks.",
    ]
    with open(path, "w", encoding="utf-8", newline=nl) as fh:
        for line in head:
            fh.write(line + nl)
        for qid, state, kind, slot, item_id, count in rows:
            fh.write(tab.join((str(qid), str(state), kind, str(slot),
                               str(item_id), str(count))) + nl)


def main():
    if not os.path.exists(WZ_DUMP):
        raise SystemExit("%s is missing - run: cargo build --release -p wz" % WZ_DUMP)
    if not os.path.exists(QUEST_ARCHIVE):
        raise SystemExit("%s is missing" % QUEST_ARCHIVE)

    quests, skipped = {}, 0
    for image in images(QUEST_ARCHIVE):
        stem = image[:-4] if image.endswith(".img") else image
        if not stem.isdigit():
            continue
        out = run("cat", QUEST_ARCHIVE, image)
        if not out:
            skipped += 1
            continue
        try:
            quests[stem] = json.loads(out)
        except ValueError:
            skipped += 1

    os.makedirs("gm-handbook", exist_ok=True)

    # A second, flat form of exactly the same data, because crates/world has no JSON
    # dependency and this repo has stayed deliberately dependency-light. Every scalar leaf
    # becomes one row of `questId <TAB> node <TAB> dotted.path <TAB> value`, which is
    # lossless for scalars and trivial to parse. The JSON below stays as the readable dump.
    flat = []

    def walk(qid, node_name, node, path):
        if isinstance(node, dict):
            for k in sorted(node, key=lambda x: (not str(x).isdigit(), str(x).isdigit() and int(x) or 0, str(x))):
                walk(qid, node_name, node[k], path + [str(k)])
        elif isinstance(node, list):
            for i, v in enumerate(node):
                walk(qid, node_name, v, path + [str(i)])
        else:
            text = " ".join(str(node).split())
            flat.append((int(qid), node_name, ".".join(path), text))

    for qid, q in quests.items():
        for node_name in ("QuestInfo", "Check", "Act", "Say"):
            node = q.get(node_name)
            if isinstance(node, dict):
                walk(qid, node_name, node, [])

    flat.sort(key=lambda r: (r[0], r[1], r[2]))
    flat_path = os.path.join("gm-handbook", "questlines.txt")
    nl, tab = chr(10), chr(9)
    with open(flat_path, "w", encoding="utf-8", newline=nl) as fh:
        fh.write("# questId" + tab + "node" + tab + "path" + tab + "value   (TAB separated)" + nl)
        fh.write("# generated by tools/dump_quests.py; the same data as quests.json, flat." + nl)
        fh.write("# node is QuestInfo | Check | Act | Say; path is the dotted key path" + nl)
        fh.write("# under it, so a Say line is like  0.2  or  0.yes.0  or  1.stop.npc.0" + nl)
        for qid, node_name, dotted, value in flat:
            fh.write(tab.join((str(qid), node_name, dotted, value)) + nl)

    path = os.path.join("gm-handbook", "quests.json")
    with open(path, "w", encoding="utf-8", newline="\n") as fh:
        json.dump(quests, fh, ensure_ascii=False, indent=1, sort_keys=True)
        fh.write("\n")

    req_rows = requirements(quests)
    req_path = os.path.join("gm-handbook", "questreq.txt")
    write_requirements(req_path, req_rows)

    def has(node):
        return sum(1 for q in quests.values() if isinstance(q.get(node), dict))

    print("%s: %d rows" % (flat_path, len(flat)))
    print("%s: %d rows (%d mob, %d item) across %d quests"
          % (req_path, len(req_rows),
             sum(1 for r in req_rows if r[2] == "mob"),
             sum(1 for r in req_rows if r[2] == "item"),
             len(set(r[0] for r in req_rows))))
    print("%s: %d quests" % (path, len(quests)))
    for node in ("QuestInfo", "Check", "Act", "Say"):
        print("  %-10s %d" % (node, has(node)))

    # Which NPC starts each quest. This is the join the server needs: the client's 0x0151
    # carries a quest id and an npc template, and Check.<state>.npc is what pairs them.
    starters = {}
    for qid, q in quests.items():
        check = q.get("Check")
        if not isinstance(check, dict):
            continue
        first = check.get("0")
        if isinstance(first, dict) and "npc" in first:
            starters.setdefault(int(first["npc"]), []).append(int(qid))
    print("  %d quests name a starting NPC, across %d NPCs"
          % (sum(len(v) for v in starters.values()), len(starters)))
    if skipped:
        print("  (%d images could not be read)" % skipped, file=sys.stderr)


if __name__ == "__main__":
    main()
