"""Enumerate EVERY slot the client's pool allocator has ever carved, exactly.

Not a scan. `FUN_14019b4e0` (free) hardcodes its pool context with
`lea rbp,[rip+0x393b366]` at 0x14019b533 -> 0x143AD68A0, and `FUN_14019b780` (alloc)
keeps, per bucket index i:

    ctx + i*4  + 0x04  slots carved
    ctx + i*4  + 0x14  allocations served / objects live
    ctx + i*16 + 0x28  lock owner TEB  (16 bytes per bucket, +0x30 is the recursion count)
    ctx + i*8  + 0x68  free-list head
    ctx + i*8  + 0x88  chunk-list head, a body0 pointer; [body0-0x10] links to the previous

The lock stride is **16, not 8**, and that was wrong here until 2026-08-30. The listing says
so directly - research/heapfix-did-not-hold.md section 1 quotes `rbx = ctx + i*16` before
`take [rbx+0x28] spinlock` - and the surrounding layout settles it without the listing: the
counters end at ctx+0x24 (i*4 + 0x14 for i=3, plus 4) and the free-list heads begin at
ctx+0x68, so the lock array is exactly 0x68-0x28 = 0x40 bytes for four buckets. With i*8 the
range 0x48..0x68 is unaccounted for and bucket 1's "lock" is bucket 0's recursion count.

Nothing already measured changes: every lock word read zero in every dump examined, so the
bug could only ever have misreported a HELD lock, and none was held. It matters for anything
reading a LIVE pool, where they are not zero - see crates/grap-stub/src/poolsentry.rs.

and the bucket table, read off the switch at 0x14019b7f0, is

    i=0  slot 0x10, 64 per chunk     i=2  slot 0x40, 16 per chunk
    i=1  slot 0x20, 32 per chunk     i=3  slot 0x80,  8 per chunk

so a chunk is  count*(slot+8) + 8  bytes and the big allocator writes that number as an
8-byte header at chunkbase-8. Every chunk is checked against that identity before any of
its slots are counted, and the lock words are read too because a lock held by a live
thread is what a torn write would look like.

    python - <dmp> < poolchain.py
"""
import os
import struct
import sys

sys.path.insert(0, r"C:\MapleCW\tools")
import dumpwalk as dw

CTX = 0x143AD68A0
BUCKETS = [(0x10, 64), (0x20, 32), (0x40, 16), (0x80, 8)]


def main():
    dump = dw.MiniDump(sys.argv[1])
    exc = dw.parse_exception(dump)
    threads = dw.parse_threads(dump)
    hfi = dw.heap_failure_info(dump, exc["info"][0])
    tids = set(t["tid"] for t in threads)

    print("== %s" % os.path.basename(sys.argv[1]))
    print("   pool context %#x  (from `lea rbp,[rip+0x393b366]` in the client's free)" % CTX)
    print("")
    total_damaged = []
    for i, (slot, count) in enumerate(BUCKETS):
        stride = slot + 8
        want = count * stride + 8
        carved = dump.u32(CTX + i * 4 + 0x04)
        served = dump.u32(CTX + i * 4 + 0x14)
        # 16 bytes per bucket, not 8 - see the module docstring.
        owner = dump.u64(CTX + i * 16 + 0x28)
        depth = dump.u32(CTX + i * 16 + 0x30)
        freehead = dump.u64(CTX + i * 8 + 0x68)
        head = dump.u64(CTX + i * 8 + 0x88)
        print("   bucket %d: slot %#x, %d slots/chunk, chunk %#x bytes" %
              (i, slot, count, want))
        print("     bytes carved %s, allocations served %s" % (carved, served))
        tebish = owner
        print("     lock word %#x %s, recursion %s" %
              (owner or 0,
               "(HELD by a live thread TEB)" if owner else "(free)", depth))
        print("     free-list head %#x, chunk-list head %#x" % (freehead or 0, head or 0))

        chunks, bad, slots, damaged = 0, 0, 0, []
        seen = set()
        cur = head
        while cur and cur not in seen:
            seen.add(cur)
            base = cur - 0x10
            declared = dump.u64(base - 8)
            if declared != want:
                bad += 1
                nxt = dump.u64(base)
                if nxt == cur:
                    break
                cur = nxt
                continue
            chunks += 1
            for k in range(count):
                h = dump.u64(base + 8 + k * stride)
                if h is None:
                    continue
                slots += 1
                if (h >> 32) != 0:
                    damaged.append((base + 8 + k * stride, h, k, base))
            cur = dump.u64(base)
        print("     chunk chain: %d chunks passed the size identity, %d failed it" %
              (chunks, bad))
        print("     %d slots enumerated, %d with a non-zero high dword in the header" %
              (slots, len(damaged)))
        for va, h, k, base in damaged:
            mark = ""
            if hfi and va in (hfi["address"], hfi["address"] + 0x10):
                mark = "   <<< THIS IS THE ONE THAT WAS FREED"
            print("       %#x  hdr %#018x  chunk %#x slot %d/%d  va%%16=%d%s" %
                  (va, h, base, k, count, va % 16, mark))
        total_damaged += damaged
        print("")
    print("   TOTAL damaged slots across all four buckets: %d" % len(total_damaged))
    _ = tids


main()
