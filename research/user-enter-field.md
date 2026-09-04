# `0x0224` UserEnterField and `0x0225` UserLeaveField — the two packets that put a second
# player on screen

**2026-08-29. Static only — no client run.** Ghidra was held for this pass; the decompilation
is in `research/msexe-userpool-enter.c`, the listings in `research/msexe-userpool-0224-enter.txt`,
`research/msexe-userpool-0225-leave.txt`, `research/msexe-userpool-userinit-1429ce270.txt` and
`research/msexe-userpool-onpacket-1429b9300.txt`.

Tags: **[L]** read off a listing, a table or raw bytes; **[D]** derived from two or more [L];
**[I]** inferred, including anything from the v214 reference tree, **which scores 1 of 8 and is
a candidate generator only**.

---

## 0. The answer, before the working

* **`0x0224` is enter, `0x0225` is leave, and it is not the enum order that says so.** `0x0224`'s
  handler allocates a `0x4438`-byte `CUser`, runs its constructor, and **inserts it into the
  pool's hash map**. `0x0225`'s handler **unlinks a node from that same hash map and calls its
  deleting destructor**. Both **[L]**.
* **`0x0225`'s body is exactly `u32 charId` — one read, at depth 4, and nothing else.** **[L]**
* **`0x0224` reuses the client's compact avatar-look reader verbatim.** `FUN_1429ce270` calls
  **`FUN_1402ee8d0` at `0x1429ce6a9`** — the same function `0x0107` / `0x0114` / `0x0138` use and
  the one `crates/net/src/opcode.rs::avatar_look()` was written against. **`avatar_look()` is
  reusable byte-for-byte, at body offset 187.** **[L]**
* Minimum body for a plain remote character: **508 bytes** + 5 per equipped item + the bytes of
  five strings. With an 8-character name and 5 equips, **541**.
* Re-sending `0x0224` for an id already in the pool is a **silent no-op** — the handler returns
  before it reads a single byte of the body. Sending `0x0224` for **our own** character id
  clears one dword in `CWvsContext` and returns; no `CUser`, no insert, no body read. Both **[L]**.

---

## 1. Routing, confirmed from `FUN_1429b9300` itself

`research/talking-back.md` §1.3 says `CField::OnPacket` (`FUN_141820080`) routes `0x224..0x39F`
to `FUN_1429b9300`. The brief's summary said `0x224` and `0x225` are "handled inline". They are
not — each has its own function. From `research/msexe-userpool-onpacket-1429b9300.txt`, **[L]**:

```text
1429b9317  mov ecx, edx
1429b9319  sub ecx, 0x224
1429b931f  je  0x1429b9467          ---> 1429b9467  mov rdx,rsi / mov rcx,rdi / call 0x1429ba3e0
1429b9325  cmp ecx, 1
1429b9328  je  0x1429b944d          ---> 1429b944d  mov rdx,rsi / mov rcx,rdi / jmp  0x1429ba980
1429b932e  lea eax, [rdx-0x226] / cmp eax,0x6c / ja   -> tail jmp 0x1429bafb0   (0x226..0x292)
1429b9350  lea eax, [rdx-0x293] / cmp eax,0x31 / ja   -> tail jmp 0x1429bb720   (0x293..0x2c4)
1429b9372  lea eax, [rdx-0x2c5] / cmp eax,0xd9 / ja               (0x2c5..0x39e, local user)
```

So **`0x0224` → `FUN_1429ba3e0`** (a `call`; it falls through to the epilogue) and
**`0x0225` → `FUN_1429ba980`** (a tail `jmp`). `FUN_1429ba980` is a **new name** — nothing in
`research/` had it.

