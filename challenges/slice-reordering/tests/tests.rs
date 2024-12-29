use slice_reordering_puzzle::*;

fn is_negative(n: i32) -> bool {
    n < 0
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[test]
fn test_reorder_and_pad_basic() {
    let mut data = [3, -2, 4, -5, 1];
    let count = reorder_and_pad(&mut data, is_negative, 0);
    assert_eq!(count, 2);
    assert_eq!(data, [-2, -5, 3, 4, 0]);
}

#[test]
fn test_reorder_and_pad_even_numbers() {
    let mut data = [1, 2, 3, 4, 5];
    let count = reorder_and_pad(&mut data, is_even, -1);
    assert_eq!(count, 2);
    assert_eq!(data, [2, 4, 1, 3, -1]);
}

#[test]
fn test_reorder_and_pad_all_group_one() {
    let mut data = [-1, -2, -3];
    let count = reorder_and_pad(&mut data, is_negative, 99);
    assert_eq!(count, 3);
    assert_eq!(data, [-1, -2, -3]);
}

#[test]
fn test_reorder_and_pad_all_group_two() {
    let mut data = [1, 2, 3];
    let count = reorder_and_pad(&mut data, is_negative, 99);
    assert_eq!(count, 0);
    assert_eq!(data, [99, 99, 99]);
}

#[test]
fn test_reorder_and_pad_mixed_values() {
    let mut data = [0, -1, 2, -3, 4];
    let count = reorder_and_pad(&mut data, is_negative, -99);
    assert_eq!(count, 2);
    assert_eq!(data, [-1, -3, 0, 2, -99]);
}
