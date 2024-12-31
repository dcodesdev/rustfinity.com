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
        self.students.entry(name.to_string()).or_insert(Student {
            name: name.to_string(),
            grades: vec![],
        });
    }

    pub fn add_grade(&mut self, name: &str, grade: u8) {
        self.students.get_mut(name).unwrap().grades.push(grade);
    }

    pub fn get_grades(&self, name: &str) -> &[u8] {
        &self.students.get(name).unwrap().grades
    }

    pub fn calculate_average(&self) -> f64 {
        let mut total = 0;
        let mut count = 0;

        for student in self.students.values() {
            total += student.grades.iter().sum::<u8>() as u64;
            count += student.grades.len();
        }

        if count == 0 {
            0.0
        } else {
            total as f64 / count as f64
        }
    }
}
