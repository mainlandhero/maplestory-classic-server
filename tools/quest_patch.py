#!/usr/bin/env python3
r"""Change the client's own quest data, and install it into `client-patched/Data`.

    python tools/quest_patch.py            # build into target/quest-patch/, and say what it did
    python tools/quest_patch.py --install  # ...and copy over client-patched, keeping .bak
    python tools/quest_patch.py --revert   # put the .bak back
    python tools/quest_patch.py --check    # release check: is the installed archive this build?

## Why the client's data and not the server

The owner, 2026-09-25: *"Go through all of Maple Island's quests, remove the requirement that
quests on the island are only for Beginners, any class should be able to do them."*

**The server never checks a quest's job.** The client does, before the server hears anything:
it decides which quest an NPC offers, runs the opening conversation itself, and only sends
`0x0151` once the player has accepted (`session/npc.rs` `on_quest_request`). A job the quest
does not list never gets the offer, so there is nothing for the server to relax.

The rule is `Check/0/job` in each quest's image of `Quest/QuestData/QuestData_000.wz`. The
client's loader `FUN_14072B230` reads it (the key name through the global at `0x143A459A8`,
`14072EA33`) into a `std::set<int>` at `demand+0xC0`, beside `editByBlacklist` at `+0xD0`
**[L]**. A quest with no `job` node leaves that set empty - which is what every quest outside
Maple Island and the four job-advancement areas already has, and those are offered to
everyone. So the patch **deletes the node**; it does not list every job, which would go stale
the day a job is added and would silently exclude any job the list forgot.

## Which quests

**Every quest whose `QuestInfo/area` is 1 - Maple Island - and that has a `Check/0/job`,**
read from the PRISTINE archive (the `.bak` once installed), not from a hand-typed list. In this
client that is all twenty, 1000 to 1019: nineteen say `[0]` (Beginner) and 1001 already lists
68 jobs. 1001 is patched too, because a list of 68 is still a list - a job not on it is refused.
The run prints every id it touched, and refuses to write if the selection is ever empty.

`QuestInfo/area` is the client's own region field: area 1's twenty images are the island's
quest chain (Sera, Heena, Roger, Sen, Nina, Todd, Sam, Lucas, Mai, Biggs, Pio, Rain).

## Getting it to players

`tools/package-server.ps1` ships `client-patched\` as the canonical client and every launcher
patches itself to it (`crates/patchset`), so the archive this installs reaches a player at the
next package and deploy - one 750 KB file. `--check` is the release check: it fails if the
installed archive is not a fresh build of this script.

Same conventions as `tools/backport_install.py`, deliberately: build from the `.bak` so a
rebuild is a function of the original and this script alone, `wz-dump verify` every build, and
never write while the client holds the file open.
"""
import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
WZ_DUMP = os.path.join(REPO, "target", "release", "wz-dump" + (".exe" if os.name == "nt" else ""))
TARGET = os.path.join(REPO, "client-patched", "Data", "Quest", "QuestData", "QuestData_000.wz")
BUILD = os.path.join(REPO, "target", "quest-patch")
VERSION = "779"
MAPLE_ISLAND = 1


def dump(*args):
    p = subprocess.run([WZ_DUMP, *args], capture_output=True, text=True, encoding="utf-8", errors="replace")
    if p.returncode != 0:
        raise SystemExit("wz-dump %s failed:\n%s%s" % (" ".join(args), p.stdout, p.stderr))
    return p.stdout


def images(archive):
    for line in dump("tree", archive, "1").splitlines():
        if "[IMG]" in line:
            yield line.split("[IMG]", 1)[1].split()[0]


def sha(path):
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def select(base):
    """(image, jobs) for every Maple Island quest with a start-time job list, from `base`."""
    chosen = []
    for img in images(base):
        quest = json.loads(dump("cat", base, img))
        if quest.get("QuestInfo", {}).get("area") != MAPLE_ISLAND:
            continue
        job = quest.get("Check", {}).get("0", {}).get("job")
        if job is None:
            continue
        jobs = sorted(job.values()) if isinstance(job, dict) else job
        chosen.append((img, jobs))
    return chosen


