# Packet transport

What sits on the socket *after* the greeting in `docs/handshake.md` is accepted. Captured
from a real client run: `research/fixtures/capture-handshake-ok.log`, 294 bytes, 16
packets.

Status: **SOLVED IN BOTH DIRECTIONS.** The captured client stream decrypts to clean
packets, and the client accepts frames we build.

The last blocker was the AES key. The key table at `0x143A86810` holds the stock
MapleStory key **on disk and is a decoy** — the client overwrites the low byte of all 32
dwords at startup. Only that table is patched; the IV shuffle table beside it is
untouched, which is exactly why framing, the header constant and the IV chain were all
provably right while everything AES-shaped failed in *both* directions.

The real key, read out of the live process with `tools/dump_runtime.py`:

```
0f 00 00 00  1b 00 00 00  c5 00 00 00  46 00 00 00
f3 00 00 00  be 00 00 00  ff 00 00 00  75 00 00 00
```

It is stable across sessions, so it is a build constant rather than a session secret.
Packet 1 of the capture decrypts to `70 00 02 64 00 00 00` — opcode `0x70`, `u8 2`,
`u32 100` — matching `FUN_1415d5b40` byte for byte.

> **Confirmed 2026-08-15.** A `--reply header` run (valid header, body withheld) left the
> client connected instead of dropping it. That is the first frame the client has ever
> accepted from us, and it validates the header rule, the `0xFFFE` constant, the `K` IV
> seed and the shuffle table together.
>
> Beware the failure mode that preceded it: an exception in the probe closed the socket,
> and the client dropped because the *server* vanished — indistinguishable from a
> rejection. Always check `probe.err` is empty before believing a negative result.

## The inbound side is solved too (2026-08-16)

Two inbound opcodes are established, and neither was found by sweeping:

* **`0x0032`** — the `Data.wz` patch reply. Body is a **zigzag varint** length; `0` means
  "nothing to patch". Seven bytes on the wire take the client from a blank, non-responding
  window to its login screen. It blocks in `recv` **on its UI thread** in `FUN_1415e7090`
  until a handler sets `conn+0x150`, and `FUN_1415e5c20` is the only thing that does.
* **`0x0010`** — the login result, answering the client's body-less `0x0080`. Body is
  `u8 result` then a `u16`-length string; **result `0` is success**.

The structural point matters more than either number: **a stage's `OnPacket` is ordinary
readable code.** The virtualised dispatcher only routes — it hands a stage its opcode, and
`FUN_141b25f30` (login) dispatches with a plain `switch` that names every login-stage
opcode at once. Read the switch before reaching for any runtime technique.
`docs/opcodes.md` has the map; `STATUS.md` has the current state and the traps.

**Everything below this line is kept as history.** Sections marked *superseded* describe
what was believed at the time, including several conclusions that turned out to be wrong;
they are retained because the reasoning is instructive, not because it is current.

## Framing — read from `FUN_1406e9530`

```
u16 a          # encodes the IV and a per-direction constant
u16 b          # a ^ payload_length
u8[a ^ b]      # payload
```

so `payload_length = a ^ b`, with both `a` and `b` little-endian u16 on the wire.
Note there is **no byte-swap of the length**, unlike classic MapleStory — the client does
a plain `len = a ^ b`. Verified against the capture: 16/16 packets, 294 bytes consumed
exactly, including three reads that TCP had coalesced.

If that 16-bit length comes out `>= 0xFF00`, the header is **8 bytes** instead of 4 and a
32-bit length follows, itself XORed with `a`. This also explains the `uVar1 = uVar2 - 4`
adjustment in `FUN_1406e99e0`.

The buffer the cipher sees holds header **and** payload: `FUN_1406e88d0` copies both, sets
`+0x20` to the payload length, and `FUN_1406e99e0` crypts `+0x20` bytes starting at
`ptr + 4`. So the payload begins after the 4-byte header, and only the payload is crypted.

## The header rule

