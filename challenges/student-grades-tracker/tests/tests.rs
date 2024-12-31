use student_grades_tracker::*;

#[test]
fn test_add_student() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    assert!(tracker.students.contains_key("Alice"));
}

#[test]
fn test_add_grade() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.add_grade("Alice", 85);
    assert_eq!(tracker.get_grades("Alice"), &[85]);
}

#[test]
fn test_get_grades() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    assert_eq!(tracker.get_grades("Alice"), &[85, 90]);
}

#[test]
fn test_calculate_average() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_student("Bob");
    tracker.add_grade("Bob", 78);
    assert!((tracker.calculate_average() - 84.333).abs() < 0.01);
}
