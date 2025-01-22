pub struct Person {
    pub name: String,
    pub age: u8,
}

pub fn is_adult(person: &Person) -> bool {
    person.age >= 18
}
