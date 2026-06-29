use financial_ops::{DecimalOperationError, checked};

#[test]
fn single_operation() {
    let a: u64 = 2;
    let b: u64 = 3;
    assert_eq!(checked! { a + b }, Some(5));
}

#[test]
fn left_associative_chain() {
    let a: u64 = 1;
    let b: u64 = 2;
    let c: u64 = 3;
    // ((1 + 2) + 3)
    assert_eq!(checked! { a + b + c }, Some(6));
}

#[test]
fn respects_operator_precedence() {
    let a: u64 = 2;
    let b: u64 = 3;
    let c: u64 = 4;
    // Must behave like `a + (b * c)`, fully checked, not `(a + b) * c`.
    assert_eq!(checked! { a + b * c }, Some(14));
}

#[test]
fn respects_parentheses() {
    let a: u64 = 2;
    let b: u64 = 3;
    let c: u64 = 4;
    assert_eq!(checked! { (a + b) * c }, Some(20));
}

#[test]
fn mixed_operators() {
    let a: u64 = 100;
    let b: u64 = 50;
    let c: u64 = 3;
    let d: u64 = 7;
    // 100 + 50 * 3 / 7 - 5  == 100 + (((50*3)/7)) - 5 == 100 + 21 - 5 == 116
    assert_eq!(checked! { a + b * c / d - 5u64 }, Some(116));
}

#[test]
fn operands_are_evaluated_once_in_order() {
    use std::cell::Cell;
    let log: Cell<u64> = Cell::new(0);
    let next = |n: u64| {
        // Encodes call order into the running value to prove single evaluation.
        log.set(log.get() * 10 + n);
        n
    };
    let result = checked! { next(1) + next(2) + next(3) };
    assert_eq!(result, Some(6));
    assert_eq!(log.get(), 123);
}

#[test]
fn overflow_yields_none() {
    assert_eq!(checked! { u8::MAX + 1u8 }, None);
}

#[test]
fn division_by_zero_yields_none() {
    let a: u64 = 10;
    let b: u64 = 0;
    assert_eq!(checked! { a / b }, None);
}

#[test]
fn result_form_ok() {
    let a: u64 = 2;
    let b: u64 = 2;
    let result: Result<u64, DecimalOperationError> =
        checked! { a + b @ DecimalOperationError::Overflow };
    assert_eq!(result, Ok(4));
}

#[test]
fn result_form_propagates_error() {
    let result: Result<u8, DecimalOperationError> =
        checked! { u8::MAX + 1u8 @ DecimalOperationError::Overflow };
    assert_eq!(result, Err(DecimalOperationError::Overflow));
}

#[test]
fn result_form_with_question_mark() -> Result<(), DecimalOperationError> {
    let a: u64 = 1_000;
    let b: u64 = 2_000;
    let total = checked! { a + b * 2u64 @ DecimalOperationError::Overflow }?;
    assert_eq!(total, 5_000);
    Ok(())
}
