pub fn print_message<T: AsRef<str>>(message: T) {
    println!("{}", message.as_ref());
}
