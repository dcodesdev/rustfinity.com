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
