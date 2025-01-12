use return_impl_trait::*;

#[test]
fn test_filter_starts_with() {
    let input = vec![
        String::from("apple"),
        String::from("apricot"),
        String::from("banana"),
    ];
    let result: Vec<&String> = filter_starts_with(&input, "ap").collect();
    assert_eq!(
        result,
        vec![&String::from("apple"), &String::from("apricot")]
    );
}

#[test]
fn test_filter_starts_with_empty_slice() {
    let input: Vec<String> = vec![];
    let result: Vec<&String> = filter_starts_with(&input, "ap").collect();
    assert!(result.is_empty());
}

#[test]
fn test_filter_starts_with_no_matches() {
    let input = vec![
        String::from("banana"),
        String::from("cherry"),
        String::from("date"),
    ];
    let result: Vec<&String> = filter_starts_with(&input, "ap").collect();
    assert!(result.is_empty());
}

#[test]
fn test_filter_starts_with_partial_matches() {
    let input = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("apricot"),
        String::from("cherry"),
    ];
    let result: Vec<&String> = filter_starts_with(&input, "ap").collect();
    assert_eq!(
        result,
        vec![&String::from("apple"), &String::from("apricot")]
    );
}

#[test]
fn test_filter_starts_with_keyword_empty() {
    let input = vec![
        String::from("apple"),
        String::from("banana"),
        String::from("cherry"),
    ];
    let result: Vec<&String> = filter_starts_with(&input, "").collect();
    assert_eq!(
        result,
        vec![
            &String::from("apple"),
            &String::from("banana"),
            &String::from("cherry")
        ]
    );
}

#[test]
fn test_filter_starts_with_case_sensitivity() {
    let input = vec![
        String::from("Apple"),
        String::from("apricot"),
        String::from("banana"),
    ];
    let result: Vec<&String> = filter_starts_with(&input, "Ap").collect();
    assert_eq!(result, vec![&String::from("Apple")]);
}
