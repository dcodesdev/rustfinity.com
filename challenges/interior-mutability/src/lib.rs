use std::cell::RefCell;
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
