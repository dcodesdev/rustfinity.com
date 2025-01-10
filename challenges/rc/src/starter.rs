use std::rc::Rc;

pub fn use_shared_data<T>(data: Rc<Vec<T>>) {
    // 1. Loop over each item in the vector and print it using `println!`
}

pub fn share_data_to_other_functions<F>(mut take_item: F, items: Vec<String>)
where
    F: FnMut(Rc<Vec<String>>),
{
    // 2. Implement the function
    // Share the data as a reference-counted pointer 3 times with the closure
}

// Example usage
pub fn main() {
    let original = Rc::new(vec![1, 2, 3]);
    let modified = clone_and_append(Rc::clone(&original), 4);

    // The original vector remains unchanged
    println!("{:?}", original); // Output: [1, 2, 3]

    // The modified vector includes the new value
    println!("{:?}", modified); // Output: [1, 2, 3, 4]
}
