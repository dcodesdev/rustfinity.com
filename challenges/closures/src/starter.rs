pub fn create_closures() -> (
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
) {
    let add_closure = |a, b| {
        // Step 1: Implement here
    };

    // Step 2:
    // Create the `subtract_closure` closure that subtracts two `i32` values.

    // Step 3:
    // Create the `multiply_closure` closure that multiplies two `i32` values.

    (add_closure, subtract_closure, multiply_closure)
}
