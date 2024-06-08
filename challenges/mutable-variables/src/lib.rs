pub fn mutating_variables() -> i32 {
    let mut x = 5;
    let mut y = 10;

    x += 10;
    y *= 2;

    x + y
}
