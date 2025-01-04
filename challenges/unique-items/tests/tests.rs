use unique_items::*;

#[test]
fn test_unique_items_basic() {
    let items = vec![
        "abc123".to_string(),
        "  ".to_string(),
        "def456".to_string(),
        "abc123".to_string(),
        "ghi789".to_string(),
        "ghi789".to_string(),
        "   def456".to_string(),
    ];

    let unique_items = unique_items(items.into_iter());
    assert_eq!(unique_items, vec!["abc123", "def456", "ghi789"]);
}

#[test]
fn test_unique_items_with_empty_entries() {
    let items = vec![
        "abc123".to_string(),
        "".to_string(),
        " ".to_string(),
        "xyz987".to_string(),
        "abc123".to_string(),
    ];

    let unique_items = unique_items(items.into_iter());
    assert_eq!(unique_items, vec!["abc123", "xyz987"]);
}

#[test]
fn test_unique_items_all_duplicates() {
    let items = vec![
        "dup123".to_string(),
        "dup123".to_string(),
        "dup123".to_string(),
    ];

    let unique_items = unique_items(items.into_iter());
    assert_eq!(unique_items, vec!["dup123"]);
}

#[test]
fn test_unique_items_no_valid_entries() {
    let items = vec![" ".to_string(), "".to_string(), "\t".to_string()];

    let unique_items = unique_items(items.into_iter());
    assert_eq!(unique_items, Vec::<String>::new());
}

#[test]
fn test_unique_items_case_sensitivity() {
    let items = vec![
        "ABC123".to_string(),
        "abc123".to_string(),
        "ABC123".to_string(),
        "def456".to_string(),
        "DEF456".to_string(),
    ];

    let unique_items = unique_items(items.into_iter());
    assert_eq!(unique_items, vec!["ABC123", "DEF456", "abc123", "def456"]);
}

#[test]
fn test_unique_items_trim_and_sort() {
    let items = vec![
        "   xyz789".to_string(),
        "abc123   ".to_string(),
        "  def456  ".to_string(),
        "ghi789".to_string(),
    ];

    let unique_items = unique_items(items.into_iter());
    assert_eq!(unique_items, vec!["abc123", "def456", "ghi789", "xyz789"]);
}

#[test]
fn test_unique_items_with_symbols() {
    let items = vec![
        "!@#$%^".to_string(),
        "!@#$%^".to_string(),
        "123".to_string(),
        "abc".to_string(),
        "abc".to_string(),
        "   xyz  ".to_string(),
    ];

    let unique_items = unique_items(items.into_iter());
    assert_eq!(unique_items, vec!["!@#$%^", "123", "abc", "xyz"]);
}

#[test]
fn test_unique_items_mixed_whitespace() {
    let items = vec![
        "  ".to_string(),
        "\t".to_string(),
        "abc".to_string(),
        " abc ".to_string(),
        "\nabc\n".to_string(),
    ];

    let unique_items = unique_items(items.into_iter());
    assert_eq!(unique_items, vec!["abc"]);
}

#[test]
fn test_unique_items_with_str() {
    let items = vec!["abc", "  ", "def", "abc", "ghi", "ghi", "   def"];
    let unique_items = unique_items(items.into_iter());
    assert_eq!(unique_items, vec!["abc", "def", "ghi"]);
}
