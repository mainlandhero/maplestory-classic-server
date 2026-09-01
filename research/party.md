# Parties: the opcodes, the UI, and the one predicate the server needs

Written 2026-09-01. **No Ghidra** (sibling agents hold the project lock), **no client run**,
own build directory (`target/agent-party`). Everything below was read out of
`client-patched\MapleStory.exe` with `tools/reads.py`, `tools/listing.py`, `tools/callers.py`,
`tools/encodes.py`, `tools/dump_stringids.py` and `target/release/wz-dump.exe`, out of the
pre-made dumps in `research/`, out of `crates/`, and out of the 452 archived runs in
`previous-runs/` and `research/fixtures/`.

Tags on every claim: **[L]** read off a listing, a table, the WZ, the string table, a capture
or the source; **[D]** derived from two or more of those; **[I]** inferred - a candidate
nothing on this machine can confirm.

The owner, 2026-09-01: *"All members of a party should see all drops killed by members of the
party."* and *"There are already party UI elements in the game."*

---

## 0. The answers, up front

1. **The party UI is real and the owner has already used it.** `UI_000.wz/UserList.img/Party`
   carries `button:create` / `invite` / `expel` / `leave` / `pickup` / `leader` / `whisper` /
   `talk`, a scrollable member list with leader/name/job/level columns, and a "show HP"
   checkbox; `PartyHP.img` carries the gauge. **[L]** And on **2026-08-28** they clicked Create:
   `research/fixtures/portal-arrival-fixed-chat-and-party-world.log` line 267 is a real
   68-byte `0x0182`, at `16:48:31.299` **by the log's own clock**. (That clock runs some hours
   ahead of the launcher's filename clock in every archived run - `world-20260828-105228.log`
   carries `14:48:52` - so the date is the file's and the time is the log's, and they are not
   the same instrument.) **[L]**
2. **The server→client opcode is `0x00A5`**, and it is one packet with a 45-arm switch on a
   leading `u8`. Not from a v214 list - from this client's own switch table, its own string
   table, and `tools/callers.py`, three independent legs. §2.
3. **`0x0182` survived checking, and its name did not.** It *is* the party request opcode, but
   *create* is **one of seven actions** on it, and the body is a **FlatBuffers table**, not a
   field list. `crates/net/src/names.rs`'s `CLIENT_PARTY_CREATE` describes the one capture
   that produced it and generalises it into a name. §3.
4. **Party drop visibility needs nothing from the client.** `crates/net/src/drops.rs` already
   establishes that `ownType` is stored at `drop+0x70` and **never gated on** - *"ownership is
   entirely the server's job"*. **[L]** So the whole feature is "who does the server send
   `0x046E` to", and that is `crates/world/src/party.rs`'s `Parties::audience`. §7.

---

## 1. The instruments, and the controls they reproduced first

`CLAUDE.md`: *"Verify the instrument before believing it."* All of these run **from the repo
root**; every throwaway script was piped in (`python - < s.py`), which leaves `sys.path[0]`
empty so the scratchpad's stale `reads.py` / `listing.py` / `callers.py` could not shadow
`tools/`.

| instrument | control | result |
|---|---|---|
| `python tools/reads.py 0x140304100 2` | the documented control in its own docstring | raw @`140304138`, u8 @`140304144`, u8 @`140304183`, then a run of u16 - **reproduced** |
| `python tools/dump_stringids.py --grep "Cash Shop"` | a feature confirmed on the owner's screen must have strings | **19 hits** out of 6 165 decrypted strings - reproduced |
| `wz-dump cat UI_000.wz UserList.img` | the same read that `research/guilds.md` used to find Buddy/Party/Blacklist and **no** Guild | same three panels, same absence - reproduced |
| the string-id enumeration (§2) | it must find thousands of ids, not a handful | 6 599 resolver call sites, **3 956 distinct ids of the 6 165 that exist** |
| the archive sweep | `<- 0x02FF` = 164 117 distinct events | reproduced over **452** files |

The archive was deduplicated **on events, not files**: the key is `(timestamp, direction,
opcode, first 120 bytes)`. `CLAUDE.md` § *"deduplicate the EVENTS"*.

### The filter that nearly lost the whole answer

The first reachability sweep parsed `research/msexe-gamestage-cases.txt` with
`if p[1].startswith("FUN_")`. That matched **179 of 273 rows** and silently dropped every case
whose body is inlined into the dispatcher - and **`0x00a5` is one of them**:

```text
0x00a5  INLINE FUN_1413bab80
```

The sweep came back with one hit, `0x00EA`, and it looked like a clean answer. Counting the
rows the parser had *not* matched is what caught it. This is the same failure `CLAUDE.md`
describes three times over: a filter that returns a confident number from a third of the
input. Every count in this document was taken after the parser was fixed to 270 of 273.

