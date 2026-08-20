# Why CH.2 cannot be selected

Static analysis only, **no Ghidra** (another session held the project lock). Everything here
came from `capstone` over `client-patched/MapleStory.exe`, the repo's `tools/xref.py`,
`tools/callers.py`, `tools/dump_va.py`, a displacement scanner written for this task, and
`wz-dump` over `UI_000.wz` / `_Canvas_000.wz`.

Labels: **[L]** read from the listing / a capture / the WZ. **[D]** derived from two or more
[L]. **[I]** inferred, including anything from the v214 reference.

---

> **SETTLED 2026-08-19 (later). Read §9 first.** The list is empty because the client
> **never rebuilds it**: the login result's `(worldId, channelId)` is compared against the
> pair the client already holds, and `(0, 0)` == `(0, 0)` skips the only function in the
> image that fills the row array. Sections 1-3 below describe a byte that only matters
> *after* §9's fix lands, and §0's "next instrument" advice was aimed one link too far
> downstream.

---

## 0. RETRACTED 2026-08-19 by a client run

**The conclusion below is wrong as stated, and the experiment it recommended made things
worse.** Section 1 said the 4th trailing `u8` is the row's enable flag and that any non-zero
value would light the row up. It was set to `1` and the Change Channel dialog came back
**completely empty** - no rows at all, where `0` had listed CH.1 and CH.2.

The count reached the client either way: `login.log` for that run shows
`channel 0 advertised`, `channel 1 advertised`, `world Scania id 0 with 2 channel(s)` and the
`0x000B` world list going out. So the byte emptied a list that had populated without it.

| 4th trailing `u8` | what the owner saw |
|---|---|
| `0` | CH.1 and CH.2 both listed. CH.2 grey, and clicking it sends **nothing at all** |
| `1` | **no channels listed** |

`crates/net` is back to `0`.

**What is probably still right, and what is not.** Every individual link below was read off
the listing and none of them is contradicted: `FUN_142cb9510` really is a 15-byte leaf
returning `arr[i]` from `singleton+0x2cc8`, its six callers really do test the result with a
bare `test eax, eax`, and the mouse gate really does `je` on zero. What the run falsifies is
the **end-to-end claim** that the 4th wire byte reaches that array as an enable flag. Some
consumer *between* the decoder and the array drops the entry when the byte is non-zero - a
byte that hides a channel does exactly this, and so would a length or a type field.

> **The next instrument is upstream, not downstream.** Read what `FUN_141b2c7c0` does with
> `chan+0x18` **before** it reaches `FUN_142cb8e10`'s argument 6, and specifically look for
> any test that skips the entry. The failure mode is "the list is built shorter", not "the
> row is drawn grey", and nothing in this document looked for that.

**The lesson is the one `CLAUDE.md` already states, in a new dress.** Every link in a chain
can be read correctly and the chain still not mean what it was taken to mean. Section 2's
six-callers-all-`test eax,eax` finding is genuinely strong evidence about *that array*, and
it was carried across a gap - the array's provenance - that was never checked as hard.

---

## 1. The answer in one line - **RETRACTED, see section 0**

**The 4th (last) trailing `u8` of each channel entry in the world list is the row's
enable flag. It must be non-zero. We send `0`.** **[D]**

The client copies that byte into a per-channel `int` array and the Change Channel dialog
refuses both the click and the normal artwork when it reads zero.

---

## 2. The predicate

`FUN_142cb9510` — 15 bytes, a leaf with no `.pdata` entry, so `tools/callers.py`-style
attribution misses it; it was found by displacement scan. **[L]**

```
000142cb9510  85 d2                    test edx, edx
000142cb9512  78 18                    js   0x142cb952c          ; idx < 0  -> 0
000142cb9514  48 8b 89 c8 2c 00 00     mov  rcx, [rcx + 0x2cc8]  ; <-- the array
000142cb951b  48 85 c9                 test rcx, rcx
000142cb951e  74 0c                    je   0x142cb952c          ; null     -> 0
000142cb9520  3b 51 f8                 cmp  edx, [rcx - 8]       ; length at [-8]
000142cb9523  73 07                    jae  0x142cb952c          ; oob      -> 0
000142cb9525  48 63 c2                 movsxd rax, edx
000142cb9528  8b 04 81                 mov  eax, [rcx + rax*4]   ; return arr[idx]
000142cb952b  c3                       ret
000142cb952c  33 c0                    xor  eax, eax
000142cb952e  c3                       ret
```

`rcx` is the singleton at `.data 0x143aa84a0`. Six call sites, **all six** test the result
with a bare `test eax, eax` — there is no enum, no magic value. **Any non-zero int
enables the row.** **[L]**

