// 1. Finish the trait definition
pub trait Person

// 2. Finish the trait definition
pub trait Student

// 3. Finish the struct definition
pub struct Undergraduate {
    // Define fields for id, name, and field_of_study here...
}

// 4. Implement the necessary traits for the Undergraduate struct

// Example usage
pub fn main() {
    let student = Undergraduate {
        id: 101,
        name: "John Doe".to_string(),
        field_of_study: "Computer Science".to_string(),
    };

    assert_eq!(student.name(), "John Doe");
    assert_eq!(student.id(), 101);
    assert_eq!(student.field_of_study(), "Computer Science");
}
