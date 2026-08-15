# Packet transport

What sits on the socket *after* the greeting in `docs/handshake.md` is accepted. Captured
from a real client run: `research/fixtures/capture-handshake-ok.log`, 294 bytes, 16
packets.

Status: **framing, header and IV chain solved and verified. Payload cipher identified in
the binary but not yet reproduced offline.**

## Framing

The classic MapleStory 4-byte header:

```
u16 a          # encodes the IV and a constant
u16 b          # a ^ payload_length
u8[a ^ b]      # payload
```

so `payload_length = LOWORD ^ HIWORD`. Verified against the capture: **16/16 packets, 294
bytes consumed exactly**, including three reads that contained several packets coalesced
by TCP.

## Header and IV chain — confirmed

```
a = ((iv[3] << 8) | iv[2]) ^ 0x00DF
```

The IV is a 4-byte value that evolves after every packet using the **stock MapleStory
shuffle table**, which is present verbatim in the client at VA `0x143A86890`.

This was verified end to end: seeding the chain with the exact `J` value the server sent
(`0x52307801`) predicts **all 16 observed headers exactly**. Seeding with `K` predicts
only the first — and only because `J` and `K` were chosen to differ solely in byte 0,
which the header does not use.

That settles three things at once: the initial IV really is the field we sent, the
evolution really is the stock algorithm, and the constant is `0x00DF`.

### The two IVs

Fields `J` and `K` of the greeting land at `conn+0xe8` and `conn+0xec`, the classic
send/recv pair:

| Slot | Field | Used for |
|---|---|---|
| `conn+0xe8` | `J` | client **send** — the stream we captured |
| `conn+0xec` | `K` | client **recv** — the stream we must produce |

`FUN_1415d36c0` is the receive loop and reads `conn+0xec`, re-shuffling it after each
packet via `FUN_140c78630(conn+0xec, 4, 0)`.

## Cipher mode is chosen by connection type

From `FUN_1415d36c0`:

```c
if (*(int *)(param_1 + 0x48) == 0) local_6c = 2;   // game/channel connection
else                               local_6c = 1;   // login connection
```

and `FUN_1406e99e0(view, iv, mode)` dispatches on it:

| Mode | Function | Algorithm |
|---|---|---|
| 1 | `FUN_140c75880` | AES |
| 2 | `FUN_1406ef9f0` | `out[i] = in[i] - (iv & 0xFF)` — a plain byte subtract |

So the **login** connection is AES and the **game** connection uses the trivial subtract.
That is worth remembering: the channel server is far cheaper to talk to.

`FUN_1406e9910` would encrypt only the 2-byte opcode, but it has **no callers** — the
whole payload is always processed.

Both crypt functions operate on `buffer + 4`, which independently confirms that the
payload begins after the 4-byte header.

## The AES, as implemented in the client

- **AES-256.** `local_154 = 8` is Nk = 8 words; the schedule expands to 60+ words.
- **Stock key.** `FUN_140c76070` reads its 8 key words from `0x143A86810` with a stride of
  4 dwords (`param_1[0]`, `[4]`, `[8]` … `[0x1c]`, and `param_1` is a `u32 *`), giving
  `13 08 06 B4 1B 0F 33 52` — the classic MapleStory key. The other 24 dwords in that
  128-byte table are **decoys**: they sit between the real ones and are never read.
- **Stock tables.** `FUN_140c759a0` builds them at runtime: GF(2^8) log/antilog with the
  `0x1b` polynomial, the S-box affine constant `0x63`, standard Rcon. No customisation.
- **OFB.** The keystream block is re-encrypted every 16 bytes and XORed into the data.
- **IV block** is the 4-byte IV repeated 4× (`local_178 = *param_4` copied to all four
  words) — the classic `multiplyBytes(iv, 4, 4)`.
- **Chunking** is `0x5B0` for the first chunk then `0x5B4`, matching the original exactly
  (`FUN_1406e99e0`, and `FUN_1406f1730(0x5b4, …)` in the handshake handler).

## Open: the payload does not decrypt

Every element above is stock, yet running AES-256-OFB with that key over the captured
payloads does not produce recognisable packets. Ruled out so far, all against packet 1:

- key: stock, the 32 low bytes of the table, raw first 32 bytes, per-word byte-swapped,
  rotations, AES-128 truncations
- IV: `J` and `K`, little- and big-endian, and every position in both chains for 24 steps
- post-step: with and without the classic Shanda transform
- mode 2 (byte subtract) — excluded independently, since the eleven 6-byte packets do not
  differ from each other by a constant

Two candidate explanations remain, in order of likelihood:

1. **The known-plaintext assumption is wrong.** The test above looks for opcode `0x70`
   first, on the grounds that `FUN_1415d5b40` runs before `FUN_1415d5c20`. But
   `FUN_1415d5aa0` runs *earlier still* and calls `FUN_1415d9cd0`, which was never
   examined and may send first. If so the oracle is invalid and the decryption may be
   closer than it looks.
2. **Something sits between the writer and the socket.** `FUN_1415dc6f0` appends a trailer
   before send, and `FUN_1415d60e0` / `FUN_1406e9530` in the receive loop are unexamined.

### Next

1. Decompile `FUN_1415d9cd0` to learn what — if anything — is sent before packet `0x70`,
   which decides whether the known-plaintext oracle above is sound.
2. Decompile the **send** counterpart of `FUN_1415d36c0` (the one reading `conn+0xe8`) and
   read the header construction directly, rather than inferring the `0x00DF` constant.
3. Once a packet decodes, the 11 consecutive 6-byte packets are the sanity check: they
   should share an opcode.
