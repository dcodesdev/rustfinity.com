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

    assert_eq!(
        result.stdout(),
        "Hello, world!\n",
        "Expected 'Hello, world!\\n'"
    );
}