---

## 2. `0x00A5` is the party result, established without any reference list

`research/msexe-gamestage-opcodes.md` warns that its own candidate names score badly, so none
of them is used here. Three independent legs, all **[L]**:

**Leg 1 - this client's own switch.** `research/msexe-gamestage-cases.txt`, extracted from
`FUN_142cbaa80` by `tools/switch_cases.py`, gives case `0x00a5` as an inlined body whose only
call is `FUN_1413bab80` (8 624 bytes, `0x1413bab80..0x1413bcd30`).

**Leg 2 - the string table, enumerated rather than searched.** `tools/dump_stringids.py`
decrypts 6 165 strings; **89** contain "party", and 43 of them sit in one contiguous block,
`0x0108..0x0132`, which is the shape of a result-code message table. To find who uses them I
enumerated every `mov edx, imm32` immediately preceding a `call 0x1408a9e40` (the resolver
`dump_stringids.py` names) across all **120 981** `.pdata` functions - 6 599 call sites, 3 956
distinct ids, 51 sites with a computed id. Twelve functions touch the party block:

```text
0x1411c99e0   3 ids   0x011c 0x011d 0x011e         the member-list column headers
0x1413b94e0   2       0x0118 0x0119
0x1413b9bc0   1       0x0121                        the CREATE request builder
0x1413b9d90   1       0x0126
0x1413b9eb0   3       0x0126 0x0127 0x0129          the INVITE request builder
0x1413ba250   1       0x0124                        the LEAVE request builder
0x1413ba470   2       0x0126 0x0128
0x1413ba590   2       0x0126 0x0128
0x1413ba710   1       0x0123
0x1413bab80  27       0x0108..0x012e               <-- the result handler
0x1413bcdd0   1       0x0121                        (also 0x0bac, outside the block)
0x142d1d130   4       0x011a 0x011b 0x011c 0x011d
```

`FUN_1413bab80` resolves **27 immediates directly and 31 reachable at depth 5**, six times
more than anything else in the image.

**Leg 3 - reachability from the other end.** `python tools/callers.py 0x1413bab80` reports
**1 call site, 0 tail jmps, 0 qword pointers**, and the one call site is `0x142cbac6e` -
inside `FUN_142cbaa80`. So the function that owns the party messages is reached from exactly
one place and that place is the channel dispatcher's `0x00a5` arm.

The same sweep over all 270 named cases finds exactly **two** that reach the party block:
`0x00A5` (31 ids) and `0x00EA` (6 ids). **`0x00EA` is not a second party opcode**, and this
server already knows what it is: `crates/net/src/revive.rs` decodes it as
`RUN_CONSOLE_COMMAND` - *"a string the client splits on carriage returns and runs as console
commands"* - and this server has sent one (`previous-runs/world-20260828-105228.log`,
`-> 0x00EA ... "/hitdamagetest 0"`). `session/gm.rs` records that `/party` is one of the slash
commands the client parses **itself**. So `0x00EA` reaches the party *request builders*
because a console command can press the same buttons, and the six ids it reaches are the
builders' own local pre-checks. It reads one `str` and nothing party-shaped. **[D]**

### The switch, walked

```asm
1413baf47  call 0x1406e8ae0            ; READ u8  code      <- the FIRST read in the function
1413baf58  add  eax, -3
1413baf5b  cmp  eax, 0x2c
1413baf5e  ja   0x1413bcbb5            ; default
1413baf6d  mov  ecx,[rdx + rax*4 + 0x13bcc50]
1413baf7e  jmp  rcx
```

A **45-entry table at `0x1413bcc50` covering codes `0x03..0x2F`**, resolving to **30 arms**;
15 of the 45 codes land on the default arm and 0x1B/0x1C share one. **[L]**

### The full code table

Meanings are the strings each arm resolves - as direct as evidence gets. "reads" is
`tools/reads.py 0x1413bab80 3` bucketed by arm.

