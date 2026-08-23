# The Cash Shop: the client never asked, and that is the whole finding

2026-08-22. The owner: *"I tried entering Cash Shop but was unfortunately not able to because the
opcode is most likely not handled. Transitioning to the cash shop is most likely similar to
transitioning to another channel, we probably need a dedicated cash shop server."*

Tags: **[L]** read out of a file, **[D]** derived from something read, **[I]** inferred.

---

## The short version

**No packet was sent.** `world.log` records every inbound packet on the channel connection and
there is nothing new in it - the unanswered opcodes that run are the same telemetry set every
session produces (`0x01A5`, `0x00E5`, `0x02F5`, `0x013D`, `0x00B8`, `0x0226`, `0x01ED`,
`0x02EB`, `0x0420`..`0x0426`, `0x0408`), all of which appear in runs where nobody went near
the Cash Shop. The client hand-closed with exit code **0**. `[L]`

So **there is no unhandled opcode.** The click never became a request, and a cash-shop server
built today would sit waiting for a connection the client has no reason to make.

**The owner's architecture is very likely right about the step after this one.** In this game
family the cash shop *is* a separate server reached by a migrate, and this project already has
every piece of that: `0x0011 MIGRATE_COMMAND`, minted single-use seeds, claim-by-channel, and
a second listener. That is not the blocker. The blocker is one gate inside the client.

---

## 1. What was ruled out, cheaply

**It is not missing art.** This is a trimmed client and `research/storage.md` §10 records a
whole UI class - `UI/UIWindow2.img/Trunk` - that cannot draw because its image was cut. That
is not the case here: `[L]`

```text
UI_000.wz          CashShopUI.img, CashShopPreview.img      both present
StatusBar.img      /button:CashShop                          present
StatusBar3.img     /mainBar/menu/button:CashShop             present
```

**It is not that the client cannot ask.** `tools/xref.py --string CashShop --callers` finds
three code references, and one of them is small enough to read: **`FUN_1411ab7b0`**, 904
bytes, referencing the string at `0x14338bd98`. `tools/encodes.py` at depth 3 shows it reaches
two packet builders: `[L]`

```text
0x1411ab8f4  call 0x142caee70 -> CTOR w_u32 w_str SEND w_u8
0x1411abb26  call 0x142cafbd0 -> CTOR w_u32 SEND
```

Both are **`gated?`** - they sit after a conditional. So the path from the button to a send
exists and was not taken.

**The three UI classes send nothing themselves.** `FUN_1410d5170` (3536 bytes),
`FUN_1410d76f0` (1573) and `FUN_142679640` (4053) all reference `UI/CashShopUI.img` and
`encodes.py` at depth 3 finds **no packet builder in any of them**. `[L]` They are window
construction; the request comes from elsewhere.

---

## 2. What that leaves: a gate, and this client has form

A button whose handler exists, whose art exists, and which produces no packet is the shape of
the **create-character flag** (`crates/grap-stub/src/session.rs`): a protected byte with two
setters, the enable one reachable **only from the Themida VM**, zeroed by the handshake on
every login. Nothing was wrong with the button; the flag behind it was clear, and the client
simply never called the sender. `[D]` that this is the same shape; **[I]** that it is the same
mechanism.

---

## 3. The question I cannot answer, and it costs the owner one sentence

Three things the screen could have done, and they need three different jobs. `CLAUDE.md`'s
rule applies exactly: they can see the GUI and I cannot.

| what the screen did | what it means | the work |
|---|---|---|
| **nothing at all** - the button does not even depress | a gate before the handler, the create-character pattern | find the flag; the hook already patches one like it |
| **a dialog or a chat line** - "cannot be used", "try again later" | there is a **string**, and a string is one `xref.py` away from the exact branch that refused | one command, then the gate is named |
| **a window opens and is blank or closes itself** | the handler ran and the *request* is the missing part | now it is a protocol job, and the owner's migrate reading is the thing to build |

Until that is known, hunting the gate statically is answering *"why is this button different"*
before anyone has established that it is - the mistake `CLAUDE.md` records under "The thing
you are comparing against may never have been a control", which cost three days once.

---

## 4. If it turns out to be the third row, here is the shape

Recorded now so it is not re-derived, and **not built**, because nothing yet needs it. `[I]`
throughout - this is the reference tree's architecture and this project has learned to label
that.

* the client sends a migrate-to-cash-shop request on the **channel** socket;
* the server answers with the same `0x0011` this project already sends for a channel change,
  carrying the cash shop's address and a **single-use seed** - `crates/world`'s migration
  table needs a third destination kind beside world and channel;
* the client disconnects and connects to that address, sends its `0x007D` migration hello,
  and the new server answers with **`SetCashShop`** rather than `SetField`. That packet is
  `0x01A1`..`0x01AA` by `research/msexe-gamestage-opcodes.md`'s block reading, which is a
  *candidate range*, not a read;