| caller | role |
|---|---|
| `0x142a308a0` @ `0x142a312b9` | Draw — picks the row bitmap |
| `0x142a31750` @ `0x142a31810` | **the mouse hit test** — gates the click |
| `0x142a316e0` @ `0x142a31712` | the Change/OK button — gates the send |
| `0x142a31c60`, `0x142a31d30`, `0x142a31e50` | keyboard arrow / page navigation — skip disabled rows |

### The mouse gate

`FUN_142a31750` sits at `0x1434879d8`, which is slot 3 of the **second** vtable (that vtable
starts at `0x1434879c0`, written to `[object+8]` by the constructor), so its `this` is
`object + 8` — hence the `[rdi-8]` vtable loads. Field names below are given at the **real
object base** (`realThis`), which is what the constructor and Draw use.

```
000142a31764  44 8b 99 98 02 00 00     mov r11d, [rcx + 0x298]   ; realThis+0x2a0 = channel count
...            ; hit-test loop: row = i/6, col = i%6
000142a317aa  44 6b c8 46              imul r9d, eax, 0x46       ; x0 = 9 + 70*col
000142a317a7  6b ca 15                 imul ecx, edx, 0x15       ; y0 = 67 + 21*row
000142a317ba  41 8d 41 44              lea eax, [r9 + 0x44]      ; width  68
000142a317c8  8d 41 14                 lea eax, [rcx + 0x14]     ; height 20
...
000142a317dc  83 bf a0 02 00 00 00     cmp dword [rdi+0x2a0], 0  ; realThis+0x2a8
000142a317e3  75 15                    jne 0x142a317fa           ; skip gate 1
000142a317ea  e8 a1 7c 28 00           call FUN_142cb9490        ; the +0x2cc0 array
000142a317f1  75 61                    jne 0x142a31854           ; NON-zero -> blocked
000142a317fa  48 85 f6                 test rsi, rsi
000142a317fd  74 55                    je  0x142a31854           ; singleton null
000142a317ff  85 db                    test ebx, ebx
000142a31801  78 51                    js  0x142a31854           ; no row hit
000142a31803  3b 9f 94 02 00 00        cmp ebx, [rdi + 0x294]    ; realThis+0x29c = CURRENT channel
000142a31809  74 49                    je  0x142a31854           ; current row excluded
000142a31810  e8 fb 7c 28 00           call FUN_142cb9510        ; the +0x2cc8 array
000142a31815  85 c0                    test eax, eax
000142a31817  74 3b                    je  0x142a31854           ; ZERO -> nothing happens
000142a31819  81 fd 01 02 00 00        cmp ebp, 0x201            ; WM_LBUTTONDOWN
000142a3182b  89 9f 90 02 00 00        mov [rdi+0x290], ebx      ; realThis+0x298 = selection
000142a31839  81 fd 03 02 00 00        cmp ebp, 0x203            ; WM_LBUTTONDBLCLK -> act
```

The hit rectangle is **68 x 20**, which is exactly the size of the `channelN` bitmaps in the
WZ (see §4). That is an independent confirmation that this is the row hit test. **[D]**

The `je 0x142a31854` at `0x142a31817` is the whole reported symptom: the click is swallowed
before anything reaches the socket. **[D]**

---

## 3. How the byte gets there

Four links, each read from the listing.

### 3.1 Decode — `FUN_141b2fac0`, the login stage's `case 0xb`

Per-channel loop at `0x141b30060`–`0x141b300f8`. `rbx` = the channel struct. **[L]**

```
141b30077  call FUN_1406e9050   string  -> chan+0x00   name
141b300b2  call FUN_1406e8c20   u32     -> chan+0x08   userCount
141b300bd  call FUN_1406e8ae0   u8      -> chan+0x0c   (we send world_id)
141b300cb  call FUN_1406e8ae0   u8      -> chan+0x10   (we send index)
141b300d9  call FUN_1406e8ae0   u8      -> chan+0x14   (we send 0)
141b300e7  call FUN_1406e8ae0   u8      -> chan+0x18   (we send 0)  <-- the enable byte
```

`0x1406e8ae0` = u8, `0x1406e8c20` = u32, `0x1406e8b80` = u16 — corroborated independently by
`research/msexe-packet-readers.c`, which was produced by a different instrument. **[D]**
Each reader advances the cursor, so read order is struct order. **[L]**

### 3.2 Transfer — `FUN_141b2c7c0`

Builds three vectors from the selected world's channel list and hands them to the singleton.
`FUN_141b41c00(vec, i)` returns `&chan[i]`; `FUN_141b41ba0(vec, i)` returns `&out[i]`.