`FUN_1406e97e0` is the whole check, and it is 11 bytes long:

```c
return (ushort)((uint)param_2 >> 0x10) ^ *(ushort *)(param_1 + 0x1c);
```

Both receive loops drop the connection unless the result is `-2`. So for anything **we**
send the client:

```
a = ((iv >> 16) & 0xFFFF) ^ 0xFFFE
b = a ^ length
```

`0xFFFE` is a hardcoded comparison in the callers, not something derived from our
greeting, so it is stable.

The constant is **direction-dependent**. The client's own outbound packets use `0x00DF`,
measured across all 16 captured headers. `tools/transport.py` carries both.

### The two IVs

Fields `J` and `K` of the greeting land at `conn+0xe8` and `conn+0xec`:

| Slot | Field | Used for |
|---|---|---|
| `conn+0xe8` | `J` (first u32) | client **send** — the stream we captured |
| `conn+0xec` | `K` (second u32) | client **recv** — the stream we must produce |

The IV is a 4-byte value that evolves once per packet using the **stock MapleStory shuffle
table**, present verbatim at VA `0x143A86890`. Seeding the chain with the exact `J` we sent
(`0x52307801`) predicts **all 16 observed headers**; seeding with `K` predicts only the
first. That pins the direction of each field.

`tools/transport.py` reproduces all 16 captured headers byte-exactly, which is the
strongest confirmation available that framing, constant and IV evolution are all correct.

## Cipher mode is chosen by connection type

> **CORRECTED 2026-08-19, off the wire.** The comment below reads `mode = 2` as "a game
> channel uses the byte subtract". **It does not.** The first two packets a real client sent
> on a real channel connection decode cleanly under **AES-256-OFB** and under nothing else -
> see "The channel cipher is AES" at the end of this file. What `conn+0x48` actually selects
> here, and why the channel did not take the branch this code appears to give it, is
> **unresolved**. The rest of this section is still a correct reading of the *code*; it is
> the inference from code to channel behaviour that was wrong.

```c
if (*(int *)(param_1 + 0x48) == 0) mode = 2;   // NOT what a channel connection does
else                               mode = 1;   // login connection
```

| Mode | Function | Algorithm |
|---|---|---|
| 1 | `FUN_140c75880` | AES-256-OFB |
| 2 | `FUN_1406ef9f0` | `out[i] = in[i] - (iv & 0xFF)` — a plain byte subtract |

So this code path says the **login** connection is AES and something else uses the trivial
subtract. **Whatever that something else is, it is not the channel a migrated client opens** -
that one is AES, measured. Do not act on this table for channel work.

`FUN_1406e9910` would crypt only the 2-byte opcode, but it has **no callers**.

## The AES — stock in every part

- **AES-256.** `Nk = 8`; `FUN_140c761f0` is a standard 14-round T-table implementation with
  little-endian word loads (`Te0` at `0x143ac3d00` indexed by the LSB).
- **Stock key.** `FUN_140c76070` reads 8 key *words* from `0x143A86810` at a stride of 4
  dwords. Dumping the table confirms each of those dwords holds only a low byte:

  ```
  13 00 00 00  08 00 00 00  06 00 00 00  b4 00 00 00
  1b 00 00 00  0f 00 00 00  33 00 00 00  52 00 00 00
  ```

  That is the classic MapleStory key. The 24 dwords interleaved between them
  (`52 2a 5b 02 10 60 ...`) are **decoys** that are never read.
- **Stock key schedule and tables.** `FUN_140c759a0` builds them at runtime with the `0x1b`
  polynomial, affine constant `0x63` and standard Rcon.
- **OFB.** `FUN_140c78270` re-encrypts the keystream block every 16 bytes and XORs it in.
  Both cores (`FUN_140c78270`, `FUN_140c78030`) are OFB, so the scheme is symmetric.
- **IV block** is the 4-byte IV repeated 4x. When the IV argument is NULL a fixed 16-byte
  default at `0x143307778` is used instead.
