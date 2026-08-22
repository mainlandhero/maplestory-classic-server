# `0x0467` is not a preload list, and the preload was never the problem

Written 2026-08-21. **No Ghidra** (another agent holds the project lock), **no client run**.
Everything below is `tools/listing.py`, `tools/reads.py`, `tools/callers.py`,
`tools/dataref.py`, `tools/pdata_lookup.py`, `tools/rtti.py`, two throwaway scripts, and
**two log files from one session** in `previous-runs/`.

Continues `research/npc-spawn.md`, `research/npc-appear.md` and `research/npc-fade.md`.
It does not contradict any of them; it corrects one line of `npc-spawn.md` §3.1 and adds
four eliminations and one measurement.

Markers: **[L]** read out of the image or a file on disk, **[D]** derived from those,
**[I]** inferred / candidate.

New artefacts written by this pass:

| file | what |
|---|---|
| `research/msexe-npc-0467.txt` | listings of the three functions that make up the `0x0467` body |
| `research/msexe-npc-template-getter.txt` | listing of `FUN_141e77b70`, the `Npc/%07d.img` loader |
| `research/msexe-npctemplate-parser.txt` | listing of `FUN_141e79060`, the 38 KB template parser |
| `research/msexe-npcpool-onpacket.txt` | listing of the NPC pool's `OnPacket`, both `.pdata` chunks |
| `research/msexe-npcdecode-listing.txt` | listing of `FUN_141e36b20`, the shared `0x044F`/`0x0451` decoder |
| `research/msexe-fieldload-lifewalk.txt` | listing of `FUN_141b7c960`, the map loader that walks `life` |

---

## 0. Answer up front

| question asked | answer |
|---|---|
| **Does `0x0467` touch the art?** | **Yes** — every entry goes through `FUN_141e77b70`, the same `Npc/%07d.img` loader `0x044F` uses, with the *same two arguments*. **[L]** |
| **Is it a template preload list?** | **No.** `npc-spawn.md` §3.1 read the entry as a bare id. It is four fields: `u32 templateId`, `str script`, `u32 dateStart`, `u32 dateEnd`, and the last two are compared against **today's date as `yyyymmdd`**. It is `SetNpcScriptable`. §1 **[L]** |
| **Would sending it fix the late first draw?** | **No, and this is measured rather than argued.** On a real field entry the `0x044F` handler — template load included — runs in **hundreds of microseconds**, on the very first entry to a map as well as on later ones. A cold parse of `Npc/%07d.img` through `FUN_141e79060` (38 506 bytes of code, ~117 property names) cannot fit in that. The art is already resident when `0x044F` arrives. §3 **[L]/[D]** |
| **Can the server send the NPCs earlier?** | **No.** The client is inside the `0x01A0 SetField` handler for **414–599 ms** on an ordinary field entry, emits `0x00DC` from *inside* it, and dispatches nothing until it returns. Both `0x044F`s are dispatched **1 ms after that return**, and each takes under half a millisecond. §4 **[L]** |
| Do mobs get a preload? | **Yes, the identical one**, from the same `life` loop, one branch away: `type=="n"` → `FUN_141e77b70`, `type=="m"` → `FUN_140495990`. A mechanism both share cannot be the difference. §2 **[L]** |
| So why are mobs instant and NPCs not? | **That comparison has never actually been made.** In the runs on disk the mobs either arrive **7.16 s after** field entry from the respawn tick, with no reference event to be late against, or arrive in the field-entry batch **on a map that has no NPCs**. The NPCs arrive **1 ms after the map finishes loading**, against the sharpest reference event in the game. §5 **[L]** |
| Is there anything to wire? | **Yes — a control, not a fix.** §8. |

---

## 1. `0x0467`, decoded properly

### 1.1 Routing

`FUN_141e75800` is the NPC pool's `OnPacket`. Its `0x0467` arm is short, and notably does
**not** pass the pool — the handler gets the packet and nothing else: **[L]**

```asm
141e75856  lea  eax,[rsi - 0x467]
141e7585c  test eax, eax
141e7585e  jne  141e75a87          ; not 0x467 -> common exit
141e75864  cmp  esi, 0x467         ; (the compiler emitted the test twice)
141e7586a  jne  141e75a87
141e75870  mov  rcx, rbx           ; rbx = the CInPacket, NOT the pool
141e75873  call 141e78110
141e75878  jmp  141e75a87
```

