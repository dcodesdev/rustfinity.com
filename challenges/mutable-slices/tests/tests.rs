use mutable_slices::*;

#[test]
fn test_transform_even_odd_mixed() {
    let mut numbers = [1, 2, 3, 4, 5];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [0, 4, 2, 8, 4]);
}

#[test]
fn test_transform_even_odd_even_only() {
    let mut numbers = [2, 4, 6];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [4, 8, 12]);
}

#[test]
fn test_transform_even_odd_odd_only() {
    let mut numbers = [1, 3, 5];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [0, 2, 4]);
}

#[test]
fn test_transform_even_odd_empty_slice() {
    let mut numbers: [i32; 0] = [];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, []);
}

#[test]
fn test_transform_even_odd_large_numbers() {
    let mut numbers = [100, 101, 102];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [200, 100, 204]);
}

#[test]
fn test_transform_even_odd_negative_numbers() {
    let mut numbers = [-1, -2, -3, -4];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [-2, -4, -4, -8]); // Odd numbers reduced by 1, even numbers doubled
}

#[test]
fn test_transform_even_odd_single_even() {
    let mut numbers = [8];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [16]); // 8 doubled
}

#[test]
fn test_transform_even_odd_single_odd() {
    let mut numbers = [7];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [6]); // 7 reduced by 1
}

#[test]
fn test_transform_even_odd_with_zeros() {
    let mut numbers = [0, 1, 2, 0];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [0, 0, 4, 0]); // 0 remains 0, odd numbers reduced by 1, even numbers doubled
}

#[test]
fn test_transform_even_odd_large_array() {
    let mut numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [0, 4, 2, 8, 4, 12, 6, 16, 8, 20]);
}

#[test]
fn test_transform_even_odd_alternating_even_odd() {
    let mut numbers = [1, 2, 3, 4, 5, 6];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [0, 4, 2, 8, 4, 12]);
}

#[test]
fn test_transform_even_odd_repeated_values() {
    let mut numbers = [2, 2, 2, 3, 3, 3];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [4, 4, 4, 2, 2, 2]); // All evens doubled, all odds reduced by 1
}

#[test]
fn test_transform_even_odd_all_zeroes() {
    let mut numbers = [0, 0, 0];
    transform_even_odd(&mut numbers);
    assert_eq!(numbers, [0, 0, 0]); // Zeros remain unchanged
}