- **Chunking** is `0x5B0` for the first chunk then `0x5B4`, restarting the IV each chunk.

> **Correction.** `tools/decrypt_capture.py` originally built the key from the low byte of
> *all 32* dwords (`13 52 2a 5b 08 02 ...`), on a "customised key" theory. That was wrong,
> and every earlier negative result — the key sweeps, the IV-position sweeps, the Shanda
> tests — was produced with that broken key and proves nothing.

## Superseded: "the client -> server payload does not decrypt"

> **Resolved.** The payloads are plain AES-256-OFB. They failed only because the on-disk
> key is a decoy, which the section above explains. Nothing in the list below was the
> cause; keep it as a record of what was eliminated, not as an open problem.

Re-run with the correct key, the captured payloads still do not decrypt. Ruled out
properly this time:

- key: stock LE, stock BE, 8-bytes-padded, the old all-32 variant, raw 32 bytes
- IV: both chains, 24 positions each, every packet tested against every position
- keystream alignment: offsets 0-8, to cover the payload/header boundary being off
- with and without the classic Shanda transform
- mode 2 (byte subtract), excluded on packet 1 against all four IV bytes and both signs

Scored without assuming an opcode — real MapleStory plaintext is zero-rich, so a 47-byte
packet should show several `00` bytes. Across 1296 trials the mean was 0.22 zeros and the
best 2, i.e. indistinguishable from chance.

**Why: the code is not in the file.** `FUN_1415d60e0` — the step that runs immediately
after decryption in both receive loops — is a 22-byte stub that tail-jumps out:

```
48 89 54 24 10       mov  [rsp+0x10], rdx
48 89 4c 24 08       mov  [rsp+0x08], rcx
48 81 ec 88 00 00 00 sub  rsp, 0x88
e9 73 74 50 03       jmp  +0x03507473        -> 0x144ADD569
```

and `0x144ADD569` lands in the **`.themida` section, which has no file bytes at all** — it
is materialised only at runtime. `FUN_1415d3990`, the send entry point (called by both
`FUN_1415d5b40` for `0x70` and `FUN_1415d5c20` for `0x71`), likewise hits `halt_baddata()`
partway through.

So the paradox is resolved, and definitively: every primitive we can read is stock, and
the packet dispatcher plus the outbound transform simply do not exist on disk. No amount
of further decompiling will recover them.

### Next

1. **Do not block on this.** The direction we need to *produce* is fully specified and
   testable against the client, which is the real oracle. `tools/transport.py` implements
   it; `handshake_probe.py --reply` exercises it.
2. If the client -> server direction is needed later, it will take dynamic analysis
   (breakpoint after `FUN_1406e99e0` and read the buffer) rather than more decompiling.
3. Remember the **game/channel connection uses mode 2**, so none of this AES applies there.

## The client's outbound opcode map

`tools/ghidra_scripts/DumpOpcodes.java` enumerates every call to the packet writer
`FUN_1406ed520(buf, opcode)`, which names one client -> server opcode per call site.
**1881 of 1894 call sites resolved, giving 657 distinct opcodes** from `0x0000` to `0x0FA0`
(`research/msexe-send-opcodes.txt`). This is the outbound half of the protocol, recovered
without decrypting anything.

The connection module (`FUN_1415d*`) sends `0x70`, `0x71`, `0x72`, `0x7D`, `0x8F`-`0x91`,
`0x95`, `0x9B`-`0x9D`, `0xA1`, `0xA7`, `0xB5`, `0xB7`, `0xBE`, `0xBF`, `0xC3`, `0x427` and
`0x42B`. Two things worth noting:

- the handshake handler `FUN_1415d10e0` itself sends `0x7D`, `0xA1` and `0xB5` in its tail,
  so some of the captured 16 packets come from the handshake, not from the `0x70`/`0x71`
  senders;
