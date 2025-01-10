use std::cell::RefCell;
use std::rc::Rc;

pub fn update_shared_data<T>(data: Rc<RefCell<Vec<T>>>, element: T) {
    // 1. Finish the function
}

pub fn iterate_and_print_shared_data<T>(data: Rc<RefCell<Vec<T>>>) {
    // 2. Borrow the data and print each item
}

// Example usage
pub fn main() {
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    // Updating shared data
    update_shared_data(Rc::clone(&shared_data), 4);
    update_shared_data(Rc::clone(&shared_data), 5);

    // Iterating and printing shared data
    println!("Shared Data:");
    iterate_and_print_shared_data(Rc::clone(&shared_data));
}
