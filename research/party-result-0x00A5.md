# `0x00A5` PARTY_RESULT: the success arms, byte by byte

Written 2026-09-03. **No Ghidra** (project lock held elsewhere), **no client run**. Everything
below comes from `client-patched\MapleStory.exe` via `tools/reads.py`, `tools/listing.py`,
`tools/callers.py`, `tools/dump_stringids.py`, direct capstone walks of the three jump tables,
and an event-deduplicated sweep of the archived runs.

Builds on `research/party.md`, which established that `0x00A5` is the party result and that
16 of 29 non-default arms are read-free. **This file decodes the arms that read fields.**

Labels: **[L]** read off a listing, a jump table, the string table or a capture; **[D]**
derived from two or more [L] facts that agree; **[I]** inferred, nothing on this machine
confirms it.

---

## 0. Instruments, and the controls they passed FIRST

| instrument | control | result |
|---|---|---|
| `python tools/reads.py 0x140304100 2` | the documented control in its own docstring | raw @`140304138`, u8 @`140304144`, u8 @`140304183`, then a run of u16, **and a mix of direct and via-helper reads** | **PASS** |
| `python tools/dump_stringids.py --grep "Cash Shop"` | must find ~19 of 6 165 | 19 hits | **PASS** |
| the 45-entry jump-table walk | every target must land inside `0x1413bab80..0x1413bcd30` | 45/45 inside, 30 distinct arms, 15 codes on the default | **PASS** |
| the archive sweep | `<- 0x02FF` must exceed 100 000 distinct events | **166 547** over **530** files | **PASS** |
| the call-target walk (§6) | the four known helpers must come back positive | 4/4 | **PASS** *(after a failure - see below)* |

### One instrument failed its control and I threw the result away

My first call-target enumeration used `reads.reaches()` without calling `reads._load()`
first. `_ent` was empty, so `calls_of` returned `[]` for every function and the walk reported
**"only 5 targets reach a read primitive"** - a clean, confident number. The control
(*"the four known helpers must appear"*) came back **False**, which is the only reason it was
caught. Re-run correctly, §6 finds the same four helpers and no others. **The number in §6 is
from the second run.** `CLAUDE.md`: a summary is a claim, not a check.

---

## 1. The answers, up front

| you want | code | body |
|---|---|---|
| **party created** | `0x0E` | `u32 partyId, u8, u32,u32,u32,i16,i16, MEMBER, str partyName, u8, u8, u32 leaderId` |
| **someone joined** | `0x13` | `str name, PARTYBLOCK` |
| **someone left / expelled / disbanded** | `0x10` | `u32 charId, u8 stillExists`, **and if `stillExists != 0`**: `u8 expelled, str name, PARTYBLOCK` |
| **invite arriving at the target** | `0x03` | `u32, u32, str, u32, u32, u32` - all six unconditional |
| **leader changed** | `0x22` | `u32 newLeaderCharId, u8 reason` |
| *(bonus)* **push whole party state** | `0x0D` | `u8 present`, **and if `present != 0`**: `PARTYBLOCK` |
| *(bonus)* **"You have invited '%s'"** | `0x1B` | `i32 outcome (0..10), str name` |

Two structures do the work: **`MEMBER`** (§3) and **`PARTYBLOCK`** (§4).

**The single most important gate in the whole packet**, and it is exactly the shape
`CLAUDE.md` warns about:

> **`MEMBER` stops dead if its first `u32` is zero.** `0x1406f2848 test eax,eax / je` returns
> from the decoder having read **four bytes and nothing else**. An empty party seat is
> **4 zero bytes**, not a zeroed 155-byte record. **[L]**

Send a full record after a zero id and every following field in the packet is off by 151+
bytes. Send four zero bytes where the client expected a full record and it under-reads.
There is no middle ground and no error - the reader just returns.

---

## 2. The dispatcher, re-walked rather than trusted

```asm
1413baf47  call 0x1406e8ae0            ; READ u8 code   - the first read in the function
1413baf4f  mov  [rsp+0x50], eax        ; the code is KEPT: 0x1B/0x1C read it back later
1413baf58  add  eax, -3
1413baf5b  cmp  eax, 0x2c
1413baf5e  ja   0x1413bcbb5            ; default arm
1413baf66  lea  rdx, [rip - 0x13baf6d] ; = 0x140000000, the image base
1413baf6d  mov  ecx, [rdx + rax*4 + 0x13bcc50]
1413baf74  add  rcx, rdx               ; entries are dword RVAs
1413baf7e  jmp  rcx
```

**The base the table is indexed against is `0x140000000` and the entries are RVAs, not
addresses.** (`research/user-enter-field.md` §2.4 - an offset table is meaningless without
its base. Stated here so the next reader does not have to re-derive it.) **[L]**

45 entries at `0x1413bcc50`, index = `code - 3`, so **codes `0x03..0x2F`**. Walk reproduced
`research/party.md`'s table exactly: 30 distinct arms, 15 codes on the default. **[L]**

**There are three jump tables in this handler, not one:**

| table | at | entries | selector | §|
|---|---|---|---|---|
| the code switch | `0x1413bcc50` | 45 | `u8 code - 3` | §2 |
| the `0x1B`/`0x1C` outcome switch | `0x1413bcd04` | 11 | the arm's own `i32` | §5.7 |
| the `0x2A` field-patch switch | `0x1406f3370` | 8 | the arm's own `i32` | §5.8 |

The second and third are **inside packet bodies** - a field whose value selects which later
fields exist. Nothing in `research/party.md` mentioned them.

### The struct every offset in this file is relative to

All the arms write into **one global party singleton at `0x143ACC700`**, and that address is
not assumed - it is where `FUN_1406f2fd0` writes `[rbx+0]` when the arms call it with their
own global. Every rip-relative operand in the arms was resolved mechanically (not by hand)
and **each one lands on an offset `FUN_1406f2fd0` independently fills**:

```text
arm 0x0E  0x1413bbb9a -> 0x143acc700  party+0x000   arm 0x22  0x1413bc562 -> party+0x000
          0x1413bbbab -> 0x143accbe0  party+0x4e0             0x1413bc571 -> party+0x008
          0x1413bbbf9 -> 0x143accbd0  party+0x4d0             0x1413bc586 -> party+0x008
          0x1413bbc0d -> 0x143acc708  party+0x008             0x1413bc59a -> party+0x000
          0x1413bbc20 -> 0x143accb40  party+0x440   arm 0x2D  0x1413bc661 -> party+0x4d9
          0x1413bbc37 -> 0x143accb58  party+0x458             0x1413bc683 -> party+0x4d8
switch    0x1413baf77 -> 0x143acc710  party+0x010   arm 0x2A  0x1413bc385 -> party+0x010
```

