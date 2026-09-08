"""Enumerate EVERY instance of the heap writer's damage shape in a full-memory dump.

Every previous pass looked at *the one object that killed the run*. This one asks how many
objects a session damages, where inside their allocation the damage sits, and whether the
answer clusters. It is an enumeration, not a sample, and every detector is required to find
a known positive control before its zeros mean anything.

    python tools/damagescan.py dumps\\maplecw-crash-372984-c0000005-1.dmp
    python tools/damagescan.py <dmp> --full          # add the whole-image sweep
    python tools/damagescan.py <dmp> --json out.json

Run it with the repo as the working directory - `CLAUDE.md`, "the scratchpad shadows the
real tools". The `dumpwalk` import below is pinned to `tools/` for the same reason.

THE DAMAGE SHAPE, as three hand-found victims define it
-------------------------------------------------------

  dump 372984   node 0x3a2f9a78, a 0x20 slot   `_Right` at body+0x10 reads
                0xffffffff301bad30 while `_Left` at body+0 holds 0x301bad30 intact
  dump 419988   object 0x3b69a4a8, a 0x40 slot  vtable pointer at body+0 reads
                0x143406c02; the real vtable is 0x143406c00
  earlier dumps free-block headers at body-8   0x20 became 0x0000000100000020

So: a 32-bit quantity inside an 8-byte-aligned field, moved by a small signed amount. Two of
the three moved the HIGH dword (+1, -1); one moved the LOW dword (+2).

THE FOUR DETECTORS, AND WHAT EACH CANNOT SEE
--------------------------------------------

D1  header        the slot's own 8-byte size header at body-8 is not the bucket's slot size.
                  This is `tools/poolchain.py`'s test, widened from "high dword non-zero" to
                  "not equal to the slot size" the way crates/grap-stub/src/poolsentry.rs
                  already does.
                  BLIND TO: everything in a payload. A header is 8 of every 0x28/0x48 bytes.

D2  high-dword    an 8-aligned qword whose value is NOT a committed address but whose LOW
                  DWORD IS a pointer. The high dword is recorded and histogrammed - not
                  filtered against a believed list - so the value family can come back
                  wrong.

                  "is a pointer" is graded, because the loosest reading is useless: 296 MB
                  of the low 4 GB is committed private memory, so roughly one random dword
                  in fourteen lands in it and a UTF-16 string sprays those. Three tiers,
                  all three counted, each against its own control:

                    T0  committed                      - the loose reading, reported only
                                                          to show how useless it is
                    T1  committed, 8-aligned, PRIVATE   - pointer-shaped
                    T2  an ENUMERATED POOL SLOT BODY    - exact; 174 528 addresses out of
                                                          2^32, so noise cannot reach it

                  BLIND TO: (a) damage to a field that was not a pointer, which is exactly
                  what the header case is, so D1 and D2 do not overlap; (b) damage that
                  happens to land on a still-valid address - a heap pointer in
                  0x40000000..0x45dab000 with its high dword set to 1 becomes an image
                  address and is silently legal, though the client's heap sits below
                  0x40000000 so that window is narrow; (c) a low-dword change, which is
                  D3's job; (d) at T2, damage to a pointer that aimed at something the pool
                  did not allocate.

D3  misaligned    an 8-aligned qword that is not itself 8-aligned as a value, but which
                  becomes a VTABLE POINTER when moved by a small delta: the aligned target
                  reads four consecutive qwords that all land in the image's .text. This is
                  exactly how the 0x40 victim was caught by hand.
                  BLIND TO: a low-dword change on any pointer that is not a vtable pointer.
                  There is no way to tell a damaged data pointer from a live one by looking
                  at it, so this detector is structurally limited to objects whose first
                  field is a vtable - and it is honest about that rather than guessing.

D4  twin         two 8-byte fields inside ONE allocation whose low dwords are equal and
                 whose high dwords are not, with at least one of the two a valid pointer.
                 The 0x20 victim's _Left/_Right.
                 BLIND TO: any object that does not hold the same pointer twice. Most do
                 not. Its value is that it needs no notion of "valid address" at all, so it
                 is the one detector that does not share D2's blind spot (b).

CONTROLS
--------

Every detector prints the population it did NOT flag, measured by the same code path:

  D1  slots enumerated whose header is exactly right
  D2  8-aligned qwords whose high dword is zero and whose low dword is committed - i.e.
      ordinary 32-bit-range pointers, the population a false positive would come from
  D3  8-aligned qwords that ARE aligned vtable pointers by the same four-qword test, and
      separately the unaligned qwords whose aligned neighbour is committed but is NOT
      vtable-shaped - the near-misses the filter rejects
  D4  fields inside one allocation holding the IDENTICAL pointer twice

and `--verify` asserts that the three hand-found victims are among the hits in their own
dumps. A detector that cannot find its positive control reports nothing worth reading.
"""

import argparse
import bisect
import json
import os
import struct
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import dumpwalk as dw  # noqa: E402

# The pool context, from `lea rbp,[rip+0x393b366]` at 0x14019b533 in the client's free.
CTX = 0x143AD68A0
BUCKETS = [(0x10, 64), (0x20, 32), (0x40, 16), (0x80, 8)]

