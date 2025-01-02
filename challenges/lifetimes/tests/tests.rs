use lifetimes::longest;

#[test]
fn test_longest_different_lengths() {
    let s1 = "short";
    let s2 = "longer";
    assert_eq!(longest(s1, s2), s2);
}

#[test]
fn test_longest_equal_lengths() {
    let s1 = "same";
    let s2 = "same";
    assert_eq!(longest(s1, s2), s1);
}

#[test]
fn test_longest_static_str() {
    let s1: &'static str = "static";
    let s2: &'static str = "longer static";
    assert_eq!(longest(s1, s2), s2);
}

#[test]
fn test_longest_edge_case_empty() {
    let s1 = "";
    let s2 = "non-empty";
    assert_eq!(longest(s1, s2), s2);
}
