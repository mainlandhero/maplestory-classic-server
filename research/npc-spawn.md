# How an NPC gets onto a field, and what the server must send

Written 2026-08-19. **No Ghidra** (the project was locked by the main session) and **no
client run**. Everything below comes from the PE itself, read with `tools/xref.py`,
`tools/callers.py`, `tools/dataref.py`, `tools/dispatchers.py` and three throwaway scripts,
plus the decompilation and case tables already in `research/`.

Markers: **[L]** read out of the image or a file on disk, **[D]** derived from those,
**[I]** inferred / candidate.

> One environment change: `capstone` was installed with `python -m pip install --user
> capstone` so the raw bytes could be disassembled without Ghidra. Everything marked [L]
> below is a disassembly of bytes in `client-patched\MapleStory.exe`, and every address is
> given so it can be re-checked.

---

## 0. Answer up front

| question | answer |
|---|---|
| **Does the client spawn NPCs itself from the map WZ's `life` nodes?** | **No.** [L] |
| **Does it wait for a server packet?** | **Yes.** The NPC pool is populated only from packets. [L] |
| the inbound opcode | **`0x044F`** — `NpcEnterField`. Also `0x0451` with its flag byte set, which carries the identical body. [L] |
| where it is dispatched | `CField::OnPacket` = **`FUN_141820080`**, which routes `0x44F..0x468` to the NPC pool **`FUN_141e75800`** on singleton `DAT_143aa84f8`. [L] |
| body size | **64 bytes** for a fully-zeroed minimum, with a trailing empty string. [D] |
| what map 1 needs | two `0x044F` packets, one per NPC, after `SetField`. Concrete bytes in §5. |

**What the client does do with `life`:** it walks it twice while loading the field and
**preloads WZ resources only** — `Npc/%07d.img` for `type == "n"` entries, `Mob/%07d.img`
for `type == "m"` entries. The field-load walk reads no NPC geometry at all, and neither
walk touches the NPC pool. (A third walk, in the **minimap**, does read NPC `x`/`y` — see
§2, it matters for reading the next run.) That is exactly the "reads `life` for
layout/preload but instantiates from packets" case the brief asked about, and it is settled
rather than assumed. Working in §2.

---

## 1. The decisive evidence: the only way to create a CNpc needs a CInPacket

### 1.1 The pool, and the one function that fills it

`FUN_141e75800(pool, opcode, packet)` is the NPC pool's `OnPacket`. Its `0x44F` case
(`0x141e7598b`) is, instruction for instruction: [L]

```
141e7598b  mov  rcx, rbx                 ; the packet
141e7598e  call 1406e8c20                ; u32  -> objectId          (esi)
141e75993  mov  r8,[rdi+8]               ; the pool's bucket array
      ... hash lookup: id % [rdi+0x10], compare [entry+0x10] == id ...
             found    -> 141e75a3c: mov rax,[entry+0x18]
                         141e75a45: or byte [obj+0x38], 1 ; exit  (no further reads)
             not found:
141e759c7  lea  rcx,[rdi+0x20]
141e759cb  call 141e77060                ; allocate a pool entry
141e759d6  call 1406e8c20                ; u32  -> templateId
141e759df  call 141e77b70                ; get NPC template  (Npc/%07d.img, String/Npc.img)
141e759f5  call 141e76e90                ; attach it to the new object
141e759fa  mov  byte [rbp+0x38], 1       ; "in field"
141e75a12  call 141e77160                ; insert into the pool map, keyed by objectId
141e75a35  call 141e36b20(obj, objectId, packet)   ; read the rest of the body
```

`FUN_141e77b70` is the template getter: it is one of only four functions that reference
`Npc/%07d.img`, and it also references `String/Npc.img`. [L]

### 1.2 The negative, with its instrument verified

`FUN_141e36b20` is the only function in the client that fills a CNpc's position, foothold
and ranges, and **it takes a `CInPacket*`**, so it cannot be driven from WZ data. [L]

