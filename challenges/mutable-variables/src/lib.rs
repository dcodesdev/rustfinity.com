pub fn mutating_variables() -> &'static str {
    let mut text = "hello";

    println!("Text is: {}", text);

    text = "bye";

    println!("Text is: {}", text);

    text
}