`0x0225`'s handler being reached by a tail `jmp` is worth noting because it is exactly the shape
that made `research/user-chat.md` call `FUN_1429bafb0` "virtual" (`tools/callers.py` counted
`call` only). **That is now fixed** — today's `tools/callers.py` prints tail-jmp sites and data
pointers in separate sections. The stale warning in `research/user-chat-round2.md` §8 can be
struck. **[L]** (observed in this session's tool output).

### Which is enter and which is leave — from the bodies, not the enum

**`0x0224` = enter.** `FUN_1429ba3e0`, `research/msexe-userpool-enter.c` lines 126-172, **[L]**:

```text
1429ba60b  mov edx, 0x40   / call 0x14019b780      allocate a 0x40-byte ZRef control block
1429ba662  mov edx, 0x4438 / call 0x14019b780      allocate a 0x4438-byte CUser
1429ba685  call 0x1429cdb70(mem, field1, charId)   CUser::CUser
1429ba7ab  lea rcx,[r15+0xf8] / lea rdx,[rbp+0x77] / lea r8,[rbp-0x51]
1429ba7b2  call 0x1429be2e0                        INSERT (id -> ref) into the pool hash map
1429ba856  call 0x1429ce270(user, packet, ..., 1)  decode the rest of the body into it
```

`pool+0xf8` / `pool+0x100` are the 31-bucket hash the constructor builds — established
independently in `research/user-chat-round2.md` §4. **[L]**

**`0x0225` = leave.** `FUN_1429ba980`, same file lines 461-490, **[L]**: it hashes the id into
the same table, walks the chain, and then

```c
puVar16 = bucket[uVar9];
if (bucket_head->key == uVar1) { bucket[uVar9] = head->next; (**head)(head, 1); }
else { walk; prev->next = node->next; (**node)(node, 1); }
```

`(**node)(node, 1)` is the scalar deleting destructor. Before that it removes the user from the
trade/partner lists at `pool+0x20`, `+0x58`, `+0x70`, `+0x88`, calls `FUN_1429b9f20(pool, id)`
and `FUN_1429c0470(pool+0xc8)`, and drops the refcounts. **A pool erase. The enum-order argument
is no longer load-bearing — this is [L].**

---

## 2. `0x0224` field table

Offsets are for the **minimum** body: five empty strings, no equipped items, every optional
block absent. Every address is the instruction that performs the read.
Strings are `0x1406e9050` — **`u16` length then that many bytes** (2 bytes when empty).

### 2.1 Header, decoded by `FUN_1429ba3e0` itself

| off | at | type | what | tag |
|---|---|---|---|---|
| 0 | `1429ba40a` | u32 | passed to `CUser::CUser` as arg 2. Nothing else in the handler touches it | [L] |
| 4 | `1429ba43b` | u32 | **the character id** — hash key, `CUser::CUser` arg 3, `FUN_142dec860(ctx,id)` | [L] |
| — | `1429ba44d` | u32 | **read only if the field at +4 was `0`**, and then *this* is the character id | [L] |
| 8 | `1429ba4a0` | u32 | compared against `FUN_142cc0400(CWvsContext)` in one UI mode; mismatch → return | [L] |

**The `if (field == 0) read another` at `+4` is real and it is not a decompiler artifact.** The
listing is unambiguous (`test eax,eax / jne 0x1429ba4f4`, and the `0x1429ba4f4` arm does not
re-read). The identical idiom sits in front of `0x0226` in `FUN_1429bafb0` at `0x1429bb001` /
`0x1429bb00c`, with `% 27` instead of `% 31` — `research/user-chat-round2.md` §5 recorded it
there and it is the same construct. **[L]**

*Why it is probably there:* the v214 reference writes `encodeInt(user.getId()); encodeInt(0);
encodeInt(chr.getId());` — a literal zero in slot 2 followed by the real id. That is exactly the
shape this branch consumes. **[I]** — but it means **both encodings decode**, and the shorter one
(a nonzero id in slot 2, three u32s total) is what the client's own code reads first.

A **captured** body from that tree agrees, and it is worth having because it is bytes rather than
a code reading — `ModernMapleSource/v214 src/logs/26 February 2026/Packets/Paladin.txt` line 1929,
929 bytes, **logged as `v.265`**:

```text
03 00 00 00      userId = 3
00 00 00 00      <- the literal zero in slot 2
05 00 00 00      charId = 5
00               byte
00 00 00 00      guildId
2C 01 00 00      level = 300              <- our body field 1
0C 00 <12 bytes> name                     <- our body field 2
00 00            "" parent name           <- our body field 3
...              then a long run of zeros: empty guild block, gender, fame, name-tag mark
```

**Read that as corroboration of the *idiom*, not of our layout.** The capture is `v.265`, a
version further from this client than the v214 source already is, and it diverges immediately
after: it carries a `byte` and a `guildId` between the header and the level, which
`FUN_1429ba3e0` does not read, and its avatar look terminates the equipment maps with **three**
`FF` where `FUN_1402ee8d0` has **two** loops. What it does establish is that "a zero in the id
slot, with the real id in the next dword" is a real encoding in this family — which is what the
branch at `0x1429ba444` exists to consume. Still **[I]**.

*And the `% 31` test next to it is harmless.* Both arms converge on a `std::function`-shaped local
`{ vtable 0x143409208, &LAB_140c93920 }`, and `0x140c93920` is **seven bytes**:
`c6 05 09 44 e3 02 01` `c3` = `mov byte [0x143AC7D30], 1 ; ret`. It sets a global flag and
returns. It is not an assert and it cannot fault, so a character id that happens to be a multiple
of 31 (217, 248, …) is not a hazard. **[L]**, read out of raw bytes rather than the decompiler.

### 2.2 Body, decoded by `FUN_1429ce270` (`CUser::Init`-shaped; **one caller**, `0x1429ba856`)

`param_1` is the new `CUser`; offsets in the last column are into it.

| off | at | type | bytes | → CUser | candidate name |
|---|---|---|---|---|---|
| 12 | `1429ce310` | u32 | 4 | `+0x406c` | level **[I]** |
| 16 | `1429ce323` | str | 2 | `+0x10d8` | **name** — **[L]**, `research/user-chat-round2.md` §3 proved `user+0x10d8` is the speaker name via the `"%s : %s"` format |
| 18 | `1429ce371` | str | 2 | `+0x10e0` | parent name, deprecated **[I]** |
| 20 | `1429ce3bb` | u32 | 4 | `+0x10e8` | guild id **[I]** |
| 24 | `1429ce3ce` | str | 2 | `+0x10f0` | guild name **[I]** |
| 26 | `1429ce418` | u16 | 2 | `+0x10fe` | guild logo background **[I]** |
| 28 | `1429ce428` | u8 | 1 | `+0x1100` | guild logo background colour **[I]** |
| 29 | `1429ce437` | u16 | 2 | `+0x1102` | guild logo **[I]** |
| 31 | `1429ce447` | u8 | 1 | `+0x1104` | guild logo colour **[I]** |
| 32 | `1429ce456` | u32 | 4 | `+0x1108` | **[I]** guild-block tail |
| 36 | `1429ce465` | u32 | 4 | `+0x110c` | **[I]** guild-block tail |
| 40 | `1429ce474` | u8 | 1 | `+0x10fc` | gender **[I]** |
| 41 | `1429ce483` | u32 | 4 | `+0x3b28` | **a per-map array index. NOT fame** - see below **[L]** |
| 45 | `1429ce492` | u32 | 4 | `+0x3770` | name-tag mark **[I]** |
| 49 | `1429ce4a1` | u8 | 1 | `+0x3774` (zero-extended to dword) | **[I]** |
| 50 | `1429ce4b3` | u32 | 4 | `+0x13b4` | **[I]** |
| 54 | `1429ce4c2` | u8 | 1 | `+0x4064` ← `(byte == 1)` | **[I]** |
| **55** | `1429ce4e4` | **raw 124** | **124** | `FUN_140a46e50([user+0x4070], …, pkt)` | **the remote temporary-stat block** |
| 179 | `1429ce4ec` | u16 | 2 | `+0x40c0` (zero-extended) | **job** **[I]**, but see below |
| 181 | `1429ce4fe` | u16 | 2 | `+0x40c4` (zero-extended) | sub-job **[I]** |
| 183 | `1429ce510` | u32 | 4 | `+0x4358` | **[I]** |
| **187** | `1429ce6a9` | **avatar look** | **195 + 5·equips** | `FUN_1402ee8d0(&look, pkt, &str, 0)` | **`avatar_look()` verbatim** |
| 382 | `1429ce6b2` | u32 | 4 | `+0x1260` | driver id **[I]** |
| 386 | `1429ce6c1` | u32 | 4 | `+0x1264` | passenger id **[I]** |
| 390 | `1429ce6da` | u8 | 1 | `vtable[0x178](user, v)` | **[I]** |
| 391 | `1429ce6ea` | u32 | 4 | compared with `+0x1418`, then `FUN_1427eb600` | **[I]** |
| 395 | `1429ce6f4` | u32 | 4 | `FUN_14277cd40(user, v)` | **[I]** |
| 399 | `1429ce6fe` | u32 | 4 | `FUN_14277e0c0(user, v, 0)` | **[I]** |
| 403 | `1429ce70a` | u8 | 1 | flag — **if nonzero, a `str` follows** → `+0x2da8` (`1429ce71a`) | **[L]** |
| 404 | `1429ce764` | u32 | 4 | `+0x1308` | damage-skin id **[I]** |
| 408 | `1429ce777` | str | 2 | `+0x1310` | **[I]** |
| 410 | `1429ce7c5` | str | 2 | `+0x1318` | **[I]** |
| 412 | `1429ce80f` | u32 | 4 | `+0x2e08` | **[I]** |
| 416 | `1429ce81e` | **i16** | 2 | `+0x3c28`, sign-extended. Read through the **u16 thunk `0x1406e8ef0`** | field seat id **[I]** |
| 418 | `1429ce830` | u32 | 4 | `+0x4080` | **chair / portable-chair item id** — it is the type argument to the factory at `1429cea9c` **[D]** |
| 422 | `1429ce843` | u32 | 4 | `+0x3b78` | **[I]** |
| **426** | `1429ce852` | **i16** | 2 | → `r8d` of `vtable[0x118]` | **X** **[D]** |
| **428** | `1429ce85f` | **i16** | 2 | → `r9d` of `vtable[0x118]` | **Y** **[D]** |
| **430** | `1429ce86c` | **u8** | 1 | `+0x6e4`, then `[rsp+0x30]` of the same call | **moveAction / stance** **[D]** |
| **431** | `1429ce885` | **i16** | 2 | `FUN_142df6c50([0x143AC18D8], v)` → `[rsp+0x38]` of the same call | **foothold id** **[D]** |
| 433 | `1429ce89c` | u8 | 1 | `FUN_141b0e070(user, v != 0)` | **[I]** |
| 434 | `1429ce8b1` | u8 | 1 | `FUN_141b0e080(user, v != 0)` | **[I]** |
| 435 | `1429cea87` | u8 | 1 | **chair/miniroom present**. Nonzero → `FUN_141712040(&o, user, +0x4080)` then `o->vtable[0x18](o, pkt)` — **an indirect decode, see §6** | **[L]** |
| 436 | `1429cec20` | u8 | 1 | **pet loop**: while nonzero → `u32 (1429cec33)`, `FUN_141eb9760(pet,user,id,pkt)`, `u8 (1429cec8a)`. `0` ends it | **[L]** |
| 437 | `1429cec9d` | u8 | 1 | **second loop** (familiars **[I]**): while nonzero → `vtable[0x150](user, pkt)`, `u8 (1429cecc2)`. **Indirect, see §6** | **[L]** |
| 438 | `1429cece7` | u32 | 4 | `+0x34c4` | taming-mob level **[I]** |
| 442 | `1429cecf6` | u32 | 4 | `+0x34c8` | taming-mob exp **[I]** |
| 446 | `1429ced05` | u32 | 4 | `+0x34cc` | taming-mob fatigue **[I]** |
| 450 | `1429ced26` | u8 | 1 | flag → `FUN_141775d90` then `o->vtable[8](o, pkt)`. **Indirect, see §6** | **[L]** |
| 451 | `1429cee29` | raw 4 | 4 | `+0x1118`. **If nonzero, a large block follows** — see §2.3 | **[L]** |
| 455 | `1429cf0db` | sub | 18 | `FUN_14073a7b0(user+0x1148, pkt)` = `raw4, u32, raw4, str, u32` — **unconditional** | **[L]** |
| 473 | `1429cf14a` | u8 | 1 | `+0x40b0`. Nonzero → `str (1429cf169)` | **[L]** |
| 474 | `1429cf9f8` | u8 | 1 | nonzero → `raw8, raw8, u32` → `FUN_1429b9490` | couple ring **[I]** |
| 475 | `1429cfa52` | u8 | 1 | nonzero → `raw8, raw8, u32` → `FUN_1429b9740` | friendship ring **[I]** |
| 476 | `1429cfaac` | u8 | 1 | nonzero → `u32, u32, u32` → `FUN_1429b9c80` | marriage ring **[I]** |
| 477 | `1429cfafe` | u8 | 1 | nonzero → `u32 count (1429cfb0a)`, then `count ×` `FUN_141404ee0` (`u32, u8` each) | **[L]** |
| 478 | `1429cfb38` | u8 | 1 | a bitmask. **bit `0x20` set → one `u32` (`1429cfb65`)**. bit `0x02` gates behaviour only | **[L]** |
| 479 | `1429cfb96` | u32 | 4 | `FUN_1427eec30(user, v)` — that callee reads nothing | **[L]** |
| 483 | `1429cfba8` | u32 | 4 | `FUN_142833030(user, v, pkt)` — reads nothing at depth 4 | **[L]** |
| — | `1429cfc0a` | u32 | 0 | **client-state gated, not wire gated — see §3** | **[L]** |
| 487 | `1429cfd2f` | sub | 4 | `FUN_142834df0`: `u32 count`; per entry `u32, str` | **[L]** |
| 491 | `1429cfd3a` | sub | 5 | `FUN_142835840` → `FUN_1408cf0d0`: `u8` then `u32`, both unconditional | **[L]** |
| 496 | `1429cfd45` | sub | 4 | `FUN_1428358a0`: one `u32` | **[L]** |
| 500 | `1429cfd4d` | u32 | 4 | count; per entry `FUN_1413ebe00(o, pkt, 0)` (`u32,u32,raw,u32×…`) | **[L]** |
| 504 | `1429cff0e` | u32 | 4 | count; per entry `u32 (1429cff33)`, `u8 (1429cff3d)` | **[L]** |

**Total minimum: 508 bytes.** Arithmetic done by script, not by hand.

### 2.3 The one large optional block

`0x1429cee29` reads **4 raw bytes** into `user+0x1118`. If that dword is nonzero the client then
reads, in order — **[L]**:

```text
1429cee44  u32   -> +0x1120
1429cee5e  raw 4 -> +0x111c
1429cee75  str   -> +0x1128
1429ceebf  u32   -> +0x113c
1429ceece  u8    -> +0x1130
1429ceee0  u8    -> +0x1138
1429ceef2  u8    -> +0x1134
1429cef04  u8    -> +0x1140
1429cf069  FUN_1408d6760(obj, pkt)     the 33-byte "speaker object" from research/user-chat.md
           FUN_1415ed1c0(&obj, &str, 0x1f)   -> posts it as a chat line
```

That is a **miniroom/shop announcement plus its chat line** **[I]** — the v214 reference does
exactly `miniRoom.encode(); chr.encodeChatInfo(...)` at the matching position, and
`FUN_1415ed1c0(.., 0x1f)` is the same chat outlet `research/user-chat-round2.md` §2 found behind
bit 2 of the chat flag byte. **Send `00 00 00 00` here and none of it is read.**

---

### 2.4 Row 41 was called "fame" and it is an array index - corrected 2026-09-03

The name came from the v214 reference tree and was tagged **[I]** honestly. It was still
written down as a name and read back as a fact, and `crates/net/src/userpool.rs` sent `0`.

`0` is a **valid array index**. The field reaches `CUser+0x3b28`, vtable slot `+0x18` hands it
to `0x14182a140`, and that accessor **reports** an out-of-range index and then honours it -
returning `base + 48*idx + 8`. With a null base and index 0 that is the address `8`, and
`0x140f9295e mov rcx,[rax]` dereferences it. Both clients died there, both dumps identical
register for register. **[L]**

`-1` is the client's own "no entry": its local `CUser` carries `0xFFFFFFFF` here, and all three
call sites are gated `cmp eax,-1 / je`. Diffing the local and remote `CUser` dword by dword
over all `0x4400` bytes gives **exactly one** offset that is `-1` in the local and `0` in ours.
**[L]** `research/0x0224-remote-user-first-use-fault.md`.

The array is **not named** - **[I]** only, possibly the map's seat list (the enclosing error
string carries `bSit`). The fix does not depend on knowing. But the symptom is map-dependent:
on a map whose array is non-empty, index 0 resolves silently and pins every remote avatar to
entry 0 rather than crashing.

