pub fn create_closures() -> (
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
    impl Fn(i32, i32) -> i32,
) {
    let add_closure = |a, b| a + b;
    let subtract_closure = |a, b| a - b;
    let multiply_closure = |a, b| a * b;

    (add_closure, subtract_closure, multiply_closure)
}