| code | arm | reads | what the client shows | tag |
|---|---|---|---|---|
| `0x03` | `1413baf80` | `u32 u32 str u32 u32 u32` | nothing; **answers `0x0183`** then may open a dialog | [L] shape, [I] which of invite / join-request |
| `0x06` | `1413bb25b` | `u32 u32 str u32 u32 u32` | same, different dialog constructor | [L] / [I] |
| `0x0D` | `1413bbab4` | `u8` + a helper | - | [L] |
| `0x0E` | `1413bbb8d` | `u32 partyId, u8, u32, u32, u32, i16, i16`, **two undecoded sub-decoders**, `u32` | *"You have created a new party."* | [L] arm, tail unread |
| `0x0F` | `1413bc8ef` | none | *"Already have joined a party."* | **[L]** |
| `0x10` | `1413bbcf1` | `u32 u8 u8 str` | expelled / left / *"the party has been disbanded"* / *"since the party leader quit"* | [L] arm + 6 strings; which `u8` picks which is unread |
| `0x11` | `1413bc924` | none | *"You have yet to join a party."* | **[L]** |
| `0x12` | `1413bc9f8` | none | *"Leaving the party is restricted while on this map."* | **[L]** |
| `0x13` | `1413bc046` | `str` | *"'%s' has joined the party."* / *"You have joined the party."* | [L] |
| `0x14` | `1413bc2d8` | `u8` | *"You have joined the party."* | [L] |
| `0x15` | `1413bc959` | none | *"Already have joined a party."* | **[L]** |
| `0x16` | `1413bc98e` | none | *"...already in full capacity."* | **[L]** |
| `0x17` | `1413bcbe5` | none | nothing - the arm is inside the epilogue | **[L]** |
| `0x19` | `1413bcb83` | none | *"You are on a map where the other party cannot complete this action."* | **[L]** |
| `0x1B` `0x1C` | `1413bb51a` | `raw`, `str` | eleven invite/join outcome messages | [L] arm + strings; the `raw` length unread |
| `0x1D` | `1413bcad8` | none | *"That cannot be used on this map."* | **[L]** |
| `0x1E` | `1413bcb0d` | `str` | *"Party invites cannot be sent to '%s''s current location."* | [L] |
| `0x1F` | `1413bc9c3` | none | *"Kicking someone from the party is restricted while on this map."* | **[L]** |
| `0x22` | `1413bc54d` | `u32 u8` | *"%s has become the leader"* / *"Due to the party leader disconnecting..."* | [L] |
| `0x23` | `1413bc850` | none | *"This can only be given to a party member within the vicinity."* | **[L]** |
| `0x24` | `1413bc885` | none | *"Unable to hand over the leadership post..."* | **[L]** |
| `0x25` | `1413bc8ba` | none | *"...only change with the party member that's on the same channel."* | **[L]** |
| `0x27` | `1413bca62` | none | *"The party leader reached the entry wait time limit..."* | **[L]** |
| `0x28` `0x29` | `1413bca97` `1413bcac4` | none | nothing | **[L]** |
| `0x2A` | `1413bc327` | `u8 u32 raw` | - | [L] |
| `0x2C` | `1413bca2d` | none | *"Cannot be done in the current map."* | **[L]** |
| `0x2D` | `1413bc63e` | `str u8` via `FUN_1406f2560` | *"...changed the party status to Public / Private."* | [L] |
| `0x2F` | `1413bc434` | `u8 u32 u32 u32` | - | [L] |
| `0x04 0x05 0x07..0x0C 0x18 0x1A 0x20 0x21 0x26 0x2B 0x2E`, and **everything outside `0x03..0x2F`** | `1413bcbb5` | **none** | *"Due to an unknown error, your party request failed."* | **[L]** |

### The free safety valve, and why it is worth a constant

**Sixteen of the 29 non-default arms read nothing at all, and so does the default.** That is
not a claim about the arms I happened to look at: `tools/reads.py 0x1413bab80 3` - which walks
helpers and tail `jmp`s - reports **no read site above `0x1413bcb15`**, and `0x1413bcb15` is
inside the `0x1E` arm. So every arm from `0x1413bc850` upward is read-free **except `0x1E`**,
whose `str` is the highest read in the whole 8 624-byte function. (The first draft of this
sentence said "every arm from `0x1413bc850` upward" with no exception, which is the arm right
in the middle of the run - written down because it is exactly the over-generalisation
`CLAUDE.md` keeps warning about, and it was caught by listing the arms rather than trusting
the sentence.)

So `0x00A5` with a **one-byte body** is safe for any of those codes, and the default arm makes
it safe for *any* byte outside the table. `net::party::refusal` builds it and refuses to build
anything else; `net::party::request_failed` is the always-answer valve.

### `0x00A5` has never been on the wire

Zero `0x00A5` in either direction across 452 archived logs, with the 164 117-event positive
control passing. Nothing in this section has been seen to work.

---

## 3. `0x0182` survived checking. Its name did not.

`crates/net/src/names.rs` line 125:

```rust
0x0182 => "CLIENT_PARTY_CREATE",
```

Its neighbours all carry an evidence note; this one carries none. `git log -S "0x0182"` puts
it in **`1ba87b7` "Make NPCs speak: answer 0x0151 with a 0x055B Say"** - a drive-by in a commit
about NPC dialogue - and `STATUS.md` line 3839 records where it came from: the capture above,
*"68 bytes carrying the length-prefixed string `"TestCharD's Party"`"*.

**So the row is not invented, and it is not a decode either.** It is one observation of one
button, turned into the name of an opcode. Two things are wrong with the result:

### It is one action of seven

