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
