// 1. Complete the definitions of the structs Person, Point, and Rectangle.
// have a look at the function below `debug_example` to get the fields of the structs.
//
// 2. Make sure you implement the Debug trait for each struct. using the derive attribute.
pub struct Person {}
pub struct Point {}
pub struct Rectangle {}

// Example function
pub fn debug_example() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    println!("{:?}", person);

    let point = Point { x: 5.0, y: -3.2 };
    println!("{:?}", point);

    let rectangle = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{:?}", rectangle);
}
