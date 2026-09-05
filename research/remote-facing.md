# `move_action` — the third of `0x0224`'s four position fields

The owner, 2026-09-04: *"the direction they face when communicated to the client that joins the map
is incorrect"*.

`crates/net/src/userpool.rs`'s `RemoteAt::move_action` is body offset **430** of
`USER_ENTER_FIELD`. `crates/world/src/session/multiplayer.rs::remote_at` sends **`0`**, always.
This file is what that byte is, where the live value is, and what the change is.

**[L]** is read off the listing or off a capture, **[D]** derived from two or more [L] or
measured over the archive, **[I]** inferred.

---

## 0. The answer, before the working

```text
move_action = (action << 1) | facing        facing: 0 = right, 1 = left
```

* It is the **same quantity, in the same encoding**, that every `0x00D9` path element already
  carries in the **first `u8` of its four-byte common tail** — the read at `0x1404b2d82`.
  Both are `CUser`'s `nMoveAction`; the chain is closed statically in both directions (§2).
* `0` is **action 0 facing right**. It is a legal decode, so nothing errors — and this client
  has **never once emitted action 0** in 28 134 archived path elements. Every existing player
  therefore appears in an action nobody has ever performed, **facing right**, whichever way
  they were actually facing. That is the owner's sentence.
* The second `u8` of the tail (`0x1404b2dce`) is **not** the stance. It is `0` in
  **28 134 of 28 134** archived elements.

The fix is `move_action = the last path element's tail byte`, and the last element's value is
the one that persists — measured at 95.3 % against a positive control that scores 98.0 % (§4).

---

## 1. What the client does with body offset 430

`FUN_1429ce270`, the `0x0224` decoder (`research/user-enter-field.md` §2.2). `r14` is
`param_1`, the new `CUser`. **[L]**

```asm
1429ce852  call 1406e8b80      ; READ u16  -> cwde -> [rsp+0x68]   x    (offset 426)
1429ce85f  call 1406e8b80      ; READ u16  -> cwde -> [rsp+0x58]   y    (offset 428)
1429ce86c  call 1406e8ae0      ; READ u8
1429ce871  movzx eax,al
1429ce874  mov  dword [r14+0x6e4],eax                              <-- offset 430
1429ce885  call 1406e8b80      ; READ u16 -> FUN_142df6c50 -> [rbp-0x78]  foothold (431)
...
1429ce960  mov  r10,[rax+0x118]        ; the vtable slot
1429ce96b  mov  [rsp+0x38],rax         ; 8th arg: the foothold OBJECT
1429ce970  mov  eax,[r14+0x6e4]
1429ce977  mov  [rsp+0x30],eax         ; 7th arg: the move action
1429ce97b  mov  [rsp+0x28],r15d        ; 6th arg
1429ce980  mov  [rsp+0x20],r15d        ; 5th arg
1429ce985  mov  r9d,[rsp+0x58]         ; 4th arg: y
1429ce98a  mov  r8d,[rsp+0x68]         ; 3rd arg: x
1429ce98f  xor  edx,edx
1429ce994  call r10                    ; place(obj, 0, x, y, 0, 0, moveAction, foothold)
1429ce997  lea  rcx,[r14+8]
1429ce99e  call [rax+0x50]             ; -> the avatar
1429ce9a4  call 1409c6d00              ; return [avatar+0x358]
1429ce9a9  mov  dword [r14+0x6e4],eax  ; ...and REFRESH the field from it
```

Three things follow, and all three are **[L]**:

1. **The three fields the owner has been fixing are the three arguments of one call.** x, y,
   move action and foothold are read consecutively and handed to `vtable[0x118]` together —
   "put this avatar here, in this pose, on this foothold". A wrong value in any one of them
   is a wrong *first draw*, which is exactly the failure mode all three produced.
