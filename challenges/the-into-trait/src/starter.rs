pub fn print_message<T: Into<String>>(message: T) {
    // Convert the message into a String and print it
    // Your code here...
}

// Example usage
pub fn main() {
    // Example 1: Using a &str
    print_message("Hello, world!");

    // Example 2: Using a String
    let greeting = String::from("Welcome to Rust!");
    print_message(greeting);
}
