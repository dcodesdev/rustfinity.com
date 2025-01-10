use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;

pub fn update_shared_data<T>(data: Rc<RefCell<Vec<T>>>, element: T) {
    let mut borrowed_data = data.borrow_mut();
    borrowed_data.push(element);
}

pub fn iterate_and_print_shared_data<T: std::fmt::Display>(data: Rc<RefCell<Vec<T>>>) {
    let borrowed_data = data.borrow();
    for item in borrowed_data.iter() {
        println!("{}", item);
    }
}

pub fn plus_one(data: Rc<RefCell<i32>>) {
    let mut borrowed_data = data.borrow_mut();
    *borrowed_data = borrowed_data.add(1);
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

    let my_num = Rc::new(RefCell::new(5));
    plus_one(Rc::clone(&my_num));
    assert_eq!(*my_num.borrow(), 6, "Value was not incremented correctly.");
}
