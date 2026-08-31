# `0x0293` verified against the client, end to end

Written 2026-08-31, **no Ghidra** (three sibling agents hold the project lock), **no client
run.** Everything below is `tools/listing.py`, `tools/reads.py`, `tools/callers.py`,
`tools/pdata_lookup.py`, `tools/rangescan.py` and four throwaway scripts (§10), all run from
the repo root so the scratchpad's stale copies could not shadow `tools/`.

Tags on every claim: **[L]** read off the listing / a raw table / a capture; **[D]** derived
from two or more of those; **[I]** inferred.

> ## STILL NOT ON A WIRE
>
> This document verifies that the bytes match. It does **not** say the feature works.
> `0x0293` has never been sent by this server and `FUN_141d598b0` with `arg3 = 0` has never
> been exercised by this server at all — see §1.3, which measures that rather than assuming
> it, and §5.6, which names the one thing that would have been a positive control and is
> itself unwired.

---

## 0. The answers, up front

| question | answer |
|---|---|
| does the body the server builds match what the client reads? | **Yes, to the byte, on all 6689 captured paths.** Server writes `4 + path_len`; client reads `4 + 14 + sum(element_len)`, and the element table was re-derived from the image with 0 disagreements. §2, §3 |
| is the path echoed verbatim, and is the slice right? | **Yes.** `path_span`, not `path_with_key_states_span`, and the ten-byte head correctly dropped. Re-confirmed from the listing, independently of `research/user-pool-tables.md`. §3 |
| is there a fault path for an unexpected element command? | **No.** All 256 byte values are handled; `> 0x4e` falls into the 5-byte common tail. The only way to fault the decoder is to run out of bytes, and that **throws a C++ exception**, it does not read wild. §4 |
| does the `(0, 0)` announcement self-heal on the first step? | **Yes — but NOT for the reason `multiplayer.rs` and `research/user-move.md` §8 give.** The path head's x/y is decoded and then **thrown away** by the client. The correction comes entirely from the elements. §5 |
| anything that would kill a client? | **One reachable null dereference**, `141d59bef`, on a zero-element path — measured absent from 170 806 captured paths but **not guarded by this server**. §6.1 |
| anything that draws wrongly? | **Three.** A spawn snapshot that is never refreshed (§6.2), a `last_position` that survives a map change (§6.3), and both clients being granted control of every mob (§6.5). |

---

## 1. The instruments, and the controls that ran first

### 1.1 `tools/reads.py`, the documented control

```
python tools/reads.py 0x140304100 2
```

reproduces `research/user-pool-tables.md` §1 exactly: `raw @140304138`, `u8 @140304144`,
`u8 @140304183`, then a run of `u16`. `tools/reads.py` on disk is **9605 bytes**, the live
one. **[L]**

`tools/dis.py` does not exist in this checkout — the brief named it; `tools/listing.py` is
the tool it meant.

### 1.2 The archive grep, controlled before any negative was taken

| search | result |
|---|---|
| files containing `0x03C6` (mob enter field, known to be sent constantly) | **81** — the search works |
| lines `<- 0x02FF` (mob move report, inbound) | **290 376** |
| lines `-> 0x03D9` (mob move push, outbound) | **0** |
| any line containing `0x0293` | **1**, and it is `research/fixtures/sweep-01f2-03c7-exit.log:399`, `01:05:21 >>> opcode 0x0293 +32B` |
| any line containing `0x0224` | **1**, same file, `01:05:00` |

The two singletons are the login-stage sweep that `research/user-pool-tables.md` already
disqualifies: `CField::OnPacket` was not the active dispatcher, so `FUN_1429bb720` was never
reached. **The negative is therefore: no `0x0293` has ever reached the user pool.** **[L]**

### 1.3 The thing that would have been a positive control is itself unwired

`0x03D9` `MobMove` is the *other* packet that goes through `FUN_141d598b0` with `arg3 = 0` —
byte-identical machinery from the wrapper down. If it had ever been sent, it would settle
most of this document without a run.

```
grep -rn "mob_move_broadcast" crates/world/src/ --include=*.rs
   crates/world/src/session/combat.rs:37:   ... in a DOC COMMENT
```

