use printing_hello_world::*;
use syntest::quote;

#[test]
fn test_compiles() {
    hello_world();
}

#[test]
fn test_hello_world() {
    let code = quote! {
        use printing_hello_world::*;

        hello_world();
    };

    let result = syntest::create_bin_and_run(&code);

    let stdout_lowercase = result.stdout().to_lowercase();

    assert!(
        stdout_lowercase.contains("hello, world"),
        "Expected 'Hello, World!' to be printed to the console.",
    );
}
