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

#[test]
fn test_longest_empty_both() {
    let s1 = "";
    let s2 = "";
    assert_eq!(longest(s1, s2), s1);
}

#[test]
fn test_longest_with_substring() {
    let s1 = "substring";
    let s2 = "sub";
    assert_eq!(longest(s1, s2), s1);
}

#[test]
fn test_longest_dynamically_allocated_strings() {
    let s1 = String::from("dynamic one");
    let s2 = String::from("dynamic two longer");
    assert_eq!(longest(&s1, &s2), &s2);
}

#[test]
fn test_longest_mixed_static_and_dynamic() {
    let s1 = "static string";
    let s2 = String::from("dynamic and longer string");
    assert_eq!(longest(s1, &s2), &s2);
}

#[test]
fn test_longest_numbers_as_strings() {
    let s1 = "12345";
    let s2 = "67890";
    assert_eq!(longest(s1, s2), s1); // They are equal in length, so s1 should be returned.
}

#[test]
fn test_longest_with_whitespace_strings() {
    let s1 = "   ";
    let s2 = " ";
    assert_eq!(longest(s1, s2), s1);
}

#[test]
fn test_longest_special_characters() {
    let s1 = "@@!!";
    let s2 = "@@@";
    assert_eq!(longest(s1, s2), s1);
}

#[test]
fn test_longest_unicode_strings() {
    let s1 = "こんにちは"; // Japanese for "Hello"
    let s2 = "안녕하세요"; // Korean for "Hello"
    assert_eq!(longest(s1, s2), s1); // Both are the same length, return s1.
}
