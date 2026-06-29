# financial-ops-macros

Procedural macros for [`financial-ops`](https://crates.io/crates/financial-ops).

This crate provides the `checked!` macro, which recursively rewrites an
arithmetic expression into a chain of the standard library's checked arithmetic
methods while preserving operator precedence and grouping.

You normally don't depend on this crate directly — the macro is re-exported as
`financial_ops::checked`.

```rust
use financial_ops::checked;

// Respects precedence: this is `a + (b * c)`, fully checked.
let value: Option<u64> = checked! { 2u64 + 3 * 4 };
assert_eq!(value, Some(14));

// Overflow short-circuits to `None`.
assert_eq!(checked! { u8::MAX + 1u8 }, None);

// With `@ error`, you get a `Result` instead.
let result = checked! { 2u64 + 2 @ "overflow" };
assert_eq!(result, Ok(4));
```
