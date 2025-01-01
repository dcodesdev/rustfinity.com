pub fn find_and_multiply(numbers: Vec<i32>, index1: usize, index2: usize) -> Option<i32> {
    let num1 = numbers.get(index1)?;
    let num2 = numbers.get(index2)?;
    Some(num1 * num2)
}
