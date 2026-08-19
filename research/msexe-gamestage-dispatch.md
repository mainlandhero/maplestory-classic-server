# The channel stage's packet switch

**Found 2026-08-19, statically.** This is the answer to the first item in `STATUS.md`'s
NEXT GOALS: the dispatcher that receives packets on a **channel** connection.

    virtualised dispatch  ->  FUN_1415d59b0  ->  FUN_142cbaa80(this, opcode, packet)
                              (64 bytes)         (12107 bytes, 273 cases, 0x70..0x39a)

`FUN_1415d59b0` is a 64-byte forwarder whose only callee is `FUN_142cbaa80`. It has **no
direct callers, no vtable entry, and nothing takes its address** - which is the signature
`tools/handler_root.py` documents for a function reached only from the Themida-virtualised
packet loop. That is how the client reaches it.

All 64 bytes, read by hand out of the image rather than decompiled:

```asm
1415d59b0  mov  [rsp+18], r8          ; packet
           mov  [rsp+10], edx         ; opcode
           mov  [rsp+08], rcx         ; "this" - saved, then never used
           sub  rsp, 38
           call 140caa750             ; -> cmp qword [143aa84a0], 0 / setne al / ret
           movzx eax, al
           test eax, eax
           jz   1415d59ee             ; nothing to dispatch to: drop the packet
           call 140caa510             ; -> mov rax, [143aa84a0] / ret
           mov  [rsp+20], rax
           mov  r8,  [rsp+50]         ; packet
           mov  edx, [rsp+48]         ; opcode
           mov  rcx, [rsp+20]         ; the object
           call 142cbaa80
           add  rsp, 38
           ret
```

which is:

```c
void FUN_1415d59b0(void *ignored, int opcode, CInPacket *packet) {
    if (DAT_143aa84a0 != NULL)
        FUN_142cbaa80(DAT_143aa84a0, opcode, packet);
}
```

**Two things follow, and both matter operationally.**

1. `FUN_142cbaa80` is a method on the global `DAT_143aa84a0` - the in-game world object.
   The same global is read all over the client (5459 RIP-relative references) and is
   **written in exactly two places**: `FUN_142ca5c50` sets it and `FUN_142ca88c0` clears
   it. Found with `tools/dataref.py`, which was written for this and which sees the
   read/write forms `tools/xref.py` is documented as missing.
2. **If `DAT_143aa84a0` is null, the packet is dropped in silence** - no dialog, no log,
   no error. A game-stage reply sent before the client has built that object would do
   nothing at all, and would look exactly like a reply the client did not understand.

   **It is not null by the time the channel connection exists.** The login stage's migrate
   handler `FUN_141b36f60` - the `0x0011` case, the packet we already send - does

   ```c
   local_80 = DAT_143aa84a0;          // no null check
   ...
   FUN_142cb9560(local_80, ...);      // and then nine setters on it, unconditionally
   FUN_142cb9590(local_80, ...);
   FUN_142cb95a0(local_80, ...);
   ...
   FUN_142caebe0(local_80);
   ```

   on the success path, in the same `0x142ca..0x142cb` subsystem that owns the dispatcher.
   The client would fault if that global were null there, so the world object is already
   live **before** it opens the channel socket, and the gate in `FUN_1415d59b0` will pass.

   This is an argument from an absent null check rather than from a positive construction
   trace, so it is strong but not proof - see "Not yet established".

The address arithmetic above was checked against an independent instrument: the third
call target computed by hand, `142cbaa80`, is the same edge the `E8 rel32` scanner found.

The two stages are disjoint, and that is what makes the identification safe:

| stage | dispatcher | inbound opcodes | code lives at |
|---|---|---|---|
| login / character select | `FUN_141b25f30` | `0x00..0x5f` | `0x141b2xxxx` |
| **game channel** | **`FUN_142cbaa80`** | **`0x70..0x39a`** | `0x142cxxxx..0x142exxxx` |

`FUN_142cbaa80` calls nothing in the login stage's address range - all 293 of its callees
sit in `0x142c..0x142e` - so these are two separate subsystems and not two views of one
switch. The opcode ranges do not overlap either, and together they cover `0x00..0x39a`
with one gap at `0x60..0x6f`.

