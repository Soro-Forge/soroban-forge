# brace-depth

Detects unbalanced brace depth in Rust source **that does not compile**.

## What it catches

When a conflicted merge is resolved badly, the usual result is an `impl` block whose closing brace is lost. Every function after that point silently nests one level deeper, and the file remains superficially plausible for thousands of lines.

The compiler's report is close to useless in this situation:

```text
error: this file contains an unclosed delimiter
    --> escrow/src/lib.rs:7295:3
2065 | impl LiquifactEscrow {
     |                      - unclosed delimiter
7295 | }
     |  ^
```

It points at the end of the file. Finding the actual divergence means bisecting by hand.

This check reports **the line of the opening brace that was never closed**, along with the final depth, so the divergence is immediate.

## Why it cannot use a parser

Every Rust analysis crate — `syn`, `rust-analyzer`, `rustc` itself — requires the input to parse. By definition it does not. This check therefore scans the source as text.

Doing that correctly is the whole difficulty. Braces appear inside string literals, raw string literals, unicode escapes such as `'\u{1F600}'`, line comments, and nested block comments, and an apostrophe may begin either a char literal or a lifetime. All of these are handled.

## Usage

```rust
use brace_depth::analyze;

let findings = analyze(source);
for finding in &findings {
    println!("line {}: {}", finding.line, finding.message);
}
```

An empty result means the source is balanced.

## Fixtures

`fixtures/` contains small Rust files exhibiting each case. They are never compiled as part of the build.
