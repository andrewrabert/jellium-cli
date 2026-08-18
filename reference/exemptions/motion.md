# The reference's motion

Taken from ADR 0050, "The reference's motion is an accepted loss".

## What this licenses

Three constructs of `reference/exemptions.tsv`: `transition`, `keyframe` and
`ripple`.

## Why no construct of the reference serves their purpose

jellyfin-web animates. Its stylesheets carry roughly fifty `transition`
declarations and thirty `@keyframes` blocks, and `emby-ripple` paints a
travelling circle under every press of a button.

iced draws each frame from the widget tree the client builds for it. A
transition is a property interpolated by the browser's own compositor between
two computed styles, and neither the interpolation nor the intermediate styles
exist here. Reproducing them means the client holding a clock per animated
control and rebuilding its appearance every frame, for every control the
reference animates.

The construct gate compares what is drawn against what the reference draws. It
has no way to see motion, and a loss it cannot see is a loss that has to be
written down rather than measured.

## What is drawn instead

jellyfin-web's fifty transition declarations, its thirty keyframe blocks and
its ripples are not reproduced. A card under the pointer changes appearance
instantly rather than growing into it, a menu appears at full size rather than
sliding up, and a pressed control shows no ripple.

Every state the reference animates between is still drawn; only the travel
between them is gone, so no control ends in an appearance the reference does
not also reach. A construct absent from the client without a row of its own
still fails the gate.
