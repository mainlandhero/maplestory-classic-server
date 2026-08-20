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

**Always write that path in full, quoted, in anything the owner will run.** An elevated window
opens in `C:\Windows\System32`, not in the repo, so a relative `tools\test-server.ps1` is
not a shorter way of saying the same thing - it is a command that fails on their machine and
works on yours. Same for any `-Probe`, `python tools/...` or `git` line handed over to be
pasted: full path, or say explicitly which directory to be in first.

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

## A run's output is evidence. Do not destroy it.

`world.log` used to be deleted at the start of every launch. **Four conclusions have turned on
that file and three of them died before they could be checked** - including one that was
reported to the owner as decisive, retracted a day later as unmeasurable, and finally proved right
by a log that only existed because the launcher had stopped deleting them.

`tools/test-server.ps1` now moves the previous run into `previous-runs/` (gitignored,
timestamped) instead of removing it. **Copy anything that settles a question into
`research/fixtures/` under a name that says what it proves**, because `previous-runs/` is
itself a rolling buffer.

The rule behind it: a launch costs the owner a manual launch, so its output is the most expensive
data this project produces. Never trade it for disk space, and **never state a conclusion
drawn from two different sessions as though one capture showed both halves** - that is exactly
how the mob-targeting answer went right, then wrong, then right again.

## Built is not wired

Several subsystems have been fully decoded, implemented, tested - and never connected, so on
screen they look identical to not existing. Quest state and the channel-list fix were both in
that state for a day while `STATUS.md` listed them as done.

When an agent hands back a "wire it like this" section, either wire it or record loudly that
it is unwired. `STATUS.md`'s START HERE marks these explicitly; keep doing that.

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

**Two scans agreeing is not corroboration when they share a blind spot.** On 2026-08-20 a
mob-range write-scan and a whole-image write-scan both reported that nothing sets
`mob+0x42c`. They agreed, and they were wrong the same way: a `[reg+disp]` write-scan cannot
see a store through a pointer that was `lea`'d and handed off, and the setter does exactly
that - `141cb4647 lea rdx,[r15+0x42c]`. Dropping the `--write` filter found it in one call.
Widening the *input* to an instrument with a structural blind spot only makes the blind spot
bigger, so re-running the same tool is not a second opinion; **changing the question is.**

The same day `tools/callers.py` was found to scan for `call` and nothing else. Of 120 981
functions, **27 909** would have come back "zero callers" while being reached by a tail `jmp`
(6 927) or a pointer in data (21 889) - and one of them had already been written up as
"zero direct callers, therefore virtual", which was wrong twice over.

## "Not found" is not "not there", and the retraction can be the mistake

On 2026-08-20 a static pass reported that a mob's body rectangle could not be involved in
touch damage. It said so carefully: the negative was tagged **[D]**, and it **named its own
blind spot**. On the strength of it a correct claim was withdrawn, and the owner was told not to
expect a snail to hurt them. Ten minutes with the client: *"I'm taking damage, and the mob is
also taking damage."* One field explained both directions all along.

The rest of this file is about not believing confident positives. This is the mirror: **a
carefully-hedged negative is not a licence to retract a measurement.** What the pass had
found was one path; what it had not found was another, and it said so.

Two habits fall out. When a report's negative contradicts something already seen on screen,
**the screen wins**. And when an agent labels a finding `[D]` with a named blind spot, quote
that blind spot when acting on it - if the sentence cannot be written down, the finding is
not strong enough to retract anything.

The same day, an agent swept for an immediate `0x12` near a state setter, found nothing, and
**refused to report it**: the setter's one call site passes a computed register, so an empty
result was the only possible outcome. That is the standard.

## The unit, not the arithmetic

Three separate bugs this month were a correct number in the wrong unit, and all three read on
screen as "the feature is broken":

* `0x03F0`'s mob HP was sent as an absolute; the field is a **percentage**. 27 of 45 drew a
  bar at 27%.
* A quest's progress count was going to be sent as an integer; the client stores **three
  zero-padded decimal characters per slot**, and an integer renders as nothing at all.
* A mob's size was sent as `0` meaning "unset"; `0` means **zero percent**, and it collapsed
  every mob's hit box onto its own centre.

In each case the doc block above the builder confidently described the wrong unit. **When a
number reaches the client and draws wrongly, suspect the unit before the arithmetic** - and
when a field's meaning comes from a comment rather than a listing, check the listing.

## The scratchpad shadows the real tools


The session scratchpad has accumulated **copies of the repo's own instruments** - `reads.py`,
`dis.py`, `callers.py`, `rangescan.py`, `poolscan.py`, `encodes.py`, `listing.py` and more -
left behind by earlier work. Python puts the script's own directory first on `sys.path`, so
**a script run from the scratchpad imports those instead of `tools/`**, silently.