- `FUN_1415d9cd0` builds **no** packet — it raises event `0x3f0` via
  `FUN_140319730(conn+0xf8, ..., 0x3f0)`. That closes the old question of whether it sends
  something before `0x70`.

The **inbound** opcode space is still unknown, because the dispatcher is the `.themida`
function above. Hence the sweep below.

## Testing against the client

The client is the only oracle available, so `handshake_probe.py` can answer it:

| `--reply` | What it sends | What it tells us |
|---|---|---|
| `header` | a valid header declaring 100 bytes, body withheld | framing and IV only — nothing is dispatched, so an unknown opcode cannot muddy the result |
| `ping` | one complete encrypted packet | whether a full packet is accepted |
| `sweep` | every opcode in a range, one per `--sweep-delay` | which inbound opcode the client reacts to |

The sweep logs one timestamped line per opcode, so if the client disconnects or the UI
changes, the last line printed says exactly where. Cipher state is carried across the
whole sweep, so the IV stays in step with the client's.

## Confirmed inbound behaviour (2026-08-15)

A `--reply sweep` run sent opcodes `0x0000`-`0x0023` as complete encrypted packets, each
with `--pad 32`, one every ~150 ms. `probe.err` was empty, so the run is trustworthy.

**The payload cipher is confirmed.** The client consumed **36 consecutive encrypted
packets** without complaint, its receive IV staying in lockstep with ours the whole way.
A wrong key, IV or mode would have failed on the first packet, not the thirty-seventh.
AES-256-OFB with the stock key, IV repeated 4x, rolled once per packet, is correct for
the direction we send.

**Opcode `0x0023` makes the client exit.** After it was sent the client reset the
connection and shut down *gracefully* — no crash, no dialog. That is the first inbound
opcode identified, and it is clearly handled. Known reactive opcodes:

| Opcode | Effect |
|---|---|
| `0x0000`-`0x0022` | consumed with no visible effect |
| `0x0023` | client closes the connection and exits cleanly |

Capture kept as `research/fixtures/sweep-0000-0023-exit.log`. `--skip` exists to carry a
growing blacklist of connection-enders across later sweeps.

## The outbound payload is a position-wise keystream — shanda is ruled out

Two runs sent the same `J`, so the client's send keystream was identical in both. Packet 1
came out as:

```
run A: 87 BB 8B 51 ... 00 58 0E  F8 02 B2 A5 06 1B 37 4F 99 4E 2F 8F D9 75 FB 8B 7B E3 54 74  D9 1C DD 76 BD E5 67 19
run B: 87 BB 8B 51 ... 00 58 0E  F2 97 2C E3 0C 8E A9 09 93 DB B1 C9 D3 E2 6C C5 4D 56 FF 32  D9 1C DD 76 BD E5 67 19
```

Bytes 0-18 identical, **19-38 differ**, 39-46 identical again. Under a stream cipher
identical ciphertext means identical plaintext, so the plaintext differs *only* in those
20 bytes — a per-run session value of some kind.

The important part is what that rules out. Shanda, and any byte-mixing transform, chains
state across the packet: a change at byte 19 would propagate to every byte after it. It
does not. So the client -> server payload is a **pure position-wise keystream**
(`ct[i] = pt[i] (op) ks[i]`), and the only thing wrong is the keystream we compute. That
is now established from data rather than from failed sweeps, and it means the extra
transform theory was the wrong shape entirely: there is no extra transform, just a
keystream we cannot yet reproduce — most likely because the send side seeds its AES from
something other than the header IV.

## Superseded: no confirmed reply *at the time*

> **Resolved.** `0x0032` and `0x0010` are both confirmed inbound opcodes now. The warning
> about the client's own chatter still stands, and `-QuietBefore` / `-ReplyTo` exist
> because of it.

**Correction.** Two earlier runs looked like the client answering us. Neither did.

