# Financial Ops

[![Crates.io](https://img.shields.io/crates/v/financial-ops)](https://crates.io/crates/financial-ops)
[![Docs.rs](https://docs.rs/financial-ops/badge.svg)](https://docs.rs/financial-ops)
[![License](https://img.shields.io/crates/l/financial-ops)]()

This crate provides a set of operations for working with financial data, more specifically, avoiding
the usage of floating point types.

## Usage

```rust
use financial_ops::CheckedDecimalOperations;

fn test_add_decimals() {
    let a: u64 = 1_0000;
    let a_decimals = 4;
    let b: u64 = 2_00;
    let b_decimals = 2;

    let (result, decimals) = a.add_decimals_checked(b, a_decimals, b_decimals)?;
    assert_eq!(result, 3_0000);
    assert_eq!(decimals, 4);

    let a: u32 = 123_45;
    let a_decimals = 2;
    let b: u32 = 0_45;
    let b_decimals = 2;

    let (result, decimals) = a.add_decimals_checked(b, a_decimals, b_decimals)?;
    assert_eq!(result, 123_90);
    assert_eq!(decimals, 2);
}
```

Very useful when dealing with money or blockchain transactions.

## Supported operations

### Checked

This set of operations will return an `Result` with the result and the number of decimals,
if the operation is successful. If the operation is not successful, it will return a `DecimalOperationError`.

```rust
use financial_ops::CheckedDecimalOperations;
```

- `add_decimals_checked`
- `sub_decimals_checked`
- `mul_decimals_checked`
- `div_decimals_checked`
- `rem_decimals_checked`

Each of these has an `_or` variant that lets you choose the error type (any
value — your own enum, or even a `&str` — no external crate required), mirroring
the `@` syntax of the [`checked!` macro](#the-checked-macro):

```rust
use financial_ops::CheckedDecimalOperations;

#[derive(Debug, PartialEq)]
enum MyError { Math }

let result = u32::MAX.add_decimals_checked_or(1, 0, 0, MyError::Math);
assert_eq!(result, Err(MyError::Math));

// The error can also just be a string:
let r: Result<(u64, u32), &str> = 10u64.divide_decimals_checked_or(0, 2, 2, "division by zero");
assert_eq!(r, Err("division by zero"));
```

- `add_decimals_checked_or`
- `sub_decimals_checked_or`
- `multiply_decimals_checked_or`
- `divide_decimals_checked_or`
- `rem_decimals_checked_or`

### Unchecked

This set of operations will return the result and the number of decimals, without any checks,
carrying the underlying operation way of handling overflows and underflows.

```rust
use financial_ops::DecimalOperations;
```

- `add_decimals`
- `sub_decimals`
- `mul_decimals`
- `div_decimals`
- `rem_decimals`

## The `checked!` macro

The `checked!` macro rewrites a normal arithmetic expression into a chain of
checked operations, recursively, while respecting operator precedence and
parentheses. Without an error it evaluates to an `Option`; with a trailing
`@ <error>` it evaluates to a `Result`.

```rust
use financial_ops::checked;

// Respects precedence: this is `a + (b * c)`, fully checked.
let value: Option<u64> = checked! { 2u64 + 3 * 4 };
assert_eq!(value, Some(14));

// Overflow short-circuits to `None`.
assert_eq!(checked! { u8::MAX + 1u8 }, None);

// With `@ <error>` you get a `Result<T, E>`, where `E` is simply the type of
// the expression you pass. A string literal makes `E = &str` — no `anyhow`,
// no external crate, no trait bounds.
let ok: Result<u64, &str> = checked! { 2u64 + 2 @ "overflow" };
assert_eq!(ok, Ok(4));

let err: Result<u8, &str> = checked! { u8::MAX + 1u8 @ "overflow" };
assert_eq!(err, Err("overflow"));
```

Because `@ <error>` expands to `.ok_or(<error>)`, the error can be anything: a
`&str`, your own enum variant, etc. To use `?` on the result, the surrounding
function's error type just needs to be the same type (or `From` it).

Supported operators: `+`, `-`, `*`, `/`, `%` (mapped to `checked_add`,
`checked_sub`, `checked_mul`, `checked_div`, `checked_rem`). Each operand is
evaluated exactly once, in left-to-right order.
