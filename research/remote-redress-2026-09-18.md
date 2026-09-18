# Redressing another player's copy IN PLACE: `0x02AE` is the packet, and `0x0138` never could

**2026-09-18. Static, plus counts from the 14:07 run's logs. One Ghidra headless run** (output
`research/msexe-remote-redress.c`); everything else from `tools/listing.py`, `tools/reads.py`,
`tools/callers.py`, `tools/rangescan.py`, `tools/dis_at.py`, `tools/pdata_lookup.py`, run with the
repo as the working directory. Nothing under `crates/` or `tools/` was touched.

Tags are the project's: **[L]** read off this client's listing, raw bytes or its own logs ·
**[D]** derived from two or more [L] · **[I]** inferred.

---

## 0. The answer

| question | answer |
|---|---|
| is there an inbound packet that re-dresses an EXISTING remote `CUser` in place | **Yes: `0x02AE`**, remote-user table C, handler `FUN_1429d5290(user, packet)`. With flag bit 0 set it decodes the compact avatar look **straight into `user+0x130`** (the user's own `AvatarLook`) and calls **`FUN_140f80200(user+0x100, 0, 0, 0, 0)`**, the avatar rebuild. That is the identical pair of operations `CUser::Init` (`FUN_14276c240`) performs to dress a fresh remote user at `0x14276c389`/`0x14276c3a8`, and the identical pair the cash-shop preview widget performs through `FUN_140f80140` to re-dress its preview `CUser`. **[L]** for every address; "it redraws" is **[D]** - never on a wire with a real body. |
| body | `u32 charId` (router) · `u8 flag` · `[bit0] compact look` · `[bit1] u8` · `[bit2] u8` · `u8 couple(0)` · `u8 friend(0)` · `u8 marriage(0)` · `u32 -> user+0x2e08` · `u32 -> user+0x4358`. **211 + 5·equips bytes with `flag = 1`.** 18 read sites, `reads.py` and the decompiler agree. **[L]** |
| what `FUN_14276c240` is | `CUser::Init`-shaped: it stores the look and rebuilds the avatar in its first 90 bytes, then spends 7 KB creating the user's Gr2D layers (name tag etc.). Its third argument is a `ZRef<IUnknown>*` handed to the avatar's layer setup; the remote init passes **a null ref** (`r15 = 0`). Not a redress routine - a construction step. **[L]** |
| what `user+0x1200` is | the head of the user's **summoned map** (`ZMap<id, ZRef<CSummoned>>` at `user+0x11e0`, 31 buckets, count `+0x11fc`, list head `+0x1200`, tail `+0x1208`). `FUN_142797be0` walks it and re-dresses each summoned's *own* avatar at `summoned+0x480`, gated on `FUN_1407f5ce0(summoned+0x30c)` = `xor eax,eax; ret`. It never touches `user+0x100`/`+0x130`. **[D]** for "summoned" (see §3), **[L]** for the walk. |
| why the opened `0x0138` drew nothing at 14:07 | **both** readings at once: the map was empty (the server has never sent a summoned-pool packet; 0 `0x03Ax` in that run's log), **and** even a non-empty map would only redress the summons' avatars, never the player's. The `avatarmod` hook patch is inert by construction and can be retired. **[L]** |
| does `0x0224` have an update branch for an id already in the pool | **No.** Hash hit → `add rax,0x18 / jne 0x1429ba955` (the epilogue). The condition is purely "key present in `pool+0xf8`"; no body flag, no state on the existing user is consulted. **[L]** |

---

## 1. `FUN_14276c240(user, look, ZRef* parent)` - `CUser::Init`, not a redress **[L]**

Listing `0x14276c240..0x14276df12` (7378 bytes). The first 0x170 bytes, read straight down:

```text
14276c28f  mov dword [rcx+0xf58], 0x64            user+0xf58 = 100
14276c299  lea r12, [rcx+0x100]                   r12 = the avatar sub-object
14276c2ac  mov rcx,[r8] ; ... call [rax+8]        *param_3 AddRef'd (IUnknown +8)
14276c2c9  lea rcx,[rsi+8] ; call 1409397b0       the user's own layer ref at user+8
14276c2f3  call 140f98f40(avatar, layer, 0, 0, &ref)     attach the avatar to the layer
14276c2ff  mov rcx,[r13] ; AddRef                 *param_3 again
14276c316..362  user+0xe58, +0xe08, +0xe00 AddRef'd
14276c384  call 141a42900([user+0x34f0], &e00, &e08, &e58, &ref)
14276c389  lea rcx,[rsi+0x130] ; mov rdx,rbx ; call 1402eeb50     COPY the look -> user+0x130
14276c3a8  call 140f80200(avatar, 0, 0, 0, 0)     REBUILD the avatar from user+0x130
14276c3b2  call 1427e87e0(user, 0)                alpha/visibility pass (walks the summoned map)
14276c3c7  GetUser(pool, [user+0x1264] ?: [user+0x1260]) -> 1427e87e0(that, 0)   driver/passenger
14276c3e2..  IWzGr2D->CreateLayer (vtable+0x168) into user+0x1530, z-order, canvases,
             FUN_140f08df0(user+0x1248), FUN_1428783a0(user+0x3488), FUN_1428336c0(user) ...
```

* **`user+0x100` is the avatar object and `user+0x130` is its `AvatarLook`.** Three independent
  sightings: `FUN_140f80130` is `lea rax,[rcx+0x30]; ret` and every caller passes `user+0x100`
  (`0x1429d55c1`); `FUN_140f80140` copies its look argument to `this+0x30`; and
  `FUN_140f80200`'s change test is `memcmp(avatar+0x213, avatar+0x30, 0x1e3)` - the previous
  look against the current one, 483 bytes each. **[L]**
* **The third argument** is a pointer to a ref-counted COM object (`vtable+8` = AddRef,
  `+0x10` = Release). It is copied into two stack refs and passed as the fifth argument of
  `FUN_140f98f40` (avatar ↔ layer attach) and of `FUN_141a42900` (a second display object at
  `user+0x34f0`). It is a parent/reference layer **[I]**. What the three call sites pass:

  | caller | site | look | third arg |
  |---|---|---|---|
  | `FUN_1429ce270` remote init (`0x0224`) | `0x1429ce9c2` | the stack look decoded at `0x1429ce6a9` | `&[rbp-0x38]` = **`&r15`, and `r15` is zero for the whole function** (`xor r15d,r15d` at `0x1429ce2ae`, used as the zero register throughout) - a **null ref** |
  | `FUN_142886870` local user, field entry | `0x14288710e` | a look built by `FUN_1402eacf0(&look, src)` from a character record | `&[rbp+0xc0]`, a ref the caller holds |
  | `FUN_1429c13b0` local user init | `0x1429c19e6` | `FUN_1402eacf0(&look, FUN_142cbe730(ctx))` - the **local** character record | `&local_288 = *param_6`, passed in by its caller |

  All three **[L]**. So a remote user is initialised with a null parent ref, and nothing in
  the argument list identifies the user - it is `this`.

* **Everything after `0x14276c3b2` is construction**: `CreateLayer` calls through the
  `IWzGr2D` singleton `DAT_143add050`, canvas variants, a 0x130-byte and a 0x38-byte
  allocation stored at `user+0x1250` / `+0x3488`. Calling it a second time on a live user
  would replace those objects, not update them. It is the wrong routine for a redress, and
  the client never calls it for one: `tools/callers.py 0x14276c240` → exactly the three call
  sites above, 0 tail jumps, 0 data pointers. **[L]**

`FUN_1429c13b0` (2739 bytes) is the **local** user's init: it takes the character id from
`FUN_142cb9550(ctx)` (= `ctx+0x232c`, `research/user-enter-field.md` §5) and the look from
`FUN_142cbe730(ctx)`, the local record. Its five callers (`0x1410d5170`, `0x141297750`,
`0x1416141a0`, `0x1429c8330`, `0x1429c97b0`) are UI code and the pool; none is in any packet
case table (§4). Even if a packet reached it, it can only dress the local character from the
local record. **[L]**

**The vtable at `0x143486730`** holds `FUN_1429c97b0` in slot 0 and is installed by two
constructors (`FUN_1429c95a0`, `FUN_1429c96b0`, `lea` at `0x1429c95bd` / `0x1429c96c7`),
neither of which has a single call site or pointer in the image (`callers.py`: nothing found;
the documented control passes). `FUN_1429c97b0` loads `L"preview_back%d"`, `L"avatar"`,
`L"view"`, `L"weaponMotion"`, `L"alertIcon"`, constructs a `CUser` with `FUN_142768ee0`, inits
it with `FUN_1429c13b0(..., 5, ...)` and then re-dresses it with
`FUN_140f80140(previewUser+0x100, this+0x3d8, 0)`. It is the **cash-shop / beauty preview
widget**, not a field object. Its siblings `FUN_1429ca8e0`, `FUN_1429caf40`, `FUN_1429cc130`
(*"put item N on the preview avatar"*: equips `1000000..1999999`, chairs `3010000..`, pets by
item type 8) make the same `FUN_140f80140(previewUser+0x100, look, 0)` call. **[L]**

> Correction to `research/beauty-2026-09-09.md` §8.2: the "five callers of `FUN_140f80140` in
> the `0x1429c...` user-pool region, the code that dresses a remote character from a `0x0224`"
> are these preview-widget functions. The `0x0224` path never calls `FUN_140f80140`; it copies
> and rebuilds inline (`0x14276c389`, `0x14276c3a8`). The conclusion drawn from it - that
> `FUN_140f80140` is the real dress primitive - was still right: it is `copy look to
> avatar+0x30, then FUN_140f80200(avatar, bForce, 0, 0, 0)` (17 instructions, `0x140f80140..
> 0x140f80178`). **[L]**

### 1.1 `FUN_140f80200(avatar, bForce, a, b, bSkip)` is the rebuild, and it is idempotent **[L]**

`research/msexe-remote-redress.c` lines 132-380. Shape:

```c
if (bForce == 0 && memcmp(avatar+0x213, avatar+0x30, 0x1e3) == 0) return;   // look unchanged
FUN_1402eeb50(avatar+0x213, avatar+0x30);          // remember it
FUN_140f809b0(avatar, &state, look, isLocal);       // build the layers from the look
FUN_140fafed0(avatar); action = ...; FUN_140f832e0(avatar, action, -1);
if (avatar+0x5ac == 0) { if (!bSkip) FUN_140fbbe20(avatar, a, b); ... vtable+0xc0/+0x70/+0x10 ... }
```

So the call with `bForce = 0` is a no-op when the look has not changed and a rebuild when it
has. `DAT_143aa8518 + 0x20` (the local user's avatar) is special-cased only for the cash-shop
and login stages (`FUN_14209ee40()->stage` type checks at the top). 25 direct callers, among
them **`0x1429d52a1` = the `0x02AE` handler**, `FUN_14276c240`, `FUN_140f80140`.

---

## 2. `user+0x1200` and why the opened `0x0138` could never redraw a player

### 2.1 The walk, corrected **[L]**

`FUN_142797be0(user, look)` (listing 866 bytes, decompile in `research/msexe-setavatarlook.c`
lines 2928-3147) iterates `node = [user+0x1200]; next = [node-0x20] ? [node-0x20]+0x28 : 0`,
takes `obj = [node+8]` (a `ZRef`, refcount at `obj+0x18`), locks `[obj+0x38]`, and per node:

```text
142797dd2  mov rbx,[rsi+8]                obj
142797de0  mov ecx,[rbx+0x30c]            an id on the object
142797de6  call 1407f5ce0                 xor eax,eax ; ret        -> always 0
142797ded  je  142797e07                  (the avatarmod patch nops this)
142797df2  call 1420dd920(obj, look)      = if ([obj+0x480]) FUN_140f80140([obj+0x480], look, 0)
142797dff  call 1420dd220(obj, 1)         = [obj+0x480] ? set its colour from [obj+0x4c4] : -
```

`FUN_1420dd920` is a 21-byte stub outside any `.pdata` entry: `mov rcx,[rcx+0x480]; test; je ret;
xor r8d,r8d; jmp 0x140f80140`. **The apply target is the pointer at `obj+0x480`, not
`obj+0x78`** as `beauty-2026-09-09.md` §8.2 wrote - the shim loads a pointer, it does not form
an address. Whatever the object is, the code redresses *a second avatar owned by it*, and the
function contains no reference to `user+0x100` or `user+0x130` at all (`grep -c "0x130\|0x100\]"`
over its listing: 0; positive control: the same grep over `FUN_1429d5290`'s listing finds the
`0x130` and `0x100` lines quoted in §3). **[L]**

### 2.2 What the nodes are: the user's summoned map **[D]**

The map object is `user+0x11e0..+0x1210`: `+0x11e0` bucket array, `+0x11e8` bucket count
(`FUN_142768ee0`, the `CUser` constructor, writes `0x1f` = 31 there), `+0x11fc` count,
`+0x1200` list head, `+0x1208` list tail (`FUN_142934860` clears all five and frees every
node - `RemoveAll`). `tools/rangescan.py 0x11e8 0x142760000 0x1429e0000` → 34 sites, 31 of
them the same `mov ecx,[rcx+0x11e8] / div / walk bucket` lookup in small functions at
`0x142796260..0x14279ba70`. **[L]**

Who fills it: the only writer besides the constructor and `RemoveAll` is reached through
`FUN_142795fd0(user, id, ...)` (lookup-or-insert by id, one caller `FUN_1420fa150`) and
`FUN_142795c80(user, &ref)` (lookup by `ref->vtable[1]()`, callers `FUN_1420f8a20` and
`FUN_1420f92b0`). `FUN_1420f8a20` is called from **`CField::OnPacket` at `0x141821e5c` for
opcodes `0x03A0..0x03C5`** - the range between the user pool (`0x224..0x39F`, `FUN_1429b9300`)
and the mob pool (`0x3C6..0x44E`, `FUN_141d30e80`), which is the **summoned pool's** slot in
every MapleStory `CField::OnPacket` (user, summoned, mob, npc, ...). Its handlers read
`u32 ownerId` then `u32 u16 u8...` (`FUN_1420f92b0`, enter) and `u8` (`FUN_1420fa150`, leave),
and every per-opcode helper is one of the `+0x11e8` lookup functions above. The node object's
member functions live in the same region (`FUN_1420dd220`, `FUN_1420dd920`). RTTI confirms a
`.?AVCSummoned@@` type descriptor exists (`tools/rtti.py --list Summon`), and no vtable can be
recovered for any game class (the tool's own documented limit), so the name is **[D]** from the
pool order plus the per-user map, not read off a locator. `obj+0x30c` is then the summon's
skill id and `FUN_1407f5ce0` an `is-avatar-look-summon(skillId)` predicate that this build
compiled to `return 0` **[I]**.

> `research/mob-combat.md` §7.2 calls `0x3A0..0x3C5 -> FUN_1420F8A20` "the drop pool". It is
> not; the drop pool is further down the chain. Not edited here.

### 2.3 Why 14:07 drew nothing - both readings are true at once **[L]**

The run's world log (`world-ch0.log`, still live; `previous-runs/world-ch0-20260918-140320.log`
is the launch before it) has **0** packets in `0x03A0..0x03AF` and **2** `0x0224`s; `crates/net`
has no summoned-pool builder at all (`grep -rn 0x03A0 crates/net/src` → only a `mob+0x3a0`
field comment). So the observer's copy of character 215 had an **empty** summoned map, the
loop body ran zero times, and 56 µs is what an empty walk plus scope-guard teardown costs.
Had the map held a summon, the patched path would have redressed *the summon's* avatar at
`summoned+0x480` (null for every non-avatar summon), never the player's. The
`grap_stub::avatarmod` patch therefore cannot produce a redraw under any server behaviour and
can be dropped; it was measured harmless and is harmless, but it is also pointless.

---

## 3. `0x02AE` - the in-place redress, decoded in full **[L]**

### 3.1 Routing, verified from bytes

`FUN_1429bb720(pool, opcode, packet)` reads `u32 charId` at `0x1429bb745`, hashes it into
`[pool+0xf8]` (`% [pool+0x100]`, 31 buckets), walks the chain on `[node+0x10] == charId`,
takes the `CUser` as `[[node+0x20]+0x28]`, then runs the same gates `0x0224` runs
(`FUN_142cc3d80(ctx) != 0` → drop; `FUN_141bc8c60/80/a0(field)` field-mode tests). The flood
gate at `0x1429bb8ae` only skips **table B**; table C is reached either way:

```text
1429bbb10  add esi,-0x293 ; cmp esi,0x30 ; ja epilogue
1429bbb22  movzx eax, byte [0x1429bbd10 + esi]        byte index 0x1b -> 3
1429bbb2b  mov ecx, [0x1429bbcd0 + eax*4] ; jmp       slot 3 -> 0x1429bbb58
1429bbb58  mov rdx,rdi ; mov rcx,rbx ; call 0x1429d5290 ; jmp 1429bbc08
```

Read off raw bytes this session (byte `0x1429bbd10[0x1b] = 3`, dword `0x1429bbcd0[3]` →
`0x1429bbb58`; control: `0x02AD` → `0x1429bbb48` → `FUN_1429d4fd0`, the chair relay that is
confirmed on two screens). **The router, the gates and the table-C dispatch are therefore the
exact code path `0x02AD` and `0x02B2` already travel live** - the closest positive control this
packet can have without a run. **[L]**

Two consequences the chair relay already paid for (`STATUS.md`, "the chair relay killed a
client"): the lookup is the pool hash **only**, so `0x02AE` addressed to the local player's own
id returns at the lookup - send it to the *other* clients on the field, carrying the changed
character's id; and the body length must be exact, because a short body here has faulted a
client before.

### 3.2 The handler `FUN_1429d5290(user, packet)`, 888 bytes, 18 reads

`tools/reads.py 0x1429d5290 3` → 18 read sites; the decompile
(`research/msexe-remote-redress.c` lines 1-131) has 6 `u8` + 4 `raw` + 7 `u32` + 1 helper = 18.
Listing addresses:

```text
off  at          type            what
 0   1429bb745   u32             charId  (router)
 4   1429d52b6   u8              flag
     ---- flag & 1 ----------------------------------------------------------------
 5   1429d5363   compact look    FUN_1402ee8d0(user+0x130, pkt, &emptyZXString, 0)
                                 -> decoded DIRECTLY into the user's AvatarLook
     1429d5379   FUN_1429b9660(pool, user+0x1228)      drop couple record   (no-op when +0x1228 == 0)
     1429d538c   FUN_1429b9910(pool, user+0x1520)      drop friendship record
     1429d539e   FUN_1429b9de0(pool, [user+0x10d0])    drop marriage record by charId
     1429d53a6   FUN_140db8a40(user)                   refresh a list hanging off the user (69 bytes)
     1429d53bf   FUN_140f80200(user+0x100, 0, 0, 0, 0) REBUILD the avatar     <- the redress
     ---- flag & 2 ----------------------------------------------------------------
     1429d53dd   u8              -> [user+0x4070]+0x28/+0x2c/+0x30 as a ZtlSecure triple (carry-item effect [I])
     ---- flag & 4 ----------------------------------------------------------------
     1429d5412   u8              -> FUN_1427eb600(user, v)   (item effect [I]; 0x0224 row 391 feeds the same setter)
     ---- unconditional -------------------------------------------------------------
     1429d5425   u8   couple     nonzero -> raw8 (+0x1228), raw8 (+0x1230), u32 -> FUN_1429b9490
     1429d547b   u8   friendship nonzero -> raw8 (+0x1520), raw8 (+0x1528), u32 -> FUN_1429b9740
     1429d54de   u8   marriage   nonzero -> u32 u32 u32 (+0x13a8..+0x13b0) -> FUN_1429b9c80 ; zero -> the three cleared
                 (zero couple/friendship also zero their +0x1228/+0x1230, +0x1520/+0x1528)
     1429d556b   u32             -> user+0x2e08     (0x0224 body offset 412; the builder sends 0)
     1429d5579   u32             -> user+0x4358     (0x0224 body offset 183; the builder sends 0)
     1429d5587   FUN_1427e90b0(user)                   also run by FUN_1428335a0 after every 0x0224 init
     1429d558f   FUN_1428336c0(user)                   also run inside FUN_14276c240
     1429d55a1   if in my party (FUN_142dec860) and the party window exists: scan look slots 0..3
                 for an item in 1114000..1114099, refresh the window (FUN_14118f5d0)
```

**Minimum body with `flag = 1`: `4 + 1 + (195 + 5·equips) + 3 + 4 + 4 = 211 + 5·equips`.**
The look is `net::opcode::avatar_look(chr)` unchanged - same reader, same `(look, pkt, str, 0)`
argument shape as `0x0224` at `0x1429ce6a9` (the fourth argument is never tested inside
`FUN_1402ee8d0`; its only `r9` uses are two `mov r9b,1` feeding an inner call). **[L]**

Nothing in the flag-1 branch is new to a packet-built user: the three record removals are the
ones `0x0225` runs on leave (`research/user-enter-field.md` §1), `FUN_1427e90b0` and
`FUN_1428336c0` have run on every remote user this server has ever spawned, and
`FUN_140f80200` is what dressed it in the first place.

### 3.3 The one archived send is not evidence

`research/fixtures/sweep-01f2-03c7-exit.log:426` - `01:05:26 >>> opcode 0x02AE +32B` - is the
opcode sweep: 32 garbage bytes addressed to whatever id the first `u32` decoded to, from a
session with nobody else in the pool, so it returned at the hash lookup. **`0x02AE` with a real
body has never been on a wire.** Its local twin `0x0331` (`FUN_14290d4e0`, table D) also reads a
look (`0x14290d745`) and is unread here; the local redraw already works through `0x007C`
(`beauty-2026-09-09.md` §8.1) so it is not needed.

---

## 4. Enumeration: no inbound handler reaches `FUN_14276c240` or `FUN_1429c13b0` **[L]**

`tools/callers.py` on each (the documented control `0x1402fa9a0` passes):

| routine | direct callers | tail jmps | pointers | in any case table? |
|---|---|---|---|---|
| `FUN_14276c240` | 3: `0x142886870`, `0x1429c13b0`, `0x1429ce270` | 0 | 0 | only `0x1429ce270` (via `0x0224`'s `FUN_1429ba3e0`), and that is the fresh-user path |
| `FUN_1429c13b0` | 5: `0x1410d5170`, `0x141297750`, `0x1416141a0`, `0x1429c8330`, `0x1429c97b0` | 0 | 0 | none |
| `FUN_1429c97b0` | 0 | 0 | 1 (vtable `0x143486730` slot 0) | its constructors have no callers in `.text` - reached from virtualised code |
| `FUN_140f80200` | 25 | 0 | 0 | **`0x1429d52a1` = `0x02AE`'s handler** (and `0x142d5aa10`, `0x142899d40`, `0x1429c8510`, `0x1429dbe30`, not in tables) |
| `FUN_140f80140` | 23 + 5 tail jmps | | | none in a table; `0x1420dd92f` is the `0x0138` summoned shim |

Grepped against `research/msexe-gamestage-cases.txt`, `msexe-field-cases.txt`,
`msexe-userpool-tables-handlers.txt`, `msexe-loginstage-cases.c`; positive controls for the grep:
`142d012e0` (line 176 of the game-stage table) and `1429d5290` (line 497 of the pool table)
are both found.

**The `0x0224` duplicate-id check, at the listing:**

```text
1429ba523  mov r8,[r14+0xf8] ; test ; je 1429ba560        no table -> proceed to the gates
1429ba52f  mov ecx,[r14+0x100] ; div ; mov rax,[r8+rdx*8]
1429ba546  cmp dword [rax+0x10], esi ; je 1429ba556       key == charId
1429ba54b  mov rax,[rax+8] ; test ; jne 1429ba546          next in chain
1429ba556  add rax,0x18 ; jne 0x1429ba955                  <- found: ALWAYS to the epilogue
```

`0x1429ba955` restores registers and returns. The only inputs are the id and the hash; nothing
from the body (which has not been read yet) and nothing on the existing user is examined. There
is no update path. **[L]**

---

## 5. Wire it like this (not wired - nothing under `crates/` was touched)

```text
0x02AE  UserAvatarModifiedRemote            to every OTHER session on the field, per look change
  u32   charId                              the character whose look changed (must be in the observer's pool)
  u8    0x01                                bit 0: a look follows. Bits 1 and 2 add one u8 each; leave clear
  ...   avatar_look(chr)                    net::opcode::avatar_look, unchanged, 195 + 5 per worn item
  u8    0                                   couple ring
  u8    0                                   friendship ring
  u8    0                                   marriage ring
  u32   0                                   -> user+0x2e08, what 0x0224 body offset 412 already sent
  u32   0                                   -> user+0x4358, what 0x0224 body offset 183 already sent
```

* Replaces the leave + enter pair in `broadcast_look_change`: no `CUser` destruction, so no blink
  and the pet copy is untouched. The pet hat case still needs the pet's own path (a pet is not in
  this look).
* A length test asserting `211 + 5·equips` for the no-ring body is worth having; the equal-sized
  `0x02AD` at 12 bytes instead of 13 faulted a client.
* Skip the changed character's own session (the router cannot address the local player; the
  local redraw is `0x007C` / `0x0070` already).
* Retire `grap_stub::avatarmod` and stop sending `0x0138`: with the gate open it walks an empty
  summoned map; with it closed it walks nothing. Either way it applies to nothing on screen.

### 5.1 The in-process fallback, only if `0x02AE` is refuted on screen

The single call to change is `0x142d0149d call 0x142797be0` in the `0x0138` handler, reached
with `rcx = r15` (the `CUser` from `FUN_1429b6c90(pool, id)` at `0x142d01328`) and
`rdx = rsp+0x30` (the just-decoded look). The redress is
`FUN_140f80140(r15 + 0x100, rdx, 0)`, which does not fit in the 13 bytes at
`0x142d01495..0x142d014a2`; it needs the `call` retargeted to a hook-owned stub:
`lea rcx,[rcx+0x100] ; xor r8d,r8d ; jmp 0x140f80140`. That skips the record removals and the
two post-passes `0x02AE` runs, and needs a client patch the packet does not. **[D]** on the
argument sources, untested.

---

## 6. Not established

* **That `0x02AE` redraws on screen.** Every address is [L]; the claim that
  `FUN_140f80200(user+0x100, 0, 0, 0, 0)` on a look that differs from `avatar+0x213` redraws a
  *remote* user is [D] from its being the init's own dress call and the preview widget's
  re-dress call. One two-client run settles it: hair or equip change on A, watch B's copy.
  No blink and the new look = works. Blink = the server is still on `--look-reenter`. Nothing
  = the gates in §3.1 or a body-length mismatch; the hook's dispatch line for `0x02AE` on the
  observer says which (present and short = handler ran; absent = router returned).
* **What `user+0x2e08` and `user+0x4358` mean.** `FUN_1427e90b0` reads the `+0x2e0x` block
  (it loads `L"replacedStandAction"`) - a stand-action override [I]. Both are re-sent as 0, the
  value the enter carried, so the redress cannot change them.
* **The names "summoned"/`CSummoned` for the `user+0x11e0` map and "avatar-look summon" for
  `FUN_1407f5ce0`.** [D]/[I] as tagged in §2.2; the pool-order argument is strong but no vtable
  or string names the class. Nothing in §3 or §5 depends on it.
* **Whether bits 1 and 2 of the flag are safe to send.** Unread beyond the store they perform.
  Leave them clear.
* **`FUN_140db8a40(user)`.** 69 bytes: fetches something via `FUN_1427be040(user, &ref)`, and if
  `[obj+0x2b0]` is a non-empty list calls `FUN_140db56b0()`. Runs on every flag-1 `0x02AE`;
  not read further.