```
141b2ca44  call FUN_141b41c00
141b2ca49  8b 40 14        mov eax, [rax + 0x14]
141b2ca5c  call FUN_141b41ba0   with rcx = &[rsp+0x80]   ; vecA[i] = chan[i]+0x14

141b2ca84  call FUN_141b41c00
141b2ca89  8b 40 18        mov eax, [rax + 0x18]
141b2ca99  call FUN_141b41ba0   with rcx = &[rsp+0x78]   ; vecB[i] = chan[i]+0x18
```

**These are the only two fields of the channel struct this function reads.** Enumerated, not
filtered: extracting *every* memory operand in the function and dropping the `rsp`-based ones
leaves exactly `[rax]` x4 (world id, string source, and the two vector-slot writes),
`[rax+0x14]` x1 and `[rax+0x18]` x1 — nothing else. `userCount` (`+0x08`), `+0x0c` and
`+0x10` are never transferred to the singleton here. **[L]**

Call site:

```
141b2cab7  lea rax, [rsp+0x78]
141b2cabc  mov [rsp+0x28], rax     ; arg6 = vecB
141b2cac1  lea rax, [rsp+0x80]
141b2cac9  mov [rsp+0x20], rax     ; arg5 = vecA
141b2cace  lea r9,  [rsp+0x88]     ; arg4 = names
141b2caed  call FUN_142cb8e10
```

### 3.3 Store — `FUN_142cb8e10` (single caller: the above)

Prologue adjusts `rsp` by `0x68` (`push rdi` + `sub rsp,0x60`), confirmed by the epilogue
restoring `rbx` from `[rsp+0x70]` (= entry `[rsp+8]`). So callee `[rsp+0x90]` = arg5 and
`[rsp+0x98]` = arg6. **[D]**

```
142cb8e9b  mov rcx, [rsp+0x90]        ; arg5 = vecA
142cb8ea6  mov [rdi + 0x2cc0], rax
142cb8ece  mov rcx, [rsp+0x98]        ; arg6 = vecB
142cb8ed9  mov [rdi + 0x2cc8], rax    ; <-- the enable array
142cb8e25  mov [rcx + 0x2258], edx    ; world id
142cb8e38  mov [rcx + 0x2260], r8d    ; current channel index
```

### 3.4 Read — `FUN_142cb9510` reads `+0x2cc8`. Chain closed. **[D]**

```
wire byte 4 of the channel entry
  -> chan+0x18                (FUN_141b2fac0)
  -> vecB[i]                  (FUN_141b2c7c0)
  -> singleton+0x2cc8         (FUN_142cb8e10)
  -> FUN_142cb9510(i)         -> must be non-zero
```

---

## 4. Which row is actually grey — confirmed, not assumed

The dialog loads four sibling canvases in its OnCreate `FUN_142a2e7a0` at
`0x142a2f9b5`/`fa1c`/`fa8c`/`fafc`, keyed by wide strings at `0x1432b2288`…`0x1432b22d0`:
`channel0` -> `+0x2b0`, `channel1` -> `+0x2b8`, `channel2` -> `+0x2c0`, `channel3` -> `+0x2c8`. **[L]**

`wz-dump cat UI_000.wz ChannelChange.img` shows exactly those four nodes under `/Channel`,
alongside `ch/0..29`, `world/{0,1,80,111}` and the three buttons. **[L]**
The bitmaps themselves are `_outlink` stubs; the real pixels are in
`UI/_Canvas/_Canvas_000.wz`. (Rendering the `UI_000.wz` copies returns four 1x1 fully
transparent images — a textbook silent negative. The instrument only speaks after
following the outlink.)

Rendered via `wz-dump canvas` + `tools/wz_png.py`, all four are **68 x 20**: **[L]**

| node | avg RGB of opaque pixels | reading |
|---|---|---|
| `channel0` | (224, 222, 212) | cream — normal / selectable |
| `channel1` | (138, 138, 138) | grey — "this is the channel you are on" |
| `channel2` | ( 56, 168, 219) | blue — selected / highlighted |
| `channel3` | (132, 131, 129) | grey — **disabled** |

Draw (`FUN_142a308a0`) picks in this order: **[L]**

```
142a31168  cmp r12d, [r15+0x298]   ; i == selection      -> channel2  (blue)
142a31287  cmp r12d, [r15+0x29c]   ; i == current        -> channel1  (grey)
142a31294  cmp dword [r15+0x2a8],0
142a312a5  call FUN_142cb9490      ; +0x2cc0 non-zero    -> channel1  (grey)
142a312b9  call FUN_142cb9510      ; +0x2cc8 == 0        -> channel3  (grey, DISABLED)
                                   ; else               -> channel0  (normal)
```

The constructor `FUN_142a2dcc0` sets `[+0x298] = -1`, then OnCreate does
`0x142a30662: mov eax,[r13+0x29c] / mov [r13+0x298],eax` — **selection := current channel**. **[L]**

