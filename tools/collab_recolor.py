#!/usr/bin/env python3
"""Real colours for the collaboration hairs and faces - the pixels, and which slot is which.

The owner, 2026-10-02: the collaboration hairs and faces could not change colour. Nexon ships
each one with eight hair ids (`42571` = Fern Hair, Red) or nine face ids (`22136` = Fern Face,
eye colour 1), but every variant's property image outlinks its layers to the BASE id's canvases
(`Character/Hair/_Canvas/00042570.img/default/hair`) - the modern client's own data does the
same, and every variant carries `collabo`, `noColorName`, `noPrism`. So every colour drew the
same picture.

This module makes the colours real. For each variant that is not the art's own colour it
recolours the base pixels toward the slot's colour, and `tools/backport_install.py` writes them
into the variant's own `_Canvas` image under `recolor/...` and repoints the variant's outlinks
there. The slot that matches the art keeps Nexon's original pixels untouched.

**Two slots per item, kept apart on purpose:**

* `art` - the slot whose colour the original art already is. It keeps Nexon's pixels.
* `default` - the slot a coupon gives (the owner's choice, `world::cosmetics`). Usually `art`;
  where it is not (Aura, Linie, Lugner's eyes) the default is a recolour.

Frieren's hair is white and there is no white hair slot; the owner chose Yellow (3) for it and
said the label is fine, so her original art sits at slot 3.

**The colour names are the client's own** - two tables in `MapleStory.exe`, read 2026-10-02:
hair `0x143a49900` Black, Red, Orange, Yellow, Green, Blue, Violet, Hazel; eyes `0x143a49940`
Black, Blue, Red, Green, Hazel, Emerald, Violet, Amethyst (slot 8 has no name there). The
target hues come from MEASURING Nexon's classic hairs and faces per slot (six hair styles, four
faces), not from the names.

Standard library plus nothing: pixels are RGBA bytes, colour maths is `colorsys`.
"""
import colorsys
import math

HAIR_NAMES = ["Black", "Red", "Orange", "Yellow", "Green", "Blue", "Violet", "Hazel"]
EYE_NAMES = ["Black", "Blue", "Red", "Green", "Hazel", "Emerald", "Violet", "Amethyst", "(8)"]

# base id -> (art slot, default slot). Hair: the art's colour is the owner's default for each.
HAIR = {
    42540: (3, 3),  # Frieren Hair - white art, at Yellow (the owner, 2026-10-02)
    42550: (3, 3),  # Frieren Hair (Sleep)
    42560: (3, 3),  # Frieren Hair (Ringlets)
    42570: (6, 6),  # Fern Hair - violet
    42580: (1, 1),  # Stark Hair - red
    42590: (5, 5),  # Himmel Hair - blue
    42600: (4, 4),  # Ubel Hair - green
}
# face style -> (art slot, default slot). The art slot is the eye colour the art is, read off a
# render of each face (2026-10-02); the default is the owner's choice where they gave one.
FACE = {
    22035: (5, 5),  # Frieren - teal eyes = the client's Emerald
    22036: (6, 6),  # Fern - violet
    22037: (2, 2),  # Stark - red (the skin-coloured patch on his face is not his eyes)
    22038: (1, 1),  # Himmel - blue
    22039: (6, 6),  # Ubel - violet
    22040: (1, 6),  # Aura - blue art, default Violet (the owner: "purple")
    22041: (6, 7),  # Linie - violet art, default Amethyst
    22042: (1, 4),  # Lugner - blue art, default Hazel
}