**A loose end, not on the fault path:** this table maps body 416 to `CUser+0x3c28`, and both
dumps read **100** there for the local *and* the remote user while the builder writes 0. Either
the row is wrong or something overwrites it after `Init`. Nobody has checked.

---

## 3. Two things the wire does not control

Everything above is a function of the bytes except two blocks, and the second one matters:

1. `0x1429ceb8d` `cmp [rsp+0x60],0 / je` and the `FUN_140f8abc0(user+0x20)` tests — these gate
   *behaviour*, not reads. Confirmed by walking the listing between them. **[L]**
2. **`0x1429cfc0a` is gated on the client's own state:**

```c
if (FUN_14087b630(user+0x4070)) {                 // a taming-mob / vehicle item is equipped
    int id = FUN_14087bc10(user+0x4070);
    if (FUN_1407e6080(id)) {
        int n = Decode4();                         // 0x1429cfc0a
        for (i < n) arr[i] = Decode4();            // 0x1429cfc23
    }
}
```

`FUN_14087b630` compares an item id against `0x1cfde0`(1900000)+0x2710, `0x1d7310`(1930000)+0x2710,
`0x1e4218`(1983000)+0x3e8 and `0x1e4600`(1984000)+0x3e8 — the taming-mob / vehicle item ranges.
**[L]** on the constants, **[D]** on the meaning. The v214 reference has the matching block behind
`RideVehicle && vehicleID == 1932249` (`is_mix_vehicle`) — **[I]**, and it corroborates rather
than decides.

