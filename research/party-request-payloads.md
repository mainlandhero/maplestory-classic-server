# Inbound party packets: the seven `0x0182` payloads and `0x0183`, byte for byte

Written 2026-09-03. **No Ghidra** (the project lock is held elsewhere), **no client run**.
Everything below is `tools/listing.py`, `tools/callers.py`, `tools/reads.py`,
`tools/encodes.py`, `tools/dump_va.py`, `tools/xref.py`, `tools/dump_stringids.py`,
`tools/pdata_lookup.py`, `target/release/wz-dump.exe`, and the one archived capture.

Companion to `research/party.md`, which established *that* `0x0182` is FlatBuffers and *that*
there are seven actions. This one gives the **payload schemas**. The outbound `0x00A5` success
bodies are a different agent's file (`research/party-result-0x00A5.md`) and are not touched here.

Tags: **[L]** read off a listing, a table, the WZ, the string table or the capture;
**[D]** derived from two or more of those; **[I]** inferred, unconfirmable on this machine.

---

## 0. The answers, up front

1. **Invite carries the target's NAME** (a FlatBuffers string). **Expel and change-leader carry
   the target's CHARACTER ID** (u32 zero-extended into a u64). The client resolves the name to
   an id *itself*, out of the member list the server gave it, and refuses locally when the name
   is not a member. So `Store::character_id_by_name` **is needed - for invite, and only invite.**
   §3, §4. **[L]**
2. **`parse_request`'s `action` and `payload_tag` are correct for all seven actions**, not just
   the create it was written against, and the reason is structural: all five builders call
   **one** encoder, `FUN_1413bd0f0`, which writes the root table identically whatever the
   action. `tools/callers.py` reports exactly 5 call sites, 0 tail jmps, 0 data pointers - the
   five builders and nothing else. §2, §6. **[L]**
3. **`parse_request`'s `name` is right for tags 2 and 5 and silently wrong for tag 4.** Slot 0
   of the tag-4 payload is a **u64 id**, and the decoder reads it as a uoffset because it never
   looks at the tag first. For ids 1 and 4 it hands back `Some("")` instead of an error. This is
   `CLAUDE.md`'s *"look at the discriminator before you read the union"* in a new place. §6.
4. **`MAX_MEMBERS = 6` is now [L], on two independent legs**, and can come off `research/party.md`
   §5's [I] list. §5.
5. The union has **six** arms, not five. Tag 6 exists in the client's dispatch table and **no
   builder in the image writes it** (enumerated, not searched). §2.
6. **Correction to `research/party.md` §3 and `crates/net/src/party.rs`'s doc**: the `0x0183`
   op byte is `0x1B` only for the arm that answers `0x00A5` code `0x03`. The code `0x06` arm
   writes **`0x1C`** (`0x1413bb3a1`). §7. **[L]**

---

## 1. The instrument, and the control that makes the rest of this file evidence

`CLAUDE.md`: *"Verify the instrument before believing it."* Everything was run **from the repo
root**, and every throwaway script was piped in (`python - < s.py`) so `sys.path[0]` is empty
and the scratchpad's stale `reads.py` / `listing.py` / `callers.py` cannot shadow `tools/`.

| instrument | control | result |
|---|---|---|
| `python tools/reads.py 0x140304100 2` | the documented control | raw @`140304138`, u8 @`140304144`, u8 @`140304183`, then a run of u16 - **reproduced** |
| `python tools/listing.py 0x140304100` | must agree with the above at the same addresses | **reproduced**, same four, same addresses |
| `python tools/encodes.py 0x141cb6880 1` | its own docstring: the `0x2ff` ctor at `141cb7eb1` | **reproduced** |
| `python tools/callers.py 0x1413bdb80` | a function known to have callers | 2 sites, both the create/pickup wrappers - **reproduced** |
| `python tools/xref.py --va 0x14327f958` | must find `FUN_1411c9540` | 4 refs, `FUN_1411c9540` among them - **reproduced** |
| the UTF-16 literal scan | must find `create`/`invite`/`expel`/`leader` at the addresses `party.md` names | 4/4 at `0x14327f958`, `0x143286378`, `0x14338dc58`, `0x14338dbc8` - **reproduced** |
| `wz-dump cat UI_000.wz UserList.img` | `party.md`'s read | same `Party` panel, same buttons - **reproduced** |
| the archive sweep | `<- 0x02FF` over 100 000 distinct events | **166 547** over 530 files |

### And one control that is worth more than all of them

The whole of §2-§4 is a reading of an encoder written in assembly. A reading can be wrong in a
way that still sounds coherent, so it was made to **produce bytes and checked against the one
capture**:

> A Python model of `FUN_1413bd0f0` + `FUN_1413bedb0` + `FUN_1413be9f0` + `FUN_1413bebe0`,
> written only from the listings, **reproduces the archived 68-byte create byte for byte**,
> all 68 of them.

That is not a restatement of the capture - the model was given `action = 0`, `tag = 5`,
`name = "TestCharD's Party"`, `slot1 = 1`, `slot2 = 0` and had to derive the padding, the
alignment, the vtable contents, the vtable length, the soffsets and the field order itself.
Had any of the following been misread, the length would not have been 68:

