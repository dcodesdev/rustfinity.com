use constants::*;

#[test]
fn compiles() {
    assert_eq!(main(), 100, "Expected main() to return 100");
}

#[test]
fn test_constants() {
    assert_eq!(MAX_SIZE, 100, "Expected MAX_SIZE to be 100");
}
