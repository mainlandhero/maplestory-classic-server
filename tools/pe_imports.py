"""List a PE's imports grouped by DLL, resolving ordinal-only imports.

Winsock is routinely imported by ordinal, so a name-based scan reports almost no
networking. This resolves the well-known WS2_32 ordinals so the socket calls show up.

    python tools/pe_imports.py <file> [dll-filter]
"""

import sys

import pefile

# WS2_32.dll exports these by ordinal; the names are stable across Windows versions.
WS2_32_ORDINALS = {
    1: "accept", 2: "bind", 3: "closesocket", 4: "connect", 5: "getpeername",
    6: "getsockname", 7: "getsockopt", 8: "htonl", 9: "htons", 10: "ioctlsocket",
    11: "inet_addr", 12: "inet_ntoa", 13: "listen", 14: "ntohl", 15: "ntohs",
    16: "recv", 17: "recvfrom", 18: "select", 19: "send", 20: "sendto",
    21: "setsockopt", 22: "shutdown", 23: "socket", 24: "GetAddrInfoW",
    51: "gethostbyaddr", 52: "gethostbyname", 53: "getprotobyname",
    54: "getprotobynumber", 55: "getservbyname", 56: "getservbyport",
    57: "gethostname", 101: "WSAAsyncSelect", 102: "WSAAsyncGetHostByAddr",
    103: "WSAAsyncGetHostByName", 111: "WSAGetLastError", 112: "WSASetLastError",
    115: "WSAStartup", 116: "WSACleanup", 151: "__WSAFDIsSet",
    500: "WSAAccept", 501: "WSAAddressToStringA", 502: "WSAAddressToStringW",
    505: "WSACloseEvent", 506: "WSAConnect", 507: "WSACreateEvent",
    508: "WSADuplicateSocketA", 509: "WSADuplicateSocketW", 510: "WSAEnumNameSpaceProvidersA",
    511: "WSAEnumNameSpaceProvidersW", 512: "WSAEnumNetworkEvents",
    513: "WSAEnumProtocolsA", 514: "WSAEnumProtocolsW", 515: "WSAEventSelect",
    516: "WSAGetOverlappedResult", 517: "WSAGetQOSByName", 518: "WSAGetServiceClassInfoA",
    519: "WSAGetServiceClassInfoW", 520: "WSAGetServiceClassNameByClassIdA",
    521: "WSAGetServiceClassNameByClassIdW", 522: "WSAHtonl", 523: "WSAHtons",
    524: "WSAInstallServiceClassA", 525: "WSAInstallServiceClassW", 526: "WSAIoctl",
    527: "WSAJoinLeaf", 528: "WSALookupServiceBeginA", 529: "WSALookupServiceBeginW",
    530: "WSALookupServiceEnd", 531: "WSALookupServiceNextA", 532: "WSALookupServiceNextW",
    533: "WSANSPIoctl", 534: "WSANtohl", 535: "WSANtohs", 536: "WSAProviderConfigChange",
    537: "WSARecv", 538: "WSARecvDisconnect", 539: "WSARecvFrom",
    540: "WSARemoveServiceClass", 541: "WSAResetEvent", 542: "WSASend",
    543: "WSASendDisconnect", 544: "WSASendTo", 545: "WSASetEvent",
    546: "WSASetServiceA", 547: "WSASetServiceW", 548: "WSASocketA",
    549: "WSASocketW", 550: "WSAStringToAddressA", 551: "WSAStringToAddressW",
    552: "WSAUnhookBlockingHook", 553: "WSAWaitForMultipleEvents",
}


def main() -> None:
    if len(sys.argv) < 2:
        print(__doc__)
        raise SystemExit(1)
    path = sys.argv[1]
    dll_filter = sys.argv[2].lower() if len(sys.argv) > 2 else None

    pe = pefile.PE(path, fast_load=True)
    pe.parse_data_directories(
        directories=[
            pefile.DIRECTORY_ENTRY["IMAGE_DIRECTORY_ENTRY_IMPORT"],
            pefile.DIRECTORY_ENTRY["IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT"],
        ]
    )

    for label, attr in (
        ("IMPORTS", "DIRECTORY_ENTRY_IMPORT"),
        ("DELAY IMPORTS", "DIRECTORY_ENTRY_DELAY_IMPORT"),
    ):
        entries = getattr(pe, attr, [])
        if not entries:
            continue
        print(f"===== {label} =====")
        for entry in entries:
            dll = entry.dll.decode("latin1", "replace")
            if dll_filter and dll_filter not in dll.lower():
                continue
            names = []
            for imp in entry.imports:
                if imp.name:
                    names.append(imp.name.decode("latin1", "replace"))
                else:
                    resolved = ""
                    if dll.lower().startswith("ws2_32"):
                        resolved = WS2_32_ORDINALS.get(imp.ordinal, "")
                    names.append(
                        f"#{imp.ordinal}" + (f" ({resolved})" if resolved else "")
                    )
            print(f"\n  {dll}  [{len(names)}]")
            for n in sorted(names):
                print(f"    {n}")
        print()
    pe.close()


if __name__ == "__main__":
    main()