| check | result | control |
|---|---|---|
| `tools/callers.py 0x141e36b20` | **4** call sites, all inside `0x141e75800..0x141e762b0`, the NPC pool's own packet code | non-empty result, and it recovered the site at `0x141e75a35` that was read by hand above |
| `tools/xref.py --va 0x141e36b20` | 0 — its address is never taken, so no vtable/indirect route | the same tool returns hits for other addresses in this session (`Map/Map/Map%d/%09d.img`, 8 refs) |
| `tools/callers.py 0x141e77060` (allocate pool entry) | 5, all in the same block | as above |
| `tools/callers.py 0x141e77160` (insert into pool map) | 5, all in the same block | as above |
| `tools/dataref.py 0x143aa84f8` (the pool singleton) | 87 references; **1 real write**, at `0x141e75022`, the pool constructor | the tool's own read/write/test forms; `--writes` narrows to 3 |
| of those 87, is the field loader among them? | **no** — `FUN_1418224c0`, `FUN_141b7c960` and `FUN_141ba5510` are all absent. The only field-side reader is `FUN_141820080` at `0x141821e97`, the routing site | the grep found `FUN_141820080`, so it was capable of matching |

So: **no code that reads the map WZ can reach the NPC pool.** [D]

---

## 2. What the client really does with `life`

Both walks are in the field-load chain `FUN_1418224c0` -> `FUN_141b7c960`, and both are
resource preloads. Found by dumping every string each function reaches, following one level
of the client's `PTR_` indirection (plain `xref.py --string` misses that form — see §7).

**`FUN_141b7c960`** (6022 bytes) is the map-image loader. Strings it reaches: [L]
`foothold`, `ladderRope`, `quarterView`, `colorFlow`, `enterScale`, `scale`,
`zoomOutField`, **`life`**, **`type`**, **`id`**, `groupName`, `sideType`, `reactor`,
`footprintData`.

Its `life` loop ends in: [L]

```
141b7d92a  cmp  word [rcx], 0x6e        ; type == u"n"
141b7d92e  jne  141b7d99f               ; not an NPC -> skip
141b7d930  cmp  word [rcx+2], 0         ; ...and exactly one character
141b7d939  mov  ecx, [rbp-0x28]         ; the life entry's id
141b7d93c  call 141e77b70               ; load Npc/%07d.img for it
```

That is the whole of it: for every `type == "n"` entry it loads the NPC's WZ template and
stops. No position, no pool, no object.

**`FUN_141ba5510`** (2395 bytes), called from the same loader at `0x141b7d0ae`, is the mob
half. It reads `life`, `isCategory`, `type`, `id`, `x`, `y`, `cy`, `fh` — but: [L]

```
141ba5828  lea  rdx,[rip+...]           ; -> u"m"
141ba5833  call [rip+...]               ; string compare
141ba583b  jne  141ba5dc3               ; type != "m" -> skip the entry entirely
141ba5841  ... reads id, x, y, cy, fh ...
141ba590d  call 140495990               ; -> Mob/%07d.img
```

`FUN_140495990` is the `Mob/%07d.img` template loader. [L] So the geometry it reads is for
mob **generators/preload**, and NPC entries never reach it.

**One more `life` walk, and it is worth knowing about before the run.**
`FUN_14246e870` is the **minimap** (`miniMap`, `hideMinimap`, `miniMapOnOff`, `width`,
`height`, `centerX`, `centerY`, `mag`) and it *does* read `life` entries with `type == "n"`,
including `hide`, `x`, `y` and `id`. [L] It never touches the NPC pool
(`tools/dataref.py 0x143aa84f8` does not list it, and it calls nothing in `0x141e3…`/
`0x141e7…`). [L]

> So the client may well draw **minimap markers for Heena and Sera from the WZ even while
> the field itself is empty** — which is a useful discriminator, not a contradiction. If the owner
> reports NPC dots on the minimap but no NPCs on screen, that is precisely the picture this
> document predicts for a server that has sent no spawn packets.

**Consequence for the negative that started this task:** the `life` reads that a string scan
turns up are real, and they are *not* evidence of client-side spawning. `mobTime`, `rx0` and
`rx1` do not exist as literals anywhere in the readable sections at all [L] — which is
consistent, because in this protocol `rx0`/`rx1` arrive in the packet as integers, not as WZ
property names.

---

## 3. Where `0x044F` is dispatched — a dispatcher the project had not found

