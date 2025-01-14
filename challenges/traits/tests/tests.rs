use traits::*;

#[test]
fn test_person_description() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    assert_eq!(person.describe(), "Person: Alice, Age: 30");
}

#[test]
fn test_book_description() {
    let book = Book {
        title: "Rust Programming".to_string(),
        author: "Jane Doe".to_string(),
    };
    assert_eq!(book.describe(), "Book: Rust Programming, Author: Jane Doe");
}

#[test]
fn test_person_with_empty_name() {
    let person = Person {
        name: "".to_string(),
        age: 25,
    };
    assert_eq!(person.describe(), "Person: , Age: 25");
}

#[test]
fn test_book_with_empty_title_and_author() {
    let book = Book {
        title: "".to_string(),
        author: "".to_string(),
    };
    assert_eq!(book.describe(), "Book: , Author: ");
}

#[test]
fn test_person_description_with_special_characters() {
    let person = Person {
        name: "J@hn_Doe!".to_string(),
        age: 45,
    };
    assert_eq!(person.describe(), "Person: J@hn_Doe!, Age: 45");
}

#[test]
fn test_book_description_with_special_characters() {
    let book = Book {
        title: "Learn Rust: A Practical Approach!".to_string(),
        author: "Dr. Cøder".to_string(),
    };
    assert_eq!(
        book.describe(),
        "Book: Learn Rust: A Practical Approach!, Author: Dr. Cøder"
    );
}

#[test]
fn test_person_age_boundary_values() {
    let young_person = Person {
        name: "Baby".to_string(),
        age: 0,
    };
    assert_eq!(young_person.describe(), "Person: Baby, Age: 0");

    let old_person = Person {
        name: "Elder".to_string(),
        age: 255, // Maximum value for `u8`
    };
    assert_eq!(old_person.describe(), "Person: Elder, Age: 255");
}
