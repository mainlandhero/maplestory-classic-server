# `0x03D2` level 0: it RELEASES. It does not despawn a mob that is in the field

**Answer: the owner is right.** For a live, ordinary mob that entered the field through `0x03C6`,
a `0x03D2` with `level = 0` stops the client simulating that mob and **returns before the
erase**. The mob stays in all three pool containers, stays drawn, and stays updatable by
`0x03D9`.

The claim being retracted is `crates/net/src/mobmove.rs:122`:

> *"Controller level `0`. **This DESPAWNS the mob - it does not merely release control.**"*

and its three echoes: `mobmove.rs:212`, `mobmove.rs:306`, `research/mob-share.md` §2/§4/§8,
`research/mob-hit-reaction.md:368`, and `research/mob-behaviour.md` §3 / §7.4 / §13.

Markers: **[L]** read out of the image, **[D]** derived from two or more [L], **[I]** inferred.

**No Ghidra** — the project lock is held elsewhere. Everything below is `tools/listing.py`,
`tools/reads.py`, `tools/callers.py`, `tools/rangescan.py`, `tools/pdata_lookup.py` and
`tools/dump_va.py` over `client-patched/MapleStory.exe`.

---

## 0. Instruments, and the controls that fired

| instrument | control | result |
|---|---|---|
| `python tools/reads.py 0x140304100 2` | the documented equipped-item decoder | **PASSED** - 25 sites, a mix of `(direct)` and `-> READS via helper` (`0x1403035a0`, `0x140303b40`) |
| `tools/dump_va.py` on the mob vtables | slot 7 must be `0x141c4ff80` in **all eight** vtables, which `research/mob-spawn.md` §5 derived independently through the eight constructors | **PASSED** - 8/8 |
| a raw disassembler for the leaves with no `.pdata` | must agree with `tools/listing.py` byte-for-byte over `0x141c54200..0x141c54231` | **PASSED** |
| `tools/callers.py` | its own docstring control | used as-is; the one negative it returned (`FUN_141c56aa3`, 0 callers) is **not** relied on below |
| the archived-log census (§6) | must find level-1 `0x03D2` bodies | **PASSED after a failure** - the first version found **0 events** and said so, because the `body` lines carry a timestamp prefix. Fixed, it finds 2 664 |

`tools/listing.py` refuses `0x141c54390`, `0x141c543c0` and `0x1409c5080` with *"no .pdata
entry"*. That is the tool working: `tools/pdata_lookup.py` confirms all three **fall in no
RUNTIME_FUNCTION at all** - they are tiny leaves with no unwind data, not mis-addressed. They
were disassembled raw, with the control above.

---

## 1. The answer, with a read address per step

`FUN_141d30e80`, the mob-pool dispatcher, `0x141d30e80..0x141d311d0` (848 bytes). The zero
branch begins at `141d30f1c`. **[L]**

| # | address | instruction | what it decides |
|---|---|---|---|
| | `141d30ee3` | `call 0x1406e8ae0` | `u8 level` -> `EBP` |
| | `141d30eee` | `call 0x1406e8c20` | `u32 objectId` -> `EBX` |
| | `141d30ef7` | `je 0x141d30f1c` | `level == 0` -> the branch below |
| | `141d30f23`..`141d30f5b` | hash walk of `pool+0x68` | find the mob; `rbx = [[bucket+0x18]+8]` = the CMob |
| 1 | `141d30f63` `141d30f69` | `mov rax,[rbx]` / `call [rax+0x48]` | **slot 9** = `FUN_141c54390` |
| | `141d30f6e` | `je 0x141d3116a` | slot 9 == 0 -> **return, nothing happens at all** |
| 2 | `141d30f77` `141d30f7c` | `xor edx,edx` / `call [rax+0x40]` | **slot 8** = `FUN_141c54200(mob, 0)` - **THE RELEASE** |
| 3 | `141d30f82` | `call 0x141c543c0` | the in-field flag |
| | `141d30f89` | `jne 0x141d3116a` | **non-zero -> RETURN.** `141d3116a` is the epilogue |
| 4 | `141d30f8f`..`141d30fd0` | `FUN_140f08f00(pool+0x98)`, `FUN_141d51320(pool+0x38,node)`, `FUN_141d51670(pool+0x68,&id)`, `FUN_141d51700(pool+0xa8,&id)` | the erase - **unreachable for an in-field mob** |

**Step 3 is the whole question, and it always bails for a live mob.** §3 proves it.

---

## 2. `[vtable+0x40]` is slot 8 `FUN_141c54200`, and `(mob, 0)` is the exact mirror of the grant

