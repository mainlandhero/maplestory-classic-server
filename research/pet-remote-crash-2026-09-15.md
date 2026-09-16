# Summoning a pet crashes another client in the same map - it is the pet MOVE, not the summon

The owner, 2026-09-15: *"Summoning the pet on character 'the owner' crashed another client 'Tester2'
present in the same map."*

## The measurement

The crash run left a hook log (`previous-runs/maplecw-hook-20260915-220508.log`, Tester2's
client) and a channel log (`world-ch0.log`, this repo's copy). The hook clock runs ~20 h ahead
of the server clock, so hook `22:05:28` is server `02:05:28`.

```
hook  22:05:28.009  331 opcode=0x0277 elapsed_us=808.8 ret=1        <- summon processed, returned
hook  22:05:28.030  795 opcode=0x0277 elapsed_us=912.1 ret=1        <- (a second 0x0277)
hook  22:05:28.449  334 opcode=0x03D9 ...ret=1                      <- a mob move, fine
hook  22:05:28.599  ***** CLIENT FAULT #1: code=0xc0000005 at 0x141d59bf3 *****
      stack: 0x143ad56f5(?) 0x141ec3f85<-TEXT 0x1429bb557<-TEXT 0x143273970(?) 0x141821e41<-TEXT
```

The dispatch line is written on handler **return**, so the two `0x0277` summon packets
**completed** (`ret=1`) and the pet idled for half a second. The fault has **no** matching
dispatch line, because the handler it was in never returned. The stack names it:

* `0x141ec3f85` is the return address inside **`FUN_141ec3f20`**, the `0x0278` **pet-move**
  handler (`net::pet::PET_MOVE`).
* `0x141821e41` is **`CField::OnPacket`**, the inbound dispatcher.

And `world-ch0.log` has the packet that drove it, server `02:05:28.593` ≈ hook `28.599`:

```
02:05:28.022 -> [Tester2#214] 0x0277 PET_ACTIVATED ... Husky (5000006) summoned   <- processed, ret=1
02:05:28.489 <- [Wisp#215]    0x0202 CLIENT_PET_MOVE ...                            <- the owner's pet walked
02:05:28.593 -> [Tester2#214] 0x0278 PET_MOVE ... The owner's pet walked                <- FIRST move -> fault
```

**The first pet move broadcast to the observer is fatal.** The summon is not.

## Why the move faults and the summon does not

The fault instruction, `tools/dis_at.py 0x141d598b0`:

```text
141d59bef  mov    rax, qword ptr [rbp + 0x18]
141d59bf3  movups xmm0, xmmword ptr [rax]        ; rax is null -> 0xC0000005
```

`FUN_141d598b0` copies five 16-byte rows (a transform/rect) out of the object at `[rbp+0x18]`.
It is reached only from the pet-move handler `FUN_141ec3f20`, which derives the object from the
pet at `[pet+0x118]`. For the observer's copy of the owner's pet that pointer chain ends in null.

The reason is the **remote** pet-init path. `0x0277` routes through the user pool by its
`u32 charId` to the user's vtable slot `+0x98`; the **local** user's implementation is
`FUN_1428a01a0` (2420 bytes) and the **remote** user's is `FUN_1429d6150` (144 bytes). The
remote one is only `CPet::Init` + `CUser::SetPet`; the ~2300 bytes the local path runs to build
the pet's **visual/layer** are simply not there. So on the observer the pet exists as data but
has no drawable, and the first move - which updates the drawable's transform - dereferences the
missing layer.

The summon body was correct - decoded from `world-ch0.log` `02:05:28.022`:

```
charId 215, petIdx 0, activated 1, init 1, item 5000006, name "Husky",
x -97, y 149, moveAction 0, foothold 166, hue -1, giantRate 100, nameTag 0, chatBalloon 0
```

Valid foothold (166, not 0), life-size (100). **Nothing the server can put in `0x0277` or
`0x0278` fixes it** - the client cannot render a remote pet on this build, so it cannot move
one either. This is the same class as every other pet finding: the packet is right and the
client's own code decides the outcome. Here the outcome is a null layer.

## The fix: pets are owner-local by default

`Config::broadcast_pets`, default `false`. The four places a pet reached other clients are all
gated on it:

| site | packet | on `false` |
|---|---|---|
| `on_pet_activate` summon | `0x0277` to the map | not sent |
| `on_pet_activate` put-away | `0x0277` to the map | not sent |
| `on_pet_move` | `0x0278` to the map | not sent - **this is the crash** |
| `pet_command_replies` | `0x0279` to the map | not sent (owner still gets its own) |
| `pet_companions` (arrivals) | `0x0277` in `Presence` | empty |

The owner's own pet is untouched: it summons, walks, picks up loot and answers commands exactly
as before - all of that rides the owner's own returned replies, none of it the map broadcast.

This turns off the owner's 2026-09-13 request (*"so other people can see pets moving"*) because that
feature crashes every observer on this client. Re-enabling it means understanding
`FUN_1429d6150` well enough to make the remote pet drawable - or accepting a static remote pet
by broadcasting the summon but never the move, which this flag does not currently split. The
flag exists so that investigation can be run again deliberately, with a second client and a dump
armed, rather than by accident.

Fixture: `research/fixtures/pet-remote-move-crashes-observer-2026-09-15.log`.
Full dump: `dumps/maplecw-crash-1057776-c0000005-1.dmp` (1.6 GB, from the crash run).
