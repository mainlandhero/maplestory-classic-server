# Packet transport

What sits on the socket *after* the greeting in `docs/handshake.md` is accepted. Captured
from a real client run: `research/fixtures/capture-handshake-ok.log`, 294 bytes, 16
packets.

Status: **the server -> client direction is fully specified and reproduces the client's
own headers byte-exactly. The client -> server payload cipher is still opaque, and we now
know why: that code path is Themida-obfuscated.**

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

**Why: the send path is obfuscated.** `FUN_1415d3990` is the send entry point (called by
both `FUN_1415d5b40` for opcode `0x70` and `FUN_1415d5c20` for `0x71`), and Ghidra hits
`halt_baddata()` partway through it. `FUN_1415d60e0`, the step that runs immediately after
decryption in both receive loops, is a 22-byte stub that is *entirely* `halt_baddata()`.
Themida has protected exactly the two functions that would show the extra transform.

So the paradox is resolved: every primitive we can read is stock, and the transform that
makes the capture opaque lives in code that cannot be read statically.

### Next

1. **Do not block on this.** The direction we need to *produce* is fully specified and
   testable against the client, which is the real oracle. `tools/transport.py` implements
   it; `handshake_probe.py --reply` exercises it.
2. If the client -> server direction is needed later, it will take dynamic analysis
   (breakpoint after `FUN_1406e99e0` and read the buffer) rather than more decompiling.
3. Remember the **game/channel connection uses mode 2**, so none of this AES applies there.
