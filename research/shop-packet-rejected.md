# `0x009E` decoded, and why the Lith Harbor potion shop kills the client

**2026-08-30.** The owner: *"clicking the potion-shop NPC in Lith Harbor kills my client"*, three
times on demand.

Markers: **[L]** read off this client's listing or a capture, **[D]** derived from two or more
[L] facts, **[I]** inferred.

---

## 0. The answer in four lines

**It is ours, and the client is behaving correctly.** **[D]**

`FUN_1404ba100`, the classic-shop row decoder, reads the field at row offset `+0x1c` as a
**`u16`** for an ordinary item and as **8 raw bytes into `row+0x40`** when the item id is a
throwing star or a bullet. `crates/net/src/classicshop.rs` writes the `i16` unconditionally,
`CLASSIC_ROW_LEN` is a fixed `157`, and **Mina and Luna both stock Subi Throwing Stars
(`2070000`)**. Each such row leaves the client **6 bytes short**; it reads off the end of the
packet, the read primitive throws with reason code **`0x26`**, and that is the `0x009E`.

Size was never involved. A 21 687-byte shop was fine and a 5 673-byte one dies.

---

## 1. `0x009E` is `CLIENT_PACKET_REJECTED`, and here is every field

The builder is **`FUN_142c4ef20`** (212 bytes), decompiled in full: **[L]**

```c
void FUN_142c4ef20(Conn *conn, u16 klass, u32 code)
{
  if (conn->lastLen - 1U < 0x1000) {          // 1 <= len <= 4096, else NOTHING is sent
    pkt = new Packet(0x9E);                   // FUN_1406ed520
    put_u16 (pkt, klass);                     // FUN_1406ed940
    put_u32 (pkt, code);                      // FUN_1406ed9d0
    put_u16 (pkt, (u16)conn->lastLen);        // FUN_1406ed940
    put_raw (pkt, conn->lastBuf, conn->lastLen);   // FUN_1406ede20
    put_u16 (pkt, conn->lastOpcode);          // FUN_1406ed940
    send    (pkt);                            // FUN_1415d01c0
    conn->lastLen = 0;                        // <- one report per packet, see §4
    destroy (pkt);
  }
}
```

`conn` is the singleton `DAT_143ac1898`; the three fields are `+0xe0` (`void* lastBuf`),
`+0xe8` (`u32 lastLen`), `+0xec` (`u16 lastOpcode`).

### The body, field by field

| offset | width | field | measured values |
|---|---|---|---|
| `0` | `u16` | **exception class** | `1` (a packet-read fault) or `3` (a client-logic fault) |
| `2` | `u32` | **reason code** | `0x26` for every class-1 event; `0x21000003` for the one class-3 |
| `6` | `u16` | **length** of the saved packet | `min(packetLen, 4096)` in practice |
| `8` | raw | **the offending packet verbatim**, its own 4-byte network header first, then the opcode, then the body | |
| `8+len` | `u16` | **the offending opcode**, again | `0x0453` on the one capture the log recorded whole |

The last row is [L] rather than [D]: the `0x0453` rejection is 26 bytes, which fits under
`world.log`'s 96-byte body cap, and its final two bytes are `53 04`.

**The previous reading of this header was wrong in one place and it matters.** The four
"varying" bytes at offset 8 are not a header field — they are the **first four bytes of our own
packet**, the length-encoded network header. Read as two `u16`, `hi ^ lo` is the packet's true
length, and that is how a truncated echo still tells you how long the original was. All six
archived events check out exactly:

```text
time          echoed  klass  code        len6   hdr hi^lo   full packet   note
00:56:36.998     168      1  0x26         158        154           154    0x007D
01:15:40.061     143      1  0x26         133        129           129    0x007E
13:59:26.210      26      1  0x26          16         12            12    0x0453
17:01:19.946    2075      3  0x21000003  2065       2061          2061    0x055E
03:15:06.286    4106      1  0x26        4096       5675          5675    0x055D  <- capped
02:02:08.600    4106      1  0x26        4096       5675          5675    0x055D  <- capped
```

`total = 2 + 4 + 2 + len6 + 2` on every row. **[L]**