**`net::mobmove::mob_move_broadcast` has no production caller**, and the archive has zero
outbound `0x03D9`. So `0x0293` will be the **first packet this server has ever pushed through
`FUN_141d598b0`**. **[L]**

---

## 2. The route to the handler, re-derived rather than looked up

`research/user-pool-tables.md` §3 says table C index 0 is `0x0293`. Re-derived from the raw
table bytes rather than taken from the document: **[L]**

```
byte index table 0x1429BBD10[0x00]  = 0x00
dword arm table  0x1429BBCD0[0x00]  = 0x1429bbb38
0x1429bbb38  mov rdx,rdi / mov rcx,rbx / call 0x1429d2e70 / jmp 0x1429bbc08
```

and the whole 0x31-entry byte table comes out matching §2's census — 15 opcodes on real arms,
34 on the default `0x1429bbc08`.

`tools/reads.py 0x1429d2e70 4` reports **exactly one read site**, and it is the call into
`FUN_141d598b0`. `tools/reads.py 0x1429bb720 1` reports the `u32` at `1429bb745` as the only
direct read before the dispatch. Both reproduce. **[L]**

### 2.1 Eight silent-drop gates sit in front of the handler, and one of them is new

Walking `FUN_1429bb720` from the listing, every one of these returns through the plain
epilogue at `0x1429bbc19` having read only the `u32` charId: **[L]**

```
1429bb756  [pool + 0xf8]  == 0                    the bucket array
1429bb770  bucket         == 0
1429bb784  hash walk falls off the chain          <- charId not in the pool
1429bb790  [node + 0x18]  == 0
1429bb79d  [node + 0x20]  == 0
1429bb7aa  [that + 0x28]  == 0                    rbx becomes the CUser here
1429bb7be  FUN_142cc3d80  != 0
1429bb7dd  FUN_141bc8c60  != 0                    plus three more field-state gates
```

and then, **inside the handler itself**, a ninth that `research/user-pool-tables.md` does not
list:

```
1429d2e76  mov rcx,[rcx + 0x11b8]
1429d2e83  je  0x1429d2ed0            <- NULL: return, having read nothing
```

`tools/rangescan.py 0x11b8 0x1429b0000 0x1429f0000` finds **7 sites**, and the one that
*installs* that pointer is `1429ce928 mov [r14 + 0x11b8], rbx` — inside **`FUN_1429ce270`**,
which is `0x0224` `UserEnterField`'s `CUser::Init` decoder (`research/user-enter-field.md`).
The surrounding code is a `QueryInterface` pattern (`cmp ecx, 0x80004002`). **[L]**

**So `0x0224` is not merely "first in the sequence" — it is the packet that creates the object
`0x0293`'s handler dereferences.** If the spawn is dropped by any of the six gates in
`user-enter-field.md` §5, every subsequent `0x0293` returns silently at `1429d2e83` and
movement looks broken with nothing in any log. That is the T1 discriminator in §8. **[D]**

---

## 3. Question 1: the server writes N, the client reads M, and N == M

### 3.1 The client's read ledger for `0x0293`

```
off  size  read at      by
  0  u32   1429bb745    FUN_1429bb720, before the dispatch          -> charId
  4  u32   1404b2650    FUN_1404b2630                               -> path+0x40
  8  i16   1404b265b                                                -> x   (see §5!)
 10  i16   1404b2675                                                -> y
 12  u16   1404b2690
 14  u16   1404b269c
 16  i16   1404b26a9    movsx / test / jle -> bail                  -> element count
 18  ...   per element, 1 command byte + a payload from the 79-entry jump table
                        END.  141d5991a `je` skips the key-state trailer.
```

`FUN_141d598b0` reads nothing else: `141d598dd mov ebx,r8d` saves arg3, `141d59918 test
ebx,ebx / je 141d5997a` skips both `141d5991f` and `141d59936`. `FUN_1429d2e70` supplies
`1429d2eb5 xor r8d,r8d`. **[L]**

### 3.2 The element table, re-derived from the image

`research/user-move.md` §3 and `net::usermove::element_len` were **not** trusted. The byte
index table at `0x1404b2f24` (79 entries) and the dword arm table at `0x1404b2ef0` (13 arms)
were read out of the PE, each arm walked by recursive descent over `tools/listing.py`'s own
output — which annotates every read primitive with its width — and the totals compared:

```
cmd   arm          image paths (+1 cmd byte)   usermove.rs   verdict
0x00  0x1404b2755  [21, 23]                   21            ok
0x0c  0x1404b2ca0  [2]                        2             ok
0x0f  0x1404b2755  [21, 23]                   23            ok
0x3d  0x1404b2d56  [5, 15]                    15            ok
0x4f  0x1404b2d56  [5, 15]                    5             ok     (past the table)
...
mismatches: 0     (79 commands + 0x4f, 0x80, 0xff)
```

The two-valued rows are the two command-gated branches, and both gates are in the listing:
`1404b27dc cmp al,0xf / je` then `1404b27ec cmp al,0x11 / jne` for the ninth `u16`, and
`1404b2d65 sub eax,0x3d / test eax,0xfffffffd / je` for `0x3d`/`0x3f`. **[L]**

**And the decoder ignores args 3 and 4.** `r9d` never appears in its 2371 bytes; `r8d` appears
twice, both times written before read (`1404b270e movzx r8d,cl`, `1404b2df2 movzx r8d,al`).
The trailer gate is entirely in the wrapper. This reproduces `research/user-pool-tables.md`
§3's claim by a different route. **[L]**

### 3.3 One captured body, end to end

`14:01:40.792` from `research/fixtures/character-on-map1-playable-world.log`, the same body
`net::usermove`'s tests use:

```
inbound 0x00D9   97 bytes = 10 head + 14 path head + 3 x 21 elements + 1 + 9 key states
outbound 0x0293  81 bytes = 4 charId + 77 path                     <- what the server builds
client reads     81 bytes = 4 (1429bb745) + 14 (1404b2650..26a9) + 63 (three 0x00 elements)
                                                                        MATCH
```

### 3.4 The whole archive

185 world logs over `previous-runs/` and `research/fixtures/`, deduplicated on
`(timestamp, opcode, body)` — the **event**, not the file, because the two directories overlap
and eleven fixture/run pairs are the same run copied mid-write (`CLAUDE.md`):

| | |
|---|---|
| raw `0x00D9` body lines | 10 921 |
| distinct events | **6689** |
| walk closed with the image-derived table | **6689 of 6689** |
| bodies where server-written == client-read | **6689 of 6689** |

`crates/net/src/usermove.rs` says 5221 over 172 logs on 2026-08-29; the archive has grown to
185 logs since. Both numbers are right for their day. **[D]**

Outbound length minus inbound length, over the same 6689:

```
delta = -16   6579      (10 head + 1 count + 9 nibble bytes dropped, 4 charId added)
delta = -15      9
delta = -14     15
delta = -13     16
delta = -12     19
delta = -11     15
delta = -10     12
delta =  -9     12
delta =  -8     12
```

**The rebroadcast is always shorter than the report it echoes**, which is the direction that
cannot desynchronise a stream by over-reading.

---

## 4. Question 3: an unexpected element command has no fault path

`1404b2731 cmp eax,0x4e / ja 0x1404b2d56` sends every byte above `0x4e` to the same block the
case bodies converge on. That block re-reads the deobfuscated command, tests it against
`0x3d`/`0x3f` (which it cannot be), and falls into the common tail at `0x1404b2d7f`:
`u8 @1404b2d82`, `u16 @1404b2db7`, `u8 @1404b2dce`. **Five bytes, no branch, no error.** **[L]**

So **every one of the 256 possible command bytes decodes**, and `element_len`'s `_ => 5` is
the listing's own behaviour rather than a guess.

### 4.1 What *does* fault is running out of bytes — and it throws

This matters because the equip crash's shape was *"a short packet read past its end and died
in a destructor"*. The packet reader does not read past its end:

```
1406e8c20  FUN, read u32
1406e8c32  mov edi,[rcx+0x18] / sub edi,[rcx+0x24]     length - position
1406e8c79  cmp edi,4 / jb 0x1406e8c91                  not enough left
1406e8c91  mov edx,0x26 / lea rcx,[rsp+0x28] / call 0x1401bb8b0
1406e8ca0  lea rdx,[rip+0x3352471] / call 0x142ef6d4c  <- _CxxThrowException
1406e8cb1  int3
```

