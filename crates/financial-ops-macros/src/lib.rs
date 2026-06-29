//! Procedural macros for [`financial-ops`](https://docs.rs/financial-ops).
//!
//! This crate provides the [`checked!`] macro, which rewrites an ordinary
//! arithmetic expression into a chain of the standard library's checked
//! arithmetic methods (`checked_add`, `checked_sub`, `checked_mul`,
//! `checked_div`, `checked_rem`), recursively, while preserving the operator
//! precedence and grouping of the original expression.
//!
//! You normally do not depend on this crate directly; the macro is re-exported
//! from `financial-ops` as `financial_ops::checked`.

use proc_macro::TokenStream;
use proc_macro2::{Spacing, Span, TokenStream as TokenStream2, TokenTree};
use quote::{format_ident, quote};
use syn::{BinOp, Expr};

/// Recursively rewrites an arithmetic expression into checked arithmetic.
///
/// The macro walks the parsed expression tree, so it honors normal Rust
/// operator precedence and parentheses. Every binary operator is replaced by
/// its checked counterpart and the operands are threaded together so each
/// sub-expression is evaluated exactly once, in left-to-right order.
///
/// # Result vs. Option
///
/// * Without an error, the macro evaluates to an `Option<T>` (it is `None` as
///   soon as any step overflows or divides by zero):
///
/// ```ignore
/// let total: Option<u64> = checked! { a + b * c };
/// ```
///
/// * With a trailing `@ <error expression>`, the macro evaluates to a
///   `Result<T, E>`, mapping the first failing step to the given error:
///
/// ```ignore
/// let total = checked! { a + b * c @ MyError::Overflow }?;
/// ```
///
/// # Supported operators
///
/// `+` → `checked_add`, `-` → `checked_sub`, `*` → `checked_mul`,
/// `/` → `checked_div`, `%` → `checked_rem`.
///
/// # Examples
///
/// ```ignore
/// use financial_ops::checked;
///
/// // Respects precedence: this is `a + (b * c)`, fully checked.
/// let value = checked! { 2u64 + 3 * 4 };
/// assert_eq!(value, Some(14));
///
/// // Overflow short-circuits to `None`.
/// assert_eq!(checked! { u8::MAX + 1u8 }, None);
/// ```
#[proc_macro]
pub fn checked(input: TokenStream) -> TokenStream {
    let input2: TokenStream2 = input.into();
    match parse_input(input2) {
        Ok((expr, error)) => match expand(&expr, error.as_ref(), 0) {
            Ok(tokens) => tokens.into(),
            Err(err) => err.to_compile_error().into(),
        },
        Err(err) => err.to_compile_error().into(),
    }
}

/// Splits the macro input into the arithmetic expression and an optional error
/// expression separated by a top-level `@`.
fn parse_input(tokens: TokenStream2) -> syn::Result<(Expr, Option<Expr>)> {
    let mut expr_tokens = TokenStream2::new();
    let mut error_tokens = TokenStream2::new();
    let mut found_at = false;

    for tt in tokens {
        if !found_at {
            if let TokenTree::Punct(ref punct) = tt {
                if punct.as_char() == '@' && punct.spacing() == Spacing::Alone {
                    found_at = true;
                    continue;
                }
            }
            expr_tokens.extend(std::iter::once(tt));
        } else {
            error_tokens.extend(std::iter::once(tt));
        }
    }

    if expr_tokens.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "expected an arithmetic expression",
        ));
    }

    let expr: Expr = syn::parse2(expr_tokens)?;

    let error = if found_at {
        if error_tokens.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "expected an error expression after `@`",
            ));
        }
        Some(syn::parse2::<Expr>(error_tokens)?)
    } else {
        None
    };

    Ok((expr, error))
}

/// Recursively expands `expr` into a checked-arithmetic token stream.
///
/// `depth` is used to generate non-colliding binding identifiers: a node at
/// depth `d` binds `__l{d}` / `__r{d}`, and its children recurse at `d + 1`, so
/// a child's bindings can never shadow an ancestor's.
fn expand(expr: &Expr, error: Option<&Expr>, depth: usize) -> syn::Result<TokenStream2> {
    match expr {
        Expr::Binary(binary) => {
            let method = method_name(&binary.op)?;
            let lhs = expand(&binary.left, error, depth + 1)?;
            let rhs = expand(&binary.right, error, depth + 1)?;

            let l = format_ident!("__l{}", depth);
            let r = format_ident!("__r{}", depth);

            let combine = match error {
                Some(err) => quote! { #l.#method(#r).ok_or(#err) },
                None => quote! { #l.#method(#r) },
            };

            Ok(quote! {
                (#lhs).and_then(|#l| (#rhs).and_then(|#r| #combine))
            })
        }
        // Transparent wrappers: keep recursing without consuming a depth level.
        Expr::Paren(paren) => expand(&paren.expr, error, depth),
        Expr::Group(group) => expand(&group.expr, error, depth),
        // Any other expression is a leaf value to be lifted into the chain.
        leaf => Ok(match error {
            Some(_) => quote! { ::core::result::Result::<_, _>::Ok(#leaf) },
            None => quote! { ::core::option::Option::Some(#leaf) },
        }),
    }
}

/// Maps a binary operator to its checked-method identifier.
fn method_name(op: &BinOp) -> syn::Result<proc_macro2::Ident> {
    let name = match op {
        BinOp::Add(_) => "checked_add",
        BinOp::Sub(_) => "checked_sub",
        BinOp::Mul(_) => "checked_mul",
        BinOp::Div(_) => "checked_div",
        BinOp::Rem(_) => "checked_rem",
        other => {
            return Err(syn::Error::new_spanned(
                other,
                "`checked!` only supports the `+`, `-`, `*`, `/`, and `%` operators",
            ));
        }
    };
    Ok(format_ident!("{}", name))
}