`FUN_141e78110` is `0x141e78110..0x141e781e7`, 215 bytes (`tools/pdata_lookup.py`). It takes
one argument, the packet. **[L]**

### 1.2 The body, field by field

Read counts cross-checked between `tools/reads.py` and `tools/listing.py`; both report the
same four reads at the same four addresses in the per-entry helper. **[L]**

```
u8  count
count x {
    u32  templateId          1402eef08   -> FUN_141e77b70(templateId, 0)
    str  script              1402eef17   u16 length + that many bytes
    u32  dateStart           1402eef58   yyyymmdd
    u32  dateEnd             1402eef63   yyyymmdd
}
```

Minimum entry: **14 bytes** with an empty string. Minimum packet: 1 byte (`count = 0`).

The helper `FUN_1402eeef0(out, packet)` writes an unaligned 0x14-byte struct —
`out+0 = templateId`, `out+4 = the string pointer` (8 bytes), `out+0xC` and `out+0x10` the
two dates. The caller passes `&[rsp+0x30]` and then reads `[rsp+0x30]`, `[rsp+0x34]`,
`[rsp+0x3c]`, `[rsp+0x40]`, which is the same struct at the same offsets. **[L]**

### 1.3 What it does with each entry

```asm
141e78131  call 1408f66e0          ; -> a 16-byte SYSTEMTIME for now; kept in xmm6
141e78145  call 1406e8ae0          ; u8 count
141e7815b  call 1402eeef0          ; one entry (the four fields above)
141e78166  call 141e77b70          ; ecx = templateId, edx = 0  -> the template
141e7817a  call 141e8b020          ; push a node onto the list at template+0x170
141e7818a  call 14019a260          ; node's string = the packet's script string
141e78193  mov  [rbx+8], ecx       ; node.dateStart
141e7819a  mov  [rbx+0xc], ecx     ; node.dateEnd
141e781ab  call 141e86bc0          ; template, now  -> pick the active node
```

`FUN_141e86bc0` is the tell: **[L]**

```asm
141e86bdf  movzx eax, word [rdx]      ; SYSTEMTIME.wYear
141e86be2  imul  r8d, eax, 0x64       ; * 100
141e86be6  movzx eax, word [rdx+2]    ; wMonth
141e86bea  add   r8d, eax
141e86bed  imul  ebp, r8d, 0x64       ; * 100
141e86bf1  movzx eax, word [rdx+6]    ; wDay   (SYSTEMTIME: +0 year, +2 month, +4 dow, +6 day)
141e86bf5  add   ebp, eax             ; ebp = yyyymmdd
...
141e86ca1  mov  eax, [rbx+8]          ; node.dateStart
141e86ca8  mov  ecx, [rbx+0xc]        ; node.dateEnd
141e86caf  cmp  eax, ebp
141e86cb1  jg   skip                  ; starts in the future
141e86cb3  cmp  ebp, ecx
141e86cb5  jle  accept                ; not yet expired
```

and the accepted node's string is stored at `template+0x178`. **[L]**

The template parser `FUN_141e79060` reads the property names **`dateStart`** and
**`dateEnd`** (`0x141e7e559`, `0x141e7e5e1`) as well as **`script`** (`0x141e801e8`), which
is the same triple in the same class. **[L]**

**So `0x0467` installs a dated script override on an NPC template.** It is the mscw analogue
of the v214 reference's `SET_NPC_SCRIPTABLE`, whose entry is
`{int nNpcId, String sScript, FT ftStart, FT ftEnd}` — the *shape* matches exactly, the
dates are `yyyymmdd` integers here rather than FILETIMEs, and the opcode number is ours.
Label the v214 corroboration a candidate per `CLAUDE.md`; every field above is read out of
mscw. **[L]**

### 1.4 The correction to `research/npc-spawn.md` §3.1

That table's row reads:

> `0x0467` | `u8 count`, then `count` template ids, each fed to `FUN_141e77b70` | a template preload list

The first half is right and the second half is wrong. There are three more fields per entry,
and the call to `FUN_141e77b70` is incidental — the handler needs the template object so it
can hang a script node off it. I have **not** edited that file; it belongs to another pass.

### 1.5 What `FUN_141e77b70` actually does — load, not reserve

`0x141e77b70..0x141e7810a`, 1434 bytes. `FUN_141e77b70(templateId, quiet)`: **[L]**

