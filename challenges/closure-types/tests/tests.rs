use closure_types::*;

#[test]
fn test_calculate_total() {
    let (calculate_total, _, _) = create_typed_closures();
    assert_eq!(calculate_total(100.0, 0.2), 120.0);
    assert_eq!(calculate_total(50.0, 0.1), 55.0);
    assert_eq!(calculate_total(0.0, 0.3), 0.0);
    assert_eq!(calculate_total(200.0, 0.15), 230.0);
}

#[test]
fn test_apply_discount() {
    let (_, mut apply_discount, _) = create_typed_closures();
    let mut total_price = 120.0;

    // Apply a discount and check the result
    apply_discount(&mut total_price, 20.0);
    assert_eq!(total_price, 100.0);

    // Apply a larger discount
    apply_discount(&mut total_price, 30.0);
    assert_eq!(total_price, 70.0);

    // Apply a zero discount
    apply_discount(&mut total_price, 0.0);
    assert_eq!(total_price, 70.0);

    // Ensure that applying a discount larger than the total works correctly
    apply_discount(&mut total_price, 100.0);
    assert_eq!(total_price, -30.0);
}

#[test]
fn test_checkout_cart() {
    let (_, _, checkout_cart) = create_typed_closures();

    // Standard cart checkout
    let cart_details = String::from("Items: Apple, Banana, Orange");
    assert_eq!(
        checkout_cart(cart_details.clone()),
        "Checkout complete: Items: Apple, Banana, Orange"
    );
}

#[test]
fn test_combined_behavior() {
    let (calculate_total, mut apply_discount, checkout_cart) = create_typed_closures();

    // Simulate a real-world use case
    let item_price = 150.0;
    let tax_rate = 0.2;
    let mut total_price = calculate_total(item_price, tax_rate);

    assert_eq!(total_price, 180.0); // Price after tax

    apply_discount(&mut total_price, 30.0); // Apply a discount
    assert_eq!(total_price, 150.0);

    let cart_details = String::from("Items: Laptop, Mouse");
    let receipt = checkout_cart(cart_details);
    assert_eq!(receipt, "Checkout complete: Items: Laptop, Mouse");
}

#[test]
fn test_fn_once_should_work_once() {
    // should not compile
    let code = syntest::quote! {
        use closure_types::*;

        let (_,_, checkout) = create_typed_closures();
        checkout(String::from("Items: Apple, Banana, Orange"));
        checkout(String::from("Items: Apple, Banana, Orange"));
    };

    let result = syntest::create_bin_and_run(&code);

    assert!(
        result
            .stderr()
            .contains("error[E0382]: use of moved value: `checkout`"),
        "`checkout_cart` closure should be FnOnce"
    );
}
