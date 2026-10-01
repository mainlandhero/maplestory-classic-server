# `0x0203` and `0x0205` - the two pet opcodes the deployed logs call UNKNOWN (2026-09-25)

The owner: *"Please take a look at those and understand what's going on in those packets."* The
deployed server's logs (`Server Investigation/`) hold 59 `0x0203 UNKNOWN` and 33 `0x0205 UNKNOWN`.

## `0x0205` - already fixed; history, not a bug

Every one of the 33 is in a build that predates the pet pick-up handler:

```text
world-ch0.log.5                          (09-14)   0205 UNKNOWN 21   named 0
previous-runs/world-ch0-20260916-005451  (09-16)   0205 UNKNOWN 12   named 0
previous-runs/world-ch0-20260916-171249  (09-16)   0205 UNKNOWN  0   named 94   <- handler shipped
every later file                                   0205 UNKNOWN  0   named ~21 000
```

The old bodies are the pet pick-up the current server parses. Split by the reference's shape
(`net::drops::PET_PICK_UP_OBJECT_ID_AT`):

```text
00000000 | 00 | fa496d00 | 01000000 | 6001 d700 | 27303101 | 01ac441e | 444b4c00
petIdx     u8   tick       u32 1      x    y      dropId@17  crc?       u32 0x004C4B44 = 5000004
```

The drop id sits at byte 17 in all 33, where the current parser reads it. **[I]** the last `u32`
is the pet's item id (`5000004` for purr, `5000001` for Cobalt - one per owner, constant). Two of
the 33 are 41 bytes: they append 12 more (`de03 d700 | ab16e056 | 0702328e` - an `x, y` equal to
the first pair, then two `u32`s, meaning not read); the offset of the drop id does not move.

## `0x0203` - the owner's client reporting the line its pet just said

**The builder is the pet performer, `FUN_141ec6680(pet, type, entry, line, flag)`** - the function
that makes a pet act and put up a speech balloon. It sends `0x0203` only when `flag != 0`:
`cmp [rbp+0x4f0], 0 / je` at `141ec76a1`, and `rbp+0x4f0` is the fifth argument. **[L]**
The fields (`tools/encodes.py 0x141ec6680`, control `0x0114`'s builder read back its known shape):

```text
141ec76cf  u32  FUN_14019a5d0(pet+0x138)        0 in all 59
141ec76db  u32  [rbp-0x78]                      changes every call
141ec76e8  u8   type                            2 (food) in all 59
141ec76fd  u8   entry, written as 0 if < 9      Moth 0x0a; everyone else 0
141ec77dc  str  the line                        "Yum, yum! Pet Food x195 left!", "Bark! Bark bark
                                                 bark!!!", "This is delicious...!"
```

No exclusive-request latch after the send (unlike `0x0206`), and the logs agree: every sender
went on to feed again and was answered.

**Who calls it with flag 1:** only `FUN_141ec4780`, the `0x027E` PetActionCommand arm
(`research/pets-loot-skills-name-relogin-2026-09-15.md` §8). Measured: all 59 `0x0203`s arrive
~80 ms after the server sent that same character a `0x027E` for their own hand feed.

**The other half exists: `0x0279`** (`FUN_141ec3fa0`) reads `u8 type -> edi, u8 entry -> ebx,
str line` and calls the performer `(pet, type, entry, line, 0)`. **[L]** That is `0x0203`'s own
fields, performed with flag 0 - so a relayed line is never reported back and cannot loop. The pair
is a pet-speech relay: the owner's client picks the line, reports it, and the server shows the
same line to everyone else.

## What this server does today, and the one thing not measured

The server answers nothing (fine - no latch) and relays nothing. Other players still see the
pet eat, because `on_use_pet_food` publishes the `0x027E` itself to the map; their clients then
pick **their own** random line from the pet's food lines, which may differ from the owner's.

**Not measured:** what a bystander's client does with that map `0x027E`. The arm passes flag 1,
so it may send its own `0x0203` about someone else's pet. None of the 59 feeds had a bystander
on the field (bus deliveries are logged, and no `0x027E` went to anyone but the feeder), so the
logs cannot say. That matters before relaying: if bystanders report too, relaying every
`0x0203` as `0x0279` would put a second, third ... balloon on every screen.

Evidence: `research/fixtures/pet-line-report-0x0203-follows-own-feed-59-deployed-world.log`.

## Built the same day: the relay

The owner: *"Please relay these pet packets ... sync the chat bubbles so that the dialogues are the
same."* The bystander question above is side-stepped rather than answered: bystanders are no
longer sent the `0x027E` at all. The owner gets it; the map gets the owner's reported line as
`0x0279`; a `0x0203` with no feed waiting (which is what a bystander's would be) is dropped. If
the owner never reports within 2 s the map gets the held `0x027E` - the old behaviour.
`Session::on_pet_line_report`, `pet_line_fallback_tick`.