1. Hash-lookup `templateId` in a global template cache (`div` by the bucket count, chain on
   `[entry+8]`, key at `[entry+0x10]`, value at `[entry+0x20]`). **Hit → `lock inc` the
   refcount and return it.**
2. Miss, and `templateId != 0` → format `Npc/%07d.img` (`0x1432ACD10`) and `String/Npc.img`
   (`0x1432ADD50`), both dumped from `.rdata` and confirmed as UTF-16 literals, then call
   **`FUN_141e79060`**, `0x141e79060..0x141e826ca`, **38 506 bytes**.
3. Re-look-up the cache and return whatever landed there.

`FUN_141e79060` reaches **117 distinct string literals**, and they are the NPC's whole
authored definition: `info`, `link`, `stand`, `move`, `speak`, `imitate`, `hide`, `shop`,
`script`, `origin`, `action`, `type`, `hit_area/lt`, `hit_area/rb`, `skeleton`, `jsonLoad`,
`aniName`, `dateStart`, `dateEnd`, `Npc/%d.img/info/button/%d`, `Etc/NpcLocation.img/%d` …
(full list reproducible with the resolver in §9). **[L]**

**Not a slot reservation.** [D]

> **Blind spot, named.** I did **not** establish whether the canvas bitmaps are decompressed
> into textures inside `FUN_141e79060` or lazily on first draw inside `Gr2D_DX11.dll`. The
> string walk shows the *property tree* is parsed; it says nothing about texture upload. That
> gap does not move any conclusion here, because `0x044F` calls the identical function with
> the identical arguments (§3), so whatever `0x0467` would trigger, `0x044F` triggers ~200 µs
> later on its own.

---

## 2. The mob path gets the identical preload

`research/npc-spawn.md` §2 established that the client's map loader walks `life` and
preloads art. Re-read here from the listing, because the two halves sit **28 instructions
apart in one function** and that is the whole answer to "do mobs get a preload": **[L]**

```asm
; FUN_141b7c960, the map-image loader
141b7d92a  cmp  word [rcx], 0x6e        ; type == u"n"
141b7d92e  jne  141b7d99f
141b7d930  cmp  word [rcx+2], 0
141b7d935  jne  141b7d99f
141b7d937  xor  edx, edx                ; quiet = 0
141b7d939  mov  ecx, [rbp-0x28]         ; the life entry's id
141b7d93c  call 141e77b70               ; -> Npc/%07d.img
...
141b7d9a6  cmp  word [rax], 0x6d        ; type == u"m"
141b7d9aa  jne  141b7d9bc
141b7d9ac  cmp  word [rax+2], 0
141b7d9b1  jne  141b7d9bc
141b7d9b3  mov  ecx, [rbp-0x28]
141b7d9b6  call 140495990               ; -> Mob/%07d.img
```

Same loop, same `life` node, one `cmp` apart. **NPCs and mobs are preloaded by the same
walk, at the same moment, from the same data.** A mechanism both share cannot be what makes
one of them late. **[D]**

And the argument at the `0x044F` site is byte-for-byte the same as at the `life` site: **[L]**

```asm
141e759d6  call 1406e8c20     ; u32 templateId
141e759db  xor  edx, edx      ; quiet = 0   <- identical to 141b7d937 and to 141e78160
141e759dd  mov  ecx, eax
141e759df  call 141e77b70
```

So `0x044F`, `0x0467` and the client's own field loader all call `FUN_141e77b70(id, 0)`.
Three call sites, one function, one argument pair. **[L]**

---

## 3. The measurement that kills the preload theory

`client-patched\maplecw-hook.log` writes one dispatch line per packet **on handler return**,
with the handler's own elapsed time. Every `0x044F` ever logged: **[L]**

| session (`previous-runs/`) | `0x01A0` elapsed | the `0x044F`s that follow it |
|---|---|---|
| `maplecw-hook-20260821-134957.log`, 13:48:54 — **first entry of that launch, map 1** | 598 545.3 µs | **386.3 µs**, **368.7 µs** |
| same, 13:48:58 | 435 781.4 µs | 317.1 µs, 181.9 µs |
| same, 13:49:08 | 415 262.8 µs | 375.0 µs |
| `maplecw-hook-20260821-202047.log`, 20:11:08 — **first entry, map 1** | 586 061.3 µs | **413.0 µs**, **196.7 µs** |
| same, 20:11:24 | 537 069.5 µs | 190.0 µs, 133.5 µs |
| same, 20:12:21 (portal to map 10) | 434 426.0 µs | 220.9 µs |
| `maplecw-hook-20260821-212737.log`, 21:20:57 — **first entry, map 40** | 592 097.2 µs | **360.0 µs**, **319.3 µs** |

