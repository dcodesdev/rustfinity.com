pub fn modify_elements(slice: &mut [i32]) {
    for num in slice.iter_mut() {
        if *num % 2 == 0 {
            *num *= 2; // Double the even number
        } else {
            *num -= 1; // Subtract 1 from the odd number
        }
    }
}