| Run | What we sent | Client output | Reading |
|---|---|---|---|
| 1 | sweep `0x0024`-`0x01C3` | 294 B, a 10 B packet at +64 s | looked like a reply to `0x0171` |
| 2 | `0x0171` alone 15 s, then `0x01C4`-`0x01DC` | 348 B, a 54 B packet at +19.7 s | looked like a reply to `0x01C4`-`0x01CA` |
| 3 | `0x01C4`-`0x01CA` at **3 s** spacing | **3393 B, all before our first packet** | nothing replied |

`0x0171` was ruled out by isolating it. `0x01C4`-`0x01CA` is now ruled out the same way:
sent slowly, with attribution unambiguous, nothing came back.

The reason both looked positive is that **the client's own startup output varies enormously
between runs**. Its 4th packet - chain position 3, header `a=0xE35B` in every run - was
**10 bytes in run 1 and 2764 bytes in run 3**. Total opening burst ranged from 294 to 3393
bytes. Packets that arrive late in a variable burst are indistinguishable from replies if
you start talking too early.

So: **no inbound opcode has yet produced a confirmed response.** The only confirmed
reactions remain the two clean exits.

| Opcode | Effect | Confidence |
|---|---|---|
| `0x0023` | client closes the connection and exits cleanly | confirmed, reproduced |
| `0x01DC` | same clean exit | confirmed once |
| everything else in `0x0000`-`0x01DC` | no visible effect | with a 32-byte zero body |

### Method, corrected

`--quiet-before` (default 5 s) now sets how long the client must be silent before we speak,
and every client line carries a timestamp plus its age since our last packet. Attribution
is only trustworthy when the client has gone quiet first and the reply lands promptly.

Also worth keeping in mind: a null result may mean the **body** was wrong rather than the
opcode. Everything so far has been sent with 32 zero bytes, so an opcode whose handler
needs real content may well have run and done nothing visible.

## Superseded: sweep coverage

> **Blind sweeping is retired.** It cost ~2 opcodes per client launch once live handlers
> started taking the client down. Read the stage's `OnPacket` switch, or walk the space
> in-process with `-Probe`. See `STATUS.md`.

`0x0000`-`0x03C7` has been swept with a 32-byte zero body. No inbound opcode has produced
a confirmed reply or any visible UI change. The only reproducible reactions are clean
exits, at `0x0023`, somewhere in `0x01CB`-`0x01DC`, somewhere in `0x01DD`-`0x01F1`, and
around `0x03C5`.

Every apparent "reply" has turned out to be the client's own asynchronous traffic. The
50-byte packet that keeps showing up is the next packet after the opening burst - chain
position 16 in one run, 18 in another - arriving at +10.3 s, +19.7 s and +94.1 s in
different runs. It is not a response to anything.

**Blind sweeping is now poor value.** ~3000 opcodes remain, exits truncate each run, and a
null result is ambiguous anyway because the body is always 32 zero bytes.

### Ruled out: the hang was not an external network wait

> Settled twice over: `tools/client-sockets.ps1` found loopback only, and then the stack
> dump showed the main thread parked in `WS2_32!recv` on our own socket. The hang was our
> protocol, and specifically the `Data.wz` exchange.

The hypothesis was worth checking. The firewall rule `MapleCW - block patched client outbound` blocks the patched client from
reaching **any** external host (`RemoteIP: Any, Protocol: Any`). Windows Firewall does not
filter loopback, which is why our probe on `127.0.0.1:8484` works while everything outbound
is silently dropped.

If the client blocks on an external endpoint - auth, CDN, telemetry - it hangs at a white
screen no matter what we send, and every null sweep result is explained without any opcode
being wrong. `tools/client-sockets.ps1` settles it: run it while the client is hanging and
look for a non-loopback socket. `SYN_SENT` to an external address means the firewall is
dropping it; only-loopback means the hang really is our protocol.

And the measurement:

`tools/client-sockets.ps1`, run against a hung client (pid 64140, `responding=False`,
57 threads), across six samples:

```
      State Local           Remote         Loopback
Established 127.0.0.1:53509 127.0.0.1:8484     True
      Bound 0.0.0.0:53509   0.0.0.0:0          True
```

