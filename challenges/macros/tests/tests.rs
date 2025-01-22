use macros::*;

// Basic operations
#[test]
fn test_addition_format() {
    assert_eq!(
        math_operations!(4, "+", 2),
        "4 + 2 = 6",
        "Addition operation failed to format correctly"
    );
}

#[test]
fn test_subtraction_format() {
    assert_eq!(
        math_operations!(4, "-", 2),
        "4 - 2 = 2",
        "Subtraction operation failed to format correctly"
    );
}

#[test]
fn test_multiplication_format() {
    assert_eq!(
        math_operations!(4, "*", 2),
        "4 * 2 = 8",
        "Multiplication operation failed to format correctly"
    );
}

#[test]
fn test_division_format() {
    assert_eq!(
        math_operations!(4, "/", 2),
        "4 / 2 = 2",
        "Division operation failed to format correctly"
    );
}

// Zero operations
#[test]
fn test_add_zero() {
    assert_eq!(
        math_operations!(5, "+", 0),
        "5 + 0 = 5",
        "Adding zero failed to format correctly"
    );
}

#[test]
fn test_subtract_zero() {
    assert_eq!(
        math_operations!(5, "-", 0),
        "5 - 0 = 5",
        "Subtracting zero failed to format correctly"
    );
}

#[test]
fn test_multiply_by_zero() {
    assert_eq!(
        math_operations!(5, "*", 0),
        "5 * 0 = 0",
        "Multiplication by zero failed to format correctly"
    );
}

// Negative numbers
#[test]
fn test_negative_addition() {
    assert_eq!(
        math_operations!(-4, "+", 2),
        "-4 + 2 = -2",
        "Addition with negative number failed to format correctly"
    );
}

#[test]
fn test_negative_multiplication() {
    assert_eq!(
        math_operations!(-3, "*", -2),
        "-3 * -2 = 6",
        "Multiplication with negative numbers failed to format correctly"
    );
}

// Larger numbers
#[test]
fn test_large_numbers() {
    assert_eq!(
        math_operations!(1000, "*", 2000),
        "1000 * 2000 = 2000000",
        "Large number multiplication failed to format correctly"
    );
}

// Same number operations
#[test]
fn test_same_number_operations() {
    assert_eq!(
        math_operations!(5, "+", 5),
        "5 + 5 = 10",
        "Addition with same numbers failed to format correctly"
    );
    assert_eq!(
        math_operations!(5, "-", 5),
        "5 - 5 = 0",
        "Subtraction with same numbers failed to format correctly"
    );
    assert_eq!(
        math_operations!(5, "*", 5),
        "5 * 5 = 25",
        "Multiplication with same numbers failed to format correctly"
    );
    assert_eq!(
        math_operations!(5, "/", 5),
        "5 / 5 = 1",
        "Division with same numbers failed to format correctly"
    );
}

// Error cases
#[test]
#[should_panic(expected = "Division by zero")]
fn test_division_by_zero() {
    let _ = math_operations!(4, "/", 0);
}

#[test]
#[should_panic(expected = "Unsupported operator")]
fn test_invalid_operator() {
    let _ = math_operations!(4, "%", 2);
}
