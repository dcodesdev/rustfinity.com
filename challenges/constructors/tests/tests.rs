use constructors::*;

#[test]
fn test_constructor_initialization() {
    let book = Book::new(
        "The Rust Programming Language",
        "Steve Klabnik and Carol Nichols",
        2019,
    );

    assert_eq!(book.title, "The Rust Programming Language");
    assert_eq!(book.author, "Steve Klabnik and Carol Nichols");
    assert_eq!(book.year, 2019);
}

#[test]
fn test_constructor_with_different_data() {
    let book = Book::new("Eloquent JavaScript", "Marijn Haverbeke", 2018);

    assert_eq!(book.title, "Eloquent JavaScript");
    assert_eq!(book.author, "Marijn Haverbeke");
    assert_eq!(book.year, 2018);
}

#[test]
fn test_constructor_handles_edge_case_year() {
    let book = Book::new("Old Book", "Unknown Author", -500);

    assert_eq!(book.title, "Old Book");
    assert_eq!(book.author, "Unknown Author");
    assert_eq!(book.year, -500);
}

#[test]
fn test_constructor_with_empty_title_and_author() {
    let book = Book::new("", "", 2023);

    assert_eq!(book.title, "");
    assert_eq!(book.author, "");
    assert_eq!(book.year, 2023);
}

#[test]
fn test_constructor_long_title_and_author() {
    let long_title = "A Very Long and Detailed Title for a Book That Goes On and On and On";
    let long_author =
        "A Very Long and Detailed Name of an Author Who Has an Incredibly Lengthy Name";
    let book = Book::new(long_title, long_author, 2024);

    assert_eq!(book.title, long_title);
    assert_eq!(book.author, long_author);
    assert_eq!(book.year, 2024);
}

#[test]
fn test_constructor_special_characters_in_title_and_author() {
    let special_title = "Rust ♥ Programming!";
    let special_author = "John \"Rustacean\" Doe";
    let book = Book::new(special_title, special_author, 2020);

    assert_eq!(book.title, special_title);
    assert_eq!(book.author, special_author);
    assert_eq!(book.year, 2020);
}

#[test]
fn test_constructor_unicode_characters_in_title_and_author() {
    let unicode_title = "Программирование на Rust";
    let unicode_author = "Иван Иванович";
    let book = Book::new(unicode_title, unicode_author, 2021);

    assert_eq!(book.title, unicode_title);
    assert_eq!(book.author, unicode_author);
    assert_eq!(book.year, 2021);
}

#[test]
fn test_constructor_negative_year() {
    let book = Book::new("Ancient Wisdom", "Unknown Sage", -3000);

    assert_eq!(book.title, "Ancient Wisdom");
    assert_eq!(book.author, "Unknown Sage");
    assert_eq!(book.year, -3000);
}

#[test]
fn test_constructor_zero_year() {
    let book = Book::new("The Dawn of Civilization", "Prehistoric Scholar", 0);

    assert_eq!(book.title, "The Dawn of Civilization");
    assert_eq!(book.author, "Prehistoric Scholar");
    assert_eq!(book.year, 0);
}
