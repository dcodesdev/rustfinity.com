pub trait Person {
    fn name(&self) -> String;
}

pub trait Student: Person {
    fn id(&self) -> u32;
    fn field_of_study(&self) -> String;
}

pub struct Undergraduate {
    pub id: u32,
    pub name: String,
    pub field_of_study: String,
}

impl Person for Undergraduate {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for Undergraduate {
    fn id(&self) -> u32 {
        self.id
    }

    fn field_of_study(&self) -> String {
        self.field_of_study.clone()
    }
}
