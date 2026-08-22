"""Read a Windows minidump without a debugger.

Only the Microsoft Store build of WinDbg is on this machine - GUI only, no scriptable
console - and `cdb.exe` is not installed. This parses the file format directly.

    python tools/dumpwalk.py dumps\\maplecw-crash-1096760-c0000374-1.dmp

Run it with the repo as the working directory. `CLAUDE.md`'s "the scratchpad shadows the
real tools" applies to the `pdata_lookup` import below; the script also pins its own
directory onto `sys.path` so the import cannot pick up a stale copy from elsewhere.

WHAT IT PRINTS

  * self-checks - the header magic, the pid, and MapleStory.exe's image base. If those
    three do not come out, the parser is wrong, not the dump. They are printed first and
    labelled PASS/FAIL rather than assumed.
  * the exception record, all fifteen ExceptionInformation slots, and - loudly - which of
    them are parameters and which are residual bytes past NumberParameters.
  * for STATUS_HEAP_CORRUPTION, the `_HEAP_FAILURE_INFORMATION` that parameter[0] points
    at. That structure carries a 32-slot stack trace captured at the moment the allocator
    noticed the damage, which is a far better artefact than anything a stack walk can
    produce. See the note on layout at `heap_failure_info`.
  * the faulting thread's register context.
  * a real `.pdata`-driven unwind of the faulting thread, and separately a value scan of
    its stack. The unwind is the one to trust; the scan is printed because stale slots are
    sometimes the only trace of a frame that has already returned.
  * the heap list out of the PEB, with each heap's Flags/ForceFlags - which is where "was
    page heap on?" is answered from the dump itself instead of from memory of the config.

WHAT IT CANNOT DO

  * There are no allocation or free stacks in this dump, because page heap was off. That
    is a property of the configuration, not a finding about the client.
  * The unwinder does not model epilogues. A frame captured mid-epilogue unwinds wrongly.
    It also stops reading unwind codes at the first UWOP_EPILOG (opcode 6, version 2
    info), which is safe only because MSVC emits those after the prologue codes.
  * The unwind is checked against the allocator's own captured stack when one exists -
    that is a positive control taken from this exact stack, and the count is printed.
"""

import argparse
import bisect
import os
import struct
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import pdata_lookup  # noqa: E402  - repo tool, reused for the on-disk cross-check

# ---------------------------------------------------------------- stream types

STREAM_NAMES = {
    0: "Unused", 3: "ThreadList", 4: "ModuleList", 5: "MemoryList",
    6: "Exception", 7: "SystemInfo", 8: "ThreadExList", 9: "Memory64List",
    10: "CommentA", 11: "CommentW", 12: "HandleData", 13: "FunctionTable",
    14: "UnloadedModuleList", 15: "MiscInfo", 16: "MemoryInfoList",
    17: "ThreadInfoList", 18: "HandleOperationList", 19: "Token",
    21: "SystemMemoryInfo", 22: "ProcessVmCounters", 24: "ThreadNames",
}

THREAD_LIST, MODULE_LIST, EXCEPTION, SYSTEM_INFO = 3, 4, 6, 7
MEMORY64_LIST, MISC_INFO, MEMORY_INFO_LIST = 9, 15, 16

# CONTEXT_AMD64 integer registers, in layout order starting at +0x78.
GPR = ["Rax", "Rcx", "Rdx", "Rbx", "Rsp", "Rbp", "Rsi", "Rdi",
       "R8", "R9", "R10", "R11", "R12", "R13", "R14", "R15"]
CTX_GPR_OFF = 0x78
CTX_RIP_OFF = 0xF8
CTX_FLAGS_OFF = 0x30
CTX_EFLAGS_OFF = 0x44
CTX_SEG_OFF = 0x38

EXC_NAMES = {
    0xC0000005: "ACCESS_VIOLATION", 0xC000001D: "ILLEGAL_INSTRUCTION",
    0xC0000025: "NONCONTINUABLE_EXCEPTION", 0xC000008C: "ARRAY_BOUNDS_EXCEEDED",
    0xC0000094: "INTEGER_DIVIDE_BY_ZERO", 0xC00000FD: "STACK_OVERFLOW",
    0xC0000374: "HEAP_CORRUPTION", 0xC0000409: "STACK_BUFFER_OVERRUN",
    0x80000003: "BREAKPOINT", 0x40010006: "DBG_PRINTEXCEPTION_C",
    0x406D1388: "MS_VC_THREAD_NAME",
}

# ntdll's _HEAP_FAILURE_TYPE. This list is a CLAIM taken from the enum as it is commonly
# published, not something read out of this machine's ntdll - there are no symbols here to
# check it against. `report_heap_failure` prints the raw number first and prints evidence
# that can disagree with the name.
HEAP_FAILURE_TYPE = {
    0: "heap_failure_internal",
    1: "heap_failure_unknown",
    2: "heap_failure_generic",
    3: "heap_failure_entry_corruption",
    4: "heap_failure_multiple_entries_corruption",
    5: "heap_failure_virtual_block_corruption",
    6: "heap_failure_buffer_overrun",
    7: "heap_failure_buffer_underrun",
    8: "heap_failure_block_not_busy",
    9: "heap_failure_invalid_argument",
    10: "heap_failure_usage_after_free",
    11: "heap_failure_cross_heap_operation",
    12: "heap_failure_freelists_corruption",
    13: "heap_failure_listentry_corruption",
    14: "heap_failure_lfh_bitmap_mismatch",
    15: "heap_failure_lfh_subsegment_counts_mismatch",
}

MEM_STATE = {0x1000: "COMMIT", 0x2000: "RESERVE", 0x10000: "FREE"}
MEM_TYPE = {0x20000: "PRIVATE", 0x40000: "MAPPED", 0x1000000: "IMAGE"}
PAGE_PROT = {
    0x01: "NOACCESS", 0x02: "READONLY", 0x04: "READWRITE", 0x08: "WRITECOPY",
    0x10: "EXECUTE", 0x20: "EXECUTE_READ", 0x40: "EXECUTE_READWRITE",
    0x80: "EXECUTE_WRITECOPY",
}


# ------------------------------------------------------------------ the reader