* the slot-to-voffset mapping (`4 -> 0`, `6 -> 1`, `8 -> 2`),
* the **default of 1** on the tag-5 payload's slot 1 (a default of 0 would have emitted an
  extra byte and a longer vtable),
* the `max(max_voffset + 2, 4)` clamp on the vtable length (`0x1413bec3c add ax,2 / cmp ax,di /
  cmovb ax,di`),
* the `PreAlign(4, minalign)` in `Finish`.

Every body printed in §3 and §7 comes out of that validated model. **They are predictions, not
captures** - only the create has ever been on the wire - but they are predictions from a model
with a passing 68-byte control.

---

## 2. The root table is one shared code path, and that is why `action` and `tag` are safe

Five builders write `0x0182`. All five do the same three stores into a 0x18-byte struct and
then call the same encoder:

```text
        struct+0x00   u8   action
        struct+0x08   u8   union tag
        struct+0x10   ptr  payload object   (0 if the allocation failed)
```

```asm
1413bdb80  mov byte  [rsp+0x20], cl      ; action        <- FUN_1413bdb80, create / pickup
1413bdbaf  mov byte  [rsp+0x28], 5       ; tag
1413bdbef  mov qword [rsp+0x30], rbx     ; payload
1413bdbf4  mov edx, 0x182 / call 0x1406ed520     ; COutPacket(0x182)
1413bdc0e  call 0x1413bd0f0                     ; encode the whole body as one w_raw
1413bdc18  call 0x1415d01c0                     ; SendPacket
```

`python tools/callers.py 0x1413bd0f0` - **5 call sites in 5 functions, 0 tail jmps, 0 qword
pointers**, and they are exactly `FUN_1413b9eb0`, `FUN_1413ba250`, `FUN_1413ba710`,
`FUN_1413bda50`, `FUN_1413bdb80`. That is an enumeration of every producer of this opcode, not
a search of a list. **[L]**

### `FUN_1413bd0f0`, the root table

```asm
1413bd16d  movzx r14d, byte ptr [rdx]      ; action      = struct+0
1413bd171  lea   rcx, [rdx + 8]
1413bd175  movzx esi,  byte ptr [rcx]      ; tag         = struct+8
1413bd183  call  0x1413bedb0               ; serialize the union -> a uoffset, or 0
```

then three fields, in this write order (FlatBuffers builds downward, so the write order is not
the slot order):

| written | source | width | voffset | **slot** | omitted when |
|---|---|---|---|---|---|
| 1st | the payload's uoffset | 4 | `8` (`1413bd21c mov edi,8`) | **2** | the union returned 0 |
| 2nd | `tag` | 1 | `6` (`1413bd29a mov esi,6`) | **1** | `tag == 0` |
| 3rd | `action` | 1 | `4` (`r13 = 4`, `1413bd316`) | **0** | `action == 0` |

The omission tests are `test r14b,r14b / jne write / cmp byte[rbp+0x27], r14b / je skip`, and
`byte [rbp+0x27]` is the force-defaults flag, initialised to **0** by
`1413bd163 mov word ptr [rbp+0x27], 0x100`. So **every** field equal to its default is dropped.
**[L]**

**Consequence, and it is the one `CLAUDE.md` warns about**: `action` absent means `0 = CREATE`,
and `tag` absent means `0`, which is not a valid tag. A body with no tag is a body the server
must refuse, not one it should default.

### The union dispatcher `FUN_1413bedb0` has six arms

```asm
1413bedc4  movzx eax, byte ptr [rcx]     ; tag
1413bedca  dec   eax
1413bedcf  cmp   eax, 5
1413bedd2  ja    0x1413befa8             ; default: *out = 0, no payload at all
1413bede1  mov   r9d, dword ptr [rdx + rax*4 + 0x13befc8]
```

The table at `0x1413befc8`, read with `tools/dump_va.py`, is in tag order:

| tag | arm | payload shape |
|---|---|---|
| 1 | `1413bedef` | one `u8` at slot 0 |
| 2 | `1413bee1f` | `string` at slot 0, `u8` at slot 1 |
| 3 | `1413bee63` | one `i64` at slot 0 |
| 4 | `1413bef06` | one `u64` at slot 0 |
| 5 | `1413bef32` | `string` slot 0, `u8` slot 1, `u8` slot 2 |
| 6 | `1413bef7b` | one `u8` at slot 0, **force-defaults set** |

**Tag 6 is written by nothing.** The five callers enumerated above write tags 5, 1, 5, 2, 3 and
4 only. An arm with no producer is not a shape the server will ever see; it is recorded so that
a future reader does not go looking for a seventh builder. **[L]**

**Tag 0 takes the default arm**: `*out = 0`, so the root's slot 2 is omitted too. A body with
`tag = 0` is a body with no payload.

---

## 3. The seven actions, byte for byte

Action-to-tag comes from each builder's own two immediates, read directly - not from
`research/party.md`'s table:

| action | button | builder | `struct+0` | `struct+8` | payload allocation |
|---|---|---|---|---|---|
| **0** CREATE | `create` | `FUN_1413bdb80` via `FUN_1413b9bc0` | `cl` = 0 (`1413b9d17 xor ecx,ecx`) | **5** (`1413bdbaf`) | `0x28` bytes |
| **1** LEAVE | `leave` | `FUN_1413ba250` | **1** (`1413ba32c`) | **1** (`1413ba33a`) | `1` byte |
| **2** PICKUP | `pickup` | `FUN_1413bdb80` via `FUN_1413b9d90` | `cl` = 2 (`1413b9e0f`) | **5** | `0x28` bytes |
| **3** INVITE | `invite` | `FUN_1413b9eb0` | **3** (`1413ba06b`) | **2** (`1413ba075`) | `0x28` bytes |
| **4** JOINREQ | *(none)* | `FUN_1413ba710` | **4** (`1413baa6e`) | **3** (`1413baa77`) | `8` bytes |
| **5** EXPEL | `expel` | `FUN_1413bda50` via `FUN_1413ba590` | `cl` = 5 (`1413ba6b3`) | **4** (`1413bda7b`) | `8` bytes |
| **6** LEADER | `leader` | `FUN_1413bda50` via `FUN_1413ba470` | `cl` = 6 (`1413ba543`) | **4** | `8` bytes |

All **[L]**.

### Tag 1 - LEAVE. An empty table.

`FUN_1413ba250` allocates **one byte and stores 0 in it**:

```asm
1413ba33f  mov edx, 1 / call 0x14019b780
1413ba355  mov byte ptr [rax], 0
```

The tag-1 arm writes that byte at slot 0 with `r9d = 0` (no force), so a value of 0 is dropped.
**The payload table therefore has no fields at all** - vtable length 4, table size 4. The
payload is present but empty, always. **[L]**

| slot | width | meaning | default | absent means | in practice |
|---|---|---|---|---|---|
| 0 | u8 | unknown; the arm resolves no string | 0 | 0 | **always absent** |

### Tag 2 - INVITE. A string, and that is the whole point.

`FUN_1413b9eb0(rcx = name, edx = flag)`:

```asm
1413ba048  mov rdx, qword ptr [rdi]          ; rdi = the caller's name argument
1413ba04b..1413ba05a                          ; strlen
1413ba05c  call 0x1401d69c0                  ; assign(chars, len) into a temp std::string
1413ba066  mov byte ptr [rsp+0x60], r12b     ; temp+0x20 = the edx argument
1413ba06b  mov byte ptr [rsp+0x28], 3        ; action 3
1413ba075  mov byte ptr [rsp+0x30], 2        ; tag 2
```

and the tag-2 arm serialises it with `FUN_1413be880`, whose voffsets are `r12w = 4` for the
string (**slot 0**) and `ebx = 6` for the byte (**slot 1**, `1413be987 mov ebx, 6`).

| slot | width | meaning | default | absent means | in practice |
|---|---|---|---|---|---|
| 0 | string (u32 length, bytes, NUL) | **the invitee's character name** | - | no name given: **refuse** | always present |
| 1 | u8 | unknown flag | 0 | 0 | **always 1** - `1411cb021 mov edx, 1` in `FUN_1411cac30`, the only caller |

The name reaches the builder as a `char*`, is `strlen`'d, and is copied verbatim into the
buffer. It is a byte string with a `u32` length prefix and a NUL terminator, so ASCII names are
unambiguous; whether a non-ASCII name arrives as UTF-8 or as the client's code page is **[I]**
and untested. **[L]** for everything else.

### Tag 3 - JOIN REQUEST. One 64-bit value, and nothing reaches it.

```asm
1413ba749  movsxd r12, ecx                 ; SIGN-extend the i32 argument
1413baa7c  mov edx, 8 / call 0x14019b780
1413baa92  mov qword ptr [rax], r12
```

| slot | width | meaning | default | absent means |
|---|---|---|---|---|
| 0 | i64 (an i32 **sign**-extended) | **unknown** | 0 | 0 |

`python tools/callers.py 0x1413ba710` - **0 call sites, 0 tail jmps, 0 data pointers**, with the
tool's own control (`0x1413bdb80` -> 2 sites) passing. So nothing in the image reaches this
builder by any scan the tool can do, and the tool's docstring names its blind spot: a computed
or virtualised call leaves no trace. **What that i32 is, is not established. Do not build it.**
The function resolves `0x0123` *"You are already in a %s. Would you like to leave your existing
%s and ask to join a new %s?"*, which makes a party id or a target character id the obvious
candidates - **[I]**, and two candidates is not an answer.

### Tag 4 - EXPEL and CHANGE LEADER. A character id.

Both wrappers do the same three things, and the middle one is the answer to the whole
name-versus-id question:

```asm
; FUN_1413ba470, the "leader" button
1413ba4c9  mov rdx, rbx                    ; rbx = the clicked member's NAME
1413ba4d1  call 0x14019a260                ; copy it into a local
1413ba4db  call 0x1413b8ec0                ; NAME -> ID, locally
1413ba4e0  test eax, eax
1413ba4e2  jne  0x1413ba537                ; 0 = not a member -> string 0x0128, never sends
1413ba537  mov eax, eax                    ; ZERO-extend the u32 id to 64 bits
1413ba539  mov qword ptr [rsp+0x38], rax
1413ba543  mov cl, 6 / call 0x1413bda50
```