So on open with 2 channels and the character on our channel 0:

- **CH.1 draws `channel2` — blue/highlighted, not grey.** It is also excluded from clicking
  (`cmp ebx,[rdi+0x294] / je`), but that exclusion is not what the owner is seeing.
- **CH.2 draws `channel3` — the disabled art**, because `singleton+0x2cc8[1] == 0`.

**Answer to "is the current channel excluded?": yes, it is — but that is not the cause.**
The grey row is CH.2 and it is grey for the disabled reason, not the current-channel reason.
The two greys are nearly the same colour (138,138,138) vs (132,131,129), which is why the
distinction cannot be made by eye. **[D]**

Field map of the dialog, for anyone reading this code again:

| offset (real base) | meaning | set by |
|---|---|---|
| `+0x298` | selected row | ctor `-1`; OnCreate `= +0x29c`; mouse writes on LBUTTONDOWN |
| `+0x29c` | current channel | `FUN_142cb9260` = `singleton+0x2260` |
| `+0x2a0` | channel count | `FUN_142cb92a0` = length of `singleton+0x2cb8` (the name array) |
| `+0x2a4` | world id | `FUN_142cb9230` = `singleton+0x2258` |
| `+0x2a8` | `singleton+0x2250 >= 0x12` | OnCreate `0x142a2e811` |

Internal consistency check: `FUN_142a316e0` sits in the **first** vtable (`this` = real base)
and uses `+0x298`/`+0x29c`; `FUN_142a31750` sits in the **second** (`this` = base+8) and uses
`+0x290`/`+0x294`. Same two fields, two different `this` conventions, both agreeing. **[D]**

---

## 5. The one-variant experiment

In `crates/net/src/opcode.rs:205`, `world_list_entry` currently emits:

```rust
out.extend_from_slice(&[world_id, i, 0, 0]);
```

**Change the last byte to 1 and nothing else:**

```rust
out.extend_from_slice(&[world_id, i, 0, 1]);
```

Leave the third byte at `0`. The third byte feeds `FUN_142cb9490` / `singleton+0x2cc0`, and
**non-zero there blocks** the row (draws `channel1` grey, and refuses the click when
`+0x2a8 == 0`). Setting both would change two variables and could mask the result. **[D]**

Do **not** bother with `userCount` — §3.2 shows it is never transferred to the dialog's
singleton. STATUS's "cheapest experiment: send a non-zero user count" is the wrong lever.

### What to watch for

| observation | meaning |
|---|---|
| CH.2 turns cream instead of grey, single click turns it blue | the predicate is satisfied; row selection works |
| Double-click on CH.2, or click CH.2 then the Change button, produces a packet in `world.log` | the send path fired — expect opcode **`0x00D2`**, see §6 |
| CH.2 still grey | the predicate is not the only gate; go to §7 |

Note that a **single click only moves the highlight** (`WM_LBUTTONDOWN` -> `[+0x298] = i`).
Sending requires `WM_LBUTTONDBLCLK` or the Change button. Ask the owner to try both.

---

## 6. Bonus: the Change Channel opcode is `0x00D2`

STATUS section 2a step 4 lists this opcode as unknown. It is not.

`FUN_142a316e0` (the OK/Change handler, vtable slot 39) re-checks the same predicate and then
calls `FUN_141892840()` -> `FUN_1418287f0(obj, [rbx+0x298])`. **[L]**
`research/msexe-packet-fields.txt:86` already names `FUN_1418287f0` as the `0x00D2` builder;
what was missing was that this is the Change Channel action. **[D]**

Encode sequence at `0x141828925`–`0x14182896a`: **[L]**

```
1406ed520(pkt, 0xD2)         ; begin
1406ed840(pkt, (u8)edi)      ; u8  target channel  <- the dialog's 0-based row index
1429e3ef0() -> 1406ed9d0     ; u32
140c7b890(DAT_143ac8210, pkt); the 14-byte integrity preamble (see transfer-field-request.md §2.2)
1415d01c0(pkt)               ; send
```

The channel number on the wire is the **0-based** row index, matching this repo's numbering
and not the client's 1-based UI labels. **[L]**

Caveat: `0x00D1`'s builder calls `FUN_140c7b890` **first** (wire offset 0), while `0x00D2`
calls it **last**. So the byte layout above is read from call order only and has not been
checked against a capture. Treat the field *set* as [L] and the field *order* as [D].

Two further gates sit in front of the send, neither of which explains the current symptom
but both of which can silently swallow a click later:

- `FUN_142cc42d0(singleton, 500, 0)` at `0x14182882b` — a 500 ms rate limit plus two busy
  flags (`singleton+0x2330`, `+0x2338`), timestamp at `+0x2334`. **[L]**