class MiniDump(object):
    """Streams and memory out of a .dmp, read in place - the file is never loaded whole."""

    def __init__(self, path):
        self.path = path
        self.f = open(path, "rb")
        self.size = os.path.getsize(path)
        head = self._at(0, 32)
        self.magic = head[:4]
        (self.version, self.n_streams, self.dir_rva,
         self.checksum, self.timestamp) = struct.unpack_from("<IIIII", head, 4)
        self.flags = struct.unpack_from("<Q", head, 24)[0]
        self.streams = {}
        self.stream_order = []
        d = self._at(self.dir_rva, self.n_streams * 12)
        for i in range(self.n_streams):
            t, sz, rva = struct.unpack_from("<III", d, i * 12)
            self.stream_order.append((t, sz, rva))
            if sz:
                self.streams[t] = (sz, rva)
        self._mem = None
        self._mem_starts = None
        self._regions = None

    def _at(self, off, n):
        self.f.seek(off)
        return self.f.read(n)

    def stream(self, t):
        if t not in self.streams:
            return None
        sz, rva = self.streams[t]
        return self._at(rva, sz)

    # -- Memory64List ------------------------------------------------------

    def _build_memory(self):
        if self._mem is not None:
            return
        self._mem, self._mem_starts = [], []
        raw = self.stream(MEMORY64_LIST)
        if raw is None:
            return
        n, base_rva = struct.unpack_from("<QQ", raw, 0)
        off = base_rva
        for i in range(n):
            start, size = struct.unpack_from("<QQ", raw, 16 + i * 16)
            self._mem.append((start, size, off))
            self._mem_starts.append(start)
            off += size

    def read(self, va, n):
        """Bytes at a virtual address, or None. Never spans two ranges."""
        self._build_memory()
        if not self._mem:
            return None
        i = bisect.bisect_right(self._mem_starts, va) - 1
        if i < 0:
            return None
        start, size, off = self._mem[i]
        if va < start or va >= start + size:
            return None
        n = min(n, start + size - va)
        return self._at(off + (va - start), n)

    def u64(self, va):
        b = self.read(va, 8)
        return None if b is None or len(b) < 8 else struct.unpack("<Q", b)[0]

    def u32(self, va):
        b = self.read(va, 4)
        return None if b is None or len(b) < 4 else struct.unpack("<I", b)[0]

    def memory_total(self):
        self._build_memory()
        return sum(sz for _, sz, _ in self._mem)

    # -- MemoryInfoList ----------------------------------------------------

    def region(self, va):
        if self._regions is None:
            self._regions = []
            raw = self.stream(MEMORY_INFO_LIST)
            if raw:
                hsz, esz = struct.unpack_from("<II", raw, 0)
                n = struct.unpack_from("<Q", raw, 8)[0]
                for i in range(n):
                    o = hsz + i * esz
                    base, alloc, aprot, _p1, rsize, state, prot, mtype = \
                        struct.unpack_from("<QQIIQIII", raw, o)
                    self._regions.append(
                        (base, rsize, alloc, aprot, state, prot, mtype))
                self._regions.sort()
            self._region_starts = [r[0] for r in self._regions]
        if not self._regions:
            return None
        i = bisect.bisect_right(self._region_starts, va) - 1
        if i < 0:
            return None
        r = self._regions[i]
        return r if va < r[0] + r[1] else None

    # -- strings -----------------------------------------------------------

    def minidump_string(self, rva):
        n = struct.unpack("<I", self._at(rva, 4))[0]
        return self._at(rva + 4, n).decode("utf-16-le", "replace")


# ------------------------------------------------------------------- symbols

class Module(object):
    def __init__(self, base, size, name, ts, checksum):
        self.base, self.size, self.path = base, size, name
        self.name = name.rsplit("\\", 1)[-1]
        self.timestamp, self.checksum = ts, checksum
        self._pdata = None
        self._pdata_starts = None
        self._exports = None
        self._export_rvas = None
        self._dirs = None

    def contains(self, va):
        return self.base <= va < self.base + self.size


