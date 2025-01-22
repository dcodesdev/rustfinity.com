pub trait Speakable {
    fn speak(&self) -> String;
}

pub struct Dog {
    pub name: String,
    pub breed: String,
}

pub struct Robot {
    pub model: String,
    pub purpose: String,
}

impl Speakable for Dog {
    fn speak(&self) -> String {
        String::from("Woof")
    }
}

impl Speakable for Robot {
    fn speak(&self) -> String {
        String::from("Beep boop")
    }
}

pub fn get_speaker(kind: &str) -> Box<dyn Speakable> {
    match kind {
        "dog" => Box::new(Dog {
            name: "Rex".to_string(),
            breed: "Labrador".to_string(),
        }),
        "robot" => Box::new(Robot {
            model: "R2-D2".to_string(),
            purpose: "assistance".to_string(),
        }),
        _ => panic!("Unknown speaker type"),
    }
}
