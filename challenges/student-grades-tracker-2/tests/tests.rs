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
fn test_student_average_grade_no_grades() {
    let student = Student {
        name: "Alice".to_string(),
        grades: vec![],
    };
    assert_eq!(student.average_grade(), 0.0);
}

#[test]
fn test_add_student() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.add_student("Bob");
    assert!(tracker.students.contains_key("Alice"));
    assert!(tracker.students.contains_key("Bob"));
    assert_eq!(tracker.students["Alice"].grades, vec![]);
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
fn test_multiple_students_grades_and_averages() {
    let mut tracker = StudentGrades::new();
    tracker.add_student("Alice");
    tracker.add_student("Bob");

    tracker.add_grade("Alice", 85);
    tracker.add_grade("Alice", 90);
    tracker.add_grade("Bob", 78);
    tracker.add_grade("Bob", 88);

    let alice = &tracker.students["Alice"];
    let bob = &tracker.students["Bob"];

    assert_eq!(alice.grades, vec![85, 90]);
    assert_eq!(bob.grades, vec![78, 88]);

    assert!((alice.average_grade() - 87.5).abs() < 0.01);
    assert!((bob.average_grade() - 83.0).abs() < 0.01);
}

#[test]
fn test_large_dataset() {
    let mut tracker = StudentGrades::new();
    for i in 1..=100 {
        tracker.add_student(&format!("Student {}", i));
    }
    for i in 1..=100 {
        tracker.add_grade(&format!("Student {}", i), i as u8);
    }

    for i in 1..=100 {
        let student = &tracker.students[&format!("Student {}", i)];
        assert_eq!(student.grades, vec![i as u8]);
        assert_eq!(student.average_grade(), i as f64);
    }
}
