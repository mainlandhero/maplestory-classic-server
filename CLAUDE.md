# MapleCW

A private, local-only server emulator for the owner's own copy of the MapleStory "Classic World" client.
Rust workspace, SQLite, patched client in `client-patched/`. Testing only.

**Start at `STATUS.md` → NEXT GOALS.** It is kept current; if it contradicts this file, it wins.

## Standing constraints

- **Never store passwords in plain text.** argon2id, salted. Session tokens stored as SHA-256 only.
- The firewall rule is scoped to the patched executable only.
- `client-patched/` exists so the original client is never touched.
- **Nothing authenticates.** The game socket carries no credentials. Say so whenever reporting progress.
- **Never renumber characters from 1.** Ids start at 200 (`FIRST_CHARACTER_ID`); a create reply carrying id 1 made the client silently refuse to transition.
- **`grap_stub::session::refresh_select_after_dispatch` is load-bearing.** The client builds
  its character-select screen once, from whatever character list exists at that instant; on a
  fast start that is 30 ms after the login request, before the list, and the mode-2 login
  handler never refills it - blank avatars. The hook now makes the client's own refill call
  after every `0x0010`. Confirmed 2026-09-12 after three wrong theories
  (`research/charselect-avatar-fade-race.md`). Do not remove it, do not gate it, and never
  put a `-Probe` watch on `141177e40` - its int3 is the byte the guard reads. A test fails if
  `hook.rs` stops calling it.

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

## The test plan lives in `tools/test-server.ps1`, and it is in TWO places

Not in `STATUS.md`. It is in the launcher, so the steps and the thing that launches them
cannot drift apart, and **the owner reads it off their console at launch** rather than opening a
file.

**There are two copies in that one script and both must be updated together:**

| where | how it is read |
|---|---|
| the `.NOTES` block at the top | `Get-Help`, and by anyone opening the file |
| the `Write-Host` blocks in the body | **printed on screen at every launch** - this is the one the owner actually sees |

On 2026-08-21 the `.NOTES` block was maintained for days while the `Write-Host` block still
told the owner to test the **inventory bag sizing**, to pass `-InventorySlots 10`, that mobs were
opt-in and unconfirmed, that typing in chat might kill the client, and **not to click Change
Channel**. All of that was from 2026-08-19 and every word of it was wrong. They pasted it back
with "I still see a whole bunch of bloat", which is how it was found.

So: **update both, then render the dialogue and read it.** A parse check is not enough - it
does not catch a quoting bug that mangles the text, and it certainly does not catch a plan
that is simply out of date. Extract the `Write-Host` block into a scratch `.ps1` with `$root`
and `$SetFieldProbe` defined and run it.

Two habits that fall out:

* **Strike finished items off.** A plan that still lists what was confirmed two runs ago
  spends the owner's launch re-testing things nobody is asking about.
* **Say what each outcome MEANS, not just what to do.** "Does it pop or fade?" with the two
  readings written down turns a run into a measurement. Every step should be a claim that
  can come back false.

## Client runs cost the owner a manual launch

Measure first. `python tools/channel_smoke.py` and `tools/login_smoke.py` exercise the real
servers over an independent Python transport — framing, cipher, every field offset — without
a launch. Only run the client when nothing cheaper can answer the question.

When a run happens, **test one variant at a time**. Changing two things at once has already
produced one unexplained crash. The owner can see the GUI and you cannot, so say exactly what to
watch for and what each outcome would mean *before* they launch.

## A run's output is evidence. Do not destroy it.

`world-ch0.log` used to be deleted at the start of every launch. **Four conclusions have turned on
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

### The corollary: `research/fixtures/` is COPIES, so counting across both double-counts

`fixtures/` holds copies of `previous-runs/` files under names that say what they prove. That
is the right design and it has one consequence nobody wrote down: **a glob over both
directories counts a capture once per name it has.** 155 world logs on disk are **119 distinct
files**, and `world-20260822-213856.log` exists under three names.

