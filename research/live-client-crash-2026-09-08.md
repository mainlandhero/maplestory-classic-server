# The live server's client crashes are the same writer, on the surface the guard page was built for

The owner, 2026-09-08: *"the actual live server have had a few client crashes. These clients do not
have dumps, but they were able to submit their hook logs."* Four hook logs and the server logs,
in `C:\Users\user\Desktop\Crash Investigation`.

Tags: **[L]** read off this client's listing or a log, **[D]** derived, **[I]** inferred.

**Nothing authenticates.** The login now carries a one-time identity token whose SHA-256 is all
the server keeps, but **the game socket still carries no credentials**, and one of the peers in
these logs is a public address (`198.51.100.92`). That is a different exposure from the local
testing this project was scoped for.

---

## 1. What was submitted, and which of it is evidence

| hook log | pid | span | probe VEH armed | ended |
|---|---|---|---|---|
| `…-20260906-235931` | 976 | 23:56:02 → 23:59:31 | yes (9 WATCH) | orderly socket teardown |
| `…-20260906-235949` | 11760 | 23:59:35 → 23:59:49 | yes | orderly socket teardown |
| **`…-20260907-000503`** | **2028** | **23:59:59 → 00:05:03** | **yes (22 WATCH)** | **`CLIENT FAULT` 0xC0000005** |
| `…-20260907-000547` | 3760 | 00:05:14 → 00:05:47 | yes | orderly socket teardown |

**Only one is a crash.** The other three end with `SOCKET … closed and cleared by the client`,
then `session: object at 0x0 is now UNREADABLE`, then a passed-through `FindWindowA` - the
client's ordinary shutdown. This is worth stating because the discriminator is not free: the
`CLIENT FAULT` line is written by `probe.rs`'s vectored handler, which only exists once a watch
arms, so a session with no `probe: watching` line could crash **silently**. All four have that
line [L], so the three quiet ones did not raise an unhandled access violation. They were exits.

The three short sessions bracket the crash in time and are almost certainly the same person
relaunching. That is a reading of the timestamps, not a measurement.

## 2. The fault

```
00:04:56.374 CLIENT FAULT #1: code=0xc0000005 at 0x14094e1d0 in MapleStory.exe
             stack: 0x142b52554 0x142bceb1c 0x141c4fe5d 0x141d1a274 0x140f08fd3
                    0x142f31839 0x140ee4078 0x141d50ed2 0x140205833 …
```

`0x14094e1d0` is inside `FUN_14094e150` (`.pdata` `0x14094e150 .. 0x14094e407`) [L]. It is a
**destructor**: it writes two vtable pointers at entry, then tears down member after member.

**The two fault sites this project has recorded are the same instruction in the same function.**
`0x14094e190` - filed locally as a "close-time crash, recorded, not chased" - and the live
`0x14094e1d0` are the two consecutive list-teardown loops:

```
0014094e18c  nop                              0014094e1ce  je    0x14094e1e5
0014094e190  mov  rbx, qword ptr [rcx]   <--  0014094e1d0  mov  rbx, qword ptr [rcx]   <--
0014094e193  mov  edx, 0x38                   0014094e1d3  mov  edx, 0x38
0014094e198  call 0x142ef3bb8                 0014094e1d8  call 0x142ef3bb8
0014094e19d  mov  rcx, rbx                    0014094e1dd  mov  rcx, rbx
0014094e1a3  jne  0x14094e190                 0014094e1e3  jne  0x14094e1d0
```

Walk a singly-linked list; **read the node's `next` pointer**, free the node, move on. The
faulting instruction is the *read of `[rcx]`*, so **`rcx` - a list node pointer - is bad.** The
first loop drains the list at `[rdi+0x810]`, the second the list at `[rdi+0x800]` [L].

**Correction to an earlier label.** These were written up as *close-time* crashes. The live one
is not: it fired 4 m 53 s into an idle session, and the process then ran long enough to write a
1.2 GB dump and log a sentry heartbeat seven seconds later. This destructor runs during play.

## 3. The nodes are pool bucket-2 objects - the `0x40` class

`0x142ef3bb8` is a thunk: `jmp 0x140205820` [L]. And `0x140205820` is `operator delete`:

```
000140205820  sub  rsp, 0x28
000140205824  mov  rdx, rcx
000140205827  lea  rcx, [rip + 0x38d1072]     ; -> 0x143AD68A0, THE POOL CONTEXT
00014020582e  call 0x14019bb50                ; THE POOL FREE
```

`0x140205833`, the return address inside that function, is **on the live fault's stack** [L].

The size passed is `0x38` (56 bytes). The client's own allocator ladder puts that in bucket 2:
`cmp rdx,0x20; ja` taken, then `cmp rax,0x40; ja` **not** taken, then `mov r8d,2` [L] - slot
`0x40`.

So the objects this destructor frees are **pool `0x40`-class objects**: the exact class
`-GuardPage` quarantines by default, and the exact class runs 2 and 5 died on locally with a map
node's pointer incremented by `+2`.

## 4. The writer is on other people's machines

This is the finding that changes the most.

`research/the-180-second-clock-2026-09-07.md` §229 records the open worry plainly: the writer
*"has only ever been observed in our environment; there is no unhandled or non-MapleCW run
anywhere"*. The crashing session settles it:

```
00:03:35.825 POOL SENTRY REPAIR: 0x3f51fdc8 was 0x0000000100000020, now 0x0000000000000020
00:03:35.827 POOL SENTRY FINDING #1: bucket 1 (slot 0x20) header 0x3f51fdc8 reads 0x0000000100000020
             DAMAGED 0x3f51fdc8  free-list=no  looks like a BSTR of 6 chars: "thumb1"
```

**The identical value, in the identical size class, on a different person's computer.**
`0x0000000100000020` is the same signature as every damaged header in both of the owner's crash
dumps. ~~The writer is a property of this client, not of this machine, this network or this
server.~~

> **THAT LAST SENTENCE WAS TOO STRONG, and it is struck. Narrowed 2026-09-08 evening.**
> That player runs our launcher, our patched client and our injected hook, so this was **never
> an independent control** - `is-the-corruption-ours-2026-09-06.md` §1's missing control is
> still missing. What the observation *does* establish is that the writer is not specific to
> **The owner's machine, network or server**, which is worth having. What it does **not** do is
> separate *"the client does this unprompted"* from *"our tampering trips the client's own
> anti-cheat"*.
>
> That second reading was not available when this was written and is now:
> `research/the-180-second-family-is-anti-cheat-2026-09-08.md` shows the module doing the
> writing is **anti-cheat** - `Crc Fail Alert!!`, `CheatEngine`, and three cheat-tool names
> stored with CR/TAB bytes spliced through them so a plain search misses them [L] - and that it
> writes **out of bounds by construction** after a detect-report-then-wait-180 s chain. Our hook
> patches `.text`. Against the reading: the two gate values are byte-identical across separate
> sessions, which looks more like configuration than a detection count [I].

The repair worked: no `0xC0000374` heap-corruption death followed. The client died **80.5 s
later** of `0xC0000005` instead - which is the same pattern as local runs 2 and 5, where the
`0x20` half was repaired and the `0x40` half killed it anyway.

## 5. Session timeline

| time | event |
|---|---|
| 00:00:02.964 | pool sentry armed, repair on, adaptive 2000 ms, dumps capped at 0 |
| 00:03:35.825 | **damage found and repaired**, bucket 1, `0x0000000100000020` |
| 00:04:56.374 | **`CLIENT FAULT` 0xC0000005 at `0x14094e1d0`** |
| 00:05:03.103 | 1.2 GB dump written to `C:\Users\wes10\Downloads\MapleCW-setup\MapleCW\dumps\` |
| 00:05:03.206 | last line: heartbeat, `1 confirmed finding(s)`, `1 header(s) repaired` |

Traffic was idle: of 747 lines the opcodes are `0x03E4` control ack (228) and `0x0453` NPC idle
chatter (69) [L]. `research/heap-corruption-2026-08-27.md` already records an idle session dying
the same way. **Nothing the player did caused this, and nothing the server sent looks
implicated.**

## 6. What this does NOT establish

The chain is strong and it is still **circumstantial**. What is proven: the same client, in the
same session, had a confirmed writer hit 80 s before it died reading a bad pointer out of an
object in the size class the writer is known to corrupt. What is **not** proven is that *this*
pointer was corrupted by *that* writer. A stale pointer into a recycled slot produces the same
fault, and so does an ordinary use-after-free with no writer involved.

**One artifact settles it, and it exists**: the 1.2 GB dump on wes10's machine. The faulting
`rcx` is the whole question - a value that differs from a live node address by `1` or `2` in the
low dword is the writer's signature and nothing else's. `tools/dumpwalk.py` and
`tools/poolchain.py` read these dumps already.

## 7. The server side is missing, and says so

`world.log`, `login.log` and everything in `previous-runs/` spans **14:48–22:09 UTC on Sep 6**
and **05:58–09:12 UTC on Sep 7** [L]. The crash is at client-local 00:04:56, i.e. ~04:04 UTC -
**inside the gap.** No file provided covers the crashes. `CLAUDE.md`'s "count the same event in
two logs" cannot be run here, and the archived logs that would close it are not in the folder.

The one substantive server window is still useful for a different reason. Both channel-0
sessions in it ended the same way:

```
07:02:58.551 ch0 #1 192.168.0.123:63308 ended: forcibly closed by the remote host (os error 10054)
07:05:05.481 ch0 #2 198.51.100.92:50454  ended: forcibly closed by the remote host (os error 10054)
```

**Two of two, a hard reset rather than a clean close.** A client that logs out closes its socket
gracefully - the three quiet hook logs show exactly that. `WSAECONNRESET` is the server-side
fingerprint of the process dying underneath the socket, and it costs nothing to count. It is the
cheapest crash rate this project has: it needs no hook log and no dump from the player.

## 8. What follows

* The `0x40` quarantine (`-GuardPage`, default bucket `0x40`) is aimed at exactly this class,
  and **has never been shipped to these users** - it is not in what they are running.
* The `0x20` repair they *are* running demonstrably worked and demonstrably was not enough.
* Ask wes10 for the dump, or for the output of the two repo tools run against it. It is the only
  thing that turns §6 from an argument into a measurement.
* Count `os error 10054` per session in the server log as a standing crash metric.