`FUN_1413ba590` (expel) is the same, `mov ebx, eax` then `mov cl, 5`. Both **[L]**.

| slot | width | meaning | default | absent means |
|---|---|---|---|---|
| 0 | u64, 8-byte aligned (a u32 **zero**-extended) | **the target's character id** | 0 | 0 = "no target": **refuse** |

The high dword is always zero. The field is written by `FUN_1413bcf50`, which aligns to 8
(`and edx, 7`), reserves 8 and stores `mov qword ptr [rax], rsi`.

**The id space is the server's own.** §5 shows the lookup returns the `u32` stored at
`member + 0` in the client's member array - the value the *server* put there via `0x00A5`
code `0x0E`. So whatever character ids this server sends in the party member list are the
character ids it will get back in an expel or a leadership handover. No name lookup is
involved on either side.

### Tag 5 - CREATE and PICK-UP RIGHTS. A string and two bytes, one with a non-zero default.

The tag-5 arm calls `FUN_1413be9f0`, whose voffsets are `r12w = 4` (string, **slot 0**),
`r13w = 8` (**slot 2**, `1413bea15 mov r13d, 8`) and `ebx = 6` (**slot 1**, `1413beb6e`).

| slot | width | meaning | **default** | absent means | in practice |
|---|---|---|---|---|---|
| 0 | string | **the party name** (create); empty for pickup | - | no name | create: present. pickup: absent |
| 1 | u8 | pick-up rights **[I]** | **1** | **1, not 0** | create: always absent (= 1) |
| 2 | u8 | pick-up rights **[I]** | 0 | 0 | create: always absent |

The default of **1** on slot 1 is the one field in this document that a careless decoder will
get wrong, and it is read straight off the listing:

```asm
1413beb42  cmp r15b, 1                     ; the slot-1 value
1413beb46  jne 0x1413beb4e                 ; != 1 -> write it
1413beb48  cmp byte ptr [rdi+0x70], 0      ; force_defaults?
1413beb4c  je  0x1413bebb1                 ; == 1 and not forced -> SKIP
```

Both create and pickup fill the payload through one helper, `FUN_1406f25f0`, which copies a
`char*` name plus `src+8` and `src+9` into `payload+0x20` and `payload+0x21`. The party
window's create route sets those two bytes with a single store:

```asm
1411c9599  mov word ptr [rbp - 0x10], 1    ; src+8 = 1, src+9 = 0
```

so a create from the Create button always has slot 1 = 1 and slot 2 = 0, and **both are
therefore always absent from the wire**. That is exactly the archived capture's payload vtable:
length 6, slot 0 only. Three independent reads agreeing on one 68-byte body. **[L]**

The meaning of the two bytes is **[I]**: `0x011A` *"The party's item pick-up rights changed to
%s."*, `0x011B` *"The party's current item pick-up rights is %s."* and `0x011E` *"Pick-Up
Rights:"* say the feature exists and that action 2 sets it, but nothing here establishes which
byte is which or what values they take.

### The seven bodies

From the validated model. Only the first has ever been on the wire.

```text
action 0 CREATE   tag 5   68 B  1000000000000a000e000000070008000a0000000000000
                                50c00000000000600080004000600000004000000110000
                                005465737443686172442773205061727479000000
                                                  (name "TestCharD's Party")
action 1 LEAVE    tag 1   36 B  1000000000000a000c000600070008000a0000000000010
                                108000000040004000 4000000
action 2 PICKUP   tag 5   44 B  1000000000000a000c000600070008000a0000000000020
                                50c00000008000800000007000800000000000002
                                                  (empty name, slot1=2, slot2=0)
action 3 INVITE   tag 2   56 B  1000000000000a000c000600070008000a0000000000030
                                20c00000008000c000800070008000000000000010400000
                                0030000004f776c00 (name "Owl", slot1=1)
action 4 JOINREQ  tag 3   48 B  1000000000000a000e000600070008000a0000000000040
                                30c000000000006000c00040006000000d2040000000000
                                00                (value 1234)
action 5 EXPEL    tag 4   48 B  1000000000000a000e000600070008000a0000000000050
                                40c000000000006000c00040006000000c8000000000000
                                00                (character id 200)
action 6 LEADER   tag 4   48 B  1000000000000a000e000600070008000a0000000000060
                                40c000000000006000c00040006000000c9000000000000
                                00                (character id 201)
```

---

## 4. Name or id: the answer, and why it is not the same answer for every button

| action | what it names the target by | why |
|---|---|---|
| invite | **name** (string) | the invitee is not in the party, so the client has no id for them |
| expel | **character id** | the client looks the name up in its own member list first |
| change leader | **character id** | the same lookup, the same refusal on a miss |

**So keep `Store::character_id_by_name`.** It is needed for invite, which is the one action
whose target is by definition not yet known to either side by id. It is *not* needed for expel
or change-leader, and using it there would be wrong twice over: it would re-resolve a name the
client never sent, and it would accept a target the client has already proved is not a member.

---

## 5. `MAX_MEMBERS = 6` is [L] now, on two independent legs

