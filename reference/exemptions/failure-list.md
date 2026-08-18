# The session's failure list

Taken from the requirements note "Jellium Web boot and failure visibility",
whose Failure reporting section states: "A failure list holding every failure
report raised in the session, including dismissed ones, is reached from the
settings menu."

## What this licenses

One construct of `reference/exemptions.tsv`: `failure-list`.

## What it serves

Every failure raised this session, dismissed ones included, read after the
fact. A failure that ends the session replaces the screen, one the user can act
on is raised as the reference's own toast until it is dismissed, and one the
client has already answered on screen is recorded and drawn nowhere. Each of
the three leaves the console record behind it, and this list is where a user
reads them without a console.

## Why no construct of the reference serves it

jellyfin-web reports a failure as a toast and keeps no record of one the user
dismissed. It draws no list of the failures a session met, on any of its
routes, so `reference/constructs.tsv` names no construct this screen could be
drawn as. The requirement that produced it is this client's own, and it stands
on this document rather than on a page of the reference.
