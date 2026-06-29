use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

/// Represents the possible errors that can occur during decimal operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecimalOperationError {
    /// Indicates that an overflow occurred during the operation.
    Overflow,
    /// Indicates that a division by zero occurred during the operation.
    DivisionByZero,
}

impl Display for DecimalOperationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DecimalOperationError::Overflow => {
                write!(f, "An overflow occurred during the operation.")
            }
            DecimalOperationError::DivisionByZero => {
                write!(f, "A division by zero occurred during the operation.")
            }
        }
    }
}

impl Error for DecimalOperationError {}
