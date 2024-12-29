use structs::*;

#[test]
fn test_is_adult() {
    let john = Person {
        name: "John".to_string(),
        age: 20,
    };
    assert_eq!(is_adult(&john), true);

    let emily = Person {
        name: "Emily".to_string(),
        age: 15,
    };
    assert_eq!(is_adult(&emily), false);

    let mike = Person {
        name: "Mike".to_string(),
        age: 18,
    };
    assert_eq!(is_adult(&mike), true);
}

#[test]
fn test_edge_cases() {
    let baby = Person {
        name: "Baby".to_string(),
        age: 0,
    };
    assert_eq!(is_adult(&baby), false);

    let elder = Person {
        name: "Elder".to_string(),
        age: 99,
    };
    assert_eq!(is_adult(&elder), true);
}