# (hue degrees, saturation, mean lightness) per slot. Hue from the classic measurement; the
# lightness is set for these brighter, softer-shaded collaboration arts (their sources average
# 0.37..0.86), keeping the classic ORDER - Black darkest, Yellow lightest, Hazel dull.
HAIR_TARGET = [
    (0, 0.04, 0.22),    # Black
    (358, 0.65, 0.40),  # Red
    (28, 0.78, 0.46),   # Orange
    (48, 0.80, 0.58),   # Yellow
    (95, 0.50, 0.40),   # Green
    (210, 0.66, 0.48),  # Blue
    (278, 0.50, 0.44),  # Violet
    (30, 0.42, 0.32),   # Hazel
]
EYE_TARGET = [
    (220, 0.15, 0.20),  # Black
    (218, 0.85, 0.50),  # Blue
    (350, 0.85, 0.46),  # Red
    (118, 0.72, 0.42),  # Green
    (36, 0.78, 0.42),   # Hazel
    (190, 0.80, 0.46),  # Emerald
    (258, 0.80, 0.52),  # Violet
    (292, 0.72, 0.50),  # Amethyst
    (220, 0.08, 0.72),  # (8) - pale grey, as the classic slot 8 measures
]


# The lightness spread a recoloured hair gets - about what the classic hairs measure.
HAIR_SPREAD = 0.17


def hair_slot(item_id):
    return item_id % 10


def hair_base(item_id):
    return item_id - item_id % 10


def eye_slot(item_id):
    return item_id // 100 % 10


def face_style(item_id):
    return item_id - eye_slot(item_id) * 100


def plan(item_id, kind):
    """`(base, slot, art slot, target)` for a collaboration variant, `target` None when the slot
    keeps Nexon's pixels. `None` for an id that is not one of these."""
    if kind == "Hair":
        base, slot = hair_base(item_id), hair_slot(item_id)
        if base not in HAIR or slot >= len(HAIR_TARGET):
            return None
        art = HAIR[base][0]
        return base, slot, art, None if slot == art else HAIR_TARGET[slot]
    base, slot = face_style(item_id), eye_slot(item_id)
    if base not in FACE or slot >= len(EYE_TARGET):
        return None
    art = FACE[base][0]
    return base, slot, art, None if slot == art else EYE_TARGET[slot]


def default_id(item_id, kind):
    """The id a coupon for this style should give - its default slot. Unchanged for others."""
    if kind == "Hair" and hair_base(item_id) in HAIR:
        return hair_base(item_id) + HAIR[hair_base(item_id)][1]
    if kind == "Face" and face_style(item_id) in FACE:
        return face_style(item_id) + FACE[face_style(item_id)][1] * 100
    return item_id


def _hue_dist(a, b):
    d = abs(a - b) % 360
    return min(d, 360 - d)


def _remap_l(l, src_mean, dst_mean):
    """Piecewise-linear lightness map that sends `src_mean` to `dst_mean` and keeps 0 at 0 and 1
    at 1 - outlines stay dark and highlights stay light, the shading survives."""
    src_mean = min(max(src_mean, 0.02), 0.98)
    if l <= src_mean:
        return l * dst_mean / src_mean
    return 1 - (1 - l) * (1 - dst_mean) / (1 - src_mean)


IRIS_MIN_SAT = 0.42


def iris_mask(rgba, w, h, art_hue, window=22.0):
    """The eye pixels of a face: saturated, mid-light, within `window` degrees of the art's eye
    hue. Brows (hair-coloured), outlines (dark), whites and skin (Stark's patch) fall outside."""
    out = [False] * (w * h)
    for i in range(w * h):
        r, g, b, a = rgba[i * 4:i * 4 + 4]
        if a < 32:
            continue
        hh, ll, ss = colorsys.rgb_to_hls(r / 255, g / 255, b / 255)
        if ss >= IRIS_MIN_SAT and 0.10 < ll < 0.92 and _hue_dist(hh * 360, art_hue) <= window:
            out[i] = True
    return out


def _opaque_hls(rgba, w, h, mask=None):
    """`{pixel index: (h, l, s)}` for the pixels a recolour touches."""
    out = {}
    for i in range(w * h):
        if rgba[i * 4 + 3] >= 16 and (mask is None or mask[i]):
            r, g, b = rgba[i * 4:i * 4 + 3]
            out[i] = colorsys.rgb_to_hls(r / 255, g / 255, b / 255)
    return out


