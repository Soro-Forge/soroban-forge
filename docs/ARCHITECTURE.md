# Architecture

## Principle: every check is an island

Soroban Forge is deliberately structured so that **no two contributors can ever conflict with each other.**

This is not an aesthetic preference. It is the core design constraint, and every other decision follows from it.

Each check lives in its own directory, is its own crate, owns its own tests and fixtures, and depends on nothing else in the repository. A contributor implementing the brace-drift detector never opens a file touched by the contributor implementing the duplicate-function detector.

## Layout

```text
tools/
  v1/
    analysis/          checks that read source and report findings
    reporting/         output formatters and aggregation
    integration/       CI wrappers and GitHub Action packaging
  v2/
    ...                later-release tools, built but not yet wired in
```

A tool directory always looks like this:

```text
tools/v1/analysis/<tool-name>/
  Cargo.toml           the crate manifest
  README.md            what this check catches, and why it matters
  src/lib.rs           the check itself, exposed as a library
  tests/               integration tests
  fixtures/            small, hand-written Rust files that exhibit the defect
```

## Rules

**1. A tool is a library, not a binary.**
Checks expose a pure function that takes source text and returns findings. They do not read the filesystem, print, or exit. This makes them trivially testable and freely composable.

**2. A tool never depends on another tool.**
If two checks need the same helper, each gets its own copy. Duplication is cheaper than coupling, and coupling is what makes parallel contribution impossible.

**3. A tool operates on text, never on a parsed AST.**
The entire point is to analyse files that do not compile. Anything that requires `syn` or `rustc` to succeed has already failed at the moment it is needed.

**4. Findings carry a line number and an explanation.**
A finding that says "file is malformed" is useless. A finding that says "brace depth entered this `impl` at line 2042 and never returned to zero; first divergence at line 5039" saves an afternoon.

**5. Fixtures are committed, small, and deliberately broken.**
Each check ships fixtures that exhibit the defect and fixtures that do not. They are plain `.rs` files that are never compiled as part of the build.

## Why a workspace of many small crates

One crate per check means `cargo test -p <tool>` runs in isolation, a broken contribution cannot break anyone else's tests, and review is confined to a single directory.

The cost is a longer workspace manifest. That is a good trade.
