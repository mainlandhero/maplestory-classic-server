# Scrolling an equip — the request, the result, and who decides

**2026-09-09.** Static pass, no client run, no Ghidra. Every address below was produced by
`tools/listing.py`, `tools/dis_at.py`, `tools/encodes.py`, `tools/reads.py`,
`tools/callers.py`, `tools/fieldrefs.py`, `tools/ripstrings.py` and
`tools/dump_stringids.py`, run with the repo as the working directory.

Labels: **[L]** measured off this client, **[D]** derived from measurements, **[I]** inferred
(including anything that came from `ModernMapleSource`, which is a different version).

---

## 0. The answers, up front

| | | |
|---|---|---|
| request opcode | **`0x0125`** | [L] |
| request body | `u32 tick; u16 srcSlot; u16 dstInvType; u16 dstSlot; u8 flag` — **11 bytes** | [L] |
| result opcode | **`0x0236`** | [L] |
| result body | `u32 charId; u8 result; u8 enchantDlg; u32 scrollItemId; u32 equipItemId` — **14 bytes** | [L] |
| `result` values | `0` fail, `1` success, `2` **destroyed**, `3` "cannot be used", anything else = a fourth arm | [L] |
| who decides success | **the server**. Nothing in the client recomputes it | [L] |
| stat change | not in `0x0236`. It comes from `0x0070` InventoryOperation, which this project already builds | [D] |
| **third packet, mandatory** | **`0x00B8`, empty body** — clears the client's request latch. Without it every later item request is silently dropped | [L] for the latch effect, [I] for it being the *intended* one — §6 |
| Clean Slate / White Scroll | **no protocol field at all.** Purely server-side policy | [L] for the absence of a field, see §7 for the named blind spot |

All 208 scroll rows in `gm-handbook/scrolls.txt` (ids `2040000`..`2048007`) route to `0x0125`.
[D], from the id-range tests in §2, which are [L].

---

## 1. How `0x0125` was found — enumeration, not a guess

`research/msexe-send-opcodes.txt` is the full enumeration of 1894 outbound builder call
sites. `0x0105`..`0x0135` is one contiguous family of builders living in `0x142cc5xxx`..
`0x142cd6xxx`. Running `tools/encodes.py <builder> 3` over every member of that family gives
each one's field sequence; three of them write `u32, u16, u16, u16, u8`:

```
0x0125  FUN_142cc7a70   u32 u16 u16 u16 u8      <- this one
0x0126  FUN_142cc7bf0   u32 u16 u16 u16 u8
0x0128  FUN_142cc7ff0   u32 u16 u16 u16         (no trailing u8)
```

`tools/callers.py` on each shows that **every** member of the family has exactly one caller,
and that fourteen of those callers are themselves called from **one** function,
`FUN_1417deab0`. That function is the drag-an-item-onto-an-item dispatcher: it gates on
source inventory type ∈ {2, 5} and target inventory type ∈ {1, 6}, fetches the source item
id, and then routes on the **item id** to one wrapper per item family (§2). [L]

`0x0125` and `0x0126` are both emitted by the wrapper `FUN_1417df1d0`, which is reached from
the dispatcher's **default** arm. That wrapper is identified beyond doubt by the string it
puts in its confirmation dialog:

```
0x1417df4ed  mov edx, 0xb2e   ->  "You've selected the %s.\r\nDo you want to use the\r\n%s on it?"
0x1417df545  mov edx, 0xb2f   ->  "\r\n(Certain scrolls cannot be used on items protected by a Guardian Scroll.)"
0x1417df9ef  mov edx, 0xbb5   ->  "You cannot use that item on this equipment."
```

[L] — string ids decrypted with `tools/dump_stringids.py`. That is the scroll dialog.

### Why the instrument is trusted here

* `tools/encodes.py 0x141cb6880 1` reproduces its documented positive control (the ctor at
  `141cb7eb1`).
* `tools/fieldrefs.py 0x2f4 --lo 0x141c40000 --hi 0x141d60000 --write` reproduces its exact
  three documented rows.
* The immediate-scan in §7 was run against two **positive controls first** (`0xBB5` at
  `0x1417df9ef` and `0xB2E` at `0x1417df4ed`, both already read out of the listing by hand)
  and found them at exactly those addresses before any negative was believed.
* **One documented control did NOT reproduce:** `tools/ripstrings.py 0x141b2c7c0 934` prints
  *"0 lea targets decoded as text"*, where its docstring says it must print at least one.
  The tool nevertheless spoke on `0x142792340` (§4), which is the positive that matters, but
  **that docstring control is stale or broken and should not be relied on by the next
  agent.**

---

## 2. `FUN_1417deab0` — the drop dispatcher, and which ids reach `0x0125`

Signature, from the call sites and from `CharacterData::GetItemSlot` (`0x1402e3770`,
which does `lea eax,[TI-1]; cmp eax,5; ja fail`, so TI is 1..6): [L]