One socket, loopback, ours. The client never attempts an external connection, so the
outbound firewall rule is not implicated and the hang really is our protocol. The sweep
premise holds. `responding=False` also says the UI thread is blocked rather than spinning.

### Ruled out: the outbound keystream is not the fixed default IV

`FUN_140c75880` falls back to a fixed 16-byte IV at `0x143307778` when passed a NULL IV,
which would make every packet share one keystream. The captures disprove it: the same
50-byte client packet appears at chain position 16 in two runs (identical first 10 payload
bytes) and at position 18 in another (`72 61 FB B9 ...` versus `AE F8 7C E7 ...`). A fixed
keystream would give the same ciphertext prefix at any position. So the outbound keystream
*does* vary with chain position — consistent with IV-based OFB, which is what makes its
resistance to the stock key so odd.


## What the client actually sends (decrypted)

`research/fixtures/capture-handshake-ok.log`, decrypted with the real key:

| # | Opcode | Body | Source |
|---|---|---|---|
| 0 | `0x0070` | `02`, `u32 100` | `FUN_1415d5b40` — version report |
| 1 | `0x0071` | `01`, `u32 1`, `u32 100`, `00`, ... | `FUN_1415d5c20` — environment report |
| 2-13 | `0x00A6` | one `u32` id each: 1, 11, 2, 5, 3, 4, 12, 13, 14, 15, 17, 18 | an enumeration of some kind |
| 14 | `0x00A1` | `u32 0` | the conditional `0xA1` in the handshake tail |
| 15 | `0x0070` | `01`, `u32 100`, ... | second version packet |

Runs whose opening burst was large show `0x008F` and `0x0090` in the same stream — the log
uploads, exactly as predicted from `FUN_1415dde80`/`FUN_1415ddf60`.

Every builder decoded here matches its decompiled source field for field, which is the
strongest possible confirmation that the whole transport — framing, header rule, IV chain
and cipher — is correct.

## Everything the sweeps "found" is void

All sweep results predate the key fix, so the client never once saw an opcode we intended.
The scattered exits at `0x0023`, `~0x01DC`, `~0x01F1` and `~0x03C5` were random garbage
opcodes occasionally landing on a disconnect handler, which is why none reproduced — and
why sending `0x0023` alone did nothing. The whole `0x0000`-`0x03C7` range is unexplored
again, but that no longer matters much: with decryption working we can simply read what
the client asks for instead of guessing.

## The dispatcher is Themida-virtualised, not merely relocated

Dumped live from `0x144ADD569` (32 KB, 83.5% non-zero, `research/themida-dispatch.bin`).
It is not plain code:

```
9c              pushfq
50              push rax
48 89 c0        mov rax, rax          <- no-op filler
48 83 ec 08     sub rsp, 8
48 89 0c 24     mov [rsp], rcx
48 b9 7f 01 ... mov rcx, 0x17F
51              push rcx
8f 44 24 08     pop qword [rsp+8]     <- value moved through the stack
48 8b 0c 24     mov rcx, [rsp]
```

83 `pushfq` in 32 KB, constants laundered through push/pop, no-op filler: a Themida VM
handler. Scanning the region for an opcode->handler table found 317 scattered image
pointers but **no run of 4 or more consecutive ones**, so the table is not there in plain
form either.

Reversing a Themida VM is a project in itself, not a next step. The inbound opcode table
should be considered unavailable by static *or* simple dynamic means.

### What that leaves

1. **Do not blind-sweep.** It was harmless when our packets were noise; now that they
   decrypt, every opcode reaches a real handler with a 32-byte zero body, and the client
   dies of malformed input at semi-random points (`0x001B` survived in isolation but the
   run died later at `0x0030`).
2. **Find a handler, then find its address in the table.** Handler *functions* are probably
   normal code — only the dispatch is virtualised. If the function behind the login-screen
   transition can be identified statically, searching process memory for its address would
   locate its table slot, and the slot index gives the opcode.
