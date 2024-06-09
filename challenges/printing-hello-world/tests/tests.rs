#[cfg(test)]

mod tests {
    use printing_hello_world::*;
    use syntest::Syntest;

    #[test]
    fn test_compiles() {
        hello_world();
    }

    #[test]
    fn test_hello_world() {
        let syntest = Syntest::from("./src/lib.rs");
        let macros = syntest.mac.macros("hello_world");

        assert_eq!(
            macros.len(),
            1,
            "Expected to print to the console only once, but {} macros used",
            macros.len()
        );

        let mac = &macros[0];

        assert_eq!(mac.name, "println", "Expected to use println macro");

        for token in mac.tokens.iter() {
            let token = token.to_lowercase();
            assert_eq!(
                (token.contains("hello"), token.contains("world")),
                (true, true),
                "Expected to print 'Hello, world!' to the console."
            );
        }
    }
}
