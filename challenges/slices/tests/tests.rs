use slices::find_largest_in_slice;

#[test]
fn test_largest_in_non_empty_slice() {
    let numbers = [10, 3, 7, 2, 99];
    assert_eq!(find_largest_in_slice(&numbers), Some(99));
}

#[test]
fn test_empty_slice() {
    let empty: [i32; 0] = [];
    assert_eq!(find_largest_in_slice(&empty), None);
}

#[test]
fn test_single_element_slice() {
    let single = [42];
    assert_eq!(find_largest_in_slice(&single), Some(42));
}

#[test]
fn test_negative_numbers() {
    let negatives = [-10, -3, -7, -2, -99];
    assert_eq!(find_largest_in_slice(&negatives), Some(-2));
}
