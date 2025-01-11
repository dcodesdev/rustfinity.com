use advanced_traits::*;

#[test]
fn test_add_millimeters_and_meters() {
    let mm = Millimeters(500);
    let m = Meters(2);

    let result = mm + m;
    assert_eq!(result.0, 2500, "Expected 500mm + 2000mm to equal 2500mm");
}

#[test]
fn test_zero_addition() {
    let mm = Millimeters(0);
    let m = Meters(5);

    let result = mm + m;
    assert_eq!(result.0, 5000, "Expected 0mm + 5000mm to equal 5000mm");
}

#[test]
fn test_large_numbers() {
    let mm = Millimeters(1_000_000);
    let m = Meters(2_000);

    let result = mm + m;
    assert_eq!(
        result.0, 3_000_000,
        "Expected 1_000_000mm + 2_000_000mm to equal 3_000_000mm"
    );
}

#[test]
fn test_addition_with_zero_meters() {
    let mm = Millimeters(1000);
    let m = Meters(0);

    let result = mm + m;
    assert_eq!(result.0, 1000, "Expected 1000mm + 0mm to equal 1000mm");
}

#[test]
fn test_addition_with_zero_millimeters() {
    let mm = Millimeters(0);
    let m = Meters(10);

    let result = mm + m;
    assert_eq!(result.0, 10000, "Expected 0mm + 10000mm to equal 10000mm");
}

#[test]
fn test_addition_with_both_zero() {
    let mm = Millimeters(0);
    let m = Meters(0);

    let result = mm + m;
    assert_eq!(result.0, 0, "Expected 0mm + 0mm to equal 0mm");
}

#[test]
fn test_addition_with_large_meters() {
    let mm = Millimeters(500);
    let m = Meters(1_000_000);

    let result = mm + m;
    assert_eq!(
        result.0, 1_000_000_500,
        "Expected 500mm + 1_000_000_000mm to equal 1_000_000_500mm"
    );
}

#[test]
fn test_multiple_additions() {
    let mm = Millimeters(1000);
    let m1 = Meters(1);
    let m2 = Meters(2);

    let result = (mm + m1) + m2;
    assert_eq!(
        result.0, 4000,
        "Expected 1000mm + 1000mm + 2000mm to equal 4000mm"
    );
}