That agreement across four independent arms is the reason the layout below is **[D]** rather
than a guess. **[L]** for each individual resolution.

---

## 3. `MEMBER` - the party member record

`FUN_1406f2830(dst, packet)`. **In memory: exactly `0xB0` bytes.** On the wire: variable.

```text
  u32  charId                       -> +0x00
  --- IF charId == 0: RETURN. Nothing further is read. ---            <<< THE GATE [L]
  str  name                         -> +0x04, copied with r8d = 0xD -> 13 bytes
  u32  a                            -> +0x14
  u32  b                            -> +0x18
  u32  c                            -> +0x1C
  u32  d                            -> +0x20
  u8   e                            -> +0x24   (zero-extended to a dword)
  u32  f                            -> +0x28
  u64  g                            -> +0x30
  raw  120 bytes (0x78)             -> +0x38   ... ends at +0xB0
```

Read addresses, all **[L]**:

| field | read at | primitive |
|---|---|---|
| `charId` | `0x1406f2843` | `0x1406e8c20` u32 |
| `name` | `0x1406f285a` | `0x1406e9050` str |
| `a` | `0x1406f2889` | u32 |
| `b` | `0x1406f2894` | u32 |
| `c` | `0x1406f289f` | u32 |
| `d` | `0x1406f28aa` | u32 |
| `e` | `0x1406f28b5` | `0x1406e8ae0` u8 |
| `f` | `0x1406f28c3` | u32 |
| `g` | `0x1406f28ce` | `0x1406e8f10` u64 |
| `raw[0x78]` | `0x1406f28e4` | `0x1406e9170` raw, `r8d = 0x78` |

**Wire size** = `4` when empty; `155 + len(name)` when occupied (this repo's `str` is
`u16 length + bytes`, `crates/net/src/packet.rs:97`).

### The `0xB0` stride and the `+0x00` / `+0x04` roles are confirmed four ways

1. `FUN_1406f2bb0` loops **exactly 6 times** adding `0xB0` (`0x1406f2bfb add rbx,0xb0`). **[L]**
2. `FUN_1406f1fb0(party+8, charId)` - the member lookup - starts at `base+8`, compares
   `[rax]` against the id, `add rax,0xb0`, `cmp r8,6`, returns the index or `-1`. **[L]**
3. Arm `0x22` formats its message with `party + 0x14 + index*0xB0` - i.e. **`member+0x04`,
   the name**, and `0xAF1` is `"%s has become the leader of the party."` **[L]**
4. `FUN_1406f1cb0` (the initialiser) zeroes 6 records of `0xB0`, and its name clear is
   `qword [rax+4] / dword [rax+0xc] / byte [rax+0x10]` = **13 bytes at +4**. **[L]**

### What the fields MEAN is much weaker than what they ARE

`FUN_1413b90e0(index, ...)` is the member-list row accessor. It reads: `charId`;
`name`; `[party+0x440 + index*4]`; then **`FUN_1402b0250(ecx = member+0x14, edx = member+0x18)`
which returns a string pointer**; then `member+0x1c`; then `member+0x20`; then sets an output
to `member.charId == [party+0x008]`. **[L]**

* `a`/`b` (`+0x14`,`+0x18`) together resolve to a **display string** - the job name is the
  obvious candidate given the member list's `0x011C` column headers. **[I]**
* `c` (`+0x1c`) and `d` (`+0x20`) are shown in the row. **level** and **channel/map** are the
  candidates. **[I] - not established, do not label them in code.**
* `e`, `f`, `g` and the 120-byte blob have no observed reader in this pass. **Unknown.**

**Named blind spot:** the 120-byte blob at `member+0x38` is `memcpy`'d, never parsed at read
time. All-zero is safe *for the read*, but whatever later consumes it was not traced. If it
turns out to be an avatar look, zeros may draw wrongly rather than crash. That is a deferred
risk, not a cleared one.

---

## 4. `PARTYBLOCK` - the whole party

`FUN_1406f2fd0(party, packet)`, calling `FUN_1406f2bb0(party+8, packet)` in the middle.
Five call sites, all in this handler (`tools/callers.py`: 5 sites, 1 function). **[L]**

```text
  u32   partyId                      -> party+0x000        0x1406f2fe8
  u8    h                            -> party+0x4E0        0x1406f2ff2  (zero-extended)
  MEMBER × 6                         -> party+0x010 + i*0xB0            <- FIXED SIX, NOT COUNTED
  u32   leaderCharId                 -> party+0x008        0x1406f2c0b
  u32   count                                              0x1406f2c25
  count × { MEMBER ; raw[8] }        -> a map at party+0x430            <- COUNT-DRIVEN
  raw   24  (0x18)                   -> party+0x440        0x1406f301c
  raw   120 (0x78)                   -> party+0x458        0x1406f3031
  str   partyName                    -> party+0x4D0        0x1406f303e
  u8    isPublic (as bool)           -> party+0x4D8        0x1406f3088
  u8    i        (as bool)           -> party+0x4D9        0x1406f309b
```

### The member list is NOT count-driven. The list after it is.

This is the field the brief asked about and the answer is unusual: **there are two member
lists.**

* The **six slots** are a fixed-length array. `0x1406f2be4 mov edi,6` / `sub rdi,1 / jne`.
  There is no count byte. Every one of the six is decoded, and an unused seat is the 4-byte
  `charId == 0` form. **[L]**
* The **second list** is `u32 count` then `count` × (a full `MEMBER` plus **8 raw bytes**),
  inserted into a red-black tree at `party+0x430` keyed by the record's `charId`, with a
  `0xE0`-byte node whose value is the `0xB0` record followed by the 8 bytes. **[L]**
  `count == 0` skips the loop cleanly (`0x1406f2c33 test eax,eax / jle`), so **zero is safe
  and is what this server should send.** **[L]**

  What that second list *is* was **not established**. Structurally it is "more member records,
  addressed by id rather than by seat". Arm `0x2A` with its flag byte clear patches a record
  in *that* tree instead of in the six slots, which is consistent with it being a wider roster
  (offline / other-channel members) - but that is **[I]** and nothing here tests it.

### The two raw blobs are per-member arrays, and one of them is proved

* **`party+0x440`, 24 bytes = six `u32`, one per seat. [L]** - `FUN_1413b90e0` reads it as
  `mov eax, [rax + rdi*4]` where `rdi` is the member index (`0x1413b9211`). An indexed read
  with stride 4 over a 24-byte region is six `u32`.
* **`party+0x458`, 120 bytes = six 20-byte records, one per seat. [L]** - arm `0x2F` computes
  `party + 0x458 + 20*index` by `lea rcx,[rax*4+0x116] / add rcx,rax / lea rcx,[r15+rcx*4]`
  (`0x1413bc50e`), and `FUN_1406f1c90` writes the record as `u32,u32,u32,i32,i32`. The two
  `i32`s arrive on the wire as **`i16` sign-extended** (`movsx`), so they are coordinates.
  A per-member `{u32,u32,u32,i16,i16}` reads as a **mystic-door / town-portal** record, which
  is **[I]**; the *layout* is **[L]**.

