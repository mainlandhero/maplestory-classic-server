"""Map the protection surface of the client's PE binaries.

Reports, per binary: sections (with entropy, to spot packing), TLS callbacks
(anti-debug hooks run before main), delay-loaded imports, and the imports that
matter for networking, process inspection, and anti-debug.

    python tools/pe_surface.py <file-or-dir> [...]
"""

import math
import os
import sys

import pefile

# Import names grouped by what they tell us about the binary's behaviour.
INTEREST = {
    "network": (
        "socket", "WSASocket", "WSAStartup", "connect", "WSAConnect", "send", "recv",
        "WSASend", "WSARecv", "bind", "listen", "accept", "gethostbyname",
        "getaddrinfo", "inet_addr", "inet_pton", "closesocket", "select",
        "WSAAsyncSelect", "WSAEventSelect", "ioctlsocket", "setsockopt",
    ),
    "anti-debug": (
        "IsDebuggerPresent", "CheckRemoteDebuggerPresent", "NtQueryInformationProcess",
        "OutputDebugString", "NtSetInformationThread", "DebugActiveProcess",
        "NtQuerySystemInformation", "GetTickCount", "QueryPerformanceCounter",
        "NtClose", "RtlAddVectoredExceptionHandler", "SetUnhandledExceptionFilter",
    ),
    "process/inject": (
        "CreateRemoteThread", "WriteProcessMemory", "ReadProcessMemory",
        "VirtualProtect", "VirtualAlloc", "VirtualAllocEx", "OpenProcess",
        "CreateToolhelp32Snapshot", "Process32First", "Process32Next",
        "EnumProcesses", "LoadLibrary", "GetProcAddress", "NtMapViewOfSection",
    ),
    "driver/service": (
        "OpenSCManager", "CreateService", "StartService", "OpenService",
        "DeviceIoControl", "CreateFile", "NtLoadDriver", "ControlService",
    ),
    "crypto": (
        "CryptAcquireContext", "CryptEncrypt", "CryptDecrypt", "CryptGenRandom",
        "BCryptEncrypt", "BCryptDecrypt", "BCryptGenRandom", "CryptCreateHash",
        "AES", "EVP_", "RAND_bytes", "SSL_", "MD5", "SHA",
    ),
}


def entropy(data: bytes) -> float:
    if not data:
        return 0.0
    counts = [0] * 256
    for b in data:
        counts[b] += 1
    n = len(data)
    e = 0.0
    for c in counts:
        if c:
            p = c / n
            e -= p * math.log2(p)
    return e


