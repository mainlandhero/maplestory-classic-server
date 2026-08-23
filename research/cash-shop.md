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