```
FUN_1417deab0(int srcInvType, int srcSlot, int dstInvType, int dstSlot,
              int arg5, int arg6, int arg7)
```

Entry gates [L]:

* `srcInvType == 2 || srcInvType == 5` (CONSUME or CASH) else return
* `srcSlot > 0` else return
* `dstInvType == 1 || dstInvType == 6` else return
* the source item must exist; `edi = itemId(srcItem)`

`dstInvType == 6` is a real inventory type in this client: `0x1402e3770`'s `TI == 6` arm
accepts **negative** positions in the ranges `-0x4BD..-0x4B0`, `-0x73A..-0x708` and
`-0x83..-0x65`, i.e. the extended/second equip windows. `TI == 1` with a negative position is
the ordinary worn-equip path. [L]

Then a ladder of item-id range tests. Everything that falls through reaches
`0x1417df158 call 0x1417df1d0` → `0x0125`. The tests, decimal, in order: [L]

| test on source item id | goes to | opcode |
|---|---|---|
| `2539001..2539002` | return | — |
| `2049400..2049499` | `FUN_1417e1b40` | `0x012A` |
| `FUN_1417ea690(id)` — true for `2049700..2049729` and `2049740..2049749` | `FUN_1417e1b40` | `0x012A` |
| `FUN_140416420(id)` — true for `2049750..2049769`, `2049790..2049799` | `FUN_1417e1b40` | `0x012A` |
| `2049780..2049789` | `FUN_1417e1b40` | `0x012A` |
| `2644100..2644109` | `FUN_1417e1b40` | `0x012A` |
| `2570000..2579999` | `FUN_1417e3a30` | `0x0129` |
| `2460000..2469999` | `FUN_1417e4c70` | `0x0133` |
| `2048305..2048399`, `2049730..2049739`, `2049900..2049909`, `2049770..2049779` | `FUN_1417e2230` | `0x012B` |
| `2591000..2591999` | `FUN_1417e3fb0` | `0x012F` |
| `2590000..2590999` | `FUN_1417e4600` | `0x012E` |
| `FUN_140419910(id)` | `FUN_1417e64e0` | — |
| `2711024..2711025` | `FUN_1417e7190` | — |
| `2730011..2730012` | `FUN_1417e6f00` | — |
| subtype `== 0x50` | `FUN_1417e7420` | — |
| subtype `== 0x4D` | `FUN_1417e7690` | — |
| `FUN_140418e20(id)` | `FUN_1417e78e0` | — |
| `2470000..2479999` | `FUN_1417e80b0` | — |
| `FUN_14041b2d0(id)` — `2049500..2049599` or subtype `0x49` | `FUN_1417e69a0` | — |
| `2048200..2048299`, `2048300..2048304` | `FUN_1417e6aa0` | — |
| **anything else** | **`FUN_1417df1d0`** | **`0x0125` / `0x0126`** |

`gm-handbook/scrolls.txt` spans `2040000`..`2048007` and the lowest excluded range starts at
`2048200`, so **all 208 scrolls fall through to `0x0125`**. [D]

Inside `FUN_1417df1d0` the split is: [L]

* `FUN_1404174b0(scrollId, equipId)` — the applicability predicate, "can this scroll go on
  this equip". True → `0x0125`.
* False → `FUN_1417ea820(scrollId)`; that returns true only for the `2530000..2531999` /
  subtype `0x30..0x32` classes (hyper-upgrade / star-force family), and that branch sends
  **`0x0126`**. So `0x0126` is a different feature, not a white-scroll variant. [D]

One more branch worth knowing: if the source item object has a non-null field at `+0x38`,
the client sends **`0x0116`** through `FUN_142ccc3a0` (a confirmation/enchant-UI request
carrying a string) instead of `0x0125`. `+0x38` was not identified. [L] that the branch
exists, **not established** what puts a value there.

---

## 3. `0x0125` — the request, byte by byte

Builder `FUN_142cc7a70`, `tools/encodes.py 0x142cc7a70 6`: [L]

```
0x142cc7af7  CTOR  0x0125
0x142cc7b09  w_u32     <- FUN_1429e3ef0()          the client tick
0x142cc7b16  w_u16     <- arg2 (bx)
0x142cc7b23  w_u16     <- arg3 (si)
0x142cc7b30  w_u16     <- arg4 (bp)
0x142cc7b44  w_u8      <- arg5 low byte, [rsp+0x4d0]
0x142cc7b4e  SEND (0x1415d01c0)
0x142cc7bbe  ~COutPacket (0x1406ed610)
```

`research/msexe-packet-fields.txt`, which was built by an independent enumeration, agrees on
the sequence for `0x0125`. Two instruments, same answer. [L]