Both are `memcpy`'d, so **all-zero is byte-safe**, and a party with no doors is the honest
value to send.

### `isPublic` is `party+0x4D8`, and arm `0x2D` proves it

`FUN_1406f2560(dst, packet)` reads `str -> dst+0`, `u8 -> dst+8`, `u8 -> dst+9`,
**unconditionally, no gate** (`0x1406f257d`, `0x1406f25bc`, `0x1406f25cc`). **[L]** It is
called with `dst = party+0x4D0`, which is why the block's tail is that shape.

Arm `0x2D` calls it, then compares the new `dst+8` against the **old `party+0x4D8`** and shows
`0x0118 "The party leader has changed the party status to Public."` when it is non-zero and
`0x0119 "...to Private."` when it is zero. **So `party+0x4D8` = "the party is Public".** **[D]**
`party+0x4D9` and `party+0x4E0` have no observed consumer. **Unknown.**

---

## 5. The arms, one at a time

Every "read at" below is an address in `FUN_1413bab80`. All string texts are from
`tools/dump_stringids.py --id`, decrypted, **[L]**.

### 5.1 `0x0E` - party created  → *"You have created a new party."* (`0x010C`)

Arm `0x1413bbb8d`. **No gates. Every field is unconditional.**

| # | type | read at | destination | note |
|---|---|---|---|---|
| 1 | `u32` | `0x1413bbb95` | party+0x000 | **partyId** [D] |
| 2 | `u8` | `0x1413bbba3` | party+0x4E0 | unknown [L] shape only |
| 3 | `u32` | `0x1413bbbb4` | party+0x458 +0 | the seat-0 20-byte record |
| 4 | `u32` | `0x1413bbbbe` | party+0x458 +4 | " |
| 5 | `u32` | `0x1413bbbc8` | party+0x458 +8 | " |
| 6 | `i16` | `0x1413bbbd2` | party+0x458 +0xC | **signed**, `movsx` |
| 7 | `i16` | `0x1413bbbe0` | party+0x458 +0x10 | **signed**, `movsx` |
| 8 | `MEMBER` | `0x1413bbbf1` | party+0x010 | **seat 0 only** - the creator |
| 9 | `str`+`u8`+`u8` | `0x1413bbc00` | party+0x4D0/8/9 | via `FUN_1406f2560`: name, isPublic, ? |
| 10 | `u32` | `0x1413bbc08` | party+0x008 | **leaderCharId** [D] |

**`0x0E` is a hand-inlined subset of `PARTYBLOCK` writing the same singleton.** It sets seats
1..5 to nothing at all - it does not clear them (`FUN_1413b8d40` is called *before* any read
and reads nothing, `tools/reads.py 0x1413b8d40 3` → none). For a brand-new party that is
correct, because seat 0 is the only occupant.

**Send fields 3-7 as zero** unless a door is meant. Field 2 unknown → `0`.

### 5.2 `0x13` - someone joined  (`0x0117` / `0x0116`)

Arm `0x1413bc046`. **Uniform: no gate changes the field list.**

```text
  str  name                    read at 0x1413bc04e
  PARTYBLOCK                   read at 0x1413bc0c0  (name != mine)
                                    or 0x1413bc158  (name == mine)
```

The client compares `name` against **its own character name** (built at the top of the handler
from `0x142cb9610`) and picks the message. **Both branches read the full `PARTYBLOCK`**, into
the same singleton `0x143ACC700`, so the wire shape does not vary. **[L]**

* name == mine → `0x0116 "You have joined the party."`
* otherwise → `0x0117 "'%s' has joined the party."`

**So `name` must be the joiner's real character name**, or the wrong player sees the
first-person message. This is a content requirement, not just a format one.

### 5.3 `0x10` - left / expelled / disbanded

Arm `0x1413bbcf1`. **This arm has the second gate, and it removes four fields.**

```text
  u32  charId                  read at 0x1413bbcf4
  u8   stillExists             read at 0x1413bbd03
  --- IF stillExists == 0: STOP. The rest of the packet is never read. ---   [L]
  u8   expelled                read at 0x1413bbd13
  str  name                    read at 0x1413bbd23
  PARTYBLOCK                   read at 0x1413bbd4b (charId == mine)
                                    or 0x1413bbe6d (charId != mine)
```

`0x1413bbd0a test al,al / je 0x1413bbf01` jumps to a block containing **no packet read at
all**. **[L]**

The client then compares `charId` against **its own character id** (`[rbp-0x80]`, set at
`0x1413babeb` from `FUN_142cb9550`):

| `stillExists` | `charId` | `expelled` | message |
|---|---|---|---|
| ≠0 | mine | ≠0 | `0x010D` *"You have been expelled from the party."* |
| ≠0 | mine | 0 | `0x010E` *"You have left the party."* |
| ≠0 | other | ≠0 | `0x0110` *"'%s' have been expelled from the party."* |
| ≠0 | other | 0 | `0x0111` *"'%s' have left the party."* |
| **0** | mine | *(not read)* | `0x0114` *"You have quit as the leader of the party. The party has been disbanded."* |
| **0** | other | *(not read)* | `0x0115` *"You have left the party since the party leader quit."* |

So **`stillExists == 0` is the disband**, and it costs 5 body bytes total. **[D]** - the arm
structure is [L] and the string ids are [L]; the *naming* of the byte follows from which
messages each branch shows.

Note the leaver's own client parses the block into a **stack temporary** (`rbp+0xda0`) and
throws it away, while everyone else parses into the singleton. Same bytes either way.

### 5.4 `0x03` and `0x06` - an invite arriving at the target

Arms `0x1413baf80` and `0x1413bb25b`. **Identical field lists, both unconditional.**

| # | type | `0x03` read at | `0x06` read at | use |
|---|---|---|---|---|
| 1 | `u32` | `0x1413baf83` | `0x1413bb25e` | looked up in a client-side list (§below) |
| 2 | `u32` | `0x1413baf8e` | `0x1413bb268` | **echoed back in the `0x0183` answer as a `u64`** |
| 3 | `str` | `0x1413baf9d` | `0x1413bb277` | passed to the dialog |
| 4 | `u32` | `0x1413bafa6` | `0x1413bb280` | passed to the dialog |
| 5 | `u32` | `0x1413bafb1` | `0x1413bb28c` | passed to the dialog |
| 6 | `u32` | `0x1413bafbd` | `0x1413bb297` | passed to the dialog |

**All six are read before any branch**, so the body never varies. **[L]**

