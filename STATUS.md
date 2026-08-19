# Where things stand — 2026-08-19: **a character stands on map 1, playable**

Pick-up notes for the next session. See `ROADMAP.md` for the plan and `docs/` for the
specs.

## START HERE

**There is a real server now.** `crates/login` replaced the Python harness on 2026-08-18,
and **characters persist between launches** - the priority the owner set the day before. The
client also no longer kills itself, so a session runs as long as you want it to.

One command, from an **elevated** shell:

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1"
```

It builds, installs the hook into `client-patched/`, starts **both servers** -
`maplecw-login` on 8484 and `maplecw-world` on 8485 - applies the client patches and
launches the client. Close the client by hand when done, then `-Stop`. `-ListOnly` prints
the stored characters and launches nothing. `-SetFieldProbe` makes the channel answer the
migration hello; see NEXT GOALS.

**The character-select screen is finished and server-driven**, all confirmed on screen:
the list, create, a truthful name check, the three-slot limit, **delete**, and persistence
across relaunches. Two things that cost real effort and must not be relearned:

* **Never renumber characters from 1.** A create reply carrying id 1 was byte-identical to a
  known-good one except the two copies of the id, and the client silently refused to
  transition. Ids start at 200.
* **Do not pass `-SessionTokens`.** The measurement it existed for is done (the answer is no),
  and passing them causes a "trouble connecting" dialog from a path we do not suppress.

The old harness still exists and still works - `test-charselect.ps1 -SkipNetCheck` - and is
the right tool for capturing packets or trying a hand-written body. It answers from canned
bodies and persists nothing.

**Reverse engineering the client: read `docs/ghidra.md` first.** The working command line,
the JDK 21 requirement, why the project locks (it matters the moment you spawn a subagent),
which of the seven packet-read primitives exist, and the two instrument mistakes that have
each produced a clean, confident, wrong answer here.

Where the answers land:

| file | what is in it |
|---|---|
| `login.log` | every packet both ways on the **login** connection, and what each reply was |
| `world.log` | the same for the **channel**, from the migration hello onward - read this one for anything past character select |
| `client-patched\maplecw-hook.log` | `WATCH` lines, session patches, client faults. Not `hook.log`, not the repo root |
| `client-exit.log` | how the client died: exit code, lifetime, job membership, handle holders. A clean `0` is a hand-close; `0xC0000409` is the fail-fast returning |
| `probe.log` | only when running the old Python harness |

The client patches are still patches. The reachability check that `__fastfail`s the client
after ~37 seconds is now neutralised by the **probe patch** `watch@1415db360:ret`, which is
what `test-server.ps1` arms - which is why the launch line carries no `-SkipNetCheck`. The
argument still works and the old harness still uses it. See "SOLVED - the ~37 second exit"
below, and `docs/launcher.md` for which patches retire.

**A frozen UI is almost always an unanswered packet, not a crash.** The client blocks its
whole interface - every button, including the quit prompt's OK - waiting on a reply. That is
what "Check" did before `0x0081` was answered and what "Choose another world" did before
`0x0082` was. **Read `login.log`** - it names every reply and what it answered, so the last
inbound line with nothing after it is the packet nobody answered. `crates/login` has a test
for this rule, and no path in it returns an error in place of a reply.

## WHERE THINGS STAND - read this first when picking up

## THE GOAL IS MET — 2026-08-19, seen on screen

A character stands on **map 1, Mushroom Town - West Entrance**, playable. The minimap and
street name are right, HP/MP/EXP are live, the tutorial NPC dialog fired, and there was **no
client fault**. Run preserved as `research/fixtures/character-on-map1-playable-*.log`.

The probe discriminated exactly as it was designed to:

```text
0x0010  140302e30 x3   from 0x1403094d0   the character-LIST path - the armed positive control
0x01A0  140304b20      from 0x142098432   the record decoder ran
0x01A0  140302e30      from 0x140304e76   the gate OPENED and the stat block decoded
        (no CLIENT FAULT line)
```

Everything the pre-flight analysis predicted held, and nothing it deferred bit: the three
all-zero randomiser seeds, the zero pair at head offsets 22/26, and the zero at offset 17 were
all sent as-is and none of them mattered.

## NEXT GOALS - set by the owner, 2026-08-19

Login and world entry are done. These five are the work now, in their order. Each carries what
is already established, so nobody re-derives it, and the **one concrete next step**.

### START HERE - what to do next, in order

Nothing below needs a client launch to *build*; every one is specified well enough to write
code against. Launches are for confirming, and two of these can share one.

| # | do this | why it is first | spec |
|---|---|---|---|
| ~~1~~ | ~~**Build the equipped-item block**~~ - **BUILT 2026-08-19, unconfirmed on screen** | See "6. Equipment" below for what to watch on the run. | `research/naked-character.md`, goal 6 |
| ~~2~~ | ~~**Answer `0x0151` with a `0x055B` Say**~~ - **BUILT 2026-08-19, unconfirmed on screen** | See goal 2 below for what to watch. | `research/npc-dialogue.md`, goal 2 |
| 3 | **Finish the mob body** - read `FUN_14046fba0`'s movement-path framing | The owner made mobs a priority. Everything else about the spawn is read; only the path's element count and dispatch value are missing. | `research/mob-spawn.md`, goal 3 |
| 4 | **Read what greys a channel row** in `ChannelChange` | Cheapest of the four, but the least valuable - it unblocks one dialog. | goal 2a |

**Then one launch** confirms 1, 2 and 3 together. They are independent - equipment is in the
record, dialogue is a reply to a click, mobs are a separate pool - so a failure in one does
not confound the others.

**Two rules that have each cost a run:**

* **Never send a script (`0x055B`) with or just before a `SetField`.** Field entry runs
  `FUN_142caa4e0`, which resets the script manager.
* **The character record has no length prefix and no resync point.** One wrong width
  desynchronises everything after it, silently.

### 1. NPCs show up on maps - **DONE 2026-08-19**

Visible and clickable. `NpcEnterField` is `0x044F`; the routing was right from the start and
the body had two zeros that mattered - **read 12 is `isEnabled`** and **read 19 is `alpha`**,
so every NPC was created, pooled, disabled and fully transparent. No fault, nothing in any
log. See the doc comment on `net::opcode::npc_enter_field`.

NPCs come from `gm-handbook/npcs.txt`, generated by `tools/dump_portals.py` out of each
field's WZ `life` node: 308 across 150 maps.

*Original notes kept below, because the routing work is still the reference for goal 2.*

### 1a. How the NPC routing was established

**Built, not working yet.** `NpcEnterField` is **`0x044F`**, a fixed **64-byte** body, routed
through **`FUN_141820080`** (`0x1a4..0x5ab`) to the NPC pool dispatcher `FUN_141e75800`
(`0x44F..0x468`). `FUN_141820080` is a dispatcher this project had not found; it fills the
gap `research/msexe-gamestage-dispatch.md` left open. Full layout: `research/npc-spawn.md`.

Settled, not assumed: **the client cannot spawn NPCs itself.** Its field loader walks the WZ
`life` node only to preload `Npc/%07d.img` art; the only code that builds a populated NPC
takes a `CInPacket *`. And the pool is **destroyed and rebuilt empty on every field entry**,
so NPCs must be re-sent after *every* `SetField`, not once.

The trigger was wrong and is fixed: `0x0238` fires **only on the very first field entry** -
measured, three portal walks produced none - while **`0x00DC`** arrives once per `SetField`,
every time, ~420 ms after. It is now `0x00DC`.

> **Next step:** one launch. The probe watches `141e75800`; if it fires while dispatching
> `0x044F` the routing and timing are right and the **body** is wrong, and if it stays silent
> the packet is never dispatched at all.

Also needed regardless: the NPC table is a **stub covering map 1 only**. The real data is
every field's WZ `life` node, and it belongs in a generator beside `tools/dump_portals.py`.

### 2. NPCs have dialogue when clicked - **BUILT 2026-08-19. Unconfirmed on screen.**

`crates/world` answers `0x0151` with a `0x055B` type-0 Say, spoken by **the NPC template the
client itself named** - by construction a real `Npc.wz` id, and a bad one costs the portrait
rather than faulting (the loader result is null-checked at `142a7b52a` and falls back to
`[ui+0x6f0]`). The builder is `net::script::npc_say`; the request parser is
`net::script::parse_quest_request`, and no parser for `0x0151` existed anywhere before.

**Verified without a launch:** `python tools/channel_smoke.py --set-field-probe` now sends
the real captured 17-byte click and checks the reply is one `0x055B`, speaks as template 1,
has `hasOverride = 0`, has message type 0, and that the body length is exactly
`20 + textLen + 6`.

> **What to watch on the run:** click Heena on map 1. **A dialog box with our text** means
> the whole chain works. **Nothing at all, and no fault** means the message was built and
> torn down, or the type/flags shifted the body - check `world.log` for the `0x055B` going
> out first. **A freeze** would be new: `0x0151` has never blocked before.

**It is text on screen and nothing more.** No quest-result packet has been found, so no
state advances - accepting the same quest twice shows the same message, and the message
says so.

**Two fields change the body length with nothing to resync on**, `hasOverride` and
`flags & 0x04`, so the flag bit is derived from the value rather than set by hand.

**One correction to `research/npc-dialogue.md`**, from re-deriving off the listing rather
than trusting the file: the second style bit is **`0x80`**, not `0x40`. `141f6fbcb` is
`movzx ebx, sil / shr ebx, 6 / and ebx, 2`, and `(0x40 >> 6) & 2 == 0`. The formula two
sections later was already right; the prose gloss was not.

*Original notes below - the identification is still the reference.*

### 2-orig. How the request was identified

The owner clicked Nina, Roger and Heena on 2026-08-19 and the client named its own request, the
way it named the portal. **`0x0151` is the NPC interaction packet**, captured in
`research/fixtures/npcs-visible-quests-clicked-world.log`:

```text
0x0151  01 e8030000 01000000 0c046d01 00000000     13-17 bytes
        ^  ^         ^         ^
        |  objectId  templateId  x,y as two u16
        a leading u8 that varies (seen 01 and 04)
