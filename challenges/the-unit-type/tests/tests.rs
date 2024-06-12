#[cfg(test)]
mod tests {
    use syntest::Syntest;
    use the_unit_type::*;

    #[test]
    fn test_compiles() {
        print_message();
    }

    #[test]
    fn test_prints() {
        let syntest = Syntest::new("print_message", "src/lib.rs");

        let macros = syntest.mac.macros();

        assert!(!macros.is_empty(), "You should use the `println!` macro");
        assert_eq!(macros.len(), 1, "You should only print one message");

        let macro_name = &macros[0].name;

        assert_eq!(macro_name, "println", "You should use the `println!` macro");

        for token in macros[0].tokens.iter() {
            let token = token.to_lowercase();

            assert_eq!(
                (token.contains("hello"), token.contains("rust")),
                (true, true),
                "You should print `Hello, Rust!`"
            )
        }
    }
}