## How it was found

Not by structure - by call-graph shape, with `tools/dispatchers.py`. A handler reads the
packet through one of five known primitives (`FUN_1406e8ae0` u8, `FUN_1406e8b80` u16,
`FUN_1406e8c20` u32, `FUN_1406e9050` string, `FUN_1406e9170` raw); a dispatcher mostly
does not read, it forwards. Ranking every function by "how many distinct packet-reading
functions do you call" put `FUN_142cbaa80` first with 170, and recovered the already-known
`FUN_141b25f30` at eighth, which is the control that says the method works.

**Two structural approaches failed first, and both failed quietly:**

* **RTTI.** The client ships 1764 type descriptors and `tools/rtti.py` reads them fine, but
  there is no `CStage`, no `CLogin`, no `CField` - the stage classes carry no RTTI at all.
* **Vtable slot alignment.** The login stage's `OnPacket` is slot 76 of vtable
  `0x1433fd540`. Aligning other vtables on a shared run of base-class stubs and reading the
  same slot produced 761 candidates, of which only 41 were opcode switches - because the
  shared "base class" was an illusion: MSVC folds identical tiny stubs (`/OPT:ICF`), so one
  `return;` address appears in 784 unrelated vtables. **Shared method pointers are not
  evidence of a shared base class in this binary.** The 41 switches it did turn up were
  real and are still useful, but the derivation was wrong and should not be repeated.

## There are exactly three entries like this, and none of them is the login stage

The 64-byte body above is a template. Searching the whole image for its 18-byte prologue
found **five** functions; three match the full body byte for byte, and each is guarded by
its own singleton:

| entry | guard / owner global | dispatcher | inbound opcodes |
|---|---|---|---|
| `FUN_1415d59b0` | `DAT_143aa84a0` | `FUN_142cbaa80` | **`0x70..0x39a`**, 273 cases |
| `FUN_1415d5a00` | `DAT_143ac97e0` | `FUN_142279c50` | `0x05c5..0x05d2`, 12 cases |
| `FUN_1415d5a50` | `DAT_143ace378` | `FUN_14177b7e0` | `0x05d5..0x05dc`, 8 cases |

In all three the gate and the getter resolve to the **same** global - `cmp qword [g],0` and
`mov rax,[g]` - so each entry is "if this subsystem exists, hand it the packet". The other
two prologue matches (`0x140c8fec5`, `0x140cad410`) have different bodies and are not
entries.

Both of the smaller two **are** case switches after all - the earlier note called them
monolithic because they call the decoders directly, and they do, but from *inside* their
case bodies rather than instead of having any. Every case is inlined, which is why a
single-call extraction saw nothing; `tools/switch_cases.py` reads them fine. Decompilation
in `research/msexe-dispatch-entries.c`.

So the inbound opcode space is now accounted for end to end:

| range | dispatcher | reached by |
|---|---|---|
| `0x0000..0x005f` | `FUN_141b25f30` | the current stage's vtable |
| `0x0051..0x006f` | `FUN_141b82b00` | chained from the login stage's `default` |
| `0x0070..0x039a` | `FUN_142cbaa80` | singleton entry `FUN_1415d59b0` |
| `0x01a0..0x01a3` | `FUN_142097ee0` | chained from the login stage's `default` - **`SetField`** |
| `0x05ac..0x05bf` | `FUN_141072ec0` | a stage vtable |
| `0x05c5..0x05d2` | `FUN_142279c50` | singleton entry `FUN_1415d5a00` |
| `0x05d5..0x05dc` | `FUN_14177b7e0` | singleton entry `FUN_1415d5a50` |

The overlap at `0x51..0x5f` is not one: the login switch matches its own cases first.

**The login stage is not in this table**, and that is the interesting part. `FUN_141b25f30`
is a virtual method, slot 76 of vtable `0x1433fd540`. So the virtualised packet loop does
two different things with each inbound packet: a **vtable call on the current stage**, and
a call to each of these three **singleton entries**.

The consequence is worth stating plainly, because it decides whether a reply can work:
**none of the three entries checks which stage the client is in.** `FUN_142cbaa80` is
gated only on the world object existing, and that object is already live by the time the
migrate command is handled. So opcode `0x0070` should reach `FUN_142d51930` whether or not
the client still thinks it is in the login stage. That is an inference from the routing,
not something a run has confirmed.