### 2.1 The vtable, read from the image

`rbx` is the CMob and `[rbx]` is therefore its **primary** vtable, so `+0x40` is slot 8 and
`+0x48` is slot 9 (`docs`: slot N at offset `N*8`; `research/mob-spawn.md` §5 fixes slot 7 at
`+0x38`). All eight mob classes: **[L]**

| vtable | slot 7 (control) | slot 8 `+0x40` | slot 9 `+0x48` |
|---|---|---|---|
| `0x1434077e8` (base) | `0x141c4ff80` | `0x141c54200` | `0x141c54390` |
| `0x14341f548` | `0x141c4ff80` | `0x141c54200` | `0x141c54390` |
| `0x14341e510` | `0x141c4ff80` | `0x141c54200` | `0x141c54390` |
| `0x14341e758` | `0x141c4ff80` | `0x141c54200` | `0x141c54390` |
| `0x1433765c0` | `0x141c4ff80` | `0x141c54200` | `0x141c54390` |
| `0x14341ede0` | `0x141c4ff80` | `0x141c54200` | `0x141c54390` |
| `0x1433767f0` | `0x141c4ff80` | `0x141c54200` | `0x141c54390` |
| `0x14341eff0` | `0x141c4ff80` | `0x141c54200` | `0x141c54390` |

Slot 7 agreeing with `mob-spawn.md` §5 in all eight is the positive control: that table was
built through the constructors, this one by reading `.rdata`. **No mob class overrides slot 8
or slot 9.** [L]

> `tools/rtti.py --vtable` still returns 0 locators for every class on this binary
> (`mob-spawn.md` §5). It was not used, and its silence was not read as evidence.

### 2.2 `FUN_141c54200` is 382 bytes, not 49

`tools/pdata_lookup.py` gives its first `.pdata` record as **49 bytes** (`0x141c54200..0x141c54231`).
`tools/listing.py` merges **4 contiguous entries** and gives `0x141c54200..0x141c5437e`.
`mob-behaviour.md` §9 already warns about this. The `edx == 0` arm at `0x141c542b5` is
**its own 190-byte `.pdata` entry** - which is why nobody had read it.

### 2.3 The two arms, side by side — this is the finding

```asm
141c5420a  mov  rdi,[rcx+0x2c0]   ; second animation interface; null -> return
141c5421d  mov  rax,[rcx+0x2b8]   ; first;                      null -> return
141c5423d  test edx,edx
141c5423f  je   0x141c542b5       ; *** arg2 == 0 -> the release ***
```

| | **grant**, `edx = 1` (from `141d34ca9`) | **release**, `edx = 0` (from `141d30f77`) |
|---|---|---|
| running test | `141c54241 call 0x1409c5080` | `141c542b5 call 0x1409c5080` |
| polarity | `141c54248 jne` -> **already running, do nothing** | `141c542bc je` -> **not running, do nothing** |
| state `mob+0x2e4` | `141c5424e/54` -> `FUN_141c4ff30(mob, 3)` | `141c542da/df/e1` -> `-2` if state was 3, `-3` if 4, else `-1` |
| pump | `141c54261 FUN_141c55750(mob, 1)` | `141c54310 FUN_141c55750(mob, 0)` (else-arm only) |
| animation call | via `FUN_141c55750(mob,1)` at `141c558b9`: `[anim+0x118]` with **four de-obfuscated live coordinates** (`141c55856`, `141c55871`, `141c55881`, `141c55890`) and `edx = 1` | `141c54349`: `[anim+0x118]` with **`rdx=0, r8=0, r9=0` and three zeroed stack dwords** (`141c5432b`..`141c54346`) |
| touches the pool? | no | **no** |

Same slot, same object, opposite polarity, opposite arguments, and neither arm frees, unlinks
or destroys anything. **`[vtable+0x40](mob, 0)` is "stop simulating this mob".** **[D]**

The `[anim+0x118]` call on the release arm is itself evidence: you do not call a method on the
animation object of something you have just deleted.

---

## 3. `FUN_141c543c0` is the in-field flag, and it is `1` for every `0x03C6` mob

### 3.1 It is a getter, not a predicate

14 bytes, no `.pdata` entry: **[L]**

```asm
141c543c0  mov  edx,[rcx+0x2e0]     ; the checksum word
141c543c6  add  rcx,0x2d8           ; the obfuscated triple's base
141c543cd  jmp  0x141d11770         ; tail-call the de-obfuscator
```

