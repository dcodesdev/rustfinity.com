use supertraits::*;

#[test]
fn test_person_trait() {
    let student = Undergraduate {
        id: 101,
        name: "John Doe".to_string(),
        field_of_study: "Computer Science".to_string(),
    };

    assert_eq!(student.name(), "John Doe");
}

#[test]
fn test_student_trait() {
    let student = Undergraduate {
        id: 101,
        name: "John Doe".to_string(),
        field_of_study: "Computer Science".to_string(),
    };

    assert_eq!(student.id(), 101);
    assert_eq!(student.field_of_study(), "Computer Science");
}

#[test]
fn test_name_and_field_of_study() {
    let student = Undergraduate {
        id: 202,
        name: "Jane Smith".to_string(),
        field_of_study: "Mathematics".to_string(),
    };

    assert_eq!(student.name(), "Jane Smith");
    assert_eq!(student.id(), 202);
    assert_eq!(student.field_of_study(), "Mathematics");
}

#[test]
fn test_multiple_undergraduates() {
    let student1 = Undergraduate {
        id: 1,
        name: "Alice".to_string(),
        field_of_study: "Physics".to_string(),
    };
    let student2 = Undergraduate {
        id: 2,
        name: "Bob".to_string(),
        field_of_study: "Chemistry".to_string(),
    };

    assert_eq!(student1.name(), "Alice");
    assert_eq!(student1.id(), 1);
    assert_eq!(student1.field_of_study(), "Physics");
    assert_eq!(student2.name(), "Bob");
    assert_eq!(student2.id(), 2);
    assert_eq!(student2.field_of_study(), "Chemistry");
}

#[test]
fn test_empty_name_and_field_of_study() {
    let student = Undergraduate {
        id: 303,
        name: "".to_string(),
        field_of_study: "".to_string(),
    };

    assert_eq!(student.name(), "");
    assert_eq!(student.id(), 303);
    assert_eq!(student.field_of_study(), "");
}

#[test]
fn test_special_characters_in_name_and_field_of_study() {
    let student = Undergraduate {
        id: 404,
        name: "Dr. Strange".to_string(),
        field_of_study: "Mystic Arts".to_string(),
    };

    assert_eq!(student.name(), "Dr. Strange");
    assert_eq!(student.id(), 404);
    assert_eq!(student.field_of_study(), "Mystic Arts");
}

#[test]
fn test_numeric_field_of_study() {
    let student = Undergraduate {
        id: 505,
        name: "Number Theory Enthusiast".to_string(),
        field_of_study: "12345".to_string(),
    };

    assert_eq!(student.name(), "Number Theory Enthusiast");
    assert_eq!(student.id(), 505);
    assert_eq!(student.field_of_study(), "12345");
}

#[test]
fn test_long_name_and_field_of_study() {
    let student = Undergraduate {
        id: 606,
        name: "A Very Long Name That Should Still Work Fine".to_string(),
        field_of_study: "A Field of Study That Is Also Surprisingly Long".to_string(),
    };

    assert_eq!(
        student.name(),
        "A Very Long Name That Should Still Work Fine"
    );
    assert_eq!(student.id(), 606);
    assert_eq!(
        student.field_of_study(),
        "A Field of Study That Is Also Surprisingly Long"
    );
}

#[test]
fn test_edge_case_id_zero() {
    let student = Undergraduate {
        id: 0,
        name: "Zero ID".to_string(),
        field_of_study: "Edge Case Studies".to_string(),
    };

    assert_eq!(student.name(), "Zero ID");
    assert_eq!(student.id(), 0);
    assert_eq!(student.field_of_study(), "Edge Case Studies");
}

#[test]
fn test_edge_case_id_large_value() {
    let student = Undergraduate {
        id: u32::MAX,
        name: "Max Value Student".to_string(),
        field_of_study: "Large Numbers".to_string(),
    };

    assert_eq!(student.name(), "Max Value Student");
    assert_eq!(student.id(), u32::MAX);
    assert_eq!(student.field_of_study(), "Large Numbers");
}