```

The object id matches one we assigned (`1000` = map 1's Heena) and the template id matches
the WZ, so the first three fields are **read off our own data**, not guessed.

**The reply is inbound `0x055B`, the script message.** Routed by `CField::OnPacket`
(`FUN_141820080`) - the same range chain that already delivers our working `0x044F` - to
`FUN_141f6f350`. Head: `u32 handle, u8, u32 speakerNpcTemplateId, u8 hasOverride,
[u32 override], u8 messageType, u16 flags, u8`. Type **0 = Say**: `u32 echo,
[u32 speakerOverride if flags & 4], str text, u8 prev, u8 next, u32`. The speaker field is
**read, not inferred** - it lands in `[ui+0x2cc]` and is fed to `FUN_141e77b70`, the NPC
template loader `npc-spawn.md` already identified. Full working: `research/npc-dialogue.md`.

**Ordering constraint that would have cost a run:** `FUN_142caa4e0` - the routine that emits
`0x0238`/`0x024D` on field entry - resets the script manager. **Never send a script with or
just before a `SetField`.**

> **Next step:** build the 32-byte minimum-viable Say and send it in answer to `0x0151`. The
> values that matter are in the research file: the speaker template must be a real `Npc.wz`
> id (`0` is not one), and `hasOverride` and `flags & 4` each change the body length with no
> resync point. **Text on screen is not a quest**, though - no quest-result packet has been
> found, so state will not advance.

### 2a. Channel swapping - two channels now run

The owner, 2026-08-19: *"In the classic world startup, the user is defaulted to channel 1 of the
server. We're not trying to change that behavior, we're trying to allow the client to swap
channels from 1 to 2 and vice versa."*

**UPDATE 2026-08-19: the dialog opens but lists no channels**, and opening it sends
**nothing** - so the list is built entirely from login data, client-side. The server did
advertise two (`world Scania id 0 with 2 channel(s)`, both addresses logged) and the world
list went out, so the count reached the wire.

The likely cause, now fixed but **untested**: every channel entry's four trailing `u8`s were
zero, which told the client each channel was **channel 0 of world 0**. They now carry
`[world_id, index, 0, 0]`. That the client reads exactly four bytes there is **[L]**, read
from its own decoder `FUN_141b2fac0` (the login stage's `case 0xb`); that they mean
world/channel/adult is **[I]** from the packet family's usual shape.

**UPDATE 2026-08-19, second run: the list now shows CH.1 and CH.2** - the
`[world_id, index, 0, 0]` fix worked. **But CH.2 cannot be selected**, and clicking it sends
**nothing at all** (`research/fixtures/channel-list-shows-two-but-unselectable-world.log`,
whose whole inbound set contains no new opcode). So the client is refusing the selection
**client-side, before it would send anything** - this is not an unanswered-packet freeze.

### Most likely next steps, in order

1. **Read what makes a channel button enabled.** The dialog is `ChannelChange.img` and its
   channel numbers are the `ch/0..18` bitmaps, so the client is drawing from its own assets
   and only the *count* came from us. Find the UI code behind `ChannelChange` and read the
   predicate that greys a row. That is the direct answer and it is static.
2. **Suspect the fields we still send as placeholders.** Each channel entry is
   `str name, u32 userCount, u8 world, u8 index, u8, u8`. The client reads exactly four `u8`s
   there **[L]**, but only the first two carry meaning we chose; the last two are `0` and the
   `u32` user count is `0`. A "channel is up" or "adult channel" flag among them would
   produce exactly this. Cheapest experiment: send a **non-zero user count** and vary the two
   trailing bytes.
3. **Check whether the current channel is excluded.** CH.1 is the one the character is on. If
   the client only enables a *different* channel it should already work, so if CH.2 is grey
   the predicate is something else - but confirm which row is actually grey before assuming.
4. **The reply, once a click does send something.** Expect a migrate command: the swap has to
   mint a migration for the **target** channel, which `store::create_migration` already takes
   as a parameter, and hand back that channel's address the way `0x0011` does at login.

**Not** the world-list layout in general: the count reaches the client, the entries parse,
and both rows draw. Only selection fails.

**The startup channel is unchanged.** `world.channel_id` is still `0`, so a login lands
where it always did. What changed is that `tools/test-server.ps1` now runs **two** channel
processes by default (`-Channels`, one process per channel, `$ChannelPort + N`) and the login
server advertises both - `crates/login/src/config.rs` is explicit that you cannot advertise
more channels than you run, because the client connects to the address for the channel it
picked.

> **Watch the indexing.** The client's UI is **1-indexed**; everything in this repo is
> **0-indexed**. The client's "channel 1" is our channel `0` on 8485, and its "channel 2" is
> our channel `1` on 8486.

**What is not built is the swap itself.** The in-game Change Channel request arrives on the
**channel** connection and its opcode is unknown - the login flow cannot carry it, because in
mode 5 the client never sends a world/channel selection at all, going straight from `0x0080`
to `0x0078`. Answering it will also mean minting a migration for the *target* channel, which
`create_migration` already takes as a parameter.

> **Next step:** launch with two channels, open Change Channel, and see (a) whether two are
> listed and (b) what the client sends when one is picked. Same method that named `0x00D1`
> and `0x0151` - the client identifies its own request far more cheaply than a static search.

### 2b. Newly identified packets - the client keeps naming its own requests

Every one of these came from the owner using a feature and the capture showing what went out.
That method has now identified four requests and cost no static analysis at all.

| opcode | what | body, as far as it is read |
|---|---|---|
| `0x00D1` | transfer field (portal) | fully decoded, `research/transfer-field-request.md` |
| `0x0151` | **quest request** | `u8 action, u32 questId, u32 npcTemplateId, [i16 x, i16 y], [u32 selection]`, builder `FUN_141f0e4c0`. **RETRACTED:** this was recorded as "NPC click" with the first `u32` as an objectId "read off our own data". It is a **quest id**. Our own logs disprove the old reading - every map's first NPC is given `object_id = 1000`, yet the client answered 1000/1002/1003/1005 for four NPCs, the same values in both sessions despite opposite visit orders. The listing agrees: that field keys six accessors on a quest table and is range-tested against 40000-40999 and 30051-30079. The **second** `u32` really is the template we sent, in all four captures |
| `0x00E7` | **chat** | `u32`, then a `u16`-length string, then a `u8` - `...05 00 "Hello" 03` |
| `0x0182` | **party create** | 68 bytes carrying the length-prefixed string `"TestCharD's Party"` |

None is answered yet. **None has caused a freeze**, so none is a blocking request.

### 2c. GM commands - `!map <id>`

Typed into any chat tab. Moves the character and persists it, so a relog stays put.

**The prefix is `!`, not `/`, and that is measured.** The owner typed `/map 1` and the session's
entire capture contains **no `0x00E7` at all**, while a plain "Hello" in the same tab had
produced one. The client parses slash commands itself - `/find`, `/whisper`, `/party`,
`/friend`, `/trade`, `/level` and `/h` are baked into the executable as strings - and an
unknown one is swallowed before it reaches the wire. A server-side command therefore has to
look like ordinary chat.

**Map ids are validated** against `gm-handbook/fields.txt`, the 426 maps with a real field
image in `Map.wz` - not against the name table, which disagrees with it in both directions
(12 named-but-absent, 6 present-but-unnamed). A bad id is refused silently, because chat is
fire-and-forget and there is no outbound notice packet yet; the server log is the only
feedback.

**No permission check, and there should not be one yet** - nothing on this server
authenticates and every connection is already the same account.

### 3. Mob spawns and mob drops - **priority, set by the owner 2026-08-19**

*"there should be tutorial monsters spawning on East Entrance to Mushroom Town (ID 30), can
we also put mob spawns and in turn mob drops as a priority please?"*

The foundation is in:

* **The data is generated.** `tools/dump_portals.py` now emits `gm-handbook/mobs.txt` from
  each field's WZ `life` node where `type == "m"` - **9928 spawns across 289 maps**. Map 30
  has its six snails (template 1), which is exactly what the owner expects to see.
* **The opcode is identified.** The mob pool is `0x3C6..0x44E` on the singleton at
  `[0x143ABFE00]`, dispatcher `FUN_141D30E80`, and **`case 0x3c6` calls `FUN_141d33630`** -
  mob enter field, the exact analogue of the NPC pool's `0x44F`.
* Mobs are **server-sent** for the same reason NPCs are: the client's field loader walks
  `life` only to preload `Mob/%07d.img`. `research/npc-spawn.md` established this for both.

> **Next step:** decompile `FUN_141d33630` and read its body off the **listing**, the way the
> NPC one was. Do not trust a reference layout for the field *values* - the NPC body was
> structurally perfect and still produced nothing, because `enabled` and `alpha` were zero.

Drops follow spawns: a mob has to exist before it can drop.

### 4. NPC quests

**Not started, and it depends on goal 2.** One thing already known: the quest record is a
presence-gated block in the character record, so it will need the same gate work
`presence[0]` needed - see `research/charrecord-presence-map.md` for the 40-row table.

### 5. Map portal transitions completely working - **DONE 2026-08-19**

Confirmed on screen by the owner: *"When I walk through the portal, the portal does spawn me in
the right connecting portal. That's fixed."* Both directions work and arrival lands on the
connecting door rather than the map spawn.

*Original notes below.* **Mostly working.** Map 1 -> 10 confirmed on screen. The stub that stranded the character on
map 10 is gone: `tools/dump_portals.py` generates **1135 portals across all 426 field
images** from the client's own `Map.wz`, and the server loads it at startup.

Two things left, both known:

* **The arrival portal is ignored.** The WZ gives `tn`, the target portal's name, and we send
  portal `0` - the spawn - so the character always arrives at the map's spawn point rather
  than at the matching door. `tn` is already in the generated table's fourth column.
* **The short `characterData = 0` form is now known to be usable.** `[world+0x2358]`, its
  precondition, measured `0x00` on the first `SetField` and **non-zero on every later one**.
  We still send the long form, which works; switching is an optimisation, not a fix.

### 6. Equipment and consumables - **BUILT 2026-08-19. Unconfirmed on screen.**

The owner keeps reporting the character as naked and the Equipment window as empty, and this is
the goal that fixes it. Full working: `research/naked-character.md`.

**What the server now sends.** `presence[2]` is set alongside `presence[0]`, and the record
carries a real equipped list: `net::opcode::equipped_block` and `net::opcode::equipped_item`.
A character wearing the four starter items sends a **743-byte record** instead of 224.

**Verified without spending a launch**, which is the whole reason route 1 was chosen over
the standalone item packets: `python tools/channel_smoke.py --set-field-probe` decodes the
real server's real bytes over the independent Python transport and now parses the equipped
block the way the client does - `u8 flagA`, then `(u16 slot, 125-byte item)` until a zero
slot, then **five** `u16` terminators. It checks all four items come back, that each is
exactly 125 bytes, that `dateExpire` is not zero, and that the record is 743 bytes.

The parse is deliberately run at **both** possible stat-block lengths (108 for the
extended-SP branch, 109 for the plain one) and required to succeed at exactly one. That is
a discriminator rather than an assumption: a width error shows up as "neither parses"
instead of as a client fault.

> **What to watch on the run, and what each outcome means:**
>
> * **Character dressed, Equipment window populated** - done.
> * **No fault, still naked, Equipment window still empty** - the *layout* is right and a
>   *value* is wrong. First suspect is `dateExpire`; see `ITEM_NEVER_EXPIRES`. This is the
>   NPC lesson exactly: that body was structurally perfect and produced nothing because
>   `isEnabled` and `alpha` were zero.
> * **No fault, still naked, but the Equipment window lists items** - the items decoded and
>   the *avatar* is not being rebuilt. A different and much smaller problem.
> * **Client faults, or freezes at "Connecting..."** - the record desynchronised. The
>   client's readers throw on underrun and the throw is reported in `ELog` (`0x008F`/
>   `0x0090`) with section-relative RVAs; `tools/pdata_lookup.py` turns those into
>   functions, which names the field that was mis-sized.

**`0x0138` is no longer sent.** The server used to push a `UserAvatarModified` on every
field entry as a guess at this problem. It is dead code at byte level (below), so it was
noise in the log and nothing else; the smoke test now asserts it is absent.

**A correction that would have shipped an expired item.** `research/naked-character.md`
gave the "permanent" `dateExpire` sentinel as `0x00_00_C9_2A_69_C0_00_00`, which is
221184000000000 and decodes to **1601-09-14**. The decimal beside it, 150842304000000000,
is right and is 2079-01-01; the hex was not. `crates/net` sends the decimal and a test
asserts it is non-zero.

**What was wrong with the old reading, and both halves were wrong.** This section used to say
the decode was a vtable call at `+0x330` on classes with no RTTI.

* **`+0x330` is not the decode.** For item type 1 it is `FUN_1402fbb30` = `return this+0x242`,
  an accessor. **The decode is at `+0x358` = `FUN_140304100`**, listing in
  `research/msexe-itemslot-equip-decode.txt`.
* **RTTI was never needed.** The pooled factory `FUN_1403095e0` reads a `u8 type` and calls a
  per-type allocator whose fallback runs the constructor, and the constructor stores its
  vtable with `LEA RAX,[0x14327E1D8]`. Positive control: `vtable+0x88` is literally
  `return 1` for type 1 and `return 2` for type 2 - the same 1/2/3 the release function
  switches on.

**A minimal equipped item is 125 bytes**, because three fields are `u32` bitmasks whose bits
each gate one optional read (17 `u16` in `FUN_140303800`, 21 mixed-width in `FUN_140303b40`).
All-zero masks read nothing past the mask.

> **The trap that would wreck the record, and the one the build honours.** `presence[2]`
> gates the equipped list **and both helper lists called right after it** - `FUN_14030b6f0`
> and `FUN_14030b9e0` re-gate through the same byte, and the second reads **three** lists.
> Setting `presence[2]` therefore costs **four extra `u16` terminators**, five in all. Omit
> them and the record desynchronises, and it has no resync point. `equipped_block` sends all
> five and a test counts them.

**Where it goes:** record offset **223**, between the three string flags (220-222) and the
final ungated `u8`. Verified by walking all 18660 bytes of `FUN_140304b20`: with
`presence={0}` it reproduces today's 224-byte record exactly.

**Ruled out, so nobody spends a run on them again:**

* **`0x0138` is dead code at byte level.** Its apply is guarded by a call to
  `0x1407f5ce0`, which is three bytes of `xor eax,eax; ret`, then `TEST/JZ`. No trigger or
  timing would ever have worked. The earlier "the list at `user+0x1200` was empty"
  explanation was **wrong**.
* **`0x0107` only logs** - it formats `"[BP:%02d] %d"` for 32 body parts and applies nothing.
  (Potentially a free read-back instrument.)
* **`0x0114` never reaches the avatar-apply primitive**, by a reachability walk with the
  `0x0138` path as a passing control.
* **No inbound opcode reaches `FUN_140f80140`** (the apply primitive) by direct call - all 23
  callers checked against the 273-case table.

**One value to watch, per the NPC lesson:** `dateExpire`, the `u64` at `+0x40`, is zero =
1601-01-01. If a run comes back "no fault, still naked", that is the first suspect.

### Undecoded traffic seen alongside all of this

`0x00D9` (every ~510 ms, coordinate-shaped - almost certainly movement), `0x013D`, `0x00B8`,
`0x02EB`, `0x01ED`, `0x0408`, `0x0184`, `0x0194`, `0x01A5`, `0x02DE`, `0x00ED`, `0x02B2`.
None is answered. None has caused a freeze, so none of them is a blocking request.

---

**Nothing authenticates.** The game socket still carries no credentials; the character is
identified by the migration row and nothing else.

---

## History - how the goal above was reached

Everything below is the record of getting a character onto map 1. It is still accurate and
several parts are load-bearing reference - the presence-array table, the stat-block layout
and the pre-flight analysis are all cited by the goals above. It is **not** a to-do list;
the work is in NEXT GOALS.

### Where the client was before world entry worked

Measured 2026-08-19, end to end. Login -> character select -> pick a character ->
"Connecting..." -> the enter-success sound -> it closes the login socket, connects to
`127.0.0.1:8485`, accepts the channel greeting, and sends `0x0070` then `0x007D` (the
migration hello, character id at offset 8).

**We now answer it, and the answer reaches the right handler.** A probe caught the client's
own dispatcher entering `FUN_142097f80` *while dispatching opcode `0x01A0`*. Everything
between the migration and `SetField` is settled:

| | |
|---|---|
| the channel's dispatcher | `FUN_142cbaa80`, 273 cases, `0x70..0x39a` - `research/msexe-gamestage-dispatch.md` |
| the cipher | **asymmetric**: client sends AES, receives a byte subtract, so we **add** `iv[0]` |
| `SetField` | inbound **`0x01A0`**, handler `FUN_142097f80` - confirmed on the wire |
| the routing | reached from the **login** stage's `OnPacket`, so **no stage transition is needed first** |
| both early returns | pass - `[world+0x33f4]` measured `0x00` |
| the 33-byte fixed head | decoded field by field, `research/msexe-stage-setfield.md` |

### H1. The character record - the last wall (historical: it fell on 2026-08-19)

`SetField` must carry `characterData = 1`, and that branch calls **`FUN_140304b20`**, an
18525-byte decoder with **126 packet reads** (117 was the listing grep's undercount - it missed a `JMP` thunk and a `u64` primitive; see `research/charrecord-decode.md`). Sending `characterData = 0` instead **faults
the client** at `FUN_1402fa540+0x1c`: that short form is "same character, new map" and
assumes state a freshly migrated client does not have. Measured, not guessed.

What is already known, in `research/charrecord-decode.md`:

* **Field 1 is a 100-byte presence array** - one byte per flag, gating whole blocks. 43
  gates consult it. An earlier note here said there was *no* mask, because a scan for `BT`
  and `TEST reg,imm` found nothing; the mask is a **byte array**, not a bitfield, and the
  scan was looking for the wrong shape.
* **Field 1 is a fixed 100-byte raw block.** The decompiler renders its size as computed and
  it is a constant.
* **All 11 raw reads are constant-sized**: 100 once, 8 bytes ten times. 180 bytes, fixed.
* **No length prefix anywhere at the top level.** A wrong width desyncs everything after it
  and there is no resynchronisation point - this record works completely or not at all.
* `FUN_140302e30`, the stat decoder **we already build for the character list**, is called
  at `140304e71`. That part is known ground.

Companion passes: `charrecord-loops.md` (loop census and the straight-line spine),
`charrecord-reuse.md` (what `crates/net` already emits), `charrecord-v214-shape.md`
(candidate names from the reference - candidates only, the method scored 1 of 8 on a
held-out control).

**Read the listing, not the decompiler, for field order.** They disagree by five `u32`
reads; the listing is complete and authoritative. `research/charrecord-decode.md` shows the
working.

### 1a. RUN OF 2026-08-19: the record was accepted, and the map is what is missing

The owner reported it plainly: enter-success sound, **the screen faded to black**, and a few
seconds later the client exited. That fade is the stage transition, and it had never
happened before.

What the logs show, in order:

```text
0x01A0 accepted   142097f80 entered, latch 0x00, both early returns passed
+422 ms           the client SENDS 0x0238 and 0x024D, both empty bodies, both built by
                  FUN_142caa4e0 in the world-object subsystem - it entered the game stage