Five builders construct `0x0182` (`research/msexe-send-opcodes.txt`, confirmed with
`tools/listing.py`), each writing an **action byte at struct+0** and a **union tag at
struct+8**. The party window's own button dispatcher `FUN_1411c9540` compares the clicked
node's name against wide-string literals in `.rdata` and routes to them. All **[L]**:

| button | literal | route | action | tag |
|---|---|---|---|---|
| `create` | `0x14327f958` | `1411c9600` -> `FUN_1413b9bc0` -> `FUN_1413bdb80(cl = 0)` | **0** | 5 |
| `leave` | `0x14336f790` | `1411c97a3` tail-jmp -> `FUN_1413ba250` (immediate 1 at `1413ba32c`) | **1** | 1 |
| `pickup` | `0x14338dc68` | `1411c982b` -> `FUN_1413b9d90` -> `FUN_1413bdb80(cl = 2)` | **2** | 5 |
| `invite` | `0x143286378` | `1411c9651` tail-jmp -> `FUN_1411cac30` -> `FUN_1413b9eb0` (immediate 3) | **3** | 2 |
| - | - | `FUN_1413ba710` (immediate 4) - **zero callers, all three scans** | **4** | 3 |
| `expel` | `0x14338dc58` | `1411c971a` -> `FUN_1413ba590` -> `FUN_1413bda50(cl = 5)` | **5** | 4 |
| `leader` | `0x14338dbc8` | `1411c987c` tail-jmp -> `FUN_1411cb1d0` -> `FUN_1413ba470` -> `FUN_1413bda50(cl = 6)` | **6** | 4 |

The literals are the same node names `UserList.img/Party` uses (`button:create` ...
`button:leader`), which is the cross-check that the dispatcher and the window are the same
thing. Action **2 = pick-up rights** is corroborated a second way: its builder resolves
`0x0126` *"You are not the master of the party."*, and `FUN_1411c99e0` - the member list -
resolves `0x011D` *"All"* and `0x011E` *"Pick-Up Rights:"*.

Action **4** is the odd one: `FUN_1413ba710` writes the immediate 4 and resolves `0x0123`
*"You are already in a %s. Would you like to leave your existing %s and ask to join a new
%s?"*, which is unmistakably an apply-to-join. But it has **zero callers by `call`, by tail
`jmp` and by data pointer**, so what reaches it is not established and the row is **[D]**.

### The body is FlatBuffers, and that is measured

`python tools/encodes.py 0x1413bd0f0` (the encoder all five builders call) reports **exactly
one write, a `w_raw`**. **[L]** So the whole body is an opaque blob. Reading the builder says
what kind: a 0x400 buffer filled **downward** (`mov qword [rbp-9], rcx / sub rcx, r13`), 4-byte
alignment padding (`neg rdx / and edx, 3`), `u16` field offsets written into a side table, and
a signed offset back to it stored at the object's head. That is FlatBuffers' vtable layout.
**[D]**

The archived capture decodes cleanly as one, which is the confirmation:

```text
research/fixtures/portal-arrival-fixed-chat-and-party-world.log:267
16:48:31.299 <- 0x0182 UNKNOWN, 68 byte body

00  10 00 00 00              root uoffset -> 0x10
06  0a 00  0e 00             vtable: size 10, table size 14
0a  00 00  07 00  08 00      slot 0 ABSENT, slot 1 at +7, slot 2 at +8
10  0a 00 00 00              table: soffset 10 back to the vtable at 0x06
17  05                       slot 1: union tag = 5
18  0c 00 00 00              slot 2: uoffset -> 0x24
1e  06 00  08 00  04 00      payload vtable: size 6, table size 8, slot 0 at +4
24  06 00 00 00              payload table: soffset 6
28  04 00 00 00              slot 0: uoffset -> 0x2c
2c  11 00 00 00              string, 17 bytes
30  "TestCharD's Party"      -> ends at 65; the last 3 bytes are alignment padding
```

Slot 0 absent means the action equals its schema default, **0 = create**; tag 5 is what
`FUN_1413bdb80` writes; and the name is `String.wz` `0x0BAC` *"%s's Party"* formatted with the
character name. Three facts agreeing on one 68-byte capture. **[L]**

`crates/net/src/party.rs::parse_request` implements the smallest bounds-checked FlatBuffers
reader that answers this, and the capture is its fixture.

### The unanswered `0x0182` did not kill the client

The server logged *"is not answered yet"*, and the session continued: `0x00B8` at 16:48:37.311,
`0x013D` at 16:48:37.490, socket closed at 16:48:57.634 - **26 seconds later**. `STATUS.md`
line 3841 records the on-screen half: *"None has caused a freeze, so none is a blocking
request."* **[L]**

