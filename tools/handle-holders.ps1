<#
.SYNOPSIS
    Which processes hold an open handle to a given process, and may they terminate it.

.DESCRIPTION
    If the client is being killed from outside, the killer needs a handle to it carrying
    PROCESS_TERMINATE (0x0001). Nothing in the client can see that call - it executes in
    the other process - so the only way to name a suspect from this side is to enumerate
    every handle on the system and ask which ones point at the client.

    That is what this does: NtQuerySystemInformation(SystemExtendedHandleInformation)
    lists every handle in every process; each one of Process type is duplicated into this
    process and GetProcessId is asked what it refers to. Read-only throughout - it opens
    for duplicate/query, duplicates with no access rights requested, and closes.

    The process type index is not hardcoded. It is discovered by opening a handle to
    ourselves and finding our own entry in the table, because the index moves between
    Windows builds and a hardcoded one silently matches the wrong type - which would
    produce a confident empty list.

    Elevation matters: without it, handles held by services running as SYSTEM cannot be
    duplicated and will be missed. The result says how many were unreachable so an empty
    list is never mistaken for "nobody holds one".

.EXAMPLE
    powershell -ExecutionPolicy Bypass -File tools\handle-holders.ps1 -TargetPid 1234

.NOTES
    The script parameter is -TargetPid, not -ClientPid, deliberately: this is meant to be
    dot-sourced by exit-forensics.ps1, dot-sourcing runs a script in the caller's scope,
    and a shared parameter name would declare it there and reset the pid being watched.

.EXAMPLE
    Dot-source it to reuse the compiled type across repeated scans:

    . .\tools\handle-holders.ps1
    Get-HandleHolder -TargetPid 1234
#>
[CmdletBinding()]
param(
    [int]$TargetPid = 0
)