def _stats(values):
    """`(mean lightness, mean saturation, lightness standard deviation)` of `(h, l, s)` triples."""
    n = len(values)
    mean_l = sum(v[1] for v in values) / n
    mean_s = sum(v[2] for v in values) / n
    std_l = math.sqrt(sum((v[1] - mean_l) ** 2 for v in values) / n)
    return mean_l, mean_s, std_l


def hair_stats(layers):
    """**One set of statistics for a whole hair**, pooled over its layers: `{node: (rgba, w, h,
    ...)}` in, `(mean lightness, mean saturation, lightness spread)` out.

    The owner, 2026-10-03: *"the hair looks fine from behind, but all of the color edits made the
    front facing bottom hair way lighter"*. Each layer used to be normalised on its own, and the
    artists draw `hairBelowBody` - the hair that hangs in front - darker than the rest (Frieren
    0.63 lightness against 0.86 for `hair`, Ubel 0.21..0.28 against 0.35; every collaboration hair
    that has the layer). Normalising it separately lifted it to the same mean as the top. One
    mapping for every layer keeps those differences. `hairShade` is left out of the pool: it is
    a flat shadow cast on the face (one value, 0.30, in every hair), not the hair's art, and
    `backport_install.recolor_looks` keeps mapping it on its own as before - against the pooled
    statistics that one value would come out near black."""
    values = []
    for node, layer in layers.items():
        if "hairShade" in node:
            continue
        rgba, w, h = layer[0], layer[1], layer[2]
        values.extend(_opaque_hls(rgba, w, h).values())
    return _stats(values) if values else None


def recolor(rgba, w, h, target, mask=None, spread=None, stats=None):
    """`rgba` with the masked pixels (all opaque ones when `mask` is None) moved to `target`
    `(hue, saturation, mean lightness)`: the hue replaced, saturation scaled so the masked mean
    lands on the target's, lightness remapped around its mean. Alpha untouched. New bytes.

    `spread`: when given, lightness is re-normalised to that standard deviation around the
    target instead of the piecewise map - for an art whose shading is squeezed into a narrow
    band. Frieren's white hair averages 0.86 lightness with little spread; the piecewise map
    kept it pastel in every colour (the 2026-10-02 contact sheet).

    `stats`: `(mean lightness, mean saturation, lightness spread)` to map from instead of this
    layer's own - [`hair_stats`], so every layer of one hair moves by the same mapping."""
    hue_t, sat_t, light_t = target
    hls = _opaque_hls(rgba, w, h, mask)
    if not hls:
        return bytes(rgba)
    mean_l, mean_s, std_l = stats if stats is not None else _stats(list(hls.values()))
    out = bytearray(rgba)
    for i, (hh, ll, ss) in hls.items():
        if spread is None:
            nl = _remap_l(ll, mean_l, light_t)
        else:
            nl = min(0.97, max(0.03, light_t + (ll - mean_l) * spread / max(std_l, 0.03)))
        # Scaled from the pixel's own saturation, but never below 60% of the target: a white
        # art's highlights carry almost none, and scaling alone left Frieren grey inside.
        ns = min(1.0, sat_t * min(1.2, max(0.6, ss / max(mean_s, 0.08))))
        r, g, b = colorsys.hls_to_rgb(hue_t / 360.0, nl, ns)
        out[i * 4] = round(r * 255)
        out[i * 4 + 1] = round(g * 255)
        out[i * 4 + 2] = round(b * 255)
    return bytes(out)


def art_eye_hue(rgba, w, h, art_slot):
    """The face's own eye hue: the saturation-weighted circular mean of the pixels near the art
    slot's classic hue - so the mask follows the art, not the table."""
    centre = EYE_TARGET[art_slot][0]
    x = y = 0.0
    for i in range(w * h):
        r, g, b, a = rgba[i * 4:i * 4 + 4]
        if a < 32:
            continue
        hh, ll, ss = colorsys.rgb_to_hls(r / 255, g / 255, b / 255)
        if ss >= 0.22 and 0.10 < ll < 0.92 and _hue_dist(hh * 360, centre) <= 22:
            x += math.cos(hh * 2 * math.pi) * ss
            y += math.sin(hh * 2 * math.pi) * ss
    if x == 0 and y == 0:
        return centre
    return math.degrees(math.atan2(y, x)) % 360


