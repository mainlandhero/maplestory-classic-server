"""Read the anti-cheat module's gates out of the RUNNING client. Read-only.

`research/the-180-second-family-is-anti-cheat-2026-09-08.md` established that eighteen
180 s/240 s tick functions each sit behind a gate, and that exactly two gates were open in
the crash dumps: `[0x143AC7F3C] >= 2` (the 180 s writer that is actually seen allocating)
and `[0x143AC7F70] >= 1`. That file inferred the gate block was **static configuration**,
because the values were byte-identical across three sessions.

Two things found on 2026-09-08 make that inference unsafe:

* **The on-disk initialiser is zero.** `0x143AC7F3C` lies in the uninitialised tail of
  `.data` (`VirtualSize > SizeOfRawData`), so the file supplies no value at all. Something
  in the running process writes the 2.
* **Nothing in `.text` writes it.** An opcode-agnostic scan for a RIP-relative displacement
  resolving to the gate - not `dataref.py`'s opcode list, which its own docstring warns is
  incomplete - finds exactly one reference, and it is the `cmp` that reads it. The writer is
  in `.themida`, in another module, or reached through a computed pointer.

So "byte-identical across three sessions" cannot separate *configuration* from *a response
to this environment*: all three sessions ran the same stub `grap64.dll`, the same hook and
the same patched client. That is `CLAUDE.md`'s "the thing you are comparing against may
never have been a control", and it is why this file reads the value instead of arguing
about it.

## The controls

A read of a wrong address returns plausible numbers, so two are checked before anything is
reported, and the tool refuses if either fails:

* **the rebase** - eight bytes of `.text` at the record initialiser must equal the bytes in
  `client-patched/MapleStory.exe`. If ASLR moved the image and we did not follow, this fails.
* **the block** - `[0x143AC7F28]` must read `0xCC` and `[0x143AC7F30]` must read
  `0x1FFA28AC`. `FUN_140c93370` writes both as immediates, unconditionally, at startup. They
  are the fingerprint that says we are looking at the right structure rather than at
  whatever else happens to live at that VA.

Nothing is written to the client. `PROCESS_VM_READ` only, exactly as `tools/dump_runtime.py`
has done on this client before.

    python tools/gatescan.py
"""

import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from dump_runtime import find_client, read  # noqa: E402

STATIC_BASE = 0x140000000
EXE = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
                   "client-patched", "MapleStory.exe")

# The record FUN_140c93370 initialises, and the two live gates.
INIT_FN = 0x140C93370
RECORD = [
    (0x143AC7F24, 4, "record THREAD      (GetCurrentThreadId + 0xF010FA1; a live tid in 37/37)"),
    (0x143AC7F28, 4, "record CODE        (0xCC written as an immediate at init)"),
    (0x143AC7F2C, 4, "record STATE       (0x12CD84 idle / 0x1AFF01 detection pending)"),
    (0x143AC7F30, 8, "record VALUE       (0x1FFA28AC written as an immediate at init)"),
    (0x143AC7F38, 4, "600 s ticker LAST"),
]

# (gate VA, threshold, tick fn, clock, whether it was open in the crash dumps)
GATES = [
    (0x143AC7F3C, 2, "FUN_140c93c80", "180 s", True),
    (0x143AC7F70, 1, "FUN_140c93d60", "240 s", True),
    (0x143AC7F80, 2, "FUN_140c942a0", "180 s", False),
    (0x143AC7F90, 2, "FUN_140c94400", "180 s", False),
    (0x143AC7D64, 2, "FUN_140c94800", "180 s", False),
    (0x143AC7D74, 3, "FUN_140c949a0", "180 s", False),
    (0x143AC7D94, 3, "FUN_140c94b70", "180 s", False),
    (0x143AC7DB0, 2, "FUN_140c94d40", "180 s", False),
    (0x143AC7FE8, 2, "FUN_140c94e40", "180 s", False),
    (0x143AC7FF0, 2, "FUN_140c94fa0", "180 s", False),
    (0x143AC7FA4, 2, "FUN_140c94560", "240 s", False),
    (0x143AC7FDC, 3, "FUN_140c946c0", "240 s", False),
]

# The three detector-armed enable flags of the readable writers.
ENABLES = [
    (0x143AC7D00, "FUN_140c93530  inc v[36]", "armed by the Jin64.dll detector"),
    (0x143AC7D10, "FUN_140c936a0  inc v[37]", "armed by the [ROYAL Connector].exe detector"),
    (0x143AC7D20, "FUN_140c93810  dec v[48]", "armed by the Royal.Secure.Runtime.dll detector"),
]


def file_bytes_at(va, n):
    with open(EXE, "rb") as fh:
        data = fh.read()
    e = struct.unpack_from("<I", data, 0x3C)[0]
    nsec = struct.unpack_from("<H", data, e + 6)[0]
    optsz = struct.unpack_from("<H", data, e + 20)[0]
    opt = e + 24
    imgbase = struct.unpack_from("<Q", data, opt + 24)[0]
    for i in range(nsec):
        o = opt + optsz + i * 40
        vsize, rva, rawsz, raw = struct.unpack_from("<IIII", data, o + 8)
        lo = imgbase + rva
        if lo <= va < lo + min(vsize, rawsz):
            off = raw + (va - lo)
            return data[off:off + n]
    return None