2. **`CUser+0x6e4` is `nMoveAction`.** `FUN_1409c6d00` is two instructions —
   `mov eax,[rcx+0x358] / ret` — and `FUN_1409c6d10` is its setter. After the placement the
   client throws away the wire byte and re-reads the avatar's own `+0x358`, which is why a
   wrong value here is a *first impression* and not a permanent state.
3. **The byte is zero-extended and passed through** — `movzx`, no range test, no fork. Unlike
   the mob's `0x02FF` builder there is no `cmp ecx,0x55 / setbe` packed-vs-raw gate here
   (`crates/world/src/mobattack.rs::decode_move_action`), so this field has **no `0xFF`
   escape hatch**: every value on this wire is packed.

### 1.1 Bit 0 is facing, and the client says so itself

A scan of `[reg+0x6e4]` over the user code range — `python tools/rangescan.py 0x6e4
0x142700000 0x142a00000`, **22 sites** — turns up the same three-instruction idiom in four
separate remote handlers: **[L]**

```asm
1429d3509  and dword [r14+0x6e4],0xfffffffe    ; clear bit 0
1429d3511  movzx eax,byte [rbp-0x78]
1429d3515  and eax,1                           ; a flag off the packet
1429d3518  or  dword [r14+0x6e4],eax            ; and put it back
```

(`FUN_1429d2ee0` = the remote attack, the arm reached after `cmp ebx,0x2a1`; and identically
`1429d42f0`/`1429d42f7` in `FUN_1429d4100`, `1429d4585`/`1429d458c` in `FUN_1429d4390`,
`1429d5e4b`/`1429d5e58` in `FUN_1429d5d90`.)

**Bit 0 is separately replaceable while the rest is preserved.** That is a flag packed into
a field, not part of an id — and a remote-attack packet's one-bit rider on a stance is
facing. This is the same `action*2 + facing` the mob's byte 35 already carries.

---

## 2. Where the live value is: the path element's FIRST tail `u8`

`crates/net/src/usermove.rs` documents the element's four-byte common tail as `u8, u16, u8`,
read at `0x1404b2d82`, `0x1404b2db7`, `0x1404b2dce`. Which `u8`?

### 2.1 The decoder stores them in different slots

`0x1404b2d7f`, with `rbx` = the element and `rsi` = the packet: **[L]**

```asm
1404b2d82  call 1406e8ae0   ; READ u8   -> obfuscated into element +0x58 / +0x59 / +0x5c
1404b2db7  call 1406e8b80   ; READ u16  -> FUN_1402f7010(&elem+0x28, v) -> +0x2c
1404b2dce  call 1406e8ae0   ; READ u8   -> obfuscated into element +0x60 / +0x61 / +0x64
```

and the encoder at `0x1404b2451` is the exact mirror — `+0x58` out as a `u8`, `+0x28` as a
`u16`, `+0x60` as a `u8`.

### 2.2 `+0x58` is fed from the avatar's `nMoveAction`

The element builder, `FUN_141d59dd0` at `0x141d5adbc`: **[L]**

```asm
141d5adbc  mov  rcx,[rbp+0x38]
141d5adc0  call 1409c6d00      ; return [avatar+0x358]   <-- nMoveAction
141d5adc5  mov  edx,eax
141d5adca  call 141d5c590
```

and `FUN_141d5c590` is a nine-line setter whose body is byte-identical to the decoder's
inlined store at `1404b2d87..1404b2db4`:

```asm
141d5c5a0  lea  rcx,[rip+0x1d65509]
141d5c5a7  call 1407386b0
141d5c5ac  mov  byte [rbx+0x58],al        ; the key
141d5c5c9  mov  byte [rbx+0x59],r9b       ; value ^ key
141d5c5d4  mov  dword [rbx+0x5c],r8d      ; the check word
```

**So the chain is closed both ways and it is the same field at both ends.**
`[avatar+0x358]` → element `+0x58` → the wire → `0x0224` body 430 → `CUser+0x6e4` →
refreshed from `[avatar+0x358]`. No arithmetic anywhere on the path: `movzx edi,dl` in and
`movzx eax,al` out.

