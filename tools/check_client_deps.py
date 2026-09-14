"""Name the DLL a client folder is missing, without running the client.

Written 2026-09-13, when a second machine showed *"Nexon Launcher failed to load"* and died
before GameGuard init - the window where the hook cannot see anything, because
`install_once` has not run yet. The client's own error text names a subsystem, not a file,
and Procmon needs a launch; this needs neither.

It walks the PE import tables - normal, delay-load and bound - of every module in the
folder, transitively, and reports every imported DLL that cannot be resolved on the search
path the client will actually use:

    the client's own directory, then System32, then PATH

which is `LOAD_WITH_ALTERED_SEARCH_PATH` order for an executable started from its own
folder. **API-set names** (`api-ms-win-*`, `ext-ms-*`) are not files on modern Windows and
are reported separately rather than as misses, because a missing apiset means a missing
*runtime*, which is a different fix from a missing file.

Nothing is executed and nothing is loaded: this reads bytes. That matters here because the
folder contains an anti-cheat module and a repacked executable, and `LoadLibrary`-ing those
from a probe would be measuring a different process than the one that fails.

    python tools/check_client_deps.py client-patched
    python tools/check_client_deps.py "C:\\Users\\Joanne\\Desktop\\MapleCW\\client"

Exit status is 1 when something is missing, so it can gate a launcher step later.

**What a clean result does NOT prove.** A DLL can be present and still fail to load - wrong
architecture, a failed signature check, an antivirus hold, or a `DllMain` that returns
false. This finds the one failure mode that is invisible from inside the process, and says
so when it finds nothing.
"""

import os
import struct
import sys

# Names Windows resolves from the loader's own tables rather than from disk. Absence of a
# file by this name is normal and says nothing.
APISET_PREFIXES = ("api-ms-win-", "ext-ms-")


class NotPe(Exception):
    pass


def _sections(data, pe, optoff, optsize):
    n = struct.unpack_from("<H", data, pe + 6)[0]
    out = []
    for i in range(n):
        o = optoff + optsize + i * 40
        if o + 40 > len(data):
            break
        va = struct.unpack_from("<I", data, o + 12)[0]
        vsz = struct.unpack_from("<I", data, o + 8)[0]
        rsz = struct.unpack_from("<I", data, o + 16)[0]
        raw = struct.unpack_from("<I", data, o + 20)[0]
        out.append((va, max(vsz, rsz), raw, rsz))
    return out


def _reader(path):
    """Return (rva -> file offset) and the raw bytes, or raise NotPe."""
    data = open(path, "rb").read()
    if data[:2] != b"MZ":
        raise NotPe("not MZ")
    pe = struct.unpack_from("<I", data, 0x3C)[0]
    if data[pe : pe + 4] != b"PE\0\0":
        raise NotPe("not PE")
    optsize = struct.unpack_from("<H", data, pe + 20)[0]
    optoff = pe + 24
    magic = struct.unpack_from("<H", data, optoff)[0]
    if magic != 0x20B:
        raise NotPe("not PE32+")
    secs = _sections(data, pe, optoff, optsize)

    def off(rva):
        for va, vsz, raw, rsz in secs:
            if va <= rva < va + vsz:
                delta = rva - va
                if delta >= rsz:  # in the virtual tail, not in the file
                    return None
                return raw + delta
        return None

    ndirs = struct.unpack_from("<I", data, optoff + 108)[0]
    dirs = []
    for i in range(ndirs):
        o = optoff + 112 + i * 8
        if o + 8 > len(data):
            break
        dirs.append(struct.unpack_from("<II", data, o))
    return data, off, dirs


def _cstr(data, at):
    if at is None or at >= len(data):
        return None
    end = data.find(b"\0", at)
    if end < 0:
        return None
    try:
        return data[at:end].decode("ascii")
    except UnicodeDecodeError:
        return None


def imports_of(path):
    """Every DLL name this module imports: normal, delay-load and bound."""
    data, off, dirs = _reader(path)
    names = set()

    # Directory 1: import descriptors, 20 bytes each, dll name at +12.
    if len(dirs) > 1 and dirs[1][0]:
        at = off(dirs[1][0])
        i = 0
        while at is not None:
            base = at + i * 20
            if base + 20 > len(data):
                break
            chunk = data[base : base + 20]
            if chunk == b"\0" * 20:
                break
            rva = struct.unpack_from("<I", chunk, 12)[0]
            n = _cstr(data, off(rva))
            if n:
                names.add(n)
            i += 1

    # Directory 13: delay-load descriptors, 32 bytes each, dll name at +4.
    if len(dirs) > 13 and dirs[13][0]:
        at = off(dirs[13][0])
        i = 0
        while at is not None:
            base = at + i * 32
            if base + 32 > len(data):
                break
            chunk = data[base : base + 32]
            if chunk == b"\0" * 32:
                break
            attrs = struct.unpack_from("<I", chunk, 0)[0]
            rva = struct.unpack_from("<I", chunk, 4)[0]
            # attrs bit 0 clear means the RVAs are really VAs (old linkers).
            n = _cstr(data, off(rva)) if attrs & 1 else None
            if n:
                names.add(n)
            i += 1

    return names


