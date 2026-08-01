# orphan-modules

Detects source files that no `mod` declaration references.

## What it catches

Rust only compiles a file if some module declares it. A test file added to a directory but never added to `mod.rs` or `tests.rs` is not a failing test — it is not a test at all. It is never compiled, never run, and never counted against coverage.

This fails silently and in the most misleading possible direction: the suite passes, the file is visibly present in the repository, and the coverage figure is computed over the subset that actually ran. A contributor can add a hundred lines of tests, watch CI go green, and have contributed nothing.

One production Soroban contract had **fifty** test files on disk and **twenty-six** declared. Half the suite had never executed.

## What it does not catch

A declaration whose file is missing is a compile error the compiler already reports clearly. This check only looks in the other direction.

## Usage

```rust
use orphan_modules::analyze;

let declarations = std::fs::read_to_string("src/tests.rs")?;
let files = ["admin.rs", "funding.rs", "funding_events.rs"];

for orphan in analyze(&declarations, &files) {
    println!("{} is never declared", orphan.file);
}
```

Reading the directory is the caller's responsibility. This crate takes text and file names so it stays testable without a filesystem, in keeping with the architecture rules.

`mod.rs`, `lib.rs`, `main.rs` and non-Rust files are ignored. Inline modules (`mod name { ... }`) count as declared. Declarations that are commented out do not.

## Fixtures

`fixtures/tests_declarations.rs` is abbreviated from a real contract's declaration file, including a declaration that was commented out and never restored.
