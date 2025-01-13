use ownership_rules::*;

#[test]
fn test_calculate_and_modify() {
    let (s, length) = calculate_and_modify();
    assert_eq!(s, "hello, world");
    assert_eq!(length, 5);
}