+3.4 s            CLIENT FAULT 0xC0000005 at 0x140ce89d6
```

**The fault moved**, which was the outcome to hope for. It is no longer `FUN_1402fa540`
(the `characterData = 0` short form). `FUN_140ce89c0` is a **reference-counted release**:

```c
obj = holder->[8];
if (obj != 0) { ... atomic_dec(obj->[0x28]); ... }   // faults reading [RBX+0x28]
```

The null check passes and the dereference still faults, so the holder contains a **non-null
but invalid** pointer - the signature of an object that was never properly constructed and
is then released during teardown.

Which is exactly what an all-zero record predicts. **The client accepted the packet,
transitioned, tried to load map `0`, failed, and faulted cleaning up.** `0` is not a map.

**So the next step is no longer "build the record" - it is "put the map id in it".**

### 1a-i. SETTLED 2026-08-19: the byte is `presence[0]`

The mapping the census could not prove is now read out of the client's own startup code,
with no client run. Full working in **`research/charrecord-presence-map.md`**.

* `FUN_1402fa9a0` is not an accessor. It is a **100-byte bytewise AND**:
  `out[i] = presence[i] & key[i]`. The gate runs its block if **any** byte of `out` is set,
  so every gate carries its own 100-byte key mask.
* Those masks sit in the uninitialised tail of `.data` and are built by **40 CRT dynamic
  initialisers** (pointer array at `.rdata 0x143264a00`), each of which zeroes 100 bytes and
  then sets **exactly one** to 1. So a gate fires **iff its one presence byte is non-zero** -
  40 gates, 40 distinct bytes.
* The gate guarding `FUN_140302e30`, the stat decoder, is entry 7. **Its byte is
  `presence[0]`** - which is byte **45** of the `SetField` body (33-byte head + three `u32`s).

**The census's own labelled guess - "key #k reads array byte k" - was wrong.** The mapping
is a permutation: entry 7 -> byte 0, entry 8 -> byte 62, entry 1 -> byte 44. Both research
documents that recorded the guess now say so. Building on it would have put every flag in
the wrong byte, and a client that skips every block looks exactly like one sent nothing.

### 1a-ii. SETTLED 2026-08-19: the map id is at stat-block offset 84

Working in **`research/charstat-layout.md`**; the flag-#7 region's own layout is in
**`research/charrecord-flag7.md`**. Three independent lines:

* The `u32` at `0x14030325e` is mangled into a 12-byte heap object hung off `record+0xfb`
  with the rolling-checksum seed `0x9a65`. `FUN_1402fa540` is the byte-for-byte inverse.
* `SetField` calls `FUN_1402fa540(user + 0xf3)` and hands the result to a lookup keyed by
  `PTR_s_mapName_143a49020`, which dereferences in `.rdata` to the ASCII string `mapName`.
  The neighbouring literal is `MAP` spliced with TAB, CR and LF.
* It sits immediately before `portal`, which is where `CharacterStat` puts a map.

**We had it in the wrong place.** `chr.map_id` went to record offset **120**, in the
character-list trailer, which is not on the `SetField` path at all - and offset 84 was a
literal zero. The test that was supposed to catch this scanned the record for any
`u32 == START_MAP_ID` and passed on any of them, so it could not fail. It now asserts the
offset, and that changing the map changes those four bytes and nothing else.

`param_3 == 0` on both paths, measured from `[RBP+0x3118]` being arg4's home slot, so the
`SetField` record and the character list share the identical 108-byte stat block. It is one
shared function now.

### 1b. READY TO RUN - and this is the run that can put a character on screen

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

Verified first without spending a launch: `python tools/channel_smoke.py --set-field-probe`
decodes the real server's real bytes with the independent Python transport and reads
`presence[0] = 1`, the character id, and the map at stat-block offset 84. 27 checks pass.

The probe's two free slots are aimed at `140304b20` and `140302e30`, which discriminate:

| hook log | means |
|---|---|
| no WATCH lines at all | the hook never armed - conclude nothing, re-run |
| `140302e30` at character select | the **positive control**: the watch is armed and works |
| `140304b20` but not `140302e30` after the migration | the gate SKIPPED the stat block, so `presence[0]` is the wrong byte |
| both, after the migration | the gate opened and the stats decoded |

**Unlike every previous run, "nothing visible" is now a failure rather than the expected
result.**

**How to read the fault if it comes back.** `research/setfield-fault-shape.md`, written
before this run. The fault at `0x140ce89d6` is a **scope-exit destructor on a stack local**,
not the teardown of a long-lived object: both real callers of `FUN_140ce89c0` end
`LEA RCX,[RSP+N]` / `CALL` / epilogue / `RET`, and the holder's `+8` was non-null garbage
because the local was never written. That makes it a latent bug in the client's own error
path - **any** early exit from either function faults at the **same address**.

> So a repeat of `0x140ce89d6` would mean "that function bailed out again", **not** "the map
> id is still wrong". The address cannot say which failure it was. The signals that
> discriminate are the two watches and what appears on screen.

A fault at a **different** address is still progress, exactly as before.

### 1b-i. Pre-flight static analysis, 2026-08-19 - what was ruled out before the launch

| question | answer | where |
|---|---|---|
| Does map 1's **field data** exist, or only its name? | **It exists.** `Map/Map/Map0/000000001.img`, 21,532 bytes, parses clean, byte-identical in both data trees. Portal **0** is a type-0 `sp` spawn at (-189, 437) with footholds 48px below. No scripts, mobs or reactors, so the server owes the field nothing extra. Surveyed all 426 field images, not a shortlist. | `research/map1-exists.md` |
| Does the map-name lookup have a bad failure mode? | `FUN_1403999e0` looks the key up first and only builds `Map/Map/Map%d/%09d.img` on a miss. **Map 1 hits**, so the fallback - which contains a **non-returning `E_POINTER`** call - is not reached. A map with no name entry would be a different story. | same, addendum |
| Is the fault a dead end or a red herring? | **A red herring for diagnosis.** It is a scope guard over an uninitialised stack local, so *every* failure in those two functions faults at the same address. | `research/setfield-fault-shape.md` |
| Does the client **block** on `0x0238`/`0x024D`? | **No block is demonstrable.** Fire-and-forget send, two-thirds of the function runs afterwards, and the client's known blocking idiom (an entry guard on a latch) is absent. Both bodies are empty, confirmed three ways. No inbound handler resembles a reply. **Recommendation: send nothing and watch.** | `research/outbound-0238-024d.md` |
| Is the `portal` byte an index or a spawn id? | **Unsettled**, and `xref.py --field` does not answer it. **Moot for map 1**, where portals 0-3 are all `sp`. | `research/charstat-layout.md` addendum |

The falsifier for the "no block" call, worth knowing before the run: **the client alive but
every button dead, with `0x0238` or `0x024D` as the last inbound line in `world.log`.**

**The zero audit** (`research/setfield-zero-audit.md`) checked every field we send as zero
around the record. Its framing is the right one: **every byte outside the record is identical
to the packet the client already accepted**, so none of these zeros aborts the handler - what
changed is that a field now really loads, which promotes the zeros the *field object*
consumes. It ranked six; the top two were then read rather than inferred:

* **`FUN_14187e880(field, 0, 0)`** (head offsets 22/26) is a **28-byte plain setter** - two
  stores into `field+0xa8`, no division, no allocation, no indexing. The zero-divisor story it
  was ranked #1 for is not at the call site. **Not a reason to invent a width/height pair.**
* **`FUN_142d16ef0`** (head offset 17) is a `std::map` in-order walk gated on the head node's
  `_Isnil` byte, so it is a **no-op on an empty tree**. Sending `0` instead of the reference's
  `1` changes nothing on a first field entry.
* The **three `u32`s before the record are randomiser seeds**, passed intact as a triple and
  nowhere else - which also retires the `(fieldId, portal, ...)` candidate in
  `charrecord-flag7.md` §6. All-zero is the absorbing state of the LFSR family, so it is worth
  fixing - but the audit's own finding is that **zero cannot fault**, so it cannot stop a map
  loading. **Deferred to after this run** rather than folded into it.

**Decision: change nothing before the launch.** The variant under test is the record -
`presence[0] = 1` and a real map id. Everything else is byte-identical to the packet the
client accepted on 2026-08-19.

`research/msexe-setfield-aftermath.c` has the fault site and `FUN_142caa4e0`, the builder
of the two packets the client sent on entering.

### 1c. The all-flags-clear minimum record is 112 bytes

**Historical now** - this is the packet that was accepted on 2026-08-19 and faded to black,
not the one the server sends today. Kept because the skip-chain it documents is what makes
the zeros in the current record safe. Settled by the loop census,
`research/charrecord-loops.md`. With every presence flag clear,
every count zero and the boolean at `0x140304cf2` zero, the client reads **7 fields, 112
bytes**:

```text
raw[100]   the presence array - all zero
u8
u32
u8
u32
u8
u8
```

Verified by walking the gate skip-target chain end to end. Supporting facts:

* **61 straight-line reads**, and **31 of the 37 read-bearing loops collapse cleanly** on a
  zero count. Every counted loop is MSVC-rotated - guard before the head - so a zero count
  never runs the body. That was checked specifically, because a bottom-tested loop would
  read fields anyway and desync everything after it. **There are none.**
* **Six loops are not count-skippable.** Four have fixed trip counts (two of them adjacent,
  forcing 15 x u32 = 60 bytes); all four sit behind presence flags instead. Two are
  **sentinel-terminated**, `u16 key; while (key != 0) {...}` - sending `0` skips them but
  costs a trailing `u16`, and reading them as counts would desync.
* **All 43 gates confirmed** to take the 100-byte field as their input. The static keys live
  at `0x143abeb10`, stride `0x70`, and all 38 land on exact multiples - which is what makes
  the index assignment trustworthy. Flag **#17** is the biggest lever: 21 reads and 7 loops.
  One gate is an `OR` - the block at `0x140305104` needs flag #5 **or** #12.

**Not proven, and it is the thing to settle before building**: the mapping from key index to
*byte offset* inside the array. `FUN_1402fa9a0` was not in the listing the census used. A
1:1 index-to-byte mapping is plausible and is inference.

**And a caveat that bounds the whole estimate**: 26 non-primitive calls also receive the
packet pointer and consume bytes that are not among the 126 reads. Zeros still collapse
them, but any non-zero count costs more wire than the census implies.

### 2. Then send it and run

`crates/world --set-field-probe` already builds and sends the head with `characterData = 0`.
Swap in the real record, check it with `python tools/channel_smoke.py --set-field-probe`
first - that validates framing, the cipher and every field offset without spending a client
launch - then run
`powershell -ExecutionPolicy Bypass -File tools/test-server.ps1 -SetFieldProbe`.

### 3. Then the ordinary game-stage work

Whatever the client asks for once the map loads. `research/msexe-gamestage-outbound.txt` is
the set of 175 client -> server opcodes the game subsystem builds, which is what to expect.

### Standing, and unchanged by any of this

* **Nothing authenticates.** The channel claims a migration by character id and the row is
  single-use; that is not a proof of identity. Say so when reporting.
* **`0x00BC` is still undecoded** and deliberately unnamed, so it logs in full.
* **`conn+0x48` meant exactly what the code said** - that question is closed. All three of
  its readings hold; a channel is simply **asymmetric**, and the retraction that once stood
  here had generalised a measurement of one direction to both. See
  [[maplecw-connection-type]] and `docs/transport.md`.


### BUILT 2026-08-19: the channel greeting, and a cipher that turned out to be wrong

* **`net::handshake::channel_greeting`** - the login greeting minus its two gated blocks,
  **26 bytes shorter**: `A..F` is 14 (it is easy to forget `B`'s 2-byte length prefix, and a
  test pins it) and the version block is 12. It starts at `G`, which is the field whose
  zero-read raised "The client is outdated". **Confirmed on screen**: the client accepted it
  and went on to send packets.

* **`net::ByteShiftCipher`** - `out[i] = in[i] - iv[0]`, and it **is** the channel's
  server -> client cipher. `crates/world` sends with it in `Shift::Add`, because the client
  subtracts on receive; it decrypts the client with `MapleCipher`, because the client sends
  AES. This entry said the opposite for most of a day - see the three-pass history in
  `docs/transport.md`, which is the more useful thing to read than this bullet.

  The methodological point outlasts the code: the polarity was going to be settled by
  logging a body under both readings and seeing which named a plausible opcode. That
  discriminator was sound. It was aimed at the wrong question, because the premise it rested
  on - that `conn+0x48` selects the cipher - was never checked against the wire.

### RETRACTED 2026-08-19: the login handshake was never failing

An `ELog` record showing `INVALID_CLIENT_VERSION` at the `G`/`H` gate was reported here as
the login connection failing on every run. **It was the previous run's migrated connection.**
The upload is a file replay - `FUN_1415ddd10` reads a log file and deletes it - so a record
that arrives at startup describes an *earlier* failure. `docs/handshake.md` carries the full
retraction and `tools/decode_elog.py` now carries the drain/experiment/re-read protocol.

The `conn+0x48` explanation is untouched and is now the *only* explanation needed: a channel
connection skips the greeting's gated blocks, so it reads `G` from our `A` field and raises
`0x348`. Login passes. Channel fails. One cause.

Two corrections fall out. A site-`840` record means the handshake **died** - `FUN_140cc2350`
reaches `_CxxThrowException` then `INT3`, and `FUN_1415d10e0` has zero catch funclets. And
**"First Connect" is dead code**: all three call sites pass `param_3 = 0`, which retires the
`high == 100` gate.

### 2026-08-19: the client has been reporting a failed login, and we called it a greeting

`0x00C0` was in the log table as `CLIENT_HELLO`. It is a **login failure report**.
`FUN_141b2a660` builds it *only* when `FUN_141d60eb0(user, pass, 0xc9, 0)` returns
non-zero, and the body is the launch mode followed by the error code. Every capture we have
carries **`0x4E20` = 20000, a Nexon Passport error**, twice per run.

That matters beyond the name. `docs/session.md` says the login form is vestigial and the
server supplies the identity - which is still true of what reaches *us* - but the client is
evidently making a local auth call and getting a failure, and it says so on the wire. Nobody
had read it. Whether that failure is connected to the handshake's `INVALID_CLIENT_VERSION`
is **not established**; they are two separate reports from the same run.

Five of eight inferred opcode names were wrong. Evidence per opcode in
`research/msexe-client-opcodes.md`; the corrected table is `crates/net/src/names.rs`.


### 2026-08-19: the migration works, and then the client says it is outdated

**Confirmed on screen.** The client selected a character, `FUN_141b36f60` was entered while
dispatching `0x0011` and ran its **whole** body (2755 us, against 355 us for the login
result - so not an early bail), closed the login socket, and **opened a second connection**.
The `0x0011` decode is right.

It then showed **"The client is outdated"** and exited cleanly (code 0, 38.7s).

**That dialog is the handshake's, not the migration's.** `0x22000007` is raised from four
sites, all inside the greeting check - this is the same dialog the whole of
`docs/handshake.md` was written about. Our server sent connection #2 the *identical* 48
bytes that connection #1 accepted, so the failure is state-dependent, not layout-dependent.
`FUN_1415d10e0`'s `param_3` selects "First Connect" from "Second Connect", and connection #2
is the second - a path this project has never exercised.

**Two things were wrong by construction and are now fixed:**

1. The client reconnected to the **login server**, because that is what the migration packet
   advertised. A login server answering a game connection is wrong whatever the dialog says.
   `crates/world` is now a separate per-channel process (the owner's instruction), and
   `World::channels` is one address per channel rather than a count.
2. The login server answers a new connection with an unprompted **`0x0032` startup gate**.
   That releases the *login* connection's startup loop; a channel has no startup loop.
   `crates/world` greets and then waits.

**The next run names the failing gate.** `FUN_140cc2350`, `FUN_1415e0e30` and
`FUN_1415e0fb0` - every raiser - funnel into **`FUN_141804870`**, so one watch identifies
the site. `rdx` is the site id (`0x348` G/H, `0x2df` L, `0x33b` second connect, `0x327` first
connect) and `r8` is the error code. `tools/test-server.ps1` arms it by default.

**`-Session mode=2` is off by default now.** It was meant to stop mode 5's auto-login but is
applied on `0x0000` dispatch - our *reply* to the login request - so it always landed after
the auto-login it was meant to prevent. It never did its job, and it writes `session+0x68`,
which `FUN_1415d10e0` reads on every connection including the channel one. That also closes
goal 4 below by deleting it rather than fixing it.


In the order that unblocks the most. Everything above the line is done and confirmed on
screen; nothing here is speculative work.

1. **Enter the game world.** `0x0078` is the select-character request and carries the
   character id - captured twice, with 203 and 204. It is unanswered, which is why the client
   sits on "Connecting...". This is Stage 4 and the biggest single step left.

   **The reply is `0x0011`, and that is no longer a candidate - it is identified, statically,
   with no client run** (2026-08-19). `case 0x11` is `FUN_141b36f60`, and it is the *only*
   login-stage handler that builds a `sockaddr_in`: `htons` appears exactly once across all
   fourteen decompiled case handlers, and it is in this one, which then calls
   `FUN_1429f14c0(PTR_u_GameIn_143a47c08, 100)` - the string **`GameIn`**.

   **Decoded so far** (full write-up in `docs/opcodes.md`): `u8 result` (0 proceeds, through
   the same `FUN_141b267c0` gate as `0x0000`), `str message`, `u8`, then `u32 ip` (four
   octets in order, straight into `sin_addr`), `u16 port` (the client `htons`es it),
   `u32 characterId` (looked up in the map at `DAT_143ac9890`; a character the login result
   did not send fails the lookup and skips the whole action block), three `u32` (the second
   one non-zero makes the client load `Etc/SpecialServerInfo.img` - send zero), a flags byte,
   `u32`, `u8`, four discarded fields, `u8[8]`, then `u32 key` and `u32 length`.

   **The tail is decoded too, and the packet is built.** The last `length` bytes are
   transformed in place and read back over; a second nested pass follows but its key is
   `FUN_140738db0(0x80000000, 0x7fffffff)` - a **random number** - and it happens after every
   read, so it is a scramble-back of already-consumed bytes, not a decode. Only the first
   pass matters, its key and length are both fields of the packet, and it inverts. Sending
   `length = 4` covers the client's one 4-byte read with a single aligned word and no partial
   tail.

   **`crates/login` now answers `0x0078`.** `Config::advertise` is new and is deliberately
   not `bind`: the octets go straight into the client's `sockaddr_in`, so the address must be
   reachable from the *client* machine - the first place the homelab move actually bites.
   Refusals use `0x0A`; see `docs/login-server.md` for the three separate ways a
   wrongly-chosen refusal code migrates the client anyway, opens a browser, or returns in
   silence.

   **What is left is one client run, and it has a falsifiable prediction.** The migration
   hands the client a `u32` seed. The only thing that reads it back is `FUN_1415d10e0`, the
   builder for outbound **`0x007D`** - so the client should reconnect and send `0x007D` with
   our seed inside. The server numbers connections and `describe()` names that packet, so the
   log alone answers it. The seed is `0xC0DE0000 ^ id`, **a placeholder, not a token**.
2. **Real sessions for multiple accounts** (the owner, 2026-08-18; testing-grade is fine). The
   token-in-`0x0073` route is measured dead, so this is one login server per account per
   port, or the `grap-stub` identity patch. `Session` should take its account from a resolver
   rather than from `Config` either way, so the swap is one function.
3. **`create=on`, the last honest patch.** Static analysis is exhausted (see below). The
   remaining route is the **in-process opcode walk** with `FUN_140c9e230` as the oracle: one
   launch covers the whole inbound opcode space. **Blocked on a small change** - `-Probe`
   takes either a walk range *or* watch targets, and this needs a walk plus the two mandatory
   patches (`1415db360:ret`, `141b2a280:rdx=0`) or the client dies at ~37s and the dialog
   blocks the screen. If the walk comes back empty, `create=on` is a permanent workaround for
   a client-side entitlement rather than a placeholder for protocol - which is worth knowing,
   because it turns a TODO into a fact.
4. **The auto-login ordering**, which the owner spotted. `mode=2` is applied when opcode `0x0000`
   is dispatched - and `0x0000` is our *reply to* the login request, so it lands after the
   auto-login it is meant to prevent. Patch the mode when the session object first becomes
   readable (the hook log shows that at +4s, long before login) instead of on a dispatched
   opcode. Hook work, not protocol.
5. **Stage 3.5 off-box**: the firewall carve-out, `crates/auth` with a configurable bind and
   TLS, `crates/launcher`. See `docs/deployment.md` and `docs/launcher.md`.
6. Smaller: the slot count should come from the account rather than the constant `3`.

## DONE - a real server, and characters that persist

**The owner set this on 2026-08-17; built 2026-08-18.** `crates/login` answers the client from a
database instead of from canned bodies, and a character created in one run is in the
character list of the next. Full notes in **`docs/login-server.md`**.

### What it is

| module | holds |
|---|---|
| `handshake` | the greeting - 48 bytes, unencrypted, server speaks first |
| `session` | the whole protocol as a **pure state machine**: bodies in, bodies out |
| `server` | the socket loop, framing, the log. Nothing protocol-shaped |

`session` having no socket and no clock is the point: every exchange measured against the
real client is a unit test, so a change can be checked without spending a client launch.

Storage is `crates/store/src/character.rs`: a `characters` table whose columns match the
protocol's `Character` field for field, plus an `equipment` table keyed by character and
slot, both cascading from `accounts`. The row maps to `net::opcode::Character` by
**exhaustive destructure in both directions**, so adding a protocol field breaks the build
until a column exists - a separate storage struct would have meant mapping nineteen fields
by hand, and a missed field is a stat that silently does not persist.

Character ids are database rowids, so they are distinct per character. That is a protocol
requirement: a canned reply that sent id 200 twice made the client drop the second character.

### The rule the whole thing is built around: always answer

**An unanswered request freezes the client's entire UI** - every button, including the OK on
the quit prompt. No path in `session` returns an error instead of a reply; a database
failure becomes a refusal the client can render, and the reason travels in the log label.
Pinned by `every_request_the_client_blocks_on_gets_an_answer`.

### The name check is the first genuinely honest answer

The harness replied "available" to every name, including ones it had already handed out.
The server checks the database, distinguishes available / already used / not allowed, and
names are unique across the **whole service** case-insensitively - the client's request
carries no account to scope by.

### How it was verified without a client launch

`tools/login_smoke.py` is a **stand-in client** built on `tools/transport.py`, an
implementation written independently of the Rust one from the client's own receive path. So
agreement between them is evidence, not one module agreeing with itself.

Verified 2026-08-18 against a fresh database: greeting accepted, gate delivered and repeated
to a quiet client, login answered with four packets in order, creation permitted, a free
name available and the same name taken immediately after creating it, create accepted, the
new character in the next login result - then the server **stopped and restarted**, and a
new connection still listed it. `maplecw-login --list` and an independent `sqlite3` read
agree.

**What that does not prove is the client's reaction.** Only a launch shows whether the
character is drawn, and drawn correctly. Every packet-level fact in this repo that turned out
to be wrong was wrong about a body the client read differently, not about a byte count.

### CONFIRMED ON SCREEN 2026-08-18 - creation, persistence, and the slot limit

The owner created three characters against the real server and reported all three working. This
closes the priority end to end.

| what | evidence |
|---|---|
| the create transitions back to character select | on screen |
| **three characters persist** | ids 201, 202, 203 in `maplecw.db`; a *fresh server* and a *fresh connection* return all three in the login result |
| **the three-character limit works** | "Create a character" is **disabled** at three, on screen, with no patch for it |
| entering the world hangs on "Connecting..." | expected - there is no channel server |

**The transition bug was the character id.** The first run created `TestChar` and the client
stayed on the creation screen. The reply was byte-identical to the one that worked on
2026-08-17 **except the two copies of the id** - `1` against `200`. Ids now start at 200
(`FIRST_CHARACTER_ID`, seeded through `sqlite_sequence`) and the client accepts them.

**Do not renumber characters from 1 again.** That is the whole finding.

**The slot limit needed no new code.** `login_result` already sends a truthful list and
`slotCount`, and `FUN_141b282d0` computes the free slot from them. So the client-side half
of the owner's "enforce it properly" goal was already satisfied by telling the truth; the
server-side half was already tested. What remains is `create=on`, which is a **different**
gate - it enables the button *at all* - and removing it is finding a packet.

### CAPTURED FOR FREE: the select-character flow

Entering the world sent three packets nothing answers yet. Bodies are in `login.log`
because it records them now. First read, from one capture:

```text
0x0078  73B  u32 0 | str "." (the placeholder PIC) | u32 203  <- the character id
              | u8 0 | str "AA-BB-CC-DD-EE-FF, 00-00-..." | str "AABBCCDDEEFF_DEADBEEF"