What happens after depends on client state, not on the packet:

* `0x03` only: if `dword [<global>+0x154] == 0` the client **auto-declines** - it sends
  `0x0183 {u8 0x1B, u8 1, u64 field2}` and never opens the dialog (`0x1413bafd3`). The
  `check:inviteParty` box in `StatusBar3.img` is the obvious candidate for that global. **[I]**
* Both: `FUN_142d01050(<global list>, field1)` - if it returns non-zero, same auto-decline.
  A blacklist / blocked-user lookup keyed on **field 1**, which is therefore **a character
  id**. **[D]**
* Otherwise a dialog opens (`FUN_141808b90` for `0x03`, `FUN_14180f2d0` for `0x06`) and is
  handed `(name, field2, field4, field5, field6)` via `FUN_141810230`.

**field 2 is the value the server gets back in `0x0183`**, so it is the invite's identity -
party id or an invite token. **[D]** Fields 4, 5 and 6 are shown in the dialog and their
meanings are **not established**. `0` is a legal value for all three as far as the *reader*
is concerned; whether the dialog draws sensibly with zeros was not tested.

**Which of `0x03`/`0x06` is invite and which is join-request is still [I]**, exactly as
`research/party.md` §8.3 says. The one new discriminator found here: **only `0x03` consults
the invite-permission option**. That is what you would expect of the *invite* path.

### 5.5 `0x22` - leader changed

Arm `0x1413bc54d`.

```text
  u32  newLeaderCharId         read at 0x1413bc550
  u8   reason                  read at 0x1413bc55a
```

Both unconditional; nothing else is read. Then, **gated entirely on client state**:

```asm
1413bc562  cmp dword [party+0x000], 0   ; partyId
1413bc569  jle <silent epilogue>        ; not in a party -> nothing happens
1413bc571  FUN_1406f1fb0(party+8, charId)
1413bc57d  cmp eax,-1 / je <silent>     ; not one of the six seats -> nothing happens
1413bc586  mov [party+0x008], charId    ; commit the new leader
```

* `reason == 0` → `0x0AF1 *"%s has become the leader of the party."*
* `reason != 0` → `0x0AF2 *"Due to the party leader disconnecting from the game, %s has been
  assigned as the new leader."*

**[L]** for the arm, the two reads and both string ids.

**The gate that matters for the server:** `0x22` is silently discarded unless the client
already has `partyId > 0` **and** already holds a seat whose `charId` matches. So it must be
sent **after** the membership change that puts the new leader in the six slots, never before,
and never as the only packet. `CLAUDE.md`'s *"a guard whose answer is ignored"* in reverse:
here the client's guard answers "no" and the server never finds out.

### 5.6 `0x0D` - push the whole party state

Arm `0x1413bbab4`. The cleanest packet in the set, and `research/party.md` had it as
"`u8` + a helper".

```text
  u8   present                 read at 0x1413bbab7
  --- IF present == 0: STOP. Nothing further is read. ---   [L]
  PARTYBLOCK                   read at 0x1413bbacd
```

`present == 0` takes `0x1413bbb2c`, which calls `FUN_1413b8d40` (reads nothing) and clears the
UI. This is **"here is your party"** / **"you have no party"** with no message text at all - no
string id is resolved on either path. For a server that wants to re-sync a client's party
window without narrating anything, this is the packet.

### 5.7 `0x1B` / `0x1C` - the invite/join **outcome**, and the second jump table

Arm `0x1413bb51a`, shared by both codes.

```text
  raw[4] outcome               read at 0x1413bb52a   <- read as 4 RAW bytes, used as i32
  str    name                  read at 0x1413bb537
```

Then `movsxd rax, dword [rbp-0x58] / cmp eax,0xA / ja default` and a **second jump table at
`0x1413bcd04`, 11 entries, same `0x140000000` base**. Every sub-arm re-reads the original code
from `[rsp+0x50]` and branches on `== 0x1C`. **[L]**

| outcome | code `0x1B` | code `0x1C` |
|---|---|---|
| 0 | `0x0112` *"You have invited '%s' to your party."* | `0x0113` *"You've requested to join %s's party."* |
| 1 | `0x0108` *"%s is currently blocking any party invitations."* | `0x0109` *"%s is currently refusing party join requests."* |
| 2 | `0x0792` *"'%s' is taking care of another invitation."* | same |
| 3 | `0x011F` *"You have already invited '%s' to your party."* | `0x0120` *"You already asked to join %s's party."* |
| 4 | `0x010A` *"%s has denied the party request."* | `0x010B` *"%s has refused your party join request."* |
| 5, 6 | *(silent - both land on the shared cleanup at `0x1413bba8c`)* | same |
| 7 | `0x012C` *"'%s' could not be found in the current server."* | same |
| 8 | `0x012D` *"Party cannot be found. Please check the party info once again."* | same |
| 9 | `0x0125` *"The party you're trying to join is already in full capacity."* | same |
| 10 | `0x0122` *"%s' is already in a party."* | same |
| >10 | *(silent, the `ja` default)* | same |

**`0x1B` with outcome 0 is what the leader sees after a successful Invite.** No further fields
are read for any outcome - the sub-arms only format a message. **The body is always exactly
`4 + 2 + len(name)` bytes.** **[L]**

This also resolves `research/party.md`'s *"the `raw` length is unread"*: it is **4**
(`0x1413bb51a mov r13d,4`).

### 5.8 `0x2A` - patch one field of one member. **Do not send this.**

Arm `0x1413bc327`.

```text
  u8     inTheSixSlots         read at 0x1413bc32a
  u32    charId                read at 0x1413bc335
  raw[4] selector              read at 0x1413bc349
  ... then FUN_1406f3270, which reads ONE MORE FIELD chosen by `selector` ...
```

`FUN_1406f3270` is an 8-entry jump table at `0x1406f3370` (`cmp eax,7 / ja`):

| selector | reads | into |
|---|---|---|
| 0 | `str` | member+0x04 (13-byte copy) |
| 1 | `u32` | member+0x14 |
| 2 | `u32` | member+0x1C |
| 3 | `u32` | member+0x20 |
| 4 | `u8` | member+0x24 |
| 5 | `u32` **and** `u64` (via `FUN_1406f29b0`) | member+0x28, member+0x30 |
| 6 | `u32` | member+0x28 |
| 7 | `raw[0x78]` | member+0x38 |
| >7 | **nothing** | — |

**Why not to send it:** whether that last field is read is gated on **client state**, not on
the packet. If `party+0x000 <= 0`, or `FUN_1406f1fb0` does not find `charId` in the six seats
(flag set), or the tree lookup misses (flag clear), the arm **jumps to the epilogue without
calling `FUN_1406f3270` at all**. The client then holds an unread tail. Harmless for this one
packet - but it means the server cannot know from its own state whether the client consumed
the field. `0x13` and `0x0D` do the same job unconditionally.

