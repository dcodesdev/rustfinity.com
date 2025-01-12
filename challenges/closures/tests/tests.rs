use closures::*;

#[test]
fn test_add_closure() {
    let (add_closure, _, _) = create_closures();
    assert_eq!(add_closure(3, 4), 7);
    assert_eq!(add_closure(-1, 1), 0);
    assert_eq!(add_closure(0, 0), 0);
}

#[test]
fn test_subtract_closure() {
    let (_, subtract_closure, _) = create_closures();
    assert_eq!(subtract_closure(10, 4), 6);
    assert_eq!(subtract_closure(4, 10), -6);
    assert_eq!(subtract_closure(0, 0), 0);
}

#[test]
fn test_multiply_closure() {
    let (_, _, multiply_closure) = create_closures();
    assert_eq!(multiply_closure(3, 5), 15);
    assert_eq!(multiply_closure(-2, 3), -6);
    assert_eq!(multiply_closure(0, 10), 0);
}