# The three hand-found victims, used as positive controls.
KNOWN = {
    "maplecw-crash-372984-c0000005-1.dmp": [
        (0x3A2F9A88, "D2", "_Right high dword -> 0xffffffff"),
        (0x3A2F9A78, "D4", "_Left/_Right twin"),
    ],
    "maplecw-crash-419988-c0000005-1.dmp": [
        (0x3B69A4A8, "D3", "vtable pointer +2"),
    ],
}


class Space(object):
    """Committed-address answers, fast enough to ask 100 million times.

    A page-granular bitmap over the low 4 GB, because every pool address and every damaged
    low dword lives there, plus a bisect over the full range table for anything above.
    The bitmap is built from the dump's own Memory64List, so "committed" here means
    "present in this dump", which is the only sense that can be checked.
    """

    def __init__(self, dump):
        dump._build_memory()
        self.dump = dump
        self.ranges = dump._mem
        self.starts = dump._mem_starts
        self.low = bytearray(1 << 20)  # 4 GB / 4 KB pages
        for start, size, _off in self.ranges:
            if start >= 1 << 32:
                continue
            end = min(start + size, 1 << 32)
            p0, p1 = start >> 12, (end + 0xFFF) >> 12
            self.low[p0:p1] = b"\1" * (p1 - p0)
        # MEM_PRIVATE only, from MemoryInfoList. Mapped files (465 MB below 4 GB in the
        # 372984 dump) are where most of the string data lives, and excluding them is what
        # makes tier 1 mean anything.
        self.priv = bytearray(1 << 20)
        dump.region(0)
        for r in (dump._regions or []):
            base, size, _alloc, _ap, state, _prot, mtype = r
            if base >= 1 << 32 or state != 0x1000 or mtype != 0x20000:
                continue
            end = min(base + size, 1 << 32)
            p0, p1 = base >> 12, (end + 0xFFF) >> 12
            self.priv[p0:p1] = b"\1" * (p1 - p0)

    def committed(self, va):
        if va < (1 << 32):
            return bool(self.low[va >> 12])
        i = bisect.bisect_right(self.starts, va) - 1
        if i < 0:
            return False
        s, sz, _ = self.ranges[i]
        return s <= va < s + sz

    def private(self, va):
        return va < (1 << 32) and bool(self.priv[va >> 12])


class Pages(object):
    """A 4 KB page cache over MiniDump.read, so a vtable read twice costs one seek."""

    def __init__(self, dump, cap=200000):
        self.dump, self.cap, self.d = dump, cap, {}

    def qwords(self, va, n):
        out = []
        for k in range(n):
            a = va + k * 8
            pg = a & ~0xFFF
            b = self.d.get(pg)
            if b is None:
                b = self.dump.read(pg, 0x1000)
                if b is None or len(b) < 0x1000:
                    return None
                if len(self.d) > self.cap:
                    self.d.clear()
                self.d[pg] = b
            o = a - pg
            if o + 8 > len(b):
                return None
            out.append(struct.unpack_from("<Q", b, o)[0])
        return out


# ------------------------------------------------------------------ the pool

def enumerate_pool(dump):
    """Every slot the client's pool allocator has carved, with its payload bytes.

    Same walk as tools/poolchain.py - the chunk-list head per bucket, each chunk checked
    against the allocator's own size identity before any of its slots are counted. Nothing
    here is a scan; a slot exists because the allocator's own chunk list says so.
    """
    slots = {}          # body -> dict
    per_bucket = []
    for i, (slot, count) in enumerate(BUCKETS):
        stride = slot + 8
        want = count * stride + 8
        head = dump.u64(CTX + i * 8 + 0x88)
        freehead = dump.u64(CTX + i * 8 + 0x68)
        carved = dump.u32(CTX + i * 4 + 0x04)
        served = dump.u32(CTX + i * 4 + 0x14)
        chunks = bad = 0
        seen = set()
        cur = head
        while cur and cur not in seen:
            seen.add(cur)
            base = cur - 0x10
            blob = dump.read(base - 8, want + 8)
            if blob is None or len(blob) < want + 8:
                bad += 1
                nxt = dump.u64(base)
                if not nxt or nxt == cur:
                    break
                cur = nxt
                continue
            declared = struct.unpack_from("<Q", blob, 0)[0]
            if declared != want:
                bad += 1
                nxt = struct.unpack_from("<Q", blob, 8)[0]
                if not nxt or nxt == cur:
                    break
                cur = nxt
                continue
            chunks += 1
            for k in range(count):
                o = 8 + 8 + k * stride           # +8 for the chunk's own size header
                hdr = struct.unpack_from("<Q", blob, o)[0]
                body = base + 8 + k * stride + 8
                slots[body] = dict(bucket=i, slot=slot, header=hdr, chunk=base,
                                   index=k, payload=blob[o + 8:o + 8 + slot])
            cur = struct.unpack_from("<Q", blob, 8)[0]
        per_bucket.append(dict(bucket=i, slot=slot, per_chunk=count, chunks=chunks,
                               bad_chunks=bad, carved=carved, served=served,
                               freehead=freehead))
    # Free list: [body] is the next free body. Walked from the cached payloads, so it costs
    # no further reads. A body not in `slots` ends the walk and is reported.
    free = set()
    broke = []
    for i, (slot, count) in enumerate(BUCKETS):
        cur = per_bucket[i]["freehead"]
        n = 0
        while cur:
            s = slots.get(cur)
            if s is None:
                broke.append((i, cur))
                break
            if cur in free:
                broke.append((i, cur))
                break
            free.add(cur)
            n += 1
            cur = struct.unpack_from("<Q", s["payload"], 0)[0]
            if n > 4000000:
                break
        per_bucket[i]["free_walked"] = n
    return slots, per_bucket, free, broke


