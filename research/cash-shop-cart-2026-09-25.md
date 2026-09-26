# The Cash Shop cart is client-side, and it is drawn in the CART tab (2026-09-25)

The owner, 2026-09-25, from the live server with a screenshot: Mushroom House Elf, Mystery Face
Coupon and Mystery Hair Color Coupon show the tile's cart button as a **checkmark**, and they
expected them in the six-slot row under the character preview (above Revert / Remove All).

Static only. Nothing here has been on a wire or a screen.

## 1. The cart is a vector of commodity SNs on the stage, and no packet touches it **[L]**

`stage+0x108 .. +0x110` (capacity `+0x118`) is a `std::vector<int>` of commodity SNs. The four
functions that own it sit together:

```text
FUN_140D74820(stage, sn)   ADD: dedupe scan, push_back; false if already there   (no packet)
FUN_140D74890(stage, sn)   ERASE: remove every copy of sn                         (no packet)
FUN_140D74930(stage, sn)   CONTAINS -> bool
FUN_140D74960(stage)       COUNT = (end - begin) / 4
```

`FUN_140D74820` reaches the vector as `add rcx, 0x108` and then `[rcx]` / `[rcx+8]`, so a
`[reg+0x108]` field scan does not see it as a writer - `tools/fieldrefs.py 0x108` over the
stage's range lists only the constructor and the erase. It was found from the other side: the
string `0x1799` *"You can store up to %d types of items in your Cart."*.

## 2. The tile's cart button **[L]**

`FUN_1410BE0E0`, the item tile's handler. On the cart button:

```text
already in the cart  ->  FUN_140D74890 (erase); if the main view [win+0x250] == 7, redraw it
not in the cart      ->  count >= 15 ?  string 0x1799 with 15 - refused
                                     :  FUN_140D74820 (add)
then FUN_1410BB390 - the tile repaints its button
```

`FUN_1410BB3A3` sets the tile's check button (`[tile+0x11e0]`) from `FUN_140D74930(stage,
[tile+0x11ac])`. **So a checkmark on a tile means that SN is in this vector** - the owner's three
items were in the cart.

## 3. Where the cart is drawn: main-window view 7, the CART tab **[D]**

`[win+0x250]` is the main window's current view. View **7** is built by `FUN_1410BF6C0`, which,
like `FUN_1410C2710`, references the button names `cartBuy` and `cartDelete`; the main window's
button dispatcher `FUN_1410BCDA0` handles `cartSelectAll` / `cartDelete` / `cartBuy`, and its
`cartDelete` path erases every checked tile (`FUN_1410C24AD` loop -> `FUN_140D74890`), then sets
`[win+0x250] = 7` and rebuilds with `FUN_1410BF6C0`.

That the pink **CART** tab is what selects view 7 is **[D]**: the view has the cart's own
buttons and is what a cart change redraws, but the tab's click handler was not traced.

**The row under the preview is a different list.** It has six slots and its own Revert /
Remove All / Buy All; the cart holds fifteen. Nothing in §1's four functions refreshes the
preview. **[I]** that it is the try-on list for wearable items - not traced.

## 4. What this means for the server

Adding to and removing from the cart need nothing from us. **Buying from the cart** does: the
cart's Buy runs the same buy builder as a single purchase (`FUN_140D78070` -> `FUN_140D785F0`,
`research/cash-shop-cash-inventory.md` §4), one buy request per cart entry, advancing
`[stage+0x164]` after each `0x0C` + `0x05AD` reply pair. Every single buy is already answered
that way; a cart of several has never been bought on a screen.

## 5. To check on a screen

1. Tick two items' cart buttons, then open the pink **CART** tab. They should be there.
   If they are not, §3's [D] is wrong and the tab is something else.
2. Buy from the cart with both ticked. Both should land in the Cash Inventory and the NX
   should drop twice. If only the first arrives, the per-item loop is waiting for something
   the single buy does not need - check `world-ch<N>.log` for how many `0x03E1` sub-op `0x02`
   arrived.
