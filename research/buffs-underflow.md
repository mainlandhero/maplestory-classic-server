# `0x007D` is confirmed, and its body is longer than four instruments can account for

Addendum to `research/buffs.md`, which is not edited here. 2026-08-22.

The owner: *"Nimble Feet crashed the client."* First send of the packet `buffs.md` §7.1 specifies.

Tags: **[L]** read out of a file, **[D]** derived from something read, **[I]** inferred.

---

## The short version

* **`0x007D` IS TemporaryStatSet.** That was `[D]` in `buffs.md` §5.1, resting on a case
  table and two neighbouring anchors. It is now **`[L]` at runtime**: the client threw from
  inside `FUN_142d563d0` while handling our packet. **This is the good news and it is not
  small.**
* **It threw because the body ran out**, at the `u32` read `0x142d5690c`, four fields from
  the end. The reader's own listing shows the branch it took.
* **The layout says that should not have happened.** Four instruments agree the handler
  consumes at most **145** of the 152 bytes sent, leaving seven. Something invisible to all
  four consumes the difference.
* The tail is now **64 zero bytes** - slack, not a computed length, and labelled as such
  everywhere it appears. `!buff <skill> <level> <tail>` makes the number typeable so a
  surviving session can bisect it instead of spending a launch per attempt.

---

## 1. What the client did, to the instruction

`client-exit.log`: exit code **`0xE06D7363`** after 60.7 s. That is the Microsoft C++
exception code, not an access violation and not the heap family - an **unhandled `throw`**.
No dump was written, because the hook's vectored handler does not treat a C++ throw as a
fault. **[L]**

`client-patched\maplecw-hook.log`, and the count that matters is across two logs:

```text
world.log   00:56:36.993  <- 0x013C   skillId 1002 level 3
world.log   00:56:36.994  -> 0x007D   152 bytes

hook.log    20:56:36.995     90 opcode=0x007C elapsed_us=134.9 ret=1     <- returned
hook.log    20:56:36.996  ***** C++ THROW #17 ... *****
                stack: ... 0x1406e8cb1<-TEXT 0x142d56911<-TEXT ...
```

**There is no dispatch line for `0x007D`.** The hook writes those on handler *return*, so the
handler was entered and never came back - the same reading that identified the equip crash.
**[L]**

The two addresses on the throw stack name the site exactly:

* **`0x142d56911`** is the instruction after the `call` at **`0x142d5690c`**, which
  `tools/reads.py` lists as the `u32` read in `FUN_142d563d0`'s tail. A `call rel32` is five
  bytes. **[L]**
* **`0x1406e8cb1`** is inside the `u32` primitive `0x1406e8c20`, and its listing says what
  that address is:

```asm
1406e8c32  mov  edi, dword ptr [rcx+0x18]   ; length
1406e8c35  sub  edi, dword ptr [rcx+0x24]   ; minus position = bytes remaining
1406e8c79  cmp  edi, 4
1406e8c7c  jb   1406e8c91                   ; fewer than four -> the raise path
1406e8c91  mov  edx, 0x26
1406e8c9b  call 1401bb8b0
1406e8cac  call 142ef6d4c                   ; throws
1406e8cb1  int3                             ; the return address seen on the stack
```

**[L]**. So: **fewer than four bytes remained.** This is an underflow, measured, not inferred.

---

## 2. The seven bytes that should have been there

Four instruments, each with its own blind spot, each run rather than remembered:

| instrument | what it says |
|---|---|
| `tools/reads.py` depth 4 on `FUN_142d563d0` | the tail is `u16, 6x u8, one conditional u8, u32, u8` - and **nothing reads the packet before the mask** |
| the listing of `0x1406e9170` (raw) | copies exactly `r8d` bytes, **no length prefix**; the call site at `0x140a16630` passes `0x7c`, so the mask is **124** |
| the listing of bit 92's block at `0x140a17e3b` | 87 lines; the `u32` at `+0x34` and the `u16` at `+0x52` are the **two arms of one `if`** (`je 0x140a166bc` / `jmp 0x140a166d9` in the first block), so a stat is 10 bytes **or** 12, never both |
| an enumeration of all **476** bit tests against `0x1402bf6d0`, each resolved by scanning back to its `mov edx, imm` | bit 92 is tested **once** of 476, so one set bit decodes one block |

All **[L]**. And `0x1402bf6d0` itself confirms our mask arithmetic exactly:

```asm
1402bf6e1  mov  eax, edx / and eax, 0x1f / mov ecx, 0x1f / sub ecx, eax   ; 31 - (i & 31)
1402bf6ef  shr  rax, 5 / mov eax, [r8+rax*4] / shr eax, cl / and eax, 1
```

Word `i>>5`, bit `31-(i&31)`. Our mask sets word 2 to `0x00000008`, which is bit 92 and
nothing else. **[L]**

Adding it up, worst case: `124 + 12 + 2 + 6 + 1 = 145`, then the `u32` needs 4 → **149 of
152**. Three to spare, and with the `i16` arm seven. **The client says it had fewer than
four.**

### What that leaves

Consumed-before-the-`u32` must be in `149..=152`, so the decoder consumed **16..20** bytes of
stats where the evidence allows only 10 or 12. Twenty is `2 x 10`, which is the only clean
fit - but bit 92 is tested once and its block reads one value. So either a second block ran
for a bit we did not set, or something outside the enumerated reads takes bytes.

**Re-running any of those four is not a second opinion**, for the reason `CLAUDE.md` gives at
length. What would be a different question:

* a **WATCH at `0x1406e8c91`** - the raise path only, so it fires once and costs nothing -
  reporting `[rbx+0x18]` and `[rbx+0x24]`. That is the length and the position, and it turns
  "fewer than four" into two exact numbers. The hook's `peek=` reads `rcx + off` and the
  object is in `rbx` there, so this needs a small extension rather than a new mechanism.
* **bisecting the tail from a length that works**, which `!buff <skill> <level> <tail>` now
  makes a chat line instead of a launch.

---

## 3. What shipped, and why it is honest about being a guess

`TAIL_LEN` is **64**: 46 bytes more than the worst layout any of the evidence supports.

Every tail field is fixed-width and zero, and the reader's only length test is "fewer than N
remaining" - there is no check that the body was fully consumed - so a longer tail cannot
change what any field decodes. **[D]** from the primitive's listing.

The constant, the log line and the test all say it is slack rather than a size. The one
outcome that would refute the reasoning above is the client dying *differently* on a
198-byte body - a complaint about length rather than a silent throw - which would mean this
reader does check for leftovers and the number has to be exact after all.

`TAIL_KNOWN_TOO_SHORT = 18` is refused by `!buff`: spending a launch to re-learn something
already in a log is the failure this repo's rules exist to prevent.


---

# Part two: the buff worked, and then `0x007E` did the same thing

2026-08-22, later. The owner: *"The buff works, but after the buff expired, the client crashed
again."*

## What the working grant settled

`hook.log`: `18 opcode=0x007D elapsed_us=602.7 ret=1` - **a dispatch line, which means the
handler returned.** It had never done that before. On screen: the icon, the countdown and the
speed. **[L] + the screen.**

One packet reaching one screen settled five things that no static pass could:

| | was | now |
|---|---|---|
| `0x007D` is TemporaryStatSet | `[D]` from a case table | `[L]` |
| the mask is 124 bytes with big-endian bits inside each word | `[L]` from three code reads | drawn |
| CTS bit **92** is Speed | `[L]` from the client's own guard | **the character moved faster** |
| the value is `i16`, not `u32` | `[I]`, and unreadable statically - the deciding constant is in Themida-packed `.data` | **settled**: a `u32` parse reads the duration as 0 and the icon would have flashed and vanished |
| the duration is milliseconds | `[D]` from three readings | **counted down for 30 s** |

And one more, which is why the padding argument is now evidence-backed rather than hopeful:
**a 198-byte body was accepted without complaint**, so this client does not check that a
packet was fully consumed. **[D]**

## And then the reset, one handler over

```text
world.log  01:15:40.058  -> 0x007E   127 bytes, 30 s after the grant
hook.log   21:15:40.059  ***** C++ THROW #3 *****
                stack: ... 0x1406e8b71<-TEXT 0x142d57327<-TEXT ...
client-exit.log          EXIT code 0xE06D7363 after 57.9s
```

**No dispatch line for `0x007E`.** `[L]`

* `0x142d57327` is the instruction after the `call` at `0x142d57322`;
* `0x1406e8b71` is the raise path of the **u8** primitive `0x1406e8ae0`, whose own listing is
  `cmp edi, 1 / jb 0x1406e8b51` - the same shape as the `u32`'s, at the same `+0x91` offset.

`tools/reads.py` at depth 4 on `FUN_142d56f80`: **[L]**