`0x142ef6d4c` loads `0x19930520`, the MSVC EH magic. `FUN_1406e8ae0` (read `u8`) is the same
147-byte shape with `cmp edi,1`. **An over-read is a C++ throw, not a wild read.** **[L]**

Whether `CField::OnPacket` catches that throw or lets it reach the client's own crash reporter
is **not established here** and would need a run or a Ghidra pass on the unwind data.

---

## 5. Question 4: the `(0, 0)` bug self-heals — and the stated mechanism is wrong

`crates/world/src/session/multiplayer.rs` and `research/user-move.md` §8 both say:

> *"It corrects itself on their first step, because `0x0293` moves a remote user the client
> already has."*

The conclusion is right. **The mechanism named is not**, and the difference decides what the owner
will see on screen.

### 5.1 The client saves the path object's x/y, decodes, and writes the old values back

```
141d598c4  mov  eax,[rcx + 0xc]              old element count -> [rsp+0x70]
141d598e0  mov  edx,[rbp + 0x24]             obfuscated x
141d598e3  call 0x1401ab420                  deobfuscate
141d598f0  mov  [rsp + 0x80],eax             SAVED x
141d598e8  mov  edx,[rbp + 0x2c]
141d598f7  call 0x1401ab420
141d59903  mov  [rsp + 0x88],eax             SAVED y
141d59910  call 0x1404b2630                  -- writes the PACKET's head x/y into +0x24/+0x2c
141d5997a  movzx ecx,word [rsp + 0x80]       the saved x again
141d59986  call 0x1402f7010                  re-obfuscate
141d59997  mov  [rbp + 0x24],eax             RESTORED
141d5999a  call 0x1402f7010
141d599a7  mov  [rbp + 0x2c],eax             RESTORED
```

`FUN_1404b2630` writes the packet's head position at `1404b2672 mov [rbx+0x24],eax` and
`1404b268d mov [rbx+0x2c],eax` — exactly the two slots that are then overwritten. **[L]**

**The path head's x/y in a `0x0293` is decoded and discarded.** A remote user's position comes
*only* from elements whose command carries one.

### 5.2 Which means the heal depends on the elements, and the archive says it always can

`net::usermove::element_carries_position` names 31 of the 79 commands, and re-counting from
the set gives 31, matching the crate's own assertion. Over the 6689 distinct captured paths:

| | |
|---|---|
| paths whose **first** element carries an absolute position | **6675 of 6689** |
| paths with **no** position-carrying element anywhere | **0 of 6689** |

First-element census: `0x00` x6637 (carries), `0x0f` x38 (carries), `0x23` x11, `0x0c` x2,
`0x0e` x1 (the last three do not, and inherit). **[D]**

**So every captured walk would place the remote user somewhere absolute.** The heal is real.
What is *not* decidable without a screen is whether the transition from the map origin to the
first element's coordinates renders as a **pop** or as the character **sliding across the
map** — the displayer interpolates between elements, and the element list it is interpolating
from starts at `(0, 0)`. That is §8's step 3.

### 5.3 The complete element command census over the archive

Nine distinct commands have ever been observed in a `0x00D9`:

```
0x00  x24438  21 bytes  carries position     0x03  x6    15 bytes  carries
0x01  x1508    9 bytes  inherits             0x05  x4    15 bytes  carries
0x02  x268     9 bytes  inherits             0x0c  x74    2 bytes  inherits, NO common tail
0x0e  x33     11 bytes  inherits             0x0f  x194  23 bytes  carries, gated 9th u16
0x23  x268     5 bytes  inherits, no payload
```

Both of the table's odd cases — the tail-skipping `0x0c` and the gated `0x0f` — are in real
traffic, so the two rows most likely to be wrong are the two with live evidence. **[D]**

---

## 6. Question 5: what would kill a client, and what would draw wrongly

### 6.1 KILL — an unconditional null dereference on a zero-element path

`FUN_141d598b0`, after the decode and on **every** path through the function:

