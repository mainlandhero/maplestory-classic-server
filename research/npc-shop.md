# The NPC shop, in both directions

Goal F. The owner, after the run of 2026-08-20: *"when I tried to click on Lucy (NPC ID 21), it
does not open their shop."*

Nothing opened because nothing built a shop packet. This file is the packet, read off the
listing. Labels as the house uses them: **[L]** read off this client's listing, **[D]**
derived from two or more [L] facts, **[I]** inferred.

**The Ghidra project was not opened.** Everything here is `tools/listing.py`,
`tools/reads.py`, `tools/callers.py`, `tools/xref.py`, `tools/fieldrefs.py`,
`tools/dataref.py` and `tools/dump_stringids.py`.

---

## UNRESOLVED, at the top where it belongs

1. **The sell direction has never been seen on a wire, in either direction.** The buy
   direction is one straight-line trace from the row bytes to the request bytes. The sell
   direction is [L] at every step *individually* - the sign test at `140d23ac5`, the tab
   accessor `FUN_140d2d040`, the inventory intersection in `FUN_140d26870`, the
   `price <= 0` arm of `FUN_140d2c060` - but nothing joins them into one observed
   transaction. If the Sell tab is empty on the run, that is the claim that failed and
   §4 says which instrument settles it.
2. **`row_key` (row offset 0) is a value the server invents, and this is a decision rather
   than a measurement.** [L] that the client only echoes it and that its one other use
   degrades gracefully on a miss. **Not established** what NexonKorea's server put there.
   `net::shop` uses the row's index in the packet.
3. **Four `u32`s in each row are unexplained** - row offsets 12, 16, 20, 24. [L] that they
   are read and where they are stored; **not established** what they mean. Sent as zero.
4. **`0x0104` sub-op 3 is not the shop UI's** and is unread. Its shape is `u8 3, u32`.
5. **Whether the shop may be opened while an NPC dialogue is up** is not established. The
   shop is a different UI object from `CUIScriptMsg`, and the `SetField` hazard that
   `net::script` documents does not obviously apply, but nothing was traced.
6. **NPC name -> template id.** `data/shops.txt` names Lucy the way the UI does;
   `crates/world/src/shops.rs` has `by_npc(&str)` and nothing maps that onto a template.
   `gm-handbook/npcstrings.txt` has `21 name Lucy`, so the join exists and is not written.
   This was already open as `research/shops.md` §6 item 2 and it is still the last thing
   between the packet and a shop on screen.

---

## 0. The instruments, checked before any of their answers were used

`tools/reads.py`'s own positive control ran first:

```text
python tools/reads.py 0x140304100 2
  0x140304138  READ raw (direct)
  0x140304144  READ u8  (direct)
  0x140304183  READ u8  (direct)
  0x1403041c2  READ u16 (direct)   ... and the rest of the documented run
```

and `tools/listing.py 0x140304100 | grep READ` reproduces the same addresses, which is the
control its own docstring specifies. `tools/callers.py`'s control
(`python tools/callers.py 0x1402fa9a0`) returns 96 call sites in 15 functions - non-empty,
so a zero from it later is a real zero. `tools/xref.py --va` and `tools/fieldrefs.py` were
each used only where a *positive* was found, never to support an absence.

On the two shop functions the read counter and the listing agree, at depth 3 and at depth 5:
**five reads** in `FUN_140d225f0` and **ten** in `FUN_140d23030`. That matters here for the
reason `CLAUDE.md` gives - a walker that saw nine where there were ten has shipped a short
packet twice on this project.

The client -> server bodies in §4 were then re-derived with a **different instrument**,
`tools/encodes.py`, whose own positive control (`python tools/encodes.py 0x141cb6880 1` ->
the `0x2FF` ctor at `141cb7eb1`) ran first. It reproduces all five builders field for field
with no helper writes and no tail `jmp` into a writer - which is the specific failure
`research/mob-combat.md` records for the encode side. Two instruments, same answer.

Dumps kept: `research/msexe-shop-handler.txt`, `research/msexe-shop-rowdecode.txt`,
`research/msexe-shop-transaction.txt`, `research/msexe-shop-selltab.txt`,
`research/msexe-shop-activelist.txt`, `research/msexe-shop-routing.txt`.

## 0a. How it was found, and why not from an opcode map

Not by aligning an enum. The chain was:

1. Enumerate every string in the image containing `Shop` (ASCII **and** UTF-16 - the UI
   paths are UTF-16 and an ASCII-only scan misses `Shop2/backgrnd` entirely). That gives
   `UI/UIWindow2.img/Shop2/backgrnd`, `/BtBuy`, `/BtSell`, `/TabBuy`, `/TabSell` at
   `0x14336c940..0x14336cad0`. **[L]**
2. `tools/xref.py --va` on each: they belong to `FUN_140d22310` (the constructor) and
   `FUN_140d23fb0` (`CreateLayout`, 7413 bytes). So the shop UI lives at `0x140d2xxxx`.
