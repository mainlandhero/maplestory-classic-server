"""Dump the client's key-binding tables out of the RUNNING client. Read-only.

`research/keyboard-layout-2026-09-08.md` decoded both directions of the protocol and left one
thing the server cannot invent: **the client's factory key layout**.

The save packet `0x0199` subtype 0 is a **delta**. It encodes only the slots where the live
table differs from a shadow copy 0x1bd bytes later, and `0x05F1` refreshes that shadow from
whatever the server sent. So a server that wants to restore a layout has to hold all 89 slots -
and for a character that has never been synced, the shadow the client diffed against is its own
factory table, which is nowhere in our code.

Guessing it is not an option: a wrong table sent with gate byte 0 rebinds or blanks every key
on the player's keyboard. So it is measured.

`FUN_1401de850` is `lea rax, [rip + 0x3095c09] ; ret` - the manager is a **static object**, not
a heap allocation, so both tables sit at fixed addresses [L]:

    live table    0x143274460          89 x { u8 type; u32 action }
    shadow table  0x143274460 + 0x1bd  the same, and it is FACTORY until a 0x05F1 arrives

We have never sent `0x05F1`, so on any client the shadow is the factory layout regardless of
what the player has rebound locally.

## The controls

* **the rebase** - eight bytes of `.text` at `FUN_1401de850` must match the file on disk.
* **the shape** - a factory table is neither empty nor full. Between 10 and 80 slots must be
  bound, or we are reading something that is not this table.
* **known bindings** - Q, W, E and I are bound to menus in the client's own default layout and
  are visible in the KEY BINDINGS dialog. All four must be non-empty.

Nothing is written to the client. `PROCESS_VM_READ` only.

    python tools/keymapdump.py            # print both tables and the controls
    python tools/keymapdump.py --rust     # emit the Rust table for net::keymap

Run it from an ELEVATED shell; the client runs elevated and OpenProcess is refused otherwise.
"""

import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from dump_runtime import find_client, read  # noqa: E402
from gatescan import file_bytes_at  # noqa: E402

STATIC_BASE = 0x140000000

SINGLETON_FN = 0x1401DE850
MANAGER = 0x143274460
SHADOW_OFF = 0x1BD
SLOTS = 89
SLOT_LEN = 5

# DirectInput scan codes for keys the client's own default layout binds to menus.
KNOWN_BOUND = {0x10: "Q", 0x11: "W", 0x12: "E", 0x17: "I"}

# A few scan codes worth naming in the printout. Not exhaustive and not needed by the tool.
NAMES = {
    0x01: "Esc", 0x02: "1", 0x03: "2", 0x04: "3", 0x05: "4", 0x06: "5", 0x07: "6",
    0x08: "7", 0x09: "8", 0x0A: "9", 0x0B: "0", 0x0F: "Tab",
    0x10: "Q", 0x11: "W", 0x12: "E", 0x13: "R", 0x14: "T", 0x15: "Y", 0x16: "U",
    0x17: "I", 0x18: "O", 0x19: "P", 0x1A: "[", 0x1B: "]", 0x1C: "Enter",
    0x1D: "LCtrl", 0x1E: "A", 0x1F: "S", 0x20: "D", 0x21: "F", 0x22: "G", 0x23: "H",
    0x24: "J", 0x25: "K", 0x26: "L", 0x27: ";", 0x28: "'",
    0x2A: "LShift", 0x2C: "Z", 0x2D: "X", 0x2E: "C", 0x2F: "V", 0x30: "B", 0x31: "N",
    0x32: "M", 0x33: ",", 0x34: ".", 0x35: "/", 0x36: "RShift", 0x38: "LAlt",
    0x39: "Space", 0x3B: "F1", 0x3C: "F2", 0x3D: "F3", 0x3E: "F4", 0x3F: "F5",
    0x40: "F6", 0x41: "F7", 0x42: "F8", 0x43: "F9", 0x44: "F10",
    0x9D: "RCtrl", 0xB8: "RAlt",
}


