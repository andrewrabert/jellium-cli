# jellium-cli

## Review

- Prioritize craft defects, then correctness, then clarity.
- Answer a read of existing code with a named list of the craft defects found,
  or with the owned claim that it found none. There is no third answer.
- Rank every absolute defect below as a finding.
- Reject a reply that defends code with a banned justification instead of
  arguing with it.
- Route every named defect into a new cycle. A named defect is fixed or routed,
  never dropped.

## Scope, by agent

- `plan` is the only agent that widens: it fixes every absolute defect in code
  it already modifies, and lists the defects it saw elsewhere.
- `execute` never widens: it reports the defects it meets and fixes none outside
  the plan.
- `review` ranks craft defects as findings and accepts no banned justification.
- `orchestrator` routes every named defect into a new cycle rather than letting
  it end in a report.

## Do not, under any circumstance

- Write `#[allow(..)]`, `#[expect(..)]`, `#![allow(..)]` or `#![expect(..)]`,
  for any lint, from clippy or from rustc, at item, module or crate level, with
  or without `reason = ".."`, inside `cfg_attr` or anywhere else.
- Lower a lint level by another mechanism: a `[lints]` entry set to `allow`, a
  loosened threshold in a `clippy.toml`, or `-A`, `--allow` or `--cap-lints` in
  a justfile recipe or a CI step. Raising strictness is permitted: add lints,
  tighten thresholds, promote `warn` to `deny`.
  `jellium-web/clippy.toml`'s nine `disallowed-methods` entries are the model:
  configuration that adds obligations rather than releasing them. They stay.
- Take a `bool` parameter standing for a mode. Take two types, or two functions.
- Cross a boundary with a sentinel standing for absence. Cross with `Option`,
  `null`, or a variant that names the absence.
- Prefix or suffix a name to dodge a collision. Rename for what the thing is.
- Write a string literal spelling the name of the symbol it sits inside.
- Write a function whose body is a single call forwarding its own arguments
  unchanged to a function of the same name.
- Declare a type that exists only to be mapped one to one onto another type in
  the same crate.
- Write a comment asserting a runtime property the types could carry instead.
- Put two doc comments on one item saying the same thing.
- Put a naked scalar in a signature where the language allows a domain type. A
  foreign boundary that can carry only a scalar converts at exactly one site,
  and no signature above that site carries the scalar.
- Introduce any of the above in code you write. Code you write is existing code
  the instant it is written, and every rule here binds it unchanged.

## Name it and record the call, either way

- A wrapper layer that only forwards.
- Where a conversion boundary sits.
- Whether a concept has earned a type of its own.

## Banned justifications

Each argues from where code came from, or from the fact that it functions,
rather than from what it says. Offering one makes the reply defective; the
caller rejects the reply rather than arguing with it.

- it already exists
- it is not new code
- it works
- it compiles
- clippy is clean
- the tests pass
- that is the existing pattern here
- it was reviewed
- it shipped
- it is temporary
- it is only glue or boundary code
- changing it is risky
- it is generated — provenance locates where a fix belongs and never justifies
  what the code is

## Vocabulary the rules above depend on

- `crate::failure::called(Call, Result<T, JsValue>) -> Option<T>` — every
  foreign call passes through it; `Call` is the closed set of JavaScript calls.
- `crate::failure::rendered(Text, &T) -> Option<String>` and
  `crate::failure::encoded` — the only sites that serialize.
- `crate::failure::decoded(Text, &str)` and `failure::parsed` deserialize;
  `failure::narrowed(Text, value)` and `failure::read(Text, &str)` narrow a
  number or read a string as one; `failure::unraised::decoded` and
  `failure::unraised::read` are those doors raising nothing. Nothing else reads.
- `crate::text::lookup(Text) -> &'static str` and `crate::text::format` — every
  user-facing sentence, addressed by a `Text` variant; a variant and its entry
  in `jellium-web/strings/en-us.json` exist together or not at all.
- `jellium-web/clippy.toml` — the nine `disallowed-methods` entries that force
  the doors above.

## Enforcement

- `just suppressions` and the CI step of the same name fail on any of the two
  prohibitions in the first group, and on a `serde_json` deserializer named
  outside `crate::failure`. Adding one to the tree breaks the build.