The call site is `0x1417dfb2b`, and `FUN_1417df1d0`'s frame is
`rbp = entryRSP - 0xB8`, so `[rbp+0xc8] = arg2`, `[rbp+0xd0] = arg3`, `[rbp+0xd8] = arg4`,
`[rbp+0xe0] = arg5`. None of those home slots is written anywhere in the function (grepped).
Therefore: [L]

```
off  size  field
  0   u16  opcode = 0x0125
  2   u32  tick             FUN_1429e3ef0() - the same "update time" every request carries
  6   u16  srcSlot          the scroll's slot in inventory type 2 (CONSUME) or 5 (CASH); > 0
  8   u16  dstInvType       1 = equip inventory / worn (negative slot), 6 = extended equip window
 10   u16  dstSlot          the target equip's slot; NEGATIVE means a worn item
 12   u8   flag             see below
                            body = 11 bytes, 13 on the wire with the opcode
```

**The middle `u16` is a target inventory TYPE, not a white-scroll slot.** Three independent
confirmations: [L]

1. the builder's register order (`bx`, `si`, `bp` = args 2, 3, 4);
2. `FUN_1417df1d0` passes arg3 as the `nTI` argument of `CharacterData::GetItemSlot`
   (`0x1402e3cd0(cd, out, TI=arg3, POS=arg4)` → `0x1402e3770`, whose first instruction is
   `lea eax,[TI-1]; cmp eax,5`);
3. the outer caller `FUN_1416fb130` loads arg3 from `[rbx+0xe4]` and arg4 from `[rbx+0xe8]`
   immediately after testing arg1 against 2 and 5 — a UI drop's (targetType, targetSlot)
   pair.

### The trailing `u8`

`arg5` reaches the wire as the low byte and is also used twice client-side: [L]

* in `FUN_1417df1d0`, `arg5 != 0` makes the client raise the confirmation dialog
  (`0xB2E`) **before** sending; `arg5 == 0` sends without asking.
* in the builder itself, `arg5 != 0` triggers an extra UI call (`0x1423045d0`) after the send.

Only two call sites of the dispatcher exist, and they pin the two observed values: [L]

| caller | what it looks like | arg3 | arg5 |
|---|---|---|---|
| `FUN_1416fb130` @ `0x1416fb320` | the inventory drag-and-drop | `[rbx+0xe4]` | **0** |
| `FUN_141782a30` @ `0x141784c4e` | a UI window path gated by `FUN_1417da100(1)` | **1** literal | **1** |

So on the wire the byte is 0 or 1 and distinguishes which UI started the scroll. Naming it
`bEnchantSkill` (v214) or `legendarySpirit` (older families) is **[I]** and unverified — I
could not identify `FUN_141782a30`. **A server should accept both values and not branch on
it until a capture says what it means.**

### What the client refuses to send at all

`FUN_142cc7a70` returns without building anything unless all of these hold: [L]

```
142cc7a99   [ctx + 0x2338] == 0
142cc7aa6   [ctx + 0x2330] == 0            <- the one-request-outstanding latch
142cc7ab3   [ctx + 0x2358] != NULL         <- the character/inventory object
142cc7ac3   FUN_1401ba9d0(cd+0x5b, cd+0x63) > 0
142cc7ad7   tick - [ctx + 0x2334] >= 0x1F4 (500 ms)
```

This is the same latch `research/npc-click.md` §4.3 documents — 37 setters, 7 clearers, all
clearers inbound handlers. **See §6 for why this is a live hazard for the scroll feature.**

---

## 4. `0x0236` — the result the server sends back

### How it was found

Not by name-matching. `research/user-pool-tables.md` already enumerates all four user-pool
dispatch tables. Running `tools/ripstrings.py` over table-A handlers, `0x0236`'s handler
`FUN_142792340` is the one that references

```
0x1427933fa -> 0x143481cd0  L"EnchantSuccess_Delay"
0x142793401 -> 0x143481d00  L"EnchantFailure_Delay"
```

and selects between them with `cmp byte [rbp+0x77], 1` — the packet's first `u8`. [L]

The obvious rival, `0x0238`, has the **identical** read shape `u8 u8 u32 u32`. It is
eliminated, not ignored: its message string is `0x111F` = *"Successfully equipped soul."*
[L] It is the soul-socket effect.

### The routing

`CUserPool::OnPacket` (`FUN_1429b9300`) sends `0x226..0x292` to `FUN_1429bafb0`. For every
opcode **except** `0x0226`, that function reads a `u32` **character id** first
(`0x1429bb08d call 0x1406e8c20`), looks the user up (`0x1429b6c90`), silently drops the
packet if no such user is in the pool, and otherwise calls the handler as
`handler(CUser*, InPacket*)`. [L]

`FUN_142792340` appears **once** across all four tables (grep of
`research/msexe-userpool-tables-handlers.txt`), so there is **one** opcode: the same packet
serves the scroller and the bystanders, addressed by char id. [L]

### The body

`tools/reads.py 0x142792340 6` — four reads, no more: [L]