`+0x60` has no such producer. It is written only by `FUN_141d55b40`/`FUN_141d56c40`, which
write `+0x58` in the same breath — element copy/assign, not a semantic source. **[L]**

---

## 3. The archive agrees, and it verifies its own offsets

7 056 distinct `0x00D9` events over 227 world logs, deduplicated on
`(timestamp, opcode, body)` — **not** on the file, because `research/fixtures/` holds copies
of `previous-runs/` and eleven pairs are the same run copied mid-write. Raw body lines:
12 449. All 7 056 walk closed with `element_len`'s table. 28 134 elements carry a tail.

*(This is a larger scope than `usermove.rs`'s own sweep, which is 5 221 over 172 files — it
excludes the live `world.log`/`world-ch1.log`, which this pass includes. The closure property
is the same and still 100 %.)*

### 3.1 The instrument's own control

Before believing any of the columns below: **the `u16` between the two candidate bytes sums
to exactly 510 ms per packet** at the median, the 90th percentile and the maximum. 510 ms is
the movement cadence `research/user-move.md` §1 measured a completely different way — from
the tick delta between packets. A field walk that had the tail at the wrong offset could not
reproduce a number nobody told it about. The middle field is the elapsed time, and the two
`u8` around it are where the listing says they are. **[D]**

### 3.2 The two candidates are not close

| | first `u8` (`1404b2d82`) | second `u8` (`1404b2dce`) |
|---|---|---|
| distinct values in 28 134 elements | **16** | **1** |
| what they are | `0x02..0x13`, in eight `±1` pairs | `0x00`, every single time |

Sixteen values forming eight consecutive pairs is `action << 1 | facing` and nothing else.
**[D]**

### 3.3 Bit 0 tracks the direction of travel

Elements that carry a position, against the sign of their step:

| | bit 0 = 0 | bit 0 = 1 |
|---|---|---|
| moving **right** (n = 12 964) | **95.3 %** | 4.7 % |
| moving **left** (n = 5 275) | 15.3 % | **84.7 %** |

and at packet level, on the value the fix would actually send (the last element, net
start→end displacement ≥ 8 px, n = 5 654): 94.6 % / 84.8 %. The second `u8` is 0 in every
row of the same table, so it carries no facing at all. **[D]**

The residual is the expected kind: a player can walk left and turn to face right on the last
step of the path, and a jump can carry you against your facing.

**`0` therefore means facing RIGHT** — so half of all remote players are drawn backwards by
chance, and anyone who walked in and stopped facing left is drawn backwards for certain.

### 3.4 What the actions are, behaviourally

Not from a name table — from what the player was doing. `action = byte >> 1`:

| action | n | what the archive says it is |
|---:|---:|---|
| **0** | **0** | **never emitted. Not one element in 28 134.** |
| 1 | 9 475 | **walking** — 72.4 % of horizontal-only steps; foothold slot non-zero |
| 2 | 2 072 | **the resting pose** — 66 % of the last elements of packets followed by >3 s of silence (n = 393); foothold non-zero |
| 3 | 12 350 | **airborne** — foothold slot **0 in 10 338 of 10 338**; 76 % of diagonal steps; mean element 83 ms |
| 4 | 3 614 | a second grounded pose — 25 % of those resting packets |
| 7, 8 | 453 | grounded with a *negative* foothold slot, ~350 ms per element, no velocity — ladder / rope **[I]** |
| 5, 9 | 170 | rare |

The action-3 row is worth its own sentence, because it is an independent instrument: the
fifth `u16` of a command-`0x00` element is the element's own foothold, and it is zero for
every action-3 element and non-zero for essentially every action-1/2/4 element. Two unrelated
readings — displacement geometry and a foothold field — pick out the same action. **[D]**