def read_table(pid, slide, base):
    raw = read(pid, base + slide, SLOTS * SLOT_LEN)
    out = []
    for i in range(SLOTS):
        kind = raw[i * SLOT_LEN]
        if isinstance(kind, str):
            kind = ord(kind)
        action = struct.unpack_from("<I", raw, i * SLOT_LEN + 1)[0]
        out.append((kind, action))
    return out


def main():
    as_rust = "--rust" in sys.argv
    pid, base = find_client()
    if pid is None:
        print("MapleStory.exe is not running - nothing to read.")
        return 2
    slide = (base - STATIC_BASE) if base else 0

    try:
        got = bytes(read(pid, SINGLETON_FN + slide, 8))
    except OSError as exc:
        print(exc)
        print()
        print("Run this from the SAME elevated window the launcher was started from:")
        print('    cd "C:/MapleCW"; python tools/keymapdump.py')
        return 2

    want = file_bytes_at(SINGLETON_FN, 8)
    if want is None or got != bytes(want):
        print("CONTROL rebase FAILED: .text at %#x is %s, file says %s"
              % (SINGLETON_FN, got.hex(), want.hex() if want else "?"))
        print("REFUSING TO REPORT - every slot below would come from the wrong address.")
        return 1

    live = read_table(pid, slide, MANAGER)
    shadow = read_table(pid, slide, MANAGER + SHADOW_OFF)

    bound = sum(1 for k, a in shadow if k or a)
    ok_shape = 10 <= bound <= 80
    missing = [n for code, n in KNOWN_BOUND.items() if not (shadow[code][0] or shadow[code][1])]
    ok_known = not missing

    print("pid %d, slide %+#x" % (pid, slide))
    print("CONTROL rebase : .text at %#x matches the file            OK" % SINGLETON_FN)
    print("CONTROL shape  : %d of %d shadow slots bound (want 10..80)  %s"
          % (bound, SLOTS, "OK" if ok_shape else "FAILED"))
    print("CONTROL known  : Q W E I all bound in the shadow            %s"
          % ("OK" if ok_known else "FAILED, unbound: " + " ".join(missing)))
    print()
    if not (ok_shape and ok_known):
        print("REFUSING TO EMIT A TABLE: a control failed, so this is not the factory layout")
        print("and shipping it would rebind or blank every key on a player's keyboard.")
        return 1

    if as_rust:
        print("// Measured from a running client by tools/keymapdump.py. The client's FACTORY")
        print("// layout, read from the shadow table at 0x143274460 + 0x1bd, which stays")
        print("// factory until a 0x05F1 arrives - and we have never sent one.")
        print("pub const CLIENT_DEFAULT_LAYOUT: [Slot; %d] = [" % SLOTS)
        for i, (kind, action) in enumerate(shadow):
            name = NAMES.get(i)
            note = ("  // %s" % name) if name and (kind or action) else ""
            print("    Slot { kind: %d, action: %d },%s" % (kind, action, note))
        print("];")
        return 0

    print("%-5s %-8s %-18s %s" % ("code", "key", "shadow (factory)", "live (this session)"))
    for i in range(SLOTS):
        lk, la = live[i]
        sk, sa = shadow[i]
        if not (lk or la or sk or sa):
            continue
        mark = "" if (lk, la) == (sk, sa) else "   <- CHANGED LOCALLY"
        print("%#04x  %-8s type=%d action=%-9d  type=%d action=%-9d%s"
              % (i, NAMES.get(i, ""), sk, sa, lk, la, mark))
    print()
    print("%d slot(s) bound in the factory table." % bound)
    changed = sum(1 for i in range(SLOTS) if live[i] != shadow[i])
    print("%d slot(s) differ from it in this session - that is exactly what a 0x0199 would carry."
          % changed)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
