/// Implements checked arithmetic operations for the specified types.
///
/// This macro generates implementations of the `CheckedAdd`, `CheckedSub`,
/// `CheckedMul`, `CheckedDiv`, and `CheckedRem` traits for the given types.
/// These traits provide methods for performing arithmetic operations that
/// return an `Option` containing the result, instead of panicking on overflow
/// or division by zero.
///
/// # Examples
///
// `impl_checked_arithmetic!(u8 u16 u32 u64 i8 i16 i32 i64);`
///
/// In this example, the `impl_checked_arithmetic` macro is used to generate
/// implementations of the `CheckedAdd`, `CheckedSub`, `CheckedMul`,
/// `CheckedDiv`, and `CheckedRem` traits for the types `u8`, `u16`, `u32`,
/// `u64`, `i8`, `i16`, `i32`, and `i64`.
///
/// The generated implementations provide methods like `checked_add`, `checked_sub`,
/// `checked_mul`, `checked_div`, and `checked_rem` that can be used to perform
/// arithmetic operations that return an `Option` containing the result.
///
/// # Safety
///
/// The generated implementations rely on the underlying arithmetic operations
/// provided by the types. It is important to ensure that the types being used
/// implement the necessary arithmetic operations correctly and handle overflow
/// and division by zero appropriately.
///
/// # Panics
///
/// The generated implementations do not panic on overflow or division by zero.
/// Instead, they return `None` to indicate that the operation could not be
/// performed without overflowing or dividing by zero.
#[macro_export]
macro_rules! impl_checked_arithmetic {
    ($($t:ty)*) => ($(
        impl $crate::core::CheckedAdd for $t {
            fn checked_add(&self, v: &Self) -> Option<Self> {
                <$t>::checked_add(*self, *v)
            }
        }
        impl $crate::core::CheckedSub for $t {
            fn checked_sub(&self, v: &Self) -> Option<Self> {
                <$t>::checked_sub(*self, *v)
            }
        }
        impl $crate::core::CheckedMul for $t {
            fn checked_mul(&self, v: &Self) -> Option<Self> {
                <$t>::checked_mul(*self, *v)
            }
        }
        impl $crate::core::CheckedDiv for $t {
            fn checked_div(&self, v: &Self) -> Option<Self> {
                <$t>::checked_div(*self, *v)
            }
        }
        impl $crate::core::CheckedRem for $t {
            fn checked_rem(&self, v: &Self) -> Option<Self> {
                <$t>::checked_rem(*self, *v)
            }
        }
    )*)
}
