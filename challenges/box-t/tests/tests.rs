use box_t::{access_animal, create_animal};

#[test]
fn test_create_animal() {
    let animal = create_animal("Bella", 3);
    assert_eq!(animal.name, "Bella");
    assert_eq!(animal.age, 3);
}

#[test]
fn test_access_animal() {
    let animal = create_animal("Max", 4);
    let (name, age) = access_animal(animal);
    assert_eq!(name, "Max");
    assert_eq!(age, 4);
}

#[test]
fn test_create_animal_empty_name() {
    let animal = create_animal("", 7);
    assert_eq!(animal.name, "");
    assert_eq!(animal.age, 7);
}

#[test]
fn test_access_animal_empty_name() {
    let animal = create_animal("", 7);
    let (name, age) = access_animal(animal);
    assert_eq!(name, "");
    assert_eq!(age, 7);
}

#[test]
fn test_create_animal_long_name() {
    let long_name = "L".repeat(100);
    let animal = create_animal(&long_name, 2);
    assert_eq!(animal.name, long_name);
    assert_eq!(animal.age, 2);
}

#[test]
fn test_access_animal_long_name() {
    let long_name = "L".repeat(100);
    let animal = create_animal(&long_name, 2);
    let (name, age) = access_animal(animal);
    assert_eq!(name, long_name);
    assert_eq!(age, 2);
}

#[test]
fn test_create_animal_boundary_age() {
    let animal = create_animal("Tiny", 0);
    assert_eq!(animal.name, "Tiny");
    assert_eq!(animal.age, 0);

    let animal = create_animal("Giant", u8::MAX);
    assert_eq!(animal.name, "Giant");
    assert_eq!(animal.age, u8::MAX);
}

#[test]
fn test_access_animal_boundary_age() {
    let animal = create_animal("Tiny", 0);
    let (name, age) = access_animal(animal);
    assert_eq!(name, "Tiny");
    assert_eq!(age, 0);

    let animal = create_animal("Giant", u8::MAX);
    let (name, age) = access_animal(animal);
    assert_eq!(name, "Giant");
    assert_eq!(age, u8::MAX);
}