Across **every** `0x044F` dispatch line in `previous-runs/` and `research/fixtures/` the
handler runs in **113–1381 µs**. On the three **first-entry-of-a-launch** rows above — the
only cases where a cold template cache is even possible — the six values are 386.3, 368.7,
413.0, 196.7, 360.0 and 319.3 µs. That handler does: read the object id, hash the pool, allocate an entry, read the template
id, **call `FUN_141e77b70`**, attach, insert, and then run the entire 20-field
`FUN_141e36b20` body — displayer creation, `FUN_142b53fc0`, `CNpc::UpdateZ`, the appear
object, four COM interface fetches.

A cache **miss** inside `FUN_141e77b70` means running `FUN_141e79060`: 38 506 bytes of code
walking a WZ property tree with ~117 named children, plus the WZ read and decrypt path. For
scale, in the same log the field load — which is exactly that kind of work, at map scale —
takes **415 000–598 000 µs**. A cold NPC template parse does not fit in 197 µs, and the
second `0x044F` of every pair is the faster one (413.0 → 196.7, 190.0 → 133.5, 448.2 → 113.0)
— the shape of a cache that is already warm and getting warmer, not of one being filled.
**[D]**

**Conclusion:** by the time the first `0x044F` is dispatched, `Npc/0000001.img` is already in
the template cache, put there by the client's own `life` walk. `0x0467` would call the same
function on the same warm cache and return in a few microseconds. **There is no art load to
move earlier, so there is nothing for a preload packet to fix.** [D]

This is the honest form of the negative: not "I looked and found nothing", but "the work the
theory says is happening late is measured at under 200 µs, and it is warm".

---

## 4. The server cannot send the NPCs earlier, and that is measured too

**One session, two logs**, as `CLAUDE.md` requires — `previous-runs/world-20260821-202046.log`
and `previous-runs/maplecw-hook-20260821-202047.log`. The `mm:ss.mmm` fields align exactly
(`11:08.109` in one, `20:11:08.171` in the other), so the two files are directly comparable
within the session. First entry to map 1: **[L]**

| clock | source | event |
|---|---|---|
| `11:07.579` | world.log | server `-> 0x01A0 SET_FIELD`, map 1 |
| ≈`11:07.585` | derived | client **enters** the `0x01A0` handler (return minus 586.061 ms) |
| `11:08.045` | world.log | client `<- 0x0238 CLIENT_FIRST_FIELD_ENTRY` — **sent from inside the handler**, 460 ms in |
| `11:08.109` | world.log | client `<- 0x00DC CLIENT_FIELD_ENTERED` — **also from inside it**, 524 ms in |
| `11:08.109` | world.log | server `-> 0x044F` ×2, in reply, same millisecond |
| `11:08.171` | hook log | `0x01A0` handler **returns**, `elapsed_us=586061.3` |
| `11:08.172` | hook log | `0x044F` dispatched, `elapsed_us=413.0` |
| `11:08.172` | hook log | `0x044F` dispatched, `elapsed_us=196.7` |

Three things fall out, and all three are load-bearing: **[D]**

1. **The client's packet dispatch is serialised behind the SetField handler.** The NPC
   packets are on the wire at `11:08.109` and are not dispatched until `11:08.172`, 63 ms
   later, because the handler had not returned. Nothing arriving during those 586 ms gets
   looked at any sooner.
2. **Therefore no re-timing on the server helps.** Answering `0x0238` instead of `0x00DC`
   would put the bytes on the wire 64 ms earlier and change the dispatch time by **zero**.
   Sending the `0x044F`s *before* `0x01A0` would be worse — the pool is rebuilt on field
   entry — and would still not be dispatched first.
3. **Both NPCs exist, fully decoded, about a millisecond after the field finishes loading.**
   Whatever the owner is watching happens after that.

This retires the second row of `research/npc-fade.md` §9's outcome table — *"the next step is
*when* the packets are sent, not *what* is in them"* — as a server-side lever. The packets
are already as early as this client can take them.

---

## 5. The control that has never been run

`crates/world/src/session/gm.rs` states the working hypothesis plainly:

> *"every mob is sent `0x03C6` **and** `0x03D2`, and mobs are instant. NPCs have only ever
> been sent `0x044F`."*

