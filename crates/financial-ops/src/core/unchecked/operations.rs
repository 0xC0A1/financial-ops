use std::ops::{Add, Div, Mul, Rem, Sub};

/// A trait for performing decimal operations.
pub trait DecimalOperations {
    /// Adds two values with different decimal precisions.
    ///
    /// # Arguments
    ///
    /// * `self` - The first value.
    /// * `other` - The second value.
    /// * `self_decimals` - The number of decimal places in the first value.
    /// * `other_decimals` - The number of decimal places in the second value.
    ///
    /// # Returns
    ///
    /// A tuple containing the result of the addition and the number of decimal places in the result.
    fn add_decimals(self, other: Self, self_decimals: u32, other_decimals: u32) -> (Self, u32)
    where
        Self: Sized;

    /// Subtracts two values with different decimal precisions.
    ///
    /// # Arguments
    ///
    /// * `self` - The first value.
    /// * `other` - The second value.
    /// * `self_decimals` - The number of decimal places in the first value.
    /// * `other_decimals` - The number of decimal places in the second value.
    ///
    /// # Returns
    ///
    /// A tuple containing the result of the subtraction and the number of decimal places in the result.
    fn sub_decimals(self, other: Self, self_decimals: u32, other_decimals: u32) -> (Self, u32)
    where
        Self: Sized;

    /// Multiplies two values with different decimal precisions.
    ///
    /// # Arguments
    ///
    /// * `self` - The first value.
    /// * `other` - The second value.
    /// * `self_decimals` - The number of decimal places in the first value.
    /// * `other_decimals` - The number of decimal places in the second value.
    ///
    /// # Returns
    ///
    /// A tuple containing the result of the multiplication and the number of decimal places in the result.
    fn multiply_decimals(self, other: Self, self_decimals: u32, other_decimals: u32) -> (Self, u32)
    where
        Self: Sized;

    /// Divides two values with different decimal precisions.
    ///
    /// # Arguments
    ///
    /// * `self` - The first value.
    /// * `other` - The second value.
    /// * `self_decimals` - The number of decimal places in the first value.
    /// * `other_decimals` - The number of decimal places in the second value.
    ///
    /// # Returns
    ///
    /// A tuple containing the result of the division and the number of decimal places in the result.
    fn divide_decimals(self, other: Self, self_decimals: u32, other_decimals: u32) -> (Self, u32)
    where
        Self: Sized;

    /// Calculates the remainder of dividing two values with different decimal precisions.
    ///
    /// # Arguments
    ///
    /// * `self` - The first value.
    /// * `other` - The second value.
    /// * `self_decimals` - The number of decimal places in the first value.
    /// * `other_decimals` - The number of decimal places in the second value.
    ///
    /// # Returns
    ///
    /// A tuple containing the remainder of the division and the number of decimal places in the result.
    fn rem_decimals(self, other: Self, self_decimals: u32, other_decimals: u32) -> (Self, u32)
    where
        Self: Sized;
}

// Blanket implementation of `DecimalOperations` for all types implementing numeric operations.
impl<T> DecimalOperations for T
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + From<u32>,
{
    fn add_decimals(self, other: Self, self_decimals: u32, other_decimals: u32) -> (Self, u32) {
        if self_decimals > other_decimals {
            let factor = T::from(10u32.pow(self_decimals - other_decimals));
            (self + other * factor, self_decimals)
        } else {
            let factor = T::from(10u32.pow(other_decimals - self_decimals));
            (self * factor + other, other_decimals)
        }
    }

    fn sub_decimals(self, other: Self, self_decimals: u32, other_decimals: u32) -> (Self, u32) {
        if self_decimals > other_decimals {
            let factor = T::from(10u32.pow(self_decimals - other_decimals));
            (self - other * factor, self_decimals)
        } else {
            let factor = T::from(10u32.pow(other_decimals - self_decimals));
            (self * factor - other, other_decimals)
        }
    }

    fn multiply_decimals(
        self,
        other: Self,
        self_decimals: u32,
        other_decimals: u32,
    ) -> (Self, u32) {
        (self * other, self_decimals + other_decimals)
    }

    fn divide_decimals(self, other: Self, self_decimals: u32, other_decimals: u32) -> (Self, u32) {
        let factor = T::from(10u32.pow(other_decimals));
        let adjusted_value = self * factor;
        (adjusted_value / other, self_decimals)
    }

    fn rem_decimals(self, other: Self, self_decimals: u32, _other_decimals: u32) -> (Self, u32) {
        let factor = T::from(10u32.pow(self_decimals));
        let adjusted_value = self * factor;
        (adjusted_value % other, self_decimals)
    }
}