def carve_scan(dump, block=1 << 22):
    """Find EVERY pool chunk in the dump by its shape, not by following a chunk list.

    This exists because the chunk-list walk above is not an enumeration of the pool - it is
    an enumeration of ONE pool context's list. The `0x40` victim `0x3b69a4a8` from dump
    419988 is the first slot of a chunk whose size header at `body-0x18` is exactly `0x488`
    and whose slot headers are all `0x40`, and it is **not on the chunk list at
    0x143AD68A0+0x88**. Following the list reported it as not existing. That is the
    "searching a known list" failure `CLAUDE.md` names, and the fix is to enumerate.

    The shape is self-verifying and cannot occur by accident: the big allocator writes
    `count*(slot+8)+8` as an 8-byte header at `chunkbase-8`, and the carve loop then writes
    the bucket's slot size into `count` headers at a fixed stride. Requiring all but two of
    those to be exactly right is 6 to 62 identical qwords at an exact stride. The two
    allowed misses are what leaves room for D1 to find a damaged header inside an otherwise
    valid chunk - requiring all of them would make the chunk containing the damage
    invisible, which is the same mistake in a new place.
    """
    pats = {}
    for i, (slot, count) in enumerate(BUCKETS):
        want = count * (slot + 8) + 8
        pats[struct.pack("<Q", want)] = (i, slot, count, want)
    chunks = {}
    scanned = 0
    for start, size, off in dump._mem:
        if start & 7:
            continue
        pos = 0
        while pos < size:
            n = min(block, size - pos) & ~7
            if n < 8:
                break
            buf = dump._at(off + pos, n)
            base_va = start + pos
            scanned += n
            for pat, (i, slot, count, want) in pats.items():
                j = buf.find(pat)
                while j != -1:
                    if (j & 7) == 0:
                        hdr_va = base_va + j
                        base = hdr_va + 8
                        if base not in chunks:
                            blob = dump.read(base, want)
                            if blob and len(blob) >= want:
                                stride = slot + 8
                                good = 0
                                hdrs = []
                                for k in range(count):
                                    h = struct.unpack_from("<Q", blob, 8 + k * stride)[0]
                                    hdrs.append(h)
                                    if h == slot:
                                        good += 1
                                if good >= count - 2 and good >= 4:
                                    chunks[base] = (i, slot, count, want, blob, hdrs)
                    j = buf.find(pat, j + 1)
            pos += n
    return chunks, scanned


def slots_from_chunks(chunks):
    slots = {}
    for base, (i, slot, count, want, blob, hdrs) in chunks.items():
        stride = slot + 8
        for k in range(count):
            o = 8 + k * stride
            body = base + 8 + k * stride + 8
            slots[body] = dict(bucket=i, slot=slot, header=hdrs[k], chunk=base,
                               index=k, payload=blob[o + 8:o + 8 + slot])
    return slots


# ------------------------------------------------------------------ detectors

def vtable_like(pages, t, text_lo, text_hi, n=4):
    q = pages.qwords(t, n)
    if q is None:
        return False
    return all(text_lo <= x < text_hi for x in q)