```
0x142792381  u8   -> [rbp+0x77]
0x14279238c  u8   -> [rbp-0x69]
0x14279239a  u32  -> stored (ZtlSecure-obfuscated) at [rbp-0x41]
0x1427925a1  u32  -> stored (ZtlSecure-obfuscated) at [rbp-0x59]
```

```
off  size  field
  0   u16  opcode = 0x0236
  2   u32  charId          read by the table-A dispatcher, not the handler
  6   u8   result          0 fail / 1 success / 2 destroyed / 3 cannot-be-used
  7   u8   enchantDlg      0 = plain scroll presentation, non-zero = the enchant-UI presentation
  8   u32  scrollItemId
 12   u32  equipItemId
                           body = 14 bytes, 16 on the wire with the opcode
```

### `result` is a four-way code, and the client's own strings name every arm

Both display paths hand the byte to `FUN_1403dccf0`, whose dispatch at `0x1403dd12f` is: [L]

```
result == 0 -> 0x1403dd938 -> string 0x01B8 / 0x01B9
result == 1 -> 0x1403dd678 -> string 0x01B6 / 0x01B7   (or 0x0C6C if FUN_140416ff0(scrollId))
result == 2 -> 0x1403dd3e4 -> string 0x01BA / 0x01BB
result == 3 -> falls through -> string 0x0DEB / 0x0DEC
else        -> 0x1403dda59 (no message)
```

Decrypted with `tools/dump_stringids.py`: [L]

| result | generic form | named form |
|---|---|---|
| 0 | `0x01B8` "The scroll lights up, but the item winds up as if nothing happened." | `0x01B9` "The %s lights up, but nothing happens to %s." |
| 1 | `0x01B6` "The scroll lights up, and then its mysterious power has been transferred to the item." | `0x01B7` "The %s lights up, and its mysterious energies transfer to %s." |
| 2 | `0x01BA` "The item is destroyed due to the overwhelming power of the scroll." | `0x01BB` "The power of %s has destroyed %s." |
| 3 | `0x0DEB` "Cannot be used on this item." | `0x0DEC` "In %s, you cannot use %s." |
| 1, Karma | — | `0x0C6C` "Karma Ring was used on [%s] and Scissor Use Limit has been restored by 1." |

The `%s` order is pinned: `FUN_1403dccf0`'s arg4 is the value tested by
`FUN_140416ff0` ("is this the Karma scissors"), so **arg4 is the scroll id and arg5 is the
equip id**, and the handler passes the packet's *first* `u32` as arg4. So the first `u32` is
the scroll, the second is the equip. [L]

`FUN_142304650` — the UI half — branches on the same byte again at `0x14230468a`
(`cmp dl, 3`), `0x1423047af` (`cmp r14b, 1`) and `0x142304882` (`cmp r14b, 2`), so all four
values are live on the wire, not just success/fail. [L]

### Presentation, and who sees what

`FUN_142792340` splits three ways on `[user vtable + 0x50]`, the predicate
`research/user-chat-round2.md` records as `IsLocalUser`-shaped **[I]**: [L]

```
0x1427927b8  if (!user->[vt+0x50]())  -> 0x1427933b4 : no text, and the Enchant effect is
                                                       skipped too (0x1427933d3 tests it again)
             else if (enchantDlg == 0) -> 0x14279335f : format the message and post it as a
                                                       chat line of type 0x0B
             else                      -> 0x1427927cd..0x142793327 : the enchant-UI path,
                                                       FUN_142304650(result, scrollId, equipId)
0x142793418  a shared tail that runs for everyone, using result==1 and [user+0xe58]
```

So the *text* only reaches the user whose `[vt+0x50]` is non-zero. Send `0x0236` to the whole
map **including the scroller**; the client sorts out who gets what. [D]

### What `0x0236` does NOT do

