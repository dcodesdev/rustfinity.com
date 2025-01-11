use advanced_match_patterns::*;

#[test]
fn test_book_with_discount_valid() {
    let data = BookItem::Book {
        pages: 42,
        discount: Some(100),
    };
    assert_eq!(
        data.check_validity(),
        false,
        "Book with discount > 50 should be invalid"
    );
}

#[test]
fn test_book_no_discount_valid() {
    let data = BookItem::Book {
        pages: 42,
        discount: None,
    };
    assert_eq!(
        data.check_validity(),
        true,
        "Book with valid pages and no discount should be valid"
    );
}

#[test]
fn test_book_negative_pages_invalid() {
    let data = BookItem::Book {
        pages: -1,
        discount: Some(10),
    };
    assert_eq!(
        data.check_validity(),
        false,
        "Book with negative pages should be invalid"
    );
}

#[test]
fn test_book_discount_out_of_range_invalid() {
    let data = BookItem::Book {
        pages: 42,
        discount: Some(60),
    };
    assert_eq!(
        data.check_validity(),
        false,
        "Book with discount > 50 should be invalid"
    );
}

#[test]
fn test_ebook_valid() {
    let data = BookItem::EBook("hello".to_string(), (1, 2));
    assert_eq!(
        data.check_validity(),
        true,
        "EBook with non-empty title and valid tuple should be valid"
    );
}

#[test]
fn test_ebook_empty_name_invalid() {
    let data = BookItem::EBook("".to_string(), (0, 0));
    assert_eq!(
        data.check_validity(),
        false,
        "EBook with empty title should be invalid"
    );
}

#[test]
fn test_ebook_invalid_tuple() {
    let data = BookItem::EBook("hello".to_string(), (1, 0));
    assert_eq!(
        data.check_validity(),
        false,
        "EBook with second tuple value <= 0 should be invalid"
    );
}

#[test]
fn test_collection_valid() {
    let data = BookItem::Collection(vec![
        BookItem::Book {
            pages: 10,
            discount: None,
        },
        BookItem::OutOfPrint,
    ]);
    assert_eq!(
        data.check_validity(),
        true,
        "Collection with at least one valid item should be valid"
    );
}

#[test]
fn test_collection_all_invalid() {
    let data = BookItem::Collection(vec![
        BookItem::Book {
            pages: -10,
            discount: None,
        },
        BookItem::OutOfPrint,
    ]);
    assert_eq!(
        data.check_validity(),
        false,
        "Collection with all invalid items should be invalid"
    );
}

#[test]
fn test_collection_empty_invalid() {
    let data = BookItem::Collection(vec![]);
    assert_eq!(
        data.check_validity(),
        false,
        "Empty collection should be invalid"
    );
}

#[test]
fn test_out_of_print_invalid() {
    let data = BookItem::OutOfPrint;
    assert_eq!(
        data.check_validity(),
        false,
        "OutOfPrint item should always be invalid"
    );
}

#[test]
fn test_nested_collection_valid() {
    let data = BookItem::Collection(vec![
        BookItem::Collection(vec![
            BookItem::Book {
                pages: 10,
                discount: None,
            },
            BookItem::OutOfPrint,
        ]),
        BookItem::EBook("nested".to_string(), (5, 6)),
    ]);
    assert_eq!(
        data.check_validity(),
        true,
        "Nested collection with at least one valid item should be valid"
    );
}

#[test]
fn test_nested_collection_all_invalid() {
    let data = BookItem::Collection(vec![
        BookItem::Collection(vec![
            BookItem::Book {
                pages: -10,
                discount: None,
            },
            BookItem::OutOfPrint,
        ]),
        BookItem::EBook("".to_string(), (0, 0)),
    ]);
    assert_eq!(
        data.check_validity(),
        false,
        "Nested collection with all invalid items should be invalid"
    );
}
