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

> **These string IDs are also the login result codes.** The login result (inbound
> `0x0010`) carries a `u8` that lands in this table: **101 is `0x65`**, "You have been
> disconnected from the login server." — which is exactly the dialog produced by replying
> with `0x65`. **Result `0` is success**; any non-zero code raises a dialog from here. See
> `docs/opcodes.md`.

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

**"The client is outdated…"** = `0x22000007`. Raised from **four** places in the
handshake handler `FUN_1415d10e0`, which is why it is ambiguous on its own:

| Site | Condition |
|---|---|
| `FUN_140cc2350(…, 0x2df, 0x22000007)` | field **L != 1** — fires *before* the version fields are even read |
| **`FUN_140cc2350(…, 0x348, 0x22000007)`** | **fields `G != 1` or `H != 1`** — runs unconditionally, *after* the version block, and rejected every probe we sent for weeks |
| `FUN_1415e0fb0(…, 0x33b, 0x22000007, &low)` | second connect, "High Version. Error." |
| `FUN_1415e0e30(…, 0x327, 0x22000007, &L)` | first connect, "Client Version is Higher" |

Four sites behind one dialog is the trap this project kept falling into: a payload that
fixes the version fields still shows the identical message if `G`/`H` are wrong. When a
code has multiple sites, the only reliable move is to find an input that yields a
*different* code, then reason from the pair.

**"You cannot access the game…"** = `0x22000001`, from
`FUN_1415e0eb0(…, 0x23d, 0x22000001, …)`. The owner confirms this is the same dialog the real
client shows when Nexon's servers are down — i.e. the generic *connection/parse failed*
message. We reach it by sending a body that is too short, so a bounds-checked field read
throws.

That pair is what makes the "truncate after L" experiment work as a discriminator: the
same `L=1` payload gives the *outdated* dialog when complete and the *cannot access*
dialog when truncated, proving the L gate was passed and the failure moved later.

## Every error-dialog raise site in the image

**Complete, 2026-08-19.** Found by scanning `.text` for `mov r8d, <error code>` and pairing
each with the `mov edx, <n>` beside it, then resolving the containing function from
`.pdata`. Fourteen sites.

| at | in function | `edx` | decimal | code |
|---|---|---|---|---|
| `1415d1eea` | `FUN_1415d10e0` | `0x2df` | 735 | `0x22000007` client is outdated |
| `1415d25d3` | `FUN_1415d10e0` | `0x327` | 807 | `0x22000007` client is outdated |
| `1415d273d` | `FUN_1415d10e0` | `0x33b` | 827 | `0x22000007` client is outdated |
| **`1415d276d`** | `FUN_1415d10e0` | **`0x348`** | **840** | `0x22000007` client is outdated |
| `1415d134e` | `FUN_1415d10e0` | `0x23d` | 573 | `0x22000001` cannot access the game |
| `1415d27a4` | `FUN_1415d10e0` | `0x34e` | 846 | `0x22000001` cannot access the game |
| `142c49ebf` | `FUN_142c48ca0` | `0x73a` | 1850 | `0x22000001` cannot access the game |
| `1415d1399` | `FUN_1415d10e0` | `0x243` | 579 | `0x21000001` |
| `1415d27bd` | `FUN_1415d10e0` | `0x34e` | 846 | `0x21000001` |
| `142c4ce98` | `FUN_142c4c6e0` | `0x7e4` | 2020 | `0x22000005` client is outdated (short) |
| `142c95b71` | `FUN_142c94bd0` | `0x195` | 405 | `0x22000009` launch error |
| `142c95bba` | `FUN_142c94bd0` | `0x1a4` | 420 | `0x22000009` launch error |
| `141b3b170` | `FUN_141b3b110` | `0xfa5` | 4005 | `0x22000009` launch error |
| `141b3b1f4` | `FUN_141b3b110` | `0xfb2` | 4018 | `0x22000009` launch error |

**Ten of the fourteen are in `FUN_1415d10e0`.** The connection handshake is where almost
every fatal client-side error in this project comes from.

### The second argument is a source line number

`FUN_140cc2350(&DAT_143271f04, 0x348, 0x22000007)` reads as
`report(module, line, code)`. `DAT_143271f04` is **not** a string - it is a dword holding
`45`, so it is a module id, and the `edx` value is a `__LINE__`.

That is what makes the client's `ELog` upload directly usable: the number immediately before
`HR` in a record **is** this table's decimal column. `840` is the `G == 1 && H == 1` gate,
with no ambiguity and no client run.

### The two records in one failure

A single failure produces **two** `ELog` records, not two failures:

* line **840** - the raise, in `FUN_1415d10e0`.
* line **1327** - not a raise site at all. The only `mov edx, 1327` in `.text` that is
  followed by a report call is at `142c462ec` in `FUN_142c45e50`, and there the code comes
  from a **variable** (`mov r8d, [rsp+0x44]`) rather than an immediate - which is why the
  immediate scan above does not list it. That is a handler re-reporting a code it caught,
  near `main` (`FUN_142c42f30`). The other `mov edx, 1327` (`142cf5a57`) sits in a run of
  consecutive constants `0x52e, 0x52f, 0x530, 0x531` - a switch, not a raiser.

So: one throw, one catch, two log lines.