It reads four fields and drives a message and an effect. It touches no item data, does not
change the equip's stats, and does not clear the request latch (`tools/fieldrefs.py 0x2330
--lo 0x142700000 --hi 0x142a00000 --write` finds nothing in the user-pool range). [L]

**The stat change must come separately, in `0x0070` InventoryOperation.** The equip record's
"Remaining Enhancements" is `item+0xfa`, bit 0 of the second optional-field mask, string
`0x039E` — see `research/equip-stats.md` §3, which this pass did not re-derive.

---

## 5. Client-side validation vs. server-side decision

**The client never computes success or failure.** [L] Nothing on the inbound path recomputes
anything: the applicability predicate `FUN_1404174b0` has exactly four call sites
(`tools/callers.py`), all of them on the outbound/UI side (`0x1403cfcd0`, `0x1417df1d0`,
`0x1417e0a60`, `0x1417e2c90`). `success` and `cursed` in `gm-handbook/scrolls.txt` are read
by the client only for the **tooltip**; the roll is the server's.

What the client *does* enforce, all before the packet leaves: [L]

| gate | at | effect |
|---|---|---|
| source inv type ∈ {2, 5}, source slot > 0 | `0x1417deb8d` | no packet |
| target inv type ∈ {1, 6} | `0x1417deba5` | no packet |
| dead check | `0x1417dec5f` | message `0x005F` "You can't do it once you're dead." |
| item-id routing ladder | §2 | a *different* opcode, or none |
| `FUN_1417e0a60(...)` | `0x1417df3ec` | no packet |
| `FUN_14038e290(...)` | `0x1417df9e6` | message `0x0BB5` "You cannot use that item on this equipment." |
| `FUN_1404174b0(scrollId, equipId)` | `0x1417dfa21` | routes to `0x0126` or refuses |
| the cash-shop / restricted check `FUN_142cc42d0(ctx, 0x1F4, 0)` when `arg6 & 8` | `0x1417df368` | message `0x00A8` "You can't do that right now." |
| confirmation dialog when `arg5 != 0` | `0x1417df953` | user must click Yes |
| the `0x2330` latch, `0x2338`, and a 500 ms throttle | `0x142cc7a99`.. | silently no packet |

The last row is the one that bites a server. **A scroll request the server does not answer
leaves `[ctx+0x2330] = 1`**, and that latch is shared by 37 request builders.

---

## 6. `[ctx + 0x2330]` — the latch, and the packets that clear it

`FUN_142cc7a70` sets `[ctx+0x2330] = 1` and `[ctx+0x2334] = tick` immediately after the send,
and refuses to build anything while `0x2330` is non-zero. [L] The latch is shared by 37
request builders (`research/npc-click.md` §4.3).

### This section was wrong once. The correction is the interesting part

My first pass ran `tools/fieldrefs.py 0x2330 --write` over `0x142c00000..0x142e10000` only,
cross-checked the clearers it found against `research/msexe-gamestage-cases.txt`, found only
`0x00A1` / `0x00A2` / `0x018C`, found that the standalone clearers (`FUN_142cd8d70`,
`FUN_142cd8e90`, `FUN_142cf3810`, `FUN_142cc52a0`, `FUN_142cae700`) have **zero** callers of
any kind — no `call`, no tail `jmp`, no qword pointer, no 4-byte switch RVA — and concluded
that the answer could not be reached statically.

That was an instrument's blind spot, not a fact. **`msexe-gamestage-cases.txt` names one
handler function per case, so a case whose body is *inlined into the dispatcher* is invisible
to it** — its own header says 104 of 285 labels are that shape. Running the scan over the
**whole image** puts four write sites inside `FUN_142cbaa80` itself, which is the game-stage
dispatcher: [L]

```
142cbae82  mov [rdi+0x2330], ebx    (ebx = 0)
142cbbd92  mov [rdi+0x2330], ebx
142cbbe84  xor ebx,ebx ; mov [rdi+0x2330], ebx
142cbcf3d  xor r8d,r8d ; mov [rdi+0x2330], r8d
```

Decoding the switch's own RVA table — `index = opcode - 0x70`, bound `0x32a`, table at
`0x142cbd9d0`, target = `0x140000000 + entry`, 273 distinct case bodies — maps each write
site to its opcode: [L]

| clears the latch | how | body reads |
|---|---|---|
| `0x00B7` | inlined in `FUN_142cbaa80` @ `0x142cbae80` | `u8, u8` |
| `0x00B8` **and** `0x00F5` | inlined, **one shared body** @ `0x142cbcf3a` | **nothing** |
| `0x00F8` | inlined @ `0x142cbbd90` | `u8` |
| `0x00F9` | inlined @ `0x142cbbe82` | `u8` |
| `0x00A1` | `FUN_142d1cdf0` | — |
| `0x00A2` | `FUN_142cd87d0` | — |
| `0x018C` | `FUN_142cd94e0` | — |

The whole-image scan is now complete: below `0x142cb0000` there are 7 hits, three of them
`movsd` stores on unrelated classes, and none of them this field. [L]

### `0x00B8` / `0x00F5` is an empty-bodied "you may send again"

```
142cbcf3a  xor  r8d, r8d
142cbcf3d  mov  [rdi+0x2330], r8d      <- clear the latch
142cbcf44  call FUN_1429e3ef0
142cbcf49  mov  [rdi+0x2334], eax      <- reset the 500 ms throttle's origin
142cbcf4f  call FUN_1429e3ef0 ; call FUN_142e54b20
142cbcf5b  call FUN_1429e3ef0 ; call FUN_142e54f40
142cbcf67  jmp  0x142cbbc10            <- the dispatcher's common exit
```

**It consumes no packet bytes at all.** [L] A two-byte packet — opcode `0x00B8` (or `0x00F5`)
and nothing else — is the client's generic unlock.

`research/msexe-gamestage-opcodes.md` fact #4 already described this body as *"reset
`param_1+0x466`, three `FUN_1429e3ef0` calls"* — and `0x466 * 8 = 0x2330`, so that note and
this scan are the same measurement seen from two sides. Nobody had connected the
decompiler's `param_1[0x466]` to the latch. One correction to that note: it says the body
*"falls through"*; the listing ends it with `jmp 0x142cbbc10`, the dispatcher's common exit.
[L]

### What this means for the scroll

The `0x0070` handler `FUN_142d51930` does **not** write `0x2330`
(`tools/fieldrefs.py 0x2330 --lo 0x142d00000 --hi 0x142d60000 --write` returns five rows,
none in it), and neither does `FUN_142792340`, the `0x0236` handler
(`--lo 0x142700000 --hi 0x142a00000` returns one unrelated `movsd`). [L]

**So answering a scroll with `0x0236` + `0x0070` alone leaves the latch set**, and the next
request from any of the 37 classes — every one of those `0x0105`..`0x0135` item requests —
is silently dropped by the client for the rest of the session. That is this project's
"always answer" rule in its exact classic form: the client's UI does not freeze, it just
goes quiet, which is much harder to notice.

The fix is one two-byte send. **Send `0x00B8` after the `0x0236` + `0x0070` pair.** It reads
no body, so there is nothing to get wrong. [D] — the effect on the latch is [L]; that
`0x00B8` is the *intended* packet for this particular request rather than merely a working
one is [I], since its candidate names in both reference enums are unrelated
(`IncubatorHotItemResult` / `CashPetPickUpOnOffResult`) and neither is worth anything.

**Still worth confirming on the first run: scroll twice in one session.** Two `0x0125` lines
in `world.log` means the unlock works. One means it does not, and `0x00F5`, `0x00F8`, `0x00F9`
and `0x00B7` are the remaining candidates, in that order of cheapness.

---

## 7. Clean Slate and White Scroll — protocol or policy?

**Policy. There is no protocol field for either, in either direction.** [L]

The request (§3) is exhaustively enumerated by `tools/encodes.py` at depth 6 and by
`research/msexe-packet-fields.txt` independently: five fields, and the three `u16` are
`srcSlot`, `dstInvType`, `dstSlot`. **There is no white-scroll slot and no white-scroll
flag.** The client cannot tell the server "consume a White Scroll with this" because it has
nowhere to put it.

The result (§4) is `charId, result, enchantDlg, scrollItemId, equipItemId`. **There is no
"the upgrade slot was preserved" bit and no "a slot was restored" count.** The equip's
remaining-enhancement number reaches the client only as an ordinary field of the item record
inside `0x0070`.

So the whole of both features is server-side arithmetic on one number:

* **White Scroll** — on a failure, do not decrement the equip's remaining-enhancement count.
  Then send `0x0236` with `result = 0` and an `0x0070` whose item record carries the
  unchanged count.
* **Clean Slate** — increment the equip's remaining-enhancement count by 1, guarded by
  whatever "has failed at least N times" rule the server keeps *in its own database*; the
  client sends no such state and is sent none. Then `0x0236` with `result = 1` and an
  `0x0070` carrying the new count.

### The strings exist, and nothing in the client uses them

The client's string table contains exactly the wording those features need:

```
0x0BB4  "Can only be used on equipment when item has failed to be upgraded at least %d time(s)."
0x0F0B  "#cUpgrade count protection is active.#"
0x0F19  "The power of the Guardian Scroll preserved the scroll."
0x0F1B  "Upgrade count protection has disappeared."
0x0F1C  "A scroll prevented upgrade count from decreasing."
0x0F1D  "Available upgrade count is unchanged due to the [Clean Failure] effect."
0x0DED  "Cannot be used on equipment that failed to get the %s upgrade."
```

Scanning the whole image for `mov r32, imm32` of each id — after proving the scan finds the
two controls `0xBB5` and `0xB2E` at the exact addresses already read out of the listings —
gives: [L]

| id | immediate references in code |
|---|---|
| `0xBB5` (control) | 6, including `0x1417df9ef` ✓ |
| `0xB2E` (control) | 8, including `0x1417df4ed` ✓ |
| `0x0F0B` | 2, in `FUN_14264f750` and `FUN_142c55890` — big renderers, tooltip-shaped **[I]** |
| `0x0F19`, `0x0F1B`, `0x0F1C`, `0x0F1D`, `0x0BB4`, `0x0DED` | **0** |

**Named blind spot:** this scan sees only ids loaded as a 32-bit immediate. An id computed at
runtime (`base + n`, a table lookup, a switch) is invisible to it, and `0x0F19`..`0x0F1D` are
five consecutive ids, which is exactly the shape a loop or table would use. So the correct
statement is *"no client code loads these ids as constants"*, not *"the client never shows
them"*. It does not change the conclusion, because the conclusion rests on the **packet
enumerations**, not on this scan: the fields are not there.

The practical consequence: if MapleCW wants any of that wording on screen, it sends it as a
chat/system message of its own. The client will not produce it.

---

## 8. What the server must accept and send

Sizes are body bytes; add 2 for the little-endian opcode.

### Inbound — `0x0125`, 11-byte body

```
 off  size  field         label  notes
   0   u32  tick          [L]    FUN_1429e3ef0(); echo nothing, ignore it
   4   u16  srcSlot       [L]    scroll slot, inventory type 2 (CONSUME) or 5 (CASH); always > 0
   6   u16  dstInvType    [L]    1 = equip inventory (worn = negative dstSlot), 6 = extended equip window
   8   u16  dstSlot       [L]    equip slot; SIGNED - negative means a worn item
  10   u8   flag          [L]    observed 0 (inventory drag) and 1 (the other UI path);
                                 meaning NOT established - accept both, do not branch on it
