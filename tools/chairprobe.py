"""Is the client's own CUser actually sitting? Read-only, from a RUNNING client.

The owner, 2026-09-08: *"I cannot get out of the chair."* Their client sends `0x00DA` and stays
seated, so leaving a chair needs something we do not send - and the packet has not been found
(`research/chairs-2026-09-08.md` §6). This is the one measurement that halves the question, and
it only exists while somebody is stuck.

    CUser + 0x3c28 == the chair id   -> the client seated ITSELF and we are blocking the
                                        stand path; the release is a packet we must send
    CUser + 0x3c28 == -1             -> the client never considered itself seated at all;
                                        the sprite is cosmetic and this is a different bug

Those two need opposite work and look identical on screen.

## How the CUser is reached

`FUN_142cd3a60` (the stand-up builder) does `mov rax, qword ptr [rcx + 0x2358]` to get the
local user from the session context, and the session context is the global the hook already
watches - `session: watching DAT_143aa84a0`. So:

    session = *(void**)0x143AA84A0
    user    = *(void**)(session + 0x2358)

## The controls, and one of them is live

Reading a wrong pointer produces plausible numbers, so nothing is reported unless both pass:

* **the rebase** - eight bytes of `.text` at `FUN_142834020` (`IsSitting`) must equal the file.
* **the session pointer** - the hook log prints the very same object every few seconds as
  `***** SESSION obj=0x... *****`. This tool prints what it read; if it does not match the
  latest such line in `client-patched/maplecw-hook.log`, the chain is wrong. That is a control
  taken from a *different instrument on the same running process*, which is much stronger than
  an internal consistency check.

`IsSitting` is `(m_0x3c18 && ...) || m_0x3c28 != -1`, so both fields are read and reported.

Read-only: `PROCESS_VM_READ`, exactly as `tools/gatescan.py`. Run from an ELEVATED shell.
"""

import os
import re
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from dump_runtime import find_client, read  # noqa: E402
from gatescan import file_bytes_at  # noqa: E402

STATIC_BASE = 0x140000000
IS_SITTING_FN = 0x142834020
SESSION_GLOBAL = 0x143AA84A0
USER_AT = 0x2358

# The LOCAL CUser, read straight out of a global instead of guessed at.
#
# `FUN_14187f150` - the chair UI logic - does `mov rcx,[rip+0x22292d6] ; mov rax,[rcx] ;
# call [rax+0x120]`, i.e. it loads this global, takes the object's vtable and calls slot 36,
# which is `IsSitting`. So the object at this address is a CUser, from the client's own code
# [L]. The neighbouring `0x143AA84A0` is the session object the hook already watches.
#
# Two guessed offsets produced two wrong conclusions before this was found; the vtable control
# below still applies, so a wrong anchor is caught rather than reported.
LOCAL_USER_GLOBAL = 0x143AA8518
CHAIR_OBJ_AT = 0x3C18
CHAIR_ID_AT = 0x3C28

# The five CUser-family vtables, from `tools/callers.py 0x142834020` - every one carries
# IsSitting at slot 36. An object whose first qword is not one of these is NOT a CUser, and
# reading +0x3c28 out of it is reading a stranger's memory.
#
# **This control is the whole reason for this edit.** The first version validated the SESSION
# pointer against the hook log and then walked `+0x2358` to a "CUser" WITHOUT checking it,
# reporting 0. A watch on SetChair later showed the client being handed -1 on a real CUser
# whose vtable was 0x1434831e0 - so the 0 was very likely read from the wrong object. Verifying
# one link of a chain and trusting the rest is exactly what CLAUDE.md warns about.
CUSER_VTABLES = [0x14337F188, 0x143413F98, 0x143481768, 0x1434831E0, 0x143486B70]
SCAN_BYTES = 0x4000
HOOK_LOG = os.path.join("client-patched", "maplecw-hook.log")


