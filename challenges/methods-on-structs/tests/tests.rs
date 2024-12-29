use methods_on_structs::Counter;

#[test]
fn test_initial_count() {
    let counter = Counter::new();
    assert_eq!(counter.get_count(), 0);
}

#[test]
fn test_increment() {
    let mut counter = Counter::new();
    counter.increment();
    assert_eq!(counter.get_count(), 1);
    counter.increment();
    assert_eq!(counter.get_count(), 2);
}

#[test]
fn test_decrement() {
    let mut counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.decrement();
    assert_eq!(counter.get_count(), 1);
}

#[test]
fn test_increment_and_decrement_combination() {
    let mut counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    counter.decrement();
    assert_eq!(counter.get_count(), 2);
}

#[test]
fn test_multiple_decrements() {
    let mut counter = Counter::new();
    counter.decrement();
    counter.decrement();
    assert_eq!(counter.get_count(), -2);
}
