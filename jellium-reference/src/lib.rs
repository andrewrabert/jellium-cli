//! The register that closes over what this client ported from jellyfin-web.
//!
//! Every ported construct is one row of `reference/provenance.tsv` cited by a
//! `// reference: <construct>` comment, every construct the reference reaches
//! and this client never observes is a `dead` row cited by nothing, and no
//! construct is both. Each row's lines stand under `reference/spans`, so a row
//! is measured against the pinned reference's text with no checkout in hand.
//!
//! This library reads the register, the spans and the tree, and depends on no
//! part of the tree it guards.

pub mod register;
pub mod spans;
pub mod tree;
