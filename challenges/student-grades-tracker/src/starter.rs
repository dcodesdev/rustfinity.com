use std::collections::HashMap;

pub struct Student {
    // 1. Define the fields
}

pub struct StudentGrades {
    // 2. Define the fields
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    // 3. Implement the methods
    pub fn add_student(&mut self, name: &str) {
        // Implement here
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        // Implement here
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        // Implement here
    }
}

// Example usage
pub fn main() {
    let mut tracker = StudentGrades::new();

    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);

    println!("{:?}", tracker.get_grades("Alice")); // [85, 90]
    println!("{:?}", tracker.get_grades("Bob")); // [78]
}