```
141d599af  je 0x141d59a5b          nothing appended -> skip the checksum loop
   ...  straight line, no further branch that leaves this block  ...
141d59bef  mov   rax,[rbp + 0x18]              the element list TAIL
141d59bf3  movups xmm0,[rax]                   <-- dereference, no null check
141d59bf6  movups [rbp + 0x64],xmm0            0x78 bytes copied out of the tail element
   ...  seven more movups/movsd, +0x10 .. +0x70  ...
141d59c46  call 0x1401ba9d0                    only NOW is anything tested
```

`[pathObj + 0x18]` is the list tail: `FUN_1404b4790` stores head at `[list+8]` and tail at
`[list+0x10]`, and the list handle is `pathObj + 8` (`1404b26be lea rax,[rbx+8]`). It is null
iff the list is empty. **[L]**

`FUN_1404b2630` appends nothing when the element count is `<= 0` — `1404b26b3 movsx / test /
jle 0x1404b2ee4`, straight to the epilogue. So **a `0x0293` whose path claims zero elements,
delivered to a remote `CUser` whose path list is still empty, dereferences NULL.** **[D]**

*How reachable is that?* Measured, over both packets built by the same encoder
`FUN_141d57c60`, deduplicated on events:

| | distinct events | with `element_count <= 0` |
|---|---:|---:|
| `0x00D9`, the player's own move | 6 689 | **0** |
| `0x02FF`, the mob move report | 164 117 | **0** |
| **total** | **170 806** | **0** |

`FUN_141d57c60` has never been observed to emit a zero-element path. **[D]** So this is a
latent hazard, not an active one — but the server has **no guard against it**:
`parse_user_move` returns early only on `element_count < 0`, and a body of
`10 + 14 + 1 + ceil(k/2)` bytes with `element_count == 0` **walks closed**, hands over a
14-byte `path_span`, and is rebroadcast as an 18-byte `0x0293`. §7 is a two-line fix.

### 6.2 DRAWS WRONGLY — the spawn snapshot is never refreshed

`Session::presence` builds the `0x0224` body **once**, at field entry, with
`self.remote_at()` — and `remote_at` reads `last_position`, which is `None` on arrival, so
`(0, 0)`. `Bus::enter_field` hands that stored `Presence::spawn` to every **later** arrival.

```
grep -rn "refresh_spawn" crates/ --include=*.rs
   crates/world/src/broadcast.rs:329:    pub fn refresh_spawn(...)      <- the definition
```

**Zero callers.** **[L]** So: A enters and walks around for ten minutes; C then enters and is
told A is at the map origin. C sees A at `(0, 0)` until A's next step. This is a strictly
worse case than the one `multiplayer.rs` documents, because A *has* a known position and it is
simply never used. `Bus::refresh_spawn` exists for exactly this and is unwired.

### 6.3 DRAWS WRONGLY — `last_position` survives a map change

`last_position` is set at `Session::new` to `None`, written by the `0x00D9` arm
(`session/mod.rs:761`) and by `on_attack` (`session/combat.rs:231`), and **cleared nowhere**.
A portal walk therefore leaves the *previous map's* coordinates in it, and
`announce_field_entry` builds the new map's `0x0224` from them.

`net::userpool::RemoteAt::foothold`'s own doc block warns about precisely this class:
*"Sending a foothold id from a different map is not legal and is the mistake to watch for when
this is wired to a stale position."* The foothold is hardcoded `0`, so that half is safe. The
x/y half is the live case and nobody wrote it down. **[L]** for the absence of a reset.

`multiplayer.rs`'s module docs say the residue is *"`(0, 0)` on a fresh arrival"* — true only
for a connection's **first** field entry.

### 6.4 SILENT — a `0x0293` published without a matching `0x0224`

`Session::publish_user_move` takes the id from `self.claimed.character_id`;
`Session::announce_field_entry` takes it from `claimed_character()`, which does a store lookup
and returns `None` if the row cannot be loaded. So a session whose character row fails to load
sends no spawn and still sends moves. The client drops them at `1429bb784` in silence, so this
costs nothing on screen — but it is the shape that makes "movement does not work" unfalsifiable
from the world log alone. Worth one `log()` line, not a fix.

Ordering itself is **correct**: `Session::handle` runs `dispatch` first and appends
`collect_mail` after, so the `0x0224`s that `on_field_entered` returns always precede any
`0x0293` from the mailbox. `Inner::post` replaces a superseded entry **in place**, so
coalescing cannot reorder a move ahead of a spawn either. **[L]**

