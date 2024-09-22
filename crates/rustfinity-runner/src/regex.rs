use regex::Regex;

pub fn extract_unittest_path(output: &str) -> Option<String> {
    let re = Regex::new(r"Running\s+(?:tests/tests\.rs\s+)?\((.+?)\)").unwrap();

    re.captures(output)
        .and_then(|caps| caps.get(1))
        .map(|match_| match_.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extracts_command() {
        let log = r#"
   Compiling rustfinity-runner v0.1.0 (/home/user/Desktop/rustfinity.com/crates/rustfinity-runner)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.78s
     Running `/home/user/Desktop/rustfinity.com/target/debug/rustfinity-runner run --code cHViIGZuIGhlbGxvX3dvcmxkKCkgewogICAgcHJpbnRsbiEoImhlbGxvIHdvcmxkIikKfQo= --challenge printing-hello-world`
   Compiling printing-hello-world v0.1.0 (/home/user/Desktop/rustfinity.com/challenges/printing-hello-world)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running unittests src/lib.rs (/home/user/Desktop/rustfinity.com/target/debug/deps/printing_hello_world-52b4488be819771f)
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/tests.rs (/home/user/Desktop/rustfinity.com/target/debug/deps/tests-083061539c0b4b6b)

running 2 tests
test tests::test_compiles ... ok
test tests::test_hello_world ... ok
    "#;

        let command = extract_unittest_path(log);
        assert_eq!(
            command,
            Some(
                "/home/user/Desktop/rustfinity.com/target/debug/deps/tests-083061539c0b4b6b"
                    .to_string()
            )
        );
    }
}