**The `0x1000` is a gate, not a truncation rule.** `conn->lastLen - 1U < 0x1000` refuses to
report anything longer than 4096 at all, so a rejection of a packet whose *saved* length
exceeded 4096 would be silent. What we observe is `lastLen == 4096` for a 5 675-byte packet,
so something upstream clamps the saved copy to a 4 096-byte buffer. **[I]** — the clamp itself
was not traced, only its result.

### The reason code is named: `0x26` is "not enough bytes left"

It is a literal immediate inside the read primitives themselves. **[L]**

```asm
; the u16 reader, FUN_1406e8b80
1406e8bd9  cmp   edi, 0x2          ; bytes remaining
1406e8bdc  jc    0x1406e8bf2
...
1406e8bf2  mov   edx, 0x26         ; <<< THE REASON CODE
1406e8bf7  lea   rcx, [rsp + 0x28]
1406e8bfc  call  0x1401bb8b0       ; construct the reason object
1406e8c01  lea   rdx, [0x143a3b118]
1406e8c0d  call  0x142ef6d4c       ; _CxxThrowException
1406e8c12  int3                    ; <- the return address the hook's stack scan reports
```

The `u32` reader `FUN_1406e8c20` carries the byte-identical tail (`ba 26 00 00 00` at
`0x1406e8c91`, `int3` at `0x1406e8cb1`). **So `0x26` means exactly one thing: a decoder asked
for more bytes than the packet had left.**

Class 1 is stamped by a shim rather than by any of the seven direct callers, which is why a
`call`-only search for it comes back with nothing but 3s:

```asm
1406ebc10  mov  rcx, qword ptr [0x143ac1898]   ; the connection
1406ebc17  mov  r8d, edx                       ; the code
1406ebc1a  mov  edx, 0x1                       ; <- class 1
1406ebc1f  jmp  0x142c4ef20
```

Its nine callers are the identical catch funclets `FUN_142fa8090 … FUN_142fa8800`: each one
reads the code out of the caught reason object (`FUN_1406f1830`), sends the `0x009E`, and
rethrows `&DAT_143a3b118`. **[L]**

---

## 2. What refuses `0x055D`: one branch, and it is a length branch after all — per row

The hook's stack scan for the throw, in `previous-runs/maplecw-hook-20260829-231513.log` and
again in `previous-runs/maplecw-hook-20260830-220211.log`, is the same three text addresses
both times: **[L]**

```text
0x1406e8c12  int3 after the u16 reader's throw          FUN_1406e8b80
0x1404ba5a4  the instruction after the call that threw  FUN_1404ba100 + 0x4a4
0x141f9f81c  the return address of `call FUN_1404ba100` FUN_141f9f5e0 + 0x23c
```

`0x141f9f817` is the per-row `<item>` call `research/classic-shop-opcode.md` already
documents, so the return address `0x141f9f81c` is exact, not approximate.

And `0x1404ba5a4` is the instruction after the read at `0x1404ba59f`:

```asm
1404ba575  lea   rdx, [rdi + 0x40]          ; the destination for the 8-byte form
1404ba579  mov   qword ptr [rdx], rbp       ; row+0x40 = 0
1404ba57c  mov   dword ptr [rdi + 0x1c], ebp ; row+0x1c = 0
1404ba57f  mov   ecx, dword ptr [rdi + 8]   ; the itemId
1404ba582  lea   eax, [rcx - 0x1f95f0]      ; itemId - 2070000
1404ba588  cmp   eax, 0x2710                ;          < 10000 ?
1404ba58d  jb    0x1404ba5ac
1404ba58f  lea   eax, [rcx - 0x238d90]      ; itemId - 2330000
1404ba595  cmp   eax, 0x2710
1404ba59a  jb    0x1404ba5ac
1404ba59c  mov   rcx, rsi
1404ba59f  call  0x1406e8b80   <<< READ u16 ; ordinary item: 2 bytes -> row+0x1c
1404ba5a4  movsx ecx, ax                    ; <- THE THROW SITE
1404ba5a7  mov   dword ptr [rdi + 0x1c], ecx
1404ba5aa  jmp   0x1404ba5ba
>1404ba5ac mov   r8d, 8
1404ba5b2  mov   rcx, rsi
1404ba5b5  call  0x1406e9170   <<< READ raw ; star/bullet: 8 bytes -> row+0x40 (rdx, above)
>1404ba5ba mov   rcx, rsi
1404ba5bd  call  0x1406e8b80   <<< READ u16 ; row+0x10c, both paths
```

