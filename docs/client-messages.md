# Client messages: decrypting the string table

The client's user-facing messages are **XOR-encrypted** inside `MapleStory.exe`, which is
why searching the binary for a dialog's text finds nothing. `tools/dump_stringids.py`
decrypts all **6165** of them.

This matters beyond curiosity: the client reports failures as dialogs, and a dialog is
often the only feedback we get. Being able to go **dialog text → string ID → error code →
the exact branch that raised it** turns a vague "it didn't work" into a precise location
in the binary.

## How it is stored

Resolution is `FUN_1408a9e40(dest, id)` → `FUN_1408aac40(id)` → `FUN_1408aa850(entry, id, locale)`:

```c
locale_tables = (u64 *) 0x143A563F8      // one pointer per locale
entries       = (u64 *) locale_tables[locale]
entry         = entries[id]              // 0x1815 (6165) ids
seed          = *(u8 *) entry            // first byte is a key seed
ciphertext    = cstring at entry + 1
```

### Key schedule — `FUN_1408aa600`

Base key, 16 bytes at `0x1432BA470`:

```
d6 de 75 86 46 64 a3 71 e8 e6 7b d3 33 30 e7 2e
```

Rotated per string by its seed byte:

1. if `seed >= 8`: rotate the key left by `(seed >> 3) % 16` **bytes**
2. rotate the whole key left by `seed & 7` **bits**

### Decryption — `FUN_1408aadb0`

Repeating-key XOR, with one quirk:

```c
k = key[i % 16];
p = c ^ k;
if (p == 0) p = k;     // never introduce a NUL, which would truncate the string
```

Confirmation that the schedule is right: ID 0 decrypts to a timestamp and IDs 1–23 to
recognisable Nexon URLs.

## Usage

```bash
python tools/dump_stringids.py --grep "outdated"
python tools/dump_stringids.py --id 106
python tools/dump_stringids.py --all-locales
```

## Error code → string ID

`FUN_141803cd0` maps an internal error code to the message shown. Recovered table:

| Error code | String ID | Message (abridged) |
|---|---|---|
| `0x21000001` | 97 | — |
| `0x21000002` | 98 | — |
| `0x21000003` | 99 | — |
| `0x21000004` | 109 | — |
| `0x21000006` | 111 | — |
| `0x21000007` | 112 | — |
| **`0x22000001`** | **100** | "You cannot access the game … maintenance / banned IP / connection not stable" |
| `0x22000002`, `0x2200000f` | 101 | "You have been disconnected from the login server." |
| `0x22000003` | 102 | not enough memory |
| `0x22000004` | 103 | "unable to locate a MapleStory data file" |
| `0x22000005` | 104 | "The client is outdated…" (short form) |
| `0x22000006` | 105 | "missing a few files needed to run the game" |
| **`0x22000007`** | **106** | **"The client is outdated. \r\nPlease download the latest client \r\nfrom maplestory.nexon.net and try again."** |
| `0x22000008` | 108 | "An error occured while communicating with the server." |
| `0x2200000a` | 110 | — |
| `0x2200000b` | 114 | — |
| `0x2200000e` | 116 | — |

## Reading the two dialogs we have actually seen

**"The client is outdated…"** = `0x22000007`. Raised from **three** places in the
handshake handler `FUN_1415d10e0`, which is why it is ambiguous on its own:

| Site | Condition |
|---|---|
| `FUN_140cc2350(…, 0x2df, 0x22000007)` | field **L != 1** — fires *before* the version fields are even read |
| `FUN_1415e0fb0(…, 0x33b, 0x22000007, &low)` | second connect, "High Version. Error." |
| `FUN_1415e0e30(…, 0x327, 0x22000007, &L)` | first connect, "Client Version is Higher" |

**"You cannot access the game…"** = `0x22000001`, from
`FUN_1415e0eb0(…, 0x23d, 0x22000001, …)`. The owner confirms this is the same dialog the real
client shows when Nexon's servers are down — i.e. the generic *connection/parse failed*
message. We reach it by sending a body that is too short, so a bounds-checked field read
throws.

That pair is what makes the "truncate after L" experiment work as a discriminator: the
same `L=1` payload gives the *outdated* dialog when complete and the *cannot access*
dialog when truncated, proving the L gate was passed and the failure moved later.
