# The fault that ended the last run is a scope guard, not a teardown

**Written 2026-08-19, before the next client launch.** It changes how the next run must be
read, so it is worth stating before the run rather than after.

## What was believed

`STATUS.md` recorded the fault at `0x140ce89d6` (`FUN_140ce89c0+0x16`) as "a refcount
release holding a non-null but invalid pointer - the signature of an object that was never
properly constructed and is then released during teardown", and concluded: the map was `0`,
the field never loaded, teardown fell over. That reading is **consistent with** the
evidence. It is not the only one, and the difference matters.

## What the callers actually look like

`FUN_140ce89c0` has 5 call sites (`python tools/callers.py 0x140ce89c0`). The two in real
code are `0x1415d6a34` and `0x1415d6c85`, and **both have the identical shape** - read off
the listing, `research/msexe-faultcaller-a.txt` and `-b.txt`:

```asm
1415d6a2f  LEA   RCX,[RSP + 0x38]
1415d6a34  CALL  0x140ce89c0
1415d6a39  ADD   RSP,0x88
1415d6a40  RET

1415d6c80  LEA   RCX,[RSP + 0x50]
1415d6c85  CALL  0x140ce89c0
1415d6c8a  ADD   RSP,0x68
1415d6c8e  RET
```

`LEA` a **stack local**, call the release, then the epilogue and `RET`. That is a C++
scope-exit destructor on a local smart pointer, not the teardown of a long-lived object.

And `FUN_140ce89c0` is:

```c
obj = holder->[8];
if (obj != 0) { ... atomic_dec(obj->[0x28]); ... }   // faulted reading [RBX+0x28]
```

So the *holder* is a stack local, its `+8` was **non-null garbage**, and the null check
passed on a value that was never written. **The local was never initialised.** Something on
the path between the prologue and the epilogue was supposed to fill it and did not.

## Why this changes how to read the next run

> **A repeat of the same fault does NOT prove the map id is still wrong.**

This is a latent bug in the client's own error path: *whenever* one of these two functions
takes an early exit that skips the acquisition, the guard at the epilogue runs over
uninitialised stack and faults. Every failure inside those functions produces the **same
address**, and the address cannot say which failure it was.

So `0x140ce89d6` again would mean "that function bailed out again", not "the map is still
zero". The signals that *do* discriminate are the two probe watches - `140304b20` and
`140302e30` - and whatever appears on screen.

Conversely, a fault at a **different** address is still real progress, exactly as before.

## Where these two functions sit

| | |
|---|---|
| `0x1415d6a50` (575 bytes, call at +0x235) | called twice from `0x1415d5710`, which has **zero call sites** - the Themida virtualised-entry signature, and it is next door to the three known dispatch entries `FUN_1415d59b0/a00/a50` |
| `0x1415d60e0` (2401 bytes, call at +0x954) | called from `0x1415d36c0`, which is called from `0x142c45e50` - the **world-object** range, the neighbourhood of the world-init chain `FUN_142c42f30` |

Ghidra decompiles both to `halt_baddata()` and gives their length as 22 and 23 bytes;
`.pdata` says 2401 and 575. Per `docs/ghidra.md` that is a lead, not a wall - the listing
read fine. The **middle** of both is junk under a linear dump and was not read; only the
epilogue is claimed here, and it is claimed because it decodes cleanly and lands exactly on
the `.pdata` bound with `INT3` padding after it.

## Two small confirmations along the way

* `DAT_14337b3c8` dumps as `20 0d 0a 09 00` - the ASCII separator set `" \r\n\t"`, which is
  what turns the literal `"\tM\rA\nP"` into `MAP`. The neighbouring literal is
  `53 63 72 0d 65 65 0a 6e` = `Scr\ree\nn` = `Screen`. The string-splicing trick in
  `docs/ghidra.md` is now read from bytes rather than inferred.
* `PTR_s_mapName_143a49020` is a table of four pointers, the first being `0x1432b3ec0`.
