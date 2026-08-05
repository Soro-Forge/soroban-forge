# enum-discriminants

Reports enum variants that resolve to the same discriminant value.

## Why this exists

`rustc` already reports this, as E0081. It only does so once the crate parses.

That caveat is the whole point of this check. A repository whose `lib.rs` holds
an unbalanced brace never reaches type checking, so every collision inside it
stays hidden behind a single parse error. Repairing the brace then surfaces a
wall of errors at once, with no way to know beforehand how many were waiting.

This check reads the text directly, so it answers that question whether or not
the crate compiles.

## What it understands

A discriminant does not have to be written on the variant to exist. Rust gives
an undecorated variant the previous value plus one, so a collision can involve
a variant that carries no number at all:

```rust
enum Code {
    First = 3,
    Second,     // 4
    Third = 4,  // collides with Second
}
```

The check follows that rule, and reads decimal, hexadecimal, binary and octal
literals, underscore separators, and type suffixes. Two spellings of one value
collide: `0x10` and `16` are the same discriminant.

When a variant carries an expression rather than a literal, such as `1 << 3`,
its value is unknown. The check reports nothing for it, and stops tracking
implicit values after it rather than guessing.

Variants are grouped by their own enum. Two enums using the same value are not
a collision.

## Usage

```rust
use enum_discriminants::analyze;

let collisions = analyze(source);

for collision in collisions {
    println!(
        "{} = {} on lines {:?}",
        collision.enum_name, collision.value, collision.lines
    );
}
```

Each `Collision` carries the enum name, the shared value, the colliding variant
names in declaration order, and the line each was declared on.
