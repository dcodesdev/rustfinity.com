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

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person: {}, Age: {}", self.name, self.age)
    }
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: {}, Author: {}", self.title, self.author)
    }
}