* leaving reverses it.

The parts this project already owns: the second listener, the seed minting, the claim, the
`0x0011` builder, and the channel-teardown behaviour that took three passes to get right
(`research/channel-select.md`). The parts it does not: `SetCashShop`'s body, the cash
inventory, a wallet, and the purchase flow.


---

# Part two: five deliberate clicks, five packets, none of them new

2026-08-22, later. The owner: *"I also clicked on the Cash Shop button 5 times before exiting the
game."* Deliberately, so the log would carry it. It does not.

## The measurement, tightened

Between the storage window closing at `02:05:00.548` and the shutdown telemetry at
`02:05:24.873` - the twenty-four seconds containing all five clicks - the client sent
**exactly five packets**: `[L]`

```text
02:05:01.898  0x02F4   12 bytes   telemetry, every session
02:05:06.189  0x00B8    1 byte    telemetry, every session
02:05:20.982  0x013D    9 bytes   the skill send-counter census
02:05:20.982  0x0070   46 bytes   env report
02:05:24.553  0x00B8    1 byte    telemetry
```

Five packets and five clicks is a coincidence of counts: none of these is new, none is
click-shaped, and `0x013D`+`0x0070` arrive together on a timer. **Nothing the client sent can
be attributed to a Cash Shop click**, across two sessions now.

## The handler has no gate, which changes what to look for

`FUN_1411ab7b0` turns out to be a **button-name dispatcher**: a chain of
`lea rdx,<name> ; call 0x142aa1a20 ; test al,al ; je <next>` comparisons. The Cash Shop arm is
`[L]`:

```asm
1411ab8c6  mov  r8d, edi
1411ab8c9  lea  rdx, [rip+0x21e04c8]   ; -> 0x14338bd98, "CashShop"
1411ab8d3  call 0x142aa1a20            ; name compare
1411ab8da  je   0x1411ab8f9            ; not it -> next name
1411ab8dc  xor  r8d, r8d
1411ab8df  xor  edx, edx
1411ab8e1  mov  rcx, [rip+0x28fcbb8]   ; -> 0x143AA84A0
1411ab8f4  jmp  0x142caee70            ; TAIL CALL the sender
```

**There is no condition on that path.** Match the name, load a global, jump to the sender. The
only test in the whole function is a null check on the *same* global at the top
(`0x1411ab7c1`), and if that were null every button in the chain would be dead, not just this
one. **[D]**

So the earlier reading - *"a gate before the handler, the create-character pattern"* - is now
the **less** likely of the three. If the click reached this dispatcher, a packet would have
gone out.

## Which moves the suspicion to the button itself

`0x142caee70` is a shared sender: `research/msexe-send-opcodes.txt` records its CTOR at
`0x142caf180` as opcode **`0x00D5`**, and exactly one `0x00D5` arrives per session - in the
shutdown batch beside `0x0420`..`0x0426`, in this run and the one before. `[L]` So other arms
of this same dispatcher do reach the wire; the Cash Shop arm did not.

**The remaining reading is that the click never became a button event at all** - the control
is drawn but not hooked, or disabled, in the status bar. That is a different kind of bug from
a protocol gap and it is not something a server change can reach.

## The observation that would settle it, still outstanding

One sentence, and it is now a sharper question than yesterday's:

> **Does the button react to the click at all - depress, highlight, make a sound?**

| | what it means |
|---|---|
| **no reaction whatever** | the control is inert. `FUN_1411ab7b0` is never entered, and the work is in the status-bar UI, not the protocol |
| **it depresses, then nothing** | it *is* entered, so the global at `0x143AA84A0` is null or `0x142caee70` bails - both findable, and both one watch away |
| **any words on screen** | quote them; a string is one `xref.py` from the branch that refused |

Nothing here is worth building until that is known. The migrate architecture in §4 stands and
is still unbuilt for the same reason: **the client has not asked for anything.**


---

# Part three: the button is live, and both of my readings are dead

2026-08-22. The owner: *"The button can be highlighted, does depress when clicked, and does make a
click sound."* And two questions worth more than the answer I had: *"Does the server need to
advertise that the cash shop is available?"* and *"Since channels have to be advertised with
their IP, I assume cash shop would have to be too."*

## The control is live, so the two candidates I named are both wrong

* **Not the null global.** `tools/dataref.py 0x143AA84A0` returns **5459** RIP-relative
  references. `[L]` That is the context singleton, not a cash-shop flag - and if it were null,
  every one of the dispatcher's nineteen buttons would be dead, not one.
* **Not a gate in the handler.** The CashShop arm has no condition on it; part two has the
  listing.

## The dispatcher is the status bar, and that is read rather than assumed

Every `lea rdx,<name> ; call 0x142aa1a20` in `FUN_1411ab7b0`, decoded as UTF-16: `[L]`