**Consequence: the body length depends on what the avatar look we just sent has equipped.** As
long as we send no mount/vehicle item, the block is absent. If a mount is ever equipped and this
is not accounted for, the client will read four bytes that are not there. Worth a comment in the
builder.

---

## 4. `avatar_look()` is reusable verbatim, at offset 187

`FUN_1429ce270` calls `FUN_1402ee8d0` at **`0x1429ce6a9`** with
`rcx = &localAvatarLook`, `rdx = the packet`, `r8 = &aZString`, `r9d = 0` — the same
`(look, pkt, str, 0)` shape `crates/net/src/opcode.rs`'s `USER_AVATAR_MODIFIED` doc comment
records for `0x0138`. `research/avatar-look-reader.c` is the decompilation of that function and it
lines up with `avatar_look()` field for field:

```text
u8 gender -> +0x20      u8 skin -> +0x21       u32 -> +0x25          u32 face -> +0x29
u32 job -> +0x1bd       u8 (discarded)          u32 hair -> +0x39
u8 slot; while != 0xFF { u32 itemId }   -> +0x39 + slot*4
u8 slot; while != 0xFF { u32 itemId }   -> +0xb9 + slot*4
u32 -> +0x2d   u32 -> +0x31   u32 -> +0x35   u32 -> +0x1c1
u32 (taken % 360) -> +0x1c5    u8 -> +0x1c9   u32 -> +0x1ca
raw 4 -> +0x1b9     raw 128 -> +0x139     u32 -> +0x1d2     raw 13 -> +0x1d6
```