That is one observation and it is worth stating precisely: it says the client survives an
unanswered party request, not that answering is optional. Answer it anyway - §7 - because a
refusal costs three bytes and the alternative is a rule this project has paid for twice.

### `0x0183` is the invite answer, and it shares a numbering space with `0x00A5`

Codes `0x03` and `0x06` of `0x00A5` both build `0x0183` immediately
(`0x1413bb0ae`, `0x1413bb39b`) with the struct `{u8 0x1B, u8 answer, u64 value}`, encoded by
`FUN_1413bd500` - which `tools/encodes.py` also shows making exactly one `w_raw`. The op byte
is **`0x1B`**, which is the `0x00A5` code that carries the invite outcomes. The dialog module
`FUN_14180b750` / `FUN_14180c6e0` sends four more `0x0183`s through the same encoder. **[L]**

### Inbound `0x0182` is a completely different packet

`research/msexe-gamestage-cases.txt` gives channel case `0x0182` as an inlined body reading a
`u32` and a `u8`. **The two directions are different namespaces**, exactly as
`crates/net/src/userpool.rs` warns. Never answer a party request by echoing its opcode. **[L]**

---

## 4. The UI: confirmed, and the owner has already pressed a button on it

The instrument is `target/release/wz-dump.exe cat client-patched/Data/UI/UI_000.wz <img>`, and
its positive control is `research/guilds.md`'s finding, reproduced here unchanged: the same
read of `UserList.img` finds `Buddy`, `Party` and `Blacklist` and **no** `Guild`. A search that
can come back empty on the same file is a search worth believing when it comes back full.

`UI_000.wz/UserList.img/Party`: **[L]**

```text
title  list_head  list_body  list_tail
button:create (id 2001)   button:invite (2002)  button:expel  (2003)  button:leave (2004)
button:pickup (2005)      button:leader (2006)  button:whisper(2007)  button:talk  (2008)
vector:list_lt (8,57)   vector:list_rb (289,297)   scroll:list_scroll (length 270)
check:showhp (2200)     vector:pickup_text      vector:showhp_text
Member: nameWidth 95, jobWidth 89, marginY 3, leader/name/job/level, leader_normal,
        leader_selected, selected
```

`PartyHP.img` carries `nw..se`, `c`, `GaugeBar`, `GaugeBar2`, `userOn`, `userOff`, `bossOn`,
`bossOff` - the party HP bar. `StatusBar3.img` carries `check:inviteParty`, `button:bossParty`
and a `party` node. **[L]**

**89 party strings** in `String.wz` locale 0, including `Party`, `Party Leader`,
`/To Party(/p)` (a chat tab), `%s's Party`, `Recruit Party`, `Joining Party`,
`UI/UIWindow.img/PartyKilling/kill` and the 43-string result block. **[L]**

And the capture in §3 closes it: **the window opens, the Create button works, and it puts a
packet on the wire.** No amount of WZ reading proves that; one archived line does.

### What the window will *not* survive

`research/guilds.md`'s lesson applies to whatever the party window does next: `0x055E` type 10
selected a `UI/UIShop.img` tab that did not exist and **killed the client**. Everything listed
above is present, so the party window itself is not in that class - but nothing here has
watched it draw a member, and a `0x00A5` code whose arm expects a `str` and gets an empty body
is the same failure in a new place. That is why `net::party::refusal` refuses to build one.

---

## 5. The state machine, and where each rule comes from

Implemented in `crates/world/src/party.rs`. Nine of eleven rules are the client's own strings.

| rule | evidence |
|---|---|
| a character is in at most one party | `0x0121` *"Already have joined a party."*, `0x0132` *"You are already in a party."* **[L]** |
| you must be in a party to leave it | `0x0124` *"You have yet to join a party."* **[L]** |
| only the leader invites, expels, hands over | `0x0126` *"You are not the master of the party."*, resolved by the **request builders** - the client refuses locally and never sends **[D]** |
| you may not invite yourself | `0x0127` **[L]** |
| you may not invite someone already in a party | `0x0122` **[L]** |
| a repeat invite to the same character is refused | `0x011F` *"You have already invited '%s' to your party."* **[L]** |
| you may only expel or promote a member | `0x0128` *"'%s'is not a member of your party."* **[L]** |
| a full party refuses a join | `0x0125` **[L]** |
| **the leader LEAVING disbands the party** | `0x0114` *"You have quit as the leader of the party. The party has been disbanded."* + `0x0115` *"You have left the party since the party leader quit."* **[L]** |
| **the leader DISCONNECTING promotes instead** | `0x0AF2` *"Due to the party leader disconnecting from the game, %s has been assigned as the new leader."* **[L]** |
| at most `MAX_MEMBERS` = 6 | **[I]** - see below |

The last two leader rows are why the module has both `apply(Request::Leave)` and
`disconnect()`. **The client has a different message for each**, so they are two transitions,
not one with a flag - and collapsing them would have been the obvious design.

