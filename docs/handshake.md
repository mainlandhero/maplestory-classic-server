# Connection handshake

Status: **format not yet reproduced.** Two solid results so far — the classic format is
ruled out, and the protocol version is known.

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

## The handshake is a serialised structure, not a fixed layout

The decode is field-by-field through two helpers:

```c
consumed = FUN_1406e8380(&dest_u32, cursor, remaining);  cursor += consumed;
consumed = FUN_1406e82f0(&dest_u8,  cursor, remaining);  cursor += consumed;
```

Each returns **how many bytes it consumed**, and the cursor advances by that amount. A
fixed-width layout would not need a returned length — this is variable-length encoding
(varint-style), which is why a fixed classic greeting gets nowhere.

Read in order at the version block:

```
u32-ish  server version range low     -> compared against 100
u32-ish  server version range high    -> compared against 100
u32-ish  nClientVersion_Temp
u8-ish   (flag)
```

preceded by several more fields decoded the same way.

## Next

1. **Decompile `FUN_1406e8380` and `FUN_1406e82f0`** — they define the wire encoding.
   Both are in unpacked `.text`, so this is tractable and is the direct path to a
   greeting the client will accept.
2. Rebuild the probe around that encoding, with version **100**.
3. Only then revisit whether `MapleSecurePC64` also encrypts the stream; so far the
   evidence points at plain-but-serialised rather than encrypted, since the client reads
   and rejects rather than failing to decrypt.
