use interior_mutability::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_update_shared_data() {
    let shared_data = Rc::new(RefCell::new(vec![10, 20]));
    update_shared_data(Rc::clone(&shared_data), 30);
    update_shared_data(Rc::clone(&shared_data), 40);

    let borrowed_data = shared_data.borrow();
    assert_eq!(
        *borrowed_data,
        vec![10, 20, 30, 40],
        "Shared data was not updated correctly."
    );
}

#[test]
fn test_reference_count_behavior() {
    let shared_data = Rc::new(RefCell::new(vec![10, 20]));
    let initial_count = Rc::strong_count(&shared_data);

    {
        let _clone = Rc::clone(&shared_data);
        let _clone2 = Rc::clone(&shared_data);
        assert_eq!(
            Rc::strong_count(&shared_data),
            initial_count + 2,
            "Reference count did not increase correctly."
        );
    }

    assert_eq!(
        Rc::strong_count(&shared_data),
        initial_count,
        "Reference count did not return to its initial value after clones were dropped."
    );
}

#[test]
fn test_empty_vector_behavior() {
    let shared_data = Rc::new(RefCell::new(Vec::<i32>::new()));

    update_shared_data(Rc::clone(&shared_data), 42);
    assert_eq!(
        *shared_data.borrow(),
        vec![42],
        "Shared data was not updated correctly for an empty vector."
    );
}

#[test]
fn test_stdout() {
    let code_1 = syntest::quote! {
        use interior_mutability::*;
        use std::cell::RefCell;
        use std::rc::Rc;

        let shared_data = Rc::new(RefCell::new(vec!["Rust".to_string(), "is".to_string(), "awesome".to_string()]));
        iterate_and_print_shared_data(Rc::clone(&shared_data));
    };

    let code_2 = syntest::quote! {
        use interior_mutability::*;
        use std::cell::RefCell;
        use std::rc::Rc;

        let shared_data = Rc::new(RefCell::new(vec![1, 2, 3, 4, 5]));
        iterate_and_print_shared_data(Rc::clone(&shared_data));
    };

    let code_3 = syntest::quote! {
        use interior_mutability::*;
        use std::cell::RefCell;
        use std::rc::Rc;

        let empty_shared_data = Rc::new(RefCell::new(Vec::<i32>::new()));
        iterate_and_print_shared_data(Rc::clone(&empty_shared_data));
    };

    let output_1 = syntest::create_bin_and_run(&code_1);
    assert_eq!(
        output_1.stdout().trim(),
        "Rust\nis\nawesome",
        "Unexpected output for first syntest block."
    );

    let output_2 = syntest::create_bin_and_run(&code_2);
    assert_eq!(
        output_2.stdout().trim(),
        "1\n2\n3\n4\n5",
        "Unexpected output for second syntest block."
    );

    let output_3 = syntest::create_bin_and_run(&code_3);
    assert_eq!(
        output_3.stdout().trim(),
        "",
        "Unexpected output for third syntest block (empty vector case)."
    );
}
