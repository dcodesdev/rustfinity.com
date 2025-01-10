use std::{fmt::Display, rc::Rc};

pub fn use_shared_data<T: Display>(data: Rc<Vec<T>>) {
    for i in data.iter() {
        println!("{i}");
    }
}

pub fn share_data_to_other_functions<F>(mut take_item: F, items: Vec<String>)
where
    F: FnMut(Rc<Vec<String>>),
{
    let my_vec = Rc::new(items);

    take_item(Rc::clone(&my_vec));
    take_item(Rc::clone(&my_vec));
    take_item(Rc::clone(&my_vec));
}
