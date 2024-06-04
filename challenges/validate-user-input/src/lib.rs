pub fn validate_user(age: i32, email: &str) -> Result<(), String> {
    if age < 0 || age > 120 {
        return Err(String::from("Invalid age"));
    }

    if !email.contains('@') {
        return Err(String::from("Invalid email"));
    }

    Ok(())
}