### `MAX_MEMBERS` is [I], and the negative behind it is measured

Two places that would fix a cap do not:

* the member list **scrolls** - `scroll:list_scroll`, `length 270`, `wheelRange 289`, over a
  box of `(8,57)..(289,297)`. The window's height caps nothing. **[L]**
* `FUN_1413b8d40`, the first thing the create-result arm calls, reads its bound out of a
  global (`1413b8d44 mov eax,[rip+0x27139b6] / test eax,eax / jle`) and passes it as a runtime
  count. No fixed array. **[L]**

**The blind spot, named**: that is two places, not an enumeration of `0x1413b8000..0x1413bf000`.
A cap in a function neither reaches would be invisible. What would settle it: enumerate every
immediate compared against a member count in that range - or seven characters and one screen,
once this machine can run two clients.

### Only four refusals have a code of their own

`AlreadyInAParty` -> `0x0F`, `NotInAParty` -> `0x11`, `PartyIsFull` -> `0x16`; everything else
-> the default `0x04`. That is not a gap in the search: `0x0126`, `0x0127` and `0x0128` are
resolved **only by the outbound builders**, so the client refuses those three locally and the
server never sees the request. All four codes are read-free, which
`crates/world/src/party.rs`'s tests assert against `net::party::is_silent_code` rather than
promise in a comment.

---

## 6. The predicate

```rust
/// Everyone who should be shown whatever `who` is shown. Always contains `who`.
pub fn Parties::audience(&self, who: CharacterId) -> Audience
pub fn Parties::shares_audience(&self, a: CharacterId, b: CharacterId) -> bool
```

`Audience` is non-empty, sorted, deduplicated and contains the character it was asked about -
four properties the tests assert, not four the doc claims. `shares_audience` is proved
reflexive, symmetric and transitive over a populated registry, and `audience(x).contains(y)`
is asserted equal to `shares_audience(x, y)` so the two cannot drift.

**It is membership, not co-location.** The caller intersects with the field; `broadcast::Bus`
already holds every subscriber's map. The client's own strings insist on the same distinction
- *"This can only be given to a party member within the vicinity."*, *"All party members must
be on the same channel."* **[L]**

### The seam into `crate::mobshare` already exists

`mobshare::Party::of(character, members)` is the value `may_see_drop` takes, and that module's
doc says the party agent's job is to *produce* it. `Audience` is exactly that list, so the
change is one line and `world::party` deliberately does not reference `mobshare` - the
dependency runs one way:

```rust
// instead of  mobshare::Party::solo(owner)
mobshare::Party::of(owner, parties.audience(owner).as_slice().iter().copied())
```

Both types promise non-empty, contains-the-owner and no-duplicates, so the conversion cannot
lose a member or invent one.

---

## 7. The smallest slice that makes party drops work

**No new opcode, no FlatBuffers writer, no party UI.** `crates/net/src/drops.rs` establishes
that the client stores `ownType` at `drop+0x70` and **never gates on it** - the drop the
client can see is the drop the server sent it. So:

1. `Fields` (or `Server`) gains one `Mutex<Parties>`.
2. Two GM chat commands, in `session/gm.rs`, because `/party` is swallowed by the client and
   `!` is ordinary chat: `!party invite <name>` and `!party leave`. Wired to
   `Parties::apply`, they need no packet decode at all.
3. `session/combat.rs`'s drop path replaces `mobshare::Party::solo(owner)` with
   `mobshare::Party::of(owner, parties.audience(owner)...)`.
4. `Session` teardown calls `Parties::disconnect(character_id)`.

That is the whole feature, and **none of it touches the client**. It is testable the moment
two clients can run, and every piece of it is unit-testable now.

### And then, cheaply, the client half

5. In the session dispatcher, answer `0x0182` with
   `net::party::refusal(refusal.result_code())` on a refusal and
   `net::party::request_failed()` on anything `parse_request` cannot read. Three bytes,
   read-free arm, and the client shows a real message instead of nothing.
6. `parse_request` already tells you which button was pressed and, for create, the name -
   so `!party` and the window's Create button can drive the same `Parties::apply`.

**What step 6 cannot do yet** is tell the *invitee* anything, because that is `0x00A5` code
`0x03`/`0x06`, whose six fields are read but whose meanings are not - and whose arm makes the
client answer `0x0183`, which would then also need handling. Invite-by-window is a second
piece of work; invite-by-`!party` is not.

## WIRE IT LIKE THIS

Exact anchors. **None of this is done** - `CLAUDE.md`'s *"built is not wired"*.

**1. `crates/world/src/server.rs` or `fields.rs`** - one registry per channel, beside
`Fields`:

```rust
parties: std::sync::Mutex<crate::party::Parties>,
```