## What the client sends back

`research/msexe-gamestage-outbound.txt` - the 175 client -> server opcodes whose builders
live in the same `0x142c..0x142e` subsystem, `0x007A..0x0428`, filtered out of
`research/msexe-send-opcodes.txt` by builder address. Disjoint from the 33 the login
subsystem builds. That is the set a channel should expect once a character is in a map.

## The case table

`research/msexe-gamestage-cases.txt` - **273 cases**, `0x0070..0x039a`, in enum order,
produced by `tools/switch_cases.py`.

**The first version of this table had 181 rows and was wrong.** It was built by matching
cases of the form

```c
case 0x70:
  FUN_142d51930(param_1,param_3);
  break;
```

which is 179 of the 273. The other **94 have their handler inlined**, so a single-call
regex dropped them without a word, and whole contiguous runs - `0x121..0x126`,
`0x13a..0x140` - simply were not there. A gap in a table built that way says nothing about
the client. `tools/switch_cases.py` also tracks brace depth, because taking every `case`
label in the function merges in the nested switches inside case bodies and makes
`FUN_142cbaa80` appear to handle `0x0..0x3`.

The largest *forwarded* handlers, a rough proxy for how much a packet carries (the inlined
cases have no size of their own, so they are absent here):

| opcode | handler | size |
|---|---|---|
| `0x008c` | `FUN_142d634c0` | 188565 |
| `0x00a9` | `FUN_142ddfd10` | 23440 |
| `0x0116` | `FUN_142cf6d40` | 16260 |
| `0x0070` | `FUN_142d51930` | 11712 |
| `0x00a7` | `FUN_142defd40` | 9620 |
| `0x0145` | `FUN_142da7550` | 8819 |

**`0x0070` is not SetField.** It was the obvious suspect - first case, one of the largest
handlers - and it is wrong. `FUN_142d51930` is **InventoryOperation**: it never touches a
map, portal, spawn point or channel id, and its 13 modes match the v214 reference's
`InventoryOperation` enum one for one, in value order and with matching payloads. Decoded
to the assembly in `research/msexe-setfield.md`, which is worth reading anyway - it is a
complete, asm-verified layout of a packet we will need later, and it is now a **confirmed
anchor** for aligning the rest of the enum.

Being the first case in the switch means nothing about being the first packet on the wire.

## SetField is `0x01A0`, and it is a *stage* packet, not a world packet

Established 2026-08-19 by three independent lines that agree.

**1. The block boundary is exact.** `FUN_142d51930` is InventoryOperation, and in the v214
reference `InventoryOperation = 0x37` is `BEGIN_CHARACTERDATA` - the first opcode of its
block. In mscw it is `0x0070`, the first case of the game dispatcher. The dispatcher then
runs to `0x019f` and stops: **271 cases spanning exactly 304 values**, and v214's
CHARACTERDATA block (`0x37..0x166`) is **also exactly 304 values**. Same block, shifted by
`+0x39`. Two stray cases sit above it, `0x0275` and `0x039a`.

So mscw `0x01a0` = v214 `0x167` = `BEGIN_STAGE` = **`SetField`**.

**2. A different dispatcher takes over at exactly that boundary.** `0x01a0..0x01a3` is
handled by `FUN_142097ee0`, which is not reached through the singleton entries at all - it
is the **virtual `OnPacket` of a stage object**, slot `anchor+26` of the six-class vtable
family. Which is correct: `SetField` is addressed to the stage, and the stage is what
changes. `FUN_142097ee0` forwards `0x1a0` to **`FUN_142097f80`** (11726 bytes).

**3. The handler says so itself.** It reads a `u32`, a `u8` and a `u32` into world setters,
brackets them with `FUN_142cb9260(world)` read before and after, and when the two differ it
builds and displays the string

```
"Ch
a n	n
e
l"        -> "Channel", with control characters spliced in
```

That is a channel-change announcement, which is `SetField`'s job. The junk bytes are an
anti-string-search measure and are why a plain search for "Channel" never found it.

