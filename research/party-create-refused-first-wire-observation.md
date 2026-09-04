# The first `0x00A5` on the wire carried its opcode **twice**, so the client read code `0xA5`

Written 2026-09-04, from the run archived as
`research/fixtures/first-0x00A5-on-the-wire-create-refused-by-client.txt` and
`research/fixtures/first-0x00A5-on-the-wire-hook.log`. **No Ghidra** (project lock held
elsewhere), **no new client run**.

Labels: **[L]** read off a listing, a log, the string table or the source; **[D]** derived from
two or more [L] facts that agree; **[I]** inferred, nothing here confirms it.

---

## 0. The answer, up front

`research/party-result-0x00A5.md`'s decode is **right**. `crates/net/src/party.rs::party_created`
builds **exactly** the 209 bytes that decode says, and I re-derived those bytes independently and
got the logged body byte for byte.

**The wiring is wrong, and by exactly two bytes.**

```text
  crates/net/src/party.rs:383   PacketWriter::with_opcode(PARTY_RESULT)   -> body = a5 00 0e 01 ...
  crates/world/src/session/mod.rs:43   Reply::packet()                    -> out  = a5 00 || body
  ------------------------------------------------------------------------------------------------
  on the wire                                                             a5 00 | a5 00 0e 01 ...
```

The framer strips the first `a5 00` as the opcode. `FUN_1413bab80`'s **first** packet read is the
`u8` code at `0x1413baf47`, and what sat there was the **second** `a5`:

```asm
1413baf47  call 0x1406e8ae0     ; READ u8 code   -> 0xA5
1413baf58  add  eax, -3         ; 0xA2
1413baf5b  cmp  eax, 0x2c
1413baf5e  ja   0x1413bcbb5     ; DEFAULT ARM
1413bcbb5  mov  edx, 0x12a      ; "Due to an unknown error, your party request failed."
```

That is the message the owner saw, and it is the only thing the client could have done with those
bytes. **[D]** from four [L] legs, §2.

`crates/net/src/party.rs` is the **only** file in `crates/net` that uses
`PacketWriter::with_opcode` for an outbound builder — two sites, `party_created` (383) and
`refusal` (410). Every other builder in the crate returns body-only and lets `Reply::packet()`
supply the opcode. **[L]**, `grep -rn with_opcode crates/net/src/*.rs`.

**Nothing about the packet's contents is in question.** Send the same 209 bytes with the opcode
appearing once (§4).

---

## 1. Instruments, and the controls they passed first

| instrument | control | result |
|---|---|---|
| `python tools/reads.py 0x140304100 2` | the documented control in its own docstring | raw @`140304138`, u8 @`140304144`, u8 @`140304183`, then a run of u16, mixed direct/via-helper | **PASS** |
| `python tools/listing.py 0x140304100` | must agree at the same addresses | same four, same addresses | **PASS** |
| `python tools/dump_stringids.py --grep "Cash Shop"` | ~19 of 6 165 | 19 | **PASS** |
| the `.text` immediate scan (§2 leg 4) | `0x129` must land on `0x1413b9fc1`; `0x10c` on `0x1413bbc80` | both, plus 13 and 7 others | **PASS** |
| `cargo test -p net party` | must run at all | **14 passed, 0 failed** — see §3, they all pin the bug | **PASS** |

The immediate scan disassembles the whole of `.text` (`0x140001000..0x14326194a`, 52.8 MB) with
one-byte resync, rather than scanning a neighbourhood or a known list. `CLAUDE.md`: *enumerate
before you filter*.

---

## 2. The four legs

**Leg 1 — the source.** `PacketWriter::with_opcode(op)` is `new()` then `w.u16(op)`
(`crates/net/src/packet.rs:22`). `Reply::packet()` is
`opcode.to_le_bytes() ++ body` (`crates/world/src/session/mod.rs:43`, prepend at `:45`). `server::send` frames
`reply.packet()` (`crates/world/src/server.rs:125`, `:151`, `:193`) — **the only send path in the
world server; `grep -rn "\.packet()" crates/world/src` finds those three and nothing else.**
`party_created` and `refusal` are reached only through `Reply.body`
(`crates/world/src/session/party.rs:148`, `:101`, `:171`, `:292`, `:306`). **[L]**

**Leg 2 — the run says the body carries the opcode, and this is a measurement, not a re-read of
the source.** `send` logs `body_hex(opcode, &packet[2..])` — it *drops* the two bytes it
prepended. So the hex in `world.log` **is** the builder's `body`. That line begins:

```text
23:03:38.767  body a5000e010000000000000000000000000000000000000000d6000000 0700 "Tester2" ...
                    ^^^^^ the opcode, inside the body
```

**Leg 3 — no other opcode in the same file does that.** In the same run:

```text
0x0453 NpcChat    body e9030000ff0300000000     -> starts e9 03 = npc object 1001, not 53 04
0x03E4 MobCtrlAck body d30700007f00010000...    -> starts d3 07 = mob 2003,       not e4 03
0x00A5 PartyResult body a5000e01...             -> starts a5 00 = ITS OWN OPCODE
```

One convention in the codebase, and `0x00A5` is the exception. **[L]**

**Leg 4 — the string can come from nowhere else.** `"Due to an unknown error, your party request
failed."` is string id **`0x012A`** (`tools/dump_stringids.py --grep "unknown error"`, entry 298).
Scanning every `mov reg, imm` in `.text` for that immediate gives **10 sites**; disassembling all
ten, **exactly one** hands it to the string resolver `FUN_1408a9e40`:

| site | what it actually does |
|---|---|
| `0x140a13557`, `0x140a2ec04`, `0x140a488f1`, `0x140aa0838` | `call 0x1402bf6d0` — a predicate, not the resolver |
| `0x140c93882` | `call 0x14090d240` — likewise |
| `0x1418e2e7b`, `0x1418e2f30` | `call 0x1406ed9d0` — **encoding 0x12A into an outbound packet** |
| `0x142cc814d` | `call 0x1406ed520` — `COutPacket(0x012A)`, an outbound opcode |
| `0x1422e79af` | `call 0x1423bba00` — not the resolver |
| **`0x1413bcbb5`** | **`call 0x1408a9e40` then `0x1415eca30` — resolve and print. `FUN_1413bab80`'s default arm.** |