**`2070000..2079999` and `2330000..2339999` — throwing stars and bullets. 8 bytes against 2.**

This is not a new fact about the client; `crates/net/src/bag.rs` already carries the identical
pair of ranges, read off a *different* site (`0x1403044ed`, the item-slot decoder), as
`BUNDLE_SERIAL_RANGES` / `bundle_has_serial()`. Two independent sites, same two ranges. **[D]**

### It is the only content-dependent length in the row

Enumerated rather than searched: every conditional jump in `0x1404ba100..0x1404ba5d6` was
tested against every `READ` between it and its target.

```text
reads in FUN_1404ba100: 42        <- the control: classic-shop-rows.md §1 measured 42/42/42
conditional jumps that skip one or more reads:
  1404ba58d -> 1404ba5ac  skips 1404ba59f READ u16
  1404ba59a -> 1404ba5ac  skips 1404ba59f READ u16
```

**Named blind spot:** this covers only reads *inside* `FUN_1404ba100`. The `u8` at `0x1404ba1c0`
gates a call to the sub-decoder `FUN_1404b9480`, whose own reads are not in this listing. We
send `0` for that flag, so it does not run — but if that byte ever becomes non-zero the
enumeration above says nothing about it.

---

## 3. The discriminating census: nine sends, nine correct, no counterexample

Every `0x055D` this server has ever sent, event-deduplicated on `(timestamp, opcode, body)`
across `previous-runs/` and `research/fixtures/` and the live `world.log`, joined against
whether `data/shops.txt` stocks an item in either range:

| shop | items | rows | bytes | stocks a star/bullet | rejected |
|---|---:|---:|---:|---|---|
| Lucy (Grocer) ×3 | 6 | 12 | 1 905 | no | no, no, no |
| Flora the Fairy | 14 | 28 | 4 417 | no | no |
| Karl | 15 | 30 | 4 731 | no | no |
| Serabi the Fairy | 69 | **138** | **21 687** | no | **no** |
| **Mina (Grocer)** ×2 | 18 | 36 | 5 673 | **Subi Throwing Stars 2070000** | **YES, both** |
| **Luna (Grocer)** | 23 | 46 | 7 243 | **Subi Throwing Stars 2070000** | **YES** |

The predicate separates them perfectly and size does not separate them at all. Every packet is
`21 + 157 × rows` to the byte, so the header and the row length are both exactly right; the
rows are simply the wrong *shape* for two of them.

**13 of the 39 shops in `data/shops.txt` stock a star.** Mina, Luna, Len the Fairy, Arturo,
Dr. Faymus, Max (Wolbi, `2070001`), Valen, Mr. Sweatbottom, 24 Hr Mobile Store, Luma, Edel the
Fairy, Glibber, Hana. Every one of them is a client death waiting for a click.

### Two things in the original brief that do not survive

* **"The same 1905-byte shop opens twice in one session, once cleanly and once fatally."**
  It does not. The `0x009E` at `17:01:19.946` in `classic-shop-opens-sell-crashes-world.log`
  carries opcode **`0x055E`** — the type-10 buy-back refresh the owner's *sell* click produced 3.4 s
  after the shop had opened normally. The fixture's own name says so. Nothing about `0x055D`
  was ever non-deterministic. **[L]**
* **"Eight constant bytes."** Six of them are constant only because five of the six archived
  events happen to be the same exception class. See §1.

### And it is not the 08-29 code regression either

The window is real but coincidental: the four shops clicked before it are the four with no
star, and the two clicked after it are the two with one. `data/shops.txt` last changed on
**2026-08-19** (`e0bf9ab`), ten days before the window, and Mina has had Subi Throwing Stars
since the file was created (`30d5ad2`). `git log -S "Subi Throwing Stars" -- data/shops.txt`
lists no commit inside 08-28…08-30. **[L]** Neither `b770d0c` nor `837bab6` is implicated.

---

## 4. The fault at `0x140ce89d6` is a *second* event, and it is not deterministic

`0x140ce89c0` (113 bytes) is a refcounted-pointer release: **[L]**

