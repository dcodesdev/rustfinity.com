use derive_debug_for_structs::*;

#[test]
fn test_debug_person() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    assert_eq!(
        format!("{:?}", person),
        "Person { name: \"Alice\", age: 30 }"
    );
}

#[test]
fn test_debug_point() {
    let point = Point { x: 5.0, y: -3.2 };
    assert_eq!(format!("{:?}", point), "Point { x: 5.0, y: -3.2 }");
}

#[test]
fn test_debug_rectangle() {
    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    assert_eq!(
        format!("{:?}", rectangle),
        "Rectangle { width: 10, height: 20 }"
    );
}