3. `tools/callers.py` on all **ten** read primitives, filtered to that range - not five,
   not seven; the whole list from `tools/reads.py`'s `PRIM`. Exactly two functions in the
   shop UI read packets: `FUN_140d225f0` and `FUN_140d23030`. (The *range* in that step is
   a guess and was later replaced by an enumeration over the class's vtable - see §4.0,
   which is where the guess turned out to matter.)
4. `tools/callers.py 0x140d225f0` -> one call site, `0x141822023`, inside `CField::OnPacket`.
   Reading the range test there gives the opcodes.

Independently, `research/msexe-send-opcodes.txt` filtered to `FUN_140d2*` gives exactly one
outbound opcode, **`0x0104`**, at four sites - which is the client -> server half.

> **An instrument note worth keeping.** `tools/callers.py` on `FUN_140d23fb0` returns
> **0 call sites** and `tools/xref.py --va` returns **0 references**, because it is a vtable
> slot and neither tool can see one. A scratch qword scanner found it at `0x14336c720`, in
> the vtable at `0x14336c6e0`. Two clean zeros about a function that is obviously called;
> exactly the shape `CLAUDE.md` says to distrust.

---

## 1. Routing - one function, two opcodes **[L]**

`CField::OnPacket` (`FUN_141820080`), the same dispatcher that already delivers `0x044F`
`NpcEnterField` and `0x055B` the script message:

```asm
141822011  lea  eax, [r9 - 0x55f]
141822018  cmp  eax, 1
14182201b  ja   0x14182202d        ; neither -> the next range test
14182201d  mov  rdx, rbx           ; CInPacket*  becomes arg 2
141822020  mov  ecx, r9d           ; the OPCODE becomes arg 1
141822023  call 0x140d225f0
```

`FUN_140d225f0(int opcode, CInPacket *pkt)` forks in its first three instructions:

```asm
140d2262b  sub  ecx, 0x55f
140d22631  je   0x140d22892        ; 0x055F -> the transaction result
140d22637  cmp  ecx, 1
140d2263a  jne  0x140d22fb5        ; neither -> return, silently
                                   ; fall through = 0x0560, the shop list
```

| direction | opcode | what |
|---|---|---|
| server -> client | **`0x0560`** | open the shop and fill it |
| server -> client | **`0x055F`** | the transaction result |
| client -> server | **`0x0104`** | four sub-ops: reopen, transact, close, and one unknown |

---

## 2. `0x0560` - the shop list

### 2.1 The head **[L]**

```text
off  size  read at      field
  0     4  140d22643    npcTemplateId   -> shopUI+0x318 (140d23063)
  4     2  140d2264e    rowCount        u16, movzx-ed: 1..65535
```

**The count is a `u16`**, `FUN_1406e8b80`. The task brief flagged exactly this hazard and
it is real: the record has no length prefix and no resynchronisation point, so a `u8` there
shifts every row by one byte and the client reads 31-byte rows out of the middle of the
list.

### 2.2 A count of zero does not open an empty shop - it opens nothing **[L]**

```asm
140d22656  test edi, edi
140d22658  jne  0x140d22819        ; count != 0 -> build the shop
```

With `rowCount == 0` the client takes the other arm entirely: it never calls the row
decoder, never creates a shop UI, builds a `CUIScriptMsg` instead (`FUN_142a61900` at
`140d2273d`, spoken by the npc template it just read) and **sends `0x0104` sub-op 2 back**
at `140d227a2`. So a shop with no rows is a dialog box.

### 2.3 The row - 31 bytes, fixed, no branch in the run **[L]**

`FUN_140d23030(shopUI, pkt, npcTemplateId, rowCount)`, `140d23030..140d23ebb`. Loop body
starts `140d230da`, back edge `140d23b86  jl 0x140d230da`.

```text
off  size  read at      stored at             what
  0     4  140d2317d    row+0x20 (obfusc.)    row key - the only thing the client hands back
  4     4  140d23346    row+0x38 (obfusc.)    itemId
  8     4  140d2351a    row+0x58, |v| +0x5c   price, SIGNED
 12     4  140d2352d    row+0x60              unknown
 16     4  140d23538    row+0x64              unknown
 20     4  140d23543    row+0x6c              unknown
 24     4  140d2354e    row+0x70              unknown
 28     1  140d23559    row+0x74              disabled flag
 29     2  140d23564    row+0x68              max quantity per purchase
```

then, **once, after the whole loop**:

```text
  *     1  140d23b96    shopUI+0x14dc         a trailing u8
```

So the body is `4 + 2 + 31n + 1`. `140d230d2  test ebx, ebx / jle 0x140d23b93` shows the
trailing byte is read whether or not the loop runs - though the outer handler never calls
the decoder with `n == 0` anyway (§2.2).

Both `+0x20` and `+0x38` are stored through the obfuscated-int helper `FUN_14022dad0` and
read back through `FUN_14019a5d0`, the same scheme `research/mob-spawn.md` documents on
`mob+0x3dc/0x3e0`. That is why they are not plain fields in the struct.