`research/msexe-gamestage-dispatch.md` accounted for `0x00..0x39a` and left `0x1a4..0x5ab`
unexplained. That range belongs to **`FUN_141820080`**, and it is the game stage's
`OnPacket`. Evidence: [L]

* it holds the literal `"Field Skill Id = %ld, Field Skill Level = %ld"`;
* its address appears in **21 qword slots in `.rdata`**, i.e. it is a virtual method of 21
  classes — found by scanning the image for its qword, which is a form `xref.py --va`
  cannot see (it reports 0). For comparison the login stage's `FUN_141b25f30` appears in
  exactly **1** (`0x1433fd7a0` = slot 76 of vtable `0x1433fd540`, the project's existing
  identification) and the base `FUN_142097ee0` in **4**. I did **not** establish that the
  21 slots are the *same* slot index — walking back to a vtable start is unreliable here
  because the tables are contiguous in `.rdata` — so treat "same virtual slot" as [I] and
  the chaining below as the real evidence;
* it chains to the base `CStage::OnPacket` **`FUN_142097ee0`** for `0x1a0..0x1a3` at
  `0x141821fee` and to `FUN_141b82b00` for `0x51..0x6f` at `0x141822007` — the same two
  chains the login stage's `default` makes, from the other side;
* `FUN_1418224c0`, the field loader from §2, is the very next function in the image.

Its prologue, decoded byte for byte: [L]

```
1418200be  cmp  edx, 0x4b3       ; opcode
1418200c4  jg / je  ...          ; two special paths above the block
1418200d0  lea  eax,[rdx-0x1a4]
1418200d6  cmp  eax, 0x7f
1418200d9  ja   141821e24        ; -> the range chain below
1418200e8  mov  eax,[__ImageBase + rax*4 + 0x1822158]   ; 128-entry jump table
```

so **`0x1A4..0x223` is a dense 128-case switch inside the field object itself**, and
everything else falls into a chain of range tests at `0x141821e24`: [L]

| opcodes | handed to | singleton |
|---|---|---|
| `0x1A4..0x223` | `FUN_141820080`'s own switch, table `0x141822158` | — |
| `0x224..0x39F` | `FUN_1429b9300` | `[0x143AC1B90]` |
| `0x3A0..0x3C5` | `FUN_1420F8A20` | `[0x143ACF078]` |
| `0x3C6..0x44E` | `FUN_141D30E80` | `[0x143ABFE00]` |
| **`0x44F..0x468`** | **`FUN_141E75800`** | **`[0x143AA84F8]` — the NPC pool** |
| `0x469..0x46D` | `FUN_14290C44E` | (static) |
| `0x46E..0x46F` | `FUN_1417A1C30` | `[0x143ACE240]` |
| `0x470..0x472` | `FUN_141C33F20` | `[0x143ACF080]` |
| `0x473..0x475` | `FUN_140D42ED0` | `[0x143AAA1C0]` |
| `0x476..0x477` | `FUN_142141E50` | `[0x143ACC528]` |
| `0x478..0x48C` | `FUN_141F2C0A0` | `[0x143ACF070]` |
| `0x48D..0x48F` | `FUN_1419C0CE0` | `[0x143ACF0B8]` |
| `0x55B..0x55C` | `FUN_141F6F320` | `[0x143ACEDB0]` |
| `0x55D..0x55E` | `FUN_141FA66C0` | (static) |
| `0x5F1..0x5F3` | `FUN_1419FFFF0` | (static) |
| `0x1A0..0x1A3` | `FUN_142097EE0` — `SetField`'s handler, the base class | — |
| `0x51..0x6F` | `FUN_141B82B00` | — |

The `0x3C6..0x44E` block is 137 opcodes on one singleton immediately below the NPC pool —
that is the **mob pool** [I], and it is where mob spawning will live when it is needed.

### 3.1 The NPC pool's own opcodes

`FUN_141e75800` decodes the opcode with a `sub`/`je` ladder, then a 20-entry jump table at
`0x141e75cb8`: [L]

