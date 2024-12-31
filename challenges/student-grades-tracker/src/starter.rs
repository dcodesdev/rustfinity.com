use std::collections::HashMap;

pub struct Student {
    pub name: String,
    pub grades: Vec<u8>,
}

pub struct StudentGrades {
    pub students: HashMap<String, Student>,
}

impl StudentGrades {
    pub fn new() -> Self {
        Self {
            students: HashMap::new(),
        }
    }

    pub fn add_student(&mut self, name: &str) {
        // Implement here
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        // Implement here
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        // Implement here
    }

    pub fn calculate_average(&self) -> f64 {
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
    println!("{:?}", tracker.calculate_average()); // 84.333...
}