### 2.4 `itemId` is field 1, not field 0 **[L], and this was the trap**

Both are item-id-shaped. The listing separates them:

* `140d2c2c8` de-obfuscates **`row+0x38`** and passes it to `FUN_1403e8af0`, which is
  `GetInventoryType(itemId)` - it divides by 1 000 000 (`0x431bde83` / `sar 0x12`), returns
  1..5, and returns **6** for an equip flagged cash. The result then indexes the player's
  inventory array at `140d2c2e3`.
* `140d25f2f` de-obfuscates `row+0x38` for the equip preview, and `140d25f95` range-tests it
  against `1 050 000 .. 1 059 999`.
* `140d26a61..140d26a68` compares `row+0x38` against an **inventory item's** `+0x20` - the
  itemId field of a `GW_ItemSlot`.

`row+0x20`, by contrast, is used in exactly two places: it is written verbatim into the
transaction request (`140d2c747`), and it keys a hash map on the *character* at `140d2604b`
whose only effect on a hit is one extra `u32` in the tooltip block (`140d26170`); a miss
falls through to the same instruction. **So field 0 is a token the server chooses and field
1 is the real `Item.wz` id.**

### 2.5 The sign of `price` chooses the tab **[L] - the finding that makes selling work**

Each decoded row is appended to one of two lists:

```asm
140d23ac5  cmp  dword ptr [rbx + 0x58], 0
140d23ac9  mov  rcx, qword ptr [rbp - 0x10]    ; &shopUI+0x338
140d23acd  jg   0x140d23ad3
140d23acf  mov  rcx, qword ptr [rbp - 0x48]    ; &shopUI+0x340
140d23ad3  call 0x140d2d0a0                    ; push_back
```

and `FUN_140d2d040`, the whole of which is 69 bytes, is the "which list is the active tab"
accessor:

```asm
140d2d049  mov   rcx, qword ptr [rcx + 0x2d0]   ; the tab control
140d2d068  cmp   dword ptr [rcx + 0x90], 0      ; its selected index
140d2d06f  mov   eax, 0x340
140d2d074  mov   edx, 0x338
140d2d079  cmove eax, edx                       ; tab 0 -> +0x338, else -> +0x340
140d2d07c  add   rax, rbx
```

> **`price > 0` is a Buy row. `price <= 0` is a Sell row.** `jg` is strictly greater, so a
> zero price is a Sell row.

`+0x58` keeps the signed value and `+0x5c` keeps `|value|`
(`140d23522  cdq / xor eax,edx / sub eax,edx`), and the buy path uses the signed one for the
tab test and the absolute one for the cost.

The four lists on the UI object, all cleared by `FUN_140d2d210` at `140d23089..140d230c9`:

| offset | what | how it is filled |
|---|---|---|
| `+0x338` | the **Buy** tab | rows with `price > 0`, straight from the packet |
| `+0x340` | the **Sell** tab's catalogue | rows with `price <= 0`, straight from the packet |
| `+0x348` | the Sell tab as **displayed** | `+0x340` intersected with the player's inventory |
| `+0x350` | the previous `+0x348` | so `FUN_140d2bd40` can diff after a transaction |

`FUN_140d26870(shopUI, invType, dest)` builds `+0x348`: it walks the character's inventory
of `invType` (`FUN_14030ccb0` for the slot count, `FUN_1402e3cd0` per slot), drops any item
failing three client-side predicates (`140d2699e`, `140d269bf`, `140d269e0`) or carrying a
non-zero `+0x38`, then **linear-searches `[shopUI+0x340]`** comparing `row+0x38` to the
item's id (`140d26a61`), and clones the matching row.

`140d26a17  test rax, rax / je 0x140d27030` means a null `+0x340` ends that search for every
item. **A shop that sends no negative-price rows has an empty Sell tab.** [L]

### 2.6 The two row fields that can silently break a shop **[L]**

* **row offset 28, the disabled flag.** `140d2c15e  cmp byte ptr [rsi + 0x74], 0 / jne` bails
  out of the *entire* transaction handler, and `140d26b09` drops the row when building the
  Sell tab. Send `0`.
* **row offset 29, the max per purchase.** `140d2c1f5  mov ebx, dword ptr [rsi + 0x68]` is
  the maximum handed to the "How many are you willing to buy?" box (string `0x4A2`), and
  `140d2c239  cmp ebx, dword ptr [rsi + 0x68] / jg` rejects the answer above it. **Zero
  makes every quantity fail**, with no message.

### 2.7 The four unknown `u32`s

[L] that they are read at offsets 12/16/20/24 and stored at `row+0x60/0x64/0x6c/0x70`, and
[L] that their only consumer is `FUN_140d25dc0`, which packs `+0x58, +0x60, +0x64, +0x6c,
+0x70` into a 21-byte block at `140d2601c..140d2603c` behind a leading `1` byte - a tooltip
/ price-info struct. **Nothing in the buy path or the list build reads them.** Sent as zero.

