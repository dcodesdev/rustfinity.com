// 1. Define the struct
pub struct Counter


// 2. Implement the associated function and methods


// Example use case
pub fn main() {
    let mut counter = Counter::new();

    counter.increment();
    assert_eq!(counter.get_count(), 1);

    counter.increment();
    counter.increment();
    assert_eq!(counter.get_count(), 3);

    counter.decrement();
    assert_eq!(counter.get_count(), 2);

    counter.decrement();
    counter.decrement();
    assert_eq!(counter.get_count(), 0);
}
