# Soroban Forge

Static analysis and repository-health tooling for [Soroban](https://developers.stellar.org/docs/build/smart-contracts/overview) smart contract projects.

Soroban Forge catches a specific and under-served class of defect: **damage introduced by the merge itself, not by any individual contributor's code.**

## Why this exists

High-velocity Soroban repositories — especially those running open-source campaigns with dozens of parallel contributors — accumulate a failure mode that ordinary tooling misses.

When a conflicted merge is resolved badly, the result is frequently not a syntax error at the conflict site. It is a file that is *structurally* wrong in ways that only surface much later:

- the same function defined twice inside one `impl` block, with the two bodies interleaved
- an `impl` block that is never closed, so every subsequent function silently nests inside it
- brace depth that drifts upward and never returns to zero
- test modules that exist on disk but are absent from `mod.rs`, so they never compile and never run
- error enum variants that collide on the same discriminant

`cargo fmt` reports `this file contains an unclosed delimiter` and stops. `cargo clippy` cannot run at all. The compiler points at the *end* of the file, thousands of lines from the actual divergence. Bisecting by hand takes hours.

These are not hypothetical. Every check in this repository is derived from a real corruption event observed in a production Soroban codebase, where a single bad merge silently duplicated eighteen functions and left the crate uncompilable for days while contributors continued to open pull requests against it.

## What it does

Soroban Forge analyses Rust source as text, so it works on files that **do not compile** — which is precisely when you need it most. Every existing Rust tool in the ecosystem requires a parseable file.

Each check is an independent tool. Each reports the exact line where structure diverges, not where the compiler eventually gave up.

## Checks

| Check | Detects | Layer of the failure |
| --- | --- | --- |
| [`brace-depth`](tools/v1/analysis/brace-depth) | Unclosed and unmatched braces, reported at the line that opened them | The parse error itself |
| [`duplicate-fn`](tools/v1/analysis/duplicate-fn) | Functions defined more than once within a single block | What the bad merge actually did |
| [`orphan-modules`](tools/v1/analysis/orphan-modules) | Test files on disk that no `mod` declaration references | Tests that silently never run |
| [`enum-discriminants`](tools/v1/analysis/enum-discriminants) | Enum variants resolving to the same discriminant value | Defects hidden behind the parse error |

Each is a library crate exposing a pure function over source text. None of them depend on one another.

## Status

Early development, and honest about it. Four checks are merged. There is not yet a way to run the toolkit against a repository without writing Rust — that work is [issue #9](https://github.com/Soro-Forge/soroban-forge/issues/9), with a GitHub Action wrapper in [issue #10](https://github.com/Soro-Forge/soroban-forge/issues/10).

The [open issues](https://github.com/Soro-Forge/soroban-forge/issues) are the roadmap. Two of them are marked good first issue.

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the repository layout and [`CONTRIBUTING.md`](CONTRIBUTING.md) if you would like to build a check. Brand assets live in [`brand/`](brand).

## Licence

MIT
