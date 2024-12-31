use student_grades_tracker_2::*;

#[test]
fn test_student_add_grade() {
    let mut student = Student {
        name: "Alice".to_string(),
        grades: vec![],
    };
    student.add_grade(85);
    student.add_grade(90);
    assert_eq!(student.grades, vec![85, 90]);
}

#[test]
fn test_student_average_grade() {
    let student = Student {
        name: "Alice".to_string(),
        grades: vec![85, 90, 78],
    };
    assert!((student.average_grade() - 84.333).abs() < 0.01);
}

#[test]
fn test_add_grade_to_student() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    assert_eq!(tracker.students["Alice"].grades, vec![85, 90]);
}

#[test]
fn test_calculate_average_with_student_methods() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_student("Bob");
    tracker.add_grade("Bob", 78);
    assert!((tracker.calculate_average() - 84.333).abs() < 0.01);
}