0x0079  67B  u32 203 | str "TestCharC" | u32s | SYSTEMTIME 2026-08-18 13:36:32
0x00BC  12B  09040000 09040000 09040000   - 1033 three times
```

**`0x0078` carries the character id** (`cb000000` = 203 = `TestCharC`), which makes it the
select-character request and its reply the migration packet - our `0x0011` candidate, and
Stage 4. The MAC list and machine id are there again: record, never gate on.

`0x0079` is a client report carrying a timestamp; the client did not block on it.

### MEASURED: a launch-argument token does NOT reach the server

**Settled 2026-08-18**, and it closes a question open since the launcher was designed. The owner
launched with `-SessionTokens "tokA tokB tokC tokD tokE tokF"` and `0x0073` came back
**byte-identical to the run without them**:

```text
26B  05000000 0000 aabbccddeeff deadbeef 00000000 764d0000 0000
     mode=5   ""   MAC          machine id
```

No token text anywhere in the log. `-NXLDEBUG` does route arguments 3 onward into the client
config at `+0x90`, but **nothing carries them onto the wire**.

**This kills the design where the launcher's single-use token rides in `0x0073`.** See
"Next goals" below for what replaces it.

**The tokens also broke that run:** a "trouble connecting" dialog appeared immediately after
the splash, and `FUN_141b2a280` - the function we suppress - was **never entered** (its watch
was armed and logs every call; the hook log has no `WATCH` lines). So it came from a
different path. **Do not pass `-SessionTokens` in ordinary runs.**

### DONE: delete a character

**`0x008B`, confirmed on screen 2026-08-18, first try.** The owner deleted `TestCharB`; the request
was `0x008B` with body `ca000000` (202), answered `0x0016 deleted "TestCharB" (id 202)`.

**It was read, not captured.** `FUN_141b28750` is the Delete button handler and is not
virtualised; `research/msexe-send-opcodes.txt` already listed the opcode against it. Checking
that table before guessing saved a launch. The counterexample is `0x008A`, which *is*
virtualised - **absence from that table means virtualised, not non-existent.**

Reply is `u32 characterId, u8 result`, correcting an earlier note that called it a single
`u32`. Two traps, both now pinned by tests:

* **A refusal must use `6`.** The switch names `6, 9, 10, 0x10, 0x12, 0x14` and a *default* -
  and the default is the branch that removes the character. Any other non-zero code deletes
  it anyway.
* **An unanswered delete disables the button for the session.** The builder sets
  `stage+0xd4` before sending and returns early while it is set; only the result clears it.

Ids are **not reused**: the freed 202 was not handed to the next character, which got 204.

### SETTLED: `create=on` cannot be resolved by reading the image

The owner asked for the packet that sets the create-character flag. **Static analysis is finished
and the answer is that no readable code sets it.** Every route checked, each with a control
proving the instrument finds things:

| scan | result | control |
|---|---|---|
| direct `call` to the setter `FUN_140c9e230` | **0** | the getter: 2 found |
| its address as a qword (vtable / fn table) | **0** | `FUN_141b25f30`, a known vtable entry: found in `.rdata` |
| the one cluster function with a live caller | initialisation | decompiled: a run-once latch that seeds protected values and never calls the setter |

The flag is a six-byte self-checksumming blob that reallocates every 0x6f accesses, seeded
during the connection handshake. `FUN_14003fb80` is a second orphan of the same shape. The
unpacked Nexon DLLs hold no creation-shaped strings.

**The evidence supports the flag being flipped from Themida-virtualised code.** The remaining
route is runtime - see "Next goals".

**One scan is not trustworthy and must not be quoted:** references to the blob pointer
`DAT_143ac8170` come back zero even with REX.R forms added, which cannot be right when three
functions dereference it. The *counter* scan (`DAT_143ac8168`) works and is what found the
cluster.

### SETTLED: the launch keyword is not a lever - stay on `-NXLDEBUG`

**The owner's hunch, checked statically 2026-08-19, entirely by reading the image - no client
run.** Full write-up in `docs/launch-protocol.md`; new decompilation in
`research/msexe-launchmode.c`, `msexe-launchconfig.c`, `msexe-modeclass.c`,
`msexe-loginbutton-modes.c`.

**`-NXLDEBUG` is not a debug mode.** The chain is short and now fully verified:

```text
argv[0] keyword -> cfg+0x38 -> FUN_142c95c90(cfg) -> session+0x68 -> FUN_142c4a810(session)
```

`FUN_142c95c90` is `return *(u32*)(cfg+0x38)` and has exactly **two** callers; the session
constructor `FUN_142c43db0` writes `session+0x68` from it **once**, and nothing else in
readable code writes that field. And `-NXL`, `-NXLDEBUG` and `-NXLPTS` all write the same
value: **5**. So `-NXLDEBUG` selects the identical mode the real Nexon Launcher passes; the
`DEBUG` suffix only changes which argv slots map to IP and port. It sets no debug flag and
relaxes no check.

| keyword | mode | reachable? |
|---|---|---|
| `GAMELAUNCHING`, `IPPORT` | 2 | only with a whitelisted IP (below) |
| `WEBSTART` | 3 | needs token 1 non-empty |
| `STEAMSTART` | 4 | (the doc previously mis-attributed mode 4 to "no keyword") |
| `-NXL`, `-NXLDEBUG`, `-NXLPTS` | 5 | what we use |
| anything else | *unset* | opens the Nexon micro-site and returns |

**Why `IPPORT` "crashed":** it `strcmp`s the IP against six hard-coded literals
(`10.9.2.131/132/133`, `44.234.166.161`, `44.234.167.163`, `44.234.163.43`) and on a miss
opens the micro-site, reports error `0x18a`, and returns **without setting a mode**. It was
never a crash in the parser - mode 2 was simply never reached.

**Mode 2 is not the mode we want anyway.** The login screen's buttons fork on `mode == 5`
(`FUN_14112a570`): `login` calls `FUN_141b3ff10` (the login request) in mode 5, and
`FUN_141b3f050(stage, 4, 600)` otherwise - a 600 ms fade to **screen 4, CharSelect**.
(This line said "world select" until 2026-08-19; see the retraction below.)
So mode 2 restores the classic WorldSelect -> ChannelSelect flow; it does **not** make the
client authenticate, so it does nothing for multi-account. It also silently kills the
login-screen Quit button, which reads as a freeze.

**`-NXLPTS` buys nothing:** its `cfg+0xc8` has no reader, and its `DAT_143a88df8 = 0` is
read in exactly one place, `FUN_141b5bb50`, the login-screen draw.

**The client's own classifier** is `FUN_1401e7bf0(mode) { return mode==3||mode==4||mode==5; }`
- "a launcher started me" vs "I was started directly". Worth grepping for alongside `== 5`.

**Static confirmation of the token measurement.** `FUN_142c95f20(cfg,out,i)` reads
`cfg+0x90+i*8` - the six session tokens - and `Xrefs` returns **no callers**, in a run where
sibling accessors did return callers, so the instrument was working. Two independent
instruments (this and the wire capture) now agree that a launcher token cannot ride in
`+0x90`.

**Only remaining launch-line option worth anything:** `-NXL <anything> <region> <ip> <port>`,
purely to set the region string at `cfg+0xc0`, which `-NXLDEBUG` leaves at the config
default. Worth one run only if something is ever traced to the region.

### Still open

* The slot count is the constant `3` rather than a property of the account.
* Entering the world. **`0x0078` is answered** - with `0x0011`, and the client migrates. What
  is left is the character record `SetField` must carry; see NEXT GOALS. (This entry used to
  say `0x0078` was unanswered and that this was why the client sat on "Connecting...". Both
  halves stopped being true on 2026-08-19.)

## NOT AUTHENTICATED - say so when reporting

**Nothing on the game socket proves who the player is.** The client's login request carries
no credentials; the login form is vestigial and the server supplies both the login result
*and* the account name. So `--account` decides whose characters every connection sees, and
two different people connecting are the same account. The server prints this at startup:

```text
serving every connection as account "maplecw" (id 1), 1 character(s) stored
NOT AUTHENTICATED: the game socket carries no credentials, so anyone who
  connects is served as that account. See docs/launcher.md.