| opcode | what the handler does | candidate name |
|---|---|---|
| `0x044F` | `u32 id`; if present, `or [obj+0x38],1` and stop; else allocate, `u32 templateId`, load template, insert, then `FUN_141e36b20` | **NpcEnterField** |
| `0x0450` | `u32 id`; `and [obj+0x38],0xFE`; if the byte is now zero, unlink and free | **NpcLeaveField** |
| `0x0451` | `u8 flag; u32 id;` if `flag == 0` clear bit 1 and detach; **if `flag != 0` and the id is unknown: allocate, `u32 templateId`, load template, `mov byte [obj+0x38],2`, insert, then the same `FUN_141e36b20` body** | **NpcChangeController** (a.k.a. SpawnNpcRequestController) |
| `0x0452` | `u32 v`; sets global `DAT_143ad2d30 = (v != 0)` and walks every NPC in the pool | a global show/hide toggle |
| `0x0453..0x0466` | `u32 id`, find the NPC, then a 20-case switch — per-NPC commands (move, emotion, script, …) | `BEGIN_NPC`-block |
| `0x0467` | `u8 count`, then `count` template ids, each fed to `FUN_141e77b70` | a template preload list |
| `0x00BE` | `u8 count`, then `count` × `u32`, appended to a vector at `pool+0x40` | a "limited NPC" id list |

`0x0468` is routed to the pool by `CField::OnPacket` but matches no case inside it, so it
falls to the common exit and does nothing. [L]

`0x00BE` is the odd one out: it is **not** in the field block. It arrives through the
*channel* dispatcher `FUN_142cbaa80`, `case 0xbe`, which does
`if (DAT_143aa84f8 != 0) FUN_141e75800(DAT_143aa84f8, 0xbe, param_3);`
(`research/msexe-gamestage-dispatch.c:342-346`) [L].

The **structure** — enter / leave / controller, then a per-object block keyed by id, then a
template block — matches the v214 reference's `BEGIN_NPCPOOL 0x340 … END_NPCPOOL 0x355`
one for one. The **numbers** do not: mscw's block starts at `0x44F`, an offset of `+0x10F`,
where the field block's offset is `+0x3A`. So the reference corroborates the *shape* and is
worthless for the *value*; every number above is read out of mscw. [D]

---

## 4. The body of `0x044F`, field by field

Two `u32`s are read by the pool handler, the remaining 20 fields by
`FUN_141e36b20(npc, objectId, packet)`.

**All 20 reads are mandatory.** Verified mechanically: the function has exactly one `RET`,
and removing any single read from the control-flow graph makes that `RET` unreachable. The
same walk proves **address order is execution order** — for every pair (A before B in
address order), deleting A makes B unreachable, so no branch can reorder them. [D] The walk
followed 0 indirect branches, i.e. there were none to miss.

| # | read | stored / used | meaning |
|---|---|---|---|
| 1 | `u32` | the pool map key | **objectId** — the NPC's runtime id in this field [L] |
| 2 | `u32` | `FUN_141e77b70(v,0)` -> `Npc/%07d.img` | **templateId** — the WZ NPC id [L] |
| 3 | `u16` -> `movsx` | `[npc+0x3f0]`, then `FUN_1409d4040(npc+0x3c0, npc+0x3f0)` | **x**, signed [D] |
| 4 | `u16` -> `movsx` | `[npc+0x3f4]`, same call | **cy** (the y the sprite stands on), signed [I] |
| 5 | `u32` | `[npc+0x5a8]` | unknown |
| 6 | `u32` | `[npc+0x5ac]` | unknown |
| 7 | `u8` | `[npc+0x1ac] = (v != 0)` | a bool; read back at `0x141e36d42` to pick an init mode. **`f` (facing)** [I] |
| 8 | `u8` | `[npc+0x1a0]`, then passed as the action index into the animation setup at `0x141e36dd0` | an action / move-action index [I] |
| 9 | `u16` -> `movsx` | `FUN_142df6c50(DAT_143ac18d8, v)` — a hash-map lookup, key at `+0x10`, value at `+0x18`, **returns null on a miss and the caller does not check** | **fh**, a foothold id looked up in the field's table [D] |
| 10 | `u16` -> `movsx` | `[npc+0x174]` | **rx0** [I] |
| 11 | `u16` -> `movsx` | `[npc+0x178]` | **rx1** [I] |
| 12 | `u16` -> `movsx` | `[npc+0x17c]` | a second range low [I] |
| 13 | `u16` -> `movsx` | `[npc+0x180]` | a second range high [I] |
| 14 | `u8` | `[npc+0x270]` | unknown |
| 15 | `u32` | `FUN_141e51de0(npc, v)` | unknown |
| 16 | `u32` | `[npc+0x4a4]` | unknown |
| 17 | `u8` | `[npc+0x4a8]` | unknown |
| 18 | `u32` | `[npc+0x4ac]` | unknown |
| 19 | `raw[8]` | `[npc+0x4b4]` | **exactly 8 bytes** [D] |
| 20 | `u32` | `[npc+0x520]`; **`0` means "use the template default"** (`[template+0x2a4]`) | a style/state id [D] |
| 21 | `u32` | only used when `[npc+0x294] == 0 && v != 0` | unknown, safe at 0 [D] |
| 22 | `str` | if non-empty, a further block runs | a name/script override [I] |

