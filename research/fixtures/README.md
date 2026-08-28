# `research/fixtures/` — captures kept for what they prove

Copies of `previous-runs/` logs, renamed to say what question they settle. A client run costs
Wisp a manual elevated launch, so these are the most expensive data this project produces.

Two things to know before reading any byte in here.

## 1. The machine identifier is REDACTED. It is not captured data.

The client's `0x0073` session identity and `0x007D` migration hello both carry the machine's
**MAC address** and a machine id derived from it. Those are real hardware identifiers for the
machine that made the capture, so before this repository was published they were replaced
throughout, in the working tree **and in every commit**:

| what | on the wire | in this repo |
|---|---|---|
| MAC | the capturing machine's | `aa bb cc dd ee ff` / `AA-BB-CC-DD-EE-FF` |
| machine id tail | derived from it | `de ad be ef` |

**Every substitution is length-preserving**, so byte offsets, packet lengths and every other
field are exactly as captured. Nothing else was touched.

So: `aabbccddeeff` and `deadbeef` in a body are **placeholders**. Do not decode them, do not
use them as a control, and do not conclude anything from the fact that they are identical
across captures - they are identical because they were substituted, not because the client
sent a constant.

## 2. These are COPIES, so counting across `previous-runs/` and here double-counts

A capture appears once per name it has been given. On 2026-08-28 that produced wrong numbers
in a write-up: 155 world logs on disk were **119 distinct files**, and one capture existed
under three names. Deduplicate by content hash before counting anything.

`tools/extract_attack_bodies.py` does it, and exits non-zero on an empty result.

And a fixture's name says what its author was looking at, **not everything the file contains**.
The two captures that settled where the attack packet carries its skill id had been sitting
here for days under names about Magic Claw damage and a cash-shop click. Grep the contents;
do not scan the names.
