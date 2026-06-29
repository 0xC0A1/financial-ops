pub mod core;

pub use core::*;

/// Recursively rewrites an arithmetic expression into checked arithmetic.
///
/// See the [`financial_ops_macros::checked`] documentation for details and
/// examples.
pub use financial_ops_macros::checked;
