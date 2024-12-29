pub struct Counter {
    count: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn increment(&mut self) {
        // Your code here...
    }

    pub fn decrement(&mut self) {
        // Your code here...
    }

    pub fn get_count(&self) -> i32 {
        // Your code here...
        0
    }
}

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