`research/party.md` §5 marked this **[I]** and named the blind spot precisely: *"that is two
places, not an enumeration... What would settle it: enumerate every immediate compared against a
member count in that range."* Both legs below are that immediate.

**Leg 1 - the array is fixed at six slots.** `FUN_1406f1ff0`, the name-to-id lookup:

```asm
1406f1ff5  lea r10, [rcx + 8]              ; members start at party+8
1406f1ff9  lea r11, [r10 + 0x420]          ; and end 0x420 bytes later
1406f2010  lea rcx, [r10 + 4]              ; the member's NAME is at member+4
           ... strcmp ...
1406f204a  add r10, 0xb0                   ; stride 0xB0
1406f2056  xor eax, eax                    ; not found -> 0
1406f205e  mov eax, dword ptr [r10]        ; found -> the u32 at member+0
```

`0x420 / 0xB0 = 6` exactly. **[L]** And the same walk gives the member record's first two
fields: **`u32 id` at +0, name at +4, 0xB0 bytes per member.**

**Leg 2 - the client refuses a seventh invite itself.** `FUN_1413b9eb0`:

```asm
1413b9fb0  lea rcx, [rip + 0x2712751]      ; -> 0x143ACC708, the party object
1413b9fb7  call 0x1406f2070                ; count members
1413b9fbc  cmp eax, 6
1413b9fbf  jl  0x1413b9fe1                 ; fewer than 6 -> proceed
1413b9fc1  mov edx, 0x129                  ; otherwise refuse locally
```

and `FUN_1406f2070` walks the identical array (`+8`, span `0x420`, stride `0xB0`) counting
slots whose id is non-zero. Two functions, two derivations, same object at `0x143ACC708`, same
number. **[L]**

A corollary worth having: **a member slot is occupied iff its id is non-zero**, so id 0 is not a
usable character id anywhere in the party system - which is also why `0` is a safe "absent"
default for the tag-4 field.

---

## 6. What `parse_request` gets right, what it gets wrong

The seven predicted bodies were run through a **line-for-line Python port of the current
`crates/net/src/party.rs::parse_request`**, with the archived capture as the port's control
(it decodes to exactly what the Rust test asserts).

```text
case             len    parse_request() today
0 CREATE  tag5   68     OK   action 0, tag 5, name "TestCharD's Party"
1 LEAVE   tag1   36     OK   action 1, tag 1, name None
2 PICKUP  tag5   44     OK   action 2, tag 5, name None
3 INVITE  tag2   56     OK   action 3, tag 2, name "Owl"      <-- already works
4 JOINREQ tag3   48     OK   action 4, tag 3, name None
5 EXPEL   tag4   48     OK   action 5, tag 4, name None
6 LEADER  tag4   48     OK   action 6, tag 4, name None
```

**Right:** `action` and `payload_tag` for all seven, and none of the seven returns `None`. That
is not luck - §2 shows the root table is one code path.

**Better than its own doc says:** the doc block says the string is *"`None` for every other
shape"*. It is not. Tag 2's slot 0 is a string uoffset exactly as tag 5's is, so
**`parse_request` already recovers the invite target's name**.

**Wrong, and silently:** slot 0 of a **tag-4** payload is a `u64` id, and `table_uoffset` reads
it as a uoffset because nothing makes the decoder look at the tag first. Sweeping the id:

```text
  character id 1  ->  name Some("")     <== invented
  character id 4  ->  name Some("")     <== invented
  every other id  ->  name None
```

It cannot panic and it will not invent a name that matches a real character, so it is not
dangerous *today*. It is dangerous the moment somebody writes `if let Some(name) = req.name`
and reaches the invite branch with an expel packet. `CLAUDE.md` already carries this exact
failure - `_HEAP_FAILURE_INFORMATION.Address` decoded as an entry when the type said pointer,
which *"always produces a plausible-looking header"*.

### The replacement

