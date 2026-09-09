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


def report_user(pid, user, slide, whence):
    chair_obj = struct.unpack("<Q", read(pid, user + CHAIR_OBJ_AT, 8))[0]
    chair_id = struct.unpack("<i", read(pid, user + CHAIR_ID_AT, 4))[0]
    vt = struct.unpack("<Q", read(pid, user, 8))[0]
    state = "SITTING" if (chair_obj or chair_id != -1) else "not sitting"
    print("  CUser %#x  (from %s, vtable %#x)" % (user, whence, vt))
    print("      +0x3c18 obj = %#x   +0x3c28 id = %d   -> IsSitting says %s"
          % (chair_obj, chair_id, state))


def main():
    pid, base = find_client()
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
    print("CONTROL session : *%#x = %#x, hook log last printed %s   %s"
          % (SESSION_GLOBAL, session,
             hex(expected) if expected else "(none found)",
             "OK" if ok_session else "MISMATCH" if expected else "UNCHECKED"))
    if expected is not None and not ok_session:
        print()
        print("REFUSING TO REPORT: the session pointer disagrees with the hook's own log, so")
        print("the user pointer below it would be read from the wrong object.")
        return 1
    print()

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
        if not found:
            print("No CUser found in the first %#x bytes of the session object." % SCAN_BYTES)
            print("Nothing is reported rather than reporting a stranger's memory.")
            return 1
        print("%d CUser(s) reachable from the session object:" % len(found))
        for off, ptr, v in found:
            report_user(pid, ptr, slide, "session + %#x" % off)
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
