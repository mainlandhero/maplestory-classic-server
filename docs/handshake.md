# Connection handshake

Status: **layout recovered, not yet confirmed on the wire.** The classic format is ruled
out, the protocol version is known, and the field readers and their order have been
decoded from the client.

## The client waits for us

Launched as `MapleStory.exe -NXLDEBUG 127.0.0.1 8484`, the client connects and sends
**zero bytes**. The server speaks first.

## The classic MapleStory handshake is ruled out

`tools/handshake_probe.py` sends a candidate greeting on each connection. Nine launches,
one variant each:

| Sent | Client behaviour |
|---|---|
| **random 16 bytes** *(control)* | no reply, exit `0x00000000` |
| **length-prefixed garbage** *(control)* | no reply, exit `0x00000000` |
| classic `v779`, locale 8 / 1 / 2 / 5 / 0 | no reply, exit `0x00000000` |
| classic `v779`, sub-version `KR` | no reply, exit `0x00000000` |
| classic `v779`, empty sub-version | no reply, exit `0x00000000` |

**The controls are the point.** Random bytes produce *exactly* the same behaviour as a
well-formed classic greeting, so the client is not parsing that format at all. Sweeping
version numbers or locale bytes against this format would have been wasted effort.

Note that receiving *anything* makes the client exit cleanly, whereas receiving nothing
leaves it running — so it does read the socket, fails to make sense of the bytes, and
shuts down deliberately.

## Protocol version is 100 (not 779)

From `FUN_1415d10e0` (`research/msexe-handshake.c`), the function holding
`First Connect : nClientVersion_Temp : %d` and `Version Check`. Every version-related
call passes the literal **100**:

```c
"LoginSvr Ver %d / nTargetVersion %d_%d / Range %d ~ %d / Temp %d", 100, ...

if ((server_low < 0x65) && (99 < server_high))        // 0x65 == 101
    "OK. Allowed Version. %d ( %d~%d )", 100, ...
else if (server_low < 0x65)
    "High Version. Error. %d ( %d~%d )", 100, ...
else
    "Low Version. Launch Patch %d ( %d~%d )", 100, ...
```

So the client's own version is **100**, and it accepts a server whose advertised range
brackets it (`low <= 100 <= high`). **779 is the WZ data version only** — the two are
unrelated, which is worth remembering before reaching for 779 again.

The function also distinguishes a **first connect** from a **second connect**
(`local_res18[0]`, i.e. the third parameter), taking different validation paths. The
login and channel connections are therefore not identical.

## The field readers (decoded)

> **Correction.** An earlier revision of this document called the encoding
> "variable-length / varint-style", reasoning that a fixed layout would not need each
> reader to return a consumed length. That was wrong. Decompiling the readers shows the
> returned length is simply the field's **constant** width; it exists so the caller can
> advance a cursor uniformly, and so every read is bounds-checked. The layout is fixed.

All four readers take `(dest, cursor, bytes_remaining)`, throw a C++ exception if the
buffer is too short, and return the number of bytes consumed:

| Function | Reads | Consumes |
|---|---|---|
| `FUN_1406e82f0` | `u8` | 1 |
| `FUN_1406e8330` | `u16` | 2 |
| `FUN_1406e8380` | `u32` | 4 |
| `FUN_1406e84d0` | **string**: `u16` length, then that many bytes | `2 + len` |

The string reader is the familiar MapleStory shape — a `u16` byte count followed by the
bytes — which `crates/net`'s `PacketWriter::str` already produces.

## Handshake layout

Reconstructed from the read order in `FUN_1415d10e0`. Two blocks are gated on the same
condition, `cfg+0x48 != 0`:

```
--- only when cfg+0x48 != 0 ---
u16      A
string   B          (post-processed into a u16)
u32      C
u32      D
u8       E
u8       F

--- always ---
u16      G          ** high bit 0x8000 is a flag, then masked to 0x7FFF **
u32      H
string   I          (post-processed into a u16)
u32      J
u32      K
u8       L

--- only when cfg+0x48 != 0 ---
u32      server version range LOW    -> must be <= 100
u32      server version range HIGH   -> must be >= 100
u32      nClientVersion_Temp         -> 0 or 100 selects the check path
u8       M
u8       N
u8       O
```

Field `G` is notable: the top bit is pulled out as a boolean before the remaining 15 bits
are used, so it is a packed flag plus value.

`cfg+0x48` is a field of the same config struct the launch parser fills
(`docs/launch-protocol.md`), so **the handshake shape depends on the launch mode** — more
evidence that `-NXLDEBUG` (mode 5) and `WEBSTART` (mode 3) are not interchangeable.

## Next

1. Build a greeting from the layout above with version **100**, and send it on connect.
   Send both variants — with and without the `cfg+0x48` blocks — since we do not yet know
   which side of that condition `-NXLDEBUG` puts us on.
2. If the client replies, the framing is right and the login flow can start.
3. Confirm whether an outer length/opcode header precedes this structure: the parser
   works over a buffer bounded by `local_2278`, so something upstream already framed it.
   That framing is the remaining unknown.
4. `MapleSecurePC64` still looks less relevant than feared — the client reads plaintext
   fields here rather than failing to decrypt.