3. **Hook the client.** A DLL in the client's own process could log what the dispatcher
   does, which sidesteps the VM entirely.

## The dispatcher hook works

`crates/grap-stub/src/hook.rs`, enabled by the marker file `maplecw-hook.enable`:

```
install_once: our code IS running. env=false marker=true -> installing
install: base=0x140000000 target=0x1415d60e0
install: hook active
    0 opcode=0x0000 elapsed_us=133.4 ret=109729296
   25 opcode=0x0019 elapsed_us=133.3 ret=109729368
```

**Themida does not checksum this part of `.text`** — the inline patch survives and the
client runs on. Three things follow:

1. **The transport is confirmed from inside the client.** Every opcode we sent arrives at
   the dispatcher as the opcode we intended, which independently validates framing, header
   rule, IV chain and AES key at the far end rather than by inference.
2. **No handler among `0x0000`-`0x0019`.** Elapsed time is flat at 119-159 us across all
   26 packets, with no outlier. Either none of these has a handler, or ~130 us is fixed
   dispatcher overhead and a real handler must be looked for as a much larger figure.
3. The `ret` value cycles through exactly **six** addresses 56 bytes apart, which looks
   like a six-entry buffer pool.

> **Correction, then confirmation.** A first reading concluded a ~25-packet ceiling from a
> log that was still being written and had two runs concatenated (the DLL appends).
> `test-one.ps1` now clears `client-patched/maplecw-hook.log` per run. A clean re-test then
> established the ceiling properly.

### The ceiling is real: 26 packets, and it is a leak not a rate limit

| Run | Spacing | Packets accepted | Last opcode |
|---|---|---|---|
| fast | 0.2 s | **26** | `0x0019` |
| slow | 3.0 s | **26** | `0x0019` |

Fifteen times the spacing, identical count. So the client is not being overrun — something
is consumed per packet and never returned. The dispatcher's return value cycling through
six addresses 56 bytes apart says that something is a **six-entry buffer pool**; unhandled
packets evidently never release their buffer.

**Now fully explained.** The pool is consumed inside `FUN_1415e7090`'s startup loop, which
allocates a `0x5b4` buffer per packet and only exits when `conn+0x150` is set — so
unhandled packets accumulate buffers the loop never returns. Answer `0x0032` and the loop
exits, and the ceiling goes with it.

**It retired sweeping either way.** At 26 opcodes per launch the ~4000-opcode space needs
about 150 runs, and past the login screen live handlers cut it to roughly 2 per launch.
The replacements are the stage `OnPacket` switch and the in-process walk.

Timing looked useless at first: across `0x0000`-`0x0019` the spread is 102-148 us, median
131, with no value even twice the median. **That was the range, not the technique.** A
handled opcode is obvious — `0x00A1` came back at 9122 us against the same ~130 us
baseline, and `0x0010` at 355 us with its handler allowed to run. `0x0000`-`0x0019` simply
has no handlers on this connection.

### Superseded: "scan memory for the handler's address"

> **That plan failed, and the record is worth keeping.** `FUN_1415e5c20` appears nowhere
> as data — not as an aligned qword in 1004 MB of committed memory, and in the image only
> as its own `.pdata` entry. There is no table of handler pointers; the mapping lives
> inside the Themida VM. The answers came instead from an in-process opcode walk
> (`0x0032`) and from a stage's readable `OnPacket` switch (`0x0010`).

The hook proves arbitrary code can run inside the client, and that is the lever. Rather
than probing opcodes from outside, identify the function behind the login-screen
transition — handler *functions* are ordinary code, only the dispatch is virtualised — then
scan process memory for its address. Its slot in the dispatch table gives the opcode
directly, with no guessing and no per-run packet budget.

## `conn+0x48` is the connection type, and it changes three things at once

