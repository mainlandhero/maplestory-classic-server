# WZ format — as used by the `mscw` client

Determined empirically 2026-08-14 from `C:\Nexon\Library\maplestorycw\appdata\Data`.

## Key parameters (this client)

| Parameter | Value |
|---|---|
| Magic | `PKG1` |
| Copyright | `Package file v1.0 Copyright 2002 Wizet, ZMS` |
| `encVer` (u16 @ `fstart`) | **34** (`0x22`) |
| **Game version** | **779** |
| **Version hash** | **`0x0000E73A`** |
| **String key** | **ZERO** — no AES; XOR-mask only |
| Offset magic | `0x581C3F6D` |

The zero key is a significant simplification: strings are obfuscated with only the
positional XOR mask, not the AES keystream used by GMS/KMS builds.

## On-disk layout (split archives)

```
Data/<Tree>/<Tree>.ini       ->  "LastWzIndex|N"
Data/<Tree>/<Tree>.wz        ->  stub header (often an EMPTY directory)
Data/<Tree>/<Tree>_000.wz    ->  the real, standalone PKG1 archive
```

- `Base/Base.wz` is the master index: 15 directory entries (Character, Effect, Etc, Item,
  Map, Mob, Morph, Npc, Quest, Reactor, Skill, Sound, String, TamingMob, UI), each `size=0`
  and pointing at a 1-byte empty-directory stub at the tail of the file. It names the trees;
  it does not contain them.
- `String/String.wz` is 63 bytes with an entry count of **0**. All content is in
  `String_000.wz`, which carries its own complete `PKG1` header.

**Therefore: parse `<Tree>_000.wz` directly as a standalone archive.** Offsets are relative to
that file's own `fstart`.

Validation: `String_000.wz` yields 24 images, 0 out-of-bounds, offsets strictly ascending,
and the final image (`UI.img` @ `0xD4425`, size 13) ends at 869426 = exact file length.

## Header

```
u8[4]   magic       "PKG1"
u64     fsize       size of the data section (file length - fstart)
u32     fstart      offset of the content section
cstr    copyright
--- at fstart ---
u16     encVer      34
cint    count       number of root entries
```

## Directory entry

```
u8   type
  1 -> unknown/skip 10 bytes
  2 -> name stored elsewhere: i32 relative offset; real type byte at (fstart+1+off)
  3 -> subdirectory
  4 -> image (.img)
<name>      (only for types 3/4; type 2 reads it at the referenced position)
cint size
cint checksum
u32  offset     (encrypted)
```

## String decoding (key = zero)

```
i8 len
len < 0 : ASCII,   n = -len,   byte[k] ^= (0xAA + k) & 0xFF
len > 0 : UTF-16LE, n = len,   u16[k]  ^= (0xAAAA + k) & 0xFFFF
```

## Offset decryption

```
o  = (offset_field_position - fstart) ^ 0xFFFFFFFF
o  = o * versionHash            (mod 2^32)
o  = o - 0x581C3F6D             (mod 2^32)
o  = rotl32(o, o & 0x1F)
o ^= encrypted_offset
o  = o + fstart * 2             (mod 2^32)
```

## Version hash derivation

```
sum = 0
for ch in decimal_string(version):  sum = sum * 32 + ord(ch) + 1
encVer_check = 0xFF ^ (sum>>24 & 0xFF) ^ (sum>>16 & 0xFF) ^ (sum>>8 & 0xFF) ^ (sum & 0xFF)
```
`version = 779` gives `sum = 0xE73A` and `encVer_check = 34`, matching the file.

> Note: the same 779 / `0xE73A` pair is expected to appear in the **network handshake**
> (the client sends its version on connect). Cross-check during Stage 2.

## Image (`.img`) property format — to be implemented

Standard WZ property serialization. Types: `0x00` null, `0x02/0x0B` short, `0x03/0x13` cint,
`0x04` float, `0x05` double, `0x08` string, `0x09` extended (Property / Canvas /
Shape2D#Vector2D / Shape2D#Convex2D / Sound_DX8 / UOL). Strings inside images use
offset-reference encoding (`0x00/0x73` inline, `0x01/0x1B` u32 offset from image start).