### 3.5 What action 0 draws as is NOT established, and here is why the obvious answer is refused

The client's avatar body-action name table can be recovered without Ghidra: `FUN_140007c70`
constructs the strings in order into consecutive slots, and
`python tools/dataref.py 0x143a47298` finds the pooled `.data` pointers. The first 36 are

```
walk1 walk2 stand1 stand2 alert swingO1 swingO2 swingO3 swingOF swingT1 swingT2 swingT3
swingTF swingP1 swingP2 swingPF stabO1 stabO2 stabOF stabT1 stabT2 stabTF shoot1 shoot2
shootF proneStab prone heal fly jump sit ladder rope dead blink alert2 ...
```

which is the canonical `BODY_ACTION` order, and it would make action 0 = `walk1`.

**It is not being used, because it fails its own check.** It predicts action 1 = `walk2`
(walking ✓), action 2 = `stand1` (idle ✓), action 4 = `alert` (a second idle ✓) — and
action 3 = `stand2`, which §3.4 measures as the **airborne** action, 10 338 of 10 338 with no
foothold. A standing pose cannot be the one the client uses in mid-air. Three of four fitting
is what a wrong table looks like when it is nearly right, and `CLAUDE.md` has a section about
exactly that.

So: **the wire action is not a direct index into that table**, or the table's base is not
where the construction order suggests. Either way the name of action 0 is unknown.

**This does not weaken anything above.** The claim the fix rests on is that the client emits
action 0 **never** and emits 1, 2, 3, 4, 5, 7, 8, 9 constantly — which is a count, not a name.

---

## 4. The last element is the one that persists

`usermove.rs` justifies `UserMove::x`/`y` with a self-consistency property: one report's last
position is the next report's start. The same test on the move action, over consecutive pairs
within one log file less than 2 s apart (n = 6 444):

| | agreement |
|---|---|
| **positive control** — last element's `(x,y)` == next report's start | **98.0 %** |
| last element's byte == next report's first element's byte | **95.3 %** |
| same action (`byte >> 1`) | 95.5 % |
| same facing (`byte & 1`) | **99.1 %** |

The control is quoted first on purpose: it says the pair selection is sound, so 95.3 % is a
property of the field and not of the pairing. Facing at 99.1 % is the strongest row and the
one that matters — a player's facing is a slow variable, which is precisely why sending a
constant `0` is so visible. **[D]**

Also measured, because the change depends on them:

* packets with **zero** elements: **0** of 7 056;
* packets whose last element is one of the three tail-less commands (`0x0c`, `0x3d`, `0x3f`):
  **0** of 7 056;
* packets with no tail-bearing element at all: **0** of 7 056;
* the byte the change would send: `0x02` (2 064), `0x06` (1 436), `0x03` (772), `0x08` (698),
  `0x04` (633), `0x09` (444), `0x07` (411), `0x05` (239), … **`0x00` and `0x01`: zero times.**

---

## 5. The change

### 5.1 `crates/net/src/usermove.rs`

Add one field to `UserMove`, filled by the existing element walk:

```rust
/// **The stance and facing the walk ended in**: `(action << 1) | facing`, `facing`
/// being `1` for left. The first `u8` of the last element's common tail
/// (`0x1404b2d82`), which the client wrote there from its own avatar's
/// `nMoveAction` — `141d5adc0 call 1409c6d00` (`= [avatar+0x358]`) feeding
/// `141d5c590`, the setter for element `+0x58`. **[L]**
///
/// This is the **same field** `net::userpool::RemoteAt::move_action` occupies at
/// `0x0224` body offset 430: `1429ce874` stores it to `CUser+0x6e4`, and
/// `1429ce9a9` refreshes that field from `[avatar+0x358]` a few instructions later.
/// `research/remote-facing.md`.
///
/// `None` when the walk read no element with a tail. **It is `Some` in all 7 056
/// distinct captured bodies** — every one has at least one, and none ends on one of
/// the three tail-less commands — but it is an `Option` rather than a `u8` because
/// the fallback would otherwise be `0`, and `0` is the *bug this field exists to
/// fix*: a legal decode, action 0, facing right, and an action this client has
/// never once emitted in 28 134 archived elements.
pub move_action: Option<u8>,
```