### 6.5 READS AS A BUG — both clients are granted control of every mob

Not this document's subject, but it will be on screen during T1 and would otherwise be read as
"the movement broadcast broke the mobs".

`session/field.rs:119` pushes a `MOB_CHANGE_CONTROLLER` for every mob on **every** field entry,
per session, unconditionally. With two clients on one map, **both** are told they control every
mob. Each rolls its own wander (`research/mob-behaviour.md`: the controlling client runs the
movement), each sends its own `0x02FF`, each is acked. So:

* the two screens will **disagree about where the mobs are**, and
* `Fields::note_position` takes whichever `0x02FF` arrived last, so a drop lands where
  *someone's* client thought the mob was.

And because `0x03D9` is unwired (§1.3), nothing corrects either screen. **[D]**

### 6.6 NOT A PROBLEM — the flood gate

`1429bb8ae` runs a per-remote-`CUser` counter: a 1000 ms window at `[user+0x40d0]`, a count at
`[user+0x40d4]`, a cap of 100 (200 in some field state, `1429bb8f8 mov ecx,0x64 / mov edx,0xc8
/ cmovg`). Over cap sets `[user+0x40d8] = 1` **for the rest of the session**, and the throttled
path at `0x1429bbaea` skips table B — the four attack opcodes — while **falling through into
table C**, which is where `0x0293` lives. **[L]**

So a movement flood cannot stop movement being drawn; it would permanently stop that player's
*swings* being drawn. At the measured ~510 ms cadence that is 2 packets/s against a cap of 100,
with a 100 ms server tick (`crates/world/src/server.rs:47 TICK_MS`). Not a live risk. Worth
knowing because the failure it produces — attacks stop, movement keeps working, forever — is
otherwise unattributable.

---

## 7. WIRE IT LIKE THIS

Two changes, both small, both in files this document did not touch.

### 7.1 Refuse a zero-element path (§6.1)

`crates/world/src/session/multiplayer.rs`, in `Session::publish_user_move`, immediately after
the `let Some(chr)` / `let Some(map)` bindings and **before** `m.path(body)`:

```rust
        // `141d59bef mov rax,[rbp+0x18]` then eight `movups` out of the element list's TAIL,
        // with no null check and no branch between it and the function entry. The list is
        // empty when `FUN_1404b2630` appended nothing, which is exactly `element_count <= 0`
        // (`1404b26b3 movsx / test / jle`). `walk_closed` does NOT cover this: a body of
        // 10 + 14 + 1 + ceil(k/2) bytes with a zero count walks closed and yields a 14-byte
        // path. No captured path has ever had one - 0 of 170 806 events across `0x00D9` and
        // `0x02FF`, `research/remote-move-verification.md` §6.1 - so this costs nothing and
        // removes the only null dereference this packet can reach.
        if m.element_count <= 0 {
            crate::server::log(
                "   move NOT rebroadcast: element_count <= 0, and 0x0293's decoder \
                 dereferences the element list tail unconditionally at 141d59bef",
            );
            return;
        }
```

The matching test belongs next to
`a_path_that_did_not_walk_closed_cannot_be_rebroadcast` in `crates/net/src/userpool.rs`, and
it must assert the **refusal**, not just the length — a 25-byte body with `element_count = 0`
parses, walks closed, and hands over a span today.

### 7.2 Give an arriving character a real position, or refresh the snapshot (§6.2, §6.3)

Two independent halves; either alone is an improvement.

* **Clear `last_position` on a field change.** It is written in two places
  (`session/mod.rs:761`, `session/combat.rs:231`) and cleared in none. `Session::on_field_entered`
  (`session/field.rs:19`) is the one hook that runs on every entry including a portal walk, and
  it already resets the chatter cursors for the same reason. Setting `self.last_position = None`
  there turns §6.3's *stale coordinates from another map* into §6.2's known `(0, 0)`.
  **Do it before `announce_field_entry()` on line 57**, or the spawn is built from the old map.

