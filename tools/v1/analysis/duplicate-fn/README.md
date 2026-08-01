# duplicate-fn

Detects functions defined more than once within a single block.

## What it catches

A merge conflict resolved by keeping both sides produces a file in which the same function appears twice inside one `impl`, frequently with the two bodies interleaved. The file may still parse. The compiler will eventually complain, but only after the far louder structural errors are cleared — and if the enclosing block was also left unclosed, it never gets that far at all.

In one production Soroban contract a single bad merge duplicated **eighteen** functions this way:

```text
DUP get_escrow                      2444, 3197
DUP get_funding_token               2491, 3219
DUP clear_legal_hold_after_delay    4813, 5075
DUP partial_settle                  6066, 6324
...
```

Nobody noticed for days, because the crate could not be compiled to find out.

## Scoping

Definitions are attributed to their **enclosing block**, not merely to their brace depth. Two `impl` blocks at the same depth are different scopes, so this is correctly silent:

```rust
impl A {
    pub fn value(&self) -> u32 { 1 }
}

impl B {
    pub fn value(&self) -> u32 { 2 }
}
```

while this is reported:

```rust
impl A {
    pub fn value(&self) -> u32 { 1 }
    pub fn value(&self) -> u32 { 2 }
}
```

Depth alone would produce a false positive on the first example, which is why it is not used.

## Usage

```rust
use duplicate_fn::analyze;

for duplicate in analyze(source) {
    println!("{} defined at {:?}", duplicate.name, duplicate.lines);
}
```

An empty result means no block defines the same function twice.

## Fixtures

`fixtures/` contains small Rust files exhibiting each case. They are never compiled as part of the build.