The v214 reference's `NpcShopItem::encode` was read and is **not** this packet - it writes
roughly 150 fields including several `FILETIME`s and six strings, against 31 fixed bytes
here. It contributed nothing and no candidate name from it is used.

---

## 3. `0x055F` - the transaction result **[L]**

Two bytes. That is the whole packet.

```asm
140d22892  mov  rcx, qword ptr [rip + 0x2d85c87]   ; -> 0x143AA8520, the current dialog
140d2289c  je   0x140d22fb5                        ; no dialog -> DROP, before any read
140d228b0  call qword ptr [rax + 0xd0]             ; a type check; failing it also drops
140d22927  mov  dword ptr [r13 + 0x14d8], esi      ; esi = 0 - THE LATCH IS CLEARED HERE
140d22931  call 0x1406e8ae0                        ; u8  dialogKind
140d22936  cmp  al, 4
140d22938  jne  0x140d22fb5                        ; anything but 4 -> DISCARD
140d22941  call 0x1406e8ae0                        ; u8  resultCode
```

**The first byte must be exactly 4.** [D] that 4 means "NPC shop": the shop UI's own
constructor passes the same literal to the base UI init at
`140d223fe  mov dword ptr [rsp + 0x28], 4`, in the function that references
`UI/UIWindow2.img/Shop2/backgrnd`.

Note the ordering: **the latch is cleared at `140d22927`, before the `cmp al, 4`.** So a
result with a wrong first byte does still un-latch the dialog - but it shows nothing and
tells the player nothing, which is the worst kind of half-working.

### 3.1 The result codes

`140d22949  test al, al / jne 0x140d229d1` splits success from failure; the failure arm does
`dec edi / cmp edi, 0xd / ja <default>` and indexes a **14-entry** table at `0x140d22fe8`
(decoded in `research/msexe-shop-routing.txt`). String ids resolved through `FUN_1408a9e40`
and decrypted with `tools/dump_stringids.py`.

| code | string id | text | re-asks for the list? |
|---:|---:|---|---|
| 0 | - | success: no message, only the list scroll position moves | no |
| 1, 2, 3 | `0xC57` | "That's not something I barter. I'll show you the list again for you to choose from." | **yes** |
| 4 | `0xC58` | "Are you begging?" | no |
| 5 | `0xC59` | "Why do you carry around so much Mesos? Come back after emptying your pockets." | no |
| 6 | `0xC5A` | "Taking your precious time, huh? I don't barter this item anymore." | **yes** |
| 7 | `0xC5B` | "I'm done bartering this item for the day. Come back tomorrow." | **yes** |
| 8 | `0xC5C` | "Why are you obsessing over this item? I won't barter anymore." | **yes** |
| 9 | `0xC5D` | "You're greedy. How about you lower the quantity?" | no |
| 10 | `0xDAB` | "Items or mesos cannot be moved.\r\nPlease contact customer support." | no |
| 11 | `0x12D6` | "That cannot be done in the current world." | no |
| 12 | `0x4AA` | "Please check if your inventory is full or not." | no |
| 13 | `0xC5E` | "I'm a little preoccupied right now, so come back later." | **yes** |
| 14 | `0x14E8` | "Item cannot be obtained when incapacitated." | no |
| >= 15 | - | nothing at all, but the latch is still cleared | no |

The "re-asks" column is `140d229df..140d22a18`, a chain of `cmp edi, N / je 0x140d22a20` for
N in `{1, 2, 3, 6, 7, 8, 0xd}`. `140d22a20` sets the flag the tail tests at `140d22f51`, and
the tail sends `0x0104` sub-op 0 carrying `shopUI+0x318`. **A server that sends one of those
seven owes a fresh `0x0560` immediately after.**

Two failures are **client-side pre-checks and never reach the server**: `0x9D` "You don't
have enough Mesos." (`140d2c266`, after `|price| * quantity` is compared against the
player's own meso count at `140d2c253`) and `0xC5F` "You do not have the item."
(`140d2c43e`).

### 3.2 What success does NOT do **[L]**

`140d22951..140d229cf` - the success arm - only adjusts the list's scroll position through
`FUN_1416ee2d0` and writes `shopUI+0x14d0`. **It does not add the item to the bag, remove
the sold item, or change the meso count.** Those still have to go out as a `0x0070`
(`crates/net/src/inventory.rs`) and as whatever carries the meso field.

---

## 4. `0x0104` - the client's request **[L]**

One opcode, a leading `u8` sub-op, four shapes. All four builders were enumerated from
`research/msexe-send-opcodes.txt` and re-read from the listing.

| site | sub-op | body after the `u8` | sender |
|---|---:|---|---|
| `140d22f61` | **0** | `u32 npcTemplateId` | `FUN_140d225f0`'s result tail - "show me the list again" |
| `140d2c736` | **1** | `u32 rowKey, u16 quantity, u16 slot` | `FUN_140d2c060` - the transaction |
| `140d227a2` | **2** | nothing | `FUN_140d225f0`'s zero-row arm |
| `140d23efe` | **2** | nothing | `FUN_140d23ed0` - the dialog closing |
| `14216a837` | **3** | `u32` | `FUN_14216a810` - **not the shop UI**; unread |