The first clause of that is now dead (§6). The second half — **"mobs are instant"** — rests
on an observation that, in every run I can find on disk, was made against a **different event
at a different time**, or on a map with no NPCs to compare against. **[L]**

`previous-runs/world-20260821-212736.log` + `maplecw-hook-20260821-212737.log`, map 40, one
session:

| clock | what |
|---|---|
| `20:57.582` | server sends both `0x044F`s |
| `20:57.634` | hook: `0x01A0` returns after 592 ms |
| `20:57.635`, `20:57.636` | hook: the two `0x044F`s dispatch, 360 µs and 319 µs |
| `21:04.743` … `21:04.750` | server sends 30 `0x03C6` **MobEnterField: SPAWN** |
| `21:04.745` … | hook: they dispatch |

**7.16 seconds apart.** `crates/world/src/session/field.rs` says why: a brand-new field is
empty, every spawn point starts due, and the respawn tick fills them in. So the mobs the owner has
been calling instant arrived **seven seconds into a settled map with nothing to compare them
against**, while the NPCs arrived **one millisecond after the map finished loading**, against
the single sharpest reference event the game has.

The one run I can find where a mob *was* sent in the field-entry batch is the current
`world.log` + `client-patched\maplecw-hook.log` at `21:30:12`: `0x01A0` returns at
`.974` after 560 ms, and the first `0x03C6` dispatches at `.975` — **1 ms**, exactly like the
NPCs. That map (10000022) has no NPCs configured, so it was still not a comparison. **[L]**

**Nobody has ever watched an NPC and a mob created at the same instant.** That is the gap,
and it is the cheapest remaining thing to close.

---

## 6. Four eliminations this pass adds

### 6.1 "Mobs get a second packet and NPCs never do" is dead

`0x0451` with `flag != 0` on an id the pool **already holds** was never enumerated — both
previous passes read only the "unknown id" arm. Its listing, `FUN_141e75f50` at
`0x141e76100`: **[L]**

```asm
141e76100  mov   rbx,[rax+0x18]        ; the pool entry
141e7610d  movzx eax, byte [rbx+0x38]
141e76111  test  al, 2
141e76113  jne   141e760ed             ; already controlled -> nothing
141e76115  or    al, 2
141e76117  mov   byte [rbx+0x38], al
141e7611a  jmp   141e760ca             ; -> FUN_141e397b0(npc, 1)
```

So `0x044F` followed by `0x0451` on the same object would set bit 1 and call
`FUN_141e397b0(npc, 1)`.

**And that has already been tested without anyone realising.** The *create* arm of the same
handler ends the same way: **[L]**

```asm
141e7608a  mov  byte [rbx+0x38], 2
141e760c5  call 141e36b20              ; the shared 20-field body
141e760e3  mov  edx, 1
141e760e8  call 141e397b0              ; <- the same call
```

`!npcecho` sends `0x0451` with `flag = 1` and a **fresh** object id, so every echo took that
arm and every echo received `FUN_141e397b0(npc, 1)`. The echoes still arrived late. **A
`0x044F` + `0x0451` pair delivers nothing the echo did not already get.** [D]

### 6.2 `!npcfx off`'s polarity was correct, so that elimination stands

Worth re-deriving, because an inverted test would have been unable to come back false —
exactly the failure `CLAUDE.md` keeps recording. The `0x0452` handler: **[L]**

```asm
141e76c7a  call  1406e8c20             ; u32 v
141e76c81  test  eax, eax
141e76c83  setne dl
141e76c86  mov   [rip -> DAT_143ad2d30], edx    ; = (v != 0)
```

and the decoder gates on `cmp dword [DAT_143ad2d30], 0; jne skip`. `!npcfx off` sends `v = 1`
→ global = 1 → the attach-and-start block is skipped. Correct.

I also checked the one other path that starts the same object, `FUN_141e4ac00` at
`0x141e4b5cd`: it is guarded by `mov rcx,[r14+0x578]; test rcx,rcx; je` — with the global
set, `npc+0x578` is never written, so that call is skipped too. `tools/dataref.py
0x143ad2d30` gives exactly **three** references in the whole image (`0x141e39489` the
decoder's gate, `0x141e64595` the rebuild helper's gate, `0x141e76c86` the write), so there
is no fourth path. **The appear-effect object really was switched off.** [L]

### 6.3 The `+0x280` layer-insert flag is not on the creation path