```asm
140ce89c0  mov  qword ptr [rsp+8], rbx
140ce89ca  mov  rdi, rcx
140ce89cd  mov  rbx, qword ptr [rcx + 8]    ; this->ctrl
140ce89d1  test rbx, rbx
140ce89d4  jz   0x140ce8a26                 ; null is handled
140ce89d6  mov  rdx, qword ptr [rbx + 0x28] ; <<< FAULT A: read the refcount
140ce89de  cmp  rax, 0xffffe
140ce89e6  mov  ecx, 0x31e                  ; a range assert
140ce89f7  xadd.lock qword ptr [rbx+0x28], rax  ; <<< FAULT B: atomic decrement
140ce8a20  call qword ptr [rax]             ; the virtual destructor at zero
```

The null case is handled, so both faults mean `this->ctrl` is **non-null and not a valid
pointer** — a destructor running on a half-constructed or already-released object. Every
address in the fault's stack scan (`0x142efbede`, `0x142efba4e`, `0x142ef70d1`, `0x142efadb5`,
`0x142ef8dd9`, `0x142f04320`) is in the same CRT block as the unwinder addresses that appear in
every `C++ THROW` stack, and the fault timestamp equals the second throw's to the millisecond.
**[D] the fault happens inside the unwind of the same exception.**

But it does **not** always happen, and that is the part worth having. Across the six archived
**class-1** rejections, one count predicts the outcome perfectly, 3 of 3 each way: **[L]**

| when | rejected | throws | third throw from `0x141804bfe` | fault | what the owner saw |
|---|---|---:|---|---|---|
| 00:56:36 | `0x007D` | 3 | yes | none | kept playing |
| 01:15:40 | `0x007E` | 3 | yes | none | kept playing |
| **02:02:08** | **`0x055D`** | **3** | **yes** | **none** | client closed its own socket |
| 13:59:26 | `0x0453` | 2 | no | `0x140ce89d6` | died |
| 03:15:06 | `0x055D` | 2 | no | `0x140ce89d6` | died |
| **02:12:19** | **`0x055D`** | **2** | **no** | **`0x140ce89f7`** | died, dump written |

The seventh event, `0x055E` at 17:01:19, is **class 3** and belongs in no column of that table:
it is not a read underflow, it throws **once**, from the client-logic constructor family
directly, and it faulted at `0x140ce89d6` anyway. Counted as corroboration it would be a
mistake — what it corroborates is only that the fault is the generic escaped-exception death,
not that the throw count means anything outside class 1.

The third throw is the client's designed recovery: an outer handler converts the read fault into
a connection error and tears the socket down (`FUN_1415e3b60` ran at `22:02:11.747`). It sends
no second `0x009E` because `FUN_142c4ef20` zeroed `conn->lastLen` on the way past — one report
per packet, by construction.

So: **the rejection is deterministic and ours; the fault is a client bug that fires on roughly
half the unwinds and is not ours to fix.** Fixing the packet removes both, because neither
happens without the throw. `0x140ce89d6`/`f7` is the client's generic "an exception escaped a
packet handler" death — my census of 99 hook logs finds it behind six other deaths on
`0x0453`, `0x055E`, `0x0560`, a SetField and a channel migrate. **It is not shop-specific and it
is not the `0xC0000374` heap family.**

A full-memory dump of the Luna death exists:
`dumps\maplecw-crash-702484-c0000005-1.dmp`, 1.35 GB, written 22:12:20.

---

## 5. The patch

`crates/net/src/classicshop.rs` only. Not applied — this file is a report.

```rust
/// The client picks the field at this position **by item id**, at `0x1404ba57f`:
/// a star or a bullet takes 8 raw bytes into `row+0x40` (the recharge unit price),
/// everything else takes an `i16` into `row+0x1c` (the bundle quantity).
///
/// Writing the `i16` for a star leaves the client **6 bytes short for that row**. It
/// reads off the end of the packet, `FUN_1406e8b80` throws reason `0x26`, and the
/// client reports the whole packet back as `0x009E` and drops the connection.
/// Measured twice on Mina and once on Luna; see `research/shop-packet-rejected.md`.
pub const RECHARGEABLE_ROW_EXTRA: usize = 6;

pub fn wire_len(&self) -> usize {
    CLASSIC_ROW_LEN
        + if crate::bag::bundle_has_serial(self.item_id) { RECHARGEABLE_ROW_EXTRA } else { 0 }
        + if self.buy_back { BUY_BACK_ROW_EXTRA } else { 0 }
}
```