def scan_payloads(slots, free, space, pages, text_lo, text_hi):
    """D2, D3, D4 over every enumerated slot payload. Exhaustive, all four buckets."""
    res = dict(
        d2=[], d2_hi=Counter(), d2_hi_t1=Counter(), d2_t0=0, d2_t1=0,
        c2_t0=0, c2_t1=0, c2_t2=0, c2_t3=0, d2_ptr64=0,
        d3=[], d3_delta=Counter(), d3_control=0, d3_nearmiss=0, d3_unaligned=0,
        d4=[], d4_control=0,
        qwords=0,
    )
    committed, private = space.committed, space.private

    # Pass 1: every value that is used SOMEWHERE as an intact pointer. This is the control
    # the 372984 write-up ran by hand - "0x00000000301bad30 occurs 13 times, the corrupted
    # form twice" - made into a predicate. A candidate whose repaired value is referenced
    # nowhere else is far more likely to be a 64-bit datum whose low dword coincidentally
    # lands on a slot body; with 393 408 slots enumerated that coincidence is no longer
    # negligible, and it produced 17 of 18 tier-2 hits before this pass existed.
    refs = Counter()
    for body, s in slots.items():
        pl = s["payload"]
        for (v,) in struct.iter_unpack("<Q", pl):
            if v >> 32:
                continue
            if v >= 0x10000 and not (v & 7) and private(v):
                refs[v] += 1
    for body, s in slots.items():
        pl = s["payload"]
        nq = len(pl) // 8
        vals = struct.unpack_from("<%dQ" % nq, pl, 0)
        res["qwords"] += nq
        bylo = defaultdict(list)
        for k, v in enumerate(vals):
            va = body + k * 8
            hi, lo = v >> 32, v & 0xFFFFFFFF
            bylo[lo].append((k, hi, v))

            # ---- D2: high dword damage on a pointer field, three tiers
            if hi == 0:
                if lo >= 0x10000 and committed(lo):
                    res["c2_t0"] += 1
                    if not (lo & 7) and private(lo):
                        res["c2_t1"] += 1
                        if lo in slots:
                            res["c2_t2"] += 1
                            if refs[lo] > 1:
                                res["c2_t3"] += 1
            else:
                if committed(v):
                    res["d2_ptr64"] += 1
                elif lo >= 0x10000 and committed(lo):
                    res["d2_t0"] += 1
                    if not (lo & 7) and private(lo):
                        res["d2_t1"] += 1
                        res["d2_hi_t1"][hi] += 1
                        if lo in slots:
                            res["d2_hi"][hi] += 1
                            res["d2"].append(
                                dict(va=va, body=body, off=k * 8, val=v,
                                     bucket=s["bucket"], free=body in free,
                                     hi=hi, lo=lo, target_slot=slots[lo]["slot"],
                                     refs=refs[lo]))

            # ---- D3: low dword damage that un-aligns a vtable pointer
            if v & 7:
                res["d3_unaligned"] += 1
                hit = None
                near = False
                for d in (2, 1, 4, -1, -2, -4, 3, -3, 6, -6):
                    t = v - d
                    if t & 7 or t < 0x10000 or not committed(t):
                        continue
                    near = True
                    if vtable_like(pages, t, text_lo, text_hi):
                        hit = (d, t)
                        break
                if hit:
                    res["d3_delta"][hit[0]] += 1
                    res["d3"].append(dict(va=va, body=body, off=k * 8, val=v,
                                          bucket=s["bucket"], free=body in free,
                                          delta=hit[0], target=hit[1]))
                elif near:
                    res["d3_nearmiss"] += 1
            elif v >= 0x10000 and committed(v) and \
                    vtable_like(pages, v, text_lo, text_hi):
                res["d3_control"] += 1

        # ---- D4: the same low dword twice with different high dwords
        for lo, group in bylo.items():
            if len(group) < 2:
                continue
            his = set(g[1] for g in group)
            good = lo >= 0x10000 and not (lo & 7) and private(lo)
            if len(his) == 1:
                if 0 in his and good:
                    res["d4_control"] += 1
                continue
            if 0 in his and good:
                res["d4"].append(dict(body=body, bucket=s["bucket"], lo=lo,
                                      free=body in free,
                                      in_pool=lo in slots, refs=refs[lo],
                                      fields=[(g[0] * 8, g[2]) for g in group]))
    return res


# ------------------------------------------------ the whole-image sweep (--full)

def sweep_full(dump, space, families, slots, block=1 << 22):
    """Every 8-aligned qword in committed memory whose high dword is in `families`.

    `families` comes from the exhaustive payload sweep above, not from a belief - that is
    the order CLAUDE.md asks for: enumerate on the cheap exhaustive instrument, filter on
    the expensive one, and say so. The blind spot it buys is stated in the report: a high
    dword outside the enumerated set is not looked for here.

    A second, exhaustive-over-all-high-dwords pass runs on a 1-in-32 sample of blocks and
    reports what the targeted scan would have missed. If that number is not zero the
    targeted scan is under-counting and says so.
    """
    pats = {}
    for hi in families:
        pats[struct.pack("<I", hi)] = hi
    hits = []
    pooled = []
    scanned = 0
    sample_extra = Counter()
    sample_bytes = 0
    sample_clean = 0
    sample_family = 0
    blk_index = 0
    for start, size, off in dump._mem:
        if start & 7:
            continue
        pos = 0
        while pos < size:
            n = min(block, size - pos)
            n &= ~7
            if n < 8:
                break
            buf = dump._at(off + pos, n)
            base = start + pos
            scanned += n
            for pat, hi in pats.items():
                i = buf.find(pat)
                while i != -1:
                    if (i & 7) == 4:
                        v = struct.unpack_from("<Q", buf, i - 4)[0]
                        lo = v & 0xFFFFFFFF
                        if lo >= 0x10000 and space.committed(lo) \
                                and not space.committed(v):
                            hits.append((base + i - 4, v))
                            if lo in slots:
                                # The only whole-image reading that HAS a control: the
                                # repaired value is one of ~350 000 enumerated slot
                                # bodies, so a coincidence costs about 1e-4. This finds a
                                # damaged pointer-to-a-pool-object held ANYWHERE - an NT
                                # heap block, a big-allocator buffer, a global, a stack
                                # slot - which is exactly what the payload sweep cannot
                                # see.
                                pooled.append((base + i - 4, v))
                    i = buf.find(pat, i + 1)
            if (blk_index & 31) == 0:
                sample_bytes += n
                for (v,) in struct.iter_unpack("<Q", buf):
                    hi, lo = v >> 32, v & 0xFFFFFFFF
                    if lo < 0x10000:
                        continue
                    if hi == 0:
                        if space.committed(lo):
                            sample_clean += 1
                    elif not space.committed(v) and space.committed(lo):
                        if hi in families:
                            sample_family += 1
                        else:
                            sample_extra[hi] += 1
            blk_index += 1
            pos += n
    return dict(hits=hits, pooled=pooled, scanned=scanned, sample_bytes=sample_bytes,
                sample_clean=sample_clean, sample_family=sample_family,
                sample_extra=sample_extra)