`Parties` is `Default` and `Clone`, holds no clock and no handles.

**2. `crates/world/src/session/combat.rs`**, the drop path that today builds
`mobshare::Party::solo(...)`:

```rust
let audience = self.server.parties.lock().unwrap().audience(owner);
let party = mobshare::Party::of(owner, audience.as_slice().iter().copied());
```

Then intersect with the field the way `Bus::others_on` already does - `audience` knows nothing
about maps.

**3. `crates/world/src/session/gm.rs`**, beside `!map`:

```rust
"party" => self.on_party_command(arg),   // invite <name> | leave | list
```

Resolve the name to a character id with whatever `!` commands already use, then
`Parties::apply(actor, Request::Invite { target })`. On `Err(refusal)` say
`refusal.message()` in chat - it is the client's own wording.

**4. `crates/world/src/session/mod.rs`**, the inbound match, next to the other `0x01xx`
requests:

```rust
net::party::CLIENT_PARTY_REQUEST => {
    let payload = body.get(2..).unwrap_or(&[]);
    match net::party::parse_request(payload) {
        Some(req) => self.on_party_request(req),      // may still end in request_failed()
        None      => vec![Reply::raw(net::party::request_failed())],
    }
}
net::party::CLIENT_PARTY_INVITE_ANSWER => vec![Reply::raw(net::party::request_failed())],
```

The second arm matters: `0x0183` is what the client sends after `0x00A5` code `0x03`, and an
unhandled one is exactly the packet this project keeps being surprised by.

**5. `Session` teardown**: `parties.lock().unwrap().disconnect(self.character_id())`, and turn
the returned `Vec<Effect>` into whatever the remaining members are told.