`research/npc-fade.md` §8 named this **[I]**: *"CNpc calls `+0x280` twenty times with
`mov edx, 0x20` (x13) and `xor edx, edx` (x7); CMob calls it ninety-two times with
`xor edx, edx` (x59) and `mov edx, 0x20` (x30). Same two values, very different ratio."*

Grepping the two functions that make up NPC creation — `FUN_141e36b20` (the whole 11 379-byte
decoder) and `FUN_141e4ac00` — for `call qword ptr [reg+0x280]` gives **zero hits in each**.
The same regex finds **5** `+0x198` calls in `research/msexe-npc-clocksync.txt`, so it can
match this form. **[L]**

Every vtable slot `FUN_141e36b20` reaches through a `call qword ptr [reg+disp]`, enumerated
rather than searched for: `+0x10` (x23), `+0xc8` (x3), `+0x200` (x2), `+0x168` (x2),
`+0x140` (x2), `+0x68`, `+0x50`, `+0x38`, `+0x160`, `+0x150`, `+0x118` — eleven slots, and
`+0x280` is not among them. Both `+0x200` (`put_color`) calls are `mov edx, 0xffffffff` —
**fully opaque** — at `0x141e38317` and `0x141e386c5`. **[L]**

So `+0x280` can only matter per-frame, not at creation, and the NPC is created opaque. This
narrows npc-fade §8's lead rather than killing it — a per-frame difference is still a
per-frame difference — but it is no longer a *creation-time* candidate.

> Blind spot: this counts only `call qword ptr [reg+disp]`. `FUN_141e36b20` also uses
> `mov rsi,[rax+X]; call rsi` (npc-fade §10 records it for `+0x118`), and that form is
> invisible to the grep. So "zero `+0x280` calls" means *zero of that encoding*.

### 6.4 `tools/rtti.py` cannot reach CNpc's vtable, and neither can the obvious workaround

I wanted CNpc's virtual `+0x38` (the one the decoder calls last, at `0x141e3954e`, the
analogue of the mob's `encodeInit`). `tools/rtti.py --vtable CNpc` reports
`typeDescriptor 0x143a9d5e0   0 locator(s)`. I wrote the inverse walk — scan every aligned
dword for the type descriptor's RVA, verify a CompleteObjectLocator around it, then find the
qword pointing at it — and it returns **0 COLs** as well.

Both tools return 0 for `CMob` and for `CLoginQueueDlg` as well, so this is not specific to
CNpc. **But two scans agreeing is not corroboration when they share a blind spot** — mine is
a re-implementation of the same walk `rtti.py` does, so of course it agrees. The correct
reading is **unresolved, not absent**: I could not locate CNpc's vtable, so I could not name
its virtual `+0x38`, and nothing in this document rests on doing so. Named here so the next
pass does not spend the same hour, and so that no one writes "CNpc has no vtable" on the
strength of either tool — the decoder demonstrably calls through one at `0x141e3954e`.

---

## 7. What is left, ordered by cost

| candidate | why it is still alive | what would kill it |
|---|---|---|
| **The premise.** Every object this client creates is late by the same beat, and NPCs are simply the ones the owner is looking at | Nothing has ever compared an NPC and a mob created at the same instant (§5). Both known "mobs are instant" observations were made 7 s after entry or against a map with no NPCs | §8's test. If both are late, this thread ends |
| **The client renders during the tail of the `0x01A0` handler** | It sends `0x0238` 126 ms before the handler returns and `0x00DC` 62 ms before. If frames are being drawn in that window the map is on screen before any object packet can be dispatched, and every field-entry object is late by 60–130 ms | A frame-counter watch, or §8's test coming back "both late at entry, both instant mid-game" |
| **CNpc's first `Update()`** — the object is constructed opaque and correct, but the layer has no canvas until the displayer's first tick | §6.3 shows creation ends opaque and z-correct; the draw is somebody else's frame | A watch on the CNpc update virtual, or §8 |
| **`Gr2D_DX11.dll`** — `research/npc-fade.md` §8's residual, unchanged | Not decompiled; no RTTI for the layer class; the ATL interface map gives the sub-object offset and no vtable | Out of reach with the current tooling. Do not start here |

The first row is the one to spend a launch on, and it is the only one that can come back
false in a single observation.

---

## 8. WIRE IT LIKE THIS