if (-not ('MapleCW.Handles' -as [type])) {
    Add-Type -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace MapleCW
{
    public class Holder
    {
        public int Pid;
        public uint Access;
        public bool CanTerminate;
    }

    public static class Handles
    {
        [StructLayout(LayoutKind.Sequential)]
        struct HandleEntry
        {
            public IntPtr Object;
            public IntPtr OwnerPid;
            public IntPtr Handle;
            public uint GrantedAccess;
            public ushort CreatorBackTraceIndex;
            public ushort ObjectTypeIndex;
            public uint HandleAttributes;
            public uint Reserved;
        }

        const int SystemExtendedHandleInformation = 64;
        const uint STATUS_INFO_LENGTH_MISMATCH = 0xC0000004;
        const uint PROCESS_TERMINATE = 0x0001;
        const uint PROCESS_DUP_HANDLE = 0x0040;
        const uint PROCESS_QUERY_LIMITED_INFORMATION = 0x1000;
        const uint DUPLICATE_SAME_ACCESS = 0x0002;

        [DllImport("ntdll.dll")]
        static extern uint NtQuerySystemInformation(int cls, IntPtr info, int len, out int needed);
        [DllImport("kernel32.dll", SetLastError = true)]
        static extern IntPtr OpenProcess(uint access, bool inherit, int pid);
        [DllImport("kernel32.dll", SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        static extern bool DuplicateHandle(IntPtr srcProc, IntPtr srcHandle, IntPtr dstProc,
                                           out IntPtr dstHandle, uint access, bool inherit, uint options);
        [DllImport("kernel32.dll", SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        static extern bool CloseHandle(IntPtr h);
        [DllImport("kernel32.dll")]
        static extern IntPtr GetCurrentProcess();
        [DllImport("kernel32.dll", SetLastError = true)]
        static extern int GetProcessId(IntPtr h);

        [StructLayout(LayoutKind.Sequential)]
        struct TokenPrivileges { public uint Count; public long Luid; public uint Attributes; }
        [DllImport("advapi32.dll", SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        static extern bool OpenProcessToken(IntPtr proc, uint access, out IntPtr token);
        [DllImport("advapi32.dll", SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        static extern bool LookupPrivilegeValue(string system, string name, out long luid);
        [DllImport("advapi32.dll", SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        static extern bool AdjustTokenPrivileges(IntPtr token, bool disableAll,
                                                 ref TokenPrivileges state, int len, IntPtr prev, IntPtr prevLen);

        /// <summary>How many Process handles could not be duplicated on the last scan.</summary>
        public static int Unreachable;
        /// <summary>How many Process handles the last scan examined.</summary>
        public static int Examined;

        public static bool EnableDebugPrivilege()
        {
            IntPtr token;
            // TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY
            if (!OpenProcessToken(GetCurrentProcess(), 0x0020 | 0x0008, out token)) return false;
            try
            {
                long luid;
                if (!LookupPrivilegeValue(null, "SeDebugPrivilege", out luid)) return false;
                TokenPrivileges tp = new TokenPrivileges();
                tp.Count = 1;
                tp.Luid = luid;
                tp.Attributes = 0x0002; // SE_PRIVILEGE_ENABLED
                if (!AdjustTokenPrivileges(token, false, ref tp, 0, IntPtr.Zero, IntPtr.Zero)) return false;
                return Marshal.GetLastWin32Error() == 0;
            }
            finally { CloseHandle(token); }
        }

        static IntPtr Snapshot(out int count, out int stride)
        {
            count = 0;
            stride = Marshal.SizeOf(typeof(HandleEntry));
            int len = 1 << 20;
            while (true)
            {
                IntPtr buf = Marshal.AllocHGlobal(len);
                int needed;
                uint status = NtQuerySystemInformation(SystemExtendedHandleInformation, buf, len, out needed);
                if (status == STATUS_INFO_LENGTH_MISMATCH)
                {
                    Marshal.FreeHGlobal(buf);
                    // needed is advisory and often short by the time it is acted on.
                    len = Math.Max(needed + (1 << 20), len * 2);
                    if (len > (1 << 28)) return IntPtr.Zero;
                    continue;
                }
                if (status != 0) { Marshal.FreeHGlobal(buf); return IntPtr.Zero; }
                count = (int)(IntPtr.Size == 8 ? Marshal.ReadInt64(buf) : Marshal.ReadInt32(buf));
                return buf;
            }
        }

        /// <summary>
        /// The type index for Process objects, discovered rather than assumed: open a
        /// handle to ourselves, find that exact (pid, handle) pair in the table, and read
        /// the index off it.
        /// </summary>
        static int ProcessTypeIndex(IntPtr buf, int count, int stride, int self, IntPtr probe)
        {
            IntPtr first = (IntPtr)(buf.ToInt64() + IntPtr.Size * 2);
            for (int i = 0; i < count; i++)
            {
                HandleEntry e = (HandleEntry)Marshal.PtrToStructure(
                    (IntPtr)(first.ToInt64() + (long)i * stride), typeof(HandleEntry));
                if (e.OwnerPid.ToInt64() == self && e.Handle == probe) return e.ObjectTypeIndex;
            }
            return -1;
        }

        public static Holder[] Find(int targetPid)
        {
            Unreachable = 0;
            Examined = 0;
            List<Holder> found = new List<Holder>();
            int self = GetProcessId(GetCurrentProcess());
            IntPtr probe = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, self);
            if (probe == IntPtr.Zero) return found.ToArray();
            int count, stride;
            IntPtr buf = Snapshot(out count, out stride);
            if (buf == IntPtr.Zero) { CloseHandle(probe); return found.ToArray(); }
            try
            {
                int typeIndex = ProcessTypeIndex(buf, count, stride, self, probe);
                if (typeIndex < 0) return found.ToArray();

                Dictionary<int, IntPtr> owners = new Dictionary<int, IntPtr>();
                Dictionary<int, uint> best = new Dictionary<int, uint>();
                IntPtr me = GetCurrentProcess();
                IntPtr first = (IntPtr)(buf.ToInt64() + IntPtr.Size * 2);
                for (int i = 0; i < count; i++)
                {
                    HandleEntry e = (HandleEntry)Marshal.PtrToStructure(
                        (IntPtr)(first.ToInt64() + (long)i * stride), typeof(HandleEntry));
                    if (e.ObjectTypeIndex != typeIndex) continue;
                    int owner = (int)e.OwnerPid.ToInt64();
                    // The target's own handle to itself is noise: a process cannot be
                    // the outsider that killed it.
                    if (owner == self || owner == targetPid || owner == 0 || owner == 4) continue;
                    Examined++;
                    IntPtr src;
                    if (!owners.TryGetValue(owner, out src))
                    {
                        src = OpenProcess(PROCESS_DUP_HANDLE, false, owner);
                        owners[owner] = src;
                    }
                    if (src == IntPtr.Zero) { Unreachable++; continue; }
                    IntPtr dup;
                    if (!DuplicateHandle(src, e.Handle, me, out dup, 0, false, DUPLICATE_SAME_ACCESS))
                    { Unreachable++; continue; }
                    try
                    {
                        if (GetProcessId(dup) == targetPid)
                        {
                            uint acc;
                            best.TryGetValue(owner, out acc);
                            best[owner] = acc | e.GrantedAccess;
                        }
                    }
                    finally { CloseHandle(dup); }
                }
                foreach (KeyValuePair<int, IntPtr> kv in owners)
                    if (kv.Value != IntPtr.Zero) CloseHandle(kv.Value);
                foreach (KeyValuePair<int, uint> kv in best)
                {
                    Holder h = new Holder();
                    h.Pid = kv.Key;
                    h.Access = kv.Value;
                    h.CanTerminate = (kv.Value & PROCESS_TERMINATE) != 0 || (kv.Value & 0x1F0FFF) == 0x1F0FFF;
                    found.Add(h);
                }
            }
            finally
            {
                Marshal.FreeHGlobal(buf);
                CloseHandle(probe);
            }
            return found.ToArray();
        }
    }
}
'@
    $null = [MapleCW.Handles]::EnableDebugPrivilege()
}

function Get-HandleHolder {
    [CmdletBinding()]
    param([Parameter(Mandatory = $true)][int]$TargetPid)

    $rows = @()
    foreach ($h in [MapleCW.Handles]::Find($TargetPid)) {
        $name = try { (Get-Process -Id $h.Pid -ErrorAction Stop).ProcessName } catch { '?' }
        $rows += [pscustomobject]@{
            Pid          = $h.Pid
            Name         = $name
            Access       = ('0x{0:X6}' -f $h.Access)
            CanTerminate = $h.CanTerminate
        }
    }
    # Attached as note properties so a caller can tell "nobody holds one" apart from
    # "this shell could not look", which are opposite findings.
    $out = , @($rows)
    $out | Add-Member -NotePropertyName Examined -NotePropertyValue ([MapleCW.Handles]::Examined) -Force
    $out | Add-Member -NotePropertyName Unreachable -NotePropertyValue ([MapleCW.Handles]::Unreachable) -Force
    return $out
}

function Format-HandleHolder {
    param([int]$TargetPid)
    $res = Get-HandleHolder -TargetPid $TargetPid
    $rows = @($res)
    $parts = @($rows | ForEach-Object {
            "$($_.Name)#$($_.Pid)$(if ($_.CanTerminate) { '(TERMINATE)' } else { '' })"
        })
    $summary = if ($parts) { $parts -join ' ' } else { '(none)' }
    return "$summary [examined $($res.Examined) process handles, $($res.Unreachable) unreachable]"
}

if ($TargetPid -gt 0) {
    Write-Output (Format-HandleHolder -TargetPid $TargetPid)
    Get-HandleHolder -TargetPid $TargetPid | Format-Table -AutoSize
}