`FUN_141d11770(base, checksum)` returns `rol([base+4],5) ^ [base]` (`141d1177f`..`141d11785`,
`141d117f4 mov eax,ebx`), and raises an integrity report when the recomputed checksum
disagrees. The matching encoder in `FUN_141c543e0` stores
`[+0x2d8] = cookie`, `[+0x2dc] = ror(cookie ^ value, 5)`, `[+0x2e0] = ror(cookie ^ 0xBAADF00D,5) + [+0x2dc]`.
The round trip is exact. **[L]**

> **A naming correction, the same class as `mob+0x2f0`/`+0x2f4` in `mob-behaviour.md` §10.2.**
> The *value* lives at `mob+0x2dc`; `mob+0x2d8` is the cookie. Calling the field
> "`mob+0x2d8`" is shorthand for the triple `mob+0x2d8`/`+0x2dc`/`+0x2e0`.

### 3.2 Every writer of that triple, enumerated two independent ways

`tools/rangescan.py 0x2d8 / 0x2dc / 0x2e0 0x141c40000 0x141d60000` (24 / 5 / 59 sites; a
displacement is a class fact, so most belong to other classes), intersected with
`tools/callers.py` on both setters. The writers that write the triple **coherently** are: **[L]**

| writer | called from | value |
|---|---|---|
| `FUN_141c4cee0` inline, `141c4d205`..`141c4d220` | the mob base constructor | **0** (`141c4d20d ror ecx,5` with no XOR = `ror(cookie ^ 0, 5)`; checksum consistent, so no integrity report) |
| `FUN_141d23730` | `141c4e6ce`, also the constructor, `xor edx,edx` | **0** |
| `FUN_141c543e0` | `141d3372c` in **`0x03C6` MobEnterField**, existing-mob branch (`141d33725 lea edx,[r15+1]`, `r15 = 0`) | **1** |
| `FUN_141c543e0` | `141d338f2` in **`0x03C6` MobEnterField**, new-mob branch (`141d338ea mov edx,1`; `rdi` is the factory's return at `141d33792`/`141d33797`) | **1** |
| `FUN_141c543e0` | `141d33dfc` in **`0x03D1`** (`FUN_141d33c70`), leave-type 0 only | **0** |

`FUN_141c543e0` has **exactly 3 call sites in 2 functions**; `FUN_141d23730` has **exactly 1**,
in the constructor. That is the complete set. **Nothing else in the image can change this
field.** [L]

So the flag is a plain **"this mob is in the field"** bit:

```
constructed ............ 0
0x03C6 MobEnterField ... 1      <- every mob this server spawns
0x03D1 leave, type 0 ... 0      <- only while the client tears the mob down
```

### 3.3 Therefore

`crates/world` sends `0x03C6` for every mob it spawns (`combat.rs:730`, `field.rs:146`,
`mobshare.rs:598/1073/1092`), and `mob_change_controller_spawning` - the 137-byte `0x03D2`
spawn - **has no call site anywhere in `crates/`**. Every mob on every client therefore has
this flag at 1, `FUN_141c543c0` returns 1, and `141d30f89 jne` returns out of the dispatcher.

**The erase at `141d30f8f` is unreachable for every mob this server has ever created.** **[D]**

### 3.4 What the erase is actually for

`0x03D1` (`FUN_141d33c70`, 1532 bytes) is the real despawn: it clears the flag at `141d33dfc`,
and then at `141d33e58` calls **slot 9** and, if the animation is still running,
`141d33e5d jne 0x141d34249` **skips the pool removal entirely** - leaving a mob in the pool
with the flag at 0, still playing out its death or fade. The `0x03D2` zero branch's step 4 is
the cleanup for exactly that state: *"release control; and if this mob has already left the
field, finish taking it out of the pool."* **[D]** for the intent, **[L]** for both branches.

A conditional erase, guarded by a flag the server controls. Not a despawn.

---

## 4. `[vtable+0x48]` is slot 9 `FUN_141c54390` — "am I simulating this mob?"

34 bytes, no `.pdata` entry: **[L]**

```asm
141c54390  mov   rax,[rcx+0x2c0]    ; the second animation interface
141c5439c  lea   rcx,[rax-0x20]
141c543a0  cmove rcx,rdx            ; null -> 0
141c543a7  jne   0x141c543ac
141c543a9  xor   eax,eax / ret      ; mob+0x2c0 == null -> 0
141c543ac  jmp   0x1409c5080        ; else the animation-running predicate
```

`FUN_1409c5080` is a 12-byte thunk, `add rcx,0x108 / jmp 0x1409d4840` - the identical
predicate slot 8 tests on **both** arms. So slot 9 returns 0 when the mob has no animation
object (`mob+0x2c0` null - a mob that never ran `encodeInit`) or when its animation is not
running. **[L]**

**The polarity is confirmed by a completely independent handler.** `0x03E4 MobCtrlAck`,
`FUN_141c82060`: **[L]**

```asm
141c82074  mov  rax,[rcx]
141c8207d  call [rax+0x48]      ; slot 9
141c82082  jne  0x141c82092     ; ALREADY running -> skip
141c82087  mov  edx,1
141c8208f  call [rax+0x40]      ; NOT running -> slot 8 with 1 -> START it
```

| packet | slot 9 | then |
|---|---|---|
| `0x03E4` ack | `== 0` (not simulating) | slot 8 `(mob, 1)` - **start** |
| `0x03D2` level 0 | `!= 0` (simulating) | slot 8 `(mob, 0)` - **stop** |
| `0x03D2` level 0 | `== 0` | **return; nothing at all happens** |

Exactly complementary. This is a two-state simulate/do-not-simulate bit, and `0x03D2`
level 0 is the "off" edge. **[D]**

---

## 5. So does it despawn? — the one case where it DOES

**No, for a mob that entered via `0x03C6`.** **[D]**

**Yes, for a mob the client only ever learned about from a 137-byte `0x03D2`.**
`FUN_141d34a70`'s new-mob branch builds the mob (`FUN_140495990` -> `FUN_141d3a540`) and
inserts it into the pool, but it **never calls `FUN_141c543e0`** - it is not one of that
function's three call sites. Such a mob keeps the constructor's **0**, so step 3 falls through
and step 4 erases it. **[D]**

This server never takes that path (§3.3), but it is a live trap:
**`mob_change_controller_spawning` and `mob_release_controller` must never be used on the same
mob.**

That is very likely where the original claim came from - it is true, for the one spawn path
nobody uses.

---

## 6. The claim was never tested on screen, and that is checkable for free

`tools/`-style census over `previous-runs/` and `research/fixtures/`, **deduplicated on
`(timestamp, direction, opcode, body)` rather than on files** (`CLAUDE.md`: deduplicate the
events, not the files):

```text
files scanned            562
distinct by content      505
distinct EVENTS      240 788

0x03D2 server -> client, by level byte:
    level 0x01      2 664      <- the positive control
    level 0x00          0      <- never sent, not once
```

The one stray `0xdf` is a pairing artefact from a `0x02FF` line that carries its body inline;
an artefact can only add spurious counts, never hide a real `00`. Independently and more
strongly: **`mob_release_controller` has zero call sites in `crates/`** (only a comment at
`mobshare.rs:72`), so the server is structurally incapable of having sent one.

**The "DESPAWNS" claim was a pure static reading, marked [L], that has never once been put in
front of the client.** It has since been quoted in `mobmove.rs` three times, in a
`debug_assert!`, in a unit test named `release_is_five_bytes_and_is_a_despawn`, in
`mob-share.md` five times and in `mob-hit-reaction.md` - and the design decision *"never
rotate control"* (`mob-share.md` §8 item 1) rests entirely on it.

**What would have shown it earlier:** the doc block at `mobmove.rs:306` *already lists both
bail-outs* - *"calls `[vtable+0x48]` (returns without doing anything if it is 0),
`[vtable+0x40](mob, 0)` and `FUN_141c543c0` (returns if non-zero), then erases"* - and then
calls the whole thing a despawn anyway. The comment described a conditional and the constant
asserted an unconditional. `CLAUDE.md`: *a comment describing a guarantee is not the
guarantee*, and *a conditional erase summarised as an unconditional one*.

---

## 7. Moving a mob from client A to client B

### 7.1 The sequence

```
1.  ->A   0x03D2  level 0, objectId            (5 bytes, mob_release_controller)
2.  ->B   0x03D2  level 1, objectId, ...       (87 bytes, mob_change_controller)
3.  ->A   0x03D9  for every 0x02FF B sends from now on
```

**Order matters, and it is the answer to the 2026-09-04 teleporting.** **[D]**

* Release first: between packets 1 and 2 **nobody** simulates. One `0x02FF` source at all times.
* Grant first: both A and B pass their slot-19 gates until packet 1 lands, both roll independent
  random wanders (`mob-behaviour.md` §5.1), both send `0x02FF`, and the server relays two
  contradictory paths. That is precisely "mobs teleport (two simulations, no revoke)".

### 7.2 Three hazards, all read out of the image

**(a) `0x03E4` un-does a release.** `141c8207d`..`141c8208f`: an ack on a mob whose slot 9 is 0
calls slot 8 with `1` and **restarts the simulation**. If a `0x02FF` from A is in flight when
the release goes out, acking it puts A straight back to simulating. **The server must stop
acking A for that mob at the moment it sends the release, and it must not ack a `0x02FF`
whose sender is not the current controller.** **[L]**

**(b) A must keep receiving `0x03D9`, or the mob freezes on A's screen.** The release leaves the
mob in `pool+0x38`, `pool+0x68` and `pool+0xa8`, so A still draws it - but A has stopped
simulating it, so nothing moves it. `0x03D9` (`FUN_141c813b0`) is what moves a mob for a
non-controller, and it has **no control gate at all**: it never reads `mob+0x960` and never
calls slot 8 or slot 9 on the main path. It writes position, animation and clears `mob+0xcd0`
(`141c81784`). **[L]**

  A useful side effect: at `141c8196b` it calls `FUN_141c56870(mob, move_action)`, the setter
  for `mob+0x3e8`. Slot 19 refuses to roll a wander unless `deobf(mob+0x3e8) <= -1`
  (`141c8d313 cmp eax,-1 / 141c8d316 jg`), so a steady `0x03D9` stream independently suppresses
  A re-simulating. **[L]**

  And `mob-behaviour.md` §12.1 still applies: **never send `0x03D9` back to the client that
  sent the `0x02FF`.**

**(c) `mob+0x960` goes stale on A.** The zero branch never writes it - only `141d34ca0` in
`FUN_141d34a70` does - so after a release A still believes the level is 1. Harmless: §4.2 of
`mob-behaviour.md` establishes there is **no `mob+0x960 != 0` test anywhere in the image**, and
its only behavioural consumer is `FUN_141cc1e40`'s `> 1`. It would surface only as a stale echo
in a `0x02FF` tail byte, and A should not be sending those any more. **[D]**

### 7.3 Does A still DRAW the mob?

**Yes.** [D] The mob is never removed from any pool container (§1 step 4 is not reached), the
release calls a method *on* its animation object rather than tearing it down, and `0x03D9`
updates it with no control gate. A "released but invisible" mob is not a state this code path
can produce - the only thing that removes a mob from A's field is `0x03D1`.

### 7.4 The one link that is [I], and how to measure it for free

**[I]:** that `[anim+0x118]` with all-zero arguments actually drives `FUN_1409c5080` to 0, so
that A's slot-19 gate at `141c8d2c3` closes and A stops emitting `0x02FF`. I read the call and
its arguments; I did not follow it into the animation object.

The symptom if it is wrong is loud and costs **no extra client run**: after the release, A keeps
sending `0x02FF` for that object id. `world.log` and `world-ch1.log` already record every
inbound packet, so:

> **Measurement:** after a handover, count inbound `0x02FF` for that object id per connection.
> Exactly one connection should be sending them, and it should be B.
> Two senders ⇒ the release did not stop A's simulation and §7.4's [I] is false.
> Zero senders ⇒ B never started; check that B had the mob in its pool (87-byte form needs
> `FUN_141d2efc0` to find it) before the grant.

Two further things that this settles cheaply and that no static pass can:

* the mob is **still on A's screen** after the release (the [D] in §7.3), and
* it **moves on A's screen** once `0x03D9` starts arriving.

---

## 8. Corrections owed

| file | line / section | says | should say |
|---|---|---|---|
| `crates/net/src/mobmove.rs` | 122-127 | "This DESPAWNS the mob" | releases; the erase is guarded by an in-field flag that `0x03C6` sets to 1 |
| `crates/net/src/mobmove.rs` | 212, 221-224, 294-297 | `debug_assert!(level != CONTROL_RELEASE)` | the assert should go; level 0 is a legitimate value for `mob_change_controller`'s 5-byte form. Keep it on `mob_change_controller_spawning` (§5 - that path really does erase) |
| `crates/net/src/mobmove.rs` | 303-318 | `mob_release_controller` doc | correct the body; the function itself is right |
| `crates/net/src/mobmove.rs` | 727-734 | test `release_is_five_bytes_and_is_a_despawn` | rename; the length assertion is correct and worth keeping |
| `research/mob-share.md` | §2 line 27, §2 line 78-98, §4 line 172, §8 line 416 | "the client's only revoke is a despawn", "never rotate" | rotation is available; §7 above is the sequence |
| `research/mob-hit-reaction.md` | 368-369 | same claim | same correction |
| `research/mob-behaviour.md` | §3, §7 item 4, §13 | same claim | **appended as §3.3 in that file** |

I have **not** edited anything under `crates/` (per the brief). The `mob-behaviour.md`
correction is appended, not rewritten.