```

Closing it was to be Stage 3.5: the launcher authenticates against `crates/auth`, gets a
single-use token, and the token reaches the login server so it can call `/consume`. **The
route that design assumed is now measured and dead** - the client does not put launch
arguments on the wire, so the token cannot ride in `0x0073`. What is left:

* **one login server per account, each on its own port**, the launcher choosing the port.
  Crude, needs no protocol, works today, and the owner said testing-grade is acceptable for now;
* or write the identity string at `DAT_143ac1898+0x1b8` from `grap-stub`, which is already
  in-process. `0x0073` sends it as its second field and it is empty because nothing computes
  it. That is a **client patch standing in for a real session**, honest only if labelled.

Until `/consume` gates the login result, do not describe a session as authenticated.

### Build it for two machines from the start - see `docs/deployment.md`

The owner will host this on a homelab box, so **the client and the server are not the same
machine**. Designing for that now is cheap; retrofitting it is not. The three things that
actually change:

1. **The firewall rule breaks the moment the server moves off-box.** It blocks all outbound
   from the client, and has been harmless only because *Windows Firewall does not filter
   loopback* - the script's own docstring says so. Off-box, our own traffic is caught by the
   rule that blocks Nexon. It has to become a block whose remote address is the complement of
   the server. The twenty Nexon addresses must stay blocked, so `-SkipNetCheck` is still
   required either way.
2. **Bind address is not advertise address.** `crates/login` binds `127.0.0.1:8484` by
   default and takes `--bind`; there is no `advertise` yet, because **nothing we send carries
   an address**. Whatever packet eventually does must carry one the *client* can reach.
   `0x0011` is the candidate and it is Stage 4.
3. **Machine identity must not be an authorisation input.** `0x0073` and `0x0078` carry a MAC
   list and a machine id; record them, never gate on them, or a second machine cannot play.

`crates/auth` already has the right shape - `POST /login` for the launcher, `POST /consume`
for the login server, tokens stored only as hashes. It binds loopback only today and will
need a configurable bind plus TLS.

### And a launcher - see `docs/launcher.md`

The owner asked for a minimal launcher that applies the client patches and takes a username and
password, since the real client uses a validated session. The design is written, but the
route it assumed is **dead**: the session array at config `+0x90` - which `-NXLDEBUG` fills
from launch arguments 3 onward - is not transmitted in `0x0073` (measured on the wire) and
has no reader in readable code (`FUN_142c95f20`, `Xrefs` with working controls). So the
launcher cannot hand the server a token through a launch argument. See
`docs/launcher.md` for the two routes that remain.

## SOLVED - the ~37 second exit (kept for the method, not the answer)

**How it dies is now measured: exit code `0xC0000409`, `STATUS_STACK_BUFFER_OVERRUN`.** On
x64 that is `__fastfail` - `int 0x29`. The client ends *itself*, deliberately.

That one fact explains every negative collected before it, and they were all real:

| what was seen | why `__fastfail` produces it |
|---|---|
| no vectored handler ever saw a fault | `int 0x29` traps straight to the kernel and is never dispatched to user-mode handlers |
| `RtlExitUserProcess` never entered | a fail-fast does not go through the ordinary exit path |
| `NtTerminateProcess` never entered | same |
| no thread drain before the process vanished | every thread is torn down at once by the kernel |
| nothing external held a terminate handle | there is nothing external to find |

**RETRACTED: "a crash or `__fastfail` is ruled out".** That rested on the Windows
Application log holding no error for these exits. The log does work - it holds a real
MapleStory `0xc0000005` - but a fail-fast is not required to produce a WER Application
Error entry, and here it produced none. The log-based negative only ever covered
WER-reported crashes, and it was stretched past what it could carry.

**It is not an external kill, and that is now measured rather than assumed.** The elevated
handle scan through the client's whole life found only `lsass`, three `svchost`s,
`RadeonSoftware` and `audiodg` holding handles to it - no Nexon process, no protection
process, nothing that appeared before the exit. The owner's anticheat-service hypothesis is not
supported.

**The instruments did speak, which is what makes the silence readable.** Both
`ntdll!RtlExitUserThread` and `ntdll!NtTerminateThread` armed with verified `int3`s and
each fired six times for ordinary thread exits, the last 5.8s before death, with the
200-hit cap nowhere near reached. Neither fired at the exit, exactly as a fail-fast
predicts.

### SETTLED: the deadline is anchored to process start, not to anything on the wire

| run | launched | exited | from launch | from login result |
|---|---|---|---|---|
| 4 - created a character, attempted to enter the world | 20:50:02.083 | 20:50:39.040 | **36.96s** | 27.0s |
| 5 - nothing clicked at all | 20:59:42.906 | 21:00:19.793 | **36.89s** | 23.8s |

**0.07 seconds apart from launch; 3.2 seconds apart from the login result.** The client
`__fastfail`s on a fixed wall-clock deadline of about 36.9 seconds from process start. Not
a CPU quota either - the two runs burned 12.13s and 11.64s of CPU.

Run 5 settles more than it was asked to. The owner touched nothing, and **the client logged
itself in anyway** at +7.4s, sending `0x00C0`, `0x0073` and `0x0080` on its own. So the
login-result interval in both runs is the client's own timing rather than a human's, and it
still moved by 3.2s while the launch interval did not move at all.

**Therefore the exit is not a protocol timeout. The server owes the client nothing**, and
everything measured on the wire is beside the point for this bug.

### Correction: `-Session mode=2` does not prevent the auto-login

The mode byte is patched when opcode `0x0000` is dispatched, and in run 5 that landed at
20:59:56.506 - about 100ms *after* the client had already sent its login sequence at
20:59:56.4. A patch cannot prevent an auto-login it arrives after. What carries the client
to character select is the login result, not this patch. Keep the patch (the transition
behaviour downstream depends on it), but the claim in its log line that "the tick should no
longer auto-login" is wrong.

### SETTLED: our own patching is not the cause

| run | patches in the image | lifetime |
|---|---|---|
| 4 | full, clicked through creation | 36.96s |
| 5 | full, nothing clicked | 36.89s |
| 6 | **none** | **36.70s** |

Run 6 used `-NoPatch`: no dispatcher detour, no `int3`, no session patch. The control is
verifiable rather than assumed - the only line the hook wrote that run is
`install_once: our code IS running. env=false marker=false -> standing down`, and all three
marker files were absent. The client's `.text` was exactly as installed, and it still
`__fastfail`ed on the same deadline.

So the `int3` watches are free to use for this. They change nothing.

### The fail-fast is an ordinary CRT fatal error, not an anticheat kill primitive

`tools/ghidra_scripts/FindFastFail.java` walked the disassembly - 11,684,028 instructions -
and found **8 real `int 0x29` sites**, all in `.text`. The raw byte scan had reported 63,
because x86 is variable-length and a byte scan is not instruction-aligned; that number was
eight times too high and should not be quoted again.

Every one sits behind the standard MSVC preamble
`MOV ECX,0x17; CALL [IsProcessorFeaturePresent]; TEST EAX,EAX; JZ skip`, and the reason
code loaded into ECX names the path:

| site | reason | what it is |
|---|---|---|
| `FUN_142f048cc`, 44 callers | `7` FATAL_APP_EXIT | `abort()` |
| `_invoke_watson` @ `142f04834`, 52 callers | `5` INVALID_ARG | the CRT invalid-parameter handler |
| `FUN_142ef3e44` | `2` STACK_COOKIE | `__report_gsfailure` |
| `FUN_142ef4c1c`, 5 callers | from `EBX` | a generic `__fastfail(code)` wrapper |
| `FUN_142ef3f2c` | from the stack | another wrapper |
| `__except_validate_context_record` | `0xd` | SEH context validation |
| `__except_validate_jump_buffer`, 2 sites | `0xd` | `longjmp` validation |

**This reframes the bug.** A deliberate "protection decided to kill you" would not go
through `abort` or the invalid-parameter handler. An uncaught C++ exception reaches
`abort` through `terminate`, and that is a timeout in the client's own code failing in a
way nobody caught - a very different thing to chase than an anticheat.

**Caveat, stated because it changes what a negative would mean:** this scan covers code
Ghidra disassembled, which is `.text`. `.boot` is Themida's own 12.8 MB and holds 140 raw
`CD 29` byte matches that the instruction walk did not see, so a fail-fast inside the
packer's runtime would not appear in the table above. If none of the four watches below
fire, that is where to look next.

### FOUND: a stack cookie failure in a Themida-virtualised function

The `-FastFail` run named it in one launch. Of the four watched CRT entry points, exactly
one fired:

```text
21:15:39.557 WATCH #1: 0x142ef3e44 ENTERED on tid 140940 ... called-from=0x142e9fe03
21:15:39.783 EXIT code 0xC0000409 after 36.6s
```

`0x142ef3e44` is `__report_gsfailure`, reason code `2`,
`FAST_FAIL_STACK_COOKIE_CHECK_FAILURE`. **A stack buffer overrun**, 226ms before the
process died - not `abort`, not the invalid-parameter handler. And it fired on **tid
140940, the main thread**, which was the first thread in the process.

`.pdata` puts the caller inside **`0x142e9ebd0 .. 0x142e9fe0c`** (4668 bytes).
`tools/pdata_lookup.py` is new and does this lookup: Ghidra had no function containing that
address and `getFunctionContaining` returned null, so `DecompileFunc` created one at the
epilogue, which decompiles to nothing. The PE exception table is authoritative - the linker
wrote it - and it has 120,981 entries covering every function with unwind data.

The function's frame is readable even though its body is not:

```text
142e9ebd0  MOV [RSP+0x20],R9B      four arguments homed: ptr, ptr, int, bool
142e9ebdf  MOV [RSP+0x8],RCX
142e9ebe4  PUSH RDI
142e9ebe5  SUB RSP,0x410
142e9ebec  MOV RAX,[0x143a8b908]   __security_cookie
142e9ebf3  XOR RAX,RSP
142e9ebf6  MOV [RSP+0x400],RAX     planted
142e9ebfe  JMP 0x144f94a9c         <- tail jump into .themida
...
142e9fddf  LEA RCX,[RSP+0x200]     a 0x200-byte local
142e9fde7  CALL 0x142e9e350        a thunk: XOR EDX,EDX; JMP 0x142e9e9e0 - a destructor
142e9fdfa  MOV RCX,[RSP+0x400]     the cookie, immediately above that local
           XOR RCX,RSP
142e9fdfe  CALL 0x142ef44b0        __security_check_cookie
142e9fe03  ADD RSP,0x410           <- the called-from the watch recorded
```

**Its body is virtualised.** The prologue plants the cookie in plain code and tail-jumps
into `.themida`; everything after that address disassembles as noise, which is exactly the
`halt_baddata()` blind spot, and `Xrefs` finds no references to it because its callers are
virtualised too. So the overflow happens inside the VM, in a body we cannot read, into the
`0x200`-byte local that sits directly beneath the cookie.

That Nexon chose to virtualise this particular function says it is security-relevant, and a
4668-byte function taking `(ptr, ptr, int, bool)` that builds something into a 512-byte
stack buffer on a timer has the shape of a periodic report builder.

### SOLVED: it is a server-reachability check overrunning its own buffer

The second `-FastFail` run answered it. `FUN_142e9ebd0` was entered **exactly once**, and
`__report_gsfailure` followed 150ms later:

```text
21:22:34.185 WATCH #1: 0x142e9ebd0 ENTERED ... rcx=0x14cb18 [0x0000067c]
                       rdx=0x14cb20 [0x0009000a] r8=0x14 r9=0x1 called-from=0x1415db7ac
21:22:34.335 WATCH #1: 0x142ef3e44 ENTERED ... called-from=0x142e9fe03
21:22:34.601 EXIT code 0xC0000409 after 36.7s
```

Once, not repeatedly - so it is on a timer, not a loop that eventually goes wrong.

`.pdata` puts the caller in `0x1415db360 .. 0x1415db7d6`, next door to the packet
dispatcher at `0x1415d60e0`, and **that function is not virtualised**. It builds a table of
twenty 4-`u16` groups on its stack and passes it as the second argument with `0x14` - 20 -
as the third. `rdx` dereferenced to `0x0009000a` in the log, which is `10, 9`: the first two
octets of the first entry. They are IP addresses:

```text
 1. 10.9.2.131        8. 44.234.161.18     15. 44.234.176.71
 2. 10.9.2.132        9. 44.234.171.239    16. 44.234.175.183
 3. 10.9.2.133       10. 44.234.78.153     17. 44.234.167.70
 4. 44.234.166.161   11. 44.234.182.63     18. 44.234.181.229
 5. 44.234.167.163   12. 44.234.171.56     19. 166.117.115.214
 6. 44.234.163.43    13. 44.234.162.137    20. 166.117.144.41
 7. 44.234.175.85    14. 44.234.159.5
```

Three Nexon-internal addresses, fifteen on AWS `us-west-2`, two more elsewhere.

**So the mechanism, end to end:** about 36 seconds after launch the client runs a
server-reachability check over twenty hardcoded addresses. The routine that does the work is
virtualised and writes its result into a `0x200`-byte stack buffer that sits directly under
its `/GS` cookie. **The patched client is firewalled, so all twenty fail**, and the
all-unreachable path overruns that buffer. `__security_check_cookie` catches it in the
epilogue, `__report_gsfailure` raises `int 0x29`, and the process dies with `0xC0000409`.

It is a latent bug in the client, on a path that never runs in production because the
servers are always reachable, and runs on every one of ours because they never are.

### The fix, and why it does not touch the firewall

Neither the buffer nor the routine can be fixed - the body is inside the VM. So the call is
skipped. `-Probe` grew a `:ret` option: log the entry and return immediately, leaving the
`int3` planted. Pointed at `FUN_1415db360`, the check never runs.

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-charselect.ps1" -SkipNetCheck
```