def analyze(path: str) -> None:
    name = os.path.basename(path)
    print("=" * 78)
    print(f"{name}  ({os.path.getsize(path):,} bytes)")
    print("=" * 78)

    try:
        pe = pefile.PE(path, fast_load=True)
        pe.parse_data_directories(
            directories=[
                pefile.DIRECTORY_ENTRY["IMAGE_DIRECTORY_ENTRY_IMPORT"],
                pefile.DIRECTORY_ENTRY["IMAGE_DIRECTORY_ENTRY_EXPORT"],
                pefile.DIRECTORY_ENTRY["IMAGE_DIRECTORY_ENTRY_TLS"],
                pefile.DIRECTORY_ENTRY["IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT"],
            ]
        )
    except Exception as e:  # noqa: BLE001 - report and continue with other files
        print(f"  !! could not parse: {e}\n")
        return

    machine = pe.FILE_HEADER.Machine
    arch = {0x8664: "x64", 0x14C: "x86", 0xAA64: "arm64"}.get(machine, hex(machine))
    print(f"  arch           {arch}")
    print(f"  entrypoint     0x{pe.OPTIONAL_HEADER.AddressOfEntryPoint:X}")
    print(f"  image base     0x{pe.OPTIONAL_HEADER.ImageBase:X}")

    # High entropy in an executable section is the classic packing signature.
    print("\n  sections:")
    print(f"    {'name':<10} {'vaddr':>10} {'vsize':>11} {'rawsize':>11} {'entropy':>8}  flags")
    for s in pe.sections:
        sname = s.Name.rstrip(b"\x00").decode("latin1", "replace")
        data = s.get_data()
        ent = entropy(data[: 1 << 20])  # 1 MB sample keeps this fast
        flags = []
        if s.IMAGE_SCN_MEM_EXECUTE:
            flags.append("X")
        if s.IMAGE_SCN_MEM_WRITE:
            flags.append("W")
        if s.IMAGE_SCN_MEM_READ:
            flags.append("R")
        marker = "  <-- packed?" if ent > 7.2 and s.IMAGE_SCN_MEM_EXECUTE else ""
        print(
            f"    {sname:<10} 0x{s.VirtualAddress:08X} {s.Misc_VirtualSize:>11,} "
            f"{s.SizeOfRawData:>11,} {ent:>8.2f}  {''.join(flags)}{marker}"
        )

    # TLS callbacks run before the entry point - a favourite anti-debug perch.
    tls = getattr(pe, "DIRECTORY_ENTRY_TLS", None)
    if tls and tls.struct.AddressOfCallBacks:
        print(f"\n  TLS callback table @ 0x{tls.struct.AddressOfCallBacks:X}")
        try:
            base = pe.OPTIONAL_HEADER.ImageBase
            rva = tls.struct.AddressOfCallBacks - base
            ptr_size = 8 if arch == "x64" else 4
            for i in range(16):
                raw = pe.get_data(rva + i * ptr_size, ptr_size)
                val = int.from_bytes(raw, "little")
                if not val:
                    break
                print(f"    callback[{i}] -> 0x{val:X}   (runs before main)")
        except Exception as e:  # noqa: BLE001
            print(f"    (could not walk callbacks: {e})")
    else:
        print("\n  TLS callbacks  none")

    # Imports, bucketed by interest.
    found = {k: [] for k in INTEREST}
    dll_count = 0
    total_imports = 0
    for entry in getattr(pe, "DIRECTORY_ENTRY_IMPORT", []):
        dll_count += 1
        dll = entry.dll.decode("latin1", "replace")
        for imp in entry.imports:
            total_imports += 1
            fn = imp.name.decode("latin1", "replace") if imp.name else f"#{imp.ordinal}"
            for cat, needles in INTEREST.items():
                if any(n in fn for n in needles):
                    found[cat].append(f"{dll}!{fn}")
                    break

    print(f"\n  imports        {total_imports} from {dll_count} DLLs")
    for cat, hits in found.items():
        if not hits:
            continue
        uniq = sorted(set(hits))
        print(f"    [{cat}] {len(uniq)}")
        for h in uniq[:14]:
            print(f"      {h}")
        if len(uniq) > 14:
            print(f"      ... +{len(uniq) - 14} more")

    delay = getattr(pe, "DIRECTORY_ENTRY_DELAY_IMPORT", [])
    if delay:
        print(f"\n  delay-loaded   {len(delay)} DLLs")
        for entry in delay[:20]:
            print(f"      {entry.dll.decode('latin1', 'replace')}")

    exports = getattr(pe, "DIRECTORY_ENTRY_EXPORT", None)
    if exports and exports.symbols:
        syms = [s.name.decode("latin1", "replace") for s in exports.symbols if s.name]
        print(f"\n  exports        {len(exports.symbols)}")
        for s in syms[:20]:
            print(f"      {s}")
        if len(syms) > 20:
            print(f"      ... +{len(syms) - 20} more")

    print()
    pe.close()


def main() -> None:
    args = sys.argv[1:]
    if not args:
        print(__doc__)
        raise SystemExit(1)
    targets = []
    for a in args:
        if os.path.isdir(a):
            for f in sorted(os.listdir(a)):
                if f.lower().endswith((".exe", ".dll", ".sys")):
                    targets.append(os.path.join(a, f))
        else:
            targets.append(a)
    for t in targets:
        analyze(t)


if __name__ == "__main__":
    main()
