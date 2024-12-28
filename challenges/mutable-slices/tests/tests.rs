use mutable_slices::*;

#[test]
fn test_modify_elements_mixed() {
    let mut numbers = [1, 2, 3, 4, 5];
    modify_elements(&mut numbers);
    assert_eq!(numbers, [0, 4, 2, 8, 4]);
}

#[test]
fn test_modify_elements_even_only() {
    let mut numbers = [2, 4, 6];
    modify_elements(&mut numbers);
    assert_eq!(numbers, [4, 8, 12]);
}

#[test]
fn test_modify_elements_odd_only() {
    let mut numbers = [1, 3, 5];
    modify_elements(&mut numbers);
    assert_eq!(numbers, [0, 2, 4]);
}

#[test]
fn test_modify_elements_empty_slice() {
    let mut numbers: [i32; 0] = [];
    modify_elements(&mut numbers);
    assert_eq!(numbers, []);
}

#[test]
fn test_modify_elements_large_numbers() {
    let mut numbers = [100, 101, 102];
    modify_elements(&mut numbers);
    assert_eq!(numbers, [200, 100, 204]);
}