**6. `crates/net/src/names.rs`** (coordinator's file, not touched here) - the row should read
something like:

```rust
0x0182 => "CLIENT_PARTY_REQUEST (FlatBuffers; action at table slot 0: 0 create, 1 leave, 2 pickup rights, 3 invite, 4 join request, 5 expel, 6 change leader)",
0x0183 => "CLIENT_PARTY_INVITE_ANSWER (FlatBuffers; op 0x1B, then an answer byte)",
0x00A5 => "PARTY_RESULT (u8 code; 45-arm switch, default 0x04 is a safe one-byte refusal)",
```

---

## 8. What I did NOT establish

Read this before building on anything above.

1. **Nothing in this document has been sent by this server.** `0x00A5` appears in **zero** of
   452 archived logs, with a working positive control. Every server->client claim is static.
2. **The bodies of eight arms are only partially read.** `0x0E`'s two sub-decoders
   (`FUN_1406f2830`, `FUN_1406f2560`), `0x10`'s two discriminating `u8`s, `0x1B`'s `raw`
   length, `0x03`/`0x06`'s six field meanings, `0x22`'s `u8`, `0x2D`'s helper. **Do not build
   any of these.** `net::party::refusal` will not let you by accident.
3. **Which of `0x03` and `0x06` is invite and which is join-request is [I].** Both read the
   same six fields and both answer `0x0183`; they differ only in the dialog constructor
   (`FUN_141808b90` vs `FUN_14180f2d0`). Rendering the two dialogs' art would settle it
   without a client run.
4. **The `0x0182` payload schema is known for tag 5 slot 0 and nothing else.** One capture,
   one action. `parse_request` returns `name: None` for every other shape, and that means
   "this decoder did not find one", never "the packet did not carry one".
5. **`MAX_MEMBERS` is [I]**, with the blind spot named in §5.
6. **The reachability sweep cannot see a vtable call.** `reads.calls_of` reads `call` and tail
   `jmp` only. A handler reaching the party module *only* virtually would be invisible, so
   "exactly two inbound cases touch party strings" carries that blind spot. **[D]**
7. **`FUN_1413ba710` (action 4) has zero callers by all three scans.** Which control reaches
   it is unknown; `tools/callers.py`'s own docstring says a computed or virtualised call
   leaves no trace.
8. **Nothing here has been seen with two clients**, which is what the whole feature is for.
   The module is deliberately shaped so that everything except "does the other player's screen
   change" is testable now: 42 unit tests in `crates/world/src/party.rs`, 13 in
   `crates/net/src/party.rs`, including the archived capture.
9. **Pick-up rights (action 2) are decoded as an action and not modelled.** The strings
   (`0x011A`, `0x011B`, `0x011D` *"All"*, `0x011E` *"Pick-Up Rights:"*) say the feature exists
   and that the leader sets it; nothing establishes the value set, and drop visibility does
   not depend on it.

---

## 9. Reproducing every number

All from the repo root; scripts piped in (`python - < s.py`) so `sys.path[0]` is empty.

```
python tools/reads.py    0x140304100 2         # the control - run this FIRST
python tools/reads.py    0x1413bab80 3         # the result handler's every read
python tools/listing.py  0x1413bab80           # the switch at 1413baf47..1413baf7e
python tools/callers.py  0x1413bab80           # 1 call site, and it is 0x142cbac6e
python tools/encodes.py  0x1413bd0f0           # ONE write, w_raw - the FlatBuffers body
python tools/encodes.py  0x1413bd500           # the same for 0x0183
python tools/listing.py  0x1411c9540           # the party window's button dispatcher
python tools/dump_stringids.py --grep "Cash Shop"   # the control
python tools/dump_stringids.py --grep party         # 89 hits, 43 of them contiguous

target\release\wz-dump.exe cat client-patched\Data\UI\UI_000.wz UserList.img
target\release\wz-dump.exe cat client-patched\Data\UI\UI_000.wz PartyHP.img

cargo test -p world --lib party --target-dir target/agent-party     # 42 pass
cargo test -p net   --lib party --target-dir target/agent-party     # 13 pass
```

**The string-id enumeration.** 120 981 `.pdata` functions, every `mov edx, imm32` immediately
before a `call 0x1408a9e40`:

```python
import sys, collections
sys.path.insert(0, "tools")
from rtti import load_pe
from dataref import parse_pdata
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
DATA, BASE, SEC = load_pe("client-patched/MapleStory.exe")
MD = Cs(CS_ARCH_X86, CS_MODE_64)
def foff(va):
    rva = va - BASE
    for s in SEC:
        if s["rsize"] and s["vaddr"] <= rva < s["vaddr"] + max(s["vsize"], s["rsize"]):
            return s["raddr"] + (rva - s["vaddr"])
hits = collections.defaultdict(set)
for start, end in parse_pdata(DATA, SEC, BASE):          # note the argument order
    o = foff(start)
    if o is None: continue
    edx = None
    for ins in MD.disasm(DATA[o:o + min(end - start, 0x8000)], start):
        if ins.mnemonic == "mov" and ins.op_str.startswith("edx, 0x"):
            edx = int(ins.op_str.split(", ")[1], 16)
        elif ins.op_str.startswith(("edx", "rdx")):
            edx = None
        elif ins.mnemonic == "call" and ins.op_str == "0x1408a9e40" and edx is not None:
            hits[start].add(edx); edx = None
# CONTROL: this must find thousands, not a handful.
assert len({i for s in hits.values() for i in s}) > 3000
PARTY = set(range(0x0108, 0x0133))
for fn in sorted(hits):
    got = hits[fn] & PARTY
    if got: print("%#x %2d %s" % (fn, len(got), " ".join("%#06x" % i for i in sorted(got))))
```

**The inbound sweep.** Every case in `research/msexe-gamestage-cases.txt` - **270 of 273 rows
carry a function, and the parser must be checked against that count**, because the row that
matters is an `INLINE` one:

```python
cases = []
for line in open("research/msexe-gamestage-cases.txt", encoding="utf-8"):
    p = line.split()
    if not p or p[0].startswith("#"): continue
    fns  = [int(x[4:], 16)  for x in p[1:] if x.startswith("FUN_")]
    fns += [int(x[10:], 16) for x in p[1:] if x.startswith("thunk_FUN_")]
    cases.append((int(p[0], 16), fns))
assert len(cases) == 273 and sum(1 for _, f in cases if f) == 270
```

then reach over `reads.calls_of` (call **and** tail jmp) to depth 5 and intersect with the map
above. Expected output, in full - this is a transcript, not a description:

```text
0x00a5  0x1413bab80  31 ids: 0x0108 ... 0x012e
0x00ea  0x142d9fa90   6 ids: 0x0121 0x0124 0x0126 0x0127 0x0128 0x0129
```

**The archive sweep**, event-deduplicated across both directories:

```python
import glob, re, collections
files = glob.glob("previous-runs/*.log") + glob.glob("research/fixtures/*.log")
pat = re.compile(r"^(\d\d:\d\d:\d\d\.\d\d\d)\s+(->|<-)\s+(0x[0-9A-Fa-f]{4})\s*(.*)$")
ev = collections.defaultdict(set)
for f in files:
    for line in open(f, encoding="utf-8", errors="replace"):
        m = pat.match(line.strip())
        if m:
            ts, d, op, rest = m.groups()
            ev[(d, int(op, 16))].add((ts, rest[:120]))     # the EVENT, not the file
assert len(ev[("<-", 0x02FF)]) > 100000                    # the control
for op in (0x00A5, 0x0182, 0x0183):
    print("%#06x  -> %d  <- %d" % (op, len(ev[("->", op)]), len(ev[("<-", op)])))
```

452 files. `0x00A5`: 0 and 0. `0x0182`: 0 and **1**. `0x0183`: 0 and 0.
