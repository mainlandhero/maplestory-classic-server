# MapleCW

A private, local-only server emulator for the MapleStory "Classic World" client the owner owns.
Rust workspace, SQLite, patched client in `client-patched/`. Testing only.

**Start at `STATUS.md` → NEXT GOALS.** It is kept current; if it contradicts this file, it wins.

## Standing constraints

- **Never store passwords in plain text.** argon2id, salted. Session tokens stored as SHA-256 only.
- The firewall rule is scoped to the patched executable only.
- `client-patched/` exists so the original client is never touched.
- **Nothing authenticates.** The game socket carries no credentials. Say so whenever reporting progress.
- **Never renumber characters from 1.** Ids start at 200 (`FIRST_CHARACTER_ID`); a create reply carrying id 1 made the client silently refuse to transition.

## Shell

Windows PowerShell **5.1**, from an **elevated** window. `pwsh` is **not installed**.

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1"
```

5.1 has no `&&`, `||`, ternary or null-coalescing.

**Heredocs halve backslashes.** `\\b` written in a `<<'PY'` heredoc reaches Python as `\b`,
which becomes a backspace and silently breaks a regex or a Rust doc comment. Use the Write
tool for anything containing backslashes, or avoid them entirely.

## Client runs cost the owner a manual launch

Measure first. `python tools/channel_smoke.py` and `tools/login_smoke.py` exercise the real
servers over an independent Python transport — framing, cipher, every field offset — without
a launch. Only run the client when nothing cheaper can answer the question.

When a run happens, **test one variant at a time**. Changing two things at once has already
produced one unexplained crash. The owner can see the GUI and you cannot, so say exactly what to
watch for and what each outcome would mean *before* they launch.

## Two rules that come from expensive mistakes

**Always answer.** An unanswered packet freezes the client's entire UI — every button,
including the quit prompt — and reads on screen as a crash. Never return an error in place
of a reply.

**Verify the instrument before believing it.** A silent negative is usually a property of
your search, not evidence of absence. Prove a search can find a positive control before
reporting that it found nothing. And **enumerate before you filter**: the two worst wrong
answers here both came from searching a known list — one looked for the wrong *shape*
(bit tests, when the mask was a byte array), one for the wrong *set* (five decoder
addresses, when there are seven). Both returned clean, confident numbers.

## Working alongside agents

An agent's files are **its** files until it reports back. Three rules, all learned the same
day by breaking them.

**Never `git add -A` while an agent is running.** Stage by path, and stage only paths you
touched yourself. A green test suite is not evidence that an agent has finished - it is
evidence that whatever it has written so far compiles. On 2026-08-19 this committed
half-written work from three different agents, and in one case a file was still being edited
as it went into the commit.

**Do not edit a file an agent owns.** Give each agent its own new files, tell it which shared
files are off-limits, and integrate yourself once it reports. `crates/world/src/session.rs`
is the usual integration point and should stay with the coordinator.

**Re-run your own analysis when you fix a shared instrument.** If a tool was wrong, every
conclusion drawn with it is suspect - including yours. On 2026-08-19 `tools/reads.py` was
fixed to count tail-call reads, four agents were told to re-run anything resting on a read
count, and the person who sent that message did not re-run their own. The result was the same
packet shipping short a second time and killing the client a second time. The fixed tool's
own output had already printed the missing read; it scrolled past unread.

## The test summary that could not count failures

`cargo test --workspace | awk -F'[ ;]' '{p+=$4; f+=$6}'` **always reports zero failures.**
With `;` as a separator, `0 failed` lands in field 7 and field 6 is the empty string between
`passed;` and the space. It was used to report "N passed, 0 failed" repeatedly on 2026-08-19
before anyone noticed the counter was structurally incapable of returning anything else.

Count with something that reads the word, not the position:

```
cargo test --workspace 2>&1 | grep -E "^test result:"   | sed -E 's/test result: (ok|FAILED)\. ([0-9]+) passed; ([0-9]+) failed.*/  /'   | awk '{p+=$2; f+=$3} END {print p" passed, "f" failed"}'
```

Or simply look for `FAILED` and `^---- `. A summary line is not a check; it is a claim, and
this one is exactly the shape `CLAUDE.md` warns about everywhere else - a clean, confident
number from an instrument nobody verified.

## Reverse engineering

**`docs/ghidra.md`** — the working command line, JDK 21, the scripts, and the traps.
Two that bite immediately:

- **The Ghidra project locks.** One process at a time. If you spawn subagents while holding
  it, tell them not to use it and hand them the text already in `research/`.
- **Field order from the listing, field meaning from the decompiler**, and cross-check the
  packet-read count between them first. When they disagree, the instrument is wrong.

The reference source at `C:\Users\user\Desktop\ModernMapleSource` is a **different game
version**. Good for naming fields and predicting structure; worthless for settling anything.
Scored against a held-out control it got **1 of 8**. Label every claim from it as a candidate.

## Where output lands

| | |
|---|---|
| `login.log` | every packet both ways on the login connection |
| `world.log` | the same for **channel 0** — read this for anything past character select |
| `world-ch1.log` | channel 1. Two channels run by default; channel N logs to `world-ch<N>.log` |
| `client-patched\maplecw-hook.log` | `WATCH` lines, session patches, client faults |
| `client-exit.log` | how the client died |
| `research/` | decompilation as `msexe-<topic>.c`, findings as `.md` beside it |
| `research/fixtures/` | runs worth keeping, named for what they prove |
| `gm-handbook/` | game data **generated** from the client's WZ — maps, items, mobs, NPCs, portals. Gitignored; regenerate with `tools/dump_names.py` and `tools/dump_portals.py`. Never hand-edit, never commit |

## Reporting

Be plain about what is measured and what is inferred. This project has repeatedly committed
a plausible inference as a fact and paid for it later — the channel cipher took three passes
because a measurement of one direction was written up as covering both. If you retract
something, say what the evidence actually was and why it did not support the claim.