On 2026-08-28 this produced the wrong numbers *in the write-up of an instrument lesson*. The
skill-id enumeration was reported as 689 swings, 2 Three Snails and 14 Magic Claws. The real
counts are **426, 1 and 7**; two of the three were exactly doubled by fixture copies, and the
control that reads as "two independent observations" is **one packet**. An agent caught it by
hashing first.

Two habits:

* **Deduplicate by content hash before counting anything across those two directories.**
  `tools/extract_attack_bodies.py` does it and exits non-zero on an empty result.
* **And a content hash is not enough either.** Found the same day, by an agent re-deriving the
  mob-move counts: `research/fixtures/skill-window-close-faults-world.log` and
  `previous-runs/world-20260820-181822.log` are **the same run**, and hash differently - they
  differ at character 74 of line 1 and by 41 KB of tail, because a fixture is copied while the
  run is still being written. **Eleven such pairs.** File-level deduplication called that two
  observations; deduplicating on `(timestamp, opcode, body)` gives **131 003** mob-move
  reports where the file-level count said 243 418, and **257** user-hits where it said 435.
  The rule is: **deduplicate the EVENTS, not the files.**
* **A fixture's name says what its author was looking at, not everything the file contains.**
  Both captures that settled the skill id had been sitting in `fixtures/` for days, under
  `magic-claw-1-damage-and-heapfix-armed-` and `cash-shop-click-sent-nothing-`. Grep the
  contents; do not scan the names.

And the reason this is worth a section rather than a footnote: **the finding was right and the
count was wrong, which is the more dangerous combination.** A wrong count does not fail loudly
the way a wrong conclusion does - it just quietly makes a one-observation control look like
corroboration.

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

**And on 2026-08-22 the filter was a *timestamp*, which is the same mistake wearing a clock.**
The owner's first sentence about the cash shop was *"the opcode is most likely not handled"*. It was
reported back to them three times, across three sessions, that **no packet was sent at all** -
"world-ch0.log records every inbound packet and there is nothing new in it". `0x00D5` was in that
log every time.

It survived because it arrives inside a six-opcode burst with `0x0420`..`0x0426`, and that
burst lands near the end of a session, so the **whole burst was filed as "shutdown telemetry"
as one object**. Nobody ever asked which opcodes were in it. The control costs one grep of the
archived runs and would have broken it on day one:

```text
                                    0x00D5   0x0420
  five runs, nobody clicked it          0      0..2     <- 0x0420 really is telemetry
  the two runs with a click             1        1      <- 0x00D5 appears nowhere else
```

The generalisation: **"nothing new arrived" is a different claim from "this thing did not
arrive", and only the second one is worth making.** When a user says an action produced no
packet, grep for the specific opcode the action's own code builds - `research/msexe-send-opcodes.txt`
names it - rather than eyeballing the tail of the log for something unfamiliar. A burst is not
an object; it is a set, and it has to be enumerated like one.

## A correctly-armed instrument can still be blind to its subject

Everything else in this file is about instruments that were *wrong*: stale, mis-scoped,
searching the wrong shape. On 2026-08-21 one was **right, and still could not see the
client**, and the difference matters because the two look identical from the outside - an
empty result.

Crash dumps had never been captured, across six heap-corruption deaths. The standing
explanation was that WER LocalDumps was misconfigured, and it had in fact been misconfigured
once. The way that got settled cost **no client run at all**:

> Build a decoy that does nothing but dereference null. **Name it `MapleStory.exe`**, because
> LocalDumps keys match on the executable's base name. Run it.

