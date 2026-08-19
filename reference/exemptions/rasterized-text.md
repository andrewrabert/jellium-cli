# Browser-rasterized text

Taken from ADR 0052, "GPU-rasterized text is an accepted loss".

## What this licenses

One construct of `reference/exemptions.tsv`: `browser-rasterized-text`.

## Why no construct of the reference serves its purpose

The reference's lettering is laid out and rasterized by the browser. Line
breaking, hinting, subpixel positioning and subpixel coverage are the browser's,
and they differ between browsers on the same page.

This client draws to a GPU canvas. iced shapes and rasterizes glyphs itself,
from the same font files the reference ships, at the same sizes and weights.
The font, the size and the weight are ported; the rasterizer is not, and there
is no rasterizer to port to.

ADR 0047 makes appearance ported from the pinned reference in the same way
behaviour is. This records the one part of appearance where that is not
achievable rather than merely unfinished.

## What is drawn instead

Lettering is laid out and rasterized by iced on a GPU canvas rather than by the
browser. Glyph positions, hinting and subpixel coverage differ from the
reference's at every size, and no gate compares them.

A screenshot comparison against the reference will never match at the pixel,
anywhere text is drawn, so no gate is built on one. Font family, size, weight,
line height and colour remain ported values under `reference/provenance.tsv`
and stay under the appearance gate. Only the rasterization of the glyphs is
out.
