use procedural_macros::Describe;

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

#[test]
fn test_describe_point() {
    let p = Point { x: 1, y: 2 };
    assert_eq!(p.describe(), "Point { x: 1, y: 2 }");
}

#[test]
fn test_describe_person() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    assert_eq!(person.describe(), "Person { name: \"Alice\", age: 30 }");
}
