pub fn add_elements(vec: &mut Vec<i32>, elements: &[i32]) {
    vec.extend_from_slice(elements);
}

pub fn remove_element(vec: &mut Vec<i32>, index: usize) {
    if let Some(_) = vec.get(index) {
        vec.remove(index);
    }
}

pub fn get_element(vec: &Vec<i32>, index: usize) -> Option<i32> {
    vec.get(index).cloned()
}