- `FUN_142cb9490(target) != 0 && !(singleton+0x2250 >= 0x12)` at `0x14182883d` — the third
  trailing byte again. Another reason to leave it `0`. **[L]**

---

## 7. Ruled out, with the positive control for each instrument

Recorded so nobody re-walks this ground.

| claim | instrument | positive control | result |
|---|---|---|---|
| `userCount` is not the gate | full-function grep of `FUN_141b2c7c0` for every `[rax+N]` field read | the same grep finds `+0x14` and `+0x18`, which are the two we were looking for | only those two; `+0x08` never read |
| the 3rd trailing byte is not the gate for *enablement* | traced `+0x2cc0` to `FUN_142cb9490`, read all 3 callers | `tools/callers.py 0x1402fa9a0` -> 96 sites, 43 in `0x140304b20` (documented control) — passes | it is an inverted *block* flag, not an enable flag |
| the WZ has no `disabled`/`normal` sub-node pair per row | `wz-dump cat UI_000.wz ChannelChange.img` | the same dump lists `BtChange/{normal,pressed,disabled,mouseOver}`, so the tool *can* see that shape | rows use four flat `channelN` siblings instead — the state is picked in code, not by node name |
| `ChannelChange` as a string is not a dead end | `tools/xref.py --string ChannelChange` reported **0 code references** | — | **that zero was my own error**, not the tool's: `xref.py --va` wants the *string start*, and I fed it the offset of the matched substring. With the real starts (`0x143488d08` etc.) every one of them resolves inside `FUN_142a2e7a0`. Anyone repeating this must subtract the prefix length. |
| the four `channelN` bitmaps are not blank | `wz-dump canvas` + `tools/wz_png.py` on `UI_000.wz` | — | **false negative**: all four came back 1x1 fully transparent because they are `_outlink` stubs. The real pixels are in `UI/_Canvas/_Canvas_000.wz` and are 68x20. |
| leaf functions are not invisible to the displacement scan | scratch scanner, `.pdata` attribution + orphan decode | ran it on disp `0x2258`, whose known getter `FUN_142cb9230` is a 2-instruction leaf | the `.pdata` pass **missed** the getter; the orphan pass found `FUN_142cb9510` and `FUN_142cb92a0` the same way. A `.pdata`-only scan of this binary silently drops every leaf getter — worth adding to `docs`' blind-spot list. |

Not consulted: the v214 reference at `C:\Users\user\Desktop\ModernMapleSource`. Nothing here
needed it — the whole chain is [L] out of this binary — so no [I] candidates are carried.

---

## 8. If the experiment fails

The chain from wire byte to `test eax, eax` is complete and every link is read from the
listing, so a failure would most likely mean the byte is not reaching `chan+0x18` at all
rather than that the predicate is wrong. In that order:

1. **Check the world list actually re-decodes.** `FUN_141b2c7c0` has two callers,
   `FUN_141b307b0` and `FUN_141b32860`, both login-stage. The singleton arrays are filled
   from the **login** world list and survive into the game stage. If the client cached an
   older list, the new byte never lands. Re-login rather than reconnecting.
2. **Confirm the byte offset on the wire** with `tools/login_smoke.py` — it can parse the
   world list packet independently and would show whether byte 4 of the entry is where we
   think it is. This is cheaper than a client run.
3. **Runtime watch.** `crates/grap-stub`'s `-Probe watch@0x142cb9510` logs arguments and
   return address and does not care how any of this was encoded. That reads the array value
   directly and settles it in one launch.

---

# 9. SETTLED — the client never rebuilds the list, because we tell it nothing changed

Written 2026-08-19 after §0's three retractions. **No Ghidra** (another session may hold the
project lock): everything below is `capstone` over `client-patched/MapleStory.exe` through
`tools/callers.py`, `tools/reads.py`, `tools/pdata_lookup.py`, `tools/dataref.py` and two
scratch scanners (a displacement scan and a whole-image pointer scan), plus `wz-dump` and
`tools/wz_png.py` over `UI_000.wz` / `_Canvas_000.wz`.

## 9.1 The answer in one line

**`FUN_142cb8e10` — the only function in the image that fills the channel-row array — is
never called, because the login result tells the client it is already on the world and
channel it already thinks it is on.** **[D]**

## 9.2 The row count, closed end to end

```text
singleton+0x2cb8              the channel-NAME array   (null until FUN_142cb8e10 runs)
  -> FUN_142cb92a0            mov rax,[rcx+0x2cb8] / test / mov eax,[rax-8]   -> 0 when null
  -> dialog+0x2a0             set by OnCreate
  -> 142a30c51  cmp dword [r15+0x2a0], edi / jle 142a3165b     the row loop is SKIPPED
     142a3164e  cmp r12d, dword [r15+0x2a0] / jl 142a30c60     its back edge
```

