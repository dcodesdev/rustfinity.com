use procedural_macros::*;

trait Describe {
    fn describe(&self) -> String;
}

#[derive(Describe)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Describe)]
struct Person {
    name: String,
    age: u32,
}

#[derive(Describe)]
struct Numbers {
    integer: i32,
    unsigned: u64,
    float: f64,
}

#[derive(Describe)]
struct Mixed {
    flag: bool,
    optional: Option<i32>,
    items: Vec<String>,
}

#[test]
fn test_describe_point() {
    let p = Point { x: 1, y: 2 };
    assert_eq!(
        p.describe(),
        "Point { x: 1, y: 2 }",
        "Point struct description does not match expected format"
    );
}

#[test]
fn test_describe_person() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    assert_eq!(
        person.describe(),
        "Person { name: \"Alice\", age: 30 }",
        "Person struct description does not match expected format"
    );
}

#[test]
fn test_describe_numbers() {
    let numbers = Numbers {
        integer: -42,
        unsigned: 100,
        float: 3.14,
    };
    assert_eq!(
        numbers.describe(),
        "Numbers { integer: -42, unsigned: 100, float: 3.14 }",
        "Numbers struct description should properly format different numeric types"
    );
}

#[test]
fn test_describe_mixed_types() {
    let mixed = Mixed {
        flag: true,
        optional: Some(123),
        items: vec!["hello".to_string(), "world".to_string()],
    };
    assert_eq!(
        mixed.describe(),
        "Mixed { flag: true, optional: Some(123), items: [\"hello\", \"world\"] }",
        "Mixed struct description should handle complex types correctly"
    );
}

#[test]
fn test_describe_mixed_with_none() {
    let mixed = Mixed {
        flag: false,
        optional: None,
        items: Vec::new(),
    };
    assert_eq!(
        mixed.describe(),
        "Mixed { flag: false, optional: None, items: [] }",
        "Mixed struct description should handle None and empty collections"
    );
}