def proc_uptime(pid):
    """Seconds since the process started, from GetProcessTimes. None if it cannot be read."""
    import ctypes
    import ctypes.wintypes as w

    k32 = ctypes.WinDLL("kernel32", use_last_error=True)
    PROCESS_QUERY_INFORMATION = 0x0400
    h = k32.OpenProcess(PROCESS_QUERY_INFORMATION, False, pid)
    if not h:
        return None
    try:
        creation, exit_, kernel, user = (w.FILETIME() for _ in range(4))
        ok = k32.GetProcessTimes(h, ctypes.byref(creation), ctypes.byref(exit_),
                                 ctypes.byref(kernel), ctypes.byref(user))
        if not ok:
            return None
        start = (creation.dwHighDateTime << 32) | creation.dwLowDateTime
        now = w.FILETIME()
        ctypes.windll.kernel32.GetSystemTimeAsFileTime(ctypes.byref(now))
        nowv = (now.dwHighDateTime << 32) | now.dwLowDateTime
        return (nowv - start) / 1e7
    finally:
        k32.CloseHandle(h)


def watch():
    """Poll the two live gates until they flip, and print the process age when they do.

    The archive brackets the flip to 38 s .. 194 s of process life (`--dumps`). That is as
    tight as dead sessions can make it, because no dump exists inside the window. This closes
    it from the other side: one number, from a client that is still running.

    Prints only on a CHANGE plus a heartbeat, so the flip is not buried in noise.
    """
    import time

    pid, base = find_client()
    if pid is None:
        print("MapleStory.exe is not running.")
        return 2
    slide = (base - STATIC_BASE) if base else 0
    watched = [(0x143AC7F3C, 2, "FUN_140c93c80 180 s"),
               (0x143AC7F70, 1, "FUN_140c93d60 240 s"),
               (0x143AC7FDC, 3, "FUN_140c946c0 240 s")]

    want = file_bytes_at(INIT_FN, 8)
    if bytes(read(pid, INIT_FN + slide, 8)) != bytes(want):
        print("CONTROL rebase FAILED - refusing to watch an address that is not the gate.")
        return 1
    print("pid %d, controls OK. Watching %d gate(s); Ctrl-C to stop."
          % (pid, len(watched)))
    print("%-9s %-12s %s" % ("uptime", "gate", "value"))

    last = {}
    beat = 0.0
    while True:
        try:
            up = proc_uptime(pid)
            for va, thr, who in watched:
                v = struct.unpack("<I", read(pid, va + slide, 4))[0]
                if va not in last:
                    last[va] = v
                    print("%8.1fs %-12s = %d  (start, need >= %d, %s)"
                          % (up or -1, hex(va), v, thr, who))
                elif v != last[va]:
                    print("%8.1fs %-12s = %d -> %d   ***FLIPPED*** need >= %d, %s  %s"
                          % (up or -1, hex(va), last[va], v, thr, who,
                             "GATE NOW OPEN" if v >= thr else ""))
                    last[va] = v
            if up is not None and up - beat >= 30:
                beat = up
                print("%8.1fs ... no change (%s)"
                      % (up, ", ".join("%s=%d" % (hex(k), x) for k, x in last.items())))
            time.sleep(1.0)
        except KeyboardInterrupt:
            print("stopped.")
            return 0
        except OSError as exc:
            print("process gone or unreadable: %s" % exc)
            return 0


def scan_dumps():
    """The same block, out of every archived minidump, behind the same two controls.

    This is what turned the gate from "static configuration" into a measurement. The archive
    already contained the *before* picture and nobody had looked: the two dumps whose gate
    reads 0 are the two shortest-lived sessions there are.
    """
    import glob

    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    from dumpwalk import MiniDump, parse_misc  # noqa

    want = file_bytes_at(INIT_FN, 8)
    rows, skipped = [], []
    for path in sorted(glob.glob(os.path.join("dumps", "*.dmp"))):
        try:
            d = MiniDump(path)
            text = bytes(d.read(INIT_FN, 8))
            code = d.u32(0x143AC7F28)
            val = d.u64(0x143AC7F30)
        except Exception as exc:
            skipped.append((os.path.basename(path), "unreadable: %s" % exc))
            continue
        if text != bytes(want) or code != 0xCC or val != 0x1FFA28AC:
            skipped.append((os.path.basename(path), "failed a control"))
            continue
        misc = parse_misc(d)
        create = misc.get("create_time") if misc else None
        rows.append((os.path.basename(path),
                     d.u32(0x143AC7F3C), d.u32(0x143AC7F70), d.u32(0x143AC7FDC),
                     (d.timestamp - create) if create else None))

    print("%d dump(s) passed both controls, %d skipped" % (len(rows), len(skipped)))
    for name, why in skipped:
        print("   %-44s %s" % (name, why))
    print()
    rows.sort(key=lambda r: (r[4] if r[4] is not None else -1))
    print("%-44s %6s %6s %6s %9s" % ("dump", "f3c", "f70", "fdc", "uptime"))
    for name, a, b, c, up in rows:
        print("%-44s %6d %6d %6d %8ss" % (name, a, b, c, up if up is not None else "?"))
    print()
    zero = [r for r in rows if r[1] == 0]
    two = [r for r in rows if r[1] == 2]
    if zero and two:
        print("gate 0x143AC7F3C == 0 : %d dump(s), uptimes %s"
              % (len(zero), sorted(r[4] for r in zero)))
        print("gate 0x143AC7F3C == 2 : %d dump(s), uptimes %d .. %d"
              % (len(two), min(r[4] for r in two), max(r[4] for r in two)))
        print()
        print("The gate is written between %ds and %ds of process life and never changes after."
              % (max(r[4] for r in zero), min(r[4] for r in two)))
    else:
        print("Only one value of the gate appears in the archive, so it brackets nothing.")
    return 0


