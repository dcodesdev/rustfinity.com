use option_propagation::*;

#[test]
fn test_find_and_multiply_valid_indices() {
    let numbers = vec![2, 4, 6, 8, 10];
    assert_eq!(find_and_multiply(numbers, 1, 3), Some(32));
}

#[test]
fn test_find_and_multiply_first_index_out_of_bounds() {
    let numbers = vec![2, 4, 6, 8, 10];
    assert_eq!(find_and_multiply(numbers, 10, 3), None);
}

#[test]
fn test_find_and_multiply_second_index_out_of_bounds() {
    let numbers = vec![2, 4, 6, 8, 10];
    assert_eq!(find_and_multiply(numbers, 1, 10), None);
}

#[test]
fn test_find_and_multiply_both_indices_out_of_bounds() {
    let numbers = vec![2, 4, 6, 8, 10];
    assert_eq!(find_and_multiply(numbers, 10, 20), None);
}

#[test]
fn test_find_and_multiply_empty_vector() {
    let numbers: Vec<i32> = vec![];
    assert_eq!(find_and_multiply(numbers, 0, 1), None);
}

#[test]
fn test_find_and_multiply_same_index() {
    let numbers = vec![3, 5, 7, 9];
    assert_eq!(find_and_multiply(numbers, 2, 2), Some(49)); // 7 * 7
}