**Do not send `0x0467`.** It is not a preload, the preload is not the problem (§3), the
packets are already as early as the client can take them (§4), and it has a side effect worth
naming: each entry **pushes a node onto a list on the process-global cached template**
(`template+0x170`) with no dedup that I could find, so a send-per-field-entry would grow that
list for the life of the process. If it is ever wanted for its real purpose — a dated script
override on an NPC — the body in §1.2 is correct and complete.

What to wire instead is **the control**, because the thing this thread is missing is not
another mechanism, it is a measurement.

### The command

Add `!spawnpair [dx]` to `crates/world/src/session/gm.rs`, beside `!npcecho`. In **one reply
batch** it sends exactly two creation packets:

```
1.  0x03C6  MobEnterField        appear_type = APPEAR_ALREADY_THERE (-1)
2.  0x044F  NpcEnterField
```

Build them by **copying values that are already on screen and already working**, changing as
little as possible:

* **the NPC** — copy the first entry of `self.config.npcs[map]` verbatim (this is exactly
  what `!npcecho` does and it has never faulted). Change only `object_id` (`+ 7000`, so it
  cannot collide with the map's `1000`-series or `!npcecho`'s `5000`-series) and `x`
  (`+ dx`, default `120`). Leave `cy`, `fh`, `f`, `rx0`, `rx1` and bytes 12..19 alone —
  `research/npc-fade.md` §7 records that touching bytes 12..19 makes NPCs vanish.
* **the mob** — take a **currently live** mob from `self.fields.mobs_on(map)` and copy its
  `as_seen()` body verbatim, the same value the respawn tick already sends successfully.
  Change only `object_id` (fresh, **non-zero and not a multiple of 178** —
  `research/mob-spawn.md` §2.1) and `x`/`y` to sit beside the NPC copy, so both are in one
  screenful. Set `appear_type = net::mob::APPEAR_ALREADY_THERE`, the value already proven to
  make a mob pop.

Refuse, with a message saying which half is missing, unless the map has **both** at least one
configured NPC and at least one live mob. That guarantees **both templates are already
resident and already being drawn**, which removes the art-load confound from the test rather
than assuming it away.

Do **not** send a `0x03D2 MobChangeController` for the copy: this test is about the first
draw, and control is a separate variable. If the copy stands still, that is expected and
fine.

Ordering inside the batch is immaterial and the run will show it: the hook log's sibling
`0x044F`s dispatch in the same millisecond (`20:11:08.172` twice), and `0x03C6`/`0x03D2`
pairs likewise. Send the mob first anyway, so any bias is *against* the NPC and a
"both together" result is the stronger one.