Length = **195 + 5 per equipped item** (19 head, 5 per equip, two `0xFF` terminators,
16 + 4 + 1 + 4 + 4 + 128 + 4 + 13 = 174 tail). `CHARACTER_NAME_LEN` is 13, which is the trailing
raw 13. **[L]/[D]**

**These exact bytes have already been through this exact reader on the wire** — they dress the
character-select screen. That is the difference between a week and an afternoon.

Caveat worth keeping: the u32 written into `+0x1bd` is the job, and it is **inside** the avatar
look; the `u16` at body offset 179 is a *separate* job field on the `CUser`. Send both.

---

## 5. What happens on a duplicate id, and on our own id

Both are answered off the listing, and both matter because the server will re-send these.

### A `0x0224` for a character id already in the pool — silent no-op, body not read **[L]**

```text
1429ba523  mov r8,[r14+0xf8]        the bucket array
1429ba52f  mov ecx,[r14+0x100] / div rcx / mov rax,[r8+rdx*8]
1429ba546  cmp dword [rax+0x10], esi   / je 0x1429ba556
1429ba556  add rax, 0x18 / jne 0x1429ba955      ---> the epilogue
```

`add rax,0x18` clears ZF only if `rax == -0x18`, which a heap node never is, so **the branch is
always taken and the handler returns.** No duplicate insert, no fault, and — because this is
*before* `0x1429ba856` — **not one byte of the body is consumed**. To move or redress an existing
remote user you must either send `0x0225` first or use one of the `0x293..0x2c4` remote opcodes.
`FUN_1429bb720` reads its `u32` charId at `0x1429bb745` and looks it up in the same hash, so those
opcodes reach the same object.

