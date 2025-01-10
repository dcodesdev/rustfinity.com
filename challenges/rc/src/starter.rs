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
    // Example usage of `use_shared_data`
    let shared_numbers = Rc::new(vec![1, 2, 3, 4, 5]);
    println!("Using shared data:");
    use_shared_data(Rc::clone(&shared_numbers));

    // Example usage of `share_data_to_other_functions`
    let strings = vec!["Rust".to_string(), "is".to_string(), "awesome!".to_string()];

    let print_data = |data: Rc<Vec<String>>| {
        println!("Printing shared data:");
        for item in data.iter() {
            println!("{}", item);
        }
    };

    println!("\nSharing data with other functions:");
    share_data_to_other_functions(print_data, strings);
}
