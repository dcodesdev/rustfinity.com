pub fn mutating_variables() -> String {
    let mut value = "hello".to_string();

    mutates_value(&mut value);

    String::from(value)
}

// Do not change this function
pub fn mutates_value(value: &mut String) {
    *value = String::from("bye")
}
