use converting_numerical_types::*;

#[test]
fn test_converts_numbers() {
    assert_eq!(numerical_type_conversion(42i32), 42u32);
    assert_eq!(numerical_type_conversion(0i32), 0u32);
    assert_eq!(numerical_type_conversion(-1i32), 4294967295u32);
    assert_eq!(numerical_type_conversion(1_000_000i32), 1_000_000u32);
}