Pairs 10/11 and 12/13 are passed to `FUN_142b53fc0` as two separate arguments
(`lea rsi,[r12+0x174]` and `lea rax,[r12+0x17c]` at `0x141e36c25` / `0x141e36d5c`), which is
why they read as two ranges rather than four scalars. [D]

**Read 19's length is 8, and that is read rather than guessed.** `mov r8d, edi` at
`0x141e38b07` supplies it; every path to that instruction passes through `mov edi,8` at
either `0x141e37e66` or `0x141e37ee6` (proved by the same avoid-walk), and nothing writes
`edi` between them and the call. [D]

### Size

```
4  objectId          4  templateId
2  x                 2  cy
4  ?                 4  ?
1  f                 1  action
2  fh                2  rx0        2  rx1        2  ?        2  ?
1  ?
4  ?   4  ?   1  ?   4  ?   8  raw   4  ?   4  ?
2  string length = 0
--------------------------------------------------------------
64 bytes of body, minimum
```

**`0x0451` differs only at the front**: `u8 flag; u32 objectId;` and then, when `flag != 0`
and the id is new, `u32 templateId` followed by the identical 20-field body — 65 bytes. [L]

---

## 5. The minimum viable spawn for map 1

Map 1's `life`, dumped with the repo's own `wz-dump`: [L]

| idx | type | id | x | y | fh | cy | rx0 | rx1 | f |
|---|---|---|---|---|---|---|---|---|---|
| 0 | n | 0000001 (Heena) | -46 | 280 | 66 | 305 | -64 | -26 | 1 |
| 1 | n | 0000002 (Sera) | 833 | 125 | 8 | 125 | 783 | 883 | - |

### Which fields must be real

