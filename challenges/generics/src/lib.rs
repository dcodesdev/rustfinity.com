pub struct ItemContainer<T> {
    pub item: T,
}

impl<T> ItemContainer<T> {
    pub fn get_item(&self) -> &T {
        &self.item
    }
}