### 5.9 `0x2F` - set one seat's 20-byte record. **Never send index > 5.**

Arm `0x1413bc434`. Body: `u8 index, u32, u32, u32, i16, i16` - all six unconditional
(`0x1413bc437`, `...446`, `...451`, `...45d`, `...469`, `...478`).

```asm
1413bc488  cmp eax, 5
1413bc48b  jbe 0x1413bc50c        ; in range -> straight to the write
           ... shows an error dialog (string 0x304) ...
1413bc508  mov eax, [rsp+0x50]    ; and then FALLS INTO the write anyway
1413bc50c  lea rcx, [party + 0x458 + 20*index]
1413bc53b  call FUN_1406f1c90     ; writes 20 bytes there
```

**An out-of-range index writes 20 bytes past a 120-byte array after warning about it.** [L]
Given this project's heap-corruption history that is worth stating plainly: `index` must be
`0..=5`, enforced at the builder, not documented above it.

### 5.10 `0x14`, `0x1E`, `0x2D` - the small ones

* **`0x14`** (`0x1413bc2d8`): `u8` at `0x1413bc2db`, nothing else. Shows `0x0116 "You have
  joined the party."` and calls `FUN_142d1d130(1, u8 != 0)`. **It does not carry a party
  block**, so it cannot populate the window on its own. **[L]**
* **`0x1E`** (`0x1413bcb0d`): `str` at `0x1413bcb15` - the highest read address in the whole
  8 624-byte function, exactly as `research/party.md` says. `0x012E`. **[L]**
* **`0x2D`** (`0x1413bc63e`): `FUN_1406f2560` at `0x1413bc64f` → `str name, u8 isPublic,
  u8 ?`, unconditional. Then gated on `party+0x000 > 0`, and the message only appears if
  `isPublic` **differs from the current value**. **[L]**

---

## 6. Enumerate before you filter: what could still be hiding

The brief warned about an unconditional `call` that consumes the packet without reading
anything itself. So rather than trusting the read walk, I enumerated **every** call and tail
`jmp` leaving `FUN_1413bab80` - **58 distinct out-of-function targets** - and walked each to
depth 6 with `reads.calls_of` (which counts tail jumps, not just `call`).

**Result: exactly nine reach a read primitive - the five direct primitives and the four known
helpers. Nothing else. [L]** Control: all four known helpers present.

I also checked the trap `tools/reads.py` documents in its own `PRIM` table - the `f64` reader
`0x1406e8fb0`, whose absence made *"every read walk that crossed one come back eight bytes
short, silently"*. Over the **1 168 functions** reachable from this handler at depth 8, the
readers used are `u8`, `u16`, `u32`, `u64`, `str`, `raw` and **not** `f64`. So this handler is
not in the affected set. **[L]**

### Named blind spots

1. **Eight indirect calls** inside the handler: `0x1413bb23f`, `0x1413bb4fe`, `0x1413bbb5a`,
   `0x1413bbe2b`, `0x1413bbffa`, `0x1413bc61e`, `0x1413bca9d`, `0x1413bcaca`. A `call
   qword [rax]` cannot be followed statically. **All eight sit after the reads in their own
   arm**, in UI-notify and cleanup paths - but "after" is an argument from position, not a
   proof, and a virtual method that read the packet would be invisible to every scan here.
   This is the same blind spot `research/party.md` §8.6 names for the inbound sweep.
2. **Arm boundaries were taken as "up to the next table target".** Every read address
   `tools/reads.py` reports was accounted for, so no read fell outside an arm - but a *call*
   in a region I did not print could exist. §6's enumeration covers the whole function, which
   is what makes this survivable.
3. **Field meanings** for `MEMBER +0x14..+0x38`, the `0x03` fields 4-6, `party+0x4D9`,
   `party+0x4E0` and the second member list are **not established**. Widths and order are;
   meanings are not. Per the brief: I am not filling them in.
4. **Nothing here has been on the wire.** Re-verified this session: 530 archived logs,
   event-deduplicated on `(timestamp, direction, opcode, first 120 bytes)`, control
   `<- 0x02FF` = **166 547** distinct events → **`0x00A5`: 0 sent, 0 received.** `0x0182`: one
   received, ever. Every byte layout above is static analysis. **[L]**

---

## 7. The builders I would add to `crates/net/src/party.rs`

**Not written - `crates/` was not touched.** Signatures and bodies for the coordinator.

The shape that matters: `Member` carries `Option`-ness in its `char_id`, because that is what
the client's gate actually is.

```rust
/// One seat in the six-slot party array.
///
/// **A seat is empty iff `char_id == 0`, and an empty seat is FOUR BYTES on the wire.**
/// `0x1406f2848 test eax,eax / je` returns from the client's decoder having read only the
/// id. This is not a convention - it is the only shape the client can parse. **[L]**
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Member {
    pub char_id: u32,
    /// Copied by the client into a **13-byte** field (`r8d = 0xD` at `0x1406f286e`), so
    /// 12 characters plus a NUL. Longer names are truncated by the client, not rejected.
    pub name: String,
    /// `member+0x14`/`+0x18`. Together they resolve to a display string via
    /// `FUN_1402b0250`; the job name is the candidate. **[I] - unnamed on purpose.**
    pub f14: u32,
    pub f18: u32,
    /// `member+0x1c`, `+0x20`. Shown in the member-list row. Level and channel are the
    /// candidates. **[I] - unnamed on purpose.**
    pub f1c: u32,
    pub f20: u32,
    /// `member+0x24`, sent as a `u8`, stored zero-extended.
    pub f24: u8,
    pub f28: u32,
    pub f30: u64,
    /// `member+0x38`, exactly 120 bytes, `memcpy`'d by the client and not parsed at read
    /// time. All-zero is safe for the read. Its consumer was not traced.
    pub blob: [u8; 0x78],
}

fn write_member(w: &mut PacketWriter, m: &Member) {
    w.u32(m.char_id);
    if m.char_id == 0 {
        return;                     // THE GATE. Anything more here desynchronises the body.
    }
    w.str(&m.name);
    w.u32(m.f14); w.u32(m.f18); w.u32(m.f1c); w.u32(m.f20);
    w.u8(m.f24);
    w.u32(m.f28);
    w.u64(m.f30);
    w.bytes(&m.blob);
}

/// The whole party, as `FUN_1406f2fd0` reads it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PartyBlock {
    pub party_id: u32,
    /// `party+0x4e0`. No observed consumer. **Unknown - send 0.**
    pub f4e0: u8,
    /// **Exactly six.** Not a count-driven list: `0x1406f2be4 mov edi,6`. Pad with
    /// `Member::default()`, which is the 4-zero-byte empty seat.
    pub seats: [Member; 6],
    pub leader_char_id: u32,
    /// `party+0x440`: six `u32`, one per seat. Proved by `[rax + rdi*4]` at `0x1413b9211`.
    pub per_seat_u32: [u32; 6],
    /// `party+0x458`: six 20-byte records, one per seat, `{u32,u32,u32,i16,i16}` - the
    /// last two sign-extended, so coordinates. A mystic door is the candidate. **[I]**
    pub per_seat_rec: [[u8; 20]; 6],
    pub name: String,
    /// `party+0x4d8`. Arm `0x2D` shows "changed the party status to Public" when set. **[D]**
    pub is_public: bool,
    /// `party+0x4d9`. No observed consumer. **Unknown - send false.**
    pub f4d9: bool,
}

fn write_party_block(w: &mut PacketWriter, p: &PartyBlock) {
    w.u32(p.party_id);
    w.u8(p.f4e0);
    for m in &p.seats {
        write_member(w, m);          // six, unconditionally
    }
    w.u32(p.leader_char_id);
    // The second, count-driven member list. Its purpose is NOT established, and `count == 0`
    // skips the client's loop cleanly (`0x1406f2c33 test eax,eax / jle`). Send nothing.
    w.u32(0);
    for v in &p.per_seat_u32 { w.u32(*v); }        // the raw[0x18]
    for r in &p.per_seat_rec { w.bytes(r); }       // the raw[0x78]
    w.str(&p.name);
    w.u8(p.is_public as u8);
    w.u8(p.f4d9 as u8);
}
```