def build(base):
    chosen = select(base)
    if not chosen:
        raise SystemExit("no Maple Island quest with a job list in %s - refusing to write an unchanged archive" % base)
    os.makedirs(BUILD, exist_ok=True)
    patch = os.path.join(BUILD, "drop-job.tsv")
    with open(patch, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# every job may start it: the whole Check/0/job node goes\n")
        fh.write("Check/0/job\tdel\n")
    spec = os.path.join(BUILD, "spec-QuestData.tsv")
    with open(spec, "w", encoding="utf-8", newline="\n") as fh:
        fh.write("# built by tools/quest_patch.py; base %s\n" % base)
        for img, _ in chosen:
            fh.write("patch\t%s\t%s\n" % (img, patch))
    out = os.path.join(BUILD, "Data", "Quest", "QuestData", "QuestData_000.wz")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    tail = dump("build", out, VERSION, spec, base).strip().splitlines()[-1:]
    print("   " + (tail[0] if tail else "(no output)"))
    for img, jobs in chosen:
        shown = str(jobs) if len(jobs) <= 4 else "%d jobs" % len(jobs)
        print("   %-10s job %s -> any" % (img, shown))

    # Prove it: every image parses, the chosen ones lost exactly that node, the rest are untouched.
    verified = dump("verify", os.path.dirname(out)).strip().splitlines()[-1:]
    print("   verify: " + (verified[0] if verified else "(no output)"))
    ids = {img for img, _ in chosen}
    for img in images(out):
        after = json.loads(dump("cat", out, img))
        before = json.loads(dump("cat", base, img))
        if img in ids:
            if "job" in after.get("Check", {}).get("0", {}):
                raise SystemExit("%s still has Check/0/job after the build" % img)
            del before["Check"]["0"]["job"]
        if after != before:
            raise SystemExit("%s differs from the base in more than Check/0/job" % img)
    print("   %d image(s) changed, every other image identical to the base" % len(ids))
    return out, chosen


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    g = ap.add_mutually_exclusive_group()
    g.add_argument("--install", action="store_true", help="copy the build over client-patched, keeping .bak")
    g.add_argument("--revert", action="store_true", help="put the .bak back")
    g.add_argument("--check", action="store_true", help="fail unless the installed archive is this build")
    args = ap.parse_args()

    if not os.path.exists(WZ_DUMP):
        raise SystemExit("no %s - run: cargo build --release -p wz" % WZ_DUMP)
    if args.revert:
        if not os.path.exists(TARGET + ".bak"):
            raise SystemExit("nothing to revert: no %s.bak" % TARGET)
        shutil.copy2(TARGET + ".bak", TARGET)
        os.remove(TARGET + ".bak")
        print("reverted %s to the original" % TARGET)
        return 0

    base = TARGET + ".bak" if os.path.exists(TARGET + ".bak") else TARGET
    print("== Quest/QuestData, base %s" % os.path.relpath(base, REPO))
    out, _ = build(base)

    if args.check:
        if not os.path.exists(TARGET + ".bak"):
            print("RELEASE CHECK FAILED: the quest patch was never installed (no .bak)")
            return 1
        if sha(TARGET) != sha(out):
            print("RELEASE CHECK FAILED: the installed QuestData_000.wz is not a fresh build of this script")
            return 1
        print("release check passed: the installed quest archive is the current build")
        return 0
    if args.install:
        if not os.path.exists(TARGET + ".bak"):
            shutil.copy2(TARGET, TARGET + ".bak")
        try:
            shutil.copy2(out, TARGET)
        except PermissionError:
            raise SystemExit("%s is in use - close the client and run --install again" % TARGET)
        print("installed %s (the original is %s.bak)" % (TARGET, os.path.basename(TARGET)))
        print("regenerate the handbook's quest tables: python tools/dump_quests.py")
    return 0


if __name__ == "__main__":
    sys.exit(main())
