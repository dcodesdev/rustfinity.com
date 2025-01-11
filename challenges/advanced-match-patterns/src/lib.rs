#[derive(Debug, Clone)]
pub enum BookItem {
    Book { pages: i32, discount: Option<i32> },
    EBook(String, (i32, i32)),
    Collection(Vec<BookItem>),
    OutOfPrint,
}

impl BookItem {
    pub fn check_validity(&self) -> bool {
        match self {
            BookItem::Book {
                pages,
                discount: Some(d),
            } if *pages > 0 && *d >= 0 && *d <= 50 => true,
            BookItem::Book {
                pages,
                discount: None,
            } if *pages > 0 => true,
            BookItem::EBook(name, (_, second)) if !name.is_empty() && *second > 0 => true,
            BookItem::Collection(items) => items.iter().any(|i| i.check_validity()),
            BookItem::OutOfPrint => false,
            _ => false,
        }
    }
}
