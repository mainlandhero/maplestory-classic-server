# Skills: the record block, the two packets, and the window that kills the client

Written 2026-08-20, from a Ghidra/static pass plus
`research/fixtures/skill-window-close-faults-{world,hook}.log`. **No client run was spent on
this.**

Every claim is tagged **[L]** (read off a listing, a capture or the client's own WZ), **[D]**
(derived from two or more [L] facts) or **[I]** (inferred). There is a
"[what I did NOT establish](#what-i-did-not-establish)" section at the end and it is not
short.

Listings dropped beside this file:

| file | what |
|---|---|
| `research/msexe-skillblock-140306d44.txt` | the character record's `presence[8]` skill block |
| `research/msexe-skillrecord-142d57f20.txt` | `0x0081` ChangeSkillRecordResult, the inbound handler |
| `research/msexe-skillup-142d4bd80.txt` | the `0x013B` skill-up request builder |
| `research/msexe-skill-masterlevel-140302650.txt` | `FUN_140302650`, the "has a master level" predicate |
| `research/msexe-skillwnd-teardown.txt` | the seven functions of the close-the-window crash chain |

Code: **`crates/net/src/skills.rs`** (new, 11 unit tests + 3 doctests, all passing) and one
line — `pub mod skills;` — in `crates/net/src/lib.rs`. **Nothing is wired.** See
["Wire it like this"](#wire-it-like-this).

---

## 0. The short version

| question | answer |
|---|---|
| Is the crashing `std::map` the skill map? | **No.** It belongs to a **UI button**, and the skill maps are hash maps at `charData+0x1039/+0x10b1/+0x10f9`, torn down by different code. [D] |
| Then what dies? | The skill window (UI type **3**) destroying its own child elements. The chain from `FUN_141157950` down to the faulting instruction is seven frames and each one checks out against the previous one's call site. [D] |
| Did the owner's `+` click send anything? | **Yes.** `0x013B`, once, at 22:17:30.209, body `ac88b80b e8030000 01000000` = tick / skill **1000** (Three Snails) / **1** point. The server logged it `UNKNOWN` and answered nothing. [L] |
| Why did clicking again do nothing? | The client sets a **one-request-outstanding latch** (`user+0x2330`) when it sends and only a server packet clears it. One `0x013B` per session, forever. [L] |
| What must the server send? | `0x0081` ChangeSkillRecordResult, and the `presence[8]` block in the character record so a level survives a relog. [D] |

---

## 1. This server has never sent a byte of skill data — confirmed

`grep -in skill crates/net/src crates/world/src crates/store/src` returns only the mob-skill
fields in `mobmove.rs`, the SP *stat* fields in `stats.rs`/`combat.rs`, and a comment in
`userhit.rs`. **[L]** The character record's `presence[8]` byte has never been set and no
channel packet has ever carried a skill id.

So the skill window the owner opened was drawn from `UI/Skill.img` plus the client's own
`Skill.wz` data for job 0, with every level read out of an empty map.

---

## 2. The character record: `presence[8]` is the skill block

### 2.1 The gate, confirmed from two independent key tables

**[L].** The record decoder `FUN_140304b20` gates this block at `0x140306d28` with key
`0x143abf280`; the client's own **encoder** `FUN_1402e5a30` — the mirror function
`research/quest-state.md` already used — gates the same block at `0x1402e6a4c` with key
`0x143abdff0`. Two different tables, and both keys have exactly one initialiser, and both
initialisers write byte **+8**:

```text
140023710  lea rcx,[0x143abf280] / call 0x140302c70   ; memset(key, 0, 100)
140023722  mov byte ptr [0x143abf288], 1              ; key[8] = 1      <- decoder
140022c20  lea rcx,[0x143abdff0] / call 0x140302c70
140022c32  mov byte ptr [0x143abdff8], 1              ; key[8] = 1      <- encoder
```

That is the same shape `research/charrecord-presence-map.md` documents for all 40 gates, and
the table in that file already lists **presence byte 8 → entry 17 → region
`[0x140306d44, 0x1403073c3)`, 21 reads, loops #13–#19**. Nothing new is being claimed about
the mechanism; what is new is **what the block is**.

`tools/dataref.py` was the instrument for the initialisers. Positive control: the same call
on `0x143abf288`'s neighbour table entry for quests returns the initialiser
`research/quest-state.md` already names, and the `0xC6` opcode hole that made this tool
report 1 write where there were 41 is fixed (see `charrecord-presence-map.md` §"How this was
measured").

### 2.2 Why it is skills and not a name off a table

Three independent [L] facts, none of which is an opcode-name alignment:

1. **The block calls `FUN_140302650(x)` on its first field**, and that function's literal
   comparisons are `4340012`, `4340010`, `2221009`, `1120012`, `1320011`, `2121425`,
   `2320498`, `5120011`, `5120012`, `5220206`, `5221022`… — **MapleStory skill ids** — and it
   divides its argument by `10000` to get a job id and by `100` for the `8000xxxx` numbering.
   `research/msexe-skill-masterlevel-140302650.txt`.
2. **Its destination `charData+0x1039` is read by the skill-window module.** A whole-`.text`
   displacement scan for `0x1039` (see §6 for the instrument) puts readers at `0x14258bc0c`,
   `0x14258bd35`, `0x14258f430`, `0x14258f558`, `0x142592920` — all inside `0x14258xxxx`,
   which is the module that loads `UI/Skill.img/entry`, `UI/Skill.img/entry/posBtSpUp` and
   `UI/Skill.img/entry/BtSpUpAll`.
3. **`0x0081`'s handler writes the same three maps**, and `0x0081` is the opcode the *only*
   other skill-shaped code path in the image uses. §3.

**[D]**: `presence[8]` is the skill block.

### 2.3 The wire format

Region `[0x140306d44, 0x1403073c3)`. The `u8` at `0x140306d47` forks the whole block:

```text
u8  bulk                                       140306d47
if bulk != 0:                                  ; SNAPSHOT - this is the arm to use
    u16 count                                  140306d57
    if count == 0 -> JMP 0x1403073c3           140306d61   <- the block ENDS. 3 bytes total.
    clear(charData+0x1039)                     140306d6e   FUN_1402fe4a0
    clear(charData+0x1009)                     140306d7a
    clear(charData+0x1021)                     140306d86
    count x {
        u32  skillId                           140306d96
        u32  level                             140306da3   -> map +0x1039
        raw8 expiration                        140306dcf   -> map +0x10f9  (FILETIME)
        if FUN_140302650(skillId):
            u32 masterLevel                    140306df9   -> map +0x10b1
    }
    JMP 0x1403073c1                            140306e24   <- and the delta lists are SKIPPED
else:                                          ; DELTA - six u16-counted lists, in order
    u16 n1 { u32 skillId, u32 level }          140306e29 / 140306e43 / 140306e52
    u16 n2 { u32 skillId }                     140306f21 / 140306f34     (levels removed)
    u16 n3 { u32 skillId, raw8 expiration }    14030700f / 140307029 / 140307042
    u16 n4 { u32 skillId }                     140307105 / 140307118     (expirations removed)
    u16 n5 { u32 skillId, u32 masterLevel }    1403071e4 / 140307203 / 140307212
    u16 n6 { u32 skillId }                     1403072e1 / 1403072f4     (master levels removed)
```

All **[L]**. Read counts: 7 list counts + 13 body reads + the leading `u8` = **21**, which is
exactly the number `charrecord-presence-map.md` records for this region. Two instruments
agreeing is what makes the layout worth trusting.

The nine destinations are three triples — *current*, *changed*, *removed*:

| | current | changed | removed |
|---|---|---|---|
| level | `+0x1039` | `+0x1009` | `+0x1021` |
| master level | `+0x10b1` | `+0x1081` | `+0x1099` |
| expiration | `+0x10f9` | `+0x10c9` | `+0x10e1` |

**[L]** from the `lea`/`mov` operands in the listing.

### 2.4 The client's own encoder writes the same thing

`FUN_1402e5a30`, the record *encoder*, gated at `0x1402e6a4c`:

```text
1402e6a75  call 0x1406ed840    ; u8  bulk
1402e6a8e  call 0x1406ed940    ; u16 count
1402e6b2b  call 0x1406ed9d0    ; u32 skillId    <- from +0x1039
1402e6b35  call 0x1406ed9d0    ; u32 level
1402e6b95  call 0x1406ede20    ; raw 8          <- from +0x10f9
1402e6b9c  call 0x140302650    ; the same predicate
1402e6bea  call 0x1406ed9d0    ; u32 masterLevel<- from +0x10b1
1402e6c0c  call 0x1406ed940    ; u16 ... and then the six delta lists, same order
```

**[L].** Field-for-field agreement between the decoder and the encoder. This is the strongest
form of evidence this project has for a record block and it is why `skill_block()` writes the
bulk flag as a **constant**: the flag and the block's length are the same decision, exactly as
`QUEST_BLOCK_BULK` is.

### 2.5 `masterLevel` is conditional, and the condition is provable for us

`FUN_140302650(skillId)` — full listing in
`research/msexe-skill-masterlevel-140302650.txt`. Its tail is:

```text
1403027ee  MOV  ECX,EDI          ; EDI = skillId/10000, the job
1403027f0  CALL 0x140286e90      ; ESI = f(job)
1403027f9  CALL 0x140302540      ; AL  = g(job)
140302800  JNE  0x14030284c      ;   g(job) != 0                  -> return 0
           <six literal skill ids: 4311003 4321006 4330009
            4331002 4340007 4341004>            ->                   return 1
140302832  CMP  ESI,4
140302835  JNE  0x14030284c      ;   f(job) != 4                  -> return 0
140302837  MOV  EAX,1                                             -> return 1
```

and `FUN_140302540(job)`, for a job outside its four jump-table sets, **is** `f(job) == 4`
(`1403025ac CALL 0x140286e90 / CMP EAX,4`). The two tests are therefore complements for an
ordinary job, so only the six literal ids reach `return 1`. **[D]**

For job **0** — every character this project has — the answer is [L] with no unknowns at all,
because both arms terminate:

* if `g(0) == 0`, then at `0x14030279b` `esi % 1000 == 0` fires on `0` → **return 0**;
* if `g(0) != 0`, the branch at `0x140302782` skips ahead and the *second* `g(0)` at
  `0x1403027f9` returns 0 at `0x140302800` → **return 0**.

`crates/net/src/skills.rs::needs_master_level` implements the six-id set, and a unit test
pins that every beginner skill costs 16 record bytes and a Dual Blade skill costs 20.

### 2.6 Where the block goes in the record

Gate addresses are monotonic in the decoder and there is no back edge between them, so block
order is address order **[L]**:

```text
0x1403061a0   presence[2]    the equipped list          (already sent)
0x140306d28   presence[8]    THE SKILL BLOCK            <- new
0x1403074a3   presence[9]    started quests             (already sent)
0x14030757b   presence[14]   completed quests           (already sent)
0x140308b3f   -              the final ungated u8       (already sent)
```

Everything between `0x1403061a0` and `0x1403074a3` was already audited when the quest blocks
went in (`crates/net/src/opcode.rs`, `character_record_for_set_field_with_quests`): the five
static gates in that range key on presence bytes 44, 21, 27, **8** and 15, and the two dynamic
gates cost nothing. Byte 8 is the one that was clear and is now ours; the audit is otherwise
unchanged. **[D]**

---

## 3. `0x0081` — the skill packet a real server sends

### 3.1 Routing

`research/msexe-gamestage-cases.txt` line 10: `0x0081 -> FUN_142d57f20`. **[L]**

> ### `0x0081` IS ALREADY IN USE ON THE OTHER CONNECTION, IN THE OTHER DIRECTION
>
> On the **login** socket `0x0081` is the client's *check name* request
> (`docs/character.md` §"`0x0081` ↔ `0x0014`"). On the **channel** socket it is
> server→client ChangeSkillRecordResult. `crates/net/src/names.rs` has one shared
> `opcode_name` table and **no** entry for `0x0081` today; adding a channel name to it would
> mislabel every name-check line in `login.log`. If it needs a name, the table needs a
> direction or a stage first.

The candidate name "ChangeSkillRecordResult" comes from
`research/msexe-gamestage-opcodes.md`, which is an order-preserving alignment against other
versions and is explicitly **[I]**. What makes the *identification* [D] rather than [I] is
that the handler writes `charData+0x1039`, `+0x1081`, `+0x1099`, `+0x10b1`, `+0x10c9`,
`+0x10e1`, `+0x10f9` — the same seven collections the record's `presence[8]` block writes —
and reads the skill-window singleton `0x143aca600`.

### 3.2 Body

`tools/reads.py 0x142d57f20 2` finds 9 direct reads; the listing puts them in this shape:

```text
u8   clearRequestLatch   142d57f4a   != 0 -> FUN_142cc4430(user, 0)     see §4.2
u8   showEffect          142d57f60   != 0 -> the skill-up effect and String ids 0xf6e / 0xf6f
u8   (ignored)           142d57f71   read; AL is clobbered by CALL 0x142cbe730 four
                                     instructions later and the value is never stored
u16  count               142d57fad   0 -> straight to the trailing u8
count x {                            20 bytes, unconditionally
   u32  skillId          142d57fe5
   i32  level            142d57ff4   SIGNED. TEST R15D,R15D / JNS at 142d5803f:
                                     negative erases from +0x1039 and +0x1009
   u32  masterLevel      142d58229   UNCONDITIONAL here - unlike the record
   raw8 expiration       142d58539
}
u8   tail                142d5888d   -> FUN_1428a7f00(uiSingleton, v); always read
```

All **[L]**. Total **6 + 20n bytes**.

The asymmetry in `masterLevel` is the trap: the record gates it behind `FUN_140302650` and
this packet does not. `0x142d58225` is a join point every path in the loop body reaches, and
the read is the first instruction after it.

### 3.3 It is a silent no-op before the first `SetField`

```text
142d57f79  CALL 0x142cbe730        ; FUN_142cbe730 = MOV RAX,[RCX+0x2358] / RET
142d57f7e  TEST RAX,RAX
142d57f81  JZ   0x142d58b85        ; -> return, having read three bytes and done nothing
```

**[L].** `+0x2358` is the character-data object and `STATUS.md` records it measuring `0x00`
on the *first* `SetField`. So `0x0081` must follow the record, never replace it — the same
trap `crate::quest`'s `0x0089` documents.

---

## 4. `0x013B` — what the client sent when the owner clicked `+`

### 4.1 The builder, and the capture agrees byte for byte

`FUN_142d4bd80(user, edx = skillId, r8d = count)`, reached from the skill window at
`0x142586ddd`:

```text
142d4bda8  MOV  EDX,0x1f4 / CALL 0x142cc42d0   ; 500 ms throttle AND the latch check
142d4bdb4  JE   ... -> send nothing at all
142d4bdb6  MOV  EDX,0x13b / CALL 0x1406ed520   ; begin packet 0x013B
142d4bdc6  CALL 0x1429e3ef0 / write u32        ; the client's own tick
142d4bdd7  write u32 EDI                       ; skillId
142d4bde3  write u32 ESI                       ; count
142d4bdf4  CALL 0x1415d01c0                    ; send
142d4bdf9  MOV  EDX,1 / CALL 0x142cc4430       ; SET the latch
```

Capture, `research/fixtures/skill-window-close-faults-world.log` line 36668:

```text
22:17:30.209 <- 0x013B UNKNOWN, 12 byte body ac88b80b e8030000 01000000
                                             tick     1000     1
```

**[L], both halves.** `1000` is Three Snails. `0x013B` appears **exactly once** in the whole
40 894-line log; the census is
`grep -oE "<- 0x[0-9A-F]{4}" … | sort | uniq -c`, 26 distinct inbound opcodes, and it is the
only one that is a plausible skill request.

### 4.2 The latch: one request per session, forever

```text
FUN_142cc42d0(user, ms, 0):
    142cc42da  CMP dword [rcx+0x2338],0 / JNE -> return 0
    142cc42e8  CMP dword [rcx+0x2330],0 / JNE -> return 0        <- THE LATCH
    142cc4312  now - [rcx+0x2334] < ms         -> return 0
               otherwise                       -> return 1

FUN_142cc4430(user, v):
    142cc4439  [rcx+0x2330] = v
    142cc4444  [rcx+0x2334] = now
```

**[L].** The sender sets it to `1` immediately after sending, and the only thing in this whole
story that sets it back to `0` is `0x0081`'s first byte. So the client's own
`0x0070`/`0x00E5`/`0x02FF` traffic continues normally — the capture shows it does, right up to
0.4 s before the fault — while **every request in this family is silently dropped at the
client**. `FUN_142cc42d0` has 264 call sites in 192 functions and `FUN_142cc4430` 241 in 204,
so the blast radius is not skills. **[L]** for the counts, **[D]** for "the same field on the
same object" — every call site seen used `+0x2330` on the first argument, but 192 functions
were not each checked.

This is not new to the project: `research/npc-click.md` §4.3 found the same latch, and
`crates/net/src/stats.rs::StatChange::excl_request_sent` is the same byte on `0x007C`. It
generalises `CLAUDE.md`'s **always answer** rule: an unanswered packet does not only freeze
the UI, it can silently disable a whole class of future requests.

### 4.3 The client does not apply the level itself

The send at `0x142d4bdf4` is followed by the latch set and the function's epilogue; the caller
fragment `FUN_142586d7f` returns immediately after. **Nothing writes `charData+0x1039` on the
send path.** **[L]** So Three Snails stayed at its old level because the level only ever
changes when `0x0081` (or a record) arrives — exactly what the owner saw.

### 4.4 The SP counter was zero and the client sent anyway

`crates/net/src/opcode.rs::character_stat_block` writes an **empty extended SP table** (`u8 0`)
for job 0, so the client's SP total is `0`. It sent `0x013B` regardless. **[L]** for both. So
"no SP" is *not* why the click appeared to do nothing — the request went out and was ignored.
Whatever gates the button is inside `FUN_142cc0370` and `FUN_1407e4d70`, which were not walked.

---

## 5. The crash: closing the skill window

### 5.1 The chain, seven frames, each verified against the previous one's call site

Fault: `research/fixtures/skill-window-close-faults-hook.log` line 13549 —
`CLIENT FAULT #1: code=0xc0000005 at 0x140fbc7cd`, hook clock `18:18:19.317`.

```text
0x143ad68a0(?)      a spilled pointer to the button class's memory pool - not a frame
0x140fbc7ef  <-TEXT return of `call 0x140fbc7b0` at 140fbc7ea   THE RECURSIVE SITE
0x1416825cb  <-TEXT return of `call 0x140fbc7b0` at 1416825c6   in FUN_141682580
0x141691cf4  <-TEXT return of `call 0x141682580` at 141691cef   in FUN_141691ce0
0x14126b132  <-TEXT return of `call qword [rax]`  at 14126b130  the refcount release
0x142c03228  <-TEXT return of `call 0x14126b0d0`  at 142c03223  in FUN_142c03210
0x142bf424e  <-TEXT return of `call qword [rax]`  at 142bf424c  in FUN_142bf3f70
0x143ad0285(?)      not code
0x141157506  <-TEXT return of `call 0x142bf3f70`  at 141157501  in FUN_1411574a0, CASE 3
0x1411579b4  <-TEXT return of `call 0x1411574a0`  at 1411579af  in FUN_141157950
```

**Every `<-TEXT` entry is exactly `call-site + instruction length` of a call that really exists
in the named function**, checked with `tools/callers.py` and `tools/listing.py`. `CLAUDE.md`'s
warning that "a stack scan is not a call stack" is why this was checked frame by frame rather
than read off. **[D]**

Reading it top-down:

```text
FUN_141157950(pWnd, flag)      calls pWnd->vtable[0xd0]() to get a UI type, then
FUN_1411574a0(type, flag)      switch(type) { ... case 3: ... }
    1411574f1  mov rcx,[0x143aca600]     ; the singleton for UI type 3
    141157501  call 0x142bf3f70          ; destroy its child-element list
FUN_142bf3f70(pWnd)
    14258418c  FUN_142c03300(&local, &pWnd->list @ +0x118)   ; move the list out
    pass 1: for each node -> element->vtable[0x18]()
    pass 2: for each node -> holder->vtable[0](1)            ; the deleting destructor
                                                            ; <- THE FAULT IS IN PASS 2
FUN_142c03210(holder, 1)       releases the smart pointer at holder+0x28, then frees 0x38
FUN_14126b0d0(sp)              lock xadd [obj-0x20+0x20],-1 ; refcount hits 0
                               -> obj->vtable[0](1)
FUN_141691ce0(obj, 1)          the scalar deleting destructor; frees 0x1438 bytes
FUN_141682580(obj)             the real destructor
    1416825ad  mov r8,[rcx+0x1420] / mov r8,[r8+8]   ; map._Myhead->_Parent, the root
    1416825c6  call 0x140fbc7b0                      ; std::map teardown
FUN_140fbc7b0(map, map, node)  the recursive red-black-tree _Erase
    140fbc7cd  cmp byte [r8+0x19],0    <- FAULT
```

### 5.2 UI type 3 is the skill window, and that is measured

`FUN_142582da0` is the constructor of the object at `0x143aca600`:

```text
142582db2  mov r9d,1 / xor r8d,r8d / lea edx,[r9+2]   ; edx = 3
142582dbf  call 0x142747470                          ; base ctor(this, 3, 0, 1)
142582ddc  mov [rip+0x154781d], rax                  ; 0x143aca600 = this
```

**[L]**, and `FUN_142582ed0` — the same class's load routine, which also writes
`0x143aca600` at `0x14258302a` — is the function that `lea`s `UI/Skill.img` at
`0x142582f20`. `tools/dataref.py 0x143aca600` finds **18** references and the only two writes
are those two, both in the skill module. **[D]: the singleton destroyed by case 3 is the skill
window.**

### 5.3 Whose map it is

Not the skill map. The object is **0x1438 bytes** (`141691cf9 mov edx,0x1438` before
`operator delete`), constructor `FUN_141682360`, destructor `FUN_141682580`, primary vtable
`0x1433ca498`. Its `std::map` lives at `+0x1420` and is the class's last member:

```text
141682500  lea rbx,[rdi+0x1420]
14168250c  mov [rbx],rsi / mov [rbx+8],rsi        ; _Myhead = 0, _Mysize = 0
141682513  lea edx,[rsi+0x30] / call 0x14019b780  ; the 0x30-byte sentinel node
141682522  node->_Left = node->_Parent = node->_Right = node
14168252d  mov word [rax+0x18], 0x101             ; _Color = 1, _Isnil = 1
141682533  mov [rbx],rax
```

**[L].** Node stride `0x30`: `_Left/_Parent/_Right/_Color/_Isnil` then an 8-byte key at
`+0x20` and an 8-byte **pointer** value at `+0x28` which the destructor releases through
`vtable+0x10`. The only other function in `0x141670000..0x1416a0000` that touches `+0x1420` is
`FUN_14168c130`, which iterates it and calls `value->vtable[0x208]()` while handling
`VARIANT`/`BSTR` — so the mapped type is a COM-ish object with a vtable of at least `0x210`
bytes. **[L]**

**The class is a UI button.** `FUN_1425840a0` — the skill window's row builder, 3273 bytes,
which `lea`s `UI/Skill.img/entry/BtSpUp` at `0x1425844ab` and `UI/Skill.img/entry/BtSpUpAll`
at `0x1425844d0` — allocates and constructs exactly two of them per row:

```text
142584682  mov edx,0x1438 / lea rcx,[0x143ad68a0] / call 0x14019b780   ; pool alloc
14258469f  call 0x141682360                                            ; construct
142584734  call qword [rax+0x80]   with the BtSpUp descriptor at [rbp+0x80]
14258479e  stored into the row at +0x11d0
   ... and again at 1425847bf for BtSpUpAll
```

**[L]**, and `0x143ad68a0` — that class's pool — is the first entry on the fault stack.
`FUN_141682360` has **246 call sites in 106 functions**, which is what a generic button looks
like. **[D]: the map is a UI button's, not the skill system's.**

The skill *data* lives in the hash maps at `charData+0x1039/+0x1081/+0x1099/+0x10b1/+0x10c9/
+0x10e1/+0x10f9` — the `FUN_1402fc930` family, buckets and chains, not a red-black tree — and
none of them is destroyed anywhere in this chain. **[L]**

### 5.4 The map was NOT empty, and that kills the tidy explanation

The obvious story — "the client tears down a structure it never populated, same as the shop"
— **is not what the evidence shows.** An empty MSVC `std::map` has
`_Myhead->_Parent == _Myhead` with `_Isnil = 1`, so the *first* call from `FUN_141682580`
would take `140fbc7d2 JNE` and return without recursing.

The stack carries `0x140fbc7ef`, which is the return address of the **recursive** call at
`0x140fbc7ea`. So the walk descended at least one level before faulting, and the faulting read
is `[r8+0x19]` where `r8 = node->_Right` of a node that had already been accepted as non-nil.
**[D]: the tree had at least one real node, and one of its child pointers was garbage.**

That is a use-after-free or a heap overwrite, not an uninitialised container. Which one, and
whether the button or only its map is stale, is **not established** — see §7.

### 5.5 Two things the log rules out

* **The four C++ throws are startup noise.** They are at hook `18:10:24.728`, `18:10:24.896`,
  `18:10:31.154`, `18:10:31.155` — the first 8 seconds — and the fault is at `18:18:19.317`,
  **475 seconds later**. The throw-logging window bug that made run 2 of the shop
  investigation look throw-free is fixed and the line reports "4 seen, 4 logged". **[L]**
* **The client was not wedged first.** It sent `0x00D9` at 22:18:18.834 and `0x02FF` at
  22:18:18.849, **0.47 s** before the fault. That is the opposite of the shop crash, where the
  client went silent the instant our `0x0560` landed and died 3.5 s later. **[L]** So `0x0560`
  and this are different failures, and nothing here suggests a server packet is the trigger.

### 5.6 Nothing the server sent is in that map

The server has never sent skill data, and the map belongs to a button constructed from
`UI/Skill.img` — which this client **does** ship, in full: `wz-dump cat UI_000.wz Skill.img`
lists `entry/BtSpUp/{normal,pressed,disabled,mouseOver}`, `entry/BtSpUpAll/...`,
`posBtSpUp {x:133,y:21}`, `button:close`, `tab:grade`, `scroll:entryScroll` and `cooltime/0..`.
**[L]** So this is *not* the `UI/UIWindow2.img` shape of the `0x0560` shop crash — the assets
are all present.

---

## 6. Instruments, and the controls for them

| instrument | what it answered | control |
|---|---|---|
| `tools/callers.py` | callers of `0x140fbc7b0` (13), `0x141682360` (246/106), `0x142cc42d0` (264/192) | its own docstring control, 43 calls in `0x140304b20`; and `0x142580390` came back "NOTHING FOUND" with the tool printing its own warning, which is the behaviour that stops a zero being read as a fact |
| `tools/listing.py` | every listing quoted here | shares `reads.py`'s loader and its merged `.pdata` extent, so it cannot disagree with the read counter about where a function ends |
| `tools/reads.py 0x142d57f20 2` | 9 direct reads in the `0x0081` handler | cross-checked against the listing by hand; the two agree |
| `tools/dataref.py` | the two presence-key initialisers, and `0x143aca600`'s 18 refs / 2 writes | positive: the quest keys' initialisers, already in `quest-state.md` |
| `tools/xref.py --va 0x1433ca498` | 2 refs, the ctor and the dtor | the same call found `0x143470c40` (`posBtSpUp`) with one referrer, which is the positive control that it can see `lea`s into `.rdata` |
| `wz-dump` | `UI/Skill.img` is complete | `research/npc-shop.md`'s note that a raw byte grep over a WZ archive is worthless still holds; this used `wz-dump cat` |

**Two instrument notes, both required by `CLAUDE.md`:**

1. **`tools/fieldrefs.py` drops `rbp`- and `rsp`-based operands.** For the `0x1039` question I
   did **not** rely on it. I wrote a separate scan that byte-prefilters `.text` for the
   little-endian disp32 and then decodes a window around each hit, **keeping `rbp` and `rsp`**,
   and it found `mov rdx,[rbp+0x1039]` at `0x14124bfab` and `mov rcx,[rbp+0x1039]` at
   `0x14124c17c` that `fieldrefs.py` does not report. Neither changed a conclusion here, but
   the scan that produced the readers list is the one that keeps them.
2. **`tools/rtti.py` cannot name any of these classes.** `--vtable CSkillInfo` prints
   *"0 locator(s)"*: the type descriptors survive but the complete-object locators do not, and
   `0x1433ca498-8` holds an ordinary function pointer rather than a COL. So **every class name
   in this document is a description, not an RTTI string** — "the skill window", "a UI button"
   — and none of them was taken from the reference tree.

---

## 7. What I did NOT establish

1. **Why the button's map node is corrupt.** §5.4 rules out "empty container"; it does not
   choose between a use-after-free of the button, a stale holder left in the window's element
   list, and a heap overwrite from somewhere else entirely. **The most likely single cause is
   `FUN_142bf3f70`'s two-pass teardown** — pass 1 calls `element->vtable[0x18]()` on every
   node before pass 2 deletes them, and if pass 1 can free an element the second pass walks a
   dangling holder — but that is **[I]** and no evidence here supports it over the
   alternatives.
2. **Whether the crash is specific to the skill window at all.** `FUN_1411574a0` is a switch
   over UI types and `FUN_142bf3f70` has **927 call sites in 480 functions**. Closing the
   inventory or the stat window may do the same thing. This is the cheapest possible
   discrimination and it costs one launch: see §8.
3. **Whether sending skill data prevents it.** Nothing measured connects the two. It is
   plausible — a populated skill map changes how many rows `FUN_1425840a0` builds — and it is
   entirely unproven. **Do not tell the owner the skill packet fixes the crash.**
4. **`FUN_140286e90`**, the job→tier function, has no `.pdata` entry and was not disassembled.
   §2.5 is arranged so that nothing depends on it for job 0.
5. **What `showEffect` (`0x0081` byte 1) looks like**, and what String ids `0xf6e`/`0xf6f` say.
   The two `FUN_1408a9e40` calls are [L]; the text is not.
6. **The `0x0081` trailing `u8`'s meaning.** It is passed to `FUN_1428a7f00(singleton, v)` and
   a `0x283` notification follows. `0` is what this module sends and no reason was found to
   send anything else.
7. **`SpPool::job_level`.** Still [I], as `crates/net/src/stats.rs` already says. It matters
   the first time SP is actually awarded, which is the moment the `+` button's own gating
   starts to matter too.
8. **What gates the `+` button.** `FUN_142cc0370(user, skillInfo) == 1`,
   `[skillInfo+0xb6] == 0`, `[skillInfo+0x3c] <= 0` and `FUN_1407e4d70(...)` were read off the
   call site and not walked. All four passed in the capture, which is all that was needed.
9. **The delta arm of the record block, and `0x0081`'s interaction with it.** Decoded, never
   built, never sent. `skill_block()` deliberately cannot produce it.
10. **Skill *cooldowns*.** `UI/Skill.img/cooltime/0..` exists and `charData+0x10c9`/`+0x10f9`
    are expirations, not cooldowns. No cooldown packet was looked for.

---

## Wire it like this

`crates/world/src/session/` belongs to the coordinator; none of the following was done.

### Step 1 — the record block (makes a skill level survive a relog)

`crates/net/src/opcode.rs`, `character_record_for_set_field_with_quests`, is already the
"pop the tail, add the blocks, put the tail back" shape. The skill block goes **before** both
quest blocks, because `0x140306d28 < 0x1403074a3`:

```rust
out[crate::skills::PRESENCE_SKILLS] = 1;          // presence byte 8
let tail = out.pop().expect("the character record is never empty");
out.extend_from_slice(&crate::skills::skill_block(skills));   // <- NEW, first
out.extend_from_slice(&quests.started_block());
out.extend_from_slice(&quests.completed_block());
out.push(tail);
```

Send `skill_block(&[])` (three bytes, `01 00 00`) for a character with no skills rather than
leaving the presence byte clear — same argument as `EMPTY_QUEST_BLOCK_LEN`: `bulk = 1` makes
the client *clear* its collections, so "this character knows nothing" is a statement.

> **This is the byte that can lose a whole world entry.** The record has no length prefix. If
> the block is wrong the symptom is an undressed character or a client that never enters the
> field — not a wrong skill list. `python tools/channel_smoke.py` exercises the record over an
> independent transport and is the right place to catch it before a launch.

### Step 2 — answer `0x013B` (makes the `+` button work)

In `Session::handle`, add `0x013B`:

```rust
0x013B => {
    let req = net::skills::SkillUpRequest::parse(body)?;   // u32 tick, u32 skillId, u32 count
    // clamp: nothing authenticates, so `count` is whatever arrived on the socket
    // ... look the skill up, check SP, raise the level, persist ...
    Some(packet(net::skills::CHANGE_SKILL_RECORD_RESULT,
                &net::skills::change_skill_record_result(
                    true,   // clear_request_latch  <- NOT OPTIONAL, see below
                    true,   // show_effect
                    &[net::skills::SkillChange::Learn(skill)])))
}
```

**Answer it even when the answer is no** — `net::skills::skill_up_refused(reason)` is six
bytes and exists for exactly that. Without a reply the client's `user+0x2330` latch stays set
and it will never send another request in that family for the rest of the session. That is a
stronger version of `CLAUDE.md`'s always-answer rule and it is the thing that made the owner's
repeat clicks do nothing.

### Step 3 — SP, so the number on screen agrees

`crates/net/src/stats.rs` already builds `0x007C` with `bits::SP` and
`Sp::Extended` (job 0 takes the extended branch). Send it **alongside** the `0x0081`, or the
skill goes up while the SP counter stays at whatever the record last said. `SpPool::job_level`
is still **[I]** — that is the risk in this step, not the packet.

### Step 4 — persistence

`crates/store` has no skill table and no `sp` column. A `character_skills` table keyed
`(character_id, skill_id)` with `level`, `master_level`, `expires_at` maps 1:1 onto
`net::skills::Skill`, and `characters` needs an `sp` column beside the existing `ap`. Loading
it into `skill_block()` at `SetField` closes the loop.

### Ordering, and the trap in it

```text
SetField (record, presence[8])   ->   0x0081 may now be sent
```

never the other way round: `0x0081` returns without doing anything while
`charData` (`+0x2358`) is null, and `STATUS.md` records that measuring `0x00` on the first
`SetField`. §3.3.

### The one launch worth spending, and what each outcome means

**Do not combine these.** One variant at a time, as `CLAUDE.md` requires.

| # | do | watch for | what it means |
|---|---|---|---|
| 1 | enter the world and open the skill window, **change nothing else**, then close it | does it still crash? | this is the control. The crash has never been reproduced twice |
| 2 | open and close the **inventory** or **stat** window instead | crash or no crash | crash → `FUN_142bf3f70`'s teardown is generic and the skill window is innocent; no crash → it is the skill window's element list, and §7 item 1 is the next static question |
| 3 | with step 1 wired: enter the world with Three Snails at level 1 in the record | the skill window shows **Lv. 1**, and the level survives `!map 40` and a relog | the `presence[8]` block landed |
| 4 | with step 2 wired: click `+` | the level goes up **and a second click also works** | the second click is the real test — it proves `clear_request_latch` cleared `user+0x2330` |

Step 4's second click is the cheap, decisive observation in this whole document: one working
click proves the packet, two prove the latch.
