# The blank char-select avatars — MEASURED 2026-09-12: the slots are placed EMPTY

> **UPDATE 2026-09-12, instrumented run (01:54).** The null-gate hypothesis below (§1) is
> **refuted by the probe** and kept only for the record. The select-UI object *is* built and
> avatar placement *does* run; the slots are placed **empty** because the per-character fill
> never runs. The measurement and the real mechanism are in §6, which supersedes §1–§5.
> The probe is re-aimed accordingly (§7).

# (SUPERSEDED) The blank char-select avatars as a fade-deadline race — 2026-09-12

The owner: *"Sometimes when the login happens too fast through transitions, the characters on
character select do not render at all... it happens more for clients that are further away."*
The stat scroll draws (STR/DEX etc. are right), the character frames draw, only the **avatar
sprites** are missing, and they appear on a return visit to select.

The 2026-09-10 theory (`research/select-screen-race-2026-09-10.md`) tied this to the client's
four background tasks (`0x007A`) and the list re-send. That was disproven: the re-send fired
correctly 403 ms after the list and the avatars still stayed blank. This file reads the
client's char-select machinery statically and finds the actual gate. **Everything here is
`[I]` — read off the listing with `tools/`, addresses resolved with the tool, not by hand —
until the probe in §4 confirms it on a launch.**

## 1. The gate: avatar placement is skipped when the list object is null

The char-select avatars are drawn by **`FUN_141179970`** (it references `L"character%d"`,
`L"selectEffect0/1"`). Every path that calls it first checks a pointer and skips placement
if it is null. In the screen builder **`FUN_141b3e060`**, CharSelect case (screen 4):

```text
141b3e0c3  mov  rcx,[rbx+0x138]        ; the screen's own UI object
141b3e0cf  mov  edx,4 ; call 141b46a70 ; builds the /pos frame + statboard
141b3e0d9  mov  rcx,[rip+0x1f8c6b0]     ; -> global 0x143aca790  (the char-LIST object)
141b3e0e0  test rcx,rcx
141b3e0e3  je   141b3e11a               ; <<< NULL -> SKIP avatar placement
141b3e0e5  mov  dl,1 ; call 141179970   ; place avatars
```

So "frames + statboard present, avatars absent" is exactly **the list object `0x143aca790`
being null at the instant the screen is drawn.** The tool confirms every reader/writer of
`0x143aca790` (gate `141b3e0d9`, decoder `141b33e9c`, login-result teardown `141b30ca0/cb1`,
populator body `141b27da0`) resolves to that one address.

## 2. Who fills `0x143aca790`, and when

The only writer of the list object is **`FUN_141b27da0`** (populates from the decoded
records), called from exactly one place: **`FUN_141b3f290`** — the "build screen contents"
wrapper, which populates (`141b3f2fb`) and then places avatars (`141b3f3ef`).

`FUN_141b3f290` is reached two ways:

* the screen switcher `FUN_141b3f050`'s **instant (no-fade) path** — `141b3f0d0`; and
* the **per-frame char-select driver `FUN_141b3efe2`**, when a pending transition's fade
  **deadline has passed**:

```text
141b3efe7  call 1429e3ef0              ; now()
141b3efec  cmp  byte [rbx+0x240],0     ; transition pending?  (switcher sets =1)
141b3eff5  je   141b3f010              ;   no -> just tick
141b3efff  call 1408fc980             ; elapsed? now >= deadline [rbx+0x248]
141b3f006  je   141b3f010              ;   not yet -> just tick
141b3f00b  call 141b3f290             ; DEADLINE PASSED -> POPULATE + PLACE
>141b3f010 call 141b3e920             ; run the crossfade animation tick
141b3f020  call 1408fc980             ; elapsed (second gate)?
141b3f03c  jmp  141b3e060             ;   -> (re)build/draw the screen  [the §1 gate]
```

The login **record decoder `FUN_141b32860` always requests a faded transition** —
`mov edx,4; mov r8d,0x258; call 141b3f050` (0x258 = 600), at both `141b33127` and
`141b335dc`. The switcher arms the transition (`[obj+0x240]=1`, start `[obj+0x244]=now`,
deadline `[obj+0x248] = now + fade*1.5`, K=1.5 read from `0x14327aa58`) and sets up the
crossfade — **it does not populate on the fade path.** Neither the crossfade setup
`FUN_141b3e1d0` nor the crossfade tick `FUN_141b3e920` populates or places (both are pure
graphics — verified: no call to `141b27da0`/`141b3f290`/`141179970`/`141b3e060` in either).

