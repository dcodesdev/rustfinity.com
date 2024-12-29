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

#[test]
fn test_exact_age_boundary() {
    let just_barely_adult = Person {
        name: "Alex".to_string(),
        age: 18,
    };
    assert_eq!(is_adult(&just_barely_adult), true);

    let just_under_adult = Person {
        name: "Taylor".to_string(),
        age: 17,
    };
    assert_eq!(is_adult(&just_under_adult), false);
}

#[test]
fn test_large_age_values() {
    let super_elder = Person {
        name: "Methuselah".to_string(),
        age: 255,
    };
    assert_eq!(is_adult(&super_elder), true);

    let extreme_child = Person {
        name: "Time Traveler".to_string(),
        age: 10,
    };
    assert_eq!(is_adult(&extreme_child), false);
}

#[test]
fn test_non_ascii_names() {
    let non_ascii_name = Person {
        name: "Åsa".to_string(),
        age: 22,
    };
    assert_eq!(is_adult(&non_ascii_name), true);

    let young_non_ascii = Person {
        name: "小明".to_string(),
        age: 12,
    };
    assert_eq!(is_adult(&young_non_ascii), false);
}

#[test]
fn test_name_edge_cases() {
    let empty_name = Person {
        name: "".to_string(),
        age: 30,
    };
    assert_eq!(is_adult(&empty_name), true);

    let whitespace_name = Person {
        name: "   ".to_string(),
        age: 25,
    };
    assert_eq!(is_adult(&whitespace_name), true);
}
