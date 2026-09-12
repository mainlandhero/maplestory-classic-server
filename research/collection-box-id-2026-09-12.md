# The Signature Style Collection box sent nothing on double-click: it was in a family the client does not open — 2026-09-12

The owner: *"Double clicking the Signature Style Collection box does not grant all 8 character
costume coupons."*

## Measured

* `world.log`: the box was bought, moved to the bag (`5222221` into Cash slot 5, restored on
  field entry) and then **no inbound packet followed the double-click** - every packet after
  that moment is enumerated in the log and is field-entry telemetry. The client did not treat
  the double-click as a use. **[L]**
* The set coupons `5681543..5681552` send `0x0114` on double-click every time - the Übel set
  was opened that way this same session. **[L]**
* The box's Item.wz node is byte-identical to the modern client's and has the same shape as a
  set coupon's `info` (`cash`, `collabo`, icons). **[L]**
* This client's own `String.wz` names no `522xxxx` item at all; family `568` is its five
  "5-slot coupons". **[L]**
* No archived world log contains `opened 5222221`: the box had never been opened on screen.

## Inferred

The classic client dispatches a Cash-tab double-click by the item id's family (`id / 10000`),
and `522` is not among the families it forwards as a use request while `568` is. **[I]** -
the static scan of the `0x0114` builder's UI callers found no readable prefix switch (it is a
table), so the pair of controls above is what this rests on.

## Changed

* `wz-dump build`: `merge` keys may be written `k=k2` (take the source's `k` as `k2`).
* `tools/backport_install.py`: the box's property node is merged into `0568.img` as
  `05681599`; its icon outlinks still name the `0522` canvas, which is copied as before; the
  string and the Cash Shop row use `5681599`. Every archive is now built from its pristine
  `.bak` when present, so a rebuild cannot carry a stale node forward (the store's item-count
  test caught exactly that: 2864 where 2863 was right).
* `world::signaturestyle::COLLECTION = 5_681_599`; the commodity test reads the regenerated
  table.
* `store::inventory::rename_item_ids`: `5222221 -> 5681599` in `inventory`, `equipment`,
  `cash_locker`, `storage_item`, on every `Store::open` - the live server's database is not
  the repo's, so a restart applies it. A test derives the table list from the schema so a
  fifth table cannot be skipped in silence.

## The mistake on the way, for the record

The first version of the `.bak`-as-base change reused one variable for the build base and the
install target, so `--install` wrote the built archives OVER the `.bak` files and moved the
pristine originals to `.bak.bak`, leaving the client's archives untouched. All 24 originals
were restored from `.bak.bak` with a size check (the pristine file is the smallest of the
three) before anything else was done. The two paths are now two variables.

## Unverified

The box opening on screen. Plan step TO(f) says what each outcome means. The repo database
held one box row at the start of this work and none by the end, so the id rewrite was proven
on the schema (in-memory, all four tables) and by opening a copy through a fresh store binary,
not on a real row.
