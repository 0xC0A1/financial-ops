# Changelog
All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog, and this project adheres to Semantic Versioning.

Note: Version 0 of Semantic Versioning is handled differently from version 1 and above. The minor version will be incremented upon a breaking change and the patch version will be incremented for features.

Note: from this release onward, the changelog and version bumps are automated
with [release-plz](https://release-plz.dev) from conventional commits. The
entries below were written by hand.

## [Unreleased]
### Features
### Fixes
### Breaking

## [1.0.0] - 2026-06-29
### Features
- Added the `checked!` macro (new `financial-ops-macros` crate, re-exported as
  `financial_ops::checked`) that recursively rewrites an arithmetic expression
  into checked arithmetic, respecting operator precedence and parentheses.
  Evaluates to an `Option` by default, or a `Result` when given a trailing
  `@ <error>`.
- Modernized CI to run `rustfmt`, `clippy`, build, and tests.
- Automated releases with release-plz: pushes to `master` open a release PR that
  bumps versions and updates changelogs; merging it publishes to crates.io and
  creates GitHub releases.
- Derived `Clone`, `Copy`, `PartialEq`, and `Eq` for `DecimalOperationError`.

### Fixes
- Made checked operation error handling consistent and more idiomatic.
- Updated repository and homepage links to the new GitHub organization (`0xC0A1`).

### Breaking
- Migrated to the Rust 2024 edition and raised the minimum supported Rust
  version (MSRV) to 1.85. Consumers must build with Rust 1.85 or newer.

## [0.1.0] - 2024-07-31

### Features
- Initial release
- Added basic operations like: addition, subtraction, multiplication, division, and remainder.

### Fixes
### Breaking