#[cfg(test)]
// Digits are grouped to reflect monetary notation (e.g. `123_45` = $123.45).
#[allow(clippy::inconsistent_digit_grouping, clippy::zero_prefixed_literal)]
mod tests {
    use super::*;

    #[test]
    fn test_add_decimals() {
        let a: u64 = 1_0000;
        let a_decimals = 4;
        let b: u64 = 2_00;
        let b_decimals = 2;

        let (result, decimals) = a.add_decimals(b, a_decimals, b_decimals);
        assert_eq!(result, 3_0000);
        assert_eq!(decimals, 4);

        let a: u32 = 123_45;
        let a_decimals = 2;
        let b: u32 = 0_45;
        let b_decimals = 2;

        let (result, decimals) = a.add_decimals(b, a_decimals, b_decimals);
        assert_eq!(result, 123_90);
        assert_eq!(decimals, 2);
    }

    #[test]
    fn test_sub_decimals() {
        let a: u64 = 3_0000;
        let a_decimals = 4;
        let b: u64 = 2_00;
        let b_decimals = 2;

        let (result, decimals) = a.sub_decimals(b, a_decimals, b_decimals);
        assert_eq!(result, 1_0000);
        assert_eq!(decimals, 4);

        let a: u32 = 123_45;
        let a_decimals = 2;
        let b: u32 = 0_45;
        let b_decimals = 2;

        let (result, decimals) = a.sub_decimals(b, a_decimals, b_decimals);
        assert_eq!(result, 123_00);
        assert_eq!(decimals, 2);
    }

    #[test]
    fn test_mul_decimals() {
        let a: u64 = 3_0000;
        let a_decimals = 4;
        let b: u64 = 2_00;
        let b_decimals = 2;

        let (result, decimals) = a.multiply_decimals(b, a_decimals, b_decimals);
        assert_eq!(result, 6_000000);
        assert_eq!(decimals, 6);

        let a: u32 = 12345;
        let a_decimals = 2;
        let b: u32 = 45;
        let b_decimals = 2;

        let (result, decimals) = a.multiply_decimals(b, a_decimals, b_decimals);
        assert_eq!(result, 555525);
        assert_eq!(decimals, 4);
    }

    #[test]
    fn test_div_decimals() {
        let a: u64 = 6_0000;
        let a_decimals = 4;
        let b: u64 = 2_00;
        let b_decimals = 2;

        let (result, decimals) = a.divide_decimals(b, a_decimals, b_decimals);
        assert_eq!(result, 3_0000);
        assert_eq!(decimals, 4);

        let a: u32 = 123_45;
        let a_decimals = 2;
        let b: u32 = 0_45;
        let b_decimals = 2;

        let (result, decimals) = a.divide_decimals(b, a_decimals, b_decimals);
        assert_eq!(result, 27433);
        assert_eq!(decimals, 2);
    }

    #[test]
    fn test_rem_decimals() {
        let a: u64 = 6_0000;
        let a_decimals = 4;
        let b: u64 = 2_00;
        let b_decimals = 2;

        let (result, decimals) = a.rem_decimals(b, a_decimals, b_decimals);
        assert_eq!(result, 0);
        assert_eq!(decimals, 4);

        let a: u32 = 123_45;
        let a_decimals = 2;
        let b: u32 = 0_45;
        let b_decimals = 2;

        let (result, decimals) = a.rem_decimals(b, a_decimals, b_decimals);
        assert_eq!(result, 15);
        assert_eq!(decimals, 2);
    }
}
