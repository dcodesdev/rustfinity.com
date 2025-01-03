pub struct MutableTextFinder<'a> {
    text: &'a mut String,
}

impl<'a> MutableTextFinder<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self { text }
    }

    pub fn find_first(&self, keyword: &str) -> Option<&str> {
        self.text.lines().find(|line| line.contains(keyword))
    }

    pub fn replace_lines(&mut self, keyword: &str, replacement: &str) {
        let modified: String = self
            .text
            .lines()
            .map(|line| {
                if line.contains(keyword) {
                    replacement.to_string()
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        self.text.clear();
        self.text.push_str(&modified);
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}