* **Call `Bus::refresh_spawn` from the `0x00D9` arm.** `crates/world/src/broadcast.rs:329`
  exists, is documented for exactly this, and has zero callers. Rebuilding `presence().spawn`
  each time `last_position` changes costs one `user_enter_field` per movement report, which is
  ~2/s per player; rebuilding it on a timer, or only when a new arrival asks, is cheaper. The
  packet is not sent to anyone who already has the character — `refresh_spawn` deliberately
  does not announce — so this is safe in the way a second `0x0224` would not be
  (`1429ba556`: a spawn for an id already in the pool returns without reading the body).

The real fix named in `multiplayer.rs` — portal coordinates out of the WZ — is still the right
one and is still a `tools/dump_portals.py` change.

---

## 8. What to watch for on the next two-client run (T1)

`tools/test-server.ps1`'s T1 currently tests **appearance** only. The three outcomes it lists
are right and should stay. What follows is a **movement** step to run after them, phrased so
each outcome names a cause. Both copies of the plan need it — the `.NOTES` block and the
`Write-Host` block.

The owner launches, as always, from an elevated window:

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

**Get both characters onto one map, then, with client A standing still, walk client B.**

| what A's screen shows | what it means |
|---|---|
| **B does not appear at all** | Stop. This is T1(c) and the move step says nothing. `0x0224` was dropped — §2.1 shows the spawn is what installs `[user+0x11b8]`, so **every** `0x0293` is also being dropped at `1429d2e83` as a consequence. Next run carries the watch on `0x1429ba60b`. |
| **B appears at the map ORIGIN (far left, on the ground) and stays there while B walks** | The spawn landed and the move did not. Two causes, and they are separable in one grep: if `world.log` has `-> 0x0293` lines, the packet went out and the client dropped it — the pool lookup at `1429bb75c` or the `+0x11b8` null. If it has none, it is server-side: `walk_closed` false, or `map_of` returning `None`. |
| **B appears at the ORIGIN and then MOVES to the right place on their first step** | **The feature works.** The origin is §6.2's known bug, not a new one. Say whether B **popped** to position or **slid/walked** across the map — §5 says the path head's coordinates are discarded and the displayer interpolates from `(0, 0)`, so the slide is the predicted behaviour and the pop would mean §5.1 is wrong. |
| **B appears at the RIGHT place immediately** | Also a result, and it contradicts §6.2 — it would mean something is supplying a position at entry that this document could not find. Worth saying so. |
| **A client dies while B is walking** | Say **which** one. **A** (the observer) dying is the serious case: it is the first `0x0293` ever decoded, and §6.1 names the one null dereference in that path. **B** (the mover) dying cannot be the broadcast — B is never sent one — and is a session-age effect, which `CLAUDE.md` says to separate first by re-testing at ~40 s of client life. |
| **The mobs disagree between the two screens** | **Expected, and not this feature.** §6.5: both clients are granted control of every mob and each rolls its own wander. Do not read it as the movement broadcast misbehaving. |

Two supporting checks that cost nothing and are worth asking for in the same breath:

* **Count `-> 0x0293` in A's `world.log` against B's walking.** ~2 per second while B moves.
  Zero with B walking is the second row above; a burst far above that would be worth knowing
  because of §6.6.
* **Have A walk too, at the same time**, once B has been seen. That is the first time both
  directions are live, and it is the case where the supersede key in `Bus::publish` can
  actually fire.

---

## 9. What is NOT established

1. **Nothing here has been on a wire.** No `0x0293` has ever reached `FUN_1429bb720` (§1.2),
   and `FUN_141d598b0` with `arg3 = 0` has never been driven by this server at all (§1.3).
2. **Whether `CField::OnPacket` catches the packet reader's C++ throw** (§4.1). The read
   primitives throw rather than over-read, which is better than the equip crash's shape, but
   what happens to that exception is unread — it needs the unwind data, i.e. Ghidra.
3. **Whether the `(0, 0)` -> real-position transition pops or slides** (§5.2). Decidable only
   on a screen. It is step 3 of §8.
4. **`FUN_141d58e60`'s callers remain unknown** — the named blind spot
   `research/user-pool-tables.md` §6.3 records. Zero by `call`, by tail `jmp` and by data
   pointer, so if it is virtual a second remote-move opcode could exist behind a vtable slot.
   This document did not close it and did not try; quoting it is the condition
   `CLAUDE.md` puts on acting on a `[D]` negative.
