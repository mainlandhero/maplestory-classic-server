# Why CH.2 cannot be selected

Static analysis only, **no Ghidra** (another session held the project lock). Everything here
came from `capstone` over `client-patched/MapleStory.exe`, the repo's `tools/xref.py`,
`tools/callers.py`, `tools/dump_va.py`, a displacement scanner written for this task, and
`wz-dump` over `UI_000.wz` / `_Canvas_000.wz`.

Labels: **[L]** read from the listing / a capture / the WZ. **[D]** derived from two or more
[L]. **[I]** inferred, including anything from the v214 reference.

---

## 1. The answer in one line

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