Then the five the brief asked for, plus the two that turned out to be the useful ones:

```rust
/// `0x0E` - **"You have created a new party."** Sent to the leader only.
///
/// Writes the same singleton `PARTYBLOCK` does, but only seat 0 - which is correct for a
/// brand-new party and wrong for anything else. Use [`party_state`] to show more members.
pub fn party_created(party_id: u32, leader: &Member, name: &str, is_public: bool) -> Vec<u8> {
    let mut w = PacketWriter::with_opcode(PARTY_RESULT);
    w.u8(result::CREATE_OK);
    w.u32(party_id);
    w.u8(0);                                     // party+0x4e0, unknown
    w.u32(0); w.u32(0); w.u32(0);                // seat 0's 20-byte record: no door
    w.i16(0); w.i16(0);
    write_member(&mut w, leader);
    w.str(name);
    w.u8(is_public as u8);
    w.u8(0);
    w.u32(leader.char_id);                       // party+0x008, the leader
    w.into_vec()
}

/// `0x13` - **"'%s' has joined the party."** / **"You have joined the party."**
///
/// `name` MUST be the joiner's real character name: the client compares it against its own
/// and picks the first- or third-person message from that. Send to every member; each one
/// decides for itself which sentence it sees. **[L]**
pub fn party_member_joined(name: &str, party: &PartyBlock) -> Vec<u8> {
    let mut w = PacketWriter::with_opcode(PARTY_RESULT);
    w.u8(result::JOIN);
    w.str(name);
    write_party_block(&mut w, party);
    w.into_vec()
}

/// `0x10` - someone left, was expelled, or the party disbanded.
///
/// `party` is `None` for a disband, and that is **not a stylistic choice**: with the
/// still-exists byte clear the client reads NOTHING further (`0x1413bbd0a`). Passing a block
/// anyway would append bytes it never consumes; omitting one while claiming the party lives
/// would leave it reading past the body.
///
/// As with [`party_member_joined`], `char_id` decides who sees the first-person wording, so
/// this goes to every member unchanged.
pub fn party_member_left(
    char_id: u32,
    expelled: bool,
    name: &str,
    party: Option<&PartyBlock>,
) -> Vec<u8> {
    let mut w = PacketWriter::with_opcode(PARTY_RESULT);
    w.u8(result::WITHDRAW);
    w.u32(char_id);
    match party {
        None => { w.u8(0); }                     // disbanded: the body ends here
        Some(p) => {
            w.u8(1);
            w.u8(expelled as u8);
            w.str(name);
            write_party_block(&mut w, p);
        }
    }
    w.into_vec()
}

/// `0x0D` - push the client's whole party state, with no message at all.
///
/// `None` clears the window. Neither path resolves a string id, so nothing is narrated.
/// This is the packet to send after any membership change that the specific messages above
/// do not already cover.
pub fn party_state(party: Option<&PartyBlock>) -> Vec<u8> {
    let mut w = PacketWriter::with_opcode(PARTY_RESULT);
    w.u8(result::UNNAMED_0D);                    // rename to PARTY_STATE
    match party {
        None => { w.u8(0); }
        Some(p) => { w.u8(1); write_party_block(&mut w, p); }
    }
    w.into_vec()
}

/// `0x03` - an invite arriving at the invitee. **Two of six fields are guesses; see below.**
///
/// `inviter_char_id` is looked up in a client-side blocked-user list; a hit auto-declines.
/// `invite_id` is echoed back verbatim in the client's `0x0183` answer, so it is the handle
/// the server will be given - use the party id unless something better exists.
/// `f4`/`f5`/`f6` are handed to the dialog and their meanings are **NOT established**; zero
/// is legal for the reader, but whether the dialog draws sensibly with zeros is untested.
pub fn party_invite(
    inviter_char_id: u32,
    invite_id: u32,
    inviter_name: &str,
    f4: u32, f5: u32, f6: u32,
) -> Vec<u8> {
    let mut w = PacketWriter::with_opcode(PARTY_RESULT);
    w.u8(result::INVITE_NOTIFY_A);
    w.u32(inviter_char_id);
    w.u32(invite_id);
    w.str(inviter_name);
    w.u32(f4); w.u32(f5); w.u32(f6);
    w.into_vec()
}

/// `0x1B` outcome 0 - **"You have invited '%s' to your party."**, for the inviter.
///
/// `outcome` selects one of eleven messages through a second jump table at `0x1413bcd04`;
/// 5, 6 and anything above 10 are silent. No further fields are read for any value, so the
/// body is always `4 + 2 + name.len()`.
pub fn party_invite_sent(outcome: i32, name: &str) -> Vec<u8> {
    let mut w = PacketWriter::with_opcode(PARTY_RESULT);
    w.u8(result::INVITE_OUTCOME);
    w.u32(outcome as u32);                       // read as raw[4], used as i32
    w.str(name);
    w.into_vec()
}

/// `0x22` - **"%s has become the leader of the party."**
///
/// **Order matters.** The client discards this silently unless it ALREADY has `partyId > 0`
/// and ALREADY holds a seat whose id matches (`0x1413bc562`, `0x1413bc57d`). Send the block
/// that seats the new leader first, then this. **[L]**
pub fn party_leader_changed(new_leader_char_id: u32, because_disconnect: bool) -> Vec<u8> {
    let mut w = PacketWriter::with_opcode(PARTY_RESULT);
    w.u8(result::LEADER_CHANGED);
    w.u32(new_leader_char_id);
    w.u8(because_disconnect as u8);
    w.into_vec()
}
```