# --------------------------------------------------------------------- report

def image_text(dump, modules):
    img = next((m for m in modules if m.name.lower() == "maplestory.exe"), None)
    if img is None:
        return None, 0, 0
    hdr = dump.read(img.base, 0x1000)
    pe = struct.unpack_from("<I", hdr, 0x3C)[0]
    nsec = struct.unpack_from("<H", hdr, pe + 6)[0]
    optsz = struct.unpack_from("<H", hdr, pe + 20)[0]
    so = pe + 24 + optsz
    for i in range(nsec):
        o = so + i * 40
        if hdr[o:o + 8].rstrip(b"\0") == b".text":
            vsz, va = struct.unpack_from("<II", hdr, o + 8)
            return img, img.base + va, img.base + va + vsz
    return img, 0, 0


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("dump")
    ap.add_argument("--full", action="store_true",
                    help="also sweep every committed byte, not just pool payloads")
    ap.add_argument("--json", default=None)
    ap.add_argument("--verify", action="store_true",
                    help="exit non-zero unless this dump's known victims are found")
    ap.add_argument("--quiet", action="store_true")
    args = ap.parse_args()

    name = os.path.basename(args.dump)
    dump = dw.MiniDump(args.dump)
    modules = dw.parse_modules(dump)
    img, text_lo, text_hi = image_text(dump, modules)
    space = Space(dump)
    pages = Pages(dump)
    misc = dw.parse_misc(dump) or {}
    exc = dw.parse_exception(dump)
    alive = (dump.timestamp - misc["create_time"]) if misc.get("create_time") else None

    print("== %s" % name)
    print("   %.1f MB committed, %d ranges, image %#x, .text %#x..%#x"
          % (dump.memory_total() / 1048576.0, len(dump._mem),
             img.base if img else 0, text_lo, text_hi))
    print("   pid %s, alive %s s, exception %s"
          % (misc.get("pid"), alive,
             ("%#x" % exc["code"]) if exc else "none (sentry dump)"))

    list_slots, per_bucket, free, broke = enumerate_pool(dump)
    chunks, cscanned = carve_scan(dump)
    slots = slots_from_chunks(chunks)

    print("")
    print("   POOL ENUMERATION")
    print("     chunk-list walk from the context at %#x, and a whole-dump scan for the"
          % CTX)
    print("     chunk SHAPE. The scan must contain every chunk the walk found - that is")
    print("     its positive control - and anything extra is a chunk the walk cannot see.")
    listed_chunks = set()
    for s in list_slots.values():
        listed_chunks.add(s["chunk"])   # both conventions put the size header at base-8
    scan_bases = set(chunks)
    missing = listed_chunks - scan_bases
    print("     [%s] %d of %d chunk-list chunks are in the shape scan (%d missed)"
          % ("PASS" if not missing else "FAIL", len(listed_chunks) - len(missing),
             len(listed_chunks), len(missing)))
    print("     %.1f MB scanned; shape scan found %d chunks, %d of them NOT on the "
          "chunk list" % (cscanned / 1048576.0, len(chunks), len(scan_bases - listed_chunks)))
    # Are the extras real? Every chunk's first qword is its chunk-list link. If the extras
    # are genuine chunks belonging to OTHER pool contexts, that link lands on another
    # accepted chunk of the same bucket, or is 0 (tail of a list). If they were stale
    # wreckage the links would be arbitrary. This is the shape scan checking itself.
    extras = scan_bases - listed_chunks
    ok = tail = badlink = 0
    for b in extras:
        link = struct.unpack_from("<Q", chunks[b][4], 0)[0]
        if link == 0:
            tail += 1
        elif link - 0x10 in chunks and chunks[link - 0x10][0] == chunks[b][0]:
            ok += 1
        else:
            badlink += 1
    print("     of the %d extra chunks, %d link to another accepted chunk of the SAME "
          "bucket, %d are list tails (link 0), %d link nowhere valid"
          % (len(extras), ok, tail, badlink))
    laid = sorted((b, b + chunks[b][3]) for b in chunks)
    overlaps = sum(1 for i in range(len(laid) - 1) if laid[i][1] > laid[i + 1][0])
    print("     %d of %d accepted chunks overlap another (must be 0)"
          % (overlaps, len(chunks)))
    for sl in slots.values():
        sl["listed"] = sl["chunk"] in listed_chunks
    tot_slots = 0
    d1 = []
    for b in per_bucket:
        i = b["bucket"]
        n = sum(1 for s in slots.values() if s["bucket"] == i)
        nl = sum(1 for s in list_slots.values() if s["bucket"] == i)
        bad = [(body, s["header"]) for body, s in slots.items()
               if s["bucket"] == i and s["header"] != b["slot"]]
        d1 += [(body, h, i) for body, h in bad]
        tot_slots += n
        print("     bucket %d slot %#-5x  scan %6d slots / list %6d slots  (%d chunks "
              "failed the size identity), %6d on the free list, header damage %d"
              % (i, b["slot"], n, nl, b["bad_chunks"], b["free_walked"], len(bad)))
    if broke:
        print("     free-list walk stopped early: %s"
              % ", ".join("bucket %d at %#x" % t for t in broke))
    live = tot_slots - len(free)
    print("     %d slots enumerated, %d known free (from the one context's free lists), "
          "%d live-or-unknown" % (tot_slots, len(free), live))

    res = scan_payloads(slots, free, space, pages, text_lo, text_hi)

    print("")
    print("   D1 header   %d damaged / %d enumerated   control: %d headers exactly right"
          % (len(d1), tot_slots, tot_slots - len(d1)))
    for body, h, i in sorted(d1)[:40]:
        print("        %#x  header %#018x  bucket %d  free=%s"
              % (body - 8, h, i, body in free))

    def ratio(a, b):
        return "1 in %s" % (("%.0f" % (float(b) / a)) if a else "-")

    print("   D2 highdw   %d payload qwords examined, %d legal 64-bit pointers not flagged"
          % (res["qwords"], res["d2_ptr64"]))
    print("        T0 committed             flagged %-6d  control %-7d  %s"
          % (res["d2_t0"], res["c2_t0"], ratio(res["d2_t0"], res["c2_t0"])))
    print("        T1 8-aligned + PRIVATE   flagged %-6d  control %-7d  %s"
          % (res["d2_t1"], res["c2_t1"], ratio(res["d2_t1"], res["c2_t1"])))
    # T4: how many OTHER fields in the whole dump carry the same target with a dirty high
    # dword. Damage is sporadic - the 372984 victim is one dirty reference against ten
    # clean ones. A STRUCTURAL field is not: eleven bucket-1 objects in that same dump each
    # hold 0x38950000 at +0x18 with a different value above it, which is what a packed
    # {u32 id; u32 counter} pair looks like and not what one stray increment looks like.
    # Nothing here is filtered on the value; the rule is about how often the shape repeats.
    tgt = Counter(h["lo"] for h in res["d2"])
    d2t3 = [h for h in res["d2"] if h["refs"] > 1]
    d2t4 = [h for h in d2t3 if tgt[h["lo"]] == 1]
    print("        T2 an enumerated slot    flagged %-6d  control %-7d  %s"
          % (len(res["d2"]), res["c2_t2"], ratio(len(res["d2"]), res["c2_t2"])))
    print("        T3 + referenced elsewhere flagged %-5d  control %-7d  %s"
          % (len(d2t3), res["c2_t3"], ratio(len(d2t3), res["c2_t3"])))
    # The value family, stated last because it is the one thing that must NOT be used to
    # find hits - it is the result of finding them. Across the 37 dumps the high dwords
    # that recur are +1/+2/+3 (all 30 damaged pool headers) and -1/-2/-3 (the tree-node
    # family). Everything else at tier 4 occurs once, at a different offset each time, and
    # is more likely a 64-bit datum whose low half coincides with a slot body.
    FAM = (1, 2, 3, 0xFFFFFFFF, 0xFFFFFFFE, 0xFFFFFFFD)
    infam = [h for h in d2t4 if h["hi"] in FAM]
    print("        T4 + the only dirty reference to that target   flagged %d, %d of them "
          "with a high dword in the +/-1,2,3 family" % (len(d2t4), len(infam)))
    if res["d2_hi_t1"]:
        print("        T1 high-dword histogram (enumerated, not filtered): %s"
              % ", ".join("%#x x%d" % (h, c) for h, c in res["d2_hi_t1"].most_common(10)))
    if res["d2_hi"]:
        print("        T2 high-dword histogram: %s"
              % ", ".join("%#x x%d" % (h, c) for h, c in res["d2_hi"].most_common(10)))
    for h in sorted(res["d2"], key=lambda x: x["va"])[:40]:
        tier = "T2"
        if h["refs"] > 1:
            tier = "T4" if tgt[h["lo"]] == 1 else "T3"
        print("        %s %#x  body %#x +%#x  bucket %d  %#018x -> slot %#x, %d clean "
              "refs, %d dirty  free=%s"
              % (tier, h["va"], h["body"], h["off"], h["bucket"], h["val"],
                 h["target_slot"], h["refs"], tgt[h["lo"]], h["free"]))

    # The same uniqueness rule D2 needs, for the same reason and found the same way: dump
    # 1007028 carries SEVEN 0x40 slots holding the identical 0x143270001 at +0x18, and
    # 0x143270000 is not a vtable start - it is the middle of a long .text pointer array,
    # so the four-qword test passes anywhere inside it. Seven identical hits in a 372 s
    # process cannot be a writer that fires every 180 s. A value that repeats is a
    # constant; damage is sporadic.
    d3val = Counter(h["val"] for h in res["d3"])
    d3u = [h for h in res["d3"] if d3val[h["val"]] == 1]
    print("   D3 vtable   %d flagged, %d of them a value that occurs only once   control: "
          "%d aligned vtable pointers; %d unaligned qwords examined, %d had a committed "
          "aligned neighbour and were rejected"
          % (len(res["d3"]), len(d3u), res["d3_control"], res["d3_unaligned"],
             res["d3_nearmiss"]))
    if res["d3_delta"]:
        print("        deltas that worked (all ten tried): %s"
              % ", ".join("%+d x%d" % (d, c) for d, c in res["d3_delta"].most_common()))
    for h in sorted(res["d3"], key=lambda x: x["va"])[:40]:
        print("        %s %#x  body %#x +%#x  bucket %d  %#x = vtable %#x %+d, value seen "
              "%dx  free=%s"
              % ("UNIQUE" if d3val[h["val"]] == 1 else "repeat", h["va"], h["body"],
                 h["off"], h["bucket"], h["val"], h["target"], h["delta"],
                 d3val[h["val"]], h["free"]))

    d4s = [h for h in res["d4"] if h["in_pool"] and h["refs"] > 1]
    print("   D4 twin     %d flagged, %d of them with the twin pointing at an enumerated "
          "slot that is referenced elsewhere   control: %d identical-pointer pairs in "
          "one object" % (len(res["d4"]), len(d4s), res["d4_control"]))
    for h in sorted(res["d4"], key=lambda x: x["body"])[:40]:
        print("        %s body %#x bucket %d lo %#x in-pool=%s refs=%d  %s  free=%s"
              % ("STRICT" if h in d4s else "loose  ", h["body"], h["bucket"], h["lo"],
                 h["in_pool"], h["refs"] - 1,
                 " ".join("+%#x=%#x" % f for f in h["fields"]), h["free"]))

    # ---- distinct objects, offsets, clustering
    objects = defaultdict(set)
    for body, h, i in d1:
        objects[body].add("D1@-8")
    for h in d2t4:
        objects[h["body"]].add("D2@+%#x" % h["off"])
    for h in d3u:
        objects[h["body"]].add("D3@+%#x" % h["off"])
    for h in d4s:
        for off, v in h["fields"]:
            if (v >> 32) != 0:
                objects[h["body"]].add("D4@+%#x" % off)
    print("")
    onlist = sum(1 for b in objects if b in slots and slots[b]["listed"])
    print("   DISTINCT DAMAGED OBJECTS: %d   (D1, D2 at tier 4, D3, D4 strict)"
          % len(objects))
    print("   %d of them are in a chunk the context's own chunk list reaches - the"
          % onlist)
    print("   authoritative set; %d are in chunk-shaped regions the list does not reach"
          % (len(objects) - onlist))
    offs = Counter()
    for body, tags in sorted(objects.items()):
        print("     %#x  bucket %d  free=%s  on-chunk-list=%s  %s"
              % (body, slots[body]["bucket"] if body in slots else -1,
                 body in free, slots[body]["listed"] if body in slots else "?",
                 ", ".join(sorted(tags))))
        for t in tags:
            offs[t.split("@")[1]] += 1
    if offs:
        print("     offsets within the allocation: %s"
              % ", ".join("%s x%d" % (o, c) for o, c in offs.most_common()))
    ad = sorted(objects)
    if len(ad) > 1:
        print("     address gaps: %s"
              % ", ".join("%#x" % (ad[i + 1] - ad[i]) for i in range(len(ad) - 1)))

    out = dict(dump=name, pid=misc.get("pid"), alive=alive,
               code=exc["code"] if exc else None,
               slots=tot_slots, free=len(free), live=live,
               d1=[(hex(b - 8), hex(h), i) for b, h, i in d1],
               d1_control=tot_slots - len(d1),
               d2=[dict(va=hex(h["va"]), off=h["off"], val=hex(h["val"]),
                        bucket=h["bucket"], free=h["free"], refs=h["refs"],
                        target_slot=h["target_slot"]) for h in res["d2"]],
               d2_t0=res["d2_t0"], d2_t1=res["d2_t1"], d2_t3=len(d2t3),
               d2_t4=[dict(va=hex(h["va"]), off=h["off"], val=hex(h["val"]),
                           bucket=h["bucket"], free=h["free"], refs=h["refs"],
                           hi=hex(h["hi"]), family=h["hi"] in FAM)
                      for h in d2t4],
               d2_family=len(infam),
               c2_t0=res["c2_t0"], c2_t1=res["c2_t1"], c2_t2=res["c2_t2"],
               c2_t3=res["c2_t3"], d4_strict=len(d4s),
               d2_ptr64=res["d2_ptr64"],
               d2_hi={hex(k): v for k, v in res["d2_hi"].items()},
               d2_hi_t1={hex(k): v for k, v in res["d2_hi_t1"].items()},
               d3=[dict(va=hex(h["va"]), off=h["off"], val=hex(h["val"]),
                        target=hex(h["target"]), delta=h["delta"],
                        bucket=h["bucket"], free=h["free"]) for h in res["d3"]],
               d3_unique=len(d3u),
               d3_control=res["d3_control"], d3_nearmiss=res["d3_nearmiss"],
               d3_unaligned=res["d3_unaligned"],
               d3_delta={str(k): v for k, v in res["d3_delta"].items()},
               d4=[dict(body=hex(h["body"]), lo=hex(h["lo"]), bucket=h["bucket"],
                        free=h["free"], in_pool=h["in_pool"], refs=h["refs"],
                        fields=[[f[0], hex(f[1])] for f in h["fields"]])
                   for h in res["d4"]],
               d4_control=res["d4_control"],
               qwords=res["qwords"],
               objects={hex(b): sorted(t) + (["on-chunk-list"]
                                             if b in slots and slots[b]["listed"] else [])
                        for b, t in objects.items()},
               objects_on_list=onlist,
               chunks_scan=len(chunks), chunks_listed=len(listed_chunks),
               chunk_link_ok=ok, chunk_link_tail=tail, chunk_link_bad=badlink,
               chunk_overlaps=overlaps,
               buckets=per_bucket)

    if args.full:
        # Enumerated, not assumed: across the 37 dumps in dumps/, the high dwords that
        # recur on confirmed damage are +1, +2, +3 (30 pool headers) and -1, -2, -3 (the
        # tree-node family, including 0x696fd98 in pid 356516 caught at -1, -2 and -3 in
        # three dumps of one process). The payload sweep above enumerates without a
        # filter; this sweep is the expensive one and uses that result. A high dword
        # outside this set is not looked for HERE, and the sampled control below says how
        # much that costs.
        fam = [1, 2, 3, 0xFFFFFFFF, 0xFFFFFFFE, 0xFFFFFFFD]
        print("")
        print("   FULL SWEEP over every committed byte, high-dword family %s"
              % ", ".join("%#x" % f for f in fam))
        fs = sweep_full(dump, space, fam, slots)
        pool_bodies = set(slots)
        inpool = 0
        for va, v in fs["hits"]:
            b = va & ~0x7
            # attribute to a pool slot if one contains this address
            for cand in (b, b - 8, b - 0x10, b - 0x18, b - 0x20, b - 0x28,
                         b - 0x30, b - 0x38, b - 0x40, b - 0x48, b - 0x50,
                         b - 0x58, b - 0x60, b - 0x68, b - 0x70, b - 0x78):
                if cand in pool_bodies:
                    inpool += 1
                    break
        print("     %.1f MB scanned, %d qwords flagged, %d of them inside an enumerated "
              "pool slot" % (fs["scanned"] / 1048576.0, len(fs["hits"]), inpool))
        print("     sample control (1 block in 32, %.1f MB, exhaustive over ALL high "
              "dwords):" % (fs["sample_bytes"] / 1048576.0))
        print("       %d clean 32-bit pointers, %d in the targeted family, %d outside it"
              % (fs["sample_clean"], fs["sample_family"], sum(fs["sample_extra"].values())))
        if fs["sample_extra"]:
            print("       high dwords the targeted scan does NOT look for: %s"
                  % ", ".join("%#x x%d" % (h, c)
                              for h, c in fs["sample_extra"].most_common(10)))
        print("")
        print("     That raw count is worth nothing on its own and is printed to say so:")
        print("     the sampled control puts ordinary clean 32-bit pointers at %d per "
              "%.1f MB," % (fs["sample_clean"], fs["sample_bytes"] / 1048576.0))
        print("     so the family fires at roughly 1 in %d against them. Outside the pool"
              % max(1, int(round(float(fs["sample_clean"]) / max(1, fs["sample_family"])))))
        print("     there is no allocation map, so a flagged qword there cannot be told")
        print("     from a legal 64-bit datum. The one whole-image reading that CAN be")
        print("     checked is below.")
        print("")
        ptgt = Counter(v & 0xFFFFFFFF for _a, v in fs["pooled"])
        uniq = [(a, v) for a, v in fs["pooled"] if ptgt[v & 0xFFFFFFFF] == 1]
        print("     %d flagged qwords whose REPAIRED value is an enumerated slot body, %d "
              "of them the only dirty reference to that slot:"
              % (len(fs["pooled"]), len(uniq)))
        for va, v in sorted(uniq)[:80]:
            r = dump.region(va)
            where = "pool payload" if (va & ~7) in slots else (
                "IMAGE" if r and r[6] == 0x1000000 else
                ("mapped file" if r and r[6] == 0x40000 else "private"))
            print("       %#x  %#018x  -> slot %#x  [%s]"
                  % (va, v, slots[v & 0xFFFFFFFF]["slot"], where))
        out["full"] = dict(scanned=fs["scanned"], n_hits=len(fs["hits"]),
                           pooled=[[hex(a), hex(v)] for a, v in fs["pooled"]],
                           pooled_unique=[[hex(a), hex(v)] for a, v in uniq],
                           in_pool=inpool,
                           sample_bytes=fs["sample_bytes"],
                           sample_clean=fs["sample_clean"],
                           sample_family=fs["sample_family"],
                           sample_extra={hex(k): v
                                         for k, v in fs["sample_extra"].items()})

    if args.json:
        with open(args.json, "w") as f:
            json.dump(out, f, indent=1)

    rc = 0
    if args.verify:
        want = KNOWN.get(name, [])
        print("")
        print("   POSITIVE CONTROLS")
        if not want:
            print("     none recorded for this dump - --verify says nothing here.")
        found = {
            "D1": set(b - 8 for b, _h, _i in d1),
            "D2": set(h["va"] for h in res["d2"]),
            "D3": set(h["va"] for h in res["d3"]),
            "D4": set(h["body"] for h in res["d4"]),
        }
        for va, det, why in want:
            ok = va in found[det]
            print("     [%s] %s must find %#x (%s)"
                  % ("PASS" if ok else "FAIL", det, va, why))
            if not ok:
                rc = 1
    return rc


if __name__ == "__main__":
    sys.exit(main() or 0)
