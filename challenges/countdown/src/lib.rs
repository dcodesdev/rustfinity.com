mod tests;

pub fn countdown(n: u32) -> Vec<u32> {
    let mut current = n;
    let mut result = Vec::new();
    while current > 0 {
        result.push(current);
        current -= 1;
    }
    result.push(0); // Add 0 at the end
    result
}