| field | must be real? | why |
|---|---|---|
| **objectId** | **yes, and unique per NPC** | it is the pool's hash key. A repeat makes `0x044F` take the "already present" path, `or [obj+0x38],1`, and **return without reading the rest of the body** [L]. Use anything distinct; the value itself is not checked. |
| **templateId** | **yes** | it goes straight to `Npc/%07d.img`. On a miss the object at `[npc+0x28]` stays null, the client fires assert `0x431` and then calls `FUN_141e36b20` anyway [L] — an untested path. Use 1 and 2, which exist. |
| **x, cy** | yes | this is where the NPC is drawn. Sign-extended, so `-46` goes on the wire as `D2 FF`. |
| **fh** | **use the real value** | it is looked up in the field's foothold map and the result is passed on without a null check. A miss yields null, which is passed by value and not dereferenced at that site, so `0` is probably survivable [I] — but map 1 gives 66 and 8 for free, so there is no reason to find out. |
| **f (#7)** | send the WZ value | 1 for Heena, 0 for Sera. Worst case they face the wrong way. |
| **rx0, rx1** | send the WZ values | free, and they bound the NPC's wander. |
| everything else | **zero** | 4 `u32`s, 3 `u8`s, 2 `u16`s, the 8 raw bytes and an empty string. Read 20 explicitly treats 0 as "use the template's default", which is the behaviour we want; read 21 is only consumed when non-zero. The rest have no readable consumer on the entry path. |

### The two packets, byte for byte

Opcode `0x044F`, little-endian, 64-byte bodies. **[D]** — assembled from the layout above,
not captured.

```
Heena  (objectId 1000, template 1)
  E8 03 00 00                      objectId  = 1000
  01 00 00 00                      template  = 1
  D2 FF                            x         = -46
  31 01                            cy        = 305
  00 00 00 00  00 00 00 00         (reads 5,6)
  01                               f         = 1
  00                               action
  42 00                            fh        = 66
  C0 FF                            rx0       = -64
  E6 FF                            rx1       = -26
  00 00  00 00                     (reads 12,13)
  00                               (read 14)
  00 00 00 00  00 00 00 00         (reads 15,16)
  00                               (read 17)
  00 00 00 00                      (read 18)
  00 00 00 00 00 00 00 00          raw[8]
  00 00 00 00                      (read 20 -> template default)
  00 00 00 00                      (read 21)
  00 00                            string, length 0

Sera   (objectId 1001, template 2)
  E9 03 00 00                      objectId  = 1001
  02 00 00 00                      template  = 2
  41 03                            x         = 833
  7D 00                            cy        = 125
  00 00 00 00  00 00 00 00         (reads 5,6)
  00                               f         = 0
  00                               action
  08 00                            fh        = 8
  0F 03                            rx0       = 783
  73 03                            rx1       = 883
  00 00  00 00                     (reads 12,13)
  <32 zero bytes>                  reads 14-21 (1+4+4+1+4+8+4+4) and the empty string (2)
```

**A built-in discriminator, worth knowing before the run.** Read 4 is `cy` or `y`; the
client stores it as the sprite's ground line, which is `cy` in every version this project
has looked at, but that is [I]. Sera cannot tell them apart (`y == cy == 125`); Heena can
(`y = 280`, `cy = 305`). So: if Sera looks right and **Heena floats about 25 px above the
ground**, read 4 is `y` and the fix is one field.

---

## 6. What has to happen first

### 6.1 The pool is destroyed and rebuilt on every field entry

`FUN_142caa4e0` — the world object's field-entry reset, already decompiled in
`research/msexe-setfield-aftermath.c` — does, at line 202-204: [L]

```c
FUN_142d39650(param_1 + 0x525);      // release the old NPC pool holder
lVar9 = FUN_142d33f40(0);            // construct a new one
param_1[0x526] = lVar9;
```

`FUN_142d33f40` is the only caller of the pool constructor at `0x141e75010`, and that
constructor is what writes `DAT_143aa84f8` [L]. So **`world[0x526]` is the NPC pool** — that
is the sub-manager the brief suspected — and it comes back **empty** every time a field is
entered. The server must send every NPC after every `SetField`. Nothing is remembered.

### 6.2 The ordering signal already exists in the logs

The same `FUN_142caa4e0` builds and sends outbound **`0x0238` and `0x024D`** at lines
488-490 of that file [L] — the two empty packets STATUS records the client sending 422 ms
after it accepted `SetField`. The pool is created at line 203, ~285 lines earlier in the
same function. So:

> **When `0x0238`/`0x024D` appear in `world.log`, the NPC pool exists and is empty.**
> That is the moment to send the spawns. [D]

### 6.3 Too early is silent, not fatal

`0x044F` reaches the pool only through `CField::OnPacket`, which is the *current stage's*
virtual `OnPacket`. While the client is still in the login stage — which is where it is when
it handles `SetField` — that slot holds `FUN_141b25f30`, whose `default` is

```c
if (iVar4 - 0x1a0U < 4)        FUN_142097ee0(...);
else if (iVar4 - 0x51U < 0x1f) FUN_141b82b00(...);
```

`0x44F` matches neither, so it **falls off the end and is dropped without a word** — no
dialog, no error, no reply. [L, `research/msexe-gamestage-dispatch.md`] A spawn sent before
the stage transition simply never happened. That is a benign failure mode, but it is
indistinguishable from a wrong packet, which is why the `0x0238` trigger is worth using.

### 6.4 Nothing else appears to be required

* **No field-contents packet is needed first.** The pool handler's `0x044F` case has no
  entry guard, no latch, and no "field ready" test — it allocates on the spot. [L]
* **No acknowledgement is sent back.** Neither `FUN_141e75800`, `FUN_141e36b20`,
  `FUN_141e75aa0` nor `FUN_141e75f50` calls the packet builder `FUN_1406ed520` or the
  sender `FUN_1415d01c0`. The control for that scan is `FUN_142caa4e0`, which calls both
  and shows up immediately — so the instrument speaks. [L] Indirect sends through a deeper
  callee were not ruled out.
* **No controller packet is needed.** `0x044F` sets the in-field bit and fully initialises
  the object by itself; `0x0451` is an alternative that sets a *different* bit (2) with the
  same body. Sending `0x044F` alone is the smaller experiment. If nothing is drawn, sending
  `0x0451` with `flag = 1` afterwards is safe — the create path there first looks the id up
  and takes a different branch when it already exists. [L]
* `0x0467` (a template preload list) and `0x00BE` (a "limited NPC" id list) are optional;
  neither creates anything. [L]

### 6.5 What I did not establish

* The meaning of reads 5, 6, 14-18 and 21-22. They are sent as zero on the reasoning that
  they have no readable consumer on the entry path — **that is the same class of argument
  the `setfield-zero-audit` was written about**, and it is weaker than a measurement.
* Whether any of the 128 field-level opcodes in `0x1A4..0x223` must precede a spawn. I did
  not decode that switch. Nothing in the NPC path consults a flag those could set, but I did
  not enumerate them.
* Whether the client renders an NPC it does not control. Classic clients do; not checked
  here.
* Whether read 4 is `cy` or `y` (§5 gives the experiment that settles it).

---

## 7. Instrument notes — two things a future session should not re-learn

### 7.1 `tools/xref.py --string` misses most of this client's literals

The client reaches most strings through a qword slot in `.data` (Ghidra names them
`PTR_u_..._143a45d78`), not with `lea` at the literal. `xref.py --string
"Map/Map/Map%d/%09d.img"` returns **2** references and does *not* include
`FUN_1403999e0`, the one the project already knows about. Following one level of qword
indirection returns **8**, including it. Every scan built on `--string` alone is short by
whatever fraction of its callers use the pointer, and it fails quietly.

The three scripts used here (find-string-and-its-pointer-slots; dump-every-string-a-function-
reaches; recover-a-jump-table-by-value) are small and were kept in the session scratchpad
rather than added to `tools/`. Say the word and they can be committed.

### 7.2 `research/msexe-gamestage-cases.txt` under-reports 11 cases, and one of them is the NPC pool

`tools/switch_cases.py` computes `inside = (depth == want)` and then skips any line where
that is false — so **every statement nested inside an `if`/`for`/`while` within a case body
is discarded**. Case `0xbe`'s entire body is inside `if (DAT_143aa84f8 != 0) { ... }`, so the
table says `0x00be  INLINE (no calls)` when the source plainly reads
`FUN_141e75800(DAT_143aa84f8,0xbe,param_3);`.

Re-extracting with the depth test relaxed reproduces the same **273 labels** — the label set
is right — but corrects 11 shapes:

| opcode | table says | actually calls |
|---|---|---|
| `0x00b3` | (no calls) | `thunk_FUN_1406e8c20`, `FUN_1406e9050`, `FUN_142d06960` |
| **`0x00be`** | (no calls) | **`FUN_141e75800`** — the NPC pool |
| `0x010a` | (no calls) | `FUN_14227d660` |
| `0x010d` | (no calls) | `thunk_FUN_142e31720` |
| `0x010f` | (no calls) | `FUN_1424efd70` |
| `0x0137` | (no calls) | `FUN_14019b600`, `FUN_142cfb470` |
| `0x0146` | (no calls) | `FUN_1428f4eb0` |
| `0x014d` | (no calls) | **`FUN_140304b20`, `FUN_1402fa9a0`** — the character-record decoder and the presence-gate helper |
| `0x015b` | (no calls) | `FUN_1406e8c20`, `FUN_141f6c630` |
| `0x017f` | (no calls) | `FUN_14114d820` |
| `0x0183` | (no calls) | `FUN_1406e8c20`, `FUN_140fcd220` |

Two knock-on points. The regex `\bFUN_[0-9a-f]+\(` also cannot match `thunk_FUN_...`
(the `\b` fails against the preceding underscore), so thunked calls are invisible on top of
the nesting bug. And `0x014d` calling `FUN_140304b20` means **there is a second inbound
packet carrying the character record** — worth knowing next time that decoder is under
discussion.
