use optional_values::*;

#[test]
fn test_find_first_even_with_even_numbers() {
    let nums = vec![1, 3, 5, 8];
    assert_eq!(find_first_even(&nums), Some(8));
}

#[test]
fn test_find_first_even_no_even_numbers() {
    let nums = vec![1, 3, 5];
    assert_eq!(find_first_even(&nums), None);
}

#[test]
fn test_find_first_even_empty_list() {
    let nums: Vec<i32> = vec![];
    assert_eq!(find_first_even(&nums), None);
}

#[test]
fn test_find_first_even_all_even_numbers() {
    let nums = vec![2, 4, 6];
    assert_eq!(find_first_even(&nums), Some(2));
}

#[test]
fn test_find_first_even_single_even_number() {
    let nums = vec![7, 8, 9];
    assert_eq!(find_first_even(&nums), Some(8));
}

#[test]
fn test_find_first_even_single_element_even() {
    let nums = vec![4];
    assert_eq!(find_first_even(&nums), Some(4));
}

#[test]
fn test_find_first_even_single_element_odd() {
    let nums = vec![3];
    assert_eq!(find_first_even(&nums), None);
}

#[test]
fn test_find_first_even_mixed_with_negatives() {
    let nums = vec![-1, -2, -3, -4];
    assert_eq!(find_first_even(&nums), Some(-2));
}

#[test]
fn test_find_first_even_large_numbers() {
    let nums = vec![1_000_001, 1_000_002, 1_000_003];
    assert_eq!(find_first_even(&nums), Some(1_000_002));
}

#[test]
fn test_find_first_even_zeros() {
    let nums = vec![0, 1, 2, 3];
    assert_eq!(find_first_even(&nums), Some(0));
}

#[test]
fn test_find_first_even_multiple_of_ten() {
    let nums = vec![10, 20, 30];
    assert_eq!(find_first_even(&nums), Some(10));
}