and, in `ClassicShopRow::write`, replace the single line `w.i16(self.bundle_quantity);` with:

```rust
    if crate::bag::bundle_has_serial(self.item_id) {
        // +0x40, 8 raw bytes. The client pre-zeroes this slot at 0x1404ba579, so zero is
        // what it already expects, and `crates/world/src/session/shop.rs`'s claim that the
        // Recharge arm is unreachable stays true. `bundle_quantity` has nowhere to go on
        // this path - the client leaves `row+0x1c` at the 0 it wrote at 0x1404ba57c.
        w.u64(0);
    } else {
        w.i16(self.bundle_quantity); // +0x1c
    }
```

`bundle_has_serial` is the existing `crates/net/src/bag.rs` predicate over
`BUNDLE_SERIAL_RANGES`; it already has the tests that pin both range edges. Reusing it rather
than writing a second copy is the point — the two sites decode the same client rule.

**Predicted wire lengths after the change**, and these are falsifiable on the next launch:
Mina `5673 -> 5685`, Luna `7243 -> 7255` (`+6` per star row, and each star is two rows, buy
and sell). Everything else is unchanged to the byte.

### The zero-risk stopgap, if a wire change is not wanted today

Drop rechargeable rows when the shop table is built. One predicate, no new wire format, and it
takes 13 shops from "kills the client" to "sells no stars". It is strictly worse as an outcome
and strictly safer as a change.

### Confidence

| claim | marker |
|---|---|
| `0x009E`'s five fields and their offsets | **[L]** — decompiled builder, and all six archived bodies satisfy `2+4+2+len+2` |
| reason code `0x26` = "not enough bytes left" | **[L]** — a literal in two read primitives |
| class `1` comes from the shim at `0x1406ebc10` | **[L]** |
| the trailing `u16` is the offending opcode | **[L]** — one complete capture (`0x0453`) |
| the `4096` is a report gate, and something upstream clamps the saved copy | **[L]** gate, **[I]** clamp |
| the rechargeable branch is what refuses `0x055D` | **[L]** listing + throw stack, **[D]** joined to the 9-send census |
| it is the only variable-length branch in the row | **[L]**, with the sub-decoder blind spot named in §2 |
| the fault is inside the unwind of the same exception | **[D]** |
| whether the fault fires is not deterministic | **[L]** — 3 throws vs 2 throws, 7 of 7 |
| the patch is correct | **[D]** — the field, its width and its destination are [L]; that `0` is the right *value* is **[I]**, from the client pre-zeroing the same slot |

---

## 6. Housekeeping for whoever reads the fixtures

**`research/fixtures/shop-055D-rejected-009E-client-fault-hook.log` is not the hook log of the
run in the `-world.log` beside it.** The world capture's shop click is at world `02:02:08.593`;
the hook fixture starts at `22:02:53.487`, which is **45 s later** — it is the *next* launch.
The matching hook log is `previous-runs/maplecw-hook-20260830-220211.log`, and it is the more
interesting of the two because it is the `0x055D` rejection that did **not** fault.

Its name also mis-states the finding: that capture has no client fault in it.

Three captures were copied out of the rolling buffer, named for what they prove:

| fixture | what it settles |
|---|---|
| `shop-055D-rejected-three-throws-NO-fault-hook.log` | the `0x055D` rejection that did **not** fault — the only one, and the reason §4 says the fault is a second event |
| `shop-055D-rejected-two-throws-fault-140ce89d6-{hook,world}.log` | the matched pair for the one that did: the send, the `0x009E`, both throws and the fault, in two files with the 4-hour clock offset |

**Still only in the live logs and not yet preserved:** the Luna death at world `02:12:19` /
hook `22:12:19`, which is the third `0x055D` rejection and the one that faulted at
`0x140ce89f7`. It is in `world.log` and `client-patched\maplecw-hook.log` while the owner is
playing; copy it out after the session rather than mid-write.

**The world log's clock runs four hours ahead of the hook's** (world `17:01:19.946` = hook
`13:01:19.946`, and again `03:15:06.286` = `23:15:06.286`). Minutes and seconds align exactly;
only the hour differs. Anyone correlating the two files by timestamp needs that, and it is why
the 4106-byte rejection looks like it has no hook log at all.
