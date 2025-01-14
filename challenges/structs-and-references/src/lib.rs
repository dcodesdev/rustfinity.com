pub struct TextFinder<'a> {
    text: &'a str,
}

impl<'a> TextFinder<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    pub fn find_first(&self, keyword: &str) -> Option<&str> {
        self.text.lines().find(|line| line.contains(keyword))
    }

    pub fn find_many(&self, keyword: &str) -> Vec<&str> {
        self.text
            .lines()
            .filter(|line| line.contains(keyword))
            .collect()
    }
}

pub fn main() {
    let text = "Rust is fast and memory-efficient.\nOwnership is key to Rust's safety.\nRustaceans love the borrow checker.";
    let finder = TextFinder::new(text);

    let first = finder.find_first("Rust");
    println!("{:?}", first); // Should print: Some("Rust is fast and memory-efficient.")

    let matches = finder.find_many("Rust");
    println!("{:?}", matches); // Should print: ["Rust is fast and memory-efficient.", "Ownership is key to Rust's safety."]
}
