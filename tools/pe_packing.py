"""One-line packing verdict per PE, for surveying a whole client install.

    python tools/pe_packing.py <dir-or-file> [...]
"""

import math
import os
import sys

import pefile

# Section names that identify a specific commercial protector.
PROTECTOR_SECTIONS = {
    ".themida": "Themida/WinLicense",
    ".winlice": "Themida/WinLicense",
    ".vmp": "VMProtect",
    ".vmp0": "VMProtect",
    ".vmp1": "VMProtect",
    "UPX0": "UPX",
    "UPX1": "UPX",
    ".aspack": "ASPack",
    ".adata": "ASPack",
    ".petite": "Petite",
    ".enigma": "Enigma",
    ".boot": "Themida (boot)",
}


def entropy(data: bytes) -> float:
    if not data:
        return 0.0
    counts = [0] * 256
    for b in data:
        counts[b] += 1
    n = len(data)
    return -sum((c / n) * math.log2(c / n) for c in counts if c)


def verdict(path: str) -> str:
    try:
        pe = pefile.PE(path, fast_load=True)
    except Exception as e:  # noqa: BLE001
        return f"unparseable ({type(e).__name__})"

    names, hints, high_entropy, odd_names = [], set(), [], 0
    for s in pe.sections:
        n = s.Name.rstrip(b"\x00").decode("latin1", "replace")
        names.append(n)
        if n in PROTECTOR_SECTIONS:
            hints.add(PROTECTOR_SECTIONS[n])
        ent = entropy(s.get_data()[: 512 << 10])
        if ent > 7.2 and s.IMAGE_SCN_MEM_EXECUTE:
            high_entropy.append(f"{n or '<unnamed>'}:{ent:.2f}")
        # Random-looking lowercase names are a hallmark of custom packers.
        if n and n[0] != "." and n.islower() and len(n) >= 6:
            odd_names += 1

    # A single import per DLL across the board means the IAT is rebuilt at runtime.
    iat_note = ""
    try:
        pe.parse_data_directories(
            directories=[pefile.DIRECTORY_ENTRY["IMAGE_DIRECTORY_ENTRY_IMPORT"]]
        )
        entries = getattr(pe, "DIRECTORY_ENTRY_IMPORT", [])
        if entries and all(len(e.imports) == 1 for e in entries) and len(entries) > 3:
            iat_note = f" IAT-obfuscated({len(entries)} DLLs x1)"
    except Exception:  # noqa: BLE001
        pass
    pe.close()

    parts = []
    if hints:
        parts.append("+".join(sorted(hints)))
    if odd_names:
        parts.append(f"{odd_names} random-named sections")
    if high_entropy:
        parts.append("high-entropy " + ",".join(high_entropy[:3]))
    if iat_note:
        parts.append(iat_note.strip())
    return "; ".join(parts) if parts else "clean"


def main() -> None:
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        raise SystemExit(1)
    targets = []
    for a in args:
        if os.path.isdir(a):
            for root, _, files in os.walk(a):
                for f in sorted(files):
                    if f.lower().endswith((".exe", ".dll", ".sys")):
                        targets.append(os.path.join(root, f))
        else:
            targets.append(a)

    for t in sorted(targets):
        size = os.path.getsize(t)
        print(f"{os.path.basename(t):<32} {size:>12,}  {verdict(t)}")


if __name__ == "__main__":
    main()
