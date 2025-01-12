use option_to_result::*;

#[test]
fn test_get_first_element_valid() {
    let numbers = vec![20, 30, 40, 50];
    assert_eq!(get_first_element(numbers, 15), Ok(20));
}

#[test]
fn test_get_first_element_empty_vector() {
    let numbers: Vec<i32> = vec![];
    assert_eq!(
        get_first_element(numbers, 15),
        Err("Vector is empty".to_string())
    );
}

#[test]
fn test_get_first_element_below_min_value() {
    let numbers = vec![10, 20, 30, 40];
    assert_eq!(
        get_first_element(numbers, 15),
        Err("First element is below the minimum allowed value".to_string())
    );
}

#[test]
fn test_get_first_element_edge_case_valid() {
    let numbers = vec![15, 20, 30];
    assert_eq!(get_first_element(numbers, 15), Ok(15));
}

#[test]
fn test_get_first_element_negative_values() {
    let numbers = vec![-5, -10, 0];
    assert_eq!(get_first_element(numbers, -10), Ok(-5));
}

#[test]
fn test_get_first_element_large_min_value() {
    let numbers = vec![1000, 2000, 3000];
    assert_eq!(
        get_first_element(numbers, 5000),
        Err("First element is below the minimum allowed value".to_string())
    );
}