### A `0x0224` carrying **our own** character id — clears one dword, returns **[L]**

```c
uVar6 = FUN_142cb9550(DAT_143aa84a0);        // mov eax,[ctx+0x232c] ; ret
if (charId == uVar6) {
    if (FUN_141892840() != 0)                 // the current field
        if (FUN_141883ea0(field) == '\0')     // 32 c0 c3  =  xor al,al ; ret  -> ALWAYS 0
            FUN_142d09590(ctx, 0);            // 89 91 10 37 00 00 c3 = mov [ctx+0x3710], 0
}
return;
```

`0x143AA84A0` is the `CWvsContext` singleton — established in `research/channel-select.md`,
`research/cash-shop.md` and `research/change-channel-reply.md`. `FUN_142cb9550` is a two-instruction
getter for `ctx+0x232c`; its sibling `0x142cb9560` is the matching setter, with two call sites.
**[L]** on the bytes, **[D]** on "`ctx+0x232c` is the local character id".

So a self-addressed `0x0224` **creates nothing, inserts nothing and reads no body**. It sets one
dword to zero. It is not a crash and it is not a duplicate — but it is also not useful, so the
server should skip itself when broadcasting.

### The gates in front of the insert, enumerated

Everything below returns without creating a `CUser`. All **[L]**, from
`research/msexe-userpool-enter.c` lines 61-124:

| gate | condition | note |
|---|---|---|
| own id | `charId == ctx[0x232c]` | above |
| already present | hash hit in `pool+0xf8` | above |
| `FUN_142cc3d80(ctx) != 0` | | whole `else` arm skipped |
| `FUN_141bc8c60(field)` | true → return | |
| `FUN_141bc8c80(field)` | true → then `thunk_FUN_1413b8e00(ctx)==0` → return, or `FUN_142dec860(ctx,id)==0` → return | |
| `FUN_141bc8ca0(field)` | true → `FUN_142cc0400(ctx)==0` → return, or `!= header field 3` → return | this is what header field 3 is for |