**[L]** for every line. Inside that loop the geometry is `col = i % 6`,
`x = 9 + 0x46*col`, `y = 0x43 + 0x15*(i/6)` — the same 68x20 grid §2's mouse hit test uses,
which is what identifies it as the row loop. **[D]**

So a null name array does not draw a grey row, or a disabled row. It draws **nothing**, and
the dialog is left with only its frame, its buttons and its world logo. That is the owner's
screenshot exactly.

## 9.3 The world row is a WZ bitmap and proves nothing

The single row the owner reads as **"WINDIA"** is `ChannelChange.img/Channel/world/0`, rendered
out of `_Canvas_000.wz`. **[L]** — I rendered it. It is keyed on the world id in
`singleton+0x2258` (`FUN_142cb9230` -> `dialog+0x2a4`), and that field is `0` whether or not
anything ever wrote it.

**This kills the inference the task brief offered as a lead** ("the world row rendering
while channel rows do not is a clue about which data source is missing"). It is not a clue:
the world logo would render identically if the packet had never arrived. **[D]**

## 9.4 The only route into the filler, and the guard on it

Enumerated, not filtered. `tools/callers.py` was run with its documented positive control
first (`0x1402fa9a0` -> 96 sites, 43 in `0x140304b20`), and every step was cross-checked
with a whole-image scan for the 8-byte little-endian VA, which finds vtable and
function-pointer-table entries that `callers.py` cannot see.

| function | direct callers | in any pointer table? |
|---|---|---|
| `FUN_142cb8e10` (fills `+0x2cb8`/`+0x2cc0`/`+0x2cc8`) | **1** — `FUN_141b2c7c0` @ `141b2caed` | no |
| `FUN_141b2c7c0` | **2** — `FUN_141b307b0` @ `141b30f2f`, `FUN_141b32860` @ `141b32efc` | no |
| `FUN_141b307b0` | **1** — `FUN_141b25f30` @ `141b25ff9` | no |
| `FUN_141b32860` | **1** — `FUN_141b307b0` @ `141b307fd` | no |
| `FUN_141b25f30` | **0** | **yes**, `.rdata 0x1433fd7a0` — a vtable slot |

`FUN_141b25f30` is the **login stage's `OnPacket`**. Its jump table (byte index at
`0x141b26668`, dword RVAs at `0x141b265dc`, base `0x140000000`) decodes to opcodes
`0x00..0x5f`, and `case 0x0b` is `FUN_141b2fac0` — the world-list decoder §3.1 already
named, which is the independent check that this table is read correctly. **`case 0x10` is
`FUN_141b307b0`, and `0x0010` is `LOGIN_RESULT`.** **[L]**

`FUN_141b307b0` forks on the mode in its first eight instructions:

```text
141b307e6  mov rcx,[rip+0x1f910ab]      the session at 0x143AB4898
141b307ed  call FUN_142c4a810           = mov eax,[rcx+0x68]   the mode
141b307f2  cmp eax,5
141b307f5  jne 141b30807                the non-mode-5 body
141b307fd  call FUN_141b32860           MODE 5 -> here, then jmp to the epilogue
```

That is the *same* `session+0x68` test `opcode.rs` already documents for the world list, and
our client is mode 5 (`login.log`: `session identity: mode=5`). **So `FUN_141b32860` is the
live handler and everything in `FUN_141b307b0` after `141b30807` is dead code here.** **[L]**

The guard, at `141b32e8e`-`141b32efc`:

```text
141b32e8e  test r12d,r12d               r12d = the login RESULT byte, read at 141b328ad
141b32e91  jne  141b33ea9               non-zero result -> nothing below runs
141b32e9a  call 1406e8ee0               u8      (a THUNK to 1406e8ae0 - see 9.7)
141b32eb2  call 1406e9170  r8d=8        raw 8   -> FUN_1408f67d0, a FILETIME
141b32ec6  call 1406e8c20  -> esi       u32     worldId
141b32ed0  call 1406e8c20  -> ebx       u32     channelId
141b32edb  call 142cb9230              = mov eax,[rcx+0x2258]   the world it holds
141b32ee0  cmp  esi,eax
141b32ee2  jne  141b32ef1               differs -> rebuild
141b32ee8  call 142cb9260              = mov eax,[rcx+0x2260]   the channel it holds
141b32eed  cmp  ebx,eax
141b32eef  je   141b32f01               BOTH equal -> SKIP THE REBUILD
141b32efc  call FUN_141b2c7c0(this, esi, ebx, 0)
```

`rcx` at both getters is `[rbp+8]`, loaded at `141b3289f` from `[rip+0x1f75bfa]` =
**`0x143AA84A0`** — the same singleton §2 found from the dialog side. **[L]**