**CONFIRMED WORKING.** The client lived **92.7 seconds** and exited with code
`0x00000000` when closed by hand - no `__fastfail`. And the check turns out to be
**periodic, not a one-shot**: `FUN_1415db360` was entered at +36.2s, +66.2s and again
moments later, each returning harmlessly. `rdx` on those entries reads `30000` and `60000` -
millisecond timer values - and `called-from` is inside `.themida`, so the scheduler is
virtualised too. Every 30-second cycle would have killed the client.

**This is a client patch, and it does not make anything reachable - it stops the client
asking.** Report it as a patch. The `/GS` site stays armed alongside it, so if the client
dies anyway the log says whether it was still a cookie failure - a second overflow - or
something else.

**`141b2a280:rdx=0` must be in this mode's watch list**, and was missing from its first
version: without it the "trouble logging in" dialog blocks the tick that enables the Login
button, which cost a run. All four slots are used now, and the reason for each is in the
script.

Firewall untouched. Turning it off would presumably also stop the crash, by letting the
check succeed, but that means letting the patched client reach Nexon - which is what the
rule exists to prevent, and is the owner's call rather than ours.

**Do not** re-test the keepalive, re-watch `RtlExitUserProcess`, go looking for an external
killer, or blame our own patches. All four are settled, each by a measurement. And do not
re-quote "63 `int 0x29` sites" - that was a byte scan, and the real number is 8.

## Character creation: what works, and what is still fake

The protocol is finished and every opcode measured. Four problems found in the first
unlimited-length session, all fixed, all worth knowing about:

| seen on screen | cause | fix |
|---|---|---|
| the created character is naked | the `0x0015` reply was a canned body built from `Character::default()` | `--build`, which runs `packet-hex create-result-from` over the request itself |
| a second character never appears | that canned body carried **id 200 every time**, so the client was told it had re-created the character it already had | the builder takes an id and increments it |
| wrong hair, and no equipment | **the avatar look wrote `face` and `hair` one field too early** - see below | field order corrected, and pinned by a test |
| "Choose another world" freezes the UI, and re-entering a world hangs | `0x0082` was never answered, and the world sequence was one-shot | `0x0082` answered with the world list; the whole `0x0080` sequence is now standing |

### The avatar look field order - the subtle one

`FUN_1402ee8d0` reads `u8 gender`, `u8 skin`, three `u32`s, a discarded byte, one more
`u32`, then the equipment pairs. We were writing `face` and `hair` into the first two
`u32`s. The destinations say what those fields really are: the third `u32` goes to
`+0x1bd`, far from the look block, and the last goes to `+0x39`, which is **index 0 of the
equipment array** the pair loop fills at `+0x39 + slot*4`. That loop rejects anything
outside slots 1..31, so index 0 can only be written by the standalone field - and it is the
hair. The Swordie source names the same run `0, face, job, pad, hair`, and the client's
reader agrees with it. Correct order:

```text
u8 gender | u8 skin | u32 0 | u32 face | u32 job | u8 pad | u32 hair | pairs, 0xFF | pairs, 0xFF
```

**Why the tests did not catch it:** the record round-trip test *skips* the look block by
size rather than reading it, so it passed with the fields transposed.
`the_avatar_look_puts_face_and_hair_where_the_client_reads_them` now asserts the exact byte
run, and was checked by putting the bug back - it fails - and taking it out again.

### Now persisted

That was all measured against the harness, which generated the list from `-Characters` at
launch and persisted nothing. `crates/login` stores it. The four fixes above are still the
reason the transaction works, and the builders they corrected are what the server uses -
in particular the avatar look field order, which is pinned by a positional test.

## THE GOAL (set 2026-08-17) - reached on the wire 2026-08-18

**The server processes an entire character creation transaction.** The owner set this after the
masked email landed and the "connection dies" problem turned out not to exist. `crates/login`
does all four steps below against real storage; what has not happened yet is a client launch
to watch it.

That means, end to end and against a real server-side implementation:

1. the client reaches CharSelect with a **character list we sent**;
2. it asks the server to **check a name**, and the server answers;
3. it sends the **create request**, and the server creates the character and answers;
4. the client returns to CharSelect **with the new character in the list**.

### Where things stand

| | |
|---|---|
| Login screen | done, `0x0032` |
| Login button lit and clickable | done, `0x000B` world entry sets `stage+0x108` |
| Masked email on the login screen | **done**, `0x0000` - no client patch needed |
| Transition to character select | done |
| Connection stays up | **not a problem** - the "reset" was our own log format, see the retraction below |
| Character record, 327 bytes | **MEASURED** - drawn on screen with the exact stats sent |
| Character list | **MEASURED**, `login_result` |
| "Create a character" button | **works**, but only with the `create=on` client patch |
| Name check `0x0081`/`0x0014` | **MEASURED** both ways |
| Create request `0x008A` | **MEASURED** - virtualised builder, so a capture was the only way |
| Create result `0x0015` | **MEASURED** - the client returns to CharSelect with the new character |
| Client exits ~37s after launch | **FIXED** - a firewalled reachability check overran its buffer. `-SkipNetCheck` skips it; the runs since use the probe patch `watch@1415db360:ret` instead, which is what `test-server.ps1` arms and why the launch line carries no `-SkipNetCheck` |
| Server-side creation | **done** - `crates/login` reads the request, stores the character, and replies from the stored row |
| Characters persist between launches | **CONFIRMED on screen** - the same three characters have come back on every launch since |
| Name check answered truthfully | **done** - available / already used / not allowed, from the database |
| Valid session | still faked by client patches, and the game socket carries no credentials at all |

### The whole transaction, as measured - and now as served

| # | client sends | we answer | builder |
|---|---|---|---|
| 1 | `0x0080` world info request | `0x0000` account, `0x000B` world, `0x000B` end, `0x0010` login result | `account_info`, `world_list_entry`, `world_list_end`, `login_result` |
| 2 | `0x00A8` open creation (placeholder PIC, `01 00 2e`) | `0x05F4` `00 00` | `enter_creation_permitted` |
| 3 | `0x0081` check name | `0x0014` name + result | `check_name_result`, or `--answer 0081=0014:<req>00` |
| 4 | `0x008A` create, 101 bytes | `0x0015` result + record | `create_character_result` |

`0x00A8` and `0x0081` arrive on **every** click, so they need standing answers, not
one-shots. `crates/login` is stateless about all of them except the login request, which it
remembers only to decide whether to repeat the startup gate to a quiet client.

### Read these first
* **`docs/login-server.md`** - the login server: what it answers, how it stores, what is
  not authenticated, and how it was checked without a client launch.
* **`docs/deployment.md`** - running the server on another machine, and what breaks first.
* **`docs/launcher.md`** - the launcher, the patch inventory, and how each patch retires.
* **`docs/character.md`** - the whole transaction, the complete record layout, the NewChar
  screen, the create-request body, and what is still not established.
* `docs/opcodes.md`, `docs/session.md`, `docs/handshake.md`, `docs/transport.md`.
* `crates/net/src/opcode.rs` - every builder, every constant, each documented with how it
  was established. The tests there are the specification.
* `research/msexe-charstats.c`, `msexe-charrecord.c`, `msexe-avatarlook.c`,
  `msexe-newchar-ui.c`, `msexe-char-create.c`, `msexe-createflag.c`,
  `msexe-createbutton-gates.c` - the decompilation this rests on.
* `research/fixtures/` - the logs behind each claim, named for what they show.

### The tools, and what each is for

| tool | use |
|---|---|
| `tools/test-server.ps1` | **the run to use.** Builds, installs the hook, starts `maplecw-login`, applies the client patches, launches the client. `-Stop` tears down, `-ListOnly` prints stored characters |
| `maplecw-login` | the login server. `--list` prints what is stored without listening; `--bind`, `--db`, `--account`, `--world` |
| `tools/login_smoke.py` | a **stand-in client** over `transport.py`: proves the transport and every reply without a client launch. `--check-quiet`, `--list-only` |
| `tools/test-charselect.ps1` | the old harness. Canned bodies, no persistence - still the right tool for capturing packets or trying a hand-written body |
| `tools/test-one.ps1` | the general harness underneath it |
| `packet-hex` | prints a reply body from the Rust builders, so hex is never typed by hand |
| `tools/handshake_probe.py` | the old stand-in server. `--answer` standing, `<req>` splices the request's payload, **`--build IN=ELEMENT` computes a reply from the request by running `packet-hex`**, `--keepalive`. **Superseded by `crates/login`** for serving; kept for capture |
| `tools/transport.py` | the cipher, the framing, and the client-stream decoder |
| `-Probe watch@A,B,C` | up to four `int3` watches, each logging the calling thread id; `<module>!<export>` for relocated modules; options `:rdx=` forces an argument, `:peek=` logs `[rcx+off]`, `:hits=` sets the per-target log cap (default 32) |
| `tools/exit-forensics.ps1` | how the client died, from outside: exit code, thread table, job membership, handle holders. Started automatically by `test-one.ps1`; verified against a killed and an orderly control |
| `tools/handle-holders.ps1` | which processes hold a handle to a given pid and may terminate it. Read-only; **needs elevation** or the list is silently short |
| `tools/pdata_lookup.py` | exact function bounds from the PE exception table - **use when Ghidra has no function** for an address, rather than letting `DecompileFunc` create one at the wrong place |
| `tools/ghidra_scripts/FindFastFail.java` | the real `int 0x29` sites, by walking the disassembly rather than scanning bytes |
| `-Session mode=2,create=on` | the client patches, comma separated |
| `tools/ghidra_scripts/DecompileFunc.java` | decompile by address, creating the function if Ghidra has none |
| `tools/ghidra_scripts/Xrefs.java` | callers, and data references |
| `tools/ghidra_scripts/DumpAsm.java` | raw listing for a VA range - **use when the decompiler says "bad instruction data"** |
| `tools/ghidra_scripts/DumpData.java` | bytes as ASCII and UTF-16 - turns `&DAT_1433881d0` into `"new"` |
| Windows Application event log | records real client crashes; verified working |

### The Swordie comparison - how to use it

`C:\Users\user\Desktop\ModernMapleSource` holds a Swordie-family server (`v214 src`) for a
modern MapleStory. This client is **MapleStory Classic World**: a modern engine running
classic content, so the source matches its *structures* but not its *numbers*.

**What it earned:** `CharacterStat.encode` agreed with `FUN_140302e30` field for field
across the record head, which turned offsets into named stats; `selectWorldResult`'s
three-list shape matched `FUN_14108d290` + `FUN_14108bdf0`; `checkDuplicatedIDResult` and
`createNewCharacterResult` matched `0x0014` and `0x0015` body for body; and
`src/main/resources/ins.txt` names the inbound opcodes, ours being its names shifted by ten
in the character range.

**How to use its numbers - this is the nuance, and it cost a pass in both directions.**
The offset is *not* constant across the whole range, so a number cannot be trusted. But
`0x008A` for the create request was predicted exactly by its `CREATE_NEW_CHARACTER(141)` at
an offset of -3, and that prediction was **discarded** because the opcode was absent from
`research/msexe-send-opcodes.txt` - a scan of `FUN_1406ed520` call sites, and therefore
blind to a builder inside the VM. So: treat its numbers as **hypotheses worth testing**,
never as facts, and remember that **absence from the send-opcode table is evidence of
virtualisation, not of non-existence**.

### Standing warnings

* **`-Session mode=2,create=on` and `-Probe watch@141b2a280:rdx=0` are client patches.**
  They make the normal flow reachable; they do not make the session valid. Say so when
  reporting.
* **Rebuilding `grap-stub` does not update the client** unless `setup-client.ps1` runs.
  `test-charselect.ps1` and `test-server.ps1` do this themselves; anything else must.
* **`cargo test` does not refresh `target/release/maplecw-login.exe`.** The same trap one
  layer up: a test run leaves the release binary stale, so a manually started server can be
  a build old enough to predate the feature under test. That happened on 2026-08-18 and the
  smoke test caught it. `cargo build --release` before starting anything by hand.
* **`powershell -File` flattens array arguments** into separate words, so a `[string[]]`
  parameter silently takes only its first element and the rest bind positionally. Pass
  delimited strings.
* **Check the console line `standing answers (N)`** before reading anything into a run. A
  missing answer leaves the client on "Connecting..." and looks like a client problem.
* **Get the date from `git log`, not from a guess.** Several notes in this repo were
  written with dates two days ahead of the commits they describe, which made a single
  afternoon read as three days of separate work. Corrected 2026-08-17.
* **An instrument that has never been seen working proves nothing by staying silent** -
  and a *discriminator* has to be shown to discriminate, not just to run. The thread-drain
  test for the exit ran perfectly and reported confidently, and was still wrong: both
  controls looked identical under it.

## Working right now

```bash
cargo test --release          # 107 tests green
cargo build --release
```

**The client runs and connects to our server:**

```bash
# 1. once: create the account the server serves
./target/release/maplecw-useradd.exe maplecw
# 2. start the login server
./target/release/maplecw-login.exe
# 3. launch the patched client (from client-patched/)
MapleStory.exe -NXLDEBUG 127.0.0.1 8484
```

`tools/test-server.ps1` does all three. The client connects to `127.0.0.1:8484`, and
GameGuard never loads.

## Done

| | |
|---|---|
| `crates/wz` | WZ parser. **9,994/9,994 images** across 102 archives parse. `wz-dump` CLI. |
| `crates/net` | **The client's real wire cipher**, verified against captures, plus the recovered inbound opcodes and their bodies, and `packet-hex` to put a body on a command line without typing it. 28 tests. |
| `crates/store` | SQLite accounts/sessions **and characters**. argon2id, per-password salt, hashed single-use tokens; `characters` + `equipment` tables cascading from `accounts`. 33 tests. |
| `crates/auth` | Local HTTP auth server (loopback only) + `maplecw-useradd`. Verified end to end. |
| `crates/login` | **The login server.** Greeting, the protocol as a pure state machine, and the socket loop. Characters persist. 21 tests, plus an end-to-end check against a stand-in client. |
| `crates/grap-stub` | No-op `grap64.dll`; GameGuard never starts. Plus the **in-process dispatcher hook**, the opcode walk / watch probe, a session monitor and patcher, and a socket watch over `connect`/`closesocket`/`shutdown`. |
| Client copy | `client-patched/` — original install untouched, firewalled outbound. |
| Tooling | `handshake_probe.py` decodes the client's live stream; `dump_runtime.py` reads its memory. |
| Canvas render | `wz-dump canvas` + `tools/wz_png.py` turn WZ canvases into PNGs. **This is how the client's baked UI text gets read** - much of its on-screen wording is pixels, invisible to any string search. Formats 1, 2 and 513. |

## Key facts (do not re-derive)

- **WZ data version 779**, hash `0x0000E73A`, **zero** string key.
- **Network protocol version is 100** — unrelated to 779. Don't conflate them again.
- Launch: **`-NXLDEBUG <ip> <port>`** is the only mode that runs *and* connects, and it is
  **not** a debug mode - it sets the same launch mode (5) as `-NXL`, which is what the real
  Nexon Launcher passes. `IPPORT` does not crash in the parser: it whitelists the IP against
  six Nexon literals and bails without setting a mode. `WEBSTART` needs token 1 non-empty.
  Settled statically 2026-08-19; see `docs/launch-protocol.md`.