So on the login path the avatars are placed **only when the ~900 ms fade deadline passes**,
via the driver's `141b3f00b` branch.

## 3. RETRACTED the same day: the driver has no draw-before-populate window

The first draft of this section said the driver could draw the screen before populating it,
because draw and populate sit behind two different elapsed checks. Reading the operands
(which the first pass had filtered out) kills that:

```text
141b3eff7  mov ecx,[rbx+0x244] ; populate when now > [+0x244]   (= arm time, or arm+fade/2)
141b3f018  mov ecx,[rbx+0x248] ; draw     when now > [+0x248]   (= arm + 1.5*fade)
141b3f2a7  mov byte [rcx+0x240],0        ; FUN_141b3f290 clears the pending flag first thing
```

`[+0x244] <= [+0x248]` always, and both run in the same per-frame function in that order,
so **populate always precedes draw**. Whatever leaves `0x143aca790` null at draw time, it is
not this ordering. The §1 gate and the §2 populate site stand; §3's race does not.

## 4. What the decoder does that the first visit may miss, and the object's real origin

Two more facts from the same read, both `[I]`:

* **The populator is a lazy singleton builder for all the login sub-UIs**, not a
  per-list refresh. `FUN_141b27da0` runs six blocks of the shape *"if
  `FUN_141128090(stage, k)` says slot k is enabled: create the object if null (ctor), then
  `[vtable+0x90](obj,0)`; else destroy it"*. Slot **5** is the character-select UI
  (`FUN_141177490` at `141b28054`, 0x5f0 bytes). So the object is created by the **first
  transition's** deadline after the stage is armed — which the **world list** already does
  (`FUN_141b2fac0` calls the switcher at `141b2fb46`) — not by the character list.
* **The character-list decoder refreshes the object rather than building it.** At the end
  of its main path: `141b33e9c mov rcx,[0x143aca790]; call FUN_141177e40`, and
  `FUN_141177e40(obj)` is `for slot in 0..3: FUN_141177e80(obj, slot); [vtable+0x90](obj,0)`
  — the per-character fill. It has **no null check**. A second path through the decoder
  (`141b330c5`: a remembered character id at `session+0x35b4` resolves) goes to the second
  transition at `141b335dc` and **exits without that refresh**.