def client_pids():
    """Every running MapleStory.exe, not just the first one `tasklist` happens to list.

    **Two clients broke this tool once.** `dump_runtime.find_client` returns `pids[0]`, and
    the hook log is shared by every client on the machine - so with Cobalt and Tester2 both
    up, the session pointer read from one process was compared against a `SESSION obj=` line
    written by the other, and the control refused a perfectly good read. The control was
    right to fire; the comparison was ambiguous.
    """
    import subprocess

    out = subprocess.run(
        ["tasklist", "/FI", "IMAGENAME eq MapleStory.exe", "/FO", "CSV", "/NH"],
        capture_output=True, text=True,
    ).stdout
    pids = []
    for line in out.splitlines():
        parts = [p.strip('"') for p in line.split('","')]
        if len(parts) > 1 and parts[0].lower().startswith("maplestory"):
            try:
                pids.append(int(parts[1]))
            except ValueError:
                pass
    return pids


def logged_session_obj():
    """The object the hook last printed, as an independent check on our pointer walk."""
    try:
        with open(HOOK_LOG, "rb") as fh:
            fh.seek(max(0, os.path.getsize(HOOK_LOG) - 400_000))
            tail = fh.read().decode("utf-8", "replace")
    except OSError:
        return None
    hits = re.findall(r"SESSION obj=0x([0-9a-fA-F]+)", tail)
    return int(hits[-1], 16) if hits else None


def scan_process_for_users(pid, slide):
    """Every CUser in the process, found by its vtable rather than by any offset.

    The session object turned out not to hold a CUser pointer in its first 0x4000 bytes, and
    two guessed offsets have already produced wrong answers on this bug. A CUser is
    self-identifying: its first qword is one of five known vtables. So this walks the committed
    private read-write regions with VirtualQueryEx and looks for those eight bytes, 8-aligned.

    Slow (hundreds of MB) but it cannot be wrong about what it finds: the vtable IS the type.
    """
    import ctypes
    import ctypes.wintypes as w

    k32 = ctypes.WinDLL("kernel32", use_last_error=True)
    PROCESS_VM_READ = 0x0010
    PROCESS_QUERY_INFORMATION = 0x0400
    MEM_COMMIT = 0x1000
    WRITABLE = 0x04 | 0x40 | 0x02 | 0x20        # RW, RWX, RO, RX - objects live in RW

    class MBI(ctypes.Structure):
        _fields_ = [("BaseAddress", ctypes.c_void_p), ("AllocationBase", ctypes.c_void_p),
                    ("AllocationProtect", w.DWORD), ("__a", w.DWORD),
                    ("RegionSize", ctypes.c_size_t), ("State", w.DWORD),
                    ("Protect", w.DWORD), ("Type", w.DWORD), ("__b", w.DWORD)]

    h = k32.OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, False, pid)
    if not h:
        return []
    wanted = {(v + slide).to_bytes(8, "little") for v in CUSER_VTABLES}
    hits = []
    addr = 0x10000
    mbi = MBI()
    read_n = ctypes.c_size_t(0)
    try:
        while addr < 0x7FFF_FFFF_0000:
            if not k32.VirtualQueryEx(h, ctypes.c_void_p(addr), ctypes.byref(mbi),
                                      ctypes.sizeof(mbi)):
                break
            base = mbi.BaseAddress or 0
            size = mbi.RegionSize or 0
            if size == 0:
                break
            if mbi.State == MEM_COMMIT and (mbi.Protect & WRITABLE) and size <= 64 * 1024 * 1024:
                buf = ctypes.create_string_buffer(size)
                if k32.ReadProcessMemory(h, ctypes.c_void_p(base), buf,
                                         size, ctypes.byref(read_n)):
                    blob = buf.raw[:read_n.value]
                    for off in range(0, len(blob) - 8, 8):
                        if blob[off:off + 8] in wanted:
                            hits.append(base + off)
            addr = base + size
    finally:
        k32.CloseHandle(h)
    return hits


