pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Person {
    pub name: String,
    pub age: u8,
}

pub struct Book {
    pub title: String,
    pub author: String,
}

// TODO: Implement the `Describable` trait for `Person` and `Book`

// Example usage
pub fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let book = Book {
        title: "Rust Programming".to_string(),
        author: "Jane Doe".to_string(),
    };

    println!("{}", person.describe());
    println!("{}", book.describe());
}