**Established 2026-08-19, statically.** This one field is why the migrated connection was
rejected, and it had been sitting in this document as a cipher-selection note without anyone
noticing what else it gates.

```c
if (*(int *)(conn + 0x48) == 0) ...   // a GAME / CHANNEL connection
else                            ...   // a LOGIN connection
```

It is read in three separate places in `FUN_1415d10e0`, and each one changes the wire:

| Read at | Effect when `+0x48 == 0` (channel) |
|---|---|
| the greeting parse | the leading `A..F` block is **not read** |
| the version block | `low`, `high`, `temp` are **not read** |
| `FUN_1406e9a65` | *appears* to select the byte subtract - **but the channel is AES, measured; see below** |

### So a channel greeting is a different packet

The login greeting we send is 48 bytes with two optional blocks in it. A channel connection
skips both, which means the client reads `G` from where we put `A`. We send `A = 00 00`, so
the client reads `G = 0`, fails `G == 1 && H == 1`, and raises site `0x348` /
`0x22000007` - **"The client is outdated"**. That is exactly the dialog that ended the
first run to enter the world, and it needs no other explanation.

The channel greeting should therefore be, in order:

```text
G   u16    1, with bit 0x8000 clear
H   u32    1
I   str    empty, so atoi(I) == 0
J   u32    client send IV  -> conn+0xe8
K   u32    client recv IV  -> conn+0xec
L   u8     1
M   u8
N   u8
O   u8     locale
```

with **no** `A..F` and **no** `low`/`high`/`temp`. That is a hypothesis derived from the
parse, not a measurement - but it is the only shape consistent with the three reads above.

### RETRACTED: the channel body cipher is **AES**, not a byte subtract

**Measured 2026-08-19.** The client's first two packets on a real channel connection decode
under AES-256-OFB, the same cipher and the same key as the login connection:

```text
packet 1  ->  0x0070, subtype 2, version 100 - byte for byte the body it sends on login
packet 2  ->  0x007D, character id 204, the machine MAC and machine id
```

Neither direction of the byte shift produced a plausible opcode, and both left the body
high-entropy. So the channel needs **no new cipher at all** - `MapleCipher` serves both
connections, and `crates/world` uses it.

`ByteShiftCipher` remains in `crates/net`: it is a faithful reading of `FUN_1406ef9f0` and
it is tested, but nothing uses it, and it is **not** the channel's cipher. Whatever code
path does use it has not been identified.

The reading of `FUN_1406ef9f0` below is still accurate as a description of that function.

### The byte subtract, as a transform (not the channel's)

`FUN_1406ef9f0(dst, src, len, iv, flag)`, read directly:

```c
cVar40 = (flag == 0) ? 0 : *iv;      // the FIRST byte of the 4-byte IV
out[i] = in[i] - cVar40;             // wrapping 8-bit subtract, whole body
```

Everything around it is unchanged from the AES path, which matters because it means most of
`crates/net` is reusable:

* the **4-byte header is not ciphered** either way - the caller does `buf + 4` before
  calling the transform, so `decode_len` and the `len = a ^ b` header stay as they are;
* the **IV evolution** is the same stock shuffle table, stepped once per packet;
* the **chunking** is the same `0x5B0` then `0x5B4`.

Only the body transform differs. A `ByteShiftCipher` implementing `Cipher` alongside
`MapleCipher`, sharing the header and IV logic, is the whole change.

The direction - which side subtracts - was never settled, and no longer needs to be:
nothing sends or receives with this transform. Recorded because the transform itself is
real code in the client.

### Why "the transport works" never proved the handshake was right

The IVs are written to `conn+0xe8`/`+0xec` at `FUN_1415d10e0` lines 426-427. The
`G == 1 && H == 1` check is at line 606, **180 lines later**. So the client can parse our
greeting, take the IVs, and *then* throw - which is exactly what it does: the throw is
caught by the message-loop handler, the client carries on, and the only trace is the `ELog`
upload nobody was reading. Working framing and working AES were never evidence about the
version check.
