use rc::*;
use std::rc::Rc;

#[test]
fn test_use_shared_data_with_strings() {
    let code = syntest::quote! {
        use rc::*;
        use std::rc::Rc;

        let data = Rc::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        use_shared_data(Rc::clone(&data)); // Expect output: "a", "b", "c"
    };

    let output = syntest::create_bin_and_run(&code);

    assert_eq!(output.stdout(), "a\nb\nc\n",)
}

#[test]
fn test_use_shared_data_with_numbers() {
    let data = Rc::new(vec![10, 20, 30]);
    use_shared_data(Rc::clone(&data)); // Expect output: 10, 20, 30
                                       // Since this function prints to stdout, manual verification is required for correctness.
}

#[test]
fn test_use_shared_data_empty_vector() {
    let data: Rc<Vec<i32>> = Rc::new(vec![]);
    use_shared_data(Rc::clone(&data)); // Expect no output
                                       // Since this function prints to stdout, manual verification is required for correctness.
}

#[test]
fn test_share_data_to_other_functions_with_custom_closure() {
    let mut captured_data = Vec::new();
    let capture_closure = |data: Rc<Vec<String>>| {
        captured_data.push(data.clone());
    };

    let items = vec!["Rust".to_string(), "Rc".to_string(), "Test".to_string()];
    share_data_to_other_functions(capture_closure, items.clone());

    assert_eq!(
        captured_data.len(),
        3,
        "The closure should have been called 3 times"
    );
    for rc_data in captured_data {
        assert_eq!(
            rc_data.as_ref(),
            &items,
            "All Rc references should contain the same vector data"
        );
    }
}

#[test]
fn test_share_data_to_other_functions_with_different_closures() {
    let mut call_count = 0;
    let count_closure = |_: Rc<Vec<String>>| {
        call_count += 1;
    };

    let items = vec!["One".to_string(), "Two".to_string()];
    share_data_to_other_functions(count_closure, items);

    assert_eq!(
        call_count, 3,
        "The closure should be invoked exactly 3 times"
    );
}

#[test]
fn test_reference_count_increase() {
    let items = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    let rc_data = Rc::new(items);

    let initial_count = Rc::strong_count(&rc_data);

    share_data_to_other_functions(
        |data: Rc<Vec<String>>| {
            let _ = Rc::clone(&data);
        },
        rc_data.as_ref().clone(),
    );

    assert_eq!(
        Rc::strong_count(&rc_data),
        initial_count,
        "The reference count should return to the initial count after function execution"
    );
}

#[test]
fn test_reference_count_after_function_execution() {
    let items = vec!["Shared".to_string()];
    let rc_data = Rc::new(items);

    let initial_count = Rc::strong_count(&rc_data);

    let capture_closure = |data: Rc<Vec<String>>| {
        assert_eq!(
            Rc::strong_count(&data),
            initial_count + 1,
            "The reference count should increase while the closure holds a clone"
        );
    };

    share_data_to_other_functions(capture_closure, rc_data.as_ref().clone());

    assert_eq!(
        Rc::strong_count(&rc_data),
        initial_count,
        "The reference count should return to the initial count after the closure releases its clone"
    );
}
