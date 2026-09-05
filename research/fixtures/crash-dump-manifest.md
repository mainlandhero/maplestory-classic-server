# The crash dumps, what each one settled, and which were pruned

**13.4 GB across twelve full-memory dumps, pruned to three on 2026-09-05** at Wisp's
instruction, after their machine began micro-freezing with kernel time running above user time.

A dump is **unreproducible without another crash**, so this file exists before the deletion
rather than after it. Every row says what the dump was read for and where that reading now
lives. If a row's finding is ever doubted, the answer is not "re-open the dump" — it is
another client run, which this project can produce.

Measured before deleting: `dumps/` was 13.8 GB of a ~31 GB repo, second only to `target/` at
15.1 GB and 48 176 files. Removing nine dumps returns ~10.5 GB.

## Kept

| dump | GB | why it is kept |
|---|---|---|
| `maplecw-crash-1010344-c0000005-1.dmp` | 1.2 | The **newest** `0xC0000005` pair, and the one that settled the seat index at body 416. Its reader-object walk is the technique every later dump analysis copied |
| `maplecw-crash-992840-c0000005-1.dmp` | 1.4 | Its counterpart, the other client in the same run. **Two dumps agreeing register-for-register** is what made `RAX = 8` a measurement rather than one observation — keeping only one would destroy that property |
| `maplecw-crash-1007028-c0000374-1.dmp` | 1.2 | The only `0xC0000374` **heap corruption** kept. That family has ~17 distinct events across the archive and no fix; this is the two-client death at 371 s of life |

## Pruned

Each was read, its finding written down, and the finding cited from the file named.

| dump | GB | what it settled | where the finding lives |
|---|---|---|---|
| `maplecw-crash-1003900-c0000005-1.dmp` | 1.4 | `0x140f9295e` is `mov rcx,[rax]` with `RAX = 8`; the accessor **reports** an out-of-range index and honours it | `research/0x0224-remote-user-first-use-fault.md` |
| `maplecw-crash-990836-c0000005-1.dmp` | 1.2 | Its counterpart — the register-for-register agreement across two processes | same |
| `maplecw-crash-970152-c0000005-1.dmp` | 1.2 | The reader's position and requested length at the throw: 681/678, 3 remaining, 4 requested | `research/0x0224-dump-read-position.md` |
| `maplecw-crash-970748-c0000005-1.dmp` | 1.3 | Its counterpart; the count recovered from the loop counter, 116 iterations, `467 = 4x116 + 3` | same |
| `maplecw-crash-967104-c0000005-1.dmp` | 1.2 | The first `0x0224` death — died **inside** the handler, no dispatch line on return | `crates/net/src/userpool.rs` (`REMOTE_STAT_TAIL_LEN`) |
| `maplecw-crash-917060-c0000374-1.dmp` | 1.2 | Heap corruption at 493 s in a client that had **no instance hooks**, which is what cleared the hook of causing it | `research/fixtures/instance-guard-armed-second-client-created-hook.log` |
| `maplecw-crash-1020604-c0000374-1.dmp` | 0.9 | Heap corruption, same family | `STATUS.md`, heap section |
| `maplecw-crash-702484-c0000005-1.dmp` | 1.3 | An older access violation, pre-dating the two-client work | `research/heap-crash-pattern.md` |
| `MapleStory.exe.826924.dmp` | 0.2 | **Cited by nothing.** A WER-era dump from before the hook wrote its own | — |

## The one thing that would be lost

`research/0x0224-dump-read-position.md` records that its analysis anchored on finding the
**known body bytes** inside the dump — each process held its counterpart's `0x0224` body,
byte-for-byte identical to `research/fixtures/0x0224-bodies-that-killed-both-clients.txt`,
which is what made the reader's position an offset into a known buffer rather than an
unanchored number. That anchoring cannot be redone on a pruned dump. **The bodies themselves
are kept** in that fixture, so a future run can re-establish it.
