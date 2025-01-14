pub struct Book {
    pub title: String,
    pub author: String,
    pub year: i32,
    pub likes: i32,
}

impl Book {
    pub fn new(title: &str, author: &str, year: i32) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            year,
            likes: 0,
        }
    }
}