Controls on the same scan: `0x129` (*"the party is full"*, local refusal) lands on `0x1413b9fc1`
exactly where `research/party-request-payloads.md` §5 puts it, and `0x10c` (*"You have created a
new party."*) on `0x1413bbc80` exactly where `research/party-result-0x00A5.md` §5.1 puts it.
**[L]**

**Named blind spot:** a *computed* `0x12A` (`mov edx, ebx` where `ebx` was derived) is invisible to
this scan — the same blind spot `CLAUDE.md` records for the `0x12`-near-a-state-setter sweep. It
does not weaken the conclusion here, because the conclusion does not rest on the negative: leg 1-3
say what the client was handed, and the arm it lands on is a two-instruction bound check.

### The `0x0E` arm could not have produced that message even if it had run

`0x1413bbb8d`..`0x1413bbcec`, read end to end: after the reads, three separate `je 0x1413bbc80`
all converge on `mov edx, 0x10c` and then `jmp 0x1413bc767`. **There is no path from the `0x0E`
arm to `0x1413bcbb5`.** If the arm runs, the client says *"You have created a new party."* and
builds the party window (`operator new(0x378)` at `0x1413bbc67`, ctor `0x14118ace0`). **[L]**

### What 19.8 ms does and does not tell you

`elapsed_us=19828.2` was read as "it did substantial work". It is the third-largest dispatch in
the whole 7 855-line hook log (only two exceed it, both ~0.6-0.9 s), so it *is* unusual — but it
**does not discriminate**, and the brief's reading of it should not be carried forward. Both arms
share a prologue that probes 5 KB of stack (`mov eax,0x13a0 / call 0x142ef4450`), builds the local
character name, and resolves a string id for the first time in the session. `CLAUDE.md`: *the
result that does not vary is the clue* — this one varies with neither arm, so it is not evidence
for either. **[D]**

---

## 3. Why it survived, and why the tests did not catch it

Two reasons, and both are named sections of `CLAUDE.md`.

**The bug is invisible on every refusal.** `result::UNKNOWN_ERROR = 0x04`, and `0x04` is one of the
fifteen codes on the **default arm** (`research/party.md` §4). `request_failed()` therefore
produces `a5 00 | a5 00 04`; the client reads `0xA5`, takes the default arm, and shows *"Due to an
unknown error, your party request failed."* — **which is exactly what code `0x04` would have
shown.** The doubled opcode changes nothing observable for a refusal. Only `0x0E`, the first code
whose arm differs from the default, could ever expose it, and this was its first send.
**[D]** — `0x04 ∈ SILENT_CODES` is [L] (`crates/net/src/party.rs:258`, `:288`), the default arm's
string is [L] (§2 leg 4).

**Every test pins the wrong invariant, self-consistently.** `cargo test -p net party` is
**14 passed, 0 failed** today. The four that encode the convention:

```text
crates/net/src/party.rs:767   refusal_builds_the_two_byte_body_for_every_silent_code
                              assert_eq!(pkt.len(), 3, "opcode u16 + one code byte")
crates/net/src/party.rs:811   request_failed_is_the_default_arm
                              assert_eq!(pkt, vec![0xA5, 0x00, result::UNKNOWN_ERROR])
crates/world/src/session/party.rs:371
                              assert_eq!(out[0].body[0..2], net::party::PARTY_RESULT.to_le_bytes())
crates/world/src/session/party.rs:427   the_refusal_is_a_read_free_code
                              assert_eq!(body.len(), 3, "the opcode's two bytes plus one code byte")
```

`crates/world/src/session/party.rs:371` is the sharpest: it **asserts that the body starts with the
opcode**, which is the defect. And `party_created`'s own doc block says the convention was checked —
*"Caught by the one test that compared a real packet against `request_failed` rather than against
itself"* — but `request_failed` has the identical defect, so the comparison was against a second
copy of the bug. `CLAUDE.md`: *a test that pins what the code already does is not a check*, and
*a constant that came from reading a header is a claim, not a fact.*

**And `world.log` could not show it.** `send` logs `packet[2..]`, so the two bytes it prepends are
the two bytes it never prints. Every doubled-opcode packet logs as if it were correct. That is
worth a line in `docs/` on its own: **the outbound log is the builder's output, not the wire.**

---

## 4. The exact packet to send

**The same 209 bytes already in the log.** Not a different body — the same one, with the opcode
appearing once. Re-derived independently from `research/party-result-0x00A5.md` §5.1's field list
and it reproduces the logged body exactly:

```text
a5000e010000000000000000000000000000000000000000d60000000700546573746572320000000008000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000000000000000000000000000000000000f005465737465723227732050617274790000d6000000
```

```text
  a5 00                     opcode PARTY_RESULT           <- supplied ONCE, by Reply::packet()
  0e                        result::CREATED                  1413baf47 u8
  01 00 00 00               partyId = 1                      1413bbb95
  00                        party+0x4e0, unknown             1413bbba3
  00000000 00000000 00000000   seat-0 20-byte record         1413bbbb4 / bbbe / bbc8
  0000 0000                 two i16, sign-extended           1413bbbd2 / bbe0
  d6 00 00 00               MEMBER charId = 214              1406f2843   <- the gate: non-zero
  07 00 "Tester2"           MEMBER name                      1406f285a
  00000000                  +0x14  job    = 0                1406f2889
  08000000                  +0x18         = 8                1406f2894
  00000000                  +0x1c                            1406f289f
  00000000                  +0x20                            1406f28aa
  00                        +0x24                            1406f28b5
  00000000                  +0x28                            1406f28c3
  0000000000000000          +0x30                            1406f28ce
  <120 zero bytes>          +0x38 blob                       1406f28e4
  0f 00 "Tester2's Party"   party name                       1406f257d
  00                        isPublic  (party+0x4d8)          1406f25bc
  00                        party+0x4d9                      1406f25cc
  d6 00 00 00               leaderCharId = 214               1413bbc08
                                                             209 bytes total
```

### The fix

`crates/net/src/party.rs` — **two lines**, and I have not touched `crates/`:

```text
383   let mut w = PacketWriter::with_opcode(PARTY_RESULT);   ->   PacketWriter::new()   // party_created
410   let mut w = PacketWriter::with_opcode(PARTY_RESULT);   ->   PacketWriter::new()   // refusal
```

and delete the `// **with_opcode, like every other builder in this file.**` comment above 383 —
it is false in both halves: it is the only builder in the file that does this, and the framer does
**not** want the opcode in the body.

Every test in §3 then needs its expectation moved by two bytes (`len 3 -> 1`, `pkt[2] -> pkt[0]`,
and `crates/world/src/session/party.rs:371` deleted outright). **Do not "fix" them by adjusting the
numbers alone** — replace :371 with the invariant that would have caught this:

```rust
// The opcode belongs to Reply::packet(), ONCE. A body that starts with it is the
// doubled-opcode bug that made the client read code 0xA5 and take the default arm.
let wire = out[0].packet();
assert_eq!(u16::from_le_bytes([wire[0], wire[1]]), net::party::PARTY_RESULT);
assert_eq!(wire[2], net::party::result::CREATED, "the code, not a second opcode byte");
assert_eq!(wire.len(), 2 + 209);
```

Better still, and `research/party-result-0x00A5.md` §7 already asks for it: a round-trip reader in
the test module that walks `wire[2..]` the way the client's primitives do and asserts the cursor
lands **exactly** on the end. That single test catches the doubled opcode, a wrong field width, and
an off-by-one in either raw blob. It is the only check here that is not a restatement of the
builder.

---

## 5. What this does NOT settle, and the one experiment that does

**This explains the message. It does not prove the body is right.** The `0x0E` arm has never run.
Everything in §4 past the code byte is still `research/party-result-0x00A5.md`'s static read, and
the first run after the fix is its first test.

Two outcomes, and they are cleanly distinguishable **before the owner has to describe anything subtle**:

| what the chat line says | reading |
|---|---|
| **"You have created a new party."** | the `0x0E` arm ran. The framing was the whole bug. Then look at the party window. |
| **"Due to an unknown error, your party request failed."** *again* | the arm still was not entered. The framing was not the only defect, and the next thing to check is whether `Reply::packet()` is really the path this reply takes. |

Nothing else can appear from this handler for these bytes, so this is a two-valued measurement, not
an impression. **[D]** — the two strings are [L] and are the only two the relevant arms resolve.

**If it says "You have created a new party.", the second question is the window.** Watch for:

* does a party window open at all (the `operator new(0x378)` / `FUN_14118ace0` path at
  `0x1413bbc67`), and
* does the member row show **Tester2** with a job and a level.

**A named risk in that row, and it is worth watching for rather than pre-emptively changing.**
`FUN_1402b0250(member+0x14, member+0x18)` returns the job-name string for the row. Reading its
first three table inserts settles what `member+0x14` is:

```text
1402b0271  mov edx, 0x22 / key 0     -> string 0x0022 "Beginner"
1402b02ae  mov edx, 0x2d / key 0x64  -> string 0x002D "Swordsman"
1402b02eb  mov edx, 0x2e / key 0x6e  -> string 0x002E "Fighter"
```

Job ids 0 / 100 / 110. **So `member+0x14` is the job id**, which upgrades
`research/party-result-0x00A5.md` §3's *"the job name is the obvious candidate"* from **[I]** to
**[D]**, and `write_member`'s `w.u32(m.job)` at `+0x14` is correct. **[L]** for the three inserts.

But `crates/net/src/party.rs:342` puts **level** at `+0x18`, which is that lookup's *second*
argument, while `FUN_1413b90e0` reads `member+0x1c` and `member+0x20` for the row's other columns
(`research/party-result-0x00A5.md` §3). If the row shows the job but a level of **0**, level belongs
at `+0x1c`. That is **[I]** and it is a one-field change; do not make it before the run, because
`+0x18` may well be a sub-job the lookup needs. `CLAUDE.md`: *test one variant at a time.*

**Do not change the field mapping and the framing in the same launch.** The framing fix is the one
that has four legs behind it; the field mapping has none.

---

## 6. Corrections to earlier files

**`research/party-result-0x00A5.md` §5.1 and §3 are right and this run corroborates them** — the
209 bytes the server built decode exactly as that file's field list says, and I re-derived them
without looking at the Rust. **One line of it is wrong**, in §7:

> ```rust
> pub fn party_created(...) -> Vec<u8> {
>     let mut w = PacketWriter::with_opcode(PARTY_RESULT);
> ```

That is where the doubled opcode entered. The suggested builders in §7 return a `Vec` that is
handed to `Reply.body`, and `Reply::packet()` prepends the opcode itself. **Every builder in §7
should start `PacketWriter::new()`.** The field lists below that line are unaffected.

**`research/party.md`** describes `net::party::request_failed` as *"the always-answer valve"*. It
is — but for the reason in §3 it has been reaching the client as `a5 00 04` behind a stray `a5 00`,
and worked only because `0xA5` and `0x04` share the default arm. After the fix it works for the
stated reason instead of by coincidence.

---

## 7. Reproducing every number

All from the repo root. The scratchpad shadows `tools/`, so scripts written there were piped in or
given an absolute `sys.path` entry (`CLAUDE.md`, *The scratchpad shadows the real tools*).

```
python tools/reads.py    0x140304100 2                  # the control - run this FIRST
python tools/listing.py  0x140304100 | grep READ        # the same four reads, same addresses
python tools/listing.py  0x1413bab80                    # the handler; 0x10c at 1413bbc80, 0x12a at 1413bcbb5
python tools/listing.py  0x1402b0250                    # the job-name table: keys 0/0x64/0x6e
python tools/dump_stringids.py --grep "unknown error"   # id 298 = 0x012A
python tools/dump_stringids.py --id 268                 # 0x010C "You have created a new party."
grep -rn with_opcode crates/net/src/*.rs                # 2 outbound sites, both in party.rs
grep -rn "\.packet()" crates/world/src                  # 3 send sites, all in server.rs
cargo test -p net party                                 # 14 passed - they pin the bug
```

The `.text` immediate scan is 40 lines of Python over `tools/reads.py`'s loader plus capstone,
disassembling `0x140001000..0x14326194a` with one-byte resync; it takes about three minutes. Its
controls are `0x129 -> 0x1413b9fc1` and `0x10c -> 0x1413bbc80`, and **both must appear or the
result means nothing.**