### Tests I would write, because one of them is the whole point

* **`an_empty_seat_is_exactly_four_bytes`** - build a `PartyBlock` with one occupant and five
  defaults, and assert the five contribute `4` bytes each. This is the gate; a test that only
  checks the occupied case passes while the packet is unparseable. `CLAUDE.md`: *"a test that
  checks one of several effects gives false confidence about the rest."*
* **`a_disband_stops_after_two_fields`** - `party_member_left(id, _, _, None).len() == 8`:
  2 opcode + 1 code + 4 `char_id` + 1 still-exists, and **nothing else**.
* **`the_block_is_fixed_size_except_for_names`** - six seats + `count == 0` + 24 + 120 + name.
* **`round_trip`** - a reader in the test module that walks the bytes the way the client's
  eight read primitives do, asserting the cursor lands exactly on the end. That is the only
  check that can catch an off-by-one in the two raw blobs, and it is cheap.

### What I would NOT add

`0x2A` (§5.8) and `0x2F` (§5.9). `0x2A`'s last field is gated on client state the server
cannot observe; `0x2F` writes out of bounds for `index > 5`. Both are strictly less useful
than `0x0D`, which does the same job unconditionally.

---

## 8. What is still unsettled

1. **Nothing in this file has been sent by this server or seen on a wire.** 530 logs, 0
   `0x00A5`, control passing at 166 547 events.
2. **`0x03` fields 4-6, `MEMBER +0x14..+0x38`, `party+0x4D9`, `party+0x4E0`.** Widths and
   order are [L]; meanings are unknown and are left unnamed in the structs above rather than
   guessed. The brief asked for this and it is the honest answer.
3. **`0x03` vs `0x06`.** Still [I]. New evidence, one bit's worth: only `0x03` consults the
   invite-permission option.
4. **The second, count-driven member list.** Structure [L], purpose [I]. `count = 0` is safe.
5. **The 120-byte `MEMBER` blob and the six 20-byte records.** Byte-safe as zeros; their
   consumers were not traced, so "safe to read" is not "draws correctly".
6. **Eight indirect calls** (§6) are unfollowed.
7. **`MAX_MEMBERS`.** `research/party.md` had it as [I]. This pass makes it **[L] = 6** for
   the wire, three independent ways: `mov edi,6`, `cmp r8,6`, and `cmp eax,5 / jbe`. That
   settles the *packet*; whether the server should also cap at six is now a consequence.

---

## 9. APPENDED 2026-09-04: it has now been on the wire, and §7's builder had a two-byte defect

Everything above §7 **survived its first wire test**. The owner pressed Create; the server built the
`0x0E` body from §5.1's field list; the 209 bytes it produced decode exactly as §5.1 and §3 say,
re-derived independently from the field list without reading the Rust. **The decode is not what
was wrong.**

**One line of §7 is wrong**, and it is the line that caused the client to show *"Due to an unknown
error, your party request failed."*:

```rust
pub fn party_created(...) -> Vec<u8> {
    let mut w = PacketWriter::with_opcode(PARTY_RESULT);   // <-- WRONG
```

The `Vec` these builders return becomes `Reply.body`, and `Reply::packet()`
(`crates/world/src/session/mod.rs:43`) prepends `opcode.to_le_bytes()` itself. The wire therefore
carried `a5 00 | a5 00 0e 01 ...`, the client's first `u8` read at `0x1413baf47` returned **`0xA5`**,
`add eax,-3 / cmp eax,0x2c / ja` sent it to the **default arm** at `0x1413bcbb5`, and that arm is
the one that loads string `0x012A`. **Every builder in §7 must start `PacketWriter::new()`.**

Two things this pass could not have known and one it could:

* **`request_failed()` has the identical defect and it is invisible**, because `UNKNOWN_ERROR = 0x04`
  is itself a default-arm code — the doubled opcode produces the same message the code would have.
  So the valve "worked" and proved nothing.
* **`world.log` cannot show it.** `server::send` logs `packet[2..]`, i.e. the body, so the two bytes
  it prepends are the two it never prints. §8.4's *"nothing here has been on the wire"* was right;
  what nobody had checked is that the log of an outbound packet is the **builder's output, not the
  wire**.
* One upgrade in the other direction: §3's *"`a`/`b` together resolve to a display string - the job
  name is the obvious candidate. **[I]**"* is now **[D]**. `FUN_1402b0250` builds a job table whose
  first three inserts are key `0` -> `0x0022` *"Beginner"*, key `0x64` -> `0x002D` *"Swordsman"*,
  key `0x6e` -> `0x002E` *"Fighter"*. **`member+0x14` is the job id.** **[L]** What `+0x18` is
  remains unknown; it is that lookup's second argument, and the level may belong at `+0x1c` instead.

Full working, with the four independent legs and the controls: `research/party-create-refused-first-wire-observation.md`.

## 10. APPENDED 2026-09-05 (evening): `0x13` without the block killed both clients, and `0x0183` is the outcome byte

The owner: *"The act of inviting someone to party crashed both clients."* Fixture:
`research/fixtures/party-join-0x13-rejected-by-client-0x009E-both-clients-exit-{world,hook}.log`.

### What the wire and the hook say, in order (all **[L]**)

```text
world.log (UTC)                                   maplecw-hook.log (local, both clients in one file)
02:12:31.603 -> 0x00A5 1b 00000000 0700 "Tester2"   to 213 (leader)          [no dispatch line - see below]
02:12:31.710 -> 0x00A5 03 d5000000 01000000 0600 "Cobalt" 12000000 c8000000 00000000  to 214
02:12:31.711 <- 0x0183 {op 0x1B, answer ABSENT (=0), value 1}   from 214, ONE MILLISECOND later
02:12:31.712 -> 0x00A5 13 0700 "Tester2"            to 214   <- the name and NOTHING else
                                                    22:12:31.713  897 opcode=0x00A5 elapsed_us=1600.9 ret=1   (214's 0x03 handler returning)
                                                    22:12:31.713-.717  C++ THROW #5 #6 #7 on 214's thread
02:12:31.715 <- 0x009E CLIENT_PACKET_REJECTED: 01 00 26000000 1000 ce4cc24c | a5 00 13 07 00 "Tester2" | a5 00
02:12:31.782 -> 0x00A5 13 0700 "Tester2"            to 213
                                                    22:12:31.792-.797  C++ THROW #5 #6 #7 #8 on 213's thread - the SAME three stacks
                                                    22:12:34.812 / .816  SOCKET ... closed and cleared by the client (FUN_1415e3b60 ran), both
02:12:34.630 / .653  both channel connections gone
```

