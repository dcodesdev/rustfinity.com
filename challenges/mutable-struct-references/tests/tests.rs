use mutable_struct_references::*;

#[test]
fn test_get_text() {
    let mut text = String::from("Initial text\nAnother line");
    let finder = MutableTextFinder::new(&mut text);

    assert_eq!(finder.get_text(), "Initial text\nAnother line");
}

#[test]
fn test_get_text_after_modification() {
    let mut text = String::from("Rust is awesome\nReplace this line\nFinal line");
    let mut finder = MutableTextFinder::new(&mut text);

    finder.replace_lines("Replace", "This line has been replaced");
    assert_eq!(
        finder.get_text(),
        "Rust is awesome\nThis line has been replaced\nFinal line"
    );
}

#[test]
fn test_get_text_large_content() {
    let mut text = "Rust ".repeat(1000) + "\nKeyword line\n" + &"Rust ".repeat(1000);
    let mut finder = MutableTextFinder::new(&mut text);

    assert!(finder.get_text().contains("Keyword line"));
    finder.replace_lines("Keyword", "Replaced line");
    assert!(finder.get_text().contains("Replaced line"));
    assert!(!finder.get_text().contains("Keyword line"));
}

#[test]
fn test_get_text_after_find_first() {
    let mut text = String::from("Line one\nLine two with keyword\nAnother line");
    let finder = MutableTextFinder::new(&mut text);

    let first = finder.find_first("keyword");
    assert_eq!(first, Some("Line two with keyword"));
    assert_eq!(
        finder.get_text(),
        "Line one\nLine two with keyword\nAnother line"
    );
}

#[test]
fn test_get_text_after_multiple_replacements() {
    let mut text = String::from("Replace this line\nKeep this line\nReplace that line too");
    let mut finder = MutableTextFinder::new(&mut text);

    finder.replace_lines("Replace", "Line replaced");
    assert_eq!(
        finder.get_text(),
        "Line replaced\nKeep this line\nLine replaced"
    );

    finder.replace_lines("Keep", "Line kept");
    assert_eq!(finder.get_text(), "Line replaced\nLine kept\nLine replaced");
}

#[test]
fn test_get_text_with_special_characters() {
    let mut text = String::from("Line with emoji 😊\nAnother line 🚀\nFinal 🌟");
    let mut finder = MutableTextFinder::new(&mut text);

    assert_eq!(
        finder.get_text(),
        "Line with emoji 😊\nAnother line 🚀\nFinal 🌟"
    );
    finder.replace_lines("🚀", "Rocket");
    assert_eq!(finder.get_text(), "Line with emoji 😊\nRocket\nFinal 🌟");
}

#[test]
fn test_get_text_empty_string() {
    let mut text = String::new();
    let finder = MutableTextFinder::new(&mut text);

    assert_eq!(finder.get_text(), "");
}

#[test]
fn test_get_text_after_no_replacement() {
    let mut text = String::from("Line one\nLine two\nLine three");
    let mut finder = MutableTextFinder::new(&mut text);

    finder.replace_lines("missing", "replacement");
    assert_eq!(finder.get_text(), "Line one\nLine two\nLine three");
}

#[test]
fn test_get_text_combined_operations() {
    let mut text = String::from("Search line\nAnother line with keyword\nLast line");
    let mut finder = MutableTextFinder::new(&mut text);

    assert_eq!(
        finder.get_text(),
        "Search line\nAnother line with keyword\nLast line"
    );

    let first = finder.find_first("keyword");
    assert_eq!(first, Some("Another line with keyword"));
    assert_eq!(
        finder.get_text(),
        "Search line\nAnother line with keyword\nLast line"
    );

    finder.replace_lines("keyword", "found");
    assert_eq!(&text, "Search line\nfound\nLast line");
}