def main():
    if "--dumps" in sys.argv:
        return scan_dumps()
    if "--watch" in sys.argv:
        return watch()
    pid, base = find_client()
    if pid is None:
        print("MapleStory.exe is not running - nothing to read.")
        return 2
    if base is None:
        # CreateToolhelp32Snapshot on an elevated target fails from a normal shell. Guessing
        # slide 0 is safe here ONLY because the rebase control below can reject it: a wrong
        # slide makes the .text comparison fail and nothing is reported.
        print("could not read the module base (run elevated for the exact one) - "
              "assuming slide 0 and letting the control decide")
        base = STATIC_BASE
    slide = base - STATIC_BASE
    print("pid %d, image base %#x, slide %+#x" % (pid, base, slide))
    print()

    def rd(va, n):
        return read(pid, va + slide, n)

    def u32(va):
        return struct.unpack("<I", rd(va, 4))[0]

    # ---- CONTROL 1: the rebase ------------------------------------------------------
    want = file_bytes_at(INIT_FN, 8)
    try:
        got = bytes(rd(INIT_FN, 8))
    except OSError as exc:
        print(exc)
        print()
        print("The client runs elevated, so a normal shell cannot open it even for reading.")
        print("Run this from the SAME elevated window the launcher was started from:")
        print()
        print("    cd C:/MapleCW")
        print("    python tools/gatescan.py")
        return 2
    ok_rebase = want is not None and got == bytes(want)
    print("CONTROL rebase : .text at %#x  file=%s  live=%s  %s"
          % (INIT_FN, want.hex() if want else "?", got.hex(),
             "OK" if ok_rebase else "MISMATCH"))

    # ---- CONTROL 2: the block -------------------------------------------------------
    code = u32(0x143AC7F28)
    value = struct.unpack("<Q", rd(0x143AC7F30, 8))[0]
    ok_block = code == 0xCC and value == 0x1FFA28AC
    print("CONTROL block  : [0x143AC7F28]=%#x (want 0xcc)  [0x143AC7F30]=%#x (want 0x1ffa28ac)  %s"
          % (code, value, "OK" if ok_block else "MISMATCH"))
    print()

    if not (ok_rebase and ok_block):
        print("REFUSING TO REPORT: a control failed, so every number below would be a read of")
        print("some address that is not the one named. Fix the rebase before believing any of it.")
        return 1

    print("the detection record (FUN_140c93370 initialises it, FUN_140c79130 stamps a detection):")
    for va, n, what in RECORD:
        raw = rd(va, n)
        v = struct.unpack("<Q" if n == 8 else "<I", raw)[0]
        print("  %#011x  %-18s %s" % (va, hex(v), what))
    state = u32(0x143AC7F2C)
    if state == 0x12CD84:
        verdict = "IDLE (0x12CD84) - no detection pending"
    elif state == 0x1AFF01:
        verdict = "DETECTION PENDING (0x1AFF01) - something was detected and not yet reported"
    else:
        verdict = "UNRECOGNISED %#x" % state
    print("  -> STATE is %s" % verdict)
    print()

    print("the eighteen tick gates (open = the tick runs its body):")
    open_now = []
    for va, thr, fn, clock, was_open in GATES:
        v = u32(va)
        is_open = v >= thr
        if is_open:
            open_now.append((fn, clock, va, v, thr))
        print("  %#011x  = %-6d  need >= %d  %-14s %-5s  %-6s   (dumps: %s)"
              % (va, v, thr, fn, clock, "OPEN" if is_open else "closed",
                 "open" if was_open else "closed"))
    print()

    print("the three detector-armed enable flags of the readable writers:")
    for va, fn, why in ENABLES:
        v = u32(va)
        print("  %#011x  = %-6d  %-26s %s" % (va, v, fn, why))
    print()

    print("%d gate(s) open." % len(open_now))
    for fn, clock, va, v, thr in open_now:
        print("   %s on the %s clock, because [%#x] = %d >= %d" % (fn, clock, va, v, thr))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
