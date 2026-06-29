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

// With `@ error`, you get a `Result` you can `?` on.
let total = checked! { 2u64 + 2 @ "overflow" }?;
assert_eq!(total, 4);
```

Supported operators: `+`, `-`, `*`, `/`, `%` (mapped to `checked_add`,
`checked_sub`, `checked_mul`, `checked_div`, `checked_rem`). Each operand is
evaluated exactly once, in left-to-right order.

## Releasing

Releases are automated with [release-plz](https://release-plz.dev) via the
`Release-plz` GitHub Actions workflow (`.github/workflows/release-plz.yml`),
driven by [Conventional Commits](https://www.conventionalcommits.org).

**One-time setup:** add a [crates.io API token](https://crates.io/settings/tokens)
as a repository secret named `CARGO_REGISTRY_TOKEN`
(*Settings → Secrets and variables → Actions*).

**The flow:**

1. Land changes on `master` using conventional commit messages
   (`feat: ...`, `fix: ...`, `feat!: ...` for breaking, etc.).
2. release-plz opens/updates a **release PR** that bumps the crate versions and
   updates the changelogs based on those commits.
3. **Merge the release PR.** release-plz then publishes the changed crates to
   crates.io in dependency order (`financial-ops-macros` before `financial-ops`)
   and creates the git tags and GitHub releases.

> Tip: PRs opened by the default `GITHUB_TOKEN` don't trigger other workflows
> (so CI won't run on the release PR). To get CI on release PRs, create a
> [GitHub App token](https://release-plz.dev/docs/github/token) or PAT and use
> it for the `Release-plz` workflow.

A manual fallback workflow (`.github/workflows/publish.yml`) is available from
the Actions tab (*Run workflow*) for one-off publishes, including a `dry_run`
option that runs `cargo publish --workspace --dry-run`.
