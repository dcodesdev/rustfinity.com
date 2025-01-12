use playing_cards::*;

#[test]
fn test_card_description_face_cards() {
    assert_eq!(card_description(&Card::King), "King");
    assert_eq!(card_description(&Card::Queen), "Queen");
    assert_eq!(card_description(&Card::Jack), "Jack");
}

#[test]
fn test_card_description_numbered_cards() {
    let card = Card::Numbered(7, "Hearts".to_string());
    assert_eq!(card_description(&card), "7 of Hearts");

    let card = Card::Numbered(10, "Diamonds".to_string());
    assert_eq!(card_description(&card), "10 of Diamonds");

    let card = Card::Numbered(1, "Clubs".to_string());
    assert_eq!(card_description(&card), "1 of Clubs");

    let card = Card::Numbered(13, "Spades".to_string());
    assert_eq!(card_description(&card), "13 of Spades");
}

#[test]
fn test_card_description_edge_cases() {
    let card = Card::Numbered(0, "Hearts".to_string());
    assert_eq!(card_description(&card), "0 of Hearts"); // Valid but unusual case

    let card = Card::Numbered(100, "Jokers".to_string());
    assert_eq!(card_description(&card), "100 of Jokers"); // Out of typical card range, but still valid in enum
}

#[test]
fn test_card_description_mixed_suits() {
    let card = Card::Numbered(2, "Stars".to_string());
    assert_eq!(card_description(&card), "2 of Stars");

    let card = Card::Numbered(8, "Moons".to_string());
    assert_eq!(card_description(&card), "8 of Moons");
}

#[test]
fn test_card_description_custom_face_cards() {
    // Ensure the face cards are not confused with Numbered cards
    assert_ne!(card_description(&Card::King), "13 of Hearts");
    assert_ne!(card_description(&Card::Queen), "12 of Spades");
    assert_ne!(card_description(&Card::Jack), "11 of Clubs");
}
