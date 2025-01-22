use structs_and_references::*;

#[test]
fn test_find_first() {
    let text = "Line one\nLine two with keyword\nAnother line";
    let finder = TextFinder::new(text);

    assert_eq!(finder.find_first("keyword"), Some("Line two with keyword"));
    assert_eq!(finder.find_first("missing"), None);
    assert_eq!(finder.find_first("Line"), Some("Line one"));
}

#[test]
fn test_find_many() {
    let text = "First line with keyword\nSecond line\nAnother keyword line\nFinal line";
    let finder = TextFinder::new(text);

    let results = finder.find_many("keyword");
    assert_eq!(
        results,
        vec!["First line with keyword", "Another keyword line"]
    );

    let empty_results = finder.find_many("nonexistent");
    assert!(empty_results.is_empty());
}

#[test]
fn test_find_many_edge_cases() {
    let text = "keyword in first line\nkeyword\nanother keyword here\nno match here";
    let finder = TextFinder::new(text);

    let results = finder.find_many("keyword");
    assert_eq!(
        results,
        vec!["keyword in first line", "keyword", "another keyword here"]
    );
}

#[test]
fn test_case_sensitivity() {
    let text = "Keyword in line one\nanother Keyword\ncase sensitivity test";
    let finder = TextFinder::new(text);

    assert_eq!(finder.find_first("Keyword"), Some("Keyword in line one"));
    assert_eq!(finder.find_first("keyword"), None);

    let results = finder.find_many("Keyword");
    assert_eq!(results, vec!["Keyword in line one", "another Keyword"]);
}

#[test]
fn test_empty_text() {
    let text = "";
    let finder = TextFinder::new(text);

    assert_eq!(finder.find_first("keyword"), None);

    let results = finder.find_many("keyword");
    assert!(results.is_empty());
}

#[test]
fn test_empty_keyword() {
    let text = "First line\nSecond line with keyword\nThird line";
    let finder = TextFinder::new(text);

    assert_eq!(finder.find_first(""), Some("First line"));

    let results = finder.find_many("");
    assert_eq!(
        results,
        vec!["First line", "Second line with keyword", "Third line"]
    );
}

#[test]
fn test_multiple_matches_in_one_line() {
    let text = "Keyword Keyword Keyword\nAnother line";
    let finder = TextFinder::new(text);

    assert_eq!(
        finder.find_first("Keyword"),
        Some("Keyword Keyword Keyword")
    );

    let results = finder.find_many("Keyword");
    assert_eq!(results, vec!["Keyword Keyword Keyword"]);
}

#[test]
fn test_no_newline_in_text() {
    let text = "Single line with Keyword";
    let finder = TextFinder::new(text);

    assert_eq!(
        finder.find_first("Keyword"),
        Some("Single line with Keyword")
    );

    let results = finder.find_many("Keyword");
    assert_eq!(results, vec!["Single line with Keyword"]);
}

#[test]
fn test_lines_with_only_whitespace() {
    let text = "   \nLine with keyword\n   \nAnother keyword line\n";
    let finder = TextFinder::new(text);

    assert_eq!(finder.find_first("keyword"), Some("Line with keyword"));

    let results = finder.find_many("keyword");
    assert_eq!(results, vec!["Line with keyword", "Another keyword line"]);
}

#[test]
fn test_whitespace_in_keyword() {
    let text = "Line with a space\nKeyword in a line\n";
    let finder = TextFinder::new(text);

    assert_eq!(finder.find_first("a "), Some("Line with a space"));

    let results = finder.find_many("a ");
    assert_eq!(results, vec!["Line with a space", "Keyword in a line"]);
}
