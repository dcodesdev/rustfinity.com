pub fn factorial(n: u32) -> u128 {
    if n == 0 {
        return 1;
    }

    let mut result: u128 = 1;
    for i in 1..=n {
        result *= i as u128;
    }
    result
}
