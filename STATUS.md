# Where things stand — 2026-09-21: **the beauty shops, and a client-side unequip block**

Pick-up notes for the next session. `docs/` has the specs; `STATUS-history.md` has the
finished goals, split out of this file on 2026-09-21 and kept verbatim for the method.
`ROADMAP.md` was deleted the same day - it was the Aug-24 staged plan and every stage in it
had either shipped or been overtaken, so it was describing a project that no longer existed.

## START HERE

*Rewritten 2026-09-04 against the code and the archive. Everything below this section is
reverse-chronological working, kept for **how** things were found and how they went wrong -
a verdict down there may have been overturned up here.*

**This is a playable single-player server, and as of 2026-09-03 a two-player one.** Two
clients run on one machine, stand on one map, and see each other move, attack, take damage
and form a party. That was the blocker on every multiplayer feature for weeks and it is gone.

One command, from an **elevated** shell:

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

It builds, installs the hook into `client-patched/`, starts **the servers** - `maplecw-login`
on 8484, `maplecw-world` on 8485 and 8486, `maplecw-auth` on 8480 - and starts **the
launcher**. Sign in there, press Start Game. **Login is enforced since 2026-09-05**: a client
that did not come through a launcher sign-in is refused at the login screen (the
`notRegisteredID` notice), not served a fallback account - the old default, which opened the
client straight in as `maplecw`, is behind `-DirectClient -FallbackAccount maplecw` for hook
debugging only. Close the client by hand when done, then `-Stop`. `-ListOnly` prints the
stored characters and launches nothing. **Launch the two clients one at a time.**

**`-SetFieldProbe` is not optional.** Its name is a fossil: it now means "the channel
answers at all". Without it `Session::handle` returns nothing for *every* packet, the
migration hello goes unanswered, and the client sits on "Connecting..." looking exactly
like a server that is not running. It cost one of the owner's manual launches on 2026-08-20.

**Nothing authenticates the game socket.** The launcher authenticates a *person* (argon2id)
and stakes a login claim the login server matches to the process that owns the socket; the
game socket itself carries no credential and never has. Say so in every progress report.

**2026-09-16..21: the beauty shops are wired, and one open client-side bug.** Since the
2026-09-15 release, all measured against the crate suites rather than a screen unless said
otherwise - **none of this has been on a client yet**:

* **The hair salons** (`crates/world/src/salon.rs`, `session/salon.rs`). The owners - Natalie
  215 in Henesys 10001044, Don Giovanni 413 in Kerning 10003005 - take both style coupons
  (5150100 Signature, 5150000 Mystery) and list whichever the player holds as a type-6 menu
  with the item's icon; the assistants - Brittany 216, Andre 414 - take the two colour
  coupons (5151100, 5151000). A style keeps the player's colour, a colour keeps their style.
  Signature opens the client's own pick-a-look box (message type `0x0a`, `npc_avatar`);
  Mystery spends the coupon on the roll as the line is clicked. Commits 8ca2ef1, 3d74e66,
  237bda8.
* **The plastic surgeries** (same modules). Denma 213 / Dr. Feeble 214 in Henesys 10001043,
  and **Orbis** 20000031's Franz 1018 / Riza 1019 - **Kerning has no plastic surgery in this
  client**, `gm-handbook/maps.txt` has exactly two. Owners take the face coupons (5152200,
  5152000) and a face keeps its **eye colour** (the hundreds digit); assistants take 5153000
  and offer the **seven** skins this client has body art for, rendered from
  `Character/0000200X.img` to check - the COT site's Green, Pink and Warm have no image here
  and would draw an invisible player. **There is no Mystery skin item** in this client, so
  that menu has one line. The pick box classifies itself by its **first** candidate id
  (`FUN_142a91f30`: `/10000` in {3,4,6} hair, {2,5} face, under 24000 skin), so skins go on
  the wire as plain 0..6 - **a skin box has never been drawn on a screen.** Commit 4bd9613,
  decompile `research/msexe-avatar-dialog-init.c`.
* Also since the release, and also unmeasured on screen: fame/defame with the day and week
  windows, the Character Info item list, inventory **Consolidate** and **Sort**, same-item
  drag merging, return scrolls on the opcode the client actually sends (`0x0123`), two pet
  fixes, and **Gift Drops** (`!giftdrop`, commit 1d31d72, from the parallel session).

**Open, and it is the client's own gate: unequip is blocked immediately after a map change.**
The owner, 2026-09-18: *"Immediately after changing maps, I cannot unequip from the Equipment
Inventory. I have to first equip something from my inventory, then I can start un-equipping."*
What the live logs settle, from `C:\Users\user\Desktop\Server Investigation`:

* **The server is not dropping it.** Every `0x0107` it receives is answered, including all
  three that were the first inventory op after a `SetField`.
* **The client is not sending it.** The server logs a receipt line (`<-`) before it handles
  anything, and across both live logs the first op after a fresh `characterData=1` entry is
  **14 equips, 23 rearranges and 0 unequips** - a request that was sent and ignored would
  still have left its receipt.
* Two candidate readings, and they need different work. **(a)** The Equip tab is the only
  tab that gets no `0x0070` on field entry - it rides in the `SetField` record instead,
  deliberately, because sending it twice would double every item - and the two unequips that
  did work early were both invType **6 (Deco)**, which *is* restored. **(b)** A brief
  post-entry window: the one real-equipment unequip that worked as a first action came **56
  seconds** after the portal, well past any such window.
* The discriminator costs no build: **after a map change, wait ~10 s touching nothing, then
  try to unequip.** Works -> (b), a timing gate, and the server cannot fix it. Still blocked
  until you equip something -> (a). `FUN_142382330` (`u_DragEnd`) gates on tab-type match and
  showed no field-entry flag, so this was not settled statically.

**2026-09-15, release: the world hub, and everything social that crosses channels.** What
this package carries that the 2026-09-13 one did not, in the order a player meets it:

* **A third server process, `maplecw-chat`, on loopback 8483** (`crates/world/src/link.rs`,
  `bin/chat_server.rs`). Every channel dials it once (`--link`, default on; `none` runs a
  channel alone). It relays character-addressed packets to the channel that hosts the
  character, keeps the directory of who is online where, and serialises party requests so
  every channel's party replica applies one sequence. Not a database - the owner's constraint -
  and nothing in it survives a restart; the channels re-announce their players. Its log is
  `chat-hub.log`. `start-server.ps1` starts it first; `package-server.ps1` ships it; it is
  never opened in the firewall or forwarded. Measured live 2026-09-15: create, invite,
  accept and pick-up-rights toggles all echoed through it and answered from the echo.
* **Parties across channels.** The registry is hub-serialised; invite, join, leave, expel,
  leader change and pick-up rights cross. Pick-up rights is a **toggle** (the button's
  request carries no value) answered with the client's own `0x2D` - "changed to Party
  Leader / All"; under Party Leader only the leader (and a drop's killer) takes a party drop.
* **Party mesos** split like party EXP: 70% to the picker, a copy of the party share to every
  other member on the map, drawn as the client's yellow *"Spotting Small Change (+n)"*; a
  player's own dropped mesos are 100% to whoever picks them up. Seen on screen.
* **Party chat** (`0x0179` -> `0x01B1`, any map, any channel), **whispers** (`0x017B` ->
  `0x01B3`; the target anywhere, the sender always answered, `/find` as a plain line) and
  **Maple Chat** (`0x01FD` -> `0x00A3`): open, the invite dialog anywhere, accept, and the
  window's six seats with each member's avatar look. Rooms still live in the channel process;
  a typed line and a closed window are the next two captures. The buddy list is not built.
* **The Cash Shop's two panels** draw and move items both ways; **cash equips live in the
  Deco tab** (150 slots from the first login) and the chat folds accented names to ASCII.
* **Summoning sacks**: the mob gets a controller and the summoning circle, then a `0x03E8`.

**Operator-visible changes:** one more process in the window and one more log; 8483 stays
closed; `chat.log` was renamed `chat-hub.log` before it ever shipped. Everything that was
true on 2026-09-13 below still is.

**2026-09-13, release: the launcher checks its own client folder, one live login claim per
account, and a superseded launch is thrown off the channel.** Three changes, all from one
thread that started with a second machine failing to start the client at all.

* **`crates/launcher/src/integrity.rs`** reads the PE import tables of every module in the
  client folder on every Start Game and reports two things into the launcher log: a DLL that
  is **missing**, and one **borrowed** from that machine's PATH rather than shipped. The
  second is the finding that cannot be made on the machine where the client works. The real
  folder measures self-contained - 29 modules, 56 ms, nothing borrowed - so the owner's constraint
  (*"we should not have any dependencies on the actual install of MapleStory"*) is now a
  measurement. It reports and never refuses. Blind to `LoadLibrary` and registry lookups, and
  it says so in its own output. `--check-client [folder]` asks without launching.
* **One live login claim per account** (`crates/store/src/claims.rs`). Staking deletes
  `WHERE account_id = ? AND token_hash <> ?` - your own earlier launches, never anybody
  else's, so the otter/owl eviction bug in that module's header stays fixed. This is what
  the live log of 2026-09-13 needed: one person signed in six times from one address, and
  from the fourth connection every one was refused with *"4 login claims are live and the
  evidence presented did not pick one out"*. The guard was right; their own retries were the
  other claims.
* **A superseded launch is disconnected** (`crates/store/src/kick.rs`, `crates/world/src/server.rs`).
  Deleting the claim only stopped the old client being recognised at its *next* login; one
  already in the world kept its channel socket. The sign-in now queues a kick in the same
  transaction, and the channel obeys it within two seconds. One row per account with a
  **timestamp, not a flag** - a connection obeys only a kick newer than the moment it joined,
  which is what stops the sign-in disconnecting the client it just authorised. Every
  deliberate close now returns `Close::Server(reason)` and the accept loop logs
  **`DISCONNECTED BY THE SERVER: <reason>`**; a socket failure stays a separate line, because
  os error 10054 is the client crashing. The channel had no server-initiated disconnects at
  all before this.

**Not covered, and worth knowing before somebody reports it as a bug:** the login socket is
not disconnected, only the channel - a player at character select on the superseded launch
stays there. No notice packet is sent before the close; nothing in this client has been
decoded as *"you were disconnected"* and inventing one is how a client freezes. And an
existing database can still hold several live claims for one account: nothing migrates them,
and each account collapses to one the next time it signs in.

**Still open from the same day:** the summoned pet does not render, and does not pick up
items or mesos. Four watches will name the gate - `-PetGates`. And Joanne's machine still
cannot start the client; the server log shows only their launcher's reachability probe, so it
dies before any socket. 2480 tests pass.

**2026-09-10, night: THE HYBRID BUILD IS INSTALLED - the modern client's Signature Style
Collection (206 items) is in `client-patched/Data`, and the Cash Shop's Special tab sells it,
badged NEW. THE SHOP TAB IS SEEN ON SCREEN** - the owner, with a screenshot: *"The items actually
render fine in the cash shop"* - nine entries, NEW badges, icons, tooltips: the classic client
loads the rebuilt archives. The equips, hair and faces on a character are still unseen. Prices
are the owner's (8,000 / 2,000 LP); the box now gives ALL eight set coupons and each coupon its set
(`world::signaturestyle`, opened by `session::cashitem`, all or nothing). The owner: *"put all of these modern maple assets into the
classic WZ data and have clients run off of a hybrid classic + selective modern asset build"*,
then *"put the full package items for sale in the special tab of Cash Shop ... Label them with
the 'NEW' icon."* `crates/wz/src/writer.rs` is a WZ writer (an image is self-contained, so a
v271 image copies into a v779 archive byte for byte; only the directory layer is new);
`tools/backport_install.py` rebuilds the 24 affected classic archives against the originals,
verifies, and installs with `.bak` siblings. Read out of the classic client: `Class 0` on a
Commodity row is the NEW badge, `Class 2` is HOT (15/15 against the owner's Main-tab screenshot),
and Special was blank because it declared no sub-tab. Prices are a placeholder (7,900 / 3,900
LP). `!hair` / `!face` exist for the id test. The test steps are in
`backport/signature-style/README.md` because another agent held the launcher script; opening
the hair/face COUPONS is the one rule left, pending a capture of the Consume-tab use opcode.
Three pre-existing traps fixed on the way: `gen_item_rules.py` read 0 rows since `unitPrice` was
added; `dump_names.py` wrote over the mob SPAWN table; `dump_names.py` decoded the dumper's
UTF-8 with the console code page, so "Übel" reached items.txt double-encoded.

**2026-09-10: the station clock is decoded and wired - `0x01BC`, type 1, `u8 hour, u8 minute,
u8 second` - and NOT YET SEEN ON SCREEN.** The owner's 2026-09-09 report was *"the server clock
does not seem to work. It just stays on 00:00."* The day before, this was written up as
**blocked behind an unfound `CStage::OnPacket`**. That was wrong: `CField::OnPacket`
(`FUN_141820080`, `0x1a4..0x5ab`) had been in `research/msexe-gamestage-dispatch.md` since
2026-08-19, and the "not found yet" sentence it rested on was a stale line in a sibling file.
A review caught it. The whole 128-case dense switch is now enumerated in
`research/msexe-field-cases.txt`; case `0x01BC` is vtable slot 59 -> `FUN_1418564d0`, whose
type-1 arm reads three bytes and calls the widget's set-time, which divides by 12 for the
AM/PM display [L]. The send is **gated on `gm-handbook/clocks.txt`** (three maps: Ellinia
Station and the two Orbis station maps) because the widget fetch throws on a map that built
none, and it carries **UTC** - the first version sent local time, the owner saw EDT on the wall
and said *"this needs to read the UTC time"*, so it is server time, the same the log stamps.
Test plan step **TK** says what each outcome on the Ellinia Station wall means.
`research/field-clock-2026-09-09.md`, third pass.

**2026-09-10, later: the Ellinia Station door was still dead, and so were the Free Market
doors, for one reason - a SCRIPT portal never sends `0x00D1`.** The owner: *"The portal in Ellinia
to go to Ellinia Station still currently does not exist."* Their two presses that morning are
in `world.log` as **`0x014A`** - `u8, str "in03", i16 x, i16 y` - logged UNKNOWN and never
answered; twelve captures across three runs, every one `in03`, and the client's own builder
(`FUN_1428b2330`: `u8, str, i16, i16`) agrees [L]. Yesterday's fix put the destination in
the portal table and left the request unhandled - "built is not wired", one file over.
`0x014A` now resolves through the same named-portal path as a walk, which covers the Free
Market too: all four of its doors are `pt 7` script portals and were sending the same
packet. `research/script-portal-request-2026-09-10.md`. **CONFIRMED ON SCREEN the same
evening** - the owner: *"I can indeed press up at the correct location and be teleported to Ellinia
station."* Before that they asked why they could not SEE the portal: portal 38 is `pt 8`, and the
client's `MapHelper.img/portal/game` has graphics for `pv`, `ph` and `psh` only, so a `pt 8`
is invisible by the client's own data and the background arch is the door. TF is testable now.

**2026-09-10: RETRACTION - Cobalt is MALE, so the "opposite-gender top" of 2026-09-09 was
never one.** The owner sent a screenshot of Cobalt wearing *Blue Sergeant (M)* and asked for a
server-side gender gate for normal and cash equips. The gate has existed since 2026-09-09 and
it let this equip through **correctly**: `characters.gender` is `0` for Cobalt - and for all
four characters - and the client's own `Etc.wz/MakeCharInfo.img` lists face `20002` and hair
`30025` only under `male`. Yesterday's *"Cobalt is female"* was asserted without checking
either, so the character-select "no top" is **open again** - the owner: *"if Cobalt is male, then
that does not explain why at character selection, the top does not render."* Gender was not
the difference. **The one difference the data does show [I]:** `Coat/01040021.img` and a
starter top's image are structurally identical (same 34 stances, same `mail`/`mailArm` parts,
`islot`/`vslot` `Ma`), but the Blue Sergeant's `info` requires **level 20, STR 30, DEX 10,
warrior**, and Cobalt is level 20 with **STR 27, DEX 5** and INT 74 on a Swordsman - an AP
reset into INT after the top went on. It is the only worn item with any requirement at all
(hat, pants, shoes and suitcase are all zero), so "four draw, one does not" lines up with
"four have no requirement, one is unmet". That is the same shape of coincidence that produced
yesterday's wrong answer, so it is a hypothesis with a discriminator, not a finding: Cobalt has
10 unspent AP; **3 into STR and 5 into DEX from the stat window** meets 30/10 with no server
change. If the top then draws at select, the select screen honours requirements and the
in-field renderer does not. It also means **this server does not enforce equip requirements on
equip at all** - level, stats or job - which is a gate the owner may want next to the gender one;
`EquipTemplate` already carries the numbers. What
landed today: the gate is now driven by a test through a real `0x0107` for a normal slot and
a cash-equip slot (`dst -105`), and **creation derives gender from the chosen face and hair**
against the client's lists, overriding the create request's `u32` gender field when the two
disagree - that field has only ever been seen carrying `0` and was never discriminated.
`net::equipgender::gender_of_look`.

**2026-09-10, evening: "not enough Leaf Points" with 105,500 LP was a schema gap, and the
reason code lied about it.** The owner: *"I just tried purchasing 5x Etc Tab 5-Slot Coupon, but I
was met with I did not have enough leaf points. I absolutely do."* `world.log` has it: the
buy parsed, SN 130500004, 100 LP against 105,500 - and the store failed with *"no such column:
failed_slots"* reading `cash_locker`. `item_columns()` had gained `failed_slots`, `inventory`
and `equipment` had been ALTERed, and `cash_locker` and `storage_item` - which share the list -
had not, so on the owner's file every locker and storage read failed. The handler then reported
every store error as NOT_ENOUGH_CASH. Fixed: both tables get the same `PRAGMA`-guarded ALTER
on open; the reason is now the store's own (`NotEnoughMesos` -> 601, `StorageFull` -> 614,
anything else -> the honest "unknown error"); `wisps_real_database_upgrades_in_place` now reads
the locker and storage for every account, and the wind-back test buys on a locker that predates
the column. **The upgrade test read two of the four item tables and was called a schema test.**
Step TC is the retest.

**2026-09-10, evening: the character-select sheet shows equipment totals.** The owner: *"the stat
screen on character select should reflect all equipment bonuses like our current character stat
window"* - 1026/1006/1073/1003 in the field, 27/5/74/4 at select. In the field the client adds
the worn items' stat blocks from the record itself; at select it has a look, which carries ids
only. So the login server now sums worn items the way the world fills the record - stored block
first, `EquipTemplate::fresh_stats` second - into the sheet it sends (`login::selectstats`;
login depends on `world` for the templates, loaded from `gm-handbook/equips.txt` at start with
a banner). Display only: the world's `SetField` still carries the bare row, which the client
adds to itself. **Side observation carried in step TG:** if the select renderer checks the Blue
Sergeant's STR 30 / DEX 10 against the sheet, it now passes, and the top should draw with no AP
spent - one launch, two readings.

**2026-09-10, night: both confirmed on screen, and the select-screen top with them.** The owner:
*"The armor render works now and the stats are accurate with the current equipment buffs."*
The top drew the moment the sheet carried totals, with no AP spent - so the select renderer
checks an item's requirements against the stat block it is handed [L, one screen], and the
2026-09-09 "no top at select" is closed: base STR 27 failed the Blue Sergeant's 30. **This
server still enforces no equip requirements at all**; that is now a known, open gate.

**2026-09-16 late: the pet-skill bits were `1 << index` and the client's are not - fixed, and
the vacuum is a two-item chain.** The owner, applying Expanded Auto Move on a fresh Husky: *"You do
not have a pet that can use this skill."* The client-side gate reads the pet's learned mask
(`pet+0x1c`) and Expanded Auto Move needs Auto Move first. Two findings: (1) `net::bag`'s bits
were `1 << index`, but `FUN_1414b89b0` and the modern `PetSkill` enum both give `Item Pouch
0x01, Expanded Auto Move 0x02, Auto Move 0x04, Ignore Item 0x08, Auto HP 0x20, Auto MP 0x40` -
so every skill item was teaching the wrong skill and a learned Auto Move showed as "Ignore Item
(Learned)". Corrected, `the_pet_skill_bits_match_the_clients_own_mask` pins them. (2) The vacuum
is Auto Move (`5190002`) THEN Expanded Auto Move (`5190003`); the box then rides `0x0198` on
wonderGrade 6 as before. `research/pet-vacuum-wondergrade-2026-09-16.md` §7. 2132 tests. Plan
step 8 LOOT has the order.

**2026-09-18: double-clicking another player opens their Character Info - `0x01FC` answered
with `0x00A2`.** The owner: *"I just tried double clicking on Tester2 to display the Character Info
window as the owner."* The click was on the wire (`u32 tick, u32 id, str "", u8 petInfo`) and
unanswered - and both its builders set the shared request latch, so it also froze ~35 other
request senders until the next map change. The reply was found without guessing an opcode: the
double-click handler's window singleton -> its one constructor site -> dispatcher case `0xA2`,
which also clears the latch (the opcode table's two candidates for `0xA2` were both wrong, and
the first guess `0x00BE` is the NPC pool). Body: result, id, name, level, job, fame, guild, the
pet out (item id, name, level, closeness, fullness, then the whole pet item behind a flag) or
zeros, two empty vectors (ITEM / CITIZENSHIP tabs), petInfo echoed; a refusal is a lone non-zero
result. `net::charinfo`, `session::charinfo`; `research/character-info-2026-09-18.md` (an
agent's decode, [L] for every field's address). Never on a screen; plan TO(c) has the readings.

**2026-09-18: the fame messages lose their doubled apostrophes.** The owner, off a screenshot:
*"Too many apostrophes."* The server sends only the names; `'%s''s level of fame` is the
client's own template, four of them (`0x00FA`, `0x00FB`, `0x0103`, `0x0104`) in the encrypted
string table. `grap_stub::fametext` rewrites them in place the way `beautytext` does - same
length, trailing spaces, bytes produced with each entry's own key (the plaintext-difference
shortcut fails on a byte that decrypted through the table's NUL quirk, and `0x00FA` has one).
`-NoFameTextPatch` is the off switch. Unverified on screen; the plan's fame step has the
reading.

**2026-09-18, third attempt: another player's look change is `0x02AE`, the user pool's own
in-place redress.** The owner: *"The leave-and-enter path causes the pets to reload for that client,
and it causes a brief blink. That is undesirable. Please find another suitable way."* An agent
read the client this time: `0x0138`'s apply walks the user's **summoned** map (`user+0x1200`,
filled only by the `0x03A0..0x03C5` pool packets) and never the player - so 14:07 could not have
drawn, and the hook's `avatarmod` patch is retired. The routine that dresses a user is the
rebuild `FUN_140f80200(user+0x100)`, and one inbound packet reaches it for a user already in the
pool: `0x02AE`, table C slot 3 beside the chair relay `0x02AD` (verified from the raw table
bytes with `0x02AD` as the control), handler `FUN_1429d5290`: flag bit 0, the compact look
decoded straight into `user+0x130`, three ring bytes, two u32, then the rebuild and the two
post-passes every `0x0224` runs. `net::lookupdate` builds it (211 + 5 per worn item, length
pinned - the chair relay one byte short faulted a client); `broadcast_look_change` sends one per
observer by default; `--look-reenter` (`-LookReenter`) keeps the leave + enter as the fallback.
`research/remote-redress-2026-09-18.md`, `research/msexe-remote-redress.c`. **[D]** that the
rebuild draws - never on a screen; plan TO(c) has the readings. The same file corrects two
older notes: `mob-combat.md` §7.2's "drop pool" at `0x3A0..0x3C5` is the summoned pool, and
`beauty` §8.2's five "user-pool callers" of the dress primitive are the cash-shop preview widget.

**2026-09-18 14:07 run: the in-place `0x0138` is INERT; leave + enter is the default again.**
The owner: *"Changing equipment once again no longer publishes to other clients."* With the hook's
`avatarmod` patch applied, the `0x0138` reached the observer, its handler ran and returned
normally, and the copy did not change - reading (b) of the plan step, measured. So
`Config::look_change_reenter` defaults to `true`: every worn / hair / face change publishes with
the brief blink and the pet respawn, which remain the open item. `--look-in-place`
(`-LookInPlace`) is the opt-in for the next attempt; the next step is static - what the
client's apply walks for a remote user - and needs no launch. `research/beauty` §8.3.

**2026-09-18 evening: another player's look change is redrawn IN PLACE - `0x0138`, gate opened
by the hook.** The leave + enter worked and the owner measured its cost: *"a weird super brief
character blink ... The regular maplestory does not have this behavior"*, and with a pet out
*"the pet completely respawns and appear sad/hungry"* - the remote `CUser` and its pet are
rebuilt, which is what that sequence is. `0x0138 UserAvatarModified` is the client's in-place
update; its apply is behind a `je` after an always-zero stub (§4.2 of the beauty research,
still right). `grap_stub::avatarmod` now nops that `je` (`142797ded`, two bytes, the function's
one caller is the `0x0138` handler; the dress primitive behind it has five callers in the
user-pool region, so it is the real one - `tools/callers.py`, [L]). The server sends ONE
`0x0138` per look change - equip on/off, cash, hair, face, pet hat - and `--look-reenter`
(`-LookReenter`) keeps the leave + enter as the fallback; `-NoAvatarModPatch` is the hook's
off switch. **Never on a screen**; [I] is that the `user+0x1200` list the apply walks holds the
drawn avatar. Plan TO(c) has the three readings and the fallback pair. `research/beauty` §8.2.

**2026-09-18: a hair or face coupon redraws the player where they stand - `0x007C`, not a
reload.** The owner: *"the player needs to enter a different map to see the hair or face updated on
their character."* The 2026-09-12 handler had dropped the re-entry on the reading that the
Beauty dialog commits the look on Confirm; it only previews, and this report measured that.
The client's `StatChanged` handler has a FACE/HAIR branch (`142d560d5` / `142d56122`) that
runs the equip handler's own redraw pair (`FUN_142ce51b0` x2, `FUN_142ce5e60`) - the path
every on-screen equip change uses **[L]** - so Confirm is answered with ONE `0x007C`: unlock
byte, the look bit, the id. `!hair` / `!face` send the same and no longer warp to the spawn
point. `beautycoupon::look_stat_changed`; `research/beauty-2026-09-09.md` §8; plan TO(c).
**Confirmed 14:51 on the changer's own screen.** The OTHER client drew nothing: it received
the `0x0224` (`world-ch0.log`), and a `0x0224` for an id already in the pool is a no-op the
research had recorded. `broadcast_look_change` now sends `0x0225` then `0x0224` then the pets
for that one character, the sequence a fresh sighting gets (§8.1). Unverified on the observer.

**2026-09-22: the trade WINDOW opens.** The owner: *"Tester2 just sent the owner a trade request, but
after the owner accepts it, the Trade window did not open."* The invite has worked since
2026-09-09; what was missing is `0x0575` **mode 4**, whose payload
`research/trade-2026-09-09.md` §3 left undecoded because it runs through a virtual call on
whichever miniroom class the room type selects. Read now: for a trade the call is
`FUN_141C423D0`, and its **only** packet read is `FUN_1402ee8d0` - the same avatar decoder
`0x0224` uses, so a member is `u8 slot`, `opcode::avatar_look`, `u32 id`, `str name`, `u16`,
with the list ended by a negative byte and the two flag bytes before it being the slot
capacity and the recipient's own slot. `net::trade::room_open`; the room lives in
`session/trade.rs` in a process-wide table keyed by the ticket (the inviter's id), the shape
`session/messenger.rs` already uses. Both sides get a mode 4 with their own `mySlot`.
**Not built: putting items in.** Modes `0x0C` and `0x10` - the trade dialog's own senders -
are still unanswered, so the window opens and nothing can be traded in it yet. **One [D]
carries risk**: the handler's trailing virtual call resolves to a method that reads nothing,
and if that resolution is wrong the body is short. Plan step 13(b) asks for `client-exit.log`
rather than assuming. Never on a screen.

**2026-09-30: the GM's Blessings are weather and a buff for the whole map.** The owner: *"these two items
should also be atmospheric effects that gives all players a buff ... it does not give players the
appropriate buff icon with a duration."* Three gaps, all closed: **Wind sent no buff at all** - its
`indieSpeed 30`/`indieJump 10` had no bit and were logged as unsupported; they now ride Speed (92) and
Jump (93, `Jump` in the client's own CTS name table), the Magic Armor precedent for `indie*` **[D]**.
**The other players got the giver's raw `0x007D`** and their sessions never recorded it; they now get
`Event::ItemBlessing` and apply it through their own session (`receive_item_blessing`), so the icon
counts down and their tick expires it. **No weather was shown**; the map now gets `0x01B7` with the
Cash item whose `stateChangeItem` is the blessing - `5121000` (GMevent1) for Wind, `5121001` (GMevent2)
for Precision **[L]** - carrying "<giver>'s blessing" line, for 30 s. `stateChangeItem` is read only by
the item-info loader, so the client does not apply the buff itself. No capture of either blessing
being used exists in any archived log. Test plan 27(d). **Not on a screen.**

**2026-09-30: weather items show on the map.** The owner: *"display my chosen message with the particular
item effect as an atmospheric effect for everyone present in the map for 30 seconds then gradually
fade out."* Their Sprinkled Chocolate arrived on `0x0116` as `tick, slot, item, str text` (the client
wrote "the owner's Chocolatey Message: " itself) and was answered with the unlock only. Now one is spent
and everyone on the map gets **`0x01B7`** - `u32 item, str message, u32 seconds, u8 0` **[L]**,
`FUN_141853820`, which hands `seconds * 1000` to the weather routine `FUN_14185b1c0`; the art and
the fall come from the item's own `path`/`floatType`. 30 seconds; the fade is the client's own
**[I]**. One effect per map at a time (a second is refused and kept); a player arriving mid-effect
gets the remainder on field entry (`Fields::weather`). All of `0512.img` (512xxxx) is routed.
`net::weather`, `session/weather.rs`, `research/msexe-weather*.c`. Test plan 27. **Not on a screen.**

**2026-09-30: Megaphones and Super Megaphones speak.** The owner: Super Megaphone *"across all channels
(with a pink background), and display the whisper icon depending on client selection"*; Megaphone
*"on the same channel (without the pink background)"*, whisper icon likewise. Both arrive on
`0x0116` as `tick, slot, item, str text, u8 whisper` (their capture, 03:52:15, and the builder
`FUN_141a50140`) and were answered with the unlock only, item kept. Now one is spent and one
`0x00AC` line goes out: **type 3** for the Super Megaphone (chat kind `0xd`, the pink one) to
everyone in `Link::everyone` via `deliver_anywhere`, the whisper path; **type 8** for the
Megaphone (kind `0xf`, no item) to this channel's bus. Both carry the sender chat-info block the
client reads - `str, str, u32, u32, u8, u32, u32, str, u32, str, u32` **[L]**, named by the v214
reference - then channel and whisper. **[D]**: type 8 is used because it is the only non-pink type
whose whisper icon follows the byte; type 2, the classic channel megaphone, reads no whisper byte.
Whether kind `0xf` draws without the pink background is unmeasured. `net::megaphone`,
`session/megaphone.rs`, `research/msexe-megaphone*.c`. Test plan 26. **Not on a screen.**

**2026-09-26: the ship from Ellinia Station to Orbis.** The owner: tickets from Joel, *"Ticket to Orbis
(Basic) - 5000 mesos, Ticket to Orbis (Regular) 20000 mesos"*, *"via NPC dialogue and selection,
not a shop window"*; a ship every 10 minutes, a 5-minute crossing, boarding *"from 5 minutes before
departure time up until 1 minute before"*; every Basic passenger for one departure in one instance
per channel; Regular is *"a private ride that only lasts 1 minute"*. `crate::boat` holds the
timetable (departures on every Unix multiple of 600 s - `:x0` on the station's UTC wall clock) and
each channel's voyages (`Fields::voyages`); `session/boat.rs` puts it on the wire. **Joel** is a
type-6 menu and sells through `Store::buy_item`, one transaction, with a grey meso line and a grey
item line. **Cherry** takes a Basic ticket only inside the window (the ticket goes after every
refusal is ruled out) into `10002091` Before Takeoff, or a Regular one straight onto `20000022`.
**A voyage is a `FieldKey` instance** on `10002091`/`20000022`/`20000023`, derived in `field_of`
exactly as the party quest's is, so the cabin portals keep it; every field entry there sends a
type-2 countdown (none of the three declares a clock node). The session tick sails what is due and
lands what has arrived at `20000010` (`Event::BoatWarp` for other passengers; the take is
test-and-set). **Purin** in the waiting room - their own line promises it - sends a passenger back to
the station, ticket not returned. A login on a ship field lands in the station (its
`forcedReturn`); a disconnect or any non-ship field entry leaves the voyage. 14 tests, 8 mutations
each caught. Test plan step 25. **Not on a screen.**
*Later the same day*, from the v96 scripts the owner pasted (Joel `1032007`, Cherry `1032008`, Purin):
Joel opens with their station introduction and Next brings up the tickets; Cherry refuses in the v96
words ("We will begin boarding 5 minutes before the takeoff...", "This ship is getting ready for
takeoff...") and otherwise asks "Do you still wish to board the ship?", No getting them "You must
have some business" line; Purin asks "Are you sure you want to get off the ship?", No getting them
"You'll get to your destination in a short while" line. Wording changed only for this server's
10/5/1-minute timetable and the paid tickets. The owner also asked that a disconnect mid-journey or
before departure put the player back in the departure station - that was already the login rule
above, tested through the real migration hello. 15 tests; 6 more mutations, each caught.
**The ship at the station animates** (the owner: arrive at `:x5` *"so players can board"*, and
*"all other times ... the opcode that animates the boat leaving"*). Ellinia Station is
`fieldType 2` with a `shipObj` (`x 1545`, `x0 2100`, `tMove 15`, `shipKind 0`), and a
`fieldType 2` field has its own handler `FUN_140d6cc90` that takes **`0x01BF`** (ship move:
type 12/state 6 arrive, type 8/state 2 leave) and **`0x01C0`** (ship state: 0/1/6 arrive, 2/5
leave) before `CField::OnPacket` **[L]**. The arrive routine plays `Whistle` and slides the ship
from `x0` to `x` over `tMove` seconds; leave is the reverse **[D]**, from the loader's store
order. Found through the key strings' pointer table, since nothing `lea`s them:
`research/ship-contimove-2026-09-26.md`. `net::ship`. Entering the station sends `0x01C0 [1,0]`
from `:x5:00` until the departure and `[2,0]` otherwise; the change itself goes to everyone on
the station as `0x01BF`, once per channel (`Voyages::take_station_change`), and only within 5 s
of it. 17 boat tests; 6 more mutations, each caught. Test plan 25(j). **Not on a screen.**
**The Crimson Balrog invasion** (the owner: *"a 50% chance that any given trip will be invaded by 2
Crimson Balrog ... at 1 minute into the 5 minute boat ride ... not on the 1 minute private
rides"*). Rolled once per shared voyage at the departure; a minute in, one passenger standing on
the deck sends `0x01BF [10, 4]` - `FUN_140d6b130`, which draws the deck's own `shipObj`
(`ship/ossyria/97`) and only on a `shipKind 1` field **[L]** - and summons two `700005` in the air
at that ship's position (the owner: they fly), controlled by that client. A latecomer on the deck gets
`0x01C0 [3, 1]` and the Balrogs with the field. Leftover Balrogs stay in the finished instance's
`Fields` entry (no instance teardown exists yet).
**2026-09-27: the ship back, Orbis to Ellinia.** The owner: *"Agatha will sell the tickets"*, and the
Platform Usher *"will offer the players a choice to be teleported to the correct tunnel to the
ship."* `crate::boat` is now two `Route`s on one timetable. Orbis side: Agatha (1000) in the booth
sells Ticket to Ellinia (Basic)/(Regular) - 4031084/5, the same 5,000/20,000 **[I]**; the Platform
Usher (1001) offers Isa's platform menu - the booth's `east00` has no target **[L]** - and
teleports to the Station Tunnel (20000011), which walks to Rini's platform (20000012, `fieldType
2` with its own `shipObj`); their second line was their ferry until 2026-09-29, when the ferry was removed. Rini (1004), Erin (1006), the
waiting room 20000013, deck 20000020 (Balrog ship at -590, -221) and cabin 20000021 mirror the
Ellinia side; the ship lands in Ellinia Station. Wording from the v96 Agatha/Rini/Erin/Isa scripts
The owner pasted (Rini's and Erin's are word for word Cherry's and Purin's). A login on a To Ellinia
ship field lands on Rini's platform, the departure station, not the booth that `forcedReturn`
names. 25 boat tests; 7 route mutations, each caught; net + world 1990 passed, 0 failed. Test plan
25(k), 25(l). **Not on a screen.**

**2026-09-25: Maple Island's quests are open to every job.** The owner: *"remove the requirement that
quests on the island are only for Beginners, any class should be able to do them."* **The server
never checked a quest's job - the client does**, from its own `Quest.wz`: it decides what an NPC
offers and runs the opening itself, so a non-Beginner never got as far as `0x0151`. The rule is
`Check/0/job`, which the loader `FUN_14072B230` reads into a `std::set<int>` at `demand+0xC0`
**[L]**; a quest without one is offered to everyone, like every quest outside the island and the
job areas. `tools/quest_patch.py` deletes that node from every `QuestInfo/area 1` quest - all
twenty, 1000 to 1019 (nineteen said `[0]`; 1001's 68-job list went too, since a list still refuses
what it omits) - builds from the pristine `.bak`, verifies all 322 images parse and that nothing
else changed, and installs into `client-patched`. **Installed on this machine. It reaches players at
the next `package-server.ps1` + deploy**, which ships `client-patched` as the canonical client the
launchers patch to; both release scripts now run `quest_patch.py --check` and refuse a stale
archive. Not on a screen: a non-Beginner at Sera should now get "Borrowing Sera's Mirror".

**2026-09-25: Scroll of Secrets and Treasure Scroll drop at 0.5%, up from 0.01%.** The owner: *"Can we
double check that Scroll of Secrets and Treasure Scroll are being dropped globally? If so ... let's
increase both of their droprate to 0.5%."* Checked end to end first: both are `*` rows in
`data/drops.txt`, `DropTables::roll_at` chains the global rows onto every mob (a mob with no table
included), `questitems::is_exempt` keeps the quest-item orphan filter off them (both carry the
client's quest flag and no quest names them), and the live banner of 2026-09-24 reads *"2 global
(event) rows"*. No archived log records one dropping - at one kill in ten thousand, expected. Now
50 basis points, one in 200; `scrolls::GLOBAL_DROP_CHANCE_BP` is what the scroll NPC quotes and
`the_two_scrolls_drop_globally_at_half_a_percent` pins the file to it. **The live server needs the
new `data/drops.txt` and a restart** - it is a data file, not a database migration.

**2026-09-25: channels are named from 1.** The owner: whispers and the buddy list showed a player on
the first channel as "Scania 0"; *"can we please show players as channels 1 and 2? Since that's
what the UI says in change channels."* The client never formats a channel number there: `/find`,
the buddy window's location check, a friend's status and an incoming whisper all hand the index
to `FUN_142cb92f0`, which returns `names[index]` from the channel-name array at
`singleton+0x2cb8` verbatim **[L]** - and our login server's world list named channel `i`
`Scania-{i}`. `net::opcode::world_list_entry` now names it `Scania-{i+1}`; the index bytes after
each name, which the client sends back to pick a channel, stay 0-based and a test pins both. Takes
effect at the next login. Never on a screen.

**2026-09-24: another player's pet no longer snaps when you enter the map.** The owner: *"if other
players have pets summoned, I see their pets snap to their proper positions. We fixed this for
players."* The player's spawn is rebuilt on every move; the pet's `0x0277` companion was built
once, at the summon or the owner's field entry, so an arrival drew the pet there and its first
`0x0278` jumped it. `on_pet_move` now reads where the pet's `0x0202` path ends
(`net::usermove::path_end`, which runs the player's own walker) and rebuilds the companion there
with the path's stance. Kept with its field key; before the pet walks on a new map the owner's
position stands. The walker was checked on four consecutive captured pet reports - each ends where
the next begins. Never on a screen.

**2026-09-24: standing up from a chair no longer blanks your HP bar on a partner's screen.** The owner:
*"When players initially sit on a chair then stand up, their HP bars appear empty when they are part
of a party."* The client's stand-up (`SetSeat`, `FUN_1428a81d0`) ends by sending `0x00DC`
(`research/chairs-2026-09-08.md` §13.2), the server answers it as a field entry, and the partner is
sent the stander's `0x0225` and a fresh `0x0224` - a new `CUser` whose HP percent (`+0x10cc`) is 0
until an `0x02B2` fills it. `party_hp_tick` only resent on a change of HP, max or recipients, and
none had changed. All six stand-ups in `research/fixtures/map-chairs-sit-stand-relay-two-clients-world.log`
show the `0x00DC` and the leave/enter. `on_field_entered` now clears `last_party_hp`. **Still
open, deliberately:** a stand-up re-runs the whole field entry (NPCs, bag restore, the leave/enter).
Telling it apart from a real entry would stop that, but the leave/enter may be what clears a map
seat on bystanders' screens, which is unmeasured. The blank bar was reported, not captured; never
on a screen since.

**2026-09-24: death in a party quest costs nothing; elsewhere a Safety Charm is spent instead of
the EXP.** The owner: *"dying in a party quest area should not take any EXP penalty ... Outside of the
party quest area, if the player has a Safety Charm, a safety charm will be removed in exchange for
keeping the player's current EXP level."* `revive` decides once (`DeathCost`): free at level 10 or
below; nothing on any First Time Together map, charm untouched; else one `5130000` from the Cash
tab; else the 10%. The notice is the client's own string `0x0852` *"The EXP did not drop after using
%s item."*, posted by `0x02D1` effect `0x0C` + `u32 itemId` (`net::revive::safety_charm_used`) - found
by scanning `.text` for `mov r32,0x852` (one site, in `FUN_1427863f0`) and reading the handler's
second switch table, whose control (effect 8 -> the item-line arm) matched. That arm is behind the
no-field gate and a revive is a `SetField`, so it and the grey "Safety Charm x1 has been lost" line
wait in `Session::after_field_entry` for the town's `0x00DC`. **[L], never on a screen** - if the
lost line shows and the notice does not, the effect id is wrong.

**2026-09-24: First Time Together item moves are grey chat lines.** The owner: Passes from NPCs should
show *"like 'You have gained experience (+X)'"*, and opening the Magic Box should show the box lost
and the prize gained *"in two different chat lines."* `0x02D1` effect 8 - the quest-reward line -
with a negative quantity for "has been lost" (`net::message::item_lost_in_chat`, string `0x00EF`).
Sent for the stage-1 Pass and Coupons, every hand-in of Passes, the stage-5 box to each member, and
the box opened (lost, then the prize - the yellow notice is gone). The "lost" wording is **[L]**,
never drawn.

**2026-09-24: bosses stand at once on a fresh map, and their timers run per channel.** The owner: *"the
first time the server has tried to load the map ... the Mushmom should be scheduled to spawn
instantly. Only after Mushmom is defeated, should the respawn timer kick in"*, and the timer keeps
counting with nobody there. `Fields::seed` books every point with its own `mobTime` due **now**,
outside the 75% first-fill draw - which also fixes a boss point the draw could leave out, after
which it never spawned until a restart. After a kill it returns at its own `mobTime` on the
process clock. Per channel by construction: each channel is its own `maplecw-world` process with
its own `Fields`. Not on a screen.

**2026-09-24: every mob's MP is the server's, and summon skills work.** The owner, on the King Slime:
*"There should be a jump attack, and there should be a summon slime attack"*; then *"10 MP every 5
seconds"*, and *"solve this for all mobs with skills"*. The move acknowledgement (`0x03E4`) carries
the mob's MP at offset 7 - the client's attack chooser skips any attack costing more than it
(`FUN_141c7c900`) - so MP-costing attacks were blocked on all 41 mobs that have one. `mobskills`
keeps MP per mob (full at spawn, 10 per 5 s, spent per granted attack), offers a random qualifying
skill at offset 11/15, and applies a reported summon (family 200) from the `0x02FF` skill field.
Table from `tools/dump_mobskills.py` -> `gm-handbook/mobskills.txt`. **Not done:** player debuffs
(120-126) and mob self-buffs (160-167) are never offered - their packets are not decoded.

**2026-09-24: `previous-runs/` keeps a week.** The owner: *"if the logs are older than 7 days in
previous-runs, please have the server automatically get rid of them."* `world::logprune`, at
world-server startup and every 24 h. Copy anything worth keeping into `research/fixtures/` first.

**2026-09-23 -> 24: First Time Together, stages 1 to 5.** Instanced per party - a field is
`(map, instance)`, so two parties on one stage have their own mobs, drops, broadcasts and clears.
**Confirmed on screen by the owner:** separate instances, stages 1 and 2 cleared, the WRONG banner,
the gate opening, Leaf coupons (`0x0114`). Built and unseen: stages 3 (5 platforms) and 4 (6
barrels) on the stage-2 rope code; stage 5 (all ten mobs at once, a Pass from each, per-member Slime
Shoes, twenty Slimes from the King, ten Passes to clear, a Companion's Magic Box for everyone -
61 prizes, one per double-click); stage EXP of 5/7/9/11/35% of each member's next level at the clear
animation; 10 entries per character per UTC day, counted and shown by Lakelis; a clear only when the
whole run is on the stage; a disconnect logs back in on the Exit; no party of one. **Placeholder
text:** Cloto's stage-4 and stage-5 intros await the owner's screenshots.

**2026-09-30: the live server's status in Discord - one message, edited every minute.** The owner: *"I want to be
able to integrate with discord ... configurable and only used on the live server ... current server status
and online members in each channel as well as current up time and patch version"*, *"only posts the status
once and then continuously update that same message ID"*, *"Upon server shutdown ... offline for
maintenance"*. The world hub (`maplecw-chat --discord-webhook-file PATH`, `world::discordstatus`) holds the
cross-channel roster and, every 60 s, PATCHes one embed: status, uptime, players per channel by name
(a channel seen and gone reads "offline"), total, server build (`buildstamp` time + digest), client patch
(`client-patch-version.txt`, written by `maplecw-auth --client-dir`). The id lives in
`discord-status-message.txt`; a new message is posted ONLY with no id or on a 404 - errors and rate limits
retry the edit. **Offline**: `maplecw-chat --discord-offline` (edit only, never posts), run by
`start-server.ps1` in `-Stop` and its shutdown `finally`, plus a console ctrl handler in the hub for the
window's X/logoff/shutdown. **Live only**: the URL is in the gitignored repo-root `discord-webhook.txt`,
shipped in the SERVER package by `package-server.ps1`; `start-server.ps1` passes the flag only when the file
is beside it; `test-server.ps1` never does. New crate `webhook` (rustls + `webpki-roots`, TLS 1.3, no tls12
so the pinned sign-in link is unchanged; the URL is never logged - `redacted()`). Tests: URL parsing and
redaction, plain and chunked responses, JSON escaping, the embed's contents, the id surviving a restart,
offline never posting; an ignored live test reached discord.com over TLS (401/404 on a fake webhook, nothing
posted). **Nothing has been posted to the real channel yet** - the first live start will.

**2026-09-30: the effect item is saved between logins.** The owner: *"The effect should persist and should be
saved between logins."* `store::effectitem` (new table, in `ITEM_ID_TABLES`, created before the rename
pass) saves every switch; the claim restores it if still held (else forgets it), so everyone else sees it
on arrival. **Client limit, measured statically**: the owner's OWN client resets it to off at every
character-data field entry (`context+0x2394`, cleared by `FUN_142CAD420`) and no packet sets it there
(`0x02A8` with one's own id is dropped by the remote-range dispatcher) - so after a relog the owner sees it
after one double-click, which the server treats as "no change". Tests: survives a relog, off stays off,
unheld is forgotten; fail with the save or the restore removed. Unseen on screen.

**2026-09-30: Shadow Style (and every `501xxxx` effect item) is seen by the whole map.** The owner: *"Upon double
click, the server should change the effect from OFF to ON and start animating the effect to the client and
other characters on the same map."* **What the log showed**: the double-click sent `0x00EC` `55724c00 05000000`
(5010005, slot 5) and, 2.03 s later, `00000000 05000000` - **item 0 is "off"**, so the second click switched
it off again, which is the (OFF) in the screenshot. The client toggles its OWN character before sending
(`FUN_142D4D8C0`: range `5010000..=5019999`, one switch per 2 s, `FUN_14277CD40(user, id)`, `0` when the
item is already active) and waits for nothing; Shadow Style's WZ effect is a `spectrum` afterimage, drawn
while MOVING. What was missing is everyone else: now `session/emote.rs` checks the id (effect range and
held in the Cash tab, or 0), keeps it on the session, relays **`0x02A8`** `u32 charId, u32 itemId` (handler
`FUN_1429D4F20`, a tail jump into the same setter **[L]**) to the map, and writes it at **`0x0224` offset
395** (read at `1429ce6f4` into that setter - was an [I] row) so a later arrival sees it too; the stored
spawn is refreshed on the switch. Per connection, like the client's copy. Tests: ON reaches the map and
not another map; a later arrival's `0x0224` carries it; OFF reaches the map; an unheld or non-effect id
switches nothing; fails with the relay or the spawn refresh removed. **Unseen on screen.**

**2026-09-29: emotes are seen by the whole map.** The owner: *"I just tried playing the Queasy emote as the owner,
can we make sure that the emotes are relayed to other clients in the same map as well please?"* The client
sends `0x00EA` - `u32 emotion, u32 duration, u8 flag`, Queasy captured as `08000000 ffffffff 00` - after
drawing the face on itself, and nothing answered or relayed it. Now `session/emote.rs` sends everyone
else on the field `0x02A6` = `u32 charId` + those nine bytes. **[L] both ends**: the sender
(`FUN_142D4D520`) calls `FUN_14282D710(user, emotion, duration, flag)` before encoding the same three,
and `0x02A6`'s handler (`FUN_1427862E0`) reads the three and calls that same function - which upgrades
`research/same-map-capability-sweep.md` §4.5's [D]. Tests: the capture's bytes reach a player on the
same map and not one on another map or the sender; a wrong-length body is not relayed; fails with the
publish removed. **Unseen on screen.**

**2026-09-29: Citizen of Honor - the town's earring, and every channel is told.** The owner: *"When someone
achieves that standing, they should automatically receive the earring for those specific towns. It should
be combined with a server wide blue text broadcast (on every channel) congratulating the player."* This
client has **no medal items** (no 1142xxx in `Character.wz`, no "medal" in `String.wz`); the grade-10
reward is the town earrings, Henesys Earrings `1032021` / Kerning City Earrings `1032022` [L]. The
turn-in that takes a town to grade 10 hands over that town's earring (Equip tab, grey chat line) and
sends `0x00AC` **type 0** - the client's blue `[Notice]` arm, chat kind 9 = `0xFF60CEFF` [L], screen
unmeasured - with the client's own unused sentence `0x17D9`, *"Let us all congratulate <name> for
becoming a Citizen of Honor in <town>!"*, to the achiever and, through the hub (`link::everyone` +
`deliver_anywhere`, the whisper route), to everyone online on every channel. Once per town: grade 10 is
reached once, and `citizenship.honor_earring` (ALTERed on, guarded) is a test-and-set. A **full Equip
tab** loses nothing - the notice says so and Arthur / Roxy hand it over on the next talk. Tests: the
earring, the mark, both players hearing the notice, no repeat; the full-tab path through the clerk; the
ALTER on an old table. Each fails with its piece removed. **Not tested across two channel processes** -
the hub path is the one whispers already use; the unit tests reach the other player through one
channel's bus.

**2026-09-29: the town general stores' unranked rows need a citizenship too.** The owner, at Raymond: *"some
items in the shop are not locked behind a rank"* - Fried Chicken, Hot Dog and Supreme Sniper Potion
(Raymond), Fried Chicken, Dried Squid and Supreme Dexterity Potion (Max) carry no rank on the source page
and in `data/shops.txt`, so they were sold to anyone. The owner chose Traveler+ (that town's citizens, any
grade) over leaving them open; the six rows are tagged and every row of the six town-hall shops is now
gated (`shops.rs` test, 46 gated rows). And **Max's Elixir (3000, Citizen of Honor+)**, on the source page
and missing from the transcription, is added at the owner's word - 933 rows.

**2026-09-29: the citizenship contract closes after Sign - the server starts the stamp.** The owner's first
run: *"when I click on Sign, the contract never went away."* The signing worked (record, effect, notice
all in the log) - the WINDOW waits after OK for a `0x055B` type `0x47` force-close from the server: result
1 plays the stamp and it closes itself ~2 s later (sending the `0x47` answer we already swallow), result
0 closes it with no stamp. `FUN_1410DEA80` starts the stamp and its only caller is that branch
(`0x141F6F486`); research §5.4 had said OK plays it - corrected there. The contract answer now ends
with `script_force_close(1)` when the standing changed and `(0)` when it was refused. Tests: the Oath
ends with the result-1 close; a contract made stale behind the window ends with result 0 and signs
nothing; both fail without it. Fixture:
`research/fixtures/citizenship-oath-signed-but-window-never-closed-no-server-force-close-world.log`.

**2026-09-29: the Regular ship waits ten seconds, and the ferry is gone.** The owner: *"The before travel
should also last 10 seconds before players get teleported to during the ride for 1 minute. This should
happen in both directions"*, then *"in Orbis, the ferry to El Nath or Sleepywood should NOT exist"*,
*"Sleepywood to El Nath should NOT have a ferry"* and *"El Nath should only be accessible by foot or
teleport scroll"*.

* **Regular ticket, both routes:** Cherry / Rini now put the passenger in the route's Before Takeoff
  room - an instance of their own, keyed by the voyage like the ship - with a 0:10 clock
  (`boat::PRIVATE_WAIT_S`); the tick then moves them to their own deck for the 1:00 crossing. Purin /
  Erin work in that room as for Basic. Joel's, Agatha's and Cherry's lines say "10 seconds". Test:
  waiting room alone at 0:10, deck alone at 1:00, arrival - Orbis and Ellinia - and it fails with the
  wait removed.
* **The Ossyria ferry line is deleted** (`crate::taxi`): Eurek the Alchemist in Sleepywood and El Nath
  and the Platform Usher in Orbis are no longer taxis - Eurek says their own line again, and the Usher's
  menu is only the platform to Victoria Island. `Network::Ossyria`, `Voice::Ferryman`/`Wanderer` and
  `FERRY_FARE_MESOS` are gone; eight taxi rows remain, and no taxi stands on or goes to Ossyria. El Nath
  is reached from Orbis on foot (the Orbis Tower) or by scroll; the ships are the only link between the
  continents, and El Nath -> Victoria is walk to Orbis, then the ship. Nothing else warps into El Nath
  (`thirdjob::INSTRUCTOR_TOWN` is a name only); `!map` is the GM's. Plan step T11 now starts with
  `!map 20001000`.

**2026-09-28: citizenship is built - contracts, grade locks, Contribution and the Community Board.**
The owner: *"Great, make the implementation"*, and for the board: *"If the player has never done them, do not
show the quest as available for pick up. If the player has completed the quest, remain in the completed
tab until it is chosen again. If the quest has been chosen as the weekly or daily, they show up normally
to everyone ... If the quest was previously completed by the player, they become active again"*.
`research/citizenship-2026-09-27.md` §5-§7. **Unseen on a client** - plan step 25.

* **State**: `store::citizenship` (new table, created on open) - per character per town `state` (1 active,
  the client's; 2 frozen, ours), `grade`, `contribution`, `certified_grade`. Sent as quest **510000**'s ex
  record `st1=..;gr1=..;ct1=..` in the character record (**block #28, presence 16**, via
  `QuestBook::ex`) and live as `0x0089` **sub-case 13**. The client locks quests, shop rows and NPC grade
  lines off that string itself.
* **Arthur (229) / Roxy (425)**, Lv 12+, in their hall: the **contract window** (`0x055B` types
  0x42..0x46): Oath (never signed), Transfer (active elsewhere - the old town freezes), Reactivation
  (frozen here - 50,000 mesos [S], refused in a Say before the window if short), and for a citizen a
  menu: standing, or the Renunciation. OK signs (effect 83 CitizenshipGet); the stamp's second answer
  (`0x47`) is swallowed; the offer is recomputed at the answer. At most one active town, always.
* **Quests**: `Check.0.citizenshipTown/Grade` refused server-side (the client's `0x50` rule);
  `Act.1.citizenshipContr` banked on a recorded turn-in - flat or the formula, evaluated at the grade
  at turn-in - with `0x0089` **sub-case 35** ("You have gained ... Contribution"), and a grade-up at
  1000/2000/.../8000/10000 [S] plays effect 84 and says to see the clerk, who hands over the **Grade
  Update certificate (0x46)** on the next talk.
* **The board - solved in the client, research §7**: `FUN_14070FAE0` refuses (`0x51`) any group quest
  its group's record does not list: quest **510001..510004** = `q1_d=<id>|<id>` / `q1_w=<id>`. A completed
  group quest is re-offered by the client itself unless `doNotRepeat` (`0x19`), same day (`0x13`) or same
  Monday-week (`0x16`) - so the completed tab and "becomes active again" are the client's own behaviour
  once the record lists it. Posted, UTC: **one resident a day** (both halves - First Greeting shows once
  ever, Asking After after it) plus a town leader at grade 5+; **one donation a week at the character's
  own grade**; a shuffled order per cycle so each comes round once. The server refuses the same five
  cases, restarts a completed row (`store::restart_quest`, the only backwards move) and sends state 0
  with forget-completion before the accept. Re-sent from the tick when the day turns.
* **Shops**: the town-hall shops' `min_grade` rows now carry `+0x104` town / `+0x108` grade (the client
  draws them locked) and `classic_buy` refuses them - **the gate that was parsed and never read**.
* **At the Quest rate** (the owner, same day: *"10x as well, similar to the current 10x global boost"*): a
  board daily/weekly's Contribution is multiplied by the `!setrates` Quest field, the one quest EXP already
  used (so EXP is NOT multiplied twice); story-arc Contribution stays flat. **And quest mesos are paid at
  all now** - *"make the server pay for quest mesos at the 10x rate too for all quests"*: `Act.1.money`
  (255 quests, all positive) was read by nothing, so every turn-in showed mesos and paid none. Paid on
  a recorded turn-in, at the Quest rate, with the "mesos (+n)" line. Quest 10303's `Act.0.money -1000`
  (a cost to start) is still not read. Test: 10x -> 1500 Contribution and 3510 mesos for a grade-2
  daily, 50 flat + 3510 mesos for a story step, nothing on a repeat turn-in; fails with either piece off.
* `!citizenship [<town> <state|grade|contr> <value>]` (GM).
* **Not built**: discounts (shops, storage, taxis - which items are tagged is [S]), Character Info's
  CITIZENSHIP section, the Citizen-of-Honor announcement, earrings/housing. **Retracted**: research
  §2.1's "grade and level gates move together" - true of the 15 story quests only.
* Tests: `net` (block #28 placement, sub-cases 13/35, all five windows, the answer parser against a
  menu cancel), `store` (restart only from Complete), `world::citizenship` (formula at 1..10, clerk
  offers, one active town, groups vs the real `Quest.wz`, posting tiers, cycles, Monday weeks), and
  ten session tests end to end. Controls: each fails with its piece disabled (gate, contract routing,
  payout, shop refusal, the record in the book). The quest audit now skips the 71 board quests (their
  refusal box is the point) and audits the story quests as a top-grade citizen.

**2026-09-26: the Cash Shop's beauty coupon preview is filled - `0x05B9`.** The owner: *"the Mystery
Hair and Signature Hair Coupon should show previews."* The panel is client-side but its data is
not: it reads a coupon -> styles map at `0x143A410A8` that only the Cash Shop stage's `0x05B9`
(flag 0 -> `FUN_1401C2910`) fills - `research/cash-shop-stage.md` §6.4 had it filed as
"peripheral". **The client never reads its own `Etc/BeautyPreview.img`** (measured: no path or key
string in the image, four controls found). Found by walking up from the panel: `FUN_1410B3440` ->
`FUN_1401C34D0(gender, coupon)` -> the map -> its only writer -> the `0x05B9` arm; layout from the
listing and the decompiler agreeing read for read. Sent on Cash Shop entry, before the wallet:
Mystery/Signature Hair = the union of both salons' VIP/REG pools, Mystery/Signature Face = the
surgery pool, male and female - exactly what the coupons give (`salon::cash_shop_previews`). Tests:
the body walks the decoder's shape; every list non-empty for both genders, base ids, art present
(`beauty.txt`); entry sends it once, before the wallet. **Not seen in the client yet** - plan step 24.

**2026-09-26: back in at the nearest spawn point; a teleport lands on a random one.** The owner: *"spawn
the player to the closest spawn point where they last were before they disconnect, change channel, go
into cash shop ... store which spawn point"* and *"if a player is teleported into a map, the server will
choose a random spawn point. Such as when Nella teleports the player back to Kerning City"*. The arrival
portal was per-connection only, so every login landed at portal 0. Now: `store::spawnpoint` (a new
`spawn_point` table, created on open - the live DB gains it with no step) holds `(map, portal index)`;
recorded as the spawn point nearest `last_position` on log off / socket drop (`Drop`), at the channel
change request (the new channel can claim before the old teardown), on entering the Cash Shop, and on
every map change (so a crash before a step still returns there). `claimed_character` applies it only
when the stored map is still the character's map. **Spawn point = a portal named `sp`** with target 0
and no script - measured: all 426 maps have one, and the other target-0 names (`tp` x72 Mystic Door
points, `st00`, `h001`, Kerning's `pc00`/`cab00`) are not places to set someone down.
`Session::teleport` picks a random `sp` and is used by: Return Scroll, respawn in town after death,
`!map`, Nella and the First Time Together warps, Phil's ride, Shanks, taxis, the daily Henesys escape.
Unchanged (exact): portal walks, the PQ `st00` arrival, the Cash Shop exit, the 2nd-job exit onto
`job00`. Tests: real-data (Kerning's 15 `sp`, lookalikes excluded, nearest, random spread) and an end
to end (log off at 950 -> back at `sp` 3; map changed -> ignored; Cash Shop records; 60 teleports land
only on spawn points, both of them) - fails with the stored portal ignored and with Drop not recording.

**2026-09-26: the 2nd job test is a quest you accept - all four classes.** The owner, at the Magician
Job Instructor: accepting *Test of Qualification* should warp into the test map with the quest in
progress (30 Dark Marbles); a regular talk re-enters while it is active, says "not ready yet, talk to
Grendel" otherwise and "nothing more to teach" after the advancement; and leaving puts you beside the
instructor. **Measured cause** (`world-ch0.log` 02:02:55-02:03:57, three tries): `20102` is
`startscript q20102s`, which the client does not ship, so it hands the start to the server (action 4);
`Say.0` has three lines and no `yes` branch, so the third ended on OK - the quest was never recorded
and the existing warp (`enter_test_field_on_quest_start`, transition-only) never ran. Now: the last
opening line of a not-yet-started test quest is Accept/Decline (`opens_the_test`); Yes records the
start and warps. Regular talk is `secondjob::examiner_talk` (re-entry is a yes/no, re-checked on Yes);
the old click-to-exchange `TestStep::Pass`, which handed out the proof outside the quests, is gone -
the proof comes from 20x03's start as the data says. The warden now exits onto **`job00`**, the spawn
point each examiner map has beside its instructor (Magician's portal 0 is 6 000 px below them). Also
fixed: the warden's notice had a run of spaces mid-sentence (a lost `\`); ~a dozen other strings
have the same damage - flagged as a separate task. Test: `the_second_advancement_walks_the_client_s_own_chain`
rewritten on the Magician branch, every leg asserted; it fails with the Accept box off and with the
exit at portal 0. world 1259 passed.

**2026-09-25: the launcher says it updated, and has Copy logs.** The owner: *"When the launcher
auto-updates, it just closes and re-opens, and the user doesn't know what happened and doesn't
realize that they have to login again"*, and *"the launcher client logs needs a "copy logs"
button"*. `selfupdate::check_and_update` now only INSTALLS; the old window shows **"Launcher
updated"** - it will close and reopen, sign in again - and `selfupdate::restart_into` starts the
new one from `eframe::App::on_exit`, so OK, the X and Alt+F4 all restart it. The new launcher
(started with `--updated-from`) opens with its own "Launcher updated - please sign in again". The
window behind either dialog is disabled. The log pane has **Copy logs**: every line, oldest first,
`[info]/[ ok ]/[WARN]/[ERR ]` tags, under a header naming the launcher's folder, with a "copied N
line(s)" confirmation. Tests: the copied text and the dialog wording (launcher 175 passed). **Not
seen on screen yet** - egui's layout is not checked by a test; plan step 21.

**2026-09-25: an unchanged launcher no longer "updates" every player.** The owner: *"whenever we
package the server and update the server, it causes the launcher to update itself when we didn't
change anything about the launcher."* The launcher compares its own SHA-256 with the server's
(`crates/launcher/src/selfupdate.rs`), and two things changed the bytes of an unchanged launcher:
**(1)** the MSVC linker stamps the link TIME into the PE header, so any relink was a new file; **(2)**
`maplecw-auth` rewrote `auth-cert-fingerprint.txt` at every start (same text, new mtime), and the
launcher's `build.rs` watches it, so the next package recompiled and relinked it. Measured in
`target-static` with the packaging flags: touching that file alone moved the hash
(`28613a5d` -> `96d414ad`). Fixed: `/Brepro` on the launcher and on `grap64.dll` (compiled into it),
and `tls.rs` writes the fingerprint file only when its text differs. After: touching the
fingerprint file, `main.rs` or the stub's source leaves the hash byte-identical; a real one-string
change moves it and reverting restores the original exactly. Tests: auth's reload test backdates
the file and fails if a restart touches it (checked with the old behaviour). launcher + auth +
grap-stub: 335 passed. **The first package after this ships ONE last launcher update** - the new
bytes carry `/Brepro` - and none after that unless the launcher (or code it uses from `store`,
`patchset`, `tlspin`) really changes.

**2026-09-25: a pet's feed line is the same on every screen.** The owner: *"Please relay these pet
packets so everyone see the pet feed speech bubbles. If possible, sync the chat bubbles so that the
dialogues are the same."* `0x0203` is the owner's client reporting the line its pet said after a
feed (`research/pet-line-report-0x0203-2026-09-25.md`); `0x0279` is its mirror - `FUN_141ec3fa0`
performs the same two bytes and string with flag 0, so nothing is reported back **[L]**. Now a feed
sends `0x027E` to the owner only and HOLDS the map's copy; the owner's report is relayed to the
field as `0x0279` byte for byte, so every screen shows the owner's line - the map's own `0x027E`
made each client pick a random one. One report per feed, within 2 s (`PET_LINE_WAIT_MS`), at most
80 characters; any other `0x0203` is logged and dropped (a bystander's, a repeat, no feed). No
report in time -> the held `0x027E` goes to the map as before. A field change drops the pending
feed. Test: `a_feed_and_the_level_it_earns_are_seen_by_the_other_player` covers the relay, the
drop and the fallback (fails with the dispatch arm off). **[I], watch on the run:** that a
bystander's `0x0279` plays the EATING animation as well as the line - the performer gets the
same bytes the owner's did, but the second is written as 0 when below 9.
`c0f1d87` (scrolls to 0.5%) had left
`session::tests::the_tutorial_sentinel_always_drops_its_shellpiece` failing - it pinned global rows
at 1 bp. The owner: *"Accept the new drop rate."* It now pins them to `scrolls::GLOBAL_DROP_CHANCE_BP`;
the tutorial Sentinel drops an extra scroll about once in 100 kills. world: 1259 passed, 0 failed.

**2026-09-25: a pet's Auto HP / Auto MP drinks the potion.** The owner: *"the pet attempts to drink
the potion for the player, but the client never actually performs the restoration ... potions are
never consumed and clients never recover."* The pet's request is **`0x0206`**, and it had no
handler: the deployed server logged 25 from Moth (`world-ch0.log.4` and `.3`, 2026-09-18..19; excerpt in
`research/fixtures/pet-auto-potion-0x0206-unanswered-25-times-deployed-world.log`) as
UNKNOWN and answered none. Body from the builder's listing (`FUN_142cca680`, control: `0x0114`'s
builder read back its known shape): `u8 pet, u32 tick, u16 slot, u32 itemId, u32 literal 1` - 15
bytes, matching both captures (Red Potion slot 8, Blue Potion slot 6). **The builder then sets the
exclusive-request latch** (`mov [rbx+0x2330],1` at `142cca8cc`), so each unanswered drink also
froze every later inventory action for the session. Now `net::useitem::CLIENT_PET_USE_ITEM` ->
`Session::on_pet_use_item` -> the same walk as a double-click (slot checked against the bag, capped
restore, stack shrinks, latch cleared); a non-potion is refused and still answered. Tests: the two
real captures parse; the session test goes through `handle` and fails with the dispatch arm off.
**Not yet looked at:** `0x0203` (59 UNKNOWN in the deployed logs) and a second `0x0205` shape (33) -
both pet-family builders.

**2026-09-25: Innocence keeps a mob drop's roll.** The owner: *"Can we make Innocence Scrolls keep a
good base roll?"* A rolled drop now remembers its roll as its own base - `store::Item::rolled_base`,
a nullable TEXT column (`rolled_base`, 17 stats comma-separated) on `inventory`, `equipment`,
`storage_item` and `cash_locker`, travelling with the item like `failed_slots` - and Innocence
reverts to THAT. An item that never rolled (NULL: anything not from a mob, and every item from
before variance) reverts to the template exactly as before. Chaos still decides which stats exist
from the template. Real scrolls on a bagged item carry the column over (they rebuild the `Item`).
**Migration verified on a copy of the repo's maplecw.db**: the column appears on all four tables
on open, every row kept, all NULL. Tests: the store round trip (bag -> worn -> bag), and
`innocence_reverts_a_rolled_drop_to_its_roll_and_anything_else_to_the_template` through
`apply_scroll` - fails with the override disabled. store + world + login: 1731 passed, 0 failed.

**2026-09-24: item variance - every equip a mob drops rolls its stats.** The owner, with a rules
sheet: *"I want to introduce item variance for any items dropped by mobs following these rules
(including those dropped by party quests such as Slime Shoes)."* `crate::variance`: range =
reqLevel / 10 (x2 for an overall, `105xxxx`); each stat the clean template HAS rolls down /
unchanged / up at 1/3 each, then a uniform distance up to its cap - primes share the range
(range / how many of STR DEX INT LUK the item has), WATK MATK Speed x1/2, ACC Avoid x1, Jump
x1/4, HP MP WDEF MDEF x5; round, floor at zero. Crit is not on the sheet and does not move;
upgrade slots are the template's. Wired at both mob-drop mint sites in `session/combat.rs`
(the shared row, and personal drops - each member's Squishy Shoes rolled on its own); a cash
equip, an id with no template and a Lv 0 item come out unchanged. **Not** applied to reactor
boxes, shops, NPC rewards, crafting or `!item` - only what a mob drops. Every roll is logged
(`variance:` lines, the template beside it). `EquipTemplate` grew `req_level` (equips.txt column
19; 1166 of 1799 equips have one). Tests: the rules one at a time in `variance.rs`, a real kill
(`a_mobs_equip_drop_comes_out_with_rolled_stats`, with the Lv 0 sword as the unchanged control)
and the King Slime's shoes; both end-to-end tests fail with the roll switched off.
Innocence keeps the roll - see the entry above.

**2026-09-24: Nella on the Exit takes every Pass and Coupon.** The owner: *"when people talk to Nella
on the Party Quest exit map, Nella should remove the player of any Passes or Coupons they may have.
They may not be taken outside of the Party Quest area."* `firsttime::STAYS_IN_THE_QUEST`; taken the
moment they are spoken to, before their question and whatever the answer, each with its grey "lost"
line. Nella INSIDE the quest takes nothing. They are the only way out - a login on a stage lands on
the Exit. The Nella test holds an unrelated Etc item as a control, and fails with the list emptied.

**2026-09-24: the King Slime's shoes land beside the Pass, in one slot every member shares.**
The owner: the per-member Squishy Shoes dropped *"right on top of the pass"*, and *"the clients should
also not have weird drop placement such as empty spaces where they do not see a drop they can pick
up because it's instanced for someone else."* The shared row (Pass, mesos) and the shoes were each
centred on the corpse on their own, so the first shoe slot was the first Pass slot. Now
`Session::party_quest_personal_drops` hands the shoes to `drops_from_kill_for`, which lays the row
out with ONE extra slot at its end and puts **every** member's pair in it: each client sees exactly
one pair there - its own - so every screen shows one even row with nothing stacked and no hole.
`the_king_slimes_shoes_share_one_slot_after_the_pass` checks each member's visible row is evenly
spaced; against the old placement it fails with a shared drop under the shoes.

**2026-09-24: the last stage of First Time Together drops Passes and mesos, nothing else.**
The owner: *"Jr Necki and Cursed Eye on the last stage ... somehow drops "Coupon" items? The only thing
they should drop are Passes and mesos."* The scrape (`data/drops.txt`) had given all three stage-5
templates - 800001, 800002 and the King Slime 800003 - a 6% row for `4001001` Coupon, stage 1's
item. Removed; the mesos are the level default. `the_last_stage_always_drops_passes_and_never_shared_shoes`
now fails on any row besides the Pass and mesos, and was checked failing against the old table.
The global scroll rows (0.5% since 2026-09-25) still roll for every mob, these included.

**2026-09-24: Maple Chat works across channels, through the hub; buddy chat is built.** The owner,
with the deployed server's logs in `Desktop\Server Investigation`: *"an invite was attempted,
but ... the client was not able to accept even when they tried to accept it, the server replied
back you are too busy"*, *"buddy chat does not work"*, and *"Maple Chat should work cross
channel, please use the hub code."*

* **Maple Chat, measured:** Cobalt opened room `0x20001` on channel 1 (`world-ch1.log`
  04:32:58); the invite reached Moth on channel 0 through the hub; their Accept arrived at channel
  0, whose per-process room registry did not hold the room - *"this channel does not hold
  (another channel's room ...)"*, mode 0 result 1 (`world-ch0.log` 04:33:04). The invite used
  the hub and the room did not. **Rooms are now `world::messenger::Rooms`, a replica per channel
  kept in step by the hub exactly the way parties are**: Open / Enter / Leave / Disconnect are
  `Frame::MessengerRequest`s the hub applies and echoes to every channel in one order, and a
  late channel gets `Frame::MessengerSnapshot`. The actor's channel builds the actor's replies
  from the echo; every OTHER member is told by the channel that hosts them, so each member is
  told exactly once and a dead channel's players are taken out of their rooms by the hub. A chat
  line and an invite change nothing and read the replica. `tests/messengerlink.rs` replays the
  production failure through a real hub - a room made on channel 1, accepted on channel 0 - and
  the window opens.
* **Buddy chat:** `0x0179` kind 0 was "not built; went nowhere" (`world-ch0.log` 04:32:28,
  Moth's `'hewwo'`). It now goes to every **accepted** friend on the server's own list, on any
  channel via the hub - a request still waiting is not a buddy.
* **Deploy all of it together.** Two new hub frame kinds (7 and 8): an old `maplecw-chat` skips
  them as unknown, so upgraded channels behind an old hub would open no room at all, silently.
  Hub, both channels, same build. net 673, world 1214 + both link integration tests.

**2026-09-24: one price rule for the whole Cash Shop.** The owner: *"make sure everything in the Cash
Shop costs 100 LP and does not have duration with the exception of shop merchants. 7 day shop
merchants should cost 700 LP, 1 day shop merchants should cost 100 LP. Collaboration signature
style packages should maintain their price."* `tools/cash_wares.py::price_rule` is that
sentence, applied by `backport_install.py` step 4f to the classic shop's own rows as well as
ours: **102 shipped rows** changed (90-day clothing, permits, emotions and effects made
permanent; the 1000 LP Megaphone and weather x11 bundles to 100, with `originalPrice` and
`discount` reset so they do not draw as a 91% sale), Cozy Coffeehouse and Granny's Food Stand
(7-day merchants) to 700. **The four collaboration pets stay at 1000 LP** (the owner: *"collab pets
should remain at 1000 LP"* - they were briefly swept to 100). Result, all 653 rows on sale: 637
at 100 LP and permanent, the four collaboration pets at 1000, three 7-day merchants at 700, the
1-day Mushroom House Elf at 100, the Signature Style box at 800 and its eight coupons at 200. "Shop merchants"
was read as the hired merchants (503); store permits (514) follow the general rule.
`commodity::tests` checks the rule over every row. Installed; `--check` passes.

**2026-09-24: deleting a cash item works.** The owner: *"I just tried deleting an item in Cash Shop,
but this is currently unhandled."* The trash button sends `0x03E1` sub-op `0x1C` with the
locker serial - `1c 01000000 01000000`, this server's own `(account << 32) | slot` for account
1, slot 1 - and it was refused with the generic `0x3D`, which is the *"Due to an unknown
error"* on their screen. It was already decoded (`research/cash-shop-actions.md` section 5); it
had never been built because of one open caveat, which the later arm table closes: the success
reply `0x05AE 0x3C` erases the item and says *"The cash item has been deleted."* but **does not
clear the in-flight latch `[stage+0x74]`**, so on its own it would leave the shop refusing
everything after one delete. The server now checks the serial is this account's and names an
occupied slot, deletes the row, sends `0x3C`, then the wallet `0x05AD` - whose arm clears the
latch and cannot re-trigger a purchase, because `0x3C` has already reset the pending kind.
Refusals stay `0x3D`, so a multi-select delete keeps the rest of its queue. Plan 15(e).

**2026-09-23: the Cash Shop sells 645 items, up from 162.** The owner: *"Add all of the items that
are not listed but named except those that are part of the collaboration signature sets since
they come from the Cash Coupons instead"*, then *"Make sure all cash equipment items do not
have time duration, and all newly added items costs 100 LP to purchase."*

* **Audit first** (`tools/audit_cash_items.py` -> `gm-handbook/cashaudit.txt`): 927 items carry
  `info/cash = 1`; the shop sold 162. The rest were 16 switched-off placeholder rows (the
  `92xxxxxx` block), 521 named items with no row, and 228 with no String.wz name at all.
* **483 rows added** by `tools/backport_install.py` step 4f, from `tools/cash_wares.py`: every
  named cash item the **pristine** client does not sell. Read from the untouched original, so
  a re-install produces the same rows and `--check` stays byte-exact - read from
  client-patched, they would all be "already listed" after the first install and vanish from
  the next. The collaboration items do not exist in the pristine client; the manifest guard
  (mapped through `HAIR_HAT_RENAMES`) must come out empty and stops the build if it does not.
  521 - 35 manifest items - 3 renamed hair-hats = 483, the same number from both directions.
* **Placement copies the classic rows** for the same kind of item: clothing by slot (Gloves 407
  and Effects 410 were empty tabs and now are not), face vs eye accessories 408/409,
  permits and hired merchants under Convenience, dye coupons under Beauty / Misc, emotions
  under Expressions. **Gender** is the client's own id rule (`FUN_140253130`) - **for equips
  only**: the first build read that digit on `5xxxxxx` ids too and locked `5010000` to male; a
  spot check of the built `Commodity.img` caught it before anything was installed.
* **No cash equipment has a duration, and every new ware is 100 LP** (the owner's rules). That
  zeroes the Period on the classic shop's own 84 clothing rows as well. The server never
  applied a period (`Commodity::period_days` is only logged), so purchases were permanent
  already; this makes the shop say so. Non-equipment keeps its classic duration (permits and
  emotions 90 days, hired merchants 7).
* Installed, `--check` passes, `commodity::tests` pins 674 rows and both rules. **The running
  world servers still hold the old table** - restart them - and the **client package must be
  rebuilt** (`tools/make-installer.ps1`) or other players' shops will not show the new rows.
  Plan step 15.

**2026-09-23: JOB, LV and the location line, read out of the client rather than guessed.**
The owner: *"The buddy list still does not have Job and level"*, and the location *"is reflecting in
chat, but it should be where it says 'Tester2 - Checking location'."*

* **JOB/LV: my tail layout was wrong, and the screen said so.** The row builder was found by
  scanning for every `imul ..., 0x149` (27 sites, all friend code), then xref-ing the one
  accessor outside the manager, `FUN_142cc3ea0(ctx, i)`. Its caller `FUN_1411be0a0` reads
  `rec+0x139` into the row as an int (LV) and hands `rec+0x13D, rec+0x141` to
  **`FUN_1402b0250(job, subJob)`**, which is the job-name lookup - a map keyed `0 ->
  "Beginner"`, `100`, `110`... with a `job == 400 && subJob == 1` Dual Blade case. So
  **`0x139` level, `0x13D` job, `0x141` subJob, `0x145` status**, all **[L]**. The same builder
  reads `0x12`, `0x16`, `0x2C` and `0x39`, confirming the rest. The reference's `inShop` does not
  exist here; it was right about 313 bytes, which is what made its last field look safe.
* **The location line needs mode `0x48`, not `0x09`.** `FUN_1418486b0` is `switch (mode)`, the
  find arm is `case 9: case 0x48:`, and inside it `mode & 0x40` picks the window over the chat
  log. The window asks with kind `0x44` = `0x40 | 5`; the answer must carry the same bit.
  Sending `0x09` is what printed *"'Tester2' is currently at 'Victoria Road : Kerning City'."*
* **[I]:** whether the channel in place 3 is 0- or 1-based. It goes out 0-based; plan 12(c2)
  asks. net 672, world 1188.

**2026-09-23: a player's drop is everybody's, and an untradeable one vanishes.** The owner:
*"users dropping items publicly in the field, but nobody except themselves were able to pick up
what was dropped on the ground"*, then the rule: tradeable items and mesos stay until expiry
for **anyone**; untradeable ones *"should just disappear, there should be an animation for it
on client side and also broadcasted to other clients as well."*

* **The pick-up bug was the packet, not the rule.** The server had allowed it since 2026-09-05
  (`public`, `may_be_taken_by`); the `0x046E` said otherwise - `ownType = 0` (user) with the
  dropper as `ownerId`, i.e. *"this is somebody else's"* to every other client. A public drop
  now goes out as `OWN_TYPE_EVERYONE`. `research/item-drop.md` had called `ownType` "stored and
  never tested"; `tools/fieldrefs.py 0x70` over the drop code finds **two reads after the
  store**, so that line was an absence and is corrected.
* **An untradeable item a player drops is a disposal**: drawn landing on every screen, taken by
  nobody (the dropper included, and silently), faded for the whole field by the expiry sweep
  after `VANISH_MS` = 1.5 s. A trade-blocked item a *mob* drops is untouched - still the
  killer's alone.
* **And a second cause, found while doing the fade:** the re-send to a player who *enters
  the map after* a drop filtered every drop on `may_see_drop(owner, viewer, party)`, which
  knows nothing of `public` - so a late arrival was never shown a player's drop at all.
  Fixed; a disposal is not re-sent to anyone.
* **The disposal uses the client's own disappearing animation: `0x046E` enter type 3.** The owner:
  *"There should be a separate animation that client should be able to animate where the drop
  fades out."* `FUN_141790f80`, the drop's per-frame update, tests `enterType == 3` four times
  with its own layer calls, the source block is read for it (so it still leaves the player's
  hand), and `drop+0x61` is clear so no client even offers the pick-up. The server's `0x046F`
  after `VANISH_MS` stays as cleanup, in case the client does not destroy the object itself.
* **No capture of the failure exists**, and that is worth saying: no archived run has a second
  player sending `0x032C` for someone else's drop, so "the client refused locally" is inferred
  from the packet, not observed. The end-to-end test proves the server half; plan step 14 is
  what proves the client half.

Also, from this run of the suite: **the friends / whisper world-side changes of 2026-09-22 now
compile and pass** - they had been blocked by the instancing refactor landing underneath them.
net 672, world 1176.

**2026-09-22 (late): presence, a red line, and a 50-buddy ceiling.** Four more of the owner's
reports, three of them one packet.

* **`0x00A7` sub-op `0x2D` is the presence notify**, and it closes three at once: *"Tester2 was
  not able to check the owner's current map location"*, *"once the owner logs off, the buddy list also
  remains showing the owner is still online"*, and *"when Tester2 logs in after the owner, the owner was not
  informed."* Body `u32 id, u32 accountId, u8 status, u32 channel, u8 account, u8 announce`.
  The arm **only acts on a change** - it compares the row's `+0x12` (channel) and `+0x145`
  (status) and returns silently when both already match - so the list goes out first (it
  carries `rec[0x11]`, the flag that greys the row, and leaves `+0x145` at zero) and the
  `0x2D` second. Sent the other way round the line would never be said. Every friend on the
  channel gets both on a login and on a logout, the logout quietly.
  * **The login notice fires on the FIRST FIELD ENTRY, not at claim.** Presence is registered
    when the character enters a field, so a notice sent at claim tells everyone the character
    is *offline* - which is exactly what the test caught. `Session::announced_presence` keeps
    it from firing again on every portal.
  * Two more confirmations of the record layout fell out: `FUN_142debab0` matches `rec+0x00`
    against a character id, and `rec+0x28` against an account id **for rows whose `rec[0x11]`
    is 5..=8** - which is the reference's "5 through 8 = account friend" exactly.
* **"%s is now your friend" is a red system line now.** The owner: *"can we send it as a red system
  message?"* It can, and the colour is inherited rather than chosen: `0x00AC` **type 5** is two
  instructions - `FUN_1415eca30(&text, 0xb)` - and kind `0xb` is the kind the client's own
  `0x32` *"%s has declined the friend request."* prints with, the line already on the owner's screen
  in that colour. The sentence itself has to be ours: **all 6165 strings were searched and
  there is no "is now your friend"** among them.
* **The buddy list holds 50.** `store::friends::FRIEND_LIST_LIMIT`, refusing the 51st with the
  client's own *"Your buddy list is full."* The window header still reads `[n/0]`: `0x2F`
  writes the client's own max to `ctx+0x118b` but also pops *"Your friends list has increased
  by %d slots! Your wallet is %d Mesos lighter"*, so it is the **purchase** result, not a way
  to state a capacity. Whatever normally writes `0x118b` is not found yet - a display gap, not
  a limit gap.

**Still open:** the window's **JOB and LV columns are blank**, because three of the record's
four tail words (`0x139`, `0x13D`, `0x141`) go out as zeros and which is which is **[I]**.
`0x2D` writes all three but **only when `account` is set**, so it cannot fill them for an
ordinary friend - they belong in the `0x15` record. The next step is one scan: find what reads
`rec+0x139` / `+0x13D` / `+0x141` in the row drawer. net 668, store 377, world 1163.

**2026-09-22 (evening): the buddy list DRAWS, and answering the group report was an infinite
loop.** On screen at last: the Buddy tab lists *"Default Group (1/1)"* with the owner in a
NAME / JOB / LV row, so the 329-byte record's measured half is right and `0x15` is the list.
Three things came back with it.

* **The loop, and it is why the client lagged and the window froze.** The owner: *"the owner's client
  started lagging a lot after the friend request was accepted"*, *"the owner opening the buddy list
  crashes/freezes the client"*. `world-ch0.log` reached **42 MB in one sitting**: **32 566**
  round trips of `0x0193` sub-op `0x14` -> `0x00A7` `0x19` + `0x15` -> sub-op `0x14`, one per
  millisecond. The cycle is structural - every list reply ends in a window refresh, and a
  refreshed window hands its group names back - so **sub-op `0x14` is a report and is now
  answered with nothing**, the same as `0x013D` and `0x00B8`. "Always answer" is about a
  request the UI waits on; this is not one, and the window draws correctly without a reply.
* **A timeout.** The owner: *"it should have a timeout if not accepted within a certain amount of
  time."* 60 s, measured from **when the balloon was raised**, not from when the request was
  made - counted from the asking, every request made while the target was offline would expire
  before its balloon could be drawn. Both sides get the client's own `0x2A` *"The request to
  add a Friend has been canceled."*, both rows go, and the store's guard decides so an answer
  that arrives first wins. A logout before the minute is up is not an answer: the row survives
  and the next login offers it again with a fresh clock.
* **A correction.** This entry's predecessor said `0x19` *empties the record array the window
  draws from*. It does not - it clears and refills its own `{id, name}` map at `manager+0x18`;
  the record arrays at `+0x00`, `+0x08` and `+0x10` are untouched. The order the two go out in
  is a preference, not a requirement, and the doc comments that said otherwise are fixed.

**Still open, and all four are one packet - `0x00A7` sub-op `0x2D`:**

1. *"Tester2 was not able to check the owner's current map location"* - the window says
   *"The owner - Checking location"* and the chat says *"the owner is not online on any channel."*
2. *"Once the owner logs off, the buddy list also remains showing the owner is still online."*
3. The **JOB and LV columns are blank** - that is the unknown 12-byte tail going out as zeros.
4. *"`Tester2 is now your friend` should also show in client opcode instead of a message we
   write"* - and note there is **no such string in the table**: all 6165 were searched, and
   the nearest the client owns is `0x03EE` *"[Friend] %s has logged in."*. So this one needs
   the sub-op found, not just swapped.

Case `0x2D` is already decompiled and it answers 1-3 directly: it reads
`u32 id, u32 accountId, u8, u32 channel, u8 hasDetail, u8`, finds the record by id, and when
`hasDetail` is set reads `str name, u32 -> rec+0x139, u32 -> rec+0x13D, u32 -> rec+0x141`,
each with `-1` meaning "leave it alone". **That names three of the four tail words**, and the
two the window shows blank are almost certainly among them. `research/friends-2026-09-21.md`.

**2026-09-22: the friend request popup CRASHED the client, and the fix decoded the 329-byte
record.** The owner: *"Adding someone as a friend causes a fatal client crash to whoever the
invitation was sent to."* The client named the packet itself - `0x009E CLIENT_PACKET_REJECTED`
class 1 reason `0x26` ("a decoder asked for more bytes than the packet had left"), echoing our
`0x00A7` verbatim, and the connection dropped 3.5 s later. The `0x1A` arm reads seven fields,
**exactly the 28 bytes this server built**, and then one **329-byte friend record** off the same
packet. Three offsets are measured off `FUN_142dec8f0` (`*rec` = the id, `rec+4` = a C string,
and **`rec[0x11] == 1`** is the test that raises the balloon at all), the rest matches the
reference tree's `Friend.encode` field for field, and its `FriendFlag` enum makes that `1`
`FriendRequest`. Four instruments, one layout - `net::friends::FriendRecord`.

**And `0x19` was never the list.** Its arm fills an id-to-name map and *clears* the record
array the window strides over, so every field entry had been handing the window an empty list.
The list is sub-op **`0x15`**: `u32 count` then `count * 0x149` bytes in one read. Both go out
now, `0x19` first because the other order throws the list away, accepted friends only, with the
online flag and channel from the roster. A waiting request stays an invitation. Plan 12.
`research/friends-2026-09-21.md` section 6. net 669, world 1156.

**2026-09-22: there is no Maple Chat typing indicator, and the first answer to that was not
evidence.** The owner: *"there's no typing indicator when a user is typing a message."* The earlier
write-up said so on the strength of eight `0x01FD` builders in
`research/msexe-send-opcodes.txt` - which is a **miss in a census whose own header says in
capitals that a miss in it is not evidence**. Re-measured three ways: `tools/builder_scan.py`
(new; a byte scan for immediate `edx` loads into the packet constructor, control re-finds
**38/38** census sites for `0x017E` before it reports anything) finds **12** sites, of which
two are real builders the census missed - `141183150` writes mode 1 and `141183200` mode 3, so
the modes are `{0,0,1,1,3,3,5,7,8,8}` and none is typing; `UI_000.wz/MapleChat.img` has 34 node
names and no typing state or canvas; and none of the 6165 decrypted strings says "typing". A
feature needs a packet, art and words, and this client has none of the three.

**2026-09-22: the friend request is a POPUP, and the chat commands are gone.** The owner:
*"there should not be any chat commands. Please use the client's built in UI elements, there
should be one similar pop up just like the party invitation, chat invitation, or trade
invitation."* There is one, and it is the same balloon family those three use: string
`0x0438` *"Friend request from"*, drawn by balloon kind `0x0E`
(`FUN_14180e3d0`, one of the 31 writers of `balloon+0x300`), raised by **`0x00A7` sub-op
`0x1A`** - found by walking the string id to `FUN_1418099f0` and its setter's callers back to
the `0x00A7` handler. **Both buttons are decoded too**, which is what retired the workaround:
Yes calls `FUN_141829a70` = `0x0193` **sub-op 2**, No calls `FUN_1418297d0` = **sub-op 6**,
both echoing the `u32` the server put in the popup - so this server puts the requester's
character id there and the answer names the pairing by itself. A refusal sends the asker
`0x32` *"%s has declined the friend request."* The balloon is raised once per session per
requester (`Session::friend_popups_raised`), not on every map change. `!friend` and everything
under it is deleted. Plan 12. Never on a screen.

**2026-09-21: the friend list, wired.** `0x0193` in, `0x00A7` out - `net::friends`,
`store::friends` (two directed rows per friendship, so each side has its own group and the
pending half is a state rather than a column), `session/friends.rs`. A request is recorded,
the asker gets the client's own *"Buddy request successfully sent to %s."*, the target's
session redraws its list and says who asked (a bus `Event::FriendRequest`, or their next
field entry when they are on another channel), and every refusal is one of the eleven
sentences this client already owns (`0x1C`..`0x30`, decrypted). **Two things are not
decoded**, both marked in the code: which of the window's `2`/`3`, `4`/`5`, `6`/`7` sub-ops is
Accept (**answered 2026-09-22: 2 for Yes, 6 for No**) - and the **329-byte record**
`0x15`/`0x18` carry, which is why the list goes out as `0x19`'s `{id, name}` rows. Whether
those rows are what the window draws is the one **[D]** left: plan step 12(b) is the reading.

**2026-09-21: the friend list is `0x0193`, and this server has never had one (the decode).** The owner:
*"Tester2 just tried adding the owner as a friend, but nothing showed up on the owner's screen."* The
client did send it - `world-ch0.log` 01:35:06.649, `0x0193` sub-op 1 with `str "Wisp"` and
`str "Default Group"`, once, and unanswered because there is no friend code in this server at
all. Decoded and written up in `research/friends-2026-09-21.md`: the ten outbound builders and
their sub-ops (1 add by name + group, 2/3, 4/5, 6/7, 0x0B, 0x0C, 0x12, 0x13, 0x14 = the group
names), and the inbound `0x0193` shape (`u32 kind, u32, u32, u32, str, u32`; kind 7 =
*"Not Find"*). **The gating unknown: nothing on the game socket populates the friend window's
rows** - `0x0193` carries one entry and no count, and no other dispatch case reaches that
code - while the executable carries a whole `CNM*Friend*` Nexon social layer and an
*"Account Friends"* string. If the list is served there, answering `0x0193` moves the request
flow and never draws a list. One Ghidra pass decides it; **NOT WIRED**, nothing built.

**2026-09-21: the pet's phantom "+1 Closeness" on a map change, and what the number was.**
The owner: *"if the pet has some sort of closeness, a message of +1 closeness still erroneously
show up bottom right on the screen, despite not actually adding any closeness."* **The number
is the closeness itself** - Lucy's is 1, read out of the `0x0070` body in `world-ch0.log` at
01:13:05.886 - so the client was reporting a rise from 0 to 1, honestly. `FUN_141ec4f60`, off
the local user's full refresh, reads the pet's cached closeness, reloads the pet from its Cash
item and prints string `0x1AC` *"%s's Closeness has increased (+%d)"* with the difference
(`0x1AD` for a fall): **[L]**, and there is no quiet path. A field entry clears the client's
bag - which is why `restore_bag_and_mesos` exists - so `CPet` was being built with no item to
read and the post-summon write then took it from 0 to 1. The entry now sends the pet's item
**before** the summon as well as after; the second write stays, because it is the re-read that
made the vacuum work (2026-09-18, on screen). **[I]** that a pet reads its item at
construction; the measured support is that only one line appears per map change and the
first-move re-summon is silent. Plan 8(d) discriminates: feed the pet first, so a line that
still appears reads the real closeness rather than 1. Never on a screen.

**2026-09-21: crafting - the six professions, the Crafting Journal, and the quests that
open its tabs.** The owner: *"We need to implement crafting in our server. After these quest
completions, they should unlock the appropriate crafting menu within the client."* The
window is the client's own, over `Etc/CraftRecipe.img`; it asks `0x02F6` *"may I start
this"*, runs the recipe's `ProcessTimeMS` animation itself, then asks again, and this server
answers `0x0398` and owns the bag, the mesos and the mastery. **Nothing is taken until the
second packet.** A profession is an ordinary skill (`92000000`..`92050000`) whose `level`
field packs the mastery: `(level << 24) | exp` - that packing is the whole reason the tab
unlocks and the bar fills, and it happens in one place, `Store::skills`. The recipe key is
the client's own `(level + profession * 10) * 1000 + index`; `tools/dump_craftrecipe.py`
writes all 348 into `gm-handbook/craftrecipes.txt`. Quest `Act.1.skill.<n>` is read now (18
rows, all six professions, three quests each) and grants the profession plus its mastery.
`!craft [profession] [level] [mastery]` opens a tab without the quest. The mastery curve is
the client's own `50, 166, 319, ...`, which disagrees with meowdb's table - settled by the owner:
*"settle for the EXP curve in the client, since that's the source of truth for the client
display."* **A quest finished before any of this existed is backfilled at the claim**, at
level 1 with an empty bar (the mastery is not replayed): the owner's Woodcrafting tab still read
*"Vicious in Henesys is looking for an apprentice"* on a level-12 character, because
`Act.1.skill` was read by nothing until today and a completed quest is never turned in
again. **The window is opened by a client keybind** - no packet opens it, which is why none
was found. `research/crafting-2026-09-21.md`, `net::craft`, `store::crafting`,
`world::crafting`, `session/craft.rs`. Plan 11c. Never on a screen.

**2026-09-19: Gift Drops - `!giftdrop` and `!giftall`, through the Administrator's box.** The owner
wanted the modern Gift Drop window for compensation. This client has no such window (no UI
image, none of its strings), its own mailbox window has no way in that any scan found, and the
Cash Shop locker was refused for non-cash items; so the Administrator's type-6 menu, the way
`!tool` works. `!giftdrop <player> <item> [n] [msg]` queues for one character; `!giftall <item>
[n] [msg]` queues one row per account, claimable on any of its characters once; both expire
after seven days (`store::GIFT_TTL_SECS`, stamped on the row). The box offers Claim (room
checked first with the quests' own refusal; the row settles BEFORE the item moves), Refuse
(gone for good) and Cancel (kept, for later or another character). An online target's box opens
at once (a bus `Event::GiftDrop`); an offline one's on the first move after their next field
entry, never with the `SetField`. `store::gifts`, `world::giftdrop`, `session/giftdrop.rs`; the
`gifts` table is in the item-id rename list. Plan 11b. Never on a screen.

**2026-09-18: the collaboration pets declare Item Pouch alone, like the classic eleven.** The owner,
off Lil Fern's shop tooltip (*"Skill: Meso Magnet, Item Pouch, Auto Move, Auto Buff"*): *"have
them match existing pets and should only have Meso Magnet and Item Pouch at default purchase
time."* Nexon's modern image declares `sweepForDrop` and `autoBuff`; the tooltip lists a skill
the image declares OR the item body has learned (`FUN_1414b89b0`), so both keys are stripped by
the installer's step 4d (983e79f, with 4d's patch rows moved out from under the badge lookup -
before that only the last pet got them at all). The server side was already uniform: every pet
row starts with `PET_SKILLS_AT_START = 0b0001`, and the Petite Luna vacuum is the item's
`wonderGrade 6`, not a key. Installed 02:10 with the client closed and read back: all four
declare `pickupItem` alone, the c22/c23 commands are in, `--check` passes, 2605 tests. Plan step
8's collab block has the reading. Unverified on screen.

**2026-09-18: the three hair-hats were female-only by ID; they wear 1007910..1007912 now.** The owner:
*"the Linie, Lugner and Aura package hair equipments still is being refused to be equipped by
the client."* Read this time rather than instrumented: the double-click is `FUN_141784fa0`, which
calls the equip function only when `FUN_142d44b20` names a body part, and that resolver drops
the item when the client's gender-from-id rule disagrees with the character. `FUN_140253130`
reads the FOURTH digit: 0/5 male, **1/6 female**, else unisex - so 100**6**910..912 were female
caps, every test character is male, and nothing was ever sent (which also retires §5's
`FUN_1417dd7e0`: that is the drag-from-a-worn-slot path). The three range predicates in front
of the digit are hard-coded, not WZ keys. The owner: *"Nexon has made these items unisex ... fix it
in the WZ data instead of patching the client"* - so the installer copies the property image
under 1007910..1007912 (digit 7, unisex, free), moves the string and the `islot Cp` patch with
it, keeps the `_Canvas` under the old name (outlinks are explicit paths), the sets say the new
numbers and `store::ITEM_ID_RENAMES` renumbers hats already held. Installed, read back, `--check`
passes, 2147 tests. `research/hair-hat-islot-2026-09-12.md` §6; plan step (i).

**2026-09-17: a quest into a full bag is refused at the NPC, before anything moves.** Mint
(Discord): *"quest continues to complete despite this happening"* under `Quest 1008 could not
give you item 1002005: inventory 1 is full (30 slots)`. The owner: *"the server should use the NPC
dialogue and display an appropriate message to say that their bag is full, please make <x>
amount of spaces in <y> tab. The quest should not complete if the user has a full inventory."*
The completion was recorded, the EXP paid, the letter taken back, and the store's refusal of
the hat became a yellow line - the Heena-quest shape (every effect must hang off one decision)
at the other end. `crate::questroom::shortfall` counts the room per tab the way
`place_into_bag` fills it (stacks topped up, `ceil(rest/slotMax)` fresh slots, a take frees the
slot it empties); `record_quest_complete` draws the reward roll FIRST, checks, and only then
writes the row, pays and hands over exactly what it drew. A shortfall is a plain `Say` from the
quest's NPC parked under `questroom::REFUSAL_PATH` ("Your bag is full ... Please make N space(s)
in your <Tab> tab. Then come and talk to me again."), and the `0x0151` handler skips its closing
line on that path. Accepting has the same rule for start items. Tests: Mint's case end to end
(refused, then completes after one slot is freed, closing line only the second time), a stack
topped up is not a refusal, an accept into a full Etc tab. 2145 tests. Plan step 11.

**2026-09-17: the in-range vacuum (Petite Luna) is FREE; only the pet's WALKING is paid.** The owner:
*"vacuuming loot within a certain range of the pet (Petite Luna) should be free. Auto move ...
should be a skill ... Expanded auto move (longRange) ... should also remain a skill"* and *"The
only default skills it should have is Meso Magnet and Item Pouch."* So the earlier tie (vacuum
keyed on the bought Expanded Auto Move) is undone: `net::bag::pet_item_with_state` now writes
`wonderGrade 6` on every pet unconditionally (`pet_wonder_grade` removed), which is the wide
`0x0198` pickup box AND the client's "Petite Luna" label - both free. `sweepForDrop`/`longRange`
(items `5190002`/`5190003`) stay paid, for the pet's walking toward drops. The installer (step
4c) declares only `pickupItem`, so a fresh tooltip is Meso Magnet + Item Pouch; a purchased
skill still lists *(Learned)* off the mask without its WZ key. **Needs `backport_install.py
--install`** to regenerate `Pet_000.wz` on the client. Also: on field entry the pet now gets the
post-summon item write (`pet_item_refresh`), without which it spawned sad and inert until
re-summoned or fed. `research/pet-vacuum-wondergrade-2026-09-16.md` §8. 2136 tests. Plan step 8.

**2026-09-16 evening: the vacuum is the client's, keyed on `wonderGrade == 6`, and it is now
tied to the bought Expanded Auto Move.** The owner: *"longRange belongs to a Pet Skill that the
clients have to purchase and activate ... Decompile the pet functions first to figure out if
vacuum already exists before we take the burden of the calculations."* It exists
(`research/pet-vacuum-wondergrade-2026-09-16.md`, all [L]): `FUN_14179e990`, the only caller of
the `0x0205` builder, scans drops against a box around the pet - the constant `(-25,-50,25,10)`
unless `FUN_14038a5b0` reads the pet's wonder grade as 6 (`FUN_140374c80` is `cmp ecx,6`), in
which case it uses a box the server supplied in **`0x0198`** (two 16-byte boxes and an item-id
list; `(0,0,0,0)` until sent). The grade comes from the pet ITEM's `u16` after `giantRate`
(`0x140304730`, `+0xba`), which this server wrote as 0. `sweepForDrop`/`longRange` are a
*declared* mask the tooltip reads and the pickup chain never does - which is why the afternoon's
WZ trio (verified in D:\MapleCW's packed `Pet_000.wz`) changed nothing. Now:
`PET_SKILLS_LEARNED_AT_START` is Item Pouch alone (store `0b0001`; a pet that had the Auto Move
bits only by default loses them on read, one that bought them keeps its row); the pet item's
wonderGrade is 6 iff Expanded Auto Move is in the mask (`net::bag::pet_wonder_grade`); `0x0198`
rides after every SetField with Nexon's own unreferenced `(-300,-370,300,220)` at `0x14327c640`
as the box. Zero server calculation per pet - the client scans, as it always did. Tests: the
grade follows the bit and nothing else; buying `5190003` flips the re-sent item's bytes 61..63
from 0 to 6 and re-summons; `0x0198` after a warp. 2132 world/net/store. Plan step 8 LOOT.

**2026-09-16 17:20: a self-updated launcher now refreshes `grap64.dll` beside it.** Found while
shipping the entry below: `stub::resolve` preferred the on-disk stub over the compiled-in copy,
so the setup zip's 09-12 stub on D:\MapleCW (and Joanne's) would have kept the 600 s window
through every launcher self-update. Now the newer of the two wins - the file if modified after
the launcher's executable (a developer's rebuild), else the launcher's copy is written over it
and the log says "rewritten with the launcher's copy". Identical bytes are left alone. Tests in
`stub.rs`; `docs/launcher.md`.

**2026-09-16 17:00: the guard page recycles at 200 s and the reserve is 16 M, because the 16:46
run spent 8 M in four and a half minutes.** The owner: *"I thought we fixed all heap corruptions with
a guard, why is there more?"* then *"Okay, let's recycle sooner."* `D:\MapleCW\previous-runs\
maplecw-hook-20260916-165530.log` [L]: `0x20+0x40` armed at 16:46:16, `0x20` at **34 173/s** (the
12:01 run it was sized from ran at 1 560/s), `0x40` at 3 396/s, and **the two classes share one
cursor** - both served counters stopped between the 240 s and 300 s heartbeats and sum to exactly
8 388 608. With a 600 s window nothing could age out before the reserve was gone, so from ~254 s
every allocation fell back to the client's own pool (9.5 M fallen back by 540 s), the sentry
caught the known `0x0000000100000020` header at 373 s in a pool slot the guard was no longer
serving, and the client died at 554 s of `0xC0000374`. The guard was never a fix - the writer is
the client's anti-cheat writing a compile-time offset past an array, and the guard absorbs it
only while it serves the allocation. Change: `REUSE_AFTER_MS` 600 s -> **200 s** (one full 180 s
firing plus 20 s; what is given up is the second and third firings, which no catch on record
needed), `MAX_SLOTS` 8 M -> **16 M** (64 GiB address space, 64 MB ring at arm, metadata still
lazy) because at 200 s the pair needs 7.5 M in flight and 8 M is a fit, not headroom. The arming
line now models the armed SET against the 16:46 rates (`armed_need`), 2.2x for the pair, and
prints the shout for `all`. Tests: the 16:46 model puts the exhaustion in 240..300 s and says
600 s could never have recycled in time; the 12:01 model still reproduces its own run with its
own window. 113 grap-stub tests. **What the next run must show:** `recycled` > 0 from the ~260 s
heartbeat on and `FELL BACK` absent - plan (1b). Why the churn was 22x the 12:01 run's is not
known: same client build, different server and pet traffic, nothing in the log ties the rate to
either. `research/guard-page-2026-09-08.md` §8.

**2026-09-16: item buffs draw their icon - potions name `-itemId`, the EXP coupon rides CTS 163.**
The owner: *"the EXP coupon effects are not applying the appropriate buff icon on the top right ...
make sure 2x and 3x coupons have the proper buff durations applied"* and *"make sure that Magic
Potions and other similar potions are applying the buff icons as well."* Two defects in one
packet. `buff_from_item` sent every potion stat with `reason: item_id` - positive, which the
client reads as a *skill* id; there is no skill 2002001, so nothing was drawn while the stat
itself applied. The EXP coupon sent no `0x007D` at all (it multiplies server-side in
`with_exp_coupon` and had no bit). Now `net::buff::item_reason` negates the id for every item
stat, and the coupon adds CTS **163 `ExpBuffRate`** - the client's own name for that index
(`research/first-job-buffs.md` App. A), standard 10-byte block at `0x140a22478` - worth its
percent (200/300) for its duration, tracked in `self.buffs` so the tick's `0x007E` takes the
icon down at the instant the multiplier stops. **The sign convention is [D]**: it is the modern
reference's (`rOption = -itemID`) and every client's for a decade, but no instruction in this
build has been read testing it; plan step 10 says what each outcome means. Test: the potion's
entry is `(10, -2002001, 600000)`, the coupon's `(300, -2450001, 900000)` on bit 163 alone, and
two ticks clear them on their own clocks.

**2026-09-16: Sign out revokes the claim, and Start Game greys the instant it is pressed.** The owner:
*"Can we make sign-out button actually revoke the claim please ... Make sure it also disables the
Start Game button once signed out."* The button used to be local and said so. Now the sign-in
keeps the session token (memory only, redacted), Sign out drops the sign-in synchronously (Start
Game is gated on it), then `POST /logout {token}` -> `AuthService::logout` ->
`store::clear_login_claim_for_token`, then deletes the client's identity file. Three answers on
screen: revoked / unknown (expired or superseded) / failed (unreachable, or a server without
`/logout` - the claim expires by itself). Tests: the service revokes exactly that claim and leaves
another account's; the HTTP shape (405 on GET, 400 on bad JSON, revoked then unknown); the
launcher's parser; and end to end over TLS with the service's own store showing the claim gone.
`docs/launcher.md`. Plan step 9.

**2026-09-16: the version gate un-patched the Nexon gate byte, so a client kept showing "failed to
load".** Joanne, via the owner: a published package needed their manual `MapleStory.exe` patch on every
launch, and their script found the byte UNPATCHED though the package ships it patched. Mechanism:
`prepare` patches the gate (`0xd9038a: 75->eb`), then the version gate (`clientpatch::check_and_patch`,
which runs AFTER `prepare`) overwrote `MapleStory.exe` with their server's canonical copy - and their
server serves an UNPATCHED exe, reverting the patch. Fix: `prepare_and_launch` re-asserts
`patch_nexon_launcher_gate` AFTER the version gate, on whatever is on disk - idempotent (no-op when
the server serves a patched exe). Their server should also serve a patched canonical exe (our packages
do) to avoid a 76 MB re-download per launch. Test `a_patch_reverted_by_a_client_download_is_re_applied`.

**2026-09-16: the launcher updates itself.** The owner: *"The launcher that we have should have the
ability to patch itself should we need to. Currently it doesn't seem able to do that."* It could
not - the version gate covered `client\` and the launcher lives beside it. Now the server package
ships `bin\maplecw-launcher.exe`, `start-server.ps1` passes it to `maplecw-auth --launcher`, and
auth publishes a one-entry manifest + the file (`/launcher/manifest`, `/launcher/file` -
`auth::launcherpatch`). At Start Game, before anything else, the launcher hashes its own exe;
if it differs it downloads, verifies size+SHA-256, renames itself to `.old`, renames the new one
onto its path, starts it with `--updated-from <old>` and closes; the new one deletes the old
(`launcher::selfupdate`). A server without `--launcher` is a warning, not a refusal. Sign-in is
redone in the new window (the claim was the old process's). **One last manual install** of the
setup package is needed on each client machine: a launcher without this code cannot update
itself. Tests: the swap on a stand-in file, the decision and the verification, both endpoints
over real TLS through the launcher's own fetchers, and the 503-means-none case. Plan step 9.

**2026-09-16: every pet is a vacuum pet - both halves this time.** The owner: *"Can we turn all pets into
vacuum pets, so they loot from long range similar to current Luna Petite pets in modern
MapleStory?"* The 09-13 attempt set the WZ keys and the same day's skill-item work zeroed them
again, so no pet has been one on a run. Now the installer declares `pickupItem 1, sweepForDrop 1,
longRange 1` on all eleven (the modern trio, 370 of 1561 pets) AND `PET_SKILLS_LEARNED_AT_START`
carries Item Pouch | Auto Move | Expanded Auto Move, ORed into every stored mask on read (the live
rows `1`/`3` need no migration). Auto HP/MP stay purchases; the shop's two Auto Move items now teach
a bit every pet has (sold still, noted). **Needs the rebuilt client package** - the keys are WZ. The
radius is Nexon's, unread; plan step 8 LOOT measures it. `research/pet-vacuum-2026-09-13.md` §3a.

**2026-09-16: pets eat, get hungry, and grow closer.** The owner: *"Pets should decrease their fullness
by 1 every 5 minutes. Using a pet food should recover the current active pet's fullness by 30 and
their closeness by 1"*, with the wiki's closeness table. The request is **`0x0112`**, never captured:
the client's item-use switch `FUN_1428af6d0` picks its builder for `itemId - 2120000 < 10000` at
`0x1428b022f`, and the builder writes `u32 tick, u16 slot, u32 itemId` [L] (`net::petfood`; the next
arm, 2260000.., is mount food -> `0x0113`). `crate::petlevel` holds the 30-level table, "up only",
the feed (+30 capped, +1) and the overfeed rule (the first free, then -1). `Session::pet_hunger_tick`
takes one fullness per five minutes out and at 0 sends the pet home (-1 closeness, put away
everywhere, a notice). A successful trick earns the table's own `inc` (+1..+3) and the pet's level
now picks its command band. `store::pets` gains `level`, `closeness`, `fullness` (added to the live
table by a guarded `ALTER`), and all three ride the pet's Cash item, re-sent on every change - which
is what Show Pet Info reads. **And the two animations, same day** - the owner: *"I do want the eating animation to play for the
client and other players. When closeness levels up, it should also play an animation."* Both read
off the client: `0x027E` (`FUN_141ec4780`, "exception list" in the reference's order, wrong) is the
pet performer - `u8 type`, type 2 = **food**: `u8 success, u32 itemId`, range-checked against
2120000..2129999 [L]; and `UserEffect` arm **9** (of 85, the only arm that calls `GetPet`) is the pet
arm - `u8 subtype, u32 petIdx` -> `CPet::OnEffect`, subtype **0** = `Effect/PetEff.img/Basic/LevelUp`
[L]. A feed sends `0x027E` to the owner and the map; a level gained (feed or trick) sends `0x02D1`
to the owner and `0x02AF` to the map. `research/pets-loot-skills-name-relogin-2026-09-15.md` §7-8.
Plan step 8. Not on a screen yet.

**2026-09-15, 23:34: teaching the Husky Auto HP KILLED the client - the owner's put-away wants a
reason byte.** The reply's last two packets were a put-away and a re-summon to the owner; the client
rejected the put-away by name (`0x009E` reason `0x26`, pos 15 = our 11-byte `0x0277` + 4) and faulted
at `0x140ce89d6`. The remote handler stops after `activated`; the LOCAL one (`FUN_1428a01a0`) reads a
`u8 reason` at `0x1428a06fa` after `SetPet(null)` and switches on it (0 = plain removal, 1..5 a
message) [L]. No put-away had ever reached an owner before - the archive has no `put away for` line.
`pet_deactivated` now carries `reason = 0` for every audience (the remote returns before it; leftover
bytes are not a rejection). Fixture
`research/fixtures/pet-putaway-to-owner-rejected-0x009E-needs-reason-byte-2026-09-15.log`; research
§2a. The skill itself was stored and the item used up before the crash (`mask now 0x0003`).

**2026-09-15, later: the pet walks on both screens (confirmed), and six more pet reports in one
message.** The owner: *"Tester2 now sees the pet, no crash"* - then Auto HP/MP/Move do nothing, no
looting, the Name Tag does nothing, Show Pet Info greyed, the pet's hat invisible to others until a
map change, and the pet gone on re-login. Every one left a packet: `0x0205` x7 (the pet's loot
request, UNKNOWN - byte 17 is the drop id, confirming `PET_PICK_UP_OBJECT_ID_AT`; now routed to
`on_pick_up`), `0x0116` x4 with **the pet's serial after the ten reset-scroll bytes** and, for the
tag, a string ("Dummy") - the skill items OR a bit into the pet item's `petSkill` mask and the
tag stores a name, both in the new `store::pets`, re-sent in the Cash item; `0x027B`
(`FUN_141ec4660`, one `str`) renames on screen; `0x0107` Deco 1 -> -114 put the hat at look slot 14,
which only a fresh `0x0224` carried, so the pet-equip slot now re-announces the look and re-summons
the pet on the map (**[I]** on the in-place redraw - the plan names the falsifier); the active pet
is persisted and `restore_active_pet` re-summons at claim time. `0x0204` is the pet-action report.
**Show Pet Info** was greyed once and enabled the next time on the same build - state, not code: the
gate is the info window's own pet array plus the local user's pet slot 0 (`FUN_1414be310`
0x1414be793..83f), so a window built before the summon stays grey until rebuilt.
`research/pets-loot-skills-name-relogin-2026-09-15.md`, fixture
`research/fixtures/pet-loot-0205-skills-nametag-0116-hat-0107-unhandled-2026-09-15-world-ch0.log`.
Plan step 8. Nothing of it on a screen yet.

**2026-09-15: summoning a pet crashes a second client in the same map - it is the pet MOVE.**
The owner: *"Summoning the pet on character the owner crashed another client Tester2 present in the same
map."* Measured from the crash run (`previous-runs/maplecw-hook-20260915-220508.log`,
`world-ch0.log` 02:05:28): the summon `0x0277` processed on Tester2 with `ret=1` and the pet
idled half a second; the **first** `0x0278` pet-move faulted at `0x141d59bf3` - `movups
xmm0,[rax]`, `rax` from `[obj+0x18]` null - inside the move applier `FUN_141d598b0`, reached
from the pet-move handler `FUN_141ec3f20` (return `0x141ec3f85` on the fault stack) under
`CField::OnPacket` (`0x141821e41`). The remote-user pet path `FUN_1429d6150` (144 bytes) runs
`CPet::Init`+`SetPet` where the local path `FUN_1428a01a0` (2420) also builds the pet's visual;
the remote pet is left with no layer for the move to write, and the summon body was correct
(foothold 166, giantRate 100), so no packet fixes it - the client cannot render a remote pet on
this build. **Root cause (corrected the same day, from the crash dump's registers): the `0x0278` body was
malformed, not the client.** The dump faulted with `rax = 0`, `rbp = 0x4f935ad8` a valid path
container - a zero-element null-deref (`research/remote-move-verification.md` §6.1), not a
missing visual. The `0x0278` dispatcher consumes `petIdx` (`0x142795b5e`) then the applier reads
`u32 key, i16 x, i16 y, u16, u16, i16 count`. The client's `0x0202` path HAS that leading key;
`CLIENT_PET_MOVE_HEAD_LEN` was 9 when the builder `FUN_142b68a20` writes a five-byte head
(`w_u32 petIdx, w_u8`, then the path encoder), so the forwarded "path" began four bytes in, the
pet's X (`-97`) was read as the count, nothing was appended and the empty list tail was
dereferenced. The head is 5 now and the path goes out whole; a zero-element path is dropped. **Pets broadcast by default again** (`Config::broadcast_pets =
true`, `--no-broadcast-pets` for the owner-local fallback). `research/pet-remote-crash-2026-09-15.md`,
fixture `research/fixtures/pet-remote-move-crashes-observer-2026-09-15.log`, dump
`dumps/maplecw-crash-1057776-c0000005-1.dmp`. NEXT GOAL: confirm on two screens that Tester2 sees
The owner's Husky walk (not crash, not teleport).

**2026-09-14: skill points were granted at advancement and then WIPED by the next SetField.**
seedling: *"job advancing to Bowman at level 12, the game did not grant them the 7 SP that they
need because they're over leveled."* The advance itself was byte-perfect - `world-ch0.log`
01:27:56.268, char 218, `01 00 01 20800000 2c010000 0101 07000000 0000`: mask JOB|SP, one pool,
tier 1, amount 7, exactly `entitlement(First, 12)`. So the grant was not the bug and "over
leveled" was a red herring. The wipe is `net::opcode::character_stat_block`: on the extended-SP
branch it writes `out.push(0); // no SP pools`, and the client's extended arm CLEARS the pool
list before reading, so every SetField zeroes SP. purr advanced (7 SP shown), walked through a
portal five seconds later, and the field entry took them back. The store already computed the
right number (`skill_points_available = entitlement - spent`, spent 0) and `skill_point_reply`
already built the correct all-pools `0x007C` - it was only ever sent after a skill-up, never
after a SetField. **"Built is not wired."** Now it rides after every SetField, like the keymap:
`go_to_map` (portal, revive, taxi) and the login SetField in `session/mod.rs` (login, channel
change). `skill_point_reply` gained a `tier_for_job` gate so a beginner is not handed a
phantom first-job pool. The stat block still sends 0, corrected a beat later by the `0x007C`;
threading real SP into the shared record builder is the tidier fix and is noted in
`research/skill-points.md` §12 as the place to move it if SP ever flickers. Fixture
`research/fixtures/seedling-bowman-lvl12-sp7-granted-then-setfield-wipes-world.log`. Also fixed:
the flaky `worldlink` party-echo test asserted the CREATE reply was absent synchronously, but a
same-process loopback hub can deliver the echo inside the same `handle` call (`handle` drains
`collect_party_outcomes` at its tail) - it now accepts the reply from `now` or a later tick and
still proves the hub path via the late-channel snapshot. Plan step 7 added. NEXT GOAL: confirm
on a screen that the 7 SP survives a portal.

**2026-09-14: the CONTROLLER tab - the fourth table, and a keep gate that was never "leave it
alone".** The owner: *"Whenever there are customization to keybindings in the controller settings, it is
not getting saved properly, and when clients switch maps, their controller settings are completely
screwed up."* The byte after the subtype in a `0x0199` CONFIRM is the **table** - `movzx edx, bl`
for keyboard preset rbx at `0x141a00849`, `mov dl, 3` with the controller static `0x143ad1070` at
`0x141a008ef` [L] - and `parse_change` dropped it as "a flag, always 0". It was 0 in twenty captures
because all twenty were the keyboard tab; both captures from the controller run carry 3. So every
controller button was stored as a keyboard scan code (Wisp#215's rows were exactly the 22 bound
slots of the controller factory), and the `0x05F1` at each SetField sent table 3 as keep - which
keeps the handler's **reset to keyboard preset 0** (`0x1419ffc73`, `mov rdx, r12` for all four
indexes), not the table the client had. The 53-entry CONFIRM that followed is the controller
default XOR keyboard preset 0, "unbind Q/W/E" included. The controller's own const is
`0x143274b20` (`FUN_1401de8d0`, 22 buttons, none past 0x27; `tools/keymapdump.py --exe` reads and
controls it, `CLIENT_CONTROLLER_LAYOUT`). Now `Change::Bindings { table, .. }`, `character_keymap`
keyed by `(character_id, preset, key)` with the deployed shape rebuilt in place and the stray rows
scrubbed (22 from #215, 0 from #213, verified on a copy of the live file), and `keymap_init` sends
**all four tables READ, 1785 bytes**. Unmeasured: the 1785-byte packet on a screen - the 1340-byte
one was taken twice today and this is the same shape with a READ table where the keep was. Lost:
the keyboard rows the controller delta overwrote (LCtrl, A) are factory again - a re-bind. Fixture
`research/fixtures/keymap-controller-table-3-delta-and-53-entry-xor-world-ch0.log`;
`research/keyboard-layout-2026-09-08.md` §9. Plan step 6 re-cut.

**2026-09-12, later: the first key-layout restore KILLED THE CLIENT, and the packet is four
tables, not one.** The owner: *"Client exited immediately upon logging into the game world."* The client
named the packet: `0x009E CLIENT_PACKET_REJECTED` reason `0x26`, position `0x1c4`, then our `0x05F1`
verbatim (448 bytes with the opcode; the position is 4 past it), and `CLIENT FAULT 0xc0000005` at
`0x140ce89d6` 3 ms later. Fixture `research/fixtures/keymap-0x05F1-one-preset-rejected-0x009E-then-
fault-{world,hook}.log`. The 2026-09-08 read of `FUN_1419ffc00` stopped at the 89-slot loop;
`tools/reads.py 0x1419ffc00 2` lists four read sites and the listing says the shape: an outer loop
`cmp r15d, 4` - **one gated table per preset**, each first reset to the const preset 0 and then
copied to its shadow - followed by a `u8` quickslot gate and, if set, 32 `u32`s. One table ended
exactly where the second gate was expected. `keymap_init` now sends preset 0 (factory + the saved
keys), presets 1 and 2 as the image ships them (`CLIENT_PRESETS_1_AND_2`, `--exe --rust` emits
them), preset 3 as keep (no const table exists for it) and the quickslot gate 0 - **1340 bytes**.
The quickslot values are unmeasured; 0 takes `FUN_1401de860`, unread. Plan step 6 re-cut with the
0x009E reading. The entry below stands for the table itself, which was right.

**2026-09-12, night: the key layout comes back on login - unverified on screen.** The owner: *"Saving
keyboard layout still does not work. I tried putting both Power Strike on control and Slash Blast on
shift. It did not survive a re-login."* The save was fine (world.log 23:56:57: three bindings
merged, three rows); the restore had been switched off since 2026-09-08 behind
`CLIENT_DEFAULT_LAYOUT = None`, waiting on a runtime dump that needs an elevated shell. Asked of the
file instead, and the file said more: **`0x143274460` is in `.rdata`, read-only** (characteristics
`0x40000040`), so it was never a live manager - it is a const table, and `0x1bd` is the stride of a
preset array: three 89-slot layouts, 41 bound each, kinds 4/5/6 only; preset 0 alone has Q, W, E and
I on menus, with LCtrl = basic 52 and Space = 54, and the owner's delta (LCtrl, LShift, '.') is
consistent with it. That is the factory layout, **[L]** from the bytes; `tools/keymapdump.py --exe`
re-derives it under the shape and known-key controls. Pasted in; `restore` now builds the READ-gate
`0x05F1` with all 89 slots after every SetField. Two new tests (net, world). Plan step 6 re-cut.
`research/keyboard-layout-2026-09-08.md` section 7.

**2026-09-13: the Cash tab starts at 150 slots, like Deco.** The owner: *"The user's Cash tab in the
Player Inventory should also come with 150 slots by default, just like the Deco tab. Currently it is
not at 150 slots."* `net::opcode::CASH_INVENTORY_SLOTS` beside `DECO_INVENTORY_SLOTS`, both fed by
`default_inventory_slots()`, and the `Store::open` migration repairs an existing `slots_cash = 30`
the same way it already repaired Deco. **The Cash tab does have a slot coupon**, unlike Deco, so
that repair needed its own argument: the coupons add five at a time, so a bought Cash tab reads 35
or 40 and never exactly 30 - matching `= 30` still means "nobody ever touched it". A coupon used at
the ceiling was already answered with *"already at the maximum of 150 slots"* rather than failing,
so nothing else changes.

**2026-09-13 (RETRACTION): the pet does NOT draw, and "0x0202 proves it draws" was an inference.**
The owner: *"Husky still does not render, and Husky does not pick up items or mesos."* The claim in the
2026-09-13 tooltip entry - that the pet is on screen because its move report arrives 504 times after
the summon - is **withdrawn**. A pet object ticks, walks and reports its position without being
drawn; movement and visibility are separate in this client, and the run-2 watches had already
measured the visibility half (`FUN_14159b0a0` never called from the ladder, so the verdict never
left "hidden"). The move packets prove only that the object exists and runs, which the name tag
already proved. `CLAUDE.md`'s rule applies: the screen wins over a proxy. **The pickup failure is
probably downstream** - a hidden pet will not run its loot logic - so the gate is the thing to find.
Three more gates settled without a launch: gate 1 is an obfuscated boolean getter that `CPet::Init`
never writes; gate 4 is a morph test; gate 5 is one byte at `*(user+0xa8)+0x2d9`. Also eliminated:
the map's `fieldLimit` (1010 carries `4` = SummonLimit; `NoPet` is `0x8000`, and 1000 carries `0`).
Plan step TO(v) is the four-watch run with **large caps**, chosen so the `called-from` inside the
ladder names the depth. `research/pet-not-drawn-2026-09-13.md`, the retraction section.

**2026-09-13: the pet skill table is the client's, and a pet starts with Meso Magnet and Item
Pouch.** The owner: *"the Husky should by default come with Meso Magnet and Item Pouch. Currently it is
missing the Item Pouch skill by default"* - the previous round cleared Nexon's own `pickupItem` and
took Item Pouch with it. **`FUN_141ed1ad0` is the skill table** (`cmp edx, 0xa`, one jump-table arm
per skill, each loading a name string) and it gives the client's own order **[L]**: 0 Item Pouch
(`pickupItem`), 1 Auto HP (`consumeHP`), 2 Expanded Auto Move (`longRange`), 3 Auto Move
(`sweepForDrop`), 4 Auto MP (`consumeMP`), then Ignore Item, Auto Buff, Auto Feed, Fatten Up, Pet
Shop. **Not the reference's numbering**, which had Expanded Auto Move second - so `net::bag`'s bits
were wrong and are now measured. **Meso Magnet is in no arm at all**: it is innate, which is why it
showed with every key cleared. The four purchasable skills are `5190000..5190003`, already sold at
100 LP under the Pets tab (SN `160300002..5`), each declaring one key plus `add 1`; learning one
needs the pet's image to declare the skill AND the mask bit, and since the image cannot change at
runtime that is a design decision, **not built**. The owner's Auto HP attempt sent **no packet at all** -
the log has no candidate after field entry - so the client refused locally. Changed: the bits, and
`PET_SKILLS_LEARNED_AT_START = Item Pouch`, and the installer restores `info/pickupItem 1`.
**The WZ install is pending a client close.** `research/pet-skills-2026-09-13.md`.

**2026-09-13: pet movement broadcast, pet chat commands, and a pet that advertises nothing.** Four
of the owner's asks, all built off the client's own data. **(1) The WZ has the whole command system** and
`tools/dump_pets.py` now joins its three parts into `gm-handbook/petcommands.txt` (**2405 rows, 12
pets**): `Item/Pet/<id>.img/interact/<n>` gives the command, the percent, the pet-level band, the
closeness and an `act` plus line KEYS per outcome; `String/PetCommand.img` gives the words
(`bad|no|badgirl|badboy`); `String/PetDialog.img` gives the text. The `act` is a node of the pet's
own image, so the **client** owns the animation. **(2) Commands are ordinary chat** - measured, the
client sends only `0x00E7` - so the chat line is untouched and `Session::pet_command_replies` adds
the pet's answer beside it, matching the **whole** message case-insensitively. The packet is
**`0x0279`** (`u8 interact index, u8 success, str line`; `FUN_141ec6680`'s first act is
`test r8d,r8d`, so the second byte is the flag **[L]**, the index is **[I]**). Pets are level 1
until closeness exists, so the first band answers. **(3) `0x0202` is the pet's move report** and
`Session::on_pet_move` forwards its path byte for byte as **`0x0278`** to the map, not to the owner.
**(4) No skill lines**: the tooltip prints a line per skill the pet IMAGE declares, so the installer
now writes `pickupItem/sweepForDrop/longRange = 0` on every pet and `PET_SKILLS_LEARNED_AT_START` is
`0` - pets start with nothing, and learning becomes a Cash Shop purchase (**not built**, and the
vacuum is inert until it is). `research/pet-commands-and-movement-2026-09-13.md`;
`crates/world/src/petcommands.rs`; test
`a_summoned_pet_walks_for_the_map_and_answers_its_command_words`.

**2026-09-13 (run 3): the pet DRAWS; its tooltip's two wrong lines are two fields we zeroed.** The
visibility hunt is closed by the log rather than by the watches: **`0x0202` arrives 504 times after
the summon and 0 times before it**, its body a movement block starting at the exact position we
placed the pet, so `0x0202` is the **pet's move report** and the pet is alive and on screen. (It is
unanswered, so other players are not told the pet moved - unbuilt, noted.) the owner's three new points:
(1) *"does not pick up items ... the skill is applied but unregistered"* - the tooltip builder
`FUN_14266f2d0` takes a **`u16` from the item**, ANDs it with each skill's bit and prints
`(Learned)` (`0x9E5`) or *"This is an unregistered pet."* (`0x9E6`) **[L]**; that `u16` is the pet
body's `petSkill` and we sent `0`, so nothing was usable. Now `PET_SKILLS_GRANTED`
(`ITEM_PICKUP|EXPANDED_AUTO_MOVE|AUTO_MOVE`; the field is **[D]**, the bit numbering **[R]**).
(2) *"My pet is not dyed"* - the dyed line is printed when `-1 < FUN_1401ba9d0(item+0xa6, +0xae)`,
and that callee is the client's obfuscated-int reader, so **hue 0 is "dyed with colour 0" and only
a negative hue is undyed** - exactly the reference's `// -1`. Now `PET_HUE_UNDYED` in the body and
in `0x0277`. (3) *"chat commands does not work"* - **measured**: typing `bad` sent ordinary
`0x00E7 CLIENT_CHAT` and we echoed a balloon; the client sends no pet packet, so the server must
recognise the word and broadcast the pet action (`0x0279`, `u8 + str` after charId and petIdx;
`FUN_141ec3fa0`). **Not built** - it needs the word-to-action table and one capture to settle
whether the command is one byte or two. `research/pet-tooltip-and-commands-2026-09-13.md`.

**2026-09-13 (run 2): the pet is hidden thirty times a second, and three of the eleven gates are
settled.** The four watches came back clean. `FUN_141ecde00`'s **first** hit is the instant of the
summon and it ran **40 times** - twice from the activation path (`0x1428a027e` the handler,
`0x142770819` SetPet) and **38 times from a periodic updater** (`0x141ec1ce8`, ~30 ms apart) - and on
none of them did it call `FUN_14159b0a0`, which only happens when the verdict *changes*. So the
verdict is "hidden" at the summon and on every frame after: **a steady-state refusal, not a race**,
and a portal cannot help because the client is already re-asking continuously. The two mid-ladder
watches had burned their 40-hit caps on unrelated callers **before** the summon, so they said
nothing - the instrument's fault, fixed by raising the caps. Settled without a launch, by
`tools/dis_at.py` on each gate: **gate 6 always passes** (`FUN_140f80860` is literally
`xor al,al; ret`), and the user vtable slot the ladder consults twice (`FUN_142889020`) is
`mov eax,1; ret`, so **gates 7 and 10 always take the proceed branch** and the local-user flag they
compute is dead. Five suspects remain (1, 4, 5, 8, 11); gate 11 is two flags on the field object at
`+0x24ac` / `+0x24b0`. Plan step TO(v) is now four watches chosen so the `called-from` inside the
ladder names the depth, with large caps.

**2026-09-13: the summoned Husky does not draw - the packet is right, the client hides it.** The owner,
with a screenshot showing the name tag **"Husky"** beside "Wisp" and no sprite. The tag is the pet
object's own, so the packet was accepted, the object built, registered and positioned - everything
the server sends is correct. Eliminated on the file: the parse (the name and position render), the
art (`5000006.img` has 22 actions and a real 41x37 `stand0/0` bitmap in the untouched canvas
archive), the templates (`CPet::Init` returns 0 unless both load, and then there is no tag), the
foothold (43 exists on map 1010; NPCs share the convention), the character record (the 108-byte stat
block is fully enumerated and has no pet-serial array), and the `init` byte (its branch only builds
a message). **What decides it is `FUN_141ecde00`, the pet's show/hide** (`research/msexe-pet-setpet.c`):
a verdict that starts at 0 and reaches 1 only through a chain of ~8 gates on the **user's and the
field's** runtime state (morph test, two user-state tests, a COM interface, the local-user test, two
field tests), then `FUN_14159b0a0(pet[8], verdict)` carries it in `rdx`. None is readable from the
file. Plan step TO(v) is four watches: the chain short-circuits, so the last predicate entered names
the gate, and `14159b0a0`'s `rdx` is the verdict (absent = never left hidden). No code changed - the
one known deviation from the reference (hue 0 where it annotates -1) is a guess and costs the same
launch as the measurement. `research/pet-not-drawn-2026-09-13.md`.

**2026-09-13: the WZ change ledger - `docs/wz-changes.md`.** The owner: *"Eventually we'll need to
reconciliate with the actual Classic World WZ and make all of the custom changes again. Please prepare
a documentation on all of the changes to the WZ we did so we can reproduce it."* Derived from
`tools/backport_install.py`, not from memory: the pipeline (extract -> build -> verify -> install ->
handbook -> `--check`), the ledger archive by archive (206 collaboration items across 27 archives;
the box and face-coupon renames; the cover links, the hair-hat islot, the new `ItemEff.img` and its
`z`; Commodity rows 159..175 and the Special tab; the pets' five keys), the server-side twins that
move with it (`ITEM_ID_RENAMES`, `signaturestyle::COLLECTION`, `cosmetics`, the 176-row commodity
test), the runtime client patches that are NOT WZ (43 distinct executable addresses in `grap-stub`),
and the reconciliation procedure - replace the `.bak` bases with the new originals, re-run, and
re-measure the eleven numbers the script read off the old data (each with the command that
re-measures it). Linked from `docs/deployment.md`.

**2026-09-13: every pet is a vacuum pet, client-driven.** The owner: *"turn every pet into a vacuum pet
... which sucks up loot in a radius around them provided that they are from a mob death drop"*, and
*"offload most of the pet driven operations on the client."* The client owns the reaching and the
asking; the server owns the pick-up. **Data:** this client names the pet keys `sweepForDrop`,
`longRange`, `pickupItem` (UTF-16, read by the pet loader `FUN_1403e54e0` and the pet
`FUN_141ed4490`), and 370 of the modern archive's 1561 pets carry exactly that trio - so the
installer's step 4c now writes `sweepForDrop 1, longRange 1` onto all eleven pets (they had
`pickupItem 1`). What the radius is on screen is Nexon's code, unread, measured by the launch.
**Server:** `LiveDrop::from_mob` (kill path only; reactors, ground drops and coins are false) goes
out as the drop's `canBePickedUpByPet` byte so the pet ignores them; `DropTable::take_by_pet`
adds the `from_mob` rule and the **type-5 leave** (`charId, petId`); `on_pick_up` tries the pet's
request shape (id at byte 17, the reference's `PET_DROP_PICK_UP_REQUEST` **[R]**) when the
player's names nothing and a pet is out, and logs the opcode that matched - the client's builder is
in `.themida`, so the capture settles the shape. A pet asking for a non-mob drop gets the unlock
alone. `research/pet-vacuum-2026-09-13.md`; test
`a_summoned_pet_picks_up_a_mob_drop_but_not_a_players_own_drop`. **The client package must be
rebuilt** (`tools/make-installer.ps1`) for the pet keys to reach the client. Plan step TO(u).

**2026-09-13: summoning a pet - built to the client's read order, unverified on screen.** The owner: *"I
tried summoning the Husky pet, but the pet does not come out."* The double-click is **`0x0147`,
`u32 tick, u16 Cash-tab slot`** (`tools/encodes.py 0x142d4ced0`; world.log 18:02:29 twice,
unanswered). The answer is **`0x0277 PetActivated`**, a per-user packet the user pool hands to the
user's vtable slot `+0x98` - found by walking table A's out-of-table range `0x277..0x27E`
(`FUN_142795b20`, the pet family) rather than its dense rows, and the pet decoder `CPet::Init`
(`FUN_141eb9760`) by a rip-relative scan for the `Item/Pet/` pointer slots that `xref.py` cannot see.
Body **[L]**: `u32 charId, u32 petIdx 0, u8 activated, u8 init, u32 itemId, str name, raw8 serial,
i16 x, i16 y, u8 moveAction, u16 foothold, u32 hue, u32 itemId, u16, u16, u8, u8`; `activated 0`
reads nothing more. `net::pet`, `session/pet.rs`: the pet at that slot is summoned beside the
character (last reported position, foothold under it) to this client and the map, the Cash-tab item
is re-sent with `active = 1` and a pairing serial, the request is closed with the empty `0x0070`
unlock; the same click puts it away; every field entry re-sends it; a relog puts it away (session
state; the store has no column). **Not built:** pet movement and the rest of `0x0278..0x027E`; the
client's own `0x0148`/`0x0149` are named but not decoded. `research/pet-summon-2026-09-13.md`,
`research/msexe-pet-*.c`. Plan step TO(u).

**2026-09-13: Rain's quiz - the CLIENT conducts it, so the server must not re-ask.** The owner, with
five timestamped screenshots of quest 1016 and the clock in shot: *"Rain still repeats their dialogue
when I select the right answer and then press OK ... the quest should be immediately completed when I
select the right answer, and then the following OK dialogue should be the end of the conversation."*
`world.log` settles it (this supersedes the 2026-09-13 "box after OK" entry below, which watched the
wrong end): for **41 s** the client drew the offer, the question and the "that's correct" box with
**zero inbound quest/script packets** (17:20:14..55, only toggles and telemetry), then sent the
turn-in - which the server answered by asking the same question **again** (17:20:55 menu, 17:21:06
answer, +350 exp, level 11). The client has the `#L` choices and `stop.0.answer` in `Quest.wz`, runs
and grades the quiz itself, and sends the turn-in only on a right answer (a wrong one shows the
`stop` line and re-asks locally; a redundant server menu it dismisses with `06 00`, 16:58:34). So a
quiz turn-in now records the completion and **says nothing** - `on_quest_request`'s `quiz_turn_in`
arm, the same shape as `silent_accept`. The whole server-side quiz driver (`quiz_menu_answer`,
`quiz_answer_key`, `.quiz.retry`, `quiz_completion_pending`, the `quiz` branch in `say_line`) is
**removed** - it was built on the theory that the server asks the quiz, which the screenshots
disprove, and it was unreachable once the menu stopped. `research/quiz-client-driven-2026-09-13.md`;
tests `a_quiz_turn_in_completes_silently_because_the_client_conducts_the_quiz` and the all-quests
audit (11 quiz turn-ins now complete silently). **Unverified on screen:** a quiz turn-in answered by
the record alone - the silent accept is the precedent, a turn-in is a different request. Plan step
TO(t) is now that check, not the watch run; the box after OK was the client's own quiz.

**2026-09-13: the box after Rain's OK is the client's, and a launch with watches names it.**
The owner: *"Rain still repeats their dialogue when I select the right answer and then press OK. I believe
the quest should be immediately completed when I select the right answer, and then the following
OK dialogue should be the end of the conversation."* The completion IS immediate now (`c161593`):
`world.log` 16:37:14 shows the right answer -> `0x0089` complete, +300 exp, QuestClear, one Say
(*"That's right! ..."*); the OK's `0x00F3` at 16:37:15.698; and **nothing sent after it** - no Say,
no menu, and no `0x0151` until 16:57. So the box the owner saw after OK is drawn by the client from the
completion record, exactly as the fixture of 04:59 showed. What the client does with a state-2
record is now read rather than guessed: `FUN_142d59e20` (`research/msexe-quest-record-handler.c`,
`-tail.c`, `-next.c`, `-available.c`) writes the record (`FUN_142d5b750`), and on the way walks
every quest whose prerequisite is the one just completed, checks its requirements
(`FUN_140711d70`) and raises a fade pop-up for a newly available one (`FUN_14180d7f0`,
`UI/FadeYesNo.img/FadeYesNo/icon6`); a separate path (`FUN_142ce6750`) can open a quest's own
dialogue through `FUN_142d9ac30`, the opener every NPC click uses. **Which of those the owner saw, and
for which quest, is not established** - the server cannot see it and the hook has never watched
those functions. Plan step TO(t) is a launch with `watch@` on all four (each logs its quest id and
return address) and asks for the exact text of the box. No server change until then: the two
plausible answers need opposite work (send the record after the OK, or leave the chain's auto-offer
alone), and the third needs none.

**2026-09-13: the spawn audit, and a kill now refills the map rather than its own point.** The owner:
*"the monster spawn seems to be at the maximum map cap every time even while being the only person
in the map."* Measured first, from the two latest runs: A Split Road (66 points) got **49**, map 50
(42 points) got **31** - exactly `spawn_capacity`'s 75%, and the distinct object ids over each
session never exceeded that, so the cap holds and does not creep. The fan site the rule came from
gives Split Road as 49.5 solo / 66 full party, so the count agrees with its own source. Three things
the audit found and left as they are: field occupancy is still untracked (`players` is always 1);
the site's +5%-per-player gradient is not the adopted 75/100 step; `mobRate` (1.3 on Split Road,
1.5 on map 50, 1.0 on map 40 - read off `Map0_000.wz`) is read by nothing here, and the site only
*speculates* it multiplies capacity upward. What made the field feel pinned: **a dead mob came back
after 7 s on the same point**, so the same 49 of 66 points stood forever and 17 were never used.
The owner: *"once the mob is dead, a completely random spawn point should be chosen that's not
necessarily the dead mob's spawn point."* `Fields` now books a `Refill::Anywhere` on a kill; when it
comes due, one free **ordinary** point (`mobTime 0`) is drawn uniformly from the whole map and that
point's mob stands up, whatever its type - one death, one refill, cap unchanged, and each type's
expected share stays its share of the map (`research/mob-spawn-selection.md` §3). A timed point
(`mobTime > 0`) still returns at its own place on its own clock (`Refill::Point`); `-1` never; a
summoned mob's death books nothing (that gate was implicit before and is explicit now, because a
stray booking would now put a random mob up). `fields::tests::
a_kill_refills_a_random_free_point_rather_than_the_one_that_emptied` and two siblings. Plan step
TO(s).

**2026-09-13: the HP ceiling counts worn items' `incMHP` / `incMMP`.** The owner, with `194 / 199` on
screen: *"There are rare instances of when the server and the player does not agree what is the max
HP for the user ... passive recovery only recovers up to 194 and stops."* The database held
`194 / 194`; the five is the Red Headband (`1002003`, template `incMHP 5`), which the record sends
on the worn item and **the client adds to the bar itself** - the same shape as the 2026-09-06 Max HP
Increase finding, at a flat rate. `Session::pools` counted the percent and not the flat, so every
ceiling it feeds (regen, potions, the level-up refill, `!heal`, the party bar) called 194 full. It
now sums the worn items' flats off the same stats the record sends (`Session::dressed`, so a missing
`equips.txt` leaves both at zero together), flat before percent - the reference server's order
**[R]**; unmeasured here, and the one character with a percent wears nothing with HP on it.
`pools::tests::wisps_ceiling_is_199_from_a_base_of_194_and_a_red_headband`,
`regen::tests::a_red_headbands_five_hp_is_regenerated_up_to`. Plan step TO(r).

**2026-09-13: the repeat-dialogue audit, every quest.** The owner: *"Please audit all of the questline
and make sure repeat dialogue is no longer a concern."* The rule the day's three fixes converge on:
**the client shows a quest's opening (`Say.0`) itself, so the server must never answer that quest's
Accept (action 1) or turn-in (action 2) with its own `Say.0`, and one request never opens two
boxes.** `no_quest_answers_its_accept_or_turn_in_with_its_own_opening_lines` walks all 316 quests
with dialogue: Accept -> 157 speak `0.yes`, 159 send the record alone; turn-in -> 287 speak `Say.1`,
11 ask a quiz (menu; completion on the answer), 7 chain to the next quest's opening (and start it,
so the client does not offer it again - 1000 -> 1001 on screen 2026-08-20), 11 send the record
alone. It found one more: a known quest's turn-in with nothing to say and nothing to chain to
(1002) fell to the NPC's `d0` greeting - the `silent_accept` return now covers turn-ins too. The
one path on which the server speaks `Say.0` is action 4, the opening script (27 captures in the
archive, all quest 1002; the client has no local text for a scripted quest). Not re-checked and
left as is: action 6 (requirement failed) would take the `"0"` arm; it has never arrived.
`research/quest-dialogue-audit-2026-09-13.md`.

**2026-09-13: the eleven pets - permanent, all in the shop, and buyable as type-3 items;
unverified on screen.** The owner: *"Brown Puppy, Panda and Dino Boy all have 3 day duration. Please edit
the WZ if needed to change all of them to permanent duration. Also please add all of the other pets
into the Cash Shop too ... They should never need to be revived."* Three findings. (1) The duration
is each pet's own `Item/Pet/<id>.img/info/life` in days - 3 for those three, 7 and 90 for the rest;
the three Commodity rows already said `Period 0`. The modern client's one permanent pet (5000060)
carries `life 0, permanent 1` **[L]**, so the installer (step 4c) patches every classic pet to that.
(2) The eight pets with no row get one under the Pets tab (SN `160000003..10`, category 6 / scope
600 by the SN arithmetic, 100 LP, Period 0). (3) **A pet purchase was refused outright** - no type-3
body existed and a bundle sent for one had killed the client (2026-08-26). `tools/reads.py
0x140304550 2` lists the pet decoder's fourteen reads after the shared base, and the reference's
`PetItem.encode` gives the same widths in the same order **[L]+[R]**: name[13], level, closeness,
fullness, dateDead, petAttribute, petSkill, remainLife, attribute, active, hue, giantRate, u16, u32.
`net::bag::pet_item_with_cash_sn` builds it (name = the item's, level 1, fullness 100, dateDead never -
the "never revived" half); `item_blob` uses it for every 500xxxx item and the three shop refusals are
gone. **Summoning a pet is not built** - the shop and the bag are this change. Plan step TO(p).

**2026-09-13: a `prop`-marked quest reward is one draw from the pool.** The owner: *"When I finished
'Please bring this letter to Lucas', Maria gave me one of every single Headband item when it's
suppose to be choose 1 randomly from the pool."* Quest 1008 (Lucas's Reply): `Act.1.item.1..7`
are seven headbands each with `prop 1`; `item.0` (the letter back) has none. **[L]** The loader
read only `id` and `count`, so the turn-in handed over every row. Now `Quest::complete_rewards`
(and `start_rewards`) keep `prop` and `gender` per item, and `config::choose_rewards` gives every
`prop 0` row, draws ONE of the `prop > 0` rows with weight `prop` from the session's rng, and drops
rows marked for the other gender. 39 quests carry the mark. Plan step TO(o).

**2026-09-13: a quiz's turn-in waits for the right answer.** The owner: *"Rain's quiz dialogue repeats
after I choose the correct answer. That is not okay."* The wire for that run: `0x0151` complete ->
record + exp + fanfare -> the menu -> the right choice -> the closing line -> nothing more from the
server. The repeat was the client's: it acts on a completion at once and offers the chain's next
quest, which the menu covered and which came back after the closing line. `Say.1.ask = 1` says the
turn-in depends on the answer, so `on_quest_request` now defers `record_quest_complete` for a
completion path that carries `ask` (`quiz_completion_pending`), and `quiz_menu_answer` fires it on
the right choice, before the line; a wrong choice or a closed box leaves the quest in progress.
The menu itself is CONFIRMED on screen (quiz 2 answered). Plan step TO(m) re-cut.

**2026-09-13: reactors - the breakable boxes - are spawned, hit, broken, looted and respawned;
unverified on screen.** The owner: *"Pio's Collecting Recycled Goods ... the items come out of breakable
wooden boxes which we do not spawn right now. We need to spawn them and provide the drops"*, then
*"make sure the rest of the reactors also spawn"*. Found from the data up: `Reactor/%07d.img` is read
through `.data` slots by two template loaders whose callers climb to **`FUN_141f2c0a0`, the reactor
pool**, called from `CField::OnPacket` at `0x141821f5a`; its switch is `add edx,-0x478 / cmp edx,0x14`
over a 21-entry table, so the pool owns **`0x0478..=0x048C`**: `0x0478` ChangeState (reads u32, u8,
u16, u16, u16, u8, u32, u32), **`0x0484` EnterField** (u8, u32, u32, u8, u16, u16, u8, str), `0x0485`
LeaveField (u32). **[L]**, and the reference's `ReactorPool.java` encodes the same widths in the same
order. Outbound, the client's builders in that code range send `0x032F` as `u32, u32, u16, u32` - the
classic ReactorHit - **[I]** until the first capture. `Reactor.wz/0000001.img` (the Wooden Box) has
events on states 0..3 and none on 4: four hits. `tools/dump_portals.py` now emits
`gm-handbook/reactors.txt` (228 placements, nine reactors; `breakAt` counted from Reactor.wz), the
world loads it and `data/reactor-drops.txt` (the fan site's feed at
`api/breakable-drops?breakableId=0000001`: Rusty Screw, Old Wooden Board, Apple, Egg; and
0000003..7: Plant Sample 4031072, Witchgrass Leaf 4031012, Coconut 4000061 - chances ours), the
field pool seeds every placement, `session/reactor.rs` sends `0x0484` on entry, answers `0x032F`
with `0x0478` to the map and on the breaking hit rolls the table through the mob quest-item filter
and lands the drops at the box, and the tick sends `0x0485` + `0x0484` after `reactorTime`. Object
ids 6000.. per map. Both packagers now require `reactors.txt`. Plan step TO(n).
`research/reactors-2026-09-13.md`.

**2026-09-13: Rain's quiz killed the client - the question is a MENU, now sent as one.** The owner: *"I
just tried taking Rain's quiz, and after finishing question one, the client exited."* Quest 1013's
`Say.1.0` is the question with four `#L<n>#` choices and `Say.1.ask = 1`; `say_line` sent it as type
0 and the client faulted (`0xc0000005` at `0x142a5ce2f`) 22 ms after the box - no throws, no
`0x009E`, the renderer itself. Fixture `research/fixtures/rain-quiz-say-with-menu-tags-client-fault-*`.
A `#L` list renders only inside message type 6. Now: a path whose node has `ask` sends its first line
as `npc_menu`; `quiz_menu_answer` claims the type-6 reply (same precondition shape as the taxi's),
grades the choice against `<path>.stop.0.answer` - **1-based**, **[D]** from all 18 quiz nodes in the
client's data, where the answer index is never among the `stop.0.<n>` wrong-choice keys - answers a
wrong choice with its `stop.0.<n>` line (by WZ index, which `Quest::say_indices` now keeps; the
positions alone would say `0,1,2` for `0,1,3`) and re-asks on Next, and a right one with the next
line. Rain's seven quizzes, Stan, Icarus, Hella and the rest are the same shape. Plan step TO(m).

**2026-09-13: Three Snails throws its shell, and refuses in red without one - unverified on
screen.** The owner: *"Three Snails is a skill that takes 1 Red Snail Shell to cast ... red error text
in chat ... Casting it should decrease the client's Red Snail Shell inventory count by 1."* The
skill table already carried `itemCon`/`itemConNo` (level 1 Snail Shell 4000001, level 2 Blue
4000002, level 3 Red 4000004, one each - **[L]** from Skill.wz) and nothing spent them.
`Session::spend_attack_item` runs first in `on_attack`: enough in the Etc tab -> lowest stack
debited, `0x0070` per stack (the arrows' shape); short -> `Err` with one `0x0089` chat line in the
client's system category 11 (`0xFFFFAFAF`, the colour of its own "You cannot" lines) and the swing
returns THAT alone - no MP, no damage, no broadcast. It is the first attack cost that refuses; MP
and arrows are floored, not refused, because the client has already spent them locally. Whether the
client also applies the damage locally on a refused cast is the open reading. Plan step TO(l).

**2026-09-13: an accept with no `yes` branch says nothing - unverified on screen.** The owner: *"Nina's
quest dialogue seems to be repeated when accepting their 'What Sen wants to eat' quest. They say the
same two dialogues before and after I click 'Accept'."* world.log 04:06:17: the `0x0151` accept for
1003, the record, then *"line 1 of 2 on path 0"* again. The client shows `Say.0` itself before the
button (the 2026-08-19 lesson in `on_quest_request`); the server's answer is `Say.0.yes`, and 1003 has
`Say.0` and `Say.0.no` but no `yes`, so the match fell through to `"0"`. Now `Some(_) if accepted =>
None` and a `silent_accept` return before `say_line`, so neither the opening nor the NPC's `d0`
greeting goes out - the record alone. Test with Heena's 1000 as the control. Plan step TO(k).

**2026-09-12, release readiness.** The owner: *"Make sure everything we worked on is release-able to the
server and client packages."* Everything this week is either server code (built by the packagers)
or client data under `client-patched\Data` (copied by `make-installer.ps1`), and the gap was that
nothing proved the copied data is the *current* backport build. Now: `backport_install.py --check`
rebuilds into a scratch dir and requires every installed archive to hash equal to it and the
handbook to postdate the install; `make-installer.ps1` runs it before copying the client and
excludes the `.bak` originals; `package-server.ps1` requires every table the servers open by name
and refuses a handbook older than the installed `String_000.wz`. `docs/deployment.md` has the
release order. Item renumbers (`RENAMES`) are mirrored in `store::ITEM_ID_RENAMES`, so the homelab
database catches up on its next open. Both packagers need the servers and the launcher stopped
(they rebuild); run them from an elevated window after `-Stop`.

**2026-09-12, latest: cape CONFIRMED; hats still refused, now instrumented; face coupons re-numbered.**
The owner: *"Nope, equipping the hair caps still does not work. Himmel's cape now looks fine. The face
coupons from the backported collaboration items still does not work."*

* **Himmel's cape**: done - the effect draws behind the body at `z -2`, so the modern image's sign
  convention holds in this client. **[L]** on screen.
* **The hair-hats**: `islot Cp` was not the gate - world.log again has **no `0x0107`** for them. The
  double-click equip path is `FUN_1417dd7e0` (it ends in the `0x0107` builder `FUN_142cc5b00` at
  `0x1417dea90`); the slot validator `FUN_140253980` it calls is a category jump table (cap -> slot
  1) and passes; the level/stat/job check is `FUN_140397db0`; a dozen other exits sit between. Not
  settled statically - plan step TO(i) carries a probe line (`1417dd7e0`, `140397db0`, `142cc5b00`,
  hits) and the readings. The ring-limit strings `0x4e8..0x4ed`/`0xb63` that function can raise are
  not this.
* **Face coupons**: no packet in any run. The Beauty Coupon dialog's opener (`0x141785d90`) accepts
  seven id ranges: `2540000..2549999` (hair), `2890000..2890999` (the modern client's "Face Coupon"
  family), `2889000`, `2893000` (skins), `2894000`/`2895000` (android faces), `2900168`.. - and
  `2897xxx` is in none of them. **[L]** So the eight face coupons wear **2890907..2890914**: installer
  `RENAMES` (generalised from the box's), `world::cosmetics` and `signaturestyle`, and
  `store::ITEM_ID_RENAMES` for coupons already in a bag. Installed, read back, handbook regenerated,
  1389 store+world tests pass. Plan step TO(j).

**2026-09-12, later: the three hair-hats go on - installed, unverified on screen.** The owner: *"The
Aura, Lugner and Linie hair does not wear when double clicked on."* world.log has **no `0x0107`** for
1006910/1006911/1006912 in any run they sat in a bag (`grep` over `previous-runs/`), so the client
refused locally and sent nothing - the same shape as the Collection box. Their `info/islot` is
`HrCp`, the modern two-slot type (hair slot and cap slot); every classic cap says `Cp`, and `MaPn`
- which the overalls carry and which equips fine - has no whole-string match in the image either,
so the client reads the type in two-letter tokens: `Hr` first, hair, which no bag can put on. **[L]**
on the data and the absent packet, **[I]** on the token reading. `tools/backport_install.py` step 1d
patches `info/islot` to `Cp` on every cap whose type starts with `Hr`; `vslot` is left as Nexon wrote
it (one variant at a time). Installed. Plan step TO(i).

**2026-09-12, later: the cape effect DRAWS - in front of the body.** The owner, with a screenshot of
Cobalt standing inside a grey block: *"Himmel's cape should have an offset and appear behind the
player's character, currently it blocks the character when idle."* So the ItemEff route is right
(the loader found the new image and drew the frame) and the depth is wrong: Nexon's node carries
`z 10` on `effect` and `effect/stand1`, which this client puts in front. The same modern image gives
its plain behind-the-body auras `z -2` (1103988), so the installer now rewrites every `z` leaf under
`1103918/effect` to -2 (`itemeff-z.tsv`, a `patch` op after the merge). The frame origin (38,141 on
81x143, centred on the body) is untouched. **[I]** on the sign; the screen decides. The install waited
for the client to close. Plan step TO(h) re-cut. The entry below is how the effect got there.

**2026-09-12, night: Himmel's cape has its effect - installed, unverified on screen.** The owner:
*"Himmel's cape should actually have an effect, but this effect currently does not appear in our
version of the game."* The cape (Himmel's Blessing, 1103918) is 1x1 frames in its own image - the
garment IS its effect - and worn-item effects live in `Effect/ItemEff.img/<id>/effect`. The classic
`Effect_000.wz` has **no `ItemEff.img` at all** (26 images), while the client's loader for it is
present: the format `Effect/ItemEff.img/%d/%s` + `effect` is read through a `.data` pointer slot
(`tools/dataref.py 0x143a47080`) by five functions, among them the avatar code and the
character-select slot filler `FUN_141177e80` - so the client dresses effects from the worn list on
its own, no packet involved. Pure missing data. `tools/backport_install.py` step 1c now merges every
set item's ItemEff node onto a new `ItemEff.img` (Himmel's is one 81x143 frame, `stand1`/`stand2`
only, `z 10`, `action 1`) and the canvas holders its outlinks name (1103930, out of a 244 MB modern
canvas image; 46 KB lands) onto a new `_Canvas/ItemEff.img`. Both verify. Plan step TO(h).
`research/himmel-cape-effect-2026-09-12.md`.

**2026-09-12, night: cash equips survive a relog - CONFIRMED on screen** (the owner: *"I do see the
cash equips on my character upon login"*), **and the weapon-sticker fix below is CONFIRMED too** (the owner, later: *"Ubel's weapon is fine in game and on character select"*) - so `look+0x2d` IS the weapon sticker, now **[L]** on screen. **The miss, as it was found:** *"for
Ubel's weapon, I do not see the proper rendering of it on character select. (It does show up fine
in the game world)"*. The field dresses from the worn list; the select screen reads the compact
look, and `look_maps` had put the weapon COVER (1703726, family 170) in slot 11 of the drawn map
with the real weapon demoted to the covered map. A cover is not a weapon and has no stance:
`look_layout` now keeps the weapon in slot 11 and sends the cover in the `u32` right after the
two maps - the weapon STICKER field (`look+0x2d`; the reference's field order, **[I]**, and the
classic client ships covers of its own so the field is exercised). Plan step TO(g) re-cut. The
original write-up follows.** The owner: *"I last had
Cobalt wear the entire Ubel outfit, but upon a fresh login, I do not see those cash items
equipped anymore."* The database had them the whole time (Cobalt's `equipment` rows: 5, 6, 7, 11
and **105, 107, 108, 111**), so this was never a persistence bug - it was two packets dropping
them. (1) `net::opcode::avatar_look` put the worn slot numbers on the wire raw, and the client's
reader keeps `1..=31` only (`0x1402ee9c0`) - character select and other players never saw a cash
item. Now `look_maps` draws a cash item at its **base** slot and puts the item it covers in the
look's second map (`+0xb9`; the first map's index 0 is the hair, so the first is the drawn one).
(2) The SetField record sent no second equipped block, so after a login the client held the worn
cash items in **no list** - not drawn, not in the Deco equip window, not removable, while the
server still had them worn. Now a character wearing anything cash opens **presence[44]** and the
record carries `cash_equipped_block`: flagA 0, `(u16 base slot, item)*`, four `u16 0` - read off
the listing at `0x140306654..0x140306830`, the mirror of the first block. Absent for everyone
else, so their record is byte-identical to before. Plan step TO(g) says what each outcome means;
the record has no resync point, so a wrong block there is fatal, not cosmetic.
`research/cash-equip-relog-2026-09-12.md`.

**2026-09-12, later: the Collection box now wears id 5681599.** The owner: *"Double clicking the
Signature Style Collection box does not grant all 8 character costume coupons."* `world.log` shows
NO packet for the double-click - the client did not treat it as a use. It opens a Cash item on
double-click by id FAMILY: Nexon's box is `5222221` (family 522, of which this client has no
items) and sent nothing; the set coupons are `5681xxx` (family 568, the client's own 5-slot
coupons) and send `0x0114` every time, measured on screen [the family reading is I; the pair of
controls is L]. No archived run had ever recorded the box opening. `backport_install.py` now
installs the box's node under `0568.img/05681599` (a `k=k2` rename form on `wz-dump build`'s
merge), its string and Cash Shop row under `5681599`, `signaturestyle::COLLECTION` matches, and
`store::inventory::rename_item_ids` rewrites `5222221 -> 5681599` in every item table on every
open (the live database gets it on restart; a test derives the table list from the schema).
The installer now builds every archive from its pristine `.bak` (a rebuild is a function of the
original and the script; the previous way carried stale nodes forward) - and its first version
installed over the `.bak` files by mistake; all 24 originals were restored from `.bak.bak`,
sizes checked. Unseen on screen; plan step TO(f).

**2026-09-12, night: opening a package shows a receipt.** The owner: *"open a NPC dialogue from
'MapleStory Administrator' along with 'You have received the following items', then list out the
items ... one per line along with the appropriate item icon."* `hand_out` now ends with a Say from
NPC 9010000 (`signaturestyle::receipt_text`: heading, then `#i<id># #t<id>#` per item actually
given, CR LF between lines), parked under `package.receipt` so OK closes it silently and no other
`0x00F3` feature claims it (tests in both directions). The NPC's String.wz name is patched to
"MapleStory Administrator" by `backport_install.py` (installed). **And Frieren's coupon asks which
version first**: Nexon ships the set as normal / Ringlets / Sleep (sale page 44291, read 2026-09-12),
so opening `5681543` opens a three-row menu from the Administrator (`FRIEREN_VERSIONS`, path
`package.frieren:<slot>`); the choice spends the coupon, End Chat keeps it, and the page's
"your choice of" Clothes / Winter Clothes becomes both (the owner's rule). The variant coupons `5681544`/
`5681545`, held directly, open their version without a menu. The dialog line break is the LITERAL
two characters backslash-n (`scrollnpc::LINE_BREAK`'s lesson; a real CR LF draws as nothing). Unseen
on screen; plan step TO(d)/(e) say what each outcome means.

**2026-09-12, afternoon: every inbound opcode has a disposition** (the owner: *"handle all of the
opcodes"*). Enumerating the archive, deduplicated: everything still logging `UNKNOWN ... not
answered yet` is a one-way client report - `0x013D` (30 s census, **must not be answered**),
`0x01ED` (log channel), `0x01A5`, `0x02DE`/`0x0184`/`0x0194` (field entry), `0x00B8`, the
`0x0420..0x0426` leaving burst, `0x0425` (resource census), and three undecoded (`0x01C1`,
`0x01B9`, `0x0226`). All named in `net::names`, listed in `is_client_report()`, routed through
`world::session::reports` (nothing back; `0x0422` logs its leave reason), and both servers now log
`is a client report; nothing is expected back` for them. The same sweep found **seventeen
handled opcodes with no name** (`0x00E5`, `0x0199`, `0x0107`... all with arms and modules) - a
test now pins that every dispatched opcode is named. `grep -c UNKNOWN` on a run should be 0.
`research/opcode-dispositions-2026-09-12.md`.

**2026-09-12: the blank select screen is MEASURED - the slots are placed EMPTY, and both
earlier theories are dead.** The 0x007A/400 ms-pause theory (2026-09-10) and the null-gate
theory (2026-09-12 morning) are both refuted. An instrumented blank login (01:54) shows the
select-UI object IS constructed and avatar placement (`141179970`) DOES run four times - the
avatars are blank because the **per-character fill never runs** (`141177e40` has no line, and
the placement loop skips every slot whose character pointer at `+0x10` is null). The fill
(`141177e80`) is reached only from the **mode-5** `0x0010` handler (`141b32860` -> `141177e40`)
or the UI's vtable build (`141177790`); the client is patched **mode 5->2**, and the mode-2
handler `FUN_141b307b0` decodes the list but fills nothing. **Then three more instrumented
logins (02:15-02:16), two good and one blank, settled the mechanism** (sec 8): the mode-race
theory is dead too (mode 2 handled all three and the good ones filled), and the select UI is
**built once**, at the fade-deadline populate, by `FUN_141177790`, from whatever character list
exists at that instant. Good logins built it ~490 ms after the login request, after the list;
the blank one built it **30 ms** after, from inside the still-running `0x0032` dispatch, from an
empty list - and the mode-2 decode of the list that followed refilled nothing. No server timing
can beat that ordering. **Fix, in the hook (2026-09-12, unverified on screen):**
`grap_stub::session::refresh_select_after_dispatch` calls the client's own
`FUN_141177e40(selectUi)` - the refill the mode-5 handler makes and mode 2 does not - after every
`0x0010` when the select UI already exists; `selectfill=off` turns it off. It is a client patch
and does not make the session valid. **09:16: three more launches, 2 blank, 1 good - the fix
never ran.** Both blank logins were the early-build ordering as predicted and SELECTFILL fired,
but its prologue guard read the default probe's own int3 on `141177e40` and refused: the
instrument defeated the fix (sec 9). Guard now tolerates the int3, watch removed. **09:23:
CONFIRMED - four more launches, all four the blank ordering, all four rescued; the owner: "it seems
consistently fixed now"** (sec 10). `refresh_select_after_dispatch` is load-bearing: a wiring
test in grap-stub fails if `hook.rs` stops calling it, and `141177e40` must never be watched in
`-Probe`. The list
re-send and `--no-list-resend` are not the mechanism and can go once this is confirmed.
`research/select-screen-race-2026-09-10.md` is superseded.

**2026-09-10, night: the locker-to-bag move is built, and the Cash Inventory is listed at
entry.** The owner: *"Coupon when bought goes into the Cash Inventory, not the storage. The player
can choose to move the coupon out of the Cash Inventory into the regular inventory in the Cash
Tab"* - and, of the retired `!locker`, *"we don't need it now that moving items in and out of
Cash Shop Inventory works."* It did not: `0x03E1` sub-op `0x0A` was refused with `0x3D` since
2026-08-26 and **no archived run contains a single attempt**, so it had never been exercised.
Built from `research/cash-shop-actions.md` §3 (the builder read in full): request `u8[8]
serial, u32 itemId, u8 tab, u16 slot`; every client-side check re-made server-side (serial
minted for this account, locker slot holds that item, tab matches the item, slot in range and
empty); take-then-place with a put-back on failure; reply `0x19` with the item body, which the
client reads as "release, place at N, erase from my locker map". Shop entry now also sends one
`0x0C` per stored locker row, because `set_cash_shop`'s three list counts are not the locker
and a coupon left in the Cash Inventory vanished from view on the next visit. **Both unseen on
a screen**; step TC(b) and (c). `0x0B` (bag to locker) and `0x1C` (delete) are still refused.

**2026-09-10, late: the AP and SP Reset Scrolls were on the wrong opcode.** The owner: *"it did not
work and it did not take the item."* Both presses are in `world.log` as **`0x0116`** with the
coupons' ten-byte body - the reset arms had been written against `0x0114` from the coupon
capture and could never fire, and nothing answered `0x0116` at all. Now dispatched to the same
arms, which run the `!resetap` / `!resetsp` refund, consume the scroll and clear the latch for
the opcode that actually arrived. Both cash-item opcodes are named in `net::names` so the log
stops calling a handled packet UNKNOWN. **Confirmed on screen the same night** - the owner: *"both AP and SP scrolls now work."*

**2026-09-11: the Beauty Coupon dialog's white-on-white item name is the client's own string.**
The owner: *"I do not seem to be able to see Übel Hair Coupon because it's completely white"*, then
on the ASCII control, *"Frieren's hair also show up as white."* Not the backport's encoding:
`tools/dump_stringids.py` decrypts id `0x0464` as `Would you like to use #fc0xffffffff#%s?` -
opaque white, for the modern dark panel; this client's `UtilDlgEx_Beauty` panel is white. The
id is fetched at exactly two sites, both in `FUN_142dc8100` [L]. `grap-stub::beautytext` now
patches the six encrypted hex digits to `000000` (black) at hook install - the bytes were
derived with the dump tool's key schedule and pinned by a key-free test (`new ^ old == 'f' ^
'0'` per byte). **[I]** that the table is not decrypted eagerly before the hook; step TB's
second outcome names the fallback (the same six bytes in the exe file). Kill switch
`-NoBeautyTextPatch` / `beautytext=off`.

**2026-09-12: the Übel outfit run - Deco-tab equips were refused by the SERVER, the hair coupon's
Confirm was unanswered, the weapon and the face are the backport's data.** The owner: *"none of the
outfit items work"*, *"my hair did not change"*, *"The Ubel Face Coupon does not open up a
corresponding UI"*, and the Beauty dialog text is *"readable now"* (TB struck). From
`world.log`: the client sent every clothing equip from the Deco tab (`invType 6`, dst `-105`,
`-107`, `-108`) and `on_inventory_move` knew only the Equip tab, so they fell to the bag-to-bag
branch and were refused. Fixed: `equip_from_tab` / `unequip_to_tab`, worn slots 101+. **Still
open:** the record discards worn slots above 31 [L] and the look's cover map is unbuilt, so a
worn cash equip shows in the Deco equip window but does not draw and does not survive a relog
as worn. The hair coupon's Confirm is **`0x0165`** (`u16 slot, u32 itemId, u16`), captured
twice unanswered; `session::beautycoupon` applies the cosmetic from `world::cosmetics` (the
coupons' `spec/cosmetic`, read from the WZ), spends the coupon and re-enters the map to redraw,
as `!hair` does. The weapon refusal is the client's own per-type check: classic covers carry a
child per weapon type (`01702001`: a real `30`, and `31`/`32`/`33` as UOL links to it), every
backported cover carries only `30` and `49`, and the equipped suitcase is type 32 [L on the data,
I on the check]. **Fixed in the data, 2026-09-12 (built, verified, installed; unseen on screen):** `tools/backport_install.py` now gives each of the six covers a UOL
to `30` for every classic weapon type (31,32,33,37..47), `wz-dump build` gained the `uol` patch
kind and layers a `patch` onto an earlier `copy` of the same image (it used to replace it).
Next launch: equip the weapon over the suitcase; it should go on (drawing is the separate open item). The face coupon: images
installed and node-identical to a classic face; the one difference is the id (22035..22042
against a classic space ending at 21825, while hair 42600 works outside its own space) - step
TH's `!face 22039` tells the two readings apart.

**Cash-equip covers do NOT work yet**, and it is not the gate: the character record keeps
equipped slots `1..=31` only (`EQUIP_SLOTS`; a cash slot is decoded and discarded, `[L]`), and
`avatar_look`'s second map - where the covered base items would go - is sent empty. A cash top
can be stored as worn but is never drawn and never listed. Building covers means the record's
cash-equipped list and both look maps, decoded from the client, not a slot-range tweak.

**2026-09-08: the client token is a SESSION credential now, and an account can only be logged
in once.** Two changes, and they only work together.

A player reported *"the launcher says the session is valid for 12 hours on Login, but after the
initial launch, if they try clicking Start Game again, the client will say the session is
invalid."* The launcher keeps the token its sign-in returned and hands the same one to every
Start Game; the server **spent it on first presentation**, and a second Start Game is a second
process. The twelve hours on screen was the CLAIM's lifetime; the token's was one packet. The
token is now honoured for as long as its claim is live, whichever process presents it.
**That is a real widening**: the token sits in plain text beside the client until the hook
deletes it, so a copy is now worth being served as that account on the LOGIN socket for up to
twelve hours instead of one race - and an **off-box replay, which used to be refused outright,
is now served**. Expiry is the only gate; changing the account's password clears its claims and
kills the token at once. `store::claims::Store::present_client_token` carries the whole trade.

A second client asking to log in as an account already being played is answered with **login
result 7**, which draws the client's own baked notice: *"That ID is already logged in. Please
try again later"* (`Login.img /Notice/text/loginAlready`, rendered). The code was confirmed two
ways: `FUN_141b267c0` switches on `result + 1`, so `case 8` is wire 7 - and `case 6` in the same
table is `notRegisteredID`, which this repo already knew is wire result 5, so the offset is
checked against something that can disagree [L]. **Nobody has put that dialog on a screen yet.**

**2026-09-08 22:10: THE SECOND CLIENT DIED IN 6 MINUTES OF HEAP CORRUPTION, WITH THE GUARD PAGE
OFF - AND IT WAS OFF BECAUSE THE LAUNCHER STRIPPED IT.** The owner closed the 7-hour client and
pressed Start Game again. The new one lasted **6m10s**.

**Cause of death** [L], `dumps/maplecw-crash-517472-c0000374-1.dmp` (all six dumpwalk
self-checks pass): `0xC0000374` STATUS_HEAP_CORRUPTION, `_HEAP_FAILURE_INFORMATION` **type 8,
`heap_failure_block_not_busy`**, entry `0x382ad9a0`. The allocator's own captured stack names
our pool:

```
#2  ntdll.dll!RtlFreeHeap+0x51
#3  MapleStory.exe+0x19bbf3   fn 0x14019bb6a     <- the pool's free path
```

The pool freed a block whose header said it was not allocated. **That is precisely the failure
the guard-page quarantine exists to intercept** - its free is a pointer swap that never touches
the pool's free list.

**Why it was off, and this is a shipping defect, not bad luck.** The two session markers, read
off the two runs' own hook logs [L]:

```
14:55:25  mode=2,create=on,guardpage=0x20+0x40     <- written by test-server.ps1 -GuardPage (a PIN)
22:04:06  mode=2,create=on                          <- written by the launcher's Start Game
```

`launcher::client::DEFAULT_SESSION` in the source *does* carry `guardpage=0x20+0x40` - commit
`6a1f9e1`, **16:47**. The launcher binaries on disk are `target/debug` **12:35** and
`target/release` **14:55**. **Both predate the commit; it was never built.** So the running
launcher rewrites the marker on every Start Game using its stale default, and in doing so
**actively strips a guard-page pin that `test-server.ps1` had already put there**. "It ships to
every client" has been true in the repo and false on disk all day - `CLAUDE.md`'s *Built is not
wired*, and this entry claimed it was shipped.

**Fix before the next client**: stop and relaunch so the launcher is rebuilt. A running launcher
is a stale binary after any launcher change and nothing says so.

```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -Stop
```
```bash
powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\test-server.ps1" -SetFieldProbe
```

**The accidental A/B, same machine, same day, same client build, same servers** - fixtures
`guardpage-ARMED-survived-6h53m-2026-09-08-extract.log` and
`guardpage-NOT-armed-heap-corruption-6min-2026-09-08-hook.log`:

| | armed 14:55 | unarmed 22:04 |
|---|---|---|
| lifetime | **25 702 s (7h08m)**, ended by hand | **370 s**, ended by heap corruption |
| CLIENT FAULT lines | **0** | **1**, `0xC0000374` |
| sentry findings | 0 | 0 |

**n = 1 on each side, so this is not proof** - the baseline crash rate is 37.7 %
(`tools/crash_rate.py`) and a 6-minute death is unremarkable for an unprotected client. What
makes it worth recording is that the one variable that differed is the guard page, and the death
signature is the exact one it targets. It is also the first time the two arms have been run on
the same day.

**And an out-of-sample confirmation arrived free.** Yesterday's claim was that
`[0x143AC7F3C]` flips between 38 s and 194 s of process life, from 37 dumps. This dump did not
exist when that was written, lived 370 s, and reads **2**. `gatescan.py --dumps` is now
**38 dumps, 38 passing both controls**, still a clean split with no overlap.

**2026-09-09: CHAIRS, THE TRADE INVITE, SHANKS, and a grey fare line.** What is confirmed on a
screen and what is merely built, kept apart on purpose:

* **Set Up chairs - CONFIRMED ON TWO SCREENS.** Sit, stand, the correct model per chair, the
  idle tick scaled from the chair's own row, and Tester2 watching Cobalt sit and stand. The owner:
  *"Sitting in different chairs also reflects the correct chair model."* `net::chair`,
  `world::chairs`, `world::session::chair`. **Do not spend a run re-testing this half.**
* **The chair relay killed a client, and it is switched back ON.** `0x02AD` went out at 12
  bytes where the handler reads 13; Tester2 faulted `0xc0000005` five milliseconds later. I
  disabled the feature; the owner: *"No, this is unacceptable. The chair appearance across
  different client is an important part of the game."* Re-enabled with the three controls that
  were all missing the first time - a depth-6 read count, the pool head's own `u32` counted
  separately, and a test asserting the body is 13 bytes. The cause was skipping
  `tools/reads.py`, which exists **because** a short packet killed this client twice before.
* **MAP CHAIRS WORK - CONFIRMED ON A SCREEN AND IN MEMORY, 2026-09-09.** The owner: *"I do see
  that I sat on a bench."* `0x0252`, body `u32 characterId; u8 bSit; if (bSit) u16 seatIndex`
  - **7 bytes seated, 5 released**. `research/map-chair-seat-2026-09-09.md`.

  **Three instruments agree on one number, which is why this one is settled rather than
  probable.** The wire, the client's own memory, and the screen:

  ```text
  16:36:30.366 <- 0x00DA  2 byte body 1900          the client asks for seat 25
  16:36:30.367 -> 0x0252  UserSitResult: character 213 on map seat 25, 7 bytes
  chairprobe    +0x3c28 chair id = 25, chair obj 0x5a46728, checksum VALID
                -> IsSitting = TRUE
  ```

  Both of the probe's own controls passed on that read (`.text` rebase, and the session
  pointer matching what the hook last logged), and the chair object's ZtlSecure checksum is
  valid - so the seat index is a value the client built, not a byte we happened to read.

  **The whole cycle, with two clients, and it is not one observation.** The owner, later the same
  session: *"I tested the map chairs with two clients, that's all working now."* The log:

  ```text
    6  sits    0x0252, 7 bytes, bSit = 1     seats 23, 24 and 25
    5  stands  0x0252, 5 bytes, bSit = 0     every one after a 0x00DA ffff
    3  relays  0x02AD UserSitRemote          sat on seat N / stood up, to the map
    0  CLIENT FAULTs
  ```

  So the **release** works too - the shape that is 5 bytes with the seat field *absent*
  rather than `0xFFFF`, which is the opposite convention from `0x00DA` and was the easy
  mistake. That is the half that failed on 09-08, when a player could sit and not get out.

  **And the risk is retired, not merely unobserved.** `0x0252` had never been on a wire
  before this session; it has now gone out **eleven times with two clients up** and nothing
  faulted. The comparison that matters is `0x02AD`, which killed a client five milliseconds
  after its first send.

  **Why three attempts failed, and it was never the body.** The client has two remote
  dispatchers that resolve the target differently. `0x02AD` lives in `FUN_1429bb720`, which
  goes **straight to the hash** at `[pool+0xf8]`; the local player is not in that hash, so
  that opcode is *structurally incapable* of addressing the player who sent the request,
  whatever it carries. `0x0252` lives in `FUN_1429bafb0`, whose `GetUser` checks
  `[pool+0x10]` - the local user - **first**. That one difference is the whole failure.
  `0x0318` was also tried and **refuted on screen**: four replies, four retries, never sat.

  Verified here rather than taken on trust: the read count (two, `u8` then a `u16` gated on
  it), the table row (index 44 of `0x1429bb5d0` calls `FUN_1428341c0`), the index arithmetic
  (`lea eax,[rsi-0x226]`, `cmp eax,0x50`), the head consuming exactly one `u32` - the other
  two read sites are inside the `0x0226`-only inline branch - and both lookups read side by
  side. The control came free: index 11 of that same table is `0x0231`, which
  `crates/net/src/userchat.rs` recorded independently on 2026-08-30.

  **The release is `bSit = 0` with the field ABSENT, not `0xFFFF` in it** - `1428341ea`
  presets `-1` and the `u16` is `movzx`-widened, so `0xFFFF` would arrive as seat 65535.
  `0x00DA` uses `0xFFFF` for that meaning, so the two conventions are opposite.

  **Two things it could still fail on, and they are different outcomes.** `1428341d3
  call [rax+0x58]` runs before any read and a non-zero return exits having consumed nothing
  - undecoded. And the client re-validates position (`seatX-10 <= myX < seatX+10`,
  `seatY-30 <= myY < seatY+30`) and on failure **sends `0x00DA 0xFFFF`** - so "nothing
  happened" and "nothing happened plus a `0x00DA ffff` in the log" mean different things.
  `0x0252` is deliberately **not broadcast**: a bystander holding a stale position for the
  sitter would fail that check and stand *itself* up.
* **The trade invite popup - BUILT TODAY, NEVER SEEN.** The owner: *"the trade request pop up never
  showed up on Cobalt's side."* The whole cause is one field: `type` must be 1 or 2, and a
  scan of all 31 writers of the balloon-kind field shows the gated site is the **only** one
  that can build this popup, so a wrong value is a silent total failure. **Accepting still
  does nothing and that is a known gap, not a regression** - the trade window is `0x0575`
  mode 4, whose per-member body is dispatched through a virtual call on the open dialog and is
  undecoded, so it is answered with nothing rather than a guess.
* **Shanks sails Southperry to Lith Harbor - CONFIRMED ON A SCREEN 2026-09-09.** The owner: *"I did
  test Shank, it did work."* They quote 1000 mesos every time, including to a player who
  finished "Mai's Final Training"; the waiver comes only after Yes, with an extra line, then a
  free trip. A test asserts the opening never says "free". They previously answered *"no template
  for NPC 15"*. **Do not spend a launch re-testing the sail.** The one thing not separately
  attested is the *free* branch - that needs a character with the quest complete, and it is
  worth a look if one is to hand rather than a run of its own.
* **Fares print a grey chat line**, `"You have lost mesos (-500)"`, instead of the red
  "Meso Penalty" text. Built, unseen.

**2026-09-08: KEY BINDINGS now SAVE to the database, and restore is built but deliberately
switched off until one number is measured.** The owner bound three skills, clicked CONFIRM, and the
whole protocol came out of that one capture - `research/keyboard-layout-2026-09-08.md`.

* `net::keymap` - `0x0199` in (subtypes 0-3), `0x05F1`/`0x05F2`/`0x05F3` out. 8 tests.
* `store::keymap` - `character_keymap` (one row per bound key) and `character_keymap_option`.
* `world::session::keymap` - stores on CONFIRM, and sends the restore straight AFTER `SetField`.

**The save half works now.** Click CONFIRM and the bindings land in the database.

**The restore half is wired and currently sends NOTHING**, because of a constraint that is easy
to miss and expensive to get wrong: **`0x0199` is a DELTA and `0x05F1` is the FULL 89 slots.** The
client diffs against a shadow table that `0x05F1` refreshes, so for a character that has never
synced, the thing it diffed against is its own **factory layout - which is nowhere in this
repo**. Sending the read gate with only the keys we know would unbind the other 86 on the
player's keyboard. So `net::keymap::CLIENT_DEFAULT_LAYOUT` is `None` and `restore` returns
`None` until it is measured. Two tests assert that, so nobody can "fix" it into shipping an
empty table.

**And it sends nothing rather than the keep-gate packet**, which was the first plan and was
wrong: the client's no-op path behind that gate (`0x1419ffd21` onward) has not been read, while
sending nothing is exactly what the server does today on a client that survived seven hours.
Preserve the measured-good behaviour; do not trade it for an unmeasured one to save a branch.

**To switch it on**, with a client running, from an ELEVATED shell:

```bash
cd "C:\MapleCW"; python tools\keymapdump.py --rust
```

`FUN_1401de850` is `lea rax,[rip+...]; ret`, so the manager is a **static** at `0x143274460`
and the factory table is the shadow at `+0x1bd` [L]. It stays factory on any client because we
have never sent `0x05F1`. The tool has three controls - rebase, 10..80 slots bound, and Q/W/E/I
all bound - and refuses to emit a table if any fails, because a wrong table moves every key a
player has. Paste the output over `CLIENT_DEFAULT_LAYOUT`.

**Known incomplete, recorded rather than guessed:** the KEY BINDINGS dialog has a **Preset 1/2/3**
selector and the client reports a switch as `0x0199` subtype 3, but **no server-to-client opcode
for it was found**. A preset change is logged and not stored, because storing something we can
never send back is worse than not storing it.

**2026-09-08 LATE: the guard page is absorbing the overrun, not detecting it - and the gate that
arms the writer is NOT in the executable.** The owner asked whether the writer hits predictable
locations, whether we could reserve those chunks for it, and whether our stub `grap64.dll` is
implicated. `research/the-180-second-family-is-anti-cheat-2026-09-08.md` §9, three answers:

* **The offset is a compile-time constant; the address is not.** `&v[0] + K` where the base is
  whatever slot the pool hands out. Nothing to reserve, because the writer aims at a *distance*.
* **But reserving the SLACK works, and the guard page already does it by accident.**
  `SLOT_BODY_OFF = 0x10`, one slot per 4 KB page, so every known `K` (max `0x220`) lands at
  `page + 0x10 + K` - inside our own committed page, 3.5 KB clear. **The instrument built as a
  stale-access detector is functioning as an overrun absorber**, which means `0 confirmed finding`
  at three hours is *not* the same claim as "nothing has gone wrong". **Test that reads the answer
  out of a client that is still running:** scan `page + 0x90 .. page + 0x300` of live quarantined
  slots for non-zero dwords at each heartbeat. Pages are handed out zeroed; a repeating non-zero
  dword is the writer caught in our padding, with no crash and no dump. It also settles the
  overrun-vs-stale-pointer question §6 left open.
* **The stub does not cause the write** [L] - the array size and the write offset are both
  compile-time immediates in `MapleStory.exe`'s own `.text`, with no `grap64` call between them.
  **Whether the stub ARMS it is undetermined, and the previous write-up was over-confident.** The
  live writer needs `[0x143AC7F3C] >= 2`; `dump_va.py` says that address has **no file bytes**
  (uninitialised `.data` tail), and an opcode-agnostic RIP-relative scan of `.text` - not
  `dataref.py`, whose own docstring warns its opcode table has been short before - finds **one**
  reference, the `cmp` that reads it. Something outside `.text` writes the 2 every session. The
  "byte-identical across three sessions" argument never had power here: all three sessions ran the
  same stub, hook and patched client.

**MEASURED, same evening: the gate is written between 38 s and 194 s of EVERY session.** The owner ran
the scan against the 3 h client; both controls passed and it reads `[0x143AC7F3C] = 2`,
`[0x143AC7F70] = 2`, everything else 0 - identical to the dumps, so "the gate is a consequence of
whatever leads to a crash" is dead. But `gatescan.py --dumps` reads the block out of **all 37
archived minidumps** behind the same controls, and **two of them read 0**: the two shortest-lived
sessions there are, at **11 s and 38 s**. Every dump from **194 s** to **7015 s** reads 2, and so
does the live 3 h run. A clean split with no overlap. **That kills "static configuration"** - the
disagreeing sessions were in `dumps/` the whole time and were never counted, which is the
cash-shop failure again: an existing archive answering a question nobody asked it. `0x143AC7FDC`
also moves (0 in 35, 2 in two dumps and 2 live, needs 3) - a second gate one step from opening.

**Next, and it costs no extra launch:** run `gatescan.py` at ~60 s and ~150 s of the next client's
life. The 38→194 s window brackets 180 s and this module runs on a 180 s clock, so a flip at 180 s
is the module's own clock; a flip at 40 s is something else, and something else is where our
environment could be implicated.

**Correction, made the same evening:** `0x143AC7F24` is `GetCurrentThreadId + 0xF010FA1`, **not**
`GetTickCount` as first published here. Subtracting the constant gives the pid in each dump's own
filename (`356516 -> 356520`, `322016 -> 322020`), and the value is in the dump's **own thread
list in 37 of 37**. The first run of that test said 0 of 37 - because it read the wrong dict key
and found zero threads in every dump, a negative from an instrument that could not have produced
a positive. It was caught only because the thread count was printed beside the verdict.

**`tools/gatescan.py` (new) reads all eighteen gates out of the RUNNING client**, read-only
(`PROCESS_VM_READ`, the same pattern `dump_runtime.py` already uses), refusing to report unless a
rebase control and a block-fingerprint control both pass. **It needs an elevated shell** - the
client runs elevated and `OpenProcess` returns error 5 otherwise. If the gate can be seen at 0 in
any healthy session, the stub question closes without ever loading the real DLL (which installs a
service and the `BlackCat64.sys` kernel driver, and is not worth it).

**2026-09-08: the guard-page quarantine now SHIPS in the launcher, and it has a kill switch
that needs no rebuild.** The owner: *"work under the assumption that if this works, all of the clients
should have it."* `launcher::client::DEFAULT_SESSION` is now
`mode=2,create=on,guardpage=0x20+0x40`, so **every** Start Game arms it - not only
`tools/test-server.ps1 -GuardPage`, which writes a pin and only ever reached the owner's machine.
That gap is exactly why the 01:33 overnight run carried no `guardpage=` token.

`0x20+0x40` and not `all`: those are the two classes damage has been seen on, and the shared
8 M-slot cursor is 2.8x one measured first window at two classes but only **1.4x** at four -
below the floor `render_armed` itself shouts about, so `all` would ship a client that falls back
mid-session and looks healthy doing it.

**To turn it off for one player: create `maplecw-hook.guardpage.off` beside their
`MapleStory.exe`.** For a whole install: `guardpage = "off"` in `maplecw-launcher.toml`. Absent
means ON. Either one makes the launcher write a session marker with **no `guardpage=` term at
all**, which is byte-for-byte the client that ran before the feature existed, and the log pane
says which way it went in words a player can read back. `--print-paths` prints `guardpage on` /
`guardpage OFF` with both switches consulted. Both directions are tested; a pin still overrides
the default and the off switch still wins on that one token.

Cost to a player: 32 GiB of *reserved* address space (no commit charge), ~350 MB of working set
[D/I], and two patches to the client. **Every failure path in `guardpage::arm` was read and
leaves the client unpatched - except one, which was a real shipping bug and is now fixed**:
`identity::install_detour`'s read-back check returned `None` *after* the jump bytes were already
over the live prologue, so "None means nothing was patched" was a comment describing a guarantee
rather than enforcing it. It now restores the original prologue from the trampoline's verbatim
copy, under the same thread-parking discipline, and **reads it back** rather than claiming a
success it has not seen. **It has run on exactly one machine, the owner's.**

**This client crashes mid-session, so the state is a LEASE and not a flag**
(`crates/store/src/presence.rs`). It is held by a live connection and released when the socket
closes - **including when it closes because the process died**, which is the common case here -
and expires by itself sixty seconds after the last renewal if a server process is killed
outright. A crashed player waits nothing in the ordinary case, at most a minute in the worst,
and needs no manual step. The lease is keyed on the client **process**, so Log Out, Choose
another world, character select and Change Channel all re-take their own lease; a socket-keyed
one would have locked players out of themselves. Character select hands the lease to the channel
rather than releasing it, or a second client could slip in during the handover.

Measured over real sockets by `tools/claims_smoke.py` (20/20), which now kills a holder's
process with no logout to prove the account frees. 2181 workspace tests.

**2026-09-07: the heap corruption is on a 180-SECOND CLOCK, the repair ships, and the next
run NAMES THE WRITER.** Six catches across three sessions, four intervals, every one 180.0 s to
within 0.12 s, generated by a client ticker template that re-arms its own timestamp on the
firing branch (`research/the-180-second-clock-2026-09-07.md`). The launcher arms
`repair=on` on every launch, which puts the damaged header back and turns the fatal free into
an ordinary one - 26 minutes, 7 catches, 7 repairs, no death, against the same idle session
dying at 23 minutes the night before. **It is a mitigation, not a fix**, and the field crash of
2026-09-07 proves it: the same writer surfaced at a *different* free - the WZ property/VARIANT
teardown during a map change - where the repair structurally cannot reach
(`research/second-crash-family-2026-09-07.md`).

**Run 1 of the write watch (2026-09-07 19:40-20:50) missed the store and moved three
things**: the writer **re-hits** slots it has hit before (three of twelve repaired headers were
damaged again, one twice - the increment, on a stable stale pointer); the sentry's
report-once rule suppressed every re-hit and that silence let catch #12's slot be re-damaged
and freed - **fixed, a repaired slot is reported again**; the 28-byte allocation from
`0x14491cafd` (Themida region) fires every 180 s ~100 ms before every catch, 14 of 14; and
the watch covered only 10% of the pool - **fixed, caught headers' pages are pinned**. Run it
again. `research/the-180-second-clock-2026-09-07.md` §7.

**Runs 2 and 3 (same evening)**: run 2 died at nine minutes on an unrelated surface (an empty
`std::map`'s head-node pointer incremented by 2 in a `0x40` slot - §8 of the clock file); run 3
went 42 minutes, **12 firings, 12 catches, none silent, pool clean at close** - the re-hit fix
works and a third of firings re-hit - but the watch opened **one** window: its re-arm guard
reset with every catch. Fixed (`window_due`, tested). Same recipe again.

**Run 4** (50 min): eleven windows on schedule, eight pinned pages - and the store landed
**inside a window on a pinned page, uncaught**: the first write to a page opened it until the
next 5 ms sweep. Fixed with `probe.rs`'s own trick - the page is open for one instruction (trap
flag, re-protect on the step), so every write faults. §10 of the clock file. Same recipe again.

**Run 5 died at 3.5 min on a DIFFERENT surface the write watch cannot reach**: the ResMan
worker-thread teardown (`heap-wild-write.md` dump 2, exactly - not new), reading a `0x40`-class
empty-map node through a pointer that was **+2**, the same increment signature as run 2 (§8, §11
of the clock file). Bucket 2, a live node, a pointer not a header, before any window opened - so
nothing built for the `0x20` pooled-free family covers it. Two of five write-watch runs (2, 5)
died this way. The `0x20` write watch is still one door-close from its own writer (run 4);
covering the `0x40`/worker surface needs the guard-page build, which is a deliberate schedule,
not a window tweak.

**2026-09-08 14:47: two answers and one self-inflicted wound.** The multi-class build plus
five `watch@` targets, and **the client closed the instant it entered the field** - the first
death in this investigation with **no exception and no crash dump at all**, which every previous
one produced.

**Two things settled [L], and they are worth the run on their own:**
* **The `0x40` free path IS intercepted.** Both first-free controls fired within a second:
  *"the FIRST free of class 0x20 / 0x40 came back through our HeapFree shim"*. That was the
  biggest unknown in the multi-class change - 53 of the client's 56 free sites are inlined and
  untraced - and it is now measured rather than inferred.
* **`FUN_140c93530` is LIVE.** `WATCH #1: 0x140c93530 ENTERED ... while dispatching opcode
  0x01A0` (SET_FIELD), called from `0x142892bf9` - which matches the static call site
  `0x142892bf4` exactly. So the five `.text` tick functions found by decompiling are **real code
  that runs**, not a copy Themida replaced. The decompilation lead is alive.

**The wound was mine, twice.** The spec `watch@A,watch@B,...` repeats a prefix the hook strips
**once**, so four of the five were refused one log line at a time and only `0x140c93530` armed.
And I told the owner to change **two things at once**, which is the rule this repo already writes down
("test one variant at a time. Changing two things at once has already produced one unexplained
crash"). So the guard page is **not** the suspect - it armed, control PASSed, both controls fired
and the client ran 30 s through login and character select - but that is a suspicion, not a fact,
because the run cannot separate them. The launcher now **refuses a repeated `watch@`** and any
target that is not a hex VA or `module!export`, proven both ways.

**Next run: the guard page ALONE.** If it survives field entry, the int3 watch was the cause.

**2026-09-08 12:01, THE GUARD PAGE'S FIRST CLIENT RUN: it works, the client lived 1h57m, and it
died on a class we were not watching.** 12:01:18 -> 13:58:14 is the longest session this project
has had. `GUARD PAGE ARMED: size class 0x20 ... control PASS` [L], and an inline hook on the pool
allocator - called from thirty threads thousands of times a second - did not destabilise
anything. **That question is closed.** Zero stale-access catches, but that is not a clean
negative: from minute six the class was only partly covered.

**The reserve ran out at six minutes.** From the heartbeats [L]: 627 172 slots served in the
first minute (a startup burst, ~10 000/s), then a flat 1 560/s. The 1 048 576-slot cursor was
spent at 06:00, four minutes before anything could age out of the 10-minute retirement queue, and
**419 588 allocations fell back** before recycling began at 10:00. Steady state alone
(1560 x 600 = 936 000) would have *fitted*; **the burst is what broke it**, and the model
reproduces the measurement to 0.4 % [D]. Live `0x20` slots held at ~26 000 all afternoon, ~104 MB
of committed pages, against the ~230 MB predicted. **Memory was never the constraint; the cursor
was.**

**The fatal object was a `0x40` slot whose vtable pointer had been incremented by 2** - it reads
`0x143406c02`, and `0x143406c00` is the genuine vtable, all four of its slots pointing into
`.text` [L]. **[I]: the class is incidental.** The writer holds a stale ADDRESS, and whichever
bucket's chunk is later carved over it is the victim - `0x40` map node `+2` (runs 2 and 5), `0x20`
tree node set to `-1` (the overnight run), `0x40` vtable `+2` (this one). Quarantining one class
is whack-a-mole.

**Both fixed in `guardpage.rs` (never run on a client).** Several classes from one shared reserve
(`guardpage=0x20+0x40`, `all`, any `+`-joined subset - **not commas**, the marker is
comma-separated); the default is now `0x20+0x40`, which contains all three deaths. One reserve
serves every class because the stamp `0x100` is above every rung of all three of the client's free
ladders, and those ladders decide on the **header value alone** [L]. **8 388 608 slots** (32 GiB,
with a fall-back ladder if it cannot be had) - 5.7x one class, 2.8x two, only **1.4x all four**,
which the ARMED line calls out in words rather than printing a bare "1x". `REUSE_AFTER_MS` **stays
600 s**: it comes from the writer's clock, not the reserve, and the cursor is the cheap thing to
grow. The metadata array is **committed lazily** - 40 bytes a slot is 320 MB at the design size,
and that array, not address space, is what pinned the reserve at 1 M. **Per-class counters**,
nothing summed, plus **`pool allocations seen by class`** for all four whether watched or not:
only `0x20`'s churn has ever been measured, and those numbers decide whether `all` is viable. And
**two new controls**, because 53 of the client's 56 free sites are inlined and untraced so "a new
class's frees reach the pointer we swapped" is an [I], not an [L]: the first free of each class
logs a line, and the heartbeat shouts `NEVER FREED` if a class is served and nothing comes back.

**All four at once is coherent but is NOT the next run** [D/I]: 1.4x headroom on an unmeasured
assumption, 3-4x the decommit rate (a TLB shootdown across every core each time) of the only
configuration with a client run behind it, and it blinds the pool sentry - with every class served
from the reserve the pool stops carving, so "pool clean" becomes vacuous.

**2026-09-08: the GUARD-PAGE quarantine is built (`crates/grap-stub/src/guardpage.rs`,
`-GuardPage`).** It covers the surface the write watch cannot: it serves one pool size class
(default `0x40`, bucket 2) one-slot-per-page from a private 2 GB reserve and DECOMMITS each slot
on free, never reusing the address, so a stale pointer into freed-and-reused memory - the
writer's habit behind every run - faults at the instruction that uses it, on any clock. The
handler logs RIP + who-allocated + who-freed and recommits so the client survives. Alloc is one
inline hook (prologue-checked, `identity::install_detour`); free is a HeapFree pointer swap
(`freeguard`'s technique); a self-test must catch a control write before it arms. Off unless
`-GuardPage`. Run it WITH `-SentryWriteWatch` (0x20) + `-PinPatches`: the two cover both
surfaces in one launch. Not yet on a client. **Reviewed before its first launch** (the owner: *"make
sure you agree"*; `research/guard-page-2026-09-08.md` §6): three defects fixed - the allocator
detour could be entered before its trampoline was stored (a crash at RIP 0 at arm time; the
trampoline is now published first and other threads are parked outside the prologue while the
jump is written, in `identity::install_detour` for both its users), the "freed from" address
would always have read the pool's own free (`0x14019bbf3`) instead of the freer, and `probe.rs`'s
handler - registered later, so it runs first - would have logged every catch as a `CLIENT FAULT`
and written a crash dump before the guard page's handler saw it. A fourth was found **in the
first fix**: it parked the threads and then called `VirtualProtect`, which takes the lock a
thread suspended inside `VirtualAlloc` holds - a deadlock of the client at arm time. Both
protect calls are now outside the parked window, which is the rule `poolsentry` already
stated. Confirmed off the listing while
reviewing: both pool frees read the one `HeapFree` slot (60 readers in all), the ladder takes a
`0x100` header to that arm, `HeapFree` gets `body-8`, and the allocator overwrites `rax` before
reading it, so the trampoline's clobber is safe. 97 grap-stub tests.

**2026-09-08: DECOMPILING FOUND FIVE CANDIDATE WRITERS, and it cost no client run.**
`research/the-180-second-tick-family-2026-09-08.md`. Scanning `.text` for the 180 000 ms
immediate (with 60 s and 30 s as controls, 98 and 154 hits) finds 42 sites, **16** of which are
the ticker template `FUN_1408fcaa0(last, 180000, now)` with `LAST := now` on the firing branch -
the shape predicted from timing alone last week, now read off the listing. **Five of the sixteen
allocate a small array, take element 0's address, add a fixed offset, then LOAD a 32-bit value,
INCREMENT or DECREMENT it, and STORE it back** [L]:

| function | op | offset |
|---|---|---|
| `FUN_140c93530` | inc | `+0x90` |
| `FUN_140c936a0` | inc | `+0x94` |
| `FUN_140c93810` | dec | `+0xc0` |
| `FUN_140c93b70` | dec | `+0xe4` |
| `FUN_140c93930` | dec | `+0x220` |

**Two increments and three decrements explain the observed value family `1`, `2`, `-1`**, which
no single-operation hypothesis does. Each calls `FUN_140ca61d0` with `edx=5` - the 28-byte
allocation already fingerprinted as firing 180 s apart ~100 ms before every catch, 14/14. Found
first by watching the client, now found again by reading it. The controls are what make it mean
something: 240 s has 3 tick functions and **0** with this shape, 90 s has 0, 60 s has 22 and
**1**. All five are live code with real call sites.

**NOT established, and the file says so at length**: none has been observed executing - the live
caller recorded in the runs is `0x14491cafd`, inside `.themida`, so [I] Themida likely runs a
copy and `.text` gives us the logic rather than the instruction. And the tempting overrun reading
(`+0x90` is 144 bytes past a 28-byte buffer) **could not be confirmed against either crash dump**,
because the buffer is a per-tick local that is freed at the bottom of the tick - by crash time
that memory has churned, so the dump cannot see what was there. A negative from an instrument
that cannot observe the thing is not evidence. Mechanism still open between an overrun and a
stale base pointer.

**What it buys**: five addresses to watch, free, on a run that is already planned -
`-Probe "watch@140c93530,watch@140c936a0,watch@140c93810,watch@140c93b70,watch@140c93930"`.
They fire and the writer is one of five known instructions; they never fire and the Themida copy
is what executes, which closes this direction cheaply. Either answer is progress.

**2026-09-08 EVENING, three of the five have now been ON A SCREEN.** The owner, from a live
session: *"The drops so far are good, I have not seen a quest item drop ... !tool seems to be
working correctly."* So **quest-item filtering is CONFIRMED** (the negative case - no quest item
dropped without the quest; the positive case, taking the quest and seeing the piece return, is
still unwatched) and **`!tool` is CONFIRMED** (its daily refusal is still unwatched). Drops in
general are confirmed good.

**One is confirmed still BROKEN, and the report names the case**: *"Mob drop placement is still
wonky, particularly when a mob is jumping, the loot drops below the current platform."* The
walking half is fixed - taking the path END instead of the head removed the median 41 px lag -
but a JUMPING mob still places wrongly, and downward. **[I] the likely cause is the path element
decode**: `dropsite.rs` treats movement path elements as uniform with xy at a fixed offset, and
MapleStory path elements carry a type with different bodies per type, so the "last element" read
lands on the wrong bytes for a jump. The original measurement's unexplained **2.9% residue** is
the thing to check that against. Under investigation.

**2026-09-08: five live-server fixes, all NEW and none seen on a client yet.** 2152 workspace
tests.

* **THE MESO DROP FROZE THE WHOLE INVENTORY, and it is the worst of the five.** The owner: *"I have
  attempted to drop 10 mesos and 5000 mesos, none of these attempts worked, but I lose all
  functionality in being able to interact with my inventory."* `0x0143` `UserDropMoneyRequest`
  was **not handled at all** - it fell to `dispatch`'s `_ => Vec::new()`. The client's own
  builder `FUN_142d4cb40` sets an exclusive-request latch at `player+0x2330` **the moment it
  sends** [L], so silence there does not fail a drop, it kills the inventory, the ability-point
  buttons, the cash shop and the item drop for the rest of the session. The unlock is `0x007C`
  with `bExclRequestSent = 1` and an **empty mask**, and the client's handler does that unlock
  as its literal first action, before any mask work [L]. Mesos still cannot be dropped;
  `crate::mesodrop` refuses AND answers. **69 latching opcodes are now enumerated** and a
  measured whitelist arm answers any of them that nothing else handles - `0x01FD` and `0x02F6`
  have also arrived unhandled in the archive and froze a client each. Item drops never had the
  bug (fixed 2026-08-19); mesos were simply never wired.
* **MOB DROPS LANDED WHERE THE WALK BEGAN.** `on_mob_move` stored the `0x02FF` **path head**,
  and the head is the position at the START of the path. The old comment granted that and called
  it *"a few pixels for a snail"*; that was never measured and is wrong. Over **642 431
  deduplicated** reports the head lags the mob by a **median 41 px**, and by more than 25 px -
  half the client's own pick-up box - **62.8%** of the time. `tools/mobmove_lag.py` re-derives
  it with three controls. Now the path END, plus `Fields::mob_site` so a mob that never reported
  drops at its `Map.wz` spawn pixel instead of **at the player's feet**.
* **QUEST ITEMS ONLY DROP FOR SOMEBODY WHO HAS THE QUEST.** The marker is item metadata the repo
  already parses for the "quest items cannot be sold" rule, so the two rules cannot drift; 119
  of 2785 items carry it. The ETC id range is **not** the marker and using it would have deleted
  242 ordinary drops (Snail Shells, Mushroom Caps) that quests merely consume. 12 flagged ids sit
  in the live table over 15 rows; 8 ids / 9 rows are filtered, the four Dark Marbles exempt
  because `secondjob` already gates them more narrowly by mob and map.
* **LEVEL-UP IS BROADCAST.** `0x02AF` `UserEffectRemote` effect 0, so other players on the map see
  and hear it. Corroborated three ways: both client dispatch tables enumerated whole, and the net
  crate already carried both the opcode and the effect id read off the client.
* **`!tool` IS A PUBLIC COMMAND** giving three daily favours: 1000 Leaf Points (per **account**
  per UTC day), one level, and reset AP & SP (each per **character** per UTC day). Claim state in
  `daily_claim`, keyed `(scope, scope_id, perk)`, gated on `stored < today` so a backwards clock
  cannot reopen it; the claim is one `BEGIN IMMEDIATE` and a failed grant releases the day.
  **The placed Maple Administrator keeps their quest 500005** - they have no branch on their template
  anywhere in `session/npc.rs` any more.

  **Why a command and not a summoned NPC, which is what was asked for.** The client's click fork
  is keyed on the **template**, not the object id [L] (`research/npc-click.md` §2): an NPC whose
  template has an offerable quest sends `0x0151 {questId, npc TEMPLATE id}`, and only the
  quest-less path sends `0x00F2 {npc OBJECT id, ...}`. A summoned copy drawn as 9010000 would
  therefore send bytes **identical** to clicking the real Administrator, so restoring their quest
  and routing a copy's click are the same fork pointing two ways - you can have one. A runtime
  spawn itself is fine and proven (`!npcecho` put object ids 6000/6001 on screen after field
  entry [L]); it is the *click* that cannot be told apart. So the "new NPC" is a **speaker, not a
  field object**: `0x055B`'s speaker field is the icon loader, so the box carries the Maple
  Administrator's portrait literally, and the conversation is told apart by
  `Conversation::path == "dailyperk.menu"`. Nothing is spawned, so nothing leaks across a map
  change, a relog or ten `!tool`s.

  **The escape hatch, named and not taken.** Four other templates are also called Maple
  Administrator and carry no quest - `800016`, `900000`, `900001`, `900002` - so one of them
  would fork to `0x00F2` and be routable by object id as a real walk-up NPC. It was declined
  because nobody has rendered those canvases, and a missing sprite is exactly the silent failure
  this repo keeps meeting. **That is the owner's call to reverse if they want a walk-up NPC.**

**2026-09-08 OVERNIGHT RUN, 8m13s: THE POOL WAS CLEAN AND THE CLIENT DIED ANYWAY.**
`research/the-writer-damages-live-objects-2026-09-08.md`. The most important result this
project has had, and it invalidates how success has been measured.

The dump says: `0xC0000005` reading `[rax+0x19]` in a **red-black tree descent**, with
`rax = 0xffffffff301bad30` and **`rdx = 0x00000000301bad30` - the same pointer, uncorrupted, in
another register** [L]. The victim is node `0x3a2f9a78`, a **`0x20`** slot, whose `_Left` holds
the tree sentinel intact and whose `_Right` holds **the same sentinel with the high dword smashed
to `-1`**. Same node, same value, one damaged. A scan of all 1216.9 MB found exactly 2 copies of
the corrupted qword against 13 of the correct one as a positive control [L].

And `tools/poolchain.py` over that dump: **0 damaged headers in 174 528 slots, all four
buckets.** The sentry caught and repaired its one finding, and the pool went to its grave in
perfect health.

**So the damage that kills is inside a LIVE object's payload** (`body + 0x14` here), not on a
free header. The sentry validates `body - 8` against the slot size and nothing else - it is not
failing at this, it is structurally incapable of seeing it. **Every "pool clean, N repaired" line
is true and says nothing about whether the client is about to die**, and the header family we
have chased for weeks is only the part of the writer's output that happens to land on a header.

`-1` is in the known value family (`1`, `2`, `-1`) and the offset is `+4` into an eight-byte
field, exactly where `0x20` became `0x0000000100000020`. **[I]** a refcount at `+4` on a freed
object explains `+1`, `+2`, `-1` and a 180 s timer in one mechanism.

**Two instrument failures, one of them mine.** The run carried **no `guardpage=` token** - the
session marker reads `mode=2,create=on`, so the guard page never armed and never logged. And it
would not have mattered: `-GuardPage` defaults to bucket **`0x40`**, chosen from runs 2 and 5,
while this victim is **`0x20`**. The flag was missing *and* aimed at the wrong class. The right
run is **`-GuardPage`** with the new default `0x20+0x40`. The ~230 MB budget was wrong by more
than a factor of two: the 12:01 run measured ~26 000 live `0x20` slots, ~104 MB [L]. `FELL BACK`
was indeed the first number to read, and it was **419 588**.

**2026-09-08 EVENING: THE POOL WALK HAS ONLY EVER SEEN ~70% OF THE POOL.**
`research/damage-enumeration-2026-09-08.md`. `tools/poolchain.py` **and the live sentry** follow
one chunk list per bucket from the pool context. There are **819** chunk lists [L]; enumerating
by chunk shape finds 3.1-3.4x more chunks, the walked ones a strict subset. **Proof, in the dump
I analysed myself**: the `0x40` object whose vtable pointer was incremented by 2 - the one that
killed the 12:01 run - is **not on the walked list** (1967 chunks walked from the context head,
victim not among them) [L]. So **every "N slots, 0 damaged" line this project has printed means
"0 damaged where we looked"**, and my own "the pool was clean at death, 0 in 174 528 slots" is
corrected in place. 14 of the 59 damaged objects now confirmed across 37 dumps sit where the
sentry cannot look, **which also means the sentry's REPAIR misses them** - that is a live defect
in shipping code, not just in analysis.

**The enumeration answered the questions the single-victim studies could not:**
* **0 to 13 damaged objects per session, 59 across 37 dumps.** Single digits - not one, not
  hundreds. Two processes are clean with their controls intact.
* **The offsets are consistent once you look from the WRITER's object, not the victim's.** The
  two populations are `body-4` (29 headers) and `body+0x14` (32 `_Tree` `_Right` fields), which
  looked contradictory. But `body_k + 0x24 == header_{k+1} + 4`, so **one object with 32-bit
  counters at `+0x14` (decremented) and `+0x24` (incremented) explains both** - and `+0x24` on a
  `0x20` slot is exactly the 4-byte overrun `poolsentry.rs` hypothesised months ago. The two
  competing readings were the same write pair all along.
* **A read-modify-write, watched directly across successive dumps of one process**: `-1 -> -2 ->
  -2 -> -3` on one `_Right` high dword, and `+1 -> +1 -> +3` on one header [L]. Nothing heals,
  nothing moves.
* **A strong TYPE cluster, no address cluster**: 11 of 16 are MSVC `_Tree` nodes, 7 of them empty
  -container head nodes, and the damaged field is `_Right` every time.
* Corrects another published number: "14 of 14 identical `0x0000000100000020`" is **23 of 29**
  over 37 dumps; the rest are `+2` and `+3`.

**2026-09-08 EVENING: THE 180-SECOND FAMILY IS THE CLIENT'S ANTI-CHEAT, AND THE CORRUPTION
LOOKS DELIBERATE.** `research/the-180-second-family-is-anti-cheat-2026-09-08.md`. **This forces
a correction to the entry below it - read both.**

The module holding the five (now six) 180 s tick functions carries **deliberately obfuscated**
strings [L], verified independently by me with a tolerant search after a plain one found
nothing: `Crc Fail Alert!!`, `CheatEngine`, `AccountId`, `RegOpenKeyTransactedA`, and three
cheat-tool names spliced with CR/TAB bytes so a `strings` dump misses them -
`J
i	n6	4.dl	l` is `Jin64.dll`, and likewise `ROYAL Connector` and
`Royal.Secure.Runtime`. **My earlier "no strings referenced in the surrounding 96 KB" was
wrong twice over**: a linear disassembly sweep desyncs the moment it crosses data, and the
string test would have rejected these anyway because of the embedded control bytes.

The writes are **out-of-bounds by construction** [L]: each tick allocates a small `int` array
from the pool, and writes a fixed compile-time offset **past its end** - `+0x90` is `v[36]` of a
5-element array. There is no pointer to be stale. And the agent traced the chain: a cheat-name
detector sets a writer's enable flag, sends a report packet, and sets that writer's start clock
to **now + 0x2BF20 (180 000 ms)**. Detect, report, then 180 s later begin corrupting the heap.

**Which of them fires, measured in two dumps by me** [L]: the six readable ones are gated on a
list size that reads **1** and they need **>= 7**, so they have never fired. The gate for the
**virtualised** sibling `FUN_140c93c80` (whose `.text` body is a `jmp` into `.themida`) reads
**2** and needs **>= 2** - it passes. The archived `rdx=0x5` allocation that precedes every
finding by ~74 ms comes from a Themida return address, which is why `callers.py` could never
find its caller.

**THE CORRECTION.** The entry below says the live player's crash proves the writer is "a
property of this client, not of our machine, network or server". **That was too strong and I
committed it.** That player runs our launcher, our patched client and our injected hook, so
they were never an independent control - `is-the-corruption-ours` §1's missing control is still
missing. What the live crash does establish is that it is not specific to the owner's *machine*. It
does **not** separate "the client does this unprompted" from "our tampering trips the client's
own anti-cheat, which then sabotages the heap on purpose". With `Crc Fail Alert!!` sitting in
this module and our hook patching `.text`, the second reading is now live and was not before.
Against it: the two gate values are byte-identical across separate sessions, which reads more
like configuration than a detection count [I].

**What follows.** If tampering is the trigger, the fix is to stop tripping it, and the whole
quarantine becomes unnecessary. The proposed probe is to write `0` to the gate at
`0x143AC7F3C` from the hook - four bytes of DATA, nothing in `.text`, so no code CRC can see
it - and watch whether the 180 s `rdx=0x5` allocations and the findings both stop. It is
self-verifying either way. **Risk, stated:** a module that answers detection by corrupting the
heap on a timer may answer interference the same way, on a delay.

**2026-09-08: THE WRITER IS ON OTHER PEOPLE'S MACHINES, and it is killing live players.**
Four hook logs off the live server (`research/live-client-crash-2026-09-08.md`). One is a crash,
and it closes a question this project had left open. A live player's client - a different person,
a different computer - reported **the identical damage signature**, `0x0000000100000020` on a
bucket-1 `0x20` header, found and repaired by the sentry at 00:03:35.
`research/the-180-second-clock-2026-09-07.md` had recorded the worry plainly: the writer *"has
only ever been observed in our environment"*. It is a property of the client, not of the owner's
machine, network or server.

The repair worked and **was not enough**: 80.5 s later the same client died of `0xC0000005` at
`0x14094e1d0`, which is `FUN_14094e150`, a **destructor** walking a linked list -
`mov rbx,[rcx]` reading a node's `next` pointer through a bad `rcx`. Two things follow. First,
`0x14094e190` - filed here as a *close-time* fault, "recorded, not chased" - is **the same
function**, its other teardown loop; and the live one fired 4 m 53 s into an **idle** session,
so the "close-time" label was wrong. Second, the nodes are `0x38` bytes freed through
`operator delete` at `0x140205820`, which loads the pool context `0x143AD68A0` and calls the
pool free `0x14019bb50` [L] - so they are **pool bucket-2 objects, the `0x40` class**. That is
exactly what the quarantine covers by default - and **as of 2026-09-08 it ships**: the
launcher's `DEFAULT_SESSION` carries `guardpage=0x20+0x40`, so these users get it on their next
launcher build. See START HERE for the kill switch.

Still circumstantial, and the artifact that settles it exists: a **1.2 GB dump on that player's
machine** (`C:\MapleCW\dumps\`). The faulting `rcx` is the whole question - off by 1 or
2 in the low dword is the writer's signature and nothing else's. Also free and worth adopting:
`os error 10054` per session in `world.log` is the **server-side fingerprint of a client dying**
(a clean logout closes gracefully), and in the one window the server logs cover it was 2 of 2.
The server logs supplied do **not** cover the crash window (a gap between 22:09 and 05:58 UTC).

**Nothing authenticates.** Login carries an identity token kept only as SHA-256, but the
**game socket still carries no credentials** - and these peers include a public address.
(That token was one-time when this was written; since 2026-09-08 it is honoured until its claim
expires. See the top of START HERE for what that widened.)

**2026-09-08, the goal changed: the owner wants the client to survive a night, not to be measured.**
That is test plan **T20** and a different command -
`-SetFieldProbe -PoolSentry -SentryQuiet -SentryRepair -GuardPage -PinPatches`, with **no**
write watch (it observes and protects nothing, at ~160 windows of read-only pages over eight
hours). Checking the guard page against that goal found it could not have lasted: the cursor only
advanced, so the 2 GB reserve was a budget of 512 K *total* allocations - under nine minutes at a
thousand a second - after which it silently fell back to the client's pool. The justification for
"the reserve is far more than a long session needs" rested on a **misread counter**: the pool
field at `ctx+i*4+0x14` is `inc`/`dec`ed around alloc and free [L], so 18 758 is `0x40` objects
*live*, not allocations served in 50 minutes. Fixed with a retirement queue: an address comes
back only after **600 s**, three firings of the 180 s clock, so what must fit is one 600-second
window (~800/s) rather than a whole night. Exhaustion is now loud in the heartbeat
(`FELL BACK - the class is NO LONGER COVERED`), because a fallen-back run looks exactly like a
protected one. `research/guard-page-2026-09-08.md` §7. 99 grap-stub tests, 2049 workspace.

**The one run worth a launch when MEASURING is `-SentryWriteWatch`** (`research/naming-the-writer-2026-09-07.md`,
`crates/grap-stub/src/writewatch.rs`). Around each *predicted* firing it puts bucket 1's pages
read-only for 1.2 s, so the damaging store faults at its own instruction and the log names the
RIP. It writes nothing to the client. **It is also the first instrument that can answer "is any
of this ours"** - the faulting RIP names a module, and 75 of 75 archived runs carried our hook,
so the archive never could. What is established today: our hook has no 180-second cadence and
none of its writes has the family's shape; what is *not* established is whether our environment
puts the client on a path it would not otherwise take.

The **other** lethal free now has a guard too: `-FreeGuard`
(`crates/grap-stub/src/freeguard.rs`) refuses a pool chunk handed to the NT heap by PCOM's WZ
property teardown, which is how the field client died on a map change. It replaces one cached
function pointer at `PCOM+0xdbb80` - the IAT is **not** the call site, so an IAT hook would
have installed cleanly and intercepted nothing. Off by default, and off during a write-watch
run.

**2026-09-07: the second- and third-job skills are audited - 149 of them - and most now do
something correct on the server.** `research/second-third-job-audit-2026-09-07.md` has one
row per skill. What landed, all unit-tested and **none of it yet on a screen**: every cast now
pays what the client's own table prices (MP, the Boosters' HP, Magic/Summoning Rocks, Shadow
Meso's mesos) whether or not a stat is granted - Teleport used to keep its MP; 33 more buffs
are granted on bits from the 408-name table (`crate::advbuffs`, `net::jobbuffs`) with their
server halves where the number is the server's - Hyper Body's HP ceiling, Power Guard's
reflection, Meso Guard's mesos, Invincible's cut, Holy Symbol's EXP, Element Amplification's
MP, Dragon Blood's drain, Combo's orbs, Soul Arrow's free shots, Bless's heal bonus; arrows and
stars come from each skill's own row (Strafe 3, Arrow Rain 8, Avenger 4, the hidden hits none);
Heal, Drain and MP Eater move HP/MP; the third-job Improved MP Recovery is flat MP per tick.
**A latent bug fell out**: a toggle (Magic Guard) expired on the next loop pass, because its
expiry was `now + 0`. **Not built, and said so in the table**: summons and Puppet, Mystic Door,
every mob-side status (slow, seal, stun, freeze, DoT - no mob-stat packet is decoded),
Pickpocket and Meso Explosion, Meso Saver, Chakra, Critical/Nimble Recovery, Final Attack's
HP absorb, Steal's theft. The test plan's T19 says what each screen outcome means.

**2026-09-06: Thief and Warrior audit - built, unit-tested, NOT yet on a screen.** Details in
`research/thief-warrior-audit-2026-09-06.md`; the plan is T16 / 0f-0h in `tools/test-server.ps1`.

* **Stars** leave the Use tab per throw with a claw (1; Lucky Seven 2, **[I]**), the same path
  as arrows. **One caveat that covers yesterday's arrows too: no `0x00E0` shoot body has ever
  been captured**, and the shared parser was decoded from melee. A shot that fails to parse
  now logs `SHOOT body did not parse` instead of taking nothing silently - look for that line
  first if a throw takes nothing.
* **Recharge** works at every Grocer: the star row carries `info/unitPrice` (measured: Subi 0.3
  ... Hwabi 1.0, now column six of `itemdata.txt`) in the eight bytes the client reads as the
  recharge double. Cost is `ceil(missing x unitPrice)`; ceil-vs-truncate is the client's and a
  run decides it.
* **Slash Blast costs HP** (3..8) as well as MP - `hpCon` had been in the table since 08-28 and
  unread. Floors at 1.
* **Party buffs** - Haste and Rage - reach every party member on the caster's map via
  `Event::PartyBuff`; each recipient builds and expires its own `0x007D`. Bits: Jump **93 [D]**,
  Weapon Attack **84 [D]** (name table + identical decoder blocks to Speed's); the buff levels
  come from the generated table's `indie*` columns. **Iron Will is self-only in this client's
  data** (no `lt`/`rb`). (`!buff` was removed later the same day on the owner's instruction, so
  the bits are seen through a character that has the skill.)
* **Max HP Increase: the CLIENT applies it, measured.** The owner's screenshot: Cobalt `358/447`
  with the database at `358/358` and Max HP Increase 15 - `447 = 358 + ⌊358×25/100⌋`, and the
  server was calling them full. Experiment A answered. The server now sends the base and raises
  every ceiling it enforces to the same expression (`session::pools`: regen, potions,
  Recovery, level-up refill, `!heal`, quest set-HP, `!resetap`, the party bar). A potion
  drunk above the base used to CUT health to the base. Max MP Increase is the same code, `[D]`.
* **GM commands pruned and two made public** (the owner, 2026-09-06): gone are the per-kind rate
  setters, `!migsweep`, `!npcfx`, `!buff`, `!unbuff`, `!buy`, `!locker`, `!kit` (and
  `crates/world/src/loadout.rs` with it). `!rates` and `!help` answer for everyone; `!help`
  shows a player only `!rates` and `!help`. `!job`, `!resetap`, `!resetsp` reply in one line;
  the working moved to the log. No "A skill has been activated." line on a skill-up.
* **`!setrates` takes five fields**: `<exp> <meso> <drop> <quest> <party%>`. Quest multiplies
  turn-in EXP. Party% is the **copy** each other member on the field receives (killer keeps
  70%); 30 is the old behaviour, 0 is allowed, and it never appears on the banner. `!rates`
  lists all five.
* **`!rates` and a client death, UNSETTLED.** 01:53:24 UTC: the client died 8 ms after the
  `!rates` reply, 51 min into the session, inside a red-black-tree walk on a garbage node
  pointer (`0xffffffff2f822a41`) reached from the chat-notice printer - and that reply was the
  first chat notice in 30 minutes. The one earlier `!rates` in the archive (08-21, same-shaped
  text) drew fine. Reads as the long-session corruption with `!rates` as the first messenger;
  **`!rates` at ~40 s of client life is the discriminator** and is in the plan. Dump:
  `dumps\maplecw-crash-179092-c0000005-1.dmp`.
* **The heap corruption: eight damaged slots in an idle session, and the value is not always
  1.** The same dump, walked: 8 damaged `0x20` slots in 3069 s - **three read
  `0x0000000200000020`**, the "single different value" `heap-wild-write.md` §10 said would
  change the reading. `1`, `2` and the map node's `-1` are all 32-bit stores at `+4`: a
  counter, not an initialiser. The session was idle - 34 k mob acks, 1 317 chatter lines, 21
  banners, 0 hits - so the writer runs on idle traffic, which `heap-corruption-2026-08-27.md`
  §7 asked about. **The sentry then ran for six minutes the same evening and caught it twice**
  (`research/heap-corruption-2026-09-06.md` §5): finding #2's slot was freed 720 ms later and
  that free was the `0xC0000374` death - the chain watched live for the first time. One slot
  was live, one free, same value: the writer holds its own pointer. The offset is `(body+4) - 8`
  - a **refcount written through a BSTR-convention data pointer as if it had an 8-byte
  cookie**, `[D]`. The sentry cannot name the instruction (every thread was asleep by the time
  a 100 ms walk finds the slot); §3.2's guard-page allocator can, and that is the next build.

**2026-09-05: the server is ready to leave this machine.** Two changes for the homelab move,
both built, tested, and exercised against the shipped binaries:

* **No hardcoded address anywhere.** Both servers bind `0.0.0.0` on an installed box and
  decide *per connection* which host to write into the migration packet - the address the
  client reached us on for a LAN, VPN or Tailscale peer; the box's own public address
  (discovered from an echo service at startup, re-checked every ten minutes) for an internet
  peer. `--advertise auto|list|<ip>`, one rule in `crates/net/src/advertise.rs`. The installed
  `start-server.ps1` carries bare ports and no IP at all. `tools/package-server.ps1` builds
  the ~9 MB server payload; `SERVER-README.txt` in it is current.
* **Sign-in is TLS 1.3 to a pinned certificate.** `maplecw-auth` makes its own certificate on
  first start and prints its fingerprint; every launcher pins exactly that value
  (`crates/tlspin`, the one definition both ends use) and **refuses to send a password when it
  has no pin**. `tiny_http` is gone - its TLS feature pinned end-of-life rustls 0.20 with
  CVE-2024-32650 unfixed - and five endpoints are served by a bounded hand-rolled loop over
  rustls 0.23. Verified three ways: unit and end-to-end tests, Python's OpenSSL against the
  shipped `maplecw-auth.exe` (TLS 1.3, fingerprint matched, plain HTTP gets an alert), and
  `tools/claims_smoke.py` end to end. The owner's first suggestion - send the argon2 hash instead
  of the password - was not done: it is pass-the-hash and makes a database leak an instant
  login for every account.
* **Found on the way:** through the real HTTP path a **wrong password had never read as a
  wrong password** in the launcher. `http::parse` refused every 401 before reading the body,
  and the unit test's fixture said `200 OK`, so it passed. The first end-to-end test that sent
  a wrong password to a live service caught it. Fixed; the plan's "wrong-password refusal
  CONFIRMED" dates from the pre-HTTP launcher and was true then.

* **Players get accounts through single-use codes, minted in game.** The owner, later the same
  day: `!registrationcode` and `!recoverycode <email|username>` mint an 8-character code
  (`XXXX-XXXX`, the confusion-free alphabet `store::codes` already had) as a chat notice on
  the GM's screen and **nowhere else** - `world.log` records "minted (not logged)". The
  launcher grew a **Register** tab (username, email, password, code) and a **Forgot
  password** tab (email or username, code, new password); the service grew `/register` and
  `/recover` (`auth::register`). Passwords a player chooses must be **8+ characters with a
  letter and a digit** (`store::PASSWORD_POLICY`, one owner, checked on the launcher first
  and on the server always). Everything checkable is checked *before* the code is spent, and
  a recovery code is consumed only against the account it was minted for, so a typo never
  burns one. Eight characters is ~39 bits, down from ~78 - so the service now budgets failed
  code attempts per peer and overall (`auth::ratelimit`; 10 and 200 per fifteen minutes),
  which is what the codes module always said shortening would require. `maplecw-useradd
  --registration-code / --recovery-code / --codes` do the same from the console. T12 in the
  test plan is the screen half; the suite covers every sentence in it.

* **Login is enforced.** The owner, after the default run opened a client that was simply served
  `maplecw`: *"I want to remove this functionality and enforce login."* The login server's
  fallback account is gone by default: a connection it cannot attribute to a launcher sign-in
  (client token in `0x0073`, owning process, or address) is answered with a **login failure**
  - code 5, the client's own `notRegisteredID` notice - and sees nobody's characters. The
  connection stays open so a `0x0073` token can still attribute it. `--fallback-account NAME`
  restores the old behaviour by name (the two smoke scripts pass it; the banner shouts when it
  is on). `test-server.ps1`'s default is the launcher path; the direct client is
  `-DirectClient -FallbackAccount <name>` and the script refuses to start one without the
  fallback, because a client that cannot pass the login screen measures nothing. Found while
  doing it: a server test named "a later claim replaces an earlier one" was passing because
  the later account was also the fallback - two live claims were always ambiguous.

* **The channel holds the first entry to the sign-in that minted it.** The owner: *"is there a
  way to enforce that the initial character enter has to be from an authenticated session from
  our launcher? Migration between channels are fine as long as the initial assumption is
  true."* Two facts a channel connection can be held to without the client carrying anything
  new: on this machine, the OS says which process owns the channel socket and the launcher
  registered that process to a sign-in (`attest_channel_connection`, which existed); off box,
  the channel connection must come from the **same address** as the login connection (recorded
  on every migration; `PeerPolicy::Require` existed and was never on because `::1` vs
  `127.0.0.1` false-refused - `same_peer` normalises that). So `--bind-migrations auto` (login,
  default) token-binds a migration when the login connection was attributed by process and
  address-binds it otherwise; `--migration-peer-policy require` (world, default) refuses a
  claim from another address; channel→channel mints record the address too. **Left open, and
  stated:** an attacker behind the victim's own NAT racing the real client inside the
  60-second migration window with a guessed character id. Closing that needs the hook to carry
  the token on the channel connection - client work, the follow-up. Not yet seen on a real
  client: whether the on-box attestation holds for the real client's channel socket
  (`claims_smoke` proves it for the login socket; the world banner self-tests the lookup).
  `-BindMigrations never` is the escape hatch if a character stops entering the world.

* **After the first enforced two-client run** (both clients entered the world - the on-box
  binding held on a real channel socket): **chat now reaches the other client** - `say_out_loud`
  was a local echo written before the bus and never revisited - and the party invite is
  answered with what the client understands: the leader gets `0x1B` outcome 0 *"You have
  invited '%s'"* ([L]; it was `UNKNOWN_ERROR` because the body was undecoded) and the target
  is handed the `0x03` that opens the invite dialog (shape [L], fields 3-6 [I]).

* **The evening run of 2026-09-05: one Invite killed both clients.** The owner: *"The act of
  inviting someone to party crashed both clients."* Two mistakes, one launch, both settled from
  the logs and the listing with no second launch:
  1. **`0x13` went out as the joiner's name and nothing else.** The client reads a six-seat
     `PARTYBLOCK` after the name (`research/party-result-0x00A5.md` §5.2 had said so since
     09-04; the builder said `str`). Each client read a party id out of bytes that were not
     there, ran off the body, threw three C++ exceptions with identical stacks, **reported the
     packet back in `0x009E CLIENT_PACKET_REJECTED`** - body `a5 00 13 07 00 "Tester2"`, the
     exact bytes - and closed its own socket. `net::party::PartyBlock` now follows the name;
     the size identity and the first real capture are tests.
  2. **The first `0x0183` ever decoded was not a click.** It arrived **1 ms** after the `0x03`
     with answer 0, and *"anything but 1 is an accept"* turned it into a join nobody had
     agreed to - which is what sent the fatal `0x13`. `tools/listing.py` over the `0x03` arm
     and the two dialog callbacks: **the handler answers 0..=3 itself, immediately** (0 = the
     dialog is opening; 1 blocking, 2 busy, 3 already invited - no dialog), and the buttons
     send **4 = Decline, 5 = Accept** later. Every value but 5 is the `0x1B` outcome the leader
     is shown for it; 5 is the one that table leaves silent. [L] for the paths and constants,
     [D] for which button is which. `net::party::invite_answer`; the world relays 1-4 to the
     leader as their own sentence and does nothing at all for 0.
  Fixture: `research/fixtures/party-join-0x13-rejected-by-client-0x009E-both-clients-exit-*`.

* **The party works on a screen, and six requests after it are built.** The owner's next run:
  the join drew both members with the right job and level (*Cobalt Magician 18*, *Tester2
  Beginner 8*), and Accept is answer 5 as decoded. They then listed what was still wrong; all
  server-side, all fixed and unit-tested (1975 pass), none yet on a screen:
  - **Leave / expel / disband now answer** with `0x10` and the party block - the Leave
    transition was answered with `UNKNOWN_ERROR` before, so *"Tester2 cannot leave"* left the
    member stuck. Both the leaver and the remaining members are told.
  - **Pick-up rights (the button that failed with "unknown error")** is routed, stored on the
    party, and answered with a `0x0D` window refresh. This client has **no standalone
    "rights changed" packet** - the string is only shown as a side effect of the
    public/private arm - so the value is stored (drop visibility is by membership) and the
    error is gone.
  - **Party EXP** - killer keeps 70% (white), the other members on the field split 30% equally
    (yellow); the free-for-all damage-share is suppressed while in a party. No AFK signal
    exists, so "on the field and online" is the eligibility test, and there is no distinct
    "party EXP" string, so the line is the ordinary one in yellow.
  - **Kill-quest credit is shared** to every party member on the field who has that quest in
    progress and needs that mob - each advances their own row.
  - **Party drops are shown to every member on the field**, owned by the killer; a member who
    later leaves is off the live roster the pick-up resolves and loses access, while the killer
    keeps it (*"unless they were the killer"*).
  - **A player's ground drop is public** - the `0x046E` is broadcast to the whole map and
    anyone on it may pick it up, an untradeable item excepted.
  **One needs a client measurement and was NOT guessed** (a guessed body has killed this
  client three times): the **meso drop** request, which appears in no capture. One run: the owner
  dropping mesos, with the inbound opcode read off `world.log`.

* **Party member HP is `0x02B2`, found by walking back from the gauge** (2026-09-06). The owner:
  *"Party member HP should've been broadcasted to party members on the same map when the
  party is formed ... you need to do some researching in the code."* The earlier note said no
  packet carried it; that was a search of the wrong tables. Working backwards - the HUD's draw
  reads a remote member's gauge as `[user+0x10cc]*64/100`, that field's only writer is a
  `SetRemoteHP(hp, max)` setter, its only caller is a remote-user handler reading `u32 hp,
  u32 maxHp`, and that handler is reached through a **third, compacted switch** in the
  remote-user router that the 39-slot table hid - pinned the opcode and the twelve-byte body
  with every link [L]. `net::userpool::user_hp_remote`; `Session::party_hp_tick` sends it to
  every party member on the same field whenever `(hp, max, who is here)` changes, one tick
  after. Two-session test passes. **Unseen on a screen**; the first run says whether the bar
  fills. Full chain and two decoys: `research/party-result-0x00A5.md` §12.
  **Unseen still:** the invite dialog itself, and every one of the seven above on a screen. T14.

* **Archer audit, 2026-09-06.** The owner: *"regular attacks or skills using bows/crossbows should
  consume arrows from the use tab depending on the attack amount ... double shot should
  consume 2 arrows, and power knockback should knock back the mob considerably."* Findings:
  - **Arrows were never taken.** `firstjob.rs` had carried the rule since 08-28 - Arrow Blow
    `bulletConsume 1`, Double Shot `2`, Power Knockback none - and `on_attack` never read it:
    built, not wired. Now wired: a normal shot 1, Arrow Blow 1, Double Shot 2, Power Knockback
    0, bows from `2060xxx`, crossbows from `2061xxx`, lowest matching Use-tab stack first, the
    new count sent as a `0x0070`, a shortfall logged and the swing **never refused**. The
    attack header has no bullet-slot field (all 33 read, most constant across 434 captures),
    so the server picks the stack. Six tests over the real captured swing.
  - **Power Knockback's push is the client's, not the server's.** `research/mob-hit-reaction.md`:
    the flinch and knockback are produced locally by the client that both swings and holds the
    mob, and no packet the server sends produces them. The distance is the skill's own
    `range` column - **130 px at level 1 to 150 at 15**, the tooltip's *"knockback N enemies
    by 130"* ([L]) - against a normal hit's short shove. What the server owes is control, and
    it hands it to whoever hits, so on a mob the other client drove the first hit transfers and
    the second pushes. MP (12 → 8) is already spent. Nothing to build; two things to watch.
  - **The rest of the book:** Critical Shot and The Eye of Amazon are client-side passives
    (the critical flag arrives inbound); **Focus is acknowledged and NOT granted** - its two
    stat bits (Accuracy, Avoidability) are unmeasured, the same gap Iron Body has.
  - **Not covered:** second-job archer skills have no cast handlers at all (Arrow Bomb's
    `noBulletConsume 1`, Soul Arrow's free shots, Strafe's `bulletCount 3` are read but
    unused). `research/archer-audit-2026-09-06.md`. T15.

* **The launcher remembers the game folder.** The owner, 2026-09-05: *"does our launcher save
  whatever the user set it to upon subsequent starts? Setting it every time is going to be
  very frustrating for users."* It did not - Browse and a typed path changed the running
  launcher only. Now a chosen folder that holds `MapleStory.exe` is written to
  `maplecw-launcher.remembered.toml` **beside the executable** (not `%LOCALAPPDATA%`: the
  launcher runs elevated, and under an over-the-shoulder UAC prompt that profile is the
  administrator's, not the player's) and read on the next start **ahead of the config file**,
  since a Browse is a person correcting the installer's guess. One key, the config's own
  literal reader, `crate::remembered`. A stale remembered folder is kept and flagged, not
  silently swapped for the default; the startup log and `--print-paths` name the source of
  the folder either way. Unit-tested at the resolver (remembered > config > layout, stale
  kept, empty file reported) and the file round trip; **not yet seen through the window** -
  the next launcher restart after a Browse is the check.

What did **not** change: the game socket still carries no credential. What changed about the
exposure: forwarded to the internet, a stranger reaching 8484 is no longer served as anyone
by default - only a connection attributed to a live sign-in is served, and attribution by
address is the weakest of the three rules. `SERVER-README.txt` says so where the ports are
listed. Ports, all TCP: 8480 (sign-in, TLS), 8484 (login), 8485 and up (one per channel).

### What works — seen on a screen

| | |
|---|---|
| **two clients, one map** | 2026-09-03. Avatars, movement, attacks, HP bars, and a late joiner told where people **are** rather than where they entered |
| **remote damage and the hurt flinch** | 2026-09-04. *"I now see the flinch damage when the non-primary player is taking damage."* |
| **the party window** | 2026-09-04. Create from the client's own window: *"You have created a new party"* and a list row with the right name, job and level |
| the single-player world | dressed character, NPCs and dialogue, portals, combat both directions, drops, pick-up, EXP, levelling, AP/SP, death and revive, buffs, quests, shops, storage, channel change |
| the Cash Shop | opens on the same socket (`0x01A3`, no migrate); the currency is **Leaf Points**, `!lp` funds it, a purchase completes |
| character select | list, create, name check, three-slot limit, delete, persistence across relaunches |

### What is built and has **never** been on a screen

CLAUDE.md's "built is not wired" category, one step further on: these are wired *and* unseen,
which on screen is indistinguishable from absent.

| | |
|---|---|
| **second job advancement**, end to end | the four hidden test fields, their mobs, the 30 marbles, the examiner and the warden. `research/second-job.md`, `research/second-job-fields.md` |
| **third job advancement** | ten jobs, four instructors on map 20001001 (the Ossyria ferry that reached them was removed 2026-09-29). `research/third-job.md` |
| **the departure handover** | measured *on the wire* (below), never watched on a screen |
| **the release-first control rotation** | landed 2026-09-04 20:37, after the last run of the day. `research/control-release-does-it-despawn.md` |
| remote `move_action` / facing, foothold, seat index | all landed 2026-09-04 after the last run |
| **EXP shares to other players** | wired 2026-08-29 and **never executed on a wire either**: 329 kill-EXP lines in the archive, zero carrying a damage fraction. Nobody has ever killed a mob together |
| return scrolls, `!npcreload`, Phil routing Beginners to their instructor | wired since 2026-08-29 |
| **arrows leave the Use tab** | 2026-09-06. A normal shot 1, Arrow Blow 1, Double Shot 2, Power Knockback 0; bows take `2060xxx`, crossbows `2061xxx`; never refuses. Six tests over real captured swings; not yet watched |

### What is blocked, and on what

| | |
|---|---|
| **the heap corruption** | `0xC0000374`, **17 distinct fault events** across ~16 archived runs (4 with dumps), deduplicated on `(timestamp, code, address)`. The damaged word is the identical `0x0000000100000020` every time. **The writer is still not found.** This is what ends a long session - the 2026-09-03 two-client death was at **371 s** of client life - and it is the one thing standing between "two players can play" and "two players can play for an hour". `research/heap-corruption-2026-08-27.md`. Do **not** pass `-HeapFix`: it armed, it held, the client died anyway, and every dump taken with it on is unusable for the free-list argument. *(Only 3 of the 17 sit in a log carrying an `ARMING` line, so client-age-at-death is measurable for 3; "never under ~192 s" is the 2026-09-03 figure and is `UNVERIFIED 2026-09-04` here - one of the three is a 7.9 s instance-guard experiment.)* |
| **party invite → a party anyone can join** | not blocked, unseen. The `0x1B` outcome, the `0x03` dialog and the `0x13` join-with-block are all decoded [L] and on the wire; the one wire test of `0x13` (2026-09-05 evening) went out **without** the block and killed both clients, and the fix has been in front of a test but not a client. What is left is a screen: the dialog, Accept refreshing both windows, Decline reading as "denied". `research/party-result-0x00A5.md` §10 |
| ~~**trade and chat rooms**~~ | **RETRACTED 2026-09-09 - see below. The trade invite is built and the popup goes out.** The row used to say the client *"declines locally and sends no packet at all"* |
| party member HP bars | **built, 2026-09-06, unseen.** `0x02B2 {u32 charId, u32 hp, u32 maxHp}`, found by walking back from the gauge to its field to its writer to its handler to its dispatch - the remote router's third, compacted switch, which the 39-slot table hid. Sent to party members on the same field on formation, arrival and every HP change. `research/party-result-0x00A5.md` §12 |
| ~~dropping mesos~~ | **BUILT 2026-09-09, unseen** - `Session::on_drop_money` in `session/ground.rs` deducts, rests the coins on a foothold and puts them on the floor for anyone to take. A negative amount is refused, which matters because `0x0143` is a signed `i32` the client's own check lets through: as a `u32` it is four billion mesos, and as a negative delta it would *pay* the dropper. The row below is what it used to say |
| ~~dropping mesos (old)~~ | **NOT blocked, and NOT implemented - the server refuses on purpose.** The opcode row above was stale: this was the second of the two absence claims flagged on 2026-09-09, and it is now retracted. `0x0143` is known, the body is 8 bytes, `net::dropmoney::parse_drop_money` decodes it and a test decodes **two captured drops** as ten mesos. What `world::mesodrop::on_drop_money` does is *refuse*: it answers the exclusive-request unlock and a chat line, `"Mesos cannot be dropped on this server."` That fix was about the **freeze** - a meso drop used to leave `+0x2330` latched and kill every later inventory action - and it was never about making the drop work. The owner, 2026-09-09: *"I still cannot drop mesos"*, which is exactly what this code does. The pieces to finish it are present: `Field::drop_item` places a player's bag drop and `net::drops::FieldDrop::money` is the money object mobs already use |
| `0x0183` accept/decline | **settled from the listing, 2026-09-05 evening**, not blocked. The answer byte is the `0x1B` outcome numbering: the `0x03` handler itself sends 0 (dialog opening), 1 (blocking), 2 (busy) or 3 (already invited) before any click, and the buttons send 4 Decline / 5 Accept. The slot order is [L] from the first capture. `net::party::invite_answer`. Unwatched on a screen: a click of each button, and a faded dialog followed by a re-invite (invites lapse server-side after 60 s) |
| second-job skill casts | none of the 66 has a cast handler. `firstjob.rs` is the shape it wants |
| the keyboard layout | **DONE on both halves as of 2026-09-12, restore unverified on screen.** `0x0199` in, `0x05F1` out; the save stores one row per key (`character_keymap`). The restore needed the client's factory table because `0x0199` is a delta; it is measured now, off the image: `0x143274460` is a **read-only const table** in `.rdata` - three 89-slot presets, stride `0x1bd` - and preset 0 (Q/W/E/I on menus, LCtrl attack, Space jump, 41 bound) is `net::keymap::CLIENT_DEFAULT_LAYOUT`. `tools/keymapdump.py --exe` re-derives it with the controls. After every SetField the READ gate and all 89 slots go out. Plan step 6 says what each screen outcome means |

### What to do next, in order

**2026-09-09: the owner named the next five, and they are FOUR features, not five.** *"Trade
requests / Hair and face coupons / Scrolling items (and clean slates and white scrolls, which
do not have items in the game but we'll need to mimic the behavior of) / Summoning Sacks /
Player games (Omok and Card Match)."*

**Trade, Omok and Match Cards are one subsystem.** All three are `CMiniRoom` - inbound
`0x017E`, outbound `0x0575` - and the room-open envelope is already decoded in
`research/trade-2026-09-09.md` §3: mode 4 with `A == 0` creates the window by `B`, where
`B == 1` is trade (pinned three independent ways) and `B == 3` / `B == 4` are the only other
two player rooms. The member list is decoded generically. **What is missing is one virtual
(`[vt+0x1C8]`) per room type** - three small decodes that hand over all three features. That
is the highest-leverage work on the list by a distance, and none of it needs a client run.

Where each of the four stands before any of this session's decode work lands:

| | ready | missing |
|---|---|---|
| **miniroom** (trade, Omok, Match Cards) | the envelope, the member list, the invite popup - all [L] | the per-room-type virtual, ×3 |
| **scrolling** | `gm-handbook/scrolls.txt`, **213 rows** with success, cursed and every stat increment, already dumped | the opcode that applies a scroll to an equip. Clean Slate and White Scroll have no items here, so those are server-side policy on the same path |
| **summoning sacks** | mob spawn, drops, damage, death and EXP all live and confirmed | the item→mob table, and whether the client sends anything but `0x010E` |
| **hair/face coupons** | hair and face are already persisted character fields; `0x0138` outbound is `UserAvatarModified` | the coupon item list, the valid hair/face values per gender, and `0x0138`'s body |

**2026-09-09, also from the owner: the Maple Island quest chain does not work.** *"I don't seem to
be able to accept Mai's Final Training quest. Neither can I do the quest to build the Relaxer,
or the quest that has me deliver letters between Rina and Lucas. Rain also seems to be missing
their quiz quest. Pio's Relaxer quest also requires the crates to spawn in and around Amherst,
but those are not spawning."*

**None of this is missing content - every one of these quests is in the client's own data.**
Amherst is map **1010** and has exactly three NPCs [L]: **Lucas 17, Pio 18, Rain 19**. The
quests, by the NPC they name [L]:

| quest | name | NPC |
|---|---|---|
| **1007 / 1008** | Letter for Lucas / Lucas's Reply | Lucas (17) |
| **1010** | Mai's Final Training | Rain (19) |
| **1012** | Pio's Collecting Recycled Goods - hands over `3010000`, The Green Relaxer | Pio (18) |
| **1013**..**1019** | Rain's Maple Quiz 1-7 | Rain (19) |

*(Rina is **not** on Maple Island - they are template 201 on map 10001000, Henesys. The Amherst
letter pair is 1007/1008. Worth confirming which NPC the owner means before anything is written.)*

**Two different causes, and they need different work.**

1. **No script offers any of them.** `data/npc-dialogue.txt` has **zero** non-comment rows [L]
   - it is an empty overlay. The machinery to accept a quest exists and is used:
   `Session::accept_quest`, `store::start_quest`, `net::script::QUEST_ACTION_START`, and
   `SCRIPT_TYPE_QUEST_YES_NO` which `jobguide.rs` already sends. So this is the **same gap
   Shanks had** - the NPC answered *"no template for NPC 15"* until a script was written for
   them - and the fix is the same shape. **[D]** that this is the cause; nobody has watched one
   of these NPCs being clicked with the log open, which is the one thing that would settle it.

2. **Pio's quest is blocked on a subsystem that does not exist.** The crates around Amherst are
   **reactors**, and there is **no reactor code in `crates/` and no reactor dump in
   `gm-handbook/`** [L] - the word appears once in the whole tree, in an unrelated comment in
   `drops.rs`. So 1012 cannot work until reactors exist: the data dump, the spawn, the hit, the
   break, and the drop. That is a feature, not a fix, and it is bigger than the other four
   combined.

**And 1010 gates something already shipped.** `world::shanks::MAIS_FINAL_TRAINING` is 1010, and
the free ride to Lith Harbor is waived on it. So the free branch is currently **unreachable** -
which is also why nobody has ever confirmed it on a screen, and why the test plan lists that one
branch as unattested. Fixing 1010 and confirming Shanks' waiver are the same job.

Everything below this banner predates that and is the older ordering.

1. **Watch a departure handover and a control rotation on a screen.** Both landed after the
   last run and both are one two-client launch. The free measurement that comes with it:
   count inbound `0x02FF` per connection for one object id - after a release there must be
   **exactly one sender**. Two senders is the teleporting the owner saw on 2026-09-04.
2. **Kill one mob with both clients**, which has **never been done**. `share_reason` writes
   *"N/M of the damage"* onto a non-majority cut; the archive holds **329** `exp from a kill`
   lines and **zero** with a fraction in them, so the EXP-sharing path - built 2026-08-29,
   the reason `broadcast.rs` exists at all - has never executed on a wire. Same launch as (1).
3. **Watch the party invite land, this time with the block.** The evening run's Invite
   killed both clients (a `0x13` with no `PARTYBLOCK`, sent because the client's "dialog
   opening" acknowledgement was taken for an accept); both are fixed and unit-tested against
   the client's reader, and neither fix has been in front of a client. Unseen: the dialog,
   Accept drawing both members in both windows, Decline reading as "denied", and a faded
   dialog followed by a re-invite. **A client dying on Accept now means the block is wrong** -
   stop and keep the logs. T14, same launch as (1) and (2).
4. **`0x0184` and `0x0194`**, which arrive unanswered at field entry, twice each, once per
   client - `FUN_142defbc0` and `FUN_142defc50`, adjacent builders 0x90 apart, and `0x0184`
   sits between the two party opcodes this server *does* answer. If either is *"tell me my
   party state on arrival"*, a client that has a party would not learn about it on entering
   a field, which looks exactly like the party system not working from a different direction.
   Not investigated and not guessed at.
5. **The heap writer.** Everything else is bounded by it.
6. **The third and second job advancements**, which are two walks and have never been tried.
   Orbis loads (below); El Nath and the ferry menu do not have an observation between them.

**The test plan is NOT here.** It is in `tools/test-server.ps1`, in **two** places - the
`.NOTES` block and the `Write-Host` dialogue the launcher prints - and both must be kept
current. **As of 2026-09-04 that file is stale and its two copies disagree**: the printed
block says T0 is answered, and the `.NOTES` block at line 68 still says *"STILL UNKNOWN:
whether this client will run twice on one machine at all"* four lines under its own "T0 is
ANSWERED". The whole plan predates the two-client run, the party window and the flinch work.
Rewriting it is the first job of whoever launches next, and it is not this file's to own.

### LANDED 2026-09-01 — two players share the mobs, and a departure hands them on

`crates/world/src/mobshare.rs` has production callers. Exactly one connection controls each
mob; a wounding hit is republished as the attacker's own `0x03F0` bytes; a mob's position is
believed only from its controller; drops go to the top damager, uncapped over-damage excluded.
**All 1839 workspace tests pass.** `research/mob-share.md` is the working.

#### The departure hook was silent, and silence was the bug

The owner: *"If the person who is controlling the movement of the mob leaves the map, then the mob
should not disappear. That's a jarring experience. The control of the mob should be handed over
to another client who is still present in the map."*

They were describing a **worse symptom than the one that was there**, and was right about the fix.
The mobs never disappeared - a release frees the claim and leaves the mob alive on the field -
but nobody was told, so every monster on that map **stood perfectly still** on the remaining
screens with no error and no log line.

The design's own cost note said *"one tick of latency ... nobody has watched it"*, and it was
not one tick: nothing in `tick` claims orphans. `claim_uncontrolled` runs on **field entry**,
which a player standing still never performs, so the freeze lasted until somebody walked
through a portal. **A cost written down as an estimate is a claim** - and this one was not out
by a factor, it was the wrong quantity, because nothing was scheduled to pay it.

The test that was supposed to cover this called `on_field_entered` a second time to make the
grants appear, which is precisely what a standing player does not do. It is now collected from
`tick`, the idle path, with no field entry after the departure.

| | |
|---|---|
| `Controllers::hand_over` | re-assigns under **one lock** - no instant with two owners and none with zero |
| `Bus::successor_on` | somebody else present on that map; lowest id, so a test can name it |
| `Bus::publish_to_subscriber` | addresses a chosen mailbox and **returns whether it landed** |
| `Session::hand_over_mobs` / `hand_over_all_mobs` | picks the heir, sends one `0x03D2` per mob **at its current position** |

Wired at **all six** exits: `go_to_map`, `on_change_channel`, `on_log_out`, the Cash Shop, and
`Drop for Session` - the last being a killed client or a dead socket, and the one that matters
most, because the person who left is the one person who cannot see the result. With nobody left
it degrades to the old release, so the next arrival still claims everything.

Proved by injection: restoring the release-only behaviour fails three of the four new tests,
and the fourth is the nobody-left case the injection is identical to.

> **CORRECTED 2026-09-04.** This entry ended *"Not observed on a screen. Two clients on one
> machine is still blocked on the `grap-stub` instance work, so every claim here rests on the
> suite."* Both halves are now out of date: two clients run (2026-09-03), and the departure
> handover has been **measured on the wire** three times. See the section above this one.

### LANDED 2026-09-02 → 09-04 — two clients, and everything that broke on the way

Three days, and the single sentence for it is: **the multiplayer half stopped being a design
and started being a thing on a screen.** Ordered by what a reader needs first.

#### Two clients run on one machine, and the guard was two guards

T0 had been open for weeks and the answer is yes. `crates/grap-stub/src/instance.rs`:

| gate | how it is beaten |
|---|---|
| `FindWindowA("MapleStoryClass")` | hooked in `user32`, and the class name is matched before anything else happens |
| `Global\WvsClientMtx` | `kernel32!CreateMutexW`/`A` are **six-byte `ff 25` forwarders**; the fix rewrites the **pointer they read**, not the code they reach |

**Patching `kernelbase` hung the client** - three launches of six died that way, and the hang
came from the *presence* of the patch, not from the logic: 77 `CreateMutex` calls run through
`DirectSound`'s `DllMain` and a detour that logged from inside one deadlocked the loader. So
the slot rewrite is eight bytes of data, leaves everything that calls `kernelbase` directly
untouched, and the detour matches the mutex name before it does anything at all.

#### `0x0224` UserEnterField: four wrong zeros in one packet

Every one of these killed both clients or drew them wrong, and every one of them was a field
where **`0` is a real value that means something** - which is why no length check and no
"is it set" test could catch any of them.

| what | was | is | how it read |
|---|---|---|---|
| `REMOTE_STAT_TAIL_LEN` | 7 | **23** | the body was short; both clients died inside the handler |
| body 416, the seat index | `0` | **`-1`** | `0` is a valid seat; the accessor **reports** out-of-range and then honours it, returning `0 + 48*0 + 8` = address 8 |
| `RemoteAt.foothold` | `0` | `Footholds::landing` | `0` means *not standing on a foothold*, and the client drew everyone **floating** |
| `RemoteAt.move_action` | `0` | `Option<u8>`, default **4** | `move_action` is `(action << 1) \| facing`, so `0` is *"facing right, doing nothing"* - a pose this client has emitted **zero** times in 28 134 archived elements |

Two instrument lessons out of that run of failures, both already in `CLAUDE.md` and both paid
for again here:

* **An offset table is meaningless without the base it is indexed against.** The seat fix went
  out at body offset 41 first and changed nothing - same instruction, same register state, same
  14-frame unwind. The static pass had the right field and the right sentinel and read its
  answer out of a `param_1`-based table while measuring against an `r15`-based one, and
  `r15 = param_1 + 0x100`. What finally settled it was a **new** instrument: take the remote
  `CUser` of the same character from the run before the change and the run after, keep only the
  dwords that went `0 -> 0xFFFFFFFF`, and exactly one offset comes back - `0x100` below where
  the table said to look. That instrument did not exist until the wrong fix had shipped.
* **Body offset 41 was called "fame"** because the v214 reference tree calls it that. It is a
  per-map array index. The row was tagged `[I]` honestly and was still read back as a fact.
  `CLAUDE.md` scores that tree 1 of 8; it is a candidate generator.

The chain is closed: `0x140f9295e` last appears in the archive on **2026-09-03 22:23**, and
the four runs after the seat fix carry **zero `CLIENT FAULT` lines**. Those runs are short
(2-5 minutes), so that is "the decode no longer kills it", not "the client is stable".

#### RETRACTED 2026-09-04: `CONTROL_RELEASE` does **not** despawn

This is the most expensive retraction in the file, because the wrong version was **[L]** and
it shaped the whole mob-sharing design.

*What it said:* `0x03D2` at level 0 *"DESPAWNS the mob ... takes the zero branch **straight**
into the pool's erase path"*, therefore control can never rotate while its holder is present,
therefore this server must never send it.

*Why it was wrong:* **"straight" was the whole error.** Two guards sit in front of that erase
and a live mob stops at the second - `FUN_141c543c0` reads an in-field flag whose complete
writer set is the constructor (0), `0x03C6` MobEnterField (**1**, both branches) and `0x03D1`
leave (0), so any mob that entered normally returns 1 and branches to the epilogue. The
original working in `research/mob-behaviour.md` §3 **described both bails**; every copy after
it dropped them, and the copy being quoted stated it flattest. It was restated in **nine
places** and derived in one.

*And it had never been tested.* 505 archived logs, 240 788 events deduplicated on
`(timestamp, direction, opcode, body)`: **2 664** `0x03D2` at level 1 and **zero** at level 0.
`mob_release_controller` had no call site in `crates/`.

The owner of the fact is now `crates/net/src/mobmove.rs::CONTROL_RELEASE`, the working is
`research/control-release-does-it-despawn.md`, and `STATUS.md` deliberately does not restate
it - which is the point of the pass that retired the other eight copies.

**There is one real despawn**, and it is almost certainly where the reading came from: a mob
known only from a 137-byte `0x03D2` never has the in-field flag set, so level 0 erases it.
True for the one spawn path this server never uses.

#### The flinch is not a packet, and the order of two packets is the fix

The owner: *"If the client that does not have mob control attacks a mob, the mob does not flinch and
get pushed back."* **There is no `MobDamaged` opcode in this client** - both mob dispatch
tables enumerated, 19 + 117 slots, 98 distinct handlers, six opcodes can set a mob action and
none carries a damage value. The flinch is produced **locally** by the client that both swings
and holds the mob's `0x03D2`, and reaches other screens as that client's own `0x02FF`.

The experiment was already sitting in `previous-runs/`, same build, same map, same two
characters, differing only in who walked in first: **controller swings, 10 wounding hits, 10
hit-action reports; non-controller swings, 15 wounding hits, 0.** Both copied into
`research/fixtures/` under names that say what they prove.

So the fix is ownership - and shipping it **without a release made mobs teleport**, because two
clients then simulate one mob. The order is not an optimisation:

```text
1. -> the old holder   0x03D2 level 0   release      <- FIRST
2. -> the attacker     0x03D2 level 1   grant
3. -> the old holder   0x03D9 relays, which it already got
```

**The first hit of a fight still will not flinch**, inherently: the grant leaves with the reply
to the swing that earned it. Mobs here take about three hits. `research/mob-hit-reaction.md`.

#### The departure handover is real, and it is measured on the wire

The 2026-09-01 entry above ended *"not observed on a screen"*. It is still not on a screen, but
it is no longer only a suite result. Three archived runs carry the line, and the heir really
drove the mobs afterwards:

```text
02:51:00.913   mob control: 30 mob(s) on map 40 handed from connection 1 to connection 3,
                            30 grant(s) delivered
02:51:00.988 .. 02:51:42.042   1 170 inbound 0x02FF over 41 s   <- the heir moving them
```

`previous-runs/world-20260903-225142.log` (also in `fixtures/` under two other names - the same
run, not three observations).

#### Party: the window draws, and three things were learned by it failing

**Create works from the client's own window.** Two bugs stood between the state machine and
that, and both are the shapes this file keeps recording:

* **The opcode went out twice.** `party_created` built with `PacketWriter::with_opcode(...)`
  and `Reply::packet()` prepends the opcode again, so the client's first read returned `0xA5`,
  fell to the default arm and drew *"Due to an unknown error, your party request failed."*
  **It survived because `UNKNOWN_ERROR` is itself a default-arm code**, so every refusal path
  had the identical defect and produced exactly the message it intended. Only a real arm could
  expose it. All four tests were green on a packet the client could not read - `CLAUDE.md`'s
  *a test that pins what the code already does is not a check*, and they now assert
  `Reply::packet()`, which is what reaches the framer.
* **The level was in the wrong slot.** `member+0x18` is the job-name lookup's second argument
  rather than a field, so level belongs at `+0x1c`. Predicted, deliberately not acted on, and
  the screen settled it: "Magician", level 0. **[D] -> [L]**.

`MAX_MEMBERS = 6` is **[L]** now on two legs (the member array is `party+8`, span 0x420, stride
0xB0; and the invite path refuses at `cmp eax,6 / jl`). **An empty seat is a zero dword, not a
zeroed 155-byte record** - writing the long form shifts every field after it, the same class as
the seat index. Invite carries a **name** the client does not resolve; expel and change-leader
carry an **id** it already resolved against its own member list.

Drops are party-scoped from the moment a party forms. There is deliberately **no `!party` GM
command**: the owner asked for it to be removed, on the grounds that a back door that works makes a
broken feature look finished.

#### Remote damage: an echo would have drawn nothing

`0x02A5` USER_HIT_REMOTE. `crates/net` said of it *"there is no builder because there is nothing
to build: the 147 bytes are the client's, and the server's job is to pass them on."* Wrong in
the direction that produces no error and no complaint:

* the handler draws the damage from HITINFO **+0xa8**, body offset 143, and gates the 1500 ms
  flinch on that same field being `> 0`;
* the client sends `0` there in **331 of 331** event-deduplicated captured bodies, and its own
  builder has no write to that slot at all.

So it is a **server-fill** field, and an echo calls the damage renderer with 0 - the MISS path.
`user_hit_remote` copies the client's 147 bytes and replaces four of them; the test asserts the
dword at 143 is the server's number and **every byte either side is byte-identical**. It is the
`applied` damage, not the client's claim, and **positive** - the client negates it itself, and a
negative selects digit set 3, the blue recovery number. Confirmed on screen 2026-09-04.

#### A late joiner was told where people entered, not where they are

`Presence.spawn` was built once at field entry and the bus handed that frozen body to every
later arrival, so an existing player appeared wherever they stood when *they* entered - the map
origin, before their first step. `Bus::refresh_spawn` was written for exactly this and **had
zero callers**. Setting the position and refreshing the bus are now one function, because two
call sites is how the refresh gets forgotten.

### LANDED 2026-08-31b — the third advancement, and the continent it is on

**This client has a third job advancement, and it is half-shipped.** The ten jobs, their skill
books and their four instructors are all here and complete. The *test* is not — and that is an
enumeration, not a failed search:

| | second job | third job |
|---|---|---|
| quest chain | 16 quests, `20000`..`20303` **[L]** | **none.** All 322 enumerated; nothing above `20303` |
| hidden test field | four, one portal each **[L]** | **none.** Every one-portal map in the archive enumerated |
| dedicated test mobs | eight, `800010`..`800017` **[L]** | **none** |
| test items | letter, 30 marbles, proof **[L]** | **none** |
| level gate in the data | `Check.0.lvmin = 30`, sixteen times **[L]** | **none.** 70 is **[I]** and ours |

So the owner's call was **no test — level 70 and click**, taken knowing the client offers no
alternative. `research/third-job.md` is the working.

**The four instructors are 1104 Tylus, 1105 Robeira, 1106 Rene and 1107 Arec**, all standing
in **Chief's Residence, map 20001001**, each placed exactly once. They are identified by their
own idle lines rather than by a remembered roster — which is worth saying because a
from-memory roster gets two of them wrong. There is no Helena in this client, and **"Chief
Stan" is NPC 202 in Henesys**, a father in a gold-watch quest.

**There is no choice at third job.** `111` is the book under `110` in `Skill.wz`, ten times
over, so the advancement is a statement rather than a menu.

#### The real blocker was travel, and it is now solved

**El Nath was unreachable.** Victoria Island is a 223-map portal component; Orbis and El Nath
are a separate 87-map one, and the link in the real game is a ship rather than a portal. **[L]**

The client ships a full ferry cast — and **four of the seven stand in rooms nothing can walk
into**. Ellinia Station has *zero* portals targeting it in the whole archive, and Ellinia's own
`in03` has no target map. The Orbis Ticketing Booth is different and was checked separately: it
is reachable from Orbis.

So the line runs on **Eurek the Alchemist**, who is the only NPC this client places on **both
continents** — Sleepywood and El Nath — whose one and only `d0` is *"I wander all over the
world of MapleStory"*, and who carries **zero** quest rows. **[L]** on all three.

```text
  any Victoria town  --cab, 500--> Sleepywood
  Sleepywood         --Eurek, 1000--> El Nath      (or Orbis)
  El Nath            --portal in01--> Chief's Residence
  and home again by the same route
```

| what | state |
|---|---|
| `thirdjob.rs` — ten jobs, four instructors, level 70, tier 3 | wired, **never on a screen** |
| ~~The Ossyria ferry line~~ | **removed 2026-09-29** at the owner's word - El Nath is on foot or by scroll |
| `skillpoints::Tier::Third` and the tier-3 SP pool in `0x007C` | wired, **never on a screen** |
| The three invisible third-job skills | already filtered — `secondjob::HIDDEN_SKILLS` holds all 13 |
| **87 Orbis/El Nath maps** | all have field images and footholds. **CORRECTED 2026-09-04 — one of them HAS been loaded**, see below |

**A behaviour change worth naming: the second SP tier now stops accruing at 70**, exactly as
the first stops at 30, because a tier with a successor should hand over to it. Nobody loses a
point they already had — `top_up` saturates and never claws back — but a level-71 second-job
character who does not advance stops earning second-job points. A test caught this change when
I made it, which is the whole reason it is stated here rather than discovered later.

#### CORRECTED 2026-09-04: this client loads an Ossyria map, and the proof was three days old

*What this section said:* *"Can this client load an Ossyria map at all? **87 maps, zero
observations.** If a client dies there, that is worth more than the advancement it was on the
way to."*

*Why it was wrong:* **Orbis is map `20000000` and it was loaded on 2026-08-28**, three days
before the sentence was written, in `previous-runs/world-20260828-142900.log`:

```text
18:25:28.774  -> 0x01A0 SetField ... carrying map 20000000 for character 213 (Cobalt)
18:25:29.300  <- 0x00DC CLIENT_FIELD_ENTERED         the client accepted it
18:25:29.301  -> 0x044F NpcEnterField  x 9           and drew the town
18:26:01.416  -> 0x01A0 SetField, GM !map 10001010   33 s later, on purpose
```

Cobalt's **stored map** was already Orbis, so the login put them there on connect - nobody had
to build a ferry to see it. Nine NPCs drew, 22 idle-chatter packets went out, the client
answered normally and **no `CLIENT FAULT` line appears in that window**. They stood still for the
33 seconds and then `!map`'d out, so this is *"the field loads and the client survives it"*, not
*"Orbis is playable"*.

Two things this cost, and both are `CLAUDE.md` rules with a new instance:

* **"Zero observations" was a property of the search.** Nobody ever asked *"has map 20000000
  ever appeared in a SetField"* - the specific-question control that the cash shop's `0x00D5`
  taught this project and that the second-job hidden fields taught it again a week later. One
  grep, free, and decisive on the day.
* **The archive double-counts.** `carrying map 20000000` appears three times on disk and is
  **one event**: the other two are `research/fixtures/` copies of that same run under
  `iron-body-no-reduction-...` and `learn-max-hp-increase-...`, named for what their author was
  looking at rather than for everything the file contains.

**Still open, and the ferry run is still worth making:** whether an El Nath map loads (Orbis is
one map of 87, and the instructors are on `20001001`), whether a character can *play* there
rather than stand for 33 seconds, and **whether the ferry menu lists two stops and not six** -
six would mean the network filter broke and every Victoria cab is selling 500-meso rides to
another continent.

---

### LANDED 2026-08-31 — the second advancement, end to end

**The four "hidden fields" the second-job chain needs are in this client, and they always
were.** `research/second-job.md` recorded that *"the Test of Qualification hidden field and
its `q20002s` script do not exist, so the client's own route to the advancement is not
walkable"*. **The script half is right and still holds** — all 205 archives and all 10 021
images were enumerated and `q20002s` occurs twice in the whole tree, both times as a *name*.
**The field half was wrong**, and it was wrong in a shape this project has paid for before:
an absence established for one thing (`Script.wz`) was carried across to a neighbouring thing
(`Map.wz`) without being re-asked. One `grep` of `gm-handbook/maps.txt` would have broken it.

| branch | test field | mobs (dedicated templates) | warden | ejects to |
|---|---:|---|---:|---:|
| Warrior | **80001300** Warrior's Rocky Mountain | 800016 Fire Boar, 800017 Lupin | 800006 | 10004023 |
| Magician | **80001100** Magician's Tree Dungeon | 800012 Curse Eye, 800013 Horny Mushroom | 800004 | 10002070 |
| Bowman | **80001000** Ant Tunnel For Bowman | 800010 Evil Eye, 800011 Zombie Mushroom | 800003 | 10001090 |
| Thief | **80001200** Thief's Construction Site | 800014 Cold Eye, 800015 Blue Mushroom | 800005 | 10003080 |

**[L]** throughout, and the branch-to-dungeon pairing is corroborated by **five nodes across
two archives** — `info/returnMap`, `life` type `n`, `life` type `m`, `String.wz/Map.img`, and
the `Quest.wz` chain. `research/second-job-fields.md` is the working.

**Each of those maps has exactly one portal — the spawn point — against 31 for Perion.**
There is no way in and no way out on foot, which is the client saying the entry is
server-side. The consequence is the thing to watch on the next run: **a character in one of
them with no working warden click is stuck**, not inconvenienced.

| what | state |
|---|---|
| `secondjob.rs` — the decision, 1400 lines | **was BUILT AND UNWIRED since 08-28. Now wired** |
| The four test fields, their mobs and their exits | data + tests, **never on a screen** |
| Examiner warps in / takes 30 marbles / gives the proof | wired, **never on a screen** |
| Warden warps out — the only door | wired, **never on a screen** |
| Instructor offers a type-6 menu of 2–3 second jobs | wired, **never on a screen** |
| The advancement itself: `0x007C` job + SP tier 2, proof consumed | wired, **never on a screen** |
| Dark Marble drops, gated on **mob AND map** | wired, **never on a screen** |
| `startscript q20002s` and its three siblings | authored in `data/quest-scripts.txt` |
| Type-6 menu renders from the server | **MEASURED — see below. No longer an open question** |

**The gate that is enforced is the proof item, not the quest chain.** `REQUIRE_QUEST_CHAIN`
is still `false`; `REQUIRE_PROOF_ITEM` is `true`. Quest `20003`'s `Check.1.item.0` is the
client's own rule, it is a fact about the bag rather than about quest rows, and `!item` can
put one there for a test run without pretending a quest happened.

**The count is 30, not 20.** The owner's brief said 20 marbles; `Check.1.item.0.count` is `30` on
all four branches and `questreq.txt` says the same from a second pass. Worth naming because
the number is the whole length of the test.

#### The old T3 is answered: **a server-sent type-6 menu renders and its lines are clickable**

`research/fixtures/type6-menu-renders-and-taxi-rides-world.log`, 2026-08-29: Lyn and the
Regular Cab each sent one type-6 box, each was answered by a 10-byte `0x00F3` ending `06 01`
with a real selection — **line 2** then **line 0** — and each was followed by the fare and the
`SetField`. Two NPCs, two different lines picked, one session. The section below still
describes it as the open question of that run; it is kept for the instrument lesson it
carries, and **its conclusion is superseded here**. The second-job choice box therefore rests
on a measurement rather than a gamble, and **Phil could now be collapsed from four yes/no
boxes to one menu** — not done, and named rather than left implicit.

#### Still open in this area — two of the four are CLOSED, checked 2026-09-04

* ~~**Skill points are not persisted when spent.**~~ **CLOSED.** `crates/store/src/skillpoints.rs`
  persists spend per character *per tier* - `skill_points_spent`, `skill_points_available`,
  `spend_skill_points` - so the pool survives a relaunch and the tier-3 pool inherits it.
* **`skilltable::book()` will offer the thirteen `invisible = 1` skills**, and the scope is
  narrower than this said: `book()`'s only non-test caller is **`gm_learn`**, so `!learn` is the
  one path that can hand out an undrawable skill. Nothing player-facing goes through it.
  `secondjob::HIDDEN_SKILLS` is asserted equal to the client's `invisible = 1` set by a test.
* **No second-job skill has a cast handler.** Nothing equivalent to `firstjob.rs` exists for
  the 66. Still true - `crates/world/` has `firstjob.rs` and `magic.rs` and no sibling.
* ~~**The MP-recovery passive still does nothing.**~~ **CLOSED**, and on both halves, which is
  the part worth keeping: `2000000` carries **two** bonuses - `y`, the item-recovery percentage,
  now read in `session/consume.rs` through `itemrecovery::restored`; and `x`, *"regenerates 1%
  of Max MP every 10 seconds"*, now in `session/regen.rs`. Implementing one and calling the
  skill done is exactly the failure `CLAUDE.md`'s quest-payout section describes.
  `research/item-recovery.md`, `research/mp-regen.md`.

---

### LANDED 2026-08-29b — what is wired, what is OFF, what is unobserved

> **SUPERSEDED 2026-08-31 — it draws.** Two boxes, two selections, one archived session;
> see the 08-31 section above. Everything from here to the end of this subsection is kept for
> the instrument lesson, which still stands. Do not act on its open question.

**The one measurement the [2026-08-29b] run was for:** does a **server-sent message type 6**
draw its `#L<n>#` lines? The client uses that box for its own NPC menus (`research/npc-click.md:196`,
`FUN_142a61900(ui, 6, ...)`, **[L]**), and the client's own WZ carries 33 such menus — but no
server has ever sent one. So **the taxis send type 6 and Phil sends a chain of yes/no boxes,
which is proven on screen.** That is deliberate: Phil is the control beside the experiment.
If Phil works and Lyn does not, the fault is type 6 and nothing else.

That design exists because of an instrument failure worth remembering. An enumeration of all
71 entries of the `0x055B` jump table concluded **no select-one-of-N box exists**. It was wrong
twice: it searched for a *count-then-N-strings* body (type 6 reads **one** string with markup),
and its write-extractor **walked each function linearly and stopped at the first `send`** —
type 6's *cancel* path is laid out before its selection path, so the tool reported the six-byte
cancel and concluded there was no index. **A linear write-scan reports whichever branch the
compiler emitted first and says nothing about the rest.** The answer had been sitting in
`research/npc-click.md` the whole time; neither pass grepped it. The coordinator then relayed
that negative to another agent as settled, which caused a **correct** type-6 implementation to
be withdrawn. Do not propagate a negative you have not checked.

| what | state — the "never on a screen" column re-checked 2026-09-04 |
|---|---|
| EXP shares reach other players | wired, **never on a wire either**, and now measured: **329** `exp from a kill` lines across the whole archive and **zero** carrying a damage fraction. Two clients have fought on one map and have never killed the same mob. `share_reason` appends *"N/M of the damage"* to a non-majority cut, so the search has a positive control and it found nothing |
| Taxi rides, 8 NPCs, 500 mesos, Lyn as tour guide | ~~never on a screen~~ **CONFIRMED 2026-08-29** — Lyn's type-6 menu drew, a line was picked, 500 mesos were taken and the `SetField` landed in Kerning City |
| Phil routes Beginners to their instructor | wired, **never on a screen**, sends yes/no. Still true - "Phil" appears in no archived world log |
| Return scrolls, 10 of them, same-continent rule | wired, **never on a screen** |
| `!npcreload` — NPC dialogue swaps under a live connection | wired, smoke-tested, **never on a screen** |
| Per-launch login claims | **fixed and proved over real sockets** |
| Migration credential binding | **BUILT AND OFF.** Still off: `login::config` defaults `bind_migrations: false`. See below |
| NPC and mob names in the dumps | done |

**`--bind-migrations` is OFF by default and until it is on, the migration hole is not fixed.**
A bound migration is refused by a channel server presenting nothing, which is today's channel
server, so switching it on unconditionally would refuse **every** character select and read as
a total outage. This is the "Built is not wired" category and it is named here rather than
listed as done. The seed cannot be the credential: measured 2026-08-29, 115 distinct `0x007D`
hello bodies against all 74 seeds ever minted, plain, both endiannesses, and the XOR form the
decompiler predicts — **8510 trials, zero hits**, character id at offset 8 passing as a
positive control on all 115.

**What IS fixed, and it was the worse bug:** `stake_login_claim` used to `DELETE FROM
login_claims` and insert one global row, so the second person to sign in **evicted** the first
and *both* connections were served as the second account. Reproduced live over sockets
(`otter saw ['OwlTwo']; owl saw ['OwlTwo']`), then closed: one claim per launch, and a
connection is matched to its launch by asking the OS which process owns the socket
(`store::peerowner`, `GetExtendedTcpTable`). The owner's constraint — *"IP cannot be the sole
discriminator"* — is satisfied by construction, and `tools/claims_smoke.py` is the acceptance
test: two accounts on `127.0.0.1`, each served its own characters, and an unattributable
connection **refused rather than guessed**.

**Still true, and said out loud as always: nothing authenticates the game socket.** The
per-launch claim decides which account a credential-less connection is served as, using a fact
the OS supplies rather than one the client asserts. That is strictly better and it is not
authentication. Known residue: an elevated launch may not return `hProcess` through the UAC
consent UI (unmeasured, costs one launch); a recycled pid inside a 12-hour claim is unguarded;
two *remote* clients behind one address are `Ambiguous` — neither impersonated, neither able to
play.

**The next cheap step on that:** `MigrationEvidence::with_token_hash(hash)` would let the world
server resolve pid → claim → `token_hash` and satisfy a bound migration **without the hook**,
for same-machine clients. Assessed, not built. It must never be constructible from anything a
connection sends.

> **SUPERSEDED 2026-09-03 — it is on a screen.** Two clients, one map, seeing each other move
> and attack. The heading below said *"WIRED AND HAS NEVER BEEN ON A SCREEN"*, which was true
> for five days. The delivery mechanism it describes is unchanged and still the right reading;
> the **body** it describes is not - `0x0224` was 508 bytes here and is 531 + name + equips
> now, and four of its fields were wrong. See "two clients, and everything that broke on the
> way" at the top of this file.

**MULTIPLAYER IS WIRED — 2026-08-29, first on a screen 2026-09-03.** The owner:
*"the client's own movement is completely disregarded ... their movements and their
attacks need to be broadcasted and shown on all clients."* The first half of that was
literally true and worse than it sounded: **no session could say anything to another
session at all.** A channel is a process, each connection is a thread with its own
socket, and `Session` is a pure state machine, so `session/combat.rs` had been computing
a second contributor's EXP share and dropping it for nine days. `research/exp-sharing.md`
said so out loud: *"only the delivery is missing"*.

`crates/world/src/broadcast.rs` is the delivery — a mailbox per connection, hung off
`Fields` so it reaches every session through an `Arc` that was already threaded there.
A publisher never touches another socket (the send cipher is a *stream* cipher, so two
threads framing onto one socket would corrupt everything after the collision,
intermittently); it appends, and the owning thread drains in `Session::handle` **and**
`Session::tick`. Both, because a player standing still sends no packets and `tick` is
the only thing waking that connection. `TICK_MS` 500 → 100 for the same reason.

Three packets go out, all decoded **statically** this session:

| packet | body | where |
|---|---|---|
| `0x0224` UserEnterField | 65 fields, **508 bytes** + name + 5·equips | `research/user-enter-field.md` |
| `0x0225` UserLeaveField | one `u32` | same, §1 |
| `0x0293` UserMoveRemote | `u32 charId` + the path **verbatim** | `research/user-pool-tables.md` |

Three things are worth not relearning:

* **`avatar_look()` is reused unchanged, at body offset 187.** `1429ce6a9 call
  0x1402ee8d0` is the same compact-look reader the character-select screen already
  accepts these exact bytes through.
* **`0x0293` must NOT carry the key-state trailer.** `1429d2eb5 XOR R8D,R8D`, exactly
  like outbound `MOB_MOVE` against `0x02FF`. Checking the *encoder's* call sites gives
  the wrong answer — `0x00D9`'s builder also passes zero, and 1082 measured bodies
  carry the trailer anyway. The encoder writes it unconditionally; only the decoder
  chooses to read it. `net::usermove::UserMove::path` is the right span,
  `path_with_key_states` is not.
* **The enter/leave assignment is [L] now**, from the bodies — `0x0224` allocates a
  `0x4438`-byte `CUser` and inserts it, `0x0225` unlinks and destroys — not [D] from
  enum order, which is all there was before.

**Known wrong, and it self-heals: a just-arrived character is announced at the map
origin.** The server's only source of position is the client's own `0x00D9` reports, and
`gm-handbook/portals.txt` carries no coordinates. It snaps on that player's first step.
The real fix is teaching `tools/dump_portals.py` to emit portal x/y. **STILL TRUE for the
arriving player**; the *other* half of it - a late joiner seeing everyone else at the origin -
was `Bus::refresh_spawn` having zero callers, and is fixed (2026-09-03).

> **ANSWERED 2026-09-03.** This paragraph said *"Nothing in `0x224..0x39F` has ever been
> observed doing anything in any archived run, so the first question on the next launch is
> whether the client accepts `0x0224` at all."* It does. It was accepted and dispatched with
> `ret=1` on 2026-09-03 once `REMOTE_STAT_TAIL_LEN` went 7 -> 23, and the run after the seat
> index went `0 -> -1` drew both avatars. The watch on `0x1429ba60b` was never needed - the
> discriminator that worked was the **two-log count**: `world.log` says what we sent,
> `client-patched\maplecw-hook.log` writes its dispatch line **on return**, so a missing line
> means the handler was entered and never came back.

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
and the instrument mistakes that have each produced a clean, confident, wrong answer here.

**There are TEN packet-read primitives, not seven.** That count has been wrong three
times - five, then seven, then eight, then nine - and every correction came from
enumerating rather than searching a neighbourhood. `tools/reads.py` carries the list and
the history. A tail `jmp` into one of them **is** a read; missing one shipped a short
packet that killed the client twice.

**The attack packet DOES carry the skill id — retracted 2026-08-28.** For nine days
`world::magic`, `research/damage-formula.md` §9.1 and `research/mob-combat.md` §7 all said the
server cannot tell which skill was cast, and every damage validator in the repo was left
unwired because of it. It is the **`u32` at body offset 2**, with the **level as the `u8` at
offset 6** — `research/attack-skill-id.md`. The evidence for the old claim was an absence in
captures that could not have contained the thing: every archived body was an ordinary swing,
where the field is legitimately `0`, and a zero field explains nothing about itself. One grep
over `previous-runs/` settled it: **426** swings at `0`, **one** Three Snails at `1000` level
**3** (its maxLevel is 3) and **seven** Magic Claws at `2001003` level **7** (the owner had put in 7).
*(Those counts are the corrected ones — the first pass said 689/2/14, because a glob over
`previous-runs/` **and** `research/fixtures/` counts a capture once per name it has: 155 world
logs on disk are 119 distinct files. Deduplicate by content hash before counting anything in
those two directories.)*
Same shape as the cash-shop opcode that sat in the log for three sessions while being reported
absent: **nobody asked the specific question.**

**"Trade is a feature that does not exist" — RETRACTED 2026-09-09, and the citation was the
problem.** The blocked-work table said the client *"declines locally and sends no packet at
all"*, and it named its method, which is what made it convincing: *"checked the documented
way, by grepping `research/msexe-send-opcodes.txt` for the builder rather than eyeballing the
tail."* That is the method `CLAUDE.md` prescribes, and it was cited **against the file that
contains the answer**. `0x017E` appears there **38 times**, and one of those rows is
`FUN_141826bc0` — the exact builder the decode identified nine days later, sitting in the
enumeration the whole time. The owner then saw the packet arrive twice in one capture, 8 ms apart.

**Why a correct instrument returned nothing: that file has no feature names in it.**
`miniroom|trade|chat|party|chair`, case-insensitive, matches **zero** lines. It is opcode →
function address and nothing else, so a grep of it phrased as a *name* comes back empty for
every feature that exists, including all the ones that demonstrably do. The search had to be
phrased as an opcode, and the opcode was what was being looked for.

So this is `CLAUDE.md`'s oldest rule with a new twist worth writing down: it is not enough to
use the right instrument, because **an instrument can be incapable of answering the question
in the form you asked it, and still answer.** Empty is a result shape, not a verdict. Ask a
file what fields it has before concluding from its silence.

**Two rows in that same table are absence claims of the same shape and have not been
re-checked.** *"dropping mesos - the client's meso-drop request appears in no capture"* and
*"second-job skill casts - none of the 66 has a cast handler"*. Neither is retracted here;
both are flagged, because the trade row also looked settled and cited a method.

> **The meso row was checked the same day and it was wrong too - three for three.** The
> opcode is `0x0143`, it is decoded in `net::dropmoney`, and a test decodes **two captured
> drops**. The claim that it "appears in no capture" was false when it was written and the
> capture was already in the tree. Flagging it took one grep; nobody had run it. The row above
> now says what the code actually does, which is refuse on purpose.
>
> That is the whole lesson twice over: **the flag is worth nothing without the grep.** I wrote
> "these two have not been re-checked" and then did not re-check them, which is the same
> failure as citing a method instead of running it - and it survived a further day only
> because the owner hit it on a screen. `second-job skill casts` is STILL unchecked.

**The keyboard layout is not saved because nothing has ever tried to save it — 2026-08-28.**
The owner: *"Upon logout then subsequent login, this customization is completely gone."* This is
not a broken feature; it is an absent one, on **both** halves. `keymap|funckey|quickslot`
case-insensitive over the whole repo matches three prose files and **zero lines in any of the
seven crates**, and none of the 14 tables stores a mapping. The client cannot cover for us:
its local settings block — enumerated straight out of the image — holds sound, graphics, chat
and UI options and **no key mapping**, so the mapping can only come from the server. Neither
opcode is known yet, and the honest limit of the search so far is written down in
`research/keymap-not-saved.md`: no keymap-shaped packet in **162 distinct captures** (and
nobody has yet changed a key *during* one, which is the whole caveat), and no key-table
decoder among the **179 of 273** channel-stage cases that have an out-of-line handler. One
step on any future run settles which half to build first, and it is in that file.

**Guilds are the Shop2 shape: every entry point ships and the window does not — 2026-08-28.**
The client has guild classes, ~20 guild GM commands, guild chat and invite toggles,
`button:guild` and `button:GuildCastle` on the status bar, `BtGuild` in the context menu, a
`vector:guildName` slot in the character-info window, and a **fourth UserList tab whose
bitmap reads "Guild"** — rendered, not guessed. `UserList.img` ships exactly three panels:
`Buddy`, `Party`, `Blacklist`. Across **782 images in 17 archives** the only guild-named
image is `Etc/guildCommon.img`, and it holds one number. So the guild window is in the same
position as the Buy Back tab that killed the client, and **half the feature was never on the
game socket at all** — `CNMGuildChatMessageEvent`, `CNMGetMyGuildListExFunc` and `WzMss.dll`'s
SOAP endpoints own guild chat, the member list and the mark image. Server side: **zero guild
code, and that is the right amount for now.** `research/guilds.md`.

**MORE THAN ONE ACCOUNT WORKS NOW — 2026-08-28, and it is unconfirmed on a client.** The
login server used to resolve `--account` **once, at startup**, so one process could only ever
be one player. It resolves **per connection** now: `maplecw-launcher` verifies a password
(argon2id), stakes a *login claim* in the shared database, and `login::server::resolve_account`
reads it on every accept, falling back to `--account` when none is live. Sign in as someone
else and press Start Game — no restart. `tools/test-server.ps1 -Launcher` drives it.

Three things that are easy to get wrong and are pinned by tests: **the claim is not consumed
on read** (the client opens a second login connection after Log Out — a single-use claim would
turn "log out" into "my characters vanished"); a **stale claim falls back rather than refusing**,
because an unanswered connection freezes the client's whole UI; and every connection **logs
which account it chose and why**. The sign-in field takes an account name *or* an email —
`accounts.email` is a new nullable unique column, and the two namespaces cannot collide
because `validate_name` allows only `[A-Za-z0-9_]`.

**It is still not authentication, and the wording matters.** The claim decides *which* account
a credential-less connection is served as. The game socket carries no credentials, `0x0073` has
been measured carrying none, and anything that reaches the login port is served as whatever the
claim names. The launcher authenticates a *person*, not a *connection*. `docs/launcher.md`.

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

## NEXT GOALS - set by the owner, 2026-08-19 onward

Login and world entry are done. Each goal carries what is already established, so nobody
re-derives it, and the **one concrete next step**.

**Where the lettered goals stand, updated 2026-09-04.** Each heading below now carries its own
verdict, so this paragraph is a summary rather than the record:

| | |
|---|---|
| **done and on a screen** | A quests, B NPC chatter, C drops, D level up, E first job advancement (packet *and* NPC conversation), F NPC shops, G storage, I bag persistence, K HP/MP per level |
| **decoded, not enforced** | J the damage formula - physical and magic both read, `check_hit` and `max_plausible_hit` still have no caller |
| **open** | H citizenship (BUILT 2026-09-28, unseen - START HERE), L attack-speed timing (lowest priority, and the source page does not carry the table) |

The live work is not covered by any letter: it is the multiplayer chain at the top of this file.

Two corrections to what used to stand here. E said *"has its packet and a `!job` command but not
its NPC conversation"* - untrue since 2026-08-31. And **Mina's classic shop counter was called
"a separate thing and still unbuilt"**: `net::classicshop` and `session/shop.rs` build it,
`0x055D` has gone out **9 times** in the archive, and `tools/test-server.ps1`'s own "what
previous runs closed" block records that the classic shop draws and that selling works. It is
not in this file's CONFIRMED table, so the screen fact is second-hand - `UNVERIFIED 2026-09-04`
as to what it looks like, but "unbuilt" is definitely wrong.

### The 2026-08-24 → 08-27 changelog (was "START HERE", superseded)

> **This is no longer the front door.** It used to say *"read this section and nothing else to
> know where the project is"*, and it was last updated on 2026-08-27 - before the second and
> third job advancements, before mob sharing, before two clients ran, before the party window.
> **The current state is the `START HERE` section at the top of this file.** Everything from
> here down is kept **for its working, not its verdicts**: the log is reverse-chronological and
> a claim in it may have been retracted further up.

**What changed over 2026-08-24 → 08-27, newest first:**

* **`-HeapFix` was tried, it armed, and it cannot work. RETRACTION of what this file said two
  entries ago.** It said the patch *"would have prevented the 2026-08-27 death"* because the
  fault address sat in the function it edits. The patch **held** - `0x14019b504` reads
  `8b 07 90` in the new dump - and the client died anyway. There is a **second pooled free** at
  `0x14019bb50` with the identical qword header load, the route to it is fixed at **compile
  time**, and a client died there **unpatched on 2026-08-20**, two days before the patch
  existed. Across every archived run: **10 deaths, 8 at one site, 2 at the other.** Fifty-six
  sites carry that ladder image-wide. The fault address said which function *died*, not which
  function the block would have been *freed through* - and one check surviving is not a
  measurement.
* **And the flag costs a measurement every run it is on.** The only constraint anyone has on
  *when* the stray `1` is written is that a damaged slot was found **on the free list**, which
  is an argument only while that free is unpatched. Every dump taken with `-HeapFix` on is
  unusable for it. The flag is **off**.
* **The damage is the same shape for the tenth time** - `0x0000000100000020`, `0x20` class,
  0 damaged in 693 272 slots outside it.
* **Magic Claw's 1 damage was the formula being right.** INT 6 on a Rogue-turned-Magician:
  `MagicTotal` seeds from `floor(INT/2)`, so the window sits at 1..2 before any defence.
  `magic::cobalt` pins it. `!job` and the skill-up now warn when the stat does not match.
* **`!resetap` and `!resetsp`.** The AP one **conserves the total** rather than recomputing a
  per-level award nothing here knows, and leaves a stat already below the floor alone.

* **The magic damage formula is decoded**, `FUN_14025FFD0` - the sibling of the physical one,
  found because the two are called from the same four sites with byte-identical setup on the
  two arms of one `if`, and because it reads the one stat slot the physical builder ignores.
  All **[L]**. Two things worth knowing: **magic has ONE uniform roll where physical has two**,
  and `MagicTotal` is seeded with `floor(INT/2)`.
* **Magic Guard is CTS bit 97 [L], and it is the SERVER's job, not a stat bit.** The bit was
  found by reading the client's own hit handler - it multiplies `secStat+0x614` by the damage,
  divides by 100 and clamps to MP - rather than from a name table. But the client **never
  writes HP**: only `0x007C` moves either bar. So setting the bit buys an icon and nothing
  else; `Session::on_user_hit` has to do the split and send HP **and** MP together, where it
  sends `hp_only` today.
* **`wdef_from_strength` and `mdef_from_intelligence` are [L] now**, not `[I]` - both are
  literal seeds in the same totals builder. And a **limitation found in our own code**: the
  critical rate and multiplier are **per-character percent fields** in this client, not the
  globals `damage.rs` assumes, so its window is right for one character only.
* **The Magician ids are not the classic tree**, confirmed independently by two agents: the
  classic buff ids `2001002`/`2001003` are the two **attacks** here, and `2001004`/`2001005`
  do not exist at all. Anything hard-coding the familiar numbers would buff two attack skills.

* **A Magician can now put a point in Magic Claw.** The blocker was never damage - it was that
  `session/skills.rs` refused every id outside the three beginner skills and clamped everything
  to level **3**. Its own comment said why: *"a refusal to invent a rule, since what a job may
  learn is `Skill.wz` data nobody has read."* `tools/dump_skills.py` reads it now - 176 skills,
  4 164 skill-levels - and `world::skilltable` gates on the **book** and on each skill's **own**
  ceiling. An empty table degrades to the old behaviour, deliberately.
* **The Magician first job is six skills**, from `200.img`, and not the list from another
  version: Improved MP Recovery and Max MP Increase (passive, 15), **Magic Guard (15, a TOGGLE
  with no duration at any level)**, Magic Armor (20, timed, seconds not ms), Energy Bolt (20),
  and **Magic Claw (20, `attackCount` 2 - its damage number is PER HIT)**. `mastery` in this
  build is a **level 1..10**, not the percentage it is elsewhere.
* **There are no skill damage formulas on this server, correct or otherwise.** `on_attack`
  applies `target.total_damage()` - the client's own number - so every skill "works" and none
  is checked. `damage.rs` has `check_hit` and `max_plausible_hit` and **neither has a caller**;
  and the whole file is physical, with `research/damage-formula.md` recording at line 231 that
  *"magic damage is a different path entirely and is not in this function."*

* **`-HeapFix` would have prevented the 2026-08-27 death, and both crashed runs were
  UNPATCHED.** `0x14019b58e` - the fault address the hook recorded - is the **return address**
  of `call rbx` = `HeapFree(heap, 0, ptr-8)` at `+0xac` of the same 288-byte function the patch
  edits at `+0x24`. That branch is reached only when the 64-bit header exceeds `0x80`; read as
  a **dword**, which is all the patch changes, `0x0000000100000020` becomes `0x20`, selects
  bucket 1, and the block goes to the pool free list instead. The session marker in both hook
  logs reads `mode=2,create=on` with no `heapfix=on`, so **nothing has tested it yet** and
  neither dump falsifies anything. **It remains a bandaid**: the stray write is untouched.
* **The damage is the same shape for the ninth time.** Nine damaged slots across **962 112**
  enumerated, every one the identical `0x0000000100000020`, every one in the `0x20` class,
  **0 of 579 008** elsewhere.
* **A damaged slot was found ON THE POOL'S FREE LIST**, and that kills a hypothesis. A damaged
  slot can never be *pushed* there - the free reads the header first and diverts to `HeapFree`,
  which is the death - so in every surviving history that slot was **unowned when the `1` was
  written**. "The object in the slot underruns its own buffer" is out; overrun-from-predecessor
  and stale-pointer stand.
* **The one-slot-per-250-s rate does not survive contact with a longer session.** 1 046 s gave
  **2**, not four. Damage tracks session age (rank correlation 0.80) and **not** map loads
  (0.05, n = 5, neither significant): the 17-minute run was almost pure idle with two map loads
  and took two slots, against ten map loads in 403 s for one.

* **HENESYS PARK IS NOT FATAL, and the proof had been sitting in `previous-runs/` unread.**
  Map `10001050` has been at the top of the test plan for four runs. On 2026-08-22 a portal
  walk put a character there **52 seconds into the connection**; the client answered `0x00DC`,
  all four NPCs drew, the session ran another fifty seconds and the socket ended **`closed`**.
  Both deaths blamed on the map ended `forcibly closed by the remote host` instead, at **389 s
  and 404 s** - and they were **different faults**: `0xC0000005`, a null read, the first time;
  `0xC0000374`, the accumulating **heap** family, the second. Two different exceptions, both at
  ~400 s, on a map that loads fine early. It was the session every time, and the discriminator
  the plan kept asking for was never actually run at 40 s.
* **Passive regeneration no longer resurrects the dead.** `hp == 0` is the only thing that
  makes a character dead, and the revive dialog fires on the **transition**, so a regen tick
  lifting HP off zero silently undid the death and the dialog could never return.
* **The floating damage number is confirmed a client-side stub**, by the discriminator
  `STATUS.md` set days ago: a Drake emptied a 238-HP bar and the number still read **1**. A
  computed value cannot be constant across snail-to-Drake. What is still open is whether the
  renderer that draws the **blue** recovery number - server-driven, and confirmed on screen -
  can be made to draw a damage number instead.

* **The client's first-ever BUY request is captured, and the layout is settled.** Three clicks
  on three different items, `0x03E1` sub-op `0x02`, and the commodity serial is at **payload
  offset 7** - each one resolving to the item the owner said they had clicked, which is what makes it
  a check rather than a recording. The offset-walk that found it is kept as the fallback for
  the builder's short arm, which has never been seen.
* **A purchase now completes**, and the thing that was blocking it was **my mistake**. This
  file said no `0x05AE` arm could report success without putting a message on screen. That is
  true of the six arms whose bodies are **inline** in the dispatcher and false for the two
  that **delegate to sub-functions**: `0x19` is silent, hands the item over, and releases both
  latches. Searching a known list instead of enumerating the space - the same failure
  `CLAUDE.md` has recorded three times.
* **The client exit is explained, and it was the Brown Puppy.** `5000001` is a **pet**, and
  this client classifies an item **twice**: the factory believes the type byte on the wire,
  the tooltip re-derives the class from the **item id**. A pet sent as a bundle gets a
  126-byte bundle allocation, and the pet tooltip then reads its integrity checksum **four
  bytes past the end of it** and throws. Deterministic, an out-of-bounds *read*, and **not**
  the heap family. Pets are refused everywhere until a type-3 body exists.
* **The non-buy sub-ops are one QUEUE**, so refusals now route by family: `0x1A` for a buy or
  a gift, **`0x3D`** for the queue (because `0x1A` empties the queue vector and would silently
  discard whatever else was pending), and **nothing at all** for `0x2B`, the one builder that
  does not latch.

* **THE CASH SHOP OPENS. Confirmed on a client 2026-08-25, and `0x01A3` is no longer `[D]`.**
  The window drew; the hook log has exactly one `0x14209ad60 ENTERED ... opcode 0x01A3`; the
  stage's own `OnPacket` took both wallets (`rdx=0x5ad`, twice); both balance fields read back
  in the right order; the `0x03E0` poll fired **twice in 103 seconds**, so `0x05AD` really does
  clear its own latch; and an empty `0x00D1` brought the field back with its NPCs. There is no
  migrate and no second server. `research/fixtures/cash-shop-opens-0x01A3-confirmed-*.log`.
* **The shop charges LEAF POINTS, not NX, and that is why nothing could be bought.** The two
  `u32`s of `0x05AD` are both correct and in the right order - `!nx 10000` put 10,000 in the
  field labelled **NX** and 0 in the one labelled **Leaf Points**. But every price tag reads
  **LP**, and with LP at 0 the client **refused the purchase itself and sent no `0x03E1` at
  all** - zero across the whole visit, grepped for that specific opcode. So the affordability
  gate is client-side and reads the second field. `!lp <amount>` grants it; `!nx` is unchanged
  and still fills the first field, which is real, displayed, and buys nothing.
* **Leaf Points and a real purchase, but only from the field.** `!nx <amount>` grants NX -
  what this client's UI calls Leaf Points - and `!buy <commoditySN>` performs a genuine sale:
  it prices the row out of the client's own `Commodity.img`, debits **Leaf Points** and
  fills a locker slot in one transaction. `!locker [slot]` moves it into the Cash tab.
  **A Buy click inside the shop window is refused and nothing is debited**, because no packet
  in this client reports a purchase *succeeded* without also putting a message on screen -
  and the one that looked like it, the wallet, re-triggers the purchase. See below.
* **Buffs are closed both ways**, storage is closed including Organize, and the pick-up latch
  bug is fixed and confirmed.
* **Two crash families are open**, and they are not the same bug: a heap one with four dumps
  and a rate **since falsified** - see the 2026-08-27 entry - and a **null dereference during a map
  load** that has been seen once.

**The next-steps table further down, under "What to do next, in order", is the 2026-08-22 one
and is superseded** - see the banner on it, and the ordered list in `START HERE` at the top of
this file.

#### CONFIRMED on a real client

*Cumulative, oldest first. The 2026-09-03/04 rows are at the bottom.*

| | |
|---|---|
| the world | a dressed character on a map, real `Character.wz` stats |
| NPCs, movement, session | clickable and speaking, portals both ways, `!map`, Log Out |
| **combat, both directions** | *"The mob killings work, I'm taking damage, and the mob is also taking damage."* |
| **drops** | fall at the mob, staggered, **arc out over half a second**, and pick up into the right bag |
| **the EXP line is WHITE** | majority damage. Which also *measured* the `white` byte - it was inferred |
| **the scrolling banner** | `0x00AC` type 4 works, and `!exprate` drives it. Both were inferred; both are now seen |
| **mobs appear instantly** | `appearType -1` on field entry, `-2` on respawn |
| **the quest chain** | Heena -> Sera -> Heena: 1000 completes, 1001 starts, the mirror changes hands |
| **character create** | creating with and without a prior delete both transition |
| **equipping, including a swap** | *"Wearing an item no longer crashes."* And this **kills the heap-corruption repro** - see below |
| **ability points** | *"Assigning AP is fine now, in bulk and in singles."* Both `0x0138` and `0x0139` |
| **the job change** | *"the job sound is fine now"* - `0x007C` bit 5, and the client plays the effect and sound itself |
| **the quest-finish fanfare** | *"the quest completion SFX is now working"* - `0x02D1` effect 15, sound and no picture as predicted |
| **Roger's quest opens** | the authored overlay: their real opening, a **Next**, then an Accept/Decline box |
| **THE CHANNEL MIGRATE OPCODE IS `0x001A`** | measured 2026-08-21. The client tore down, connected to **127.0.0.1:8486** and sent a migration hello - so `u32 ip` network-order and `u16 port` little-endian are confirmed too |
| **death and revive** | the dialog appears, including for a character who logged in **already dead**; revive warps to the town, 50 HP, -10% EXP above level 10, and they could move and fight afterwards |
| **channel change** | claimed by channel, a real `SetField`, and inventory and mesos carried over |
| **bulk skill points** | `0x013B` carries a **count**, and adding three at once now adds three |
| **fresh spawns are spread across the map** | the per-type quota was already balanced; which points inside each type was not |
| **the blue recovery number** | *"I do see 10 in blue above the character"* - `0x02D1` effect `0x41`, and **not** the `0x007C` recovery trailer, which drew nothing across three ticks |
| **a full Equip tab no longer blocks other bags** | the full bag only *triggered* it; a refusal that sent a chat line and no `0x0070` left the client's latch set and it stopped asking for **everything** |
| **buffs, both directions** | Nimble Feet grants and a right-click cancels it. One screen settled `0x007D`, the 124-byte mask, bit 92 = Speed, the **`i16`** value width (unreadable statically - the deciding constant is in Themida-packed `.data`) and milliseconds |
| **Three Snails** | works and deals damage |
| **storage, end to end** | the window, 30 slots, mesos both ways, items in **and** out, the 100-meso deposit fee (ten deposits, ten fees), and Organize Item repacking the box |
| **the Cash Shop button is answered** | `0x00D5` is an **exclusive request**: unanswered it fired once per session and left `[ctx+0x2330]` set. Three clicks give three requests, latch `0/0/0` |
| **THE CASH SHOP OPENS** | `0x01A3` on the same channel socket, **no migrate**. One `0x14209ad60` hook line dispatching `0x01A3`; the stage's `OnPacket` took both `0x05AD`s; both balance fields right, in order; the `0x03E0` poll fired twice in 103 s; an empty `0x00D1` brought the field back with its NPCs. The opcode was `[D]` from three discriminators and is now **read** |
| **the shop's currency is LEAF POINTS** | every price tag reads `LP`, and a client holding 10,000 NX and 0 LP **refused the purchase itself and sent zero `0x03E1`**. `!lp` funds it; `!nx` fills the other field and buys nothing |
| **the type-6 NPC menu renders from the server** | 2026-08-29. Lyn and the Regular Cab each sent one box; each answered by a 10-byte `0x00F3` ending `06 01` with a real selection - **line 2** then **line 0** - each followed by the fare and the `SetField`. Two NPCs, two different lines, one session. `research/fixtures/type6-menu-renders-and-taxi-rides-world.log` |
| **TWO CLIENTS, ONE MAP** | 2026-09-03. Cobalt, Robin and Tester2 on Snail Hunting Ground I, seeing each other **move and attack**. The instance guard is `FindWindowA` + the `CreateMutex` forwarder slot; six launches and three decoded crash dumps to get there |
| **remote avatars are dressed and positioned** | 2026-09-03. `0x0224` accepted and dispatched `ret=1` once the tail was 23 bytes and the seat index was `-1`; a late joiner is told where people **are**, not where they entered |
| **remote damage and the hurt flinch** | 2026-09-04. *"I now see the flinch damage when the non-primary player is taking damage."* `0x02A5` with the damage written into HITINFO **+0xa8** - a plain echo would have drawn nothing at all |
| **the mob flinch follows control** | 2026-09-04, measured rather than watched: controller swings, 10 wounding hits, **10** hit-action reports; non-controller swings, 15 wounding hits, **0** |
| **THE PARTY WINDOW DRAWS** | 2026-09-04. *"You have created a new party."* and a list row with the right name, job and level. Create only - invite's success body is still undecoded |

#### The purchase, the pet, and a mistake of mine that cost two days

2026-08-26/27. The owner bought three items in the shop and then ran `!buy` / `!locker` from the
field; the client exited shortly after. Three agents were fanned out on it.

**The buy request, captured for the first time.** `0x03E1` sub-op `0x02`, 15-byte payload,
matching the builder's read shape exactly - the long arm of the `cmov` pair:

```text
02 | 01 | 02 00 00 00 | 00 00 | 00 68 89 09 | 00 00 00 00   SN 160000000 Brown Puppy
02 | 01 | 02 00 00 00 | 00 00 | a0 ee 8a 09 | 00 00 00 00   SN 160100000 Red Hat
02 | 01 | 02 00 00 00 | 00 00 | e1 fb 8d 09 | 00 00 00 00   SN 160300001 Water of Life
                                ^ payload offset 7
```

All three resolved to the item the owner named, through the offset-walk, without being told where
to look. `net::cashshop::BUY_SERIAL_OFFSET`. **[L]**

**The refusal worked exactly as designed** - three clicks, three `0x1A` messages, no ejection,
the shop stayed usable and Exit was fine. That is the first `0x05AE` ever on this wire.

**RETRACTION: "there is no silent arm" was wrong, and it was mine.** `research/cash-shop-stage.md`
§6.2.1 enumerated the six `0x05AE` arms whose bodies are inline in `FUN_140D7DCA0` and
concluded every arm that clears the latch shows a message. Two arms **delegate to
sub-functions** and are not in that set. `0x19` is silent: it clears `[stage+0x74]` when
`bRelease` is non-zero (`0x140D7F8F8 je` skips it on zero) and clears `[stage+0x120]` through
`FUN_140D74A70`'s cancel path at `0x140D74B40`. I read both branches myself, because two
agents described that arm differently and the difference decides whether a buy leaves the shop
wedged. So the purchase is now `0x05AE 0x19` then `0x05AD`, **in that order** - a wallet sent
first re-triggers the buy. `research/cash-shop-buy-done.md`.

**The client exit was the Brown Puppy, and the control is clean.** `5000001` is in
`5000000..=5009999`, which `FUN_1401B1040` classifies as type 3. The factory `FUN_1403095E0`
believes the **wire's type byte**; the tooltip `FUN_142694130` re-derives the class from the
**item id**. We sent a pet as a bundle, so a 126-byte bundle object was allocated and the pet
tooltip read its `0xBAADF00D`-verified checksum at `item+0x7e`, four bytes past the end, and
threw `ZException` - process exit `0xE06D7363`. A grep of **every** archived log for
`ADD: item 5xxxxxx` returns exactly one hit: that run. The first time a pet id ever reached
this client's bag is the run that died. `research/cash-item-throw.md`.

Note what this was *not*: the `0x0070` handler **returned** cleanly in 149 us, and the throw
came 3.4 s later out of the client's own loop. The packet parsed; the **draw** killed it. A
guard now refuses pets in `!item`, `!locker` and the purchase path, and `!locker` puts the pet
**back in the locker** rather than losing it. Only 4 of 159 sale rows are pets; drops and
shops cannot reach one at all.

**The other sub-ops are a queue.** `FUN_140D74A70` pops 32-byte records and `[stage+0x120]`
holds the **in-flight kind**, not a boolean. `FUN_140D74C70` - what makes `0x1A` better than
`0x1E` - empties the vector at `[stage+0x128]`, and that vector **is** the queue. So `0x1A` is
right for a buy and wrong for `0x0A`/`0x0B`/`0x1C`; those get `0x3D` with a `u16` reason.
`0x2B` does not latch at all and is now deliberately unanswered, which is the one documented
exception to "always answer" - measured with a positive control, and explained on the inbound
log label because `Session` has no logger by design. `research/cash-shop-actions.md`.

Also worth keeping: **`0x0A` is the locker to Cash-tab move**, answered with `0x19` - the real
version of what `!locker` fakes today. And `0x1B` is a trap: with `bToSlot = 0` it reaches
`FUN_1401ABD80(tabArray + 0)`, whose first instruction dereferences with no null check.

**The reference source scored 0 of 5 on numbers this pass** (1 of 8 lifetime). Every candidate
was killed against the client rather than aligned to.

#### The cash shop opens, and the currency is Leaf Points

2026-08-25. One launch settled the entire entry path, and every prediction in the plan came
back the way it was written down - which is worth saying, because the opcode was a `[D]`
derived from three discriminators and this was the run that could have falsified it.

```text
04:23:23.355  <- 0x00D5                            the button
04:23:23.355  -> 0x01A3 SetCashShop                and the window DREW
     hook     ***** 0x14209ad60 ENTERED ... while dispatching opcode 0x01A3   <- exactly once
     hook     ***** 0x140d734e0 ENTERED ... rdx=0x5ad                         <- twice
04:23:53.419  <- 0x03E0  ... 04:24:53  <- 0x03E0   two polls in 103 s: the 60 s throttle
04:25:06.603  <- 0x00D1  EMPTY BODY                the Exit button
04:25:06.611  -> 0x01A0 SetField                   the field came back, NPCs and all
```

`0x01A3` is no longer `[D]`. There is no migrate and no second server, and
`crates/cashshop` was never needed. `research/fixtures/cash-shop-opens-0x01A3-confirmed-*.log`.

**The balance fields are both right, and neither is the one that buys.** `!nx 10000` put
10,000 in the field the UI labels **NX** and 0 in the one it labels **Leaf Points** - so the
two `u32`s are in the right order and the right unit. But every price tag in the shop reads
**LP**.

**The decisive observation is a negative, and it is the specific-opcode kind.** With LP at 0
the client sent **zero `0x03E1`** across the whole 103-second visit. Not "nothing new arrived"
- a grep for that one opcode, which is the control `CLAUDE.md` records the cash shop itself
teaching this project the hard way. Had the gate been reading the NX field it would have sent,
because that field held 10,000. So the affordability check is **client-side** and it reads the
second balance.

`!lp <amount>` grants it. `!nx` is deliberately unchanged - the owner asked for a second command,
not a changed one - and the field it fills is real and displayed. It simply buys nothing.

`store::buy_cash_item` and the shop's own affordability check now debit Leaf Points, and the
tests moved with them: `a_purchase_that_cannot_be_afforded_leaves_both_sides_untouched` now
funds a million NX first, so a purchase that leans on the wrong pot fails the suite rather
than the run. **`Commodity.img`'s `Price` column is labelled "NX" in
`research/cash-shop-items.md`, off the WZ property name.** The screen disagrees with the
property name, and the screen wins.

**TO FIX: NPC chatter follows the player into the cash shop.** The stage's `OnPacket` logged
**38 hits with `rdx=0x453`** in 103 seconds - idle chatter for the field they left. The stage
has four arms and returns for everything else, so it is **harmless**; it is log noise and a
server that is still treating a shopping player as though they were on the field. The owner asked
for it to be fixed, deliberately not in the same change as the purchase work.

The fix wants a `Session` flag for "which stage am I on", set in `on_cash_shop_request` and
cleared in `on_cash_shop_exit`, gating the chatter in `tick()`. **That flag is wanted twice
over**: nothing on the `0x01A3` path writes `[ctx+0x31fc]`, one of the six gates the Cash Shop
button checks, and a stage flag is what would let the server reason about re-entry at all.

#### Leaf Points, a real purchase from the field, and why the shop window still refuses

2026-08-24, the owner: *"we need a way to add Leaf Points (NX) in our server so we can attempt to
make purchases in the Cash Shop so we can finish that entire transaction flow."*

`!nx` already existed and works. What was missing was everything after it, and one piece of
it turns out to be genuinely unavailable in this client.

**The server now knows what things cost.** `gm-handbook/commodity.txt` - 159 sale rows out of
the client's own `Etc/Commodity.img` - is loaded at start-up into `world::commodity`, keyed by
**SN**, not item id. That distinction is load-bearing: `130200000` is one Megaphone for 100 NX
and `130200001` is eleven for 1000, and both are item `5070000`. The banner prints the row
count in both directions, because the client draws its catalogue from its own copy of the same
file - so an empty table here looks exactly like a fully stocked shop.

A measurement that fell out of loading it: **the 21 rows priced at 0 NX are exactly the 21
rows that are switched off.** Nothing buyable in this client is free, so no purchase can be
tested without `!nx` first.

**`!buy <sn>` performs the sale for real** - `store::buy_cash_item` checks the balance, debits
it and fills a locker slot in one transaction - and `!locker [slot]` lists the locker or hands
an item to the Cash tab. The hand-over is two stores and therefore two transactions, so the
failure path puts the item back with `store::put_cash_item`; an item that left the locker and
failed to reach the bag would simply cease to exist.

**And a Buy click inside the shop window is refused, deliberately.** Three things were read
out of the image and each one closes a door:

* every `0x05AE` arm that clears the shop's in-flight latch also calls `FUN_140D7C7F0`, which
  shows a message. **There is no silent one**: `0x140D7C818 cmp edx, 0x7e / ja` sends every
  reason outside `1..=0x7F` - including `0` - to string 661, the generic error.
* answering with the **wallet** would clear the latch and *re-trigger the purchase*. The buy
  builder sets `[stage+0x120] = 1` at its send site (`0x140D7A500`) and the `0x05AD` arm calls
  that same builder back when it sees a 1 (`0x140D736E6` -> `0x140D736F6`). That is a farming
  loop, not a purchase.
* the remaining candidate needs `FUN_1402D0950`'s **71-byte** cash-item record, whose field
  meanings past the serial are explicitly not established. This project has shipped a packet
  short twice this month.

So nothing is debited on that path. **Every effect hangs off the transition** - the Heena rule
- and a purchase the client is never told about is not a transition.

**A correction to `research/cash-shop-stage.md`, section 11.4.** It recommended refusing with
`0x05AE` sub-op `0x1E`. That is wrong, and the reason is the same `[stage+0x120]`: `0x1E`
leaves the pending purchase armed, so the client's own unprompted 60-second wallet poll makes
it buy again by itself. Sub-op **`0x1A`** enters the identical body one call earlier, at
`FUN_140D74C70`, which clears that field first. Both the byte index table at `0x140D7E194` and
the 22-entry jump table at `0x140D7E13C` were dumped rather than inferred; the same dump also
corrects the file's claim that `0x1A` consumes nothing - it falls through into `0x1E`'s
`u8 nReason` read, so a `0x1A` sent without a reason byte reads past the end of the packet.

**The v214 reference was checked against all of this afterwards, and it closed a door rather
than opening one.** Its `CashItemType` enum names both halves of the pair. Where it agrees it
agrees exactly - `Req_Buy(2)` and `Req_Gift(3)` are the numbers already read off the client's
own `cmov`, and all four of its `_Failed` entries that are live here land on our `u8 nReason`
arms. But **`Res_Buy_Done(14)` maps to sub-op `0x0E`, which is dead in this build** - the byte
index table gives `0x0E..0x12` the default slot. The surviving candidate is
`Res_AddedCashItem_Done(3)`, and our `0x03` does read `u16 count` then that many cash-item
records - but its arm never touches `[stage+0x74]` or `[stage+0x120]`, so it cannot release the
UI on its own, and it still needs the 71-byte record. **[I]** throughout;
`research/cash-shop-stage.md` section 6.6.

**The instrument for the next run.** No real `0x03E1` has ever been captured, so which field
of the buy payload carries the serial is unknown. Rather than pick one, the handler reads a
`u32` at **every** offset and checks each against the client's own sale list, reports
`Several` rather than guessing if two match, and writes the answer to `world.log`. It cannot
invent a serial: it can only return one that is really in `Commodity.img`.

#### The 2026-08-22 run: four answers, two of them negative and both useful

* **Quest EXP is in the chat log and the fanfare plays.** The owner: *"Yup, quest exp is in the
  chat now, which is correct. SFX is playing as expected."* Both confirmed on screen. The
  grey **item** line is still unreported - no quest that grants an item was run.
* **The blue recovery number does NOT come from the `0x007C` trailer.** A clean negative, and
  the packet is not in doubt: `world.log` has three `idle regen ... with the recovery trailer`
  lines and the bar moved on each. The owner: *"The blue number on top of the character is still
  not drawn, but the HP is going up."* So the trailer reaches the client, is well-formed, and
  draws nothing. The next thing to check is the **argument order** at `142d548ef`:
  `FUN_140fd31f0(uiGlobal, hpRecovery, mpRecovery, oldHp, oldMp)` takes snapshots of
  `record+0x5b`/`+0x73`, and if those are taken *after* the mask block has already stored the
  new totals then `oldHp == newHp` and there is nothing to draw. If that is it, the fix is to
  send the trailer **without** the hp/mp fields and let the client add them.
* **The floating damage number is a stub. Settled.** The owner: *"Red snail is decreasing my bar by
  10 but the damage number is still shown as 1."* Two different mobs, damage 3 and damage 10,
  and the number is **1 both times** - as it was on all 25 hits of the previous run. A
  computed value cannot be constant across a 10x change in the input. The client does not
  compute mob damage, and `user-hit.md` §4.4 establishes the number is drawn at *send* time by
  `FUN_142771360`.
  **CORRECTED 2026-08-27: the redraw packet was found.** *"unless a redraw packet is found"*
  was the right hedge and it has been answered. `FUN_142771360` forks at `0x142771395` on the
  **sign** of its `i32`: positive picks the blue recovery digits, negative picks the damage
  ones - so the blue number the owner has already seen and the damage number are the same call with
  the sign flipped, and the server now sends `0x02D1` effect `0x41` with a negative amount.
  What is still true is that **the client's own `1` cannot be suppressed**, so the screen shows
  two numbers. `research/damage-number-draw.md`.
* **The NPC "fade" is not a fade, and `0x0452` is not the cause.** The owner: *"I turned npcfx off,
  but npcecho copy of Heena that newly showed up still faded in. Once it fades in, it is no
  see through, it's just absent-then-present, it's a very fast fade in effect but
  noticeable."*
  Two results in one sentence. The `0x90` appear object is **eliminated** - turning it off
  changed nothing - and that was the only creation-time branch left in `FUN_141e36b20`. And
  the observable nobody had ever reported is now reported: **not see-through**. So it was
  never an alpha ramp. `research/npc-fade.md` §8 named this exact possibility and said it was
  cheap to settle and nobody had: *"an object that arrives late, or that is drawn while the
  map's own transition is still running, reads on screen the same way."*
  The question changes from *"which field controls alpha"* to **"why is the first draw
  late"**, and the candidate that fits a late first draw is the pool's `0x0467` **template
  preload list** - `u8 count`, then that many template ids, each fed to `FUN_141e77b70`.
  Untried, and it is the natural shape for "load the art before you need it".

#### Fixed 2026-08-22: bulk skill points only ever added one

The owner: *"I just tried to bulk add 3 points into Three Snails, but it only went up 1 point."*

`0x013B` carries a count and `on_skill_up` threw it away - `let next = level + 1`. The capture
is unambiguous: `<- 0x013B 940e8711 e8030000 03000000` is tick, skill **1000**, count **3**,
and the reply said *"skill 1000 raised 0 -> 1"*.

`SkillUpRequest::count`'s own doc block already said what it was *and* what to do with it -
*"`1` for `BtSpUp`, `min(sp, maxLevel - level)` for `BtSpUpAll` ... the server must clamp
again: nothing here authenticates"*. Both halves were ignored. Same shape as the quest
payouts: the information was in front of the caller.

Now honoured and clamped to **`BEGINNER_SKILL_MAX_LEVEL = 3`**, which is **[L]** out of the
client's own `Skill.wz` rather than game knowledge: `000.img` gives `masterLevel = 3` *and*
exactly three numbered `level` children for all three of 1000/1001/1002. The level table is
the stronger half - a `masterLevel` could be a ceiling the job never reaches, but a level
table cannot describe a level it does not contain. Re-derive with
`target/release/wz-dump cat "client-patched/Data/Skill/Skill_000.wz" 000.img`.

**Still missing, and worth saying out loud: this server does not track SP at all.** There is
no `sp` column on `characters`, so nothing checks that a point was available to spend. The
cap on the level table is the only limit.

#### Where a dead character respawns: the data exists now

`tools/dump_returnmaps.py` -> `gm-handbook/returnmaps.txt`, 426 rows, **tab** separated
because one map is called `The Resting Spot, Pig Park`. Re-running it is byte-identical, and
every count below was re-derived from the file rather than taken from the report.

* **`returnMap` is the revive anchor.** 426/426 fields carry it, none is the sentinel, and it
  lands on a `town == 1` field for 388 of them. It is **not** a map exit: cross-referenced
  against `portals.txt` it is not among the field's own portal targets on 316 of the 401
  fields that have a door, and 130 fields have no portal path to it at all within six hops.
  Map 40 returns to 60 Southperry, several screens away. A teleport target. That it is the
  **death** destination is **[D]**, not [L] - the WZ never names the event - and the argument
  is `Return Scroll - Nearest Town` (2030000) carrying the same sentinel in `spec/moveTo`
  where every other return scroll carries a literal town id.
* **`forcedReturn` is eject, not respawn. Do not use it for death.** Sentinel on 354/426; 72
  carry a real id, and those 72 are ship cabins mid-flight, timed subway depots, PQ stages and
  instanced dungeons - fields you must not be left standing in. It disagrees with `returnMap`
  on 28 of the 44 where both are real, and the disagreement is the discriminator: `Dead Mine I`
  returns to El Nath, the town, but force-ejects to the field outside the mine. Decoded and
  deliberately unwired; it is a login-placement question.
* **`town` is coarser than "town square"** and a resolver that stops on it is wrong: shop
  interiors carry `town == 1` too, and **94 of the 115 town fields point `returnMap`
  elsewhere**. Stopping on the flag revives the player inside Southperry Armor Store. The
  shipped rule is one **unconditional** hop, then walk while not a town, cap 8.
* **It terminates.** 421 of 426 resolve in one hop, 5 in two, none deeper, none with no
  destination. There are 33 self-loop cycles (a PQ, 22 event stages, 3 test maps); returning
  the last real field means a PQ death lands on stage 1 instead of hanging. None of the 33 is
  reachable on foot - walking portals from map 1 reaches 25 fields, all Maple Island.
* **`NO_MAP = 999_999_999`** is the client's own "no map id here" sentinel, the same number
  `portal/<n>/tm` uses. Never warp to it.

**A note on how that was got, because it is this repo's own rule biting the coordinator.** The
brief I wrote suggested `VRLimit` as the positive control for the WZ reader. **There is no
`VRLimit` key in this client** - it is `VRTop`/`VRBottom`/`VRLeft`/`VRRight` - so that control
could only ever have failed, and a reader verified against it would have been declared broken
while working. The agent caught it and used `version`/`bgm`/`mapMark` (426 each) plus
`fieldType` (379, so it also proves the reader *discriminates*) instead. Handing out a control
that cannot pass is the same mistake as trusting a scan that cannot find anything.

#### Death and revive: BUILT 2026-08-22, unseen

`0x0315` opens the revive dialog and **the client never opens it by itself.** The chain was
enumerated two ways with different blind spots: `CUIRevive` is constructed at exactly one
site, `tools/dataref.py` on the dialog global finds 13 references with a **single write** in
that constructor, and that constructor's only reachable caller is index `0x50` of the
local-user table at `0x14289d660` - `0x2C5 + 0x50 = 0x0315`. *Control:* index `0xC` decodes to
`0x02D1`, which `research/level-up.md` had established independently. **[L]**

Body is 23 bytes, eight fields, and only two matter: `a`'s bit 0 must be set and `b` must not
be `9`. **Both failures are silent**, which is why the builder takes no arguments.

**Ordering is a real constraint, not a nicety.** The handler gates on a client-side HP test of
the value the server just wrote, so a `0x0315` that overtakes its `0x007C` is dropped without a
word. It goes after, in the same batch, and is gated on the **transition** (`before > 0 &&
hp == 0`) because a dead character can still be hit.

**The revive click is an ordinary `0x00D1` with `targetField = 0`**, 25 bytes where a portal
walk is 34. `parse_transfer_field` reads that `0` as a perfectly good map id, so without a
guard a revive would have warped the character **to map 0**. The guard is on the server's own
`hp == 0`, deliberately not on the packet shape.

Revive does: `reviveMap` from `gm-handbook/returnmaps.txt`, 50 HP, and -10% EXP above level 10.
A second `0x007C` follows the `SetField` because ~65 client sites gate on the sign of HP and a
character revived without it arrives in town unable to act.

`0x01E7` (revive on the spot) is answered and refused - the button is hidden unless a Respawn
Token counter is above zero and nothing here sets it, so seeing that packet at all is itself
the finding.

**And there is a free instrument if it does not work:** when the opener refuses, the client
reports it outbound as `0x02C6`, 44 bytes, carrying which of four things went wrong. Success
is silence.

#### CONFIRMED 2026-08-22: death and revive, first time out

The owner: *"I already performed the revive last time, it worked, and then the crash happened."*
The log corroborates it exactly, and it covers **both** entry points:

```text
02:50:28.547 -> 0x0315 ShowReviveDialog on field entry: character 212 arrived on map
                10000022 with 0 HP
02:50:31.161 -> 0x01A0 SetField, REVIVE: map 10000022 -> 10000000, hp 50/194,
                no exp penalty at level 10
```

They clicked the button 2.6 s after the dialog packet, so they saw it. The **dead-login** path is
the one that mattered - that character had been stranded since the previous session and would
never have got a dialog from the combat path. And the half most likely to fail silently, "can
you act afterwards", is answered by what happened next: **they reached level 11.**

**And I nearly had them re-test all of it.** The plan still opened with "LOG IN AS Idiot
FIRST". Two of the other steps were in the same state - the blue number went out **16 times**
as `0x02D1` effect `0x41` and the grey quest item line went out for quest 10001 - so three of
five steps were asking for work the run had already done. `CLAUDE.md` has a section telling me
to strike finished items off, and one telling me to count the same event in two logs; the fix
for both was one `grep` of `world.log` that I did not run before writing the plan.

What is genuinely still unknown is **what they saw**, not what was sent. The plan now asks only
that.

#### Idle regeneration does not touch a full bar

The owner: *"the server should not try to idle regenerate if a character is full HP."*

The both-full case was already an early return and the log bears it out - a heal ends with a
capped `+4 hp -> 194/194` and then stops. The half that was **not** handled is one bar being
full: with HP at maximum and MP short, the `0x007C` still carried `hp = <full>`, restating a
value that had not moved. Each field is now present only if it changed.

**Left open deliberately, because it is the owner's call and not mine:** whether MP regeneration
should also stop when HP is full. Independent regeneration is the usual behaviour in this game
family, so stopping it would be a real change rather than a tidy-up, and their wording could
mean either.

#### 2026-08-22: fresh spawns were bunched at one end of the map

The owner, on Right Around Lith Harbor: *"the mobs that spawn are completely concentrated on the
left side of the map on fresh spawn. The spawn points that gets activated should be randomly
chosen even on fresh spawn."*

`share_balanced` computes a per-**type** quota and then took `idx.iter().take(n)` inside each
group - the first n in WZ order. `life` entries run left to right, so every fresh field put its
mobs at the low-x end.

**The interesting part is that this function was already the fix for the other half of the same
bug.** Its own test says so: *"Taking the first N in WZ order would return almost all of one
type, which is the bug this replaces."* That fix was correct and stays; it balanced **which
types** spawn and said nothing about **where**. A partial fix that names itself in a test is
easy to read as a whole one.

Now shuffled per group with a seeded splitmix64 (`(map << 32) ^ now_ms * const`), so the quota
is untouched and only the choice of positions moves. Three tests, and the third is the control:
the spread across the list, that two seeds differ while one seed reproduces, and that the
per-type counts are **identical across four seeds**. The spread test was checked against the
old `take(n)` and fails on it for every seed rather than probabilistically.

#### A second crash dump, and it corrects the first one's guess

2026-08-21 23:01, 1.33 GB, `0xC0000374` again, 644 s of life against 596 s.

**The stack is the same path frame for frame** - `RtlFreeHeap` <- `MapleStory.exe`
`fn 0x14019b4e0` <- `PCOM.dll` `fn 0x152c12ce0` <- the PCOM chain <- `oleaut32!VariantClear` <-
`NAMESPACE.DLL`. Two independent deaths on one call path is corroboration the single dump could
not give, and it points the search at that refcounted release path rather than anywhere else.

**And the first dump's finding generalises exactly - my first reading of this dump was
wrong.** Every damaged header in both processes is the identical `0x0000000100000020`: five
instances, two processes, one value, and all five in the `0x20` size class (0 damaged in
249 496 slots of the other three classes, 5 in 158 688 of that one).

I had written the opposite - *"differently garbled header bytes ... a wild write, not one
repeatable off-by-one"* - and used it to retract the first dump's finding. That came from one
line of `dumpwalk.py` output. **`_HEAP_FAILURE_INFORMATION.Address` is the entry (`ptr-0x10`)
for a type-8 failure and the caller's pointer for a type-9**, and the tool decoded it as an
entry either way. The "garbled bytes" were `0072005000000010` - a **BSTR**: length prefix
`0x10`, sixteen bytes, eight UTF-16 characters, beginning `"Pr"`. The string `"Property"`.

`tools/dumpwalk.py` now refuses that decode unless the failure type says it is an entry, and
`tools/poolchain.py` enumerates the pool exactly - 2 238 chunks passing a size identity with
0 failures, so it validates itself. The two failure types are two arms of one `if` in ntdll
(`test r13b, 0xf`), and since the pool stride is 40 the headers alternate 0/8 mod 16 - **the
type is decided by the parity of the damaged slot's index**, not by different damage.

`research/heap-wild-write.md`, `research/fixtures/heap-second-dump-same-damage-value.log`.

#### Storage is wired, 2026-08-22

`0x0572` out, `0x00F6` in, both found statically with no client run. Clicking a storage keeper
now opens the box, beside the shop branch in `on_npc_click` and before the conversation
fallback - which is why Mr. Kim did nothing visible rather than doing something wrong: they have
no `d0` line to fall back to.

Verified here rather than taken on trust: the storage gate mask comes out as mesos at 1 and the
six bags at `[2, 3, 4, 5, 6, 44]`, byte-for-byte `net::bag::BAG_PRESENCE_BYTE`, derived by the
agent from a **different** key table. The `44` is what makes that a real cross-check rather than
a coincidence. Mr. Kim is template **105** (`npcstrings.txt`) and their `info/trunkPut` is **100**
straight out of `Npc_000.wz`.

**Three things that would each have cost a launch:**

* **The take-out "index" is positional, not a slot** - the 0-based position within its type's
  list *as the server sent it* - while the put-in request at the **same field offset** carries
  the player's real 1-based bag slot.
* **Every `0x00F6` latches `dlg+0x334` and only a `0x0572` clears it.** Every path out of the
  handler answers, including the ones that change nothing.
* **The wire's meso sign is the opposite of the store's.** `0x00F6` mode 7 is one signed `i64`
  where **positive withdraws**; `store::move_storage_mesos` takes positive to mean **deposit**.
  Getting that backwards would not error, crash, or look wrong in a log - it would quietly move
  money the other way. The test for it was checked against an un-negated version and fails.

Item movement is **not** built - take-out and put-in answer with the box unchanged and say so.
Mesos work in both directions, through the store's single transaction. And per-account rather
than per-character is **[I]**: nothing on the wire carries an owner in either direction, so one
launch with two characters on one account is what would settle it.

#### Nimble Feet: `0x013D` was a coincidence, and it is an anti-cheat census

I wrote that `0x013D` *"arrives exactly every 30 seconds carrying skill 1002, and 30 s is the
skill's duration - that is where to look."* **The 30 is unrelated to the 30.** One grep of the
same log settles it: `0x013D` was already ticking at 02:50:28 and 02:50:59 with a nine-byte
**empty** body, and Nimble Feet was not raised until 02:51:11. The cadence predates the skill.

`0x013D` is a **30-second anti-cheat census**:
`u8 reset, u32 opcodeCount, {u32 opcode, u32 n, n x {u32 skillId, u32 count}}, ...`, and the
unexplained `0x13c = 316` is the **opcode `0x013C`**. Proven by counting rather than by
reading, which is why it is worth repeating: the 41-byte body claims skill 0 x **7** under
opcode `0x00DF`, and `grep -c "<- 0x00DF" world.log` is **7**; it claims skill 1002 x **1**
under `0x013C`, and there is exactly **1**. Both re-checked here. **The client is reporting,
not asking** - 22 went unanswered in that session with no freeze, so it must not be answered.

**What was actually wanted:** **[L]** unless marked.

* **The skill-use packet is `0x013C`**, one per cast, 51 bytes:
  `u32 skillId, u32 skillLevel, u32 tick, u32 crcLevel, u32 crcSkill, u8, u32, u32, u32,
  u16 x, u16 y, u8`. Cross-checked two ways - the x/y match the bracketing `0x00D9` movement
  packets, and both checksums reappear in the `0x01A5` bodies for skill 1002. The server needs
  offsets 0 and 4.
* **The grant is `0x007D` TemporaryStatSet**: a **124-byte mask** (31 LE `u32` words, bit
  `1 << (31 - (idx & 31))` in word `idx >> 5`), then per set bit `{value, u32 reason,
  u32 duration}`, then a 13-or-14-byte tail. **Duration is milliseconds**, and that is
  measured, not assumed - the client stores `tExpire = tick + duration` and `timeGetTime` is
  this PE's only clock import.
* **Nimble Feet (1002)**: `speed` **+10 at every level**; `time` **10 / 20 / 30 seconds**;
  `mpCon` 4/7/10; `cooltime` 180 s. Only the duration scales with level.
* **CTS bit 92 is Speed**, and from this client rather than the reference tree: string `0x14DA`
  is *"Nimble Feet cannot be used while another Speed increase effect is active"*, the only
  `mov r32,0x14DA` in the image reads `secStat+0x5cc/+0x5d4` and `+0x5d8/+0x5e0`, and those are
  exactly the value/reason pairs one identified block writes. The reference tree has 14 entries
  where this client has 13, so it does not align and was not used.

**Two hazards for whoever wires it.** `0x007D` **collides with the inbound migration-hello
constant** already in `crates/world`, so the outbound one needs its own name. And the value
field is `u32` or sign-extended `i16` depending on a constant in **Themida-packed `.data`** -
32% bit density, i.e. not in the file at all. That is a missing section, not a failed search.
The write-up sends `i16` and neutralises the tail with 18 zero bytes so all four possible
parses stay in bounds, and makes the wrong-width case **visible**: under a `u32` parse the
duration reads 0, so the icon flashes and vanishes instead of counting down.

`research/buffs.md`. Nothing is built.

#### The classic shop is decoded, including the price

`research/classic-shop-rows.md`. The four things `classic-shop-opcode.md` listed as undone:

* **The price is `row+0x38`, a 64-bit meso amount** - **[L]** from three independent sites: the
  affordability check multiplies it by quantity and compares against the player's mesos
  (failure prints *"You don't have enough Mesos."*), the discount function reads it, and the
  row renderer formats it and appends string `0x4AF` = `" Mesos"`. It is **not** negated for
  the sell tab; that was a Shop2 convention.
* **The conditional tail: the predecessor stopped one branch too early.** The `u8`'s `je`
  target is the instruction before the *next* read, so it guards **one** sub-decoder and the
  27 reads after it are unconditional. The row is **42 direct reads**, not thirteen fields -
  42/42/42 across listing, decompiler and `tools/reads.py`.
* **Head `a` is the NPC template id**, now [D] rather than [I]; `d`/`name`/`e` are echoed
  verbatim in every request; `c` and `b` remain unexplained.
* **`0x055E` is a 41-case switch**, not a type byte. Unknown types are safe.
* **The request opcode is `0x00F5`, not Shop2's `0x0104`** - four sub-ops, and its latch is
  stricter: only a `0x055E` clears it, and a fresh `0x055D` cannot, because `0x055D` is
  discarded whenever any modal is open **including a `0x055B` script message**.

**Buy-back is the same row array tagged by a per-row `u8`** - not a second block and not a
separate packet. The client sends the *same* sub-op for a buy-back as for a buy, so the server
tells them apart purely by which row index it is, because the server set the flag.

**Three sentences that are worth more than the rest, given this packet has killed the client
twice:**

* `row+0xa4` is a sale-end FILETIME compared against the wall clock **with no sentinel**, and
  `0` **hides every row** - the shop opens empty with nothing in any log.
* `row+0x10c = 0` makes every purchase **fail silently**.
* Gates on `row+0xf0` and `row+0xa4` drop a row **before** its trailing item blob is read, so a
  buy-back-flagged row that gets dropped **desynchronises the stream by one byte and every
  later row is garbage**. Never flag a row any gate might drop.

Nothing is built. §9 gives the exact 336-byte body for a two-item shop as a field list *and* a
hex dump, so an implementation can be diffed rather than re-derived.

#### Storage is built; the shop is unbuilt on purpose

* **Mr. Kim's storage is BUILT and half of it is CONFIRMED as of 2026-08-22.** The window
  opens with 30 slots and mesos move both ways, both seen on screen. It was the textbook
  "built is not wired": the database half had been ready for days and nothing had ever put it
  on the wire. **Item put-in and take-out were then a second instance of the same thing** -
  the requests parsed and the answer was "not implemented yet, here is the unchanged box" -
  and the owner hit exactly that: *"the item did not move to storage, and it did not charge the 100
  meso fee that it said it was going to charge."* Both arms are wired now and neither has been
  on a screen.
* **Mina's shop is off deliberately, and turning it on blind would kill the client.**
  `research/classic-shop-opcode.md`: this client has **two** shop windows. `crates/net/shop.rs`
  builds `0x0560`, whose art (`UI/UIWindow2.img/Shop2`) is **absent from this client's WZ** -
  which is why the shop killed the client twice, the constructor dying before a single row byte
  was read. The right one is **`0x055D`**, whose art is present.
  **The row is now decoded** (`research/classic-shop-rows.md`): 42 reads, not thirteen fields,
  and the **price is `row+0x38`, a u64**, from three independent sites. It is still not built,
  and the reason has moved: not "the price is unknown" any more, but the three fields above -
  a FILETIME with no sentinel, a silent-failure gate, and a dropped row that desynchronises
  every row after it.

**The owner's buyback spec, recorded because only they have it:** the counter keeps the **last 15
items sold to any NPC**, so a sale can be undone by buying it back, and the list is **cleared on
server restart or player logout** - i.e. it is per-session, not persisted. The client side of
that exists: `repurchaseInfo` is one of the classic window's own `.rdata` fragments, and the
screenshots show the `Buy Back` tab beside `All`.

#### Death had one entry point and needed two

The owner, 2026-08-22, the first time death was in front of a client: *"My character 'Idiot' has 0
HP from last time, and I don't see any revive dialogue when I login because I immediately
spawned in dead."*

`on_user_hit` opens the dialog on the **transition** alive -> dead. That is right there, and
it is exactly what stops a dead character being re-prompted on every further hit. But **logging
in dead is not a transition** - the zero was already in the database - so nothing fired, and the
character was stranded with `!heal` as the only way out.

The general form is worth keeping, because the gating decision that caused it was the *correct*
one: **a state that can be entered by more than one route needs its recovery offered on all of
them.** Gating on the transition is right for the event and wrong for the state, and the second
entry point here is not another packet, it is a *reload*.

Fixed in `on_field_entered`, which is the right moment and that is measured rather than hoped:
field entry runs `FUN_142caa4e0` and tears dialogs down, but `0x00DC` is emitted from *inside*
the `SetField` handler and the client dispatches nothing until that returns ~586 ms later, so a
reply to `0x00DC` lands about a millisecond after the reset has finished
(`research/npc-preload.md` §4). A `0x007C` restating `hp = 0` goes first, for the same reason it
does in combat.

#### The blue recovery number was on the wrong packet, and the second theory was wrong too

`0x007C`'s recovery trailer **can never draw anything**. It was sent on three ticks of a real
run, on a bar that visibly moved, and drew nothing - and the reason is not a bad body:
`FUN_140fd31f0` is a **statistics counter**. Running totals at `+0x208`/`+0x210`, effective
healing separated from wasted against `maxHp - oldHp`, per-hour averages, a reset on the hour.
Its entire call list is two tick functions, a getter twice, and a tail `jmp`.

My follow-up theory - that the `oldHp` snapshot is taken *after* the mask block stores the new
total, so the delta is zero - is **also dead**, and is recorded because testing it would have
cost a client run: the snapshots are taken 65 bytes earlier, into `[rbp-0x58]` and callee-saved
`r15`, where the mask block cannot reach them.

What draws it is **`0x02D1` effect `0x41`** - the *same renderer as the damage number*, with a
**positive** argument, because the sign is what selects digit set 2 (`NoBlue`) over set 3
(`NoViolet`). Of the eleven call sites of the digit-set loader, exactly one ever asks for set 2.
Now sent by idle regen, after the `0x007C`, HP only.

**One link in that chain is unmeasured** and it is named rather than buried: effects `0x41` and
`0x23` share a suppression gate at `14278bd75`. If the hook log shows the `0x02D1` dispatched
*and returned* with nothing on screen, that gate is the place to look - and swapping `0x41` for
`0x23` does **not** test it.

#### Death and revive: opened, not built

The owner: *"My HP hit 0, I see the tombstone on my character, but I do not see the revive
confirmation. Reviving a character should warp them to the nearest town, start at 50 HP, and
reduce their EXP by 10% unless they are level 10 or below."*

What was found, all **[L]** unless marked:

* **The dialog exists and its art is present** - `UI/Revive.img` in `UI_000.wz`, 1472 bytes,
  with `backgrnd`, **`button:town`**, `anibutton:spot`, and a message rectangle. Unlike the
  QuestClear case there is nothing missing from the WZ; the dialog can draw.
* `FUN_1411a3440` loads that string. It has **zero callers of any kind** and **one qword
  pointer** to it - slot 4 of a 66-slot vtable at `0x14338acf8`. It is a virtual method.
* The vtable is installed by `FUN_1411a2ea0`, which has exactly **one** caller,
  `FUN_142cb5cb0`, which in turn has **three**: `FUN_14289a3a0`, `FUN_142903cf3` and
  `FUN_1429376c0`. All three are in the user-hit family - `user-hit.md` §7 already named
  `FUN_14289a3a0` as the one `0x00E5`-family function that touches the request latch.
* Above that the chain **stops**: `FUN_1429bbd50` and its siblings have zero calls, zero tail
  jmps and zero data pointers. That is the Themida-VM dispatch signature - the same wall the
  channel-migrate reply hit, where the opcode could only be found by sweeping.

**The hypothesis that ties this to the damage stub [I]:** the revive dialog is opened from the
client's own hit path, and the client's own damage is a constant 1, so by its arithmetic it
never dies. The tombstone the owner sees comes from `hp = 0` in our `0x007C` (§6.2: hp<=0 gates
every action off); the dialog does not, because nothing on the client's side ever concluded
the player died.

**Nothing is built.** Death detection, the revive effects (nearest town, 50 HP, -10% EXP above
level 10) and the dialog trigger are all still to do, and `returnMap` is not dumped - it is a
known `Map.wz` key (`config.rs`'s list) that `tools/dump_portals.py` does not currently emit.

#### CONFIRMED on the 2026-08-21 evening run - the big two

* **THE CHANNEL CHANGE WORKS.** The owner: *"Channel changed successfully seemingly. I checked my
  inventory items and mesos, seems like everything carried over."* `world-ch1.log` says how:
  `MIGRATION HELLO: character id 212` then *"claimed the migration for character 212 of
  account 1"* and a real `SetField` carrying map 40 - not the minimal fallback that killed the
  client last time. Goal closed: the opcode is `0x001A`, the body is `u8 ok, u32 ip` network
  order, `u16 port` little-endian, and identification is by **channel** because a migrate
  carries no character id.
* **THE HEAP CORRUPTION IS IN A DUMP.** For the first time in this project, after six
  undiagnosed deaths. `dumps\maplecw-crash-1096760-c0000374-1.dmp`, **1 010 MB**, written by
  the hook in **878 ms**, exception `0xC0000374` = `STATUS_HEAP_CORRUPTION` at
  `0x7ffca83af509` in ntdll. The client died at **596 s** of life, which extends the old
  193-482 s band rather than fitting it.
  The hook's own heuristic stack scan produced exactly one client frame, `0x14019b58e` - a
  lead and nothing more. **The real stack is in the dump**, and reading it needs a debugger:
  `tools/analyse-dump.ps1` finds the file and prints the WinDbg commands, because only the
  Store build of WinDbg is on this machine and it has no scriptable console.
  **Page heap is still off**, so `!heap -p -a` cannot print allocation and free stacks. `kb`
  and `!heap -s` are what carry the weight until it is enabled.

Also confirmed the same run, all of them first sightings on a screen:

| | |
|---|---|
| **create on the second login** | *"Done, no issues"* |
| **the client refuses to forfeit a completed quest** | so the client-side half of the farming loop was never open; the three server-side holes were |
| **consumables, with the cap** | *"hurt down to 136, it recovered to 146, the stack went from 2 to 1"*. The log says `+11 hp (now 146/146)` - a **100 HP potion capped at the 11 that were missing**, which is the cap working, not a 10-point potion |
| **Sera's idle chatter** | *"seem okay now"* - the byte revert holds |
| **the mob-damage override** | *"the snail hit me for 1 with the number on top of my head, but I actually took 3 damage according to the HP bar"* - exactly the predicted outcome |
| **the damage model, at a second stat point** | 14 swings at STR 35 with the same axe: **17..31**. The model spans **15.5..32.6** across the three attack actions (swing 2.4, mixed 1.8, stab 1.2 at `incWAT` 17), snail PDD is **0** and it is level 1 so neither defence nor level gap applies. Fits, and 14 samples would not be expected to reach either extreme |

#### The NPC fade: `!npcecho` says the creation packet is NOT the difference

The owner: *"with !npcecho, Heena still faded in."* The command sent template 1 (Heena) and
template 2 (Sera) on map 1 as **`0x0451` NpcChangeController** - the other creation packet,
the one mobs get and NPCs never had - 70 px to the side.

**The copies faded too.** That kills both standing theories at once: the creation route is not
what makes mobs pop, and the *timing* theory dies with it, because these arrived minutes after
field entry rather than during it. What is left is that the client fades an NPC because it is
an NPC, in its own rendering path, with no field of either packet to change it - which is what
two static passes concluded and which I was wrong to restate as "the server has no lever"
before there was a measurement. Now there is one.

**Confirmed, and the question is closed.** The owner: *"The copy that was spawned in additionally
faded in as well after !npcecho was executed. The original Heena stayed on the screen."* So it
was the new object, created through the packet mobs get, arriving minutes after field entry -
and it still faded.

**And the echo could never have answered it.** `0x044F` and `0x0451` **run the same decoder
body**, `FUN_141e36b20` - the pool's two creation cases differ only in the state byte they set
before calling it. Comparing them was comparing a thing with itself, and I designed that test
without noticing. The fade was always going to survive it.

The owner asked again - *"Is there no other way that NPC can be spawned on the client side?"* - and
the enumeration this time is the pool's own opcode table rather than a neighbourhood:

| | |
|---|---|
| `0x044F` | NpcEnterField - creates |
| `0x0450` | NpcLeaveField |
| `0x0451` | NpcChangeController - creates, same body |
| **`0x0452`** | **`u32 v`, then `DAT_143ad2d30 = (v != 0)`, then it walks every NPC** |
| `0x0453..0x0466` | a 20-case per-NPC command block |
| `0x0467` | template preload list |
| `0x00BE` | a "limited NPC" id list, arriving through the *channel* dispatcher |

So **no, there is no third way to spawn one** - that part is now a real enumeration. But
`0x0452` is not a spawn and it is the thing that matters.

#### `0x0452` is the appear-effect switch, and it is the first server lever found on this

Inside `FUN_141e36b20` - the body **both** creation packets share - is a block gated on
`DAT_143ad2d30 == 0`: **[L]**

```text
FUN_14019b780(&DAT_143ad68a0, 0x90)   allocate a 0x90-byte object from a pool
FUN_140d13c80(obj, npc)               construct it against this NPC
FUN_142df7280(DAT_143ac18d8) -> +0xc  a clock value
FUN_140d13cc0(obj, that)              stamp it
npc[0xaf] = obj ; FUN_140d13270(obj)  hang it on the NPC and start it
```

A per-object, timestamped, started-on-creation thing is an animation, and it is built **only
while that global is zero**. `0x0452` sets the global - and its two branches are what make
this more than a guess, because they are not symmetrical: **`v == 0` runs the identical
allocate/construct/stamp sequence on every NPC already in the pool, and `v != 0` calls
`FUN_141e64690` to tear it down.** One packet creates and destroys exactly the thing creation
creates. **[L]** for both branches, read from `tools/listing.py` on `FUN_141e76c60`; **[D]**
for "that object is the appear animation".

`research/npc-spawn.md` §3.1 called `0x0452` *"a global show/hide toggle"* from a quick read
and moved on. It had the right packet in the table the whole time.

Wired as **`!npcfx on|off`**, deliberately not into field entry, so one run compares faded
NPCs and popped ones on the same map: `!map 1`, watch them fade, `!npcfx off`, `!npcecho`.
The polarity is inverted on the wire and the builder takes a `bool` for that reason.

**And one thing nobody has ever answered**, flagged as unproven by `research/npc-fade.md`
itself: whether what the owner sees is a *fade* at all. A see-through NPC and one that is simply
absent-then-present look the same in a sentence and are different bugs.

Worth recording how that went, because it is not a clean win: two static passes had said the
server could not fix it, I over-generalised one of them into a claim they had not made, the owner
pushed back, and the measurement came out agreeing with the original conclusion. Getting a
measurement was still right - it is what turned a static inference into something known - but
the *reason* I asked for it was wrong.

#### The owner's new request, built and unseen: the blue recovery number

*"the idle recovery should pop up with a blue number of the recovery amount above the player's
head. I don't see that here, the HP bar just moves up without a number indication."*

The bar always moved, because `hp`/`mp` carry the new totals. Nothing drew a number because
`0x007C`'s **second optional trailer** was absent - `u8 flag`, then `u32 hpRecovery,
u32 mpRecovery`, which the client hands to
`FUN_140fd31f0(uiGlobal, hpRecovery, mpRecovery, oldHp, oldMp)`. That call takes a **UI**
global, and it is the argument `research/level-up.md` used to name those two fields in the
first place. `user-hit.md` §5.2 said the trailer "should stay `None`" - correct for a *hit*,
which draws its own number client-side, and this is the case the field exists for.

Sent by **idle regeneration only**, carrying the **amounts** rather than the new totals, and
absent when nothing moved so a full bar cannot draw "+0". **[L]** for the shape, **[D]** for
the hp/mp pairing.

**Potions deliberately do not send it**, and that correction is the owner's: *"Potion recovery
should not trigger the recovery number, that's only for idle regeneration standing or sitting
in a chair in the Set-up tab or sitting on a chair in a map."* It went into the consumable
path first on the reasoning that a potion recovers and the field is called recovery - which is
arguing from the encoding outwards. **The field is the regeneration indicator, and which
events may raise it is a property of the game that no byte layout can tell you.** A test now
asserts the absence, because "a recovery is a recovery" is a persuasive-sounding reason to put
it back.

**Chairs are the other case** the owner named - the Set-up tab chair and map chairs - and this
server has neither. When it grows them, that path sends this trailer too.

#### The floating damage number is a constant, and that is worth one measurement

The client claimed **1 on all 25 hits** in that run - not a distribution, a constant - while
the server computed 3 fifteen times and 4 ten times. `incoming_damage` already applies the
player's own defence, so 3-4 is a defended number.

A computed value would vary. A constant will not. So either the client's formula genuinely
yields 1 for a snail against this character, or **the field is a stub and the client never
computes mob damage at all** - in which case the floating number will read 1 for every mob
forever and no server change can move it, because `user-hit.md` §4.4 establishes it is drawn
at *send* time by `FUN_142771360`.

> **Read on: the second half of that sentence was wrong.** The field IS a stub - the Drake run
> of 2026-08-27 settled that, 224 captured hits across mobs rated 3 to 287 all reporting 1 -
> but "no server change can move it" did not follow. A *different* number can be drawn beside
> it. See the entry above.

**One hit from a much stronger mob discriminates.** If the number moves off 1, the client
computes and our formula is the thing that disagrees. If it stays 1, it is a stub.

#### PROVEN ON THE WIRE by the 13:48 run of 2026-08-21, but never reported

That run was read properly afterwards and it contains far more than the one thing that was
said about it at the time. **These all worked**, and `world.log` has the whole exchange:

* **Roger's quest, the entire chain.** The 6-byte yes/no accept parsed (`000000001001`),
  quest 1002 was stored, the apple went into Use slot 1, `0x007C` set HP to **25/130**, and
  the grey item line went out as `0x02D1` effect 8. Then the apple was eaten - `0x010E`
  arrived - and the server answered with +30 HP, an `0x0070` that emptied the slot, the quest
  completed, and +3 EXP.
* **Consumables.** That apple *is* the consumable path, answered correctly end to end.
* **Quest completion and its EXP payout.**

The chain is stronger than a log usually is, because the client had to have **drawn** the
apple for the owner to double-click it: the `0x010E` names item 2010000 in Use slot 1, which
existed only because the accept was granted. So the accept, the grant and the HP change all
reached the screen.

**What is still unknown is what any of it LOOKED like** - the grey chat line, where the quest
EXP appeared, whether the fanfare drew anything. Those are questions about the screen and
they are what step 2 of the plan asks.

#### WIRED, and NOT yet seen on a screen

Everything here compiles, is tested and is connected. **None of it has been seen by the
client.** The heading exists separately because `STATUS.md` has twice called something done
while it was unwired. Five rows came off this table on 2026-08-21 once the 13:48 run was read
properly - Roger's quest, the fanfare, consumables, the quest payout and `!migsweep` - because
leaving them here would have spent a launch re-testing what a log already answered.

| | what to look for |
|---|---|
| **the Etc bag and mesos survive a relog** | they always persisted - nothing ever *sent* them. Field entry now re-sends `0x0070` per item plus `0x007C` for the balance |
| **items stack** | Garnet Ore into one slot, not three. `slotMax 0` means unspecified, not one |
| **idle regeneration** | +10 HP and MP every 10 s after 10 s of no movement, attack or damage |
| **the Tutorial Jr. Sentinel** | 100% Shellpiece, no mesos, no second drop |
| **`!setrates <exp> <meso> <drop>`** | all three on one anchor, one banner. Every rate command now refuses below 1x |
| **drops land on the floor** | a mob killed **on a slope or a step**, not on flat ground - flat looks identical before and after, which is why this went unnoticed. `world.log` names the foothold for any drop that moved |
| **giving up a quest works** | start 1001, press give up in the quest window, then click Sera again. Test on **1001, not 1000** - 1000 is completed by then and its give-up button can never send anything |
| **the channel migrate, end to end** | `0x001A` alone now, and the far end claims **by channel** when the hello names nobody. Only the opcode has been seen working; the arrival has not |
| **the hook writes its own crash dump** | any fault should now leave `dumps\maplecw-crash-<pid>-<code>-1.dmp` and two `CRASH DUMP:` lines in the hook log. Tested in-process, never yet against the client |

**One warning about testing the AP fix, because it would otherwise produce a false pass.**
Every `0x007C` clears the client's one-request latch, and **idle regeneration sends one every
ten seconds**. Field entry clears it too. So *click, wait, click* succeeds whether or not the
AP handler works at all - measured, not feared: in `world-20260821-001440.log` a regen
`0x007C` landed 11 s after an AP request and re-opened the window. The discriminating test is
**two clicks inside ~3 s with no map change**, and `grep 0x007C world.log` between the two
`<- 0x0138` lines is the free cross-check.

#### Solved 2026-08-21, late: a finished quest could be farmed

The owner: *"I was able to complete the Heena quest multiple times, this is not okay."* **Three
independent holes**, any one of which is enough on its own. All three are the same shape - the
store was the authority, answered correctly every time, and the caller asked and then did the
work anyway.

* **The turn-in paid out twice.** `apply_quest_completion_rewards` sat *outside* the match on
  `store::complete_quest`, which correctly returns `None` for a quest that is already
  complete. Quest 1001's `Act.1` is `exp 2`, so it was two experience per click, indefinitely.
* **The accept handed the items over twice.** Same shape: `grant_quest_start_items` and
  `apply_quest_hp` were outside the match on `start_quest`. Quest 1001's `Act.0` is Sera's
  Mirror and 1002's is Roger's apple plus an HP change, so both were farmable a click at a
  time.
* **Give-up deleted a completed row.** `forget_quest`'s `DELETE` had no state predicate, so
  forfeiting a *finished* quest put the character back to never having touched it - after
  which the other two were not even needed. The forfeit handler's own doc block already said
  this must not happen; that sentence was describing a wire flag, and nothing enforced the
  database half. **A comment describing a guarantee is not the guarantee.**

**Why it survived a test suite.** The turn-in test counted **fanfares**, and the fanfare was
the one effect that *was* correctly gated on `recorded`. It passed on every run while the
experience doubled beside it. The three new tests fail without the fixes - checked by
reverting each one rather than assumed.

Fixed by making every effect hang off the transition: `record_quest_complete` returns early
when nothing changed, `record_quest_start` pays `Act.0` only when the row is new and sends
nothing at all for a completed quest (re-sending `quest_accepted` would put it back in the
client's *started* list), and the completed-row guard now lives in `forget_quest` itself.

**Still worth watching on the next run:** the server is now authoritative, but nothing here
explains why the *client* offered the turn-in again. Either its journal is not being updated
in-session, or it simply lets you re-click and the server has always been the only guard. The
new log lines say which - a repeat is now silent where a genuine inconsistency still prints.

#### Solved 2026-08-21, and what each one cost to find

* **Roger's quest could not start, and the reason was an absence.** `q1002s` is **not in the
  client at all** - all 205 `.wz` archives and all 10021 images enumerated, 0 errors, and the
  name occurs exactly twice, both times as a *name*. `Data/Etc/Script/Script.wz` is a 63-byte
  header with **zero entries**. Corroborated by a second, non-overlapping instrument: the
  **Spanish-only** `CommandGuide.img` documents `loadscript`, `scriptrun`, `runlua` and
  `loadquest` - commands whose purpose is to let *the server* re-read script files. So the
  bodies are ours to author: `data/quest-scripts.txt`, an overlay in `questlines.txt`'s own
  format. Five of Roger's six lines are their own words from `String.wz`.
* **Drops fell through the floor.** The stagger in `drops_from_kill` moved each item sideways
  and **nothing ever moved it vertically**. Replaying today's placement over every mob spawn
  point: **29% land on no surface at all**, 8.2% by more than the 10 px the client's pick-up
  box forgives. A third of the 94089 floor segments are **sloped**, so reading `y1` instead of
  interpolating would have been wrong 27577 times - usually by less than an icon's height,
  which is the shape of a bug that reads as bad luck.
* **The jump height is [L], not a fan site.** `Map_000.wz` ships `Physics.img`:
  `jumpSpeed 555.0`, `gravityAcc 2000.0`, so `v^2/2a` = **77 px**. Only the closed-form apex
  is [I] - the client integrates per frame.
* **The quest-finish fanfare is `0x02D1` effect 15.** Pinned through the *only two* readers of
  `Sound/Game.img/QuestClear`; the one that is not the pet-skill notice is the packet-driven
  effect handler. `research/level-up.md` had documented only **one** of that handler's two
  switches on the effect byte, which is why the arm had never been seen.
* **The missing quest EXP line was never missing.** The run's own log has it -
  `03 01 0200000000000000 00 0000...` - **byte-for-byte the same shape** as the `+200` lines
  from kills in the same session. Quest 1001's `Act.1.exp` is literally **2**. Two independent
  passes reached that separately. *Nothing was built for this*, which is the right outcome.
* **"Adding stats doesn't change my damage" - it does, and the floor is what stands still.**
  Character 206's real kit (level 7, DEX 10, the `1312000` axe, `incWAT` **17**, no mastery)
  gives **15..20 at STR 5** and **16..27 at STR 30**. Six times the STR moves the ceiling by
  7.7 points and the floor by **0.6**, because `M = (mastery/10 + 0.1) * 0.8` is **0.08**
  without a mastery skill - the primary stat contributes a twelfth of its weight to the
  minimum and its full weight to the maximum. Pinned as a test in `damage.rs`.
  **And the measurement agrees**: four of the eight real hits in that run (21, 21, 24, 26)
  are **above the STR-5 ceiling of 20**, so the client is scaling with STR. What is not
  scaling is the bottom of the range, which is what a player watching small numbers sees.
  **`incWAT` multiplies the entire expression**, so a better weapon moves damage far more
  than stats do at this level.
* **Mobs hit for 1 because the CLIENT said 1, and the server had no opinion.**
  `on_user_hit` applied `hit.damage` verbatim. Template 2's `PADamage` is **3** and
  `damage::incoming_damage` over that character gives **3 or 4**.
  **The value 3 appears at no offset in any of the twelve `0x00E5` bodies** - checked as a
  `u32` across all 144 offsets - so this is not the server misreading a field that holds the
  real number elsewhere. That check mattered: `UserHit::damage`'s own doc says its offset is
  **undiscriminated**, because every capture carries 1 and 1 also sits at six other offsets.
  The server now computes its own from the client's own `Mob.wz` data and **logs both
  numbers**, so the next run says whether the floating number and the health bar disagree on
  screen.
* **The channel migrate is `0x001A`, and a sweep found it in one run.** `research/change-
  channel-reply.md` had everything about that packet except its opcode, which no scan could
  reach - `FUN_1415d8c00` has zero callers of every kind, zero 4-byte RVA references, and
  `.themida` has `SizeOfRawData = 0`. Ten candidates went out and the **hook log named the
  winner**, because it writes one dispatch line per inbound opcode on handler *return*:
  `0x0019` took 64 us and did nothing, `0x001A` took **354 ms** and the socket closed.
* **And the same run showed why the migration still failed.** A channel migrate carries **no
  character id** - seven bytes, `ok`/`ip`/`port` - so the client's `0x007D` on the new channel
  reported id **32513**, which is `01 7f 00 00` read straight back out of our own body.
  Channel 1 refused it, answered with the **MINIMAL SetField** (whose own doc says it will not
  put a character on a map), and the client faulted 3.2 s later. A channel migration is now
  claimed **by channel** when the hello names nobody, and refuses when more than one is
  pending rather than guessing.
* **Answering `0x00D2` with `0x0011` was worse than not answering.** `0x0011` is a
  login-stage opcode below the channel switch's `0x70` floor, so it could never dispatch -
  and `0x00D2` **latches on send**, like `0x0107`, `0x010E` and both AP requests. The owner: *"the
  transfer did not go through, but I lost all ability to attack once the attempt was made."*
  Both halves are that one fact.
* **Roger's Accept was being dropped by the parser.** The dialogue was right all along - the
  overlay loaded, the Accept box drew - and `parse_script_reply` read a `u32 echo` and a
  string unconditionally. **A yes/no box replies with SIX bytes** (`handle, type, action`) and
  echoes nothing, so the reader errored, `on_script_reply` saw `None`, and the accept vanished.
  No HP drop, no apple, and clicking Roger again replayed the opening. The module already
  documented the *outbound* half of that rule and the inbound parser did not mirror it.
* **"Create a character" was dead after any Log Out, and the delete was a red herring.** The
  client's handshake calls `FUN_140c9e8a0`, which stores plaintext **0** into the flag gating
  the button, on **every** success - and our `create=on` patch latched on a static bool and
  set it once per *launch*. `login.log` had two `0x0010`s; the hook log had one
  `called FUN_140c9e230`. So creation worked on the first login of a launch and nowhere else.
  Now re-armed per login result, and a re-arm prints.
* **`0x009E` is the client's "I could not handle this packet" report** - a gift, like
  `0x025F` for drops. Its body carries the offending **opcode and body verbatim**. It appears
  in no other run in this repo, which is what made it a discriminator rather than noise.
* **The crash-dump instrument could never have worked, and it is now replaced.** WER was
  misconfigured once and was fixed; that was not the reason. Settled at **no cost in client
  runs** by a decoy: a program that does nothing but dereference null, **named
  `MapleStory.exe`** because LocalDumps keys match on the base name. It wrote a **9.4 MB dump
  into `dumps\`**. The real client raised the *same* code, `0xC0000005`, at **13:49:56 - 88
  minutes after WER was switched on**, which the registry key's own last-write time
  (`12:21:01`) proves - and produced nothing. Same machine, same hour, same executable name,
  same exception; every variable held but one. **The client ships its own crash reporting and
  never reaches `WerFault`.** `CrashReportClient.exe` sits beside it and it already uploads
  its own call stack in `0x008F`/`0x0090`.
  So the dump now comes from **the hook's vectored handler, which was already catching that
  exact fault and only logging it** - it is what writes the `CLIENT FAULT` line. First-chance,
  so the dump is taken at the faulting instruction rather than after the client has unwound,
  which for `0xC000_0374` is the difference between a usable heap and none.
  `crates/grap-stub/src/minidump.rs`, tested end to end - the test writes a real `MDMP` and
  reads it back, because a dump writer that has never written a dump is exactly the
  instrument this repo keeps getting caught by.
  **And the struct it needed is `packed(4)`, not naturally aligned.** `minidumpapiset.h`
  wraps every `MINIDUMP_*` in `<pshpack4.h>`, so `MINIDUMP_EXCEPTION_INFORMATION` is 16 bytes
  with the pointer at offset 4. With `repr(C)` it is 24 with the pointer at 8, and
  `MiniDumpWriteDump` returns `ERROR_NOACCESS` **whatever the contents are** - which is what
  eventually pointed at layout rather than data. A test asserting 24/8 sat beside it, passing.
* **The hook log is archived now instead of deleted.** `world.log` was being moved into
  `previous-runs/` while `client-patched\maplecw-hook.log` was deleted at the same moment.
  That is precisely the asymmetry that makes `CLAUDE.md`'s "count the same event in two logs"
  impossible for any run but the current one - and three of this project's answers came from
  exactly that comparison. Both halves now land in the repo's `previous-runs/`.
* **"The server cannot fix the NPC fade" was my over-generalisation, and the owner caught it.** Two agents proved a real negative - no field of `0x044F` controls it - and I restated that as *the server has no lever*, which does not follow. The owner: *"You shouldn't need to patch the client. Are there no way for the server to send the NPC data to the client so that it appears instantly?"* There is a second creation packet, `0x0451`, and it sets a different state byte. **Two corrections to this entry, both 2026-08-22.** *"It is the one mobs get"* is wrong - mobs get `0x03C6`/`0x03D2`, and `0x0451` is an NPC packet. And the distinction turned out not to matter at all: `0x044F` and `0x0451` **call the same decoder body**, `FUN_141e36b20`, so the test built on this reasoning could only ever have reported "no difference". The pushback was still right and the enumeration it forced still found `0x0452`; the specific reason given here was not.
* **The dump instrument was configured and switched off at the master switch.** The owner,
  reasonably, believed crash dumps were enabled - `HKLM\...\Windows Error Reporting\
  LocalDumps\MapleStory.exe` points `DumpFolder` at the repo's `dumps\`, `DumpType` 2 (full),
  `DumpCount` 2. All correct. But
  **`HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\Disabled = 1`** - WER is off
  machine-wide, so `WerFault` never runs and never honours LocalDumps. **Five crashes since
  the folder was pointed at the repo on 2026-08-20 23:24 have produced zero dumps**, and the
  newest file anywhere on the machine is from 2026-08-19.
  This is `CLAUDE.md`'s own rule biting: an instrument that *looks* armed and is not. Page
  heap is separately absent from IFEO, which the 418 MB peak working set independently
  confirms - a page-heap run peaks near 990 MB.
* **A byte I swapped killed the client, and the comparison is the cleanest this project has
  produced.** A static pass read `0x044F` byte 20 as the facing bool and byte 21 as the
  animation action, both **[L]** off the listing, so they were swapped and byte 21 given the
  constructor's default of `0`. The next run died on Sera's first idle line. Two `0x044F`
  bodies for them on map 1, **differing in exactly those two bytes and identical everywhere
  else**: `...00 01 0800...` chattered lines 1, 2 and 3 over 23 s; `...01 00 0800...` produced
  `0x009E`, two C++ throws, `0xC0000005`, and **no dispatch line for the `0x0453` at all**.
  The chat packet is innocent - `e9030000ff0000000000` is byte-for-byte one that had worked
  11 s earlier on map 40. **Reverted to the bytes that are measured to work.** The attribution
  may still be right; the conclusion drawn from it was not.
* **`0x009E` is the client's "I could not handle this packet" report** - a gift, like
  `0x025F` for drops. Its body carries the offending **opcode and body verbatim**
  (`...5304 e9030000ff0000000000 5304`). It appears in **no other run in this repo**, which is
  what made it a discriminator rather than noise.
* **Consumables did nothing because nothing answered `0x010E`.** The owner: *"I tried to consume
  Red Potion, but it did not recover 100 HP."* The request was on the wire and logged
  `UNKNOWN`: `f7e1140f 0100 80841e00 01000000` - tick, slot 1, item **2000000**, and both the
  id and the slot check out against state this server had written. There was exactly **one**
  in the run despite the owner using more than one item, which is the request-latch signature
  again.
* **What a potion restores is in `spec`, not `info`** - which is why it had never been dumped.
  Every earlier pass read the `info` child. Red Potion `spec/hp` is 100 and Roger's Apple is
  30, matching its own tooltip. `hp`/`mp` are flat and `hpR`/`mpR` are **percentages of the
  maximum**; both units exist in this client's data and some items carry both.
* **Mode 1 of `0x0070` is `UpdateQuantity`.** Drinking one of two potions cannot use mode 3,
  which removes the **whole slot** - the client would empty it while the server still held
  one. `take_quest_item` sends mode 3 for a partial take too, and is correct there only by
  accident: quest 1001 takes one out of a stack of one.
* **AP: the roadmap named half the opcode.** `0x0139` is the **bulk** request; a plain `+`
  click sends **`0x0138`**, a different opcode with a different body - the stat window's
  handler is a flat chain of twelve name comparisons, `"strup"` against `"strupall"`, calling
  two different builders. Answering only `0x0139` still looks broken to anyone using the
  button. The six mask bits are now **[L] by name** from this client's own string ids
  (`SID_MSCW_STAT_STR` and its five siblings), which upgrades six constants in
  `crates/net/src/stats.rs` that were `[I]` from the reference version. Two traps in that
  table: the button says "HP" but the bit is **max** HP, and the widths differ *inside one
  reply* - u16 for the four stats, u32 for max HP/MP, in a body with no resync point.
* **Job advancement: the packet was found before goal E was written.** It is `0x007C` mask
  bit 5, already decoded and already in `stats.rs`; nobody had joined the two up. **The
  client plays the fanfare itself** from that packet, and `JobChanged` *is* one of
  `BasicEff.img`'s 40 nodes - so unlike the quest-clear case, expect sound **and** picture.
  **Do not also send `0x02D1`:** effect 14 reads two `u16`s where effect 15 reads none, so
  reaching for `user_effect_local(14)` by analogy would have shipped a 1-byte body to a
  5-byte read - the short-packet mistake that has killed this client twice.
* **And the first job advancement has no quest at all.** Enumerated rather than searched: the
  whole key space of `Act` and `Check` across all 322 quests is 17 and 32 shapes, and **there
  is no `Act.<n>.job` key anywhere**. `Act.<n>.item.<n>.job` exists 57 times and is a *reward
  filter* - a search for "job" would have matched it and returned a confident wrong yes. The
  `Test`/`Proof of Qualification` quests are the **second** advancement at level 30
  (`Check.0.job` requires 100/400 already, `Check.0.lvmin` is 30, and quest 20001 says "2nd
  job advancement" in words).
* **Change Channel is not a stage case at all.** The migrate reply is `FUN_1415d8c00`, a
  **socket-level** handler dispatched from the Themida VM - established by a fully enumerated
  funnel, not an absence: the connect has exactly one caller, which has exactly two, one of
  which is the login-stage `0x0011` we already send. The body is measured - `u8 ok`,
  `u32 ip` in **network** order, `u16 port` **little-endian** because the client `htons`es it
  - with the read count cross-checked by two instruments that agree exactly. **The opcode is
  the one thing that cannot be read statically** (zero callers, zero RVA references,
  `.themida` `SizeOfRawData = 0`), so `!migsweep` exists to settle it in one run.
* **The quest forfeit was in an opcode we already parse.** `0x0151` **action 3**, a **5-byte**
  body - and `parse_quest_request` needs a 9-byte head, so it returned `None`, and `None` was
  silence. Two captures show the packet arriving and nothing coming back. `0x01ED` and
  `0x01A5` both went unanswered four times in the same run and both looked like better
  candidates; `0x01ED` was ruled out by finding **its two bodies byte-for-byte in a capture
  from 2026-08-19 that contains no quest traffic at all**.
* **The AP latch has a confound, and finding it saved a run.** Every `0x007C` clears
  `ctx+0x2330`, and idle regeneration sends one every ten seconds - so *click, wait, click*
  would have passed even against a completely broken AP handler. The agent that found it also
  **weakened its own earlier claim**: the latch explains some of the three missing clicks,
  not provably all three, because one of the two captures had its latch cleared by regen
  11 s in.
* **The EXP curve was checked against the client at last, and it agrees** - 99 of 99 levels,
  zero disagreements, and the file now runs to 119 because the client's table has 100..119
  and the guide stopped at 100. It cost **no client run**: the dump was already sitting in a
  fixture. What it did cost was fixing `tools/decode_dump.py`, whose off-by-one label made the
  first comparison say all 98 levels disagreed. See "Instruments that have lied".
* **Equipping over a worn item now swaps** - and the client does the swap **itself**. Mode 2
  of `FUN_142d51930` is an unconditional two-way exchange (`142d52c13` writes the displaced
  item into `oldPos`), so the reply stays **one entry, 14 bytes**. A second entry would not be
  redundant, it would be *wrong*: it would move the item that had just arrived.

#### Solved 2026-08-20, and what each one cost to find

* **Sera recited Heena's tutorial.** Quest 1000 has no `Say.1`, so the completion fell back to
  `Say.0` - the opening of the quest being finished. A completion now chains via
  `Act.1.nextQuest` instead.
* **Sera handed over nothing.** `Act.0.item` was never read. Now it is, and `Act.1.item` too.
* **Etc items and mesos "did not persist".** They persisted perfectly; the client was never
  told. Both reasons were already written down in this repo and neither had been noticed.
* **Garnet Ore would not stack.** `ShopTable::max_per_purchase` already had the right rule and
  **three callers had each hand-rolled `slot_max.max(1)` instead of calling it.**
* **A full Equip tab was blamed for blocking Etc pick-ups** and was innocent - a test proves
  it. The symptom was the missing restore above.

#### The 2026-08-22 morning run, and the three things it changed

The owner's report, in their words: *"Fresh spawns look correct now."* Then three problems.

**1. A full equip bag stopped every pick-up, for everything.** *"when my equip slots are full,
I should be able to get more items in my other inventory where I still have slots, such as Use,
ETC, or mesos. Currently I'm not able to do that."*

The full bag **triggered** it and was not what blocked them. `on_pick_up`'s bag-refusal branch
was the one exit from that handler that sent a chat line and **no `0x0070`** - and the client
latches `player+0x2330` when it asks, with only an inbound `0x0070` clearing it. The file's own
doc comment two screens below says exactly that rule.

Measured, not inferred: `world.log` has **six** `0x032C` pick-up requests, the sixth answered
`"inventory 1 is full (30 slots)"` with a `0x00BB` alone, and then **zero** further requests in
the following four minutes across **56** drops. The client had stopped asking. Fixed, and the
test was checked against the unfixed code first - it fails with `Replies were: ["0x00BB"]`,
which is the log line verbatim.

**2. Storage moved mesos but not items, and did not take the fee it advertised.** *"I tried to
store an item with Mr. Kim. The item did not move to storage, and it did not charge the 100
meso fee that it said it was going to charge."* One missing arm, and the fee text is the
**client's own**, read out of `Npc.wz` before anything is sent - so an unbuilt deposit reads as
a promise broken. `PutIn` and `TakeOut` are wired now, the fee hangs off the store's `Ok` and
nothing else, and the take-out index is resolved through the same function that orders the
wire so the two cannot drift.

**3. Selecting `GoodTest` killed the client instantly - and that is not yet the same claim as
"map 10 is fatal".** The dump is the third of the `0xC0000374` family and is written up in
`research/heap-third-dump.md`. It died **inside** the `0x01A0` SetField handler (no dispatch
line, and the hook writes those on return), 0.33 s after the packet. But the client was already
306 s old and carrying one damaged pool slot, and a character-select round trip is the biggest
resource release in a session. **One login settles it**: log in as `GoodTest` first, at ~40 s of
client life. It is step 1 of the plan and the order is the whole experiment.

#### The 2026-08-22 midday run: three confirmations and one new crash family

The owner: *"1. Logging in as GoodTest seems fine. 2. Inventories are now working correctly, when
my Equip tab is full, I can continue to pick up items that belong to other slots. ... 4. Blue
recovery number is now good, I do see 10 in blue above the character."*

* **Map 10 is not fatal.** Yesterday's experiment - log in as `GoodTest` first, at ~40 s of
  client life - came back clean in one login. The instant crash the day before was the
  session, not the map, exactly as the alternative reading predicted. Worth noting because
  the same shape of question is open again below and it resolved the *other* way this time.
* **The pick-up latch is fixed on screen.** A full Equip tab now refuses equips and nothing
  else, which is what the owner asked for in the sentence that opened it.
* **The blue recovery number draws.** `0x02D1` effect `0x41` renders "+10 in blue above the
  character", which closes a chain that had one unmeasured link in it (the suppression gate
  at `0x14278bd75`) and two runs of nobody being sure.
* **Storage items were not tested** - step 3 went unreported. It is step 3 again.

**And Nimble Feet was still dead, for the reason the whole file keeps recording.** The owner:
*"Nimble Feet still does not give me a buff despite me activating the skill."* `0x013C` had
been arriving all along - one at 13:00:49, 51 bytes, `skillId 1002 level 3`, whose two
checksum dwords match the ones `research/buffs.md` §3 read off a **different** session - and
it was logged as `UNKNOWN` and dropped. `research/buffs.md` had the whole packet written out
and nothing called it. Built now: `crates/net/src/buff.rs`, `crates/world/src/session/buff.rs`,
and a `!buff` GM command that sends the same bytes with the skill check, the MP and the
180-second cooldown all out of the way.

**The teleport crash is a SECOND crash family, not another heap one.**
`research/henesys-park-null-deref.md`. The owner said *"GoodTest teleporting to map 10001050
crashed again"*, and "again" is the part that needed checking: the code is `0xC0000005`, an
**access violation reading `[0 + 0x3530]`** - a null object pointer - where the other three
are the allocator refusing a bad free. The damaged pool slot was present in this dump too
**and was never touched**, which is the first time the two have been shown to be separable.
Whether the map itself is fatal or the session was is the same one-command experiment as
yesterday, and it is step 1 of the plan.

#### The Cash Shop latch is cleared, and `0x00D5` is closed as a mechanism

2026-08-22, the owner: *"I see the message 'Cash Shop is not available on this server'."*

The falsifiable half held. Three clicks produced **three** `0x00D5` requests and three `0x0070`
replies, and the peek read `[ctx+0x2330]` as **0, 0, 0** where the unanswered run read
**0, 1, 1**. So `inventory_rejected()` really does clear that latch, and the button has stopped
being a once-per-session button. `research/cash-shop.md` part seven.

**Still unseen**: the pick-up implication. That latch gates the pick-up sweep too, so before
this a player who clicked Cash Shop was killing every later pick-up in the session. The 29-second
run had no drops, so that is reasoned rather than measured - one kill and one step to confirm.

**Unblocked, not built**: the client asks and waits, so the migrate architecture the owner proposed
in their first message is now the next real piece of work rather than a maybe. The pieces already
here are the `0x0011` builder, seed minting, claim-by-channel and a second listener; the pieces
missing are `SetCashShop`'s body (a candidate range, not a read), a cash inventory, a wallet and
a purchase flow.

#### RETRACTION: the Cash Shop was sending all along, and it is `0x00D5`

2026-08-22, the owner: *"I logged in, clicked Cash Shop, then counted to 3, then clicked Cash Shop,
then counted to 3, then clicked Cash Shop, then exited the game."*

**This file said three times that the Cash Shop click sent nothing. It was wrong.** `0x00D5`
went out on the first click of that session and of the two before it.

The peek on the latch, three clicks 5.6 s apart so the rate limiter is irrelevant:
`[ctx+0x2330]` reads **0**, then **1**, then **1**. The button fires once, sets the
exclusive-request latch, and waits. At the same millisecond as click 1, `world.log` carries
`0x00D5`, five bytes - which is exactly the opcode `msexe-send-opcodes.txt` records for the
`COutPacket` inside `FUN_142caee70`, the function the watch fired on.

**How the negative survived:** `0x00D5` arrives inside a burst with `0x0420`..`0x0426`, and
that burst lands near the end of a session, so the whole thing was filed as "shutdown
telemetry" **as a unit** without separating the opcodes. One grep over the archives breaks it:
`0x0420` appears in five runs where nobody touched the button; `0x00D5` appears in **exactly
the two runs with a click**, once each. That is "enumerate before you filter" with a new face -
the filter was *when it arrived*, not *which opcode it was*. The owner said *"the opcode is most
likely not handled"* in their first message and was right.

**Shipped:** `0x00D5` is parsed and answered with the `0x0070` that clears `[ctx+0x2330]`,
plus a line saying the Cash Shop is unavailable. Not a cash shop - but that latch also gates
the **pick-up sweep** (`research/pick-up-latch.md` §2.2.2), so a player who clicked Cash Shop
was leaving it set for the rest of the session. Falsifiable next run: **one `0x00D5` per click**
instead of one per session.

And `research/cash-shop.md` §4 - the migrate to a dedicated cash shop server, which is what
The owner proposed on day one - is now the live next step rather than a footnote.

#### The Cash Shop sender IS entered, and its six exits are named

2026-08-22, the owner: *"as a test, I took out a 'Sword' from the storage before spamming the cash
shop button and then leaving the game to try to get you something usable."*

It was. Both watches fired and **the sender's twenty timestamps are the dispatcher's first
twenty, to the millisecond** - so every click runs dispatcher -> `FUN_142caee70`, one for one,
and **the sender is what refuses**. The storage-take-out also landed: `took item 1302000 out of
storage slot 10`, which closes the last untested half of that window.

`FUN_142caee70` has **six exits before the packet**, and the three message ids are decrypted
out of the client's own table:

* *"You cannot go into the cash shop. Please try again later."* (`0x091E`)
* *"You must close the window before using the Cash Shop or changing channels."* (`0x0486`)
* *"You can't do this while taking the quiz."* (`0x0AE2`)

and three that **return silently**, which is the symptom: `[ctx+0x2338]`, **`[ctx+0x2330]`**,
and `tick - [ctx+0x2334] < 0x1f4`.

**`[ctx+0x2330]` is the exclusive-request latch** - the same field `research/pick-up-latch.md`
is about, the one an inbound `0x0070` clears, and the one whose stuck state killed every
pick-up earlier the same day. A stuck latch would kill this button silently, and that would be
**our** bug.

**`0x1f4` is 500 ms**, and the clicks came at ~150 ms - so every click after the first died on
the rate limiter alone. Spamming was the worst way to test it, and nobody knew.

Next run: three clicks three seconds apart, on a fresh login before opening anything, then
again after using storage. The watch peeks `+0x2330` directly. `research/cash-shop.md` part
five.

#### The Cash Shop button is live, both of my readings are dead, and two watches are armed

> **PARTLY SUPERSEDED.** The watches were right and the reasoning about the gate was not:
> the sender IS entered and it sent `0x00D5`. See the RETRACTION above.

2026-08-22, the owner: *"The button can be highlighted, does depress when clicked, and does make a
click sound."*

So the control is live and something after it refuses. That kills both candidates: the global
the handler loads has **5459** references - it is the context singleton, and if it were null
all nineteen status-bar buttons would be dead - and the CashShop arm has no condition on it
at all.

The dispatcher is confirmed as the status bar by reading its arms rather than assuming:
`ChatLogMin, ChatLogMax, ChatPrev, ChatNext, ChatTargetSelect, CashShop, Menu, Shortcut,
Claim, Mailbox, Equip, Inven, Stat, StatUp, Skill, SkillUp, Key, QuickSlot, QuickSlotD`.
**`Inven` is in the same chain and Inven works**, which makes it a free positive control.

**The owner asked whether the server has to advertise the cash shop, and whether it needs an
address like a channel.** The first is plausible and has a precedent - the world list's
per-channel enable byte, which once emptied the Change Channel dialog when it was wrong. The
second contains a correction worth keeping: **channels are not advertised with an address.**
`world_list_entry` writes a name, a user count and four bytes per channel and no address at
all; the client learns one from the `0x0011` migrate reply *after* it asks. So an address
cannot be a precondition for asking, and neither can a flag the client never gets to check.

Two watches are armed in the default `-SetFieldProbe` set - `1411ab7b0` and `142caee70` -
and one run splits it three ways. `research/cash-shop.md` part three.

#### Storage is done bar Organize, and five Cash Shop clicks sent nothing

> **SUPERSEDED.** "Sent nothing" is wrong - `0x00D5` was in that log too. See the
> RETRACTION above. Organize and the storage half of this entry stand.

2026-08-22, the owner: *"The storage fees are working, but I tried hitting the 'Organize Item' 3
times, but it did not perform anything. I also clicked on the Cash Shop button 5 times before
exiting the game."*

**Storage items and the fee are confirmed** - ten deposits in one session, `mode 13` each
time, and the 100 mesos charged. That closes the last of the storage window bar one button.

**"Organize Item" did nothing because it was written to do nothing.** Mode 6 answered with
the unchanged box and the log line called it *"a legal no-op"*. Legal it was, and a no-op is
not what the button says it does - answering with the shape of a success while changing
nothing is the same failure the storage window itself started as. `Store::sort_storage`
repacks to slots 1..n, grouped by inventory type then item id, in one transaction that
**re-reads the box and refuses to commit if the count changed**. The order is a pure function
of the contents, so the second and third clicks cannot reshuffle - which matters, because the owner
clicked three times.

**Five deliberate Cash Shop clicks produced five packets, and every one is routine telemetry**
that arrives in sessions where nobody touches the button. Two sessions now with zero
cash-shop traffic.

And the reading has changed. `FUN_1411ab7b0` is a button-**name** dispatcher, and its CashShop
arm has **no condition at all**: match the name, load a global, tail-jump to the sender. So "a
gate in the handler" is now the *less* likely explanation, because reaching that function at
all would have produced a packet. **The button itself is the suspect**, which sharpens the one
thing only the owner can see: does it react to the click - depress, highlight, a sound?
`research/cash-shop.md` part two.

#### Buffs are done, and the Cash Shop click never became a packet

> **SUPERSEDED.** The click *did* become a packet - `0x00D5`. See the RETRACTION above.
> The buff half of this entry stands.

2026-08-22, the owner: *"I confirmed that the buff now works. Three Snails also works as intended
and dealt damage. I tried entering Cash Shop but was unfortunately not able to because the
opcode is most likely not handled. Transitioning to the cash shop is most likely similar to
transitioning to another channel, we probably need a dedicated cash shop server."*

**Buffs are closed, both directions.** One `0x013F` in, one `0x007E` out, **no retry loop** -
which is the client saying the answer was accepted. Exit code 0. Between this run and the last
two, that subsystem settled `0x007D`, the 124-byte mask and its big-endian-within-word bit
order, CTS bit 92 = Speed, the **`i16`** value width - unreadable statically, the deciding
constant is in Themida-packed `.data` - milliseconds, and `0x013F`'s layout. **Three Snails
works and deals damage.**

**The Cash Shop is not an unhandled opcode: no packet was sent at all.** `world.log` records
every inbound packet and the only unanswered ones are the telemetry set every session
produces. A cash-shop server built today would wait for a connection the client has no reason
to make.

It is also not missing art - `CashShopUI.img`, `CashShopPreview.img` and `button:CashShop` in
**both** status bars are present, unlike the classic shop's `UIWindow2.img`, which really was
cut. And the button's handler **does** reach two packet builders (`FUN_1411ab7b0`, 904 bytes,
both sends behind a conditional). So the client refused the click internally, which is the
shape of the create-character flag.

**What the screen did decides which of three jobs this is**, and only the owner can see it:
nothing at all (a gate), a dialog with words (a string, one command from the branch), or a
window that opens blank (the request is the missing part, and their migrate reading becomes the
thing to build). `research/cash-shop.md`.

#### The client never removes a temporary stat, and `0x013F` is how it asks

2026-08-22, the owner: *"the buff works, but after the expiry, the buff did not go away. (It just
kept flashing, but the temporary stats were still there) I also tried to pre-emptively kill
the buff by right clicking on the icon, it also did not dismiss the buff."*

**No crash** - the client hand-closed with exit code 1 - so taking `0x007E` off the expiry did
stop the deaths. It also stopped the buff ending, and that was a retraction waiting to happen.

The reasoning for removing it was that `0x007D`'s duration reaches the client's own `tExpire`,
so the client would drop the stat itself. It *knows* when the buff ends and **flashes the
icon**; it does not remove anything, and at the thirty-second mark it sent nothing at all.
`tExpire` drives the animation and nothing else. **Removal is the server's job on both paths.**
The plan named that outcome in advance, which is the only reason the wrong version was worth
one run.

Right-clicking the icon sends **`0x013F`**, and it sent **fourteen** identical 133-byte bodies
in three seconds - one every ~180 ms, a retry loop rather than fourteen clicks. One capture
pins its layout, because two constraints have to hold at once: the mask must be 124 bytes
**and** the single set bit must decode to a stat that was granted. Only a mask starting at
body offset **9** does both, landing on bit 92. So the body is `u32 skillId`, five bytes, then
the same CTS mask. `research/buffs-underflow.md` part three.

Both removal paths are built. A cancel for a bit the server is not holding is refused with a
line rather than honoured, and `bits_in_mask` is the exact inverse of `stat_mask` with a
round-trip test over all 992 bits.

#### Nimble Feet works, and one screen settled five unreadable things

2026-08-22, the owner: *"The buff works, but after the buff expired, the client crashed again."*

`0x007D` dispatched **and returned** - the hook writes those on return and it had never
appeared for this opcode before - and the icon, the countdown and the speed were all on
screen. That single packet settled, at once: the opcode; the 124-byte mask with big-endian
bits inside each little-endian word; **CTS bit 92 is Speed** (the character moved faster);
the value is an **`i16`**, which could not be read statically because the deciding constant
lives in Themida-packed `.data`; and the duration is **milliseconds**. It also showed that a
198-byte body is accepted without complaint, so this client does not check that a packet was
fully consumed - which turns the padding below from a hope into a supported choice.

**Then `0x007E` did the same thing one handler over**, thirty seconds later, and this time
the arithmetic is exact. `reads.py` on `FUN_142d56f80` finds **three reads after the mask**
that `research/buffs.md` §7.1 never lists, so the 127 bytes it specifies are `3 + 124`
consumed and then a `u8` with nothing left - the throw stack names `0x142d57322` and the u8
primitive's `cmp edi, 1 / jb` raise path. Minimum 129, or 133 if a gated `u32` fires.

Two things changed and only one is a length: `0x007E` is padded like its sibling **and the
natural expiry no longer sends it at all**. The client holds its own `tExpire` from
`0x007D`'s duration field, so both sides drop the stat on the same clock with no packet
crossing. `0x007E` now fires only from `!unbuff`, which is what dispel, death and logout will
need. `research/buffs-underflow.md`.

#### Nimble Feet crashed the client, and the crash proved the opcode

2026-08-22 evening. First send of the `0x007D` packet `research/buffs.md` §7.1 specifies, and
the client died with `0xE06D7363` - an **unhandled C++ throw**, not an access violation and
not the heap family.

**It threw because the body ran out.** The throw stack names `0x142d56911`, the instruction
after the `call` at `0x142d5690c`, which is the `u32` read four fields from the end of
`FUN_142d563d0`'s tail - and that primitive's own listing is `cmp edi, 4 / jb <raise>` where
`edi = length - position`. There is no dispatch line for `0x007D` in the hook log, and the
hook writes those on return.

**The crash is also the best news of the day**: the throw happened *inside* `FUN_142d563d0`,
so `0x007D` **is** TemporaryStatSet. That was `[D]` resting on a case table and two
neighbouring anchors; it is now `[L]` from a live capture.

**And one thing does not add up, which is written down rather than smoothed over.** Four
instruments - `reads.py` at depth 4, the raw primitive's listing, bit 92's own 87-line
decoder block, and an enumeration of all 476 bit tests - agree the handler consumes **at most
145** of the 152 bytes sent. Seven to spare. Something none of them can see takes the
difference, and re-running any of them is not a second opinion.

The tail is **64 zero bytes** now, and it is labelled as slack rather than a computed length
everywhere it appears. `!buff <skill> <level> <tail>` makes the number typeable so a session
that survives can bisect it in chat lines instead of one launch per attempt; 18 and below are
refused, because 18 has already killed a client once. `research/buffs-underflow.md`.

#### What to do next, in order — SUPERSEDED, the 2026-08-22 list

> **Do not work off this table.** It was rewritten 2026-08-22 after the Cash Shop run and eight
> of its eleven rows have since closed or been retracted. The live list is `START HERE` at the
> top of this file. What follows is annotated rather than deleted, because three of the closures
> are corrections and the reason each one was wrong is the part worth keeping.
>
> | row | what happened |
> |---|---|
> | 1 Cash Shop | **CLOSED 2026-08-25.** It opens, `0x01A3` confirmed on the wire; the currency is Leaf Points and a purchase completes |
> | 2 Henesys Park null deref | **RETRACTED.** The map is not fatal - a character stood there 52 s into a connection and the session ran another fifty. Both deaths blamed on it were at ~400 s and were **different exceptions**. It was the session, not the map |
> | 3 the classic shop counter | **BUILT since that table was written.** `net::classicshop` + `session/shop.rs`; `0x055D` appears 9 times in the archive, and the launcher's own closed-list says the counter draws and selling works |
> | 4 the heap wild write | **still open**, but the `-HeapFix` sentence in it is **retracted**: the patch armed, held, and the client died anyway at a second pooled free the route to which is fixed at compile time |
> | 8 job advancement, the conversation | **CLOSED 2026-08-31.** First job through the instructors; second and third built end to end and unseen |
> | 11 the NPC first draw | **the comparison itself was never a control.** The mobs it was measured against were not in the field-entry batch at all - they arrived from the respawn tick 7.16 s later, with nothing to be late against. Nobody has ever watched an NPC and a mob created at the same instant |
>
> Rows 5, 6, 7, 9 and 10 are **unrevisited** - `UNVERIFIED 2026-09-04`.

**Rewritten 2026-08-22, after the Cash Shop run.** Rows close fast at the moment - buffs both
directions, storage end to end including Organize, the pick-up latch, and the Cash Shop
button's latch have all closed since the last rewrite - and a list that still names finished
work is how a launch gets spent re-testing.

Two rows are **decoded and deliberately not built**, each with a byte-level body an
implementation can be diffed against rather than re-derived. One is **in progress**.

| # | do this | state |
|---|---|---|
| 1 | **The Cash Shop** | **BUILT, ENTIRELY UNCONFIRMED.** Entry is `0x01A3` on the same channel socket - four arms of one stage forwarder, **no migrate and no second server** - plus `0x05AD` for the balance, an empty `0x00D1` to leave, and a non-ejecting `0x05AE` refusal for every in-shop click. `crates/store/src/cash.rs` holds a per-account NX wallet and a locker; `world::commodity` prices 159 sale rows by SN; `!nx`, `!buy` and `!locker` drive the whole transaction from the field. **`0x01A3` is derived, not read** - `0x01A0` is the only stage packet ever confirmed on a wire. Next: does the window draw, and what does a real `0x03E1` look like. `research/cash-shop-stage.md`, and `research/cash-shop.md` parts one to seven |
| 2 | **The Henesys Park null dereference** | **A different family from the heap crash.** `0xC0000005` reading `[0 + 0x3530]`, 328 ms into the `0x01A0` handler, no dispatch line - and the damaged pool slot in that dump was a **bystander**. Whether map `10001050` is fatal or the 389-second session was is **still not established**, and one GM command settles it. `tools/check_map_resources.py` has already ruled out a missing tile, object, background or map mark, with a positive control. `research/henesys-park-null-deref.md` §3 |
| 3 | **The classic shop counter** | **Decoded, not built**, and the price is settled: **`row+0x38`, a u64**, from three independent sites. The row is **42 reads**, not thirteen fields. Request opcode is **`0x00F5`**, not `0x0104`. **Three traps**: `row+0xa4` is a FILETIME with no sentinel and `0` hides every row; `row+0x10c = 0` fails purchases silently; a dropped row desynchronises the byte stream. `research/classic-shop-rows.md` |
| 4 | **The heap wild write** | **Six dumps** of that family now. The value is the identical `0x0000000100000020` **nine for nine**, the class is `0x20` nine for nine (**0 of 579 008** elsewhere, out of 962 112 enumerated). **The one-per-250-s rate is FALSIFIED**: 1 046 s produced 2, not 4, and damage tracks session *age* (rank corr 0.80) rather than map loads (0.05). A damaged slot was found **on the free list**, which kills "the object underruns its own buffer" and leaves overrun-from-predecessor and stale-pointer. The **writer is still not found**. `-HeapFix` **would have prevented the 2026-08-27 fault** - `0x14019b58e` is the return address of the `HeapFree` call at `+0xac` of the function it patches at `+0x24` - and **has never been switched on in a crashed run**. `research/heap-corruption-2026-08-27.md` |
| 5 | **The pick-up after a Cash Shop click** | Ten seconds, no setup. `[ctx+0x2330]` gates the pick-up sweep as well, so before the fix one click killed every later pick-up in the session. That implication is **reasoned, not seen** - the 29-second run had no drops in it |
| 6 | **The two `0x00DF` header fields** | The damage formula is decoded and cannot be *used* without the **action** and the **skill id**, neither parsed out of the attack header. `research/damage-formula.md` |
| 7 | **The grey item line** | The packet goes out - `0x02D1` effect 8, category 6 - on every successful pick-up, and nobody has reported what it looks like. One glance, no setup |
| 8 | **Job advancement, the conversation** | The *packet* is done and `!job` tests it; the NPC path is not. Instructors are **not in the towns** - 511 on map 10004003, 313 on 10002003, 221 on 10001051, 411 on 10003003, pinned by a test |
| 9 | **`tools/dump_equips.py` hard-codes its columns** | Its docstring claims the set is enumerated and it is not. All 1760 equip images carry `attackSpeed` and `attack` on 203 weapons each, neither in `equips.txt` |
| 10 | **The other script quests** | 1002 and the four `Proof of Qualification` closes are authored. The `Test of Qualification` four are the **second** advancement at level 30 |
| 11 | **The NPC first draw** | Every server-side cause is eliminated: not the creation packet, not the appear-effect object, not a preload, not the timing - and it is **not a fade**, it is a late first draw. What has never existed is a **control**: one mob and one NPC created in the same batch on a settled map. `research/npc-preload.md` §8 |

**Two refusal paths still send a packet the client cannot dispatch.** `change_channel_refused`
answers with `0x0011`, and so does the no-such-channel case - both undispatchable on a channel
socket, so neither clears the `0x00D2` latch. Nothing decoded can. Said out loud rather than
left to be rediscovered.

**Two contradictions still open**, of three found 2026-08-21 - the third is struck through
below. The heading has now been wrong twice: it said "two" over three bullets for a day, then
"none adjudicated" over a bullet that had been adjudicated. **A count in a heading drifts from
the list under it unless something makes them agree**, which is the same failure this file
keeps catching elsewhere, and it is why the resolved bullet is struck through rather than
deleted.

* **The `white` EXP byte.** `STATUS.md` above says `white = 1` was *confirmed on screen*;
  `research/exp-sharing.md` marks it **[I]**. Only `white = 0` -> yellow is [L] (from the owner's
  screenshot). The server sends 1 and the screen has looked right, so this is a labelling
  fault rather than a behavioural one - but one of the two files is wrong.
* **`research/npc-chatter.md` §8.2 retracted `[npc+0x270]` on a bad scan**, and the retraction
  was the mistake. The decoder does write it - `141e36df4 41 89 84 24 70 02 00 00` is
  `mov [r12+0x270], eax`. That scan's positive control used `rsi` as a base, which needs no
  SIB byte, while `r12` always does, **so the control could not exercise the encoding that was
  actually there**. `opcode.rs`'s `ENABLED` label was right all along. This is the third time a
  carefully-hedged negative has been used to withdraw something correct.
* ~~**The `0x0070` read count.**~~ **ADJUDICATED 2026-08-22 in favour of 17.**
  `python tools/reads.py 0x142d51930` re-run today reports 17 - 14 direct, 3 through the
  `0x140303530` helper - and `reads.py` is the authority by construction: it carries the list
  of ten read primitives, a count this project has had wrong four times. `msexe-setfield.md`
  now says so at the point of the claim. Nobody has ever pointed at which site is the
  eighteenth, and the mode-2 path is unaffected either way.

**Dead assets, so nobody hunts for them:** `Sound/Game.img` contains `IncEXP` and `questCount`
and **neither name appears anywhere in the 76 MB executable, in either encoding** - while the
same scan finds `PickUpItem`, `Portal`, `QuestAlert`, `LevelUp` and `JobChanged`. There is no
EXP-gain sound in this client to send.

#### THE TEST PLAN

**It lives in `tools/test-server.ps1` now**, at the top of the file, so the steps and the thing
that launches them cannot drift apart. Run the launcher and read them there.

Two watches still need their own runs, neither combinable with the plan:
`-SetFieldProbe -UserState` for the user state machine, and `-SetFieldProbe -MobTargets`.

#### Change Channel: we answer with a LOGIN-stage opcode on a GAME connection - 2026-08-21

The owner changed channels and nothing happened. The capture says the request arrived, we answered
it, and the client ignored the answer:

```text
<- 0x00D2  19 byte body  01 2d2c010d 64000000 47580000 ...   (target channel 1, 0-based)
-> 0x0011  MIGRATE_COMMAND ... to channel 1 at 127.0.0.1:8486
   ...six seconds of ordinary mob traffic...
   ch0 connection closed
```

**Channel 1 was listening on 8486 and received no connection at all** - `world-ch1.log` is 23
lines of startup banner and nothing else.

**The body is not the problem.** Compared byte for byte against the `0x0011` the *login*
server sends, which demonstrably works, the two are structurally identical - same length,
same fields, differing only in the port (`2521` = 8485 vs `2621` = 8486, both correct) and the
single-use seed.

**The opcode is the problem.** `0x0011` is a **login-stage** opcode. The channel stage
dispatches through `FUN_142cbaa80`, whose labels are **`0x70..0x19f`** plus the outliers
`0x275` and `0x39a` - `research/msexe-gamestage-opcodes.md`. `0x0011` is **below the bottom of
that range**, so a channel connection cannot dispatch it at all. The client is not refusing
the migration; it never sees one.

That also explains the note this reply already carried: *"The reply SHAPE is inference; no
capture."* The shape was inferred from the login migrate, and so, silently, was the opcode.

**Next step:** find the channel stage's own migrate/change-channel reply. It is somewhere in
`0x70..0x19f`, and the two request/response pairs already identified in that switch (`0x162`
answering with `0x175`, `0x275` with `0x17e`) are the model for how to look - a case that
*sends* something is rare enough to enumerate.

#### The AP request is `0x0139`, and the body decodes - 2026-08-21

The owner tried allocating ability points three times over two runs and nothing happened. It was
not identified until they did a **bulk** assign, which put a distinctive payload on the wire:

```text
<- 0x0139  16 byte body  dfbdfe0c 01000000 40000000 1e000000
                         ^tick    ^count   ^0x40    ^30
```

`0x40` is the **STR** bit and `30` is the amount they typed. So the shape is
`u32 tick, u32 count, u32 statMask, u32 amount`, and it sits one slot below `0x013B`, the
skill-up request - which is exactly where a stat-up request belongs.

**Why it took three attempts to find.** The earlier hunts looked at the *unanswered* census
and `0x0139` never rose above one occurrence, because a single-point click and a bulk assign
do not produce the same traffic. What found it was searching the bodies for the **number they
typed** rather than ranking opcodes by count. Enumerating by frequency hid it; searching for
the payload did not.

**Not implemented.** The handler, the stat mask table and the `0x007C` reply that confirms the
new value are all still to write. This is now a small job rather than an open question.

#### Roger's quest needs a SCRIPT engine, which does not exist - 2026-08-21

The owner: *"Roger's Apple quest doesn't start as expected. All I see is 'Hey, nice weather isn't
it', which is just their normal text."*

**Quests in this client come in two kinds and only one is implemented.** [L], from the WZ:

```text
1002  Check  0.npc          3
1002  Check  0.startscript  q1002s
1002  Check  1.endscript    q1002e
1002  Say    1.stop.item.0  Eat the #r#t2010000##k I gave you...
```

Quest 1002 has **no `Say.0` at all**. Its opening is a *script*, `q1002s`, and the client
asks for it with `0x0151` **action 4** - `QUEST_ACTION_OPENING_SCRIPT`, which
`crates/net/src/script.rs` already names. The capture shows exactly that, three times over as
The owner retried: `04 ea030000 03000000`.

With no `Say.0` to play, the conversation falls through to the NPC's own idle line - which is
what was on screen. **Nothing is broken; a whole feature is missing.** The Say-tree quests
(1000, 1001) work because they *have* Say trees.

What a script engine needs, and it is not small: the scripts are named but their bodies are
not in `questlines.txt` at all, so the first question is where `q1002s` lives - a WZ node this
project has not dumped, or something compiled into the client.

#### Character create does not transition, 2026-08-21 - narrowed, not solved

The owner created "Tester", the client said the name was available, and then **stayed on the
creation screen**. Clicking OK again said the name was taken - because it was: the character
exists.

**Established from the captures, all [L]:**

* The character **is created**. Id 206 is in the database with its equips.
* The reply is **not** the problem. `0x0015` for "Tester" is **353 bytes**, and so is the
  `0x0015` for "Engineer" on 2026-08-19 that **did** transition - same length, same head
  structure field for field, differing only in the id, the name, the cosmetic ids and the
  random stat roll (both totalling 25).
* The **flow into it is identical** in both runs: `0x00A8` -> `0x05F4` -> `0x0081` name check
  -> `0x008A` -> `0x0015`.
* So this is a **regression in something other than the create reply**, and the reply is
  ruled out rather than assumed innocent.
* The static route is **blocked**: the `0x0015` handler `FUN_141b2cf30` decompiles to
  `halt_baddata()` - 35 bytes of bad instruction data. `research/msexe-char-create.c`.

**What actually differs between the working run and the failing one** is the account's
character-list state. On 08-19 the client logged in holding **3** characters and had deleted
one earlier in the same session; today it held **1** and had deleted two in a *previous*
session.

**The list-state hypothesis is FALSIFIED.** The owner ran both halves of the experiment on
2026-08-21: creating without deleting first **transitions**, and delete-then-create
**transitions**. Four mechanisms are now eliminated:

* **not the reply** - byte-compatible and the same 353 bytes as a create that worked;
* **not the list state** - both orders work;
* **not the cosmetics** - the face and hair in the reply match the request and the stored row
  exactly, for the failing character and the working ones alike;
* **not a live channel connection** - `world.log` for that run begins six minutes *after* the
  failed create, so no channel socket existed.

So it happened **once**, and nothing reproduces it. That is where it rests. The capture is
`research/fixtures/create-did-not-transition-tester-login.log`, and the static route is closed
- `FUN_141b2cf30`, the `0x0015` handler, is 35 bytes of `halt_baddata()`.

**If it recurs, the thing worth writing down is what came before it** - how long the client
had been up, what screen it came from, and whether a channel session had been entered and left
in the same launch. The packet is not the variable, so the state is.

#### The heap corruption, and what the last run actually showed

The owner, 2026-08-20: *"I tried to execute `!map 45`, which the command should guard me against,
but the client crashed."* **The guard is innocent, and this is what the evidence says.**

* **No chat packet reached the server at all.** Enumerating every inbound opcode in that
  run's `world.log` gives no `0x00E7` of any kind. `!map 45` was never transmitted, so
  `gm_map` never ran.
* **The guard was live.** `world.log.err` is 0 bytes, and a missing field table writes to
  stderr there, so `gm-handbook/fields.txt` loaded. 45 is not in it, and the command would
  have refused.
* **The fault is `0xC0000374`, heap corruption**, raised 325 ms after the last packet inside
  `FUN_14019b4e0` - a heap helper beside the client's allocator and free. That is where it
  was *detected*, not what corrupted it, and heap corruption is detected arbitrarily long
  after the write that caused it.
* **It is not new and not today's code.** Four sightings now, the earliest on 2026-08-19,
  before any of today's packets existed. Full write-up and the three captures:
  **`research/heap-corruption.md`**.
* **The new banner packet has never been sent in any of them** - zero `0x00AC` - so it is
  not that, and neither are the rate multipliers or the level gains.

**The next step is page heap, and the owner has to run it** because it writes a system key. The
exact command is in `research/heap-corruption.md`. It makes the client fault at the
instruction that corrupts rather than at the next free, which is the difference between a
diagnosable crash and this one. **The two full captures have also never been diffed**, and
that costs no client run at all.

One real defect did come out of it: `map_exists` is fail-open on an empty field table, so a
missing generated file silently removes the `!map` guard. There *was* a warning for that, on
**stderr**, where nothing reads it - the banner that reaches `world.log` is stdout. The banner
now reports the field count, and `!map` refuses outright rather than answering as though it
had checked.

#### Things that are NOT open, so nobody re-opens them

* **The pick-up opcode is `0x032C`**, object id at body offset 13. It can never be confirmed
  further: the builder is in `.themida`, whose `SizeOfRawData` is **0**.
* **`pool+0x90` is not a latch.** It is a client-side anti-cheat check that clears itself.
* **`mob+0xa88` is not the mob's animation object** - it is the avatar-look renderer, and
  `0` is correct for it on all 193 mob templates. The animation object is `mob+0x610`.
* **Item quests need no running count.** The client counts the bag live.
* **The shop is not a row problem and not a map problem.** One correctly-formed row killed
  the client exactly as twelve did, and the map loads fine without a shop.
* **There are TWO shop windows and we were sending the wrong one.** `0x055D`/`0x055E` are the
  classic `UIShop.img/Shop`; `0x0560`/`0x055F` are Shop2, whose art is missing. Both are arms
  of the same range chain in `CField::OnPacket`, and that chain is validated by `0x01A0`
  landing on the SetField handler two entries away. `research/classic-shop-opcode.md`.
* **The client cannot settle goal K.** The EXP curve at `0x143AC2400` has no sibling table:
  a `lea` scan of the whole neighbourhood that fills it finds exactly one `.data` target and
  it is the EXP curve itself. HP/MP per level is a server-side rule and the client never
  computes it - it is told new maxima. See goal K.
* **The bag is not the unequip blocker.** `presence[7]` lands; 125 slots render, minimum 30 and
  maximum 125, both from the owner. And **the four lists after the equipped one are not four bags** -
  only the first is, the Equip tab; the other three take positions 3000+, which nothing here can
  create. `research/bag-lists.md`.
* **`0x02FF` must be answered**, or mobs freeze after one simulation step. **`0x0107` must
  always be answered, including refusals**, or the whole inventory UI dies.
* **The client computes its own damage** — *and that is no longer true everywhere.* It holds for
  damage **to** mobs: we send consequences, never numbers, and the floating damage number over a
  mob is a client-side stub that read `1` while a Drake emptied a 238-HP bar. It does **not**
  hold for a player being hurt on somebody else's screen: `0x02A5`'s HITINFO **+0xa8** is a
  server-fill field, the client writes `0` there in 331 of 331 captures, and an echo draws
  nothing. Corrected 2026-09-04.
* **`0x0301` is a MOB picking up a drop**, not the player's request. It nearly shipped as one.
* **`0x00AC` is BroadcastMsg and type 4 is the banner**, `0x00AB` is TownPortal, and the
  candidate table's two `BroadcastMsg` entries are both wrong. `research/broadcast-banner.md`
  has the working, including the bit mask that says which types carry no string.

#### Instruments that have lied, and are now fixed

Every one of these produced a clean, confident, wrong answer. They are listed because the
next wrong answer will come from an instrument nobody has checked yet.

| tool | what it did |
|---|---|
| `tools/callers.py` | scanned for `call` only. **27 909** of 120 981 functions would have come back "zero callers" while reachable by a tail `jmp` or a vtable pointer. Now reports three kinds |
| `tools/rtti.py` | mapped addresses in the zero-initialised tail of `.data` into `.pdata` and returned those bytes as data - 120 confident wrong numbers for the EXP curve. Now raises |
| `tools/fieldrefs.py`, `tools/rangescan.py` | **silently drop `rbp`-based operands.** A scan for a field's writers came back without the one writer everybody already knew about. **Not fixed** - work around it and say you did |
| `tools/encodes.py` | misses fields written by a **loop**, so a body length from it alone is short and confident. **Not fixed** |
| the probe's throw log | logged throws only 25 s after arming, so a client that died at 23 s recorded **zero throws** and read as "it did not throw". Now logs the first eight always, and every fault line reports throws *seen* against *logged* |
| `tools/decode_dump.py --exp-curve` | labelled entry `i` as level **`i+1`**, so the first-ever comparison against `data/exp-curve.txt` reported **all 98 levels disagreeing when all 99 agree**. Its own comment asserted the wrong indexing, so reading the code confirmed the bug rather than catching it. It also printed a **permanent** false `monotonic: NO - suspect` - the trailing cap zero compared against the last real level. **Fixed 2026-08-21**, and the fix is checkable without trusting either table: 1 hole + levels 1..120 = 121 entries with exactly 2 zeros, which is the 119 non-zero it reports |

---

> Everything below is **older working, kept for its method rather than its verdicts.**
> Where it disagrees with the section above, the section above is right.

#### THE FIX: every mob was sent a size of **zero percent** - 2026-08-20

Body offset 91 -> `mob+0xd64` is the mob's **size percentage**, and this server sent `0`
there from the day mobs were implemented. `0` is not "unset"; it is *zero percent*. **[L]**

```text
141c571bc  cmp   ecx, 0x64          ; scale == 100?
141c571bf  je    0x141c573c8        ;   yes -> skip the adjustment entirely
141c571cb  sub   eax, r8d           ; width = right - left
141c571d4  sar   eax, 1             ; halfWidth
141c571da  lea   eax, [rcx - 0x64]  ; scale - 100
141c571f6  mulsd ...                ; delta = halfWidth * (scale - 100) / 100
141c571fe  add   [rdi + 8], eax     ; right += delta
141c57201  sub   r8d, eax           ; left  -= delta
```

At `0` the multiplier is `-1.0`, so `delta = -halfWidth`, both edges collapse onto the centre
and `left >= right`. `141d326ae` then **skips** the rect instead of rejecting it, `r14b` stays
`0`, and `141d327c6` drops the mob - with all seventeen documented gates green, which is
precisely why seventeen green gates explained nothing.

**Invisible because the two paths disagree about what `0` means**: the rendering paths test
`<= 0` and read it as "no scale set, draw normally"; the hit-box path tests `!= 100`. The
snails drew, animated and walked while having no hit box at all.

**`mob+0xa88` is not the bug and the label was wrong.** It is the mob's *avatar-look*
renderer - mobs drawn as player characters - and **0 of 193** mob images carry the
`avatarLook` node that would allocate it. Null is correct. The animation object is
`mob+0x610`. `research/mob-a88.md`.

> **CONFIRMED ON SCREEN, and this un-does a retraction.** The owner, after the fix: *"The mob
> killings work, I'm taking damage, and the mob is also taking damage."* One field, both
> directions.
>
> The retraction is worth keeping because it was the more expensive mistake. A static pass
> reported that the mob-to-player path reads the **player's** body rect and takes its
> rectangles from the mob's **attack template**, and that a snail has no `attack` node - so
> this field could not be involved. The two-direction claim was withdrawn on that basis, and
> The owner was told not to expect the snail to hurt them.
>
> What that pass had found was the *attack-node* path, for mobs with attack animations. It
> stated in its own words that a **body**/touch-damage path was **not found**, and marked it
> `[D]` with a named blind spot: a function handed the template as an argument reads
> `[rcx+0x7c]` without touching `+0x3a8`, so the intersection scan cannot prove absence.
> **"Not found" was read as "not there".** That is this file's own rule pointing the other
> way - a silent negative believed because it was tidy - and it cost a correct answer being
> withdrawn hours after it was made. `research/touch-damage.md` remains the best map of the
> attack-node path; only its negative failed.

#### MEASURED: `mob+0xa88` is null on every mob - 2026-08-20, on a real client

`research/fixtures/mob-a88-null-on-every-mob-{world,hook,exit}.log`. The client closed
cleanly (exit 0); nothing crashed.

| | |
|---|---|
| positive control `140304100` | **10 lines** - the hook armed, so the readings mean something |
| distinct mobs the collector walked | **30**, all inside a single swing |
| `mob+0xa88` | **`0` on all 40 readings**, every mob without exception |
| `mob+0x42c` | `0xffffffee` / `0xffffffef` - **not** zero |
| attacks sent | 11, **every one 127 bytes**, which is zero targets |
| first collector hit -> first `0x00DF` | 16 ms, so the pairing holds as before |
| `0x02FF` mob movement reports | 990 - the mobs were alive and moving, not a dead field |

**The predicted root cause is confirmed.** The animation object is null on every mob, so
`FUN_141c57120` bails at `141c57185` and writes an all-zero rectangle, so `141d326ae` skips
every rect, so `141d327c6` rejects every mob - with all seventeen documented gates green.

**One prediction was wrong and it does not change the answer.** The forecast was that a null
`0xa88` would show up as an all-zero `mob+0x42c`; the measured left edge is **-18/-17**, a
perfectly ordinary coordinate. That is consistent rather than contradictory: `mob+0x42c` is
an *input* to `FUN_141c57120`, and the `0xa88` bail at `141c57185` happens **before** the
input is ever consulted. A real value sitting in a field that is never read is exactly what
a bail-out looks like from the outside. Worth writing down, because reading `0x42c` alone -
which the superseded watch pair would have done - would have shown a healthy rectangle and
pointed the next pass in precisely the wrong direction.

**What this is now:** `mob+0xa88` has two writers in the mob code range, the constructor's
null and `FUN_141cd1620`. So the question has moved off the attack path entirely and onto
the **spawn** path: what makes `FUN_141cd1620` run, and whether anything our `0x03C6` sends
can reach it. Nothing about the swing, the attack rectangle, or the gate chain needs more
work.

#### The gate that rejects our mobs is geometric, and it was never in the table

`research/mob-gates-arm-c.md`. All seventeen documented gates pass; the rejection is at
label `141d32a18`, which the earlier write-up dismissed as *"a nested container-growth block,
plus two list-walk exits"*.

```
141d326ae  cmp [rdi],eax / jge   left >= right  -> SKIP this rect (not a rejection)
141d327c6  test r14b,r14b / je   nothing intersected -> REJECT THE MOB
```

**An all-zero rectangle satisfies the per-rect filter and is skipped**, so `r14b` stays 0,
the loop finishes clean, and every gate anyone had enumerated is green. Fifteen slots, the
mob examined, nothing accepted - exactly the symptom.

The chain reaches a single field:

```
FUN_141cd1620   sets mob+0xa88, the animation object (the constructor leaves it NULL)
FUN_141cb4600   141cb4645  mob+0xa88 == 0 -> RETURN, writing no rect
                141cb4647  otherwise -> lea rdx,[r15+0x42c]  <- THE SETTER
FUN_141c57120   141c57185  mob+0xa88 == 0 -> bail -> ALL-ZERO rect
```

**`mob+0xa88` gates the rect in two independent places.** `-MobTargets` now reads that field
and the rect together, because reading the cause beside the effect separates three outcomes
where two halves of the rect separated only two. The three readings are in the launcher's own
comment, beside the probe string.

> **A retraction worth reading.** "Nothing in the mob code range sets `mob+0x42c`" was
> reported, then found wrong. Two scans - one over the mob range, one over the whole image -
> agreed, and both were blind the same way: a `[reg+disp]` write-scan cannot see a store made
> through a `lea`'d pointer, which is precisely what the setter does. `CLAUDE.md` now carries
> the general form: re-running the same tool is not a second opinion.

#### THE ROWS ARE NOT THE VARIABLE - run 2, 2026-08-20

`research/fixtures/shop-one-row-still-faults-{world,hook,exit}.log`. `-ShopRows 1` sent
**one buy row, 38 bytes** (`4 + 2 + 31 + 1`, and the log line confirms *"1 rows (1 buy, 0
sell) - 11 row(s) HELD BACK"*). The client hung, then died: fault at `0x140ce89d6`, 2.9 s
after the packet, exit `0xC0000005`.

Twelve rows killed it and one correctly-formed row killed it. **So the fault is not in row
content, not in row count, and not in the sell rows.** It is the shop path itself.

**And map 1013 is not the variable either - established without spending a launch on it.**
In `research/fixtures/mob-a88-null-on-every-mob-world.log` the client entered the world
**on map 1013**, received Lucy's `NpcEnterField` (template 21), stood there 5.4 s, walked
away and exited cleanly with code 0. The map loads, Lucy spawns, everything is fine. The
only difference in the two crashing runs is that they were **clicked** and we answered with
`0x0560`.

> **Run 2 logged zero C++ throws, and that is the instrument, not the client.** The probe
> logged throws only 25 s after arming. The watches armed at `09:46:10.6`, so the window
> opened at `09:46:35.6` - and the client was already dead at `09:46:33.8`. Read naively
> that says "the client did not throw", which is the opposite of the truth, since run 1
> caught a throw 10 ms after the same packet. **Fixed**: the first eight throws are now
> logged whatever the clock says, and every fault line reports how many throws were *seen*
> against how many were *logged*, so those two can never be confused again.

#### Lucy's counter kills the client - what the logs establish, 2026-08-20

Fixtures: `research/fixtures/shop-opens-then-client-faults-{world,hook,exit}.log`.

```
11:51:21.098  -> 0x0560 OpenShop, 12 rows (6 buy, 6 sell), 379 bytes
11:51:21.108  C++ THROW #7                       <- ten milliseconds later
              (the client sends NOTHING further, at all)
11:51:24.656  CLIENT FAULT 0xc0000005 at 0x140ce89f7
```

**The throw is the event and the fault is the wreckage**, and that is now a chain rather
than a guess:

* seven throws in the whole two-minute session, and this one lands 10 ms after our packet;
* from that instant the client sent nothing - no movement, no shop request - so it was
  wedged immediately, 3.5 s before it died;
* `0x140ce89f7` is inside `FUN_140ce89c0`, a **reference-counted release**, and the fault
  stack's first frame `0x14308e3ed` is the return address of the call at `0x14308e3e8` in
  `FUN_14308e3db` - a 24-byte destructor thunk;
* `research/setfield-fault-shape.md` had already characterised that function as **a latent
  bug in the client's own error path**: any early exit that skips the acquisition leaves the
  holder's `+8` as uninitialised stack, and the epilogue's guard then releases it.

A throw unwinds, unwinding runs destructors, and one of them ran over a local the skipped
path never wrote. So **the fault address says "something bailed out", not "the shop"** - it
is the same address a completely different subsystem faulted at in
`setfield-accepted-client-entered-world-hook.log`.

**Four hypotheses checked and dead**, so nobody re-checks them:

| | |
|---|---|
| the body is short | **no.** `tools/reads.py` over the row decoder gives exactly the nine documented fields plus the trailing `u8`: `4 + 2 + 31n + 1` = 379, which is what went out. The two *extra* `u8` reads it finds in the outer handler belong to the `0x055F` arm - `140d22888` is followed immediately by `jmp` to the exit |
| an item id the client cannot resolve | **no.** All six ids are in the client's own `itemdata`. `!item` refuses unknown ids; the shop has no such check, so this was the obvious suspect |
| the disabled flag, or a zero max-per-purchase | **no.** The two fields `research/npc-shop.md` §2.6 names as able to silently break a shop are 0 and 100/200/1 |
| the old Amherst crash | **no.** That one had the NPC never appearing and died in the `SetField` tail. Here the map loaded, Lucy spawned, and they were clickable |

The array-constructor frame in the throw's stack is **stale** - there is no call to
`FUN_142ef44fc` anywhere in the shop handler. A stack scan is not a call stack.

**What is not established: which row, or whether rows are the variable at all.**
`--shop-rows 1` (`-ShopRows 1`) is the lever - one **buy** row, because the buy direction has
a straight-line trace behind it and the sell direction has never been on a wire in either
direction. A counter that opens says the shop path is sound; one that still dies says rows
are not the variable and the next work is static, not another launch.

#### The loop RUNS, with room for fifteen - measured, 2026-08-20, and it moves the question

**No client run was spent on this.** It came out of a fixture that had been sitting in
`research/fixtures/` for a day with the answer already in it.

The suspicion was that `FUN_141d31b20` returns at `141d31c96` before examining a single mob,
when argument 17 is at least argument 4. Filtering
`melee-collector-runs-once-per-swing-hook.log` to the collector's **own** watch lines - the
file carries four watches and the attribution matters - all six entries read:

```
0x141d31b20 ENTERED ... rcx=0x37d74fa0 rdx=0x146530 r8=0x145de0 r9=0xf  called-from=0x141d2545a
```

| | | |
|---|---|---|
| `r9 = 0xf` | **argument 4 = 15** | the loop's capacity, hardcoded at the call site |
| argument 17 | the output cursor, 0 on entry | so `0 >= 15` is **false** |
| `called-from` | `0x141d2545a` | `FUN_141d25360`, and **not** `0x1428c2c32` |

**So the loop ran, with room for fifteen targets, and accepted none.** The early-out is dead
as an explanation. Two corrections come with that, and the second is the important one:

* `141d31c96` is the **loop header**, not a one-shot precondition. Argument 4 is a capacity
  and argument 17 is the output cursor, incremented at `141d327de`/`141d32939` with
  `141d32a62` re-entering. `research/mob-target-gates.md` §4.2 reads it as a precondition.
* **`research/mob-target-gates.md` §6 dismissed gates 3, 5, 11, 14, 15 and 16 as "switched
  off by the arguments at the only known call site" - and that call site was the wrong one.**
  It analysed `0x1428c2c2d` inside `FUN_1428c1fa0`. The client calls through
  `FUN_141d25360`, a different arm with different arguments, which reaches the same collector
  at `141d25455`. Those six gates were ruled out for a caller the client never uses, so the
  clean "every gate passes" result was answering about the wrong path.

`research/mob-collector-callsites.md` has all 86 call sites with their return addresses, so
any future `called-from=` reads straight off a table. It also establishes that **every** entry
is a 5-byte `call rel32` - no tail `jmp`, no vtable, no function pointer anywhere in the image
- so a return address always names a real call site.

The next pass re-runs the gate analysis against **arm C's** arguments. That is static work and
it does not need a launch either.

#### `FUN_141d31b20` IS the melee collector - measured, and it corrects a prediction

`research/fixtures/melee-collector-runs-once-per-swing-{world,hook}.log`, one session:

| collector hit (hook clock) | attack sent (server clock) | gap |
|---|---|---|
| `01:31:40.568` | `05:31:40.570` | 2 ms |
| `01:31:41.668` | `05:31:41.669` | 1 ms |
| four more, same shape | | 1 ms |

**Six swings, six hits, each 1-2 ms before its `0x00DF`, in order.** No unpaired hits, no
unpaired attacks. Every attack is 127 bytes, which is zero targets.

Two corrections fall out of that, and both would mislead the next person:

* The caller is **`0x141d2545a`**, not the `0x1428c2c2d` that `research/mob-target-gates.md`
  predicted. Its outcome table maps "called from elsewhere" to *"per-frame noise, melee is
  elsewhere"* - which is exactly wrong here.
* `while dispatching opcode` in a WATCH line is **sticky** - it is the last inbound opcode,
  not the reason for the call. It read as mob traffic and nearly filed this as noise.

So the gate chain is the right instrument, the collector runs once per swing, and it
collects nothing. The next run watches **the gates**, not the entry.

**Do not change `appear_type`.** `>= 0` writes the literal `1` into `mob+0x504`, which is
exactly what gate 2 rejects on - it would make every mob permanently unhittable and look
like a regression somewhere else. The warning is on the field in `crates/net/src/mob.rs`.

#### Things that are NOT open, so nobody re-opens them

> **MERGED 2026-09-04.** There were two lists under this exact heading. This one was a subset of
> the newer one - the list **further up**, immediately after "The heap corruption, and what the
> last run actually showed" - and its two unique items (the bag numbers, and the four lists that
> are not four bags) have been folded into it. **Read that list.** It also carries the correction
> to *"the client computes its own damage"*, which stopped being true everywhere when `0x02A5`
> started carrying a server-filled number.

#### The test plan for the next run — STRUCK 2026-09-04

> **Ten steps used to stand here and nine are confirmed**: `!item` and the `0x0070` it rides on,
> the shop counter and both prices, the quest journal across a map change, `!exp`, the drop, the
> pick-up and the channel-change row colour. They are in the CONFIRMED table near the top of this
> file. Re-running them costs the owner a launch and answers nothing.
>
> **The exception is step 6, the chat balloon, and it is a genuine hole.** `0x0231 USER_CHAT` has
> gone out exactly **twice in the whole archive** - 2026-08-19 *"Hello David"* and 2026-08-29
> *"hello"* - and no capture can say whether a balloon drew, because that is a screen fact and
> nobody has been asked. `UNVERIFIED 2026-09-04`. One sentence from the owner on any future run
> settles it, and it is free: type something without a `!` and say whether a bubble appears.
>
> **A test plan does not belong in `STATUS.md` at all** - `CLAUDE.md` has the section on why, and
> this block is the fossil it was written about. The plan lives in `tools/test-server.ps1`, in
> two copies that must be updated together.
>
> Three things from that block are not duplicated anywhere and are kept below: the `-Probe`
> positive-control trap, the `0x025F` retraction, and the two watches that still need runs of
> their own.

**Do not pass `-Probe` unless you mean to.** With `-SetFieldProbe` and no explicit `-Probe`
the launcher installs a matched pair *plus* `140304100:hits=200`, the **positive control**:
no lines from it means the hook never armed and nothing else in the log proves anything.
Passing `-Probe` by hand replaces all four slots and silently drops it.

**`0x025F` was a false alarm, and the retraction is worth reading.** The claim was that the
client "builds it six times from inside `DropEnterField`, so it will arrive the moment a
sword lands". That misread **six call sites as six sends**. All six are the same shape -
`test ptr,ptr / jne carry-on / mov r8d,<code> / call the builder / jmp abandon` - so it is an
**error report**, sent only when the drop handler hits a null and gives up. Nothing waits on
a reply: the builder's `SendPacket` is followed by the stack cookie and `RET`, five of six
callers jump straight to the handler's exit, and no capture in the repo contains one.

So it is a **gift, not a hazard**. A `0x025F` means our `0x046E` was rejected and the drop
was abandoned - which otherwise looks exactly like "nothing on the ground, no fault, nothing
in any log". Its second `u32` is a `__LINE__`-like code naming which of the six null checks
failed; `research/item-drop.md` §10.1 turns it into an address. Expect site 2 (`0x3d1`)
first: it is the only null that comes from a **lookup by id** rather than a packet field.

Two watches, each needing its own run, neither combinable with the above:

* chat: `-Probe "watch@1415db360:ret,141b2a280:rdx=0,14276df20:peek=10d0:hits=6,140304100:hits=200"`
* **mob targeting: `-SetFieldProbe -MobTargets`** - a switch since 2026-08-20, so the four
  slots and the positive control come out right without hand-typing them. Go to map 40 and
  swing at a snail six or eight times.

**Do not spend a run re-asking whether the loop starts.** It does: argument 4 is 15 and the
cursor is 0, measured off an existing fixture - see "The loop RUNS" below. `-MobTargets`
survives as the frame to hang the *gate* watches on once the arm-C analysis names them; its
`141d31b20:args=17` slot is worth keeping only because the cursor is the one number that says
whether **any** mob was accepted mid-loop, and `called-from=` names the arm in one word.

`:args=<n>` dumps integer arguments **5..=n** off the stack (argument *n* at `[rsp + 8*n]`, so
argument 17 is `[rsp+0x88]`). Arguments 1-4 were always in the log as rcx/rdx/r8/r9, and
`stack_trace` filters to things that look like code addresses, so it discards exactly the
small integers this turns on. Slots 5..16 print too, on purpose: "argument 17" is a
decompiler's numbering, and if it disagrees with the ABI by one the right value is still on
the line.

**Nothing authenticates.** The game socket carries no credentials, and none of the above
changes that.

### THE NEW GOALS - set by the owner, 2026-08-19, later in the day

These are goals rather than tasks; each is bigger than a sitting.

#### A. Quest state that actually advances - **kill quests DONE 2026-08-20**

**The paragraph below is stale and kept for its method.** Quests persist, accepting is
recorded on `0x0151` action 1, the journal is sent on field entry, and a kill now counts
toward every started quest that named that mob. The count is **three zero-padded decimal
characters per mob slot** - an integer there renders as nothing, which is exactly what a
broken counter looks like. Item quests need no running count: the client counts the bag live.
`research/quest-progress.md`.

What is *not* done: turning a finished quest in, and what the client expects when the player
returns to the NPC short of the requirement.

The original write-up, from when none of it worked:

**Nothing about quests persists or changes.** Accepting does nothing, the same line comes
back every time, and the journal never fills. Full write-up is in its own section below
("NEW GOAL ... quest state that actually advances"). In short it needs the **presence-gated
quest block** in the character record - the third block of the same shape as `presence[0]`
and `presence[2]`, both of which are worked examples - a **quest-result packet that has
never been found in either direction**, a `quest_state` table, and the `Check`/`Act` rules,
which are a pure function over data already generated.

**Depends on item 1 above:** a quest cannot be *accepted* until the client can answer a
yes/no box.

#### B. NPC idle chatter - **DONE**, and the open question below is answered

> **ANSWERED.** The paragraph below asks *"whether the server sends it at all"*. It does, it is
> confirmed on screen, and the proof is a side effect nobody was looking for: chatter for the
> field a player had **left** kept arriving while they were in the Cash Shop - 38 dispatches
> with `rdx=0x453` in 103 seconds. It is server-side, it works, and the residual bug is that it
> follows the player onto a stage they are not on. That fix wants a "which stage am I on" flag
> on `Session` and is still not done.

The owner, 2026-08-19: NPCs should cycle their idle lines **in order, on a cooldown**. The data
is generated - `gm-handbook/npcstrings.txt`, 441 chatter lines across 266 NPCs, and Robin's
ten match an outside listing exactly, in order.

**What is not settled is whether the server sends it at all.** The client may well do this
itself, the way its minimap reads the WZ directly and the way it picks its own
`0x00F2`-vs-`0x0151` click path out of `Quest.wz`. If it is client-side, the answer is a
**field in the `0x044F` body** - and that body has form: every NPC was invisible for days
because `isEnabled` and `alpha` were zero while the layout was perfect. Several fields there
are still placeholders.

#### C. Mob drops - **DONE 2026-08-20**

Items fall when a mob dies, at the mob's position, staggered, expiring after two minutes,
and the pick-up works. 993 rows over 170 mob templates from the community database, plus a
global table for event items that every mob rolls. `crates/world/src/droptables.rs`,
`data/drops.txt`, `tools/scrape_drops.py`.

**The chances are ours, not data.** The source has vote scores and no drop rates; only the
meso rows carry a real chance. The score is kept as a column so the policy can be retuned
without scraping again.

#### D. Character level up - **DONE 2026-08-20**, and the curve is worth a check

A kill awards the mob's own EXP from `gm-handbook/mobtemplates.txt`, levels carry over, and
the client plays its own level-up animation from the `0x007C`. The curve is
`data/exp-curve.txt` - **and it is no longer a community guide's word. The comparison was
finally made on 2026-08-21 and levels 1..99 agree EXACTLY, 99 of 99**, against the dump in
`research/fixtures/equip-into-empty-hat-slot-kills-client-hook.log`. The file now runs to
**119**, because the client's table carries 100..119 and the guide stopped at 100. Level 120
is the cap; the client's own entry for it is `0`.

**It took fixing the instrument first, and that is the point.** `tools/decode_dump.py
--exp-curve` labelled entry `i` as level `i+1` - its comment said *"Entry 0 is level 1's
requirement"* - so the first comparison reported **all 98 levels disagreeing when none of them
do**. Entry `i` is level `i`'s requirement: `[0] = 0` is the hole that makes the index
1-based, `[1] = 15` is level 1, `[119] = 28171993`, `[120] = 0`. One hole plus levels 1..120
is 121 entries with exactly two zeros, which is the 119 non-zero the dump reports - the counts
settle it without needing either table to be trusted. The same fix cleared a **permanent**
false `monotonic: NO - suspect`, which was the trailing cap zero being compared against the
last real level.

What a level *awards* is policy, tagged `[I]`, in one place: five AP, flat HP/MP, no SP.

The original brief follows.

> *"Once we can kill mobs, the next thing to handle is character level up. Once the EXP
> reaches or exceeds 100%, the player advances to the next level. They receive their level up
> animation client side and gets extra HP/MP, 5 ability points, and 3 skill points. If they
> are under level 10, they only receive 1 skill point per level."*

Blanket-accepted as the rule. What it needs, and which parts are blocked:

| part | blocked on |
|---|---|
| EXP arriving at all | **mob combat** - EXP has no source until a mob can be killed |
| the EXP-to-next-level curve | nothing - it is in the client, and extracting it is independent |
| +5 AP, +3 SP (1 below level 10) | nothing - server-side arithmetic |
| extra HP/MP | nothing to *decide*, but the per-level amounts come from the client's own job data |
| the stat change reaching the client | **mob combat** - that agent owns the stat-change packet, and level/HP/MP/AP/SP ride the same one |
| the level-up animation | nothing - the owner says the client owns the animation, so this is one effect packet |

**One ambiguity, and it is stated rather than guessed.** "If they are under level 10" is read
here as **the level being reached**: 5 -> 6 gives 1 SP, and 9 -> **10** gives 3, because the
new level is not under 10. The other reading (the level being left) would make 9 -> 10 give
1. It is one comparison to flip if that is wrong, and it is called out here so it is a
decision rather than an accident.

**The trap to avoid.** HP/MP per level is exactly the kind of number this project keeps
getting from the reference server and paying for later - the inventory bag was 24 for a
commit because of it. Take it from this client's own WZ, and if it cannot be found there,
say so and label the number **[I]** rather than shipping it as fact.

#### E. First job advancement at level 10 - **DONE**, packet and NPC conversation both

> *"Once player levels up to 10, they should be able to job advance to one of the 4 primary
> job IDs, either Thief/Magician/Bowman/Warrior by talking to the respective job advancement
> instructors. Each job has a pre-requisite, which I assume is widely available on the
> internet. Magician 35 INT, Warrior 35 STR, Thief 35 LUK, Bowman 35 DEX."*

Blocked on **D (level up)** - there is no way to reach level 10 yet - and it shares the
script machinery with **A (quest state)**.

**The job ids are [L], from this client.** `net::opcode::uses_extended_sp` decodes the SP
fork's three literal bit masks in `FUN_140302e30`, and they give exactly the explorer tree:
`100/110/111/112/120/121/122/130/131/132` and the same shape at `200`, `300`, `400`, `500`.
So the four first jobs are **Warrior 100, Magician 200, Bowman 300, Thief 400** (and Pirate
500 exists in the masks whether or not we use it).

**The instructors are [L], from the client's own `String.wz/Npc.img`** via
`gm-handbook/npcstrings.txt`:

| job | instructor | template |
|---|---|---:|
| Bowman | Athena Pierce | **221** |
| Magician | Grendel the Really Old | **313** |
| Thief | Dark Lord | **411** |
| Warrior | Dances with Balrog | **511** |

There are also four NPCs literally named `<Job> Job Instructor` - 227, 319, 424, 514 - and a
second set at 800003/800004. Which set a real advancement uses is **not** established; the
named four are the ones the game is known for.

**The 35-stat prerequisite is [I], and the client will not help.** The owner flagged it as
fan-site sourced themself. A scan of every `Check` node in all 322 quests
(`gm-handbook/questlines.txt`) finds **no `int`, `str`, `dex` or `luk` requirement anywhere**
- and that is a *verified* negative, not a failed search: the same scan enumerates 20 other
Check keys including `lvmin` (406 uses), `job` (104), `skill` (48) and `item` (798). The
requirement is not in `Quest.wz`.

That is the expected answer rather than a surprise: in this game family the first job
advancement is an **NPC script**, and NPC scripts are server-side. So:

> **The client does not enforce this rule, which means the server is the only thing that
> can.** There is nothing to verify it against and nothing that will catch it being wrong.
> Ship the owner's numbers, label them **[I]**, and put them in one named table so they are one
> edit to change - do not scatter `35` through the code.

**What still has to be found:** the packet that actually changes a character's job, and
whether the client needs anything beyond the stat block's `job` field at the next `SetField`.
`research/charstat-layout.md` has the stat block; the job field is at `+0x33` and it is
already sent on every `SetField`, so the cheapest first experiment is whether simply storing
a new job and re-sending the record is enough to make the client show a first-job character.

#### F. NPC shops - **DONE**: the counter draws, buying and selling both work

**`0x0560` cannot work on this client and no server byte can change that.** The shop window's
constructor loads `UI/UIWindow2.img/Shop2/backgrnd`; that image is **not in this client's
WZ**; the resource call fails, `_com_issue_errorex` throws, and the unwinder faults. The
client never returns from the handler - neither crashing run has a numbered dispatch line for
`0x0560` - and it dies **before reading a single row byte**, which is why one correctly-formed
row killed it exactly as twelve did. Two of the owner's manual launches went on this.

It is **off by default** and a shopkeeper falls through to ordinary dialogue.

**That question is answered.** There are two shop windows on two adjacent opcode pairs, and
we were sending the wrong one:

| | window | art | open | result |
|---|---|---|---|---|
| classic | `UI/UIShop.img/Shop` | **present** | **`0x055D`** | `0x055E` |
| Shop2 | `UI/UIWindow2.img/Shop2` | **absent** | `0x0560` | `0x055F` |

Both are arms of the same range chain in `CField::OnPacket`, and the chain is validated two
entries away by `0x01A0..0x01A3` landing on the SetField handler. Full working, including how
the strings were found and what the instrument was checked against, in
`research/classic-shop-opcode.md`.

**What is left is the body, and it is not small.** The head is decoded and confirmed by two
instruments - `u32`, `u8`, a conditional `u32`, then `u32`, `u32`, a 31-character string, a
`u32` and a `u16` row count. But **each row carries a thirteen-field item structure**
(`FUN_1404ba100`, one caller, bespoke to this shop) with an undecoded conditional tail, and
**nothing yet says which of those fields is the price**.

**Nothing was implemented, deliberately.** `crates/net/src/shop.rs` still builds `0x0560`.
Pointing it at `0x055D` without decoding the body would send a short body to a handler that
reads a longer one - the same mistake as the truncated chat packet, with the same
consequence. `research/npc-shop.md` is the template for finishing it; most of its method
transfers.

Everything else about the shop is built and tested - the row layout, both tabs, the
transaction, the untradeable rule - and is waiting on that one answer.

The original brief:

> *"NPC shops when clicked on by the client should open the appropriate NPC shop with the
> appropriate shop list with appropriate item prices. Users should also be able to sell items
> in their inventory back to the NPC shop for mesos, such as unused equips and monster ETC
> drops. Please do not allow quest items to be sold."*

#### G. Storage - **DONE 2026-08-22, end to end including Organize**

> *"Storage is kind of like inventory, except all of the characters of a particular account
> share this inventory. The storage stores mesos and items. Please do not allow untradeable
> items to be stored."*

**That blocker is gone, 2026-08-20.** This paragraph used to say storage was blocked on the
inventory existing at all: no item in a bag, no meso balance, `0x0107` never sent. All three
are now false. The bag persists across a map change (goal I, confirmed on screen), every
`0x0107` is answered including refusals, and meso drops credit `characters.mesos` and report
the new balance with `0x007C` bit 18 - the only way this client is ever told one.

What storage still needs is its own work: an **account-scoped** container beside the
character-scoped one, and **the opcode that opens the storage window - which nobody has
looked for.** There is no storage decompilation in `research/` at all, so treat that as the
first step rather than assuming it resembles the shop window. `crates/world/src/shops.rs`
already has `may_be_stored` waiting for a caller. Shops and storage are both "move an item
between two containers", so whichever gets built first should own that machinery.

##### What the client already gives us, measured

| | where | |
|---|---|---|
| **item price** | `Item.wz` `<item>/info/price` | **[L]** - read directly out of `Item/Etc/Etc_000.wz`; `04000001` is `price: 1`, `04000002` is `price: 2` |
| **the quest-item flag** | `Item.wz` `<item>/info/quest` | **[L]** - `04000000` carries `quest: 1` **and** `price: 0`. The owner's "do not allow quest items to be sold" is enforceable from the client's own data, not a fan site |
| **the untradeable flag** | `info/tradeBlock` | **[L]** - already extracted for equips by `tools/dump_equips.py`, which measured **7 of 1760**. The owner's "do not allow untradeable items to be stored" is likewise enforceable from client data |
| **stack size** | `info/slotMax` | **[L]** - `200` on the ETC items sampled |

That is a good outcome and worth saying explicitly: **both of the owner's restrictions are
properties the client ships**, so neither has to be invented or taken from a fan site. Extend
the `tools/dump_*.py` family to write a `gm-handbook/itemdata.txt` carrying
`id, price, quest, tradeBlock, slotMax` and both rules become table lookups.

##### What the client does NOT have, and this is the scoping fact

**Shop contents are not in the client.** Checked three ways, each with a control:

* `Data/Etc/Script/Script_000.wz` contains **no images at all** - the client ships no NPC
  scripts. Control: the same `wz-dump tree` call on `Etc_000.wz` lists 67.
* Of those 67 images, none is a shop or storage list. The only near-misses are
  `CashShopCategory.img` (cash shop UI categories) and `NpcNoticeBoard.img` (13 bytes).
* `Npc.wz` carries no shop node reachable from the NPC template.

So **which items an NPC sells, and for how much, is server data we author** - which is how
the real service works too, and it is why `price` alone is not a shop.

> **An instrument correction, because it nearly became a finding.** The first pass at this
> searched the `.wz` files for the raw bytes `shop`/`storage` and got zero, which looked like
> an answer. It was not: WZ encodes property names, and the same search returns **zero for
> `price`, `quest`, `info` and `icon`** - all of which had just been read out of that exact
> archive with `wz-dump`. A raw byte grep over a WZ archive is a broken instrument and any
> negative from one is worthless. Use `wz-dump`.

##### What still has to be found

1. **The inventory item itself** - the record block that puts an item in a bag, and `0x0107`.
   The equipped-list block (`presence[2]`) is already decoded and is the closest model.
2. **The shop dialog packet** - what opens the shop UI and carries the list, and the inbound
   buy/sell request. The NPC-click path (`0x00F2`) already reaches the server, so the trigger
   exists; only the reply is missing.
3. **The storage dialog packet**, and the storage NPC templates.
4. **Mesos.** Nothing maintains a meso balance today. The character stat block has a field
   for it (`research/charstat-layout.md`); it is sent as zero and has never been exercised.

##### Storage is per ACCOUNT, and that is a schema decision

`characters` is keyed per character; storage is not. It wants its own table keyed on
`account_id`, alongside a meso column - **not** a column on `characters`. Worth stating
before anyone adds it in the wrong place: the account is already the unit that owns
characters (`crates/store/src/db.rs`), so the foreign key is natural.

#### H. Citizenship - set by the owner, 2026-08-19

**2026-09-28: BUILT - see START HERE.** **2026-09-27: `research/citizenship-2026-09-27.md` extends this** - the site's shops, resident items and
rules matched against the client; two corrections from the data (weekly pay is fixed per quest; grade
and level gates move together, which derives the level column); the client's own hooks (shop-row
grade fields, Character Info section, effects 83/84, the contract window, `/citizenship`); and the one
open question before building: which packet tells the client its own town and grade.
**ANSWERED the same day (§5 of that file): there is no citizenship packet.** The client keeps it in
hidden quest **510000**'s `key=value;` record - `st1/gr1/ct1` (Henesys), `st2/gr2/ct2` (Kerning City) -
sent in the character record (presence byte 16, `u16 n, n x (u32 quest, str)`) and live as `0x0089`
sub-case 13 (`u32 quest, str`). Quest start/complete and shop rows all lock on `st == 1 && gr >= need`
(start refusal `0x50`). Without that record the client refuses every citizenship quest, offered or not.
**The contract window, same day (§5.4):** ScriptMessage types **`0x42`..`0x46`** = Oath / Transfer /
Reactivation (shows the meso fee) / Renunciation / Grade Update, bodies `u8 town, [u8], [u32], u32 npcTemplate`;
answers on `0x00F3` as `u32 handle, u8 0x42+v, u8 1|0`, then `u32 handle, u8 0x47, u8 0x42+v` when the stamp finishes.
All text is the client's own string pool (`0x17E0`..`0x17ED`).

Classic World's town-membership system, and **it is the thing the shop data's rank tags
were waiting for.** Pick Henesys or Kerning City, work a community board, climb ten grades
that unlock town shop items, discounts, storage rates and eventually housing.

##### This is the best-corroborated spec in the project, and that is measurable

The owner's source is a fan site drawn from the second closed online test - the same "COT2" the
shop data is labelled with. Normally that would make every number **[I]**. It does not here,
because **the client ships the same data and it agrees**.

All **88** citizenship quests exist in this client's own `Quest.wz`, ids `506000`-`506141`,
and the count matches the site's exactly. Quest `506001` ("First Greeting with Rina")
checks field for field:

| | client `Quest.wz` | the site |
|---|---|---|
| level gate | `Check.0.lvmin = 12` | Lv 12 |
| town | `Check.0.citizenshipTown = 1` | Henesys |
| grade gate | `Check.0.citizenshipGrade = 1` | grade 1 |
| the board | `Check.0.npc = 235` | Community Board (Henesys) |
| the resident | `Check.1.npc = 201` | Rina |
| EXP | `Act.1.exp = 321` | 321 |
| mesos | `Act.1.money = 351` | 351 |
| reward | `Act.1.item.0 = 2010004 x5` | Orange x5 |
| contribution | `Act.1.citizenshipContr.amountFormula` | "100 at grade 1, +50 per grade" |

That last row is the striking one: the client carries the rule as a **literal formula
string**, `100 + ( ( citizenshipGrade - 1 ) x 50 )`, in 36 quests. **[L]**

##### What the client's own data establishes

| | |
|---|---|
| `Check.N.citizenshipTown` | 86 uses, values **1 and 2** only - 45 quests for town 1, 41 for town 2 |
| `Check.N.citizenshipGrade` | 86 uses, values **1..9** as gates |
| `Act.N.citizenshipContr.town` | 86 uses - which town banks the contribution |
| `Act.N.citizenshipContr.amount` | 50 uses, flat: 50, 80, 100, 180, 500, 750, 1000, 1250, 1500, 1750, 2000, 2250, 2500 |
| `Act.N.citizenshipContr.amountFormula` | 36 uses, the formula above |

So **contribution is awarded through `Act.citizenshipContr`**, with either a flat `amount` or
a formula, and it is gated by `Check.citizenshipTown` + `citizenshipGrade`. All **[L]**.

##### The grade table

Levels, contribution thresholds and the three discount columns are **[I]** from the site -
none of them appears in `Quest.wz`. The names are corroborated: they are exactly the rank
tags in `data/shops.txt`.

| grade | name | Lv | contribution | shop disc. | storage disc. | storage fee/item |
|---:|---|---:|---:|---:|---:|---:|
| 1 | Traveler | 12 | from the quest | 5% | 5% | 95 |
| 2 | Visitor | 17 | 1,000 | 7% | 10% | 90 |
| 3 | Helpful Stranger | 22 | 2,000 | 9% | 15% | 85 |
| 4 | Recognized Guest | 27 | 3,000 | 11% | 20% | 80 |
| 5 | **Town Resident** | 32 | 4,000 | 13% | 25% | 75 |
| 6 | Trusted Neighbor | 37 | 5,000 | 15% | 30% | 70 |
| 7 | Distinguished Citizen | 42 | 6,000 | 17% | 35% | 65 |
| 8 | Town Patron | 47 | 7,000 | 19% | 40% | 60 |
| 9 | Guardian of the Village | 52 | 8,000 | 21% | 45% | 55 |
| 10 | Citizen of Honor | 57 | 10,000 | 25% | 50% | 50 |

Grade 5 unlocks VIP dailies from the town leaders - 3,487 EXP against 321 for a regular
daily, and both numbers appear in the client's quest data.

##### THIS UNBLOCKS THE SHOP DATA

`data/shops.txt` records rank tags exactly as the live UI shows them - `Visitor+`,
`Town Resident+`, `Guardian of the Village+` - and its header says the mapping to grade
numbers "is NOT established". **It is now**, and it is just the table above: the tag is the
grade name, `+` means that grade or higher. That turns every gated shop row into an integer
comparison.

##### The rest of the rules, all [I] from the site

* Sign with **Arthur** (Henesys Town Hall) or **Roxy** (Kerning City Civic Center). One town
  at a time; switching banks the old town's progress and restores it on return. First move
  free; reactivation 50,000 mesos at grade 1, other grades unconfirmed **by the site itself**.
* Dailies: `First Greeting` runs once, `Asking After` repeats. Weekly asks for 100 of one
  monster's ETC drop and pays 500 contribution at grade 1, +250 per grade.
* Citizen discounts apply in the **active** town, on tagged items only - buff potions, pet
  food, return scrolls, the town cab.
* Resident's Chair: 10,000 mesos from the furnishings store at Visitor, **trade-blocked**,
  restores 20 HP / 5 MP per 10 seconds seated, one per town. Both chairs are already in
  `data/shops.txt` (Oak, Weston) tagged `Visitor+`.
* Town Earrings: Lv 57, awarded at Citizen of Honor, untradeable.
* Ten NPCs per town greet with different dialogue as the grade rises - which is the same
  machinery as goal B's idle chatter, keyed on grade.

##### Dependencies

Blocked on **D (level up)** for the level gates to mean anything, and it shares the quest
machinery with **A**. But the quest data is already extracted and the gates are already
expressible: `crates/store` needs a per-character `(town, grade, contribution)` and
`crates/world` needs to honour `Check.citizenshipTown`/`citizenshipGrade`, both of which are
small next to what is already built.

#### I. The bag has to persist - **DONE**, confirmed across relaunches

The owner, after the first successful unequip: *"when I travel to map 40, the item I un-equipped
re-equipped itself. This is not correct, items taken off should persist as is during
transitions from map to map."*

**Today the unequip works on screen and nowhere else.** `0x0107` is answered with a `0x0070`
and the client moves the item, but the server writes nothing down: the equipped list in the
character record is still built from `crates/store`'s `equipment` rows, so the next
`SetField` - a portal, a `!map`, a relog - puts the item straight back on.

That was a deliberate choice with the wrong ceiling on it. The alternative available at the
time was to delete the `equipment` row, which would have left the item **nowhere at all**,
because nothing stores bag contents. "Comes back" beat "is gone". The owner's answer is that
neither is acceptable, and they are right - the move has to survive the transition.

##### Two halves, and the second is the real work

**1. Somewhere to put it.** An `inventory` table keyed by character - `(character_id,
inv_type, slot, item_id, count)` - and `on_inventory_move` writing to it: delete the
`equipment` row, insert the inventory row. Small, and it follows the idioms already in
`crates/store/src/character.rs` and `quest.rs`.

**2. Getting it back onto the wire, which is not decoded.** The character record's
`presence[2]` region ends with four `u16` terminators that this server sends as zero:

```rust
b.extend_from_slice(&0u16.to_le_bytes()); // end of the equipped list
b.extend_from_slice(&0u16.to_le_bytes()); // FUN_14030b6f0
b.extend_from_slice(&[0u8; 6]);           // FUN_14030b9e0, three lists
```

Those four are **the bag inventories**, sent empty. `research/naked-character.md` established
that setting `presence[2]` opens three list readers rather than one, and that omitting the
terminators desynchronises everything after them - but nothing has read what a **non-empty**
list looks like. Until that is decoded, an item can be stored in the database and still not
appear in the bag after a field entry, which from the player's side is indistinguishable from
losing it.

Existing material to start from, not yet worked: `research/msexe-invlist-b6f0.txt`,
`research/msexe-invlist-b9e0.txt`, `research/msexe-invkey-23d0.txt`, and the equipped-item
blob decode in `research/msexe-setfield.md` §"The item blob", which gives the **bundle**
(`FUN_140304450`), **pet** (`FUN_140304550`) and **equip** (`FUN_140304100`) shapes - the
same blobs a bag list must carry.

##### It is the same table three goals are waiting on

**F (NPC shops)** cannot sell into a bag that does not exist, **G (storage)** is a second
container needing the same item representation, and this is the first. Whoever does it should
know they are unblocking all three, and should design the table for all three rather than for
the unequip alone.

##### The one thing already proven

`0x0070` InventoryOperation mode 2 moves an item on screen, confirmed by the owner. So the
*outbound* half of any inventory change is settled - `crates/net/src/inventory.rs`. What is
missing is durability and the record block, not the ability to tell the client.

#### J. The damage formula - **DECODED, physical and magic; no validator is wired**

The owner: *"an integral part of our server"*, with three links. All three are captured in
**`research/meowdb-combat-formulas.md`** - the formulas, the weapon multiplier table, the
accuracy and avoidability model, the defence order, the crit rule and the site's own list of
what it could not resolve.

**Today the client computes its own damage and the server accepts it.** That is measured, not
assumed: `research/mob-combat.md`. So this goal is not "make damage work" - damage works. It
is **making the server the authority**, which matters the moment anything is meant to be
balanced or resisted.

The one thing to know before starting: the formula needs `TotalWATK`, mastery, the weapon's
type and the four stats, and **the server already has all of them** - equipment carries real
`Character.wz` stats (`research/equip-stats.md`) and the stat block is decoded. What it does
not have is a notion of *which attack* was used, which is in the attack packet nobody parses
past its damage numbers.

Do the **[I]** labelling honestly here. The site is a fan site with a good prior on this
client - see the note at the top of that file - not a listing.

#### K. HP and MP per level - **DONE 2026-08-20**, and the client cannot check it

| class | HP / level | MP / level |
|---|---|---|
| Beginner | +16 | +12 |
| Warrior | +28 | +12 |
| Bowman | +22 | +17 |
| Thief | +22 | +17 |
| Magician | +16 | +22 |

Plus a fixed 500 points split at job advancement, and +25% of base from the maxed Improving
Max HP / Max MP skills. Full table and the job split in `research/meowdb-combat-formulas.md`.

**This contradicts what MapleCW ships.** `crates/world/src/expcurve.rs` gives every character
`LevelGains { ap: 5, max_hp: 14, max_mp: 10 }` at every level. A Beginner should be getting
**+16 / +12**.

**Neither number is measured.** Ours was a placeholder; the site's is a fan site.

**And the client will not settle it, which this entry claimed yesterday that it would.** That
claim was an inference and it is now checked: the EXP curve at `0x143AC2400` has **no sibling
table**. A `lea reg,[rip+disp32]` scan of the whole 0x500-byte neighbourhood that fills it
finds exactly one `.data` target, and it is the EXP curve itself. That fits what the client is
for - it draws the EXP bar, so it needs the EXP table; it is *told* new maxima on level up and
never computes them, so it has no reason to carry HP/MP per level at all.

The blind spot, named rather than left implicit: that scan cannot see a table reached through
a register, through `mov reg, imm64`, or from `.themida`. So this is "no evidence of one in
the obvious place", not "provably absent".

Which leaves the fan site's behavioural measurement as the best evidence available, and it
is written down **as that** - a measurement of the live COT2 service, not a listing.

**Implemented.** `LevelGains::for_class` carries all five lines and `ClassLine::of_job` keys
them on the hundreds digit of the job id, which is `[L]` from this client's own SP fork
masks. A beginner now gains **+16 HP / +12 MP** instead of +14 / +10, and since job
advancement is goal E, that is what every character on this server gets. AP stays at five and
is ours, not theirs.

Two things deliberately left out, because neither has anything to hang off yet: the 500-point
**job-advancement bonus** and the **+25% from maxed Improving Max HP/MP**. Both are in
`research/meowdb-combat-formulas.md`. Adding an unwired table for them is the failure mode
`CLAUDE.md` calls "built is not wired".

Pirate (`500`) has no row in the source and falls back to the beginner line. That is a
placeholder and is labelled one - nothing can reach it today.

Five constants and a job split. It is the smallest of the three and the only one that changes
something a player would notice today.

#### L. Attack speed and animation timing - set by the owner, 2026-08-20

Captured in `research/meowdb-combat-formulas.md`, and the honest summary is that **the page
does not carry the table**. It gives the model - timing quantised to **30 ms steps**, four
labels (base animation, animation time, weapon speed, timing range), Weapon Boosters worth two
speed stages at every level, Spell Booster worth one at Lv1-10 and two at Lv11+ and not
available to Clerics or Priests - and then points at per-skill pages for the numbers.

**The server does not check attack intervals at all today**, and on a single-player local
server it does not need to. This goal is here because the owner asked for it and because the
numbers have to come from the client's own WZ rather than that page if they are ever wanted.
Lowest priority of the three, and say so rather than quietly leaving it out.

### RUN OF 2026-08-19: the equipped list DECODED, and the mob body kills the client

The owner entered the world with `TestCharD` on **map 40** and the client exited immediately.
Preserved as `research/fixtures/mob-body-faults-client-{world,hook,exit}.log`.

**This run answered three questions and only one badly.**

```text
14:27:49.788  140302e30 x3  from 0x1403094d0   character select - THE POSITIVE CONTROL
14:27:52.421  140302e30     from 0x140304e76   while dispatching 0x01A0: the record decoded
14:27:52.421  140304100 x4  from 0x140309686 <- 0x140306223 <- 0x140309f70
14:27:52.951  opcode=0x044F  dispatched
14:27:52.952  opcode=0x044F  dispatched
14:27:52.952  CLIENT FAULT 0xC0000005 at 0x141c810b0
```

**The equipped-item block works.** `140304100` is the type-1 equip decode at `vtable+0x358`,
and it fired **four times** - once per stored item - from inside the equipped-list loop at
`0x140306223`. Its `rcx` points at an object whose first qword is `0x14327E1D8`, the type-1
vtable, exactly as `research/naked-character.md` predicted. So `presence[2]`, the block's
position at record offset 223, the `u16` slot loop and the 125-byte item body are all
**right**, and the record decoded past them into the NPC dispatch.

**What is still unknown is whether the character LOOKS dressed**, because the client died
before the owner could see it. Decode confirmed; appearance not.

**The mob body faults the client, and the fault names the field.** `0x141c810b0` is
`+0x70` into `FUN_141c81040`:

```asm
141c81094  MOV   RAX,qword ptr [RSI + 0x2b8]
141c8109b  MOV   EDX,0x848
141c810a0  TEST  RAX,RAX
141c810a3  LEA   RCX,[RAX + 0x828]
141c810aa  CMOVE RCX,RDX              ; RAX == 0  ->  RCX = 0x848
141c810ae  XOR   EDX,EDX
141c810b0  CMP   qword ptr [RCX],RDX  ; <-- faults reading 0x848
```

So **`mob+0x2b8` was null** and the client dereferenced it with no guard. It died on the
**first** `0x03C6`, after both `0x044F` NPCs had dispatched cleanly - so mobs are the only
thing that changed the outcome, and the other three builds are not implicated.

This is the failure `research/mob-spawn.md` ranked **first**: the body is structurally wrong
somewhere, and the WZ-template-driven blocks (`template+0x104` adds 16 bytes, `+0x1a0` adds
4) are the leading suspect, because a wrong length there misreads every field after it -
which is exactly how a pointer field ends up null.

> **Next instrument:** find who *writes* `template+0x104` and `+0x1a0` and read the WZ
> property name beside it, or dump `Mob.wz` template 2 directly and look for the properties
> that would set them. Then find what assigns `mob+0x2b8`, which names the field that did
> not arrive.

**Mobs are OFF by default now** (`send_mobs`, `--mobs` to re-enable), so the next run gets
the other three builds in front of the client without waiting for this. The world server
prints a line saying so on startup.

### Mob capacity: a spawn point is not a mob

The owner, 2026-08-19: map 40 has 40 spawn points but a real server keeps about **30** alive on
it, and the cap rises with player count.

**Checked in the WZ, and it is not there.** Map 40's whole `info` node is `AmbientBGM(v)`,
`MR*`/`VR*`, `bgm`, `cloud`, `fieldLimit`, `fieldLimit2`, `fieldLimit_tw`, `fieldScript`,
`fieldType`, `fly`, `forcedReturn`, `hideMinimap`, `mapDesc`, `mapMark`, **`mobRate`**,
`moveLimit`, `noMapCmd`, `onFirstUserEnter`, `onUserEnter`, `partyStandAlone`,
`personalShop`, `quarterView`, `returnMap`, `standAlone`, `swim`, `town`, `version`. **No
capacity field of any name.** `mobRate` is `1.0` and is a respawn *rate*, not a cap. Its
`life` node has 42 entries: 40 mobs plus Robin and Sam, which also confirms the generated
table.

So the cap is **server policy**, and the owner adopted the fan site's rule blanket on
2026-08-19: **75% of the spawn points below six players on the field, 100% at six or more**,
with nothing in between. `world::config::spawn_capacity(points, players)`.

**[I], and adopted knowingly** - the owner flagged the source as an unofficial fan site themself.
Nothing in this client corroborates it. Two things are still unsettled and are written down
rather than smoothed over:

* **The rounding.** The one datapoint is 40 -> 30, which both flooring and rounding up
  reproduce. The code floors. A small map is where they differ: 6 spawn points give 4 by
  flooring and 5 by rounding up.
* **The threshold's edge.** The site's columns are labelled "Solo" and "6+ players", so
  `CROWD_THRESHOLD` is **six or more**. The owner's wording was "more than 6". If strictly more
  was meant, that constant is the single number to change.

**The crowded branch is written and untaken.** `players` is the count on the *field*, and it
is always `1` today because this server has no field-occupancy tracking at all. It is a
parameter rather than a constant so that adding occupancy is a change at the call site.

The full spawn-point list stays intact in `Config::mobs`; the cap is applied where they are
*sent*, because respawn will need the points that are not currently filled.

**And which points are filled matters as much as how many.** The owner, 2026-08-19: *"on maps
with multiple mobs, there's a concept of shares, the map will try to maintain the balance
ratio between the mobs under the cap."* Taking the first N in WZ order is wrong and was the
first thing written here - the generated table is grouped by spawn index, so on The Field
South of Ellinia (45 spawns: Snail 10, Blue Snail 16, Shroom 7, Red Snail 6, Orange Mushroom
6) the first 33 rows **miss a whole type**. `config::share_balanced` apportions the cap by
**largest remainder**: `floor(count * cap / total)` each, leftovers to the largest
remainders, ties broken by template id. For that map at cap 33 it gives 7 / 12 / 5 / 5 / 4,
every type within one slot of its exact share. A test pins both the split and the fact that
the naive version drops a type.

**[I] again, and only the shape.** That the engine balances by share is the owner's, from the
same fan site as the capacity scalar, and the exact rounding the real engine uses is
unknown. What the code guarantees is that the result is capped, proportional and stable
between runs.

**Not yet built: respawn.** A share-balanced *refill* when a mob dies is the same rule
applied over time, and nothing here kills mobs yet.

### THE RUN of 2026-08-19 — STRUCK 2026-09-04, every step of it is answered

> Six steps stood here and all six closed long ago: a dressed character enters the world, the
> tooltips read their stats, **mobs are on** (the `send_mobs = false` in step 2 was reverted the
> same week - `!map 30` is ordinary now), both NPC click paths answer, the Change Channel row
> goes cream then blue, and `0x00D2` is answered and migrates on `0x001A`.
>
> It is struck rather than deleted because of what it got right about *method*: every step named
> **what each outcome would mean** before the launch, which is what turns a run into a
> measurement. The two rules under it have each cost a run and are kept.

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

### 2. NPCs have dialogue when clicked - **DONE 2026-08-19, confirmed on screen**

NPCs speak the game's own lines on **both** click paths, quest NPCs open the real quest
dialogue, and pressing **Accept** answers with the quest's `yes` branch. What is *not* done
is quest **state**: nothing persists, so the same conversation is available every time. That
is goal A.

*Original notes below - the identification is still the reference.*

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
> resync point. **Text on screen is still not a quest** - no quest-result packet has been
> found, so state will not advance.

### 2f. THE MOB WATCH FIRED, and it says there are two objects

Run of 2026-08-19 with `-Mobs -MobLimit 1`, one mob on map 40 (`template 2, object id 2000,
hp 45, 137 bytes`). Client died on entering the world. Preserved as
`research/fixtures/mob-watch-2b8-null-second-object-{hook,world}.log`.

```text
WATCH #1: 0x141c81040 ENTERED while dispatching opcode 0x03C6
  rcx=0x382a9760 [0x43407950]  [rcx+0x2b8]=u32:0x00000000  called-from=0x1409c687d
  stack: 0x1409c687d 0x142f0492d 0x140c79143 0x142ac10d0
         0x142ac1128 0x142f0492d 0x141c81297 0x142ac10d0 0x141c50da8
CLIENT FAULT #1: 0xC0000005 at 0x141c810b0
```

**`mob+0x2b8` is confirmed null** - the diagnosis holds, and the watch did in one hover what
a direct-call graph could not do at all.

**SOLVED. There is no second object, and the field named here is the wrong one.** See the
correction immediately below; the reading in this section is kept because the *measurement*
is what solved it.

**What this looked like at the time:** `0x141c50da8` is inside
`encodeInit` and **past** the `+0x2b8` assignment at `0x141c50c9c`. So `encodeInit` is running
and has already executed that assignment - for *some* mob - while the mob in `rcx` still holds
the constructor's zero. The reading that fits is **`encodeInit` initialising mob A reaches
`FUN_141c81040` on mob B**, which has not been initialised yet.

**Two more discriminations from the same line:**

* **The vtable low dword is `0x43407950`, and it is not one of the eight** the static pass
  predicted (`0x434077e8`, `0x4341f548`, `0x4341e510`, `0x4341e758`, `0x433765c0`,
  `0x4341ede0`, `0x433767f0`, `0x4341eff0`). Either that set is incomplete or `rcx` is not a
  mob at all. The claim it rested on - that the eight aligned `.rdata` qwords holding
  `FUN_141c81040` *are* the eight mob vtables - now needs re-checking.
* **The second watch never fired.** `141c532ab:peek=24` produced no line, so the body read at
  offset 107 never happened. That does not contradict the above; the fault simply came first.

`called-from=0x1409c687d` is `FUN_1409c50a0 + 0x17dd` (7184 bytes). `0x142f0492d` appears
**twice** in the chain, which has the shape of a trampoline or dispatcher, and `0x141c81297`
is past `FUN_141c81040`'s own `.pdata` end - so it is a different function, plausibly the
same virtual on a different class.

> **The shape worth looking at first**, because it is the only category that both fits "our
> packet cannot make `+0x2b8` null" *and* explains a second object appearing: a **value** in
> our body steering control flow. The 35 unconditional reads are verified identical to ours,
> so the layout is right - but `summonType` is `1` in both reachable templates' WZ while we
> send `appear_type = -2`, and the head carries two `u8`s whose meaning is unread. If one of
> them makes the client treat this as a summon **with a parent**, mob B is the parent it goes
> looking for and does not have.

**Mobs remain off by default.** Nothing about the body has been changed, deliberately: it is
the one part verified three ways, and changing it now would confound the only measurement
that has ever discriminated here.

### 2g. THE MOB CRASH, SOLVED - and it is one byte we send

**`0x143407950` is not a ninth vtable. It is the mob's own second base**, written by the
constructor at `mob+8` alongside `0x1434077e8` at `mob+0` and `0x1434079c0` at `mob+0x10`.
`FUN_141c81040` is slot 1 of that table, so `rcx = mob + 8` - and **`[rcx+0x2b8]` is
`mob+0x2c0`**, not the `mob+0x2b8` the whole first pass analysed. Two independent locks agree
on `this = mob+8`: `FUN_141c76190` and `FUN_141c81040` assert the *same* id `0x431` on
`mob+0x3c8` and `this+0x3c0`, and `encodeInit` reads the template at `[rsi+0x3a8]` where
`FUN_141c81040` reads it at `[rsi+0x3a0]`.

`mob+0x2c0` has exactly two writers: the constructor's zero, and **`141c50eed` in
`encodeInit`** - a *second* interface object, created at `141c50e77`. So `+0x2b8` and
`+0x2c0` are a **pair**, built `0x1c8` bytes apart, and the callback lands **between** them.

**The lever is `move_action`, body offset 35, and we were sending 0.** That byte is
`action*2 + facing` - `encodeInit` splits it twelve instructions later, `AND EDI,1` for
facing and `SAR EAX,1` for the action, into a 16-entry jump table. The chain:

```text
offset 35 = 0  ->  obfuscated into mob+0x3dc/0x3e0
141c50cfd  EDI = ROL([mob+0x3e0],5) XOR [mob+0x3dc]      the plaintext back
141c50da5  CALL iface->vtable[0x118], EDI as arg 7        <-- DOMINATES the tail reads
1409c6858  TEST EBX,0xfffffffe                            <-- only action == 0 falls through
1409c687a  CALL [mob8_vtable+8] = FUN_141c81040 -> mob+0x2c0 is still null -> 0x848 -> dead
```

`TEST EBX,0xfffffffe` is the whole answer: **any action >= 1 skips it.** The call at
`141c50da5` dominates the tail, so the callback itself is unavoidable - only the value is a
lever. `net::mob::MOVE_ACTION_MIN_SAFE` is `2`, `FieldMob::new` defaults to it, and both a
unit test and the smoke test pin it.

> **The pass condition on the next run:** **no `141c81040` line at all**, and `141c532ab`
> firing with cursor **`0x71`**. If the stance looks wrong on screen, `3` is the same action
> facing the other way and `5` is action 2 - all of them skip the fault, so that is a *look*
> question, not a crash question.

**An instrument correction that matters beyond this.** The probe's `stack:` line is a
**heuristic scan of the stack for code-shaped values, not an unwind.** In the capture,
`0x142ac10d0` is a function *start*, which no return address can be, and `0x140c79143` is the
return of an indirect call in an unrelated function - both stale. **Only `called-from=` is
exact.** The "two objects" reading in §2f was built partly on those frames.

**And the first pass's own retraction, which is the useful part:** it blamed `mob+0x2b8`,
proved from a dominator test that no packet byte could make it null, and was right about that
field and wrong about which field was being read. The tell was a "slot 46/47/48" wobble it
noticed and explained away instead of chasing - offsets into *secondary* vtables.

### 2e. RUN OF 2026-08-19 (evening): what it settled, and the best lead yet on unequip

Preserved as `research/fixtures/stats-work-then-map-loses-them-world.log`.

**Working on screen, confirmed by the owner:** idle chatter (Robin cycling their lines), item stats
on the tooltips, the `!map` chat notice for both a bad id and a good one, and **Log Out**
returning to the login screen.

**The tooltip investigation was chasing my own bug.** Items had correct stats on entering the
world and lost them the moment `!map` was used, because `go_to_map` still sent the *bare*
record. Every earlier tooltip observation was made on map 40, reached with `!map`. There was
no second object.

**World select cannot change channels, and that hypothesis is dead.** The owner: *"So I actually
cannot change channel via world select. This client doesn't implement that. When I go to
world select, the moment I choose 'Classic', I am redirected to the classic login screen with
my masked email."* So the run where CH.1 and CH.2 listed is still unexplained, and **all
three** attributions so far - the four trailing bytes, the enable byte, the world-select
route - are retracted. Whatever populates that list, nothing yet proposed does it.

#### The unequip still never reaches the wire, and there is now a strong candidate why

The owner tried repeatedly to unequip the Undershirt. **The capture contains no `0x0107` at all**,
confirming `research/npc-click.md` §4: the client drops it inside `FUN_142cc5b00`, before
building anything, at one of six pre-send gates.

> **Leading hypothesis: the bag has no slots.** An unequip needs somewhere to put the item,
> and until 2026-08-19 the server never told the client how big any inventory was.

**BUILT the same day, and the hypothesis above was right about the symptom and wrong about
every detail of the mechanism.** The full working is `research/inventory-slots.md`; what
this section had wrong is worth keeping, because the two errors are the two this project
keeps making.

* It said the counts sit at **record offsets 219-222**, in the bytes labelled "one `u8` and
  three optional-string flags". **They do not.** They are at offset **223**, in a block that
  did not exist in the layout at all, switched on by `presence[7]`. The old labelling of
  219-222 was never wrong - it just was not the thing being looked for.
* It said there are **five**. There are **six**: the loop's trip count is the literal
  `MOV R15D,0x6` at `0x140305def`.

Both errors came from the same place - reasoning from the reference server's shape instead
of reading this client's loop - and both were the *reference being right about the concept
and wrong about the numbers*, which is exactly the failure `CLAUDE.md` scores it 1 of 8 for.
Ten minutes on the listing gave the right answer, and the listing had been sitting in
`research/msexe-charrecord-full.txt` the whole time.

> **What is still open** is the part that mattered: whether a null inventory array is why
> the unequip never reaches the wire. The arithmetic is measured - a null array makes the
> default slot count `-1`, and `CMP dword ptr [RSP+0x60],0x0 / JL` at `0x140305f09` then
> skips that inventory's whole slot walk - but **whether these arrays start null has not
> been read**, and it lives in a constructor nobody has opened. **[I]** with a mechanism.
>
> **The one-variant test: `-InventorySlots 10`.** Go UNDER the default, not over. The owner's
> inventory window is **six rows of five** with a scrollbar, so at any number of 30 or more a
> fixed viewport and a real slot count look identical on screen. At 10 they do not.
>
> **And the screenshot already weakens the hypothesis**: those thirty cells were drawn in a
> session where the server sent no sizes at all. Either the arrays were never null - in which
> case this explanation is wrong and should be dropped - or the grid is a viewport and the
> screenshot says nothing either way. The run at 10 is what separates them.

#### And a client state dump nobody had seen: `0x0420`-`0x0426`

Six packets arrive together, once, ~7 minutes in. `0x0421` is **1115 bytes** and carries the
character id (204), the name `TestCharD`, and **our four item ids** - `1040003`, `1060002`,
`1072003`, `1302000` - in equipped-slot order. `0x0420` carries a timestamp string and the
**NPC object ids we assigned** (1000, 1001, 1002). `0x0426` is 20 bytes beginning
`ffffffffffffffff`.

This is the client reporting its own world state back, and it is a **free read-back
instrument** of exactly the kind `research/equip-stats.md` wished for: it says what the
client thinks it is wearing, in its own words. Nobody has decoded it. None of the six is
answered and none has caused a freeze.

### 2d. The unequip request never reached the wire - and the gate that ate it is a hazard

The owner tried to unequip the Undershirt on 2026-08-19 and was not sure whether anything was
sent. **Nothing was.** The move request is outbound **`0x0107`** - `FUN_142cc5b00(player,
invType, src, dst, count)`, body `u32 tick, u8 invType, i16 srcSlot, i16 dstSlot, i16 count`,
11 bytes, with **negative slots meaning equipped** (read, not assumed: `test r14d,r14d; js`,
`cmp r14d,-0xb`, `cmp edi,-0xb`). Their action would have been `invType=1, src=-5`. It is not
in the capture and nothing else in that session has the shape - every other captured opcode
is accounted for against `research/msexe-packet-fields.txt`. The client dropped it inside
`FUN_142cc5b00`, before building anything, at one of six pre-send gates.

> **And one of those gates is an "always answer" landmine that has nothing to do with this
> bug.** `player->[0x2330]` is a **one-request-outstanding latch**: set immediately after the
> send at `142cc5f01` by **37** functions across the image, and cleared by only **7**, *all
> of which are inbound packet handlers* - `FUN_142cc52a0` is literally
> `read(packet); [this+0x2330] = 0`. So if the server ever fails to answer one of those 37
> requests, **every later request in that class is silently dropped** - no dialog, no freeze,
> nothing in any log. This is not what happened here (none of the 37 setters appears in the
> capture), but it is the mechanism by which a single unanswered packet turns into "the
> inventory stopped working" three minutes later.

**Next instrument, and it needs no new code:** `WATCH` on `0x142cc5b00` plus its two exits
`0x142cc5c16` (bail) and `0x142cc5ea3` (`mov edx,0x107`). One drag of an item distinguishes
"the UI never asked" from "a gate blocked it" from "it sent and the capture is wrong".

### 2c-ii. Two NPC-click packets, and the server answered the wrong one - **FIXED, unconfirmed**

The owner clicked Robin on map 40 and nothing happened. The capture has **no `0x0151` at all**;
the client sent **`0x00F2`** and nothing answered it.

**Which one goes out is decided inside the client, from `Quest.wz`.** `FUN_1428de280` forks
on `FUN_141e39b50(npc)` - whether the NPC has a non-empty script name, a string the *client*
fills at construction from its own quest singleton - and only a menu line carrying a quest id
in `npc->[0x200]/[0x208]/[0x210]` reaches the `0x0151` builder. **Every other outcome sends
`0x00F2`.** Robin (template 8) has no quests. The server does not select the path and cannot.

`0x00F2` is `u32 npcObjectId, i16 charX, i16 charY, u32 tail`, 12 bytes. Answered now with
the same `0x055B` Say. Full working: `research/npc-click.md`.

**The trap in answering it**, which would have produced an unexplainable failure: `0x0151`
hands over the NPC's **template** id and `0x00F2` hands over the **object** id we chose,
while `0x055B`'s speaker field wants a template. The handler maps back through the table
that assigned it, **scoped to the character's current map**, because `config::load_npcs`
restarts the numbering on every field. Sending the object id straight through would not
fault - the loader result is null-checked at `142a7b52a` - it would just draw a box with no
portrait.

**Two corrections to the first reading of that capture**, both from the client's own builder
rather than from the numbers lining up:

* **Field 1 really is our object id** - `[npc+0x190]`, written from `NpcEnterField`'s
  objectId by the `CNpc` constructor at `141e35f59` and by the body decoder at `141e36b73`.
  The code reads back the field the spawn packet wrote. That distinction matters, because
  the *previous* "read off our own data" claim here was retracted for exactly the reason it
  invites: every map's first NPC is given object id 1000, so a `1000` proves nothing alone.
* **Fields 2 and 3 are the CHARACTER's position, not the NPC's.** The first reading called
  `275` Robin's `cy`; it is the player's `y`, and Robin's `cy` is 275 only because the player
  was standing on the same ground line. All four build sites read the local-user singleton,
  and the same pair appears verbatim in both `0x00D9` packets of the same capture.

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

### THE CHANNEL LIST IS NOT ABOUT THE PACKET AT ALL - 2026-08-19, from two captures

**Both of my explanations for this dialog were wrong, and the world list was never the
variable.** Comparing the captured `0x000B` bodies settles it: the world-list body from the
run where **CH.1 and CH.2 were listed** and the body from the run where the dialog was
**empty** are **byte-identical**.

```text
run WITH channels listed  0006005363616e6961000000000208005363616e69612d30...0001000000000000000000
run with an EMPTY dialog  0006005363616e6961000000000208005363616e69612d30...0001000000000000000000
```

Both are `research/fixtures/world-select-0076-login.log` and today's `login.log`. Same world
id, same name, same channel count, same four trailing bytes per entry. So no value in that
packet decides whether the dialog has rows.

**What differs between the two runs is the route through the login screens.**

| capture | `0x0076` world-select requests | Change Channel |
|---|---|---|
| `world-select-0076-login.log` | **2** | **CH.1 and CH.2 listed** |
| today | **0** | **empty** |

The owner reached character select by auto-login today and never opened world select. In the run
that worked they had clicked "Choose another world" and picked Classic.

**That fits the static reading exactly, and rescues most of it.** `research/channel-select.md`
established that the rows are drawn from an `int` array at `singleton+0x2cc8`, filled by
`FUN_142cb8e10`'s sixth argument. Nothing in that chain is contradicted - what was never
checked is **who calls `FUN_142cb8e10`, and when**. If the world-select screen is what runs
it, then a client that skips that screen never populates the array, and the dialog is empty
no matter what the world list said.

> **The test costs nothing and needs no code change.** On the next run, click **"Choose
> another world"**, pick **Classic**, then enter the world and open Change Channel. If the
> rows are back, the answer is "world select populates the array" and the server owes the
> dialog nothing. If they are still missing, the route is not the variable either and the
> next instrument is a watch on `FUN_142cb8e10`.

**What this cost, and the lesson.** Two client runs were spent on a byte that never mattered:
first `[world_id, index, 0, 0]`, which was credited with fixing the list, and then the enable
byte, which was blamed for emptying it. The first "fix" was almost certainly a coincidence -
it landed in the same run as the world-select work - and the second was a coincidence in the
opposite direction. **A change and an observation in the same run are not a measurement**,
and neither of those runs changed one variable.

### SOLVED 2026-08-19: the predicate is the 4th trailing `u8`, and we were sending 0

Full working, every link read off the listing: **`research/channel-select.md`**.

**`FUN_142cb9510`** is a 15-byte leaf that returns `arr[i]` from the per-channel `int` array
at `singleton+0x2cc8`. It has six callers - Draw, the mouse hit test, the Change button and
three keyboard navigators - and **all six test the result with a bare `test eax, eax`**.
There is no enum and no magic value: **any non-zero int enables the row.** **[L]**

The chain from the wire:

```text
4th trailing u8 of the channel entry
  -> chan+0x18         FUN_141b2fac0, the login case 0xb - four sequential u8 reads into
                       +0x0c, +0x10, +0x14, +0x18
  -> vecB[i]           FUN_141b2c7c0 at 141b2ca89
  -> singleton+0x2cc8  FUN_142cb8e10, argument 6, at 142cb8ed9
  -> FUN_142cb9510(i)  must be non-zero
```

The mouse gate at `142a31810` takes `je` on zero, which is exactly "the click produces
nothing on the wire" - what the owner measured when they clicked CH.2 and the whole capture
contained no new opcode.

**RETRACTED 2026-08-19 by the run.** The one-byte fix - the channel entry ending
`[world_id, i, 0, 1]` instead of `[world_id, i, 0, 0]` - **emptied the dialog**. The owner: *"On
change channel UI, I'm back to no channels being shown again, it's completely blank."*

The count reached the client either way: that run's `login.log` has `channel 0 advertised`,
`channel 1 advertised`, `world Scania id 0 with 2 channel(s)` and the `0x000B` world list
going out. So the byte emptied a list that populated without it.

| 4th trailing `u8` | what the owner saw |
|---|---|
| `0` | CH.1 and CH.2 both listed. CH.2 grey, and clicking it sends **nothing at all** |
| `1` | **no channels listed** |

`crates/net` is back to `0`. Every individual link in the derivation still reads correctly -
`FUN_142cb9510` is a leaf returning `arr[i]`, its six callers all test with a bare
`test eax, eax`, the mouse gate `je`s on zero. What is falsified is the **end-to-end** claim
that the 4th wire byte arrives in that array as an enable flag: something between the
decoder and the array drops the entry when it is non-zero.

> **The next instrument is upstream, not downstream.** Read what `FUN_141b2c7c0` does with
> `chan+0x18` **before** it becomes `FUN_142cb8e10`'s argument 6, looking for a test that
> **skips the entry**. The failure mode to search for is "the list is built shorter", not
> "the row is drawn grey" - and nothing in `research/channel-select.md` looked for that.

**Which row is grey, confirmed rather than assumed.** The dialog loads four sibling canvases
`channel0..channel3`, all 68x20 - exactly the hit rectangle (`lea eax,[r9+0x44]` /
`lea eax,[rcx+0x14]`). Rendered from `_Canvas_000.wz`: `channel0` cream (normal),
`channel1` grey (current), `channel2` blue (selected), `channel3` grey (disabled). OnCreate
sets `selection := current` and Draw checks selection first, so **CH.1 draws blue**. The
current channel *is* excluded from clicking (`142a31803`), but that is not the cause -
**CH.2 is the grey one, and it is grey for the disabled reason.** The two greys differ by
about six RGB points, which is why they cannot be told apart by eye.

**`userCount` is ruled out, and the old advice here was wrong.** This section used to
propose a non-zero user count as the cheapest experiment. Enumerating *every* memory operand
in `FUN_141b2c7c0` and dropping the `rsp`-based ones leaves `[rax]` x4, `[rax+0x14]` and
`[rax+0x18]` and nothing else, so the `u32` reaches no gate at all.

**And the Change Channel opcode is `0x00D2`**, which this file listed as unknown.
`FUN_142a316e0`, the Change button, re-checks the same predicate and then calls
`FUN_1418287f0` - which `research/msexe-packet-fields.txt:86` already named as the `0x00D2`
builder. What was missing was that this is the Change Channel action. Body: `u8
targetChannel` (**0-based**, matching this repo's numbering), a `u32`, and the shared
14-byte `FUN_140c7b890` preamble. The field **set** is **[L]**; the field **order** is
**[D]** and unsettled - `0x00D1` calls that preamble first while `0x00D2` calls it last, and
there is no capture to decide. **Do not build the answer from that ordering** until a click
produces one.

> **The reply, once a click does send something.** Expect a migrate command: the swap has to
> mint a migration for the **target** channel, which `store::create_migration` already takes
> as a parameter, and hand back that channel's address the way `0x0011` does at login.

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

**What is not built is the swap itself, and this is now a live hazard.** The request is
**`0x00D2`** on the **channel** connection (above), and **nothing answers it**. Until the
enable byte landed, that did not matter, because the click never produced a packet. Now it
may.

> **An unanswered packet freezes the client's whole UI** - every button, including the quit
> prompt's OK. So on the next run, **click Change Channel last**. If the client freezes
> right after, that is the unanswered `0x00D2`, not a crash, and `world.log`'s last inbound
> line names it - which is the measurement this run is for. Do the equipment and dialogue
> checks first, because a freeze ends the session.
>
> The login flow cannot carry the swap: in mode 5 the client never sends a world/channel
> selection at all, going straight from `0x0080` to `0x0078`. Answering it will mean minting
> a migration for the **target** channel, which `create_migration` already takes as a
> parameter, and handing back that channel's address the way `0x0011` does at login.

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
(12 named-but-absent, 6 present-but-unnamed).

**A refusal now says why, in the chat window.** It used to be silent, because there was no
outbound "tell the player something" packet - that is `0x00BB` (`u8 force, str text`) and it
is found and sent. `force` must be **1**: with `0` the client shows only the first line after
each field entry and drops the rest, which reads exactly like the feature being broken.
Confirmed on screen for both a bad id and a good one.

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

### BUILT 2026-08-19 - and the premise this section rested on was wrong

`net::mob::mob_enter_field`, wired into `crates/world`. **137 bytes**: an 11-byte head, a
20-byte mask, and a 106-byte tail. `gm-handbook/mobs.txt` feeds it, so map 30 gets its six
snails and map 40 its forty.

**`FUN_14046fba0` is not a movement-path decoder.** This file described it as one, and that
framing is what made the body look unbuildable. It is `MobStat::DecodeTemporary`: a flat
list of optional fields, each gated by **one bit of the 20-byte raw block** that
`FUN_141c76190` reads immediately before calling it. `MOV R15,R8` at `14046fbbf` is the only
write to `R15` in all 10528 bytes, and the first loop bounds the bit index at
`CMP EDI,0xa0` = 160 bits = 20 bytes. **All 331 packet reads sit behind a mask-bit test, so
an all-zero mask costs zero bytes.** **[L]**

That scan was itself checked: a guard-interval pass covered 328 of the 331. The other three
(`140471db3`/`dc1`/`dd4`) sit behind an **OR of two bits** - `BT/JC take; BT/JNC skip` -
which the pass could not see because it only looked at the first conditional jump after each
test. Read by hand at `140471da4`. 331 of 331.

**Two readers the earlier pass had missed:**

* `FUN_141cc9410` was recorded here as "reads NOTHING". It calls `FUN_14085acd0(..., packet)`,
  which reads **57 bytes** unconditionally - gated by the `u8` at `141d33734`/`141d338fa`,
  so we send 0.
* The real body is behind a **virtual call**, `CALL [RAX+0x38]` at `141d33929` =
  `FUN_141c4ff80`: 52 reads, **106 bytes minimum**, and where x, y, foothold and HP live.
  All eight concrete mob classes share slot 7, and each vtable was reached through the
  constructor that installs it rather than by aligning tables - so this is not the
  `/OPT:ICF` trap.

**That function also found an eighth packet-read primitive.** Two instruments disagreed, 50
reads against 52. Sweeping *all* 116 direct call targets - enumerating rather than filtering
against the known list - turned up **`0x1406e8ef0`, a bare `JMP 0x1406e8b80`**, a second u16
thunk. With it both say 52. `docs/ghidra.md`'s table now says eight, and carries the checked
negative that **it appears in none of** `FUN_140304b20` (the character record),
`FUN_140304100` (the equipped item) or `FUN_141f6f350` (the script message) - so those
layouts stand.

> **RESOLVED, and it was the wrong suspect.** This paragraph ranked the WZ-template-driven
> blocks as the most likely first-attempt failure. They are **`patrol`** (`template[0x104]`)
> and **`targetFromSvr`** (`template[0x1a0]`), parsed by `FUN_14047d990` - and **none of the
> 193 mob images in this client carries either**. `Mob.ini` says `LastWzIndex|0`, so 193 is
> all of them, and the same search finds keys present in 1, 13, 37 and 193 images, so a zero
> is a real zero rather than a broken search. Templates 1 and 2 are both clean.
>
> **The body is not implicated at all.** Its 52 reads are `.pdata`-bounded, a dominator test
> picks out exactly **35 unconditional** ones, and those 35 are identical to what `mob.rs`
> emits. And `mob+0x2b8` is **client-side**: `encodeInit` fills it from a `QueryInterface`
> on a freshly allocated object, in a block that **dominates** body offset 107 - so any body
> that parses that far has filled the field. A 137-byte body cannot make it null.
>
> **Two corrections to what this file said about the run.** The fault landed **~23 ms after
> the send, inside the dispatch** - there is no `elapsed_us` line for any `0x03C6`, and that
> line is written after the trampoline returns - not half a second later; the two logs are
> stamped four hours apart in different zones. And it was the **first** mob, not the
> fortieth: `-MobLimit 1` reproduces it, so **volume is not the variable** and the
> blast-radius reasoning that flag was added for was based on a misread timestamp.
>
> So `+0x2b8` was null because either the virtual ran **before** `encodeInit` reached the
> assignment, or `encodeInit` **threw** first. That could not be settled statically: every
> step of the real path goes through a vtable, which a direct-call graph cannot see. Note
> also that **the absence of a C++ THROW line is not evidence** - the hook's threshold is
> 25 s and the fault landed at 11.7 s.
>
> **Next step is a measurement, not a change.** Changing a value or a width would confound
> the one thing verified three ways.

**The NPC lesson's mob equivalent is HP.** Zero is structurally legal and draws a mob at 0%;
the bar is `hp * 100 / maxHp` through an `IDIV` at `141c50502` with **no zero guard**. The
server sends 100 until `Mob.wz` gives the real value, and the smoke test asserts no mob goes
out with `hp = 0`.

Two other fields are deliberately not zero: `calcDamageIndex = 1` (**[I]**, the reference's
initialised value) and offset 74 = `-1` (**[L]** - `CMP R14D,-0x1 / JLE` means a negative
value skips a call that zero would enter with an out-of-range index).

**Object ids start at 2000**, so mobs and NPCs cannot collide even if the two pools share an
id space, and `FieldMob::new` steps any id that is zero or a multiple of 178 past itself.

**What the reference was worth here, measured:** it matched the head **5 for 5** and the
three blocks after it in order, plus about ten structural details inside `encodeInit` - and
got the three special template ids **wrong** (`8910000/8910100/9990033` against the real
`8909488/8909588/9990545`). Shape yes, numbers no, which is exactly what
`CLAUDE.md` says to expect from it.

Still open: meanings for 17 of the 35 `encodeInit` fields, sent as zero on a "no readable
consumer" argument - which is the argument `research/setfield-zero-audit.md` exists to
distrust.

Drops follow spawns: a mob has to exist before it can drop.

### NEW GOAL, set by the owner 2026-08-19: **quest state that actually advances**

The owner split the quest work into three and asked for the first two now, with this recorded as
its own goal:

1. **Say the right line for the right NPC and quest** - **DONE**, see below.
2. **Paging and yes/no** - blocked on `0x00F3`, which is being decoded.
3. **Quest state that actually advances** - *this goal*.

**Nothing about quests persists or changes.** Accepting a quest does nothing, the same line
comes back every time, and the journal never fills. What that needs, in rough order:

* **Where quest state lives on the wire.** The character record has a **presence-gated quest
  block** - `research/charrecord-presence-map.md` has the 40-row table, and the block needs
  the same treatment `presence[0]` (the stat block) and `presence[2]` (the equipped list)
  each got. Those two are the worked examples; this is the third of the same shape, and both
  earlier ones took a listing walk plus one client run.
* **The quest-result packet.** No packet that *accepts* or *completes* a quest has been
  found in either direction. Until one is, nothing the client does can change state, and
  nothing the server sends can tell it that state changed.
* **Storage.** `crates/store` would need a `quest_state` table keyed by character and quest,
  with the same exhaustive-destructure discipline `character.rs` uses so a new field cannot
  silently fail to persist.
* **`Check` and `Act`.** The generated table already carries both: `Check.<state>` has `npc`,
  `lvmin` and `job`, and `Act.<state>` has the rewards and `nextQuest`. Deciding whether a
  character *may* start a quest is a pure function over data we already have - it is the
  wire format that is missing, not the rules.

**What is already in hand, so nobody re-derives it:** all 322 quests with their full `Say`
trees, `Check` requirements and `Act` rewards, generated from the client's own
`Quest.wz/QuestData` by `tools/dump_quests.py`; and the confirmation that the WZ's quest ids
are the protocol's, because NPC template 1 starts exactly quest 1000 and a real client's
`0x0151` carried quest 1000 with template 1.

**Depends on goal 2's `0x00F3` work** for anything interactive: a quest cannot be *accepted*
until the client can answer a yes/no box.

### 4a. Quest dialogue - **the right line, DONE 2026-08-19**

`0x0151` is answered with that quest's own opening line out of `Quest.wz`, chosen by the
request's action byte: the `Say."0"` conversation for a start or opening-script action, and
`Say."1"` for a completion one. It falls back to the NPC's `d0` line and then to a notice, so
a quest the table does not have still produces something rather than silence.

**Only the first line is sent, and that is a decision rather than an omission.** Paging means
setting the `next` flag, which asks the client to send a `0x00F3` when the user presses it -
and `0x00F3`'s body is not decoded, so the server could not answer. An unanswered request
here does not merely do nothing: `research/npc-click.md` found `player->[0x2330]`, a
**one-request-outstanding latch** set by 37 functions and cleared only by inbound handlers,
so leaving one outstanding silently kills every later request in its class with no dialog and
nothing in any log. One line that ends cleanly beats four that wedge the client.

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

### 6. Equipment and consumables - **the equipment half is DONE, confirmed on screen**

The character is dressed and **every item carries its `Character.wz` stats**, on every
`SetField` rather than only the first - see §2e for the bug that made it look otherwise for
most of a day.

**Consumables are not started**, and the nearest concrete step is the bag: the unequip
request never reaches the wire, and the leading explanation is that the bag has **no slots**
(§2e, START HERE item 2).

*Original notes below.*

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
  **Amended 2026-08-20: the entry set was 23 and is 28.** `tools/callers.py` saw `call` only,
  so five tail-jump entries were invisible - and with them a chain nobody walked,
  `FUN_142d012e0` -> `FUN_142797be0` @ `0x142797df5` -> shim `0x1420dd920` -> `FUN_140f80140`.
  Neither new function is in the case table, so the **conclusion survives**; the *evidence as
  stated* no longer covers the entry set, which is a different thing and worth saying.

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
  `powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\firewall.ps1" -Remove`
  (needs elevation, which is also why the path is absolute - an elevated window opens in
  `C:\Windows\System32`).
- `client-patched/` has the GameGuard stub installed;
  `powershell -ExecutionPolicy Bypass -File "C:\MapleCW\tools\setup-client.ps1" -Restore`
  puts the real DLL back.
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