### 4.0 "The shop UI builds no other packet" - enumerated, after the first version was filtered

The first version of this claim was `tools/callers.py 0x1406ed520` **filtered to
`0x140d2xxxx`**, an address range picked by looking at where the functions happened to sit.
That is a filter over a guessed set, which is the shape `CLAUDE.md` names as the source of
the two worst wrong answers on this project - and here it was demonstrably too narrow:
`FUN_140d31a00` builds three packets from just past the `0x140d30000` bound.

Redone as an enumeration over the class's **actual** method set - the 76 entries of the shop
UI's vtable at `0x14336c6e0`, walked to the `COutPacket` ctor with `tools/reads.py`'s own
loader and tail-`jmp` handling, so it cannot disagree with the read and encode counters:

```text
vtable 0x14336c6e0: 76 entries
methods reaching the COutPacket ctor within 3 levels: 5
  IN-RANGE  0x140d23ed0   direct ctor site 0x140d23efe     the close, sub-op 2
  IN-RANGE  0x140d262f0   via a helper                     -> FUN_140d2c060, sub-op 1
  IN-RANGE  0x140d26630   via a helper                     -> FUN_140d2c060, sub-op 1
  OUT-RANGE 0x142bf24a0   via a helper                     -> not 0x0104
  OUT-RANGE 0x142bf5d90   via a helper -> FUN_142d178d0    builds 0x02D9, u32 u32
```

The two the range filter would have missed are **inherited base-class UI methods** and build
`0x02D9` and one deeper unidentified packet - neither is `0x0104`. Note also that they are
not evidence of anything shop-specific: `research/msexe-gamestage-dispatch.md` records that
MSVC `/OPT:ICF` folds identical stubs, so one method address appears in hundreds of unrelated
vtables, and a shared slot is not ownership.

`FUN_140d225f0` - which builds sub-ops 0 and 2 - is not a vtable method at all; it is the
free function the dispatcher calls, found through `tools/callers.py` in §0a. So the two
routes into this class are both covered, and **the five sites in the table above are all of
`0x0104`.**

`tools/callers.py 0x1406ed520` restricted to the same class agrees.

`tools/encodes.py` at depth 3 gives the same five bodies with nothing extra:

```text
FUN_140d2c060   CTOR 140d2c736, w_u8 140d2c742, w_u32 140d2c756, w_u16 140d2c764, w_u16 140d2c771
FUN_140d225f0   CTOR 140d227a2, w_u8 140d227b1                                (the zero-row close)
FUN_140d225f0   CTOR 140d22f61, w_u8 140d22f6d, w_u32 140d22f79              (the re-request)
FUN_140d23ed0   CTOR 140d23efe, w_u8 140d23f0b
FUN_14216a810   CTOR 14216a837, w_u8 14216a844, w_u32 14216a850
```

The widths themselves are not taken on trust either: `1406ed840` = 1 byte, `1406ed940` = 2,
`1406ed9d0` = 4 was read at each writer's `ADD dword ptr [rbx+0x428],n` in
`research/mob-behaviour.md` §5, independently of this work.

### 4.1 Sub-op 1, the transaction **[L]**

```asm
140d2c73c  mov   dl, 1                       ; sub-op
140d2c747  lea   rcx, [rsi + 0x20]           ; the selected row
140d2c74b  call  0x14019a5d0                 ; de-obfuscate the row key
140d2c756  call  0x1406ed9d0                 ; u32 rowKey
140d2c75b  movzx edx, word ptr [rsp + 0x60]
140d2c764  call  0x1406ed940                 ; u16 quantity
140d2c769  movzx edx, r15w
140d2c771  call  0x1406ed940                 ; u16 slot
140d2c77f  mov   dword ptr [r14 + 0x14d8], 1 ; THE LATCH
```

Nine bytes, and **the wire does not say which direction it is**. The client took the row out
of one of the two tabs and sends only its key, so the server tells buy from sell by looking
`rowKey` up in the list it sent - positive price means buy, negative means sell.

The buy arm and the sell arm of `FUN_140d2c060` are chosen by the same sign test the tabs
use:

```asm
140d2c187  cmp  dword ptr [rsi + 0x58], edi   ; edi = 0
140d2c18a  jle  0x140d2c2a5                   ; price <= 0 -> the sell arm
```

