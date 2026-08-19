# Ghidra, for whoever picks this up next

Everything here has been rediscovered at least once. It is written down so it is not
rediscovered again.

## The command that works

From **PowerShell** (`pwsh` is not installed on this machine — see `STATUS.md`
housekeeping). Copy this whole block:

```powershell
$env:JAVA_HOME="C:\Program Files\Eclipse Adoptium\jdk-21.0.6.7-hotspot"
$env:PATH="$env:JAVA_HOME\bin;$env:PATH"
& "C:\Users\user\Desktop\ghidra_12.1.2_PUBLIC\support\analyzeHeadless.bat" `
  "C:\MapleCW\research\ghidra" msexe `
  -process MapleStory.exe -noanalysis `
  -scriptPath "C:\MapleCW\tools\ghidra_scripts" `
  -postScript DecompileFunc.java "<out-file>" 142097f80 140304b20
```

| piece | why |
|---|---|
| `JAVA_HOME` → **JDK 21** | under JDK 25 the bundled Felix aborts with "The data file must be inside the data dir". If it has already run under 25, delete `%APPDATA%\ghidra\ghidra_12.1.2_PUBLIC\osgi\felixcache` |
| project `research/ghidra`, program `msexe` | ~1.2 GB, gitignored, **already analysed**. `grap64`, `mssecure` and `nexoncm` are there too. Never re-import |
| `-noanalysis` | analysis is done; without this you wait many minutes for nothing |
| output to a **file** | Ghidra's logger flattens multi-line output, which mangles decompiled C. Never scrape stdout |

Expect **1–3 minutes** per invocation regardless of how little you ask for — the cost is
opening the program, so **batch addresses into one run**.

## One process at a time

**The project locks.** A second `analyzeHeadless` against `msexe` fails or blocks.

This matters most when working with subagents: if you spawn agents while you hold Ghidra,
**tell them explicitly not to use it** and give them text that is already on disk. Every
disassembly and decompilation this project has produced lives in `research/`, so most
agent-shaped work needs no Ghidra at all.

## The scripts

In `tools/ghidra_scripts/`:

| script | use |
|---|---|
| `DecompileFunc.java` | addresses → C. `+callees` follows one level. **`@listfile`** reads one hex address per line — needed because cmd.exe truncates a command line at 8191 characters, which is fewer than a few hundred addresses. Creates a function when Ghidra never made one |
| `DumpAsm.java` | `<out> <addr> <length>` — the raw listing with hex bytes. **The authority when it matters**; see below |
| `Xrefs.java` | callers *and* data references |
| `DumpData.java` | bytes as ASCII and UTF-16 — turns an unnamed `&DAT_…` into a string |
| `DumpOpcodes.java`, `DumpPacketFields.java`, `FindStringRefs.java`, `FindConstArgCalls.java`, `FindFastFail.java`, `DumpExports.java` | as named |

Several `-postScript` flags can be chained in one invocation.

## The decompiler lies about order; the listing does not

This has cost real time twice. The rule:

> **Field order from the listing. Field meaning from the decompilation.**

The decompiler reorders freely once it reaches string handling or vector growth, and it
reconstructs control flow in ways that duplicate call sites. The listing is linear and
literal.

**Always cross-check the packet-read count between the two before trusting either.** If
they disagree, something is wrong with your instrument and not with the binary:

* `FUN_142097f80` — 50 in both. Safe to read.
* `FUN_140304b20` — 122 decompiled against 117 in the listing. The gap was **mine**: a
  `JMP` thunk the listing grep could not see, plus a primitive nobody had counted. Once
  both were included, both instruments said **126**.

## Bound every dump by `.pdata`

`DumpAsm` takes a length, not a function, and happily runs past the end into whatever is
next. On 2026-08-19 a scan of the mob-spawn decoder `FUN_141d33630` (1582 bytes) was dumped
with a length of `0x1200` and reported **34 packet reads**. There are **7** - the other 27
belonged to neighbouring functions. The number looked entirely plausible for a mob spawn.

```bash
python tools/pdata_lookup.py 0x141d33630     # -> 0x141d33630 .. 0x141d33c5e (1582 bytes)
```

Get the bounds first, dump that length, and filter the results to the range as well.

## Enumerate before you filter

The two worst mistakes on this project have the same shape: an instrument that searches for
a *known list* reports only what is on the list, and returns a clean, confident number
either way.

* Scanning for a presence mask with `BT` and `TEST reg,imm` found nothing, so the record was
  declared to have no mask. The mask is a **byte array**, tested with `CMP byte ptr [r],0`.
  Wrong **shape**.
* Counting packet reads by grepping five known decoder addresses missed a **sixth** (a `u64`
  reader) and a **thunk** to a seventh. Wrong **set**.

So: before counting calls to "the decoders", list *every* call target in the range and look
at what is actually there.

```bash
grep -oE "CALL +0x1406e[89][0-9a-f]{3}" listing.asm | sort | uniq -c | sort -rn
```

The **eight** packet-read primitives, as of 2026-08-19:

| address | reads |
|---|---|
| `0x1406e8ae0` | u8 |
| `0x1406e8b80` | u16 |
| `0x1406e8c20` | u32 |
| `0x1406e8ef0` | **u16** - a bare `JMP 0x1406e8b80`. **Added 2026-08-19.** |
| `0x1406e8f00` | u32 - a bare `JMP 0x1406e8c20`, invisible to a search for the target |
| `0x1406e8f10` | u64 |
| `0x1406e9050` | string: u16 length, then that many bytes |
| `0x1406e9170` | n raw bytes, `n` in `R8D` at the call |

**This table said seven until 2026-08-19, and the missing row was found by a disagreement,
not by a search.** Two instruments gave 50 and 52 reads for `FUN_141c4ff80`, the mob's
`encodeInit`. Sweeping *all* 116 direct call targets in that function - enumerating rather
than filtering against the known list - turned up `0x1406e8ef0`, five bytes of
`e9 8b fc ff ff`, a `JMP` to the u16 primitive. With it, both instruments say 52.

**Two thunks now, so "grep for the primitive addresses" undercounts twice.** Any scan built
on a hard-coded list of read primitives has to carry both, and the honest way to find a
third is to enumerate every call target in the function and ask what each one is.

**What the eighth primitive does NOT touch, checked rather than assumed.**
`python tools/callers.py 0x1406e8ef0` gives **35 call sites in 13 functions**, and none of
them is `FUN_140304b20` (the character record), `FUN_140304100` or its sub-decoders (the
equipped item), or `FUN_141f6f350`/`FUN_141f6fb20` (the script message). So the layouts
built on those walks stand. The positive control for that negative is in the same output:
`FUN_141c4ff80`, the function that exposed the thunk, is in the list.

## "No function there" is the normal state

Auto-analysis only creates functions it can reach, and code reached only from the
Themida-virtualised dispatcher has no caller in `.text` — so **the interesting handlers are
exactly the ones missing**. `.pdata` still bounds them; `DecompileFunc` creates one, and
`tools/pdata_lookup.py` gives exact bounds when you need them first.

Likewise `halt_baddata()` in decompiled output is a **lead, not a wall** — twice it was a
tail jump Ghidra did not follow, and `DumpAsm` showed it immediately.

## Python tools that answer faster than a headless run

Reach for these first; a headless run costs minutes and these cost seconds.

| tool | question |
|---|---|
| `tools/pdata_lookup.py` | which function contains this address, and what are its exact bounds |
| `tools/xref.py` | what takes the address of this string or symbol (**`lea` only** — see below) |
| `tools/dataref.py` | what **reads, writes or tests** this global — the half `xref.py` cannot see. `--writes` alone usually finds a singleton's constructor and destructor |
| `tools/callers.py` | every `call rel32` to a function - the question **neither** of the two above can answer |
| `tools/dispatchers.py` | which functions are inbound packet dispatchers, by call-graph shape |
| `tools/switch_cases.py` | every case of a decompiled switch, **including inlined bodies**, brace-depth aware so nested switches are not merged in. Has been **wrong twice**: once dropping 94 of 273 cases outright, once reporting the wrong *shape* for 42 of 273 because it tested `depth == want` for case bodies as well as case labels and so discarded everything nested in an `if`. Both were invisible in its output. Re-run it rather than trusting a committed table |
| `tools/dump_portals.py` | portals, NPCs and mob spawns out of `Map.wz` into `gm-handbook/` - game data regenerated from the client rather than typed into source |
| `tools/rtti.py` | class names and vtables from RTTI — but there is **no `CStage`, `CLogin` or `CField`**; the stage classes carry none |

`tools/xref.py` returning "0 references" means *nothing takes its address*, not *nothing
uses it*. That blind spot hid the "trouble logging in" raiser for three sessions.
`tools/dataref.py` exists to close it.

**Neither of them sees a `CALL`.** `xref.py --callers --va 0x1402fa9a0` returns
"0 code references" for a function with **43** call sites - `--callers` annotates
`--string` results and does not scan `call rel32` at all. `tools/callers.py` does, and its
docstring carries the positive control: 43 calls, all in `0x140304b20`, first `0x140304e49`,
last `0x1403091e7`. Run the control before believing any zero from any of the three.

`tools/dataref.py` had a matching hole: its opcode table lacked `0xC6`
(`mov byte [rip+d], imm8`) *and* `--writes` filtered on a label the opcode would not have
carried, so it reported **1** write to the character-record flag table where there are
**41**. Two independent filters, both dropping the same evidence, both silently.

## Traps particular to this binary

* **Identical functions are folded** (`/OPT:ICF`), so one `return;` stub's address appears
  in 784 unrelated vtables. **Shared vtable entries are not evidence of a shared base
  class.** Aligning vtables on that produced 761 confident, wrong candidates.
* **Strings are spliced with control characters** to defeat search — the channel-change
  message is `"Channel"` with a CR, a TAB, a CR and an LF through it. A string search
  failing is weak evidence.
* Themida sections (`.themida`, `.vm_sec`, `.boot`) hold VM bytecode. The original x86 for
  the packet dispatcher no longer exists. **But the switches it calls are ordinary readable
  code** — "the code that reaches it is unreadable" stood in for "the code is unreadable"
  for months. See `research/msexe-gamestage-dispatch.md`.

## Where the output goes

`research/`. Decompilation as `research/msexe-<topic>.c`, findings as `.md` beside it.
Keep the `//===` header format the existing files use — several tools parse it.