### What the owner does

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1"
```

1. Log in and `!map 40` (Snail Hunting Ground I). **Map 1 will not do**, and this is counted
   rather than remembered: `gm-handbook/mobs.txt` has **0** rows for map 1 and **40** for
   map 40, and `gm-handbook/npcs.txt` has **2** rows for each. Map 40 is the only map in the
   current config known to have both halves; on map 1 the mob's art would be a cold load and
   the test would be measuring the wrong thing. **[L]**
2. **Wait for the snails to spawn** (the respawn tick, about seven seconds) and for the map
   to settle completely. This must not be run during a map transition.
3. Stand still, look at the empty ground beside the NPC, and type `!spawnpair`.
4. Report **which of the two you see first**, and roughly by how much.

### What each outcome means

| on screen | meaning |
|---|---|
| **the snail pops solid, the NPC is absent for a beat then present** | It is CNpc-specific and it is not the packet, not the art, not the timing, not the creation route, not the appear effect. Everything the server can reach is now eliminated, and the only place left is CNpc's first frame / `Gr2D_DX11.dll` — `research/npc-fade.md` §8. Report that as a **client-side, unreachable** result and stop spending launches on it |
| **both are absent for the same beat, then both present** | **It was never an NPC bug.** Creating any object in this client costs that beat, mobs included; the reason mobs looked instant is §5 — they arrive seven seconds into a settled map with nothing to compare them to. The whole thread closes, and `STATUS.md`'s "NPCs do not appear instantly" should be rewritten as "newly created objects take N ms to first draw, mobs and NPCs alike" |
| **both pop instantly** | Mid-game creation is instant for both, so the lateness is scoped to **field entry**. Since §4 shows the packets are already 1 ms behind the field load, the remaining explanation is that the client draws the map *before* `CStage::OnSetField` returns. That is measurable with a frame watch and is not a server bug |
| **the NPC pops and the snail is late** | The premise is backwards. Re-measure everything, starting with which one the owner has actually been watching |
| **nothing appears** | The bodies, not the theory. `world.log` names both packets; the hook log says whether each was dispatched and how long its handler took. A missing hook line means the handler was entered and never returned |
| **the client faults** | Almost certainly the mob copy's `object_id` or `x`/`y`. `research/mob-spawn.md` §11 has the whole history of that body. The NPC half is the proven `!npcecho` path |

The first three rows are mutually exclusive and each of them ends a different line of
enquiry. That is what makes this worth a launch and `0x0467` not.

### One documentation change, no bytes

`crates/net/src/opcode.rs` and `research/npc-spawn.md` §3.1 both describe `0x0467` as a
template preload list. It is `SetNpcScriptable`: `u8 count`, then per entry
`u32 templateId`, `str script`, `u32 dateStart`, `u32 dateEnd`, dates as `yyyymmdd`. I have
not touched either file — they are the coordinator's and another pass's.

---

## 9. Instruments, controls, and what each one could not see

| instrument | control run | blind spot, named |
|---|---|---|
| `tools/listing.py` | its `READ` lines for `FUN_1402eeef0` reproduce `tools/reads.py`'s four reads at the same four addresses — the cross-check its docstring demands | shares `reads.py`'s loader and extent, so it cannot disagree with it about where a function ends. A guarantee, not a check |
| `tools/reads.py` | as above; it also correctly marked the two trailing `u32`s `gated?` where the listing shows them on the straight-line path | at depth 3 through `FUN_141e78110` it summarised the helper as *"u32 str"* and dropped the two trailing `u32`s. **Running it on the helper directly gives all four.** A depth-limited walk here comes back **short**, which is precisely the failure `CLAUDE.md` records twice. Do not trust a nested read summary |
| `tools/callers.py` | `FUN_140d13c80` → 2 sites, both of which I had already read by hand | `FUN_1418224c0` came back **0 calls, 62 data pointers** — it is a virtual, so the field-load chain cannot be proved by call graph. I did not claim it was; §3's argument rests on the measured handler times instead |
| `tools/dataref.py` | 3 references to `DAT_143ad2d30`, one of them the write in the handler I then read | reads/writes/tests of a global, and only the opcodes on its `OPS` list |
| `tools/pdata_lookup.py` | caught that `FUN_141e75800` is **two** `.pdata` chunks (`0x141e75800..0x141e75a9a` and `0x141e75aa0..0x141e75d08`); a single-chunk dump would have missed the `0x0453..0x0466` block | — |
| `tools/rtti.py` and my inverse COL walk | none that discriminates. **Both return 0 for `CNpc`, `CMob` and `CLoginQueueDlg` alike, and mine is a re-implementation of the same walk — a shared blind spot, not corroboration** (§6.4) | neither reached a vtable in this image. Treat the result as *unresolved*, never as "class X has no vtable"; the decoder calls through CNpc's at `0x141e3954e` |
| rip-operand string resolver (new, scratchpad) | run on `FUN_141e77b70` it returns **exactly** the two strings `research/npc-spawn.md` names for it, `Npc/%07d.img` and `String/Npc.img`, and nothing else | one level of `PTR_` indirection only; a literal copied inline (`mov rax,[rip+d]` of the *bytes*) is invisible, which is the hole `tools/xref.py`'s docstring records |
| `call qword ptr [reg+0x280]` grep | the same regex finds 5 `+0x198` calls in `research/msexe-npc-clocksync.txt` | matches only that encoding; `mov rsi,[rax+X]; call rsi` is invisible (§6.3) |
| the hook log's `elapsed_us` | it separates a 586 000 µs field load from a 196 µs NPC decode **in the same file, seconds apart**, so it discriminates across three and a half orders of magnitude | the line is written **on return**. A missing line means entered-and-never-returned, not not-dispatched. It also times the *handler*, not the frame — it cannot say when anything was drawn |
| world.log ↔ hook log alignment | the `mm:ss.mmm` fields agree to the millisecond within one session (`11:08.109` / `20:11:08.171`) | the `hh` fields do **not** agree — world.log's is not wall clock. Align on `mm:ss` and only ever **within one session** |

`ModernMapleSource` was consulted once, for the *shape* of `SET_NPC_SCRIPTABLE` in §1.3, and
is labelled a candidate there. Nothing in this document depends on it.
