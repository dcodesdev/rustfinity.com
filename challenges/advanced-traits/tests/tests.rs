use advanced_traits::*;

#[test]
fn test_add_millimeters_and_meters() {
    let mm = Millimeters(500);
    let m = Meters(2);

    let result = mm + m;
    assert_eq!(result.0, 2500);
}

#[test]
fn test_zero_addition() {
    let mm = Millimeters(0);
    let m = Meters(5);

    let result = mm + m;
    assert_eq!(result.0, 5000);
}

#[test]
fn test_large_numbers() {
    let mm = Millimeters(1_000_000);
    let m = Meters(2_000);

    let result = mm + m;
    assert_eq!(result.0, 3_000_000);
}
