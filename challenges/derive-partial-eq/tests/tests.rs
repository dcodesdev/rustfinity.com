use derive_partial_eq::*;

#[test]
fn test_pending_status_equality() {
    let status1 = OrderStatus::Pending;
    let status2 = OrderStatus::Pending;

    if status1 != status2 {
        panic!("Pending statuses are not equal");
    }
}

#[test]
fn test_shipped_status_equality() {
    let status1 = OrderStatus::Shipped;
    let status2 = OrderStatus::Shipped;

    if status1 != status2 {
        panic!("Shipped statuses are not equal");
    }
}

#[test]
fn test_cancelled_status_equality() {
    let cancelled1 = OrderStatus::Cancelled("Out of stock".to_string());
    let cancelled2 = OrderStatus::Cancelled("Out of stock".to_string());

    if cancelled1 != cancelled2 {
        panic!("Cancelled statuses are not equal");
    }
}

#[test]
fn test_cancelled_status_inequality() {
    let cancelled1 = OrderStatus::Cancelled("Out of stock".to_string());
    let cancelled2 = OrderStatus::Cancelled("Customer request".to_string());

    if cancelled1 == cancelled2 {
        panic!("Cancelled statuses are equal");
    }
}

#[test]
fn test_mixed_status_inequality() {
    let pending = OrderStatus::Pending;
    let shipped = OrderStatus::Shipped;
    let cancelled = OrderStatus::Cancelled("Out of stock".to_string());

    if pending == shipped || pending == cancelled || shipped == cancelled {
        panic!("Mixed statuses are equal");
    }
}
