"""List the DLLs a PE imports. Names only, no third-party module.

`tools/pe_imports.py` is the richer instrument - it resolves ordinal-only imports and
prints every function - but it needs `pefile`, and `pefile` is installed in WISP's
per-user site-packages only. An ELEVATED window does not see it, so on 2026-09-06
`tools/package-server.ps1` died in its own verification step with

    ModuleNotFoundError: No module named 'pefile'

`python -s tools/pe_imports.py` reproduces that exactly, which is what identified it.

That check is the entire point of the packaging script - "the server box needs nothing
installed" is asserted there or it is merely believed - so it must not depend on
anything the account running it might not have. This reads the import directory with
the standard library and nothing else.

    python tools/pe_import_dlls.py <file> [<file> ...]

Prints one name per line, lower-cased and unique, delay imports marked. Exits non-zero
and says why if a file cannot be read as a PE, so a caller cannot mistake a failure for
a clean result.
"""

import struct
import sys

IMPORT_DIR = 1
DELAY_IMPORT_DIR = 13


class NotAPe(Exception):
    pass


class Image:
    """Just enough PE to turn an RVA into a file offset and read the two directories."""

    def __init__(self, blob: bytes) -> None:
        self.blob = blob
        if blob[:2] != b"MZ":
            raise NotAPe("no MZ signature")
        (e_lfanew,) = struct.unpack_from("<I", blob, 0x3C)
        if blob[e_lfanew : e_lfanew + 4] != b"PE\0\0":
            raise NotAPe("no PE signature at e_lfanew")

        coff = e_lfanew + 4
        n_sections, opt_size = struct.unpack_from("<H", blob, coff + 2)[0], struct.unpack_from(
            "<H", blob, coff + 16
        )[0]
        opt = coff + 20
        (magic,) = struct.unpack_from("<H", blob, opt)
        if magic == 0x10B:  # PE32
            self.image_base = struct.unpack_from("<I", blob, opt + 28)[0]
            n_dirs_at, dirs_at = opt + 92, opt + 96
        elif magic == 0x20B:  # PE32+
            self.image_base = struct.unpack_from("<Q", blob, opt + 24)[0]
            n_dirs_at, dirs_at = opt + 108, opt + 112
        else:
            raise NotAPe("optional header magic %#06x is neither PE32 nor PE32+" % magic)

        (n_dirs,) = struct.unpack_from("<I", blob, n_dirs_at)
        self.dirs = []
        for i in range(n_dirs):
            self.dirs.append(struct.unpack_from("<II", blob, dirs_at + i * 8))

        # Section headers follow the optional header, 40 bytes each.
        self.sections = []
        sec = opt + opt_size
        for i in range(n_sections):
            vsize, vaddr, rawsize, rawptr = struct.unpack_from("<IIII", blob, sec + i * 40 + 8)
            self.sections.append((vaddr, max(vsize, rawsize), rawptr))

    def directory(self, index):
        if index >= len(self.dirs):
            return (0, 0)
        return self.dirs[index]

    def offset(self, rva):
        """File offset for an RVA, or None when it lands outside every section."""
        for vaddr, vlen, rawptr in self.sections:
            if vaddr <= rva < vaddr + vlen:
                return rawptr + (rva - vaddr)
        return None

    def string(self, rva):
        off = self.offset(rva)
        if off is None:
            return None
        end = self.blob.find(b"\0", off)
        if end < 0:
            return None
        return self.blob[off:end].decode("latin1", "replace")


def imported_dlls(blob: bytes):
    """[(name, is_delay_import)] in the order the directories list them."""
    img = Image(blob)
    found = []

    rva, size = img.directory(IMPORT_DIR)
    if rva and size:
        off = img.offset(rva)
        if off is None:
            raise NotAPe("import directory RVA %#x is not inside any section" % rva)
        # IMAGE_IMPORT_DESCRIPTOR is 20 bytes; the Name RVA is at +12. An all-zero
        # descriptor terminates the array.
        while off + 20 <= len(blob):
            fields = struct.unpack_from("<IIIII", blob, off)
            if not any(fields):
                break
            name = img.string(fields[3])
            if name:
                found.append((name, False))
            off += 20

    rva, size = img.directory(DELAY_IMPORT_DIR)
    if rva and size:
        off = img.offset(rva)
        if off is None:
            raise NotAPe("delay import directory RVA %#x is not inside any section" % rva)
        # ImgDelayDescr is 32 bytes. Attributes bit 0 says the pointers in it are RVAs;
        # linkers older than VS2005 wrote virtual addresses instead, so subtract the
        # image base when the bit is clear. Reading a VA as an RVA lands in the wrong
        # section and yields a plausible-looking wrong name rather than an error.
        while off + 32 <= len(blob):
            attrs, name_rva = struct.unpack_from("<II", blob, off)
            if not attrs and not name_rva:
                break
            if not (attrs & 1):
                name_rva -= img.image_base
            name = img.string(name_rva)
            if name:
                found.append((name, True))
            off += 32

    return found


def main() -> None:
    if len(sys.argv) < 2:
        print(__doc__)
        raise SystemExit(2)

    status = 0
    for path in sys.argv[1:]:
        try:
            with open(path, "rb") as fh:
                blob = fh.read()
        except OSError as exc:
            print("ERROR %s: %s" % (path, exc), file=sys.stderr)
            status = 1
            continue
        try:
            found = imported_dlls(blob)
        except (NotAPe, struct.error) as exc:
            print("ERROR %s: %s" % (path, exc), file=sys.stderr)
            status = 1
            continue
        if not found:
            # A PE with no imports at all is possible but is not something this tree
            # produces, and reporting it as a clean result would be the wrong answer.
            print("ERROR %s: no imported DLLs found" % path, file=sys.stderr)
            status = 1
            continue

        seen = set()
        for name, delayed in found:
            key = name.lower()
            if key in seen:
                continue
            seen.add(key)
            print(key + ("  (delay)" if delayed else ""))

    raise SystemExit(status)


if __name__ == "__main__":
    main()