def recolor_layer(rgba, w, h, kind, target, art_slot, eye_hue=None, stats=None):
    """One layer of a variant: the whole hair, or a face's irises only. For a hair, `stats` is
    [`hair_stats`] of all its layers - without it the layer is normalised on its own, which
    lightens a layer the art draws darker."""
    if kind == "Hair":
        return recolor(rgba, w, h, target, spread=HAIR_SPREAD, stats=stats)
    hue = eye_hue if eye_hue is not None else art_eye_hue(rgba, w, h, art_slot)
    return recolor(rgba, w, h, target, iris_mask(rgba, w, h, hue))


def _self_test():
    # A two-pixel violet "hair" recoloured red: hue moves, alpha and the dark/light order stay.
    px = bytes([60, 20, 80, 255, 200, 160, 230, 255])
    out = recolor(px, 2, 1, HAIR_TARGET[1])
    h0 = colorsys.rgb_to_hls(*(v / 255 for v in out[0:3]))
    h1 = colorsys.rgb_to_hls(*(v / 255 for v in out[4:7]))
    assert _hue_dist(h0[0] * 360, 358) < 3 and _hue_dist(h1[0] * 360, 358) < 3, (h0, h1)
    assert h0[1] < h1[1], "the darker pixel stays darker"
    assert out[3] == 255 and out[7] == 255
    # Two layers, one drawn darker (the hair hanging in front): with the hair's pooled
    # statistics the darker layer stays darker after the recolour; normalised alone it would
    # land on the same mean as the light one.
    top = bytes([230, 230, 230, 255, 250, 250, 250, 255])
    below = bytes([150, 150, 150, 255, 170, 170, 170, 255])
    pooled = hair_stats({"default/hair": (top, 2, 1), "default/hairBelowBody": (below, 2, 1)})
    lt = [colorsys.rgb_to_hls(*(v / 255 for v in recolor_layer(top, 2, 1, "Hair", HAIR_TARGET[7], 3, stats=pooled)[k:k + 3]))[1] for k in (0, 4)]
    lb = [colorsys.rgb_to_hls(*(v / 255 for v in recolor_layer(below, 2, 1, "Hair", HAIR_TARGET[7], 3, stats=pooled)[k:k + 3]))[1] for k in (0, 4)]
    assert sum(lb) / 2 < sum(lt) / 2 - 0.1, (lt, lb)
    alone = [colorsys.rgb_to_hls(*(v / 255 for v in recolor_layer(below, 2, 1, "Hair", HAIR_TARGET[7], 3)[k:k + 3]))[1] for k in (0, 4)]
    assert abs(sum(alone) / 2 - HAIR_TARGET[7][2]) < 0.02, "the control: alone, it is lifted to the target mean"
    assert hair_stats({"default/hairShade/0": (below, 2, 1)}) is None, "the shade is not in the pool"
    # The slots: art keeps pixels, others get a target, defaults follow the owner.
    assert plan(42576, "Hair")[3] is None, "Fern's violet IS the art"
    assert plan(42570, "Hair")[3] == HAIR_TARGET[0], "Fern in Black is a recolour"
    assert plan(42543, "Hair")[3] is None, "Frieren's white at Yellow"
    assert plan(22140, "Face")[3] is None and plan(22640, "Face")[3] == EYE_TARGET[6], "Aura: blue art, violet recolour"
    assert default_id(2_2040, "Face") == 22640 and default_id(42570, "Hair") == 42576 and default_id(42540, "Hair") == 42543
    assert default_id(30000, "Hair") == 30000, "a classic hair is not touched"
    # A skin-hued pixel next to a red eye is not in Stark's iris mask.
    eye = bytes([200, 30, 40, 255]) + bytes([200, 150, 110, 255])
    m = iris_mask(eye, 2, 1, art_hue=355)
    assert m == [True, False], m
    print("collab_recolor self-test: ok")


if __name__ == "__main__":
    _self_test()