```

Read `dstSlot` as an `i16`. The client will never send `srcSlot <= 0` and will never send a
`dstInvType` other than 1 or 6. [L]

**Always answer**, even on refusal — `result = 3` exists precisely for that (§4).

### Outbound — `0x0236`, 14-byte body, broadcast to the map including the scroller

```
 off  size  field          label  notes
   0   u32  charId         [L]    the scroller; the client drops the packet if no such user is in the pool
   4   u8   result         [L]    0 = fail, 1 = success, 2 = item destroyed, 3 = cannot be used here
   5   u8   enchantDlg     [L]    0 selects the plain scroll presentation; send 0
   6   u32  scrollItemId   [L]    the scroll consumed - fills the first %s
  10   u32  equipItemId    [L]    the target equip - fills the second %s
```

`enchantDlg = 0` is the safe value: it is the branch that produces the ordinary chat line and
the `EnchantSuccess_Delay` / `EnchantFailure_Delay` field effect. Non-zero routes into the
enchant-UI object and was not traced. [D]

### Outbound — the actual stat change

`0x0070` InventoryOperation, with the updated equip record. **Not decoded here** — it is
already this project's most-used packet; `research/equip-stats.md` has the item record's
field layout and `research/equip-block.md` the block form.

On `result == 2` (destroyed) the item must be **removed** by `0x0070`; `0x0236` alone will
print "The item is destroyed" and leave the item on screen. [D]

The scroll itself is consumed in every case except `result == 3`, and that too is `0x0070`'s
job. [D]

### Outbound — `0x00B8`, empty body, to the scroller only

Two bytes, opcode and nothing else. It clears `[ctx+0x2330]` and resets `[ctx+0x2334]`, and
its case body reads no packet bytes. [L] **Send it on every `0x0125`, including the
refusals** — the latch is set by the builder before the server has any say in it, so a
refused scroll latches the client exactly as a successful one does. Without it the next
request from any of the 37 sharing builders is silently dropped for the rest of the session
(§6).

### Clean Slate / White Scroll — server-side policy, no protocol

* Nothing in `0x0125` names a white scroll. **[L]**
* Nothing in `0x0236` reports a preserved or restored upgrade slot. **[L]**
* The only thing the client will ever see is the equip's remaining-enhancement number inside
  the `0x0070` item record. **[D]**
* The wording for both features exists in the string table and no client code loads those ids
  as constants; if MapleCW wants it on screen it must send its own message. **[L]**, with the
  blind spot named in §7.

---

## 9. What I could not determine, and the cheapest thing that settles each

| open | cheapest discriminator |
|---|---|
| **What the trailing `u8` of `0x0125` means.** Two call sites give 0 and 1; `FUN_141782a30` is unidentified. | One capture (below). If the byte is 0 for a normal inventory drag, nothing else needs doing. |
| **Whether `0x00B8` is the *right* unlock** for a scroll (§6). That it clears the latch is [L]; that it is the packet the real server sends here is [I]. | **Scroll twice in one client session** with `0x00B8` wired. Two `0x0125` lines in `world.log` → done. One → try `0x00F5`, then `0x00F8`, `0x00F9`, `0x00B7`. |
| **Whether `enchantDlg != 0` is ever wanted.** Untraced branch into the enchant-UI object. | Nothing needed; send 0. |
| **What `[srcItem + 0x38]` is**, the field that diverts the request to `0x0116` instead of `0x0125`. | Only matters if a real scroll ever takes that branch; a capture will show `0x0116` instead of `0x0125` if it does. |
| **Which `0x0236` presentation path is the local user's.** `[vt+0x50]` is `IsLocalUser`-shaped **[I]** from `research/user-chat-round2.md`, never measured. | Broadcast to everyone and watch one screen. |
| **The `0x0070` record's remaining-enhancement encoding** — read from `research/equip-stats.md`, not re-derived here. | It is already implemented; a scroll that visibly changes the tooltip number confirms it. |

### Would one capture beat more static work? Yes, for the request; no, for the result.

**For `0x0125`: yes, decisively.** The server logs every unhandled inbound opcode with its
full body — the archived runs show `<- 0x010E UNKNOWN, 14 byte body f7e1140f010080841e0001000000`
— so a single scroll attempt writes the opcode *and* all 11 bytes into `world.log`. That
would confirm the field order, pin the trailing `u8` for the normal drag path, and show
whether a worn item really sends a negative `dstSlot`, in one action. None of those three is
fully settled by static work alone.

`grep -c "0x0125" previous-runs/world*.log research/fixtures/*world*.log` is **0** today, and
`0x010E` is present 51 times in the same files, so the grep speaks. Nobody has ever scrolled
in front of this server.

**For `0x0236`: no.** A capture cannot show it — the server has to send it first. The four
result codes and their exact strings came out of the client's own string table and are
already as settled as they can get without a screen.

### The one-line ask for a run

Put a scroll and a scrollable equip in the bag, drag the scroll onto the equip **twice**, and
paste the `<- 0x0125` lines from `world.log`. Two lines confirms both the body *and* that the
`0x00B8` unlock works; one line means the unlock is the wrong opcode and §6's fallback list
is next.

---

## 10. Correction log

* **§6 was written wrong first and is now right.** The first version concluded that nothing
  reachable clears `[ctx+0x2330]` and filed it as an unresolvable hazard. That came from
  scanning a sub-range of the image and from trusting
  `research/msexe-gamestage-cases.txt`, which lists one *named handler* per case and
  therefore cannot show a case whose body is inlined into the dispatcher — its own header
  says 104 of 285 labels are exactly that shape. The whole-image scan put four writers inside
  `FUN_142cbaa80`, and decoding the switch's RVA table named the opcodes. This is the same
  failure mode `CLAUDE.md` records for the `mob+0x42c` write-scan: the tool was working, the
  question was too narrow. **Widening the range was not a second opinion; reading the switch
  table was.**
* The negative that survived: `FUN_142cd8d70`, `FUN_142cd8e90`, `FUN_142cf3810`,
  `FUN_142cc52a0` and `FUN_142cae700` still have no caller of any kind. They clear the same
  latch and nothing reaches them. Unexplained, and no longer load-bearing.
* `tools/ripstrings.py`'s documented positive control does not reproduce (§1). Unrelated to
  this finding, but the next agent should not trust that docstring.

---

## 11. Correction to the correction: `0x00B8` is NOT needed. Added by the coordinator, 2026-09-09.

§10 concluded that `0x0236` + `0x0070` leave the latch set and that a third packet - `0x00B8`,
empty body - is **mandatory** on every `0x0125`. **That is wrong, and the way it is wrong is
the same failure §10 was itself correcting, one turn later and in the mirror.**

`0x007C` `StatChanged` with `bExclRequestSent = 1` clears the latch. **[L]**

```text
142d547ad  call 0x1406e8ae0     read u8 bExclRequestSent
142d547b2  test al, al
142d547b4  je   0x142d547c0     ... byte is 0: skip
142d547b6  xor  edx, edx
142d547b8  mov  rcx, r14
142d547bb  call 0x142cc4430     the SETTER, with 0  ->  [ctx+0x2330] = 0
```

**Why the scan could not see it.** §10's whole-image pass searched for *inline writes* to
`+0x2330`. This clear is not an inline write - it is a **call to the setter** `0x142cc4430`
with `edx = 0`. One shape, missed.

That is exactly, and oppositely, the bug found in `net::dropmoney::LATCHING_REQUESTS` the same
day: that list was built by searching for `call 0x142cc4430` and was blind to the two opcodes
that store `+0x2330` **inline**. One search saw only calls; the other saw only writes. Neither
was wrong about what it found, and both were wrong about what they concluded from silence.
`CLAUDE.md`: *the two worst wrong answers here both came from searching a known list - one
looked for the wrong SHAPE.* Twice in one day, in both directions.

**And this one is settled on a screen, not only in a listing.** `0x007C` with
`bExclRequestSent = 1` is precisely what `world::mesodrop` and every chair handler already
send, and the owner, 2026-09-09, after the meso drop shipped: *"meso dropping is fine now, inventory
is fine as well after meso dropping."* An unlock that did not unlock would have shown there
first.

**So the answer set for a scroll is two packets, not three:** `0x0236` for the effect and
message, `0x0070` for the stat change - and the unlock rides on whichever `StatChanged` the
handler already sends, as it does everywhere else in this server.

**Do not implement `0x00B8`.** §10 labels it `[I]` and notes its candidate names in both
reference enums are unrelated junk. It is an unproven packet solving a problem that does not
exist, and sending an undecoded opcode is how this client has been killed twice.

The client-run ask is unchanged and still worth doing - **scroll twice in one session** - but
its meaning is now narrower: it confirms the `0x0125` body and field order. The unlock half is
already answered.
