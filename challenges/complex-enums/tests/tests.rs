use complex_enums::*;

#[test]
fn test_describe_dog() {
    let dog = Animal::Dog;
    assert_eq!(describe_animal(&dog), "A friendly dog.");
}

#[test]
fn test_describe_cat() {
    let cat = Animal::Cat("Whiskers".to_string());
    assert_eq!(describe_animal(&cat), "A cat named Whiskers.");

    let cat = Animal::Cat("Shadow".to_string());
    assert_eq!(describe_animal(&cat), "A cat named Shadow.");
}

#[test]
fn test_describe_bird_can_fly() {
    let bird = Animal::Bird {
        species: "Eagle".to_string(),
        can_fly: true,
    };
    assert_eq!(describe_animal(&bird), "A Eagle that can fly.");
}

#[test]
fn test_describe_bird_cannot_fly() {
    let bird = Animal::Bird {
        species: "Penguin".to_string(),
        can_fly: false,
    };
    assert_eq!(describe_animal(&bird), "A Penguin that cannot fly.");
}

#[test]
fn test_mixed_animals() {
    let dog = Animal::Dog;
    assert_eq!(describe_animal(&dog), "A friendly dog.");

    let cat = Animal::Cat("Luna".to_string());
    assert_eq!(describe_animal(&cat), "A cat named Luna.");

    let bird = Animal::Bird {
        species: "Sparrow".to_string(),
        can_fly: true,
    };
    assert_eq!(describe_animal(&bird), "A Sparrow that can fly.");
}
