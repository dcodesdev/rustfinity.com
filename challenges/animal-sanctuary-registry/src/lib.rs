mod tests;

use std::collections::HashMap;

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    let animals = registry.entry(section.to_string()).or_insert_with(Vec::new);

    if animals.iter().find(|&a| a == animal).is_none() {
        animals.push(animal.to_string());
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    if let Some(animals) = registry.get(section) {
        let mut sorted_animals = animals.clone();
        sorted_animals.sort();
        sorted_animals
    } else {
        Vec::new()
    }
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut animals: Vec<String> = registry
        .iter()
        .flat_map(|(_, animals)| animals.clone())
        .collect();
    animals.sort();
    animals
}