Note what this rehabilitates: **the six-vtable family was not a dead end after all.** The
mistake was the derivation (COMDAT folding, above), not the family. `FUN_142097ee0` is a
real stage `OnPacket`, and two of the six override it - `FUN_141b82b00` (`0x51..0x6f`) and
`FUN_141df5940` (`0x1001+`, `0x8002+`, `0xa002+`).

## No stage transition is needed first, and the `0x60..0x6f` gap was never a gap

The login stage's own `OnPacket` ends its `default` branch with a chain to its base class:

```c
switchD_141b25f9b_caseD_1:
    if (iVar4 - 0x1a0U < 4)        FUN_142097ee0(param_1, param_2, param_3);
    else if (iVar4 - 0x51U < 0x1f) FUN_141b82b00(param_1, param_2, param_3);
```

Two things fall out of four lines.

**`SetField` is handled while the client still believes it is in the login stage.** That is
the stage it is in while showing "Connecting...", and `0x1a0..0x1a3` is exactly the range it
forwards. So a `SetField` can be sent the moment the migration hello arrives, with no
transition to arrange first and nothing to get wrong about ordering.

**`0x60..0x6f` was never unclaimed.** `FUN_141b82b00` covers `0x51..0x6f` as an inherited
handler, not as a rival stage. Its apparent overlap with the login stage's own `0x00..0x5f`
is not an overlap at all: the login switch matches its own cases *first*, and only what it
does not match falls through to this chain. The effective ranges are disjoint.

## A fourth line, arrived independently - and a warning about the third

A second pass over the enum, working only from block structure against the reference source
and **not knowing that `FUN_142097f80` had been found**, derived the same answer: `SetField`
at `0x01A0`, one slot past the game dispatcher's last case, in a stage `OnPacket` it noted
the project had not located. It had. That is `FUN_142097ee0` -> `FUN_142097f80`.

It also predicted from the reference that `SetField` **opens with an 8-byte FILETIME** -
and the first thing `FUN_142097f80` does, before any null check, is
`FUN_1406e9170(pkt, &buf, 8)` followed by `FUN_1408f67d0(buf)`.

**But score the reference before leaning on it.** Run blind against the already-decoded
login range as a held-out control, the same alignment method scored **1 of 8**, and the one
it got was the block's first entry, which any alignment gets for free. Adding the game-range
control makes it **1 of 9**. The method reproduces a block's *interior* well once given an
anchor inside it, and determines the block's *offset* not at all. So:

* every candidate name from the reference is a candidate, never a finding;
* `0x01A0` is not believed because the arithmetic said so - it is believed because
  `FUN_142097f80` announces a channel change and opens with the predicted 8 raw bytes.

Two structural details from that pass, both of which this repo's own table agrees with, so
the two instruments corroborate rather than merely coexist:

* **`0x0071..0x007a` really is a hole** - ten opcodes with no case at all.
* **`0x0121..0x0126` are not cases of the outer switch.** They are dispatched from its tail
  under `if (DAT_143ad1850 != 0)`, six consecutive opcodes to one UI object.

And a caution that limits all of this: case `0x018b` carries the literal
`"BossFirstClearRecord Is Empty ( bossID[%d] worldID[%d]"`, and **neither reference version
has a matching name**. mscw's enum is not a clean subsequence of the reference, so any
argument resting on block sizes lining up exactly is weaker than it looks - including the
304-against-304 match above, which the same pass suggests is a cancellation of an insertion
and a deletion rather than a preserved block.

Full layout of the packet: `research/msexe-stage-setfield.md`.

## Not yet established

* A positive trace of **when `FUN_142ca5c50` runs**. The argument above says the world
  object is live by migrate time because the migrate handler dereferences it without a
  check; it does not say when it was built. Its callers are `FUN_142c43970` <- `FUN_142c42f30` <-
  `FUN_142ef49e4`, and that last one has no callers and no address taken either, so the
  chain runs off into the virtualised region and static reading stops there.
* Whether the login connection reaches `FUN_141b25f30` through this same entry or through
  the vtable it does sit in (slot 76 of `0x1433fd540`).
* What `FUN_14177b7e0` handles - the third entry's dispatcher, still unread.

