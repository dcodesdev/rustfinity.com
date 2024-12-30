use vectors::*;

#[test]
fn test_add_elements() {
    let mut vec = vec![1, 2, 3];
    add_elements(&mut vec, &[4, 5]);
    assert_eq!(vec, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_remove_element_valid_index() {
    let mut vec = vec![1, 2, 3, 4, 5];
    remove_element(&mut vec, 2);
    assert_eq!(vec, vec![1, 2, 4, 5]);
}

#[test]
fn test_remove_element_invalid_index() {
    let mut vec = vec![1, 2, 3];
    remove_element(&mut vec, 10); // Should do nothing
    assert_eq!(vec, vec![1, 2, 3]);
}

#[test]
fn test_get_element_valid_index() {
    let vec = vec![1, 2, 3];
    assert_eq!(get_element(&vec, 1), Some(2));
}

#[test]
fn test_get_element_invalid_index() {
    let vec = vec![1, 2, 3];
    assert_eq!(get_element(&vec, 10), None);
}
