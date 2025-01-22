pub struct Animal {
    pub name: String,
    pub age: u8,
}

pub fn create_animal(name: &str, age: u8) -> Box<Animal> {
    Box::new(Animal {
        name: name.to_string(),
        age,
    })
}

pub fn access_animal(animal: Box<Animal>) -> (String, u8) {
    let Animal { name, age } = *animal;
    (name, age)
}
