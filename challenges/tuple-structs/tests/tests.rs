use tuple_structs::*;

#[test]
fn test_area() {
    let rect = Rectangle(4, 5);
    assert_eq!(area(&rect), 20);

    let square = Rectangle(10, 10);
    assert_eq!(area(&square), 100);
}

#[test]
fn test_area_zero() {
    let flat_rect = Rectangle(0, 5);
    assert_eq!(area(&flat_rect), 0);

    let empty_rect = Rectangle(0, 0);
    assert_eq!(area(&empty_rect), 0);
}

#[test]
fn test_large_values() {
    let large_rect = Rectangle(1_000, 2_000);
    assert_eq!(area(&large_rect), 2_000_000);
}

#[test]
fn test_edge_cases() {
    let tall_skinny = Rectangle(1, 100);
    assert_eq!(area(&tall_skinny), 100);

    let wide_flat = Rectangle(100, 1);
    assert_eq!(area(&wide_flat), 100);
}
