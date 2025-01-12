pub enum Animal {
    // Define the Animal variants here
}

pub fn describe_animal(animal: &Animal) -> String {
    // Your code here...
}

// Example use case
pub fn main() {
    let dog = Animal::Dog;
    assert_eq!(describe_animal(&dog), "A friendly dog.");

    let cat = Animal::Cat("Whiskers".to_string());
    assert_eq!(describe_animal(&cat), "A cat named Whiskers.");

    let bird = Animal::Bird {
        species: "Penguin".to_string(),
        can_fly: false,
    };
    assert_eq!(describe_animal(&bird), "A Penguin that cannot fly.");
}
