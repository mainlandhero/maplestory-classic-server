# The Übel weapon refused over the suitcase: the cover had no child for weapon type 32 — 2026-09-12

The owner: *"Ubel's weapon still cannot be equipped over all weapons."* No `0x0107` for it ever
reached the server (grep of the last ten channel logs for a Deco-tab move: none), so the
refusal is the client's, before it sends.

## The data, read with `wz-dump cat` **[L]**

| image | children besides `info` |
|---|---|
| classic cover `01702001.img` | `30` (30 stances), `31 -> 30`, `32 -> 30`, `33 -> 30` — the three are **UOL links** |
| backported `01703726.img` (Übel) and all five siblings `0170372[2-7]` | `30` (33 stances), `49` (22 stances) |

Cobalt's suitcase `1322999` is type 32. The client looks up the equipped weapon's type as a
child of the cover's image; the classic cover says so by spelling 31/32/33 as links, and the
modern covers say only 30 and 49 (49 = gun, a type this client's Weapon archive does not
contain). The lookup itself is **[I]**; the classic file is the control.

## The fix

* `wz-dump build`'s `patch` op gained a `uol` leaf kind, and `merge`/`strings`/`patch` now start
  from the image an **earlier row of the same spec** produced (a `patch` after a `copy` used to
  replace the copy with an empty image plus the patch — `merge_images` keeps the last addition
  per name).
* `tools/backport_install.py` step 1b: for every backported cover, a TSV linking every weapon
  type the classic archive contains (30 31 32 33 37 38 39 40 41 42 43 44 45 46 47, read off
  the archive's image prefixes) to `30`, the classic covers' own pattern. Six covers, 14 links
  each. Built and verified (466/466 Weapon images parse; the built `01703726.img` shows all 15
  types with `30` intact).
* **Not installed yet**: the client was running and held the archives. Close it and run
  `python "C:\MapleCW\tools\backport_install.py" --install`.

## What to expect

Equipping the weapon over the suitcase should now go on (the tooltip's "all weapons" made
true). Two-handed types (40..47) link to one-handed stances; they will equip and may not draw
in two-handed poses — and covers do not draw at all yet (the character record carries worn
slots 1..31 only), so nothing regresses. Whether it *draws* is the next piece of work, already
recorded in STATUS.