- Handshake framing: **`u16` little-endian body length, then the body** (length excludes
  itself; the client rewinds over the prefix). Confirmed working.
- `MapleStory.exe` is **Themida**-protected with a rebuilt IAT — do not patch it on disk.
  Find code via **string xrefs**, never import xrefs. That technique has worked four
  times now.
- The client cannot be killed with `Stop-Process`; use `taskkill /F`.

## Transport: SOLVED IN BOTH DIRECTIONS

**The handshake is solved.** `FUN_1415d10e0` line 606 rejects the connection unless fields
`G == 1` **and** `H == 1`, raising the *same* `0x22000007` "client is outdated" error as a
version mismatch, unconditionally — which is why every early version sweep looked
identical. Full table in `docs/handshake.md`.

**The packet transport is solved**, and the client both accepts our frames and has its own
stream fully decoded. See `docs/transport.md`.

```
len     = a ^ b                     # two u16 LE; no byte-swap, unlike classic MapleStory
a       = ((iv >> 16) & 0xFFFF) ^ K # K = 0xFFFE for packets we send, 0x00DF for the client's
payload = AES-256-OFB(key, iv repeated 4x)   # chunks 0x5B0 then 0x5B4
iv       -> stock shuffle table at 0x143A86890, rolled once per packet
```

Our chain seeds from `K`, the **second** u32 of the greeting (`conn+0xec`); the client
transmits on `J`, the first (`conn+0xe8`). Lengths `>= 0xFF00` use an 8-byte header.

### The AES key is a decoy on disk — do not "fix" it

The table at `0x143A86810` holds the **stock** MapleStory key in the file, and the client
overwrites the low byte of all 32 dwords at startup. Only that table — the shuffle table
beside it is untouched, which is exactly why framing, the header constant and the IV chain
were provably correct while everything AES-shaped failed in *both* directions at once.

```
0f 00 00 00  1b 00 00 00  c5 00 00 00  46 00 00 00
f3 00 00 00  be 00 00 00  ff 00 00 00  75 00 00 00
```

Read with `tools/dump_runtime.py` (read-only, needs an elevated shell), stable across
sessions, so it is a build constant. `the_disk_key_is_a_decoy_and_does_not_decrypt` guards
against reverting it.

With it, every captured packet matches its decompiled builder field for field — packet 1 is
`70 00 02 64 00 00 00`, exactly `FUN_1415d5b40`'s `u8 2, u32 100`.

### What the client sends, and what it waits for

Login connection startup, read from the handshake tail (`conn+0x48 != 0` selects it):
**`0x70` version, `0x71` environment, `0x8F`/`0x90`/`0x91` log uploads, optional `0xA1`** —
then the handler *returns*. The hang is in the main loop, waiting on the socket. The eleven
6-byte packets are `0x00A6` carrying an incrementing id.

`0x8F`-`0x91` read a file up to 8 KB, upload it and delete it; they need no reply, and they
are why opening bursts varied 294 to 3393 bytes between runs.

### All earlier sweep results are void

Every sweep predates the key fix, so the client never saw an opcode we intended, and the
scattered exits at `0x0023`, `~0x01DC`, `~0x01F1`, `~0x03C5` were random garbage opcodes
hitting a disconnect handler — none reproduced, and `0x0023` sent alone did nothing.

Note the trap that hid this: **acceptance only proves the header**. A bad payload decrypts
to a random opcode and is silently ignored, not rejected.

### The startup gate is solved - the client reaches its login screen

**Inbound opcode `0x0032`, body `0x00`.** Seven bytes on the wire, and the client goes from
a blank non-responding window to the login screen. Verified with a single packet and no
probe: `flag=0->1 state=0->2`.

It was never a login handshake. The client hashes `Data.wz` into `conn+0x14c`, sends
`0x00A1` carrying that `u32`, and blocks in `recv` **on its UI thread** inside
`FUN_1415e7090`, looping recv -> decrypt -> dispatch until a handler sets the byte at
`conn+0x150`. Only `FUN_1415e5c20` does that, and it is a `Data.wz` patch handler whose
first field is a **zigzag varint** length (`FUN_1406efcc0`):

| length | client does |
|---|---|
| `0` | nothing to patch - sets the flag and carries on |
| `> 0` | expects that many bytes in 64 KB chunks, then writes `Data.wz` |
| `< 0` | deletes `Data.wz` and carries on |

This client ships no `Data.wz` at all (a `Data/` directory instead), so it sends hash `0`
and a varint `0` is the right answer. See `crates/net/src/opcode.rs`.

That also explains the old "26 packet ceiling": every unhandled packet allocates a `0x5b4`
buffer inside that loop and the loop never exits to free them. A leak, not a limit.

### The login exchange

**Superseded in part:** "the client logs in by itself" is true only in **mode 5**
(`-NXLDEBUG`), where a per-frame tick calls the same function the Login button calls. With
`-Session mode=2` the client waits for the button, which is the real flow. See "THE GOAL"
at the top.

After the gate the client sends:

```
0x00C0  05 00 00 00 20 4e 00 00
0x0073  26B  05 00 00 00 00 00 aa bb cc dd ee ff de ad be ef...   <- 20 bytes, constant
0x0080  (empty body)                                              <- the login request
0x007A  01 01 4x 00 00 00 ...
```

then waits **4-7 seconds** and abandons the connection. `0x0073` and `0x0080` are both
built by `FUN_141b21ea0`, the function that loads `UI/Login.img`.

**The reply is inbound `0x0010`**, and this is the structural find of the session: the
login stage's `OnPacket` is `FUN_141b25f30`, and it is an **ordinary readable switch on the
opcode**. The Themida-virtualised dispatcher hands a stage its opcode; the stage dispatches
in plain code. So the whole login-stage opcode map is readable:

```
0x00, 0x0b-0x18, 0x23, 0x25-0x27, 0x29, 0x2b, 0x34-0x39, 0x45-0x48, 0x4a, 0x50, 0x5f, 0x5f4
case 0x10 -> FUN_141b307b0    the login result
```

Watch mode confirmed at runtime that `FUN_141b307b0` **is entered while dispatching
`0x0010`**, so the opcode and the stage are both right.

Body of `0x0010`, from `FUN_141b307b0` and `FUN_1406e9050` (strings are `u16` length then
bytes):

```
u8  result
str message
if result == 0:    u8, 8 bytes, u32, u32, 4B, 4B, 4B, u32, u8,
                   then FUN_14108d290 and FUN_14108bdf0 read further
if result == 0x83: two more u32
```

**Result `0` is success.** `FUN_141b267c0(this, result, 0, ...)` raises the error dialog,
and the proceed branch is `cVar6 != 0 && result == 0`. `0x65`/`0x67` are *not* success -
they take a different branch that re-sends `0x0080`. Misreading them as success cost three
runs of the same dialog.

Corroborated independently: non-zero results are **error message IDs**, resolved through
`FUN_141803cd0` in `docs/client-messages.md`. `0x65` is 101, *"You have been disconnected
from the login server"* - exactly the dialog that replying `0x65` produced.

### DONE - the login result is accepted, the client reaches character select

`0x0010` with `body = 00 00 00` + 256 zero bytes ran the success path to completion and the
client's UI **advanced to character select**. Compare `0x65`, which dropped the connection
in 0.0 s with no follow-up. Fixture:
`research/fixtures/reply-0010-result0-advanced-to-charselect.log`.

Full field list in `docs/opcodes.md`. Two fields matter beyond filler: the `u32` world id
and `u32` channel id, which the client looks up in a world list it does not yet have.

### The whole login stage has two variants, and we are in mode 5

**Check this before decoding any login-stage handler.** Several open with

```c
if (FUN_142c4a810(DAT_143ac1898) == 5) { <other handler>(...); return; }
```

`session+0x68` is **5** in our client - transmitted as the first `u32` of `0x0073`, captured
as `05 00 00 00`. Mode 5 is what **`-NXL`, `-NXLDEBUG` and `-NXLPTS` all** set, so it is the
production Nexon-Launcher mode rather than a debug one, and it is how we launch. So the
mode-5 branch is always the live one and the handler the switch names first is dead code
for us. `0x000B`, the login flow, and the Login button all fork this way. Decoding the
wrong side costs a full analysis pass. Table in `docs/opcodes.md`.

### The Login button is enabled by one byte, and `0x000B` sets it

The owner: the button starts **disabled** in an invalid session. `FUN_14112a720`, the
`ClassicIntro` tick, enables the control named `"login"` only when
`FUN_141b2a160(stage)` - that is, `*(u8 *)(stage + 0x108)` - is non-zero. The screen
builder `FUN_141129930` creates it disabled.

`stage+0x108` is written by the **world-list handler**, one line above the list append:

```c
*(undefined1 *)(param_1 + 0x108) = 1;
piVar10 = (int *)FUN_141b44520(param_1 + 0x100, 0xffffffff);
```

So **inbound `0x000B` enables the button**, populates the list the login result searches,
and is the one thing missing since the client first reached the login screen. Only a real
world entry does it - the terminator branch returns before both writes.

The account field is a different object: `FUN_142cb83a0` is `DAT_143aa84a0 + 0x22f8`,
rendered into `textAccount` when non-empty. `DAT_143aa84a0` also holds world id `+0x2258`
and channel id `+0x2260`; it is **not** the `DAT_143ac1898` that carries the `0x0073`
identity.

### MILESTONE - login screen -> Login button -> character select

**Reached 2026-08-17.** The owner clicked a lit Login button, the client played its animated
transition into character select, and "Create a character" was the blocker - the goal set
at the start of the day.

The recipe, all four parts needed together:

```
powershell -ExecutionPolicy Bypass -File "<repo>\tools\test-one.ps1" -Reply ping
  -Opcode 0x0032 -Body 00 -PingFirst 0x0032 -PingBody 00 -QuietBefore 4 -HookLog on
  -Session mode=2 -Probe watch@141b2a280:rdx=0 -ReplyTo 0x0080
  -ReplySeq "000b:0006005363616e6961000000000108005363616e69612d30000000000000000000000000000000"
```

| Part | Why it is needed |
|---|---|
| `0x0032` gate | releases the startup loop; login screen appears |
| `0x000B` world **entry** | sets `stage+0x108`, which is what enables the Login button |
| `-Session mode=2` | leaves mode 5 so the per-frame tick stops auto-logging-in and the button gets a turn |
| `-Probe watch@141b2a280:rdx=0` | suppresses the "trouble logging in" dialog, which otherwise **blocks the tick** and stops the button ever being enabled |

**No terminator.** With the mode patched, a second `0x000B` is handled by the classic
`FUN_141b2fac0`, whose terminator transitions to WorldSelect - a screen this service does
not use.

**Two of those four are client-side patches.** They make the client's normal flow
reachable; they do **not** make the session valid. Describe results accordingly.

**Everything after the login screen was offline.** The client closed the connection at
8.4s - immediately after sending `0x007A`, its loading-complete report - so the Login click,
the transition and the "Create a character" clicks all happened with no server attached,
and **sent nothing**. Those transitions are purely client-side.

**Adding the login result back did not keep the connection alive.** It *was* dispatched and
handled (`2 opcode=0x0010 ... ret=1`), and the client closed 0.42s later, exactly as it does
without one. So the close is not a rejection of our reply.

### RETRACTED: "the client does not migrate" was never established

This section used to read "SETTLED: the client does not migrate", on the strength of a
`connect` hook that logged **nothing**. That was the same silent-negative mistake the repo
warns about everywhere else, and it took two runs to notice.

**The hook has never logged a `CONNECT` line at all — including for the connection to
`127.0.0.1:8484`, which certainly happened.** So "no connect was logged" says nothing
about the client's behaviour until the hook is shown to work. It may be that the client
reaches its socket through a path `ws2_32!connect`/`WSAConnect` do not cover; it may be
that the hook is simply broken. Either way the migration question is **open**, and so is
everything that was inferred from it.

`netwatch` now runs a **self-test** at install: it makes its own loopback `connect` and
`closesocket` and reports whether its handler caught them.

```
netwatch: SELF-TEST ok - 2 of our own calls were caught, so a later absence of lines is a
real negative
```

Read that line before reading anything else from this hook. `SELF-TEST FAILED` means every
negative it reports is worthless.

**What is still true:** `tools/watch-sockets.ps1` saw only one socket, and no second
endpoint was ever observed. That is weak evidence for one connection, not proof.

**What was inferred from the retracted claim, and is now unsupported:**

* that no channel server is needed;
* that the unread fields in the login result cannot be a server address.
* **The close is *not* explained.** It was recorded here as an ~8s idle timeout; that was
  inferred from timing alone and the timing has a second explanation - see "CORRECTION"
  above. What is settled is only that no reconnect follows it.

### CLOSED - "the client has no character list"

This section used to say the missing character-list packet was the next thing to find. It
was found and it is done. The list is inside `0x0010` (`FUN_14108d290` then
`FUN_14108bdf0`), it has been sent, and the client draws it. `FUN_141b28570`, nominated
here on the strength of where it is called from and never actually read, turned out to be a
290-byte state check with nothing to do with character lists.

The inert screen had a second cause that outlived the list: `FUN_141b282d0` gates the
"Create a character" button on three `0x0010` tail fields **and** on an obfuscated flag the
handshake sets to zero. See `docs/character.md`.

### SOLVED - `FUN_141b2a280` raises the prompt

`called-from=0x141b2a61e` at a watch on the notice display named it.
`FUN_141b2a280(stage, code, flag)` shows `loginTroubleAskSupport` for **codes -1, 6, 8, 9
and 12** (`0x2681` bit-tested at `code + 1`); `code == 0` is success. Full code -> notice
table in `docs/session.md`, and the whole baked dialog table in `docs/client-notices.md`.

It is a near-duplicate of `FUN_141b267c0` - same mapping, different function.

**Why it hid for three sessions, and the lesson:** it never takes the string's address. It
**copies the literal inline** with RIP-relative `mov`. `tools/xref.py` matches `lea`, so it
reported three references, all of which were then proven never entered - and the real raiser
was invisible to every scan built on it. A "0 references" result means *nothing takes its
address*, not *nothing uses it*. That warning is now at the top of `xref.py`.

Independently corroborated: an exhaustive render of all 170 `/Notice/` canvases found no
duplicate node and no numeric twin, so the dialog on screen is definitely this one.

### The code is 12, and the caller is virtualised

```
WATCH #1: 0x141b2a280 ENTERED  rdx=0xc (as i32 12)  r8=0x1  called-from=0x144c05eb2
```

* **`code = 12`** - in the trouble set, and a *generic* failure: no specific notice maps to
  it, unlike 4 (`incorrectPassword`) or 5 (`notRegisteredID`). The client is not reporting a
  named reason, it is reporting "login did not succeed".
* **`r8 = 1`** - the flag argument, which sets `stage+0xf0 = 1`.
* **`called-from = 0x144c05eb2` is inside `.themida`.** The immediate caller is virtualised
  and cannot be decompiled.

No packet carried this. Only `0x0032` was ever dispatched, so the client generated code 12
on its own.

**The stack walk is a dead end, and that is settled.** A 0x400-byte, 16-slot scan of the
stack at the call found exactly one image address: the VM return address itself.

```
stack: 0x144c05eb2(vm)
```