* **Buy arm** (`price > 0`): quantity comes from the "How many?" box, range-checked against
  row offset 29; `slot` is `0` (`140d2c168  mov r15d, edi`). Then `140d2c242` computes
  `|price| * quantity`, `140d2c253` reads the player's mesos, and `140d2c25d  cmp rax, rbx /
  jge` is the only thing standing between the click and the send.
* **Sell arm** (`price <= 0`): quantity is forced to `1` at `140d2c16b` and nothing on that
  path overwrites it; the client scans the player's inventory of
  `GetInventoryType(row.itemId)` from slot 1 upward (`140d2c2f0..140d2c3f0`) for the item and
  sends the slot it stopped on, or `row+0x50` when the row already carries one (`140d2c421`).

So `slot != 0` is a **hint** that this is a sell - [L] that the buy arm always writes 0 - but
it is a client-supplied field and it is not the decision.

### 4.2 The latch, and why an unanswered sub-op 1 is a dead shop **[L]**

`FUN_140d2c060`'s first act:

```asm
140d2c0a6  cmp  dword ptr [rcx + 0x14d8], edi    ; edi = 0
140d2c0ac  jne  0x140d2c4f7                      ; -> the epilogue. Nothing happens.
```

`140d2c4f7` is the stack-cookie check and the `ret`. So while `shopUI+0x14d8` is set, **every
further click on Buy or Sell is a silent no-op** - no message, no sound, nothing in any log.

This is the `net::inventory::inventory_rejected` precedent: on 2026-08-19 a refusal that sent
nothing left `player+0x2330` latched and killed every later inventory action for the session.
`net::shop::shop_result` exists so that a refusal is one call and not a temptation.

**But it is not the same latch, and the difference is worth stating rather than smoothing
over.** `tools/fieldrefs.py 0x14d8 --lo 0x140d20000 --hi 0x140d30000` enumerates **four**
writers, not one:

```text
140d2c77f  mov dword ptr [r14 + 0x14d8], 1      in FUN_140d2c060   the SET
140d22927  mov dword ptr [r13 + 0x14d8], esi    in FUN_140d225f0   0x055F clears it
140d23082  mov dword ptr [rcx + 0x14d8], r14d   in FUN_140d23030   0x0560 clears it too
140d258ff  mov dword ptr [r15 + 0x14d8], r14d   in FUN_140d23fb0   CreateLayout clears it
```

So the dialog is dead **until the next `0x055F` or `0x0560`**, not for the session: closing
the shop and clicking the NPC again recovers it. That is a real difference from the inventory
case and it should not be overstated in either direction - it is still an invisible failure
the player has no reason to try to fix.

### 4.3 Sub-op 2, closing **[L]**

`FUN_140d23ed0` sends it and calls the UI teardown `FUN_14177fef0` on the next instruction -
it does not wait. `FUN_140d225f0`'s zero-row arm sends it having never built a dialog.
**Nothing latches, so no answer is required**; the server should drop its own shop state and
send nothing. That is a narrow exemption from "always answer", the same shape as the one
`net::script` records for `0x0151`, and it rests on the teardown being unconditional rather
than on an absence of evidence.

---

## 5. Wire it like this

`crates/world/src/session.rs` is the coordinator's file; none of this is wired.

**Two pieces of state per session**, because the client only ever hands back a row key:

```rust
// the npc template whose shop is open, and the rows exactly as they went on the wire
open_shop: Option<(u32, Vec<net::shop::ShopRow>)>,
```

### On an NPC click

`net::script::parse_npc_click` already gives the object id; map it to a template the way the
placeholder dialogue path already does. Then:

```rust
if let Some(shop) = shops.by_npc(npc_name_for(template)) {
    let mut rows = Vec::new();
    // Buy rows: the price is AUTHORED, data/shops.txt, never ItemData::price.
    for it in &shop.items {
        if it.min_grade.is_some() { continue; }          // goal H's gate, until it exists
        rows.push(ShopRow::buy(rows.len() as u32, it.item_id, it.buy_price,
                               max_per_purchase(&shops, it.item_id)));
    }
    // Sell rows: WITHOUT THESE THE SELL TAB IS EMPTY. The price is ItemData::price,
    // which is info/price, which is the sell price - and it goes on the wire negated,
    // which ShopRow::sell does.
    for it in &shop.items {
        let d = match shops.item_data.get(&it.item_id) { Some(d) => d, None => continue };
        if !d.may_be_sold() { continue; }                // the owner's quest-item rule
        rows.push(ShopRow::sell(rows.len() as u32, it.item_id, d.price));
    }
    self.open_shop = Some((template, rows.clone()));
    out.push(packet(net::shop::OPEN_SHOP, &net::shop::open_shop(template, &rows)));
} else {
    // unchanged: the existing 0x055B placeholder dialogue
}
```

Three notes on that block. **`rows` must never be empty** - §2.2 - so a shop that resolves
to nothing should fall through to the dialogue path instead. The sell rows should really be
every item the player might carry that this NPC buys, not only the ones it stocks; stocking
is what the transcription has, so it is the honest starting point. And
`max_per_purchase` needs a helper, because **`ItemData::slot_max` is zero for 2495 of the
2785 rows in `gm-handbook/itemdata.txt`** - Red Potion included - and zero on the wire makes
every quantity fail (§2.6):

```rust
/// Row offset 29. **[L]** that the client uses it as the maximum in its "How many?" box and
/// rejects anything above it. The numbers below are a **[I]** server policy, not a client
/// fact: the client's `info/slotMax` is absent on 2495 of 2785 items, so it cannot supply
/// this on its own, and 0 is the one value that must never go out.
fn max_per_purchase(shops: &ShopTable, item_id: u32) -> u16 {
    match shops.item_data.get(&item_id).map(|d| d.slot_max) {
        Some(n) if n > 0 => n,                       // [L] from info/slotMax
        _ if item_id / 1_000_000 == 1 => 1,          // an equip: one at a time
        _ => 100,                                    // [I] a policy number, in ONE place
    }
}
```

### On `0x0104`

```rust
net::shop::CLIENT_SHOP_REQUEST => match net::shop::parse_shop_request(body) {
    Some(ShopRequest::Reopen { npc_template_id }) => {
        // re-send the SAME rows, so the row keys stay valid
        if let Some((t, rows)) = &self.open_shop {
            if *t == npc_template_id {
                out.push(packet(OPEN_SHOP, &open_shop(*t, rows)));
            }
        }
    }
    Some(ShopRequest::Close) | None => { self.open_shop = None; }   // nothing is owed
    Some(ShopRequest::Other { .. }) => {
        // Not a transaction, so nothing is latched. Log it and answer nothing.
    }
    Some(ShopRequest::Transaction(t)) => { /* below */ }
},
```

`None` is only a body with no sub-op byte at all, and it latches nothing.

### On a transaction - **every arm ends in a `shop_result`**

```rust
let Some((template, rows)) = self.open_shop.as_ref() else {
    out.push(packet(SHOP_TRANSACTION_RESULT, &shop_result(ShopResult::Busy)));
    return;
};
let Some(row) = rows.iter().find(|r| r.row_key == t.row_key) else {
    // Busy re-asks for the list, which is the right recovery for "I don't know that row".
    out.push(packet(SHOP_TRANSACTION_RESULT, &shop_result(ShopResult::UnknownItem)));
    return;   // and then re-send OPEN_SHOP, because UnknownItem re-asks
};