* initialise `move_action: None` in the struct literal in `parse_user_move`;
* inside the `'walk:` loop, after the `element_carries_position` block and **before**
  `p += element_len(command)`:

  ```rust
  // The common tail's first u8 — the stance and facing. Tail-less commands
  // (0x0c, 0x3d, 0x3f) jump straight to the loop back-edge at 0x1404b2ec0 and
  // carry none, so they leave the previous element's value standing, exactly as
  // they do for the position.
  if element_has_common_tail(command) {
      let tail = p + element_len(command) - ELEMENT_COMMON_TAIL_LEN;
      if let Some(&a) = path.get(tail) {
          m.move_action = Some(a);
      }
  }
  ```

* and a small public predicate beside `element_carries_position`, since `element_len`
  currently hides the `tail` boolean it already computes:

  ```rust
  /// Does this element carry the four-byte common tail at `0x1404b2d7f`?
  ///
  /// `0x0c` (`0x1404b2ca0`) and `0x3d`/`0x3f` (`0x1404b2e58`) `jmp 0x1404b2ec0` and
  /// skip it. Everything else, including every command past the 79-entry jump table,
  /// falls through to it. **[L]**
  pub fn element_has_common_tail(command: u8) -> bool {
      !matches!(command, 0x0c | 0x3d | 0x3f)
  }
  ```

Tests worth having, because the ones already in the file would all still pass with this
field wrong. **The three fixture bodies already in the file are enough** — all three have
three command-`0x00` elements and the tail byte lands at **body offset 83** in each:

| fixture | element tails (`action`) | `move_action` | reads as |
|---|---|---|---|
| `MAP1_FIRST` | `06 06 04` | **`0x04`** | action 2, facing right |
| `MAP1_SECOND` | `04 02 02` | **`0x02`** | action 1, facing right |
| `MELEE` | `04 08 08` | **`0x08`** | action 4, facing right |

* assert those three against `body[83]` directly, not against the parser's own arithmetic;
* the three tails' `u16` durations are `210+30+270`, `390+64+56` and `60+90+360` — **510 ms
  each**, which is the §3.1 control in miniature and pins the tail offset in a unit test;
* **`MAP1_FIRST`'s last is `0x04` and `MAP1_SECOND`'s first is `0x04`.** The two fixtures are
  consecutive packets on one connection, and `one_report_ends_where_the_next_one_starts`
  already pins that the position pairs across them. The move action pairs across them too,
  which is §4's property on the two bodies the file already holds — the best available test
  that "the last element is the one that persists" without touching the archive;
* `element_has_common_tail` agrees with `element_len` for all 256 commands:
  `element_len(c) == 1 + payload + if tail {4}` is the invariant, so assert
  `element_len(c) >= 5 || !element_has_common_tail(c)` and pin the three exceptions;
* the truncated-body test asserts `move_action` is the last element it *did* read, not `None`;
* extend the archive sweep with `assert!(m.move_action.is_some())` and
  `assert_ne!(m.move_action, Some(0))` — 7 056 of 7 056 today, and the second assertion is
  the one that fails loudly if the offset ever drifts back onto the always-zero byte.

### 5.2 `crates/world/src/session/`

* `mod.rs` — a field beside `last_position`:
  `last_move_action: Option<u8>`, `None` in the constructor;
* `mod.rs`, the `CLIENT_USER_MOVE` arm, beside `self.note_own_position(m.x, m.y);`:
  ```rust
  // Only ever overwritten by a packet that actually carried one. `0x00DF`'s
  // AttackRequest has no action or facing field, so the attack path leaves this
  // alone rather than clearing it.
  if let Some(a) = m.move_action {
      self.last_move_action = Some(a);
  }
  ```
