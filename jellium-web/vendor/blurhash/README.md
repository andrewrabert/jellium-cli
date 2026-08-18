# blurhash

`blurhash` 0.2.3 as published, vendored for this workspace.

> A pure Rust implementation of [Blurhash](https://github.com/woltapp/blurhash).

Blurhash is an algorithm written by [Dag Ågren](https://github.com/DagAgren) for
[Wolt (woltapp/blurhash)](https://github.com/woltapp/blurhash) that encodes an
image into a short ASCII string. Decoding that string answers a gradient of
colours standing for the original image, which is what a card draws while its
own image loads.

Upstream is [whisperfish/blurhash-rs](https://github.com/whisperfish/blurhash-rs).
`reference/PINNED`'s third row names the revision this tree was taken from, and
`reference/vendor.tsv` records what became of each file of it.

## What this tree changes

The optional `image` and `gdk-pixbuf` dependencies are gone, and with them the
`image` and `gdk-pixbuf` features: nothing here reads an image file. The
benches, the criterion and proptest dev-dependencies and `data/` are gone for
the same reason. `wasm-bindgen-test` runs the crate's own unit tests on the
target this workspace builds for, so `src/base83.rs` and `src/dc.rs` mark their
tests `#[wasm_bindgen_test]` and `src/lib.rs`'s own test module, which reads
`data/octocat.png` through the `image` crate, is gone. The crate-level doc
example decodes rather than encodes, for the same reason.

The algorithm itself — `src/ac.rs`, `src/dc.rs`, `src/error.rs`, `src/util.rs`,
`build.rs` and everything of `src/lib.rs` above its test module — is upstream's,
unchanged.