```rust
/// The payload of a [`CLIENT_PARTY_REQUEST`], decoded according to its union tag.
///
/// The tag is the discriminator and is read BEFORE the payload - a FlatBuffers union
/// member decoded under the wrong tag does not error, it returns something plausible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartyPayload {
    /// Tag 1, [`action::LEAVE`]. One `u8` at slot 0, default 0; the client allocates a
    /// single byte and stores 0 in it, so the field is **always** absent. **[L]**
    Leave { flag: u8 },
    /// Tag 2, [`action::INVITE`]. The invitee's **name** at slot 0 and a `u8` at slot 1
    /// (default 0; the window's only call site passes 1). `name` is `None` when the
    /// slot is absent, which the world must treat as "no target" and refuse. **[L]**
    Invite { name: Option<String>, flag: u8 },
    /// Tag 3, [`action::JOIN_REQUEST`]. One `i64` at slot 0 (an `i32` sign-extended),
    /// default 0. **What the value means is NOT established** and the builder has zero
    /// callers by all three of `tools/callers.py`'s scans. Do not act on it.
    JoinRequest { value: i64 },
    /// Tag 4, [`action::EXPEL`] and [`action::CHANGE_LEADER`]. The target's **character
    /// id** at slot 0: a `u32` zero-extended to `u64`, 8-byte aligned, default 0. The
    /// client resolved the name to this id itself, out of the member list this server
    /// sent it, so **no name lookup belongs on this path**. `0` means the field was
    /// absent - refuse. **[L]**
    Target { character_id: u64 },
    /// Tag 5, [`action::CREATE`] and [`action::SET_PICKUP_RIGHTS`]. The party name at
    /// slot 0, then two `u8`s whose meaning is [I] (pick-up rights).
    ///
    /// **`rights_a` defaults to 1, not 0.** An absent slot 1 means 1. The create button
    /// always sends 1 and 0, so both are always absent from a real create - which is
    /// why the archived capture's payload vtable carries only slot 0.
    Named { name: Option<String>, rights_a: u8, rights_b: u8 },
    /// Tag 0, tag 6, or a tag whose payload table this decoder could not follow. The
    /// action and the tag are still trustworthy; nothing else is.
    Absent,
}

pub struct PartyRequest {
    pub action: u8,
    pub payload_tag: u8,
    pub payload: PartyPayload,
}

impl PartyRequest {
    /// The name this request names a target by, if it names one by name at all.
    /// **`None` for expel and change-leader, which name their target by id** - see
    /// [`PartyPayload::Target`].
    pub fn target_name(&self) -> Option<&str> { ... }
    /// The character id this request names a target by, if any.
    pub fn target_id(&self) -> Option<u64> { ... }
}

pub fn parse_request(body: &[u8]) -> Option<PartyRequest> {
    let root = read_u32(body, 0)? as usize;
    let (table, vtable) = table_at(body, root)?;
    let action = table_u8(body, table, vtable, 0)?.unwrap_or(0);
    let payload_tag = table_u8(body, table, vtable, 1)?.unwrap_or(0);

    // The payload is best effort. A body whose action and tag read cleanly is worth
    // acting on, and a payload this decoder cannot follow must never turn into "no
    // request" - CLAUDE.md's "always answer" starts with being able to answer at all.
    let payload = (|| {
        let off = table_uoffset(body, table, vtable, 2).ok()??;
        let (pt, pvt) = table_at(body, off)?;
        Some(match payload_tag {
            1 => PartyPayload::Leave { flag: table_u8(body, pt, pvt, 0)?.unwrap_or(0) },
            2 => PartyPayload::Invite {
                name: table_uoffset(body, pt, pvt, 0)?.and_then(|s| read_string(body, s)),
                flag: table_u8(body, pt, pvt, 1)?.unwrap_or(0),
            },
            3 => PartyPayload::JoinRequest {
                value: table_i64(body, pt, pvt, 0)?.unwrap_or(0),
            },
            4 => PartyPayload::Target {
                character_id: table_u64(body, pt, pvt, 0)?.unwrap_or(0),
            },
            5 => PartyPayload::Named {
                name: table_uoffset(body, pt, pvt, 0)?.and_then(|s| read_string(body, s)),
                rights_a: table_u8(body, pt, pvt, 1)?.unwrap_or(1),   // DEFAULT 1
                rights_b: table_u8(body, pt, pvt, 2)?.unwrap_or(0),
            },
            _ => PartyPayload::Absent,
        })
    })().unwrap_or(PartyPayload::Absent);

    Some(PartyRequest { action, payload_tag, payload })
}
```

Two properties worth asserting in tests rather than describing here:

* **the action/tag pairs are fixed**, so a mismatched pair is a body no builder in this client
  produces and the world should refuse it: `(0,5) (1,1) (2,5) (3,2) (4,3) (5,4) (6,4)`;
* **`unwrap_or(1)` on slot 1 of tag 5** - the one default in the whole schema that is not zero.
  A test that builds a tag-5 payload with slot 1 absent and asserts `rights_a == 1` is the only
  thing that will stop somebody "tidying" it to `unwrap_or(0)`.

`tools/` has no FlatBuffers writer; the model used for §3 and §7 is 130 lines of Python and is
worth committing beside these tests if they need fixtures.

---

## 7. `0x0183 CLIENT_PARTY_INVITE_ANSWER`

A **different schema** from `0x0182`: a flat root table with no union. `FUN_1413bd500`, whose
`tools/encodes.py` output is one `w_raw` exactly like the other encoder.

```asm
1413bd583  movzx esi, byte ptr [rdx]       ; struct+0 = op
1413bd586  movzx ebx, byte ptr [rdx + 1]   ; struct+1 = answer
1413bd58a  mov   rdi, qword ptr [rdx + 8]  ; struct+8 = a 64-bit value
```

| slot | voffset | width | meaning | default | absent means |
|---|---|---|---|---|---|
| **0** | 4 | u8 | **op** - which notification is being answered: `0x1B` or `0x1C` | 0 | 0, which is not a valid op: refuse |
| **1** | 6 | u8 | **answer** - see below | 0 | 0, which the client never sends: refuse |
| **2** | 8 | u64, 8-aligned | **the token the server chose**, echoed back | 0 | 0: refuse |

All **[L]**, and the write order is slot 2, slot 1, slot 0, exactly as in `0x0182`.

### There is no party id in it - there is an echo, and the server picks it

Slot 2 is **not** independently meaningful. It is the **second `u32` of the `0x00A5` body** the
client is answering, zero-extended:

```asm
; the 0x00A5 code-0x03 arm
1413baf8e  call 0x1406e8c20   <<< READ u32      ; field 2 of the notification
1413baf93  mov r13d, eax                        ; zero-extends into r13
   ...
1413bb0b4  mov byte  ptr [rbp-0x68], 0x1b       ; op
1413bb0b8  mov byte  ptr [rbp-0x67], dil        ; answer
1413bb0bc  mov qword ptr [rbp-0x60], r13        ; the echo
```

So **whatever this server puts in field 2 of `0x00A5` code `0x03` comes back verbatim.** The
server should put something it can look the pending invite up by - the inviter's character id
is the natural choice - rather than trying to guess a party id out of the answer. The dialog
path carries the same value through `dialog+0x328`.

### The op byte says which notification, and `party.md` has it half right

| `0x00A5` code | dialog type (`dialog+0x300`) | `0x0183` op |
|---|---|---|
| `0x03` | `0x19` | **`0x1B`** (`1413bb0b4`) |
| `0x06` | `0x12` | **`0x1C`** (`1413bb3a1`, and `FUN_14180f2d0` sets type `0x12`) |

`research/party.md` §3 and `crates/net/src/party.rs`'s `CLIENT_PARTY_INVITE_ANSWER` doc say the
op byte *is* `0x1B`. That is the `0x03` arm only. **[L]**

The mapping was checked from both ends: the two dialog dispatchers' jump tables
(`0x14180c64c`, base type 4; `0x14180cb8c`, base type 6) send op `0x1C` from their type-`0x12`
arm and op `0x1B` from their type-`0x19` arm, in both dispatchers. Four sites, one rule.

### The answer byte

| value | where it comes from | meaning |
|---|---|---|
| `1` | `1413bb131`, before any dialog | refused: the notification names me, or a local pre-check failed |
| `2` | `1413bb127` / `1413bb383` | refused by the second `FUN_142d98230` check |
| `3` | `1413bb098` / `1413bb360` | refused by the first `FUN_142d98230` check |
| `4` | `FUN_14180c6e0`, control id **2001** | the dialog's **second** button |
| `5` | `FUN_14180b750`, control id **2000** | the dialog's **first** button |
| `0` | never sent | the value that means "open the dialog and ask" - and it is the schema default, so it could not be sent even if it were |

1, 2 and 3 are automatic: the client sends them without showing anything, and only
`answer == 0` reaches the dialog constructor (`1413bb0dc test edi,edi / jne skip`).

**Which of 4 and 5 is accept is [D], not [L].** Both come from one dispatcher:

```asm
; FUN_1418091b0(rcx = dialog, edx = control id)
1418091fc  mov eax, edi
141809208  sub eax, 0x7d0                  ; 0x7D0 = 2000
14180920d  je  0x141809240                 ; 2000 -> FUN_14180b750, answer 5
14180920f  cmp eax, 1
141809212  je  0x141809225                 ; 2001 -> FUN_14180c6e0, answer 4
```

Two things point the same way and neither is a measurement:

* **2000 is this client's id for a lone confirm button.** `UI_000.wz/UtilDlgEx.img`'s
  `UtilDlgEx_AvatarResult` panel has exactly one button, `button:confirm`, and it carries
  `"id": 2000`. **[L]**
* the 2000 path stores the handler's **bool return** into `dialog+0x2d8`; the 2001 path writes a
  qword 1 over `dialog+0x2d4`, which zeroes it. An affirmative action that can fail and report
  it; a cancel that cannot. **[L]**

**Two blind spots, both named, and the first one killed a third argument I had written here.**

* The obvious reading - "`BtOK` is listed before `BtCancel`, so it gets the lower id" - **does
  not survive checking, and the check is one command.** In `UtilDlgEx.img` the `BtOK` and
  `BtCancel` nodes carry **no `id` at all**; they are four canvases each (normal, pressed,
  disabled, mouseOver) and nothing else. And in `UserList.img/Party`, `2000` is the *panel's*
  own `"ID"` while its buttons run `2001`..`2006` - so 2000 is not "the first button" as a rule.
  The node-order argument was written down, checked, and is wrong.
* `tools/xref.py --va` on the `BtOK` literal (`0x1432b1e30`) returns **0 references**, and the
  tool's own docstring says that means "nothing takes its address" - this client copies short
  literals inline. The control passes on the same run (`--va 0x14327f958` finds
  `FUN_1411c9540`), so the search works and simply cannot see this. The dialog's button ids are
  assigned in code that this instrument is structurally blind to.

**Do not hard-code it.** It costs one line to log both and one client run - which the invite
feature needs anyway - to settle: send `0x00A5` code `0x03`, and see which byte comes back when
The owner presses each button. Until then, treat 4 and 5 as "a dialog answer, polarity unknown" and
refuse both rather than adding a member who declined.

### The bodies

```text
op 0x1B answer 5 value 7777 -> 32 B  1000000000000a0010000600070008000a00000000001b05611e000000000000
op 0x1B answer 4 value 7777 -> 32 B  1000000000000a0010000600070008000a00000000001b04611e000000000000
op 0x1C answer 5 value 7777 -> 32 B  1000000000000a0010000600070008000a00000000001c05611e000000000000
op 0x1C answer 4 value 7777 -> 32 B  1000000000000a0010000600070008000a00000000001c04611e000000000000
```