let result = if row.is_buy_row() {
    // The same helper, for the same reason: slot_max is 0 on most items and a max_stack of
    // 1 would put every potion in its own bag slot.
    let max_stack = max_per_purchase(&shops, row.item_id);
    let qty = t.quantity.clamp(1, row.max_per_purchase);       // the client's number is a claim
    let cost = u32::from(qty) * row.price as u32;              // the SERVER's price, not the client's
    if store.mesos(character_id)? < cost {
        ShopResult::NotEnoughMesos
    } else {
        match store.buy_item(character_id, inv_type_of(row.item_id),
                             &Item::of(row.item_id, qty), max_stack, cost) {
            Ok(changed) => { /* send 0x0070 for `changed`, and the meso change */ ShopResult::Success }
            Err(StoreError::BagFull { .. }) => ShopResult::InventoryFull,
            Err(_)                          => ShopResult::Busy,
        }
    }
} else {
    let unit = row.price.unsigned_abs();                       // the SERVER's price again
    match store.sell_item(character_id, inv_type_of(row.item_id), t.slot, Some(t.quantity), unit) {
        Ok(_balance) => { /* send 0x0070 for the removal, and the meso change */ ShopResult::Success }
        Err(StoreError::ItemMayNotBeSold { .. }) => ShopResult::UnknownItem, // + re-send OPEN_SHOP
        Err(StoreError::SlotEmpty { .. })        => ShopResult::UnknownItem, // + re-send OPEN_SHOP
        Err(_)                                   => ShopResult::Busy,        // + re-send OPEN_SHOP
    }
};
out.push(packet(SHOP_TRANSACTION_RESULT, &shop_result(result)));
if result.rerequests() {
    out.push(packet(OPEN_SHOP, &open_shop(*template, rows)));
}
```

Three rules that the code above encodes and that are easy to lose:

1. **Never trust `t.quantity` or the client's arithmetic.** It computed
   `|price| * quantity` against its own meso count before sending, but that is its number
   and its balance.
2. **`ItemData::price` is the SELL price** - `research/shops.md` §4 - so a buy must use
   `ShopItem::buy_price` from `data/shops.txt` and a sell must use `ItemData::price`.
   `ShopRow::sell` negates for you; do not negate twice.
3. **`Success` still needs the `0x0070` and the meso change.** §3.2.

---

## 6. The one-variant test

One launch. The variant is **the shop packet**; nothing else in this change touches a byte
any previous run depended on.

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

`-SetFieldProbe` is not optional - without it `Session::handle` returns nothing for every
packet.

**Lucy is on map 1013, "Amherst Department Store"**, and they are the only NPC in the whole
generated table carrying template 21 (`gm-handbook/npcs.txt`: `1013, 21, -258, 84, ...`).
`data/shops.txt` calls their map "Maple Road (Beginner Zone)", which is the street label, not
the id.

The route is the one the 2026-08-20 capture already used, so it is known to work:
**`!map 1010`, then walk the `in02` portal into 1013.** In that capture the client survived
1013 and Lucy rendered as object id 1000 and was clicked twice - which is worth saying out
loud, because `STATUS.md` still lists "Amherst (map 1013) kills the client" as open work.
That is one capture, not a retraction; if it faults this time, the shop is not implicated
and the run has answered a different question.

```text
previous-runs/world-20260820-012657.log
05:23:01.285 -> 0x01A0 SET_FIELD  portal "in02" -> map 1013 portal 1
05:23:13.164 <- 0x00F2 CLIENT_NPC_CLICK  body e80300002c01b600ffffffff   (object id 1000)
05:23:13.165 -> 0x055B SCRIPT_MESSAGE    "This server has no dialogue for NPC template 21 yet."
```

**The owner: get into the world, `!map 1010`, walk the `in02` portal into Amherst Department
Store, and click Lucy. Then, in this order:**

| # | do | what to watch | what each outcome means |
|---|---|---|---|
| 1 | click Lucy | does a **shop window** appear, with a Buy and a Sell tab? | **A shop window** = `0x0560` is right end to end. **The old dialogue box** ("This server has no dialogue for NPC template 21 yet.") = the click never reached the shop branch; the NPC-name-to-template join is missing, §Unresolved 6, and no packet was sent. **A dialog box with our text instead of a window** = the row count went out as **zero**, §2.2. **A freeze or a fault** = the row is mis-sized; `world.log`'s last outbound line names the packet and the body length should be `7 + 31n` |
| 2 | read the **Buy** tab | are all **six** of Lucy's items listed, with the prices from `data/shops.txt` (Red Potion 50, Orange Potion 150, Apple 20, Egg 30, Orange 50, Pet Food 35)? | **Six rows, right prices** = the 31-byte row and the sign test are both right. **Fewer rows, or garbage names** = the row is the wrong width; count how many rendered before it went wrong and that names the offset. **Rows present but every one greyed out** = row offset 28 went out non-zero |
| 3 | switch to the **Sell** tab | is it **empty** or does it list what you are carrying? | **Empty** is the expected failure if §Unresolved 1 is wrong, and it is *also* what you get if the character is carrying none of Lucy's six items - so **buy a Red Potion at step 4 first, then come back to this step**. Anything listed = the negative-price rows land in `+0x340` and the intersection works |
| 4 | buy **one Red Potion** | the potion, the meso count, and then **click Buy again** | **Item arrives and mesos drop** = the whole loop works. **Nothing visible but the second click still opens the quantity box** = the `0x055F` went out and `Store::buy_item`/`0x0070` did not. **The second click does nothing at all, silently** = the `0x055F` never went out and the dialog is latched, §4.2 - that is the failure this whole module exists to prevent, and `world.log` will show the `0x0104` with nothing after it. Closing and re-clicking Lucy clears it, so the run is not lost |
| 5 | close the shop, click Lucy again | does it re-open? | Yes = sub-op 2 was handled and the server dropped its state cleanly |

**Do not** change anything else in the same run, and in particular do not re-enable mobs -
the shop dialog and the mob body have nothing in common, and a fault would be ambiguous.

Steps 1-3 are three disjoint observables on one packet, step 4 is the second packet, and
step 5 is the third. A failure at step 1 makes 2-5 unreadable; nothing else gates anything.

---

## 7. What I did NOT establish

* **The sell direction end to end** - §Unresolved 1. Every link is [L]; the chain is not.
* **What `row_key` originally was.** [L] that the client only echoes it and that its one
  other use (a hash map on the character at `140d2604b`) misses harmlessly. **[D]** that the
  server may therefore choose it. It could be a commodity serial, a list index, or a buy-limit
  handle, and nothing here discriminates.
* **The four `u32`s at row offsets 12, 16, 20, 24.** Read, stored, copied into a tooltip
  block, never otherwise consulted on any path traced. Sent as zero.
* **The trailing `u8` after the rows** (`140d23b96` -> `shopUI+0x14dc`). Read; nothing that
  consumes `+0x14dc` was chased. Sent as zero.
* **`0x0104` sub-op 3.** Shape [L], owner [L] (`FUN_14216a810`, a method on the singleton at
  `0x143AC8690`, called from `FUN_142163340` and `FUN_142163f00` after a yes/no box).
  Meaning not established, and whether it needs an answer is not established.
* **Whether `0x0560` may be sent while an NPC dialogue is open**, and whether the shop has an
  ordering hazard against `SetField` the way `0x055B` does. Untraced.
* **What `[shopUI+0x14c8]` distinguishes.** It is `0` or `1` and selects between the active
  tab's list and `+0x348`; who writes it was not chased. It does not change any byte on the
  wire.
* **How the Buy tab is rendered.** `+0x338` has only three references in the whole
  `0x140d2xxxx` range - the constructor, the destructor, and the filler - so the control that
  draws it reaches it through `FUN_140d2d040` rather than through a direct field reference.
  That is consistent, but it is an argument from an accessor rather than a traced draw call.
* **The meso-change packet.** Buying and selling both move mesos and nothing here sends that.
  `research/charstat-layout.md` has the field; it has never been exercised.