So the shape of the bug that fits every observation is: **the character list is decoded
before the client has constructed its select UI** (the ctor fires on a frame after the world
list's transition arms; the list follows the world list by 0 ms pre-pause, 400 ms since),
so the refresh finds nothing to fill and the object built moments later is empty. The
2026-09-03..07 window drew because 400 ms was enough then; the 09-08 guard-page build slowed
the client's start-up frames (its task durations went 64 ms → 787 ms) and 400 ms stopped
being enough; a distant or slow client loses the same way. **This is a hypothesis.** It is
one launch from being a measurement, and the measurement is cheap.

## 5. The probe — wired as the launcher default, plan step TL

```
watch@1415db360:ret,141b2a280:rdx=0,142ef3e44:hits=8,141177490:hits=4,141177e40:hits=8,141179970:hits=12
```

| watch | what its line says |
|---|---|
| `141177490` | the select UI object's **constructor** - the moment the client built it. Compare with login.log's `-> 0x0010` time; both are the same machine's clock |
| `141177e40` | the list decoder's **refresh** of it. **Its `rcx` IS the object**: `rcx=0x0` means the list arrived before it existed |
| `141179970` | **avatar placement entered** = the null-gate passed |

Watch lines are written on entry, so a missing line means never entered. Readings, written
down before the launch:

* **avatars draw**, ctor before the `0x0010`, refresh `rcx != 0`, placement present → the
  good case works as read; model confirmed.
* **avatars blank**, refresh `rcx=0x0` (or no refresh line) and **no** placement → the list
  was decoded before the select UI existed. **That is the bug**, and the ctor's timestamp
  says how long the server must hold (or repeat) the list.
* **avatars blank but placement present** → placed and drew nothing: the avatar art is not
  resident at select; a different fix.
* **the client dies at the list** → `FUN_141177e40` does not tolerate a null object; a
  finding in itself (`tools/decode_elog.py`).

The owner cannot force the blank case; the probe stays the default across the next few logins,
and a good login still pins the normal ordering.

**The 01:49 launch on 2026-09-12 reproduced the blank screen and measured nothing** (the old
watches were armed; the launcher's compiled `DEFAULT_PROBE` is the copy that reaches the
client, and only the script's had been changed). The 01:54 launch, correctly instrumented,
is §6.

---

## 6. MEASURED (2026-09-12, 01:54) — the object is built, placement runs, the slots are empty

All six watches armed (`probe: watching 0x141177490` … `0x141179970`). On a blank-avatar
login the hook log shows:

```text
01:54:09.546  141177490 ENTERED  called-from=141b28059   <- the select-UI object IS constructed
01:54:09.638  141179970 ENTERED  called-from=141b3f3f4  rdx=0   <- placement runs (during 0x0032)
01:54:10.045  141179970 ENTERED  called-from=141b3f3f4  rdx=0   <- again (during 0x0010 #1)
01:54:10.300  141179970 ENTERED  called-from=141b3f3f4  rdx=0   <- again (during 0x0010 #2, resend)
01:54:12.689  141179970 ENTERED  called-from=141b3e0ec  rdx=1   <- screen-build repaint
              141177e40           NEVER ENTERED
```

login.log (same clock +4 h): world list 09.516, `0x0010` #1 at 09.917, `0x0010` #2 at 10.246.
The hook patched the client's mode 5→2 at **10.029 — between the two `0x0010`s**.

So §1's gate is innocent: **the object is non-null (its ctor ran at 09.546) and placement is
entered four times.** The avatars are blank because the **per-character fill never runs** —
`141177e40` has no line. `141179970` places the slot frames and select effects
(`character%d`, `selectEffect0/1`); the per-slot *character* object at `slot+0x10` is filled
by `FUN_141177e80`, and its loop **skips any slot whose `+0x10` is null** (`141179b2a
cmp qword [rdi+0x10],0; je next`). Nothing wrote those pointers, so every slot is skipped.

`FUN_141177e80` is reached only from `FUN_141177e40` (the login handler's slot refresh) or
`FUN_141177790` (the select UI's vtable+0x20 build method). **Neither ran** — the placement
lines come from `141b3f3f4`/`141b3e0ec` (the fade driver and screen build), never from
`1411778cb` (inside `141177790`).

### Why the fill is skipped — the leading hypothesis, from the call graph

The `0x0010` handler is `FUN_141b307b0`, which forks on the session mode at its head
(`cmp eax,5; jne <mode-2 body>`):

* **mode 5** → `FUN_141b32860`, which decodes the list (`14108bdf0`) **and** refreshes the
  select slots (`141177e40` at `141b33e9c`) — avatars fill.
* **mode 2** → the in-line body, which decodes the list (`14108bdf0` at `141b3101c`) and
  transitions, but **never calls `141177e40`, `141177e80`, `141177790`, `141b27da0` or
  `141b3f290`.** It fills nothing. (Confirmed: those are the only fill-chain calls in either
  function; grep of `FUN_141b307b0`'s listing finds only the `14108bdf0` call.)

The hook patches mode 5→2 after the world list, so **whether `0x0010` is handled in mode 5
(fills) or mode 2 (does not) depends on whether the client dispatches the list before or
after the patch** — which varies with timing and latency. This launch patched the mode
between the two `0x0010`s, and the fill never ran. **This is `[I]`** — it explains every
observation (intermittent, worse on slow/distant clients, drew in some eras) but the good
case has not been captured, and it does not yet explain how mode-2 *ever* draws. §7 settles
both with one launch.

The fade-deadline machinery in §2–§4 is real and correctly read, but it builds and places
the select UI; it does **not** fill per-character data, so it was never the whole story.

## 7. The re-aimed probe — catches the fill on a good run, its absence on a blank one

```
watch@1415db360:ret,141b2a280:rdx=0,141b32860:hits=4,141177e40:hits=6,141177790:hits=6,141177e80:hits=16
```

| watch | what its line says |
|---|---|
| `141b32860` | the **mode-5** `0x0010` handler ran (the one that fills). Absent = the list was handled in mode 2 |
| `141177e40` | the mode-5 **slot refresh** ran |
| `141177790` | the select UI's **vtable build** ran (the other route to the fill) |
| `141177e80` | the **per-slot fill** itself — present on any login whose avatars draw |

Readings, before the launch:

* **avatars draw**, `141177e80` present → the fill ran; `141b32860`+`141177e40` present means
  it filled through the mode-5 handler (so blank ⟺ mode-2, and the fix is to keep the list in
  mode 5 or make mode-2 fill); `141177790` present instead means a vtable route fills and the
  mode is not the discriminator.
* **avatars blank**, `141177e80` absent → confirms §6: the slots are placed empty. Compare
  `141b32860` present/absent between a good and a blank login to confirm the mode race.

`141177490` (ctor) and `141179970` (placement) are dropped — §6 established they run on every
login and do not discriminate.

## 8. MEASURED (2026-09-12, 02:15–02:16): three logins, two good, one blank — the mechanism

The owner: *"I relaunched 3 times, the first 2 launches drew the avatar at character select just
fine, but the third launch drew blank avatars."* All three ran the §7 probe (hook logs
`maplecw-hook-20260912-021549`, `-021601`, and the live one; one `login.log`, +4 h).

```text
                    good #1        good #2        BLANK #3
login request 0x0080   35.338         54.986         08.040
mode patched 5->2      35.341 (+3ms)  54.989 (+3ms)  08.595 (+555ms)   <- world list dispatched late
-> 0x0010 (list)       35.740         55.388         08.441
141177790 build        35.822 (+484)  55.486 (+500)  08.070 (+30ms, "while dispatching 0x0032")
141177e80 fill x3      35.828-.903    55.495-.602    08.076-.077 (all three within 1 ms)
141b32860 / 141177e40  never          never          never
```

* **§6's mode-race theory is dead**: mode 2 handled `0x0010` on all three, and the two good
  logins filled anyway - through the UI's build method `FUN_141177790` (called from
  `142bf3cce`, the generic create path), not through `141177e40`.
* **The select UI is built once, at the fade-deadline populate, and `FUN_141177790` fills the
  slots from whatever character list exists at that instant.** On the good logins that was
  ~490 ms after the login request, after the list. On the blank login the build ran **30 ms**
  after the login request, from *inside the still-running `0x0032` dispatch* - the client
  dispatched nothing for the next 555 ms (the world list, sent at 08.040, was dispatched at
  08.595) - so it filled from an **empty** list. The `0x0010` that followed was decoded by
  the mode-2 body, which calls nothing in the fill chain, and nothing refilled the slots.
  Placement then skipped all three (`slot+0x10` null). That is the blank screen.
* The three fill lines on the blank run carry different arguments (`r8=0x20 r9=0x140331540`
  vs `r8=0x14dda8 r9=0xfffb` on the good runs) and land within 1 ms of each other - an empty
  fill against a good one, visible in the log.

**No server timing can fix this ordering**: whatever the server sends is dispatched after the
build. The client's own answer is the call the mode-5 handler makes after every decode and
the mode-2 handler does not: `FUN_141177e40(selectUi)`, which re-fills the three slots from
the decoded list. The hook now makes that call after every `0x0010` dispatch **when the
object already exists** (`grap_stub::session::refresh_select_after_dispatch`; `selectfill=off`
in the session marker turns it off). In the good ordering the object does not exist yet at
that moment, the hook logs so and does nothing, and the build fills exactly as before. **This
patches the client** and does not make the session valid.

What the next launches must show (plan step TL): the hook log line `SELECTFILL: called
FUN_141177e40` on a login whose select UI was built early, followed by `141177e40` and three
`141177e80` watch lines *after* the `0x0010` - and avatars. On a login whose build came after
the list: `SELECTFILL: the select UI is not built yet`, and the build fills as before.

## 9. 09:16, three more launches with the fix armed: 2 blank, 1 good — the fix never ran

Both blank logins were the early-build ordering exactly as §8 predicts (build at +30 ms inside
the `0x0032` dispatch, three empty fills with `r8=0x20 r9=0x140331540`), the mode patch came
~500 ms later, and `SELECTFILL` fired after each `0x0010` — and **refused**:

```text
SELECTFILL: refusing to call 0x141177e40 - expected [48, 89, 5c, ...], found [cc, 89, 5c, ...]
```

`0xCC` is the probe's own int3: the default probe watched `141177e40` to measure the fill,
and the guard read the watch. **The instrument defeated the fix.** The good login (`not built
yet`, build at +492 ms after the list) drew as before. Fixture:
`research/fixtures/selectfill-refused-by-own-probe-int3-early-build-blank-hook.log`.

So the run is a positive result for the model (blank ⟺ early build, on all five blank logins
now measured) and a null result for the fix. Two changes: the guard accepts `0xCC` at byte 0
(the probe's handler restores it for any caller) and logs that it called through an int3;
and `141177e40` is off both `DEFAULT_PROBE`s - the `SELECTFILL:` line already says when the
call happens. Still unverified on screen; plan step TL is the same measurement.