class Symbols(object):
    """Resolve an address to module + containing function + nearest export.

    The function bounds come from the image's .pdata, read out of the dump's own memory so
    the answer is about the image that was actually mapped. Export names are a separate,
    weaker signal: a nearest-preceding-export match on a non-exported function is a guess
    and is printed as one.
    """

    def __init__(self, dump, modules):
        self.dump = dump
        self.modules = sorted(modules, key=lambda m: m.base)
        self._bases = [m.base for m in self.modules]

    def module_for(self, va):
        i = bisect.bisect_right(self._bases, va) - 1
        if i < 0:
            return None
        m = self.modules[i]
        return m if m.contains(va) else None

    def _dirs_of(self, m):
        if m._dirs is None:
            m._dirs = {}
            hdr = self.dump.read(m.base, 0x400)
            if hdr and hdr[:2] == b"MZ":
                pe = struct.unpack_from("<I", hdr, 0x3C)[0]
                if pe + 0x108 < len(hdr) and hdr[pe:pe + 4] == b"PE\0\0":
                    opt = pe + 24
                    magic = struct.unpack_from("<H", hdr, opt)[0]
                    dd = opt + (112 if magic == 0x20B else 96)
                    nd = struct.unpack_from("<I", hdr, dd - 4)[0]
                    for i in range(min(nd, 16)):
                        rva, sz = struct.unpack_from("<II", hdr, dd + i * 8)
                        m._dirs[i] = (rva, sz)
        return m._dirs

    def pdata(self, m):
        """Sorted RUNTIME_FUNCTION (begin, end, unwind) RVAs for a module."""
        if m._pdata is None:
            m._pdata, m._pdata_starts = [], []
            rva, sz = self._dirs_of(m).get(3, (0, 0))
            if rva and sz:
                raw = self.dump.read(m.base + rva, sz)
                if raw:
                    for i in range(len(raw) // 12):
                        b, e, u = struct.unpack_from("<III", raw, i * 12)
                        if b == 0 and e == 0:
                            continue
                        m._pdata.append((b, e, u))
                    m._pdata.sort()
                    m._pdata_starts = [p[0] for p in m._pdata]
        return m._pdata

    def function(self, va):
        """(func_start_va, func_end_va) containing va, or None."""
        m = self.module_for(va)
        if m is None:
            return None
        p = self.pdata(m)
        if not p:
            return None
        i = bisect.bisect_right(m._pdata_starts, va - m.base) - 1
        if i < 0:
            return None
        b, e, _u = p[i]
        if va - m.base >= e:
            return None
        return (m.base + b, m.base + e)

    def exports(self, m):
        if m._exports is None:
            m._exports, m._export_rvas = [], []
            rva, sz = self._dirs_of(m).get(0, (0, 0))
            if rva and sz:
                d = self.dump.read(m.base + rva, 40)
                if d and len(d) == 40:
                    nfun, nnam = struct.unpack_from("<II", d, 20)
                    afun, anam, aord = struct.unpack_from("<III", d, 28)
                    funs = self.dump.read(m.base + afun, nfun * 4) or b""
                    nams = self.dump.read(m.base + anam, nnam * 4) or b""
                    ords = self.dump.read(m.base + aord, nnam * 2) or b""
                    by_ord = {}
                    for i in range(len(nams) // 4):
                        nr = struct.unpack_from("<I", nams, i * 4)[0]
                        o = struct.unpack_from("<H", ords, i * 2)[0]
                        s = self.dump.read(m.base + nr, 96)
                        if s:
                            by_ord[o] = s.split(b"\0", 1)[0].decode("latin1")
                    for i in range(len(funs) // 4):
                        fr = struct.unpack_from("<I", funs, i * 4)[0]
                        if not fr or rva <= fr < rva + sz:  # forwarder
                            continue
                        m._exports.append((fr, by_ord.get(i, "ord#%d" % i)))
                    m._exports.sort()
                    m._export_rvas = [e[0] for e in m._exports]
        return m._exports

    def describe(self, va, want_export=True):
        m = self.module_for(va)
        if m is None:
            r = self.dump.region(va)
            if r:
                return "%#018x  [%s %s %s]" % (
                    va, MEM_STATE.get(r[4], hex(r[4])),
                    MEM_TYPE.get(r[6], hex(r[6])), PAGE_PROT.get(r[5], hex(r[5])))
            return "%#018x  [not in any module]" % va
        out = "%#018x  %s+%#x" % (va, m.name, va - m.base)
        fn = self.function(va)
        if fn:
            out += "  fn %#x+%#x" % (fn[0], va - fn[0])
        if fn and fn[1] - fn[0] > 0x8000:
            out += "  [span %#x - packer region, probably not a real function]" % (
                fn[1] - fn[0])
        if want_export:
            ex = self.exports(m)
            if ex:
                i = bisect.bisect_right(m._export_rvas, va - m.base) - 1
                if i >= 0:
                    er, en = ex[i]
                    delta = (va - m.base) - er
                    if delta == 0:
                        out += "  = %s!%s" % (m.name, en)
                    elif fn and fn[0] - m.base == er:
                        out += "  = %s!%s+%#x" % (m.name, en, delta)
                    elif delta < 0x8000:
                        out += "  (~%s!%s+%#x)" % (m.name, en, delta)
        return out


SYMBOL_LEGEND = """  Reading the symbol column: `= mod!name+0x..` is EXACT - the containing function's
  start is that exported address. `(~mod!name+0x..)` is only the nearest export BEFORE
  the address and names nothing; internal functions inherit whatever export precedes
  them, so treat those as orientation, not identification. `fn 0x..` is the containing
  function from the image's .pdata, which is authoritative."""


# ------------------------------------------------------------------ unwinding

UWOP_PUSH_NONVOL, UWOP_ALLOC_LARGE, UWOP_ALLOC_SMALL, UWOP_SET_FPREG = 0, 1, 2, 3
UWOP_SAVE_NONVOL, UWOP_SAVE_NONVOL_FAR, UWOP_EPILOG = 4, 5, 6
UWOP_SAVE_XMM128, UWOP_SAVE_XMM128_FAR, UWOP_PUSH_MACHFRAME = 8, 9, 10


class Unwinder(object):
    """RtlVirtualUnwind, by hand, from the images mapped in the dump.

    Not a heuristic: each step reads the function's UNWIND_INFO and replays the prologue
    backwards. It is checked against the allocator's captured stack in `main` - if those
    frames do not reappear here, believe the captured one and not this.
    """

    def __init__(self, dump, syms):
        self.dump, self.syms = dump, syms
        self.notes = []

    def _unwind_info(self, m, unwind_rva):
        raw = self.dump.read(m.base + unwind_rva, 4)
        if not raw or len(raw) < 4:
            return None
        ver_flags, size_prolog, count, frame = struct.unpack("<BBBB", raw)
        version, flags = ver_flags & 7, ver_flags >> 3
        codes = self.dump.read(m.base + unwind_rva + 4, count * 2 + 2) or b""
        return dict(version=version, flags=flags, size_prolog=size_prolog,
                    count=count, frame_reg=frame & 0xF, frame_off=(frame >> 4) * 16,
                    codes=codes, rva=unwind_rva)

    def step(self, regs):
        """One frame. Returns (frame_dict, next_regs) or (frame_dict, None) at the end."""
        rip, rsp = regs["Rip"], regs["Rsp"]
        m = self.syms.module_for(rip)
        frame = dict(rip=rip, rsp=rsp, kind="leaf")
        if m is None:
            return frame, None
        p = self.syms.pdata(m)
        rf = None
        if p:
            i = bisect.bisect_right(m._pdata_starts, rip - m.base) - 1
            if i >= 0 and rip - m.base < p[i][1]:
                rf = p[i]
        if rf is None:
            # Leaf function: no unwind data, return address sits at [rsp].
            ret = self.dump.u64(rsp)
            if ret is None:
                return frame, None
            nxt = dict(regs)
            nxt["Rip"], nxt["Rsp"] = ret, rsp + 8
            return frame, nxt

        frame["kind"] = "unwound"
        frame["func"] = m.base + rf[0]
        regs = dict(regs)
        off = rip - m.base - rf[0]
        unwind_rva = rf[2]
        chain_guard = 0
        while True:
            chain_guard += 1
            if chain_guard > 32:
                self.notes.append("unwind chain > 32 at %#x" % rip)
                break
            info = self._unwind_info(m, unwind_rva & ~1)
            if info is None:
                break
            regs = self._apply(info, regs, off)
            if regs is None:
                return frame, None
            if info["flags"] & 0x4:  # UNW_FLAG_CHAININFO
                base = (info["count"] + 1) & ~1
                nxt = self.dump.read(
                    m.base + (unwind_rva & ~1) + 4 + base * 2, 12)
                if not nxt or len(nxt) < 12:
                    break
                cb, ce, cu = struct.unpack("<III", nxt)
                off = max(off - (cb - rf[0]), 0) if cb <= rf[0] else 0xFFFF
                unwind_rva = cu
                continue
            break
        ret = self.dump.u64(regs["Rsp"])
        if ret is None or ret == 0:
            return frame, None
        regs["Rip"] = ret
        regs["Rsp"] = regs["Rsp"] + 8
        return frame, regs

    def _apply(self, info, regs, off_in_func):
        codes, count = info["codes"], info["count"]
        in_prolog = off_in_func < info["size_prolog"]
        i = 0
        while i < count:
            if 2 * i + 1 >= len(codes):
                break
            code_off = codes[2 * i]
            op = codes[2 * i + 1] & 0xF
            opinfo = codes[2 * i + 1] >> 4
            slots = 1
            if op == UWOP_ALLOC_LARGE:
                slots = 2 if opinfo == 0 else 3
            elif op in (UWOP_SAVE_NONVOL, UWOP_SAVE_XMM128):
                slots = 2
            elif op in (UWOP_SAVE_NONVOL_FAR, UWOP_SAVE_XMM128_FAR):
                slots = 3
            elif op == UWOP_EPILOG:
                # Version-2 epilogue metadata. MSVC emits it after the prologue codes, so
                # everything that matters has already been applied. Stop rather than risk
                # mis-stepping the node list.
                break
            elif op == 7:
                slots = 3
            skip = in_prolog and code_off > off_in_func
            if not skip:
                if op == UWOP_PUSH_NONVOL:
                    v = self.dump.u64(regs["Rsp"])
                    if v is None:
                        return None
                    regs[GPR[opinfo]] = v
                    regs["Rsp"] += 8
                elif op == UWOP_ALLOC_LARGE:
                    if opinfo == 0:
                        regs["Rsp"] += struct.unpack_from(
                            "<H", codes, 2 * (i + 1))[0] * 8
                    else:
                        regs["Rsp"] += struct.unpack_from(
                            "<I", codes, 2 * (i + 1))[0]
                elif op == UWOP_ALLOC_SMALL:
                    regs["Rsp"] += opinfo * 8 + 8
                elif op == UWOP_SET_FPREG:
                    regs["Rsp"] = regs[GPR[info["frame_reg"]]] - info["frame_off"]
                elif op in (UWOP_SAVE_NONVOL, UWOP_SAVE_NONVOL_FAR):
                    if op == UWOP_SAVE_NONVOL:
                        d = struct.unpack_from("<H", codes, 2 * (i + 1))[0] * 8
                    else:
                        d = struct.unpack_from("<I", codes, 2 * (i + 1))[0]
                    v = self.dump.u64(regs["Rsp"] + d)
                    if v is not None:
                        regs[GPR[opinfo]] = v
                elif op == UWOP_PUSH_MACHFRAME:
                    regs["Rsp"] += 40 if opinfo == 0 else 48
            i += slots
        return regs

    def walk(self, regs, limit=64):
        frames, seen = [], set()
        cur = dict(regs)
        for _ in range(limit):
            fr, nxt = self.step(cur)
            frames.append(fr)
            if nxt is None:
                break
            key = (nxt["Rip"], nxt["Rsp"])
            if key in seen or nxt["Rsp"] <= cur["Rsp"]:
                if nxt["Rsp"] <= cur["Rsp"]:
                    self.notes.append(
                        "stopped: rsp did not advance at %#x" % cur["Rip"])
                break
            seen.add(key)
            cur = nxt
        return frames


# ---------------------------------------------------------------- the streams

def parse_modules(dump):
    raw = dump.stream(MODULE_LIST)
    if raw is None:
        return []
    n = struct.unpack_from("<I", raw, 0)[0]
    out = []
    for i in range(n):
        o = 4 + i * 108
        base, size, checksum, ts, name_rva = struct.unpack_from("<QIIII", raw, o)
        out.append(Module(base, size, dump.minidump_string(name_rva), ts, checksum))
    return out


def parse_threads(dump):
    raw = dump.stream(THREAD_LIST)
    if raw is None:
        return []
    n = struct.unpack_from("<I", raw, 0)[0]
    out = []
    for i in range(n):
        o = 4 + i * 48
        tid, susp, pcls, pri, teb, stk_start, stk_sz, stk_rva, ctx_sz, ctx_rva = \
            struct.unpack_from("<IIIIQQIIII", raw, o)
        out.append(dict(tid=tid, suspend=susp, priority_class=pcls, priority=pri,
                        teb=teb, stack=(stk_start, stk_sz), stack_rva=stk_rva,
                        ctx=(ctx_sz, ctx_rva)))
    return out


def parse_context(dump, ctx_sz, ctx_rva):
    b = dump._at(ctx_rva, min(ctx_sz, 1232))
    if len(b) < 0x100:
        return None
    regs = {}
    for i, nm in enumerate(GPR):
        regs[nm] = struct.unpack_from("<Q", b, CTX_GPR_OFF + i * 8)[0]
    regs["Rip"] = struct.unpack_from("<Q", b, CTX_RIP_OFF)[0]
    regs["EFlags"] = struct.unpack_from("<I", b, CTX_EFLAGS_OFF)[0]
    regs["ContextFlags"] = struct.unpack_from("<I", b, CTX_FLAGS_OFF)[0]
    regs["MxCsr"] = struct.unpack_from("<I", b, 0x34)[0]
    cs, ds, es, fs, gs, ss = struct.unpack_from("<HHHHHH", b, CTX_SEG_OFF)
    regs["SegCs"], regs["SegSs"] = cs, ss
    return regs


def parse_exception(dump):
    raw = dump.stream(EXCEPTION)
    if raw is None:
        return None
    tid = struct.unpack_from("<I", raw, 0)[0]
    code, flags, rec, addr, nparam = struct.unpack_from("<IIQQI", raw, 8)
    info = [struct.unpack_from("<Q", raw, 40 + i * 8)[0] for i in range(15)]
    ctx_sz, ctx_rva = struct.unpack_from("<II", raw, 160)
    return dict(tid=tid, code=code, flags=flags, record=rec, address=addr,
                nparam=nparam, info=info, ctx=(ctx_sz, ctx_rva))


def parse_system_info(dump):
    raw = dump.stream(SYSTEM_INFO)
    if raw is None:
        return None
    arch, level, rev = struct.unpack_from("<HHH", raw, 0)
    ncpu, ptype = struct.unpack_from("<BB", raw, 6)
    major, minor, build, plat = struct.unpack_from("<IIII", raw, 8)
    return dict(arch=arch, ncpu=ncpu, product=ptype,
                version="%d.%d.%d" % (major, minor, build), level=level, rev=rev)


def parse_misc(dump):
    raw = dump.stream(MISC_INFO)
    if raw is None:
        return None
    size, flags1 = struct.unpack_from("<II", raw, 0)
    out = dict(size=size, flags1=flags1)
    if size >= 24:
        pid, create, user, kern = struct.unpack_from("<IIII", raw, 8)
        out.update(pid=pid, create_time=create, user_time=user, kernel_time=kern)
    return out


def parse_vm_counters(dump):
    """MINIDUMP_PROCESS_VM_COUNTERS_2.

    The first version of this read `<IIQQ...` from offset 8 and reported the *current*
    working set as the peak and a quota field as the working set - a plausible-looking
    number in the wrong field, which is the mistake `CLAUDE.md` names as "the unit, not the
    arithmetic". The layout is USHORT Revision, USHORT Flags, ULONG PageFaultCount, then
    ULONG64s from +8, and it is checked below against something that can disagree:
    PrivateUsage normally equals PagefileUsage, and VirtualSize must exceed WorkingSetSize.
    """
    raw = dump.stream(22)
    if raw is None or len(raw) < 0x70:
        return None
    rev, flags, faults = struct.unpack_from("<HHI", raw, 0)
    q = struct.unpack_from("<" + "Q" * 13, raw, 8)
    out = dict(revision=rev, flags=flags, page_fault_count=faults,
               peak_working_set=q[0], working_set=q[1],
               pagefile=q[6], peak_pagefile=q[7],
               peak_virtual=q[8], virtual=q[9], private=q[10],
               private_working_set=q[11], shared_commit=q[12])
    out["layout_ok"] = (out["peak_working_set"] >= out["working_set"]
                        and out["virtual"] > out["working_set"]
                        and out["peak_virtual"] >= out["virtual"])
    return out


# ------------------------------------------------------------ heap structures

def heap_failure_info(dump, va):
    """Decode ntdll's `_HEAP_FAILURE_INFORMATION`.

    The offsets below are a claim, not a fact - there are no symbols on this machine. Two
    things in the data agree with them and are printed so the claim can fail out loud:

      * StackTrace is 32 pointers at +0x58, so it ends exactly at +0x158. In this dump the
        pointer run stops at +0x138 and +0x140..+0x150 are zero, which is what a partly
        filled 32-slot array looks like and not what a coincidence looks like.
      * The last two live entries are the canonical bottom of a Windows thread
        (BaseThreadInitThunk, RtlUserThreadStart). A wrong offset would not land on those.
    """
    b = dump.read(va, 0x160)
    if not b or len(b) < 0x160:
        return None
    ver, length, ftype = struct.unpack_from("<III", b, 0)
    heap, addr = struct.unpack_from("<QQ", b, 0x10)
    p1, p2, p3 = struct.unpack_from("<QQQ", b, 0x20)
    prev_blk, next_blk = struct.unpack_from("<QQ", b, 0x38)
    expected = struct.unpack_from("<QQ", b, 0x48)
    trace = [struct.unpack_from("<Q", b, 0x58 + i * 8)[0] for i in range(32)]
    return dict(version=ver, length=length, type=ftype, heap=heap, address=addr,
                param1=p1, param2=p2, param3=p3,
                previous_block=prev_blk, next_block=next_blk,
                expected_entry=expected, trace=trace)


def heap_summary(dump, teb):
    """PEB ProcessHeaps. The cheap half of `!heap -s`; see the note in `main`."""
    peb = dump.u64(teb + 0x60)
    if peb is None:
        return None
    default = dump.u64(peb + 0x30)
    nheaps = dump.u32(peb + 0xE8)
    arr = dump.u64(peb + 0xF0)
    if not nheaps or nheaps > 64 or not arr:
        return None
    heaps = []
    raw = dump.read(arr, nheaps * 8) or b""
    for i in range(len(raw) // 8):
        h = struct.unpack_from("<Q", raw, i * 8)[0]
        sig = dump.u32(h + 0x10)
        fl = dump.u32(h + 0x70)
        ff = dump.u32(h + 0x74)
        mask = dump.u32(h + 0x7C)
        enc = dump.u64(h + 0x88)
        heaps.append(dict(base=h, signature=sig, flags=fl, force_flags=ff,
                          encode_mask=mask, encoding=enc))
    return dict(peb=peb, default=default, count=nheaps, heaps=heaps)


def containing_block(dump, dump_region_alloc_base, target, encoding, limit=4000000):
    """Walk a heap segment's block chain and find the block that contains `target`.

    This is the measurement that separates the two stories a heap fault always has:

      * `target` lands inside a large, BUSY, valid block  ->  the metadata was never
        touched and somebody freed an interior pointer.
      * `target` is where a block header should be, and the chain breaks there  ->  the
        metadata really was overwritten.

    The walk verifies itself: every step decodes an entry, checks the standard
    `b0^b1^b2 == b3` header checksum, and steps by the entry's own size. A wrong
    Encoding key or a wrong offset does not walk hundreds of blocks - it fails on the
    first one. The number of blocks walked is returned so the caller can print it.
    """
    seg = dump_region_alloc_base
    if dump.u32(seg + 0x10) != 0xFFEEFFEE:
        return None
    first, last = dump.u64(seg + 0x40), dump.u64(seg + 0x48)
    if not first or not last:
        return None
    e, n = first, 0
    while e and e < last and n < limit:
        d = decode_heap_entry(dump, e, encoding)
        if d is None or d["size"] == 0 or not d["checksum_ok"]:
            return dict(walked=n, broke_at=e,
                        why="unreadable" if d is None else
                            ("size 0" if d["size"] == 0 else "header checksum failed"))
        nxt = e + d["size"] * 16
        if e <= target < nxt:
            return dict(walked=n, entry=e, end=nxt, bytes=d["size"] * 16,
                        busy=bool(d["flags"] & 1), flags=d["flags"],
                        unused=d["unused"], user=e + 0x10,
                        usable=d["size"] * 16 - 0x10 - d["unused"],
                        offset=target - (e + 0x10))
        e = nxt
        n += 1
    return dict(walked=n, broke_at=e, why="ran off the segment")


def decode_heap_entry(dump, entry_va, encoding):
    """XOR-decode the 8 header bytes at entry+8 and report the standard checksum."""
    raw = dump.read(entry_va, 16)
    if not raw or len(raw) < 16:
        return None
    enc = struct.unpack_from("<Q", raw, 8)[0]
    dec = enc ^ (encoding or 0)
    d = struct.pack("<Q", dec)
    size, flags, tag, prev_size, seg_off, unused = struct.unpack("<HBBHBB", d)
    ok = (d[0] ^ d[1] ^ d[2]) == d[3]
    return dict(encoded=enc, decoded=dec, size=size, flags=flags, tag=tag,
                prev_size=prev_size, seg_off=seg_off, unused=unused, checksum_ok=ok)


# --------------------------------------------------------------------- output

def hexdump(dump, va, length, indent="  "):
    out = []
    for off in range(0, length, 16):
        b = dump.read(va + off, 16)
        if not b:
            out.append("%s%016x  <not in dump>" % (indent, va + off))
            continue
        txt = "".join(chr(c) if 32 <= c < 127 else "." for c in b)
        out.append("%s%016x  %-47s  %s" % (indent, va + off, b.hex(" "), txt))
    return "\n".join(out)


def rule(title):
    print("")
    print("=" * 78)
    print("== " + title)
    print("=" * 78)


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("dump")
    ap.add_argument("--thread", type=lambda s: int(s, 0), default=None,
                    help="walk this thread id instead of the faulting one")
    ap.add_argument("--frames", type=int, default=64)
    ap.add_argument("--scan", type=int, default=0x4000,
                    help="bytes of stack to scan above rsp (0 disables)")
    ap.add_argument("--raw", nargs=2, default=None, metavar=("VA", "LEN"),
                    help="hex dump LEN bytes at VA and exit")
    ap.add_argument("--dis", nargs=2, default=None, metavar=("VA", "LEN"),
                    help="disassemble LEN bytes at VA and exit (needs capstone). Reads "
                         "the image AS MAPPED, so it sees packer-unpacked code that is "
                         "not in the file on disk")
    ap.add_argument("--modules", action="store_true", help="print the full module list")
    ap.add_argument("--threads", action="store_true", help="print the full thread list")
    ap.add_argument("--expect-pid", type=int, default=None)
    ap.add_argument("--expect-image", default="MapleStory.exe")
    ap.add_argument("--expect-base", type=lambda s: int(s, 0), default=0x140000000)
    args = ap.parse_args()

    dump = MiniDump(args.dump)
    modules = parse_modules(dump)
    syms = Symbols(dump, modules)

    if args.raw:
        va = int(args.raw[0], 0)
        print(hexdump(dump, va, int(args.raw[1], 0)))
        return

    if args.dis:
        try:
            import capstone
        except ImportError:
            print("capstone is not installed; --dis needs it. Everything else works.")
            return
        va, n = int(args.dis[0], 0), int(args.dis[1], 0)
        b = dump.read(va, n) or b""
        md = capstone.Cs(capstone.CS_ARCH_X86, capstone.CS_MODE_64)
        print("  %s" % syms.describe(va))
        for ins in md.disasm(b, va):
            print("  %#x  %-22s %s" % (ins.address, ins.mnemonic, ins.op_str))
        return

    # ---------------------------------------------------------- self-checks
    rule("SELF-CHECKS  (if these fail the parser is wrong, not the dump)")
    misc = parse_misc(dump) or {}
    image = next((m for m in modules if m.name.lower() == args.expect_image.lower()), None)
    pid = misc.get("pid")
    want_pid = args.expect_pid
    if want_pid is None:
        base = os.path.basename(args.dump)
        parts = base.split("-")
        for p in parts:
            if p.isdigit() and len(p) >= 5:
                want_pid = int(p)
                break
    checks = [
        ("header magic is MDMP", dump.magic == b"MDMP", dump.magic),
        ("stream directory parses", len(dump.streams) > 0,
         "%d streams" % len(dump.streams)),
        ("module list has %s" % args.expect_image, image is not None,
         image.path if image else "absent"),
        ("%s image base is %#x" % (args.expect_image, args.expect_base),
         image is not None and image.base == args.expect_base,
         "%#x" % image.base if image else "-"),
        ("process id", want_pid is None or pid == want_pid,
         "%s (expected %s)" % (pid, want_pid)),
        ("Memory64List covers the file",
         abs(dump.memory_total() - dump.size) < dump.size * 0.02,
         "%d of %d bytes" % (dump.memory_total(), dump.size)),
    ]
    failed = 0
    for name, ok, detail in checks:
        print("  [%s] %-42s %s" % ("PASS" if ok else "FAIL", name, detail))
        failed += 0 if ok else 1
    if failed:
        print("\n  %d self-check(s) FAILED - treat everything below as unreliable."
              % failed)

    # -------------------------------------------------------------- header
    rule("FILE")
    print("  %s" % os.path.abspath(args.dump))
    print("  %d bytes, minidump version %#x, flags %#x" %
          (dump.size, dump.version, dump.flags))
    print("  streams: %s" % ", ".join(
        "%s(%d)" % (STREAM_NAMES.get(t, "?"), sz)
        for t, sz, _ in dump.stream_order if sz))

    si = parse_system_info(dump)
    if si:
        print("  windows %s, %d cpus, arch %d" % (si["version"], si["ncpu"], si["arch"]))
    if misc:
        print("  pid %s, user time %ss, kernel time %ss" %
              (misc.get("pid"), misc.get("user_time"), misc.get("kernel_time")))
        if misc.get("create_time"):
            print("  process created %d, dump written %d -> alive %d s" %
                  (misc["create_time"], dump.timestamp,
                   dump.timestamp - misc["create_time"]))
    vm = parse_vm_counters(dump)
    if vm:
        mb = 1048576.0
        print("  working set %.1f MB (peak %.1f MB), private %.1f MB, "
              "pagefile %.1f MB (peak %.1f MB)" %
              (vm["working_set"] / mb, vm["peak_working_set"] / mb,
               vm["private"] / mb, vm["pagefile"] / mb, vm["peak_pagefile"] / mb))
        print("  virtual %.1f MB (peak %.1f MB), %d page faults   [layout check: %s]" %
              (vm["virtual"] / mb, vm["peak_virtual"] / mb, vm["page_fault_count"],
               "PASS" if vm["layout_ok"] else "FAIL - field offsets are wrong"))
    print("  %d modules, %d threads, %.1f MB of memory in Memory64List" %
          (len(modules), len(parse_threads(dump)), dump.memory_total() / 1048576.0))

    # ----------------------------------------------------------- exception
    exc = parse_exception(dump)
    threads = parse_threads(dump)
    if exc is None:
        print("\n  no ExceptionStream in this dump.")
        return

    rule("EXCEPTION")
    print(SYMBOL_LEGEND)
    print("")
    print("  thread          %d" % exc["tid"])
    print("  code            %#010x  %s" %
          (exc["code"], EXC_NAMES.get(exc["code"], "?")))
    fl = []
    if exc["flags"] & 0x1:
        fl.append("NONCONTINUABLE")
    if exc["flags"] & 0x2:
        fl.append("UNWINDING")
    if exc["flags"] & 0x80:
        fl.append("SOFTWARE_ORIGINATE")
    print("  flags           %#010x  %s" % (exc["flags"], "|".join(fl) or "-"))
    print("  address         %s" % syms.describe(exc["address"]))
    print("  nested record   %#x" % exc["record"])
    print("  NumberParameters %d" % exc["nparam"])
    print("")
    for i, v in enumerate(exc["info"]):
        tag = "param" if i < exc["nparam"] else " past "
        note = "" if i < exc["nparam"] else "   <- NOT a parameter"
        print("   %s[%2d] %s%s" % (tag, i, syms.describe(v, want_export=False), note))
    print("")
    print("  Only the first %d slot(s) are exception parameters. Everything below that is"
          % exc["nparam"])
    print("  whatever was in the EXCEPTION_RECORD's 15-slot array and must not be read as")
    print("  data - some of it is stale stack, some is the writer's own frame.")

    if exc["code"] == 0xC0000374:
        print("")
        print("  STATUS_HEAP_CORRUPTION is raised at the NEXT allocator walk, not where the")
        print("  damage was done. The fault address above names ntdll because ntdll is what")
        print("  noticed. It is not the bug. The stack is where the value is.")

    # ------------------------------------------------------- heap failure
    if exc["code"] == 0xC0000374 and exc["nparam"] >= 1:
        rule("HEAP FAILURE INFORMATION  (ntdll!_HEAP_FAILURE_INFORMATION @ param[0])")
        hfi = heap_failure_info(dump, exc["info"][0])
        if hfi is None:
            print("  param[0] %#x is not readable in this dump." % exc["info"][0])
        else:
            tname = HEAP_FAILURE_TYPE.get(hfi["type"], "unknown")
            print("  Version         %d" % hfi["version"])
            print("  Length          %#x" % hfi["length"])
            print("  Type            %d   candidate name: %s" % (hfi["type"], tname))
            print("                  (the enum is a claim - no symbols here to check it)")
            print("  Heap            %s" % syms.describe(hfi["heap"], False))
            print("  Address         %s" % syms.describe(hfi["address"], False))
            print("  Param1/2/3      %#x %#x %#x" %
                  (hfi["param1"], hfi["param2"], hfi["param3"]))
            print("  PreviousBlock   %#x" % hfi["previous_block"])
            print("  NextBlock       %#x" % hfi["next_block"])
            print("  ExpectedEntry   %#x %#x" % hfi["expected_entry"])
            live = [t for t in hfi["trace"] if t]
            print("")
            print("  StackTrace[32]: %d slots used, %d zero. Innermost first." %
                  (len(live), 32 - len(live)))
            print("  This is the allocator's own capture at the moment it noticed the")
            print("  damage - a real backtrace, not a scan. It is the best artefact here.")
            print("")
            for i, t in enumerate(hfi["trace"]):
                if not t:
                    continue
                print("   #%-2d %s" % (i, syms.describe(t)))
            tail = [t for t in live[-2:]]
            ok_tail = all(syms.module_for(t) and
                          syms.module_for(t).name.lower() in ("kernel32.dll", "ntdll.dll")
                          for t in tail)
            print("")
            print("  [%s] bottom-of-thread control: last two frames are kernel32/ntdll"
                  % ("PASS" if ok_tail else "FAIL"))

            hs = None
            ft = next((t for t in threads if t["tid"] == exc["tid"]), None)
            if ft:
                hs = heap_summary(dump, ft["teb"])
            enc = None
            if hs:
                for h in hs["heaps"]:
                    if h["base"] == hfi["heap"]:
                        enc = h["encoding"]
            rule("MEMORY AT THE REPORTED FAILURE ADDRESS")
            r = dump.region(hfi["address"])
            if r:
                print("  region %#x + %#x, AllocationBase %#x, %s %s %s" %
                      (r[0], r[1], r[2], MEM_STATE.get(r[4], hex(r[4])),
                       MEM_TYPE.get(r[6], hex(r[6])), PAGE_PROT.get(r[5], hex(r[5]))))
                seg_sig = dump.u32(r[2] + 0x10)
                seg_heap = dump.u64(r[2] + 0x28)
                if seg_sig is not None:
                    print("  AllocationBase+0x10 = %#x %s, +0x28 = %#x %s" %
                          (seg_sig,
                           "(FFEEFFEE: an NT heap segment)" if seg_sig == 0xFFEEFFEE
                           else "(not a heap segment signature)",
                           seg_heap or 0,
                           "-> heap %#x" % hfi["heap"] if seg_heap == hfi["heap"] else ""))
            if enc is not None:
                print("")
                e = decode_heap_entry(dump, hfi["address"], enc)
                if e:
                    print("  Reading those 16 bytes as a _HEAP_ENTRY (XOR key %#x):" % enc)
                    print("    encoded %016x -> decoded %016x" %
                          (e["encoded"], e["decoded"]))
                    print("    size %#x  flags %#x  prevsize %#x  unused %#x  "
                          "header checksum %s" %
                          (e["size"], e["flags"], e["prev_size"], e["unused"],
                           "OK" if e["checksum_ok"] else "BAD"))
                    print("    UnusedBytes (entry+0xf) = %#04x. RtlFreeHeap's busy test is"
                          % e["unused"])
                    print("    `test byte [entry+0xf], 0x3f`; zero there means 'not a")
                    print("    block'. That byte is the one to look at.")
                if r:
                    print("")
                    cb = containing_block(dump, r[2], hfi["address"], enc)
                    if cb is None:
                        print("  Containing-block walk: %#x is not a heap segment header."
                              % r[2])
                    elif "entry" in cb:
                        print("  Containing-block walk from the segment's FirstEntry:")
                        print("    %d blocks stepped, every header checksum valid." %
                              cb["walked"])
                        print("    %#x lies INSIDE the block whose entry is %#x:" %
                              (hfi["address"], cb["entry"]))
                        print("      %#x .. %#x, %d bytes, flags %#x (BUSY=%d), unused %#x"
                              % (cb["entry"], cb["end"], cb["bytes"], cb["flags"],
                                 cb["busy"], cb["unused"]))
                        print("      user data %#x .. %#x (%d bytes); the reported address"
                              % (cb["user"], cb["user"] + cb["usable"], cb["usable"]))
                        print("      is +%#x into it." % cb["offset"])
                        print("")
                        if cb["busy"]:
                            print("    So the surrounding heap metadata is INTACT and this")
                            print("    address is interior to a live block. That is the")
                            print("    signature of a free() of a pointer that was never a")
                            print("    block base - not of trampled metadata.")
                        else:
                            print("    The containing block is not busy.")
                    else:
                        print("  Containing-block walk stopped after %d blocks at %#x: %s."
                              % (cb["walked"], cb["broke_at"], cb["why"]))
                        print("    A chain that breaks is itself evidence - it is what")
                        print("    overwritten metadata looks like.")
            print("")
            print(hexdump(dump, hfi["address"] - 0x60, 0xC0))

    # ------------------------------------------------------------- context
    tid = args.thread if args.thread is not None else exc["tid"]
    th = next((t for t in threads if t["tid"] == tid), None)
    if th is None:
        print("\n  thread %#x not in the thread list" % tid)
        return
    ctx_sz, ctx_rva = (exc["ctx"] if tid == exc["tid"] else th["ctx"])
    regs = parse_context(dump, ctx_sz, ctx_rva)

    rule("THREAD %d CONTEXT" % tid)
    print("  teb %#x, stack %#x .. %#x (%d bytes committed), suspend %d, priority %d" %
          (th["teb"], th["stack"][0], th["stack"][0] + th["stack"][1],
           th["stack"][1], th["suspend"], th["priority"]))
    print("  ContextFlags %#x  EFlags %#x  MxCsr %#x  cs %#x ss %#x" %
          (regs["ContextFlags"], regs["EFlags"], regs["MxCsr"],
           regs["SegCs"], regs["SegSs"]))
    for i in range(0, 16, 2):
        print("  %-4s %#018x    %-4s %#018x" %
              (GPR[i], regs[GPR[i]], GPR[i + 1], regs[GPR[i + 1]]))
    print("  Rip  %s" % syms.describe(regs["Rip"]))

    # -------------------------------------------------------------- unwind
    rule("THREAD %d UNWIND  (.pdata / UNWIND_INFO, a real unwind)" % tid)
    uw = Unwinder(dump, syms)
    frames = uw.walk(regs, args.frames)
    for i, fr in enumerate(frames):
        print("  %02d  %s%s" % (i, syms.describe(fr["rip"]),
                                "   [leaf/no unwind data]" if fr["kind"] == "leaf" else ""))
    for n in uw.notes:
        print("  note: %s" % n)

    if exc["code"] == 0xC0000374 and exc["nparam"] >= 1:
        hfi = heap_failure_info(dump, exc["info"][0])
        if hfi:
            live = [t for t in hfi["trace"] if t]
            got = set(fr["rip"] for fr in frames)
            hit = sum(1 for t in live if t in got)
            print("")
            print("  [%s] positive control: %d of %d frames from the allocator's own"
                  % ("PASS" if hit >= max(3, len(live) // 2) else "WEAK", hit, len(live)))
            print("        captured stack also appear in this unwind. They are two")
            print("        independent readings of the same stack; a low count means the")
            print("        unwinder is wrong, because the captured one cannot be.")

    # ---------------------------------------------------------------- scan
    if args.scan:
        rule("THREAD %d STACK SCAN  (heuristic - stale slots look exactly like frames)"
             % tid)
        top = th["stack"][0] + th["stack"][1]
        lo = regs["Rsp"]
        n = min(args.scan, top - lo)
        raw = dump.read(lo, n) or b""
        unwound = set(fr["rip"] for fr in frames)
        hits = 0
        for off in range(0, len(raw) - 8, 8):
            v = struct.unpack_from("<Q", raw, off)[0]
            m = syms.module_for(v)
            if m is None:
                continue
            if syms.function(v) is None:
                continue
            mark = " <- also in the unwind" if v in unwound else ""
            print("  %#018x  %s%s" % (lo + off, syms.describe(v), mark))
            hits += 1
        print("")
        print("  %d code-looking values in %d bytes of stack. %d of them are frames the"
              % (hits, len(raw), sum(1 for off in range(0, len(raw) - 8, 8)
                                     if struct.unpack_from("<Q", raw, off)[0] in unwound)))
        print("  unwinder also produced; the rest are leads at best. A value on the stack")
        print("  is not a frame - callee-saved copies, spilled function pointers and")
        print("  return addresses from calls that already returned all look identical.")

    # --------------------------------------------------------------- heaps
    rule("HEAPS  (PEB ProcessHeaps)")
    hs = heap_summary(dump, th["teb"])
    if hs is None:
        print("  could not read the PEB heap list.")
    else:
        print("  PEB %#x, default heap %#x, %d heaps" %
              (hs["peb"], hs["default"], hs["count"]))
        print("")
        print("  %-18s %-12s %-10s %-10s %s" %
              ("base", "signature", "Flags", "ForceFlags", "note"))
        for h in hs["heaps"]:
            notes = []
            if h["signature"] == 0xFFEEFFEE:
                notes.append("NT heap")
            elif h["signature"] == 0xDDEEDDEE:
                notes.append("segment heap")
            else:
                notes.append("unrecognised signature")
            if h["base"] == hs["default"]:
                notes.append("process default")
            if exc["code"] == 0xC0000374 and exc["nparam"] >= 1:
                f = heap_failure_info(dump, exc["info"][0])
                if f and h["base"] == f["heap"]:
                    notes.append("*** THE FAILING HEAP ***")
            print("  %#018x %#-12x %#-10x %#-10x %s" %
                  (h["base"], h["signature"] or 0, h["flags"] or 0,
                   h["force_flags"] or 0, ", ".join(notes)))
        any_dbg = any((h["flags"] or 0) & 0x40000000 or (h["force_flags"] or 0)
                      for h in hs["heaps"])
        print("")
        print("  ForceFlags is 0 on %d of %d heaps." %
              (sum(1 for h in hs["heaps"] if not h["force_flags"]), len(hs["heaps"])))
        print("  That is this dump's own answer to 'was page heap on?': %s." %
              ("something is set - check it" if any_dbg else "no, it was off"))
        print("")
        print("  A full `!heap -s` is NOT attempted. Per-heap committed/reserved totals")
        print("  live in _HEAP fields whose offsets move between Windows builds, and")
        print("  there are no symbols on this machine to pin them. What is printed above")
        print("  is only what self-verifies: the segment signature at +0x10 reads")
        print("  FFEEFFEE, which is the check that says the layout guess was right.")

    # ------------------------------------------------------- pdata crosscheck
    rule("CROSS-CHECK  (in-dump .pdata vs the image on disk)")
    if image is None:
        print("  no image module to check.")
    else:
        in_dump = syms.pdata(image)
        print("  %s in-dump .pdata: %d RUNTIME_FUNCTION entries" %
              (image.name, len(in_dump)))
        disk = os.path.join("client-patched", image.name)
        if not os.path.exists(disk):
            print("  %s not on disk from this working directory - skipped." % disk)
            print("  (run from the repo root; see the note at the top of this file)")
        else:
            try:
                data, base, sections = pdata_lookup.load(disk)
                sec = next((s for s in sections if s[0] == ".pdata"), None)
                if sec is None:
                    print("  on-disk image has no .pdata")
                else:
                    _, _va, vsize, raw, rawsize = sec
                    n = min(vsize, rawsize) // 12
                    ondisk = []
                    for i in range(n):
                        b, e, u = struct.unpack_from("<III", data, raw + i * 12)
                        if b or e:
                            ondisk.append((b, e, u))
                    print("  %s on-disk .pdata SECTION: %d entries, image base %#x" %
                          (image.name, len(ondisk), base))
                    live = set(in_dump)
                    missing = [x for x in ondisk if x not in live]
                    extra = sorted(set(in_dump) - set(ondisk))
                    print("  [%s] every on-disk entry is present, byte-identical, in the"
                          % ("PASS" if not missing else "FAIL"))
                    print("         table the running image actually used (%d missing)"
                          % len(missing))
                    if extra:
                        print("  the live table has %d entries the file does not, spanning"
                              % len(extra))
                        print("  RVA %#x .. %#x." % (extra[0][0], extra[-1][0]))
                        print("")
                        print("  That difference is NOT a mismatch, and the reason matters")
                        print("  for every static pass in this repo. This image is")
                        print("  Themida-packed. `pdata_lookup.load` finds the table by")
                        print("  SECTION NAME (.pdata) - but the PE's exception DATA")
                        print("  DIRECTORY, which is what the CPU and RtlLookupFunctionEntry")
                        print("  use, points somewhere else entirely, into a section with")
                        print("  SizeOfRawData 0 that only exists once the packer has")
                        print("  unpacked it. So on disk the directory target is empty and")
                        print("  the section is the only readable table; in memory the")
                        print("  directory target is a SUPERSET - the original functions")
                        print("  plus the packer's own. Reading the section on disk is")
                        print("  therefore correct for client code and blind to .themida,")
                        print("  which is exactly the blind spot research/ already names.")
            except Exception as e:                      # noqa: BLE001
                print("  could not read %s: %s" % (disk, e))

    # ------------------------------------------------------------- listings
    if args.modules:
        rule("MODULES")
        for m in sorted(modules, key=lambda x: x.base):
            print("  %#018x %#010x  %s" % (m.base, m.size, m.path))
    if args.threads:
        rule("THREADS")
        print("  Rip here comes from each thread's own ThreadList context, which for the")
        print("  faulting thread is the dump writer's capture (it is inside NtGetContext"
              ")")
        print("  and NOT the fault site. The EXCEPTION section above has the real one.")
        print("")
        for t in threads:
            c = parse_context(dump, t["ctx"][0], t["ctx"][1])
            rip = c["Rip"] if c else 0
            print("  tid %-8d teb %#014x  %s" % (t["tid"], t["teb"], syms.describe(rip)))

    print("")
    print("NOTE  Page heap was off for this process, so there are no allocation or free")
    print("      stacks anywhere in this file. With page heap on, every block would carry")
    print("      the stack that allocated it and, for a freed block, the stack that freed")
    print("      it - which turns 'something corrupted this' into 'this call did'. Its")
    print("      absence here is a property of the configuration and says nothing about")
    print("      the client.")


if __name__ == "__main__":
    main()
