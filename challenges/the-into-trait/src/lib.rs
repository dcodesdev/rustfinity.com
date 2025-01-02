pub fn print_message<T: Into<String>>(message: T) {
    let message: String = message.into();
    println!("{}", message);
}