* **The client named the packet.** `0x009E` carries the offending packet verbatim: opcode
  `0x00A5`, body `13 07 00 54 65 73 74 65 72 32` - the `0x13` with the name and no block. There
  is no inference in identifying the killer.
* **Both clients threw the same three stacks** (`0x142ef6ddc` - the throw site every C++ THROW
  in the log shares - then `0x1401d67aa`, `0x1406e8cb1`, `0x1406e90e8`, `0x14019b7eb`,
  `0x1406f2fed`). `0x1406e8cb1`/`0x1406e90e8` are inside the read primitives §3 lists;
  `0x1406f2fed` is inside `FUN_1406f2fd0`, the `PARTYBLOCK` reader. Reading a block that is
  not there is what threw. No `CLIENT FAULT`, no dump: this was a caught exception followed by
  the client's own orderly shutdown, not a crash in the WER sense.
* **§5.2 was right and the builder was wrong.** `str name, PARTYBLOCK` has been in this file
  since 09-04; `net::party::joined` wrote `str`. The block is now written by
  `net::party::write_party_block` exactly as §4 lists it, and the size identity (4 bytes per
  empty seat, 155 + name per occupied one, 185 for an empty block) is a test.
* **The `0x1B` outcome to the leader has no dispatch line** and did no harm - 213 kept
  dispatching for another 180 ms and died of the `0x13`. Why the hook wrote no line for it is
  **not established**; it is noted here so nobody reads that absence as the `0x1B` being the
  killer.

### `0x0183`'s answer byte, read off the client (**[L]** paths, **[D]** which button is which)

The first `0x0183` ever decoded arrived **1 ms after the `0x03`**, inside the handler's own
1.6 ms - no human clicked. It was taken for an accept ("anything but 1"), which is what sent the
fatal `0x13`. `tools/listing.py 0x1413bab80` over the `0x03` arm (`0x1413baf80..0x1413bb136`):

```text
1413bafc9  mov edi, r14d                      ; answer = 0
1413bafd3  cmp [global+0x154], 0 / je bb131   ; invite option off  -> edi = 1, jump to the send
1413bafea  FUN_142d01050(list, field1) / jne bb131 ; inviter blocked -> edi = 1
1413bb08f  FUN_142d98230(ctx, 0x19, &entry, field2) ; already holds THIS invite -> edi = 3
1413bb11a  FUN_142d98230(ctx, 0x19, &.., 0)          ; holds ANOTHER one        -> edi = 2
1413bb0a2  new packet 0x183; {0x1B, edi, field2}; FUN_1413bd500; send      <- ALWAYS, first
1413bb0dc  test edi,edi / jne bb245             ; nonzero: done, no dialog
1413bb0e4  alloc 0x370, FUN_141808b90 (dialog), FUN_141810230(dlg, name, field2, f4, f5, f6)
```

and the two dialog callbacks: `FUN_14180b750` encodes `word 0x51b` = `{0x1B, 5}` (and
`0x51c` = `{0x1C, 5}` for the join-request twin); `FUN_14180c6e0` encodes `0x41b` / `0x41c` =
answer **4**. Value is `[obj+0x328]`, field 2 as saved by the dialog.

So the answer byte is **§5.7's outcome numbering**: 1 blocking, 2 busy, 3 already invited,
4 denied - the leader's own table - with **0 = "received, the dialog is opening"** (sent by
the handler, not a click) and **5 = the one outcome §5.7 leaves silent**, i.e. Accept, whose
announcement is `0x13`. That 5 is Accept and 4 Decline is **[D]**: 4 lines up with *"%s has
denied the party request"*, and `FUN_14180b750` (the 5) is the callback that calls the party
UI's apply path with flag 1 where `FUN_14180c6e0` (the 4) calls it with flag 0. The paths and
constants are [L]. `net::party::invite_answer` is the owner; the world relays 1..=4 to the
leader as their own `0x1B` sentence, accepts on 5, and does nothing at all on 0.

### What this does NOT establish

* Whether the dialog draws sensibly with level and job in fields 4-5. It was constructed
  (answer 0, 1.6 ms) and died with the client three seconds later. Still the next run's
  measurement.
* Whether the member list refreshes on `0x13`'s block. The block has never reached a living
  client.
* Which of `0x03`/`0x06` is invite - unchanged from §8.3; only `0x03` was sent.

## 11. APPENDED 2026-09-05 (later): the party now works on a screen, and the six requests after it

The join drew both members on both clients, at the level and job the block carries (the owner's
screenshot: the party window listed *Cobalt Magician 18* and *Tester2 Beginner 8*). Accept is
answer **5**, confirming §10. Six follow-on requests were then built, all server-side and
unit-tested; the ones that need a client-measured opcode are called out.

* **Leave / expel / disband now answer with `0x10`.** The Leave transition was in the session's
  *undecoded* list, so it was answered with `UNKNOWN_ERROR` and the member stayed in the party
  (*"Tester2 cannot also leave"*). `net::party::member_left` builds §5.3: `u32 charId, u8
  stillExists, [u8 expelled, str name, PARTYBLOCK]`, disband stopping after the still-exists
  byte. `Effect::Departed`/`Disbanded` are handled.
* **Pick-up rights (action 2) is routed and stored, and answered with `0x0D`.** There is **no
  standalone "rights changed" packet in this client** - the string `0x011A` is only ever shown
  as a side effect of the `0x2D` public/private arm (`FUN_142d1d130(edi=0)`), and no `0x00A5`
  arm stores the rights global `0x2710497` from the wire. So the server stores the mode
  (`Party::pickup_rights`, slot 1 of the tag-5 payload, default 1) and refreshes the window
  with `party_state` (`0x0D`), which removes the *"unknown error"*. Drop visibility is by
  membership, not by this value.
* **Party EXP, quest credit and drops are shared by membership**, not by damage - see
  `research/exp-sharing.md` (updated) and `crate::mobshare::may_see_drop`, which already took a
  `Party`. A party drop is owned by the killer and shown to every current member on the field;
  a member who leaves is off the live roster the pick-up resolves and loses it, while the
  killer keeps it. A player's own ground drop is public to the whole map.

Still needing a client-measured opcode, and therefore **not built** (guessing a body has
killed this client three times): the **party member HP** push - no `0x00A5`-adjacent packet
carries a member's live HP, and the client sends only a one-byte `0x00B8` toggle, not its HP -
and the **meso drop** request, which does not appear in any capture (the client may send a
distinct opcode for it, or none). Both want one measurement: the owner dropping mesos, and a party
member taking damage, with the inbound opcode read off `world.log`.