It wrote a 9.4 MB dump into `dumps\`. Meanwhile the real client had raised the *same*
exception code, `0xC0000005`, at 13:49:56 - **88 minutes after WER was switched on**, which
the registry key's own last-write time proves - and produced nothing.

Same machine, same hour, same executable name, same exception code. Every variable held but
one. The configuration was never the problem: **the client ships its own crash reporting, and
a process that handles its own faults never reaches `WerFault`.** No amount of configuring
would have fixed that, and "check `dumps\` after the next death" - which is what the test
plan said - was a step that could only ever come back empty.

Two things generalise:

* **A positive control has to be close to the subject to be worth anything.** "WER works on
  this machine" was already known and was useless. "WER works *for a process named
  `MapleStory.exe` writing to that folder on this machine at this hour*" is the control that
  localises the failure, and it is only one `rustc` invocation away from the useless one.
* **When the instrument is armed and still silent, move the vantage point rather than
  widening the instrument.** The hook's vectored exception handler was *already catching this
  exact fault* - it is what writes the `CLIENT FAULT` line - and was only logging it. The dump
  now comes from there. This is the same rule as the `mob+0x42c` write-scan: re-running the
  tool is not a second opinion, **changing the question is.**

The self-test in `crates/grap-stub/src/minidump.rs` exists for the same reason. A dump writer
that has never written a dump is exactly the kind of instrument this file keeps warning about.

## The thing you are comparing against may never have been a control

Three days went into *"mobs appear instantly and NPCs fade in - find the difference."* Two
static passes, a decompilation pass, a new opcode, two GM commands and three client runs came
out of that sentence. On 2026-08-22 the sentence itself was checked, in a log that had been
sitting in `previous-runs/` the whole time:

```text
01:20:57.582  -> 0x044F NpcEnterField   template 8, template 9      <- field entry
01:21:04.743  -> 0x03C6 MobEnterField   template 2, object id 2000  <- 7.16 s later
```

**The mobs were not in the field-entry batch.** They arrived from the respawn tick, seven
seconds after the map had settled, with nothing to be late against. The one archived run where
mobs *were* in the entry batch is map 10000022, which has no NPCs. **Nobody has ever watched an
NPC and a mob created at the same instant**, which is the only observation the whole comparison
rested on.

Everything downstream was still real work - the `0x044F` field enumeration, `0x0451`, `0x0452`
and its appear-effect object are all decoded and all correctly eliminated. But they were
answering *"why is A different from B"* when nobody had established that A and B differ.

The habit: **before hunting for the difference between two cases, check that both were observed
under the same conditions.** A remembered contrast is not a measurement. It is worth one grep of
an archived log, and here that grep was free and would have been decisive on day one.

### The corollary that has now paid twice: separate "this thing" from "this session" first

Both crashes the owner reported on 2026-08-22 arrived as *"X crashed the client"* - character
`GoodTest`, then a teleport to map `10001050`. Each was **one observation**, and each had the
same two readings: the thing is fatal, or the session had been running long enough for
something else to fire. They look identical on screen and they need opposite work.

The discriminator is the same both times and costs no launch of its own: **do it first, at
~40 s of client life.** `GoodTest` came back (b) - the character and its map were innocent,
and a day of "why is map 10 special" was avoided by one login. Map `10001050` is still open.

Two things make it worth writing down rather than rediscovering:

* **The answer went a different way each time**, so neither reading is the safe default. The
  temptation after the first result is to assume "it is always the session"; that is the same
  mistake in a new direction.
* **Do not start the static work before the experiment.** Naming the field at `[0 + 0x3530]`
  is one Ghidra pass and it is sitting there - but it answers *"why is this map different"*,
  which is precisely the question nobody has established has an answer yet.

The same pass produced the corollary, which is cheaper still: `research/` said `0x0467` was
"a template preload list", and it is `SetNpcScriptable` - `u32 templateId; str script;
u32 dateStart; u32 dateEnd` per entry. **A table row written from a quick read is a claim.**
Two separate briefs sent an agent after that row before anybody counted its reads.

## A guard whose answer is ignored is not a guard

The owner, 2026-08-21: *"I was able to complete the Heena quest multiple times, this is not okay."*

`store::complete_quest` guards on `state = InProgress`. `store::start_quest` is
`INSERT OR IGNORE`. Both had been correct since the day they were written, and both were
being **asked and then ignored**: the payout and the item grant sat *outside* the match on
the return value, so a repeat click re-paid `Act.1` and re-handed `Act.0`. The journal row
never changed - the database was right the whole time - and the screen still gave out two
experience per click, for as many clicks as anyone liked.

Three things generalise, and the third is the one that cost the most:

* **Every effect hangs off the transition, not off the request.** If the store says "nothing
  changed", nothing may follow. Return early on the refusal rather than gating each effect
  separately, because separately is how one gets missed.
* **A refusal that is reported to no one will be ignored eventually.** All three call sites
  captured the store's answer into a *log string* and then carried on. The answer looked
  handled.
* **A test that checks one of several effects gives false confidence about the rest.** The
  turn-in test counted **fanfares** - and the fanfare was the single effect that *was*
  correctly gated, so it passed on every run while the experience doubled beside it. When a
  handler produces N effects, the test has to say something about N of them, or name the ones
  it is not covering.

And a fourth, which is this file's oldest rule wearing new clothes: `record_quest_forfeit`'s
doc block said *"A forfeit undoes an acceptance. It must not silently wipe a completion the
player earned."* That sentence was about a flag on the wire. The `DELETE` underneath it had
no state predicate at all, so give-up on a finished quest removed the row and put the
character back to never having touched it - a complete farming loop out of one missing `AND`.
**A comment describing a guarantee is not the guarantee.** Put it where it is enforced, and
in this case that is the row, not the caller.

## A test that pins what the code already does is not a check

Writing that dump writer, `MiniDumpWriteDump` returned `ERROR_NOACCESS` on **every** call
that carried exception information - synthetic pointers or a live handler's, stack or heap,
pseudo process handle or a real one, from the faulting thread or another, at every dump type.
All measured, all identical.

There was a test beside it asserting the exception-info struct was **24 bytes, aligned 8**.
It passed. It had been written by reading the field list - `DWORD`, pointer, `BOOL` - and
`repr(C)` duly produced 24/8, so the test agreed with the code and neither agreed with
Windows. `minidumpapiset.h` wraps every `MINIDUMP_*` structure in `<pshpack4.h>`, so the real
layout is **16 bytes with the pointer at offset 4**. Packing it and changing nothing else
turned the same call into a 22 MB dump.

Two habits:

* **A constant that came from reading a header is a claim, not a fact** - the same rule this
  file already applies to field offsets and units. Assert it against something that can
  disagree: here, a call that fails.
* **The result that does not vary is the clue.** Six variables were changed and the error
  never moved, which is what finally pointed at layout rather than data. A failure that
  ignores its inputs is not being caused by them.

## A field whose meaning depends on a type byte will decode as garbage without saying so

The fourth retraction that was itself the mistake, and the first where **I** made it rather
than an agent, in a file that already carries three warnings about exactly this.

Two crash dumps of the same failure. The first found a damaged block header - `0x20` had
become `0x0000000100000020`, a stray `1` in the high dword - and honestly flagged "always at
`+4`, always `1`" as its single inference, saying a second dump would settle it. The second
dump arrived. `tools/dumpwalk.py` printed a header for it that looked nothing like the first,
so it was written up as *"differently garbled bytes, therefore a wild write, not one
repeatable off-by-one"* - and that went into `STATUS.md`, a fixture name, a commit message and
both copies of the test plan.

**`_HEAP_FAILURE_INFORMATION.Address` is not one thing.** For a **type 8** failure it is the
heap *entry*, `ptr - 0x10`. For a **type 9** it is the *pointer the caller passed*. The tool
decoded it as an entry either way, because nothing in the struct forces you to look at the
type first. Decoding a pointer as an entry XOR-decodes whatever the object happens to begin
with, and it **always produces a plausible-looking header**.

The bytes were `0072005000000010`:

```text
10 00 00 00 50 00 72 00
^^^^^^^^^^^ BSTR length prefix 0x10 = 16 bytes = 8 UTF-16 characters
            ^^^^^^^^^^^ "Pr"          -> the string "Property"