`FUN_141892840` is the current-field getter — named in `research/user-move.md` §1 and
`research/item-drop.md` §1, so these are field-mode tests, not user tests. The same three
`FUN_141bc8c6/8/a0` gates sit in front of `FUN_1429bb720`'s remote opcodes at `0x1429bb7d6`,
`0x1429bb7f5` and further down, which is a second, independent sighting of the same trio.

**None of the six has ever been observed live**, and any of them silently swallows the packet.
If the first two-client test shows nothing, this table is where to look, and a watch on
`0x1429ba60b` (past every gate, at the allocation) answers it in one run.

---

## 6. Instrument checks, and what I could NOT settle

### The controls that were run first

* `python tools/listing.py 0x140304100 | grep READ` printed reads at `140304138 raw`,
  `140304144 u8`, `140304183 u8`, then a run of `u16` — its own documented control. Passed.
* **Read count cross-checked between the two instruments, as `docs/ghidra.md` demands.**
  `tools/reads.py 0x1429ce270 3` gives **86 direct reads** — 27 `u8`, 8 `u16`, 37 `u32`, 8 `str`,
  6 `raw`. Counting primitive calls in the Ghidra decompilation of the same function gives
  **86**, split 27 / 7+1 thunk / 37 / 8 / 6. **Identical in count and in type distribution.**
  That is the check that would have caught a missed tail `jmp`, and it is clean.
* **The remote-table positive control the brief asked for.** Walking `0x1429bbc34` from the raw
  image with `index = opcode - 0x29e` (the base and the `cmp eax,0x26` bound read off
  `0x1429bb940`): index **7 → `0x02A5`**, stub `0x1429bb9b2 → 0x1429d48c0`; index **0x11 →
  `0x02AF`**, stub `0x1429bba36 → call 0x1427863f0`. Those are exactly what `research/user-hit.md`
  §5.3 and `research/level-up.md` §6 established independently, and exactly what the pre-existing
  `research/msexe-userpool-tables-raw.txt` says. 39 entries, 20 distinct bodies, `0x1429bbb10`
  is the `default:` arm with 17 opcodes on it.
  *Note this table is not on `0x0224`'s path* — `0x0224` and `0x0225` are dispatched by
  `sub`/`cmp`/`je`, not by a table, so there was no table of mine to validate. The control
  validates the method, not the result.
* `tools/pdata_lookup.py` was used for every function bound before dumping, per
  `docs/ghidra.md`'s "bound every dump by `.pdata`".
* All tooling run with the repo as the working directory.

### Named blind spots — a hedged negative

1. **Four decodes happen through indirect calls and no static instrument here can follow them.**
   `[chairObj+0x18](o, pkt)` at `0x1429ceb44`, `[user vtable+0x150](user, pkt)` at the second
   loop, `[o+8](o, pkt)` at the `FUN_141775d90` block, and the pet decoder's own internals.
   **All four sit behind a wire flag we can send as zero**, so the minimum body is unaffected —
   but the moment a chair, a familiar or a pet is sent, the length above is wrong and the shape
   has to be read out of the concrete class first. This is the same class of blind spot that
   `CLAUDE.md` records for `mob+0x42c`: the write is through a pointer that was handed off.
2. **`FUN_142833030(user, v, pkt)` takes the packet and `tools/reads.py` finds no reads in it at
   depth 4.** That is consistent with "it does not read", and it is also consistent with "it reads
   through an indirect call". I did not open it. If the first two-client test desynchronises after
   body offset 483, this is the first suspect.
3. **X and Y are `[D]`, not `[L]`.** Two signed `u16`s in a row become args 3 and 4 of
   `vtable[0x118]`; nothing in the listing labels which is which. What makes it [D] rather than
   [I] is that `research/user-move.md` measured `i16 x` then `i16 y` in the client's **own**
   outbound movement path against 1082 captured bodies, and the v214 reference writes
   `encodePosition` at the matching offset. If the first remote character appears rotated 90° in
   the map, swap these two before touching anything else.
4. **Every name in the "candidate" column of §2.2 marked [I] is from the v214 reference**, which
   scores 1 of 8 against a held-out control. The alignment is unusually good — level, name, parent
   name, the six-field guild block, gender, fame, job, sub-job, avatar look, driver/passenger,
   chair, position, moveAction, foothold, chair-encode flag, pet loop, familiar loop, three
   taming-mob dwords, miniroom, three ring flags, the `0x20`-bit mask, then the rings — but
   **alignment is not identification**, and the two versions demonstrably differ: our temporary-stat
   mask is `0x7C` = 124 bytes where the reference's `DecodeForRemote` reads `0x84` = 132. **Field
   *positions* are [L]; field *meanings* are [I] until something on screen agrees.**