The **identical** guard sits at `141b30f0a`-`141b30f2f` in the non-mode-5 body, so it is
protocol semantics rather than a mode-5 quirk. **[L]**

## 9.5 What we send, and where it lands

`crates/login/src/session.rs:229,239` calls `login_result(world.id, world.channel_id, ..)`,
and both are `0` (`crates/login/src/config.rs:49`). `tools/reads.py 0x141b32860 2` gives the
ordered reads, and they line up with `opcode.rs::login_result` field for field:

| body offset | our builder | the client's read |
|---|---|---|
| 0 | `LOGIN_OK` | `141b328ad` u8 result |
| 1 | `str ""` | `141b328c4` str message |
| 3 | `0u8` | `141b32e9a` u8 (via the `1406e8ee0` thunk) |
| 4 | `0u64` | `141b32eb2` raw 8 |
| **12** | **`world_id`** | **`141b32ec6` u32 -> the world compare** |
| **16** | **`channel_id`** | **`141b32ed0` u32 -> the channel compare** |

The two `u32` at `141b32a41`/`141b32a4c` that `reads.py` lists earlier are behind
`cmp r12d,0x83 / jne` at `141b32a31` and are not on our path. **[L]**

## 9.6 Why `(0, 0)` is the pair the client already holds

**[D], from the client's own routines — and deliberately *not* claimed as [L].**

* `FUN_142ce9600` writes `0` to `+0x2258` and `+0x2260` and then nulls `+0x2cb8`, `+0x2cc0`
  and `+0x2cc8`. That is the client's own "no world, no channel, no list" state.
* The session teardown `FUN_142cad420` saves `+0x2258` into `+0x2264` and writes `0` over it
  (`142cad956`), and sets `+0x225c` to `-1`. It leaves `+0x2260` alone.
* The constructor `FUN_142ca5c50` nulls `+0x2c90`, `+0x2ca0`, `+0x2cb0`, `+0x2cb8`, `+0x2cc0`,
  `+0x2cc8`, `+0x2cd0` (`142ca6c8b`-`142ca6cb5`) and writes the singleton global at
  `142ca5c7d`. **It never writes `+0x2258` or `+0x2260`.**
* The object is `FUN_14019b780(pool, 0x41a8)`, which for a size over `0x80` goes to
  `FUN_14019d350` -> an imported allocator called with `edx = 0` — i.e. **no
  `HEAP_ZERO_MEMORY`**.

So on a fresh process those two ints are whatever the heap block held. Fresh pages are
zeroed by the OS, so `(0, 0)` is overwhelmingly likely and is what the observed empty dialog
requires — but it is **not guaranteed**, and that is the best available explanation for the
one run in which CH.1 and CH.2 appeared and never came back:

> **If the block is ever recycled memory, the compare fails and the list populates by
> accident.** A nondeterministic populate fits the history better than any of the three
> retracted stories: two "fixes" that each appeared to work once, and one that appeared to
> break it, all measured one run apart with the variable uncontrolled.

**And I could not reconstruct that run from the artefacts.**
`research/fixtures/world-select-0076-login.log` contains **one** `0x0076`, not the two
`STATUS.md` credits it with, and it contains **no `0x0078` at all** — that client never
entered the world, so the in-game dialog cannot have been observed in that session. Its
login-side counterpart for `channel-list-shows-two-but-unselectable-world.log` (17:21) was
not preserved. I am recording that as a gap rather than filling it with a story.

## 9.7 Instrument corrections

* **There is a ninth packet-read primitive, and it is a `u8`.** `0x1406e8ee0` is
  `jmp 0x1406e8ae0`. The list everyone works from carries the `u16` and `u32` thunks
  (`0x1406e8ef0`, `0x1406e8f00`) but not this one, and `tools/reads.py` does not report the
  read at `141b32e9a` because of it. It is the same failure as "five decoder addresses when
  there are seven": a known list, filtered rather than enumerated. `reads.py` should add it.
* **A displacement scan that decodes from a back-offset can name the wrong instruction.**
  My scanner reported `142cad957: mov [rdi+0x2258], esp`; the real instruction starts one
  byte earlier and is `142cad956: mov [rdi+0x2258], r12d`. It also reported the constructor's
  `mov qword [rdi+0x2cb8], rbx` as a `dword` write. **Use it to locate, then disassemble to
  read.**
* **`callers.py` cannot see a virtual.** `FUN_141b25f30` has zero call sites and is the whole
  login dispatcher. The pointer scan found it in one aligned `.rdata` qword. Any "0 callers"
  from `callers.py` must be followed by a pointer scan before it means anything.

## 9.8 ELIMINATED, with the evidence

