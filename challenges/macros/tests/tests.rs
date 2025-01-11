use macros::*;

// Basic operations
#[test]
fn test_addition_format() {
    assert_eq!(math_operations!(4, "+", 2), "4 + 2 = 6");
}

#[test]
fn test_subtraction_format() {
    assert_eq!(math_operations!(4, "-", 2), "4 - 2 = 2");
}

#[test]
fn test_multiplication_format() {
    assert_eq!(math_operations!(4, "*", 2), "4 * 2 = 8");
}

#[test]
fn test_division_format() {
    assert_eq!(math_operations!(4, "/", 2), "4 / 2 = 2");
}

// Zero operations
#[test]
fn test_add_zero() {
    assert_eq!(math_operations!(5, "+", 0), "5 + 0 = 5");
}

#[test]
fn test_subtract_zero() {
    assert_eq!(math_operations!(5, "-", 0), "5 - 0 = 5");
}

#[test]
fn test_multiply_by_zero() {
    assert_eq!(math_operations!(5, "*", 0), "5 * 0 = 0");
}

// Negative numbers
#[test]
fn test_negative_addition() {
    assert_eq!(math_operations!(-4, "+", 2), "-4 + 2 = -2");
}

#[test]
fn test_negative_multiplication() {
    assert_eq!(math_operations!(-3, "*", -2), "-3 * -2 = 6");
}

// Larger numbers
#[test]
fn test_large_numbers() {
    assert_eq!(math_operations!(1000, "*", 2000), "1000 * 2000 = 2000000");
}

// Same number operations
#[test]
fn test_same_number_operations() {
    assert_eq!(math_operations!(5, "+", 5), "5 + 5 = 10");
    assert_eq!(math_operations!(5, "-", 5), "5 - 5 = 0");
    assert_eq!(math_operations!(5, "*", 5), "5 * 5 = 25");
    assert_eq!(math_operations!(5, "/", 5), "5 / 5 = 1");
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