```text
0x142d56fc3  u8
0x142d56fd1  u8
0x142d56fdf  u8
0x142d57040  raw            <- 124
0x142d571c3  u32 via helper, gated
0x142d57322  u8             <- threw
0x142d57360  u8
```

`research/buffs.md` §7.1 lists only the first four and calls the body 127 bytes. **Three
reads after the mask are missing from that description.**

**This time the arithmetic is exact and nothing is left over.** `3 + 124 = 127` consumed, the
gated `u32` did not fire - had it, the throw would have been at `0x142d571c3` - and then a
`u8` with zero bytes left. Minimum **129**, or **133** if that `u32` ever fires.

## What shipped

* `0x007E` is padded to the same `TAIL_LEN` (191 bytes total) and `!unbuff [tail]` bisects it.
* **The natural expiry no longer sends it at all.** The client stores its own expiry from
  `0x007D`'s duration field (`buffs.md` §5.3 reads the decoder adding it to the current tick),
  so at thirty seconds both sides drop the stat with nothing crossing the wire. The packet
  that killed the client twice is off the one code path every buff takes; it now fires only
  when a person asks for it, which is what dispel, death and logout will need.

If the icon *never* disappears on its own, the client does not self-expire after all - a
harmless and very informative outcome, and `!unbuff` becomes mandatory rather than a test.


---

# Part three: the client never removes a temporary stat, and `0x013F` is how it asks

2026-08-22, later still. The owner: *"the buff works, but after the expiry, the buff did not go
away. (It just kept flashing, but the temporary stats were still there) I also tried to
pre-emptively kill the buff by right clicking on the icon, it also did not dismiss the buff."*

Exit code this run was **`0x00000001`** - a hand-close. **No crash**, so removing `0x007E`
from the expiry did stop the deaths. It also stopped the buff ending.

## The retraction, and it was a written-down prediction

The expiry stopped sending `0x007E` on the reading that the client would drop the stat itself:
`buffs.md` §5.3 reads the decoder adding `0x007D`'s duration to the current tick and storing
it, so the client plainly *knows* when the buff ends.

It does know. **It flashes the icon and keeps the stat.** `tExpire` drives the animation and
nothing else, and at the thirty-second mark the client sent **nothing at all** - not a
request, not a report. `[L]`, from `world.log`: the grant is at `01:27:11.497`, and between
`01:27:41` and the hand-close there is no packet from the client but movement and env
reports.

The plan named this outcome in advance - *"icon NEVER goes away -> the client does not
self-expire. Harmless, and very informative"* - which is the only reason a wrong version was
worth one run. **Removal is the server's job on both paths.**

## `0x013F`, and how one capture pins its layout

Right-clicking the icon produced **fourteen** identical 133-byte bodies in three seconds, one
every ~180 ms. `[L]` That cadence is a retry loop rather than fourteen clicks, and it is the
clearest thing the client has said about waiting on us.

The body has exactly **three** non-zero bytes: `ea 03` at 0..1, and `0x08` at offset 17.

```text
ea030000 0000000000000000000000000008 0000...   (133 bytes)
^^^^^^^^ u32 skillId = 1002                     ^ offset 17
```

For each candidate mask start, two things must both come out right - the mask must be 124
bytes, and the set bit must decode by the client's own
`words[i>>5] >> (31 - (i&31))` to a stat that was actually granted:

| mask starts at | mask length | the `0x08` decodes to |
|---:|---:|---|
| 4 | 129 | bit 116 |
| 8 | 125 | bit 84 |
| **9** | **124** | **bit 92** - the Speed bit we granted |
| 10 | 123 | bit 36 |

**Only offset 9 satisfies both.** `[D]` So the body is `u32 skillId`, five bytes, then the
same 124-byte CTS mask `0x007D` and `0x007E` use.

This is **one sample**, and one sample is a reading rather than a settled layout - but a
coincidence would have to satisfy both constraints at once, and the bit it lands on is the
one the player was pointing at.

## What shipped

* `0x013F` is parsed and answered with `0x007E` for the bits **the server is actually
  holding**. A bit we never granted is refused with a chat line rather than honoured: telling
  the client to clear a stat that came from somewhere else is worse than doing nothing.
* The expiry sends `0x007E` again.
* `net::buff::bits_in_mask` is the exact inverse of `stat_mask`, with a round-trip test over
  all 992 bits - a decoder off by one would cancel the wrong stat and report success.
* Right-clicking is now the **fast** way to test `0x007E`: no thirty-second wait, and it
  retries by itself if the answer is wrong.
