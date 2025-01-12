use iterators::*;

#[test]
fn test_filter_even_numbers() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];
    let result = filter_even_numbers(numbers.into_iter());
    assert_eq!(result, vec![1, 3, 5, 7]);
}

#[test]
fn test_filter_even_numbers_all_odd() {
    let numbers = vec![1, 3, 5, 7, 9];
    let result = filter_even_numbers(numbers.into_iter());
    assert_eq!(result, vec![1, 3, 5, 7, 9]);
}

#[test]
fn test_filter_even_numbers_all_even() {
    let numbers = vec![2, 4, 6, 8, 10];
    let result = filter_even_numbers(numbers.into_iter());
    assert_eq!(result, vec![]);
}

#[test]
fn test_filter_even_numbers_with_negatives() {
    let numbers = vec![-3, -2, -1, 0, 1, 2];
    let result = filter_even_numbers(numbers.into_iter());
    assert_eq!(result, vec![-3, -1, 1]);
}

#[test]
fn test_filter_even_numbers_large_input() {
    let numbers: Vec<i32> = (1..=1000).collect();
    let result = filter_even_numbers(numbers.into_iter());
    assert_eq!(
        result,
        (1..=1000).filter(|x| x % 2 != 0).collect::<Vec<i32>>()
    );
}

#[test]
fn test_uppercase_strings() {
    let words = vec!["rust", "iterator", "challenge"];
    let result = uppercase_strings(words.iter().map(|&s| s));
    assert_eq!(result, vec!["RUST", "ITERATOR", "CHALLENGE"]);
}

#[test]
fn test_uppercase_strings_mixed_case() {
    let words = vec!["RuSt", "IterATOR", "ChAllEngE"];
    let result = uppercase_strings(words.iter().map(|&s| s));
    assert_eq!(result, vec!["RUST", "ITERATOR", "CHALLENGE"]);
}

#[test]
fn test_uppercase_strings_special_characters() {
    let words = vec!["hello!", "rust@", "123"];
    let result = uppercase_strings(words.iter().map(|&s| s));
    assert_eq!(result, vec!["HELLO!", "RUST@", "123"]);
}

#[test]
fn test_uppercase_strings_unicode() {
    let words = vec!["rust", "こんにちは", "добрый"];
    let result = uppercase_strings(words.iter().map(|&s| s));
    assert_eq!(result, vec!["RUST", "こんにちは", "ДОБРЫЙ"]);
}

#[test]
fn test_uppercase_strings_with_empty_string() {
    let words = vec!["hello", ""];
    let result = uppercase_strings(words.iter().map(|&s| s));
    assert_eq!(result, vec!["HELLO", ""]);
}

#[test]
fn test_empty_input() {
    let numbers: Vec<i32> = vec![];
    assert_eq!(filter_even_numbers(numbers.into_iter()), vec![]);

    let words: Vec<&str> = vec![];
    assert_eq!(uppercase_strings(words.into_iter()), Vec::<&str>::new());
}

#[test]
fn test_edge_cases() {
    let single_number = vec![2];
    assert_eq!(filter_even_numbers(single_number.into_iter()), vec![]);

    let single_word = vec!["single"];
    assert_eq!(
        uppercase_strings(single_word.into_iter()),
        vec!["SINGLE".to_string()]
    );
}