def search_path(client_dir):
    dirs = [client_dir, os.path.join(os.environ.get("SystemRoot", "C:\\Windows"), "System32")]
    dirs += [d for d in os.environ.get("PATH", "").split(os.pathsep) if d]
    seen, out = set(), []
    for d in dirs:
        k = os.path.normcase(os.path.abspath(d)) if d else d
        if k and k not in seen:
            seen.add(k)
            out.append(d)
    return out


def resolve(name, dirs):
    for d in dirs:
        p = os.path.join(d, name)
        if os.path.isfile(p):
            return p
    return None


def in_client_dir(path, client_dir):
    root = os.path.normcase(os.path.abspath(client_dir))
    return os.path.normcase(os.path.abspath(path)).startswith(root + os.sep)


def is_system(path):
    """True for anything under the Windows directory - the OS's to ship, not ours."""
    win = os.path.normcase(os.path.abspath(os.environ.get("SystemRoot", "C:\\Windows")))
    return os.path.normcase(os.path.abspath(path)).startswith(win + os.sep)


def main(argv):
    client_dir = argv[1] if len(argv) > 1 else "client-patched"
    if not os.path.isdir(client_dir):
        print("not a directory: %s" % client_dir)
        return 2
    dirs = search_path(client_dir)

    roots = [
        os.path.join(client_dir, f)
        for f in sorted(os.listdir(client_dir))
        if f.lower().endswith((".exe", ".dll"))
    ]
    if not roots:
        print("no .exe or .dll in %s - is this the client folder?" % client_dir)
        return 2

    pending = list(roots)
    done = set()
    missing = {}   # name -> who imported it
    elsewhere = {}  # name -> (where it was found, who imported it)
    apisets = set()
    unreadable = []

    while pending:
        path = pending.pop()
        key = os.path.normcase(os.path.abspath(path))
        if key in done:
            continue
        done.add(key)
        try:
            deps = imports_of(path)
        except NotPe as e:
            unreadable.append((path, str(e)))
            continue
        except Exception as e:  # a truncated or packed header is data, not a crash
            unreadable.append((path, repr(e)))
            continue
        for dep in sorted(deps):
            low = dep.lower()
            if low.startswith(APISET_PREFIXES):
                apisets.add(low)
                continue
            hit = resolve(dep, dirs)
            if hit is None:
                missing.setdefault(low, set()).add(os.path.basename(path))
            elif in_client_dir(hit, client_dir):
                # Recurse only into the folder's own modules. The first version walked into
                # System32's graph too - 250 modules - and came back with two "missing" OS
                # DLLs (`hvsifiletrust.dll`, `pdmutilities.dll`) on a machine where the
                # client runs perfectly. Those are ordinary optional imports of `SHELL32`
                # and a printing module, resolved by servicing paths this check does not
                # model. An instrument that reports two misses on the known-good control
                # cannot report a miss on the broken one, so the walk stops at the OS
                # boundary: what Windows ships is Windows' problem.
                pending.append(hit)
            elif not is_system(hit) and in_client_dir(path, client_dir):
                # Resolved, but from neither the folder nor Windows - so it came off this
                # machine's PATH. The owner, 2026-09-13: *"our client should still work if
                # there's no C:\\Nexon folder ... We should not have any dependencies on the
                # actual install of MapleStory."* This is the list that says whether that
                # holds, and it is the one thing a check run on the WORKING machine can
                # discover about the broken one: a module borrowed from a real install here
                # is simply absent there, and the folder looks complete on both.
                elsewhere.setdefault(dep.lower(), (hit, set()))[1].add(os.path.basename(path))

    print("client folder : %s" % os.path.abspath(client_dir))
    print("modules read  : %d" % len(done))
    print("search path   : %s" % dirs[0])
    print("                %s" % dirs[1])
    print("                + %d PATH entries" % (len(dirs) - 2))
    print()

    if unreadable:
        print("could not be parsed as PE32+ (not necessarily a problem):")
        for p, why in unreadable:
            print("   %-28s %s" % (os.path.basename(p), why))
        print()

    unresolved_apisets = sorted(a for a in apisets if resolve(a, dirs) is None)
    if unresolved_apisets:
        print("API-set names with no file on disk (%d) - normal on Windows 8+, the loader" % len(unresolved_apisets))
        print("resolves these itself. Listed only so a missing UCRT is visible:")
        for a in unresolved_apisets[:8]:
            print("   %s" % a)
        if len(unresolved_apisets) > 8:
            print("   ... and %d more" % (len(unresolved_apisets) - 8))
        print()

    if elsewhere:
        print("BORROWED FROM THIS MACHINE (%d) - resolved from neither the client folder nor" % len(elsewhere))
        print("Windows, so the folder is NOT self-contained and these will be absent on a PC")
        print("that never installed the game:")
        for name in sorted(elsewhere):
            where, by = elsewhere[name]
            print("   %-24s %s" % (name, where))
            print("   %-24s imported by %s" % ("", ", ".join(sorted(by))))
        print()

    if not missing:
        print("NOTHING MISSING. Every imported DLL resolves on the client's search path.")
        print()
        print("This does not prove the client can start: a DLL that is present can still")
        print("fail to load (wrong architecture, blocked by antivirus, a DllMain that")
        print("returns false, or a delay-load that fails only when first called). It rules")
        print("out the one cause that is invisible from inside the process.")
        return 0

    print("MISSING (%d):" % len(missing))
    for name in sorted(missing):
        print("   %-24s imported by %s" % (name, ", ".join(sorted(missing[name]))))
    return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