5. **Whether the element list is ever emptied between packets.** `141d59c59 lea rcx,[rbp+8] /
   call 0x1404b4af0` clears it when the accumulated duration exceeds a budget
   (`141d59c4b lea ecx,[rbx+0x1388]`, i.e. budget + 5000), and re-seeds it from the copy the
   null dereference made. That is enough to establish §6.1's hazard and **not** enough to say
   how the list behaves over a long session.
6. **The meaning of the path head's `u32` at +0 and the two `u16` at +8/+10** is still
   unestablished, exactly as `research/user-move.md` §8 says — and now with the extra note
   that the head's *x/y* are established to be **discarded** (§5.1), which makes the whole
   14-byte path head carry nothing an observer uses except the element count.

---

## 10. Reproducing every number

All of these run **from the repo root**. The four throwaway scripts were run as
`python - < script.py`, which leaves `sys.path[0]` empty so the scratchpad's stale
`reads.py` / `listing.py` / `callers.py` cannot shadow `tools/`.

```
python tools/reads.py    0x140304100 2        # the documented control - run this FIRST
python tools/reads.py    0x1429bb720 1        # ONE direct read, the u32 charId at 1429bb745
python tools/reads.py    0x1429d2e70 4        # ONE read site, and it is the path call
python tools/reads.py    0x141d598b0 1        # the decode call, then the two gated trailer reads
python tools/listing.py  0x1429bb720          # the eight gates, both tables, the flood counter
python tools/listing.py  0x1429d2e70          # 1429d2eb5 XOR R8D,R8D before 1429d2ec6
python tools/listing.py  0x141d598b0          # the save/restore of x/y, and 141d59bef
python tools/listing.py  0x1404b2630          # the 79-command decoder
python tools/listing.py  0x1406e8c20          # the u32 reader's bounds check and its throw
python tools/pdata_lookup.py 0x14099fe20      # no .pdata entry: it is `lea rax,[rcx+0x640]; ret`
python tools/rangescan.py 0x11b8 0x1429b0000 0x1429f0000    # 7 sites; 1429ce928 is the setter
```

The four scripts, all run from the repo root:

**1. Table C, from the raw table bytes.** Confirms `0x0293` -> index 0 -> `0x1429bbb38`.

```python
import struct, sys
sys.path.insert(0, "tools")
from rtti import load_pe
DATA, BASE, SECTIONS = load_pe("client-patched/MapleStory.exe")
def foff(va):
    rva = va - BASE
    for s in SECTIONS:
        if s['vaddr'] <= rva < s['vaddr'] + s['vsize']:
            return s['raddr'] + (rva - s['vaddr'])
    raise ValueError(hex(va))
idxs = [DATA[foff(0x1429BBD10 + i)] for i in range(0x31)]
arms = [BASE + struct.unpack_from("<I", DATA, foff(0x1429BBCD0 + j*4))[0]
        for j in range(max(idxs) + 1)]
for i, b in enumerate(idxs):
    print(hex(0x293 + i), "index", hex(b), hex(arms[b]))
```

**2. The element table, re-derived from the listing.** Feed it
`python tools/listing.py 0x1404b2630 > pathdec.txt` first; it parses that file's
`<<< READ u16` annotations for widths, reads the two jump tables out of the PE as above, and
walks each arm by recursive descent stopping at the loop back edge `0x1404b2ec0` and the
epilogue `0x1404b2ee4`. Prints one row per command; the assertion is `mismatches: 0`.

**3. The `0x00D9` archive sweep.** Globs `previous-runs/world*.log` and
`research/fixtures/*world*.log`, keys on `(timestamp, opcode, body)`, walks each body with the
table above and prints the element-count histogram, the outbound-minus-inbound length
histogram, the command census and the count of bodies that did not close. Exits non-zero on an
empty result, because "the sweep found nothing" and "the sweep did not run" must not look the
same.

**4. The `0x02FF` sweep**, the same shape, with `net::mobmove::parse_mob_move`'s variable head
walk reimplemented (`u32, u16, u8, u8, u64, 2, n1 x u32, n2 x u16, gated 11 x u32, 1 + 5 x u32
+ 1`) to find the path head. 164 117 distinct events, 0 heads that would not walk, 0 element
blocks that overrun the body, 0 with `element_count <= 0`.
