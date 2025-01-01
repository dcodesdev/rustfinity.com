pub fn get_first_element(numbers: Vec<i32>, min_value: i32) -> Result<i32, String> {
    let first_element = numbers.first().ok_or("Vector is empty".to_string())?;
    if *first_element < min_value {
        Err("First element is below the minimum allowed value".to_string())
    } else {
        Ok(*first_element)
    }
}