```text
ChatLogMin  ChatLogMax  ChatPrev  ChatNext  ChatTargetSelect  CashShop  Menu
Shortcut  Claim  Mailbox  Equip  Inven  Stat  StatUp  Skill  SkillUp  Key
QuickSlot  QuickSlotD
```

Nineteen arms, and they are the status bar's buttons. **`Inven` is in the same chain as
`CashShop`, and Inven works** - which turns it into a positive control that costs one extra
click: the watch below must log a line for Inven before its silence on CashShop means
anything.

## On the two questions, and the second one has a correction in it

**"Does the server need to advertise that the cash shop is available?"** Very possibly, and
there is a real precedent in this project rather than a guess: the world list carries a
**per-channel enable byte**, and `research/channel-select.md` records a run where getting it
wrong emptied the Change Channel dialog completely. An account- or world-level "cash shop
available" flag would be exactly that shape. **[I]**

But it cannot be what silences *this* click. A flag the client checks would have to be checked
somewhere, and the only conditional on the path is the context null test.

**"Since channels have to be advertised with their IP, I assume cash shop would have to be
too."** **Channels are not.** `crates/net/src/opcode.rs`'s `world_list_entry` writes, per
channel: a name, a `u32` user count, and four bytes `[world, index, 0, CHANNEL_ENABLED]`.
**No address.** `[L]` The client learns a channel's address only from the `0x0011`
MIGRATE_COMMAND, *after* it asks to change channel.

So a cash shop address would arrive the same way - in a migrate reply - which means **it
cannot be a precondition for the client asking.** The analogy holds for the architecture in §4
and points the opposite way for the gate.

## The one run that splits it three ways

Two watches, armed in the launcher's default `-SetFieldProbe` set (they displaced
`141c532ab`, a regression check on mob spawning that has rendered correctly for days):

```text
1411ab7b0:hits=60    the status-bar name dispatcher
142caee70:hits=20    the sender its CashShop arm tail-jumps to
```

**Click `Inven` first.** Then Cash Shop, two or three times.

| hook log shows | what it means |
|---|---|
| a line for Inven, none for Cash Shop | the click never reaches the dispatcher - a status-bar UI problem, and no server change touches it |
| `1411ab7b0` for Cash Shop, nothing from `142caee70` | the dispatcher runs and the name never matches. `rdx` on entry is the button index; comparing it with Inven's names the mismatch |
| both | the sender refuses, and it is 1012 bytes to read |
| nothing at all, not even Inven | the hook did not arm and the run proves nothing |


---

# Part four: which status bar, and why the control button matters

The owner: *"More specifically, I don't know what Inven is."* A fair question, and chasing it turned
up something worth keeping.

## `Inven` is the Inventory button, and the client ships two status bars

`Inven` is a **WZ node name**, not a label on screen: `StatusBar.img/button:Inven`, the button
that opens the item bags. Its siblings on the same bar are `Equip`, `Stat`, `Skill`, `Key`,
`Menu`, `Shortcut`, `Mailbox` - and `CashShop`.

But the client also ships **`StatusBar3.img`**, a completely different modern bar whose Cash
Shop button lives at `mainBar/menu/button:CashShop` beside `Event`, `Character`, `Community`,
`Setting` and `ExitDungeon`. If *that* were the bar in use, the click would go to a different
dispatcher and the watch on `FUN_1411ab7b0` would be aimed at nothing.

## Which one is live, with the control run first

`tools/xref.py --string <name> --callers`, classic-bar names as the positive control and
modern-bar-only names as the question: `[L]`

```text
  QuickSlotD           2 code reference(s)     StatusBar.img
  Inven                1 code reference(s)     StatusBar.img
  ChatTargetSelect     5 code reference(s)     StatusBar.img
  monsterCollection    0 code reference(s)     StatusBar3.img only
  bossParty            0 code reference(s)     StatusBar3.img only
  dailyGift            0 code reference(s)     StatusBar3.img only
  GuildCastle          0 code reference(s)     StatusBar3.img only
```

**The control passes, so the zeros mean something.** `StatusBar3.img` is dead art in this
build - the same situation as `UI/UIWindow2.img/Trunk`, whose absence keeps the classic shop
window unbuildable. This client draws `StatusBar.img`, `FUN_1411ab7b0` is its dispatcher, and
the watch is aimed correctly.

That also settles the button list: the nineteen arms of `FUN_1411ab7b0` **are**
`StatusBar.img`'s own buttons, name for name, matched against the WZ rather than assumed.

## So the control click can be any of them

The point of clicking another button first is not `Inven` specifically - it is that **the
control and the subject go through the same function**. `Equip`, `Stat`, `Skill`, `Key`,
`Menu` and `Mailbox` are all in the same comparison chain and all work, so any one of them
proves the watch is armed before Cash Shop's silence is allowed to mean anything.

Without it, "no WATCH lines" has two readings - the click did not reach the dispatcher, or the
hook never armed - and this project has spent runs on exactly that ambiguity.