This is not hypothetical and it is not merely untidy. The scratchpad's `reads.py` is 3359
bytes from an earlier session; `tools/reads.py` is 8500 bytes. The old one **does not contain
`0x142d23ef0`** - the tenth read primitive, the one that sits 6 MB from the other nine and was
found only by enumerating every `jmp` into a reader instead of searching the neighbourhood.
A read walk that imports it comes back **short, clean and confident**, which is exactly the
failure that shipped a truncated chat packet and killed the client **twice**.

Two habits, either is enough:

* Run repo tooling with **the repo as the working directory** - `python tools/reads.py ...`.
* When a script must live in the scratchpad, pipe it in rather than naming it:
  `python - args < script.py`, which leaves `sys.path[0]` empty.

And the general form, which is the same rule this file makes everywhere else: **an instrument
that is silently the wrong version is worse than one that is missing.** A missing tool raises;
a stale one answers.

## Working alongside agents

An agent's files are **its** files until it reports back. Three rules, all learned the same
day by breaking them.

**Never `git add -A` while an agent is running.** Stage by path, and stage only paths you
touched yourself. A green test suite is not evidence that an agent has finished - it is
evidence that whatever it has written so far compiles. On 2026-08-19 this committed
half-written work from three different agents, and in one case a file was still being edited
as it went into the commit.

**Do not edit a file an agent owns.** Give each agent its own new files, tell it which shared
files are off-limits, and integrate yourself once it reports. `crates/world/src/session/`
is the usual integration point and should stay with the coordinator.

**Re-run your own analysis when you fix a shared instrument.** If a tool was wrong, every
conclusion drawn with it is suspect - including yours. On 2026-08-19 `tools/reads.py` was
fixed to count tail-call reads, four agents were told to re-run anything resting on a read
count, and the person who sent that message did not re-run their own. The result was the same
packet shipping short a second time and killing the client a second time. The fixed tool's
own output had already printed the missing read; it scrolled past unread.

**A cargo run made while agents are working is not evidence.** Every agent shares one
`target/`, and concurrent invocations race the fingerprints. On 2026-08-20 `cargo build -p
store` succeeded, and the `cargo test --workspace` immediately after it failed with *"struct
`Character` does not have a field named `exp`"* - naming a field that was on disk, in the
right file, and had just compiled. The compiler was reporting a **stale sibling crate**:
`available fields are: ... and 17 others` counted 22 where the source had 23. Re-running it,
unchanged, passed.

The failure direction is the harmless one. The same race can hand back a **clean pass against
code that is no longer there**, which looks exactly like a green suite. So: when a build
result is surprising, re-run it before believing it, and **re-run the whole suite once the
agents have reported** - that run is the one that counts. A `cargo` result is an instrument
like any other here, and this is the same rule as everywhere else in this file.

## The test summary that could not count failures

`cargo test --workspace | awk -F'[ ;]' '{p+=$4; f+=$6}'` **always reports zero failures.**
With `;` as a separator, `0 failed` lands in field 7 and field 6 is the empty string between
`passed;` and the space. It was used to report "N passed, 0 failed" repeatedly on 2026-08-19
before anyone noticed the counter was structurally incapable of returning anything else.

Count with something that reads the word, not the position:

```
cargo test --workspace 2>&1 | grep -E "^test result:" | awk '{for(i=1;i<=NF;i++){if($i=="passed;")p+=$(i-1); if($i=="failed;")f+=$(i-1)}} END {print p" passed, "f" failed"}'
```

That reads the number *before* the word `passed;` / `failed;` instead of trusting a field
index, so a change in the line's punctuation cannot silently zero it. It also contains no
backslashes, and that is deliberate: the first version of this section used `sed` with
backreferences, and the heredoc mangled `\1 \2 \3` into raw control bytes exactly as the
shell note above says it would. **The documented fix for a broken instrument was itself
committed broken.** Check a code block after writing it, not just the prose around it.

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
| `previous-runs/` | the last few runs' logs, archived by the launcher instead of deleted. A rolling buffer - gitignored |
| `research/fixtures/` | runs worth keeping, named for what they prove. Copy from `previous-runs/` before it rolls |
| `gm-handbook/` | game data **generated** from the client's WZ — maps, items, mobs, NPCs, portals. Gitignored; regenerate with `tools/dump_names.py` and `tools/dump_portals.py`. Never hand-edit, never commit |

## Reporting

Be plain about what is measured and what is inferred. This project has repeatedly committed
a plausible inference as a fact and paid for it later — the channel cipher took three passes
because a measurement of one direction was written up as covering both. If you retract
something, say what the evidence actually was and why it did not support the claim.