5. **Whether `FUN_1429ce270`'s reads are all on one straight path was checked against the
   decompiler, not proved by a dominator analysis.** The reads.py `gated?` column flags every
   read after any conditional branch and so over-reports; I resolved each by reading the listing
   and the decompiled control flow, and the two agree on the sequence. A branch that skips a read
   in a way both instruments render the same way would not have been caught.
6. **Nothing here has been on the wire.** No `0x0224` has ever been sent by this server, and
   `research/user-chat-round2.md` §1 records that **no packet in `0x224..0x39F` has ever been
   observed to do anything** in any archived run. The whole user-pool branch is unexercised.

### One correction to a file I am not editing

`research/user-chat-round2.md` §8 closes with "**Instrument fix, out of scope here but
load-bearing:** `tools/callers.py` counts `call` only." That is no longer true —
`python tools/callers.py 0x1429ce270` printed a `call` section, a **tail jmp** section and a
**qword pointer** section in this session. The instrument has been fixed; the warning in that
file is stale. I have not edited it.

---

## 7. WIRE IT LIKE THIS — and it is **not wired**

Nothing in this document is connected to anything. `crates/net/` has no `0x0224` builder and
`crates/world/src/session/` has no broadcast.

```text
0x0224  UserEnterField                              0x0225  UserLeaveField
  u32   userId          (any nonzero value)           u32  charId
  u32   charId          (nonzero -> no extra field)
  u32   fieldCheck      (0 is fine unless the        <- that is the entire body.
        FUN_141bc8ca0 UI mode is active)
  ---- CUser::Init, from here on ----
  u32   level
  str   name
  str   ""                     parent name
  u32   guildId
  str   ""                     guild name
  u16 u8 u16 u8                guild logo/bg + colours
  u32 u32                      guild block tail
  u8    gender
  u32   fame
  u32   nameTagMark
  u8    ?
  u32   ?
  u8    ?                      -> stored as (v == 1)
  raw   124 zero bytes         the remote temporary-stat mask; all clear = no buffs
  u16   job
  u16   subJob
  u32   ?
  ---- avatar_look(chr), unchanged ----
  u32 u32                      driver, passenger
  u8    ?
  u32 u32 u32                  ?
  u8    0                      no trailing string
  u32   ?
  str "" str ""
  u32   ?
  i16   ?                      field seat
  u32   0                      chair item id
  u32   ?
  i16   x
  i16   y
  u8    moveAction
  i16   foothold
  u8 0  u8 0
  u8 0                         no chair to decode
  u8 0                         no pets
  u8 0                         no familiars
  u32 u32 u32                  taming-mob level / exp / fatigue
  u8 0
  raw   00 00 00 00            no miniroom
  raw4 u32 raw4 str("") u32    FUN_14073a7b0, unconditional
  u8 0
  u8 0  u8 0  u8 0             couple / friendship / marriage
  u8 0
  u8 0                         the 0x20-bit mask
  u32 u32
  u32   0                      FUN_142834df0 count
  u8 0  u32                    FUN_142835840
  u32                          FUN_1428358a0
  u32   0                      count
  u32   0                      count
```

508 bytes with empty strings and no equips. **Send no mount/vehicle equip** or §3 applies.

Order of work, cheapest first:

1. **Build `0x0224` and `0x0225` in `crates/net/`, reusing `avatar_look()` unchanged.** A
   byte-length test against 508 + strings + 5·equips is worth having, because a short body here
   is the failure that has killed this client twice.
2. **On field entry, send one `0x0224` per *other* character in the map, and skip our own id.**
   Self-addressed is harmless but pointless (§5).
3. **On leave, send `0x0225` with the id.** One u32.
4. **Do not re-send `0x0224` to update a user** — it is a no-op (§5). Leave-then-enter, or a
   remote opcode from `0x293..0x2c4`.
**Companion document.** `research/user-pool-tables.md` was written the same day by a sibling
agent and enumerates the `0x226..0x292` and `0x293..0x2c4` dispatch tables — the remote *move*
and *attack* opcodes, i.e. what you need once a second character is on screen. It does not touch
`0x0224` or `0x0225` and nothing in it is contradicted here; its raw table dump
(`research/msexe-userpool-tables-raw.txt`) is what my independent walk of `0x1429bbc34` in §6
reproduced entry for entry. I have not otherwise checked its claims.

5. **The first client run should carry a watch on `0x1429ba60b`** (the `mov edx,0x40` allocation,
   past every gate in §5) with `140304100:hits=200` as the positive control, per
   `research/user-chat-round2.md` §9's slot budget. If `1429ba60b` never fires, the answer is one
   of the six gates and no amount of body work will help.
