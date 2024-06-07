# Syntest

Syntest is a small crate that is used to test the rustfinity challenges not just by output and behavior but also by the syntax of the code.

## Usage

```rust
use syntest::Syntest;

#[test]
fn test_syntax() {
  // opens the file and reads the code
  let syntest = Syntest::from("./src/main.rs");

  // gets all the local variables inside a function
  let variables = syntest.variables("main");

  // Checks if all of the variables are used
  variables.iter().for_each(|variable| {
    assert!(variable.is_used());
  });
}
```
