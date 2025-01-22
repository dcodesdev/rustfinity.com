use generics::*;

#[test]
fn test_integer_item() {
    let container = ItemContainer { item: 42 };
    assert_eq!(*container.get_item(), 42);
}

#[test]
fn test_string_item() {
    let container = ItemContainer {
        item: String::from("Hello"),
    };
    assert_eq!(container.get_item(), "Hello");
}

#[test]
fn test_custom_struct_item() {
    struct Point {
        x: i32,
        y: i32,
    }

    let container = ItemContainer {
        item: Point { x: 1, y: 2 },
    };
    assert_eq!(container.get_item().x, 1);
    assert_eq!(container.get_item().y, 2);
}

#[test]
fn test_nested_container() {
    let inner_container = ItemContainer { item: 42 };
    let outer_container = ItemContainer {
        item: inner_container,
    };

    assert_eq!(*outer_container.get_item().get_item(), 42);
}

#[test]
fn test_large_data() {
    let large_vector: Vec<u8> = vec![1, 2, 3, 4, 5];
    let container = ItemContainer {
        item: large_vector.clone(),
    };

    assert_eq!(container.get_item(), &large_vector);
}

#[test]
fn test_item_lifetime() {
    let original_string = String::from("Owned");
    let container = ItemContainer {
        item: &original_string,
    };

    assert_eq!(*container.get_item(), "Owned");
}

#[test]
fn test_empty_container() {
    let container: ItemContainer<Option<i32>> = ItemContainer { item: None };

    assert_eq!(*container.get_item(), None);
}