def find_users(pid, session, slide):
    """Every pointer in the session object that points at a real CUser.

    No guessed offsets: each 8-aligned qword is dereferenced and kept only if its target's
    first qword is one of `CUSER_VTABLES`. The control IS the search, so a miss returns
    nothing rather than something plausible.
    """
    try:
        blob = bytes(read(pid, session, SCAN_BYTES))
    except OSError:
        return []
    wanted = {v + slide for v in CUSER_VTABLES}
    out = []
    for off in range(0, len(blob) - 8, 8):
        ptr = struct.unpack_from("<Q", blob, off)[0]
        if ptr < 0x10000 or ptr > 0x7FFF_FFFF_FFFF or ptr & 7:
            continue
        try:
            vt = struct.unpack("<Q", read(pid, ptr, 8))[0]
        except OSError:
            continue
        if vt in wanted:
            out.append((off, ptr, vt))
    return out


def chair_object_flag(pid, obj):
    """Evaluate `FUN_141716a90(obj)` - the second half of IsSitting - rather than guess.

    It is a ZtlSecure accessor, not a state test [L]:

        edx = byte[obj+0x40]                 ; key
        eax = byte[obj+0x41]                 ; key ^ value
        bl  = al ^ dl                        ; THE VALUE
        edx ^= 0xbaadf00d ; ror edx,5 ; add edx,eax
        cmp edx, dword[obj+0x44]             ; the stored checksum

    Returns `(value, checksum_ok)`. A non-null pointer alone says nothing: the first version of
    this tool printed "SITTING" for any non-null object and that was an overclaim.
    """
    raw = bytes(read(pid, obj + 0x40, 8))
    key = raw[0]
    keyed = raw[1]
    stored = struct.unpack_from("<I", raw, 4)[0]
    value = keyed ^ key
    e = (key ^ 0xBAADF00D) & 0xFFFFFFFF
    e = ((e >> 5) | (e << 27)) & 0xFFFFFFFF
    e = (e + keyed) & 0xFFFFFFFF
    return value, e == stored


def report_user(pid, user, slide, whence):
    chair_obj = struct.unpack("<Q", read(pid, user + CHAIR_OBJ_AT, 8))[0]
    chair_id = struct.unpack("<i", read(pid, user + CHAIR_ID_AT, 4))[0]
    vt = struct.unpack("<Q", read(pid, user, 8))[0]
    print("  CUser %#x  (from %s, vtable %#x)" % (user, whence, vt))
    print("      +0x3c28 chair id  = %d %s" % (chair_id, "(absent - correct)" if chair_id == -1 else ""))
    print("      +0x3c18 chair obj = %#x" % chair_obj)

    obj_half = False
    if chair_obj:
        try:
            value, ok = chair_object_flag(pid, chair_obj)
        except OSError:
            print("      the object is unreadable - the object half cannot be evaluated")
            value, ok = None, False
        if value is not None:
            print("      FUN_141716a90(obj) = %d, checksum %s"
                  % (value, "VALID" if ok else "INVALID - this is not that structure"))
            obj_half = bool(value) and ok
        # The object IS the seated state, so name its class. Its vtable identifies it, and
        # the vtable's slots are where a "clear this" method would live - which is what the
        # release has to reach. The id field at +0x3c28 is not involved at all.
        try:
            ovt = struct.unpack("<Q", read(pid, chair_obj, 8))[0]
            print("      chair object vtable = %#x  (static %#x)" % (ovt, ovt - slide))
            head = bytes(read(pid, chair_obj, 0x50))
            print("      object head: %s" % head[:0x30].hex())
        except OSError:
            print("      chair object header unreadable")

    sitting = obj_half or chair_id != -1
    print("      -> IsSitting = (obj && flag) || id != -1 = %s" % ("TRUE" if sitting else "FALSE"))
    if sitting and chair_id == -1:
        print("         ...and it is TRUE because of the OBJECT half alone. The id field is")
        print("         correct; whatever holds the player in the chair is that object.")


