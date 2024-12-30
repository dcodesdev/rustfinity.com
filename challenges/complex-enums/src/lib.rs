pub enum Animal {
    Dog,
    Cat(String),
    Bird { species: String, can_fly: bool },
}

pub fn describe_animal(animal: &Animal) -> String {
    match animal {
        Animal::Dog => "A friendly dog.".to_string(),
        Animal::Cat(name) => format!("A cat named {}.", name),
        Animal::Bird { species, can_fly } => {
            if *can_fly {
                format!("A {} that can fly.", species)
            } else {
                format!("A {} that cannot fly.", species)
            }
        }
    }
}