* `field.rs`, next to `self.last_position = None;` on a warp:
  `self.last_move_action = None;` — a stance is a fact about the map you were on, for the
  same reason the position is;
* `multiplayer.rs::remote_at`:
  ```rust
  let move_action = self.last_move_action.unwrap_or(net::userpool::MOVE_ACTION_STANDING);
  net::userpool::RemoteAt { x, y, move_action, foothold }
  ```

### 5.3 The fallback, and it must not be `0`

```rust
/// **What to send when nobody has moved yet**: action 2, facing right.
///
/// `0` is what this field used to be and it is the wrong kind of wrong — it decodes
/// cleanly into an action this client has **never emitted**, in 28 134 archived path
/// elements. `4` is action 2, which is what the client itself reports as its resting
/// pose: 261 of the 393 archived packets that are followed by more than three seconds
/// of silence end on it. **[D]**, `research/remote-facing.md` §3.4.
///
/// The facing half is still a coin flip for a player who has not moved, and it heals on
/// their first `0x0293`. The stance half is not a guess.
pub const MOVE_ACTION_STANDING: u8 = 4;
```

`RemoteAt::default()` should use it too, or `Default` quietly reintroduces `0`.

### 5.4 What this does not do

* **`0x00DF` gives us no facing.** `net::combat::AttackRequest` exposes `x`, `y`, `tick`,
  `attack_type` and the targets and nothing else; its own doc block says **30 header fields
  are unnamed**, so a facing byte may well be sitting in that packet unread. Nobody has
  looked. Until somebody does, a player who arrives, swings, and stands still is announced
  with `MOVE_ACTION_STANDING` rather than with the direction they swung — and that is the
  narrow case where this fix still shows a coin flip.
* **Nothing here is authenticated.** The move action is taken verbatim from the client's own
  packet, like the path already is.
* **`Presence::spawn` still has to be rebuilt for this to be seen.** `Bus::refresh_spawn` is
  wired and fires from `note_own_position`, which fires on the same packet — so the stance
  reaches the bus with the position it belongs to. But a change to `last_move_action` alone,
  on a packet where the position did not change, returns early and does **not** refresh.
  Since a stance change without a position change is common (turning on the spot, landing),
  either move the early-return test to cover both, or accept that the announced stance can be
  one packet stale. **This is a real gap in the change and it is not measured.**

---

## 6. Blind spots

* **The second tail `u8` (element `+0x60`) is unidentified.** All that is established is that
  it is not the stance: `0` in 28 134 of 28 134 elements, and no producer that is not an
  element copy. A different client state — a different job, a mount, a ladder — could make it
  non-zero and it would still be unnamed.
* **The action ids are characterised, not named** (§3.5). The candidate table was found and
  refused because it contradicts a 10 338-of-10 338 measurement.
* **Nothing here has been on a screen.** Every claim is either a listing read or an archive
  measurement. What a joining client actually *draws* for a given byte has not been observed,
  and the one thing that has — `0` looks wrong — is the owner's sentence, not a controlled test.
* **A remote `CUser` built from a packet is not the local one.** The seat-index crash
  (`research/user-enter-field.md` §2.4) proved that the hard way. Every value this change can
  send is one the client just produced for *its own* user; whether the same value is safe on
  a packet-built remote is untested, except that `0` — a value the client never produces — has
  been sent for days without a crash.
* **`tools/listing.py` cannot read `FUN_1429d2ee0`.** Its linear sweep desynchronises at
  `0x1429d31e7` and stops, 800 bytes short of the `+0x6e4` writes, and it does not say so —
  `grep 6e4` over its output returns nothing, which reads exactly like "not there".
  `tools/rangescan.py` resyncs a byte at a time and found them. Positive control run first:
  `python tools/reads.py 0x140304100 2` printed the documented reads.