`0x0183` needs its own decoder - `parse_request` cannot be pointed at it, because slot 2 here is
an inline `u64` and there slot 2 is a uoffset to a table.

---

## 8. The archive still holds one party packet, and the brief that sent me here said otherwise

The task said `research/fixtures/` and `previous-runs/` *"now contain runs where two clients
shared a map and the owner pressed Create"*. **They do not.** Event-deduplicated on
`(timestamp, direction, opcode, first 120 bytes)` per `CLAUDE.md`, over **530** files
(287 in `previous-runs/`, 243 in `research/fixtures/`):

```text
control  <- 0x02FF   166 547 distinct events
0x00A5   ->   0   <-   0
0x0182   ->   0   <-   1      the same 2026-08-28 create, 16:48:31.299
0x0183   ->   0   <-   0
```

The single `0x0182` is the one `research/party.md` already had. A raw `grep` for the literal
`0x0182` and `0x0183` across both directories finds three and one occurrence respectively, and
the extras are `research/fixtures/sweep-0024-01c3-reply-0171.log`'s outbound opcode sweep
(`>>> opcode 0x0182 +32B`), not party traffic.

**78 files have been added since `research/party.md` counted 452, and none of them contains a
party packet.** Said the other way: everything in §3 and §7 except the create is a prediction
from static analysis with a validated model, and nothing here has been on the wire.

---

## 9. What I did NOT establish

1. **Six of the seven bodies have never been observed.** The model that produced them passes a
   68-byte control on the seventh, which is the strongest check available without a client run,
   and it is still not a capture.
2. **Tag 3's value is unknown**, and its builder is unreachable by all three `callers.py` scans.
   Two candidates (party id, character id) is not an answer. **Do not build it.**
3. **Tag 5's slots 1 and 2 have no established meaning.** Their widths, defaults and slots are
   [L]; "pick-up rights" is [I] from three strings.
4. **Tag 2's slot 1 and tag 1's slot 0 have no established meaning.** Both are [L] as fields.
5. **`0x0183`'s accept-versus-decline polarity is [D]** with the blind spot named in §7. This is
   the one thing in this document that would cause a wrong *action* rather than a refusal, and
   it is the one thing a single client run settles for free.
6. **Non-ASCII names are untested.** The field is a length-prefixed byte string; the encoding of
   a name outside ASCII is [I].
7. **The union has a sixth arm nobody writes.** If a control this scan cannot see (a virtual
   call, a computed target) reaches a tag-6 producer, the shape is one forced `u8` at slot 0 -
   recorded so that it is not mistaken for a decode failure.
8. **Everything here is the client's *outbound* schema.** What the server must put in `0x00A5`
   to make any of it happen is `research/party-result-0x00A5.md`'s subject, not this file's.

---

## 10. Reproducing every number

All from the repo root; throwaway scripts piped in (`python - < s.py`).

```
python tools/reads.py    0x140304100 2      # the control - run this FIRST
python tools/listing.py  0x1413bd0f0        # the 0x0182 root table, all three fields
python tools/listing.py  0x1413bedb0        # the union dispatcher, six arms
python tools/dump_va.py  0x1413befc8 24     # the union jump table, in tag order
python tools/listing.py  0x1413be880        # tag 2: string slot 0, u8 slot 1 (ebx=6)
python tools/listing.py  0x1413be9f0        # tag 5: slot 0, slot 2 (r13=8), slot 1 (ebx=6, DEFAULT 1)
python tools/listing.py  0x1413bcf50        # the 8-byte writer used by tag 4
python tools/listing.py  0x1413bebe0        # EndTable; the max(max_voffset+2, 4) clamp at 1413bec3c
python tools/listing.py  0x1413bd500        # the 0x0183 root table
python tools/callers.py  0x1413bd0f0        # 5 call sites - every 0x0182 builder, enumerated
python tools/callers.py  0x1413ba710        # 0 by all three scans (control: 0x1413bdb80 -> 2)
python tools/listing.py  0x1413ba470        # leader: name -> id, then mov eax,eax
python tools/listing.py  0x1413ba590        # expel: the same
python tools/listing.py  0x1406f1ff0        # the member array: +8, 0x420, stride 0xB0, id at +0
python tools/listing.py  0x1411c9540        # the button dispatcher; create's "mov word,1"
python tools/listing.py  0x1418091b0        # control 2000 -> answer 5, control 2001 -> answer 4
python tools/xref.py     --va 0x14327f958   # control: must find FUN_1411c9540

target\release\wz-dump.exe cat client-patched\Data\UI\UI_000.wz UtilDlgEx.img
```

`0x1406f2070` has **no `.pdata` entry** (`tools/pdata_lookup.py` says so) - `listing.py` will
refuse it. Disassemble it directly with `tools/rtti.py`'s loader and capstone; it is 14
instructions.

The encoder model, the seven bodies and the `parse_request` port are 300 lines of Python
total, all reproducible from the listings above. The single thing to keep from them is the
control: **the model must emit the archived 68 bytes exactly**, or nothing it emits is worth
reading.
