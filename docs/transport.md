# Packet transport

What sits on the socket *after* the greeting in `docs/handshake.md` is accepted. Captured
from a real client run: `research/fixtures/capture-handshake-ok.log`, 294 bytes, 16
packets.

Status: **the server -> client direction is solved and CONFIRMED against the real client** —
it accepted a frame we built and stayed connected. The client -> server payload cipher is
still opaque, and we now know why: that code path does not exist on disk.

> **Confirmed 2026-08-15.** A `--reply header` run (valid header, body withheld) left the
> client connected instead of dropping it. That is the first frame the client has ever
> accepted from us, and it validates the header rule, the `0xFFFE` constant, the `K` IV
> seed and the shuffle table together.
>
> Beware the failure mode that preceded it: an exception in the probe closed the socket,
> and the client dropped because the *server* vanished — indistinguishable from a
> rejection. Always check `probe.err` is empty before believing a negative result.

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

```c
if (*(int *)(param_1 + 0x48) == 0) mode = 2;   // game/channel connection
else                               mode = 1;   // login connection
```

| Mode | Function | Algorithm |
|---|---|---|
| 1 | `FUN_140c75880` | AES-256-OFB |
| 2 | `FUN_1406ef9f0` | `out[i] = in[i] - (iv & 0xFF)` — a plain byte subtract |

So the **login** connection is AES and the **game** connection uses the trivial subtract.
The channel server is far cheaper to talk to.

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

## Open: the client -> server payload still does not decrypt

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
