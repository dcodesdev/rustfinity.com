use primitive_data_types::*;

#[test]
fn test_compiles() {
    let (x, y, z, a) = data_types();

    assert_eq!(x, 42, "Expected x to be 42");
    assert_eq!(y, 3.14, "Expected y to be 3.14");
    assert_eq!(z, false, "Expected z to be false");
    assert_eq!(a, 'a', "Expected a to be 'a'");
}
