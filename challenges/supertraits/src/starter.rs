pub trait Person {
    fn name(&self) -> String;
}

pub trait Student: Person {
    fn id(&self) -> u32;
    fn field_of_study(&self) -> String;
}

pub struct Undergraduate {
    // Define fields for id, name, and field_of_study here...
}

// Implement Person for Undergraduate
impl Person for Undergraduate {
    fn name(&self) -> String {
        // Your code here...
    }
}

// Implement Student for Undergraduate
impl Student for Undergraduate {
    fn id(&self) -> u32 {
        // Your code here...
    }

    fn field_of_study(&self) -> String {
        // Your code here...
    }
}

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