No `.text` frames at all. Themida runs the VM on **its own stack**, so the caller chain is
not there to find and widening the scan only reads more VM stack. **Who decided code 12
cannot be answered by walking back from the call.**

Two ways forward, and they answer different questions.

**1. What does the client do if it believes login succeeded?** `FUN_141b2a280` returns
success for code `0`, so rewriting the code at its entry answers that in one run.

```
powershell -ExecutionPolicy Bypass -File "<repo>\tools\test-one.ps1" -Reply ping
  -Opcode 0x0032 -Body 00 -PingFirst 0x0032 -PingBody 00 -QuietBefore 4
  -HookLog on -Probe watch@141b2a280:rdx=0 -Session watch
```

This is a **client-side patch** - it makes the client stop concluding it failed; it does not
make the session valid. Say so when reporting results. What it buys is the rest of the
flow: whether the login screen becomes usable, and where the client gets stuck next.

**2. Why the client concludes failure.** The decision is virtualised, but what it *consults*
need not be. The `CNM*` session interface lives in `nexon_api_x64.dll` / `nmcogame64.dll`,
both **unpacked** - readable statically and hookable at their exports. That is the honest
route to a genuinely valid session, and it needs no client runs to start.

### How it was found

A watch on the notice display `FUN_141b4ac80(name, ...)` caught it:

```
WATCH #1: 0x141b4ac80 ENTERED  rcx=0x14d148 [0x057f5ed8] "loginTroubleAskSupport"
          rdx=0x14d100  r8=0x0  r9=0xe
```

Settled by that line: the dialog **is** `loginTroubleAskSupport` (not some similar node),
it is raised through `FUN_141b4ac80`, and the name arrives **intact** - so it is not built
at runtime.

Which leaves a genuine puzzle. Something loaded that name, but:

* the `.rdata` literal at `0x1433d5d98` has exactly **three** code references, and all three
  are proven never entered;
* there is **no pointer-table reference** either - a scan for the qword `0x1433d5d98`
  anywhere in the file finds nothing;
* `rcx` pointed at a **heap** copy (`0x057f5ed8`), not the literal.

The leading explanation is that the caller is **virtualised**: a `lea` inside Themida VM
bytecode is invisible to every static scan we have. If so, static analysis is finished here
and everything further must be measured.

**Next:** watch mode now logs `called-from`, read from `[rsp]` at the breakpoint - the
breakpoint sits on the function's first byte, so the `call` has just pushed the return
address. Re-run the same command; the caller names itself.

```
powershell -ExecutionPolicy Bypass -File "<repo>\tools\test-one.ps1" -Reply ping
  -Opcode 0x0032 -Body 00 -PingFirst 0x0032 -PingBody 00 -QuietBefore 4
  -HookLog on -Probe watch@141b4ac80 -Session watch
```

A `called-from` inside `.themida` (roughly `0x144C0000`+) confirms the virtualised-caller
theory. Anything in `.text` names a real function to decompile.

#### Older note, now superseded

**One question, and it has a designed experiment:** what result code reaches
`FUN_141b267c0`, and when? The dialog is raised for result -1, 6, 8 or 9, but the owner sees it
*before* any login exchange, and `0x0032` (handled by `FUN_1415e5c20`) never touches that
path. Two of its callers - `FUN_141b2b120`, a 31-byte wrapper that passes the code straight
through, and `FUN_141b2ae80` - have no callers and are in no vtable, so they are reached
only through the virtualised dispatcher and **cannot be traced statically**.

So observe it. **`-Probe watch@<VA>` now does this**: it reports *every* entry with the
dispatching opcode and the first four integer arguments (`rcx`, `rdx`, `r8`, `r9`), rather
than announcing one hit and disarming. `rdx` is the result code, and the switch above turns
that number into the dialog on screen.

```
powershell -ExecutionPolicy Bypass -File "<repo>\tools\test-one.ps1" -Reply ping
  -Opcode 0x0010 -PingFirst 0x0032 -PingBody 00 -ReplyTo 0x0080 -QuietBefore 4
  -HookLog on -Probe watch@141b267c0
```

Send only the gate, so nothing we send can be the cause: if the dialog still appears, the
failing code came from the client itself. Read `rdx as i32` out of the `***** WATCH #n`
lines in the hook log.

One known limit: it stops logging after 32 hits so a per-frame caller cannot fill the disk.
(The old "can only arm once the hook has seen a dispatch" limit is **gone** - watch now
arms from `install()`, which also removes the race that twice brought the dialog back.)

**The auto-advance is not a bug.** In mode 5 the `ClassicIntro` tick calls
`FUN_141b3ff10` - *the same function the Login button calls* - as soon as `0x000B` sets
`stage+0x108`. Mode 5 is the **Nexon-Launcher** mode, not a debug one (`-NXL` sets it too):
a launcher-started client already holds a session, so it logs in without the button. That is
why the flow does not match a normal server.

**To get the click-the-button flow**, write anything but `5` to `[0x143ac1898] + 0x68`
(`session+0x68`) from `grap-stub` once the world list has landed. Then the tick's
auto-login goes false, the button still enables (that happens as a side effect of the
`+0x108` check, independent of mode), and clicking Login takes
`FUN_141b3f050(stage, 4, 600)`.

> **RETRACTED 2026-08-19 (second pass).** A "correction" here previously said screen 4 was
> **world select** and that the original "straight to CharSelect" reading was wrong. **The
> correction was the error.** `docs/session.md` carries the stage table read out of
> `FUN_141127730`, which registers each id against a screen *name* - data, not inference:
> **1 Title, 2 WorldSelect, 3 ClassicIntro, 4 CharSelect, 5 NewChar.** So screen 4 is
> CharSelect and the original claim was right.
>
> What actually lands on world select is the **world-list terminator**: its branch in
> `FUN_141b2fac0` calls `FUN_141b3f050(param_1, 2, 400)` - screen **2**, WorldSelect. That is
> why sending the terminator put the client there, and it is the lever for
> "Choose another world". Switching modes sends `0x000B` to the
classic handler `FUN_141b2fac0` instead of `FUN_141b31ff0`, which is safe: their read
sequences were compared field by field and are identical. **Be honest about what this is** -
it makes the client follow the normal flow, it does not make the session valid.

### DONE - the world list

The login result makes the client search for its world in the list at `stage+0x100`
(`FUN_141b2c7c0`), and only inbound **`0x000B`** appends to it. Format decoded from
`FUN_141b31ff0` (the mode-5 handler) and built by
`crates::net::opcode::{world_list_entry, world_list_end}`, with a test that re-reads the
bytes the way the client does.

Sent on every run since, and it works. **Use `tools/test-charselect.ps1`** rather than the
hand-written `-ReplySeq` that used to be here: it generates every body from `packet-hex`,
so the command cannot drift from the builders. That drift is not hypothetical - the
world-list hex on a command line was once two characters too long, and the only reason it
was caught is that a test happened to compare against the builder.

**Send the world entry only, never the terminator plus the mode patch.** With the mode
patched, a second `0x000B` reaches the classic `FUN_141b2fac0`, whose terminator branch
transitions to WorldSelect - a screen this service does not use. That cost a run.

### The session identity - see `docs/session.md`

Short version, because two long-standing assumptions turned out to be wrong:

- **"Having trouble logging in" is `/Notice/text/loginTroubleAskSupport`** - a baked bitmap
  in `Login.img`, which is why no string search ever found it. **Solved:** raised by
  `FUN_141b2a280(stage, code, flag)` for codes -1, 6, 8, 9 and 12; measured live as **code
  12**, a generic failure, from a **virtualised** caller. Three other candidates were ruled
  out by measurement first. Full table and the tooling lesson in `docs/session.md`.
  `Login.img` also has **two** login screens (`Title_new`, and `ClassicIntro` = ours, the
  one carrying `find_id`/`find_pw`); `FUN_141129930` builds `ClassicIntro`.
- **The empty identity did not block the login.** The client still sent `0x0073` and
  `0x0080` and accepted a `result = 0` reply. It is a real gap but not the current blocker.
- **The account name is server-supplied** (`0x0000` / `0x0012`), so the session may be too.
  That demotes the launcher-handoff theory this section was built around - see "THE GOAL".

The identity is one `char *` at **`DAT_143ac1898 + 0x1b8`**, read by `FUN_142c50400` and
sent as the second field of `0x0073`, where we captured a **zero-length string**. Nothing
computes it. The six `+0x90` launcher tokens are ruled out. Next: find its writer in
`nexon_api_x64.dll` / `nmcogame64.dll` (both unpacked), or write the field directly from
`grap-stub`, which is already in-process.

The old "constant 20-byte tail of `0x0073`" question is closed: it is a 16-byte GUID plus a
4-byte counter, not session data.

### The opcode walk, and how to aim it

`crates/grap-stub/src/probe.rs` walks the inbound opcode space **inside** the client:
snapshot one captured packet, rewrite its opcode, re-dispatch, watch an oracle. The whole
enum in one launch instead of ~2 opcodes per launch over the wire. Faults are caught by a
vectored handler and the loop resumes; `ExitProcess`, `TerminateProcess`,
`RtlExitUserProcess` and `NtTerminateProcess` are detoured so a handler cannot end the run;
progress is appended to a resume file so a fatal opcode costs one launch, not the search.

`-Probe <from>-<to>[@targetVA][#N]`, or `-Probe watch@<VA>` to observe whether a function
runs at all.

**Aim it with `tools/handler_root.py`, never by hand.** A walk target must be a dispatcher
entry: no direct callers **and in no vtable**. `FUN_141b25f30` has no callers but *is* a
vtable entry, and aiming at it burned a full 3968-opcode run that missed cleanly.

**And time it.** `#N` starts the walk on the Nth dispatched packet. The walk runs inside
whatever loop the client is in, so walking for a login-stage handler before the login
screen exists cannot work no matter what address is used.

### Traps that cost time - do not re-learn these

**Protocol**

* The **on-disk AES key is a decoy**; read the real one from a running client.
* **Accepting a packet only proves the header.** A bad payload decrypts to a random opcode
  and is silently ignored, not rejected.
* Login result **`0` is success**; `0x65`/`0x67` are a different branch entirely.
* **Check the `session+0x68 == 5` fork before decoding any login-stage handler.** The
  handler the switch names is often a shim that hands off to the mode-5 one, and we are
  always mode 5.
* **The "trouble logging in" prompt is a bitmap, but it *is* a state readout.** It appears
  before any packet exchange, so no reply can clear it and it does not measure the wire.
  For wire questions use the identity string in `0x0073` and how long the connection
  survives a reply; for the prompt, look at which login screen was built.
* The client's opening burst varies **294 to 3393 bytes** because `0x8F`-`0x91` upload and
  delete log files.

**The walk**

* **Snapshot the packet before the dispatch, never after.** The dispatcher consumes the
  opcode and moves the cursor 4 -> 6, so an after-snapshot replays "opcode 0" every time:
  4096 dispatches, no faults, reported as an empty range.
* **One oracle per walk.** With a target armed, `conn+0x150` is *expected* to be set
  already, so consulting it as well reports a hit on the first opcode tested.
* **Aim at dispatcher entries** - no callers *and* no vtable. Use `handler_root.py`.
* **Time the walk** into the phase where the handler exists (`#N`).
* **Append resume records.** `fs::write` truncates first, so dying mid-write leaves an
  empty file and the next launch restarts from zero and dies in the same place.
* The probe detours `ExitProcess`, so the client survives and, being elevated, **cannot be
  killed from a normal shell** - use `taskkill /F /IM MapleStory.exe /T` from an elevated
  one, or the leftover holds port 8484 and the DLL file.

**The harness**

* **Answer on packet arrival, not on a timer.** The client sends `0x0080` at +4.4s, +6.1s
  or +7.2s and gives up seconds later; a fixed delay once fired 0.12s *before* the request
  it was meant to answer. Use `-ReplyTo`.
* `-QuietBefore` also gates timed replies, and the client is rarely quiet for that long.
* PowerShell variable names are **case-insensitive**: a `$probe` local silently ate the
  `-Probe` parameter.
* A parameter that never arrives looks exactly like one that arrives and does nothing -
  `test-one.ps1` echoes the real command line for that reason.
* Scripts are invoked as `powershell -ExecutionPolicy Bypass -File "<abs path>"`.
* **Rebuilding `grap-stub` does not update the client.** `cargo build` writes
  `target/release/grap64.dll`, but the client loads `client-patched/grap64.dll`, and only
  `tools/setup-client.ps1` copies one to the other. Skip it and the run silently uses the
  old hook - the most expensive kind of failure here, because it looks like the new code
  did nothing. Compare hashes if in doubt.

### Testing loop that works

The client's dialog is the oracle; it is not visible to the agent. Run **one variant at
a time** and have the owner report what they see. That loop found the framing, disproved the
version-check theory, and confirmed the `L` gate.

`tools\test-one.ps1` runs one variant: it starts the probe, launches the client at
BelowNormal priority pinned off core 0 (the client otherwise saturates the host while a
test sits waiting for a dialog to be read), and tears both down with `-Stop`.

The probe now **holds the connection open** (`--hold`, default 300s). Closing it early
makes the client's `recv` return 0, which sends it down its own disconnect path
(`FUN_1415d10e0` recurses with `param_2 = 0` → `0x22000001`) and looks exactly like a
rejected handshake.

## Housekeeping

**Run scripts with Windows PowerShell, from an elevated shell:**

```
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1"
```

`pwsh` is **not installed** on this machine - PowerShell 7 was never set up and the shell is
5.1 - so anything written as `pwsh tools/...` errors with "not recognized". This file said
exactly that in two places, and `firewall.ps1` and `setup-client.ps1` in three more, until
2026-08-19.

Elevation matters separately: `exit-forensics.ps1` runs from `test-server.ps1` and cannot
see SYSTEM-owned handles without it. It reports how many it could not reach, so a short
list is never misread as an empty one.

When editing these scripts, remember what 5.1 does not have: `&&`, `||`, ternary,
null-coalescing.

- Firewall rule `MapleCW - block patched client outbound` is **active**. Remove with
  `powershell -ExecutionPolicy Bypass -File tools/firewall.ps1 -Remove` (needs elevation).
- `client-patched/` has the GameGuard stub installed; `powershell -ExecutionPolicy Bypass -File tools/setup-client.ps1
  -Restore` puts the real DLL back.
- Ghidra projects in `research/ghidra/` (~1.2 GB, gitignored). `msexe`, `grap64`,
  `mssecure`, `nexoncm` are all analysed — reuse them rather than re-importing.
- **Ghidra: `docs/ghidra.md` is the full workflow** - read that, not this bullet. The two
  things that bite first are the JDK, below, and that **the project locks**: never let a
  subagent run it while you are.
- **Ghidra needs JDK 21, not 25.** Under JDK 25 the bundled Felix 7.0.5 aborts with
  `Bundle org.apache.felix.framework [0] The data file must be inside the data dir`.
  Prefix headless runs with:

  ```powershell
  $env:JAVA_HOME="C:\Program Files\Eclipse Adoptium\jdk-21.0.6.7-hotspot"
  $env:PATH="$env:JAVA_HOME\bin;$env:PATH"
  ```

  If it was already run under 25, also delete
  `%APPDATA%\ghidra\ghidra_12.1.2_PUBLIC\osgi\felixcache` (a regenerable script cache).
