#[derive(PartialEq)]
pub enum OrderStatus {
    Pending,
    Shipped,
    Cancelled(String),
}
