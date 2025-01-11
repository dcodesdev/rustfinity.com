#[derive(Debug)]
pub enum BookItem {
    Book { pages: i32, discount: Option<i32> },
    EBook(String, (i32, i32)),
    Collection(Vec<BookItem>),
    OutOfPrint,
}

// Removed standalone check_validity. We'll rely on BookItem::check_validity.

// Example usage
pub fn main() {
    let book_a = BookItem::Book {
        pages: 42,
        discount: Some(100),
    };
    let ebook_b = BookItem::EBook("hello".to_string(), (1, 2));
    let collection_c = BookItem::Collection(vec![book_a.clone(), BookItem::OutOfPrint]);

    println!("{}", book_a.check_validity()); // e.g. true
    println!("{}", ebook_b.check_validity()); // e.g. true
    println!("{}", collection_c.check_validity()); // e.g. true
    println!("{}", BookItem::OutOfPrint.check_validity()); // e.g. false
}
