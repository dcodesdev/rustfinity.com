use std::collections::HashMap;

pub struct Student {
    pub name: String,
    pub grades: Vec<u8>,
}

impl Student {
    pub fn add_grade(&mut self, grade: u8) {
        self.grades.push(grade);
    }

    pub fn average_grade(&self) -> f64 {
        if self.grades.is_empty() {
            0.0
        } else {
            self.grades.iter().sum::<u8>() as f64 / self.grades.len() as f64
        }
    }
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
        self.students.entry(name.to_string()).or_insert(Student {
            name: name.to_string(),
            grades: vec![],
        });
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        if let Some(student) = self.students.get_mut(name) {
            student.add_grade(grade);
        }
    }

    pub fn calculate_average(&self) -> f64 {
        let mut total = 0.0;
        let mut count = 0;

        for student in self.students.values() {
            total += student.average_grade() * student.grades.len() as f64;
            count += student.grades.len();
        }

        if count == 0 {
            0.0
        } else {
            total / count as f64
        }
    }
}
