use slice_manipulation::*;

#[test]
fn test_update_valid_indices() {
    let mut data = vec![1, 2, 3, 4, 5];
    update_slice(&mut data, &[1, 3, 4], 7);
    assert_eq!(data, vec![1, 7, 3, 7, 7]);
}

#[test]
fn test_update_with_out_of_bounds_indices() {
    let mut data = vec![10, 20, 30];
    update_slice(&mut data, &[2, 5], 100); // Index 5 is out of bounds
    assert_eq!(data, vec![10, 20, 100]);
}

#[test]
fn test_update_empty_indices() {
    let mut data = vec![5, 15, 25];
    update_slice(&mut data, &[], 50); // No indices provided
    assert_eq!(data, vec![5, 15, 25]); // No changes made
}

#[test]
fn test_update_all_indices() {
    let mut data = vec![0, 0, 0];
    update_slice(&mut data, &[0, 1, 2], 42);
    assert_eq!(data, vec![42, 42, 42]);
}

#[test]
fn test_update_no_valid_indices() {
    let mut data = vec![8, 9, 10];
    update_slice(&mut data, &[3, 4, 5], 99); // All indices are out of bounds
    assert_eq!(data, vec![8, 9, 10]); // No changes made
}
