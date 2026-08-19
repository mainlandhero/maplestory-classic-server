# The channel stage's packet switch

**Found 2026-08-19, statically.** This is the answer to the first item in `STATUS.md`'s
NEXT GOALS: the dispatcher that receives packets on a **channel** connection.

    virtualised dispatch  ->  FUN_1415d59b0  ->  FUN_142cbaa80(this, opcode, packet)
                              (64 bytes)         (12107 bytes, 181 cases, 0x70..0x39a)

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

## The case table

`research/msexe-gamestage-cases.txt` - 181 `opcode -> handler` pairs in enum order.

The largest handlers, which is a decent proxy for how much a packet carries:

| opcode | handler | size |
|---|---|---|
| `0x008c` | `FUN_142d634c0` | 188565 |
| `0x00a9` | `FUN_142ddfd10` | 23440 |
| `0x0116` | `FUN_142cf6d40` | 16260 |
| **`0x0070`** | **`FUN_142d51930`** | **11712** |
| `0x00a7` | `FUN_142defd40` | 9620 |
| `0x0145` | `FUN_142da7550` | 8819 |

`0x0070` is the **first** case in the switch and one of the largest handlers, which is why
it is the first suspect for the packet that answers the client's migration hello. That is
a suspicion, not a finding - `research/msexe-setfield.md` is where it gets settled.

## Not yet established

* A positive trace of **when `FUN_142ca5c50` runs**. The argument above says the world
  object is live by migrate time because the migrate handler dereferences it without a
  check; it does not say when it was built. Its callers are `FUN_142c43970` <- `FUN_142c42f30` <-
  `FUN_142ef49e4`, and that last one has no callers and no address taken either, so the
  chain runs off into the virtualised region and static reading stops there.
* Whether the login connection reaches `FUN_141b25f30` through this same entry or through
  the vtable it does sit in (slot 76 of `0x1433fd540`).
* Opcodes `0x60..0x6f`, which neither dispatcher claims.