| claim | how it was tested | result |
|---|---|---|
| an in-game packet populates the list | **enumerated** every instruction in the image whose 32-bit displacement is `0x2cb8` or `0x2cc8` (positive control: the scan finds both `142cb8eda`, a known write, and `142cb9515`, a known *leaf* read that `.pdata` attribution misses) | the only filler is `FUN_142cb8e10`; the only other touchers are the constructor and `FUN_142ce9600`, both of which **null** them. No packet handler anywhere writes them |
| the list is populated by a UI-owned packet dispatcher in the `0x226..0x276` family (`FUN_1429bafb0`), the way `0x0231` was found | the same enumeration — it covers the whole image, so it covers that family | no handler in any dispatcher can reach the array except through `FUN_142cb8e10`, which is called from one place. **Not walked opcode by opcode, because it does not need to be** |
| the client *requests* the list when the dialog opens | `world.log`, a live in-game session, has 25 distinct inbound opcodes and logs every unanswered one with its full body | The owner has previously measured that opening the dialog sends **nothing at all**, and nothing in this capture is timed to a dialog open. Consistent with §9.4: the data is a login-time cache |
| the world row proves the transfer ran | rendered `Channel/world/0` from `_Canvas_000.wz` | it is the **"WINDIA" bitmap**, drawn from world id `0`, which is also the value of an untouched field. Proves nothing either way |
| `FUN_141b2c7c0` will accept any world id | read its lookup: it walks `stage+0x100` comparing `world->[0] == arg2`, leaves the index `-1` when not found, and `FUN_141b44860` then bails at `141b2c88b` | **the world id must be one we advertised.** A sentinel world id silently does nothing |
| `FUN_141b2c7c0` will accept any channel index | read `FUN_141b20d80(world, channel)` at `141b2c8d4`: true only if `0 <= channel < len(world->[0x28])`, else `141b2c8e0: xor al,al; ret` | **the channel index must be valid for the world.** A sentinel channel silently does nothing either |

## 9.9 The fix, and why it is safe

Intersect §9.8's last two rows with "must not equal `(0, 0)`". With one world advertised as
id `0`, exactly one move is left: **send a channel index of `1` in the login result**, which
requires at least **two** channels advertised. `net::channel::priming_channel` is that rule,
with the `None` case for a one-channel world spelled out — with a single channel the only
valid index is `0`, which is the pair the client already holds, so a one-channel world
**cannot** populate this dialog at all.

**The wrong channel at login costs nothing**, because `SetField` corrects it: the `0x01A0`
handler `FUN_142097f80` reads a `u32` at body offset 8 (`142098065`) and passes it straight
to `FUN_142cb91e0`, the `+0x2260` setter, at `14209806f`. **[L]** That field is already the
real channel in `opcode::set_field_head`, and it is re-sent on every field entry, so the
dialog's "current channel" is right by the time it can be opened. The only window where
`+0x2260` is wrong is the character-select screen, which shows no channel.

## 9.10 What to watch on the run, and what each outcome means

`FUN_142cb8e10` and the tail of `FUN_141b2c7c0` have — as far as anything here can tell —
**never executed in this project**. Expect the unexpected from them, not just from the rows.

| observation | meaning |
|---|---|
| the dialog shows **two rows**, CH.1 and CH.2 | settled. §9 is right |
| CH.1 draws **blue** (it is `selection`, set to `current` by OnCreate) and CH.2 draws **grey** | expected, and §2/§4 explain it: CH.2 is grey because `singleton+0x2cc8[1]` is `0`. **That is the next variable, and it must be tested on its own run** |
| still **one row** | the guard is not the gate. Then the singleton was **not** `(0,0)` — put a watch on `0x142cb8e10` and read its `edx`/`r8d` |
| a fault or a freeze at the character-select screen | new code ran. `FUN_142cb8e10` also writes `[global+0x18c]`, `+0x31c4 = -1`, `+0x31cc = -1` and allocates a `0x58` object at `142cb8f15`; `FUN_141b2c7c0` ends with `FUN_1415e3cc0`/`FUN_14019a260`/`FUN_1415f3f10`. Read the `ELog` (`0x008F`/`0x0090`) first |

**One correction that follows from all of this, and that nobody should read as re-opening a
retraction.** §0 retracted the enable byte because setting it to `1` emptied a dialog that
`0` had populated. Under §9 the dialog is empty by default and populated only by accident,
so **both** of those observations are consistent with the byte having no effect at all. The
byte is therefore **un-measured, not disproven** — and it becomes live the moment rows exist,
because `FUN_142cb9510` returning `0` is what draws `channel3` and swallows the click. Do not
change it in the same run as §9.9. One variable at a time; that is the whole lesson of §0.
