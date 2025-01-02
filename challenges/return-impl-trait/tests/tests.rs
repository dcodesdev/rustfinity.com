use return_impl_trait::*;

#[test]
fn test_filter_even_numbers() {
    let input = [1, 2, 3, 4, 5, 6];
    let result: Vec<&i32> = filter_even_numbers(&input).collect();
    assert_eq!(result, vec![&2, &4, &6]);
}

#[test]
fn test_filter_even_numbers_empty() {
    let input: [i32; 0] = [];
    let result: Vec<&i32> = filter_even_numbers(&input).collect();
    assert!(result.is_empty());
}

#[test]
fn test_filter_even_numbers_no_evens() {
    let input = [1, 3, 5, 7];
    let result: Vec<&i32> = filter_even_numbers(&input).collect();
    assert!(result.is_empty());
}

#[test]
fn test_filter_even_numbers_all_evens() {
    let input = [2, 4, 6, 8];
    let result: Vec<&i32> = filter_even_numbers(&input).collect();
    assert_eq!(result, vec![&2, &4, &6, &8]);
}

#[test]
fn test_filter_even_numbers_negative_and_positive() {
    let input = [-3, -2, -1, 0, 1, 2];
    let result: Vec<&i32> = filter_even_numbers(&input).collect();
    assert_eq!(result, vec![&-2, &0, &2]);
}

#[test]
fn test_filter_even_numbers_large_numbers() {
    let input = [1000, 2001, 3002, 4003, 5004];
    let result: Vec<&i32> = filter_even_numbers(&input).collect();
    assert_eq!(result, vec![&1000, &3002, &5004]);
}

#[test]
fn test_filter_even_numbers_with_duplicates() {
    let input = [2, 2, 3, 3, 4, 4];
    let result: Vec<&i32> = filter_even_numbers(&input).collect();
    assert_eq!(result, vec![&2, &2, &4, &4]);
}

#[test]
fn test_filter_even_numbers_large_range() {
    let input: Vec<i32> = (1..=100).collect();
    let result: Vec<&i32> = filter_even_numbers(&input).collect();
    assert_eq!(
        result,
        (2..=100)
            .step_by(2)
            .collect::<Vec<i32>>()
            .iter()
            .collect::<Vec<&i32>>()
    );
}
