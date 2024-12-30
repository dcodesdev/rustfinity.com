use vectors::*;

#[test]
fn test_add_elements_empty_vector() {
    let mut vec = Vec::new();
    add_elements(&mut vec, &[1, 2, 3]);
    assert_eq!(vec, vec![1, 2, 3]);
}

#[test]
fn test_add_elements_empty_slice() {
    let mut vec = vec![1, 2, 3];
    add_elements(&mut vec, &[]);
    assert_eq!(vec, vec![1, 2, 3]);
}

#[test]
fn test_add_elements_large_input() {
    let mut vec = vec![1];
    let large_input: Vec<i32> = (2..=1000).collect();
    add_elements(&mut vec, &large_input);
    assert_eq!(vec.len(), 1000);
    assert_eq!(vec[999], 1000);
}

#[test]
fn test_remove_element_first() {
    let mut vec = vec![1, 2, 3];
    remove_element(&mut vec, 0);
    assert_eq!(vec, vec![2, 3]);
}

#[test]
fn test_remove_element_last() {
    let mut vec = vec![1, 2, 3];
    remove_element(&mut vec, 2);
    assert_eq!(vec, vec![1, 2]);
}

#[test]
fn test_remove_element_out_of_bounds_high() {
    let mut vec = vec![1, 2, 3];
    remove_element(&mut vec, 10); // Should do nothing
    assert_eq!(vec, vec![1, 2, 3]);
}

#[test]
fn test_remove_element_out_of_bounds_negative() {
    let mut vec = vec![1, 2, 3];
    remove_element(&mut vec, usize::MAX); // Should do nothing
    assert_eq!(vec, vec![1, 2, 3]);
}

#[test]
fn test_get_element_empty_vector() {
    let vec: Vec<i32> = Vec::new();
    assert_eq!(get_element(&vec, 0), None);
}

#[test]
fn test_get_element_boundary_cases() {
    let vec = vec![1, 2, 3];
    assert_eq!(get_element(&vec, 0), Some(1)); // First element
    assert_eq!(get_element(&vec, 2), Some(3)); // Last element
    assert_eq!(get_element(&vec, 3), None); // Out of bounds
}

#[test]
fn test_combined_operations() {
    let mut vec = vec![10];
    add_elements(&mut vec, &[20, 30, 40]);
    assert_eq!(vec, vec![10, 20, 30, 40]);

    remove_element(&mut vec, 1);
    assert_eq!(vec, vec![10, 30, 40]);

    assert_eq!(get_element(&vec, 1), Some(30));
    assert_eq!(get_element(&vec, 3), None);
}
