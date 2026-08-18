# Backdrop blur

Taken from ADR 0051, "Backdrop blur is an accepted loss".

## What this licenses

One construct of `reference/exemptions.tsv`: `backdrop-blur`.

## Why no construct of the reference serves its purpose

`backdrop-filter: blur(...)` samples whatever the browser has already
composited behind an element and blurs that sample. It is a compositor
operation over the page's own backing store.

iced hands the client a canvas and no read access to what stands beneath a
widget. A blur of the page behind a surface would mean rendering the page to an
offscreen target, blurring that target, and drawing the surface over it, per
surface, per frame.

The dark scheme — the one scheme this client targets — already turns backdrop
blur off on the header, so the reference itself draws most of these surfaces
unblurred.

## What is drawn instead

Where the reference blurs the page behind a raised surface, this client paints
a solid scrim at matched perceived luminance. The dark scheme disables backdrop
blur on the header, so this stands only on the playback icon overlay.

One surface differs: the playback icon overlay reads as a flat scrim rather
than as blurred page. The luminance is matched, so the contrast the overlay's
lettering stands at is the reference's.