```

A string. Enumerating the allocator's own slots properly - `tools/poolchain.py`, which walks
every chunk and checks a size identity per chunk, 2 238 passed and 0 failed - shows **every
damaged header in both processes is the identical `0x0000000100000020`**, all five of them in
the same size class. The original finding was right the whole time.

Three things generalise:

* **Look at the discriminator before you read the union.** A struct with a `type` field has
  members whose meaning changes with it, and a decoder that ignores that never errors - it
  prints something. `dumpwalk.py` now refuses that decode unless the type says the field is an
  entry, and prints why instead.
* **A retraction needs a stronger instrument than the claim it retracts, not a weaker one.**
  The claim came from an exact enumeration of a pool; the retraction came from one line of
  incidental output. That asymmetry alone should have stopped it.
* **"Different" is a much weaker observation than "the same".** Two identical values across
  two processes is a measurement. Two values that merely fail to match can differ because one
  of them is not a value at all, which is what happened here.

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

## Count the same event in two logs

Three of this session's answers came from a **count mismatch between two files**, not from
reading either one closely:

* the **equip crash** - the hook writes one dispatch line per packet *on handler return*, and
  20 sibling `0x0070`s had one while the fatal one did not. The client died inside the
  handler.
* the **create button dead after Log Out** - `login.log` had **two** `0x0010`s and the hook
  log had **one** `called FUN_140c9e230`. Our `create=on` patch latched once per launch while
  the client's handshake zeroed the flag on every login.
* the **channel migrate opcode** - ten candidates went out, and the dispatch line for
  `0x001A` took **354 ms** where its neighbour took 64 µs. The long one is a socket teardown.

None of these is visible in a single log. `world-ch0.log` and `login.log` say what the *server*
sent; `client-patched\maplecw-hook.log` says what the *client* did with it. When something
"did nothing", count the event in both and compare - and remember the dispatch line is
written on **return**, so a missing one means the handler was entered and never came back.

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

### The full suite is heavy now - run it when publishing, not after every edit

The owner, 2026-09-18: the workspace is past 2 600 tests and a full `cargo test --workspace` is
no longer a quick check. **Run the full suite only when publishing the server files
(`tools/package-server.ps1` / `tools/make-installer.ps1`) and only when asked to.** For
ordinary work, run the crate or the filter that covers the change:

```
cargo test -p world -- beauty
```

A change to a shared crate (`net`, `store`) still needs the crates that depend on it, so
name them: `cargo test -p store -p world`. The publish-time full run is the one whose count
goes in the commit message and in `STATUS.md`; a filtered run's count is not a suite count
and should not be written as one.

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
| `world-ch0.log` | the same for **channel 0** — read this for anything past character select |
| `world-ch1.log` | channel 1. Two channels run by default; **channel N logs to `world-ch<N>.log`, channel 0 included** |
| `chat-hub.log` | the world hub `maplecw-chat` (port 8483): every channel's link, who is online where, every party request in the order it was applied |
| `client-patched\maplecw-hook.log` | `WATCH` lines, session patches, client faults |
| `client-exit.log` | how the client died |
| `research/` | decompilation as `msexe-<topic>.c`, findings as `.md` beside it |
| `previous-runs/` | the last few runs' logs, archived by the launcher instead of deleted. A rolling buffer - gitignored. **Files last written more than 7 days ago are deleted** by the world server when its log opens and daily after (`world::logprune`, the owner 2026-09-24) - copy anything worth keeping into `research/fixtures/` first |

**Channel 0's log was called `world.log` until 2026-09-14.** It is `world-ch0.log` now, so a
two-channel server does not read as one channel plus a mystery file. Anything already in
`previous-runs/` or `research/fixtures/` keeps the name it was written under - those are
archived files and a reference to one is not stale, so `...-world.log` in a doc comment or a
fixture name is correct and was deliberately left alone.
| `research/fixtures/` | runs worth keeping, named for what they prove. Copy from `previous-runs/` before it rolls |
| `gm-handbook/` | game data **generated** from the client's WZ — maps, items, mobs, NPCs, portals. Gitignored; regenerate with `tools/dump_names.py` and `tools/dump_portals.py`. Never hand-edit, never commit |

## Reporting

Be plain about what is measured and what is inferred. This project has repeatedly committed
a plausible inference as a fact and paid for it later — the channel cipher took three passes
because a measurement of one direction was written up as covering both. If you retract
something, say what the evidence actually was and why it did not support the claim.
