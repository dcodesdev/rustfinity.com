pub fn create_typed_closures() -> (
    impl Fn(f64, f64) -> f64,
    impl FnMut(&mut f64, f64),
    impl FnOnce(String) -> String,
) {
    let calculate_total = |price: f64, tax_rate: f64| price + price * tax_rate;

    let apply_discount = |total: &mut f64, discount: f64| {
        *total -= discount;
    };

    let checkout_cart = |cart_details: String| format!("Checkout complete: {}", cart_details);

    (calculate_total, apply_discount, checkout_cart)
}
