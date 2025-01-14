pub fn update_slice(slice: &mut [i32], indices: &[usize], value: i32) {
    for &index in indices {
        if let Some(element) = slice.get_mut(index) {
            *element = value;
        }
    }
}
