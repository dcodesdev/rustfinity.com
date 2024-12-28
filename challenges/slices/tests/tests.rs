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

#[test]
fn test_mixed_positive_and_negative() {
    let mixed = [-10, 20, -30, 40, -50];
    assert_eq!(find_largest_in_slice(&mixed), Some(40));
}

#[test]
fn test_all_equal_elements() {
    let equal = [5, 5, 5, 5];
    assert_eq!(find_largest_in_slice(&equal), Some(5));
}

#[test]
fn test_large_numbers() {
    let large_numbers = [1_000_000, 2_000_000, 3_000_000];
    assert_eq!(find_largest_in_slice(&large_numbers), Some(3_000_000));
}

#[test]
fn test_edge_case_with_min_and_max_i32() {
    let edge_cases = [i32::MIN, 0, i32::MAX];
    assert_eq!(find_largest_in_slice(&edge_cases), Some(i32::MAX));
}

#[test]
fn test_repeated_pattern_slice() {
    let pattern = [1, 3, 2, 1, 3, 2, 1, 3, 2];
    assert_eq!(find_largest_in_slice(&pattern), Some(3));
}

#[test]
fn test_slice_with_zero() {
    let with_zero = [-1, 0, 1];
    assert_eq!(find_largest_in_slice(&with_zero), Some(1));
}

#[test]
fn test_largest_at_end() {
    let end_large = [10, 20, 30, 40, 50];
    assert_eq!(find_largest_in_slice(&end_large), Some(50));
}

#[test]
fn test_largest_at_start() {
    let start_large = [99, 10, 20, 30];
    assert_eq!(find_largest_in_slice(&start_large), Some(99));
}
