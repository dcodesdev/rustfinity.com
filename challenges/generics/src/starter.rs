pub struct ItemContainer<T> {
    pub item: T,
}

impl<T> ItemContainer<T> {
    // TODO: Implement the `get_item` method to return a reference to the item.
}

// Example usage
pub fn main() {
    let container = ItemContainer { item: 42 };

    println!("Current item: {}", container.get_item());
}
