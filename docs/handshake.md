# Connection handshake

Status: **framing confirmed; body parsed; field `L` gate found and passed; version
comparison still not satisfied.** See `docs/client-messages.md` for how the client's
dialogs are decoded back to error codes — that is what makes progress measurable here.

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

## Framing: `u16` length, then the body

From the receive loop in `FUN_1415d10e0` (`research/msexe-handshake.c`, the block before
the field reads):

```c
buf = connection_buffer();  start = buf;  cursor = buf;
total_len = 0;  have_len = 0;

loop:
    want = have_len ? total_len - (cursor - start)
                    : 2 - (cursor - start);      // exactly 2 bytes first
    n = recv(sock, cursor, want);
    cursor += n;
    if (n == 0) goto disconnected;

    if (!have_len && (cursor - start) == 2) {
        total_len = *(u16 *)start;               // the length prefix
        if (buffer_capacity < total_len) goto abort;
        have_len = 1;
        cursor = start;                          // rewind, overwriting the prefix
        goto loop;
    }
    if (!have_len || (cursor - start) != total_len) goto loop;

end = cursor;        // start + total_len
cursor = start;      // fields are then parsed from offset 0
```

So on the wire:

```
u16   body_length      (little-endian; does NOT count these two bytes)
u8[body_length]        body — the field layout below, starting at offset 0
```

The rewind is the detail worth noting: the two length bytes are consumed and then
**overwritten** by the body, so the body genuinely starts at offset 0 and the length is
not part of the parsed structure.

This also means the earlier probe had the framing **right** (`0E 00` + 14 bytes) and only
the body wrong — consistent with the client reading, failing to make sense of the
contents, and exiting.

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

**Field `L` is a hard gate.** Immediately after the fields are read:

```c
if (L == 1) {
    ... proceed ...
} else {
    FUN_140cc2350(..., 0x2df, 0x22000007);   // -> "The client is outdated"
}
```

Sending `L = 0` produces the outdated dialog *before the version fields are examined at
all*, which is why every version combination looked identical until this was found.

`cfg+0x48` is a field of the same config struct the launch parser fills
(`docs/launch-protocol.md`), so **the handshake shape depends on the launch mode** — more
evidence that `-NXLDEBUG` (mode 5) and `WEBSTART` (mode 3) are not interchangeable.

## Confirmed on the wire

Tested against the running client (`-NXLDEBUG 127.0.0.1 8484`), with the client's own
error dialog as the signal:

| Payload | Client behaviour |
|---|---|
| random bytes / length-prefixed garbage | silently exits, **no dialog** |
| recovered layout, **gated blocks absent** | silently exits, **no dialog** |
| recovered layout, **gated blocks present** | **"The client is outdated"** dialog |

This is the important result. A structured payload produces a *specific version error*
where garbage produces nothing, which means:

1. **The framing is right.** `u16` length + body is accepted.
2. **The client parses our body.** It gets far enough to run the version comparison and
   report a verdict on it.
3. **`cfg+0x48 != 0` under `-NXLDEBUG`** — the gated blocks belong, since omitting them
   loses the dialog entirely.

## The dialog is NOT the version check (corrected)

A single test settled this. Sending a body with **every `u32` slot set to 100** — so
whichever slot the client reads as the version range must read 100 — still produced
"The client is outdated".

Under `FUN_1415d10e0` that is impossible: `low = 100, high = 100` satisfies
`low < 101 && high > 99` on the second-connect path, and `high == 100` is the
`Version OK` branch on the first-connect path. Neither can reach a patch/outdated
verdict.

**So the dialog does not come from the version check in `FUN_1415d10e0`.** Earlier
revisions of this document attributed it there; that was wrong.

Supporting evidence: the exact dialog text appears **nowhere as plaintext** in any
client binary — not `MapleStory.exe`, not `String.wz`, not any DLL. A search across the
whole install finds only unrelated matches in `NexonAnalytics64.dll` and CEF resources.
The message is therefore assembled or looked up by **error code**, matching the pattern
already seen in the launch parser:

```c
FUN_1429e4fa0("http://maplestory.nexon.net/micro-site/20701", 0, 0);
FUN_141804870(&DAT_143271f04, 0x1a4, 0x22000009, ...);   // error code
```

Known codes in these paths: `0x195`, `0x1a4`, `0x23d`, `0x243`, `0x327`, `0x33b`,
`0x3dc`, `0x3df`, `0x3e3`, `0x3e6`.

What still holds: the framing is right and the client parses our body — a structured
payload produces a specific error while garbage produces silence. What is wrong is the
assumption about *which* check is failing.

## Superseded: the version-field misalignment theory

Every combination tried produces the same "outdated" dialog:

```
(low,high,temp) = (1,200,100) (1,100,0) (100,100,0) (1,100,1) (0,100,0)
                  (100,100,100) (1,100,100) (99,101,0) (1,200,0)
```

`high = 100` should take the `Version OK` branch and `low = 1, high = 200` should take
`OK. Allowed Version`, yet both still report outdated. So the client is **not** reading
our intended values — the three version `u32`s are landing at the wrong byte offsets.

Since the dialog only appears when the *first* gated block is present, that block
(`u16, string, u32, u32, u8, u8`) is the prime suspect: if its shape is wrong, everything
after it shifts.

Exact bytes sent for `(1, 100, 0)`, for reference when re-deriving:

```
2E 00                          length = 46
00 00                          A   u16
00 00                          B   string, len 0
00 00 00 00  00 00 00 00       C, D  u32
00 00                          E, F  u8
00 00                          G   u16
00 00 00 00                    H   u32
00 00                          I   string, len 0
00 00 00 00  00 00 00 00       J, K  u32
00                             L   u8
01 00 00 00                    low  = 1     <- believed position
64 00 00 00                    high = 100   <- believed position
00 00 00 00                    temp = 0
00 00 00                       M, N, O
```

## Confirmed: `L = 1` is accepted

Both the `L` gate and a version mismatch raise the **same** error code (`0x22000007`) and
therefore the same dialog, so `L` could not be validated by the dialog alone. The
discriminator was to send `L = 1` with the body **truncated immediately after `L`**:

| Payload | Dialog | Meaning |
|---|---|---|
| `L=1`, full body | "The client is outdated" (`0x22000007`) | past the `L` gate, fails later |
| `L=1`, truncated after `L` | "You cannot access the game…" (`0x22000001`) | past the `L` gate; the version read runs off the end and throws |
| `L=0`, full body | "The client is outdated" | stopped at the `L` gate |

The truncated case producing a *different* error is the proof: `L = 1` is accepted, and
the failure has moved past it. The owner confirms `0x22000001` is the same dialog the real
client shows when Nexon's servers are down — i.e. the generic connection/parse failure,
exactly what a short read should look like.

## Still open: the version comparison

With `L = 1` and `low=1, high=100, temp=0`, the client still reports outdated. Per the
decompiled logic **both** branches should pass:

- second connect: `(low < 101) && (high > 99)` → `1 < 101 && 99 < 100` → OK
- first connect: `temp != 100` → `v = high = 100` → `v == 100` → "Version OK"

Since neither should fail, the three version `u32`s are still not landing where we place
them. A payload with *every* field set to a passing value simultaneously (`u8 = 1`,
`u32 = 100`, `u16 = 100`, strings `"100"`) changed the behaviour — the client exited
cleanly instead of showing a dialog — which suggests the checks were satisfied and the
failure moved on again.

## Next

1. **Map the error codes to messages.** Find what `FUN_1429e4fa0` and the
   `FUN_141804870` / `FUN_1415e0*` family do with a code, and which code produces the
   outdated text. That names the real failing check instead of guessing at it.
2. **Enable the client's own log.** `FUN_14019cfe0` writes `MapleStory.LOG`
   (built from the exe path, suffix `LOG`, guarded by a `ZtlLog` mutex) and the version
   branches log their actual numbers — `"Launch Patcher : %d < %d -> Target : %d_%d"`
   would name the values directly. It is gated behind `FUN_140933f30`; finding what
   enables it removes all the guesswork.
3. `MapleSecurePC64` still looks less relevant than feared — the client is reading our
   plaintext fields, not failing to decrypt.