def main():
    # With two clients up, `find_client` returns whichever `tasklist` lists first - which may
    # be the other player's. Say which pids exist and let one be named.
    pids = client_pids()
    if len(pids) > 1:
        print("%d clients running: %s" % (len(pids), ", ".join(str(p) for p in pids)))
        print("Reading the first unless --pid is given. Each is one character; the seat "
              "fields below are that client's own.")
        print()
    want = None
    for i, a in enumerate(sys.argv):
        if a == "--pid" and i + 1 < len(sys.argv):
            want = int(sys.argv[i + 1])
    pid, base = find_client()
    if want is not None:
        if want not in pids:
            print("no MapleStory.exe with pid %d - running: %s"
                  % (want, ", ".join(str(p) for p in pids)))
            return 2
        pid, base = want, None
    if pid is None:
        print("MapleStory.exe is not running - and this measurement only exists while it is.")
        return 2
    slide = (base - STATIC_BASE) if base else 0

    try:
        got = bytes(read(pid, IS_SITTING_FN + slide, 8))
    except OSError as exc:
        print(exc)
        print()
        print("Run from the SAME elevated window the launcher was started from:")
        print('    cd "C:/MapleCW"; python tools/chairprobe.py')
        return 2

    want = file_bytes_at(IS_SITTING_FN, 8)
    if want is None or got != bytes(want):
        print("CONTROL rebase FAILED: .text at %#x is %s, file says %s"
              % (IS_SITTING_FN, got.hex(), want.hex() if want else "?"))
        return 1
    print("pid %d, slide %+#x" % (pid, slide))
    print("CONTROL rebase  : .text at IsSitting matches the file            OK")

    session = struct.unpack("<Q", read(pid, SESSION_GLOBAL + slide, 8))[0]
    expected = logged_session_obj()
    ok_session = expected is not None and session == expected
    many = len(client_pids()) > 1
    print("CONTROL session : *%#x = %#x, hook log last printed %s   %s"
          % (SESSION_GLOBAL, session,
             hex(expected) if expected else "(none found)",
             "OK" if ok_session else "ADVISORY" if many else "MISMATCH" if expected else "UNCHECKED"))
    if expected is not None and not ok_session:
        if many:
            # The hook log is shared by every client, so its last line may belong to the OTHER
            # one. Not a reason to refuse - the vtable control below is the hard gate and it
            # checks the object itself rather than a log written by a sibling process.
            print("   (%d clients are running and they share one hook log, so its last line"
                  % len(client_pids()))
            print("    may be the other client's. Downgraded to advisory; the vtable control")
            print("    below is the one that cannot be fooled by a sibling process.)")
        else:
            print()
            print("REFUSING TO REPORT: the session pointer disagrees with the hook's own log,")
            print("and only one client is running, so that is a real mismatch.")
            return 1
    print()

    # Preferred: the global the client's own chair code dereferences.
    user = struct.unpack("<Q", read(pid, LOCAL_USER_GLOBAL + slide, 8))[0]
    if user:
        try:
            vt0 = struct.unpack("<Q", read(pid, user, 8))[0]
        except OSError:
            vt0 = 0
        if (vt0 - slide) in CUSER_VTABLES:
            print("local CUser = *%#x = %#x   (vtable OK)" % (LOCAL_USER_GLOBAL, user))
            print()
            report_user(pid, user, slide, "the client's own local-user global")
            return 0
        print("*%#x = %#x but its vtable %#x is not a CUser - falling through"
              % (LOCAL_USER_GLOBAL, user, vt0))

    user = struct.unpack("<Q", read(pid, session + USER_AT, 8))[0]
    if not user:
        print("the session holds a NULL local user at +%#x - nothing to read." % USER_AT)
        return 1
    print("local CUser = *(session + %#x) = %#x" % (USER_AT, user))
    vt = struct.unpack("<Q", read(pid, user, 8))[0]
    vt_static = vt - slide
    ok_vt = vt_static in CUSER_VTABLES
    print("CONTROL type    : vtable %#x %s"
          % (vt, "is a CUser vtable   OK" if ok_vt
             else "is NOT any of the five CUser vtables   FAILED"))
    if not ok_vt:
        print("   (%#x is not a pointer at all - session+%#x is some other field)"
              % (vt, USER_AT))
        print()
        print("Falling back to SEARCHING the session object for a CUser, which needs no guessed")
        print("offset: every candidate is accepted only if its vtable is one of the five, so")
        print("this cannot report a wrong object the way a fixed offset silently did.")
        print()
        found = find_users(pid, session, slide)
        if found:
            print("%d CUser(s) reachable from the session object:" % len(found))
            for off, ptr, v in found:
                report_user(pid, ptr, slide, "session + %#x" % off)
            return 0

        print("No CUser pointer in the first %#x bytes of the session object." % SCAN_BYTES)
        print("Scanning the whole process for CUser vtables instead - this takes a moment.")
        print()
        objs = scan_process_for_users(pid, slide)
        if not objs:
            print("No CUser anywhere in committed memory. Nothing is reported.")
            return 1
        print("%d CUser object(s) found by vtable:" % len(objs))
        for o in objs:
            try:
                report_user(pid, o, slide, "memory scan")
            except OSError:
                print("  CUser %#x  (unreadable)" % o)
        return 0
    print()

    chair_obj = struct.unpack("<Q", read(pid, user + CHAIR_OBJ_AT, 8))[0]
    chair_id = struct.unpack("<i", read(pid, user + CHAIR_ID_AT, 4))[0]
    print("  CUser + %#x  (chair OBJECT ptr) = %#x" % (CHAIR_OBJ_AT, chair_obj))
    print("  CUser + %#x  (chair ID)         = %d (%#x)"
          % (CHAIR_ID_AT, chair_id, chair_id & 0xFFFFFFFF))
    print()

    # 0 is a THIRD case and the first version of this tool got it wrong: it tested only
    # `!= -1` and reported "the client set the field itself", which is exactly the value it
    # could not have set from a chair id. `-1` is the client's own absent form (its initialiser
    # writes that literal); a real Set Up chair would read 3010005. Zero is neither.
    if chair_id == 0:
        print("VERDICT: the field is ZERO - neither the client's absent form (-1) nor a chair id.")
        print("  IsSitting is `(obj && ...) || m_0x3c28 != -1`, so ZERO MAKES IT PERMANENTLY TRUE.")
        print("  The client believes it is seated whatever it is doing, and standing cannot")
        print("  clear a state that was never legitimately entered.")
        print()
        print("  THE NEXT MEASUREMENT IS CHEAP AND DECISIVE: relog, and run this again BEFORE")
        print("  sitting on anything.")
        print("    still 0 -> the field is 0 from login, the chair is a red herring, and")
        print("               something we send sets it. research/user-enter-field.md maps")
        print("               body offset 416 to this field, and userpool.rs sends i16(-1)")
        print("               there - so the local user is being built by some other path.")
        print("    -1      -> login is clean and SITTING is what wrote 0, i.e. the client")
        print("               called SetChair(0) because it had no id to use.")
        return 0
    if chair_id == -1 and not chair_obj:
        print("VERDICT: the client does NOT consider itself seated.")
        print("  Both halves of IsSitting are absent, so the sprite on screen is cosmetic and")
        print("  'cannot get out of the chair' is NOT a missing release packet. The question")
        print("  changes: find what draws the seated avatar without setting this state.")
    elif chair_id != -1:
        print("VERDICT: the client IS seated, and it set the field ITSELF - we never sent it.")
        print("  chair id %d. So a release exists and is a packet we do not send; the target is" % chair_id)
        print("  whatever calls SetChair (FUN_1428341a0, vtable slot 38) with -1.")
    else:
        print("VERDICT: seated via the chair OBJECT (+%#x), not the id field." % CHAIR_OBJ_AT)
        print("  That is the portable/miniroom path. The id field is untouched, so the release")
        print("  clears the object rather than the id - a different function to hunt.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
